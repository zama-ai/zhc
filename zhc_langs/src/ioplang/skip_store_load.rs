use crate::ioplang::{IopLang, IopTypeSystem};
use zhc_ir::{IR, ValId};
use zhc_utils::{FastMap, svec};

/// Eliminates redundant store-load pairs on ciphertext blocks.
///
/// Performs forward dataflow analysis to track which value was last stored at each block index
/// of each ciphertext. When an [`ExtractCtBlock`](super::IopInstructionSet::ExtractCtBlock)
/// reads from an index that was previously written by a
/// [`StoreCtBlock`](super::IopInstructionSet::StoreCtBlock), all uses of the extracted value
/// are replaced with the originally stored value, bypassing the memory round-trip.
pub fn skip_store_load(ir: &mut IR<IopLang>) {
    #[derive(Debug, PartialEq, Eq, Clone)]
    enum ValAnn {
        StoresBlocks(FastMap<u8, ValId>),
        ShouldBeReplaced(ValId),
        NotConcerned,
    }

    let ann_ir = ir.forward_dataflow_analysis(|_, valmap, op| {
        use super::IopInstructionSet::*;
        match op.get_instruction() {
            DeclareCiphertext => ((), svec![ValAnn::StoresBlocks(FastMap::new())]),
            Input { typ, .. } if typ == IopTypeSystem::Ciphertext => {
                ((), svec![ValAnn::StoresBlocks(FastMap::new())])
            }
            StoreCtBlock { index } => {
                let ValAnn::StoresBlocks(map) = valmap.get(&op.get_arg_valids()[1]).unwrap() else {
                    panic!()
                };
                let mut map = map.clone();
                map.insert(index, op.get_arg_valids()[0]);
                ((), svec![ValAnn::StoresBlocks(map)])
            }
            ExtractCtBlock { index } => {
                let ValAnn::StoresBlocks(map) = valmap.get(&op.get_arg_valids()[0]).unwrap() else {
                    panic!()
                };
                match map.get(&index) {
                    Some(valid) => ((), svec![ValAnn::ShouldBeReplaced(*valid)]),
                    None => ((), svec![ValAnn::NotConcerned]),
                }
            }
            _ => ((), svec![ValAnn::NotConcerned; op.get_return_arity()]),
        }
    });

    let valanns = ann_ir.into_valmap();

    for (old_valid, ann) in valanns.into_iter() {
        if let ValAnn::ShouldBeReplaced(new_valid) = ann {
            ir.replace_val_use(old_valid, new_valid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ioplang::IopInstructionSet;
    use zhc_ir::{IRError, dce::eliminate_dead_code};
    use zhc_utils::assert_display_is;

    /// Store then extract at the same index: extract should be replaced by stored value
    #[test]
    fn test_simple_store_load_elimination() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, block) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 42 }, svec![])?;
        let (_, ct) = ir.add_op(IopInstructionSet::DeclareCiphertext, svec![])?;
        let (_, ct_stored) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![block[0], ct[0]],
        )?;
        let (_, extracted) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 0 },
            svec![ct_stored[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![extracted[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
                %0 : CtBlock = let_ct_block<42>();
                %1 : Ct = decl_ct();
                %2 : Ct = store_ct_block<0>(%0 : CtBlock, %1 : Ct);
                %3 : CtBlock = extract_ct_block<0>(%2 : Ct);
                output<0, CtBlock>(%3 : CtBlock);
            "#
        );

        skip_store_load(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = let_ct_block<42>();
            output<0, CtBlock>(%0 : CtBlock);
            "#
        );

        Ok(())
    }

    /// Extract from an index that was never stored: no replacement
    #[test]
    fn test_extract_unstored_index() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, block) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 42 }, svec![])?;
        let (_, ct) = ir.add_op(IopInstructionSet::DeclareCiphertext, svec![])?;
        let (_, ct_stored) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![block[0], ct[0]],
        )?;
        let (_, extracted) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 1 },
            svec![ct_stored[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![extracted[0]],
        )?;

        let before = ir.format().to_string();
        skip_store_load(&mut ir);
        let after = ir.format().to_string();

        assert_eq!(before, after);

        Ok(())
    }

    /// Multiple stores to different indices, extract from each
    #[test]
    fn test_multiple_stores_different_indices() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![])?;
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![])?;
        let (_, ct) = ir.add_op(IopInstructionSet::DeclareCiphertext, svec![])?;
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![b0[0], ct[0]],
        )?;
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 1 },
            svec![b1[0], ct1[0]],
        )?;
        let (_, e0) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 0 },
            svec![ct2[0]],
        )?;
        let (_, e1) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 1 },
            svec![ct2[0]],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![e0[0], e1[0]])?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
                %0 : CtBlock = let_ct_block<10>();
                %1 : CtBlock = let_ct_block<20>();
                %2 : Ct = decl_ct();
                %3 : Ct = store_ct_block<0>(%0 : CtBlock, %2 : Ct);
                %4 : Ct = store_ct_block<1>(%1 : CtBlock, %3 : Ct);
                %5 : CtBlock = extract_ct_block<0>(%4 : Ct);
                %6 : CtBlock = extract_ct_block<1>(%4 : Ct);
                %7 : CtBlock = add_ct(%5 : CtBlock, %6 : CtBlock);
                output<0, CtBlock>(%7 : CtBlock);
            "#
        );

        skip_store_load(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = let_ct_block<10>();
            %1 : CtBlock = let_ct_block<20>();
            %7 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
            output<0, CtBlock>(%7 : CtBlock);
            "#
        );

        Ok(())
    }

    /// Overwrite: store twice at the same index, extract gets the latest value
    #[test]
    fn test_overwrite_same_index() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![])?;
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![])?;
        let (_, ct) = ir.add_op(IopInstructionSet::DeclareCiphertext, svec![])?;
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![b0[0], ct[0]],
        )?;
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![b1[0], ct1[0]],
        )?;
        let (_, extracted) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 0 },
            svec![ct2[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![extracted[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
                %0 : CtBlock = let_ct_block<10>();
                %1 : CtBlock = let_ct_block<20>();
                %2 : Ct = decl_ct();
                %3 : Ct = store_ct_block<0>(%0 : CtBlock, %2 : Ct);
                %4 : Ct = store_ct_block<0>(%1 : CtBlock, %3 : Ct);
                %5 : CtBlock = extract_ct_block<0>(%4 : Ct);
                output<0, CtBlock>(%5 : CtBlock);
            "#
        );

        skip_store_load(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %1 : CtBlock = let_ct_block<20>();
            output<0, CtBlock>(%1 : CtBlock);
            "#
        );

        Ok(())
    }

    /// Input ciphertext: extracts should not be replaced (unknown contents)
    #[test]
    fn test_input_ciphertext_no_replacement() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, ct) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::Ciphertext,
            },
            svec![],
        )?;
        let (_, extracted) =
            ir.add_op(IopInstructionSet::ExtractCtBlock { index: 0 }, svec![ct[0]])?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![extracted[0]],
        )?;

        let before = ir.format().to_string();
        skip_store_load(&mut ir);
        let after = ir.format().to_string();

        assert_eq!(before, after);

        Ok(())
    }

    /// Input ciphertext then store: extract at stored index should be replaced
    #[test]
    fn test_input_ciphertext_then_store() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, ct) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::Ciphertext,
            },
            svec![],
        )?;
        let (_, block) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 99 }, svec![])?;
        let (_, ct_stored) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![block[0], ct[0]],
        )?;
        let (_, extracted) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 0 },
            svec![ct_stored[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![extracted[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
                %0 : Ct = input<0, Ct>();
                %1 : CtBlock = let_ct_block<99>();
                %2 : Ct = store_ct_block<0>(%1 : CtBlock, %0 : Ct);
                %3 : CtBlock = extract_ct_block<0>(%2 : Ct);
                output<0, CtBlock>(%3 : CtBlock);
            "#
        );

        skip_store_load(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %1 : CtBlock = let_ct_block<99>();
            output<0, CtBlock>(%1 : CtBlock);
            "#
        );

        Ok(())
    }

    /// No store/load pairs: no-op
    #[test]
    fn test_no_store_load() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, b1) = ir.add_op(
            IopInstructionSet::Input {
                pos: 1,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![b0[0], b1[0]])?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        )?;

        let before = ir.format().to_string();
        skip_store_load(&mut ir);
        let after = ir.format().to_string();

        assert_eq!(before, after);

        Ok(())
    }

    /// Extract from intermediate ciphertext version (not the latest)
    #[test]
    fn test_extract_from_intermediate_version() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, b0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 10 }, svec![])?;
        let (_, b1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 20 }, svec![])?;
        let (_, ct) = ir.add_op(IopInstructionSet::DeclareCiphertext, svec![])?;
        let (_, ct1) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![b0[0], ct[0]],
        )?;
        let (_, ct2) = ir.add_op(
            IopInstructionSet::StoreCtBlock { index: 0 },
            svec![b1[0], ct1[0]],
        )?;
        let (_, e1) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 0 },
            svec![ct1[0]],
        )?;
        let (_, e2) = ir.add_op(
            IopInstructionSet::ExtractCtBlock { index: 0 },
            svec![ct2[0]],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![e1[0], e2[0]])?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
                %0 : CtBlock = let_ct_block<10>();
                %1 : CtBlock = let_ct_block<20>();
                %2 : Ct = decl_ct();
                %3 : Ct = store_ct_block<0>(%0 : CtBlock, %2 : Ct);
                %4 : Ct = store_ct_block<0>(%1 : CtBlock, %3 : Ct);
                %5 : CtBlock = extract_ct_block<0>(%3 : Ct);
                %6 : CtBlock = extract_ct_block<0>(%4 : Ct);
                %7 : CtBlock = add_ct(%5 : CtBlock, %6 : CtBlock);
                output<0, CtBlock>(%7 : CtBlock);
            "#
        );

        skip_store_load(&mut ir);
        eliminate_dead_code(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = let_ct_block<10>();
            %1 : CtBlock = let_ct_block<20>();
            %7 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
            output<0, CtBlock>(%7 : CtBlock);
            "#
        );

        Ok(())
    }
}
