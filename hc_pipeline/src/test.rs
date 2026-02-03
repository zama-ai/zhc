use hc_builder::{
    builder::CiphertextSpec,
    iops::{add::add, cmp::cmp_gt},
};
use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::Ioplang;

pub fn get_add_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let mut ir = add(CiphertextSpec::new(
        integer_w as u16,
        msg_w as u8,
        carry_w as u8,
    ));
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    ir
}

pub fn get_cmp_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let mut ir = cmp_gt(CiphertextSpec::new(
        integer_w as u16,
        msg_w as u8,
        carry_w as u8,
    ));
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    ir
}

#[test]
fn test_add_ir() {
    let ir = get_add_ir(16, 2, 2);
    ir.check_ir(
        "
        %0 : CtInt = input<0, CtInt>();
        %1 : CtInt = input<1, CtInt>();
        %2 : PtBlock = let_pt_block<1>();
        %3 : CtInt = zero_ct();
        %4 : CtBlock = extract_ct_block<0>(%0);
        %5 : CtBlock = extract_ct_block<1>(%0);
        %6 : CtBlock = extract_ct_block<2>(%0);
        %7 : CtBlock = extract_ct_block<3>(%0);
        %8 : CtBlock = extract_ct_block<4>(%0);
        %9 : CtBlock = extract_ct_block<5>(%0);
        %10 : CtBlock = extract_ct_block<6>(%0);
        %11 : CtBlock = extract_ct_block<7>(%0);
        %12 : CtBlock = extract_ct_block<0>(%1);
        %13 : CtBlock = extract_ct_block<1>(%1);
        %14 : CtBlock = extract_ct_block<2>(%1);
        %15 : CtBlock = extract_ct_block<3>(%1);
        %16 : CtBlock = extract_ct_block<4>(%1);
        %17 : CtBlock = extract_ct_block<5>(%1);
        %18 : CtBlock = extract_ct_block<6>(%1);
        %19 : CtBlock = extract_ct_block<7>(%1);
        %20 : CtBlock = add_ct(%4, %12);
        %21 : CtBlock = add_ct(%5, %13);
        %22 : CtBlock = add_ct(%6, %14);
        %23 : CtBlock = add_ct(%7, %15);
        %24 : CtBlock = add_ct(%8, %16);
        %25 : CtBlock = add_ct(%9, %17);
        %26 : CtBlock = add_ct(%10, %18);
        %27 : CtBlock = add_ct(%11, %19);
        %28 : CtBlock, %29 : CtBlock = pbs2<ManyCarryMsg>(%20);
        %30 : CtBlock = pbs<ExtractPropGroup0>(%21);
        %31 : CtBlock = pbs<ExtractPropGroup1>(%22);
        %32 : CtBlock = pbs<ExtractPropGroup2>(%23);
        %33 : CtBlock = pbs<ExtractPropGroup0>(%24);
        %34 : CtBlock = pbs<ExtractPropGroup1>(%25);
        %35 : CtBlock = pbs<ExtractPropGroup2>(%26);
        %36 : CtBlock = pbs<ExtractPropGroup3>(%27);
        %39 : CtBlock = add_ct(%29, %30);
        %40 : CtBlock = add_ct(%33, %34);
        %41 : CtBlock = add_ct(%27, %28);
        %42 : CtBlock = add_ct(%39, %31);
        %43 : CtBlock = add_ct(%40, %35);
        %44 : CtBlock = pbs<SolvePropGroupFinal0>(%39);
        %45 : CtBlock = add_ct(%42, %32);
        %46 : CtBlock = add_ct(%43, %36);
        %47 : CtBlock = pbs<SolvePropGroupFinal1>(%42);
        %48 : CtBlock = add_ct(%20, %44);
        %49 : CtBlock = pbs<SolvePropGroupFinal2>(%45);
        %50 : CtBlock = pbs<ReduceCarryPad>(%46);
        %51 : CtBlock = add_ct(%21, %47);
        %52 : CtInt = store_ct_block<0>(%48, %3);
        %53 : CtBlock = add_pt(%50, %2);
        %54 : CtBlock = add_ct(%33, %49);
        %55 : CtBlock = add_ct(%40, %49);
        %56 : CtBlock = add_ct(%43, %49);
        %57 : CtBlock = add_ct(%22, %49);
        %58 : CtInt = store_ct_block<1>(%51, %52);
        %59 : CtBlock = pack_ct<4>(%53, %49);
        %60 : CtBlock = pbs<SolvePropGroupFinal0>(%54);
        %61 : CtBlock = pbs<SolvePropGroupFinal1>(%55);
        %62 : CtBlock = pbs<SolvePropGroupFinal2>(%56);
        %63 : CtInt = store_ct_block<2>(%57, %58);
        %64 : CtBlock = pbs<SolvePropCarry>(%59);
        %65 : CtBlock = add_ct(%23, %60);
        %66 : CtBlock = add_ct(%24, %61);
        %67 : CtBlock = add_ct(%25, %62);
        %68 : CtBlock = add_ct(%26, %64);
        %69 : CtInt = store_ct_block<3>(%65, %63);
        %70 : CtInt = store_ct_block<4>(%66, %69);
        %71 : CtInt = store_ct_block<5>(%67, %70);
        %72 : CtInt = store_ct_block<6>(%68, %71);
        %73 : CtInt = store_ct_block<7>(%41, %72);
        output<0, CtInt>(%73);
    ",
    );
}

