use std::path::Path;

use hc_builder::{builder::BlockConfig, iops::cmp::cmp_gt};
use hc_frontend::{BuilderContext, create_rhai_engine};
use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::Ioplang;

fn get_ir(path: &Path, integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let context = BuilderContext {
        integer_w,
        msg_w,
        carry_w,
        nu_msg: 2,
        nu_bool: 2,
    };
    let (engine, builder) = create_rhai_engine(context);
    engine.run_file(path.into()).unwrap();
    drop(engine);
    IR::<Ioplang>::try_from(builder).unwrap()
}

pub fn get_add_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("iop/add.rhai");
    let mut ir = get_ir(&path, integer_w, msg_w, carry_w);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    ir
}

pub fn get_sub_ir(integer_w: i64, msg_w: i64, carry_w: i64) -> IR<Ioplang> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("iop/sub.rhai");
    let mut ir = get_ir(&path, integer_w, msg_w, carry_w);
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
        %9 : Ciphertext = input<1, Ciphertext>();
        %18 : Lut2 = gen_lut2<ManyCarryMsg>();
        %23 : Lut1 = gen_lut1<SolvePropGroupFinal0>();
        %24 : Lut1 = gen_lut1<SolvePropGroupFinal1>();
        %25 : Lut1 = gen_lut1<SolvePropGroupFinal2>();
        %26 : Lut1 = gen_lut1<ExtractPropGroup0>();
        %27 : Lut1 = gen_lut1<ExtractPropGroup1>();
        %28 : Lut1 = gen_lut1<ExtractPropGroup2>();
        %32 : Ciphertext = let<Ciphertext>();
        %40 : CiphertextBlock = extract_ct_block(%0, %1);
        %41 : CiphertextBlock = extract_ct_block(%0, %2);
        %42 : CiphertextBlock = extract_ct_block(%0, %3);
        %43 : CiphertextBlock = extract_ct_block(%0, %4);
        %44 : CiphertextBlock = extract_ct_block(%0, %5);
        %45 : CiphertextBlock = extract_ct_block(%0, %6);
        %46 : CiphertextBlock = extract_ct_block(%0, %7);
        %48 : CiphertextBlock = extract_ct_block(%9, %1);
        %49 : CiphertextBlock = extract_ct_block(%9, %2);
        %50 : CiphertextBlock = extract_ct_block(%9, %3);
        %51 : CiphertextBlock = extract_ct_block(%9, %4);
        %52 : CiphertextBlock = extract_ct_block(%9, %5);
        %53 : CiphertextBlock = extract_ct_block(%9, %6);
        %54 : CiphertextBlock = extract_ct_block(%9, %7);
        %56 : CiphertextBlock = add_ct(%40, %48);
        %57 : CiphertextBlock = add_ct(%41, %49);
        %58 : CiphertextBlock = add_ct(%42, %50);
        %59 : CiphertextBlock = add_ct(%43, %51);
        %60 : CiphertextBlock = add_ct(%44, %52);
        %61 : CiphertextBlock = add_ct(%45, %53);
        %62 : CiphertextBlock = add_ct(%46, %54);
        %64 : CiphertextBlock, %65 : CiphertextBlock = pbs2(%56, %18);
        %66 : CiphertextBlock = pbs(%57, %26);
        %67 : CiphertextBlock = pbs(%58, %27);
        %68 : CiphertextBlock = pbs(%59, %28);
        %69 : CiphertextBlock = pbs(%60, %26);
        %70 : CiphertextBlock = pbs(%61, %27);
        %71 : CiphertextBlock = pbs(%62, %28);
        %73 : CiphertextBlock = add_ct(%66, %65);
        %74 : CiphertextBlock = add_ct(%70, %69);
        %75 : CiphertextBlock = add_ct(%57, %65);
        %76 : Ciphertext = store_ct_block(%64, %32, %1);
        %77 : CiphertextBlock = add_ct(%67, %73);
        %78 : CiphertextBlock = add_ct(%71, %74);
        %79 : CiphertextBlock = pbs(%73, %23);
        %80 : Ciphertext = store_ct_block(%75, %76, %2);
        %81 : CiphertextBlock = add_ct(%68, %77);
        %83 : CiphertextBlock = pbs(%77, %24);
        %84 : CiphertextBlock = add_ct(%58, %79);
        %85 : CiphertextBlock = pbs(%81, %25);
        %87 : CiphertextBlock = add_ct(%59, %83);
        %88 : Ciphertext = store_ct_block(%84, %80, %3);
        %90 : CiphertextBlock = add_ct(%69, %85);
        %91 : CiphertextBlock = add_ct(%74, %85);
        %92 : CiphertextBlock = add_ct(%78, %85);
        %93 : Ciphertext = store_ct_block(%87, %88, %4);
        %95 : CiphertextBlock = pbs(%90, %25);
        %96 : CiphertextBlock = pbs(%91, %23);
        %97 : CiphertextBlock = pbs(%92, %24);
        %99 : CiphertextBlock = add_ct(%60, %95);
        %100 : CiphertextBlock = add_ct(%61, %96);
        %101 : CiphertextBlock = add_ct(%62, %97);
        %102 : Ciphertext = store_ct_block(%99, %93, %5);
        %103 : Ciphertext = store_ct_block(%100, %102, %6);
        %104 : Ciphertext = store_ct_block(%101, %103, %7);
        output<0, Ciphertext>(%104);
    ",
    );
}

