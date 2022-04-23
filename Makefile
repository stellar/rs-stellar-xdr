test: build
	cargo test

build: src/xdr.rs xdr
	cargo build --profile dev
	cargo build --profile release

src/xdr.rs: xdr
	docker run -it --rm -v $$PWD:/wd -w /wd ruby /bin/bash -c '\
		gem install specific_install && \
		gem specific_install https://github.com/leighmcculloch/stellar--xdrgen.git -b rust-no-deps && \
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
		'
	rustfmt src/xdr.rs

xdr:
	mkdir xdr
	cd xdr && \
	curl -LO https://github.com/stellar/stellar-core/raw/master/src/xdr/Stellar-SCP.x && \
	curl -LO https://github.com/stellar/stellar-core/raw/master/src/xdr/Stellar-ledger-entries.x && \
	curl -LO https://github.com/stellar/stellar-core/raw/master/src/xdr/Stellar-ledger.x && \
	curl -LO https://github.com/stellar/stellar-core/raw/master/src/xdr/Stellar-overlay.x && \
	curl -LO https://github.com/stellar/stellar-core/raw/master/src/xdr/Stellar-transaction.x && \
	curl -LO https://github.com/stellar/stellar-core/raw/master/src/xdr/Stellar-types.x

clean:
	rm -fr xdr src/xdr.rs
