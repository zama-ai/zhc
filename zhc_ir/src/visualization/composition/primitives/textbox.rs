use zhc_utils::graphics::{Color, Frame, Size};

use super::*;
use crate::visualization::svg::{DominantBaseline, TextAnchor};

/// Text element that renders string content with typography styling.
pub struct TextBox<C: Class = NoClass> {
    pub content: String,
    styler: Styler<C>,
    variable: VariableCell,
}

impl<C: Class> TextBox<C> {
    /// Creates a new text box with the given content string.
    pub fn new(modifier: Option<StyleModifier>, content: String) -> Self {
        Self {
            content,
            styler: Styler::new(modifier),
            variable: VariableCell::fresh(),
        }
    }
}

impl<C: Class> SceneElement for TextBox<C> {
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

impl<C: Class> SceneSolver for TextBox<C> {
    fn solve_size(&mut self) {
        let style = self.styler.get();
        let size = style
            .font_size
            .get_text_size(&self.content)
            .pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, available: Frame) {
        let style = self.styler.get();
        let frame = available.resize(&self.get_size(), style.halign, style.valign);
        self.variable.set_frame(frame);
    }
}

impl<C: Class> Renderable for TextBox<C> {
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

        // Text content (one element per line)
        for (line_index, line) in self.content.lines().enumerate() {
            elements.push(SvgElement::Text {
                x: frame.position.x.0 + style.padding.0,
                y: frame.position.y.0
                    + style.padding.0
                    + (line_index as f64 * style.font_size.0 * 1.2),
                content: line.to_string(),
                font_size: style.font_size.0,
                font_family: Some(style.font.0.to_string()),
                fill: Some(style.font_color.to_string()),
                text_anchor: TextAnchor::from(style.font_halign),
                dominant_baseline: DominantBaseline::Hanging,
                class: None,
                id: None,
            });
        }

        elements
    }
}
