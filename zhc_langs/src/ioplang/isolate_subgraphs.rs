use zhc_ir::{
    AnnIR, IR, OpId, OpIdRaw, OpMap, dce::eliminate_dead_code, translation::eager_translate_ann,
};
use zhc_utils::iter::Intermediate;

use crate::ioplang::{IopInstructionSet, IopLang};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ComponentId(OpIdRaw);

struct UnionFind {
    map: OpMap<OpId>,
    count: OpIdRaw,
}

impl UnionFind {
    pub fn from_ir(ir: &IR<IopLang>) -> Self {
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

pub fn isolate_subgraphs<D: Fn(IopInstructionSet) -> bool>(
    ir: &IR<IopLang>,
    should_duplicate: D,
) -> Vec<IR<IopLang>> {
    let mut uf = UnionFind::from_ir(ir);

    for valref in ir.walk_vals_linear() {
        let origin = valref.get_origin().opref;
        for user in valref.get_users_iter() {
            if !(should_duplicate(origin.get_instruction())) {
                uf.union(*origin, *user);
            }
        }
    }

    let mut output = Vec::new();
    for component in uf.components_iter().intermediate() {
        let annir = AnnIR::new(
            ir,
            ir.totally_mapped_opmap(|op| {
                if uf.find(*op) == component {
                    true
                } else if should_duplicate(op.get_instruction()) {
                    true
                } else {
                    false
                }
            }),
            ir.filled_valmap(()),
        );
        let mut output_ir = eager_translate_ann(&annir, |op, translator| {
            if !op.get_annotation() {
                // If the op is not in the component, we continue.
                return;
            }
            translator.direct_translation(op.clone(), op.get_instruction());
        });
        eliminate_dead_code(&mut output_ir);
        if output_ir.n_ops() > 0 {
            output.push(output_ir);
        }
    }

    output
}

#[cfg(test)]
mod test {
    use zhc_ir::IR;
    use zhc_utils::{assert_display_is, svec};

    use crate::ioplang::{
        IopInstructionSet, IopLang, IopTypeSystem, cut_transfers, isolate_subgraphs,
    };

    #[test]
    fn test_cut_transfers() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, l) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 3 }, svec![]);
        let (_, r) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 5 }, svec![]);
        let (_, rets) = ir.add_op(IopInstructionSet::AddCt, svec![l[0], r[0]]);
        let (_, t) = ir.add_op(IopInstructionSet::Transfer, svec![rets[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![t[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = let_ct_block<3>();
            %1 : CtBlock = let_ct_block<5>();
            %2 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
            %3 : CtBlock = transfer(%2 : CtBlock);
            _consume<CtBlock>(%3 : CtBlock);
        "#
        );

        cut_transfers(&mut ir);
        let components = isolate_subgraphs(&ir, |op| {
            use IopInstructionSet::*;
            match op {
                InputCiphertext { .. }
                | InputPlaintext { .. }
                | ExtractCtBlock { .. }
                | ExtractPtBlock { .. }
                | DeclareCiphertext { .. }
                | LetCiphertextBlock { .. }
                | LetPlaintextBlock { .. } => true,
                _ => false
            }
        });

        assert_display_is!(
            components[0].format(),
            r#"
                %0 : CtBlock = transfer_in<#1>();
                _consume<CtBlock>(%0 : CtBlock);
            "#
        );

        assert_display_is!(
            components[1].format(),
            r#"
                %0 : CtBlock = let_ct_block<3>();
                %1 : CtBlock = let_ct_block<5>();
                %2 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
                transfer_out<#1>(%2 : CtBlock);
            "#
        );
    }

    #[test]
    fn test_shared_input_prevents_split() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, i1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 1 }, svec![]);
        let (_, i2) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 2 }, svec![]);
        let (_, add1) = ir.add_op(IopInstructionSet::AddCt, svec![i1[0], i2[0]]);
        let (_, t1) = ir.add_op(IopInstructionSet::Transfer, svec![add1[0]]);
        let (_, add2) = ir.add_op(IopInstructionSet::AddCt, svec![t1[0], i1[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![add2[0]],
        );

        assert_display_is!(
            ir.format().with_walker(zhc_ir::PrintWalker::Linear),
            r#"
                %0 : CtBlock = let_ct_block<1>();
                %1 : CtBlock = let_ct_block<2>();
                %2 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
                %3 : CtBlock = transfer(%2 : CtBlock);
                %4 : CtBlock = add_ct(%3 : CtBlock, %0 : CtBlock);
                _consume<CtBlock>(%4 : CtBlock);
            "#
        );

        cut_transfers(&mut ir);
        let components = isolate_subgraphs(&ir, |op| {
            use IopInstructionSet::*;
            match op {
                InputCiphertext { .. }
                | InputPlaintext { .. }
                | ExtractCtBlock { .. }
                | ExtractPtBlock { .. }
                | DeclareCiphertext { .. }
                | LetCiphertextBlock { .. }
                | LetPlaintextBlock { .. } => true,
                _ => false
            }
        });
        assert_eq!(components.len(), 2);

        // Consumer 1
        assert_display_is!(
            components[0].format(),
            r#"
                %0 : CtBlock = let_ct_block<1>();
                %1 : CtBlock = transfer_in<#1>();
                %2 : CtBlock = add_ct(%1 : CtBlock, %0 : CtBlock);
                _consume<CtBlock>(%2 : CtBlock);
            "#
        );

        // Consumer 2
        assert_display_is!(
            components[1].format(),
            r#"
                %0 : CtBlock = let_ct_block<1>();
                %1 : CtBlock = let_ct_block<2>();
                %2 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
                transfer_out<#1>(%2 : CtBlock);
            "#
        );
    }
}
