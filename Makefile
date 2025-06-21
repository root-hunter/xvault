WORKSPACE ?= xvault

test:
	cargo build -p ${WORKSPACE}
	cargo test -p ${WORKSPACE} --verbose

run:
	cargo run -p ${WORKSPACE}

dev: test
	cd ${WORKSPACE}; cargo run 

test-init-assets:
	mkdir -p ./${WORKSPACE}/tmp
	mkdir -p ./${WORKSPACE}/assets
	mkdir -p ./${WORKSPACE}/exports

	rm -rf ./${WORKSPACE}/tmp/temp_repo
	rm -rf ./${WORKSPACE}/assets/*

	git clone --depth 1 https://github.com/pfalcon/canterbury-corpus ./${WORKSPACE}/tmp/temp_repo
	rsync -av --progress ./${WORKSPACE}/tmp/temp_repo/ ./${WORKSPACE}/assets --exclude .git
	rm -rf ./${WORKSPACE}/tmp/temp_repo

PROFILE ?= dev

test-coverage:
	cargo tarpaulin \
	--exclude-files xvault/src/main.rs */tests/* \
	--all-features --profile=${PROFILE}