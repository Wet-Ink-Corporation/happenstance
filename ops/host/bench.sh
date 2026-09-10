#!/usr/bin/env bash
#
# Runs `benchmarks/run.sh` under the declared conditions, and records what those
# conditions actually were.
#
# It adds no threshold and reads no result. What it does is: assert the machine
# first, pin the process to one thread per physical core, run the harness
# unmodified, and write the before/after throttle and thermal counters beside the
# output so a run that was interfered with can be RECOGNISED afterwards rather
# than silently averaged in.
#
# The distinction that keeps this inside CF-34: recognising a throttled run is
# not the same as rejecting one. Nothing here decides whether a figure counts --
# `preflight.sh --assert` decides whether the run STARTS, on conditions read
# before the first sample exists, and everything after the run is reported for a
# human. `benchmarks/run.sh:13-20` stays true: it is not a gate step, and neither
# is this.
#
#   usage: ./bench.sh [--fast] [--suffix NAME]
#
# --suffix names this run's copies of the raw output, so N repetitions do not
# overwrite each other. That is what the flakiness measurement in
# `benchmarks/results/flakiness/` needs and what a single run does not.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
# shellcheck source=./host.env
. "${HOST_ENV:-$here/host.env}"

# rustup installs into ~/.cargo/bin and 10-toolchain.sh deliberately passes
# `--no-modify-path`. A non-interactive `ssh host cmd` does not read the profile
# that would otherwise add it -- Ubuntu's .bashrc returns early when not
# interactive -- so without this every toolchain row below reads ABSENT on a host
# where the toolchain is installed and fine. A preflight that reports a hole it
# invented is worse than one that reports nothing.
case ":$PATH:" in
  *":$HOME/.cargo/bin:"*) ;;
  *) PATH="$HOME/.cargo/bin:$PATH"; export PATH ;;
esac

suffix=""
args=()
while [ $# -gt 0 ]; do
  case "$1" in
    --suffix) suffix="$2"; shift 2 ;;
    *) args+=("$1"); shift ;;
  esac
done

stamp="$(date -u +%Y%m%dT%H%M%SZ)"
tag="${suffix:-$stamp}"
outdir="$repo/benchmarks/results/raw"
mkdir -p "$outdir"

throttle_counters() {
  local c total=0 v
  for c in /sys/devices/system/cpu/cpu[0-9]*/thermal_throttle/core_throttle_count; do
    [ -r "$c" ] || continue
    v="$(cat "$c")"
    total=$((total + v))
  done
  echo "$total"
}

temp_now() { sensors -u 2>/dev/null | awk '/_input/{print int($2); exit}'; }

# --- 1. the machine, asserted before anything is measured -------------------
echo "==> preflight: the declared conditions, before the first sample exists"
PREFLIGHT_REPORT="$outdir/host-conditions${suffix:+-$suffix}.txt" \
  "$here/preflight.sh" --assert

# --- 2. affinity -------------------------------------------------------------
# One thread per physical core. On this host SMT siblings are (0,1) (2,3) ...
# so the even CPUs are one-per-core -- the same 0x55-shaped mask
# experiments/busy-timeout-margin/ already uses, and whose README is emphatic
# that a mask must be READ BACK off the live process rather than trusted from
# the flag that set it. `taskset -p` below is that read-back.
if [ "$SMT" = "on" ]; then
  mask="0,2,4,6"
else
  mask="0,1,2,3"
fi

echo
echo "==> affinity: taskset -c $mask (one thread per physical core)"

pre_throttle="$(throttle_counters)"
pre_temp="$(temp_now)"

# --- 3. the harness, unmodified ---------------------------------------------
echo
echo "==> benchmarks/run.sh ${args[*]:-}"
set +e
RESULTS_SUFFIX="$suffix" taskset -c "$mask" bash "$repo/benchmarks/run.sh" "${args[@]:-}"
status=$?
set -e

# --- 4. what the machine did while it ran, REPORTED ------------------------
post_throttle="$(throttle_counters)"
post_temp="$(temp_now)"

after="$outdir/host-after${suffix:+-$suffix}.txt"
{
  echo "# What the machine did during the run. Reported, never judged: nothing"
  echo "# here decides whether a figure counts. A non-zero throttle delta means"
  echo "# read the wall-clock columns with suspicion and re-run; it does not mean"
  echo "# the run failed, and no threshold turns it into one."
  echo "run                  $tag"
  echo "exit_status          $status"
  echo "affinity_mask        $mask"
  echo "affinity_readback    $(taskset -p $$ 2>/dev/null | sed 's/.*: //' || echo unknown)"
  echo "throttle_before      $pre_throttle"
  echo "throttle_after       $post_throttle"
  echo "throttle_delta       $((post_throttle - pre_throttle))"
  echo "temp_before_c        ${pre_temp:-unknown}"
  echo "temp_after_c         ${post_temp:-unknown}"
  echo "governor             $GOVERNOR"
  echo "epp                  $EPP"
  echo "max_freq_khz         ${MAX_FREQ_KHZ:-uncapped}"
  echo "smt                  $SMT"
} > "$after"

echo
cat "$after"
echo
echo "conditions: $outdir/host-conditions${suffix:+-$suffix}.txt"
echo "aftermath:  $after"

exit "$status"
