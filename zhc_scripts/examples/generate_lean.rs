use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

type FromPath = PathBuf;
type ToPath = PathBuf;
type Associations = Vec<(FromPath, ToPath)>;

const HEADER: &str = "zhc_iops_l2r/include/zhc.h";
const EXPORTS: &str = "zhc_iops_r2l/src/exports.rs";
const R2L_VENDOR: &str = "zhc_iops_r2l/vendor";
const LEANRT_VENDOR: &str = "zhc_leanrt_sys/vendor";
const LEANINIT_VENDOR: &str = "zhc_leaninit_sys/vendor";
const VENDORS: [&str; 3] = [R2L_VENDOR, LEANRT_VENDOR, LEANINIT_VENDOR];
const LEAN_SRC: &str = "zhc_lean";
const LEAN_REPO: &str = "https://github.com/leanprover/lean4";
const RUNTIME_UTIL_HEADERS: [&str; 3] = ["alloc.h", "io.h", "macros.h"];
const ROOT_MODULE: &str = "Zhc.Ffi.R2l.Export";

fn workspace_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}

fn lean_dir() -> PathBuf {
    workspace_dir().join(LEAN_SRC)
}

fn staging_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zhc_generate_lean_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

fn lake() -> Command {
    let lake = std::env::var("LAKE").unwrap_or_else(|_| "lake".into());
    let mut cmd = Command::new(lake);
    cmd.current_dir(lean_dir())
        .env_remove("RUSTC_WORKSPACE_WRAPPER");
    cmd
}

fn run(cmd: &mut Command) -> String {
    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("failed to run {cmd:?}: {e}"));
    assert!(
        out.status.success(),
        "{cmd:?} failed\n--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn lean_version() -> String {
    read(&lean_dir().join("lean-toolchain"))
        .trim()
        .rsplit(':')
        .next()
        .unwrap()
        .trim_start_matches('v')
        .to_owned()
}

fn check_lean_version() {
    let expected = lean_version();
    let stdout = run(lake().arg("--version"));
    let actual = stdout
        .split("Lean version ")
        .nth(1)
        .and_then(|s| s.split(')').next())
        .map(str::trim)
        .unwrap_or_else(|| panic!("could not parse the output of `lake --version`:\n{stdout}"));
    println!(
        "Detected Lean {actual} (expected {expected}): {}",
        stdout.trim()
    );
    assert!(
        actual == expected,
        "Lean version mismatch: {} pins Lean {expected}, but `lake` uses Lean {actual}. \
         Use `lake` from elan, or set LAKE to a `lake` binary from Lean {expected}.",
        lean_dir().join("lean-toolchain").display()
    );
}

fn lake_build() {
    check_lean_version();
    run(lake().args(["build", ROOT_MODULE]));
}

fn imports(source: &str) -> Vec<String> {
    let line = source
        .lines()
        .find_map(|l| l.strip_prefix("// Imports:"))
        .expect("no `// Imports:` line in the Lean compiler output");
    let mut words = line.split_whitespace();
    let mut modules = Vec::new();
    while let Some(word) = words.next() {
        if word == "import" {
            let module = match words.next() {
                Some("all") => words.next(),
                module => module,
            };
            modules.push(module.expect("truncated `// Imports:` line").to_owned());
        }
    }
    modules
}

fn is_init(module: &str) -> bool {
    module == "Init" || module.starts_with("Init.")
}

fn module_c_path(module: &str) -> PathBuf {
    PathBuf::from(format!("{}.c", module.replace('.', "/")))
}

fn lean_prefix() -> PathBuf {
    PathBuf::from(run(lake().args(["env", "lean", "--print-prefix"])).trim())
}

fn fetch_lean_sources(staging: &Path) -> PathBuf {
    println!("Fetch lean sources from {LEAN_REPO} ...");
    let clone = staging.join("lean4");
    run(Command::new("git")
        .args([
            "clone",
            "--quiet",
            "--depth",
            "1",
            "--filter=blob:none",
            "--sparse",
            "--branch",
            &format!("v{}", lean_version()),
            LEAN_REPO,
        ])
        .arg(&clone));
    run(Command::new("git").arg("-C").arg(&clone).args([
        "sparse-checkout",
        "set",
        "src/runtime",
        "src/include",
        "src/util",
    ]));
    let commit = run(Command::new("git")
        .arg("-C")
        .arg(&clone)
        .args(["rev-parse", "HEAD"]));
    println!("Checked out lean sources at commit {}", commit.trim());
    clone
}

