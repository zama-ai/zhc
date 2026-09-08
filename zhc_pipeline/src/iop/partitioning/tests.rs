
use zhc_crypto::integer_semantics::Flavor;
use zhc_ir::AnnIRView;
use zhc_langs::ioplang::IopInstructionSet;
use zhc_utils::{assert_display_is, svec};

use super::*;

#[test]
fn test_materialize_partitioning_replicates_shared_operations() {
    let p0 = PartitionId(0);
    let p1 = PartitionId(1);
    let mut ir = IR::empty();
    let (lhs, lhs_values) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 1 }, svec![]);
    let (rhs, rhs_values) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 2 }, svec![]);
    let (add, _) = ir.add_op(
        IopInstructionSet::AddCt {
            flavor: Flavor::Wrapping,
        },
        svec![lhs_values[0], rhs_values[0]],
    );

    let shared = PartitionPlacement::Shared([p0, p1].into_iter().collect());
    let mut placements = ir.empty_opmap();
    placements.insert(lhs, shared.clone());
    placements.insert(rhs, shared.clone());
    placements.insert(add, shared);

    let val_annotations = ir.filled_valmap(());
    assert_display_is!(
        AnnIRView::new(&ir, &placements, &val_annotations).format(),
        r#"
                %0 = let_ct_block<1>();
                    operation -> Shared(Stack({PartitionId(0), PartitionId(1)}))
                %1 = let_ct_block<2>();
                    operation -> Shared(Stack({PartitionId(0), PartitionId(1)}))
                %2 = wrapping_add_ct(%0, %1);
                    operation -> Shared(Stack({PartitionId(0), PartitionId(1)}))
            "#
    );

    let (materialized, partitions) = materialize_partitioning(&ir, &placements);
    let val_annotations = materialized.filled_valmap(());
    assert_display_is!(
        AnnIRView::new(&materialized, &partitions, &val_annotations).format(),
        r#"
                %0 = let_ct_block<1>();
                    operation -> PartitionId(0)
                %1 = let_ct_block<1>();
                    operation -> PartitionId(1)
                %2 = let_ct_block<2>();
                    operation -> PartitionId(0)
                %3 = let_ct_block<2>();
                    operation -> PartitionId(1)
                %4 = wrapping_add_ct(%0, %2);
                    operation -> PartitionId(0)
                %5 = wrapping_add_ct(%1, %3);
                    operation -> PartitionId(1)
            "#
    );
}

#[test]
fn test_materialize_partitioning_inserts_one_copy_per_destination() {
    let p0 = PartitionId(0);
    let p1 = PartitionId(1);
    let mut ir = IR::empty();
    let (lhs, lhs_values) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 1 }, svec![]);
    let (rhs, rhs_values) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 2 }, svec![]);
    let (add, _) = ir.add_op(
        IopInstructionSet::AddCt {
            flavor: Flavor::Wrapping,
        },
        svec![lhs_values[0], rhs_values[0]],
    );
    let (sub, _) = ir.add_op(
        IopInstructionSet::SubCt {
            flavor: Flavor::Wrapping,
        },
        svec![lhs_values[0], rhs_values[0]],
    );

    let mut placements = ir.empty_opmap();
    placements.insert(lhs, PartitionPlacement::Exclusive(p0));
    placements.insert(
        rhs,
        PartitionPlacement::Shared([p0, p1].into_iter().collect()),
    );
    placements.insert(add, PartitionPlacement::Exclusive(p1));
    placements.insert(sub, PartitionPlacement::Exclusive(p1));

    let val_annotations = ir.filled_valmap(());
    assert_display_is!(
        AnnIRView::new(&ir, &placements, &val_annotations).format(),
        r#"
                %0 = let_ct_block<1>();
                    operation -> Exclusive(PartitionId(0))
                %1 = let_ct_block<2>();
                    operation -> Shared(Stack({PartitionId(0), PartitionId(1)}))
                %2 = wrapping_add_ct(%0, %1);
                    operation -> Exclusive(PartitionId(1))
                %3 = wrapping_sub_ct(%0, %1);
                    operation -> Exclusive(PartitionId(1))
            "#
    );

    let (materialized, partitions) = materialize_partitioning(&ir, &placements);
    let val_annotations = materialized.filled_valmap(());
    assert_display_is!(
        AnnIRView::new(&materialized, &partitions, &val_annotations).format(),
        r#"
                %0 = let_ct_block<1>();
                    operation -> PartitionId(0)
                %1 = let_ct_block<2>();
                    operation -> PartitionId(0)
                %2 = let_ct_block<2>();
                    operation -> PartitionId(1)
                %3 = _copy(%0);
                    operation -> PartitionId(1)
                %4 = wrapping_add_ct(%3, %2);
                    operation -> PartitionId(1)
                %5 = wrapping_sub_ct(%3, %2);
                    operation -> PartitionId(1)
            "#
    );
}
