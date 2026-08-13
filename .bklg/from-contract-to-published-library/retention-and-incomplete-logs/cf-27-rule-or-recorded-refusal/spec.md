---
item: HS-S0122
stage: spec
created: 2026-08-12T13:48:03.043Z
updated: 2026-08-12T13:48:03.043Z
template_sig: 87bbf1d0
rendered_sig: b15777cc
---

# Spec — CF-27's own rule, written or refused in writing

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-11, AC-14, **DoD 15** (`:402-405`), exit criterion 5 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md:239-249` — **gate decision 4**: retention's answer is constrained to what needs no published-surface change |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — **AC-007** (`:219-222`); DR-11, DR-12 (`:181-185`) |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal/spec.md` |
| Key briefs | `.bklg/.../retention-and-incomplete-logs/_decomposition.md` — architecture brief **DA-3(3)** (`:268-270`, the expected escalation), **DA-4** (`:276-308`, the defaulted fixture seam), **DA-7** (`:363-379`, the pre-computed option table), **DA-8** (`:381-397`, the lie already on a shipped surface), *Gate mechanics* (`:468-496`, checks 4 / 6 / 8 / 9); testing brief row **AC-007** (`:609`) |
| Signed-off design | `.bklg/.../retention-and-incomplete-logs/_design.md` — **N/A by sign-off**: this project records *no* user-facing surface; what was approved (2026-08-12) is the no-surface determination itself |
| Story map row | `.bklg/.../retention-and-incomplete-logs/_storymap.md:76` — slice `clause-exit-and-surface-record`, first of three |
| Discover stage | `.bklg/.../cf-27-rule-or-recorded-refusal/discover.md` — the signal ledger, the answered questions, and the three named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:517` (ADR-0028's lifecycle row: *"an explicit written refusal … is a legitimate answer"*); `RUNBOOK.md:4626-4674` (phase 14 in full) |
| The clause itself | `spec/SPECIFICATION.md:8034-8060` — CF-27, `[DEFERRED]`, and the `Rule:` line that names this story's subject |

## One-line PR slice

Settle CF-27's own rule either way: if ADR-0028 decides, write
`suffix_store_is_distinguishable_from_a_young_store` into
`crates/happenstance-testkit/src/suite.rs` with its assertion fixed by the retention decision and
register it exactly once in `for_each_event_store_rule!`
(`crates/happenstance-testkit/src/registry.rs:94`) — CF-27 already claims the name, so `spec-trace`
check 6 (`xtask/src/spec_trace.rs:727`) is satisfied; if ADR-0028 refuses, the rule is **not**
written, CF-27's `Rule:` line keeps its `(new)`/`†` marker (`xtask/src/spec_trace.rs:1626-1631`),
and the clause text records in writing why it cannot exist and what a reader is therefore on its
own against.

## Executive summary

This PR closes the last open thing CF-27 asks for: the rule the clause names but nobody has been
able to write. It is the only story in the project whose deliverable is *conditional on an ADR* —
and the conditionality is the point, not a hedge. `spec-trace` is green on both branches by
different mechanisms, so the branch is chosen by ADR-0028's text and never by which one keeps the
gate quiet.

Delta against the project brief and the story map, not a restatement of them:

- **The branch is read, not chosen.** HS-S0121 lands ADR-0028; this story quotes its decision
  statement and executes the matching branch. If ADR-0028 is not on disk when this story starts,
  that is a dependency failure to halt on and report — the same discipline `_storymap.md:111-115`
  applies to the projection runner and the `0.2.0` baseline.
- **The refusal branch is the one reachable inside gate decision 4, and it is not the cheap
  branch.** *Distinguishable* means the store **reports** something, and every reporting primitive
  ES-39 contemplates is a change to a port published at `0.2.0` (DA-7). So the honest refusal is a
  full deliverable: CF-27's clause text has to say what cannot be checked, why, and what a reader
  is left holding — cited to the three reader observations this project already recorded, not
  asserted afresh.
- **Both branches are specified in full, on purpose.** `discover.md:34` is binding here: *a story
  that has planned only the branch it expects will improvise the other one at the worst moment.*
  The acceptance criteria below are branch-labelled; the ones for the branch not taken are
  discharged as `not-applicable — branch B` with ADR-0028 quoted, never silently dropped.
- **This story moves no marker and regenerates no census.** CF-27 leaving `[DEFERRED]`, the
  `(new)` markers coming off, `spec-trace --write` and §1.3's hand-reconciled prose census are all
  `marker-moves-and-spec-trace-green`'s (HS-S0123). This story edits CF-27's *prose* and, on the
  decide branch, adds a rule; it does not touch the maturity marker.
- **A decide branch does not proceed silently.** Writing the rule presupposes a primitive; adopting
  a primitive is a published-surface change. That is an AC-012 escalation raised through
  `surface-diff-and-the-ac-012-escalation` (HS-S0124), and **no such change is made here**
  (`project.md:183-185`: a conclusion that a surface change is required is a finding, not a
  licence).

## Context pack

Read this section and you can start. Everything deeper is a signposted anchor below.

**1. What the clause actually asks for, and why nothing can answer it today.** CF-27
(`spec/SPECIFICATION.md:8034-8060`) requires a completeness instrument *"that deliberately holds
only a suffix of its own log, **and reports that it does**"*. Its `Rule:` line names
`suffix_store_is_distinguishable_from_a_young_store` and says, in the clause's own words, that
*"its assertion is fixed by the retention decision, which is ES-39's"*, adding *"This is the
instrument, not the primitive."* The port has four methods and none of them carries a completeness
channel: `read` (`crates/happenstance-core/src/store.rs:119`), `append` (`:213`), `head`, and
`contains_event_id` (`:268`). So an application holding only an `impl EventStore` has nowhere to
learn the answer — which is exactly what CF-27's `Rejects:` calls *"every conformant store's
silence about its own history"*.

**2. The branch is ADR-0028's to choose and this story's to execute.** HS-S0121 authors ADR-0028 as
one question — *what is a store permitted to forget, and how does it say so* — through
`.kb/_intake/` and `/redkiln:kb-ingest`. This story reads the accepted atom under `.kb/decisions/`,
quotes its decision statement into the record, and takes the matching branch. Two branches, both
terminal, both gate-green by *different* mechanisms:

- **Decide** — the rule is written, and `spec-trace` check 6 (`xtask/src/spec_trace.rs:727`,
  `check_rule_ownership` at `:770-800`) is satisfied because CF-27 already claims the name. Every
  rule in the registry must be claimed by a clause, retired by one, or listed in
  `UNCLAIMED_PENDING_ADR` (`:1978`) as owing a decision — and that array is a ratchet that can only
  shrink, so it is not an escape hatch this story may use.
- **Refuse** — the rule is not written, and `spec-trace` check 4 (`:695-711`) is satisfied because
  a clause may name a rule that does not exist **only** while its `Rule:` line carries `(new)` or
  `†` (`:1626-1631`). Keeping the marker is the mechanism, and the marker alone is not the
  deliverable.

**3. The refusal has terms, and `SilentNonRule` is the failure mode it exists to prevent.** A
refusal that merely leaves the marker in place is green on every check and worthless: the reader
who arrives at CF-27 in two years finds a rule that has been "new" for two years and no record of
why (`discover.md:49`). Project AC-007's second half is the whole deliverable —
*"ADR-0028's refusal records in writing why it cannot exist **and what a reader is therefore on its
own against**"* (`project.md:219-222`). "What a reader is on its own against" is not a phrase to
paraphrase; this project has three recorded observations that fill it, and the refusal text cites
them by artefact:

- `read_decision_model` (`crates/happenstance-core/src/store.rs:321-331`) returns the last
  **retained** match, and that position feeds `AppendCondition::after_opt` — so a follow-up
  conditional append is admitted where the destroyed history would have rejected it, silently, in
  existing code, with no new type (`_decomposition.md:113-124`, composition root 4).
- `IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) and its shipped sibling
  `EventStore::contains_event_id` answer `false` for an event this store minted and acknowledged.
  DA-8 (`_decomposition.md:381-397`) is precise about why that is worse than a gap: the doc's own
  reduction — *"if the identifier's store half is not its own incarnation the answer is `false`,
  and if it is, the question reduces to whether that position exists"* — is exactly wrong under
  forgetting, and `false` now means *"never had it"* to every caller. A peer that re-sends on
  `false` re-sends forever.
- The projection runner resuming across the hole, with its actual state recorded.

