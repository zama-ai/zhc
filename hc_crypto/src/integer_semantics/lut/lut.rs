#![allow(non_snake_case)]
use super::super::{EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage};

const CMP_INFERIOR: EmulatedCiphertextBlockStorage = 0;
const CMP_EQUAL: EmulatedCiphertextBlockStorage = 1;
const CMP_SUPERIOR: EmulatedCiphertextBlockStorage = 2;

pub fn None_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    block
}

pub fn MsgOnly_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    block.mask_message()
}

pub fn CarryOnly_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    block.mask_carry()
}

pub fn CarryInMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    block.move_carry_to_message()
}

pub fn MultCarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    block
        .spec
        .from_data((carry_val * msg_val) & block.spec.data_mask())
}

pub fn MultCarryMsgLsb_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    block
        .spec
        .from_message((carry_val * msg_val) & block.spec.message_mask())
}

pub fn MultCarryMsgMsb_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    let result = ((carry_val * msg_val) >> block.spec.message_size()) & block.spec.message_mask();
    block.spec.from_message(result)
}

pub fn BwAnd_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    block
        .spec
        .from_message((carry_val & msg_val) & block.spec.message_mask())
}

pub fn BwOr_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    block
        .spec
        .from_message((carry_val | msg_val) & block.spec.message_mask())
}

pub fn BwXor_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    block
        .spec
        .from_message((carry_val ^ msg_val) & block.spec.message_mask())
}

pub fn CmpSign_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Signed comparison with 0. Based on behavior of negacyclic function.
    // Example for Padding| 4bit digits (i.e 2msg2Carry)
    // 1|xxxx -> SignLut -> -1 -> 0|1111
    // x|0000 -> SignLut ->  0 -> 0|0000
    // 0|xxxx -> SignLut ->  1 -> 0|0001
    let result = if block.storage != 0 { 1 } else { 0 };
    block.spec.from_message(result)
    // WARN: in practice return value with padding that could encode -1, 0, 1
    //       But should always be follow by an add to reach back range 0, 1, 2
    //       To ease degree handling considered an output degree of 1 to obtain
    //       degree 2 after add
    // Not a perfect solution but the easiest to prevent degree error
}

pub fn CmpReduce_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Carry contain MSB cmp result, msg LSB cmp result
    // Reduction is made from lsb to msb as follow
    // MSB      | LSB | Out
    // Inferior | x   | Inferior
    // Equal    | x   | x
    // Superior | x   | Superior
    let carry_field = (block.storage & block.spec.carry_mask()) >> block.spec.message_size();
    let msg_field = block.storage & block.spec.message_mask();
    let result = match (carry_field, msg_field) {
        (CMP_EQUAL, lsb_cmp) => lsb_cmp,
        _ => carry_field,
    };
    block.spec.from_message(result)
}

pub fn CmpGt_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = match block.raw_mask_message() {
        CMP_SUPERIOR => 1,
        _ => 0,
    };
    block.spec.from_message(result)
}

pub fn CmpGte_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = match block.raw_mask_message() {
        CMP_SUPERIOR | CMP_EQUAL => 1,
        _ => 0,
    };
    block.spec.from_message(result)
}

// Could be merge with Gt/Gte
pub fn CmpLt_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = match block.raw_mask_message() {
        CMP_INFERIOR => 1,
        _ => 0,
    };
    block.spec.from_message(result)
}

pub fn CmpLte_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = match block.raw_mask_message() {
        CMP_INFERIOR | CMP_EQUAL => 1,
        _ => 0,
    };
    block.spec.from_message(result)
}

pub fn CmpEq_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = match block.raw_mask_message() {
        CMP_EQUAL => 1,
        _ => 0,
    };
    block.spec.from_message(result)
}

pub fn CmpNeq_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = match block.raw_mask_message() {
        CMP_EQUAL => 0,
        _ => 1,
    };
    block.spec.from_message(result)
}

pub fn ManyGenProp_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    let result = (carry_val << 1) |                                   // Generate
                 ((msg_val == block.spec.message_mask()) as EmulatedCiphertextBlockStorage); // Propagate
    block.spec.from_data(result)
}

pub fn ManyGenProp_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    block.mask_message()
}

