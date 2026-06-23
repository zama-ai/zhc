use crate::{OpIdRaw, visualization::{StyleModifier, VisualAnnotation}};
use zhc_utils::{Dumpable, graphics::ColorScale};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartitionId(pub OpIdRaw);

impl Dumpable for PartitionId {
    fn dump_to_string(&self) -> String {
        format!("Partition {:?}", self.0)
    }
}

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
