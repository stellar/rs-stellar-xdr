#!/usr/bin/env bash

set -e
alias npx='npx -y'
(
    cd crates/stellar-xdr-wasm
    # iterates over all targets and builds each on in it's on folder
    for TARGET in ${TARGETS:-bundler nodejs web no-modules} ; do
        OUT_DIR=pkg/$TARGET
        npx wasm-pack build --"${PROFILE:-dev}" \
                               --target "$TARGET" \
                               --out-dir "$OUT_DIR" \
                               --out-name "stellar-xdr-wasm-$TARGET"
        (
            cd "$OUT_DIR";
             # shellcheck disable=SC2016
            npx node-jq --arg TARGET "stellar-xdr-wasm-$TARGET" \
                            '.name = $TARGET' package.json > tmp.json;
            mv tmp.json package.json;
            npx yalc publish;
        )
    done
)
