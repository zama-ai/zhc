use super::*;
use std::marker::PhantomData;
use zhc_utils::graphics::{Frame, Size};

/// Empty element that takes up space according to its padding but renders nothing.
pub struct Empty<C: Class = NoClass> {
    class: PhantomData<C>,
    variable: VariableCell,
}

impl<C: Class> Empty<C> {
    /// Creates a new empty element.
    pub fn new() -> Self {
        Self {
            class: PhantomData,
            variable: VariableCell::fresh(),
        }
    }
}

impl<C: Class> Element for Empty<C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = Size::ZERO.pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let frame = available.resize(&self.get_size(), style.halign, style.valign);
        self.variable.set_frame(frame);
    }

    fn get_size(&self) -> Size {
        self.variable.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.variable.get_frame()
    }

    fn get_variable_cell(&self) -> VariableCell {
        self.variable.clone()
    }
}
