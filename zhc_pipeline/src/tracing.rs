use std::path::Path;

use zhc_ir::IR;
use zhc_langs::doplang::DopLang;
use zhc_sim::{
    Simulator,
    hpu::{DOp, DOpId, Events, Hpu, HpuConfig},
};

pub fn trace_execution(ir: &IR<DopLang>, config: &HpuConfig, path: impl AsRef<Path>) {
    let mut simulator =
        Simulator::from_simulatable(config.freq, Hpu::new(&config), zhc_sim::TracingLevel::Load);
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
    simulator.now();
    simulator.dump_trace(path.as_ref());
}
