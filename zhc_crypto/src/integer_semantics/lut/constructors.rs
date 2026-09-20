//! Named constructors for builtin lookup tables.

use super::{Lut1, Lut2, builtin::*};
use crate::integer_semantics::CiphertextBlockSpec;

impl Lut1 {
    /// Precomputes the `None` table for the given block specification.
    pub fn none(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("None", spec, None_0)
    }

    /// Precomputes the `MsgOnly` table for the given block specification.
    pub fn msg_only(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MsgOnly", spec, MsgOnly_0)
    }

    /// Precomputes the `CarryOnly` table for the given block specification.
    pub fn carry_only(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CarryOnly", spec, CarryOnly_0)
    }

    /// Precomputes the `CarryInMsg` table for the given block specification.
    pub fn carry_in_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CarryInMsg", spec, CarryInMsg_0)
    }

    /// Precomputes the `MultCarryMsg` table for the given block specification.
    pub fn mult_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MultCarryMsg", spec, MultCarryMsg_0)
    }

    /// Precomputes the `MultCarryMsgLsb` table for the given block specification.
    pub fn mult_carry_msg_lsb(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MultCarryMsgLsb", spec, MultCarryMsgLsb_0)
    }

    /// Precomputes the `MultCarryMsgMsb` table for the given block specification.
    pub fn mult_carry_msg_msb(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MultCarryMsgMsb", spec, MultCarryMsgMsb_0)
    }

    /// Precomputes the `BwAnd` table for the given block specification.
    pub fn bw_and(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("BwAnd", spec, BwAnd_0)
    }

    /// Precomputes the `BwOr` table for the given block specification.
    pub fn bw_or(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("BwOr", spec, BwOr_0)
    }

    /// Precomputes the `BwXor` table for the given block specification.
    pub fn bw_xor(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("BwXor", spec, BwXor_0)
    }

    /// Precomputes the `CmpSign` table for the given block specification.
    pub fn cmp_sign(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpSign", spec, CmpSign_0)
    }

    /// Precomputes the `CmpReduce` table for the given block specification.
    pub fn cmp_reduce(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpReduce", spec, CmpReduce_0)
    }

    /// Precomputes the `CmpGt` table for the given block specification.
    pub fn cmp_gt(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpGt", spec, CmpGt_0)
    }

    /// Precomputes the `CmpGte` table for the given block specification.
    pub fn cmp_gte(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpGte", spec, CmpGte_0)
    }

    /// Precomputes the `CmpLt` table for the given block specification.
    pub fn cmp_lt(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpLt", spec, CmpLt_0)
    }

    /// Precomputes the `CmpLte` table for the given block specification.
    pub fn cmp_lte(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpLte", spec, CmpLte_0)
    }

    /// Precomputes the `CmpEq` table for the given block specification.
    pub fn cmp_eq(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpEq", spec, CmpEq_0)
    }

    /// Precomputes the `CmpNeq` table for the given block specification.
    pub fn cmp_neq(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpNeq", spec, CmpNeq_0)
    }

    /// Precomputes the `ReduceCarry2` table for the given block specification.
    pub fn reduce_carry2(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ReduceCarry2", spec, ReduceCarry2_0)
    }

    /// Precomputes the `ReduceCarry3` table for the given block specification.
    pub fn reduce_carry3(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ReduceCarry3", spec, ReduceCarry3_0)
    }

    /// Precomputes the `ReduceCarryPad` table for the given block specification.
    pub fn reduce_carry_pad(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ReduceCarryPad", spec, ReduceCarryPad_0)
    }

    /// Precomputes the `GenPropAdd` table for the given block specification.
    pub fn gen_prop_add(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("GenPropAdd", spec, GenPropAdd_0)
    }

    /// Precomputes the `IfTrueZeroed` table for the given block specification.
    pub fn if_true_zeroed(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IfTrueZeroed", spec, IfTrueZeroed_0)
    }

    /// Precomputes the `IfFalseZeroed` table for the given block specification.
    pub fn if_false_zeroed(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IfFalseZeroed", spec, IfFalseZeroed_0)
    }

    /// Precomputes the `Ripple2GenProp` table for the given block specification.
    pub fn ripple2_gen_prop(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("Ripple2GenProp", spec, Ripple2GenProp_0)
    }

    /// Precomputes the `CmpGtMrg` table for the given block specification.
    pub fn cmp_gt_mrg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpGtMrg", spec, CmpGtMrg_0)
    }

    /// Precomputes the `CmpGteMrg` table for the given block specification.
    pub fn cmp_gte_mrg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpGteMrg", spec, CmpGteMrg_0)
    }

    /// Precomputes the `CmpLtMrg` table for the given block specification.
    pub fn cmp_lt_mrg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpLtMrg", spec, CmpLtMrg_0)
    }

    /// Precomputes the `CmpLteMrg` table for the given block specification.
    pub fn cmp_lte_mrg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpLteMrg", spec, CmpLteMrg_0)
    }

    /// Precomputes the `CmpEqMrg` table for the given block specification.
    pub fn cmp_eq_mrg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpEqMrg", spec, CmpEqMrg_0)
    }

    /// Precomputes the `CmpNeqMrg` table for the given block specification.
    pub fn cmp_neq_mrg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CmpNeqMrg", spec, CmpNeqMrg_0)
    }

    /// Precomputes the `IsSome` table for the given block specification.
    pub fn is_some(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IsSome", spec, IsSome_0)
    }

    /// Precomputes the `CarryIsSome` table for the given block specification.
    pub fn carry_is_some(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CarryIsSome", spec, CarryIsSome_0)
    }

    /// Precomputes the `CarryIsNone` table for the given block specification.
    pub fn carry_is_none(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("CarryIsNone", spec, CarryIsNone_0)
    }

    /// Precomputes the `MultCarryMsgIsSome` table for the given block specification.
    pub fn mult_carry_msg_is_some(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MultCarryMsgIsSome", spec, MultCarryMsgIsSome_0)
    }

    /// Precomputes the `MultCarryMsgMsbIsSome` table for the given block specification.
    pub fn mult_carry_msg_msb_is_some(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MultCarryMsgMsbIsSome", spec, MultCarryMsgMsbIsSome_0)
    }

    /// Precomputes the `IsNull` table for the given block specification.
    pub fn is_null(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IsNull", spec, IsNull_0)
    }

    /// Precomputes the `IsNullPos1` table for the given block specification.
    pub fn is_null_pos1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IsNullPos1", spec, IsNullPos1_0)
    }

    /// Precomputes the `NotNull` table for the given block specification.
    pub fn not_null(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("NotNull", spec, NotNull_0)
    }

    /// Precomputes the `MsgNotNull` table for the given block specification.
    pub fn msg_not_null(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MsgNotNull", spec, MsgNotNull_0)
    }

    /// Precomputes the `MsgNotNullPos1` table for the given block specification.
    pub fn msg_not_null_pos1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("MsgNotNullPos1", spec, MsgNotNullPos1_0)
    }

    /// Precomputes the `SolvePropGroupFinal0` table for the given block specification.
    pub fn solve_prop_group_final0(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolvePropGroupFinal0", spec, SolvePropGroupFinal0_0)
    }

    /// Precomputes the `SolvePropGroupFinal1` table for the given block specification.
    pub fn solve_prop_group_final1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolvePropGroupFinal1", spec, SolvePropGroupFinal1_0)
    }

    /// Precomputes the `SolvePropGroupFinal2` table for the given block specification.
    pub fn solve_prop_group_final2(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolvePropGroupFinal2", spec, SolvePropGroupFinal2_0)
    }

    /// Precomputes the `ExtractPropGroup0` table for the given block specification.
    pub fn extract_prop_group0(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ExtractPropGroup0", spec, ExtractPropGroup0_0)
    }

    /// Precomputes the `ExtractPropGroup1` table for the given block specification.
    pub fn extract_prop_group1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ExtractPropGroup1", spec, ExtractPropGroup1_0)
    }

    /// Precomputes the `ExtractPropGroup2` table for the given block specification.
    pub fn extract_prop_group2(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ExtractPropGroup2", spec, ExtractPropGroup2_0)
    }

    /// Precomputes the `ExtractPropGroup3` table for the given block specification.
    pub fn extract_prop_group3(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ExtractPropGroup3", spec, ExtractPropGroup3_0)
    }

    /// Precomputes the `SolveProp` table for the given block specification.
    pub fn solve_prop(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolveProp", spec, SolveProp_0)
    }

    /// Precomputes the `SolvePropCarry` table for the given block specification.
    pub fn solve_prop_carry(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolvePropCarry", spec, SolvePropCarry_0)
    }

    /// Precomputes the `SolveQuotient` table for the given block specification.
    pub fn solve_quotient(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolveQuotient", spec, SolveQuotient_0)
    }

    /// Precomputes the `SolveQuotientPos1` table for the given block specification.
    pub fn solve_quotient_pos1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("SolveQuotientPos1", spec, SolveQuotientPos1_0)
    }

    /// Precomputes the `IfPos1FalseZeroed` table for the given block specification.
    pub fn if_pos1_false_zeroed(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IfPos1FalseZeroed", spec, IfPos1FalseZeroed_0)
    }

    /// Precomputes the `IfPos1FalseZeroedMsgCarry1` table for the given block specification.
    pub fn if_pos1_false_zeroed_msg_carry1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "IfPos1FalseZeroedMsgCarry1",
            spec,
            IfPos1FalseZeroedMsgCarry1_0,
        )
    }

    /// Precomputes the `ShiftLeftByCarryPos0Msg` table for the given block specification.
    pub fn shift_left_by_carry_pos0_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ShiftLeftByCarryPos0Msg", spec, ShiftLeftByCarryPos0Msg_0)
    }

    /// Precomputes the `ShiftLeftByCarryPos0MsgNext` table for the given block specification.
    pub fn shift_left_by_carry_pos0_msg_next(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ShiftLeftByCarryPos0MsgNext",
            spec,
            ShiftLeftByCarryPos0MsgNext_0,
        )
    }

    /// Precomputes the `ShiftRightByCarryPos0Msg` table for the given block specification.
    pub fn shift_right_by_carry_pos0_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ShiftRightByCarryPos0Msg", spec, ShiftRightByCarryPos0Msg_0)
    }

    /// Precomputes the `ShiftRightByCarryPos0MsgNext` table for the given block specification.
    pub fn shift_right_by_carry_pos0_msg_next(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ShiftRightByCarryPos0MsgNext",
            spec,
            ShiftRightByCarryPos0MsgNext_0,
        )
    }

    /// Precomputes the `IfPos0TrueZeroed` table for the given block specification.
    pub fn if_pos0_true_zeroed(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IfPos0TrueZeroed", spec, IfPos0TrueZeroed_0)
    }

    /// Precomputes the `IfPos0FalseZeroed` table for the given block specification.
    pub fn if_pos0_false_zeroed(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IfPos0FalseZeroed", spec, IfPos0FalseZeroed_0)
    }

    /// Precomputes the `IfPos1TrueZeroed` table for the given block specification.
    pub fn if_pos1_true_zeroed(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("IfPos1TrueZeroed", spec, IfPos1TrueZeroed_0)
    }
}

