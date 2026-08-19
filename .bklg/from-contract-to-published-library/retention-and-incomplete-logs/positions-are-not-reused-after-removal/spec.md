---
item: HS-S0116
stage: spec
created: 2026-08-12T13:47:56.971Z
updated: 2026-08-12T13:47:56.971Z
template_sig: 87bbf1d0
rendered_sig: 114f26ac
---

# Spec — ES-38's owed rule, and the store that renumbers on compaction

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD **15** (`:402-404`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — gate decision 4, the no-surface-change constraint (`:239-249`) |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — **AC-003** (`:202-204`), **AC-004** (`:205-209`), **AC-011** (`:236-239`), DR-3/DR-4 (`:150-159`) |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/positions-are-not-reused-after-removal/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` — architecture **DA-2** (`:173-232`), **DA-3** (`:234-274`), **DA-4** (`:276-308`), composition roots 2 and 3 (`:98-113`), seam map (`:49-86`); testing brief AC-004 row (`:605`), AC-011 row (`:612`) |
| Signed-off design | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — **surfaces: N/A, approved 2026-08-12** (`:38-48`, `:86-95`). This project renders none; the design's binding content for this story is its API-surface note (`:32-36`), which scopes any `Fixture` addition under AC-011 |
| This story's discover | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/positions-are-not-reused-after-removal/discover.md` — the signal ledger, the two questions deferred to spec (`:35`, `:39`), and the wrong implementation (`:45-54`) |
| Dependency's spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/spec.md` — the instrument this rule runs against, its type names and its mount |
| Story map row | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:70` (this story), `:133-136` (merge order inside the slice) |
| Roadmap pointer | `RUNBOOK.md:4626-4674` (phase 14 in full, including the "a rule name rather than 'conformance failed'" bar), `RUNBOOK.md:165` (its status row) |

## One-line PR slice

Write ES-38's owed rule `positions_are_not_reused_after_removal` into
`crates/happenstance-testkit/src/suite.rs`, register it exactly once in `for_each_event_store_rule!`
(`crates/happenstance-testkit/src/registry.rs:94`), give `Fixture` the **defaulted** removal seam the rule
needs in the exact shape `MID_BATCH_FAULT` set, keep it green against the honest instrument and against the
gapped conformant variant while it reports an honest skip against `MemoryFixture`, and land ES-38's own named
wrong implementation — the store that renumbers on compaction — as a `Defect` plus a `REGISTRY` row that
fails **exactly** this rule.

## Executive summary

This PR discharges an obligation that has been written down and unowned since phase 4. ES-38 is `[FROZEN]`,
it names `positions_are_not_reused_after_removal` as its rule, and it records in the clause itself why the
rule was **deliberately not written**: it needs a store events can be removed from, `Fixture` declares no such
capability, and the instrument belonged to CF-27 which was `[DEFERRED]` (`spec/SPECIFICATION.md:4309-4319`).
**Owner: phase 14.** That is this story. HS-S0114 removed the first blocker by building the instrument;
HS-S0115 ran the experiment so the rule is written *after* the evidence rather than to the answer someone
expected (`_decomposition.md:543-546`).

The delta is four edits and one of them is the whole story:

1. **`Fixture` gains the removal seam** — a **defaulted** `Capability` const plus a **defaulted** panicking
   method, mirroring `MID_BATCH_FAULT` (`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`).
   The discover deferred *"does the rule need a seam at all?"* to this spec, to be settled against the
   instrument as built. Settled below: **it does**, because HS-S0114's instrument fixes its retained set at
   construction and declines `REOPEN`, so no existing seam lets a rule make a store lose something *during*
   the rule.
2. **The rule** — one `pub async fn` in `suite.rs`, gated with `require!`, asserting only against positions
   the store itself assigned.
3. **The mutant** — `CompactingRestoreStore`, a `Defect` whose single overridden step re-derives positions
   from the survivors after a removal. This is ES-38's `Rejects:` verbatim and it is not invented here.
4. **The bookkeeping a new capability drags with it**, which is the part that is easy to miss and impossible
   to skip: a line in `harness.rs`'s hand-written `declines()` list, a row in `MUST_SKIP`, and honest support
   on `GappedPositionFixture` — because `capability_skips_are_reported` asserts that the fully-capable
   fixture skips **nothing** (`crates/happenstance-testkit/tests/mutation_coverage.rs:3306-3314`).

What this PR does **not** do: move any marker in `spec/SPECIFICATION.md` (HS-S0123's), touch
`happenstance-core` at all, or make any required trait item. ES-38's clause text is implemented, not amended.

## Context pack

Everything below is a decision this story must honour. Nothing here is a reading list; the deeper artifacts
sit behind the anchors table the second pass appends.

**The assertion is already written, in the clause.** ES-38: a store from which events have been removed by
any means outside the port *"MUST continue to satisfy every clause of this section with respect to the events
it still holds. In particular it MUST NOT reuse a position it has previously assigned, its remaining positions
MUST remain unique and strictly monotonic, and `Query::all()` MUST mean 'every event this store holds'"*
(`spec/SPECIFICATION.md:4299-4307`, `[FROZEN]`). This story writes that in Rust. It does not renegotiate it,
and it edits nothing in the clause — including the `(new)` annotation on its `Rule:` line, which comes off in
`marker-moves-and-spec-trace-green` and is raised there rather than assumed here.

**Decision — the seam is needed, and it is `Fixture`'s.** The discover left this open on purpose and named
the evidence that would settle it (`discover.md:35`). The instrument as built settles it: HS-S0114's
`RetainedSet` is a value fixed when the fixture is constructed, its fixture presents an *empty* store at
`connect()`, and it declines `REOPEN` with a real reason. So there is no existing route by which a rule can
make a store lose something mid-rule. Two no-seam alternatives were weighed and both lose:

- **Hang the rule off `REOPEN`** — a fixture that prunes as it reopens. It fails three ways. The honest
  instrument declines `REOPEN` by design, so the rule would never *run* against the store this project built.
  It silently narrows ES-38's *"removed by any means outside the port"* to *"removed at restart"*, which
  cannot observe a device that prunes while running. And a truncating-reopen mutant would also fail
  `acknowledged_writes_survive_a_reopen` and `reopened_store_does_not_reissue_an_event_id`, which destroys
  AC-004's load-bearing word — **exactly** — under `mutants_fail_exactly_their_declared_rules`.
- **Construct the fixture with the hole already present.** This is DA-3's pre-seeding, forbidden outright, and
  it cannot observe reuse anyway: nothing was assigned before the hole, so there is no previously-assigned
  position to reuse.

**Decision — the seam's shape is `MID_BATCH_FAULT`'s, item for item, and a required item is not a
trade-off.** A **defaulted** associated `Capability` const whose default is `Capability::declined(<reason>)`,
plus a **defaulted** method whose body panics naming *both* ways of reaching it — a fixture that declares the
capability and forgets the override, or a rule that arrived without a `require!` gate
(`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`). Two live rules already ride that pattern,
so the shape is proven rather than proposed. A **required** item breaks every `Fixture` impl in and out of the
workspace and is an AC-011 violation, not a cost (`_decomposition.md:276-308`, `:612`).

**Decision — the seam takes a set of positions, never a floor.** `remove_below(p)` is `earliest_position()`
wearing a fixture's clothes, and ES-39's whole argument against that primitive is that a regulated purge is
*scattered, not a prefix* — a floor *"ships looking correct until a claim runs long"*
(`spec/SPECIFICATION.md:4336-4341`). A prefix-shaped seam would let the testkit pre-decide ES-39 in a place
no application can read, which is DA-4's own test for where a thing belongs (`_decomposition.md:299-305`).
The seam therefore takes the set of positions to remove, and the rule exercises a **scattered** set with
survivors below the hole (DA-1).

**Decision — it is a `Capability`, not a fact, and the thing that is a fact has no home on `Fixture` at
all.** `crates/happenstance-testkit/src/contract.rs:44-54` draws the line: a capability is a **trade** — the
fixture could have co-operated and owes a reason — while a limit is a fact. *"Can you be made to forget?"* is
a trade. *"What do you not hold?"* is a fact, it is ES-39's port primitive, and putting it on `Fixture` would
be answering ES-39 where no application can read the answer.

**Decision — `MemoryFixture` declines, and that is the strongest sentence in the decline.** The reference
fixture is not edited and inherits the default decline. It cannot honestly do otherwise: the only way to
shrink a `MemoryEventStore` is `restore` (`crates/happenstance-core/src/memory.rs:163`), which re-derives
`position_at(first_index + offset)` with `first_index = stored.len()` (`:277-282`, `:386-389`) — so a
`MemoryFixture` that supported removal **would be the mutant**. AC-004's *"green against `MemoryFixture`"* is
therefore met as a **reported skip carrying a stated reason**, emitted as a test like every other rule, never
`#[cfg]`-ed out — the arrangement `contract.rs:31-42` rejects by name. The rule's *running*-green evidence
comes from the honest instrument and from `GappedPositionFixture`, and both are required below.

**Decision — the removal leaves the highest assigned position in place, and the residual is recorded rather
than dropped.** Every honest store in this workspace allocates the next position from the events it currently
holds: `Log::append` calls `commit(&mut self.events, …)` (`crates/happenstance-testkit/tests/mutation_coverage/correct.rs:456-480`)
and `MemoryEventStore` uses the `Vec` index. A rule that removed the store's own head would therefore oblige
every fixture to grow a high-water mark it needs for nothing else — rewriting `correct.rs`'s allocator for a
case ES-38's `Rejects:` does not name, since a device pruning to save space removes the **oldest**. So the
rule removes a scattered subset **below** the highest assigned position, which is the device-prune and
regulated-purge shape, and still catches the named mutant, whose renumbering reuses positions the store handed
out for the survivors it kept. The case not staged — removal of the store's own head — is named in the rule's
rustdoc as a residual and handed to ADR-0028, not left to be discovered.

**Decision — the mutant is a `Defect`, and it is one defect.** The testing brief's AC-004 row asks for *"a
`Defect` implementing DA-2(a)'s rejected construction"* (`_decomposition.md:605`) and that is what lands: one
overridden step in the `MutantStore<D>` family, plugged into `for_each_mutant!` and `REGISTRY` like the other
forty. The alternative — a bespoke fixture literally wrapping a real `MemoryEventStore` and calling `restore`
— loses on two counts: this binary is `Rc`-backed and single-threaded on purpose, so that it keeps exercising
the flavour ADR-0001 exists for (`crates/happenstance-testkit/tests/mutation_coverage/correct.rs:530-536`),
and a store outside the `Defect` family weakens the one-defect-per-store discipline the whole registry rests
on. What must be identical is the **arithmetic**: positions re-derived from the survivors rather than from the
last position assigned, with `memory.rs:277-282`, `:386-389` cited in the row's `provenance` as the real code
that does exactly this.

**Decision — the mutant fails exactly this rule, and the mechanism is that no other rule calls the seam.**
Because removal is reachable only through the new capability, and only this rule requires it, a store whose
defect lives in the removal step cannot fail anything else. That is not luck; it is the reason to put the
defect there rather than in `sequence` or `select`, and it is what makes
`mutants_fail_exactly_their_declared_rules` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2889`)
green in both directions rather than by a narrow escape.

**A new capability is never one edit — three meta-tests will find out.** This is the part that has no
mechanical backstop and says so in its own comment:

- `harness.rs`'s `declines()` writes the capability names out by hand, *"the one list in the binary with no
  mechanical backstop"*, and a capability with no line there makes every other mutant's skip **unaccountable**
  — `assert_undeclared_outcome` fails on a skip citing a capability the fixture's decline list does not
  contain (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:523-570`,
  `mutation_coverage.rs:2907-2925`).
