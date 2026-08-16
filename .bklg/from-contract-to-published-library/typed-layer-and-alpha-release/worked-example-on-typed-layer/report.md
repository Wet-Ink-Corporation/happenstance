---
item: "HS-S0029"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The worked example, rewritten on the typed layer and actually executed

## Findings Ledger

**Outcome: eight of eight ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `examples/course-subscriptions/src/main.rs` — the running application,
composition root item 2 — with two secondary mounts that are what make the story
falsifiable rather than merely written: `examples/course-subscriptions/Cargo.toml`, where
the dependency moves onto the crate a user installs, and
`xtask/src/proof.rs::ARTEFACTS:251-256`, the gate row that makes deleting *or emptying* the
execution test a loud failure.

**The headline.** The transcript is byte-identical to the one the example printed before
this PR, and everything beneath it is new. That is not a coincidence to be admired — it is
AC-002, asserted literally, and it is what makes the rewrite invisible to the reader the
example exists for.

| AC | result | proved by | notes |
| --- | --- | --- | --- |
| AC-001 | satisfied | `runs::the_binary_completes_the_dcb_cycle` | The gate now **executes** the binary. Before this PR the only red available was `error: no test target named runs`, because `cargo test --workspace --all-features` compiled the example and never called `main`. Exit 0, empty stderr, seven markers in order, three refusals. |
| AC-002 | satisfied | `runs::the_transcript_is_the_designed_composition`, `runs::the_transcript_holds_its_budget` | Composition and budget read off the same captured stdout. 25 lines against the 45 ceiling; widest line 58 columns against 80; widest marker 48 against 60; no ANSI escape, no carriage return. The unstyled render this rejects — a program that prints only the final log — passes every AC-001 assertion and fails these. |
| AC-003 | satisfied | `runs::the_example_depends_only_on_the_typed_layer` | Manifest names `happenstance` with `std`/`memory`/`json`; no `happenstance-core` entry; zero `happenstance_core` paths in the source. The build is the other half: `MemoryEventStore`, `Tags`, `Tag` and `EventType` all resolve through the facade's glob. **One assertion was refined** — see *Deviations*. |
| AC-004 | satisfied | `runs::the_deleted_constructs_are_gone`, `runs::capacity_refusal_carries_the_decoded_capacity` | All five spellings absent — deleted, not wrapped. The capacity is read off the decoded `CourseDefined` variant, and the `(2/2)` assertion is the discriminator: an `unwrap_or(0)`-shaped decode refuses the *first* subscribe with `(0/0)`. |
| AC-005 | satisfied | `runs::the_event_set_is_named_once`, `runs::the_fold_lives_in_the_users_file` | No query literal survives; a brace-matched scan of all three `fn apply` bodies finds no wildcard arm; the enum, its `DomainEvent` impl and all three folds are ordinary code in `src/main.rs` with no `macro_rules!`. This is the precondition the slice-mate consumes for its diagnostic span, and it holds. |
| AC-006 | satisfied | `runs::subscribe_composes_two_models` | `src/main.rs:457` hands `commit` a tuple of two distinct decision models. The test extracts `subscribe`'s body and then the paren-matched boundary group inside it, so it is structural rather than formatting-sensitive. Both refusals only the pair can produce fire in the same run. |
| AC-007 | satisfied | `runs::the_loop_absorbs_every_handler` | Zero `AppendCondition`, `read_decision_model`, `AppendError`; exactly three commit sites; a `Retry::` at each. The private `commit` is gone *into* `happenstance::commit`, so the after-anchor comes from the read and `conflicting_position` is never branched on. |
| AC-008 | satisfied | `cargo xtask proof-artefact` + a **rehearsed** negative control | Green with the row: `course-subscriptions/runs: 2 named tests present`. Emptied: **exit 1**, naming both missing tests. A bare `cargo test` over the same emptied file exits **0** with `running 0 tests` — which is the failure the row closes. Restored from a byte copy, re-run green. |

**Gate evidence.** `cargo xtask affected --base main` → `affected gate passed` (fmt, clippy
`-D warnings`, the affected packages' tests, five file-reading lints, `spec-trace`).
`cargo xtask proof-artefact` green separately, because the story-grain gate does not run it
(`xtask/src/affected.rs:119-125`). `cargo run -q -p course-subscriptions` read by a human.

**Deviations, all three recorded in `implementation-report.md` with their reasoning.**

1. **Two validated identity newtypes** (`CourseId`, `StudentId`) that no planning artefact
   named. They exist because `DomainEvent::tags` is infallible and `Tags` has no infallible
   constructor: the alternatives were an `unwrap` (forbidden by NF-001) or absorbing an
   unreachable `Err` as the empty tag set — which is the *same silent-fallback shape* this
   story exists to delete. They cost 70 of the file's 532 lines, and that cost is itself
   evidence for AC-013.
2. **AC-003's manifest assertion drops comment lines** before forbidding the string
   `happenstance-core`. The first spelling forbade the manifest's own *explanation* of the
   move. A real dependency entry still fails it.
3. **`Retry::attempts(3)` rather than `Retry::once()`** at the three call sites. Both are
   visible bounds; `once()` reads as "does not retry", which is a claim about the loop
   rather than about this example.

**One defect logged, not fixed — D-2, for AC-012.** *`DomainEvent::tags` is infallible
while `Tags` has only a fallible constructor, so an implementor whose tag values are
runtime strings has no total path unless they invent a validated newtype.* Same family as
the design's own **D-1**. No edit to `crates/happenstance-core/**` or
`crates/happenstance/**` was made (NF-008, AC-A02); it is routed to
`defect-log-and-macros-verdict`.

**One number handed forward, unjudged.** AC-013's ratio is measured off *this file* by M7.
The mapping ceremony is the larger half by a clear margin, and it worsened relative to the
design's 2.4:1 prediction exactly as that prediction said it would with a third variant.
The verdict is M7's; this report records the measurement's subject, not its conclusion.

**Nothing deferred.** `unsubscribe`'s *not subscribed* refusal remains unexercised by
`main`, which is EC-007 and is deliberate: adding an eighth section would improve coverage
and violate the signed-off composition. It is a design change if it is ever worth closing,
not an implementer's call.
