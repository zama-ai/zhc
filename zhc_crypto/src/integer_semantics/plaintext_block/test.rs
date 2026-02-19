use super::{EmulatedPlaintextBlock, PlaintextBlockSpec};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn spec_creation() {
    let spec = PlaintextBlockSpec(4);
    assert_eq!(spec.message_size(), 4);
}

#[test]
fn spec_message_mask() {
    let spec = PlaintextBlockSpec(3);
    assert_eq!(spec.message_mask(), 0b111);

    let spec = PlaintextBlockSpec(1);
    assert_eq!(spec.message_mask(), 0b1);

    let spec = PlaintextBlockSpec(8);
    assert_eq!(spec.message_mask(), 0b1111_1111);
}

#[test]
fn spec_overflow_checks() {
    let spec = PlaintextBlockSpec(3);

    assert!(!spec.overflows_message(0b111));
    assert!(spec.overflows_message(0b1_000));
    assert!(!spec.overflows_message(0));
}

#[test]
fn from_message() {
    let spec = PlaintextBlockSpec(4);
    let block = spec.from_message(0b1010);
    assert_eq!(block.storage, 0b1010);
    assert_eq!(block.spec, spec);
}

#[test]
fn raw_message_bits() {
    let spec = PlaintextBlockSpec(4);
    let block = EmulatedPlaintextBlock {
        storage: 0b1111_1010,
        spec,
    };
    assert_eq!(block.raw_message_bits(), 0b1010);
}

#[test]
fn raw_mask_message() {
    let spec = PlaintextBlockSpec(4);
    let block = EmulatedPlaintextBlock {
        storage: 0b1111_1010,
        spec,
    };
    assert_eq!(block.raw_mask_message(), 0b1010);
}

#[test]
fn raw_mask_message_ignores_high_bits() {
    let spec = PlaintextBlockSpec(3);
    let block = EmulatedPlaintextBlock {
        storage: 0b1111_1010,
        spec,
    };
    assert_eq!(block.raw_mask_message(), 0b010);
}

#[test]
fn equality_same_spec() {
    let spec = PlaintextBlockSpec(4);
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
    let spec1 = PlaintextBlockSpec(4);
    let spec2 = PlaintextBlockSpec(3);
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
    let spec = PlaintextBlockSpec(4);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b11111010,
        spec,
    };

    assert_eq!(block1, block2);
}

#[test]
fn partial_ordering() {
    let spec = PlaintextBlockSpec(4);
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
        spec: PlaintextBlockSpec(3),
    };

    assert!(block1 < block2);
    assert!(block1.partial_cmp(&block3).is_none());
}

#[test]
fn hash_consistency() {
    let spec = PlaintextBlockSpec(4);
    let block1 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block2 = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };
    let block3 = EmulatedPlaintextBlock {
        storage: 0b1111_1010,
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
    let spec = PlaintextBlockSpec(4);
    let block = EmulatedPlaintextBlock {
        storage: 0b1010,
        spec,
    };

    assert_eq!(format!("{:?}", block), "10_ptblock");
    assert_eq!(format!("{:#?}", block), "1010_ptblock");
}

#[test]
#[should_panic(expected = "exceeds maximum value for message size")]
fn from_message_overflow_panics() {
    let spec = PlaintextBlockSpec(3);
    spec.from_message(0b1_000);
}