pub fn ReduceCarry2_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry = block.storage >> 2;
    let prop = (block.storage & 3 == 3) as EmulatedCiphertextBlockStorage;
    block.spec.from_data((carry << 1) | prop)
}

pub fn ReduceCarry3_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry = block.storage >> 3;
    let prop = (block.storage & 7 == 7) as EmulatedCiphertextBlockStorage;
    block.spec.from_data((carry << 1) | prop)
}

pub fn ReduceCarryPad_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // This corresponds to the accumulated propagation status
    // of 4 consecutive blocks.
    // !! The padding bit is used.
    // +1 must be done after this PBS to retrieve the propagation status value.
    // 0_1111 => 0_0000 + 1 => 1 Propagate
    // 0_xxxx -> 1_1111 + 1 => 0 No carry
    // 1_xxxx -> 0_0001 + 1 => 2 Generate
    let result = if block.storage == block.spec.data_mask() {
        0
    } else {
        block.spec.complete_mask()
    };

    block.spec.from_complete(result)
}

pub fn GenPropAdd_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let lhs = block.raw_mask_message();
    let rhs = block.move_carry_to_message().raw_mask_message();
    let rhs_gen = rhs >> 1;

    block
        .spec
        .from_message((lhs + rhs_gen) & block.spec.message_mask())
}

pub fn IfTrueZeroed_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let value = block.raw_mask_message();
    let cond = block.move_carry_to_message().raw_mask_message();

    let result = if cond != 0 { 0 } else { value };
    block.spec.from_message(result)
}

pub fn IfFalseZeroed_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let value = block.raw_mask_message();
    let cond = block.move_carry_to_message().raw_mask_message();

    let result = if cond != 0 { value } else { 0 };
    block.spec.from_message(result)
}

pub fn Ripple2GenProp_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = block.raw_mask_message() * 2;
    block.spec.from_data(result)
}

pub fn ManyCarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    block.spec.from_message(block.raw_mask_message())
}

pub fn ManyCarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = block.storage >> block.spec.message_size();
    block.spec.from_data(result)
}

pub fn CmpGtMrg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (CMP_SUPERIOR, _) | (CMP_EQUAL, CMP_SUPERIOR) => 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn CmpGteMrg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (CMP_SUPERIOR, _) | (CMP_EQUAL, CMP_SUPERIOR) | (CMP_EQUAL, CMP_EQUAL) => 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn CmpLtMrg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (CMP_INFERIOR, _) | (CMP_EQUAL, CMP_INFERIOR) => 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn CmpLteMrg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (CMP_INFERIOR, _) | (CMP_EQUAL, CMP_INFERIOR) | (CMP_EQUAL, CMP_EQUAL) => 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn CmpEqMrg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (CMP_EQUAL, CMP_EQUAL) => 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn CmpNeqMrg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (CMP_EQUAL, CMP_EQUAL) => 0,
        _ => 1,
    };

    block.spec.from_message(result)
}

pub fn IsSome_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let result = if block.storage != 0 { 1 } else { 0 };
    block.spec.from_message(result)
}

pub fn CarryIsSome_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let result = if carry_field != 0 { 1 } else { 0 };
    block.spec.from_message(result)
}

pub fn CarryIsNone_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let result = if carry_field != 0 { 0 } else { 1 };
    block.spec.from_message(result)
}

pub fn MultCarryMsgIsSome_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    let carry_x_msg = (carry_val * msg_val) & block.spec.data_mask();

    let result = if carry_x_msg != 0 { 1 } else { 0 };
    block.spec.from_message(result)
}

pub fn MultCarryMsgMsbIsSome_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_val = block.move_carry_to_message().raw_mask_message();
    let msg_val = block.raw_mask_message();
    let mul_msb = ((carry_val * msg_val) >> block.spec.message_size()) & block.spec.message_mask();

    let result = if mul_msb != 0 { 1 } else { 0 };
    block.spec.from_message(result)
}

pub fn IsNull_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field as usize, msg_field as usize) {
        (0, 0) => 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn IsNullPos1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Output boolean at bit position 1 instead of 0
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (0, 0) => 1 << 1,
        _ => 0,
    };

    block.spec.from_message(result)
}

