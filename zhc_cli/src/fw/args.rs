//! Command-line parsing for `fw`.

use std::path::PathBuf;

use zhc::compat::Iop;

const USAGE_LINE: &str = "Usage: fw --iop <IOP> --integer-w <INTEGER_W> [--output <OUTPUT>]";

const USAGE: &str = "\
Dumps the HPU assembly generated for a static IOp at a given integer width.

Usage: fw --iop <IOP> --integer-w <INTEGER_W> [--output <OUTPUT>]

Options:
      --iop <IOP>              Static IOp name (e.g. `Add`, `CmpGt`, `OvfMul`, `RightShift`),
                                case-insensitive. See --list for every supported name.
      --integer-w <INTEGER_W>  Integer width in bits (e.g. 32, 64).
      --output <OUTPUT>        Write the assembly to this file instead of printing it to stdout.
      --list                   Print every supported --iop name and exit.
  -h, --help                   Print help.
  -V, --version                Print version.
";

pub struct Args {
    pub iop: Iop,
    pub integer_w: u16,
    pub output: Option<PathBuf>,
}

/// Parses `fw`'s command line, or prints help/version/list and exits.
pub fn parse(argv: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut iop: Option<Iop> = None;
    let mut integer_w: Option<u16> = None;
    let mut output: Option<PathBuf> = None;

    let mut argv = argv.into_iter();
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("fw {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--list" => {
                for iop in Iop::ALL_STATIC {
                    println!("{}", iop_name(iop));
                }
                std::process::exit(0);
            }
            "--iop" => iop = Some(parse_iop(&value_of(&mut argv, "--iop")?)?),
            "--integer-w" => {
                let raw = value_of(&mut argv, "--integer-w")?;
                integer_w = Some(
                    raw.parse::<u16>()
                        .map_err(|err| format!("--integer-w: invalid value `{raw}`: {err}"))?,
                );
            }
            "--output" => output = Some(PathBuf::from(value_of(&mut argv, "--output")?)),
            other => return Err(brief(&format!("unknown argument `{other}`"))),
        }
    }

    let iop = iop.ok_or_else(|| missing("--iop"))?;
    let integer_w = integer_w.ok_or_else(|| missing("--integer-w"))?;

    Ok(Args {
        iop,
        integer_w,
        output,
    })
}

/// The name an `Iop` is selected by on the command line: its bare variant name (`Debug` prints
/// exactly that for the fieldless static variants in [`Iop::ALL_STATIC`]).
fn iop_name(iop: &Iop) -> String {
    format!("{iop:?}")
}

fn parse_iop(s: &str) -> Result<Iop, String> {
    Iop::ALL_STATIC
        .iter()
        .find(|iop| iop_name(iop).eq_ignore_ascii_case(s))
        .cloned()
        .ok_or_else(|| {
            format!("--iop: unknown static IOp `{s}` (run `fw --list` to see supported names)")
        })
}

fn missing(flag: &str) -> String {
    brief(&format!(
        "the following required argument was not provided: {flag}"
    ))
}

fn brief(message: &str) -> String {
    format!("{message}\n\n{USAGE_LINE}\n\nFor more information, try '--help'.")
}

fn value_of(argv: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    argv.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}
