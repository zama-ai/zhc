//! Applies recorded expect updates to source files.
//!
//! Run after test failures: `cargo run --bin update-expects`

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use zhc_utils::assert_display::ExpectUpdate;

fn main() {
    let target_dir = find_target_dir();
    let updates = ExpectUpdate::load_all(&target_dir);

    if updates.is_empty() {
        println!("No pending updates.");
        return;
    }

    // Group by file
    let mut by_file: HashMap<String, Vec<ExpectUpdate>> = HashMap::new();
    for update in updates {
        by_file.entry(update.file.clone()).or_default().push(update);
    }

    // Apply updates to each file
    for (file, mut updates) in by_file {
        // Sort by position descending (apply from end to start to preserve offsets)
        updates.sort_by(|a, b| (b.line, b.column).cmp(&(a.line, a.column)));

        let mut content = fs::read_to_string(&file).expect("Failed to read source file");

        for update in updates {
            content = apply_update(&content, &update);
        }

        fs::write(&file, content).expect("Failed to write source file");
        println!("Updated: {}", file);
    }

    ExpectUpdate::clear_all(&target_dir);
    println!("Done.");
}

/// Find the target directory from current working directory.
fn find_target_dir() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    for _ in 0..20 {
        if dir.join("Cargo.toml").exists() {
            return dir.join("target");
        }
        if !dir.pop() {
            break;
        }
    }

    PathBuf::from("target")
}

/// Apply a single update to the content.
fn apply_update(content: &str, update: &ExpectUpdate) -> String {
    // Find byte offset of (line, column)
    let mut offset = 0;
    for (i, line) in content.lines().enumerate() {
        if i + 1 == update.line as usize {
            offset += (update.column - 1) as usize;
            break;
        }
        offset += line.len() + 1; // +1 for newline
    }

    // From offset, find r#" start
    let rest = &content[offset..];
    let raw_start = rest.find("r#\"").expect("Could not find r#\" in source");

    // Find matching "#
    let after_open = &rest[raw_start + 3..];
    let raw_end = after_open.find("\"#").expect("Could not find closing \"#");

    let global_start = offset + raw_start + 3; // after r#"
    let global_end = offset + raw_start + 3 + raw_end; // before "#

    // Compute indentation from the line where r#" appears
    let before_raw = &content[..offset + raw_start];
    let last_nl = before_raw.rfind('\n').map_or(0, |p| p + 1);
    let line_content = &content[last_nl..offset + raw_start];
    let base_indent = line_content.len() - line_content.trim_start().len() + 4;

    // Format the new value with indentation
    let indent = " ".repeat(base_indent);
    let mut new_val = String::from("\n");
    for line in update.actual.lines() {
        new_val.push_str(&indent);
        new_val.push_str(line);
        new_val.push('\n');
    }
    new_val.push_str(&" ".repeat(base_indent - 4));

    format!(
        "{}{}{}",
        &content[..global_start],
        new_val,
        &content[global_end..]
    )
}
