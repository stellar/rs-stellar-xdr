XDR_BASE_URL=https://github.com/graydon/stellar-core/raw/wasm-runtime/src
XDR_FILES= \
	xdr/Stellar-SCP.x \
	xdr/Stellar-ledger-entries.x \
	xdr/Stellar-ledger.x \
	xdr/Stellar-overlay.x \
	xdr/Stellar-transaction.x \
	xdr/Stellar-types.x \
	xdr/Stellar-contract.x

test: build
	cargo test

build: src/xdr.rs
	cargo build --profile dev
	cargo build --profile release

src/xdr.rs: $(XDR_FILES)
	docker run -it --rm -v $$PWD:/wd -w /wd ruby /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/leighmcculloch/stellar--xdrgen.git -b rust-no-deps && \
		xdrgen \
			--language rust \
			--namespace xdr \
			--output src/ \
			$(XDR_FILES) \
		'
	rustfmt src/xdr.rs

$(XDR_FILES):
	curl -L -o $@ $(XDR_BASE_URL)/$@

clean:
	rm -f xdr/*.x
	rm -f src/xdr.rs
	cargo clean
