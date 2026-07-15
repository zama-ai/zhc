use zhc_ir::{IR, OpIdRaw};
use zhc_langs::{vmlang::{VmByteCode, VmLang}};
use zhc_sim::{MHz, Simulator};

mod analyze;
mod sim;

pub use analyze::*;
pub use sim::*;
use zhc_utils::{small::SmallVec};

use crate::scheduler::SchedPolicy;

pub struct VmExecutionPlan {
    pub irs: Vec<Vec<VmByteCode>>,
    pub locks_table: Vec<u8>,
    pub successors_table: Vec<SmallVec<OpIdRaw>>,
    pub nregs: usize
}

#[allow(unused)]
pub fn schedule<'a>(ir: &'a IR<VmLang>, n_threads: u8, policy: SchedPolicy) -> VmExecutionPlan {
    let ann_ir = analyze(ir);
    let mut sim = Simulator::from_simulatable(
        MHz(400),
        LightVm::new(&ann_ir, n_threads, policy),
        zhc_sim::TracingLevel::None,
    );
    sim.play();
    let light_vm = sim.into_simulatable();
    VmExecutionPlan {
        irs: light_vm.schedules.into_iter().map(|deq| deq.into()).collect(),
        locks_table: light_vm.locks_table,
        successors_table: light_vm.successors_table,
        nregs: light_vm.last_introduced_reg.0 as usize + 1
    }
}

#[cfg(test)]
mod test {}
