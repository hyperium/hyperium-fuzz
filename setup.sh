#!/usr/bin/env bash

set -eu
set -x

# TOOLCHAIN=nightly
TOOLCHAIN=nightly-2023-04-15

rustup update
rustup override set "$TOOLCHAIN"

rustup component add --toolchain "$TOOLCHAIN" rust-src rustfmt llvm-tools-preview

cargo "+$TOOLCHAIN" install --force --git https://github.com/f0rki/cargo-libafl --branch pr
# cargo "+$TOOLCHAIN" install --force --git https://github.com/f0rki/cargo-fuzz --branch careful
cargo "+$TOOLCHAIN" install --force --git https://github.com/rust-fuzz/cargo-fuzz
