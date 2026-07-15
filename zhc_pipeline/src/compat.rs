use std::{fmt::Display, str::FromStr};

use zhc_builder::{
    Builder, CiphertextSpec, add, add_simd, bitwise_and, bitwise_inv, bitwise_or, bitwise_xor,
    cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq, count_0, count_1, div, erc7984,
    erc7984_simd, if_then_else, if_then_zero, ilog2, lead0, lead1, mul, overflow_add, overflow_mul,
    overflow_sub, rem, rotate_left, rotate_right, shift_left, shift_right, sub, trail0, trail1,
};
use zhc_ir::IR;
use zhc_langs::{doplang::DopLang, hpulang::HpuLang};
use zhc_sim::{MHz, hpu::HpuConfig};

use crate::{
    alternative_pipeline, latency, regular_pipeline,
    translation_table::{DOpRepr, generate_translation_table},
};

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
    // AddPt,
    // SubPt,
    // PtSub,
    // MulPt,
    // DivPt,
    // ModPt,
    // OvfAddPt,
    // OvfSubPt,
    // OvfPtSub,
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
            // "ADDS" => Ok(Iop::AddPt),
            // "SUBS" => Ok(Iop::SubPt),
            // "SSUB" => Ok(Iop::PtSub),
            // "MULS" => Ok(Iop::MulPt),
            // "DIVS" => Ok(Iop::DivPt),
            // "MODS" => Ok(Iop::ModPt),
            // "OVF_ADDS" => Ok(Iop::OvfAddPt),
            // "OVF_SUBS" => Ok(Iop::OvfSubPt),
            // "OVF_SSUB" => Ok(Iop::OvfPtSub),
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

    /// Generates a translation table for the specified operation configuration.
    ///
    /// Takes the HPU hardware configuration in `hpu_config`, and an integer arithmetic
    /// configuration in `integer_config` to produce an hex stream.
    pub fn get_translation_table(
        &self,
        hpu_config: &HpuConfig,
        spec: CiphertextSpec,
    ) -> Vec<DOpRepr> {
        generate_translation_table(&self.get_scheduled_and_allocated(hpu_config, spec).1)
    }

    pub fn compute_latency(&self, hpu_config: &HpuConfig, spec: CiphertextSpec, freq: MHz) -> f64 {
        latency::compute_latency(
            &self.get_scheduled_and_allocated(hpu_config, spec).1,
            &hpu_config,
        )
        .0
        .as_ts(freq.period())
    }

    pub fn get_scheduled_and_allocated(
        &self,
        hpu_config: &HpuConfig,
        spec: CiphertextSpec,
    ) -> (IR<HpuLang>, IR<DopLang>) {
        let ir = match self {
            Iop::CmpGt => cmp_gt(spec).optimize_ir(),
            Iop::CmpGte => cmp_gte(spec).optimize_ir(),
            Iop::CmpLt => cmp_lt(spec).optimize_ir(),
            Iop::CmpLte => cmp_lte(spec).optimize_ir(),
            Iop::CmpEq => cmp_eq(spec).optimize_ir(),
            Iop::CmpNeq => cmp_neq(spec).optimize_ir(),
            Iop::IfThenElse => if_then_else(spec).optimize_ir(),
            Iop::IfThenZero => if_then_zero(spec).optimize_ir(),
            Iop::Add => add(spec).optimize_ir(),
            Iop::Sub => sub(spec).optimize_ir(),
            Iop::Mul => mul(spec).optimize_ir(),
            Iop::Ilog2 => ilog2(spec).optimize_ir(),
            Iop::CountZeros => count_0(spec).optimize_ir(),
            Iop::CountOnes => count_1(spec).optimize_ir(),
            Iop::LeadingZeros => lead0(spec).optimize_ir(),
            Iop::LeadingOnes => lead1(spec).optimize_ir(),
            Iop::TrailingZeros => trail0(spec).optimize_ir(),
            Iop::TrailingOnes => trail1(spec).optimize_ir(),
            Iop::Div => div(spec).optimize_ir(),
            Iop::Mod => rem(spec).optimize_ir(),
            // Iop::AddPt => todo!(),
            // Iop::SubPt => todo!(),
            // Iop::PtSub => todo!(),
            // Iop::MulPt => todo!(),
            // Iop::DivPt => todo!(),
            // Iop::ModPt => todo!(),
            // Iop::OvfAddPt => todo!(),
            // Iop::OvfSubPt => todo!(),
            // Iop::OvfPtSub => todo!(),
            // Iop::OvfMulPt => todo!(),
            // Iop::RightShiftPt => todo!(),
            // Iop::LeftShiftPt => todo!(),
            // Iop::RightRotPt => todo!(),
            // Iop::LeftRotPt => todo!(),
            Iop::OvfAdd => overflow_add(spec).optimize_ir(),
            Iop::OvfSub => overflow_sub(spec).optimize_ir(),
            Iop::OvfMul => overflow_mul(spec).optimize_ir(),
            Iop::BwAnd => bitwise_and(spec).optimize_ir(),
            Iop::BwOr => bitwise_or(spec).optimize_ir(),
            Iop::BwXor => bitwise_xor(spec).optimize_ir(),
            Iop::BwNot => bitwise_inv(spec).optimize_ir(),
            Iop::RightShift => shift_right(spec).optimize_ir(),
            Iop::LeftShift => shift_left(spec).optimize_ir(),
            Iop::RightRot => rotate_right(spec).optimize_ir(),
            Iop::LeftRot => rotate_left(spec).optimize_ir(),
            Iop::Erc7984 => erc7984(spec).optimize_ir(),
            Iop::Erc7984Simd => erc7984_simd(spec).optimize_ir(),
            Iop::AddSimd => add_simd(spec).optimize_ir(),
            // Iop::MemCpy => todo!(),
        };
        match (self, spec.int_size()) {
            (Iop::Mul, _)
            | (Iop::OvfMul, _)
            | (Iop::RightRot | Iop::LeftRot | Iop::LeftShift | Iop::RightShift, 128) => {
                alternative_pipeline(ir, hpu_config)
            }
            _ => regular_pipeline(ir, hpu_config),
        }
    }
}


