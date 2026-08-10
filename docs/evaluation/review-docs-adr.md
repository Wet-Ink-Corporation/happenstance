# Review — documentation, ADR corpus, RUNBOOK

**Lens:** are the documents true, coherent, and load-bearing?
**Date:** 2026-08-05 · **Repo:** `D:\repos\happenstance` @ `9fd2337` + uncommitted working tree

---

## Summary

The corpus is unusually strong. Factual drift against *code* is close to zero — I checked the
claims that were most likely to be wrong and they hold. The drift is almost entirely **internal**:
ADR-0006 was accepted today and nothing else in the repository has been told. The second cluster of
problems is not drift at all but **sequencing**: the RUNBOOK schedules the two phases that can
invalidate the contract (the typed layer, and the `!Send` proof) after four phases of code written
against it, and it takes the one named risk that both naming ADRs say could reopen them — someone
else registering `happenstance` on crates.io — and puts it in no phase at all.

---

## 1. Factual drift

### Verified true — do not relitigate

| Claim | Where | Verdict |
|---|---|---|
| "27 rules" | `README.md:66`, `RUNBOOK.md:153` | **True.** 27 `conformance_test!` entries in `testkit/src/lib.rs:93-129` (8 query + 5 read + 2 position + 4 append + 7 condition + 1 concurrency) and 27 `pub async fn`s in `suite.rs`. |
| "There is a unit test asserting this" (read returns the stream at top level) | `CLAUDE.md:56` | **True.** `memory.rs:329-340`, `read_stream_is_send`, with a comment naming the constraint. |
| `cargo xtask wasm # just the wasm32 check` | `CLAUDE.md:99` | **True.** `xtask/src/main.rs:117` → `REQUIRED[3..4]`, index 3 is the wasm step. |
| "defined once in `xtask/src/main.rs` and is exactly what CI runs" | `README.md:156`, `CONTRIBUTING.md:30` | **True.** `ci.yml` runs `cargo xtask ci`; the alias exists in `.cargo/config.toml`. |
| "ADR-0002 was superseded thirty-two minutes after it was written" | `CONTRIBUTING.md:14`, `ADR-0006:152` | **True.** `85dade5` 13:02:45 → `6e8c20d` 13:34:11 = 31m26s. |
| "the manifest currently carries a comment saying this is missing" (per-crate README) | `RUNBOOK.md:435-437`, `ADR-0005:81-85` | **True.** `crates/happenstance/Cargo.toml` lines 11-13. |
| Status table, 6 crates and their states | `README.md:63-70` | **True**, including "stub, open questions written down" — the stubs really do carry design notes. |
| CLAUDE.md repository map vs the tree | `CLAUDE.md:26-36` | **True today**, false after the rename (see §1.1). |
| Every `.kb/decision/NNNN` link and every RUNBOOK anchor | all | **All resolve.** `#phase-7--publish-01`, `#standing-constraints`, `#release-hazard`, `#decision-ledger` all match their headings. |
| Phase 0 state ("`.idea/` and `.mcp.json` undecided", "gate green at `9fd2337`") | `RUNBOOK.md:92,106` | **True.** Both are still untracked; `9fd2337` is HEAD. |

### 1.1 — CLAUDE.md names the wrong crate everywhere, and one constraint inverts

ADR-0006 is `Status: accepted`. It has not been executed. CLAUDE.md has not been told.

- `CLAUDE.md:9-10` — "`happenstance` defines the contract" → after the rename, `happenstance` is the
  typed layer.
- `CLAUDE.md:26-36` — the repository map lists `crates/happenstance/` as "the contract" and
  `crates/happenstance-runtime/` as a named seam. ADR-0006:80: "`happenstance-runtime` ceases to
  exist."
- `CLAUDE.md:38-39` — "everything depends on `happenstance`; `happenstance` depends on nothing in
  this workspace." After the rename this is exactly inverted: `happenstance` will depend on
  `happenstance-core` and on adapters.
