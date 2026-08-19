---
item: HS-S0103
stage: discover
created: 2026-08-12T13:03:23.952Z
updated: 2026-08-12T13:03:23.952Z
template_sig: 86ce4036
rendered_sig: ca1688fd
---

# Discover — happenstance-sync-testkit and the sync rule registry

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: create `crates/happenstance-sync-testkit/` (`publish = false`, its own `version` key), depending on `happenstance-testkit` for `Capability`, `RuleOutcome` and the three emitters, with `for_each_sync_peer_rule!` + `sync_peer_conformance!` in the registry's exact shape (callback as `tt`, fixture hoisted behind `__conformance_fixture`), a single-flavour GAT-free peer fixture carrying its own round-trip counter, and a `no_orphan_sync_rules` meta-test | `_storymap.md`, *Slices* table, `sync-conformance-suite` row 1 | Substrate, not rules. The crate and the registry shape; the rules land in HS-S0105 |
| **Depends on `ingest-store-and-memory-peer-round-trip` (HS-S0102)**, which supplies a `MemorySyncPeer` and an `IngestStore` with real bodies — the thing the fixture hands a rule | `_storymap.md`, *Slices* `depends_on` | *"A fixture whose methods are `todo!()` cannot be the thing a conformance suite is pointed at"* (`crates/happenstance-sync/src/memory.rs:8-10`) |
| **AC-003** — `sync_peer_conformance!` ships in `happenstance-sync-testkit`, is emitted through the existing rule registry so it inherits the tokio, blocking and `wasm32` harnesses, and a declined capability appears in the output with the fixture's stated reason rather than vanishing | `project.md`, *Acceptance criteria*, AC-003 | Three obligations: the crate, the registry emission, the declension reporting |
| **AC-015** — nothing the charter excluded is published; the new crate is born `publish = false` and the name-claim disposition holds | `project.md`, *Acceptance criteria*, AC-015; `_storymap.md`, *Coverage* AC-015 row | The single line `publish = false` in the new manifest is this AC's whole mechanism |
| The recorded disposition: **this project does not claim either crate name**, against `RUNBOOK.md:4552-4555`'s work item, because a name reservation *is* a placeholder publish and the exclusion is stated in three artifacts against the claim's one | `_decomposition.md`, *AC-015 — the disposition on claiming the two crate names* | A recommendation with a named owner (`publication-and-positioning`, HS-P0016), not a settlement. Do not resolve it in a commit message |
| `crates/*` is a workspace glob, so the crate is a member the moment the directory exists | `Cargo.toml:3`; `_decomposition.md`, *The seam, in package terms* | Creating the directory changes what `--workspace` means. Nothing else has to be edited to include it — which is also how it escapes the checks that name crates explicitly |
| The registry shape to copy exactly: one macro holding the entire rule enumeration and handing it to a **caller-supplied emitter**, so tokio, blocking and `wasm32` harnesses share one definition | `crates/happenstance-testkit/src/registry.rs:93-110` | `for_each_sync_peer_rule!` is the same shape with a different list |
| The callback is captured as `$($callback:tt)+` and **not** `$cb:path`, because a parsed `path` fragment cannot sit in callee position — *"which would quietly forbid `let names = for_each_event_store_rule!(...)` — exactly what the meta-test below needs"* | `crates/happenstance-testkit/src/registry.rs:95-102` | The `tt` capture is what makes `no_orphan_sync_rules` writable at all |
| The entry macro hoists the fixture expression behind `async fn __conformance_fixture() -> impl Fixture` so the emitter never learns the fixture's type, and `$emit` is `$crate::`-qualified by the caller because a bare name would resolve in the *adapter's* crate | `crates/happenstance-testkit/src/lib.rs:315-342` | Two mechanical details that are invisible until they fail at a crate boundary |
| `no_orphan_rules` asserts **both directions**: every declared rule is registered, and every registered rule was found in the source scan | `crates/happenstance-testkit/src/registry.rs:411-435` | `no_orphan_sync_rules` is this, retargeted. One direction alone lets a rule exist and never run |
| A rule whose capability is unmet is **still emitted as a test**, returns `RuleOutcome::Skipped`, and the harness reports it — because `#[cfg]`-ing it out produces a binary where *"an adapter author who declines a capability to turn a red build green gets a green build and no record of the trade"* | `crates/happenstance-testkit/src/contract.rs:31-42` | The argument is already written. Reuse it rather than re-deriving it |
| `RuleOutcome` is `#[must_use]` with the message *"a rule's outcome must be reported, or a skipped rule is …"*, which is what turns "an emitter must report the skip" from prose into a build failure | `crates/happenstance-testkit/src/contract.rs:465-473` | Depend on `happenstance-testkit` for this type; redeclaring it re-opens the hole |
| **No `Send` flavour and no `trait_variant` on the fixture trait** — nothing ever spawns a fixture, and a second flavour *"would exclude precisely the adapters ADR-0001 exists for"* | `crates/happenstance-testkit/src/contract.rs:76-84` | The peer fixture inherits this verbatim. It is the same argument on a different port |
| **Owned associated types, never a GAT** — `where Self: 'a` on a GAT implemented for a foreign trait is one of five independently necessary ingredients of the rustc ICE this repository minimised, still reproducing on 1.97.1 | `crates/happenstance-testkit/src/contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/` | *"The handle owning a refcount instead of borrowing a lifetime is the difference between a compiling contract and an ICE"* |
| `happenstance_testkit::Fixture` cannot be reused directly: its `type Store: EventStore` is event-store-shaped and says nothing about a peer | `crates/happenstance-testkit/src/contract.rs:122-126`; `_decomposition.md`, *Composition root* §2 | Depend on the crate for `Capability`/`RuleOutcome`/emitters; declare only the genuinely different fixture trait |
| Methods are spelled `-> impl Future` rather than `async fn`, because `async fn` in a **public** trait fires `async_fn_in_trait` and the gate runs `-D warnings` — and the desugared form makes the *absence* of `+ Send` visible at the declaration | `crates/happenstance-testkit/src/contract.rs:86-96` | A style decision with a mechanical reason. The sync fixture inherits it |
| **SY-15 `[FROZEN]`** — every port method completable in one round trip; the rule is *"`peer_conformance!` invoked against a fixture peer whose transport asserts on its own round-trip count and panics on a second call within one operation"* | `spec/SPECIFICATION.md:6292-6300` | The round-trip counter is not a nicety: a `[FROZEN]` clause names it as the enforcement mechanism |
| The port *cannot express* "one round trip with no held state"; it must be checked by a fixture peer that counts its own round trips | `crates/happenstance-sync/src/peer.rs:43-50` | The counter lives on the fixture. This story is where it gets a home |
| **SY-17 `[FROZEN]`** — the rule is *"`happenstance-sync-testkit` compiling its own suite against a `!Send` fixture peer holding a `!Send` store behind an `Rc`"* | `spec/SPECIFICATION.md:6341-6360` | The fixture contract must admit a `!Send` peer. A `Send` bound anywhere on it fails a frozen clause by construction |
| `happenstance-sync-testkit` must carry its own `version` key, for the same reason `happenstance-testkit` does: adding a rule is semver-MINOR for the bar and nothing for the contract | `xtask/src/main.rs:420-436`; `xtask/src/lints.rs:45` (`TESTKIT_MANIFEST`, CF-32) | The key must exist here; teaching the lint to read it is HS-S0104's |
| A third crate cannot write `impl IngestStore for SomeoneElsesStore` — coherence bars it, and there is no blanket `impl<S: EventStore> IngestStore for S` | `crates/happenstance-sync/src/ingest.rs:31-37` | *"not a problem for a conformance suite, which takes an implementation rather than supplying one"* — but it does shape what the fixture may ask for |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered
elsewhere and inherited here; what this story owes is that the fixture cannot
hide the answer.** SY-1 and SY-6 are `[FROZEN]`, ADR-0026 reconciles with them,
and the design consequence for the fixture contract is that it must be able to
hand a rule an `EventGroup` carrying a **populated** `guard`, including one whose
`after` is `Some(_)` — otherwise `wire_condition_with_after_is_refused` has
nothing to refuse and SY-6 becomes untestable
(`_decomposition.md`, testing brief *Fixtures and seams to mock*).

