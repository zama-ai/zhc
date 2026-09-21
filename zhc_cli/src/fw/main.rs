//! `fw`: dumps a custom-IOp-loadable `dop.asm` file for a static [`Iop`](zhc::compat::Iop).
//!
//! One of `zhc_cli`'s utilities, built as its own binary (`src/fw/main.rs`, declared via `[[bin]]`
//! in `Cargo.toml`), alongside `dop_fmt` (`src/dop_fmt/`).
//!
//! ```text
//! fw --iop <IOP> --integer-w <INTEGER_W> [--output <OUTPUT>]
//! ```
//!
//! Builds the `Iop`'s HPU pipeline at the given integer width (using the default `HpuConfig`
//! preset — this tool has no board/parameter set of its own) and gets its assembly listing via
//! [`Pipeline::get_hpu_assembly`](zhc_pipeline::Pipeline::get_hpu_assembly), which is always a
//! complete `dop.asm` file — `!preamble { ... }` block (signature, LUT tables) followed by the
//! instruction listing — loadable as-is by `tfhe-hpu-backend`'s custom-IOp mechanism once named
//! `<base>_v<N>.asm` for the target HPU node. With `--output`, it's written to that file;
//! otherwise it's printed to stdout.

mod args;

use std::process::ExitCode;

use args::Args;
use zhc::PipelineExt;
use zhc::builder::CiphertextSpec;
use zhc::compat::Iop;
use zhc::config::hpu::HpuConfig;
use zhc_pipeline::Pipeline;

/// Builds `iop`'s HPU pipeline, mirroring the legacy-scheduler special-casing
/// `Iop::get_translation_table`/`Iop::compute_latency` already apply for these same ops.
fn build_pipeline(iop: &Iop, hpu_config: &HpuConfig, spec: CiphertextSpec) -> Pipeline {
    let pipeline = Pipeline::new()
        .with_builder(iop.to_builder(spec))
        .with_hpu_config(hpu_config.clone());
    match (iop, spec.int_size()) {
        (Iop::Mul, _)
        | (Iop::OvfMul, _)
        | (Iop::RightRot | Iop::LeftRot | Iop::LeftShift | Iop::RightShift, 128) => {
            pipeline.with_legacy_hpu_scheduler()
        }
        _ => pipeline,
    }
}

fn run(args: &Args) -> Result<(), String> {
    let hpu_config = HpuConfig::default();
    let spec = CiphertextSpec::new(args.integer_w, 2, 2);

    let mut pipeline = build_pipeline(&args.iop, &hpu_config, spec);
    let asm_path = pipeline.get_hpu_assembly();

    match &args.output {
        Some(output) => {
            std::fs::copy(asm_path, output)
                .map_err(|err| format!("writing {}: {err}", output.display()))?;
        }
        None => {
            let asm = std::fs::read_to_string(asm_path)
                .map_err(|err| format!("reading generated assembly: {err}"))?;
            print!("{asm}");
        }
    }

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
