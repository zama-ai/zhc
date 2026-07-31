use std::str::FromStr;

use zhc_builder::{
    Builder, CiphertextSpec, add, add_simd, bitwise_and, bitwise_inv, bitwise_or, bitwise_xor,
    cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq, count_0, count_1, div, erc7984,
    erc7984_simd, if_then_else, if_then_zero, ilog2, lead0, lead1, mul, overflow_add, overflow_mul,
    overflow_sub, rem, rotate_left, rotate_right, shift_left, shift_right, sub, trail0, trail1,
    adds, overflow_ssub, overflow_subs, ssub, subs
};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig};
use zhc_utils::units::Microseconds;

use crate::{Pipeline, hpu::translation_table::DOpRepr};

/// Iops supported by the pipeline.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Iop {
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpEq,
    CmpNeq,
    IfThenElse,
    IfThenZero,
    /// Wrapping addition of two encrypted integers.
    Add,
    /// Wrapping subtraction of two encrypted integers.
    Sub,
    /// Wrapping multiplication of two encrypted integers (LSB result).
    Mul,
    Ilog2,
    CountZeros,
    CountOnes,
    LeadingZeros,
    LeadingOnes,
    TrailingZeros,
    TrailingOnes,
    /// Unsigned division of two encrypted integers (quotient and remainder).
    Div,
    /// Unsigned remainder of two encrypted integers.
    Mod,
    Adds,
    /// Wrapping subtraction of a scalar from an encrypted integer (`ct - imm`).
    Subs,
    /// Wrapping subtraction of an encrypted integer from a scalar (`imm - ct`).
    Ssub,
    /// `ct - imm` with unsigned-underflow detection.
    OvfSubs,
    /// `imm - ct` with unsigned-underflow detection.
    OvfSsub,
    // MulPt,
    // DivPt,
    // ModPt,
    // OvfAddPt,
    // OvfMulPt,
    // RightShiftPt,
    // LeftShiftPt,
    // RightRotPt,
    // LeftRotPt,
    OvfAdd,
    OvfSub,
    OvfMul,
    BwAnd,
    BwOr,
    BwXor,
    BwNot,
    RightShift,
    LeftShift,
    RightRot,
    LeftRot,
    Erc7984,
    Erc7984Simd,
    AddSimd,
    // MemCpy,
}

impl FromStr for Iop {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADD" => Ok(Iop::Add),
            "SUB" => Ok(Iop::Sub),
            "MUL" => Ok(Iop::Mul),
            "CMP_GT" => Ok(Iop::CmpGt),
            "CMP_GTE" => Ok(Iop::CmpGte),
            "CMP_LT" => Ok(Iop::CmpLt),
            "CMP_LTE" => Ok(Iop::CmpLte),
            "CMP_EQ" => Ok(Iop::CmpEq),
            "CMP_NEQ" => Ok(Iop::CmpNeq),
            "IF_THEN_ZERO" => Ok(Iop::IfThenZero),
            "IF_THEN_ELSE" => Ok(Iop::IfThenElse),
            "COUNT0" => Ok(Iop::CountZeros),
            "COUNT1" => Ok(Iop::CountOnes),
            "ILOG2" => Ok(Iop::Ilog2),
            "LEAD0" => Ok(Iop::LeadingZeros),
            "LEAD1" => Ok(Iop::LeadingOnes),
            "TRAIL0" => Ok(Iop::TrailingZeros),
            "TRAIL1" => Ok(Iop::TrailingOnes),
            "DIV" => Ok(Iop::Div),
            "MOD" => Ok(Iop::Mod),
            "ADDS" => Ok(Iop::Adds),
            "SUBS" => Ok(Iop::Subs),
            "SSUB" => Ok(Iop::Ssub),
            "OVF_SUBS" => Ok(Iop::OvfSubs),
            "OVF_SSUB" => Ok(Iop::OvfSsub),
            // "MULS" => Ok(Iop::MulPt),
            // "DIVS" => Ok(Iop::DivPt),
            // "MODS" => Ok(Iop::ModPt),
            // "OVF_ADDS" => Ok(Iop::OvfAddPt),
            // "OVF_MULS" => Ok(Iop::OvfMulPt),
            // "SHIFTS_R" => Ok(Iop::RightShiftPt),
            // "SHIFTS_L" => Ok(Iop::LeftShiftPt),
            // "ROTS_R" => Ok(Iop::RightRotPt),
            // "ROTS_L" => Ok(Iop::LeftRotPt),
            "OVF_ADD" => Ok(Iop::OvfAdd),
            "OVF_SUB" => Ok(Iop::OvfSub),
            "OVF_MUL" => Ok(Iop::OvfMul),
            "BW_AND" => Ok(Iop::BwAnd),
            "BW_OR" => Ok(Iop::BwOr),
            "BW_XOR" => Ok(Iop::BwXor),
            "BW_NOT" => Ok(Iop::BwNot),
            "SHIFT_R" => Ok(Iop::RightShift),
            "SHIFT_L" => Ok(Iop::LeftShift),
            "ROT_R" => Ok(Iop::RightRot),
            "ROT_L" => Ok(Iop::LeftRot),
            "ERC_7984" => Ok(Iop::Erc7984),
            "ERC_7984_SIMD" => Ok(Iop::Erc7984Simd),
            "ADD_SIMD" => Ok(Iop::AddSimd),
            // "MEMCPY" => Ok(Iop::MemCpy),
            _ => Err(()),
        }
    }
}

