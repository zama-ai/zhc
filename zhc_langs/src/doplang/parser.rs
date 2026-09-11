//! Text assembly parsing for the DOP dialect — the reverse of [`emit_assembly`](super::emit_assembly).
//!
//! Reconstructs a context-threaded [`IR<DopLang>`] graph from a `dop.asm` listing: one
//! instruction per line, `OPCODE arg0 arg1 ...`, blank lines and lines whose first
//! non-whitespace character is `;` or `#` are ignored. Each operand is parsed by a small
//! recursive-descent [`Cursor`] over the operand grammar mirrored from each operand type's own
//! `.asm(...)` method (e.g. [`CtReg::asm`](super::CtReg::asm)), then
//! checked against the operand kind each mnemonic expects at that position (e.g. `ADD`'s three
//! operands must all be registers) — mirroring the strictness of
//! `tfhe_hpu_backend::asm::dop::arg::FromAsm`, so e.g. `ADD R2 R4 TI[4].2` is rejected even
//! though `TI[4].2` parses fine as a token in isolation.

use std::fmt;

use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::IR;
use zhc_utils::svec;

use crate::doplang::{
    CtDstVar, CtHeap, CtIo, CtMem, CtReg, CtSrcVar, DopInstructionSet, DopLang, LutRef, MASK_PBS2,
    MASK_PBS4, MASK_PBS8, PtArg, PtConst, PtSrcVar, UserFlag, VirtId,
};

/// A parsed operand token of not-yet-determined kind.
///
/// Not the same `Argument` `DopInstructionSet` used to expose before its operand fields were
/// split into standalone types (see `instruction_set.rs`): this one is private to the parser,
/// existing only because [`parse_argument`] can't know which concrete kind a token names until
/// it has matched the token's grammar. [`parse_instruction`]'s `reg`/`mem`/`lut`/`user_flag`/
/// `virt_id`/`imm_any` helpers immediately narrow it down to the concrete type (or narrower
/// `CtMem`/`PtArg`) each field actually requires.
#[derive(Debug, Clone, Copy)]
enum Argument {
    PtConst(PtConst),
    CtHeap(CtHeap),
    CtIo(CtIo),
    CtSrcVar(CtSrcVar),
    CtDstVar(CtDstVar),
    PtSrcVar(PtSrcVar),
    CtReg(CtReg),
    LutRef(LutRef),
    UserFlag(UserFlag),
    VirtId(VirtId),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::PtConst(inner) => fmt::Display::fmt(inner, f),
            Argument::CtHeap(inner) => fmt::Display::fmt(inner, f),
            Argument::CtIo(inner) => fmt::Display::fmt(inner, f),
            Argument::CtSrcVar(inner) => fmt::Display::fmt(inner, f),
            Argument::CtDstVar(inner) => fmt::Display::fmt(inner, f),
            Argument::PtSrcVar(inner) => fmt::Display::fmt(inner, f),
            Argument::CtReg(inner) => fmt::Display::fmt(inner, f),
            Argument::LutRef(inner) => fmt::Display::fmt(inner, f),
            Argument::UserFlag(inner) => fmt::Display::fmt(inner, f),
            Argument::VirtId(inner) => fmt::Display::fmt(inner, f),
        }
    }
}

impl From<PtConst> for Argument {
    fn from(v: PtConst) -> Self {
        Self::PtConst(v)
    }
}
impl From<CtHeap> for Argument {
    fn from(v: CtHeap) -> Self {
        Self::CtHeap(v)
    }
}
impl From<CtIo> for Argument {
    fn from(v: CtIo) -> Self {
        Self::CtIo(v)
    }
}
impl From<CtSrcVar> for Argument {
    fn from(v: CtSrcVar) -> Self {
        Self::CtSrcVar(v)
    }
}
impl From<CtDstVar> for Argument {
    fn from(v: CtDstVar) -> Self {
        Self::CtDstVar(v)
    }
}
impl From<PtSrcVar> for Argument {
    fn from(v: PtSrcVar) -> Self {
        Self::PtSrcVar(v)
    }
}
impl From<CtReg> for Argument {
    fn from(v: CtReg) -> Self {
        Self::CtReg(v)
    }
}
impl From<LutRef> for Argument {
    fn from(v: LutRef) -> Self {
        Self::LutRef(v)
    }
}
impl From<UserFlag> for Argument {
    fn from(v: UserFlag) -> Self {
        Self::UserFlag(v)
    }
}
impl From<VirtId> for Argument {
    fn from(v: VirtId) -> Self {
        Self::VirtId(v)
    }
}

