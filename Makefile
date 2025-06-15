test:
	cargo test --verbose

run:
	cargo run

dev: test
	cargo run