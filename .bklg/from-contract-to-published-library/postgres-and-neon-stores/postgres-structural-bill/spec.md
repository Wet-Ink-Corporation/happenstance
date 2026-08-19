---
item: HS-S0066
stage: spec
created: 2026-08-12T13:47:05.124Z
updated: 2026-08-12T13:47:05.124Z
template_sig: 87bbf1d0
rendered_sig: 025685c0
---

# Spec — The structural bill written where a consumer meets it

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-structural-bill/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — *Architecture brief* §2 (Root A, Root D), §3, §10 third bullet; *Testing brief* AC table row **AC-009** (`:519`) and the AC-009 mapping row at `:65` |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **no user-facing surface**, approved 2026-08-12. `## Items`, `## Signatures` and `## The doctest` are all `N/A`, so this story renders **no** design surface and invents none. |
| Story map row | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md:65` (slice row) and `:142-146` (the longer statement) |
| Roadmap pointer | `RUNBOOK.md:4311-4390` — phase 10 in full; `RUNBOOK.md:303` — ADR-0024's queue row |

## One-line PR slice

A consumer reading the Postgres crate's public docs meets the frontier `head`, the absence of
read-your-own-writes and the cluster-wide staleness bound, and the choice between doc prose and a
clause of its own is recorded as a decision rather than defaulted.

## Executive summary

`spec/SPECIFICATION.md:2833-2842` **already** states the three consequences of the arm-C shape —
`head()` reports a frontier rather than `max(position)`, read-your-own-writes does not hold, and
staleness is bounded by the longest open write transaction anywhere in the cluster, at 0.688 ms
unloaded and 4010.719 ms behind an unrelated five-second write. So the delta this PR lands is not
"discover the bill". It is two things the existing prose does not do:

1. **Move the bill to where a consumer meets it.** Nobody adopting `happenstance-postgres` reads
   `spec/SPECIFICATION.md`. They read docs.rs. The crate root
   (`crates/happenstance-postgres/src/lib.rs`) today says the mechanism is an **open decision**
   (`:28-36`), and `head`'s own item doc says it is `todo!()` and blocked (`event_store.rs:146-161`).
   After the slice-mate ADR-0024 lands, both of those sentences are false and the crate is
   advertising a question it has answered. This PR replaces them with the shipped bill.
2. **Settle where the normative statement lives, on the record.** The project's own brief is exact
   about this: AC-009 "records whether that is *sufficient* or a clause of its own is owed — **as a
   decision, not a default**" (`_decomposition.md:65`). There is real in-tree evidence on both sides,
   and the failure mode is not choosing wrongly — it is nobody choosing, and the answer being
   whatever the implementer happened to type.

It also closes the third ordered sub-question of
`.kb/open-questions/postgres-arm-c-structural-cost.md`, which is the one this repository has left
explicitly unanswered: *which* staleness magnitude a caller should plan for, given a measured range
spanning four orders of magnitude. A doc that repeats both numbers without telling a caller which to
budget for has restated the experiment, not written a bill.

## Context pack

Read this section and you can start. Everything below the anchors table is deferred, not optional.

**The mechanism is already chosen; this story does not re-choose it.** ADR-0013 lifted ES-10 to
`[FROZEN]` on the strength of the phase-2 experiment, and arm C — `xid8` + `pg_snapshot_xmin` — is
the only arm that both passes the inversion detector on both writer pairs *and* leaves writers
unserialised, at 0.987 / 0.993 / 1.015 / 1.026 against a bracketing baseline at 1 / 8 / 32 / 64
clients (`spec/SPECIFICATION.md:2866-2872`). The slice-mate story
`adr-0024-position-visibility-mechanism` (HS-S0065) re-measures that against the **real adapter** and
records the choice. **This story writes the bill for whatever ADR-0024 actually chose.** That is a
live conditional, not a formality: if ADR-0024 picks a lock arm, the frontier collapses back to
`max(position)`, read-your-own-writes *does* hold, and shipping arm-C prose by inertia would be a
documented capability limit the adapter does not have
(`crates/happenstance-postgres/src/event_store.rs:43-77`).

**The three consequences, stated as the decisions they are.** Under arm C:

- **`head()` is a frontier, not a maximum.** `max(position)` can name a row whose predecessors are
  still in flight, and a head is a promise that nothing at or below it will appear later. The frontier
  is `max(position) WHERE xid < pg_snapshot_xmin(pg_current_snapshot())`, and it *trails* the maximum
  (`crates/happenstance-postgres/src/event_store.rs:146-161`). This is why ES-30's
  `head_is_the_highest_visible_position` asserts a **bound rather than an equality**
  (`spec/SPECIFICATION.md:2840-2842`, `crates/happenstance-testkit/src/suite.rs:1798`) — the rule was
  already written for this adapter before this adapter existed.
- **Read-your-own-writes does not hold.** `append` returning `Ok(P)` does **not** promise that the next
  `head()` is at or above *P* (`spec/SPECIFICATION.md:2833-2836`). This is the second kind of portfolio
  row: the adapter compiles, every signature is satisfied, and then a guarantee a caller assumed is
  simply absent (`crates/happenstance-postgres/src/event_store.rs:68-75` records that framing).
- **Staleness is bounded by the longest open write transaction anywhere in the cluster.** One forgotten
  `BEGIN` in an unrelated application, in an unrelated *database*, is a ceiling on every reader and
  every projection. Measured at 0.688 ms with no holder and 4010.719 ms behind an unrelated five-second
  write (`experiments/position-visibility/results/staleness_pinned.txt`). **It is a documented
  capability limit, not a tuning parameter** (`spec/SPECIFICATION.md:2840-2841`) — writing it as
  something an operator can turn down would be a factual error, not a stylistic one.

**The clause-siting decision, with the evidence that already exists on each side.** Do not treat this
as a coin toss; the repository has already been burned once here.

- *Against a clause of its own, and it is strong.* ES-10 ends with: "**This clause is the sole
  statement of the visibility invariant.** VT-12 carried a second copy; the two drifted apart within
  one editing pass, which is the argument against stating a requirement twice" — VT-12 survives only as
  a cross-reference so citations resolve (`spec/SPECIFICATION.md:2852-2856`, `:1056`). A new clause
  restating the frontier is exactly the second copy that failed before.
- *For a clause of its own.* Every clause in this document names the conformance rule that checks it
  and the wrong implementation it forbids (`CLAUDE.md`, *Open questions*). The bill's three
  consequences are today **prose inside** a frozen clause, carrying no rule id of their own; a reader
  auditing the clause ledger at phase 12 cannot tell which of them is checked by anything. If a
  consequence is checkable and unchecked, prose is where that fact hides.
- *The bar either way.* A new clause must name its rule and its rejected implementation, or it is
  decorative in exactly the way CF-4 and `CLAUDE.md`'s "a rule that no adapter can fail is decorative"
  forbid. A declination must name VT-12 as its reason and say what a phase-12 auditor should read
  instead. **Silence is the one outcome this AC forbids.**

**Where the decision is recorded, and the immutability trap.** Decision records reach `.kb/decisions/`
through `.kb/_intake/` and `/redkiln:kb-ingest`, never by hand — hand-writing atoms is what commit
`0269720` reverted (`_decomposition.md` *Architecture brief* §2, Root D). **Accepted atoms are
immutable**, and `redkiln validate --kb` checks them against `HEAD`. This story is a slice-mate of
HS-S0065 and is implemented in the *same* context, so the clause-siting verdict must be written into
ADR-0024's staged intake document and its `references/adr/` long form **before** that wave is handed to
`/redkiln:kb-ingest`. If the atom has already been accepted by the time this verdict is reached, the
correction is a **new superseding atom staged into `.kb/_intake/`** — never an edit to the accepted
body, and never a new ADR number invented here on the spot. Route it through the runbook's ADR queue
(`RUNBOOK.md:262-268`) rather than minting one as a side effect.

**The mount is documentation, and documentation is gated here.** `cargo xtask ci`'s "documentation"
step runs `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with
`RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-300`), which exists precisely because the step used
to print "generated 3 warnings" and exit 0. So a broken intra-doc link in the bill fails the gate, and
a fenced example in the bill written as ```` ```text ```` compiles nothing while a ```` ```no_run ````
one is checked. Prefer the checked fence.

