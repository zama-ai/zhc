use crate::helpers::{c_str, deref, free, guard, table, write_handle};
use crate::spec_block::zhc_spec_block;
use crate::status::zhc_status;
use std::ffi::c_char;
use zhc_builder::{Lut4, SpecBlock};

pub struct zhc_lut4(pub Lut4);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut4_free(ptr: *mut zhc_lut4) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut4_new(
    name: *const c_char,
    spec: *const zhc_spec_block,
    table_1: *const u16,
    table_2: *const u16,
    table_3: *const u16,
    table_4: *const u16,
    len: usize,
    out: *mut *mut zhc_lut4,
) -> zhc_status {
    guard(|| {
        let spec: SpecBlock = unsafe { deref(spec) }?.0;
        let name = unsafe { c_str(name) }?;
        let t1 = unsafe { table(spec, table_1, len, 2) }?;
        let t2 = unsafe { table(spec, table_2, len, 2) }?;
        let t3 = unsafe { table(spec, table_3, len, 2) }?;
        let t4 = unsafe { table(spec, table_4, len, 2) }?;
        let lut = Lut4::from_fn(
            name,
            spec,
            |b| spec.from_complete(t1[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t2[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t3[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t4[b.raw_data_bits() as usize]),
        );
        unsafe { write_handle(out, zhc_lut4(lut)) }
    })
}
