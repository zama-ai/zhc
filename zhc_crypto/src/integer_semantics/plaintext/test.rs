use super::super::PlaintextBlockSpec;
use super::{EmulatedPlaintext, PlaintextSpec};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn spec_creation() {
    let spec = PlaintextSpec::new(16, 4);
    assert_eq!(spec.int_size(), 16);
    assert_eq!(spec.block_count(), 4);
    assert_eq!(spec.block_spec(), PlaintextBlockSpec(4));
}

#[test]
fn spec_int_mask() {
    let spec = PlaintextSpec::new(8, 4);
    assert_eq!(spec.int_mask(), 0b1111_1111);

    let spec = PlaintextSpec::new(12, 4);
    assert_eq!(spec.int_mask(), 0b1111_1111_1111);
}

#[test]
fn spec_block_mask() {
    let spec = PlaintextSpec::new(8, 4);
    assert_eq!(spec.block_mask(0), 0b1111);
    assert_eq!(spec.block_mask(1), 0b1111_0000);
}

#[test]
fn spec_overflow_checks() {
    let spec = PlaintextSpec::new(8, 4);

    assert!(!spec.overflows_int(0b1111_1111));
    assert!(spec.overflows_int(0b1_0000_0000));
    assert!(!spec.overflows_int(0));
}

#[test]
fn from_int() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext = spec.from_int(0b1111_0101);
    assert_eq!(plaintext.storage, 0b1111_0101);
    assert_eq!(plaintext.spec, spec);
}

#[test]
fn len() {
    let spec = PlaintextSpec::new(16, 4);
    let plaintext = spec.from_int(0b1111_0000_1111_0000);
    assert_eq!(plaintext.len(), 4);
}

#[test]
fn get_block() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext = spec.from_int(0b1111_0101);

    let block0 = plaintext.get_block(0);
    assert_eq!(block0.storage, 0b0101);

    let block1 = plaintext.get_block(1);
    assert_eq!(block1.storage, 0b1111);
}

#[test]
fn get_block_correct_spec() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext = spec.from_int(0b1111_0101);
    let block = plaintext.get_block(0);
    assert_eq!(block.spec, PlaintextBlockSpec(4));
}

#[test]
fn raw_int_bits() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext = EmulatedPlaintext {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    assert_eq!(plaintext.raw_int_bits(), 0b1111_0101);
}

#[test]
fn equality_same_spec() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext1 = spec.from_int(0b1111_0101);
    let plaintext2 = spec.from_int(0b1111_0101);
    let plaintext3 = spec.from_int(0b1111_0110);

    assert_eq!(plaintext1, plaintext2);
    assert_ne!(plaintext1, plaintext3);
}

#[test]
fn equality_different_spec() {
    let spec1 = PlaintextSpec::new(8, 4);
    let spec2 = PlaintextSpec::new(12, 4);
    let plaintext1 = spec1.from_int(0b1111_0101);
    let plaintext2 = spec2.from_int(0b1111_0101);

    assert_ne!(plaintext1, plaintext2);
}

#[test]
fn equality_ignores_extra_bits() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext1 = spec.from_int(0b1111_0101);
    let plaintext2 = EmulatedPlaintext {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    assert_eq!(plaintext1, plaintext2);
}

#[test]
fn partial_ordering() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext1 = spec.from_int(0b1111_0101);
    let plaintext2 = spec.from_int(0b1111_0110);
    let plaintext3 = PlaintextSpec::new(12, 4).from_int(0b1111_0101);

    assert!(plaintext1 < plaintext2);
    assert!(plaintext1.partial_cmp(&plaintext3).is_none());
}

#[test]
fn hash_consistency() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext1 = spec.from_int(0b1111_0101);
    let plaintext2 = spec.from_int(0b1111_0101);
    let plaintext3 = EmulatedPlaintext {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    let mut hasher3 = DefaultHasher::new();

    plaintext1.hash(&mut hasher1);
    plaintext2.hash(&mut hasher2);
    plaintext3.hash(&mut hasher3);

    assert_eq!(hasher1.finish(), hasher2.finish());
    assert_eq!(hasher1.finish(), hasher3.finish());
}

#[test]
fn display_formatting() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext = spec.from_int(0b1111_0101);

    assert_eq!(format!("{:?}", plaintext), "15_5_pint");
    assert_eq!(format!("{:#?}", plaintext), "1111_0101_pint");
}

#[test]
#[should_panic(expected = "Tried to get nonexistent block")]
fn get_block_out_of_bounds_panics() {
    let spec = PlaintextSpec::new(8, 4);
    let plaintext = spec.from_int(0b1111_0101);
    plaintext.get_block(2);
}

#[test]
#[should_panic(expected = "Tried to create malformed plaintext spec")]
fn spec_creation_zero_message_panics() {
    PlaintextSpec::new(8, 0);
}

#[test]
#[should_panic(expected = "Tried to create malformed plaintext spec")]
fn spec_creation_misaligned_int_size_panics() {
    PlaintextSpec::new(9, 4);
}

#[test]
#[should_panic(expected = "exceeds maximum value for int size")]
fn from_int_overflow_panics() {
    let spec = PlaintextSpec::new(8, 4);
    spec.from_int(0b1_0000_0000);
}

#[test]
#[should_panic(expected = "Tried to get block mask for nonexistent block")]
fn block_mask_out_of_bounds_panics() {
    let spec = PlaintextSpec::new(8, 4);
    spec.block_mask(2);
}