- **`CLAUDE.md:50-52` — "Never put `serde` in `happenstance`'s default features."** This is the
  dangerous one. After the rename, `happenstance` is the crate that *contains the codecs* and must
  have serde. A session following this instruction literally would refuse serde to the crate that
  needs it and permit it into the contract crate — the precise inversion of ADR-0003. ADR-0006:86-91
  anticipates the ambiguity and resolves it ("it attaches to the crate that defines the ports"), but
  the resolution lives only in the ADR.
- `CLAUDE.md:128` — the open-questions list still carries "Whether `happenstance-runtime` is the
  right name and the right seam", which ADR-0006 closed today. A session reading CLAUDE.md would
  treat a settled decision as open.
- `CLAUDE.md:110-115` says the RUNBOOK ledger "is the source of truth" and then keeps four stale
  summaries beside it. Duplicated state drifts; this corpus is already the proof.

Same staleness in `crates/happenstance-runtime/src/lib.rs:10` — "Even the crate name is provisional"
— and in its `Cargo.toml` description.

**Fix:** execute the rename now. It is mechanical, nothing is published, and the current state — an
accepted ADR that the operating document contradicts — is the worst state the repo can be in for
every session between now and phase 3. If it is not executed today, demote ADR-0006 to `proposed`
and say so at the top of CLAUDE.md.

### 1.2 — The README quick start does not compile, and nothing checks it

`README.md:87-100`:

```rust
let store = MemoryEventStore::new();
store.append(/* … */).await?;
let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
```

`.await` and `?` at what rustdoc would make `fn main()` scope. It does not compile as a doctest —
and no `#![doc = include_str!("../README.md")]` exists anywhere in the workspace, so nothing ever
tries. Meanwhile `CLAUDE.md:89`: *"Doctests are documentation that cannot rot — prefer a runnable
example to a described one."* The first code a new user reads is the one example in the repository
that can rot, and it has.

`memory.rs:36-68` shows the correct pattern for exactly this snippet, hidden `#[tokio::main]` and
all. Port it, then include the README as a doctest — that kills the whole class. The
DCB-in-sixty-seconds block at `README.md:29-50` is a deliberate fragment (`subscribed` is
undeclared) and should be fenced ```rust,ignore, otherwise including the README breaks the build.

`README.md:82-85` also advertises `happenstance = "0.1"`, which does not resolve. The "Status:
early" banner covers this, but see §4 on name reservation.

### 1.3 — CONTRIBUTING claims a semver baseline that does not exist

`CONTRIBUTING.md:80-81`: "Pull requests run an extra `cargo-semver-checks` job against the published
crates." Nothing is published. `ci.yml`'s `semver` job targets `happenstance, happenstance-testkit`
with no baseline. `RUNBOOK.md:453-454` is honest about this ("verify on the next pull request that
it *does* something"); CONTRIBUTING is not. One sentence.

### 1.4 — The testkit's "What is checked" table overclaims by one row

`testkit/src/lib.rs:36` — "Positions | unique; strictly monotonic; **gaps permitted**". There is no
gaps-permitted rule; there are two position rules. "Gaps permitted" is the *absence* of an
assertion, correctly so, but it is listed as a thing the suite checks.

### 1.5 — The testkit's two doctests verify nothing

`testkit/src/lib.rs:12-15` uses `# #[cfg(feature = "doctest-only")]`. That feature does not exist in
`happenstance-testkit/Cargo.toml`, so the line is permanently cfg'd out and the block compiles
nothing (`MemoryEventStore` is not even imported). `lib.rs:64-71` wraps in a `macro_rules! ignore`.
Both are elaborate spellings of ```` ```ignore ````. Harmless, but this is the crate whose thesis is
*"a claim about behaviour is worth exactly as much as the test that checks it"* (`lib.rs:3-4`). Use
```` ```ignore ```` and point at `tests/memory_conformance.rs`, which is the real proof.

### 1.6 — The `serde` feature has zero tests

`grep` for a serde-gated test module returns nothing. No round-trip test exists anywhere. Yet:

