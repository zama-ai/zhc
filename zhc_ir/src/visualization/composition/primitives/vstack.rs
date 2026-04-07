use super::*;
use zhc_utils::{
    graphics::{Color, Frame, Height, Justify, Size},
    iter::Separate,
};

enum Spaced<E> {
    Element(E),
    Space,
}

/// Vertical stack container that arranges child elements top to bottom with spacing.
pub struct VStack<E, C: Class = NoClass> {
    pub content: Vec<E>,
    styler: Styler<C>,
    variable: VariableCell,
}

impl<E, C: Class> VStack<E, C> {
    /// Creates a new vertical stack with the given child elements.
    pub fn new(modifier: Option<StyleModifier>, content: Vec<E>) -> Self {
        Self {
            content,
            styler: Styler::new(modifier),
            variable: VariableCell::fresh(),
        }
    }
}

impl<E, C: Class> SceneElement for VStack<E, C> {
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

impl<E: SceneSolver, C: Class> SceneSolver for VStack<E, C> {
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
                    size.stack_vertical(element.get_size())
                }
                Spaced::Space => size.pad_bottom(style.spacing),
            })
            .pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, available: Frame) {
        let style = self.styler.get();
        let intrinsic = self.get_size();

        // Determine VStack's frame based on justify mode.
        let frame = match style.vjustify {
            Justify::Pack => available.resize(&intrinsic, style.halign, style.valign),
            Justify::Space | Justify::Spread => {
                available.resize_horizontal(intrinsic.width, style.halign)
            }
        };
        self.variable.set_frame(frame.clone());

        // Compute inner frame (after padding) and child heights.
        let inner = frame.crop_around(style.padding);
        let heights: Vec<Height> = self.content.iter().map(|e| e.get_size().height).collect();

        // Get child frames from pack, justify, or spread.
        let child_frames = match style.vjustify {
            Justify::Pack => inner.pack_vertical(&heights, style.valign, style.spacing),
            Justify::Space => inner.justify_vertical(&heights),
            Justify::Spread => inner.spread_vertical(&heights),
        };

        // Position each child.
        for (element, child_frame) in self.content.iter_mut().zip(child_frames) {
            element.solve_frame(child_frame);
        }
    }
}

impl<E: Renderable, C: Class> Renderable for VStack<E, C> {
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

        // Render separators between children if enabled
        if style.draw_separators && self.content.len() > 1 {
            for i in 0..self.content.len() - 1 {
                let child_frame = self.content[i].get_frame();
                let next_frame = self.content[i + 1].get_frame();
                let sep_y = (child_frame.bottom_left().y.0 + next_frame.top_left().y.0) / 2.0;

                elements.push(SvgElement::Rect {
                    x: frame.position.x.0,
                    y: sep_y - style.border_width.0 / 2.0,
                    width: frame.size.width.0.0,
                    height: style.border_width.0,
                    rx: None,
                    fill: Some(style.border_color.to_string()),
                    stroke: None,
                    stroke_width: None,
                    class: None,
                    id: None,
                    data_val: None,
                });
            }
        }

        // Render children
        for child in &self.content {
            elements.extend(child.render());
        }

        elements
    }
}
