//! Latency computation for HPU operations.
//!
//! This module provides functionality to compute execution latency for device
//! operations by simulating their execution on the target HPU hardware. The
//! latency computation takes into account hardware characteristics, operation
//! dependencies, and execution pipeline behavior.

use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_sim::{
    Cycle, Simulator,
    hpu::{DOp, DOpId, Events, Hpu, HpuConfig},
};

/// Computes the execution latency for the given device operation IR.
///
/// Takes an intermediate representation `ir` containing device operations and
/// the hardware configuration `config` to simulate execution and determine
/// the total number of cycles required for completion.
pub fn compute_latency(ir: &IR<DopLang>, config: &HpuConfig) -> Cycle {
    let mut simulator = Simulator::from_simulatable(config.freq, Hpu::new(&config));
    let dops = ir
        .walk_ops_linear()
        .map(|a| DOp {
            raw: a.get_instruction(),
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
        allocator::allocate_registers, batch_scheduler::batch_schedule,
        translation::lower_iop_to_hpu,
    };
    use zhc_builder::{CiphertextSpec, add, cmp_gt, count_0, lead0, mul_lsb, overflow_mul_lsb};
    use zhc_ir::IR;
    use zhc_langs::ioplang::IopLang;
    use zhc_sim::{
        Cycle, MHz,
        hpu::{HpuConfig, PhysicalConfig},
    };
    use zhc_utils::assert_display_is;

    fn pipeline(ir: &IR<IopLang>) -> Cycle {
        let ir = lower_iop_to_hpu(&ir);
        let config = HpuConfig::from(PhysicalConfig::tuniform_64b_pfail128_psi64());
        let batched = batch_schedule(&ir, &config);
        let allocated = allocate_registers(&batched, &config);
        compute_latency(&allocated, &config)
    }

    #[test]
    fn test_latency_add_ir() {
        let lat = pipeline(&add(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            format!("{}us", lat.as_ts(MHz(400).period())),
            r#"
                3244.7000000000003us
            "#
        );
    }

    #[test]
    fn test_latency_cmp_ir() {
        let lat = pipeline(&cmp_gt(CiphertextSpec::new(128, 2, 2)).into_ir());
        assert_display_is!(
            format!("{}us", lat.as_ts(MHz(400).period())),
            r#"
                12886.9125us
            "#
        );
    }

    #[test]
    fn test_latency_count0() {
        let lat = pipeline(&count_0(CiphertextSpec::new(128, 2, 2)).into_ir());
        assert_display_is!(
            format!("{}us", lat.as_ts(MHz(400).period())),
            r#"
                11439.960000000001us
            "#
        );
    }

    #[test]
    fn test_latency_mul_lsb_ir() {
        let lat = pipeline(&mul_lsb(CiphertextSpec::new(64, 2, 2)).into_ir());
        assert_display_is!(
            format!("{}us", lat.as_ts(MHz(400).period())),
            r#"
                128728.8975us
            "#
        );
    }

    #[test]
    fn test_latency_overflow_mul_lsb_ir() {
        let lat = pipeline(&overflow_mul_lsb(CiphertextSpec::new(64, 2, 2)).into_ir());
        assert_display_is!(
            format!("{}us", lat.as_ts(MHz(400).period())),
            r#"
                180263.2825us
            "#
        );
    }

    #[test]
    fn test_latency_overflow_lead_0() {
        let lat = pipeline(&lead0(CiphertextSpec::new(64, 2, 2)).into_ir());
        assert_display_is!(
            format!("{}us", lat.as_ts(MHz(400).period())),
            r#"
                13743.335000000001us
            "#
        );
    }
}
