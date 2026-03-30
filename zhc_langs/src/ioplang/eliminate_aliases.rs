use crate::ioplang::IopLang;
use zhc_ir::{IR, ValId};
use zhc_utils::svec;

/// Removes all alias operations from the IR.
///
/// Each alias output value is replaced by the alias's input value throughout the IR, transitively
/// collapsing alias chains. The alias operations themselves are deleted. All other operations
/// remain unchanged. The transformation is applied in-place to `ir`.
pub fn eliminate_aliases(ir: &mut IR<IopLang>) {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum OpAction {
        Keep,
        Delete,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum ValAction {
        Keep(ValId),
        ReplaceWith(ValId),
    }

    let ann_ir = ir.forward_dataflow_analysis(|op| {
        use super::IopInstructionSet::*;
        match op.get_instruction() {
            Inspect { .. } => {
                let valid = match op
                    .get_args_iter()
                    .next()
                    .unwrap()
                    .get_annotation()
                    .clone()
                    .unwrap_analyzed()
                {
                    ValAction::Keep(valid) => valid,
                    ValAction::ReplaceWith(valid) => valid,
                };
                (OpAction::Delete, svec![ValAction::ReplaceWith(valid)])
            }
            _ => (
                OpAction::Keep,
                op.get_returns_iter()
                    .map(|val| ValAction::Keep(val.get_id()))
                    .collect(),
            ),
        }
    });
    let (opactions, valactions) = ann_ir.into_maps();

    for (old_valid, action) in valactions.into_iter() {
        if let ValAction::ReplaceWith(new_valid) = action {
            ir.replace_val_use(old_valid, new_valid);
        }
    }

    for (opid, action) in opactions.into_iter() {
        if let OpAction::Delete = action {
            ir.delete_op(opid);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ioplang::{IopInstructionSet, IopTypeSystem};

    use super::*;

    use zhc_utils::assert_display_is;

    /// Single alias should be eliminated, replacing all uses with the original value
    #[test]
    fn test_single_alias_elimination() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, aliased) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        );
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![aliased[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = inspect(%0);
                _consume<CtBlock>(%1);
            "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                _consume<CtBlock>(%0);
            "#
        );
    }

    /// Chain of aliases: a -> alias -> alias -> alias -> use
    /// All aliases should be eliminated, final use should point to original
    #[test]
    fn test_chained_aliases() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, a1) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        );
        let (_, a2) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a1[0]],
        );
        let (_, a3) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a2[0]],
        );
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a3[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = inspect(%0);
                %2 = inspect(%1);
                %3 = inspect(%2);
                _consume<CtBlock>(%3);
            "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                _consume<CtBlock>(%0);
            "#
        );
    }

    /// Alias with multiple uses: all uses should be redirected
    #[test]
    fn test_alias_multiple_uses() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, aliased) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        );
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![aliased[0], aliased[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = inspect(%0);
                %2 = add_ct(%1, %1);
                _consume<CtBlock>(%2);
            "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %2 = add_ct(%0, %0);
                _consume<CtBlock>(%2);
            "#
        );
    }

    /// Multiple independent aliases from different sources
    #[test]
    fn test_multiple_independent_aliases() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, inp1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 1 }, svec![]);
        let (_, a0) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp0[0]],
        );
        let (_, a1) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp1[0]],
        );
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![a0[0], a1[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = let_ct_block<1>();
                %2 = inspect(%0);
                %3 = inspect(%1);
                %4 = add_ct(%2, %3);
                _consume<CtBlock>(%4);
            "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = let_ct_block<1>();
                %4 = add_ct(%0, %1);
                _consume<CtBlock>(%4);
            "#
        );
    }

    /// No aliases in IR: should be a no-op
    #[test]
    fn test_no_aliases() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, inp1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 1 }, svec![]);
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![inp0[0], inp1[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );

        let before = ir.format().to_string();
        eliminate_aliases(&mut ir);
        let after = ir.format().to_string();

        assert_eq!(before, after);
    }

    /// Diamond pattern with aliases:
    /// inp -> alias1 -> add
    ///    \-> alias2 -/
    #[test]
    fn test_diamond_aliases() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, a1) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        );
        let (_, a2) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        );
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![a1[0], a2[0]]);
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = inspect(%0);
                %2 = inspect(%0);
                %3 = add_ct(%1, %2);
                _consume<CtBlock>(%3);
            "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %3 = add_ct(%0, %0);
                _consume<CtBlock>(%3);
            "#
        );
    }

    /// Mixed chain: real op between aliases
    /// inp -> alias -> add_ct -> alias -> output
    #[test]
    fn test_alias_interleaved_with_ops() {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp0) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 0 }, svec![]);
        let (_, inp1) = ir.add_op(IopInstructionSet::LetCiphertextBlock { value: 1 }, svec![]);
        let (_, a0) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp0[0]],
        );
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![a0[0], inp1[0]]);
        let (_, a1) = ir.add_op(
            IopInstructionSet::Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        );
        ir.add_op(
            IopInstructionSet::_Consume {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a1[0]],
        );

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = let_ct_block<1>();
                %2 = inspect(%0);
                %3 = add_ct(%2, %1);
                %4 = inspect(%3);
                _consume<CtBlock>(%4);
            "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
                %0 = let_ct_block<0>();
                %1 = let_ct_block<1>();
                %3 = add_ct(%0, %1);
                _consume<CtBlock>(%3);
            "#
        );
    }
}
