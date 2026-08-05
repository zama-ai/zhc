//! On-demand compilation of homomorphic circuits into HPU programs.
//!
//! This module exposes [`Pipeline`], the entry point for turning a circuit — a `Builder` from
//! the `zhc_builder` crate — into everything the compiler can derive from it: intermediate
//! representations at every abstraction level, binary instruction streams, assembly listings,
//! performance metrics, execution traces, and interactive graph drawings.
//!
//! # Pull-Based Compilation
//!
//! The compilation flow is described once and for all, as a fixed graph whose nodes are
//! compilation steps — lowering, scheduling, register allocation, code generation, measuring,
//! tracing — and whose edges are the artifacts they exchange. A [`Pipeline`] walks that graph
//! *lazily*: creating one costs nothing, and requesting an artifact runs only the steps that this
//! particular artifact depends on. Steps that have already run are never run again, so requesting
//! the device-level IR and then the instruction stream derived from it performs the work they
//! share exactly once.
//!
//! The API therefore comes in two halves. The `with_*` methods declare the inputs of the
//! compilation and are meant to be chained right after [`Pipeline::new`], while the `get_*`
//! methods request artifacts, in any order and as many times as needed.
//!
//! # Single-HPU, Multi-HPU, and Software VM Flows
//!
//! Three families of artifacts coexist, one per compilation target. The single-HPU flow
//! compiles the circuit for one HPU and is driven by an `HpuConfig`. The multi-HPU flow
//! spreads the circuit over the HPUs of a multi-HPU system — inserting the inter-HPU
//! transfers this requires — and is driven by a `MultiHpuConfig`; its artifacts are exposed by the
//! `get_multi_*` methods and are usually collections, holding one entry per HPU. The software VM
//! flow runs the circuit on the CPU cores of the host machine instead of on HPU hardware, and is
//! driven by a `VmConfig` — the cryptographic and memory-layout parameters of one VM instance —
//! together with a `Topology` describing those cores, which defaults to the topology detected on
//! the current machine but can be overridden with [`with_topology`](Self::with_topology); its
//! lowering goes directly from the integer-level IR to a VM-specific language and from there
//! straight to an executable plan, with no intermediate stages shared with the HPU flows. The
//! three configurations — `HpuConfig`, `MultiHpuConfig`, and `VmConfig`, all from the `zhc_config`
//! crate — are mutually exclusive, so a given pipeline serves exactly one of the three families of
//! methods, never more than one.
//!
//! # Examples
//!
//! Compiling a circuit for a single HPU, then reading the instruction stream and the latency it
//! is expected to take:
//!
//! ```rust,no_run
//! # use zhc_pipeline::Pipeline;
//! # use zhc_builder::{Builder, CiphertextBlockSpec};
//! # use zhc_config::hpu::HpuConfig;
//! # let builder = Builder::new(CiphertextBlockSpec(2, 2));
//! let mut pipeline = Pipeline::new()
//!     .with_builder(builder)
//!     .with_hpu_config(HpuConfig::default());
//!
//! let stream = pipeline.get_hpu_stream().clone();
//! println!("{} instructions, {}", stream[0], pipeline.get_hpu_metrics().latency);
//!
//! // Both artifacts came from the same intermediate representations, which the pipeline
//! // computed once and kept.
//! println!("{} device operations", pipeline.get_doplang().n_ops());
//! ```
//!
//! Compiling the same circuit for a multi-HPU system, then opening the trace of its simulated
//! execution:
//!
//! ```rust,no_run
//! # use zhc_pipeline::Pipeline;
//! # use zhc_builder::{Builder, CiphertextBlockSpec};
//! # use zhc_config::multi_hpu::MultiHpuConfig;
//! # let builder = Builder::new(CiphertextBlockSpec(2, 2));
//! let mut pipeline = Pipeline::new()
//!     .with_builder(builder)
//!     .with_multi_hpu_config(MultiHpuConfig::default());
//!
//! // One instruction stream per HPU of the system.
//! for (hpu, stream) in pipeline.get_multi_hpu_stream().iter().enumerate() {
//!     println!("HPU {hpu}: {} instructions", stream[0]);
//! }
//!
//! pipeline.get_multi_hpu_trace().open().unwrap();
//! ```
//!
//! Running the same circuit on the software VM, then reading the plan it was compiled to:
//!
//! ```rust,no_run
//! # use zhc_pipeline::Pipeline;
//! # use zhc_builder::{Builder, CiphertextBlockSpec};
//! # use zhc_config::vm::VmConfig;
//! # let builder = Builder::new(CiphertextBlockSpec(2, 2));
//! # let vm_config: VmConfig = unimplemented!();
//! let mut pipeline = Pipeline::new()
//!     .with_builder(builder)
//!     .with_vm_config(vm_config);
//!
//! let plan = pipeline.get_vm_execution_plan();
//! println!("{} worker threads", plan.irs.len());
//! ```

use std::sync::LazyLock;

use zhc_builder::{Builder, Type};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig, vm::VmConfig};
use zhc_ir::{
    IR, OpMap, Signature, ValId, evaluation::LazyEvaluator, partition::PartitionId,
    visualization::Hierarchy,
};
use zhc_langs::{
    doplang::DopLang,
    hpulang::{HpuLang, HpuLocality, LutId},
    ioplang::IopLang,
    pipelinelang::{PipelineInstructionSet, PipelineLang},
    vmlang::VmLang,
};
use zhc_utils::{
    files::{FileHandle, PerfettoTrace},
    svec,
    topology::Topology,
};

use crate::{
    hpu::{metrics::HpuMetrics, translation_table::DOpRepr},
    misc::PbsMetrics,
    vm::scheduler::VmExecutionPlan,
};

struct ArtifactsValids {
    builder: ValId,
    ioplang: ValId,
    slack_drawing: ValId,
    pbs_metrics: ValId,
    partitions: ValId,
    prototype: ValId,
    hpu_config: ValId,
    hpulang_translated: ValId,
    hpu_lut_payload: ValId,
    hpulang_scheduled: ValId,
    doplang: ValId,
    hpu_stream: ValId,
    hpu_metrics: ValId,
    hpu_trace: ValId,
    hpu_assembly: ValId,
    multi_hpu_config: ValId,
    multi_hpulang_translated: ValId,
    multi_hpu_localities: ValId,
    multi_hpulang_scheduled: ValId,
    multi_doplang: ValId,
    multi_hpu_trace: ValId,
    multi_hpu_stream: ValId,
    multi_hpu_assembly: ValId,
    vm_config: ValId,
    topology: ValId,
    vmlang: ValId,
    vm_execution_plan: ValId,
}

