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
# The order is the argument. Step 1 establishes that all **six** shapes — the
# two new ones included — are conformant stores, and step 2 that this crate's
# transcription of the shipped chain is still the SQL the adapter emits. Only
# then does a clock start.
#
# **Step 1 is the one that matters here.** The shape under test rewrites an
# uncorrelated subquery into a correlated one, and the correlation is spelled
# with a table alias. An alias that resolved to the inner table would make the
# predicate the tautology `position = position` — no compile error, no runtime
# error, a two-tag guard silently behaving as a one-tag guard, and the fastest
# arm in every cell below. A shape that is fast and wrong wins every benchmark.
#
# Wall-clock budget on the machine in the README: about twenty-five minutes, of
# which the guard-cost block over three log sizes is the bulk. NF-003 requires
# it to terminate unattended, which it does — every busy timeout is finite
# (5,000 ms), no step waits on another process, and every temporary database is
# removed by the test that made it.
#
# Disk: the timed steps create and delete SQLite files totalling roughly 470 MB
# under the system temporary directory.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw

echo "==> 1. conformance first: a shape that is fast and wrong wins every benchmark"
echo "       (6 shapes x 89 rules. The two new ones are the reason this step is"
echo "        first rather than thorough: a mis-scoped alias in the correlated"
echo "        EXISTS is a silent one-tag guard, and only the rules catch it.)"
cargo test --test arms_are_conformant 2>&1 | tee results/raw/conformance.txt

echo
echo "==> 2. the conditions control, forced to fire"
cargo test --test conditions_are_enforced -- --nocapture 2>&1 \
  | tee results/raw/conditions.txt

echo
echo "==> 3. what the adapter actually emits, off a sqlite3_trace_v2 callback"
echo "       (the standing guard on this crate's transcription of the shipped"
echo "        chain — the baseline every ratio below is taken against)"
cargo test --test emitted_sql -- --nocapture 2>&1 | tee results/raw/emitted-sql.txt

echo
echo "==> 4. the guard: six shapes, four scenarios, three log sizes, --release"
echo "       (round-robin within each round, so no shape is permanently first)"
cargo test --release --test guard_cost -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/guard-cost.txt

echo
echo "==> 5. the plans behind those numbers — the mechanism, not the timing"
cargo test --release --test query_plan -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/query-plans.txt

echo
echo "==> 6. does the correlated form make most-selective-first correct?"
echo "       (the shipped chain and the EXISTS form, both orderings, alternating)"
cargo test --release --test seed_ordering -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/seed-ordering.txt

echo
echo "==> 7. the adversarial corpus: no tag selective, so the seed cannot help."
echo "       This is the shape where the correlated form could plausibly lose,"
echo "       and the run is not a recommendation until it has been looked at."
cargo test --release --test unselective_pair -- --nocapture --test-threads=1 2>&1   | tee results/raw/unselective-pair.txt

echo
echo "==> 8. the READ path: no max() to stop at, so this is where the guard's"
echo "       margin either transfers or does not. Both corpora, both shapes."
cargo test --release --test read_path -- --nocapture --test-threads=1 2>&1   | tee results/raw/read-path.txt

echo
echo "==> 9. the one wrapper repair that needs no estimate: Query::all, whose"
echo "       membership test is a tautology over the whole table. Three replay"
echo "       depths, because the first run refuted the flat cost it was"
echo "       proposed on and the axis it does move on had to be measured."
cargo test --release --test all_query_wrapper -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/all-query-wrapper.txt

echo
echo "==> 10. the fourth candidate read-path.md did not have: the read's window"
echo "        pushed INTO each arm rather than applied outside it. Both corpora,"
echo "        three depths and one backwards cell, with the returned page as the"
echo "        control — soundness is prior to speed, and an arm ordered the wrong"
echo "        way returns a page that is short rather than obviously wrong."
cargo test --release --test windowed_arms -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/windowed-arms.txt

echo
echo "==> 11. the selectivity lookup's own cost, at VT-23's item floor"
cargo test --release --test selectivity_cost -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/selectivity-cost.txt

echo
echo "Raw rows are under results/raw/. The tables in results/*.md are written by"
echo "hand from them, because a table nobody read is a table nobody checked."
