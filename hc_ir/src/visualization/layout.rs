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

use std::fmt::Debug;

use hc_utils::{
    FastMap,
    iter::{Deduped, Median, Merger1Of2, Merger2Of2, MultiZip},
    small::SmallVec,
    svec,
};

use crate::{Depth, Dialect, IR, OpRef, val_ref::ValRef};

/// Represents either an operation or value at a specific depth in the IR.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum Node<'ir, D: Dialect> {
    Operation(OpRef<'ir, D>),
    Value(ValRef<'ir, D>, Depth),
}

// impl<'ir, D: Dialect> std::fmt::Debug for Node<'ir, D> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Node::Operation(op_ref) => write!(f, "Op(\"{}\")", op_ref),
//             Node::Value(val_ref, depth) => write!(f, "Val(\"{}\", {})", val_ref, depth),

//         }
//     }
// }

impl<'ir, D: Dialect> Node<'ir, D> {
    /// Returns nodes that this node depends on in the layer above.
    pub fn above(&self) -> impl Iterator<Item = Node<'ir, D>> {
        match self {
            Node::Operation(op_ref) => op_ref
                .get_args_iter()
                .map(|v| {
                    if v.get_origin().get_depth() == op_ref.get_depth() - 1 {
                        Node::Operation(v.get_origin())
                    } else {
                        Node::Value(v, op_ref.get_depth() - 1)
                    }
                })
                .merge_1_of_2(),
            Node::Value(val_ref, depth) => if val_ref.get_origin().get_depth() == depth - 1 {
                std::iter::once(Node::Operation(val_ref.get_origin()))
            } else {
                std::iter::once(Node::Value(val_ref.clone(), depth - 1))
            }
            .merge_2_of_2(),
        }
        .dedup()
    }

    /// Returns nodes that depend on this node in the layer below.
    pub fn below(&self) -> impl Iterator<Item = Node<'ir, D>> {
        match self {
            Node::Operation(op_ref) => op_ref
                .get_returns_iter()
                .flat_map(|r| (std::iter::repeat(r.clone()), r.get_users_iter()).mzip())
                .map(|(r, u)| {
                    if u.get_depth() == op_ref.get_depth() + 1 {
                        Node::Operation(u)
                    } else {
                        Node::Value(r, op_ref.get_depth() + 1)
                    }
                })
                .merge_1_of_2(),
            Node::Value(val_ref, depth) => val_ref
                .get_users_iter()
                .filter(|u| u.get_depth() > *depth)
                .map(move |u| {
                    if u.get_depth() == depth + 1 {
                        Node::Operation(u)
                    } else {
                        Node::Value(val_ref.clone(), depth + 1)
                    }
                })
                .merge_2_of_2(),
        }
        .dedup()
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

/// Tracks node positions during layout refinement.
enum Position {
    Average(f64),
    Index(usize),
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Average(avg) => write!(f, "Avg({})", avg),
            Position::Index(idx) => write!(f, "Idx({})", idx),
        }
    }
}

impl Position {
    /// Extracts the average value for positioning calculations.
    ///
    /// # Panics
    ///
    /// Panics if the position is an index rather than an average.
    pub fn unwrap_average(&self) -> f64 {
        match self {
            Position::Average(a) => *a,
            Position::Index(_) => panic!(),
        }
    }

    /// Extracts the final index position after sorting.
    ///
    /// # Panics
    ///
    /// Panics if the position is an average rather than an index.
    pub fn unwrap_index(&self) -> usize {
        match self {
            Position::Index(i) => *i,
            Position::Average(_) => panic!(),
        }
    }
}

type VStack<T> = Vec<T>;
type HStack<T> = Vec<T>;

/// Hierarchical layout of IR nodes arranged by depth with optimized positioning.
pub struct Layout<'ir, D: Dialect> {
    layout: VStack<HStack<Node<'ir, D>>>,
    position_buffer: FastMap<Node<'ir, D>, Position>,
}

impl<'ir, D: Dialect> Layout<'ir, D> {
    /// Creates a new layout from the given IR with iterative positioning refinement.
    pub fn from_ir(ir: &'ir IR<D>) -> Self {
        let mut layout = vec![vec![]; (ir.depth() + 1) as usize];
        let mut position_buffer = FastMap::new();

        for op in ir.walk_ops_linear() {
            let node = Node::Operation(op.clone());
            position_buffer.insert(
                node.clone(),
                Position::Index(layout[op.get_depth() as usize].len()),
            );
            layout[op.get_depth() as usize].push(Node::Operation(op));
        }

        for val in ir.walk_vals_linear() {
            let from = val.get_origin();
            for to in val.get_users_iter() {
                for i in from.get_depth() + 1..to.get_depth() {
                    let node = Node::Value(val.clone(), i);
                    position_buffer.insert(node.clone(), Position::Index(layout[i as usize].len()));
                    layout[i as usize].push(Node::Value(val.clone(), i))
                }
            }
        }

        let mut output = Self {
            layout,
            position_buffer,
        };
        for _ in 0..10 {
            output.reorder_top_down();
            output.reorder_bottom_up();
        }
        output
    }

