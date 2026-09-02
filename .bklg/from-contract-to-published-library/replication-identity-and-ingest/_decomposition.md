# Briefs — What a position means across a store boundary (HS-P0017)

Companion file, following `_grounding.md`'s precedent: not a redkiln item, no
system frontmatter. This artifact holds **all** of this project's briefs
(architecture / testing — `ux` and `deployment` are unwarranted for this project
per `.bklg/from-contract-to-published-library/_decomposition.md`, *Warranted
briefs*), one `##` section each. Grounding for every section is
[`_grounding.md`](_grounding.md); the AC spine is [`project.md`](project.md).

---

## Architecture brief

### Intent

Turn `crates/happenstance-sync/` from a phase-2 *instrument* into a port whose
answer about cross-boundary identity is written down, cited, and instrumented —
and mount that instrument into the machinery this workspace already has, rather
than beside it.

The crate says of itself: *"Status: a phase-2 sketch, not the protocol"*
(`crates/happenstance-sync/src/lib.rs:3`), bodies outside `memory` are `todo!()`
under a scoped allow (`crates/happenstance-sync/src/lib.rs:135-139`), and the
one thing it never got to is stated at `crates/happenstance-sync/src/lib.rs:116-119`:
*"Append conditions across a boundary. Still* the *central design question…
which is a position rather than an answer."*

This brief's job is to say **where each piece lands, what it wires to, and which
Accepted atoms and `[FROZEN]` clauses have already decided the question before
anyone opens an editor.** It decides no ADR — see *The two ADRs are not this
brief's to write*, below.

### The seam, in package terms

Per `CLAUDE.md`'s repository map and its dependency rule, this project touches:

| Crate / file | What lands there | Rule that governs it |
| --- | --- | --- |
| `crates/happenstance-sync/` | real bodies for `SyncPeer`, `IngestStore`, the runner; message set on the envelope; the scoped `#![allow(clippy::todo)]` is **removed** | port crate, not an adapter (`CLAUDE.md`, *Dependency rule* exception) |
| `crates/happenstance-sync-testkit/` **(new)** | `sync_peer_conformance!`, the sync rule registry, the peer fixture contract, the mutant registry | `crates/*` is a workspace glob (`Cargo.toml:3`), so the crate is a member the moment the directory exists |
| `crates/happenstance-cloudflare/`, `crates/happenstance-neon/` | the two structurally unlike peer impls (foreign trait, local type — the only place coherence allows them) | peer adapters depend on the port crate exactly as store adapters depend on `happenstance-core` (`CLAUDE.md`) |
| `crates/happenstance-core/` | **at most** one additive inherent operation on `MemoryEventStore` — see *The write-path seam* | `EventStore` itself does **not** grow (`RUNBOOK.md:450-455`, SY-8) |
| `xtask/src/main.rs`, `xtask/src/lints.rs`, `xtask/src/spec_trace.rs` | the gate steps and lint scopes that currently name only `happenstance-testkit` | see *Gate mounts* — this is where a suite silently escapes three checks |
| `CHANGELOG.md` | one entry per new rule, naming a defect | CF-29, enforced by `xtask/src/lints.rs:525` |
| `spec/SPECIFICATION.md` | **repairs only**, under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | project DoD 7 |
| `.kb/_intake/` → `.kb/decisions/`, `.kb/open-questions/`, `.kb/maps/` | ADR-0026, ADR-0027, the ADR-0003 amendment, the two resolutions | atoms are authored by `/redkiln:kb-ingest`, never by hand (`CLAUDE.md`, *Where the work lives*) |

**Not touched, and each names its owner.** `EventStore`'s trait signature
(constraint below); the Durable Object and Neon *stores* themselves (HS-P0013,
HS-P0014 — this project puts peer clothes on stores those projects built); the
registry (`publication-and-positioning`, and see AC-A10 below).

### Composition root, and what each capability mounts into

This is the section the implementer should read twice. Every artefact below has
a **real existing file it wires into**; none of them is a component that can be
finished in isolation and integrated later.

**1. `sync_peer_conformance!` mounts into the rule-registry pattern, not into a
new one.** The event-store suite's shape is: one macro holding the entire rule
enumeration and handing it to a *caller-supplied emitter*
(`crates/happenstance-testkit/src/registry.rs:94-102`), so tokio, blocking and
`wasm32` harnesses share one definition; the entry macro hoists the fixture
expression behind `async fn __conformance_fixture()` so the emitter never learns
the fixture's type (`crates/happenstance-testkit/src/lib.rs:315-342`). The sync
suite must be `for_each_sync_peer_rule!` + `sync_peer_conformance!` in that exact
shape (AC-003). Two mechanical details carry over and are easy to lose:

- the callback is captured as `$($callback:tt)+`, **not** `$cb:path`, and the
  comment at `crates/happenstance-testkit/src/registry.rs:95-101` says why —
  a parsed `path` fragment cannot sit in callee position, which would forbid
  `let names = for_each_sync_peer_rule!(...)` and with it the orphan-rule
  meta-test;
- `RuleOutcome` is `#[must_use]` (`crates/happenstance-testkit/src/contract.rs:471`),
  which is what makes "an emitter must report a skip" a build failure rather
  than prose.

**2. The declined-capability contract is reused, not re-invented.**
`Capability` and `RuleOutcome` already exist at
`crates/happenstance-testkit/src/contract.rs:368-433` and `:473-483`, with the
argument for reporting rather than `#[cfg]`-ing a skipped rule at `:31-42`.
`happenstance-sync-testkit` should **depend on `happenstance-testkit`** for those
two types and for the three emitters, and define only what is genuinely different:
a peer fixture whose associated types are a `SyncPeer` and an `IngestStore`-capable
store. `happenstance_testkit::Fixture` cannot be reused directly — its
`type Store: EventStore` (`crates/happenstance-testkit/src/contract.rs:125`) is
event-store-shaped and says nothing about a peer. Two constraints on the new
fixture trait, both learned already:

- **no `Send` flavour and no `trait_variant`** on the fixture, for the reason at
  `crates/happenstance-testkit/src/contract.rs:76-84` (nothing ever spawns a
  fixture, and a second flavour would exclude precisely the adapters ADR-0001
  exists for);
- **owned associated types, never a GAT** — `type Store<'a> where Self: 'a` on a
  foreign trait is one of five ingredients of the rustc ICE this repository
  already minimised, still reproducing on 1.97.1
  (`crates/happenstance-testkit/src/contract.rs:97-111`,
  `experiments/rustc-ice-gat-foreign-trait/`).

