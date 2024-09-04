# cargo install cargo-watch
dev:
	cargo watch -x check -x test -x run

fmt:
	cargo fmt

check: fmt
	cargo check

PATTERN?="update_db"
test: check
	cargo test ${PATTERN}

test-verbose: check
	cargo test -- --nocapture

# cargo install cargo-tarpaulin
cov:
	cargo tarpaulin --ignore-tests

# rustup component add clippy
lint-check: check
	cargo clippy -- -D warnings

# rustup component add rustfmt, for CI pipeline
fmt-check:
	cargo fmt -- --check

# cargo install cargo-audit
audit:
	cargo audit

# cargo install cargo-deny
# equivalent to cargo-audit
deny-audit:
	cargo deny

build:
	cargo build

# cargo install cargo-asm
asm:
	cargo asm

show-todos:
	grep -rni ./src -e 'todo'
	grep -rni ./tests -e 'todo'

# cargo install cargo-udeps
scan:
	cargo +nightly udeps
