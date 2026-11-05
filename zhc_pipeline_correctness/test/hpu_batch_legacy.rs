use zhc::compat::Iop;
use zhc_builder::{SpecInteger, add};
use zhc_config::hpu::PhysicalConfig;
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
use zhc_pipeline_correctness::equivalence_check::check_iop_hpu_equivalence;
use zhc_pipeline_correctness_macro::test_matrix;
use zhc_utils::assert_display_is;

use zhc_config::hpu::HpuConfig;
use zhc_pipeline::{
    SchedPolicy,
    passes::{hpu_batch_legacy, lower_iop_to_hpu},
};

fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
    let ir = lower_iop_to_hpu(&ir).output;
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
    hpu_batch_legacy(&ir, &config, SchedPolicy::AsSoonAsPossible)
}

#[test]
fn test_batch_scheduler() {
    let ir = pipeline(&add(SpecInteger::new(16, 2, 2)).optimize_ir());
    assert_display_is!(
        ir.format().show_types(false),
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
            %24 = add_ct(%18, %18);
            %25 = add_ct(%21, %21);
            %26, %27, %28, %29, %30, %31, %32, %33 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = batch_arg<4, CtRegister>();
                %a5 = batch_arg<5, CtRegister>();
                %a6 = batch_arg<6, CtRegister>();
                %a7, %a8 = pbs_2<Lut2("ManyCarryMsg")>(%a0);
                %a9 = pbs<Lut1("ExtractPropGroup0")>(%a1);
                %a10 = pbs<Lut1("ExtractPropGroup1")>(%a2);
                %a11 = pbs<Lut1("ExtractPropGroup2")>(%a3);
                %a12 = pbs<Lut1("ExtractPropGroup0")>(%a4);
                %a13 = pbs<Lut1("ExtractPropGroup1")>(%a5);
                %a14 = pbs_f<Lut1("ExtractPropGroup2")>(%a6);
                batch_ret<0, CtRegister>(%a7);
                batch_ret<1, CtRegister>(%a8);
                batch_ret<2, CtRegister>(%a9);
                batch_ret<3, CtRegister>(%a10);
                batch_ret<4, CtRegister>(%a11);
                batch_ret<5, CtRegister>(%a12);
                batch_ret<6, CtRegister>(%a13);
                batch_ret<7, CtRegister>(%a14);
            }(%16, %17, %18, %19, %20, %21, %22);
            %34 = add_ct(%17, %27);
            dst_st<0.0_tdst>(%26);
            %35 = add_ct(%27, %28);
            %36 = add_ct(%31, %32);
            %37 = add_ct(%35, %29);
            %38 = add_ct(%24, %35);
            %39 = add_ct(%36, %33);
            %40 = add_ct(%37, %30);
            %41, %42, %43, %44 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = pbs<Lut1("MsgOnly")>(%a2);
                %a5 = pbs<Lut1("SolvePropGroupFinal1")>(%a0);
                %a6 = pbs<Lut1("HalfMsg")>(%a3);
                %a7 = pbs_f<Lut1("SolvePropGroupFinal2")>(%a1);
                batch_ret<0, CtRegister>(%a7);
                batch_ret<1, CtRegister>(%a5);
                batch_ret<2, CtRegister>(%a4);
                batch_ret<3, CtRegister>(%a6);
            }(%37, %40, %34, %38);
            dst_st<0.1_tdst>(%43);
            %45 = add_ct(%19, %42);
            dst_st<0.2_tdst>(%44);
            %46 = add_ct(%31, %41);
            %47 = add_ct(%36, %41);
            %48 = add_ct(%39, %41);
            %49 = add_ct(%20, %41);
            %50 = add_ct(%25, %46);
            %51, %52, %53, %54, %55 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = batch_arg<4, CtRegister>();
                %a5 = pbs<Lut1("MsgOnly")>(%a2);
                %a6 = pbs<Lut1("SolvePropGroupFinal1")>(%a0);
                %a7 = pbs<Lut1("SolvePropGroupFinal2")>(%a1);
                %a8 = pbs<Lut1("MsgOnly")>(%a3);
                %a9 = pbs_f<Lut1("HalfMsg")>(%a4);
                batch_ret<0, CtRegister>(%a6);
                batch_ret<1, CtRegister>(%a7);
                batch_ret<2, CtRegister>(%a5);
                batch_ret<3, CtRegister>(%a8);
                batch_ret<4, CtRegister>(%a9);
            }(%47, %48, %45, %49, %50);
            dst_st<0.3_tdst>(%53);
            %56 = add_ct(%22, %51);
            %57 = add_ct(%23, %52);
            dst_st<0.4_tdst>(%54);
            dst_st<0.5_tdst>(%55);
            %58, %59 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = pbs<Lut1("MsgOnly")>(%a0);
                %a3 = pbs_f<Lut1("MsgOnly")>(%a1);
                batch_ret<0, CtRegister>(%a2);
                batch_ret<1, CtRegister>(%a3);
            }(%56, %57);
            dst_st<0.6_tdst>(%58);
            dst_st<0.7_tdst>(%59);
        "#
    )
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
