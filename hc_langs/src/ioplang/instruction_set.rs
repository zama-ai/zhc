use hc_ir::{DialectInstructionSet, Signature, sig};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::ioplang::{
    IopTypeSystem,
    lut::{Lut1Def, Lut2Def, Lut4Def, Lut8Def},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IopInstructionSet {
    Input { pos: usize, typ: IopTypeSystem },
    Output { pos: usize, typ: IopTypeSystem },
    Alias { typ: IopTypeSystem },
    DeclareCiphertext,
    LetPlaintextBlock { value: u8 },
    LetCiphertextBlock { value: u8 },
    AddCt,
    WrappingAddCt,
    TemperAddCt,
    SubCt,
    PackCt { mul: u8 },
    AddPt,
    WrappingAddPt,
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
    WrappingPbs { lut: Lut1Def },
    WrappingPbs2 { lut: Lut2Def },
    WrappingPbs4 { lut: Lut4Def },
    WrappingPbs8 { lut: Lut8Def },
}

impl Display for IopInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IopInstructionSet::Input { pos, typ } => write!(f, "input<{pos}, {typ}>"),
            IopInstructionSet::Output { pos, typ } => write!(f, "output<{pos}, {typ}>"),
            IopInstructionSet::Alias { .. } => write!(f, "alias"),
            IopInstructionSet::DeclareCiphertext => write!(f, "decl_ct"),
            IopInstructionSet::LetPlaintextBlock { value } => write!(f, "let_pt_block<{value}>"),
            IopInstructionSet::LetCiphertextBlock { value } => write!(f, "let_ct_block<{value}>"),
            IopInstructionSet::PackCt { mul } => write!(f, "pack_ct<{mul}>"),
            IopInstructionSet::AddCt => write!(f, "add_ct"),
            IopInstructionSet::WrappingAddCt => write!(f, "wrapping_add_ct"),
            IopInstructionSet::TemperAddCt => write!(f, "temper_add_ct"),
            IopInstructionSet::SubCt => write!(f, "sub_ct"),
            IopInstructionSet::AddPt => write!(f, "add_pt"),
            IopInstructionSet::WrappingAddPt => write!(f, "wrapping_add_pt"),
            IopInstructionSet::SubPt => write!(f, "sub_pt"),
            IopInstructionSet::PtSub => write!(f, "pt_sub"),
            IopInstructionSet::MulPt => write!(f, "mul_pt"),
            IopInstructionSet::ExtractCtBlock { index } => write!(f, "extract_ct_block<{index}>"),
            IopInstructionSet::ExtractPtBlock { index } => write!(f, "extract_pt_block<{index}>"),
            IopInstructionSet::StoreCtBlock { index } => write!(f, "store_ct_block<{index}>"),
            IopInstructionSet::Pbs { lut } => write!(f, "pbs<{lut:?}>"),
            IopInstructionSet::Pbs2 { lut } => write!(f, "pbs2<{lut:?}>"),
            IopInstructionSet::Pbs4 { lut } => write!(f, "pbs4<{lut:?}>"),
            IopInstructionSet::Pbs8 { lut } => write!(f, "pbs8<{lut:?}>"),
            IopInstructionSet::WrappingPbs { lut } => write!(f, "wrapping_pbs<{lut:?}>"),
            IopInstructionSet::WrappingPbs2 { lut } => write!(f, "wrapping_pbs2<{lut:?}>"),
            IopInstructionSet::WrappingPbs4 { lut } => write!(f, "wrapping_pbs4<{lut:?}>"),
            IopInstructionSet::WrappingPbs8 { lut } => write!(f, "wrapping_pbs8<{lut:?}>"),
        }
    }
}

impl DialectInstructionSet for IopInstructionSet {
    type TypeSystem = IopTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        use IopTypeSystem::*;
        match self {
            IopInstructionSet::Input { typ, .. } => sig![() -> (typ.clone())],
            IopInstructionSet::Output { typ, .. } => sig![(typ.clone()) -> ()],
            IopInstructionSet::DeclareCiphertext => sig![() -> (Ciphertext)],
            IopInstructionSet::LetPlaintextBlock { .. } => sig![() -> (PlaintextBlock)],
            IopInstructionSet::LetCiphertextBlock { .. } => sig![() -> (CiphertextBlock)],
            IopInstructionSet::AddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::WrappingAddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::TemperAddCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::SubCt => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::PackCt { .. } => {
                sig![(CiphertextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::AddPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::WrappingAddPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::SubPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::PtSub => {
                sig![(PlaintextBlock, CiphertextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::MulPt => {
                sig![(CiphertextBlock, PlaintextBlock) -> (CiphertextBlock)]
            }
            IopInstructionSet::ExtractCtBlock { .. } => sig![(Ciphertext) -> (CiphertextBlock)],
            IopInstructionSet::ExtractPtBlock { .. } => sig![(Plaintext) -> (PlaintextBlock)],
            IopInstructionSet::StoreCtBlock { .. } => {
                sig![(CiphertextBlock, Ciphertext) -> (Ciphertext)]
            }
            IopInstructionSet::Pbs { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            IopInstructionSet::Pbs2 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::Pbs4 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::Pbs8 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::WrappingPbs { .. } => sig![(CiphertextBlock) -> (CiphertextBlock)],
            IopInstructionSet::WrappingPbs2 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::WrappingPbs4 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::WrappingPbs8 { .. } => {
                sig![(CiphertextBlock) -> (CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock, CiphertextBlock)]
            }
            IopInstructionSet::Alias { typ } => sig![(typ.clone()) -> (typ.clone())],
        }
    }
}
