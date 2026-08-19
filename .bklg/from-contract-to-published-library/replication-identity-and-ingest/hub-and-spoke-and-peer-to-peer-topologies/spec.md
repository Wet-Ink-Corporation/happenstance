---
item: HS-S0110
stage: spec
created: 2026-08-12T13:47:51.212Z
updated: 2026-08-12T13:47:51.212Z
template_sig: 87bbf1d0
rendered_sig: 4c7fa6cb
---

# Spec — Both topologies are first class, and hub-ness is an edge

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` (AC-007 at `:214-218`) |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/hub-and-spoke-and-peer-to-peer-topologies/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — architecture *Non-prescriptive implementation notes* on the topology question and the module doc (`:499-506`), the findings-before-deletion warning (`:590-599`), AC-A04 (`:542-544`); testing brief *Integration* → AC-007's two topologies (`:745-751`), *Fixtures and seams to mock* (`:802-811`), *Merge-gate commands* (`:775-800`), and *Notes* on what it deliberately does not test (`:848-853`) |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12 (`:120-125`). No surface ids exist for this story to render; the API-surface obligation is discharged through the briefs and `standards/rust/70-rustdoc-obligations.md` |
| Story map row | `…/replication-identity-and-ingest/_storymap.md:67` (slice `runner-and-topologies`, row 2); merge order at `:132-133`; backbone activity A5 *run a fleet* at `:44` |
| This story's discovery | `…/hub-and-spoke-and-peer-to-peer-topologies/discover.md` — the signal ledger, the six questions (four answered, two deferred to here) and the four named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13 in full); `RUNBOOK.md:306` (ADR-0027 in the queue); `RUNBOOK.md:4597-4602` (the proof artefact this story feeds) |

## One-line PR slice

Exercise both topologies over the same suite and the same runner, with one adapter type instantiated
in both roles on different edges so SY-9's "hub-ness is an edge property" is tested rather than
asserted, and bring `crates/happenstance-sync/src/lib.rs`'s module documentation into line with
whichever shape landed.

## Executive summary

The crate has documented two target topologies since phase 2 and has run neither.
`Watermark` is a version vector rather than a scalar *specifically* because a hub sees many logs and
a spoke sees one, and the crate calls that "a constraint on the port's shape and not merely on its
documentation" (`crates/happenstance-sync/src/identity.rs:208-215`,
`crates/happenstance-sync/src/lib.rs:62-66`). SY-9 is `[FROZEN]` on the strongest form of the claim —
hub-ness is a property of an **edge**, and one adapter type must be usable as a hub to one set of
peers and a symmetric peer to another *simultaneously* (`spec/SPECIFICATION.md:6090-6106`). Nothing
has ever constructed one adapter twice and driven both edges.

**The delta this PR lands** is the first test that could fail if any of that were wrong: SY-9's own
prescribed rule `one_adapter_serves_both_roles` in `crates/happenstance-sync-testkit/src/rules.rs`
with the `HubFlaggedPeer` mutant CF-1 obliges it to carry, plus the two runner-grain topology
scenarios — a hub with **two** spokes and a symmetric pair — over the runner
`send-free-sync-runner` (HS-S0109) landed hours earlier in the same slice, and the documentation
correction AC-007's third clause asks for, arrived at by reading the file first.

The predecessors put everything else in place: `sync-testkit-crate-and-rule-registry` (HS-S0103)
declared `BOTH_ROLES` as a fixture capability with this story named as its consumer
(`…/sync-testkit-crate-and-rule-registry/spec.md:296-298`), `gate-mounts-for-the-sync-suite`
(HS-S0104) taught `RULE_FILES` and the position-literal and changelog lints about the fourth suite,
and `headline-rules-and-mutant-registry` (HS-S0105) built the `Declared` registry and its five
meta-tests that this story's rule and mutant now join. **This story adjudicates no design**: whether
the two topologies are one abstraction or two is ADR-0027's, landed in slice 1 and carried on this
story's `depends_on` edge (`…/_storymap.md:67`).

## Context pack

Read this before opening any file. Every numbered item below is a decision already taken; the
anchors behind them are for retrieval, not re-deliberation.

**1. SY-9 is `[FROZEN]` and it wrote this story's headline test for us.** *"Hub-ness MUST NOT be a
property of the port's type or constructor. One adapter type MUST be usable simultaneously as a hub
to one set of peers and as a symmetric peer to another"* — and the `Rule:` field specifies the
mechanism completely: *"a compile-and-run test that constructs one peer adapter twice, drives one
edge in each role, and asserts both exchanges complete"* (`spec/SPECIFICATION.md:6090-6106`). That
is a **suite rule**, in `happenstance-sync-testkit`, not an integration test in
`happenstance-sync/tests/`. Its `Rejects:` is `Peer::new(is_hub: bool)` and separate
`HubPeer`/`SpokePeer` traits, with Kestrel Rotor as the case: a vessel that is a hub to nine
technician tablets over ship's wifi and a symmetric peer to a shore depot over satellite, *at the
same time*, so a boolean on the constructor forces it to hold two incompatible adapter instances
over one store "and gives neither of them a name for what the other is doing".

**2. The word doing the work is *simultaneously*, and the easy version proves less.** Two scenarios
that each construct their own adapter — a `MemorySyncPeer` hub here, a `MemorySyncPeer` spoke there,
both green — satisfy every word of AC-007 as a reader skims it and never exercise the claim. The
distinguishing test is **structural**: one `let peer = …` binding, two edges, **both driven before
either exchange completes**. Write it that way deliberately; the version that constructs twice reads
more naturally and is the suite-shaped mutant this story's `discover.md` names (`:141-152`).

**3. The oracle must stop declining `BOTH_ROLES`, and the declined path must not be lost in the
move.** HS-S0103 shipped `MemorySyncPeerFixture` *declining* `BOTH_ROLES` with a stated reason,
deliberately, so the skip machinery is exercised on every run
(`…/sync-testkit-crate-and-rule-registry/spec.md:368`). Left as it is, SY-9's rule is permanently
`Skipped` against the only always-on peer in the tree, and AC-007 reports green while its headline
claim never runs. **This story flips `MemorySyncPeerFixture::BOTH_ROLES` to supported and moves the
declined-capability exercise onto a variant fixture in `crates/happenstance-sync-testkit/tests/`**,
so DR-8's "a declined capability still runs and reports the fixture's stated reason" keeps a live
instrument (`project.md`, DR-8; `crates/happenstance-testkit/src/contract.rs:359-433`). A fixture
that cannot present one local store with two distinct far sides declines honestly — that is what
`BOTH_ROLES` is for, and it is the legitimate answer for a live peer at
`durable-object-and-neon-peers` (HS-S0111).

**4. Two peer handles in one rule is not a second peer relationship on the port.** SY-8 is `[FROZEN]`
— the port describes exactly one relationship, and *"a rule that needed two would be testing the
runner"* (`spec/SPECIFICATION.md:6070-6086`). SY-9's rule is the named exception in the
specification's own text, and it stays inside SY-8 because what it exercises is **one fixture
instance**: one isolated backing store, two handles onto it, in the same sense `SECOND_HANDLE`
already carries for the event-store suite (`CLAUDE.md`, *The rule that matters*). The alternative —
demote SY-9's rule to an integration scenario in `happenstance-sync/tests/` — loses, because a
frozen clause's `Rule:` field names a testkit rule, `spec-trace` reads that field, and the two live
peers at HS-S0111 would then never be asked the question at all.

**5. A hub needs at least two spokes, and this is not padding.** A hub with one spoke is a
peer-to-peer pair with a label on it. It exercises none of the properties that separate the
topologies — the version-vector watermark, per-peer confirmation state, and the hub's ability to
adjudicate across logs a spoke cannot see. `ScalarWatermark` (collapse `Watermark` to a single
position, because in a two-node test that is all it ever holds) passes every symmetric test and
fails only once **two** spokes have divergent progress (`discover.md:154-159`;
`crates/happenstance-sync/src/identity.rs:208-215`).

**6. Every assertion is over `EventId` sets and per-origin watermark advancement. Never a position.**
Three nodes means three independent position sequences and the natural way to write "everything
arrived" is to compare numbers. SY-19 is `[FROZEN]`: no expression relates two peers' positions and
none can be constructed, because `SequencePosition` carries no origin — Kestrel Rotor's fourteen
events sit at vessel position 38,102 and depot position 3,918,442
(`spec/SPECIFICATION.md:6408-6428`). CF-6 forbids the literal
(`spec/SPECIFICATION.md:7233-7249`) and HS-S0104 has already pointed the lint at this crate.

**7. SY-10 is exercised as a runner scenario, not as a suite rule, and the divergence is recorded
rather than repaired.** SY-10 is `[PROVISIONAL]` — both topologies first class, *"the runner MUST
permit a different merge rule in each direction of one edge"* — and its `Rule:` field names
`directional_merge_rules_compose` in `happenstance-sync-testkit`
(`spec/SPECIFICATION.md:6110-6128`). A rule expressing it needs two peer handles **in two different
directions with two different policies**, which is the runner, and SY-8 is frozen while SY-10 is
provisional: the frozen clause wins. So the directional-merge exercise lands in
`crates/happenstance-sync/tests/`, `directional_merge_rules_compose` keeps its `(new)` marker, and
the finding — *this clause's `Rule:` field points at the wrong tier* — is written down and handed to
`frozen-clause-repairs` (HS-S0112) and `clause-arithmetic-and-deferral-renewals` (HS-S0113). It is
**not** softened, edited, or silently satisfied by a differently-named rule here.

**8. SY-10's falsification test is run in the honest direction, and a positive result stops this
story rather than being absorbed by it.** The clause carries a real experiment: *"implement the hub's
rule on a spoke and show the spoke's projections stay correct"* (`spec/SPECIFICATION.md:6112-6117`).
Run it. If the spoke's projections stay correct, SY-10 is falsified — that is a marker change, which
takes a new decision atom and routes to HS-S0113 with a recorded finding, never a clause quietly
relaxed in this diff (`discover.md`, *Gate: Discover* box 7). If they do not, `SymmetricWithDeclinedPush`
— model hub-and-spoke as peer-to-peer with one side declining to push — is refuted by name, and the
thing it loses is stated in the finding: the hub's adjudication (it holds every log; the spoke does
not) and, in Kestrel Cold Chain, a **commercial** confidentiality boundary, Scottish subcontractors
not holding Yorkshire's parts pricing (`spec/SPECIFICATION.md:6120-6128`).

**9. SY-31's rule is not landed here, and that is a written disposition rather than an omission.**
A per-peer confirmation watermark must be written through `ProjectionStore` under a reserved
`ProjectionId` (`spec/SPECIFICATION.md:6731-6760`, `[PROVISIONAL]`). Its rule
`watermark_advances_transactionally_with_a_read_model` needs a `ProjectionStore` handle that
`SyncPeerFixture` does not have and — by PS-9, quoted inside SY-31's own text — a *generic* runner
cannot write a read model beside the watermark at all. Per-edge confirmation state in this story's
scenarios is therefore the runner's per-peer resume token plus `Watermark` per origin store; the
clause keeps its `(new)` marker and its renewal is HS-S0113's. Adding the ingredient to the fixture
trait here would be a fixture-contract change three stories after it was signed off.

**10. The module documentation gets read before it gets edited, and its findings outlive it.**
AC-007's third clause asks that `crates/happenstance-sync/src/lib.rs` "no longer describes only one"
topology. **It already describes both** — peer-to-peer at `:53-58`, hub and spoke at `:60-66`, with
the version-vector reason attached — and the architecture brief says so and says to verify before
rewriting (`…/_decomposition.md:499-506`). The genuinely stale prose is elsewhere in the same file:
the `# Status: a phase-2 sketch, not the protocol` header (`:3-15`) and the *hard part, stated
honestly* list (`:98-133`), whose bullets this project has now answered. Every paragraph deleted must
first have its **finding** moved into the ADR that consumed it — the findings are not the same as the
`todo!()`s, and this crate is the most honestly self-documenting one in the workspace
(`…/_decomposition.md:590-599`).

