all: test clippy fmt run

test:
	cargo test

clippy:
	cargo clippy

fmt:
	cargo fmt

run:
	cargo run
