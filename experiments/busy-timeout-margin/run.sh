#!/usr/bin/env bash
#
# Re-derives everything under `results/raw/` from a clean checkout.
#
# NF-001: a figure a second person cannot reproduce is a claim with a table
# attached. This script is the whole of what produced them; `README.md` states
# the machine, the toolchain, the build profile and the core-count method, and
# every row prints the settings it was produced under beside itself — the
# affinity mask off the live process, the pragmas off the live connection.
#
# It is NOT a gate step and must never become one. CF-34: performance is
# measured by a separate harness which is not part of the conformance bar, and a
# benchmark that can turn a merge red teaches people to re-run until green. This
# one would be worse than most: its whole job is to run the gate's contention
# shape until it breaks. Nothing in `.redkiln/config.yaml` or `cargo xtask ci`
# invokes it, and the crate carries an empty `[workspace]` table so cargo cannot
# see it from the root manifest.
#
# It terminates unattended (NF-003). Every busy wait is bounded by the 5,000 ms
# cap the counting handler enforces, every race is bounded by its contenders'
# waits, and the one probe that can genuinely hang — `tests/lost_wakeup.rs` — is
# joined through a channel with a ten-second deadline and reports the hang
# instead of waiting for it.
#
# Wall-clock budget on the machine in the README: about eighteen minutes — two
# conformance runs, two release controls, a release bridge, five core-count
# pairs, the shipped arm, a four-point contender sweep, six repeats at the
# shipped contender count, and the wakeup probe.

set -euo pipefail

# Git Bash rewrites anything in an argument that looks like a POSIX path, which
# mangles the `--nocapture`/`--skip` arguments handed through PowerShell to
# libtest. Both variables are needed: one covers the conversion, the other the
# argument-list rewrite.
export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL='*'

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

# One run at a time, and this is a correctness guard rather than tidiness.
#
# Two copies of this script on one machine do not produce two runs; they produce
# two *wrong* runs. They interleave their writes into `results/raw/`, so a row
# and the `waits/` dump beside it can come from different runs — and, far worse,
# each becomes the other's background load. A sixty-four-way race measured while
# a second sixty-four-way race is running is not the configuration any row claims
# to be. That happened during this experiment's own development and it cost two
# complete runs, which is why the guard is here and not in a comment.
#
# `mkdir` is the lock: it is atomic on every filesystem this could run on, needs
# no `flock` (Git Bash has none), and leaves a directory a human can delete if a
# run is killed mid-flight — which the message says.
lock=results/.run-lock
if ! mkdir "$lock" 2>/dev/null; then
  echo "refusing to start: $lock exists, so another ./run.sh is already running." >&2
  echo "A concurrent run would be this one's background load and both rows would" >&2
  echo "be mislabelled. If no run is active, remove $lock and try again." >&2
  exit 1
fi
trap 'rmdir "$lock" 2>/dev/null || true' EXIT

rm -rf results/raw
mkdir -p results/raw/waits
# `pwd -W`, not `$here`, and the difference is a raw file that went missing.
#
# `$here` is a Git Bash path — `/d/repos/happenstance/…`. The test binary is a
# native Windows process, so `Path::new("/d/repos/…")` resolves against the
# *current drive root* and `create_dir_all` cheerfully builds `D:\d\repos\…`
# instead. It succeeds, so nothing is printed, and `results/raw/waits/` is empty
# while the dumps sit in a directory nobody looks in. That is precisely the class
# of failure this experiment is about — a call that reports success and does
# something else — so it is fixed here rather than worked around in the reader.
export HS_WAITS_DIR="$(pwd -W)/results/raw/waits"

# The five affinity masks, and why each is the mask it is.
#
# `taskset` does not exist on Windows; the constraint is a processor affinity
# mask, applied by `affinity-run.ps1` to itself so that the test binary inherits
# it at creation. There is therefore no window in which the child runs
# unconstrained, and the child checks the mask it actually got before it measures
# anything.
#
# The bits are chosen to select distinct *physical* cores, which a naive
# `0x1,0x3,0xf,0xff` sequence does not. This host is a 13th Gen i9-13905H: 14
# physical cores, 20 logical, hybrid — six performance cores with SMT on logical
# 0-11 (paired: 0/1 are one core), eight efficiency cores with no SMT on logical
# 12-19. The claim is not taken from a datasheet: `0x3` measures **0.9** cores of
# throughput on this machine's own speedup probe against `0x5`'s 1.7, which is
# what two SMT siblings of one core look like and what two separate cores look
# like. `results/raw/cores-*.txt` carries every one of those readings.
#
#   1 core   0x1     logical 0
#   2 cores  0x5     logical 0, 2
#   4 cores  0x55    logical 0, 2, 4, 6
#   8 cores  0x3555  the six P-cores' primaries plus two E-cores
#  20 cores  0xfffff every logical processor — the unconstrained machine, and
#                    the configuration ADR-0022 §11's row was measured on
MASKS=("1:0x1" "2:0x5" "4:0x55" "8:0x3555" "20:0xfffff")

