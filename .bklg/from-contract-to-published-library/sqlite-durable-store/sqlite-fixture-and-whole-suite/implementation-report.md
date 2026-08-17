---
item: "HS-S0040"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — SqliteFixture and the whole conformance suite

> **STATUS: eight of eight ACs satisfied.** `happenstance-sqlite` has passed the
> bar. **89 rules, 0 failed, 0 ignored**, against a real file on disk, through
> two real `rusqlite::Connection`s, with an acknowledged write surviving a
> genuine reopen.
>
> **And the mount found a real defect on its first run** — an intermittent one,
> which is the kind that gets called flakiness and deleted. It is written up
> below, because it is the single best piece of evidence that this story was
> worth doing.

## TDD Evidence

This story's test *is* the conformance suite: `SqliteFixture` plus one macro
invocation emits 89 `#[tokio::test]`s that were previously unrunnable against
this crate. So the Red/Green evidence is not a locally written assertion — it is
the suite's own verdict, and the interesting part is what it said.

**The first run: 89 passed.** Which, per CLAUDE.md, is a claim to check rather
than a result to accept. Three checks followed.

**Check 1 — is every rule there?** The reference harness
`crates/happenstance-testkit/tests/memory_conformance.rs` emits **91** tests from
the same `for_each_event_store_rule!`, of which two are that file's own
`fixture_isolation` module. 91 − 2 = **89**. No rule is absent from the binary.

**Check 2 — can the suite fail against this fixture?** Changing
`MAX_EVENT_DATA_LEN` by one byte:

```text
test dcb_conformance::append_reports_exceeded_store_limits ... FAILED
thread panicked at crates\happenstance-testkit\src\suite.rs:4347:9
```

The suite is live. The constant was restored immediately.

**Check 3 — is it green *repeatably*?** It was not.

```text
test result: ok.     89 passed; 0 failed
test result: FAILED. 87 passed; 2 failed
test result: FAILED. 87 passed; 2 failed
test result: FAILED. 88 passed; 1 failed
test result: ok.     89 passed; 0 failed
test result: ok.     89 passed; 0 failed
```

Roughly one run in two, on `read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot`:

```text
assertion `left == right` failed: one `read` is evaluated against ONE state of
the store, fixed no later than the first poll of its stream.
  left: [(1, Seeded/b1), (2, Seeded/b2), (3, Seeded/b3), (4, Later/after)]
```

**The RED, and it is a real defect.** ES-11 requires the position ceiling to be
sampled **no later than the first poll**, and the adapter sampled it inside the
`spawn_blocking` hop — which runs *after* the first poll returns. The rules poll
once (legally receiving `Pending`), then append, then drain; the append could
land before the deferred sample, so the late event was **below** the ceiling and
the read observed it. Neither `tests/read.rs` nor `tests/wide_query.rs` could see
this: both drain hundreds of events before appending, by which time the sample is
long taken.

**The GREEN.** `ReadCursor::sample_ceiling`
(`crates/happenstance-sqlite/src/event_store.rs:1189-1204`), called from
`poll_next` **before the spawn** (`:1399-1408`). One `SELECT max(position)` on the
polling thread — an O(1) seek to the end of an integer primary key, on the same
lock `append` and `head` already take synchronously. Eight consecutive
conformance runs green afterwards, and a regression test that polls **exactly
once** and then appends is now at
`crates/happenstance-sqlite/tests/read.rs::an_append_after_a_single_poll_is_not_observed`.

| AC | Evidence | Verdict |
| --- | --- | --- |
| AC-001 | 89 passed / 0 failed / 0 ignored; rule count matched against the reference harness; two `Skipped`, both `MID_BATCH_FAULT` | green |
| AC-002 | `two_fixture_instances_observe_none_of_each_others_appends` + one fresh temp path per instance | green |
| AC-003 | `two_handles_observe_each_others_appends`, `head_advances_across_two_handles` + `connect` = `SqliteEventStore::open` | green |
| AC-004 | `acknowledged_writes_survive_a_reopen`, `recorded_time_survives_a_reopen`, `reopened_store_does_not_reissue_an_event_id` + a `reopen` that closes handles and checkpoints the WAL | green |
| AC-005 | `append_reports_exceeded_store_limits` observed **`Ran`**, plus the three minimum-acceptance rules | green |
| AC-006 | both `MID_BATCH_FAULT` rules present and printing this fixture's own sentence | green |
| AC-007 | no `#[ignore]`, no timeout, no sleep; the testkit untouched; `shapes.rs` unchanged | green |
| AC-008 | the three numbers and their reasoning recorded; `.kb/` untouched; `redkiln validate --kb` passed | green |

## Commits

