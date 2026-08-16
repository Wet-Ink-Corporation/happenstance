---
item: "HS-S0029"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The worked example, rewritten on the typed layer and actually executed

**All eight ACs are satisfied. Nothing is blocked and nothing is deferred.**

The transcript a reader sees is **byte-identical** to the one the example printed before
this PR — same seven sections, same wording, same aligned log — and everything beneath it
is different. `parse_capacity`, the `format!` payload, the empty byte payloads and the
private `commit` are gone; the event set is declared once, as `Enrolment`'s `EVENT_TYPES`,
and every query is derived from it; `subscribe` composes two decision models as a tuple;
all three handlers go through `happenstance::commit` with the retry bound spelled at the
call site.

And for the first time the repository can *observe* that. `examples/course-subscriptions/tests/runs.rs`
spawns the compiled binary and reads what it printed, and `xtask/src/proof.rs::ARTEFACTS`
holds two of its tests by name — so deleting the instrument fails the gate and emptying it
fails the gate, which was rehearsed rather than assumed.

## TDD Evidence

The order was the one the spec asked for: **write the execution test first, against the
example as it stands today.** That split the red cleanly in two, and the split is worth
reading, because the two halves are red for different reasons.

**Stage one — the instrument did not exist.** Before `tests/runs.rs` was written:

```console
$ cargo test -p course-subscriptions --test runs
error: no test target named `runs`
```

That is AC-001's and AC-002's real red. The *behaviour* they assert already worked; what
did not exist was anything in the gate that could tell. `cargo test --locked --workspace
--all-features` (`xtask/src/main.rs:143-154`) compiled the example and never called
`main`, which is exactly the finding the story was written against.

**Stage two — ten tests over today's binary.** With `runs.rs` written and nothing else
changed:

```console
$ cargo test -p course-subscriptions --test runs
test result: FAILED. 4 passed; 6 failed
```

The four that passed are the four that read the *output*:
`the_binary_completes_the_dcb_cycle`, `the_transcript_is_the_designed_composition`,
`the_transcript_holds_its_budget`, `capacity_refusal_carries_the_decoded_capacity`. That
is the point of writing them first — a green instrument over the *old* binary proves the
instrument works before the rewrite starts moving the thing it observes, and from that
moment the transcript assertions were a regression harness rather than an after-the-fact
claim.

The six that failed are the six that read the *source*, and each failed on the assertion
that names the missing behaviour:

| AC | test | red message, before the rewrite | green after |
| --- | --- | --- | --- |
| AC-001 | `runs::the_binary_completes_the_dcb_cycle` | `error: no test target named runs` (stage one) | ✅ exit 0, seven markers in order, three refusals |
| AC-002 | `runs::the_transcript_is_the_designed_composition` | same | ✅ composition asserted literally |
| AC-002 | `runs::the_transcript_holds_its_budget` | same | ✅ 25 lines / 45, ≤ 80 columns, no escape, no `\r` |
| AC-003 | `runs::the_example_depends_only_on_the_typed_layer` | *the example still depends on the crate adapter authors pin* | ✅ manifest names `happenstance`, source names no `happenstance_core` |
| AC-004 | `runs::the_deleted_constructs_are_gone` | ``parse_capacity` survives in the example — it was renamed or wrapped, not deleted` | ✅ all five spellings absent |
| AC-004 | `runs::capacity_refusal_carries_the_decoded_capacity` | green before and after — see the note below | ✅ `rejected: course c1 is full (2/2)` |
| AC-005 | `runs::the_event_set_is_named_once` | ``QueryItem::new` survives: the event set is still named a second time by hand` | ✅ no query literal, no wildcard arm in any fold |
| AC-005 | `runs::the_fold_lives_in_the_users_file` | *the domain enum is not declared here* | ✅ enum, `DomainEvent` impl and three folds in `src/main.rs` |
| AC-006 | `runs::subscribe_composes_two_models` | ``impl DecisionModel for Seats` is missing` | ✅ two models in one paren-matched tuple |
| AC-007 | `runs::the_loop_absorbs_every_handler` | ``AppendCondition` survives: a handler still spells the cycle out by hand` | ✅ three commit sites, three `Retry::`, no cycle by hand |
| AC-008 | `cargo xtask proof-artefact` | see the rehearsal below | ✅ `course-subscriptions/runs: 2 named tests present` |