- `MUST_SKIP` must gain this rule's name, or CF-18's hole reopens: a rule whose `require!` gate is later
  deleted passes `capability_skips_are_reported` in silence
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:3144-3165`).
- `GappedPositionFixture` must **support** removal honestly, because the same test asserts that a fixture
  which supports every capability skips nothing (`:3306-3314`). This is a gift rather than a tax: it is CF-6's
  behavioural enforcement, so the rule is exercised against a store whose positions are **not** dense from 1
  and a literal-position assertion cannot survive it.

**No literal position values — and this is the story where that bites hardest.** CF-6 is `[FROZEN]`: every
position assertion must be anchored on a value the store under test assigned
(`spec/SPECIFICATION.md:7233-7245`), and the specification permits gaps, so `assert_eq!(…, [1, 2, 3])` would
convert a `MAY` into a `MUST` with no ADR behind it. Two concrete bans follow, both carried from the discover's
gate note (`discover.md:72`): the rule captures the positions the store actually assigned, before and after
removal, and asserts non-reuse and strict monotonicity **between those captured values**; and it never names
the instrument's first retained position, which is arbitrary — chosen by the retained predicate, different in
the suffix and scattered configurations, and meaningless against any other store.

**A rule that no adapter can fail is decorative, and here the machinery enforces it.** `every_rule_has_a_mutant`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:2734`) fails the moment this rule is registered and
before its mutant exists — the rule and its wrong implementation are one commit by construction, not by
discipline. CF-29 adds the third limb: a rule *"MUST land in the same release as its mutant and its changelog
entry naming the defect it detects"* (`spec/SPECIFICATION.md:8140-8142`), enforced by a `cargo xtask ci` lint
that wants a real sentence, not a listing (`xtask/src/lints.rs:525-580`, `MIN_CHARS_PER_RULE` at `:446`).

**No published-surface change, and the version consequence is stated rather than discovered.** Everything
added here is additive: a new `pub` rule function reachable through `pub use suite::rules`
(`crates/happenstance-testkit/src/lib.rs:189`), one registry line, and two **defaulted** `Fixture` items.
`happenstance-core` is untouched. And the second sentence AC-011 is owed:
`crates/happenstance-testkit/Cargo.toml:4-14` states in the crate's own words that adding a conformance rule
is semver-MINOR and *"can turn a passing adapter's CI red"* — which is the mechanism `CLAUDE.md`'s rule that
matters exists to produce, not a violation. Both sentences belong in
`surface-diff-and-the-ac-012-escalation`'s recorded finding; this story's job is to keep the addition in the
additive shape and to hand that story an accurate enumeration. **No version number moves in this PR.**

**The persona-journey slice.** The reader is the adapter author whose store prunes — a device saving space, a
regulated purge — and who has until now had nothing in the suite that could tell them their compaction
invalidated every checkpoint and every replicated `after` naming their store. After this PR the suite says it
by name: `positions_are_not_reused_after_removal`, red, with the fixture's own words when the fixture declines
and the rule's own assertion when it runs. That named-rule failure is `RUNBOOK.md:4626-4674`'s bar — *a rule
name rather than "conformance failed"* — and this story owns one of AC-003's two names.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through every layer the testkit has: a rule, its
  registration, the fixture seam it needs, its wrong implementation, and the meta-tests that make the claim
  hold in both directions.
- **Slice / milestone**: `owed-rules-and-mutants`. Slice-mate, implemented in the same context and mounted as
  one integrated surface: `condition-over-removed-history-does-not-reject`
  (`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/condition-over-removed-history-does-not-reject/`),
  which lands ES-40's rule and its mutant. This story merges first (`_storymap.md:133-136`) because its mutant
  is the by-product of DA-2's rejected construction and is already in hand.
- **Mount point**: **`crates/happenstance-testkit/src/registry.rs`** — the rule name is added inside
  `for_each_event_store_rule!` (`:94`), in the *Sequence positions* group beside `positions_are_unique` and
  `positions_are_strictly_monotonic` (`:139-141`). This is composition root 2 of the architecture brief and
  **the only mount a rule has**: one line there makes the rule run in `memory_conformance.rs`,
  `local_conformance.rs`, `memory_conformance_blocking.rs`, `memory_conformance_wasm.rs`,
  `fixture_instruments.rs` and the instrument's own target at once through the four emitters, and — through
  `for_each_mutant!` — against every registered mutant. A rule body in `suite.rs` that is absent from this
  macro is invisible to all of them, and `no_orphan_rules` (`:412`) is what catches it.
- **Wires into**:
  - `happenstance_testkit::Fixture` / `Capability` (`crates/happenstance-testkit/src/contract.rs:120-321`) —
    the seam lands here, defaulted, beside `MID_BATCH_FAULT` (`:200-211`) and `arm_mid_batch_fault`
    (`:290-307`); `Capability::declined` and the trade-versus-fact line at `:44-54`.
  - `suite.rs`'s `require!` macro (`crates/happenstance-testkit/src/suite.rs:37-46`) and its `RuleOutcome`
    contract — a declined capability returns `RuleOutcome::Skipped { capability, reason }` and the rule is
    still emitted as a test.
  - `suite.rs`'s existing helpers — `append_ok`, `read_ok`, `event`, `positions_of`, `collect`
    (`crates/happenstance-testkit/src/suite.rs:105-140`) — so the rule reads like its neighbours and its
    assertions are raised in `suite.rs`, which is what `FailureMode::Assertion`'s **positive** origin check
    depends on (`crates/happenstance-testkit/tests/mutation_coverage.rs:2965-2975`).
  - `crates/happenstance-testkit/tests/mutation_coverage.rs` — `REGISTRY` (`:324`), `for_each_mutant!`
    (`:2056`), `MUST_SKIP` (`:3157`), and the three meta-tests at `:2734`, `:2754`, `:2889`.
  - `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` — the `Defect` trait (`:110-200`) and
    `MutantFixture<D>` (`:353-390`); `.../correct.rs` — `Log` (`:450-480`), the home of the honest removal
    step; `.../variants.rs` — `GappedPositionFixture` (`:229-300`) and `DecliningFixture` (`:487-560`);
    `.../harness.rs` — `Subject` (`:60-70`) and `declines()` (`:523-570`).
  - `crates/happenstance-testkit/tests/completeness_instrument.rs` — HS-S0114's instrument target and its
    `ForgettingFixture`, which declares the capability and implements removal by narrowing its retained set.
    **If that target is absent when this story starts, halt and report a dependency failure**; do not write a
    bespoke stand-in, which would demonstrate the rule against a store nobody built.
  - `CHANGELOG.md` `## [Unreleased] / ### Added` (`:25-27`) — CF-29's entry.
- **Renders surfaces**: **none.**
  `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md:38-48` records
  `## Surfaces` and `## Items` as *"N/A — no user-facing surface"*, approved at the design sign-off gate on
  2026-08-12 (`:86-95`). There is no surface id to claim and no `## Signatures` block to match. The design's
  binding content for this story is its API-surface note (`:32-36`): a defaulted `Fixture` capability and/or
  method plus `pub fn` rule bodies are testkit-internal conformance surface, reviewed under AC-011.
