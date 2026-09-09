use zhc_builder::{
    Builder, IntegerCiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, cmp_gt, count_0,
    count_1, if_then_else, if_then_zero, mul,
};
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
use zhc_utils::assert_display_is;

use crate::equivalence_check::check_iop_hpu_equivalence;

fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
    zhc_pipeline::passes::lower_iop_to_hpu(&ir).output
}

#[test]
fn test_translate_add_ir() {
    let ir = pipeline(&add(IntegerCiphertextSpec::new(16, 2, 2)).optimize_ir());
    assert_display_is!(
        ir.format(),
        r#"
            %0 = src_ld<0.0_tsrc>();
            %1 = src_ld<0.1_tsrc>();
            %2 = src_ld<0.2_tsrc>();
            %3 = src_ld<0.3_tsrc>();
            %4 = src_ld<0.4_tsrc>();
            %5 = src_ld<0.5_tsrc>();
            %6 = src_ld<0.6_tsrc>();
            %7 = src_ld<0.7_tsrc>();
            %8 = src_ld<1.0_tsrc>();
            %9 = src_ld<1.1_tsrc>();
            %10 = src_ld<1.2_tsrc>();
            %11 = src_ld<1.3_tsrc>();
            %12 = src_ld<1.4_tsrc>();
            %13 = src_ld<1.5_tsrc>();
            %14 = src_ld<1.6_tsrc>();
            %15 = src_ld<1.7_tsrc>();
            %16 = add_ct(%0, %8);
            %17 = add_ct(%1, %9);
            %18 = add_ct(%2, %10);
            %19 = add_ct(%3, %11);
            %20 = add_ct(%4, %12);
            %21 = add_ct(%5, %13);
            %22 = add_ct(%6, %14);
            %23 = add_ct(%7, %15);
            %24, %25 = pbs_2<Lut2("ManyCarryMsg")>(%16);
            %26 = pbs<Lut1("ExtractPropGroup0")>(%17);
            %27 = pbs<Lut1("ExtractPropGroup1")>(%18);
            %28 = pbs<Lut1("ExtractPropGroup2")>(%19);
            %29 = pbs<Lut1("ExtractPropGroup0")>(%20);
            %30 = pbs<Lut1("ExtractPropGroup1")>(%21);
            %31 = pbs<Lut1("ExtractPropGroup2")>(%22);
            %32 = add_ct(%25, %26);
            %33 = add_ct(%32, %27);
            %34 = add_ct(%33, %28);
            %35 = pbs<Lut1("SolvePropGroupFinal2")>(%34);
            %36 = add_ct(%29, %30);
            %37 = add_ct(%36, %31);
            %38 = pbs<Lut1("SolvePropGroupFinal0")>(%32);
            %39 = pbs<Lut1("SolvePropGroupFinal1")>(%33);
            %40 = add_ct(%29, %35);
            %41 = pbs<Lut1("SolvePropGroupFinal0")>(%40);
            %42 = add_ct(%36, %35);
            %43 = pbs<Lut1("SolvePropGroupFinal1")>(%42);
            %44 = add_ct(%37, %35);
            %45 = pbs<Lut1("SolvePropGroupFinal2")>(%44);
            %46 = add_ct(%17, %25);
            %47 = add_ct(%18, %38);
            %48 = add_ct(%19, %39);
            %49 = add_ct(%20, %35);
            %50 = add_ct(%21, %41);
            %51 = add_ct(%22, %43);
            %52 = add_ct(%23, %45);
            %53 = pbs<Lut1("MsgOnly")>(%24);
            %54 = pbs<Lut1("MsgOnly")>(%46);
            %55 = pbs<Lut1("MsgOnly")>(%47);
            %56 = pbs<Lut1("MsgOnly")>(%48);
            %57 = pbs<Lut1("MsgOnly")>(%49);
            %58 = pbs<Lut1("MsgOnly")>(%50);
            %59 = pbs<Lut1("MsgOnly")>(%51);
            %60 = pbs<Lut1("MsgOnly")>(%52);
            dst_st<0.0_tdst>(%53);
            dst_st<0.1_tdst>(%54);
            dst_st<0.2_tdst>(%55);
            dst_st<0.3_tdst>(%56);
            dst_st<0.4_tdst>(%57);
            dst_st<0.5_tdst>(%58);
            dst_st<0.6_tdst>(%59);
            dst_st<0.7_tdst>(%60);
        "#
    );
}