pub fn NotNull_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let carry_field = block.move_carry_to_message().raw_mask_message();
    let msg_field = block.raw_mask_message();

    let result = match (carry_field, msg_field) {
        (0, 0) => 0,
        _ => 1,
    };

    block.spec.from_message(result)
}

pub fn MsgNotNull_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let msg_field = block.raw_mask_message();

    let result = match msg_field {
        0 => 0,
        _ => 1,
    };

    block.spec.from_message(result)
}

pub fn MsgNotNullPos1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Return the null (0) or not null (1)
    // status of the msg part.
    // Put the result at position 1.
    let msg_field = block.raw_mask_message();

    let result = match msg_field {
        0 => 0,
        _ => 1 << 1,
    };

    block.spec.from_message(result)
}

pub fn ManyMsgSplitShift1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Use manyLUT : split msg in halves, inverse their position
    // in the message, and  output them separately.
    let lsb_size = (block.spec.message_size()).div_ceil(2);
    let msg_lsb = block.raw_mask_message() & ((1 << lsb_size) - 1);

    block.spec.from_message(msg_lsb << lsb_size)
}

pub fn ManyMsgSplitShift1_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let lsb_size = block.spec.message_size().div_ceil(2);
    let result = block.raw_mask_message() >> lsb_size; // msg_msb

    block.spec.from_message(result)
}

pub fn SolvePropGroupFinal0_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the propagation status of
    // of 4 blocks.
    // The input contains the sum of the propagate status
    // of (position + 1) blocks + the carry of previous group.
    // The result depends on the position to solve. Here we solve position 0.
    // The output value is then directly the carry.
    // 1/0 + [0]
    // 0x => NO_CARRY(0)
    // 1x => GENERATE(1)
    let position = 0;
    let pos_w = position + 2;
    let result = (block.storage >> (pos_w - 1)) & 1; // msb
    block.spec.from_message(result)
}

pub fn SolvePropGroupFinal1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the propagation status of
    // of 4 blocks.
    // The input contains the sum of the propagate status
    // of (position + 1) blocks + the carry of previous group.
    // The result depends on the position to solve. Here we solve position 1.
    // The output value is then directly the carry.
    // 1/0 + + [0] + [1] << 1
    // 0xx => NO_CARRY(0)
    // 1xx => GENERATE(1)
    let position = 1;
    let pos_w = position + 2;
    let result = (block.storage >> (pos_w - 1)) & 1; // msb
    block.spec.from_message(result)
}

pub fn SolvePropGroupFinal2_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the propagation status of
    // of 4 blocks.
    // The input contains the sum of the propagate status
    // of (position + 1) blocks + the carry of previous group.
    // The result depends on the position to solve. Here we solve position 2.
    // The output value is then directly the carry.
    // 1/0 + [0] + [1] << 1 + [2] << 2
    // 0xxx => NO_CARRY(0)
    // 1xxx => GENERATE(1)
    let position = 2;
    let pos_w = position + 2;
    let result = (block.storage >> (pos_w - 1)) & 1; // msb
    block.spec.from_message(result)
}

pub fn ExtractPropGroup0_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Extract propagation status and
    // set the value at the correct position location.
    // Here the position is 0.
    let position = 0;
    let msg = block.raw_mask_message();
    let carry = block.move_carry_to_message().raw_mask_message() & 1;

    let result = if carry == 1 {
        2 << position // Generate
    } else if msg == block.spec.message_mask() {
        1 << position // Propagate
    } else {
        0 << position // No carry
    };

    block.spec.from_message(result)
}

pub fn ExtractPropGroup1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Extract propagation status and
    // set the value at the correct position location.
    // Here the position is 1.
    let position = 1;
    let msg = block.raw_mask_message();
    let carry = block.move_carry_to_message().raw_mask_message() & 1;

    let result = if carry == 1 {
        2 << position // Generate
    } else if msg == block.spec.message_mask() {
        1 << position // Propagate
    } else {
        0 << position // No carry
    };

    block.spec.from_data(result)
}

