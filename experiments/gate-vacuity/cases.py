#!/usr/bin/env python3
"""Two counts over `spec/E2E-CASES.md` and `spec/SPECIFICATION.md`, no build.

CF-37 is `[FROZEN]`: every E2E case MUST name the clause or clauses it
exercises.  CF-38 is `[FROZEN]` and enumerates five conditions the traceability
check MUST fail on, the fourth being *a case naming no clause*.  Finding Q-04
says 54 of 58 cases name none; the refutation says the number that matters is
the inverse relation -- clauses' `Cases:` lines -- and that its orphan count is
zero.  Both are cheap to compute and they are different questions, so this
prints both and labels which is which.

It reads text and builds nothing, so it is the one arm of this experiment that
needs no worktree.  Run it against the pinned tree:

    python experiments/gate-vacuity/cases.py <repo-root>

`cases_of`'s `none`-prefix discard is reproduced from `spec_trace.rs`; the rest
of that file's parsing is not, so treat the orphan number as a corroboration of
the refutation's method rather than as an independent re-implementation of it.
"""

from __future__ import annotations

import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".")
cases_md = (root / "spec" / "E2E-CASES.md").read_text(encoding="utf-8")
spec_md = (root / "spec" / "SPECIFICATION.md").read_text(encoding="utf-8")

CLAUSE_ID = re.compile(r"\b(?:ES|VT|WF|PS|SY|CF)-\d+\b")

# One entry per `### E2E-nn` heading, holding everything up to the next heading
# of the same or a higher level.
heads = list(re.finditer(r"^### (E2E-\d+)\b(.*)$", cases_md, re.M))
bodies: dict[str, str] = {}
for i, h in enumerate(heads):
    end = heads[i + 1].start() if i + 1 < len(heads) else len(cases_md)
    bodies[h[1]] = cases_md[h.start() : end]

nameless = [cid for cid, body in bodies.items() if not CLAUSE_ID.search(body)]

# The inverse relation: every case any clause's `Cases:` field claims.
#
# `field_line` (`spec_trace.rs:1503-1541`) joins the marker's own line with every
# following line until a blank one, another field head, a heading or a bold run,
# so a `Cases:` list wrapped across two lines claims everything on both. A first
# cut of this file read the marker line alone and reported four orphans that are
# not orphans; the continuation is the whole difference.
FIELD_HEAD = re.compile(r"^[-*`\s]*([A-Za-z][A-Za-z0-9 /-]*):")


def field_head(line: str) -> str | None:
    m = FIELD_HEAD.match(line)
    return m[1] if m else None


claimed: set[str] = set()
spec_lines = spec_md.splitlines()
for i, line in enumerate(spec_lines):
    if field_head(line) != "Cases":
        continue
    text = line.split(":", 1)[1].strip().lstrip("`*").strip()
    for nxt in spec_lines[i + 1 :]:
        t = nxt.strip()
        if not t or field_head(nxt) or t.startswith("#") or t.startswith("**"):
            break
        text += " " + t
    if text.lower().startswith("none"):
        continue
    claimed.update(m[0] for m in re.finditer(r"\bE2E-\d+\b", text))

orphans = sorted(set(bodies) - claimed, key=lambda c: int(c.split("-")[1]))

print(f"cases: {len(bodies)}")
print(f"cases whose body contains no clause identifier: {len(nameless)}")
print(f"  {' '.join(sorted(nameless, key=lambda c: int(c.split('-')[1])))}")
print(f"cases claimed by at least one clause `Cases:` line: {len(claimed & set(bodies))}")
print(f"orphans (a case no clause claims): {len(orphans)}")
print(f"  {' '.join(orphans) if orphans else '(none)'}")
print(f"per-case `Clauses:` fields present: {cases_md.count('Clauses:')}")
