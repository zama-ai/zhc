//! Text assembly emission and parsing for the DOP dialect.
//!
//! Converts between a register-allocated [`IR<DopLang>`] graph and a human-readable assembly
//! listing. Each instruction appears on its own line as `OPCODE arg0 arg1 ...`, with each operand
//! formatted via its own `.asm(...)` method (e.g. [`CtReg::asm`](super::CtReg::asm)).

use crate::doplang::{DopInstructionSet, DopLang};
use std::fmt::Write;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_crypto::integer_semantics::{Type, lut::LutDecoder};
use zhc_ir::{IR, Signature};

mod parser;

pub use parser::*;

const SEPARATOR_WIDTH: usize = 80;

fn separator(prefix: &str) -> String {
    let head = format!("; {prefix}");
    let dashes = SEPARATOR_WIDTH.saturating_sub(head.len());
    format!("{head}{}\n", "-".repeat(dashes))
}

/// Renders a `!preamble { ... }` block: `signature` and every `luts` entry, in the exact form
/// [`parse_preamble`] expects at the top of a `dop.asm` file (see that module's format
/// description) — the write-side counterpart of [`parse_preamble`], the way [`emit_assembly`] is
/// the write-side counterpart of the body parser. Combine both to produce a full, loadable
/// `dop.asm` file: this preamble first, then [`emit_assembly`]'s instruction listing.
pub fn emit_preamble(signature: &Signature<Type>, luts: &LutRegistry) -> String {
    let mut out = String::new();
    out.push_str(&separator(""));
    out.push_str("; !preamble {\n");
    out.push_str("; [signature]\n");
    out.push_str(&format!("; {signature}\n"));
    out.push_str("; [lut]\n");
    let mut luts = luts.iter_luts().collect::<Vec<_>>();
    luts.sort_by_key(|(lid, _)| lid.0);
    for (_, raw) in luts {
        let values = raw
            .lut()
            .iter()
            .map(|b| b.raw_complete_bits().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("; {}: [{values}]\n", raw.name()));
    }
    out.push_str(&separator("} "));
    out
}

/// Emits a textual assembly listing from a DOP instruction stream.
///
/// Walks `ir` in linear order and formats each instruction as a newline-terminated line containing
/// the opcode mnemonic followed by space-separated operand representations. The `_START` and `_END`
/// pseudo-instruction is omitted from the output. For `WAIT`, the slot operand appears only when
/// present.
///
/// Returns an empty string when the IR contains no instructions.
pub fn emit_assembly(ir: &IR<DopLang>, lreg: &impl LutDecoder) -> String {
    let mut output = String::new();
    for op in ir.walk_ops_linear() {
        let asm = format_assembly(op.get_instruction(), lreg);
        writeln!(output, "{}", asm).unwrap();
    }
    output
}

/// Emits a textual assembly for a DOP instruction.
pub fn format_assembly(dop: &DopInstructionSet, lreg: &impl LutDecoder) -> String {
    use DopInstructionSet::*;
    match dop {
        _START | _END => "".to_string(),
        ADD { dst, src1, src2 } => format!(
            "ADD {} {} {}",
            dst.asm(lreg),
            src1.asm(lreg),
            src2.asm(lreg)
        ),
        SUB { dst, src1, src2 } => format!(
            "SUB {} {} {}",
            dst.asm(lreg),
            src1.asm(lreg),
            src2.asm(lreg)
        ),
        MAC {
            dst,
            src1,
            src2,
            cst,
        } => format!(
            "MAC {} {} {} {}",
            dst.asm(lreg),
            src1.asm(lreg),
            src2.asm(lreg),
            cst.asm(lreg)
        ),
        ADDS { dst, src, cst } => {
            format!("ADDS {} {} {}", dst.asm(lreg), src.asm(lreg), cst.asm(lreg))
        }
        SUBS { dst, src, cst } => {
            format!("SUBS {} {} {}", dst.asm(lreg), src.asm(lreg), cst.asm(lreg))
        }
        SSUB { dst, src, cst } => {
            format!("SSUB {} {} {}", dst.asm(lreg), src.asm(lreg), cst.asm(lreg))
        }
        MULS { dst, src, cst } => {
            format!("MULS {} {} {}", dst.asm(lreg), src.asm(lreg), cst.asm(lreg))
        }
        LD { dst, src } => format!("LD {} {}", dst.asm(lreg), src.asm(lreg)),
        ST { dst, src } => format!("ST {} {}", dst.asm(lreg), src.asm(lreg)),
        PBS { dst, src, lut } => {
            format!("PBS {} {} {}", dst.asm(lreg), src.asm(lreg), lut.asm(lreg))
        }
        PBS_ML2 { dst, src, lut } => format!(
            "PBS_ML2 {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        PBS_ML4 { dst, src, lut } => format!(
            "PBS_ML4 {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        PBS_ML8 { dst, src, lut } => format!(
            "PBS_ML8 {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        PBS_F { dst, src, lut } => format!(
            "PBS_F {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        PBS_ML2_F { dst, src, lut } => format!(
            "PBS_ML2_F {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        PBS_ML4_F { dst, src, lut } => format!(
            "PBS_ML4_F {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        PBS_ML8_F { dst, src, lut } => format!(
            "PBS_ML8_F {} {} {}",
            dst.asm(lreg),
            src.asm(lreg),
            lut.asm(lreg)
        ),
        SYNC { is_inner, flag, .. } => {
            if *is_inner {
                format!("SYNC {}", flag.asm(lreg))
            } else {
                format!("SYNC")
            }
        }
        WAIT { flag, slot } => match slot {
            Some(slot) => format!("WAIT {} {}", flag.asm(lreg), slot.asm(lreg)),
            None => format!("WAIT {}", flag.asm(lreg)),
        },
        NOTIFY {
            virt_id,
            flag,
            slot,
        } => format!(
            "NOTIFY {} {} {}",
            virt_id.asm(lreg),
            flag.asm(lreg),
            slot.asm(lreg)
        ),
        LD_B2B { flag, slot } => format!("LD_B2B {} {}", flag.asm(lreg), slot.asm(lreg)),
    }
}