**11. The persona-journey slice.** Backbone activity **A5 — run a fleet** (`…/_storymap.md:44`). The
consumer is the application author deploying the Kestrel Rotor arrangement: one node that is
authoritative to many devices and an equal to one upstream, at the same time, over one store. What
they must be able to observe is that this needs no second adapter instance, no constructor flag and
no second trait — and, at HS-S0111, that their own adapter is asked the same question.

**12. What is deliberately not proved here.** Scoped replication and the confidentiality mechanism
SY-10's rationale rests on (SY-27/SY-28, `[DEFERRED]` against
`references/evaluation/PRESSURE-TEST.md:685-693`) — renewed at HS-S0113, not built. The live peers
(HS-S0111). The merge rule itself, which is ADR-0027's. And any `[FROZEN]` clause edit, which is out
of scope for the whole initiative.

## Integration contract

**Archetype** — `capability`. A slice through the fixture contract, the rule registry, the runner and
the crate's own documentation, observable by an adapter author running one command.

**Slice / milestone** — `runner-and-topologies`. Slice-mate: `send-free-sync-runner` (HS-S0109),
implemented in the same context and mounted as one surface; HS-S0109 merges first
(`…/_storymap.md:132-133`). This story is the slice's second and last row.

**Mount point** — `crates/happenstance-sync-testkit/src/registry.rs`, the `for_each_sync_peer_rule!`
enumeration. That macro is this suite's composition root: a rule defined in `src/rules.rs` and not
enumerated there exists and never runs, which is exactly what `no_orphan_sync_rules` fails on
(`…/sync-testkit-crate-and-rule-registry/spec.md:367`; the shape it mirrors is
`crates/happenstance-testkit/src/registry.rs:411-435`). `one_adapter_serves_both_roles` is not landed
until the enumeration names it and all three harnesses — tokio, blocking, `wasm32` — therefore run
it. Three secondary mounts are part of the same wiring and are **not** scope drift:
`crates/happenstance-sync-testkit/src/fixtures.rs` (the `BOTH_ROLES` flip on the reference fixture),
`crates/happenstance-sync-testkit/tests/mutation_coverage.rs` (the `Declared` table, which CF-1 and
CF-2 make mandatory the moment a rule exists), and `CHANGELOG.md` (CF-29's entry naming the defect
the rule detects).

**Wires into** — real sibling contracts, all in-tree at this point in the merge order:

- `crates/happenstance-sync-testkit/src/contract.rs` — `SyncPeerFixture`: `Local`, `Remote`, `Peer`,
  `connect()`, `reconnect()`, `round_trips()`, and the `BOTH_ROLES` / `PEER_RECONSTRUCTION`
  capability constants HS-S0103 declared (`…/sync-testkit-crate-and-rule-registry/spec.md:281-306`).
  Single-flavour and GAT-free (AC-A04, `…/_decomposition.md:542-544`) — this story adds no flavour
  and no GAT.
- `crates/happenstance-sync-testkit/src/rules.rs` and `src/registry.rs` — where the rule is defined
  and where it is enumerated.
- `crates/happenstance-sync-testkit/src/fixtures.rs` — `MemorySyncPeerFixture`, the reference
  implementation and the always-on leg.
- The runner from HS-S0109, re-exported at `crates/happenstance-sync/src/lib.rs` — the thing both
  topologies are wirings *of*. Bound on `EventStore` / `SyncPeer` / `IngestStore`, one name of each
  pair per module (CLAUDE.md constraint 4).
- `crates/happenstance-sync/src/peer.rs` — `SyncPeer`, and `:75-80`'s *"Hub-ness is not on the port"*,
  the prose this story converts into a test.
- `crates/happenstance-sync/src/identity.rs` — `Watermark` (`:208-224`) and `EventId`: the version
  vector under test, and the only identity assertions may key on.
- `crates/happenstance-testkit` — `Capability` and `RuleOutcome`, consumed through the sync testkit's
  re-exports, never redeclared.

**Renders surfaces** — **none.** `…/_design.md` records *"N/A — no user-facing surface"* and the
sign-off approves that determination (`:76-125`). There is no surface id to claim. The obligation
this project carries instead is rustdoc and a compiled example on every new public item
(`standards/rust/70-rustdoc-obligations.md`), carried as an AC rather than a footnote.

**Conformance rule(s)** — **one added:** `one_adapter_serves_both_roles`, SY-9's own named rule,
gated on `BOTH_ROLES`, with the registered mutant `HubFlaggedPeer` that fails exactly it and passes
every other rule (CF-1 – CF-4), a provenance naming the Kestrel Rotor vessel, and a `CHANGELOG.md`
entry naming the defect (CF-29, `spec/SPECIFICATION.md:8141-8150`). **One deliberately not added:**
`directional_merge_rules_compose` — SY-10's, kept out by SY-8 and recorded as a finding (Context pack
7).

