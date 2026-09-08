use zhc_ir::{
    AnnOpRef, IR, ValId,
    translation::{Order, Translation, translate_ann},
    visualization::VisualAnnotation,
};
use zhc_utils::{
    SafeAs,
    iter::{CollectInSmallVec, MultiZip},
    small::SmallSet,
    svec,
};

use crate::{
    coarselang::{CoarseInstructionSet, CoarseLang},
    ioplang::{IopInstructionSet, IopLang},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ann(SmallSet<ValId>);
impl VisualAnnotation for Ann {}

pub fn coarsen_ioplang(ir: &IR<IopLang>) -> Translation<CoarseLang> {
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
                let rets = translator.add_op(
                    CoarseInstructionSet::Op {
                        inp_size: args.len().sas(),
                        oup_size: opref.get_return_arity().sas(),
                    },
                    args,
                );
                (opref.get_returns_iter(), rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| translator.register_translation(old, new));
            }
            _ => {}
        },
    )
}
