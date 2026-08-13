---
item: HS-S0105
stage: spec
created: 2026-08-12T13:47:45.092Z
updated: 2026-08-12T13:47:45.092Z
template_sig: 87bbf1d0
rendered_sig: 50b93ed1
---

# Spec — The two headline rules, green, with mutants that fail them by name

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project (charter) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-002, AC-003, AC-004, AC-008; DR-4, DR-5, DR-7 |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/headline-rules-and-mutant-registry/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — *architecture: Composition root* §3 (the mutant registry's existing table shape) and §6 (the gate mounts this story depends on); *testing: The test mix, tier by tier* (Unit), *Fixtures and seams to mock* (the `guard` row and the round-trip-counter row) |
| Story map row | `…/replication-identity-and-ingest/_storymap.md`, *Slices*, `sync-conformance-suite` row 3; merge order at *Merge order* 3 |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved by the repository owner 2026-08-12, `design.capture` a declared skip. There is no surface id to render; the API-surface obligation is discharged in the architecture brief and here |
| Discover stage (this story) | `…/headline-rules-and-mutant-registry/discover.md` — the signal ledger, the three wrong peers by name, and *Gate: Discover* box 6 on why positions are the sharp edge here |
| Slice-mate specs | `…/sync-testkit-crate-and-rule-registry/spec.md` (HS-S0103 — the fixture contract and registry this story fills), `…/gate-mounts-for-the-sync-suite/spec.md` (HS-S0104 — the lints that watch it) |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13); this story is exit criterion `RUNBOOK.md:4611-4612`, which names both headline rules verbatim |

## One-line PR slice

Land the rules the frozen clauses already name — `ingest_never_rejects` (SY-1),
`compensation_is_atomic_with_the_losing_event` (SY-2) and `wire_condition_with_after_is_refused` (SY-6)
— green against `MemorySyncPeer`, each with a compiled wrong peer in the testkit's own `tests/` and a
`Declared` mutant entry carrying a never-empty provenance naming the real peer shape that makes it
plausible.

## Executive summary

**This PR is where the sync suite starts measuring, and where it first becomes possible to be wrong.**

HS-S0103 built the instrument and deliberately put no rule in it: `for_each_sync_peer_rule!` expands to an
empty enumeration and `no_orphan_sync_rules` is green and vacuous. HS-S0104 taught the gate's single-crate
constants about the new crate, so `RULE_FILES`, the position-literal lint and the changelog-per-rule lint
now watch a directory with nothing in it. This story puts the first three rules there, and with them the
first mutant registry the sync axis has ever had.

The delta is small in files and large in what it settles. Three `[FROZEN]` clauses have named these rules
since phase 2 and none of them exists, which is why `crates/happenstance-sync/src/lib.rs:113-115` can still
say idempotent ingest is *"Not yet checked by anything"*. Each of the three clauses also writes the rule's
assertion **and** its wrong implementation for us, in the clause's own `Rule:` and `Rejects:` fields
(`spec/SPECIFICATION.md:5848-5867`, `:5886-5900`, `:5981-6029`). So the design work here is not deciding
what the rules assert. It is three narrower things, and each has a way to be got wrong that this repository
has already lived through:

1. **Where a rule reaches the receiving side's ingest path at all.** `IngestStore::ingest` takes
   `&[ReplicatedEvent]` and no guard (`crates/happenstance-sync/src/ingest.rs:150`), and `MemorySyncPeer`
   *"is a peer, not a store: it holds whatever was pushed to it"*
   (`crates/happenstance-sync/src/memory.rs:1-6`). Nothing in the tree today turns a `PushBatch` into local
   writes. That seam is the runner's in production and the runner is HS-S0109's — so this story adds it to
   the **fixture**, not to the port.
2. **Keeping each mutant a scalpel.** CF-3 requires every mutant to fail every rule it declares *and pass
   every rule it does not* (`spec/SPECIFICATION.md:7200-7207`). Two of the three wrong peers here are one
   careless line away from failing two rules each, and the careless line is the *obvious* one.
3. **Never asserting a position.** All three rules are about positions that came from somewhere else. CF-6
   is enforced behaviourally by a conformant variant and only secondarily by a grep
   (`spec/SPECIFICATION.md:7233-7249`).

What exists afterwards is the first sync-side answer that is mechanical rather than written: SY-1's
*"nothing in the workspace today can observe that"* stops being true.

## Context pack

The decisions this story must honour, stated as decisions. Everything deeper is behind a signposted anchor.

**1. The three rules are not designed here; they are compiled here.** Each clause supplies the assertion
and the target:

- **SY-1** `[FROZEN]` — *two peers each accept a conflicting fact under byte-identical **position-free**
  conditions; assert both facts are present in **both** logs after a full exchange*
  (`spec/SPECIFICATION.md:5848-5855`). Note what the assertion is **not**: it is not "the ingest returned
  `Ok`". A peer can return `Ok` and drop the event, or win the wrong side of the conflict. The assertion is
  on the final contents of both logs, keyed by `EventId`.
- **SY-2** `[FROZEN]` — the compensation MUST be appended in the **same `append` call** as the losing
  event, and *"a reader MUST NOT be able to observe a state in which the log holds the losing event with
  nothing resolving it"* (`spec/SPECIFICATION.md:5871-5900`). The store-side half rides on ES-18 and is not
  this rule's; the sync-side half is.
- **SY-6** `[FROZEN]` — a wire-carried condition is evidence, not an instruction, and any guard whose
  `after` is `Some(_)` MUST be **refused as ingest input** (`spec/SPECIFICATION.md:5973-6029`).

Re-deriving any of the three from `MemorySyncPeer`'s behaviour is the failure `RS-60-4` names
(`standards/rust/60-what-a-test-must-prove.md`): an oracle that shares a subroutine with the implementation
tests the subroutine. Write each assertion from the clause, then check it against the oracle — in that
order.

