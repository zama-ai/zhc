use std::fmt::Debug;

use crate::visualization::{NoClass, TextBox, composition::{DynamicElement, StyleModifier}};

pub trait VisualAnnotation: Debug + 'static {
    fn style_modifier(&self) -> Option<StyleModifier> {
        None
    }

    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        Some(Box::new(TextBox::<NoClass>::new(
            None,
            format!("{:?}", self),
        )))
    }
}

impl VisualAnnotation for () {}
