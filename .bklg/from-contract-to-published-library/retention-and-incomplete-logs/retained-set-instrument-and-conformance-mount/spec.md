---
item: HS-S0114
stage: spec
created: 2026-08-12T13:47:55.143Z
updated: 2026-08-12T13:47:55.143Z
template_sig: 87bbf1d0
rendered_sig: 7bd7aafd
---

# Spec — The completeness instrument, mounted through the conformance suite

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` (architecture: DA-1 – DA-4, *Composition roots*, *Seam map*; testing: AC-001/AC-002/AC-003 rows, *Fixtures and seams*) |
| Signed-off design | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — **surfaces: N/A, approved 2026-08-12**. This project renders none, so no surface obligation falls on this story. |
| Grounding | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` (§4.1 the `Fixture` contract; §6 why ES-38 is a testkit gap, not a core gap) |
| Story map row | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md` — slice `forgetting-instrument`, row 1 |
| Roadmap pointer | `RUNBOOK.md:4626-4674` (phase 14), `RUNBOOK.md:165` (its status row) |

## One-line PR slice

Land the completeness instrument as a decorator over a live inner `MemoryEventStore` parameterised by a
retained-set predicate (suffix **and** scattered, per DA-1), forgetting by hiding at the port rather than by
rebuilding the inner store (DA-2 — the rebuild is ES-38's mutant), with a `Fixture` that presents an *empty*
store at `connect()` like `MemoryFixture`, a filtering `read` that stays non-`async` and returns the stream at
the top level with `ReadOptions` applied over the retained set, positions unique and strictly monotonic across
the hole, and the whole thing mounted through `happenstance_testkit::event_store_conformance!` in a new
`crates/happenstance-testkit/tests/` target — never `src/fixtures/`, never named by any of the three
publishable crates' public API, and documented as an instrument that is never a target.

## Executive summary

This PR lands **one file** — a new native-only integration-test target in `happenstance-testkit` — carrying a
retained-set decorator, its `Fixture`, and the `event_store_conformance!` invocation that mounts it. Nothing
under `src/` changes: no rule is added, no registry line is touched, no `Fixture` item is added, no
`happenstance-core` signature or doc moves. That is the whole delta.

The **pointer**: `crates/happenstance-testkit/tests/fixture_instruments.rs` is the exact precedent — an
axis-specific, testkit-adjacent instrument wrapping a live `MemoryEventStore`, mounted at `:201-205`. Copy
its *mounting*; do not copy its *construction* (`DurableFixture::reopen` rebuilds through
`MemoryEventStore::restore` at `:190-193`, which is DA-2(a), the rejected shape and ES-38's own mutant).

The **delta** against that precedent is four things, and each is a decision this spec makes rather than
leaves open: forgetting is *hiding at the port*, so the inner store keeps everything and positions never
renumber; the decorator owns the two read semantics forgetting actually changes (which rows exist, and where
`limit` is counted) while delegating the rest to core so a decorator bug cannot be mistaken for a forgetting
signal; the append **condition** is evaluated against the retained view, not the inner log, because a store
that has genuinely destroyed history cannot see what it destroyed; and the committed mount is the **control**
— the configuration whose retained set hides nothing — so that a later red rule under a real hole is
attributable to forgetting.

What this PR deliberately does **not** produce is the experiment's answer. Running the suite under a real
hole and committing the enumerated per-configuration pass list is the slice-mate
`cf-27-experiment-and-recorded-pass-list`'s (AC-002). This story is the instrument the experiment needs, and
it is delivered mounted, because under `CLAUDE.md`'s rule that matters an unmounted instrument does not exist.

## Context pack

Everything below is a decision this story must honour. Nothing here is optional and nothing here is a
reading list; the deeper artifacts sit behind the anchors table the second pass appends.

**The instrument is CF-27's, and CF-27 is `[DEFERRED]` and owned by this pass.** `spec/SPECIFICATION.md:8034-8060`
asks for "a testkit-adjacent store that deliberately holds only a suffix of its own log", built "as a
decorator over any `EventStore`", with the full suite run against it. Its `Rejects:` is the whole motive: a
90-day prune happens entirely outside the port, `EventStore` has two write-visible methods and neither
deletes, and afterwards the store passes every rule unchanged — *a holed log and a young log are the same
value* (§3.7, `spec/SPECIFICATION.md:4263-4272`).

**Decision — the retained set is arbitrary, not a suffix.** DA-1 widens CF-27's own wording, and the widening
is load-bearing rather than cosmetic. ES-38 and ES-39 both say "a scattered subset"
(`spec/SPECIFICATION.md:4309-4311`, `:4326-4328`), and ES-39's argument against `earliest_position()` is that
a regulated purge is *scattered, not a prefix*, so a floor "ships looking correct until a claim runs long"
(`:4336-4341`). A suffix-only instrument therefore **structurally cannot falsify the floor** — it makes the
wrong answer look right. So: the retained set is a parameter, and both a suffix configuration and a scattered
one (survivors below the hole) must be constructible from this story's types. The widening is recorded in the
instrument's own rustdoc here, and carried into ADR-0028 by
`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/adr-0028-and-the-open-question-wave/`.

**Decision — forget by hiding at the port; the rebuild is the mutant, not the instrument.** DA-2 settles this
against the code. `MemoryEventStore::append` assigns `position_at(first_index + offset)` where
`first_index = stored.len()` (`crates/happenstance-core/src/memory.rs:386-389`, `position_at` at `:277-282`):
positions come from the **`Vec` index**, not from the last position assigned. A store rebuilt through
`MemoryEventStore::restore` (`:163`) from a truncated snapshot therefore assigns `1, 2, 3…` again and reuses
positions it has already handed out — which is ES-38's `Rejects:` verbatim, *"an adapter that renumbers on
compaction"* (`spec/SPECIFICATION.md:4321-4323`). That construction is a free, faithful **mutant**, and it
belongs to `positions-are-not-reused-after-removal`, not here. This story's instrument keeps the inner store
whole and filters what the port reports, so positions stay unique and strictly monotonic across the hole.

**Decision — the decorator changes only what forgetting changes.** DA-2's second mechanical trap is that
filtering moves what `ReadOptions` mean: `limit` applies *after* filtering
(`read_limit_applies_after_filtering`, `crates/happenstance-testkit/src/registry.rs:130`) and `from` names a
position, not an index (`crates/happenstance-testkit/src/suite.rs:1490-1531`). A decorator that drops rows
*after* the inner store applied `limit` returns short pages and fails several rules for a decorator bug
rather than for forgetting. So the decorator delegates the caller's `query`, `from`, `to` and `backwards` to
the inner store with `limit` **cleared**, then drops forgotten rows and applies `limit` itself. Core keeps
owning query matching and ordering; the instrument owns exactly the two things a hole affects.

**Decision — `read` stays non-`async` and returns the stream at the top level.** `CLAUDE.md` constraint 3 and
[ADR-0001](.kb/decisions/0001-async-port-flavours.md) / [ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md):
nesting the stream inside a future silently drops `+ Send` on the `Send` flavour and defeats the two-trait
design. The filtering therefore happens **inside a `Stream`**, never by collecting in an `async fn`.
`DurableHandle::read` merely forwards (`crates/happenstance-testkit/tests/fixture_instruments.rs:126-133`) and
is not a precedent for filtering. `happenstance-testkit` depends on `futures-core` only
(`crates/happenstance-testkit/Cargo.toml:33`), so the adapter is hand-written — the same reason
`happenstance_core::collect` is hand-written (`crates/happenstance-core/src/store.rs:285`).

**Decision — the append condition is evaluated over the retained view.** This is the sharpest correctness
point in the story and it is easy to get wrong by delegating. `AppendCondition::is_violated_by`
(`crates/happenstance-core/src/append.rs:225-234`) and `Guard::is_violated_by` (`:239-253`) are pure
predicates over events that *still exist*, and the two-arm `match` has no third arm — so over a store whose
matching history is gone the condition passes **vacuously**, which is ES-40's specified behaviour
(`spec/SPECIFICATION.md:4351-4379`). If this instrument's `append` handed the condition down to the inner
store, the inner store would see the forgotten events and reject, and the instrument would model nothing —
`condition_over_removed_history_does_not_reject` would be unwritable against it and the story after next
would be blocked. So the decorator evaluates the condition itself against its retained set and delegates the
write unconditionally. Ordering is preserved: an empty batch is still refused **before** the condition is
looked at (`empty_batch_is_refused_before_the_condition_is_evaluated`,
`crates/happenstance-core/src/store.rs:205-212`).

**Decision — the fixture presents an *empty* store at `connect()`, exactly like `MemoryFixture`.** DA-3 names
the two dishonest resolutions and they are forbidden here. **Pre-seeding** — a `connect()` that hands back a
store already holding history — fails `reading_an_empty_store_yields_nothing`,
`head_of_an_empty_store_is_none`, `condition_against_an_empty_store_admits_the_append` and
`two_fixture_instances_observe_none_of_each_others_appends`, all for reasons about *seeding* and none about
*forgetting*; it would satisfy AC-003 by the letter and destroy AC-002 entirely, because the pass list is
evidence only if every failure excluded from it is about completeness. **An eager sliding window** is the same
defect with more noise. `MemoryFixture::new()` over a new, empty `MemoryEventStore`
(`crates/happenstance-testkit/src/fixtures.rs:245-250`, `:270-292`) is the shape to copy.

**Decision — the committed mount is the control.** Because the fixture starts empty and this story writes no
rule that triggers forgetting, the honest configuration to *commit as a green test target* is the one whose
retained set hides nothing: it proves the decorator's `read`, `head`, `contains_event_id` and `append` are
faithful, which is what makes any later red under a real hole attributable to forgetting rather than to the
decorator. This is precisely the role `fixture_instruments.rs` describes for itself — *"what stays here is the
**control**: the correct sibling, running the whole suite"* (`:36-38`). Configurations that hide something are
committed as further mounts **only where the suite is green against them**; a configuration that turns rules
red is triaged (decorator bug → fix here; genuine incompleteness signal → recorded and handed to
`cf-27-experiment-and-recorded-pass-list`), and **is never resolved by shrinking the hole until it goes
green**. That is DA-3's dishonesty inverted, and it is equally forbidden.

**Decision — add nothing to `Fixture`.** DA-4 pre-authorises a **defaulted** `Capability` const plus a
**defaulted** panicking method mirroring `MID_BATCH_FAULT`
(`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`) *if a rule needs the store to lose
something during the rule*. This story writes no rule, so it adds none — DA-4's own closing instruction: *"Add
nothing if the rules can be written without it. A defaulted const nobody consults is one more line every
adapter author reads and no information."* A **required** item here would break every `Fixture` impl in and
out of the workspace and blow AC-011.

**Decision — one concrete inner store, and the narrowing is recorded.** CF-27 says "over any `EventStore`",
and a fully generic decorator over `S: EventStore` is more work and buys nothing the experiment needs (the
architecture brief's *Notes*). The inner store is a live `MemoryEventStore`. Record it in the instrument's
rustdoc as a deliberate narrowing so the clause and the instrument agree afterwards.

**The persona-journey slice.** The reader is an adapter author and the repository owner running CF-27's
experiment. What they get from this PR is a store that can be told what it no longer holds, handed to the
same macro every other fixture is handed to, and a suite run whose green means the decorator is faithful.
What they must *not* be able to conclude from it is that the completeness axis is discharged: hiding at the
port discharges **falsifiability** and not **implementability** (CF-25/CF-26,
`spec/SPECIFICATION.md:8003-8032`), the far end of the axis still wants "a device adapter second"
(`:8090-8098`), and that residual exposure is recorded open in the instrument's own documentation rather than
implied closed.

**AC-013's discipline, stated once.** The instrument lives in `crates/happenstance-testkit/tests/`, never in
`src/fixtures/` — `crates/happenstance-testkit/Cargo.toml:23-26` states in the crate's own words that
`MemoryFixture` there is *"a published item"*, and that is the precedent this must not repeat. It declares no
crate of its own. It is documented as an instrument that is never a target, in `CLAUDE.md`'s strongest form.
One nuance to state rather than discover: files under `tests/` are *shipped inside the `.crate` archive* by
`cargo package`, so "not published" here means **not nameable** — nothing in the public API of
`happenstance-core`, `happenstance` or `happenstance-testkit` refers to it, and no downstream crate can
import it.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate consumed by three capability stories in this same
  project (`cf-27-experiment-and-recorded-pass-list`, `decision-model-and-ingest-observed`,
  `projection-runner-across-the-hole`, per the story item's `blocks:` list). Never a double, never a
  `todo!()`.
- **Slice / milestone**: `forgetting-instrument`. Slice-mate: `cf-27-experiment-and-recorded-pass-list`
  (`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/`),
  which runs the suite in both configurations and commits the pass list. The two are implemented in one
  context and mounted as one integrated surface; this story merges first (`_storymap.md` *Merge order* 1).
- **Mount point**: `crates/happenstance-testkit/tests/completeness_instrument.rs` — a **new** native-only
  integration-test target carrying `happenstance_testkit::event_store_conformance!`, whose definition is
  `crates/happenstance-testkit/src/lib.rs:312-356`. Copy the invocation shape at
  `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` (`mod_name = …, fixture = …`) verbatim.
  This is composition root 1 of the architecture brief. Until this invocation exists the instrument "does not
  exist" under `CLAUDE.md`'s rule that matters, and AC-001 is not met by a type that merely compiles. Test
  targets are auto-discovered — `crates/happenstance-testkit/Cargo.toml` declares no `[[test]]` sections — so
  the mount needs no manifest edit.
- **Wires into**:
  - `happenstance_testkit::Fixture` / `Capability` (`crates/happenstance-testkit/src/contract.rs:120-321`) —
    `type Store: EventStore` at `:125` is the weaker bound and accepts both flavours; `SECOND_HANDLE` (`:161`,
    a MUST) and `REOPEN` (`:173`, a SHOULD) are the two required consts and must be answered deliberately.
  - `happenstance_core::{EventStore, SendEventStore}` (`crates/happenstance-core/src/store.rs:119-269`) —
    `read` at `:119-123`, `append` at `:213-217`, `head`, `contains_event_id` at `:268`.
  - `happenstance_core::MemoryEventStore` (`crates/happenstance-core/src/memory.rs`) — the inner store;
    `snapshot()` (used at `crates/happenstance-testkit/tests/fixture_instruments.rs:151`) is how the decorator
    resolves an `EventId` to a position and how it materialises its retained view.
  - `happenstance_core::{AppendCondition, ConditionViolated, ReadOptions, Query}` —
    `AppendCondition::is_violated_by` (`crates/happenstance-core/src/append.rs:225-234`),
    `ConditionViolated::at` / `::unspecified` (`crates/happenstance-core/src/error.rs:150-165`),
    `ReadOptions`'s four `pub` fields on a `#[non_exhaustive]` `Copy` struct
    (`crates/happenstance-core/src/query.rs:267-289`).
  - `crates/happenstance-testkit/src/fixtures.rs:186-292` — `MemoryHandle` / `MemoryFixture`, the reference
    `Fixture` impl whose `connect()` shape and honest `Capability::declined` reason are the models.
  - `futures_core::Stream` (`crates/happenstance-testkit/Cargo.toml:33`) — the only stream dependency
    available; no `futures-util`.
