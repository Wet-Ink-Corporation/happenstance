---
item: HS-S0105
stage: discover
created: 2026-08-12T13:03:25.964Z
updated: 2026-08-12T13:03:25.964Z
template_sig: 86ce4036
rendered_sig: b68508f1
---

# Discover — The two headline rules, green, with mutants that fail them by name

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: land the rules the frozen clauses already name — `ingest_never_rejects` (SY-1), `compensation_is_atomic_with_the_losing_event` (SY-2), `wire_condition_with_after_is_refused` (SY-6) — green against `MemorySyncPeer`, each with a compiled wrong peer in the testkit's own `tests/` and a `Declared` mutant entry carrying a never-empty provenance naming the real peer shape that makes it plausible | `_storymap.md`, *Slices* table, `sync-conformance-suite` row 3 | Three rules, three compiled wrong peers, three registry rows. The rules are not invented here — the clauses name them |
| **Depends on `sync-testkit-crate-and-rule-registry` (HS-S0103)** for the registry, the fixture contract and the round-trip counter | `_storymap.md`, *Slices* `depends_on` | A rule needs somewhere to be enumerated and something to be handed |
| **Depends on `gate-mounts-for-the-sync-suite` (HS-S0104)** for `RULE_FILES`, the position-literal lint scope and the changelog-per-rule lint | `_storymap.md`, *Slices* `depends_on`; `_storymap.md`, *Slices* note | *"The suite is not landed until the gate runs it and a mutant fails it"* — the ordering is deliberate, not cosmetic |
| **Depends on `adr-0027-merge-compensation-and-message-set` (HS-S0099)** for the compensation contract these rules assert | `_storymap.md`, *Slices* `depends_on`; `RUNBOOK.md:306` | `compensation_is_atomic_with_the_losing_event` asserts a contract; the contract must be decided first |
| **AC-008** — the two headline rules pass, *each with a mutant that fails it by name* | `project.md`, *Acceptance criteria*, AC-008; `RUNBOOK.md:4611-4612` | The RUNBOOK's exit criterion names both rules verbatim. This is the phase's own bar |
| **AC-002** — the central question's answer made mechanical | `project.md`, *Acceptance criteria*, AC-002; `_storymap.md`, *Coverage* AC-002 row | The rules are how SY-1 and SY-6 stop being prose |
| **AC-003** — rules present in output, declined capabilities reported with the fixture's stated reason | `project.md`, *Acceptance criteria*, AC-003 | Rules must be visible in the run even when a capability is declined |
| **AC-004** — every mutant carries a non-empty provenance naming the real peer shape that makes it plausible, and the suite decodes no payload byte on any path | `project.md`, *Acceptance criteria*, AC-004; `spec/SPECIFICATION.md:7186-7215` (CF-2 – CF-4) | Provenance is a `const` field, not a comment |
| **SY-1 `[FROZEN]`**, rule and mutant both already written for us: *"two peers each accept a conflicting fact under byte-identical position-free conditions; assert both facts are present in both logs after a full exchange"*; `Rejects:` *an ingest that calls `append(events, Some(&origin_condition))` and routes `AppendError::ConditionViolated` into a rejection path* | `spec/SPECIFICATION.md:5841-5867` | The rule's assertion **and** the mutant's shape are dictated by the clause. Do not re-derive either |
| SY-1's own reasoning for why the mutant is plausible: *"It is the natural first cut, because the condition arrives on the wire already… It passes every event-store conformance rule, because it is one correct `append` call. It produces a peer set that never converges, and nothing in the workspace today can observe that."* | `spec/SPECIFICATION.md:5861-5867` | This paragraph *is* the provenance string. CF-4's "never empty" is satisfied by a real argument, not a label |
| **SY-2 `[FROZEN]`** — the compensation MUST be appended in the same `append` call as the losing event; `Rejects:` compensate-after-commit, whose window *"survives the window closing, because the slice does not"* | `spec/SPECIFICATION.md:5871-5900` | The store-side half rides on ES-18; the sync-side half is this rule |
| **SY-6 `[FROZEN]`** — a guard whose `after` is `Some(_)` MUST be refused as ingest input, with two independently fatal defects; the refusal is *mechanically available today* because `Guard` is `#[non_exhaustive]` with public fields while `AppendCondition::guards` is private and readable only through `guards()` | `spec/SPECIFICATION.md:5973-6029` | Readable-but-not-literal-constructible is what makes the refusal checkable. The rule tests a refusal, not a rejection of an event |
| **CF-1** — every rule MUST be paired with at least one mutant in the testkit's own `tests/` that fails it; a rule introduced without one *"is a defect in the suite and MUST NOT be merged"* | `spec/SPECIFICATION.md:7161-7185` | The clause's own worked example is two rules that were decorative for four phases |
| **CF-2 / CF-3** — mutants registered as **data**, a `const` table naming per mutant the exact set of rules it fails, and the meta-test asserts **both** directions: every mutant fails every rule it declares and passes every rule it does not | `spec/SPECIFICATION.md:7186-7207` | *"The second assertion is what keeps a mutant a scalpel."* A mutant broken in more ways than it claims is the commonest decay |
| **CF-4** — non-empty provenance naming the real adapter shape; `Rejects:` *the saboteur*, `struct AlwaysWrong`, which *"satisfies CF-1 mechanically and proves nothing, because no author would have written it"* | `spec/SPECIFICATION.md:7208-7215` | The bar is "the mutant that earns its place is the one someone would ship" |
| The existing `Declared` table shape: `name`, `kind`, `fails: &'static [&'static str]`, never-empty `provenance`, `mode`, and per-rule `expect` pins naming *the exact assertion* the mutant should trip | `crates/happenstance-testkit/tests/mutation_coverage.rs:140-180` | Copy the shape. The `expect` pins exist because *"`FailureMode` says the rule rejected the store; it does not say which of the rule's assertions did the rejecting"* |
| Worked provenance strings to match in register and specificity | `crates/happenstance-testkit/tests/mutation_coverage.rs:325-350` | e.g. `GappedPositionStore`'s three-clause justification, and `PagedStreamStore`'s "any store with a network under it" |
| **DR-4** — the suite is an instrument, not a decoration: every sync rule carries a registered wrong implementation that fails it and passes everything it does not | `project.md`, *Derived requirements*, DR-4 | Restated at project grain because it is the commonest failure mode in this repository's own history |
| The standing corollary: *"a rule that no adapter can fail is decorative. Before adding one, name a plausible wrong implementation it rejects, and write that implementation into the testkit's own `tests/` if one does not already exist there."* | `CLAUDE.md`, *The rule that matters* | Non-negotiable, and the reason this story lands three rules rather than thirty-five |
| The mutant peer must not reuse `MemorySyncPeer`'s own dispatch to fake its bug; the oracle must be spelled in a direction sharing no subroutine with the implementation | `standards/rust/60-what-a-test-must-prove.md` RS-60-3, RS-60-4; `_decomposition.md`, testing brief *Unit* | A mutant built by wrapping the oracle tests the wrapper |
| CF-6 / DR-7 — no rule asserts on a literal position value; every position assertion is anchored on a value the store under test assigned | `spec/SPECIFICATION.md:7233-7249`; `project.md`, DR-7 | Load-bearing here in the strong form: the interesting positions in these rules are foreign ones |
| Never report a mutant pass rate as a fraction — *"the denominator is a choice"* | `spec/SPECIFICATION.md:7251-7256` | Report which defects the set covers and which axes it leaves uncovered |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered, and
this story is the answer's enforcement.** No. `ingest_never_rejects` is SY-1's
own rule and its assertion is dictated by the clause: two peers each accept a
conflicting fact under **byte-identical, position-free** conditions, and after a
full exchange both facts are present in **both** logs
(`spec/SPECIFICATION.md:5848-5855`). Note what the clause's assertion is *not*:
it is not "the ingest returned `Ok`". A peer can return `Ok` and drop the event,
or return `Ok` and win the wrong side of the conflict. The assertion is on the
final contents of both logs, keyed by identity.

