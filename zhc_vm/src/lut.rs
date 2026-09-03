use zhc_config::vm::{LUTS_REGISTRY_SIZE, VmConfig};
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec,
    lut::{LutRegistry, RawLut},
};
use zhc_utils::SafeAs;

/// Writes the accumulator of every table of `lut_reg` into `registry`.
///
/// Table `LutId(i)` lands in slot `i`. Slots past the last registered table are left untouched.
pub fn build_registry(params: &VmConfig, lut_reg: &LutRegistry, registry: &mut [u64]) {
    let slot = params.lut_alloc_size();
    assert_eq!(registry.len(), LUTS_REGISTRY_SIZE * slot);
    let n_luts = lut_reg.iter_luts().count();
    assert!(
        n_luts <= LUTS_REGISTRY_SIZE,
        "plan uses {n_luts} lookup tables but the registry holds only {LUTS_REGISTRY_SIZE}"
    );
    assert!(
        n_luts <= u8::MAX as usize + 1,
        "plan uses {n_luts} lookup tables but bytecode ids are 8 bits wide"
    );
    for (lid, raw) in lut_reg.iter_luts() {
        let chunk = &mut registry[lid.0 * slot..(lid.0 + 1) * slot];
        build_accumulator(params, raw, chunk);
    }
}

/// Builds the GLWE accumulator of a raw table.
///
/// The raw table stores its `k` sub-tables consecutively, each covering the inputs whose top
/// `log2(k)` data bits are clear. Filling box `v` of the body with entry `v` therefore lays the
/// sub-tables out in consecutive slices of the polynomial, which is the layout the many-LUT
/// sample extraction expects.
fn build_accumulator(params: &VmConfig, raw: &RawLut, out: &mut [u64]) {
    out.fill(0);

    let n = params.bsk_polynomial_size;
    let body = &mut out[params.bsk_glwe_dim * n..];

    let spec = CiphertextBlockSpec(params.carry_size.sas(), params.message_size.sas());
    assert_eq!(
        raw.spec(),
        &spec,
        "lookup table {raw:?} does not match the VM block spec"
    );
    let modulus_sup = 1usize << spec.data_size();
    let box_size = n / modulus_sup;
    assert_eq!(raw.lut().len(), modulus_sup);

    let encode = |v: u64| v.wrapping_mul(params.delta.sas::<u64>());

    for (entry, sub_lut_box) in raw.lut().iter().zip(body.chunks_exact_mut(box_size)) {
        sub_lut_box.fill(encode(entry.raw_complete_bits().sas()));
    }

    let half_box = box_size / 2;
    for c in body[..half_box].iter_mut() {
        *c = c.wrapping_neg();
    }
    body.rotate_left(half_box);
}
