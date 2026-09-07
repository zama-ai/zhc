use crate::PipelineExt;

use zhc_builder::{
    Builder, CiphertextSpec, add, add_simd, adds, bitwise_and, bitwise_inv, bitwise_or,
    bitwise_xor, cast, cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq, count_0, count_1, div,
    divs, erc7984, erc7984_simd, flip, if_then_else, if_then_zero, ilog2, lead0, lead1, memcpy,
    mods, mul, muls, overflow_add, overflow_adds, overflow_mul, overflow_muls, overflow_ssub,
    overflow_sub, overflow_subs, rem, rotate_left, rotate_right, rots_left, rots_right, shift_left,
    shift_right, shifts_left, shifts_right, ssub, sub, subs, sum, trail0, trail1,
};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig};
use zhc_pipeline::Pipeline;
use zhc_utils::units::Microseconds;

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
    /// Wrapping multiplication of an encrypted integer by a scalar (`ct * imm`, LSB result).
    Muls,
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
    /// Unsigned division of an encrypted integer by a scalar (quotient and remainder).
    Divs,
    /// Unsigned remainder of an encrypted integer by a scalar.
    Mods,
    Adds,
    /// Wrapping subtraction of a scalar from an encrypted integer (`ct - imm`).
    Subs,
    /// Wrapping subtraction of an encrypted integer from a scalar (`imm - ct`).
    Ssub,
    /// `ct + imm` with unsigned-overflow detection.
    OvfAdds,
    /// `ct * imm` with overflow detection.
    OvfMuls,
    /// `ct - imm` with unsigned-underflow detection.
    OvfSubs,
    /// `imm - ct` with unsigned-underflow detection.
    OvfSsub,
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
    /// Logical right shift of an encrypted integer by a scalar.
    RightShifts,
    /// Logical left shift of an encrypted integer by a scalar.
    LeftShifts,
    /// Right rotation of an encrypted integer by a scalar.
    RightRots,
    /// Left rotation of an encrypted integer by a scalar.
    LeftRots,
    Erc7984,
    Erc7984Simd,
    AddSimd,
    /// Block for block copy of an encrypted integer (`dst = src`).
    MemCpy,
    Cast {
        to_size: u16,
    },
    Flip,
    Sum {
        n: u16,
    },
}

impl Iop {
    pub const ALL_STATIC: &[Iop] = &[
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
        Iop::Muls,
        Iop::Ilog2,
        Iop::CountZeros,
        Iop::CountOnes,
        Iop::LeadingZeros,
        Iop::LeadingOnes,
        Iop::TrailingZeros,
        Iop::TrailingOnes,
        Iop::Div,
        Iop::Mod,
        Iop::Divs,
        Iop::Mods,
        Iop::Adds,
        Iop::Subs,
        Iop::Ssub,
        Iop::OvfAdds,
        Iop::OvfMuls,
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
        Iop::RightShifts,
        Iop::LeftShifts,
        Iop::RightRots,
        Iop::LeftRots,
        Iop::OvfAdd,
        Iop::OvfSub,
        Iop::OvfMul,
        Iop::Erc7984,
        Iop::Erc7984Simd,
        Iop::MemCpy,
        Iop::Flip,
    ];

    pub const TEST_ITER: &[Iop] = &[
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
        Iop::Muls,
        Iop::Ilog2,
        Iop::CountZeros,
        Iop::CountOnes,
        Iop::LeadingZeros,
        Iop::LeadingOnes,
        Iop::TrailingZeros,
        Iop::TrailingOnes,
        Iop::Div,
        Iop::Mod,
        Iop::Divs,
        Iop::Mods,
        Iop::Adds,
        Iop::Subs,
        Iop::Ssub,
        Iop::OvfAdds,
        Iop::OvfMuls,
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
        Iop::RightShifts,
        Iop::LeftShifts,
        Iop::RightRots,
        Iop::LeftRots,
        Iop::OvfAdd,
        Iop::OvfSub,
        Iop::OvfMul,
        Iop::Erc7984,
        Iop::Erc7984Simd,
        Iop::MemCpy,
        Iop::Cast { to_size: 2 },
        Iop::Cast { to_size: 4 },
        Iop::Cast { to_size: 8 },
        Iop::Cast { to_size: 16 },
        Iop::Cast { to_size: 32 },
        Iop::Cast { to_size: 64 },
        Iop::Cast { to_size: 128 },
        Iop::Flip,
        Iop::Sum { n: 3 },
        Iop::Sum { n: 5 },
        Iop::Sum { n: 6 },
        Iop::Sum { n: 9 },
        Iop::Sum { n: 11 },
        Iop::Sum { n: 13 },
        Iop::Sum { n: 17 },
        Iop::Sum { n: 26 },
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
            Iop::Muls => muls(spec),
            Iop::Ilog2 => ilog2(spec),
            Iop::CountZeros => count_0(spec),
            Iop::CountOnes => count_1(spec),
            Iop::LeadingZeros => lead0(spec),
            Iop::LeadingOnes => lead1(spec),
            Iop::TrailingZeros => trail0(spec),
            Iop::TrailingOnes => trail1(spec),
            Iop::Div => div(spec),
            Iop::Mod => rem(spec),
            Iop::Divs => divs(spec),
            Iop::Mods => mods(spec),
            Iop::Adds => adds(spec),
            Iop::Subs => subs(spec),
            Iop::Ssub => ssub(spec),
            Iop::OvfAdds => overflow_adds(spec),
            Iop::OvfMuls => overflow_muls(spec),
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
            Iop::RightShifts => shifts_right(spec),
            Iop::LeftShifts => shifts_left(spec),
            Iop::RightRots => rots_right(spec),
            Iop::LeftRots => rots_left(spec),
            Iop::OvfAdd => overflow_add(spec),
            Iop::OvfSub => overflow_sub(spec),
            Iop::OvfMul => overflow_mul(spec),
            Iop::Erc7984 => erc7984(spec),
            Iop::Erc7984Simd => erc7984_simd(spec),
            Iop::MemCpy => memcpy(spec),
            Iop::Cast { to_size } => cast(spec, *to_size),
            Iop::Flip => flip(spec),
            Iop::Sum { n } => sum(spec, *n as usize),
        }
    }

    pub fn get_hpu_pipeline(&self, hpu_config: &HpuConfig, spec: CiphertextSpec) -> Pipeline {
        let pipeline = Pipeline::new()
            .with_builder(self.to_builder(spec))
            .with_hpu_config(hpu_config.clone());
        match (self, spec.int_size()) {
            (Iop::Mul, _)
            | (Iop::OvfMul, _)
            | (Iop::RightRot | Iop::LeftRot | Iop::LeftShift | Iop::RightShift, 128) => {
                pipeline.with_legacy_hpu_scheduler()
            }
            _ => pipeline,
        }
    }

    pub fn compute_latency(&self, hpu_config: &HpuConfig, spec: CiphertextSpec) -> Microseconds {
        let pipeline = Pipeline::new()
            .with_builder(self.to_builder(spec))
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