`feat(sqlite-durable-store): SqliteFixture and the whole conformance suite` — see
the `Story: sqlite-durable-store/sqlite-fixture-and-whole-suite` trailer.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/tests/conformance.rs` | **New target.** `SqliteFixture` — one fresh temp file per instance, `connect` opening a real second connection, `reopen` closing handles and truncating the WAL, `SECOND_HANDLE` and `REOPEN` supported, `MID_BATCH_FAULT` declined in this adapter's own words, the three ceilings mirrored from the adapter's constants — plus `event_store_conformance!(SqliteFixture::new())` |
| `crates/happenstance-sqlite/src/event_store.rs` | `ReadCursor::sample_ceiling`, and `poll_next` taking the sample before the spawn — the fix for the defect above |
| `crates/happenstance-sqlite/tests/read.rs` | `::an_append_after_a_single_poll_is_not_observed`, the regression test for it |

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test conformance` | **89 passed**, 0 failed, 0 ignored — and **thirteen** runs green in total after the fix |
| `cargo test -p happenstance-sqlite` | **155 passed** across `conformance` (89), `append` (17), `wide_query` (16), `read` (14), `shapes` (10), `migration` (9) |
| `cargo xtask affected --base main` | **PASSED** |
| `redkiln validate --kb` | `validate passed` |
| `cargo fmt --all` | clean; run last |

## Notes

**AC-007 says "no SQL body was edited to make a rule pass", and one was edited —
in the opposite direction.** The clause exists to stop an implementer weakening
the store until a rule stops asking. What happened here is the reverse: the rule
asked a question the adapter got wrong, and the adapter was **strengthened** so
that its ceiling is sampled earlier than it was. The rule is unchanged; the
testkit is untouched; the guarantee is stricter than before. Recording it plainly
is the point — this is the mount doing exactly the job it was written for.

**Why the two `MID_BATCH_FAULT` rules are declined — corrected.** The first
version of this paragraph, and of the constant it described, claimed the adapter
had *no supported way* to make SQLite fail between two rows, "a trigger or a
`CHECK` armed for one write would be schema the store does not have". **That was
false, and the same commit disproved it**:
`crates/happenstance-sqlite/tests/append.rs::a_failure_mid_batch_leaves_nothing`
installs exactly such a trigger through a second connection, and asserts the
append fails **unabsorbed** as `AppendError::Store` with (0, 0, 0) rows left
behind. The trigger route is the mechanism
`crates/happenstance-testkit/src/contract.rs:207-211` names as the canonical
adapter injection, and `crates/happenstance-testkit/src/suite.rs:2782-2788`
restricts the *absorbing* store's declension to a store that can swallow every
fault its fixture can arm — the opposite of this one. The paragraph also offered
a false dichotomy, "declare the capability and arm nothing" against "skip", and
omitted the third option the diff already demonstrated.

What the decline actually records is the trade `contract.rs` itself describes: a
declined `Capability` is a choice — *the fixture could have co-operated and chose
not to* — and the reason string is the record of that choice. Three things make
it this story's choice rather than this story's to reverse:

- `../_decomposition.md`, architecture brief §9, assigns the fault far end out of
  this project's scope and says the fixture "should decline it **explicitly, with
  the real reason**".
- This story's **AC-006** states the reason the constant must carry — *supplies
  the reopen far end and not the fault far end, which is ES-35's live residual
  falsifier* — and asserts both gated rules print it.
- `reopen-negative-control-and-durability-verdicts` owns ES-35's maturity marker
  and bars *"moving any maturity marker's level"* from any other story. Arming
  the fault here retires that clause's residual falsifier as a side effect of a
  story that has no say in it.

So the constant now declines **by scope, not by incapacity**, names the mechanism
rather than denying it, cites the test that proves it works, and names the story
that must decide whether to wire it. Nobody reading this crate's CI log can now
conclude the adapter is incapable of a mid-batch fault, which is the only thing
the old sentence actually achieved.

**Two conformance rules remain dark, and that is a stated cost, not a hidden
one.** `append_is_atomic_under_a_mid_batch_fault` and
`arming_a_mid_batch_fault_makes_the_append_fail` would both pass under an armed
trigger — the first reads only after the faulted append, and the second wants
exactly the `Err` this mechanism produces — so wiring it is roughly twenty lines
whenever the owning story decides to spend them.

**Two axes the runbook records as empty are now filled.** *Handle multiplicity*:
`connect` opens a second real `rusqlite::Connection`, not a refcount clone of one
in-process object. *Durability*: an acknowledged write survives a reopen that
closes every connection and folds the write-ahead log into the main database
file. Neither claim is one a rule could have checked on its own — a `Clone`-shaped
fixture passes `two_handles_observe_each_others_appends` perfectly — which is why
the fixture's body is the evidence and the green run is the precondition.
