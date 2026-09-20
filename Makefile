.PHONY: all-test all-big-test all-fmt-check all-check \
	zhc-test zhc-big-test zhc-brrr zhc-update-expects zhc-fmt zhc-fmt-check zhc-mem-topo zhc-check \
	zhc-bench zhc-bench-export zhc-bench-diff zhc-bench-compile zhc-analyze zhc-profile \
	lean-extern-libs lean-update-ffi lean-check-ffi lean-test \
	vm-test vm-check vm-bench vm-throughput vm-throughput-verify vm-microbench vm-profile-throughput vm-profile

# Wall-clock seconds the profiled bench_vm run loops for (override: make vm-profile SECS=30).
SECS ?= 20

all-test: zhc-test lean-test

all-big-test:
	RUST_MIN_STACK=268435456 cargo test --release -- --ignored

all-fmt-check: zhc-fmt-check

all-check: zhc-check lean-check-ffi

lean-extern-libs:
	cd zhc_lean && lake build libzhc_lean_shim:shared libzhc_iops_l2r:shared

lean-update-ffi: lean-extern-libs
	cargo run --locked -p zhc_scripts --example generate_lean

lean-check-ffi: lean-extern-libs
	cargo test --locked -p zhc_scripts --example generate_lean -- --ignored --nocapture generated_files_are_up_to_date

lean-test: lean-extern-libs
	cd zhc_lean && lake test

zhc-test:
	cargo test --locked --release

zhc-big-test:
	cargo test --release -- --ignored

zhc-brrr:
	cargo test --locked --release -- brrr

zhc-update-expects:
	cargo run --locked --bin update-expects

zhc-fmt:
	cargo +nightly fmt

zhc-fmt-check:
	cargo +nightly fmt --check

zhc-mem-topo:
	cargo run --locked -p zhc_utils --example mem_topo

zhc-check:
	RUSTFLAGS="-D warnings" cargo check --locked
	RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

zhc-bench:
	cargo run --locked --release -p zhc_bench

zhc-bench-export:
	cargo run --locked --release -p zhc_bench -- export

zhc-bench-diff:
	RUSTFLAGS="-A warnings" cargo run --locked --release -p zhc_bench -- diff

zhc-bench-compile:
	cargo run --locked --release -p zhc_bench -- compile

zhc-analyze:
	cargo run --locked --release -p zhc_bench -- analyze

zhc-profile:
	rm -rf zhc_profile_pipeline.trace
	EXAMPLE_BIN=$$(RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo build --locked -p zhc --example profile_pipeline --profile profiling \
		--message-format=json 2>/dev/null \
		| grep -o '"executable":"[^"]*/profile_pipeline[^"]*"' | tail -1 | sed 's/.*:"//;s/"$$//'); \
	xcrun xctrace record --template "Time Profiler (High Freq)" \
		--output zhc_profile_pipeline.trace \
		--launch -- $$EXAMPLE_BIN
	open zhc_profile_pipeline.trace

vm-test:
	cargo test --locked -p zhc_vm --release

vm-check:
	RUSTFLAGS="-D warnings" cargo check --locked -p zhc_vm
	RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p zhc_vm

vm-bench:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench --locked -p zhc_vm --bench vm --profile profiling

vm-throughput:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench --locked -p zhc_vm --bench throughput --profile profiling

vm-throughput-verify:
	VM_VERIFY=1 \
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench --locked -p zhc_vm --bench throughput --profile profiling

vm-microbench:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 run --locked --profile profiling -p zhc_vm --example microbench

vm-profile-throughput:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
	cargo +nightly-2026-04-22 bench --locked -p zhc_vm --bench throughput --profile profiling --features profiling

vm-profile:
	rm -rf bench_vm.trace
	BENCH_BIN=$$(RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench --locked -p zhc_vm --bench vm --profile profiling --no-run \
		--message-format=json 2>/dev/null \
		| grep -o '"executable":"[^"]*/vm-[^"]*"' | tail -1 | sed 's/.*:"//;s/"$$//'); \
	xcrun xctrace record --template "CPU Counters" \
		--output bench_vm.trace \
		--launch -- $$BENCH_BIN --bench --profile-time $(SECS)
	open bench_vm.trace
