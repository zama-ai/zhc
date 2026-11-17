use crate::helpers::{deref, free, guard, write, write_handle};
use crate::status::zhc_status;
use zhc_builder::SpecBlock;

pub struct zhc_spec_block(pub SpecBlock);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_block_new(
    carry: u8,
    message: u8,
    out: *mut *mut zhc_spec_block,
) -> zhc_status {
    guard(|| unsafe { write_handle(out, zhc_spec_block(SpecBlock(carry, message))) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_block_free(ptr: *mut zhc_spec_block) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_block_carry(
    spec: *const zhc_spec_block,
    out: *mut u8,
) -> zhc_status {
    guard(|| {
        let carry = unsafe { deref(spec) }?.0.0;
        unsafe { write(out, carry) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_spec_block_message(
    spec: *const zhc_spec_block,
    out: *mut u8,
) -> zhc_status {
    guard(|| {
        let message = unsafe { deref(spec) }?.0.1;
        unsafe { write(out, message) }
    })
}
