XDR_BASE_URL_CURR=https://github.com/stellar/stellar-core/raw/master/src/protocol-curr/xdr
XDR_BASE_LOCAL_CURR=xdr/curr
XDR_FILES_CURR= \
	Stellar-SCP.x \
	Stellar-ledger-entries.x \
	Stellar-ledger.x \
	Stellar-overlay.x \
	Stellar-transaction.x \
	Stellar-types.x
XDR_FILES_LOCAL_CURR=$(addprefix xdr/curr/,$(XDR_FILES_CURR))

XDR_BASE_URL_NEXT=https://github.com/stellar/stellar-xdr-next/raw/main
XDR_BASE_LOCAL_NEXT=xdr/next
XDR_FILES_NEXT= \
	Stellar-SCP.x \
	Stellar-ledger-entries.x \
	Stellar-ledger.x \
	Stellar-overlay.x \
	Stellar-transaction.x \
	Stellar-types.x \
	Stellar-contract.x \
	Stellar-contract-env-meta.x \
	Stellar-contract-spec.x
XDR_FILES_LOCAL_NEXT=$(addprefix xdr/next/,$(XDR_FILES_NEXT))

export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

CARGO_HACK_ARGS=--feature-powerset --exclude-features default --group-features base64,serde,num-bigint,arbitrary,hex

all: build test

test:
	cargo hack test $(CARGO_HACK_ARGS)

build: generate
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets
	cargo hack clippy $(CARGO_HACK_ARGS) --all-targets --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

generate: src/curr.rs src/next.rs

src/curr.rs: $(XDR_FILES_LOCAL_CURR)
	> $@
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b 633a01ee && \
		xdrgen --language rust --namespace curr --output src/ $^ \
		'
	rustfmt $@

src/next.rs: $(XDR_FILES_LOCAL_NEXT)
	> $@
	docker run -i --rm -v $$PWD:/wd -w /wd docker.io/library/ruby:latest /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/stellar/xdrgen.git -b 633a01ee && \
		xdrgen --language rust --namespace next --output src/ $^ \
		'
	rustfmt $@

clean:
	rm -f src/curr.rs
	rm -f src/next.rs
	cargo clean

$(XDR_FILES_LOCAL_CURR):
	curl -L -o $@ $(XDR_BASE_URL_CURR)/$(notdir $@)

$(XDR_FILES_LOCAL_NEXT):
	curl -L -o $@ $(XDR_BASE_URL_NEXT)/$(notdir $@)

reset-xdr:
	rm -f xdr/*/*.x
	$(MAKE) build

fmt:
	cargo fmt --all

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
