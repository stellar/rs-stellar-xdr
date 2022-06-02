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

XDR_BASE_URL_NEXT=https://github.com/stellar/stellar-core/raw/master/src/protocol-next/xdr
XDR_BASE_LOCAL_NEXT=xdr/next
XDR_FILES_NEXT= \
	Stellar-SCP.x \
	Stellar-ledger-entries.x \
	Stellar-ledger.x \
	Stellar-overlay.x \
	Stellar-transaction.x \
	Stellar-types.x \
	Stellar-contract.x
XDR_FILES_LOCAL_NEXT=$(addprefix xdr/next/,$(XDR_FILES_NEXT))

export RUSTFLAGS=-Dwarnings -Dclippy::all -Dclippy::pedantic

all: build test

test:
	cargo test --no-default-features --features 'std'
	cargo test --no-default-features --features 'alloc'
	cargo test --no-default-features --features ''
	cargo test --no-default-features --features 'next,std'
	cargo test --no-default-features --features 'next,alloc'
	cargo test --no-default-features --features 'next'

build: src/curr.rs src/next.rs
	cargo clippy --all-targets --no-default-features --features 'std'
	cargo clippy --all-targets --no-default-features --features 'alloc'
	cargo clippy --all-targets --no-default-features --features ''
	cargo clippy --all-targets --no-default-features --features 'std' --release --target wasm32-unknown-unknown
	cargo clippy --all-targets --no-default-features --features 'alloc' --release --target wasm32-unknown-unknown
	cargo clippy --all-targets --no-default-features --features '' --release --target wasm32-unknown-unknown

	cargo clippy --all-targets --no-default-features --features 'next,std'
	cargo clippy --all-targets --no-default-features --features 'next,alloc'
	cargo clippy --all-targets --no-default-features --features 'next'
	cargo clippy --all-targets --no-default-features --features 'next,std' --release --target wasm32-unknown-unknown
	cargo clippy --all-targets --no-default-features --features 'next,alloc' --release --target wasm32-unknown-unknown
	cargo clippy --all-targets --no-default-features --features 'next' --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

src/curr.rs: $(XDR_FILES_LOCAL_CURR)
	docker run -it --rm -v $$PWD:/wd -w /wd ruby /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/leighmcculloch/stellar--xdrgen.git -b rustlib && \
		xdrgen --language rust --namespace curr --output src/ $^ \
		'
	rustfmt src/curr.rs

src/next.rs: $(XDR_FILES_LOCAL_NEXT)
	docker run -it --rm -v $$PWD:/wd -w /wd ruby /bin/bash -c '\
		gem install specific_install -v 0.3.7 && \
		gem specific_install https://github.com/leighmcculloch/stellar--xdrgen.git -b rustlib && \
		xdrgen --language rust --namespace next --output src/ $^ \
		'
	rustfmt src/next.rs

$(XDR_FILES_LOCAL_CURR):
	curl -L -o $@ $(XDR_BASE_URL_CURR)/$(notdir $@)

$(XDR_FILES_LOCAL_NEXT):
	curl -L -o $@ $(XDR_BASE_URL_NEXT)/$(notdir $@)

clean:
	rm -f xdr/*.x
	rm -f src/curr.rs
	rm -f src/next.rs
	cargo clean
