use super::*;
use zhc_ir::visualization::{StyleModifier, VisualAnnotation};
use zhc_utils::{Store, graphics::ColorScale, units::Cycle};

#[derive(Serialize, Debug, Clone, Copy, StoreIndex, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionId(u16);

impl VisualAnnotation for PartitionId {
    fn style_modifier(&self) -> Option<StyleModifier> {
        Some(StyleModifier {
            fill_color: Some(
                ColorScale::RAINBOW.interpolate((self.0 as f64 * 0.6180339887498949) % 1.0),
            ),
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug)]
pub struct PartitionSpec {
    pub parallelism: u16,
    pub latency: Cycle,
}

#[derive(Clone, Copy, Debug)]
pub enum TiePolicy {
    Concentrate,
    Spread,
}

#[derive(Clone, Debug)]
pub struct PartitionerConfig {
    pub specs: Store<PartitionId, PartitionSpec>,
    pub sched_policy: SchedPolicy,
    pub tie_policy: TiePolicy,
}
