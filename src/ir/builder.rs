use std::io::Write;
use std::ops::Deref;
use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use super::args::*;
use super::operations::IrOperation;

#[derive(Debug, Clone)]
pub struct IrBuilder {
    operations: Vec<IrOperation>,
    register_counter: usize,
}

impl IrBuilder {
    pub fn new() -> Self {
        IrBuilder {
            operations: Vec::new(),
            register_counter: 0,
        }
    }
    pub fn ssa_register(&mut self) -> Register {
        let reg = Register::Virt(self.register_counter);
        self.register_counter += 1;
        reg
    }

    pub fn push(&mut self, op: IrOperation) {
        self.operations.push(op)
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        for op in self.operations.iter() {
            writeln!(file, "{}", op.to_string())?;
        }
        Ok(())
    }
    pub fn operations(&self) -> &[IrOperation] {
        self.operations.as_slice()
    }
}

#[derive(Clone)]
pub struct IrBuilderWrapped {
    inner: Arc<Mutex<IrBuilder>>,
}

impl Deref for IrBuilderWrapped {
    type Target = Mutex<IrBuilder>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl IrBuilderWrapped {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(IrBuilder::new())),
        }
    }
}
