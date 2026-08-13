---
item: HS-S0023
stage: implement
created: 2026-08-12T13:46:19.582Z
updated: 2026-08-12T13:46:19.582Z
---

# Acceptance ledger — The command loop — read, decide, append, retry on ConditionViolated

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
  criterion: "**P1 is protected from the lost update `append`'s return value invites.** GIVEN P1 has written one `commit` call against a boundary another writer is also appending to, WHEN the loop builds the `AppendCondition` for any attempt — the first or the fourth — THEN its `after` is the last position that attempt's own `read_decision_model` actually observed (`crates/happenstance-core/src/store.rs:315-330`), never the position the previous attempt's `append` returned (`:131-145`), so no event sitting below that return is excluded from the condition"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — commit/commit_with pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/tests/command_loop.rs::after_anchor_comes_from_the_read (with its named wrong implementation threads_append_return_is_rejected)"

- id: AC-002
  criterion: "**P1 can build, inspect and discard a decision without consequence.** GIVEN a domain rule P1 encoded as an `Err` return from their `decide` closure, WHEN the loop reaches that refusal, THEN it returns `CommandError::Refused(d)` carrying P1's own error type, and the store is byte-identical to before the call — no partial append, no probe write, and nothing before the append performed any write at all (AC-U12, `_decomposition.md:170-178`)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — commit/commit_with pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/tests/command_loop.rs::refusal_appends_nothing"

- id: AC-003
  criterion: "**P1 gets back a value that says what happened, not a tuple that freezes what can be said.** GIVEN an uncontended store, WHEN P1 awaits `commit`, THEN they receive `Committed { position, attempts }` where `position` is what `append` assigned and `attempts` is 1; the type is `#[non_exhaustive]` and `#[must_use]`, so a later observability field is not a breaking change and a dropped outcome does not compile quietly (AC-U06, AC-U09; `_design.md`, *Signatures*)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — Committed pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/src/command.rs::tests::first_try_reports_one_attempt, plus crates/happenstance/tests/docs_composition.rs::committed_is_non_exhaustive_and_must_use"

- id: AC-004
  criterion: "**P1's retry re-decides against the world as it now is, and their own inputs survive it.** GIVEN a competing writer lands an event between P1's read and P1's append, WHEN the loop retries, THEN it re-derives the query, re-reads, folds a **fresh clone of the pristine boundary** and calls `decide` again with that fresh state — it never reuses the previous fold, never re-submits the previous event batch, and never mutates the boundary value P1 passed in (AC-U13, `_decomposition.md:179-185`; `_design.md`, *Shape decision*, `DecisionModel` supertrait row)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — commit/commit_with pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/tests/command_loop.rs::retry_refolds_from_pristine_state and ::retry_does_not_resubmit_the_previous_batch"

- id: AC-005
  criterion: "**P1's command works against a remote store, not only an in-process one.** GIVEN a store that reports `ConditionViolated` with `conflicting_position: None` — which a one-shot-HTTP adapter legitimately does (`crates/happenstance-core/src/error.rs:139-147`) — WHEN the loop decides whether to retry, THEN it decides through `AppendError::is_condition_violated()` (`error.rs:253`) and nothing else, retries normally, and still surfaces the hint it declined to branch on by carrying the `ConditionViolated` value into whatever error it eventually returns rather than discarding it"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — commit/commit_with and CommandError pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/tests/command_loop.rs::retries_when_conflicting_position_is_none, ::branching_on_some_is_rejected, ::exhausted_carries_the_violation"

- id: AC-006
  criterion: "**P1 can see the bound before they hit it, and running out is a distinct answer.** GIVEN P1 wrote `Retry::attempts(2.try_into()?)` at the call site — a required argument with no `Default` and no hidden 3 — WHEN the store violates on every attempt, THEN exactly 2 appends are attempted and the call returns `CommandError::Exhausted { attempts: 2, source }`, distinguishable from a store failure and from a refusal; `Retry::once()` attempts exactly 1 (AC-U13; `_design.md`, *Shape decision*, Retry-bound row)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — Retry and CommandError pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/tests/command_loop.rs::exhaustion_is_bounded_and_named, plus crates/happenstance/src/command.rs::tests::retry_has_no_default"