impl Iop {
    pub const ALL: &[Iop] = &[
        Iop::CmpGt,
        Iop::CmpGte,
        Iop::CmpLt,
        Iop::CmpLte,
        Iop::CmpEq,
        Iop::CmpNeq,
        Iop::IfThenElse,
        Iop::IfThenZero,
        Iop::Add,
        Iop::AddSimd,
        Iop::Sub,
        Iop::Mul,
        Iop::Ilog2,
        Iop::CountZeros,
        Iop::CountOnes,
        Iop::LeadingZeros,
        Iop::LeadingOnes,
        Iop::TrailingZeros,
        Iop::TrailingOnes,
        Iop::Div,
        Iop::Mod,
        Iop::Adds,
        Iop::Subs,
        Iop::Ssub,
        Iop::OvfSubs,
        Iop::OvfSsub,
        Iop::BwAnd,
        Iop::BwOr,
        Iop::BwXor,
        Iop::BwNot,
        Iop::RightShift,
        Iop::LeftShift,
        Iop::RightRot,
        Iop::LeftRot,
        Iop::OvfAdd,
        Iop::OvfSub,
        Iop::OvfMul,
        Iop::Erc7984,
        Iop::Erc7984Simd,
    ];

    /// Returns the builder for this operation with the given ciphertext spec.
    pub fn to_builder(&self, spec: CiphertextSpec) -> Builder {
        match self {
            Iop::CmpGt => cmp_gt(spec),
            Iop::CmpGte => cmp_gte(spec),
            Iop::CmpLt => cmp_lt(spec),
            Iop::CmpLte => cmp_lte(spec),
            Iop::CmpEq => cmp_eq(spec),
            Iop::CmpNeq => cmp_neq(spec),
            Iop::IfThenElse => if_then_else(spec),
            Iop::IfThenZero => if_then_zero(spec),
            Iop::Add => add(spec),
            Iop::AddSimd => add_simd(spec),
            Iop::Sub => sub(spec),
            Iop::Mul => mul(spec),
            Iop::Ilog2 => ilog2(spec),
            Iop::CountZeros => count_0(spec),
            Iop::CountOnes => count_1(spec),
            Iop::LeadingZeros => lead0(spec),
            Iop::LeadingOnes => lead1(spec),
            Iop::TrailingZeros => trail0(spec),
            Iop::TrailingOnes => trail1(spec),
            Iop::Div => div(spec),
            Iop::Mod => rem(spec),
            Iop::Adds => adds(spec),
            Iop::Subs => subs(spec),
            Iop::Ssub => ssub(spec),
            Iop::OvfSubs => overflow_subs(spec),
            Iop::OvfSsub => overflow_ssub(spec),
            Iop::BwAnd => bitwise_and(spec),
            Iop::BwOr => bitwise_or(spec),
            Iop::BwXor => bitwise_xor(spec),
            Iop::BwNot => bitwise_inv(spec),
            Iop::RightShift => shift_right(spec),
            Iop::LeftShift => shift_left(spec),
            Iop::RightRot => rotate_right(spec),
            Iop::LeftRot => rotate_left(spec),
            Iop::OvfAdd => overflow_add(spec),
            Iop::OvfSub => overflow_sub(spec),
            Iop::OvfMul => overflow_mul(spec),
            Iop::Erc7984 => erc7984(spec),
            Iop::Erc7984Simd => erc7984_simd(spec),
        }
    }

    pub fn get_translation_table(
        &self,
        hpu_config: &HpuConfig,
        spec: CiphertextSpec,
    ) -> Vec<DOpRepr> {
        let pipeline = Pipeline::new()
            .with_builder(self.get_builder(spec))
            .with_hpu_config(hpu_config.clone());
        let mut pipeline = match (self, spec.int_size()) {
            (Iop::Mul, _)
            | (Iop::OvfMul, _)
            | (Iop::RightRot | Iop::LeftRot | Iop::LeftShift | Iop::RightShift, 128) => {
                pipeline.with_legacy_hpu_scheduler()
            }
            _ => pipeline,
        };
        pipeline.get_hpu_stream().to_owned()
    }

    pub fn compute_latency(&self, hpu_config: &HpuConfig, spec: CiphertextSpec) -> Microseconds {
        let pipeline = Pipeline::new()
            .with_builder(self.get_builder(spec))
            .with_hpu_config(hpu_config.clone());
        let mut pipeline = match (self, spec.int_size()) {
            (Iop::Mul, _)
            | (Iop::OvfMul, _)
            | (Iop::RightRot | Iop::LeftRot | Iop::LeftShift | Iop::RightShift, 128) => {
                pipeline.with_legacy_hpu_scheduler()
            }
            _ => pipeline,
        };
        pipeline.get_hpu_metrics().latency
    }

