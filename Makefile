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

generate: generate-xdrgen-files xdr/curr-version xdr/next-version xdr-json/curr xdr-json/next

generate-xdrgen-files: src/curr/generated.rs src/next/generated.rs
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:3.1 /bin/bash -c \
		'cd xdr-generator && bundle install --quiet && bundle exec ruby generate.rb'
	#rustfmt $^

src/next/generated.rs: $(sort $(wildcard xdr/curr/*.x))
	@:

src/curr/generated.rs: $(sort $(wildcard xdr/next/*.x))
	@:

xdr/curr-version: $(wildcard .git/modules/xdr/curr/**/*) $(wildcard xdr/curr/*.x)
	git submodule status -- xdr/curr | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr/curr-version

xdr/next-version: $(wildcard .git/modules/xdr/next/**/*) $(wildcard xdr/next/*.x)
	git submodule status -- xdr/next | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' | tr -d '+' > xdr/next-version

xdr-json/curr: src/curr/generated.rs
	cargo run --features cli -- +curr types schema-files --out-dir xdr-json/curr

xdr-json/next: src/next/generated.rs
	cargo run --features cli -- +next types schema-files --out-dir xdr-json/next

clean:
	rm -f src/*/generated.rs
	rm -f xdr/*-version
	rm -f xdr-json/curr/*.json
	rm -f xdr-json/next/*.json
	cargo clean --quiet

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
