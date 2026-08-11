# Wave `2026-08-10-intake-2` — placement and adjudication

The ordered action plan. **Thirty-eight operations, one per destination atom**: thirty-six atoms
created and two existing `open_question` atoms amended. Every link is **outbound to an atom
created by an earlier operation**, with one declared exception (Op 6, below), so the Integration
phase can run creates in parallel without a dangling reference; reciprocal backlinks are the
Maps phase's.

**No operation edits an accepted decision atom's body.** None exists to edit, and none created by
this wave is edited by a later operation in it.

---

## Standing choices, applied to every atom in the wave

### 1. One atom per ADR — never fewer, never more

Four extract digests proposed splitting a single ADR across two or three atoms (ADR-0011 into
three, ADR-0015 into three), and `00` shows the opposite pressure too — five groups of ADRs that
score 85–92 on subject identity and could be collapsed. Both are refused, for the same reason.

**A decision atom is identified by its `adr_id` and by the `supersedes` / `superseded_by` pair.**
Three atoms carrying `adr_id: ADR-0011` makes "what does ADR-0020 supersede" unanswerable and
makes the decision map's one-row-per-decision contract impossible. One atom carrying two `adr_id`s
is not expressible at all. The unit the corpus already commits to — a human signed one document —
is the unit the atom must be.

The word budget is met a different way: **the atom is a summary and a pointer, not the ADR.** The
seventeen intake files run 302 to 15,305 words and are byte-identical to files that still exist in
`docs/adr/` (`00`, provenance). `docs/adr/` is not consumed by this ingest and stays canonical. So
each decision atom carries, in 300–900 words: what was decided, the commitments in the imperative
words the ADR used, what lost and why, the provisional or superseded status and its condition, and
`docs/adr/NNNN-*.md` in `source_paths` as the full record. **No atom paraphrases an ADR's reasoning
in place of citing it**, because a paraphrase of a decision is a second decision that nobody signed.

The one split that *is* permitted is **by kind, never by section**: evidence leaves for a
`reference` atom (the decisions README requires it), a durable mechanism leaves for a `concept`
atom, a corpus-operating rule leaves for a `governance` atom, transferable practice leaves for a
`playbook` atom, and an unresolved question leaves for `open-questions/`. Ops 1–4 and 22–38 are all
of that kind, and each says which ADR section it drained.

### 2. `status`, and the two values the schema does not have

`KbFrontmatter.status` is `draft | proposed | accepted | superseded | withdrawn`. The imported
corpus uses two more (`00`, CL-D). The convention applied here, and applied uniformly:

| Source header | `status` | How the lost information is kept |
| --- | --- | --- |
| "accepted" | `accepted` | — |
| "accepted — **provisional**" (0001 lifted, 0003, 0004) or "provisional in named parts" (0011, 0012, 0014, 0015) | `accepted` | The first clause of `summary` names it provisional, names the falsifier, and names the lifting phase. A reader who reads only the summary is not misled. |
| "**superseded** by" (0002) | `superseded` + `superseded_by` | — |
| "**partly superseded** by" (0005, 0006) | **`accepted`** | Prose + `related`. See below. |

**Partial supersession does not use the frontmatter pair, and that is a deliberate call.**
Marking ADR-0006 `superseded` would strip `status: accepted` from the crate-allocation rule that
`CLAUDE.md` enforces today — *everything depends on `happenstance-core`* — because ADR-0007
explicitly does **not** carry that half forward: *"its naming decision is untouched and still
stands, only the runner allocation moves."* A `superseded` atom whose live half is carried by
nothing is worse than an `accepted` atom whose dead half is labelled. So: `supersedes` /
`superseded_by` are used **only for full supersession**; a partial one is stated in the summary's
first clause, in the body, on the decision map, and as a `related` edge. The residual gap is
Op 27's, not silently absorbed.

### 3. `authority_tier`, `phase`, `reversibility`

`authority_tier` follows the layer READMEs exactly: `decision` in `decisions/`, `note` in
`reference/`, `open-questions/` and `concepts/`, `guideline` in `playbooks/` and `governance/`.
Nothing invented.

`phase` is taken from RUNBOOK.md's own ADR ledger (`RUNBOOK.md:283-296`) where the ledger states
it — 0008 → 1, 0009 → 2, 0029 → 2, 0010 → 3, 0011–0015 → 4, 0016 → 5 — and is `0` for the seven
scaffold and rename ADRs dated 2026-08-05/06, which RUNBOOK.md places at "Phase 0 — Ground clear"
(`RUNBOOK.md:713`; ADR-0006 is "executed in phase 0", `:527`). ADR-0001 is `phase: 0` with its
phase-1 lift stated in the body, rather than `phase: 1`, because the ledger's phase-1 row is
ADR-0008's.

