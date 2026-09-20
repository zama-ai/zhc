use std::env;
use std::path::{Path, PathBuf};

const EXCLUDED: &[&str] = &["openssl.cpp"];

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

fn toolchain_version(vendor_dir: &Path) -> (String, String, String) {
    let toolchain = read(&vendor_dir.join("lean-toolchain"));
    let version = toolchain
        .trim()
        .rsplit(':')
        .next()
        .unwrap()
        .trim_start_matches('v');
    let mut parts = version.split('.').map(str::to_owned);
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), Some(patch), None) => (major, minor, patch),
        _ => panic!("the Lean toolchain {version} is not a release version"),
    }
}

fn runtime_sources(src: &Path) -> Vec<PathBuf> {
    let cmake = read(&src.join("runtime/CMakeLists.txt"));
    let block = cmake
        .split_once("RUNTIME_OBJS")
        .and_then(|(_, rest)| rest.split_once(')'))
        .expect("no `RUNTIME_OBJS` list in src/runtime/CMakeLists.txt")
        .0;
    block
        .split_whitespace()
        .filter(|f| f.ends_with(".cpp") && !EXCLUDED.contains(f))
        .map(|f| src.join("runtime").join(f))
        .collect()
}

fn write_headers(vendor_dir: &Path, include: &Path) -> String {
    let lean_include = include.join("lean");
    std::fs::create_dir_all(&lean_include).unwrap();
    for entry in std::fs::read_dir(vendor_dir.join("src/include/lean")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "h") {
            std::fs::copy(&path, lean_include.join(path.file_name().unwrap())).unwrap();
        }
    }

    let (major, minor, patch) = toolchain_version(vendor_dir);
    let version = format!("{major}.{minor}.{patch}");
    let target = env::var("TARGET").unwrap();
    std::fs::write(
        lean_include.join("version.h"),
        format!(
            "#pragma once\n\
             #define LEAN_VERSION_MAJOR {major}\n\
             #define LEAN_VERSION_MINOR {minor}\n\
             #define LEAN_VERSION_PATCH {patch}\n\
             #define LEAN_VERSION_IS_RELEASE 1\n\
             #define LEAN_SPECIAL_VERSION_DESC \"\"\n\
             #define LEAN_VERSION_STRING \"{version}\"\n\
             #define LEAN_PLATFORM_TARGET \"{target}\"\n\
             #define LEAN_MANUAL_ROOT \"https://lean-lang.org/doc/reference/{version}/\"\n"
        ),
    )
    .unwrap();
    std::fs::write(include.join("githash.h"), "#define LEAN_GITHASH \"\"\n").unwrap();
    std::fs::write(
        lean_include.join("config.h"),
        "#pragma once\n#include <lean/version.h>\n\n#define LEAN_IS_STAGE0 0\n",
    )
    .unwrap();
    version
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = manifest_dir.join("vendor");
    let src = vendor_dir.join("src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let include = out_dir.join("include");
    let version = write_headers(&vendor_dir, &include);

    let uv = pkg_config::Config::new()
        .probe("libuv")
        .unwrap_or_else(|e| {
            panic!(
                "\n\nzhc_leanrt_sys links libuv, but pkg-config could not find it: {e}\n\n\
             Install libuv: `brew install libuv` or `apt install libuv1-dev`.\n"
            )
        });

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .include(&include)
        .include(&src)
        .includes(&uv.include_paths)
        .define("NDEBUG", None)
        .define("LEAN_MULTI_THREAD", None)
        .define("LEAN_EXPORTING", None)
        .define("LEAN_BUILD_TYPE", "\"Release\"")
        .flag("-ffp-contract=off")
        .flag_if_supported("-ftls-model=local-exec")
        .opt_level(3)
        .warnings(false)
        .flag("-w")
        .files(runtime_sources(&src))
        .compile("leanrt");

    println!("cargo:include={}", include.display());
    println!("cargo:version={version}");
}