pub fn ExtractPropGroup2_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Extract propagation status and
    // set the value at the correct position location.
    // Here the position is 2.
    let position = 2;
    let msg = block.raw_mask_message();
    let carry = block.move_carry_to_message().raw_mask_message() & 1;

    let result = if carry == 1 {
        2 << position // Generate
    } else if msg == block.spec.message_mask() {
        1 << position // Propagate
    } else {
        0 << position // No carry
    };

    block.spec.from_data(result)
}

pub fn ExtractPropGroup3_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Extract propagation status and
    // set the value at the correct position location.
    // Here the position is 3.
    let position = 3;
    let msg = block.raw_mask_message();
    let carry = block.move_carry_to_message().raw_mask_message() & 1;

    let result = if carry == 1 {
        2 << position // Generate
    } else if msg == block.spec.message_mask() {
        1 << position // Propagate
    } else {
        0 << position // No carry
    };

    block.spec.from_complete(result)
}

pub fn SolveProp_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the propagation status.
    // 2 propagation status are stored in the input:
    // MSB : propagation to solved
    // LSB : neighbor's propagation
    let msb = block.move_carry_to_message().raw_mask_message();
    let lsb = block.raw_mask_message();

    let result = if msb == 1 {
        // Propagate
        lsb
    } else {
        msb
    };

    block.spec.from_message(result)
}

pub fn SolvePropCarry_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the propagation status.
    // A propagation status and a carry are stored in the input:
    // Output a carry value.
    // MSB : propagation to solved
    // LSB : neighbor's carry bit
    let msb = block.move_carry_to_message().raw_mask_message();
    let lsb = block.raw_mask_message();

    let result = if msb == 1 {
        // Propagate
        lsb
    } else {
        msb >> 1 // Since generate equals 2. Here we want a carry output
    };

    block.spec.from_message(result)
}

pub fn SolveQuotient_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the quotient of a division.
    // The input contains the sum of 4 bits, representing the comparison of current remaining
    // and the different multiples of the divider.
    // Note that the values form a multi-hot. Therefore, their sum
    // gives the value of the divider quotient, that corresponds to the remaining.
    // 'b0000 => 3 (sum = 0)
    // 'b1000 => 2 (sum = 1)
    // 'b1100 => 1 (sum = 2)
    // 'b1110 => 0 (sum = 3)
    let v = block.raw_mask_data();

    let result = match v as usize {
        0 => 3,
        1 => 2,
        2 => 1,
        3 => 0,
        _ => 0,
        //_  => panic!("Unknown quotient value {}!",v) // should not end here
    };

    block.spec.from_message(result)
}

pub fn SolveQuotientPos1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Solve the quotient of a division.
    // The input contains the sum of 4 bits, representing the comparison of current remaining
    // and the different multiples of the divider.
    // Note that the comparison stored in position 1 instead of 0.
    // Therefore the sum value is doubled.
    // Note that the values form a multi-hot. Therefore, their sum
    // gives the value of the divider quotient, that corresponds to the remaining.
    // 'b0000 => 3 (sum = 0*2)
    // 'b1000 => 2 (sum = 1*2)
    // 'b1100 => 1 (sum = 2*2)
    // 'b1110 => 0 (sum = 3*2)
    let v = block.raw_mask_data();

    let result = match v {
        0 => 3,
        2 => 2,
        4 => 1,
        6 => 0,
        _ => 0,
        //_  => panic!("Unknown quotient value {}!",v) // should not end here
    };

    block.spec.from_message(result)
}

pub fn IfPos1FalseZeroed_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain CondCt in Carry bit 1 and ValueCt in Msg. If condition it's *FALSE*, value ct
    // is forced to 0
    let value = block.raw_mask_message();
    let cond = (block.storage >> (block.spec.message_size() + 1)) & 1;

    let result = if cond != 0 { value } else { 0 };
    block.spec.from_message(result)
}

pub fn IfPos1FalseZeroedMsgCarry1_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain CondCt in Carry bit 1
    // and ValueCt in Msg + 1 carry bit. If condition it's *FALSE*, value ct is forced to 0
    let value = block.storage & (block.spec.message_mask() * 2 + 1);
    let cond = (block.storage >> (block.spec.message_size() + 1)) & 1;

    let result = if cond != 0 { value } else { 0 };
    block.spec.from_data(result)
}