`reversibility` is a judgement and is recorded as one. `low` where the ADR freezes a clause or a
signature that publish will lock (0001, 0003, 0008, 0009, 0011, 0012, 0013, 0015); `medium` where
the decision is additive or carries its own collapse condition (0004, 0006, 0007, 0010, 0014,
0029); `high` for the three that were in fact reversed cheaply pre-publish or that the RUNBOOK
itself calls free to reverse (0002, 0005, 0016 — *"the format is private to happenstance, which is
what makes every reversal in it free rather than breaking"*, `RUNBOOK.md:295`).

### 4. `source_paths`

Every atom keeps its originating `.kb/_intake/…` path — the intake files are deleted by a
successful ingest and the atom must still say where it came from — **plus** the surviving
`docs/adr/NNNN-*.md`, plus the repo paths that ground it. All were verified to resolve in this
worktree (`00`, provenance).

### 5. Two new layers

`.kb/concepts/` and `.kb/governance/` do not exist. Both are named in `.kb/README.md`'s suggested
layout and both are kinds `KbFrontmatter` already declares, so neither is an invention — but the
`.kb` README also says a layer ships a README stating its contract, and the previous wave's
placements were deterministic precisely because every layer had one. **The Integration phase must
add `concepts/README.md` and `governance/README.md`**, one screen each, mirroring the voice of
`reference/README.md`: what belongs, what does not, why the layer exists. Op 4 and Op 22 are the
first atoms in them.

### 6. What is *not* extracted

- **The amendment ledgers.** Seven ADRs end with "amendments this decision owes the
  specification" — thirteen items in ADR-0012, eighteen in ADR-0015, nine in ADR-0014. They are
  pointers to `spec/SPECIFICATION.md` edits, most already applied. They stay named-and-counted in
  the decision atom with a `related` edge to `kb-playbook-repair-frozen-clause-001`, whose method
  they are instances of. Copying them into `.kb/` would be the mirror-nobody-maintains the
  reference README bars.
- **ADR-0010's "a `Retires:` line is a claim to re-examine, not a filing."** Considered for a
  playbook and declined: it is inseparable from the five-times-amended narrative of that ADR's own
  §4, and the method it generalises to — build the wrong implementation before believing a rule is
  decorative — is already the decision's own §1. Kept in Op 14, linked to the repair playbook.
- **ADR-0016 §16's three phase-5 follow-ups**, and ADR-0005's "reserve the name". Both are work,
  and the open-questions README bars a task from that layer. Recorded in the decision bodies;
  the crate-name reservation is also carried in `unresolved`, because a follow-up with no backlog
  item is exactly the thing that vanishes.

---

## Ops 1–3 — the evidence

The decisions README is explicit: *"the measurement or compilation a decision rests on is a
`reference` atom that this one cites. Separating them is what lets a decision be superseded
without invalidating the evidence underneath it."* Three reference atoms, created first because
eight later operations link to them.

### Op 1 — what the compiler said about the port traits

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `reference` |
| **destPath** | `.kb/reference/port-traits-compiled-findings.md` |
| **sourceFiles** | `0001`, `0008`, `0009`, `0010`, `0011` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-reference-port-traits-compiled-findings-001
title: What the compiler said about the two port flavours
kind: reference
status: accepted
authority_tier: note
summary: >-
  The register of findings that were established by compiling EventStore and ProjectionStore
  rather than by reasoning about them, gathered from four ADRs written between 2026-08-05 and
  2026-08-08 against trait-variant 0.1.3 on rustc 1.97.1. LocalMemoryEventStore (Rc<RefCell<..>>)
  passes all twenty-seven conformance rules under three harnesses natively and under
  wasm-bindgen-test on wasm32-unknown-unknown, and its direct impl sits beside the blanket
  impl<T: SendEventStore> EventStore for T in a downstream crate with no error[E0119]. RefCell is
  Send and surrenders only Sync — Rc is what does the Send work, so CF-28's wording names the
  wrong type. JsValue is Send + Sync on non-atomics wasm32; Rc in happenstance-cloudflare's error
  is the real hazard, so ES-6's premise is half wrong. #[tokio::test] drives a !Send store because
  it expands to Runtime::block_on, not tokio::spawn. The associated type cannot be varied between
  flavours — five spellings tried, five diagnostics. And dynosaur does not erase this port
  (error[E0277]), against ADR-0001's stated consequence; a hand-written wrapper does, with no
  unsafe.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0001-async-port-flavours.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0009-error-send-sync.md
  - .kb/_intake/0010-the-suite-must-prove-itself.md
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0008-one-derivation-for-both-ports.md
  - docs/adr/0009-error-send-sync.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-testkit/src/fixtures.rs
  - references/evaluation/phase-4-reconciliation.md
last_reviewed: 2026-08-10
```

**Rationale.** The wave's largest merge, five files, anchored at 95 by a finding three documents
state in the same words (`00`, CL-A). Two copies of a measurement is two things to update and one
that quietly goes stale — and here it would have been three.

**Must contain, and must not.** Must: every finding dated, with the toolchain and crate version it
was taken against, and the clause each one corrects (ES-2, ES-3, ES-5, ES-6's premise, CF-23,
CF-28). Must not: the conclusion drawn from any of them. That ES-6's rule is therefore writable is
ADR-0009's; that the derivation scheme covers both ports is ADR-0008's. The reference README bars
the conclusion from this layer precisely so a superseding ADR does not invalidate the measurement.

### Op 2 — the position-visibility experiment

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `reference` |
| **destPath** | `.kb/reference/position-visibility-experiment-2026-08.md` |
| **sourceFiles** | `0013` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-reference-position-visibility-experiment-001
title: The position-visibility experiment — four arms against real PostgreSQL
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-2 measurement that ADR-0013 rests on, run against real PostgreSQL with fsync=on and
  kept in experiments/position-visibility/. Four strategies for buying position visibility when
  nextval() allocates outside the transaction were measured against an inversion detector. Arm C
  (xid8 + pg_snapshot_xmin) is the only one that passes on both writer pairs while leaving writers
  unserialised, at throughput ratios to baseline of 0.987, 0.993, 1.015 and 1.026 at 1, 8, 32 and
  64 clients — the one-client figure from a separate 90-second pass, because the 30-second
  baseline spread was 3.70x. Two positive controls fired: the baseline reproduces the inversion,
  and arm A collapses to 0.062x at 64 clients with p99 60x worse. Arm B-tag, the per-boundary
  lock, measured 0.935 at 64 clients and was rejected on invariant grounds rather than on cost.
  Read-your-own-writes staleness under arm C was 0.688 ms unloaded and 4010.719 ms behind an
  unrelated five-second write in an unrelated database.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - experiments/position-visibility/
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

**Rationale.** Cited by more than the decision that commissioned it: `CLAUDE.md` carries "how a
Postgres adapter buys position visibility" as a standing open question, phase 10 and ADR-0024 own
the adapter that must express arm C, and Op 31 is the open question about whether it can. A
measurement with three future consumers is the reference README's central case.

The staleness figure must survive verbatim, with its conditions. It is the one number that argues
*against* the chosen arm, and an evidence atom that keeps only the favourable half is an argument.

### Op 3 — the wire-format measurements

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `reference` |
| **destPath** | `.kb/reference/wire-format-encoding-measurements.md` |
| **sourceFiles** | `0016` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-reference-wire-format-measurements-001
title: Wire-format encoding measurements, and two instruments that measured wrong
kind: reference
status: accepted
authority_tier: note
summary: >-
  The measurements ADR-0016 rests on, taken 2026-08-09 and kept in experiments/wire-format/,
  outside the workspace and outside the gate, reproducible with cargo test -- --nocapture. A 340
  KiB Turnstile payload encodes to 1,243,464 bytes as a JSON array of raw bytes, 464,218 as
  base64, 696,322 as hex and 348,163 in postcard. skip_serializing_if is worth 26 bytes of JSON
  and one byte per field of postcard, not the figures the clauses claimed. StoreId's JSON size
  ratio is 1.7353x, not 4x. base64 costs zero new crates, already being in the graph via sqlx's
  0.22 pin. Two instruments were themselves measured and found wanting: a postcard round trip of a
  lone Event fails with "Hit the end of buffer" for three of four shapes, so a silent wrong-value
  decode needs a specific neighbour in the buffer; and a compile_fail doctest passes whenever the
  snippet fails to compile for any reason at all — three of four deliberately-broken spellings
  reported green against a false assertion. W7 found the DCB reference implementation publishes no
  wire format: EventStore.ts contains no serialisation code.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - docs/adr/0016-the-wire-format.md
  - experiments/wire-format/
  - crates/happenstance-sync/src/wire.rs
last_reviewed: 2026-08-10
```

**Rationale.** Kept out of Op 20's decision atom for the ordinary reason, and kept as one atom
rather than two because the "instrument that measured wrong" findings are what make the size
numbers trustworthy — the `compile_fail` finding in particular is the reason WF-12's enforcement
changed, and it generalises past this ADR. It is filed as a measurement rather than a playbook
because it is a fact about one tool at one version, not a method: `rustdoc` on stable 1.97.1
silently ignores a mismatched `compile_fail` annotation, and that is a dated statement.

---

## Op 4 — the concept the read-side decisions all assume

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `concept` |
| **destPath** | `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` |
| **sourceFiles** | `0011` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-concept-torn-read-append-boundary-001
title: Why the append condition does not catch a torn read
kind: concept
status: accepted
authority_tier: note
summary: >-
  The obvious objection to spending anything on read isolation is that the append condition
  re-checks at the store, so a stale read is caught on write. It is not, and the reason is that
  the boundary is derived from the read itself: read_decision_model returns the maximum position
  it observed and AppendCondition::after_opt consumes it, so a torn read that missed an event
  below its own observed maximum produces a condition the store evaluates as satisfied. The
  failure is a silently accepted append, not a rejected one — the direction that loses data rather
  than the direction that retries. Two structural reasons stand behind it: a fold across two
  states can be internally inconsistent in a way no condition can express, and a read-only
  consumer never appends at all, so for it the read is the only isolation there is. This is the
  mechanism ADR-0011's sampling instant, ADR-0012's guards and ADR-0013's visibility invariant are
  each protecting; none of them states it, because each assumes it.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0011-read-laziness-and-isolation.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/append.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

**Rationale.** The extract flagged this one for adjudication: fold it into ADR-0011's atom, or open
a layer for it. Opened. It commits nothing, so it is not a decision; it prescribes nothing, so it
is not a playbook; and it is assumed by three separate ADRs, which is the test for whether an
explanation has outgrown the document that happened to state it. Folding it into Op 15 would bury
a shared premise inside one of its three consumers, and Op 15 is already carrying seven decisions.

---

## Ops 5–21 — the ADR corpus

Seventeen `create_new` operations, one per ADR, in ADR-number order so that a supersession target
is always created before the atom that names it. All seventeen share:
`kind: decision`, `authority_tier: decision`, `mapsImpact: decisionMap + domainMap`,
`sourceFiles: [the one intake file]`, and `source_paths` carrying the intake path, the surviving
`docs/adr/` path, and the grounding code.

`classification: extends` on all seventeen — nothing in `.kb/` is contradicted, and the
supersessions among them arrive already resolved by the documents themselves (`01`).

### Op 5 — ADR-0001

```yaml
id: kb-decision-0001
title: Async ports in two flavours, Send and !Send
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0001
reversibility: low
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Each async storage port is defined once with no Send bound, and the Send flavour is derived by
  #[trait_variant::make(SendEventStore: Send)], with SendEventStore implying EventStore through a
  blanket impl so generic code binds the weaker one. read returns its Stream at the top level of
  the return type rather than from an async fn, so the stream itself carries the derived
  Send-ness; two tests hold that shape and it takes both. Authored provisional on 2026-08-05 with
  no !Send implementer in existence; the marker was lifted 2026-08-06 when LocalMemoryEventStore
  passed the suite natively and on wasm32, and ADR-0008 records the lift. Rejected -
  #[async_trait], which injects + Send and kills the Workers target; a !Send-only port, which
  taxes native tokio callers; a sync core with an async wrapper, which a Durable Object's async
  SqlStorage cannot implement; and two hand-written traits per port. The naming departs from
  trait_variant's own LocalEventStore convention because Local already means the on-device store.
  Its stated consequence that dynosaur erases the port is wrong and ADR-0011 corrects it.
depends_on: []
related:
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0001-async-port-flavours.md
  - docs/adr/0001-async-port-flavours.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/memory.rs
  - CLAUDE.md
last_reviewed: 2026-08-10
```

### Op 6 — ADR-0002 (arrives superseded)

```yaml
id: kb-decision-0002
title: Prefixed crate names, with a parallel claim on eventum
kind: decision
status: superseded
authority_tier: decision
adr_id: ADR-0002
reversibility: high
phase: 0
supersedes: null
superseded_by: kb-decision-0005
summary: >-
  Superseded by ADR-0005 and kept for the record. The bare eventum name on crates.io belonged to
  an unrelated crate dormant since 2020, so the decision was to publish under prefixed names
  immediately with eventum-core as the documented entry point, pursue a name-release request in
  parallel, and add a thin facade later if the name ever freed - nothing needing a rename for that
  to happen. Rejected - waiting for the name, which blocks all publishing on an unresponsive
  owner, and renaming the whole project, which spends the repository, organisation and branding to
  solve what a prefix already solves. It was superseded thirty-two minutes after being written,
  when the project was renamed to happenstance and the collision stopped existing. Its body is
  kept verbatim rather than corrected, because rewriting eventum to happenstance inside it would
  turn a true statement about a crates.io registration into a false one.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0002-crate-naming.md
  - docs/adr/0002-crate-naming.md
  - CONTRIBUTING.md
last_reviewed: 2026-08-10
```

**The wave's one forward reference.** `superseded_by: kb-decision-0005` names an atom created by
Op 9. It is authored at create time rather than deferred to the Maps phase because a `superseded`
decision with an empty `superseded_by` is a broken record for as long as it exists, and because
the id is deterministic and lands in the same wave, before validation runs. The reciprocal
(`supersedes: [kb-decision-0002]` on Op 9) is a normal backward reference. This is the only
forward link in the plan and it is declared here so the Integration phase does not treat it as an
error.

### Op 7 — ADR-0003

```yaml
id: kb-decision-0003
title: Opaque payloads in the contract crate
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0003
reversibility: low
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Provisional, and lifts at phase 13 when happenstance-sync round-trips an event between two
  stores without deserialising its payload. Event::data is bytes::Bytes; happenstance-core carries
  no serde in its default features and never parses a payload; an off-by-default serde feature
  adds Serialize and Deserialize for the envelope types only, so replication can put an envelope
  on the wire without making the payload any less opaque. Encoding and decoding - a Codec, a
  DomainEvent mapping - belong to happenstance, the layer above. Consequences - adapters carry no
  domain knowledge, a peer forwards bytes it cannot parse and therefore cannot corrupt, binary
  codecs stay possible, the contract crate's dependency graph and semver surface stay small, and
  Bytes clones are refcount bumps. The cost is that application code cannot pattern-match a domain
  event straight out of the store. Rejected - a generic payload parameter, which infects every
  adapter signature and blocks dyn storage, and serde_json::Value in core, which locks the
  ecosystem to JSON and forces a parse on every pass-through read.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0003-opaque-payloads.md
  - docs/adr/0003-opaque-payloads.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-10
```

### Op 8 — ADR-0004

```yaml
id: kb-decision-0004
title: Rust 2024 edition, MSRV 1.85
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0004
reversibility: medium
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Provisional, and amended rather than superseded by ADR-0029 - the number in this decision is no
  longer the MSRV, and its reasoning is what ADR-0029 acted on. Edition 2024 with rust-version
  1.85 and the toolchain pinned at 1.97.1 in rust-toolchain.toml, which are deliberately two
  different facts: the floor a consumer may build with, and the compiler contributors use. Edition
  2024 is not incidental - its rule that return-position impl Trait captures all in-scope
  lifetimes by default is what lets EventStore::read return a stream borrowing &self with no
  explicit + '_. Let-chains were avoided as post-1.85 (ADR-0029 makes them available). The MSRV is
  verified only in CI, by a job running cargo hack check --no-dev-deps --rust-version on a pinned
  toolchain, and the standing instruction if that job fails is to raise the MSRV rather than work
  around it - which is exactly what happened. happenstance-core is no_std plus alloc under
  --no-default-features. Policy - an MSRV bump is a minor version bump and is called out in the
  changelog; it becomes a promise rather than a preference at first publish, phase 12.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0004-edition-and-msrv.md
  - docs/adr/0004-edition-and-msrv.md
  - rust-toolchain.toml
  - Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-10
```

### Op 9 — ADR-0005

```yaml
id: kb-decision-0005
title: Rename the project to happenstance, and make it the contract crate
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0005
reversibility: high
phase: 0
supersedes:
  - kb-decision-0002
superseded_by: null
summary: >-
  Partly superseded by ADR-0006 - the rename stands, the crate allocation is reversed. Two
  decisions were taken under one "and". First, rename the project from eventum to happenstance,
  whose crates.io endpoint is free and whose GitHub repository was already renamed; that half is
  untouched. Second, because the bare name was now available, allocate it to the contract crate
  rather than to a batteries-included facade, with adapters keeping the happenstance- prefix; that
  half ADR-0006 reverses, on the ground that the argument given for it was unevidenced and
  contradicted by the runbook's own facade plan. Recorded consequences - one fewer crate and no
  dependency on the crates.io team, but the facade is left with no natural name (happenstance-full
  or a later rename), old links redirect through GitHub with no package-level breakage, and a
  per-crate README is owed before first publish. Its outstanding follow-up is that nothing yet
  reserves the name: publishing even a placeholder claims it, and someone else registering it
  first would reopen this decision.
depends_on: []
related:
  - kb-decision-0002
source_paths:
  - .kb/_intake/0005-rename-to-happenstance.md
  - docs/adr/0005-rename-to-happenstance.md
  - CONTRIBUTING.md
  - CLAUDE.md
last_reviewed: 2026-08-10
```

### Op 10 — ADR-0006

```yaml
id: kb-decision-0006
title: The bare name goes to the typed layer; the contract becomes happenstance-core
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0006
reversibility: medium
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Partly superseded by ADR-0007 - the naming decision stands and is the rule in force today; the
  projection-runner relocation is corrected. ADR-0005's allocation is inverted: happenstance-core
  holds the ports, types, errors and the in-memory reference store, carries no serde, is no_std
  plus alloc and is what an adapter author pins; happenstance holds Codec, DomainEvent,
  DecisionModel and the command loop, re-exports the contract, feature-gates the adapters, and is
  what an application cargo adds. happenstance-runtime ceases to exist. The workspace rule that
  follows is that everything depends on happenstance-core and happenstance-core depends on nothing
  in this workspace. The serde boundary moves with the contract rather than with the name: the
  contract crate's documentation must say so in its first paragraph and happenstance must point at
  it. Grounded in who imports what, and in serde_core, futures-core and tracing-core all giving
  the bare name to what applications import. Rejected - renaming the typed layer to
  happenstance-domain, deferring the allocation, and ADR-0005's own happenstance-full. Not marked
  provisional, because a naming decision is settled by being made rather than by being tested.
depends_on:
  - kb-decision-0005
related:
  - kb-decision-0003
source_paths:
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - docs/adr/0006-bare-name-to-the-typed-layer.md
  - CLAUDE.md
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

### Op 11 — ADR-0007

```yaml
id: kb-decision-0007
title: The projection runner decodes, and therefore splits across the seam
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0007
reversibility: medium
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Partly supersedes ADR-0006's runner allocation while leaving its naming decision untouched and
  standing. The runner splits at the decode boundary: happenstance-core holds the checkpoint pump,
  which reads from a checkpoint, chunks, begins a batch, invokes a callback per SequencedEvent and
  commits with the last position applied, and never decodes; happenstance holds the Projection
  trait and the runner an application actually uses, with decoded events, the projection's Query
  and the store's Batch. ADR-0006's discriminator - encoding, not orchestration - survives intact;
  what fails is its claim that the whole runner never decodes, asserted before any runner existed.
  Three shape decisions travel with it: a projection nominates events with Query, the same type a
  decision model uses, so there is no second filtering vocabulary; Projection::Store is an
  associated type, so a projection spanning two stores is unrepresentable; and checkpoints stay
  per store and projection id, so two stores projecting the same events sit at different positions
  and callers reading both must tolerate the skew. Falsifier - if the core pump has acquired no
  caller but the typed one when phase 7 exits, collapse it upward and supersede this decision.
depends_on:
  - kb-decision-0006
related: []
source_paths:
  - .kb/_intake/0007-projection-runner-decodes.md
  - docs/adr/0007-projection-runner-decodes.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

### Op 12 — ADR-0008

```yaml
id: kb-decision-0008
title: One derivation scheme, both ports, and what a provided body owes
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0008
reversibility: low
phase: 1
supersedes: null
superseded_by: null
summary: >-
  ProjectionStore had copied EventStore's #[trait_variant::make] construction with no decision of
  its own, and PS-35 requires one document covering both ports. It gets one: a single derivation
  scheme serves both, and three rules bind any provided method on either. A provided body is
  hand-desugared to -> impl Future and never written as async fn. A body holding &self across an
  await takes where Self: Sync at the point of use and never in the attribute, because the
  attribute's whole bound list reaches read's stream and would reject any Cell-hiding adapter. And
  one body must compile against both the bare and the derived flavour, checked at the port. The
  ports diverge in the consequences: ProjectionStore's Batch GAT means a provided body cannot hold
  a batch across a suspension point under any remedy tried, and EventStore structurally cannot
  exhibit that. This decision also lifts ADR-0001's provisional marker, with full proof still owed
  by the phase-9 Durable Object adapter. Rejected - two hand-written traits per port, a different
  scheme per port, and Sync in the attribute. Four amendments are owed to the specification and
  none changes a normative MUST.
depends_on:
  - kb-decision-0001
related:
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - docs/adr/0008-one-derivation-for-both-ports.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 13 — ADR-0009

```yaml
id: kb-decision-0009
title: Error stays unbounded, and the strength goes in a marker
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0009
reversibility: low
phase: 2
supersedes: null
superseded_by: null
summary: >-
  Error keeps exactly core::error::Error + 'static on both ports and both flavours, and the
  stronger property moves downstream into a blanket-implemented marker trait -
  ThreadSafeEventStore over SendEventStore<Error: Send + Sync> - that generic code opts into. Four
  compiled findings decide it: a workspace-wide check with + Send + Sync fails only
  happenstance-cloudflare's Rc<str> error; ES-6's premise about JsValue is half wrong, since
  JsValue is Send + Sync on non-atomics wasm32 and Rc is the hazard; the conflict signal never
  travels in Self::Error because the contract lifts it to AppendError::ConditionViolated; and the
  derived Send flavour does not imply a Send error, so ES-6's rule could not have been written
  against today's port for any adapter. Rejected - widening Error directly, which breaks the
  wasm32 target the two-flavour design exists for; leaving ES-6 deferred, which is abandonment
  behind an unwritable rule; a second SendError associated type, which trait_variant would copy
  into the !Send flavour too; and shipping the marker in happenstance-core now. The same answer
  covers ProjectionStore and PS-35, and nothing about the port changes.
depends_on:
  - kb-decision-0008
related:
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0009-error-send-sync.md
  - docs/adr/0009-error-send-sync.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/error.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 14 — ADR-0010

```yaml
id: kb-decision-0010
title: The conformance suite's own proof obligation
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0010
reversibility: medium
phase: 3
supersedes: null
superseded_by: null
summary: >-
  Twenty-seven conformance rules had never been shown to reject anything, and four plausible wrong
  implementations passed the suite as written. So a rule may not be added until a store exists
  that fails it: every rule owes a mutant with stated provenance, naming the real implementation
  mistake it models, and three meta-tests enforce that - every rule has a mutant, each mutant
  fails exactly its declared rules and passes every other, and conformant variants pass
  everything. No pass rate is ever quoted over the mutant set. The fixture contract changes from a
  fresh empty store to a factory that can be asked more than once for a handle onto the same
  backing store, and a capability an adapter declines is skipped with its stated reason rather
  than vanishing - a skip must be reported, never silent. The runtime wrapper stays a parameter,
  ratified from phase 1. Its fourth section was amended five times in place: three rules were
  proposed for retirement and none was retired, and MID_BATCH_FAULT was minted because a fault
  must be injected by the fixture rather than by a decorator. Rejected - cargo-mutants over the
  reference store, a mechanical mutant per rule, and rejection without exactness.
depends_on: []
related:
  - kb-reference-port-traits-compiled-findings-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0010-the-suite-must-prove-itself.md
  - docs/adr/0010-the-suite-must-prove-itself.md
  - crates/happenstance-testkit/src/fixtures.rs
  - crates/happenstance-testkit/src/registry.rs
  - crates/happenstance-testkit/src/suite.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 15 — ADR-0011

```yaml
id: kb-decision-0011
title: A read is one sample with a ceiling, and &Query stays
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0011
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  Seven dispositions on the read side, two clauses left provisional and two clauses minted. read
  keeps query: &Query - the opaque return type captures the query's lifetime whether or not one is
  named, so an escaping stream owns its query as a parameter rather than as a local, and ES-13's
  own stated remedy is wrong on that point. The port's promise is corrected from lazy to evaluated
  against one state sampled no later than the first poll: laziness is permitted and never
  required, building a stream is not guaranteed free, and a caller must not depend on whether
  events appended between the call and the first poll appear. An adapter issuing more than one
  statement per read must capture a position ceiling no later than the first poll and bound every
  later statement by it, which discharges both the one-snapshot and the shared-snapshot clauses
  across all three store shapes. read gains no + Unpin bound, minted provisional with a hard
  phase-12 deadline because the bound is unaddable after publish. ReadOptions gains an inclusive
  to and a limit of Option<usize> so limit(0) yields nothing, and is frozen with exactly one lower
  bound: no exclusive after, ever. No extension trait and no prelude at 0.1.
depends_on:
  - kb-decision-0008
  - kb-decision-0010
related:
  - kb-decision-0001
  - kb-concept-torn-read-append-boundary-001
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0011-read-laziness-and-isolation.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/query.rs
  - references/evaluation/phase-4-reconciliation.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 16 — ADR-0012

```yaml
id: kb-decision-0012
title: append keeps its borrowed batch, and phase 4 declines what it cannot measure
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0012
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  append keeps events: &[Event] and the lift of that clause to frozen is declined this phase,
  because the question is a measurement phase 8 owns; the ADR states the five things that
  measurement must produce, and names Vec<Event> as the only surviving successor if it fires. An
  empty batch is refused before any condition is evaluated. A condition is evaluated only against
  events the store already held when append began, never against the batch being appended.
  Dropping the future does not cancel the append: an adapter may have committed before or after
  the drop, a caller must not treat a drop as evidence the append did not land, and the store is
  left fully applied or unchanged. A verbatim reissue of a conditional, self-matching batch lands
  at most once; the other two shapes must duplicate, which makes a payload-deduplicating store
  non-conformant on purpose. append returns the last position in the batch, assigned in slice
  order and strictly ascending but not necessarily contiguous. conflicting_position is a hint that
  may be absent. AppendCondition becomes a non-empty sequence of guards with a private field and
  an accessor - a public field let downstream code write an empty guard list and get a silently
  unconditional append. CF-39 is minted for mid-batch fault injection.
depends_on:
  - kb-decision-0010
related:
  - kb-concept-torn-read-append-boundary-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0012-append-shape-and-preconditions.md
  - docs/adr/0012-append-shape-and-preconditions.md
  - crates/happenstance-core/src/append.rs
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-testkit/src/fixtures.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 17 — ADR-0013

```yaml
id: kb-decision-0013
title: Positions are assigned once and become visible in order
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0013
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  The visibility invariant lifts from provisional to frozen and is stated once: after any reader
  has observed an event at position P, no later read against that store may yield an event at a
  position at or below P that was not already visible, and an adapter must not make an event
  visible below one it has already exposed. The lift is what moves the position-allocation axis
  into the acceptance ledger, and the freeze names the four axes it is accepting on and the three
  it is not. Three caveats travel with it. The invariant stays global rather than per consistency
  boundary - the cheaper per-boundary lock would keep the append condition sound and make the
  projection checkpoint unsound, so it is rejected on grounds rather than on cost, at a measured
  9 percent. head() reports the visibility frontier and not the maximum assigned position. And
  SequencePosition::next uses NonZeroU64::checked_add so that overflow signals None, with the
  Option free in the niche; that change is in the tree at event.rs:278, verified 2026-08-10. The
  contract documents checkpoint.next() as the sound resume idiom over a store with gaps, because
  ReadOptions::from is a threshold predicate and not an equality seek.
depends_on:
  - kb-decision-0010
related:
  - kb-decision-0011
  - kb-reference-position-visibility-experiment-001
  - kb-concept-torn-read-append-boundary-001
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/src/store.rs
  - experiments/position-visibility/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 18 — ADR-0014

```yaml
id: kb-decision-0014
title: The store mints identity, records a time, and the caller supplies neither
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0014
reversibility: medium
phase: 4
supersedes: null
superseded_by: null
summary: >-
  Provisional in four named parts, each with a falsifier and an owning phase, none reachable by
  phase 4. Three types land in happenstance-core and replace the phase-2 placeholders in
  happenstance-sync: StoreId as sixteen bytes, EventId as a store and a position with private
  fields and accessors, and RecordedAt as a newtype over a signed i64 of milliseconds. Position,
  event id and recorded time are all assigned by the store at append; the store id is assigned by
  the deployment. An adapter may mint a store id once and re-mint only if it can detect a restore
  or clone, or if the deployment documents when re-minting is invoked; otherwise it must mint a
  fresh incarnation on every open, and must record which mechanism it uses in the adapter-shapes
  reference. i64 rather than u64 is deliberate and against the placeholder; core ships no
  RecordedAt::now(), and no rule may compare two recorded times or compare one against a position.
  A new required port method, contains_event_id, ships provisional. SequencedEvent::new is
  superseded rather than widened, with no deprecated arm, and Event::into_parts returns a
  non-exhaustive struct instead of a positional tuple. append's signature is unchanged.
depends_on:
  - kb-decision-0008
  - kb-decision-0009
related:
  - kb-decision-0012
  - kb-decision-0013
source_paths:
  - .kb/_intake/0014-event-identity-and-recorded-time.md
  - docs/adr/0014-event-identity-and-recorded-time.md
  - crates/happenstance-core/src/identity.rs
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-sync/src/identity.rs
  - references/adapter-shapes.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 19 — ADR-0015

```yaml
id: kb-decision-0015
title: Validated identifiers, byte equality, and the two kinds of bound
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0015
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  One const fn walks the bytes and validates both EventType::new and EventType::from_static, so
  there is one rule and not two; an exhaustive check over all 1,112,064 scalar values shows the
  byte walk sees the C1 range and the seven bidirectional controls that an ASCII-only narrowing
  would have missed. Both types move from Box<str> to Cow<'static, str> and gain a const
  from_static; Eq, Hash and Ord are hand-written so Borrow<str>'s consistency obligation stays
  explicit. Equality is byte equality over UTF-8 with no normalisation, case folding or trimming,
  documented with two worked failures. The load-bearing distinction is between two kinds of bound:
  a validity invariant is enforced by the constructor and re-enforced on deserialisation, because
  a violating value must be unrepresentable, while a capacity limit is enforced only at the store
  boundary and must not be enforced by a constructor or in Deserialize - rejecting at decode
  destroys the quarantine path a peer needs, since an event refused at decode has no local
  position, cannot be named and cannot be forwarded. Four minimum floors become public constants,
  never ceilings; AppendError gains ExceedsStoreLimit; CF-40 is minted for a fixture's numeric
  limits.
depends_on:
  - kb-decision-0003
related:
  - kb-decision-0012
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0015-validated-identifiers-and-store-limits.md
  - docs/adr/0015-validated-identifiers-and-store-limits.md
  - crates/happenstance-core/src/tag.rs
  - crates/happenstance-core/src/validate.rs
  - crates/happenstance-core/src/limits.rs
  - crates/happenstance-core/src/error.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 20 — ADR-0016

```yaml
id: kb-decision-0016
title: The wire format is happenstance's own, and an unknown version is refused before the message is read
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0016
reversibility: high
phase: 5
supersedes: null
superseded_by: null
summary: >-
  The format is private to happenstance with no compatibility obligation to the DCB reference,
  which publishes no wire format at all - and that is what makes every reversal in it free rather
  than breaking. Twelve clauses are discharged, several by amending the clause rather than by
  satisfying its letter, and two frozen clauses are amended by this new decision rather than by an
  edit. Presence obligations bind the encoder only; the decoder stays asymmetric on optional
  fields and fails closed. Query becomes an externally tagged two-variant enum. Two frozen field
  names are corrected to track the types the code had already moved to. happenstance-sync gains a
  minimal generic envelope whose Deserialize is hand-written, because serde's derive reads every
  field into a local before constructing and offers no hook to refuse an unknown format version
  before the message is decoded - a derived envelope plus a post-hoc check is indistinguishable
  from the correct one except by counting how many times the inner Deserialize ran. base64 is
  added as an optional dependency inside the serde feature at zero new crates. ReadOptions loses
  Serialize and Deserialize, enforced by a const assertion rather than by a compile_fail doctest,
  which was measured to be decorative.
depends_on:
  - kb-decision-0003
  - kb-decision-0012
related:
  - kb-decision-0014
  - kb-reference-wire-format-measurements-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - docs/adr/0016-the-wire-format.md
  - crates/happenstance-sync/src/wire.rs
  - crates/happenstance-core/src/event.rs
  - experiments/wire-format/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 21 — ADR-0029

```yaml
id: kb-decision-0029
title: The MSRV is 1.97.1
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0029
reversibility: medium
phase: 2
supersedes: null
superseded_by: null
summary: >-
  The MSRV is raised from 1.85 to 1.97.1 and rusqlite stays at 0.40. This amends ADR-0004 rather
  than superseding it - that decision's body stays verbatim, because its reasoning is what this
  one acted on: it is ADR-0004 that says the floor is a preference until first publish and invites
  the trade. rust-version and rust-toolchain.toml remain two different facts and are not collapsed
  into one. The forcing evidence is that libsqlite3-sys 0.38.1's build script uses cfg_select!,
  and that it, rusqlite, sqlx, sqlx-core and sqlx-postgres declare no rust-version at all, so
  neither cargo hack --rust-version nor resolver = "3" could see the break coming; only running
  the compiler found it. Let-chains, stable since 1.88, are now available, and CLAUDE.md's
  constraint forbidding them is rewritten. The msrv CI job is kept with a comment although it is
  now vacuous, because it runs the same compiler the gate runs and will stop being vacuous the day
  the pin and the floor diverge. Phase 12 must revisit the floor at first publish, when it stops
  being a preference and becomes a promise. Rejected - pinning rusqlite back to 0.37, a
  per-package rust-version, and raising only as far as cfg_select! requires.
depends_on:
  - kb-decision-0004
related: []
source_paths:
  - .kb/_intake/0029-msrv-raised-to-1-97-1.md
  - docs/adr/0029-msrv-raised-to-1-97-1.md
  - rust-toolchain.toml
  - Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-10
```

---

## Ops 22–24 — what the ADR chain taught, extracted by kind

### Op 22 — rewrite the referent, never the reasoning

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `governance` |
| **destPath** | `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **sourceFiles** | `0005`, `0006`, `0007` |
| **classification** | `aligns` |
| **mapsImpact** | domainMap |

```yaml
id: kb-governance-referent-not-reasoning-001
title: Rewrite the referent, never the reasoning
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  How this corpus edits a decision record that is already signed, refined by a chain that had to
  apply it to itself three times in two days. The immutability rule says supersede rather than
  edit; this is the discrimination it needs in practice. A rename that preserves meaning may be
  rewritten in place - ADR-0001, ADR-0003, ADR-0004 and ADR-0007 had their crate names rewritten
  to happenstance-core at phase 0 rather than left as period spelling under a note, because
  renaming an identifier is not reversing a decision. Reasoning inside a decision that still
  stands is never touched. And a superseded decision's body is left factually intact even where it
  is now wrong about the world: ADR-0002 keeps every mention of eventum, because rewriting them to
  happenstance would convert a true claim about a crates.io registration into a false one, and the
  value of a superseded record is that it says what was true when the choice was made. The test is
  the same shape as the repair-versus-amendment test one layer down: ask whether the edit changes
  what the document asserts, not whether it changes the document.
depends_on: []
related:
  - kb-decision-0002
  - kb-decision-0005
  - kb-decision-0006
  - kb-decision-0007
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0005-rename-to-happenstance.md
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - .kb/_intake/0007-projection-runner-decodes.md
  - docs/adr/0002-crate-naming.md
  - docs/adr/0006-bare-name-to-the-typed-layer.md
  - CONTRIBUTING.md
  - .kb/decisions/README.md
last_reviewed: 2026-08-10
```

**Rationale.** `governance`, not `playbook`: it is a rule about how the decision record itself is
maintained, it is enforced mechanically (`redkiln validate --kb` checks an accepted decision's
body against `HEAD`), and reversing it would require a decision. Three files carry it at a score
of 86 (`00`, CL-B) — merged, not triplicated.

`aligns`, not `extends`: `.kb/decisions/README.md` already states the immutability rule and the
repair-versus-amendment test. This atom does not introduce them; it adds the referent-versus-
reasoning discrimination the README has no room for, and the three worked instances, and it cites
the README rather than restating it. **The README is not edited** — the previous wave established
that a layer README is a contract and not a merge target, and that holds.

### Op 23 — one decision per ADR title

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `playbook` |
| **destPath** | `.kb/playbooks/one-decision-per-adr-title.md` |
| **sourceFiles** | `0005`, `0006`, `0007` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-playbook-one-decision-per-adr-title-001
title: An "and" in a decision's title is usually a second, weaker decision
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  Three times in two days, a title joining two claims with "and" turned out to be carrying a
  strong decision and a weak one, and the weak one was reversed within days while the strong one
  stood. ADR-0005 bundled a rename with a crate allocation; ADR-0006 bundled the corrected
  allocation with the projection runner's home; ADR-0007 named the pattern outright. The
  discriminator is not the conjunction itself but what each half rests on: the halves that stood
  were settled by being made, and the halves that fell were claims about code that did not exist
  yet - a decision about where code lives, taken before that code exists, is a guess. So: when a
  title needs an "and", either split the document, or mark the weaker half provisional with the
  observation that would refute it and the phase that would produce it. That is what the corpus
  did each time, and it is why each reversal cost a paragraph rather than an argument. It stops
  being worth the split when both halves are settled by the same evidence, in which case the
  conjunction is describing one decision with two consequences.
depends_on: []
related:
  - kb-decision-0005
  - kb-decision-0006
  - kb-decision-0007
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0005-rename-to-happenstance.md
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - .kb/_intake/0007-projection-runner-decodes.md
  - docs/adr/0007-projection-runner-decodes.md
last_reviewed: 2026-08-10
```

**Rationale.** `playbook` rather than `governance`, on the playbooks README's own test: reversing
it needs no decision, it prescribes an authoring practice rather than binding the record, and it
carries a stated condition under which it stops holding. Kept separate from Op 22 at a score of
45 — the two are about different acts (how to scope a decision; how to correct one), and merging
them would put a method and a rule in one file with two claim-sets.

### Op 24 — hand-polling cold futures

| | |
| --- | --- |
| **op** | `create_new` |
| **kind** | `playbook` |
| **destPath** | `.kb/playbooks/testing-interleavings-with-cold-futures.md` |
| **sourceFiles** | `0013` |
| **classification** | `extends` |
| **mapsImpact** | domainMap |

```yaml
id: kb-playbook-cold-future-hand-polling-001
title: Testing an interleaving by hand-polling two cold futures
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  How to make a conformance rule observe a specific interleaving with no executor, no second
  thread and no clock. Build two append futures from one handle, pin them in place, and poll them
  out of order - A, B, B, A - to model a slow transaction that publishes late. It works because a
  Rust future is cold until polled, which is the property a reader coming from C# or JavaScript
  will not expect: a Task is already running when you hold it, and a future is not, so the test
  owns the schedule rather than racing it. That keeps the rule inside two constraints the fixture
  contract imposes - no Send bound, and no wall clock. Its known blind spot is that the window
  varies with the adapter's poll shape, so an adapter whose append needs three or more polls may
  slip through. The direct repair, a poll-budget capability on the fixture, was considered and
  rejected at this phase: it would be the first non-boolean capability, it sits close to the
  no-clock line, and calibrating it needs a real I/O-bound adapter that does not exist yet.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0010
  - kb-reference-position-visibility-experiment-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/fixtures.rs
last_reviewed: 2026-08-10
```

**Rationale.** A technique arrived at by discarding a cheaper alternative, with the alternative
named and priced — the playbooks README's third case, exactly. Its blind spot is Op 32's open
question, and the two are linked so a reader of either finds the other.

---

## Ops 25–26 — the two existing atoms this wave amends

Both are `open_question` atoms, so amending them is permitted; neither is a decision. Both were
left by the previous wave pointing at ADRs it declined to import, and both say so in their own
bodies. The amendment is links plus a paragraph — **no existing sentence is deleted**, because an
open question's value is that it records the state of knowledge on the day it was filed.

### Op 25 — ES-6 is writable now, and still unwritten

| | |
| --- | --- |
| **op** | `merge_existing` |
| **kind** | `open_question` |
| **destPath / mergeTargetPath** | `.kb/open-questions/es-6-names-an-unwritable-rule.md` |
| **sourceFiles** | `0008`, `0009` |
| **classification** | `extends` |
| **mapsImpact** | openQuestionIndex |

```yaml
id: kb-open-question-es-6-unwritable-rule-001
title: ES-6 is frozen and names a rule that cannot be written
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-6 is FROZEN and names store_error_crosses_a_join_handle as its rule, marked (new) and
  rendered with † where the legend reads "does not exist yet". The identifier occurs as no fn
  anywhere in the workspace — only in prose and in comments, one of which states the rule is
  unwritable against today's port for every adapter, with SendStoreWithLocalError as the probe.
  spec-trace's check 4 deliberately skips clauses whose rule is (new) or †, and that escape hatch
  has no expiry, so a rule scheduled forever is indistinguishable from one scheduled for next
  week. ADR-0009 is accepted and makes the rule writable, so this may be a scheduling gap rather
  than a design gap — but the clause is frozen and names an unwritten rule. Settled by writing the
  rule against ADR-0009's marker, or by deciding that a † with no owning phase is a hard failure.
  Found independently the same day by references/evaluation/review-citation-drift.md §2. ADR-0008
  and ADR-0009 are now imported as decision atoms: ADR-0008 adds that the rule, when written, must
  assert on the future's Output and not on the future, since a future-only check is decorative
  against a !Send error, and ADR-0009 supplies the ThreadSafeEventStore marker the rule's bound
  would name.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-decision-0008
  - kb-decision-0009
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0009-error-send-sync.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/src/send_shape.rs
  - references/evaluation/review-citation-drift.md
  - docs/adr/0008-one-derivation-for-both-ports.md
  - docs/adr/0009-error-send-sync.md
last_reviewed: 2026-08-10
```

**Must change in the body, and must not.** Must: the sentence *"ADR-0009 is named here by id only;
it is not imported as a decision atom in this wave"* is now false and is the one line that has to
move — replace it with the link, and say which wave imported it. Must add: ADR-0008's constraint
on the unwritten rule, which is the most actionable thing anyone has said about it. Must not:
close the question. The rule is writable and still unwritten, and nothing this wave imported
writes it.

### Op 26 — the two provisional falsifiers, with owners

| | |
| --- | --- |
| **op** | `merge_existing` |
| **kind** | `open_question` |
| **destPath / mergeTargetPath** | `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` |
| **sourceFiles** | `0001`, `0008`, `0014` |
| **classification** | `extends` |
| **mapsImpact** | openQuestionIndex |

```yaml
id: kb-open-question-provisional-falsifiers-001
title: Two provisional markers whose falsifiers can no longer falsify
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-7 and VT-9 are PROVISIONAL and each names a falsifier that no longer discriminates. ES-7's
  falsifier is error[E0119] on a downstream direct impl, and its own named instrument —
  LocalMemoryEventStore in happenstance-testkit — compiles natively and on wasm32 with no such
  error. VT-9's falsifier is a target that cannot supply a wall clock at append time, and one is
  already in the build: on wasm32-unknown-unknown there is no clock and every event is stamped
  from_millis(0), yet VT-9's MUST is satisfied there because its rules assert presence and
  stability of a recorded time and never recency. Moving a maturity marker is an ADR's act, so
  both are recorded rather than moved. The transferable observation, worth keeping however these
  are settled: a falsifier that has already occurred without changing anything is a marker that
  has quietly become decoration, and spec-trace cannot detect it — it sees that a marker exists,
  not whether its condition has been met. Both markers now have named owners in the imported ADR
  corpus: ES-7's evidence is ADR-0001's lift condition, discharged by ADR-0008 and compiled in the
  port-traits findings, and VT-9 is one of ADR-0014's four provisional parts, owned by phase 9's
  Workers skeleton.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-decision-0001
  - kb-decision-0008
  - kb-decision-0014
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/0001-async-port-flavours.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0014-event-identity-and-recorded-time.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/tests/local_conformance.rs
  - crates/happenstance-core/src/memory.rs
  - docs/adr/0001-async-port-flavours.md
  - docs/adr/0014-event-identity-and-recorded-time.md
last_reviewed: 2026-08-10
```

**Must not** be confused with Op 27, and the body must say so in a sentence: this atom is about
`[PROVISIONAL]` markers on *specification clauses*; Op 27 is about the *frontmatter status of an
imported ADR*. Same word, two different objects, and a reader who conflates them will close the
wrong one.

---

## Ops 27–38 — the twelve open questions

All twelve: `op: defer_open_question`, `kind: open_question`, `status: accepted`,
`authority_tier: note`, `mapsImpact: domainMap + openQuestionIndex`, `depends_on: []`.

`status: accepted` follows the convention the previous wave set and stated: it is the live state
of a question, not a claim it has been answered, and the open-questions README's resolution path
is `accepted → withdrawn | superseded`.

Each body follows the open-questions README's four-part shape — what is true today, what is not
decided, what forces it, ordered sub-questions — and each carries **its owning phase and its named
refuter in the body**, because `phase` is a decision-only frontmatter key and inventing it here
would be inventing a key on an atom I am authoring.

Eleven of the twelve are the ADRs' own "what this decision leaves open" sections, which is why
each has an owner and a refuter already: they were written by the person who declined to settle
them. The twelfth (Op 27) is this wave's own, and is the only one whose subject is the KB rather
than the library.

### Op 27 — the status vocabulary the schema cannot express

`classification: conflicts` · `destPath: .kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md`
· `sourceFiles: 0001, 0003, 0004, 0005, 0006, 0011, 0012, 0014, 0015`

```yaml
id: kb-open-question-adr-status-vocabulary-001
title: An accepted-but-provisional ADR has no status the KB can express
kind: open_question
status: accepted
authority_tier: note
summary: >-
  KbFrontmatter's status is draft, proposed, accepted, superseded or withdrawn. The imported ADR
  corpus uses two values it does not have, and both are load-bearing. Nine of the seventeen ADRs
  are "accepted — provisional", either wholly (ADR-0001 before its lift, ADR-0003, ADR-0004) or in
  named parts (ADR-0011, ADR-0012, ADR-0014, ADR-0015), each with a stated falsifier and a lifting
  phase; ADR-0003 says in terms that work contradicting a provisional decision still needs a
  superseding ADR but "does not owe deference to a decision the code has not yet voted on", which
  is a weaker authority than accepted and is lost by mapping to it. Two more are "partly
  superseded" — ADR-0005 and ADR-0006 — where marking the atom superseded would strip status from
  a half that still binds, and marking it accepted hides that a half does not. The 2026-08-10
  import applied a stated convention rather than inventing keys: status accepted, with the
  qualification and its falsifier in the first clause of the summary, and the supersedes pair used
  only for full supersession. What is not decided is whether that is the corpus's answer or a
  stopgap.
depends_on: []
related:
  - kb-open-question-provisional-falsifiers-001
  - kb-decision-0003
  - kb-decision-0005
  - kb-decision-0006
source_paths:
  - .kb/_intake/0003-opaque-payloads.md
  - .kb/_intake/0004-edition-and-msrv.md
  - .kb/_intake/0005-rename-to-happenstance.md
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - .kb/_intake/0014-event-identity-and-recorded-time.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2/02-placement-and-adjudication.md
  - .kb/README.md
  - .kb/decisions/README.md
last_reviewed: 2026-08-10
```

**Ordered sub-questions the body must carry.** (1) Is "provisional" a `status`, an
`authority_tier`, or a passthrough key the corpus owns and redkiln ignores — noting that the
open-questions README explicitly blesses passthrough keys on an imported corpus and equally
explicitly forbids inventing one on an atom being authored, so the answer differs by who is
writing. (2) Does a partly-superseded decision need a third relation beside `supersedes`, or is
prose enough given that the decision map will carry it anyway. (3) Whether the falsifier and
lifting phase should be machine-readable at all, given that the same problem one layer down —
markers whose falsifiers have quietly stopped discriminating — is already `kb-open-question-provisional-falsifiers-001`.

### Op 28 — the projection store's missing apply seam

`classification: requires-new-decision` · `destPath: .kb/open-questions/projection-store-batch-has-no-apply-seam.md`
· `sourceFiles: 0007`

```yaml
id: kb-open-question-projection-batch-no-apply-001
title: ProjectionStore::Batch carries no bounds, so nothing can write to it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ProjectionStore::Batch is an associated type with no trait bounds, so generic code can begin a
  batch and commit it and cannot write anything into it — the port has no apply. ADR-0007 records
  this and declines to settle it, and it is the reason ADR-0006's relocation of the projection
  runner went unchallenged for a day: nothing had tried to compile against the port. What is not
  decided is what vocabulary writes into a batch — a bound on the associated type, a second
  associated type, or a method on the port — and the answer is entangled with ADR-0008's finding
  that a provided body cannot hold the Batch GAT across a suspension point under any remedy tried.
  Owned by phase 6, which freezes ProjectionStore and carries ADR-0017 for what a projection batch
  owns and what vocabulary writes into it. Refuted, or rather forced, by the first real projection
  adapter: the port has no conformance suite, and a port without one is a guess.
depends_on: []
related:
  - kb-decision-0007
  - kb-decision-0008
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
source_paths:
  - .kb/_intake/0007-projection-runner-decodes.md
  - docs/adr/0007-projection-runner-decodes.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

### Op 29 — the union rule nobody owns

`classification: requires-new-decision` · `destPath: .kb/open-questions/query-union-rule-is-owed-and-unowned.md`
· `sourceFiles: 0011`

```yaml
id: kb-open-question-query-union-rule-unowned-001
title: query_union_is_item_concatenation is owed by a clause and owned by no one
kind: open_question
status: accepted
authority_tier: note
summary: >-
  VT-31's residue is a conformance rule rather than a signature or a semantics question:
  query_union_is_item_concatenation, named at ES-15's Rejects line as the thing that would catch
  an item-sorting adapter, is owed and unwritten. ADR-0011 declines it explicitly, on the ground
  that no ADR in the phase-4 queue is the right instrument for a rule — "it belongs to whoever
  writes rules next" — and flags this as the one disposition in that document a human should
  confirm rather than inherit, because declining is cheap and losing the rule is not. What is not
  decided is who writes it and against which wrong implementation, given ADR-0010's standing
  requirement that a rule may not be added until a store exists that fails it. Forced by the next
  pass that writes conformance rules, and by phase 4's close, after which an unclaimed rule has no
  obvious reader. This is the open question in the wave that most wants a human's yes rather than
  a later wave's inference.
depends_on: []
related:
  - kb-decision-0011
  - kb-decision-0010
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0011-read-laziness-and-isolation.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-08-10
```

### Op 30 — global or per-boundary

`classification: requires-new-decision` · `destPath: .kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
· `sourceFiles: 0013`

```yaml
id: kb-open-question-global-vs-boundary-visibility-001
title: Whether the visibility invariant needs to be global
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0013 freezes the visibility invariant globally rather than per consistency boundary, and
  says openly that it closed the question by decision rather than by evidence. The argument is
  that a per-boundary invariant would keep AppendCondition sound and make the projection
  checkpoint unsound, because a checkpoint is a single position across all boundaries. The measured
  cost of the choice is real and small: arm B-tag, the per-boundary lock, ran at 0.935 of baseline
  at 64 clients, so the global invariant is worth about nine percent of throughput at that load.
  What is not decided is whether the premise holds — it rests on the projection checkpoint being
  global, which phase 6 has not frozen. Refuted by a projection checkpoint design that turns out
  to be boundary-scoped, which would remove the argument the global choice rests on and make that
  nine percent real money. Owned by phase 6, the ProjectionStore freeze. If it fires, this section
  of ADR-0013 reopens and needs its own decision.
depends_on: []
related:
  - kb-decision-0013
  - kb-reference-position-visibility-experiment-001
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - experiments/position-visibility/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 31 — arm C through a real driver

`classification: requires-new-decision` · `destPath: .kb/open-questions/postgres-arm-c-structural-cost.md`
· `sourceFiles: 0013`

```yaml
id: kb-open-question-postgres-arm-c-cost-001
title: Whether a real Postgres adapter can express arm C cleanly
kind: open_question
status: accepted
authority_tier: note
summary: >-
  The mechanism by which a Postgres adapter buys position visibility is settled — xid8 plus
  pg_snapshot_xmin, the only arm that passed the inversion detector on both writer pairs while
  leaving writers unserialised. What is not settled is whether a real adapter can express it
  cleanly through sqlx and at what structural cost, because the experiment measured four SQL
  strategies and not four implementations of the port: no connection pooling, no transaction
  lifetime tied to a trait method, no cursor, no error mapping. The staleness the arm buys is also
  unpriced against a real workload — 0.688 ms unloaded, but 4010.719 ms behind an unrelated
  five-second write in an unrelated database, and nothing yet says which of those a caller should
  plan for. Refuted by an adapter that cannot express arm C cleanly, or whose frontier staleness
  is unacceptable under load. Owned by phase 10 and by ADR-0024, which also owns the choice of
  happenstance-postgres's actual mechanism.
depends_on: []
related:
  - kb-decision-0013
  - kb-reference-position-visibility-experiment-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - experiments/position-visibility/
  - crates/happenstance-postgres/src/lib.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

### Op 32 — how strong the one rule actually is

`classification: requires-new-decision` · `destPath: .kb/open-questions/poll-count-bounds-the-visibility-rule.md`
· `sourceFiles: 0013`

```yaml
id: kb-open-question-poll-count-rule-strength-001
title: The rule that checks the visibility invariant has an uncalibrated window
kind: open_question
status: accepted
authority_tier: note
summary: >-
  nothing_below_an_observed_position_appears_later is the only rule checking the invariant ADR-0013
  lifted to frozen, and its discriminating power varies with something nobody has measured: the
  adapter's poll shape. The technique builds two append futures from one handle and polls them out
  of order, which works because a Rust future is cold until polled — but the window it opens is
  bounded by the number of polls the adapter's append needs, and an adapter needing three or more
  may slip through it. What is not decided is how to calibrate it. The direct repair, a POLL_BUDGET
  capability on the fixture, was considered and rejected at phase 4: it would be the first
  non-boolean capability, it sits close to the no-clock rule, and calibrating it needs a real
  I/O-bound adapter that does not exist. Refuted by a poll-padding decorator over
  PreCommitPositionStore that the rule fails to reject, calibrated against a three-poll adapter.
  Owned by phase 10. If it fires, only the rule changes — the clause it checks does not.
depends_on: []
related:
  - kb-decision-0013
  - kb-playbook-cold-future-hand-polling-001
  - kb-decision-0010
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/fixtures.rs
last_reviewed: 2026-08-10
```

### Op 33 — two frozen clauses whose rules nobody is scheduled to write

`classification: requires-new-decision` · `destPath: .kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
· `sourceFiles: 0013, 0011`

```yaml
id: kb-open-question-es-38-and-gap-read-unowned-001
title: Two frozen clauses name rules that are unwritten and unscheduled
kind: open_question
status: accepted
authority_tier: note
summary: >-
  A FROZEN marker binds the design; it does not assert that anything checks it. Two clauses
  demonstrate the difference. ES-38's rule, positions_are_not_reused_after_removal, cannot be
  written today: it needs a store capable of removing events, the fixture declares no such
  capability, and the completeness instrument it depends on is deferred with nothing planned
  before phase 14. And read_from_a_gap_position, which ES-9 is owed, is now named by two accepted
  decisions — ADR-0013 and ADR-0011 — and scheduled by neither; ADR-0011 claims ES-9 itself but
  assigns no owner to this rule, and ADR-0013 says explicitly that "0013 covered it" must not
  become the reason it goes unclaimed. What is not decided is who writes each, and whether a
  frozen clause may name a rule with no owning phase at all. Owner for ES-38's rule: phase 14.
  Owner for read_from_a_gap_position: unassigned, which is the point. Related in shape but not in
  subject to the ES-6 question, which is about a rule that cannot be written rather than one
  nobody has been asked to write.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0011
  - kb-open-question-es-6-unwritable-rule-001
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-08-10
```

### Op 34 — ProjectionId is unvalidated

`classification: requires-new-decision` · `destPath: .kb/open-questions/projection-id-is-unvalidated.md`
· `sourceFiles: 0015`

```yaml
id: kb-open-question-projection-id-unvalidated-001
title: ProjectionId::new is infallible, and that was never decided
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ProjectionId::new is infallible and unvalidated: the empty string succeeds and becomes a
  checkpoint row's primary key. ADR-0015 validated every other identifier in the crate and
  declined to validate this one, for two stated reasons — there is no conformance suite for
  ProjectionStore, which is provisional and freezes at phase 6, and calling the type "deliberately
  opaque" in its docstring would be false, because there was never a decision, only an omission.
  The docstring says instead that the question is open and names its owner. What is not decided is
  whether a projection id is validated at all, and if so against what: the identifier rules
  ADR-0015 applies to EventType and Tag are about wire safety and byte equality, and a checkpoint
  key's constraints come from the stores that persist it. Forced by phase 6, the ProjectionStore
  freeze, after which the type is on a frozen port. Refuted, in the sense of settled, by the first
  projection store whose backing table rejects a key the constructor accepts.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0007
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/0015-validated-identifiers-and-store-limits.md
  - docs/adr/0015-validated-identifiers-and-store-limits.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
```

### Op 35 — who owns the fixture's numeric limits

`classification: conflicts` · `destPath: .kb/open-questions/cf-40-fixture-limits-ownership.md`
· `sourceFiles: 0015, 0012`

```yaml
id: kb-open-question-cf-40-ownership-001
title: CF-40's ownership is claimed and disclaimed by the same decision
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-40 is the clause requiring a fixture to declare its actual numeric limits — maximum event
  data length, tags per event, events per batch — as optional constants defaulting to no ceiling,
  reported through the declined-capability path. ADR-0015 mints it, and contradicts itself about
  whether it owns it: the header and introduction say CF-40 was settled at sign-off and "lands
  here", while the Consequences section for the same decision says the fixture-constant ownership
  question "is not settled" and that decision 8 "sets out both and declines to choose". The other
  claimant is ADR-0012, which already owns CF-39 and MID_BATCH_FAULT in the same fixture-contract
  area, and ADR-0015's own reasoning is that neither decision may resolve this unilaterally. Both
  decisions are accepted and neither is edited, so the contradiction stands as imported. What is
  not decided is which document the fixture's capability surface belongs to as a whole — the
  question is not really about CF-40 but about whether the fixture contract has one owner or is
  amended by whichever decision needs it next. Forced by phase 8, the first adapter with real
  limits.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0012
  - kb-decision-0010
source_paths:
  - .kb/_intake/0015-validated-identifiers-and-store-limits.md
  - .kb/_intake/0012-append-shape-and-preconditions.md
  - docs/adr/0015-validated-identifiers-and-store-limits.md
  - docs/adr/0012-append-shape-and-preconditions.md
  - crates/happenstance-testkit/src/fixtures.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

**This is the wave's one genuine conflict between two things it is importing**, and authority
rule 4 governs: it is deferred, not resolved. Both decision atoms record it and link here; neither
body is edited to prefer one reading, because both readings are in one accepted document and
picking between them is a decision this wave has no standing to take.

### Op 36 — the DCB reference publishes nothing to be compatible with

`classification: requires-new-decision` · `destPath: .kb/open-questions/dcb-reference-publishes-no-wire-format.md`
· `sourceFiles: 0016`

```yaml
id: kb-open-question-dcb-no-published-format-001
title: There is no DCB wire format to interoperate with
kind: open_question
status: accepted
authority_tier: note
summary: >-
  WF-1's interoperability half stays DEFERRED, and the reason changed when it was checked: the DCB
  reference implementation publishes no wire format at all. EventStore.ts contains no serialisation
  code, and the specification's JSON snippets are explicitly a "potential" representation rather
  than a required one. So happenstance's format is not diverging from a standard; there is no
  standard to diverge from, which is what makes the format private and every reversal in it free.
  What is not decided is what happens if that changes. Refuted by a DCB implementation that
  publishes an encoding — at which point the question becomes whether interoperability is worth
  anything to this project, and the answer is not obviously yes. Owned by phase 13, the sync
  crate and its testkit, which is the first point at which anything of ours is on a wire another
  implementation could be at the far end of.
depends_on: []
related:
  - kb-decision-0016
  - kb-reference-wire-format-measurements-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - docs/adr/0016-the-wire-format.md
  - experiments/wire-format/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
```

### Op 37 — human-readable encoding on a peer that cannot buffer

`classification: requires-new-decision` · `destPath: .kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
· `sourceFiles: 0016`

```yaml
id: kb-open-question-human-readable-encoding-limits-001
title: Whether a human-readable payload encoding is available at all on a memory-limited peer
kind: open_question
status: accepted
authority_tier: note
summary: >-
  WF-11 adds base64 for the human-readable half of the wire format, and stays PROVISIONAL on a
  falsifier that turns out to be broader than base64. serde's Serializer offers no streaming entry
  point for a human-readable string: serialize_str and collect_str both materialise the whole
  payload first. That is a property of serde's data model rather than of base64, so it falsifies
  human-readable payload encoding as a category — hex has the same problem, and so would any
  alternative — and if it fires, WF-11's MUST has to be re-scoped to formats rather than to peers.
  What is not decided is whether any peer will actually meet the condition. Refuted by a Workers
  peer that must forward a payload it cannot buffer, which is the first realistic place a memory
  ceiling and a large payload meet. Owned by phase 9, the Cloudflare Durable Object adapter. The
  binary half is unaffected: postcard encodes the same 340 KiB payload in 348,163 bytes against
  base64's 464,218.
depends_on: []
related:
  - kb-decision-0016
  - kb-decision-0003
  - kb-reference-wire-format-measurements-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - docs/adr/0016-the-wire-format.md
  - crates/happenstance-sync/src/wire.rs
  - experiments/wire-format/
last_reviewed: 2026-08-10
```

### Op 38 — what FORMAT_VERSION = 1 names

`classification: requires-new-decision` · `destPath: .kb/open-questions/sync-message-set-and-format-version.md`
· `sourceFiles: 0016`

```yaml
id: kb-open-question-sync-message-set-undesigned-001
title: FORMAT_VERSION = 1 names a message set that does not exist yet
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0016 ships a generic envelope with a hand-written Deserialize that refuses an unknown
  format_version before decoding the message, and deliberately ships no enum of message kinds —
  the message set is phase 13's design, and inventing one at phase 5 would be designing the
  replication protocol inside a wire-format decision. So the version field currently names nothing
  in particular: FORMAT_VERSION = 1 is a promise about a vocabulary that has not been chosen. What
  is not decided is what that vocabulary is, and therefore what a version bump would mean.
  Refuted by a phase-13 design in which versions are negotiated per connection rather than carried
  per message, which would make the field dead weight on every message and removing it a format
  break. Owned by phase 13. Distinct from the question about the DCB reference's absent format:
  that one is about whether anyone else's encoding matters, this one is about what our own
  messages are.
depends_on: []
related:
  - kb-decision-0016
  - kb-open-question-dcb-no-published-format-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - docs/adr/0016-the-wire-format.md
  - crates/happenstance-sync/src/wire.rs
  - crates/happenstance-sync/src/lib.rs
last_reviewed: 2026-08-10
```

---

## Adjudications

**Seventeen decision atoms are authored, and none of them is a decision this wave took.** The
previous wave declined to author any, on three arguments it inherited from the intake corpus
itself: a pass that both discovers and decides cannot be audited; a decision taken to close an
ingest is taken for the wrong reason; and ADRs here have a described process with a human sign-off
an ingest cannot supply. **All three still hold, and none of them applies to transcription.** Each
of these seventeen documents was written, argued, and accepted before this wave existed; two of
them name their sign-off events and dates in their own headers. Twelve claims that *would* need a
new decision are in `open-questions/`, exactly as last wave.

**No accepted decision atom's body is edited, including by later operations in this wave.** Three
opportunities to break that rule were declined and are worth naming, because each looked like a
tidy-up:

- ADR-0011 shows that ADR-0001's `dynosaur` consequence is wrong. ADR-0001's atom keeps it, with
  a `related` edge to the reference atom that carries the compiled correction. Correcting the
  corrected document is what the corpus calls rewriting the reasoning (Op 22).
- ADR-0004's let-chains constraint is rescinded by ADR-0029. It stays verbatim in Op 8, because
  ADR-0029 amends rather than supersedes and says explicitly that ADR-0004's reasoning is what it
  acted on.
- ADR-0015 contradicts itself about CF-40. Neither reading is deleted (Op 35).

**`supersede` appears as no operation's `op`, and that is correct rather than a gap.** The
operation type exists for a new atom superseding an atom already accepted in the corpus, whose
frontmatter then gets the one permitted metadata flip. Nothing in `.kb/decisions/` is accepted —
the layer is empty — so ADR-0002 does not get flipped; it arrives already superseded, in 2026-08-05
and by a document in the same batch, and is authored in that state. `create_new` with
`status: superseded` is the honest spelling. A later wave importing ADR-0017 onwards may well need
the real thing.

**Nothing was routed to `product/` or `design/`.** Same as last wave, same reason: an event
sourcing library has no personas and no interaction surface, and forcing a fit would give a stock
answer the standing of a finding.

**Nothing was routed to `narratives/` or `roadmaps/`.** Both were considered. The ADR chain's
story — 0002 superseded in thirty-two minutes, 0005 partly reversed four hours later, 0006 partly
corrected the next day — is a genuine narrative, and `RUNBOOK.md` is a genuine roadmap. Neither is
imported: the narrative's durable content is the two atoms in Ops 22 and 23, and mirroring a
fifteen-phase runbook into a `roadmap` atom would be the mirror-nobody-maintains the reference
README bars, against a document that changes weekly.

## What the Maps phase inherits

- **A decision map must be created.** `maps/README.md` names it as belonging in that layer — "one
  row per `decision` atom, its status, and what it supersedes or is superseded by" — and it does
  not exist, because the previous wave created zero decision atoms. Seventeen land here. The map
  must carry the supersession chain (0002 → 0005 → 0006 → 0007), the amendment (0004 ← 0029), and
  the **partial** supersessions as an annotation, since the frontmatter deliberately does not
  carry them (Standing choice 2). Do not infer supersession from `status` alone: ADR-0005 and
  ADR-0006 are `accepted` and are both partly reversed.
- **The domain map needs new sections**, appended rather than edited, per its own "Adding a
  domain" instruction. The existing section is "Specification governance & conformance". The
  natural new ones, on the evidence of this wave's clusters: *the storage ports and their two
  flavours* (0001, 0008, 0009, Op 1); *the contract's value types and identity* (0014, 0015);
  *read, append and position semantics* (0011, 0012, 0013, Op 2, Op 4, Op 24); *crate naming and
  packaging* (0002, 0005, 0006, 0007, Op 22, Op 23); *the toolchain* (0004, 0029); *the wire
  format* (0016, Op 3); *the conformance suite* (0010). Seven sections is a lot to add at once,
  and grouping is the Maps phase's call — but the crate-naming section is specifically requested
  by the ADR-0002 extract and specifically wanted, because four atoms in it are unreachable from
  anything else in the corpus.
- **Twelve new bullets on the open-questions index**, plus two existing bullets whose atoms
  changed (Ops 25 and 26). Eleven of the twelve belong to library domains that do not have index
  sections yet.
- **Reciprocal backlinks.** Every link in this plan except Op 6's declared `superseded_by` is
  outbound to an earlier operation. The inbound halves are the Maps phase's: each decision atom
  back to the open questions it spawned (0007→28, 0011→29, 0013→30/31/32/33, 0015→34, 0015+0012→35,
  0016→36/37/38), Op 1 back to Ops 5, 12, 13, 14, 15, and Op 4 back to Ops 15, 16, 17.
- **`decisionMap: true` on nineteen operations** — the seventeen decision atoms, plus Ops 25 and
  26, whose amended `related` lists now reach into the decision corpus.
