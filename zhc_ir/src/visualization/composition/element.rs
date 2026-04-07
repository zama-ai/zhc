use zhc_utils::graphics::{Frame, Size};

use super::*;

/// Query interface for scene graph elements after layout.
pub trait SceneElement {
    /// Returns the computed size from the sizing phase.
    fn get_size(&self) -> Size;

    /// Returns the final positioned frame from the positioning phase.
    fn get_frame(&self) -> Frame;

    fn get_variable_cell(&self) -> VariableCell;
}

/// Layout solver interface for scene graph elements.
pub trait SceneSolver: SceneElement {
    /// Computes the element's intrinsic size based on content and style.
    fn solve_size(&mut self);

    /// Positions the element within the available frame using style alignment rules.
    fn solve_frame(&mut self, available: Frame);
}

impl<T: SceneElement> SceneElement for Option<T> {
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

impl<T: SceneSolver> SceneSolver for Option<T> {
    fn solve_size(&mut self) {
        match self {
            Some(e) => e.solve_size(),
            None => {}
        }
    }

    fn solve_frame(&mut self, available: Frame) {
        match self {
            Some(e) => e.solve_frame(available),
            None => {}
        }
    }
}