static PIPELINE: LazyLock<(IR<PipelineLang>, ArtifactsValids)> = LazyLock::new(|| {
    use PipelineInstructionSet::*;
    let mut ir = IR::<PipelineLang>::empty();

    // Commons
    let (_, rets) = ir.add_op(InputBuilder, svec![]);
    let builder = rets[0];
    let (_, rets) = ir.add_op(BuilderToIopLang, svec![builder]);
    let ioplang = rets[0];
    let (_, rets) = ir.add_op(BuilderToPartitions, svec![builder]);
    let partitions = rets[0];
    let (_, rets) = ir.add_op(BuilderToPrototype, svec![builder]);
    let prototype = rets[0];
    let (_, rets) = ir.add_op(DrawSlack, svec![ioplang]);
    let slack_drawing = rets[0];
    let (_, rets) = ir.add_op(ComputePbsMetrics, svec![ioplang]);
    let pbs_metrics = rets[0];

    // Hpu
    let (_, rets) = ir.add_op(InputHpuConfig, svec![]);
    let hpu_config = rets[0];
    let (_, rets) = ir.add_op(IopLangToHpuLang, svec![ioplang]);
    let hpulang_translated = rets[0];
    let hpu_lut_payload = rets[1];
    let (_, rets) = ir.add_op(ScheduleHpuLang, svec![hpulang_translated, hpu_config]);
    let hpulang_scheduled = rets[0];
    let (_, rets) = ir.add_op(AllocateDopLang, svec![hpulang_scheduled, hpu_config]);
    let doplang = rets[0];
    let (_, rets) = ir.add_op(GenerateHpuStream, svec![doplang]);
    let hpu_stream = rets[0];
    let (_, rets) = ir.add_op(TraceHpuExecution, svec![doplang, hpu_config]);
    let hpu_trace = rets[0];
    let (_, rets) = ir.add_op(ComputeHpuMetrics, svec![doplang, hpulang_scheduled]);
    let hpu_metrics = rets[0];
    let (_, rets) = ir.add_op(GenerateHpuAssembly, svec![doplang]);
    let hpu_assembly = rets[0];

    // Multi-Hpu
    let (_, rets) = ir.add_op(InputMultiHpuConfig, svec![]);
    let multi_hpu_config = rets[0];
    let (_, rets) = ir.add_op(IopLangToMultiHpu, svec![ioplang, partitions]);
    let multi_hpulang_translated = rets[0];
    let multi_hpu_localities = rets[1];
    let (_, rets) = ir.add_op(
        ScheduleMultiHpuLang,
        svec![
            multi_hpulang_translated,
            multi_hpu_localities,
            multi_hpu_config
        ],
    );
    let multi_hpulang_scheduled = rets[0];
    let (_, rets) = ir.add_op(
        AllocateMultiDopLang,
        svec![multi_hpulang_scheduled, multi_hpu_config],
    );
    let multi_doplang = rets[0];
    let (_, rets) = ir.add_op(
        TraceMultiHpuExecution,
        svec![multi_doplang, multi_hpu_config],
    );
    let multi_hpu_trace = rets[0];
    let (_, rets) = ir.add_op(GenerateMultiHpuStream, svec![multi_doplang]);
    let multi_hpu_stream = rets[0];
    let (_, rets) = ir.add_op(GenerateMultiHpuAssembly, svec![multi_doplang]);
    let multi_hpu_assembly = rets[0];

    // Vm
    let (_, rets) = ir.add_op(InputVmConfig, svec![]);
    let vm_config = rets[0];
    let (_, rets) = ir.add_op(InputTopology, svec![]);
    let topology = rets[0];
    let (_, rets) = ir.add_op(IopLangToVmLang, svec![ioplang]);
    let vmlang = rets[0];
    let (_, rets) = ir.add_op(GenerateVmExecutionPlan, svec![vmlang, vm_config, topology]);
    let vm_execution_plan = rets[0];

    (
        ir,
        ArtifactsValids {
            builder,
            ioplang,
            pbs_metrics,
            slack_drawing,
            partitions,
            prototype,
            hpu_config,
            hpulang_translated,
            hpu_lut_payload,
            hpulang_scheduled,
            doplang,
            hpu_stream,
            hpu_metrics,
            hpu_trace,
            hpu_assembly,
            multi_hpu_config,
            multi_hpulang_translated,
            multi_hpu_localities,
            multi_hpulang_scheduled,
            multi_doplang,
            multi_hpu_trace,
            multi_hpu_stream,
            multi_hpu_assembly,
            vm_config,
            topology,
            vmlang,
            vm_execution_plan,
        },
    )
});

#[allow(non_snake_case)]
fn IR() -> &'static IR<PipelineLang> {
    &PIPELINE.0
}
#[allow(non_snake_case)]
fn VALIDS() -> &'static ArtifactsValids {
    &PIPELINE.1
}

mod artifacts;
mod context;
mod evaluation;

use artifacts::*;

use crate::pipeline::context::PipelineContext;

/// A lazily-evaluated compilation of a circuit into HPU artifacts.
///
/// Holds the inputs of a compilation — the circuit and the configuration of the target — together
/// with every artifact computed so far. Artifacts are requested one at a time with the `get_*`
/// methods, which run the compilation steps still missing and keep their results, so the same
/// pipeline can be queried again and again without ever redoing work.
///
/// Instances are configured by chaining the `with_*` methods on a fresh [`new`](Self::new)
/// pipeline. Requesting an artifact takes `&mut self`, since the call may compile, and hands back
/// a reference into the pipeline's own storage.
pub struct Pipeline {
    eval: LazyEvaluator<'static, PipelineLang, PipelineArtifact>,
    context: PipelineContext,
}

impl Pipeline {
    /// Creates a pipeline with no circuit and no target configuration.
    ///
    /// Nothing is compiled at this point: every step of the flow starts out pending. The inputs
    /// must be declared with [`with_builder`](Self::with_builder) and one of
    /// [`with_hpu_config`](Self::with_hpu_config) or
    /// [`with_multi_hpu_config`](Self::with_multi_hpu_config) before an artifact that needs them
    /// is requested.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let pipeline = Pipeline::new()
    ///     .with_builder(builder)
    ///     .with_hpu_config(HpuConfig::default());
    /// ```
    pub fn new() -> Self {
        Pipeline {
            eval: LazyEvaluator::from_ir(IR()),
            context: PipelineContext::new(),
        }
    }

    fn eventually_report_failure(&self) {
        if !self.eval.is_ok() {
            let failed = self
                .eval
                .as_view()
                .walk_ops_linear()
                .find(|op| op.get_annotation().is_panic())
                .unwrap();
            panic!(
                "Failed to evaluate pipeline. Panic occured evaluating step:\n{}",
                failed.format()
            )
        }
    }

    /// Renders the current state of the compilation as an interactive HTML graph.
    ///
    /// Draws the graph of compilation steps as it stands, annotated with the state of each of
    /// them: which artifacts have been computed, which are still pending, and which ones failed.
    /// The call itself compiles nothing, which makes it a convenient way to see what a series of
    /// `get_*` calls actually triggered — or, on a fresh pipeline, to discover the compilation
    /// flow itself.
    ///
    /// The returned handle points at a freshly created temporary file, which can be displayed in
    /// the default browser with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if the HTML file cannot be written.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// // Displays the whole compilation flow, with every step still pending.
    /// Pipeline::new().draw_state().open().unwrap();
    /// ```
    pub fn draw_state(&self) -> FileHandle {
        let h_root = Hierarchy::new();
        let h_commons = h_root.make_child("Commons");
        let h_hpu = h_root.make_child("Hpu");
        let h_mhpu = h_root.make_child("Multi-Hpu");
        let h_vm = h_root.make_child("Vm");
        let opmap = self.eval.as_view().totally_mapped_opmap(|op| {
            use zhc_langs::pipelinelang::Affinity::*;
            match op.get_instruction().get_affinity() {
                Commons => h_commons.clone(),
                Hpu => h_hpu.clone(),
                MultiHpu => h_mhpu.clone(),
                Vm => h_vm.clone(),
            }
        });

        self.eval.as_view().draw_to_html(Some(opmap))
    }

