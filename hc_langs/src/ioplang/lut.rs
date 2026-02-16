use hc_crypto::integer_semantics::{
    EmulatedCiphertextBlock,
    lut::{protect_lookup, wrapping_lookup},
};

/// Enumeration of all available LUT1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lut1Def {
    None,
    MsgOnly,
    CarryOnly,
    CarryInMsg,
    MultCarryMsg,
    MultCarryMsgLsb,
    MultCarryMsgMsb,
    BwAnd,
    BwOr,
    BwXor,
    CmpSign,
    CmpReduce,
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpEq,
    CmpNeq,
    ReduceCarry2,
    ReduceCarry3,
    ReduceCarryPad,
    GenPropAdd,
    IfTrueZeroed,
    IfFalseZeroed,
    Ripple2GenProp,
    CmpGtMrg,
    CmpGteMrg,
    CmpLtMrg,
    CmpLteMrg,
    CmpEqMrg,
    CmpNeqMrg,
    IsSome,
    CarryIsSome,
    CarryIsNone,
    MultCarryMsgIsSome,
    MultCarryMsgMsbIsSome,
    IsNull,
    IsNullPos1,
    NotNull,
    MsgNotNull,
    MsgNotNullPos1,
    SolvePropGroupFinal0,
    SolvePropGroupFinal1,
    SolvePropGroupFinal2,
    ExtractPropGroup0,
    ExtractPropGroup1,
    ExtractPropGroup2,
    ExtractPropGroup3,
    SolveProp,
    SolvePropCarry,
    SolveQuotient,
    SolveQuotientPos1,
    IfPos1FalseZeroed,
    IfPos1FalseZeroedMsgCarry1,
    ShiftLeftByCarryPos0Msg,
    ShiftLeftByCarryPos0MsgNext,
    ShiftRightByCarryPos0Msg,
    ShiftRightByCarryPos0MsgNext,
    IfPos0TrueZeroed,
    IfPos0FalseZeroed,
    IfPos1TrueZeroed,
}

impl Lut1Def {
    pub(crate) fn protect_lookup(&self, inp: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
        protect_lookup(self.get_fn(), inp)
    }

    pub(crate) fn wrapping_lookup(&self, inp: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
        wrapping_lookup(self.get_fn(), inp)
    }