**Clause(s)** — discharges **SY-9** (its `Rule:` field becomes real and its `(new)` marker becomes
HS-S0112's to drop). Exercises **SY-10** and runs its falsification test without changing its marker.
Constrained by **SY-8** (one relationship per rule), **SY-19** (no cross-node position expression),
**SY-31** (disposition recorded, rule not landed), CF-1 – CF-6 and CF-29. **No `[FROZEN]` clause is
edited.** If SY-10's falsification comes out positive, that is a new decision atom and a re-plan
routed to HS-S0113 — never a marker moved in this diff
(`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`).

**Advances DoD scenario** — initiative **DoD 14**, *replication has an answer on disk*
(`.bklg/from-contract-to-published-library/initiative.md:398-401`), on its topology half: ADR-0027's
answer to *one abstraction or two* stops being a claim in an atom and becomes something a peer can
fail. It is also a hard precondition for the project's own **DoD 4** proof artefact — one suite green
against three peers (`project.md`, *Definition of done* 4) — because from this story on, the suite
the two live peers run at HS-S0111 contains the question SY-9 froze.

## PR boundary

**In this PR**

- `one_adapter_serves_both_roles` in `crates/happenstance-sync-testkit/src/rules.rs`, enumerated in
  `for_each_sync_peer_rule!`, gated on `BOTH_ROLES`, written structurally: one adapter binding, two
  edges, both driven before either completes, assertions over `EventId` sets.
- `MemorySyncPeerFixture::BOTH_ROLES` flipped to supported, with whatever the fixture needs to
  present one local store and two distinct far sides — and the declined-capability instrument
  preserved by a variant fixture in `crates/happenstance-sync-testkit/tests/` that declines it with a
  stated reason.
- `HubFlaggedPeer` in `crates/happenstance-sync-testkit/tests/`, registered in the `Declared` table
  with a per-rule `expect` pin and a non-empty provenance, plus the `CHANGELOG.md` entry.
- The two topology scenarios in `crates/happenstance-sync/tests/`: a hub with **two** spokes whose
  progress diverges, and a symmetric pair (the A—B—C transitive arrangement of E2E-42), both over the
  HS-S0109 runner.
- SY-10's directional-merge exercise and its falsification test, run in the honest direction, with
  the outcome recorded either way.
- The `ScalarWatermark` demonstration: the two-spoke scenario is written so that a scalar watermark
  could not pass it.
- Documentation corrections in `crates/happenstance-sync/src/lib.rs` — read first, edited only where
  this project has made a sentence false, and each deleted paragraph's finding moved into the ADR
  that consumed it beforehand. Rustdoc on every new public item; `# Errors` naming conditions.
- This story's own backlog folder (`_ledger.md`, the implementation report).

**Explicitly not in this PR**

- The runner itself, its bounds and its `!Send` instruments — `send-free-sync-runner` (HS-S0109),
  merged first in this slice.
- `directional_merge_rules_compose` as a suite rule, and any change to SY-10's marker.
- SY-31's `watermark_advances_transactionally_with_a_read_model`, and any `ProjectionStore`
  ingredient on `SyncPeerFixture`.
- Live peers, and any real Durable Object or Neon environment — `durable-object-and-neon-peers`
  (HS-S0111).
- Scoped replication / the confidentiality mechanism (SY-27, SY-28) — deferrals renewed at HS-S0113.
- Any edit to a `[FROZEN]` clause, any `(new)`-marker sweep, any clause repair — `frozen-clause-repairs`
  (HS-S0112).
- Gate-constant and lint-scope edits (`RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, the
  position-literal and changelog lints, the two `wasm32` steps) — HS-S0104's, consumed here.
- Any change to `EventStore`'s, `SyncPeer`'s or `IngestStore`'s trait signature.

```
crates/happenstance-sync/src/**
crates/happenstance-sync/tests/**
crates/happenstance-sync-testkit/**
CHANGELOG.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/hub-and-spoke-and-peer-to-peer-topologies/**
```

**Merge DoD (one line).** `cargo test -p happenstance-sync -p happenstance-sync-testkit
--all-features`, `cargo xtask lints && cargo xtask spec-trace` and `cargo xtask ci --fast` are green,
`one_adapter_serves_both_roles` **runs** (not `Skipped`) against `MemorySyncPeerFixture` in all three
harnesses, and `HubFlaggedPeer` fails exactly it.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **One adapter, two roles, at the same time** | One `let peer = …` binding drives one edge as a hub and another as a symmetric peer, both in flight before either exchange completes, and both complete. No role parameter, no `is_hub`, no `HubPeer`/`SpokePeer` split anywhere in the diff. | `spec/SPECIFICATION.md:6090-6106` (SY-9, `Rule:` and `Rejects:`); `crates/happenstance-sync/src/peer.rs:75-80`; `discover.md:141-152` (the suite-shaped mutant this refuses) |
| **The rule is mounted, not merely defined** | Defined in `src/rules.rs`, enumerated in `for_each_sync_peer_rule!`, therefore emitted by the tokio, blocking and `wasm32` harnesses from one enumeration; `no_orphan_sync_rules` asserts both directions. | `crates/happenstance-testkit/src/registry.rs:93-110`, `:411-435`; `…/sync-testkit-crate-and-rule-registry/spec.md:367`, `:313-315` |
| **`BOTH_ROLES` becomes supported on the oracle, and declining stays exercised** | `MemorySyncPeerFixture` supports the capability so SY-9's rule actually runs on the always-on leg; a variant fixture in the testkit's own `tests/` declines it with a stated reason, so the skip machinery keeps a live instrument and DR-8 is not quietly retired. A fixture that cannot present two distinct far sides over one local store declines honestly. | `…/sync-testkit-crate-and-rule-registry/spec.md:296-298`, `:368`; `crates/happenstance-testkit/src/contract.rs:137-146`, `:359-433`; `project.md`, DR-8 |
| **Two handles, one relationship — SY-8 is not bent** | The rule holds two peer handles onto **one fixture instance** (one isolated backing store), in the sense `SECOND_HANDLE` already carries for the event-store suite. It does not give the port a second peer, and nothing in `SyncPeer` gains a plural. | `spec/SPECIFICATION.md:6070-6086` (SY-8); `CLAUDE.md`, *The rule that matters*; `crates/happenstance-sync-testkit/src/contract.rs` (`type Peer: SyncPeer`, one relationship) |
| **Hub-and-spoke means one hub and at least two spokes** | A one-spoke hub is a peer pair with a label and exercises none of the distinguishing properties. The two spokes' progress diverges, so a scalar watermark cannot pass the scenario — which is the standing defence of `Watermark` being a version vector. | `…/_decomposition.md:745-751` (testing brief); `crates/happenstance-sync/src/identity.rs:208-215`; `crates/happenstance-sync/src/lib.rs:62-66`; `discover.md:154-159` |
| **Peer-to-peer is the transitive arrangement, not just a pair** | A—B—C where A and C never communicate; each syncs only with its neighbour; all three converge on the union of facts, keyed by `EventId`. Forwarding is by what a node *holds*, never by what it originated. | `spec/E2E-CASES.md:1105-1124` (E2E-42); `spec/SPECIFICATION.md:6408-6428` (SY-19, arrival order) |
| **Every assertion is over identity and per-origin watermarks** | Set of `EventId`s each node holds, and watermark advancement per origin `StoreId`. No literal position value, no expression relating two nodes' positions — none can be constructed. | `spec/SPECIFICATION.md:6408-6428` (SY-19), `:7233-7249` (CF-6); `crates/happenstance-sync/src/lib.rs:100-104`; `discover.md`, *Gate: Discover* box 6 |
| **The mutant is mandatory, named and provenanced** | `HubFlaggedPeer` (`Peer::new(is_hub: bool)`) is registered in the `Declared` table, fails exactly `one_adapter_serves_both_roles` at a pinned assertion and passes every other rule; provenance names the vessel that is a hub to nine tablets and a peer to a shore depot at once. CF-1 makes this non-optional the moment the rule exists. | `spec/SPECIFICATION.md:7161-7232` (CF-1 – CF-5), `:6101-6106` (the provenance's source); `crates/happenstance-testkit/tests/mutation_coverage.rs:140-186`, `:325-350`; `…/headline-rules-and-mutant-registry/spec.md:392-394` |
| **SY-10 is exercised at the runner, and the tier divergence is a recorded finding** | A different merge rule in each direction of one edge is demonstrated in `crates/happenstance-sync/tests/`; the clause's `Rule:` field naming a testkit rule is written down for HS-S0112/HS-S0113 rather than satisfied by a differently-shaped rule here. | `spec/SPECIFICATION.md:6110-6128` (SY-10), `:6070-6086` (SY-8, why the frozen clause wins); `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` |
| **SY-10's falsification test runs, and a positive result escalates** | *Implement the hub's rule on a spoke and show the spoke's projections stay correct.* Recorded either way. Correct projections falsify SY-10 → new decision atom, HS-S0113, no marker moved here. Incorrect → `SymmetricWithDeclinedPush` refuted by name, with what it loses stated: adjudication, and a commercial confidentiality boundary. | `spec/SPECIFICATION.md:6112-6117`, `:6120-6128`; `discover.md:126-139`, *Gate: Discover* box 7 |
| **SY-31's disposition is written, not assumed** | The per-peer confirmation watermark stays out of this story: its rule needs a `ProjectionStore` handle no `SyncPeerFixture` has, and PS-9 (quoted inside SY-31) says a generic runner cannot write a read model beside it. Per-edge confirmation state here is the runner's per-peer resume token plus `Watermark` per origin; renewal is HS-S0113's. | `spec/SPECIFICATION.md:6731-6760` (SY-31 and its PS-9 paragraph); `…/_decomposition.md:542-544` (AC-A04, the fixture stays single-flavour and GAT-free) |
| **The module documentation is corrected by reading first** | `lib.rs:52-66` already describes both topologies; AC-007's third clause is verified against the file before any edit. What changes is prose this project has made false — the phase-2-sketch header (`:3-15`) and the *hard part* list (`:98-133`) — and each deleted paragraph's finding is moved into the ADR that consumed it first. | `crates/happenstance-sync/src/lib.rs:3-15`, `:52-66`, `:98-133`; `…/_decomposition.md:499-506`, `:590-599` |
| **The surface is documented as designed** | Rustdoc on every new public item; `# Errors` sections naming conditions rather than error types; the topology example compiles and is run by the gate. `happenstance-sync-testkit` is `publish = false`, so any doctest leaving the package needs RS-62-5's out-of-package harness rather than `cargo test --doc`. | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/62-doctests-and-harnesses.md` (RS-62-5); `…/_decomposition.md:700-707` |

## Data and migrations

**N/A.** This story introduces no storage schema, no on-disk format and no wire-format change. It
adds one conformance rule, one mutant, one fixture capability flip and two integration scenarios.

Three near-misses, named so the section's subject is not mistaken for absent:

- **`Watermark` is not redefined here.** It is already a version vector of `(StoreId,
  SequencePosition)` kept sorted for canonical comparison (`crates/happenstance-sync/src/identity.rs:208-224`).
  This story *depends* on that shape and writes the scenario a scalar could not pass; it does not
  change the type, and collapsing it would be a topology decision disguised as a simplification.
- **No wire movement.** `crates/happenstance-sync/src/wire.rs` is untouched and no derive is added to
  `PushBatch`, `EventGroup` or `ReplicatedEvent` — a `#[derive]` on a public message type *is* a wire
  format and is ADR-0027's by name (`crates/happenstance-sync/src/lib.rs:86-94`).
- **No per-peer state gains a persistent home.** SY-31's reserved `ProjectionId` is exactly where
  persistent per-peer confirmation state *would* live, and Context pack 9 records why this story does
  not put it there. Resume tokens remain the caller's, owned and opaque, as HS-S0109 landed them.

## Acceptance criteria

Each criterion is written from the intent of a persona this initiative names, crossing the whole
slice rather than one function. The consumers here are the **application author** running a fleet
(journey *Choose a contract before a database*, backbone A5 — *run a fleet*, `…/_storymap.md:44`),
the **adapter author** who writes the two live peers one story later (journey *Learn when you are
finished*), and the **constrained-runtime developer** whose node is a hub and a spoke at once
(journey *Event-source at the edge without hand-rolling it*) — all carried from
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` and
named at `.bklg/from-contract-to-published-library/initiative.md:227-250`.

Test paths below are the paths the implementer creates; only the **Anchors** are required to exist
in the tree today.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **One node is a hub and an equal at the same time, and nothing in the library asks it to choose.** GIVEN an application author deploying the Kestrel Rotor arrangement — one vessel store that is authoritative to nine technician tablets over ship's wifi and a symmetric peer to a shore depot over satellite — WHEN they construct **one** adapter value and attach it to both edges, THEN both edges exchange and both complete, with no `is_hub` argument, no role parameter, no `HubPeer`/`SpokePeer` split and no second adapter instance anywhere on the path. The two edges are **both in flight before either exchange completes**: a test that drives one edge to quiescence and then the other satisfies the words and not the claim (SY-9, `spec/SPECIFICATION.md:6090-6106`). | `crates/happenstance-sync-testkit/src/rules.rs::one_adapter_serves_both_roles` — one `let peer = …` binding, two peer handles onto **one** fixture instance, both edges' futures created and polled before either resolves (the cold-future interleaving shape of `.kb/playbooks/testing-interleavings-with-cold-futures.md`, never a sleep or a thread race); assertions over the `EventId` set each far side holds. Executed against the oracle through `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs`. |
| AC-002 | **A green run means the whole bar was applied, including the newest question.** GIVEN an adapter author who invokes `sync_peer_conformance!` and reads "all passed", WHEN SY-9's rule exists in the `rules` module, THEN it is enumerated in `for_each_sync_peer_rule!` and therefore emitted by **all three** harnesses — tokio, blocking and `wasm32` — from one enumeration; a rule defined and not enumerated fails the meta-test in both directions and names itself. The author adds nothing per harness to receive it. | `crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules` (both directions, the retargeted shape of `crates/happenstance-testkit/src/registry.rs:411-435`), plus the rule's presence in each of the three harness outputs: `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs`, `…_blocking.rs`, `…_wasm.rs`. |
| AC-003 | **The headline question is actually asked of the always-on peer, and declining it still costs a printed line rather than a vanished test.** GIVEN the reference oracle `MemorySyncPeerFixture`, WHEN the suite runs, THEN `one_adapter_serves_both_roles` reports **Ran**, not `Skipped` — `BOTH_ROLES` is supported, because a fixture that can present one local store with two distinct far sides has no honest reason to decline. AND GIVEN a fixture that genuinely cannot, WHEN it declares `BOTH_ROLES` as `declined("…")`, THEN the rule is still emitted as a test, returns `RuleOutcome::Skipped`, and the harness prints the fixture's own stated reason — so DR-8's instrument survives the flip instead of being retired by it. | `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs` — assert the rule's outcome for the reference fixture is `Ran`; plus `crates/happenstance-sync-testkit/tests/both_roles_declined.rs::declining_both_roles_reports_the_fixtures_stated_reason`, a variant fixture that declines with a reason and is asserted to produce a `Skipped` carrying that exact string (the pattern at `crates/happenstance-testkit/src/contract.rs:359-433`). |
| AC-004 | **A fleet is a hub and *many* spokes, and the library can tell them apart.** GIVEN an application author running one authoritative store with at least **two** spokes whose progress has diverged — one caught up, one far behind, each holding facts the other has never seen — WHEN the runner drives every edge to quiescence, THEN each spoke holds the union of what the hub holds for it, the hub holds both spokes' facts, and the hub's `Watermark` advances **per origin `StoreId`** rather than as one number. Collapsing `Watermark` to a scalar must make this scenario fail: a one-spoke hub is a peer pair with a label and proves nothing (`crates/happenstance-sync/src/identity.rs:208-215`). | `crates/happenstance-sync/tests/hub_with_two_spokes.rs::divergent_spokes_need_a_version_vector` — three stores over the HS-S0109 runner; assertions over `EventId` sets and per-origin watermark entries only. The `ScalarWatermark` demonstration is the same test read as a negative control, and the implementation report records that a scalar could not pass it. |
| AC-005 | **Peers converge on what they hold, not on what they originated — even when they never meet.** GIVEN the symmetric arrangement A—B—C in which A and C never communicate and each node syncs only with its neighbour, WHEN every edge is driven with the same adapter type in both directions, THEN all three converge on the union of facts keyed by `EventId`, C receives A's events by way of B, and no node's forwarding decision depends on which store originated a fact (E2E-42, `spec/E2E-CASES.md:1105-1124`). Arrival order is not a property any assertion may rely on (SY-19). | `crates/happenstance-sync/tests/transitive_convergence.rs::a_and_c_converge_without_ever_meeting` — over the HS-S0109 runner; assertions on held `EventId` sets at each node, executed in more than one edge order so the outcome cannot be an artefact of the order chosen. |
| AC-006 | **The new rule can fail, and the thing it fails is written down and plausible.** GIVEN the adapter author trusting the suite to discriminate, WHEN `HubFlaggedPeer` — `Peer::new(is_hub: bool)`, SY-9's own named rejection — is run against the suite, THEN it fails **exactly** `one_adapter_serves_both_roles` at a pinned assertion and passes every other rule; it is registered in the `Declared` table with a non-empty provenance naming the vessel that is a hub to nine tablets and a peer to a shore depot at once; and `CHANGELOG.md` carries an entry naming the **defect the rule detects**, not the rule (CF-1 – CF-4, CF-29, `spec/SPECIFICATION.md:7161-7232`, `:8141-8150`). A rule with no registered mutant is decorative and CF-1 makes it a build failure. | `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` — the `Declared` entry with its per-rule `expect` pin, in the shape of `crates/happenstance-testkit/tests/mutation_coverage.rs:140-186`, `:324-350`; the wrong peer itself under `crates/happenstance-sync-testkit/tests/`; the changelog assertion via `cargo xtask lints` (HS-S0104's changelog-per-rule lint, already pointed at this crate). |
| AC-007 | **A hub and a spoke may merge differently across one edge, and the runner permits it without knowing which end is which.** GIVEN an application author whose hub adjudicates across every log while a spoke sees only its own, WHEN they attach a different merge policy to each direction of the **same** edge, THEN both directions run and the runner exposes no role parameter to make that possible — policy is per direction, per edge, and never a property of the adapter's type. The exercise lands at runner grain, in `crates/happenstance-sync/tests/`, and **not** as a suite rule: SY-8 is `[FROZEN]` that a rule needing two peer handles is testing the runner, SY-10 is `[PROVISIONAL]`, and the frozen clause wins. The finding — *SY-10's `Rule:` field names a testkit rule and points at the wrong tier* — is recorded verbatim in the implementation report and handed to `frozen-clause-repairs` and `clause-arithmetic-and-deferral-renewals`; `directional_merge_rules_compose` keeps its `(new)` marker and SY-10's marker is untouched here. | `crates/happenstance-sync/tests/directional_merge_rules.rs::a_different_rule_in_each_direction_of_one_edge` (SY-10, `spec/SPECIFICATION.md:6110-6128`); the recorded finding in `…/hub-and-spoke-and-peer-to-peer-topologies/implementation-report.md`; `cargo xtask spec-trace` green with SY-10's marker and `Rule:` line unchanged. |
| AC-008 | **The clause that is only provisional gets its experiment run, and an inconvenient answer stops the story instead of being absorbed by it.** GIVEN SY-10's own falsification test — *"implement the hub's rule on a spoke and show the spoke's projections stay correct"* (`spec/SPECIFICATION.md:6112-6117`) — WHEN it is run in the honest direction, THEN the outcome is recorded either way. If the spoke's projections stay correct, SY-10 is falsified: **stop**, record the finding, raise a new decision atom and route the marker change to `clause-arithmetic-and-deferral-renewals` — never relax a clause in this diff. If they do not, `SymmetricWithDeclinedPush` is refuted **by name**, and what it loses is stated: the hub's adjudication (it holds every log; the spoke does not) and, in Kestrel Cold Chain, a commercial confidentiality boundary — Scottish subcontractors not holding Yorkshire's parts pricing (`spec/SPECIFICATION.md:6120-6128`). "We did not get to it" is not an outcome. | `crates/happenstance-sync/tests/sy10_falsification.rs::the_hubs_merge_rule_applied_on_a_spoke` — the spoke's projections asserted correct or incorrect, with the assertion written before the result is known; the outcome, either way, written into the implementation report and cited by the ledger row. |
| AC-009 | **A reader of the crate meets a replication library, not a phase-2 sketch that has quietly become one.** GIVEN an application author landing on `happenstance-sync`'s front page on docs.rs, WHEN they read the module documentation, THEN both topologies are described with the reason `Watermark` is a version vector attached (verified against `crates/happenstance-sync/src/lib.rs:52-66` **before** any edit is made — AC-007's third clause may already be satisfied there), every sentence this project has made false is corrected, every new public item carries rustdoc with an `# Errors` section naming conditions rather than error types, and a compiled example shows one adapter on two edges. AND every paragraph deleted has had its **finding** moved into the ADR that consumed it first — the findings are not the same as the `todo!()`s, and this crate's honesty about what the sketch proved is the thing most easily lost in a tidy-up (`…/_decomposition.md:590-599`). | Read-first evidence: a diff over `crates/happenstance-sync/src/lib.rs` showing `:3-15` and `:98-133` treated and `:52-66` justified line by line; `cargo test -p happenstance-sync --doc` for the compiled example; `cargo xtask ci --fast`'s docs step with `-D warnings` for `standards/rust/70-rustdoc-obligations.md`; for any doctest that lands in `happenstance-sync-testkit` (`publish = false`), RS-62-5's out-of-package harness rather than `cargo test --doc`. |

**Coverage of the traced project AC.** Project **AC-007** — *"Hub-and-spoke and peer-to-peer are each
exercised, one adapter type serves both roles simultaneously on different edges (SY-9,
`spec/SPECIFICATION.md:6090-6106`), and `crates/happenstance-sync/src/lib.rs`'s module documentation
no longer describes only one"* (`project.md:214-218`) — has three clauses and each is a separate row,
because each is green while the others fail. *Both topologies exercised* is AC-004 (hub, two spokes)
and AC-005 (symmetric, transitive). *One adapter type in both roles simultaneously* is AC-001, made
non-decorative by AC-002 (it actually runs), AC-003 (it runs against the oracle rather than skipping)
and AC-006 (something can fail it). *The module documentation* is AC-009. AC-007 and AC-008 are the
SY-10 obligations this story inherits by standing where the topology question lives — they extend the
project AC rather than duplicating it, and both are written so that a finding, not a silent pass, is
the deliverable.

## Interaction quality

**This story renders no surface, and that is a signed-off determination rather than an omission.**
`…/_design.md` records *"N/A — no user-facing surface"* across Items, Signatures, Placement, States,
Anti-patterns and The doctest, and the sign-off approves **the no-surface determination itself**
(`…/_design.md:76-125`). There is no route, DOM node, TUI pane or screenshot, so the COMPOSITION
family has no signed-off surface ids to bind to and no pixel density budget to meet. Nothing is
dropped: the obligation the design stage transfers in its place is the **public API and suite-output
surface**, and it is carried as table rows above, never as prose here. Every invariant below is an
`AC-###` row; this section only says which row carries it and how it is verified.

**STATE family (library and suite analogues).**

- *In-place rather than a context jump* — **AC-001**. Becoming a hub is an edge attachment on a value
  the author already holds. It does not relocate them into a different type, a different trait or a
  second adapter instance. This is the row that fails if hub-ness migrates onto the constructor.
- *Non-occlusion* — **AC-002** and **AC-003**. The newest question is visible in every harness's
  output rather than hidden behind an enumeration someone forgot to extend (AC-002), and a declined
  capability prints its own stated reason instead of the rule disappearing from the binary (AC-003).
  A `Skipped` that looks like a pass is the occlusion this project's DR-8 exists to prevent.
- *Preserved state across a context change* — **AC-004** and **AC-005**. Per-origin watermark
  advancement and held `EventId` sets survive every edge being driven in any order; nothing is reset
  by a neighbour's progress, and AC-005 is run in more than one edge order for exactly that reason.
- *Reversibility* — **AC-007** and **AC-008**. The directional-merge exercise and the falsification
  test are both written so that an inconvenient result routes outward (a recorded finding, a new
  decision atom, HS-S0112/HS-S0113) instead of being absorbed by a quiet edit here. A clause relaxed
  in this diff is the irreversible move.
- *Reachability* — **AC-002** and **AC-009**. Reachable from every runtime that needs it (all three
  harnesses, including `wasm32`, from one enumeration) and reachable from the documentation (a
  compiled example showing one adapter on two edges, executed by the gate).

**COMPOSITION family (the API- and output-surface obligation the design transferred).**

- *Presentation exists at all* — **AC-009**. The library analogue of "a control carries real composed
  presentation, not bare markup" is a public item with real rustdoc, an `# Errors` section naming
  conditions, and a compiled example. A suite that type-checks with no docs is the unstyled render:
  every structural assertion passes and it is unusable. **AC-006** carries the same invariant for the
  suite's *output* — a mutant with an empty provenance is a failure message with no composition.
- *Placement* — **AC-002**, via the Integration contract's mount point. The rule is composed into
  `for_each_sync_peer_rule!`, the suite's composition root; a rule reachable only from its own test
  file is unmounted, which is precisely `no_orphan_sync_rules`' subject.
- *Transience* — **AC-003**. Which parts of the output are persistent chrome and which are revealed
  on demand: a rule's line is persistent (it appears whether it ran, skipped or failed), and the
  fixture's stated reason is revealed with the `Skipped`. Nothing is opened-on-demand and nothing
  vanishes — a vanished rule is the transience failure.
- *Density budget, with its real numbers* — **AC-004**. The only density figure this story has is a
  topology's minimum size, and it is **one hub and at least two spokes with divergent progress**, and
  **three nodes** for the symmetric case (AC-005). One spoke is under budget: a `ScalarWatermark`
  passes it, and the scenario stops discriminating.
- *Hierarchy* — **AC-001** and **AC-007**. What is subordinate to what: policy is subordinate to the
  **edge and direction**, never to the adapter's type; and the suite tier is subordinate to the
  frozen clause (SY-8) rather than to the provisional one (SY-10) when the two disagree.
- *Named anti-patterns* — carried from `discover.md`'s four wrong implementations, each bound to a
  row: `HubFlaggedPeer` → AC-001/AC-006, `SymmetricWithDeclinedPush` → AC-008, *two scenarios with
  two separately-constructed adapters* → AC-001 (the "simultaneously" clause), and `ScalarWatermark`
  → AC-004.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | A fixture genuinely cannot present one local store with two distinct far sides. | It declares `BOTH_ROLES` as `declined("…")` with its own reason. `one_adapter_serves_both_roles` is still emitted as a test, returns `RuleOutcome::Skipped`, and the harness prints that reason. It never vanishes from the binary and never reports a pass (`crates/happenstance-testkit/src/contract.rs:359-433`). This is the legitimate answer for a live peer at `durable-object-and-neon-peers`, and it is why AC-003 keeps a declining variant in the tree. |
| EC-002 | One of the two edges fails mid-rule while the other is still in flight. | The rule **fails**, and its message names which edge and which role failed. It must not report a pass because the other edge completed, and it must not be satisfiable by driving the edges sequentially and reporting the last one. The rule's own assertion order is written so that a partial completion cannot look like a success. |
| EC-003 | SY-10's falsification test comes out **positive** — the hub's merge rule on a spoke leaves the spoke's projections correct. | **Stop.** Record the finding, raise a new decision atom, and route the marker change to `clause-arithmetic-and-deferral-renewals` (HS-S0113). Do not edit SY-10's marker, soften its text, or delete its rule reference in this diff (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`). The story's AC-008 row is satisfied by the *recorded outcome*, not by a particular result. |
| EC-004 | An assertion cannot be expressed without comparing positions across two nodes. | That is a defect in the assertion, never in SY-19. Re-key it on the `EventId` set the node holds or on per-origin watermark advancement. `SequencePosition` carries no origin and no expression relating two peers' positions can be constructed (`spec/SPECIFICATION.md:6408-6428`); the position-literal lint fails the gate if one is written anyway (CF-6). |
| EC-005 | Flipping `MemorySyncPeerFixture::BOTH_ROLES` removes the instrument HS-S0103's own AC-006 cited as evidence. | The declining variant fixture lands **in the same commit** as the flip, and the implementation report names it as the replacement instrument. A green run whose declined-capability exercise has silently disappeared is a regression `redkiln verify --grain story` is entitled to catch on the base-branch comparison, and it retires DR-8 by accident. |
| EC-006 | A peer errors on one edge of the two-spoke scenario. | The runner's contract from `send-free-sync-runner` applies unchanged and is **not** redefined here: the other spokes keep progressing, the failure is surfaced identified with the peer it belongs to, and that peer's resume token is not advanced past what was actually ingested. This story adds no new error taxonomy and widens no error bound. |
| EC-007 | The implementer is tempted to land `directional_merge_rules_compose` as a suite rule so SY-10's `Rule:` line resolves. | Refuse. SY-8 is `[FROZEN]` and SY-10 is `[PROVISIONAL]`; the frozen clause wins, the exercise stays at runner grain (AC-007), and the tier divergence is handed on as a finding. A differently-shaped rule wearing the clause's name is worse than a `(new)` marker, because `spec-trace` would then read as satisfied. |
| EC-008 | A paragraph of `crates/happenstance-sync/src/lib.rs` is about to be deleted and its finding has no home yet. | Do not delete it. Move the finding into the ADR that consumed it first, then delete. If no ADR consumed it, the paragraph is still true and stays (`…/_decomposition.md:590-599`). |

## Non-functional

| id | requirement | check |
| --- | --- | --- |
| NF-001 | No new third-party dependency enters `happenstance-sync`'s or `happenstance-sync-testkit`'s non-dev graph; both crates still build on the MSRV and for `wasm32-unknown-unknown`, and `tokio` stays a dev-dependency. | `cargo xtask ci --fast`; `cargo xtask wasm`; CI's `msrv` job; `standards/rust/50-dependency-hygiene.md`. |
| NF-002 | No assertion anywhere in this PR names a literal position value, and no expression relates two nodes' positions. Three-node topologies are where the temptation peaks and SY-19 says none can be constructed. | `cargo xtask lints` (the position-literal lint, pointed at the sync suite by HS-S0104); review of the diff. |
| NF-003 | `EventStore`, `SyncPeer` and `IngestStore` trait signatures are byte-identical before and after; `happenstance-core` is untouched; `publish = false` still reads `false` on both sync crates. | `git diff` over `crates/happenstance-core/src/`, `crates/happenstance-sync/src/peer.rs` and `src/ingest.rs`; the `cargo package --list` assertions already inside the gate. |
| NF-004 | `SyncPeerFixture` stays **single-flavour and GAT-free** (AC-A04, `…/_decomposition.md:542-544`). `BOTH_ROLES` changes value on one fixture; the trait gains no associated type, no `ProjectionStore` ingredient and no second flavour. | Review against `…/sync-testkit-crate-and-rule-registry/spec.md:281-306`; `cargo xtask ci --fast`. |
| NF-005 | The rule's two-edge interleaving is **deterministic**: no `sleep`, no wall-clock timing, no thread race. Both futures are created and polled before either resolves, using the cold-future technique the workspace already settled on. | `.kb/playbooks/testing-interleavings-with-cold-futures.md`; the rule run repeatedly in CI (`cargo test -p happenstance-sync-testkit --all-features`) without flake. |
| NF-006 | `cargo clippy --workspace --all-targets -- -D warnings` is clean with no new `#![allow]`, and the scoped `#![allow(clippy::todo)]` removed earlier in this project is not reintroduced. | `cargo xtask ci --fast`. |
| NF-007 | `cargo xtask spec-trace` stays green: SY-9's `Rule:` line resolves to a real symbol after this PR, and **no `[FROZEN]` clause text is edited** — not SY-8, not SY-9, not SY-19. SY-10's and SY-31's markers are unchanged. | `cargo xtask spec-trace`; a diff review over `spec/SPECIFICATION.md` showing zero clause-text changes. |
| NF-008 | The new rule is emitted by all three harnesses from **one** enumeration; no harness gains a hand-maintained rule list, and the `wasm32` harness check compiles it rather than skipping it. | `crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules`; `cargo xtask wasm` (the sync-harness check HS-S0104 added, `xtask/src/main.rs:203-283`). |
| NF-009 | The suite decodes no payload byte on any path added here; `Event::data` and `Event::metadata` are compared as opaque `Bytes` or not at all (DR-5). | Review of the diff; the testing brief's *Fixtures and seams* row on payload bytes (`…/_decomposition.md:802-811`). |

## Implementation notes (non-prescriptive)

Constraints with the answer left open, per the architecture brief's own *Non-prescriptive
implementation notes* (`…/_decomposition.md:488-521`).

- **How the fixture presents two far sides is open.** Two receiver stores behind one fixture
  instance, or one receiver addressed by two `StoreId`s — both are admissible. What is not open is
  that it is **one fixture instance** (one isolated backing store, two handles), because that is what
  keeps the rule inside SY-8. Record which alternative lost and why.
- **How "both in flight" is expressed is open, but not whether.** Cold futures polled alternately,
  `futures::join!` over two un-awaited exchange futures, or a hand-written poll loop — pick one and
  say why at the test. The prohibition is on the sequential version, and on anything that depends on
  timing to be true.
- **How a directional merge policy is spelled is ADR-0027's, not this story's.** A closure per
  direction, a policy value on the edge, or whatever the atom landed. AC-007 constrains the
  *property* — per direction of one edge, never per adapter type — and deliberately not the spelling.
- **Where the scenarios live.** `crates/happenstance-sync/tests/` with one file per arrangement is
  the least surprising placement given the crate's existing test list, but the requirement is the
  tier (runner grain, not suite), not the filename.
- **The declining variant fixture's shape.** Reuse the reference fixture and override the one
  constant if the crate's shape allows it; a wholly separate type is fine too. What matters is that
  it is a *plausible* decline — a peer that cannot address two far sides — and not a fixture that
  declines in order to make a test exist.
- **The mutant's home.** `crates/happenstance-sync-testkit/tests/` beside the mutants
  `headline-rules-and-mutant-registry` landed, in whatever module layout that story chose. Do not
  invent a second mutant-registration concept.
- **What to do with `lib.rs`'s `# Status: a phase-2 sketch` header.** Rewriting it, replacing it with
  a "what this crate now is" section, or deleting it once its findings are relocated are all
  admissible. Decide it with HS-S0109 in the same context — the two stories edit adjacent prose in
  one file by design — and record the decision in the report.

## Tests and CI (merge gate)

Grounded in the project testing brief's *Merge-gate commands* (`…/_decomposition.md:775-800`), in the
order a story actually runs them, story grain first.

| tier | command / path | proves |
| --- | --- | --- |
| Story grain (affected) | `cargo xtask affected --base main` | Only what this diff could break; the grain `.redkiln/config.yaml`'s `verify:` block wires to `story`. |
| Static / lints | `cargo xtask lints` | NF-002 (no position literal in the sync suite), NF-006, and CF-29's changelog-per-rule check for AC-006. |
| Static / trace | `cargo xtask spec-trace` | NF-007: SY-9's `Rule:` resolves; SY-10's and SY-31's markers and text unchanged; no `[FROZEN]` clause edited. |
| Unit (suite) | `crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules` | AC-002 and NF-008: the rule is enumerated, in both directions, and reaches all three harnesses from one list. |
| Unit (mutants) | `crates/happenstance-sync-testkit/tests/mutation_coverage.rs` | AC-006: `HubFlaggedPeer` fails exactly `one_adapter_serves_both_roles` at a pinned assertion, passes every other rule, and carries a non-empty provenance (CF-1 – CF-4). |
| Suite (oracle, three harnesses) | `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs`, `…_blocking.rs`, `…_wasm.rs` | AC-001 and AC-003: the rule **Ran** — not `Skipped` — against `MemorySyncPeerFixture` in every harness. |
| Suite (declined path) | `crates/happenstance-sync-testkit/tests/both_roles_declined.rs` | AC-003's second half and EC-001/EC-005: a declined `BOTH_ROLES` still emits a test and prints the fixture's stated reason. |
| Integration (native) | `crates/happenstance-sync/tests/hub_with_two_spokes.rs` | AC-004: one hub, two spokes with divergent progress, per-origin watermark advancement; the arrangement a `ScalarWatermark` cannot pass. |
| Integration (native) | `crates/happenstance-sync/tests/transitive_convergence.rs` | AC-005: A—B—C converge on what each holds, in more than one edge order (E2E-42). |
| Integration (native) | `crates/happenstance-sync/tests/directional_merge_rules.rs` | AC-007: a different merge rule in each direction of one edge, at runner grain, with no role parameter. |
| Integration (native) | `crates/happenstance-sync/tests/sy10_falsification.rs` | AC-008: SY-10's own experiment run in the honest direction, with the assertion written before the result is known. |
| Doc | `cargo test -p happenstance-sync --doc` | AC-009's compiled example — one adapter, two edges. For any doctest landing in `happenstance-sync-testkit` (`publish = false`), RS-62-5's out-of-package harness instead (`standards/rust/62-doctests-and-harnesses.md`). |
| Package run | `cargo test -p happenstance-sync -p happenstance-sync-testkit --all-features` | The whole story's test surface in one command, the brief's third line. |
| Gate (`wasm32`) | `cargo xtask wasm` | NF-008's `wasm32` half: the sync harness check compiles the new rule rather than skipping it (`xtask/src/main.rs:203-283`, extended by HS-S0104). |
| Gate (project ceiling) | `cargo xtask ci --fast` | `integration_scoped` — fmt, clippy `-D warnings`, tests, the wasm32 steps, docs with `-D warnings` (AC-009's rustdoc bar), `spec-trace`, packaging assertions. This project is not terminal, so the whole `cargo xtask ci` stays `closeout-and-durable-audience`'s. |
| Ledger | `_ledger.md` rows cite **which** command produced each row's evidence | `.redkiln/config.yaml`'s `require_ledger: true`; a green `ci --fast` proves the gate passed, never on its own that AC-001's simultaneity is the thing that ran. |

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation in this PR |
| --- | --- | --- |
| **The easy version ships instead of the right one.** Two scenarios, each constructing its own adapter, both green — every word of AC-007 reads satisfied and SY-9's claim is never exercised. | The sequential version is easier to read, easier to debug, and looks identical in a diff review. It is `discover.md:141-152`'s suite-shaped mutant. | AC-001 states the structural requirement in the criterion itself (one binding, both edges in flight before either completes) and NF-005 forbids the timing-dependent shortcut; the ledger row quotes it verbatim, so a sequential implementation cannot be flipped `satisfied: true` honestly. |
| **Flipping `BOTH_ROLES` retires DR-8 by accident.** HS-S0103's AC-006 cites the reference fixture *declining* `BOTH_ROLES` as its evidence; this story makes that fixture support it. | `redkiln verify` compares against the base branch, and a silently-vanished declined-capability exercise is exactly the regression it is entitled to catch. | EC-005 requires the declining variant to land in the same commit; AC-003 carries both halves — Ran on the oracle **and** Skipped-with-reason on the variant — so neither half can be satisfied alone. |
| **`ScalarWatermark` arrives as a simplification.** With one spoke, a scalar watermark is all the tests ever need, and collapsing the version vector reads as tidying. | It is a topology decision disguised as a type simplification, and the crate says so at `crates/happenstance-sync/src/lib.rs:62-66`. | AC-004's minimum wiring is one hub and **two** spokes with divergent progress, and the implementation report records that a scalar could not pass it. |
| **SY-10's falsification comes out positive mid-story.** The experiment is real and its result is not known in advance. | A positive result invalidates a `[PROVISIONAL]` marker, which is a decision-record change, not a code change — and the cheapest local move is to quietly not run the experiment. | AC-008 makes the *recorded outcome* the deliverable rather than a particular result; EC-003 names the escalation (new atom, HS-S0113) so it is a written route rather than a judgement call at 5pm. |
| **Two peer handles read as bending SY-8.** A reviewer who reaches SY-8 before SY-9 will see a rule holding two handles and call it a violation. | Both clauses are `[FROZEN]` and the reconciliation is in the specification's own text, not in this spec. | Context pack 4 states the reconciliation (one fixture instance, two handles, the `SECOND_HANDLE` sense) and the *Behavior and interfaces* row cites SY-8's line range beside SY-9's; the PR body should carry the same two sentences. |
| **Slice-mate collision in `lib.rs`.** HS-S0109 corrects the runner prose and this story corrects the topology prose, in the same file, in the same slice. | Adjacent lines, one context, two stories — the classic double-edit that loses a paragraph. | The slice is implemented in one context by design; HS-S0109 leaves `:52-66` to this story's AC-009 and this story leaves the runner promise at `:42-50` to HS-S0109. EC-008 governs anything either one deletes. |
| **Findings deleted with the prose that held them.** Implementing the project makes most of `src/lib.rs:98-133` untrue, and deleting it is the obvious tidy. | The sketch landed nine phases early *to produce those findings*; the type-checker transcript at `:126-133` is the most useful thing the crate produced. | AC-009's final clause, EC-008, and the architecture brief's own warning (`…/_decomposition.md:590-599`). Move the finding into the consuming ADR first, or leave the paragraph. |
| **Coupling to HS-S0109's runner shape.** The runner's concrete shape is deliberately open at HS-S0109's spec exit; all four scenarios here are written against it. | The scenarios cannot be written until the shape exists, and a shape change underneath them rewrites four files. | Same slice, one context, HS-S0109 merges first (`…/_storymap.md:132-133`); the scenarios bind on behaviour — fan-out, per-peer tokens, no role parameter — which HS-S0109's AC-001/AC-009 fix regardless of shape. |
| **Coupling to ADR-0027's merge answer.** AC-007's directional policy is spelled however the atom spelled it. | The atom is in slice 1 and lands long before this story, but a late amendment would land under AC-007. | `depends_on` names it explicitly; the implementation notes make the *spelling* non-prescriptive and hold only the property, so a different spelling costs a line rather than a redesign. |

## Dependencies

**Blocks on** (must be merged first; the story map's computed order, `…/_storymap.md:132-133`):

- `send-free-sync-runner` (HS-S0109) — supplies the runner that both topologies are wirings *of*, its
  per-peer resume tokens and its `EventStore`/`SyncPeer`/`IngestStore` bounds. Every scenario in
  AC-004, AC-005, AC-007 and AC-008 is driven through it, and its AC-009 (*hub-ness is an edge; the
  runner exposes no role parameter*) is the shape AC-001 then proves at the port. Same slice, one
  context; this story is the slice's second and last row.
- `adr-0027-merge-compensation-and-message-set` (HS-S0099) — answers whether hub-and-spoke and
  peer-to-peer are one abstraction or two, and how a merge rule is expressed. **This story builds and
  tests whichever shape the atom landed; it adjudicates nothing** (`discover.md`, *Questions* 1).

Transitively also `sync-testkit-crate-and-rule-registry` (HS-S0103) — `SyncPeerFixture`,
`for_each_sync_peer_rule!` and the `BOTH_ROLES` constant this story flips —
`gate-mounts-for-the-sync-suite` (HS-S0104) — the gate constants and the two `wasm32` steps — and
`headline-rules-and-mutant-registry` (HS-S0105) — the `Declared` registry and its meta-tests that
AC-006's mutant joins.

**Unlocks:**

- `durable-object-and-neon-peers` (HS-S0111) — DoD 4's proof artefact. From this story on, the suite
  the two live peers run **contains** SY-9's question, so each is asked whether one of its adapters
  serves both roles — or declines `BOTH_ROLES` honestly with a stated reason.
- `frozen-clause-repairs` (HS-S0112) — inherits SY-9's `(new)` marker, now droppable because the rule
  exists, and AC-007's recorded finding about SY-10's `Rule:` field naming the wrong tier.
- `clause-arithmetic-and-deferral-renewals` (HS-S0113) — inherits AC-008's falsification outcome, the
  SY-31 disposition recorded in Context pack 9, and the SY-27/SY-28 deferrals this story names and
  does not build.

## Anchors (progressive disclosure)

Everything load-bearing that is *not* in the Context pack. Link, open on demand, do not paste in
bulk. Every path below exists in the tree today; the test paths in the acceptance table do not, and
that is the difference between an anchor and a deliverable.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | SY-9 (`:6090-6106`) is the frozen claim **and** specifies the rule's mechanism and its `Rejects:` mutant; SY-8 (`:6070-6086`) is why only this rule sits in the suite and the scenarios do not; SY-10 (`:6110-6128`) carries the directional-merge requirement, its falsification test and the confidentiality rationale; SY-19 (`:6408-6428`) is why no assertion may name a position; SY-31 (`:6731-6760`) is the per-peer watermark this story deliberately leaves alone; CF-1 – CF-6 (`:7161-7249`) and CF-29 (`:8141-8150`) are the mutant, provenance and changelog obligations. | Before writing the rule, and again before writing each scenario's assertions. Re-read SY-8 beside SY-9 before defending the two-handle shape in review. | AC-001, AC-004, AC-005, AC-006, AC-007, AC-008 |
| `spec/E2E-CASES.md` | E2E-42 (`:1105-1124`) is the A—B—C transitive arrangement AC-005 implements, stated as observable behaviour — forwarding by what a node *holds*, not what it originated. | Before writing the symmetric scenario, so it is the specified arrangement rather than a two-node pair with a third bolted on. | AC-005 |
| `crates/happenstance-sync/src/peer.rs` | The port itself, and the prose this story converts into a test: *"a Durable Object that is a spoke to a cloud estate store and a hub to 138 tablets is one peer wearing each hat"* (`:75-80`), plus the one-relationship paragraph and the non-`async` `limits()` reasoning. | Before the first line of the rule, to see what the port already promises and must not gain. | AC-001, AC-007 |
| `crates/happenstance-sync/src/identity.rs` | `Watermark` as a version vector of `(StoreId, SequencePosition)` kept sorted (`:208-224`), and `EventId`. These are the only identities assertions may key on, and the version-vector shape is what AC-004's two divergent spokes exist to defend. | Before writing any convergence assertion, and before anyone proposes simplifying the type. | AC-004, AC-005 |
| `crates/happenstance-sync/src/lib.rs` | The documentation surface AC-009 acts on: the phase-2-sketch header (`:3-15`), the topology paragraphs that may already satisfy AC-007's third clause (`:52-66`), and the *hard part, stated honestly* list whose bullets this project has answered (`:98-133`). | **Read before editing**, not after — the architecture brief says the file may already be right. Open again before deleting any paragraph. | AC-009 |
| `crates/happenstance-sync/src/memory.rs` | `MemorySyncPeer`, the oracle every native scenario is built from, and the source of the fixture the suite runs against. | When choosing what each node in a three-node arrangement actually is. | AC-004, AC-005 |
| `crates/happenstance-sync/src/ingest.rs` | `IngestStore`'s real shape — `ingest`, `holds`, `watermark`, `store_id` — and its idempotence contract. `holds` and `watermark` are how a scenario asks "what does this node have" without touching a position. | When writing convergence assertions, before reaching for anything numeric. | AC-004, AC-005 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_*_rule!` and `no_orphan_*` in their settled form (`:93-110`, `:411-435`) — the enumeration shape `happenstance-sync-testkit` mirrors, and the reason an unenumerated rule silently never runs. | At mount time, when adding the rule to the enumeration. | AC-002 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability`, `declined(…)`, and the declined-capability machinery (`:137-146`, `:359-433`) that makes a `Skipped` print the fixture's own reason. The pattern AC-003's variant fixture must match rather than reinvent. | Before flipping `BOTH_ROLES` and before writing the declining variant. | AC-003 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The `Declared` registry in full — the struct (`:141-186`), the per-rule `expect` pin, the never-empty `provenance`, and the live entries (`:324-350`) showing what a plausible provenance reads like. | Before registering `HubFlaggedPeer`; the format is copied, not invented. | AC-006 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture`, the reference implementation whose capability declarations `MemorySyncPeerFixture` mirrors — the shape of "the always-on leg that declines nothing it can honestly do". | When flipping the oracle's `BOTH_ROLES` and deciding what the fixture must present. | AC-003 |
| `.kb/playbooks/testing-interleavings-with-cold-futures.md` | The workspace's settled technique for asserting two operations are genuinely concurrent without a sleep or a thread race. NF-005 is this playbook, and it is what makes AC-001's *simultaneously* mechanical rather than aspirational. | Before writing the rule's two-edge drive loop. | AC-001 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The three-part repair form and, more importantly here, the boundary: what you may do to a clause you disagree with without editing it. EC-003 and AC-007's finding both route through this. | The moment SY-10 or its `Rule:` field looks wrong — before touching `spec/SPECIFICATION.md`. | AC-007, AC-008 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted atom behind CF-1 – CF-4: a rule with no registered mutant is decorative, and the mutant must fail exactly the rule it names. AC-006 is this atom applied to a fourth suite. | Before deciding whether the mutant is optional. It is not. | AC-006 |
| `.kb/decisions/0009-error-send-sync.md` | Errors keep exactly `core::error::Error + 'static` on every port and flavour. Two edges driven concurrently is the arrangement that tempts `+ Send + Sync` on the way past; NF-003 forbids it. | If the two-edge drive loop asks for a bound the port does not have. | AC-001 |
| `standards/rust/60-what-a-test-must-prove.md` | What separates a test that could fail from one that could not — the atom behind CLAUDE.md's *a rule that no adapter can fail is decorative*. The sequential two-adapter version of AC-001 is precisely the failure this atom describes. | Before writing the rule, and again when reviewing it. | AC-001, AC-006 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-5's out-of-package harness — the only way a `publish = false` crate's doctests run at all — and RS-62-3 on what a green async test does and does not prove. | Before putting any example in `happenstance-sync-testkit`. | AC-009 |
| `standards/rust/70-rustdoc-obligations.md` | The full rustdoc bar this project owes **instead of** a rendered surface: which items need docs, the `# Errors` form (conditions, not error types), and the compiled-example requirement. AC-009's composition invariant is this atom. | Before the first `pub` item is written, not after. | AC-009 |
| `standards/rust/41-declarative-macros.md` | The `tt`-callback and hoisted-fixture discipline `for_each_sync_peer_rule!` follows; getting the callback form wrong is how a rule is enumerated and still does not run. | When adding the rule name to the enumeration. | AC-002 |
| `xtask/src/main.rs` | The `wasm32` steps at `:203-283` with their stated reasoning, extended to six by HS-S0104. This file decides whether "runs in all three harnesses" is true or merely written down. | When verifying AC-002's `wasm32` leg, and if the wasm check passes suspiciously fast. | AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | The architecture brief's non-prescriptive notes and the module-doc warning (`:499-506`), the findings-before-deletion rule (`:590-599`), AC-A04's single-flavour GAT-free fixture (`:542-544`); the testing brief's AC-007 paragraph (`:745-751`), *Fixtures and seams* (`:802-811`), *Merge-gate commands* (`:775-800`) and *Notes* on what it deliberately does not test (`:848-853`). | Before fixing the fixture's shape, before the test plan, and before touching `lib.rs`. | AC-003, AC-004, AC-009 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/hub-and-spoke-and-peer-to-peer-topologies/discover.md` | The four named wrong implementations in full — `HubFlaggedPeer` (`:113-124`), `SymmetricWithDeclinedPush` (`:126-139`), the two-adapters-two-scenarios mutant (`:141-152`) and `ScalarWatermark` (`:154-159`) — each with the reason its gate stays green. | Before writing each test, to check it would actually fail the mutant it claims to reject. | AC-001, AC-004, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/sync-testkit-crate-and-rule-registry/spec.md` | The fixture contract this story consumes: the trait's associated types and capability constants (`:281-306`), the MUST-versus-trade policy (`:319`), `no_orphan_sync_rules` (`:323`), and — critically — AC-006 (`:368`), whose evidence is the declined `BOTH_ROLES` this story flips. EC-005 exists because of that line. | Before flipping `BOTH_ROLES`, and before assuming any fixture ingredient exists. | AC-003 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/headline-rules-and-mutant-registry/spec.md` | How the three earlier sync rules were shaped, registered and provenanced, and the five meta-tests AC-006's entry now has to satisfy. Copy the established form rather than inventing a fourth. | Before writing the rule body and the `Declared` entry. | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/send-free-sync-runner/spec.md` | The slice-mate's contract: per-peer resume tokens (its AC-006/AC-007), the no-rejection-above-the-port rule (its AC-008), and its AC-009 — one type on two edges, no role parameter — which this story's AC-001 proves at the port. Its EC-001 is the error behaviour EC-006 inherits unchanged. | Before writing any scenario, and whenever a scenario seems to need runner behaviour this story would otherwise invent. | AC-004, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The personas the acceptance criteria are written from, and the qualification that all rest on secondary evidence. Reading a criterion as a capability instead of an intent is the failure this prevents. | Before re-wording any acceptance criterion. | AC-001, AC-004, AC-009 |
| `references/evaluation/PRESSURE-TEST.md` | `:685-693` is the deferral evidence behind SY-27/SY-28 — scoped replication, the mechanism SY-10's confidentiality rationale actually rests on. Knowing it is deferred is what stops AC-008's finding from over-claiming. | When writing AC-008's finding, before asserting anything about confidentiality. | AC-008 |
| `RUNBOOK.md` | Phase 13 in full (`:4516-4620`), ADR-0027's place in the queue (`:306`), and the proof artefact this story feeds (`:4597-4602`). | If the story's place in the phase, or what it is owed by and owes to, becomes unclear. | AC-007, AC-008 |
| `.redkiln/config.yaml` | The `verify:` block wiring `cargo xtask affected --base main` to the story grain and `cargo xtask ci --fast` to `integration_scoped`, plus `require_ledger: true` — why each ledger row must name **which** command produced its evidence. | When filling `_ledger.md`. | AC-002, AC-009 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated** — AC-001 … AC-009, none added and
   none dropped. The mapping: AC-001 one adapter in both roles simultaneously, AC-002 the rule is
   mounted and reaches all three harnesses, AC-003 the oracle stops declining `BOTH_ROLES` while the
   declined path keeps an instrument, AC-004 hub with two divergent spokes, AC-005 transitive
   symmetric convergence, AC-006 the mandatory mutant, AC-007 SY-10's directional merge at runner
   grain with the tier finding, AC-008 SY-10's falsification test with its outcome recorded either
   way, AC-009 the documentation corrected by reading first.
2. **Project AC-007 is split across four rows rather than one.** Its sentence has three clauses and
   the middle one is worthless without a mounting story: *both topologies exercised* is AC-004 and
   AC-005, *one adapter type in both roles simultaneously* is AC-001 (with AC-002, AC-003 and AC-006
   making it non-decorative), and *the module documentation* is AC-009. Collapsing them would let two
   green scenarios that construct separate adapters satisfy the clause, which is the story's own
   named mutant.
3. **`directional_merge_rules_compose` is not landed as a suite rule, and this is decided rather than
   deferred.** SY-8 is `[FROZEN]` — a rule needing two peer handles in two directions with two
   policies is testing the runner — and SY-10 is `[PROVISIONAL]`. The frozen clause wins; the
   exercise lands at runner grain (AC-007), SY-10's `Rule:` line keeps its `(new)` marker, and the
   observation that the field points at the wrong tier is a recorded finding for HS-S0112/HS-S0113.
   `discover.md`'s open question *"Is `directional_merge_rules_compose` in scope here?"* is answered
   here: not as a suite rule, yes as a scenario.
4. **`one_adapter_serves_both_roles` holds two peer handles onto one fixture instance and this does
   not bend SY-8.** One fixture instance is one isolated backing store; two handles onto it is the
   sense `SECOND_HANDLE` already carries in the event-store suite. SY-9's `Rule:` field names a
   testkit rule explicitly, and demoting it to an integration test would mean the two live peers at
   HS-S0111 are never asked the question at all.
5. **The oracle's `BOTH_ROLES` flips, and the declined-capability instrument moves rather than
   disappearing.** HS-S0103 deliberately shipped the reference fixture declining it so the skip
   machinery ran on every build; left alone, SY-9's rule would be permanently `Skipped` against the
   only always-on peer in the tree and AC-007 would report green with its headline claim never run.
   Both halves are AC-003, and EC-005 requires them in one commit.
6. **SY-31's per-peer confirmation watermark is not landed here, and the reason is written down.**
   Its rule needs a `ProjectionStore` handle `SyncPeerFixture` does not have, and PS-9 — quoted
   inside SY-31's own text — says a generic runner cannot write a read model beside the watermark
   anyway. Adding the ingredient would be a fixture-contract change three stories after sign-off
   (NF-004). Per-edge confirmation state in these scenarios is the runner's per-peer resume token
   plus `Watermark` per origin; the renewal is HS-S0113's.
7. **AC-009 is a read-first obligation, not a rewrite.** AC-007's third clause asks that the module
   documentation "no longer describes only one" topology; `crates/happenstance-sync/src/lib.rs:52-66`
   already describes both, with the version-vector reason attached. The criterion therefore requires
   the verification *and* the corrections that are genuinely owed — the phase-2-sketch header and the
   *hard part* list — with every deleted paragraph's finding relocated into the consuming ADR first.
   A diff that rewrites `:52-66` because an AC said "documentation" would destroy the best statement
   of the constraint in the crate.
8. **No `[FROZEN]` clause is edited and no marker is moved in this diff.** Not SY-8, not SY-9, not
   SY-19; SY-10's and SY-31's provisional markers are left exactly as found even when this story's
   own evidence bears on them. Marker changes need a decision atom and route to HS-S0113 (EC-003).
9. **This story adjudicates no design.** Whether the two topologies are one abstraction or two is
   ADR-0027's, landed in slice 1 and carried on this story's `depends_on` edge. What is settled here
   is only how the landed shape is *tested*, and the implementation notes deliberately leave the
   spelling of a directional merge policy to the atom rather than restating it.
