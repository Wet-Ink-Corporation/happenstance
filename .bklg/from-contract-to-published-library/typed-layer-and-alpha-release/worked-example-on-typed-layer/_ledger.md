---
item: HS-S0029
stage: implement
created: 2026-08-12T13:46:26.084Z
updated: 2026-08-12T13:46:26.084Z
---

# Acceptance ledger — The worked example, rewritten on the typed layer and actually executed

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "**P1 gets a demonstration, not a claim.** GIVEN P1 has been told a consistency boundary can be modelled before a database is chosen, WHEN the **gate** — not a human at a keyboard — executes the compiled `course-subscriptions` binary, THEN the process exits 0 having performed the whole canonical cycle in order: define c1 → refuse the duplicate definition → subscribe s1 and s2 → refuse s1's duplicate → refuse s3 for capacity → unsubscribe s1 and subscribe s3 into the freed seat → print the final log; with **no `todo!()` reached**, no panic, and no `unwrap` on a fallible path. A step that *compiles* the example does not satisfy this"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — the running application; main's observable steps (:37-79) survive verbatim and everything beneath them is rewritten"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::the_binary_completes_the_dcb_cycle"

- id: AC-002
  criterion: "**P1 and P4 read the transcript they were promised, and the rewrite is invisible in it.** GIVEN the signed-off surface `worked-example-transcript` (`_design.md`, *Surfaces*; approved 2026-08-12), WHEN the binary's stdout is read at 80 columns with stdout piped and no TTY, THEN the **composition** is the one at `examples/course-subscriptions/src/main.rs:37-79` unchanged — seven `== … ==` markers in that order and wording, a blank line before each but the first, result lines indented 3, every refusal carrying the literal `rejected: ` prefix **and its value** (`course c1 is full (2/2)`, never `capacity exceeded`), then `== final log ==` and one `{:>3}  {:<22} {:?}` row per event — AND the **budget** holds: no line past **80 columns**, no marker past **60**, at most **45 lines** total, a tag list wrapping to a continuation line indented **8** while position and event type never truncate, no event-type name grown past the **22**-column field, and **zero** colour and **zero** motion"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — the rendered surface worked-example-transcript, emitted by main (:37-79)"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::the_transcript_is_the_designed_composition and examples/course-subscriptions/tests/runs.rs::the_transcript_holds_its_budget"

- id: AC-003
  criterion: "**P1 is shown the crate they actually installed.** GIVEN P1 ran `cargo add happenstance` because that is the crate the README points at (ADR-0006), WHEN they open the example that is supposed to show them how to use it, THEN its manifest names **`happenstance`** with `std`, `memory` and `json` and `serde` for the domain enum, carries **no `happenstance-core` entry**, and **no `happenstance_core::` path appears anywhere in its source** — every contract name it still uses (`MemoryEventStore`, `Tags`, `EventType`, `Query`) resolving through `pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs:75`)"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/Cargo.toml — the dependency moves off happenstance-core onto happenstance (std, memory, json), plus serde; composition root item 3"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::the_example_depends_only_on_the_typed_layer"

- id: AC-004
  criterion: "**P1's capacity rule is enforced by a decode, not by a byte scraper that fails silently.** GIVEN the old example answered *\"what capacity?\"* with `parse_capacity`'s `unwrap_or(0)` (`:231-237`), WHEN the rewritten example runs, THEN `parse_capacity`, the `format!(\"{{\\\"capacity\\\":{capacity}}}\").into_bytes()` payload (`:97-100`), the `&b\"{}\"[..]` empty payloads (`:168`, `:192`) and the private `commit` (`:207-224`) are **absent from the file** — deleted, not renamed and not wrapped behind a nicer name — and the capacity the refusal prints arrives from `DomainEvent::decode` through the `json` `Codec`"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — the three deletions land beneath main; composition root item 2"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::the_deleted_constructs_are_gone and examples/course-subscriptions/tests/runs.rs::capacity_refusal_carries_the_decoded_capacity"

