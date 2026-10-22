use zhc::compat::Iop;
use zhc_builder::{SpecInteger, add, cmp_gt};
use zhc_config::hpu::{HpuConfig, PhysicalConfig};
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{IR, PrintWalker};
use zhc_langs::{doplang::DopLang, ioplang::IopLang};
use zhc_utils::assert_display_is;
use zhc_validation_macro::test_matrix;

use zhc_pipeline::{
    SchedPolicy, extract_lut_registry, optimize_iop,
    passes::{hpu_allocate_registers, hpu_schedule, lower_iop_to_hpu},
};
use zhc_validation::equivalence_check::check_iop_dop_equivalence;

fn pipeline(ir: &IR<IopLang>, schedp: SchedPolicy) -> (IR<DopLang>, LutRegistry, HpuConfig) {
    let lut_reg = extract_lut_registry(ir);
    let ir = lower_iop_to_hpu(ir).output;
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
    let scheduled = hpu_schedule(&ir, &config, schedp);
    let allocated = hpu_allocate_registers(&scheduled, &config, &lut_reg, 1, 0);
    (allocated, lut_reg, config)
}

#[test]
fn smoke1() {
    let ir = pipeline(
        &optimize_iop(&add(SpecInteger::new(16, 2, 2)).ir()),
        SchedPolicy::AsSoonAsPossible,
    )
    .0;
    assert_display_is!(
        ir.format(),
        r#"
            %0 = _START();
            %1 = LD<R(0), TC(1, 3)>(%0);
            %2 = LD<R(1), TC(1, 2)>(%1);
            %3 = LD<R(2), TC(1, 1)>(%2);
            %4 = LD<R(3), TC(1, 0)>(%3);
            %5 = LD<R(4), TC(0, 3)>(%4);
            %6 = ADD<R(0), R(4), R(0)>(%5);
            %7 = LD<R(4), TC(0, 2)>(%6);
            %8 = LD<R(5), TC(0, 1)>(%7);
            %9 = LD<R(6), TC(0, 0)>(%8);
            %10 = LD<R(7), TC(1, 6)>(%9);
            %11 = ADD<R(1), R(4), R(1)>(%10);
            %12 = LD<R(4), TC(1, 5)>(%11);
            %13 = LD<R(8), TC(1, 4)>(%12);
            %14 = LD<R(9), TC(0, 6)>(%13);
            %15 = LD<R(10), TC(0, 5)>(%14);
            %16 = ADD<R(2), R(5), R(2)>(%15);
            %17 = LD<R(5), TC(0, 4)>(%16);
            %18 = LD<R(11), TC(1, 7)>(%17);
            %19 = LD<R(12), TC(0, 7)>(%18);
            %20 = ADD<R(3), R(6), R(3)>(%19);
            %21 = ADD<R(6), R(9), R(7)>(%20);
            %22 = ADD<R(4), R(10), R(4)>(%21);
            %23 = ADD<R(5), R(5), R(8)>(%22);
            %24 = ADD<R(7), R(1), R(1)>(%23);
            %25 = ADD<R(8), R(12), R(11)>(%24);
            %26 = ADD<R(9), R(4), R(4)>(%25);
            %27 = PBS<R(10), R(5), LUT(1)>(%26);
            %28 = PBS<R(11), R(4), LUT(2)>(%27);
            %29 = PBS<R(12), R(6), LUT(3)>(%28);
            %30 = PBS2<R(14, 2), R(3), LUT(0)>(%29);
            %31 = PBS<R(13), R(2), LUT(1)>(%30);
            %32 = PBS<R(16), R(1), LUT(2)>(%31);
            %33 = PBSF<R(17), R(0), LUT(3)>(%32);
            %34 = ADD<R(1), R(15), R(13)>(%33);
            %35 = ST<TC(0, 0), R(14)>(%34);
            %36 = ADD<R(3), R(1), R(16)>(%35);
            %37 = ADD<R(4), R(3), R(17)>(%36);
            %38 = ADD<R(11), R(10), R(11)>(%37);
            %39 = ADD<R(12), R(11), R(12)>(%38);
            %40 = ADD<R(2), R(2), R(15)>(%39);
            %41 = ADD<R(1), R(7), R(1)>(%40);
            %42 = PBS<R(7), R(1), LUT(7)>(%41);
            %43 = PBS<R(13), R(2), LUT(6)>(%42);
            %44 = PBS<R(14), R(4), LUT(4)>(%43);
            %45 = PBSF<R(15), R(3), LUT(5)>(%44);
            %46 = ADD<R(1), R(11), R(14)>(%45);
            %47 = ST<TC(0, 2), R(7)>(%46);
            %48 = ST<TC(0, 1), R(13)>(%47);
            %49 = ADD<R(2), R(12), R(14)>(%48);
            %50 = ADD<R(3), R(10), R(14)>(%49);
            %51 = ADD<R(4), R(5), R(14)>(%50);
            %52 = ADD<R(0), R(0), R(15)>(%51);
            %53 = ADD<R(3), R(9), R(3)>(%52);
            %54 = PBS<R(5), R(3), LUT(7)>(%53);
            %55 = PBS<R(7), R(0), LUT(6)>(%54);
            %56 = PBS<R(9), R(4), LUT(6)>(%55);
            %57 = PBS<R(10), R(2), LUT(4)>(%56);
            %58 = PBSF<R(11), R(1), LUT(5)>(%57);
            %59 = ADD<R(0), R(8), R(10)>(%58);
            %60 = ST<TC(0, 5), R(5)>(%59);
            %61 = ST<TC(0, 3), R(7)>(%60);
            %62 = ST<TC(0, 4), R(9)>(%61);
            %63 = ADD<R(1), R(6), R(11)>(%62);
            %64 = PBS<R(2), R(1), LUT(6)>(%63);
            %65 = PBSF<R(3), R(0), LUT(6)>(%64);
            %66 = ST<TC(0, 6), R(2)>(%65);
            %67 = ST<TC(0, 7), R(3)>(%66);
            _END(%67);
        "#
    );
}

