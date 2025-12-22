//! Latency computation for HPU operations.
//!
//! This module provides functionality to compute execution latency for device
//! operations by simulating their execution on the target HPU hardware. The
//! latency computation takes into account hardware characteristics, operation
//! dependencies, and execution pipeline behavior.

use hpuc_ir::IR;
use hpuc_langs::doplang::Doplang;
use hpuc_sim::{
    Cycle, Simulator,
    hpu::{DOp, DOpId, Events, Hpu, HpuConfig},
};

/// Computes the execution latency for the given device operation IR.
///
/// Takes an intermediate representation `ir` containing device operations and
/// the hardware configuration `config` to simulate execution and determine
/// the total number of cycles required for completion.
pub fn compute_latency(ir: &IR<Doplang>, config: HpuConfig) -> Cycle {
    let mut simulator = Simulator::from_simulatable(config.freq, Hpu::new(&config));
    let dops = ir
        .walk_ops_linear()
        .map(|a| DOp {
            raw: a.get_operation(),
            id: DOpId(a.get_id().into()),
        })
        .collect();
    let event = Events::IscPushDOps(dops);
    simulator.dispatch(event);
    simulator.play_until_event(Events::IscProcessOver);
    simulator.now()
}

#[cfg(test)]
mod test {
    use super::compute_latency;
    use crate::{
        allocator::allocate_registers,
        batcher::batch,
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir, get_sub_ir},
        translation::IoplangToHpulang,
    };
    use hpuc_ir::{IR, translation::Translator};
    use hpuc_langs::ioplang::Ioplang;
    use hpuc_sim::{
        Cycle, MHz,
        hpu::{HpuConfig, PhysicalConfig},
    };

    fn pipeline(ir: &IR<Ioplang>) -> Cycle {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(&ir, &config);
        let batched = batch(&scheduled);
        let allocated = allocate_registers(&batched, &config);
        compute_latency(&allocated, config)
    }

    #[test]
    fn test_latency_add_ir() {
        let lat = pipeline(&get_add_ir(16, 2, 2));
        assert_eq!(lat, Cycle(1784087));
        println!("{}us", lat.as_ts(MHz(300).period()));
    }

    #[test]
    fn test_latency_sub_ir() {
        let lat = pipeline(&get_sub_ir(16, 2, 2));
        assert_eq!(lat, Cycle(1808161));
        println!("{}us", lat.as_ts(MHz(300).period()));
    }

    #[test]
    fn test_latency_cmp_ir() {
        let lat = pipeline(&get_cmp_ir(128, 2, 2));
        assert_eq!(lat, Cycle(5759543));
        println!("{}us", lat.as_ts(MHz(300).period()));
    }
}
