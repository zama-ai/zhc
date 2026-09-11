//! Read-only property checks over a [`DopLang`] program: instruction mix, PBS usage, and
//! register usage. See also [`count_spills`](super::count_spills), which lives in its own module.

use zhc_ir::IR;
use zhc_utils::Dumpable;

use super::{Affinity, CtReg, DopInstructionSet, DopLang, MASK_PBS2, MASK_PBS4, MASK_PBS8};

/// Instruction counts per hardware pipeline lane ([`Affinity`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InstructionMix {
    pub alu: usize,
    pub mem: usize,
    pub pbs: usize,
    pub ctl: usize,
}

impl InstructionMix {
    /// Total instruction count across all lanes.
    pub fn total(&self) -> usize {
        self.alu + self.mem + self.pbs + self.ctl
    }
}

/// Counts instructions per hardware pipeline lane.
pub fn instruction_mix(ir: &IR<DopLang>) -> InstructionMix {
    let mut mix = InstructionMix::default();
    for op in ir.walk_ops_linear() {
        match op.get_instruction() {
            DopInstructionSet::_START | DopInstructionSet::_END => continue,
            instr => match instr.affinity() {
                Affinity::Alu => mix.alu += 1,
                Affinity::Mem => mix.mem += 1,
                Affinity::Pbs => mix.pbs += 1,
                Affinity::Ctl => mix.ctl += 1,
            },
        }
    }
    mix
}

impl Dumpable for InstructionMix {
    fn dump_to_string(&self) -> String {
        format!(
            "Alu={} Mem={} Pbs={} Ctl={} (total={})",
            self.alu,
            self.mem,
            self.pbs,
            self.ctl,
            self.total()
        )
    }
}

/// Programmable-bootstrap instruction counts, split into flushing and non-flushing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PbsUsage {
    pub plain: usize,
    pub flushing: usize,
}

impl PbsUsage {
    /// Total PBS instruction count, flushing and non-flushing combined.
    pub fn total(&self) -> usize {
        self.plain + self.flushing
    }
}

/// Counts programmable-bootstrap instructions, split into flushing and non-flushing.
pub fn pbs_usage(ir: &IR<DopLang>) -> PbsUsage {
    let mut usage = PbsUsage::default();
    for op in ir.walk_ops_linear() {
        let instr = op.get_instruction();
        if instr.is_pbs() {
            if instr.is_pbs_flush() {
                usage.flushing += 1;
            } else {
                usage.plain += 1;
            }
        }
    }
    usage
}

impl Dumpable for PbsUsage {
    fn dump_to_string(&self) -> String {
        format!(
            "{} PBS instruction(s) ({} plain, {} flushing)",
            self.total(),
            self.plain,
            self.flushing
        )
    }
}

/// The number of consecutive registers a many-LUT destination's mask spans.
fn reg_span(mask: usize) -> usize {
    match mask {
        MASK_PBS2 => 2,
        MASK_PBS4 => 4,
        MASK_PBS8 => 8,
        _ => 1,
    }
}

/// The number of registers the program requires: the highest register index touched, plus one
/// (accounting for the register span a many-LUT PBS destination covers).
pub fn register_usage(ir: &IR<DopLang>) -> usize {
    let push_max = |a: &CtReg, max: &mut Option<usize>| {
        let top = a.addr + reg_span(a.mask) - 1;
        *max = Some(max.map_or(top, |m| m.max(top)));
    };

    let mut max_reg: Option<usize> = None;
    for op in ir.walk_ops_linear() {
        use DopInstructionSet::*;
        match op.get_instruction() {
            ADD { dst, src1, src2 } | SUB { dst, src1, src2 } => {
                push_max(dst, &mut max_reg);
                push_max(src1, &mut max_reg);
                push_max(src2, &mut max_reg);
            }
            MAC { dst, src1, src2, .. } => {
                push_max(dst, &mut max_reg);
                push_max(src1, &mut max_reg);
                push_max(src2, &mut max_reg);
            }
            ADDS { dst, src, .. }
            | SUBS { dst, src, .. }
            | SSUB { dst, src, .. }
            | MULS { dst, src, .. }
            | PBS { dst, src, .. }
            | PBS_ML2 { dst, src, .. }
            | PBS_ML4 { dst, src, .. }
            | PBS_ML8 { dst, src, .. }
            | PBS_F { dst, src, .. }
            | PBS_ML2_F { dst, src, .. }
            | PBS_ML4_F { dst, src, .. }
            | PBS_ML8_F { dst, src, .. } => {
                push_max(dst, &mut max_reg);
                push_max(src, &mut max_reg);
            }
            LD { dst, .. } => push_max(dst, &mut max_reg),
            ST { src, .. } => push_max(src, &mut max_reg),
            _START | _END | SYNC | WAIT { .. } | NOTIFY { .. } | LD_B2B { .. } => {}
        }
    }
    max_reg.map_or(0, |m| m + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doplang::parse_assembly;

    fn ir_from(body: &str) -> IR<DopLang> {
        let src = format!(
            "# !preamble {{\n# [signature]\n# Ciphertext<8, 2, 2> -> ()\n# [lut]\n\
             # None: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]\n# }}\n{body}"
        );
        parse_assembly(&src).expect("valid file must parse").1
    }

    #[test]
    fn instruction_mix_buckets_by_affinity() {
        let ir = ir_from("LD R1 TH.0\nADD R2 R1 R1\nSYNC\n");
        let mix = instruction_mix(&ir);
        assert_eq!(mix.alu, 1);
        assert_eq!(mix.mem, 1);
        assert_eq!(mix.ctl, 1);
        assert_eq!(mix.pbs, 0);
        assert_eq!(mix.total(), 3);
    }

    #[test]
    fn pbs_usage_splits_plain_and_flushing() {
        let ir = ir_from("LD R1 TH.0\nPBS R2 R1 PbsNone\nPBS_F R3 R1 PbsNone\n");
        let usage = pbs_usage(&ir);
        assert_eq!(usage.plain, 1);
        assert_eq!(usage.flushing, 1);
        assert_eq!(usage.total(), 2);
    }

    #[test]
    fn register_usage_accounts_for_many_lut_span() {
        // PBS_ML4 R4 R1 ... touches R4..=R7, so 8 registers are required.
        let ir = ir_from("LD R1 TH.0\nPBS_ML4 R4 R1 PbsNone\n");
        assert_eq!(register_usage(&ir), 8);
    }

    #[test]
    fn register_usage_zero_when_no_registers_touched() {
        let ir = ir_from("");
        assert_eq!(register_usage(&ir), 0);
    }
}