    pub fn get_builder(&self, spec: CiphertextSpec) -> Builder {
        match self {
            Iop::CmpGt => cmp_gt(spec),
            Iop::CmpGte => cmp_gte(spec),
            Iop::CmpLt => cmp_lt(spec),
            Iop::CmpLte => cmp_lte(spec),
            Iop::CmpEq => cmp_eq(spec),
            Iop::CmpNeq => cmp_neq(spec),
            Iop::IfThenElse => if_then_else(spec),
            Iop::IfThenZero => if_then_zero(spec),
            Iop::Add => add(spec),
            Iop::Sub => sub(spec),
            Iop::Mul => mul(spec),
            Iop::Ilog2 => ilog2(spec),
            Iop::CountZeros => count_0(spec),
            Iop::CountOnes => count_1(spec),
            Iop::LeadingZeros => lead0(spec),
            Iop::LeadingOnes => lead1(spec),
            Iop::TrailingZeros => trail0(spec),
            Iop::TrailingOnes => trail1(spec),
            Iop::Div => div(spec),
            Iop::Mod => rem(spec),
            Iop::Adds => adds(spec),
            Iop::Subs => subs(spec),
            Iop::Ssub => ssub(spec),
            Iop::OvfSubs => overflow_subs(spec),
            Iop::OvfSsub => overflow_ssub(spec),
            // Iop::MulPt => todo!(),
            // Iop::DivPt => todo!(),
            // Iop::ModPt => todo!(),
            // Iop::OvfAddPt => todo!(),
            // Iop::OvfMulPt => todo!(),
            // Iop::RightShiftPt => todo!(),
            // Iop::LeftShiftPt => todo!(),
            // Iop::RightRotPt => todo!(),
            // Iop::LeftRotPt => todo!(),
            Iop::OvfAdd => overflow_add(spec),
            Iop::OvfSub => overflow_sub(spec),
            Iop::OvfMul => overflow_mul(spec),
            Iop::BwAnd => bitwise_and(spec),
            Iop::BwOr => bitwise_or(spec),
            Iop::BwXor => bitwise_xor(spec),
            Iop::BwNot => bitwise_inv(spec),
            Iop::RightShift => shift_right(spec),
            Iop::LeftShift => shift_left(spec),
            Iop::RightRot => rotate_right(spec),
            Iop::LeftRot => rotate_left(spec),
            Iop::Erc7984 => erc7984(spec),
            Iop::Erc7984Simd => erc7984_simd(spec),
            Iop::AddSimd => add_simd(spec),
            // Iop::MemCpy => todo!(),
        }
    }
}

pub fn mh_mul(spec: CiphertextSpec, config: MultiHpuConfig) -> Pipeline {
    let schoolbook_depth = std::cmp::max(2, config.n_hpus / 2) as usize;
    let builder = zhc_builder::mh_mul(spec, schoolbook_depth);
    match config.n_hpus {
        2 => {
            builder.group_partitions_id(&[1, 2, 0, 9]);
            builder.group_partitions_id(&[3, 4, 6, 7]);
        }
        4 => {
            builder.group_partitions_id(&[4, 6, 7, 0, 9]);
            builder.group_partitions_id(&[2]);
            builder.group_partitions_id(&[3]);
            builder.group_partitions_id(&[1]);
        }
        8 => {
            builder.group_partitions_id(&[1, 8, 24, 0, 36]);
            builder.group_partitions_id(&[2, 3, 23]);
            builder.group_partitions_id(&[4, 5, 25, 27, 28]);
            builder.group_partitions_id(&[6, 7, 29]);
            builder.group_partitions_id(&[9, 10, 26]);
            builder.group_partitions_id(&[11, 12, 30, 32, 34]);
            builder.group_partitions_id(&[14, 19]);
            builder.group_partitions_id(&[15, 16, 31, 33]);
        }
        _ => {
            panic!("n_hpus is out-of-range. Frogs only contains up to 8 nodes");
        }
    }
    Pipeline::new()
        .with_builder(builder)
        .with_multi_hpu_config(config)
}

#[cfg(test)]
mod test {
    use zhc_builder::CiphertextSpec;
    use zhc_config::multi_hpu::MultiHpuConfig;

    use crate::compat::mh_mul;

    #[test]
    fn test_mh_mul_pipeline() {
        const INT_SIZES: [u16; 4] = [8, 16, 32, 64];
        const MH_FACTORS: [u8; 3] = [2, 4, 8];

        for int_size in INT_SIZES {
            for mh in MH_FACTORS {
                let mut pl = mh_mul(
                    CiphertextSpec::new(int_size, 2, 2),
                    MultiHpuConfig {
                        n_hpus: mh,
                        ..Default::default()
                    },
                );
                pl.get_multi_hpu_trace();
            }
        }
    }
}
