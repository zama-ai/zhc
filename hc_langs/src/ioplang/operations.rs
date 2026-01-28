use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use hc_ir::{DialectOperations, Signature, sig};

use crate::ioplang::lut::{Lut1Def, Lut2Def, Lut4Def, Lut8Def};

use super::types::Types;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Litteral {
    PlaintextBlock(u8),
}

impl Display for Litteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Litteral::PlaintextBlock(i) => write!(f, "{}_pt_block", i),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operations {
    Input { pos: usize, typ: Types },
    Output { pos: usize, typ: Types },
    LetCiphertext,
    Constant { value: Litteral },
    AddCt,
    SubCt,
    PackCt { mul: u8 },
    AddPt,
    SubPt,
    PtSub,
    MulPt,
    ExtractCtBlock { index: u8 },
    ExtractPtBlock { index: u8 },
    StoreCtBlock { index: u8 },
    Pbs { lut: Lut1Def },
    Pbs2 { lut: Lut2Def },
    Pbs4 { lut: Lut4Def },
    Pbs8 { lut: Lut8Def },
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Input { pos, typ } => write!(f, "input<{pos}, {typ}>"),
            Operations::Output { pos, typ } => write!(f, "output<{pos}, {typ}>"),
            Operations::LetCiphertext => write!(f, "let_ct"),
            Operations::Constant { value } => write!(f, "constant<{value}>"),
            Operations::PackCt { mul } => write!(f, "pack_ct<{mul}>"),
            Operations::AddCt => write!(f, "add_ct"),
            Operations::SubCt => write!(f, "sub_ct"),
            Operations::AddPt => write!(f, "add_pt"),
            Operations::SubPt => write!(f, "sub_pt"),
            Operations::PtSub => write!(f, "pt_sub"),
            Operations::MulPt => write!(f, "mul_pt"),
            Operations::ExtractCtBlock { index } => write!(f, "extract_ct_block<{index}>"),
            Operations::ExtractPtBlock { index } => write!(f, "extract_pt_block<{index}>"),
            Operations::StoreCtBlock { index } => write!(f, "store_ct_block<{index}>"),
            Operations::Pbs { lut } => write!(f, "pbs<{lut:?}>"),
            Operations::Pbs2 { lut } => write!(f, "pbs2<{lut:?}>"),
            Operations::Pbs4 { lut } => write!(f, "pbs4<{lut:?}>"),
            Operations::Pbs8 { lut } => write!(f, "pbs8<{lut:?}>"),
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
            Operations::LetCiphertext => sig![() -> (Ciphertext)],
            Operations::Constant {
                value: Litteral::PlaintextBlock(_),
            } => sig![() -> (PlaintextBlock)],
            Operations::AddCt => sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::SubCt => sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::PackCt { .. } => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            Operations::AddPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::SubPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::PtSub => sig![(PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)],
            Operations::MulPt => sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)],
            Operations::ExtractCtBlock { .. } => sig![(Ciphertext) -> (CiphertextBlock)],
            Operations::ExtractPtBlock { .. } => sig![(Plaintext) -> (PlaintextBlock)],
            Operations::StoreCtBlock { .. } => sig![(CiphertextBlock, Ciphertext) -> (Ciphertext)],
            Operations::Pbs { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            Operations::Pbs2 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock)]
            }
            Operations::Pbs4 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            Operations::Pbs8 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
        }
    }
}
