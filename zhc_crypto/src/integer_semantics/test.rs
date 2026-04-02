use super::{
    CiphertextBlockSpec, EmulatedCiphertextBlock, EmulatedPlaintextBlock, PlaintextBlockSpec,
};

#[test]
fn has_active_ith_bit_lsb() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec,
    };

    assert!(block.has_active_ith_bit(0));
    assert!(!block.has_active_ith_bit(1));
}

#[test]
fn has_active_ith_bit_various() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_01_010, // bits 1, 3, 5 are set
        spec,
    };

    assert!(!block.has_active_ith_bit(0));
    assert!(block.has_active_ith_bit(1));
    assert!(!block.has_active_ith_bit(2));
    assert!(block.has_active_ith_bit(3));
    assert!(!block.has_active_ith_bit(4));
    assert!(block.has_active_ith_bit(5)); // padding bit
}

#[test]
#[should_panic]
fn has_active_ith_bit_out_of_bounds() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_00_000,
        spec,
    };

    block.has_active_ith_bit(6); // complete_size = 6, so index 6 is out of bounds
}

#[test]
fn has_active_last_ith_bit_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_01_010, // bits 1, 3, 5 set
        spec,
    };

    // last_ith_bit(i) = ith_bit(complete_size - i - 1)
    assert!(block.has_active_last_ith_bit(0)); // bit 5 (padding)
    assert!(!block.has_active_last_ith_bit(1)); // bit 4
    assert!(block.has_active_last_ith_bit(2)); // bit 3
    assert!(!block.has_active_last_ith_bit(3)); // bit 2
    assert!(block.has_active_last_ith_bit(4)); // bit 1
    assert!(!block.has_active_last_ith_bit(5)); // bit 0
}

#[test]
fn has_active_padding_bit_set() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_00_000,
        spec,
    };

    assert!(block.has_active_padding_bit());
}

#[test]
fn has_active_padding_bit_unset() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_11_111,
        spec,
    };

    assert!(!block.has_active_padding_bit());
}

#[test]
fn neg_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_01_010, // 10 in 6-bit space
        spec,
    };

    let result = block.neg();
    // Two's complement: ~10 & 0b111111 = 53, 53 + 1 = 54 = 0b110110
    assert_eq!(result.storage, 0b1_10_110);
    assert_eq!(result.spec, spec);
}

#[test]
fn neg_zero() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_00_000,
        spec,
    };

    let result = block.neg();
    assert_eq!(result.storage, 0b0_00_000);
}

#[test]
fn neg_one() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec,
    };

    let result = block.neg();
    // ~1 & 63 = 62, 62 + 1 = 63 = 0b111111
    assert_eq!(result.storage, 0b1_11_111);
}

#[test]
fn neg_with_padding_set() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_00_001, // padding already set, value = 33
        spec,
    };

    let result = block.neg();
    // ~33 & 63 = 30, 30 + 1 = 31 = 0b011111
    assert_eq!(result.storage, 0b0_11_111);
}

#[test]
fn neg_preserves_spec() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };

    assert_eq!(block.neg().spec, spec);
}

#[test]
fn protect_add_ct_ct_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_10_011,
        spec,
    };

    let result = lhs.protect_add(rhs);
    assert_eq!(result.storage, 0b0_11_101);
    assert_eq!(result.spec, spec);
}

#[test]
fn protect_add_ct_ct_max_values() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_111,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_10_000,
        spec,
    };

    let result = lhs.protect_add(rhs);
    assert_eq!(result.storage, 0b0_11_111);
}

#[test]
#[should_panic(expected = "Spec mismatch")]
fn protect_add_ct_ct_spec_mismatch_panics() {
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec: CiphertextBlockSpec(2, 3),
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_10_011,
        spec: CiphertextBlockSpec(3, 3),
    };

    lhs.protect_add(rhs);
}

#[test]
#[should_panic(expected = "lhs has active padding bit")]
fn protect_add_ct_ct_lhs_padding_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_10_011,
        spec,
    };

    lhs.protect_add(rhs);
}

#[test]
#[should_panic(expected = "rhs has active padding bit")]
fn protect_add_ct_ct_rhs_padding_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_10_011,
        spec,
    };

    lhs.protect_add(rhs);
}

#[test]
#[should_panic(expected = "Overflow occured while performing protect-add")]
fn protect_add_ct_ct_carry_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_11_111,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec,
    };

    lhs.protect_add(rhs);
}

#[test]
fn temper_add_ct_ct_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_10_011,
        spec,
    };

    let result = lhs.temper_add(rhs);
    assert_eq!(result.storage, 0b1_11_101);
}

#[test]
#[should_panic(expected = "Overflow occured while performing temper-add")]
fn temper_add_ct_ct_padding_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_11_111,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec,
    };

    lhs.temper_add(rhs);
}

#[test]
fn wrapping_add_ct_ct_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_10_011,
        spec,
    };

    let result = lhs.wrapping_add(rhs);
    assert_eq!(result.storage, 0b1_11_101);
}

