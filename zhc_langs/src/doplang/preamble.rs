//! Preamble parsing for `dop.asm` files.
//!
//! A `dop.asm` file must contain a `!preamble { ... }` block declaring everything the plain
//! instruction grammar (see [`parse_instructions`](super::parser::parse_instructions)) cannot
//! express on its own: the external [`Signature<Type>`] of the graph of DOP the file describes,
//! and any literal lookup tables it references by name. Ordinary description comments (naming
//! the operation, documenting expected results, ...) may precede the block; the first
//! non-comment, non-blank line must be the `!preamble {` opener itself.
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
//! * `[signature]` holds one line written in [`Signature<Type>`]'s own `Display` syntax (the
//!   same `(arg, arg, ...) -> (ret, ret, ...)` form `{:?}` already renders, with each `Type`
//!   spelled `Ciphertext<int_size, carry_size, message_size>` or `Plaintext<int_size,
//!   message_size>`, and the surrounding parens dropped when a side has exactly one element).
//!   There's no separate "iop assembly" grammar here — the section is a straight, round-trippable
//!   textual encoding of the zhc [`Signature`] type itself. There's no separate `[ciphertext_spec]`
//!   section either: the file's block spec (carry/message width) is derived from the `Ciphertext`/
//!   `Plaintext` types named here — every one of them must agree on carry/message width (their
//!   `int_size` may differ freely, e.g. when a return packs more than one argument's worth of
//!   blocks), and at least one `Ciphertext` type must be present so the block spec is derivable at
//!   all. A `dop.asm` program is meant to be one part of a possibly multi-board operation (see
//!   `tfhe-rs`'s `custom_iop` fixtures under `zhc_cli/dop_fmt/examples`, whose `_v0`/`_v1`/...
//!   siblings are different boards' fragments of the *same* logical operation) — every sibling
//!   should declare the operation's full signature, even where its own instruction stream only
//!   touches a subset of it.
//! * `[lut]` holds zero or more `name: [v0, v1, ..., vN]` lines, one raw lookup table per line.
//!   The table must have exactly `2^data_size` entries for the file's block spec; entries are
//!   indexed by the input block's raw data bits (padding cleared) and are otherwise
//!   uninterpreted, matching [`Lut1::from_fn`]'s table layout.
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, PlaintextSpec, Type,
    lut::{Lut1, LutRegistry},
};
use zhc_ir::{IR, Signature};

use super::parser::{COMMENT_PREFIX, ParseError, parse_instructions};
use crate::doplang::DopLang;

/// The two sections a `!preamble { ... }` block must declare (`[lut]` may be empty, but the
/// header itself is still required so a file without LUTs reads the same as one with).
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

    // Skip any leading description comments (ordinary `;`/`#` lines, e.g. naming the operation)
    // that precede the mandatory `!preamble {` opener — only comments are allowed there, so the
    // first non-comment, non-blank line must be the opener itself.
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

/// Strips a leading `;`/`#` comment prefix (after trimming leading whitespace), returning the
/// rest of the line, or `None` if the line isn't a comment.
fn strip_comment(line: &str) -> Option<&str> {
    line.trim_start().strip_prefix(COMMENT_PREFIX)
}

/// Recognizes the preamble block's closing line: a bare `}`, or one dressed up with a trailing
/// run of dashes for visibility (e.g. `} ------...`), as rendered by
/// `zhc_cli::dop_fmt::convert::render_preamble`.
fn is_closing_brace(content: &str) -> bool {
    content
        .strip_prefix('}')
        .is_some_and(|rest| rest.chars().all(|c| c == ' ' || c == '-'))
}

/// Derives the file's block spec (carry/message width) from its `[signature]`: every
/// `Ciphertext`/`Plaintext` type named there must agree on carry/message width (a `Ciphertext`'s
/// `int_size` may still differ freely between entries), and at least one `Ciphertext` type must
/// be present, since a `Plaintext`'s message width alone doesn't pin down the carry width.
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

/// Splits `s` on top-level commas only, i.e. commas not nested inside a `<...>` group (as found
/// in a `Ciphertext<a, b, c>`/`Plaintext<a, b>` type). Mirrors `Signature<T: Debug>`'s own
/// `Display` impl, which uses `debug_tuple` — this is its inverse.
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

/// Parses one `Ciphertext<int_size, carry_size, message_size>` or `Plaintext<int_size,
/// message_size>` token — the exact form `Type`'s `Debug` impl emits.
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

/// Parses one side of a signature (`()`, a bare `Type`, or a `(Type, Type, ...)` tuple) —
/// mirroring `Signature<T: Debug>::fmt`'s 0/1/N-arity cases.
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

#[cfg(test)]
mod tests {
    use super::*;
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

        assert_eq!(
            preamble.signature,
            {
                let mut sig = Signature::empty();
                sig.push_arg(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
                sig.push_arg(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
                sig.push_ret(Type::Ciphertext(CiphertextSpec::new(8, 2, 2)));
                sig
            }
        );

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
        let src = format!(
            "# !preamble {{\n# [signature]\n# {displayed}\n# [lut]\n# }}\nSYNC\n"
        );

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
        assert!(err.message.contains("no Ciphertext entry"), "{}", err.message);
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
        assert!(err.message.contains("every entry must agree"), "{}", err.message);
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
}
