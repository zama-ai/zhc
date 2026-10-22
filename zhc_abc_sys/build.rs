use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let abc_src = manifest_dir.join("abc");
    if !abc_src.join("Makefile").exists() {
        panic!(
            "ABC sources not found at {}. Run `git submodule update --init`.",
            abc_src.display()
        );
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("abc-build");
    std::fs::create_dir_all(&build_dir).unwrap();

    let jobs = env::var("NUM_JOBS").unwrap_or_else(|_| "4".to_string());
    let status = Command::new("make")
        .current_dir(&build_dir)
        .arg("-f")
        .arg(abc_src.join("Makefile"))
        .arg(format!("ABCSRC={}", abc_src.display()))
        .arg("CCACHE=")
        .arg("ABC_USE_NO_READLINE=1")
        .arg("ABC_USE_PIC=1")
        .arg("ABC_USE_STDINT_H=1")
        .arg("OPTFLAGS=-O2 -fno-common")
        .arg(format!("-j{jobs}"))
        .arg("libabc.a")
        .status()
        .expect("failed to run make for ABC");
    assert!(status.success(), "ABC build failed");

    cc::Build::new()
        .file(manifest_dir.join("csrc/zhc_abc.c"))
        .include(abc_src.join("src"))
        .define("ABC_USE_STDINT_H", "1")
        .define("ABC_USE_PTHREADS", None)
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-write-strings")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("zhc_abc_wrapper");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=abc");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=c++");
            println!("cargo:rustc-link-lib=dl");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=dl");
            println!("cargo:rustc-link-lib=rt");
        }
        _ => {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=csrc/zhc_abc.c");
    println!("cargo:rerun-if-changed=abc/Makefile");
    println!("cargo:rerun-if-changed=abc/src");
}
