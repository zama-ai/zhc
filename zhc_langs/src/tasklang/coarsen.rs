use zhc_ir::{
    AnnOpRef, IR, ValId,
    translation::{Order, Translation, translate_ann},
    visualization::VisualAnnotation,
};
use zhc_utils::{
    SafeAs,
    iter::{CollectInSmallVec, CollectInVec, MultiZip},
    small::SmallSet,
    svec,
};

use crate::{
    ioplang::{IopInstructionSet, IopLang},
    tasklang::{TaskInstructionSet, TaskLang},
};

/// Set of PBS results that an IOP value is computed from through non-PBS operations only.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ann(SmallSet<ValId>);
impl VisualAnnotation for Ann {}

/// Coarsens an IOP program into a task graph with one task per PBS.
///
/// Each `Pbs`, `Pbs2`, `Pbs4` and `Pbs8` operation of `ir` gives one
/// [`Task`](TaskInstructionSet::Task) with a work of 1. The other operations give no task. The
/// arguments of a task are the distinct PBS results that the input of its PBS is computed from
/// through non-PBS operations only, in unspecified order. The results of a task are the results
/// of its PBS that are arguments of some task, in declaration order. The returned [`Translation`]
/// maps each task to its PBS, and each task result to its PBS result.
///
/// # Panics
///
/// Panics if a task has more than `u16::MAX` arguments.
pub fn coarsen_ioplang(ir: &IR<IopLang>) -> Translation<TaskLang> {
    use IopInstructionSet::*;
    let analyzed = ir.forward_dataflow_analysis(
        |opref: AnnOpRef<'_, '_, IopLang, zhc_ir::Analysing<()>, zhc_ir::Analysing<Ann>>| {
            match opref.get_instruction() {
                Pbs { .. } | Pbs2 { .. } | Pbs4 { .. } | Pbs8 { .. } => {
                    let ann = opref
                        .get_returns_iter()
                        .map(|valref| {
                            let mut oup = SmallSet::new();
                            oup.insert(valref.get_id());
                            Ann(oup)
                        })
                        .cosvec();
                    ((), ann)
                }
                _ => {
                    let oup: SmallSet<ValId> = opref
                        .get_args_iter()
                        .flat_map(|arg| {
                            arg.get_annotation().clone().unwrap_analyzed().0.into_iter()
                        })
                        .collect();

                    ((), svec![Ann(oup); opref.get_return_arity()])
                }
            }
        },
    );

    let mut used: SmallSet<ValId> = SmallSet::new();
    for opref in analyzed.walk_ops_linear() {
        if matches!(
            opref.get_instruction(),
            Pbs { .. } | Pbs2 { .. } | Pbs4 { .. } | Pbs8 { .. }
        ) {
            for va in opref
                .get_args_iter()
                .next()
                .unwrap()
                .get_annotation()
                .0
                .iter()
            {
                used.insert(*va);
            }
        }
    }

    translate_ann(
        analyzed.view(),
        Order::Topological,
        |opref, translator| match opref.get_instruction() {
            Pbs { .. } | Pbs2 { .. } | Pbs4 { .. } | Pbs8 { .. } => {
                let args = opref
                    .get_args_iter()
                    .next()
                    .unwrap()
                    .get_annotation()
                    .0
                    .iter()
                    .map(|va| translator.translate_val(va))
                    .cosvec();
                let live_rets = opref
                    .get_returns_iter()
                    .filter(|ret| used.contains(&ret.get_id()))
                    .covec();
                let rets = translator.add_op(
                    TaskInstructionSet::Task {
                        inp_size: args.len().sas(),
                        oup_size: live_rets.len().sas(),
                        work: 1,
                    },
                    args,
                );
                (live_rets.into_iter(), rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| translator.register_translation(old, new));
            }
            _ => {}
        },
    )
}
