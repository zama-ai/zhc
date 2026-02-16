use hc_builder::{
    builder::CiphertextSpec,
    iops::{add::add, cmp::cmp_gt},
};
use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::IopLang;
use hc_utils::assert_display_is;

pub fn get_add_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<IopLang> {
    let ir = add(CiphertextSpec::new(
        integer_w as u16,
        msg_w as u8,
        carry_w as u8,
    ))
    .into_ir();
    ir
}

pub fn get_cmp_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<IopLang> {
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
    assert_display_is!(
        ir.format(),
        r#"
            %0 : CtInt = input<0, CtInt>();
            %1 : CtInt = input<1, CtInt>();
            %89 : CtInt = decl_ct();
            %2 : CtBlock = extract_ct_block<0>(%0 : CtInt);
            %3 : CtBlock = extract_ct_block<1>(%0 : CtInt);
            %4 : CtBlock = extract_ct_block<2>(%0 : CtInt);
            %5 : CtBlock = extract_ct_block<3>(%0 : CtInt);
            %6 : CtBlock = extract_ct_block<4>(%0 : CtInt);
            %7 : CtBlock = extract_ct_block<5>(%0 : CtInt);
            %8 : CtBlock = extract_ct_block<6>(%0 : CtInt);
            %9 : CtBlock = extract_ct_block<7>(%0 : CtInt);
            %10 : CtBlock = extract_ct_block<0>(%1 : CtInt);
            %11 : CtBlock = extract_ct_block<1>(%1 : CtInt);
            %12 : CtBlock = extract_ct_block<2>(%1 : CtInt);
            %13 : CtBlock = extract_ct_block<3>(%1 : CtInt);
            %14 : CtBlock = extract_ct_block<4>(%1 : CtInt);
            %15 : CtBlock = extract_ct_block<5>(%1 : CtInt);
            %16 : CtBlock = extract_ct_block<6>(%1 : CtInt);
            %17 : CtBlock = extract_ct_block<7>(%1 : CtInt);
            %18 : CtBlock = add_ct(%2 : CtBlock, %10 : CtBlock);
            %19 : CtBlock = add_ct(%3 : CtBlock, %11 : CtBlock);
            %20 : CtBlock = add_ct(%4 : CtBlock, %12 : CtBlock);
            %21 : CtBlock = add_ct(%5 : CtBlock, %13 : CtBlock);
            %22 : CtBlock = add_ct(%6 : CtBlock, %14 : CtBlock);
            %23 : CtBlock = add_ct(%7 : CtBlock, %15 : CtBlock);
            %24 : CtBlock = add_ct(%8 : CtBlock, %16 : CtBlock);
            %25 : CtBlock = add_ct(%9 : CtBlock, %17 : CtBlock);
            %27 : CtBlock, %28 : CtBlock = pbs2<ManyCarryMsg>(%18 : CtBlock);
            %29 : CtBlock = pbs<ExtractPropGroup0>(%19 : CtBlock);
            %30 : CtBlock = pbs<ExtractPropGroup1>(%20 : CtBlock);
            %31 : CtBlock = pbs<ExtractPropGroup2>(%21 : CtBlock);
            %32 : CtBlock = pbs<ExtractPropGroup0>(%22 : CtBlock);
            %33 : CtBlock = pbs<ExtractPropGroup1>(%23 : CtBlock);
            %34 : CtBlock = pbs<ExtractPropGroup2>(%24 : CtBlock);
            %36 : CtBlock = add_ct(%28 : CtBlock, %29 : CtBlock);
            %44 : CtBlock = add_ct(%32 : CtBlock, %33 : CtBlock);
            %74 : CtBlock = add_ct(%19 : CtBlock, %28 : CtBlock);
            %81 : CtBlock = pbs<MsgOnly>(%27 : CtBlock);
            %37 : CtBlock = add_ct(%36 : CtBlock, %30 : CtBlock);
            %45 : CtBlock = add_ct(%44 : CtBlock, %34 : CtBlock);
            %56 : CtBlock = pbs<SolvePropGroupFinal0>(%36 : CtBlock);
            %82 : CtBlock = pbs<MsgOnly>(%74 : CtBlock);
            %90 : CtInt = store_ct_block<0>(%81 : CtBlock, %89 : CtInt);
            %38 : CtBlock = temper_add_ct(%37 : CtBlock, %31 : CtBlock);
            %57 : CtBlock = pbs<SolvePropGroupFinal1>(%37 : CtBlock);
            %75 : CtBlock = add_ct(%20 : CtBlock, %56 : CtBlock);
            %91 : CtInt = store_ct_block<1>(%82 : CtBlock, %90 : CtInt);
            %39 : CtBlock = pbs<SolvePropGroupFinal2>(%38 : CtBlock);
            %76 : CtBlock = add_ct(%21 : CtBlock, %57 : CtBlock);
            %83 : CtBlock = pbs<MsgOnly>(%75 : CtBlock);
            %62 : CtBlock = add_ct(%32 : CtBlock, %39 : CtBlock);
            %64 : CtBlock = add_ct(%44 : CtBlock, %39 : CtBlock);
            %66 : CtBlock = add_ct(%45 : CtBlock, %39 : CtBlock);
            %77 : CtBlock = add_ct(%22 : CtBlock, %39 : CtBlock);
            %84 : CtBlock = pbs<MsgOnly>(%76 : CtBlock);
            %92 : CtInt = store_ct_block<2>(%83 : CtBlock, %91 : CtInt);
            %63 : CtBlock = pbs<SolvePropGroupFinal0>(%62 : CtBlock);
            %65 : CtBlock = pbs<SolvePropGroupFinal1>(%64 : CtBlock);
            %67 : CtBlock = pbs<SolvePropGroupFinal2>(%66 : CtBlock);
            %85 : CtBlock = pbs<MsgOnly>(%77 : CtBlock);
            %93 : CtInt = store_ct_block<3>(%84 : CtBlock, %92 : CtInt);
            %78 : CtBlock = add_ct(%23 : CtBlock, %63 : CtBlock);
            %79 : CtBlock = add_ct(%24 : CtBlock, %65 : CtBlock);
            %80 : CtBlock = add_ct(%25 : CtBlock, %67 : CtBlock);
            %94 : CtInt = store_ct_block<4>(%85 : CtBlock, %93 : CtInt);
            %86 : CtBlock = pbs<MsgOnly>(%78 : CtBlock);
            %87 : CtBlock = pbs<MsgOnly>(%79 : CtBlock);
            %88 : CtBlock = pbs<MsgOnly>(%80 : CtBlock);
            %95 : CtInt = store_ct_block<5>(%86 : CtBlock, %94 : CtInt);
            %96 : CtInt = store_ct_block<6>(%87 : CtBlock, %95 : CtInt);
            %97 : CtInt = store_ct_block<7>(%88 : CtBlock, %96 : CtInt);
            output<0, CtInt>(%97 : CtInt);
        "#
    );
}

