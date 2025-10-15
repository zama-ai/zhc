use std::{fmt, sync::Arc};

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
#[derive(Clone)]
pub struct PbsLut {
    xfer_fn: Arc<dyn Fn(u8) -> Vec<u8>>,
    deg_fn: Arc<dyn Fn(u8) -> u8>,
    xfer_str: String,
    deg_str: String,
}

impl PbsLut {
    /// Enable crate module to implement custom PbsLut new fn
    pub(crate) fn new_raw(
        xfer_fn: Arc<dyn Fn(u8) -> Vec<u8>>,
        deg_fn: Arc<dyn Fn(u8) -> u8>,
        xfer_str: String,
        deg_str: String,
    ) -> Self {
        Self {
            xfer_fn,
            deg_fn,
            xfer_str,
            deg_str,
        }
    }
    /// Call inner xfer function
    /// This function describe the behavior of Pbs Look-up-table.
    pub fn xfer(&self, x: u8) -> Vec<u8> {
        (self.xfer_fn)(x)
    }

    /// Call inner deg function
    /// This function describe the impact of Look-up-table on the Degree.
    pub fn deg(&self, x: u8) -> u8 {
        (self.deg_fn)(x)
    }
}

impl PbsLut {
    /// Clean extra useless tokens
    /// Since PartialEq trait is implemented on closure String it's important to get ride of this
    fn cleanify_closure(s: &str) -> String {
        let mut result = s.trim().to_string();

        // Remove type annotations like ": u8"
        result = result.replace(": u8", "");

        // Clean up extra spaces
        result.split_whitespace().collect::<Vec<_>>().join(" ")
    }
    /// Function used to nomalize display
    /// Want to get ride of Rust subtilities and have something similar to Rhai syntax
    /// i.e. mainly Vec syntax to Rhai Array
    fn norm_closure_fmt(s: &str) -> String {
        // Replace vec![...] with [...]
        s.replace("vec!", "")
    }
}

impl PartialEq for PbsLut {
    fn eq(&self, other: &Self) -> bool {
        (self.xfer_str == other.xfer_str) && (self.xfer_str == other.xfer_str)
    }
}

impl std::fmt::Display for PbsLut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let norm_xfer = Self::norm_closure_fmt(&self.xfer_str);
        let norm_deg = Self::norm_closure_fmt(&self.deg_str);
        write!(f, "Pbs {{\n  xfer: {norm_xfer},\n  deg: {norm_deg}\n}}",)
    }
}
// NB: Debug fallback to Display implementation
// No way to add the xfer_fn/deg_fn properties
impl std::fmt::Debug for PbsLut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// Conveniance macro use to define new PbsLut with closure only
#[macro_export]
macro_rules! pbs_lut {
    (xfer: $xfer:expr, deg: $deg:expr) => {
        PbsLut {
            xfer_fn: Arc::new($xfer),
            deg_fn: Arc::new($deg),
            xfer_str: PbsLut::cleanify_closure(stringify!($xfer)),
            deg_str: PbsLut::cleanify_closure(stringify!($deg)),
        }
    };
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