impl Lut2 {
    /// Precomputes the `ManyGenProp` table for the given block specification.
    pub fn many_gen_prop(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ManyGenProp", spec, ManyGenProp_0, ManyGenProp_1)
    }

    /// Precomputes the `ManyCarryMsg` table for the given block specification.
    pub fn many_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ManyCarryMsg", spec, ManyCarryMsg_0, ManyCarryMsg_1)
    }

    /// Precomputes the `ManyMsgSplitShift1` table for the given block specification.
    pub fn many_msg_split_shift1(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyMsgSplitShift1",
            spec,
            ManyMsgSplitShift1_0,
            ManyMsgSplitShift1_1,
        )
    }

    /// Precomputes the `ManyInv1CarryMsg` table for the given block specification.
    pub fn many_inv1_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv1CarryMsg",
            spec,
            ManyInv1CarryMsg_0,
            ManyInv1CarryMsg_1,
        )
    }

    /// Precomputes the `ManyInv2CarryMsg` table for the given block specification.
    pub fn many_inv2_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv2CarryMsg",
            spec,
            ManyInv2CarryMsg_0,
            ManyInv2CarryMsg_1,
        )
    }

    /// Precomputes the `ManyInv3CarryMsg` table for the given block specification.
    pub fn many_inv3_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv3CarryMsg",
            spec,
            ManyInv3CarryMsg_0,
            ManyInv3CarryMsg_1,
        )
    }

    /// Precomputes the `ManyInv4CarryMsg` table for the given block specification.
    pub fn many_inv4_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv4CarryMsg",
            spec,
            ManyInv4CarryMsg_0,
            ManyInv4CarryMsg_1,
        )
    }

    /// Precomputes the `ManyInv5CarryMsg` table for the given block specification.
    pub fn many_inv5_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv5CarryMsg",
            spec,
            ManyInv5CarryMsg_0,
            ManyInv5CarryMsg_1,
        )
    }

    /// Precomputes the `ManyInv6CarryMsg` table for the given block specification.
    pub fn many_inv6_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv6CarryMsg",
            spec,
            ManyInv6CarryMsg_0,
            ManyInv6CarryMsg_1,
        )
    }

    /// Precomputes the `ManyInv7CarryMsg` table for the given block specification.
    pub fn many_inv7_carry_msg(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "ManyInv7CarryMsg",
            spec,
            ManyInv7CarryMsg_0,
            ManyInv7CarryMsg_1,
        )
    }

    /// Precomputes the `ManyMsgSplit` table for the given block specification.
    pub fn many_msg_split(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn("ManyMsgSplit", spec, ManyMsgSplit_0, ManyMsgSplit_1)
    }

    /// Precomputes the `Manym2lPropBit1MsgSplit` table for the given block specification.
    pub fn manym2l_prop_bit1_msg_split(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "Manym2lPropBit1MsgSplit",
            spec,
            Manym2lPropBit1MsgSplit_0,
            Manym2lPropBit1MsgSplit_1,
        )
    }

    /// Precomputes the `Manym2lPropBit0MsgSplit` table for the given block specification.
    pub fn manym2l_prop_bit0_msg_split(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "Manym2lPropBit0MsgSplit",
            spec,
            Manym2lPropBit0MsgSplit_0,
            Manym2lPropBit0MsgSplit_1,
        )
    }

    /// Precomputes the `Manyl2mPropBit1MsgSplit` table for the given block specification.
    pub fn manyl2m_prop_bit1_msg_split(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "Manyl2mPropBit1MsgSplit",
            spec,
            Manyl2mPropBit1MsgSplit_0,
            Manyl2mPropBit1MsgSplit_1,
        )
    }

    /// Precomputes the `Manyl2mPropBit0MsgSplit` table for the given block specification.
    pub fn manyl2m_prop_bit0_msg_split(spec: CiphertextBlockSpec) -> Self {
        Self::from_fn(
            "Manyl2mPropBit0MsgSplit",
            spec,
            Manyl2mPropBit0MsgSplit_0,
            Manyl2mPropBit0MsgSplit_1,
        )
    }
}
