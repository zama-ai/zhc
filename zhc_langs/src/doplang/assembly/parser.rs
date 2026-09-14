//! Text assembly parsing for the DOP dialect — the reverse of
//! [`emit_assembly`](super::emit_assembly).
//!
//! A `dop.asm` file is a mandatory `!preamble { ... }` block followed by an instruction listing.
//! [`parse_assembly`] is the entry point for the whole file; [`parse_preamble`] parses the block
//! alone, for callers whose body is not `dop.asm` text (e.g. a hex listing).
//!
//! # Preamble
//!
//! The `!preamble { ... }` block declares everything the plain instruction grammar cannot express
//! on its own: the external [`Signature<Type>`] of the graph of DOP the file describes, and any
//! literal lookup tables it references by name. Ordinary description comments (naming the
//! operation, documenting expected results, ...) may precede the block; the first non-comment,
//! non-blank line must be the `!preamble {` opener itself.
//!
//! Every preamble line is itself an ordinary ASM comment (prefixed with `;` or `#`), so a
//! listing with a preamble parses unchanged under any tool that only understands the plain
//! instruction grammar — the preamble is inert unless specifically looked for.
//!
//! ```text
//! ; A description of what this program does goes here, before the preamble.
//! # !preamble {
//! # [signature]
//! # (Ciphertext<8, 2, 2>, Ciphertext<8, 2, 2>) -> Ciphertext<8, 2, 2>
//! # [lut]
//! # my_lut: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]
//! # }
//! ADD R2 R1 R3
//! ...
//! ```
//!
//! * `[signature]` holds one line written in [`Signature<Type>`]'s own `Display` syntax (the same
//!   `(arg, arg, ...) -> (ret, ret, ...)` form `{:?}` already renders, with each `Type` spelled
//!   `Ciphertext<int_size, carry_size, message_size>` or `Plaintext<int_size, message_size>`, and
//!   the surrounding parens dropped when a side has exactly one element). There's no separate "iop
//!   assembly" grammar here — the section is a straight, round-trippable textual encoding of the
//!   zhc [`Signature`] type itself. There's no separate `[ciphertext_spec]` section either: the
//!   file's block spec (carry/message width) is derived from the `Ciphertext`/ `Plaintext` types
//!   named here — every one of them must agree on carry/message width (their `int_size` may differ
//!   freely, e.g. when a return packs more than one argument's worth of blocks), and at least one
//!   `Ciphertext` type must be present so the block spec is derivable at all. A `dop.asm` program
//!   is meant to be one part of a possibly multi-board operation (see `tfhe-rs`'s `custom_iop`
//!   fixtures under `zhc_cli/dop_fmt/examples`, whose `_v0`/`_v1`/... siblings are different
//!   boards' fragments of the *same* logical operation) — every sibling should declare the
//!   operation's full signature, even where its own instruction stream only touches a subset of it.
//! * `[lut]` holds zero or more `name: [v0, v1, ..., vN]` lines, one raw lookup table per line. The
//!   table must have exactly `2^data_size` entries for the file's block spec; entries are indexed
//!   by the input block's raw data bits (padding cleared) and are otherwise uninterpreted, matching
//!   [`Lut1::from_fn`]'s table layout.
//!
//! # Instruction listing
//!
//! The body reconstructs a context-threaded [`IR<DopLang>`] graph: one instruction per line,
//! `OPCODE arg0 arg1 ...`, blank lines and lines whose first non-whitespace character is `;` or
//! `#` are ignored. Each operand is parsed by a small recursive-descent [`Cursor`] over the
//! operand grammar mirrored from each operand type's own `.asm(...)` method (e.g.
//! [`CtReg::asm`](super::CtReg::asm)), then checked against the operand kind each mnemonic
//! expects at that position (e.g. `ADD`'s three operands must all be registers) — mirroring the
//! strictness of `tfhe_hpu_backend::asm::dop::arg::FromAsm`, so e.g. `ADD R2 R4 TI[4].2` is
//! rejected even though `TI[4].2` parses fine as a token in isolation.

