use crate::helpers::{c_str, deref, free, guard, table, write_handle};
use crate::spec_block::zhc_spec_block;
use crate::status::zhc_status;
use std::ffi::c_char;
use zhc_builder::{Lut1, SpecBlock};

pub struct zhc_lut1(pub Lut1);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut1_free(ptr: *mut zhc_lut1) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut1_new(
    name: *const c_char,
    spec: *const zhc_spec_block,
    table_1: *const u16,
    len: usize,
    out: *mut *mut zhc_lut1,
) -> zhc_status {
    guard(|| {
        let spec: SpecBlock = unsafe { deref(spec) }?.0;
        let name = unsafe { c_str(name) }?;
        let t1 = unsafe { table(spec, table_1, len, 0) }?;
        let lut = Lut1::from_fn(name, spec, |b| {
            spec.from_complete(t1[b.raw_data_bits() as usize])
        });
        unsafe { write_handle(out, zhc_lut1(lut)) }
    })
}
