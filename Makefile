XDR_BASE_URL=https://github.com/graydon/stellar-core/raw/wasm-runtime/src/xdr

test: build
	cargo test

build: src/xdr.rs xdr
	cargo build --profile dev
	cargo build --profile release

src/xdr.rs: xdr
	docker run -it --rm -v $$PWD:/wd -w /wd ruby /bin/bash -c '\
		gem install specific_install && \
		gem specific_install https://github.com/leighmcculloch/stellar--xdrgen.git -b rust-no-deps-4 && \
		xdrgen \
			--language rust \
			--namespace xdr \
			--output src/ \
			xdr/Stellar-SCP.x \
			xdr/Stellar-ledger-entries.x \
			xdr/Stellar-ledger.x \
			xdr/Stellar-overlay.x \
			xdr/Stellar-transaction.x \
			xdr/Stellar-types.x \
			xdr/Stellar-contract.x \
		'
	rustfmt src/xdr.rs

xdr:
	mkdir xdr
	cd xdr && \
	curl -LO $(XDR_BASE_URL)/Stellar-SCP.x && \
	curl -LO $(XDR_BASE_URL)/Stellar-ledger-entries.x && \
	curl -LO $(XDR_BASE_URL)/Stellar-ledger.x && \
	curl -LO $(XDR_BASE_URL)/Stellar-overlay.x && \
	curl -LO $(XDR_BASE_URL)/Stellar-transaction.x && \
	curl -LO $(XDR_BASE_URL)/Stellar-types.x && \
	curl -LO $(XDR_BASE_URL)/Stellar-contract.x && \
	true

clean:
	rm -fr xdr
	rm -fr src/xdr.rs
