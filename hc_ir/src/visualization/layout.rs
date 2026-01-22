//! Graph layout system for intermediate representation visualization.
//!
//! Provides hierarchical layout of IR operations and values across depth layers,
//! with automatic positioning based on data flow relationships. The main `Layout`
//! struct arranges `Node` elements (operations and values) in a 2D grid where
//! vertical layers represent IR depths and horizontal positioning minimizes
//! edge crossings.
//!
//! Layout construction uses iterative refinement with `reorder_bottom_up` and
//! `reorder_top_down` passes that position nodes based on the average positions
//! of their dependencies. The `Position` enum tracks both temporary averages
//! during refinement and final indices after sorting.
//
// TODO:
// + add fractional approximate positioning for the input / output order.
// + Make graph more compact (do not use height only to position the nodes).
// + Fix

use std::fmt::Debug;

use hc_utils::{
    FastMap,
    iter::{CollectInSmallVec, Deduped, Median, MergerOf2, MultiZip},
    small::{SmallSet, SmallVec},
    svec,
};

use crate::{AnnIR, AnnOpRef, AnnValRef, Dialect, IR, OpId, OpRef, ValId, val_ref::ValRef};

/// The height of an op is the largest distance between this op and an output/effect operation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Height(pub(super) u16);

impl Height {
    pub fn inc(self) -> Self {
        Height(self.0 + 1)
    }

    pub fn dec(self) -> Self {
        Height(self.0 - 1)
    }

    pub fn to_top_distance(self, ir_depth: u16) -> u16 {
        ir_depth - self.0 - 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Children(pub(super) SmallSet<OpId>);

impl std::hash::Hash for Children {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut sorted_ops = self.0.iter().cosvec();
        sorted_ops.sort_unstable();
        sorted_ops.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Analysis {
    height: Height,
    children: Children
}

fn analyze<D: Dialect>(ir: &IR<D>) -> AnnIR<'_, D, Analysis, ()> {
    ir.backward_dataflow_analysis::<Analysis, ()>(|opmap, _, opref| {
        let height = opref
            .get_users_iter()
            .map(|user| opmap.get(&user).unwrap().height.0)
            .max()
            .map(|a| Height(a).inc())
            .unwrap_or(Height(0));
        let children = opref.get_users_iter().fold(Children(SmallSet::new()), |mut acc, user| {
            acc.0.insert(user.get_id());
            acc.0.extend(opmap.get(&user).unwrap().children.0.iter().cloned());
            acc
        });
        (Analysis{height, children}, svec![(); opref.get_return_valids().len()])
    })
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum HeightedNode<'ir, 'ann, D: Dialect> {
    Operation(AnnOpRef<'ir, 'ann, D, Analysis, ()>),
    Value(AnnValRef<'ir, 'ann, D, Analysis, ()>, Height),
}

impl<'ir, 'ann, D: Dialect> std::fmt::Debug for HeightedNode<'ir, 'ann, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeightedNode::Operation(op_ref) => write!(f, "Op(\"{}\")", op_ref),
            HeightedNode::Value(val_ref, height) => write!(f, "Val(\"{}\", {:?})", val_ref, height),
        }
    }
}

impl<'ir, 'ann, D: Dialect> HeightedNode<'ir, 'ann, D> {
    fn above(&self) -> impl Iterator<Item = HeightedNode<'ir, 'ann, D>> {
        match self {
            HeightedNode::Operation(op_ref) => op_ref
                .get_args_iter()
                .map(|v| {
                    if v.get_origin().get_annotation().height == op_ref.get_annotation().height.inc() {
                        HeightedNode::Operation(v.get_origin())
                    } else {
                        HeightedNode::Value(v, op_ref.get_annotation().height.inc())
                    }
                })
                .merge_1_of_2(),
            HeightedNode::Value(val_ref, height) => {
                if val_ref.get_origin().get_annotation().height == height.inc() {
                    std::iter::once(HeightedNode::Operation(val_ref.get_origin()))
                } else {
                    std::iter::once(HeightedNode::Value(val_ref.clone(), height.inc()))
                }
                .merge_2_of_2()
            }
        }
        .dedup()
    }

    fn below(&self) -> impl Iterator<Item = HeightedNode<'ir, 'ann, D>> {
        match self {
            HeightedNode::Operation(op_ref) => op_ref
                .get_returns_iter()
                .flat_map(|r| (std::iter::repeat(r.clone()), r.get_users_iter()).mzip())
                .map(|(r, u)| {
                    if u.get_annotation().height == op_ref.get_annotation().height.dec() {
                        HeightedNode::Operation(u)
                    } else {
                        HeightedNode::Value(r, op_ref.get_annotation().height.dec())
                    }
                })
                .merge_1_of_2(),
            HeightedNode::Value(val_ref, height) => val_ref
                .get_users_iter()
                .filter(|u| u.get_annotation().height < *height)
                .map(move |u| {
                    if u.get_annotation().height == height.dec() {
                        HeightedNode::Operation(u)
                    } else {
                        HeightedNode::Value(val_ref.clone(), height.dec())
                    }
                })
                .merge_2_of_2(),
        }
        .dedup()
    }
}

