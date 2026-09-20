# zhc_leaninit_sys

Part of [ZHC](https://github.com/zama-ai/zhc), the compiler toolchain for FHE computation.

This crate builds the `Init` library of [Lean 4](https://github.com/leanprover/lean4) transpiled from Lean to C code by Lean itself, and then vendored in `vendor/`. The init library is necessary to execute Lean code transpiled to C, that depends on the Init Lean library. The crate has no Rust API. It only links the static library `leaninit`.

## License

Released under the BSD-3-Clause-Clear license. See [LICENSE](https://github.com/zama-ai/zhc/blob/main/LICENSE). The C code in `vendor/` comes from the Lean sources, which are distributed under the Apache-2.0 license. That license is reproduced in [vendor/LICENSE](vendor/LICENSE).
