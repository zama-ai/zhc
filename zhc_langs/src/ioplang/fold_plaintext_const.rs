use crate::ioplang::{IopInstructionSet, IopLang};
use zhc_crypto::integer_semantics::{EmulatedPlaintext, PlaintextBlockSpec};
use zhc_ir::{IR, ValId};
use zhc_utils::{SafeAs, svec};

/// Folds constant plaintext block extractions into block constants.
///
/// Performs forward dataflow analysis to recover, for every
/// [`LetPlaintext`](IopInstructionSet::LetPlaintext), the concrete
/// [`EmulatedPlaintext`] it denotes (using `block_spec` to fix the radix). Each
/// [`ExtractPtBlock`](IopInstructionSet::ExtractPtBlock) reading from such a constant is then
/// replaced by a freshly inserted [`LetPlaintextBlock`](IopInstructionSet::LetPlaintextBlock)
/// carrying the decomposed block value, computed exactly as the interpreter would via
/// [`EmulatedPlaintext::get_block`].
///
/// The now-unused `LetPlaintext` and `ExtractPtBlock` operations are left for dead-code
/// elimination to remove.
pub fn fold_plaintext_const(ir: &mut IR<IopLang>, block_spec: PlaintextBlockSpec) {
    #[derive(Debug, PartialEq, Eq, Clone)]
    enum ValAnn {
        /// The value is a constant plaintext with the given concrete value.
        ConstPlaintext(EmulatedPlaintext),
        NotConcerned,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum OpAnn {
        /// This `ExtractPtBlock` reads block `value` (already decomposed) from a constant
        /// plaintext; its output `valid` should be replaced by a fresh block constant.
        Fold { value: u8, valid: ValId },
        NotConcerned,
    }

    let ann_ir = ir.forward_dataflow_analysis(|op| {
        use IopInstructionSet::*;
        match op.get_instruction() {
            LetPlaintext { int_size, value } => {
                let pt = block_spec.plaintext_spec(int_size).from_int(value);
                (OpAnn::NotConcerned, svec![ValAnn::ConstPlaintext(pt)])
            }
            ExtractPtBlock { index } => {
                let input = op
                    .get_args_iter()
                    .next()
                    .unwrap()
                    .get_annotation()
                    .clone()
                    .unwrap_analyzed();
                match input {
                    ValAnn::ConstPlaintext(pt) => {
                        let value = pt.get_block(index).raw_message_bits().sas::<u8>();
                        let valid = op.get_returns_iter().next().unwrap().get_id();
                        (OpAnn::Fold { value, valid }, svec![ValAnn::NotConcerned])
                    }
                    ValAnn::NotConcerned => {
                        (OpAnn::NotConcerned, svec![ValAnn::NotConcerned])
                    }
                }
            }
            _ => (
                OpAnn::NotConcerned,
                svec![ValAnn::NotConcerned; op.get_return_arity()],
            ),
        }
    });

    let (opanns, _) = ann_ir.into_maps();

    for (_, ann) in opanns.into_iter() {
        if let OpAnn::Fold { value, valid } = ann {
            let (_, new) = ir.add_op(IopInstructionSet::LetPlaintextBlock { value }, svec![]);
            ir.replace_val_use(valid, new[0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ioplang::IopTypeSystem;
    use zhc_crypto::integer_semantics::PlaintextBlockSpec;
    use zhc_ir::{PrintWalker, dce::eliminate_dead_code};
    use zhc_utils::assert_display_is;

    /// A single extraction from a constant plaintext is folded to a block constant.
    ///
    /// With 2-bit blocks, `0b11_10_01_00 = 0xE4` decomposes (LSB first) into blocks
    /// `0, 1, 2, 3`. Extracting block 2 yields `2`.
    #[test]
    fn test_single_extraction() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, pt) = ir.add_op(
            IopInstructionSet::LetPlaintext {
                int_size: 8,
                value: 0xE4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 2 }, svec![pt[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::PlaintextBlock,
            },
            svec![blk[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_pt<8, 228>();
                %1 = extract_pt_block<2>(%0);
                _consume<PtBlock>(%1);
            "#
        );

        fold_plaintext_const(&mut ir, PlaintextBlockSpec(2));
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Topo),
            r#"
                %2 = let_pt_block<2>();
                _consume<PtBlock>(%2);
            "#
        );
    }

    /// Every block of a constant plaintext is folded independently.
    #[test]
    fn test_full_decomposition() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, pt) = ir.add_op(
            IopInstructionSet::LetPlaintext {
                int_size: 8,
                value: 0xE4,
            },
            svec![],
        );
        let (_, b0) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 0 }, svec![pt[0]]);
        let (_, b1) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 1 }, svec![pt[0]]);
        let (_, b2) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 2 }, svec![pt[0]]);
        let (_, b3) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 3 }, svec![pt[0]]);
        for b in [b0, b1, b2, b3] {
            ir.add_op(
                IopInstructionSet::_Consume {
                    typ: IopTypeSystem::PlaintextBlock,
                },
                svec![b[0]],
            );
        }

        fold_plaintext_const(&mut ir, PlaintextBlockSpec(2));
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Topo),
            r#"
                %5 = let_pt_block<0>();
                %6 = let_pt_block<1>();
                %7 = let_pt_block<2>();
                %8 = let_pt_block<3>();
                _consume<PtBlock>(%5);
                _consume<PtBlock>(%6);
                _consume<PtBlock>(%7);
                _consume<PtBlock>(%8);
            "#
        );
    }

    /// Extraction from a non-constant plaintext (a program input) is left untouched.
    #[test]
    fn test_input_plaintext_not_folded() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, pt) = ir.add_op(
            IopInstructionSet::InputPlaintext {
                pos: 0,
                int_size: 8,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 0 }, svec![pt[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::PlaintextBlock,
            },
            svec![blk[0]],
        );

        let before = ir.format().to_string();
        fold_plaintext_const(&mut ir, PlaintextBlockSpec(2));
        let after = ir.format().to_string();

        assert_eq!(before, after);
    }

    /// A 4-bit block spec slices the value with a wider radix.
    ///
    /// `0xAB = 0b1010_1011` decomposes (LSB first) into blocks `0xB, 0xA`.
    #[test]
    fn test_wider_block_spec() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, pt) = ir.add_op(
            IopInstructionSet::LetPlaintext {
                int_size: 8,
                value: 0xAB,
            },
            svec![],
        );
        let (_, b0) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 0 }, svec![pt[0]]);
        let (_, b1) = ir.add_op(IopInstructionSet::ExtractPtBlock { index: 1 }, svec![pt[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::PlaintextBlock,
            },
            svec![b0[0]],
        );
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::PlaintextBlock,
            },
            svec![b1[0]],
        );

        fold_plaintext_const(&mut ir, PlaintextBlockSpec(4));
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format().with_walker(PrintWalker::Topo),
            r#"
                %3 = let_pt_block<11>();
                %4 = let_pt_block<10>();
                _consume<PtBlock>(%3);
                _consume<PtBlock>(%4);
            "#
        );
    }

    /// No constant plaintexts: the pass is a no-op.
    #[test]
    fn test_no_constants() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, blk) = ir.add_op(IopInstructionSet::LetPlaintextBlock { value: 3 }, svec![]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::PlaintextBlock,
            },
            svec![blk[0]],
        );

        let before = ir.format().to_string();
        fold_plaintext_const(&mut ir, PlaintextBlockSpec(2));
        let after = ir.format().to_string();

        assert_eq!(before, after);
    }
}