impl Display for Iop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Iop::CmpGt => write!(f, "cmp_gt"),
            Iop::CmpGte => write!(f, "cmp_gte"),
            Iop::CmpLt => write!(f, "cmp_lt"),
            Iop::CmpLte => write!(f, "cmp_lte"),
            Iop::CmpEq => write!(f, "cmp_eq"),
            Iop::CmpNeq => write!(f, "cmp_neq"),
            Iop::IfThenElse => write!(f, "if_then_else"),
            Iop::IfThenZero => write!(f, "if_then_zero"),
            Iop::Add => write!(f, "add"),
            Iop::Sub => write!(f, "sub"),
            Iop::Mul => write!(f, "mul"),
            Iop::Ilog2 => write!(f, "ilog2"),
            Iop::CountZeros => write!(f, "count_zeros"),
            Iop::CountOnes => write!(f, "count_ones"),
            Iop::LeadingZeros => write!(f, "leading_zeros"),
            Iop::LeadingOnes => write!(f, "leading_ones"),
            Iop::TrailingZeros => write!(f, "trailing_zeros"),
            Iop::TrailingOnes => write!(f, "trailing_ones"),
            Iop::Div => write!(f, "div"),
            Iop::Mod => write!(f, "mod"),
            Iop::OvfAdd => write!(f, "ovf_add"),
            Iop::OvfSub => write!(f, "ovf_sub"),
            Iop::OvfMul => write!(f, "ovf_mul"),
            Iop::BwAnd => write!(f, "bw_and"),
            Iop::BwOr => write!(f, "bw_or"),
            Iop::BwXor => write!(f, "bw_xor"),
            Iop::BwNot => write!(f, "bw_not"),
            Iop::RightShift => write!(f, "right_shift"),
            Iop::LeftShift => write!(f, "left_shift"),
            Iop::RightRot => write!(f, "right_rot"),
            Iop::LeftRot => write!(f, "left_rot"),
            Iop::Erc7984 => write!(f, "erc7984"),
            Iop::Erc7984Simd => write!(f, "erc7984_simd"),
            Iop::AddSimd => write!(f, "add_simd"),

        }
    }
}