**Hub-and-spoke versus peer-to-peer — not this story's.** ADR-0027 owns the
question; SY-9's `one_adapter_serves_both_roles` is
`hub-and-spoke-and-peer-to-peer-topologies`' (HS-S0110). These three rules each
take exactly one peer handle, per SY-8 — *"a rule that needed two would be
testing the runner"* (`spec/SPECIFICATION.md:6074-6077`). SY-1's own rule text
mentions "two peers", which is a scenario the fixture arranges, not a second
handle on the port.

**Why only three rules when thirty-five clauses schedule them? — Answered:
because each rule owes a compiled wrong peer, and this story is where that price
is first paid.** CF-1 makes a rule without a mutant unmergeable and CLAUDE.md
makes it a discipline. Three rules with three mutants is a slice; thirty-five
rules with thirty-five mutants is a project. The remaining rules land with the
stories that need them — the wire rules with `message-set-on-the-envelope`, the
topology rule with HS-S0110, the round-trip and resume rules with the live peers.

**Is `wire_condition_with_after_is_refused` a *third* headline rule or scope
creep? — Answered: it is required and it is not creep.** AC-002 names SY-1 *and*
SY-6 as the pair the central question is reconciled against, and SY-6 is the half
that has a mechanically checkable refusal available today
(`spec/SPECIFICATION.md:6023-6029`). Landing SY-1's rule without SY-6's would
leave the answer half-enforced: the receiver would be forbidden from *evaluating*
a condition while still being handed an unreplicable one.

