mod args;
pub use args::{ImmCell, MemCell, Register};

mod builder;
pub use builder::{IrBuilder, IrBuilderWrapped};

mod dag;
pub use dag::IrDag;
mod frontend;
pub use frontend::create_rhai_engine;
mod operations;
pub use operations::{IrOperation, OpKind};

mod egui;
pub use egui::dag_display;
