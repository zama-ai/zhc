use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use zhc::{
    compat::Iop,
    prelude::{Pipeline, PipelineExt},
};
use zhc_builder::{Builder, CiphertextSpec};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig};
use zhc_utils::{
    Dumpable,
    data_visulization::DynamicTable,
    files::{Extension, FileHandle},
    units::Microseconds,
};

const ALL_BITS: &[u16] = &[8, 16, 32, 64, 128];
const RESULTS_DIR: &str = "zhc_bench/results";
const SITE_DIR: &str = "zhc_bench/site";
const DELTA_THRESHOLD: f64 = 0.5;
const DEFAULT_REPS: usize = 3;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

/// Panic reports buffered by the hook, printed at the end so they don't mangle the tables.
static PANIC_REPORTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

thread_local! {
    static PANIC_CONTEXT: RefCell<Option<String>> = const { RefCell::new(None) };
    static LAST_PANIC: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Replaces the default panic hook (which prints at panic time) with one buffering reports
/// into PANIC_REPORTS. Backtraces are kept when RUST_BACKTRACE is set.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let mut report = info.to_string();
        if std::env::var("RUST_BACKTRACE").is_ok_and(|v| v != "0") {
            report = format!("{}\n{}", report, std::backtrace::Backtrace::force_capture());
        }
        match PANIC_CONTEXT.with_borrow(|c| c.clone()) {
            Some(context) => LAST_PANIC.set(Some(format!("[{}] {}", context, report))),
            None => PANIC_REPORTS.lock().unwrap().push(report),
        }
    }));
}

/// Runs `f`, catching a panic so the remaining runs still execute. Returns None on panic,
/// tagging the buffered report with `context` to identify the run.
fn catch_panic<T>(context: &str, f: impl FnOnce() -> T) -> Option<T> {
    PANIC_CONTEXT.set(Some(context.to_string()));
    let result = catch_unwind(AssertUnwindSafe(f)).ok();
    PANIC_CONTEXT.set(None);
    if let Some(report) = LAST_PANIC.take()
        && result.is_none()
    {
        PANIC_REPORTS.lock().unwrap().push(report);
    }
    result
}

const SPINNER: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

