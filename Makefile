export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

CARGO_HACK_ARGS=--feature-powerset --exclude-features default --group-features base64,serde,arbitrary,hex

XDRGEN_VERSION=57beb46b
CARGO_DOC_ARGS?=--open

all: build test

test:
	cargo hack test $(CARGO_HACK_ARGS)

build: generate
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets --release --target wasm32-unknown-unknown

doc:
	cargo test --doc --all-features
	RUSTDOCFLAGS="--cfg docs" cargo +nightly doc --all-features $(CARGO_DOC_ARGS)

install:
	cargo install --path . --force --features cli

readme:
	cargo readme > README.md

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

generate: src/curr/generated.rs xdr/curr-version src/next/generated.rs xdr/next-version

src/curr/generated.rs: $(sort $(wildcard xdr/curr/*.x))
	> $@
ifeq ($(LOCAL_XDRGEN),)
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b $(XDRGEN_VERSION) && \
		xdrgen --language rust --namespace generated --output src/curr $^ \
		'
else
	docker run -i --rm -v $$PWD/../xdrgen:/xdrgen -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		pushd /xdrgen && bundle install --deployment && rake install && popd && \
		xdrgen --language rust --namespace generated --output src/curr $^ \
		'
endif
	rustfmt $@

xdr/curr-version: $(wildcard .git/modules/xdr/curr/**/*) $(wildcard xdr/curr/*.x)
	git submodule status -- xdr/curr | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' > xdr/curr-version

src/next/generated.rs: $(sort $(wildcard xdr/next/*.x))
	> $@
ifeq ($(LOCAL_XDRGEN),)
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b $(XDRGEN_VERSION) && \
		xdrgen --language rust --namespace generated --output src/next $^ \
		'
else
	docker run -i --rm -v $$PWD/../xdrgen:/xdrgen -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		pushd /xdrgen && bundle install --deployment && rake install && popd && \
		xdrgen --language rust --namespace generated --output src/next $^ \
		'
endif
	rustfmt $@

xdr/next-version: $(wildcard .git/modules/xdr/next/**/*) $(wildcard xdr/next/*.x)
	git submodule status -- xdr/next | sed 's/^ *//g' | cut -f 1 -d " " | tr -d '\n' > xdr/next-version

clean:
	rm -f src/*/generated.rs
	rm -f xdr/*-version
	cargo clean

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
