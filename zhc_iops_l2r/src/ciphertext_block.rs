use crate::helpers::free;
use zhc_builder::CiphertextBlock;

pub struct zhc_ciphertext_block(pub CiphertextBlock);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_block_free(ptr: *mut zhc_ciphertext_block) {
    unsafe { free(ptr) }
}
