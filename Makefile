.PHONY: test update-expects fmt fmt-check check

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
