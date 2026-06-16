use zhc_ir::IR;
use zhc_utils::{iter::CollectInSmallVec, svec};

use crate::ioplang::{IopInstructionSet, IopLang};

pub fn cut_transfers(ir: &mut IR<IopLang>) {
    assert!(
        ir.walk_ops_linear()
            .filter(|op| matches!(
                op.get_instruction(),
                IopInstructionSet::TransferIn { .. } | IopInstructionSet::TransferOut { .. }
            ))
            .count()
            == 0
    );
    let mut counter: u8 = 0;
    let annir = ir.forward_dataflow_analysis(|opref| match opref.get_instruction() {
        IopInstructionSet::Transfer => {
            counter += 1;
            (Some(counter), svec![(); opref.get_return_arity()])
        }
        _ => (None, svec![(); opref.get_return_arity()]),
    });
    let transfers = annir
        .walk_ops_linear()
        .filter(|op| matches!(op.get_instruction(), IopInstructionSet::Transfer))
        .map(|op| {
            (
                op.get_id(),
                op.get_arg_valids()[0],
                op.get_return_valids()[0],
                op.get_annotation().unwrap(),
            )
        })
        .cosvec();

    for (topid, tro_valid, tri_valid, trid) in transfers.into_iter() {
        ir.add_op(
            IopInstructionSet::TransferOut { uid: trid },
            svec![tro_valid],
        );
        let (_, tri) = ir.add_op(IopInstructionSet::TransferIn { uid: trid }, svec![]);
        ir.replace_val_use(tri_valid, tri[0]);
        ir.delete_op(topid);
    }
}

#[cfg(test)]
mod test {
    use zhc_ir::IR;
    use zhc_utils::{assert_display_is, svec};

    use crate::ioplang::{IopInstructionSet, IopLang, IopTypeSystem, cut_transfers};

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
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                %3 = transfer(%2);
                _consume<CtBlock>(%3);
            "#
        );

        cut_transfers(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                _consume<CtBlock>(%4);
                transfer_out<#1>(%2);
                %4 = transfer_in<#1>();
            "#
        );
    }
}
