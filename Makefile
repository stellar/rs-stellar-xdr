XDR_BASE_URL=https://github.com/graydon/stellar-core/raw/wasm-runtime/src
XDR_FILES= \
	xdr/Stellar-SCP.x \
	xdr/Stellar-ledger-entries.x \
	xdr/Stellar-ledger.x \
	xdr/Stellar-overlay.x \
	xdr/Stellar-transaction.x \
	xdr/Stellar-types.x \
	xdr/Stellar-contract.x

all: build test

test:
	cargo test --no-default-features --features 'std'
	cargo test --no-default-features --features 'alloc'
	cargo test --no-default-features --features ''

build: src/lib.rs
	cargo build --no-default-features --features 'std'
	cargo build --no-default-features --features 'alloc'
	cargo build --no-default-features --features ''
	cargo build --no-default-features --features 'std' --release --target wasm32-unknown-unknown
	cargo build --no-default-features --features 'alloc' --release --target wasm32-unknown-unknown
	cargo build --no-default-features --features '' --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

src/lib.rs: $(XDR_FILES)
	docker run -it --rm -v $$PWD:/wd -w /wd ruby /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/leighmcculloch/stellar--xdrgen.git -b rust-no-deps-no-alloc && \
		xdrgen \
			--language rust \
			--namespace lib \
			--output src/ \
			$(XDR_FILES) \
		'
	rustfmt src/lib.rs

$(XDR_FILES):
	curl -L -o $@ $(XDR_BASE_URL)/$@

clean:
	rm -f xdr/*.x
	rm -f src/lib.rs
	cargo clean
