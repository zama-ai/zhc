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

/// Implement basics operation as function for ease binding with frontend
impl IrBuilder {
    // Arith operations =======================================================
    pub fn add(&mut self, src_a: Register, src_b: Register) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Add {
            dst: dst.clone(),
            src_a,
            src_b,
        };
        self.push(op);
        dst
    }
    pub fn sub(&mut self, src_a: Register, src_b: Register) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Sub {
            dst: dst.clone(),
            src_a,
            src_b,
        };
        self.push(op);
        dst
    }
    pub fn mac(&mut self, src_a: Register, src_b: Register, imm_b: ImmCell) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Mac {
            dst: dst.clone(),
            src_a,
            src_b,
            imm_b,
        };
        self.push(op);
        dst
    }

    // ArithMsg operations ===================================================
    pub fn adds(&mut self, src_a: Register, imm_b: ImmCell) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Adds {
            dst: dst.clone(),
            src_a,
            imm_b,
        };
        self.push(op);
        dst
    }
    pub fn subs(&mut self, src_a: Register, imm_b: ImmCell) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Subs {
            dst: dst.clone(),
            src_a,
            imm_b,
        };
        self.push(op);
        dst
    }
    pub fn ssub(&mut self, imm_a: ImmCell, src_b: Register) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Ssub {
            dst: dst.clone(),
            imm_a,
            src_b,
        };
        self.push(op);
        dst
    }
    pub fn muls(&mut self, src_a: Register, imm_b: ImmCell) -> Register {
        let dst = self.ssa_register();
        let op = IrOperation::Muls {
            dst: dst.clone(),
            src_a,
            imm_b,
        };
        self.push(op);
        dst
    }

    pub fn pbs_ml(&mut self, src: Vec<Register>, lut: PbsLut) -> Vec<Register> {
        let dst = src.iter().map(|_| self.ssa_register()).collect::<Vec<_>>();
        let op = IrOperation::Pbs {
            dst: dst.clone(),
            src,
            lut,
            flush: false, // Flushing handled by compiler only
        };
        self.push(op);
        dst
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
