use crate::helpers::free;
use zhc_builder::PlaintextBlock;

pub struct zhc_plaintext_block(pub PlaintextBlock);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_plaintext_block_free(ptr: *mut zhc_plaintext_block) {
    unsafe { free(ptr) }
}
