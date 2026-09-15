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
/// Force discriminants by hand to match the legacy fixed opcode bytes, so
/// [`get_opcode`](Iop::get_opcode) stays compatible with tooling and deployed hardware that
/// already expect those specific values, rather than an incidental declaration order.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Iop {
    // Ct x Imm -------------------------------------------------------------------
    // Arith operations
    Adds = 0xA0,
    Subs = 0xA1,
    Ssub = 0xA2,
    Muls = 0xA3,
    Divs = 0xA4,
    Mods = 0xA5,
    // Overflowing Arith
    OvfAdds = 0xA8,
    OvfSubs = 0xA9,
    OvfSsub = 0xAA,
    OvfMuls = 0xAB,

    // Rotation and Shift
    RightShifts = 0xAC,
    LeftShifts = 0xAD,
    RightRots = 0xAE,
    LeftRots = 0xAF,

    // Ct x Ct -------------------------------------------------------------------
    // Arith operations
    Add = 0xE0,
    Sub = 0xE2,
    Mul = 0xE4,
    Div = 0xE5,
    Mod = 0xE6,

    // Overflowing Arith
    OvfAdd = 0xE8,
    OvfSub = 0xEA,
    OvfMul = 0xEC,

    // BW operations
    BwAnd = 0xD0,
    BwOr = 0xD1,
    BwXor = 0xD2,
    BwNot = 0xD3,

    // Rotation and shift
    RightShift = 0xDC,
    LeftShift = 0xDD,
    RightRot = 0xDE,
    LeftRot = 0xDF,

    // Cmp operations
    CmpGt = 0xC0,
    CmpGte = 0xC1,
    CmpLt = 0xC2,
    CmpLte = 0xC3,
    CmpEq = 0xC4,
    CmpNeq = 0xC5,

    // Ternary operations
    // IfThenZero -> Select or force to 0
    // Take 1Ct and a Boolean Ct as input
    IfThenZero = 0xCA,
    // IfThenElse -> Select operation
    // Take 2Ct and a Boolean Ct as input
    IfThenElse = 0xCB,

    // Custom algorithm
    // ERC7984 -> Found xfer algorithm
    // 2Ct <- func(3Ct)
    Erc7984 = 0x80,

    // Count bits
    CountZeros = 0x81,
    CountOnes = 0x82,
    Ilog2 = 0x83,
    LeadingZeros = 0x84,
    LeadingOnes = 0x85,
    TrailingZeros = 0x86,
    TrailingOnes = 0x87,

    // SIMD for maximum throughput
    AddSimd = 0xF0,
    Erc7984Simd = 0xF1,

    Sum { n: u16 } = 0xF8,

    // Utility operations --------------------------------------------------------
    Cast { to_size: u16 } = 0xFC,
    Flip = 0xFE,
    // Used to handle real clone of ciphertext already uploaded in the Hpu memory
    MemCpy = 0xFF,
}

impl Iop {
    pub fn get_opcode(&self) -> u8 {
        // SAFETY: repr(u32) guarantees the discriminant is the first u8 in the layout
        // NB: Enum discriminant is used and opcode
        unsafe { *(self as *const Self as *const u8) }
    }
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