**The persona slice.** The "user" is an adapter author and a consuming application, not a screen
(`_storymap.md:18-23`). The humane outcome is that somebody choosing `happenstance-postgres` for a
read-after-write workflow learns it will not serve them **on the docs.rs landing page**, before they
have written a line — rather than in production, from a read model that is correct about a state the
business forbids (`crates/happenstance-postgres/src/lib.rs:12-21`).

**Not this story.** Whether ES-10 should have been per-boundary rather than global is a real live
question with a measured ~9% cost at 64 clients, and it is **owned by phase 6**
(`projection-store-freeze`) — read
`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` for context, do not settle it
here (`project.md` *Out of scope*). The poll-padding decorator that `spec/SPECIFICATION.md:2844-2850`
owes phase 10 belongs to `postgres-rule-controls`, not here. Removing `publish = false` and the
skeleton markers belongs to `deskeleton-and-package-readiness`.

## Integration contract

- **Archetype**: `capability`. The observable artefact is a rendered public documentation surface plus
  a recorded decision — which is what "user-observable" means in this repository's only medium
  (`_storymap.md:18-23`).
- **Slice / milestone**: `position-visibility-decision`. Slice-mate: `adr-0024-position-visibility-mechanism`
  (HS-S0065), which is also this story's `depends_on` and its `blocked_by` frontmatter edge. The two are
  implemented in one context and mounted as one integrated surface: **the ADR's verdict and the bill
  that quotes it must not be able to disagree.**
- **Mount point**: `crates/happenstance-postgres/src/lib.rs` — the crate-root rustdoc. This is the
  composition root for this capability because it is the docs.rs landing page: the one page a consumer
  meets before any method, and today the page that still advertises "**Open decisions** — how ES-10 is
  bought" (`:28-36`) and "**Status: not implemented**" (`:3-9`). A bill written only into a module doc
  or a backlog artifact is constructed-but-unmounted.
- **Second render path (same mount, item grain)**:
  `crates/happenstance-postgres/src/event_store.rs` — the item docs on `head` (`:151-166`) and `append`
  (`:137-145`), and the module-level ES-10 note (`:26-70`), whose three-candidate framing becomes false
  the moment ADR-0024 chooses. The consequence a caller hits is hit *through a method*, so the method's
  own doc must carry it and not merely defer upward.
- **Wires into**:
  - `spec/SPECIFICATION.md:2816-2872` — ES-10, the normative source the bill must agree with and the
    document the siting decision is about. `[FROZEN]`: **not editable here** except by the ADR route.
  - `.kb/_intake/` and the `references/adr/0024-*.md` long form authored by HS-S0065 — where the siting
    verdict is recorded (Root D, `_decomposition.md` *Architecture brief* §2).
  - `.kb/open-questions/postgres-arm-c-structural-cost.md` — sub-question 3 is the staleness magnitude
    this bill must answer; reconciled through `.kb/_intake/`, never by hand-editing the atom.
  - `experiments/position-visibility/results/staleness_pinned.txt` and
    `experiments/position-visibility/README.md` — the measured numbers the bill cites, plus whatever
    ADR-0024 re-measured against the real adapter.
  - `crates/happenstance-testkit/src/suite.rs:1798` — `head_is_the_highest_visible_position`, the rule
    that already asserts a bound rather than an equality, i.e. the existing check the frontier claim
    rests on.
  - `standards/rust/70-rustdoc-obligations.md` — the house rules the rendered surface is held to.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for this project and its
  `## Items`, `## Signatures` and `## The doctest` blocks are `N/A` (approved 2026-08-12). This story
  therefore claims no design item and must not invent one. It changes **documentation on existing public
  items**; it adds no new public item, and adding one would be a surface nobody signed off.
- **Conformance rule(s)**: **not adapter-observable, and the reason is the deliverable.** A doc comment
  cannot fail a conformance rule, and inventing one that greps for prose would be decorative in the exact
  way CLAUDE.md forbids. The nearest thing to an observing rule is
  `head_is_the_highest_visible_position` (`crates/happenstance-testkit/src/suite.rs:1798`), which the
  bill's frontier claim must remain consistent with but which this story does not change. This is
  precisely why AC-009 is graded **Static** in the testing brief (`_decomposition.md:519`) — rustdoc build
  plus a human read, plus `redkiln validate --kb` if the record lands as an atom.
- **Clause(s)**: **ES-10** (`spec/SPECIFICATION.md:2816-2872`) — this story **discharges the consumer-facing
  half of its documented capability limit** and decides whether it is amended by the addition of a
  sibling clause. ES-10 is `[FROZEN]`: its existing text is not edited here, and any new clause is minted
  through ADR-0024's record, never by an edit. **ES-30** (`spec/SPECIFICATION.md:3941`) is cited, not
  changed — the bill must not contradict its bound-not-equality reading.
- **Advances DoD scenario**: initiative **DoD 5** — "a store that does not serialise its writers passes
  the suite, with the position-visibility cost measured rather than estimated"
  (`.bklg/from-contract-to-published-library/initiative.md:373-374`). This story owns the half of "cost"
  the experiment could not price: the **structural** bill, as opposed to the throughput number
  (`project.md` DR-3). It also feeds initiative **DoD 10** ("the published crate looks finished",
  `initiative.md:387-389`), since docs.rs is where that is judged.

