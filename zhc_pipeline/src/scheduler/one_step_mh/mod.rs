use zhc_ir::{IR, OpId, OpMap};
use zhc_langs::hpulang::{HpuId, HpuInstructionSet, HpuLang, HpuLocality, TransferId};
use zhc_sim::{MHz, Simulator, hpu::MultiHpuConfig};

mod affinity;
mod analyze;
mod batcher;
mod sim;

pub use affinity::*;
pub use analyze::*;
pub use batcher::*;
pub use sim::*;
use zhc_utils::{SafeAs, small::SmallMap};

use crate::scheduler::SchedPolicy;

#[allow(unused)]
pub fn schedule<'a>(
    ir: &'a IR<HpuLang>,
    localities: OpMap<HpuLocality>,
    config: &MultiHpuConfig,
    policy: SchedPolicy,
) -> Vec<IR<HpuLang>> {
    let ann_ir = analyze(ir, localities);
    let mut sim = Simulator::from_simulatable(
        MHz(400),
        LightMultiHpu::new(&ann_ir, config, policy),
        zhc_sim::TracingLevel::Events,
    );
    sim.play();
    let mut transfers_counter: SmallMap<HpuId, u8> =
        (0..config.n_hpus).map(|i| (HpuId(i), 1)).collect();
    let transfer_map: SmallMap<OpId, TransferId> = ir
        .walk_ops_linear()
        .filter(|a| a.get_instruction().is_transfer())
        .map(|op| {
            let HpuInstructionSet::Transfer{ to, .. } = op.get_instruction() else {unreachable!()};
            let i = *transfers_counter.get(&to).unwrap();
            *transfers_counter.get_mut(&to).unwrap() = i.strict_add(1);
            (op.get_id(), TransferId(i.sas()))
        })
        .collect();
    sim.into_simulatable()
        .hpus
        .into_iter()
        .map(|hpu| batch(ir, hpu.id, hpu.schedule.into(), &transfer_map))
        .collect()
}
