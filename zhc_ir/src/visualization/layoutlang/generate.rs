use std::collections::VecDeque;

use zhc_utils::{
    BiMap, Dumpable, SafeAs,
    iter::{CollectInSmallVec, MultiZip},
    small::SmallVec,
    svec,
};

use crate::{
    AnnIR, AnnOpRef, Depth, Dialect, FormatContext, IR, OpId, OpRef, ValId,
    visualization::{
        Hierarchy, OpContent,
        layoutlang::{LayoutDialect, LayoutInstructionSet},
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Original(ValId);
impl Dumpable for Original {
    fn dump_to_string(&self) -> String {
        format!("O({})", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Local(ValId);
impl Dumpable for Local {
    fn dump_to_string(&self) -> String {
        format!("L({})", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Parent(ValId);
impl Dumpable for Parent {
    fn dump_to_string(&self) -> String {
        format!("P({})", self.0)
    }
}

struct StackFrame {
    ir: IR<LayoutDialect>,
    input_map: BiMap<Original, Local>,
    output_map: BiMap<Original, Local>,
    args: SmallVec<Parent>,
    ops: SmallVec<OpId>,
    hierarchy: Hierarchy,
    depth: Depth,
}

impl Dumpable for StackFrame {
    fn dump_to_string(&self) -> String {
        let args: Vec<_> = self.args.iter().map(|a| a.dump_to_string()).collect();
        let ops: Vec<_> = self.ops.iter().map(|o| o.dump_to_string()).collect();
        format!(
            "StackFrame {{\n\
             \x20 depth: {}\n\
             \x20 hierarchy: {}\n\
             \x20 args: [{}]\n\
             \x20 ops: [{}]\n\
             \x20 input_map: {}\n\
             \x20 output_map: {}\n\
             \x20 ir:\n{}\n\
             }}",
            self.depth,
            self.hierarchy.dump_to_string(),
            args.join(", "),
            ops.join(", "),
            self.input_map.dump_to_string(),
            self.output_map.dump_to_string(),
            self.ir.dump_to_string(),
        )
    }
}

impl StackFrame {
    fn has_original(&self, original: &Original) -> bool {
        self.input_map.has_dom(original) | self.output_map.has_dom(original)
    }

    fn map_original(&self, original: &Original) -> Local {
        if self.input_map.has_dom(original) {
            assert!(!self.output_map.has_dom(original));
            self.input_map.get_dom(original).unwrap().to_owned()
        } else {
            self.output_map.get_dom(original).unwrap().to_owned()
        }
    }

    fn map_local(&self, local: &Local) -> Original {
        if self.input_map.has_codom(local) {
            assert!(!self.output_map.has_codom(local));
            self.input_map.get_codom(local).unwrap().to_owned()
        } else {
            self.output_map.get_codom(local).unwrap().to_owned()
        }
    }

    fn bring(&mut self, original: &Original, parent: &Parent) -> Local {
        assert!(!self.input_map.has_dom(&original));
        let (opid, rets) = self.ir.add_op(
            LayoutInstructionSet::GroupInput {
                pos: self.args.len().sas(),
                valid: original.0,
            },
            svec![],
        );
        self.depth = std::cmp::max(self.depth, *self.ir.get_op(opid).depth);
        self.args.push(*parent);
        self.input_map.insert(*original, Local(rets[0]));
        Local(rets[0])
    }

    fn level_to(&mut self, local: &Local, depth: Depth) -> Local {
        assert!(depth <= self.depth);
        let origin = self.ir.get_val(local.0).get_origin().opref;
        assert!(*origin.depth <= depth);
        let original = self.map_local(local);
        let mut running = local.to_owned();
        let mut orig_opid = origin.get_id();
        loop {
            if *self.ir.get_op(orig_opid).depth >= depth {
                break;
            }
            let (opid, l) = self.ir.add_op(
                LayoutInstructionSet::Dummy { valid: original.0 },
                svec![running.0],
            );
            orig_opid = opid;
            running = Local(l[0]);
        }
        if self.input_map.has_dom(&original) {
            assert!(!self.output_map.has_dom(&original));
            *self.input_map.get_dom_mut(&original).unwrap() = running.clone();
        } else {
            *self.output_map.get_dom_mut(&original).unwrap() = running.clone();
        }
        running
    }

    fn insert_op<'ir, D: Dialect>(&mut self, op: &OpRef<'ir, D>) {
        let arg_depth = op
            .get_arg_valids()
            .iter()
            .map(|a| {
                let local = self.map_original(&Original(*a));
                self.ir.get_val(local.0).get_origin().opref.get_depth()
            })
            .max()
            .unwrap_or(0);
        let inputs = op
            .get_arg_valids()
            .iter()
            .map(|v| {
                let local = self.map_original(&Original(*v));
                self.level_to(&local, arg_depth).0
            })
            .cosvec();
        let (opid, rets) = self.ir.add_op(
            LayoutInstructionSet::Operation {
                opid: op.id,
                op: OpContent::from_op(op, &FormatContext::new().show_types(true)),
                args: op.get_arg_valids().iter().cloned().collect(),
                returns: op.get_return_valids().iter().cloned().collect(),
            },
            inputs,
        );
        self.depth = std::cmp::max(self.depth, *self.ir.get_op(opid).depth);
        self.ops.push(op.get_id());
        (rets.iter(), op.get_return_valids().iter())
            .mzip()
            .map(|(l, o)| (Local(*l), Original(*o)))
            .for_each(|(l, o)| {
                self.output_map.insert(o, l);
            });
    }

    fn finalize<D: Dialect>(
        &mut self,
        orig_ir: &IR<D>,
        remaining: &VecDeque<OpId>,
    ) -> SmallVec<(Original, Local)> {
        let outputs_to_map = self
            .output_map
            .iter()
            .map(|(l, r)| (*l, *r))
            .filter(|(output, _)| {
                let valref = orig_ir.get_val(output.0);
                valref
                    .get_uses_iter()
                    .any(|u| remaining.contains(&u.opref.get_id()))
            })
            .cosvec();
        for (pos, (original, local_out)) in outputs_to_map.iter().enumerate() {
            let local = self.level_to(local_out, self.depth);
            self.ir.add_op(
                LayoutInstructionSet::GroupOutput {
                    pos: pos.sas(),
                    valid: original.0,
                },
                svec![local.0],
            );
        }
        outputs_to_map
    }
}

struct Stack<'ir, 'ann, D: Dialect> {
    ir: &'ann AnnIR<'ir, D, Hierarchy, ()>,
    frames: Vec<StackFrame>,
    remaining: VecDeque<OpId>,
}

impl<'ir, 'ann, D: Dialect> Dumpable for Stack<'ir, 'ann, D> {
    fn dump_to_string(&self) -> String {
        if self.frames.is_empty() {
            return String::from("Stack { empty }");
        }

        let mut lines: Vec<String> = Vec::new();
        let width = 60;
        let bar = "─".repeat(width - 2);

        // Build each frame's content (bottom of stack = index 0, top = last)
        for (i, frame) in self.frames.iter().enumerate() {
            let is_top = i == self.frames.len() - 1;
            let is_bottom = i == 0;

            // Frame header
            let header = format!(
                " Frame {} │ {} │ depth: {}",
                i,
                frame.hierarchy.comment(),
                frame.depth
            );
            let padded_header = format!("{:width$}", header, width = width - 2);

            if is_bottom {
                lines.push(format!("╔{}╗", bar));
            } else {
                lines.push(format!("╠{}╣", bar));
            }
            lines.push(format!("║{}║", padded_header));
            lines.push(format!("╟{}╢", "┄".repeat(width - 2)));

            // IR content, indented
            let ir_str = frame.ir.dump_to_string();
            for ir_line in ir_str.lines() {
                let content = format!("  {}", ir_line);
                let padded = format!("{:width$}", content, width = width - 2);
                // Truncate if too long
                let truncated: String = padded.chars().take(width - 2).collect();
                lines.push(format!("║{}║", truncated));
            }
            if ir_str.is_empty() {
                let empty = format!("{:width$}", "  (empty)", width = width - 2);
                lines.push(format!("║{}║", empty));
            }

            if is_top {
                lines.push(format!("╚{}╝", bar));
            }
        }

        lines.join("\n")
    }
}

impl<'ir, 'ann, D: Dialect> Stack<'ir, 'ann, D> {
    pub fn from(ir: &'ann AnnIR<'ir, D, Hierarchy, ()>) -> Self {
        let root = ir
            .walk_ops_linear()
            .next()
            .unwrap()
            .get_annotation()
            .get_root();
        let remaining = ir.walk_ops_linear().map(|op| op.get_id()).collect();
        let mut output = Stack {
            ir,
            frames: Vec::new(),
            remaining,
        };
        output.push_frame(root);
        output
    }

    pub fn process(&mut self) {
        while let Some(opid) = self.remaining.front() {
            let opref = self.ir.get_op(*opid);
            self.push_op(opref);
            self.remaining.pop_front();
        }
    }

    fn push_frame(&mut self, hierarchy: Hierarchy) {
        self.frames.push(StackFrame {
            ir: IR::empty(),
            input_map: BiMap::new(),
            output_map: BiMap::new(),
            args: SmallVec::new(),
            ops: SmallVec::new(),
            hierarchy,
            depth: 0,
        });
    }

    fn prepare_input(&mut self, original: &Original) -> Local {
        // To prepare the input, we need to trace back in the hierarchy to find the frame that last
        // used the original valid to be used. Then from this point inputs must be
        // transitively added to every frames down to the current frame.
        // Note that it is not possible for a valid to not be mapped by any of the stack frames. At
        // the very least, the root frame will map the valid.

        // We walk the stack up to find the last frame with the valid.
        let mut cursor = self.frames.len() - 1;
        loop {
            if self.frames[cursor].has_original(original) {
                break;
            } else {
                cursor -= 1;
            }
        }
        // We walk down the stack, transitively bringing the valid to the current frame.
        let mut output = self.frames[cursor].map_original(original);
        loop {
            if cursor == self.frames.len() - 1 {
                break;
            } else {
                cursor += 1;
                output = self.frames[cursor].bring(original, &Parent(output.0));
            }
        }
        output
    }

    fn pop_frame(&mut self) {
        let mut previous_frame = self.frames.pop().unwrap();
        let outputs_to_map = previous_frame.finalize(self.ir, &self.remaining);
        let StackFrame {
            ir,
            args,
            hierarchy,
            ..
        } = previous_frame;

        let current_frame = self.frames.last_mut().unwrap();
        let arg_depth = args
            .iter()
            .map(|a| current_frame.ir.get_val(a.0).get_origin().opref.get_depth())
            .max()
            .unwrap_or(0);
        let new_args = args
            .into_iter()
            .map(|a| current_frame.level_to(&Local(a.0), arg_depth).0)
            .cosvec();

        let (opid, rets) = current_frame.ir.add_op(
            LayoutInstructionSet::Group {
                name: hierarchy.comment(),
                ir,
            },
            new_args,
        );
        current_frame.depth =
            std::cmp::max(current_frame.depth, *current_frame.ir.get_op(opid).depth);
        for (local, (original, _)) in (rets.into_iter(), outputs_to_map.into_iter()).mzip() {
            self.frames
                .last_mut()
                .unwrap()
                .output_map
                .insert(original, Local(local));
        }
    }

    fn hierarchy(&self) -> &Hierarchy {
        &self.frames.last().unwrap().hierarchy
    }

    fn push_op(&mut self, op: AnnOpRef<'ir, 'ann, D, Hierarchy, ()>) {
        // We get the common ancestor between the current position in the hierarchy and the one of
        // the operation.
        let ancestor = self
            .hierarchy()
            .common_ancestor(op.get_annotation())
            .unwrap();
        // From the current position in the hierarchy, back to the ancestor, we pop the frame.
        for _ in Hierarchy::range(&ancestor, self.hierarchy().clone()).rev() {
            self.pop_frame();
        }
        // From the ancestor position in the hierarchy, down to the op hierachy position, we push
        // frames.
        for h in Hierarchy::range(&ancestor, op.get_annotation().clone()) {
            self.push_frame(h);
        }
        // Now correctly framed, we can pull every necessary inputs.
        op.get_arg_valids().iter().for_each(|v| {
            self.prepare_input(&Original(*v));
        });
        self.frames.last_mut().unwrap().insert_op(&*op);
    }

    pub fn finish(mut self) -> IR<LayoutDialect> {
        while self.frames.len() > 1 {
            self.pop_frame();
        }
        self.frames.pop().unwrap().ir
    }
}

pub fn generate_layout_ir<'ir, 'ann, D: Dialect>(
    input: &'ann AnnIR<'ir, D, Hierarchy, ()>,
) -> IR<LayoutDialect> {
    let mut stack = Stack::from(input);
    stack.process();
    stack.finish()
}
