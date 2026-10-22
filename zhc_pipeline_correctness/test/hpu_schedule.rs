use zhc::compat::Iop;
use zhc_builder::{SpecInteger, mul};
use zhc_config::hpu::PhysicalConfig;
use zhc_langs::ioplang::IopLang;
use zhc_pipeline_correctness_macro::test_matrix;
use zhc_utils::assert_display_is;

use zhc_pipeline::{
    SchedPolicy,
    passes::{hpu_schedule, lower_iop_to_hpu},
};

use zhc_config::hpu::HpuConfig;
use zhc_ir::IR;
use zhc_langs::hpulang::HpuLang;

fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
    let ir = lower_iop_to_hpu(&ir).output;
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
    let scheduled = hpu_schedule(&ir, &config, SchedPolicy::AsLateAsPossible);
    scheduled
}

#[test]
fn smoke() {
    let ir = pipeline(&zhc_pipeline::passes::optimize_iop(
        &mul(SpecInteger::new(8, 2, 2)).ir(),
    ));
    assert_display_is!(
        ir.format(),
        r#"
            %0 = src_ld<1.0_tsrc>();
            %1 = src_ld<0.0_tsrc>();
            %2 = src_ld<1.1_tsrc>();
            %3 = mac<4_imm>(%1, %0);
            %4 = src_ld<0.1_tsrc>();
            %5 = mac<4_imm>(%1, %2);
            %6 = mac<4_imm>(%4, %0);
            %7, %8, %9 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = pbs<Lut1("MultCarryMsgLsb")>(%a2);
                %a4 = pbs<Lut1("MultCarryMsgLsb")>(%a1);
                %a5 = pbs_f<Lut1("MultCarryMsgMsb")>(%a0);
                batch_ret<0, CtRegister>(%a5);
                batch_ret<1, CtRegister>(%a4);
                batch_ret<2, CtRegister>(%a3);
            }(%3, %5, %6);
            %10 = src_ld<1.2_tsrc>();
            %11 = src_ld<0.3_tsrc>();
            %12 = mac<4_imm>(%1, %10);
            %13 = src_ld<0.2_tsrc>();
            %14 = mac<4_imm>(%11, %0);
            %15 = mac<4_imm>(%13, %2);
            %16 = src_ld<1.3_tsrc>();
            %17 = mac<4_imm>(%13, %0);
            %18 = mac<4_imm>(%1, %16);
            %19 = mac<4_imm>(%4, %10);
            %20 = mac<4_imm>(%4, %2);
            %21 = add_ct(%9, %8);
            %22 = add_ct(%7, %21);
            %23, %24, %25, %26, %27, %28, %29, %30, %31, %32, %33 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = batch_arg<4, CtRegister>();
                %a5 = batch_arg<5, CtRegister>();
                %a6 = batch_arg<6, CtRegister>();
                %a7 = batch_arg<7, CtRegister>();
                %a8 = batch_arg<8, CtRegister>();
                %a9 = batch_arg<9, CtRegister>();
                %a10 = pbs<Lut1("MultCarryMsgLsb")>(%a4);
                %a11 = pbs<Lut1("MultCarryMsgLsb")>(%a1);
                %a12 = pbs<Lut1("MultCarryMsgLsb")>(%a5);
                %a13 = pbs<Lut1("MultCarryMsgLsb")>(%a2);
                %a14 = pbs<Lut1("MultCarryMsgLsb")>(%a6);
                %a15 = pbs<Lut1("MultCarryMsgLsb")>(%a7);
                %a16 = pbs<Lut1("MultCarryMsgMsb")>(%a0);
                %a17 = pbs<Lut1("MultCarryMsgLsb")>(%a8);
                %a18 = pbs<Lut1("MultCarryMsgMsb")>(%a3);
                %a19 = pbs<Lut1("MultCarryMsgMsb")>(%a1);
                %a20 = pbs_f<Lut1("CarryInMsg")>(%a9);
                batch_ret<0, CtRegister>(%a16);
                batch_ret<1, CtRegister>(%a11);
                batch_ret<2, CtRegister>(%a19);
                batch_ret<3, CtRegister>(%a13);
                batch_ret<4, CtRegister>(%a18);
                batch_ret<5, CtRegister>(%a10);
                batch_ret<6, CtRegister>(%a12);
                batch_ret<7, CtRegister>(%a14);
                batch_ret<8, CtRegister>(%a15);
                batch_ret<9, CtRegister>(%a17);
                batch_ret<10, CtRegister>(%a20);
            }(%5, %12, %18, %6, %20, %19, %17, %15, %14, %22);
            %34 = add_ct(%28, %24);
            %35 = add_ct(%29, %26);
            %36 = add_ct(%30, %34);
            %37 = add_ct(%31, %35);
            %38 = add_ct(%23, %36);
            %39 = add_ct(%32, %37);
            %40 = add_ct(%27, %38);
            %41 = add_ct(%25, %39);
            %42 = add_ct(%33, %40);
            %43, %44, %45, %46 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = pbs<Lut1("MsgOnly")>(%a3);
                %a5 = pbs<Lut1("MultCarryMsgMsb")>(%a0);
                %a6 = pbs<Lut1("MultCarryMsgMsb")>(%a1);
                %a7 = pbs_f<Lut1("CarryInMsg")>(%a2);
                batch_ret<0, CtRegister>(%a5);
                batch_ret<1, CtRegister>(%a6);
                batch_ret<2, CtRegister>(%a7);
                batch_ret<3, CtRegister>(%a4);
            }(%20, %17, %42, %41);
            %47 = add_ct(%43, %46);
            %48 = add_ct(%44, %47);
            %49 = add_ct(%45, %48);
            %50, %51, %52, %53 = batch {
                %a0 = batch_arg<0, CtRegister>();
                %a1 = batch_arg<1, CtRegister>();
                %a2 = batch_arg<2, CtRegister>();
                %a3 = batch_arg<3, CtRegister>();
                %a4 = pbs<Lut1("MultCarryMsgLsb")>(%a0);
                %a5 = pbs<Lut1("MsgOnly")>(%a1);
                %a6 = pbs<Lut1("MsgOnly")>(%a2);
                %a7 = pbs_f<Lut1("MsgOnly")>(%a3);
                batch_ret<0, CtRegister>(%a4);
                batch_ret<1, CtRegister>(%a5);
                batch_ret<2, CtRegister>(%a6);
                batch_ret<3, CtRegister>(%a7);
            }(%3, %22, %42, %49);
            dst_st<0.0_tdst>(%50);
            dst_st<0.1_tdst>(%51);
            dst_st<0.2_tdst>(%52);
            dst_st<0.3_tdst>(%53);
        "#
    )
}

#[test_matrix(iop = @all_iops, size = @main_precs)]
#[ignore]
fn correctness(iop: Iop, size: u16) {
    let b = iop.to_builder(SpecInteger::new(size, 2, 2));
    let spec = *b.spec();
    let iop_ir = zhc_pipeline::passes::optimize_iop(&b.ir());
    let hpu_ir = pipeline(&iop_ir);
    zhc_pipeline_correctness::equivalence_check::check_iop_hpu_equivalence(
        &iop_ir, &hpu_ir, spec, 100,
    );
}
