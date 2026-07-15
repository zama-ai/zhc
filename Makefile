.PHONY: test update-expects fmt fmt-check check bench bench-export bench-diff analyze bench-vm vm-bench vm-throughput vm-throughput-verify vm-microbench vm-model vm-bootstrap vm-profile-model vm-profile vm-profile-remote mem-topo

# Wall-clock seconds the profiled bench_vm run loops for (override: make vm-profile SECS=30).
SECS ?= 20


test:
	cargo test --release $(if $(F),-- $(F))

update-expects:
	cargo run --bin update-expects

fmt:
	cargo +nightly fmt

fmt-check:
	cargo +nightly fmt --check

check:
	RUSTFLAGS="-D warnings" cargo check
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

bench:
	cargo run --release -p zhc_bench $(if $(F),-- $(F))

bench-export:
	cargo run --release -p zhc_bench -- export

bench-diff:
	RUSTFLAGS="-A warnings" cargo run --release -p zhc_bench -- diff

analyze:
	cargo run --release -p zhc_bench -- analyze $(if $(F),$(F))

vm-bench:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench -p zhc_vm --bench vm --profile profiling

vm-throughput:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench -p zhc_vm --bench throughput --profile profiling

vm-throughput-verify:
	VM_VERIFY=1 $(if $(CORES),VM_CORES=$(CORES)) \
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench -p zhc_vm --bench throughput --profile profiling

vm-microbench:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 run --profile profiling -p zhc_vm --example microbench

vm-profile-throughput:
	RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
	cargo +nightly-2026-04-22 bench -p zhc_vm --bench throughput --profile profiling --features profiling

vm-profile:
	rm -rf bench_vm.trace
	BENCH_BIN=$$(RUSTFLAGS="-C target-cpu=native -A warnings" CARGO_PROFILE_RELEASE_LTO=fat \
		cargo +nightly-2026-04-22 bench -p zhc_vm --bench vm --profile profiling --no-run \
		--message-format=json 2>/dev/null \
		| grep -o '"executable":"[^"]*/vm-[^"]*"' | tail -1 | sed 's/.*:"//;s/"$$//'); \
	xcrun xctrace record --template "CPU Counters" \
		--output bench_vm.trace \
		--launch -- $$BENCH_BIN --bench --profile-time $(SECS)
	open bench_vm.trace