**4. Why "just write the rule against the instrument" is the trap.** A rule that distinguishes the
instrument from a young store using something *only the instrument* exposes — a method on the
concrete type, a `Capability` on its `Fixture`, or a downcast in the rule body — goes green, gets
claimed by CF-27, and gets registered. It proves the instrument remembers its own retained
predicate, which was never in doubt, while CF-27 asks whether an **application** holding an
`impl EventStore` can tell. DA-4 names the boundary that forbids it: *"Can you be made to forget?"*
is a **trade** and belongs on `Fixture` as a `Capability`; *what a store does not hold* is a
**fact** with no home on `Fixture` at all, because putting it there answers ES-39 in a place no
application can read (`_decomposition.md:299-305`). The defence written into this spec is
mechanical: **on the decide branch the rule body may use only `Fixture`'s declared surface and the
`EventStore` port**, exactly as every other rule in `suite.rs` does. `discover.md:45` names this
wrong implementation `SelfReportingSuffixStore`.

**5. Gate decision 4 binds the decide branch into an escalation.** *"Retention's answer is
constrained to what needs no published-surface change"*
(`.bklg/from-contract-to-published-library/_decomposition.md:239-249`) — under 0.x every one of
DA-7's four options is a minor bump, i.e. a `0.3.0` this initiative's exit criteria do not
contemplate. DA-7 has already pre-computed the option set so that reaching it is a decision and not
a discovery: `earliest_position()` (a floor — argued against by name at
`spec/SPECIFICATION.md:4336-4341`, because a regulated purge is *scattered, not a prefix*, and a
floor *"ships looking correct until a claim runs long"*), a set of retained ranges (new method plus
new public type), a third outcome on condition evaluation (a new variant reaching every caller's
`match`), and DA-8's tri-state `contains_event_id` (smallest signature delta, same semver class).
If ADR-0028 decided by adopting one of these, this story raises the escalation and **stops**;
`surface-diff-and-the-ac-012-escalation` (HS-S0124) owns the artefact, and the mechanical check is
that no diff touches the public signatures in `crates/happenstance-core/src/store.rs` or
`crates/happenstance-core/src/append.rs` (`_decomposition.md:614`).

**6. Where a rule body may live, and where it may not.** Rule names resolve against
`crates/happenstance-testkit/src/suite.rs` **only** — a body in a `tests/` file is invisible to
`spec-trace` check 4 (`xtask/src/spec_trace.rs:700-711`). The one registration point is
`for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`), which feeds all
four emitters (`__emit_tokio`, `__emit_blocking`, `__emit_wasm`, `__emit_rule_names`) and, through
`for_each_mutant!`, every registered mutant. A body in `suite.rs` that is not in the registry runs
nowhere. Rule bodies take the shape
`pub async fn <name><F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome`, and a
capability-gated rule uses `require!` (`crates/happenstance-testkit/src/suite.rs:37-42`), returning
`RuleOutcome::Skipped` with the fixture's stated reason rather than being `#[cfg]`-ed out —
`contract.rs`'s module docs reject that arrangement by name (`:31-42`).

**7. Any fixture-side seam takes the defaulted shape, and only if it earns it.** The precedent is
exact and already carries two live rules: a **defaulted** `Capability` associated const whose
default is `Capability::declined(<reason>)` (`crates/happenstance-testkit/src/contract.rs:207-211`)
plus a **defaulted** method that panics with a message naming both ways of reaching it (`:297-307`).
A **required** trait item would break every `Fixture` impl in and out of the workspace, which is
project AC-011's named failure. DA-4's closing instruction is binding: *add nothing if the rule can
be written without it* — a defaulted const nobody consults is one more line every adapter author
reads and no information.

**8. No literal position values, in either branch.** The specification permits gaps, so asserting
`[1, 2, 3]` converts a `MAY` into a `MUST` without an ADR. There is a sharper form of this here:
distinguishability must never be asserted as *"the store's history begins at position N"*, because
that is `earliest_position()` reimplemented in a test — the very shape ES-39 argues against
(`spec/SPECIFICATION.md:4336-4342`). The instrument's first retained position is a parameter of its
retained predicate and differs between the suffix and scattered configurations
(`_decomposition.md:150-171`, DA-1). Any assertion compares against positions the store actually
assigned (`discover.md:68`).

**9. Nothing `[FROZEN]` is amended here.** CF-27 is `[DEFERRED]` and the refusal branch edits
CF-27's own prose; ES-39 (`spec/SPECIFICATION.md:4325-4349`) is `[DEFERRED]`; ES-37 (`:4274-4297`)
and ES-38 (`:4299-4323`) are `[FROZEN]` and are **cited, not amended**. If the spec stage's reading
turns out to require a frozen clause to move, that is a new ADR written first and a re-plan, per
`project.md:112-114`.

**10. The persona-journey slice.** The reader this PR serves is the adapter author, and the
maintainer who arrives at CF-27 wanting to know whether the completeness axis has an instrument
with teeth. What they get from this PR is a clause that no longer defers the question to a rule
nobody wrote: either the rule exists and bites, or the clause says in its own text that it cannot
exist, why, and precisely which three readers will be wrong in the meantime. Either way the reader
stops guessing — which is DoD 15's *"the reader either fails loudly or the refusal to define this
is recorded as a decision"* landing in the one place a reader of the specification will actually
find it.

## Integration contract

- **Archetype**: `capability` — a user-observable slice. On the refusal branch what the user (a
  reader of `spec/SPECIFICATION.md`) observes is a clause that answers instead of deferring; on the
  decide branch it is a conformance rule that runs in every emitter against every registered
  mutant.
- **Slice / milestone**: `clause-exit-and-surface-record`. Slice-mates, implemented in the same
  context and mounted as one integrated result: `marker-moves-and-spec-trace-green` (HS-S0123,
  merges after this story — a marker can only move once the rule it names exists or is recorded as
  never-to-exist) and `surface-diff-and-the-ac-012-escalation` (HS-S0124, last — and the home of
  this story's escalation if the decide branch is taken). Merge order is `_storymap.md:146-149`.
- **Mount point**: **`spec/SPECIFICATION.md:8034-8060` — CF-27's clause**, which
  `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) reads as composition root 7 of the
  architecture brief (`_decomposition.md:143-145`). This is the render path for this story's
  primary artefact on both branches: on the refusal branch the clause body carries the recorded
  refusal, and on the decide branch the clause's existing `Rule:` claim is what makes the new rule
  legal under check 6. **On the decide branch a second mount is taken and is not optional**:
  `for_each_event_store_rule!` at `crates/happenstance-testkit/src/registry.rs:94` — composition
  root 2 (`_decomposition.md:94-106`) — because a rule body in `suite.rs` that is absent from that
  macro is invisible to all four emitters and to `for_each_mutant!`, i.e. it does not exist under
  `CLAUDE.md`'s rule that matters.
- **Wires into**:
  - `.kb/decisions/0028-<slug>.md` — ADR-0028, landed by HS-S0121 through `/redkiln:kb-ingest`.
    The decision statement this story quotes and executes. **Read-only here**: an accepted decision
    atom is immutable (`CLAUDE.md`, *Where the work lives*).
  - `crates/happenstance-testkit/src/suite.rs` — the only file a rule body may live in
    (`xtask/src/spec_trace.rs:700-711`), and the source of the body shape and the `require!` gate
    (`:15-42`).
  - `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`, `RuleOutcome`;
    capabilities are trades and limits are facts (`:44-54`); the defaulted-seam precedent at
    `:207-211` and `:297-307`.
  - `crates/happenstance-testkit/tests/mutation_coverage.rs:324` (`REGISTRY`) plus a `Defect` impl
    under `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` — the decide branch's
    "registered wrong implementation", proven load-bearing in both directions by
    `mutant_registry_is_exhaustive` and `mutants_fail_exactly_their_declared_rules`. No parallel
    harness may be invented (`_decomposition.md:107-112`).
  - The completeness instrument and its `event_store_conformance!` mount, landed by
    `retained-set-instrument-and-conformance-mount` in a
    `crates/happenstance-testkit/tests/` target shaped like
    `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` — the honest store a decide-branch
    rule must be green against.
  - `crates/happenstance-core/src/store.rs` and `crates/happenstance-core/src/append.rs` — **read
    only, and the check is mechanical**: no diff touches their public signatures
    (`_decomposition.md:614`).
  - The three recorded reader observations from `decision-model-and-ingest-observed` and
    `projection-runner-across-the-hole` — the refusal branch's citations for *what a reader is on
    its own against*.
- **Public items**: **none.** `_design.md` records `N/A — no user-facing surface` for this project
  and that determination is what was signed off (2026-08-12), so there is no `## Items` block to
  claim from. On the decide branch this story adds one `pub fn` rule body in `suite.rs` — additive,
  `0.2.x`-class, enumerated by `surface-diff-and-the-ac-012-escalation` (project AC-011) — and
  `_design.md:32-36` already records that class of addition as scoped elsewhere rather than here.
- **Conformance rule(s)**: `suffix_store_is_distinguishable_from_a_young_store` — **written and
  registered on the decide branch, deliberately not written on the refusal branch**. This is the
  one story in the project where "no rule" is a legitimate outcome, and it is legitimate only
  because CF-27's `Rule:` line keeps its `(new)`/`†` marker and the clause text records why. On the
  refusal branch this story adds no rule name to `crates/happenstance-testkit/src/registry.rs` and
  adds no entry to `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1978`) — that array is for a
  rule that *exists* and is claimed by nobody, which is the opposite of this situation.
