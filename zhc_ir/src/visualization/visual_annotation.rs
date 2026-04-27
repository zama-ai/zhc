use std::fmt::Debug;

use crate::visualization::composition::{DynamicElement, StyleModifier};

pub trait VisualAnnotation: Debug + 'static {
    fn style_modifier(&self) -> Option<StyleModifier> {
        None
    }

    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}
