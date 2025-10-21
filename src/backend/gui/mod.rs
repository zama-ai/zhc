mod node;
mod viewer;
pub use viewer::dag_display;

mod gui_impl;

use eframe::egui::Color32;

pub trait GraphFmt: Clone {
    fn fmt_short(&self) -> String;
    fn fmt_long(&self) -> String;
}

pub enum Format {
    Rectangle,
    Ellipse,
    Circle,
    Diamond,
}
pub trait GraphShow: Clone {
    fn format(&self) -> Format;
    fn color(&self) -> Color32;
}
