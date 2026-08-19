# Grounding — What a store may forget, and how a reader finds out (HS-P0018)

Companion to [`project.md`](project.md) and [`_intake-brief.md`](_intake-brief.md).
Everything below was checked against the worktree at
`D:\repos\happenstance\.claude\worktrees\from-contract-to-published-library`
on 2026-08-12. Paths are repo-relative; every one was opened and read, not
inferred from a citation elsewhere.

## 1. What already governs this project (Accepted decision atoms)

`.kb/decisions/` holds seventeen atoms today: `0001`–`0016` and `0029`
(`0017`–`0028` do not exist on disk yet — see §5, "what has not landed yet").
None targets retention or completeness directly; the ones that bound this
project's *shape* rather than its *answer* are:

- **ADR-0001** (`.kb/decisions/0001-async-port-flavours.md`) — the two-flavour
  port design (`EventStore` / `SendEventStore`, no `#[async_trait]`). Binds any
  new rule this project writes: it must compile against both flavours the same
  way every existing rule does, because `for_each_event_store_rule!` is one
  registry for both.
- **ADR-0013** (`.kb/decisions/0013-position-assignment-and-visibility.md`) —
  ES-38 is *settled by* this ADR (`spec/SPECIFICATION.md:4299` says so) even
  though its rule is unwritten. Its own "What this ADR leaves open" section is
  quoted verbatim in the open-question atom (§3) as the origin of ES-38's
  "Owner: phase 14" line. This project discharges an obligation ADR-0013
  already created; it does not renegotiate ES-38's text.