- **Public items**: one new `pub async fn` in `suite::rules`, and two new **defaulted** associated items on
  `happenstance_testkit::Fixture`. Both are additive; neither is required. Enumerated for
  `surface-diff-and-the-ac-012-escalation`, which proves the claim by diff.
- **Conformance rule(s)**: **`positions_are_not_reused_after_removal`** — written, registered exactly once,
  and observed by `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive`,
  `mutants_fail_exactly_their_declared_rules` and `capability_skips_are_reported`. It is the rule this story
  exists for; there is no second one.
- **Clause(s)**: **ES-38** (`spec/SPECIFICATION.md:4299-4323`, `[FROZEN]`) — its `Rule:` obligation is
  **discharged**, and the clause text is not edited. The `(new)` annotation comes off in
  `marker-moves-and-spec-trace-green`; **`spec/SPECIFICATION.md` is byte-identical after this PR**, because a
  marker moved in the same commit as the rule is a `spec-trace` shape this repository moves deliberately and
  in one place. **CF-1**, **CF-2**, **CF-3**, **CF-6**, **CF-18**, **CF-29** and **CF-33** are all *observed*
  by this diff and none is amended. No new ADR is owed: implementing a `[FROZEN]` clause's own named rule is
  what the clause asks for (ADR-0013's *"What this ADR leaves open"*,
  [`.kb/decisions/0013-position-assignment-and-visibility.md`](.kb/decisions/0013-position-assignment-and-visibility.md)).
- **Advances DoD scenario**: initiative DoD **15** — *"Incomplete logs have an answer on disk … a store that
  holds only a suffix of its own log is exercised against a reader"*
  (`.bklg/from-contract-to-published-library/initiative.md:402-404`). This story lands the half that makes the
  exercise **detect** something: the first of the two named failures project AC-003 requires.

## PR boundary

```
crates/happenstance-testkit/src/suite.rs
crates/happenstance-testkit/src/registry.rs
crates/happenstance-testkit/src/contract.rs
crates/happenstance-testkit/tests/mutation_coverage.rs
crates/happenstance-testkit/tests/mutation_coverage/**
crates/happenstance-testkit/tests/completeness_instrument.rs
CHANGELOG.md
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/positions-are-not-reused-after-removal/**
```

**In this PR**

- The rule body in `suite.rs`, its one line in `for_each_event_store_rule!`, and the defaulted capability
  const plus defaulted panicking method in `contract.rs`.
- `CompactingRestoreStore` as a `Defect` in `mutation_coverage/mutants.rs`, its line in `for_each_mutant!`,
  and its `REGISTRY` row with a non-empty `fails`, a real `provenance`, a `FailureMode` and a per-rule `expect`
  pin.
- The honest removal step (`correct.rs`), honest support on `GappedPositionFixture` (`variants.rs`), the
  fourth line in `harness.rs`'s `declines()`, and the new row in `MUST_SKIP`.
- The capability declaration and removal implementation on HS-S0114's `ForgettingFixture`, inside
  `tests/completeness_instrument.rs` — removal by narrowing the retained set, never by rebuilding the inner
  store.
- One `CHANGELOG.md` entry under `## [Unreleased] / ### Added`, naming the **defect** this rule detects in a
  sentence, not a listing.
- This story's own backlog folder — this spec, its `_ledger.md`, its implementation report.

**Explicitly not in this PR**

- **No `spec/SPECIFICATION.md` edit of any kind** — no marker move, no `(new)` removal, no clause text. That
  is `marker-moves-and-spec-trace-green`'s, and it raises the `(new)` question rather than inheriting an
  assumption.
- **No `happenstance-core` change**, of signature or of rustdoc. ES-40's documentation sentence is the
  slice-mate's.
- **No second rule.** CF-39's precedent — a companion rule proving a declared capability actually fires — is
  weighed and declined below; the non-vacuity check lives inside this rule.
- **No `src/fixtures.rs` change.** `MemoryFixture` inherits the default decline, for the reason recorded above.
- **No required `Fixture` item**, no change to `SECOND_HANDLE` or `REOPEN`, and no third mandatory const.
- **No version bump** to `happenstance-testkit` and no `Cargo.toml` edit.
- **No ADR, no `.kb/` file, no `.kb/_intake/` staging**, and no edit to the recorded pass list — HS-S0115's
  artefact is **regenerated by its committed command** if this rule changes its denominator, never hand-edited.
- **No `experiments/` artefact.** This story produces a rule, not a measurement.

**Merge DoD (one line)** — `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green with the new
rule emitted in every conformance target, `cargo test -p happenstance-testkit --test mutation_coverage` green
including all four meta-tests, and `git diff --stat` touching nothing outside the globs above.

## Behavior and interfaces

Names are proposals except where marked **binding**; the *shape* is binding everywhere. Where a name is a
proposal, keeping the vocabulary of the brief it came from is the cheapest way for the next reader to match
the two documents.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The seam: a defaulted capability on `Fixture`** | A `Capability` associated const (proposed `REMOVAL`) whose **default is `Capability::declined(<reason>)`**. **Binding: defaulted, never required** — a third required const breaks every `Fixture` impl in and out of the workspace. The default reason is written to be true of the case that matters and names the hazard: a store whose only way to shrink is to rebuild, and a rebuild that re-derives positions is the defect this rule exists to catch. | `crates/happenstance-testkit/src/contract.rs:200-211` (`MID_BATCH_FAULT`'s default), `:44-54` (trade versus fact), `:161`/`:173` (the two required consts, untouched); `_decomposition.md:276-308` |
| **The seam: a defaulted method that panics when unimplemented** | Proposed `fn remove_events(&self, positions: &[SequencePosition]) -> impl Future<Output = ()>`, defaulted, body panics with a message naming **both** ways of reaching it — a fixture that declares the capability and forgets the override, or a rule that arrived without a `require!` gate. **Binding: it takes a set of positions, not a floor** — a `remove_below(p)` seam is `earliest_position()`'s shape and would pre-decide ES-39 inside the testkit. `contract.rs` will need `use happenstance_core::SequencePosition;`, which it does not import today. | `crates/happenstance-testkit/src/contract.rs:290-307` (`arm_mid_batch_fault`'s provided body), `:67` (its imports); `spec/SPECIFICATION.md:4336-4341` (why a floor is the wrong shape); `_decomposition.md:299-305` |
| **The rule: gated, emitted, never compiled out** | `pub async fn positions_are_not_reused_after_removal<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome`, opening with `require!(F: REMOVAL)`. `require!` and not `must!`: removal is a trade a fixture may honestly decline, not a MUST. A declining fixture reports `RuleOutcome::Skipped` **with its own stated reason** and the test still exists in the binary. | `crates/happenstance-testkit/src/suite.rs:37-46`, `:2706-2712` (the shape to copy); `crates/happenstance-testkit/src/contract.rs:31-42` (why `#[cfg]` is rejected by name) |
| **The rule: the arrangement** | Append events in more than one batch so that more than one position is assigned; capture every position **from what the store returned and what a read of it reports**, never from a literal. Remove a **scattered** subset — at least two non-adjacent positions, survivors both below and above the hole — that **excludes the highest assigned position**. Then append again and capture the new position. | `spec/SPECIFICATION.md:4299-4307`; DA-1 (`_decomposition.md:150-171`); `crates/happenstance-testkit/tests/mutation_coverage/correct.rs:456-480` (why the head is left in place) |
| **The rule: the assertions** | Four, in this order. (1) **Non-vacuity** — the removed positions are no longer readable, so a fixture that declares the capability and quietly no-ops cannot pass. (2) **Non-reuse** — the position assigned after removal is not equal to any position captured before it, *including the removed ones*. (3) **Strict monotonicity** — it is strictly greater than the maximum of every position ever assigned. (4) **The remaining view** — the positions `Query::all()` reports are unique, strictly increasing, and a subset of what the store assigned plus the new one. **Binding: every comparison is between captured values**; no integer list literal and no `SequencePosition` built from a literal appears anywhere in the rule. | `spec/SPECIFICATION.md:4299-4307`, CF-6 at `:7233-7245`; `xtask/src/lints.rs:628` (`no_position_literals`, the cheap second line); `discover.md:72` |
| **No clock, and no second list of rule names** | The rule reads no time (CF-33, `xtask/src/lints.rs:231`), and nothing in this PR writes a rule name anywhere except `for_each_event_store_rule!`, `REGISTRY`'s `fails`/`expect` and `MUST_SKIP` — each of which is checked against `__emit_rule_names` by a meta-test. | `crates/happenstance-testkit/src/registry.rs:293`, `:412`; `crates/happenstance-testkit/tests/mutation_coverage.rs:2810-2822` |
| **Green where it runs: the honest instrument** | `ForgettingFixture` declares the capability and implements removal by **narrowing its retained set** — hiding at the port, inner `MemoryEventStore` untouched, `restore` called nowhere. Positions therefore continue from the inner store and the rule passes in **both** DA-1 configurations. This is DA-2(b) doing exactly what it was built for, and it is the symmetry the story rests on: the honest store hides, the mutant rebuilds. | `_decomposition.md:173-232`; `.../retained-set-instrument-and-conformance-mount/spec.md` (the `RetainedSet` type and the mount) |
| **Green where it runs: the gapped conformant variant** | `GappedPositionFixture` **must** support the capability honestly — `capability_skips_are_reported` asserts the fully-capable fixture skips nothing. Honest removal drops the rows from its `Log` and leaves the allocator alone. This is CF-6's behavioural enforcement: the rule is exercised against a store whose positions are neither dense nor starting at 1, so a literal-position assertion cannot survive it. Trap: its durable `committed` record is a second copy — decide deliberately whether removal touches it and say so in a comment, because a resurrecting reopen is a decorator bug that would read as a forgetting signal. | `crates/happenstance-testkit/tests/mutation_coverage.rs:3306-3314`; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116-121`, `:229-260`; `spec/SPECIFICATION.md:7233-7238` |
| **Honest skip everywhere else, and accounted for** | `MemoryFixture`, `DurableFixture`, `DecliningFixture` and every `MutantFixture<D>` decline by default and report a skip. **Binding: a fourth `declined("REMOVAL", S::REMOVAL)` line in `harness.rs`'s `declines()`** — without it every mutant's skip of this rule is a skip *"nothing in the registry accounts for"* and `mutants_fail_exactly_their_declared_rules` fails across the whole table. **Binding: the rule's name is added to `MUST_SKIP`**, or a future deletion of its `require!` gate passes `capability_skips_are_reported` in silence. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:523-570`; `crates/happenstance-testkit/tests/mutation_coverage.rs:2907-2925`, `:3144-3165`, `:3288-3296` |
| **The mutant: `CompactingRestoreStore`** | A `Defect` whose **single** overridden step is removal: it retains the survivors and then re-derives their positions from the survivors themselves, so the next append hands out a position the store has already assigned and already acknowledged. **Binding: one overridden method.** Its `provenance` names the real code that does this — `MemoryEventStore::restore` over a truncated snapshot, `position_at(first_index + offset)` with `first_index = stored.len()` — and the real adapter shape: a device pruning to save space, which silently invalidates every checkpoint and every replicated `after` naming that store. It declares the capability supported; nothing else in `for_each_mutant!` does. | `crates/happenstance-core/src/memory.rs:163`, `:277-282`, `:386-389`; `spec/SPECIFICATION.md:4321-4323`; `_decomposition.md:173-194`, `:605`; `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:110-200` |
| **The mutant: its registry row** | `Declared { name: "CompactingRestoreStore", kind: Kind::Mutant, fails: &["positions_are_not_reused_after_removal"], provenance: <never empty>, mode: FailureMode::Assertion, expect: &[("positions_are_not_reused_after_removal", <substring of the non-reuse assertion>)] }`, plus its type in `for_each_mutant!`. `fails` is non-empty or `mutant_registry_is_exhaustive` rejects the row; the `expect` pin names which of the rule's four assertions did the rejecting, so a later edit cannot keep the meta-test green while the mutant stops demonstrating what it claims. | `crates/happenstance-testkit/tests/mutation_coverage.rs:141-186` (the `Declared` fields and why `expect` is keyed by rule), `:324`, `:2056`, `:2834-2841` |
| **Exactly, in both directions** | The mutant fails this rule and **no other**, and the mechanism is structural: removal is reachable only through the new capability and only this rule requires it, so a defect in the removal step cannot leak into another rule. Every other registered store passes or skips-with-an-accounted-reason. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2889-2935`; `project.md:205-209` |
| **Non-vacuity, and why there is no companion rule** | CF-39's precedent — `arming_a_mid_batch_fault_makes_the_append_fail`, a second rule proving a declared capability actually fires — is weighed and **declined**: a mid-batch fault is unobservable except through its effect on the append, whereas removal is directly observable by a read, so the check folds into assertion (1) above. Landing a second rule would also require a second mutant under CF-1 and a second changelog entry, for a claim this rule already makes. Recorded here so the omission reads as a decision. | `crates/happenstance-testkit/src/suite.rs:2794-2800`; `crates/happenstance-testkit/tests/mutation_coverage.rs:2734-2748` (CF-1) |
| **The changelog entry (CF-29)** | One entry under `## [Unreleased] / ### Added` naming the rule **and the defect it detects** in prose — a store that renumbers when it compacts, and what that costs a reader holding a checkpoint or a replicated `after`. The lint divides an entry's length by the number of rules it names and wants at least 120 characters per rule, so a bullet that lists this rule beside others fails even though the name is present. | `spec/SPECIFICATION.md:8140-8168`; `xtask/src/lints.rs:446`, `:525-580`; `CHANGELOG.md:25-27` |
| **Additive, and enumerated for the story that proves it** | One `pub` rule function (reachable via `pub use suite::rules`), one defaulted `Capability` const, one defaulted method. No required item, no signature change in `happenstance-core`, no version bump. The enumeration is handed to `surface-diff-and-the-ac-012-escalation` together with the second sentence AC-011 is owed: additive under semver **and** a red build for any adapter that renumbers. | `crates/happenstance-testkit/src/lib.rs:189`; `crates/happenstance-testkit/Cargo.toml:4-14`; `project.md:236-239`; `_decomposition.md:612` |

