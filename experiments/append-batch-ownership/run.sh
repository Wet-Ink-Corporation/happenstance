#!/usr/bin/env bash
#
# Re-derives everything under `results/raw/` from a clean checkout.
#
# NOT a gate step and must never become one (CF-34). Nothing in `cargo xtask ci`
# invokes this, and the crate's empty `[workspace]` table keeps it out of the
# root manifest's reach.
#
# Needs, besides the pinned toolchain:
#   * the wasm32-unknown-unknown target;
#   * `wasm-bindgen-test-runner` matching this crate's locked `wasm-bindgen`
#     exactly (0.2.126): `cargo install wasm-bindgen-cli --version 0.2.126 --locked`;
#   * Node 24 on PATH, because the Durable Object host opens `node:sqlite` with
#     workerd's `limits` and refuses on a Node without them
#     (`crates/happenstance-cloudflare/src/host.rs`). With nvm:
#     `export NVM_DIR=/opt/nvm; . "$NVM_DIR/nvm.sh"; nvm use 24`.
#
# Order matters: conformance first, on both legs, because an arm that is fast
# and wrong wins every benchmark, and `set -e` stops the run before a single
# timing row is written if any conformance test fails.
#
# Wall-clock budget on the machine in README.md: about six minutes, of which
# the Cloudflare sweep is two and a half and the Cloudflare contention run most
# of the rest. Every loop is bounded by a constant in the source (NF-003).

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
wasm=(cargo test --release --target wasm32-unknown-unknown)

echo "==> conditions"
{
  echo "date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "commit: $(git -C "$here" rev-parse --short HEAD) (plus this directory, uncommitted at run time if so)"
  echo
  uname -srvmo
  grep -m1 'model name' /proc/cpuinfo || true
  echo "cpus: $(nproc)"
  grep -m1 MemTotal /proc/meminfo || true
  echo
  rustc --version --verbose
  cargo --version
  echo "node: $(node --version)"
  wasm-bindgen-test-runner --version
  echo
  echo "locked versions (this crate's Cargo.lock):"
  for crate in bytes worker wasm-bindgen js-sys wasm-bindgen-test; do
    version="$(grep -A1 "^name = \"$crate\"$" Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p')"
    echo "  $crate $version"
  done
} 2>&1 | tee results/raw/conditions.txt

echo
echo "==> conformance, host: the move is a move, the replicas are the store"
cargo test --release --test host --test exact --test contention -- --test-threads=1 2>&1 \
  | tee results/raw/conformance-host.txt

echo
echo "==> conformance, cloudflare: read-back equality, B0 calibration, B1 and O1 deltas"
"${wasm[@]}" --test cloudflare 2>&1 | tee results/raw/conformance-cloudflare.txt

echo
echo "==> host: caller-side prices, memory control, memory contention"
cargo test --release --test measure_host -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/host.txt

echo
echo "==> cloudflare: the sweep (batch 1, 16, 128 x tags x payload x regime)"
"${wasm[@]}" --test measure_cloudflare -- --nocapture sweep_ 2>&1 \
  | tee results/raw/cloudflare-sweep.txt

echo
echo "==> cloudflare: k=8 contention, the caller-side axis"
"${wasm[@]}" --test measure_cloudflare -- --nocapture contention_ 2>&1 \
  | tee results/raw/cloudflare-contention.txt

echo
echo "Raw rows are under results/raw/. The tables in results/*.md are written by"
echo "hand from them, because a table nobody read is a table nobody checked."
