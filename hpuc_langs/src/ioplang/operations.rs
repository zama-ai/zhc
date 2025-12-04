use std::{fmt::{Debug, Display}, hash::Hash, rc::Rc};

use hpuc_ir::{DialectOperations, Signature, sig};

use super::types::Types;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Litteral {
    PlaintextBlock(usize),
    Index(usize),
}

impl Display for Litteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Litteral::PlaintextBlock(i) => write!(f, "{}_pt_block", i),
            Litteral::Index(i) => write!(f, "{}_idx", i),
        }
    }
}

#[derive(Clone)]
pub struct LutGenerator(Rc<Box<dyn Fn(u8) -> u8>>);

impl LutGenerator {

    pub fn new(f: impl Fn(u8) -> u8 + 'static) -> Self {
        LutGenerator(Rc::new(Box::new(f)))
    }

    pub fn identity() -> Self {
        LutGenerator(Rc::new(Box::new(|a| a)))
    }
}

impl Debug for LutGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LutGenerator({:p})", Rc::as_ptr(&self.0))
    }
}

impl PartialEq for LutGenerator {
    fn eq(&self, other: &Self) -> bool {
        let self_ptr = Rc::as_ptr(&self.0);
        let other_ptr = Rc::as_ptr(&other.0);
        self_ptr.eq(&other_ptr)
    }
}

impl Eq for LutGenerator {}

impl Hash for LutGenerator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operations {
    Input { pos: usize, typ: Types },
    Output { pos: usize, typ: Types },
    Let { typ: Types },
    Constant { value: Litteral },
    GenerateLut { name: String, gene: LutGenerator},
    GenerateLut2 { name: String, gene: [LutGenerator; 2]},
    GenerateLut4 { name: String, gene: [LutGenerator; 4]},
    GenerateLut8 { name: String, gene: [LutGenerator; 8]},
    AddCt,
    SubCt,
    Mac,
    AddPt,
    SubPt,
    PtSub,
    MulPt,
    ExtractCtBlock,
    ExtractPtBlock,
    StoreCtBlock,
    Pbs,
    Pbs2,
    Pbs4,
    Pbs8,
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Input { pos, typ } => write!(f, "input<{}, {}>", pos, typ),
            Operations::Output { pos, typ } => write!(f, "output<{}, {}>", pos, typ),
            Operations::Let { typ } => write!(f, "let<{}>", typ),
            Operations::Constant { value } => write!(f, "constant<{}>", value),
            Operations::GenerateLut { name, .. } => write!(f, "gen_lut1<{name}>"),
            Operations::GenerateLut2 { name, .. } => write!(f, "gen_lut2<{name}>"),
            Operations::GenerateLut4 { name, .. } => write!(f, "gen_lut4<{name}>"),
            Operations::GenerateLut8 { name, .. } => write!(f, "gen_lut8<{name}>"),
            Operations::Mac => write!(f, "mac"),
            Operations::AddCt => write!(f, "add_ct"),
            Operations::SubCt => write!(f, "sub_ct"),
            Operations::AddPt => write!(f, "add_pt"),
            Operations::SubPt => write!(f, "sub_pt"),
            Operations::PtSub => write!(f, "pt_sub"),
            Operations::MulPt => write!(f, "mul_pt"),
            Operations::ExtractCtBlock => write!(f, "extract_ct_block"),
            Operations::ExtractPtBlock => write!(f, "extract_pt_block"),
            Operations::StoreCtBlock => write!(f, "store_ct_block"),
            Operations::Pbs => write!(f, "pbs"),
            Operations::Pbs2 => write!(f, "pbs2"),
            Operations::Pbs4 => write!(f, "pbs4"),
            Operations::Pbs8 => write!(f, "pbs8"),
        }
    }
}

impl DialectOperations for Operations {
    type Types = Types;

    fn get_signature(&self) -> Signature<Self::Types> {
        use Types::*;
        match self {
            Operations::Input { typ, .. } => sig![() -> (typ.clone())],
            Operations::Output { typ, .. } => sig![(typ.clone()) -> ()],
            Operations::Let { typ, .. } => sig![() -> (typ.clone())],
            Operations::Constant {
                value: Litteral::PlaintextBlock(_),
            } => sig![() -> (PlaintextBlock)],
            Operations::Constant {
                value: Litteral::Index(_),
            } => sig![() -> (Index)],
            Operations::GenerateLut { .. } => sig![()-> (Lut1)],
            Operations::GenerateLut2 { .. } => sig![()-> (Lut2)],
            Operations::GenerateLut4 { .. } => sig![()-> (Lut4)],
            Operations::GenerateLut8 { .. } => sig![()-> (Lut8)],
            Operations::AddCt => sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::SubCt => sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::Mac => {
                sig![( PlaintextBlock, CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            Operations::AddPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::SubPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::PtSub => sig![(PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::MulPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::ExtractCtBlock => sig![(Ciphertext, Index) -> (CiphertextBlock)],
            Operations::ExtractPtBlock => sig![(Plaintext, Index) -> (PlaintextBlock)],
            Operations::StoreCtBlock => sig![(CiphertextBlock, Ciphertext, Index) -> (Ciphertext)],
            Operations::Pbs => sig![(CiphertextBlock, Lut1) -> (CiphertextBlock)],
            Operations::Pbs2 => sig![(CiphertextBlock, Lut2) -> (CiphertextBlock, CiphertextBlock)],
            Operations::Pbs4 => {
                sig![(CiphertextBlock, Lut4) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            Operations::Pbs8 => {
                sig![(CiphertextBlock, Lut8) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
        }
    }
}
