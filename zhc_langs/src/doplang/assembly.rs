//! Text assembly emission and parsing for the DOP dialect.
//!
//! Converts between a register-allocated [`IR<DopLang>`] graph and a human-readable assembly
//! listing. Each instruction appears on its own line as `OPCODE arg0 arg1 ...`, with operands
//! formatted via [`Argument::asm`].

use crate::doplang::instruction_set::LUT_ALIASES;
use crate::doplang::{
    Argument, DopInstructionSet, DopLang, MASK_NONE, MASK_PBS2, MASK_PBS4, MASK_PBS8,
};
use std::fmt::Write;
use zhc_ir::IR;
use zhc_utils::svec;

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
        }
        .unwrap();
    }

    output
}

/// Parse error with line number and description.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parses a single argument token into an [`Argument`].
///
/// Recognized formats:
/// - `123` → `PtConst { val: 123 }`
/// - `TH.N` → `CtHeap { addr: N }`
/// - `@N` → `CtIo { addr: N }`
/// - `TS[N].M` → `CtSrcVar { id: N, block: M }`
/// - `TD[N].M` → `CtDstVar { id: N, block: M }`
/// - `TI[N].M` → `PtSrcVar { id: N, block: M }`
/// - `RN` → `CtReg { mask, addr: N }` (mask provided separately)
/// - `PbsXXX` → `LutId { id }` (looked up in LUT_ALIASES)
/// - `FN` → `UserFlag { flag: N }`
/// - `NN` → `VirtId { id: N }`
fn parse_argument(s: &str, reg_mask: usize) -> Result<Argument, String> {
    // PtConst: plain number
    if let Ok(val) = s.parse::<u8>() {
        return Ok(Argument::PtConst { val });
    }

    // CtHeap: TH.N
    if let Some(rest) = s.strip_prefix("TH.") {
        let addr = rest
            .parse::<usize>()
            .map_err(|_| format!("invalid heap address: {rest}"))?;
        return Ok(Argument::CtHeap { addr });
    }

    // CtIo: @N
    if let Some(rest) = s.strip_prefix('@') {
        let addr = rest
            .parse::<usize>()
            .map_err(|_| format!("invalid I/O address: {rest}"))?;
        return Ok(Argument::CtIo { addr });
    }

    // CtSrcVar: TS[N].M
    if let Some(rest) = s.strip_prefix("TS[") {
        let (id, block) = parse_var_bracket(rest)?;
        return Ok(Argument::CtSrcVar { id, block });
    }

    // CtDstVar: TD[N].M
    if let Some(rest) = s.strip_prefix("TD[") {
        let (id, block) = parse_var_bracket(rest)?;
        return Ok(Argument::CtDstVar { id, block });
    }

    // PtSrcVar: TI[N].M
    if let Some(rest) = s.strip_prefix("TI[") {
        let (id, block) = parse_var_bracket(rest)?;
        return Ok(Argument::PtSrcVar { id, block });
    }

    // CtReg: RN
    if let Some(rest) = s.strip_prefix('R') {
        let addr = rest
            .parse::<usize>()
            .map_err(|_| format!("invalid register address: {rest}"))?;
        return Ok(Argument::CtReg {
            mask: reg_mask,
            addr,
        });
    }

    // LutId: PbsXXX
    if let Some(alias) = s.strip_prefix("Pbs") {
        let id = LUT_ALIASES
            .iter()
            .position(|&a| a == alias)
            .ok_or_else(|| format!("unknown LUT alias: {alias}"))?;
        return Ok(Argument::LutId { id });
    }

    Err(format!("unrecognized argument: {s}"))
}

/// Parses `N].M` from a variable reference like `TS[N].M`.
fn parse_var_bracket(s: &str) -> Result<(usize, usize), String> {
    let (id_str, rest) = s
        .split_once(']')
        .ok_or_else(|| "missing ']' in variable".to_string())?;
    let rest = rest
        .strip_prefix('.')
        .ok_or_else(|| "missing '.' after ']'".to_string())?;
    let id = id_str
        .parse::<usize>()
        .map_err(|_| format!("invalid variable id: {id_str}"))?;
    let block = rest
        .parse::<usize>()
        .map_err(|_| format!("invalid block index: {rest}"))?;
    Ok((id, block))
}

