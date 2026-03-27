use super::*;
use std::marker::PhantomData;
use zhc_utils::{
    graphics::{Frame, Justify, Size, Width},
    iter::Separate,
};

enum Spaced<E> {
    Element(E),
    Space,
}

/// Horizontal stack container that arranges child elements left to right with spacing.
pub struct HStack<E: Element, C: Class = NoClass> {
    pub content: Vec<E>,
    class: PhantomData<C>,
    variable: VariableCell,
}

impl<E: Element, C: Class> HStack<E, C> {
    /// Creates a new horizontal stack with the given child elements.
    pub fn new(content: Vec<E>) -> Self {
        Self {
            content,
            class: PhantomData,
            variable: VariableCell::fresh(),
        }
    }
}

impl<E: Element, C: Class> Element for HStack<E, C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = self
            .content
            .iter_mut()
            .map(Spaced::Element)
            .separate_with(|| Spaced::Space)
            .fold(Size::ZERO, |size, element| match element {
                Spaced::Element(element) => {
                    element.solve_size(stylesheet);
                    size.stack_horizontal(element.get_size())
                }
                Spaced::Space => size.pad_right(style.spacing),
            })
            .pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let intrinsic = self.get_size();

        // Determine HStack's frame based on justify mode.
        let frame = match style.hjustify {
            Justify::Pack => available.resize(&intrinsic, style.halign, style.valign),
            Justify::Space | Justify::Spread => {
                available.resize_vertical(intrinsic.height, style.valign)
            }
        };
        self.variable.set_frame(frame.clone());

        // Compute inner frame (after padding) and child widths.
        let inner = frame.crop_around(style.padding);
        let widths: Vec<Width> = self.content.iter().map(|e| e.get_size().width).collect();

        // Get child frames from pack, justify, or spread.
        let child_frames = match style.hjustify {
            Justify::Pack => inner.pack_horizontal(&widths, style.halign, style.spacing),
            Justify::Space => inner.justify_horizontal(&widths),
            Justify::Spread => inner.spread_horizontal(&widths),
        };

        // Position each child.
        for (element, child_frame) in self.content.iter_mut().zip(child_frames) {
            element.solve_frame(stylesheet, child_frame);
        }
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
