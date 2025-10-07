use std::path::PathBuf;

/// Define CLI arguments
pub use clap::Parser;
pub use clap_num::maybe_hex;
#[derive(clap::Parser, Debug, Clone)]
#[command(long_about = "HPU IOp compiler, construct IOp Dag based on user description")]
pub struct Args {
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

    let (engine, builder) = hpuic::create_rhai_engine();

    //Execute user script to populate the builder
    engine.run_file(PathBuf::from(&args.input))?;

    //Dump IR in a file
    if let Some(out_ssa) = args.out_ssa.as_ref() {
        builder.lock().unwrap().write_to_file(out_ssa)?;
    }

    //Convert IR into Dag
    let dag = hpuic::IrDag::from_operations(builder.lock().unwrap().operations());
    println!("DAG stats:");
    println!("Nodes: {}", dag.get_graph().node_count());
    println!("Edge: {}", dag.get_graph().edge_count());

    // Dump graph in a dot file
    if let Some(out_dot) = args.out_dot.as_ref() {
        dag.write_dot_file(out_dot)?;
    }

    // Create Graph gui if required
    if args.view {
        hpuic::dag_display(&dag);
    }
    Ok(())
}