- **Renders surfaces**: **none.**
  `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md:38-48` records
  `## Surfaces` and `## Items` as `N/A — no user-facing surface`, approved at the design sign-off gate on
  2026-08-12. There is no surface id for this story to claim and no `## Signatures` block to match. The
  design's binding content for this story is its framing paragraph (`:21-36`): the instrument belongs in
  `crates/happenstance-testkit/tests/`, and any `Fixture` addition is scoped under AC-A05/AC-A06/AC-011 — this
  story makes none.
- **Conformance rule(s)**: **none added, and none may be.** This story is observed by the *whole existing
  event-store family* running green against the instrument through `for_each_event_store_rule!`
  (`crates/happenstance-testkit/src/registry.rs:94-102`, which feeds `__emit_tokio` and the three other
  emitters). Adding a rule here would be a rule written before the experiment that is its evidence — the
  architecture brief's own sequencing note (*"a rule written first will be written to the answer someone
  expected"*). ES-38's and ES-40's rules belong to the `owed-rules-and-mutants` slice.
- **Clause(s)**: discharges no clause and amends none. It **builds** what CF-27
  (`spec/SPECIFICATION.md:8034-8060`) defers, so CF-27's `[DEFERRED]` marker moves in
  `marker-moves-and-spec-trace-green`, not here; it supplies ES-40's named `[PROVISIONAL]` falsifier
  (`:4351-4379`) and the instrument ES-38's unwritten rule waits on (`:4299-4323`, `[FROZEN]`, *Owner: phase
  14*). **No `spec/SPECIFICATION.md` edit is in this PR** — a marker moved before the rule exists fails
  `spec-trace` checks 4 and 6 (`xtask/src/spec_trace.rs:700-711`, `:727`).
