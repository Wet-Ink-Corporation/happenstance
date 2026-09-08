---
id: kb-decision-0054
title: after_opt applies to every guard, is pinned by tests, and gains a name that says so
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0054
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  The blanket-scope behaviour is kept and pinned rather than changed: after_opt rewrites the after field of every guard already present, and calling it after and_guard silently widens what the condition tolerates. Two unit tests pin the hazardous order, both doc blocks name the erasure, and a scope-carrying spelling is added with the old name deprecated. Changing the semantics to fill-only-None is rejected because it contradicts VT-30's MUST and ADR-0012's restatement. The behaviour was observed and unpinned: an edit changing it would have passed green across the workspace.
depends_on:
  - kb-decision-0012
related:
  - kb-open-question-vt-30-marker-stale-unscheduled-001
  - kb-concept-torn-read-append-boundary-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/after-opt-scope.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# after_opt applies to every guard, is pinned by tests, and gains a name that says so

## Decision

`AppendCondition::after_opt` keeps its blanket scope: it rewrites the `after` field of
*every* guard the builder currently holds, discarding whatever boundary `and_guard` had
attached to each one. That is not incidental — VT-30 `[PROVISIONAL]`
(`spec/SPECIFICATION.md:1861-1867`) says `after`/`after_opt` "MUST continue to apply the
given boundary to every guard," and a `[PROVISIONAL]` clause binds until the thing that
would falsify it happens (`spec/SPECIFICATION.md:198-200`). ADR-0012 §9 restates the same
sentence in an accepted, immutable record (`references/adr/0012-append-shape-and-preconditions.md:604-608`).
So the semantics are not open here; only the spelling and the pinning are.

Two unit tests are added to `crates/happenstance-core/src/append.rs`'s existing `mod
tests`: one asserting that `new(q1).and_guard(q2, Some(p2)).after(p1)` leaves both guards
at `p1` — the hazardous order, which raises a quiet guard's boundary and widens what the
condition tolerates, exactly the lost-update shape VT-30's `Rejects:` clause exists
against — and one asserting the safe order (`after_opt` before `and_guard`) keeps
boundaries independent. Before this decision the behaviour was observed, not pinned: an
edit changing `after_opt` from "overwrite every guard" to "fill only guards whose `after`
is `None`" passes every test in the workspace, because every call site either carries one
guard, calls `after_opt` before any `and_guard`, or passes `None`. That is the plausible
wrong implementation CLAUDE.md's rule asks a new test to reject.

Both doc blocks — `and_guard`'s and `after_opt`'s — gain a sentence stating the erasure and
the order that causes it; today neither says it, and the type's own example (`append.rs:92-94`)
only ever writes the safe order.

A scope-carrying name (`after_every_guard` / `after_every_guard_opt`) is introduced
alongside the existing `after`/`after_opt`, with `#[deprecated]` on the old spellings. This
is additive (minor) and available before and after `0.2.0`, unlike a hard rename, which
would break roughly fifteen in-tree call sites and two source-substring gate tests
(`xtask/tests/first_encounter.rs:423, :696`) for no more safety than a warning buys. The
alias survives whatever VT-30 later does: if the guard sequence is ever withdrawn, the
alias dies in the same edit that removes `and_guard`; if VT-30's marker eventually lifts on
Postgres and benchmark-harness evidence, the alias is renamed or deleted in that same pass.

## Alternatives rejected

**Changing the semantics** so `after_opt` fills only guards whose `after` is `None` — the
option this decision exists to foreclose — contradicts a binding `[PROVISIONAL]` MUST and
its restatement in `kb-decision-0012`, and it is precisely the edit the new pin tests are
written to reject. **A hard rename** (`after_every_guard`, breaking) was declined in favour
of the deprecated alias: it spends a break for a warning rather than a compile error, and
the workspace's own precedent against deprecated arms (`CHANGELOG.md:1697-1700`) rested on
a premise — nothing yet published — that has since expired, which argues for reconsidering
the taste, not for repeating the refusal. **Reshaping `AppendCondition` so the blanket
setter is unreachable after any guard is added** (a builder-state change making the
hazardous order a compile error) is not taken here: it is real, it is the only option that
converts the hazard into a diagnostic rather than a warning, and it costs the widest
call-site break of the four considered. It stays open, tracked separately, because it also
needs evidence about whether a future replication ingest policy wants to re-blanket an
already-assembled condition — a use case only `happenstance-sync`, still unwritten, would
exercise.

## What this does not settle

Whether the builder-state option is ever taken; whether VT-30's own `[PROVISIONAL]` marker
lifts, on Postgres and multi-guard-benchmark evidence that does not yet exist; and whether
the benchmark harness gains a multi-guard scenario. None of those are decided by pinning
today's mandated behaviour under a name that says what it does.
