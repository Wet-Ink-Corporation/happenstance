#!/usr/bin/env bash
#
# Re-derives everything under `results/` from a clean checkout.
#
# NF-001: a figure a second person cannot reproduce is a claim with a table
# attached. This script is the whole of what produced them — the README states
# the machine, the toolchain and the pragma settings, and every figure is
# printed beside the settings it was produced on.
#
# It is NOT a gate step and must never become one. CF-34: performance is
# measured by a separate harness which is not part of the conformance bar, and
# a benchmark that can turn a merge red teaches people to re-run until green.
# Nothing in `.redkiln/config.yaml` or `cargo xtask ci` invokes this.
#
# Wall-clock budget on the machine in the README: about five minutes, of which
# the 64-contender races are 220 s. NF-003 requires it to terminate unattended,
# which it does — every busy timeout is
# finite (5,000 ms) and there is no watchdog anywhere to rescue it if one were
# not.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw

echo "==> conformance first: an arm that is fast and wrong wins every benchmark"
cargo test --test candidates_are_conformant 2>&1 | tee results/raw/conformance.txt

echo "==> the durability control: refuse to measure under a setting that cannot ship"
cargo test --test durability_settings_are_enforced 2>&1 \
  | tee results/raw/durability.txt

echo "==> the harness, verbatim: event_store_benchmarks! at n=512, k=64, N=5000"
cargo test --release --test measure -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/harness.txt

echo "==> caller-side control: the tag storages under one probe, round-robin"
cargo test --release --test tag_storage_probe -- --nocapture 2>&1 \
  | tee results/raw/tag-storage.txt

echo "==> caller-side control: the strategies at 8 and 64 connections, round-robin"
cargo test --release --test contention_at_64 -- --nocapture 2>&1 \
  | tee results/raw/contention.txt

echo
echo "Raw rows are under results/raw/. The tables in results/*.md are written by"
echo "hand from them, because a table nobody read is a table nobody checked."