fn run_parallel<J: Sync, R: Send>(
    label: &str,
    jobs: &[J],
    n_threads: usize,
    f: impl Fn(&J) -> R + Sync,
) -> Vec<R> {
    let show_progress = io::stderr().is_terminal();
    let next = AtomicUsize::new(0);
    let (sender, receiver) = mpsc::channel();
    let mut results: Vec<Option<R>> = (0..jobs.len()).map(|_| None).collect();

    thread::scope(|scope| {
        for _ in 0..n_threads.clamp(1, jobs.len().max(1)) {
            let sender = sender.clone();
            let (next, f) = (&next, &f);
            scope.spawn(move || {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(job) = jobs.get(index) else { break };
                    sender.send((index, f(job))).unwrap();
                }
            });
        }
        drop(sender);

        let mut done = 0;
        let mut frame = 0;
        loop {
            if show_progress {
                eprint!(
                    "\r\x1b[K{} {}: {}/{} done",
                    SPINNER[frame % SPINNER.len()],
                    label,
                    done,
                    jobs.len()
                );
                io::stderr().flush().unwrap();
                frame += 1;
            }
            match receiver.recv_timeout(Duration::from_millis(100)) {
                Ok((index, result)) => {
                    results[index] = Some(result);
                    done += 1;
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        if show_progress {
            eprint!("\r\x1b[K");
            io::stderr().flush().unwrap();
        }
    });

    results.into_iter().map(|r| r.unwrap()).collect()
}

fn cost_rank(iop: &Iop) -> u8 {
    match iop {
        Iop::Div | Iop::Divs | Iop::Mod | Iop::Mods => 0,
        Iop::Mul | Iop::Muls | Iop::OvfMul | Iop::OvfMuls => 1,
        _ => 2,
    }
}

fn bench_target(
    target: Target,
    iops: &[Iop],
    bits: &[u16],
    n_threads: usize,
    measure: impl Fn(&Iop, Target, u16) -> Option<Microseconds> + Sync,
) -> IopResults {
    let mut jobs: Vec<(Iop, u16)> = iops
        .iter()
        .flat_map(|iop| bits.iter().map(move |&b| (iop.clone(), b)))
        .collect();
    jobs.sort_by_key(|(iop, b)| (cost_rank(iop), Reverse(*b)));

    let measured = run_parallel(&target.name(), &jobs, n_threads, |(iop, b)| {
        measure(iop, target, *b)
    });

    let mut results = IopResults::new();
    for ((iop, b), latency) in jobs.iter().zip(measured) {
        let entry = results.entry(format!("{:?}", iop)).or_default();
        if let Some(latency) = latency {
            entry.insert(*b, latency);
        }
    }
    results
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Hpu,
    MultiHpu(u8),
}

impl Target {
    const ALL: &[Target] = &[
        Target::Hpu,
        Target::MultiHpu(2),
        Target::MultiHpu(4),
        Target::MultiHpu(8),
    ];

    fn name(&self) -> String {
        match self {
            Target::Hpu => "hpu".into(),
            Target::MultiHpu(n) => format!("mhpu{}", n),
        }
    }

    fn parse(name: &str) -> Option<Target> {
        match name {
            "hpu" => Some(Target::Hpu),
            _ => name
                .strip_prefix("mhpu")
                .and_then(|n| n.parse::<u8>().ok())
                .filter(|&n| n > 0)
                .map(Target::MultiHpu),
        }
    }

    fn pipeline(&self, iop: &Iop, spec: CiphertextSpec) -> Pipeline {
        match self {
            Target::Hpu => iop_pipeline(iop, &HpuConfig::default(), spec),
            Target::MultiHpu(n_hpus) => Pipeline::new()
                .with_builder(iop.to_builder(spec))
                .with_multi_hpu_config(MultiHpuConfig {
                    n_hpus: *n_hpus,
                    ..Default::default()
                }),
        }
    }

    fn latency(&self, iop: &Iop, spec: CiphertextSpec) -> Microseconds {
        let mut pipeline = self.pipeline(iop, spec);
        match self {
            Target::Hpu => pipeline.get_hpu_metrics().latency,
            Target::MultiHpu(_) => pipeline.get_multi_hpu_metrics().latency,
        }
    }

    fn compile(&self, pipeline: &mut Pipeline) {
        match self {
            Target::Hpu => {
                pipeline.get_hpu_stream();
            }
            Target::MultiHpu(_) => {
                pipeline.get_multi_hpu_stream();
            }
        }
    }
}

/// Parsed filter options from CLI arguments.
struct Filters {
    iops: Vec<Iop>,
    bits: Vec<u16>,
    targets: Vec<Target>,
    reps: usize,
    jobs: Option<usize>,
}

impl Filters {
    fn threads_or(&self, default: usize) -> usize {
        self.jobs.unwrap_or(default).max(1)
    }
}

fn all_cores() -> usize {
    thread::available_parallelism().map_or(1, |n| n.get())
}

impl Filters {
    /// Parse filters from CLI args. Returns filters and remaining args.
    fn parse(args: &[String]) -> (Self, Vec<String>) {
        let mut iop_patterns: Vec<String> = vec![];
        let mut bit_values: Vec<u16> = vec![];
        let mut target_names: Vec<String> = vec![];
        let mut reps = DEFAULT_REPS;
        let mut jobs = None;
        let mut remaining = vec![];
        let mut iter = args.iter().peekable();

        while let Some(arg) = iter.next() {
            if arg == "-i" || arg == "--iops" {
                if let Some(val) = iter.next() {
                    iop_patterns.extend(val.split(',').map(|s| s.trim().to_lowercase()));
                }
            } else if let Some(val) = arg.strip_prefix("--iops=") {
                iop_patterns.extend(val.split(',').map(|s| s.trim().to_lowercase()));
            } else if arg == "-b" || arg == "--bits" {
                if let Some(val) = iter.next() {
                    bit_values.extend(val.split(',').filter_map(|s| s.trim().parse::<u16>().ok()));
                }
            } else if let Some(val) = arg.strip_prefix("--bits=") {
                bit_values.extend(val.split(',').filter_map(|s| s.trim().parse::<u16>().ok()));
            } else if arg == "-t" || arg == "--targets" {
                if let Some(val) = iter.next() {
                    target_names.extend(val.split(',').map(|s| s.trim().to_lowercase()));
                }
            } else if let Some(val) = arg.strip_prefix("--targets=") {
                target_names.extend(val.split(',').map(|s| s.trim().to_lowercase()));
            } else if arg == "-r" || arg == "--reps" {
                if let Some(val) = iter.next() {
                    reps = val.trim().parse().unwrap_or(DEFAULT_REPS);
                }
            } else if let Some(val) = arg.strip_prefix("--reps=") {
                reps = val.trim().parse().unwrap_or(DEFAULT_REPS);
            } else if arg == "-j" || arg == "--jobs" {
                if let Some(val) = iter.next() {
                    jobs = val.trim().parse().ok();
                }
            } else if let Some(val) = arg.strip_prefix("--jobs=") {
                jobs = val.trim().parse().ok();
            } else {
                remaining.push(arg.clone());
            }
        }

        // Filter iops by case-insensitive substring match
        let iops: Vec<Iop> = if iop_patterns.is_empty() {
            Iop::TEST_ITER.to_vec()
        } else {
            Iop::TEST_ITER
                .iter()
                .filter(|iop| {
                    let name = format!("{:?}", iop).to_lowercase();
                    iop_patterns.iter().any(|p| name.contains(p))
                })
                .cloned()
                .collect()
        };

        // Filter bits, defaulting to all if none specified
        let bits: Vec<u16> = if bit_values.is_empty() {
            ALL_BITS.to_vec()
        } else {
            ALL_BITS
                .iter()
                .filter(|b| bit_values.contains(b))
                .copied()
                .collect()
        };

        let targets: Vec<Target> = if target_names.is_empty() {
            Target::ALL.to_vec()
        } else {
            target_names
                .iter()
                .map(|name| {
                    Target::parse(name).unwrap_or_else(|| {
                        eprintln!("Error: unknown target '{}' (use hpu or mhpuN)", name);
                        std::process::exit(1);
                    })
                })
                .collect()
        };

        (
            Self {
                iops,
                bits,
                targets,
                reps: reps.max(1),
                jobs,
            },
            remaining,
        )
    }
}

type IopResults = BTreeMap<String, BTreeMap<u16, Microseconds>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchResult {
    commit: String,
    timestamp: String,
    results: BTreeMap<String, IopResults>,
}

impl BenchResult {
    fn get(&self, target: Target, iop: &str, bits: u16) -> Option<Microseconds> {
        self.results
            .get(&target.name())
            .and_then(|iops| iops.get(iop))
            .and_then(|m| m.get(&bits))
            .copied()
    }
}

#[derive(Deserialize)]
struct LegacyBenchResult {
    commit: String,
    timestamp: String,
    results: IopResults,
}

fn parse_result(content: &str) -> BenchResult {
    if let Ok(result) = serde_json::from_str::<BenchResult>(content) {
        return result;
    }
    let legacy: LegacyBenchResult = serde_json::from_str(content).expect("failed to parse");
    BenchResult {
        commit: legacy.commit,
        timestamp: legacy.timestamp,
        results: BTreeMap::from([(Target::Hpu.name(), legacy.results)]),
    }
}

fn run_target_tables(
    filters: &Filters,
    n_threads: usize,
    measure: impl Fn(&Iop, Target, u16) -> Option<Microseconds> + Sync,
    cell: impl Fn(Target, &str, u16, Option<Microseconds>) -> String,
) {
    for (i, &target) in filters.targets.iter().enumerate() {
        let results = bench_target(target, &filters.iops, &filters.bits, n_threads, &measure);
        if i > 0 {
            println!();
        }
        println!("{}", target.name());
        let columns = filters.bits.iter().map(|b| format!("{}b", b));
        let rows = filters.iops.iter().map(|iop| format!("{:?}", iop));
        let mut table = DynamicTable::new(columns, rows);
        for (row, iop) in filters.iops.iter().enumerate() {
            let iop_name = format!("{:?}", iop);
            for (col, &bits) in filters.bits.iter().enumerate() {
                let value = results.get(&iop_name).and_then(|m| m.get(&bits)).copied();
                table.set(row, col, cell(target, &iop_name, bits, value));
            }
        }
        table.finish();
    }
}

fn get_commit_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("failed to get commit hash");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_commit_short() -> String {
    resolve_rev_short("HEAD")
}

fn resolve_rev_short(rev: &str) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--short", rev])
        .output()
        .expect("failed to resolve revision");
    if !output.status.success() {
        eprintln!("Error: unknown revision '{}'", rev);
        std::process::exit(1);
    }
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_timestamp() -> String {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("failed to get timestamp");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn check_git_clean() {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("failed to check git status");
    let status = String::from_utf8_lossy(&output.stdout);
    if !status.trim().is_empty() {
        eprintln!("Error: git tree is dirty. Commit your changes before running benchmarks.");
        std::process::exit(1);
    }
}

fn measure_latency(iop: &Iop, target: Target, bits: u16) -> Option<Microseconds> {
    let spec = CiphertextSpec::new(bits, 2, 2);
    let context = format!("{:?} {}b {} latency", iop, bits, target.name());
    catch_panic(&context, || target.latency(iop, spec))
}

fn median(samples: &mut [f64]) -> f64 {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len();
    if n % 2 == 1 {
        samples[n / 2]
    } else {
        (samples[n / 2 - 1] + samples[n / 2]) / 2.0
    }
}

/// Returns the pipeline compiling this iop, with the same scheduler `compute_latency` picks.
fn iop_pipeline(iop: &Iop, config: &HpuConfig, spec: CiphertextSpec) -> Pipeline {
    let pipeline = Pipeline::new()
        .with_builder(iop.to_builder(spec))
        .with_hpu_config(config.clone());
    match (iop, spec.int_size()) {
        (Iop::Mul, _)
        | (Iop::OvfMul, _)
        | (Iop::RightRot | Iop::LeftRot | Iop::LeftShift | Iop::RightShift, 128) => {
            pipeline.with_legacy_hpu_scheduler()
        }
        _ => pipeline,
    }
}

/// Measures the compile time of one iop: median wall-clock over `reps` compilations.
fn measure_compile(iop: &Iop, target: Target, bits: u16, reps: usize) -> Option<Microseconds> {
    let spec = CiphertextSpec::new(bits, 2, 2);
    // A pipeline caches its steps, so each repetition compiles on a fresh one.
    let context = format!("{:?} {}b {} compile", iop, bits, target.name());
    let mut samples: Vec<f64> = catch_panic(&context, || {
        (0..reps)
            .map(|_| {
                let mut pipeline = target.pipeline(iop, spec);
                let tic = Instant::now();
                target.compile(&mut pipeline);
                tic.elapsed().as_secs_f64() * 1e6
            })
            .collect()
    })?;
    Some(Microseconds(median(&mut samples)))
}

fn run_benchmarks(n_threads: usize) -> BenchResult {
    let mut results: BTreeMap<String, IopResults> = BTreeMap::new();

    for &target in Target::ALL {
        let iop_results =
            bench_target(target, Iop::TEST_ITER, ALL_BITS, n_threads, measure_latency);
        let n_measured: usize = iop_results.values().map(|m| m.len()).sum();
        println!(
            "Benchmarked {}: {}/{} runs",
            target.name(),
            n_measured,
            Iop::TEST_ITER.len() * ALL_BITS.len()
        );
        results.insert(target.name(), iop_results);
    }

    BenchResult {
        commit: get_commit_hash(),
        timestamp: get_timestamp(),
        results,
    }
}

fn save_result(result: &BenchResult) {
    let dir = PathBuf::from(RESULTS_DIR);
    fs::create_dir_all(&dir).expect("failed to create results dir");

    let filename = format!("{}.json", get_commit_short());
    let path = dir.join(filename);

    let json = serde_json::to_string_pretty(result).expect("failed to serialize");
    fs::write(&path, json).expect("failed to write result");
    println!("Saved results to {}", path.display());
}

fn load_all_results() -> Vec<BenchResult> {
    let dir = PathBuf::from(RESULTS_DIR);
    if !dir.exists() {
        return vec![];
    }

    let mut results = vec![];
    for entry in fs::read_dir(&dir).expect("failed to read results dir") {
        let entry = entry.expect("failed to read entry");
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            let content = fs::read_to_string(&path).expect("failed to read file");
            results.push(parse_result(&content));
        }
    }

    results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    results
}

fn load_result_by_rev(rev: &str) -> Option<BenchResult> {
    let short = resolve_rev_short(rev);
    let path = PathBuf::from(RESULTS_DIR).join(format!("{}.json", short));
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(&path).expect("failed to read file");
    Some(parse_result(&content))
}

fn find_latest_baseline() -> Option<BenchResult> {
    load_all_results().into_iter().last()
}

fn list_available_baselines() -> Vec<String> {
    let dir = PathBuf::from(RESULTS_DIR);
    if !dir.exists() {
        return vec![];
    }
    fs::read_dir(&dir)
        .expect("failed to read results dir")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .collect()
}

fn format_latency(us: f64) -> String {
    let int_part = us.round() as u64;
    let int_str = int_part
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    format!("{} µs", int_str)
}

/// Formats a wall-clock duration with a unit fitting its magnitude.
fn format_compile_time(us: f64) -> String {
    if us >= 1_000_000.0 {
        format!("{:.2} s", us / 1_000_000.0)
    } else if us >= 1_000.0 {
        format!("{:.2} ms", us / 1_000.0)
    } else {
        format!("{:.0} µs", us)
    }
}

fn format_diff(curr: f64, base: f64, use_color: bool) -> String {
    if base == 0.0 {
        return "-".into();
    }
    let pct = (curr - base) / base * 100.0;
    let sign = if pct >= 0.0 { "+" } else { "" };
    let text = format!("{}{:.1}%", sign, pct);
    if !use_color || pct.abs() < DELTA_THRESHOLD {
        return text;
    }
    if pct > 0.0 {
        format!("{}{}{}", RED, text, RESET)
    } else {
        format!("{}{}{}", GREEN, text, RESET)
    }
}

fn run_diff_incremental(baseline: &BenchResult, use_color: bool, filters: &Filters) {
    let baseline_short = &baseline.commit[..7.min(baseline.commit.len())];
    let baseline_date = &baseline.timestamp[..10.min(baseline.timestamp.len())];
    println!("vs {} ({})\n", baseline_short, baseline_date);

    run_target_tables(
        filters,
        filters.threads_or(all_cores()),
        measure_latency,
        |target, iop_name, bits, value| match (value, baseline.get(target, iop_name, bits)) {
            (Some(curr), Some(base)) => format_diff(curr.0, base.0, use_color),
            (None, _) => "panic!".into(),
            _ => "-".into(),
        },
    );
}

fn run_latency_table(filters: &Filters) {
    run_target_tables(
        filters,
        filters.threads_or(all_cores()),
        measure_latency,
        |_, _, _, value| match value {
            Some(us) => format_latency(us.0),
            None => "panic!".into(),
        },
    );
}

/// Prints compile times as an iops x bits table.
fn run_compile_table(filters: &Filters) {
    run_target_tables(
        filters,
        filters.threads_or(1),
        |iop, target, bits| measure_compile(iop, target, bits, filters.reps),
        |_, _, _, value| match value {
            Some(us) => format_compile_time(us.0),
            None => "panic!".into(),
        },
    );
}

/// Customize this function during development to analyze the IR.
fn analyze_ir(builder: &Builder) -> String {
    let ir = builder.optimize_ir();
    format!("{} ops", ir.n_ops())
}

fn run_analyze(filters: &Filters) {
    let columns = filters.bits.iter().map(|b| format!("{}b", b));
    let rows = filters.iops.iter().map(|iop| format!("{:?}", iop));
    let mut table = DynamicTable::new(columns, rows);

    for (row, iop) in filters.iops.iter().enumerate() {
        for (col, bits) in filters.bits.iter().enumerate() {
            let spec = CiphertextSpec::new(*bits, 2, 2);
            let context = format!("{:?} {}b analyze", iop, bits);
            let cell = catch_panic(&context, || analyze_ir(&iop.to_builder(spec)))
                .unwrap_or_else(|| "panic!".into());
            table.set(row, col, cell);
        }
    }

    table.finish();
}

fn generate_html(results: &[BenchResult]) {
    let dir = PathBuf::from(SITE_DIR);
    fs::create_dir_all(&dir).expect("failed to create site dir");

    let data_json = serde_json::to_string(results).expect("failed to serialize");

    let html = format!(
        r##"<!DOCTYPE html>
<html>
<head>
    <title>ZHC Benchmark Results</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: system-ui, sans-serif; margin: 2rem; background: #1a1a2e; color: #eee; }}
        h1 {{ color: #00d4ff; }}
        .charts {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(500px, 1fr)); gap: 2rem; }}
        .chart-container {{ background: #16213e; padding: 1rem; border-radius: 8px; }}
        canvas {{ max-height: 300px; }}
        table {{ border-collapse: collapse; width: 100%; margin-top: 2rem; }}
        th, td {{ border: 1px solid #333; padding: 8px; text-align: right; }}
        th {{ background: #16213e; }}
        tr:nth-child(even) {{ background: #1a1a2e; }}
        tr:nth-child(odd) {{ background: #16213e; }}
    </style>
</head>
<body>
    <h1>ZHC Benchmark Results</h1>
    <div class="charts" id="charts"></div>
    <h2>Latest Results (μs)</h2>
    <div id="table"></div>
    <script>
        const DATA = {data_json};
        const BITS = [8, 16, 32, 64, 128];
        const COLORS = ['#ff6384', '#36a2eb', '#ffce56', '#4bc0c0', '#9966ff'];

        const latestResults = DATA.length > 0 ? DATA[DATA.length - 1].results : {{}};
        const targets = Object.keys(latestResults);
        const series = targets.flatMap(target =>
            Object.keys(latestResults[target]).map(iop => ({{ target, iop }})));
        const label = (s) => targets.length > 1 ? `${{s.iop}} [${{s.target}}]` : s.iop;

        const chartsDiv = document.getElementById('charts');
        series.forEach((s, idx) => {{
            const container = document.createElement('div');
            container.className = 'chart-container';
            container.innerHTML = `<canvas id="chart-${{idx}}"></canvas>`;
            chartsDiv.appendChild(container);

            const ctx = document.getElementById(`chart-${{idx}}`).getContext('2d');
            const datasets = BITS.map((bits, i) => ({{
                label: `${{bits}}b`,
                data: DATA.map(r => r.results[s.target]?.[s.iop]?.[bits] ?? null),
                borderColor: COLORS[i],
                tension: 0.1,
                fill: false,
            }}));

            new Chart(ctx, {{
                type: 'line',
                data: {{
                    labels: DATA.map(r => r.commit.slice(0, 7)),
                    datasets,
                }},
                options: {{
                    responsive: true,
                    plugins: {{
                        title: {{ display: true, text: label(s), color: '#00d4ff' }},
                        legend: {{ labels: {{ color: '#eee' }} }},
                    }},
                    scales: {{
                        x: {{ ticks: {{ color: '#aaa' }}, grid: {{ color: '#333' }} }},
                        y: {{
                            type: 'logarithmic',
                            ticks: {{ color: '#aaa' }},
                            grid: {{ color: '#333' }},
                            title: {{ display: true, text: 'Latency (μs)', color: '#aaa' }}
                        }},
                    }},
                }},
            }});
        }});

        // Format number with space as thousand separator
        function fmt(val) {{
            const [int, dec] = val.toFixed(2).split('.');
            const spaced = int.replace(/\B(?=(\d{{3}})+(?!\d))/g, ' ');
            return spaced + '.' + dec;
        }}

        // Generate table with latest results
        let html = '';
        targets.forEach(target => {{
            html += `<h3>${{target}}</h3><table><tr><th>Operation</th>`;
            BITS.forEach(b => html += `<th>${{b}}b</th>`);
            html += '</tr>';
            Object.keys(latestResults[target]).forEach(iop => {{
                html += `<tr><td style="text-align:left">${{iop}}</td>`;
                BITS.forEach(b => {{
                    const val = latestResults[target][iop]?.[b];
                    html += `<td>${{val ? fmt(val) : '-'}}</td>`;
                }});
                html += '</tr>';
            }});
            html += '</table>';
        }});
        document.getElementById('table').innerHTML = html;
    </script>
</body>
</html>
"##
    );

    let path = dir.join("index.html");
    fs::write(&path, html).expect("failed to write html");
    println!("Generated {}", path.display());
}

/// Resolves the baseline to diff against: a given revision, or the latest saved result.
fn resolve_baseline(rev_arg: Option<&String>) -> BenchResult {
    if let Some(rev) = rev_arg {
        match load_result_by_rev(rev) {
            Some(b) => b,
            None => {
                let available = list_available_baselines();
                eprintln!("Error: no saved results for '{}'", rev);
                if available.is_empty() {
                    eprintln!("No baselines available. Run 'zhc_bench export' first.");
                } else {
                    eprintln!("Available baselines: {}", available.join(", "));
                }
                std::process::exit(1);
            }
        }
    } else {
        match find_latest_baseline() {
            Some(b) => b,
            None => {
                eprintln!("Error: no baseline found.");
                eprintln!("Run 'zhc_bench export' on a commit first.");
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    install_panic_hook();
    let args: Vec<String> = std::env::args().collect();
    let (filters, remaining) = Filters::parse(&args[1..]);
    let cmd = remaining.first().map(|s| s.as_str()).unwrap_or("run");

    if filters.iops.is_empty() {
        eprintln!("Error: no iops match the filter");
        std::process::exit(1);
    }
    if filters.bits.is_empty() {
        eprintln!("Error: no bits match the filter");
        std::process::exit(1);
    }

    match cmd {
        "run" => {
            run_latency_table(&filters);
        }
        "analyze" => {
            run_analyze(&filters);
        }
        "compile" => {
            run_compile_table(&filters);
        }
        "export" => {
            check_git_clean();
            let result = run_benchmarks(filters.threads_or(all_cores()));
            save_result(&result);
            let all = load_all_results();
            generate_html(&all);
        }
        "diff" => {
            let use_color = !remaining.iter().any(|a| a == "--no-color");
            let rev_arg = remaining.iter().skip(1).find(|a| !a.starts_with("--"));
            let baseline = resolve_baseline(rev_arg);
            run_diff_incremental(&baseline, use_color, &filters);
        }
        _ => {
            eprintln!("Usage: zhc_bench [run|export|diff|compile|analyze] [OPTIONS]");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  run                     - Run benchmarks and display latency table");
            eprintln!("  analyze                 - Run custom IR analysis (edit analyze_ir fn)");
            eprintln!(
                "  export                  - Run benchmarks, save results, and regenerate site"
            );
            eprintln!(
                "  diff [REV] [--no-color] - Compare latencies against REV (default: latest baseline)"
            );
            eprintln!(
                "  compile                 - Measure compiler wall-clock times (local only, not stored)"
            );
            eprintln!();
            eprintln!("Filter options (for run, diff, and compile):");
            eprintln!(
                "  -i, --iops=PATTERNS     - Comma-separated iop name patterns (case-insensitive substring match)"
            );
            eprintln!("  -b, --bits=VALUES       - Comma-separated bit widths (8,16,32,64,128)");
            eprintln!(
                "  -t, --targets=NAMES     - Comma-separated targets: hpu, mhpuN (default: hpu,mhpu2,mhpu4,mhpu8)"
            );
            eprintln!(
                "  -r, --reps=N            - Compile-time repetitions, median kept (default: {})",
                DEFAULT_REPS
            );
            eprintln!(
                "  -j, --jobs=N            - Worker threads (default: all cores, 1 for compile)"
            );
            eprintln!();
            eprintln!("Examples:");
            eprintln!("  zhc_bench run -i mul,div -b 8,16");
            eprintln!("  zhc_bench diff --iops=cmp --bits=64");
            eprintln!("  zhc_bench compile -i mul -b 8,16");
            eprintln!("  zhc_bench run -i add -t hpu,mhpu2,mhpu4");
        }
    }

    let reports = PANIC_REPORTS.lock().unwrap();
    if !reports.is_empty() {
        let file = FileHandle::random(Extension::Txt);
        reports.join("\n\n").dump_to_file(&file);
        eprintln!("\nError: {} run(s) panicked.", reports.len());
        eprintln!("Panic traces dumped to {}", file.as_ref().display());
        std::process::exit(1);
    }
}
