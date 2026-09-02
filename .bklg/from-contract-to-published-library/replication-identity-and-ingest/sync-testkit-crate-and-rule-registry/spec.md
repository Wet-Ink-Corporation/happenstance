---
item: HS-S0103
stage: spec
created: 2026-08-12T13:47:43.019Z
updated: 2026-08-12T13:47:43.019Z
template_sig: 87bbf1d0
rendered_sig: ed527670
---

# Spec — happenstance-sync-testkit and the sync rule registry

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project (charter) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/sync-testkit-crate-and-rule-registry/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — *architecture: Composition root* §1–§3, §6; *testing: The test mix, tier by tier* (Unit), *Fixtures and seams to mock* |
| Story map row | `…/replication-identity-and-ingest/_storymap.md`, *Slices*, `sync-conformance-suite` row 1 |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12. There is no surface id to render; the API-surface obligation is discharged in the architecture brief and here |
| Discover stage (this story) | `…/sync-testkit-crate-and-rule-registry/discover.md` — signal ledger, the three named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13); the work item is `RUNBOOK.md:4576-4580`, the proof artefact `RUNBOOK.md:4597-4602` |

## One-line PR slice

Create `crates/happenstance-sync-testkit/` (`publish = false`, its own `version` key), depending on
`happenstance-testkit` for `Capability`, `RuleOutcome` and the three emitters, with
`for_each_sync_peer_rule!` + `sync_peer_conformance!` in the registry's exact shape (callback as `tt`,
fixture hoisted behind `__conformance_fixture`), a single-flavour GAT-free peer fixture carrying its own
round-trip counter, and a `no_orphan_sync_rules` meta-test.

## Executive summary

**This PR lands the empty instrument, correctly shaped, and nothing that measures with it yet.**

The workspace already has one conformance suite and the machinery under it was built to be reused: a rule
enumeration held in one macro and handed to a caller-supplied emitter so three harnesses share one
definition (`crates/happenstance-testkit/src/registry.rs:93-110`), a `Capability`/`RuleOutcome` pair that
makes a declined capability report itself rather than vanish
(`crates/happenstance-testkit/src/contract.rs:31-42`), and a fixture contract carrying two hard-won
negative results — no second flavour, no GAT (`:76-84`, `:97-111`). `happenstance-sync` has no suite at
all, which is why `crates/happenstance-sync/src/lib.rs:113-115` can still say idempotent ingest is *"Not
yet checked by anything"*.

The delta over the parent story (`ingest-store-and-memory-peer-round-trip`, HS-S0102, which gives
`IngestStore` and `MemorySyncPeer` real bodies) is a second crate that **depends on**
`happenstance-testkit` rather than forking it, and declares exactly one genuinely new thing: a peer
fixture whose associated types are a `SyncPeer` and an `IngestStore`-capable store, single-flavour,
GAT-free, and carrying the round-trip counter that `[FROZEN]` SY-15 names as its own enforcement
mechanism (`spec/SPECIFICATION.md:6292-6300`).

**It adds no conformance rule, deliberately.** The rules are `headline-rules-and-mutant-registry`'s
(HS-S0105) and the ordering exists so the registry is not shaped around whichever three rules happened to
be written first. What stops that from making this story unfalsifiable is the pair of *conformant-variant*
fixtures it must write beside the trait: a fixture contract validated only by the in-process oracle is
validated by the one thing it is supposed to be independent of.

## Context pack

The decisions this story must honour. Each is stated as a decision, not as a pointer; the deeper
artefacts are behind the anchors.