## Data and migrations

**N/A.** No schema, no persisted format, no migration, and no committed artefact. Every store this story
touches is an in-process `Vec` behind an `Rc` or an `Arc` that exists for the lifetime of one test, and
"removal" here is a test-time operation on such a store, not a storage operation on a medium — the port has
no delete method and ES-37 (`spec/SPECIFICATION.md:4275-4277`, `[FROZEN]`) forbids growing one.

Two adjacent things that are deliberately **not** data changes, stated so they are not mistaken for one. The
wire format of [ADR-0016](.kb/decisions/0016-the-wire-format.md) is untouched: no `serde` derive, no envelope
type, no feature. And HS-S0115's recorded pass list under `experiments/completeness-pass-list/` is a committed
artefact whose **denominator this story changes** by adding a rule — it is refreshed by re-running that
story's committed command, never by hand-editing the results, so that a later reader can still tell a stale
list from a wrong one.

## Acceptance criteria

Eight criteria, and every one is framed from the intent of a person rather than from the existence of a
symbol. Two people are in scope. The **adapter author** whose store prunes is the initiative's
*Learn when you are finished* journey — *"from a signature that type-checks to a suite that says pass or fail
and names why"* (`.bklg/from-contract-to-published-library/initiative.md:245-247`) — and today the suite says
nothing at all to them. The **repository owner** is discharging phase 14's owed rule and needs the failure to
arrive as a rule name, not as "conformance failed" (`RUNBOOK.md:4667`, `project.md:202-204`).

