use std::fmt;

/// Register slot could be Virtual (within IR ssa) and Physical (after register allocation)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    Virt(usize), // SSA virtual registers
    Phys(usize), // Physical registers (i.e. after register allocation)
}

impl Register {
    pub fn new_virt(id: usize) -> Self {
        Self::Virt(id)
    }
    pub fn new_phys(id: usize) -> Self {
        Self::Phys(id)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Virt(id) => write!(f, "Rv{}", id),
            Self::Phys(id) => write!(f, "R{}", id),
        }
    }
}

/// Carry virtual operand kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserKind {
    Src,
    Dst,
}
impl fmt::Display for UserKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserKind::Src => write!(f, "Src"),
            UserKind::Dst => write!(f, "Dst"),
        }
    }
}

/// MemoryCelly
/// Used to depicts memory slot
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemCell {
    Raw(usize), //@x -> x is addr offset
    User {
        kind: UserKind,
        slot: usize,
        digit: usize,
    }, //TS[x].y ->  Source operand x at digit y
    VHeap(usize), // Ssa like heap access
    PHeap(usize), // Physical heap access (i.e. after heap allocation)
}

impl fmt::Display for MemCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Raw(ofst) => write!(f, "@{}", ofst),
            Self::User { kind, slot, digit } => write!(f, "T{kind}[{slot}].{digit}"),
            Self::VHeap(slot) => write!(f, "Hv{slot}"),
            Self::PHeap(slot) => write!(f, "H{slot}"),
        }
    }
}

/// ImmCell
/// Used to depicts Imm value
/// Immediat could be know at compile time (i.e. constant) or only at runtime
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImmCell {
    Cst(usize),
    Virt { slot: usize, digit: usize },
}

impl fmt::Display for ImmCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cst(val) => write!(f, "#{}", val),
            Self::Virt { slot, digit } => write!(f, "TI[{slot}].{digit}"),
        }
    }
}

/// PbsLut
/// Immediat could be know at compile time (i.e. constant) or only at runtime
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PbsLut {
    name: String,
}
impl PbsLut {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for PbsLut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lut[{}]", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Register(Register),
    Address(MemCell),
    Immediate(ImmCell),
    Pbs(PbsLut),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{reg}"),
            Self::Address(addr) => write!(f, "{addr}"),
            Self::Immediate(imm) => write!(f, "{imm}"),
            Self::Pbs(lut) => write!(f, "{lut}"),
        }
    }
}
