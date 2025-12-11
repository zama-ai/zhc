use hpuc_ir::IR;
use hpuc_langs::doplang::Doplang;
use hpuc_sim::{hpu::{DOp, DOpId, Events, Hpu, HpuConfig}, Cycle, Simulator};

pub fn compute_latency(ir: &IR<Doplang>, config: HpuConfig) -> Cycle {
    let mut simulator = Simulator::from_simulatable(config.freq, Hpu::new(&config));
    let dops = ir.walk_ops_linear().map(|a| {
        DOp{
            raw: a.get_operation(),
            id: DOpId(a.get_id().into()),
        }
    }).collect();
    let event = Events::IscPushDOps(dops);
    simulator.dispatch(event);
    simulator.play();
    simulator.now()
}

#[cfg(test)]
mod test {
    use hpuc_ir::{IR, translation::Translator};
    use hpuc_langs::ioplang::Ioplang;
    use hpuc_sim::{hpu::{HpuConfig, PhysicalConfig}, Cycle, MHz};
    use crate::{allocator::allocate_registers, scheduler::schedule, test::{get_add_ir, get_cmp_ir, get_sub_ir}, translation::IoplangToHpulang};
    use super::compute_latency;

    fn pipeline(ir: &IR<Ioplang>) -> Cycle {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());
        let scheduled = schedule(&ir, &config);
        let allocated = allocate_registers(&scheduled, &config);
        compute_latency(&allocated, config)
    }

    #[test]
    fn test_latency_add_ir() {
        let lat = pipeline(&get_add_ir(16, 2, 2));
        assert_eq!(lat, Cycle(176146));
        println!("{}us", lat.as_ts(MHz(400).period()));
    }

    #[test]
    fn test_latency_sub_ir() {
        let lat = pipeline(&get_sub_ir(16, 2, 2));
        assert_eq!(lat, Cycle(295259));
        println!("{}us", lat.as_ts(MHz(400).period()));
    }

    #[test]
    fn test_latency_cmp_ir() {
        let lat = pipeline(&get_cmp_ir(16, 2, 2));
        assert_eq!(lat, Cycle(301796));
        println!("{}us", lat.as_ts(MHz(400).period()));
    }
}
