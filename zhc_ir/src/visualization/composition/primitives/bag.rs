use super::*;
use zhc_utils::graphics::{Frame, Size};

/// Container for multiple SceneElements without layout relationships.
/// Elements manage their own positions (e.g., via VariableWatch).
pub struct Bag<E> {
    pub content: Vec<E>,
    variable: VariableCell,
}

impl<E> Bag<E> {
    pub fn new(content: Vec<E>) -> Self {
        Self {
            content,
            variable: VariableCell::fresh(),
        }
    }
}

impl<E> SceneElement for Bag<E> {
    fn get_size(&self) -> Size {
        Size::ZERO
    }

    fn get_frame(&self) -> Frame {
        self.variable.get_frame()
    }

    fn get_variable_cell(&self) -> VariableCell {
        self.variable.clone()
    }
}

impl<E: Renderable> Renderable for Bag<E> {
    fn render(&self) -> Vec<SvgElement> {
        self.content.iter().flat_map(|e| e.render()).collect()
    }
}
