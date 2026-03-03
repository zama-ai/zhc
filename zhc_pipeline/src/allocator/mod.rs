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
    use zhc_builder::{CiphertextSpec, add, cmp_gt};
    use zhc_ir::{IR, PrintWalker};
    use zhc_langs::{doplang::DopLang, ioplang::IopLang};
    use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
    use zhc_utils::assert_display_is;

    use crate::{batch_scheduler::batch_schedule, translation::lower_iop_to_hpu};

    use super::allocate_registers;

    fn pipeline(ir: &IR<IopLang>) -> IR<DopLang> {
        let ir = lower_iop_to_hpu(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let batched = batch_schedule(&ir, &config);
        let allocated = allocate_registers(&batched, &config);
        allocated
    }

    #[test]
    fn test_allocate_add_ir() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            ir.format(),
            r#"
                %0 : Ctx = _INIT();
                %1 : Ctx = LD<R(0), TC(0, 0)>(%0 : Ctx);
                %2 : Ctx = LD<R(1), TC(1, 0)>(%1 : Ctx);
                %3 : Ctx = ADD<R(0), R(0), R(1)>(%2 : Ctx);
                %4 : Ctx = LD<R(1), TC(0, 1)>(%3 : Ctx);
                %5 : Ctx = LD<R(2), TC(1, 1)>(%4 : Ctx);
                %6 : Ctx = ADD<R(1), R(1), R(2)>(%5 : Ctx);
                %7 : Ctx = LD<R(2), TC(0, 2)>(%6 : Ctx);
                %8 : Ctx = LD<R(3), TC(1, 2)>(%7 : Ctx);
                %9 : Ctx = ADD<R(2), R(2), R(3)>(%8 : Ctx);
                %10 : Ctx = LD<R(3), TC(0, 3)>(%9 : Ctx);
                %11 : Ctx = LD<R(4), TC(1, 3)>(%10 : Ctx);
                %12 : Ctx = ADD<R(3), R(3), R(4)>(%11 : Ctx);
                %13 : Ctx = LD<R(4), TC(0, 4)>(%12 : Ctx);
                %14 : Ctx = LD<R(5), TC(1, 4)>(%13 : Ctx);
                %15 : Ctx = ADD<R(4), R(4), R(5)>(%14 : Ctx);
                %16 : Ctx = LD<R(5), TC(0, 5)>(%15 : Ctx);
                %17 : Ctx = LD<R(6), TC(1, 5)>(%16 : Ctx);
                %18 : Ctx = ADD<R(5), R(5), R(6)>(%17 : Ctx);
                %19 : Ctx = LD<R(6), TC(0, 6)>(%18 : Ctx);
                %20 : Ctx = LD<R(7), TC(1, 6)>(%19 : Ctx);
                %21 : Ctx = ADD<R(6), R(6), R(7)>(%20 : Ctx);
                %22 : Ctx = PBS<R(7), R(6), LUT(49)>(%21 : Ctx);
                %23 : Ctx = PBS<R(8), R(5), LUT(48)>(%22 : Ctx);
                %24 : Ctx = PBS<R(9), R(4), LUT(47)>(%23 : Ctx);
                %25 : Ctx = PBS<R(10), R(3), LUT(49)>(%24 : Ctx);
                %26 : Ctx = PBS<R(11), R(2), LUT(48)>(%25 : Ctx);
                %27 : Ctx = PBS<R(12), R(1), LUT(47)>(%26 : Ctx);
                %28 : Ctx = PBS2F<R(14, 2), R(0), LUT(26)>(%27 : Ctx);
                %29 : Ctx = ADD<R(0), R(15), R(12)>(%28 : Ctx);
                %30 : Ctx = ADD<R(11), R(0), R(11)>(%29 : Ctx);
                %31 : Ctx = ADD<R(10), R(11), R(10)>(%30 : Ctx);
                %32 : Ctx = ADD<R(1), R(1), R(15)>(%31 : Ctx);
                %33 : Ctx = PBS<R(12), R(10), LUT(46)>(%32 : Ctx);
                %34 : Ctx = PBS<R(13), R(11), LUT(45)>(%33 : Ctx);
                %35 : Ctx = PBS<R(15), R(1), LUT(1)>(%34 : Ctx);
                %36 : Ctx = PBS<R(16), R(0), LUT(44)>(%35 : Ctx);
                %37 : Ctx = PBSF<R(17), R(14), LUT(1)>(%36 : Ctx);
                %38 : Ctx = ST<TC(0, 0), R(17)>(%37 : Ctx);
                %39 : Ctx = ST<TC(0, 1), R(15)>(%38 : Ctx);
                %40 : Ctx = ADD<R(0), R(9), R(12)>(%39 : Ctx);
                %41 : Ctx = ADD<R(1), R(9), R(8)>(%40 : Ctx);
                %42 : Ctx = ADD<R(8), R(1), R(12)>(%41 : Ctx);
                %43 : Ctx = ADD<R(1), R(1), R(7)>(%42 : Ctx);
                %44 : Ctx = ADD<R(1), R(1), R(12)>(%43 : Ctx);
                %45 : Ctx = ADD<R(2), R(2), R(16)>(%44 : Ctx);
                %46 : Ctx = ADD<R(3), R(3), R(13)>(%45 : Ctx);
                %47 : Ctx = ADD<R(4), R(4), R(12)>(%46 : Ctx);
                %48 : Ctx = PBS<R(7), R(4), LUT(1)>(%47 : Ctx);
                %49 : Ctx = PBS<R(9), R(1), LUT(46)>(%48 : Ctx);
                %50 : Ctx = PBS<R(10), R(8), LUT(45)>(%49 : Ctx);
                %51 : Ctx = PBS<R(11), R(0), LUT(44)>(%50 : Ctx);
                %52 : Ctx = PBS<R(12), R(3), LUT(1)>(%51 : Ctx);
                %53 : Ctx = PBSF<R(13), R(2), LUT(1)>(%52 : Ctx);
                %54 : Ctx = ST<TC(0, 2), R(13)>(%53 : Ctx);
                %55 : Ctx = ST<TC(0, 3), R(12)>(%54 : Ctx);
                %56 : Ctx = ST<TC(0, 4), R(7)>(%55 : Ctx);
                %57 : Ctx = ADD<R(0), R(5), R(11)>(%56 : Ctx);
                %58 : Ctx = ADD<R(1), R(6), R(10)>(%57 : Ctx);
                %59 : Ctx = LD<R(2), TC(0, 7)>(%58 : Ctx);
                %60 : Ctx = LD<R(3), TC(1, 7)>(%59 : Ctx);
                %61 : Ctx = ADD<R(2), R(2), R(3)>(%60 : Ctx);
                %62 : Ctx = ADD<R(2), R(2), R(9)>(%61 : Ctx);
                %63 : Ctx = PBS<R(3), R(2), LUT(1)>(%62 : Ctx);
                %64 : Ctx = PBS<R(4), R(1), LUT(1)>(%63 : Ctx);
                %65 : Ctx = PBSF<R(5), R(0), LUT(1)>(%64 : Ctx);
                %66 : Ctx = ST<TC(0, 5), R(5)>(%65 : Ctx);
                %67 : Ctx = ST<TC(0, 6), R(4)>(%66 : Ctx);
                %68 : Ctx = ST<TC(0, 7), R(3)>(%67 : Ctx);
            "#
        );
    }

    #[test]
    fn test_allocate_cmp_ir() {
        let ir = pipeline(&cmp_gt(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 : Ctx = _INIT();
                %1 : Ctx = LD<R(0), TC(0, 1)>(%0 : Ctx);
                %2 : Ctx = LD<R(1), TC(0, 0)>(%1 : Ctx);
                %3 : Ctx = MAC<R(0), R(0), R(1), PT_I(4)>(%2 : Ctx);
                %4 : Ctx = LD<R(1), TC(0, 3)>(%3 : Ctx);
                %5 : Ctx = LD<R(2), TC(0, 2)>(%4 : Ctx);
                %6 : Ctx = MAC<R(1), R(1), R(2), PT_I(4)>(%5 : Ctx);
                %7 : Ctx = LD<R(2), TC(0, 5)>(%6 : Ctx);
                %8 : Ctx = LD<R(3), TC(0, 4)>(%7 : Ctx);
                %9 : Ctx = MAC<R(2), R(2), R(3), PT_I(4)>(%8 : Ctx);
                %10 : Ctx = LD<R(3), TC(0, 7)>(%9 : Ctx);
                %11 : Ctx = LD<R(4), TC(0, 6)>(%10 : Ctx);
                %12 : Ctx = MAC<R(3), R(3), R(4), PT_I(4)>(%11 : Ctx);
                %13 : Ctx = LD<R(4), TC(1, 1)>(%12 : Ctx);
                %14 : Ctx = LD<R(5), TC(1, 0)>(%13 : Ctx);
                %15 : Ctx = MAC<R(4), R(4), R(5), PT_I(4)>(%14 : Ctx);
                %16 : Ctx = LD<R(5), TC(1, 3)>(%15 : Ctx);
                %17 : Ctx = LD<R(6), TC(1, 2)>(%16 : Ctx);
                %18 : Ctx = MAC<R(5), R(5), R(6), PT_I(4)>(%17 : Ctx);
                %19 : Ctx = LD<R(6), TC(1, 5)>(%18 : Ctx);
                %20 : Ctx = LD<R(7), TC(1, 4)>(%19 : Ctx);
                %21 : Ctx = MAC<R(6), R(6), R(7), PT_I(4)>(%20 : Ctx);
                %22 : Ctx = LD<R(7), TC(1, 7)>(%21 : Ctx);
                %23 : Ctx = LD<R(8), TC(1, 6)>(%22 : Ctx);
                %24 : Ctx = MAC<R(7), R(7), R(8), PT_I(4)>(%23 : Ctx);
                %25 : Ctx = PBS<R(8), R(7), LUT(0)>(%24 : Ctx);
                %26 : Ctx = PBS<R(9), R(6), LUT(0)>(%25 : Ctx);
                %27 : Ctx = PBS<R(10), R(5), LUT(0)>(%26 : Ctx);
                %28 : Ctx = PBS<R(11), R(4), LUT(0)>(%27 : Ctx);
                %29 : Ctx = PBS<R(12), R(3), LUT(0)>(%28 : Ctx);
                %30 : Ctx = PBS<R(13), R(2), LUT(0)>(%29 : Ctx);
                %31 : Ctx = PBS<R(14), R(1), LUT(0)>(%30 : Ctx);
                %32 : Ctx = PBSF<R(15), R(0), LUT(0)>(%31 : Ctx);
                %33 : Ctx = SUB<R(0), R(15), R(11)>(%32 : Ctx);
                %34 : Ctx = SUB<R(1), R(14), R(10)>(%33 : Ctx);
                %35 : Ctx = SUB<R(2), R(13), R(9)>(%34 : Ctx);
                %36 : Ctx = SUB<R(3), R(12), R(8)>(%35 : Ctx);
                %37 : Ctx = PBS<R(4), R(3), LUT(10)>(%36 : Ctx);
                %38 : Ctx = PBS<R(5), R(2), LUT(10)>(%37 : Ctx);
                %39 : Ctx = PBS<R(6), R(1), LUT(10)>(%38 : Ctx);
                %40 : Ctx = PBSF<R(7), R(0), LUT(10)>(%39 : Ctx);
                %41 : Ctx = ADDS<R(0), R(6), PT_I(1)>(%40 : Ctx);
                %42 : Ctx = ADDS<R(1), R(7), PT_I(1)>(%41 : Ctx);
                %43 : Ctx = MAC<R(0), R(0), R(1), PT_I(4)>(%42 : Ctx);
                %44 : Ctx = ADDS<R(1), R(4), PT_I(1)>(%43 : Ctx);
                %45 : Ctx = ADDS<R(2), R(5), PT_I(1)>(%44 : Ctx);
                %46 : Ctx = MAC<R(1), R(1), R(2), PT_I(4)>(%45 : Ctx);
                %47 : Ctx = PBS<R(2), R(1), LUT(11)>(%46 : Ctx);
                %48 : Ctx = PBSF<R(3), R(0), LUT(11)>(%47 : Ctx);
                %49 : Ctx = MAC<R(0), R(2), R(3), PT_I(4)>(%48 : Ctx);
                %50 : Ctx = PBSF<R(1), R(0), LUT(27)>(%49 : Ctx);
                %51 : Ctx = ST<TC(0, 0), R(1)>(%50 : Ctx);
            "#
        );
    }
}
