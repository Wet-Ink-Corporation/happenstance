---
item: HS-S0110
stage: discover
created: 2026-08-12T13:03:32.047Z
updated: 2026-08-12T13:03:32.047Z
template_sig: 86ce4036
rendered_sig: 1f4d5ad7
---

# Discover — Both topologies are first class, and hub-ness is an edge

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: exercise both topologies over the same suite and the same runner, with one adapter type instantiated in both roles on different edges so SY-9's "hub-ness is an edge property" is tested rather than asserted, and bring `crates/happenstance-sync/src/lib.rs`'s module documentation into line with whichever shape landed | `_storymap.md`, *Slices* table, `runner-and-topologies` row 2 | Two scenarios, one adapter type in two roles, and a documentation correction |
| **Depends on `send-free-sync-runner` (HS-S0109)**, which supplies the runner both topologies are wirings of | `_storymap.md`, *Slices* `depends_on` | Topology is a runner configuration, so the runner must exist first |
| **Depends on `adr-0027-merge-compensation-and-message-set` (HS-S0099)**, which answers whether hub-and-spoke and peer-to-peer are one abstraction or two | `_storymap.md`, *Slices* `depends_on`; `RUNBOOK.md:306` | This story builds and tests whichever shape the atom landed. It does not adjudicate the shape |
| **AC-007** — both topologies are exercised, one adapter type serves both roles simultaneously on different edges, and `crates/happenstance-sync/src/lib.rs`'s module documentation no longer describes only one | `project.md`, *Acceptance criteria*, AC-007 | Three parts, and the third needs checking against the file before it is acted on |
| **SY-9 `[FROZEN]`** — hub-ness MUST NOT be a property of the port's type or constructor; one adapter type MUST be usable simultaneously as a hub to one set of peers and as a symmetric peer to another. Rule: `one_adapter_serves_both_roles` — *"a compile-and-run test that constructs one peer adapter twice, drives one edge in each role, and asserts both exchanges complete"* | `spec/SPECIFICATION.md:6090-6106` | The rule's mechanism is fully specified. "Simultaneously" and "one adapter type" are both load-bearing words |
| SY-9's `Rejects:` — `Peer::new(is_hub: bool)`, or separate `HubPeer`/`SpokePeer` traits. Kestrel Rotor is the case: a vessel that is a hub to nine technician tablets over ship's wifi and a symmetric peer to a shore depot over satellite, *at the same time*; *"A boolean on the constructor forces the vessel to hold two incompatible adapter instances over one store and gives neither of them a name for what the other is doing"* | `spec/SPECIFICATION.md:6101-6106` | The mutant is written for us, with its scenario |
| **SY-10 `[PROVISIONAL]`** — both topologies are first-class and *"the runner MUST permit a different merge rule in each direction of one edge"*. Rule: `directional_merge_rules_compose` | `spec/SPECIFICATION.md:6110-6128` | This is where the one-abstraction-or-two question lives, and it is a claim about the **runner**, not the port |
| SY-10's `Rejects:` — *"treating hub-and-spoke as peer-to-peer with one side declining to push"*; the hub sees every log and can adjudicate, a spoke sees one and cannot; Kestrel Cold Chain's edge tier is strictly hub-and-spoke for a **commercial** reason — a spoke's slice is a confidentiality boundary — so *"a design that models the asymmetric case as a degenerate symmetric one has no place to put that"* | `spec/SPECIFICATION.md:6120-6128` | The second mutant, also written for us, and its cost is confidentiality rather than correctness |
| SY-10's falsifier: *"falsified if a hub and a spoke can be shown to need the same merge rule in a deployment where the hub holds every log and the spoke holds one. Falsification test: implement the hub's rule on a spoke and show the spoke's projections stay correct."* | `spec/SPECIFICATION.md:6112-6117` | A `[PROVISIONAL]` marker with a real experiment. This story is where the experiment becomes runnable |
| The crate's module documentation as it stands describes **both** topologies — peer-to-peer at `:53-58`, hub and spoke at `:60-66` — and gives the reason `Watermark` is a version vector rather than a scalar: *"the hub sees every log, the spokes see one each, and the merge rule a spoke needs is not the merge rule the hub needs"* | `crates/happenstance-sync/src/lib.rs:52-66` | **Verify before rewriting.** AC-007's third part is a correction, and the file may already satisfy it; the architecture brief says so explicitly |
| `Watermark` is a version vector *because* of hub-and-spoke, and that is *"a constraint on the port's shape and not merely on its documentation"* | `crates/happenstance-sync/src/lib.rs:62-66` | A scalar watermark is a topology decision disguised as a type simplification |
| **SY-31 `[PROVISIONAL]`** — a per-peer confirmation watermark MUST NOT be an event and MUST be written through `ProjectionStore` under a reserved `ProjectionId`; it rejects the watermark-as-event (*"a fact about a link, not about the world"*) and the watermark in adapter-private storage | `spec/SPECIFICATION.md:6731-6760` | Per-peer confirmation state is what makes a fleet observable, and it is where a hub with many spokes differs from one symmetric pair |
| **SY-8 `[FROZEN]`** — the port describes exactly one peer relationship; *"a rule that needed two would be testing the runner"* | `spec/SPECIFICATION.md:6070-6086` | So the topology scenarios are runner-grain integration tests, and only `one_adapter_serves_both_roles` sits in the suite |
| The port's own statement: *"Nothing here says whether the far side is a hub or a spoke. A store in the middle of a chain — a Durable Object that is a spoke to a cloud estate store and a hub to 138 tablets — is one peer wearing each hat, and a `is_hub()` on the port would force it to answer a question that has two answers."* | `crates/happenstance-sync/src/peer.rs:75-80` | The port is already correct. This story is what makes it *tested* |
| Testing brief: two integration scenarios over the same suite — a hub-and-spoke wiring (one hub, at least two spokes) and a peer-to-peer wiring, *"with one adapter type instantiated in both roles on different edges in at least one scenario"*, because SY-9's claim *"is untested until some test literally does this"* | `_decomposition.md`, testing brief *Integration* | Two spokes minimum, not one — a one-spoke hub is indistinguishable from a peer pair |
| The peer fixture must carry no role parameter, because HS-S0110 needs to instantiate one fixture type twice in different roles | `sync-testkit-crate-and-rule-registry`'s discover, *Questions* | A constraint this story imposed on HS-S0103 and now consumes |

