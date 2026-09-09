#!/usr/bin/env bash
#
# Re-derives everything under `results/raw/` and one `results/history/` entry
# from a clean checkout.
#
# NF-001, in this repository's own words: a figure a second person cannot
# reproduce is a claim with a table attached. This script is the whole of what
# produced them. `README.md` states the machine, the OS, the filesystem, the
# toolchain and the build profile; every figure is printed beside the conditions
# that produced it, and the SQLite pragmas are read back off the live connection
# rather than trusted from the PRAGMA that issued them.
#
# It is NOT a gate step and must never become one. CF-34
# (`spec/SPECIFICATION.md:8747`) rejects a benchmark result gating a merge: a
# threshold nobody can justify becomes a threshold everybody raises, and the
# number stops meaning anything the second time it is moved. Nothing in
# `.redkiln/config.yaml` or `cargo xtask ci` invokes this, the crate carries an
# empty `[workspace]` table so cargo cannot reach it from the root manifest, and
# `xtask/src/affected.rs`'s INERT list names `benchmarks/` so a diff here does
# not even widen the affected gate.
#
# ---------------------------------------------------------------------------
# Budgets, so a reader knows what they are starting
# ---------------------------------------------------------------------------
#
# Wall clock on the machine in README.md: about 35-45 minutes, of which the
# criterion sweeps are the bulk. `--fast` runs the two conformance controls, the
# two measurement binaries and nothing else, in about four minutes; it is what
# to use when the question is "did I break the instrument".
#
# Disk: the SQLite arms create and delete a temporary database per criterion
# iteration under the system temporary directory. Peak occupancy is a few
# hundred megabytes; every fixture removes its own file and both WAL sidecars on
# drop, and a killed run leaves files named `happenstance-bench-*` behind.
#
# Threads: `--test-threads=1` on every test invocation. The counting allocator's
# counters are process-global, so a second test thread's allocations would land
# inside whatever measurement region happens to be open.
#
# `.cargo/config.toml` at the repository root sets `-D warnings` ambient and
# cargo config discovery walks up from here, so this crate is built under the
# same warning policy as the workspace. Nothing below overrides it.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

fast=0
if [[ "${1:-}" == "--fast" ]]; then
  fast=1
fi

mkdir -p results/raw results/history

# Every `tee` below writes to a fixed filename, so N repetitions of this script
# overwrite each other and only the last survives. That is right for one run, and
# wrong for the one question a single run cannot answer: how far the numbers move
# BETWEEN runs. That is the 40% in README.md's "What none of this shows", and it
# has never had an instrument behind it -- the evidence for it was overwritten by
# the next run that produced it.
#
# `RESULTS_SUFFIX=03 ./run.sh --fast` writes `raw/overhead-03.log` rather than
# `raw/overhead.log`, so ten runs leave ten files to compare. Unset, every
# filename below is exactly what it was.
sfx="${RESULTS_SUFFIX:+-$RESULTS_SUFFIX}"

echo "==> conditions: the toolchain every figure below was produced by"
{
  rustc --version --verbose
  echo
  cargo --version
} 2>&1 | tee results/raw/conditions${sfx}.txt

echo
echo "==> CONTROL 1: conformance first. Both timed arms pass the full suite"
echo "    before any figure taken through them is kept — a wrong arm is always"
echo "    the fastest, and this is what stops that reading as a finding."
cargo test --release --test conformance_first -- --test-threads=1 2>&1 \
  | tee results/raw/conformance${sfx}.txt

echo
echo "==> CONTROL 2: the instruments measure, and the controls fire"
cargo test --release --test instruments_work -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/instruments${sfx}.txt
cargo test --release --test controls_fire -- --test-threads=1 --nocapture 2>&1 \
  | tee results/raw/controls${sfx}.txt

echo
echo "==> the headline: what happenstance costs above raw SQL on the same file."
echo "    Interleaved round-robin in one process, because criterion runs group"
echo "    members sequentially and on this class of host sequential arms have"
echo "    drifted 2.7-3.0x (RUNBOOK.md:1628-1634)."
# stdout and stderr are split, not merged: the binaries write **CSV** to stdout
# and their human-readable report to stderr, and cargo writes its own progress
# to stderr too. Merging them — which the first version of this script did —
# produces a `.csv` with `Compiling happenstance-benchmarks v0.0.0` as its
# first row, which no reader and no spreadsheet can parse. The process
# substitution keeps the report on the terminal while it runs.
cargo run --release --bin overhead   2> >(tee results/raw/overhead${sfx}.log >&2)   | tee results/raw/overhead${sfx}.csv

echo
echo "==> allocations: the reproducible half. These counts were identical to the"
echo "    digit across four runs on a host whose wall-clock medians moved 40%."
cargo run --release --bin allocations   2> >(tee results/raw/allocations${sfx}.log >&2)   | tee results/raw/allocations${sfx}.csv

if [[ "$fast" == "1" ]]; then
  echo
  echo "--fast: the criterion sweeps were skipped. results/raw/ carries the"
  echo "controls, the overhead ratios and the allocation counts; no new"
  echo "results/history/ entry was written, because a partial run must not"
  echo "displace a complete one under the same filename."
  exit 0
fi

echo
echo "==> the criterion sweeps. --save-baseline so criterion's own statistical"
echo "    comparison is available locally; src/bin/collect.rs then flattens the"
echo "    same estimates into results/history/, because a baseline under"
echo "    target/ is not a record anybody can read or diff."
for bench in \
  store_append \
  store_replay \
  store_conditional \
  store_query_shapes \
  typed_codec \
  typed_command \
  projection_runner
do
  echo
  echo "--- $bench ---"
  cargo bench --bench "$bench" -- --save-baseline latest 2>&1 \
    | tee "results/raw/$bench${sfx}.txt"
done

echo
echo "==> the history entry: one committed JSON per run, named for the date and"
echo "    the commit it was taken at. CF-34's own model is that benchmarks are"
echo "    compared against that adapter's own history, and this is the file that"
echo "    makes that possible."
cargo run --release --bin collect   2> >(tee results/raw/collect${sfx}.log >&2)   | tee results/raw/collect${sfx}.csv

echo
echo "Raw output is under results/raw/, and one entry was added to"
echo "results/history/. The tables in results/*.md are written by hand from"
echo "them, because a table nobody read is a table nobody checked."
