//! Read-only property checks over a [`DopLang`] program: instruction mix, PBS usage, and
//! register usage. See also [`count_spills`](super::count_spills), which lives in its own module.

use zhc_ir::IR;
use zhc_utils::Dumpable;

use super::{Affinity, DopInstructionSet, DopLang};

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

/// sync instruction counts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SyncUsage(pub usize);

/// Counts sync instructions, split into global and inner one.
pub fn sync_usage(ir: &IR<DopLang>) -> SyncUsage {
    let mut usage = SyncUsage::default();
    for op in ir.walk_ops_linear() {
        use DopInstructionSet::*;
        let instr = op.get_instruction();
        match instr {
            SYNC { .. } => usage.0 += 1,
            _ => {}
        }
    }
    usage
}

impl Dumpable for SyncUsage {
    fn dump_to_string(&self) -> String {
        format!("{} Sync instruction(s)", self.0,)
    }
}

#[cfg(test)]
mod tests {
    use crate::doplang::parse_assembly;

    use super::*;

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
}