## PR boundary

```
crates/happenstance-postgres/src/**
spec/SPECIFICATION.md
references/adr/0024-*.md
.kb/_intake/**
.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-structural-bill/**
```

**In this PR**

- The structural bill on `crates/happenstance-postgres/src/lib.rs`'s crate-root rustdoc: the frontier
  `head`, the absence of read-your-own-writes, and the staleness bound with the magnitude a caller should
  plan for.
- The same bill at item grain on `crates/happenstance-postgres/src/event_store.rs` — `head`, `append`,
  and the module ES-10 note rewritten from three candidates to one shipped mechanism.
- Deletion of the now-false advertisements: the "Open decisions — how ES-10 is bought" bullet
  (`lib.rs:28-36`) and the "Status: not implemented" framing on the paths this slice has made real.
- The clause-siting verdict, written into ADR-0024's staged intake document and its `references/adr/`
  long form **before** ingest — or, if that atom is already accepted, staged into `.kb/_intake/` as a
  superseding record. Plus, in the "clause is owed" branch only, the new clause in
  `spec/SPECIFICATION.md` naming its rule and the wrong implementation it forbids.
- An answer to sub-question 3 of `.kb/open-questions/postgres-arm-c-structural-cost.md`, staged through
  `.kb/_intake/` — or the explicit, reasoned statement that it remains open with what would close it.
- Adding this story's own ledger under its backlog folder (`require_ledger: true`,
  `.redkiln/config.yaml:62-67`).

**Explicitly not in this PR**

- Choosing the mechanism, or producing the throughput/long-running-transaction number — HS-S0065.
- Any `todo!()` body, any SQL, any schema column — `postgres-append-and-frontier-head`.
- The poll-padding decorator over `PreCommitPositionStore` that `spec/SPECIFICATION.md:2844-2850` owes
  this phase — `postgres-rule-controls`.
- Removing `publish = false`, the scoped `#![allow(clippy::todo)]`, or adding READMEs and licence files —
  `deskeleton-and-package-readiness`.
- Hand-editing any file under `.kb/decisions/`, `.kb/maps/` or `.kb/open-questions/`. Atoms are
  `/redkiln:kb-ingest`'s to write; this story stages into `.kb/_intake/`.
- Editing ES-10's existing `[FROZEN]` text, reopening global-versus-per-boundary visibility, or any
  Neon-side documentation.

**Merge DoD (one line).** `cargo xtask ci` is green including the `-D warnings` rustdoc step, the rendered
crate root states all three consequences with the planning magnitude named, and the clause-siting verdict
exists as a written decision naming its loser — not as a silence.