/// Line-prefixes that mark a whole line as a comment, mirroring
/// `tfhe_hpu_backend::asm::ASM_COMMENT_PREFIX`.
pub(super) const COMMENT_PREFIX: [char; 2] = [';', '#'];

/// Error produced while parsing a `dop.asm` listing.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dop.asm:{}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parses the instruction listing of a `dop.asm` file (i.e. everything after its mandatory
/// `!preamble { ... }` block) into a context-threaded [`IR<DopLang>`] graph.
///
/// Not exported outside `doplang`: the only public entry point is
/// [`parse_assembly`](super::preamble::parse_assembly), which derives `lreg` from the file's
/// own `[lut]` preamble section rather than accepting one from the caller.
///
/// `lreg` resolves `Pbs<Name>` operands to the [`LutId`](zhc_crypto::integer_semantics::lut::LutId)
/// registered under that name; every LUT referenced by the listing must already be registered.
///
/// The DOP dialect threads a single `Ctx` value through the whole stream: a synthesized
/// `_START` produces the initial token, every parsed instruction consumes and re-produces it in
/// program order, and a synthesized `_END` consumes the final one.
pub(super) fn parse_instructions(src: &str, lreg: &LutRegistry) -> Result<IR<DopLang>, ParseError> {
    let mut ir: IR<DopLang> = IR::empty();
    let (_, start_rets) = ir.add_op(DopInstructionSet::_START, svec![]);
    let mut ctx = start_rets[0];

    for (idx, raw_line) in src.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with(COMMENT_PREFIX) {
            continue;
        }

        let instr = parse_instruction(line, lreg).map_err(|message| ParseError {
            line: idx + 1,
            message,
        })?;
        let (_, rets) = ir.add_op(instr, svec![ctx]);
        ctx = rets[0];
    }

    ir.add_op(DopInstructionSet::_END, svec![ctx]);
    Ok(ir)
}