#[test]
fn test_sub_ir() {
    let ir = get_sub_ir(16, 2, 2);
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
        %9 : Ciphertext = input<1, Ciphertext>();
        %18 : PlaintextBlock = constant<3_pt_block>();
        %26 : Lut2 = gen_lut2<ManyCarryMsg>();
        %31 : Lut1 = gen_lut1<SolvePropGroupFinal0>();
        %32 : Lut1 = gen_lut1<SolvePropGroupFinal1>();
        %33 : Lut1 = gen_lut1<SolvePropGroupFinal2>();
        %34 : Lut1 = gen_lut1<ExtractPropGroup0>();
        %35 : Lut1 = gen_lut1<ExtractPropGroup1>();
        %36 : Lut1 = gen_lut1<ExtractPropGroup2>();
        %40 : Lut1 = gen_lut1<MsgOnly>();
        %41 : Ciphertext = let<Ciphertext>();
        %49 : CiphertextBlock = extract_ct_block(%0, %1);
        %50 : CiphertextBlock = extract_ct_block(%0, %2);
        %51 : CiphertextBlock = extract_ct_block(%0, %3);
        %52 : CiphertextBlock = extract_ct_block(%0, %4);
        %53 : CiphertextBlock = extract_ct_block(%0, %5);
        %54 : CiphertextBlock = extract_ct_block(%0, %6);
        %55 : CiphertextBlock = extract_ct_block(%0, %7);
        %57 : CiphertextBlock = extract_ct_block(%9, %1);
        %58 : CiphertextBlock = extract_ct_block(%9, %2);
        %59 : CiphertextBlock = extract_ct_block(%9, %3);
        %60 : CiphertextBlock = extract_ct_block(%9, %4);
        %61 : CiphertextBlock = extract_ct_block(%9, %5);
        %62 : CiphertextBlock = extract_ct_block(%9, %6);
        %63 : CiphertextBlock = extract_ct_block(%9, %7);
        %65 : CiphertextBlock = pt_sub(%18, %57);
        %66 : CiphertextBlock = pt_sub(%18, %58);
        %67 : CiphertextBlock = pt_sub(%18, %59);
        %68 : CiphertextBlock = pt_sub(%18, %60);
        %69 : CiphertextBlock = pt_sub(%18, %61);
        %70 : CiphertextBlock = pt_sub(%18, %62);
        %71 : CiphertextBlock = pt_sub(%18, %63);
        %73 : CiphertextBlock = add_ct(%49, %65);
        %74 : CiphertextBlock = add_ct(%50, %66);
        %75 : CiphertextBlock = add_ct(%51, %67);
        %76 : CiphertextBlock = add_ct(%52, %68);
        %77 : CiphertextBlock = add_ct(%53, %69);
        %78 : CiphertextBlock = add_ct(%54, %70);
        %79 : CiphertextBlock = add_ct(%55, %71);
        %81 : CiphertextBlock, %82 : CiphertextBlock = pbs2(%73, %26);
        %83 : CiphertextBlock = pbs(%74, %34);
        %84 : CiphertextBlock = pbs(%75, %35);
        %85 : CiphertextBlock = pbs(%76, %36);
        %86 : CiphertextBlock = pbs(%77, %34);
        %87 : CiphertextBlock = pbs(%78, %35);
        %88 : CiphertextBlock = pbs(%79, %36);
        %90 : CiphertextBlock = add_ct(%83, %82);
        %91 : CiphertextBlock = add_ct(%87, %86);
        %92 : CiphertextBlock = add_ct(%74, %82);
        %93 : CiphertextBlock = pbs(%81, %40);
        %94 : CiphertextBlock = add_ct(%84, %90);
        %95 : CiphertextBlock = add_ct(%88, %91);
        %96 : CiphertextBlock = pbs(%90, %31);
        %97 : CiphertextBlock = pbs(%92, %40);
        %98 : Ciphertext = store_ct_block(%93, %41, %1);
        %99 : CiphertextBlock = add_ct(%85, %94);
        %101 : CiphertextBlock = pbs(%94, %32);
        %102 : CiphertextBlock = add_ct(%75, %96);
        %103 : Ciphertext = store_ct_block(%97, %98, %2);
        %104 : CiphertextBlock = pbs(%99, %33);
        %106 : CiphertextBlock = add_ct(%76, %101);
        %107 : CiphertextBlock = pbs(%102, %40);
        %109 : CiphertextBlock = add_ct(%86, %104);
        %110 : CiphertextBlock = add_ct(%91, %104);
        %111 : CiphertextBlock = add_ct(%95, %104);
        %112 : CiphertextBlock = pbs(%106, %40);
        %113 : Ciphertext = store_ct_block(%107, %103, %3);
        %115 : CiphertextBlock = pbs(%109, %33);
        %116 : CiphertextBlock = pbs(%110, %31);
        %117 : CiphertextBlock = pbs(%111, %32);
        %118 : Ciphertext = store_ct_block(%112, %113, %4);
        %120 : CiphertextBlock = add_ct(%77, %115);
        %121 : CiphertextBlock = add_ct(%78, %116);
        %122 : CiphertextBlock = add_ct(%79, %117);
        %123 : CiphertextBlock = pbs(%120, %40);
        %124 : CiphertextBlock = pbs(%121, %40);
        %125 : CiphertextBlock = pbs(%122, %40);
        %126 : Ciphertext = store_ct_block(%123, %118, %5);
        %127 : Ciphertext = store_ct_block(%124, %126, %6);
        %128 : Ciphertext = store_ct_block(%125, %127, %7);
        output<0, Ciphertext>(%128);
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
