use zhc_ir::IR;
use zhc_langs::hpulang::HpuLang;
use zhc_sim::{MHz, Simulator, hpu::HpuConfig};

mod affinity;
mod analyze;
mod batcher;
mod sim;

pub use affinity::*;
pub use analyze::*;
pub use batcher::*;
pub use sim::*;

use crate::scheduler::SchedPolicy;

#[allow(unused)]
pub fn schedule<'a>(ir: &'a IR<HpuLang>, config: &HpuConfig, policy: SchedPolicy) -> IR<HpuLang> {
    let ann_ir = analyze(ir);
    let mut sim = Simulator::from_simulatable(
        MHz(400),
        LightHpu::new(&ann_ir, config, policy),
        zhc_sim::TracingLevel::None,
    );
    sim.play();
    let schedule = sim.into_simulatable().schedule;
    let oup = batch(ir, schedule.into());
    oup
}

#[cfg(test)]
mod test {
    use crate::translation::lower_iop_to_hpu;
    use zhc_builder::{CiphertextSpec, mul};
    use zhc_langs::ioplang::IopLang;
    use zhc_sim::hpu::PhysicalConfig;
    use zhc_utils::assert_display_is;

    use super::*;

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = lower_iop_to_hpu(&ir).output;
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(&ir, &config, SchedPolicy::AsLateAsPossible);
        scheduled
    }

    #[test]
    fn test_scheduler() {
        let ir = pipeline(&mul(CiphertextSpec::new(8, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = src_ld<1.0_tsrc>();
                %1 = src_ld<0.2_tsrc>();
                %2 = src_ld<1.1_tsrc>();
                %3 = src_ld<0.1_tsrc>();
                %4 = mac<4_imm>(%1, %0);
                %5 = mac<4_imm>(%3, %2);
                %6 = src_ld<0.0_tsrc>();
                %7 = mac<4_imm>(%3, %0);
                %8 = mac<4_imm>(%6, %0);
                %9 = src_ld<1.2_tsrc>();
                %10 = mac<4_imm>(%6, %2);
                %11 = mac<4_imm>(%6, %9);
                %12, %13, %14, %15, %16, %17, %18, %19 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = pbs<Lut@5>(%a2);
                    %a7 = pbs<Lut@6>(%a1);
                    %a8 = pbs<Lut@5>(%a1);
                    %a9 = pbs<Lut@6>(%a0);
                    %a10 = pbs<Lut@6>(%a3);
                    %a11 = pbs<Lut@5>(%a3);
                    %a12 = pbs<Lut@5>(%a4);
                    %a13 = pbs_f<Lut@5>(%a5);
                    batch_ret<0, CtRegister>(%a9);
                    batch_ret<1, CtRegister>(%a8);
                    batch_ret<2, CtRegister>(%a7);
                    batch_ret<3, CtRegister>(%a6);
                    batch_ret<4, CtRegister>(%a11);
                    batch_ret<5, CtRegister>(%a10);
                    batch_ret<6, CtRegister>(%a12);
                    batch_ret<7, CtRegister>(%a13);
                }(%8, %10, %11, %7, %5, %4);
                %20 = src_ld<1.3_tsrc>();
                %21 = mac<4_imm>(%3, %9);
                %22 = mac<4_imm>(%6, %20);
                %23 = add_ct(%15, %14);
                %24 = add_ct(%13, %12);
                %25 = add_ct(%17, %23);
                %26 = add_ct(%16, %24);
                %27 = add_ct(%18, %25);
                %28 = add_ct(%19, %27);
                %29, %30, %31, %32, %33, %34, %35 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = batch_arg<6, CtRegister>();
                    %a7 = pbs<Lut@5>(%a1);
                    %a8 = pbs<Lut@6>(%a0);
                    %a9 = pbs<Lut@6>(%a2);
                    %a10 = pbs<Lut@5>(%a3);
                    %a11 = pbs<Lut@6>(%a4);
                    %a12 = pbs<Lut@1>(%a6);
                    %a13 = pbs_f<Lut@3>(%a5);
                    batch_ret<0, CtRegister>(%a8);
                    batch_ret<1, CtRegister>(%a7);
                    batch_ret<2, CtRegister>(%a9);
                    batch_ret<3, CtRegister>(%a10);
                    batch_ret<4, CtRegister>(%a11);
                    batch_ret<5, CtRegister>(%a13);
                    batch_ret<6, CtRegister>(%a12);
                }(%11, %22, %5, %21, %4, %26, %28);
                %36 = src_ld<0.3_tsrc>();
                %37 = mac<4_imm>(%36, %0);
                %38 = mac<4_imm>(%1, %2);
                %39 = add_ct(%30, %29);
                %40 = add_ct(%31, %39);
                %41 = add_ct(%32, %40);
                %42 = add_ct(%33, %41);
                %43 = add_ct(%34, %35);
                %44, %45, %46, %47, %48 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = pbs<Lut@1>(%a4);
                    %a6 = pbs<Lut@5>(%a0);
                    %a7 = pbs<Lut@5>(%a1);
                    %a8 = pbs<Lut@3>(%a2);
                    %a9 = pbs_f<Lut@3>(%a3);
                    batch_ret<0, CtRegister>(%a6);
                    batch_ret<1, CtRegister>(%a7);
                    batch_ret<2, CtRegister>(%a8);
                    batch_ret<3, CtRegister>(%a9);
                    batch_ret<4, CtRegister>(%a5);
                }(%38, %37, %28, %43, %42);
                %49 = add_ct(%44, %48);
                %50 = add_ct(%45, %49);
                %51 = add_ct(%46, %50);
                %52 = add_ct(%47, %51);
                %53, %54, %55, %56 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = pbs<Lut@5>(%a0);
                    %a5 = pbs<Lut@1>(%a1);
                    %a6 = pbs<Lut@1>(%a2);
                    %a7 = pbs_f<Lut@1>(%a3);
                    batch_ret<0, CtRegister>(%a4);
                    batch_ret<1, CtRegister>(%a5);
                    batch_ret<2, CtRegister>(%a6);
                    batch_ret<3, CtRegister>(%a7);
                }(%8, %26, %43, %52);
                dst_st<0.0_tdst>(%53);
                dst_st<0.1_tdst>(%54);
                dst_st<0.2_tdst>(%55);
                dst_st<0.3_tdst>(%56);
            "#
        )
    }
}