**2. The rules drive one fixture-supplied *apply* seam, and that seam is this story's one addition to the
slice-mate's trait.** All three clauses constrain what a receiver does when it turns a `PushBatch` into
local writes. That code path exists nowhere in the tree: `IngestStore::ingest` accepts
`&[ReplicatedEvent]` with no guard and no group structure
(`crates/happenstance-sync/src/ingest.rs:150`), and `PushBatch`'s own doc says flattening it is the defect
`EventGroup` exists to prevent (`crates/happenstance-sync/src/peer.rs:183-190`, `PushBatch`'s own doc). So the receiving side must
be reached through **one required method on `SyncPeerFixture`** — apply a batch to `Self::Local` the way
this adapter's real deployment does — added here, in the same slice, to the trait HS-S0103 wrote.

Two alternatives were weighed and lost. *Wait for the runner (HS-S0109) and drive the rules through it*:
the rules would then be testing the runner, which SY-8 forbids in as many words — *"a rule that needed two
[peer handles] would be testing the runner"* (`spec/SPECIFICATION.md:6074-6077`) — and the slice would
invert. *Let the rule play the receiver itself*, calling `ingest` directly and authoring the compensation:
the rule then asserts against its own code, and every peer in the world passes.

**3. `compensation_is_atomic_with_the_losing_event` is observed *between* the two writes, not at rest.**
`CompensateAfterCommit` — ingest the losing event, return, let a projection or a follow-up command author
the compensation — leaves a log that converges eventually and passes any rule that reads a quiesced store.
The rule catches it by reading the local store **at the point the apply call returns and before anything
else runs**: a conformant receiver has already written both events in one `append`, a compensate-after
receiver has written one. This is a deterministic sequencing, not a race and not a timing window — no rule
may read a clock (CF-33) and none here needs to.

**4. Domain adjudication is a declared `Capability`, and it is the declined-capability path this story owes
project AC-003.** SY-2 is conditional: *"where the receiving peer's domain determines that an ingested
event conflicts with a fact it already holds"*. A fixture whose deployment has no adjudicating domain
honestly has nothing to demonstrate, so it declines — and the rule still emits, still runs, and reports the
fixture's **stated reason** rather than vanishing from the binary
(`crates/happenstance-testkit/src/contract.rs:31-42`). A `Capability` is an associated `const` and
therefore cannot mean "the environment was down today"; that distinction was settled by HS-S0103's Context
pack §5 and is inherited, not re-decided.

**5. SY-6's refusal is asserted as a refusal, never as an error variant.** The clause says the condition
*"MUST be refused as ingest input"* and does not say through which channel. `WireError` is a decoding
failure and is explicitly not it (`crates/happenstance-sync/src/lib.rs:91-94`), and how much `SyncError`
grows is ADR-0026's, named as ADR-0026's in the crate itself. So the rule asserts two observable facts and
no third: the apply call returns `Err`, and **nothing was appended** — measured against a head the
receiving store itself reported. A rule that matched on an error variant would pre-empt a decision that
belongs to an accepted atom, and would fail every adapter whose error enum is its own.

**6. That refusal is not the content disagreement SY-1 forbids, and the port's own doc comment currently
reads as though it might be.** `SyncPeer::push`'s rustdoc says *"Transport- or authorisation-level refusals
only. A peer may not refuse a push because it disagrees with the events in it"*
(`crates/happenstance-sync/src/peer.rs:144-148`). Refusing an `after`-carrying guard is not disagreement
with an event: it is refusal of a **structurally unreplicable condition**, and the events in that group are
untouched facts that remain durable at the origin either way. The two clauses are consistent and the doc
comment is one clarifying sentence short of saying so. **This story does not edit it** —
`crates/happenstance-sync/` is outside the PR boundary — it records the finding for
`frozen-clause-repairs` (HS-S0112), which owns every repair whose `Rejects:` or prose names a symbol this
project changed.

**7. Each mutant must be a scalpel, and the obvious version of two of them is not.** CF-3's second
direction — *"every registered mutant … passes every rule it does not [declare]"* — is what keeps a mutant
diagnostic (`spec/SPECIFICATION.md:7200-7207`). Concretely, and this is the paragraph to re-read before
writing the wrong peers:

- **`ConditionEvaluatingIngest`** evaluates the origin's guard as a local precondition and routes
  `AppendError::ConditionViolated` into a rejection. If it evaluates *every* guard it is handed, it also
  accepts and evaluates an `after`-carrying one and therefore fails `wire_condition_with_after_is_refused`
  as well — two declared failures, one of them accidental. It must **refuse `after`-carrying guards
  correctly** and evaluate only position-free ones. That is not a softening of the mutant: SY-1's own rule
  text specifies *position-free* conditions, so the mutant is being made to fail exactly the clause it
  targets.
- **`PositionPortingPeer`** accepts the `after`-carrying guard and evaluates it in its own numbering. If it
  also evaluates position-free guards it fails `ingest_never_rejects` too. It must evaluate **only** guards
  carrying `after: Some(_)` and leave position-free ones alone.

Both constraints are CF-3 in practice, and both are the kind of thing a reviewer notices after the
meta-test has been green for a phase.

**8. The mutant registry is data in the shape that already exists, in `tests/`, not `src/`.** The
event-store registry is a `const` table of `Declared { name, kind, fails, provenance, mode, expect }`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:140-186`), and the per-rule `expect` pins exist
because *"`FailureMode` says the rule rejected the store; it does not say which of the rule's assertions
did the rejecting"*. Copy that shape. Keeping it under `tests/` is what keeps HS-S0104's lint scopes
meaningful: CF-33's `TESTKIT_SRC` is `src/` **deliberately**, because `tests/` is where the
deliberately-wrong implementations live (`xtask/src/lints.rs:32-42`).

**9. Provenance is an argument, not a label.** CF-4 rejects *the saboteur* — `struct AlwaysWrong` — because
*"no author would have written it"*, and sets the bar at *"the mutant that earns its place is the one
someone would ship"* (`spec/SPECIFICATION.md:7208-7215`). Each of the three clauses has already written the
argument: SY-1's *"the natural first cut, because the condition arrives on the wire already… It passes
every event-store conformance rule, because it is one correct `append` call"*; SY-6's *"worse than no
check, because it looks like enforcement"*; SY-2's device that *"cuts its next slice from a hub log that
says one physical compressor is held twice"*. Take the provenance from the clause's argument, at the
register and specificity of the worked strings at
`crates/happenstance-testkit/tests/mutation_coverage.rs:325-350`.

**10. No literal position, in the strong form, because every interesting position here is a foreign one.**
CF-6 (`spec/SPECIFICATION.md:7233-7249`) and DR-7. `ingest_never_rejects` asserts on `EventId` membership
and never on where an event landed. `compensation_is_atomic_with_the_losing_event` asserts adjacency
relative to a head the receiving store reported. `wire_condition_with_after_is_refused` builds its `after`
from a position the **origin** store actually assigned, so the refusal is exercised with a real foreign
position rather than a magic integer. The behavioural enforcement is CF-5's conformant variants — which
HS-S0103 already wrote and which these rules must not break — and the lint HS-S0104 mounted is the cheap
second line, in that order.

**11. The suite decodes no payload byte, on any path.** DR-5 and SY-35 `[FROZEN]`: everything replication
reasons about is in the `EventType` or the `Tags`. The three rules distinguish facts by `EventId`, event
type and tags; a conflicting fact is *conflicting* because the fixture's domain says so, never because a
rule parsed `Event::data`. A suite that parsed a payload would certify a peer that does, which is the
guarantee ADR-0003 exists to buy (`RUNBOOK.md:4576-4580`), and the byte-identity round trip that lifts its
`provisional` marker is HS-S0107's and depends on this staying true.

**12. This story adds three rules and not thirty-five, and the reason is a price, not a schedule.** CF-1
makes a rule without a mutant unmergeable and CLAUDE.md makes it a standing discipline: *name a plausible
wrong implementation it rejects, and write that implementation into the testkit's own `tests/`*. Three
rules with three compiled wrong peers is a slice; thirty-five with thirty-five is a project. The remaining
`SY`/`WF` rules land with the stories that need them — the wire rules with `message-set-on-the-envelope`
(HS-S0106), the topology rule with `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110), the resume and
round-trip rules with `durable-object-and-neon-peers` (HS-S0111).

**13. The `(new)` markers stay on, and no `[FROZEN]` clause is touched.** `xtask/src/spec_trace.rs` treats
a `Rule:` line containing `(new)` as *scheduled*, so SY-1, SY-2 and SY-6 keep rendering as unwritten until
someone removes the marker. That removal is `frozen-clause-repairs`' (HS-S0112) and the separation is
deliberate: the marker comes off once the rule exists **and the gate can see it**. `spec/SPECIFICATION.md`
is therefore not in this story's PR boundary at all.

**14. The persona-journey slice.** The reader is the **adapter author** on *Learn when you are finished*
(`.bklg/from-contract-to-published-library/initiative.md:241-248`), and this is the moment the suite starts
answering. Before this PR `sync_peer_conformance!(MyFixture::new())` runs and reports zero rules — a green
result that means nothing. After it, the same invocation reports three rules by name, each either passing,
failing with the assertion that failed, or skipped with the fixture's own stated reason. That is the first
half of *A3 — Prove a peer conforms* becoming true (`…/_storymap.md`, *Backbone*).

## Integration contract

- **Archetype**: `capability` — a slice a reader can observe end to end: an adapter author runs the suite
  and gets three named verdicts about their peer.
- **Slice / milestone**: `sync-conformance-suite`. Slice-mates, implemented in one context and mounted as
  one integrated surface: `sync-testkit-crate-and-rule-registry` (HS-S0103) and
  `gate-mounts-for-the-sync-suite` (HS-S0104). Merge order within the slice is HS-S0103 → HS-S0104 → **this
  story** (`…/_storymap.md`, *Merge order* 3). Both are `blocked_by` edges in this story's frontmatter,
  along with `adr-0027-merge-compensation-and-message-set` (HS-S0099), which decides the compensation
  contract `compensation_is_atomic_with_the_losing_event` asserts.
- **Mount point**: `crates/happenstance-sync-testkit/src/registry.rs` — the `for_each_sync_peer_rule!`
  enumeration, which is *the only place the rule set is written down* and the single expansion three
  harnesses share (the sync analogue of `crates/happenstance-testkit/src/registry.rs:68-102`). A rule
  function that exists in `rules.rs` but is absent from that enumeration is compiled, unreachable and
  green — the library form of the component nobody imported — and `no_orphan_sync_rules` is what makes that
  a build failure rather than a silence. Adding each rule to the enumeration is therefore the mount, and it
  is what makes the rule appear in all three harnesses at
  `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs`, `…_blocking.rs` and `…_wasm.rs`
  without any harness being edited.
- **Wires into**:
  - `crates/happenstance-sync-testkit/src/rules.rs` — where the three rule functions live, in the shape
    `pub async fn <rule><F: SyncPeerFixture>(open: impl AsyncFn() -> F) -> RuleOutcome`, mirroring
    `crates/happenstance-testkit/src/suite.rs:210`;
  - `crates/happenstance-sync-testkit/src/contract.rs` — `SyncPeerFixture`, which this story extends by one
    required apply method and one `Capability` (Context pack §2, §4);
  - `crates/happenstance-sync-testkit/src/fixtures.rs` — `MemorySyncPeerFixture`, the oracle these rules
    must be green against, and the reference implementation an adapter author copies;
  - `crates/happenstance-testkit/src/contract.rs` — `Capability` and `RuleOutcome`, consumed through the
    sibling crate, never redeclared;
  - `crates/happenstance-sync/src/peer.rs` — `SyncPeer`, `PushBatch`, `EventGroup` and its **public**
    `guard: Option<AppendCondition>` field, `Ack`, `Pulled`;
  - `crates/happenstance-sync/src/ingest.rs` — `IngestStore::{ingest, holds, watermark}` and `Ingested`;
  - `crates/happenstance-core/src/append.rs` — `AppendCondition::guards()` and `Guard`'s public `after`,
    which is what makes SY-6's refusal readable-but-not-literal-constructible
    (`spec/SPECIFICATION.md:6023-6029`);
  - `CHANGELOG.md` — one entry per rule naming the defect it detects (CF-29), read by
    `xtask/src/lints.rs:525` over the `RULE_FILES` set HS-S0104 extended.
- **Renders surfaces**: **none.** `…/replication-identity-and-ingest/_design.md` records a no-surface
  determination, approved 2026-08-12, with `design.capture` a declared skip. There is no surface id for
  this story to claim. The public items it adds carry the repository-wide rustdoc obligation
  (`standards/rust/70-rustdoc-obligations.md`) instead.
