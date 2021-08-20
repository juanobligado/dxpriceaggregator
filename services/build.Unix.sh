#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd ceramic_adapter
cargo update --aggressive
marine build --release

cd ../aggregator_service
cargo update --aggressive
marine build --release

cd ..

mkdir -p artifacts
rm -f artifacts/*.wasm

cp ceramic_adapter/target/wasm32-wasi/release/ceramic_adapter.wasm artifacts/
cp aggregator_service/target/wasm32-wasi/release/aggregator_service.wasm artifacts/
