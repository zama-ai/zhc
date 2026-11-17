use std::env;
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

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = manifest_dir.join("vendor");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor");

    let lean_include = PathBuf::from(env::var("DEP_LEANRT_INCLUDE").unwrap());
    let runtime_version = env::var("DEP_LEANRT_VERSION").unwrap();
    let version = toolchain_version(&vendor_dir);
    assert!(
        runtime_version == version,
        "\n\n\
         Lean version mismatch: the vendored Init comes from Lean {version}, \
         but zhc_leanrt_sys builds the runtime of Lean {runtime_version}.\n"
    );

    let mut files = Vec::new();
    c_files(&vendor_dir, &mut files);
    files.sort();
    cc::Build::new()
        .include(&lean_include)
        .flag("-ffp-contract=off")
        .flag("-fvisibility=hidden")
        .warnings(false)
        .flag("-w")
        .files(&files)
        .compile("leaninit");

    println!("cargo:version={version}");
}
