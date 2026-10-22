# zhc_abc

Part of [ZHC](https://github.com/zama-ai/zhc), the compiler toolchain for FHE computation.

This crate is a safe Rust interface to the combinational equivalence checker of [ABC](https://github.com/berkeley-abc/abc). It lets you build two and-inverter graphs in memory, with structural hashing, and ask whether they compute the same Boolean function. A negative answer comes with a counterexample. It is internal to the workspace and not published.

## License

Released under the BSD-3-Clause-Clear license. See [LICENSE](https://github.com/zama-ai/zhc/blob/main/LICENSE).
