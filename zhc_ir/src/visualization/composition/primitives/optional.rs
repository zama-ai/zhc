#![allow(unused)]
use super::*;
use zhc_utils::graphics::{Frame, Size};

pub enum Optional<E> {
    Some(E),
    None(VariableCell),
}

impl<E> Optional<E> {
    pub fn new(content: Option<E>) -> Self {
        match content {
            Some(e) => Optional::Some(e),
            None => Optional::None(VariableCell::fresh()),
        }
    }
}

impl<E: SceneElement> Optional<E> {
    pub fn maybe_variable_cell(&self) -> Option<VariableCell> {
        match self {
            Optional::Some(e) => Some(e.get_variable_cell()),
            Optional::None(_) => None,
        }
    }
}

impl<E: SceneElement> SceneElement for Optional<E> {
    fn get_size(&self) -> Size {
        match self {
            Optional::Some(e) => e.get_size(),
            Optional::None(variable_cell) => variable_cell.get_size(),
        }
    }

    fn get_frame(&self) -> Frame {
        match self {
            Optional::Some(e) => e.get_frame(),
            Optional::None(variable_cell) => variable_cell.get_frame(),
        }
    }

    fn get_variable_cell(&self) -> VariableCell {
        match self {
            Optional::Some(e) => e.get_variable_cell(),
            Optional::None(_) => panic!(),
        }
    }
}

impl<E: SceneSolver> SceneSolver for Optional<E> {
    fn solve_size(&mut self) {
        match self {
            Optional::Some(e) => e.solve_size(),
            Optional::None(v) => v.set_size(Size::ZERO),
        }
    }

    fn solve_frame(&mut self, available: Frame) {
        match self {
            Optional::Some(e) => e.solve_frame(available),
            Optional::None(variable_cell) => variable_cell.set_frame(available),
        }
    }
}

impl<E> From<Option<E>> for Optional<E> {
    fn from(value: Option<E>) -> Self {
        match value {
            Some(e) => Optional::Some(e),
            None => Optional::None(VariableCell::fresh()),
        }
    }
}

impl<E: Renderable> Renderable for Optional<E> {
    fn render(&self) -> Vec<SvgElement> {
        match self {
            Optional::Some(e) => e.render(),
            Optional::None(_) => vec![],
        }
    }
}