"The function compiles" is a criterion nowhere below. `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:412`)
and `every_rule_has_a_mutant` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2734`) both exist
because a rule body that compiles and is never emitted, or is emitted and can never fail, is the named wrong
implementation this project's testing brief writes into the AC-004 row (`_decomposition.md:605`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The pruning adapter author is told which rule they broke, by name.** GIVEN an adapter author whose store compacts — a device saving space, a regulated purge — runs `happenstance_testkit::event_store_conformance!` against their own fixture, WHEN that store re-derives positions from the events it kept, THEN the run fails a test named `positions_are_not_reused_after_removal` rather than reporting "conformance failed", because the rule body exists in `crates/happenstance-testkit/src/suite.rs` and its name appears **exactly once** inside `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`), in the *Sequence positions* group beside `positions_are_unique` and `positions_are_strictly_monotonic` (`:139-141`), so all four emitters and `for_each_mutant!` carry it without a second registration anywhere. | `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:412`) green — the rule is declared **and** registered; `cargo test -p happenstance-testkit --test mutation_coverage` shows `CompactingRestoreStore`'s failure reported under this rule's module name; the rule's generated module is present in `memory_conformance`, `local_conformance`, `memory_conformance_blocking`, `memory_conformance_wasm`, `fixture_instruments` and the instrument's own target (a `--list` of each target's test names) |
| AC-002 | **The seam arrives without breaking one existing `Fixture` impl.** GIVEN an adapter author who already implements `Fixture` in their own crate against `happenstance-testkit` `0.2.x`, WHEN they take the version carrying this rule, THEN their impl compiles **untouched** and their store is simply reported as declining removal — because the seam is a **defaulted** `Capability` associated const (default `Capability::declined(<reason>)`) plus a **defaulted** method taking a **set** of positions (`&[SequencePosition]`), in the exact shape `MID_BATCH_FAULT` set (`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`); no required item is added, `SECOND_HANDLE` (`:161`) and `REOPEN` (`:173`) are unchanged, and no floor-shaped `remove_below` seam exists to pre-decide ES-39 inside the testkit. | `cargo test -p happenstance-testkit --all-targets` green with `MemoryFixture`, `DurableFixture`, `DecliningFixture`, `GappedPositionFixture`'s peers and every `MutantFixture<D>` **unedited for the const**; `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`) reporting the const and the method as additive with defaults; review of the `contract.rs` diff for the literal token `= Capability::declined` on the const and a provided body on the method |
| AC-003 | **The rule catches renumbering using only values the store itself said.** GIVEN the rule is run against a store whose positions are neither dense nor starting at 1, WHEN it appends across more than one batch, removes a **scattered** subset that excludes the highest assigned position, and appends once more, THEN it passes or fails purely on captured values — (1) the removed positions are no longer readable, (2) the newly assigned position equals **no** position captured earlier including the removed ones, (3) it is strictly greater than the maximum of every position ever assigned, and (4) the positions `Query::all()` reports are unique, strictly increasing and a subset of what the store assigned plus the new one — and **no integer list literal and no `SequencePosition` built from a literal appears anywhere in the rule body**, in particular not the instrument's arbitrary first retained position. | The rule green against `GappedPositionFixture`, whose positions are deliberately gapped (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:229-260`) — a literal-anchored assertion cannot survive it; `cargo xtask lints`' `no_position_literals` (`xtask/src/lints.rs:628`) as the cheap second line; review of the rule body against CF-6 (`spec/SPECIFICATION.md:7233-7245`) |
| AC-004 | **The honest forgetting store passes, in both retained configurations.** GIVEN the repository owner runs the suite against HS-S0114's instrument in the **suffix** and the **scattered** configurations (DA-1), WHEN this rule runs — the instrument's `ForgettingFixture` declaring the capability and removing by **narrowing its retained set**, inner `MemoryEventStore` untouched and `restore` called nowhere — THEN it is green in both, so any later red under this rule name is attributable to a store that *renumbers*, never to a store that merely *forgets*. | `cargo test -p happenstance-testkit --test completeness_instrument` green in both configurations with this rule's module present and passing in each; the target contains no call to `MemoryEventStore::restore` (`crates/happenstance-core/src/memory.rs:163`) |
| AC-005 | **The reference fixture says *why* it cannot help, and the silence is accounted for.** GIVEN a reader of the conformance output for `MemoryFixture` (and for `DurableFixture`, `DecliningFixture` and every `MutantFixture<D>`), WHEN this rule reaches them, THEN a test carrying the rule's name **still exists in the binary** and reports `RuleOutcome::Skipped { capability, reason }` with the fixture's own stated reason — never `#[cfg]`-ed out, the arrangement `crates/happenstance-testkit/src/contract.rs:31-42` rejects by name — and the skip is *accountable*: `harness.rs`'s hand-written `declines()` gains its line and `MUST_SKIP` gains this rule's name, so no other mutant's outcome becomes unexplainable and a future deletion of the `require!` gate cannot pass in silence. | `capability_skips_are_reported` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`) green, including its assertion that the fully-capable fixture skips **nothing** (`:3306-3314`) and its `MUST_SKIP` sweep (`:3288-3296`); `mutants_fail_exactly_their_declared_rules` (`:2889`) green across the whole table, which is what fails if the `declines()` line is missing (`:2907-2925`); the rule's name present in `MUST_SKIP` (`:3157`) |
| AC-006 | **The wrong implementation the specification names is in the tree, and it fails exactly this rule.** GIVEN ES-38's `Rejects:` names *"an adapter that renumbers on compaction"* (`spec/SPECIFICATION.md:4321-4323`), WHEN `cargo test -p happenstance-testkit --test mutation_coverage` runs, THEN `CompactingRestoreStore` exists as a `Defect` whose **single** overridden step re-derives the survivors' positions from the survivors, is listed in `for_each_mutant!` and carries a `REGISTRY` row with a non-empty `fails: &["positions_are_not_reused_after_removal"]`, a real `provenance` citing `crates/happenstance-core/src/memory.rs:277-282`, `:386-389`, `FailureMode::Assertion`, and a per-rule `expect` substring pinning **which** of the rule's four assertions did the rejecting — and it fails this rule and **no other**. | `every_rule_has_a_mutant` (`:2734`), `mutant_registry_is_exhaustive` (`:2754`, which rejects an empty `fails`) and `mutants_fail_exactly_their_declared_rules` (`:2889`) all green — the last **in both directions**: the mutant fails what it declares and nothing it does not; the `FailureMode::Assertion` origin check (`:2965-2975`) confirming the panic is raised in `suite.rs` |
| AC-007 | **The release tells an adapter author what just turned their CI red, in a sentence.** GIVEN an adapter author who took a minor bump of `happenstance-testkit` and found a new red test, WHEN they open `CHANGELOG.md` at `## [Unreleased] / ### Added`, THEN they find an entry naming this rule **and the defect it detects** in prose — a store that renumbers when it compacts, and what that costs a reader holding a checkpoint or a replicated `after` — rather than the rule's name listed beside others in a bullet. | `cargo xtask lints` — CF-29's changelog check (`xtask/src/lints.rs:525-580`), which divides the entry's length by the number of rules named and requires at least `MIN_CHARS_PER_RULE` (`:446`); the entry present under `CHANGELOG.md:25-27`'s heading |
| AC-008 | **Nothing published breaks, and the story that proves it inherits an accurate list.** GIVEN `surface-diff-and-the-ac-012-escalation` must report this initiative's surface delta against the `0.2.0` baseline, WHEN it diffs this PR, THEN it finds exactly **three** additive public items — one `pub async fn` in `suite::rules` (reachable via `crates/happenstance-testkit/src/lib.rs:189`), one defaulted `Capability` const and one defaulted method on `Fixture` — with `happenstance-core` untouched, `spec/SPECIFICATION.md` **byte-identical**, no `Cargo.toml` edit and no version bump; and this story's implementation report hands over that enumeration **together with** the second sentence AC-011 is owed: additive under semver *and* capable of turning a passing adapter's CI red (`crates/happenstance-testkit/Cargo.toml:4-14`). | `git diff --stat` against the PR-boundary globs only; `git diff -- spec/SPECIFICATION.md crates/happenstance-core crates/happenstance-testkit/Cargo.toml` **empty**; `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`) reporting no breaking change; content review of the implementation report for the three-item enumeration and both sentences |

## Interaction quality

This story renders **no surface**, and that is a signed-off determination rather than an omission:
`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md:38-80` records
`## Surfaces`, `## Items`, `## Signatures` and `## Anti-patterns` as *"N/A — no user-facing surface"*,
approved by the repository owner on 2026-08-12 at the `/redkiln:plan` design sign-off gate (`:86-95`), with
`design.capture` a **declared** skip per `CLAUDE.md`.

Per the rule that an invariant which is not a table row is never gated, **every invariant that applies is
already an `AC-###` row above.** This section only says which row carries it and how it is verified; it
introduces no new obligation, and the implementer may not add one here.

**COMPOSITION family — N/A, by signed-off design.** There is no composition, placement, transience,
density budget, hierarchy or named visual anti-pattern to honour, because there is no rendered control:
`_design.md:26-30` records that the readers in this project are `read_decision_model`, a projection runner
and `IngestStore::holds` — Rust code paths, not a screen — and that *"no route, no DOM selector, no
component exists to enumerate."* The design's only binding content for this story is its **API-surface
note** (`:32-36`), which scopes a defaulted `Fixture` capability and `pub fn` rule bodies as testkit-internal
conformance surface reviewed under AC-011 — carried by **AC-002** and **AC-008** above.

**STATE family — applies in its library register, and each applicable invariant is an AC row.** The
"surface" this story does have is the one an adapter author actually meets: the conformance run's output,
and the rustdoc on the three new items.

