use crate::ciphertext_block::zhc_ciphertext_block;
use crate::status::zhc_status;
use ::zhc_builder::{CiphertextBlock, SpecBlock};
use std::ffi::{CStr, CString, c_char};
use zhc_utils::Dumpable;

pub(crate) fn guard(f: impl FnOnce() -> Result<(), zhc_status>) -> zhc_status {
    match f() {
        Ok(()) => zhc_status::ZHC_OK,
        Err(status) => status,
    }
}

pub(crate) unsafe fn deref<'a, T>(ptr: *const T) -> Result<&'a T, zhc_status> {
    if ptr.is_null() {
        return Err(zhc_status::ZHC_ERR_NULL_POINTER);
    }
    Ok(unsafe { &*ptr })
}

pub(crate) unsafe fn write<T>(out: *mut T, value: T) -> Result<(), zhc_status> {
    if out.is_null() {
        return Err(zhc_status::ZHC_ERR_NULL_POINTER);
    }
    unsafe { out.write(value) };
    Ok(())
}

pub(crate) unsafe fn write_handle<T>(out: *mut *mut T, value: T) -> Result<(), zhc_status> {
    unsafe { write(out, Box::into_raw(Box::new(value))) }
}

pub(crate) unsafe fn free<T>(ptr: *mut T) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) });
    }
}

pub(crate) unsafe fn c_str(s: *const c_char) -> Result<String, zhc_status> {
    unsafe { deref(s) }?;
    Ok(unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned())
}

pub(crate) unsafe fn write_string(out: *mut *mut c_char, s: String) -> Result<(), zhc_status> {
    let s = CString::new(s).map_err(|_| zhc_status::ZHC_ERR_INVALID_ARGUMENT)?;
    unsafe { write(out, s.into_raw()) }
}

pub(crate) unsafe fn write_dump<T: Dumpable>(
    value: &T,
    out: *mut *mut c_char,
) -> Result<(), zhc_status> {
    unsafe { write_string(out, value.dump_to_string()) }
}

pub(crate) unsafe fn table<'a>(
    spec: SpecBlock,
    table: *const u16,
    len: usize,
    k: u8,
) -> Result<&'a [u16], zhc_status> {
    let data_size = spec.data_size();
    if data_size < k {
        return Err(zhc_status::ZHC_ERR_INVALID_ARGUMENT);
    }
    if table.is_null() {
        return Err(zhc_status::ZHC_ERR_NULL_POINTER);
    }
    if len != 1usize << (data_size - k) {
        return Err(zhc_status::ZHC_ERR_INVALID_ARGUMENT);
    }
    let table = unsafe { std::slice::from_raw_parts(table, len) };
    if table.iter().any(|&v| v > spec.complete_mask()) {
        return Err(zhc_status::ZHC_ERR_INVALID_ARGUMENT);
    }
    Ok(table)
}

pub(crate) unsafe fn out_cts<const N: usize>(
    out: *mut *mut zhc_ciphertext_block,
    blocks: [CiphertextBlock; N],
) -> Result<(), zhc_status> {
    if out.is_null() {
        return Err(zhc_status::ZHC_ERR_NULL_POINTER);
    }
    for (i, block) in blocks.into_iter().enumerate() {
        unsafe { write_handle(out.add(i), zhc_ciphertext_block(block)) }?;
    }
    Ok(())
}
