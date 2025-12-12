use super::builder::{BuilderContext, IopBuilder};
use hpuc_ir::{IRError, ValId};
use hpuc_langs::ioplang::{Ioplang, Litteral, LutGenerator, Operations, Types};
use hpuc_utils::svec;

use rhai::{Array, Dynamic, Engine, EvalAltResult, INT, ImmutableString};

/// Error conversion
/// From ir type to rhai one
trait ToRhaiiError {
    fn to_rhaii_error(self) -> Box<EvalAltResult>;
}

impl ToRhaiiError for IRError<Ioplang> {
    fn to_rhaii_error(self) -> Box<EvalAltResult> {
        format!("IRError {self}").into()
    }
}

/// Create an instance of the rhai scripting engine bind with an IrBuilder
pub fn create_rhai_engine(context: BuilderContext) -> (Engine, IopBuilder) {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(128, 64);

    let builder = IopBuilder::new(context);

    // Register types ==========================================================
    // Common ioplang type
    engine.register_type::<Types>();
    engine.register_type::<ValId>();

    // Builder context with associated getter
    engine.register_type::<BuilderContext>();
    engine.register_get("integer_w", |val: &mut BuilderContext| val.integer_w);
    engine.register_get("msg_w", |val: &mut BuilderContext| val.msg_w);
    engine.register_get("carry_w", |val: &mut BuilderContext| val.carry_w);
    engine.register_get("nu_msg", |val: &mut BuilderContext| val.nu_msg);
    engine.register_get("nu_bool", |val: &mut BuilderContext| val.nu_bool);
    engine.register_get("blk_nb", |val: &mut BuilderContext| val.blk_nb());

    // Helper function for print/debug support ================================
    engine.on_print(|x| println!("rhai info: {x}"));
    engine.on_debug(|x, src, pos| println!("rhai debug @{src:?}:{pos:?}: {x}"));

    // Helper function for context retrieval =================================
    let builder_clone = builder.clone();
    engine.register_fn("get_context", move || -> BuilderContext {
        builder_clone.context().clone()
    });

    // Helper function for boundaries computation =============================
    engine.register_fn("clog2", |x: i64| -> i64 { (x as f64).log2().ceil() as i64 });

    // Helper function for common iteration pattern ==========================
    engine.register_fn("chunks_by", |arr: Array, chunk_size: i64| -> Array {
        arr.chunks(chunk_size as usize)
            .map(|chunk| Dynamic::from(chunk.to_vec()))
            .collect()
    });
    engine.register_fn("zip", |a: Array, b: Array| -> Array {
        a.into_iter()
            .zip(b)
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

    // Range doesn't support map
    // Provide a function to cast Range into Array
    engine.register_fn("collect", |range: std::ops::Range<INT>| -> Array {
        range.map(Dynamic::from).collect()
    });

    // Helper function to create IR constants ======================================
    let builder_clone = builder.clone();
    engine.register_fn(
        "pt_cst",
        move |val: i64| -> Result<ValId, Box<EvalAltResult>> {
            let context = builder_clone.context();
            if val >= (1 << (context.msg_w + context.carry_w + 1)) {
                return Err(IRError::Range {
                    typ: Types::PlaintextBlock,
                }
                .to_rhaii_error());
            }
            let mut ir = builder_clone.ir();

            let (_, pt) = ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::PlaintextBlock(val as usize),
                    },
                    svec![],
                )
                .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(pt[0])
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "idx_cst",
        move |val: i64| -> Result<ValId, Box<EvalAltResult>> {
            let context = builder_clone.context();
            if val >= context.blk_nb() {
                return Err(IRError::Range { typ: Types::Index }.to_rhaii_error());
            }
            let mut ir = builder_clone.ir();

            let (_, pt) = ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(val as usize),
                    },
                    svec![],
                )
                .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(pt[0])
        },
    );

    let builder_clone = builder.clone();
    engine.register_fn(
        "pbs_lut",
        move |name: ImmutableString, deg: i64| -> Result<ValId, Box<EvalAltResult>> {
            let mut ir = builder_clone.ir();
            let (_, lut) = match deg {
                1 => ir.add_op(
                    Operations::GenerateLut {
                        name: name.to_string(),
                        gene: LutGenerator::identity(),
                    },
                    svec![],
                ),
                2 => ir.add_op(
                    Operations::GenerateLut2 {
                        name: name.to_string(),
                        gene: [LutGenerator::identity(), LutGenerator::identity()],
                    },
                    svec![],
                ),
                4 => ir.add_op(
                    Operations::GenerateLut4 {
                        name: name.to_string(),
                        gene: [
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                        ],
                    },
                    svec![],
                ),
                8 => ir.add_op(
                    Operations::GenerateLut8 {
                        name: name.to_string(),
                        gene: [
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                            LutGenerator::identity(),
                        ],
                    },
                    svec![],
                ),
                _ => panic!(),
            }
            .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(lut[0])
        },
    );

    let builder_clone = builder.clone();
    engine.register_fn("ct_var", move || -> Result<ValId, Box<EvalAltResult>> {
        let mut ir = builder_clone.ir();

        let (_, ct) = ir
            .add_op(
                Operations::Let {
                    typ: Types::Ciphertext,
                },
                svec![],
            )
            .map_err(ToRhaiiError::to_rhaii_error)?;
        Ok(ct[0])
    });

    // Input/Output related operations ==============================================
    let builder_clone = builder.clone();
    engine.register_fn(
        "input",
        move |slot: i64| -> Result<ValId, Box<EvalAltResult>> {
            let mut ir = builder_clone.ir();
            let (_, input) = ir
                .add_op(
                    Operations::Input {
                        pos: slot as usize,
                        typ: Types::Ciphertext,
                    },
                    svec![],
                )
                .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(input[0])
        },
    );

    let builder_clone = builder.clone();
    engine.register_fn(
        "load",
        move |input: ValId, pos: i64| -> Result<ValId, Box<EvalAltResult>> {
            let context = builder_clone.context();
            if pos >= context.blk_nb() {
                panic!("Required to load out-of-bound block");
            }

            let mut ir = builder_clone.ir();
            let (_, index) = ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(pos as usize),
                    },
                    svec![],
                )
                .map_err(ToRhaiiError::to_rhaii_error)?;
            let (_, block) = ir
                .add_op(Operations::ExtractCtBlock, svec![input, index[0]])
                .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(block[0])
        },
    );

    let builder_clone = builder.clone();
    engine.register_fn(
        "store",
        move |ct_blk: ValId, output: ValId, pos: i64| -> Result<ValId, Box<EvalAltResult>> {
            let context = builder_clone.context();
            if pos >= context.blk_nb() {
                panic!("Required to store out-of-bound block");
            }

            let mut ir = builder_clone.ir();
            let (_, index) = ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(pos as usize),
                    },
                    svec![],
                )
                .map_err(ToRhaiiError::to_rhaii_error)?;
            let (_, res) = ir
                .add_op(Operations::StoreCtBlock, svec![ct_blk, output, index[0]])
                .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(res[0])
        },
    );

    let builder_clone = builder.clone();
    engine.register_fn(
        "output",
        move |output: ValId, slot: i64| -> Result<(), Box<EvalAltResult>> {
            let mut ir = builder_clone.ir();
            let _ = ir
                .add_op(
                    Operations::Output {
                        pos: slot as usize,
                        typ: Types::Ciphertext,
                    },
                    svec![output],
                )
                .map_err(ToRhaiiError::to_rhaii_error)?;
            Ok(())
        },
    );

    // Arith operations =======================================================
    let builder_clone = builder.clone();
    engine.register_fn(
        "add",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .add(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "adds",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .adds(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "+",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .addx(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );

    // Register sub operation
    let builder_clone = builder.clone();
    engine.register_fn(
        "sub",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .sub(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "subs",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .subs(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "ssub",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .ssub(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "-",
        move |src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .subx(src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );

    // Register mac operation
    let builder_clone = builder.clone();
    engine.register_fn(
        "mac",
        move |src_a: ValId, src_b: ValId, cst_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .mac(src_a, src_b, cst_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );
    let builder_clone = builder.clone();
    engine.register_fn(
        "*+",
        move |cst_a: ValId, src_a: ValId, src_b: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .mac(cst_a, src_a, src_b)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );

    // Register Pbs operation (for explicit calls)
    let builder_clone = builder.clone();
    engine.register_fn(
        "pbs_ml",
        move |src: ValId, lut: ValId| -> Result<Array, Box<EvalAltResult>> {
            Ok(builder_clone
                .pbs_ml(src, lut)
                .map_err(ToRhaiiError::to_rhaii_error)?
                .into_iter()
                .map(Dynamic::from)
                .collect())
        },
    );
    // Register Variante for single PBS to reduce boilerplate code
    let builder_clone = builder.clone();
    engine.register_fn(
        "pbs",
        move |src: ValId, lut: ValId| -> Result<ValId, Box<EvalAltResult>> {
            builder_clone
                .pbs(src, lut)
                .map_err(ToRhaiiError::to_rhaii_error)
        },
    );

    (engine, builder)
}