- **Public items** (in place of `_design.md`'s `## Items`, recorded N/A for this project):
  `happenstance_sync_testkit::rules::{ingest_never_rejects, compensation_is_atomic_with_the_losing_event,
  wire_condition_with_after_is_refused}`; the extension of `SyncPeerFixture` by its apply method and its
  `DOMAIN_ADJUDICATION` capability constant; and the enlarged `for_each_sync_peer_rule!` enumeration. The
  three wrong peers and the `Declared` table are `tests/`-private on purpose.
- **Conformance rule(s)**: the three named above — **added by this story**, which is the whole story. Each
  is the rule its clause's `Rule:` field already names, at the name the clause uses. Plus the sync-side
  mutant meta-tests in the shape of `crates/happenstance-testkit/tests/mutation_coverage.rs`:
  `every_rule_has_a_mutant` (CF-1), `mutant_registry_is_exhaustive` (CF-2),
  `mutants_fail_exactly_their_declared_rules` (CF-3), `every_mutant_states_its_provenance` (CF-4) and
  `conformant_variants_pass_everything` (CF-5/CF-6) over the variants HS-S0103 wrote.
- **Clause(s)**: **discharges** SY-1 (`spec/SPECIFICATION.md:5841-5867`), SY-2 (`:5871-5900`) and SY-6
  (`:5973-6029`) by supplying the rule each names, and satisfies CF-1 – CF-6 and CF-29 for those three
  rules. **Amends none, edits none.** It conforms by construction to SY-8 (one peer handle per rule), SY-11
  (re-delivery is a no-op, relied on by SY-1's full-exchange assertion) and SY-35 (no payload decoded). The
  `(new)` marker removal on SY-1/SY-2/SY-6 and the `push` doc-comment clarification of Context pack §6 are
  both `frozen-clause-repairs`' (HS-S0112) and are recorded here for it to find.
- **Advances DoD scenario**: project **DoD 4** — *"the proof artefact exists, and it would not exist if the
  design were wrong"* — this is the first PR in which the suite can return a verdict other than "no rules
  ran", and project **AC-008** is `RUNBOOK.md:4611-4612`'s own exit criterion. Through it, initiative
  **DoD 14** (*"Replication has an answer on disk"*,
  `.bklg/from-contract-to-published-library/initiative.md:398-401`): ADR-0026 writes the answer down and
  these three rules are what make it observable rather than asserted. Initiative **DoD 13** (*"the gate is
  green on the assembled whole"*) is a not-regressed obligation — three new rules must not turn
  `cargo xtask ci --fast` red, and CF-29's changelog lint is the one most likely to catch a missing entry.

## PR boundary

```
crates/happenstance-sync-testkit/src/**
crates/happenstance-sync-testkit/tests/**
.bklg/from-contract-to-published-library/replication-identity-and-ingest/headline-rules-and-mutant-registry/**
CHANGELOG.md
```

**In this PR**

- Three rule functions in `crates/happenstance-sync-testkit/src/rules.rs`, each written from its clause:
  `ingest_never_rejects`, `compensation_is_atomic_with_the_losing_event`,
  `wire_condition_with_after_is_refused`.
- All three added to `for_each_sync_peer_rule!` in `src/registry.rs` — the mount, and the reason they
  appear in the tokio, blocking and `wasm32` harnesses without a harness being edited.
- The one required apply method on `SyncPeerFixture` (`src/contract.rs`), its rustdoc, and the
  `DOMAIN_ADJUDICATION` capability constant; `MemorySyncPeerFixture` (`src/fixtures.rs`) updated to
  implement it.
- Three compiled wrong peers in `crates/happenstance-sync-testkit/tests/` — `ConditionEvaluatingIngest`,
  `PositionPortingPeer`, `CompensateAfterCommit` — each a real shape someone would ship, none of them built
  by wrapping `MemorySyncPeer`'s dispatch.
- The `Declared` mutant registry and its five meta-tests, in the shape of
  `crates/happenstance-testkit/tests/mutation_coverage.rs`, including the per-rule `expect` pins.
- Three `CHANGELOG.md` entries, one per rule, each naming the defect that rule detects (CF-29).

**Explicitly not in this PR**

- **Any further `SY`/`WF` rule.** The wire rules are `message-set-on-the-envelope`'s (HS-S0106), the
  topology rule `hub-and-spoke-and-peer-to-peer-topologies`' (HS-S0110), the resume, round-trip and limit
  rules `durable-object-and-neon-peers`' (HS-S0111). Each owes its own mutant at the same price.
- **Any edit to `spec/SPECIFICATION.md`**, including dropping the `(new)` markers from SY-1/SY-2/SY-6's
  `Rule:` lines and the `push` doc-comment finding of Context pack §6. Both are HS-S0112's — hence no
  `spec/**` in the boundary above.
- **Any change to `crates/happenstance-sync/`.** If a rule cannot be written without one, that is a finding
  to raise against ADR-0026, not a widened boundary — the port is what these rules exist to hold still.
- **Any `xtask/**` change.** `RULE_FILES`, the position-literal lint scope and the changelog lint scope are
  HS-S0104's and are already in the tree when this story starts; this story is the first content those
  lints see.
- **Anything under `.kb/`.** Atoms arrive only through `/redkiln:kb-ingest`.
- **A mutant pass rate.** Never a fraction — the denominator is a choice
  (`spec/SPECIFICATION.md:7251-7256`).

The implementer **may** additionally touch the composition-root and contract files named in the Integration
contract where mounting requires it — `src/registry.rs`, `src/contract.rs` and `src/fixtures.rs` are inside
the boundary precisely because a rule that is not enumerated, or that has no seam to drive, is not mounted.
Extending a slice-mate's trait is mounting and not scope drift; extending the **port** is drift, and the
boundary above is drawn so that the difference is visible in `git diff` rather than argued afterwards.

**Merge DoD**: `cargo xtask affected --base main` and `cargo xtask lints && cargo xtask spec-trace` green;
`cargo test -p happenstance-sync-testkit --all-features` green with all three rules named in the output of
all three harnesses and the five meta-tests passing; `cargo xtask ci --fast` green on the tree;
`_ledger.md` cites evidence per AC-###.

## Behavior and interfaces

The **binding** parts of the shape below are the ones carrying a citation — the assertions, the refusal
being asserted as a refusal, the mutants' declared sets, the provenance bar, the position discipline. Names
and the exact decomposition are the implementer's, and a deviation is legitimate when it is recorded with
its reason.

```rust
// crates/happenstance-sync-testkit/src/contract.rs — the one addition to HS-S0103's trait.

pub trait SyncPeerFixture {
    // …HS-S0103's items: Local, Remote, Peer, PEER_RECONSTRUCTION, BOTH_ROLES,
    // connect, reconnect, round_trips…

    /// Whether this fixture's receiving side has an adjudicating domain (SY-2).
    /// A genuine trade: a peer with no conflict policy honestly has nothing to
    /// demonstrate, declines, and the rule reports the stated reason.
    const DOMAIN_ADJUDICATION: Capability;

    /// Applies a pulled batch to the local side the way this adapter's real
    /// deployment does: groups atomically, foreign `EventId`s preserved, and —
    /// where `DOMAIN_ADJUDICATION` is available — any compensation authored in
    /// the *same* `append` as the losing event.
    ///
    /// This is the receiving path SY-1, SY-2 and SY-6 all constrain. It lives on
    /// the fixture because `IngestStore::ingest` takes no guard and no groups,
    /// and because a rule that reached it through the runner would be testing
    /// the runner (SY-8).
    ///
    /// # Errors
    ///
    /// The fixture's error when the batch is refused as ingest input — the only
    /// refusal any rule here asserts on (SY-6). Never an error because the
    /// receiver disagrees with an event (SY-1).
    fn apply(
        &self,
        local: &Self::Local,
        batch: &PushBatch,
    ) -> impl Future<Output = Result<Applied, Self::Error>>;
}
```

| Behavior or contract | Details | Evidence path | AC |
| --- | --- | --- | --- |
| **`ingest_never_rejects` asserts convergence, not a return value** | Two stores each accept a conflicting fact under **byte-identical, position-free** conditions; a full exchange in both directions runs; the rule then asserts that both facts are present in **both** logs, keyed by `EventId`. Not `Ok`-ness, not counts, not positions. Re-delivery during the exchange is a no-op (SY-11), so a second exchange changes nothing and the rule may assert that too | `spec/SPECIFICATION.md:5841-5867`, `:6135-6143`; `discover.md`, *Questions* first entry | AC-001 |
| **`compensation_is_atomic_with_the_losing_event` reads mid-sequence** | The receiving side is given a fact its domain adjudicates against one it already holds. The rule reads the local log **at the point `apply` returns** and asserts the losing event is never observable without something resolving it — adjacency measured relative to a head the receiving store reported. Deterministic sequencing, no clock (CF-33), no thread race | `spec/SPECIFICATION.md:5871-5900`; `discover.md`, *The wrong implementation*, `CompensateAfterCommit` | AC-002 |
| **`wire_condition_with_after_is_refused` asserts refusal + no write, never a variant** | The rule builds a guard whose `after` is `Some(p)` where `p` is a position the **origin** store actually assigned, sends it as `EventGroup::guard`, and asserts (a) `apply` returns `Err` and (b) the receiving store's log is unchanged against a head it reported. It does **not** match an error variant: `SyncError`'s extension is ADR-0026's and `WireError` is a decoding failure, not this | `spec/SPECIFICATION.md:5973-6029`, `:6023-6029`; `crates/happenstance-sync/src/lib.rs:91-94`; `crates/happenstance-core/src/append.rs` (`Guard`'s public `after`, `AppendCondition::guards()`) | AC-003 |
| **The apply seam is the fixture's, and it is required** | One method on `SyncPeerFixture`, not a change to `IngestStore` or `SyncPeer`. `ingest` takes `&[ReplicatedEvent]` with no guard, and `MemorySyncPeer` holds what it is pushed rather than applying it, so nothing else in the tree can be driven by these clauses. The runner is HS-S0109's and a rule that used it would test it (SY-8) | `crates/happenstance-sync/src/ingest.rs:150`; `crates/happenstance-sync/src/memory.rs:1-6`; `spec/SPECIFICATION.md:6074-6077` | AC-012 |
| **A declined `DOMAIN_ADJUDICATION` skips loudly** | The rule is still emitted, still runs, returns `RuleOutcome::Skipped` and reports the fixture's stated reason. `#[cfg]`-ing it out yields a build in which declining a capability to turn a red build green leaves no record of the trade. `RuleOutcome` is `#[must_use]`, which is what makes reporting the skip a build failure rather than prose | `crates/happenstance-testkit/src/contract.rs:31-42`, `:465-473`; `project.md`, DR-8; `…/sync-testkit-crate-and-rule-registry/spec.md`, *Context pack* §5 | AC-012 |
| **`ConditionEvaluatingIngest` — declares exactly `ingest_never_rejects`** | Reads `EventGroup::guard`, calls `append(events, Some(&origin_condition))` on the local store, maps `AppendError::ConditionViolated` into a rejection. One correct `append` call, so it passes every event-store rule; clean errors, so it looks well-engineered; reachable **because the field is public**. It must refuse `after`-carrying guards correctly, or it fails SY-6's rule too and stops being a scalpel (CF-3) | `spec/SPECIFICATION.md:5856-5867`, `:7200-7207`; `crates/happenstance-sync/src/peer.rs:236-243` | AC-004 |
| **`PositionPortingPeer` — declares exactly `wire_condition_with_after_is_refused`** | Accepts the `after`-carrying guard and evaluates it in its own numbering. On lockstep fixtures it is *correct*; on a realistic one it passes **vacuously**, because `after: 288455` read locally names an unrelated recent event — *"worse than no check, because it looks like enforcement"*. It must evaluate **only** guards carrying `after: Some(_)`, or it fails `ingest_never_rejects` as well. Provenance: every log-shipping protocol in the world resumes from a scalar offset | `spec/SPECIFICATION.md:6008-6021`, `:6144-6152`; `discover.md`, *The wrong implementation* | AC-005 |
| **`CompensateAfterCommit` — declares exactly `compensation_is_atomic_with_the_losing_event`** | Ingests the losing event, returns, then authors the compensation from a follow-up. Every event arrives, both counts are right, the log converges at rest. It must declare `DOMAIN_ADJUDICATION` **available**, or the rule it targets skips and CF-3's first direction cannot be satisfied | `spec/SPECIFICATION.md:5893-5900`; `discover.md`, *The wrong implementation* | AC-006 |
| **None of the three mutants is built by wrapping the oracle** | A mutant that delegates to `MemorySyncPeer` and perturbs the result tests the wrapper, and shares a subroutine with the implementation the oracle is supposed to be independent of | `standards/rust/60-what-a-test-must-prove.md` RS-60-3, RS-60-4; `…/_decomposition.md`, testing brief *Unit* | AC-004, AC-005, AC-006 |
| **The registry is a `const` table with per-rule `expect` pins** | `Declared { name, kind, fails, provenance, mode, expect }` in `tests/`, matching the existing shape field for field. A pin names the **exact assertion** the mutant should trip, because `FailureMode` says a rule rejected the peer and not which assertion did it; a pin naming a rule the mutant does not declare is an error | `crates/happenstance-testkit/tests/mutation_coverage.rs:140-186`; `spec/SPECIFICATION.md:7186-7207`; `…/_decomposition.md`, *Composition root* §3 | AC-007 |
| **Five meta-tests, both directions asserted** | `every_rule_has_a_mutant` (CF-1), `mutant_registry_is_exhaustive` (CF-2), `mutants_fail_exactly_their_declared_rules` (CF-3 — fails everything declared *and* passes everything not), `every_mutant_states_its_provenance` (CF-4), `conformant_variants_pass_everything` (CF-5) over HS-S0103's two conformant variants, which these rules must not break | `spec/SPECIFICATION.md:7161-7232`; `crates/happenstance-testkit/tests/mutation_coverage.rs` | AC-007 |
| **Provenance names a shape someone would ship** | Never empty (CF-4), and taken from the clause's own argument for plausibility rather than invented. Register and specificity to match the worked strings already in the tree — `GappedPositionStore`'s three-clause justification, `PagedStreamStore`'s *"any store with a network under it"* | `spec/SPECIFICATION.md:7208-7215`; `crates/happenstance-testkit/tests/mutation_coverage.rs:325-350` | AC-008 |
| **No rule asserts a literal position** | Membership is by `EventId`; adjacency is relative to a head the store reported; SY-6's `after` comes from a position the origin assigned. The behavioural enforcement is the conformant variants, the lint HS-S0104 mounted is the second line | `spec/SPECIFICATION.md:7233-7249`; `project.md`, DR-7; `discover.md`, *Gate: Discover* box 6 | AC-009 |
| **No rule and no mutant decodes a payload** | Facts are distinguished by `EventId`, `EventType` and `Tags`. Nothing reads `Event::data` or `Event::metadata` structurally, on any path, including the wrong peers — a mutant that parsed a payload would make the suite's own no-decode claim false | `spec/SPECIFICATION.md:6867-6880`; `project.md`, DR-5; `RUNBOOK.md:4576-4580` | AC-010 |
| **One changelog entry per rule, naming the defect; coverage reported by axis** | CF-29's lint checks that each rule's name appears in `CHANGELOG.md` and that the entry naming it carries real prose; it is *"the weakest check in the gate"* and the substance is review's. The registry's own doc states which defects the set covers and which axes it leaves uncovered — **never a fraction**, because the denominator is a choice | `spec/SPECIFICATION.md:8141-8168`, `:7251-7256`; `xtask/src/lints.rs:505-540` | AC-011 |
| **The rules reach all three harnesses through one enumeration** | Adding a name to `for_each_sync_peer_rule!` is the whole of mounting: tokio, blocking and `wasm32` expand the same list, and `no_orphan_sync_rules` — vacuous until now — becomes load-bearing in both directions | `crates/happenstance-testkit/src/registry.rs:68-102`, `:411-435`; `…/sync-testkit-crate-and-rule-registry/spec.md`, *Behavior and interfaces* | AC-013 |
| **No `[FROZEN]` clause is edited, and two findings are handed on** | `spec/SPECIFICATION.md` is outside the boundary. The `(new)` markers on SY-1/SY-2/SY-6 and the `SyncPeer::push` doc-comment clarification (Context pack §6) are written down here for `frozen-clause-repairs` (HS-S0112) under the repair playbook's three-part form | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `xtask/src/spec_trace.rs`; `project.md`, AC-012, DoD 7 | AC-013 |

## Data and migrations

**N/A — no persistent data, no schema, no migration.** Everything this story adds runs in process and in
memory. The rules drive `MemorySyncPeerFixture`, whose oracle holds its log in a `Vec` behind a `Mutex`
(`crates/happenstance-sync/src/memory.rs:1-10`); the three wrong peers are hand-written shapes in `tests/`
with no backing store at all; the mutant registry is a `const` table compiled into a test binary. Nothing
here reads or writes a file, a database or a network, and no fixture in this PR needs an environment —
`durable-object-and-neon-peers` (HS-S0111) is where infrastructure enters, and it is a `blocks` edge from
here.

Three adjacent things that would be mistaken for data work and are not:

- **The wire.** These rules construct `PushBatch` and `EventGroup` values in memory and never serialise
  one. `Envelope<T>`, `FORMAT_VERSION` and the message set are `message-set-on-the-envelope`'s (HS-S0106),
  and a `#[derive(Serialize, Deserialize)]` on a public message type **is** a wire-format decision,
  authorised by ADR-0027 by name and not by this PR (`crates/happenstance-sync/src/lib.rs:86-94`).
- **Positions.** `wire_condition_with_after_is_refused` puts a real foreign `SequencePosition` on a guard,
  which looks like data crossing a boundary. It is a value read back out of the origin store within the
  same test process; nothing is stored, and the point of the rule is that it must never be *interpreted* on
  the far side.
- **Crate-version arithmetic.** Adding a rule is semver-MINOR for `happenstance-sync-testkit` and nothing
  for the contract crate, which is why the crate carries its own `version` key
  (`xtask/src/main.rs:420-436`). That is a release-metadata consequence of this PR, not a migration, and
  the crate stays `publish = false` throughout (project AC-015).

## Acceptance criteria

The persona throughout is **the adapter author** on *Learn when you are finished*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-167`;
`.bklg/from-contract-to-published-library/initiative.md:241-248`) — someone who wants *"an executable
definition of 'correct' they can run against their own storage system, rather than a prose specification
they have to interpret"*. Their loop today ends at step 3: they run a suite whose green *"is evidence about
one storage shape wearing four hats"*. On the sync axis it is worse — the suite reports zero rules. Each
criterion below is one step of that loop becoming true.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author whose peer receives a batch carrying a position-free append condition, **WHEN** they run `sync_peer_conformance!(MyFixture::new())`, **THEN** `ingest_never_rejects` runs and reports pass only if — after two stores each accepted a conflicting fact under **byte-identical, position-free** conditions and a full exchange ran in both directions — **both facts are present in both logs, keyed by `EventId`**; a peer that returns `Ok` and drops or adjudicates away the foreign fact fails, and neither `Ok`-ness, event counts nor positions are ever the assertion | `crates/happenstance-sync-testkit/src/rules.rs::ingest_never_rejects` green against `MemorySyncPeerFixture` in all three harnesses (`crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs`, `…_blocking.rs`, `…_wasm.rs`), and red against `ConditionEvaluatingIngest` at its pinned assertion in `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` |
| AC-002 | **GIVEN** an adapter author whose receiving peer has a domain that adjudicates an ingested fact against one it already holds, **WHEN** they run the suite, **THEN** `compensation_is_atomic_with_the_losing_event` reads the local log **at the point the fixture's `apply` returns and before anything else runs** and reports pass only if no reader could have observed the losing event with nothing resolving it — a receiver that ingests, returns, and compensates from a follow-up fails, even though its log converges at rest | `crates/happenstance-sync-testkit/src/rules.rs::compensation_is_atomic_with_the_losing_event` green against `MemorySyncPeerFixture` in all three harnesses; red against `CompensateAfterCommit` at the pinned "losing event observable unresolved" assertion in `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` |
| AC-003 | **GIVEN** an adapter author whose peer is handed an `EventGroup` whose `guard` carries `after: Some(p)` — where `p` is a position the **origin** store actually assigned — **WHEN** they run the suite, **THEN** `wire_condition_with_after_is_refused` reports pass only if the receiving side both (a) returns `Err` from `apply` and (b) leaves its log unchanged against a head **it itself reported**; the rule matches no error variant, so a peer whose error enum is its own still passes | `crates/happenstance-sync-testkit/src/rules.rs::wire_condition_with_after_is_refused` green against `MemorySyncPeerFixture` in all three harnesses; red against `PositionPortingPeer`; review confirms the rule body names no `SyncError`/`WireError` variant (`crates/happenstance-sync/src/lib.rs:91-94`) |
| AC-004 | **GIVEN** an adapter author who wants to know the suite can tell them apart from a plausible mistake, **WHEN** they read `tests/`, **THEN** `ConditionEvaluatingIngest` exists as a compiled peer that evaluates the origin's *position-free* guard locally and routes `AppendError::ConditionViolated` into a rejection, is not built by wrapping `MemorySyncPeer`, declares **exactly** `ingest_never_rejects`, and **passes the other two rules** | `mutants_fail_exactly_their_declared_rules` (CF-3) in `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` asserts both directions for this mutant; the crate compiles the peer, so a port change breaks the build rather than rotting it |
| AC-005 | **GIVEN** the same author, **WHEN** they read `tests/`, **THEN** `PositionPortingPeer` exists as a compiled peer that accepts an `after`-carrying guard and evaluates it **in its own numbering**, evaluates *only* `after: Some(_)` guards, declares **exactly** `wire_condition_with_after_is_refused`, and passes the other two — the mutant this project is named after and the one a counting rule cannot see | `mutants_fail_exactly_their_declared_rules` (CF-3) for this mutant, with its per-rule `expect` pin naming the "log unchanged" assertion rather than merely `FailureMode` |
| AC-006 | **GIVEN** the same author, **WHEN** they read `tests/`, **THEN** `CompensateAfterCommit` exists as a compiled peer that ingests the losing event, returns, and authors the compensation afterwards, declares `DOMAIN_ADJUDICATION` **available** so the rule it targets cannot skip past it, declares **exactly** `compensation_is_atomic_with_the_losing_event`, and passes the other two | `mutants_fail_exactly_their_declared_rules` (CF-3) for this mutant; and `every_rule_has_a_mutant` (CF-1) would fail if the capability were declined and the rule skipped instead of failing |
| AC-007 | **GIVEN** an adapter author who does not trust a suite's self-report, **WHEN** the sync suite runs, **THEN** the mutant set is **data** — a `const` table of `Declared { name, kind, fails, provenance, mode, expect }` in `tests/` in the shape the event-store registry already uses — and five meta-tests hold it: every rule has a mutant (CF-1), the registry is exhaustive (CF-2), each mutant fails exactly its declared rules and passes every other (CF-3), every mutant states its provenance (CF-4), and the conformant variants pass everything (CF-5) | `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` — the five meta-tests, each per-rule `expect` pin naming the exact assertion, mirroring `crates/happenstance-testkit/tests/mutation_coverage.rs:140-186`; `.kb/decisions/0010-the-suite-must-prove-itself.md:56-66` is the standing obligation |
| AC-008 | **GIVEN** an adapter author reading a failure report, **WHEN** a mutant is named in it, **THEN** its `provenance` is non-empty and argues, at the register of the strings already in the tree, why *someone would ship this* — SY-1's *"the natural first cut… it passes every event-store conformance rule"*, SY-6's *"worse than no check, because it looks like enforcement"*, SY-2's hub log that says one physical compressor is held twice — never `AlwaysWrong`, never a label | `every_mutant_states_its_provenance` (CF-4) in `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` asserts non-emptiness; adversarial review asserts the substance against `crates/happenstance-testkit/tests/mutation_coverage.rs:325-350` and `spec/SPECIFICATION.md:7208-7215` |
| AC-009 | **GIVEN** an adapter author whose store assigns positions with gaps or outside the transaction, **WHEN** they run the three new rules, **THEN** none of them asserts on a literal position value: membership is by `EventId`, adjacency is measured relative to a head the receiving store reported, and SY-6's `after` is built from a position the origin store actually assigned | `cargo xtask lint-position-literals` over the sync-suite scope HS-S0104 mounted (`xtask/src/lints.rs`), **plus** the behavioural line — `conformant_variants_pass_everything` (CF-5/CF-6) over HS-S0103's conformant variants stays green with the three rules present |
| AC-010 | **GIVEN** an evaluator or adapter author relying on payloads staying opaque, **WHEN** the whole suite runs, **THEN** no rule and no wrong peer reads `Event::data` or `Event::metadata` structurally on any path — facts are distinguished by `EventId`, `EventType` and `Tags` alone, so the suite could never certify a peer that decodes | Review of `crates/happenstance-sync-testkit/src/rules.rs` and `crates/happenstance-sync-testkit/tests/**` against `.kb/decisions/0003-opaque-payloads.md` and `spec/SPECIFICATION.md:6867-6880`; `cargo test -p happenstance-sync-testkit --all-features` with payload bytes compared only for equality |
| AC-011 | **GIVEN** a reader of the changelog deciding whether to upgrade the testkit, **WHEN** they read the entry for this release, **THEN** each of the three rules has its own entry naming the **defect it detects** (CF-29), and the registry's own doc states which defects the set covers and which axes it leaves uncovered — **never a pass rate**, because the denominator is a choice | `cargo xtask lints` — the changelog-per-rule lint over the `RULE_FILES` set HS-S0104 extended (`xtask/src/lints.rs:505-540`); review against `spec/SPECIFICATION.md:7251-7256` and `.kb/playbooks/verify-the-referent-and-report-coverage.md` |
| AC-012 | **GIVEN** an adapter author whose peer has **no adjudicating domain**, **WHEN** they run the suite, **THEN** they still get a verdict on every rule: `SyncPeerFixture` carries the one required `apply` seam (documented, with an `# Errors` section) through which all three rules reach the receiving side, and a declined `DOMAIN_ADJUDICATION` leaves `compensation_is_atomic_with_the_losing_event` **emitted, run, and reported as skipped with the fixture's own stated reason** — never `#[cfg]`-ed away, never silently green | `crates/happenstance-sync-testkit/src/contract.rs` (the trait method and constant, with rustdoc) and `crates/happenstance-sync-testkit/src/fixtures.rs`; a declining fixture in `crates/happenstance-sync-testkit/tests/` whose run output carries the stated reason, mirroring `crates/happenstance-testkit/src/contract.rs:31-42` |
| AC-013 | **GIVEN** an adapter author who wrote one `sync_peer_conformance!` invocation and edited no harness, **WHEN** they run it under tokio, under the blocking bridge and under `wasm32`, **THEN** all three rules appear **by name** in all three, because the only place they are written down is `for_each_sync_peer_rule!` — and the record is left true: `spec/SPECIFICATION.md` is unedited, with the `(new)` markers on SY-1/SY-2/SY-6 and the `SyncPeer::push` doc-comment finding recorded for `frozen-clause-repairs` (HS-S0112) | `no_orphan_sync_rules` green in **both** directions over the enlarged enumeration; all three harness binaries name all three rules in their output; `git diff --name-only main -- spec/` empty and `cargo xtask spec-trace` green; the two findings written into this story's implementation report for HS-S0112 under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`'s three-part form |

**Traceability to the project's ACs.** AC-001, AC-003, AC-009 and AC-013 discharge project **AC-002** (the
central question made mechanical, no `[FROZEN]` clause edited). AC-012 and AC-013 discharge project
**AC-003** (rules present in output; a declined capability reports its reason rather than vanishing).
AC-004 – AC-010 discharge project **AC-004** (the suite discriminates; provenance names a real shape; no
payload decoded). AC-001, AC-002, AC-004, AC-006 discharge project **AC-008** — `RUNBOOK.md:4611-4612`'s
own exit criterion, which names both headline rules verbatim.

## Interaction quality

This story **renders no surface**: `…/replication-identity-and-ingest/_design.md` records a no-surface
determination approved by the repository owner on 2026-08-12, and `design.capture` is a *declared* skip
(`_design.md:12-22`, `:116-126`). The **composition family is therefore N/A by a signed-off decision, not
by omission**, and the API-surface obligation it stands in for is discharged in the architecture brief and
in the rustdoc obligation carried by AC-012 and AC-013 (`standards/rust/70-rustdoc-obligations.md`).

The state family does apply, translated into the only thing this story puts in front of a human: **the
conformance run's output**, which is the adapter author's entire feedback loop. Every applicable invariant
below is already an AC row above — this section says which row carries it and how it is verified, and adds
no new obligation.

| Invariant (state family) | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — three rules arrive inside the author's existing single invocation; no harness is edited, no second command is learned | **AC-013** | `no_orphan_sync_rules` both directions; the three harness binaries name the three rules with no harness diff |
| **Non-occlusion** — a declined capability must not hide the rule it gates; the rule still emits, still runs, still reports | **AC-012** | the declining fixture's run output carries the stated reason; `RuleOutcome` is `#[must_use]`, so failing to report is a build failure |
| **Preserved state, not imposed state** — the rules assert against values the peer itself reported (its own head, its own `EventId`s, the origin's own position), so an adapter's numbering survives contact with the suite | **AC-009** | `lint-position-literals` over the sync scope, plus the conformant variants staying green |
| **Reversibility** — nothing here is a one-way edit: no `[FROZEN]` clause, no port, no harness is changed, and every finding is handed on rather than applied in place | **AC-013** | `git diff --name-only main -- spec/` empty; `crates/happenstance-sync/**` and `xtask/**` absent from the diff |
| **Reachable by the ordinary path** — the three rules run under the commands the author already runs, with no feature flag, no env var and no opt-in | **AC-011**, **AC-013** | `cargo test -p happenstance-sync-testkit --all-features` and `cargo xtask ci --fast` both exercise them; `cargo xtask lints` sees the changelog entries |
| **A failure is legible, not a bare assertion** — a red run names the rule, the assertion that failed and, for a registered mutant, the shape someone would have shipped | **AC-007**, **AC-008** | per-rule `expect` pins in the `Declared` table; `every_mutant_states_its_provenance` (CF-4) |

The named anti-pattern this section exists to forbid is the suite's own version of an unstyled render: **a
run that prints green while running nothing**. That is precisely today's state — `for_each_sync_peer_rule!`
expands to an empty enumeration — and it satisfies every structural assertion perfectly. AC-013's
both-directions orphan check and AC-012's loud skip are what make it fail.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The fixture declines `DOMAIN_ADJUDICATION` | `compensation_is_atomic_with_the_losing_event` still emits and still runs, returns `RuleOutcome::Skipped` and reports the fixture's **stated reason**. It must not be `#[cfg]`-ed out, must not pass, and must not be reported as an environmental problem — a `Capability` is an associated `const` and cannot mean "the environment was down today" (`crates/happenstance-testkit/src/contract.rs:31-42`) |
| EC-002 | The receiving side is handed a guard with `after: Some(_)` | `apply` returns `Err` **and** appends nothing. The rule asserts both; it asserts no error variant, because `SyncError`'s shape is ADR-0026's and `WireError` is a decoding failure (`crates/happenstance-sync/src/lib.rs:91-94`) |
| EC-003 | The receiving side refuses a batch because it *disagrees with an event in it* | That is the SY-1 defect, not a refusal SY-6 permits, and `ingest_never_rejects` must fail the peer. The distinction is structural: SY-6 refuses an unreplicable **condition**; SY-1 forbids refusing a **fact** (`crates/happenstance-sync/src/peer.rs:144-148`, and the doc-comment finding of Context pack §6) |
| EC-004 | A rule function exists in `rules.rs` but is absent from `for_each_sync_peer_rule!` | `no_orphan_sync_rules` fails the build. A compiled, unreachable, green rule is the exact failure this story's mount exists to prevent |
| EC-005 | A mutant fails a rule it did not declare, or passes one it did | `mutants_fail_exactly_their_declared_rules` (CF-3) fails. The likely instances are named in Context pack §7 and are not hypothetical: an over-broad `ConditionEvaluatingIngest` and an over-broad `PositionPortingPeer` each fail two rules by accident |
| EC-006 | A rule is added without a registered mutant, or a registry row names a rule that does not exist | `every_rule_has_a_mutant` (CF-1) / `mutant_registry_is_exhaustive` (CF-2) fail. Merging anyway is forbidden by CF-1's own text and by CLAUDE.md's standing corollary |
| EC-007 | A `CHANGELOG.md` entry is missing for a rule, or is a bare rule name with no prose | `cargo xtask lints`' changelog-per-rule lint fails (`xtask/src/lints.rs:505-540`). It is *"the weakest check in the gate"* — the substance is review's, and a placeholder entry passing the lint is a review finding, not a green |
| EC-008 | A literal position value appears in a rule body | `cargo xtask lint-position-literals` fails over the sync scope. If a rule genuinely needs a position, it reads one back from the store that assigned it |
| EC-009 | A rule cannot be written without changing `crates/happenstance-sync/` | **Stop and record a finding against ADR-0026**; do not widen the boundary. The port is the thing these rules exist to hold still, and a port edit made to satisfy a rule inverts the instrument |
| EC-010 | `MemorySyncPeerFixture` cannot express `apply` without duplicating `MemorySyncPeer`'s dispatch | The fixture may hold its own receiving logic, but the rule's oracle must be spelled in a direction sharing no subroutine with it (`standards/rust/60-what-a-test-must-prove.md`, RS-60-4). If that is impossible, it is a finding about the fixture contract, raised against HS-S0103's spec in the same slice |
| EC-011 | A wrong peer is written by wrapping `MemorySyncPeer` and perturbing its result | Rejected in review: the mutant then tests the wrapper. One defect is one deliberately different implementation (RS-60-3) |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| NF-001 | **No clock, no sleep, no thread race.** AC-002's mid-sequence observation is a deterministic *sequencing* — read the local log at the point `apply` returns — not a timing window | CF-33 forbids a rule reading a clock; a flaky headline rule would be worse than no rule. Checked by review and by the rule running identically under tokio, the blocking bridge and `wasm32` |
| NF-002 | **No `Send` bound is introduced anywhere on the rule path.** The fixture is single-flavour and GAT-free by HS-S0103's decision; these rules must not reach for a `Send` flavour to hold a value across an await | CLAUDE.md binding constraints 1 and 4; `standards/rust/21-send-is-not-inherited.md`. Checked by the `wasm32` harness compiling and by the `wasm32` gate steps HS-S0104 added |
| NF-003 | **Every new public item carries rustdoc, and the doctests actually run.** `happenstance-sync-testkit` is `publish = false`, so `cargo test --doc` skips it — the out-of-package harness `RS-62-5` names is required | `standards/rust/70-rustdoc-obligations.md`, `standards/rust/62-doctests-and-harnesses.md`; `cargo clippy -- -D warnings` and the testkit's doctest harness |
| NF-004 | **No new dependency.** The three rules, the three wrong peers and the registry use what `happenstance-sync-testkit` already has after HS-S0103; in particular no `serde` and no encoding crate enters on this path | `cargo deny` and the feature-powerset steps inside `cargo xtask ci`; ADR-0003's constraint on payload opacity (`.kb/decisions/0003-opaque-payloads.md`) |
| NF-005 | **The story-grain gate stays fast.** Three in-process rules over an in-memory oracle; no fixture in this PR needs an environment, a socket or a file | `cargo xtask affected --base main` remains the story-grain command; infrastructure enters at `durable-object-and-neon-peers` (HS-S0111), which is a `blocks` edge from here |
| NF-006 | **Coverage is reported by axis, never as a fraction.** The registry's doc states which defects the three-rule set covers and which axes — topology, wire format, resume, limits — it does not | `spec/SPECIFICATION.md:7251-7256`; `.kb/playbooks/verify-the-referent-and-report-coverage.md` |
| NF-007 | **`publish = false` holds throughout.** Adding a rule is semver-MINOR for the testkit and nothing for the contract crate; neither sync crate is released by this PR | `cargo package --list` assertion inside `cargo xtask ci` (project AC-015) |

## Implementation notes (non-prescriptive)

Names, decomposition and file layout are the implementer's. What follows is the order of work that keeps
the expensive mistakes cheap, and it is a suggestion everywhere it is not carrying a citation.

**Write each assertion from the clause before opening `memory.rs`.** All three clauses state their own
assertion and their own target (`spec/SPECIFICATION.md:5848-5867`, `:5871-5900`, `:5973-6029`). Reading
`MemorySyncPeer` first and restating what it does produces an oracle that shares a subroutine with the
implementation — RS-60-4's named failure. A workable order per rule: (1) write the assertion from the
clause text; (2) write the wrong peer the clause's `Rejects:` field names; (3) watch the rule fail it;
(4) then make it green against `MemorySyncPeerFixture`. Step 3 before step 4 is what makes the rule an
instrument rather than a description.

**Land the `apply` seam first, because all three rules need it.** It is one required method on
`SyncPeerFixture` plus the `DOMAIN_ADJUDICATION` constant. Extending the slice-mate's trait mid-slice is
expected and is why the two stories are implemented in one context; extending `IngestStore` or `SyncPeer`
is not (Context pack §2, PR boundary).

**For AC-002's mid-sequence read**, the simplest shape that is not a race: have `apply` return once, and
have the rule read the local store immediately afterwards through the fixture's own local handle. A
conformant receiver has already written both events in one `append`; `CompensateAfterCommit` has written
one. No barrier, no channel, no `sleep`. If a shape appears to need one, that is a signal the assertion has
drifted toward "the log converges eventually", which every peer passes.

**For AC-003's foreign position**, append to the origin store, read the assigned `SequencePosition` back
out, and build the `Guard` from it (`crates/happenstance-core/src/append.rs` — `Guard`'s public `after`,
readable through `AppendCondition::guards()`). That is what makes the rule exercise a *real* foreign
position without a literal appearing anywhere, and it is the shape SY-6 itself points at
(`spec/SPECIFICATION.md:6023-6029`).

**Keep each mutant narrow on purpose, and say so in the code.** The two over-broad shapes are named in
Context pack §7. A one-line comment on each wrong peer stating which rule it is *not* allowed to fail costs
nothing and is what a reviewer needs six months from now, when CF-3 goes red and the question is whether the
mutant or the rule moved.

**Copy the `Declared` table field for field** from `crates/happenstance-testkit/tests/mutation_coverage.rs:140-186`
rather than designing a second shape. Two registries with different shapes is two things to keep in step,
and the `expect` pins exist for a reason the event-store suite learned the expensive way: `FailureMode`
says a rule rejected the peer, not which assertion did it.

**Write the `CHANGELOG.md` entries as you write each rule, not at the end.** The lint checks the rule name
appears; the entry that earns its place names the defect — *"catches a receiver that re-evaluates the
origin's append condition locally, which produces a peer set that never converges"* — not *"added
`ingest_never_rejects`"*.

**Two findings to carry out of this story rather than fix in it**: the `(new)` markers on SY-1/SY-2/SY-6's
`Rule:` lines, and the `SyncPeer::push` doc comment whose *"may not refuse a push because it disagrees with
the events in it"* reads as though it might forbid SY-6's refusal (Context pack §6). Both belong to
`frozen-clause-repairs` (HS-S0112); write them into the implementation report in the repair playbook's
three-part form so that story can pick them up without re-deriving them.

## Tests and CI (merge gate)

Grounded in `…/replication-identity-and-ingest/_decomposition.md`, *Testing brief* — *The test mix, tier by
tier* and *Merge-gate commands*. Nothing below invents a tier: this story is Unit + Integration weight on
an already-mounted Static tier.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo xtask lints` (`.redkiln/config.yaml:48`, `reachability_static`) | The changelog-per-rule lint sees three new rule names with real prose (**AC-011**, EC-007); `lint-position-literals` sees no literal in the sync scope (**AC-009**, EC-008) |
| Static | `cargo xtask spec-trace` | No `[FROZEN]` clause was edited and no `Rejects:` symbol rotted; the `(new)` markers still render SY-1/SY-2/SY-6 as scheduled, which is correct until HS-S0112 (**AC-013**) |
| Static | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` | The three wrong peers compile clean; no `#[allow]` smuggled in to make a mutant build (NF-003) |
| Unit | `cargo test -p happenstance-sync-testkit --all-features` → `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` | The five meta-tests: CF-1 every rule has a mutant, CF-2 registry exhaustive, CF-3 exactly-declared in both directions, CF-4 provenance non-empty, CF-5 conformant variants pass everything (**AC-004**, **AC-005**, **AC-006**, **AC-007**, **AC-008**, EC-005, EC-006) |
| Unit | `crates/happenstance-sync-testkit/src/registry.rs` → `no_orphan_sync_rules` | Both directions over the enlarged enumeration: no rule in `rules.rs` is unmounted, no enumerated name is missing (**AC-013**, EC-004) |
| Unit | the testkit's out-of-package doctest harness (`standards/rust/62-doctests-and-harnesses.md`, RS-62-5) | The new public items' doctests actually run in a `publish = false` crate (NF-003, and the presentation half of **AC-012**) |
| Integration | `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs` (tokio) | All three rules green against the oracle and **named in the output** (**AC-001**, **AC-002**, **AC-003**, **AC-013**) |
| Integration | `crates/happenstance-sync-testkit/tests/memory_peer_conformance_blocking.rs` | The same three rules through the blocking bridge, from the same enumeration, with no harness edit (**AC-013**) |
| Integration | `cargo xtask wasm` → `crates/happenstance-sync-testkit/tests/memory_peer_conformance_wasm.rs` | The same three rules compile and run on `wasm32` — no `Send` bound crept in (**AC-013**, NF-002) |
| Integration | a declining fixture in `crates/happenstance-sync-testkit/tests/` | `DOMAIN_ADJUDICATION` declined ⇒ the rule emits, runs, is reported `Skipped` with the fixture's stated reason (**AC-012**, EC-001) |
| Integration | review pass over `crates/happenstance-sync-testkit/src/rules.rs` and `tests/**` | No `Event::data` / `Event::metadata` read structurally on any path, including the wrong peers (**AC-010**) |
| Project gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story grain: what this diff could break, run before anything else |
| Project gate | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`) | This project's ceiling — three new rules do not turn the assembled gate red (initiative DoD 13). `cargo xtask ci` is the terminal project's, not this one's |
| Ledger | `.bklg/…/headline-rules-and-mutant-registry/_ledger.md` (`.redkiln/config.yaml:67`, `require_ledger: true`) | One row per AC-### with cited evidence — a green gate proves the gate passed, never that AC-002's mid-sequence assertion is the one that ran |

## Risks and coupling (PR-scoped)

| risk | why it is live here | mitigation, inside this PR |
| --- | --- | --- |
| **A headline rule that no peer can fail** | The exact failure this repository already lived through: `positions_are_unique` and `positions_are_strictly_monotonic` were decorative for four phases (`spec/SPECIFICATION.md:7167-7185`). The sync-shaped version is `receiver.len() == sender.len()` after an exchange, which every peer above passes — including `ConditionEvaluatingIngest` on a non-conflicting fixture | Write the wrong peer before the rule and watch it fail (Implementation notes). CF-1 and CF-3 hold the line mechanically; the discover stage already names all three targets |
| **A mutant broader than it declares** | Two of the three are one careless line from failing a second rule, and the careless line is the obvious one (Context pack §7) | CF-3's second direction is asserted, not assumed; each wrong peer carries a comment naming the rules it must **pass** |
| **The oracle writes the assertions** | Reading `MemorySyncPeer` and restating it produces a suite that certifies `MemorySyncPeer` (RS-60-4) | Assertion from the clause first, oracle second; review checks the direction, since no test can |
| **Extending `SyncPeerFixture` mid-slice churns HS-S0103** | The `apply` seam did not exist when HS-S0103's spec was written; adding a required method to a just-landed trait is real churn | The two stories are slice-mates implemented in **one context** by design (`…/_storymap.md`, *Merge order* 3). The extension is one method and one constant, and it is the mount — if it grows past that, stop and re-plan |
| **ADR-0027's compensation contract lands differently than assumed** | `compensation_is_atomic_with_the_losing_event` asserts a contract that `adr-0027-merge-compensation-and-message-set` (HS-S0099) decides. That story is a hard `depends_on` for exactly this reason | Read the accepted atom before writing the rule. If it disagrees with SY-2's frozen text, that is a finding and a re-plan, never a rule bent to fit (discover *Box 7*) |
| **SY-6's refusal channel is ADR-0026's, not this rule's** | A rule matching an error variant would pre-empt an accepted atom and fail every adapter with its own error enum | The rule asserts `Err` + nothing appended and matches no variant (**AC-003**, EC-002) |
| **The `push` doc comment reads as contradicting SY-6** | A future reader could conclude the refusal is itself a SY-1 violation (Context pack §6) | Recorded as a finding for HS-S0112 in the repair playbook's form; `crates/happenstance-sync/` stays out of the diff |
| **Boundary drift into the port** | The tempting fix for a hard rule is a small port change | `crates/happenstance-sync/**`, `xtask/**` and `spec/**` are outside the PR boundary; EC-009 makes the alternative explicit — a finding against ADR-0026 |
| **`wasm32` breakage discovered late** | The third harness is the one nobody runs locally by habit, and it is where a stray `Send` bound surfaces | `cargo xtask ci --fast` contains the `wasm32` steps HS-S0104 added; run it before declaring done, per CLAUDE.md *Commands* |

**Coupling out.** `message-set-on-the-envelope` (HS-S0106) and `durable-object-and-neon-peers` (HS-S0111)
both consume the rule shape and the registry shape this story sets; `frozen-clause-repairs` (HS-S0112)
consumes its two recorded findings. A shape decided loosely here is re-decided three times downstream.

## Dependencies

**Blocks on** (all three are `blocked_by` edges in this story's frontmatter, and the first two are
slice-mates in `sync-conformance-suite`):

| story slug | what this story cannot start without |
| --- | --- |
| `sync-testkit-crate-and-rule-registry` (HS-S0103) | The crate itself, `for_each_sync_peer_rule!` / `sync_peer_conformance!`, `SyncPeerFixture` and its round-trip counter, `no_orphan_sync_rules`, the conformant variants, and the three harnesses. There is nowhere to mount a rule until this exists |
| `gate-mounts-for-the-sync-suite` (HS-S0104) | `RULE_FILES`, the position-literal lint scope, the changelog-per-rule lint scope and the two `wasm32` steps. *"The suite is not landed until the gate runs it and a mutant fails it"* — this story is the first content those lints see |
| `adr-0027-merge-compensation-and-message-set` (HS-S0099) | The compensation contract `compensation_is_atomic_with_the_losing_event` asserts. A rule may not assert a contract that has not been decided (`RUNBOOK.md:306`; `…/_storymap.md`, *Merge order* 1) |

Transitively: ADR-0026 (HS-S0098) via HS-S0099, and the ingest seam and memory oracle
(`memory-store-ingest-seam`, `ingest-store-and-memory-peer-round-trip`) via HS-S0103.

**Unlocks**:

| story slug | what it takes from here |
| --- | --- |
| `message-set-on-the-envelope` (HS-S0106) | The rule + mutant shape the WF rules copy, and a non-empty enumeration to add to |
| `durable-object-and-neon-peers` (HS-S0111) | Three rules that mean something, to run against two peers unlike the oracle — the project's DoD 4 proof artefact |
| `frozen-clause-repairs` (HS-S0112) | Rules that exist, so the `(new)` markers can come off; plus this story's two recorded findings |
| `byte-identical-round-trip-and-idempotent-replay` (HS-S0108) | The no-payload-decode discipline (**AC-010**) it extends into a negative control |

## Anchors (progressive disclosure)

Open these at the moment named, not before. Every path below was confirmed to exist in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (SY-1 `:5841-5867`, SY-2 `:5871-5900`, SY-6 `:5973-6029`) | The three clauses write each rule's assertion **and** its wrong implementation. Reading them is what stops the oracle from writing the assertions | Before writing each rule, and again before writing its wrong peer | AC-001, AC-002, AC-003 |
| `spec/SPECIFICATION.md` (CF-1 – CF-6, `:7161-7256`) | The mutant regime: rule-owes-mutant, registry-as-data, exactly-declared in both directions, provenance, conformant variants, no literal position, no pass rate | Before writing the `Declared` table and the five meta-tests | AC-004, AC-005, AC-006, AC-007, AC-008, AC-009, AC-011 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted atom behind all of it — *"a rule may not be added until a store exists in the testkit's own `tests/` that fails it"*, and why a skip must be reported rather than silent | First, before any code; it is the standing constraint the whole story is an instance of | AC-004, AC-007, AC-012 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` (table `:140-186`, provenance `:325-350`) | The exact `Declared` shape to copy field for field, and the register the provenance strings must match | While writing the registry and each provenance string | AC-007, AC-008 |
| `crates/happenstance-testkit/src/contract.rs` (`:31-42`) | `Capability` and `RuleOutcome` — the declined-capability semantics this story inherits rather than re-decides | Before adding `DOMAIN_ADJUDICATION` and before writing the declining fixture | AC-012 |
| `crates/happenstance-testkit/src/registry.rs` (`:68-102`, `:411-435`) | The enumeration-as-single-source-of-truth pattern and the orphan meta-test, in the shape the sync registry mirrors | When mounting each rule into `for_each_sync_peer_rule!` | AC-013 |
| `crates/happenstance-testkit/src/suite.rs` | The rule-function signature shape (`open: impl AsyncFn() -> F`) the sync rules mirror | When writing the first rule's signature | AC-001, AC-002, AC-003 |
| `crates/happenstance-sync/src/peer.rs` | `SyncPeer`, `PushBatch`, `EventGroup`'s **public** `guard` field — the reason `ConditionEvaluatingIngest` is reachable at all — and the `push` doc comment of Context pack §6 | Before building any batch, and again when recording the doc-comment finding | AC-003, AC-004, AC-013 |
| `crates/happenstance-sync/src/ingest.rs` | `IngestStore::{ingest, holds, watermark}` takes `&[ReplicatedEvent]` and no guard — the fact that forces the `apply` seam onto the fixture rather than the port | Before adding the fixture method, when the temptation is to widen `IngestStore` | AC-012 |
| `crates/happenstance-sync/src/memory.rs` | The oracle: *"a peer, not a store — it holds whatever was pushed to it"*. Read it **after** the assertions are written, to check them, never to derive them | After each rule's assertion exists, to make it green | AC-001, AC-002 |
| `crates/happenstance-core/src/append.rs` | `Guard`'s public `after` and `AppendCondition::guards()` — readable but not literal-constructible, which is what makes SY-6's refusal mechanically checkable | When building the `after`-carrying guard from a real origin position | AC-003, AC-009 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-3 (one defect is one deliberately different implementation) and RS-60-4 (spell the oracle in a direction sharing no subroutine with the implementation) | Before writing each wrong peer, especially if wrapping `MemorySyncPeer` starts to look convenient | AC-004, AC-005, AC-006 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-5: a `publish = false` crate's doctests are skipped by `cargo test --doc`, so the out-of-package harness is required | When adding rustdoc examples to the new public items | AC-012 |
| `standards/rust/70-rustdoc-obligations.md` | The repository-wide obligation that stands in for a design surface here, including the `# Errors` section on the fallible fixture method | When writing the `apply` method's docs | AC-012 |
| `xtask/src/lints.rs` (scopes `:32-42`, changelog lint `:505-540`) | Why the registry lives in `tests/` and not `src/`, and exactly what the changelog lint checks and does not check | Before choosing where the mutants live, and when writing the changelog entries | AC-007, AC-011 |
| `xtask/src/spec_trace.rs` | Why the `(new)` markers must stay on until HS-S0112 removes them, and what would break if this story removed them early | Only if tempted to touch `spec/SPECIFICATION.md` | AC-013 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The three-part form the two handed-on findings must be written in so HS-S0112 can act on them without re-deriving | When writing the implementation report, at the end | AC-013 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | Coverage reported by axis rather than as a fraction — the discipline behind the registry's own doc paragraph | When writing the registry's coverage note | AC-011 |
| `.kb/decisions/0003-opaque-payloads.md` | Why no rule and no mutant may read a payload byte, and what a suite that did would certify | Before writing anything that compares two events | AC-010 |
| `.bklg/…/headline-rules-and-mutant-registry/discover.md` | The signal ledger and *The wrong implementation* — all three wrong peers already argued in full, plus *Gate: Discover* box 6 on why positions are the sharp edge here | First, alongside this spec; it is the shortest path to the three mutants | AC-004, AC-005, AC-006, AC-009 |
| `.bklg/…/replication-identity-and-ingest/_decomposition.md` (*Testing brief*, *Composition root* §3, §6) | Which tier proves which AC, the fixtures-and-seams table (the `guard` row and the round-trip-counter row), and the gate mounts this story's lints depend on | When wiring the tests and before running the merge gate | AC-009, AC-011, AC-012, AC-013 |
| `.bklg/…/sync-testkit-crate-and-rule-registry/spec.md` | The fixture contract, the registry shape and the declined-capability decision this story extends rather than re-decides | Before adding the `apply` method — it is that spec's trait being extended | AC-012, AC-013 |
| `.bklg/…/gate-mounts-for-the-sync-suite/spec.md` | Exactly which lint scopes and `wasm32` steps are already in the tree, so this story neither re-adds nor assumes them | Before running `cargo xtask lints` for the first time | AC-009, AC-011 |
| `RUNBOOK.md` (phase 13, `:4516-4620`; exit criterion `:4611-4612`) | The phase's own bar, naming both headline rules verbatim — the sentence project AC-008 is a restatement of | At the start, for orientation, and at the end to check the bar is met | AC-001, AC-002 |

## Clarifications resolved during spec

1. **The AC set is exactly the thirteen the front half enumerated** — AC-001 through AC-013 — and the
   acceptance ledger carries one row per id. Nothing was added or dropped. Two of them (AC-012, AC-013) are
   supported by two rows each in *Behavior and interfaces*, which is deliberate: each is one criterion with
   two observable halves, not two criteria sharing an id.
2. **`wire_condition_with_after_is_refused` is a first-class criterion, not a rider on SY-1.** The discover
   stage asked whether it was scope creep and answered no; this spec makes it AC-003 with its own mutant
   (AC-005) and its own error condition (EC-002), because project AC-002 names SY-1 *and* SY-6 as the pair
   the central question is reconciled against.
3. **"Refused" is asserted as `Err` plus no write, and never as an error variant.** The discover stage
   deferred the channel question to spec. Resolved: the rule asserts two observable facts and matches no
   variant, because `SyncError`'s shape belongs to ADR-0026 and a variant match would fail every adapter
   whose error enum is its own (EC-002).
4. **The mutant registry lives in `tests/`, not `src/`.** Deferred by discover with the precedent noted;
   resolved in favour of the precedent, because CF-33's `TESTKIT_SRC` scope is `src/` *precisely* so that
   `tests/` can hold deliberately-wrong implementations. Putting the registry in `src/` would make
   HS-S0104's lint scopes fight this story's content.
5. **The receiving-side seam is one required method on `SyncPeerFixture`, added by this story.** Neither
   `IngestStore::ingest` nor `MemorySyncPeer` can be driven by these clauses as they stand, and the runner
   that would do it in production is HS-S0109's — a rule that used it would be testing the runner (SY-8).
   Extending the slice-mate's trait is mounting; extending the port is drift.
6. **Interaction quality's composition family is N/A by a signed-off decision.** `_design.md` records a
   no-surface determination approved 2026-08-12 with `design.capture` a declared skip, so there is no
   surface id, no density budget and no design-system primitive to compose against. The state family does
   apply, translated to the conformance run's output, and every applicable invariant is carried by an
   existing AC row — none was left as prose.
7. **Whether AC-013 should have been split into "the mount reaches three harnesses" and "the record is left
   true".** Kept as one criterion: both halves are the same claim — the three rules become visible to the
   adapter author *without* anything being edited that was not this story's to edit — and splitting it would
   have changed the id set the front half fixed. The two handed-on findings are named inside AC-013's
   verification so they cannot be lost.
8. **No pass rate, anywhere.** NF-006 and AC-011 both say it, because the temptation arrives at report time
   rather than at implementation time: three rules with three mutants invites "100% mutant kill rate", and
   the denominator is a choice.
