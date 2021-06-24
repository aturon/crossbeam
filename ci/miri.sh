#!/bin/bash

set -ex

export RUSTFLAGS="-D warnings"

if [[ "$OSTYPE" != "linux"* ]]; then
    exit 0
fi

toolchain=nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri)
rustup set profile minimal
rustup default "$toolchain"
rustup component add miri

# -Zmiri-disable-stacked-borrows is needed for https://github.com/crossbeam-rs/crossbeam/issues/545
# -Zmiri-ignore-leaks is needed for https://github.com/crossbeam-rs/crossbeam/issues/579
export MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-disable-stacked-borrows -Zmiri-ignore-leaks -Zmiri-compare-exchange-weak-failure-rate=0.0"
cargo miri test --all --exclude benchmarks
