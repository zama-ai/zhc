use std::fmt::Debug;

use crate::{
    OpId, ValId, evaluation::{Evaluation, ValState}, visualization::{
        NoClass, TextBox,
        composition::{DynamicElement, StyleModifier},
    }
};

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

impl VisualAnnotation for OpId {}
impl VisualAnnotation for ValId {}

impl VisualAnnotation for () {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        None
    }
}

impl<V: Evaluation + VisualAnnotation> VisualAnnotation for ValState<V> {
    fn style_modifier(&self) -> Option<StyleModifier> {
        match self {
            ValState::Evaluated(v) => v.style_modifier(),
            _ => None,
        }
    }

    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        match self {
            ValState::Evaluated(v) => v.widget(),
            _ => None,
        }
    }
}

impl<V: VisualAnnotation> VisualAnnotation for Option<V> {
    fn widget(&self) -> Option<Box<dyn DynamicElement>> {
        match self {
            Some(v) => v.widget(),
            None => None,
        }
    }

}
