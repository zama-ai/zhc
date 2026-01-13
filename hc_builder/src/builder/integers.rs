use hc_ir::ValId;
use hc_utils::iter::AllEq;
use hc_utils::small::SmallVec;

pub fn blocks_count(width: u8, config: &BlockConfig) -> u8 {
    width.div_ceil(config.message_width)
}

/// Configuration parameters for homomorphic integer operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockConfig {
    pub message_width: u8,
    pub carry_width: u8,
}

impl BlockConfig {
    pub fn shift(&self) -> usize {
        2usize.pow(self.message_width as u32)
    }
}

/// A handle to an encrypted message block in the IR.
#[derive(Clone, Copy)]
pub struct CiphertextBlock {
    pub valid: ValId,
    pub config: BlockConfig,
}

/// A handle to a plaintext message block in the IR.
#[derive(Clone, Copy)]
pub struct PlaintextBlock {
    pub valid: ValId,
    pub config: BlockConfig,
}

pub struct EncryptedInteger {
    pub blocks: SmallVec<CiphertextBlock>,
    pub width: u8,
}

impl AsRef<[CiphertextBlock]> for EncryptedInteger {
    fn as_ref(&self) -> &[CiphertextBlock] {
        self.blocks().as_ref()
    }
}

impl EncryptedInteger {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn blocks(&self) -> &[CiphertextBlock] {
        self.blocks.as_slice()
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn from_blocks(width: u8, blocks: SmallVec<CiphertextBlock>) -> Self {
        assert!(blocks.iter().map(|b| b.config).all_eq().unwrap());
        assert_eq!(
            blocks.len(),
            blocks_count(width, &blocks[0].config) as usize
        );
        Self { blocks, width }
    }
}

pub struct PlaintextInteger {
    blocks: SmallVec<PlaintextBlock>,
    width: u8,
}

impl AsRef<[PlaintextBlock]> for PlaintextInteger {
    fn as_ref(&self) -> &[PlaintextBlock] {
        self.blocks().as_ref()
    }
}

impl PlaintextInteger {
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn blocks(&self) -> &[PlaintextBlock] {
        self.blocks.as_slice()
    }

    pub fn from_blocks(width: u8, blocks: SmallVec<PlaintextBlock>) -> Self {
        assert!(blocks.iter().map(|b| b.config).all_eq().unwrap());
        assert_eq!(
            blocks.len(),
            blocks_count(width, &blocks[0].config) as usize
        );
        Self { blocks, width }
    }
}
