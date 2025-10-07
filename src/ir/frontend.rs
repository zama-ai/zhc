//! Binding between Ir and rhai scripting engine

use super::args::*;
use super::builder::IrBuilderWrapped;
use super::operations::IrOperation;
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Scope};

/// Create an instance of the rhai scripting engine bind with an IrBuilder
pub fn create_rhai_engine() -> (Engine, IrBuilderWrapped) {
    let mut engine = Engine::new();
    let builder = IrBuilderWrapped::new();

    // Register type
    engine.register_type::<Register>();
    engine.register_type::<MemCell>();
    engine.register_type::<ImmCell>();

    // Helper function for print/debug support ================================
    engine.on_print(|x| println!("rhai info: {x}"));
    engine.on_debug(|x, src, pos| println!("rhai debug @{src:?}:{pos:?}: {x}"));

    // Helper function to create IR type ======================================
    engine.register_fn("input_vars", |slot: i64, width: i64| -> Vec<MemCell> {
        let digit = width / 2; // TODO let this configurable
        (0..digit)
            .map(|i| MemCell::User {
                kind: UserKind::Src,
                slot: slot as usize,
                digit: i as usize,
            })
            .collect::<Vec<_>>()
    });
    engine.register_fn("output_vars", |slot: i64, width: i64| -> Vec<MemCell> {
        let digit = width / 2; // TODO let this configurable
        (0..digit)
            .map(|i| MemCell::User {
                kind: UserKind::Dst,
                slot: slot as usize,
                digit: i as usize,
            })
            .collect::<Vec<_>>()
    });

    engine.register_fn("imm_cst", |val: i64| -> ImmCell {
        ImmCell::Cst(val as usize)
    });

    engine.register_fn("pbs_lut", |val: ImmutableString| -> PbsLut {
        PbsLut::new(val.as_str())
    });

    // Helper function to iterate over input/output vectors
    engine.register_indexer_get(
        |vec: &mut Vec<MemCell>, index: i64| -> Result<MemCell, Box<EvalAltResult>> {
            let idx = index as usize;
            vec.get(idx).cloned().ok_or_else(|| {
                format!(
                    "Index {} out of bounds for vec of length {}",
                    index,
                    vec.len()
                )
                .into()
            })
        },
    );

    // Register indexing set operation
    engine.register_indexer_set(
        |vec: &mut Vec<MemCell>, index: i64, value: MemCell| -> Result<(), Box<EvalAltResult>> {
            let idx = index as usize;
            if idx < vec.len() {
                vec[idx] = value;
                Ok(())
            } else {
                Err(format!(
                    "Index {} out of bounds for vec of length {}",
                    index,
                    vec.len()
                )
                .into())
            }
        },
    );

    // Memory related operations ==============================================
    // Register load operation
    let builder_clone = builder.clone();
    engine.register_fn("load", move |mem: MemCell| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Load {
            dst: dst.clone(),
            mem,
        };
        builder.push(op);
        dst
    });

    // Register store operation
    let builder_clone = builder.clone();
    engine.register_fn("store", move |src: Register, mem: MemCell| {
        let mut builder = builder_clone.lock().unwrap();
        let op = IrOperation::Store { mem, src };
        builder.push(op);
    });

    // Register sync operation
    let builder_clone = builder.clone();
    engine.register_fn("sync", move || {
        let mut builder = builder_clone.lock().unwrap();
        let op = IrOperation::Sync {};
        builder.push(op);
    });

    // Arith operations =======================================================
    // Register add operation
    let builder_clone = builder.clone();
    engine.register_fn("add", move |src_a: Register, src_b: Register| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Add {
            dst: dst.clone(),
            src_a,
            src_b,
        };
        builder.push(op);
        dst
    });

    // Register sub operation
    let builder_clone = builder.clone();
    engine.register_fn("sub", move |src_a: Register, src_b: Register| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Sub {
            dst: dst.clone(),
            src_a,
            src_b,
        };
        builder.push(op);
        dst
    });

    // Register mac operation
    let builder_clone = builder.clone();
    engine.register_fn(
        "mac",
        move |src_a: Register, src_b: Register, imm: ImmCell| -> Register {
            let mut builder = builder_clone.lock().unwrap();
            let dst = builder.ssa_register();
            let op = IrOperation::Mac {
                dst: dst.clone(),
                src_a,
                src_b,
                imm,
            };
            builder.push(op);
            dst
        },
    );

    // Scalar operations =====================================================
    // Register adds operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn("adds", move |src_a: Register, imm_b: ImmCell| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Adds {
            dst: dst.clone(),
            src_a,
            imm_b,
        };
        builder.push(op);
        dst
    });
    // Register subs operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn("subs", move |src_a: Register, imm_b: ImmCell| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Subs {
            dst: dst.clone(),
            src_a,
            imm_b,
        };
        builder.push(op);
        dst
    });
    // Register ssub operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn("ssub", move |imm_a: ImmCell, src_b: Register| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Ssub {
            dst: dst.clone(),
            imm_a,
            src_b,
        };
        builder.push(op);
        dst
    });
    // Register muls operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn("muls", move |src_a: Register, imm_b: ImmCell| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Muls {
            dst: dst.clone(),
            src_a,
            imm_b,
        };
        builder.push(op);
        dst
    });

    // Register Pbs operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "pbs_ml",
        move |src: Vec<Register>, lut: PbsLut| -> Vec<Register> {
            let mut builder = builder_clone.lock().unwrap();
            let dst = src
                .iter()
                .map(|_| builder.ssa_register())
                .collect::<Vec<_>>();
            let op = IrOperation::Pbs {
                dst: dst.clone(),
                src,
                lut,
                flush: false, // Flushing handled by compiler only
            };
            builder.push(op);
            dst
        },
    );
    // Register Variante for single PBS to reduce boilerplate code
    let builder_clone = builder.clone();
    engine.register_fn("pbs", move |src: Register, lut: PbsLut| -> Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = IrOperation::Pbs {
            dst: vec![dst.clone()],
            src: vec![src],
            lut,
            flush: false, // Flushing handled by compiler only
        };
        builder.push(op);
        dst
    });

    // *** OPERATOR OVERLOADING: Enable `a + b` syntax ***
    // TODO
    // let builder_clone = builder.clone();
    // engine.register_fn("+", move |src_a: Register, src_b: Register| -> Register {
    //     builder_clone.add(src_a, src_b)
    // });

    (engine, builder)
}
