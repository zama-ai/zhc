use zhc_abc::{self as ABC, CheckOptions, Verdict, check_equivalence};
use zhc_builder::SpecBlock;
use zhc_crypto::integer_semantics::Type;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{IR, Signature};
use zhc_langs::{doplang::DopLang, hpulang::HpuLang, ioplang::IopLang, vmlang::VmLang};

fn assert_equivalent(reference: &ABC::Circuit, candidate: &ABC::Circuit, what: &str) {
    let verdict = check_equivalence(reference, candidate, &CheckOptions::default())
        .unwrap_or_else(|err| panic!("{what} programs cannot be compared: {err}"));
    match verdict {
        Verdict::Equivalent => {}
        Verdict::NotEquivalent(cex) => panic!(
            "{what} outputs differ at output bit {} for input bits {}",
            cex.output,
            cex.inputs
                .iter()
                .map(|b| if *b { '1' } else { '0' })
                .collect::<String>()
        ),
        Verdict::Undecided => panic!("{what} equivalence is undecided within the check limits"),
    }
}

pub fn check_iop_equivalence(
    reference_ir: &IR<IopLang>,
    candidate_ir: &IR<IopLang>,
    spec: SpecBlock,
    signature: &Signature<Type>,
) {
    let reference = crate::ioplang::build_circuit(reference_ir, spec, signature);
    let candidate = crate::ioplang::build_circuit(candidate_ir, spec, signature);
    assert_equivalent(&reference, &candidate, "IOP");
}

pub fn check_iop_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_ir: &IR<HpuLang>,
    spec: SpecBlock,
    signature: &Signature<Type>,
) {
    let reference = crate::ioplang::build_circuit(iop_ir, spec, signature);
    let candidate = crate::hpulang::build_circuit(hpu_ir, spec, signature);
    assert_equivalent(&reference, &candidate, "IOP and HPU");
}

pub fn check_iop_multi_hpu_equivalence(
    iop_ir: &IR<IopLang>,
    hpu_irs: &[IR<HpuLang>],
    spec: SpecBlock,
    signature: &Signature<Type>,
) {
    let reference = crate::ioplang::build_circuit(iop_ir, spec, signature);
    let candidate = crate::hpulang::build_multi_hpu_circuit(hpu_irs, spec, signature);
    assert_equivalent(&reference, &candidate, "IOP and multi-HPU");
}

pub fn check_iop_vm_equivalence(
    iop_ir: &IR<IopLang>,
    vm_ir: &IR<VmLang>,
    spec: SpecBlock,
    signature: &Signature<Type>,
) {
    let reference = crate::ioplang::build_circuit(iop_ir, spec, signature);
    let candidate = crate::vmlang::build_circuit(vm_ir, spec, signature);
    assert_equivalent(&reference, &candidate, "IOP and VM");
}

pub fn check_iop_dop_equivalence(
    iop_ir: &IR<IopLang>,
    dop_ir: &IR<DopLang>,
    lut_reg: &LutRegistry,
    spec: SpecBlock,
    signature: &Signature<Type>,
    num_registers: usize,
) {
    let mut reference = crate::ioplang::build_circuit(iop_ir, spec, signature);
    let candidate = crate::doplang::build_circuit(dop_ir, spec, num_registers, lut_reg, signature);
    while reference.n_inputs() < candidate.n_inputs() {
        reference.input();
    }
    assert_equivalent(&reference, &candidate, "IOP and DOP");
}

pub fn check_iop_multi_dop_equivalence(
    iop_ir: &IR<IopLang>,
    dop_irs: &[IR<DopLang>],
    lut_reg: &LutRegistry,
    spec: SpecBlock,
    signature: &Signature<Type>,
    num_registers: usize,
) {
    let mut reference = crate::ioplang::build_circuit(iop_ir, spec, signature);
    let candidate =
        crate::doplang::build_multi_hpu_circuit(dop_irs, spec, num_registers, lut_reg, signature);
    while reference.n_inputs() < candidate.n_inputs() {
        reference.input();
    }
    assert_equivalent(&reference, &candidate, "IOP and multi-HPU DOP");
}
