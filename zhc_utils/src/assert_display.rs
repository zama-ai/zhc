//! Runtime support for `assert_display_is!` macro.

use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A recorded expectation mismatch.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectUpdate {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub actual: String,
}

impl ExpectUpdate {
    /// Load all pending updates from target/expect_updates/.
    pub fn load_all(target_dir: &PathBuf) -> Vec<Self> {
        let dir = target_dir.join("expect_updates");
        if !dir.exists() {
            return Vec::new();
        }

        fs::read_dir(&dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            .filter_map(|e| {
                let content = fs::read_to_string(e.path()).ok()?;
                serde_json::from_str(&content).ok()
            })
            .collect()
    }

    /// Clear all pending updates.
    pub fn clear_all(target_dir: &PathBuf) {
        let dir = target_dir.join("expect_updates");
        if dir.exists() {
            fs::remove_dir_all(&dir).ok();
        }
    }
}

/// Check actual against expected. On mismatch, record update and panic.
pub fn check(actual: &str, expected: &str, file: &str, line: u32, column: u32, manifest_dir: &str) {
    let actual_norm = normalize(actual);
    let expected_norm = normalize(expected);

    if actual_norm == expected_norm {
        return;
    }

    // Record the mismatch for later update
    record_mismatch(file, line, column, actual, expected, manifest_dir);

    panic!(
        "assert_display_is! mismatch at {}:{}:{}\n\n--- Expected ---\n{}\n\n--- Actual ---\n{}\n\nRun `cargo run --bin update-expects` to apply fixes.",
        file, line, column, expected, actual
    );
}

/// Normalize: trim each line, join, trim overall.
fn normalize(s: &str) -> String {
    s.lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Record a mismatch to target/expect_updates/.
fn record_mismatch(
    file: &str,
    line: u32,
    column: u32,
    actual: &str,
    expected: &str,
    manifest_dir: &str,
) {
    let workspace_root = find_workspace_root(manifest_dir);
    let dir = workspace_root.join("target/expect_updates");
    fs::create_dir_all(&dir).expect("Failed to create expect_updates directory");

    // Use a unique filename based on file path and hash of expected content.
    // This ensures that re-running tests after modifying the file (changing line numbers)
    // overwrites the previous update file rather than creating a new one.
    let mut hasher = DefaultHasher::new();
    expected.hash(&mut hasher);
    let hash = hasher.finish();

    let safe_name = file.replace(['/', '\\'], "_");
    let filename = format!("{}_{:016x}.json", safe_name, hash);
    let path = dir.join(filename);

    // file!() returns workspace-relative path, make it absolute
    let abs_file = workspace_root.join(file);
    let update = ExpectUpdate {
        file: abs_file.to_string_lossy().to_string(),
        line,
        column,
        actual: actual.to_string(),
    };

    let json = serde_json::to_string_pretty(&update).expect("Failed to serialize update");
    fs::write(&path, json).expect("Failed to write update file");
}

/// Find workspace root by walking up from manifest_dir.
fn find_workspace_root(manifest_dir: &str) -> PathBuf {
    let mut dir = PathBuf::from(manifest_dir);
    loop {
        if dir.join("Cargo.toml").exists() {
            let content = fs::read_to_string(dir.join("Cargo.toml")).unwrap_or_default();
            if content.contains("[workspace]") {
                return dir;
            }
        }
        if !dir.pop() {
            break;
        }
    }
    // Fallback: parent of manifest_dir
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf()
}