# The recorded shape, held equal to `experiments/append-condition`'s so that the
# release control is checkable against ADR-0022 §11's row rather than merely
# comparable to it.
export HS_ROUNDS=10
export HS_SEED=5000
export HS_CONTENDERS=64

# The name of the binary's core-constraint test, skipped in every racing launch
# and run alone in its own.
CORES_TEST=the_core_constraint_is_what_was_asked_for

# Locates a built test binary without parsing cargo's human output.
exe_of() {
  local target="$1"
  shift
  cargo test "$@" --test "$target" --no-run --message-format=json 2>/dev/null \
    | grep -o '"executable":"[^"]*"' \
    | sed 's/^"executable":"//; s/"$//; s#\\\\#/#g' \
    | grep -- "/${target}-" \
    | tail -1
}

# Runs one launch of a test binary under one affinity mask.
#
# `affinity-run.ps1` carries the whole of the mechanism and the whole of the
# argument for it; the short version is that it constrains *itself* and lets the
# child inherit, so there is no window in which the child runs unconstrained.
# The mask reaches the binary a second time as `HS_EXPECTED_AFFINITY`, and the
# binary refuses to emit a figure if what the operating system reports for it
# disagrees: an affinity call that silently failed produces a twenty-core number
# wearing a two-core label, which is worse than no number at all.
under_affinity() {
  local decimal="$1" exe="$2" out="$3"
  shift 3
  powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass \
    -File ./affinity-run.ps1 -Mask "$decimal" -Exe "$exe" -Out "$out" -TestArgs "$*"
}

echo "==> the handler transcribes SQLite's own back-off table, not one of its own"
cargo test --lib 2>&1 | tee results/raw/handler-unit.txt

# CONFORMANCE FIRST. AC-001: a figure only counts for a store that passes the
# suite, and a wrong arm is always the fastest. Twice, because this crate
# *replaces the busy handler* — a change to the store, on the write path, in the
# exact place contention resolves — and a handler that gave up an entry early or
# never gave up at all would move every number in `results/` while looking, in a
# timing table, exactly like a fast one. The handler is chosen once per process
# from the environment, so covering both means two invocations rather than two
# modules.
echo
echo "==> CONFORMANCE, under the counting handler — the one every wait_ms figure"
echo "    below is produced by."
cargo test --test arms_are_conformant 2>&1 | tee results/raw/conformance-counting.txt

echo
echo "==> CONFORMANCE, under SQLite's own handler — the one every busy figure"
echo "    below is produced by, including the headroom sweep."
HS_HANDLER=default cargo test --test arms_are_conformant 2>&1 \
  | tee results/raw/conformance-default.txt

debug_exe="$(exe_of busy_margin)"
release_exe="$(exe_of busy_margin --release)"
wakeup_exe="$(exe_of lost_wakeup)"
echo "debug:   $debug_exe"
echo "release: $release_exe"

echo
echo "==> CONTROL: --release, every core, one racing test alone — the shape"
echo "    ADR-0022 §11's 64-contender row was produced under. If this does not"
echo "    land on median 2,724,759 us / max 3,496,577 us / busy=0, every figure"
echo "    below is a different experiment rather than a delta from a known point."
HS_LABEL=control-release-c20-alone HS_ARM=begin-immediate-probe \
  under_affinity 1048575 "$release_exe" results/raw/control-release-c20-alone.txt \
  exactly_one_of_n_contenders_commits --exact --nocapture

echo
echo "==> THE CONTROL'S OWN CONTROL: the identical launch with SQLite's default"
echo "    busy handler and no accounting — what \`experiments/append-condition\`"
echo "    measured under. Installing a handler to measure a handler can perturb"
echo "    the thing measured (a Rust sleep and a Win32 Sleep need not have the"
echo "    same timer resolution), so the difference between this row and the one"
echo "    above is what the instrument costs, measured instead of assumed."
HS_LABEL=control-release-c20-alone-default HS_ARM=begin-immediate-probe HS_HANDLER=default \
  under_affinity 1048575 "$release_exe" results/raw/control-release-c20-alone-default.txt \
  exactly_one_of_n_contenders_commits --exact --nocapture