- id: AC-005
  criterion: "**P1 cannot leave a new event type half-handled, and the compiler tells them where.** GIVEN the hazard this phase exists for — a DCB handler naming its event set twice, once in a query and once in a fold (`:114-125` and `:140-155` today) — WHEN a variant is added to the example's domain enum, THEN every decision model's `apply` fails to compile until its arm is written, because each `apply` matches the **enum** with no `_ => {}` arm and the query is **derived** by `Boundary::query` from `EVENT_TYPES` and the models' validated `scope` — no `QueryItem::new` or `Query::from_items` literal survives; AND the enum and its folds are ordinary, visible code in `examples/course-subscriptions/src/main.rs`, so the slice-mate's diagnostic `-->` span lands on **P1's own match arm** and not in a macro body or under `crates/`"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — the domain enum, its DomainEvent impl and each DecisionModel::apply, declared in the user's own file"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::the_event_set_is_named_once and examples/course-subscriptions/tests/runs.rs::the_fold_lives_in_the_users_file"

- id: AC-006
  criterion: "**P1 sees the mechanism that makes a dynamic boundary dynamic, not a hand-collapsed shortcut.** GIVEN `subscribe` is the case that motivates DCB — the course's capacity and everyone holding a seat is one concern, this student's own history is another — WHEN it builds its boundary, THEN it passes **two distinct `DecisionModel`s as a tuple** `(B1, B2)` whose OR'd query is the union of their fragments, each absorbing only what it nominated, with no macro invocation in the caller's face and no single hand-written model collapsing the two concerns; and **both** refusals that only the pair can produce fire in the same run — `student s1 is already subscribed to c1` and `course c1 is full (2/2)`"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — subscribe (:113-174), the only composed handler"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::subscribe_composes_two_models"

- id: AC-007
  criterion: "**P1 never writes the read-decide-append-retry cycle, and therefore cannot write it wrong.** GIVEN the loop landed by `command-loop`, WHEN each of the three handlers appends, THEN it goes through `happenstance::commit` with an explicit `Retry` **visible at the call site** and a closure returning the events on acceptance or the handler's own error on refusal; and **no `AppendCondition`, no `read_decision_model` call and no `AppendError` match survives anywhere in the example** — so the `after` anchor comes from the read and never from `append`'s return (`crates/happenstance-core/src/store.rs:131-145`), `conflicting_position` is never branched on (`crates/happenstance-core/src/error.rs:139-147`), and the retry bound is a required argument rather than a hidden default. The example is uncontended, so `Committed.attempts` is 1 on every commit: a visible bound on an unexercised path"
  satisfied: false
  evidence: ""
  mount_point: "examples/course-subscriptions/src/main.rs — the three handlers define_course (:86), subscribe (:113) and unsubscribe (:177), each calling happenstance::commit"
  verifying_test: "examples/course-subscriptions/tests/runs.rs::the_loop_absorbs_every_handler"

- id: AC-008
  criterion: "**The proof P1's successor inherits cannot be emptied in silence.** GIVEN this repository has already paid once for a proof artefact that a deletion failed and an emptying passed (`xtask/src/proof.rs:9-23`), WHEN the execution test is deleted **or** truncated to zero tests, THEN `cargo xtask proof-artefact` fails by **name** rather than reporting `running 0 tests` — because `xtask/src/proof.rs::ARTEFACTS` carries a row naming package `course-subscriptions`, target `runs`, and the test names AC-001 and AC-002 rest on"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs::ARTEFACTS (:132-149) — one Artefact row naming package course-subscriptions, target runs; the gate step at xtask/src/main.rs:178-190 already runs it"
  verifying_test: "cargo xtask proof-artefact (green with the row present), plus the rehearsed negative control: empty examples/course-subscriptions/tests/runs.rs, confirm the step fails naming the missing test, restore"
```
