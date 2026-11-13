use std::ffi::{CString, c_char};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}