The implementer MAY also touch the wiring named in the Integration contract — `spec/SPECIFICATION.md` and
the ADR-0024 record — to mount this bill; that is not scope drift.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The crate root states the frontier `head`** | A consumer landing on docs.rs reads, before any method, that `head()` reports the **visibility frontier** and not `max(position)`, and that the frontier trails the maximum. Stated as shipped behaviour, not as a candidate among three. | `crates/happenstance-postgres/src/lib.rs:28-36` (the open-decision bullet this replaces); `crates/happenstance-postgres/src/event_store.rs:146-161` (the frontier spelled out per mechanism); `spec/SPECIFICATION.md:2833-2834` |
| **`head`'s item doc carries it too** | The consequence is met through a method call, so `head`'s own rustdoc states the frontier and links ES-10's reading rather than deferring silently to the crate root. Intra-doc links resolve or the gate fails. | `crates/happenstance-postgres/src/event_store.rs:146-161`; `xtask/src/main.rs:290-300` (`RUSTDOCFLAGS=-D warnings`) |
| **Read-your-own-writes is documented as absent** | Stated in the caller's terms and not the mechanism's: `append` returning `Ok(P)` does **not** promise the next `head()` is at or above *P*, and a read issued immediately after a successful append may not contain it. Named as a **capability limit**, so a reader knows it is not a bug to report. | `spec/SPECIFICATION.md:2833-2836`; `crates/happenstance-postgres/src/event_store.rs:68-75` |
| **`append`'s item doc names the limit at the point of surprise** | The `# Errors` / behaviour prose on `append` says what `Ok(P)` does and does not promise. This is where a caller forms the wrong expectation, so this is where it is corrected. | `crates/happenstance-postgres/src/event_store.rs:136-144`; `standards/rust/70-rustdoc-obligations.md` |
| **The staleness bound is stated with its scope** | Bounded by the longest open write transaction **anywhere in the cluster** — including an unrelated transaction in an unrelated database. Both measured endpoints appear with their conditions: 0.688 ms with no holder, 4010.719 ms behind an unrelated five-second write. | `experiments/position-visibility/results/staleness_pinned.txt`; `spec/SPECIFICATION.md:2836-2841` |
| **A caller is told which magnitude to plan for** | The open question states plainly that "nothing yet says which of those a caller should plan for". The bill answers it — the planning figure, its assumed conditions, and what makes it degrade — or records that it is still open and what would close it. Repeating both numbers without a recommendation restates the experiment. | `.kb/open-questions/postgres-arm-c-structural-cost.md` (*Ordered sub-questions* §3, *What is not decided*) |
| **It is a limit, not a knob** | No wording that implies an operator can tune the staleness down. The specification names this distinction directly, and it is why ES-30's rule asserts a bound rather than an equality. | `spec/SPECIFICATION.md:2840-2842`; `crates/happenstance-testkit/src/suite.rs:1798` |
| **The bill matches the mechanism ADR-0024 actually chose** | Mechanism-conditional, and this is a real branch: under either lock arm the frontier collapses to `max(position)` and read-your-own-writes holds, so the arm-C bill would be describing an adapter that does not exist. The doc quotes the ADR's verdict; the two cannot disagree. | `crates/happenstance-postgres/src/event_store.rs:43-77`; the slice-mate record at `references/adr/0024-*.md` / `.kb/_intake/` |
| **The clause-siting verdict is recorded, with its loser named** | Exactly one of: (a) no new clause — recorded with VT-12's drift as the stated reason and a pointer to what a phase-12 auditor reads instead; or (b) a new clause in `spec/SPECIFICATION.md` naming its conformance rule and the wrong implementation it forbids, with `cargo xtask spec-trace` green. Silence is not an outcome. | `spec/SPECIFICATION.md:2852-2856` and `:1056` (the VT-12 precedent); `_decomposition.md:65`; `xtask/src/main.rs:315` (`specification traceability`) |
| **The record reaches the KB the sanctioned way** | Staged under `.kb/_intake/` for `/redkiln:kb-ingest`; if ADR-0024's atom is already accepted, a **superseding** record, never an edit to the accepted body. No ADR number is invented here. | `_decomposition.md` *Architecture brief* §2 (Root D); `CLAUDE.md` (accepted atoms immutable, `0269720`); `RUNBOOK.md:262-268` |
| **The stale advertisements are gone** | The crate root no longer says the ES-10 mechanism is an open decision, and no path this slice made real still claims "not implemented". A crate that documents a question it has answered is worse than one that documents nothing. | `crates/happenstance-postgres/src/lib.rs:3-9,28-36` |
| **Any example in the bill is compiled** | Code in the bill uses a checked fence (```` ```no_run ````, or ```` ```rust ```` where it can run), not ```` ```text ````. The gate compiles doctests; an uncompiled fence is prose that rots without warning. | `standards/rust/70-rustdoc-obligations.md`; `xtask/src/main.rs:143` (the `tests` step) |
| **Out of the bill's scope, stated in it** | The bill does not reopen global-versus-per-boundary visibility (phase 6's), does not restate ES-10's normative sentence a second time, and does not describe Neon. | `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`; `project.md` *Out of scope* |

## Data and migrations

**N/A.** This story writes no SQL, adds no column, and touches no schema. The migration that carries
migration 1's identity/time columns plus whatever column the chosen mechanism needs belongs to
`postgres-schema-and-live-fixture` and `postgres-append-and-frontier-head`
(`_storymap.md:60-61`); this story only *documents* what the mechanism already landed there costs a
caller. The single artefact it produces that is not a doc comment is a decision record staged under
`.kb/_intake/`, which `/redkiln:kb-ingest` — not this story — integrates.

## Acceptance criteria

Seven criteria. The "user" is an adapter author and a consuming application, not a screen
(`_storymap.md:18-23`), so each criterion is written from the journey it improves:
*Decide in one sitting* (the evaluator's bounded look at public evidence) and *Choose a contract
before a database* (the application author's first hour), both named at
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` and
carried into `initiative.md:241-250`. The medium is the rendered docs.rs page and the written
decision — the only two things this repository can show a user.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator making an adopt-or-decline call in one sitting, **WHEN** they land on `happenstance-postgres`'s docs.rs page and read the crate root *before* opening a single method, **THEN** the crate root states — in the persistent chrome of the crate-level rustdoc, above the module and item listings — that `head()` reports the **visibility frontier** rather than `max(position)`, that the frontier **trails** the maximum, and that this is the shipped behaviour of the mechanism ADR-0024 chose, naming the arm that lost exactly once (RS-70-5) rather than presenting three candidates. | `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-300`) renders the page; the read of the rendered crate root is recorded as evidence in `_ledger.md` per the Static tier the testing brief assigns AC-009 (`_decomposition.md:519`). Source of record: `crates/happenstance-postgres/src/lib.rs`. |
| AC-002 | **GIVEN** an application author who arrived by deep link to a method — `#method.append` or `#method.head` — rather than through the crate root, **WHEN** they read that method's own documentation, **THEN** the consequence they are about to hit is stated there and not merely deferred upward: `append`'s docs say that returning `Ok(P)` does **not** promise the next `head()` is at or above *P* and that a read issued immediately afterwards may not contain the event, and `head`'s docs say what a frontier is. It is named a **capability limit**, in the caller's terms rather than the mechanism's, so a reader knows it is not a bug to report. | `cargo doc … -D warnings` (`xtask/src/main.rs:290-300`) plus a read of the rendered `PostgresEventStore` item page. Source of record: `crates/happenstance-postgres/src/event_store.rs` (`append` at `:136-144`, `head` at `:146-161`). Consistency with `spec/SPECIFICATION.md:2833-2836` checked in the same read. |
| AC-003 | **GIVEN** an application author choosing a contract before a database, who needs to know whether a read-after-write workflow is servable, **WHEN** they read the staleness statement, **THEN** they learn its **scope** (bounded by the longest open write transaction *anywhere in the cluster*, including an unrelated transaction in an unrelated database), both measured endpoints with their conditions (0.688 ms with no holder; 4010.719 ms behind an unrelated five-second write), **and the single magnitude they should budget for** — which closes sub-question 3 of `.kb/open-questions/postgres-arm-c-structural-cost.md`, or states explicitly that it remains open and what measurement would close it. The wording admits no reading in which an operator can tune the bound down: it is a documented capability limit, not a knob. | `cargo doc … -D warnings`; numbers checked verbatim against `experiments/position-visibility/results/staleness_pinned.txt`. The open-question disposition is staged into `.kb/_intake/` and checked by `redkiln validate --kb` after ingest. |
| AC-004 | **GIVEN** the slice-mate `adr-0024-position-visibility-mechanism` has just chosen the adapter's mechanism, **WHEN** a reader compares the ADR's verdict with the crate's documentation, **THEN** they cannot be made to disagree: the bill describes the mechanism actually chosen. If ADR-0024 selected either lock arm, the frontier collapses to `max(position)`, read-your-own-writes **does** hold, and AC-001–AC-003's arm-C prose must not ship — a documented limit the adapter does not have is as false as an undocumented one it does. | Read of the ADR-0024 record (`references/adr/0024-*.md` and its `.kb/_intake/` staging) beside the rendered crate root, in the one context the slice is implemented in; the correspondence is cited line-for-line in `_ledger.md`. Backstop: `crates/happenstance-postgres/src/event_store.rs:43-77` enumerates what each arm costs, so a mismatch is visible in the same file. |
| AC-005 | **GIVEN** a phase-12 publication auditor reading the clause ledger, **WHEN** they ask whether the three consequences are stated normatively or only as prose inside ES-10, **THEN** they find a written verdict with its loser named — exactly one of **(a)** *no new clause*, recorded with VT-12's within-one-editing-pass drift as the stated reason and a pointer to what the auditor should read instead; or **(b)** a new clause in `spec/SPECIFICATION.md` naming its conformance rule and the wrong implementation it forbids. Silence, or a clause that names no rule, fails this criterion. The verdict is written into ADR-0024's staged intake document and `references/adr/` long form **before** ingest, or — if that atom is already accepted — staged as a **superseding** record; never an edit to an accepted body, and no ADR number invented here. | Branch (b) only: `cargo xtask spec-trace` (`xtask/src/main.rs:315`) proves the new clause's rule and case citations resolve. Both branches: `redkiln validate --kb` (accepted-decision immutability against `HEAD`) after `/redkiln:kb-ingest`, plus the verdict text itself cited in `_ledger.md`. Precedent read: `spec/SPECIFICATION.md:2852-2856` and `:1056`. |
| AC-006 | **GIVEN** the same evaluator, **WHEN** they scan the crate root for what is unresolved, **THEN** the crate no longer advertises a question it has answered: the "**Open decisions — how ES-10 is bought**" bullet (`lib.rs:28-36`) is gone, no path this slice made real still says "**Status: not implemented**" (`lib.rs:3-9`), and the module-level ES-10 note (`event_store.rs:26-77`) reads as one shipped mechanism rather than three candidates and no measurement. The crate root ends with **no more than five** `#` sections — it carries four today — so the bill lands by **replacing** stale chrome, not by appending a fifth and sixth section beneath it. | Rendered crate-root read plus a diff review against `crates/happenstance-postgres/src/lib.rs:1-63` and `event_store.rs:26-77`; the heading count is read off the rendered page and cited in `_ledger.md`. `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) green over the change. |
| AC-007 | **GIVEN** any reader on any docs.rs build of this crate, **WHEN** the page renders, **THEN** the bill exists as **composed rustdoc** and survives the gate: it is `//!`/`///` documentation with real headings and not a `//` body comment (which renders nowhere), every fenced code block in it uses a **checked** fence (` ```no_run ` or ` ```rust `, never ` ```text `), every intra-doc link resolves in **every** feature configuration the crate can be built in (RS-70-2), and nothing load-bearing sits behind `#[doc(hidden)]`, a non-default feature, or `--document-private-items`. No new public item is added and no custom HTML, CSS or JavaScript is introduced into the rustdoc. | `cargo doc … -D warnings` (`xtask/src/main.rs:290-300`) — `rustdoc::broken_intra_doc_links` is `deny`, so an unresolved link is a hard error. `cargo test -p happenstance-postgres --doc`, reached by the gate's `tests` step (`xtask/src/main.rs:143`), compiles every checked fence. The `semver compatibility` job (`.github/workflows/ci.yml:279`) proves the public API surface did not change. |

