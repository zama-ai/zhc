# zhc_iops_r2l

Part of [ZHC](https://github.com/zama-ai/zhc), the compiler toolchain for FHE computation.

This crate lets Rust call the integer operations that are written and proven in Lean (`zhc_lean`). It compiles the C code that the Lean compiler emits for those operations, vendored in `vendor/`, and it exposes one Rust function for each exported operation. Use `zhc_iops` for the public API.

## License

Released under the BSD-3-Clause-Clear license. See [LICENSE](https://github.com/zama-ai/zhc/blob/main/LICENSE). The C code in `vendor/ir` contains code that the Lean compiler specialised from the Lean `Init` library, which is distributed under the Apache-2.0 license. That license is reproduced in [vendor/LICENSE](vendor/LICENSE).
