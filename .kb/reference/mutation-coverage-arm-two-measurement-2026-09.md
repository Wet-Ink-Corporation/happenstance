---
id: kb-reference-mutation-coverage-arm-two-001
title: ES-22's landed == 0 arm is reached by two registered stores on every run, in both feature configurations
kind: reference
status: accepted
authority_tier: note
summary: >-
  A 2026-09-04 measurement mutating arm 2 of dropped_append_future_leaves_no_partial_batch
  (the byte-identical snapshot_of(&after) == snapshot_of(&before) comparison) so
  it must fail if reached at all. Under both --all-features and --no-default-features,
  cargo test -p happenstance-testkit fails identically: PreCommitPositionStore and
  AwaitAcrossBorrowStore both reach and pass arm 2, and have since the rule's first
  commit d480446. YieldingRowAtATimeStore, the store filed under the rule's own
  axis, lands on arm 1 instead. A companion check on the neighbouring rule
  append_is_atomic_under_a_mid_batch_fault finds no equivalent gap.
depends_on: []
related:
  - kb-decision-0040
  - kb-decision-0010
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/es-22-arm-two-is-reached-the-finding-is-wrong.md
  - .kb/_intake/f2-5-holds-the-release-for-phase-10.md
last_reviewed: 2026-09-07
---

# ES-22's landed == 0 arm is reached by two registered stores on every run, in both feature configurations

## What this is a pointer to

The instrument is a temporary mutation of
`dropped_append_future_leaves_no_partial_batch`'s arm 2 in
`crates/happenstance-testkit/src/suite.rs`, applied and reverted in the same
session; `git status` was clean of it afterward. This atom is the citable
summary of the measurement it produced. The conclusion drawn from it — that
audit finding F2-5's claim "arm 2 has never run against anything" is false, and
what residual survives the refutation — belongs to the decision record that
cites this atom (`kb-decision-0040`), not to this atom.

## What was measured, and how

F2-5's actionable claim was: only `YieldingRowAtATimeStore` suspends inside
`append`, its suspension point sits at the *end* of the loop body — after the
write — so it lands on `landed == 1`, fails arm 1, and arm 2 is never reached
by anything. To test this, arm 2's right-hand side was replaced with an empty
vector, which cannot equal the left because the rule seeds an `Existing` row
before it starts — so any store reaching that line fails there by construction.

`cargo test -p happenstance-testkit --all-features` failed immediately:
`PreCommitPositionStore` panicked on the mutated arm with `left` carrying the
seeded one-row snapshot and `right` empty — a real, non-vacuous comparison
between two populated snapshots, exactly what the rule's own prose demands.
`cargo test -p happenstance-testkit --no-default-features --test
mutation_coverage` reproduced the identical failure, so the finding was not a
stale observation about one feature configuration.

Because the assertion in `mutants_fail_exactly_their_declared_rules` fires
eagerly, the panic named only the first offender. Replacing the mutation with a
print statement instead of a panic left every store running to completion and
printing `ARM2-REACHED after=["Existing"]` twice — exactly two stores, and only
two, reach arm 2:

| Store | Suspension point | `landed` after one poll |
|---|---|---|
| `YieldingRowAtATimeStore` | end of the loop body, after the row is written | 1 — arm 1, its declared axis |
| `PreCommitPositionStore` | after allocating the position, before publishing to `committed` | 0 — arm 2 |
| `AwaitAcrossBorrowStore` | after `borrow_mut()`, before `correct::commit` | 0 — arm 2 |

Both stores that reach arm 2 pass it: their declared defects are on other axes
(position visibility and a borrow held across an await respectively), and their
own doc comments describe their suspension point as that defect's mechanism
rather than as anything to do with cancellation — which is why reading the
corpus by declared axis, as the original finding did, misses both. `git log -S`
places `PreCommitPositionStore` and the rule itself in the same commit,
`d480446` (2026-08-08): arm 2 has been executing since the hour it was written.

A companion check applied the identical print-instead-of-panic method to
`append_is_atomic_under_a_mid_batch_fault`. Both of its arms execute, three
times between them, and the `Err` arm carries both a passing store
(`GappedPositionStore`) and a failing one (`NoTransactionStore`) — the strongest
form, decisive rather than merely reached. No comparable gap exists there.

## What the measurement does and does not establish

It establishes, mechanically and reproducibly, that arm 2 is not dead code: it
runs on every `mutation_coverage` invocation in both feature configurations and
is exercised by exactly two of the suite's registered stores. It does **not**
establish that this coverage is sufficient — both stores reaching arm 2 are
`Kind::Mutant` on other axes, not `Kind::ConformantVariant`, so nothing in the
portfolio would catch arm 2 over-specifying (rejecting a legal store). Nor does
it touch F2-5's other, separate claim, which the measurement leaves untouched:
every store that has ever reached either arm of the rule is an in-process
`Rc<RefCell<...>>` in the testkit's own test target, where "the future was
dropped" means a local went out of scope rather than a connection was severed —
a gap the built `happenstance-postgres` adapter later closed, per
`f2-5-holds-the-release-for-phase-10`.
