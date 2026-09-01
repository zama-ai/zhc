//! Text assembly emission and parsing for the DOP dialect.
//!
//! Converts between a register-allocated [`IR<DopLang>`] graph and a human-readable assembly
//! listing. Each instruction appears on its own line as `OPCODE arg0 arg1 ...`, with operands
//! formatted via [`Argument::asm`].

use crate::doplang::{DopInstructionSet, DopLang};
use std::fmt::Write;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::IR;

/// Emits a textual assembly listing from a DOP instruction stream.
///
/// Walks `ir` in linear order and formats each instruction as a newline-terminated line containing
/// the opcode mnemonic followed by space-separated operand representations. The `_START` and `_END`
/// pseudo-instruction is omitted from the output. For `WAIT`, the slot operand appears only when
/// present.
///
/// Returns an empty string when the IR contains no instructions.
pub fn emit_assembly(ir: &IR<DopLang>, lreg: &LutRegistry) -> String {
    let mut output = String::new();
    for op in ir.walk_ops_linear() {
        use DopInstructionSet::*;
        match op.get_instruction() {
            _START => Ok(()),
            _END => Ok(()),
            ADD { dst, src1, src2 } => {
                writeln!(
                    output,
                    "ADD {} {} {}",
                    dst.asm(lreg),
                    src1.asm(lreg),
                    src2.asm(lreg)
                )
            }
            SUB { dst, src1, src2 } => {
                writeln!(
                    output,
                    "SUB {} {} {}",
                    dst.asm(lreg),
                    src1.asm(lreg),
                    src2.asm(lreg)
                )
            }
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => writeln!(
                output,
                "MAC {} {} {} {}",
                dst.asm(lreg),
                src1.asm(lreg),
                src2.asm(lreg),
                cst.asm(lreg)
            ),
            ADDS { dst, src, cst } => {
                writeln!(
                    output,
                    "ADDS {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    cst.asm(lreg)
                )
            }
            SUBS { dst, src, cst } => {
                writeln!(
                    output,
                    "SUBS {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    cst.asm(lreg)
                )
            }
            SSUB { dst, src, cst } => {
                writeln!(
                    output,
                    "SSUB {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    cst.asm(lreg)
                )
            }
            MULS { dst, src, cst } => {
                writeln!(
                    output,
                    "MULS {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    cst.asm(lreg)
                )
            }
            LD { dst, src } => writeln!(output, "LD {} {}", dst.asm(lreg), src.asm(lreg)),
            ST { dst, src } => writeln!(output, "ST {} {}", dst.asm(lreg), src.asm(lreg)),
            PBS { dst, src, lut } => {
                writeln!(
                    output,
                    "PBS {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    lut.asm(lreg)
                )
            }
            PBS_ML2 { dst, src, lut } => {
                writeln!(
                    output,
                    "PBS_ML2 {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    lut.asm(lreg)
                )
            }
            PBS_ML4 { dst, src, lut } => {
                writeln!(
                    output,
                    "PBS_ML4 {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    lut.asm(lreg)
                )
            }
            PBS_ML8 { dst, src, lut } => {
                writeln!(
                    output,
                    "PBS_ML8 {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    lut.asm(lreg)
                )
            }
            PBS_F { dst, src, lut } => {
                writeln!(
                    output,
                    "PBS_F {} {} {}",
                    dst.asm(lreg),
                    src.asm(lreg),
                    lut.asm(lreg)
                )
            }
            PBS_ML2_F { dst, src, lut } => writeln!(
                output,
                "PBS_ML2_F {} {} {}",
                dst.asm(lreg),
                src.asm(lreg),
                lut.asm(lreg)
            ),
            PBS_ML4_F { dst, src, lut } => writeln!(
                output,
                "PBS_ML4_F {} {} {}",
                dst.asm(lreg),
                src.asm(lreg),
                lut.asm(lreg)
            ),
            PBS_ML8_F { dst, src, lut } => writeln!(
                output,
                "PBS_ML8_F {} {} {}",
                dst.asm(lreg),
                src.asm(lreg),
                lut.asm(lreg)
            ),
            SYNC => writeln!(output, "SYNC"),
            WAIT { flag, slot } => match slot {
                Some(slot) => writeln!(output, "WAIT {} {}", flag.asm(lreg), slot.asm(lreg)),
                None => writeln!(output, "WAIT {}", flag.asm(lreg)),
            },
            NOTIFY {
                virt_id,
                flag,
                slot,
            } => writeln!(
                output,
                "NOTIFY {} {} {}",
                virt_id.asm(lreg),
                flag.asm(lreg),
                slot.asm(lreg)
            ),
            LD_B2B { flag, slot } => {
                writeln!(output, "LD_B2B {} {}", flag.asm(lreg), slot.asm(lreg))
            }
        }
        .unwrap();
    }
    output
}
