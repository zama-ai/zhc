use hc_crypto::integer_semantics::{
    CiphertextBlockSpec, CiphertextSpec, PlaintextBlockSpec, PlaintextSpec,
};
use hc_ir::ValId;
use hc_utils::iter::AllEq;
use hc_utils::small::SmallVec;

/// A handle to an encrypted message block in the IR.
#[derive(Clone, Copy)]
pub struct CiphertextBlock {
    pub valid: ValId,
    pub spec: CiphertextBlockSpec,
}

pub struct Ciphertext {
    pub blocks: SmallVec<CiphertextBlock>,
    pub spec: CiphertextSpec,
}

impl AsRef<[CiphertextBlock]> for Ciphertext {
    fn as_ref(&self) -> &[CiphertextBlock] {
        self.blocks().as_ref()
    }
}

impl Ciphertext {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn blocks(&self) -> &[CiphertextBlock] {
        self.blocks.as_slice()
    }

    pub fn int_size(&self) -> u16 {
        self.spec.int_size()
    }

    pub fn from_blocks(blocks: SmallVec<CiphertextBlock>) -> Self {
        assert!(!blocks.is_empty());
        assert!(blocks.iter().map(|b| b.spec).all_eq().unwrap());
        let spec = CiphertextSpec::new(
            (blocks.len() * blocks[0].spec.message_size() as usize) as u16,
            blocks[0].spec.carry_size(),
            blocks[0].spec.message_size(),
        );
        Self { blocks, spec }
    }
}

/// A handle to a plaintext message block in the IR.
#[derive(Clone, Copy)]
pub struct PlaintextBlock {
    pub valid: ValId,
    pub spec: PlaintextBlockSpec,
}

pub struct Plaintext {
    pub blocks: SmallVec<PlaintextBlock>,
    pub spec: PlaintextSpec,
}

impl AsRef<[PlaintextBlock]> for Plaintext {
    fn as_ref(&self) -> &[PlaintextBlock] {
        self.blocks().as_ref()
    }
}

impl Plaintext {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn int_size(&self) -> u16 {
        self.spec.int_size()
    }

    pub fn blocks(&self) -> &[PlaintextBlock] {
        self.blocks.as_slice()
    }

    pub fn from_blocks(blocks: SmallVec<PlaintextBlock>) -> Self {
        assert!(!blocks.is_empty());
        assert!(blocks.iter().map(|b| b.spec).all_eq().unwrap());
        let spec = PlaintextSpec::new(
            (blocks.len() * blocks[0].spec.message_size() as usize) as u16,
            blocks[0].spec.message_size(),
        );
        Self { blocks, spec }
    }
}