- **Advances DoD scenario**: initiative DoD **15** — *"Incomplete logs have an answer on disk … a store that
  holds only a suffix of its own log is exercised against a reader"*
  (`.bklg/from-contract-to-published-library/initiative.md:402-404`). This story lands the store half; the
  reader half is `readers-against-the-hole`'s and the answer-on-disk half is `the-retention-decision`'s.

## PR boundary

```
crates/happenstance-testkit/tests/**
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/**
```

**In this PR**

- One new native-only test target, `crates/happenstance-testkit/tests/completeness_instrument.rs`, holding:
  the retained-set type, the decorating handle, the `Fixture` impl, and the `event_store_conformance!`
  invocation(s) that mount it.
- Module- and item-level rustdoc recording the three narrowings/widenings this story takes: the retained set
  is arbitrary despite CF-27's word "suffix"; the inner store is concrete rather than generic; hiding at the
  port discharges falsifiability and not implementability (CF-25/CF-26 residual, recorded open).
- The story's own backlog folder (this spec, its ledger, its implementation report).

**Explicitly not in this PR**

- **No `crates/happenstance-testkit/src/**` change at all** — no rule body in `suite.rs`, no line in
  `for_each_event_store_rule!`, no `Fixture` item in `contract.rs`, nothing in `src/fixtures.rs`.
