#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd ceramic_adapter
cargo update --aggressive
marine build --release

cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp ceramic_adapter/target/wasm32-wasi/release/ceramic_adapter.wasm artifacts/
#cp facade/target/wasm32-wasi/release/facade.wasm artifacts/
