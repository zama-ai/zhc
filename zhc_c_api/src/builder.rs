//! Bindings of [`zhc_builder::Builder`].
//!
//! Every function returns a [`zhc_status`] and writes its result, if any, to the trailing `out`
//! argument. Panics of the bound Rust method are caught and reported as [`zhc_status::Panic`].

use crate::types::*;
use ::zhc_builder::{Builder, CiphertextBlock, PlaintextBlock};
use std::ffi::c_char;

#[repr(transparent)]
pub struct zhc_builder(pub(crate) Builder);

/// Shorthand: dereferences the builder handle to the wrapped [`Builder`].
unsafe fn b<'a>(builder: *const zhc_builder) -> Fallible<&'a Builder> {
    Ok(&unsafe { deref(builder) }?.0)
}

/// Shorthand: copies the wrapped value out of a ciphertext block handle.
unsafe fn ct(ptr: *const zhc_ciphertext_block) -> Fallible<CiphertextBlock> {
    Ok(unsafe { deref(ptr) }?.0)
}

/// Shorthand: copies the wrapped value out of a plaintext block handle.
unsafe fn pt(ptr: *const zhc_plaintext_block) -> Fallible<PlaintextBlock> {
    Ok(unsafe { deref(ptr) }?.0)
}

/// Shorthand: writes a ciphertext block result to `out`.
unsafe fn out_ct(out: *mut *mut zhc_ciphertext_block, block: CiphertextBlock) -> Fallible {
    unsafe { write_handle(out, zhc_ciphertext_block(block)) }
}

