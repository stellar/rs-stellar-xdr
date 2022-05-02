XDR_BASE_URL=https://github.com/graydon/stellar-core/raw/wasm-runtime/src/xdr
XDR_FILES= \
	Stellar-SCP.x \
	Stellar-ledger-entries.x \
	Stellar-ledger.x \
	Stellar-overlay.x \
	Stellar-transaction.x \
	Stellar-types.x \
	Stellar-contract.x

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
			$(addprefix xdr/,$(XDR_FILES)) \
		'
	rustfmt src/xdr.rs

$(XDR_FILES): xdr
	curl -L -o xdr/$@ $(XDR_BASE_URL)/$@

xdr:
	mkdir -p xdr

clean:
	rm -fr xdr
	rm -fr src/xdr.rs
	cargo clean