- id: AC-007
  criterion: "**P1 matches on the vocabulary they already learned, and every error carries the value.** GIVEN any failure a caller can act on, WHEN P1 inspects it, THEN it is a `#[non_exhaustive]` `CommandError<E, D>` variant whose payload is a typed `#[source]` — the store's own `E`, P1's own `D`, a `CodecError`, an `InvalidQuery` — with no `String` payload and no `happenstance`-local re-spelling of `ConditionViolated`, and the chain is reachable through `core::error::Error::source` (AC-U05, AC-U08; `standards/rust/30-error-taxonomy.md` RS-30-2, RS-30-4)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — CommandError pub use'd at the crate root beside the surviving pub use happenstance_core::*; (:75)"
  verifying_test: "crates/happenstance/src/command.rs::tests::store_read_failure_chains_to_source, ::decode_failure_names_its_position, ::encode_failure_names_its_event_type, plus crates/happenstance/tests/docs_composition.rs::no_string_payloads_in_command_error"

- id: AC-008
  criterion: "**P3's edge build survives P1's command loop.** GIVEN a Workers or `wasm32` caller on the `!Send` flavour and a multi-threaded caller on the `Send` flavour, WHEN either compiles `commit`/`commit_with`, THEN both work: this crate's generic code binds `EventStore` (the weaker requirement that accepts both), imports exactly one flavour name per module, uses no `#[async_trait]`, and — because `S::Error` carries no `Send` bound (ADR-0009) — the loop collapses each `AppendError` to a retry decision **before** the next read's await, so the whole future stays `Send` inside a real `tokio::spawn` (`CLAUDE.md` constraints 1 and 4; `crates/happenstance-core/src/memory.rs:643-700`)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — the crate root whose items are generic over EventStore, the weaker flavour"
  verifying_test: "crates/happenstance/tests/flavours.rs::commit_spawns_from_generic and ::commit_binds_the_weak_flavour, plus cargo xtask affected --base main (clippy -D warnings)"

- id: AC-009
  criterion: "**P4 lands on the crate root and meets the loop as a working thing, not a plan.** GIVEN P4's one bounded, one-shot sitting (`_decomposition.md`, UX brief persona table), WHEN they open `docs.rs/happenstance`, THEN region 2's first program calls `commit` and compiles; region 4's `**The command loop**` bullet is an intra-doc link to that function **in place**, out from under the word *Planned* (`crates/happenstance/src/lib.rs:35,47-48`); `commit`, `commit_with`, `Retry`, `Committed` and `CommandError` are reachable from the crate root beside the surviving `pub use happenstance_core::*;` (`:75`) and none of them shadows a core name; and `commit`'s **own** item page carries the retry policy, the verbatim-resubmission distinction and the rejected alternative as persistent prose — a doc comment that says *see the specification* fails this (AC-U10, AC-U14, AC-U15; `_design.md`, *Composition* regions 2 and 4, *Transience policy* rows for the retry policy and for `Boundary`/`Committed`/`CommandError`; anti-patterns 1, 2, 14)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — the module doc's command-loop bullet (:47-48) rewritten in place as an intra-doc link, beside the glob re-export (:75)"
  verifying_test: "crates/happenstance/tests/docs_composition.rs::roadmap_bullet_became_a_link, ::no_planned_heading_survives, ::no_item_shadows_a_core_name, ::retry_policy_is_on_commit_itself, ::landing_page_names_only_the_persistent_four, plus cargo test -p happenstance --doc and cargo doc -p happenstance --no-deps with rustdoc::broken_intra_doc_links denied"

- id: AC-010
  criterion: "**P4 can read the page without scrolling sideways and can tell what a feature turns on.** GIVEN the same one sitting, at 1024x768, WHEN P4 reads the rendered page, THEN the composed presentation holds its budget: doc prose at most 80 columns, code inside a doc fence at most 72 columns, the first sentence of every item this story adds at most 80 characters and a complete claim, every added identifier at most 24 characters, and the crate-root module doc at most 130 lines (`_design.md`, *Density budget*); AND the feature story is legible without opening `Cargo.toml` — `commit` is the JSON convenience gated `#[cfg(feature = \"json\")]` and renders a `doc_cfg` gate badge on docs.rs, `commit_with` is ungated and reachable with the gate off, and no intra-doc link on the page resolves in only some feature configurations (AC-U14; RS-70-2, RS-70-4, RS-51-5; anti-patterns 3, 5, 6)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — the crate-root module doc and its item table, plus crates/happenstance/Cargo.toml's [package.metadata.docs.rs] block"
  verifying_test: "crates/happenstance/tests/docs_composition.rs::density_budget_holds and ::docsrs_metadata_is_present, plus cargo check -p happenstance --no-default-features (and --features std) and cargo doc -p happenstance --no-deps in both feature states"
```
