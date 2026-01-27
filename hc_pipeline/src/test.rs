use hc_builder::{builder::BlockConfig, iops::{add::add, cmp::cmp_gt}};
use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::Ioplang;

pub fn get_add_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let mut ir = add(
        integer_w as u8,
        &BlockConfig {
            message_width: msg_w as u8,
            carry_width: carry_w as u8,
        },
    );
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    ir
}

pub fn get_cmp_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let mut ir = cmp_gt(
        integer_w as u8,
        &BlockConfig {
            message_width: msg_w as u8,
            carry_width: carry_w as u8,
        },
    );
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    ir
}

#[test]
fn test_add_ir() {
    let ir = get_add_ir(16, 2, 2);
    ir.check_ir(
        "
        %0 : Ciphertext = input<0, Ciphertext>();
        %1 : Index = constant<0_idx>();
        %2 : Index = constant<1_idx>();
        %3 : Index = constant<2_idx>();
        %4 : Index = constant<3_idx>();
        %5 : Index = constant<4_idx>();
        %6 : Index = constant<5_idx>();
        %7 : Index = constant<6_idx>();
        %8 : Index = constant<7_idx>();
        %9 : Ciphertext = input<1, Ciphertext>();
        %18 : Lut2 = gen_lut2<ManyCarryMsg>();
        %19 : Lut1 = gen_lut1<ReduceCarryPad>();
        %20 : Lut1 = gen_lut1<SolvePropGroupFinal0>();
        %21 : Lut1 = gen_lut1<SolvePropGroupFinal1>();
        %22 : Lut1 = gen_lut1<SolvePropGroupFinal2>();
        %23 : Lut1 = gen_lut1<ExtractPropGroup0>();
        %24 : Lut1 = gen_lut1<ExtractPropGroup1>();
        %25 : Lut1 = gen_lut1<ExtractPropGroup2>();
        %26 : Lut1 = gen_lut1<ExtractPropGroup3>();
        %27 : Lut1 = gen_lut1<SolvePropCarry>();
        %29 : PlaintextBlock = constant<1_pt_block>();
        %30 : PlaintextBlock = constant<4_pt_block>();
        %31 : Ciphertext = let<Ciphertext>();
        %40 : CiphertextBlock = extract_ct_block(%0, %1);
        %41 : CiphertextBlock = extract_ct_block(%0, %2);
        %42 : CiphertextBlock = extract_ct_block(%0, %3);
        %43 : CiphertextBlock = extract_ct_block(%0, %4);
        %44 : CiphertextBlock = extract_ct_block(%0, %5);
        %45 : CiphertextBlock = extract_ct_block(%0, %6);
        %46 : CiphertextBlock = extract_ct_block(%0, %7);
        %47 : CiphertextBlock = extract_ct_block(%0, %8);
        %48 : CiphertextBlock = extract_ct_block(%9, %1);
        %49 : CiphertextBlock = extract_ct_block(%9, %2);
        %50 : CiphertextBlock = extract_ct_block(%9, %3);
        %51 : CiphertextBlock = extract_ct_block(%9, %4);
        %52 : CiphertextBlock = extract_ct_block(%9, %5);
        %53 : CiphertextBlock = extract_ct_block(%9, %6);
        %54 : CiphertextBlock = extract_ct_block(%9, %7);
        %55 : CiphertextBlock = extract_ct_block(%9, %8);
        %56 : CiphertextBlock = add_ct(%40, %48);
        %57 : CiphertextBlock = add_ct(%41, %49);
        %58 : CiphertextBlock = add_ct(%42, %50);
        %59 : CiphertextBlock = add_ct(%43, %51);
        %60 : CiphertextBlock = add_ct(%44, %52);
        %61 : CiphertextBlock = add_ct(%45, %53);
        %62 : CiphertextBlock = add_ct(%46, %54);
        %63 : CiphertextBlock = add_ct(%47, %55);
        %64 : CiphertextBlock, %65 : CiphertextBlock = pbs2(%56, %18);
        %66 : CiphertextBlock = pbs(%57, %23);
        %67 : CiphertextBlock = pbs(%58, %24);
        %68 : CiphertextBlock = pbs(%59, %25);
        %69 : CiphertextBlock = pbs(%60, %23);
        %70 : CiphertextBlock = pbs(%61, %24);
        %71 : CiphertextBlock = pbs(%62, %25);
        %72 : CiphertextBlock = pbs(%63, %26);
        %75 : CiphertextBlock = add_ct(%65, %66);
        %76 : CiphertextBlock = add_ct(%69, %70);
        %77 : CiphertextBlock = add_ct(%63, %64);
        %78 : CiphertextBlock = add_ct(%75, %67);
        %79 : CiphertextBlock = add_ct(%76, %71);
        %80 : CiphertextBlock = pbs(%75, %20);
        %81 : CiphertextBlock = add_ct(%78, %68);
        %82 : CiphertextBlock = add_ct(%79, %72);
        %83 : CiphertextBlock = pbs(%78, %21);
        %84 : CiphertextBlock = add_ct(%56, %80);
        %85 : CiphertextBlock = pbs(%81, %22);
        %86 : CiphertextBlock = pbs(%82, %19);
        %87 : CiphertextBlock = add_ct(%57, %83);
        %88 : Ciphertext = store_ct_block(%84, %31, %1);
        %89 : CiphertextBlock = add_pt(%86, %29);
        %90 : CiphertextBlock = add_ct(%69, %85);
        %91 : CiphertextBlock = add_ct(%76, %85);
        %92 : CiphertextBlock = add_ct(%79, %85);
        %93 : CiphertextBlock = add_ct(%58, %85);
        %94 : Ciphertext = store_ct_block(%87, %88, %2);
        %95 : CiphertextBlock = mac(%30, %89, %85);
        %96 : CiphertextBlock = pbs(%90, %20);
        %97 : CiphertextBlock = pbs(%91, %21);
        %98 : CiphertextBlock = pbs(%92, %22);
        %99 : Ciphertext = store_ct_block(%93, %94, %3);
        %100 : CiphertextBlock = pbs(%95, %27);
        %101 : CiphertextBlock = add_ct(%59, %96);
        %102 : CiphertextBlock = add_ct(%60, %97);
        %103 : CiphertextBlock = add_ct(%61, %98);
        %104 : CiphertextBlock = add_ct(%62, %100);
        %105 : Ciphertext = store_ct_block(%101, %99, %4);
        %106 : Ciphertext = store_ct_block(%102, %105, %5);
        %107 : Ciphertext = store_ct_block(%103, %106, %6);
        %108 : Ciphertext = store_ct_block(%104, %107, %7);
        %109 : Ciphertext = store_ct_block(%77, %108, %8);
        output<0, Ciphertext>(%109);
    ",
    );
}

