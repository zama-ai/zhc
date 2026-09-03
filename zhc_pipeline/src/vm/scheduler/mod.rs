use zhc_config::vm::VmConfig;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{IR, OpIdRaw};
use zhc_langs::vmlang::{VmByteCode, VmLang};
use zhc_sim::Simulator;

mod analyze;
mod sim;

pub use analyze::*;
pub use sim::*;
use zhc_utils::{SafeAs, small::SmallVec, topology::Topology, units::MHz};

use crate::SchedPolicy;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VmExecutionPlan {
    pub irs: Vec<Vec<VmByteCode>>,
    pub lut_reg: LutRegistry,
    pub locks_table: Vec<u8>,
    pub successors_table: Vec<SmallVec<OpIdRaw>>,
    pub nregs: usize,
}

#[allow(unused)]
pub fn schedule<'a>(
    ir: &'a IR<VmLang>,
    lut_reg: &LutRegistry,
    config: &VmConfig,
    topology: &Topology,
    policy: SchedPolicy,
) -> VmExecutionPlan {
    let ann_ir = analyze(ir);
    let mut sim = Simulator::from_simulatable(
        MHz(400),
        LightVm::new(&ann_ir, &lut_reg, topology.n_processors().sas(), policy),
        zhc_sim::TracingLevel::None,
    );
    sim.play();
    let light_vm = sim.into_simulatable();
    VmExecutionPlan {
        irs: light_vm
            .schedules
            .into_iter()
            .map(|deq| deq.into())
            .collect(),
        lut_reg: lut_reg.clone(),
        locks_table: light_vm.locks_table,
        successors_table: light_vm.successors_table,
        nregs: light_vm.last_introduced_reg.0 as usize + 1,
    }
}

#[cfg(test)]
mod test {}