Coverage: this story is the **sole owner** of project **AC-009** (`_storymap.md:225`), and AC-001 – AC-007
above partition it — AC-001/AC-002/AC-003 are the three consequences on the consumer-facing surface,
AC-004 keeps them true of the shipped mechanism, AC-005 is the "decision, not a default" half the brief
names explicitly (`_decomposition.md:65`), and AC-006/AC-007 are what stop the surface being a lie or a
render failure. No other project AC is claimed here.

## Interaction quality

RFC §6.7/D6. **Every invariant below is already carried by an AC-### row in the table above** — this
section says which row carries it and how it is verified, and adds no criterion of its own. That is
deliberate: `redkiln verify` extracts ACs from table cells and `- AC-###:` bullets, so an invariant
stated only as prose here would never be gated.

**The surface question, answered first.** `_design.md` (approved 2026-08-12) declares
**N/A — no user-facing surface** at `## Surfaces` (`:10-12`) and repeats `N/A` at `## Items`,
`## Signatures`, `## Anti-patterns` and `## The doctest` (`:46-84`). There is therefore **no signed-off
composition to honour and none may be invented** — inventing one would be a surface a human never
approved. What follows is the honest translation of both families into the only rendered medium this
story has: the docs.rs page and the written decision.

**State family**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump.** The reader learns the limit on the page they are already on. "See `spec/SPECIFICATION.md`" as the *only* statement is the failure — that document is not on docs.rs and a consumer never opens it. | AC-001, AC-002 | Rendered crate root and item page carry the statement themselves; the spec is cited, never substituted for. |
| **Non-occlusion.** Nothing load-bearing is hidden behind `#[doc(hidden)]`, a non-default feature, or `--document-private-items` (which docs.rs does not pass). Both `event-store` and `projection-store` are default features today (`crates/happenstance-postgres/Cargo.toml:29`) — the invariant is that moving the bill somewhere that changes this fails the criterion. | AC-007 | `cargo doc … -D warnings`; the rendered default-feature page is what is read. |
| **Preserved entry point.** A reader who arrived by anchor deep-link to `#method.append` must meet the limit without scrolling up to the crate root. Deferring the whole bill upward is the anti-pattern. | AC-002 | Read of the rendered `PostgresEventStore` item page in isolation. |
| **Reversibility.** The decisions are reversible *knowingly*: the losing arm is named once (RS-70-5, `standards/rust/70-rustdoc-obligations.md:243-249`) and the clause-siting verdict names what it rejected, so a future reader can reopen either without re-deriving the argument. | AC-004, AC-005 | Ledger evidence quotes the named loser in each case. |
| **Reachability.** Rustdoc's own chrome is left unmodified — no custom HTML, CSS or JavaScript — so the page keeps the keyboard and screen-reader behaviour rustdoc already ships. | AC-007 | `cargo doc … -D warnings`; the diff introduces no `#[doc(html_…)]` change beyond the existing `html_no_source` (`lib.rs:58`). |

