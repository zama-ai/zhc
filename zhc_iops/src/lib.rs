use std::ffi::{CStr, c_char, c_void};
use std::fmt;

use zhc_builder::{Builder, IntegerCiphertextSpec};
use zhc_c_api as _;

unsafe extern "C" {
    fn zhc_iops_string_free(str: *mut c_char);
    fn zhc_iops_add_ripple_carry(int_size: u16, out: *mut *mut c_void) -> *mut c_char;
    fn zhc_iops_add_hillis_steele(int_size: u16, out: *mut *mut c_void) -> *mut c_char;
    fn zhc_iops_add_tree(int_size: u16, out: *mut *mut c_void) -> *mut c_char;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

fn take(err: *mut c_char, out: *mut c_void) -> Result<Builder, Error> {
    if !err.is_null() {
        let msg = unsafe { CStr::from_ptr(err) }.to_string_lossy().into_owned();
        unsafe { zhc_iops_string_free(err) };
        return Err(Error(msg));
    }
    Ok(*unsafe { Box::from_raw(out.cast::<Builder>()) })
}

fn check_block_spec(name: &str, spec: IntegerCiphertextSpec) -> Result<(), Error> {
    let block = spec.block_spec();
    if block.carry_size() != 2 || block.message_size() != 2 {
        return Err(Error(format!(
            "{name}: only carry=2 message=2 is supported, got carry={} message={}",
            block.carry_size(),
            block.message_size()
        )));
    }
    Ok(())
}

fn emit_int(
    name: &str,
    emit: unsafe extern "C" fn(u16, *mut *mut c_void) -> *mut c_char,
    spec: IntegerCiphertextSpec,
) -> Result<Builder, Error> {
    check_block_spec(name, spec)?;
    let mut out = std::ptr::null_mut();
    let err = unsafe { emit(spec.int_size(), &mut out) };
    take(err, out)
}

pub fn add_ripple_carry(spec: IntegerCiphertextSpec) -> Result<Builder, Error> {
    emit_int("add_ripple_carry", zhc_iops_add_ripple_carry, spec)
}

pub fn add_hillis_steele(spec: IntegerCiphertextSpec) -> Result<Builder, Error> {
    emit_int("add_hillis_steele", zhc_iops_add_hillis_steele, spec)
}

pub fn add_tree(spec: IntegerCiphertextSpec) -> Result<Builder, Error> {
    emit_int("add_tree", zhc_iops_add_tree, spec)
}
