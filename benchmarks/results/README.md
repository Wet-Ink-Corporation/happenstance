# `benchmarks/results/`

Everything here comes from one `./run.sh`. Nothing is carried across runs.

## How to read this directory

| Path | What it is | Who wrote it |
| --- | --- | --- |
| [`GRADES.md`](GRADES.md) | The reader-facing table: what to expect, in orders of magnitude | by hand, from `raw/` |
| `<topic>.md` | Per-group tables with their caveats | by hand, from `raw/` |
| `raw/*.txt`, `raw/*.csv` | Command output, tee'd verbatim | `run.sh` |
| `history/<date>-<commit>.json` | One entry per complete run | `src/bin/collect.rs` |

The tables are written **by hand** from `raw/`, because a table nobody read is a
table nobody checked. That is `experiments/one-connection-latency/run.sh`'s rule
and it holds here for the same reason.

## The conditions are in one place, and they are not optional

`../README.md#conditions` — machine, OS, filesystem, toolchain, build profile,
and the SQLite pragmas **read back off the live connection** rather than trusted
from the `PRAGMA` that issued them. Every CSV in `raw/` also carries them in
`#`-prefixed comment lines above its header, so a figure and its conditions
cannot be separated by copying one of them.

`.kb/reference/README.md:22-28` is the rule this enforces:

> A measurement without a commit sha or a date is not a weaker reference, it is
> a false one.

## `representative` is a column, and `false` is not a failure

Every row carries it. It is `false` when the figure came from
`corpus::Regime::Interned` — the control arm, not the workload — or when the
paired run it came from drifted, or when the arm's median sat within 10× of the
timer's own cost.

A row that cannot be a headline is still committed. It is what makes the
representative rows' provenance checkable, and dropping it would leave a reader
unable to tell "we did not measure that" from "we measured it and did not like
the answer".

## The discarded passes are kept

`RUNBOOK.md:1649-1650`: **commit your evidence, or it is not evidence.** A run
whose drift figures put it outside `paired::STABLE_DRIFT_MARGIN` is committed
under `raw/` like any other, with its own `NOTE:` line saying so. The
position-visibility experiment keeps its failed sequential pass under
`results/discarded-sequential/` for the same reason.

## What a history entry is for

CF-34: *"benchmarks are published per adapter and compared against that
adapter's own history."* Two entries in `history/` are a diff. Each is named for
the **run's** UTC date and the commit it was taken at, with `-dirty` appended
when the tree carried uncommitted changes — a figure taken against uncommitted
work is not reproducible by anybody, and the honest way to say so is to record
it.

criterion's own `--save-baseline` is used too and is the better tool for *"did
this change make it slower"* within one afternoon on one machine. What it does
not give is a committed, portable, human-readable record: a criterion baseline
lives under `target/`, is not in the repository, and is unreadable without
criterion.