**Composition family** — no `_design.md` surface, so these are the medium's own equivalents, and they
are still blocking:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** The bill is composed documentation — `//!`/`///` with real `#` headings — not a `//` body comment. A comment inside a `todo!()` body renders on no page; it is this medium's exact equivalent of shipping bare markup. Today's frontier explanation lives in precisely that invisible form (`event_store.rs:147-159`), which is the wrong implementation this invariant rejects. | AC-007 | Rendered page shows the text; `cargo doc … -D warnings`. |
| **Placement and hierarchy.** Crate root first and above the item listings (the landing page an evaluator meets before any method); item docs second, at the point of surprise. Not the reverse, and not one without the other. | AC-001, AC-002 | Both pages read; the ordering is cited in `_ledger.md`. |
| **Transience.** Crate root = **persistent chrome**, on screen the moment the page loads. Item docs = **revealed**, on the method. Nothing load-bearing is **opened-on-demand** — no `<details>`, no collapsed section, no "see also" carrying the only copy of a limit. | AC-001, AC-002, AC-007 | Rendered read; absence of collapsed containers in the diff. |
| **Density budget, with its real number.** The crate root carries **four** `#` sections today (`lib.rs:3, 10, 28, 49`). The bill lands at **≤ 5** by replacing "Open decisions" and the "Status: not implemented" framing, not by appending beside them. A sixth section is a page nobody finishes reading. | AC-006 | Heading count read off the rendered page and recorded in the ledger. |
| **Named anti-patterns.** `_design.md` names none (no surface), so the repository's standing three apply: **a second copy of a normative statement** (ES-10's VT-12 drift, `spec/SPECIFICATION.md:2852-2856`); **prose that narrates the signature** rather than naming the constraint (RS-70-5's "Not" example, `standards/rust/70-rustdoc-obligations.md:280-297`); and **a decorative check** — a clause or rule no implementation can fail (`CLAUDE.md`). | AC-005, AC-007 | AC-005's branch (b) must name a rule and a wrong implementation or fall back to (a); AC-007's read rejects signature-narrating prose. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | ADR-0024 chooses a **lock arm** (serialised sequence table or advisory lock) rather than arm C. | The arm-C bill must not ship. Under either lock, allocation order is commit order, the frontier collapses to `max(position)` and read-your-own-writes holds (`crates/happenstance-postgres/src/event_store.rs:43-63`). AC-001 – AC-003 are then satisfied by documenting *that* shape and its own cost — writer serialisation — not by inertia. Record in the ledger which branch was taken and why. |
| **EC-002** | ADR-0024's decision atom has already been **accepted** by `/redkiln:kb-ingest` when the clause-siting verdict is reached. | Stage a **new superseding record** under `.kb/_intake/`. Never edit the accepted body — `redkiln validate --kb` checks accepted atoms against `HEAD` and will fail the gate. Never invent an ADR number here; route it through the runbook's ADR queue (`RUNBOOK.md:262-268`). |
| **EC-003** | An intra-doc link in the bill resolves under `--all-features` but not in some other configuration (RS-70-2). | Use the house spelling: name the type without linking it and say in one sentence why it is not linked, or use paired `#[cfg_attr(feature = "…", doc = "…")]` with an ungated fallback (`standards/rust/70-rustdoc-obligations.md:101-121`). A link that fails in any buildable configuration is a hard error, not a warning. |
| **EC-004** | Sub-question 3 cannot be answered honestly — no re-measurement under contention exists, because the slice-mate's harness did not produce one. | **Do not invent a planning figure.** AC-003 is satisfied by an explicit, reasoned statement that the magnitude remains open, naming the measurement that would close it, staged through `.kb/_intake/` so the open-question atom is updated rather than quietly contradicted. Repeating both endpoints with no guidance and no admission is the failure. |
| **EC-005** | Branch (b) is chosen but no conformance rule can be named for the new clause. | Branch (b) is unavailable: a clause naming no rule is decorative in exactly the way `CLAUDE.md` and CF-4 forbid. Fall back to branch (a) — declination with VT-12 named — and record that the missing rule is *why*. Do not mint a rule that greps for prose. |
| **EC-006** | `cargo xtask spec-trace` fails after a new clause is added. | The clause's rule/case citations do not resolve. Fix the citations, or withdraw the clause to branch (a). Do not silence the step: it exists because 24 real dangling citations survived hand-written cross-references (`xtask/src/main.rs:303-315`). |
| **EC-007** | The bill's wording would restate ES-10's normative sentence a second time. | Cite ES-10, do not copy it. Two copies of the visibility invariant is the precise failure VT-12 already produced, within one editing pass (`spec/SPECIFICATION.md:2852-2856`). |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | **No public API change.** No new public item, no signature change, no visibility change. | This story documents existing items. `_design.md` signed off **no surface**, so adding a public item here is a surface nobody approved. Checked by the `semver compatibility` job (`.github/workflows/ci.yml:279`). |
| **NF-002** | **Documentation builds clean under `-D warnings`.** | `RUSTDOCFLAGS=-D warnings` exists because the step printed "generated 3 warnings" and exited 0 for as long as it ran (`xtask/src/main.rs:285-301`). A bill that warns is a bill that will be silently edited into rot. |
| **NF-003** | **Every number is quoted with its unit and its conditions, verbatim from the measurement.** | 0.688 ms and 4010.719 ms are meaningless without "no holder" and "behind an unrelated five-second write in an unrelated database". Source: `experiments/position-visibility/results/staleness_pinned.txt`; the specification quotes them the same way (`spec/SPECIFICATION.md:2836-2841`). Rounding or dropping conditions is a factual error. |
| **NF-004** | **No dependency, feature, MSRV or `Cargo.toml` change.** | The crate's feature set (`crates/happenstance-postgres/Cargo.toml:28-34`) and the workspace MSRV of 1.97.1 (ADR-0029) are untouched by a documentation story. Removing `publish = false` belongs to `deskeleton-and-package-readiness`. |
| **NF-005** | **`clippy::doc_markdown` stays enabled; proper nouns go in `doc-valid-idents`.** | An `allow` disables the lint for every doc comment in its scope (RS-70-3, `standards/rust/70-rustdoc-obligations.md:152-158`). The bill is prose-heavy and will tempt exactly that shortcut. |
| **NF-006** | **The bill is legible to a reader who has not read the specification.** | The evaluator journey is a *bounded* look at public evidence (`initiative.md:249-250`). A bill whose sentences only parse if you already know what ES-10 says has moved the problem, not solved it. Verified in the same manual read the Static tier requires (`_decomposition.md:519`). |
| **NF-007** | **Story-grain gate stays tree-local.** No Docker, no network, no credentials to verify this story. | `cargo xtask ci --fast` and `cargo xtask affected --base <base>` are the wired grains (`.redkiln/config.yaml:40,55`), and AC-011's Docker-free rule (`_decomposition.md:521`) is a project-wide constraint this story must not be the one to break. |

## Implementation notes (non-prescriptive)

Shape suggestions only; the implementer owns the wording.

- **Write the ADR first, then quote it.** This story shares a context with HS-S0065. The cheapest way to
  satisfy AC-004 is to let the ADR's verdict paragraph be the source and the rustdoc be a restatement in
  the caller's vocabulary — mechanism words in the ADR, caller words in the crate. Writing the doc first
  and back-filling the ADR is how the two end up disagreeing.
- **Three consequences, one ordering, everywhere.** Frontier → no read-your-own-writes → staleness bound.
  Crate root states all three; `head` states the first; `append` states the second and third. The reader
  who lands anywhere gets the same order.
- **A `no_run` fence is the strongest thing available here.** A short example that appends and then reads
  `head()` back, with the comment naming what is *not* promised, is checked by `cargo test --doc` and
  therefore cannot rot silently. ` ```text ` buys nothing (`standards/rust/70-rustdoc-obligations.md`,
  and the gate's `tests` step at `xtask/src/main.rs:143`).
- **Say the planning number as a number.** "Sub-millisecond under normal operation; unbounded above by
  the longest open write transaction in the cluster" is a bill. "Between 0.688 ms and 4010.719 ms" is the
  experiment restated, which is what the open question already says is insufficient
  (`.kb/open-questions/postgres-arm-c-structural-cost.md`, *What is not decided*).
- **The module ES-10 note wants deletion, not extension.** `event_store.rs:37-77` is a three-candidate
  comparison ending in "Nothing above is a measurement, and the choice is owed one." After ADR-0024 that
  last sentence is false. Keep the *rejected* arms only to the extent RS-70-5 wants — one naming of the
  alternative that lost — and delete the rest rather than annotating it.
- **The invisible prose is the real find.** The best explanation of the frontier in the tree today sits
  inside `head`'s `todo!()` body as a `//` comment (`event_store.rs:147-159`). It renders nowhere. Promote
  it to `///`, do not rewrite it from scratch.
- **Leave the `todo!()` bodies alone.** Documenting the shipped behaviour of a method whose body is still
  `todo!()` is fine and expected here if `postgres-append-and-frontier-head` has not merged yet — but the
  merge order puts it first (`_storymap.md:249-254`), so in practice the bodies will be real. If they are
  not, say what the crate *will* do under the chosen mechanism and do not claim it is done.

## Tests and CI (merge gate)

Grounded in the testing brief's AC-009 row, which grades this **Static** (`_decomposition.md:519`) — the
tier is a statement about the *instrument*, not a licence to skip. Every row below is a real command or a
real path.

| tier | command / path | proves |
| --- | --- | --- |
| **Static — rustdoc** | `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-300`) | The bill renders; every intra-doc link resolves; no rustdoc warning. AC-001, AC-002, AC-003, AC-006, AC-007. |
| **Compiled — doctests** | `cargo test -p happenstance-postgres --doc`, reached by the gate's `tests` step (`xtask/src/main.rs:143`) | Every checked fence in the bill compiles. An uncompiled ` ```text ` fence would pass the row above and fail nothing. AC-007. |
| **Static — traceability** | `cargo xtask spec-trace` (`xtask/src/main.rs:315`); also runs unconditionally inside `cargo xtask affected` (`.redkiln/config.yaml:36-40`) | Branch (b) only: a new clause's rule and case citations resolve, and §7.1/§7.2 still equal what the checker computes. AC-005. |
| **Static — knowledge base** | `redkiln validate --kb` | The staged decision record carries valid `KbFrontmatter`, and no accepted atom's body was edited. AC-005, and AC-003's open-question disposition. |
| **Static — public surface** | `semver compatibility` job (`.github/workflows/ci.yml:279`) | No public API changed. NF-001. |
| **Story grain** | `cargo xtask affected --base <base>` (`.redkiln/config.yaml:40`) | fmt, clippy `-D warnings` and tests for `happenstance-postgres` and its dependents, plus the five file-reading lints and `spec-trace` — which is what makes a docs-and-specification story gateable at all. |
| **Integration grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | This project's non-terminal bar: `REQUIRED` steps only. Includes the rustdoc step, the doctests and `spec-trace`. |
| **Human read (recorded)** | The rendered crate root and `PostgresEventStore` item page, read against this spec's AC table; evidence cited in `_ledger.md` | The half no compiler reaches: that the three consequences are *stated*, in the caller's vocabulary, in the right order, in the right places. This is what the Static tier means by "plus a manual read" (`_decomposition.md:519`). |
| **Ledger gate** | `redkiln verify --grain story` over `.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-structural-bill/_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:62-67`) | Every AC-001 – AC-007 row present, `satisfied: true`, with non-placeholder evidence. Blocks `implement → report`. |

