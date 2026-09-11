//! Named selection of `zhc_langs::doplang`'s property-checking functions for the `--passes` flag.
//!
//! The dialect itself only exposes plain functions (`count_spills`, `instruction_mix`,
//! `pbs_usage`, `register_usage`), each returning a `Dumpable` report — matching how
//! `ioplang`/`hpulang` expose their own passes. [`DopPass`] is this CLI's own concern: a
//! name-to-function lookup so `--passes NameA,NameB` can select which of them to run and print.

use std::fmt;
use std::str::FromStr;

use zhc_ir::IR;
use zhc_langs::doplang::{DopLang, count_spills, instruction_mix, pbs_usage, register_usage};
use zhc_utils::Dumpable;

/// A named property check, run and printed on request from `--passes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DopPass {
    /// Counts `ST` instructions that spill a register to the heap.
    Spills,
    /// Counts instructions per hardware pipeline lane ([`Affinity`](zhc_langs::doplang::Affinity)).
    InstructionMix,
    /// Counts programmable-bootstrap instructions, split into flushing and non-flushing.
    PbsUsage,
    /// Reports the number of registers the program requires.
    RegisterUsage,
}

impl DopPass {
    /// Every available pass, in a stable order.
    pub const ALL: &[DopPass] = &[
        DopPass::Spills,
        DopPass::InstructionMix,
        DopPass::PbsUsage,
        DopPass::RegisterUsage,
    ];

    /// The name this pass is selected by (also its `Display`/`Debug` form).
    pub fn name(&self) -> &'static str {
        match self {
            DopPass::Spills => "Spills",
            DopPass::InstructionMix => "InstructionMix",
            DopPass::PbsUsage => "PbsUsage",
            DopPass::RegisterUsage => "RegisterUsage",
        }
    }

    /// Runs the pass over `ir`, returning a human-readable report line.
    pub fn run(&self, ir: &IR<DopLang>) -> String {
        match self {
            DopPass::Spills => format!("{} heap-store instruction(s)", count_spills(ir)),
            DopPass::InstructionMix => instruction_mix(ir).dump_to_string(),
            DopPass::PbsUsage => pbs_usage(ir).dump_to_string(),
            DopPass::RegisterUsage => {
                format!("{} register(s) required", register_usage(ir))
            }
        }
    }
}

impl fmt::Display for DopPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for DopPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DopPass::ALL
            .iter()
            .find(|pass| pass.name().eq_ignore_ascii_case(s))
            .copied()
            .ok_or_else(|| {
                let available = DopPass::ALL
                    .iter()
                    .map(DopPass::name)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("unknown pass `{s}` (available: {available})")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc_langs::doplang::parse_assembly;

    fn ir_from(body: &str) -> IR<DopLang> {
        let src = format!(
            "# !preamble {{\n# [signature]\n# Ciphertext<8, 2, 2> -> ()\n# [lut]\n\
             # None: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]\n# }}\n{body}"
        );
        parse_assembly(&src).expect("valid file must parse").1
    }

    #[test]
    fn parses_pass_names_case_insensitively() {
        assert_eq!("spills".parse::<DopPass>().unwrap(), DopPass::Spills);
        assert_eq!(
            "RegisterUsage".parse::<DopPass>().unwrap(),
            DopPass::RegisterUsage
        );
        assert!("bogus".parse::<DopPass>().is_err());
    }

    #[test]
    fn spills_reports_heap_stores() {
        let ir = ir_from("LD R1 TH.0\nST TH.1 R1\nST TH.2 R1\nADD R2 R1 R1\n");
        assert!(DopPass::Spills.run(&ir).starts_with("2"));
    }

    #[test]
    fn instruction_mix_reports_affinity_counts() {
        let ir = ir_from("LD R1 TH.0\nADD R2 R1 R1\nSYNC\n");
        let report = DopPass::InstructionMix.run(&ir);
        assert!(report.contains("Alu=1"));
        assert!(report.contains("Mem=1"));
        assert!(report.contains("Ctl=1"));
    }

    #[test]
    fn register_usage_accounts_for_many_lut_span() {
        // PBS_ML4 R4 R1 ... touches R4..=R7, so 8 registers are required.
        let ir = ir_from("LD R1 TH.0\nPBS_ML4 R4 R1 PbsNone\n");
        assert!(
            DopPass::RegisterUsage.run(&ir).starts_with("8 "),
            "{}",
            DopPass::RegisterUsage.run(&ir)
        );
    }
}
