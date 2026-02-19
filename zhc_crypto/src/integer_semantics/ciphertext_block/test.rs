use super::{CiphertextBlockSpec, EmulatedCiphertextBlock};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn spec_creation() {
    let spec = CiphertextBlockSpec(3, 4);
    assert_eq!(spec.carry_size(), 3);
    assert_eq!(spec.message_size(), 4);
    assert_eq!(spec.padding_size(), 1);
    assert_eq!(spec.data_size(), 7);
    assert_eq!(spec.complete_size(), 8);
}

#[test]
fn spec_masks() {
    let spec = CiphertextBlockSpec(2, 3);
    assert_eq!(spec.message_mask(), 0b000_0_00_111);
    assert_eq!(spec.carry_mask(), 0b000_11_000);
    assert_eq!(spec.padding_mask(), 0b000_1_00_000);
    assert_eq!(spec.data_mask(), 0b000_0_11_111);
    assert_eq!(spec.complete_mask(), 0b000_1_11_111);
}

#[test]
fn spec_overflow_checks() {
    let spec = CiphertextBlockSpec(2, 3);

    assert!(!spec.overflows_message(0b000_0_00_111));
    assert!(spec.overflows_message(0b000_0_01_000));

    assert!(!spec.overflows_carry(0b000_0_11_111));
    assert!(spec.overflows_carry(0b000_1_00_000));

    assert!(!spec.overflows_padding(0b000_1_11_111));
    assert!(spec.overflows_padding(0b000_1_0_00_000));
}

#[test]
fn from_message() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = spec.from_message(0b000_0_00_101);
    assert_eq!(block.storage, 0b000_0_00_101);
    assert_eq!(block.raw_message_bits(), 0b000_0_00_101);
    assert_eq!(block.raw_carry_bits(), 0);
    assert_eq!(block.raw_padding_bits(), 0);
}

#[test]
fn from_carry() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = spec.from_carry(0b10);
    assert_eq!(block.storage, 0b000_0_10_000);
    assert_eq!(block.raw_message_bits(), 0);
    assert_eq!(block.raw_carry_bits(), 0b000_0_00_010);
    assert_eq!(block.raw_padding_bits(), 0);
}

#[test]
fn from_data() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = spec.from_data(0b000_0_11_101);
    assert_eq!(block.storage, 0b000_0_11_101);
    assert_eq!(block.raw_data_bits(), 0b11_101);
}

#[test]
fn from_complete() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = spec.from_complete(0b000_1_11_101);
    assert_eq!(block.storage, 0b000_1_11_101);
    assert_eq!(block.raw_complete_bits(), 0b1_11_101);
}

#[test]
fn raw_bit_extractors() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b000_1_11_101,
        spec,
    };

    assert_eq!(block.raw_message_bits(), 0b000_0_00_101);
    assert_eq!(block.raw_carry_bits(), 0b11);
    assert_eq!(block.raw_padding_bits(), 1);
    assert_eq!(block.raw_data_bits(), 0b11_101);
    assert_eq!(block.raw_complete_bits(), 0b1_11_101);
}

#[test]
fn raw_bit_extractors_with_extra_bits() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b011_1_11_111,
        spec,
    };

    assert_eq!(block.raw_message_bits(), 0b111);
    assert_eq!(block.raw_carry_bits(), 0b11);
    assert_eq!(block.raw_padding_bits(), 0b1);
}

#[test]
fn move_carry_to_message() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b000_0_11_000,
        spec,
    };

    let moved = block.move_carry_to_message();
    assert_eq!(moved.storage, 0b11);
}

#[test]
fn equality_same_spec() {
    let spec = CiphertextBlockSpec(2, 3);
    let block1 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let block2 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let block3 = EmulatedCiphertextBlock {
        storage: 0b1_11_111,
        spec,
    };

    assert_eq!(block1, block2);
    assert_ne!(block1, block3);
}

#[test]
fn equality_different_spec() {
    let spec1 = CiphertextBlockSpec(2, 3);
    let spec2 = CiphertextBlockSpec(3, 2);
    let block1 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec: spec1,
    };
    let block2 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec: spec2,
    };

    assert_ne!(block1, block2);
}

#[test]
fn equality_ignores_extra_bits() {
    let spec = CiphertextBlockSpec(2, 3);
    let block1 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let block2 = EmulatedCiphertextBlock {
        storage: 0b11_1_11_101,
        spec,
    };

    assert_eq!(block1, block2);
}

#[test]
fn partial_ordering() {
    let spec = CiphertextBlockSpec(2, 3);
    let block1 = EmulatedCiphertextBlock {
        storage: 0b0_00_101,
        spec,
    };
    let block2 = EmulatedCiphertextBlock {
        storage: 0b0_00_110,
        spec,
    };
    let block3 = EmulatedCiphertextBlock {
        storage: 0b0_00_101,
        spec: CiphertextBlockSpec(3, 2),
    };

    assert!(block1 < block2);
    assert!(block1.partial_cmp(&block3).is_none());
}

#[test]
fn partial_ordering_ignore_extra_bits() {
    let spec = CiphertextBlockSpec(2, 3);
    let block1 = EmulatedCiphertextBlock {
        storage: 0b0_00_101,
        spec,
    };
    let block2 = EmulatedCiphertextBlock {
        storage: 0b11_0_00_101,
        spec,
    };
    let block3 = EmulatedCiphertextBlock {
        storage: 0b0_00_110,
        spec,
    };

    assert!(block1 < block3);
    assert!(block2 < block3);
    assert_eq!(block1.partial_cmp(&block2), Some(std::cmp::Ordering::Equal));
}

#[test]
fn hash_consistency() {
    let spec = CiphertextBlockSpec(2, 3);
    let block1 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let block2 = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let block3 = EmulatedCiphertextBlock {
        storage: 0b11_1_11_101,
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
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };

    assert_eq!(format!("{:?}", block), "1_3_5_ctblock");
    assert_eq!(format!("{:#?}", block), "1_11_101_ctblock");
}

#[test]
#[should_panic(expected = "exceeds maximum value for message size")]
fn from_message_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    spec.from_message(0b1_000);
}

#[test]
#[should_panic(expected = "exceeds maximum value for carry size")]
fn from_carry_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    spec.from_carry(0b1_00);
}

#[test]
#[should_panic(expected = "exceeds maximum value for data size")]
fn from_data_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    spec.from_data(0b1_00_000);
}

#[test]
#[should_panic(expected = "exceeds maximum value for complete size")]
fn from_complete_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    spec.from_complete(0b10_00_000);
}
