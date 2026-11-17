use crate::helpers::{deref, free, guard, write_handle};
use crate::spec_integer::zhc_spec_integer;
use crate::status::zhc_status;
use zhc_builder::CiphertextInteger;

pub struct zhc_ciphertext_integer(pub CiphertextInteger);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_integer_free(ptr: *mut zhc_ciphertext_integer) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_integer_clone(
    ct: *const zhc_ciphertext_integer,
    out: *mut *mut zhc_ciphertext_integer,
) -> zhc_status {
    guard(|| {
        let value = unsafe { deref(ct) }?.0;
        unsafe { write_handle(out, zhc_ciphertext_integer(value)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_integer_spec(
    ct: *const zhc_ciphertext_integer,
    out: *mut *mut zhc_spec_integer,
) -> zhc_status {
    guard(|| {
        let spec = unsafe { deref(ct) }?.0.spec();
        unsafe { write_handle(out, zhc_spec_integer(spec)) }
    })
}
