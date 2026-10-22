use zhc::compat::Iop;
use zhc_builder::{SpecInteger, add, cmp_gt};
use zhc_ir::IR;
use zhc_langs::{ioplang::IopLang, vmlang::VmLang};
use zhc_pipeline::passes::{lower_iop_to_vm, optimize_iop};
use zhc_utils::assert_display_is;
use zhc_validation_macro::test_matrix;

use zhc_validation::equivalence_check::check_iop_vm_equivalence;

fn pipeline(ir: &IR<IopLang>) -> IR<VmLang> {
    lower_iop_to_vm(ir)
}

#[test]
fn smoke1() {
    let ir = pipeline(&optimize_iop(&add(SpecInteger::new(16, 2, 2)).ir()));
    assert_display_is!(
        ir.format(),
        r#"
            %0 = src_ld<0, 0>();
            %1 = src_ld<0, 1>();
            %2 = src_ld<0, 2>();
            %3 = src_ld<0, 3>();
            %4 = src_ld<0, 4>();
            %5 = src_ld<0, 5>();
            %6 = src_ld<0, 6>();
            %7 = src_ld<0, 7>();
            %8 = src_ld<1, 0>();
            %9 = src_ld<1, 1>();
            %10 = src_ld<1, 2>();
            %11 = src_ld<1, 3>();
            %12 = src_ld<1, 4>();
            %13 = src_ld<1, 5>();
            %14 = src_ld<1, 6>();
            %15 = src_ld<1, 7>();
            %16 = add_ct(%0, %8);
            %17 = add_ct(%1, %9);
            %18 = add_ct(%2, %10);
            %19 = add_ct(%3, %11);
            %20 = add_ct(%4, %12);
            %21 = add_ct(%5, %13);
            %22 = add_ct(%6, %14);
            %23 = add_ct(%7, %15);
            %24 = ks(%16);
            %25, %26 = pbs_2<Lut2("ManyCarryMsg")>(%24);
            %27 = ks(%17);
            %28 = pbs<Lut1("ExtractPropGroup0")>(%27);
            %29 = ks(%18);
            %30 = pbs<Lut1("ExtractPropGroup1")>(%29);
            %31 = ks(%19);
            %32 = pbs<Lut1("ExtractPropGroup2")>(%31);
            %33 = ks(%20);
            %34 = pbs<Lut1("ExtractPropGroup0")>(%33);
            %35 = ks(%21);
            %36 = pbs<Lut1("ExtractPropGroup1")>(%35);
            %37 = ks(%22);
            %38 = pbs<Lut1("ExtractPropGroup2")>(%37);
            %39 = add_ct(%26, %28);
            %40 = add_ct(%39, %30);
            %41 = add_ct(%40, %32);
            %42 = ks(%41);
            %43 = pbs<Lut1("SolvePropGroupFinal2")>(%42);
            %44 = add_ct(%34, %36);
            %45 = add_ct(%44, %38);
            %46 = add_ct(%34, %43);
            %47 = ks(%40);
            %48 = pbs<Lut1("SolvePropGroupFinal1")>(%47);
            %49 = add_ct(%44, %43);
            %50 = ks(%49);
            %51 = pbs<Lut1("SolvePropGroupFinal1")>(%50);
            %52 = add_ct(%45, %43);
            %53 = ks(%52);
            %54 = pbs<Lut1("SolvePropGroupFinal2")>(%53);
            %55 = add_ct(%17, %26);
            %56 = add_ct(%19, %48);
            %57 = add_ct(%20, %43);
            %58 = add_ct(%22, %51);
            %59 = add_ct(%23, %54);
            %60 = ks(%55);
            %61 = pbs<Lut1("MsgOnly")>(%60);
            %62 = add_ct(%18, %18);
            %63 = add_ct(%62, %39);
            %64 = ks(%63);
            %65 = pbs<Lut1("HalfMsg")>(%64);
            %66 = ks(%56);
            %67 = pbs<Lut1("MsgOnly")>(%66);
            %68 = ks(%57);
            %69 = pbs<Lut1("MsgOnly")>(%68);
            %70 = add_ct(%21, %21);
            %71 = add_ct(%70, %46);
            %72 = ks(%71);
            %73 = pbs<Lut1("HalfMsg")>(%72);
            %74 = ks(%58);
            %75 = pbs<Lut1("MsgOnly")>(%74);
            %76 = ks(%59);
            %77 = pbs<Lut1("MsgOnly")>(%76);
            dst_st<0, 0>(%25);
            dst_st<0, 1>(%61);
            dst_st<0, 2>(%65);
            dst_st<0, 3>(%67);
            dst_st<0, 4>(%69);
            dst_st<0, 5>(%73);
            dst_st<0, 6>(%75);
            dst_st<0, 7>(%77);
        "#
    );
}

