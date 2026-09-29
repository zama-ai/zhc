use std::collections::HashMap;
use std::fmt::Display;
use std::process::Command;

use serde_json::Value;
use zhc_ir::{
    Dialect, DialectInstructionSet, DialectTypeSystem, Format, FormatContext, IR, Signature, ValId,
};
use zhc_utils::svec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildTypeSystem {
    Crate,
}

impl Display for BuildTypeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildTypeSystem::Crate => write!(f, "Crate"),
        }
    }
}

impl DialectTypeSystem for BuildTypeSystem {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildInstructionSet {
    Compile { name: String, n_deps: usize },
}

impl Format for BuildInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _ctx: &FormatContext) -> std::fmt::Result {
        match self {
            BuildInstructionSet::Compile { name, .. } => write!(f, "compile<name: {name}>"),
        }
    }
}

impl DialectInstructionSet for BuildInstructionSet {
    type TypeSystem = BuildTypeSystem;

    fn get_signature(&self) -> Signature<Self::TypeSystem> {
        match self {
            BuildInstructionSet::Compile { n_deps, .. } => Signature(
                svec![BuildTypeSystem::Crate; *n_deps],
                svec![BuildTypeSystem::Crate],
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildLang;

impl Dialect for BuildLang {
    type TypeSystem = BuildTypeSystem;
    type InstructionSet = BuildInstructionSet;
}

fn local_dependency_tree() -> Vec<(String, Vec<String>)> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let manifest = concat!(env!("CARGO_MANIFEST_DIR"), "/../Cargo.toml");
    let output = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .args(["--manifest-path", manifest])
        .output()
        .expect("failed to run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Value = serde_json::from_slice(&output.stdout).expect("invalid metadata");

    let mut tree: Vec<(String, Vec<String>)> = metadata["packages"]
        .as_array()
        .expect("no packages")
        .iter()
        .filter(|package| package["name"].as_str().unwrap().starts_with("zhc"))
        .map(|package| {
            let name = package["name"].as_str().unwrap().to_string();
            let mut deps: Vec<String> = package["dependencies"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|dep| dep.get("path").is_some())
                .filter(|dep| dep["kind"].as_str() != Some("dev"))
                .filter(|dep| dep["name"].as_str().unwrap().starts_with("zhc"))
                .map(|dep| dep["name"].as_str().unwrap().to_string())
                .collect();
            deps.sort();
            deps.dedup();
            (name, deps)
        })
        .collect();
    tree.sort();
    tree
}

fn add_crate(
    name: &str,
    tree: &HashMap<String, Vec<String>>,
    ir: &mut IR<BuildLang>,
    built: &mut HashMap<String, ValId>,
    visiting: &mut Vec<String>,
) -> ValId {
    if let Some(val) = built.get(name) {
        return *val;
    }
    assert!(
        !visiting.iter().any(|n| n == name),
        "dependency cycle: {visiting:?} -> {name}"
    );
    visiting.push(name.to_string());
    let deps = tree.get(name).cloned().unwrap_or_default();
    let args = deps
        .iter()
        .map(|dep| add_crate(dep, tree, ir, built, visiting))
        .collect();
    let (_, rets) = ir.add_op(
        BuildInstructionSet::Compile {
            name: name.to_string(),
            n_deps: deps.len(),
        },
        args,
    );
    visiting.pop();
    built.insert(name.to_string(), rets[0]);
    rets[0]
}

fn main() {
    let tree = local_dependency_tree();

    println!("Local dependency tree:");
    for (name, deps) in tree.iter() {
        println!("  {name} -> [{}]", deps.join(", "));
    }

    let names: Vec<String> = tree.iter().map(|(name, _)| name.clone()).collect();
    let tree: HashMap<String, Vec<String>> = tree.into_iter().collect();
    let mut ir: IR<BuildLang> = IR::empty();
    let mut built = HashMap::new();
    for name in names.iter() {
        add_crate(name, &tree, &mut ir, &mut built, &mut Vec::new());
    }

    println!("\nBuild IR:");
    println!("{}", ir.format());

    let drawing = ir.draw_to_html(None);
    println!("\nDrawing: {}", drawing.as_ref().display());
    drawing.open().expect("failed to open the drawing");
}
