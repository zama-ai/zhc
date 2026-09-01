use zhc_builder::Builder;
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig, vm::VmConfig};
use zhc_crypto::integer_semantics::lut::LutId;
use zhc_utils::topology::Topology;

#[derive(Debug)]
pub struct PipelineContext {
    pub builder: Option<Builder>,
    pub hpu_config: Option<HpuConfig>,
    pub multi_hpu_config: Option<MultiHpuConfig>,
    pub vm_config: Option<VmConfig>,
    pub topology: Topology,
    pub legacy_hpu_scheduler: bool,
    pub hpu_trace_events: bool,
    pub hpu_lut_relocation: Option<Vec<LutId>>,
}

impl PipelineContext {
    pub fn new() -> Self {
        PipelineContext {
            builder: None,
            hpu_config: None,
            multi_hpu_config: None,
            vm_config: None,
            topology: Topology::detect_topology(),
            legacy_hpu_scheduler: false,
            hpu_trace_events: false,
            hpu_lut_relocation: None,
        }
    }
}
