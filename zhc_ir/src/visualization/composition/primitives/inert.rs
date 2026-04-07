use super::*;
use zhc_utils::graphics::{Frame, Size};

/// Wrapper that delegates SceneElement but provides trivial (no-op) SceneSolver.
pub struct Inert<E: SceneElement>(pub E);

impl<E: SceneElement> Inert<E> {
    pub fn new(element: E) -> Self {
        Self(element)
    }
}

impl<E: SceneElement> SceneElement for Inert<E> {
    fn get_size(&self) -> Size {
        self.0.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.0.get_frame()
    }

    fn get_variable_cell(&self) -> VariableCell {
        self.0.get_variable_cell()
    }
}

impl<E: SceneElement> SceneSolver for Inert<E> {
    fn solve_size(&mut self) {}

    fn solve_frame(&mut self, _available: Frame) {}
}

impl<E: Renderable> Renderable for Inert<E> {
    fn render(&self) -> Vec<SvgElement> {
        self.0.render()
    }
}