## Questions

**Are hub-and-spoke and peer-to-peer one abstraction or two? — Answered by
ADR-0027 and tested here, and the specification already fixes both ends of the
answer.** SY-9 is `[FROZEN]`: not two *types*, because hub-ness is an edge
property and one adapter serves both roles at once. SY-10 is `[PROVISIONAL]` and
says they are not one *behaviour* either: *"the runner MUST permit a different
merge rule in each direction of one edge"*, because the hub sees every log and
can adjudicate while a spoke sees one and cannot. So the shape the answer must
have is: one port, one adapter type, one runner — with **directional** policy on
each edge. What ADR-0027 settles is how that policy is expressed; what this story
proves is that both wirings work with one adapter type instantiated twice.

**Does ingest re-check the writer's asserted append conditions? — Not this
story's, and the asymmetry does not reopen it.** A hub can adjudicate *because*
it holds every log, and adjudication is a domain decision producing a
compensation (SY-2), not a precondition on the append. SY-1 applies identically
on both sides of every edge.

**Does the module documentation actually need rewriting? — Deferred to spec, and
it must start with a read rather than an edit.** AC-007 asks that
`crates/happenstance-sync/src/lib.rs` *"no longer describes only one"* topology;
the file describes both today, at `:52-66`, and states hub-and-spoke is *"at
least as common as the symmetric case and it is not a special case of it"*. The
architecture brief flags this directly: *"verify what AC-007 is actually asking
for against the file before rewriting it"*. The genuine documentation work is
elsewhere in the same file — the *"phase-2 sketch, not the protocol"* header and
the *"hard part, stated honestly"* list, which stop being true when this project
lands — and each of those paragraphs is a finding that must be moved into the ADR
that consumed it before it is deleted (`_decomposition.md`, *Notes*).

**How many spokes? — Answered: at least two.** A hub with one spoke is a
peer-to-peer pair with a label on it, and it exercises none of the properties
that distinguish the topologies: the version-vector watermark, per-peer
confirmation state, and the hub's ability to adjudicate across logs a spoke
cannot see.

**Is `directional_merge_rules_compose` in scope here? — Deferred to spec.** It is
SY-10's rule and SY-10 is `[PROVISIONAL]`; whether it lands as a suite rule (it
would need two peer handles, which SY-8 says makes it a runner test) or as an
integration scenario is a real question. `one_adapter_serves_both_roles` is
unambiguously a suite rule — SY-9's `Rule:` field describes it as a
compile-and-run test constructing one adapter twice.

**Does the confidentiality property get tested? — Explicitly deferred, and named
so it is not lost.** SY-10's rationale rests on a spoke's slice being a
confidentiality boundary, and the mechanism for that is scoped replication —
SY-27 and SY-28, both `[DEFERRED]` against
`references/evaluation/PRESSURE-TEST.md:685-693`. This story exercises the
*topology*; the scoping that makes the confidentiality claim real is renewed, not
built (`clause-arithmetic-and-deferral-renewals`, HS-S0113).

## Decision

