use super::EmulatedBoolCiphertext;
use crate::integer_semantics::CiphertextBlockSpec;

#[test]
fn round_trip() {
    let spec = CiphertextBlockSpec(2, 2);
    for value in [false, true] {
        let ciphertext = EmulatedBoolCiphertext::from_bool(value, spec);
        assert_eq!(ciphertext.as_bool(), value);
        assert_eq!(ciphertext.spec(), spec);
        assert!(ciphertext.get_block().is_message_only());
    }
}

#[test]
#[should_panic(expected = "non-boolean block")]
fn rejects_non_boolean_message() {
    EmulatedBoolCiphertext::from_block(CiphertextBlockSpec(2, 2).from_message(2));
}

#[test]
#[should_panic(expected = "non-boolean block")]
fn rejects_dirty_block() {
    EmulatedBoolCiphertext::from_block(CiphertextBlockSpec(2, 2).from_carry(1));
}