- **No mutant.** DA-2(a)'s renumbering store is `positions-are-not-reused-after-removal`'s, together with its
  `REGISTRY` row (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`).
- **No `happenstance-core` change**, of signature or of rustdoc. ES-40's documentation sentence is
  `condition-over-removed-history-does-not-reject`'s.
- **No `spec/SPECIFICATION.md` edit**, no marker move, no `(new)` removal.
- **No recorded pass list and no reader observation.** Those are the slice-mate's and
  `readers-against-the-hole`'s.
- **No ADR, no `.kb/` file, no `.kb/_intake/` staging.**
- **No `Cargo.toml` edit** — the target is auto-discovered and needs no new dependency.

**Merge DoD (one line)** — `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is green with the
new conformance target running, the instrument reachable only through `event_store_conformance!`, and
`git diff --stat` touching nothing outside the two globs above.

## Behavior and interfaces

Names below are proposals except where marked **binding**; the architecture brief's own data-flow diagram
(`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:401-411`) calls the
handle `ForgettingHandle`, and keeping its vocabulary is the cheapest way for the next reader to match the two
documents.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The retained set is an inspectable value, not a closure** | `RetainedSet` is an enum — at minimum `All` (the control), `AtOrAbove(SequencePosition)` (the suffix/prune case) and `Excluding(BTreeSet<SequencePosition>)` (the scattered regulated-purge case, survivors below the hole) — with one method, `retains(&self, position: SequencePosition) -> bool`. **Binding: it must derive `Debug` and name its configuration.** A boxed `Fn` predicate would be smaller and is rejected: the slice-mate commits a pass list *per configuration*, and a recorded result whose configuration cannot be printed is not data. Both DA-1 configurations must be constructible from this type without editing it. | `spec/SPECIFICATION.md:4309-4311`, `:4326-4328`, `:4336-4341`; `_decomposition.md` DA-1 (`:150-171`) |
| **Forgetting hides; the inner store keeps everything** | `ForgettingFixture` owns `Arc<MemoryEventStore>` plus the `RetainedSet`. Nothing is ever removed from the inner store, and `MemoryEventStore::restore` is **never called**. Positions therefore continue from the inner store's `Vec` length and are never reused: an append after a hole opens allocates above the hole, not into it. | `crates/happenstance-core/src/memory.rs:386-389`, `:277-282`, `:163`; `spec/SPECIFICATION.md:4321-4323`; `_decomposition.md` DA-2 (`:173-232`) |
| **`read` is non-`async` and filters inside the stream** | **Binding.** Signature mirrors `crates/happenstance-testkit/tests/fixture_instruments.rs:126-133`: `fn read(&self, query: &Query, options: ReadOptions) -> impl futures_core::Stream<Item = …> + Send`. Implementation: copy `options`, clear `limit`, call `self.inner.read(query, inner_options)`, `Box::pin` the result, and return a hand-written `Stream` that polls it, skips rows whose position the `RetainedSet` does not retain, and counts the caller's `limit` down over what survives. `Box::pin` rather than hand pin-projection so no `unsafe` appears in an instrument; the cost is one allocation per read in a test. | `CLAUDE.md` constraint 3; [ADR-0001](.kb/decisions/0001-async-port-flavours.md); [ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md); `crates/happenstance-testkit/Cargo.toml:33` |
| **`ReadOptions` mean what the suite says over the retained set** | `limit` is applied by the decorator *after* filtering, never by the inner store; `from`/`to`/`backwards` are delegated so core keeps owning bounds and ordering, and `from` continues to name a **position, not an index** — reading backwards from an unoccupied position must still yield the next retained event below it. The rejected alternative — the decorator re-implementing query matching and ordering over `snapshot()` — loses because a re-implementation bug and a forgetting signal become indistinguishable, which is exactly what the control mount exists to prevent. | `crates/happenstance-testkit/src/registry.rs:126-138`; `crates/happenstance-testkit/src/suite.rs:1490-1531`; `crates/happenstance-core/src/query.rs:267-289`; `_decomposition.md:226-232` |
| **`head` reports the highest *retained* position** | Not the inner store's head. A store that hides its top and still reports it is lying in the one place `head_is_the_highest_visible_position` and `head_advances_across_two_handles` can see, and would make the pass list meaningless. | `crates/happenstance-testkit/src/registry.rs:144-147`; `_decomposition.md:401-411` |
| **`contains_event_id` answers `false` for a forgotten event** | Resolve the id to a position through the inner store's `snapshot()` and consult the `RetainedSet` — **not** through `EventId::position()`. The position half of an id belongs to the *origin* store for an ingested event, and reducing membership to it is precisely the reduction DA-8 shows to be wrong under forgetting. This wrong-but-plausible `false` is the observation `decision-model-and-ingest-observed` consumes; this story only has to make it truthful about the retained set. | `crates/happenstance-core/src/store.rs:250-269`; `_decomposition.md` DA-8 (`:381-397`) |
| **`append` evaluates the condition over the retained view, then writes unconditionally** | Order: (1) empty batch → delegate straight to the inner store so `AppendError::NoEvents` (`crates/happenstance-core/src/error.rs:219-225`) still precedes any condition evaluation; (2) `Some(condition)` → scan the retained events and return `AppendError::ConditionViolated(ConditionViolated::at(p))` on the first retained event for which `condition.is_violated_by(p, ty, tags)` holds; (3) delegate `inner.append(events, None)`. Delegating the condition instead would let the inner store see the forgotten history and reject — modelling nothing, and blocking `condition_over_removed_history_does_not_reject` two stories later. | `crates/happenstance-core/src/append.rs:225-234`, `:239-253`; `crates/happenstance-core/src/store.rs:205-217`; `crates/happenstance-core/src/error.rs:150-165`; `spec/SPECIFICATION.md:4351-4379` |
| **Check-then-write is serialised across handles** | The evaluate-then-append pair shares one `tokio::sync::Mutex` held in the fixture and cloned into every handle, so `racing_conditional_appends_elect_one_winner` still elects one winner. A `std::sync::Mutex` is wrong here: the guard would be held across an `.await` and `clippy::await_holding_lock` is a workspace-level deny — the same hazard `fixture_instruments.rs:140-145` records. Legitimate because the target is native-only and the emitter is `__emit_tokio`. | `crates/happenstance-testkit/tests/fixture_instruments.rs:140-145`; `crates/happenstance-testkit/Cargo.toml:50-51`; `crates/happenstance-testkit/src/registry.rs:89-91` |
| **The fixture presents an empty store at `connect()`** | `connect()` is an `Arc` clone handing back a handle onto a store holding nothing, exactly `MemoryFixture`'s shape — the handle **owns a refcount** rather than borrowing a lifetime, which is what keeps `Fixture::Store` an ordinary associated type instead of a GAT. No pre-seeding, no eager sliding window; both are named dishonest resolutions. | `crates/happenstance-testkit/src/fixtures.rs:243-292`; `crates/happenstance-testkit/src/contract.rs:100-111`; `_decomposition.md` DA-3 (`:234-274`) |
| **Capabilities are answered deliberately and honestly** | `SECOND_HANDLE = Capability::SUPPORTED` — it is the one MUST, and two handles onto one `Arc<MemoryEventStore>` genuinely observe each other. `REOPEN` is **declined with the real reason**, following `MemoryFixture`'s wording: there is no durable medium behind the inner store, and the one rebuild that could imitate one is DA-2(a), the mutant. No `MID_BATCH_FAULT`, no new `Fixture` item of any kind. | `crates/happenstance-testkit/src/contract.rs:127-211`; `crates/happenstance-testkit/src/fixtures.rs:270-292` |
| **One flavour, no `#[async_trait]`** | The handle implements `SendEventStore` concretely, as both existing instruments do; `Fixture::Store: EventStore` is the weaker bound and accepts it. Import only one of the two names in the module (constraint 4) — having both in scope makes the method calls ambiguous. `#[async_trait]` is forbidden outright. | `CLAUDE.md` constraints 1 and 4; `crates/happenstance-testkit/src/contract.rs:120-125`; `crates/happenstance-testkit/tests/fixture_instruments.rs:123-133`; `crates/happenstance-testkit/src/fixtures.rs:191-217` |
| **Mounted, and mounted as the control** | At least one `event_store_conformance!` invocation with `RetainedSet::All`, shaped exactly as `fixture_instruments.rs:201-205`, distinct `mod_name`. Further invocations for hole-carrying configurations are committed **only where the suite is green**; a red rule is triaged as decorator bug (fix here) or genuine incompleteness signal (record, hand to the slice-mate, halt). **Never widen the retained set until the suite goes green** — that is DA-3's dishonesty inverted. | `crates/happenstance-testkit/src/lib.rs:312-356`; `crates/happenstance-testkit/tests/fixture_instruments.rs:36-38`, `:201-205`; `_decomposition.md:234-274` |
| **Instrument, never a target — documented, not just located** | Lives in `tests/`, never `src/fixtures/`; declares no crate; nothing in the three publishable crates' public API names it. Module docs state that it is an instrument that is never a target, that its retained set is arbitrary despite the clause's word "suffix", that the inner store is concrete (a deliberate narrowing of CF-27's *"over any `EventStore`"*), and that hiding at the port discharges **falsifiability** and not **implementability**, leaving CF-25/CF-26's completeness-axis exposure open rather than implied closed. | `crates/happenstance-testkit/Cargo.toml:23-26`; `spec/SPECIFICATION.md:8003-8032`, `:8090-8098`; `CLAUDE.md` (skeleton/instrument discipline); `_decomposition.md:205-213`, `:547-555` |

## Data and migrations

**N/A.** No schema, no persisted format, no stored artifact. The inner store is a `Vec<SequencedEvent>` behind
an `RwLock` that exists for the lifetime of one test (`crates/happenstance-core/src/memory.rs`), the retained
set is an in-memory value constructed per fixture, and nothing this story writes is read by a later process.
The one *serialised* thing in the neighbourhood — the wire format of
[ADR-0016](.kb/decisions/0016-the-wire-format.md) — is untouched: this story adds no `serde` derive, no
envelope type, and no feature. The recorded artifacts this project does produce (the pass list, the reader
observations, ADR-0028) belong to later stories in the slice order.

## Acceptance criteria

Eight criteria. Every one is framed from the intent of the two people this project has —
the **adapter author** reading the testkit to learn what an honest store looks like, and the **repository
owner** running CF-27's experiment (`project.md:191-197`, `:244-247`;
`.bklg/from-contract-to-published-library/initiative.md:402-404`). "The type compiles" is not a criterion
anywhere below: AC-001's project-grain wording is *exists **and** is reachable by the suite*
(`_storymap.md:87`), and the testing brief's named wrong implementation for it is precisely the instrument
that was built and never wired into the macro (`_decomposition.md:603`).

Throughout, **control mount** means the `event_store_conformance!` invocation whose retained set is
`RetainedSet::All`, and **hole-carrying mount** means an invocation whose retained set hides something. Only
mounts that are green are committed; a red rule is triaged, never dissolved by widening the retained set
(the Context pack's control decision, `_decomposition.md:234-274`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the repository owner is about to run CF-27's experiment and needs a store that can be told what it no longer holds, **WHEN** they run `happenstance-testkit`'s test targets, **THEN** `happenstance_testkit::event_store_conformance!` has already been invoked on the retained-set fixture in its **control** configuration inside `crates/happenstance-testkit/tests/completeness_instrument.rs`, every rule the `for_each_event_store_rule!` family emits runs against it and passes, and the instrument is reachable **through that macro alone** — no hand-rolled test driver anywhere in the target, and no `#[tokio::test]` that exercises the store outside a generated module. | `cargo test -p happenstance-testkit --test completeness_instrument` — the macro at `crates/happenstance-testkit/src/lib.rs:312-356` expands the whole family at `crates/happenstance-testkit/src/registry.rs:94-102`; mount shaped as `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205`. Reachability is `_decomposition.md`'s AC-T02 (`:697-699`), checked by reading the target for any driver that calls the store directly. |
| AC-002 | **GIVEN** an experimenter who must commit a pass list **per configuration** and cannot publish a result whose configuration is unprintable, **WHEN** they construct the suffix case (a prune keeping only the top of the log) and the scattered case (a regulated purge with survivors *below* the hole), **THEN** both are values of the same retained-set type constructed without editing it, `retains()` answers correctly for a position inside the hole, above it and below it in each, and the type's `Debug` rendering names the variant **and its bound** so the recorded row identifies the shape that produced it. | Target-local unit tests in `completeness_instrument.rs` — one over each configuration's `retains()` at the three interesting positions, one asserting the `Debug` string names variant and bound. Motive is `spec/SPECIFICATION.md:4309-4311`, `:4326-4328`, `:4336-4341` (a suffix-only instrument structurally cannot falsify ES-39's floor) and DA-1 (`_decomposition.md:150-171`). |
| AC-003 | **GIVEN** an adapter author who needs to know that a position handed out before a hole opened is never handed out again, **WHEN** a hole opens and a further append lands, **THEN** the inner `MemoryEventStore` was never rebuilt — `MemoryEventStore::restore` is called nowhere in this target — the newly assigned position is strictly greater than every position the store had already assigned including forgotten ones, and the retained view still reads unique, strictly monotonic positions across the hole. | `positions_are_unique` and `positions_are_strictly_monotonic` (`crates/happenstance-testkit/src/registry.rs:141-142`) green in the hole-carrying mount, plus a target-local `append_after_a_hole_allocates_above_it` comparing the new position against the pre-hole inner head. The construction this rejects is `MemoryEventStore::restore` (`crates/happenstance-core/src/memory.rs:163`) over a truncated snapshot, which re-derives `position_at(stored.len() + offset)` (`:277-282`, `:386-389`) — ES-38's *"adapter that renumbers on compaction"* (`spec/SPECIFICATION.md:4321-4323`), owned as a mutant by `positions-are-not-reused-after-removal`. |
| AC-004 | **GIVEN** a caller paging a store that has forgotten rows in the *middle* of the range it is reading, **WHEN** it reads with a `limit`, a `from`, a `to` and/or `backwards`, **THEN** it gets `limit` **retained** events rather than a short page (the limit is spent on survivors, never on forgotten rows), `from` still names a **position and not an index** so a read from an unoccupied position still yields the next retained event, ordering and bounds still come from core — **AND** `read` is declared non-`async`, returning the stream at the top level, so the `Send` flavour keeps `+ Send` on the stream rather than on a future wrapping it. | `read_limit_applies_after_filtering`, `read_backwards_limit_applies_after_filtering`, `read_limit_truncates`, `read_limit_zero_yields_nothing`, `limit_applies_across_items_not_per_item`, `read_from_is_inclusive`, `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window`, `read_backwards_from_with_limit`, `read_from_a_gap_position` (`crates/happenstance-testkit/src/registry.rs:124-138`) green in the hole-carrying mount. The shape obligation is discharged at the signature and re-proved by a target-local generic `fn requires_send<S: Stream + Send>(_: S)` applied to the handle's `read` — the discipline of `CLAUDE.md` constraint 3, [ADR-0001](.kb/decisions/0001-async-port-flavours.md), [ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md). |
| AC-005 | **GIVEN** a reader asking the store what it currently has, **WHEN** the top of the log is outside the retained set and when an event it once held has been forgotten, **THEN** `head()` reports the highest **retained** position rather than the inner store's, and `contains_event_id` answers `false` for the forgotten event — resolved by looking the id up in the inner store's `snapshot()` and consulting the retained set, **never** by reading `EventId::position()`, because that half of an id belongs to the *origin* store for an ingested event. | `head_is_the_highest_visible_position`, `head_of_an_empty_store_is_none`, `head_advances_across_two_handles`, `contains_event_id_reports_membership` (`crates/happenstance-testkit/src/registry.rs:144-158`) green in both mounts, plus target-local `head_reports_the_highest_retained_position` and `contains_event_id_is_false_for_a_forgotten_event`. The reduction this rejects is DA-8's (`_decomposition.md:381-397`); the resulting wrong-but-plausible `false` is the observation `decision-model-and-ingest-observed` consumes. |
| AC-006 | **GIVEN** an ingest author appending under a condition to a store whose matching history has been destroyed, **WHEN** the append is submitted, **THEN** an empty batch is still refused **before** the condition is looked at; the condition is evaluated by the instrument against its **retained view alone**, so it passes vacuously where the destroyed history would have rejected — ES-40's specified behaviour, not a bug; a violation found on a *retained* event is reported as `AppendError::ConditionViolated(ConditionViolated::at(p))` naming that retained position; the write is then delegated **unconditionally** so the inner store never re-judges it; and the evaluate-then-write pair is serialised across handles so a race still elects exactly one winner. | `empty_batch_is_refused_before_the_condition_is_evaluated`, `append_rejects_empty_batch`, the whole `condition_*` family (`crates/happenstance-testkit/src/registry.rs:190-203`), `condition_rejection_is_reported_as_condition_violated`, `racing_conditional_appends_elect_one_winner` and `interleaved_appends_on_one_handle_elect_one_winner` (`:206-209`) green in both mounts; plus a target-local `condition_over_a_forgotten_match_admits_the_append`. Contract: `crates/happenstance-core/src/append.rs:225-234`, `:239-253`; `crates/happenstance-core/src/store.rs:205-217`; `spec/SPECIFICATION.md:4351-4379`. |
| AC-007 | **GIVEN** an adapter author reading this fixture to learn what an honest one looks like, **WHEN** they call `connect()` on a fresh fixture instance, **THEN** they receive a handle onto a store holding **nothing** — no pre-seeded history, no eager sliding window, both named dishonest resolutions in DA-3 — two handles from one instance observe each other's appends while two instances observe neither, `SECOND_HANDLE` is `Capability::SUPPORTED` because two handles onto one `Arc` genuinely do observe each other, `REOPEN` is **declined with the real reason** in `MemoryFixture`'s own wording, and **no item of any kind is added to the `Fixture` trait**. | `reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`, `condition_against_an_empty_store_admits_the_append`, `two_fixture_instances_observe_none_of_each_others_appends`, `two_handles_observe_each_others_appends` (`crates/happenstance-testkit/src/registry.rs:105-129`) green, and `acknowledged_writes_survive_a_reopen` (`:107`) reported as **declined with the fixture's stated reason** rather than absent from the binary. The no-trait-addition half is `git diff --stat` empty over `crates/happenstance-testkit/src/**` — DA-4's closing instruction (`_decomposition.md:276-309`), modelled on `crates/happenstance-testkit/src/fixtures.rs:243-292` and `crates/happenstance-testkit/src/contract.rs:127-211`. |
| AC-008 | **GIVEN** a future maintainer or a downstream user who might mistake the instrument for a shippable adapter, **WHEN** they look for it from outside the crate, **THEN** it is not nameable: it lives in `crates/happenstance-testkit/tests/` and never `src/fixtures/`, declares no crate of its own, and no public item of `happenstance-core`, `happenstance` or `happenstance-testkit` refers to it — **AND** when they read it, its module rustdoc states in terms that it is an instrument that is never a target, that its retained set is arbitrary despite CF-27's word "suffix" and why, that the inner store is concrete as a deliberate narrowing of *"over any `EventStore`"*, and that hiding at the port discharges **falsifiability** and not **implementability**, leaving CF-25/CF-26's completeness-axis exposure recorded **open** rather than implied closed. | Static: `git diff --stat` confined to the PR-boundary globs (so nothing lands in `src/fixtures/`); a search of the three publishable crates' `src/` for the instrument's type names returning nothing; `cargo package --list` over the three publishable crates, the existing gate step (`cargo xtask ci`), showing no `src/` item naming it; and a content review of the module docs against the four sentences above. Precedent this must not repeat: `crates/happenstance-testkit/Cargo.toml:23-26`, which calls `MemoryFixture` *"a published item"* in the crate's own words. Residual: `spec/SPECIFICATION.md:8003-8032`, `:8090-8098`. |

**Coverage of the traced project ACs.** Project **AC-001** (`project.md:191-197`) is carried by AC-001
(existence *and* reachability through the macro) with AC-002 – AC-007 supplying the behaviour that makes the
mount green rather than merely present. Project **AC-013** (`project.md:244-247`) is carried whole by AC-008 —
its three limbs (no crate of its own, absent from every publishable public API, documented as never a target)
are the three limbs of AC-008.

## Interaction quality

This story renders **no surface**, and that is a signed-off determination rather than an omission:
`_design.md:38-80` records `## Surfaces`, `## Items`, `## Signatures` and `## Anti-patterns` as
*"N/A — no user-facing surface"*, approved by the repository owner on 2026-08-12 at the `/redkiln:plan`
design sign-off gate (`_design.md:90-95`), with `design.capture` a **declared** skip.

So the two families divide as follows, and — per the rule that an invariant which is not a table row is
never gated — **every invariant that applies is already an `AC-###` row above**. This section says which row
carries it and how it is verified; it introduces no new obligation.

**COMPOSITION family — N/A, by signed-off design.** There is no composition, placement, transience,
density budget or hierarchy to honour because there is no rendered control: `_design.md:26-30` records that
the "readers" in this project are `read_decision_model`, a projection runner and `IngestStore::holds` —
Rust code paths, not a screen — and that *"no route, no DOM selector, no component exists to enumerate."*
No AC below claims a composition invariant, and none may be added by the implementer without a new design
sign-off.

**STATE family — applies in its library register, and every applicable invariant is an AC row.** The
"surface" this story does have is the one a developer actually reads: the conformance run's own output and
the instrument's rustdoc. Mapped:

| State invariant | Register it takes here | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Reachability** (the keyboard-reachability analogue) | The instrument is reachable *only* along the path every other fixture is reached along — `event_store_conformance!` — never by a bespoke driver that would let it drift out of the family | **AC-001** | The target contains no `#[tokio::test]` outside a generated module; `_decomposition.md:697-699` (AC-T02) |
| **Non-occlusion / legibility of failure** | A failure names the **rule** and the **configuration**, never "conformance failed": the emitters already name the rule, and AC-002's `Debug` requirement is what names the configuration | **AC-001**, **AC-002** | `project.md:202-204` (AC-003's rule-name requirement, `RUNBOOK.md:4667`); the `Debug` assertion in AC-002 |
| **In-place, not context-jump** (state preserved through the operation) | Forgetting is applied **in place at the port**; the inner store is not swapped, rebuilt or re-derived, so nothing the store already told a caller changes underneath it | **AC-003** | `MemoryEventStore::restore` absent from the target; `append_after_a_hole_allocates_above_it` |
| **Preserved identity through the change** (the selection/scroll analogue) | Positions and event identities survive the hole intact — an id already handed out still resolves, and a position already assigned is never reissued | **AC-003**, **AC-005** | `positions_are_unique`, `positions_are_strictly_monotonic`, `contains_event_id_reports_membership` |
| **Reversibility** | Widening the retained set restores full visibility with no rebuild and no data loss — the operation is a view change, not a destruction. **The forbidden use of that reversibility is a first-class constraint**: a red rule is never made green by widening the hole | **AC-001** (its triage clause), **AC-003** | `_decomposition.md:234-274` (DA-3's dishonest resolutions, inverted); triage recorded in the implementation report, not silently applied |
| **Honest declaration of what is unavailable** (the disabled-state analogue) | A declined capability reports the fixture's **stated reason** and still runs, rather than vanishing from the binary | **AC-007** | `acknowledged_writes_survive_a_reopen` reported as declined-with-reason; `crates/happenstance-testkit/src/contract.rs:161-211` |
| **Presentation exists at all**, in its only available medium | The instrument's documentation is the presentation, and it must carry the four load-bearing sentences rather than a bare location comment — a correctly-placed file with no rustdoc satisfies every mechanical check and teaches nothing | **AC-008** | Content review against AC-008's four named sentences |

## Error conditions

| id | condition | required behaviour | evidence |
| --- | --- | --- | --- |
| EC-001 | An append is submitted with an empty batch *and* a condition, against a store with a hole | `AppendError::NoEvents` is returned and the condition is **never evaluated**. The instrument must not reorder these by evaluating its retained view first — the ordering is a rule, not an accident | `crates/happenstance-core/src/store.rs:205-212`; `crates/happenstance-core/src/error.rs:219-225`; `empty_batch_is_refused_before_the_condition_is_evaluated` (AC-006) |
| EC-002 | The condition is violated by an event that is **retained** | `AppendError::ConditionViolated(ConditionViolated::at(p))` where `p` is the retained position that violated it. A forgotten position must never appear in a `ConditionViolated`, because the store is modelling one that cannot see it | `crates/happenstance-core/src/error.rs:150-165`; `condition_rejection_is_reported_as_condition_violated` (AC-006) |
| EC-003 | The condition would have been violated **only** by forgotten history | The append is **admitted**. This is ES-40's specified vacuous pass, not an error to be corrected; the instrument must not "helpfully" fail closed on its own gap — that shape is `condition-over-removed-history-does-not-reject`'s registered mutant | `spec/SPECIFICATION.md:4351-4379`; `crates/happenstance-core/src/append.rs:239-253` (two arms, no third); AC-006 |
| EC-004 | The inner store rejects for a reason of its own — a store limit, a `MemoryStoreError` | The error is propagated **unchanged**. The decorator maps no error type and invents none; a bespoke error here would make a store-limit failure indistinguishable from a forgetting signal, which is exactly what the control mount exists to prevent | `append_reports_exceeded_store_limits` (`crates/happenstance-testkit/src/registry.rs:187`); `crates/happenstance-core/src/error.rs` |
| EC-005 | `contains_event_id` is asked about an id this store **forgot** versus one it **never held** | Both answer `false`, and the two are indistinguishable through the port. This is **not** repaired here — it is the finding DA-8 predicts and `decision-model-and-ingest-observed` records, and repairing it would need a port surface (a tri-state answer) that is DA-7's escalation and this project's AC-012, never a change | `_decomposition.md:381-397`; `project.md:240-243`; AC-005 |
| EC-006 | A hole-carrying mount turns a rule **red** during implementation | Triage, in writing, into exactly one of two buckets: a decorator bug (fix it here, in this story) or a genuine incompleteness signal (record it, hand it to `cf-27-experiment-and-recorded-pass-list`, do **not** commit the red mount). **Never** widen the retained set until the suite goes green | The Context pack's control decision; `_decomposition.md:234-274` |
| EC-007 | A `std::sync::Mutex` guard is held across the `.await` in `append` | Rejected at the gate: `clippy::await_holding_lock` is denied workspace-wide. The serialising lock is a `tokio::sync::Mutex`, legitimate because this target is native-only under `__emit_tokio` | `crates/happenstance-testkit/tests/fixture_instruments.rs:140-145`; `crates/happenstance-testkit/Cargo.toml:50-51`; `crates/happenstance-testkit/src/registry.rs:89-91` |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| NF-001 | **No `unsafe` anywhere in the instrument.** The inner stream is `Box::pin`ned rather than hand pin-projected | An instrument that needs `unsafe` to be believed is not an instrument. The cost is one heap allocation per `read` in a test, which is not a budget this repository spends on |
| NF-002 | **No new dependency and no `Cargo.toml` edit.** The stream adapter is hand-written over `futures_core::Stream` | `crates/happenstance-testkit/Cargo.toml:33` carries `futures-core` only — no `futures-util`. Same reason `happenstance_core::collect` is hand-written (`crates/happenstance-core/src/store.rs:285`). Test targets are auto-discovered, so the mount needs no `[[test]]` section |
| NF-003 | **Native-only, and wasm32 is provably unaffected.** The target opens with `#![cfg(not(target_arch = "wasm32"))]` | Mirrors `crates/happenstance-testkit/tests/fixture_instruments.rs:40-43`. The four mandatory `wasm32` steps in `cargo xtask ci --fast` must be green and unchanged — the instrument adds no `Send` bound and no runtime to that target |
| NF-004 | **`cargo clippy -D warnings` clean**, including `await_holding_lock`; any `#![allow]` is scoped at the target and justified in one line, following the precedent's `#![allow(clippy::unwrap_used)]` | `crates/happenstance-testkit/tests/fixture_instruments.rs:43-44`; `cargo xtask ci --fast` runs clippy with `-D warnings` |
| NF-005 | **No `#[async_trait]`, and exactly one of `EventStore` / `SendEventStore` imported in the module** | `CLAUDE.md` constraints 1 and 4 — `#[async_trait]` injects `+ Send` and makes wasm32 impossible; both trait names in scope makes the method calls ambiguous. The handle implements `SendEventStore` concretely, as both existing instruments do (`crates/happenstance-testkit/tests/fixture_instruments.rs:123-133`, `crates/happenstance-testkit/src/fixtures.rs:191-217`) |
| NF-006 | **The suite run stays cheap enough to be a merge-gate step** — the filtering is O(retained) per read with no quadratic rescan of the inner log per row | The whole event-store family runs against this fixture on every `cargo xtask affected` touching the testkit; a decorator that rescans `snapshot()` per emitted row would make the gate the slowest thing in the repository for no signal |
| NF-007 | **`spec/SPECIFICATION.md` is byte-identical after this PR** | A marker moved before the rule it names exists fails `spec-trace` checks 4 and 6 (`xtask/src/spec_trace.rs:700-711`, `:727`). CF-27's marker move is `marker-moves-and-spec-trace-green`'s |

## Implementation notes (non-prescriptive)

These are the traps this spec's research turned up. None is binding on *how*; each is binding on *what must
still be true afterwards*.

- **Read `fixture_instruments.rs` twice, for opposite reasons.** Take `:201-205` (the mount) and `:140-145`
  (the async-lock reasoning) as models; take `:186-193` (`reopen` through `MemoryEventStore::restore`) as the
  shape you are *not* building. It is the single most likely accidental copy in this story, and it is ES-38's
  own mutant.
- **Clear `limit` on the way down, apply it on the way up.** The single easiest bug here is delegating the
  caller's `ReadOptions` whole: the inner store then spends the limit on rows the decorator is about to drop,
  and half a dozen read rules go red for a decorator bug wearing a forgetting costume. `from`, `to`,
  `backwards` and the `Query` go down untouched; only `limit` is withheld.
- **`ReadOptions` is `#[non_exhaustive]` with `pub` fields and is `Copy`** (`crates/happenstance-core/src/query.rs:267-289`)
  — so copy-then-mutate is available in-crate-adjacent code; check how the existing callers spell it before
  inventing a builder.
- **`snapshot()` is the resolution mechanism for both `contains_event_id` and the condition scan**
  (used at `crates/happenstance-testkit/tests/fixture_instruments.rs:151`). Take it once per operation, not
  once per row.
- **Name the handle `ForgettingHandle` and the fixture `ForgettingFixture`** unless there is a reason not to:
  the architecture brief's data-flow diagram already uses that vocabulary (`_decomposition.md:401-411`), and
  matching it costs nothing and saves the next reader a translation.
- **Write the module rustdoc first, not last.** AC-008's four sentences are the part of this story most
  likely to be lost to "it's obvious from the directory", and the directory is exactly what
  `crates/happenstance-testkit/Cargo.toml:23-26` records as insufficient.
- **If a hole-carrying mount is green, commit it.** The control is the *minimum*, not the target: every green
  configuration committed here is one the slice-mate does not have to re-establish before it can record a
  pass list.
- **Order the work Red-first where the suite permits it.** The control mount can be written and run before
  the filtering is implemented (with `RetainedSet::All` the decorator is a pass-through), which isolates
  "the decorator delegates faithfully" from "the decorator filters correctly" — and that separation is the
  whole argument for the control.

## Tests and CI (merge gate)

Grounded in the project testing brief's *Test mix* row for AC-001 (`_decomposition.md:603`) and its
*Merge-gate commands* (`:620-650`). This project is rank 5 and **not** terminal, so `cargo xtask ci --fast`
(`.redkiln/config.yaml:55`) is its ceiling; the whole gate is `closeout-and-durable-audience`'s.

| tier | command / path | proves |
| --- | --- | --- |
| unit (target-local) | `cargo test -p happenstance-testkit --test completeness_instrument` — the non-macro `#[test]`/`#[tokio::test]` items in `crates/happenstance-testkit/tests/completeness_instrument.rs` | AC-002 (both configurations constructible, `retains()` correct at three positions, `Debug` names variant and bound), AC-003 (`append_after_a_hole_allocates_above_it`), AC-005 (`head_reports_the_highest_retained_position`, `contains_event_id_is_false_for_a_forgotten_event`), AC-006 (`condition_over_a_forgotten_match_admits_the_append`) |
| integration (conformance) | the same command — the modules `happenstance_testkit::event_store_conformance!` (`crates/happenstance-testkit/src/lib.rs:312-356`) generates from `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94-102`) | AC-001 (the family runs and passes against the control mount, reachable through the macro alone), AC-004 (the ten read/limit/bound rules at `registry.rs:124-138`), AC-006 (the `condition_*` and concurrency families at `:190-209`), AC-007 (the empty-store, two-handle and two-instance rules, plus `REOPEN` reported as declined-with-reason) |
| compile-time | the generic `requires_send` application in the same target | AC-004's shape half — the stream carries `+ Send` at the top level because the bound is written at the definition, not because a future was marked `Send` ([ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md)) |
| story grain (automatic) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | the whole of the above plus fmt, clippy `-D warnings` (NF-004) and the five file-reading lints over `happenstance-testkit` and its dependents; runs `spec-trace` unconditionally, which is what holds NF-007 |
| integration grain, tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | NF-007 — no marker moved, no `(new)` removed, §7.1/§7.2 unchanged |
| integration grain, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | NF-003 — the four mandatory `wasm32` steps still green with the new native-only target present; plus the doc builds and the `cargo package --list` licence/README assertion |
| static (surface) | `git diff --stat` against the two PR-boundary globs; a search of `crates/happenstance-{core,testkit}/src/` and `crates/happenstance/src/` for the instrument's type names | AC-007's no-trait-addition half and AC-008 — nothing under `src/**`, nothing nameable from a publishable public API |
| static (content review) | the module and item rustdoc of `completeness_instrument.rs` | AC-008's documentation half: instrument-never-a-target, the arbitrary-retained-set widening, the concrete-inner-store narrowing, and the CF-25/CF-26 falsifiability-not-implementability residual left open |

**Not run here, on purpose.** `cargo xtask ci` (the whole gate) is HS-P0019's proof artefact, useful locally
and never recorded as this story's (`_decomposition.md:647-650`). `redkiln validate --kb` has nothing to
check: this story authors no atom. `cargo-semver-checks` belongs to `surface-diff-and-the-ac-012-escalation`
at the PR grain; this story's contribution to AC-011 is the *absence* of a `src/**` diff, which the row above
already proves.

## Risks and coupling (PR-scoped)

| risk | why it is real here | containment inside this PR |
| --- | --- | --- |
| **The rebuild gets copied by accident** | `DurableFixture::reopen` is fifty lines above the mount this story is told to copy, and it is a *working* forgetting mechanism — it just happens to be the mutant | AC-003 asserts positions above the hole and forbids `MemoryEventStore::restore` in the target; the Context pack names it before the code is written |
| **A decorator bug is read as a forgetting signal** | Both look identical from the suite's output: a rule name and a red mark | The control mount (AC-001) is committed first and is the *minimum*; EC-006 makes triage a required written step rather than a judgement call |
| **`limit` semantics silently invert** | Delegating `ReadOptions` whole is the shorter code and compiles cleanly | AC-004 pins six limit-related rules to the hole-carrying mount, where a whole-delegation returns short pages |
| **The story quietly grows into the experiment** | Running the suite under a hole is one command away, and the pass list is the interesting artefact | The PR boundary excludes recorded data outright; AC-002 requires the configurations to be *constructible*, not *reported*. The pass list is `cf-27-experiment-and-recorded-pass-list`'s AC-002 |
| **A `Fixture` item gets added "while we are here"** | DA-4 pre-authorises a defaulted shape, which reads as permission | AC-007's `git diff --stat` over `src/**` is empty; DA-4's own instruction is to add nothing if the rules can be written without it, and this story writes no rule |
| **Three downstream stories inherit anything wrong here** | `cf-27-experiment-and-recorded-pass-list`, `decision-model-and-ingest-observed` and `projection-runner-across-the-hole` all read *through* this instrument (`story.md` `blocks:`) | The control mount is precisely the guarantee they need: green control ⇒ any later red is attributable to forgetting. That is why it is AC-001 and not a nicety |
| **The residual gets read as closed** | A green suite against a forgetting store reads like "completeness is handled" | AC-008 requires the falsifiability-vs-implementability sentence in the rustdoc, and the Context pack states it as the persona's explicit non-conclusion (`spec/SPECIFICATION.md:8003-8032`, `:8090-8098`) |

**Coupling, stated plainly.** This PR touches one new file and no existing one. Its only coupling to the rest
of the workspace is *inbound*: it consumes `happenstance-core`'s port and `happenstance-testkit`'s `Fixture`
and macro as they stand today. Nothing outside `crates/happenstance-testkit/tests/` can observe a change from
it, which is what makes `cargo xtask affected --base main` a sufficient story-grain gate.

## Dependencies

**Blocks-on: none.** `depends_on: []`, matching `story.md`'s `blocked_by: []` and `_storymap.md:68` (the
`depends_on` column reads `—`). Everything this story consumes already exists in the tree today and was
verified while writing this spec: `happenstance_testkit::event_store_conformance!`
(`crates/happenstance-testkit/src/lib.rs:312-356`), `Fixture` / `Capability`
(`crates/happenstance-testkit/src/contract.rs`), `MemoryEventStore`
(`crates/happenstance-core/src/memory.rs`) and the mount precedent
(`crates/happenstance-testkit/tests/fixture_instruments.rs:201-205`). It is `_storymap.md`'s merge-order
position 1 for exactly that reason: *"Nothing else in this project can merge first."*

**Unlocks** (`story.md` frontmatter `blocks: [HS-S0115, HS-S0118, HS-S0119]`):

| story slug | what it takes from this one |
| --- | --- |
| `cf-27-experiment-and-recorded-pass-list` (slice-mate) | the mounted instrument in both DA-1 configurations; it runs the suite and commits the enumerated per-configuration pass list. Implemented in the same context as this story and mounted as one integrated surface; this story merges first |
| `decision-model-and-ingest-observed` | a store to run `read_decision_model` and `IngestStore::holds` against — AC-005's truthful-but-uninformative `false` is the observation it records |
| `projection-runner-across-the-hole` | a store with a hole for the projection runner to resume across |

**Not a dependency, deliberately.** `positions-are-not-reused-after-removal` and
`condition-over-removed-history-does-not-reject` are downstream in merge order but carry no edge to this
story in `_storymap.md`; they depend on the *experiment*, not on the instrument. Their mutants — including
DA-2(a)'s renumbering store, which falls out of this story's rejected construction — are theirs to write.

## Anchors (progressive disclosure)

Open these when the bound criterion is the one in hand. Everything load-bearing that this spec distilled
rather than reproduced is here; nothing below was pasted in bulk, and every path was confirmed to exist
while writing this spec.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | The one precedent for a testkit-adjacent instrument wrapping a live `MemoryEventStore`. `:201-205` is the mount to copy verbatim; `:140-145` is the async-lock reasoning to reuse; `:36-38` states the control's role in the author's own words; `:186-193` is the rebuild to **not** copy | First, before writing any code — and again at `:186-193` specifically before touching anything that resembles reopening | AC-001, AC-003, AC-006 |
| `crates/happenstance-testkit/src/lib.rs` | `event_store_conformance!` at `:312-356` — the exact macro arity and the `mod_name` / `fixture` spelling the mount must match | When writing the mount invocation | AC-001 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!` at `:94-102` is the authoritative list of what will run against the instrument; the rule names cited throughout the AC table are its lines `:105-217` | When triaging any red rule, to see which family it belongs to before deciding decorator-bug vs signal | AC-001, AC-004, AC-005, AC-006, AC-007 |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` contract: `type Store: EventStore` at `:125`, `SECOND_HANDLE` at `:161` (a MUST), `REOPEN` at `:173` (a SHOULD), and the `MID_BATCH_FAULT` defaulted-seam precedent at `:207-211`, `:297-307` that DA-4 says to mirror **only if a rule needs it** | When writing the `Fixture` impl and answering the two capability consts | AC-007 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` / `MemoryHandle` at `:186-292` — the reference impl whose empty-at-`connect()` shape and honest declined-capability wording are the models this story copies | Immediately before implementing `connect()` and the capability declarations | AC-007 |
| `crates/happenstance-core/src/memory.rs` | `position_at` at `:277-282` and `append`'s `first_index = stored.len()` at `:386-389` are *why* a rebuild reuses positions; `restore` at `:163` is the call that must not appear | Before implementing append, and whenever tempted to model forgetting by reconstruction | AC-003 |
| `crates/happenstance-core/src/store.rs` | The port itself: `read` at `:119-123` (non-`async`, stream at the top level), `append` at `:213-217`, the empty-batch ordering at `:205-212`, `contains_event_id` at `:250-269`, and `collect` at `:285` as the hand-written-adapter precedent | When writing each method signature — the shape is copied from here, not invented | AC-004, AC-005, AC-006 |
| `crates/happenstance-core/src/append.rs` | `AppendCondition::is_violated_by` at `:225-234` and `Guard::is_violated_by` at `:239-253` — two arms and no third, which is *why* a condition over destroyed history passes vacuously rather than erroring | Before implementing the condition scan; re-read if the vacuous pass ever feels like a bug to fix | AC-006 |
| `crates/happenstance-core/src/query.rs` | `ReadOptions` at `:267-289` — a `#[non_exhaustive]` `Copy` struct with four `pub` fields; how to copy-and-clear `limit` without a builder | When writing the delegating `read` | AC-004 |
| `crates/happenstance-core/src/error.rs` | `ConditionViolated::at` / `::unspecified` at `:150-165` and `AppendError::NoEvents` at `:219-225` — the exact error values EC-001 and EC-002 require | When writing `append`'s error paths | AC-006 |
| `spec/SPECIFICATION.md` | CF-27 at `:8034-8060` (the experiment this instrument *is*), ES-38 at `:4299-4323` (the renumbering mutant, verbatim), ES-39 at `:4326-4341` (why a scattered purge defeats a floor — the argument DA-1's widening rests on), ES-40 at `:4351-4379` (the specified vacuous pass), §3.7 at `:4263-4272` (a holed log and a young log are the same value), CF-25/CF-26 at `:8003-8032` and `:8090-8098` (the residual left open) | Open CF-27 and §3.7 before starting; ES-38 before AC-003; ES-39 before AC-002; ES-40 before AC-006; CF-25/CF-26 before writing the rustdoc | AC-002, AC-003, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | The architecture brief's DA-1 (`:150-171`, arbitrary retained set), DA-2 (`:173-232`, hide-don't-rebuild + the `ReadOptions` trap), DA-3 (`:234-274`, the two dishonest resolutions and the control), DA-4 (`:276-309`, add nothing to `Fixture`), DA-8 (`:381-397`, the `contains_event_id` reduction), the data-flow vocabulary (`:401-411`), and the testing brief's AC-001 row (`:603`) and merge-gate commands (`:620-650`) | DA-1/DA-2 before the type; DA-3 before committing any mount; DA-4 if a `Fixture` addition is ever contemplated; the testing brief before running the gate | AC-001 – AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | The project-grain wording of AC-001 (`:191-197`) and AC-013 (`:244-247`) this story traces to, plus AC-011/AC-012's surface-change discipline (`:235-243`) that bounds EC-005 | When checking that a criterion above still says what the project asked for; before any thought of changing a port surface | AC-001, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The signed-off no-surface determination (`:38-80`, `:90-95`) and the framing that scopes any `Fixture` addition to AC-A05/AC-A06/AC-011 (`:32-36`) | Before writing anything in the Interaction quality register, and before adding any `pub` item | AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md` | Merge order (`:126-149`), the four shaping facts (`:16-44`) — especially fact 2, that AC-003's failures are owed by mutants and never by pre-seeding — and this story's row (`:68`) | At the start, to confirm nothing has been re-sliced; and if the temptation to make a rule fail here ever arises | AC-001, AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` | The verified-anchor set behind every path in this spec: §4.1 the `Fixture` contract, §6 why ES-38 is a testkit gap and not a core gap | If any cited line number has drifted, or before concluding that `happenstance-core` needs a change | AC-006, AC-008 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted decision behind `read`'s shape: no `#[async_trait]`, one definition, `trait_variant` derives the `Send` flavour | Before writing the `read` signature | AC-004 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | Why the `Send` obligation must be discharged at the definition rather than by auto-trait leakage — the reason AC-004's compile-time assertion is written generically | Alongside ADR-0001, when writing the stream type | AC-004 |
| `crates/happenstance-testkit/Cargo.toml` | `:23-26` calls `MemoryFixture` *"a published item"* in the crate's own words — the precedent AC-008 exists not to repeat; `:33` is the `futures-core`-only dependency set; `:50-51` the tokio dev-dependency behind the async lock | Before choosing where the file goes, and before reaching for `futures-util` | AC-008, NF-002, EC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `REGISTRY` at `:324` — where DA-2(a)'s renumbering store belongs, and where it must **not** be added by this story | Only to confirm this story adds no row there | AC-003 |
| `RUNBOOK.md` | Phase 14 at `:4626-4674` — the roadmap entry this project executes, including `:4667`'s "reported as a rule name" requirement and `:4658-4668`'s observed-not-asserted discipline | For orientation on where this sits in the plan of record; and when deciding how a red rule is reported | AC-001 |
| `xtask/src/spec_trace.rs` | Checks 4 and 6 at `:700-711` and `:727` — what fails if a marker moves before its rule exists | Only if a `spec/SPECIFICATION.md` edit is ever contemplated in this PR (it must not be) | NF-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's eight**, AC-001 – AC-008. None was added and none dropped; the
   ledger matches. AC-001 and AC-008 carry the two traced project ACs whole, and AC-002 – AC-007 are the
   behaviour that makes AC-001's mount green rather than merely present.
2. **"Reachable by the suite" was read strictly.** Project AC-001 could be satisfied by a `Fixture` that
   compiles against `event_store_conformance!`'s bound. It is not: the testing brief's named wrong
   implementation for that row is *"the instrument built but never wired into the macro"*
   (`_decomposition.md:603`), so AC-001 requires the invocation to exist and its generated modules to pass.
3. **Interaction quality is answered, not skipped.** The composition family is N/A by signed-off design
   (`_design.md:38-80`, approved `:90-95`), and the state family is mapped into its library register with
   every applicable invariant already carried by an existing `AC-###` row. No prose-only invariant was left
   in that section, because a bullet there would never be extracted, gated or tested.
4. **The hole-carrying mount is required behaviour but not required *committed output*.** AC-004 – AC-006
   are verified "in the hole-carrying mount", and green hole-carrying mounts are committed. But if triage
   under EC-006 finds a genuine incompleteness signal, the red mount is **not** committed and the finding is
   handed to `cf-27-experiment-and-recorded-pass-list` — recording it here would be the pass list, which is
   outside this PR's boundary.
5. **`contains_event_id`'s ambiguity is a finding, not a defect of this story.** EC-005 states that a
   forgotten event and a never-held event are indistinguishable through the port and that this must **not**
   be repaired here. Repair needs a tri-state answer, which is a port surface change — DA-7's escalation and
   the project's AC-012, which this initiative's exit criteria do not contemplate.
6. **No AC asserts a literal position value.** `CLAUDE.md`'s rule against `[1, 2, 3]` applies to conformance
   rules, and although this story writes none, AC-003's target-local test obeys it too: it compares the new
   position against positions the store actually assigned, never against a literal.
7. **The `Debug` requirement on the retained set is load-bearing, not stylistic.** It was promoted to a full
   AC (AC-002) rather than left as an implementation note because the slice-mate's deliverable is a pass list
   *per configuration*, and a recorded result whose configuration cannot be printed is not data — DR-2's own
   failure mode one level up.
8. **Nothing here waits on `publication-and-positioning` or HS-P0011.** The `0.2.0` registry baseline is a
   citation this story never dereferences, and the projection runner is `projection-runner-across-the-hole`'s
   dependency, not this story's. `depends_on` is genuinely empty.