/// Parses a single non-empty, non-comment line into a [`DopInstructionSet`] operation.
fn parse_instruction(line: &str, lreg: &LutRegistry) -> Result<DopInstructionSet, String> {
    use DopInstructionSet::*;

    let mut tokens = line.split_whitespace();
    let mnemonic = tokens.next().expect("line was checked non-empty");
    let operands: Vec<&str> = tokens.collect();

    let arg = |i: usize| -> Result<Argument, String> {
        let tok = operands
            .get(i)
            .ok_or_else(|| format!("{mnemonic}: missing operand {i}"))?;
        parse_argument(tok, lreg)
    };
    let expect_arity = |n: usize| -> Result<(), String> {
        if operands.len() == n {
            Ok(())
        } else {
            Err(format!(
                "{mnemonic}: expected {n} operand(s), got {}",
                operands.len()
            ))
        }
    };
    // Reinterprets a many-LUT destination as covering a group of `n` consecutive registers.
    // The textual form (`R<addr>`) never carries the mask: it is implied by the opcode.
    let masked_dst = |a: CtReg, mask: usize| CtReg { mask, addr: a.addr };

    // Per-operand kind checks, mirroring the strictness of
    // `tfhe_hpu_backend::asm::dop::arg::FromAsm` (e.g. `PeArithInsn::from_args` rejects
    // anything but `Arg::Reg` for its register operands). `parse_argument` alone can't tell
    // `ADD R2 R4 TI[4].2` apart from a valid line: both parse fine as individual tokens, but
    // the third operand resolves to a `PtSrcVar`, not a `CtReg`, so it must be rejected here.
    // Unlike before this refactor, the accepted kind is now also enforced by the return type:
    // `DopInstructionSet::ADD` simply has no field a `PtSrcVar` could be assigned to.
    let reg = |i: usize| -> Result<CtReg, String> {
        let a = arg(i)?;
        match a {
            Argument::CtReg(inner) => Ok(inner),
            other => Err(format!(
                "{mnemonic}: operand {i} must be a register (R<n>), got `{other}`"
            )),
        }
    };
    let mem = |i: usize| -> Result<CtMem, String> {
        let a = arg(i)?;
        match a {
            Argument::CtHeap(inner) => Ok(CtMem::Heap(inner)),
            Argument::CtIo(inner) => Ok(CtMem::Io(inner)),
            Argument::CtSrcVar(inner) => Ok(CtMem::Src(inner)),
            Argument::CtDstVar(inner) => Ok(CtMem::Dst(inner)),
            other => Err(format!(
                "{mnemonic}: operand {i} must be a memory location (@.., TH.., TS[..]/TD[..]), got `{other}`"
            )),
        }
    };
    let lut = |i: usize| -> Result<LutRef, String> {
        let a = arg(i)?;
        match a {
            Argument::LutRef(inner) => Ok(inner),
            other => Err(format!(
                "{mnemonic}: operand {i} must be a LUT name (Pbs<Name>), got `{other}`"
            )),
        }
    };
    let user_flag = |i: usize| -> Result<UserFlag, String> {
        let a = arg(i)?;
        match a {
            Argument::UserFlag(inner) => Ok(inner),
            other => Err(format!(
                "{mnemonic}: operand {i} must be a flag (F<n>), got `{other}`"
            )),
        }
    };
    let virt_id = |i: usize| -> Result<VirtId, String> {
        let a = arg(i)?;
        match a {
            Argument::VirtId(inner) => Ok(inner),
            other => Err(format!(
                "{mnemonic}: operand {i} must be a board id (N<n>), got `{other}`"
            )),
        }
    };
    // Ciphertext-scalar arithmetic (`ADDS`/`SUBS`/`SSUB`/`MULS`/`MAC`) accepts either a literal
    // constant or a patchable template, matching `PeArithMsgInsn`/`PeArithInsn::from_args`'s
    // `Arg::Imm(id)` (in practice a bare constant is by far the common case for `MAC`, but the
    // hardware field accepts a template just as well).
    let imm_any = |i: usize| -> Result<PtArg, String> {
        let a = arg(i)?;
        match a {
            Argument::PtConst(inner) => Ok(PtArg::Const(inner)),
            Argument::PtSrcVar(inner) => Ok(PtArg::Var(inner)),
            other => Err(format!(
                "{mnemonic}: operand {i} must be a plaintext immediate (constant or TI[..]), got `{other}`"
            )),
        }
    };
    match mnemonic {
        "ADD" => {
            expect_arity(3)?;
            Ok(ADD {
                dst: reg(0)?,
                src1: reg(1)?,
                src2: reg(2)?,
            })
        }
        "SUB" => {
            expect_arity(3)?;
            Ok(SUB {
                dst: reg(0)?,
                src1: reg(1)?,
                src2: reg(2)?,
            })
        }
        "MAC" => {
            expect_arity(4)?;
            Ok(MAC {
                dst: reg(0)?,
                src1: reg(1)?,
                src2: reg(2)?,
                cst: imm_any(3)?,
            })
        }
        "ADDS" => {
            expect_arity(3)?;
            Ok(ADDS {
                dst: reg(0)?,
                src: reg(1)?,
                cst: imm_any(2)?,
            })
        }
        "SUBS" => {
            expect_arity(3)?;
            Ok(SUBS {
                dst: reg(0)?,
                src: reg(1)?,
                cst: imm_any(2)?,
            })
        }
        "SSUB" => {
            expect_arity(3)?;
            Ok(SSUB {
                dst: reg(0)?,
                src: reg(1)?,
                cst: imm_any(2)?,
            })
        }
        "MULS" => {
            expect_arity(3)?;
            Ok(MULS {
                dst: reg(0)?,
                src: reg(1)?,
                cst: imm_any(2)?,
            })
        }
        "LD" => {
            expect_arity(2)?;
            Ok(LD {
                dst: reg(0)?,
                src: mem(1)?,
            })
        }
        "ST" => {
            expect_arity(2)?;
            Ok(ST {
                dst: mem(0)?,
                src: reg(1)?,
            })
        }
        "PBS" => {
            expect_arity(3)?;
            Ok(PBS {
                dst: reg(0)?,
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_ML2" => {
            expect_arity(3)?;
            Ok(PBS_ML2 {
                dst: masked_dst(reg(0)?, MASK_PBS2),
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_ML4" => {
            expect_arity(3)?;
            Ok(PBS_ML4 {
                dst: masked_dst(reg(0)?, MASK_PBS4),
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_ML8" => {
            expect_arity(3)?;
            Ok(PBS_ML8 {
                dst: masked_dst(reg(0)?, MASK_PBS8),
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_F" => {
            expect_arity(3)?;
            Ok(PBS_F {
                dst: reg(0)?,
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_ML2_F" => {
            expect_arity(3)?;
            Ok(PBS_ML2_F {
                dst: masked_dst(reg(0)?, MASK_PBS2),
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_ML4_F" => {
            expect_arity(3)?;
            Ok(PBS_ML4_F {
                dst: masked_dst(reg(0)?, MASK_PBS4),
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "PBS_ML8_F" => {
            expect_arity(3)?;
            Ok(PBS_ML8_F {
                dst: masked_dst(reg(0)?, MASK_PBS8),
                src: reg(1)?,
                lut: lut(2)?,
            })
        }
        "SYNC" => {
            expect_arity(0)?;
            Ok(SYNC)
        }
        "WAIT" => match operands.len() {
            1 => Ok(WAIT {
                flag: user_flag(0)?,
                slot: None,
            }),
            2 => Ok(WAIT {
                flag: user_flag(0)?,
                slot: Some(mem(1)?),
            }),
            n => Err(format!("WAIT: expected 1 or 2 operand(s), got {n}")),
        },
        "NOTIFY" => {
            expect_arity(3)?;
            Ok(NOTIFY {
                virt_id: virt_id(0)?,
                flag: user_flag(1)?,
                slot: mem(2)?,
            })
        }
        "LD_B2B" => {
            expect_arity(2)?;
            Ok(LD_B2B {
                flag: user_flag(0)?,
                slot: mem(1)?,
            })
        }
        other => Err(format!("unknown opcode `{other}`")),
    }
}

/// Parses a single operand token into an [`Argument`], dispatching on its leading grammar
/// production. The productions are prefix-disjoint (`R`, `TH.`, `@`, `TS[`, `TD[`, `TI[`,
/// `Pbs`, `F`, `N`, or a bare integer), so a linear cascade of attempts is unambiguous.
fn parse_argument(tok: &str, lreg: &LutRegistry) -> Result<Argument, String> {
    let mut c = Cursor::new(tok);

    if c.eat_str("TH.") {
        let addr = c.parse_uint(tok, "CT_H address")?;
        c.expect_eof(tok)?;
        return Ok(CtHeap::new(addr).into());
    }
    if c.eat_str("TS[") {
        let id = c.parse_uint(tok, "CT_S id")?;
        c.expect_char(']', tok)?;
        c.expect_char('.', tok)?;
        let block = c.parse_uint(tok, "CT_S block")?;
        c.expect_eof(tok)?;
        return Ok(CtSrcVar::new(id, block).into());
    }
    if c.eat_str("TD[") {
        let id = c.parse_uint(tok, "CT_D id")?;
        c.expect_char(']', tok)?;
        c.expect_char('.', tok)?;
        let block = c.parse_uint(tok, "CT_D block")?;
        c.expect_eof(tok)?;
        return Ok(CtDstVar::new(id, block).into());
    }
    if c.eat_str("TI[") {
        let id = c.parse_uint(tok, "PT_I id")?;
        c.expect_char(']', tok)?;
        c.expect_char('.', tok)?;
        let block = c.parse_uint(tok, "PT_I block")?;
        c.expect_eof(tok)?;
        return Ok(PtSrcVar::new(id, block).into());
    }
    if c.eat_str("R") {
        let addr = c.parse_uint::<usize>(tok, "register address")?;
        c.expect_eof(tok)?;
        return Ok(CtReg::new(addr).into());
    }
    if c.eat_str("@") {
        let addr = c.parse_uint(tok, "CT_IO address")?;
        c.expect_eof(tok)?;
        return Ok(CtIo::new(addr).into());
    }
    if c.eat_str("Pbs") {
        let name = c.rest();
        if name.is_empty() {
            return Err(format!("`{tok}`: expected a LUT name after `Pbs`"));
        }
        let id = lookup_lut_id(lreg, name)
            .ok_or_else(|| format!("`{tok}`: no LUT named `{name}` is registered"))?;
        return Ok(LutRef::new(id).into());
    }
    if c.eat_str("F") {
        let flag = c.parse_uint(tok, "flag id")?;
        c.expect_eof(tok)?;
        return Ok(UserFlag::new(flag).into());
    }
    if c.eat_str("N") {
        let id = c.parse_uint(tok, "board id")?;
        c.expect_eof(tok)?;
        return Ok(VirtId::new(id).into());
    }
    let val = c.parse_uint(tok, "plaintext constant")?;
    c.expect_eof(tok)?;
    Ok(PtConst::new(val).into())
}

/// Resolves a LUT name (as it appears after the `Pbs` prefix) to its registered id.
fn lookup_lut_id(lreg: &LutRegistry, name: &str) -> Option<usize> {
    lreg.iter_luts()
        .find(|(_, raw)| raw.name() == name)
        .map(|(lid, _)| lid.0)
}

/// A minimal cursor over an operand token, supporting the small set of productions
/// (`str` literals, decimal/hex unsigned integers, single-character delimiters) needed by
/// [`parse_argument`].
struct Cursor<'a> {
    rest: &'a str,
}

impl<'a> Cursor<'a> {
    fn new(s: &'a str) -> Self {
        Self { rest: s }
    }

    fn rest(&self) -> &'a str {
        self.rest
    }

    /// Consumes `lit` if `rest` starts with it.
    fn eat_str(&mut self, lit: &str) -> bool {
        if let Some(rest) = self.rest.strip_prefix(lit) {
            self.rest = rest;
            true
        } else {
            false
        }
    }

    /// Consumes a single expected character.
    fn expect_char(&mut self, expected: char, tok: &str) -> Result<(), String> {
        let mut chars = self.rest.chars();
        match chars.next() {
            Some(c) if c == expected => {
                self.rest = chars.as_str();
                Ok(())
            }
            _ => Err(format!("`{tok}`: expected `{expected}`")),
        }
    }

    /// Fails unless the whole token has been consumed.
    fn expect_eof(&self, tok: &str) -> Result<(), String> {
        if self.rest.is_empty() {
            Ok(())
        } else {
            Err(format!("`{tok}`: unexpected trailing `{}`", self.rest))
        }
    }

    /// Parses an unsigned integer, accepting an optional `0x` hex prefix.
    fn parse_uint<T>(&mut self, tok: &str, what: &str) -> Result<T, String>
    where
        T: TryFrom<u128>,
    {
        let (digits, radix) = if let Some(hex) = self.rest.strip_prefix("0x") {
            let end = hex
                .find(|c: char| !c.is_ascii_hexdigit())
                .unwrap_or(hex.len());
            let (digits, rest) = hex.split_at(end);
            self.rest = rest;
            (digits, 16)
        } else {
            let end = self
                .rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(self.rest.len());
            let (digits, rest) = self.rest.split_at(end);
            self.rest = rest;
            (digits, 10)
        };
        if digits.is_empty() {
            return Err(format!("`{tok}`: expected a {what}"));
        }
        let val = u128::from_str_radix(digits, radix)
            .map_err(|err| format!("`{tok}`: invalid {what}: {err}"))?;
        T::try_from(val).map_err(|_| format!("`{tok}`: {what} out of range"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doplang::emit_assembly;
    use crate::ioplang::Lut1Def;
    use zhc_crypto::integer_semantics::CiphertextBlockSpec;

    fn registry() -> LutRegistry {
        let spec = CiphertextBlockSpec(2, 2);
        let mut lreg = LutRegistry::empty();
        lreg.register_l1(&Lut1Def::None.into_lut(spec));
        lreg.register_l1(&Lut1Def::CarryInMsg.into_lut(spec));
        lreg
    }

    fn instructions(ir: &IR<DopLang>) -> Vec<DopInstructionSet> {
        ir.walk_ops_linear()
            .map(|op| op.get_instruction().clone())
            .filter(|instr| {
                !matches!(instr, DopInstructionSet::_START | DopInstructionSet::_END)
            })
            .collect()
    }

    #[test]
    fn parses_every_instruction_form() {
        let lreg = registry();
        let src = "\
            ADD R2 R1 R3\n\
            SUB R2 R1 R3\n\
            MAC R2 R1 R3 4\n\
            ADDS R2 R1 10\n\
            SUBS R2 R1 TI[4].0\n\
            SSUB R2 R1 TI[2].4\n\
            MULS R2 R1 10\n\
            LD R1 @400\n\
            LD R3 TS[8].4\n\
            ST TD[4].0 R4\n\
            ST TH.60 R4\n\
            PBS R2 R1 PbsNone\n\
            PBS_F R2 R1 PbsCarryInMsg\n\
            PBS_ML2 R4 R1 PbsNone\n\
            SYNC\n\
            WAIT F0\n\
            WAIT F0 TH.0\n\
            NOTIFY N1 F3 TH.2\n\
            LD_B2B F4 TS[0].0\n\
            ; a full-line comment is ignored\n\
        ";

        let ir = parse_instructions(src, &lreg).expect("valid listing must parse");
        let ops = instructions(&ir);
        println!("Parsed Ops: {ops:?}");
        assert_eq!(ops.len(), 19);

        assert_eq!(
            ops[0],
            DopInstructionSet::ADD {
                dst: CtReg::new(2usize),
                src1: CtReg::new(1usize),
                src2: CtReg::new(3usize),
            }
        );
        assert_eq!(
            ops[13],
            DopInstructionSet::PBS_ML2 {
                dst: CtReg::ml2(4usize),
                src: CtReg::new(1usize),
                lut: LutRef::new(0usize),
            }
        );
        assert_eq!(
            ops[16],
            DopInstructionSet::WAIT {
                flag: UserFlag::new(0),
                slot: Some(CtMem::heap(0usize)),
            }
        );
    }

    #[test]
    fn round_trips_through_emit_assembly() {
        let lreg = registry();
        let mut ir: IR<DopLang> = IR::empty();
        let (_, start) = ir.add_op(DopInstructionSet::_START, svec![]);
        let (_, r1) = ir.add_op(
            DopInstructionSet::LD {
                dst: CtReg::new(1usize),
                src: CtMem::heap(3usize),
            },
            svec![start[0]],
        );
        let (_, r2) = ir.add_op(
            DopInstructionSet::PBS {
                dst: CtReg::new(2usize),
                src: CtReg::new(1usize),
                lut: LutRef::new(1usize),
            },
            svec![r1[0]],
        );
        ir.add_op(DopInstructionSet::_END, svec![r2[0]]);

        let asm = emit_assembly(&ir, &lreg);
        let reparsed = parse_instructions(&asm, &lreg).expect("emitted assembly must reparse");

        assert_eq!(instructions(&ir), instructions(&reparsed));
    }

    #[test]
    fn rejects_unknown_lut_name() {
        let lreg = registry();
        let err = parse_instructions("PBS R2 R1 PbsBogus\n", &lreg).unwrap_err();
        assert_eq!(err.line, 1);
    }

    #[test]
    fn rejects_wrong_arity() {
        let lreg = registry();
        let err = parse_instructions("ADD R1 R2\n", &lreg).unwrap_err();
        assert_eq!(err.line, 1);
    }

    #[test]
    fn rejects_immediate_where_register_expected() {
        let lreg = registry();
        let err = parse_instructions("ADD R2 R4 TI[4].2\n", &lreg).unwrap_err();
        assert_eq!(err.line, 1);
        assert!(err.message.contains("operand 2"), "{}", err.message);
        assert!(err.message.contains("register"), "{}", err.message);
    }

    #[test]
    fn rejects_register_where_memory_expected() {
        let lreg = registry();
        // LD's source must be a memory location, not a register.
        let err = parse_instructions("LD R1 R2\n", &lreg).unwrap_err();
        assert_eq!(err.line, 1);
        assert!(err.message.contains("memory location"), "{}", err.message);

        // ST's destination must be a memory location, not a register.
        let err = parse_instructions("ST R1 R2\n", &lreg).unwrap_err();
        assert!(err.message.contains("memory location"), "{}", err.message);
    }

    #[test]
    fn rejects_register_where_lut_expected() {
        let lreg = registry();
        let err = parse_instructions("PBS R2 R1 R3\n", &lreg).unwrap_err();
        assert!(err.message.contains("LUT"), "{}", err.message);
    }

    #[test]
    fn mac_accepts_templated_multiplier() {
        let lreg = registry();
        // MAC's multiplier is usually a literal constant, but a patchable `TI[..]` template is
        // just as valid a hardware field value.
        let ir = parse_instructions("MAC R2 R1 R3 TI[4].0\n", &lreg).expect("valid line must parse");
        assert_eq!(
            instructions(&ir),
            vec![DopInstructionSet::MAC {
                dst: CtReg::new(2usize),
                src1: CtReg::new(1usize),
                src2: CtReg::new(3usize),
                cst: PtArg::var(4, 0),
            }]
        );
    }

    #[test]
    fn mac_rejects_register_multiplier() {
        let lreg = registry();
        let err = parse_instructions("MAC R2 R1 R3 R4\n", &lreg).unwrap_err();
        assert!(err.message.contains("plaintext immediate"), "{}", err.message);
    }

    #[test]
    fn rejects_memory_where_flag_expected() {
        let lreg = registry();
        let err = parse_instructions("WAIT TH.0\n", &lreg).unwrap_err();
        assert!(err.message.contains("flag"), "{}", err.message);
    }
}