echo
echo "==> BRIDGE: --release, every core, three racing tests concurrent. Isolates"
echo "    'the gate overlaps its rules' from 'the gate builds in debug'."
HS_LABEL=bridge-release-c20-concurrent HS_ARM=begin-immediate-probe \
  under_affinity 1048575 "$release_exe" results/raw/bridge-release-c20-concurrent.txt \
  --nocapture --skip "$CORES_TEST"

echo
echo "==> THE MATRIX: debug, --test-threads at its default so the three"
echo "    CONTENDERS=64 rules overlap as they do under \`cargo xtask ci\`."
for entry in "${MASKS[@]}"; do
  cores="${entry%%:*}"
  hex="${entry##*:}"
  decimal=$((hex))
  label="debug-c${cores}"

  echo "--> ${cores} core(s), mask ${hex}: the constraint, alone"
  HS_LABEL="$label" \
    under_affinity "$decimal" "$debug_exe" "results/raw/cores-${label}.txt" \
    "$CORES_TEST" --exact --nocapture

  echo "--> ${cores} core(s), mask ${hex}: three racing tests, concurrent"
  HS_LABEL="$label" HS_ARM=begin-immediate-probe \
    under_affinity "$decimal" "$debug_exe" "results/raw/matrix-${label}.txt" \
    --nocapture --skip "$CORES_TEST"
done

echo
echo "==> THE SHIPPED ARM at the worst configuration the matrix found."
echo "    Every row above races \`begin-immediate-probe\`, because that is the arm"
echo "    ADR-0022 §11's number came from and a delta has to hold the arm fixed."
echo "    \`happenstance-sqlite\` ships \`monotonic-guard\` (ADR-0022 §4), so the"
echo "    margin that actually applies to the adapter is measured here too."
HS_LABEL=debug-c20-guard HS_ARM=monotonic-guard \
  under_affinity 1048575 "$debug_exe" results/raw/matrix-debug-c20-guard.txt \
  --nocapture --skip "$CORES_TEST"

echo
echo "==> HEADROOM: the gate's own configuration — debug, three racing tests"
echo "    concurrent, every core — under SQLite's *own* busy handler, swept over"
echo "    the contender count. \`CONTENDERS\` is 64 today and its documentation"
echo "    calls it movable; this says what moving it costs against the 5,000 ms"
echo "    cap. The row where \`busy\` stops being 0 is ADR-0022 §11's own re-open"
echo "    trigger firing, and it is the first time anything in this tree could."
for contenders in 64 96 128 160; do
  echo "--> ${contenders} contenders"
  HS_LABEL="headroom-c20-n${contenders}" HS_ARM=begin-immediate-probe HS_HANDLER=default \
  HS_CONTENDERS="$contenders" HS_ROUNDS=5 \
    under_affinity 1048575 "$debug_exe" "results/raw/headroom-n${contenders}.txt" \
    --nocapture --skip "$CORES_TEST"
done

echo
echo "==> THE RATE AT 64. The sweep above runs each contender count once, and at"
echo "    64 — the count that actually ships — the answer sits on the boundary:"
echo "    the worst race lands within a few per cent of the cap, so whether any"
echo "    contender exhausts it is a coin the host tosses. One sample cannot tell"
echo "    'it does not happen' from 'it did not happen that time', and the"
echo "    difference between those two is the difference between a green suite"
echo "    and an intermittently red one. Six more launches of the identical"
echo "    configuration; \`results/\` reports how many produced busy > 0."
for repeat in 1 2 3 4 5 6; do
  echo "--> repeat ${repeat} of 6, at 64 contenders"
  HS_LABEL="rate-c20-n64-r${repeat}" HS_ARM=begin-immediate-probe HS_HANDLER=default \
  HS_CONTENDERS=64 HS_ROUNDS=5 \
    under_affinity 1048575 "$debug_exe" "results/raw/rate-n64-r${repeat}.txt" \
    --nocapture --skip "$CORES_TEST"
done

echo
echo "==> M-1's lost-wakeup probe. A hang and a timeout exhaustion are the same"
echo "    event from outside, and CF-33 forbids the suite the watchdog that could"
echo "    tell them apart — so the two are measured in one session."
"$wakeup_exe" --nocapture --test-threads=1 2>&1 | tee results/raw/lost-wakeup.txt

echo
echo "Raw rows are under results/raw/, per-contender waits under"
echo "results/raw/waits/. The tables in results/*.md are written by hand from"
echo "them, because a table nobody read is a table nobody checked."
