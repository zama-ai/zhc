//! Binding between Ir and rhai scripting engine

use crate::ir;
use rhai::{Array, Dynamic, Engine, EvalAltResult, ImmutableString, Scope};

/// Create an instance of the rhai scripting engine bind with an IrBuilder
pub fn create_rhai_engine() -> (Engine, ir::IrBuilderWrapped) {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(128, 64);
    let builder = ir::IrBuilderWrapped::new();

    // Register type
    engine.register_type::<ir::Register>();
    engine.register_type::<ir::MemCell>();
    engine.register_type::<ir::ImmCell>();

    // Helper function for print/debug support ================================
    engine.on_print(|x| println!("rhai info: {x}"));
    engine.on_debug(|x, src, pos| println!("rhai debug @{src:?}:{pos:?}: {x}"));

    // Helper function for boundaries computation =============================
    engine.register_fn("clog2", |x: i64| -> i64 { (x as f64).log2().ceil() as i64 });

    // Helper function to create IR type ======================================
    engine.register_fn("input_vars", |slot: i64, width: i64| -> Array {
        let digit = width / 2; // TODO let this configurable
        (0..digit)
            .map(|i| ir::MemCell::User {
                kind: ir::UserKind::Src,
                slot: slot as usize,
                digit: i as usize,
            })
            .map(|x| Dynamic::from(x))
            .collect()
    });
    engine.register_fn("output_vars", |slot: i64, width: i64| -> Array {
        let digit = {
            // Ceiling div
            (width + (2 - 1)) / 2
        };
        (0..digit)
            .map(|i| ir::MemCell::User {
                kind: ir::UserKind::Dst,
                slot: slot as usize,
                digit: i as usize,
            })
            .map(|x| Dynamic::from(x))
            .collect()
    });

    engine.register_fn("imm_cst", |val: i64| -> ir::ImmCell {
        ir::ImmCell::Cst(val as usize)
    });

    engine.register_fn(
        "pbs_lut",
        ir::PbsLut::from_rhai,
        // // |xfer: ImmutableString, deg: ImmutableString| -> ir::PbsLut {
        // //     ir::PbsLut::from_rhai(xfer.as_str(), deg.as_str())
        // },
    );

    // Use dynamic array type instead
    // // Helper function to iterate over input/output vectors
    // engine.register_indexer_get(
    //     |vec: &mut Vec<ir::MemCell>, index: i64| -> Result<ir::MemCell, Box<EvalAltResult>> {
    //         let idx = index as usize;
    //         vec.get(idx).cloned().ok_or_else(|| {
    //             format!(
    //                 "Index {} out of bounds for vec of length {}",
    //                 index,
    //                 vec.len()
    //             )
    //             .into()
    //         })
    //     },
    // );

    // // Register indexing set operation
    // engine.register_indexer_set(
    //     |vec: &mut Vec<ir::MemCell>,
    //      index: i64,
    //      value: ir::MemCell|
    //      -> Result<(), Box<EvalAltResult>> {
    //         let idx = index as usize;
    //         if idx < vec.len() {
    //             vec[idx] = value;
    //             Ok(())
    //         } else {
    //             Err(format!(
    //                 "Index {} out of bounds for vec of length {}",
    //                 index,
    //                 vec.len()
    //             )
    //             .into())
    //         }
    //     },
    // );

    // Helper function for common iteration pattern
    engine.register_fn("chunks_by", |arr: Array, chunk_size: i64| -> Array {
        arr.chunks(chunk_size as usize)
            .map(|chunk| Dynamic::from(chunk.to_vec()))
            .collect()
    });
    engine.register_fn("zip", |a: Array, b: Array| -> Array {
        a.into_iter()
            .zip(b.into_iter())
            .map(|(x, y)| {
                let pair: Array = vec![x, y];
                Dynamic::from(pair)
            })
            .collect()
    });

    engine.register_fn("enumerate", |arr: rhai::Array| -> rhai::Array {
        arr.into_iter()
            .enumerate()
            .map(|(i, v)| {
                let mut map = rhai::Map::new();
                map.insert("index".into(), (i as i64).into());
                map.insert("value".into(), v);
                Dynamic::from(map)
            })
            .collect()
    });
    // Memory related operations ==============================================
    // Register load operation
    let builder_clone = builder.clone();
    engine.register_fn("load", move |mem: ir::MemCell| -> ir::Register {
        let mut builder = builder_clone.lock().unwrap();
        let dst = builder.ssa_register();
        let op = ir::IrOperation::Load {
            dst: dst.clone(),
            mem,
        };
        builder.push(op);
        dst
    });

    // Register store operation
    let builder_clone = builder.clone();
    engine.register_fn("store", move |src: ir::Register, mem: ir::MemCell| {
        let mut builder = builder_clone.lock().unwrap();
        let op = ir::IrOperation::Store { mem, src };
        builder.push(op);
    });

    // Register sync operation
    let builder_clone = builder.clone();
    engine.register_fn("sync", move || {
        let mut builder = builder_clone.lock().unwrap();
        let op = ir::IrOperation::Sync {};
        builder.push(op);
    });

    // Arith operations =======================================================
    let builder_clone = builder.clone();
    engine.register_fn(
        "add",
        move |src_a: ir::Register, src_b: ir::Register| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.add(src_a, src_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "+",
        move |src_a: ir::Register, src_b: ir::Register| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.add(src_a, src_b)
        },
    );

    // Register sub operation
    let builder_clone = builder.clone();
    engine.register_fn(
        "sub",
        move |src_a: ir::Register, src_b: ir::Register| -> ir::Register {
            let mut builder = builder_clone.lock().unwrap();
            builder.sub(src_a, src_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "-",
        move |src_a: ir::Register, src_b: ir::Register| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.sub(src_a, src_b)
        },
    );

    // Register mac operation
    let builder_clone = builder.clone();
    engine.register_fn(
        "mac",
        move |src_a: ir::Register, src_b: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut builder = builder_clone.lock().unwrap();
            builder.mac(src_a, src_b, imm_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "+*",
        move |src_a: ir::Register, src_b: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.mac(src_a, src_b, imm_b)
        },
    );

    // Scalar operations =====================================================
    // Register adds operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "adds",
        move |src_a: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.adds(src_a, imm_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "+",
        move |src_a: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.adds(src_a, imm_b)
        },
    );
    // Register subs operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "subs",
        move |src_a: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.subs(src_a, imm_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "-",
        move |src_a: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.subs(src_a, imm_b)
        },
    );
    // Register ssub operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "ssub",
        move |imm_a: ir::ImmCell, src_b: ir::Register| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.ssub(imm_a, src_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "-",
        move |imm_a: ir::ImmCell, src_b: ir::Register| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.ssub(imm_a, src_b)
        },
    );
    // Register muls operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "muls",
        move |src_a: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.muls(src_a, imm_b)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "*",
        move |src_a: ir::Register, imm_b: ir::ImmCell| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock.muls(src_a, imm_b)
        },
    );

    // Register Pbs operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "pbs_ml",
        move |ml_len: i64, src: ir::Register, lut: ir::PbsLut| -> Array {
            let mut inner_lock = builder_clone.lock().unwrap();
            inner_lock
                .pbs_ml(ml_len as usize, src, lut)
                .into_iter()
                .map(|x| Dynamic::from(x))
                .collect()
        },
    );
    // Register Variante for single PBS to reduce boilerplate code
    let builder_clone = builder.clone();
    engine.register_fn(
        "pbs",
        move |src: ir::Register, lut: ir::PbsLut| -> ir::Register {
            let mut inner_lock = builder_clone.lock().unwrap();
            let mut dst = inner_lock.pbs_ml(1, src, lut);
            dst.pop().unwrap()
        },
    );
    (engine, builder)
}
