# zhc_leanrt_sys

Part of [ZHC](https://github.com/zama-ai/zhc), the compiler toolchain for FHE computation.

This crate builds the runtime of [Lean 4](https://github.com/leanprover/lean4) from the Lean sources vendored in `vendor/`. The runtime is necessary to execute Lean code that was proven, then transpiled to C by Lean. It is built without GMP, mimalloc and OpenSSL. The crate has no Rust API. It only links the static library `leanrt`.

## License

Released under the BSD-3-Clause-Clear license. See [LICENSE](https://github.com/zama-ai/zhc/blob/main/LICENSE). The Lean sources in `vendor/` are distributed under the Apache-2.0 license, which is reproduced in [vendor/LICENSE](vendor/LICENSE).
