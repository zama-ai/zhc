use super::*;
use zhc_utils::graphics::{Frame, Size};

/// Empty element that takes up space according to its padding but renders nothing.
pub struct Empty<C: Class = NoClass> {
    styler: Styler<C>,
    variable: VariableCell,
}

impl<C: Class> Empty<C> {
    /// Creates a new empty element.
    pub fn new(modifier: Option<StyleModifier>) -> Self {
        Self {
            styler: Styler::new(modifier),
            variable: VariableCell::fresh(),
        }
    }
}

impl<C: Class> SceneElement for Empty<C> {
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

impl<C: Class> SceneSolver for Empty<C> {
    fn solve_size(&mut self) {
        let style = C::STYLE;
        let size = Size::ZERO.pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, available: Frame) {
        let style = self.styler.get();
        let frame = available.resize(&self.get_size(), style.halign, style.valign);
        self.variable.set_frame(frame);
    }
}

impl<C: Class> Renderable for Empty<C> {
    fn render(&self) -> Vec<SvgElement> {
        let style = self.styler.get();
        let frame = self.get_frame();

        background_rect(&style, &frame)
    }
}
