use std::fmt::Debug;

use crate::visualization::composition::{SceneSolver, StyleModifier};

pub trait VisualAnnotation: Debug + 'static {
    fn style_modifier(&self) -> Option<StyleModifier>;
    fn widget(&self) -> Option<Box<dyn SceneSolver>>;
}
