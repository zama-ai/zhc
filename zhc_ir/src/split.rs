use zhc_utils::iter::Intermediate;

use crate::{AnnIR, Dialect, IR, OpId, OpIdRaw, OpMap, dce::eliminate_dead_code, translation::{Order, translate_ann}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ComponentId(OpIdRaw);

struct UnionFind {
    map: OpMap<OpId>,
    count: OpIdRaw,
}

impl UnionFind {
    pub fn from_ir<D: Dialect>(ir: &IR<D>) -> Self {
        UnionFind {
            map: ir.totally_mapped_opmap(|opref| opref.get_id()),
            count: ir.n_ops(),
        }
    }

    pub fn find(&mut self, opid: OpId) -> ComponentId {
        let root = {
            let mut a = opid;
            while a != self.map[a] {
                a = self.map[a];
            }
            a
        };
        // path compression
        let mut a = opid;
        while a != root {
            let next = self.map[a];
            self.map[a] = root;
            a = next;
        }
        ComponentId(root.0)
    }

    pub fn union(&mut self, l: OpId, r: OpId) {
        let lg = self.find(l);
        let rg = self.find(r);
        if lg == rg {
            return;
        }
        self.map[OpId(lg.0)] = OpId(rg.0);
        self.count -= 1;
    }

    pub fn components_iter(&self) -> impl Iterator<Item = ComponentId> {
        self.map
            .iter()
            .filter(|(o, c)| o == *c)
            .map(|(c, _)| ComponentId(c.0))
    }
}

pub fn split<D: Dialect>(
    ir: &IR<D>,
) -> Vec<IR<D>> {
    let mut uf = UnionFind::from_ir(ir);

    for valref in ir.walk_vals_linear() {
        let origin = valref.get_origin().opref;
        for user in valref.get_users_iter() {
            uf.union(*origin, *user);
        }
    }

    let mut output = Vec::new();
    for component in uf.components_iter().intermediate() {
        let annir = AnnIR::new(
            ir,
            ir.totally_mapped_opmap(|op| {
                if uf.find(*op) == component {
                    true
                } else {
                    false
                }
            }),
            ir.filled_valmap(()),
        );

        let mut output_ir = translate_ann(&annir, Order::Topological, |op, translator| {
            if !op.get_annotation() {
                return;
            }
            translator.direct_translation(&*op, op.get_instruction());
        }).output;
        eliminate_dead_code(&mut output_ir);
        if output_ir.n_ops() > 0 {
            output.push(output_ir);
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testlang::*;
    use zhc_utils::{assert_display_is, svec};

    #[test]
    fn test_empty_ir() {
        let ir = IR::<TestLang>::empty();
        let components = split(&ir);
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_single_component_kept_whole() {
        let mut ir = IR::<TestLang>::empty();
        let (_, i0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (_, i1) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        let (_, add) = ir.add_op(TestInstructionSet::Add, svec![i0[0], i1[0]]);
        ir.add_op(TestInstructionSet::Return, add);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                %2 = add(%0, %1);
                return(%2);
            "#
        );

        let components = split(&ir);
        assert_eq!(components.len(), 1);
        assert_display_is!(
            components[0].format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = int_input<pos: 1>();
                %2 = add(%0, %1);
                return(%2);
            "#
        );
    }

    #[test]
    fn test_disconnected_graphs_split() {
        let mut ir = IR::<TestLang>::empty();

        // First subgraph: inc(input(0)) -> return.
        let (_, a0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (_, a1) = ir.add_op(TestInstructionSet::Inc, svec![a0[0]]);
        ir.add_op(TestInstructionSet::Return, a1);

        // Second, independent subgraph: inc(input(1)) -> return.
        let (_, b0) = ir.add_op(TestInstructionSet::IntInput { pos: 1 }, svec![]);
        let (_, b1) = ir.add_op(TestInstructionSet::Inc, svec![b0[0]]);
        ir.add_op(TestInstructionSet::Return, b1);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = inc(%0);
                return(%1);
                %2 = int_input<pos: 1>();
                %3 = inc(%2);
                return(%3);
            "#
        );

        let components = split(&ir);
        assert_eq!(components.len(), 2);
        assert_display_is!(
            components[0].format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = inc(%0);
                return(%1);
            "#
        );
        assert_display_is!(
            components[1].format(),
            r#"
                %0 = int_input<pos: 1>();
                %1 = inc(%0);
                return(%1);
            "#
        );
    }

    #[test]
    fn test_shared_input_prevents_split() {
        let mut ir = IR::<TestLang>::empty();

        // Both chains read the same input value, so they stay in one component.
        let (_, i0) = ir.add_op(TestInstructionSet::IntInput { pos: 0 }, svec![]);
        let (_, l) = ir.add_op(TestInstructionSet::Inc, svec![i0[0]]);
        ir.add_op(TestInstructionSet::Return, l);
        let (_, r) = ir.add_op(TestInstructionSet::Inc, svec![i0[0]]);
        ir.add_op(TestInstructionSet::Return, r);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = inc(%0);
                return(%1);
                %2 = inc(%0);
                return(%2);
            "#
        );

        let components = split(&ir);
        assert_eq!(components.len(), 1);
        assert_display_is!(
            components[0].format(),
            r#"
                %0 = int_input<pos: 0>();
                %1 = inc(%0);
                %2 = inc(%0);
                return(%1);
                return(%2);
            "#
        );
    }
}
