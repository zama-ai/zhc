use std::fmt::{Debug, Display};

use crate::{
    gir::{DialectOperations, Signature},
    sig, svec,
};

use super::types::Types;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operations {
    Input { pos: usize, typ: Types },
    Constant { value: Litteral },
    AddCt,
    SubCt,
    Mac,
    AddPt,
    SubPt,
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
            Operations::Constant { value } => write!(f, "constant<{}>", value),
            Operations::Mac => write!(f, "mac"),
            Operations::AddCt => write!(f, "add_ct"),
            Operations::SubCt => write!(f, "sub_ct"),
            Operations::AddPt => write!(f, "add_pt"),
            Operations::SubPt => write!(f, "sub_pt"),
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
            Operations::Constant {
                value: Litteral::PlaintextBlock(_),
            } => sig![() -> (PlaintextBlock)],
            Operations::Constant {
                value: Litteral::Index(_),
            } => sig![() -> (Index)],
            Operations::AddCt => sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::SubCt => sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::Mac => {
                sig![(CiphertextBlock, PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            Operations::AddPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::SubPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::MulPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::ExtractCtBlock => sig![(Ciphertext, Index) -> (CiphertextBlock)],
            Operations::ExtractPtBlock => sig![(Plaintext, Index) -> (PlaintextBlock)],
            Operations::StoreCtBlock => sig![(Ciphertext, Index, CiphertextBlock) -> (Ciphertext)],
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
