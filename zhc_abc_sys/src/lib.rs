#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_uchar};

#[repr(C)]
pub struct Gia_Man_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn zhc_abc_gia_start(n_objs_max: c_int) -> *mut Gia_Man_t;
    pub fn zhc_abc_gia_stop(p: *mut Gia_Man_t);
    pub fn zhc_abc_gia_append_ci(p: *mut Gia_Man_t) -> c_int;
    pub fn zhc_abc_gia_hash_and(p: *mut Gia_Man_t, lit0: c_int, lit1: c_int) -> c_int;
    pub fn zhc_abc_gia_append_co(p: *mut Gia_Man_t, lit: c_int) -> c_int;
    pub fn zhc_abc_gia_ci_num(p: *mut Gia_Man_t) -> c_int;
    pub fn zhc_abc_gia_co_num(p: *mut Gia_Man_t) -> c_int;
    pub fn zhc_abc_gia_and_num(p: *mut Gia_Man_t) -> c_int;
    pub fn zhc_abc_cec_verify(
        p0: *mut Gia_Man_t,
        p1: *mut Gia_Man_t,
        conflict_limit: c_int,
        time_limit: c_int,
        verbose: c_int,
        out_fail: *mut c_int,
        cex_inputs: *mut c_uchar,
        cex_len: c_int,
    ) -> c_int;
}