#[test]
fn test_cmp_ir() {
    let ir = get_cmp_ir(16, 2, 2);
    ir.check_ir(
        "
        %0 : Ciphertext = input<0, Ciphertext>();
        %1 : Index = constant<0_idx>();
        %2 : Index = constant<1_idx>();
        %3 : Index = constant<2_idx>();
        %4 : Index = constant<3_idx>();
        %5 : Index = constant<4_idx>();
        %6 : Index = constant<5_idx>();
        %7 : Index = constant<6_idx>();
        %8 : Index = constant<7_idx>();
        %9 : Ciphertext = input<1, Ciphertext>();
        %18 : Lut1 = gen_lut1<CmpSign>();
        %19 : Lut1 = gen_lut1<CmpReduce>();
        %20 : Lut1 = gen_lut1<CmpGtMrg>();
        %22 : Lut1 = gen_lut1<None>();
        %23 : PlaintextBlock = constant<4_pt_block>();
        %24 : Lut1 = gen_lut1<None>();
        %26 : PlaintextBlock = constant<1_pt_block>();
        %32 : Ciphertext = let<Ciphertext>();
        %34 : CiphertextBlock = extract_ct_block(%0, %1);
        %35 : CiphertextBlock = extract_ct_block(%0, %2);
        %36 : CiphertextBlock = extract_ct_block(%0, %3);
        %37 : CiphertextBlock = extract_ct_block(%0, %4);
        %38 : CiphertextBlock = extract_ct_block(%0, %5);
        %39 : CiphertextBlock = extract_ct_block(%0, %6);
        %40 : CiphertextBlock = extract_ct_block(%0, %7);
        %41 : CiphertextBlock = extract_ct_block(%0, %8);
        %42 : CiphertextBlock = extract_ct_block(%9, %1);
        %43 : CiphertextBlock = extract_ct_block(%9, %2);
        %44 : CiphertextBlock = extract_ct_block(%9, %3);
        %45 : CiphertextBlock = extract_ct_block(%9, %4);
        %46 : CiphertextBlock = extract_ct_block(%9, %5);
        %47 : CiphertextBlock = extract_ct_block(%9, %6);
        %48 : CiphertextBlock = extract_ct_block(%9, %7);
        %49 : CiphertextBlock = extract_ct_block(%9, %8);
        %50 : CiphertextBlock = mac(%23, %35, %34);
        %51 : CiphertextBlock = mac(%23, %37, %36);
        %52 : CiphertextBlock = mac(%23, %39, %38);
        %53 : CiphertextBlock = mac(%23, %41, %40);
        %54 : CiphertextBlock = mac(%23, %43, %42);
        %55 : CiphertextBlock = mac(%23, %45, %44);
        %56 : CiphertextBlock = mac(%23, %47, %46);
        %57 : CiphertextBlock = mac(%23, %49, %48);
        %58 : CiphertextBlock = pbs(%50, %22);
        %59 : CiphertextBlock = pbs(%51, %22);
        %60 : CiphertextBlock = pbs(%52, %22);
        %61 : CiphertextBlock = pbs(%53, %22);
        %62 : CiphertextBlock = pbs(%54, %24);
        %63 : CiphertextBlock = pbs(%55, %24);
        %64 : CiphertextBlock = pbs(%56, %24);
        %65 : CiphertextBlock = pbs(%57, %24);
        %66 : CiphertextBlock = sub_ct(%58, %62);
        %67 : CiphertextBlock = sub_ct(%59, %63);
        %68 : CiphertextBlock = sub_ct(%60, %64);
        %69 : CiphertextBlock = sub_ct(%61, %65);
        %70 : CiphertextBlock = pbs(%66, %18);
        %71 : CiphertextBlock = pbs(%67, %18);
        %72 : CiphertextBlock = pbs(%68, %18);
        %73 : CiphertextBlock = pbs(%69, %18);
        %74 : CiphertextBlock = add_pt(%70, %26);
        %75 : CiphertextBlock = add_pt(%71, %26);
        %76 : CiphertextBlock = add_pt(%72, %26);
        %77 : CiphertextBlock = add_pt(%73, %26);
        %78 : CiphertextBlock = mac(%23, %75, %74);
        %79 : CiphertextBlock = mac(%23, %77, %76);
        %80 : CiphertextBlock = pbs(%78, %19);
        %81 : CiphertextBlock = pbs(%79, %19);
        %82 : CiphertextBlock = mac(%23, %81, %80);
        %83 : CiphertextBlock = pbs(%82, %20);
        %84 : Ciphertext = store_ct_block(%83, %32, %1);
        output<0, Ciphertext>(%84);
    ",
    );
}