// Shift related Pbs
pub fn ShiftLeftByCarryPos0Msg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain shift amount only bit 1 considered
    let value = block.raw_mask_message();
    let shift = block.move_carry_to_message().raw_mask_message() & 0x1;

    let result = (value << shift) & block.spec.message_mask();
    block.spec.from_message(result)
}

pub fn ShiftLeftByCarryPos0MsgNext_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain shift amount only bit 1 considered
    let value = block.raw_mask_message();
    let shift = block.move_carry_to_message().raw_mask_message() & 0x1;

    let result = ((value << shift) & block.spec.carry_mask()) >> block.spec.message_size();
    block.spec.from_message(result)
}

pub fn ShiftRightByCarryPos0Msg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain shift amount only bit 1 considered
    let value = block.raw_mask_message();
    let shift = block.move_carry_to_message().raw_mask_message() & 0x1;

    let result = (value >> shift) & block.spec.message_mask();
    block.spec.from_message(result)
}

pub fn ShiftRightByCarryPos0MsgNext_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain shift amount only bit 1 considered
    // NB: MsgNext with right shift is the content of blk at the right position (i.e. LSB side)
    let value = block.raw_mask_message();
    let shift = block.move_carry_to_message().raw_mask_message() & 0x1;

    let result = ((value << block.spec.message_size()) >> shift) & block.spec.message_mask();
    block.spec.from_message(result)
}

pub fn IfPos0TrueZeroed_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain CondCt in Carry[0] and ValueCt in Msg. If condition it's *TRUE*, value ct is
    // forced to 0
    let value = block.raw_mask_message();
    let cond = block.move_carry_to_message().raw_mask_message() & 0x1;

    let result = if cond != 0 { 0 } else { value };
    block.spec.from_message(result)
}

pub fn IfPos0FalseZeroed_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain CondCt in Carry[0] and ValueCt in Msg. If condition it's *FALSE*, value ct is
    // forced to 0
    let value = block.raw_mask_message();
    let cond = block.move_carry_to_message().raw_mask_message() & 0x1;

    let result = if cond != 0 { value } else { 0 };
    block.spec.from_message(result)
}

// If then zero with condition in Carry0 or Carry1
pub fn IfPos1TrueZeroed_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Ct must contain CondCt in Carry[1] and ValueCt in Msg. If condition it's *TRUE*, value ct is
    // forced to 0
    let value = block.raw_mask_message();
    let cond = (block.move_carry_to_message().raw_mask_message() >> 1) & 0x1;

    let result = if cond != 0 { 0 } else { value };
    block.spec.from_message(result)
}

// NB: Lut IfPos1FalseZeroed already defined earlier
pub fn ManyInv1CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 1;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv1CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 1;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv2CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 2;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv2CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 2;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv3CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 3;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv3CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 3;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv4CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 4;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv4CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 4;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv5CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 5;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv5CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 5;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv6CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 6;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv6CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 6;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv7CarryMsg_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Proceed Inv - ct
    // Extract message and carry using many LUT.
    let inv = 7;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value & block.spec.message_mask()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyInv7CarryMsg_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let inv = 7;
    let mut value = block.storage & block.spec.data_mask();
    let result = if value > inv {
        0
    } else {
        value = inv - value;
        value >> block.spec.message_size()
    };

    EmulatedCiphertextBlock {
        storage: result,
        spec: block.spec,
    }
}

pub fn ManyMsgSplit_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Use manyLUT : split msg in halves
    let lsb_size = block.spec.message_size().div_ceil(2);

    let result = block.raw_mask_message() & ((1 << lsb_size) - 1); // msg_lsb
    block.spec.from_message(result)
}

pub fn ManyMsgSplit_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let lsb_size = block.spec.message_size().div_ceil(2);

    let result = block.raw_mask_message() >> lsb_size; // msg_msb
    block.spec.from_message(result)
}

pub fn Manym2lPropBit1MsgSplit_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Use ManyLut
    // In carry part, contains the info if neighbor has a bit=1 (not null)
    // or not (null).
    // Propagate bits equal to 1 from msb to lsb.
    // Split resulting message part into 2. Put both in lsb.
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from msb to lsb
    for idx in (0..block.spec.message_size()).rev() {
        let mut b = (m >> idx) & 1;
        m &= (1 << idx) - 1;
        if c > 0 {
            b = 1;
        } // propagate to lsb
        if b == 1 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: exp & ((1 << lsb_size) - 1) as EmulatedCiphertextBlockStorage, // msg_lsb
        spec: block.spec,
    }
}

