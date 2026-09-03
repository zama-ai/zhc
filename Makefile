.PHONY: test update-expects fmt fmt-check check bench bench-export bench-diff bench-compile bench-compile-diff analyze bench-vm vm-bench vm-throughput vm-throughput-verify vm-microbench vm-model vm-bootstrap vm-profile-model vm-profile vm-profile-remote mem-topo zhc-profile

# Wall-clock seconds the profiled bench_vm run loops for (override: make vm-profile SECS=30).
SECS ?= 20


test:
	cargo test --locked --release

brrr:
	cargo test --locked --release -- brrr

update-expects:
	cargo run --locked --bin update-expects

fmt:
	cargo +nightly fmt

fmt-check:
	cargo +nightly fmt --check

mem-topo:
	cargo run --locked -p zhc_utils --example mem_topo

check:
	RUSTFLAGS="-D warnings" cargo check --locked
	RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

bench:
	cargo run --locked --release -p zhc_bench

bench-export:
	cargo run --locked --release -p zhc_bench -- export

bench-diff:
	RUSTFLAGS="-A warnings" cargo run --locked --release -p zhc_bench -- diff

bench-compile:
	cargo run --locked --release -p zhc_bench -- compile

bench-compile-diff:
	RUSTFLAGS="-A warnings" cargo run --locked --release -p zhc_bench -- compile-diff

analyze:
	cargo run --locked --release -p zhc_bench -- analyze

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