- ADR-0003 makes this feature the load-bearing enabler for the entire replication story;
- `event.rs:335-336` — `EventWire` exists "to keep the wire format stable";
- `tag.rs:318-319` — `Tags::deserialize` re-canonicalises so "a peer that sends unsorted or
  duplicated tags cannot smuggle a non-canonical `Tags` into memory".

That last is a *security* claim about untrusted input, asserted in a doc comment, never executed.
`--all-features` compiles these impls; nothing runs them. A `round_trip(x) == x` proptest per
envelope type plus a direct non-canonical-input case is an hour's work and belongs before publish.

---

## 2. The ADR corpus's epistemic state

**Healthy, with one process gap and one calibration error. Not thrashing.**

Two naming ADRs in one day looks like churn and is not. ADR-0005 bundled two decisions under an
"and"; ADR-0006 unbundled them, kept the good one, and reversed the one that "rode in on its
momentum". That is what an ADR corpus is *for*. The hygiene around it is genuinely excellent and
rare: ADR-0002 is kept verbatim, and ADR-0005:53-60 explains that rewriting `eventum` →
`happenstance` inside it "would invert the claim into a falsehood" — applying the repo's own rule to
itself. Keep that.

### 2.1 — The provisional marker: real improvement, wrong instrument

The **lift conditions** are the valuable part. "Lifts when a genuine `!Send` implementer passes the
conformance suite" (ADR-0001) is a falsification test. Stating one is a real epistemic improvement
over an undated `accepted`.

The **word** is not. Read what it operatively says (ADR-0001:11-12): *"work that contradicts it still
needs a superseding ADR, but it does not owe deference."* Compare CONTRIBUTING.md:6-8 for **every**
ADR: *"Changing one is fine — but it means writing a new ADR that supersedes the old one."* The
obligation is identical. "Provisional" is a tone control, not a rule — and a status that changes no
obligation is an invitation to argue about status.

**Two problems it does cause:**