fn copy_lean_runtime(lean_sources: &Path) -> Associations {
    let mut files = BTreeSet::new();
    tree_files(lean_sources, Path::new("src/runtime"), &mut files);
    tree_files(lean_sources, Path::new("src/include/lean"), &mut files);
    files.extend(
        RUNTIME_UTIL_HEADERS
            .iter()
            .map(|h| Path::new("src/util").join(h)),
    );
    files.insert(PathBuf::from("LICENSE"));
    let vendor_dir = PathBuf::from(LEANRT_VENDOR);
    let mut associations: Associations = files
        .into_iter()
        .map(|rel| (lean_sources.join(&rel), vendor_dir.join(rel)))
        .collect();
    associations.push((
        lean_dir().join("lean-toolchain"),
        vendor_dir.join("lean-toolchain"),
    ));
    associations
}

fn init_modules(src_dir: &Path, dir: &Path, modules: &mut Vec<String>) {
    let entries = std::fs::read_dir(src_dir.join(dir))
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", src_dir.join(dir).display()));
    for entry in entries {
        let rel = dir.join(entry.unwrap().file_name());
        if src_dir.join(&rel).is_dir() {
            init_modules(src_dir, &rel, modules);
        } else if rel.extension().is_some_and(|e| e == "lean") {
            let module = rel.with_extension("").to_str().unwrap().replace('/', ".");
            modules.push(module);
        }
    }
}

fn transpile_init_module(prefix: &Path, staging: &Path, module: &str) -> (FromPath, Vec<String>) {
    let src_dir = prefix.join("src/lean");
    let rel = module_c_path(module);
    let from = staging.join("init").join(&rel);
    std::fs::create_dir_all(from.parent().unwrap()).unwrap();
    run(Command::new(prefix.join("bin/lean"))
        .current_dir(lean_dir())
        .arg("-R")
        .arg(&src_dir)
        .arg("-c")
        .arg(&from)
        .arg(src_dir.join(rel.with_extension("lean"))));
    let imports = imports(&read(&from));
    (from, imports)
}

fn transpile_lean_init_to_c(staging: &Path, lean_sources: &Path) -> (Associations, Vec<String>) {
    println!("Transpile lean Init to C with lean ...");
    check_lean_version();
    let prefix = lean_prefix();
    let mut modules = vec!["Init".to_owned()];
    init_modules(&prefix.join("src/lean"), Path::new("Init"), &mut modules);
    modules.sort();
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                while let Some(module) = modules.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let result = transpile_init_module(&prefix, staging, module);
                    results.lock().unwrap().push((module.clone(), result));
                }
            });
        }
    });
    let vendor_dir = PathBuf::from(LEANINIT_VENDOR);
    let mut associations = Associations::new();
    let mut imported = Vec::new();
    for (module, (from, imports)) in results.into_inner().unwrap() {
        associations.push((from, vendor_dir.join(module_c_path(&module))));
        imported.extend(imports);
    }
    for module in &imported {
        assert!(
            is_init(module) && modules.contains(module),
            "an `Init` module imports {module}, which is not an `Init` source of the toolchain."
        );
    }
    associations.push((lean_sources.join("LICENSE"), vendor_dir.join("LICENSE")));
    associations.push((
        lean_dir().join("lean-toolchain"),
        vendor_dir.join("lean-toolchain"),
    ));
    (associations, modules)
}

fn transpile_lean_iops_to_c(lean_sources: &Path, init_modules: &[String]) -> Associations {
    println!("Transpile lean sources to C with lake ...");
    lake_build();
    let ir_dir = lean_dir().join(".lake/build/ir");
    let vendor_dir = PathBuf::from(R2L_VENDOR);
    let mut associations = Associations::new();
    let mut seen = BTreeSet::new();
    let mut todo = vec![ROOT_MODULE.to_owned()];
    while let Some(module) = todo.pop() {
        if !seen.insert(module.clone()) {
            continue;
        }
        if is_init(&module) {
            assert!(
                init_modules.contains(&module),
                "{ROOT_MODULE} imports {module}, which is not an `Init` source of the toolchain."
            );
            continue;
        }
        assert!(
            module == "Zhc" || module.starts_with("Zhc."),
            "{ROOT_MODULE} imports {module}. Only `Init` is vendored from the Lean toolchain, \
             so the exported code must not import other Lean libraries."
        );
        let rel = module_c_path(&module);
        let from = ir_dir.join(&rel);
        todo.extend(imports(&read(&from)));
        associations.push((from, vendor_dir.join("ir").join(rel)));
    }
    for rel in ["lean-toolchain", "ffi/zhc_l2r.c", "ffi/zhc_r2l.c"] {
        associations.push((lean_dir().join(rel), vendor_dir.join(rel)));
    }
    associations.push((lean_sources.join("LICENSE"), vendor_dir.join("LICENSE")));
    associations
}