**Not run here, deliberately.** No Docker, no live Postgres, no credentials — the live-server job belongs
to `postgres-schema-and-live-fixture` and the conformance families to
`postgres-append-and-frontier-head`. AC-011's Docker-free default gate (`_decomposition.md:521`) is a
constraint this story is easy to break by "just verifying the frontier once".

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **The bill ships arm-C prose after ADR-0024 chose a lock arm.** | Low / High | AC-004 exists solely for this, and EC-001 names the required behaviour. The two stories share one context precisely so the verdict and its restatement cannot drift. |
| **The clause-siting verdict is defaulted rather than decided** — the implementer writes doc prose, nobody writes down that a clause was considered and declined, and the phase-12 auditor re-derives the argument. | **Medium** / Medium | This is the failure mode the brief names in terms (`_decomposition.md:65`). AC-005 forbids silence and requires the loser be named; EC-005 closes the escape hatch of a rule-less clause. |
| **A second copy of the visibility invariant.** A new clause, or over-enthusiastic rustdoc, restates ES-10's normative sentence and the two drift. | Medium / High | EC-007 and NF-004's precedent: this exact drift already happened once with VT-12, within one editing pass (`spec/SPECIFICATION.md:2852-2856`). Cite, never copy. |
| **A number is invented.** The planning magnitude is asserted without a measurement behind it. | Medium / High | EC-004 makes "still open, and here is what closes it" an acceptable answer, which removes the incentive to guess. NF-003 requires conditions travel with every figure. |
| **An accepted atom is edited.** ADR-0024 is ingested first, then the clause-siting verdict is bolted on by editing the atom body. | Low / High | EC-002; `redkiln validate --kb` fails against `HEAD`, and `CLAUDE.md` records that hand-authoring atoms is what commit `0269720` reverted. |
| **Scope creep into phase 6.** The global-versus-per-boundary question is genuinely adjacent — the bill's "anywhere in the cluster" scope is the *global* premise — and it is tempting to settle it in a sentence. | Medium / Medium | `project.md` *Out of scope*, and the front half's *Not this story*. The bill may state the scope; it may not decide whether the scope should be global (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`). |
| **The docs change lands but the stale advertisements survive** — a crate root that states the frontier *and* still lists "Open decisions — how ES-10 is bought". | Medium / Medium | AC-006 names both stale blocks by line, and the ≤ 5 heading budget makes "append beside it" fail. |
| **Coupling: `postgres-append-and-frontier-head` merges after this.** | Low / Low | The merge order puts it first (`_storymap.md:249-254`). If it slips, AC-002's item docs describe behaviour whose body is still `todo!()`; state the chosen mechanism's behaviour, do not claim implementation. |

## Dependencies

**Blocks on**

- `adr-0024-position-visibility-mechanism` (HS-S0065) — **hard**. This story documents a decision; until
  the decision exists there is nothing to write, and AC-004 is unsatisfiable. Same slice
  (`position-visibility-decision`), same implementation context, and this story's `depends_on` and
  `blocked_by` frontmatter edge (`_storymap.md:65`).

Transitively, through HS-S0065: `postgres-schema-and-live-fixture` and `postgres-append-and-frontier-head`
(`_storymap.md:249-254`) — the ADR's number is a re-measurement against a working append path, not against
four SQL strategies.

**Unlocks**

- `deskeleton-and-package-readiness` — names `postgres-structural-bill` among its own `depends_on`
  (`_storymap.md:71`). It cannot honestly remove `publish = false` and the skeleton framing from a crate
  root that still advertises an open decision.
- `far-end-discharge-record` (transitively) — ES-10's status "after this work — discharged, still exposed,
  or amended, and against which store" is exactly what AC-005's verdict records
  (`_storymap.md:72`).

## Anchors (progressive disclosure)

Everything above is enough to start. Open these at the moment named, not before — and **link**, never
bulk-paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (ES-10, lines 2816-2872) | The normative source. Carries the three consequences verbatim, the arm-C throughput ratios, the poll-count limitation phase 10 owes, and the VT-12 sole-statement paragraph that is the strongest argument against a new clause. `[FROZEN]` — read it, do not edit it. | Before writing the first sentence of the bill (AC-001), and again before deciding the clause siting (AC-005). | AC-001, AC-002, AC-003, AC-005 |
| `crates/happenstance-postgres/src/lib.rs` | The mount. Lines 3-9 and 28-36 are the two stale blocks the PR deletes; lines 10-26 are the "instrument first" framing that stays and must not be contradicted; line 58's `html_no_source` is the only doc attribute in play. Four `#` headings — the density budget's real number. | First file opened at implementation. | AC-001, AC-003, AC-006, AC-007 |
| `crates/happenstance-postgres/src/event_store.rs` | The second render path and the best prose in the tree: `:26-77` is the three-candidate ES-10 note that becomes false, `:43-63` is what each lock arm costs (the EC-001 branch), `:64-75` is arm C's structural cost including the read-your-own-writes framing, and `:147-159` is the frontier explanation currently trapped inside a `todo!()` body where it renders nowhere. | Immediately after `lib.rs`, when writing the item-grain half. | AC-002, AC-004, AC-006 |
| `.kb/open-questions/postgres-arm-c-structural-cost.md` | Sub-question 3 (`:89-92`) is the question AC-003 must answer or explicitly leave open, and *What is not decided* (`:62-70`) states plainly that nothing yet says which magnitude a caller should plan for. Also names the four gaps between "a SQL strategy passed" and "an adapter passed". | Before writing the staleness paragraph. | AC-003 |
| `experiments/position-visibility/results/staleness_pinned.txt` | The measurement itself. Both endpoints and their conditions, verbatim — the only admissible source for the two numbers (NF-003). | When quoting a number. | AC-003 |
| `experiments/position-visibility/README.md` | What the experiment did and did not do, including the `ratios.csv` versus `ratios-c1long.csv` substitution the specification cites at `:248-256`. Read it before characterising the cost of the chosen arm, so the bill does not over-claim. | Alongside the staleness paragraph, when tempted to generalise. | AC-003, AC-004 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The accepted atom that froze ES-10 and is explicit that it settles the *mechanism's affordability*, not the adapter's choice — the sentence AC-004 depends on for knowing that ADR-0024, not ADR-0013, owns what shipped. | Before writing AC-004's correspondence check. | AC-004 |
| `standards/rust/70-rustdoc-obligations.md` | The house rules the rendered surface is held to: RS-70-2 (`:93-148`) on links that resolve in only some configurations, RS-70-3 (`:152-158`) on `doc_markdown`, RS-70-5 (`:243-297`) on naming the alternative that lost **once** and its "Not" example of signature-narrating prose. | Before writing any doc comment; again before review. | AC-001, AC-007 |
| `xtask/src/main.rs` | The gate, defined once. `:143` the `tests` step that compiles doctests, `:285-301` the `-D warnings` rustdoc step and the story of why it exists, `:303-324` `spec-trace` and the 24 dangling citations that motivated it. | When you want to know what will actually fail. | AC-005, AC-007 |
| `crates/happenstance-testkit/src/suite.rs` (`head_is_the_highest_visible_position`, ~`:1798`) | The rule that already asserts a **bound rather than an equality** — the existing check the frontier claim rests on. The bill must stay consistent with it; this story does not change it. | When writing the `head` item doc. | AC-002 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | *Architecture brief* §2 Root D (the `.kb/_intake/` route and the `0269720` lesson) and the testing brief's AC-009 row (`:519`, the Static tier and what the manual read must confirm), plus the AC-009 architecture row (`:65`, "as a decision, not a default"). | Before staging anything into `.kb/_intake/`. | AC-005, AC-003 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off design: `N/A — no user-facing surface` at `## Surfaces`, `## Items`, `## Signatures`, `## Anti-patterns`, `## The doctest`. Read it to confirm that **no** composition was approved, so none may be invented and no public item added. | Before adding anything that looks like an API. | AC-007, NF-001 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The four personas and the journeys the AC table is written from — in particular *Decide in one sitting* (the evaluator's bounded look at public evidence) and *Choose a contract before a database*. The bill is written for those readers, not for a specification editor. | When judging whether a sentence is legible to someone who has not read the spec (NF-006). | AC-001, AC-003 |
| `RUNBOOK.md` (phase 10, `:4311-4390`; the ADR queue, `:262-268`) | The plan of record: what phase 10 owes, and the sanctioned route for minting an ADR — which is *not* here. | If you find yourself about to invent an ADR number. | AC-005 |
| `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` | The adjacent question that is **not** this story's. Read it to recognise the temptation and stop, not to settle it. | If the staleness scope paragraph starts to argue about whether the invariant should be global. | AC-003 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's seven**, AC-001 – AC-007; none added, none dropped. They
   partition project AC-009 as described under the acceptance table.
