use crate::helpers::{deref, free, guard, write, write_handle};
use crate::spec_block::zhc_spec_block;
use crate::status::zhc_status;
use zhc_builder::SpecInteger;

pub struct zhc_spec_integer(pub SpecInteger);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_integer_free(ptr: *mut zhc_spec_integer) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_integer_int_size(
    spec: *const zhc_spec_integer,
    out: *mut u16,
) -> zhc_status {
    guard(|| {
        let int_size = unsafe { deref(spec) }?.0.int_size();
        unsafe { write(out, int_size) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_integer_block(
    spec: *const zhc_spec_integer,
    out: *mut *mut zhc_spec_block,
) -> zhc_status {
    guard(|| {
        let block = unsafe { deref(spec) }?.0.block_spec();
        unsafe { write_handle(out, zhc_spec_block(block)) }
    })
}