enum Position {
    Approximate(f64),
    Index(usize),
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Approximate(avg) => write!(f, "Avg({})", avg),
            Position::Index(idx) => write!(f, "Idx({})", idx),
        }
    }
}

impl Position {
    fn unwrap_average(&self) -> f64 {
        match self {
            Position::Approximate(a) => *a,
            Position::Index(_) => panic!(),
        }
    }

    fn unwrap_index(&self) -> usize {
        match self {
            Position::Index(i) => *i,
            Position::Approximate(_) => panic!(),
        }
    }
}

type VStack<T> = Vec<T>;
type HStack<T> = Vec<T>;

#[derive(Debug)]
struct LayoutBuilder<'ir, 'ann, D: Dialect> {
    // Layout is a vertical stack (top-to-bottom ordered) of horizontal stacks (left-to-right ordered) of nodes.
    layout: VStack<HStack<HeightedNode<'ir, 'ann, D>>>,
    position_buffer: FastMap<HeightedNode<'ir, 'ann, D>, Position>,
}

impl<'ir, 'ann, D: Dialect> LayoutBuilder<'ir, 'ann, D> {
    fn from_analyzed_ir(ann_ir: &'ann AnnIR<'ir, D, Analysis, ()>) -> Self {
        let depth = ann_ir.depth() + 1;
        let mut layout = vec![vec![]; depth as usize];
        let mut position_buffer = FastMap::new();

        // We require annotation of op height because it puts the operation closer to their
        // user than the depth (which puts all input nodes at the top even though they are
        // used way later). But that said, the layout is still from top to bottom. So we
        // need to convert the height into a top-rooted index.
        // panic!("{}", depth + 1);
        let h2i = |h: &Height| h.to_top_distance(depth) as usize;

        // We add a node for each operation. Nothing fancy.
        for op in ann_ir.walk_ops_linear() {
            let node = HeightedNode::Operation(op.clone());
            position_buffer.insert(
                node.clone(),
                Position::Index(layout[h2i(&op.get_annotation().height)].len()),
            );
            layout[h2i(&op.get_annotation().height)].push(HeightedNode::Operation(op.clone()));
        }

        // Now, for every value that is not used only in the layer below its origin, we need
        // to add intermediate val nodes.
        for val in ann_ir.walk_vals_linear() {
            let from_height = val.get_origin().get_annotation().height;
            let to_height = val
                .get_users_iter()
                .map(|to| to.get_annotation().height)
                .min()
                .unwrap_or(from_height)
                .inc();
            // Range is reverse ordered, because from is higher than to.
            for i in to_height.0..from_height.0 {
                let height = Height(i);
                let node = HeightedNode::Value(val.clone(), height);
                position_buffer.insert(node.clone(), Position::Index(layout[h2i(&height)].len()));
                layout[h2i(&height)].push(node)
            }
        }
        // We create the builder object
        let mut builder = Self {
            layout,
            position_buffer,
        };

        for _ in 0..10 {
            builder.reorder_bottom_up();
            builder.reorder_top_down();
        }

        builder
    }

    fn depth(&self) -> usize {
        self.layout.len()
    }

    fn reorder_bottom_up(&mut self) {
        for layer in (0..self.depth()).rev() {
            for node in self.layout[layer].iter() {
                let maybe_median = node
                    .below()
                    .map(|n| self.position_buffer.get(&n).unwrap().unwrap_index() as f64)
                    .median();
                let Some(pos) = self.position_buffer.get_mut(node) else {
                    unreachable!()
                };
                match maybe_median {
                    Some(median) => {
                        *pos = Position::Approximate(median);
                    },
                    None => {
                        *pos = Position::Approximate(pos.unwrap_index() as f64);
                    },
                }
            }
            self.layout[layer].as_mut_slice().sort_unstable_by(|a, b| {
                self.position_buffer
                    .get(a)
                    .unwrap()
                    .unwrap_average()
                    .total_cmp(&self.position_buffer.get(b).unwrap().unwrap_average())
            });
            for (i, node) in self.layout[layer].iter().enumerate() {
                let Some(pos) = self.position_buffer.get_mut(node) else {
                    unreachable!()
                };
                *pos = Position::Index(i);
            }
        }
    }

    fn reorder_top_down(&mut self) {
        for layer in 0..self.depth() {
            for node in self.layout[layer].iter() {
                let maybe_median = node
                    .above()
                    .map(|n| self.position_buffer.get(&n).unwrap().unwrap_index() as f64)
                    .median();
                let Some(pos) = self.position_buffer.get_mut(node) else {
                    unreachable!()
                };
                match maybe_median {
                    Some(median) => {
                        *pos = Position::Approximate(median);
                    },
                    None => {
                        *pos = Position::Approximate(pos.unwrap_index() as f64);
                    },
                }
            }
            self.layout[layer].as_mut_slice().sort_unstable_by(|a, b| {
                self.position_buffer
                    .get(a)
                    .unwrap()
                    .unwrap_average()
                    .total_cmp(&self.position_buffer.get(b).unwrap().unwrap_average())
            });
            for (i, node) in self.layout[layer].iter().enumerate() {
                let Some(pos) = self.position_buffer.get_mut(node) else {
                    unreachable!()
                };
                *pos = Position::Index(i);
            }
        }
    }

