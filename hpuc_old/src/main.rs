use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use clap::Parser;
pub use clap_num::maybe_hex;
use hpuc_frontend::{BuilderContext, create_rhai_engine};
use hpuc_gui::display;
use hpuc_ir::IR;
use hpuc_langs::ioplang;

/// Define CLI arguments
#[derive(clap::Parser, Debug, Clone)]
#[command(long_about = "HPU IOp compiler, construct IOp Dag based on user description")]
pub struct Args {
    // Builder context ========================================================
    /// integer width
    #[arg(long, default_value_t = 64)]
    pub integer_w: usize,

    /// message width
    #[arg(long, default_value_t = 2)]
    pub msg_w: usize,

    /// carry width
    #[arg(long, default_value_t = 2)]
    pub carry_w: usize,

    /// nu message
    /// Maximum computation alowed on full message ciphertext
    #[arg(long, default_value_t = 2)]
    pub nu_msg: usize,

    /// nu bool
    /// Maximum computation alowed on boolean ciphertext
    #[arg(long, default_value_t = 2)]
    pub nu_bool: usize,

    // Execution info ========================================================
    /// IOp algorithm described with Rhai
    #[arg(long, default_value = "iop/demo.rhai")]
    pub input: String,

    /// Output file
    /// Algo output as asm in SSA form
    #[arg(long)]
    pub out_ssa: Option<String>,

    /// Output file
    /// Algo output as dot file
    #[arg(long)]
    pub out_dot: Option<String>,

    /// View Dag in interactive windows
    #[arg(long)]
    pub view: bool,
}

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("User Options: {args:?}");

    let context = BuilderContext {
        integer_w: args.integer_w as i64,
        msg_w: args.msg_w as i64,
        carry_w: args.carry_w as i64,
        nu_msg: args.nu_msg as i64,
        nu_bool: args.nu_bool as i64,
    };
    let (engine, builder) = create_rhai_engine(context);

    // Execute user script to populate the builder
    engine.run_file(PathBuf::from(&args.input))?;

    // //Dump IR in a file
    // if let Some(out_ssa) = args.out_ssa.as_ref() {
    //     builder.lock().unwrap().write_to_file(out_ssa)?;
    // }

    // Exract inner IR and convert into graph
    drop(engine);
    let ir = Arc::new(IR::<ioplang::Ioplang>::try_from(builder).unwrap());
    let graph = ir.to_contextual_graph();
    println!("DAG stats:");
    println!("Nodes: {}", graph.node_count());
    println!("Edge: {}", graph.edge_count());

    // // Dump graph in a dot file
    // if let Some(out_dot) = args.out_dot.as_ref() {
    //     dag.write_dot_file(out_dot)?;
    // }

    // Create Graph gui if required
    if args.view {
        let name = Path::new(&args.input)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("HpuCompiler");
        display(&graph, name);
    }
    Ok(())
}