#[test]
fn smoke2() {
    let ir = pipeline(&optimize_iop(&cmp_gt(SpecInteger::new(16, 2, 2)).ir()));
    assert_display_is!(
        ir.format(),
        r#"
            %0 = src_ld<0, 0>();
            %1 = src_ld<0, 1>();
            %2 = src_ld<0, 2>();
            %3 = src_ld<0, 3>();
            %4 = src_ld<0, 4>();
            %5 = src_ld<0, 5>();
            %6 = src_ld<0, 6>();
            %7 = src_ld<0, 7>();
            %8 = src_ld<1, 0>();
            %9 = src_ld<1, 1>();
            %10 = src_ld<1, 2>();
            %11 = src_ld<1, 3>();
            %12 = src_ld<1, 4>();
            %13 = src_ld<1, 5>();
            %14 = src_ld<1, 6>();
            %15 = src_ld<1, 7>();
            %16 = mac<4>(%1, %0);
            %17 = mac<4>(%3, %2);
            %18 = mac<4>(%5, %4);
            %19 = mac<4>(%7, %6);
            %20 = mac<4>(%9, %8);
            %21 = mac<4>(%11, %10);
            %22 = mac<4>(%13, %12);
            %23 = mac<4>(%15, %14);
            %24 = sub_ct(%16, %20);
            %25 = ks(%24);
            %26 = pbs<Lut1("CmpSign")>(%25);
            %27 = add_cst<1>(%26);
            %28 = sub_ct(%17, %21);
            %29 = ks(%28);
            %30 = pbs<Lut1("CmpSign")>(%29);
            %31 = add_cst<1>(%30);
            %32 = sub_ct(%18, %22);
            %33 = ks(%32);
            %34 = pbs<Lut1("CmpSign")>(%33);
            %35 = add_cst<1>(%34);
            %36 = sub_ct(%19, %23);
            %37 = ks(%36);
            %38 = pbs<Lut1("CmpSign")>(%37);
            %39 = add_cst<1>(%38);
            %40 = mac<4>(%31, %27);
            %41 = ks(%40);
            %42 = pbs<Lut1("CmpReduce")>(%41);
            %43 = mac<4>(%39, %35);
            %44 = ks(%43);
            %45 = pbs<Lut1("CmpReduce")>(%44);
            %46 = mac<4>(%45, %42);
            %47 = ks(%46);
            %48 = pbs<Lut1("CmpGtMrg")>(%47);
            dst_st<0, 0>(%48);
        "#
    );
}

#[test_matrix(iop = @all_iops, size = @main_precs)]
#[ignore]
fn validate_lower_iop_to_vm(iop: Iop, size: u16) {
    let b = iop.to_builder(SpecInteger::new(size, 2, 2));
    let spec = *b.spec();
    let iop_ir = optimize_iop(&b.ir());
    let vm_ir = pipeline(&iop_ir);
    check_iop_vm_equivalence(&iop_ir, &vm_ir, spec, &b.signature());
}