**3. The mutant registry mounts onto the existing `const` table shape.** CF-2/CF-4's
mechanism is implemented once already: a `Declared` table whose entries carry
`fails: &'static [&'static str]`, a **never-empty** `provenance` naming the real
adapter shape that makes the mutant plausible, and per-rule `expect` pins naming
the exact assertion the mutant should trip
(`crates/happenstance-testkit/tests/mutation_coverage.rs:140-175`; worked
provenance strings at `:325-350`). AC-004's sync mutant registry is that shape
with peer-shaped provenance — *"a receiver that calls `append(events,
Some(&origin_condition))`"* is already written for us as SY-1's `Rejects:`
(`spec/SPECIFICATION.md:5856-5867`).

**4. The three peers mount into three different crates, and the middle one is a
retirement.**

- `MemorySyncPeer` is real today (`crates/happenstance-sync/src/memory.rs:1-10`
  — *"Unlike the two stand-ins in this crate's `tests/`, the bodies here are
  real"*) and is the oracle and doctest target. **Discrepancy to settle:**
  `RUNBOOK.md:4581-4585` asks for it *behind a `memory` feature*, but
  `crates/happenstance-sync/src/lib.rs:145` declares `pub mod memory;`
  unconditionally while `crates/happenstance-sync/Cargo.toml:44` scopes `memory`
  to the `IngestStore for MemoryEventStore` coherence proof only
  (`crates/happenstance-sync/src/ingest.rs:206`). Decide it deliberately; both
  answers are cheap while `publish = false`
  (`crates/happenstance-sync/Cargo.toml:12`) and neither is cheap afterwards.
- `crates/happenstance-sync/tests/real_peer_shapes.rs` holds the two `todo!()`
  stand-ins with *"the type properties of the real thing and none of its
  dependencies"* (`:16-20`). Its own header states the honest limit of that
  evidence. When the real impls land in `happenstance-cloudflare` and
  `happenstance-neon`, this file's stand-ins are **superseded, and the file's
  finding is not** — it is the transcript for ADR-0026's *"the type checker did
  not force that choice"* (`crates/happenstance-sync/src/lib.rs:130-132`).
  Retire the `todo!()`s, keep the record.
- `crates/happenstance-sync/tests/cursor_shape_probe.rs` is the compiled proof
  that `impl Stream` admits the one-shot-HTTP peer *by buffering and replaying*
  — "legal, `Send`, and a lie" (`crates/happenstance-sync/src/peer.rs:43-50`).
  It must keep compiling; ADR-0026 cites it for why `pull` returns a batch.

**5. The write-path seam is the one place this project can reach the published
contract crate, and it must do so in its mildest form.** This is the residual
risk `RUNBOOK.md:462-466` refuses to assume away, arriving on schedule.
`crates/happenstance-sync/src/ingest.rs:52-71` states it precisely: phase 4 gave
`SequencedEvent` an `id` field, so a foreign identity now has *a place to sit and
no door to come in through* — `EventStore::append` mints
`EventId::new(self.store_id, position)` for every event it writes, and
`MemoryEventStore::restore` (`crates/happenstance-core/src/memory.rs:163`) builds
a **new** store from an owned snapshot while `ingest` holds `&self`. Four options,
with their costs:

| Option | Cost | Verdict |
| --- | --- | --- |
| (a) `happenstance-core` grows an **inherent** `&self` operation on `MemoryEventStore` that accepts an already-identified `SequencedEvent` | additive on a concrete type in a published crate: semver-minor, no trait change, no adapter obliged to have an opinion | **recommended** — smallest surface that keeps the coherence proof and the oracle |
| (b) grow `EventStore` with an ingest-shaped method | forbidden: `RUNBOOK.md:450-455`, SY-8 (`spec/SPECIFICATION.md:6070-6086`), and CLAUDE.md's own scope exclusion | **refused** |
| (c) drop `impl IngestStore for MemoryEventStore` and give the sync crate its own store | loses the "local trait, foreign type" coherence proof (`crates/happenstance-sync/src/ingest.rs:26-31`) and the doctest target | fallback only |
| (d) do nothing — real adapters own their types and write `impl IngestStore for MyStore` in their own crate with their own private write path | correct for every real adapter, and **no help at all** for `MemoryEventStore`, which core owns | true, and not sufficient |

Note what (d) means for scope: the two networked peers need **no core change**;
coherence lets them write the impl themselves. Only the in-memory oracle needs
core's cooperation, which is why the recommendation is (a) and why the blast
radius is one additive inherent method rather than a port change. If a *real*
peer turns out to need a seam on the **port**, that is `RUNBOOK.md:462-466`'s
residual risk landing hard: stop, and raise a new decision atom and a re-plan.

**6. Gate mounts — where a sync suite silently escapes three checks.** Every one
of these is a hard-coded single-crate constant today:

- `xtask/src/spec_trace.rs:85-89` — `RULE_FILES: [&str; 3]`, all three under
  `crates/happenstance-testkit/src/`. This is the set `spec-trace` sweeps so that
  every rule is claimed by a clause and every clause resolves to a rule. A sync
  rules file that is not in this array means every `SY` clause keeps rendering as
  scheduled and no sync rule can ever be orphan-checked. **AC-014 cannot pass
  without this edit.**
- `xtask/src/lints.rs:42` — `TESTKIT_SRC`, CF-33's no-clock scope; `:45` —
  `TESTKIT_MANIFEST`, CF-32's own-version check. Both name `happenstance-testkit`
  only. `happenstance-sync-testkit` must carry its own `version` key for the same
  reason (`xtask/src/main.rs:420-436`: adding a rule is semver-MINOR for the bar
  and nothing for the contract), and the lint must be taught to look at it.
- `xtask/src/main.rs:203-283` — the four `wasm32` steps. **AC-009's mount** is a
  fifth and sixth: a `wasm32` build of `happenstance-sync` and a `wasm32` check
  of the *sync* conformance harness, beside the existing contract-crate,
  harness, Cloudflare and Neon steps. `xtask/src/main.rs:559-590` is the
  `wasm32` feature powerset, which is where the sync crates' feature
  combinations belong. AC-009 says *"built and exercised for `wasm32` inside the
  gate rather than asserted in prose"* — this is the file that decides whether
  that sentence is true.
- `CHANGELOG.md` via `xtask/src/lints.rs:525` — one entry per rule naming a
  defect. The first run of that lint found 25 of 55 rules with no entry
  (`xtask/src/main.rs:394-405`); a new suite starts at zero.
- `lint-position-literals` (`xtask/src/main.rs:375-392`) — DR-7/CF-6. No sync
  rule may assert a literal position, and `GappedPositionStore`
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`) is the
  conformant variant that enforces it for real.

**7. The wire mounts *inside* `Envelope<T>`, never around it.**
`crates/happenstance-sync/src/wire.rs:8-16` is explicit: `Envelope` is generic in
`T` and *is not* an enum of message kinds, because an
`enum Message { Push(..), Pull(..) }` would put the message set beside the version
check — and the message set is what this project designs. So the message set is
new types that instantiate `T`; the envelope itself does not change shape. Two
consequences the implementer inherits:

- adding `#[derive(Serialize, Deserialize)]` to `PushBatch`, `EventGroup` or
  `ReplicatedEvent` **is** a wire-format decision, stated as such at
  `crates/happenstance-sync/src/lib.rs:86-94` ("a `#[derive]` on a public message
  type *is* a wire format… **what travels is phase 13's**"). It belongs in
  ADR-0027, not in a commit;
- `FORMAT_VERSION` bumps on a **shape** change and never on a capacity bound
  (`crates/happenstance-sync/src/wire.rs:46-65`, WF-9). Whether it bumps at all
  is a consequence of the message-set design, and the open question
  `.kb/open-questions/sync-message-set-and-format-version.md` is exactly the
  question of whether the version is per-message or per-connection-negotiated.
  WF-8's before-any-decoding obligation is discharged by the hand-written
  `Deserialize`, and the derive is not an option for the decoding half
  (`crates/happenstance-sync/src/lib.rs:78-84`).

### The Accepted atoms that constrain this, and where they bite

- **ADR-0001** (`.kb/decisions/0001-async-port-flavours.md`) — no
  `#[async_trait]`; two flavours via `trait_variant`. Both new ports already
  comply (`crates/happenstance-sync/src/peer.rs:81`,
  `crates/happenstance-sync/src/ingest.rs:119`). The place it bites is the
  **runner**: SY-17 is `[FROZEN]` (`spec/SPECIFICATION.md:6341`) and CLAUDE.md
  constraint 4 applies — bind `EventStore`/`SyncPeer`/`IngestStore`, never the
  `Send` flavours, and import only one name of each pair per module or method
  calls go ambiguous with `error[E0034]`
  (`crates/happenstance-sync/src/peer.rs:26-28`). AC-009's tripwire exists
  because the `!Send` peer sits **mid-chain**, which is exactly the arrangement
  that tempts a `SendEventStore` bound.
- **ADR-0003** (`.kb/decisions/0003-opaque-payloads.md`) — the lift condition is
  in the atom's own Status section: *"It lifts at phase 13, when
  `happenstance-sync` round-trips an event between two stores without
  deserialising its payload"* (`:88-92`). That sentence **is** AC-006. Cite it;
  do not restate it as a new condition. See the tension below on *how* it lifts.
- **ADR-0009** (`.kb/decisions/0009-error-send-sync.md`) — `Error` keeps exactly
  `core::error::Error + 'static` on every port and flavour, with the stronger
  property in a downstream marker trait. `spec/SPECIFICATION.md:7006-7019` says
  the answer *"was taken for the whole workspace at once, `SyncPeer` included,
  which is why this section neither adds to it nor may diverge from it."* The
  sketch already conforms (`crates/happenstance-sync/src/peer.rs:91`,
  `crates/happenstance-sync/src/ingest.rs:122`) — the risk is a runner that adds
  `+ Send + Sync` on the way past.
- **ADR-0013** (`.kb/decisions/0013-position-assignment-and-visibility.md:24-30`)
  — the visibility invariant is global rather than per-boundary, and a
  `SequencePosition` is a within-one-store visibility predicate, not an identity.
  This is the *textual source* for this project's own title. ADR-0026 cites it;
  it does not re-derive it.
- **ADR-0016** (`.kb/decisions/0016-the-wire-format.md`) — the format is
  happenstance's own; `Envelope<T>` and the hand-written `Deserialize` ship; the
  replication protocol, message set and `SyncError` extension are explicitly
  **not** settled there and are named as this project's
  (`.kb/open-questions/sync-message-set-and-format-version.md:19-23`). Build on
  the envelope, not around it.
- **Governance and playbooks that apply directly:**
  `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` (the
  repair-vs-gap test, used below for AC-012 and for the `(new)` markers);
  `.kb/playbooks/one-decision-per-adr-title.md` (DR-1);
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md`.

### Tension 1 — the two ADRs are not this brief's to write, and they gate the code

AC-001 requires ADR-0026 and ADR-0027 **accepted and merged before the code they
constrain** (`RUNBOOK.md:4606`). Nothing in the planning grain may author them:
`CLAUDE.md` states atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`,
never by hand, and the one attempt to hand-write them was reverted (`0269720`).

**So the sequencing the implementer inherits is:** the first slice writes the two
ADRs as `.kb/_intake/` documents (long form under `references/adr/0026-*.md` and
`0027-*.md`, per `CLAUDE.md`'s two-places-on-purpose rule), runs
`/redkiln:kb-ingest`, and only then opens a `.rs` file that a clause constrains.
The atoms' numbers are already reserved in the ADR queue at `RUNBOOK.md:305-306`;
`.kb/decisions/` today holds 0001–0016 and 0029 only, so nothing collides.

What this brief records instead of drafting them: **what each must answer**, in
the next section, and the split that makes AC-014's arithmetic come out.

### Tension 2 — the "central question" is already answered, and it is `[FROZEN]`

AC-002 and the intake brief both read as though *does ingest re-check the
writer's asserted append conditions* is open. It is not. Two clauses answer it
and both are `[FROZEN]`:

- **SY-1** (`spec/SPECIFICATION.md:5841-5867`) — ingest MUST NOT refuse for any
  reason that is a function of the receiving store's state, and MUST NOT evaluate
  an `AppendCondition` — its own or the origin's — as a precondition. Rule:
  `ingest_never_rejects`.
- **SY-6** (`spec/SPECIFICATION.md:5973-6029`) — a wire-carried condition is
  evidence, not an instruction, and one carrying a position-relative boundary
  MUST be refused as ingest input. Rule: `wire_condition_with_after_is_refused`.

`spec/SPECIFICATION.md:5798-5836` carries the convergence argument in full, plus
the three cases that look like counter-examples and are not — Cold Chain's
"re-checking" is *"a domain decision about facts already accepted, running after
the ingest, not a gate in front of it"* (`:5825-5827`), and Wattline is *"SY-1
with an empty conflict set"*. SY-2 (`:5871-5900`) is the frozen answer to what
happens instead: losing event and compensation in one atomic `append`.

**ADR-0026's job on AC-002 is reconciliation and citation, not deliberation.**
Re-opening it would be amending a `[FROZEN]` clause, which CLAUDE.md forbids
without a new ADR and which the project's own *Out of scope* excludes from the
whole initiative. If the implementation genuinely finds SY-1 or SY-6 wrong, the
output is a recorded finding and a re-plan — never a quieter version of the
clause.

### Tension 3 — the clause ledger, re-derived (AC-014, and the project's own risk row)

The intake brief's *counts* are right and its *ranges* are wrong. Computed from
`spec/SPECIFICATION.md` at HEAD:

- **`[FROZEN]` (21):** SY-1, 2, 3, 4, 5, 6, 8, 9, 11, 12, 13, 15, 16, 17, 19,
  24, 25, 26, 33, 34, 35
- **`[PROVISIONAL]` (9):** SY-7, 10, 20, 21, 22, 23, 29, 30, 31
- **`[DEFERRED]` (5):** SY-14, 18, 27, 28, 32

Also: the wire clauses are **`WF-*`**, not `VT-*` (`spec/SPECIFICATION.md:1892`
onward); `VT-*` is section 2's value-type family. The intake brief's ledger is
superseded by the paragraph above, per its own last gate box ("*Where this brief
and `SPECIFICATION.md` disagree, the specification wins*").

**The arithmetic defect to fix before drafting, not after.** `RUNBOOK.md:4535`
says phase 13 discharges SY-1 – SY-35. The intake brief's split gives ADR-0027
SY-1 – SY-7 and SY-19 – SY-31, ADR-0026 SY-8 – SY-18, and hands SY-32 off — which
leaves **SY-33, SY-34 and SY-35 assigned to nothing**. AC-014 requires the union
of the two ADRs' clause ranges to *equal* this project's stated range at exit, so
assign them explicitly at the ADR outline: SY-33/SY-34 are transport-refusal
clauses (peer-shaped, ADR-0026) and SY-35 is the "everything replication reasons
about is in the tags" clause that DR-5 rests on (reconciliation-shaped,
ADR-0027). Fix the split; do not fix the total by editing the RUNBOOK.

**Named experiments already on file for the five deferrals** (AC-010 — a renewal
with no experiment is a build failure under CF-38, `spec/SPECIFICATION.md:213-217`):
SY-14, SY-27 and SY-28 cite `references/evaluation/PRESSURE-TEST.md:685-693`
(the whole-log-versus-scoped experiment, and the round-trip rule's ability to
assert log equality is its consequence); SY-18 cites the Turnstile KV-backed peer
shape; SY-32 is the retention case. All five must be *settled or renewed by
name*, and the file is present — `references/evaluation/PRESSURE-TEST.md` — so
the citations resolve.

**WF-1's interoperability half** (`spec/SPECIFICATION.md:1892-1921`) is
`[DEFERRED]` on the strongest available ground: the DCB specification and its
reference TypeScript library publish **no** wire format at all. `RUNBOOK.md:4593-4595`
requires this be recorded inside **ADR-0026's envelope section** — "named,
deferred, with the experiment being a specific external implementation to
interoperate with. Not silence." That is where AC-010's WF-1 half and
`.kb/open-questions/dcb-reference-publishes-no-wire-format.md`'s resolution land
together.

### Tension 4 — AC-012's audit is a pre-commitment, not a hunt

The intake brief names "two mis-worded ingest clauses" without ids. A targeted
sweep for `re-check|recheck|re-evaluat` across `spec/SPECIFICATION.md` returns
five substantive hits, and **every one reads as consistent with SY-1/SY-6 as
frozen**: `:4373` (ES-38's `Rejects:`, which *presupposes* unconditional ingest
and calls the vacuous pass "the *normal* path"), `:5825` (the Cold Chain
paragraph, which distinguishes a post-ingest domain decision from a gate),
`:5862`, `:5998` and `:6014` (SY-1's and SY-6's own `Rejects:` fields). This
pass found **no clause still describing ingest as re-checking conditions**. The
implementer should re-run the sweep at implementation time and record the result
either way — a null finding recorded is worth more than a null finding assumed.

What *is* live is the inverse, and it is a pre-commitment this project owes:

- **SY-1, SY-6 and SY-12 all name a wrong implementation that depends on today's
  code.** SY-1 and SY-6 name the **public** `guard: Option<AppendCondition>`
  field on `EventGroup` (`crates/happenstance-sync/src/peer.rs:236-243`) —
  *"prose is not a type constraint… a receiver is handed exactly the value it
  would need to re-evaluate"*. SY-12 (`spec/SPECIFICATION.md:6153-6183`) already
  records that its original exemplar was withdrawn and that the crate now mints
  `(StoreId, SequencePosition)` — confirmed live at
  `crates/happenstance-sync/src/identity.rs:87-124`. All three currently name a
  wrong implementation that still exists.
- **Therefore:** if ADR-0026/ADR-0027 make `guard` private, rename it, or remove
  it, those clauses lose their named wrong implementation *in the same commit*,
  and a clause whose `Rejects:` names nothing that exists rejects nothing. Owe
  them a repair in that commit, in the form
  `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` prescribes:
  **the MUST stays verbatim**, the discharge is named as a discharge, and the
  code and test that assert it are cited.

**The `(new)` markers are a repair, and DoD 7 must not be read as forbidding
them.** `xtask/src/spec_trace.rs:1626-1630` treats a `Rule:` line containing
`(new)` (or `†`, or a leading `new `) as *scheduled*, so the rule is never
resolved. The moment `ingest_never_rejects` and
`compensation_is_atomic_with_the_losing_event` exist, SY-1's and SY-2's `Rule:`
lines must lose `(new)` or `spec-trace` keeps reporting them unwritten. Apply the
playbook's mechanical test: the set of implementations the clause admits is
**unchanged**, so this is a **repair**, not an amendment. Say so in ADR-0026 so
that DoD 7's "*`git diff` over `spec/SPECIFICATION.md` shows only additions a
decision record authorises*" is satisfied by an authorisation rather than by an
argument after the fact.

### Tension 5 — ADR-0003 cannot lose `provisional` by editing ADR-0003

AC-006 and the project's DoD both say ADR-0003's `provisional` marker is lifted.
`.kb/decisions/0003-opaque-payloads.md` is `status: accepted`, and **an accepted
decision atom is immutable** — `redkiln validate --kb` checks each one against
`HEAD` (`CLAUDE.md`, *Where the work lives*). Editing its Status section fails
validation, which is DoD 5's own gate.

The workspace already has the pattern: **ADR-0029 amends ADR-0004 without
touching it** — `supersedes: null`, `depends_on: [kb-decision-0004]`, and a
summary that says in its own words *"This amends ADR-0004 rather than superseding
it — that decision's body stays verbatim"*
(`.kb/decisions/0029-msrv-raised-to-1-97-1.md:12-26`). Recommended shape: the
lift rides inside **ADR-0026** (the atom whose round-trip evidence proves it),
declaring `depends_on: kb-decision-0003` and stating the lift and its evidence in
its summary. A standalone atom is equally legitimate and costs an ADR number the
RUNBOOK queue does not carry. Either way: **a new atom, authored through
kb-ingest, never an edit.** If the round trip cannot be made byte-identical, AC-006's
second arm applies — record why it cannot lift, which is also a new atom.

### Data flow, one pass end to end

Every hop below already has a type in the tree; nothing here is proposed.

```
origin store: append(events, Some(&condition))
  → SequencedEvent { position, id: EventId, recorded_at, event }   (core; ES-41)
  → ReplicatedEvent { id, recorded_at, event }                     (identity.rs:171-180)
  → EventGroup { guard: Option<AppendCondition>, events }          (peer.rs:236-243)
        guard travels as EVIDENCE — SY-1, SY-6, both [FROZEN]
  → PushBatch { groups }                                           (peer.rs:191-196)
        decomposition is explicit on the wire, never re-inferred (SY-30)
  → Envelope<T> { format_version, message }                        (wire.rs; WF-8)
  → transport (SyncPeer::push / SyncPeer::pull → Pulled { batch, resume })
receiver:
  → dedupe on EventId via IngestStore::holds / EventStore::contains_event_id
  → IngestStore::ingest: each group atomic; foreign EventId preserved;
    event lands at the LOCAL TAIL with a fresh local position (SY-5, SY-19)
  → Ingested { appended, skipped, last_local }                     (ingest.rs:178-191)
  → Ack { appended, skipped, confirmed: Watermark }                (peer.rs:253-272)
        the watermark is a confirmation, NOT an event (SY-31)
  → runner advances the owned, Clone resume token                  (peer.rs:95-118, SY-16)
```

**The payload `Bytes` are never touched at any hop.** That is ADR-0003's payoff,
DR-5's rule, and SY-35's clause simultaneously — and AC-006's byte-identity
assertion is the measurement that turns it from a claim into evidence. A suite
that parsed `data` would certify a peer that does
(`RUNBOOK.md:4576-4580`).

### AC-011 — SY-32 is a handoff, and the DAG edge is already right

`spec/SPECIFICATION.md:7000-7003` states plainly that SY-32 depends on ES-39 and
cannot be settled ahead of it; `RUNBOOK.md:4641-4644` assigns ES-39 to **ADR-0028
under `retention-and-incomplete-logs`**. `PeerLimits::retention_floor` already
exists on the port (`crates/happenstance-sync/src/peer.rs:296-301`) — what is
missing is not a field but the store-side primitive that lets a store say what it
does not hold.

So ADR-0026 **records the dependency by name and cannot discharge it**. The
`replication → retention` edge in
`.bklg/from-contract-to-published-library/_decomposition.md` (*Dependency DAG*,
ranks 4 → 5) is already ordered correctly for this: retention runs after
replication, which is what a handoff needs. **No edge flip; confirmation only** —
and AC-011 is satisfied by writing that confirmation down, not by re-deciding it.
The project's own risk row ("three documents, two answers") resolves the same
way: `RUNBOOK.md:4535`'s "SY-1 – SY-35" is the phase's *range*, and SY-32 leaves
it as an explicit, named handoff rather than as a settled clause.

### AC-015 — the disposition on claiming the two crate names

AC-015 requires this disposition be *recorded in the architecture brief rather
than taken silently*. Here it is, with the conflict stated first.

`RUNBOOK.md:4552-4555` opens phase 13's work list with *"Claim `happenstance-sync`
and `happenstance-sync-testkit` on crates.io, per phase 0's rule that a name is
reserved when its phase starts."* The charter's *Out of scope*, the decomposition
(`.bklg/from-contract-to-published-library/_decomposition.md:47`) and this
project's own *Out of scope* all forbid publishing either sync crate in this
release train. **A name reservation is a placeholder publish** — it requires
`publish = true`, puts a version on the registry, and drags both crates into the
gate's `cargo package --list` assertion that each publishable crate carries both
licence files and a README (`CLAUDE.md`, *Commands*).

**Recorded disposition: this project does not claim either name.** Both crates
keep `publish = false` (`crates/happenstance-sync/Cargo.toml:12`, and the new
testkit is created with the same line) for the duration. The reasons, in order:
the exclusion is stated in three artifacts and the claim in one; a placeholder
publish is a publish, and DoD 7-shaped audits will read it as one; and the crate
that justifies the name is what phase 0's rule is *for*, which means the claim
belongs to whoever owns the release train that ships them — `publication-and-positioning`
owns "what ships and when" and this project does not.

**This is a recommendation with a named owner, not a settlement.** If the human
or HS-P0016's owner prefers to reserve the names defensively, that is a
legitimate answer and the change is one line per manifest plus the two licence
files and a README each — but it is then a *release-train* decision recorded
there, not a side effect here.

### Non-prescriptive implementation notes

Things the implementer should decide, with the constraint attached and no answer
prescribed:

- **Where the runner lives and what it is called.** `RUNBOOK.md:4539-4543` and
  `crates/happenstance-sync/src/lib.rs:42-50` settle only that fan-out, ordering
  and merge policy sit *above* the port, in the same division of labour that puts
  the projection runner above `ProjectionStore`. Whether that is one type, a
  trait, or a function is open — but "add a second peer" must stay a runner
  configuration rather than a breaking change to the port (`peer.rs:65-73`).
- **Whether hub-and-spoke and peer-to-peer are one abstraction or two.** SY-9 is
  `[FROZEN]` (`spec/SPECIFICATION.md:6090-6106`) — hub-ness is an *edge*
  property, not a property of the port's type or constructor, because a store in
  the middle of a chain is one peer wearing each hat. SY-10 is `[PROVISIONAL]`
  and is where the abstraction question actually lives. AC-007 additionally
  requires `crates/happenstance-sync/src/lib.rs`'s module doc to stop describing
  only one topology — it currently describes both at `:52-66`, so verify what
  AC-007 is actually asking for against the file before rewriting it.
- **How much of `SyncError` grows.** ADR-0016 deliberately left it untouched
  (`crates/happenstance-sync/src/lib.rs:91-94`), naming the extension as
  ADR-0026's. `WireError` stays a decoding failure and is not folded in.
- **Round-trip counting.** `crates/happenstance-sync/src/peer.rs:43-50` states
  that the port *cannot express* "one round trip with no held state", and that it
  must be checked by a fixture peer that counts its own round trips. That is a
  conformance-rule design constraint for the testing brief and a fixture-shape
  constraint here: the peer fixture needs somewhere for that counter to live.
- **Infrastructure confirmation, at slice 1 rather than at implementation.** This
  project carries **no deployment brief**, so the Durable Object and Neon
  environments the two unlike peers need must already exist from HS-P0013 and
  HS-P0014. The Neon axis *cannot be faked without destroying the thing it exists
  to test* (project.md, risk table). Confirm both environments before the peer
  slices start; if either is absent, that is a blocker to raise, not a stand-in
  to write.

### Acceptance Criteria

Architecture-grain and checkable by reading the diff. Each traces to the project
AC it serves.

- **AC-A01 — No new port lands outside `happenstance-sync`.** `EventStore`'s
  trait signature in `crates/happenstance-core/src/store.rs` is byte-identical
  before and after this project. Any core change is an *inherent* method on a
  concrete type, and its semver class is stated in the ADR that authorises it.
  *(project AC-002, DoD 7; `RUNBOOK.md:450-455`)*
- **AC-A02 — The suite is emitted through the registry pattern.**
  `sync_peer_conformance!` expands via a `for_each_sync_peer_rule!` that captures
  its callback as `tt` and hoists the fixture behind `__conformance_fixture`, and
  the tokio, blocking and `wasm32` harnesses share one rule enumeration.
  *(project AC-003)*
- **AC-A03 — `Capability` and `RuleOutcome` are reused, not re-declared.**
  `happenstance-sync-testkit` depends on `happenstance-testkit` for both, and a
  declined capability reports the fixture's stated reason rather than vanishing.
  *(project AC-003)*
- **AC-A04 — The peer fixture is single-flavour and GAT-free.** No
  `trait_variant` on the fixture trait, and no `type X<'a> where Self: 'a`.
  *(`crates/happenstance-testkit/src/contract.rs:76-111`)*
- **AC-A05 — Every gate constant that names `happenstance-testkit` has been
  taught about `happenstance-sync-testkit`, or a written reason says why not.**
  Specifically `xtask/src/spec_trace.rs:85-89` (`RULE_FILES`),
  `xtask/src/lints.rs:42` (`TESTKIT_SRC`) and `:45` (`TESTKIT_MANIFEST`).
  *(project AC-004, AC-014)*
- **AC-A06 — The `wasm32` path is a gate step, not a claim.**
  `xtask/src/main.rs`'s wasm step list contains a build of `happenstance-sync`
  and a check of the sync conformance harness, and `cargo xtask wasm` runs them.
  *(project AC-009)*
- **AC-A07 — The runner binds the weaker flavour.** No `SendEventStore`,
  `SendSyncPeer` or `SendIngestStore` appears in a generic bound on the ingest
  path or the runner, and no module imports both names of a pair.
  *(project AC-009; CLAUDE.md constraint 4)*
- **AC-A08 — The message set instantiates `Envelope<T>`; the envelope does not
  become an enum.** Any derive added to `PushBatch`, `EventGroup` or
  `ReplicatedEvent` is authorised by ADR-0027 by name.
  *(`crates/happenstance-sync/src/wire.rs:8-16`,
  `crates/happenstance-sync/src/lib.rs:86-94`)*
- **AC-A09 — Nothing in `.kb/` was hand-authored.** ADR-0026, ADR-0027, the
  ADR-0003 amendment and both open-question resolutions arrive through
  `.kb/_intake/` and `/redkiln:kb-ingest`; `.kb/decisions/0003-opaque-payloads.md`
  is unchanged in the diff; `redkiln validate --kb` passes.
  *(project AC-001, AC-006, AC-013)*
- **AC-A10 — `publish = false` still reads `false` on both sync crates at exit**,
  and the name-claim disposition above is the recorded answer.
  *(project AC-015)*
- **AC-A11 — Every clause whose `Rejects:` names a symbol this project changed
  has a repair in the same commit**, in the playbook's three-part form, and each
  repair's justification names the mechanical test (implementation set unchanged).
  *(project AC-012, DoD 7)*
- **AC-A12 — The clause split adds up.** The union of ADR-0026's and ADR-0027's
  stated clause ranges equals SY-1 – SY-35 minus SY-32's named handoff, computed
  and written down at exit rather than asserted.
  *(project AC-014, DR-9)*

### Notes

**What this brief deliberately does not decide.** The conformance rule set and
its mutants (testing brief, and DR-4's "name the wrong peer it rejects" bar); the
merge rule and the compensation contract (ADR-0027's, and re-deliberating them
here would be the side-effect authorship the repository has already reverted
once); whether replication is whole-log or scoped (SY-27/SY-28, deferred against
a named experiment at `references/evaluation/PRESSURE-TEST.md:685-693`); and the
`SyncRunner`'s concrete shape.

**One thing to carry forward that no AC captures.** `crates/happenstance-sync/`
is currently the most honestly self-documenting crate in the workspace — it
states what its sketch proved, what it did not, and where the type checker
declined to make a choice for us (`src/lib.rs:106-133`, `src/peer.rs:43-50`,
`src/ingest.rs:64-71`). Implementing the crate will delete most of that prose
because it will stop being true. **The findings are not the same as the
`todo!()`s.** Move each finding into the ADR that consumed it before deleting
the paragraph that holds it, or this project quietly discards the evidence the
phase-2 sketch was built to produce — which is the whole reason the sketch landed
nine phases early (`RUNBOOK.md:462-466`).

---

## Testing brief

### Intent

Say, tier by tier, **which command proves which AC**, so that "green" and "done"
stay the same claim. This project's proof artefact is a suite, not a feature
(`project.md`, DoD 4), so the testing brief is not a bolt-on section — it is
where most of the fifteen ACs actually cash out. Nothing below invents a
mechanism: every tier reuses a shape `happenstance-testkit` already proved, per
the architecture brief's *Composition root* — this brief only says which project
AC each reused piece discharges.

The mix has a structural asymmetry the brief states up front rather than lets
the implementer discover mid-story: two of three peers are **not mockable**.
`RUNBOOK.md:462-466`'s residual risk and the project's own risk table both say
the Neon axis "cannot be faked without destroying the thing it exists to test"
— a one-shot-HTTP peer with no interactive transaction is a claim about what a
*real* constraint permits, and a mock of it would assert only what the mock's
author already believed. So the integration tier below has a live half and an
in-process half, and DoD 4's "two structurally unlike" is proven by the live
half or it is not proven.

### The test mix, tier by tier

**Static — reused gate steps, taught about a fourth suite.** No new mechanism;
the architecture brief's *Gate mounts* (§6) is the change list. This tier
discharges the ACs that are claims about the *shape* of the suite rather than
about any one peer's behaviour:

- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`
  — unchanged, applies to the two new crates the moment they exist.
- `cargo xtask spec-trace` — the mechanism DoD 2 and AC-014 name by name: every
  `SY`/`WF` clause's `Rule:` resolves to a rule that exists, no rule is orphaned,
  and a `[DEFERRED]` clause with no named experiment is a build failure under
  CF-38 (`xtask/src/spec_trace.rs:1`, `:1092`). This is the check for AC-002 (no
  `[FROZEN]` clause silently re-opened), AC-010 (every deferral renewed by
  name), AC-012 (a `Rejects:` symbol that no longer exists is a repair, and
  `spec-trace` is what would otherwise let it rot unnoticed) and AC-014 (the
  clause-range arithmetic itself).
- `cargo xtask lint-position-literals` (CF-6) and a new sync-suite scope for
  `TESTKIT_SRC`/`TESTKIT_MANIFEST` (CF-33, CF-32,
  `xtask/src/lints.rs:33-45`) — extended per architecture brief §6, second
  bullet, to `happenstance-sync-testkit`. Proves AC-004's "no literal position"
  half and AC-A05.
- `cargo xtask lints` — the five-lint story-grain gate `reachability_static`
  wires (`.redkiln/config.yaml:48`); runs unconditionally per story, so a story
  whose whole deliverable is a spec correction still gets checked.
- `redkiln validate --kb && redkiln doctor` — the merge gate for AC-001, AC-006
  (the ADR-0003 lift is a new atom, never an edit — Tension 5) and AC-013 (the
  two open-question atoms resolved in place, `.kb/maps/open-questions-index.md`
  updated). An accepted decision atom is immutable, so this is also the check
  that catches an edit to `0003-opaque-payloads.md`'s body before it becomes a
  merge conflict with the discipline itself.
- `cargo package --list` assertion (part of `cargo xtask ci`, per CLAUDE.md
  *Commands*) — proves AC-015/AC-A10: both sync crates still carry
  `publish = false`, so neither is dragged into the licence-file-and-README
  check that only a publishable crate owes.
- `cargo test -p xtask --doc` and `cargo xtask lint-constitution` — unchanged,
  but relevant the moment this brief or the ADRs cite a `standards/rust/` atom
  with a compiled example.

**Unit — the two crates' own internals, and the mutant registry.** Ordinary
`cargo test -p happenstance-sync -p happenstance-sync-testkit --all-features`.
This is where DR-4 and CF-1–CF-4 are discharged:

- The mutant registry, in the exact `Declared`-table shape at
  `crates/happenstance-testkit/tests/mutation_coverage.rs:140-175` — a
  `fails: &'static [&str]`, a never-empty `provenance`, and per-rule `expect`
  pins. AC-004's requirement that every mutant's provenance name a *real* peer
  shape is what `RS-60-3` (`standards/rust/60-what-a-test-must-prove.md`, "one
  defect is one overridden default method, selected by a `PhantomData` marker")
  and `RS-60-4` ("spell the oracle in a direction that shares no subroutine
  with the implementation") both bear on directly — the mutant peer must not
  reuse `MemorySyncPeer`'s own dispatch to fake its bug.
- AC-008's two headline rules — `ingest_never_rejects` (SY-1) and
  `compensation_is_atomic_with_the_losing_event` (SY-2) — each need a named
  wrong peer in `happenstance-sync-testkit`'s own `tests/`, per CLAUDE.md's
  corollary: *"a rule that no adapter can fail is decorative… name a plausible
  wrong implementation it rejects, and write that implementation into the
  testkit's own `tests/`."* The plausible wrong peer for `ingest_never_rejects`
  is exactly the one SY-1's `Rejects:` line already names — a receiver that
  calls `append(events, Some(&origin_condition))` (`spec/SPECIFICATION.md:5856-5867`)
  — which the architecture brief's §3 already identifies as the mutant
  registry's first entry; this tier is where it becomes a compiled `struct`.
- A `no_orphan_sync_rules` meta-test, the same shape as
  `crates/happenstance-testkit/src/registry.rs:412-434`: scans
  `happenstance-sync-testkit`'s `rules` module against
  `for_each_sync_peer_rule!`'s enumeration in both directions. This is AC-003's
  "no rule is silently absent" made mechanical rather than asserted in prose.
- `identity.rs`'s `(StoreId, SequencePosition)` construction
  (`crates/happenstance-sync/src/identity.rs:87-101`) and `wire.rs`'s
  `FORMAT_VERSION` refusal path (`crates/happenstance-sync/src/wire.rs:46-65`,
  WF-8/WF-9) get direct unit coverage — an unknown format version is refused
  **before** the message is decoded, which is the property `Envelope<T>`'s
  hand-written `Deserialize` exists to buy and the derive path cannot express
  (architecture brief §7).
- Doctests on every new public item, per the repository-wide obligation
  (`standards/rust/70-rustdoc-obligations.md`, `CLAUDE.md` *House style*).
  `happenstance-sync-testkit` is `publish = false` like `happenstance-testkit`
  itself, so its doctests need the same out-of-package harness
  `RS-62-5` names (`standards/rust/62-doctests-and-harnesses.md`) rather than
  the default `cargo test --doc`, which skips a crate that never publishes.
  `MemorySyncPeer` is the doctest target the architecture brief already names
  (§4, first bullet) — it is real today and is where "one round trip with no
  held state" (peer.rs:43-50) gets its first working example.

**Integration — the conformance suite, run three times, two of them live.**
This tier is the proof artefact DoD 4 names and where AC-003, AC-005, AC-006,
AC-007, AC-009 and (operationally) AC-002 actually get proven, not just
declared:

- `MemorySyncPeer` — in-process, `cargo test -p happenstance-sync-testkit
  --features memory-peer` (naming per architecture brief §4's `memory`-feature
  discrepancy to settle; whichever answer lands, this is the fast, always-on
  leg that runs on every `cargo xtask affected` pass touching the sync crates).
  This is the oracle: `RS-60-4`'s "shares no subroutine" applies here too — the
  suite's assertions must not be written by reading `MemorySyncPeer`'s
  implementation and restating it.
- The Durable Object peer (`happenstance-cloudflare`, HS-P0013's build) — a
  `wasm32-unknown-unknown` target, socket-reachable. Runs the suite against a
  real or locally-emulated Worker, per whatever harness HS-P0013 already
  established for that crate; this project adds the sync suite as a second
  thing that harness runs, not a new harness.
- The Neon peer (`happenstance-neon`, HS-P0014's build) — one-shot HTTP, no
  interactive transaction, no cursor. **This leg cannot be gated behind a
  feature flag that silently no-ops**; per DR-8 and the declined-capability
  pattern (`crates/happenstance-testkit/src/contract.rs:368-433`), if the Neon
  test environment is genuinely unavailable in a given run, the suite reports a
  **declined capability with the fixture's stated reason** — never a vanished
  test target. Whether that environment exists yet from HS-P0013/HS-P0014 is
  the infrastructure-confirmation item the architecture brief's *Non-prescriptive
  implementation notes* puts at slice 1: confirm before the peer slices start,
  raise a blocker if it is absent, do not stand in a mock for it.
- **AC-006's round trip** is an integration-tier assertion by construction: it
  needs two real stores and a real hop between them. Assert the payload
  `Bytes` byte-identical at the receiver and assert that replaying the same
  batch twice is an observable no-op (idempotent re-ingest, SY-5/SY-19). DR-5
  is the negative control this tier owes: a suite variant (or a targeted test
  inside `happenstance-sync-testkit`) that would fail if any assertion touched
  `Event::data` or `Event::metadata` — the same shape as `GappedPositionStore`
  proving CF-6 by existing as a conformant variant a naive assertion would trip
  over (`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`).
- **AC-007's two topologies** run as two integration scenarios over the same
  suite: a hub-and-spoke wiring (one peer as hub, at least two spokes) and a
  peer-to-peer wiring, with **one adapter type instantiated in both roles on
  different edges** in at least one scenario — SY-9's `[FROZEN]` claim that
  hub-ness is an edge property, not a type property
  (`spec/SPECIFICATION.md:6090-6106`), is untested until some test literally
  does this.
- **AC-009's `!Send` tripwire** needs a real `tokio::spawn`, not just a type
  check, for the same reason CLAUDE.md binding constraint 3 gives for
  `spawns_from_generic`: a `#[tokio::test]` that never spawns proves nothing
  about `Send` (`RS-62-3`, `standards/rust/62-doctests-and-harnesses.md` —
  "a green `#[tokio::test]` says nothing about `Send`"). The ingest path and
  the runner must be exercised inside a real spawned task with the `!Send`
  peer sitting mid-chain, matching the architecture brief's own warning that
  this is exactly the arrangement that tempts a `SendEventStore` bound.
  `wasm32` coverage for the same path is the static tier's job
  (`cargo xtask wasm`, architecture brief §6 third bullet); this integration
  test is the native-target half of the same claim.

**No new E2E tier.** `.redkiln/config.yaml:57-60` reserves `e2e:
"cargo xtask ci"` for the **terminal** project — this project is not terminal
(`project.md` frontmatter, `terminal: false`), and DoD 1 says so explicitly:
"the whole gate is `closeout-and-durable-audience`'s, not this project's."
`integration_scoped: "cargo xtask ci --fast"` (`.redkiln/config.yaml:55`) is
this project's ceiling, and it already contains the wasm32 steps this project
adds to. The live Durable Object and Neon legs above are this project's
closest analogue to an E2E tier — cross-process, cross-network, real
infrastructure — without being *the* whole-initiative E2E, which stays out of
scope per DoD 1.

### Merge-gate commands

In the order a story actually runs them, story grain first:

```
cargo xtask affected --base main            # what this diff could break (story grain)
cargo xtask lints && cargo xtask spec-trace # reachability_static, unconditional
cargo test -p happenstance-sync -p happenstance-sync-testkit --all-features
cargo xtask ci --fast                       # integration_scoped — this project's ceiling
redkiln validate --kb && redkiln doctor     # KB atoms: AC-001, AC-006, AC-013
```

`cargo xtask ci --fast` already contains: fmt, clippy `-D warnings`, the full
test run, the four-then-six wasm32 steps (architecture brief §6, third
bullet), docs, `spec-trace`, the `--no-default-features` doc build and
`cargo package --list` — see CLAUDE.md *Commands* for the authoritative list
and which steps degrade to `skipped` versus fail when a tool is absent
(`cargo-hack`/`cargo-deny` both resolve on this machine today, so they run for
real). Nothing above substitutes for that command; the list only orders what a
story runs *before* paying for the whole thing.

Per DoD 3 and `.redkiln/config.yaml:62-67` (`require_ledger: true`), every
story's `_ledger.md` cites which of the commands above produced the evidence
for each AC-### it claims — a green `cargo xtask ci --fast` proves the gate
passed, never on its own that AC-006's byte-identity assertion is the one that
ran.

### Fixtures and seams to mock — and the one seam that must not be

| Seam | Treatment | Why |
| --- | --- | --- |
| `MemorySyncPeer` | Real, in-process, the oracle (architecture brief §4). Not a mock. | It is real today and doctest-tested; mocking the oracle would make the suite circular. |
| Two networked peers (Durable Object, Neon) | Real infrastructure, gated as a **declined capability** when unavailable, never as a silent no-op. | DR-6 requires spread across a genuinely unlike shape; a mock of "no interactive transaction" only proves the mock author's assumption about what that constraint permits. |
| The mutant peers (CF-1–CF-4 registry) | Deliberately-wrong, hand-written peer/store shapes in `happenstance-sync-testkit`'s own `tests/` — not mocks of a real dependency, but adversarial implementations of the *port itself*. | This is the one place "fake it" is correct: the registry's job is to be wrong in a named, provenance-carrying way (`mutation_coverage.rs:140-175`). |
| `guard: Option<AppendCondition>` on `EventGroup` | Real field, exercised with both `Some` and `None`, and with a position-relative condition to trip `wire_condition_with_after_is_refused` (SY-6). | It is the live wrong-implementation target Tension 4 names for SY-1/SY-6/SY-12; a suite that never populates it cannot prove the clause. |
| Round-trip counter (peer.rs:43-50's "cannot be expressed by the port" property) | A fixture-owned counter, incremented by the fixture's own `push`/`pull` implementation, asserted by the rule — never inferred from the port's return type. | The architecture brief's *Non-prescriptive implementation notes* states the port cannot express this; the fixture is where the check has to live. |
| Payload bytes (`Event::data`, `Event::metadata`) | Never decoded, never asserted on structurally — only compared as opaque `Bytes` for equality. | DR-5, verbatim: "A suite that parses a payload would certify a peer that does." |

### Acceptance Criteria

Every project AC-### mapped to the tier(s) that prove it. "Tier" abbreviations:
**S**tatic, **U**nit, **I**ntegration.

- **AC-001** — S (`redkiln validate --kb`, the merge-order check that the ADRs
  predate the code) + process (the sequencing itself is enforced by story
  order per the architecture brief's Tension 1, not by a single command).
- **AC-002** — S (`spec-trace` refuses a diff that edits a `[FROZEN]` clause)
  + I (`ingest_never_rejects` and `wire_condition_with_after_is_refused`
  green, with mutants).
- **AC-003** — U (`no_orphan_sync_rules` meta-test) + I (the three-peer run,
  each capability's declined/ran status visible in output).
- **AC-004** — U (the mutant registry, CF-1–CF-4) + S
  (`lint-position-literals` extended to the sync suite, CF-6).
- **AC-005** — I (three peers, two of them the live legs above).
- **AC-006** — I (byte-identity assertion + idempotent-replay assertion) + S
  (`redkiln validate --kb` for the ADR-0003 lift atom).
- **AC-007** — I (the two topology scenarios, one adapter in both roles).
- **AC-008** — U (named wrong peers for the two headline rules) + I (green
  against all three real peers).
- **AC-009** — I (real `tokio::spawn` with the `!Send` peer mid-chain) + S
  (`cargo xtask wasm`, the new sync steps).
- **AC-010** — S (`spec-trace`'s CF-38 check: a `[DEFERRED]` clause with no
  named experiment fails the gate).
- **AC-011** — S (`spec-trace` plus the architecture brief's written
  disposition; no rule discharges ES-39 from this project).
- **AC-012** — S (`spec-trace`'s `Rejects:`-symbol resolution) + process (the
  targeted sweep Tension 4 already ran once and the implementer re-runs).
- **AC-013** — S (`redkiln validate --kb && redkiln doctor`).
- **AC-014** — S (`cargo xtask spec-trace`, the exact mechanism DoD 2 names).
- **AC-015** — S (`cargo package --list` assertion inside `cargo xtask ci`).

### Notes

**What this brief deliberately does not test.** Whether hub-and-spoke and
peer-to-peer are one abstraction or two (SY-10, `[PROVISIONAL]`) is a design
question the architecture brief leaves open; this brief tests whichever shape
lands, twice, per AC-007 — it does not adjudicate the shape. The `SyncRunner`'s
concrete type is likewise undecided; the `!Send`/spawn test above constrains
its *behaviour*, not its name.

**A rule this project must not add.** Per CLAUDE.md's first testkit
corollary, before landing any sync rule beyond the two headline ones (AC-008),
name the plausible wrong peer it rejects and write that peer into
`happenstance-sync-testkit`'s own `tests/` in the same change — a rule that
currently passes for every peer in the tree is decorative until a wrong one is
written down, same as the event-store suite's own history.

**Where this brief's evidence is checked.** Per DoD 3, every story's
`_ledger.md` is the place per-AC evidence is cited; this brief only says which
commands and fixtures can produce that evidence, not the ledger content
itself.
