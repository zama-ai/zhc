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

Built on a custom dialect-generic SSA-IR, it relies on a central `IopLang` dialect to simplify the most demanding tasks in compiler development: adding frontends, writing hardware-agnostic optimizations, and developing target-specific backend:

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

This flexible architecture enables speed at every step:
+ [Design](docs/TOUR.md#circuit-design): Circuit definition, visualization and evaluation on emulated semantics. Programmatic frontends that make sense for circuit design.
+ [Compilation](docs/TOUR.md#compilation): Progressive lowering from high-level representation down to specialized hardware ISA. Target-agnostic optimizations, then hardware-aware scheduling and register allocation. All fast enough to support JIT scenarios.
+ [Execution](docs/TOUR.md#execution): Optimized instruction streams for HPU, HPU clusters, and CPU targets. HPU and Multi-HPU streams can be evaluated against an accurate hardware simulator, delivering a streamlined development experience.

It underlies the [tfhe-rs](https://github.com/zama-ai/tfhe-rs) backend for the [(Multi)HPU](https://github.com/zama-ai/hpu_fpga) platform, as well as the experimental Circuit-API. It focuses on the [TFHE](https://eprint.iacr.org/2021/1402.pdf) cryptosystem. It depends on no more than [a few foundational crates](https://github.com/zama-ai/zhc/blob/main/Cargo.toml), making whole-toolchain audit possible.

# Get started

To design a new FHE algorithm, add ZHC to a Rust project and follow the [tour](docs/TOUR.md), or look at the [examples](zhc/examples):

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

# Repository layout

ZHC is made of many crates, ensuring fast from-source compilation. Here is a map of those:

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
| [`zhc_validation`](zhc_validation) | Snapshot and differential tests for every pass. |
| [`zhc_validation_macro`](zhc_validation_macro) | Test matrix macro over operations and bit widths. |
| [`zhc_bench`](zhc_bench) | Latency benchmarks across operations and commits. |
| [`zhc_cli`](zhc_cli) | Command-line tools. `dop_fmt` converts, checks, interprets and profiles device programs. |
| [`zhc_utils`](zhc_utils) | Stores, small vectors, iterators, topology detection, trace and file helpers. |
| [`zhc_utils_macro`](zhc_utils_macro) | Derive and attribute macros used across the workspace. |

# License

ZHC is released under the BSD-3-Clause-Clear license. See [LICENSE](LICENSE).
