//! Hand-rolled command-line parsing for `dop_fmt` (no `clap`).
//!
//! Mirrors the exact flag set and behavior clap previously generated: `--from`/`--to` are
//! required paths; `--passes`, `--inputs`, and `--perf-rpt` are repeatable and each occurrence
//! may itself be a comma-separated list; `--inputs` requires `--simulate` and `--perf-rpt`
//! requires `--perf`; `-h`/`--help` and `-V`/`--version` print and exit immediately.

use std::path::PathBuf;
use std::str::FromStr;

use crate::inputs::InputValue;
use crate::passes::DopPass;
use crate::perf::PerfReport;

const USAGE_LINE: &str = "Usage: dop_fmt [OPTIONS] --from <FROM> --to <TO>";

const USAGE: &str = "\
Converts between dop.asm text and hex, with optional property checks, interpretation, and performance estimation.

Usage: dop_fmt [OPTIONS] --from <FROM> --to <TO>

Options:
      --from <FROM>          Input file. A `.hex` extension is read as hex; anything else as dop.asm text.
      --to <TO>              Output file. A `.hex` extension is written as hex; anything else as dop.asm text.
      --passes <PASSES>      Property-check passes to run and report, e.g. `--passes Spills,PbsUsage`.
                             Repeatable, comma-separated.
      --simulate             Interpret the loaded program (a structural smoke test: does it evaluate
                             without panicking?). Heap/io memory starts at zero; ciphertext template
                             sources (TS[..]) are seeded from --inputs where given and randomly
                             otherwise; plaintext templates (TI[..]) are always random.
      --inputs <VALUE>       Whole-integer value (decimal, or hex with a `0x` prefix) for one ciphertext
                             template source, e.g. `--inputs 0x55 --inputs 24`. Positional: the first
                             value seeds TS[0], the second TS[1], and so on. Repeatable,
                             comma-separated. Requires --simulate.
      --perf                 Run the loaded program through the real discrete-event HPU simulator to
                             estimate hardware latency and PBS batching behavior.
      --perf-rpt <REPORT>    Which --perf report(s) to print, e.g. `--perf-rpt latency,pbs-batch`. All
                             reports print if none are given. Repeatable, comma-separated. Requires --perf.
  -h, --help                 Print help.
  -V, --version              Print version.
";

pub struct Args {
    pub from: PathBuf,
    pub to: PathBuf,
    pub passes: Vec<DopPass>,
    pub simulate: bool,
    pub inputs: Vec<InputValue>,
    pub perf: bool,
    pub perf_rpt: Vec<PerfReport>,
}

/// Parses `dop_fmt`'s command line, or prints help/version and exits (matching clap's own
/// behavior for `-h`/`--help`/`-V`/`--version`) — this never returns for those flags.
pub fn parse(argv: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut from: Option<PathBuf> = None;
    let mut to: Option<PathBuf> = None;
    let mut passes = Vec::new();
    let mut simulate = false;
    let mut inputs = Vec::new();
    let mut perf = false;
    let mut perf_rpt = Vec::new();

    let mut argv = argv.into_iter();
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("dop_fmt {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--from" => from = Some(PathBuf::from(value_of(&mut argv, "--from")?)),
            "--to" => to = Some(PathBuf::from(value_of(&mut argv, "--to")?)),
            "--passes" => extend(&mut passes, &value_of(&mut argv, "--passes")?)?,
            "--simulate" => simulate = true,
            "--inputs" => extend(&mut inputs, &value_of(&mut argv, "--inputs")?)?,
            "--perf" => perf = true,
            "--perf-rpt" => extend(&mut perf_rpt, &value_of(&mut argv, "--perf-rpt")?)?,
            other => return Err(brief(&format!("unknown argument `{other}`"))),
        }
    }

    let from = from.ok_or_else(|| missing("--from"))?;
    let to = to.ok_or_else(|| missing("--to"))?;
    if !inputs.is_empty() && !simulate {
        return Err("--inputs requires --simulate".to_string());
    }
    if !perf_rpt.is_empty() && !perf {
        return Err("--perf-rpt requires --perf".to_string());
    }

    Ok(Args {
        from,
        to,
        passes,
        simulate,
        inputs,
        perf,
        perf_rpt,
    })
}

fn missing(flag: &str) -> String {
    brief(&format!(
        "the following required argument was not provided: {flag}"
    ))
}

/// An error message followed by the one-line usage summary and a hint toward `--help`, matching
/// clap's own (concise) error style rather than dumping the full option list on every mistake.
fn brief(message: &str) -> String {
    format!("{message}\n\n{USAGE_LINE}\n\nFor more information, try '--help'.")
}

/// Consumes and returns the next token as a flag's value, erroring if the flag was the last
/// token on the command line.
fn value_of(argv: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    argv.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

/// Splits `raw` on commas and parses each piece into `list`, so a single `--flag a,b` occurrence
/// behaves the same as `--flag a --flag b`.
fn extend<T: FromStr<Err = String>>(list: &mut Vec<T>, raw: &str) -> Result<(), String> {
    for tok in raw.split(',') {
        list.push(tok.parse()?);
    }
    Ok(())
}