#[test]
fn smoke2() {
    let ir = pipeline(
        &optimize_iop(&cmp_gt(SpecInteger::new(16, 2, 2)).ir()),
        SchedPolicy::AsSoonAsPossible,
    )
    .0;
    assert_display_is!(
        ir.format().with_walker(PrintWalker::Linear),
        r#"
            %0 = _START();
            %1 = LD<R(0), TC(1, 7)>(%0);
            %2 = LD<R(1), TC(1, 6)>(%1);
            %3 = MAC<R(0), R(0), R(1), PT_I(4)>(%2);
            %4 = LD<R(1), TC(1, 5)>(%3);
            %5 = LD<R(2), TC(1, 4)>(%4);
            %6 = LD<R(3), TC(1, 3)>(%5);
            %7 = LD<R(4), TC(1, 2)>(%6);
            %8 = MAC<R(1), R(1), R(2), PT_I(4)>(%7);
            %9 = LD<R(2), TC(1, 1)>(%8);
            %10 = LD<R(5), TC(1, 0)>(%9);
            %11 = LD<R(6), TC(0, 7)>(%10);
            %12 = LD<R(7), TC(0, 6)>(%11);
            %13 = MAC<R(3), R(3), R(4), PT_I(4)>(%12);
            %14 = LD<R(4), TC(0, 5)>(%13);
            %15 = LD<R(8), TC(0, 4)>(%14);
            %16 = LD<R(9), TC(0, 3)>(%15);
            %17 = LD<R(10), TC(0, 2)>(%16);
            %18 = MAC<R(2), R(2), R(5), PT_I(4)>(%17);
            %19 = LD<R(5), TC(0, 1)>(%18);
            %20 = LD<R(11), TC(0, 0)>(%19);
            %21 = MAC<R(6), R(6), R(7), PT_I(4)>(%20);
            %22 = MAC<R(4), R(4), R(8), PT_I(4)>(%21);
            %23 = MAC<R(7), R(9), R(10), PT_I(4)>(%22);
            %24 = MAC<R(5), R(5), R(11), PT_I(4)>(%23);
            %25 = SUB<R(0), R(6), R(0)>(%24);
            %26 = SUB<R(1), R(4), R(1)>(%25);
            %27 = SUB<R(3), R(7), R(3)>(%26);
            %28 = SUB<R(2), R(5), R(2)>(%27);
            %29 = PBS<R(4), R(2), LUT(0)>(%28);
            %30 = PBS<R(5), R(3), LUT(0)>(%29);
            %31 = PBS<R(6), R(1), LUT(0)>(%30);
            %32 = PBSF<R(7), R(0), LUT(0)>(%31);
            %33 = ADDS<R(0), R(4), PT_I(1)>(%32);
            %34 = ADDS<R(1), R(5), PT_I(1)>(%33);
            %35 = ADDS<R(2), R(6), PT_I(1)>(%34);
            %36 = ADDS<R(3), R(7), PT_I(1)>(%35);
            %37 = MAC<R(0), R(1), R(0), PT_I(4)>(%36);
            %38 = MAC<R(1), R(3), R(2), PT_I(4)>(%37);
            %39 = PBS<R(2), R(1), LUT(1)>(%38);
            %40 = PBSF<R(3), R(0), LUT(1)>(%39);
            %41 = MAC<R(0), R(2), R(3), PT_I(4)>(%40);
            %42 = PBSF<R(1), R(0), LUT(2)>(%41);
            %43 = ST<TC(0, 0), R(1)>(%42);
            _END(%43);
        "#
    );
}

#[test_matrix(iop = @all_iops, size = @main_precs, schedp = @all_schedps)]
#[ignore]
fn validate_hpu_allocate_register(iop: Iop, size: u16, schedp: SchedPolicy) {
    let spec = SpecInteger::new(size, 2, 2);
    let b = iop.to_builder(spec);
    let iop_ir = optimize_iop(&b.ir());
    let (dop_ir, lut_reg, config) = pipeline(&iop_ir, schedp);
    check_iop_dop_equivalence(
        &iop_ir,
        &dop_ir,
        &lut_reg,
        spec.block_spec(),
        &b.signature(),
        config.regf_size,
    );
}
