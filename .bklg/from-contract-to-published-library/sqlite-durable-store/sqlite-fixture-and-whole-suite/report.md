---
item: "HS-S0040"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — SqliteFixture and the whole conformance suite

## Findings Ledger

**Eight of eight ACs satisfied. `happenstance-sqlite` has passed the bar.** 89
rules, 0 failed, 0 ignored, against a real file on disk, through two real
`rusqlite::Connection`s, with an acknowledged write surviving a genuine reopen.
It is the first adapter in this workspace that has run the suite at all.

**Mount point:** `crates/happenstance-sqlite/tests/conformance.rs` — a new
integration target whose whole content is `SqliteFixture` and
`happenstance_testkit::event_store_conformance!(SqliteFixture::new())`.
Auto-discovered by `cargo test -p happenstance-sqlite`, so
`cargo xtask affected --base main` reaches it with no script edit.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the bar is cleared whole | **Met** | **89 passed, 0 failed, 0 ignored.** 89 is the whole of `for_each_event_store_rule!`, checked rather than assumed: the reference harness emits 91 from the same macro, two of which are its own `fixture_isolation` module. Exactly two rules report `Skipped`, both `MID_BATCH_FAULT` |
| **AC-002** — two fixtures are two stores | **Met** | `two_fixture_instances_observe_none_of_each_others_appends` green, **plus** `SqliteFixture::new` minting one fresh temp path per instance from a process-local ordinal — the half no rule can see |
| **AC-003** — the second handle is a second connection | **Met** | `two_handles_observe_each_others_appends` (a MUST that panics rather than skips) and `head_advances_across_two_handles` green, **plus** `connect` calling `SqliteEventStore::open`, never `open_in_memory` and never an `Arc` clone. This fills the handle-multiplicity far end `RUNBOOK.md:691` records as empty |
| **AC-004** — an acknowledged write outlives its process state | **Met** | The three reopen rules green, **plus** a `reopen` that closes every connection this fixture opened and checkpoints the WAL with `TRUNCATE`, and never deletes or recreates the file. This fills the durability far end `RUNBOOK.md:692` records as empty |
| **AC-005** — the ceilings are stated numbers | **Met** | `append_reports_exceeded_store_limits` observed as **`Ran`**, not the `NO_STORE_LIMITS` skip that reads as a pass to anyone with only the exit code. The three constants are **mirrored** from the adapter's own, so the declared and enforced numbers cannot diverge |
| **AC-006** — the declined capability speaks for itself | **Met** | Both gated rules present in the binary, both printing a `SKIP` line quoting this fixture's own sentence — which names ES-35's live residual falsifier — rather than the trait's generic default |
| **AC-007** — it went green by passing | **Met, with one change recorded** | No `#[ignore]`, no timeout, no sleep. `git diff --stat HEAD -- crates/happenstance-testkit crates/happenstance-core spec .kb` **empty**. `shapes.rs` green unchanged. One SQL-path change, in the opposite direction to what the criterion guards against — see below |
| **AC-008** — the numbers are facts and CF-40 stays open | **Met** | The three ceilings and the reasoning for each are recorded in AC-005's evidence and in the constants' own rustdoc. `git diff --name-only HEAD -- .kb/` **empty**; `redkiln validate --kb` passed |

## The finding, which is why this story exists

The first conformance run was green. Per CLAUDE.md that is a claim to check, and
the third check — *is it green repeatably?* — said no:

```text
test result: ok.     89 passed; 0 failed
test result: FAILED. 87 passed; 2 failed
test result: FAILED. 87 passed; 2 failed
test result: FAILED. 88 passed; 1 failed
```

Roughly one run in two, on `read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot`. **It is a real ES-11 defect, not flakiness.**
The clause requires the position ceiling to be sampled *no later than the first
poll*, and the adapter sampled it inside the `spawn_blocking` hop — which runs
after the first poll returns. The rules poll once (legally receiving `Pending`),
then append, then drain; the append could land before the deferred sample, and
the late event was then below the ceiling and visible to the read.

Neither of this project's own read targets could see it: both drain hundreds of
events before appending, by which time the sample is long taken. **Only the
conformance suite asks the question in the shape that fails.**

The fix moved the sample onto the polling thread, before the spawn
(`ReadCursor::sample_ceiling`, `crates/happenstance-sqlite/src/event_store.rs:1189-1204`).
Thirteen conformance runs green since. A regression test that polls exactly once
and then appends is now at
`crates/happenstance-sqlite/tests/read.rs::an_append_after_a_single_poll_is_not_observed`,
so the defect cannot return silently.

**On AC-007's "no SQL body was edited to make a rule pass":** the criterion
exists to stop an implementer weakening a store until a rule stops asking. This
is the reverse — the rule asked, the adapter was wrong, and the adapter was made
*stricter*. The rule is unchanged, the testkit is untouched, and the guarantee is
stronger than it was. Recorded here rather than folded into a green tick.

## Deferred, and to whom

- **CF-40's clause home** stays open. This story needed the *capability* to
  declare numeric limits and gets no say in which ADR owns the clause;
  `.kb/open-questions/cf-40-fixture-limits-ownership.md` is untouched and
  escalated to the ADR queue.
- **ES-35's residual falsifier** — a store that loses a write to a *fault* rather
  than to an instruction — is not retired by this project, and the
  `MID_BATCH_FAULT` declension says so in the fixture's own words. The verdict on
  the marker is `reopen-negative-control-and-durability-verdicts`'.
- **The concurrency family and the model family** are not mounted here: they are
  `concurrency-family-and-contender-count`'s and
  `model-family-and-mutant-pass-column`'s. This story mounts
  `event_store_conformance!` and nothing else.

## What is still `todo!()`

The projection store's `migrate`, `checkpoint`, `commit` and `rollback`. Every
`todo!()` on the **event store** path is gone, and
`#![allow(clippy::todo)]` at `crates/happenstance-sqlite/src/lib.rs:83` dies with
the last of those four, in `instrument-markers-removed-and-gate-green`.