pub fn Manym2lPropBit1MsgSplit_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from msb to lsb
    for idx in (0..block.spec.message_size()).rev() {
        let mut b = (m >> idx) & 1;
        m &= (1 << idx) - 1;
        if c > 0 {
            b = 1;
        } // propagate to lsb
        if b == 1 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: (exp & block.spec.message_mask()) >> lsb_size, // msg_msb
        spec: block.spec,
    }
}

pub fn Manym2lPropBit0MsgSplit_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Use ManyLut
    // In carry part, contains the info if neighbor has a bit=0 (not null)
    // or not (null).
    // Propagate bits equal to 0 from msb to lsb.
    // Split resulting message part into 2. Put both in lsb.
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from msb to lsb
    for idx in (0..block.spec.message_size()).rev() {
        let mut b = (m >> idx) & 1;
        m &= (1 << idx) - 1;
        if c > 0 {
            b = 0;
        } // propagate to lsb
        if b == 0 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: exp & ((1 << lsb_size) - 1), // msg_lsb
        spec: block.spec,
    }
}

pub fn Manym2lPropBit0MsgSplit_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from msb to lsb
    for idx in (0..block.spec.message_size()).rev() {
        let mut b = (m >> idx) & 1;
        m &= (1 << idx) - 1;
        if c > 0 {
            b = 0;
        } // propagate to lsb
        if b == 0 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: (exp & block.spec.message_mask()) >> lsb_size, // msg_msb
        spec: block.spec,
    }
}

pub fn Manyl2mPropBit1MsgSplit_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Use ManyLut
    // In carry part, contains the info if neighbor has a bit=1 (not null)
    // or not (null).
    // Propagate bits equal to 1 from lsb to msb.
    // Split resulting message part into 2. Put both in lsb.
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from lsb to msb
    for idx in 0..block.spec.message_size() {
        let mut b = m & 1;
        m >>= 1;
        if c > 0 {
            b = 1;
        } // propagate to msb
        if b == 1 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: exp & ((1 << lsb_size) - 1), // msg_lsb
        spec: block.spec,
    }
}

pub fn Manyl2mPropBit1MsgSplit_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from lsb to msb
    for idx in 0..block.spec.message_size() {
        let mut b = m & 1;
        m >>= 1;
        if c > 0 {
            b = 1;
        } // propagate to msb
        if b == 1 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: (exp & block.spec.message_mask()) >> (lsb_size as u8), // msg_msb
        spec: block.spec,
    }
}

pub fn Manyl2mPropBit0MsgSplit_0(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    // Use ManyLut
    // In carry part, contains the info if neighbor has a bit=0 (not null)
    // or not (null).
    // Propagate bits equal to 0 from lsb to msb.
    // Split resulting message part into 2. Put both in lsb.
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from lsb to msb
    for idx in 0..block.spec.message_size() {
        let mut b = m & 1;
        m >>= 1;
        if c > 0 {
            b = 0;
        } // propagate to msb
        if b == 0 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: exp & ((1 << lsb_size) - 1), // msg_lsb
        spec: block.spec,
    }
}

pub fn Manyl2mPropBit0MsgSplit_1(block: EmulatedCiphertextBlock) -> EmulatedCiphertextBlock {
    let mut c = block.storage & block.spec.carry_mask();
    let mut m = block.storage & block.spec.message_mask();
    let mut exp = 0;
    // Expand from lsb to msb
    for idx in 0..block.spec.message_size() {
        let mut b = m & 1;
        m >>= 1;
        if c > 0 {
            b = 0;
        } // propagate to msb
        if b == 0 {
            c = 1;
        }
        exp += b << idx;
    }
    let lsb_size = block.spec.message_size().div_ceil(2);

    EmulatedCiphertextBlock {
        storage: (exp & block.spec.message_mask()) >> (lsb_size as u8), // msg_msb
        spec: block.spec,
    }
}
