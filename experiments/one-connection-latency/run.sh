#!/usr/bin/env bash
#
# Re-derives everything under `results/raw/` from a clean checkout.
#
# NF-001: a figure a second person cannot reproduce is a claim with a table
# attached. This script is the whole of what produced them — README.md states the
# machine, the OS, the filesystem, the toolchain and the build profile, and every
# figure is printed beside the settings that produced it, read back off the live
# connection rather than trusted from the PRAGMA that was issued.
#
# It is NOT a gate step and must never become one. CF-34: performance is measured
# by a separate harness which is not part of the conformance bar, and a benchmark
# that can turn a merge red teaches people to re-run until green. Nothing in
# `.redkiln/config.yaml` or `cargo xtask ci` invokes this, and the crate carries
# an empty `[workspace]` table so cargo cannot reach it from the root manifest
# even by accident.
#
# Wall-clock budget on the machine in README.md: about seven minutes, of which
# instrument (b) is 290 s (85 s of it seeding a million events) and the ceiling
# arm is 7 s. NF-003 requires it to terminate unattended, which it does: every
# replay is bounded by a page count *and* a wall-clock budget, both constants in
# the source, and every busy timeout is the adapter's own finite 5,000 ms.
#
# Disk: the ceiling arm writes about 1 GB (520 MiB of events plus its
# write-ahead log) into the system temporary directory and removes all three
# files when it ends. Instrument (b) writes about 500 MB for the same span.
#
# `.cargo/config.toml` at the repository root sets `-D warnings` ambient and
# cargo config discovery walks up from here, so this crate is built under the
# same warning policy as the workspace. Nothing below overrides it.

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
echo "==> the copy has not drifted: query_sql.rs and row.rs against the originals"
cargo test --test the_copy_has_not_drifted -- --nocapture 2>&1 \
  | tee results/raw/drift.txt

echo
echo "==> CONTROL 2: the conformance suite at every PAGE_SIZE that gets timed"
echo "    (a page size that drops rows at a boundary is fast and wrong)"
cargo test --test replica_is_conformant 2>&1 \
  | tee results/raw/conformance.txt

echo
echo "==> instruments (a) and (c): reactor stall, and the residual after the seam"
cargo test --release --test reactor_stall -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/reactor-stall.txt

echo
echo "==> R-1: one page at the declared data ceiling"
cargo test --release --test page_residency_at_the_ceiling -- \
  --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/ceiling-residency.txt

echo
echo "==> instrument (b): lock hold and residency, 1M events, 12 configurations"
echo "    (this is the long one — about five minutes)"
cargo test --release --test page_lock_hold -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/page-lock-hold.txt

echo
echo "Raw rows are under results/raw/. The tables in results/*.md are written by"
echo "hand from them, because a table nobody read is a table nobody checked."