use std::fmt;

use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, PlaintextSpec, Type,
    lut::{Lut1, LutRegistry},
};
use zhc_ir::{IR, Signature};
use zhc_utils::svec;

use crate::doplang::{
    CtDstVar, CtHeap, CtIo, CtMem, CtReg, CtSrcVar, DopInstructionSet, DopLang, LutRef, MASK_PBS2,
    MASK_PBS4, MASK_PBS8, PtArg, PtConst, PtSrcVar, UserFlag, VirtId,
};

const COMMENT_PREFIX: [char; 2] = [';', '#'];

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

/// Parses a full `dop.asm` file: its mandatory [`Preamble`] followed by the instruction
/// listing.
///
/// The `[lut]` preamble section is the only source of [`LutRegistry`] entries — there is no way
/// to pass one in from the caller, so every `Pbs<Name>` operand in the listing must be declared
/// there.
pub fn parse_assembly(src: &str) -> Result<(Preamble, IR<DopLang>), ParseError> {
    let (preamble, header_lines, body) = parse_preamble(src)?;
    let ir = parse_instructions(&body, &preamble.luts).map_err(|mut err| {
        err.line += header_lines;
        err
    })?;
    Ok((preamble, ir))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Signature,
    Lut,
}

/// The declarations gathered from a `dop.asm` file's mandatory `!preamble { ... }` block.
#[derive(Debug, Clone)]
pub struct Preamble {
    pub block_spec: CiphertextBlockSpec,
    pub signature: Signature<Type>,
    pub luts: LutRegistry,
}

/// Parses the mandatory leading `!preamble { ... }` block out of `src`, regardless of what the
/// body that follows contains.
///
/// This is the building block [`parse_assembly`] uses for `dop.asm` text, but the preamble
/// format itself has no opinion on the body: it's equally at home in front of a hex listing (one
/// instruction per line, as hex text) or any other DOP encoding, since every preamble line is
/// just an ordinary `;`/`#` comment regardless of what follows it.
///
/// Returns the parsed [`Preamble`], the number of physical lines the block occupied (for
/// error-line offsetting), and the remaining body text.
pub fn parse_preamble(src: &str) -> Result<(Preamble, usize, String), ParseError> {
    let mut lines = src.lines().enumerate();

    loop {
        let Some((idx, line)) = lines.next() else {
            return Err(ParseError {
                line: 1,
                message: "dop.asm files must contain a `!preamble { ... }` block before the \
                          first instruction, got an empty file"
                    .to_string(),
            });
        };
        if line.trim().is_empty() {
            continue;
        }
        let Some(content) = strip_comment(line).map(str::trim) else {
            return Err(ParseError {
                line: idx + 1,
                message: format!(
                    "expected a comment line (description or the `!preamble {{` opener) before \
                     the first instruction, got `{line}`"
                ),
            });
        };
        if content == "!preamble {" {
            break;
        }
        // Otherwise this is an ordinary description comment preceding the preamble — skip it.
    }

    let mut section: Option<Section> = None;
    let mut sig_lines = Vec::new();
    let mut lut_lines = Vec::new();
    let mut end_line = None;

    for (idx, raw_line) in lines.by_ref() {
        let Some(content) = strip_comment(raw_line) else {
            return Err(ParseError {
                line: idx + 1,
                message: "expected a comment line inside a `!preamble { ... }` block".to_string(),
            });
        };
        let content = content.trim();
        if content.is_empty() {
            continue;
        }
        if is_closing_brace(content) {
            end_line = Some(idx);
            break;
        }
        if let Some(name) = content.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            section = Some(match name {
                "signature" => Section::Signature,
                "lut" => Section::Lut,
                other => {
                    return Err(ParseError {
                        line: idx + 1,
                        message: format!("unknown preamble section `[{other}]`"),
                    });
                }
            });
            continue;
        }
        match section {
            Some(Section::Signature) => sig_lines.push((idx, content.to_string())),
            Some(Section::Lut) => lut_lines.push((idx, content.to_string())),
            None => {
                return Err(ParseError {
                    line: idx + 1,
                    message: "preamble content before any `[section]` header".to_string(),
                });
            }
        }
    }
    let end_line = end_line.ok_or_else(|| ParseError {
        line: src.lines().count(),
        message: "unterminated `!preamble {` block (missing a `}` line)".to_string(),
    })?;

    let signature = parse_signature_section(&sig_lines)?;
    let sig_line = sig_lines.first().map(|(i, _)| i + 1).unwrap_or(1);
    let block_spec = derive_block_spec(&signature).map_err(|message| ParseError {
        line: sig_line,
        message: format!("[signature]: {message}"),
    })?;
    let luts = parse_lut_section(&lut_lines, block_spec)?;

    let body = src
        .lines()
        .skip(end_line + 1)
        .collect::<Vec<_>>()
        .join("\n");

    Ok((
        Preamble {
            block_spec,
            signature,
            luts,
        },
        end_line + 1,
        body,
    ))
}

