use super::super::SpecBlock;
use super::EmulatedPlaintextBlock;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn from_message() {
    let spec = SpecBlock(2, 4);
    let block = EmulatedPlaintextBlock::from_message(spec, 0b1010);
    assert_eq!(block.storage, 0b1010);
    assert_eq!(block.spec, spec);
}

#[test]
fn from_complete() {
    let spec = SpecBlock(2, 4);
    let block = EmulatedPlaintextBlock::from_complete(spec, 0b1_11_1010);
    assert_eq!(block.storage, 0b1_11_1010);
    assert_eq!(block.raw_complete_bits(), 0b1_11_1010);
    assert_eq!(block.raw_message_bits(), 0b1010);
    assert_eq!(block.raw_carry_bits(), 0b11);
}

#[test]
fn raw_message_bits() {
    let spec = SpecBlock(2, 4);
    let block = EmulatedPlaintextBlock {
        storage: 0b1111_1010,
        spec,
    };
    assert_eq!(block.raw_message_bits(), 0b1010);
}

#[test]
fn raw_complete_bits_ignores_high_bits() {
    let spec = SpecBlock(2, 3);
    let block = EmulatedPlaintextBlock {
        storage: 0b11_1_11_010,
        spec,
    };
    assert_eq!(block.raw_complete_bits(), 0b1_11_010);
}

#[test]
fn has_only_predicates() {
    let spec = SpecBlock(2, 3);
    assert!(EmulatedPlaintextBlock::from_message(spec, 0b101).is_message_only());
    assert!(!EmulatedPlaintextBlock::from_complete(spec, 0b0_10_000).is_message_only());
    assert!(EmulatedPlaintextBlock::from_complete(spec, 0b0_10_000).is_carry_only());
    assert!(!EmulatedPlaintextBlock::from_complete(spec, 0b0_10_001).is_carry_only());
    assert!(!EmulatedPlaintextBlock::from_complete(spec, 0b1_10_000).is_carry_only());
}

#[test]
fn equality_same_spec() {
    let spec = SpecBlock(2, 4);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block3 = EmulatedPlaintextBlock {
        storage: 0b1011,
        spec,
    };

    assert_eq!(block1, block2);
    assert_ne!(block1, block3);
}

#[test]
fn equality_different_spec() {
    let spec1 = SpecBlock(2, 4);
    let spec2 = SpecBlock(2, 3);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec: spec1,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec: spec2,
    };

    assert_ne!(block1, block2);
}

#[test]
fn equality_ignores_extra_bits() {
    let spec = SpecBlock(2, 4);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b1_0_00_1010,
        spec,
    };

    assert_eq!(block1, block2);
}

#[test]
fn partial_ordering() {
    let spec = SpecBlock(2, 4);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b1100,
        spec,
    };
    let block3 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec: SpecBlock(2, 3),
    };

    assert!(block1 < block2);
    assert!(block1.partial_cmp(&block3).is_none());
}

#[test]
fn hash_consistency() {
    let spec = SpecBlock(2, 4);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block3 = EmulatedPlaintextBlock {
        storage: 0b1_0_00_1010,
        spec,
    };

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    let mut hasher3 = DefaultHasher::new();

    block1.hash(&mut hasher1);
    block2.hash(&mut hasher2);
    block3.hash(&mut hasher3);

    assert_eq!(hasher1.finish(), hasher2.finish());
    assert_eq!(hasher1.finish(), hasher3.finish());
}

#[test]
fn display_formatting() {
    let spec = SpecBlock(2, 4);
    let block = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };

    assert_eq!(format!("{:?}", block), "0_0_10_ptblock");
    assert_eq!(format!("{:#?}", block), "0_00_1010_ptblock");
}

#[test]
#[should_panic(expected = "exceeds maximum value for message size")]
fn from_message_overflow_panics() {
    let spec = SpecBlock(2, 3);
    EmulatedPlaintextBlock::from_message(spec, 0b1_000);
}

#[test]
#[should_panic(expected = "exceeds maximum value for complete size")]
fn from_complete_overflow_panics() {
    let spec = SpecBlock(2, 3);
    EmulatedPlaintextBlock::from_complete(spec, 0b10_00_000);
}
