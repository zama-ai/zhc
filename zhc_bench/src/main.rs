use std::collections::BTreeMap;
use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use zhc_builder::{Builder, CiphertextSpec};
use zhc_config::hpu::HpuConfig;
use zhc_pipeline::{Pipeline, compat::Iop};
use zhc_utils::{data_visulization::DynamicTable, units::Microseconds};

const ALL_BITS: &[u16] = &[8, 16, 32, 64, 128];
const RESULTS_DIR: &str = "zhc_bench/results";
const SITE_DIR: &str = "zhc_bench/site";
const DELTA_THRESHOLD: f64 = 0.5;
// Compile times are wall-clock measurements, noisier than the deterministic latency model, so
// diffs only get colored past a larger delta.
const COMPILE_DELTA_THRESHOLD: f64 = 5.0;
const DEFAULT_REPS: usize = 3;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

/// Panic reports buffered by the hook, printed at the end so they don't mangle the tables.
static PANIC_REPORTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Replaces the default panic hook (which prints at panic time) with one buffering reports
/// into PANIC_REPORTS. Backtraces are kept when RUST_BACKTRACE is set.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let mut report = info.to_string();
        if std::env::var("RUST_BACKTRACE").is_ok_and(|v| v != "0") {
            report = format!("{}\n{}", report, std::backtrace::Backtrace::force_capture());
        }
        PANIC_REPORTS.lock().unwrap().push(report);
    }));
}

/// Runs `f`, catching a panic so the remaining runs still execute. Returns None on panic,
/// tagging the buffered report with `context` to identify the run.
fn catch_panic<T>(context: &str, f: impl FnOnce() -> T) -> Option<T> {
    let result = catch_unwind(AssertUnwindSafe(f)).ok();
    if result.is_none()
        && let Some(report) = PANIC_REPORTS.lock().unwrap().last_mut()
    {
        *report = format!("[{}] {}", context, report);
    }
    result
}

/// Parsed filter options from CLI arguments.
struct Filters {
    iops: Vec<Iop>,
    bits: Vec<u16>,
    reps: usize,
}