fn strip_comment(line: &str) -> Option<&str> {
    line.trim_start().strip_prefix(COMMENT_PREFIX)
}

fn is_closing_brace(content: &str) -> bool {
    content
        .strip_prefix('}')
        .is_some_and(|rest| rest.chars().all(|c| c == ' ' || c == '-'))
}

fn derive_block_spec(sig: &Signature<Type>) -> Result<CiphertextBlockSpec, String> {
    let mut block_spec: Option<CiphertextBlockSpec> = None;
    let mut message_size: Option<u8> = None;

    for t in sig.get_args().iter().chain(sig.get_returns()) {
        match t {
            Type::Ciphertext(cs) => {
                let s = cs.block_spec();
                match block_spec {
                    None => {
                        message_size = Some(s.message_size());
                        block_spec = Some(s);
                    }
                    Some(prev) if prev == s => {}
                    Some(prev) => {
                        return Err(format!(
                            "a Ciphertext entry has carry/message width {}/{}, but an earlier \
                             entry has {}/{} — every entry must agree",
                            s.carry_size(),
                            s.message_size(),
                            prev.carry_size(),
                            prev.message_size()
                        ));
                    }
                }
            }
            Type::Plaintext(ps) => {
                let m = ps.block_spec().message_size();
                match message_size {
                    None => message_size = Some(m),
                    Some(prev) if prev == m => {}
                    Some(prev) => {
                        return Err(format!(
                            "a Plaintext entry has message width {m}, but an earlier entry has \
                             {prev} — every entry must agree"
                        ));
                    }
                }
            }
        }
    }

    block_spec.ok_or_else(|| {
        "no Ciphertext entry to derive the file's carry/message block width from — at least one \
         is required, even if the instruction stream itself doesn't touch it"
            .to_string()
    })
}

