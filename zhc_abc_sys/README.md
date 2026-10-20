# zhc_abc_sys

Part of [ZHC](https://github.com/zama-ai/zhc), the compiler toolchain for FHE computation.

This crate builds the [ABC](https://github.com/berkeley-abc/abc) logic synthesis and verification system from the sources vendored as a git submodule in `abc/`, and exposes the handful of C entry points needed to build and-inverter graphs in memory and run combinational equivalence checking on them. It is internal to the workspace and not published. Use `zhc_abc` for the safe API.

The ABC sources are pinned to a fixed commit by the submodule. Run `git submodule update --init` before building. The build needs `make` and a C and C++ compiler.

## License

Released under the BSD-3-Clause-Clear license. See [LICENSE](https://github.com/zama-ai/zhc/blob/main/LICENSE). ABC itself is distributed under its own permissive license, see `abc/copyright.txt`.
