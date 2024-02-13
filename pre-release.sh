#!/usr/bin/env bash
set -e

export RUSTFLAGS="-D warnings -A unused_imports"
export CARGO_INCREMENTAL=0

cargo install cargo-hack --locked

cargo hack test --feature-powerset -p $1
cargo hack clippy --feature-powerset -p $1

