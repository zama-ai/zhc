use zhc_utils::graphics::{Frame, Size};

use super::*;

/// Defines the two-phase layout protocol for all renderable diagram elements.
pub trait Element {
    /// Computes the element's intrinsic size based on content and stylesheet.
    fn solve_size(&mut self, stylesheet: &StyleSheet);

    /// Positions the element within the available frame using stylesheet alignment rules.
    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame);

    /// Returns the computed size from the sizing phase.
    fn get_size(&self) -> Size;

    /// Returns the final positioned frame from the positioning phase.
    fn get_frame(&self) -> Frame;

    fn get_variable_cell(&self) -> VariableCell;
}

impl<T: Element> Element for Option<T> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        match self {
            Some(e) => e.solve_size(stylesheet),
            None => {}
        }
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        match self {
            Some(e) => e.solve_frame(stylesheet, available),
            None => {}
        }
    }

    fn get_size(&self) -> Size {
        match self {
            Some(e) => e.get_size(),
            None => panic!(),
        }
    }

    fn get_frame(&self) -> Frame {
        match self {
            Some(e) => e.get_frame(),
            None => panic!(),
        }
    }

    fn get_variable_cell(&self) -> VariableCell {
        match self {
            Some(e) => e.get_variable_cell(),
            None => panic!(),
        }
    }
}
