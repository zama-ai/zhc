use zhc_ir::{AnnIR, IR};
use zhc_langs::{doplang::DopLang, hpulang::HpuLang};
use zhc_sim::hpu::HpuConfig;

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
        Builder, CiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, cmp_gt, div,
        if_then_else, if_then_zero, mul_lsb,
    };
    use zhc_ir::{IR, PrintWalker};
    use zhc_langs::{doplang::DopLang, ioplang::IopLang};
    use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
    use zhc_utils::assert_display_is;

    use crate::{batcher::batch, test::check_iop_dop_equivalence, translation::lower_iop_to_hpu};

    use super::allocate_registers;

    fn pipeline(ir: &IR<IopLang>) -> IR<DopLang> {
        let ir = lower_iop_to_hpu(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let batched = batch(&ir, &config);
        let allocated = allocate_registers(&batched, &config);
        allocated
    }

    #[test]
    fn test_allocate_add_ir() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 = _INIT();
                %1 = LD<R(0), TC(0, 0)>(%0);
                %2 = LD<R(1), TC(1, 0)>(%1);
                %3 = ADD<R(0), R(0), R(1)>(%2);
                %4 = LD<R(1), TC(0, 1)>(%3);
                %5 = LD<R(2), TC(1, 1)>(%4);
                %6 = ADD<R(1), R(1), R(2)>(%5);
                %7 = LD<R(2), TC(0, 2)>(%6);
                %8 = LD<R(3), TC(1, 2)>(%7);
                %9 = ADD<R(2), R(2), R(3)>(%8);
                %10 = LD<R(3), TC(0, 3)>(%9);
                %11 = LD<R(4), TC(1, 3)>(%10);
                %12 = ADD<R(3), R(3), R(4)>(%11);
                %13 = LD<R(4), TC(0, 4)>(%12);
                %14 = LD<R(5), TC(1, 4)>(%13);
                %15 = ADD<R(4), R(4), R(5)>(%14);
                %16 = LD<R(5), TC(0, 5)>(%15);
                %17 = LD<R(6), TC(1, 5)>(%16);
                %18 = ADD<R(5), R(5), R(6)>(%17);
                %19 = LD<R(6), TC(0, 6)>(%18);
                %20 = LD<R(7), TC(1, 6)>(%19);
                %21 = ADD<R(6), R(6), R(7)>(%20);
                %22 = PBS2<R(8, 2), R(0), LUT(26)>(%21);
                %23 = PBS<R(7), R(1), LUT(47)>(%22);
                %24 = PBS<R(10), R(2), LUT(48)>(%23);
                %25 = PBS<R(11), R(3), LUT(49)>(%24);
                %26 = PBS<R(12), R(4), LUT(47)>(%25);
                %27 = PBS<R(13), R(5), LUT(48)>(%26);
                %28 = PBSF<R(14), R(6), LUT(49)>(%27);
                %29 = ADD<R(0), R(9), R(7)>(%28);
                %30 = ADD<R(7), R(0), R(10)>(%29);
                %31 = ADD<R(10), R(7), R(11)>(%30);
                %32 = ADD<R(1), R(1), R(9)>(%31);
                %33 = PBS<R(9), R(10), LUT(46)>(%32);
                %34 = PBS<R(11), R(7), LUT(45)>(%33);
                %35 = PBS<R(15), R(0), LUT(44)>(%34);
                %36 = PBS<R(16), R(1), LUT(1)>(%35);
                %37 = PBSF<R(17), R(8), LUT(1)>(%36);
                %38 = ST<TC(0, 0), R(17)>(%37);
                %39 = ST<TC(0, 1), R(16)>(%38);
                %40 = ADD<R(0), R(12), R(9)>(%39);
                %41 = ADD<R(1), R(12), R(13)>(%40);
                %42 = ADD<R(7), R(1), R(9)>(%41);
                %43 = ADD<R(1), R(1), R(14)>(%42);
                %44 = ADD<R(1), R(1), R(9)>(%43);
                %45 = ADD<R(2), R(2), R(15)>(%44);
                %46 = ADD<R(3), R(3), R(11)>(%45);
                %47 = ADD<R(4), R(4), R(9)>(%46);
                %48 = PBS<R(8), R(0), LUT(44)>(%47);
                %49 = PBS<R(9), R(7), LUT(45)>(%48);
                %50 = PBS<R(10), R(1), LUT(46)>(%49);
                %51 = PBS<R(11), R(4), LUT(1)>(%50);
                %52 = PBS<R(12), R(3), LUT(1)>(%51);
                %53 = PBSF<R(13), R(2), LUT(1)>(%52);
                %54 = ST<TC(0, 2), R(13)>(%53);
                %55 = ST<TC(0, 3), R(12)>(%54);
                %56 = ST<TC(0, 4), R(11)>(%55);
                %57 = ADD<R(0), R(5), R(8)>(%56);
                %58 = ADD<R(1), R(6), R(9)>(%57);
                %59 = LD<R(2), TC(0, 7)>(%58);
                %60 = LD<R(3), TC(1, 7)>(%59);
                %61 = ADD<R(2), R(2), R(3)>(%60);
                %62 = ADD<R(2), R(2), R(10)>(%61);
                %63 = PBS<R(3), R(0), LUT(1)>(%62);
                %64 = PBS<R(4), R(1), LUT(1)>(%63);
                %65 = PBSF<R(5), R(2), LUT(1)>(%64);
                %66 = ST<TC(0, 5), R(3)>(%65);
                %67 = ST<TC(0, 6), R(4)>(%66);
                %68 = ST<TC(0, 7), R(5)>(%67);
            "#
        );
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let ir = pipeline(&cmp_gt(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 = _INIT();
                %1 = LD<R(0), TC(0, 1)>(%0);
                %2 = LD<R(1), TC(0, 0)>(%1);
                %3 = MAC<R(0), R(0), R(1), PT_I(4)>(%2);
                %4 = LD<R(1), TC(0, 3)>(%3);
                %5 = LD<R(2), TC(0, 2)>(%4);
                %6 = MAC<R(1), R(1), R(2), PT_I(4)>(%5);
                %7 = LD<R(2), TC(0, 5)>(%6);
                %8 = LD<R(3), TC(0, 4)>(%7);
                %9 = MAC<R(2), R(2), R(3), PT_I(4)>(%8);
                %10 = LD<R(3), TC(0, 7)>(%9);
                %11 = LD<R(4), TC(0, 6)>(%10);
                %12 = MAC<R(3), R(3), R(4), PT_I(4)>(%11);
                %13 = LD<R(4), TC(1, 1)>(%12);
                %14 = LD<R(5), TC(1, 0)>(%13);
                %15 = MAC<R(4), R(4), R(5), PT_I(4)>(%14);
                %16 = LD<R(5), TC(1, 3)>(%15);
                %17 = LD<R(6), TC(1, 2)>(%16);
                %18 = MAC<R(5), R(5), R(6), PT_I(4)>(%17);
                %19 = LD<R(6), TC(1, 5)>(%18);
                %20 = LD<R(7), TC(1, 4)>(%19);
                %21 = MAC<R(6), R(6), R(7), PT_I(4)>(%20);
                %22 = LD<R(7), TC(1, 7)>(%21);
                %23 = LD<R(8), TC(1, 6)>(%22);
                %24 = MAC<R(7), R(7), R(8), PT_I(4)>(%23);
                %25 = PBS<R(8), R(0), LUT(0)>(%24);
                %26 = PBS<R(9), R(1), LUT(0)>(%25);
                %27 = PBS<R(10), R(2), LUT(0)>(%26);
                %28 = PBS<R(11), R(3), LUT(0)>(%27);
                %29 = PBS<R(12), R(4), LUT(0)>(%28);
                %30 = PBS<R(13), R(5), LUT(0)>(%29);
                %31 = PBS<R(14), R(6), LUT(0)>(%30);
                %32 = PBSF<R(15), R(7), LUT(0)>(%31);
                %33 = SUB<R(0), R(8), R(12)>(%32);
                %34 = SUB<R(1), R(9), R(13)>(%33);
                %35 = SUB<R(2), R(10), R(14)>(%34);
                %36 = SUB<R(3), R(11), R(15)>(%35);
                %37 = PBS<R(4), R(0), LUT(10)>(%36);
                %38 = PBS<R(5), R(1), LUT(10)>(%37);
                %39 = PBS<R(6), R(2), LUT(10)>(%38);
                %40 = PBSF<R(7), R(3), LUT(10)>(%39);
                %41 = ADDS<R(0), R(5), PT_I(1)>(%40);
                %42 = ADDS<R(1), R(4), PT_I(1)>(%41);
                %43 = MAC<R(0), R(0), R(1), PT_I(4)>(%42);
                %44 = ADDS<R(1), R(7), PT_I(1)>(%43);
                %45 = ADDS<R(2), R(6), PT_I(1)>(%44);
                %46 = MAC<R(1), R(1), R(2), PT_I(4)>(%45);
                %47 = PBS<R(2), R(0), LUT(11)>(%46);
                %48 = PBSF<R(3), R(1), LUT(11)>(%47);
                %49 = MAC<R(0), R(3), R(2), PT_I(4)>(%48);
                %50 = PBSF<R(1), R(0), LUT(27)>(%49);
                %51 = ST<TC(0, 0), R(1)>(%50);
            "#
        );
    }

    #[test]
    fn allocator_correctness() {
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.into_ir();
            let dop_ir = pipeline(&iop_ir);
            check_iop_dop_equivalence(&iop_ir, &dop_ir, spec, config.regf_size, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(mul_lsb(spec));
            check(div(spec));
        }
    }
}