fn parse_lut_section(
    lines: &[(usize, String)],
    block_spec: CiphertextBlockSpec,
) -> Result<LutRegistry, ParseError> {
    let expected_len = 1usize << block_spec.data_size();
    let mut registry = LutRegistry::empty();
    for (idx, line) in lines {
        let (name, table) = line.split_once(':').ok_or_else(|| ParseError {
            line: idx + 1,
            message: format!("[lut] line must be `name: [v0, v1, ...]`, got `{line}`"),
        })?;
        let name = name.trim();
        let table = table.trim();
        let table = table
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or_else(|| ParseError {
                line: idx + 1,
                message: format!("[lut] `{name}`: table must be bracketed, e.g. `[0, 1, ...]`"),
            })?;
        let values = table
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|v| {
                if let Some(hex) = v.strip_prefix("0x") {
                    u16::from_str_radix(hex, 16)
                } else {
                    v.parse::<u16>()
                }
                .map_err(|err| ParseError {
                    line: idx + 1,
                    message: format!("[lut] `{name}`: invalid table entry `{v}`: {err}"),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if values.len() != expected_len {
            return Err(ParseError {
                line: idx + 1,
                message: format!(
                    "[lut] `{name}`: expected {expected_len} entries (2^data_size for this \
                     file's block spec), got {}",
                    values.len()
                ),
            });
        }
        let lut = Lut1::from_fn(name, block_spec, move |b| {
            block_spec.from_data(values[b.raw_data_bits() as usize])
        });
        registry.register_l1(&lut);
    }
    Ok(registry)
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = s[start..].trim();
    if !last.is_empty() || !parts.is_empty() {
        parts.push(last);
    }
    parts
}

fn parse_type(tok: &str) -> Result<Type, String> {
    let tok = tok.trim();
    let invalid = || format!("`{tok}`: expected `Ciphertext<...>` or `Plaintext<...>`");

    let parse_fields = |inner: &str| -> Result<Vec<u16>, String> {
        inner
            .split(',')
            .map(str::trim)
            .map(|f| f.parse::<u16>().map_err(|err| format!("`{tok}`: {err}")))
            .collect()
    };

    if let Some(rest) = tok.strip_prefix("Ciphertext<") {
        let inner = rest.strip_suffix('>').ok_or_else(invalid)?;
        let fields = parse_fields(inner)?;
        let [int_size, carry_size, message_size] = fields.as_slice() else {
            return Err(format!(
                "`{tok}`: expected `Ciphertext<int_size, carry_size, message_size>`"
            ));
        };
        Ok(Type::Ciphertext(
            zhc_crypto::integer_semantics::CiphertextSpec::new(
                *int_size,
                *carry_size as u8,
                *message_size as u8,
            ),
        ))
    } else if let Some(rest) = tok.strip_prefix("Plaintext<") {
        let inner = rest.strip_suffix('>').ok_or_else(invalid)?;
        let fields = parse_fields(inner)?;
        let [int_size, message_size] = fields.as_slice() else {
            return Err(format!(
                "`{tok}`: expected `Plaintext<int_size, message_size>`"
            ));
        };
        Ok(Type::Plaintext(PlaintextSpec::new(
            *int_size,
            *message_size as u8,
        )))
    } else {
        Err(invalid())
    }
}

fn parse_type_list(s: &str) -> Result<Vec<Type>, String> {
    let s = s.trim();
    if s == "()" {
        return Ok(vec![]);
    }
    if let Some(inner) = s.strip_prefix('(').and_then(|rest| rest.strip_suffix(')')) {
        split_top_level_commas(inner)
            .into_iter()
            .map(parse_type)
            .collect()
    } else {
        Ok(vec![parse_type(s)?])
    }
}

fn parse_signature_section(lines: &[(usize, String)]) -> Result<Signature<Type>, ParseError> {
    let [(idx, line)] = lines else {
        return Err(ParseError {
            line: lines.first().map(|(i, _)| i + 1).unwrap_or(1),
            message: format!(
                "[signature] must contain exactly one line, got {}",
                lines.len()
            ),
        });
    };
    let parse = || -> Result<Signature<Type>, String> {
        let (args, rets) = line
            .split_once("->")
            .ok_or_else(|| format!("expected `<args> -> <returns>`, got `{line}`"))?;
        let mut sig = Signature::empty();
        for t in parse_type_list(args)? {
            sig.push_arg(t);
        }
        for t in parse_type_list(rets)? {
            sig.push_ret(t);
        }
        Ok(sig)
    };
    parse().map_err(|message| ParseError {
        line: idx + 1,
        message: format!("[signature]: {message}"),
    })
}

fn parse_instructions(src: &str, lreg: &LutRegistry) -> Result<IR<DopLang>, ParseError> {
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
    let masked_dst = |a: CtReg, mask: u8| CtReg { mask, addr: a.addr };

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
        let addr = c.parse_uint::<u8>(tok, "register address")?;
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
        return Ok(LutRef::new(id as u16).into());
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

fn lookup_lut_id(lreg: &LutRegistry, name: &str) -> Option<usize> {
    lreg.iter_luts()
        .find(|(_, raw)| raw.name() == name)
        .map(|(lid, _)| lid.0)
}

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

    fn eat_str(&mut self, lit: &str) -> bool {
        if let Some(rest) = self.rest.strip_prefix(lit) {
            self.rest = rest;
            true
        } else {
            false
        }
    }

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

    fn expect_eof(&self, tok: &str) -> Result<(), String> {
        if self.rest.is_empty() {
            Ok(())
        } else {
            Err(format!("`{tok}`: unexpected trailing `{}`", self.rest))
        }
    }

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
    use zhc_crypto::integer_semantics::CiphertextSpec;

    #[test]
    fn parses_full_preamble_and_body() {
        let src = "\
            # !preamble {\n\
            # [signature]\n\
            # (Ciphertext<8, 2, 2>, Ciphertext<8, 2, 2>) -> Ciphertext<8, 2, 2>\n\
            # [lut]\n\
            # my_lut: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]\n\
            # }\n\
            LD R1 @0x400\n\
            PBS R2 R1 Pbsmy_lut\n\
        ";

        let (preamble, ir) = parse_assembly(src).expect("valid file must parse");

        assert_eq!(preamble.block_spec, CiphertextBlockSpec(2, 2));

        assert_eq!(preamble.signature, {
            let mut sig = Signature::empty();
            sig.push_arg(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
            sig.push_arg(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
            sig.push_ret(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
            sig
        });

        assert!(
            preamble
                .luts
                .iter_luts()
                .any(|(_, raw)| raw.name() == "my_lut")
        );

        // 2 real instructions + synthesized _START/_END.
        assert_eq!(ir.walk_ops_linear().count(), 4);
    }

    #[test]
    fn signature_round_trips_through_its_own_display() {
        let mut sig = Signature::empty();
        sig.push_arg(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
        sig.push_arg(Type::Plaintext(PlaintextSpec::new(8, 2)));
        sig.push_ret(Type::Ciphertext(CiphertextSpec::new(16, 2, 2)));
        sig.push_ret(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));

        let displayed = format!("{sig}");
        let src = format!("# !preamble {{\n# [signature]\n# {displayed}\n# [lut]\n# }}\nSYNC\n");

        let (preamble, _) = parse_assembly(&src).expect("valid file must parse");
        assert_eq!(preamble.signature, sig);
        // int_size may differ freely between entries (16 vs 8 above); only carry/message width
        // is required to agree.
        assert_eq!(preamble.block_spec, CiphertextBlockSpec(2, 2));
    }

    #[test]
    fn rejects_file_without_preamble() {
        let src = "ADD R2 R1 R3\n";
        let err = parse_assembly(src).unwrap_err();
        assert!(err.message.contains("!preamble"), "{}", err.message);
    }

    #[test]
    fn allows_description_comments_before_the_preamble() {
        let src = "\
            ; CUST_HPU0_42\n\
            ; Some description of what this program does.\n\
            ; !preamble {\n\
            ; [signature]\n\
            ; Ciphertext<8, 2, 2> -> Ciphertext<8, 2, 2>\n\
            ; [lut]\n\
            ; }\n\
            SYNC\n\
        ";
        let (preamble, ir) = parse_assembly(src).expect("valid file must parse");
        assert_eq!(preamble.block_spec, CiphertextBlockSpec(2, 2));
        // 1 real instruction + synthesized _START/_END.
        assert_eq!(ir.walk_ops_linear().count(), 3);
    }

    #[test]
    fn allows_a_dashed_separator_before_and_after_the_preamble() {
        let src = "\
            ; CUST_HPU0_42\n\
            ; -----\n\
            ; !preamble {\n\
            ; [signature]\n\
            ; Ciphertext<8, 2, 2> -> Ciphertext<8, 2, 2>\n\
            ; [lut]\n\
            ; } -----\n\
            SYNC\n\
        ";
        let (preamble, _) = parse_assembly(src).expect("valid file must parse");
        assert_eq!(preamble.block_spec, CiphertextBlockSpec(2, 2));
    }

    #[test]
    fn rejects_instruction_before_the_preamble() {
        let src = "SYNC\n; !preamble {\n; [signature]\n; () -> ()\n; [lut]\n; }\n";
        let err = parse_assembly(src).unwrap_err();
        assert!(err.message.contains("!preamble"), "{}", err.message);
    }

    #[test]
    fn rejects_signature_with_no_ciphertext_entry() {
        let src = "# !preamble {\n# [signature]\n# () -> ()\n# [lut]\n# }\nSYNC\n";
        let err = parse_assembly(src).unwrap_err();
        assert!(
            err.message.contains("no Ciphertext entry"),
            "{}",
            err.message
        );
    }

    #[test]
    fn rejects_signature_with_mismatched_block_width() {
        let src = "\
            # !preamble {\n\
            # [signature]\n\
            # (Ciphertext<8, 2, 2>, Ciphertext<8, 1, 3>) -> ()\n\
            # [lut]\n\
            # }\n\
            SYNC\n\
        ";
        let err = parse_assembly(src).unwrap_err();
        assert!(
            err.message.contains("every entry must agree"),
            "{}",
            err.message
        );
    }

    #[test]
    fn rejects_lut_with_wrong_entry_count() {
        let src = "\
            # !preamble {\n\
            # [signature]\n\
            # (Ciphertext<8, 2, 2>, Ciphertext<8, 2, 2>) -> Ciphertext<8, 2, 2>\n\
            # [lut]\n\
            # too_short: [0, 1, 2]\n\
            # }\n\
            LD R1 @0x400\n\
        ";
        let err = parse_assembly(src).unwrap_err();
        assert!(err.message.contains("expected 16 entries"));
    }

    #[test]
    fn rejects_unterminated_preamble() {
        let src = "# !preamble {\n# [signature]\n# Ciphertext<16, 2, 2> -> ()\n";
        let err = parse_assembly(src).unwrap_err();
        assert!(err.message.to_lowercase().contains("unterminated"));
    }

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
            .filter(|instr| !matches!(instr, DopInstructionSet::_START | DopInstructionSet::_END))
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
                dst: CtReg::new(2u8),
                src1: CtReg::new(1u8),
                src2: CtReg::new(3u8),
            }
        );
        assert_eq!(
            ops[13],
            DopInstructionSet::PBS_ML2 {
                dst: CtReg::ml2(4u8),
                src: CtReg::new(1u8),
                lut: LutRef::new(0u8),
            }
        );
        assert_eq!(
            ops[16],
            DopInstructionSet::WAIT {
                flag: UserFlag::new(0),
                slot: Some(CtMem::heap(0u16)),
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
                dst: CtReg::new(1u8),
                src: CtMem::heap(3u16),
            },
            svec![start[0]],
        );
        let (_, r2) = ir.add_op(
            DopInstructionSet::PBS {
                dst: CtReg::new(2u8),
                src: CtReg::new(1u8),
                lut: LutRef::new(1u16),
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
        let ir =
            parse_instructions("MAC R2 R1 R3 TI[4].0\n", &lreg).expect("valid line must parse");
        assert_eq!(
            instructions(&ir),
            vec![DopInstructionSet::MAC {
                dst: CtReg::new(2u8),
                src1: CtReg::new(1u8),
                src2: CtReg::new(3u8),
                cst: PtArg::var(4, 0),
            }]
        );
    }

    #[test]
    fn mac_rejects_register_multiplier() {
        let lreg = registry();
        let err = parse_instructions("MAC R2 R1 R3 R4\n", &lreg).unwrap_err();
        assert!(
            err.message.contains("plaintext immediate"),
            "{}",
            err.message
        );
    }

    #[test]
    fn rejects_memory_where_flag_expected() {
        let lreg = registry();
        let err = parse_instructions("WAIT TH.0\n", &lreg).unwrap_err();
        assert!(err.message.contains("flag"), "{}", err.message);
    }
}