    /// Sets the circuit to compile.
    ///
    /// The `builder` argument holds the integer-level circuit as recorded by the `zhc_builder`
    /// crate: its inputs, the integer operations applied to them, and its outputs. Every artifact
    /// but the target configurations descends from it. The circuit is optimized on the way in, so
    /// the IR returned by [`get_ioplang`](Self::get_ioplang) is the optimized form of what is
    /// given here.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let pipeline = Pipeline::new().with_builder(builder);
    /// ```
    pub fn with_builder(mut self, builder: Builder) -> Self {
        self.context.builder = Some(builder);
        self
    }

    /// Sets the configuration of the single HPU to compile for.
    ///
    /// The `config` argument describes the target hardware — clock frequency, register file size,
    /// bootstrapping batch bounds, memory and ALU latencies — and drives scheduling, register
    /// allocation, and the timing model behind metrics and traces. Setting it selects the
    /// single-HPU flow, that is, every `get_*` method without a `multi_` in its name.
    ///
    /// # Panics
    ///
    /// Panics if a multi-HPU configuration was already set with
    /// [`with_multi_hpu_config`](Self::with_multi_hpu_config), as the two flows are mutually
    /// exclusive.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_config::hpu::HpuConfig;
    /// let pipeline = Pipeline::new().with_hpu_config(HpuConfig::default());
    /// ```
    pub fn with_hpu_config(mut self, config: HpuConfig) -> Self {
        assert!(self.context.multi_hpu_config.is_none() && self.context.vm_config.is_none());
        self.context.hpu_config = Some(config);
        self
    }

    /// Sets the configuration of the multi-HPU system to compile for.
    ///
    /// The `config` argument describes the target system: the HPU configuration its HPUs
    /// share, and how many of them there are. Setting it selects the multi-HPU flow, that is,
    /// every `get_multi_*` method, which spreads the circuit over the HPUs and inserts the
    /// transfers needed to move ciphertexts between them.
    ///
    /// # Panics
    ///
    /// Panics if a single-HPU configuration was already set with
    /// [`with_hpu_config`](Self::with_hpu_config), as the two flows are mutually exclusive.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// let pipeline = Pipeline::new().with_multi_hpu_config(MultiHpuConfig::default());
    /// ```
    pub fn with_multi_hpu_config(mut self, config: MultiHpuConfig) -> Self {
        assert!(self.context.hpu_config.is_none() && self.context.vm_config.is_none());
        self.context.multi_hpu_config = Some(config);
        self
    }

    /// Sets the configuration of the software VM to compile for.
    ///
    /// The `config` argument holds the cryptographic and memory-layout parameters of the VM
    /// instance — key and decomposition parameters, register file size — and drives lowering and
    /// scheduling for the software VM flow. Setting it selects that flow, that is, every method
    /// named after a VM artifact: [`get_vmlang`](Self::get_vmlang),
    /// [`get_vm_execution_plan`](Self::get_vm_execution_plan), and their `into_*` counterparts.
    ///
    /// # Panics
    ///
    /// Panics if a single-HPU configuration was already set with
    /// [`with_hpu_config`](Self::with_hpu_config), or a multi-HPU one with
    /// [`with_multi_hpu_config`](Self::with_multi_hpu_config), as the three flows are mutually
    /// exclusive.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_config::vm::VmConfig;
    /// # let config: VmConfig = unimplemented!();
    /// let pipeline = Pipeline::new().with_vm_config(config);
    /// ```
    pub fn with_vm_config(mut self, config: VmConfig) -> Self {
        assert!(self.context.multi_hpu_config.is_none() && self.context.hpu_config.is_none());
        self.context.vm_config = Some(config);
        self
    }

    /// Selects the legacy scheduler for the single-HPU flow.
    ///
    /// The default scheduler orders operations under a single as-late-as-possible step, whereas
    /// the legacy one is two-step and more often than not gives worse results. The
    /// choice changes the execution order picked for the circuit, and with it everything derived
    /// from [`get_scheduled_hpulang`](Self::get_scheduled_hpulang) — batching, register pressure,
    /// instruction stream, and measured latency. It has no effect on the multi-HPU flow.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let mut pipeline = Pipeline::new()
    ///     .with_builder(builder)
    ///     .with_hpu_config(HpuConfig::default())
    ///     .with_legacy_hpu_scheduler();
    ///
    /// // Scheduling, and everything downstream of it, now goes through the legacy scheduler.
    /// println!("{}", pipeline.get_hpu_metrics().latency);
    /// ```
    pub fn with_legacy_hpu_scheduler(mut self) -> Self {
        self.context.legacy_hpu_scheduler = true;
        self
    }

    /// Records the individual events of the simulated device in the execution traces.
    ///
    /// By default, a trace holds the successive states of the units of the device and the load
    /// counters measured on them, which is what following an execution along its timeline calls
    /// for. This turns the simulation to its most verbose tracing mode, which adds one section to
    /// the trace: every event the simulated device goes through, each on its own track and carrying
    /// its payload — an operation being issued to a processing element, a dependency being
    /// unlocked, a bootstrapping batch being launched or landing, a unit becoming unavailable. This
    /// is the level of detail that answers *why* the device did something, where the states and
    /// counters answer *when*.
    ///
    /// The extra events cost simulation time and trace size, hence the opt-in. The setting applies
    /// to both flows, that is to the traces returned by [`get_hpu_trace`](Self::get_hpu_trace) and
    /// [`get_multi_hpu_trace`](Self::get_multi_hpu_trace), and to nothing else: the other
    /// artifacts, metrics included, are computed identically with or without it.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let builder = Builder::new(CiphertextBlockSpec(2, 2));
    /// let mut pipeline = Pipeline::new()
    ///     .with_builder(builder)
    ///     .with_hpu_config(HpuConfig::default())
    ///     .with_trace_hpu_events();
    ///
    /// // The trace now carries the event tracks, on top of the states and the counters.
    /// pipeline.get_hpu_trace().open().unwrap();
    /// ```
    pub fn with_trace_hpu_events(mut self) -> Self {
        self.context.hpu_trace_events = true;
        self
    }

    /// Sets the hardware topology the software VM schedules across.
    ///
    /// The `topology` argument describes the cores and memory of the machine the VM's compiled
    /// program will run on, and drives how many worker threads
    /// [`get_vm_execution_plan`](Self::get_vm_execution_plan) schedules work over. A fresh
    /// [`Pipeline`] already carries the topology of the machine it runs on, detected
    /// automatically, so this method is only needed to compile against a topology other than the
    /// current one — for instance a smaller one, to see how the schedule adapts to fewer cores.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_utils::topology::Topology;
    /// // Schedules as if only a single core were available.
    /// let pipeline = Pipeline::new().with_topology(Topology::single_core());
    /// ```
    pub fn with_topology(mut self, topology: Topology) -> Self {
        self.context.topology = topology;
        self
    }