The crate has documented two target topologies since phase 2 and has never run
either: `Watermark` is a version vector rather than a scalar specifically because
a hub sees every log while a spoke sees one, and that is called out as *"a
constraint on the port's shape and not merely on its documentation"* — but with
no runner and no suite, nothing has ever wired a hub to two spokes or driven one
adapter type in both roles at once. SY-9 is frozen on the strongest form of the
claim, that hub-ness is a property of an **edge** rather than of a node, with
Kestrel Rotor as the case: a vessel that is a hub to nine tablets over ship's wifi
and a symmetric peer to a shore depot over satellite, simultaneously. That claim
is untested until a test literally constructs one adapter twice and drives one
edge in each role. This story does that, wires a hub with at least two spokes and
a symmetric pair over the same runner and the same suite, and corrects whatever in
`crates/happenstance-sync/src/lib.rs`'s module documentation the landed shape has
made untrue — after reading it, because the topology paragraphs may already be
right and the paragraphs that are actually stale are the ones about the crate
being a sketch. The spec stage will cover: the two wirings and their minimum
sizes; which adapter type is instantiated in both roles and on which edges;
`one_adapter_serves_both_roles` as a suite rule with its mutant; whether
`directional_merge_rules_compose` is a suite rule or an integration scenario; the
per-peer confirmation watermark's placement under SY-31; and a documentation diff
justified line by line against what the landed design made false.

## The wrong implementation

**`HubFlaggedPeer` — SY-9's own named target.** `Peer::new(is_hub: bool)`, or a
`HubPeer` and a `SpokePeer` trait. It passes every conformance rule in the suite,
because every rule takes one peer handle and each flavour behaves correctly on
its own edge. It passes both topology scenarios, if each scenario constructs the
adapter it needs. What it cannot do is Kestrel Rotor: a vessel that is a hub to
nine tablets and a symmetric peer to a shore depot *at the same time* must hold
two incompatible adapter instances over one store, *"and gives neither of them a
name for what the other is doing"* (`spec/SPECIFICATION.md:6101-6106`). The rule
that fails it is `one_adapter_serves_both_roles`, and the mutant belongs in
`crates/happenstance-sync-testkit/tests/` with a provenance naming the vessel: a
boolean on the constructor is what everyone writes first, because on any single
edge it is correct.

**`SymmetricWithDeclinedPush` — SY-10's named target, and the subtler one.**
Model hub-and-spoke as peer-to-peer where the spoke declines to push. Both
scenarios pass. Every event reaches every node. The merge rule is uniform, which
reads as elegance. Two things are lost and neither is a correctness failure a test
of *convergence* can see: the hub's ability to adjudicate — it holds every log and
the spoke does not, so the same merge rule cannot be right on both sides — and the
confidentiality boundary, which in Kestrel Cold Chain is a **commercial**
constraint, Scottish subcontractors not holding Yorkshire's parts pricing. *"A
design that models the asymmetric case as a degenerate symmetric one has no place
to put that"* (`spec/SPECIFICATION.md:6120-6128`). The instrument is SY-10's own
falsification test run in the honest direction: implement the hub's rule on a
spoke and show the spoke's projections stay correct — if they do, SY-10 is
falsified and the marker moves by decision record; if they do not, this mutant is
refuted by name.

**The suite-shaped mutant: two topology scenarios that use two different adapter
types.** A `MemorySyncPeer` hub and a `MemorySyncPeer` spoke, constructed
separately, in two separate tests, each green. Every word of AC-007 reads as
satisfied — both topologies exercised, both green — and the actual claim, *one
adapter type in both roles **on different edges simultaneously***, is never
exercised. It is the topology-grain instance of this project's standing hazard: a
suite green against peers that are all the same shape, so the port is frozen
against one arrangement wearing two names. The distinguishing test is
structural rather than behavioural — one `let peer = Adapter::new(...)`, two
edges, both driven before either completes — and it must be written that way
deliberately, because the version that constructs twice is easier to read and
proves less.

**And the type-simplification mutant: `ScalarWatermark`.** Collapse `Watermark`
from a version vector to a single position, because in a two-node test that is
all it ever holds. Every peer-to-peer test passes. The hub-and-spoke scenario
fails only once there are **two** spokes with divergent progress — which is
exactly why the minimum wiring in this story is one hub and two spokes, and why a
one-spoke hub proves nothing (`crates/happenstance-sync/src/lib.rs:62-66`).

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

**Box 6, and multi-node topologies are where a literal is most tempting and most
wrong.** With a hub and two spokes there are three independent position
sequences, and the natural way to write "everything arrived" is to compare
numbers. SY-19 is explicit that no expression relates two peers' position numbers
and that none can be constructed, because `SequencePosition` carries no origin —
Kestrel Rotor's fourteen events sit at vessel position 38,102 and depot position
3,918,442 (`spec/SPECIFICATION.md:6421-6428`). Every assertion here is over the
**set of `EventId`s** each node holds and over watermark advancement per origin
store; no rule or scenario added here asserts a literal position, and none
compares a position across a node boundary.

**Box 7.** This story edits no `[FROZEN]` clause. It implements SY-9 — the first
test that makes its claim falsifiable — and it exercises `[PROVISIONAL]` SY-10
and SY-31. The design it builds is authorised by ADR-0027, written first in slice
1 and this story's direct `depends_on` edge. If SY-10's falsification test comes
out positive, that is a marker change requiring a new decision atom and it routes
to `clause-arithmetic-and-deferral-renewals` (HS-S0113) with a recorded finding —
never a clause softened in this diff. The module-documentation changes are prose
in a `.rs` file, not clause text, and every paragraph deleted must first have its
finding moved into the ADR that consumed it.
