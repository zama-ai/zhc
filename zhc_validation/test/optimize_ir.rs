use zhc::compat::Iop;
use zhc_builder::SpecInteger;
use zhc_pipeline::passes::optimize_iop;
use zhc_validation::equivalence_check::check_iop_equivalence;
use zhc_validation_macro::test_matrix;

#[test]
fn smoke() {
    let builder = Iop::Add.to_builder(SpecInteger::new(8, 2, 2));
    let spec = *builder.spec();
    let original = builder.ir().clone();
    let optimized = optimize_iop(&original);
    check_iop_equivalence(&original, &optimized, spec);
}

#[test_matrix(iop = @all_iops, size = @main_precs)]
#[ignore]
fn validate_optimize_ir(iop: Iop, size: u16) {
    let builder = iop.to_builder(SpecInteger::new(size, 2, 2));
    let spec = *builder.spec();
    let original = builder.ir().clone();
    let optimized = optimize_iop(&original);
    check_iop_equivalence(&original, &optimized, spec);
}