fn generate_r2l_rust_exports_bindings(staging: &Path) -> Associations {
    println!("Generate r2l rust exports bindings with lean ...");
    lake_build();
    let from = staging.join("exports.rs");
    run(lake()
        .args([
            "env",
            "lean",
            "--run",
            "Zhc/Ffi/R2l/GenerateRustBinding.lean",
            ROOT_MODULE,
        ])
        .arg(&from));
    vec![(from, PathBuf::from(EXPORTS))]
}

fn generate_l2r_zhc_header(staging: &Path) -> Associations {
    println!("Generate l2r zhc header with cbindgen ...");
    let crate_dir = workspace_dir().join("zhc_iops_l2r");
    let config = cbindgen::Config::from_file(crate_dir.join("cbindgen.toml")).unwrap();
    let from = staging.join("zhc.h");
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file(&from);
    vec![
        (from.clone(), PathBuf::from(HEADER)),
        (from, PathBuf::from(R2L_VENDOR).join("include/zhc.h")),
    ]
}

fn generate(staging: &Path) -> Associations {
    let lean_sources = fetch_lean_sources(staging);
    let mut associations = copy_lean_runtime(&lean_sources);
    let (init, init_modules) = transpile_lean_init_to_c(staging, &lean_sources);
    associations.extend(init);
    associations.extend(transpile_lean_iops_to_c(&lean_sources, &init_modules));
    associations.extend(generate_r2l_rust_exports_bindings(staging));
    associations.extend(generate_l2r_zhc_header(staging));
    associations
}

fn tree_files(root: &Path, dir: &Path, files: &mut BTreeSet<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root.join(dir)) else {
        return;
    };
    for entry in entries {
        let rel = dir.join(entry.unwrap().file_name());
        if root.join(&rel).is_dir() {
            tree_files(root, &rel, files);
        } else {
            files.insert(rel);
        }
    }
}

fn stale(associations: &Associations) -> Vec<ToPath> {
    let root = workspace_dir();
    let targets: BTreeSet<&ToPath> = associations.iter().map(|(_, to)| to).collect();
    let mut vendored = BTreeSet::new();
    for vendor in VENDORS {
        tree_files(&root, Path::new(vendor), &mut vendored);
    }
    let extra = vendored.into_iter().filter(|to| !targets.contains(to));
    let changed = associations
        .iter()
        .filter(|(from, to)| std::fs::read_to_string(root.join(to)).ok() != Some(read(from)))
        .map(|(_, to)| to.clone());
    changed
        .chain(extra)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn update(associations: &Associations) {
    let root = workspace_dir();
    for vendor in VENDORS {
        let vendor_dir = root.join(vendor);
        if vendor_dir.exists() {
            std::fs::remove_dir_all(&vendor_dir).unwrap();
        }
    }
    for (from, to) in associations {
        let to = root.join(to);
        std::fs::create_dir_all(to.parent().unwrap()).unwrap();
        std::fs::copy(from, &to).unwrap_or_else(|e| {
            panic!("failed to copy {} to {}: {e}", from.display(), to.display())
        });
    }
}

fn main() {
    let staging = staging_dir();
    let associations = generate(&staging);
    let stale = stale(&associations);
    update(&associations);
    std::fs::remove_dir_all(&staging).unwrap();
    for path in &stale {
        println!("updated {}", path.display());
    }
    println!(
        "{} files generated, {} updated",
        associations.len(),
        stale.len()
    );
}

#[test]
#[ignore]
fn generated_files_are_up_to_date() {
    let staging = staging_dir();
    let stale = stale(&generate(&staging));
    std::fs::remove_dir_all(&staging).unwrap();
    assert!(
        stale.is_empty(),
        "These generated files are out of date: {stale:?}. \
         Run `cargo run -p zhc_scripts --example generate_lean` to regenerate them."
    );
}
