#!/usr/bin/env python3
"""Between-run spread of criterion point estimates, for one regime.

Companion to `flakiness.py`, which does the same for the paired runner's
`overhead.log`. This one exists because on a host whose clocksource is `hpet`
the paired runner is not usable -- its own control,
`the_paired_sampler_sees_a_difference_it_was_given`, fails there -- while
criterion is, because criterion batches iterations and amortises the clock.
Losing one instrument is not a reason to answer the question with none.

Input is one file per run, each holding criterion's point estimates in order,
one per line, as `<value> <unit>`:

    cargo bench --bench typed_codec -- --warm-up-time 1 --measurement-time 3 \\
      | grep -oE 'time:   \\[[0-9.]+ [num]s' | sed -E 's/time:   \\[//' > run-1.txt

Arms are matched BY POSITION, not by name. Criterion prints its benchmark id on a
separate line from its timing and the ids are not on the captured lines, so the
n-th estimate in one file is the n-th in another only because criterion runs the
group in a fixed order. That holds for repeated runs of one unchanged binary,
which is the only comparison this makes; it does NOT hold across a code change
that adds or reorders an arm, and comparing two such sets would silently pair
different benchmarks. Check the line counts match before believing the output --
the tool refuses when they do not.

It asserts nothing and gates nothing. CF-34.
"""

from __future__ import annotations

import argparse
import sys

UNIT = {"ns": 1.0, "us": 1e3, "\u00b5s": 1e3, "ms": 1e6, "s": 1e9}


def load(path):
    out = []
    for line in open(path, encoding="utf-8"):
        parts = line.split()
        if len(parts) != 2:
            continue
        try:
            out.append(float(parts[0]) * UNIT[parts[1]])
        except (KeyError, ValueError):
            continue
    return out


def median(xs):
    s = sorted(xs)
    n = len(s)
    return 0.0 if n == 0 else (float(s[n // 2]) if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2.0)


def rmad_pct(xs):
    m = median(xs)
    return 0.0 if m == 0 else 100.0 * median([abs(x - m) for x in xs]) / m


def summarise(label, paths):
    runs = [load(p) for p in paths]
    runs = [r for r in runs if r]
    if len(runs) < 2:
        print(f"{label}: need at least 2 non-empty runs", file=sys.stderr)
        return None
    n = len(runs[0])
    if any(len(r) != n for r in runs):
        print(f"{label}: runs have different arm counts {[len(r) for r in runs]} -- "
              f"refusing to pair by position", file=sys.stderr)
        return None

    rows = []
    for i in range(n):
        vals = [r[i] for r in runs]
        lo, hi = min(vals), max(vals)
        rows.append((median(vals), hi / lo if lo else float("inf"), rmad_pct(vals)))
    return rows


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--a-label", default="A")
    ap.add_argument("--b-label", default="B")
    ap.add_argument("--a", nargs="+", required=True, help="regime A run files")
    ap.add_argument("--b", nargs="+", required=True, help="regime B run files")
    args = ap.parse_args()

    a = summarise(args.a_label, args.a)
    b = summarise(args.b_label, args.b)
    if a is None or b is None:
        return 2
    if len(a) != len(b):
        print("the two regimes have different arm counts; not comparable", file=sys.stderr)
        return 2

    print()
    print(f"criterion between-run spread: {len(args.a)} runs of {args.a_label} "
          f"vs {len(args.b)} of {args.b_label}")
    print()
    print(f"{'arm':>4}  {args.a_label:>28}  {args.b_label:>28}")
    print(f"{'#':>4}  {'median':>10} {'spread':>7} {'rMAD':>7}  "
          f"{'median':>10} {'spread':>7} {'rMAD':>7}   {'faster':>7}")
    print("-" * 84)

    worst_a = worst_b = 0.0
    for i, (ra, rb) in enumerate(zip(a, b), 1):
        ma, sa, da = ra
        mb, sb, db = rb
        worst_a = max(worst_a, sa)
        worst_b = max(worst_b, sb)
        print(f"{i:>4}  {ma:>9.1f}n {sa:>6.3f}x {da:>6.2f}%  "
              f"{mb:>9.1f}n {sb:>6.3f}x {db:>6.2f}%   {ma / mb if mb else 0:>6.2f}x")

    print("-" * 84)
    print(f"{'worst':>4}  {'':>10} {worst_a:>6.3f}x {'':>7}  {'':>10} {worst_b:>6.3f}x")
    print()
    print("spread = max/min of the run medians for that arm. The statistic")
    print("         benchmarks/README.md's '40% between runs' is comparable to.")
    print("rMAD   = median absolute deviation over the median; robust to one bad run.")
    print("faster = A median / B median. Not the point of the table, but free.")
    print()
    print("Nothing here is a threshold and nothing here gates anything. CF-34.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