// ---------------------------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_new(
    spec: zhc_ciphertext_block_spec,
    out: *mut *mut zhc_builder,
) -> zhc_status {
    guard(|| unsafe { write_handle(out, zhc_builder(Builder::new(spec.into()))) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_free(builder: *mut zhc_builder) {
    unsafe { free(builder) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_spec(
    builder: *const zhc_builder,
    out: *mut zhc_ciphertext_block_spec,
) -> zhc_status {
    guard(|| {
        let spec = *unsafe { b(builder) }?.spec();
        unsafe { write(out, spec.into()) }
    })
}

// ---------------------------------------------------------------------------------------------
// Diagnostics
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_draw(
    builder: *const zhc_builder,
    kind: zhc_ir_kind,
    out: *mut *mut zhc_file_handle,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.draw(kind.into());
        unsafe { write_handle(out, zhc_file_handle(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_debug(
    builder: *const zhc_builder,
    out: *mut *mut c_char,
) -> zhc_status {
    guard(|| unsafe { write_debug(builder.cast::<Builder>(), out) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_dump(
    builder: *const zhc_builder,
    out: *mut *mut c_char,
) -> zhc_status {
    guard(|| unsafe { write_dump(builder.cast::<Builder>(), out) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_dump_noise(
    builder: *const zhc_builder,
    out: *mut *mut c_char,
) -> zhc_status {
    guard(|| {
        let s = unsafe { b(builder) }?.dump_noise_to_string();
        unsafe { write_string(out, s) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_check_noise(builder: *const zhc_builder) -> zhc_status {
    guard(|| {
        unsafe { b(builder) }?.check_noise();
        Ok(())
    })
}

// ---------------------------------------------------------------------------------------------
// Comments
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_push_comment(
    builder: *const zhc_builder,
    comment: *const c_char,
) -> zhc_status {
    guard(|| {
        let comment = unsafe { c_str(comment) }?;
        unsafe { b(builder) }?.push_comment(comment);
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_pop_comment(builder: *const zhc_builder) -> zhc_status {
    guard(|| {
        unsafe { b(builder) }?.pop_comment();
        Ok(())
    })
}

// ---------------------------------------------------------------------------------------------
// Inputs, outputs, declarations
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_ciphertext_input(
    builder: *const zhc_builder,
    int_size: u16,
    out: *mut *mut zhc_integer_ciphertext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_ciphertext_input(int_size);
        unsafe { write_handle(out, zhc_integer_ciphertext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_ciphertext_declare(
    builder: *const zhc_builder,
    int_size: u16,
    out: *mut *mut zhc_integer_ciphertext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_ciphertext_declare(int_size);
        unsafe { write_handle(out, zhc_integer_ciphertext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_ciphertext_store_block(
    builder: *const zhc_builder,
    ct: *const zhc_integer_ciphertext,
    index: u8,
    block: *const zhc_ciphertext_block,
    out: *mut *mut zhc_integer_ciphertext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_ciphertext_store_block(
            unsafe { deref(ct) }?.0,
            index,
            unsafe { self::ct(block) }?,
        );
        unsafe { write_handle(out, zhc_integer_ciphertext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_ciphertext_get_block(
    builder: *const zhc_builder,
    ct: *const zhc_integer_ciphertext,
    index: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_ciphertext_get_block(unsafe { deref(ct) }?.0, index);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_ciphertext_inspect(
    builder: *const zhc_builder,
    src: *const zhc_integer_ciphertext,
    out: *mut *mut zhc_integer_ciphertext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_ciphertext_inspect(unsafe { deref(src) }?.0);
        unsafe { write_handle(out, zhc_integer_ciphertext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_ciphertext_output(
    builder: *const zhc_builder,
    ct: *const zhc_integer_ciphertext,
) -> zhc_status {
    guard(|| {
        unsafe { b(builder) }?.integer_ciphertext_output(unsafe { deref(ct) }?.0);
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_plaintext_input(
    builder: *const zhc_builder,
    int_size: u16,
    out: *mut *mut zhc_integer_plaintext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_plaintext_input(int_size);
        unsafe { write_handle(out, zhc_integer_plaintext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_integer_plaintext_get_block(
    builder: *const zhc_builder,
    pt: *const zhc_integer_plaintext,
    index: u8,
    out: *mut *mut zhc_plaintext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.integer_plaintext_get_block(unsafe { deref(pt) }?.0, index);
        unsafe { write_handle(out, zhc_plaintext_block(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_bool_ciphertext_input(
    builder: *const zhc_builder,
    out: *mut *mut zhc_bool_ciphertext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.bool_ciphertext_input();
        unsafe { write_handle(out, zhc_bool_ciphertext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_bool_ciphertext_from_block(
    builder: *const zhc_builder,
    block: *const zhc_ciphertext_block,
    out: *mut *mut zhc_bool_ciphertext,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.bool_ciphertext_from_block(unsafe { ct(block) }?);
        unsafe { write_handle(out, zhc_bool_ciphertext(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_bool_ciphertext_get_block(
    builder: *const zhc_builder,
    value: *const zhc_bool_ciphertext,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.bool_ciphertext_get_block(unsafe { deref(value) }?.0);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_bool_ciphertext_output(
    builder: *const zhc_builder,
    value: *const zhc_bool_ciphertext,
) -> zhc_status {
    guard(|| {
        unsafe { b(builder) }?.bool_ciphertext_output(unsafe { deref(value) }?.0);
        Ok(())
    })
}

// ---------------------------------------------------------------------------------------------
// Block constants and inspection
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_let_plaintext(
    builder: *const zhc_builder,
    value: u8,
    out: *mut *mut zhc_plaintext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_let_plaintext(value);
        unsafe { write_handle(out, zhc_plaintext_block(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_let_ciphertext(
    builder: *const zhc_builder,
    value: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_let_ciphertext(value);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_inspect(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_inspect(unsafe { ct(src) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block add
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_add_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_add_with(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_add(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_add(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_add(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r =
            unsafe { b(builder) }?.block_temper_add(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_add(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r =
            unsafe { b(builder) }?.block_wrapping_add(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block sub
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_sub_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_sub_with(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_sub(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_sub(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_sub(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r =
            unsafe { b(builder) }?.block_temper_sub(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_sub(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r =
            unsafe { b(builder) }?.block_wrapping_sub(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block shl
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_shl_with(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    amount: u8,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_shl_with(unsafe { ct(src) }?, amount, flavor.into());
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_shl(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    amount: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_shl(unsafe { ct(src) }?, amount);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_shl(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    amount: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_temper_shl(unsafe { ct(src) }?, amount);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_shl(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    amount: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_wrapping_shl(unsafe { ct(src) }?, amount);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block add plaintext
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_add_plaintext_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_add_plaintext_with(
            unsafe { ct(src_a) }?,
            unsafe { pt(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_add_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_add_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_add_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_temper_add_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_add_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_wrapping_add_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block sub plaintext
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_sub_plaintext_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_sub_plaintext_with(
            unsafe { ct(src_a) }?,
            unsafe { pt(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_sub_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_sub_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_sub_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_temper_sub_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_sub_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_wrapping_sub_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block plaintext sub
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_plaintext_sub_with(
    builder: *const zhc_builder,
    src_a: *const zhc_plaintext_block,
    src_b: *const zhc_ciphertext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_plaintext_sub_with(
            unsafe { pt(src_a) }?,
            unsafe { ct(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_plaintext_sub(
    builder: *const zhc_builder,
    src_a: *const zhc_plaintext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_plaintext_sub(unsafe { pt(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_plaintext_sub(
    builder: *const zhc_builder,
    src_a: *const zhc_plaintext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_temper_plaintext_sub(unsafe { pt(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_plaintext_sub(
    builder: *const zhc_builder,
    src_a: *const zhc_plaintext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_wrapping_plaintext_sub(unsafe { pt(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block mul plaintext
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_mul_plaintext_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_mul_plaintext_with(
            unsafe { ct(src_a) }?,
            unsafe { pt(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_mul_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_mul_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_mul_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_temper_mul_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_mul_plaintext(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_plaintext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_wrapping_mul_plaintext(unsafe { ct(src_a) }?, unsafe { pt(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block mac
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_mac_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    mul: u8,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_mac_with(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            mul,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_mac(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    mul: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_mac(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?, mul);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_mac(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    mul: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_temper_mac(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            mul,
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_mac(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    mul: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_wrapping_mac(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            mul,
        );
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block pack
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_pack_with(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    flavor: zhc_flavor,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_pack_with(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            flavor.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_pack(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_pack(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_temper_pack(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r =
            unsafe { b(builder) }?.block_temper_pack(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_pack(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_wrapping_pack(unsafe { ct(src_a) }?, unsafe { ct(src_b) }?);
        unsafe { out_ct(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Block lookups
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup_with(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut1,
    check: zhc_lookup_check,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_lookup_with(
            unsafe { ct(src) }?,
            unsafe { deref(lut) }?.0.clone(),
            check.into(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut1,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_lookup(unsafe { ct(src) }?, unsafe { deref(lut) }?.0.clone());
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_padding_lookup(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut1,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_padding_lookup(unsafe { ct(src) }?, unsafe { deref(lut) }?.0.clone());
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_wrapping_lookup(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut1,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_wrapping_lookup(unsafe { ct(src) }?, unsafe { deref(lut) }?.0.clone());
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_pack_then_lookup(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    lut: *const zhc_lut1,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_pack_then_lookup(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            unsafe { deref(lut) }?.0.clone(),
        );
        unsafe { out_ct(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_mac_then_lookup(
    builder: *const zhc_builder,
    src_a: *const zhc_ciphertext_block,
    src_b: *const zhc_ciphertext_block,
    mul: u8,
    lut: *const zhc_lut1,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_mac_then_lookup(
            unsafe { ct(src_a) }?,
            unsafe { ct(src_b) }?,
            mul,
            unsafe { deref(lut) }?.0.clone(),
        );
        unsafe { out_ct(out, r) }
    })
}

/// Writes `N` result blocks into the caller-provided `out` array of `N` handle pointers.
unsafe fn out_cts<const N: usize>(
    out: *mut *mut zhc_ciphertext_block,
    blocks: [CiphertextBlock; N],
) -> Fallible {
    if out.is_null() {
        return Err(zhc_status::NullPointer);
    }
    for (i, block) in blocks.into_iter().enumerate() {
        unsafe { out_ct(out.add(i), block) }?;
    }
    Ok(())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup2_with(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut2,
    check: zhc_lookup_check,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let (r0, r1) = unsafe { b(builder) }?.block_lookup2_with(
            unsafe { ct(src) }?,
            unsafe { deref(lut) }?.0.clone(),
            check.into(),
        );
        unsafe { out_cts(out, [r0, r1]) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup2(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut2,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let (r0, r1) = unsafe { b(builder) }?
            .block_lookup2(unsafe { ct(src) }?, unsafe { deref(lut) }?.0.clone());
        unsafe { out_cts(out, [r0, r1]) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup4_with(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut4,
    check: zhc_lookup_check,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_lookup4_with(
            unsafe { ct(src) }?,
            unsafe { deref(lut) }?.0.clone(),
            check.into(),
        );
        unsafe { out_cts(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup4(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut4,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_lookup4(unsafe { ct(src) }?, unsafe { deref(lut) }?.0.clone());
        unsafe { out_cts(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup8_with(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut8,
    check: zhc_lookup_check,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?.block_lookup8_with(
            unsafe { ct(src) }?,
            unsafe { deref(lut) }?.0.clone(),
            check.into(),
        );
        unsafe { out_cts(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_lookup8(
    builder: *const zhc_builder,
    src: *const zhc_ciphertext_block,
    lut: *const zhc_lut8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { b(builder) }?
            .block_lookup8(unsafe { ct(src) }?, unsafe { deref(lut) }?.0.clone());
        unsafe { out_cts(out, r) }
    })
}
