use super::super::{
    CiphertextBlockSpec, EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage,
    lut::{
        legacy::{self, DigitParameters},
        lut,
    },
};

/// Generic comparison function between legacy and lut implementations
fn compare_legacy_vs_lut<F, G>(legacy_fn: F, lut_fn: G, spec: CiphertextBlockSpec)
where
    F: Fn(&DigitParameters, usize) -> usize,
    G: Fn(EmulatedCiphertextBlock) -> EmulatedCiphertextBlock,
{
    // Generate test blocks for the given spec
    let max_storage = (1 << spec.complete_size()) - 1;
    for storage in 0..=max_storage {
        let block = EmulatedCiphertextBlock { storage, spec };
        let (params, val) = {
            let params = {
                let spec = block.spec;
                DigitParameters {
                    msg_w: spec.message_size() as usize,
                    carry_w: spec.carry_size() as usize,
                }
            };
            (params, block.storage as usize)
        };

        // Call legacy function
        let legacy_output = legacy_fn(&params, val);
        let legacy_result = {
            let spec = block.spec;
            EmulatedCiphertextBlock {
                storage: legacy_output as EmulatedCiphertextBlockStorage,
                spec,
            }
        };

        // Call lut function
        let lut_result = lut_fn(block);

        // Compare results
        assert_eq!(
            legacy_result, lut_result,
            "Mismatch for input block {:#?}: legacy={:#?}, lut={:#?}",
            block, legacy_result, lut_result
        );
    }
}

