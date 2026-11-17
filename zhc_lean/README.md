# zhc_lean

Part of [ZHC](https://github.com/zama-ai/zhc), the compiler toolchain for FHE computation.

This Lean 4 package contains the integer operations of ZHC that are written and proven in Lean. Lean transpiles them to C code, which is vendored in `zhc_iops_r2l` and exposed to Rust by `zhc_iops`.

The Lean release is set in `lean-toolchain`. Run `make lean-test` to run the tests, and `make lean-update-ffi` to update the vendored code after a change.

## License

Released under the BSD-3-Clause-Clear license. See [LICENSE](https://github.com/zama-ai/zhc/blob/main/LICENSE).