- **ADR-0029** (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`) — MSRV 1.97.1,
  let-chains available. `append.rs:245-248` shows the house style still prefers
  a `match` over a let-chain for a two-arm predicate; any new rule body should
  follow that precedent rather than reach for a let-chain by default.

**No Accepted atom answers ES-39, CF-27 or SY-32.** That is exactly what the
open-question atom in §3 and the `[DEFERRED]` markers in §2 say, and it is why
this project's deliverable is ADR-0028 itself, not a citation to one that
already exists.

## 2. Specification clauses (ground truth for the ACs)

All read directly from `spec/SPECIFICATION.md` (9070 lines total); line numbers
below are current, not carried over from `project.md`'s citations (which were
independently re-verified and match).

| Clause | Lines | Marker | What it says |
|---|---|---|---|
| §3.7 header | 4260–4272 | prose | "A holed log and a young log are the same value... The gap is that nothing can *ask*." |
| ES-37 | 4274–4297 | `[FROZEN]` | `EventStore` MUST NOT grow delete/truncate/redact/compact/tombstone at 0.1. No rule — "the absence of a method is not checkable." Names redaction (not delete) as the strongest candidate to revisit in 0.2, because shredding covers `data` but not `tags` (`event.rs:437-447`). |
| ES-38 | 4299–4323 | `[FROZEN]` | Names `positions_are_not_reused_after_removal` **(new)**. "The rule was examined at phase 4 and deliberately not written... Owner: phase 14." Rejects an adapter that renumbers on compaction. |
| ES-39 | 4325–4349 | `[DEFERRED]` | Whether a store can declare what it does not hold. Names three candidate primitives (floor / retained-ranges / third outcome) and argues against `earliest_position()` by name: a regulated purge is *scattered, not a prefix*. Names `a_store_reports_the_history_it_does_not_hold` **(new)**, "writable only once the primitive is chosen." No owning phase recorded in the runbook ledger (flagged in the clause text itself as a gap). |
| ES-40 | 4351–4379 | `[PROVISIONAL — axis: completeness, falsified or given its assertion by CF-27's instrument]` | Documentation-only clause: an `AppendCondition` is sound only over a complete store. Names `condition_over_removed_history_does_not_reject` **(new)**, asserting the **vacuous pass** as specified behaviour. Ties directly to `Guard::is_violated_by` (`append.rs:239-253`, verified below). |
| SY-32 | §5.10, ~6774–6810 | `[DEFERRED]` | A peer MUST report its retention floor; a runner MUST detect a peer offline longer than another's window. Names `retention_gap_is_reported_not_silent` (new, `happenstance-sync-testkit`). Rejects `earliest_position()` for the same scattered-purge reason as ES-39. |
| CF-25/CF-26 | 8003–8021 | `[FROZEN]` | The freeze-axis and fixture-vs-adapter rules this project's residual-exposure risk note (project.md) is written against. CF-26: a fixture satisfies falsifiability, not implementability — the reason AC-011/the "device adapter second" exclusion is legitimate rather than a shortcut. |
| CF-27 | 8034–8060 | `[DEFERRED — owned by "the pass that settles retention and deletion"]` | The completeness instrument itself: "a testkit-adjacent store that deliberately holds only a suffix of its own log, and reports that it does." Names `suffix_store_is_distinguishable_from_a_young_store` (new; "its assertion is fixed by the retention decision, which is ES-39's"). Explicitly: "This is the instrument, not the primitive." |
| Portfolio table | 8090–8098 | table | Completeness row: "**No, and nothing is planned.**" "Fixture first; a device adapter second." — the device adapter is explicitly the *next* project's problem, not this one's (matches project.md's Out-of-scope entry). |

`EventStore::read` (`store.rs:119-123`) and `::append` (`store.rs:213-217`) —
verified: two methods, neither deletes, exactly as CF-27's "Rejects" clause and
project.md's Code anchors both say.

`Guard::is_violated_by` at `append.rs:239-253` (not the `AppendCondition`-level
one at 225-234, which fans out to it) is the pure predicate ES-40 is written
against: `match self.after { Some(after) if position <= after => false, _ =>
self.query.matches(...) }`. There is no third arm — confirms "no third outcome"
verbatim and shows exactly where a `condition_over_removed_history_does_not_reject`
rule would assert against.

## 3. The open-question atom this project must resolve (not delete)

`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
(`id: kb-open-question-es-38-and-gap-read-unowned-001`, `status: accepted`).

It carries **two** sub-threads, and only one is this project's:

- **ES-38's rule** — blocked on CF-27's instrument, "nothing planned before
  phase 14." This project *is* phase 14.
- **`read_from_a_gap_position`** (ES-9's owed rule) — a *different*, unrelated
  gap: not blocked on an instrument, blocked on nobody having claimed it. It is
  already registered in `for_each_event_store_rule!`
  (`crates/happenstance-testkit/src/registry.rs`, `--- Read options ---` block),
  so it has a rule body today; the atom's concern is ownership, not existence.

**Sub-question 3, verbatim from the atom**: *"When CF-27 lands, does
`positions_are_not_reused_after_removal` get written against whatever
removal-capable fixture arrives with it, or does phase 14 need its own
instrument decision first?"* — this is exactly AC-015 / DR-10's target, and
project.md's Out-of-scope section is correct to fence off
`read_from_a_gap_position`'s ownership as a *separate*, not-adopted-by-default
decision so the two threads don't get resolved together in passing.

Resolving this atom means changing its `status` (currently `accepted`) to
reflect resolution and answering sub-question 3 in its body — through whatever
atom-authoring path is live for open-question atoms (the project's own AC-015
language: "its `status` reflecting the resolution"). No `_intake/` pattern for
open-question mutation was found in this pass; flag as a question for the
architecture brief rather than assume `kb-ingest` is the only path, since
`kb-ingest`'s documented job (per `CLAUDE.md`) is consuming `.kb/_intake/` into
new/merged atoms, and ADR-0028 is the artifact explicitly routed that way — the
open-question atom's own resolution mechanics should be confirmed against
whatever `redkiln:kb-ingest` actually does with an existing atom of
`kind: open_question`, not assumed.

## 4. Code patterns this project must follow

### 4.1 The `Fixture` contract and where instruments already live

- `crates/happenstance-testkit/src/contract.rs` — `Fixture` trait,
  `Capability` (trades, need a stated reason when declined — `Capability::declined`
  rejects an empty reason string) vs. three `Option<usize>` limits (facts, no
  reason owed). `SECOND_HANDLE` and `REOPEN` are the two existing associated
  `Capability` consts (`:161`, `:173`) — CF-27's instrument needs a *third* kind
  of declared fact (holds-a-suffix), and the risk note in project.md is right
  that any new fixture-side seam must take the **defaulted associated const**
  shape (a required method addition would be breaking on a publishable crate).
- `crates/happenstance-testkit/src/registry.rs:94` —
  `macro_rules! for_each_event_store_rule` is the actual definition (the doc
  prose calling it "the one place the rule set lives" is at
  `crates/happenstance-testkit/src/lib.rs:84-89`, matching project.md's
  citation). `positions_are_not_reused_after_removal` and
  `condition_over_removed_history_does_not_reject` are **absent** from the
  current registry — confirms both ES-38 and ES-40's rules are genuinely
  unwritten today, not merely unrun.
- `crates/happenstance-testkit/tests/fixture_instruments.rs` — `DurableFixture`
  is the closest existing precedent for "an axis-specific instrument fixture,
  testkit-adjacent, native-only." It wraps a live `MemoryEventStore` plus a
  side "disk" `Vec<SequencedEvent>`, using `MemoryEventStore::restore` (not
  `with_events`, which would re-stamp) to preserve store-assigned facts across
  reopen. CF-27's suffix store — "a decorator over any `EventStore`" per its
  spec text and project.md's DR-1 — is the same shape one axis over: it should
  wrap/decorate an inner store and change what a subset of operations report,
  not build a new backing medium.
- `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` and
  `crates/happenstance-testkit/tests/mutation_coverage.rs` (`REGISTRY` at
  `:324`, meta-tests `mutant_registry_is_exhaustive` at `:2754` and
  `mutants_fail_exactly_their_declared_rules` at `:2889`) — this is the
  concrete mechanism AC-004 and AC-005 mean by "a registered wrong
  implementation." A mutant is a `Defect` impl overriding exactly one step
  (documented rationale at the top of `mutants.rs`: one defect per store, so a
  red rule is red for the declared reason). Eight mutants are written out
  longhand rather than as `Defect` overrides, each for a stated shape reason
  (`CachedHeadFixture`, `LosingFixture`, etc. — see the module doc). ES-38's
  "renumbers on compaction" mutant and ES-40's "rejects/errors instead of
  vacuous pass" mutant are new entries in this same table, not a new
  mechanism — CLAUDE.md's corollary ("a rule that no adapter can fail is
  decorative... write that implementation into the testkit's own `tests/`")
  is discharged by adding a row to `REGISTRY`, matching what CLAUDE.md and the
  ACs both already assume.
- `crates/happenstance-testkit/src/fixtures/` — `MemoryFixture`, the reference
  `Fixture` impl to model any new fixture on.

### 4.2 Core types the redaction question (AC-010) turns on

- `crates/happenstance-core/src/tag.rs:79` — `Tag(Cow<'static, str>)`, and its
  `PartialEq`/`Eq`/`Ord`/`Hash` impls (`:170-193`) are all **manual**, each
  delegating to `self.as_str()` — i.e., **byte/string equality**, exactly as
  ES-37's prose and project.md's AC-010 state. There is no separate hash or
  digest field a redaction scheme could swap in without changing what equality
  (and therefore every tag-keyed index) means.
- `crates/happenstance-core/src/event.rs` — `EventParts` (`#[non_exhaustive]`)
  separates `data: Bytes` (opaque, shreddable) from `tags: Tags` (queryable,
  not opaque) — the structural fact ES-37 cites as the argument for redaction
  over delete: shredding can cover `data` but "cannot cover `tags`... and there
  is no store-side update path" (confirmed: no `set_tags`/`redact` method
  exists on `EventStore` in `store.rs`).

### 4.3 The replication seam this project's reader runs through

- `crates/happenstance-sync/src/ingest.rs` — `IngestStore`, a coherence-based
  extension trait (local trait, foreign type) rather than a growth of
  `EventStore::append`. Its own doc comment states the unresolved half
  plainly: the write path still "cannot be written *truthfully*" for a
  now-different reason than originally recorded (worth reading in full before
  writing the "ingest path" half of AC-006 — this file is where that reader
  lives, not a new module). `happenstance-sync`'s `Cargo.toml` carries
  `publish = false` explicitly (confirmed at `crates/happenstance-sync/Cargo.toml:12`).

### 4.4 Publishable-crate surface (AC-011/AC-013's constraint)

- `crates/happenstance-core/Cargo.toml`, `crates/happenstance/Cargo.toml`,
  `crates/happenstance-testkit/Cargo.toml` all omit `publish = false` (checked
  directly — Cargo defaults an absent key to publishable), consistent with
  CLAUDE.md's "three publishable crates" and the `cargo package --list`
  assertion it names. `happenstance-testkit`'s own `Cargo.toml` comment
  (`:25`) states `MemoryFixture` is itself "a *published* item," which is the
  precedent AC-013 leans on: the instrument must not become one more
  `fixtures`-module export by accident — it belongs beside `DurableFixture` in
  `tests/`, not in `src/fixtures/`.

## 5. What has not landed yet — a planning-time fact, not a tension

Every sibling project in this initiative — including
`replication-identity-and-ingest` (HS-P0017, this project's declared
dependency) and `publication-and-positioning` (HS-P0016, which creates the
`0.2.0` baseline AC-011 diffs against) — is currently at `stage: storymap`,
`status: planning` (checked directly in each project's frontmatter). None of
ADR-0017 through ADR-0028 exists in `.kb/decisions/` yet; SY-32's answer,
ADR-0026/0027, and the `0.2.0` registry baseline are all *future* artifacts
from this project's point of view in the DAG, not present ones.

This is the initiative's normal plan-ahead-of-build sequencing
(`_decomposition.md`'s merge order), not a defect to flag — but it means this
project's architecture/testing briefs must cite SY-32 and the `0.2.0` baseline
**by id and by the mechanism that will produce them** (an ADR atom under
`.kb/decisions/`, a semver-diff artifact `publication-and-positioning` is
building), not by quoting content that does not exist in this worktree today.
DR-1 through DR-12 in project.md are already written this way (citing ids, not
content), so no correction is needed there — this is confirmation, not a
finding.

## 6. Tensions and risks worth carrying into the briefs

- **The no-surface-change constraint (DR-11/AC-011) is the sharpest edge.**
  ES-39's three candidate primitives are all **port-surface** shapes (a
  method, a set, a third `Result` variant) — none of them is free under
  AC-011. ES-40 is genuinely a documentation-only clause
  (`is_violated_by` already behaves this way; the clause only requires stating
  it), so it can be discharged without a surface change. ES-38's rule needs no
  new port surface either — it is a **testkit** rule against a **testkit**
  fixture, and `positions_are_not_reused_after_removal`'s absence from
  `for_each_event_store_rule!` is a testkit gap, not a core-crate gap. The
  actual collision risk is concentrated on ES-39 and CF-27's own rule
  (`suffix_store_is_distinguishable_from_a_young_store`, "its assertion is
  fixed by the retention decision") — if the retention decision requires a new
  `EventStore` method to be distinguishable at all, that is exactly the
  AC-012 escalation this project's Out-of-scope section already anticipates.
  This confirms — does not merely restate — the intake brief's own flagged
  "question most likely to collide with the no-surface-change constraint."
- **The `Fixture` capability-const pattern (defaulted, not required) is the
  concrete way DR-1's "instrument... cannot become a target" and AC-011's
  no-breaking-change constraint are simultaneously satisfiable inside
  `happenstance-testkit`**, should the suffix store need to declare anything
  about itself to the harness (e.g., "holds a suffix" as a fact, mirroring how
  `REOPEN`/`SECOND_HANDLE` are declared today). A **required** trait method
  would break every existing `Fixture` impl in the workspace; a defaulted
  associated const would not — this is `contract.rs`'s own documented reason
  CF-39 was defaulted, reused here as the load-bearing precedent.
- **AC-004/AC-005's "registered wrong implementation" is not a new mechanism
  to invent.** `mutation_coverage.rs`'s `REGISTRY` + `for_each_mutant!` +
  the two meta-tests already do exactly this for every other rule in the
  suite; the architecture brief should specify two new `REGISTRY` rows, not a
  parallel harness.

## Anchors used above (all verified to exist)

- `.kb/decisions/0001-async-port-flavours.md`
- `.kb/decisions/0013-position-assignment-and-visibility.md`
- `.kb/decisions/0029-msrv-raised-to-1-97-1.md`
- `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`
- `spec/SPECIFICATION.md` (§3.7 4260–4272; ES-37 4274–4297; ES-38 4299–4323;
  ES-39 4325–4349; ES-40 4351–4379; SY-32 §5.10; CF-25/26 8003–8021; CF-27
  8034–8060; portfolio table 8090–8098)
- `RUNBOOK.md:307` (ADR-0028 queue row), `RUNBOOK.md:4626-4674` (phase 14 in
  full)
- `crates/happenstance-core/src/store.rs:119` (`read`), `:213` (`append`)
- `crates/happenstance-core/src/append.rs:225-234` (`AppendCondition::is_violated_by`),
  `:239-253` (`Guard::is_violated_by`)
- `crates/happenstance-core/src/tag.rs:79` (`Tag`), `:170-193` (manual
  `PartialEq`/`Eq`/`Ord`/`Hash`), `:281` (`Tags`)
- `crates/happenstance-core/src/event.rs` (`EventParts`, `#[non_exhaustive]`)
- `crates/happenstance-testkit/src/contract.rs` (`Fixture`, `Capability`,
  `SECOND_HANDLE` `:161`, `REOPEN` `:173`)
- `crates/happenstance-testkit/src/lib.rs:84-89`
- `crates/happenstance-testkit/src/registry.rs:94` (`for_each_event_store_rule!`)
- `crates/happenstance-testkit/tests/fixture_instruments.rs` (`DurableFixture`)
- `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`
- `crates/happenstance-testkit/tests/mutation_coverage.rs:324` (`REGISTRY`),
  `:2754` (`mutant_registry_is_exhaustive`), `:2889`
  (`mutants_fail_exactly_their_declared_rules`)
- `crates/happenstance-sync/src/ingest.rs` (`IngestStore`)
- `crates/happenstance-sync/Cargo.toml:12` (`publish = false`)
- `crates/happenstance-core/Cargo.toml`, `crates/happenstance/Cargo.toml`,
  `crates/happenstance-testkit/Cargo.toml` (no `publish = false`)
- `.redkiln/config.yaml` (`verify:` block — `integration_scoped` is this
  project's non-terminal bar, `cargo xtask ci --fast`)
- `CLAUDE.md` (the rule that matters; the two corollaries; the skeleton
  discipline)
