use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{IR, OpId, OpMap};
use zhc_langs::hpulang::{HpuLang, HpuLocality, TransferId};
use zhc_sim::Simulator;

mod affinity;
mod analyze;
mod batcher;
mod sim;

pub use affinity::*;
pub use analyze::*;
pub use batcher::*;
pub use sim::*;
use zhc_utils::{small::SmallMap, units::MHz};

use crate::SchedPolicy;

#[allow(unused)]
pub fn schedule<'a>(
    ir: &'a IR<HpuLang>,
    localities: &OpMap<HpuLocality>,
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
    // Flag allocation. On real ucore firmware a transfer's flag indexes the
    // *sender's* local `mhdma_table[iid][flag]`: `NOTIFY` writes the outgoing
    // descriptor (src_ct_id, target hid) there, and it is read back lazily when
    // the ISC acks the SYNC (`generate_user_notify`). It also indexes the
    // *receiver's* table for the matching WAIT/LD_B2B. A flag must therefore be
    // unique across every transfer incident to any single board — both its
    // outbound and inbound transfers. A per-destination counter only guarantees
    // uniqueness among a receiver's inbound transfers, so a board sending to
    // several destinations reuses low flags and its NOTIFYs clobber each other's
    // descriptor: one destination never gets notified and deadlocks in WAIT.
    // A single global counter makes every transfer's flag unique, which trivially
    // satisfies the per-board constraint (flag 0 is reserved for source pre-load).
    let mut transfers_counter: u8 = 1;
    let transfer_map: SmallMap<OpId, TransferId> = ir
        .walk_ops_linear()
        .filter(|a| a.get_instruction().is_transfer())
        .map(|op| {
            let i = transfers_counter;
            transfers_counter = transfers_counter.strict_add(1);
            (op.get_id(), TransferId(i))
        })
        .collect();
    sim.into_simulatable()
        .hpus
        .into_iter()
        .map(|hpu| batch(ir, hpu.id, hpu.schedule.into(), &transfer_map))
        .collect()
}