- **Clause(s)**: **CF-27** (`spec/SPECIFICATION.md:8034-8060`, `[DEFERRED]`) — its `Rule:` line and,
  on the refusal branch, its body prose. **Its maturity marker is not moved here**; CF-27 and ES-39
  leaving `[DEFERRED]`, the `(new)` markers coming off the lines whose rules now exist,
  `cargo xtask spec-trace --write` regenerating §7.1/§7.2 and the hand reconciliation of §1.3's
  prose census are all `marker-moves-and-spec-trace-green`'s (project AC-014). ES-37 and ES-38
  (`[FROZEN]`) and ES-39 (`[DEFERRED]`) are cited and not amended; no `[FROZEN]` clause changes, so
  no new ADR is owed by this story.
- **Advances DoD scenario**: initiative **DoD 15** — *"Incomplete logs have an answer on disk … a
  store that holds only a suffix of its own log is exercised against a reader, and the reader
  either fails loudly or the refusal to define this is recorded as a decision"*
  (`.bklg/from-contract-to-published-library/initiative.md:402-405`). HS-S0121 put the decision on
  disk in `.kb/`; this story is where that answer reaches the **specification** — the artefact an
  adapter author reads — and with it exit criterion 5's first half. HS-P0019 re-observes DoD 15 as
  part of the assembled set.

## PR boundary

**In this PR**

Branch-independent:

- The recorded branch selection: ADR-0028's decision statement quoted, with the atom path, into
  this story's own backlog folder.
- `spec/SPECIFICATION.md` — **CF-27's clause body and/or its `Rule:` line only**, and only the
  edits the taken branch requires.
- This story's own backlog folder: `_ledger.md` and, at report stage, its implementation report.

Refusal branch additionally:

- CF-27's clause text records, in terms, why
  `suffix_store_is_distinguishable_from_a_young_store` cannot be written: that distinguishability
  is a *reported* property, that the port carries no completeness channel, that every candidate
  primitive is an ES-39 port surface, and what a reader is therefore on its own against — cited to
  the three recorded reader observations.
- CF-27's `Rule:` line **keeps** `(new)`/`†`, and the clause says why the marker is being kept
  rather than cleared, so the next reader does not read it as an oversight.

Decide branch additionally:

- `crates/happenstance-testkit/src/suite.rs` — the rule body, with rustdoc naming the wrong
  implementation it rejects.
- `crates/happenstance-testkit/src/registry.rs` — one line in `for_each_event_store_rule!`.
- `crates/happenstance-testkit/tests/mutation_coverage.rs` + `.../mutation_coverage/mutants.rs` —
  one `Defect` and one `REGISTRY` row with a non-empty `fails` list.
- `crates/happenstance-testkit/src/contract.rs` — **only if** the rule cannot be written without a
  seam, and then only DA-4's defaulted `Capability` const plus defaulted panicking method.
- The escalation finding handed to `surface-diff-and-the-ac-012-escalation`, if the decision
  adopted a primitive.

**Explicitly not in this PR**

- **Any marker move or census edit** — CF-27's or ES-39's `[DEFERRED]`, ES-40's `[PROVISIONAL]`,
  `cargo xtask spec-trace --write`, §7.1/§7.2, §1.3's prose census. All
  `marker-moves-and-spec-trace-green` (HS-S0123).
- **Any change to a port surface.** No new method, variant, type or signature in
  `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`; a decision
  that needs one is escalated and stopped (project AC-012, DA-7).
- **The escalation artefact itself and the `0.2.0` surface diff** —
  `surface-diff-and-the-ac-012-escalation` (HS-S0124).
- **Re-deciding ADR-0028, or editing any `.kb/` atom.** Accepted decision atoms are immutable; a
  correction is a new atom through `/redkiln:kb-ingest`, which is HS-S0121's mount, not this
  story's.
- **The instrument, the two owed rules and the reader observations** — all merged upstream
  (`retained-set-instrument-and-conformance-mount`, `positions-are-not-reused-after-removal`,
  `condition-over-removed-history-does-not-reject`, `decision-model-and-ingest-observed`,
  `projection-runner-across-the-hole`). This story consumes them and re-derives none.
- **`read_from_a_gap_position`'s ownership** and any other clause in the gap-read neighbourhood
  (`project.md:124-127`).
- **Adding anything to `UNCLAIMED_PENDING_ADR`** (`xtask/src/spec_trace.rs:1978`) — that array is a
  ratchet that can only shrink, and using it here would assert an open question that this story is
  closing.

**The implementer may also touch the mounts named in the Integration contract** — CF-27's clause in
`spec/SPECIFICATION.md` and, on the decide branch, `for_each_event_store_rule!` in
`crates/happenstance-testkit/src/registry.rs` — because that is how this slice is mounted, not
scope drift.

```
spec/SPECIFICATION.md
crates/happenstance-testkit/src/suite.rs
crates/happenstance-testkit/src/registry.rs
crates/happenstance-testkit/src/contract.rs
crates/happenstance-testkit/tests/**
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal/**
```