2. **This story renders no design surface, and that is signed off, not assumed.** `_design.md` records
   `N/A — no user-facing surface` in every relevant block (`:10-12`, `:46-84`). The composition family in
   *Interaction quality* is therefore the medium's own equivalent — rendered rustdoc — and not an
   invented design. No public item may be added (NF-001).
3. **The interaction-quality invariants are carried by AC rows, not by prose bullets.** `redkiln verify`
   extracts ACs from `| AC-### |` cells and `- AC-###:` bullets; an invariant stated only in the
   *Interaction quality* section would be ungated and untested. Every invariant there names the AC that
   carries it.
4. **"Static" is the tier, not a licence.** The testing brief grades AC-009 Static
   (`_decomposition.md:519`). This spec reads that as: two compiler-checked instruments (the `-D warnings`
   rustdoc build and the doctest compilation) plus a recorded human read, with `spec-trace` and
   `redkiln validate --kb` on the branches that reach them — not "no verification".
5. **AC-003 has an honest escape, and it is not silence.** If no contention re-measurement exists, the
   criterion is met by stating explicitly that the planning magnitude remains open and naming the
   measurement that would close it (EC-004). What it is *not* met by is quoting both endpoints and
   leaving the caller to choose — the failure the open question already names.
6. **AC-005's branch (b) is conditional on a nameable rule.** A clause that names no conformance rule and
   no wrong implementation is decorative and is refused (EC-005); the fallback is branch (a) with the
   missing rule recorded as the reason. This keeps "decide, don't default" from becoming "add a clause to
   look decisive".
7. **The density budget's number was measured, not chosen.** `lib.rs` carries four `#` headings today
   (`:3, 10, 28, 49`); the budget is ≤ 5 so the bill must replace stale chrome rather than append to it.
8. **No ADR number is minted here.** The clause-siting verdict rides ADR-0024's record. If that atom is
   already accepted, it is superseded through `.kb/_intake/` (EC-002), and the runbook's ADR queue owns
   any new number (`RUNBOOK.md:262-268`).
