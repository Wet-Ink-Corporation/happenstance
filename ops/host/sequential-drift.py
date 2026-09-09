#!/usr/bin/env python3
"""Is a ratio between two arms of one SEQUENTIAL criterion run reproducible?

The question this answers, and why it is the one that matters
-------------------------------------------------------------
`benchmarks/src/paired.rs` exists for exactly one recorded reason. RUNBOOK.md:

    the sequential design this criterion specified -- arms first, baseline
    re-run last to bound drift -- FAILED: the baseline moved 2.7x at one client
    and 3.0x at 64, larger than two of the three effects.

That is why `benchmarks/README.md` rules "absolutes from criterion, ratios from
the paired runner, never mixed in one table". The rule is a consequence of a
measurement, so it is falsifiable by another one: if a sequential criterion run
no longer drifts, criterion can supply the ratios and the paired runner is not
needed on that host.

That matters here because on a host whose clocksource is `hpet` the paired
runner does not work at all -- its own control,
`the_paired_sampler_sees_a_difference_it_was_given`, fails there. So "is the
paired runner still necessary" and "does the clocksource need fixing" are the
same question.

What it computes
----------------
Input is N snapshots of one criterion target, taken from criterion's own
`target/criterion/**/new/estimates.json` so that arms are matched BY NAME. That
is not a detail: matching by position silently pairs different benchmarks when a
run emits a different number of lines, which is how the first attempt at this
went wrong.

    for i in 1 2 3; do
      rm -rf target/criterion
      cargo bench --bench store_append >/dev/null 2>&1
      python3 snap.py > named-$i.tsv        # id<TAB>median_ns per estimates.json
    done

Two statistics:

  per-arm spread   max/min of one arm's median across the N runs. Reproducibility
                   of an absolute.
  ratio spread     for every PAIR of arms, max/min of (arm_b / arm_a) across the
                   N runs. This is the statistic a ratio table depends on, and
                   the one RUNBOOK.md's 2.7-3.0x was about. If the machine drifts
                   during a run, arms measured far apart move relative to each
                   other and this is where it shows.

It asserts nothing and gates nothing. CF-34.
"""

from __future__ import annotations

import argparse
import statistics as st
import sys


def load(path):
    d = {}
    for line in open(path, encoding="utf-8"):
        key, _, value = line.rstrip("\n").partition("\t")
        if value:
            try:
                d[key] = float(value)
            except ValueError:
                pass
    return d


def quantile(sorted_values, q):
    if not sorted_values:
        return float("nan")
    i = min(len(sorted_values) - 1, int(q * len(sorted_values)))
    return sorted_values[i]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("snapshots", nargs="+", help="named-N.tsv, one per run of ONE target")
    ap.add_argument("--drift", type=float, default=2.7,
                    help="the recorded sequential drift to test against (default 2.7)")
    args = ap.parse_args()

    runs = [load(p) for p in args.snapshots]
    if len(runs) < 2:
        print("need at least 2 runs", file=sys.stderr)
        return 2

    common = sorted(set.intersection(*(set(r) for r in runs)))
    if not common:
        print("no arm names common to every run", file=sys.stderr)
        return 2

    print(f"\n{len(runs)} runs, {len(common)} arms matched by name\n")

    spread = {k: max(r[k] for r in runs) / min(r[k] for r in runs) for k in common}
    vals = sorted(spread.values())
    print("per-arm spread (max/min of the run medians)")
    print(f"  median {st.median(vals):.4f}x   p90 {quantile(vals, .9):.4f}x   "
          f"worst {vals[-1]:.4f}x")
    print("  worst five:")
    for k in sorted(common, key=lambda k: -spread[k])[:5]:
        med = sorted(r[k] for r in runs)[len(runs) // 2]
        print(f"    {spread[k]:.4f}x  {med / 1000:9.2f} us  {k}")

    pairs = []
    for a in range(len(common)):
        for b in range(a + 1, len(common)):
            ka, kb = common[a], common[b]
            ratios = [r[kb] / r[ka] for r in runs]
            pairs.append(max(ratios) / min(ratios))
    pairs.sort()

    print("\nratio spread over every arm pair -- what a ratio table depends on")
    print(f"  {len(pairs)} pairs")
    print(f"  median {st.median(pairs):.4f}x   p90 {quantile(pairs, .9):.4f}x   "
          f"p99 {quantile(pairs, .99):.4f}x   worst {pairs[-1]:.4f}x")
    over = sum(1 for x in pairs if x > args.drift)
    print(f"\n  pairs whose ratio moved more than {args.drift}x "
          f"(the recorded sequential drift): {over} of {len(pairs)}")
    if over == 0:
        print(f"  -> no pair anywhere in this run drifted like the discarded pass did.")
    print("\nNothing here is a threshold and nothing here gates anything. CF-34.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