**Merge DoD.** ADR-0028's branch is quoted and executed; CF-27 no longer leaves the question to a
rule nobody wrote — either the rule exists in `suite.rs`, is registered exactly once, is green
against the honest instrument and red against a registered wrong implementation, or the clause text
records in writing why it cannot exist and what a reader is on its own against with its `(new)`/`†`
marker deliberately kept; no diff touches the public signatures in `crates/happenstance-core`; and
`cargo xtask spec-trace` and `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) are
green, with `cargo xtask ci --fast` (`:55`) green at the slice's integration grain.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The branch is read from ADR-0028, never chosen here | The accepted atom's decision statement is quoted verbatim into the record with its path, and the matching branch executed. Absence of the atom halts the story and is reported as a dependency failure, not worked around | `.bklg/.../_storymap.md:75-76`, `:146-149`; `project.md:219-222`; `_storymap.md:111-115` (the halt-don't-substitute rule this project already applies twice) |
| Both branches are gate-green, by different mechanisms | Decide: check 6 is satisfied because CF-27 already claims the rule name. Refuse: check 4 is satisfied because the `Rule:` line keeps `(new)`/`†`. Neither mechanism is evidence for the branch — the ADR is | `xtask/src/spec_trace.rs:725-727` and `:770-800` (check 6, `check_rule_ownership`); `:695-711` and `:1626-1631` (check 4 and the `schedules_new` predicate) |
| **Refusal** — the clause records why the rule cannot exist | Distinguishability is a *reported* property; `EventStore` exposes `read`, `append`, `head`, `contains_event_id` and no completeness channel; CF-27 itself makes the assertion depend on ES-39's primitive; every candidate primitive is a port-surface change forbidden by gate decision 4 | `spec/SPECIFICATION.md:8042-8044`; `crates/happenstance-core/src/store.rs:119`, `:213`, `:268`; `spec/SPECIFICATION.md:4325-4349`; `.bklg/from-contract-to-published-library/_decomposition.md:239-249` |
| **Refusal** — the clause records what a reader is on its own against | Three named readers, cited to the recorded observations rather than re-described: a conditional append admitted over a last-*retained* match; `holds()`/`contains_event_id` answering `false` for an event this store minted, so a peer re-sends forever; a projection resuming across the hole | `crates/happenstance-core/src/store.rs:321-331`; `crates/happenstance-sync/src/ingest.rs:164`; `crates/happenstance-core/src/store.rs:256-263` with `_decomposition.md:381-397` (DA-8); `_decomposition.md:113-141` (composition roots 4–6) |
| **Refusal** — the marker is kept **and** explained | `(new)`/`†` stays on the `Rule:` line so check 4 holds, and the clause says the marker is deliberate. A marker kept without that sentence is `SilentNonRule` — green forever, informative to nobody | `xtask/src/spec_trace.rs:1626-1631`; `discover.md:49`; `project.md:219-222` (AC-007's second half) |
| **Refusal** — nothing is added to the registry or to the pending-ADR array | No rule name enters `for_each_event_store_rule!`; `UNCLAIMED_PENDING_ADR` is untouched, because it exists for a rule that exists and is claimed by nobody — the inverse of this case, and its own doc calls it a list that can only shrink | `crates/happenstance-testkit/src/registry.rs:94`; `xtask/src/spec_trace.rs:1970-2008` |
| **Decide** — the body lives in `suite.rs` and nowhere else | Rule names resolve against `suite.rs` only; a body in a `tests/` file is invisible to check 4. Shape: `pub async fn <name><F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome`, `require!`-gated if it needs a capability, returning `RuleOutcome::Skipped` with the fixture's stated reason rather than being `#[cfg]`-ed out | `xtask/src/spec_trace.rs:700-711`; `crates/happenstance-testkit/src/suite.rs:15-42`; `crates/happenstance-testkit/src/contract.rs:31-42` |
| **Decide** — one registry line, feeding four emitters and every mutant | `for_each_event_store_rule!` is the single registration point; it drives `__emit_tokio`, `__emit_blocking`, `__emit_wasm`, `__emit_rule_names` and, via `for_each_mutant!`, every registered wrong implementation | `crates/happenstance-testkit/src/registry.rs:94`; `crates/happenstance-testkit/src/lib.rs:84-89`; `_decomposition.md:94-106` |
| **Decide** — the assertion is the one ADR-0028 fixes, and it reads only the port | The body may use `Fixture`'s declared surface and the `EventStore` port and nothing else: no method on the concrete instrument type, no downcast, no `Capability` standing in for a completeness report. What a store does not hold is a **fact** with no home on `Fixture` — that is ES-39's port primitive | `discover.md:36`, `:45` (`SelfReportingSuffixStore`); `_decomposition.md:299-305` (DA-4, trades versus facts); `crates/happenstance-testkit/src/contract.rs:44-54` |
| **Decide** — the rule can fail | Green against the honest instrument and against `MemoryFixture`; red against a registered wrong implementation — one `Defect` impl plus one `REGISTRY` row with a non-empty `fails` list, certified in both directions by the two meta-tests. A rule no adapter can fail is decorative | `crates/happenstance-testkit/tests/mutation_coverage.rs:324`; `_decomposition.md:107-112`; `CLAUDE.md` (*the rule that matters*, first corollary) |
| **Decide** — a fixture seam only if the rule cannot be written without one | DA-4's shape and no other: a **defaulted** `Capability` const defaulting to `Capability::declined(<reason>)` plus a **defaulted** method that panics naming both ways of reaching it. A **required** trait item breaks every `Fixture` impl and is project AC-011's named failure | `crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`; `_decomposition.md:276-308`; `project.md:319-325` |
| **Decide** — adopting a primitive escalates and stops | The finding names the surface, the version consequence (a `0.3.0` this initiative's exit criteria do not contemplate) and DA-7's four rows; the change is not made, and the check is that no diff touches the public signatures in `store.rs` or `append.rs` | `_decomposition.md:363-379` (DA-7), `:614`; `project.md:183-185`, `:240-243`; `.bklg/from-contract-to-published-library/_decomposition.md:239-249` |
| No assertion names a literal position value | The specification permits gaps; and the sharper form — *"history begins at position N"* — is `earliest_position()` reimplemented in a test, the shape ES-39 argues against. Compare against positions the store actually assigned; the first retained position differs between the suffix and scattered configurations | `CLAUDE.md` (*the rule that matters*); `spec/SPECIFICATION.md:4336-4342`; `_decomposition.md:150-171` (DA-1); `discover.md:68` |
| No marker moves, no census regeneration | This story edits CF-27's prose and, on the decide branch, adds a rule. `[DEFERRED]`/`[PROVISIONAL]` markers, `spec-trace --write`, §7.1/§7.2 and §1.3's hand census are HS-S0123's | `_decomposition.md:468-496` (*Gate mechanics*); `xtask/src/main.rs:671-674`; `_storymap.md:77` |
| Nothing `[FROZEN]` is amended | ES-37 (`:4274-4297`) and ES-38 (`:4299-4323`) are cited as binding; CF-27 and ES-39 are `[DEFERRED]`. A frozen clause moving would need a new ADR written first and a re-plan | `spec/SPECIFICATION.md:4274-4297`, `:4299-4323`; `project.md:112-114`; `CLAUDE.md` (*Open questions, deliberately unresolved*) |
| The gate this story is measured by | `cargo xtask affected --base main` at the story grain — which runs the five file-reading lints and `spec-trace` unconditionally, so a story that only edits `spec/SPECIFICATION.md` is still gated on something — and `cargo xtask ci --fast` at the slice's integration grain | `.redkiln/config.yaml:40`, `:55`; `_decomposition.md:620-640` |

## Data and migrations

**N/A — this story persists nothing and migrates nothing.** It writes no schema, no store, no
serialised format, and no on-disk state an application reads. Three adjacent facts are worth stating
so the absence is a determination rather than an omission:

- **The event log is not touched.** The instrument forgets by *hiding at the port* (DA-2,
  `_decomposition.md:173-233`) over a live inner `MemoryEventStore`; nothing in this story rebuilds,
  truncates or renumbers a store, and `MemoryEventStore::restore` — the rebuild path whose position
  renumbering is ES-38's registered mutant — is not called from any code this story adds.
- **The knowledge base is read-only here.** ADR-0028 arrives accepted and immutable from HS-S0121's
  ingest wave; this story quotes it and writes nothing under `.kb/`. The only governed mutation in
  this slice's neighbourhood already happened upstream.
- **Reversal is a single revert.** Both branches land as one commit touching at most CF-27's clause,
  one rule body, one registry line and one mutant row; reverting restores CF-27's text and the
  registry to their prior state with no data, no snapshot and no baseline to unwind. On the decide
  branch the only thing a revert must not strand is the escalation finding, which is why that
  finding is handed to `surface-diff-and-the-ac-012-escalation` rather than kept in the diff.

## Acceptance criteria

Eight criteria. **AC-001** and **AC-008** hold on both branches; **AC-002 – AC-004** are the decide
branch; **AC-005 – AC-007** are the refusal branch. A criterion for the branch **not** taken is
discharged in `_ledger.md` as `not-applicable — branch <A|B>` with ADR-0028's decision statement
quoted as its evidence — never silently dropped, because a dropped row is indistinguishable from a
row nobody got to (`discover.md:34`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The branch is read from ADR-0028, not chosen here.** GIVEN a maintainer who in two years wants to know *why* CF-27 ended the way it did, WHEN this story lands, THEN ADR-0028's decision statement is quoted **verbatim** into this story's record together with the accepted atom's `.kb/decisions/` path, the branch executed is stated as following from that quote, and no file under `.kb/` appears in this story's diff; AND if the accepted atom is not on disk when the story starts, the story **halts and reports a dependency failure** rather than inferring a branch from what the gate would tolerate. | Content review of the implementation report against `.bklg/.../_storymap.md:75-76`, `:146-149` and `project.md:219-222`; mechanical: `git diff --name-only main...HEAD` lists no `.kb/` path (accepted atoms are immutable, `CLAUDE.md` *Where the work lives*), and `redkiln validate --kb` is clean. |
| AC-002 | **Decide branch — the rule exists where a rule is allowed to exist, and runs everywhere rules run.** GIVEN an adapter author who runs `happenstance_testkit::event_store_conformance!` against their store, WHEN ADR-0028 decides the retention question, THEN `suffix_store_is_distinguishable_from_a_young_store` is a rule body in `crates/happenstance-testkit/src/suite.rs` (and in no `tests/` file) whose assertion is the one ADR-0028 fixes, registered **exactly once** in `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) so it reaches `__emit_tokio`, `__emit_blocking`, `__emit_wasm` and `__emit_rule_names`, and `cargo xtask spec-trace` check 6 passes because CF-27 already claims the name — with nothing added to `UNCLAIMED_PENDING_ADR`. | `cargo test -p happenstance-testkit --all-targets` — the rule name appears in every conformance target's output (`crates/happenstance-testkit/tests/memory_conformance.rs`, `memory_conformance_blocking.rs`, `memory_conformance_wasm.rs`); `cargo xtask spec-trace` (checks 4 and 6, `xtask/src/spec_trace.rs:695-711`, `:725-727`, `check_rule_ownership` `:765-800`); mechanical: exactly one occurrence of the rule name in `registry.rs`. |
| AC-003 | **Decide branch — the rule can be failed, and something in the tree fails it.** GIVEN a maintainer who must trust that a green suite means something, WHEN the rule ships, THEN it is green against `MemoryFixture` and against the honest completeness instrument mounted by `retained-set-instrument-and-conformance-mount`, and **red** against a registered wrong implementation — one `Defect` impl under `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` plus one `REGISTRY` row (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`) whose `fails` list names this rule and is non-empty — with `mutant_registry_is_exhaustive` and `mutants_fail_exactly_their_declared_rules` green in both directions and no parallel harness invented. | `cargo test -p happenstance-testkit --test mutation_coverage`; `cargo test -p happenstance-testkit --test memory_conformance`; the honest-instrument target landed upstream in the shape of `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205`. |
| AC-004 | **Decide branch — the rule asks the *port*, not the instrument.** GIVEN an application that holds only an `impl EventStore` — the reader CF-27's `Rejects:` is about — WHEN the rule runs, THEN its body uses only `Fixture`'s declared surface and the `EventStore` port: no method on a concrete instrument type, no downcast, and no `Capability` standing in as a completeness report; AND if it genuinely cannot be written without a fixture seam, that seam is DA-4's shape and nothing else — a **defaulted** `Capability` associated const defaulting to `Capability::declined(<reason>)` plus a **defaulted** method that panics naming both ways of reaching it — with no **required** trait item added. A rule that distinguishes the instrument by *asking the instrument* (`SelfReportingSuffixStore`) fails this criterion even with every gate green. | Adversarial review of the body against `discover.md:36`, `:45` and `_decomposition.md:299-305`; mechanical support: the body compiles inside the **library** crate, which cannot name a type defined in a `tests/` target, and `cargo test -p happenstance-testkit --test memory_conformance` exercises it through a fixture that hides nothing; any new `contract.rs` item is checked against the precedent at `crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307` and against project AC-011 via `cargo-semver-checks` at the PR grain. |
| AC-005 | **Refusal branch — CF-27 itself says why the rule cannot exist.** GIVEN an adapter author who opens CF-27 looking for the completeness instrument's teeth, WHEN ADR-0028 refuses, THEN **CF-27's own clause body** — in place, not a sibling clause, not an appendix, not a backlog note — states that distinguishability is a *reported* property; that `EventStore`'s surface (`read`, `append`, `head`, `contains_event_id`) carries no completeness channel; that the clause's own assertion is fixed by ES-39's primitive and no primitive has been chosen; and that every candidate primitive is a change to a surface published at `0.2.0`, which gate decision 4 forbids. | Content review against `project.md:219-222` and `spec/SPECIFICATION.md:8042-8044`, `:4325-4349`, `crates/happenstance-core/src/store.rs:119`, `:213`, `:268`; mechanical: the added prose lies inside CF-27's own clause range and `cargo xtask spec-trace` is green (`.redkiln/config.yaml:40`). |
| AC-006 | **Refusal branch — the clause names what a reader is therefore on its own against.** GIVEN a reader who will now build on a store that may have forgotten, WHEN they read CF-27, THEN the clause names three concrete readers with their **observed** outcomes cited to this project's recorded observations rather than predicted afresh: a conditional append admitted over a last-*retained* match from `read_decision_model` (`crates/happenstance-core/src/store.rs:321-331`) feeding `AppendCondition::after_opt`; `IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) and `EventStore::contains_event_id` answering `false` for an event this store minted, so a peer re-sends forever (DA-8); and the projection runner resuming across the hole with its actual recorded state. | Citation check: each of the three resolves to a real recorded observation from `decision-model-and-ingest-observed` / `projection-runner-across-the-hole` (project AC-006), not to prose invented here — the same bar `_decomposition.md:609`'s AC-006 row sets against "recording a *predicted* outcome in prose"; content review against `project.md:219-222` (AC-007's second half). |
| AC-007 | **Refusal branch — the marker is kept *and* explained, and nothing enters the registry.** GIVEN the next reader, who must be able to tell a deliberate non-rule from an oversight, WHEN the refusal lands, THEN CF-27's `Rule:` line keeps its `(new)`/`†` marker **and** the clause says in words that the marker is deliberate and why; AND no rule name is added to `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) and no entry is added to `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1978`), which exists for a rule that *exists* and is claimed by nobody — the inverse of this case, and a ratchet that can only shrink. This is the criterion `SilentNonRule` fails. | `cargo xtask spec-trace` (check 4, `xtask/src/spec_trace.rs:695-711` and the `schedules_new` predicate at `:1626-1631`) — green *because* the marker is present; mechanical: `git diff` shows `crates/happenstance-testkit/src/registry.rs` and `xtask/src/spec_trace.rs` untouched; content review for the deliberateness sentence against `discover.md:49`. |
| AC-008 | **Both branches — the `0.2.0` surface is not moved, the markers are not moved, and a decision that needs a surface escalates instead of landing.** GIVEN the maintainer of three crates published at `0.2.0`, WHEN this story lands on either branch, THEN no diff touches the public signatures in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`; CF-27's `[DEFERRED]` marker, ES-39's and ES-40's markers, §7.1/§7.2 and §1.3's prose census are **unchanged** (all HS-S0123's); and if ADR-0028's decision can only be executed by adopting one of DA-7's four primitives, the finding — naming the surface, the version consequence (a `0.3.0` this initiative's exit criteria do not contemplate) and which DA-7 row it is — is handed to `surface-diff-and-the-ac-012-escalation` and **the change is not made here**. | Mechanical: `git diff main...HEAD -- crates/happenstance-core/src/store.rs crates/happenstance-core/src/append.rs` shows no public-signature change (`_decomposition.md:614`); `git diff` on `spec/SPECIFICATION.md` shows no marker or census line changed; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) and `cargo xtask ci --fast` (`:55`) green; `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`). |

**Coverage of the traced project AC.** Project **AC-007** — *"`suffix_store_is_distinguishable_from_a_young_store` either exists with its assertion fixed by
the retention decision, or ADR-0028's refusal records in writing why it cannot exist and what a
reader is therefore on its own against"* (`project.md:219-222`) — is covered end to end: its first
half by AC-002 – AC-004, its second half by AC-005 – AC-007, and the branch selection that decides
which half applies by AC-001. AC-008 carries the project's DR-11/DR-12 guard (`project.md:181-185`)
so that "settled either way" cannot be bought with a port change.

## Interaction quality

RFC §6.7/D6. **Every invariant below is carried by an AC row in the table above** — this section
says which row carries it and how it is verified, and adds no criterion of its own.

**Composition family — discharged by sign-off, not by omission.** This project's `_design.md`
records `N/A — no user-facing surface` under *Surfaces*, with an empty `yaml` block and the framing
that *"the 'readers' involved are `read_decision_model`, a projection runner, and
`IngestStore::holds` — Rust code paths, not a screen a human looks at. No route, no DOM selector, no
component exists to enumerate."* That determination — not a screen — is what a human signed off on
2026-08-12, so there is no approved composition, transience policy, density budget or named
anti-pattern for this story to honour or contradict, and inventing one here would be re-deciding
design in a spec. The nearest thing this story renders is a **clause in
`spec/SPECIFICATION.md`**, whose composition is fixed by the shape `cargo xtask spec-trace` parses
(a maturity marker, a `Rule:` line, a `Rejects:` line, prose) and by the clauses around it — a
contract this story consumes and does not set.

**State family — the invariants that do apply, and the rows that carry them.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump.** The answer lands in the artefact the reader already has open — CF-27's own body — rather than in a sibling clause, an appendix, an ADR the reader must go find, or a backlog note. A refusal recorded somewhere else is `SilentNonRule` wearing a coat. | AC-005, AC-006 | The added prose lies inside CF-27's clause range in `spec/SPECIFICATION.md`; content review, plus `cargo xtask spec-trace` green over the edited file. |
| **Non-occlusion.** The refusal text is added to CF-27 without displacing what the clause already carries: its `Rule:` line, its `Rejects:` line and its maturity marker all survive verbatim. | AC-007, AC-008 | `git diff` on `spec/SPECIFICATION.md` shows additions inside CF-27 and no deletion of the `Rule:`/`Rejects:`/marker lines; `spec-trace` check 4 depends on the `Rule:` line's marker surviving. |
| **Preserved position — the reader's other entry points still land.** §7.1/§7.2's generated tables and §1.3's prose census are the index a reader arrives through; this story leaves all three byte-identical so no cross-reference shifts under a reader mid-navigation. Regenerating them is HS-S0123's, deliberately. | AC-008 | `git diff` shows no §7.1/§7.2/§1.3 line changed; `cargo xtask spec-trace` (checks 8 and 9) green without `--write`. |
| **Reversibility.** Either branch is a single revert: at most CF-27's clause body, one rule body, one registry line and one mutant row. Nothing this story writes needs unwinding in a second place — which is why a decide-branch escalation is *handed to* HS-S0124 rather than parked in this diff. | AC-001, AC-008 | Revert-in-one-commit is structural (see *Data and migrations*); mechanical check is that the diff touches only the paths enumerated in *PR boundary*. |
| **Reachability of the failure.** On the decide branch the rule must be reachable from every path a reader of the suite uses — all four emitters and every registered mutant — because a body present in `suite.rs` but absent from `for_each_event_store_rule!` runs nowhere and is unreachable in the only sense this repository recognises. | AC-002, AC-003 | `cargo test -p happenstance-testkit --all-targets`; the rule name present in `__emit_rule_names`' output and in the tokio, blocking and wasm targets. |
| **Legibility of a deliberate absence.** A reader must be able to distinguish "no rule because we decided none can exist" from "no rule because nobody got to it". The marker alone cannot carry that distinction; the sentence beside it is what does. | AC-007 | Content review against `discover.md:49`; the marker itself is checked by `spec-trace` check 4 (`xtask/src/spec_trace.rs:1626-1631`). |

Keyboard reachability, focus/scroll/selection preservation and revealed-versus-persistent chrome
have no referent in a Markdown specification clause and a `cargo test` run; they are recorded here as
inapplicable for this story rather than passed over.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | **ADR-0028 is not on disk under `.kb/decisions/` when this story starts.** | Halt and report a dependency failure on `adr-0028-and-the-open-question-wave` (HS-S0121). Do not infer a branch, do not write a provisional refusal, do not draft the atom here — an accepted atom is authored through `/redkiln:kb-ingest` and hand-writing one is the mistake `CLAUDE.md` records the revert `0269720` for. Same discipline `_storymap.md:111-115` applies to the projection runner and the `0.2.0` baseline. |
| EC-002 | **ADR-0028 exists but its decision statement does not resolve this rule's assertion** — it answers what a store may forget without saying how a store says so, or it defers the reporting half. | Treat it as the **refusal branch for this rule** and say so explicitly in CF-27's text, quoting the ADR's own words for the deferral: the rule cannot be written because the primitive it needs was deferred, not because the question was answered "no". Do not infer a primitive from the ADR's silence — that is `QuietPrimitive` (`discover.md:50`) reached by reading rather than by coding. |
| EC-003 | **ADR-0028 decides by adopting a primitive** — one of DA-7's four rows, or any other change to `EventStore`/`AppendCondition`. | Raise the AC-012 escalation (surface, version consequence, DA-7 row) to `surface-diff-and-the-ac-012-escalation` and **stop**. Do not add the method, variant or type. `project.md:183-185`: a conclusion that a surface change is required is a finding, not a licence. AC-008's `git diff` check is the mechanical proof it was not taken. |
| EC-004 | **The rule can be written, but only by asking the instrument** — a concrete-type accessor, a downcast, or a `Capability` used as a completeness report. | Do not write it. That is `SelfReportingSuffixStore` (`discover.md:45`): it satisfies CF-27's *name* and none of its meaning, and it answers ES-39 in a place no application can read (`_decomposition.md:299-305`). The honest outcome is the refusal branch plus, if the decision genuinely intended a primitive, EC-003's escalation. |
| EC-005 | **The rule is written and nothing in the tree fails it.** | It is decorative and must not merge in that state (`CLAUDE.md`, *the rule that matters*, first corollary). Either name and build the plausible wrong implementation as a `Defect` + `REGISTRY` row (AC-003), or conclude the assertion has no content and take the refusal branch with that finding recorded. |
| EC-006 | **`spec-trace` check 6 fails**: a rule name is in the registry that no clause claims. | Do not reach for `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1978`). That array is a shrinking ratchet for rules awaiting a decision, and this story's whole job is that the decision now exists. Either CF-27 claims the name (it already does, so this failure means the registered name diverged from the clause's spelling) or the rule does not belong in the registry. |
| EC-007 | **`spec-trace` check 4 fails on the refusal branch**: CF-27 names a rule that does not exist and the `Rule:` line lost its `(new)`/`†`. | Restore the marker — the marker is the mechanism that makes the refusal legal (`xtask/src/spec_trace.rs:1626-1631`) — and confirm the deliberateness sentence is present. Never satisfy the check by deleting the `Rule:` line: that erases the question CF-27 exists to pose. |
| EC-008 | **The reading requires a `[FROZEN]` clause to move** — ES-37 (`spec/SPECIFICATION.md:4274-4297`) or ES-38 (`:4299-4323`). | Stop and re-plan. A new decision atom is written first, per `project.md:112-114` and `CLAUDE.md`'s *Changing a `[FROZEN]` clause requires a new ADR, not an edit*. Reopen `discover.md:68`'s frozen-clause box rather than treating it as covering the case. |
| EC-009 | **The honest instrument or an upstream reader observation is missing** when the decide branch needs it green, or when the refusal branch needs it cited. | Halt on the upstream story (`retained-set-instrument-and-conformance-mount`, `decision-model-and-ingest-observed`, `projection-runner-across-the-hole`) rather than constructing a stand-in: a hazard demonstrated against a reader nobody ships proves nothing (`_storymap.md`, closing notes). |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| NF-001 | **No `#[async_trait]`, and nothing that implies `+ Send`.** A decide-branch rule body runs in `__emit_wasm` as well as tokio and blocking, so it must compile and hold on the `!Send` flavour. Rule bodies take the existing shape `pub async fn <name><F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome`. | `CLAUDE.md` constraint 1 and [ADR-0001](../../../../.kb/decisions/0001-async-port-flavours.md); checked by `cargo xtask ci --fast`'s four `wasm32` steps and `crates/happenstance-testkit/tests/memory_conformance_wasm.rs`. |
| NF-002 | **A declined capability skips loudly, never disappears.** If the decide branch needs a capability gate, it uses `require!` and returns `RuleOutcome::Skipped` carrying the fixture's stated reason — not `#[cfg]`. | `crates/happenstance-testkit/src/suite.rs:37-42`; `crates/happenstance-testkit/src/contract.rs:31-42` rejects the `#[cfg]` arrangement by name. |
| NF-003 | **No assertion names a literal position value**, and no assertion is phrased as *"the history begins at position N"* — that is `earliest_position()` reimplemented in a test, the shape ES-39 argues against. Compare against positions the store actually assigned. | `CLAUDE.md`, *the rule that matters*; `spec/SPECIFICATION.md:4336-4342`; `_decomposition.md:150-171` (DA-1); `discover.md:68`. |
| NF-004 | **Deterministic and bounded.** A new rule runs in every emitter against every registered mutant, so its cost is multiplied; it must use no wall-clock timing, no sleeps and no unbounded reads, in keeping with the suite's existing bodies. | `cargo test -p happenstance-testkit --all-targets` runtime observed against the pre-change baseline; the multiplication is structural in `crates/happenstance-testkit/src/registry.rs:94` and `for_each_mutant!`. |
| NF-005 | **Rustdoc obligation.** A decide-branch rule body documents the wrong implementation it rejects, in the shape the rest of `suite.rs` uses; the refusal branch's equivalent obligation is CF-27's own prose. | `standards/rust/70-rustdoc-obligations.md`; `cargo xtask ci --fast`'s doc build. |
| NF-006 | **Additive-only public surface.** On the decide branch the only new `pub` items are the rule function and, if unavoidable, a defaulted `Fixture` const/method — `0.2.x`-class. Enumeration of those items is `surface-diff-and-the-ac-012-escalation`'s (project AC-011); this story's obligation is to add nothing outside that class. | `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`); `crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`. |
| NF-007 | **MSRV unchanged at 1.97.1.** Nothing here needs a newer feature; moving the floor is an ADR, not a side effect. | `CLAUDE.md` binding constraint 5 / [ADR-0029](../../../../.kb/decisions/0029-msrv-raised-to-1-97-1.md); CI's `msrv` job. |
| NF-008 | **The knowledge base is written by the CLI, not by this story.** ADR-0028 is read-only; no `.kb/` file is edited, and no item frontmatter under `.bklg/` is touched. | `CLAUDE.md` *Where the work lives*; `redkiln validate --kb` and `redkiln doctor` clean. |

## Implementation notes (non-prescriptive)

Not a recipe — the shape of the ground, so the implementer spends their judgement on the assertion
rather than on the mechanics.

- **Read the atom before anything else.** The first artefact this story produces is the quoted
  decision statement with its path (AC-001). Producing it first makes the branch a fact in the
  record rather than a conclusion the diff implies.
- **Write the refusal text as if the reader has nothing else.** The failure mode is not a missing
  argument, it is a *compressed* one: "the port has no completeness channel" is true and useless on
  its own. The three reader observations are what make the consequence concrete, and each is cited
  to a recorded actual value rather than re-argued — the same bar `_decomposition.md:609` sets for
  project AC-006 against writing down a predicted outcome.
- **If you find yourself wanting a new method, you have found EC-003.** That is the *expected*
  escalation (DA-3(3), `_decomposition.md:268-270`), named in advance so reaching it is a decision
  and not a discovery. Write the finding, hand it to HS-S0124, stop.
- **On the decide branch, land the mutant with the rule, in the same change.** Not because a
  convention says so, but because the mutant is how you learn whether the assertion has content:
  a rule you cannot write a plausible wrong implementation for is a rule with no content (EC-005).
- **Check the registry line by counting, not by eye.** One occurrence of the rule name in
  `crates/happenstance-testkit/src/registry.rs`; a body in `suite.rs` without it compiles, passes
  review, and runs nowhere.
- **Do not tidy the markers while you are in the file.** `spec/SPECIFICATION.md` will be full of
  `[DEFERRED]` markers and `(new)` lines that are about to move — all of them HS-S0123's, which
  merges *after* this story precisely so it can see this story's outcome. Moving one here creates a
  conflict in the slice's second PR and destroys the ordering's whole reason.
- **`git diff` is a checking instrument here, not just a review courtesy.** Three of this story's
  criteria (AC-001's `.kb/` untouched, AC-007's registry untouched, AC-008's core signatures and
  markers untouched) are proved by the absence of a change, and absence is easiest to prove by
  looking at the diff deliberately rather than by remembering not to.

## Tests and CI (merge gate)

Grounded in the project testing brief (`_decomposition.md`, *Test mix* row **AC-007** at `:609`, and
*Merge-gate commands* at `:620-640`). This project ships tests, not an application, so there is no UI
e2e tier; the nearest equivalent is the conformance suite run in full through
`event_store_conformance!`.

| tier | command / path | proves |
| --- | --- | --- |
| static — spec gate | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) | Check 4 (`:695-711`, `:1626-1631`): on the refusal branch CF-27 may name a rule that does not exist **because** its `Rule:` line keeps `(new)`/`†`. Check 6 (`:725-727`, `check_rule_ownership` `:765-800`): on the decide branch the registered rule is claimed by CF-27. Checks 8/9: §7.1/§7.2 unchanged. AC-002, AC-005, AC-007, AC-008. |
| static — diff assertions | `git diff main...HEAD -- crates/happenstance-core/src/store.rs crates/happenstance-core/src/append.rs`; `git diff --name-only main...HEAD` | No public-signature change in `happenstance-core` (`_decomposition.md:614`); no `.kb/` path in the diff; on the refusal branch no change to `crates/happenstance-testkit/src/registry.rs`. AC-001, AC-007, AC-008. |
| unit — registry and mutants (decide branch) | `cargo test -p happenstance-testkit --test mutation_coverage` | `mutant_registry_is_exhaustive` and `mutants_fail_exactly_their_declared_rules` green in both directions over the new `Defect` + `REGISTRY` row (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`) — the rule is red against a registered wrong implementation and nothing else is collaterally red. AC-003. |
| integration — the conformance suite, all emitters (decide branch) | `cargo test -p happenstance-testkit --all-targets`; `crates/happenstance-testkit/tests/memory_conformance.rs`, `memory_conformance_blocking.rs`, `memory_conformance_wasm.rs`, and the honest instrument's target in the shape of `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` | The rule reaches `__emit_tokio`, `__emit_blocking` and `__emit_wasm`, is green against `MemoryFixture` and against the honest instrument, and reads only the port. AC-002, AC-003, AC-004. |
| static — semver | `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`) | Every addition is additive `0.2.x`-class; no **required** `Fixture` trait item (project AC-011's named failure). AC-004, AC-008. |
| content review | CF-27's clause body read against `project.md:219-222`, `discover.md:45`, `:49`, `_decomposition.md:299-305` | The refusal states why the rule cannot exist, what a reader is on its own against, and that the kept marker is deliberate — i.e. that it is not `SilentNonRule`; and on the decide branch that the assertion is not `SelfReportingSuffixStore`. AC-004, AC-005, AC-006, AC-007. |
| story grain (fires on stage transition) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | Maps the diff to packages plus dependents and runs fmt / clippy `-D warnings` / tests for that set, **and** runs the five file-reading lints and `spec-trace` unconditionally — so a story that only edits `spec/SPECIFICATION.md` is still gated on something. All ACs. |
| integration grain, cheap tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | The slice's fast gate, including *"no retired rule is still live"* (`xtask/src/main.rs:342`). AC-002, AC-007. |
| integration grain, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The bar this project's Definition of done names: fmt, clippy, tests, the four mandatory `wasm32` steps, `spec-trace`, doc builds. `cargo xtask ci` (the whole gate) is HS-P0019's, not this story's. AC-002 – AC-004, AC-008. |
| process, by hand | `redkiln validate --kb && redkiln doctor` | The accepted ADR-0028 atom is unmodified and the KB is conformant — this story reads it and writes nothing under `.kb/`. AC-001. |

## Risks and coupling (PR-scoped)

- **Hard dependency on HS-S0121, and it is a halt rather than a workaround.** Without the accepted
  atom there is no branch to execute; EC-001 is the required behaviour. The tempting failure is to
  write the refusal text "because it's obviously the refusal branch" — `discover.md:33` does predict
  that outcome, and the prediction is exactly what must not be substituted for the decision.
- **Slice ordering is load-bearing in both directions.** `marker-moves-and-spec-trace-green` merges
  *after* this story because a marker can only move once the rule it names exists or is recorded as
  never-to-exist (`_storymap.md:146-149`); this story therefore must not move a marker, and HS-S0123
  must not start from an assumption about which branch was taken. Both edit
  `spec/SPECIFICATION.md`, so a marker touched here is also a merge conflict there.
- **The decide branch couples this story to HS-S0124.** The escalation finding leaves this diff and
  lands in `surface-diff-and-the-ac-012-escalation`. If it is instead kept locally, a revert of this
  story strands it — which is why *Data and migrations* calls that out as the one thing a revert must
  not lose.
- **Two green-but-wrong branches, and the gate cannot tell them apart.** `SilentNonRule` (refusal,
  marker kept, nothing recorded) and `SelfReportingSuffixStore` (decide, rule green because it asks
  the instrument) both pass every mechanical check in this repository. AC-004 – AC-007 are content
  criteria for that reason, and the review that reads them is the only instrument that catches them.
- **A new rule's blast radius is the whole suite.** A decide-branch rule runs against every fixture
  and every registered mutant, so it can turn a currently-green adapter red — which
  `crates/happenstance-testkit/Cargo.toml`'s own text says is the mechanism working, not a
  violation. It still needs to be observed rather than discovered by someone else, so the mutation
  meta-tests are run in both directions (AC-003).
- **`UNCLAIMED_PENDING_ADR` is the wrong escape hatch and looks like the right one.** It is a
  shrinking ratchet for rules awaiting a decision (`xtask/src/spec_trace.rs:1970-2008`); using it
  here would assert an open question at the exact moment the decision closed it. EC-006.
- **Consumed upstream work, re-derived nowhere.** The instrument, the two owed rules and the three
  reader observations all arrive from merged siblings. If any is absent, EC-009 halts; a synthetic
  stand-in would make this story's central citations fictional.

## Dependencies

**Blocks on**

- `adr-0028-and-the-open-question-wave` (HS-S0121) — the accepted ADR-0028 atom under
  `.kb/decisions/`, authored through `.kb/_intake/` and `/redkiln:kb-ingest`. Its decision statement
  *is* this story's branch selector (AC-001); absent, EC-001 halts.

Also consumed, merged earlier in the project and not re-derived here:
`retained-set-instrument-and-conformance-mount` (the honest instrument and its
`event_store_conformance!` mount, AC-003's green target), `decision-model-and-ingest-observed` and
`projection-runner-across-the-hole` (the three recorded reader observations AC-006 cites),
`positions-are-not-reused-after-removal` and `condition-over-removed-history-does-not-reject` (the
registry and mutant shapes AC-002/AC-003 follow).

**Unlocks**

- `marker-moves-and-spec-trace-green` (HS-S0123) — CF-27's and ES-39's markers can only move once
  this story has made the rule exist or recorded it as never-to-exist.
- `surface-diff-and-the-ac-012-escalation` (HS-S0124) — receives this story's escalation finding on
  the decide branch, and enumerates any additive `pub` item this story added on it (project AC-011).

Merge order within `clause-exit-and-surface-record`: this story → `marker-moves-and-spec-trace-green`
→ `surface-diff-and-the-ac-012-escalation` (`_storymap.md:146-149`).

## Anchors (progressive disclosure)

Link, do not paste. The Context pack above is sufficient to start; open these at the moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/maps/decision-map.md` | The index that resolves **ADR-0028** to its atom path once HS-S0121's ingest wave lands; the atom itself is this story's branch selector and is immutable. | First, before any edit — the branch cannot be chosen, only read. If ADR-0028 is not indexed here or under `.kb/decisions/`, EC-001 halts the story. | AC-001 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | AC-007's exact wording (`:219-222`) — *"and what a reader is therefore on its own against"* is the half a paraphrase loses — plus DR-11/DR-12 (`:181-185`), the finding-not-licence rule. | Before writing the refusal text, and again before signing off AC-008. | AC-005, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | The briefs: DA-3(3) (`:268-270`, the expected escalation), DA-4 (`:276-308`, trades versus facts and the defaulted seam), DA-7 (`:363-379`, the four pre-computed primitives), DA-8 (`:381-397`, the `contains_event_id` lie), *Gate mechanics* (`:468-496`), the testing row for AC-007 (`:609`), the mechanical no-surface check (`:614`). | DA-3/DA-7 when the decide branch is indicated; DA-4 before adding any `Fixture` item; DA-8 while writing AC-006's second reader. | AC-003, AC-004, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | Gate decision 4 (`:239-249`) — retention's answer is constrained to what needs no published-surface change. This is what makes the refusal branch reachable and the decide branch an escalation. | Before concluding that a primitive may be adopted. | AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal/discover.md` | The three named wrong implementations — `SelfReportingSuffixStore` (`:45`), `SilentNonRule` (`:49`), `QuietPrimitive` (`:50`) — and the literal-position judgement (`:68`). These are the failure modes the content criteria exist to catch. | Before writing the rule body (decide) or the refusal text (refuse); again at self-review. | AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md` | The slice's merge order (`:146-149`) and the halt-don't-substitute precedent (`:111-115`) this story inherits for a missing dependency. | At the start, to confirm HS-S0123 has not already moved markers; and on any missing upstream artefact. | AC-001 |
| `spec/SPECIFICATION.md` | CF-27 itself (`:8034-8060`) — the mount, its `Rule:` line and its `Rejects:`; ES-39 (`:4325-4349`) for the primitive the assertion waits on and the scattered-purge argument against a floor (`:4336-4342`); ES-37 (`:4274-4297`) and ES-38 (`:4299-4323`) as `[FROZEN]` and cited only. | Immediately before editing the clause; re-read `:4336-4342` before phrasing any assertion about where history begins. | AC-005, AC-006, AC-007 |
| `xtask/src/spec_trace.rs` | The gate's own logic: check 4 (`:695-711`) and the `(new)`/`†` predicate (`:1626-1631`) that makes the refusal legal; check 6 (`:725-727`) and `check_rule_ownership` (`:765-800`); `UNCLAIMED_PENDING_ADR` (`:1978`) and why it is not available here. | When either check fails, and before touching anything that looks like a marker. | AC-002, AC-007 |
| `crates/happenstance-testkit/src/suite.rs` | The only file a rule body may live in, the body shape, and the `require!` gate (`:37-42`); also the projection hazard already written down at `:1490-1531`. | On the decide branch, before writing the body. | AC-002, AC-004 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!` at `:94` — the single registration point feeding all four emitters and, via `for_each_mutant!`, every mutant. A body absent from it runs nowhere. | On the decide branch, in the same change as the body; on the refusal branch, only to confirm it is untouched. | AC-002, AC-007 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture`, `Capability`, `RuleOutcome`; capabilities are trades and limits are facts (`:44-54`); the `#[cfg]` rejection (`:31-42`); the defaulted-seam precedent (`:207-211`, `:297-307`). | Only if the decide branch appears to need a fixture seam — and read `:44-54` before concluding it does. | AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `REGISTRY` at `:324` and the two meta-tests that certify a rule can fail, in both directions. | On the decide branch, alongside the rule body. | AC-003 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | The mount shape (`:201-205`) the honest instrument's target follows — the target a decide-branch rule must be green against. | On the decide branch, when wiring the rule against the instrument. | AC-003 |
| `crates/happenstance-core/src/store.rs` | The port's whole surface — `read` (`:119`), `append` (`:213`), `contains_event_id` (`:268`) — the evidence that no completeness channel exists; and `read_decision_model` (`:321-331`), AC-006's first reader. **Read-only**; its public signatures must not change. | While writing the refusal's "why", and again for AC-008's diff check. | AC-005, AC-006, AC-008 |
| `crates/happenstance-sync/src/ingest.rs` | `IngestStore::holds` (`:164`) — AC-006's second reader, the one where `false` means *"never had it"* and a peer re-sends forever. | While writing AC-006's second citation. | AC-006 |
| `.redkiln/config.yaml` | The `verify:` block: story grain `:40`, cheap tripwire `:48`, non-terminal integration grain `:55` — the commands that fire on stage transition whether or not anyone types them. | Before claiming the merge gate is green. | AC-008 |
| `CLAUDE.md` | *The rule that matters* and its first corollary (a rule no adapter can fail is decorative), the no-literal-positions instruction, and *Where the work lives* (accepted atoms immutable; the CLI is the only writer of item frontmatter). | Before adding a rule without a mutant, and before touching anything under `.kb/`. | AC-001, AC-003 |
| `RUNBOOK.md` | `:517` — the lifecycle row that already recognises *"an explicit written refusal … is a legitimate answer"*; `:4626-4674`, phase 14 in full, including the "reported as a rule name, not 'conformance failed'" bar. | When justifying that the refusal branch is a completed deliverable rather than a deferral. | AC-005 |

## Clarifications resolved during spec

1. **The eight AC ids are branch-labelled, and the untaken branch's rows are discharged, not
   dropped.** AC-001 and AC-008 hold on both branches; AC-002 – AC-004 are the decide branch and
   AC-005 – AC-007 the refusal branch. `redkiln verify --grain story` requires every spec AC to be
   present and `satisfied: true` with non-placeholder evidence, so the untaken branch's rows are
   flipped with `not-applicable — branch <A|B>` **plus ADR-0028's quoted decision statement and the
   atom's path** as their evidence. That is a real, checkable citation rather than a placeholder, and
   it keeps the record of what was considered — which `discover.md:34` asks for explicitly. No AC id
   was added or dropped relative to the first pass.
2. **"Which branch?" is deliberately unanswered here.** The discover stage predicted the refusal
   branch and gave its reasoning (`discover.md:33`, `:41`), and this spec repeats that reasoning
   under *Context pack* — but a prediction is not the decision. AC-001 makes the quoted ADR the
   selector, and EC-002 covers the awkward middle case where ADR-0028 answers the forgetting
   question without answering the reporting one.
3. **The composition family of RFC §6.7/D6 is discharged by the signed-off `_design.md`, not
   waived.** `_design.md`'s *Surfaces* section records `N/A — no user-facing surface` with an empty
   `yaml` block, and that determination is what a human approved on 2026-08-12. There is therefore
   no approved composition, density budget or named anti-pattern for this story to honour; the
   *Interaction quality* section says so in those terms and maps the state-family invariants that do
   apply onto AC rows, rather than inventing a design contract in a spec.
4. **The state-family invariants were translated to this story's actual medium.** A specification
   clause and a `cargo test` run have no focus, scroll or keyboard model, so those are recorded as
   inapplicable; what survives translation — in-place versus context jump, non-occlusion,
   preserved position for the reader's other entry points, reversibility, reachability of the
   failure, and the legibility of a deliberate absence — is carried by AC-005 – AC-008 and by
   AC-002/AC-003.
5. **The verification of AC-004 is partly structural and partly human, and the split is stated.**
   A downcast to the honest instrument's concrete type is impossible by the crate graph — `suite.rs`
   is in the library crate and the instrument lives in a `tests/` target downstream of it — so the
   residual risk is a `Capability` used as a completeness report, which no compiler can catch. That
   half is an explicit review criterion against `_decomposition.md:299-305`, not a test.
6. **ADR-0028's atom path is cited by index, not by guessed filename.** `.kb/decisions/0028-<slug>.md`
   does not exist yet, so the anchor table points at `.kb/maps/decision-map.md` — a file that does
   exist — as the resolver. Citing a guessed path would have made the story's first instruction a
   dead link.
7. **Nothing here reopens the frozen-clause box.** `discover.md:68` asks that the box be re-opened if
   the spec stage concludes a published-surface change is required. This spec concludes no such
   thing: it specifies both branches and routes the surface-requiring outcome to EC-003's escalation,
   which is a finding raised to HS-S0124 rather than a change made here.
