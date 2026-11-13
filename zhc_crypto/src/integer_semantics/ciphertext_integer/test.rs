use super::super::{SpecBlock, SpecInteger};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn spec_creation() {
    let spec = SpecInteger::new(16, 2, 4);
    assert_eq!(spec.int_size(), 16);
    assert_eq!(spec.block_count(), 4);
    assert_eq!(spec.block_spec(), SpecBlock(2, 4));
}

#[test]
fn spec_int_mask() {
    let spec = SpecInteger::new(8, 2, 4);
    assert_eq!(spec.int_mask(), 0b1111_1111);

    let spec = SpecInteger::new(12, 2, 3);
    assert_eq!(spec.int_mask(), 0b111_111_111_111);

    let spec = SpecInteger::new(128, 2, 2);
    assert_eq!(spec.int_mask(), u128::MAX);
}

#[test]
fn spec_block_mask() {
    let spec = SpecInteger::new(8, 2, 4);
    assert_eq!(spec.block_mask(0), 0b1111);
    assert_eq!(spec.block_mask(1), 0b1111_0000);
}

#[test]
fn spec_overflow_checks() {
    let spec = SpecInteger::new(8, 2, 4);

    assert!(!spec.overflows_int(0b1111_1111));
    assert!(spec.overflows_int(0b1_0000_0000));
    assert!(!spec.overflows_int(0));
}

#[test]
fn from_int() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    assert_eq!(ciphertext.blocks[..2], [0b0101, 0b1111]);
    assert_eq!(ciphertext.spec, spec);
    assert_eq!(ciphertext.as_storage(), 0b1111_0101);
}

#[test]
fn len() {
    let spec = SpecInteger::new(16, 3, 4);
    let ciphertext = spec.from_int(0b1111_0000_1111_0000);
    assert_eq!(ciphertext.len(), 4);
}

#[test]
fn get_block() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);

    let block0 = ciphertext.get_block(0);
    assert_eq!(block0.storage, 0b0101);

    let block1 = ciphertext.get_block(1);
    assert_eq!(block1.storage, 0b1111);
}

#[test]
fn get_block_correct_spec() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    let block = ciphertext.get_block(0);
    assert_eq!(block.spec, SpecBlock(2, 4));
}

#[test]
fn set_block() {
    let spec = SpecInteger::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    let new_block = spec.block_spec().from_message(0b1010);

    ciphertext.set_block(0, new_block);
    assert_eq!(ciphertext.as_storage(), 0b1111_1010);
}

#[test]
fn set_block_preserves_other_blocks() {
    let spec = SpecInteger::new(12, 2, 3);
    let mut ciphertext = spec.from_int(0b111_110_101_010);
    let new_block = spec.block_spec().from_message(0b001);

    ciphertext.set_block(1, new_block);
    assert_eq!(ciphertext.as_storage(), 0b111_110_001_010);
}

#[test]
fn set_block_keeps_dirty_block() {
    let spec = SpecInteger::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    let dirty_block = spec.block_spec().from_data(0b11_1010);

    ciphertext.set_block(0, dirty_block);
    assert_eq!(ciphertext.get_block(0), dirty_block);
    assert_eq!(
        ciphertext.get_block(1),
        spec.block_spec().from_message(0b1111)
    );
}

#[test]
#[should_panic(expected = "block 0 is dirty")]
fn as_storage_dirty_carry_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    ciphertext.set_block(0, spec.block_spec().from_data(0b01_0101));
    ciphertext.as_storage();
}

#[test]
#[should_panic(expected = "block 1 is dirty")]
fn as_storage_dirty_padding_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    ciphertext.set_block(1, spec.block_spec().from_complete(0b1_00_1111));
    ciphertext.as_storage();
}

#[test]
#[should_panic(expected = "Spec mismatch")]
fn set_block_other_spec_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    ciphertext.set_block(0, SpecBlock(3, 4).from_message(0b1010));
}

#[test]
fn equality_same_spec() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let ciphertext2 = spec.from_int(0b1111_0101);
    let ciphertext3 = spec.from_int(0b1111_0110);

    assert_eq!(ciphertext1, ciphertext2);
    assert_ne!(ciphertext1, ciphertext3);
}

#[test]
fn equality_different_spec() {
    let spec1 = SpecInteger::new(8, 2, 4);
    let spec2 = SpecInteger::new(8, 3, 4);
    let ciphertext1 = spec1.from_int(0b1111_0101);
    let ciphertext2 = spec2.from_int(0b1111_0101);

    assert_ne!(ciphertext1, ciphertext2);
}

#[test]
fn equality_compares_carry_bits() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let mut ciphertext2 = spec.from_int(0b1111_0101);
    ciphertext2.set_block(0, spec.block_spec().from_data(0b01_0101));

    assert_eq!(ciphertext1.get_block(1), ciphertext2.get_block(1));
    assert_ne!(ciphertext1, ciphertext2);
}

#[test]
fn hash_consistency() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext1 = spec.from_int(0b1111_0101);
    let mut ciphertext2 = spec.from_int(0b1111_0000);
    ciphertext2.set_block(0, spec.block_spec().from_message(0b0101));

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    ciphertext1.hash(&mut hasher1);
    ciphertext2.hash(&mut hasher2);

    assert_eq!(ciphertext1, ciphertext2);
    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn display_formatting() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    assert_eq!(format!("{:?}", ciphertext), "15_5_ctint");
    assert_eq!(format!("{:#?}", ciphertext), "1111_0101_ctint");
}

#[test]
#[should_panic(expected = "Tried to get nonexistent block")]
fn get_block_out_of_bounds_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    let ciphertext = spec.from_int(0b1111_0101);
    ciphertext.get_block(2);
}

#[test]
#[should_panic(expected = "Tried to set nonexistent block")]
fn set_block_out_of_bounds_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    let mut ciphertext = spec.from_int(0b1111_0101);
    let block = spec.block_spec().from_message(0b1010);
    ciphertext.set_block(2, block);
}

#[test]
#[should_panic(expected = "carry size must be non-zero")]
fn spec_creation_zero_carry_panics() {
    SpecInteger::new(8, 0, 4);
}

#[test]
#[should_panic(expected = "message size must be non-zero")]
fn spec_creation_zero_message_panics() {
    SpecInteger::new(8, 2, 0);
}

#[test]
#[should_panic(expected = "precision must be non-zero")]
fn spec_creation_zero_precision_panics() {
    SpecInteger::new(0, 2, 2);
}

#[test]
#[should_panic(expected = "is not a multiple of message size")]
fn spec_creation_unaligned_precision_panics() {
    SpecInteger::new(7, 2, 2);
}

#[test]
#[should_panic(expected = "exceeds maximum value for int size")]
fn from_int_overflow_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    spec.from_int(0b1_0000_0000);
}

#[test]
#[should_panic(expected = "Tried to get block mask for nonexistent block")]
fn block_mask_out_of_bounds_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    spec.block_mask(2);
}
