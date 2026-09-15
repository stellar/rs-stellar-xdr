.PHONY: all test build doc install readme fuzz fuzz-reduce fuzz-corpus-json watch generate generate-files clean fmt publish

export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

CARGO_HACK_ARGS=--feature-powerset --exclude-features default --group-features base64,serde,arbitrary,hex,rand

CARGO_DOC_ARGS?=--open

all: build test

test:
	cargo hack test $(CARGO_HACK_ARGS)

build: generate
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets
	cargo clippy --no-default-features --release --target wasm32v1-none

doc:
	cargo test --doc --all-features
	RUSTDOCFLAGS="--cfg docs" cargo +nightly doc --package stellar-xdr --all-features $(CARGO_DOC_ARGS)

install:
	cargo install --locked --path . --force --features cli

readme:
	cargo +nightly rustdoc -- -Zunstable-options -wjson \
		&& echo '# stellar-xdr' \
		| cat target/doc/stellar_xdr.json \
		| jq -r '"# stellar-xdr\n\n" + .index[.root|tostring].docs' \
		> README.md

# Run the fuzz corpus.
# Leak detection is disabled because the fuzzer tests the const impls that
# contain &'static data and so their Arbitrary impls explicitly leak.
fuzz:
	ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run spec-entry -- -rss_limit_mb=0 -max_len=10240 -runs=0

# Shrinks each corpus to the smallest set of inputs with the same coverage.
fuzz-reduce:
	ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz cmin spec-entry -- -rss_limit_mb=0

fuzz-coverage:
	rustup component add --toolchain nightly llvm-tools-preview
	@RUST_LLVM_COV=$$(find $$(rustc +nightly --print sysroot) -name llvm-cov) && \
	RUST_TARGET_TRIPLE=$$(rustc +nightly -vV | sed -n 's|host: ||p') && \
	for TARGET in $$(cargo +nightly fuzz list); do \
		echo "=== Coverage for $$TARGET ===" && \
		cargo +nightly fuzz coverage $$TARGET && \
		$$RUST_LLVM_COV report \
			-instr-profile=fuzz/coverage/$$TARGET/coverage.profdata \
			-object target/$$RUST_TARGET_TRIPLE/coverage/$$RUST_TARGET_TRIPLE/release/$$TARGET \
			--ignore-filename-regex ".cargo/registry" && \
		$$RUST_LLVM_COV show \
			-instr-profile=fuzz/coverage/$$TARGET/coverage.profdata \
			-object target/$$RUST_TARGET_TRIPLE/coverage/$$RUST_TARGET_TRIPLE/release/$$TARGET \
			--ignore-filename-regex ".cargo/registry" \
			--format=html \
			> coverage-$$TARGET.html; \
	done

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

generate: generate-files xdr-version xdr-json xdr-definitions-json

CUSTOM_DEFAULT_IMPL=TransactionEnvelope
CUSTOM_STR_IMPL=PublicKey,AccountId,ContractId,MuxedAccount,MuxedAccountMed25519,SignerKey,SignerKeyEd25519SignedPayload,NodeId,ScAddress,AssetCode,AssetCode4,AssetCode12,ClaimableBalanceId,PoolId,MuxedEd25519Account,Int128Parts,UInt128Parts,Int256Parts,UInt256Parts

generate-files: src/generated.rs

src/generated.rs: $(sort $(wildcard xdr/*.x))
	cargo run --manifest-path xdr-generator-rust/generator/Cargo.toml -- \
		$(addprefix --input ,$(sort $(wildcard xdr/*.x))) \
		--output $@ \
		--custom-default $(CUSTOM_DEFAULT_IMPL) \
		--custom-str $(CUSTOM_STR_IMPL)
	rustfmt $@ && find src/generated -name '*.rs' | xargs rustfmt

xdr-version: $(wildcard .git/modules/xdr/**/*) $(wildcard xdr/*.x)
	git submodule status -- xdr | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr-version

xdr-json: src/generated.rs
	mkdir -p xdr-json
	cargo run --features cli -- types schema-files --out-dir xdr-json

xdr-definitions-json: $(sort $(wildcard xdr/*.x))
	mkdir -p xdr-definitions-json
	cargo run --manifest-path xdr-generator-rust/generator-definitions-json/Cargo.toml -- \
		$(addprefix --input ,$(sort $(wildcard xdr/*.x))) \
		--output xdr-definitions-json/xdr.json

clean:
	rm -f src/generated.rs
	rm -rf src/generated
	rm -f xdr-version
	rm -fr xdr-json
	rm -fr xdr-definitions-json
	cargo clean --quiet

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
