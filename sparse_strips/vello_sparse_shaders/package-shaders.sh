#!/bin/sh
# Regenerate `src/generated/compiled_shaders.rs` from `shaders/*.wesl`.
set -eu
cd "$(dirname "$0")"
cargo run -p vello_sparse_shaders --bin package-shaders --features shadergen
rustfmt --edition 2024 src/generated/compiled_shaders.rs