#[test]
fn test_none_0_equivalence() {
    compare_legacy_vs_lut(legacy::None_0, lut::None_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_msgonly_0_equivalence() {
    compare_legacy_vs_lut(legacy::MsgOnly_0, lut::MsgOnly_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_carryonly_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CarryOnly_0,
        lut::CarryOnly_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_carryinmsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CarryInMsg_0,
        lut::CarryInMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_multcarrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MultCarryMsg_0,
        lut::MultCarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_multcarrymsglsb_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MultCarryMsgLsb_0,
        lut::MultCarryMsgLsb_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_multcarrymsgmsb_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MultCarryMsgMsb_0,
        lut::MultCarryMsgMsb_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_bwand_0_equivalence() {
    compare_legacy_vs_lut(legacy::BwAnd_0, lut::BwAnd_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_bwor_0_equivalence() {
    compare_legacy_vs_lut(legacy::BwOr_0, lut::BwOr_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_bwxor_0_equivalence() {
    compare_legacy_vs_lut(legacy::BwXor_0, lut::BwXor_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmpsign_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpSign_0, lut::CmpSign_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmpreduce_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpReduce_0,
        lut::CmpReduce_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpgt_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpGt_0, lut::CmpGt_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmpgte_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpGte_0, lut::CmpGte_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmplt_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpLt_0, lut::CmpLt_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmplte_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpLte_0, lut::CmpLte_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmpeq_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpEq_0, lut::CmpEq_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_cmpneq_0_equivalence() {
    compare_legacy_vs_lut(legacy::CmpNeq_0, lut::CmpNeq_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_manygenprop_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyGenProp_0,
        lut::ManyGenProp_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manygenprop_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyGenProp_1,
        lut::ManyGenProp_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_reducecarry2_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ReduceCarry2_0,
        lut::ReduceCarry2_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_reducecarry3_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ReduceCarry3_0,
        lut::ReduceCarry3_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_reducecarrypad_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ReduceCarryPad_0,
        lut::ReduceCarryPad_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_genpropadd_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::GenPropAdd_0,
        lut::GenPropAdd_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_iftruezero_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfTrueZeroed_0,
        lut::IfTrueZeroed_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_iffalsezero_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfFalseZeroed_0,
        lut::IfFalseZeroed_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_ripple2genprop_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::Ripple2GenProp_0,
        lut::Ripple2GenProp_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manycarrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyCarryMsg_0,
        lut::ManyCarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpgtmrg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpGtMrg_0,
        lut::CmpGtMrg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpgtemrg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpGteMrg_0,
        lut::CmpGteMrg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpltmrg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpLtMrg_0,
        lut::CmpLtMrg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpltemrg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpLteMrg_0,
        lut::CmpLteMrg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpeqmrg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpEqMrg_0,
        lut::CmpEqMrg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_cmpneqmrg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CmpNeqMrg_0,
        lut::CmpNeqMrg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_issome_0_equivalence() {
    compare_legacy_vs_lut(legacy::IsSome_0, lut::IsSome_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_carryissome_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CarryIsSome_0,
        lut::CarryIsSome_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_carryisnone_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::CarryIsNone_0,
        lut::CarryIsNone_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_multcarrymsgissome_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MultCarryMsgIsSome_0,
        lut::MultCarryMsgIsSome_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_multcarrymsgmsbissome_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MultCarryMsgMsbIsSome_0,
        lut::MultCarryMsgMsbIsSome_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_isnull_0_equivalence() {
    compare_legacy_vs_lut(legacy::IsNull_0, lut::IsNull_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_isnullpos1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IsNullPos1_0,
        lut::IsNullPos1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_notnull_0_equivalence() {
    compare_legacy_vs_lut(legacy::NotNull_0, lut::NotNull_0, CiphertextBlockSpec(2, 2));
}

#[test]
fn test_msgnotnull_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MsgNotNull_0,
        lut::MsgNotNull_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_msgnotnullpos1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::MsgNotNullPos1_0,
        lut::MsgNotNullPos1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manymsgsplitshift1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyMsgSplitShift1_0,
        lut::ManyMsgSplitShift1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solvepropgroupfinal0_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolvePropGroupFinal0_0,
        lut::SolvePropGroupFinal0_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solvepropgroupfinal1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolvePropGroupFinal1_0,
        lut::SolvePropGroupFinal1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solvepropgroupfinal2_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolvePropGroupFinal2_0,
        lut::SolvePropGroupFinal2_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_extractpropgroup0_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ExtractPropGroup0_0,
        lut::ExtractPropGroup0_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_extractpropgroup1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ExtractPropGroup1_0,
        lut::ExtractPropGroup1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_extractpropgroup2_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ExtractPropGroup2_0,
        lut::ExtractPropGroup2_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_extractpropgroup3_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ExtractPropGroup3_0,
        lut::ExtractPropGroup3_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solveprop_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolveProp_0,
        lut::SolveProp_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solvepropcarry_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolvePropCarry_0,
        lut::SolvePropCarry_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solvequotient_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolveQuotient_0,
        lut::SolveQuotient_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_solvequotientpos1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::SolveQuotientPos1_0,
        lut::SolveQuotientPos1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_ifpos1falsezero_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfPos1FalseZeroed_0,
        lut::IfPos1FalseZeroed_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_ifpos1falsezeromsgcarry1_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfPos1FalseZeroedMsgCarry1_0,
        lut::IfPos1FalseZeroedMsgCarry1_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_shiftleftbycarrypos0msg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ShiftLeftByCarryPos0Msg_0,
        lut::ShiftLeftByCarryPos0Msg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_shiftleftbycarrypos0msgnext_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ShiftLeftByCarryPos0MsgNext_0,
        lut::ShiftLeftByCarryPos0MsgNext_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_shiftrightbycarrypos0msg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ShiftRightByCarryPos0Msg_0,
        lut::ShiftRightByCarryPos0Msg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_shiftrightbycarrypos0msgnext_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ShiftRightByCarryPos0MsgNext_0,
        lut::ShiftRightByCarryPos0MsgNext_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_ifpos0truezero_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfPos0TrueZeroed_0,
        lut::IfPos0TrueZeroed_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_ifpos0falsezero_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfPos0FalseZeroed_0,
        lut::IfPos0FalseZeroed_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_ifpos1truezero_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::IfPos1TrueZeroed_0,
        lut::IfPos1TrueZeroed_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv1carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv1CarryMsg_0,
        lut::ManyInv1CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv2carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv2CarryMsg_0,
        lut::ManyInv2CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv3carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv3CarryMsg_0,
        lut::ManyInv3CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv4carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv4CarryMsg_0,
        lut::ManyInv4CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv5carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv5CarryMsg_0,
        lut::ManyInv5CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv6carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv6CarryMsg_0,
        lut::ManyInv6CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv7carrymsg_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv7CarryMsg_0,
        lut::ManyInv7CarryMsg_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manymsgsplit_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyMsgSplit_0,
        lut::ManyMsgSplit_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manym2lpropbit1msgsplit_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manym2lPropBit1MsgSplit_0,
        lut::Manym2lPropBit1MsgSplit_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manym2lpropbit0msgsplit_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manym2lPropBit0MsgSplit_0,
        lut::Manym2lPropBit0MsgSplit_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyl2mpropbit1msgsplit_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manyl2mPropBit1MsgSplit_0,
        lut::Manyl2mPropBit1MsgSplit_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyl2mpropbit0msgsplit_0_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manyl2mPropBit0MsgSplit_0,
        lut::Manyl2mPropBit0MsgSplit_0,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manycarrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyCarryMsg_1,
        lut::ManyCarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manymsgsplitshift1_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyMsgSplitShift1_1,
        lut::ManyMsgSplitShift1_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv1carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv1CarryMsg_1,
        lut::ManyInv1CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv2carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv2CarryMsg_1,
        lut::ManyInv2CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv3carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv3CarryMsg_1,
        lut::ManyInv3CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv4carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv4CarryMsg_1,
        lut::ManyInv4CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv5carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv5CarryMsg_1,
        lut::ManyInv5CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv6carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv6CarryMsg_1,
        lut::ManyInv6CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyinv7carrymsg_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyInv7CarryMsg_1,
        lut::ManyInv7CarryMsg_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manymsgsplit_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::ManyMsgSplit_1,
        lut::ManyMsgSplit_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manym2lpropbit1msgsplit_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manym2lPropBit1MsgSplit_1,
        lut::Manym2lPropBit1MsgSplit_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manym2lpropbit0msgsplit_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manym2lPropBit0MsgSplit_1,
        lut::Manym2lPropBit0MsgSplit_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyl2mpropbit1msgsplit_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manyl2mPropBit1MsgSplit_1,
        lut::Manyl2mPropBit1MsgSplit_1,
        CiphertextBlockSpec(2, 2),
    );
}

#[test]
fn test_manyl2mpropbit0msgsplit_1_equivalence() {
    compare_legacy_vs_lut(
        legacy::Manyl2mPropBit0MsgSplit_1,
        lut::Manyl2mPropBit0MsgSplit_1,
        CiphertextBlockSpec(2, 2),
    );
}
