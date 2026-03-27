use super::*;
use std::marker::PhantomData;
use zhc_utils::graphics::{Frame, Height, Size, Width};

/// A spacer that takes up a fixed size plus padding, rendering nothing.
pub struct Spacer<C: Class = NoClass> {
    size: Size,
    class: PhantomData<C>,
    variable: VariableCell,
}

impl<C: Class> Spacer<C> {
    /// Creates a spacer with explicit width and height.
    pub fn new(width: Width, height: Height) -> Self {
        Self {
            size: Size { width, height },
            class: PhantomData,
            variable: VariableCell::fresh(),
        }
    }

    /// Creates a horizontal spacer (zero height).
    #[allow(unused)]
    pub fn horizontal(width: Width) -> Self {
        Self::new(width, Height::ZERO)
    }

    /// Creates a vertical spacer (zero width).
    pub fn vertical(height: Height) -> Self {
        Self::new(Width::ZERO, height)
    }
}

impl<C: Class> Element for Spacer<C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = self.size.pad(style.padding);
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