    fn get_fn(&self) -> fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
        use hc_crypto::integer_semantics::lut::*;
        match self {
            Lut1Def::None => None_0,
            Lut1Def::MsgOnly => MsgOnly_0,
            Lut1Def::CarryOnly => CarryOnly_0,
            Lut1Def::CarryInMsg => CarryInMsg_0,
            Lut1Def::MultCarryMsg => MultCarryMsg_0,
            Lut1Def::MultCarryMsgLsb => MultCarryMsgLsb_0,
            Lut1Def::MultCarryMsgMsb => MultCarryMsgMsb_0,
            Lut1Def::BwAnd => BwAnd_0,
            Lut1Def::BwOr => BwOr_0,
            Lut1Def::BwXor => BwXor_0,
            Lut1Def::CmpSign => CmpSign_0,
            Lut1Def::CmpReduce => CmpReduce_0,
            Lut1Def::CmpGt => CmpGt_0,
            Lut1Def::CmpGte => CmpGte_0,
            Lut1Def::CmpLt => CmpLt_0,
            Lut1Def::CmpLte => CmpLte_0,
            Lut1Def::CmpEq => CmpEq_0,
            Lut1Def::CmpNeq => CmpNeq_0,
            Lut1Def::ReduceCarry2 => ReduceCarry2_0,
            Lut1Def::ReduceCarry3 => ReduceCarry3_0,
            Lut1Def::ReduceCarryPad => ReduceCarryPad_0,
            Lut1Def::GenPropAdd => GenPropAdd_0,
            Lut1Def::IfTrueZeroed => IfTrueZeroed_0,
            Lut1Def::IfFalseZeroed => IfFalseZeroed_0,
            Lut1Def::Ripple2GenProp => Ripple2GenProp_0,
            Lut1Def::CmpGtMrg => CmpGtMrg_0,
            Lut1Def::CmpGteMrg => CmpGteMrg_0,
            Lut1Def::CmpLtMrg => CmpLtMrg_0,
            Lut1Def::CmpLteMrg => CmpLteMrg_0,
            Lut1Def::CmpEqMrg => CmpEqMrg_0,
            Lut1Def::CmpNeqMrg => CmpNeqMrg_0,
            Lut1Def::IsSome => IsSome_0,
            Lut1Def::CarryIsSome => CarryIsSome_0,
            Lut1Def::CarryIsNone => CarryIsNone_0,
            Lut1Def::MultCarryMsgIsSome => MultCarryMsgIsSome_0,
            Lut1Def::MultCarryMsgMsbIsSome => MultCarryMsgMsbIsSome_0,
            Lut1Def::IsNull => IsNull_0,
            Lut1Def::IsNullPos1 => IsNullPos1_0,
            Lut1Def::NotNull => NotNull_0,
            Lut1Def::MsgNotNull => MsgNotNull_0,
            Lut1Def::MsgNotNullPos1 => MsgNotNullPos1_0,
            Lut1Def::SolvePropGroupFinal0 => SolvePropGroupFinal0_0,
            Lut1Def::SolvePropGroupFinal1 => SolvePropGroupFinal1_0,
            Lut1Def::SolvePropGroupFinal2 => SolvePropGroupFinal2_0,
            Lut1Def::ExtractPropGroup0 => ExtractPropGroup0_0,
            Lut1Def::ExtractPropGroup1 => ExtractPropGroup1_0,
            Lut1Def::ExtractPropGroup2 => ExtractPropGroup2_0,
            Lut1Def::ExtractPropGroup3 => ExtractPropGroup3_0,
            Lut1Def::SolveProp => SolveProp_0,
            Lut1Def::SolvePropCarry => SolvePropCarry_0,
            Lut1Def::SolveQuotient => SolveQuotient_0,
            Lut1Def::SolveQuotientPos1 => SolveQuotientPos1_0,
            Lut1Def::IfPos1FalseZeroed => IfPos1FalseZeroed_0,
            Lut1Def::IfPos1FalseZeroedMsgCarry1 => IfPos1FalseZeroedMsgCarry1_0,
            Lut1Def::ShiftLeftByCarryPos0Msg => ShiftLeftByCarryPos0Msg_0,
            Lut1Def::ShiftLeftByCarryPos0MsgNext => ShiftLeftByCarryPos0MsgNext_0,
            Lut1Def::ShiftRightByCarryPos0Msg => ShiftRightByCarryPos0Msg_0,
            Lut1Def::ShiftRightByCarryPos0MsgNext => ShiftRightByCarryPos0MsgNext_0,
            Lut1Def::IfPos0TrueZeroed => IfPos0TrueZeroed_0,
            Lut1Def::IfPos0FalseZeroed => IfPos0FalseZeroed_0,
            Lut1Def::IfPos1TrueZeroed => IfPos1TrueZeroed_0,
        }
    }
}

/// Enumeration of all available LUT2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lut2Def {
    ManyGenProp,
    ManyCarryMsg,
    ManyMsgSplitShift1,
    ManyInv1CarryMsg,
    ManyInv2CarryMsg,
    ManyInv3CarryMsg,
    ManyInv4CarryMsg,
    ManyInv5CarryMsg,
    ManyInv6CarryMsg,
    ManyInv7CarryMsg,
    ManyMsgSplit,
    Manym2lPropBit1MsgSplit,
    Manym2lPropBit0MsgSplit,
    Manyl2mPropBit1MsgSplit,
    Manyl2mPropBit0MsgSplit,
}

impl Lut2Def {
    pub(crate) fn protect_lookup(
        &self,
        inp: EmulatedCiphertextBlock,
    ) -> (EmulatedCiphertextBlock, EmulatedCiphertextBlock) {
        let (f0, f1) = self.get_fns();
        (protect_lookup(f0, inp), protect_lookup(f1, inp))
    }

    pub(crate) fn wrapping_lookup(
        &self,
        inp: EmulatedCiphertextBlock,
    ) -> (EmulatedCiphertextBlock, EmulatedCiphertextBlock) {
        let (f0, f1) = self.get_fns();
        (wrapping_lookup(f0, inp), wrapping_lookup(f1, inp))
    }

