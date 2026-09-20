// Generated from `Zhc.Ffi.R2l.Export` by `Zhc/Ffi/R2l/GenerateRustBinding.lean`. Do not edit by hand.

use super::*;

pub fn add_hillis_steele(builder: &::zhc_builder::Builder, x: &::zhc_builder::CiphertextInteger, y: &::zhc_builder::CiphertextInteger) -> ::zhc_builder::CiphertextInteger {
    unsafe extern "C" {
        fn zhc_lean_export_add_hillis_steele(builder: *mut c_void, x: *mut c_void, y: *mut c_void, out0: *mut *mut c_void) -> *mut c_void;
    }
    prepare();
    let mut out0: *mut zhc_ciphertext_integer = std::ptr::null_mut();
    let res = unsafe { zhc_lean_export_add_hillis_steele(box_builder(builder), box_ciphertext_integer(x), box_ciphertext_integer(y), (&raw mut out0).cast()) };
    let out0 = unsafe { take(out0) };
    finish(res);
    out0.unwrap().0
}

pub fn add_ripple_carry(builder: &::zhc_builder::Builder, x: &::zhc_builder::CiphertextInteger, y: &::zhc_builder::CiphertextInteger) -> ::zhc_builder::CiphertextInteger {
    unsafe extern "C" {
        fn zhc_lean_export_add_ripple_carry(builder: *mut c_void, x: *mut c_void, y: *mut c_void, out0: *mut *mut c_void) -> *mut c_void;
    }
    prepare();
    let mut out0: *mut zhc_ciphertext_integer = std::ptr::null_mut();
    let res = unsafe { zhc_lean_export_add_ripple_carry(box_builder(builder), box_ciphertext_integer(x), box_ciphertext_integer(y), (&raw mut out0).cast()) };
    let out0 = unsafe { take(out0) };
    finish(res);
    out0.unwrap().0
}

#[macro_export]
macro_rules! for_each_export {
    ($m:ident) => {
        $m! {
            fn proven_add_hillis_steele = add_hillis_steele(x: &::zhc_builder::CiphertextInteger, y: &::zhc_builder::CiphertextInteger) -> ::zhc_builder::CiphertextInteger;
            fn proven_add_ripple_carry = add_ripple_carry(x: &::zhc_builder::CiphertextInteger, y: &::zhc_builder::CiphertextInteger) -> ::zhc_builder::CiphertextInteger;
        }
    };
}