    fn into_vertices(self) -> VStack<HStack<Vertex>> {
        self.layout
            .into_iter()
            .map(|layer| {
                layer
                    .into_iter()
                    .map(|n| match n {
                        HeightedNode::Operation(ann_op_ref) => {
                            Vertex::Operation(ann_op_ref.get_id())
                        }
                        HeightedNode::Value(ann_val_ref, _) => Vertex::Value(ann_val_ref.get_id()),
                    })
                    .collect()
            })
            .collect()
    }
}

/// Coordinates in the layout
#[derive(Debug, Clone)]
pub struct Coordinates {
    pub layer: u16,
    pub node: u16,
    pub spec: CoordinatesSpec,
}

#[derive(Debug, Clone)]
pub enum CoordinatesSpec {
    OpArg(u8),
    OpRet(u8),
    Val,
}

#[derive(Debug, Clone)]
pub enum Vertex {
    Operation(OpId),
    Value(ValId),
}

#[derive(Debug, Clone)]
pub struct Link {
    pub value: ValId,
    pub path: SmallVec<Coordinates>,
}

#[derive(Debug)]
pub struct Layout {
    vertices: VStack<HStack<Vertex>>,
    links: Vec<Link>,
}

impl Layout {
    pub fn from_ir<D: Dialect>(ir: &IR<D>) -> Self {
        let ann_ir = analyze(ir);
        let builder = LayoutBuilder::from_analyzed_ir(&ann_ir);
        let vertices = builder.into_vertices();
        let mut links = Vec::new();

        // Now we have to build the list of links. For that we use a worklist approach, and iterate through the layers.
        #[derive(Debug)]
        struct FutureLink<'ir, D: Dialect> {
            value: ValRef<'ir, D>,
            path: SmallVec<Coordinates>,
            user: OpRef<'ir, D>,
        }
        let mut work_list: Vec<FutureLink<D>> = Vec::new();
        for (layer_i, layer) in vertices.iter().enumerate() {
            // STEP 0 -> We build two maps, one that maps to valids and another to opids.
            let (val_map, op_map) = {
                let mut val_map = FastMap::new();
                let mut op_map = FastMap::new();
                layer
                    .iter()
                    .enumerate()
                    .for_each(|(node_i, node)| match node {
                        Vertex::Operation(opid) => {
                            op_map.insert(opid, node_i as u16);
                        }
                        Vertex::Value(valid) => {
                            val_map.insert(valid, node_i as u16);
                        }
                    });
                (val_map, op_map)
            };

            // STEP 1 -> We finalize every link in the worklist that finishes at this layer.
            work_list.retain_mut(|fut| {
                if op_map.contains_key(&*fut.user) {
                    let arg_id = fut
                        .user
                        .get_arg_valids()
                        .iter()
                        .position(|id| *id == fut.value.get_id())
                        .unwrap();
                    fut.path.push(Coordinates {
                        layer: layer_i as u16,
                        node: *op_map.get(&*fut.user).unwrap(),
                        spec: CoordinatesSpec::OpArg(arg_id as u8),
                    });
                    links.push(Link {
                        value: fut.value.id,
                        path: fut.path.clone(),
                    });
                    false
                } else {
                    true
                }
            });

            // STEP 2 -> The worklist contains only transient links in the layer. We give them their control point in the layer.
            work_list.iter_mut().for_each(|fut| {
                fut.path.push(Coordinates {
                    layer: layer_i as u16,
                    node: *val_map.get(&*fut.value).unwrap(),
                    spec: CoordinatesSpec::Val,
                });
            });

            // STEP 3 -> We append the worklist with future links for every operation returns in the layer.
            work_list.extend(op_map.into_iter().flat_map(|(opid, node_i)| {
                ir.get_op(*opid)
                    .get_returns_iter()
                    .enumerate()
                    .flat_map(move |(ret_i, ret)| {
                        ret.get_users_iter().map(move |user| FutureLink {
                            value: ret.clone(),
                            path: svec![Coordinates {
                                layer: layer_i as u16,
                                node: node_i as u16,
                                spec: CoordinatesSpec::OpRet(ret_i as u8)
                            }],
                            user,
                        })
                    })
            }));
        }
        Layout { vertices, links }
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item = impl Iterator<Item = &Vertex>> {
        self.vertices.iter().map(|a| a.iter())
    }

    pub fn iter_links(&self) -> impl Iterator<Item = &Link> {
        self.links.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::gen_complex_ir;

    #[test]
    fn test() {
        let ir = gen_complex_ir().unwrap();
        let analyzed_ir = analyze(&ir);
        // analyzed_ir.check_ir("");
        let _layout = Layout::from_ir(&analyzed_ir);
    }
}
