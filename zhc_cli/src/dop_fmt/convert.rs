//! `dop.asm` <-> hex conversion helpers.
//!
//! Both formats share the exact same mandatory `!preamble { ... }` block (signature, LUT
//! tables) — see `zhc_langs::doplang::preamble`. Only the instruction listing that follows
//! differs: `dop.asm` text mnemonics, or one hex-encoded 32-bit DOp word per line (lowercase, no
//! `0x` prefix). Neither format loses information relative to the other, so converting between
//! them is lossless in both directions.
//!
//! A hex line's LUT references are plain numeric ids, resolved against the preamble's `[lut]`
//! section by registration order (the same order `Pbs<name>` resolves against by name in
//! `dop.asm` text) — so no id remapping is needed in either direction.
//!
//! Every load always goes through the parser and every write always goes through the
//! corresponding generator — even for a same-kind `--from`/`--to` run — so the tool doubles as a
//! formatter: it always re-renders the file from the parsed [`zhc_langs::doplang`] IR rather than
//! ever passing input bytes through unchanged.

use std::panic::{AssertUnwindSafe, catch_unwind};

use zhc_crypto::integer_semantics::CiphertextBlockSpec;
use zhc_crypto::integer_semantics::Type;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{IR, Signature};
use zhc_langs::doplang::{DopLang, emit_assembly, emit_preamble, parse_assembly, parse_preamble};
use zhc_pipeline::passes::{hpu_decode_translation_table, hpu_generate_translation_table};

/// A loaded DOP program, with enough context (block spec, signature, LUT registry) to run
/// passes or a simulation over it, or to re-render it in either file format.
pub struct Loaded {
    pub ir: IR<DopLang>,
    pub block_spec: CiphertextBlockSpec,
    pub signature: Signature<Type>,
    pub luts: LutRegistry,
}

fn catch<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|payload| {
        payload
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "operation panicked with a non-string payload".to_string())
    })
}

/// Parses a `dop.asm` file's text into a [`Loaded`] program.
pub fn load_asm(src: &str) -> Result<Loaded, String> {
    let (preamble, ir) = parse_assembly(src).map_err(|err| err.to_string())?;
    Ok(Loaded {
        ir,
        block_spec: preamble.block_spec,
        signature: preamble.signature,
        luts: preamble.luts,
    })
}

/// Parses a hex file's text — the same mandatory `!preamble { ... }` block as `dop.asm`,
/// followed by one hex-encoded 32-bit DOp word per line (matching what [`to_hex_text`] emits) —
/// into a [`Loaded`] program.
pub fn load_hex(src: &str) -> Result<Loaded, String> {
    let (preamble, header_lines, body) = parse_preamble(src).map_err(|err| err.to_string())?;
    let words = parse_hex_words(&body).map_err(|err| offset_hex_error(err, header_lines))?;

    let mut framed = Vec::with_capacity(words.len() + 1);
    framed.push(words.len() as u32);
    framed.extend(words);
    let ir = catch(|| hpu_decode_translation_table(&framed, None))?;

    Ok(Loaded {
        ir,
        block_spec: preamble.block_spec,
        signature: preamble.signature,
        luts: preamble.luts,
    })
}

fn parse_hex_words(src: &str) -> Result<Vec<u32>, String> {
    src.lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.trim();
            if line.is_empty() || line.starts_with([';', '#']) {
                None
            } else {
                Some((idx, line))
            }
        })
        .map(|(idx, line)| {
            u32::from_str_radix(line, 16)
                .map_err(|err| format!("hex:{}: invalid word `{line}`: {err}", idx + 1))
        })
        .collect()
}

fn offset_hex_error(err: String, header_lines: usize) -> String {
    match err.split_once(':') {
        Some((prefix, rest)) if prefix == "hex" => {
            if let Some((num, rest)) = rest.split_once(':')
                && let Ok(n) = num.parse::<usize>()
            {
                return format!("hex:{}:{rest}", n + header_lines);
            }
            format!("{prefix}:{rest}")
        }
        _ => err,
    }
}

fn format_hex_words(words: &[u32]) -> String {
    words.iter().map(|w| format!("{w:x}\n")).collect()
}

/// Converts a loaded program to hex text (the format [`load_hex`] reads back): the same
/// preamble as [`to_asm_text`], followed by one hex-encoded DOp word per line.
///
/// Always re-renders from `loaded`'s IR, even if it was itself loaded from hex — so running the
/// tool `--from x.hex --to x.hex` reformats rather than copying the file unchanged.
///
/// # Errors
///
/// Returns an error (rather than panicking) if the program contains an instruction
/// `generate_translation_table` cannot encode — currently just `SYNC`, whose hardware encoding
/// needs sync metadata (`iid`/`hid`/`flag`) that `DopInstructionSet::SYNC` doesn't carry.
pub fn to_hex_text(loaded: &Loaded) -> Result<String, String> {
    let framed = catch(|| hpu_generate_translation_table(&loaded.ir, None))?;
    let mut out = emit_preamble(&loaded.signature, &loaded.luts);
    out.push_str(&format_hex_words(&framed[1..]));
    Ok(out)
}

/// Converts a loaded program to `dop.asm` text: its preamble (signature, LUT tables), followed
/// by the instruction listing.
///
/// Always re-renders from `loaded`'s IR, even if it was itself loaded from `dop.asm` text — so
/// running the tool `--from x.asm --to x.asm` reformats rather than copying the file unchanged.
pub fn to_asm_text(loaded: &Loaded) -> String {
    let mut out = emit_preamble(&loaded.signature, &loaded.luts);
    out.push_str(&emit_assembly(&loaded.ir, &loaded.luts));
    out
}