| State invariant | The register it takes here | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Reachability** (the keyboard-reachability analogue) | The rule is reachable along the one path every other rule is reached along — a single line in `for_each_event_store_rule!` — never by a bespoke driver, and never registered twice | **AC-001** | `no_orphan_rules` (`registry.rs:412`); the rule's module present in every emitted target |
| **Non-occlusion / legibility of failure** | A failure names the **rule** and, through the `expect` pin, **which assertion** rejected — never "conformance failed" and never a bare panic with no origin | **AC-001**, **AC-006** | `RUNBOOK.md:4667`; the per-rule `expect` substring; the `FailureMode::Assertion` origin check (`mutation_coverage.rs:2965-2975`) |
| **Honest declaration of what is unavailable** (the disabled-state analogue) | A fixture that cannot be made to forget reports a **stated reason** and still emits its test, rather than vanishing from the binary | **AC-005** | `capability_skips_are_reported` (`:3184`); `contract.rs:31-42`'s rejection of `#[cfg]`-ing rules out |
| **Non-vacuity** (the "the control does something" invariant) | A fixture that declares the capability and quietly no-ops must not pass: assertion (1) reads the store back and requires the removed positions to be gone | **AC-003** | The rule's first assertion, exercised against `GappedPositionFixture` and the instrument |
| **Preserved identity through the change** (the selection/scroll analogue) | Positions already handed out survive the removal intact — the store never reissues one, and its remaining view stays unique and strictly increasing | **AC-003**, **AC-004** | Assertions (2)–(4); green against the instrument in both DA-1 configurations |
| **In-place, not context-jump** | Honest forgetting is applied **at the port**; the inner store is not swapped, rebuilt or re-derived, so nothing the store already told a caller changes underneath it. The rebuild is the *mutant*, deliberately | **AC-004**, **AC-006** | `MemoryEventStore::restore` absent from the instrument target; present, by design, only in the mutant's one overridden step |
| **Reversibility / no silent scope drift** | The obligation discharged is ES-38's own, unedited: the clause text is implemented and `spec/SPECIFICATION.md` is byte-identical, so the marker move stays a separate, reversible decision | **AC-008** | `git diff -- spec/SPECIFICATION.md` empty; NF-001 |
| **Presentation exists at all**, in its only medium | The rustdoc on the rule and on the two `Fixture` items is the presentation: what the rule asserts, what a declining fixture is saying, and the residual case (removal of the store's own head) handed to ADR-0028 rather than left to be discovered | **AC-002**, **AC-008** (and NF-008) | Content review against the three named sentences; `cargo doc` in `cargo xtask ci --fast` |
| **Backward compatibility of the reader's own work** | An existing `Fixture` impl keeps working with no edit — the library equivalent of not resetting a user's state on upgrade | **AC-002** | `cargo-semver-checks`; every existing fixture unedited for the const |

## Error conditions

| id | condition | required behaviour | evidence |
| --- | --- | --- | --- |
| EC-001 | A fixture declares the removal capability as supported but does not override the defaulted method | The provided body **panics**, with a message naming **both** ways of arriving here: a fixture that declared the capability and forgot the override, and a rule that arrived without a `require!` gate. This is `arm_mid_batch_fault`'s own arrangement, copied rather than reinvented | `crates/happenstance-testkit/src/contract.rs:290-307` |
| EC-002 | The fixture under test declines the removal capability | `require!(F: …)` returns `RuleOutcome::Skipped { capability, reason }` carrying the fixture's stated reason. This is neither a pass nor a failure, and the test is still emitted | `crates/happenstance-testkit/src/suite.rs:37-46`; AC-005 |
| EC-003 | A fixture declares the capability and then silently no-ops on removal | The rule **fails**, at assertion (1): the removed positions must no longer be readable before non-reuse is even considered. A capability that can be declared and ignored is a decorative rule wearing a gate | AC-003; `crates/happenstance-testkit/src/suite.rs` (the rule body) |
| EC-004 | Removal is asked for a position the store does not hold, or holds no longer | Defined and documented on the seam as a **no-op for that position**, never a panic and never an error — the seam's contract is "make this store stop holding these", which is already true of a position it does not hold. The rule itself never asks for a position it did not capture, so this path exists for future rules and honest fixtures, not for this one | The seam's rustdoc (NF-008); `crates/happenstance-testkit/src/contract.rs:290-307` (the precedent's documented contract) |
| EC-005 | The mutant fails a rule it did not declare | Fix the **mutant**, not the registry row: narrow the defect back to one overridden step, per the one-defect-per-store discipline. Widening `fails` to silence the meta-test is permitted by the machinery and forbidden here, because it destroys AC-004's load-bearing word *exactly* | `mutants_fail_exactly_their_declared_rules` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2889-2935`); `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:110-200`; `project.md:205-209` |
| EC-006 | The new capability is added without its line in `harness.rs`'s `declines()` | The whole mutant table goes red, not one row: every mutant's skip of this rule becomes an outcome *"nothing in the registry accounts for"*. The symptom is diffuse and the cause is one missing line — diagnose here first when many rows fail at once | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:523-570`; `mutation_coverage.rs:2907-2925` |
| EC-007 | `GappedPositionFixture`'s durable `committed` record resurrects removed events on a later reopen | Decide deliberately whether honest removal touches that second copy and **say so in a comment**. A resurrecting reopen is a decorator bug that reads exactly like a forgetting signal, and it will be attributed to this rule rather than to the fixture | `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116-121`, `:229-260` |
| EC-008 | The changelog entry names the rule but is too short, or lists it beside other rules | `cargo xtask lints` fails: the CF-29 check divides entry length by rules named and requires `MIN_CHARS_PER_RULE`. The fix is prose about the **defect**, not padding | `xtask/src/lints.rs:446`, `:525-580` |
| EC-009 | The instrument target `crates/happenstance-testkit/tests/completeness_instrument.rs` is absent when this story starts | **Halt and report a dependency failure.** Do not write a bespoke forgetting store to demonstrate the rule against — a rule proven against a store nobody built proves nothing, and this story's `depends_on` exists for exactly this | `_storymap.md:133-136`; the dependency's spec at `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/spec.md` |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| NF-001 | **`spec/SPECIFICATION.md` is byte-identical after this PR.** No marker move, no `(new)` removal, no clause text edit | A marker moved in the same commit as its rule is a `spec-trace` shape this repository moves deliberately and in one place — `marker-moves-and-spec-trace-green`'s (`xtask/src/spec_trace.rs`). Held by AC-008's `git diff` and by `cargo xtask spec-trace` running unconditionally in `cargo xtask affected` |
| NF-002 | **No new dependency, no `Cargo.toml` edit and no version bump.** `contract.rs` gains at most one `use happenstance_core::SequencePosition;` | The seam is expressed in types the testkit already depends on. A version bump here would pre-empt `surface-diff-and-the-ac-012-escalation`'s finding (`project.md:236-243`) |
| NF-003 | **The four mandatory `wasm32` steps stay green.** The defaulted method returns `impl Future<Output = ()>` with **no `Send` bound written at the definition**, and no runtime is introduced | `CLAUDE.md` constraint 1 and [ADR-0001](.kb/decisions/0001-async-port-flavours.md): a `+ Send` bound injected into a fixture seam would make the Cloudflare/`wasm32` flavour impossible. The rule is emitted into `memory_conformance_wasm` too, so this is not hypothetical. `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) |
| NF-004 | **No `#[async_trait]` anywhere, and exactly one of `EventStore` / `SendEventStore` imported per module** | `CLAUDE.md` constraints 1 and 4 — both names in scope makes method calls ambiguous, which surfaces as an inference error in the rule body rather than as a trait error |
| NF-005 | **`cargo clippy -D warnings` clean**, and **no `unsafe`** in the rule, the seam or the mutant | The gate runs clippy with `-D warnings`; any `#![allow]` is scoped and justified on one line, following `crates/happenstance-testkit/tests/fixture_instruments.rs:43-44` |
| NF-006 | **The rule reads no clock** | CF-33, enforced by a file-reading lint (`xtask/src/lints.rs:231`) that runs unconditionally in `cargo xtask affected` |
| NF-007 | **The rule is cheap enough to be a merge-gate step** — a bounded number of appends and one removal, no sleeps, no retries and no quadratic rescan | The whole event-store family runs against every registered fixture and every mutant on each `cargo xtask affected` touching the testkit; a slow rule multiplies across roughly forty stores |
| NF-008 | **Rustdoc on all three new public items**, carrying three load-bearing sentences: what the rule asserts and against which clause; what a fixture is saying when it declines removal; and the **residual** — removal of the store's own highest assigned position is *not* staged by this rule, and is handed to ADR-0028 rather than left to be discovered | `standards/rust/70-rustdoc-obligations.md`; the residual's reason is in the Context pack (every honest store in the workspace allocates from what it currently holds). `cargo doc` runs inside `cargo xtask ci --fast` |
| NF-009 | **No second list of rule names.** The rule's name appears only in `for_each_event_store_rule!`, `REGISTRY`'s `fails`/`expect` and `MUST_SKIP` — each checked against `__emit_rule_names` by a meta-test | `crates/happenstance-testkit/src/registry.rs:293`, `:412`; `crates/happenstance-testkit/tests/mutation_coverage.rs:2810-2822` |

## Implementation notes (non-prescriptive)

None of these binds *how*; each binds what must still be true afterwards.

- **Order the work so the machinery does the reminding.** Register the rule name **first**. `every_rule_has_a_mutant`
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:2734`) goes red immediately and stays red until the
  mutant exists — the rule and its wrong implementation become one commit by construction rather than by
  discipline. Let it be red for a while; that is the tool working.
- **Write the mutant before the rule body if you want a fast red.** The mutant's arithmetic is already known
  (`crates/happenstance-core/src/memory.rs:277-282`, `:386-389`), so the rule can be written against a store
  that is *known* to be wrong rather than against one that is hoped to be right.
- **Copy `MID_BATCH_FAULT` twice, not once.** The const at `crates/happenstance-testkit/src/contract.rs:207-211`
  and the provided method at `:290-307` are one pattern in two halves; taking only the const yields a
  capability nothing can act on, and taking only the method yields an ungated panic.
- **`contract.rs` does not import `SequencePosition` today.** Check `:67` before assuming the type is in
  scope — this is the one compile error this story is most likely to meet first.
- **Capture positions from two sources, not one.** Take what `append` returned *and* what a read reports,
  and reconcile them; a rule that only trusts the return value cannot notice a store that acknowledged one
  position and stored another.
- **Do not remove the store's head.** Every honest store in the workspace allocates the next position from
  what it currently holds (`crates/happenstance-testkit/tests/mutation_coverage/correct.rs:456-480`), so
  removing the head would oblige every fixture to grow a high-water mark for a case ES-38's `Rejects:` does
  not name. Removing a scattered subset **below** the head is both the device-prune shape and enough to catch
  the mutant.
- **Read `crates/happenstance-testkit/tests/fixture_instruments.rs:186-193` as the shape you are *not*
  building.** It reopens through `MemoryEventStore::restore` and is a working forgetting mechanism — it just
  happens to be this story's mutant. It is the single most likely accidental copy in the PR.
- **When many mutant rows go red at once, look at `declines()` before looking at the rule.** EC-006 is a
  one-line cause with a whole-table symptom.
- **Write the changelog entry when you write the rustdoc**, from the same sentences. CF-29's lint wants prose
  about the defect, and the defect's description already has to exist for NF-008.
- **Names in this spec are proposals except where marked binding.** `REMOVAL`, `remove_events` and
  `CompactingRestoreStore` come from the briefs and the discover; keeping them costs nothing and saves the
  next reader a translation between two documents.

## Tests and CI (merge gate)

Grounded in the project testing brief's AC-004 row (`_decomposition.md:605`), its AC-003 row (`:604`) and
its AC-011 row (`:612`), and in the *Merge-gate commands* section (`:620-650`). This project is rank 5 and
**not** terminal, so `cargo xtask ci --fast` is its ceiling; the whole gate is
`closeout-and-durable-audience`'s.

| tier | command / path | proves |
| --- | --- | --- |
| unit (registry) | `cargo test -p happenstance-testkit --test mutation_coverage` — `mutant_registry_is_exhaustive` (`:2754`) and `every_rule_has_a_mutant` (`:2734`) | **AC-001** (the rule is declared and registered), **AC-006** (the row exists with a non-empty `fails` and a real `provenance`) |
| integration (mutation) | the same command — `mutants_fail_exactly_their_declared_rules` (`:2889`) and `capability_skips_are_reported` (`:3184`) | **AC-005** (skips reported and accounted for, `MUST_SKIP` populated, the fully-capable fixture skipping nothing), **AC-006** (**exactly**, in both directions) |
| integration (conformance, honest) | `cargo test -p happenstance-testkit --test completeness_instrument` — the modules `event_store_conformance!` generates against `ForgettingFixture` in the suffix and the scattered configurations | **AC-004** (green in both), and **AC-003**'s non-vacuity against a store that genuinely forgets |
| integration (conformance, gapped) | `cargo test -p happenstance-testkit --test mutation_coverage` against `GappedPositionFixture` | **AC-003** (CF-6 enforced behaviourally: a literal-position assertion cannot survive a store whose positions are neither dense nor starting at 1), **AC-005** (a fully-capable fixture skips nothing) |
| integration (reference fixtures) | `cargo test -p happenstance-testkit` over `memory_conformance`, `local_conformance`, `memory_conformance_blocking` | **AC-001** (the rule's module is emitted in each), **AC-005** (`MemoryFixture` reports a skip with a stated reason, not an absence) |
| compile-time / wasm32 | the four mandatory `wasm32` steps inside `cargo xtask ci --fast` | **NF-003** — the seam introduces no `Send` bound and no runtime; the rule is emitted into `memory_conformance_wasm` and still builds for the `!Send` flavour |
| static (lints) | `cargo xtask lints` — `no_position_literals` (`xtask/src/lints.rs:628`), the CF-29 changelog check (`:525-580`), the CF-33 clock check (`:231`) | **AC-003**'s literal ban (second line of defence), **AC-007**, **NF-006** |
| static (spec) | `cargo xtask spec-trace` | **NF-001** — no marker moved and §7.1/§7.2 unchanged; the rule now exists for the clause that claims it |
| static (surface) | `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`); `git diff --stat` against the PR-boundary globs; `git diff -- spec/SPECIFICATION.md crates/happenstance-core crates/happenstance-testkit/Cargo.toml` | **AC-002** (defaulted, not required), **AC-008** (three additive items, nothing else moved) |
| story grain (automatic) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | all of the above for `happenstance-testkit` and its dependents, plus fmt, clippy `-D warnings` (NF-004, NF-005), and the five file-reading lints and `spec-trace` unconditionally |
| integration grain, tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | AC-007 and NF-001 in isolation, cheaply |
| integration grain, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | NF-003, NF-008 (the doc builds), and the `cargo package --list` licence/README assertion over the three publishable crates |
| static (content review) | the rustdoc on the rule and the two `Fixture` items; the implementation report's surface enumeration | **NF-008**'s three sentences including the head-removal residual, and **AC-008**'s hand-over to `surface-diff-and-the-ac-012-escalation` |

**Not run here, on purpose.** `cargo xtask ci` (the whole gate) is HS-P0019's recorded proof artefact —
useful locally, never this story's (`_decomposition.md:647-650`). `redkiln validate --kb` has nothing to
check: this story authors no atom, and ADR-0028 is `adr-0028-and-the-open-question-wave`'s. Re-running
HS-S0115's pass-list command is **not** this story's gate step either; the denominator change is recorded and
the artefact is regenerated by its own committed command.

## Risks and coupling (PR-scoped)

| risk | why it is real here | containment inside this PR |
| --- | --- | --- |
| **The rule is written to the answer someone expected** | The obvious assertion — "positions restart from 1 after removal, so check they do not" — is a literal in disguise, and it is what a rule written before the evidence would say | The `depends_on` edge is exactly this: HS-S0115's pass list arrives first (`_decomposition.md:543-546`). AC-003 bans literals and the gapped fixture enforces it behaviourally |
| **The seam grows into a floor** | `remove_below(p)` is shorter to write and reads naturally against a suffix store | AC-002 makes the set-shaped signature a criterion, with ES-39's scattered-purge argument (`spec/SPECIFICATION.md:4336-4341`) as its reason. A floor would pre-decide ADR-0028's central question inside the testkit |
| **A required `Fixture` item lands "because it is cleaner"** | The two existing required consts (`SECOND_HANDLE`, `REOPEN`) make a third look consistent | AC-002 and `cargo-semver-checks`; DA-4's precedent is `MID_BATCH_FAULT`, which is defaulted precisely for this reason (`_decomposition.md:276-308`) |
| **The mutant is sloppy and fails several rules** | A store rebuilt from a truncated snapshot can also disturb `contains_event_id` and head monotonicity — the discover deferred this to spec against the code | One overridden step, and the removal step only (AC-006, EC-005). The structural argument holds: removal is reachable only through the new capability and only this rule requires it |
| **A whole-table mutant failure is misdiagnosed as a rule bug** | The missing `declines()` line produces dozens of red rows far from its cause | EC-006 names the symptom and the cause together, and the Context pack names it before any code is written |
| **The rebuild gets copied by accident into the honest side** | `fixture_instruments.rs:186-193` is a *working* forgetting mechanism sitting fifty lines from code this story reads | AC-004 requires `MemoryEventStore::restore` to be absent from the instrument target; the symmetry is stated as the story's spine — the honest store hides, the mutant rebuilds |
| **The story quietly grows a marker move** | Removing `(new)` from ES-38's `Rule:` line is one character and feels like finishing the job | NF-001 and AC-008: `spec/SPECIFICATION.md` byte-identical. `marker-moves-and-spec-trace-green` (HS-S0123) raises the `(new)` question rather than inheriting an assumption |
| **A second rule appears** | CF-39's precedent (a companion rule proving a capability fires) is real and tempting | Weighed and declined in the Behavior table: removal is directly observable by a read, so non-vacuity folds into assertion (1). A second rule would owe a second mutant under CF-1 and a second changelog entry |

**Coupling, stated plainly.** Everything this PR touches lives in `crates/happenstance-testkit` plus one
`CHANGELOG.md` entry. Its coupling is *inbound* — it consumes `happenstance-core`'s port unchanged — and
*sideways* to one file this story does not own: `crates/happenstance-testkit/tests/completeness_instrument.rs`,
HS-S0114's target, which gains a capability declaration and a removal implementation. Nothing outside the
testkit can observe a change, which is what makes `cargo xtask affected --base main` a sufficient
story-grain gate. Two downstream stories inherit this PR's result: HS-S0123 needs the rule to *exist* before
it can move ES-38's marker, and HS-S0121 needs the rule's outcome as ADR-0028's evidence.

## Dependencies

**Blocks-on: `cf-27-experiment-and-recorded-pass-list`** (HS-S0115), matching `story.md`'s
`blocked_by: [HS-S0115]` and `_storymap.md:69`, `:133-136`.

| what it supplies | why this story cannot start without it |
| --- | --- |
| The enumerated per-configuration pass list from CF-27's experiment | It is the evidence that says whether the honest instrument already passes what this rule will assert. Writing the rule first means writing it to the answer someone expected (`_decomposition.md:543-546`) |
| DA-3's finding on which of the three outcomes holds | It settles whether AC-004's "green in both configurations" is a prediction or an observation |
| Transitively, HS-S0114's instrument target `crates/happenstance-testkit/tests/completeness_instrument.rs` and its `ForgettingFixture` | The rule needs a store that can honestly be made to forget. If the target is absent when this story starts, that is EC-009 — halt and report, never write a bespoke stand-in |

**Unlocks** (`story.md` frontmatter `blocks: [HS-S0121, HS-S0123]`):

| story slug | what it takes from this one |
| --- | --- |
| `adr-0028-and-the-open-question-wave` (HS-S0121) | The rule's existence and its result as evidence in ADR-0028's record, and the set-versus-floor argument already exercised in code rather than only argued in prose |
| `marker-moves-and-spec-trace-green` (HS-S0123) | ES-38's rule existing, which is the precondition for taking `(new)` off its `Rule:` line and for `spec-trace`'s checks 4 and 6 to stay green after the marker moves |

**Slice-mate, not a dependency.** `condition-over-removed-history-does-not-reject` carries no edge to this
story in `_storymap.md`; both depend on the experiment. They are implemented in the same context and mounted
as one integrated surface, and this story merges first because its mutant is the by-product of DA-2's
rejected construction and is already in hand (`_storymap.md:133-136`).

## Anchors (progressive disclosure)

Open these when the bound criterion is the one in hand. Everything load-bearing this spec distilled rather
than reproduced is here; nothing was pasted in bulk, and every path below was confirmed to exist while
writing this spec.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | ES-38 at `:4299-4323` is the assertion this story writes in Rust — its normative sentence (`:4299-4307`), the recorded reason it was deliberately not written and its phase-14 owner (`:4309-4319`), and its `Rejects:` naming the mutant verbatim (`:4321-4323`). ES-39 at `:4336-4341` is why a floor is the wrong seam shape. CF-6 at `:7233-7245` is the literal-position ban. CF-29 at `:8140-8168` is the rule/mutant/changelog triple | ES-38 before writing anything; ES-39 before fixing the seam's signature; CF-6 before the first assertion; CF-29 before the changelog entry | AC-001, AC-003, AC-006, AC-007 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!` at `:94` is the **mount** — the one line that makes the rule real; the *Sequence positions* group at `:139-141` is where it goes; `:412` is `no_orphan_rules`, which catches a body registered nowhere; `:293` is the name-emitting callback the meta-tests compare against | First, before the rule body — register the name and let `every_rule_has_a_mutant` go red | AC-001, NF-009 |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` contract. `:44-54` draws the capability-versus-fact line this seam depends on; `:161` and `:173` are the two **required** consts that must stay two; `:200-211` and `:290-307` are the `MID_BATCH_FAULT` defaulted-seam pattern in both halves; `:31-42` rejects `#[cfg]`-ing a rule out by name; `:67` is the import list that lacks `SequencePosition` | Immediately before adding the seam, and again at `:67` when the first compile error arrives | AC-002, AC-005, EC-001 |
| `crates/happenstance-testkit/src/suite.rs` | `require!` at `:37-46` and the `RuleOutcome` contract — how a declined capability becomes a reported skip; `:105-140` are the helpers (`append_ok`, `read_ok`, `event`, `positions_of`, `collect`) that make the rule read like its neighbours; `:2706-2712` is a live `require!`-gated rule to copy; `:2794-2800` is CF-39's companion-rule precedent this story weighed and declined | Before writing the rule body; `:2794-2800` only if a second rule is ever contemplated | AC-003, AC-005 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The mutation harness. `:141-186` are `Declared`'s fields and why `expect` is keyed by rule; `:324` is `REGISTRY`; `:2056` is `for_each_mutant!`; `:2734`, `:2754`, `:2889` and `:3184` are the four meta-tests; `:2907-2925` is the unaccounted-skip failure EC-006 produces; `:3157` is `MUST_SKIP`; `:3306-3314` is the fully-capable-fixture-skips-nothing assertion; `:2965-2975` is the assertion-origin check | Before writing the `REGISTRY` row; again the moment more than one mutant row goes red | AC-005, AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | The `Defect` trait at `:110-200` and `MutantFixture<D>` at `:353-390` — the one-defect-per-store discipline in the authors' own words, and the exact method set a defect may override | When writing `CompactingRestoreStore`, and when EC-005 fires | AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/correct.rs` | `Log` and `Log::append` at `:450-480` — the honest allocator this story must **not** rewrite, and the reason the rule leaves the store's head in place; `:530-536` records why this binary is `Rc`-backed and single-threaded, which is why the mutant is a `Defect` and not a bespoke store | Before deciding which positions the rule removes; before proposing any store outside the `Defect` family | AC-003, AC-006, NF-008 |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` | `GappedPositionFixture` at `:229-300` — the conformant store whose positions are gapped, which must gain honest removal support; `:116-121` is the durable `committed` second copy EC-007 warns about | When adding honest removal to the variants, and when a reopen resurrects something | AC-003, AC-005, EC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | `declines()` at `:523-570` — *"the one list in the binary with no mechanical backstop"*, which needs a fourth line; `Subject` at `:60-70` | At the same moment the capability is added to `contract.rs`, not later | AC-005, EC-006 |
| `crates/happenstance-core/src/memory.rs` | Why a rebuild reuses positions: `restore` at `:163`, `position_at` at `:277-282`, and `append`'s `first_index = stored.len()` at `:386-389`. This is the mutant's arithmetic and the `provenance` the registry row must cite; it is also why `MemoryFixture` cannot honestly support removal | Before writing the mutant; before ever concluding `MemoryFixture` could just do this | AC-005, AC-006 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | `:186-193` reopens through `MemoryEventStore::restore` — a *working* forgetting mechanism that is precisely this story's mutant, sitting next to code that will be read for other reasons; `:201-205` is the mount shape | Open once, deliberately, to fix in mind the thing not to copy | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount/spec.md` | HS-S0114's spec — the `RetainedSet` type, the `ForgettingFixture`/`ForgettingHandle` vocabulary, its declined `REOPEN`, and the target `crates/happenstance-testkit/tests/completeness_instrument.rs` that this story extends. That file does not exist in the tree until HS-S0114 lands, which is why this spec cites the spec rather than the path | Before touching the instrument target — and immediately, if the target is missing (EC-009) | AC-004, EC-009 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | DA-1 (`:150-171`, the scattered retained set), DA-2 (`:173-232`, hide-don't-rebuild and the rejected construction that *is* the mutant), DA-3 (`:234-274`, the dishonest resolutions), DA-4 (`:276-308`, add nothing required to `Fixture` and where an answer belongs), composition roots (`:98-113`), the testing brief's AC-004 row (`:605`) and AC-011 row (`:612`), and the merge-gate commands (`:620-650`) | DA-2 before the mutant; DA-4 before the seam; the testing brief before running the gate | AC-002, AC-004, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | The project-grain wording this story traces to — AC-003 (`:202-204`), AC-004 (`:205-209`) and AC-011 (`:236-239`) — plus DR-3/DR-4 (`:150-159`) and the instrument-is-never-a-target rule (`:141-145`) | When checking a criterion still says what the project asked; before any thought of changing a port surface | AC-001, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The signed-off no-surface determination (`:38-48`, `:86-95`) and the API-surface note (`:32-36`) that scopes this story's `Fixture` addition under AC-011 | Before writing anything in the Interaction quality register, and before adding any `pub` item | AC-002, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/positions-are-not-reused-after-removal/discover.md` | This story's own signal ledger, the two questions deferred to spec (`:35`, `:39`) and the wrong implementation as first named (`:45-54`), including the two ways this story can go wrong while everything stays green | If a decision in the Context pack seems arbitrary — the discover records what it was weighed against | AC-003, AC-006 |
| `xtask/src/lints.rs` | `MIN_CHARS_PER_RULE` at `:446` and the CF-29 changelog check at `:525-580` — why a bullet listing the rule beside others fails; `no_position_literals` at `:628`; the CF-33 clock check at `:231` | When writing the changelog entry, and if `cargo xtask lints` rejects it | AC-003, AC-007, NF-006 |
| `crates/happenstance-testkit/Cargo.toml` | `:4-14` states in the crate's own words that adding a conformance rule is semver-MINOR and *"can turn a passing adapter's CI red"* — the second sentence AC-008 must hand over | When writing the implementation report's surface enumeration | AC-008 |
| `crates/happenstance-testkit/src/lib.rs` | `:189` is `pub use suite::rules` — the re-export that makes the new rule function part of the published surface, and therefore why AC-008 counts three items and not one | When enumerating the additive surface | AC-008 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The accepted decision behind ES-38's obligation, and its *"What this ADR leaves open"* — the reason implementing a `[FROZEN]` clause's own named rule owes no new ADR | If it ever seems that this story needs an ADR of its own | AC-001 |
| `.kb/decisions/0001-async-port-flavours.md` | Why no `#[async_trait]` and no `+ Send` bound may be written into the seam's future — the constraint that makes `wasm32` possible at all | Before writing the defaulted method's signature | NF-003, NF-004 |
| `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` | The open question this story's rule partly discharges, and whose remaining `read_from_a_gap_position` thread is **not** closed here | Only to confirm this story closes nothing in it — HS-S0121 owns the atom's supersession | AC-001 |
| `RUNBOOK.md` | Phase 14 in full at `:4626-4674`, including the *"a rule name rather than 'conformance failed'"* bar at `:4667`, and its status row at `:165` | For orientation on where this sits in the plan of record, and when deciding how a failure is reported | AC-001 |
| `CONTRIBUTING.md` | `:291-296` — `cargo-semver-checks` at the PR grain, which is what proves *this diff* does not break the API it branched from | When running AC-002's and AC-008's static checks | AC-002, AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's eight**, AC-001 – AC-008. None was added and none dropped, and
   the ledger matches. Project AC-003 is carried by AC-001 (the failure arrives as a rule name); project
   AC-004 by AC-001, AC-003, AC-004, AC-005 and AC-006 (written, registered, non-decorative, green against
   the reference fixture as a reported skip, and failing **exactly**); project AC-011 by AC-002 and AC-008.
