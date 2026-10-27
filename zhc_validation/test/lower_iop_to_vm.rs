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
            %46 = ks(%39);
            %47 = pbs<Lut1("SolvePropGroupFinal0")>(%46);
            %48 = ks(%40);
            %49 = pbs<Lut1("SolvePropGroupFinal1")>(%48);
            %50 = add_ct(%34, %43);
            %51 = ks(%50);
            %52 = pbs<Lut1("SolvePropGroupFinal0")>(%51);
            %53 = add_ct(%44, %43);
            %54 = ks(%53);
            %55 = pbs<Lut1("SolvePropGroupFinal1")>(%54);
            %56 = add_ct(%45, %43);
            %57 = ks(%56);
            %58 = pbs<Lut1("SolvePropGroupFinal2")>(%57);
            %59 = add_ct(%17, %26);
            %60 = add_ct(%18, %47);
            %61 = add_ct(%19, %49);
            %62 = add_ct(%20, %43);
            %63 = add_ct(%21, %52);
            %64 = add_ct(%22, %55);
            %65 = add_ct(%23, %58);
            %66 = ks(%25);
            %67 = pbs<Lut1("MsgOnly")>(%66);
            %68 = ks(%59);
            %69 = pbs<Lut1("MsgOnly")>(%68);
            %70 = ks(%60);
            %71 = pbs<Lut1("MsgOnly")>(%70);
            %72 = ks(%61);
            %73 = pbs<Lut1("MsgOnly")>(%72);
            %74 = ks(%62);
            %75 = pbs<Lut1("MsgOnly")>(%74);
            %76 = ks(%63);
            %77 = pbs<Lut1("MsgOnly")>(%76);
            %78 = ks(%64);
            %79 = pbs<Lut1("MsgOnly")>(%78);
            %80 = ks(%65);
            %81 = pbs<Lut1("MsgOnly")>(%80);
            dst_st<0, 0>(%67);
            dst_st<0, 1>(%69);
            dst_st<0, 2>(%71);
            dst_st<0, 3>(%73);
            dst_st<0, 4>(%75);
            dst_st<0, 5>(%77);
            dst_st<0, 6>(%79);
            dst_st<0, 7>(%81);
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
            %17 = ks(%16);
            %18 = pbs<Lut1("None")>(%17);
            %19 = mac<4>(%3, %2);
            %20 = ks(%19);
            %21 = pbs<Lut1("None")>(%20);
            %22 = mac<4>(%5, %4);
            %23 = ks(%22);
            %24 = pbs<Lut1("None")>(%23);
            %25 = mac<4>(%7, %6);
            %26 = ks(%25);
            %27 = pbs<Lut1("None")>(%26);
            %28 = mac<4>(%9, %8);
            %29 = ks(%28);
            %30 = pbs<Lut1("None")>(%29);
            %31 = mac<4>(%11, %10);
            %32 = ks(%31);
            %33 = pbs<Lut1("None")>(%32);
            %34 = mac<4>(%13, %12);
            %35 = ks(%34);
            %36 = pbs<Lut1("None")>(%35);
            %37 = mac<4>(%15, %14);
            %38 = ks(%37);
            %39 = pbs<Lut1("None")>(%38);
            %40 = sub_ct(%18, %30);
            %41 = ks(%40);
            %42 = pbs<Lut1("CmpSign")>(%41);
            %43 = add_cst<1>(%42);
            %44 = sub_ct(%21, %33);
            %45 = ks(%44);
            %46 = pbs<Lut1("CmpSign")>(%45);
            %47 = add_cst<1>(%46);
            %48 = sub_ct(%24, %36);
            %49 = ks(%48);
            %50 = pbs<Lut1("CmpSign")>(%49);
            %51 = add_cst<1>(%50);
            %52 = sub_ct(%27, %39);
            %53 = ks(%52);
            %54 = pbs<Lut1("CmpSign")>(%53);
            %55 = add_cst<1>(%54);
            %56 = mac<4>(%47, %43);
            %57 = ks(%56);
            %58 = pbs<Lut1("CmpReduce")>(%57);
            %59 = mac<4>(%55, %51);
            %60 = ks(%59);
            %61 = pbs<Lut1("CmpReduce")>(%60);
            %62 = mac<4>(%61, %58);
            %63 = ks(%62);
            %64 = pbs<Lut1("CmpGtMrg")>(%63);
            dst_st<0, 0>(%64);
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
    check_iop_vm_equivalence(&iop_ir, &vm_ir, spec);
}
