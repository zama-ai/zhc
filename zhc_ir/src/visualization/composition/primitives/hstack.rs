use super::*;
use zhc_utils::{
    graphics::{Color, Frame, Justify, Size, Width},
    iter::Separate,
};

enum Spaced<E> {
    Element(E),
    Space,
}

/// Horizontal stack container that arranges child elements left to right with spacing.
pub struct HStack<E, C: Class = NoClass> {
    pub content: Vec<E>,
    styler: Styler<C>,
    variable: VariableCell,
}

impl<E, C: Class> HStack<E, C> {
    /// Creates a new horizontal stack with the given child elements.
    pub fn new(modifier: Option<StyleModifier>, content: Vec<E>) -> Self {
        Self {
            content,
            styler: Styler::new(modifier),
            variable: VariableCell::fresh(),
        }
    }
}

impl<E, C: Class> SceneElement for HStack<E, C> {
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

impl<E: SceneSolver, C: Class> SceneSolver for HStack<E, C> {
    fn solve_size(&mut self) {
        let style = self.styler.get();
        let size = self
            .content
            .iter_mut()
            .map(Spaced::Element)
            .separate_with(|| Spaced::Space)
            .fold(Size::ZERO, |size, element| match element {
                Spaced::Element(element) => {
                    element.solve_size();
                    size.stack_horizontal(element.get_size())
                }
                Spaced::Space => size.pad_right(style.spacing),
            })
            .pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, available: Frame) {
        let style = self.styler.get();
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
            element.solve_frame(child_frame);
        }
    }
}

impl<E: Renderable, C: Class> Renderable for HStack<E, C> {
    fn render(&self) -> Vec<SvgElement> {
        let style = self.styler.get();
        let frame = self.get_frame();
        let mut elements = Vec::new();

        // Background rect if visible
        if style.fill_color != Color::TRANSPARENT || style.border_color != Color::TRANSPARENT {
            elements.push(SvgElement::Rect {
                x: frame.position.x.0,
                y: frame.position.y.0,
                width: frame.size.width.0.0,
                height: frame.size.height.0.0,
                rx: (style.corner_radius.0 > 0.0).then_some(style.corner_radius.0),
                fill: Some(style.fill_color.to_string()),
                stroke: Some(style.border_color.to_string()),
                stroke_width: Some(style.border_width.0),
                class: None,
                id: None,
                data_val: None,
            });
        }

        // Render children
        for child in &self.content {
            elements.extend(child.render());
        }

        elements
    }
}
