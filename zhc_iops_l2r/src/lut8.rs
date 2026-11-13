use crate::helpers::{c_str, deref, free, guard, table, write_handle};
use crate::spec_block::zhc_spec_block;
use crate::status::zhc_status;
use std::ffi::c_char;
use zhc_builder::{Lut8, SpecBlock};

pub struct zhc_lut8(pub Lut8);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut8_free(ptr: *mut zhc_lut8) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut8_new(
    name: *const c_char,
    spec: *const zhc_spec_block,
    table_1: *const u16,
    table_2: *const u16,
    table_3: *const u16,
    table_4: *const u16,
    table_5: *const u16,
    table_6: *const u16,
    table_7: *const u16,
    table_8: *const u16,
    len: usize,
    out: *mut *mut zhc_lut8,
) -> zhc_status {
    guard(|| {
        let spec: SpecBlock = unsafe { deref(spec) }?.0;
        let name = unsafe { c_str(name) }?;
        let t1 = unsafe { table(spec, table_1, len, 3) }?;
        let t2 = unsafe { table(spec, table_2, len, 3) }?;
        let t3 = unsafe { table(spec, table_3, len, 3) }?;
        let t4 = unsafe { table(spec, table_4, len, 3) }?;
        let t5 = unsafe { table(spec, table_5, len, 3) }?;
        let t6 = unsafe { table(spec, table_6, len, 3) }?;
        let t7 = unsafe { table(spec, table_7, len, 3) }?;
        let t8 = unsafe { table(spec, table_8, len, 3) }?;
        let lut = Lut8::from_fn(
            name,
            spec,
            |b| spec.from_complete(t1[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t2[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t3[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t4[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t5[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t6[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t7[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t8[b.raw_data_bits() as usize]),
        );
        unsafe { write_handle(out, zhc_lut8(lut)) }
    })
}