**What does "refused" mean for SY-6 — an error, a panic, a filtered field? —
Deferred to spec, and it is ADR-0026's `SyncError` question wearing a rule's
clothes.** The clause says the condition *"MUST be refused as ingest input"*; it
does not say by which channel. `WireError` is a decoding failure and is not it
(`crates/happenstance-sync/src/lib.rs:91-94`).

**Does the mutant registry live in `tests/` or `src/`? — Deferred to spec, with
the precedent noted.** The event-store registry is
`crates/happenstance-testkit/tests/mutation_coverage.rs`, and CF-33's `TESTKIT_SRC`
scope is `src/` precisely because `tests/` holds the deliberately-wrong
implementations (`xtask/src/lints.rs:33-42`). Following the precedent keeps the
lint scopes HS-S0104 sets meaningful.

## Decision

Three `[FROZEN]` clauses name three rules that do not exist —
`ingest_never_rejects`, `compensation_is_atomic_with_the_losing_event` and
`wire_condition_with_after_is_refused` — and each of the three also names, in its
`Rejects:` field and in full prose, the wrong implementation it exists to reject.
So the specification has already done the design work twice over and the gap is
purely that nothing compiles it: SY-1's rejected shape (`append(events,
Some(&origin_condition))` with `ConditionViolated` routed into a rejection path)
is *"the natural first cut"* that *"passes every event-store conformance rule"*
and produces a peer set that never converges, and *"nothing in the workspace
today can observe that"*. This story makes the workspace able to observe it: the
three rules, green against `MemorySyncPeer`, each paired with a compiled wrong
peer in `happenstance-sync-testkit`'s own `tests/` and a `Declared` registry row
naming the exact set of rules it fails, the exact assertion it should trip, and a
provenance string taken from the clause's own argument for why an author would
ship it. The spec stage will cover: each rule's assertion, written from the
clause rather than from `MemorySyncPeer`'s implementation; the three wrong peers
and why each is plausible; the `Declared` table and its `expect` pins; the
`CHANGELOG.md` entry per rule naming a defect (CF-29); the declined-capability
path for each rule; and the reporting discipline — which defects the set covers
and which axes it leaves uncovered, never a fraction.

## The wrong implementation

**`ConditionEvaluatingIngest`, in `crates/happenstance-sync-testkit/tests/` —
SY-1's own named target, and the reason this rule exists.** The receiver reads
`EventGroup::guard`, calls `append(events, Some(&origin_condition))` on its local
store, and maps `AppendError::ConditionViolated` into a rejection. It is one
correct `append` call, so it passes every rule in the event-store suite. It
returns clean errors, so it looks well-engineered. It is reachable *because the
field is public*: the crate's prose says the guard is evidence rather than an
instruction, and *"prose is not a type constraint: `EventGroup::guard` is a
**public** `Option<AppendCondition>` field on the wire type… so a receiver is
handed exactly the value it would need to re-evaluate"*
(`spec/SPECIFICATION.md:5856-5867`; `crates/happenstance-sync/src/peer.rs:236-243`).
What it produces is a peer set that never converges, because rejection is a
function of local state and different peers therefore reject different events.
Its registry row declares `ingest_never_rejects`; its provenance is the clause's
paragraph.

**`PositionPortingPeer` — the mutant this whole project is named after, and the
one a counting rule cannot see.** It accepts the `after`-carrying guard instead of
refusing it, and evaluates it in its own numbering. On any test where both stores
were appended to in lockstep it is *correct*. On any realistic test it passes
**vacuously**: `after: 288455` interpreted locally names an unrelated recent
event, the condition scans an arbitrary tail and succeeds, and the ingest looks
enforced. *"That is worse than no check, because it looks like enforcement"*
(`spec/SPECIFICATION.md:6015-6021`). Its sibling defect is worse still and needs
no `after` at all: at `after: None`, re-delivery matches the receiver's **own**
already-accepted copy, returns `ConditionViolated`, and the receiver adjudicates
against the event it just accepted — an inversion rather than a duplicate, which
is E2E-33 and *"the sharpest single case in the catalogue"*. SY-11 states the
same failure from the other side: *"A duplicate is visible; an inversion looks
like a decision"* (`spec/SPECIFICATION.md:6144-6152`). It declares
`wire_condition_with_after_is_refused`, and its provenance is that every
log-shipping protocol in the world resumes from a scalar offset.

**`CompensateAfterCommit`** — ingest the losing event, return `Ack`, let a
projection or a follow-up command author the compensation. Every event arrives,
both counts are right, the log converges eventually, and any rule that asserts on
the final state of a quiesced store is green. It declares
`compensation_is_atomic_with_the_losing_event`, and the rule that catches it must
observe the log **between** the two appends — which means the assertion is on
what a reader can see mid-sequence, not on what the log holds at rest
(`spec/SPECIFICATION.md:5876-5880`).

**And the meta-mutant: a rule that no peer can fail.** The failure mode the
repository has already lived through — CF-1's own worked example is
`positions_are_unique` and `positions_are_strictly_monotonic`, which *"read a
quiescent store back through `read`, which every adapter returns in position
order, so both were satisfied by sorting on the way out"* and had gone four
phases without a store that could fail them
(`spec/SPECIFICATION.md:7167-7185`). The sync-shaped version of that mistake is a
rule asserting `receiver.len() == sender.len()` after an exchange: it is green
against every peer above, including `ConditionEvaluatingIngest` on a
non-conflicting fixture, `FlatteningIngest`, and a peer that silently reorders.
The guard is CF-3's second direction — every mutant must **pass** every rule it
does not declare — plus the standing instruction to name the plausible wrong peer
*before* writing the rule.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

**Box 6, and this is the story where it does the most work.** All three rules
involve positions and none may assert on one. `ingest_never_rejects` asserts that
both *facts* are present in both logs, keyed on `EventId`, never on where they
landed. `compensation_is_atomic_with_the_losing_event` asserts adjacency of the
losing event and its compensation **relative to each other and to a head the
receiving store reported**, never at literal offsets.
`wire_condition_with_after_is_refused` constructs an `after` value from a
position the *origin* store actually assigned, so the refusal is exercised with a
real foreign position rather than a magic integer — and the refusal is asserted
on the returned error, not on any position at all. Two enforcement layers back
this: `lint-position-literals`, extended to this crate by HS-S0104, and the
conformant-variant discipline CF-5 and CF-6 establish
(`spec/SPECIFICATION.md:7222-7249`).

**Box 7.** This story edits no `[FROZEN]` clause. It implements three of them,
and it is the story that turns SY-1's, SY-2's and SY-6's `Rule:` fields from
`(new, happenstance-sync-testkit)` into names that resolve — but **dropping the
`(new)` markers is `frozen-clause-repairs`' (HS-S0112)**, not this story's, and
that separation is deliberate: the marker comes off only once the rule exists and
the gate can see it. The decisions that authorise the rules' shape, ADR-0026 and
ADR-0027, are both written first and are this story's transitive and direct
`depends_on` edges. If a clause's named rule turns out to be unimplementable as
written, the output is a recorded finding and a repair in the same change, never
a rule bent to fit.