/// Parses a textual assembly listing into a DOP instruction stream.
///
/// Each non-empty line must contain `OPCODE arg0 arg1 ...` (whitespace-separated). Empty lines and
/// lines containing only whitespace are ignored. A synthetic `_INIT` instruction is prepended to
/// produce the initial context token.
///
/// Register masks for `CtReg` arguments are inferred from instruction type:
/// - `PBS_ML2`, `PBS_ML2_F` → `MASK_PBS2`
/// - `PBS_ML4`, `PBS_ML4_F` → `MASK_PBS4`
/// - `PBS_ML8`, `PBS_ML8_F` → `MASK_PBS8`
/// - All others → `MASK_NONE`
pub fn parse_assembly(input: &str) -> Result<IR<DopLang>, ParseError> {
    let mut ir = IR::empty();

    // Synthesize _INIT to get the initial context token.
    let (_, rets) = ir.add_op(DopInstructionSet::_INIT, svec![]);
    let mut ctx = rets[0];

    for (line_idx, line) in input.lines().enumerate() {
        let line_num = line_idx + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let opcode = tokens.next().ok_or_else(|| ParseError {
            line: line_num,
            message: "empty line".to_string(),
        })?;

        let args: Vec<&str> = tokens.collect();

        // Determine the register mask based on opcode (for destination registers).
        let dst_mask = match opcode {
            "PBS_ML2" | "PBS_ML2_F" => MASK_PBS2,
            "PBS_ML4" | "PBS_ML4_F" => MASK_PBS4,
            "PBS_ML8" | "PBS_ML8_F" => MASK_PBS8,
            _ => MASK_NONE,
        };

        let parse_arg = |idx: usize, mask: usize| -> Result<Argument, ParseError> {
            let token = args.get(idx).ok_or_else(|| ParseError {
                line: line_num,
                message: format!("expected argument at position {idx}"),
            })?;
            parse_argument(token, mask).map_err(|e| ParseError {
                line: line_num,
                message: e,
            })
        };

        let instr = match opcode {
            "ADD" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src1 = parse_arg(1, MASK_NONE)?;
                let src2 = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::ADD { dst, src1, src2 }
            }
            "SUB" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src1 = parse_arg(1, MASK_NONE)?;
                let src2 = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::SUB { dst, src1, src2 }
            }
            "MAC" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src1 = parse_arg(1, MASK_NONE)?;
                let src2 = parse_arg(2, MASK_NONE)?;
                let cst = parse_arg(3, MASK_NONE)?;
                DopInstructionSet::MAC {
                    dst,
                    src1,
                    src2,
                    cst,
                }
            }
            "ADDS" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src = parse_arg(1, MASK_NONE)?;
                let cst = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::ADDS { dst, src, cst }
            }
            "SUBS" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src = parse_arg(1, MASK_NONE)?;
                let cst = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::SUBS { dst, src, cst }
            }
            "SSUB" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src = parse_arg(1, MASK_NONE)?;
                let cst = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::SSUB { dst, src, cst }
            }
            "MULS" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src = parse_arg(1, MASK_NONE)?;
                let cst = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::MULS { dst, src, cst }
            }
            "LD" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src = parse_arg(1, MASK_NONE)?;
                DopInstructionSet::LD { dst, src }
            }
            "ST" => {
                let dst = parse_arg(0, MASK_NONE)?;
                let src = parse_arg(1, MASK_NONE)?;
                DopInstructionSet::ST { dst, src }
            }
            "PBS" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS { dst, src, lut }
            }
            "PBS_ML2" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_ML2 { dst, src, lut }
            }
            "PBS_ML4" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_ML4 { dst, src, lut }
            }
            "PBS_ML8" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_ML8 { dst, src, lut }
            }
            "PBS_F" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_F { dst, src, lut }
            }
            "PBS_ML2_F" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_ML2_F { dst, src, lut }
            }
            "PBS_ML4_F" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_ML4_F { dst, src, lut }
            }
            "PBS_ML8_F" => {
                let dst = parse_arg(0, dst_mask)?;
                let src = parse_arg(1, MASK_NONE)?;
                let lut = parse_arg(2, MASK_NONE)?;
                DopInstructionSet::PBS_ML8_F { dst, src, lut }
            }
            "SYNC" => DopInstructionSet::SYNC,
            _ => {
                return Err(ParseError {
                    line: line_num,
                    message: format!("unknown opcode: {opcode}"),
                });
            }
        };

        let (_, rets) = ir.add_op(instr, svec![ctx]);
        ctx = rets[0];
    }

    Ok(ir)
}