impl Filters {
    /// Parse filters from CLI args. Returns filters and remaining args.
    fn parse(args: &[String]) -> (Self, Vec<String>) {
        let mut iop_patterns: Vec<String> = vec![];
        let mut bit_values: Vec<u16> = vec![];
        let mut reps = DEFAULT_REPS;
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
            } else if arg == "-r" || arg == "--reps" {
                if let Some(val) = iter.next() {
                    reps = val.trim().parse().unwrap_or(DEFAULT_REPS);
                }
            } else if let Some(val) = arg.strip_prefix("--reps=") {
                reps = val.trim().parse().unwrap_or(DEFAULT_REPS);
            } else {
                remaining.push(arg.clone());
            }
        }

        // Filter iops by case-insensitive substring match
        let iops: Vec<Iop> = if iop_patterns.is_empty() {
            Iop::ALL.to_vec()
        } else {
            Iop::ALL
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

        (
            Self {
                iops,
                bits,
                reps: reps.max(1),
            },
            remaining,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchResult {
    commit: String,
    timestamp: String,
    results: BTreeMap<String, BTreeMap<u16, Microseconds>>,
    /// Wall-clock time taken to compile each iop's instruction stream, per bit width.
    /// Defaults to empty when loading baselines recorded before compile times existed.
    #[serde(default)]
    compile: BTreeMap<String, BTreeMap<u16, Microseconds>>,
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

fn bench_iop(iop: &Iop, config: &HpuConfig, bits_filter: &[u16]) -> BTreeMap<u16, Microseconds> {
    let mut bits_results = BTreeMap::new();
    for &bits in bits_filter {
        let spec = CiphertextSpec::new(bits, 2, 2);
        let context = format!("{:?} {}b latency", iop, bits);
        if let Some(latency) = catch_panic(&context, || iop.compute_latency(&config, spec)) {
            bits_results.insert(bits, latency);
        }
    }
    bits_results
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

/// Measures compile times for one iop: median wall-clock over `reps` compilations per bit width.
fn bench_compile_iop(
    iop: &Iop,
    config: &HpuConfig,
    bits_filter: &[u16],
    reps: usize,
) -> BTreeMap<u16, Microseconds> {
    let mut bits_results = BTreeMap::new();
    for &bits in bits_filter {
        let spec = CiphertextSpec::new(bits, 2, 2);
        // A pipeline caches its steps, so each repetition compiles on a fresh one.
        let context = format!("{:?} {}b compile", iop, bits);
        let samples: Option<Vec<f64>> = catch_panic(&context, || {
            (0..reps)
                .map(|_| {
                    let mut pipeline = iop_pipeline(iop, config, spec);
                    let tic = Instant::now();
                    pipeline.get_hpu_stream();
                    tic.elapsed().as_secs_f64() * 1e6
                })
                .collect()
        });
        if let Some(mut samples) = samples {
            bits_results.insert(bits, Microseconds(median(&mut samples)));
        }
    }
    bits_results
}

fn run_benchmarks(reps: usize) -> BenchResult {
    let config = HpuConfig::default();
    let mut results: BTreeMap<String, BTreeMap<u16, Microseconds>> = BTreeMap::new();
    let mut compile: BTreeMap<String, BTreeMap<u16, Microseconds>> = BTreeMap::new();

    for iop in Iop::ALL {
        let iop_name = format!("{:?}", iop);
        println!("Benchmarking {}", iop_name);
        let bits_results = bench_iop(iop, &config, ALL_BITS);
        for (&bits, &latency) in &bits_results {
            println!("  {}b: {}", bits, latency);
        }
        results.insert(iop_name.clone(), bits_results);

        let compile_results = bench_compile_iop(iop, &config, ALL_BITS, reps);
        for (&bits, &us) in &compile_results {
            println!("  {}b compile: {}", bits, format_compile_time(us.0));
        }
        compile.insert(iop_name, compile_results);
    }

    BenchResult {
        commit: get_commit_hash(),
        timestamp: get_timestamp(),
        results,
        compile,
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
            let result: BenchResult = serde_json::from_str(&content).expect("failed to parse");
            results.push(result);
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
    Some(serde_json::from_str(&content).expect("failed to parse"))
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

fn format_diff(curr: f64, base: f64, use_color: bool, threshold: f64) -> String {
    if base == 0.0 {
        return "-".into();
    }
    let pct = (curr - base) / base * 100.0;
    let sign = if pct >= 0.0 { "+" } else { "" };
    let text = format!("{}{:.1}%", sign, pct);
    if !use_color || pct.abs() < threshold {
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

    let columns = filters.bits.iter().map(|b| format!("{}b", b));
    let rows = filters.iops.iter().map(|iop| format!("{:?}", iop));
    let mut table = DynamicTable::new(columns, rows);

    let config = HpuConfig::default();

    for (row, iop) in filters.iops.iter().enumerate() {
        let iop_name = format!("{:?}", iop);
        let bits_results = bench_iop(iop, &config, &filters.bits);

        for (col, bits) in filters.bits.iter().enumerate() {
            let cell = match (
                bits_results.get(bits),
                baseline.results.get(&iop_name).and_then(|m| m.get(bits)),
            ) {
                (Some(&curr), Some(&base)) => {
                    format_diff(curr.0, base.0, use_color, DELTA_THRESHOLD)
                }
                (None, _) => "panic!".into(),
                _ => "-".into(),
            };
            table.set(row, col, cell);
        }
    }

    table.finish();
}

fn run_latency_table(filters: &Filters) {
    let columns = filters.bits.iter().map(|b| format!("{}b", b));
    let rows = filters.iops.iter().map(|iop| format!("{:?}", iop));
    let mut table = DynamicTable::new(columns, rows);

    let config = HpuConfig::default();

    for (row, iop) in filters.iops.iter().enumerate() {
        let bits_results = bench_iop(iop, &config, &filters.bits);

        for (col, bits) in filters.bits.iter().enumerate() {
            let cell = match bits_results.get(bits) {
                Some(&us) => format_latency(us.0),
                None => "panic!".into(),
            };
            table.set(row, col, cell);
        }
    }

    table.finish();
}

/// Prints compile times as an iops x bits table.
fn run_compile_table(filters: &Filters) {
    let columns = filters.bits.iter().map(|b| format!("{}b", b));
    let rows = filters.iops.iter().map(|iop| format!("{:?}", iop));
    let mut table = DynamicTable::new(columns, rows);

    let config = HpuConfig::default();

    for (row, iop) in filters.iops.iter().enumerate() {
        let bits_results = bench_compile_iop(iop, &config, &filters.bits, filters.reps);
        for (col, bits) in filters.bits.iter().enumerate() {
            let cell = match bits_results.get(bits) {
                Some(&us) => format_compile_time(us.0),
                None => "panic!".into(),
            };
            table.set(row, col, cell);
        }
    }

    table.finish();
}

/// Diffs compile times against a baseline as an iops x bits table.
fn run_compile_diff(baseline: &BenchResult, use_color: bool, filters: &Filters) {
    let baseline_short = &baseline.commit[..7.min(baseline.commit.len())];
    let baseline_date = &baseline.timestamp[..10.min(baseline.timestamp.len())];

    if baseline.compile.is_empty() {
        eprintln!(
            "Error: baseline {} has no compile-time data.",
            baseline_short
        );
        eprintln!("Re-run 'zhc_bench export' on that commit to record it.");
        std::process::exit(1);
    }

    println!("vs {} ({})\n", baseline_short, baseline_date);

    let columns = filters.bits.iter().map(|b| format!("{}b", b));
    let rows = filters.iops.iter().map(|iop| format!("{:?}", iop));
    let mut table = DynamicTable::new(columns, rows);

    let config = HpuConfig::default();

    for (row, iop) in filters.iops.iter().enumerate() {
        let iop_name = format!("{:?}", iop);
        let bits_results = bench_compile_iop(iop, &config, &filters.bits, filters.reps);
        for (col, bits) in filters.bits.iter().enumerate() {
            let cell = match (
                bits_results.get(bits),
                baseline.compile.get(&iop_name).and_then(|m| m.get(bits)),
            ) {
                (Some(&curr), Some(&base)) => {
                    format_diff(curr.0, base.0, use_color, COMPILE_DELTA_THRESHOLD)
                }
                (None, _) => "panic!".into(),
                _ => "-".into(),
            };
            table.set(row, col, cell);
        }
    }

    table.finish();
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
    <h2>Compile Times</h2>
    <div class="charts" id="compile-charts"></div>
    <h2>Latest Compile Times (ms)</h2>
    <div id="compile-table"></div>
    <script>
        const DATA = {data_json};
        const BITS = [8, 16, 32, 64, 128];
        const COLORS = ['#ff6384', '#36a2eb', '#ffce56', '#4bc0c0', '#9966ff'];

        // Get all IOPs from the latest result
        const iops = DATA.length > 0 ? Object.keys(DATA[DATA.length - 1].results) : [];

        // Create a chart for each IOP
        const chartsDiv = document.getElementById('charts');
        iops.forEach(iop => {{
            const container = document.createElement('div');
            container.className = 'chart-container';
            container.innerHTML = `<canvas id="chart-${{iop}}"></canvas>`;
            chartsDiv.appendChild(container);

            const ctx = document.getElementById(`chart-${{iop}}`).getContext('2d');
            const datasets = BITS.map((bits, i) => ({{
                label: `${{bits}}b`,
                data: DATA.map(r => r.results[iop]?.[bits] ?? null),
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
                        title: {{ display: true, text: iop, color: '#00d4ff' }},
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
        if (DATA.length > 0) {{
            const latest = DATA[DATA.length - 1];
            let html = '<table><tr><th>Operation</th>';
            BITS.forEach(b => html += `<th>${{b}}b</th>`);
            html += '</tr>';
            iops.forEach(iop => {{
                html += `<tr><td style="text-align:left">${{iop}}</td>`;
                BITS.forEach(b => {{
                    const val = latest.results[iop]?.[b];
                    html += `<td>${{val ? fmt(val) : '-'}}</td>`;
                }});
                html += '</tr>';
            }});
            html += '</table>';
            document.getElementById('table').innerHTML = html;
        }}

        const compileIops = DATA.length > 0
            ? Object.keys(DATA[DATA.length - 1].compile ?? {{}})
            : [];

        // Create a compile-time chart for each IOP; commits without compile data show as gaps
        const compileChartsDiv = document.getElementById('compile-charts');
        compileIops.forEach(iop => {{
            const container = document.createElement('div');
            container.className = 'chart-container';
            container.innerHTML = `<canvas id="compile-chart-${{iop}}"></canvas>`;
            compileChartsDiv.appendChild(container);

            const ctx = document.getElementById(`compile-chart-${{iop}}`).getContext('2d');
            const datasets = BITS.map((bits, i) => ({{
                label: `${{bits}}b`,
                data: DATA.map(r => r.compile?.[iop]?.[bits] ?? null),
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
                        title: {{ display: true, text: iop, color: '#00d4ff' }},
                        legend: {{ labels: {{ color: '#eee' }} }},
                    }},
                    scales: {{
                        x: {{ ticks: {{ color: '#aaa' }}, grid: {{ color: '#333' }} }},
                        y: {{
                            type: 'logarithmic',
                            ticks: {{ color: '#aaa' }},
                            grid: {{ color: '#333' }},
                            title: {{ display: true, text: 'Compile time (μs)', color: '#aaa' }}
                        }},
                    }},
                }},
            }});
        }});

        // Generate table with latest compile totals
        if (compileIops.length > 0) {{
            const latest = DATA[DATA.length - 1];
            let html = '<table><tr><th>Operation</th>';
            BITS.forEach(b => html += `<th>${{b}}b</th>`);
            html += '</tr>';
            compileIops.forEach(iop => {{
                html += `<tr><td style="text-align:left">${{iop}}</td>`;
                BITS.forEach(b => {{
                    const val = latest.compile?.[iop]?.[b];
                    html += `<td>${{val != null ? fmt(val / 1000) : '-'}}</td>`;
                }});
                html += '</tr>';
            }});
            html += '</table>';
            document.getElementById('compile-table').innerHTML = html;
        }}
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
            let result = run_benchmarks(filters.reps);
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
        "compile-diff" => {
            let use_color = !remaining.iter().any(|a| a == "--no-color");
            let rev_arg = remaining.iter().skip(1).find(|a| !a.starts_with("--"));
            let baseline = resolve_baseline(rev_arg);
            run_compile_diff(&baseline, use_color, &filters);
        }
        _ => {
            eprintln!("Usage: zhc_bench [run|export|diff|compile|compile-diff|analyze] [OPTIONS]");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  run                     - Run benchmarks and display latency table");
            eprintln!("  analyze                 - Run custom IR analysis (edit analyze_ir fn)");
            eprintln!(
                "  export                  - Run benchmarks (latency + compile times), save results, and regenerate site"
            );
            eprintln!(
                "  diff [REV] [--no-color] - Compare latencies against REV (default: latest baseline)"
            );
            eprintln!("  compile                 - Measure compiler wall-clock times");
            eprintln!(
                "  compile-diff [REV] [--no-color] - Compare compile times against REV (default: latest baseline)"
            );
            eprintln!();
            eprintln!("Filter options (for run, diff, compile, and compile-diff):");
            eprintln!(
                "  -i, --iops=PATTERNS     - Comma-separated iop name patterns (case-insensitive substring match)"
            );
            eprintln!("  -b, --bits=VALUES       - Comma-separated bit widths (8,16,32,64,128)");
            eprintln!(
                "  -r, --reps=N            - Compile-time repetitions, median kept (default: {})",
                DEFAULT_REPS
            );
            eprintln!();
            eprintln!("Examples:");
            eprintln!("  zhc_bench run -i mul,div -b 8,16");
            eprintln!("  zhc_bench diff --iops=cmp --bits=64");
            eprintln!("  zhc_bench compile -i mul -b 8,16");
            eprintln!("  zhc_bench compile-diff HEAD~1 -b 64");
        }
    }

    let reports = PANIC_REPORTS.lock().unwrap();
    if !reports.is_empty() {
        for report in reports.iter() {
            eprintln!("\n{}", report);
        }
        eprintln!("\nError: {} run(s) panicked.", reports.len());
        std::process::exit(1);
    }
}
