use crate::ioplang::IopLang;
use hc_ir::{IR, ValId};
use hc_utils::svec;

/// Removes all `Alias` operations from the IR.
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

    let ann_ir = ir.forward_dataflow_analysis(|_, valmap, op| {
        use super::IopInstructionSet::*;
        match op.get_operation() {
            Alias { .. } => {
                let valid = match valmap[op.get_arg_valids()[0]] {
                    ValAction::Keep(valid) => valid,
                    ValAction::ReplaceWith(valid) => valid,
                };
                (OpAction::Delete, svec![ValAction::ReplaceWith(valid)])
            }
            _ => (
                OpAction::Keep,
                op.get_returns_iter()
                    .map(|val| ValAction::Keep(*val))
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
    use hc_ir::IRError;
    use hc_utils::assert_display_is;

    /// Single alias should be eliminated, replacing all uses with the original value
    #[test]
    fn test_single_alias_elimination() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, aliased) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![aliased[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = alias(%0 : CtBlock);
            output<0, CtBlock>(%1 : CtBlock);
        "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            output<0, CtBlock>(%0 : CtBlock);
        "#
        );

        Ok(())
    }

    /// Chain of aliases: a -> alias -> alias -> alias -> use
    /// All aliases should be eliminated, final use should point to original
    #[test]
    fn test_chained_aliases() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, a1) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        )?;
        let (_, a2) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a1[0]],
        )?;
        let (_, a3) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a2[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a3[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = alias(%0 : CtBlock);
            %2 : CtBlock = alias(%1 : CtBlock);
            %3 : CtBlock = alias(%2 : CtBlock);
            output<0, CtBlock>(%3 : CtBlock);
        "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            output<0, CtBlock>(%0 : CtBlock);
        "#
        );

        Ok(())
    }

    /// Alias with multiple uses: all uses should be redirected
    #[test]
    fn test_alias_multiple_uses() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, aliased) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![aliased[0], aliased[0]])?;
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
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = alias(%0 : CtBlock);
            %2 : CtBlock = add_ct(%1 : CtBlock, %1 : CtBlock);
            output<0, CtBlock>(%2 : CtBlock);
        "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %2 : CtBlock = add_ct(%0 : CtBlock, %0 : CtBlock);
            output<0, CtBlock>(%2 : CtBlock);
        "#
        );

        Ok(())
    }

    /// Multiple independent aliases from different sources
    #[test]
    fn test_multiple_independent_aliases() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp0) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, inp1) = ir.add_op(
            IopInstructionSet::Input {
                pos: 1,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, a0) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp0[0]],
        )?;
        let (_, a1) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp1[0]],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![a0[0], a1[0]])?;
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
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = input<1, CtBlock>();
            %2 : CtBlock = alias(%0 : CtBlock);
            %3 : CtBlock = alias(%1 : CtBlock);
            %4 : CtBlock = add_ct(%2 : CtBlock, %3 : CtBlock);
            output<0, CtBlock>(%4 : CtBlock);
        "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = input<1, CtBlock>();
            %4 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
            output<0, CtBlock>(%4 : CtBlock);
        "#
        );

        Ok(())
    }

    /// No aliases in IR: should be a no-op
    #[test]
    fn test_no_aliases() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp0) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, inp1) = ir.add_op(
            IopInstructionSet::Input {
                pos: 1,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![inp0[0], inp1[0]])?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        )?;

        let before = ir.format().to_string();
        eliminate_aliases(&mut ir);
        let after = ir.format().to_string();

        assert_eq!(before, after);

        Ok(())
    }

    /// Diamond pattern with aliases:
    /// inp -> alias1 -> add
    ///    \-> alias2 -/
    #[test]
    fn test_diamond_aliases() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, a1) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        )?;
        let (_, a2) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp[0]],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![a1[0], a2[0]])?;
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
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = alias(%0 : CtBlock);
            %2 : CtBlock = alias(%0 : CtBlock);
            %3 : CtBlock = add_ct(%1 : CtBlock, %2 : CtBlock);
            output<0, CtBlock>(%3 : CtBlock);
        "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %3 : CtBlock = add_ct(%0 : CtBlock, %0 : CtBlock);
            output<0, CtBlock>(%3 : CtBlock);
        "#
        );

        Ok(())
    }

    /// Mixed chain: real op between aliases
    /// inp -> alias -> add_ct -> alias -> output
    #[test]
    fn test_alias_interleaved_with_ops() -> Result<(), IRError<IopLang>> {
        let mut ir: IR<IopLang> = IR::empty();

        let (_, inp0) = ir.add_op(
            IopInstructionSet::Input {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, inp1) = ir.add_op(
            IopInstructionSet::Input {
                pos: 1,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![],
        )?;
        let (_, a0) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![inp0[0]],
        )?;
        let (_, sum) = ir.add_op(IopInstructionSet::AddCt, svec![a0[0], inp1[0]])?;
        let (_, a1) = ir.add_op(
            IopInstructionSet::Alias {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![sum[0]],
        )?;
        ir.add_op(
            IopInstructionSet::Output {
                pos: 0,
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![a1[0]],
        )?;

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = input<1, CtBlock>();
            %2 : CtBlock = alias(%0 : CtBlock);
            %3 : CtBlock = add_ct(%2 : CtBlock, %1 : CtBlock);
            %4 : CtBlock = alias(%3 : CtBlock);
            output<0, CtBlock>(%4 : CtBlock);
        "#
        );

        eliminate_aliases(&mut ir);

        assert_display_is!(
            ir.format(),
            r#"
            %0 : CtBlock = input<0, CtBlock>();
            %1 : CtBlock = input<1, CtBlock>();
            %3 : CtBlock = add_ct(%0 : CtBlock, %1 : CtBlock);
            output<0, CtBlock>(%3 : CtBlock);
        "#
        );

        Ok(())
    }
}
