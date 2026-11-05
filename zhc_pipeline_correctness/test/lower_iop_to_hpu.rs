use zhc::compat::Iop;
use zhc_builder::{SpecInteger, add, cmp_gt};
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
use zhc_pipeline_correctness_macro::test_matrix;
use zhc_utils::assert_display_is;

use zhc_pipeline_correctness::equivalence_check::check_iop_hpu_equivalence;

fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
    zhc_pipeline::passes::lower_iop_to_hpu(&ir).output
}

#[test]
fn smoke1() {
    let ir = pipeline(&add(SpecInteger::new(16, 2, 2)).optimize_ir());
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
            %38 = add_ct(%29, %35);
            %39 = pbs<Lut1("SolvePropGroupFinal1")>(%33);
            %40 = add_ct(%36, %35);
            %41 = pbs<Lut1("SolvePropGroupFinal1")>(%40);
            %42 = add_ct(%37, %35);
            %43 = pbs<Lut1("SolvePropGroupFinal2")>(%42);
            %44 = add_ct(%17, %25);
            %45 = add_ct(%19, %39);
            %46 = add_ct(%20, %35);
            %47 = add_ct(%22, %41);
            %48 = add_ct(%23, %43);
            %49 = pbs<Lut1("MsgOnly")>(%44);
            %50 = add_ct(%18, %18);
            %51 = add_ct(%50, %32);
            %52 = pbs<Lut1("HalfMsg")>(%51);
            %53 = pbs<Lut1("MsgOnly")>(%45);
            %54 = pbs<Lut1("MsgOnly")>(%46);
            %55 = add_ct(%21, %21);
            %56 = add_ct(%55, %38);
            %57 = pbs<Lut1("HalfMsg")>(%56);
            %58 = pbs<Lut1("MsgOnly")>(%47);
            %59 = pbs<Lut1("MsgOnly")>(%48);
            dst_st<0.0_tdst>(%24);
            dst_st<0.1_tdst>(%49);
            dst_st<0.2_tdst>(%52);
            dst_st<0.3_tdst>(%53);
            dst_st<0.4_tdst>(%54);
            dst_st<0.5_tdst>(%57);
            dst_st<0.6_tdst>(%58);
            dst_st<0.7_tdst>(%59);
        "#
    );
}

#[test]
fn smoke2() {
    let ir = pipeline(&cmp_gt(SpecInteger::new(16, 2, 2)).optimize_ir());
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
            %17 = mac<4_imm>(%3, %2);
            %18 = mac<4_imm>(%5, %4);
            %19 = mac<4_imm>(%7, %6);
            %20 = mac<4_imm>(%9, %8);
            %21 = mac<4_imm>(%11, %10);
            %22 = mac<4_imm>(%13, %12);
            %23 = mac<4_imm>(%15, %14);
            %24 = sub_ct(%16, %20);
            %25 = pbs<Lut1("CmpSign")>(%24);
            %26 = add_cst<1_imm>(%25);
            %27 = sub_ct(%17, %21);
            %28 = pbs<Lut1("CmpSign")>(%27);
            %29 = add_cst<1_imm>(%28);
            %30 = sub_ct(%18, %22);
            %31 = pbs<Lut1("CmpSign")>(%30);
            %32 = add_cst<1_imm>(%31);
            %33 = sub_ct(%19, %23);
            %34 = pbs<Lut1("CmpSign")>(%33);
            %35 = add_cst<1_imm>(%34);
            %36 = mac<4_imm>(%29, %26);
            %37 = pbs<Lut1("CmpReduce")>(%36);
            %38 = mac<4_imm>(%35, %32);
            %39 = pbs<Lut1("CmpReduce")>(%38);
            %40 = mac<4_imm>(%39, %37);
            %41 = pbs<Lut1("CmpGtMrg")>(%40);
            dst_st<0.0_tdst>(%41);
        "#
    );
}

#[test_matrix(iop = @all_iops, size = @main_precs)]
#[ignore]
fn correctness(iop: Iop, size: u16) {
    let b = iop.to_builder(SpecInteger::new(size, 2, 2));
    let spec = *b.spec();
    let iop_ir = b.optimize_ir();
    let hpu_ir = pipeline(&iop_ir);
    check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
}
