use crate::helpers::{deref, free, guard, write_handle};
use crate::spec_integer::zhc_spec_integer;
use crate::status::zhc_status;
use zhc_builder::PlaintextInteger;

pub struct zhc_plaintext_integer(pub PlaintextInteger);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_plaintext_integer_free(ptr: *mut zhc_plaintext_integer) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_plaintext_integer_spec(
    pt: *const zhc_plaintext_integer,
    out: *mut *mut zhc_spec_integer,
) -> zhc_status {
    guard(|| {
        let spec = unsafe { deref(pt) }?.0.spec();
        unsafe { write_handle(out, zhc_spec_integer(spec)) }
    })
}
