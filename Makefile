export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

CARGO_HACK_ARGS=--feature-powerset --exclude-features default --group-features base64,serde,arbitrary,hex,rand

CARGO_DOC_ARGS?=--open

all: build test

test:
	cargo hack test $(CARGO_HACK_ARGS)

build: generate
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets
	cargo clippy --no-default-features --features curr,next --release --target wasm32v1-none

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

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

generate: generate-xdrgen-files generate-rust-files xdr/curr-version xdr/next-version xdr-json/curr xdr-json/next check-generated-match

generate-xdrgen-files: src/curr/generated.rs src/next/generated.rs
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:3.1 /bin/bash -c \
		'cd xdr-generator && bundle install --quiet && bundle exec ruby generate.rb'
	rustfmt $^

CUSTOM_DEFAULT_IMPL=TransactionEnvelope
CUSTOM_STR_IMPL=PublicKey,AccountId,ContractId,MuxedAccount,MuxedAccountMed25519,SignerKey,SignerKeyEd25519SignedPayload,NodeId,ScAddress,AssetCode,AssetCode4,AssetCode12,ClaimableBalanceId,PoolId,MuxedEd25519Account,Int128Parts,UInt128Parts,Int256Parts,UInt256Parts

generate-rust-files: src/curr/generated-rust.rs src/next/generated-rust.rs

src/curr/generated-rust.rs: $(sort $(wildcard xdr/curr/*.x))
	cargo run --manifest-path xdr-generator-rust/Cargo.toml -- \
		$(addprefix --input ,$(sort $(wildcard xdr/curr/*.x))) \
		--output $@ \
		--custom-default $(CUSTOM_DEFAULT_IMPL) \
		--custom-str $(CUSTOM_STR_IMPL)
	rustfmt $@

src/next/generated-rust.rs: $(sort $(wildcard xdr/next/*.x))
	cargo run --manifest-path xdr-generator-rust/Cargo.toml -- \
		$(addprefix --input ,$(sort $(wildcard xdr/next/*.x))) \
		--output $@ \
		--custom-default $(CUSTOM_DEFAULT_IMPL) \
		--custom-str $(CUSTOM_STR_IMPL)
	rustfmt $@

check-generated-match:
	@echo "Checking that Ruby and Rust generators produce identical output..."
	@diff -q src/curr/generated.rs src/curr/generated-rust.rs || \
		(echo "ERROR: src/curr/generated.rs and src/curr/generated-rust.rs differ" && exit 1)
	@diff -q src/next/generated.rs src/next/generated-rust.rs || \
		(echo "ERROR: src/next/generated.rs and src/next/generated-rust.rs differ" && exit 1)
	@echo "OK: Ruby and Rust generator outputs match"

src/next/generated.rs: $(sort $(wildcard xdr/curr/*.x))
	> $@

src/curr/generated.rs: $(sort $(wildcard xdr/next/*.x))
	> $@

xdr/curr-version: $(wildcard .git/modules/xdr/curr/**/*) $(wildcard xdr/curr/*.x)
	git submodule status -- xdr/curr | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr/curr-version

xdr/next-version: $(wildcard .git/modules/xdr/next/**/*) $(wildcard xdr/next/*.x)
	git submodule status -- xdr/next | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr/next-version

xdr-json/curr: src/curr/generated.rs
	mkdir -p xdr-json/curr
	cargo run --features cli -- +curr types schema-files --out-dir xdr-json/curr

xdr-json/next: src/next/generated.rs
	mkdir -p xdr-json/next
	cargo run --features cli -- +next types schema-files --out-dir xdr-json/next

clean:
	rm -f src/*/generated.rs
	rm -f src/*/generated-rust.rs
	rm -f xdr/*-version
	rm -fr xdr-json/curr
	rm -fr xdr-json/next
	cargo clean --quiet

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
