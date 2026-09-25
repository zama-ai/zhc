use std::path::{Path, PathBuf};
use std::process::Command;

fn run(cmd: &mut Command) -> String {
    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("failed to run {cmd:?}: {e}"));
    if !out.status.success() {
        panic!(
            "{cmd:?} failed\n--- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    String::from_utf8(out.stdout).unwrap()
}

fn lake(lean_dir: &Path) -> Command {
    let mut cmd = Command::new(std::env::var("LAKE").unwrap_or_else(|_| "lake".into()));
    cmd.current_dir(lean_dir).env_remove("RUSTC_WORKSPACE_WRAPPER");
    cmd
}

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let lean_dir = crate_dir.join("zhc_lean");
    let c_api_include = crate_dir.join("../zhc_c_api/include");

    for path in [
        "zhc_lean/Zhc",
        "zhc_lean/Zhc.lean",
        "zhc_lean/ffi",
        "zhc_lean/lakefile.lean",
        "zhc_lean/lean-toolchain",
        "zhc_lean/lake-manifest.json",
        "csrc",
        "../zhc_c_api/include",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    println!("cargo:rerun-if-env-changed=LAKE");

    run(lake(&lean_dir).args(["build", "Zhc:static", "libzhc_lean_shim:static"]));

    let prefix = PathBuf::from(
        run(lake(&lean_dir).args(["env", "lean", "--print-prefix"]))
            .trim()
            .to_owned(),
    );

    cc::Build::new()
        .file(crate_dir.join("csrc/zhc_iops.c"))
        .include(prefix.join("include"))
        .include(&c_api_include)
        .warnings_into_errors(true)
        .compile("zhc_iops_stub");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    for lib in ["libzhc__lean_Zhc.a", "libzhc_lean_shim.a"] {
        std::fs::copy(lean_dir.join(".lake/build/lib").join(lib), out_dir.join(lib)).unwrap();
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!(
        "cargo:rustc-link-search=native={}",
        prefix.join("lib/lean").display()
    );
    println!("cargo:rustc-link-search=native={}", prefix.join("lib").display());

    for lib in [
        "zhc__lean_Zhc",
        "zhc_lean_shim",
        "Std",
        "Init",
        "leanrt",
        "leancpp",
        "uv",
        "gmp",
    ] {
        println!("cargo:rustc-link-lib=static:-bundle={lib}");
    }
    println!("cargo:rustc-link-lib=dylib=c++");
}
