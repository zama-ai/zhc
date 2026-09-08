use crate::SchedPolicy;

use super::*;

mod batcher;
mod scheduler;

use zhc_config::hpu::HpuConfig;
use zhc_ir::IR;
use zhc_langs::hpulang::HpuLang;

#[allow(unused)]
pub fn schedule(
    ir: &IR<HpuLang>,
    config: &HpuConfig,
    batch_policy: SchedPolicy,
    sched_policy: SchedPolicy,
) -> IR<HpuLang> {
    let batched = batcher::batch(ir, config, batch_policy);
    scheduler::schedule(&batched, config, sched_policy)
}

#[allow(unused)]
pub fn small_schedule(
    ir: &IR<HpuLang>,
    config: &HpuConfig,
    sched_policy: SchedPolicy,
) -> IR<HpuLang> {
    scheduler::schedule(&ir, config, sched_policy)
}

pub use batcher::batch;
pub use scheduler::schedule as schedule_batched;