    fn depth(&self) -> usize {
        self.layout.len()
    }

    fn reorder_bottom_up(&mut self) {
        for layer in (0..self.depth()).rev() {
            for node in self.layout[layer].iter() {
                let a: f64 = node
                    .below()
                    .map(|n| self.position_buffer.get(&n).unwrap().unwrap_index() as f64)
                    .median()
                    .unwrap_or(0.);
                let Some(pos) = self.position_buffer.get_mut(node) else {
                    unreachable!()
                };
                *pos = Position::Average(a);
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
                let a: f64 = node
                    .above()
                    .map(|n| self.position_buffer.get(&n).unwrap().unwrap_index() as f64)
                    .median()
                    .unwrap_or(0.);
                let Some(pos) = self.position_buffer.get_mut(node) else {
                    unreachable!()
                };
                *pos = Position::Average(a);
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

    pub fn iter_vertices<'a>(
        &'a self,
    ) -> impl Iterator<Item = impl Iterator<Item = &'a Node<'ir, D>>> {
        self.layout.iter().map(|a| a.iter())
    }

    pub fn iter_links(&self) -> impl Iterator<Item = (SmallVec<Coordinates>, String)> {
        #[derive(Debug)]
        struct FutureLink<'ir, D: Dialect> {
            value: ValRef<'ir, D>,
            path: SmallVec<Coordinates>,
            goal: OpRef<'ir, D>,
        }
        let mut output = Vec::new();
        let mut work_list: Vec<FutureLink<'ir, D>> = Vec::new();

        for (layer_i, layer) in self.layout.iter().enumerate() {
            // STEP 0 -> We build two maps, one that maps to valids and another to opids.
            let (val_map, op_map) = {
                let mut val_map = FastMap::new();
                let mut op_map = FastMap::new();
                layer
                    .iter()
                    .enumerate()
                    .for_each(|(node_i, node)| match node {
                        Node::Operation(op_ref) => {
                            op_map.insert(op_ref.to_owned(), node_i as u16);
                        }
                        Node::Value(val_ref, _) => {
                            val_map.insert(val_ref.to_owned(), node_i as u16);
                        }
                    });
                (val_map, op_map)
            };

            // STEP 1 -> We finalize every link in the worklist that finishes at this layer.
            work_list.retain_mut(|fut| {
                if op_map.contains_key(&fut.goal) {
                    let arg_id = fut
                        .goal
                        .get_arg_valids()
                        .iter()
                        .position(|id| *id == fut.value.get_id())
                        .unwrap();
                    fut.path.push(Coordinates {
                        layer: layer_i as u16,
                        node: *op_map.get(&fut.goal).unwrap(),
                        spec: CoordinatesSpec::OpArg(arg_id as u8),
                    });
                    output.push((fut.path.clone(),format!("{}: {}", fut.value.to_string(), fut.value.get_type())));
                    false
                } else {
                    true
                }
            });

            // STEP 2 -> The worklist contains only transient links in the layer. We give them their control point in the layer.
            work_list.iter_mut().for_each(|fut| {
                fut.path.push(Coordinates {
                    layer: layer_i as u16,
                    node: *val_map.get(&fut.value).unwrap(),
                    spec: CoordinatesSpec::Val,
                });
            });

            // STEP 3 -> We append the worklist with future links for every operation returns in the layer.
            work_list.extend(op_map.into_iter().flat_map(|(opref, node_i)| {
                opref
                    .get_returns_iter()
                    .enumerate()
                    .flat_map(move |(ret_i, ret)| {
                        ret.get_users_iter().map(move |cons| FutureLink {
                            value: ret.clone(),
                            path: svec![Coordinates {
                                layer: layer_i as u16,
                                node: node_i as u16,
                                spec: CoordinatesSpec::OpRet(ret_i as u8)
                            }],
                            goal: cons,
                        })
                    })
            }));
        }

        output.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::gen_complex_ir;

    #[test]
    fn test() {
        let ir = gen_complex_ir().unwrap();
        let _layout = Layout::from_ir(&ir);
    }
}