#[test]
fn wrapping_add_ct_ct_overflow_wraps() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_11_111,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec,
    };

    let result = lhs.wrapping_add(rhs);
    assert_eq!(result.storage, 0b0_00_000);
}

#[test]
fn wrapping_add_ct_ct_masks_extra_bits() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b11111_11_111,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b00000_00_001,
        spec,
    };

    let result = lhs.wrapping_add(rhs);
    assert_eq!(result.storage, 0b0_00_000);
}

#[test]
fn protect_add_ct_pt_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b011,
        spec: pt_spec,
    };

    let result = lhs.protect_add_pt(rhs);
    assert_eq!(result.storage, 0b0_01_101);
}

#[test]
#[should_panic(expected = "Spec mismatch")]
fn protect_add_ct_pt_spec_mismatch_panics() {
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec: CiphertextBlockSpec(2, 3),
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b011,
        spec: PlaintextBlockSpec(8),
    };

    lhs.protect_add_pt(rhs);
}

#[test]
#[should_panic(expected = "lhs has active padding bit")]
fn protect_add_ct_pt_padding_panics() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b011,
        spec: pt_spec,
    };

    lhs.protect_add_pt(rhs);
}

#[test]
fn temper_add_ct_pt_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b011,
        spec: pt_spec,
    };

    let result = lhs.temper_add_pt(rhs);
    assert_eq!(result.storage, 0b1_01_101);
}

#[test]
fn wrapping_add_ct_pt_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_11_111,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b001,
        spec: pt_spec,
    };

    let result = lhs.wrapping_add_pt(rhs);
    assert_eq!(result.storage, 0b0_00_000);
}

#[test]
fn protect_sub_ct_ct_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_11_101,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };

    let result = lhs.protect_sub(rhs);
    assert_eq!(result.storage, 0b0_10_011);
}

#[test]
#[should_panic(expected = "lhs has active padding bit")]
fn protect_sub_ct_ct_lhs_padding_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };

    lhs.protect_sub(rhs);
}

#[test]
#[should_panic(expected = "rhs has active padding bit")]
fn protect_sub_ct_ct_rhs_padding_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_11_101,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec,
    };

    lhs.protect_sub(rhs);
}

#[test]
#[should_panic(expected = "Underflow occured while performing protect-sub")]
fn protect_sub_ct_ct_underflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_11_101,
        spec,
    };

    lhs.protect_sub(rhs);
}

#[test]
fn temper_sub_ct_ct_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec,
    };

    let result = lhs.temper_sub(rhs);
    assert_eq!(result.storage, 0b0_10_011);
}

#[test]
#[should_panic(expected = "Underflow occured while performing temper-sub")]
fn temper_sub_ct_ct_underflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_11_101,
        spec,
    };

    lhs.temper_sub(rhs);
}

#[test]
fn wrapping_sub_ct_ct_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_11_101,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };

    let result = lhs.wrapping_sub(rhs);
    assert_eq!(result.storage, 0b0_10_011);
}

#[test]
fn wrapping_sub_ct_ct_underflow_wraps() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_11_101,
        spec,
    };

    let result = lhs.wrapping_sub(rhs);
    assert_eq!(result.storage, 0b1_01_101);
}

#[test]
fn protect_sub_ct_pt_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_01_101,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b010,
        spec: pt_spec,
    };

    let result = lhs.protect_sub_pt(rhs);
    assert_eq!(result.storage, 0b0_01_011);
}

#[test]
#[should_panic(expected = "lhs has active padding bit")]
fn protect_sub_ct_pt_padding_panics() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_01_101,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b010,
        spec: pt_spec,
    };

    lhs.protect_sub_pt(rhs);
}

#[test]
fn temper_sub_ct_pt_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_01_101,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b010,
        spec: pt_spec,
    };

    let result = lhs.temper_sub_pt(rhs);
    assert_eq!(result.storage, 0b1_01_011);
}

#[test]
fn wrapping_sub_ct_pt_underflow_wraps() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec: ct_spec,
    };
    let rhs = EmulatedPlaintextBlock {
        storage: 0b101,
        spec: pt_spec,
    };

    let result = lhs.wrapping_sub_pt(rhs);
    assert_eq!(result.storage, 0b1_11_100);
}

#[test]
fn protect_shl_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };

    let result = block.protect_shl(1);
    assert_eq!(result.storage, 0b0_10_100);
}

#[test]
#[should_panic(expected = "lhs has active padding bit")]
fn protect_shl_padding_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec,
    };

    block.protect_shl(1);
}

#[test]
#[should_panic(expected = "Overflow occured while performing protect-shl")]
fn protect_shl_carry_overflow_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_10_000,
        spec,
    };

    block.protect_shl(1);
}

#[test]
fn wrapping_shl_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_10_100,
        spec,
    };

    let result = block.wrapping_shl(1);
    assert_eq!(result.storage, 0b1_01_000);
}

