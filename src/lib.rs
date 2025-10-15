mod ir;
pub use ir::{BuilderContext, ImmCell, IrBuilder, IrBuilderWrapped, IrDag, MemCell, Register};
mod frontend;
pub use frontend::{create_rhai_engine, dag_display};
