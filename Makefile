test:
	cargo build
	cargo test --verbose

run:
	cargo run

dev: test
	cargo run

test-init-assets:
	mkdir -p ./tmp
	mkdir -p ./assets
	mkdir -p ./exports

	rm -rf ./tmp/temp_repo
	rm -rf ./assets/*

	git clone --depth 1 https://github.com/pfalcon/canterbury-corpus ./tmp/temp_repo
	rsync -av --progress ./tmp/temp_repo/ ./assets --exclude .git
	rm -rf ./tmp/temp_repo

PROFILE ?= dev

test-coverage:
	cargo tarpaulin --exclude-files src/main.rs --all-features --profile=${PROFILE}