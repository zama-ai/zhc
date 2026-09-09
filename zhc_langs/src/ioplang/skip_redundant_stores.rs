use crate::ioplang::IopLang;
use zhc_ir::{Analysing, IR, ValId};
use zhc_utils::{small::SmallSet, svec};

/// Eliminates redundant store operations that are overwritten before output.
///
/// Performs backward dataflow analysis from
/// [`OutputIntegerCiphertext`](super::IopInstructionSet::OutputIntegerCiphertext) operations, tracking which
/// block indices have been stored. When a [`StoreIntegerCiphertextBlock`](super::IopInstructionSet::StoreIntegerCiphertextBlock)
/// writes to an index that will be overwritten by a later store before the output, the earlier
/// store is redundant and removed.
pub fn skip_redundant_stores(ir: &mut IR<IopLang>) {
    #[derive(Debug, PartialEq, Eq, Clone)]
    enum OpAnn {
        ShouldKeep(SmallSet<u8>),
        ShouldRemove(SmallSet<u8>, ValId, ValId),
        NotConcerned,
    }

    let ann_ir = ir.backward_dataflow_analysis(|op| {
        use super::IopInstructionSet::*;
        match op.get_instruction() {
            OutputIntegerCiphertext { .. } => (
                OpAnn::ShouldKeep(SmallSet::new()),
                svec![(); op.get_return_arity()],
            ),
            StoreIntegerCiphertextBlock { index } => {
                // (CiphertextBlock, IntegerCiphertext) -> (IntegerCiphertext)
                // We assume linear chains for stores
                let output_ct = op.get_returns_iter().next().unwrap();

                let mut set = match output_ct.get_users_iter().count() {
                    0 => SmallSet::new(),
                    1 => {
                        let next_op = output_ct.get_users_iter().next().unwrap();
                        match next_op.get_annotation() {
                            Analysing::Analyzed(OpAnn::ShouldKeep(set))
                            | Analysing::Analyzed(OpAnn::ShouldRemove(set, ..)) => set.clone(),
                            Analysing::Analyzed(OpAnn::NotConcerned) => SmallSet::new(),
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        panic!("Expected a linear chain of stores, but found a branch.");
                    }
                };
                if set.contains(&index) {
                    let input_ct_valid = op.get_args_iter().nth(1).unwrap().get_id();
                    let output_ct_valid = output_ct.get_id();
                    (
                        OpAnn::ShouldRemove(set, input_ct_valid, output_ct_valid),
                        svec![(); op.get_return_arity()],
                    )
                } else {
                    set.insert(*index);
                    (OpAnn::ShouldKeep(set), svec![(); op.get_return_arity()])
                }
            }
            _ => (OpAnn::NotConcerned, svec![(); op.get_return_arity()]),
        }
    });

    let opanns = ann_ir.into_opmap();

    for (_, ann) in opanns.into_iter().rev() {
        if let OpAnn::ShouldRemove(_, input_ct_valid, output_ct_valid) = ann {
            ir.replace_val_use(output_ct_valid, input_ct_valid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ioplang::{IopInstructionSet, IopTypeSystem};
    use zhc_ir::dce::eliminate_dead_code;
    use zhc_utils::assert_display_is;

    /// Store twice to the same index before output: first store should be removed
    #[test]
    fn test_simple_redundant_store() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![]);
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![]);
        let (_, ct) = ir.add_op(
            IopInstructionSet::DeclareIntegerCiphertext { int_size: 2 },
            svec![],
        );
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b0[0], ct[0]],
        );
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b1[0], ct1[0]],
        );
        ir.add_op(
            IopInstructionSet::OutputIntegerCiphertext { pos: 0 },
            svec![ct2[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<10>();
                %1 = let_ct_block<20>();
                %2 = decl_ct<2>();
                %3 = store_ct_block<0>(%0, %2);
                %4 = store_ct_block<0>(%1, %3);
                output<0>(%4);
            "#
        );

        skip_redundant_stores(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %1 = let_ct_block<20>();
                %2 = decl_ct<2>();
                %4 = store_ct_block<0>(%1, %2);
                output<0>(%4);
            "#
        );
    }

    /// Store to different indices: no redundancy, nothing removed
    #[test]
    fn test_stores_different_indices() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![]);
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![]);
        let (_, ct) = ir.add_op(
            IopInstructionSet::DeclareIntegerCiphertext { int_size: 4 },
            svec![],
        );
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b0[0], ct[0]],
        );
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 1 },
            svec![b1[0], ct1[0]],
        );
        ir.add_op(
            IopInstructionSet::OutputIntegerCiphertext { pos: 0 },
            svec![ct2[0]],
        );

        skip_redundant_stores(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<10>();
                %1 = let_ct_block<20>();
                %2 = decl_ct<4>();
                %3 = store_ct_block<0>(%0, %2);
                %4 = store_ct_block<1>(%1, %3);
                output<0>(%4);
            "#
        );
    }

    /// Store three times to same index: first two are redundant
    #[test]
    fn test_multiple_redundant_stores() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![]);
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![]);
        let (_, b2) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 30 }, svec![]);
        let (_, ct) = ir.add_op(
            IopInstructionSet::DeclareIntegerCiphertext { int_size: 2 },
            svec![],
        );
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b0[0], ct[0]],
        );
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b1[0], ct1[0]],
        );
        let (_, ct3) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b2[0], ct2[0]],
        );
        ir.add_op(
            IopInstructionSet::OutputIntegerCiphertext { pos: 0 },
            svec![ct3[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<10>();
                %1 = let_ct_block<20>();
                %2 = let_ct_block<30>();
                %3 = decl_ct<2>();
                %4 = store_ct_block<0>(%0, %3);
                %5 = store_ct_block<0>(%1, %4);
                %6 = store_ct_block<0>(%2, %5);
                output<0>(%6);
            "#
        );

        skip_redundant_stores(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %2 = let_ct_block<30>();
                %3 = decl_ct<2>();
                %6 = store_ct_block<0>(%2, %3);
                output<0>(%6);
            "#
        );
    }

    /// Mixed: one index stored twice (redundant), another stored once (not redundant)
    #[test]
    fn test_mixed_redundant_and_not() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![]);
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![]);
        let (_, b2) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 30 }, svec![]);
        let (_, ct) = ir.add_op(
            IopInstructionSet::DeclareIntegerCiphertext { int_size: 4 },
            svec![],
        );
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b0[0], ct[0]],
        );
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 1 },
            svec![b1[0], ct1[0]],
        );
        let (_, ct3) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b2[0], ct2[0]],
        );
        ir.add_op(
            IopInstructionSet::OutputIntegerCiphertext { pos: 0 },
            svec![ct3[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<10>();
                %1 = let_ct_block<20>();
                %2 = let_ct_block<30>();
                %3 = decl_ct<4>();
                %4 = store_ct_block<0>(%0, %3);
                %5 = store_ct_block<1>(%1, %4);
                %6 = store_ct_block<0>(%2, %5);
                output<0>(%6);
            "#
        );

        skip_redundant_stores(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %1 = let_ct_block<20>();
                %2 = let_ct_block<30>();
                %3 = decl_ct<4>();
                %5 = store_ct_block<1>(%1, %3);
                %6 = store_ct_block<0>(%2, %5);
                output<0>(%6);
            "#
        );
    }

    /// No stores, just output: no-op
    #[test]
    fn test_no_stores() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, ct) = ir.add_op(
            IopInstructionSet::InputIntegerCiphertext {
                pos: 0,
                int_size: 2,
            },
            svec![],
        );
        ir.add_op(
            IopInstructionSet::OutputIntegerCiphertext { pos: 0 },
            svec![ct[0]],
        );

        skip_redundant_stores(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = input_ciphertext<0, 2>();
                output<0>(%0);
            "#
        );
    }

    /// Redundant stores with _Consume sink (not OutputIntegerCiphertext)
    #[test]
    fn test_consume_sink() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![]);
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![]);
        let (_, ct) = ir.add_op(
            IopInstructionSet::DeclareIntegerCiphertext { int_size: 2 },
            svec![],
        );
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b0[0], ct[0]],
        );
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b1[0], ct1[0]],
        );
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::IntegerCiphertext,
            },
            svec![ct2[0]],
        );

        skip_redundant_stores(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %1 = let_ct_block<20>();
                %2 = decl_ct<2>();
                %4 = store_ct_block<0>(%1, %2);
                _consume<IntegerCiphertext>(%4);
            "#
        );
    }

    /// Input ciphertext then stores with redundancy
    #[test]
    fn test_input_then_redundant_stores() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, ct) = ir.add_op(
            IopInstructionSet::InputIntegerCiphertext {
                pos: 0,
                int_size: 2,
            },
            svec![],
        );
        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![]);
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![]);
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b0[0], ct[0]],
        );
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreIntegerCiphertextBlock { index: 0 },
            svec![b1[0], ct1[0]],
        );
        ir.add_op(
            IopInstructionSet::OutputIntegerCiphertext { pos: 0 },
            svec![ct2[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = input_ciphertext<0, 2>();
                %1 = let_ct_block<10>();
                %2 = let_ct_block<20>();
                %3 = store_ct_block<0>(%1, %0);
                %4 = store_ct_block<0>(%2, %3);
                output<0>(%4);
            "#
        );

        skip_redundant_stores(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = input_ciphertext<0, 2>();
                %2 = let_ct_block<20>();
                %4 = store_ct_block<0>(%2, %0);
                output<0>(%4);
            "#
        );
    }
}
