use hc_builder::{
    builder::CiphertextSpec,
    iops::{add::add, cmp::cmp_gt},
};
use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::IopLang;
use hc_utils::assert_display_is;

pub fn get_add_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<IopLang> {
    let mut ir = add(CiphertextSpec::new(
        integer_w as u16,
        msg_w as u8,
        carry_w as u8,
    ));
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
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
            %9 : CtInt = input<1, CtInt>();
            %72 : CtInt = zero_ct();
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
            %18 : CtBlock = add_ct(%1 : CtBlock, %10 : CtBlock);
            %19 : CtBlock = add_ct(%2 : CtBlock, %11 : CtBlock);
            %20 : CtBlock = add_ct(%3 : CtBlock, %12 : CtBlock);
            %21 : CtBlock = add_ct(%4 : CtBlock, %13 : CtBlock);
            %22 : CtBlock = add_ct(%5 : CtBlock, %14 : CtBlock);
            %23 : CtBlock = add_ct(%6 : CtBlock, %15 : CtBlock);
            %24 : CtBlock = add_ct(%7 : CtBlock, %16 : CtBlock);
            %25 : CtBlock = add_ct(%8 : CtBlock, %17 : CtBlock);
            %26 : CtBlock, %27 : CtBlock = pbs2<ManyCarryMsg>(%18 : CtBlock);
            %28 : CtBlock = pbs<ExtractPropGroup0>(%19 : CtBlock);
            %29 : CtBlock = pbs<ExtractPropGroup1>(%20 : CtBlock);
            %30 : CtBlock = pbs<ExtractPropGroup2>(%21 : CtBlock);
            %31 : CtBlock = pbs<ExtractPropGroup0>(%22 : CtBlock);
            %32 : CtBlock = pbs<ExtractPropGroup1>(%23 : CtBlock);
            %33 : CtBlock = pbs<ExtractPropGroup2>(%24 : CtBlock);
            %35 : CtBlock = add_ct(%27 : CtBlock, %28 : CtBlock);
            %39 : CtBlock = add_ct(%31 : CtBlock, %32 : CtBlock);
            %57 : CtBlock = add_ct(%19 : CtBlock, %27 : CtBlock);
            %64 : CtBlock = pbs<MsgOnly>(%26 : CtBlock);
            %36 : CtBlock = add_ct(%35 : CtBlock, %29 : CtBlock);
            %40 : CtBlock = add_ct(%39 : CtBlock, %33 : CtBlock);
            %47 : CtBlock = pbs<SolvePropGroupFinal0>(%35 : CtBlock);
            %65 : CtBlock = pbs<MsgOnly>(%57 : CtBlock);
            %73 : CtInt = store_ct_block<0>(%64 : CtBlock, %72 : CtInt);
            %37 : CtBlock = add_ct(%36 : CtBlock, %30 : CtBlock);
            %48 : CtBlock = pbs<SolvePropGroupFinal1>(%36 : CtBlock);
            %58 : CtBlock = add_ct(%20 : CtBlock, %47 : CtBlock);
            %74 : CtInt = store_ct_block<1>(%65 : CtBlock, %73 : CtInt);
            %38 : CtBlock = pbs<SolvePropGroupFinal2>(%37 : CtBlock);
            %59 : CtBlock = add_ct(%21 : CtBlock, %48 : CtBlock);
            %66 : CtBlock = pbs<MsgOnly>(%58 : CtBlock);
            %49 : CtBlock = add_ct(%31 : CtBlock, %38 : CtBlock);
            %51 : CtBlock = add_ct(%39 : CtBlock, %38 : CtBlock);
            %53 : CtBlock = add_ct(%40 : CtBlock, %38 : CtBlock);
            %60 : CtBlock = add_ct(%22 : CtBlock, %38 : CtBlock);
            %67 : CtBlock = pbs<MsgOnly>(%59 : CtBlock);
            %75 : CtInt = store_ct_block<2>(%66 : CtBlock, %74 : CtInt);
            %50 : CtBlock = pbs<SolvePropGroupFinal0>(%49 : CtBlock);
            %52 : CtBlock = pbs<SolvePropGroupFinal1>(%51 : CtBlock);
            %54 : CtBlock = pbs<SolvePropGroupFinal2>(%53 : CtBlock);
            %68 : CtBlock = pbs<MsgOnly>(%60 : CtBlock);
            %76 : CtInt = store_ct_block<3>(%67 : CtBlock, %75 : CtInt);
            %61 : CtBlock = add_ct(%23 : CtBlock, %50 : CtBlock);
            %62 : CtBlock = add_ct(%24 : CtBlock, %52 : CtBlock);
            %63 : CtBlock = add_ct(%25 : CtBlock, %54 : CtBlock);
            %77 : CtInt = store_ct_block<4>(%68 : CtBlock, %76 : CtInt);
            %69 : CtBlock = pbs<MsgOnly>(%61 : CtBlock);
            %70 : CtBlock = pbs<MsgOnly>(%62 : CtBlock);
            %71 : CtBlock = pbs<MsgOnly>(%63 : CtBlock);
            %78 : CtInt = store_ct_block<5>(%69 : CtBlock, %77 : CtInt);
            %79 : CtInt = store_ct_block<6>(%70 : CtBlock, %78 : CtInt);
            %80 : CtInt = store_ct_block<7>(%71 : CtBlock, %79 : CtInt);
            output<0, CtInt>(%80 : CtInt);
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
        %56 : CtInt = zero_ct();
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