#[cfg(test)]
mod test {
    use super::parse_assembly;

    #[test]
    fn test_load() {
        parse_assembly(TEST_STRING).unwrap();
    }

    const TEST_STRING: &str = "
    LD        R0               TS[1].0
    LD        R1               TS[0].0
    MAC       R2               R0               R1               4
    PBS       R3               R2               PbsShiftRightByCarryPos0Msg
    PBS       R4               R2               PbsShiftRightByCarryPos0MsgNext
    LD        R5               TS[0].1
    MAC       R6               R0               R5               4
    PBS       R7               R6               PbsShiftRightByCarryPos0Msg
    PBS       R8               R6               PbsShiftRightByCarryPos0MsgNext
    LD        R9               TS[0].2
    MAC       R10              R0               R9               4
    PBS       R11              R10              PbsShiftRightByCarryPos0Msg
    PBS       R12              R10              PbsShiftRightByCarryPos0MsgNext
    LD        R13              TS[0].3
    MAC       R14              R0               R13              4
    PBS       R15              R14              PbsShiftRightByCarryPos0Msg
    PBS       R16              R14              PbsShiftRightByCarryPos0MsgNext
    LD        R17              TS[0].4
    MAC       R18              R0               R17              4
    PBS       R19              R18              PbsShiftRightByCarryPos0Msg
    PBS       R20              R18              PbsShiftRightByCarryPos0MsgNext
    LD        R21              TS[0].5
    ST        TH.113           R46
    LD        R38              TH.111
    MAC       R49              R44              R38              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.114           R50
    PBS       R50              R49              PbsIfPos1FalseZeroed
    ST        TH.115           R54
    ADD       R54              R52              R50
    MAC       R49              R44              R38              4
    LD        R50              TH.113
    MAC       R52              R44              R50              4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.116           R62
    PBS       R62              R52              PbsIfPos1FalseZeroed
    ST        TH.117           R10
    ADD       R10              R53              R62
    MAC       R52              R44              R50              4
    LD        R62              TH.115
    MAC       R53              R44              R62              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.118           R26
    PBS       R26              R53              PbsIfPos1FalseZeroed
    ST        TH.119           R58
    ADD       R58              R49              R26
    MAC       R53              R44              R62              4
    LD        R26              TH.117
    MAC       R49              R44              R26              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.120           R1
    PBS       R1               R49              PbsIfPos1FalseZeroed
    ST        TH.121           R42
    ADD       R42              R52              R1
    MAC       R49              R44              R26              4
    LD        R1               TH.119
    MAC       R52              R44              R1               4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.122           R4
    PBS       R4               R52              PbsIfPos1FalseZeroed
    ST        TH.123           R5
    ADD       R5               R53              R4
    MAC       R52              R44              R1               4
    LD        R4               TH.121
    MAC       R53              R44              R4               4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.124           R3
    PBS       R3               R53              PbsIfPos1FalseZeroed
    ST        TH.125           R8
    ADD       R8               R49              R3
    MAC       R53              R44              R4               4
    LD        R3               TH.123
    MAC       R49              R44              R3               4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.126           R9
    PBS       R9               R49              PbsIfPos1FalseZeroed
    ST        TH.127           R7
    ADD       R7               R52              R9
    MAC       R49              R44              R3               4
    LD        R9               TH.125
    MAC       R52              R44              R9               4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.128           R12
    PBS       R12              R52              PbsIfPos1FalseZeroed
    ST        TH.129           R13
    ADD       R13              R53              R12
    MAC       R52              R44              R9               4
    LD        R12              TH.127
    MAC       R53              R44              R12              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.130           R11
    PBS       R11              R53              PbsIfPos1FalseZeroed
    ST        TH.131           R16
    ADD       R16              R49              R11
    MAC       R53              R44              R12              4
    LD        R11              TH.129
    MAC       R49              R44              R11              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.132           R17
    PBS       R17              R49              PbsIfPos1FalseZeroed
    ST        TH.133           R15
    ADD       R15              R52              R17
    MAC       R49              R44              R11              4
    LD        R17              TH.131
    MAC       R52              R44              R17              4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.134           R20
    PBS       R20              R52              PbsIfPos1FalseZeroed
    ST        TH.135           R21
    ADD       R21              R53              R20
    MAC       R52              R44              R17              4
    LD        R20              TH.133
    MAC       R53              R44              R20              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.136           R19
    PBS       R19              R53              PbsIfPos1FalseZeroed
    ST        TH.137           R24
    ADD       R24              R49              R19
    MAC       R53              R44              R20              4
    LD        R19              TH.135
    MAC       R49              R44              R19              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.138           R25
    PBS       R25              R49              PbsIfPos1FalseZeroed
    ST        TH.139           R23
    ADD       R23              R52              R25
    MAC       R49              R44              R19              4
    LD        R25              TH.137
    MAC       R52              R44              R25              4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.140           R28
    PBS       R28              R52              PbsIfPos1FalseZeroed
    ST        TH.141           R29
    ADD       R29              R53              R28
    MAC       R52              R44              R25              4
    LD        R28              TH.139
    MAC       R53              R44              R28              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.142           R27
    PBS       R27              R53              PbsIfPos1FalseZeroed
    ST        TH.143           R32
    ADD       R32              R49              R27
    MAC       R53              R44              R28              4
    LD        R27              TH.141
    MAC       R49              R44              R27              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.144           R33
    PBS       R33              R49              PbsIfPos1FalseZeroed
    ST        TH.145           R31
    ADD       R31              R52              R33
    MAC       R49              R44              R27              4
    LD        R33              TH.143
    MAC       R52              R44              R33              4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.146           R36
    PBS       R36              R52              PbsIfPos1FalseZeroed
    ST        TH.147           R37
    ADD       R37              R53              R36
    MAC       R52              R44              R33              4
    LD        R36              TH.145
    MAC       R53              R44              R36              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.148           R35
    PBS       R35              R53              PbsIfPos1FalseZeroed
    ST        TH.149           R40
    ADD       R40              R49              R35
    MAC       R53              R44              R36              4
    LD        R35              TH.147
    MAC       R49              R44              R35              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.150           R41
    PBS       R41              R49              PbsIfPos1FalseZeroed
    ST        TH.151           R0
    ADD       R0               R52              R41
    MAC       R49              R44              R35              4
    LD        R41              TH.149
    MAC       R52              R44              R41              4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.152           R45
    PBS       R45              R52              PbsIfPos1FalseZeroed
    ST        TH.153           R39
    ADD       R39              R53              R45
    MAC       R52              R44              R41              4
    LD        R45              TH.151
    MAC       R53              R44              R45              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.154           R43
    PBS       R43              R53              PbsIfPos1FalseZeroed
    ST        TH.155           R47
    ADD       R47              R49              R43
    MAC       R53              R44              R45              4
    LD        R43              TH.153
    MAC       R49              R44              R43              4
    PBS       R52              R53              PbsIfPos1TrueZeroed
    ST        TH.156           R48
    PBS       R48              R49              PbsIfPos1FalseZeroed
    ST        TH.157           R56
    ADD       R56              R52              R48
    MAC       R49              R44              R43              4
    LD        R48              TH.155
    MAC       R52              R44              R48              4
    PBS       R53              R49              PbsIfPos1TrueZeroed
    ST        TH.158           R51
    PBS       R51              R52              PbsIfPos1FalseZeroed
    ST        TH.159           R59
    ADD       R59              R53              R51
    MAC       R52              R44              R48              4
    LD        R51              TH.156
    MAC       R53              R44              R51              4
    PBS       R49              R52              PbsIfPos1TrueZeroed
    ST        TH.160           R55
    PBS       R55              R53              PbsIfPos1FalseZeroed
    ST        TH.161           R61
    ADD       R61              R49              R55
    LD        R53              TS[1].1
    LD        R55              TH.157
    MAC       R49              R53              R55              4
    LD        R52              TH.161
    MAC       R51              R53              R52              4
    PBS       R57              R49              PbsIfPos0TrueZeroed
    PBS       R60              R51              PbsIfPos0FalseZeroed
    ADD       R63              R57              R60
    LD        R6               TH.159
    MAC       R18              R53              R6               4
    MAC       R30              R53              R2               4
    PBS       R38              R18              PbsIfPos0TrueZeroed
    PBS       R50              R30              PbsIfPos0FalseZeroed
    ADD       R62              R38              R50
    MAC       R26              R53              R52              4
    MAC       R1               R53              R14              4
    PBS       R4               R26              PbsIfPos0TrueZeroed
    PBS       R3               R1               PbsIfPos0FalseZeroed
    ADD       R9               R4               R3
    MAC       R12              R53              R2               4
    MAC       R11              R53              R22              4
    PBS       R17              R12              PbsIfPos0TrueZeroed
    PBS       R20              R11              PbsIfPos0FalseZeroed
    ADD       R19              R17              R20
    MAC       R25              R53              R14              4
    MAC       R28              R53              R34              4
    PBS       R27              R25              PbsIfPos0TrueZeroed
    PBS       R33              R28              PbsIfPos0FalseZeroed
    ADD       R36              R27              R33
    MAC       R35              R53              R22              4
    MAC       R41              R53              R46              4
    PBS       R45              R35              PbsIfPos0TrueZeroed
    PBS       R43              R41              PbsIfPos0FalseZeroed
    ADD       R33              R45              R49
    MAC       R28              R8               R40              4
    MAC       R41              R8               R15              4
    PBS       R51              R28              PbsIfPos0TrueZeroed
    PBS       R50              R41              PbsIfPos0FalseZeroed
    ADD       R56              R51              R50
    MAC       R44              R8               R59              4
    MAC       R55              R8               R32              4
    PBS       R6               R44              PbsIfPos0TrueZeroed
    PBS       R52              R55              PbsIfPos0FalseZeroed
    ADD       R2               R6               R52
    MAC       R61              R8               R9               4
    MAC       R62              R8               R29              4
    PBS       R16              R61              PbsIfPos0TrueZeroed
    PBS       R21              R62              PbsIfPos0FalseZeroed
    ADD       R30              R16              R21
    MAC       R17              R8               R5               4
    MAC       R3               R8               R46              4
    PBS       R48              R17              PbsIfPos0TrueZeroed
    PBS       R11              R3               PbsIfPos0FalseZeroed
    ADD       R57              R48              R11
    MAC       R12              R8               R24              4
    MAC       R1               R8               R37              4
    PBS       R18              R12              PbsIfPos0TrueZeroed
    PBS       R49              R1               PbsIfPos0FalseZeroed
    ADD       R45              R18              R49
    MAC       R43              R8               R31              4
    MAC       R41              R8               R47              4
    PBS       R50              R43              PbsIfPos0TrueZeroed
    PBS       R51              R41              PbsIfPos0FalseZeroed
    ADD       R28              R50              R51
    MAC       R55              R8               R0               4
    MAC       R52              R8               R38              4
    PBS       R6               R55              PbsIfPos0TrueZeroed
    PBS       R44              R52              PbsIfPos0FalseZeroed
    ADD       R62              R6               R44
    MAC       R21              R8               R54              4
    MAC       R16              R8               R42              4
    PBS       R61              R21              PbsIfPos0TrueZeroed
    PBS       R3               R16              PbsIfPos0FalseZeroed
    ADD       R11              R61              R3
    MAC       R48              R8               R15              4
    MAC       R17              R8               R63              4
    PBS       R1               R48              PbsIfPos0TrueZeroed
    PBS       R49              R17              PbsIfPos0FalseZeroed
    ADD       R18              R1               R49
    MAC       R12              R8               R32              4
    MAC       R41              R8               R22              4
    PBS       R51              R12              PbsIfPos0TrueZeroed
    PBS       R50              R41              PbsIfPos0FalseZeroed
    ADD       R43              R51              R50
    MAC       R52              R8               R29              4
    MAC       R44              R8               R26              4
    PBS       R6               R52              PbsIfPos0TrueZeroed
    PBS       R55              R44              PbsIfPos0FalseZeroed
    ADD       R16              R6               R55
    MAC       R3               R8               R46              4
    MAC       R61              R8               R39              4
    PBS       R21              R3               PbsIfPos0TrueZeroed
    PBS       R17              R61              PbsIfPos0FalseZeroed
    ADD       R49              R21              R17
    MAC       R1               R8               R37              4
    MAC       R48              R8               R35              4
    PBS       R41              R1               PbsIfPos0TrueZeroed
    ST        TD[0].31         R34
    ST        TD[0].30         R42
    ST        TD[0].29         R32
    ST        TD[0].28         R5
    ST        TD[0].27         R13
    ST        TD[0].26         R15
    ST        TD[0].25         R40
    ST        TD[0].24         R3
    ST        TD[0].23         R4
    ST        TD[0].22         R58
    ST        TD[0].21         R27
    ST        TD[0].20         R23
    ST        TD[0].19         R31
    ST        TD[0].18         R1
    ST        TD[0].17         R14
    ST        TD[0].16         R35
    ST        TD[0].15         R59
    ";
}
