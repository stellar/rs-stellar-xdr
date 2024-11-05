export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

CARGO_HACK_ARGS=--feature-powerset --exclude-features default --group-features base64,serde,arbitrary,hex

CARGO_DOC_ARGS?=--open

XDRGEN_VERSION=b7bc57ecdd277c9575930d3e17c12dfaa76655fc
# XDRGEN_LOCAL=1
XDRGEN_TYPES_CUSTOM_STR_IMPL_CURR=PublicKey,AccountId,MuxedAccount,MuxedAccountMed25519,SignerKey,SignerKeyEd25519SignedPayload,NodeId,ScAddress,AssetCode,AssetCode4,AssetCode12,ClaimableBalanceId
XDRGEN_TYPES_CUSTOM_STR_IMPL_NEXT=PublicKey,AccountId,MuxedAccount,MuxedAccountMed25519,SignerKey,SignerKeyEd25519SignedPayload,NodeId,ScAddress,AssetCode,AssetCode4,AssetCode12,ClaimableBalanceId
XDRGEN_TYPES_CUSTOM_JSONSCHEMA_IMPL_CURR=PublicKey,AccountId,MuxedAccount,MuxedAccountMed25519,SignerKey,SignerKeyEd25519SignedPayload,NodeId,ScAddress,AssetCode,AssetCode4,AssetCode12,ClaimableBalanceId
XDRGEN_TYPES_CUSTOM_JSONSCHEMA_IMPL_NEXT=PublicKey,AccountId,MuxedAccount,MuxedAccountMed25519,SignerKey,SignerKeyEd25519SignedPayload,NodeId,ScAddress,AssetCode,AssetCode4,AssetCode12,ClaimableBalanceId

all: build test

test:
	cargo hack test $(CARGO_HACK_ARGS)

build: generate
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets --release --target wasm32-unknown-unknown

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

generate: src/curr/generated.rs xdr/curr-version src/next/generated.rs xdr/next-version

src/curr/generated.rs: $(sort $(wildcard xdr/curr/*.x))
	> $@
ifeq ($(XDRGEN_LOCAL),)
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.8 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b $(XDRGEN_VERSION) && \
		xdrgen --language rust --namespace generated --output src/curr \
			--rust-types-custom-str-impl $(XDRGEN_TYPES_CUSTOM_STR_IMPL_CURR) \
			--rust-types-custom-jsonschema-impl '$(XDRGEN_TYPES_CUSTOM_JSONSCHEMA_IMPL_CURR)' \
		$^ \
		'
else
	docker run -i --rm -v $$PWD/../xdrgen:/xdrgen -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		pushd /xdrgen && bundle install --deployment && rake install && popd && \
		xdrgen --language rust --namespace generated --output src/curr \
			--rust-types-custom-str-impl $(XDRGEN_TYPES_CUSTOM_STR_IMPL_CURR) \
			--rust-types-custom-jsonschema-impl '$(XDRGEN_TYPES_CUSTOM_JSONSCHEMA_IMPL_CURR)' \
		$^ \
		'
endif
	rustfmt $@

xdr/curr-version: $(wildcard .git/modules/xdr/curr/**/*) $(wildcard xdr/curr/*.x)
	git submodule status -- xdr/curr | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr/curr-version

src/next/generated.rs: $(sort $(wildcard xdr/next/*.x))
	> $@
ifeq ($(XDRGEN_LOCAL),)
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.8 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b $(XDRGEN_VERSION) && \
		xdrgen --language rust --namespace generated --output src/next \
			--rust-types-custom-str-impl $(XDRGEN_TYPES_CUSTOM_STR_IMPL_NEXT) \
			--rust-types-custom-jsonschema-impl '$(XDRGEN_TYPES_CUSTOM_JSONSCHEMA_IMPL_NEXT)' \
		$^ \
		'
else
	docker run -i --rm -v $$PWD/../xdrgen:/xdrgen -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		pushd /xdrgen && bundle install --deployment && rake install && popd && \
		xdrgen --language rust --namespace generated --output src/next \
			--rust-types-custom-str-impl $(XDRGEN_TYPES_CUSTOM_STR_IMPL_NEXT) \
			--rust-types-custom-jsonschema-impl '$(XDRGEN_TYPES_CUSTOM_JSONSCHEMA_IMPL_NEXT)' \
		$^ \
		'
endif
	rustfmt $@

xdr/next-version: $(wildcard .git/modules/xdr/next/**/*) $(wildcard xdr/next/*.x)
	git submodule status -- xdr/next | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr/next-version

clean:
	rm -f src/*/generated.rs
	rm -f xdr/*-version
	cargo clean

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
