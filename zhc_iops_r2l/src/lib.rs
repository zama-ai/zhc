use std::ffi::{CStr, c_char, c_void};

use ::zhc_builder::{Builder, CiphertextBool, CiphertextInteger, PlaintextInteger};
use ::zhc_iops_l2r::{
    zhc_builder, zhc_ciphertext_bool, zhc_ciphertext_integer, zhc_plaintext_integer,
};
use ::zhc_leaninit_sys as _;
use ::zhc_leanrt_sys as _;

unsafe extern "C" {
    fn zhc_iops_prepare() -> *mut c_char;
    fn zhc_iops_result(res: *mut c_void) -> *mut c_char;
    fn zhc_iops_string_free(str: *mut c_char);
    fn zhc_lean_box_builder(ptr: *mut c_void) -> *mut c_void;
    fn zhc_lean_box_ciphertext_integer(ptr: *mut c_void) -> *mut c_void;
    #[allow(dead_code)]
    fn zhc_lean_box_plaintext_integer(ptr: *mut c_void) -> *mut c_void;
    #[allow(dead_code)]
    fn zhc_lean_box_ciphertext_bool(ptr: *mut c_void) -> *mut c_void;
}

fn check(err: *mut c_char) {
    if !err.is_null() {
        let msg = unsafe { CStr::from_ptr(err) }
            .to_string_lossy()
            .into_owned();
        unsafe { zhc_iops_string_free(err) };
        panic!("{msg}");
    }
}

fn prepare() {
    check(unsafe { zhc_iops_prepare() });
}

fn box_builder(builder: &Builder) -> *mut c_void {
    let ptr = Box::into_raw(Box::new(zhc_builder(builder.clone()))).cast::<c_void>();
    unsafe { zhc_lean_box_builder(ptr) }
}

fn box_ciphertext_integer(x: &CiphertextInteger) -> *mut c_void {
    let ptr = Box::into_raw(Box::new(zhc_ciphertext_integer(*x))).cast::<c_void>();
    unsafe { zhc_lean_box_ciphertext_integer(ptr) }
}

#[allow(dead_code)]
fn box_plaintext_integer(x: &PlaintextInteger) -> *mut c_void {
    let ptr = Box::into_raw(Box::new(zhc_plaintext_integer(*x))).cast::<c_void>();
    unsafe { zhc_lean_box_plaintext_integer(ptr) }
}

#[allow(dead_code)]
fn box_ciphertext_bool(x: &CiphertextBool) -> *mut c_void {
    let ptr = Box::into_raw(Box::new(zhc_ciphertext_bool(*x))).cast::<c_void>();
    unsafe { zhc_lean_box_ciphertext_bool(ptr) }
}

unsafe fn take<T>(ptr: *mut T) -> Option<Box<T>> {
    (!ptr.is_null()).then(|| unsafe { Box::from_raw(ptr) })
}

fn finish(res: *mut c_void) {
    check(unsafe { zhc_iops_result(res) });
}

#[rustfmt::skip]
mod exports;
pub use exports::*;