**Hub-and-spoke versus peer-to-peer — not settled here, and the fixture must not
settle it by accident.** SY-9 is `[FROZEN]`: hub-ness is an edge property, so the
fixture trait must not have a role parameter, a `is_hub` constant, or two fixture
traits. `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) needs to
instantiate **one** fixture type twice in different roles
(`spec/SPECIFICATION.md:6090-6106`), which is a constraint on this story's trait
even though the topology question is ADR-0027's.

**What are the fixture's associated types and capabilities? — Deferred to spec,
under four fixed constraints.** No `trait_variant` and no `Send` flavour; owned
associated types and no GAT; `-> impl Future` rather than `async fn`; and it must
admit a `!Send` peer holding a `!Send` store behind an `Rc`, because SY-17's
`Rule:` field says so. What the associated types actually are — a `Peer:
SyncPeer` plus a `Store: IngestStore`, or one type satisfying both — is real
design and belongs to spec.

**Which capabilities does a peer fixture declare? — Deferred to spec, with the
policy already fixed.** The event-store fixture declares `SECOND_HANDLE` and
`REOPEN` (`crates/happenstance-testkit/src/contract.rs:161`, `:173`), and one of
them is exempt from declension because it is a MUST rather than a trade
(`:137-146`). The peer axis has different honest answers — a live Neon
environment, a peer that can be dropped and reconstructed, a peer that can be
driven in both roles — and each needs the same MUST-versus-trade judgement made
explicitly rather than by copying the list.

**Where does the round-trip counter live? — Answered: on the fixture, and
nowhere else.** The port cannot express the property
(`crates/happenstance-sync/src/peer.rs:43-50`) and SY-15's `Rule:` field already
prescribes a fixture peer that counts its own round trips and panics on a second
call within one operation (`spec/SPECIFICATION.md:6292-6300`). This story gives
the counter a home; HS-S0105 and HS-S0111 use it.

**Do the two crate names get claimed on crates.io? — Answered: no, and the
disposition is recorded rather than taken silently.** `RUNBOOK.md:4552-4555`
carries the claim as phase 13's first work item; the charter, the initiative
decomposition and this project's own *Out of scope* all forbid publishing either
sync crate in this release train. A name reservation is a placeholder publish. The
new manifest is born `publish = false` and the decision belongs to
`publication-and-positioning` (HS-P0016), which owns what ships and when
(`_decomposition.md`, *AC-015*).

**Does this story add any conformance rule? — No, deliberately.** It lands the
registry, the fixture contract and the meta-test. The first rules are HS-S0105's,
and the ordering exists so the registry is not shaped around whichever three rules
happened to be written first.

## Decision

The workspace has one conformance suite and the machinery under it was built to
be reused — a rule enumeration held in one macro and handed to a caller-supplied
emitter so three harnesses share one definition, a `Capability`/`RuleOutcome`
pair that makes a declined capability report itself instead of vanishing, and a
fixture contract carrying two hard-won negative results (no second flavour, no
GAT). `happenstance-sync` has no suite at all, which is the reason
`crates/happenstance-sync/src/lib.rs:113-115` can say idempotent ingest is *"Not
yet checked by anything"*. This story creates `happenstance-sync-testkit` on top
of the existing machinery rather than beside it: a new crate that **depends on**
`happenstance-testkit` for the two contract types and the three emitters and
declares only what is genuinely different — a peer fixture whose associated types
are a `SyncPeer` and an `IngestStore`-capable store, single-flavour, GAT-free, and
carrying the round-trip counter a `[FROZEN]` clause names as its own enforcement
mechanism. The spec stage will cover: the crate manifest, `publish = false` and
its own `version` key; the peer fixture trait, its associated types and its
capability constants with each MUST-versus-trade call made explicitly;
`for_each_sync_peer_rule!` with the `tt` capture and `sync_peer_conformance!`
with the `__conformance_fixture` hoist and `$crate::`-qualified emitter; the
re-export path for `Capability` and `RuleOutcome`; the three harnesses; and
`no_orphan_sync_rules` asserting both directions.

## The wrong implementation

**The mutant that this story is uniquely able to build and no later story can
undo: a fixture contract shaped like `MemorySyncPeer`.** Give the fixture a
`connect()` that hands back a peer *handle* the rules then drive across several
calls, and everything works. It is the obvious shape, it mirrors the event-store
fixture almost exactly, and `MemorySyncPeer` satisfies it perfectly because it is
an in-process object that stays alive. It quietly encodes two assumptions that
are false for two of the three peers this project must certify: that a peer can
hold state between calls (SY-15 forbids requiring it —
`spec/SPECIFICATION.md:6292-6300`), and that a peer handle outlives the exchange
(SY-16's `Rejects:` is *"an in-memory cursor, which passes every test written
against a process that stays alive and fails the deployment the crate exists
for"*, `:6323-6340`). The suite would then be green against three peers that are
all the same shape — an in-process oracle plus two adapters bent to fit a fixture
contract written from the oracle — which is the workspace's own standing rule
turned inside out: *"a port with one implementation is shaped like that
implementation"* (`crates/happenstance-sync/src/lib.rs:30-35`). The refutation
lives in `crates/happenstance-sync-testkit/tests/` as **`OneShotHttpFixture`**, a
fixture whose peer panics on a second transport call within one operation and
reconstructs its peer object between every exchange; it must be written **in this
story**, alongside the trait it constrains, because a fixture contract validated
only by the oracle is validated by the thing it is supposed to be independent of.
It is a *conformant variant*, not a mutant — it must pass everything, in the shape
CF-5 already uses for `GappedPositionStore` and `PagedStreamStore`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:326-349`).

