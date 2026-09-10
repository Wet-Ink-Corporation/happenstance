#!/usr/bin/env python3
"""Between-run dispersion of the paired medians, across N runs of one host.

Why this exists
---------------
`benchmarks/README.md` has carried this for as long as there have been results:

    The wall-clock medians on this host moved by up to 40% between runs while the
    allocation counts stayed identical to the digit.

It is the most consequential sentence in that file -- `benchmarks/src/paired.rs`
exists because of it, `STABLE_DRIFT_MARGIN` is 0.15 because of it, and four
`[PROVISIONAL]` clauses name the absent measurement it implies. And it has never
had an instrument behind it, for a mundane reason: `run.sh` tees to a fixed
`results/raw/overhead.log`, so the evidence for run N is destroyed by run N+1.
`RESULTS_SUFFIX` fixes that; this reads what it leaves.

What it reports, and why not just drift
---------------------------------------
`paired.rs` already computes `ArmReport::drift` -- second-half median over
first-half median -- and prints a caveat when it leaves +/-15%. Quoting drift
alone would UNDERSTATE the problem badly: in the committed
`results/raw/overhead.log` every arm sits at 0.932-1.121, comfortably inside the
margin, so by that measure the old host looks fine. Drift is a WITHIN-run
statistic. The 40% is a BETWEEN-run one, and nothing measured it.

So three things, per arm x scenario cell:

  spread      max/min of the N run medians. Directly comparable to the 40%.
  rMAD        median absolute deviation over the median, as a percentage. Robust
              to one bad run in a way max/min is not, which is why N should be
              10 rather than 5 -- with five, max/min is decided by a single
              interrupt.
  tails       p99/median and max/median, averaged over the N runs. This is where
              a busy machine actually shows: the committed log has `sqlite` at a
              540 us median with a 145 ms max, a ratio of 270x. A 145 ms sample
              in a 540 us arm is a preemption, not a measurement.
  drift       min and max of the per-run drift, for continuity with what
              paired.rs already prints.

It asserts nothing. There is no threshold here on any budget, and there must not
be: CF-34 (`spec/SPECIFICATION.md:8747`) rejects a benchmark result gating a
merge, and every number below is derived from samples this repository measured --
which is exactly the input `ops/host/preflight.sh` is careful never to read. This
prints a table and a human reads it.

Usage
-----
    RESULTS_SUFFIX=01 ./ops/host/bench.sh --fast     # ... 01 through 10
    ./ops/host/flakiness.py benchmarks/results/raw/overhead-*.log

    # comparing two hosts, or two regimes on one host:
    ./ops/host/flakiness.py --label "governor=powersave" raw/overhead-a*.log
    ./ops/host/flakiness.py --label "governor=performance" raw/overhead-b*.log
"""

from __future__ import annotations

import argparse
import re
import sys
from collections import defaultdict

# `  raw/same-schema   n=200  median=  352255ns p95= 628735ns p99= 7139327ns max= 9928703ns   drift=1.015`
ROW = re.compile(
    r"^\s*(?P<arm>\S+)\s+n=(?P<n>\d+)\s+"
    r"median=\s*(?P<median>\d+)ns\s+"
    r"p95=\s*(?P<p95>\d+)ns\s+"
    r"p99=\s*(?P<p99>\d+)ns\s+"
    r"max=\s*(?P<max>\d+)ns\s+"
    r"drift=(?P<drift>[\d.]+)"
)


def median(xs):
    s = sorted(xs)
    n = len(s)
    if n == 0:
        return 0.0
    mid = n // 2
    return float(s[mid]) if n % 2 else (s[mid - 1] + s[mid]) / 2.0


def rmad_pct(xs):
    """Median absolute deviation over the median, as a percentage.

    Robust where max/min is not: one interrupted run moves max/min by its whole
    excursion and moves this by almost nothing.
    """
    m = median(xs)
    if m == 0:
        return 0.0
    return 100.0 * median([abs(x - m) for x in xs]) / m


def parse(path):
    """One file -> {arm: row}. A repeated arm name keeps the FIRST occurrence.

    overhead.log prints several scenario blocks and an arm name can appear in
    more than one. Keeping the first is arbitrary but stable across files, which
    is what matters for comparing run to run; a scenario-aware version would need
    the block headers, and the blocks are not labelled in a machine-readable way.
    """
    out = {}
    for line in open(path, encoding="utf-8", errors="replace"):
        m = ROW.match(line)
        if not m:
            continue
        arm = m.group("arm")
        if arm in out:
            continue
        out[arm] = {k: float(m.group(k)) for k in ("median", "p95", "p99", "max", "drift")}
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("logs", nargs="+", help="results/raw/overhead-*.log from N runs of ONE host")
    ap.add_argument("--label", default="", help="what this set of runs is (host, regime)")
    args = ap.parse_args()

    runs = []
    for p in args.logs:
        rows = parse(p)
        if rows:
            runs.append((p, rows))
        else:
            print(f"note: no paired rows in {p}", file=sys.stderr)

    if len(runs) < 2:
        print("need at least 2 runs; between-run dispersion is not a property of one run", file=sys.stderr)
        return 2

    by_arm = defaultdict(list)
    for _, rows in runs:
        for arm, r in rows.items():
            by_arm[arm].append(r)

    print()
    if args.label:
        print(f"between-run dispersion -- {args.label}")
    else:
        print("between-run dispersion")
    print(f"{len(runs)} runs: {', '.join(p.split('/')[-1] for p, _ in runs)}")
    print()
    print(f"{'arm':<26} {'runs':>4} {'median':>12} {'spread':>8} {'rMAD':>7} "
          f"{'p99/med':>8} {'max/med':>8} {'drift':>13}")
    print("-" * 92)

    for arm in sorted(by_arm):
        rs = by_arm[arm]
        if len(rs) < 2:
            continue
        meds = [r["median"] for r in rs]
        spread = max(meds) / min(meds) if min(meds) else float("inf")
        p99r = median([r["p99"] / r["median"] for r in rs if r["median"]])
        maxr = median([r["max"] / r["median"] for r in rs if r["median"]])
        drifts = [r["drift"] for r in rs]
        print(f"{arm:<26} {len(rs):>4} {median(meds):>11.0f}n "
              f"{spread:>7.2f}x {rmad_pct(meds):>6.1f}% "
              f"{p99r:>7.1f}x {maxr:>7.1f}x "
              f"{min(drifts):>6.3f}-{max(drifts):<6.3f}")

    print()
    print("spread  = max/min of the run medians. The figure comparable to the 40%")
    print("          benchmarks/README.md records for the Windows host.")
    print("rMAD    = median absolute deviation over the median. Robust to one bad run.")
    print("p99/med, max/med = the tails, median over runs. This is where a busy")
    print("          machine shows: a 145 ms sample in a 540 us arm is a preemption.")
    print("drift   = min-max of paired.rs's own within-run statistic, for continuity.")
    print()
    print("Nothing here is a threshold and nothing here gates anything. CF-34.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
