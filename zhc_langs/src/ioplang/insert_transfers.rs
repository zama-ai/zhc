use zhc_ir::{IR, OpIdRaw, OpMap, ValUse, cse};
use zhc_utils::{iter::CollectInSmallVec, svec};

use crate::ioplang::{IopInstructionSet, IopLang};

pub type Partition = OpIdRaw;

pub fn insert_transfers(ir: &mut IR<IopLang>, partitions: OpMap<Partition>) {
    let val_uses_to_transfer = ir
        .walk_vals_linear()
        .flat_map(|val| val.get_uses_iter().map(move |uze| (val.clone(), uze)))
        .filter(|(val, uze)| partitions[val.get_origin().opref] != partitions[*uze.opref])
        .map(|(val, uze)| (val.get_id(), ValUse{ opid: uze.opref.get_id(), position: uze.position}))
        .cosvec();

    for (valid, val_use) in val_uses_to_transfer.into_iter() {
        let (_,valids) = ir.add_op(IopInstructionSet::Transfer, svec![valid]);
        ir.replace_val_use_at(val_use, valids[0]);
    }
}

#[cfg(test)]
mod test {
    use zhc_ir::{IR, PrintWalker};
    use zhc_utils::{assert_display_is, svec};

    use crate::ioplang::{IopInstructionSet, IopLang, IopTypeSystem, insert_transfers};

    #[test]
    fn test_no_transfer_within_single_partition() {
        use IopInstructionSet::*;

        let mut ir: IR<IopLang> = IR::empty();
        let (_, l) = ir.add_op(LetCiphertextBlock { value: 3 }, svec![]);
        let (_, r) = ir.add_op(LetCiphertextBlock { value: 5 }, svec![]);
        let (_, sum) = ir.add_op(AddCt, svec![l[0], r[0]]);
        ir.add_op(
            _Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );
        let partitions = ir.totally_mapped_opmap(|_| 0);
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                _consume<CtBlock>(%2);
            "#
        );

        insert_transfers(&mut ir, partitions);

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                _consume<CtBlock>(%2);
            "#
        );
    }

    #[test]
    fn test_transfer_on_crossing_edge() {
        use IopInstructionSet::*;

        let mut ir: IR<IopLang> = IR::empty();
        let (_, l) = ir.add_op(LetCiphertextBlock { value: 3 }, svec![]);
        let (_, r) = ir.add_op(LetCiphertextBlock { value: 5 }, svec![]);
        let (op_add, sum) = ir.add_op(AddCt, svec![l[0], r[0]]);
        let (op_consume, _) = ir.add_op(
            _Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );

        let partitions = ir.totally_mapped_opmap(|op| {
            if op.get_id() == op_add || op.get_id() == op_consume {
                1
            } else {
                0
            }
        });

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                _consume<CtBlock>(%2);
            "#
        );
        insert_transfers(&mut ir, partitions);

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%3, %4);
                _consume<CtBlock>(%2);
                %3 = transfer(%0);
                %4 = transfer(%1);
            "#
        );
    }

    #[test]
    fn test_internal_use_preserved() {
        use IopInstructionSet::*;

        let mut ir: IR<IopLang> = IR::empty();
        let (_, a) = ir.add_op(LetCiphertextBlock { value: 3 }, svec![]);
        let (_, b) = ir.add_op(LetCiphertextBlock { value: 5 }, svec![]);
        // Internal consumer of %0 and %1, kept in partition 0.
        let (_, inner) = ir.add_op(AddCt, svec![a[0], b[0]]);
        // Crossing consumer of %0 and %2, placed in partition 1.
        let (op_cross, outer) = ir.add_op(AddCt, svec![a[0], inner[0]]);
        let (op_consume, _) = ir.add_op(
            _Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![outer[0]],
        );

        let partitions = ir.totally_mapped_opmap(|op| {
            if op.get_id() == op_cross || op.get_id() == op_consume {
                1
            } else {
                0
            }
        });

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                %3 = add_ct(%0, %2);
                _consume<CtBlock>(%3);
            "#
        );
        insert_transfers(&mut ir, partitions);

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<3>();
                %1 = let_ct_block<5>();
                %2 = add_ct(%0, %1);
                %3 = add_ct(%4, %5);
                _consume<CtBlock>(%3);
                %4 = transfer(%0);
                %5 = transfer(%2);
            "#
        );
    }

    #[test]
    fn test_fanout_inserts_one_transfer_per_use() {
        use IopInstructionSet::*;

        let mut ir: IR<IopLang> = IR::empty();
        let (_, src) = ir.add_op(LetCiphertextBlock { value: 7 }, svec![]);
        let (op_lit, lit) = ir.add_op(LetCiphertextBlock { value: 1 }, svec![]);
        // Two distinct partition-1 consumers of the partition-0 value %0.
        let (op_add, s) = ir.add_op(AddCt, svec![src[0], lit[0]]);
        let (op_sub, d) = ir.add_op(SubCt, svec![src[0], lit[0]]);
        let (op_join, j) = ir.add_op(AddCt, svec![s[0], d[0]]);
        let (op_consume, _) = ir.add_op(
            _Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![j[0]],
        );

        // Only %0's producer sits in partition 0; everything else in partition 1.
        let partitions = ir.totally_mapped_opmap(|op| {
            if op.get_id() == op_lit
                || op.get_id() == op_add
                || op.get_id() == op_sub
                || op.get_id() == op_join
                || op.get_id() == op_consume
            {
                1
            } else {
                0
            }
        });

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<7>();
                %1 = let_ct_block<1>();
                %2 = add_ct(%0, %1);
                %3 = sub_ct(%0, %1);
                %4 = add_ct(%2, %3);
                _consume<CtBlock>(%4);
            "#
        );
        insert_transfers(&mut ir, partitions);

        // %0 is transferred twice — once per crossing use (%5 for the add,
        // %6 for the sub) — rather than shared through a single transfer.
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = let_ct_block<7>();
                %1 = let_ct_block<1>();
                %2 = add_ct(%5, %1);
                %3 = sub_ct(%6, %1);
                %4 = add_ct(%2, %3);
                _consume<CtBlock>(%4);
                %5 = transfer(%0);
                %6 = transfer(%0);
            "#
        );
    }
}