#[test]
fn test_cmp_ir() {
    let ir = get_cmp_ir(16, 2, 2);
    assert_display_is!(
        ir.format(),
        r#"
            %0 : CtInt = input<0, CtInt>();
            %9 : CtInt = input<1, CtInt>();
            %36 : PtBlock = let_pt_block<1>();
            %56 : CtInt = decl_ct();
            %1 : CtBlock = extract_ct_block<0>(%0 : CtInt);
            %2 : CtBlock = extract_ct_block<1>(%0 : CtInt);
            %3 : CtBlock = extract_ct_block<2>(%0 : CtInt);
            %4 : CtBlock = extract_ct_block<3>(%0 : CtInt);
            %5 : CtBlock = extract_ct_block<4>(%0 : CtInt);
            %6 : CtBlock = extract_ct_block<5>(%0 : CtInt);
            %7 : CtBlock = extract_ct_block<6>(%0 : CtInt);
            %8 : CtBlock = extract_ct_block<7>(%0 : CtInt);
            %10 : CtBlock = extract_ct_block<0>(%9 : CtInt);
            %11 : CtBlock = extract_ct_block<1>(%9 : CtInt);
            %12 : CtBlock = extract_ct_block<2>(%9 : CtInt);
            %13 : CtBlock = extract_ct_block<3>(%9 : CtInt);
            %14 : CtBlock = extract_ct_block<4>(%9 : CtInt);
            %15 : CtBlock = extract_ct_block<5>(%9 : CtInt);
            %16 : CtBlock = extract_ct_block<6>(%9 : CtInt);
            %17 : CtBlock = extract_ct_block<7>(%9 : CtInt);
            %18 : CtBlock = pack_ct<4>(%2 : CtBlock, %1 : CtBlock);
            %20 : CtBlock = pack_ct<4>(%4 : CtBlock, %3 : CtBlock);
            %22 : CtBlock = pack_ct<4>(%6 : CtBlock, %5 : CtBlock);
            %24 : CtBlock = pack_ct<4>(%8 : CtBlock, %7 : CtBlock);
            %26 : CtBlock = pack_ct<4>(%11 : CtBlock, %10 : CtBlock);
            %28 : CtBlock = pack_ct<4>(%13 : CtBlock, %12 : CtBlock);
            %30 : CtBlock = pack_ct<4>(%15 : CtBlock, %14 : CtBlock);
            %32 : CtBlock = pack_ct<4>(%17 : CtBlock, %16 : CtBlock);
            %19 : CtBlock = pbs<None>(%18 : CtBlock);
            %21 : CtBlock = pbs<None>(%20 : CtBlock);
            %23 : CtBlock = pbs<None>(%22 : CtBlock);
            %25 : CtBlock = pbs<None>(%24 : CtBlock);
            %27 : CtBlock = pbs<None>(%26 : CtBlock);
            %29 : CtBlock = pbs<None>(%28 : CtBlock);
            %31 : CtBlock = pbs<None>(%30 : CtBlock);
            %33 : CtBlock = pbs<None>(%32 : CtBlock);
            %34 : CtBlock = sub_ct(%19 : CtBlock, %27 : CtBlock);
            %38 : CtBlock = sub_ct(%21 : CtBlock, %29 : CtBlock);
            %42 : CtBlock = sub_ct(%23 : CtBlock, %31 : CtBlock);
            %46 : CtBlock = sub_ct(%25 : CtBlock, %33 : CtBlock);
            %35 : CtBlock = pbs<CmpSign>(%34 : CtBlock);
            %39 : CtBlock = pbs<CmpSign>(%38 : CtBlock);
            %43 : CtBlock = pbs<CmpSign>(%42 : CtBlock);
            %47 : CtBlock = pbs<CmpSign>(%46 : CtBlock);
            %37 : CtBlock = add_pt(%35 : CtBlock, %36 : PtBlock);
            %41 : CtBlock = add_pt(%39 : CtBlock, %36 : PtBlock);
            %45 : CtBlock = add_pt(%43 : CtBlock, %36 : PtBlock);
            %49 : CtBlock = add_pt(%47 : CtBlock, %36 : PtBlock);
            %50 : CtBlock = pack_ct<4>(%41 : CtBlock, %37 : CtBlock);
            %51 : CtBlock = pack_ct<4>(%49 : CtBlock, %45 : CtBlock);
            %52 : CtBlock = pbs<CmpReduce>(%50 : CtBlock);
            %53 : CtBlock = pbs<CmpReduce>(%51 : CtBlock);
            %54 : CtBlock = pack_ct<4>(%53 : CtBlock, %52 : CtBlock);
            %55 : CtBlock = pbs<CmpGtMrg>(%54 : CtBlock);
            %57 : CtInt = store_ct_block<0>(%55 : CtBlock, %56 : CtInt);
            output<0, CtInt>(%57 : CtInt);
        "#
    );
}