#[test]
fn wrapping_shl_overflow_wraps() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_11_100,
        spec,
    };

    let result = block.wrapping_shl(1);
    assert_eq!(result.storage, 0b1_11_000);
}

#[test]
fn protect_shr_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_10_100,
        spec,
    };

    let result = block.protect_shr(1);
    assert_eq!(result.storage, 0b0_01_010);
}

#[test]
#[should_panic(expected = "lhs has active padding bit")]
fn protect_shr_padding_panics() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_10_100,
        spec,
    };

    block.protect_shr(1);
}

#[test]
fn wrapping_shr_basic() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b1_10_100,
        spec,
    };

    let result = block.wrapping_shr(1);
    assert_eq!(result.storage, 0b0_11_010);
}

#[test]
fn protect_sub_pt_ct_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedPlaintextBlock {
        storage: 0b111,
        spec: pt_spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_00_010,
        spec: ct_spec,
    };

    let result = lhs.protect_sub_ct(rhs);
    assert_eq!(result.storage, 0b0_00_101);
    assert_eq!(result.spec, ct_spec);
}

#[test]
#[should_panic(expected = "Spec mismatch")]
fn protect_sub_pt_ct_spec_mismatch_panics() {
    let lhs = EmulatedPlaintextBlock {
        storage: 0b111,
        spec: PlaintextBlockSpec(8),
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec: CiphertextBlockSpec(2, 3),
    };

    lhs.protect_sub_ct(rhs);
}

#[test]
#[should_panic(expected = "rhs has active padding bit")]
fn protect_sub_pt_ct_rhs_padding_panics() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedPlaintextBlock {
        storage: 0b111,
        spec: pt_spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec: ct_spec,
    };

    lhs.protect_sub_ct(rhs);
}

#[test]
#[should_panic(expected = "Underflow occured while performing protect-sub")]
fn protect_sub_pt_ct_underflow_panics() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedPlaintextBlock {
        storage: 0b010,
        spec: pt_spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_01_101,
        spec: ct_spec,
    };

    lhs.protect_sub_ct(rhs);
}

#[test]
fn wrapping_sub_pt_ct_basic() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedPlaintextBlock {
        storage: 0b111,
        spec: pt_spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec: ct_spec,
    };

    let result = lhs.wrapping_sub_ct(rhs);
    assert_eq!(result.storage, 0b0_11_101);
    assert_eq!(result.spec, ct_spec);
}

#[test]
fn wrapping_sub_pt_ct_underflow_wraps() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(3);
    let lhs = EmulatedPlaintextBlock {
        storage: 0b010,
        spec: pt_spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_01_101,
        spec: ct_spec,
    };

    let result = lhs.wrapping_sub_ct(rhs);
    assert_eq!(result.storage, 0b1_10_101);
}

#[test]
fn protect_operations_preserve_spec() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b0_00_011,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_00_011,
        spec,
    };

    assert_eq!(lhs.protect_add(rhs).spec, spec);
    assert_eq!(lhs.protect_sub(rhs).spec, spec);
}

#[test]
fn temper_operations_preserve_spec() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_01_010,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_10_011,
        spec,
    };

    assert_eq!(lhs.temper_add(rhs).spec, spec);
    assert_eq!(lhs.temper_sub(rhs).spec, spec);
}

#[test]
fn wrapping_operations_preserve_spec() {
    let spec = CiphertextBlockSpec(2, 3);
    let lhs = EmulatedCiphertextBlock {
        storage: 0b1_11_111,
        spec,
    };
    let rhs = EmulatedCiphertextBlock {
        storage: 0b0_00_001,
        spec,
    };

    assert_eq!(lhs.wrapping_add(rhs).spec, spec);
    assert_eq!(lhs.wrapping_sub(rhs).spec, spec);
}

#[test]
fn shift_operations_preserve_spec() {
    let spec = CiphertextBlockSpec(2, 3);
    let block = EmulatedCiphertextBlock {
        storage: 0b0_01_010,
        spec,
    };

    assert_eq!(block.protect_shl(1).spec, spec);
    assert_eq!(block.protect_shr(1).spec, spec);
    assert_eq!(block.wrapping_shl(1).spec, spec);
    assert_eq!(block.wrapping_shr(1).spec, spec);
}

#[test]
fn plaintext_ct_operations_return_ct_spec() {
    let ct_spec = CiphertextBlockSpec(2, 3);
    let pt_spec = PlaintextBlockSpec(6);
    let pt = EmulatedPlaintextBlock {
        storage: 0b111,
        spec: pt_spec,
    };
    let ct = EmulatedCiphertextBlock {
        storage: 0b0_00_010,
        spec: ct_spec,
    };

    assert_eq!(pt.protect_sub_ct(ct).spec, ct_spec);
    assert_eq!(pt.wrapping_sub_ct(ct).spec, ct_spec);
}
