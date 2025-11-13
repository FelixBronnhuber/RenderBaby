ci: fmt clippy test build doc

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy -- -D warnings

test:
	cargo test --all --verbose

build:
	cargo build --release

doc:
	cargo doc --no-deps --document-private-items

.PHONY: ci fmt clippy test build doc