#[test]
fn test_translate_cmp_ir() {
    let ir = pipeline(&cmp_gt(IntegerCiphertextSpec::new(16, 2, 2)).optimize_ir());
    assert_display_is!(
        ir.format(),
        r#"
            %0 = src_ld<0.0_tsrc>();
            %1 = src_ld<0.1_tsrc>();
            %2 = src_ld<0.2_tsrc>();
            %3 = src_ld<0.3_tsrc>();
            %4 = src_ld<0.4_tsrc>();
            %5 = src_ld<0.5_tsrc>();
            %6 = src_ld<0.6_tsrc>();
            %7 = src_ld<0.7_tsrc>();
            %8 = src_ld<1.0_tsrc>();
            %9 = src_ld<1.1_tsrc>();
            %10 = src_ld<1.2_tsrc>();
            %11 = src_ld<1.3_tsrc>();
            %12 = src_ld<1.4_tsrc>();
            %13 = src_ld<1.5_tsrc>();
            %14 = src_ld<1.6_tsrc>();
            %15 = src_ld<1.7_tsrc>();
            %16 = mac<4_imm>(%1, %0);
            %17 = pbs<Lut1("None")>(%16);
            %18 = mac<4_imm>(%3, %2);
            %19 = pbs<Lut1("None")>(%18);
            %20 = mac<4_imm>(%5, %4);
            %21 = pbs<Lut1("None")>(%20);
            %22 = mac<4_imm>(%7, %6);
            %23 = pbs<Lut1("None")>(%22);
            %24 = mac<4_imm>(%9, %8);
            %25 = pbs<Lut1("None")>(%24);
            %26 = mac<4_imm>(%11, %10);
            %27 = pbs<Lut1("None")>(%26);
            %28 = mac<4_imm>(%13, %12);
            %29 = pbs<Lut1("None")>(%28);
            %30 = mac<4_imm>(%15, %14);
            %31 = pbs<Lut1("None")>(%30);
            %32 = sub_ct(%17, %25);
            %33 = pbs<Lut1("CmpSign")>(%32);
            %34 = add_cst<1_imm>(%33);
            %35 = sub_ct(%19, %27);
            %36 = pbs<Lut1("CmpSign")>(%35);
            %37 = add_cst<1_imm>(%36);
            %38 = sub_ct(%21, %29);
            %39 = pbs<Lut1("CmpSign")>(%38);
            %40 = add_cst<1_imm>(%39);
            %41 = sub_ct(%23, %31);
            %42 = pbs<Lut1("CmpSign")>(%41);
            %43 = add_cst<1_imm>(%42);
            %44 = mac<4_imm>(%37, %34);
            %45 = pbs<Lut1("CmpReduce")>(%44);
            %46 = mac<4_imm>(%43, %40);
            %47 = pbs<Lut1("CmpReduce")>(%46);
            %48 = mac<4_imm>(%47, %45);
            %49 = pbs<Lut1("CmpGtMrg")>(%48);
            dst_st<0.0_tdst>(%49);
        "#
    );
}

#[test]
fn correctness() {
    let check = |b: Builder| {
        let spec = *b.spec();
        let iop_ir = b.optimize_ir();
        let hpu_ir = pipeline(&iop_ir);
        check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
    };
    for size in (2..=64).step_by(2) {
        let spec = IntegerCiphertextSpec::new(size, 2, 2);
        check(add(spec));
        check(bitwise_and(spec));
        check(bitwise_or(spec));
        check(bitwise_xor(spec));
        check(if_then_else(spec));
        check(if_then_zero(spec));
        check(mul(spec));
        if spec.int_size().is_multiple_of(2) {
            check(count_0(spec));
            check(count_1(spec));
        }
    }
}