**The one test that was green throughout, said out loud.**
`capacity_refusal_carries_the_decoded_capacity` passed against the old binary too, because
the old `parse_capacity` happened to return the right number for this input. It is a
*discriminator*, not a red-then-green row: its job is to reject the wrong replacement, and
the wrong replacement is loud — an `unwrap_or(0)`-shaped decode makes the **first**
subscribe refuse `(0/0)` and can never reach the right transcript. AC-004's red half is
`the_deleted_constructs_are_gone`, which was genuinely red.

**AC-008's negative control was rehearsed, not assumed.**

```console
$ printf '' > examples/course-subscriptions/tests/runs.rs
$ cargo xtask proof-artefact
xtask failed: `course-subscriptions`'s `runs` is missing 2 of the tests the gate names:
["runs::the_binary_completes_the_dcb_cycle", "runs::the_transcript_is_the_designed_composition"]

The target exists and builds, so `cargo test` would have exited 0 with nothing to say.
These are the clauses' own names — if one was renamed deliberately, update
`xtask/src/proof.rs` and the clause in `SPECIFICATION.md` that cites it, in the same
change. Listed: []
$ echo $?
1
```

The file was then restored from a byte-for-byte copy (**not** `git restore` — on Windows
that re-checks-out CRLF and the formatter rejects the whole file) and the step re-run
green. A bare `cargo test -p course-subscriptions --test runs` over the same emptied file
reports `running 0 tests` and exits **0**, which is the whole argument for the row.

## Commits

One checkpoint commit, carrying the whole story — the rewrite, the new test target, the
gate row, this report and the ledger:

```
feat(typed-layer-and-alpha-release): Worked example on typed layer
Story: typed-layer-and-alpha-release/worked-example-on-typed-layer
```

Its short SHA is reported in this run's slice digest and is recoverable here with
`git log --grep "Story: typed-layer-and-alpha-release/worked-example-on-typed-layer" --oneline`.

## Changes

