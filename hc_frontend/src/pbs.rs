//! Provide a way to define new PbsLut from Rhai
//! PbsLut is defined on rest side through a macro whihich is impossible to bridge on the rhai side.
//! Try to provide a function with a syntax as much close as possible to rust macro

use std::sync::Arc;

use rhai::Engine;

use hc_ir::PbsLut;

impl PbsLut {
    /// Helper function to strip "|x|" or "|x: u8|" prefix from closure-like strings
    fn strip_closure_prefix(script: &str) -> &str {
        let trimmed = script.trim();

        // Look for patterns like "|x|", "|x: u8|", etc.
        if trimmed.starts_with('|') {
            // Find the closing |
            if let Some(close_pos) = trimmed[1..].find('|') {
                trimmed[close_pos + 2..].trim_start()
            } else {
                trimmed
            }
        } else {
            trimmed
        }
    }

    /// Create PbsLut from Rhai script strings (with |x| prefix)
    pub fn from_rhai(xfer: &str, deg: &str) -> Result<Self, Box<rhai::EvalAltResult>> {
        let engine = Engine::new();

        // Strip the "|x|" prefix if present, keeping it for display
        let xfer = Self::strip_closure_prefix(xfer);
        let deg = Self::strip_closure_prefix(deg);

        // Compile the scripts once
        let xfer_ast = engine.compile(xfer)?;
        let deg_ast = engine.compile(deg)?;

        // Create closures that evaluate the Rhai scripts
        let xfer_fn = {
            let engine = Engine::new();
            let ast = xfer_ast.clone();
            move |x: u8| -> Vec<u8> {
                let mut scope = rhai::Scope::new();
                scope.push("x", x as i64);

                match engine.eval_ast_with_scope::<rhai::Array>(&mut scope, &ast) {
                    Ok(array) => array
                        .iter()
                        .filter_map(|v| v.as_int().ok())
                        .map(|i| i as u8)
                        .collect(),
                    Err(_) => vec![],
                }
            }
        };

        let deg_fn = {
            let engine = Engine::new();
            let ast = deg_ast.clone();
            move |x: u8| -> u8 {
                let mut scope = rhai::Scope::new();
                scope.push("x", x as i64);

                engine
                    .eval_ast_with_scope::<i64>(&mut scope, &ast)
                    .unwrap_or(0) as u8
            }
        };

        Ok(Self::new_raw(
            Arc::new(xfer_fn),
            Arc::new(deg_fn),
            xfer.to_string(),
            deg.to_string(),
        ))
    }
}
