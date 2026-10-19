use super::EmulatedCiphertextBool;
use crate::integer_semantics::SpecBlock;

#[test]
fn round_trip() {
    let spec = SpecBlock(2, 2);
    for value in [false, true] {
        let ciphertext = EmulatedCiphertextBool::from_bool(value, spec);
        assert_eq!(ciphertext.as_bool(), value);
        assert_eq!(ciphertext.spec(), spec);
        assert!(ciphertext.get_block().is_message_only());
    }
}

#[test]
#[should_panic(expected = "non-boolean block")]
fn rejects_non_boolean_message() {
    EmulatedCiphertextBool::from_block(SpecBlock(2, 2).from_message(2));
}

#[test]
#[should_panic(expected = "non-boolean block")]
fn rejects_dirty_block() {
    EmulatedCiphertextBool::from_block(SpecBlock(2, 2).from_carry(1));
}

#[test]
fn display_formatting() {
    let spec = SpecBlock(2, 2);
    let ciphertext = EmulatedCiphertextBool::from_bool(true, spec);
    assert_eq!(format!("{:?}", ciphertext), "true_ctbool");
    assert_eq!(format!("{:#?}", ciphertext), "true_ctbool");
}