**1. The fixture contract is the deliverable, and it is the thing most likely to be got wrong.** The
obvious shape — `connect()` hands back a peer *handle* that rules then drive across several calls —
mirrors the event-store fixture almost exactly and `MemorySyncPeer` satisfies it perfectly, because it is
an in-process object that stays alive. It encodes two assumptions that are false for two of the three
peers this project must certify: that a peer may hold state between calls (SY-15 `[FROZEN]` forbids
*requiring* it) and that a peer handle outlives the exchange (SY-16 `[FROZEN]`, whose `Rejects:` is *"an
in-memory cursor, which passes every test written against a process that stays alive and fails the
deployment the crate exists for"*, `spec/SPECIFICATION.md:6323-6340`). A suite green against three peers
that are all the same shape is the workspace's own standing rule turned inside out — *"a port with one
implementation is shaped like that implementation"* (`crates/happenstance-sync/src/lib.rs:30-35`).

**2. Three `[FROZEN]` clauses bind the trait's shape, and this story edits none of them.**

- **SY-15** — every port method completable in one round trip, no held state. Its `Rule:` field prescribes
  the enforcement: *a fixture peer whose transport asserts on its own round-trip count and panics on a
  second call within one operation*. The counter is therefore not a nicety; it is the mechanism a frozen
  clause names. It lives on the fixture because the port **cannot express** the property
  (`crates/happenstance-sync/src/peer.rs:43-50`).
- **SY-16** — resume state is an owned, transferable value the caller supplies per call. The fixture must
  be able to *drop the peer object and rebuild it* so a rule can prove the token survives.
- **SY-17** — the port and its runner are written against `EventStore`, not `SendEventStore`, and the
  clause's `Rule:` field is *"`happenstance-sync-testkit` compiling its own suite against a `!Send`
  fixture peer holding a `!Send` store behind an `Rc`"* (`spec/SPECIFICATION.md:6341-6360`). That
  compiled artefact is owed **by this story**, since this story is where the trait that would forbid it
  is written. A `Send` bound anywhere on the fixture fails a frozen clause by construction.

**3. Reuse `Capability` and `RuleOutcome`; do not redeclare them.** A rule whose capability is unmet is
still emitted as a test, returns `RuleOutcome::Skipped`, and the harness reports it — because `#[cfg]`-ing
it out yields a binary in which *"an adapter author who declines a capability to turn a red build green
gets a green build and no record of the trade"* (`crates/happenstance-testkit/src/contract.rs:31-42`).
`RuleOutcome` is `#[must_use]` with that reasoning in the message (`:465-473`), which is what turns "an
emitter must report the skip" into a build failure. Redeclaring either type in the new crate re-opens the
hole silently. `happenstance_testkit::Fixture` itself is **not** reusable: its `type Store: EventStore`
(`:122-126`) is event-store-shaped and says nothing about a peer.

**4. Two mechanical details of the registry that are invisible until they fail at a crate boundary.**
The callback is captured as `$($callback:tt)+`, **not** `$cb:path` — a parsed `path` fragment cannot sit
in callee position, which *"would quietly forbid `let names = for_each_event_store_rule!(...)` — exactly
what the meta-test below needs"* (`crates/happenstance-testkit/src/registry.rs:95-102`). And the entry
macro hoists the fixture expression behind `async fn __conformance_fixture() -> impl Fixture` so the
emitter never learns the fixture's type, with `$emit` `$crate::`-qualified by the caller because a bare
name is substituted verbatim and would resolve in the *adapter's* crate
(`crates/happenstance-testkit/src/lib.rs:315-342`). Get the first wrong and `no_orphan_sync_rules` becomes
unwritable — and the usual response to "we could not write the orphan check" is to drop the check.

**5. Capabilities on a peer fixture are not the event-store list retyped, and each MUST-versus-trade call
is made here.** The event-store fixture declares `SECOND_HANDLE` (a **MUST** — the rule fails rather than
skips, `crates/happenstance-testkit/src/contract.rs:137-146`) and `REOPEN` (a trade), plus three
`Option<usize>` **limits**, which are facts rather than trades. On the peer axis:

- *Peer reconstruction* is the **MUST**, and it is SY-16's analogue of `SECOND_HANDLE`: a fixture that
  cannot drop and rebuild its peer makes an in-memory cursor indistinguishable from an owned token, which
  is precisely the defect the clause exists to catch. Declining it must fail, not skip.
- *Both roles* (one fixture instance driven as hub and as spoke on different edges, SY-9) is a genuine
  **trade** — a peer whose far side is a fixed endpoint honestly cannot serve as someone's hub — and is
  what `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) consumes.
- **No limit constants.** `SyncPeer` already carries `fn limits(&self) -> PeerLimits`
  (`crates/happenstance-sync/src/peer.rs:150-158`), which `EventStore` does not; SY-18's rule is
  `peer_declares_its_own_limits` (`spec/SPECIFICATION.md:6370`). Copying `MAX_EVENT_DATA_LEN` and
  friends onto the fixture would state a limit in two places, and two places can disagree.
- **A `Capability` is an associated `const`, so it cannot mean "the network was down today."** A fixture
  variant compiled without a live transport may decline; a transiently unreachable environment is a test
  **failure**. Collapsing the two would make `declined` the sanctioned way to turn a red build green —
  the exact move `contract.rs:31-42` was written against. `durable-object-and-neon-peers` (HS-S0112)
  inherits this distinction; it is settled here rather than there.

**6. `publish = false` is written explicitly and is not inherited from the sibling.**
`crates/happenstance-testkit/Cargo.toml` carries **no** `publish` key — it is one of the three
publishable crates the gate's `cargo package --list` assertion covers (`CLAUDE.md`, *Commands*). Copying
that manifest as a template produces a publishable crate and breaks project AC-015 on the first line of
the diff. The recorded disposition is that this project claims **neither** sync crate name, against
`RUNBOOK.md:4552-4555`'s work item, because a name reservation *is* a placeholder publish and the
exclusion is stated in three artifacts against the claim's one; the decision belongs to
`publication-and-positioning` (HS-P0016), which owns what ships and when
(`…/replication-identity-and-ingest/_decomposition.md`, *AC-015 — the disposition on claiming the two
crate names*). The crate carries its own `version` key for the reason `happenstance-testkit` does —
adding a rule is semver-MINOR for the bar and nothing for the contract (`xtask/src/main.rs:420-436`).

**7. Creating the directory is the whole of "adding a member", which is also how the crate escapes every
check that names a crate explicitly.** `members = ["crates/*", "examples/*", "xtask"]` (`Cargo.toml:3`).
`RULE_FILES` (`xtask/src/spec_trace.rs:85-89`), `TESTKIT_SRC` (`xtask/src/lints.rs:42`) and
`TESTKIT_MANIFEST` (`:45`) each name `happenstance-testkit` alone. Teaching them is
`gate-mounts-for-the-sync-suite`'s (HS-S0104) — the very next story in this slice, landed in the same
context. **Do not lean on those lints to catch anything in this PR**; until HS-S0104 merges, this crate's
`src/` is outside CF-33's no-clock scope and outside CF-6's position-literal scope, and the obligations
have to be met by construction. The story map says so on purpose: *"a suite that never reaches
`spec-trace`, `lint-position-literals` or the `wasm32` steps is exactly the failure the architecture brief
calls 'silently escapes three checks'"* (`…/_storymap.md`, *Coverage* notes).

**8. The suite never decodes a payload, and the fixture is where that becomes structural.** SY-35
`[FROZEN]` (`spec/SPECIFICATION.md:6867-6880`): anything replication reasons about is in the `EventType`
or `Tags`, and the rule `the_sync_suite_never_decodes` works by the fixture domain supplying payloads
*not decodable in any codec*. That rejects statically — *"the suite will not compile a fixture that needs
to decode"* — which is a property of the **fixture contract**, i.e. of this story. Nothing this PR adds
may hand a rule a decoded `Event::data` or `Event::metadata`, and no fixture method may return one
parsed.

**9. The persona-journey slice.** This project's reader is the adapter author who wants to know whether
their peer is conformant, and today the honest answer is that nothing can tell them. The slice this story
realises is the first half of *A3 — Prove a peer conforms* (`…/_storymap.md`, *Backbone*): after this PR,
`sync_peer_conformance!(MyFixture::new())` compiles, expands into three harnesses and runs — reporting
zero rules. That is a deliberately unimpressive milestone and the right one: the shape is what later
stories cannot change cheaply, and the rules are what they can.

**10. One naming divergence, recorded and not fixed here.** `RUNBOOK.md:4576`, `project.md` AC-003, the
architecture brief's AC-A02 and the story map all say `sync_peer_conformance!`. SY-15's `Rule:` field says
`peer_conformance!` (`spec/SPECIFICATION.md:6298`) and SY-15 is `[FROZEN]`. This story ships
`sync_peer_conformance!` — three artifacts to one, and the `sync_` prefix is what keeps it unambiguous
beside `event_store_conformance!` in a workspace that has both. The clause is **not edited**: a `Rule:`
field naming a macro that ships under another name is a `Rejects:`-grade repair under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, and it is `frozen-clause-repairs`'
(HS-S0114, project AC-012). This story's job is to write the divergence down where that story will find
it.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate consumed by capability slices in this same
  project. Not a double, not a flag, not a `todo!()`.
- **Slice / milestone**: `sync-conformance-suite`. Slice-mates, implemented in one context and mounted as
  one surface: `gate-mounts-for-the-sync-suite` (HS-S0104) and `headline-rules-and-mutant-registry`
  (HS-S0105). Merge order within the slice is this story → HS-S0104 → HS-S0105
  (`…/_storymap.md`, *Merge order* 3).
- **Mount point**: `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs` — the real harness
  that invokes `sync_peer_conformance!` against a fixture wrapping the `MemorySyncPeer` HS-S0102 made
  real. This is the analogue of `crates/happenstance-testkit/tests/memory_conformance.rs`, and it is what
  makes the macro reachable rather than constructed-but-unmounted: a suite no harness invokes is the
  library form of the component nobody imported. Two sibling harnesses mount the other two flavours —
  `memory_peer_conformance_blocking.rs` and `memory_peer_conformance_wasm.rs`, matching
  `crates/happenstance-testkit/tests/memory_conformance_blocking.rs` and `…_wasm.rs`.
- **Wires into**:
  - `crates/happenstance-testkit/src/contract.rs` — `Capability`, `RuleOutcome`, and the declined-capability
    argument, consumed as a dependency;
  - `crates/happenstance-testkit/src/registry.rs:228-300` — `__emit_tokio`, `__emit_blocking`, `__emit_wasm`
    and `__emit_rule_names`, the three harness emitters plus the meta-test emitter;
  - `crates/happenstance-sync/src/peer.rs` — `SyncPeer`, `Pulled`, `PushBatch`, `EventGroup`, `PeerLimits`;
  - `crates/happenstance-sync/src/ingest.rs` — `IngestStore`;
  - `crates/happenstance-sync/src/memory.rs` — `MemorySyncPeer`, the oracle the reference fixture wraps;
  - `Cargo.toml:3` — the `crates/*` member glob, which admits the new crate the moment the directory exists.
- **Renders surfaces**: **none.** `…/replication-identity-and-ingest/_design.md` records a no-surface
  determination, approved by the repository owner on 2026-08-12, with `design.capture` a declared skip.
  There is no surface id for this story to claim. The public API surface it *does* add carries the
  repository-wide rustdoc obligation (`standards/rust/70-rustdoc-obligations.md`).
- **Public items** (in place of `_design.md`'s `## Items`, which this project records as N/A):
  `happenstance_sync_testkit::{SyncPeerFixture, sync_peer_conformance, for_each_sync_peer_rule}`, the
  re-exports of `Capability` and `RuleOutcome`, and `fixtures::MemorySyncPeerFixture`. The `rules` module
  exists and is empty of rules.
- **Conformance rule(s)**: **none added, and that is the point.** The rules are HS-S0105's. What this
  story adds instead is one meta-test, `no_orphan_sync_rules`, asserting both directions between the
  `rules` module and `for_each_sync_peer_rule!`'s enumeration — the shape of
  `crates/happenstance-testkit/src/registry.rs:411-435`.
- **Clause(s)**: discharges nothing on its own; **conforms by construction** to SY-8
  (`spec/SPECIFICATION.md:6070-6086`, one peer handle per rule — enforced here by the fixture handing out
  exactly one), SY-15, SY-16, SY-17 and SY-35. Amends none. The SY-15 naming divergence (Context pack
  §10) is recorded for HS-S0114, not repaired here.
- **Advances DoD scenario**: project **DoD 4** — *"the proof artefact exists, and it would not exist if
  the design were wrong: one suite green against three peers"* — by building the instrument the artefact
  is made of. Through it, initiative **DoD 14** (*"Replication has an answer on disk"*,
  `.bklg/from-contract-to-published-library/initiative.md:398-401`), which this project is the only one
  that can close: the instrument is what turns ADR-0026's written answer into something mechanically
  observable rather than asserted. Initiative **DoD 13** (*"the gate is green on the assembled whole"*) is
  a not-regressed obligation: the new member must not turn `cargo xtask ci --fast` red.

## PR boundary

```
crates/happenstance-sync-testkit/**
.bklg/from-contract-to-published-library/replication-identity-and-ingest/sync-testkit-crate-and-rule-registry/**
CHANGELOG.md
```

**In this PR**

- The new crate: manifest (`publish = false`, own `version`, workspace-inherited edition/MSRV/licence/lints),
  `README.md`, `src/lib.rs`, `src/contract.rs`, `src/registry.rs`, `src/fixtures.rs`, an empty-of-rules
  `src/rules.rs`.
- `SyncPeerFixture`, its capability constants and its round-trip counter.
- `for_each_sync_peer_rule!` and `sync_peer_conformance!`.
- `fixtures::MemorySyncPeerFixture`, the reference implementation, wrapping `MemorySyncPeer`.
- Three mounted harnesses under `tests/`, plus the two conformant-variant fixtures (round-trip discipline;
  `!Send` peer behind an `Rc`) and the `no_orphan_sync_rules` meta-test.
- One `CHANGELOG.md` entry for the new crate. Not a per-rule entry — there are no rules — and CF-29's
  per-rule obligation begins at HS-S0105.

**Explicitly not in this PR**

- **Any conformance rule.** HS-S0105.
- **The mutant registry** (CF-1 – CF-4). HS-S0105; the two fixtures written here are *conformant
  variants* under CF-5, which must pass everything, not mutants.
- **`RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, the position-literal and changelog lint scopes, and
  the two new `wasm32` gate steps.** HS-S0104 — hence no `xtask/**` in the boundary above.
- **Any edit to `spec/SPECIFICATION.md`**, including the SY-15 naming repair. HS-S0114.
- **Any change to `crates/happenstance-sync/`**, including the `memory`-feature discrepancy at
  `…/_decomposition.md` *Composition root* §4. If the fixture cannot be written without one, that is a
  finding to raise, not a widened boundary.
- **Anything under `.kb/`.** Atoms arrive only through `/redkiln:kb-ingest`.

The implementer **may** additionally touch the composition-root files named in the Integration contract
where mounting requires it; nothing in that list is expected to need editing, because `Cargo.toml:3`'s
glob is the whole of the wiring. If the root `Cargo.toml` does need a line (a workspace dependency entry
for the new crate, say), that is mounting and not scope drift — add it to the boundary block deliberately
rather than widening the block to make a red gate green.

**Merge DoD**: `cargo xtask affected --base main` and `cargo xtask lints && cargo xtask spec-trace` green;
`cargo test -p happenstance-sync-testkit --all-features` green with the three harnesses present in the
output; `cargo xtask ci --fast` green on the tree; `_ledger.md` cites evidence per AC-###.

## Behavior and interfaces

The **binding** parts of the shape below are the ones carrying a citation: single flavour, no GAT, owned
associated types, `-> impl Future`, the counter on the fixture, no duplicated limits. Names and the exact
decomposition of the associated types are the implementer's, and a deviation is legitimate if it is
recorded with the reason.

```rust
/// One replication edge: a local store, the far side a rule may seed, and the
/// single peer handle this side holds onto it.
pub trait SyncPeerFixture {
    /// The receiving side. `IngestStore` to accept a foreign identity,
    /// `EventStore` to read the result back — both the weaker, `Send`-free
    /// flavours, which is what admits the Durable Object peer.
    type Local: IngestStore + EventStore;
    /// The far side, reachable as a store so a rule can put facts on it
    /// without going through the port under test.
    type Remote: IngestStore + EventStore;
    /// Exactly one peer relationship (SY-8). A rule that needed two would be
    /// testing the runner.
    type Peer: SyncPeer;

    /// SY-16's analogue of `SECOND_HANDLE`: a MUST, enforced by rules that
    /// fail rather than skip.
    const PEER_RECONSTRUCTION: Capability;
    /// SY-9: one fixture instance driven as hub and as spoke on different
    /// edges. A genuine trade; HS-S0110 consumes it.
    const BOTH_ROLES: Capability;

    fn connect(&self) -> impl Future<Output = (Self::Local, Self::Remote, Self::Peer)>;
    /// Drops the peer object and rebuilds it against the same far side.
    fn reconnect(&self) -> impl Future<Output = Self::Peer>;
    /// Transport calls this fixture has made. Incremented by the fixture's own
    /// peer wrapper; never inferred from a return type.
    fn round_trips(&self) -> u64;
}
```

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The crate exists as a workspace member, born unpublishable** | `crates/happenstance-sync-testkit/Cargo.toml` carries `publish = false` **written explicitly** and its own `version` key (not `version.workspace = true`); `edition`, `rust-version`, `license`, `repository`, `lints` inherited. The sibling is *not* a template for the `publish` line: `happenstance-testkit` has no `publish` key and is publishable | `crates/happenstance-sync/Cargo.toml:12`; `crates/happenstance-testkit/Cargo.toml:1-22`; `Cargo.toml:3`; `xtask/src/main.rs:420-436`; `…/_decomposition.md`, *AC-015* |
| **`Capability`, `RuleOutcome` and the three emitters are consumed, not re-declared** | `happenstance-testkit` is a `[dependencies]` entry; the two types are re-exported from the new crate's root so an adapter author names one crate; `__emit_tokio` / `__emit_blocking` / `__emit_wasm` are reached through it. Nothing in the new crate defines a type named `Capability` or `RuleOutcome` | `crates/happenstance-testkit/src/contract.rs:31-42`, `:465-473`; `crates/happenstance-testkit/src/registry.rs:228-292`; `…/_decomposition.md`, *Composition root* §2 |
| **`for_each_sync_peer_rule!` holds the whole enumeration and takes a `tt` callback** | `($($callback:tt)+) => { $($callback)+! { … } }`. Provably in expression position: `let names = for_each_sync_peer_rule!(rule_names);` compiles, as a doctest on the macro. `$cb:path` compiles too and silently forbids exactly this | `crates/happenstance-testkit/src/registry.rs:80-102` |
| **`sync_peer_conformance!` hoists the fixture and `$crate::`-qualifies the emitter** | Four arms mirroring the event-store macro: `(mod_name, emit, fixture)`, `(mod_name, fixture)`, `(fixture)`, defaulting to `mod_name = dcb_sync_conformance` and `emit = $crate::__emit_tokio`. The fixture expression sits behind `async fn __conformance_fixture() -> impl SyncPeerFixture`, re-evaluated per test so every rule gets its own fixture. `emit` must be `$crate::`-qualified by the caller or it resolves in the adapter's crate | `crates/happenstance-testkit/src/lib.rs:311-358` |
| **Three harnesses share one enumeration** | tokio, blocking and `wasm32` harnesses each expand the same `for_each_sync_peer_rule!`; a rule added once appears in all three without touching a harness | `crates/happenstance-testkit/src/registry.rs:93-110`; `…/_decomposition.md`, *Composition root* §1 |
| **The fixture trait is single-flavour and GAT-free** | No `trait_variant`, no `Send` bound anywhere, no `type X<'a> where Self: 'a`. Nothing ever spawns a fixture; a second flavour would exclude precisely the adapters ADR-0001 exists for, and a `where Self: 'a` GAT on a foreign-trait impl is one of five ingredients of the rustc ICE this repository minimised and that still reproduces on 1.97.1 | `crates/happenstance-testkit/src/contract.rs:76-84`, `:97-111`; `experiments/rustc-ice-gat-foreign-trait/`; `.kb/decisions/0001-async-port-flavours.md` |
| **Methods are `-> impl Future`, not `async fn`** | `async fn` in a *public* trait fires `async_fn_in_trait` and the gate runs `-D warnings`; the desugared form also makes the **absence** of `+ Send` visible at the declaration, which is the ADR-0001 point. An impl may still write `async fn` | `crates/happenstance-testkit/src/contract.rs:86-96` |
| **The round-trip counter lives on the fixture** | `fn round_trips(&self) -> u64`, incremented by the fixture's own peer wrapper. The port cannot express "one round trip with no held state"; SY-15's `Rule:` field prescribes a fixture peer that counts its own and panics on a second call within one operation | `crates/happenstance-sync/src/peer.rs:43-50`; `spec/SPECIFICATION.md:6292-6300`; `…/_decomposition.md`, *Fixtures and seams to mock*, last-but-one row |
| **Capabilities: one MUST, one trade, no limits** | `PEER_RECONSTRUCTION` is a MUST (rules fail rather than skip, as `SECOND_HANDLE` does); `BOTH_ROLES` is a trade; no `Option<usize>` limit constants, because `SyncPeer::limits()` already carries them and SY-18 tests them there. A declined capability still emits a test and reports the fixture's stated reason | `crates/happenstance-testkit/src/contract.rs:31-42`, `:137-146`; `crates/happenstance-sync/src/peer.rs:150-158`; `spec/SPECIFICATION.md:6363-6372` |
| **A declined capability is a compile-time property, never a flaky environment** | `Capability` is an associated `const`; a fixture variant compiled without a live transport may decline, an unreachable environment at run time **fails**. Documented on the trait, because HS-S0112 inherits it | `crates/happenstance-testkit/src/contract.rs:359-420`; `project.md`, DR-8 |
| **`MemorySyncPeerFixture` is a published item, not a test helper** | It lives in `fixtures`, the same call `happenstance-testkit` made for `MemoryFixture`: it is the reference implementation an adapter author reads before writing their own | `crates/happenstance-testkit/Cargo.toml:24-27`; `crates/happenstance-testkit/src/fixtures.rs` |
| **Two conformant variants, written here, in the crate's own `tests/`** | (a) a one-shot-HTTP-shaped fixture whose peer panics on a second transport call within one operation and reconstructs its peer object between exchanges (SY-15/SY-16); (b) a `!Send` fixture peer holding a `!Send` store behind an `Rc`, which SY-17's `Rule:` field names as the compiled artefact. Both **must pass everything** — CF-5's shape, as `GappedPositionStore` and `PagedStreamStore` already are — and neither is a mutant | `spec/SPECIFICATION.md:6292-6300`, `:6341-6360`, `:7221-7232`; `crates/happenstance-testkit/tests/mutation_coverage.rs:326-349`; `discover.md`, *The wrong implementation* |
| **`no_orphan_sync_rules` asserts both directions** | Every rule in the `rules` module is enumerated by the macro, and every enumerated name was found by the source scan. One direction alone lets a rule exist and never run; the other lets the macro name a rule that has quietly been renamed. Green and vacuous at this story, load-bearing from HS-S0105 | `crates/happenstance-testkit/src/registry.rs:411-435` |
| **Nothing decodes a payload, and no bare foreign position is reachable** | No fixture method returns a parsed `Event::data` or `Event::metadata`; a position reaches a rule only inside an `EventId` or as a value the receiving store reported. CF-6's lint does not cover this crate until HS-S0104, so the shape is the enforcement | `spec/SPECIFICATION.md:6867-6880`, `:7233-7249`; `project.md`, DR-5, DR-7; `discover.md`, *Gate: Discover* box 6 |
| **Rustdoc on every public item, with a compiled example on the entry macro** | Repository-wide obligation. The entry macro's doctest is the adapter author's first contact with the suite and is what makes "it compiles for a crate that has never heard of us" a checked claim rather than a hope. Doctests run for a `publish = false` member — `happenstance-sync` is one and its doctests run today — so no special harness is owed unless an `include_str!` leaves the package | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/62-doctests-and-harnesses.md` RS-62-5; `crates/happenstance-sync/Cargo.toml:12` |
| **No `[FROZEN]` clause is edited, and one divergence is recorded** | The macro ships as `sync_peer_conformance!`; SY-15's `Rule:` field says `peer_conformance!`. Written down for `frozen-clause-repairs` (HS-S0114) under the repair playbook, never edited here | `spec/SPECIFICATION.md:6298`; `RUNBOOK.md:4576`; `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `project.md`, AC-012 |

## Data and migrations

**N/A — no persistent data, no schema, no migration.** This story adds a `dev`-grade library crate whose
entire runtime footprint is in-process: the reference fixture wraps `MemorySyncPeer`, which holds its log
in a `Vec` behind a `Mutex` (`crates/happenstance-sync/src/memory.rs:1-10`), and the two conformant
variants in `tests/` are hand-written shapes with no backing store at all. Nothing here reads or writes a
file, a database or a network.

Two adjacent things that are *not* data migrations but would be mistaken for them:

- **The wire format.** `Envelope<T>` and `FORMAT_VERSION` are settled by ADR-0016 and the message set is
  `message-set-on-the-envelope`'s (HS-S0107). This story adds no message type and no derive; a
  `#[derive(Serialize, Deserialize)]` on a public message type *is* a wire-format decision
  (`crates/happenstance-sync/src/lib.rs:86-94`).
- **Crate-version arithmetic.** The new crate starts at its own `version` and that number is a *bar*
  version, not a contract version. Adding a rule later is semver-MINOR for it and nothing for
  `happenstance-core` — which is the whole reason the key is written by hand rather than inherited
  (`crates/happenstance-testkit/Cargo.toml:4-14`).

## Acceptance criteria

The two personas this story serves are the initiative's, carried from
`.bklg/from-contract-to-published-library/initiative.md:241-248`: the **adapter author** on *Learn when
you are finished* — *"from a signature that type-checks to a suite that says pass or fail and names
why"* — and the **constrained-runtime developer** on *Event-source at the edge without hand-rolling it*,
whose peer is `!Send` and whose fear is orphaned tooling. Each criterion below is one of their moments,
crossing the whole stack from the macro invocation in their crate to what `cargo test` prints.

Every criterion is verified by a test at a real path in this PR. Paths under
`crates/happenstance-sync-testkit/` do not exist yet — they are this story's deliverables, and the
ledger's `verifying_test` column names them so the implementer cannot quietly satisfy an AC somewhere
else.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The crate is born unpublishable, and the release train cannot pick it up by accident.** GIVEN the repository owner is holding `0.2.0` to the three crates the charter names, WHEN this PR adds a fourth workspace member through `Cargo.toml:3`'s `crates/*` glob, THEN `crates/happenstance-sync-testkit/Cargo.toml` carries `publish = false` **written explicitly** (not absent, not inherited from the sibling, which has no such key) and its own `version` key rather than `version.workspace = true`, and no publishable crate gains a dependency on it. | `crates/happenstance-sync-testkit/tests/manifest_shape.rs::crate_is_born_unpublishable` — parses the crate's own `Cargo.toml` from `CARGO_MANIFEST_DIR` and asserts `package.publish == false` and that `package.version` is a literal. Corroborated by `cargo xtask ci --fast`'s `cargo package --list` step still covering exactly three crates (`xtask/src/main.rs:420-436`). |
| AC-002 | **An adapter author points the suite at their peer and it runs.** GIVEN an adapter author who has just written a `SyncPeerFixture` in a crate that has never heard of this workspace's internals, WHEN they write `happenstance_sync_testkit::sync_peer_conformance!(MyFixture::new());` at the top level of a `tests/` file with no other setup, THEN it compiles and `cargo test` emits a `dcb_sync_conformance` module that runs to completion — reporting zero rules at this story, which is honest rather than empty, because the rules are HS-S0105's. | `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs` — the mount point, invoking the macro against `fixtures::MemorySyncPeerFixture`, in the shape of `crates/happenstance-testkit/tests/memory_conformance.rs`. |
| AC-003 | **The same peer is certified on all three runtimes without the author writing three suites.** GIVEN the constrained-runtime developer whose peer must work under tokio, under no runtime at all, and on `wasm32-unknown-unknown`, WHEN they mount the suite three times with the three emitters, THEN all three harnesses expand the **one** `for_each_sync_peer_rule!` enumeration, so a rule added once in HS-S0105 appears in all three without a harness being touched. | `crates/happenstance-sync-testkit/tests/memory_peer_conformance_blocking.rs` and `…/memory_peer_conformance_wasm.rs`, mirroring `crates/happenstance-testkit/tests/memory_conformance_blocking.rs` and `…_wasm.rs`; the emitters are `happenstance_testkit::{__emit_tokio, __emit_blocking, __emit_wasm}` (`crates/happenstance-testkit/src/registry.rs:228-292`). |
| AC-004 | **The rule list is a value the repository can inspect, not just a thing that expands.** GIVEN a maintainer who needs the enumeration in expression position to write any meta-check over it, WHEN they write `let names = for_each_sync_peer_rule!(happenstance_sync_testkit::__emit_rule_names);`, THEN it compiles — because the callback is captured as `$($callback:tt)+` and not as `$cb:path`, a parsed `path` fragment being unable to sit in callee position. | A compiled doctest on `for_each_sync_peer_rule!` in `crates/happenstance-sync-testkit/src/registry.rs`, run by `cargo test -p happenstance-sync-testkit --doc`; the shape and the reason are `crates/happenstance-testkit/src/registry.rs:80-102`. |
| AC-005 | **No rule can exist and never run, and no name can be enumerated that no longer exists.** GIVEN the adapter author trusting a green run to mean the whole bar was applied, WHEN a rule is defined in the `rules` module but omitted from the macro — or enumerated in the macro after being renamed in the module — THEN the meta-test fails and names the offending rule, in **both** directions. Green and vacuous at this story; load-bearing from HS-S0105. | `crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules`, the retargeted shape of `crates/happenstance-testkit/src/registry.rs:412-434`, scanning this crate's own `rules` source against the enumeration. |
| AC-006 | **Declining a capability costs the author a line in the output, never a vanished test.** GIVEN an adapter author whose peer genuinely cannot do something, WHEN their fixture declares that `Capability` as `declined("…")`, THEN the rule is still emitted as a test, returns `RuleOutcome::Skipped`, and the harness prints the fixture's own stated reason — and neither type is redeclared in this crate: both are consumed from `happenstance-testkit` and re-exported from the new crate's root so the author names one crate. | `crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs` — the reference fixture declines `BOTH_ROLES` with a stated reason, so the skip machinery is exercised end to end on every run; plus `crates/happenstance-sync-testkit/tests/manifest_shape.rs::contract_types_are_reexported_not_redeclared` asserting `happenstance_sync_testkit::Capability` and `::RuleOutcome` resolve to the `happenstance_testkit` items (`crates/happenstance-testkit/src/contract.rs:31-42`, `:359-433`, `:465-473`). |
| AC-007 | **A `!Send` peer holding a `!Send` store is a first-class citizen of the bar.** GIVEN the constrained-runtime developer on Cloudflare Workers, whose store is behind an `Rc` and can never be `Send`, WHEN they implement `SyncPeerFixture` for it, THEN the suite compiles and runs against it — because the fixture trait has **no** `Send` bound, no `trait_variant` second flavour, no GAT, and spells its methods `-> impl Future` rather than `async fn`. This is SY-17's own prescribed artefact (`spec/SPECIFICATION.md:6341-6360`) and a `Send` bound anywhere on the trait fails a `[FROZEN]` clause by construction. | `crates/happenstance-sync-testkit/tests/send_free_fixture.rs` — a conformant-variant fixture whose peer holds a `!Send` store behind an `Rc`, mounted through `sync_peer_conformance!` and asserted `!Send` by a negative static check. Reinforced by the `wasm32` harness of AC-003. |
| AC-008 | **The bar is not shaped like the in-process oracle it was first written against.** GIVEN a one-shot-HTTP peer that cannot hold a transport open and must be reconstructed between exchanges, WHEN it is driven through the same fixture contract as `MemorySyncPeer`, THEN it passes everything — because the fixture exposes `round_trips()` as a counter it increments itself (never inferred from a return type) and `reconnect()` that drops the peer object and rebuilds it against the same far side, which is the mechanism SY-15's `Rule:` field names and the shape SY-16's `Rejects:` demands. | `crates/happenstance-sync-testkit/tests/one_shot_http_fixture.rs::OneShotHttpFixture` — a **conformant variant** in the CF-5 shape of `GappedPositionStore`/`PagedStreamStore` (`crates/happenstance-testkit/tests/mutation_coverage.rs:326-349`), whose peer panics on a second transport call within one operation; it must pass every emitted rule, and its `round_trips()` is asserted directly. |
| AC-009 | **What a fixture may honestly decline is decided here, once, and each call is a MUST or a trade on the record.** GIVEN HS-S0110 needing one fixture in both roles and HS-S0112 needing an unavailable environment reported rather than hidden, WHEN a fixture author reads the trait, THEN they find exactly two capability constants — `PEER_RECONSTRUCTION`, a **MUST** whose decline makes rules fail rather than skip (as `SECOND_HANDLE` does), and `BOTH_ROLES`, a genuine trade — **zero** limit constants, because `SyncPeer::limits()` already carries them and SY-18 tests them there, and rustdoc stating that an associated `const` cannot mean "the network was down today": a transiently unreachable environment is a failure. | `crates/happenstance-sync-testkit/tests/one_shot_http_fixture.rs::declining_peer_reconstruction_fails_rather_than_skips` — a variant declining the MUST, asserted to produce a failure and not a `Skipped`; the trade half is AC-006's declined `BOTH_ROLES`. Policy source: `crates/happenstance-testkit/src/contract.rs:137-146`, `:216-232`; `crates/happenstance-sync/src/peer.rs:150-158`. |
| AC-010 | **The suite cannot certify a peer that reads payloads, because it cannot read one itself.** GIVEN SY-35 `[FROZEN]` — replication reasons only about `EventType` and `Tags` — WHEN a rule author tries to assert on a decoded `Event::data` or `Event::metadata`, THEN there is no fixture method that hands them one: nothing returns a parsed payload, and a position reaches a rule only inside an `EventId` or as a value the receiving store reported, never as a bare foreign integer. CF-6's lint does not cover this crate until HS-S0104, so the trait shape is the enforcement. | `crates/happenstance-sync-testkit/tests/one_shot_http_fixture.rs::payloads_are_undecodable` — the fixture domain supplies payload `Bytes` that are not valid in any codec, which is SY-35's own prescribed mechanism (`spec/SPECIFICATION.md:6867-6880`); plus a `rg`-shaped source assertion in `manifest_shape.rs` that no public signature in `src/` returns a decoded payload type. |
| AC-011 | **The adapter author's first contact with the suite is a worked example, not a signature.** GIVEN someone landing on `happenstance-sync-testkit`'s docs with no context, WHEN they read the entry macro's page, THEN they find a compiled example that invokes `sync_peer_conformance!` the way a foreign crate would (`$crate::`-qualified emitter and all), rustdoc on every public item, the CHANGELOG entry for the new crate, and — recorded rather than repaired — the note that SY-15's `Rule:` field says `peer_conformance!` while this crate ships `sync_peer_conformance!`, handed forward to HS-S0114. | `cargo test -p happenstance-sync-testkit --doc` (the entry-macro doctest in `crates/happenstance-sync-testkit/src/lib.rs`) plus the `docs` step of `cargo xtask ci --fast` with `-D warnings`; the divergence note is cited by `file:line` in the ledger. |

**Traceability.** Project AC-003 (*the suite exists and no rule is silently absent*) is carried by
AC-002, AC-003, AC-004, AC-005, AC-006, AC-007, AC-008, AC-009, AC-010 and AC-011 — the crate, the
registry emission and the declension reporting, which are the three obligations the discover ledger
splits it into. Project AC-015 (*nothing was published that the charter excluded*) is carried by
AC-001 alone, which is the whole of that AC's mechanism per `…/_storymap.md`, *Coverage* AC-015 row.

## Interaction quality

**This story renders no surface.** `…/replication-identity-and-ingest/_design.md` records a no-surface
determination approved by the repository owner on 2026-08-12, with every composition heading — *Items*,
*Placement and re-export*, *The states the API must express*, *Anti-patterns*, *The doctest* — marked
`N/A — no user-facing surface`, and `design.capture` a **declared** skip. What that file *does* record
(`_design.md:58-71`) is that the public API surface is the only surface here and that its obligations are
discharged in the architecture and testing briefs. So the two families below are read against the surface
that exists: the API an adapter author meets and the text `cargo test` prints.

Every invariant that applies is already an `AC-###` row in the table above. This section says only which
row carries it and how it is verified — nothing here is a free-standing obligation, because a bullet in
this section gets no ledger row and is never gated.

**State family**

- **In-place, not a context jump** — carried by AC-002 and AC-003. The suite runs inside the adapter
  author's own crate under their own `cargo test`, from one macro line; there is no separate runner
  binary, no out-of-tree step and no second toolchain invocation. Verified by the three mounted
  harnesses.
- **Non-occlusion** — carried by AC-006. A declined capability must not hide the rule: the test is still
  emitted, still named, and prints the fixture's reason. `#[cfg]`-ing it out is the occlusion this
  forbids, and the argument is already written at `crates/happenstance-testkit/src/contract.rs:31-42`.
- **Preserved state across the interaction** — carried by AC-002 and AC-008. Each rule gets its own
  fixture, because `sync_peer_conformance!` hoists the fixture expression behind
  `async fn __conformance_fixture()` and calls it per test; and `round_trips()` is per-fixture, so one
  rule's transport count is never another's. The analogue of preserved selection here is per-rule
  identity: a failure names the rule that failed, not the module.
- **Reversibility** — carried by AC-001. `publish = false`, written explicitly, is what keeps every shape
  decision in this PR revocable: nothing reaches a registry, so nothing here becomes a promise. It is
  also the reason the shape may be got wrong once and fixed, which the ordering (registry before rules)
  depends on.
- **Reachability without special tooling** — carried by AC-002, AC-004 and AC-011. Everything is reachable
  from `cargo test` and `cargo doc` alone: the macro is usable in expression position (AC-004), the
  reference fixture is a published item rather than a `#[cfg(test)]` helper (AC-009), and the entry point
  carries a compiled example (AC-011). The keyboard-reachability analogue for a library is that no step
  requires an editor, a plugin, or a hand-written harness.

**Composition family**

- **Presentation exists at all** — carried by AC-011. The API analogue of "not bare markup" is a public
  item with real rustdoc and a *compiled* example, per `standards/rust/70-rustdoc-obligations.md`. A
  crate that exports `SyncPeerFixture` with a one-line summary and no worked invocation is the unstyled
  render: it satisfies every type check and teaches nobody.
- **Composition and placement** — carried by AC-006 and AC-009. One crate name for the adapter author:
  `Capability` and `RuleOutcome` are re-exported from the new crate's root rather than made an import
  from a second crate, and `MemorySyncPeerFixture` sits in a public `fixtures` module — the same
  placement call `happenstance-testkit` made (`crates/happenstance-testkit/src/fixtures.rs`), for the
  same reason: it is the reference implementation an author reads before writing their own.
- **Transience** — carried by AC-006. Persistent chrome is the rule itself, emitted on every run whatever
  the fixture declares. What is *revealed* is the reason string, printed only when the capability is
  declined. Nothing is opened on demand: there is no verbosity flag that must be passed before a skip
  becomes visible, because a skip nobody sees is the failure mode the whole `Capability` shape exists to
  prevent.
- **Density budget, with the real numbers** — carried by AC-009 and AC-007. The trait ships **three**
  associated types (`Local`, `Remote`, `Peer`), **two** capability constants, **zero** limit constants,
  and **three** methods (`connect`, `reconnect`, `round_trips`). The crate ships **two** macros, **three**
  harnesses, **one** meta-test, **two** conformant-variant fixtures and **zero** rules. A fourth
  associated type, a third capability, or any `Option<usize>` limit is over budget and needs a stated
  reason in the implementation report — the limits especially, because `SyncPeer::limits()` already
  states them and two statements can disagree.
- **Hierarchy** — carried by AC-002 and AC-003. One entry point (`sync_peer_conformance!`) is what an
  adapter author uses; `for_each_sync_peer_rule!` and the emitters are the layer beneath, reached only by
  someone mounting a non-default flavour. The default arm exists precisely so the common case is one
  line.
- **Named anti-patterns** — `_design.md` records `N/A` under *Anti-patterns*, so the operative set is
  this story's own three named wrong implementations in `…/sync-testkit-crate-and-rule-registry/discover.md`,
  *The wrong implementation*: a fixture contract shaped like `MemorySyncPeer` (rejected by AC-007 and
  AC-008), a suite that compiles and runs nothing (rejected by AC-005), and `$cb:path` in place of
  `$($callback:tt)+` (rejected by AC-004). Each is refuted by a compiled artefact in this PR, not by a
  paragraph.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | A fixture declares `Capability::declined("")` — a decline with no reason. | Rejected at const evaluation by `happenstance-testkit`'s own assertion (`crates/happenstance-testkit/src/contract.rs:412-418`). This crate must not route around it by introducing any other constructor for a declined capability. |
| EC-002 | A fixture declines `PEER_RECONSTRUCTION`, the MUST. | Rules that need it **fail**, naming the fixture and the constant; they do not return `RuleOutcome::Skipped`. The distinction is `SECOND_HANDLE`'s (`crates/happenstance-testkit/src/contract.rs:137-146`) and is what stops SY-16's in-memory-cursor defect from being declined away. |
| EC-003 | A peer makes a second transport call within one port operation. | The conformant-variant fixture's transport panics with a message naming the operation and the round-trip count, which is SY-15's prescribed enforcement (`spec/SPECIFICATION.md:6292-6300`). A port that needs two round trips fails loudly rather than passing slowly. |
| EC-004 | The environment a fixture variant needs is transiently unreachable at run time. | **Test failure.** Not a declined capability. A `Capability` is an associated `const` and cannot express "today"; collapsing the two would make `declined` the sanctioned way to turn a red build green. Documented on the trait because HS-S0112 inherits it. |
| EC-005 | A rule exists in the `rules` module but not in `for_each_sync_peer_rule!`, or vice versa. | `no_orphan_sync_rules` fails and names the rule and the direction. Vacuously green at this story, and the failure text must already be written for a non-empty module — a meta-test whose message is only exercised at HS-S0105 is a message nobody has read. |
| EC-006 | A caller passes a bare emitter name — `sync_peer_conformance!(my_mod, __emit_blocking, F::new())`. | The name resolves in the *caller's* crate and fails to compile there. Not repairable from this side; the entry macro's rustdoc states the `$crate::`-qualified form and the doctest uses it (`crates/happenstance-testkit/src/lib.rs:315-342`). |
| EC-007 | The fixture expression is evaluated once and shared across rules. | Forbidden by construction: the expression sits inside `async fn __conformance_fixture()`, which every emitted test calls. A refactor that hoists it to a `static` or a `OnceCell` silently makes every rule share one backing store and must be rejected in review — one fixture instance is one isolated backing store (`CLAUDE.md`, *The rule that matters*). |
| EC-008 | A rule needs a decoded payload to make its assertion. | Unreachable: no fixture method returns a parsed `Event::data` or `Event::metadata`, and the variant fixtures' payloads are not decodable in any codec. A rule author who wants one must change the trait, which is a visible diff and a `[FROZEN]`-clause conversation (SY-35). |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| NF-001 | The publishable dependency graph is unchanged. No crate without `publish = false` gains a dependency on `happenstance-sync-testkit`, directly or transitively. | AC-001's manifest test plus `cargo xtask ci --fast`'s `cargo package --list` step, which still covers exactly the three publishable crates (`xtask/src/main.rs:420-436`). |
| NF-002 | No `Send` bound appears anywhere in this crate's public surface — no `trait_variant`, no `+ Send`, no `SendEventStore` import. | AC-007's `!Send` fixture is the compiled proof; a source-level assertion in `manifest_shape.rs` is the cheap tripwire. `CLAUDE.md` binding constraints 1 and 4. |
| NF-003 | The new member does not turn `cargo xtask ci --fast` red or materially slower. With zero rules the three harnesses are near-instant; the cost is compile time for one more member. | `cargo xtask ci --fast` green on the tree, and `cargo xtask affected --base main` green at the story grain. |
| NF-004 | The crate holds the workspace MSRV of 1.97.1 by inheriting `rust-version`, and uses no feature above that floor. | `rust-version.workspace = true` in the manifest (`Cargo.toml:8`); CI's `msrv` job. `.kb/decisions/0029-msrv-raised-to-1-97-1.md` is the authority for why the floor is a deliberate trade rather than a habit. |
| NF-005 | Doctests run for this member despite `publish = false` — the obligation is not quietly skipped. | `cargo test -p happenstance-sync-testkit --doc` present in the story's evidence, with `happenstance-sync` (also `publish = false`, `crates/happenstance-sync/Cargo.toml:12`) as the precedent that they do run; `standards/rust/62-doctests-and-harnesses.md` RS-62-5 if an `include_str!` ever leaves the package. |
| NF-006 | No `unsafe`, and no crate-level `#![allow]` beyond the workspace lints table. In particular no `#![allow(clippy::todo)]` — this crate ships no `todo!()`, which is what distinguishes it from the six skeletons. | `cargo clippy --workspace --all-targets -- -D warnings` inside `cargo xtask ci --fast`. |
| NF-007 | The crate is legible to a reader who has never seen `happenstance-testkit`: its module documentation states what a sync peer fixture is and why it is not `happenstance_testkit::Fixture`. | Part of AC-011's rustdoc obligation; `crates/happenstance-testkit/src/contract.rs:122-126` is the reason the two traits are separate. |

## Implementation notes (non-prescriptive)

Everything here is a suggestion with a reason. Deviate freely, but record the deviation and the reason in
the implementation report — the citations in *Behavior and interfaces* are the parts that are not
negotiable.

- **Copy the sibling's structure, not its manifest.** `src/lib.rs`, `src/contract.rs`, `src/registry.rs`,
  `src/fixtures.rs`, `src/rules.rs` mirrors `happenstance-testkit` file for file, which is worth more than
  it looks: the next person to read both is comparing them. The manifest is the one file to write from
  scratch, because copying it is exactly how `publish = false` goes missing (Context pack §6).
- **Write `src/rules.rs` empty but present, with module documentation explaining that it is empty on
  purpose and who fills it.** An absent module reads as an oversight; a present, documented, empty one
  reads as an ordering decision. `no_orphan_sync_rules` needs the file to scan either way.
- **Write the two conformant-variant fixtures before the trait is final.** They are the only instruments
  in this PR that can disagree with the trait — the reference fixture wraps `MemorySyncPeer` and will
  agree with almost any shape. If `OneShotHttpFixture` cannot be written against a draft trait without
  contortion, the trait is wrong and that is the finding.
- **Consider whether `Local` and `Remote` are genuinely two associated types.** The sketch has three;
  one store type used twice would be two. The reason to keep them apart is that a rule seeds the far side
  directly and reads the near side back, and the two are structurally unlike for the Neon peer. If they
  collapse cleanly, collapsing them is a density win — say so.
- **`reconnect()` returning only the peer (not the stores) is deliberate** in the sketch: the far side
  must survive the reconstruction or the test proves nothing. If the implementer finds a shape where the
  stores must come back too, that is a real finding about SY-16 and belongs in the report.
- **Do not add a `Runtime` or role parameter to the fixture trait.** SY-9 is `[FROZEN]` on hub-ness being
  an edge property, and HS-S0110 must instantiate *one* fixture type in both roles
  (`spec/SPECIFICATION.md:6090-6106`). A role parameter here would make that story impossible without
  reopening a frozen clause.
- **The `memory` feature discrepancy** noted at `…/_decomposition.md`, *Composition root* §4 is out of
  boundary. If the reference fixture cannot be built without touching `happenstance-sync`'s features,
  raise it rather than widening the PR.
- **Read `crates/happenstance-testkit/src/lib.rs:311-358` before writing the entry macro's arms.** The
  four-arm default cascade and the `$crate::` qualification are both easy to approximate and hard to
  debug once approximated at a crate boundary.

## Tests and CI (merge gate)

Tiers and commands are the project testing brief's (`…/_decomposition.md`, *The test mix, tier by tier*
and *Merge-gate commands*), narrowed to what this story can actually prove. This story's tiers are
**Static** and **Unit**; the Integration tier the brief describes (three peers, two of them live) is
HS-S0112's, and nothing here may claim it.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` (inside `cargo xtask ci --fast`) | The new member is subject to the workspace's lints from the moment the directory exists; NF-006, and the `-D warnings` run is what makes `async_fn_in_trait` a hard error rather than a note. |
| Static | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`, `reachability_static`) | No clause cites a rule that does not exist, and no existing rule is orphaned. **It does not yet see this crate** — `RULE_FILES`, `TESTKIT_SRC` and `TESTKIT_MANIFEST` all name `happenstance-testkit` alone until HS-S0104. Running it here proves this PR did not break the existing suite, and nothing more. |
| Static | `cargo xtask ci --fast`'s `cargo package --list` assertion (`xtask/src/main.rs:420-436`) | AC-001 / NF-001: the publishable set is still exactly three crates, so the new member is not being dragged into the licence-and-README check a publishable crate owes. |
| Static | `cargo xtask wasm` (the four existing steps, inside `cargo xtask ci --fast`) | The `wasm32` build of `happenstance-core` and the conformance-harness check still pass with a fourth member present. The two **new** `wasm32` steps that would run *this* crate's harness are HS-S0104's; AC-003's `wasm32` harness is compiled here and gated there. |
| Unit | `cargo test -p happenstance-sync-testkit --all-features` | AC-002, AC-003, AC-005 through AC-010: the three mounted harnesses run to completion, `no_orphan_sync_rules` passes in both directions, the declined `BOTH_ROLES` reports its reason in the output, and both conformant-variant fixtures pass every emitted rule. |
| Unit | `cargo test -p happenstance-sync-testkit --doc` | AC-004 (the expression-position doctest on `for_each_sync_peer_rule!`) and AC-011 (the entry-macro example an adapter author reads first). NF-005: these must actually run for a `publish = false` member. |
| Unit | `crates/happenstance-sync-testkit/tests/manifest_shape.rs` | AC-001 and the AC-006 / NF-002 source assertions. The crate checks its own manifest and its own public surface because no gate constant names it until HS-S0104 — the *"silently escapes three checks"* gap the story map calls out (`…/_storymap.md`, *Coverage* notes). |
| Integration (project ceiling) | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`) | Initiative DoD 13's not-regressed obligation: the gate is still green on the assembled whole with the new member in it. Not a claim about any peer. |
| Story grain | `cargo xtask affected --base main` | What this diff could break, which for a new leaf member should be a short list — a long one is a signal that something was wired further than intended. |
| Ledger | `_ledger.md` with cited evidence per AC-### (`.redkiln/config.yaml:62-67`, `require_ledger: true`) | DoD 3: a green gate proves the gate passed, never that the eleven criteria above are the things that passed. |

**Not run here, and not to be claimed:** `redkiln validate --kb && redkiln doctor` is the KB merge gate
for AC-001/AC-006/AC-013 of the *project*, and this story touches nothing under `.kb/`. Running it is
harmless; citing it as evidence for anything in this ledger is not.

## Risks and coupling (PR-scoped)

| risk | shape it takes | mitigation inside this PR |
| --- | --- | --- |
| **The fixture contract is written from the oracle and freezes the wrong shape.** | `MemorySyncPeer` satisfies almost any trait, so the suite goes green and the defect only surfaces at HS-S0112 when a Durable Object and a one-shot-HTTP peer have to be bent to fit. By then three stories depend on the shape. | The two conformant-variant fixtures (AC-007, AC-008) are written **in this PR**, beside the trait, precisely because a contract validated only by the oracle is validated by the thing it must be independent of. If either cannot be written cleanly, that is the finding, not a workaround. |
| **The crate escapes three gate checks until HS-S0104 merges.** | `RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, `lint-position-literals` and the changelog-per-rule lint all name `happenstance-testkit` alone; a green local gate here proves less than it looks. | Stated explicitly in Context pack §7 and in the Tests table above; the obligations those lints would catch are met by construction (AC-010) and by the crate's own `manifest_shape.rs`. The slice merges as a unit, top to bottom (`…/_storymap.md`, *Merge order* 3). |
| **The parent story is not actually done.** | If `IngestStore` or `MemorySyncPeer` still carry `todo!()` bodies, the reference fixture is a fixture over a panic and the harnesses are green because nothing ran. | Hard dependency on `ingest-store-and-memory-peer-round-trip` (HS-S0102). Before writing the fixture, confirm the `#![allow(clippy::todo)]` is gone from `crates/happenstance-sync/` — its presence is the tell. |
| **`publish = false` is lost by copying the sibling manifest.** | `crates/happenstance-testkit/Cargo.toml` has no `publish` key because it *is* publishable. A copy-paste start produces a publishable crate on the first line of the diff and breaks project AC-015 silently. | AC-001 is a test over the crate's own manifest, not a review item. |
| **Scope creep into rules.** | Writing "just one rule" to prove the registry works is the obvious temptation and it inverts the ordering the slice exists to protect: the registry would then be shaped around that rule. | The PR boundary forbids it and the Executive summary says why. The registry is proved by the two conformant variants and the meta-test, not by a rule. |
| **The SY-15 naming divergence gets "fixed" here.** | Editing `spec/SPECIFICATION.md:6298` to say `sync_peer_conformance!` is a one-character-class change to a `[FROZEN]` clause and is forbidden. | Recorded, not repaired (AC-011, Context pack §10). The repair is HS-S0114's under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`. |
| **A GAT sneaks into the fixture trait.** | `type Handle<'a> where Self: 'a` is the natural Rust reflex for a borrowed handle, and it is one of five independently necessary ingredients of an ICE that still reproduces on 1.97.1. | Owned associated types, stated in *Behavior and interfaces* with the citation; `experiments/rustc-ice-gat-foreign-trait/` is the reproduction if anyone doubts it. |

## Dependencies

**Blocks on**

- `ingest-store-and-memory-peer-round-trip` (HS-S0102) — supplies real `IngestStore` and `MemorySyncPeer`
  bodies. The reference fixture wraps that peer; a fixture over `todo!()` cannot be the thing a
  conformance suite is pointed at (`crates/happenstance-sync/src/memory.rs:8-10`).

Transitively, through HS-S0102: `memory-store-ingest-seam` (HS-S0101), and slice 1's three
decision-record stories — nothing in a `.rs` file a clause constrains merges before ADR-0026 and ADR-0027
do (`…/_storymap.md`, *Merge order* 1).

**Unlocks**

- `gate-mounts-for-the-sync-suite` (HS-S0104) — teaches `RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`,
  the position-literal and changelog lints, and the two new `wasm32` steps about the crate this story
  creates. Next in the slice, same context.
- `headline-rules-and-mutant-registry` (HS-S0105) — the first three rules and the mutant registry plug
  into `for_each_sync_peer_rule!` and `no_orphan_sync_rules` becomes load-bearing.
- Downstream of those: `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110), which instantiates one
  fixture type in both roles and consumes `BOTH_ROLES`; and `durable-object-and-neon-peers` (HS-S0112),
  which is DoD 4's proof artefact and inherits this story's declined-capability distinction.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Everything load-bearing enough to change what gets written is
here; everything already decided is inline in the Context pack.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/registry.rs` | The exact registry shape to reproduce: the `tt` capture and its stated reason at `:80-102`, the three emitters at `:228-292`, and `no_orphan_rules` at `:412-434`. Approximating any of the three is a defect that surfaces only at a crate boundary. | Before writing `src/registry.rs` — first file of the PR. | AC-003, AC-004, AC-005 |
| `crates/happenstance-testkit/src/lib.rs` | `:311-358` is the entry macro's four-arm cascade, the `__conformance_fixture` hoist and the `$crate::`-qualified emitter. The hoist is what makes every rule get its own fixture. | Before writing `sync_peer_conformance!`. | AC-002, AC-006 |
| `crates/happenstance-testkit/src/contract.rs` | The fixture contract's two hard-won negative results (`:76-84` no second flavour, `:97-111` no GAT), the `-> impl Future` reason (`:86-96`), the MUST-versus-trade line (`:137-146`), the limits-are-not-capabilities argument (`:216-232`), and `Capability`/`RuleOutcome` themselves (`:359-433`, `:465-473`). | Before declaring `SyncPeerFixture` and its constants. | AC-006, AC-007, AC-009 |
| `spec/SPECIFICATION.md` | The three `[FROZEN]` clauses that bind the trait's shape — SY-15 at `:6292-6300` (the round-trip-counting fixture peer, prescribed by name), SY-16 at `:6323-6340` (the owned token, and the in-memory-cursor `Rejects:`), SY-17 at `:6341-6360` (the `!Send` fixture as the compiled artefact) — plus SY-8 `:6070-6086`, SY-9 `:6090-6106`, SY-18 `:6363-6372` and SY-35 `:6867-6880`. Read the clause text, not the summary. | Before finalising the trait's methods, constants and payload policy. | AC-007, AC-008, AC-009, AC-010 |
| `crates/happenstance-sync/src/peer.rs` | `:43-50` states that the port *cannot express* one-round-trip-no-held-state, which is the whole argument for the counter living on the fixture; `:150-158` is `fn limits(&self) -> PeerLimits`, the reason no limit constants are copied onto the fixture. | When deciding where the counter lives and whether limits are constants. | AC-008, AC-009 |
| `crates/happenstance-sync/src/memory.rs` | The oracle the reference fixture wraps, and the file whose `todo!()` bodies HS-S0102 removes — the tell for whether the dependency is genuinely satisfied. | At the start, as a dependency check, and again when writing `fixtures::MemorySyncPeerFixture`. | AC-002 |
| `crates/happenstance-sync/src/ingest.rs` | `:31-37` explains why a third crate cannot write `impl IngestStore for SomeoneElsesStore` — coherence, with no blanket impl — which shapes what the fixture may ask of its associated types. | When choosing the bounds on `Local` and `Remote`. | AC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `:326-349` is CF-5's conformant-variant shape (`GappedPositionStore`, `PagedStreamStore`) — a variant that must pass *everything*, which is not a mutant and must not be registered as one. Both fixtures this story writes are that shape. | Before writing `one_shot_http_fixture.rs` and `send_free_fixture.rs`. | AC-007, AC-008 |
| `crates/happenstance-testkit/tests/memory_conformance.rs` | The mount point's exact shape — a `#![cfg(not(target_arch = "wasm32"))]` harness, one macro line, and a sibling module proving the fixture's own instrument can fail. Its blocking and `wasm32` siblings are the templates for the other two harnesses. | When creating `tests/memory_peer_conformance.rs` and its two siblings. | AC-002, AC-003 |
| `crates/happenstance-testkit/Cargo.toml` | `:4-14` is why a testkit carries its own `version` key — a rule added is semver-MINOR for the bar and nothing for the contract. Also the counter-example: it has **no** `publish` key, so it is not a template for the manifest. | While writing the new manifest, with the diff open. | AC-001 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | *Composition root* §1–§3 and §6 (what mounts where, and which gate constants HS-S0104 will teach), *AC-015 — the disposition on claiming the two crate names*, and the testing brief's *Fixtures and seams to mock* table, whose round-trip-counter and payload-bytes rows are this story's. | Before the manifest (AC-015 disposition) and before the fixture table decisions. | AC-001, AC-008, AC-010 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/sync-testkit-crate-and-rule-registry/discover.md` | *The wrong implementation* names the three defects this PR exists to refute, each with the citation that makes it concrete. The named anti-pattern set for this story, since `_design.md` records none. | When reviewing the finished diff against what it was supposed to reject. | AC-004, AC-005, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | The signed-off no-surface determination, and `:58-71`'s statement that the public API surface is the only surface and where its obligations are discharged. It is what makes the Interaction quality section above legitimate rather than a dodge. | Before writing the implementation report's design section. | AC-011 |
| `standards/rust/70-rustdoc-obligations.md` | The repository-wide rustdoc obligation in full, with the mechanism attached — CLAUDE.md deliberately does not repeat it. | Before writing doc comments on the new public items. | AC-011 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-3 (*a green `#[tokio::test]` says nothing about `Send`*) and RS-62-5 (doctests and packaging) — the first shapes how AC-007's `!Send` claim is worded, the second is NF-005. | When writing the doctests and the `!Send` assertion. | AC-007, AC-011 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-4's *"spell the oracle in a direction that shares no subroutine with the implementation"* — the conformant variants must not borrow `MemorySyncPeer`'s dispatch to look conformant. | While writing the two variant fixtures. | AC-007, AC-008 |
| `experiments/rustc-ice-gat-foreign-trait/` | The minimised reproduction behind "no GAT". Open it only if someone proposes a lifetime-parameterised associated type; it is the fastest way to end that conversation. | Only if a GAT is proposed. | AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted atom behind the single-flavour, `Send`-free shape — the authority, where `crates/happenstance-testkit/src/contract.rs` is the application of it. | If the `Send` question is reopened in review. | AC-007 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The three-part repair form. Read it to confirm that the SY-15 naming divergence is a *repair* owed to HS-S0114 and not something to do here. | When recording the divergence, before being tempted to edit the clause. | AC-011 |
| `xtask/src/main.rs` | `:420-436` is the `cargo package --list` assertion over the three publishable crates — the corroborating half of AC-001 and NF-001. | When verifying AC-001 rather than when writing the manifest. | AC-001 |
| `xtask/src/lints.rs` | `:33-45` — `TESTKIT_SRC` and `TESTKIT_MANIFEST`, both naming `happenstance-testkit` alone. Read to confirm what this crate is *not* yet subject to, so nothing here leans on a lint that cannot see it. | Before claiming any static-tier evidence in the ledger. | AC-010 |
| `RUNBOOK.md` | `:4516-4620` is phase 13; `:4576-4580` is this story's work item and `:4597-4602` the proof artefact the whole slice climbs toward. Orientation for why the empty instrument is the right milestone. | Once, if the ordering (registry before rules) is questioned. | AC-002 |

## Clarifications resolved during spec

1. **The AC set is the eleven the first pass enumerated — AC-001 through AC-011 — with none added or
   dropped.** They partition as: one for the crate's publishability (project AC-015), and ten for the
   suite's existence and shape (project AC-003, which the discover ledger splits into three obligations —
   the crate, the registry emission, the declension reporting).
2. **This story renders no surface, so the Interaction quality section is read against the API and the
   test output.** `_design.md` marks every composition heading `N/A` and `design.capture` a declared skip,
   while stating at `:58-71` that the public API surface is the only surface here. The composition
   invariants are therefore mapped onto placement, re-export, rustdoc-as-presentation and a stated density
   budget, and every one of them is carried by an existing `AC-###` row rather than by a prose bullet.
3. **The named anti-pattern set comes from `discover.md`, not from `_design.md`.** The design file records
   `N/A — no user-facing surface` under *Anti-patterns*, so the operative set is the three wrong
   implementations this story was written to reject, each already refuted by a compiled artefact in the
   PR.
4. **The Integration tier of the project testing brief is not this story's.** Three peers, two of them
   live, is HS-S0112's; this story's ceiling is Static plus Unit, plus `cargo xtask ci --fast` as a
   not-regressed check. The Tests table says so explicitly so no ledger row can cite a three-peer run that
   did not happen.
5. **`manifest_shape.rs` is a deliberate addition, not scope creep.** No gate constant names this crate
   until HS-S0104, so AC-001's `publish = false`, AC-006's no-redeclaration and NF-002's no-`Send`-bound
   would otherwise be review items rather than tests. A test the crate runs over itself is the cheapest
   thing that closes the *"silently escapes three checks"* window for one PR.
6. **The SY-15 naming divergence is recorded in the crate's own documentation, not only in this spec.**
   That is the difference between HS-S0114 finding it and HS-S0114 having to rediscover it: the spec is
   read by the implementer of *this* story, and the crate is read by everyone after.
7. **`EC-005`'s failure message must be written for a non-empty rules module even though the module is
   empty here.** A meta-test whose only exercise is the vacuous case ships a message nobody has read, and
   the first person to read it will be debugging a genuine orphan under time pressure.
