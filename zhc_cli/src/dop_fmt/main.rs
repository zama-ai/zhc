//! `dop_fmt`: standalone `dop.asm` <-> hex conversion tool.
//!
//! One of `zhc_cli`'s utilities, built as its own binary (`src/dop_fmt/main.rs`, declared via
//! `[[bin]]` in `Cargo.toml`); further utilities are expected to live alongside it as sibling
//! `src/<name>/` modules with their own `[[bin]]` entry.
//!
//! ```text
//! dop_fmt --from <path> --to <path> [--passes NameA,NameB ...]
//!         [--simulate [--inputs VALUE ...]] [--perf [--perf-rpt REPORT ...]]
//! ```
//!
//! The conversion direction is inferred from the file extensions: a `.hex` path holds one
//! hex-encoded 32-bit DOp word per line (matching [`convert::to_hex_text`]); anything else
//! (`.asm`, `.txt`, no extension, ...) is `dop.asm` text. Both formats share the same mandatory
//! `!preamble { ... }` block, so neither direction loses information. `--from`/`--to` may use
//! the same kind — the file always goes through parse-then-regenerate regardless (see
//! [`convert::to_asm_text`]/[`convert::to_hex_text`]), so this doubles as a formatter rather
//! than copying the input unchanged.
//!
//! `--passes` selects one or more [`DopPass`] checks to run and print a report line for.
//!
//! `--simulate` and `--perf` are independent and may be combined: `--simulate` interprets the
//! program (see [`sim::simulate`]) to check it evaluates without panicking, with heap/io memory
//! starting at zero and ciphertext template sources (`TS[..]`) optionally seeded via
//! whole-integer `--inputs` values (see [`inputs`]) — positional by `TS` id, decomposed into
//! per-block slots by `dop_fmt` itself — printing the whole-integer values used and produced as
//! `val_a, val_b -> val_r`; `--perf` instead runs it through the real discrete-event HPU
//! simulator (see [`perf::run`]) to estimate hardware latency and PBS batching behavior,
//! reporting whichever `--perf-rpt` categories were requested (or all of them, if none were).

mod args;
mod convert;
mod inputs;
mod passes;
mod perf;
mod sim;

use std::path::Path;
use std::process::ExitCode;

use args::Args;
use perf::PerfReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Asm,
    Hex,
}

fn kind_of(path: &Path) -> Kind {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("hex") => Kind::Hex,
        _ => Kind::Asm,
    }
}

fn run(args: &Args) -> Result<(), String> {
    let src = std::fs::read_to_string(&args.from)
        .map_err(|err| format!("reading {}: {err}", args.from.display()))?;

    let from_kind = kind_of(&args.from);
    let to_kind = kind_of(&args.to);

    let loaded = match from_kind {
        Kind::Asm => convert::load_asm(&src),
        Kind::Hex => convert::load_hex(&src),
    }
    .map_err(|err| format!("{}: {err}", args.from.display()))?;

    for pass in &args.passes {
        println!("[{pass}] {}", pass.run(&loaded.ir));
    }

    if args.simulate {
        let inputs: Vec<usize> = args.inputs.iter().map(|v| v.0).collect();
        let block_spec = loaded.block_spec;
        let report = sim::simulate(
            &loaded.ir,
            block_spec,
            &loaded.luts,
            &loaded.signature,
            &inputs,
        )
        .map_err(|err| format!("--simulate: {err}"))?;
        match &report.outcome {
            sim::SimOutcome::Ok(n) => {
                let inputs = report.inputs;
                let outputs = report.outputs;
                println!("[simulate] ok: {n} instruction(s) evaluated");
                println!("[simulate] Dec: {inputs:?} -> {outputs:?}");
                println!("[simulate] Hex: {inputs:x?} -> {outputs:x?}");
            }
            sim::SimOutcome::Failed(failures) => {
                println!(
                    "[simulate] failed: {} instruction(s) panicked",
                    failures.len()
                );
                let inputs = report.inputs;
                println!("[simulate] inputs: {inputs:?}/{inputs:x?}");
                for (instr, msg) in failures {
                    println!("  {instr}: {msg}");
                }
            }
        }
    }

    if args.perf {
        let reports: &[PerfReport] = if args.perf_rpt.is_empty() {
            PerfReport::ALL
        } else {
            &args.perf_rpt
        };
        perf::run(&loaded.ir).print(reports);
    }

    let out = match to_kind {
        Kind::Hex => convert::to_hex_text(&loaded)
            .map_err(|err| format!("encoding {}: {err}", args.to.display()))?,
        Kind::Asm => convert::to_asm_text(&loaded),
    };
    std::fs::write(&args.to, out).map_err(|err| format!("writing {}: {err}", args.to.display()))?;

    Ok(())
}

fn main() -> ExitCode {
    let args = match args::parse(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
