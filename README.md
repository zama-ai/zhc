<h3 align="center">
  <pre>
╭─────╮   ┬ ╭───╮
   ╱  │   │ │    
  ╱   ╰───╮ │    
 ╱    │   │ │    
╰───╯ ┴   ╰─────╯
</pre>
</h3>
<h3 align="center">Compiler infrastructure for encrypted computation.</h3>

<hr/>

<p align="center">
  <a href="https://crates.io/crates/zhc"><img src="https://img.shields.io/crates/v/zhc?style=flat-square" alt="crates.io"></a>
  <a href="https://docs.rs/zhc"><img src="https://img.shields.io/docsrs/zhc?style=flat-square" alt="docs.rs"></a>
  <a href="https://github.com/zama-ai/zhc/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/zhc?style=flat-square" alt="license"></a>
</p>

# What is ZHC?

ZHC is an open-source compiler toolchain for [FHE](https://en.wikipedia.org/wiki/Homomorphic_encryption) computation: it turns integer circuits into optimized instruction streams for hardware that computes directly on encrypted data. From arithmetic to silicon, ZHC optimizes FHE programs operating above the cryptography level, and ensures high performance is reached on every target. 

Built on a custom dialect-generic SSA-IR, it relies on a central `IopLang` dialect to simplify the most demanding tasks in compiler development: frontends addition, hardware-agnostic optimizations, and target-specific backend development:

<div align="center">
  <pre><sub>
      ╭───────────╮                                        ╭───────────────╮
      │   Rust    │                                   ┌───▶│      HPU      │
      │  Builder  │────┐              ┌───────┐       │    ╰───────────────╯
      ╰───────────╯    │              │       │       │                     
                       │         ╭─────────╮  │       │    ╭───────────────╮
                       ├────────▶│ IopLang │──────────┼───▶│  HPU Cluster  │
      ╭───────────╮    │         ╰─────────╯  │       │    ╰───────────────╯
      │  TFHE-rs  │    │              ▲       │       │                     
      │  Circuit  │────┘              └───────┘       │    ╭───────────────╮
      ╰───────────╯                      opt          └───▶│      CPU      │
                                                           ╰───────────────╯
</sub></pre>
</div>

This flexible architecture, enables speed at every step:
+ [Design](#circuit-design): Circuit definition, visualization and evaluation on emulated semantics. Programmatic frontends that makes sense for circuit design.
+ [Compilation](#compilation): Progressive lowering from high-level representation down to specialized hardware ISA. Target-agnostic optimizations, then hardware-aware scheduling and register allocation. All fast enough to support JIT scenarios.
+ [Execution](#execution): Optimized instruction streams for HPU, HPU clusters, and CPU targets. HPU and Multi-HPU streams can be evaluated against an accurate hardware simulator, delivering a streamlined development experience.

It underlies [tfhe-rs](https://github.com/zama-ai/tfhe-rs) backend for the [(Multi)HPU](https://github.com/zama-ai/hpu_fpga) platform, as well as the experimental Circuit-API. It focuses on the [TFHE](https://eprint.iacr.org/2021/1402.pdf) cryptosystem. It depends on no more than [a few foundational crates](https://github.com/zama-ai/zhc/Cargo.toml#L67-L77), making whole toolchain audit possible.

# Get started

To design a new FHE algorithm, add ZHC to a Rust project and follow the [tour](#a-tour-of-zhc), or look at the [examples](zhc/examples):

```shell
cargo add zhc
```

To work on the compiler itself, clone the repository and make sure you have Rust 1.85 or later (2024 edition):

```shell
git clone https://github.com/zama-ai/zhc
cd zhc
make test          # cargo test --release
make check         # warnings as errors, docs build
make big-test      # ignored tests: differential checks on every operation
make bench         # predicted latency for every operation and bit width
```

The CPU target depends on tfhe-rs and is built and tested separately:

```shell
make vm-test
```

To get up to speed quickly, start looking into [`Pipeline`](https://docs.rs/zhc_pipeline/latest/zhc_pipeline/struct.Pipeline.html) first, then [`Builder`](https://docs.rs/zhc_builder/latest/zhc_builder/struct.Builder.html), then [`IR`](https://docs.rs/zhc_ir/latest/zhc_ir/struct.IR.html). If more context is needed, the module-level design documentation included with every crate should have you covered.

# A tour of ZHC

## Circuit Design

ZHC's main frontend is the Rust [`Builder`](https://docs.rs/zhc_builder/latest/zhc_builder/struct.Builder.html) object which can be used to programmatically build arithmetic circuits. It provides access to design-oriented features:

<img align="right" width="300" src="docs/assets/rendered_ir.png" alt="IR visualization">

```rust
use zhc::prelude::*;

// 2 carry bits / 2 message bits
let bd = Builder::new(CiphertextBlockSpec(2, 2));
let pti =  bd.plaintext_input(2);
let pt =   bd.plaintext_split(pti)[0];
let ct =   bd.block_let_ciphertext(0);
let trv =  bd.block_add_plaintext(ct, pt);
let sh =   bd.block_lookup(
                trv,
                Lut1Def::custom(
                    "sh", 
                    |e| e.protect_shr(1)
                )
           );

bd.draw(IrKind::Original).open().unwrap();
```

<br clear="right">

The [`draw`](https://docs.rs/zhc_builder/latest/zhc_builder/struct.Builder.html#method.draw) method renders the underlying intermediate representation to an interactive graph suitable for visual inspection. This graph can optionally be annotated with various information such as emulation values, noise level, etc.

The API exposed by the `Builder` object offers semantics tailored to the TFHE cryptosystem. For maximum expressivity and safety, operations are available in [different flavors](https://docs.rs/zhc_crypto/latest/zhc_crypto/integer_semantics/index.html) depending on how they treat the __padding bit__. To simplify the debugging without having to go through encryption/decryption cycles, the builder object exposes an emulation layer which performs an abstract interpretation of the program and annotates the intermediate values:

```rust
bd.interpret()                               // On stdout:
    .with_inputs([pti.make_value(3)])        // ╔═══════════════════════════════════
    .dump();                                 // ║ Interpretation for : [3_pt]
                                             // ║───────────────────────────────────
                                             // ║ %0 = input_plaintext<0, 2>();
                                             // ║     %0 -> 11_pt
                                             // ║ %1 = extract_pt_block<0>(%0);
                                             // ║     %1 -> 11_ptblock
                                             // ║ %2 = let_ct_block<0>();
                                             // ║     %2 -> 0_00_00_ctblock
                                             // ║ %3 = add_pt(%2, %1);
                                             // ║     %3 -> 0_00_11_ctblock
                                             // ║ %4 = pbs<Protect, Lut1("sh")>(%3);
                                             // ║     %4 -> 0_00_01_ctblock
                                             // ╚═══════════════════════════════════
```

<details>
<summary>More on correctness</summary>
<br>
    
The invariants of the semantics are checked during abstract interpretation. This is especially useful when using the [`Builder::test_random`](https://docs.rs/zhc_builder/latest/zhc_builder/struct.Builder.html#method.test_random) method, which emulate the circuit on random inputs against a plaintext oracle. It allows the users to check their implementation at the algorithm level. Since no crypto is used for this test, the circuit can be evaluated on a large number of inputs to verify its implementation.

</details>

<details>
<summary>More on noise</summary>
<br>

Noise management is a big part of implementing FHE algorithms. It grows through operator application, but must always stay within some bounds to ensure data are not corrupted. The [`Builder::dump_noise`](https://docs.rs/zhc_builder/latest/zhc_builder/struct.Builder.html#method.dump_noise) method performs a noise analysis and displays it in a suitable format:
```rust
bd.dump_noise();                        // On stdout:
                                        // ╔════════════════════════════════════════════
                                        // ║ Noise Analysis
                                        // ║────────────────────────────────────────────
                                        // ║ @0   |  %0 = input_plaintext<0, 2>();
                                        // ║      |      %0 -> ░░░░░░░░░░░░   0%
                                        // ║ @1   |  %1 = extract_pt_block<0>(%0);
                                        // ║      |      %1 -> ░░░░░░░░░░░░   0%
                                        // ║ @2   |  %2 = let_ct_block<0>();
                                        // ║      |      %2 -> ░░░░░░░░░░░░   0%
                                        // ║ @3   |  %3 = add_pt(%2, %1);
                                        // ║      |      %3 -> ░░░░░░░░░░░░   0%
                                        // ║ @4   |  %4 = pbs<Protect, Lut1("sh")>(%3);
                                        // ║      |      %4 -> ▓░░░░░░░░░░░   8% (fresh)
                                        // ╚════════════════════════════════════════════
```

Note that ZHC also checks the noise during compilation to ensure that noise-incorrect circuits can never reach the end of the pipeline and produce executable artifacts.

</details>

## Compilation

<img align="left" width="160" src="docs/assets/restricted_pipeline.png" alt="Pipeline">

The compiler itself is managed via the [`Pipeline`](https://docs.rs/zhc_pipeline/latest/zhc_pipeline/struct.Pipeline.html) object. It provides a lazy, query-based compilation of all the artifacts, and is itself based on a [ZHC Dialect](zhc_langs/src/pipelinelang/mod.rs). This mechanism ensures a single source of truth ever exist for every artifact derivation. The pipeline, restricted to the HPU backend is represented on the left. 

Every intermediate artifacts is cached inside the pipeline, and can be pulled for inspection with `get_*` methods. This makes for a faster debugging experience, and prevents artifacts to be recompiled when not strictly necessary. On the other hand, modifying a pipeline input has the immediate effect of invalidating every transitively dependent artifacts, for a predictable user experience.

Here is an example of pulling the text assembly listing for HPU via [`Pipeline::get_hpu_assembly`](https://docs.rs/zhc_pipeline/latest/zhc_pipeline/struct.Pipeline.html#method.get_hpu_assembly) when compiling the 16-bit addition circuit:
```rust
let mut pl = Pipeline::new()
    .with_builder(bd)
    .with_hpu_config(Default::default());
pl.get_hpu_assembly().open().unwrap();
```

<br clear="left">

```text
; !preamble {
; [signature]
; (Ciphertext<16, 2, 2>, Ciphertext<16, 2, 2>) -> Ciphertext<16, 2, 2>
; [lut]
; ManyCarryMsg: [0, 1, 2, 3, 0, 1, 2, 3, 0, 0, 0, 0, 1, 1, 1, 1]
; ExtractPropGroup0: [0, 0, 0, 1, 2, 2, 2, 2, 0, 0, 0, 1, 2, 2, 2, 2]
; ...
; }
LD R0 TS[1].3
LD R1 TS[0].3
ADD R0 R1 R0
...
PBS R4 R3 PbsExtractPropGroup0
PBS_ML2 R6 R1 PbsManyCarryMsg
PBS R5 R2 PbsExtractPropGroup1
```


<details>
<summary>More on verification</summary>
<br>
    
Every pass of the pipeline (including scheduling and register allocation) is verified, on [_the complete IOP library_](https://docs.rs/zhc/latest/zhc/prelude/compat/enum.Iop.html), for precisions ranging from 2 to 128 bits. The current approach used for verification is differential evaluation. For each circuit, the IRs before and after the pass are both emulated (each dialect having its own emulator) on a large number of inputs, and the results are asserted bitwise equal.

</details>

<details>
<summary>More on compilation time</summary>
<br>
    
Compilation time is on the order of a few microseconds per instruction (well below the time to execute them on any platform) so compilation can be pipelined with execution, staying off the critical path. This makes Just-In-Time compilation of FHE programs possible: operation graphs produced at runtime can be compiled as whole programs on-the-fly, and the scheduling and batching gains largely repay the compile time.

</details>

## Execution

<img align="right" width="300" src="docs/assets/trace.png" alt="Perfetto trace">

HPU board / cluster may not be readily available for development, impairing the ability of designers to iterate on their algorithms. Hopefully, ZHC includes a discrete-time simulator of the HPU and HPU-cluster targets, which can be used to get an accurate estimation of the HPU behavior on the compiled streams. Even better, [Perfetto](https://perfetto.dev/) traces of the execution can be extracted from the simulator right on the [`Pipeline::get_hpu_trace`](https://docs.rs/zhc_pipeline/latest/zhc_pipeline/struct.Pipeline.html#method.get_hpu_trace) method. 

<br clear="right">

For even faster feedback loops, simpler aggregate metrics can be accessed with [`Pipeline::get_hpu_metrics`](https://docs.rs/zhc_pipeline/latest/zhc_pipeline/struct.Pipeline.html#method.get_hpu_metrics). The simulated latency is decomposed into key components: the time an ideal schedule would spend bootstrapping, the time lost to under-filled batches, and the time the PBS execution unit spends waiting:

```rust
pl.get_hpu_metrics().dump();
// On stdout:
// ╔══════════════════════════════════════════════════════════════════
// ║ HPU Metrics
// ║──────────────────────────────────────────────────────────────────
// ║   Latency          : 3224.84 µs
// ║     PBS ideal      : 1554.46 µs  (lower bound)
// ║     Batch overhead : 1554.46 µs  (under-filled batches)
// ║     Starvation     : 115.92 µs  (PE waiting for work)
// ║   Slot occupancy   : 43.8%
// ║   Timeout launches : 0 / 4 batches
// ║──────────────────────────────────────────────────────────────────
// ║   Batch Size Histogram:
// ║     4 │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▓● (2)
// ║     5 │░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▓● (1)
// ║     8 │░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▓● (1)
// ╚══════════════════════════════════════════════════════════════════
```

FHE is special for its large imbalance in operation latency: the ratio of __pbs__ to __linear__ operations runtimes (the two broad categories) can be in the hundred to the thousand. Approaches assuming homogeneous latencies are either unsuitable in this regime, or plainly inapplicable depending on the target. The various targetted platforms (HPUs, HPU clusters, and experimentally CPUs) all bring a different solution to the above-mentioned problem. All with a different set of constraints and challenges. To ensure peak performance can be reached for every platform, each one is the target of a separate branch of the backend.



<details>
<summary>More on HPU scheduling</summary>
<br>

As tasted in the assembly shown above, HPU follows a standard processor architecture with ciphertext register file, dedicated processing elements, and accompanying ciphertext-level ISA. PBS processing is particular in that it operates on batches of ciphertexts, with a latency more or less independent of the batch size. Our scheduler mixes as-late-as-possible list-scheduling with a lightweight simulation, to both schedule and batch PBS operations together in one pass.

</details>

<details>
<summary>More on Multi-HPU</summary>
<br>

To throw more compute power at a circuit, HPUs can be interconnected into a cluster, allowing fast transfers of ciphertexts between boards. Large computations can then be spread over more processing elements, lowering the execution latency. Thanks to an automatic partitioning mechanism, no manual intervention is needed to balance the workload over the boards.

A task-level parallelism framework allows to coarsen an [`IopLang`](https://docs.rs/zhc_langs/latest/zhc_langs/ioplang/index.html) circuit is coarsened into a pbs task graph. This DAG of tasks is then mapped onto the cluster, accounting for the dependencies, the locality of the data and the preferred concentration of work. The mapping is materialized back into `IopLang` with explicit transfers between boards, and replication of cheap work where it makes sense.

</details>

<details>
<summary>More on CPU</summary>
<br>

CPU machines (beefy multi-sockets and small laptops alike) can be targeted by ZHC via the `zhc_vm` crate. It exposes a topology-aware multi-threaded virtual machine for a custom FHE bytecode. Requiring no other dependency than `tfhe-rs` for the implementation of the cryptographic operators, `zhc_vm` is a small, self-contained runtime built for maximum performance.

ZHC leverages its task-level parallelism framework to derive a [`VmExecutionPlan`](https://docs.rs/zhc_pipeline/latest/zhc_pipeline/struct.VmExecutionPlan.html): a static schedule of the bytecode executed by every thread of the machine, plus a lock table ensuring the safety of the execution. Nothing is decided at runtime, which removes the overhead and complexity of the dynamic work-stealing schedulers commonly used.

</details>

# Repository layout

ZHC is made of many crates, ensuring fast from-sources compilation. Here is a map of those:

| Crate | Role |
|---|---|
| [`zhc`](zhc) | Facade crate. Re-exports everything and connects the builder to the pipeline. |
| [`zhc_ir`](zhc_ir) | Dialect-generic IR, analyses, passes, evaluation, translation, visualization. |
| [`zhc_langs`](zhc_langs) | The dialects: `ioplang`, `hpulang`, `doplang`, `vmlang`, `pipelinelang`. |
| [`zhc_langs_macro`](zhc_langs_macro) | Macro generating the pipeline dialect from a function body. |
| [`zhc_builder`](zhc_builder) | Circuit builder, integer operation library, interpreter, randomized tests. |
| [`zhc_crypto`](zhc_crypto) | Emulated TFHE integer semantics and lookup tables. |
| [`zhc_pipeline`](zhc_pipeline) | The lazy compilation pipeline: lowering, scheduling, allocation, code generation, metrics. |
| [`zhc_sim`](zhc_sim) | Discrete-event simulator, HPU model, multi-HPU model, tracing. |
| [`zhc_config`](zhc_config) | HPU, multi-HPU and VM configurations, with physical parameter presets. |
| [`zhc_vm`](zhc_vm) | Multi-threaded CPU runtime executing VM plans with tfhe-rs. |
| [`zhc_profiling`](zhc_profiling) | Host-side profiling spans with pluggable backends. |
| [`zhc_pipeline_correctness`](zhc_pipeline_correctness) | Snapshot and differential tests for every pass. |
| [`zhc_pipeline_correctness_macro`](zhc_pipeline_correctness_macro) | Test matrix macro over operations and bit widths. |
| [`zhc_bench`](zhc_bench) | Latency benchmarks across operations and commits. |
| [`zhc_cli`](zhc_cli) | Command-line tools. `dop_fmt` converts, checks, interprets and profiles device programs. |
| [`zhc_utils`](zhc_utils) | Stores, small vectors, iterators, topology detection, trace and file helpers. |
| [`zhc_utils_macro`](zhc_utils_macro) | Derive and attribute macros used across the workspace. |

# Contributing

Contributions are welcome. Open an issue or a pull request on GitHub. Before submitting, run `make fmt`, `make check` and `make test`.

Security issues should be reported privately to security@zama.ai, see [SECURITY.md](SECURITY.md).

# License

ZHC is released under the BSD-3-Clause-Clear license. See [LICENSE](LICENSE).