**The second mutant is a suite that compiles and runs nothing.**
`for_each_sync_peer_rule!` enumerating eleven rules while
`happenstance-sync-testkit`'s `rules` module defines fourteen: the three
unlisted rules are dead code, `cargo test` is green, and `cargo xtask spec-trace`
is green too until `RULE_FILES` learns about the crate (HS-S0104). Nothing in the
tree today would notice, because the existing `no_orphan_rules` scans
`happenstance-testkit`'s own files (`crates/happenstance-testkit/src/registry.rs:411-435`).
`no_orphan_sync_rules` must assert **both** directions for the same reason the
original does — one direction lets a rule exist and never run; the other lets the
macro name a rule that has quietly been renamed.

**And the mutant that is a single fragment specifier.** Writing
`macro_rules! for_each_sync_peer_rule { ($cb:path) => { … } }` instead of
`$($callback:tt)+`. It compiles, the harnesses work, and every conformance run is
correct — and the meta-test above becomes unwritable, because a parsed `path`
fragment cannot sit in callee position and `let names = for_each_sync_peer_rule!(…)`
is rejected with *"macro expansion ignores `!` and any tokens following"*
(`crates/happenstance-testkit/src/registry.rs:95-102`). The failure surfaces as
"we could not write the orphan check", which reads like a limitation rather than
a defect, and the usual response is to drop the check.

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

**Box 6.** This story adds no conformance rule — it adds the registry the rules
plug into. It does make one commitment that CF-6's lint cannot: the peer fixture
trait exposes **no** way for a rule to obtain a bare position value that crossed a
boundary. A position reaches a rule only inside an `EventId` or as a value the
receiving store reported, so a rule that wanted to assert on a foreign position
would have to reach past the fixture to do it. `lint-position-literals` is taught
about this crate in HS-S0104; the trait shape is the part that has to be right
first.

**Box 7.** This story edits no `[FROZEN]` clause. Three constrain it — SY-15's
round-trip-counting fixture peer, SY-16's owned resume token, SY-17's `!Send`
fixture — and the trait conforms to all three by construction. Where the fixture
contract has freedom, ADR-0026 is the authorising decision and it is written first
(slice 1, `_storymap.md` *Merge order*). If drafting finds the SY-15 `Rule:`
field's prescribed mechanism unbuildable, that is a recorded finding and a
`Rejects:`-grade repair under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, not a quieter
fixture.
