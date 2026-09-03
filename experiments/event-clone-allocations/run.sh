#!/usr/bin/env bash
#
# Re-derives everything under `results/raw/` from a clean checkout.
#
# NF-001: a figure a second person cannot reproduce is a claim with a table
# attached. This script is the whole of what produced them — README.md states the
# machine, the OS, the filesystem, the toolchain and the build profile, and every
# figure is printed beside the settings that produced it by the code that
# produced it.
#
# It is NOT a gate step and must never become one. CF-34: performance is measured
# by a separate harness which is not part of the conformance bar, and a benchmark
# that can turn a merge red teaches people to re-run until green. Nothing in
# `.redkiln/config.yaml` or `cargo xtask ci` invokes this, and the crate carries
# an empty `[workspace]` table so cargo cannot reach it from the root manifest
# even by accident.
#
# Wall-clock budget on the machine in README.md: about a minute, of which the
# million-event case is roughly six seconds. NF-003 requires it to terminate
# unattended, which it does: there is no loop here whose bound is anything but a
# constant in the source.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw

echo "==> conditions: the toolchain that produced every figure below"
{
  rustc --version --verbose
  echo
  cargo --version
} 2>&1 | tee results/raw/conditions.txt

echo
echo "==> conformance first: an arm that is fast and wrong wins every benchmark"
echo "    (the regimes build the same event, encode to the same bytes in both"
echo "     formats, round-trip framed, and the read replicas match memory.rs)"
cargo test --test arms_are_equivalent -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/conformance.txt

echo
echo "==> arms 1-4 and the size_of table: one clone, and one encode"
cargo test --release --test measure_clone -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/clone.txt

echo
echo "==> arms 5-6: read(limit=1) at 10 / 10k / 1M, before and after the fix"
cargo test --release --test measure_read -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/read.txt

echo
echo "Raw rows are under results/raw/. The tables in results/*.md are written by"
echo "hand from them, because a table nobody read is a table nobody checked."
