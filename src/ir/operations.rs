use super::args::*;

#[derive(Debug, Clone, PartialEq)]
pub enum IrOperation {
    // Load/Store and Sync
    Load {
        dst: Register,
        mem: MemCell,
    },
    Store {
        mem: MemCell,
        src: Register,
    },
    Sync {},

    // Arith operation
    Add {
        dst: Register,
        src_a: Register,
        src_b: Register,
    },
    Sub {
        dst: Register,
        src_a: Register,
        src_b: Register,
    },
    Mac {
        // src_a + (src_b*imm_b)
        dst: Register,
        src_a: Register,
        src_b: Register,
        imm_b: ImmCell,
    },

    // Arith with scalar
    Adds {
        dst: Register,
        src_a: Register,
        imm_b: ImmCell,
    },
    Subs {
        dst: Register,
        src_a: Register,
        imm_b: ImmCell,
    },
    Ssub {
        dst: Register,
        imm_a: ImmCell,
        src_b: Register,
    },
    Muls {
        dst: Register,
        src_a: Register,
        imm_b: ImmCell,
    },

    // Pbs operation
    // TODO: Define many lut implicitly or explicitly ?
    Pbs {
        dst: Vec<Register>,
        src: Register,
        lut: PbsLut,
        flush: bool,
    },
}

impl std::fmt::Display for IrOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrOperation::Load { dst, mem } => write!(f, "LD {dst} {mem}"),
            IrOperation::Store { mem, src } => write!(f, "ST {mem} {src}"),
            IrOperation::Sync {} => write!(f, "SYNC"),
            IrOperation::Add { dst, src_a, src_b } => write!(f, "ADD {dst} {src_a} {src_b}"),
            IrOperation::Sub { dst, src_a, src_b } => write!(f, "SUB {dst} {src_a} {src_b}"),
            IrOperation::Mac {
                dst,
                src_a,
                src_b,
                imm_b,
            } => write!(f, "MAC {dst} {src_a} {src_b} {imm_b}"),
            IrOperation::Adds { dst, src_a, imm_b } => write!(f, "ADDS {dst} {src_a} {imm_b}"),
            IrOperation::Subs { dst, src_a, imm_b } => write!(f, "SUBS {dst} {src_a} {imm_b}"),
            IrOperation::Ssub { dst, imm_a, src_b } => write!(f, "SSUB {dst} {imm_a} {src_b}"),
            IrOperation::Muls { dst, src_a, imm_b } => write!(f, "MULS {dst} {src_a} {imm_b}"),
            IrOperation::Pbs {
                dst,
                src,
                lut,
                flush,
            } => {
                let vec_dst = dst
                    .into_iter()
                    .map(|x| x.to_string())
                    .reduce(|acc, e| acc + e.as_str())
                    .expect("Error while expanding Pbs dst field");
                write!(
                    f,
                    "PBS{} [{vec_dst}] {src} {lut}",
                    if *flush { "_F" } else { "" }
                )
            }
        }
    }
}

/// Gather operation in categories
#[derive(Clone, Debug)]
pub enum OpKind {
    Mem,
    Arith,
    ArithMsg,
    Pbs,
    Ucore,
}
impl From<&IrOperation> for OpKind {
    fn from(op: &IrOperation) -> Self {
        match op {
            IrOperation::Load { .. } | IrOperation::Store { .. } | IrOperation::Sync { .. } => {
                Self::Mem
            }
            IrOperation::Add { .. } | IrOperation::Sub { .. } | IrOperation::Mac { .. } => {
                Self::Arith
            }
            IrOperation::Adds { .. }
            | IrOperation::Subs { .. }
            | IrOperation::Ssub { .. }
            | IrOperation::Muls { .. } => Self::ArithMsg,
            IrOperation::Pbs { .. } => Self::Pbs,
        }
    }
}

/// IrCell enable to gather Ir arguments that introduce dependency
/// Usefull for Dag construct
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum IrCell {
    Register(Register),
    Mem(MemCell),
}

impl std::fmt::Display for IrCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrCell::Register(register) => write!(f, "{register}",),
            IrCell::Mem(mem_cell) => write!(f, "{mem_cell}",),
        }
    }
}

impl IrOperation {
    pub fn get_inputs(&self) -> Vec<IrCell> {
        match self {
            IrOperation::Load { mem, .. } => vec![IrCell::Mem(mem.clone())],
            IrOperation::Store { src, .. } => vec![IrCell::Register(src.clone())],
            IrOperation::Sync {} => vec![],
            IrOperation::Add { src_a, src_b, .. } => vec![
                IrCell::Register(src_a.clone()),
                IrCell::Register(src_b.clone()),
            ],
            IrOperation::Sub { src_a, src_b, .. } => vec![
                IrCell::Register(src_a.clone()),
                IrCell::Register(src_b.clone()),
            ],
            IrOperation::Mac { src_a, src_b, .. } => vec![
                IrCell::Register(src_a.clone()),
                IrCell::Register(src_b.clone()),
            ],
            IrOperation::Adds { src_a, .. } => vec![IrCell::Register(src_a.clone())],
            IrOperation::Subs { src_a, .. } => vec![IrCell::Register(src_a.clone())],
            IrOperation::Ssub { src_b, .. } => vec![IrCell::Register(src_b.clone())],
            IrOperation::Muls { src_a, .. } => vec![IrCell::Register(src_a.clone())],
            IrOperation::Pbs { src, .. } => vec![IrCell::Register(src.clone())],
        }
    }
    pub fn get_outputs(&self) -> Vec<IrCell> {
        match self {
            IrOperation::Load { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Store { mem, .. } => vec![IrCell::Mem(mem.clone())],
            IrOperation::Sync {} => vec![],
            IrOperation::Add { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Sub { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Mac { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Adds { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Subs { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Ssub { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Muls { dst, .. } => vec![IrCell::Register(dst.clone())],
            IrOperation::Pbs { dst, .. } => dst
                .iter()
                .map(|x| IrCell::Register(x.clone()))
                .collect::<Vec<_>>(),
        }
    }
}
