//! Drives the C test programs in `tests/c/` and checks the header against the built library.
//!
//! Every `tests/c/*.c` file is a standalone program. It is compiled with the system C compiler
//! against `include/zhc.h`, linked with the static library produced by this crate, and run with
//! a scratch directory as its only argument. A non-zero exit status fails the test.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The directory holding the build artifacts of the current profile, e.g. `target/debug`.
fn artifacts_dir() -> PathBuf {
    // The test binary lives in `target/<profile>/deps/`.
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Builds the static library once per test run and returns its path.
///
/// `cargo test` only builds the test targets, not the `staticlib` crate type, so the tests
/// build it themselves with the same profile and target directory.
fn static_lib() -> PathBuf {
    static BUILD: std::sync::Once = std::sync::Once::new();
    let artifacts = artifacts_dir();
    BUILD.call_once(|| {
        let target_dir = artifacts.parent().unwrap();
        let profile = artifacts.file_name().unwrap().to_str().unwrap();
        let mut cargo = Command::new(env!("CARGO"));
        cargo
            .args(["build", "-p", "zhc_c_api", "--lib", "--target-dir"])
            .arg(target_dir);
        match profile {
            "debug" => {}
            "release" => {
                cargo.arg("--release");
            }
            other => {
                cargo.args(["--profile", other]);
            }
        }
        run(&mut cargo);
    });
    let lib = artifacts.join("libzhc_c_api.a");
    assert!(lib.exists(), "{} not found after build", lib.display());
    lib
}

fn header() -> PathBuf {
    manifest_dir().join("include").join("zhc.h")
}

fn scratch_dir(name: &str) -> PathBuf {
    let dir = artifacts_dir().join("zhc_c_api_tests").join(name);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn c_compiler() -> String {
    std::env::var("CC").unwrap_or_else(|_| "cc".to_string())
}

fn system_libs() -> &'static [&'static str] {
    if cfg!(target_os = "linux") {
        &["-lpthread", "-lm", "-ldl"]
    } else {
        &["-lpthread", "-lm"]
    }
}

fn run(cmd: &mut Command) -> String {
    let output = cmd
        .output()
        .unwrap_or_else(|e| panic!("cannot run {cmd:?}: {e}"));
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "{cmd:?} failed with {}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
        output.status
    );
    stdout
}

fn compile_and_run(source: &Path) {
    let name = source.file_stem().unwrap().to_str().unwrap();
    let scratch = scratch_dir(name);
    let exe = scratch.join(name);

    let mut cc = Command::new(c_compiler());
    cc.args(["-Wall", "-Wextra", "-Werror", "-std=c99"])
        .arg("-I")
        .arg(manifest_dir().join("include"))
        .arg("-o")
        .arg(&exe)
        .arg(source)
        .arg(static_lib())
        .args(system_libs());
    run(&mut cc);

    let stdout = run(Command::new(&exe).arg(&scratch));
    assert!(
        stdout.trim_end().ends_with("ok"),
        "{name} did not end with `ok`:\n{stdout}"
    );
}

#[test]
fn builder() {
    compile_and_run(&manifest_dir().join("tests/c/builder.c"));
}

#[test]
fn lut() {
    compile_and_run(&manifest_dir().join("tests/c/lut.c"));
}

#[test]
fn diagnostics() {
    compile_and_run(&manifest_dir().join("tests/c/diagnostics.c"));
}

#[test]
fn pipeline_inputs() {
    compile_and_run(&manifest_dir().join("tests/c/pipeline_inputs.c"));
}

#[test]
fn pipeline_artifacts() {
    compile_and_run(&manifest_dir().join("tests/c/pipeline_artifacts.c"));
}

/// Every `.c` file in `tests/c/` must have a `#[test]` above.
#[test]
fn every_c_program_has_a_test() {
    let declared: BTreeSet<&str> = [
        "builder",
        "lut",
        "diagnostics",
        "pipeline_inputs",
        "pipeline_artifacts",
    ]
    .into_iter()
    .collect();
    let on_disk: BTreeSet<String> = std::fs::read_dir(manifest_dir().join("tests/c"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "c"))
        .map(|p| p.file_stem().unwrap().to_str().unwrap().to_string())
        .collect();
    let on_disk: BTreeSet<&str> = on_disk.iter().map(String::as_str).collect();
    assert_eq!(declared, on_disk, "tests/c/ and the #[test] list differ");
}

/// Names of the `zhc_*` functions declared in the header.
fn header_symbols() -> BTreeSet<String> {
    let text = std::fs::read_to_string(header()).unwrap();
    let bytes = text.as_bytes();
    let mut out = BTreeSet::new();
    let mut i = 0;
    while i < bytes.len() {
        let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
        if !is_ident(bytes[i]) || (i > 0 && is_ident(bytes[i - 1])) {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && is_ident(bytes[i]) {
            i += 1;
        }
        let ident = &text[start..i];
        if ident.starts_with("zhc_") && bytes.get(i) == Some(&b'(') {
            out.insert(ident.to_string());
        }
    }
    out
}

/// Names of the `zhc_*` functions exported by the static library.
fn library_symbols() -> BTreeSet<String> {
    let stdout = run(Command::new("nm").arg("-g").arg(static_lib()));
    stdout
        .lines()
        .filter_map(|line| {
            // `<addr> T <name>`; on macOS names carry a leading underscore.
            let mut parts = line.split_whitespace();
            let _addr = parts.next()?;
            let kind = parts.next()?;
            let name = parts.next()?;
            (kind == "T").then(|| name.trim_start_matches('_').to_string())
        })
        .filter(|name| name.starts_with("zhc_"))
        .collect()
}

#[test]
fn header_matches_library() {
    let header = header_symbols();
    let library = library_symbols();
    let only_in_header: Vec<_> = header.difference(&library).collect();
    let only_in_library: Vec<_> = library.difference(&header).collect();
    assert!(
        only_in_header.is_empty() && only_in_library.is_empty(),
        "declared but not exported: {only_in_header:?}\nexported but not declared: {only_in_library:?}"
    );
    assert!(!header.is_empty());
}
