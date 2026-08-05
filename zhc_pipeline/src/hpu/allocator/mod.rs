use zhc_config::hpu::HpuConfig;
use zhc_ir::{AnnIR, IR};
use zhc_langs::{doplang::DopLang, hpulang::HpuLang};

mod allocator;
mod batch_map;
mod heap;
mod live_range;
mod register_file;
mod register_state;
mod translator;
mod value_state;

/// Allocates physical registers to values in the scheduled IR.
///
/// Takes a scheduled intermediate representation `ir` containing HPU operations
/// and the hardware configuration `config` to produce a new IR in the device
/// operation language with physical register assignments for all values.
pub fn allocate_registers(ir: &IR<HpuLang>, config: &HpuConfig) -> IR<DopLang> {
    let allocator = allocator::Allocator::init(ir, config.regf_size);
    let allocation = allocator.allocate_registers();
    let annir = AnnIR::new(ir, allocation, ir.filled_valmap(()));
    translator::translate(&annir)
}

#[cfg(test)]
mod test {
    use zhc_builder::{
        Builder, CiphertextSpec, add, adds, bitwise_and, bitwise_or, bitwise_xor, cast, cmp_gt,
        div, flip, if_then_else, if_then_zero, mul, overflow_ssub, overflow_subs, ssub, subs, sum,
    };
    use zhc_config::hpu::{HpuConfig, PhysicalConfig};
    use zhc_ir::{IR, PrintWalker};
    use zhc_langs::{doplang::DopLang, ioplang::IopLang};
    use zhc_utils::assert_display_is;

    use crate::{
        SchedPolicy,
        hpu::{lowering::lower_iop_to_hpu, scheduler::legacy::schedule},
        test::check_iop_dop_equivalence,
    };

    use super::allocate_registers;

    fn pipeline(ir: &IR<IopLang>) -> IR<DopLang> {
        let ir = lower_iop_to_hpu(&ir).translation.output;
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(
            &ir,
            &config,
            SchedPolicy::AsSoonAsPossible,
            SchedPolicy::AsSoonAsPossible,
        );
        allocate_registers(&scheduled, &config)
    }

