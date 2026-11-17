use crate::ciphertext_block::zhc_ciphertext_block;
use crate::ciphertext_bool::zhc_ciphertext_bool;
use crate::ciphertext_integer::zhc_ciphertext_integer;
use crate::helpers::{deref, free, guard, out_cts, write_dump, write_handle};
use crate::lut1::zhc_lut1;
use crate::lut2::zhc_lut2;
use crate::lut4::zhc_lut4;
use crate::lut8::zhc_lut8;
use crate::plaintext_block::zhc_plaintext_block;
use crate::plaintext_integer::zhc_plaintext_integer;
use crate::spec_block::zhc_spec_block;
use crate::status::zhc_status;
use ::zhc_builder::{Builder, LookupCheck, Type};
use std::ffi::c_char;
use zhc_crypto::integer_semantics::{EmulatedCiphertextBool, EmulatedPlaintextInteger};
use zhc_langs::ioplang::IopValue;

pub struct zhc_builder(pub Builder);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_new(
    spec: *const zhc_spec_block,
    out: *mut *mut zhc_builder,
) -> zhc_status {
    guard(|| {
        let spec = unsafe { deref(spec) }?.0;
        unsafe { write_handle(out, zhc_builder(Builder::new(spec))) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_free(builder: *mut zhc_builder) {
    unsafe { free(builder) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_spec(
    builder: *const zhc_builder,
    out: *mut *mut zhc_spec_block,
) -> zhc_status {
    guard(|| {
        let spec = *unsafe { deref(builder) }?.0.spec();
        unsafe { write_handle(out, zhc_spec_block(spec)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_dump(
    builder: *const zhc_builder,
    out: *mut *mut c_char,
) -> zhc_status {
    guard(|| unsafe { write_dump(&deref(builder)?.0, out) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_integer_input(
    builder: *const zhc_builder,
    int_size: u16,
    out: *mut *mut zhc_ciphertext_integer,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .ciphertext_integer_input(int_size);
        unsafe { write_handle(out, zhc_ciphertext_integer(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_integer_declare(
    builder: *const zhc_builder,
    int_size: u16,
    out: *mut *mut zhc_ciphertext_integer,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .ciphertext_integer_declare(int_size);
        unsafe { write_handle(out, zhc_ciphertext_integer(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_integer_store_block(
    builder: *const zhc_builder,
    ct: *const zhc_ciphertext_integer,
    index: u8,
    block: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_integer,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?.0.ciphertext_integer_store_block(
            unsafe { deref(ct) }?.0,
            index,
            unsafe { deref(block) }?.0,
        );
        unsafe { write_handle(out, zhc_ciphertext_integer(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_integer_get_block(
    builder: *const zhc_builder,
    ct: *const zhc_ciphertext_integer,
    index: u8,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .ciphertext_integer_get_block(unsafe { deref(ct) }?.0, index);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_integer_output(
    builder: *const zhc_builder,
    ct: *const zhc_ciphertext_integer,
) -> zhc_status {
    guard(|| {
        unsafe { deref(builder) }?
            .0
            .ciphertext_integer_output(unsafe { deref(ct) }?.0);
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_plaintext_integer_input(
    builder: *const zhc_builder,
    int_size: u16,
    out: *mut *mut zhc_plaintext_integer,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .plaintext_integer_input(int_size);
        unsafe { write_handle(out, zhc_plaintext_integer(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_plaintext_integer_get_block(
    builder: *const zhc_builder,
    pt: *const zhc_plaintext_integer,
    index: u8,
    out: *mut *mut zhc_plaintext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .plaintext_integer_get_block(unsafe { deref(pt) }?.0, index);
        unsafe { write_handle(out, zhc_plaintext_block(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_bool_input(
    builder: *const zhc_builder,
    out: *mut *mut zhc_ciphertext_bool,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?.0.ciphertext_bool_input();
        unsafe { write_handle(out, zhc_ciphertext_bool(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_bool_store_block(
    builder: *const zhc_builder,
    block: *const zhc_ciphertext_block,
    out: *mut *mut zhc_ciphertext_bool,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .ciphertext_bool_store_block(unsafe { deref(block) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_bool(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_bool_get_block(
    builder: *const zhc_builder,
    value: *const zhc_ciphertext_bool,
    out: *mut *mut zhc_ciphertext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?
            .0
            .ciphertext_bool_get_block(unsafe { deref(value) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_ciphertext_bool_output(
    builder: *const zhc_builder,
    value: *const zhc_ciphertext_bool,
) -> zhc_status {
    guard(|| {
        unsafe { deref(builder) }?
            .0
            .ciphertext_bool_output(unsafe { deref(value) }?.0);
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_block_let_plaintext(
    builder: *const zhc_builder,
    value: u8,
    out: *mut *mut zhc_plaintext_block,
) -> zhc_status {
    guard(|| {
        let r = unsafe { deref(builder) }?.0.block_let_plaintext(value);
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
        let r = unsafe { deref(builder) }?.0.block_let_ciphertext(value);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_add(unsafe { deref(src_a) }?.0, unsafe { deref(src_b) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_sub(unsafe { deref(src_a) }?.0, unsafe { deref(src_b) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_shl(unsafe { deref(src) }?.0, amount);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?.0.block_wrapping_mac(
            unsafe { deref(src_a) }?.0,
            unsafe { deref(src_b) }?.0,
            mul,
        );
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_add_plaintext(unsafe { deref(src_a) }?.0, unsafe { deref(src_b) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_sub_plaintext(unsafe { deref(src_a) }?.0, unsafe { deref(src_b) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_plaintext_sub(unsafe { deref(src_a) }?.0, unsafe { deref(src_b) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_mul_plaintext(unsafe { deref(src_a) }?.0, unsafe { deref(src_b) }?.0);
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?
            .0
            .block_wrapping_lookup(unsafe { deref(src) }?.0, unsafe { deref(lut) }?.0.clone());
        unsafe { write_handle(out, zhc_ciphertext_block(r)) }
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
        let r = unsafe { deref(builder) }?.0.block_lookup2_with(
            unsafe { deref(src) }?.0,
            unsafe { deref(lut) }?.0.clone(),
            LookupCheck::Permissive,
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
        let r = unsafe { deref(builder) }?.0.block_lookup4_with(
            unsafe { deref(src) }?.0,
            unsafe { deref(lut) }?.0.clone(),
            LookupCheck::Permissive,
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
        let r = unsafe { deref(builder) }?.0.block_lookup8_with(
            unsafe { deref(src) }?.0,
            unsafe { deref(lut) }?.0.clone(),
            LookupCheck::Permissive,
        );
        unsafe { out_cts(out, r) }
    })
}

fn input_value(typ: &Type, blocks: &mut impl Iterator<Item = u16>) -> IopValue {
    let mut next = || blocks.next().expect("Not enough input blocks.");
    match typ {
        Type::CiphertextInteger(spec) => {
            let mut value = spec.from_int(0);
            for i in 0..spec.block_count() {
                value.set_block(i, spec.block_spec().from_complete(next()));
            }
            IopValue::CiphertextInteger(value)
        }
        Type::PlaintextInteger(spec) => {
            let int = (0..u32::from(spec.block_count())).fold(0, |acc, i| {
                acc | u128::from(next()) << (i * u32::from(spec.block_spec().message_size()))
            });
            IopValue::PlaintextInteger(EmulatedPlaintextInteger::from_int(*spec, int))
        }
        Type::CiphertextBool(spec) => IopValue::CiphertextBool(EmulatedCiphertextBool::from_block(
            spec.from_complete(next()),
        )),
    }
}

fn output_blocks(value: &IopValue) -> Vec<u16> {
    match value {
        IopValue::CiphertextInteger(v) => (0..v.len())
            .map(|i| v.get_block(i).raw_complete_bits())
            .collect(),
        IopValue::CiphertextBool(v) => vec![v.get_block().raw_complete_bits()],
        _ => unreachable!(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_builder_interpret(
    builder: *const zhc_builder,
    inputs: *const u16,
    inputs_len: usize,
    outputs: *mut u16,
    outputs_len: usize,
) -> zhc_status {
    guard(|| {
        let builder = &unsafe { deref(builder) }?.0;
        let mut blocks = unsafe { std::slice::from_raw_parts(inputs, inputs_len) }
            .iter()
            .copied();
        let values: Vec<_> = (builder.signature().get_args().iter())
            .map(|typ| input_value(typ, &mut blocks))
            .collect();
        assert!(blocks.next().is_none(), "Too many input blocks.");
        let results: Vec<u16> = (builder.interpret().with_inputs(values).get_outputs().iter())
            .flat_map(output_blocks)
            .collect();
        unsafe { std::slice::from_raw_parts_mut(outputs, outputs_len) }.copy_from_slice(&results);
        Ok(())
    })
}
