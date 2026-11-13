//! Drives the C test programs in `tests/c/`.

use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn artifacts_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn static_lib() -> PathBuf {
    static BUILD: std::sync::Once = std::sync::Once::new();
    let artifacts = artifacts_dir();
    BUILD.call_once(|| {
        let target_dir = artifacts.parent().unwrap();
        let profile = artifacts.file_name().unwrap().to_str().unwrap();
        let mut cargo = Command::new(env!("CARGO"));
        cargo
            .args(["build", "-p", "zhc_iops_l2r", "--lib", "--target-dir"])
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
    let lib = artifacts.join("libzhc_iops_l2r.a");
    assert!(lib.exists(), "{} not found after build", lib.display());
    lib
}

fn scratch_dir(name: &str) -> PathBuf {
    let dir = artifacts_dir().join("zhc_iops_l2r_tests").join(name);
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
fn interpreter() {
    compile_and_run(&manifest_dir().join("tests/c/interpreter.c"));
}

#[test]
fn diagnostics() {
    compile_and_run(&manifest_dir().join("tests/c/diagnostics.c"));
}
