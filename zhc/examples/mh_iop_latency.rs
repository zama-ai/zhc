//! Simulated latency of any built-in IOP spread over several HPU boards.
//!
//! Run with:
//!   cargo run --release -p zhc --example mh_iop_latency -- <iop>[:param] [bits] [n_hpus...]
//! [--manual|--auto]
//!
//! `<iop>` is matched like in `iop_latency` (`mul`, `Mul`, `div`...), case and `_`/`-` ignored.
//! IOPs with a parameter take it after a colon: `sum:5`, `cast:8`, `erc7984simd:64`,
//! `addsimd:16`; without it, the first variant of the IOP lists is used. For `mhmul`, `par_n`
//! follows the board count (`MhMul { par_n: n_hpus }`, and `par_n = 2` for the 1-HPU reference).
//!
//! `bits` defaults to 64. The board counts default to `1 2 4 8`; 1 uses the single-HPU
//! simulation for reference.
//!
//! Partitioning: automatic by default; `--manual` uses the partitions the builder declared (the
//! hand-made per-board groups of `MhMul`). `MhMul` defaults to `--manual`, the configuration it
//! is designed for.

use zhc::PipelineExt;
use zhc::compat::Iop;
use zhc_builder::SpecInteger;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_langs::doplang::DopInstructionSet;
use zhc_pipeline::Pipeline;

fn variant_name(iop: &Iop) -> String {
    let dbg = format!("{iop:?}");
    dbg.split([' ', '{', '('])
        .next()
        .unwrap_or(&dbg)
        .to_string()
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| *c != '_' && *c != '-')
        .collect::<String>()
        .to_lowercase()
}

/// The IOP to build for `n_hpus` boards.
fn resolve(name: &str, param: Option<u16>, n_hpus: u8) -> Iop {
    let key = normalize(name);
    match (key.as_str(), param) {
        ("mhmul", _) => {
            return Iop::MhMul {
                par_n: n_hpus.max(2),
            };
        }
        ("sum", Some(n)) => return Iop::Sum { n },
        ("cast", Some(to_size)) => return Iop::Cast { to_size },
        ("erc7984simd", Some(n)) => return Iop::Erc7984Simd { n },
        ("addsimd", Some(n)) => return Iop::AddSimd { n },
        (_, Some(_)) => panic!("`{name}` takes no parameter"),
        _ => {}
    }
    // Parameterized IOPs (Sum, Cast, MhMul...) are only in TEST_ITER.
    Iop::ALL_STATIC
        .iter()
        .chain(Iop::TEST_ITER.iter())
        .find(|iop| normalize(&variant_name(iop)) == key)
        .cloned()
        .unwrap_or_else(|| panic!("unknown IOP `{name}`"))
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let manual_flag = args
        .iter()
        .position(|a| a == "--manual")
        .map(|i| args.remove(i));
    let auto_flag = args
        .iter()
        .position(|a| a == "--auto")
        .map(|i| args.remove(i));
    let spec_arg = args
        .first()
        .expect("usage: mh_iop_latency <iop>[:param] [bits] [n_hpus...] [--manual|--auto]");
    let (name, param) = match spec_arg.split_once(':') {
        Some((n, p)) => (
            n.to_string(),
            Some(p.parse::<u16>().expect("parameter must be an integer")),
        ),
        None => (spec_arg.clone(), None),
    };
    let bits: u16 = args
        .get(1)
        .map_or(64, |s| s.parse().expect("bits must be an integer"));
    let boards: Vec<u8> = if args.len() > 2 {
        args[2..]
            .iter()
            .map(|s| s.parse().expect("n_hpus must be an integer"))
            .collect()
    } else {
        vec![1, 2, 4, 8]
    };
    let manual = match (manual_flag, auto_flag) {
        (Some(_), Some(_)) => panic!("--manual and --auto are exclusive"),
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => normalize(&name) == "mhmul",
    };
    let spec = SpecInteger::new(bits, 2, 2);

    let first = resolve(&name, param, boards.first().copied().unwrap_or(1));
    println!(
        "{} on {bits} bits, {} partitioning",
        variant_name(&first),
        if manual { "manual" } else { "automatic" }
    );
    let mut reference = None;
    for n_hpus in boards {
        let iop = resolve(&name, param, n_hpus);
        let latency = if n_hpus == 1 {
            let mut pl = Pipeline::new()
                .with_builder(iop.to_builder(spec))
                .with_hpu_config(Default::default());
            let l = pl.get_hpu_metrics().latency;
            println!("  1 HPU : {l}  ({iop:?})");
            l.0
        } else {
            let config = MultiHpuConfig {
                n_hpus,
                ..Default::default()
            };
            let mut pl = Pipeline::new()
                .with_builder(iop.to_builder(spec))
                .with_builder_partitions(manual)
                .with_multi_hpu_config(config);
            let transfers: Vec<usize> = pl
                .get_multi_doplang()
                .iter()
                .map(|ir| {
                    ir.walk_ops_linear()
                        .filter(|op| matches!(op.get_instruction(), DopInstructionSet::WAIT { .. }))
                        .count()
                })
                .collect();
            let l = pl.get_multi_hpu_metrics().latency;
            println!(
                "  {n_hpus} HPUs: {l}  ({iop:?}; transfers received per board: {transfers:?})"
            );
            l.0
        };
        match reference {
            None => reference = Some(latency),
            Some(r) => println!("          speed-up vs first: x{:.2}", r / latency),
        }
    }
}
