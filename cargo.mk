##
##===============================================================================
##make cargo-*
cargo-help:### 	cargo-help
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
##===============================================================================
cargo-release-all:### 	cargo-release-all
## 	cargo-release-all 	recursively cargo build --release
	for t in */Cargo.toml;  do echo $$t; cargo b -r -vv --manifest-path $$t; done
	for t in ffi/*/Cargo.toml;  do echo $$t; cargo b -r -vv --manifest-path $$t; done
## 	:
##===============================================================================
cargo-clean-all:### 	cargo-clean-all - clean release artifacts
## 	cargo-clean-all 	recursively cargo clean --release
	for t in */Cargo.toml;  do echo $$t; cargo clean --release -vv --manifest-path $$t; done
## 	:
##===============================================================================
cargo-publish-all:### 	cargo-publish-all
## 	cargo-publish-all 	recursively publish rust projects
## 	CARGO_REGISTRY_TOKEN=<token> make cargo-publish-all
	for t in Cargo.toml;  do echo $$t; cargo publish -vv --manifest-path $$t; done
	#for t in src/bin/**/Cargo.toml;  do echo $$t; cargo publish -vv --manifest-path $$t; done
## 	:

##===============================================================================
cargo-install-bins:### 	cargo-install-bins
## 	cargo-install-all 	recursively cargo install -vv $(SUBMODULES)
## 	*** cargo install -vv --force is NOT used.
## 	*** cargo install -vv --force --path <path>
## 	*** to overwrite deploy cargo.io crates.
	export RUSTFLAGS=-Awarning;  for t in $(SUBMODULES); do echo $$t; cargo install --bins --path  $$t -vv 2>/dev/null || echo ""; done
	#for t in $(SUBMODULES); do echo $$t; cargo install -vv gnostr-$$t --force || echo ""; done

##===============================================================================
cargo-b:cargo-build### 	cargo b
cargo-build:### 	cargo build
## 	cargo-build q=true
	@. $(HOME)/.cargo/env
	@RUST_BACKTRACE=all cargo b $(QUIET)
## 	:
##===============================================================================
cargo-i:cargo-install
cargo-install:### 	cargo install --path .
	@. $(HOME)/.cargo/env
	@cargo install --force --path .
	@cargo install --force --path ./src/bin/lightning-search_dashboard
	@cargo install --force --path ./src/bin/mempool-space_dashboard
##===============================================================================
cargo-bench:### 	cargo-bench
	@. $(HOME)/.cargo/env
	@cargo bench
##===============================================================================
cargo-br:cargo-build-release### 	cargo-br
## 	cargo-br q=true
## 	:
cargo-build-release:### 	cargo-build-release
## 	cargo-build-release q=true
## 	:
	@. $(HOME)/.cargo/env
	@cargo b --release $(QUIET)
##===============================================================================
cargo-c:cargo-check
cargo-check:### 	cargo-check
## 	cargo-check
## 	:
	@. $(HOME)/.cargo/env
## 	cargo check --manifest-path ./src/bin/mempool-space_dashboard/Cargo.toml
	cargo check --manifest-path ./src/bin/mempool-space_dashboard/Cargo.toml
## 	cargo check --manifest-path ./src/bin/lightning_search/Cargo.toml
	cargo check --manifest-path ./src/bin/lightning-search_dashboard/Cargo.toml
## 	cargo check --manifest-path ./Cargo.toml
	cargo check --manifest-path ./Cargo.toml
## 	:
##===============================================================================
cargo-clippy:### 	cargo-clippy
	@cargo clippy --fix --bins --allow-dirty --allow-staged || true
## 	:
##===============================================================================
cargo-docs:cargo-doc
cargo-d:cargo-doc
docs:cargo-doc
cargo-doc:### 	cargo-doc
## 	:
	@. $(HOME)/.cargo/env
	@cargo test --doc $(VERBOSE)
	@cargo doc --no-deps --all-features $(VERBOSE)
	@cat src/lib.rs | sed 's/\/\/! //g' | sed 's/\/\/!//g' | sed 's/\/\/\//### /g' | sed 's/\/\///g' > README.temp
	@cat README.temp | sed 's/\\-/-/g' > README.temp2
#cat LIB.temp2
	@cat README.temp2 | sed 's/unwrap_used/unwrap\\_used/g' > README.md
	git diff doc/README.md

cargo-fmt:### cargo +nightly fmt
	cargo +nightly fmt
cargo-sort:### cargo +nightly sort
	cargo +nightly sort

##===============================================================================
cargo-t:cargo-test
cargo-test:cargo-clippy### 	cargo-test
##cargo-test
## 	:
	@. $(HOME)/.cargo/env
	FORCE=--force $(MAKE) cargo-i
##cargo test --bin lightning-search
	cargo test --manifest-path src/bin/lightning-search_dashboard/Cargo.toml
##cargo test --bin mempool-space_dashboard
	cargo test --manifest-path src/bin/mempool-space_dashboard/Cargo.toml
##cargo test --bins
	cargo test --bins
##cargo test
	cargo test -- --nocapture
cargo-test-ignored:### 	cargo-test-ignored
##cargo-test-ignore
	@. $(HOME)/.cargo/env
##cargo test -- --ignored
	@cargo test -- --ignored
## 	:
# ##===============================================================================
# cargo-report:### 	cargo-report
# ##cargo-report
# 	@. $(HOME)/.cargo/env
# 	cargo report future-incompatibilities --id 1
##===============================================================================
cargo-dist:### 	cargo-dist -h
##cargo-dist 	cargo-dist -h
	cargo dist -h
cargo-dist-init:### 	cargo-dist-init
##cargo-dist-init
##	RUSTFLAGS="--cfg tokio_unstable" cargo dist init
	RUSTFLAGS="--cfg tokio_unstable" cargo dist init
cargo-dist-build:### 	cargo-dist-build
##cargo-dist-build
##	RUSTFLAGS="--cfg tokio_unstable" cargo dist build --artifacts=global
	RUSTFLAGS="--cfg tokio_unstable" cargo dist build --artifacts=global
cargo-dist-generate:### 	cargo-dist-generate
##cargo-dist-generate
##	RUSTFLAGS="--cfg tokio_unstable" cargo dist generate
	RUSTFLAGS="--cfg tokio_unstable" cargo dist generate
cargo-dist-manifest-global:### 	cargo dist manifest --artifacts=global
##cargo-dist-manifest-global
##	RUSTFLAGS="--cfg tokio_unstable" cargo dist manifest --artifacts=global
	RUSTFLAGS="--cfg tokio_unstable" cargo dist manifest --artifacts=global
cargo-dist-plan:### 	cargo-dist-plan
##cargo-dist-plan
##	RUSTFLAGS="--cfg tokio_unstable" cargo dist plan
	RUSTFLAGS="--cfg tokio_unstable" cargo dist plan

# vim: set noexpandtab:
# vim: set setfiletype make
