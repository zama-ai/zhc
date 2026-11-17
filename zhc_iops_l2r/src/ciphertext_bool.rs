use crate::helpers::{deref, free, guard, write_handle};
use crate::spec_block::zhc_spec_block;
use crate::status::zhc_status;
use zhc_builder::CiphertextBool;

pub struct zhc_ciphertext_bool(pub CiphertextBool);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_bool_free(ptr: *mut zhc_ciphertext_bool) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_bool_clone(
    value: *const zhc_ciphertext_bool,
    out: *mut *mut zhc_ciphertext_bool,
) -> zhc_status {
    guard(|| {
        let value = unsafe { deref(value) }?.0;
        unsafe { write_handle(out, zhc_ciphertext_bool(value)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_bool_spec(
    value: *const zhc_ciphertext_bool,
    out: *mut *mut zhc_spec_block,
) -> zhc_status {
    guard(|| {
        let spec = unsafe { deref(value) }?.0.spec();
        unsafe { write_handle(out, zhc_spec_block(spec)) }
    })
}
