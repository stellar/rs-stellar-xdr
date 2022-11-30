export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

CARGO_HACK_ARGS=--feature-powerset --exclude-features default --group-features base64,serde,arbitrary,hex

XDRGEN_VERSION=964a9cc3

all: build test

test:
	cargo hack test $(CARGO_HACK_ARGS)

build: generate
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

generate: src/curr.rs src/next.rs

src/curr.rs: $(sort $(wildcard xdr/curr/*.x))
	> $@
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b $(XDRGEN_VERSION) && \
		xdrgen --language rust --namespace curr --output src/ $^ \
		'
	rustfmt $@

src/next.rs: $(sort $(wildcard xdr/next/*.x))
	> $@
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b $(XDRGEN_VERSION) && \
		xdrgen --language rust --namespace next --output src/ $^ \
		'
	rustfmt $@

clean:
	rm -f src/curr.rs
	rm -f src/next.rs
	cargo clean

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
