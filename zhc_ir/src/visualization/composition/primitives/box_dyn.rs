use crate::visualization::{
    composition::{SceneElement, SceneSolver},
    svg::Renderable,
};

pub trait DynamicElement: SceneSolver + Renderable {}

impl<T: SceneSolver + Renderable> DynamicElement for T {}

impl SceneElement for Box<dyn DynamicElement> {
    fn get_size(&self) -> zhc_utils::graphics::Size {
        self.as_ref().get_size()
    }

    fn get_frame(&self) -> zhc_utils::graphics::Frame {
        self.as_ref().get_frame()
    }

    fn get_variable_cell(&self) -> crate::visualization::composition::VariableCell {
        self.as_ref().get_variable_cell()
    }
}

impl SceneSolver for Box<dyn DynamicElement> {
    fn solve_size(&mut self) {
        self.as_mut().solve_size();
    }

    fn solve_frame(&mut self, available: zhc_utils::graphics::Frame) {
        self.as_mut().solve_frame(available);
    }
}

impl Renderable for Box<dyn DynamicElement> {
    fn render(&self) -> Vec<crate::visualization::svg::SvgElement> {
        self.as_ref().render()
    }
}
