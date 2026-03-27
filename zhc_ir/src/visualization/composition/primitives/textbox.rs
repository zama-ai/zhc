use std::marker::PhantomData;

use zhc_utils::graphics::{Frame, Size};

use super::*;

/// Text element that renders string content with typography styling.
pub struct TextBox<C: Class = NoClass> {
    pub content: String,
    class: PhantomData<C>,
    variable: VariableCell,
}

impl<C: Class> TextBox<C> {
    /// Creates a new text box with the given content string.
    pub fn new(content: String) -> Self {
        Self {
            content,
            class: PhantomData,
            variable: VariableCell::fresh(),
        }
    }
}

impl<C: Class> Element for TextBox<C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = style
            .font_size
            .get_text_size(&self.content)
            .pad(style.padding);
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
