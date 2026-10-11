#!/usr/bin/env bash
#
# Re-derives everything under `results/raw/` from a clean checkout.
#
# It is NOT a gate step and must never become one: the crate carries an empty
# `[workspace]` table so cargo cannot reach it from the root manifest, and
# nothing in `cargo xtask ci` invokes this. It terminates unattended — every loop
# in the source is bounded by the Unicode codespace or by a constant-length
# corpus — in a few seconds at `--release`.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw

echo "==> conditions: the toolchain and tree that produced every line below"
{
  rustc --version --verbose
  echo
  cargo --version
  echo
  echo "tree: $(git rev-parse --short HEAD)"
} 2>&1 | tee results/raw/conditions.txt

echo
echo "==> the pre-registered criteria, each as a test, and the wrong validators"
echo "    each part of the harness must catch"
cargo test --release --locked -- --test-threads=1 2>&1 | tee results/raw/tests.txt

echo
echo "==> the run: E11-R per position and constructor, then the corpus"
cargo run --release --locked --quiet 2>&1 | tee results/raw/run.txt
