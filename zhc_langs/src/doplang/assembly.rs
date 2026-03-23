//! Text assembly emission for the DOP dialect.
//!
//! Converts a register-allocated [`IR<DopLang>`] graph into a human-readable assembly listing. Each
//! instruction appears on its own line as `OPCODE arg0 arg1 ...`, with operands formatted via
//! [`Argument::asm`].

use crate::doplang::{DopInstructionSet, DopLang};
use std::fmt::Write;
use zhc_ir::IR;

/// Emits a textual assembly listing from a DOP instruction stream.
///
/// Walks `ir` in linear order and formats each instruction as a newline-terminated line containing
/// the opcode mnemonic followed by space-separated operand representations. The `_INIT`
/// pseudo-instruction is omitted from the output. For `WAIT`, the slot operand appears only when
/// present.
///
/// Returns an empty string when the IR contains no instructions.
pub fn emit_assembly(ir: &IR<DopLang>) -> String {
    let mut output = String::new();
    for op in ir.walk_ops_linear() {
        use DopInstructionSet::*;
        match op.get_instruction() {
            ADD { dst, src1, src2 } => {
                writeln!(output, "ADD {} {} {}", dst.asm(), src1.asm(), src2.asm())
            }
            SUB { dst, src1, src2 } => {
                writeln!(output, "SUB {} {} {}", dst.asm(), src1.asm(), src2.asm())
            }
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => writeln!(
                output,
                "MAC {} {} {} {}",
                dst.asm(),
                src1.asm(),
                src2.asm(),
                cst.asm()
            ),
            ADDS { dst, src, cst } => {
                writeln!(output, "ADDS {} {} {}", dst.asm(), src.asm(), cst.asm())
            }
            SUBS { dst, src, cst } => {
                writeln!(output, "SUBS {} {} {}", dst.asm(), src.asm(), cst.asm())
            }
            SSUB { dst, src, cst } => {
                writeln!(output, "SSUB {} {} {}", dst.asm(), src.asm(), cst.asm())
            }
            MULS { dst, src, cst } => {
                writeln!(output, "MULS {} {} {}", dst.asm(), src.asm(), cst.asm())
            }
            LD { dst, src } => writeln!(output, "LD {} {}", dst.asm(), src.asm()),
            ST { dst, src } => writeln!(output, "ST {} {}", dst.asm(), src.asm()),
            PBS { dst, src, lut } => {
                writeln!(output, "PBS {} {} {}", dst.asm(), src.asm(), lut.asm())
            }
            PBS_ML2 { dst, src, lut } => {
                writeln!(output, "PBS_ML2 {} {} {}", dst.asm(), src.asm(), lut.asm())
            }
            PBS_ML4 { dst, src, lut } => {
                writeln!(output, "PBS_ML4 {} {} {}", dst.asm(), src.asm(), lut.asm())
            }
            PBS_ML8 { dst, src, lut } => {
                writeln!(output, "PBS_ML8 {} {} {}", dst.asm(), src.asm(), lut.asm())
            }
            PBS_F { dst, src, lut } => {
                writeln!(output, "PBS_F {} {} {}", dst.asm(), src.asm(), lut.asm())
            }
            PBS_ML2_F { dst, src, lut } => writeln!(
                output,
                "PBS_ML2_F {} {} {}",
                dst.asm(),
                src.asm(),
                lut.asm()
            ),
            PBS_ML4_F { dst, src, lut } => writeln!(
                output,
                "PBS_ML4_F {} {} {}",
                dst.asm(),
                src.asm(),
                lut.asm()
            ),
            PBS_ML8_F { dst, src, lut } => writeln!(
                output,
                "PBS_ML8_F {} {} {}",
                dst.asm(),
                src.asm(),
                lut.asm()
            ),
            SYNC => writeln!(output, "SYNC"),
            _INIT => Ok(()),
            WAIT { flag, slot } => match slot {
                Some(slot) => writeln!(output, "WAIT {} {}", flag.asm(), slot.asm()),
                None => writeln!(output, "WAIT {}", flag.asm()),
            },
            NOTIFY {
                virt_id,
                flag,
                slot,
            } => writeln!(
                output,
                "NOTIFY {} {} {}",
                virt_id.asm(),
                flag.asm(),
                slot.asm()
            ),
            LD_B2B { flag, slot } => writeln!(output, "LD_B2B {} {}", flag.asm(), slot.asm()),
        }
        .unwrap();
    }

    output
}