    fn get_fns(
        &self,
    ) -> (
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) {
        use hc_crypto::integer_semantics::lut::*;
        match self {
            Lut2Def::ManyGenProp => (ManyGenProp_0, ManyGenProp_1),
            Lut2Def::ManyCarryMsg => (ManyCarryMsg_0, ManyCarryMsg_1),
            Lut2Def::ManyMsgSplitShift1 => (ManyMsgSplitShift1_0, ManyMsgSplitShift1_1),
            Lut2Def::ManyInv1CarryMsg => (ManyInv1CarryMsg_0, ManyInv1CarryMsg_1),
            Lut2Def::ManyInv2CarryMsg => (ManyInv2CarryMsg_0, ManyInv2CarryMsg_1),
            Lut2Def::ManyInv3CarryMsg => (ManyInv3CarryMsg_0, ManyInv3CarryMsg_1),
            Lut2Def::ManyInv4CarryMsg => (ManyInv4CarryMsg_0, ManyInv4CarryMsg_1),
            Lut2Def::ManyInv5CarryMsg => (ManyInv5CarryMsg_0, ManyInv5CarryMsg_1),
            Lut2Def::ManyInv6CarryMsg => (ManyInv6CarryMsg_0, ManyInv6CarryMsg_1),
            Lut2Def::ManyInv7CarryMsg => (ManyInv7CarryMsg_0, ManyInv7CarryMsg_1),
            Lut2Def::ManyMsgSplit => (ManyMsgSplit_0, ManyMsgSplit_1),
            Lut2Def::Manym2lPropBit1MsgSplit => {
                (Manym2lPropBit1MsgSplit_0, Manym2lPropBit1MsgSplit_1)
            }
            Lut2Def::Manym2lPropBit0MsgSplit => {
                (Manym2lPropBit0MsgSplit_0, Manym2lPropBit0MsgSplit_1)
            }
            Lut2Def::Manyl2mPropBit1MsgSplit => {
                (Manyl2mPropBit1MsgSplit_0, Manyl2mPropBit1MsgSplit_1)
            }
            Lut2Def::Manyl2mPropBit0MsgSplit => {
                (Manyl2mPropBit0MsgSplit_0, Manyl2mPropBit0MsgSplit_1)
            }
        }
    }
}

/// Enumeration of all available LUT4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lut4Def {}

impl Lut4Def {
    pub(crate) fn protect_lookup(
        &self,
        inp: EmulatedCiphertextBlock,
    ) -> (
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
    ) {
        let (f0, f1, f2, f3) = self.get_fns();
        (
            protect_lookup(f0, inp),
            protect_lookup(f1, inp),
            protect_lookup(f2, inp),
            protect_lookup(f3, inp),
        )
    }

    pub(crate) fn wrapping_lookup(
        &self,
        inp: EmulatedCiphertextBlock,
    ) -> (
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
    ) {
        let (f0, f1, f2, f3) = self.get_fns();
        (
            wrapping_lookup(f0, inp),
            wrapping_lookup(f1, inp),
            wrapping_lookup(f2, inp),
            wrapping_lookup(f3, inp),
        )
    }

    fn get_fns(
        &self,
    ) -> (
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) {
        unreachable!()
    }
}

/// Enumeration of all available LUT8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lut8Def {}

impl Lut8Def {
    pub(crate) fn protect_lookup(
        &self,
        inp: EmulatedCiphertextBlock,
    ) -> (
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
    ) {
        let (f0, f1, f2, f3, f4, f5, f6, f7) = self.get_fns();
        (
            protect_lookup(f0, inp),
            protect_lookup(f1, inp),
            protect_lookup(f2, inp),
            protect_lookup(f3, inp),
            protect_lookup(f4, inp),
            protect_lookup(f5, inp),
            protect_lookup(f6, inp),
            protect_lookup(f7, inp),
        )
    }

    pub(crate) fn wrapping_lookup(
        &self,
        inp: EmulatedCiphertextBlock,
    ) -> (
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
        EmulatedCiphertextBlock,
    ) {
        let (f0, f1, f2, f3, f4, f5, f6, f7) = self.get_fns();
        (
            wrapping_lookup(f0, inp),
            wrapping_lookup(f1, inp),
            wrapping_lookup(f2, inp),
            wrapping_lookup(f3, inp),
            wrapping_lookup(f4, inp),
            wrapping_lookup(f5, inp),
            wrapping_lookup(f6, inp),
            wrapping_lookup(f7, inp),
        )
    }

    fn get_fns(
        &self,
    ) -> (
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
        fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
    ) {
        unreachable!()
    }
}
