use super::super::{EmulatedPlaintextBlock, SpecBlock, SpecInteger};
use super::EmulatedPlaintextInteger;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn from_int() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    assert_eq!(plaintext.storage, 0b1111_0101);
    assert_eq!(plaintext.spec, spec);
    assert_eq!(plaintext.as_storage(), 0b1111_0101);
}

#[test]
fn len() {
    let spec = SpecInteger::new(16, 2, 4);
    let plaintext = EmulatedPlaintextInteger::from_int(spec, 0b1111_0000_1111_0000);
    assert_eq!(plaintext.len(), 4);
}

#[test]
fn get_block() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);

    let block0 = plaintext.get_block(0);
    assert_eq!(block0.storage, 0b0101);
    assert_eq!(
        block0,
        EmulatedPlaintextBlock::from_message(SpecBlock(2, 4), 0b0101)
    );

    let block1 = plaintext.get_block(1);
    assert_eq!(block1.storage, 0b1111);
}

#[test]
fn get_block_correct_spec() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let block = plaintext.get_block(0);
    assert_eq!(block.spec, SpecBlock(2, 4));
}

#[test]
fn as_storage_masks_extra_bits() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext = EmulatedPlaintextInteger {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    assert_eq!(plaintext.as_storage(), 0b1111_0101);
}

#[test]
fn equality_same_spec() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext1 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let plaintext2 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let plaintext3 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0110);

    assert_eq!(plaintext1, plaintext2);
    assert_ne!(plaintext1, plaintext3);
}

#[test]
fn equality_different_spec() {
    let spec1 = SpecInteger::new(8, 2, 4);
    let spec2 = SpecInteger::new(12, 2, 4);
    let plaintext1 = EmulatedPlaintextInteger::from_int(spec1, 0b1111_0101);
    let plaintext2 = EmulatedPlaintextInteger::from_int(spec2, 0b1111_0101);

    assert_ne!(plaintext1, plaintext2);
}

#[test]
fn equality_ignores_extra_bits() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext1 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let plaintext2 = EmulatedPlaintextInteger {
        storage: 0b1111_1111_1111_0101,
        spec,
    };

    assert_eq!(plaintext1, plaintext2);
}

#[test]
fn partial_ordering() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext1 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let plaintext2 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0110);
    let plaintext3 = EmulatedPlaintextInteger::from_int(SpecInteger::new(12, 2, 4), 0b1111_0101);

    assert!(plaintext1 < plaintext2);
    assert!(plaintext1.partial_cmp(&plaintext3).is_none());
}

#[test]
fn hash_consistency() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext1 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let plaintext2 = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    let plaintext3 = EmulatedPlaintextInteger {
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
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);

    assert_eq!(format!("{:?}", plaintext), "15_5_ptint");
    assert_eq!(format!("{:#?}", plaintext), "1111_0101_ptint");
}

#[test]
#[should_panic(expected = "Tried to get nonexistent block")]
fn get_block_out_of_bounds_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    let plaintext = EmulatedPlaintextInteger::from_int(spec, 0b1111_0101);
    plaintext.get_block(2);
}

#[test]
#[should_panic(expected = "exceeds maximum value for int size")]
fn from_int_overflow_panics() {
    let spec = SpecInteger::new(8, 2, 4);
    EmulatedPlaintextInteger::from_int(spec, 0b1_0000_0000);
}
