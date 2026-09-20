use std::path::{Path, PathBuf};

fn c_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            c_files(&path, files);
        } else if path.extension().is_some_and(|e| e == "c") {
            files.push(path);
        }
    }
}

fn toolchain_version(vendor_dir: &Path) -> String {
    let path = vendor_dir.join("lean-toolchain");
    let toolchain = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    toolchain
        .trim()
        .rsplit(':')
        .next()
        .unwrap()
        .trim_start_matches('v')
        .to_owned()
}

fn lean_build(lean_include: &Path, vendor_dir: &Path) -> cc::Build {
    let mut build = cc::Build::new();
    build
        .include(lean_include)
        .include(vendor_dir.join("include"))
        .flag("-ffp-contract=off")
        .flag("-fvisibility=hidden");
    build
}

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = crate_dir.join("vendor");
    println!("cargo:rerun-if-changed=vendor");

    let lean_include = PathBuf::from(std::env::var("DEP_LEANRT_INCLUDE").unwrap());
    let runtime_version = std::env::var("DEP_LEANRT_VERSION").unwrap();
    let expected = toolchain_version(&vendor_dir);
    assert!(
        runtime_version == expected,
        "\n\n\
         Lean version mismatch: the vendored C code comes from Lean {expected}, \
         but zhc_leanrt_sys builds the runtime of Lean {runtime_version}.\n"
    );

    lean_build(&lean_include, &vendor_dir)
        .file(vendor_dir.join("ffi/zhc_r2l.c"))
        .warnings_into_errors(true)
        .compile("zhc_r2l");

    let mut ir = Vec::new();
    c_files(&vendor_dir.join("ir"), &mut ir);
    ir.sort();
    lean_build(&lean_include, &vendor_dir)
        .files(&ir)
        .warnings(false)
        .flag("-w")
        .compile("zhc_lean_ir");

    lean_build(&lean_include, &vendor_dir)
        .file(vendor_dir.join("ffi/zhc_l2r.c"))
        .std("c11")
        .warnings_into_errors(true)
        .compile("zhc_l2r");
}
