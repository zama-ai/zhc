use super::*;
use zhc_utils::graphics::{Frame, Height, Size, Width};

/// A spacer that takes up a fixed size plus padding, rendering nothing.
pub struct Spacer<C: Class = NoClass> {
    size: Size,
    styler: Styler<C>,
    variable: VariableCell,
}

impl<C: Class> Spacer<C> {
    /// Creates a spacer with explicit width and height.
    pub fn new(modifier: Option<StyleModifier>, width: Width, height: Height) -> Self {
        Self {
            size: Size { width, height },
            styler: Styler::new(modifier),
            variable: VariableCell::fresh(),
        }
    }

    /// Creates a horizontal spacer (zero height).
    #[allow(unused)]
    pub fn horizontal(modifier: Option<StyleModifier>, width: Width) -> Self {
        Self::new(modifier, width, Height::ZERO)
    }

    /// Creates a vertical spacer (zero width).
    pub fn vertical(modifier: Option<StyleModifier>, height: Height) -> Self {
        Self::new(modifier, Width::ZERO, height)
    }
}

impl<C: Class> SceneElement for Spacer<C> {
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

impl<C: Class> SceneSolver for Spacer<C> {
    fn solve_size(&mut self) {
        let style = self.styler.get();
        let size = self.size.pad(style.padding);
        self.variable.set_size(size);
    }

    fn solve_frame(&mut self, available: Frame) {
        let style = self.styler.get();
        let frame = available.resize(&self.get_size(), style.halign, style.valign);
        self.variable.set_frame(frame);
    }
}

impl<C: Class> Renderable for Spacer<C> {
    fn render(&self) -> Vec<SvgElement> {
        // Spacers are invisible
        vec![]
    }
}
