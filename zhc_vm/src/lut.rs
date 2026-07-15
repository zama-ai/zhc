use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec,
    lut::{LookupCheck, Lut1, Lut2},
};
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::SafeAs;

use crate::params::VMParams;

pub const N_LUTS: usize = 76;

enum LutDef {
    One(Lut1Def),
    Two(Lut2Def),
}

impl LutDef {
    fn lut_nb(&self) -> usize {
        match self {
            LutDef::One(_) => 1,
            LutDef::Two(_) => 2,
        }
    }
}

fn lut_defs() -> [LutDef; N_LUTS] {
    use Lut1Def::*;
    use Lut2Def::*;
    [
        LutDef::One(None),
        LutDef::One(MsgOnly),
        LutDef::One(CarryOnly),
        LutDef::One(CarryInMsg),
        LutDef::One(MultCarryMsg),
        LutDef::One(MultCarryMsgLsb),
        LutDef::One(MultCarryMsgMsb),
        LutDef::One(BwAnd),
        LutDef::One(BwOr),
        LutDef::One(BwXor),
        LutDef::One(CmpSign),
        LutDef::One(CmpReduce),
        LutDef::One(CmpGt),
        LutDef::One(CmpGte),
        LutDef::One(CmpLt),
        LutDef::One(CmpLte),
        LutDef::One(CmpEq),
        LutDef::One(CmpNeq),
        LutDef::Two(ManyGenProp),
        LutDef::One(ReduceCarry2),
        LutDef::One(ReduceCarry3),
        LutDef::One(ReduceCarryPad),
        LutDef::One(GenPropAdd),
        LutDef::One(IfTrueZeroed),
        LutDef::One(IfFalseZeroed),
        LutDef::One(Ripple2GenProp),
        LutDef::Two(ManyCarryMsg),
        LutDef::One(CmpGtMrg),
        LutDef::One(CmpGteMrg),
        LutDef::One(CmpLtMrg),
        LutDef::One(CmpLteMrg),
        LutDef::One(CmpEqMrg),
        LutDef::One(CmpNeqMrg),
        LutDef::One(IsSome),
        LutDef::One(CarryIsSome),
        LutDef::One(CarryIsNone),
        LutDef::One(MultCarryMsgIsSome),
        LutDef::One(MultCarryMsgMsbIsSome),
        LutDef::One(IsNull),
        LutDef::One(IsNullPos1),
        LutDef::One(NotNull),
        LutDef::One(MsgNotNull),
        LutDef::One(MsgNotNullPos1),
        LutDef::Two(ManyMsgSplitShift1),
        LutDef::One(SolvePropGroupFinal0),
        LutDef::One(SolvePropGroupFinal1),
        LutDef::One(SolvePropGroupFinal2),
        LutDef::One(ExtractPropGroup0),
        LutDef::One(ExtractPropGroup1),
        LutDef::One(ExtractPropGroup2),
        LutDef::One(ExtractPropGroup3),
        LutDef::One(SolveProp),
        LutDef::One(SolvePropCarry),
        LutDef::One(SolveQuotient),
        LutDef::One(SolveQuotientPos1),
        LutDef::One(IfPos1FalseZeroed),
        LutDef::One(IfPos1FalseZeroedMsgCarry1),
        LutDef::One(ShiftLeftByCarryPos0Msg),
        LutDef::One(ShiftLeftByCarryPos0MsgNext),
        LutDef::One(ShiftRightByCarryPos0Msg),
        LutDef::One(ShiftRightByCarryPos0MsgNext),
        LutDef::One(IfPos0TrueZeroed),
        LutDef::One(IfPos0FalseZeroed),
        LutDef::One(IfPos1TrueZeroed),
        LutDef::Two(ManyInv1CarryMsg),
        LutDef::Two(ManyInv2CarryMsg),
        LutDef::Two(ManyInv3CarryMsg),
        LutDef::Two(ManyInv4CarryMsg),
        LutDef::Two(ManyInv5CarryMsg),
        LutDef::Two(ManyInv6CarryMsg),
        LutDef::Two(ManyInv7CarryMsg),
        LutDef::Two(ManyMsgSplit),
        LutDef::Two(Manym2lPropBit1MsgSplit),
        LutDef::Two(Manym2lPropBit0MsgSplit),
        LutDef::Two(Manyl2mPropBit1MsgSplit),
        LutDef::Two(Manyl2mPropBit0MsgSplit),
    ]
}

pub fn build_registry(params: &VMParams, registry: &mut [u64]) {
    let slot = params.lut_alloc_size();
    assert_eq!(registry.len(), N_LUTS * slot);
    for (def, chunk) in lut_defs().iter().zip(registry.chunks_exact_mut(slot)) {
        build_accumulator(params, def, chunk);
    }
}

fn build_accumulator(params: &VMParams, def: &LutDef, out: &mut [u64]) {
    out.fill(0);

    let n = params.bsk_polynomial_size;
    let body = &mut out[params.bsk_glwe_dim * n..];

    let spec = CiphertextBlockSpec(params.carry_size.sas(), params.message_size.sas());
    let modulus_sup = 1usize << spec.data_size();
    let box_size = n / modulus_sup;

    let lut_nb = def.lut_nb();
    let boxes_per_chunk = modulus_sup / lut_nb;
    let chunk = n / lut_nb;

    let encode = |v: u64| v.wrapping_mul(params.delta.sas::<u64>());

    let (lut1, lut2) = match def {
        LutDef::One(d) => (Some(d.into_lut(spec)), Option::<Lut2>::None),
        LutDef::Two(d) => (Option::<Lut1>::None, Some(d.into_lut(spec))),
    };

    for (j, chunk_buf) in body.chunks_exact_mut(chunk).enumerate() {
        for (v, sub_lut_box) in chunk_buf
            .chunks_exact_mut(box_size)
            .enumerate()
            .take(boxes_per_chunk)
        {
            let inp = spec.from_data(v.sas());
            let out_val = match (&lut1, &lut2) {
                (Some(lut1), Option::None) => lut1.lookup(inp, LookupCheck::AllowOutputPadding),
                (Option::None, Some(lut2)) => {
                    let (o1, o2) = lut2.lookup(inp, LookupCheck::AllowOutputPadding);
                    if j == 0 { o1 } else { o2 }
                }
                _ => unreachable!(),
            };
            sub_lut_box.fill(encode(out_val.raw_complete_bits().sas()));
        }
    }

    let half_box = box_size / 2;
    for c in body[..half_box].iter_mut() {
        *c = c.wrapping_neg();
    }
    body.rotate_left(half_box);
}