| File | Shape of the change |
| --- | --- |
| `examples/course-subscriptions/src/main.rs` | Rewritten **beneath** `main`. `main` (`:50-96`) is unchanged in every observable respect. New: two validated identity newtypes (`:112`, `:149`), the `Enrolment` domain enum and its `DomainEvent` impl (`:194`, `:209`), a `Refusal` error enum (`:255`), three decision models (`:305`, `:341`, `:381`), three handlers rewritten onto `happenstance::commit` (`:421`, `:454`, `:494`), and a `rejected` renderer (`:527`). Deleted: `parse_capacity`, the `format!` payload, the two empty byte payloads, the private `commit`, the three bare `const &str` event-type names, and every `Query`/`QueryItem`/`AppendCondition`/`AppendError` mention. |
| `examples/course-subscriptions/Cargo.toml` | `happenstance-core` → `happenstance` with `std`, `memory`, `json`; `serde` and `thiserror` added; `anyhow` and `tokio` retained. Five entries, which is NF-004's four plus its named admissible fifth. |
| `examples/course-subscriptions/tests/runs.rs` | **New.** One spawn of `CARGO_BIN_EXE_course-subscriptions`, ten tests inside `mod runs`, two source scanners (brace- and paren-matched) so the source assertions are structural rather than formatting-sensitive. No new dependency: `std::process::Command` and a Cargo-provided env var. |
| `xtask/src/proof.rs` | One `const WORKED_EXAMPLE_TESTS` (`:214`) and one `Artefact` row (`:251-256`). The module doc's target count moved from five to six. No new gate step — `xtask/src/main.rs:178-190` already runs this. |
| `Cargo.lock` | Edges only, as predicted: `course-subscriptions` gains `happenstance`, `serde` and `thiserror` and loses its direct `happenstance-core` edge. **No new node** — every crate was already in the graph. |

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p course-subscriptions --test runs` | 10 passed, 0 failed |
| `cargo run -q -p course-subscriptions` | Seven-section transcript, exit 0, read by a human and diffed against the pre-rewrite capture: identical |
| `cargo xtask proof-artefact` | green; `course-subscriptions/runs: 2 named tests present` |
| `cargo xtask proof-artefact` with `runs.rs` emptied | **exit 1**, naming both missing tests (the rehearsal above) |
| `cargo fmt --all` then `cargo fmt --check` (inside the gate) | clean; the formatter ran last, after the clippy fix |
| `cargo xtask affected --base main` | **`affected gate passed`** — fmt, `clippy -D warnings` over `course-subscriptions`, `happenstance`, `xtask` and the rest, the affected packages' tests, the five file-reading lints and `spec-trace` |

One clippy finding was raised and fixed inside the story rather than allowed:
`too_many_lines` (101/100) on `the_transcript_is_the_designed_composition`. The fix was to
lift the log-region column assertions into `assert_log_region` (`tests/runs.rs:196`), which
also gave that region its own doc comment. No assertion was weakened or removed.

## Notes

**Deviation 1 — two validated identity newtypes, which the spec did not name.**
`DomainEvent::tags` is infallible; `Tags` has no infallible constructor, because
`Tag::key_value` is the only way in and it can refuse. A domain type whose tags come from
runtime values therefore has three options: hold the validated form, `unwrap`, or absorb an
unreachable `Err` as the empty tag set. NF-001 forbids the second, and the third is the
*same shape* as the `unwrap_or(0)` this story exists to delete — replacing a silent zero
with a silent empty tag set would have undercut AC-004's headline claim in the same file.
So `CourseId` and `StudentId` (`src/main.rs:112`, `:149`) validate once at the handler's
edge and carry the built `Tag` from then on, which makes `tags()` (`:222`) total: no
`unwrap`, no fallback, no unreachable arm. It is the same resolution the design already
made for `DecisionModel::scope`, applied to the event instead of the model.

**Defect candidate D-2, for AC-012's log.** *`DomainEvent::tags` is infallible while
`Tags` has only a fallible constructor, so an implementor whose tag values are runtime
strings has no total path unless they invent a validated newtype. The typed layer offers no
`Tags` constructor for pre-validated inputs and no fallible `tags` variant.* Clause: none
directly; it is the same family as **D-1** (`_design.md`, *Shape decision*, the residual) —
`happenstance-core` has no infallible constructor for values a caller has already
validated. **Not fixed here**: no edit to `crates/happenstance-core/**` or
`crates/happenstance/**` (NF-008, AC-A02). Routed to AC-012's defect log for
`defect-log-and-macros-verdict`. The cost is measurable and is in this file: 70 of the
example's 532 lines are the two newtypes and their serde bridges.

**Deviation 2 — `AC-003`'s manifest assertion reads the manifest with comments stripped.**
The first spelling was `!MANIFEST.contains("happenstance-core")`, which failed green code:
the manifest's own comment *explains* the move off `happenstance-core`, and a check that
forbids the explanation forbids the documentation. The assertion now drops `#` lines first
and then forbids the string. A real `happenstance-core = …` entry still fails it, so no
discriminating power was traded away — only a false positive on prose.

**Deviation 3 — the retry bound is `Retry::attempts(ATTEMPTS.try_into()?)`, not
`Retry::once()`.** Both satisfy AC-007's *visible at the call site*. `attempts(3)` was
chosen because `once()` reads as "this command does not retry", which is a claim about the
loop rather than about this example: the example is uncontended, so the bound is never
spent either way, and a bound of three says *bounded* where a bound of one says *absent*.

**AC-013's ratio, recorded here because M7 reads it off this file rather than off the
doctest.** In `src/main.rs`, counting non-blank, non-comment lines: the mapping ceremony —
`EVENT_TYPES`, `event_type`, `tags`, `encode`, `decode`, plus the two identity newtypes
that exist only so `tags()` can be total — is the larger half by a clear margin over the
domain (the enum's three variants, three folds and three decisions). The design predicted
2.4:1 for a two-variant enum and predicted it would worsen with a third; a third variant
plus real tags is what this file has, and it did worsen. The **verdict** is
`defect-log-and-macros-verdict`'s, not this PR's (NF-002, *Clarifications* item 4).

**Not touched, deliberately.** No `crates/happenstance*/src/**` edit; no `[FROZEN]` clause;
no `spec/SPECIFICATION.md`; no new public item; no conformance rule. `unsubscribe`'s *not
subscribed* refusal stays unexercised (EC-007) because `main`'s observable steps outrank
coverage here.