    /// Returns the circuit being compiled.
    ///
    /// Hands back the circuit given to [`with_builder`](Self::with_builder), which is convenient
    /// when the circuit was produced elsewhere and only the pipeline holds on to it.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// let builder: &Builder = pipeline.get_builder();
    /// ```
    pub fn get_builder(&mut self) -> &Builder {
        self.eval.pull_val(&mut self.context, VALIDS().builder);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().builder)
            .unwrap()
            .unwrap_builder_ref()
    }

    /// Returns the input and output types of the circuit.
    ///
    /// The prototype is the I/O signature of the circuit: the type of every argument it takes and
    /// of every value it returns, in declaration order. Each of these types is either an encrypted
    /// or a plaintext integer, of a given bit width and block layout. This is what a caller of the
    /// compiled program needs to know to run it — which values to encrypt, in which order to hand
    /// them over, and what to expect back.
    ///
    /// The signature is read straight from the circuit, so asking for it triggers no compilation
    /// work.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// let prototype = pipeline.get_prototype();
    /// println!("{:?} -> {:?}", prototype.get_args(), prototype.get_returns());
    /// ```
    pub fn get_prototype(&mut self) -> &Signature<Type> {
        self.eval.pull_val(&mut self.context, VALIDS().prototype);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().prototype)
            .unwrap()
            .unwrap_prototype_ref()
    }

    /// Returns the configuration of the target HPU.
    ///
    /// Hands back the configuration given to [`with_hpu_config`](Self::with_hpu_config), which is
    /// useful to re-read the hardware parameters the single-HPU artifacts were compiled against.
    ///
    /// # Panics
    ///
    /// Panics if no single-HPU configuration was set with
    /// [`with_hpu_config`](Self::with_hpu_config).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new().with_hpu_config(HpuConfig::default());
    /// println!("{} registers", pipeline.get_hpu_config().regf_size);
    /// ```
    pub fn get_hpu_config(&mut self) -> &HpuConfig {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_config);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpu_config)
            .unwrap()
            .unwrap_hpu_config_ref()
    }

    /// Returns the optimized integer-level IR of the circuit.
    ///
    /// This is the first artifact derived from the circuit: an IR in the IOP language, whose
    /// operations still work on whole encrypted integers rather than on radix blocks, as left by
    /// the optimization passes of the builder. Every other artifact of the pipeline is compiled
    /// from it, so it is the right place to look at what is actually being compiled.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// println!("{} integer operations", pipeline.get_ioplang().n_ops());
    /// ```
    pub fn get_ioplang(&mut self) -> &IR<IopLang> {
        self.eval.pull_val(&mut self.context, VALIDS().ioplang);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().ioplang)
            .unwrap()
            .unwrap_iop_lang_ref()
    }

    /// Returns the block-level HPU IR of the circuit, before scheduling.
    ///
    /// Lowering the integer-level IR replaces each integer operation by the block-level operations
    /// and programmable bootstrapping lookups that implement it, giving an IR in the HPU language.
    /// Its operations are still in translation order — no execution order has been picked yet, and
    /// no register has been assigned — which makes this artifact the one to inspect when checking
    /// how an integer operation is expressed in terms of block operations.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// println!("{} block operations", pipeline.get_translated_hpulang().n_ops());
    /// ```
    pub fn get_translated_hpulang(&mut self) -> &IR<HpuLang> {
        self.eval
            .pull_val(&mut self.context, VALIDS().hpulang_translated);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpulang_translated)
            .unwrap()
            .unwrap_hpu_lang_translated_ref()
    }

    /// Returns the tables of the non-builtin LUTs the circuit references,
    /// keyed by the gid allocated to each.
    ///
    /// A builtin LUT is already resident in the board's LUT memory,
    /// so the instruction stream refers to it by gid alone and it carries no payload.
    /// A non-builtin LUT is a `Lut1Def::Table`, today emitted only by `iop_match_value`.
    /// It is assigned a gid past the builtin range,
    /// and its table has to be uploaded before the instruction stream runs,
    /// or the device would bootstrap against whatever that slot happens to hold.
    ///
    /// Tables are deduplicated by content, so an operation reusing one lookup table many times
    /// pays for it once.
    pub fn get_hpu_lut_payload(&mut self) -> &Vec<(LutId, Vec<u8>)> {
        self.eval
            .pull_val(&mut self.context, VALIDS().hpu_lut_payload);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpu_lut_payload)
            .unwrap()
            .unwrap_hpu_lut_payload_ref()
    }

    /// Returns the block-level HPU IR of the circuit, after scheduling.
    ///
    /// Scheduling picks an execution order for the block-level operations and groups programmable
    /// bootstrappings into the batches the device processes in one go, within the resources the
    /// HPU configuration declares. Operands are still symbolic values at this point; they are
    /// bound to physical locations later, by [`get_doplang`](Self::get_doplang).
    ///
    /// Which scheduler produces this artifact can be changed with
    /// [`with_legacy_hpu_scheduler`](Self::with_legacy_hpu_scheduler).
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_hpu_config`](Self::with_hpu_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// // Draws the scheduled program, whose operations are laid out in execution order.
    /// pipeline.get_scheduled_hpulang().draw_to_html(None).open().unwrap();
    /// ```
    pub fn get_scheduled_hpulang(&mut self) -> &IR<HpuLang> {
        self.eval
            .pull_val(&mut self.context, VALIDS().hpulang_scheduled);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpulang_scheduled)
            .unwrap()
            .unwrap_hpu_lang_scheduled_ref()
    }

    /// Returns the device-level IR of the circuit.
    ///
    /// Register allocation rewrites the scheduled block-level IR into the DOP language, where
    /// every operand is a physical location of the HPU — a register of the register file or a
    /// memory slot — and where the loads and stores needed to spill values are explicit. This is
    /// the last intermediate representation before code generation, and therefore the one the
    /// instruction stream, the assembly listing, the metrics, and the trace are all read from.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_hpu_config`](Self::with_hpu_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// println!("{} device operations", pipeline.get_doplang().n_ops());
    /// ```
    pub fn get_doplang(&mut self) -> &IR<DopLang> {
        self.eval.pull_val(&mut self.context, VALIDS().doplang);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().doplang)
            .unwrap()
            .unwrap_dop_lang_ref()
    }

    /// Returns the binary instruction stream to be sent to the device.
    ///
    /// Encodes each operation of the device-level IR into the machine word the HPU decodes, in
    /// execution order. The first word of the stream is a header holding the number of
    /// instructions that follow, so a stream carrying `n` instructions is `n + 1` words long.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_hpu_config`](Self::with_hpu_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// let stream = pipeline.get_hpu_stream();
    /// println!("{} instructions in {} words", stream[0], stream.len());
    /// ```
    pub fn get_hpu_stream(&mut self) -> &Vec<DOpRepr> {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_stream);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpu_stream)
            .unwrap()
            .unwrap_hpu_stream_ref()
    }

    /// Returns the bootstrapping metrics of the circuit.
    ///
    /// Characterizes the circuit before it is compiled for any particular HPU: how many
    /// programmable bootstrappings it performs, how long its longest chain of dependent
    /// bootstrappings is, and how much freedom the remaining ones have in time. These figures
    /// depend on the circuit alone, not on a target configuration, which makes them the right way
    /// to gauge the intrinsic cost and the available parallelism of a circuit.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// let metrics = pipeline.get_pbs_metrics();
    /// println!("{} bootstrappings, {} deep", metrics.count, metrics.critical_length);
    /// ```
    pub fn get_pbs_metrics(&mut self) -> &PbsMetrics {
        self.eval.pull_val(&mut self.context, VALIDS().pbs_metrics);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().pbs_metrics)
            .unwrap()
            .unwrap_pbs_metrics_ref()
    }

    /// Returns the performance metrics of the compiled program.
    ///
    /// Runs the device-level program through the timing model of the HPU and reports the latency
    /// it takes, the theoretical lower bound it is worth comparing against, the time the
    /// bootstrapping unit spent idle, and the distribution of the batch sizes the scheduler
    /// achieved. This is the artifact to look at when judging how well a circuit was compiled.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_hpu_config`](Self::with_hpu_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// let metrics = pipeline.get_hpu_metrics();
    /// println!("{} (lower bound {})", metrics.latency, metrics.lower_bound);
    /// ```
    pub fn get_hpu_metrics(&mut self) -> &HpuMetrics {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_metrics);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpu_metrics)
            .unwrap()
            .unwrap_hpu_metrics_ref()
    }

    /// Returns a trace of the simulated execution of the compiled program.
    ///
    /// Replays the device-level program on the timing model of the HPU and records what each
    /// unit of the device does over time. Where [`get_hpu_metrics`](Self::get_hpu_metrics) sums
    /// the execution up in a handful of numbers, this shows it instruction by instruction, which
    /// is how a stall or an unexpectedly small batch is tracked down. The returned handle points
    /// at a trace file that can be displayed in the Perfetto UI with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_hpu_config`](Self::with_hpu_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// pipeline.get_hpu_trace().open().unwrap();
    /// ```
    pub fn get_hpu_trace(&mut self) -> &PerfettoTrace {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_trace);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpu_trace)
            .unwrap()
            .unwrap_hpu_trace_ref()
    }

    /// Returns an interactive drawing of the circuit's scheduling slack.
    ///
    /// Draws the integer-level IR as a graph whose operations are coloured by their slack — how
    /// much they can be moved in time without delaying the circuit — on a traffic-light scale, so
    /// that the critical path stands out from the operations that can wait. The returned handle
    /// points at an HTML file that can be displayed in the default browser with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// pipeline.get_slack_drawing().open().unwrap();
    /// ```
    pub fn get_slack_drawing(&mut self) -> &FileHandle {
        self.eval
            .pull_val(&mut self.context, VALIDS().slack_drawing);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().slack_drawing)
            .unwrap()
            .unwrap_slack_drawing_ref()
    }

    /// Returns the partition each operation of the circuit belongs to.
    ///
    /// A partition is a labelled cluster of neighbouring operations, declared while building the
    /// circuit, that the compiler treats as a single unit of work; the returned map associates
    /// every operation of the optimized integer-level IR with its own. The multi-HPU flow reads
    /// this map to decide which HPU runs what, so partitioning a circuit is how its placement is
    /// steered.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// let partitions = pipeline.get_partitions();
    /// println!("{} operations placed in partitions", partitions.iter().count());
    /// ```
    pub fn get_partitions(&mut self) -> &OpMap<PartitionId> {
        self.eval.pull_val(&mut self.context, VALIDS().partitions);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().partitions)
            .unwrap()
            .unwrap_partitions_ref()
    }

    /// Returns a human-readable assembly listing of the compiled program.
    ///
    /// Emits the device-level program as assembly text and writes it to a file. The content is the
    /// same program as the one encoded by [`get_hpu_stream`](Self::get_hpu_stream), in a form
    /// meant to be read rather than executed. The returned handle can be displayed with its `open`
    /// method.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_hpu_config`](Self::with_hpu_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// pipeline.get_hpu_assembly().open().unwrap();
    /// ```
    pub fn get_hpu_assembly(&mut self) -> &FileHandle {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_assembly);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().hpu_assembly)
            .unwrap()
            .unwrap_hpu_assembly_ref()
    }

    /// Returns the configuration of the target multi-HPU system.
    ///
    /// Hands back the configuration given to
    /// [`with_multi_hpu_config`](Self::with_multi_hpu_config), which is useful to re-read the
    /// HPU configuration and the HPU count the multi-HPU artifacts were compiled against.
    ///
    /// # Panics
    ///
    /// Panics if no multi-HPU configuration was set with
    /// [`with_multi_hpu_config`](Self::with_multi_hpu_config).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// # let mut pipeline = Pipeline::new().with_multi_hpu_config(MultiHpuConfig::default());
    /// println!("{} HPUs", pipeline.get_multi_hpu_config().n_hpus);
    /// ```
    pub fn get_multi_hpu_config(&mut self) -> &MultiHpuConfig {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_config);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpu_config)
            .unwrap()
            .unwrap_multi_hpu_config_ref()
    }

    /// Returns the block-level HPU IR of the whole system, before scheduling.
    ///
    /// Lowers the integer-level IR the way the single-HPU flow does, then assigns every
    /// operation to a HPU following the circuit's partitions, and inserts an explicit transfer
    /// wherever an operation consumes a value that lives on another HPU. The result is one IR
    /// covering the whole system: where each of its operations runs is told by
    /// [`get_multi_hpu_localities`](Self::get_multi_hpu_localities), and it is cut into per-HPU
    /// programs by [`get_scheduled_multi_hpulang`](Self::get_scheduled_multi_hpulang).
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// let ir = pipeline.get_translated_multi_hpulang();
    /// println!("{} block operations, transfers included", ir.n_ops());
    /// ```
    pub fn get_translated_multi_hpulang(&mut self) -> &IR<HpuLang> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpulang_translated);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpulang_translated)
            .unwrap()
            .unwrap_multi_hpu_lang_translated_ref()
    }

    /// Returns the placement of each operation over the HPUs of the system.
    ///
    /// Associates every operation of the IR returned by
    /// [`get_translated_multi_hpulang`](Self::get_translated_multi_hpulang) with the HPU it runs
    /// on, with the pair of HPUs a transfer moves data between, or with the set of HPUs it is
    /// replicated on. HPUs are numbered in the order the circuit's partitions are first met, so
    /// this map is what to read to know where the compiler decided to put the work.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_langs::hpulang::HpuLocality;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// let transfers = pipeline
    ///     .get_multi_hpu_localities()
    ///     .iter()
    ///     .filter(|(_, locality)| matches!(**locality, HpuLocality::Transfer { .. }))
    ///     .count();
    /// println!("{transfers} inter-HPU transfers");
    /// ```
    pub fn get_multi_hpu_localities(&mut self) -> &OpMap<HpuLocality> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_localities);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpu_localities)
            .unwrap()
            .unwrap_multi_hpu_localities_ref()
    }

    /// Returns the scheduled block-level HPU IR of each HPU.
    ///
    /// Schedules the operations of the whole system at once — accounting for each HPU's own
    /// resources and for the transfers the HPUs wait on — then splits the outcome into one IR
    /// per HPU, in HPU order. Each of them is a block-level program of the same shape the
    /// single-HPU flow produces, restricted to the operations its HPU runs.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_multi_hpu_config`](Self::with_multi_hpu_config), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_multi_hpu_config(MultiHpuConfig::default());
    /// for (hpu, ir) in pipeline.get_scheduled_multi_hpulang().iter().enumerate() {
    ///     println!("HPU {hpu}: {} block operations", ir.n_ops());
    /// }
    /// ```
    pub fn get_scheduled_multi_hpulang(&mut self) -> &Vec<IR<HpuLang>> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpulang_scheduled);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpulang_scheduled)
            .unwrap()
            .unwrap_multi_hpu_lang_scheduled_ref()
    }

    /// Returns the device-level IR of each HPU.
    ///
    /// Allocates registers separately for every HPU's scheduled program, against the HPU
    /// configuration the system shares, giving one device-level IR per HPU in HPU order. Each
    /// of them is what its HPU will actually run, and is the program the per-HPU streams,
    /// listings, and traces are generated from.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_multi_hpu_config`](Self::with_multi_hpu_config), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_multi_hpu_config(MultiHpuConfig::default());
    /// for (hpu, ir) in pipeline.get_multi_doplang().iter().enumerate() {
    ///     println!("HPU {hpu}: {} device operations", ir.n_ops());
    /// }
    /// ```
    pub fn get_multi_doplang(&mut self) -> &Vec<IR<DopLang>> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_doplang);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_doplang)
            .unwrap()
            .unwrap_multi_dop_lang_ref()
    }

    /// Returns a trace of the simulated execution of the whole system.
    ///
    /// Replays the per-HPU programs together on the timing model of the system, so that the
    /// activity of every HPU and the transfers between them appear side by side in a single
    /// trace. This is where the cost of splitting a circuit across HPUs becomes visible: HPUs
    /// idling while a transfer completes show up as gaps. The returned handle points at a trace
    /// file that can be displayed in the Perfetto UI with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_multi_hpu_config`](Self::with_multi_hpu_config), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_multi_hpu_config(MultiHpuConfig::default());
    /// pipeline.get_multi_hpu_trace().open().unwrap();
    /// ```
    pub fn get_multi_hpu_trace(&mut self) -> &PerfettoTrace {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_trace);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpu_trace)
            .unwrap()
            .unwrap_multi_hpu_trace_ref()
    }

    /// Returns the binary instruction stream of each HPU.
    ///
    /// Encodes every HPU's device-level program into machine words, giving one stream per HPU
    /// in HPU order. Each stream is laid out exactly like the single-HPU one returned by
    /// [`get_hpu_stream`](Self::get_hpu_stream): a header word holding the instruction count,
    /// followed by the instructions themselves.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_multi_hpu_config`](Self::with_multi_hpu_config), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_multi_hpu_config(MultiHpuConfig::default());
    /// for (hpu, stream) in pipeline.get_multi_hpu_stream().iter().enumerate() {
    ///     println!("HPU {hpu}: {} instructions", stream[0]);
    /// }
    /// ```
    pub fn get_multi_hpu_stream(&mut self) -> &Vec<Vec<DOpRepr>> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_stream);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpu_stream)
            .unwrap()
            .unwrap_multi_hpu_stream_ref()
    }

    /// Returns a human-readable assembly listing for each HPU.
    ///
    /// Emits every HPU's device-level program as assembly text and writes it to its own file,
    /// giving one handle per HPU in HPU order. The content is the same as what
    /// [`get_multi_hpu_stream`](Self::get_multi_hpu_stream) encodes, in a form meant to be read
    /// rather than executed, and each file can be displayed with its `open` method.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_multi_hpu_config`](Self::with_multi_hpu_config), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::multi_hpu::MultiHpuConfig;
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_multi_hpu_config(MultiHpuConfig::default());
    /// for listing in pipeline.get_multi_hpu_assembly() {
    ///     println!("{listing:?}");
    /// }
    /// ```
    pub fn get_multi_hpu_assembly(&mut self) -> &Vec<FileHandle> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_assembly);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().multi_hpu_assembly)
            .unwrap()
            .unwrap_multi_hpu_assembly_ref()
    }

    /// Returns the configuration of the target software VM.
    ///
    /// Hands back the configuration given to [`with_vm_config`](Self::with_vm_config), which is
    /// useful to re-read the cryptographic and memory-layout parameters the VM artifacts were
    /// compiled against.
    ///
    /// # Panics
    ///
    /// Panics if no VM configuration was set with [`with_vm_config`](Self::with_vm_config).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_config::vm::VmConfig;
    /// # let config: VmConfig = unimplemented!();
    /// # let mut pipeline = Pipeline::new().with_vm_config(config);
    /// println!("{} registers", pipeline.get_vm_config().regf_size);
    /// ```
    pub fn get_vm_config(&mut self) -> &VmConfig {
        self.eval.pull_val(&mut self.context, VALIDS().vm_config);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().vm_config)
            .unwrap()
            .unwrap_vm_config_ref()
    }

    /// Returns the hardware topology the software VM schedules across.
    ///
    /// Hands back the topology given to [`with_topology`](Self::with_topology), or the one
    /// detected automatically on a fresh [`Pipeline`] otherwise. This is what
    /// [`get_vm_execution_plan`](Self::get_vm_execution_plan) reads to decide how many worker
    /// threads to schedule work over.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// let mut pipeline = Pipeline::new();
    /// println!("{} processors", pipeline.get_topology().n_processors());
    /// ```
    pub fn get_topology(&mut self) -> &Topology {
        self.eval.pull_val(&mut self.context, VALIDS().topology);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().topology)
            .unwrap()
            .unwrap_topology_ref()
    }

    /// Returns the IR of the circuit as the software VM executes it.
    ///
    /// Lowers the optimized integer-level IR directly into the VM language, bypassing the HPU
    /// language and the device-level IR entirely: values live in ciphertext registers with no
    /// spilling, and keyswitching is an explicit instruction feeding every bootstrapping. This is
    /// the last representation shared across VM topologies — the same IR is scheduled by
    /// [`get_vm_execution_plan`](Self::get_vm_execution_plan) whatever the target topology.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), or if a step this
    /// artifact depends on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)));
    /// println!("{} VM operations", pipeline.get_vmlang().n_ops());
    /// ```
    pub fn get_vmlang(&mut self) -> &IR<VmLang> {
        self.eval.pull_val(&mut self.context, VALIDS().vmlang);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().vmlang)
            .unwrap()
            .unwrap_vm_lang_ref()
    }

    /// Returns the executable plan the software VM runs.
    ///
    /// Schedules the VM-language IR over the worker threads implied by the current
    /// [`get_topology`](Self::get_topology), producing register-allocated bytecode ready to
    /// execute — one instruction stream per worker thread, with the cross-thread dependencies
    /// each instruction waits on baked in. Unlike the HPU flows' scheduled artifacts, this is the
    /// terminal artifact of the VM flow: nothing is derived from it further.
    ///
    /// # Panics
    ///
    /// Panics if no circuit was set with [`with_builder`](Self::with_builder), if no configuration
    /// was set with [`with_vm_config`](Self::with_vm_config), or if a step this artifact depends
    /// on panics, in which case the message names the failing step.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::vm::VmConfig;
    /// # let config: VmConfig = unimplemented!();
    /// # let mut pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_vm_config(config);
    /// let plan = pipeline.get_vm_execution_plan();
    /// println!("{} worker threads", plan.irs.len());
    /// ```
    pub fn get_vm_execution_plan(&mut self) -> &VmExecutionPlan {
        self.eval
            .pull_val(&mut self.context, VALIDS().vm_execution_plan);
        self.eventually_report_failure();
        self.eval
            .get_val(VALIDS().vm_execution_plan)
            .unwrap()
            .unwrap_vm_execution_plan_ref()
    }

    /// Consumes the pipeline and returns the owned circuit.
    ///
    /// The owning counterpart of [`get_builder`](Self::get_builder).
    ///
    /// # Panics
    ///
    /// See [`get_builder`](Self::get_builder).
    pub fn into_builder(mut self) -> Builder {
        self.eval.pull_val(&mut self.context, VALIDS().builder);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().builder)
            .unwrap()
            .unwrap_builder()
    }

    /// Consumes the pipeline and returns the owned prototype.
    ///
    /// The owning counterpart of [`get_prototype`](Self::get_prototype).
    ///
    /// # Panics
    ///
    /// See [`get_prototype`](Self::get_prototype).
    pub fn into_prototype(mut self) -> Signature<Type> {
        self.eval.pull_val(&mut self.context, VALIDS().prototype);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().prototype)
            .unwrap()
            .unwrap_prototype()
    }

    /// Consumes the pipeline and returns the owned HPU configuration.
    ///
    /// The owning counterpart of [`get_hpu_config`](Self::get_hpu_config).
    ///
    /// # Panics
    ///
    /// See [`get_hpu_config`](Self::get_hpu_config).
    pub fn into_hpu_config(mut self) -> HpuConfig {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_config);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpu_config)
            .unwrap()
            .unwrap_hpu_config()
    }

    /// Consumes the pipeline and returns the owned integer-level IR of the circuit.
    ///
    /// The owning counterpart of [`get_ioplang`](Self::get_ioplang).
    ///
    /// # Panics
    ///
    /// See [`get_ioplang`](Self::get_ioplang).
    pub fn into_ioplang(mut self) -> IR<IopLang> {
        self.eval.pull_val(&mut self.context, VALIDS().ioplang);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().ioplang)
            .unwrap()
            .unwrap_iop_lang()
    }

    /// Consumes the pipeline and returns the owned block-level HPU IR, before scheduling.
    ///
    /// The owning counterpart of [`get_translated_hpulang`](Self::get_translated_hpulang).
    ///
    /// # Panics
    ///
    /// See [`get_translated_hpulang`](Self::get_translated_hpulang).
    pub fn into_translated_hpulang(mut self) -> IR<HpuLang> {
        self.eval
            .pull_val(&mut self.context, VALIDS().hpulang_translated);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpulang_translated)
            .unwrap()
            .unwrap_hpu_lang_translated()
    }

    /// Consumes the pipeline and returns the owned block-level HPU IR, after scheduling.
    ///
    /// The owning counterpart of [`get_scheduled_hpulang`](Self::get_scheduled_hpulang).
    ///
    /// # Panics
    ///
    /// See [`get_scheduled_hpulang`](Self::get_scheduled_hpulang).
    pub fn into_scheduled_hpulang(mut self) -> IR<HpuLang> {
        self.eval
            .pull_val(&mut self.context, VALIDS().hpulang_scheduled);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpulang_scheduled)
            .unwrap()
            .unwrap_hpu_lang_scheduled()
    }

    /// Consumes the pipeline and returns the owned device-level IR of the circuit.
    ///
    /// The owning counterpart of [`get_doplang`](Self::get_doplang).
    ///
    /// # Panics
    ///
    /// See [`get_doplang`](Self::get_doplang).
    pub fn into_doplang(mut self) -> IR<DopLang> {
        self.eval.pull_val(&mut self.context, VALIDS().doplang);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().doplang)
            .unwrap()
            .unwrap_dop_lang()
    }

    /// Consumes the pipeline and returns the owned binary instruction stream.
    ///
    /// The owning counterpart of [`get_hpu_stream`](Self::get_hpu_stream).
    ///
    /// # Panics
    ///
    /// See [`get_hpu_stream`](Self::get_hpu_stream).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_pipeline::Pipeline;
    /// # use zhc_builder::{Builder, CiphertextBlockSpec};
    /// # use zhc_config::hpu::HpuConfig;
    /// # let pipeline = Pipeline::new()
    /// #     .with_builder(Builder::new(CiphertextBlockSpec(2, 2)))
    /// #     .with_hpu_config(HpuConfig::default());
    /// // The stream is now owned, and the pipeline it came from can no longer be used.
    /// let stream: Vec<_> = pipeline.into_hpu_stream();
    /// ```
    pub fn into_hpu_stream(mut self) -> Vec<DOpRepr> {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_stream);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpu_stream)
            .unwrap()
            .unwrap_hpu_stream()
    }

    /// Consumes the pipeline and returns the owned bootstrapping metrics.
    ///
    /// The owning counterpart of [`get_pbs_metrics`](Self::get_pbs_metrics).
    ///
    /// # Panics
    ///
    /// See [`get_pbs_metrics`](Self::get_pbs_metrics).
    pub fn into_pbs_metrics(mut self) -> PbsMetrics {
        self.eval.pull_val(&mut self.context, VALIDS().pbs_metrics);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().pbs_metrics)
            .unwrap()
            .unwrap_pbs_metrics()
    }

    /// Consumes the pipeline and returns the owned performance metrics.
    ///
    /// The owning counterpart of [`get_hpu_metrics`](Self::get_hpu_metrics).
    ///
    /// # Panics
    ///
    /// See [`get_hpu_metrics`](Self::get_hpu_metrics).
    pub fn into_hpu_metrics(mut self) -> HpuMetrics {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_metrics);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpu_metrics)
            .unwrap()
            .unwrap_hpu_metrics()
    }

    /// Consumes the pipeline and returns the owned execution trace.
    ///
    /// The owning counterpart of [`get_hpu_trace`](Self::get_hpu_trace).
    ///
    /// # Panics
    ///
    /// See [`get_hpu_trace`](Self::get_hpu_trace).
    pub fn into_hpu_trace(mut self) -> PerfettoTrace {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_trace);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpu_trace)
            .unwrap()
            .unwrap_hpu_trace()
    }

    /// Consumes the pipeline and returns the owned slack drawing handle.
    ///
    /// The owning counterpart of [`get_slack_drawing`](Self::get_slack_drawing).
    ///
    /// # Panics
    ///
    /// See [`get_slack_drawing`](Self::get_slack_drawing).
    pub fn into_slack_drawing(mut self) -> FileHandle {
        self.eval
            .pull_val(&mut self.context, VALIDS().slack_drawing);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().slack_drawing)
            .unwrap()
            .unwrap_slack_drawing()
    }

    /// Consumes the pipeline and returns the owned partition map.
    ///
    /// The owning counterpart of [`get_partitions`](Self::get_partitions).
    ///
    /// # Panics
    ///
    /// See [`get_partitions`](Self::get_partitions).
    pub fn into_partitions(mut self) -> OpMap<PartitionId> {
        self.eval.pull_val(&mut self.context, VALIDS().partitions);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().partitions)
            .unwrap()
            .unwrap_partitions()
    }

    /// Consumes the pipeline and returns the owned assembly listing handle.
    ///
    /// The owning counterpart of [`get_hpu_assembly`](Self::get_hpu_assembly).
    ///
    /// # Panics
    ///
    /// See [`get_hpu_assembly`](Self::get_hpu_assembly).
    pub fn into_hpu_assembly(mut self) -> FileHandle {
        self.eval.pull_val(&mut self.context, VALIDS().hpu_assembly);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().hpu_assembly)
            .unwrap()
            .unwrap_hpu_assembly()
    }

    /// Consumes the pipeline and returns the owned multi-HPU system configuration.
    ///
    /// The owning counterpart of [`get_multi_hpu_config`](Self::get_multi_hpu_config).
    ///
    /// # Panics
    ///
    /// See [`get_multi_hpu_config`](Self::get_multi_hpu_config).
    pub fn into_multi_hpu_config(mut self) -> MultiHpuConfig {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_config);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpu_config)
            .unwrap()
            .unwrap_multi_hpu_config()
    }

    /// Consumes the pipeline and returns the owned block-level HPU IR of the whole system, before
    /// scheduling.
    ///
    /// The owning counterpart of
    /// [`get_translated_multi_hpulang`](Self::get_translated_multi_hpulang).
    ///
    /// # Panics
    ///
    /// See [`get_translated_multi_hpulang`](Self::get_translated_multi_hpulang).
    pub fn into_translated_multi_hpulang(mut self) -> IR<HpuLang> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpulang_translated);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpulang_translated)
            .unwrap()
            .unwrap_multi_hpu_lang_translated()
    }

    /// Consumes the pipeline and returns the owned placement map of the system's operations.
    ///
    /// The owning counterpart of [`get_multi_hpu_localities`](Self::get_multi_hpu_localities).
    ///
    /// # Panics
    ///
    /// See [`get_multi_hpu_localities`](Self::get_multi_hpu_localities).
    pub fn into_multi_hpu_localities(mut self) -> OpMap<HpuLocality> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_localities);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpu_localities)
            .unwrap()
            .unwrap_multi_hpu_localities()
    }

    /// Consumes the pipeline and returns the owned scheduled block-level HPU IR of each HPU.
    ///
    /// The owning counterpart of
    /// [`get_scheduled_multi_hpulang`](Self::get_scheduled_multi_hpulang).
    ///
    /// # Panics
    ///
    /// See [`get_scheduled_multi_hpulang`](Self::get_scheduled_multi_hpulang).
    pub fn into_scheduled_multi_hpulang(mut self) -> Vec<IR<HpuLang>> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpulang_scheduled);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpulang_scheduled)
            .unwrap()
            .unwrap_multi_hpu_lang_scheduled()
    }

    /// Consumes the pipeline and returns the owned device-level IR of each HPU.
    ///
    /// The owning counterpart of [`get_multi_doplang`](Self::get_multi_doplang).
    ///
    /// # Panics
    ///
    /// See [`get_multi_doplang`](Self::get_multi_doplang).
    pub fn into_multi_doplang(mut self) -> Vec<IR<DopLang>> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_doplang);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_doplang)
            .unwrap()
            .unwrap_multi_dop_lang()
    }

    /// Consumes the pipeline and returns the owned execution trace of the whole system.
    ///
    /// The owning counterpart of [`get_multi_hpu_trace`](Self::get_multi_hpu_trace).
    ///
    /// # Panics
    ///
    /// See [`get_multi_hpu_trace`](Self::get_multi_hpu_trace).
    pub fn into_multi_hpu_trace(mut self) -> PerfettoTrace {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_trace);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpu_trace)
            .unwrap()
            .unwrap_multi_hpu_trace()
    }

    /// Consumes the pipeline and returns the owned binary instruction stream of each HPU.
    ///
    /// The owning counterpart of [`get_multi_hpu_stream`](Self::get_multi_hpu_stream).
    ///
    /// # Panics
    ///
    /// See [`get_multi_hpu_stream`](Self::get_multi_hpu_stream).
    pub fn into_multi_hpu_stream(mut self) -> Vec<Vec<DOpRepr>> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_stream);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpu_stream)
            .unwrap()
            .unwrap_multi_hpu_stream()
    }

    /// Consumes the pipeline and returns the owned assembly listing handles, one per HPU.
    ///
    /// The owning counterpart of [`get_multi_hpu_assembly`](Self::get_multi_hpu_assembly).
    ///
    /// # Panics
    ///
    /// See [`get_multi_hpu_assembly`](Self::get_multi_hpu_assembly).
    pub fn into_multi_hpu_assembly(mut self) -> Vec<FileHandle> {
        self.eval
            .pull_val(&mut self.context, VALIDS().multi_hpu_assembly);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().multi_hpu_assembly)
            .unwrap()
            .unwrap_multi_hpu_assembly()
    }

    /// Consumes the pipeline and returns the owned software VM configuration.
    ///
    /// The owning counterpart of [`get_vm_config`](Self::get_vm_config).
    ///
    /// # Panics
    ///
    /// See [`get_vm_config`](Self::get_vm_config).
    pub fn into_vm_config(mut self) -> VmConfig {
        self.eval.pull_val(&mut self.context, VALIDS().vm_config);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().vm_config)
            .unwrap()
            .unwrap_vm_config()
    }

    /// Consumes the pipeline and returns the owned IR of the circuit as the software VM executes
    /// it.
    ///
    /// The owning counterpart of [`get_vmlang`](Self::get_vmlang).
    ///
    /// # Panics
    ///
    /// See [`get_vmlang`](Self::get_vmlang).
    pub fn into_vmlang(mut self) -> IR<VmLang> {
        self.eval.pull_val(&mut self.context, VALIDS().vmlang);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().vmlang)
            .unwrap()
            .unwrap_vm_lang()
    }

    /// Consumes the pipeline and returns the owned executable plan the software VM runs.
    ///
    /// The owning counterpart of [`get_vm_execution_plan`](Self::get_vm_execution_plan).
    ///
    /// # Panics
    ///
    /// See [`get_vm_execution_plan`](Self::get_vm_execution_plan).
    pub fn into_vm_execution_plan(mut self) -> VmExecutionPlan {
        self.eval
            .pull_val(&mut self.context, VALIDS().vm_execution_plan);
        self.eventually_report_failure();
        self.eval
            .into_val(VALIDS().vm_execution_plan)
            .unwrap()
            .unwrap_vm_execution_plan()
    }
}
