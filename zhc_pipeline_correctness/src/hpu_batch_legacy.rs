use crate::equivalence_check::check_iop_hpu_equivalence;
use zhc_builder::{
    Builder, CiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, div, if_then_else,
    if_then_zero, mul,
};
use zhc_config::hpu::PhysicalConfig;
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
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
    let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).optimize_ir());
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
            %24, %25, %26, %27, %28, %29, %30, %31 = batch {
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
            %32 = add_ct(%17, %25);
            %33 = add_ct(%25, %26);
            %34 = add_ct(%29, %30);
            %35 = add_ct(%33, %27);
            %36 = add_ct(%34, %31);
            %37 = add_ct(%35, %28);
            %38, %39, %40, %41, %42 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = batch_arg<4, CtRegister>();
                %a5 = pbs<Lut1("MsgOnly")>(%a0);
                %a6 = pbs<Lut1("MsgOnly")>(%a4);
                %a7 = pbs<Lut1("SolvePropGroupFinal0")>(%a1);
                %a8 = pbs<Lut1("SolvePropGroupFinal1")>(%a2);
                %a9 = pbs_f<Lut1("SolvePropGroupFinal2")>(%a3);
                batch_ret<0, CtRegister>(%a9);
                batch_ret<1, CtRegister>(%a7);
                batch_ret<2, CtRegister>(%a8);
                batch_ret<3, CtRegister>(%a5);
                batch_ret<4, CtRegister>(%a6);
            }(%24, %33, %35, %37, %32);
            dst_st<0.0_tdst>(%41);
            dst_st<0.1_tdst>(%42);
            %43 = add_ct(%18, %39);
            %44 = add_ct(%19, %40);
            %45 = add_ct(%29, %38);
            %46 = add_ct(%34, %38);
            %47 = add_ct(%36, %38);
            %48 = add_ct(%20, %38);
            %49, %50, %51, %52, %53, %54 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = batch_arg<4, CtRegister>();
                %a5 = batch_arg<5, CtRegister>();
                %a6 = pbs<Lut1("MsgOnly")>(%a3);
                %a7 = pbs<Lut1("MsgOnly")>(%a4);
                %a8 = pbs<Lut1("SolvePropGroupFinal0")>(%a0);
                %a9 = pbs<Lut1("SolvePropGroupFinal1")>(%a1);
                %a10 = pbs<Lut1("SolvePropGroupFinal2")>(%a2);
                %a11 = pbs_f<Lut1("MsgOnly")>(%a5);
                batch_ret<0, CtRegister>(%a8);
                batch_ret<1, CtRegister>(%a9);
                batch_ret<2, CtRegister>(%a10);
                batch_ret<3, CtRegister>(%a6);
                batch_ret<4, CtRegister>(%a7);
                batch_ret<5, CtRegister>(%a11);
            }(%45, %46, %47, %43, %44, %48);
            dst_st<0.2_tdst>(%52);
            dst_st<0.3_tdst>(%53);
            %55 = add_ct(%21, %49);
            %56 = add_ct(%22, %50);
            %57 = add_ct(%23, %51);
            dst_st<0.4_tdst>(%54);
            %58, %59, %60 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = pbs<Lut1("MsgOnly")>(%a0);
                %a4 = pbs<Lut1("MsgOnly")>(%a1);
                %a5 = pbs_f<Lut1("MsgOnly")>(%a2);
                batch_ret<0, CtRegister>(%a3);
                batch_ret<1, CtRegister>(%a4);
                batch_ret<2, CtRegister>(%a5);
            }(%55, %56, %57);
            dst_st<0.5_tdst>(%58);
            dst_st<0.6_tdst>(%59);
            dst_st<0.7_tdst>(%60);
        "#
    )
}

#[test]
fn correctness() {
    let check = |b: Builder| {
        let spec = *b.spec();
        let iop_ir = b.optimize_ir();
        let hpu_ir = pipeline(&iop_ir);
        check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
    };
    for size in 2..=64 {
        let spec = CiphertextSpec::new(size, 2, 2);
        check(add(spec));
        check(bitwise_and(spec));
        check(bitwise_or(spec));
        check(bitwise_xor(spec));
        check(if_then_else(spec));
        check(if_then_zero(spec));
        check(mul(spec));
        check(div(spec));
    }
}
