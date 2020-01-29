all: check
.PHONY: all

clean:
	@cargo clean
.PHONY: clean

build:
	@cargo build
.PHONY: build

doc:
	@cargo doc --all-features
.PHONY: doc

check: style lint test
.PHONY: check

style:
	@rustup component add rustfmt --toolchain stable 2> /dev/null
	cargo +stable fmt -- --check
.PHONY: style

format:
	@rustup component add rustfmt --toolchain stable 2> /dev/null
	cargo +stable fmt
.PHONY: format

lint:
	@rustup component add clippy --toolchain stable 2> /dev/null
	cargo +stable clippy --all-features --tests --all --examples -- -D clippy::all
.PHONY: lint

test: checkall testall
.PHONY: test

checkall:
	cargo check --all-features
	cargo check --no-default-features
.PHONY: checkall

testall:
	cargo test --all-features --all
	cargo test --no-default-features --all
	yarn
	yarn test
.PHONY: testall
