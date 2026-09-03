#!/usr/bin/env bash
#
# Re-derives everything under `results/` from a clean checkout.
#
# NF-001: a figure a second person cannot reproduce is a claim with a table
# attached. This script is the whole of what produced them — the README states
# the machine, the toolchain and the pragma settings, and every figure is printed
# beside the settings it was produced on, read back off the live connection by
# `happenstance_sqlite::connection::ConnectionSettings::read_back` rather than
# trusted from the `PRAGMA` that was issued.
#
# It is NOT a gate step and must never become one. CF-34: performance is measured
# by a separate harness which is not part of the conformance bar, and a benchmark
# that can turn a merge red teaches people to re-run until green. Nothing in
# `.redkiln/config.yaml` or `cargo xtask ci` invokes this, and the crate carries
# an empty `[workspace]` table so cargo cannot see it from the root at all.
#
# The order is the argument. Steps 1 and 2 establish that every shape whose
# figure appears below is a **conformant store** and that this crate's
# transcription is the SQL the adapter actually emits. Only then does a clock
# start. A shape that is fast and wrong wins every benchmark.
#
# Wall-clock budget on the machine in the README: about thirteen minutes measured
# (06:14-06:27 on the recorded run; the harness durations sum to 730 s), of which
# the million-event block is the bulk. NF-003 requires it to terminate
# unattended, which it does — every busy timeout is finite (5,000 ms), no step
# waits on another process, and every temporary database is removed by the test
# that made it.
#
# Disk: the timed steps create and delete SQLite files totalling roughly 470 MB
# under the system temporary directory — one 184 MB store for the guard costs,
# a second for the query plans, and a 92 MB one for the seed-ordering control.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw

echo "==> 1. conformance first: a shape that is fast and wrong wins every benchmark"
echo "       (4 shapes x 89 rules; the two bounded shapes answer a *narrower*"
echo "        question, which is exactly the wrong implementation this catches)"
cargo test --test arms_are_conformant 2>&1 | tee results/raw/conformance.txt

echo "==> 2. the conditions control, forced to fire"
cargo test --test conditions_are_enforced -- --nocapture 2>&1 \
  | tee results/raw/conditions.txt

echo "==> 3. what the adapter actually emits, off a sqlite3_trace_v2 callback"
echo "       (the standing guard on this crate's transcription of it)"
cargo test --test emitted_sql -- --nocapture 2>&1 | tee results/raw/emitted-sql.txt

echo "==> 4. the guard, four shapes plus the real adapter, three log sizes, --release"
cargo test --release --test guard_cost -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/guard-cost.txt

echo "==> 5. the plans behind those numbers, and the paged read at 10^6 rows"
cargo test --release --test query_plan -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/query-plans.txt

echo "==> 6. does tag_cardinality's ordering buy the shipped chain anything?"
cargo test --release --test seed_ordering -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/seed-ordering.txt

echo "==> 7. the query-planning quadratics at the specification's own floor"
cargo test --release --test selectivity_cost -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/selectivity.txt

echo
echo "Raw rows are under results/raw/. The tables in results/*.md are written by"
echo "hand from them, because a table nobody read is a table nobody checked."