#[test]
fn test_cmp_ir() {
    let ir = get_cmp_ir(16, 2, 2);
    ir.check_ir(
        "
        %0 : CtInt = input<0, CtInt>();
        %1 : CtInt = input<1, CtInt>();
        %2 : PtBlock = let_pt_block<1>();
        %6 : CtInt = zero_ct();
        %7 : CtBlock = extract_ct_block<0>(%0);
        %8 : CtBlock = extract_ct_block<1>(%0);
        %9 : CtBlock = extract_ct_block<2>(%0);
        %10 : CtBlock = extract_ct_block<3>(%0);
        %11 : CtBlock = extract_ct_block<4>(%0);
        %12 : CtBlock = extract_ct_block<5>(%0);
        %13 : CtBlock = extract_ct_block<6>(%0);
        %14 : CtBlock = extract_ct_block<7>(%0);
        %15 : CtBlock = extract_ct_block<0>(%1);
        %16 : CtBlock = extract_ct_block<1>(%1);
        %17 : CtBlock = extract_ct_block<2>(%1);
        %18 : CtBlock = extract_ct_block<3>(%1);
        %19 : CtBlock = extract_ct_block<4>(%1);
        %20 : CtBlock = extract_ct_block<5>(%1);
        %21 : CtBlock = extract_ct_block<6>(%1);
        %22 : CtBlock = extract_ct_block<7>(%1);
        %23 : CtBlock = pack_ct<4>(%8, %7);
        %24 : CtBlock = pack_ct<4>(%10, %9);
        %25 : CtBlock = pack_ct<4>(%12, %11);
        %26 : CtBlock = pack_ct<4>(%14, %13);
        %27 : CtBlock = pack_ct<4>(%16, %15);
        %28 : CtBlock = pack_ct<4>(%18, %17);
        %29 : CtBlock = pack_ct<4>(%20, %19);
        %30 : CtBlock = pack_ct<4>(%22, %21);
        %31 : CtBlock = pbs<None>(%23);
        %32 : CtBlock = pbs<None>(%24);
        %33 : CtBlock = pbs<None>(%25);
        %34 : CtBlock = pbs<None>(%26);
        %35 : CtBlock = pbs<None>(%27);
        %36 : CtBlock = pbs<None>(%28);
        %37 : CtBlock = pbs<None>(%29);
        %38 : CtBlock = pbs<None>(%30);
        %39 : CtBlock = sub_ct(%31, %35);
        %40 : CtBlock = sub_ct(%32, %36);
        %41 : CtBlock = sub_ct(%33, %37);
        %42 : CtBlock = sub_ct(%34, %38);
        %43 : CtBlock = pbs<CmpSign>(%39);
        %44 : CtBlock = pbs<CmpSign>(%40);
        %45 : CtBlock = pbs<CmpSign>(%41);
        %46 : CtBlock = pbs<CmpSign>(%42);
        %47 : CtBlock = add_pt(%43, %2);
        %48 : CtBlock = add_pt(%44, %2);
        %49 : CtBlock = add_pt(%45, %2);
        %50 : CtBlock = add_pt(%46, %2);
        %51 : CtBlock = pack_ct<4>(%48, %47);
        %52 : CtBlock = pack_ct<4>(%50, %49);
        %53 : CtBlock = pbs<CmpReduce>(%51);
        %54 : CtBlock = pbs<CmpReduce>(%52);
        %55 : CtBlock = pack_ct<4>(%54, %53);
        %56 : CtBlock = pbs<CmpGtMrg>(%55);
        %57 : CtInt = store_ct_block<0>(%56, %6);
        output<0, CtInt>(%57);
    ",
    );
}