2. **"Green against `MemoryFixture`" (project AC-004) is satisfied by a reported skip, not by a pass.** The
   reference fixture cannot honestly support removal — its only shrinking route re-derives positions and
   *would be the mutant* — so the criterion is met by a test that exists, runs, and reports the fixture's own
   stated reason. AC-005 states this in those terms so a reader cannot mistake the skip for an evasion, and
   the rule's *running*-green evidence is moved to AC-004's instrument and AC-003's gapped fixture.
3. **The discover's preferred answer — "no seam" — lost, against the instrument as built.** The discover
   said to try it first (`discover.md:35`). HS-S0114's `RetainedSet` is fixed at construction, its fixture
   presents an empty store at `connect()` and it declines `REOPEN`, so nothing existing lets a rule make a
   store lose something *during* the rule. Both no-seam alternatives are recorded with their reasons in the
   Context pack rather than left as an unexplained reversal.
4. **The discover's deferred question — "does the mutant fail exactly this rule?" — is answered
   structurally, not empirically.** Removal is reachable only through the new capability and only this rule
   requires it, so a defect confined to the removal step cannot leak into another rule. EC-005 records what
   to do if the empirical answer disagrees: narrow the mutant, never widen `fails`.
5. **No companion rule.** CF-39's precedent (a second rule proving a declared capability actually fires) was
   weighed and declined; removal is directly observable by a read, so the non-vacuity check folds into the
   rule's own first assertion (AC-003). Recorded so the omission reads as a decision rather than an oversight.
6. **The head-removal case is a documented residual, not a silent omission.** The rule removes only below the
   store's highest assigned position; NF-008 requires the reason and the hand-off to ADR-0028 in the rule's
   rustdoc.
7. **`crates/happenstance-testkit/tests/completeness_instrument.rs` is cited through HS-S0114's spec, not as
   a path.** It does not exist in the tree yet. The anchors table therefore points at the dependency's
   `spec.md`, and EC-009 makes its absence at implementation time a halt-and-report rather than a licence to
   improvise.
