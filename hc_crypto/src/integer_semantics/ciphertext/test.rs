use super::super::CiphertextBlockSpec;
use super::{Ciphertext, CiphertextSpec};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn spec_creation() {
    let spec = CiphertextSpec::new(16, 2, 4);
    assert_eq!(spec.int_size(), 16);
    assert_eq!(spec.block_count(), 4);
    assert_eq!(spec.block_spec(), CiphertextBlockSpec(2, 4));
}

#[test]
fn spec_int_mask() {
    let spec = CiphertextSpec::new(8, 2, 4);
    assert_eq!(spec.int_mask(), 0b1111_1111);

    let spec = CiphertextSpec::new(12, 2, 3);
    assert_eq!(spec.int_mask(), 0b111_111_111_111);
}

#[test]
fn spec_block_mask() {
    let spec = CiphertextSpec::new(8, 2, 4);
    assert_eq!(spec.block_mask(0), 0b1111);
    assert_eq!(spec.block_mask(1), 0b1111_0000);
}

#[test]
fn spec_overflow_checks() {
    let spec = CiphertextSpec::new(8, 2, 4);

    assert!(!spec.overflows_int(0b1111_1111));
    assert!(spec.overflows_int(0b1_0000_0000));
    assert!(!spec.overflows_int(0));
}

#[test]
fn from_int() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    assert_eq!(ciphertext.storage, 0b1111_0101);
    assert_eq!(ciphertext.spec, spec);
}

#[test]
fn len() {
    let spec = CiphertextSpec::new(16, 3, 4);
    let ciphertext = spec.from_int(0b1111_0000_1111_0000);
    assert_eq!(ciphertext.len(), 4);
}

#[test]
fn get_block() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);

    let block0 = ciphertext.get_block(0);
    assert_eq!(block0.storage, 0b0101);

    let block1 = ciphertext.get_block(1);
    assert_eq!(block1.storage, 0b1111);
}

#[test]
fn get_block_correct_spec() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    let block = ciphertext.get_block(0);
    assert_eq!(block.spec, CiphertextBlockSpec(2, 4));
}

#[test]
fn set_block() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    let new_block = spec.block_spec().from_message(0b1010);

    ciphertext.set_block(0, new_block);
    assert_eq!(ciphertext.storage, 0b1111_1010);
}

#[test]
fn set_block_preserves_other_blocks() {
    let spec = CiphertextSpec::new(12, 2, 3);
    let mut ciphertext = spec.from_int(0b111_110_101_010);
    let new_block = spec.block_spec().from_message(0b001);

    ciphertext.set_block(1, new_block);
    assert_eq!(ciphertext.storage, 0b111_110_001_010);
}

#[test]
fn raw_int_bits() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext = Ciphertext {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    assert_eq!(ciphertext.raw_int_bits(), 0b1111_0101);
}

#[test]
fn equality_same_spec() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let ciphertext2 = spec.from_int(0b1111_0101);
    let ciphertext3 = spec.from_int(0b1111_0110);

    assert_eq!(ciphertext1, ciphertext2);
    assert_ne!(ciphertext1, ciphertext3);
}

#[test]
fn equality_different_spec() {
    let spec1 = CiphertextSpec::new(8, 2, 4);
    let spec2 = CiphertextSpec::new(8, 3, 4);
    let ciphertext1 = spec1.from_int(0b1111_0101);
    let ciphertext2 = spec2.from_int(0b1111_0101);

    assert_ne!(ciphertext1, ciphertext2);
}

#[test]
fn equality_ignores_extra_bits() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let ciphertext2 = Ciphertext {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    assert_eq!(ciphertext1, ciphertext2);
}

#[test]
fn partial_ordering() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let ciphertext2 = spec.from_int(0b1111_0110);
    let ciphertext3 = CiphertextSpec::new(8, 3, 4).from_int(0b1111_0101);

    assert!(ciphertext1 < ciphertext2);
    assert!(ciphertext1.partial_cmp(&ciphertext3).is_none());
}

#[test]
fn partial_ordering_ignore_extra_bits() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0110);
    let ciphertext2 = Ciphertext {
        storage: 0b1111_1111_0101,
        spec,
    };
    let ciphertext3 = spec.from_int(0b1111_0110);

    assert!(ciphertext1 > ciphertext2);
    assert!(ciphertext1 <= ciphertext3);
    assert!(ciphertext2 < ciphertext3);
}

#[test]
fn hash_consistency() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let ciphertext2 = spec.from_int(0b1111_0101);
    let ciphertext3 = Ciphertext {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    let mut hasher3 = DefaultHasher::new();

    ciphertext1.hash(&mut hasher1);
    ciphertext2.hash(&mut hasher2);
    ciphertext3.hash(&mut hasher3);

    assert_eq!(hasher1.finish(), hasher2.finish());
    assert_eq!(hasher1.finish(), hasher3.finish());
}

#[test]
fn display_formatting() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);

    assert_eq!(format!("{}", ciphertext), "15_5_cint");
    assert_eq!(format!("{:#}", ciphertext), "1111_0101_cint");
}

#[test]
#[should_panic(expected = "Tried to get nonexistent block")]
fn get_block_out_of_bounds_panics() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    ciphertext.get_block(2);
}

#[test]
#[should_panic(expected = "Tried to set nonexistent block")]
fn set_block_out_of_bounds_panics() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    let block = spec.block_spec().from_message(0b1010);
    ciphertext.set_block(2, block);
}

#[test]
#[should_panic(expected = "Tried to set a dirty block")]
fn set_block_dirty_block_panics() {
    let spec = CiphertextSpec::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    let dirty_block = spec.block_spec().from_data(0b1_1010);
    ciphertext.set_block(0, dirty_block);
}

#[test]
#[should_panic(expected = "Tried to create malformed ciphertext spec")]
fn spec_creation_zero_carry_panics() {
    CiphertextSpec::new(8, 0, 4);
}

#[test]
#[should_panic(expected = "Tried to create malformed ciphertext spec")]
fn spec_creation_zero_message_panics() {
    CiphertextSpec::new(8, 2, 0);
}

#[test]
#[should_panic(expected = "Tried to create malformed ciphertext spec")]
fn spec_creation_misaligned_int_size_panics() {
    CiphertextSpec::new(9, 2, 4);
}

#[test]
#[should_panic(expected = "exceeds maximum value for int size")]
fn from_int_overflow_panics() {
    let spec = CiphertextSpec::new(8, 2, 4);
    spec.from_int(0b1_0000_0000);
}

#[test]
#[should_panic(expected = "Tried to get block mask for nonexistent block")]
fn block_mask_out_of_bounds_panics() {
    let spec = CiphertextSpec::new(8, 2, 4);
    spec.block_mask(2);
}
