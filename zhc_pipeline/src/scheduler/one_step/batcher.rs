use zhc_ir::{
    IR,
    translation::{Order, translate},
};
use zhc_langs::hpulang::HpuLang;
use zhc_utils::iter::{CollectInSmallVec, CollectInVec, MultiZip, ReconcilerOf2};

use crate::scheduler::{
    one_step::SchedElm,
    utils::{Batch, Batches},
};

pub fn batch(ir: &IR<HpuLang>, sched: Vec<SchedElm>) -> IR<HpuLang> {
    // We get the batches back
    let mut batches = Batches::new();
    sched
        .iter()
        .filter_map(|elm| match elm {
            SchedElm::Op(_) => None,
            SchedElm::Batch(opids) => Some(opids),
        })
        .for_each(|opids| {
            let mut batch = Batch::new(opids.len());
            for o in opids.iter() {
                batch.push(ir.get_op(o));
            }
            batches.push(batch);
        });

    // We get the schedule back
    let flat_sched = sched
        .into_iter()
        .flat_map(|elm| match elm {
            SchedElm::Op(opid) => std::iter::once(opid).reconcile_1_of_2(),
            SchedElm::Batch(opids) => opids.into_iter().reconcile_2_of_2(),
        })
        .covec();
    let batchmap = batches.into_batch_map();
    translate(ir, Order::Custom(flat_sched), move |opref, engine| {
        use zhc_langs::hpulang::HpuInstructionSet::*;
        match opref.get_instruction() {
            AddCt
            | SubCt
            | Mac { .. }
            | AddPt
            | SubPt
            | PtSub
            | MulPt
            | AddCst { .. }
            | SubCst { .. }
            | CstSub { .. }
            | MulCst { .. }
            | CstCt { .. }
            | ImmLd { .. }
            | DstSt { .. }
            | SrcLd { .. }
            | TransferIn { .. }
            | TransferOut { .. } => {
                let new_args = opref
                    .get_arg_valids()
                    .iter()
                    .map(|valid| engine.translate_val(*valid))
                    .cosvec();
                let new_rets = engine.add_op(opref.get_instruction(), new_args);
                (opref.get_return_valids().iter(), new_rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| engine.register_translation(*old, new));
            }
            Pbs { .. }
            | Pbs2 { .. }
            | Pbs4 { .. }
            | Pbs8 { .. }
            | PbsF { .. }
            | Pbs2F { .. }
            | Pbs4F { .. }
            | Pbs8F { .. } => {
                if engine.has_translation(opref.get_return_valids()[0]) {
                    return;
                }
                let batch = batchmap.get(&opref.get_id()).unwrap();
                let (batch_ir, inputs, outputs) = batch.gen_batch_ir();
                let block = Box::new(batch_ir);
                let new_args = inputs
                    .into_iter()
                    .map(|arg| engine.translate_val(arg.get_id()))
                    .collect();
                let new_rets = engine.add_op(Batch { block }, new_args);
                (outputs.into_iter(), new_rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| engine.register_translation(old.get_id(), new));
            }
            Batch { .. } | BatchArg { .. } | BatchRet { .. } => {
                panic!("Unexpected batch operations encountered.")
            }
        }
    })
}