    #[test]
    fn test_allocate_add_ir() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = _START();
                %1 = LD<R(0), TC(0, 0)>(%0);
                %2 = LD<R(1), TC(1, 6)>(%1);
                %3 = LD<R(2), TC(0, 1)>(%2);
                %4 = LD<R(3), TC(1, 5)>(%3);
                %5 = LD<R(4), TC(0, 2)>(%4);
                %6 = LD<R(5), TC(1, 4)>(%5);
                %7 = LD<R(6), TC(0, 3)>(%6);
                %8 = LD<R(7), TC(1, 3)>(%7);
                %9 = LD<R(8), TC(0, 4)>(%8);
                %10 = ADD<R(6), R(6), R(7)>(%9);
                %11 = LD<R(7), TC(1, 2)>(%10);
                %12 = LD<R(9), TC(0, 5)>(%11);
                %13 = LD<R(10), TC(1, 1)>(%12);
                %14 = ADD<R(5), R(8), R(5)>(%13);
                %15 = LD<R(8), TC(0, 6)>(%14);
                %16 = LD<R(11), TC(1, 0)>(%15);
                %17 = LD<R(12), TC(0, 7)>(%16);
                %18 = LD<R(13), TC(1, 7)>(%17);
                %19 = ADD<R(3), R(9), R(3)>(%18);
                %20 = ADD<R(0), R(0), R(11)>(%19);
                %21 = ADD<R(4), R(4), R(7)>(%20);
                %22 = ADD<R(1), R(8), R(1)>(%21);
                %23 = ADD<R(2), R(2), R(10)>(%22);
                %24 = PBS2<R(8, 2), R(0), LUT(26)>(%23);
                %25 = PBS<R(7), R(2), LUT(47)>(%24);
                %26 = PBS<R(10), R(4), LUT(48)>(%25);
                %27 = PBS<R(11), R(6), LUT(49)>(%26);
                %28 = PBS<R(14), R(5), LUT(47)>(%27);
                %29 = PBS<R(15), R(3), LUT(48)>(%28);
                %30 = PBSF<R(16), R(1), LUT(49)>(%29);
                %31 = ADD<R(0), R(12), R(13)>(%30);
                %32 = ADD<R(7), R(9), R(7)>(%31);
                %33 = ADD<R(10), R(7), R(10)>(%32);
                %34 = ADD<R(2), R(2), R(9)>(%33);
                %35 = ADD<R(9), R(10), R(11)>(%34);
                %36 = PBS<R(11), R(8), LUT(1)>(%35);
                %37 = PBS<R(12), R(2), LUT(1)>(%36);
                %38 = PBS<R(13), R(7), LUT(44)>(%37);
                %39 = PBS<R(17), R(10), LUT(45)>(%38);
                %40 = PBSF<R(18), R(9), LUT(46)>(%39);
                %41 = ADD<R(2), R(14), R(15)>(%40);
                %42 = ADD<R(7), R(2), R(16)>(%41);
                %43 = ST<TC(0, 0), R(11)>(%42);
                %44 = ADD<R(4), R(4), R(13)>(%43);
                %45 = ST<TC(0, 1), R(12)>(%44);
                %46 = ADD<R(5), R(5), R(18)>(%45);
                %47 = ADD<R(6), R(6), R(17)>(%46);
                %48 = ADD<R(7), R(7), R(18)>(%47);
                %49 = ADD<R(8), R(14), R(18)>(%48);
                %50 = ADD<R(2), R(2), R(18)>(%49);
                %51 = PBS<R(9), R(4), LUT(1)>(%50);
                %52 = PBS<R(10), R(6), LUT(1)>(%51);
                %53 = PBS<R(11), R(8), LUT(44)>(%52);
                %54 = PBS<R(12), R(2), LUT(45)>(%53);
                %55 = PBS<R(13), R(7), LUT(46)>(%54);
                %56 = PBSF<R(14), R(5), LUT(1)>(%55);
                %57 = ST<TC(0, 2), R(9)>(%56);
                %58 = ADD<R(2), R(3), R(11)>(%57);
                %59 = ST<TC(0, 4), R(14)>(%58);
                %60 = ST<TC(0, 3), R(10)>(%59);
                %61 = ADD<R(0), R(0), R(13)>(%60);
                %62 = ADD<R(1), R(1), R(12)>(%61);
                %63 = PBS<R(3), R(2), LUT(1)>(%62);
                %64 = PBS<R(4), R(1), LUT(1)>(%63);
                %65 = PBSF<R(5), R(0), LUT(1)>(%64);
                %66 = ST<TC(0, 5), R(3)>(%65);
                %67 = ST<TC(0, 7), R(5)>(%66);
                %68 = ST<TC(0, 6), R(4)>(%67);
                _END(%68);
            "#
        );
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let ir = pipeline(&cmp_gt(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = _START();
                %1 = LD<R(0), TC(0, 0)>(%0);
                %2 = LD<R(1), TC(1, 7)>(%1);
                %3 = LD<R(2), TC(0, 1)>(%2);
                %4 = LD<R(3), TC(1, 6)>(%3);
                %5 = MAC<R(0), R(2), R(0), PT_I(4)>(%4);
                %6 = LD<R(2), TC(0, 2)>(%5);
                %7 = LD<R(4), TC(1, 5)>(%6);
                %8 = LD<R(5), TC(0, 3)>(%7);
                %9 = MAC<R(1), R(1), R(3), PT_I(4)>(%8);
                %10 = LD<R(3), TC(1, 4)>(%9);
                %11 = LD<R(6), TC(0, 4)>(%10);
                %12 = LD<R(7), TC(1, 3)>(%11);
                %13 = LD<R(8), TC(0, 5)>(%12);
                %14 = MAC<R(2), R(5), R(2), PT_I(4)>(%13);
                %15 = LD<R(5), TC(1, 2)>(%14);
                %16 = LD<R(9), TC(0, 6)>(%15);
                %17 = LD<R(10), TC(1, 1)>(%16);
                %18 = LD<R(11), TC(0, 7)>(%17);
                %19 = MAC<R(3), R(4), R(3), PT_I(4)>(%18);
                %20 = LD<R(4), TC(1, 0)>(%19);
                %21 = MAC<R(5), R(7), R(5), PT_I(4)>(%20);
                %22 = MAC<R(4), R(10), R(4), PT_I(4)>(%21);
                %23 = MAC<R(6), R(8), R(6), PT_I(4)>(%22);
                %24 = MAC<R(7), R(11), R(9), PT_I(4)>(%23);
                %25 = PBS<R(8), R(0), LUT(0)>(%24);
                %26 = PBS<R(9), R(2), LUT(0)>(%25);
                %27 = PBS<R(10), R(6), LUT(0)>(%26);
                %28 = PBS<R(11), R(7), LUT(0)>(%27);
                %29 = PBS<R(12), R(4), LUT(0)>(%28);
                %30 = PBS<R(13), R(5), LUT(0)>(%29);
                %31 = PBS<R(14), R(3), LUT(0)>(%30);
                %32 = PBSF<R(15), R(1), LUT(0)>(%31);
                %33 = SUB<R(0), R(8), R(12)>(%32);
                %34 = SUB<R(1), R(11), R(15)>(%33);
                %35 = SUB<R(2), R(9), R(13)>(%34);
                %36 = SUB<R(3), R(10), R(14)>(%35);
                %37 = PBS<R(4), R(0), LUT(40)>(%36);
                %38 = PBS<R(5), R(2), LUT(40)>(%37);
                %39 = PBS<R(6), R(3), LUT(40)>(%38);
                %40 = PBSF<R(7), R(1), LUT(40)>(%39);
                %41 = ADDS<R(0), R(4), PT_I(1)>(%40);
                %42 = ADDS<R(1), R(7), PT_I(1)>(%41);
                %43 = ADDS<R(2), R(5), PT_I(1)>(%42);
                %44 = ADDS<R(3), R(6), PT_I(1)>(%43);
                %45 = MAC<R(0), R(2), R(0), PT_I(4)>(%44);
                %46 = MAC<R(1), R(1), R(3), PT_I(4)>(%45);
                %47 = PBS<R(2), R(0), LUT(51)>(%46);
                %48 = PBSF<R(3), R(1), LUT(51)>(%47);
                %49 = MAC<R(0), R(3), R(2), PT_I(4)>(%48);
                %50 = PBSF<R(1), R(0), LUT(27)>(%49);
                %51 = ST<TC(0, 0), R(1)>(%50);
                _END(%51);
            "#
        );
    }

    #[test]
    fn allocator_correctness() {
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.optimize_ir();
            let dop_ir = pipeline(&iop_ir);
            check_iop_dop_equivalence(&iop_ir, &dop_ir, spec, config.regf_size, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(adds(spec));
            check(subs(spec));
            check(ssub(spec));
            check(overflow_subs(spec));
            check(overflow_ssub(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(flip(spec));
            check(sum(spec, 5));
            check(mul(spec));
            check(div(spec));
            check(cast(spec, 2));
            check(cast(spec, 128));
        }
    }
}
