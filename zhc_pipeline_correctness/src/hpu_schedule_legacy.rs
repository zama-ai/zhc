use zhc_builder::{IntegerCiphertextSpec, count_0};
use zhc_config::hpu::PhysicalConfig;
use zhc_ir::IR;
use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
use zhc_utils::assert_display_is;

use crate::equivalence_check::check_iop_hpu_equivalence;
use zhc_config::hpu::HpuConfig;
use zhc_pipeline::{
    SchedPolicy,
    passes::{hpu_batch_legacy, hpu_schedule_batched_legacy, lower_iop_to_hpu},
};

fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
    let ir = lower_iop_to_hpu(ir).output;
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
    let batched = hpu_batch_legacy(&ir, &config, SchedPolicy::AsSoonAsPossible);
    hpu_schedule_batched_legacy(&batched, &config, SchedPolicy::AsSoonAsPossible)
}

#[test]
fn test_scheduler() {
    let ir = pipeline(&count_0(IntegerCiphertextSpec::new(16, 2, 2)).optimize_ir());
    assert_display_is!(
        ir.format(),
        r#"
            %0 = cst_ct<0_imm>();
            %1 = src_ld<0.0_tsrc>();
            %2 = src_ld<0.7_tsrc>();
            %3 = src_ld<0.1_tsrc>();
            %4 = src_ld<0.6_tsrc>();
            %5 = src_ld<0.2_tsrc>();
            %6 = src_ld<0.5_tsrc>();
            %7 = src_ld<0.3_tsrc>();
            %8 = src_ld<0.4_tsrc>();
            %9, %10, %11, %12, %13, %14, %15, %16, %17, %18, %19, %20, %21, %22, %23, %24 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = batch_arg<4, CtRegister>();
                %a5 = batch_arg<5, CtRegister>();
                %a6 = batch_arg<6, CtRegister>();
                %a7 = batch_arg<7, CtRegister>();
                %a8, %a9 = pbs_2<Lut2("ManyMsgSplit")>(%a0);
                %a10, %a11 = pbs_2<Lut2("ManyMsgSplit")>(%a1);
                %a12, %a13 = pbs_2<Lut2("ManyMsgSplit")>(%a2);
                %a14, %a15 = pbs_2<Lut2("ManyMsgSplit")>(%a3);
                %a16, %a17 = pbs_2<Lut2("ManyMsgSplit")>(%a4);
                %a18, %a19 = pbs_2<Lut2("ManyMsgSplit")>(%a5);
                %a20, %a21 = pbs_2<Lut2("ManyMsgSplit")>(%a6);
                %a22, %a23 = pbs_2f<Lut2("ManyMsgSplit")>(%a7);
                batch_ret<0, CtRegister>(%a8);
                batch_ret<1, CtRegister>(%a9);
                batch_ret<2, CtRegister>(%a10);
                batch_ret<3, CtRegister>(%a11);
                batch_ret<4, CtRegister>(%a12);
                batch_ret<5, CtRegister>(%a13);
                batch_ret<6, CtRegister>(%a14);
                batch_ret<7, CtRegister>(%a15);
                batch_ret<8, CtRegister>(%a16);
                batch_ret<9, CtRegister>(%a17);
                batch_ret<10, CtRegister>(%a18);
                batch_ret<11, CtRegister>(%a19);
                batch_ret<12, CtRegister>(%a20);
                batch_ret<13, CtRegister>(%a21);
                batch_ret<14, CtRegister>(%a22);
                batch_ret<15, CtRegister>(%a23);
            }(%1, %3, %5, %7, %8, %6, %4, %2);
            dst_st<0.7_tdst>(%0);
            dst_st<0.3_tdst>(%0);
            dst_st<0.6_tdst>(%0);
            dst_st<0.4_tdst>(%0);
            dst_st<0.5_tdst>(%0);
            %25 = add_ct(%9, %10);
            %26 = add_ct(%16, %17);
            %27 = add_ct(%25, %11);
            %28 = add_ct(%26, %18);
            %29 = add_ct(%27, %12);
            %30 = add_ct(%28, %19);
            %31 = add_ct(%29, %13);
            %32 = add_ct(%30, %20);
            %33 = add_ct(%31, %14);
            %34 = add_ct(%32, %21);
            %35 = add_ct(%33, %15);
            %36 = add_ct(%34, %22);
            %37 = add_ct(%23, %24);
            %38, %39, %40, %41, %42, %43 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3, %a4 = pbs_2<Lut2("ManyInv2CarryMsg")>(%a2);
                %a5, %a6 = pbs_2<Lut2("ManyInv7CarryMsg")>(%a0);
                %a7, %a8 = pbs_2f<Lut2("ManyInv7CarryMsg")>(%a1);
                batch_ret<0, CtRegister>(%a5);
                batch_ret<1, CtRegister>(%a6);
                batch_ret<2, CtRegister>(%a7);
                batch_ret<3, CtRegister>(%a8);
                batch_ret<4, CtRegister>(%a3);
                batch_ret<5, CtRegister>(%a4);
            }(%35, %36, %37);
            %44 = add_ct(%38, %40);
            %45 = add_ct(%39, %41);
            %46 = add_ct(%44, %42);
            %47 = add_ct(%45, %43);
            %48, %49, %50, %51 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = pbs<Lut1("MsgOnly")>(%a0);
                %a3 = pbs<Lut1("CarryInMsg")>(%a0);
                %a4, %a5 = pbs_2f<Lut2("ManyCarryMsg")>(%a1);
                batch_ret<0, CtRegister>(%a2);
                batch_ret<1, CtRegister>(%a3);
                batch_ret<2, CtRegister>(%a4);
                batch_ret<3, CtRegister>(%a5);
            }(%46, %47);
            dst_st<0.0_tdst>(%48);
            %52 = add_ct(%49, %50);
            %53, %54 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1, %a2 = pbs_2f<Lut2("ManyCarryMsg")>(%a0);
                batch_ret<0, CtRegister>(%a1);
                batch_ret<1, CtRegister>(%a2);
            }(%52);
            dst_st<0.1_tdst>(%53);
            %55 = add_ct(%54, %51);
            %56, %57 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1, %a2 = pbs_2f<Lut2("ManyCarryMsg")>(%a0);
                batch_ret<0, CtRegister>(%a1);
                batch_ret<1, CtRegister>(%a2);
            }(%55);
            dst_st<0.2_tdst>(%56);
        "#
    )
}

#[test]
fn correctness() {
    use zhc_builder::*;
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
        check(div(spec));
    }
}