1. **No retirement mechanism.** No phase in the RUNBOOK has "lift ADR-0001's marker" as work or as
   an exit criterion. `RUNBOOK.md:195` does exactly this for the projection port (*"`projection.rs`
   no longer says 'provisional'"*), which proves the author knows how to wire it and simply didn't
   for the ADRs. A marker nobody is tasked with removing becomes permanent, and a permanently
   provisional ADR is indistinguishable from no ADR.
2. **Three different epistemic states, one label.**
   - **ADR-0001 — correctly provisional.** No `!Send` implementer exists. The lift condition can
     genuinely falsify the decision. This one earns the marker.
   - **ADR-0003 — overstated.** Only the *payoff* is unproven. The *decision* is overdetermined: its
     own "Alternatives rejected" kills generic-over-`E` (infects every adapter signature, forecloses
     `dyn`) and `serde_json::Value` (locks the ecosystem to JSON) on grounds that do not depend on
     replication ever existing. Marking the whole ADR provisional understates a settled decision.
   - **ADR-0004 — category error.** An MSRV is a policy, not a hypothesis; no experiment lifts it.
     The note's actual content (*"a self-imposed constraint that can be raised at any time at zero
     cost to anyone"*) is excellent and is the best single sentence in the corpus — it just isn't
     provisionality.

**Recommendation:** drop the status word. Make `## Evidence` a required ADR section — *what supports
this, and what would falsify it* — which is what the lift conditions already are. Then wire each
falsification test to an exit criterion in the phase that owns it. That keeps everything the marker
bought and removes the ambiguity about what "accepted — provisional" means.

### 2.2 — ADR-0006's self-exemption: coherent, but the weakest available defence

> *"It is not marked provisional only because a naming decision is settled by being made, not by
> being tested."* (ADR-0006:159-160)

**Coherent**, given how provisional is defined here: a lift condition is an empirical test, naming
has none, so the marker would be permanent and therefore useless.

**But it is the weakest defence of a correct conclusion, and ADR-0006 is its own counterexample.**
ADR-0005's naming decision was overturned five hours later by *argument* — the who-imports-what
table, the ecosystem-precedent table — not by code. "Settled by being made" is precisely the claim
ADR-0005 could have made, and it would have been wrong. The honest form is: *naming decisions are
settled by argument, not evidence; this one is settled because the argument is strong and no
counter-argument survives* — and then name the counter-argument that would reopen it (evidence that
adapter authors, not applications, are the dominant import population).

**A second asymmetry.** ADR-0006:21-22 indicts ADR-0005 for "an empirical claim about the behaviour
of users who do not exist, asserted without evidence" — and then presents its own "Who imports what
/ Population: **the overwhelming majority**" table (lines 41-46) with no evidence either,
substituting ecosystem precedent. Precedent is better than assertion, but it is not the categorically
different thing the rhetoric implies.

**A third: one claim is overstated.** ADR-0006:23-26 — "the facade it says 'may never be built' is
**phase 3** … The two documents … contradict each other." ADR-0005's cost was about a
*batteries-included facade*. RUNBOOK phase 3 scheduled a *typed layer* (codecs, `DomainEvent`,
`DecisionModel`), which `happenstance-runtime/Cargo.toml` confirms is not a facade — it gates no
adapters. ADR-0006 *makes* it a facade by decision (line 76: "Re-exports the contract; feature-gates
the adapters"). The contradiction is manufactured by the decision it justifies.

**The conclusion is right.** `serde_core`/`serde`, `futures-core`/`futures`, `tracing-core`/`tracing`
is the correct precedent for this exact shape, and ADR-0006:63-67 lands the sharpest point in the
corpus — that `-core` "existed for no reason but the collision" was true of `eventum-core` and is
false here. The argument should simply own that it is a judgment call made on precedent.

---

## 3. Decisions with no ADR

| Decision | Where recorded | Needs an ADR before 0.1? |
|---|---|---|
| `append` returns the **last** position | `store.rs:125-128` (doc comment, "the specification does not require this"), conformance rule `append_returns_last_written_position` | **Yes** |
| `AppendError::NoEvents` as a contract-level runtime error | `error.rs:154-161` | **Yes** |
| No timestamp and no id on `Event` | nowhere | **Yes** — with the phase-3 identity ADR |
| `bytes::Bytes` and `futures_core::Stream` in the public API | ADR-0003 covers `Bytes` as a payload representation only | **Yes** |
| 255-byte limits on tags and event types | `tag.rs:10-14` (partial reason), `event.rs:13-14` (none) | No — doc comment |
| Control-character rejection | `tag.rs:46-47`, `event.rs:36-37`, error text | No — doc comment |
| `Tags` canonical sort as an adapter-indexable guarantee | `tag.rs:137-145` + `properties.rs:33-40` | **No — already right** |
| `ReadOptions.from` inclusive vs `AppendCondition.after` exclusive | both documented, both conformance-tested | No — cross-reference them |
| `collect()` discards partial results on error | `store.rs:158-161`, `# Errors` | **No — already right** |

**`bytes` and `futures-core` are the one I would not skip.** ADR-0003 argues `Bytes` is the right
payload *representation*. It never addresses the semver consequence: `bytes` and `futures-core` are
now permanent **public** dependencies of the crate whose stated purpose is "a semver surface that
should almost never move" (ADR-0006:113-114). `futures-core` is a `0.x` — under Cargo's rules every
minor bump is breaking. `lib.rs:104-106` re-exports `bytes` as mitigation and does not re-export
`futures_core`, so the weaker of the two dependencies is the unmitigated one. The alternatives
(a local `Stream` trait, waiting for `core::async_iter`) are worse, but that is what the ADR should
say.

**`AppendError::NoEvents` is the sharpest of the rest**, because it is an internal-consistency
problem rather than a taste one. This codebase makes illegal states unrepresentable *everywhere
else* — `Query` is an enum so empty is not constructible, `SequencePosition` is `NonZeroU64`,
`ReadOptions::limit(0)` is silently ignored — and the README advertises the principle by name
(`README.md:104`). `append(&[])` is the single place it is abandoned, and the abandonment is then
promoted to a conformance rule every third-party adapter must implement forever. Either take a
non-empty type, or record why the runtime error won.

**`append` returning the last position** is beyond-spec, is a conformance rule, and interacts with
the phase-3 identity decision: if `SequencedEvent` gains an `EventId`, callers may want the appended
events back rather than one position. The doc comment gives the reason; an ADR would give the
rejected alternatives (`Vec<SequencePosition>`, `Vec<SequencedEvent>`), which is the part that
matters when someone asks in a year.

**`from` inclusive / `after` exclusive** is *correct* and deliberate — one means "start here", the
other means "I have already seen up to here" — but the two are never named side by side, and it is
the single easiest thing for an adapter author to get backwards. One cross-reference in each
doc comment.

---

## 4. RUNBOOK phase ordering

### 4.1 — The ledger has become a shadow ADR store, and it contradicts the phase bodies

`RUNBOOK.md:18-21`, rule 3: *"Decisions are settled in ADRs, not here."* Then 8 of 13 ledger rows
carry substantive decisions marked **decided**, ADR **pending**. That is the failure mode rule 3
names, in the document that names it. Worse, three of them contradict the phase bodies they govern:

1. **Testkit restructure.** Ledger (`:72`): "decided — **no testkit restructure needed**; add a rule
   *registry* macro + a `!Send` reference store (**pulled forward to phase 1**)". Phase 1's Work list
   (`:135-149`) contains no such item. Phase 5's body (`:318-323, 329-330`) still says "Restructure
   the testkit if required" and treats the harness as an open design problem.
2. **Event identity.** Ledger (`:67`): "decided — Lamport pair `(origin, origin_position)` as
   `EventId` on `SequencedEvent`". Phase 3's Work (`:226-228`): "UUIDv7 or content hash; metadata key
   or a field on `Event`". Release hazard (`:392-393`): "a field on `Event` or a reserved metadata
   key". `happenstance-sync/src/lib.rs:33-36`: "a UUIDv7 or a content hash in the event's metadata".
   **Four framings of one decided question, on three different types.**
3. **ADR-0006's consequences never propagated.** ADR-0006:93-101 moves the projection runner into the
   contract crate. `RUNBOOK.md:242-243` still lists it as phase 3 Work;
   `happenstance-runtime/src/lib.rs:37-40` still lists it as planned contents. And this matters
   structurally: phase 3's stated dependency on phase 2 is *"a projection store to run projections
   into"* (`:205-206`) — that was the **runner's** dependency, not the typed layer's. ADR-0006
   removed it, which frees phase 3 to move earlier. See §4.3.

### 4.2 — Phase 4 is off the critical path to publish, and phase 2 says "freeze"

Phase 2 freezes the projection port against one adapter. Phase 4 is the deliberately-unlike second
adapter — and the status table has phase 7 depending on **2, 3, 6**. Not 4.

So the port ships "frozen", proven by exactly the one adapter it was designed around, with the
adapter that exists to falsify it not required before release. Phase 4's own text knows this: *"One
implementation freezes a port; two prove it"* (`:263`).

**Cheapest fix is not reordering Ladybug.** The specific thing at risk is the GAT:

```rust
type Batch<'a> where Self: 'a;
async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error>;
```

That shape assumes the write handle **borrows from the store** — true of a `rusqlite` transaction,
not obviously true of an engine whose write handle is owned, or `!Send`, or must outlive the call.
Type-checking a sketched `lbug` `Batch<'a>` signature against this GAT — no implementation, no `lbug`
dependency, an afternoon — retires most of the risk inside phase 2. Failing that: make phase 7 depend
on 4, or ship the projection port behind an `unstable-projection` feature at 0.1.

### 4.3 — SQLite before the typed layer is backwards

Phase 3's stated reason for coming after 1 and 2 is *"It needs a durable store to be worth
demonstrating"* (`:205`). That conflates **demonstration** with **discovery**. Demonstration needs
SQLite. Discovery does not: `MemoryEventStore` is a conformant `EventStore` today.

The two failure modes are not symmetric.

- **SQLite discovers implementability defects.** Real, but narrowed by the fact that one conformant
  implementation already exists. What it genuinely adds is whether the append-condition can be
  pushed into SQL and whether the streaming read survives a connection boundary.
- **The typed layer discovers contract defects that change `happenstance-core`'s public API.** Does
  `Event` need an id? A timestamp? Does `DecisionModel` composition require `Query` to expose
  union/intersection operators it does not have? Does the retry loop need `ConditionViolated` to
  carry the conflicting *events*, not just a position? Does the command loop need all assigned
  positions back from `append`?

Every item in the second list forces revision of every adapter written against the first. **And the
RUNBOOK already knows one of them is coming** — it schedules event identity, a change to `Event` or
`SequencedEvent`, in phase 3, *after* SQLite and the projection port have been built and frozen
against the current shape.

**Recommendation:** split phase 3.
- **3a — typed-layer spike over `MemoryEventStore`, before SQLite.** `DomainEvent`, `DecisionModel`,
  the command loop, and the event-identity ADR. Exit criterion: `course-subscriptions` rewritten
  with typed events over the in-memory store. No durable store required.
- **3b — stays where phase 3 is.** Re-point the worked example at SQLite; `Codec` implementations.

ADR-0006 has already removed the dependency that made this impossible.

### 4.4 — The wasm proof is fifth, and it is the largest structural risk in the project

ADR-0001 is the most committing decision in the repository. It shapes every port signature, forbids
`#[async_trait]`, forbids `dyn EventStore`, forces `read` to be non-`async`, and creates the E0034
footgun that CLAUDE.md, CONTRIBUTING, the RUNBOOK and the `store` module all have to warn about. Its
entire justification is a target exercised in **phase 5**. ADR-0001 says so itself: *"no `!Send`
implementation of these ports exists anywhere, not even a reference one."*

And the current wasm evidence is thinner than advertised. `xtask/src/main.rs:62-73` runs
`cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std`
— the `memory` feature is excluded, so the only `EventStore` implementation in the workspace is never
wasm-checked. `README.md:126` — "CI builds `happenstance` for `wasm32-unknown-unknown` on every
commit so this stays true" — is literally true and materially weaker than it reads.

Compounding it: phase 5's own risk statement (`:316-323`) is that the conformance rules are welded to
`#[tokio::test]` and may need restructuring. **That is a change to the testkit's public macro API** —
the crate phase 7 publishes, and the thing every phase-1-through-4 adapter invokes. Discovering it in
phase 5 means changing the contract four adapters depend on.

**Recommendation — retire this for about two days of work:**
- Move the rule-**registry** restructure and the `RefCell`-backed `!Send` reference store into
  **phase 1's exit criteria** (the ledger already says the store is "pulled forward to phase 1"; the
  phase body was never updated).
- Add **phase 1.5**: run the rule registry under `wasm-bindgen-test` on `wasm32-unknown-unknown`
  against that `!Send` store. No Cloudflare, no Durable Object, no `workerd`. That proves the three
  things ADR-0001 actually asserts — a genuinely `!Send` type can implement `EventStore`, generic
  code binding the weak trait accepts it, and the blanket impl creates no coherence conflict — and
  proves the rules run off tokio.
- Phase 5 then becomes an integration exercise rather than a design-validation exercise, which is
  what a phase five should be.

### 4.5 — Missing phases

| Missing | Severity | Why |
|---|---|---|
| **Name reservation** | **Highest value per hour in this document** | ADR-0005:88-91 — *"Nothing prevents someone else registering it before the first real release, which would force this entire ADR to be reopened. Publishing even a placeholder version claims it."* ADR-0006:126-128 — *"still outstanding and now covers **two** names… Both are free today; neither is reserved."* **No phase owns it.** The first publish is phase 7, gated behind SQLite, the projection port, the typed layer, wasm and replication. That is months of exposure on the one risk both naming ADRs flag as able to invalidate them. Cost: one hour. Do it immediately after the rename, for all six names. |
| **Benchmarks / performance baseline** | High | "fast, efficient" are the first two words of the stated goal. There is no `criterion` dependency, no `benches/`, and no performance exit criterion in any phase. Ledger row 2 defers the `max(position)` fast path "until measured" — measured with what? Put the harness **in the testkit**, so third-party adapters inherit a benchmark suite the way they inherit conformance. No other Rust DCB library offers that. |
| **Schema / event evolution** | High | Absent from every phase, every ledger row and every ADR. `Codec`'s codec tag handles *encoding* migration; nothing handles *schema* migration. This is contract-shaping: does `EventType` carry a version suffix? Does an upcaster need a read-path hook `EventStore` does not have? For a library aiming at "premiere framework", this is a bigger hole than replication. |
| **Subscriptions / live tailing** | Medium-high | `read` is one-shot. There is no way to follow the log, so the projection runner — now in the contract crate — must poll. Whether catch-up subscription is an `EventStore` method is a **contract** decision that would change the crate published at phase 7. |
| **Fuzzing / robustness** | Medium | `Tag`, `EventType`, `Query` and the whole serde envelope parse untrusted input on the replication path, with zero tests (§1.6). `cargo fuzz` targets plus the missing round-trip proptest. |
| **Alpha for real feedback** | Medium | Phase 7's exit is "crates live, docs.rs green" — no feedback loop anywhere. The goal is "widely usable, trusted, respected"; no phase gets the API in front of a user who is not the author, before it is frozen. |
| **Guide / book / example gallery** | Medium | README plus rustdoc is good. "Batteries" and "a delight to lean on" imply a guide: the DCB mental model, migrating from aggregates, testing decision models, writing an adapter (currently three lines in CONTRIBUTING). |

**On `happenstance-macros` being out of scope for 0.1** (`RUNBOOK.md:234`): defensible, but the
criterion should be written down. `DomainEvent` maps a Rust type to its `EventType` and `Tags`;
without a derive that is boilerplate per event type, and `README.md:161-162` names Disintegrate's
macro-driven design as the contrast. If the answer to "I have forty event types" is six hundred lines
of hand-written mapping, the "delight" goal fails at first contact with a real domain. Make it a
phase-3a exit criterion — *if the rewritten worked example carries more mapping boilerplate than
domain logic, the derive is in scope for 0.1* — and, regardless of ship date, design `DomainEvent` so
a derive is mechanical. A trait that cannot be derived cleanly is a trait that changes when the derive
arrives, and that risk is currently written down nowhere.

---

## 5. The release hazard and its self-correction

`RUNBOOK.md:408-421`. **The correction is right on its first move and overcorrects on its second.**

Right: an unpublished API is not a thing to protect; "breaking 0.2" is not a hazard that exists yet;
and the general form — *"treating an unpublished API as something to protect will quietly bias every
design decision toward the option that changes least"* — is the single most useful sentence in the
RUNBOOK.

Overcorrected: *"an event store whose events have no identity is not finished"* is asserted, not
argued. It swaps one unargued forcing function (publication) for another (completeness). It is true
of a *replicating* store; the DCB specification itself identifies an event by type, data and tags,
with the store's position as its identity. The stronger and narrower argument is available and is
not made: **the identity question can invalidate `SequencePosition`'s role in
`AppendCondition.after`**, which is the load-bearing type in the entire design. Answer it early
because it can invalidate the contract — not because a store is "unfinished" without it.

**Where the identified bias still survives — in the same document:**

1. **Phase 7's "Why here" is the bias, verbatim.** `RUNBOOK.md:430-432`: *"Publishing freezes the
   public API, so it comes after the two phases that change it."* A `0.x` crate freezes nothing —
   Cargo's rules permit `0.1 → 0.2` to break and the ecosystem expects it. This sentence orders the
   entire back half of the plan around avoiding a breaking change that is explicitly allowed, which
   is exactly the error the correction names four hundred lines earlier.
2. **Phase 2's "freeze the projection port"** — frozen against one adapter, before publication, for
   no reason but that publication is coming. Same bias (§4.2).
3. **`#[non_exhaustive]` as headroom.** `RUNBOOK.md:399-400` — *"`#[non_exhaustive]` is already on
   the public types precisely to leave room for it."* Designing for additive change on an unpublished
   crate is the same instinct. (Minor factual note: `Event` does **not** carry the attribute —
   `event.rs:181-183`. It has private fields and a builder, which is a stronger form of the same
   property, so the effect holds and the letter does not.)

**Recommendation:** replace *"publishing freezes the public API"* with *"publishing 0.1 starts the
feedback loop"*, move a `0.1.0-alpha` ahead of the projection freeze, and let `0.2` break. That is
the same insight the release-hazard correction reached, applied to the phase ordering it did not
revisit.

---

## 6. CLAUDE.md as an operating document

Accurate today except as noted; **substantially wrong the moment the rename lands** (§1.1), with
constraint 2 inverting into an actively harmful instruction. Three further points:

1. **Constraint 5 loses ADR-0004's nuance.** `CLAUDE.md:61` — "No let-chains. Stable only from 1.88;
   the MSRV is 1.85." ADR-0004's provisional note says the MSRV *"can be raised at any time at zero
   cost to anyone. Do not treat it as a hard limit."* CLAUDE.md states the constraint without the
   caveat, so a future session reads it as immovable. Given `append.rs:101-102` already contains a
   comment explaining a `match` written to avoid a let-chain, this constraint is actively shaping
   code on the strength of a promise made to nobody. Restate the caveat where the constraint lives.
2. **The open-questions list should be deleted, not updated.** `CLAUDE.md:110-115` already says the
   RUNBOOK ledger is the source of truth. Keeping four hand-maintained summaries beside a canonical
   ledger guarantees drift — and one of the four is already wrong (§1.1). Keep the pointer, drop the
   summaries.
3. **Everything else in it is good and should not change.** The audience note ("do not explain event
   sourcing; do explain why a newtype"), the "an adapter that has not run the suite is not an
   adapter" rule, and the "never assert on literal position values" rule — the last of which is
   restated in CONTRIBUTING and honoured in the suite (`suite.rs`'s `positions_of` compares against
   assigned positions, never literals). That rule is doing real work.

---

## What is right — do not relitigate

- **The conformance suite as a published artefact.** `testkit/src/lib.rs:3-6` and
  `tests/memory_conformance.rs:3-7` — validating the suite against the reference store *in both
  directions* ("a rule that no correct store can pass is worse than no rule at all") is the best idea
  in the project and the README is right to lead with it.
- **ADR-0002 kept verbatim, with the reason stated** (ADR-0005:53-60). Rare and correct.
- **ADR-0001's "Alternatives rejected"** — four alternatives, each killed on a specific technical
  ground. This is the template the other ADRs should match.
- **ADR-0004's provisional note** — the correct application of the unpublished-API insight, written
  before the RUNBOOK articulated it.
- **`Tags` canonical ordering** as a documented contract guarantee (`tag.rs:137-145`) with the
  adapter-indexing consequence named *and* property-tested with a comment explaining why the
  idempotence matters (`properties.rs:33-40`). Nothing to add.
- **`collect()`'s `# Errors`** honestly documenting that partial results are discarded
  (`store.rs:158-161`), plus "Do not use it to replay an entire log."
- **The stub crates carrying real design notes.** README's claim about this (`:72-74`) is true, and
  `happenstance-sync/src/lib.rs:24-53` in particular is a model of writing down what you do not know.
- **The RUNBOOK's session protocol and per-phase exit gates**, and *"'It compiles' is not an exit
  criterion anywhere in this document."*
- **Phase 1's honest prediction** (`:154-157`) that
  `racing_conditional_appends_elect_one_winner` will be the rule that hurts. That is what a good
  runbook does.
- **`.cargo/config.toml`'s comment** explaining that without the alias, the command three documents
  call "the whole gate" fails outright. Exactly the right kind of comment.
