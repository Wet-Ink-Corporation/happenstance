# Pre-publication review — the five release-bound crates

- **Date:** 2026-09-03
- **Pinned to:** `56ef6c5ddc3476a9a896dfe1824f4c46de33a3ed`
- **Produced by:** a multi-agent review commissioned for the 0.2.0 release —
  28 scoped finder dimensions and six second-pass instruments, every finding
  through a reproduce-veto and an adversarial settled/remediation pass, and six
  measurements built under [`experiments/`](../../experiments/).
- **Lifecycle:** immutable evidence. Superseded rather than edited — a later
  review that disagrees with an entry writes a superseding document. The one
  permitted in-place change is repointing a `file:line` citation at the text it
  already named ([`README.md:228-231`](README.md)).

## What this document does not do

**Nothing here is fixed.** No entry is repaired by a line edit, and the commit
that adds this document touches no path under `crates/**` or `spec/**`.

It writes **no ADR**, amends **no clause**, and changes **no frontmatter**.
Where an entry implies a decision it names the decision and its owner and stops.
[`.kb/decisions/`](../../.kb/decisions/) and [`RUNBOOK.md`](../../RUNBOOK.md)
settle; this document routes. That separation is the one this repository already
keeps between the pass that discovers and the pass that decides
([`repairing-a-frozen-clause-without-amending-it.md`](../../.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md)).

It also states no ratio over the mutation registry, per
[ADR-0010](../../.kb/decisions/0010-the-suite-must-prove-itself.md): the
denominator is an author's choice. Where the suite is measured, the measurement
names the store and the rule, not a fraction.

## The baseline this was taken against

`cargo xtask ci` was run whole at the pinned commit and is **green, exit 0**,
with all four `OPTIONAL` steps *run* rather than skipped: the feature powerset,
the wasm32 feature powerset, `cargo deny` licences and advisories, and the
nightly `--cfg docsrs` build. So no entry below is a thing the gate would have
caught. Every one of them lives in the space above a fully green gate, and
several are about that space being smaller than it looks.

## How to read an entry

Every entry carries the same labelled fields, in this order. An entry missing
the routing is not an entry.

| Field | What it is for |
| --- | --- |
| **Clause** | The clause ID it bears on **and that clause's maturity marker** — or an explicit statement that no clause governs, which is itself a result |
| **Examined** | What was looked at, as `path:line` at the pinned commit |
| **Found** | What the code does, quoted rather than paraphrased |
| **Why a defect** | The discriminator between a defect and a preference, stated rather than left to the reader |
| **Semver** | breaking / additive / none — and what 0.2.0 costs, because three crates are already live |
| **Remediation** | Deliberately **subordinate** to the clause and the routing: the judgement about whether a fix is right is the most recessive thing in an entry, not its conclusion |
| **Routing** | Where it goes next. Never *"fix it"* |

## Why the semver column is the spine

`happenstance`, `happenstance-core` and `happenstance-testkit` are **already on
crates.io at `0.2.0-alpha.1`** — verified against the registry, not read off the
README. `0.2.0` adds `happenstance-sqlite`, and `happenstance-cloudflare` is
publish-ready and held back.

`0.2.0-alpha.1` is a pre-release, so no `^0.2` requirement resolves to it. That
makes a breaking change to the three published crates **nearly free today and
permanent the day 0.2.0 ships**. Several entries below are cheap now and
impossible later, and that asymmetry — not their severity in isolation — is what
orders this document.

## Verdict

This is a strong codebase, and the reason this document is long is not that it is a weak one. Eighty-nine entries came out of twenty-eight review dimensions, six differently-instrumented second passes and six measurements, against a workspace whose gate is green with every optional step running. Almost none of what follows is the kind of defect a gate can hold. That is the finding underneath all the others.

The three things worth saying before the list.

**The governance corpus works, and it is the reason the findings are the shape they are.** Repeatedly the right answer was already written down — in a clause's `Rejects:` line, in an ADR's own falsifier, in a module doc naming the defect one file before the file that has it. VT-23's `Rejects:` describes `X-1` verbatim, written before the code. ADR-0022 §9 and §11 each wrote the re-open trigger that this review's measurements then fired. `projection_store.rs:98` calls the asymmetry *"the defect"* while the event store one file away has it. The corpus is not decorative and it is not stale; what it lacks is anything that reads it back against the code on a schedule. Fifteen entries corroborate a recorded open question rather than discovering something new — that is not duplication, it is the ledger telling you which deferred things are now costing.

**The distinctive risk is not bugs. It is claims.** The largest single theme is twelve rendered surfaces that state something a commit, an accepted ADR or the registry already made false — and the gate cannot see any of it, because prose is not type-checked and `spec-trace` verifies that a citation *resolves*, not that it says what the citing sentence claims. This review found an instance of that class in the document it modelled its own format on, and made the same class of error four times itself, which is recorded in *What this review got wrong*.

**Three crates are already public, and that is the clock.** `0.2.0-alpha.1` is a pre-release, so no `^0.2` requirement resolves to it. Every breaking change to `happenstance`, `happenstance-core` and `happenstance-testkit` is close to free today and permanent the day `0.2.0` ships. Nothing else in this document is as time-dependent as that, and it is why the list below is ordered by *when a fix stops being available* rather than by severity.

### What must be settled before `0.2.0`, in order

**First — the shapes that cannot be changed afterwards.** These are breaking on a published crate and cost nothing today.

1. **`Y-1`** — `commit` cannot express "correct, and nothing to record", and the crate's own `given/when/then` DSL passes the decision that produces the error. `Committed::position` has to become optional or gain a sibling outcome. The library already answers this question correctly one file away, in `runner.rs:499-519`.
2. **`P-2`** — nothing ties `DomainEvent::tags` to `DecisionModel::scope`, so a model whose scope is not covered by its own events' tags folds empty forever and its append condition never fires. Two of the crate's own rendered examples teach the violation.
3. **`B-2`, `B-4`+`B-5`, `AE-3`** — the root glob mounts `happenstance-core`'s semver-exempt projection surface at `happenstance::`; the tuple `Boundary` silently requires one shared `Event` type and documents the arity ceiling instead; `after_opt` rewrites every per-guard boundary `and_guard` set, with nothing at any layer noticing call order.
4. **`AE-4`** — `Event::metadata` is a fourth unbounded payload with no floor and no `StoreLimit` variant, so no adapter has a conformant way to refuse one. Its fix is additive *by luck* — three separate declarations happened to go the right way — and the conformance rule that would check it is a testkit minor every adapter takes involuntarily, so the floor and the rule want the same release.
5. **`L2-02`'s declension** — the tolerance that lets a fixture say "this refusal was contention, not a defect" has to land on `Fixture` as a defaulted associated const, because `ConcurrentFixture` has a blanket impl. `happenstance-testkit` is already live.

**Second — correctness in the crate `0.2.0` introduces.** Available at the same price later, but they are the first public behaviour of the adapter this specification exists to certify.

6. **`X-3`** — `row.rs:105,116` decode positions with `unsigned_abs()`, so a negative value maps onto its positive twin and forges a duplicate position and a duplicate `EventId`. The correct conversion is already written, one file away, under a doc comment explaining why negatives must be *rejected*.
7. **`X-1`** — the query plan is chunked by UNION arms and never by bound parameters, at two sites, one inside `BEGIN IMMEDIATE`. Measured: 400 items × 128 tags is 51,600 parameters against SQLite's 32,766, and the crate's own public `planned_statement_count` calls that plan valid.
8. **`J-2`** and **`J-3`+`J-4`** — `append` stalls a `current_thread` reactor for 886 ms where the sibling in the same crate costs 27.5 ms, while the crates.io README says the opposite; and `#[derive(Clone)]` publishes connection sharing the struct's own doc disclaims, with poisoning making that sharing permanent. The `Clone` half is breaking and free only until the crate ships.

**Third — pages that are wrong while serving.** Cheap, and each is read by someone deciding whether to depend on this.

9. `happenstance-testkit`'s README says **"No adapter has run this suite"** — on crates.io, now. `happenstance-sqlite`'s front page says it carries `publish = false`. `store.rs:153` still promises the laziness ADR-0011 retracted. And `RUNBOOK.md:156-163` calls phases 6–9 `not started`, which routes the next contributor — by the repository's own session protocol — to redo work that is done.

**Fourth — the instruments, because everything above is only as good as they are.**

10. **`gate-vacuity`** — `cargo test --list` output is byte-identical with and without `#[ignore]`, so `proof.rs:29`'s central argument does not hold: all 31 named proof tests can be silenced and the gate runs 33 steps and exits 0, with four of nine artefacts running zero tests.
11. **`L1-1`, `L2-01`, `WA-1..3`** — measured, four deliberately-wrong stores pass all 89 event-store rules failing nothing. No rule composes forwards `from` with `limit`, which is the read a projection runner issues every tick.

### What is right, and should not be touched

Said explicitly, because a document this long otherwise reads as a verdict on the whole.

The two-flavour port derivation and everything ADR-0001 protects; the `!Send` path; `unsafe_code = "forbid"` and the `Unpin` discipline that follows from it. The error taxonomy — typed source chains, `#[non_exhaustive]` throughout, `Display` carrying values rather than categories — which was the best-executed dimension in the review and produced almost nothing. The command loop's four positional arguments and `Retry` having no `Default`. Tuple composition. The mutation registry as an idea, and the refusal to quote a ratio over it. The decision to keep the projection port gated. The `experiments/` pattern, which this review used six times and which made three of its own findings retract.

### What this review got wrong

Recorded in full in *Corrections*, and summarised here because a review that self-corrects and does not say so teaches the reader to trust it more than it has earned. Three entries carried a `0.2.0` deadline against a crate that does not ship at `0.2.0` — the sections knew, the theme layer did not, so the error entered at aggregation. A stale-count finding carried a stale count. A remedy this review proposed was falsified by the measurement it asked for. Two prior-art citations handed to a section were wrong and the section dropped them. And two agents were run against one experiment directory because a relaunch was issued against work that had not failed.

Three measurements refuted part of what the findings that commissioned them predicted. That is the design working, not the design failing.

---

## The measurements

Six experiments were built for this review, under the house pattern
(`experiments/append-condition/`'s shape: an empty `[workspace]` table, a
`run.sh` that re-derives `results/` from clean and never becomes a gate step
per CF-34, conformance established before any clock, and `results/raw/` kept
whole because a raw file someone edited is not raw).

| experiment | the one question it answers | what it found |
| --- | --- | --- |
| [`gate-vacuity`](../../experiments/gate-vacuity/) | how much of the gate can be silenced without it going red? | all 31 named proof tests; four of nine artefacts then run zero tests and `cargo xtask ci` exits 0 |
| [`suite-against-wrong-adapters`](../../experiments/suite-against-wrong-adapters/) | do the wrong stores this review named pass the suite? | four of four fail nothing across all 89 event-store rules |
| [`event-clone-allocations`](../../experiments/event-clone-allocations/) | is an `Event` clone two allocations or `t + 2`? | `t + 2` exactly — 66 against a `from_static` control's 1, at the 64-tag floor |
| [`shipped-append-condition-sql`](../../experiments/shipped-append-condition-sql/) | what does the chain the adapter actually emits cost? | it loses to the aggregate that justified it, 9 of 9 cells; the documented tag ordering costs 38–44x |
| [`one-connection-latency`](../../experiments/one-connection-latency/) | how long does an `append` stall the reactor? | 885.8 ms against a same-crate control's 27.5 ms; the obvious fix leaves a 635 ms residual |
| [`busy-timeout-margin`](../../experiments/busy-timeout-margin/) | what is the margin on the only liveness bound in the system? | 1.3x–1.4x on the plateau; `busy > 0` observed at the shipped contender count |

Three of the six refuted or qualified the finding that commissioned them. Each
says so in its own `results/`, and the entries above carry the refutation rather
than the prediction.

**Read every figure with its experiment's stated limits.** Neither latency
experiment measures the whole adapter; `busy-timeout-margin` measures no
`happenstance-sqlite` code at all; every `wait_ms` is a lower bound; the
allocation counts are counts and requested bytes, not resident memory; and all
of it is one machine, one run per cell, on a loaded host. The claims are ratios
and orders of magnitude, never third significant figures.

---

# What this review did not open

Repository: `D:\repos\happenstance` @ `56ef6c5`. Date: 2026-09-03.

Twenty-eight finder dimensions and six second-pass instruments produced ~112 canonical
entries. The dimensions were scoped **by file set**, which is what made them productive
and is also what bounds them: a dimension scoped to `crates/happenstance-sqlite/src/`
finds everything wrong in that directory and nothing that is a property of the workspace,
of the release, or of the world outside `git`.

This section is the census of that boundary. It follows the method of
`references/evaluation/review-blind-spots.md` (2026-08-06): every absence below is a
**counted** absence across `findings-index.md` and `sections/*.md`, not an impression.

## The census

Counts are `grep -oiF <term>` over the review's own output. `idx` = `findings-index.md`
(all 112 entries), `sec` = the eleven written sections.

| Term | idx | sec | Repository state at `56ef6c5` |
|---|---|---|---|
| `permissions` (as a GH Actions key) | 0 | 1 | `.github/workflows/ci.yml` has **no `permissions:` key at any level** |
| `GITHUB_TOKEN` / `SHA-pin` / `supply chain` | 0 | 0 | six actions, all mutable refs; `dtolnay/rust-toolchain@master` at `ci.yml:312` |
| `release workflow` / `cargo owner` / `OIDC` / `trusted publishing` / `yank` | 0 | 0 | `.github/workflows/` contains exactly one file |
| root `README.md:<line>` cited as a finding | 0 | 0 | 291 lines, two rule counts, the GitHub front door |
| `tracing` / `metric` / `telemetry` | 1 | 3 | `grep -rn tracing --include=*.rs crates/ xtask/ examples/` → **1 hit**, a string literal in `xtask/src/lints.rs:1161`. Zero manifest entries |
| `public dependency` / `lockstep` / `pub use happenstance_core;` | 0 | 0 | `pub use happenstance_core;` appears in **no** crate |
| `compile time` / `build time` as a cost borne by an adapter author | 0 | 0 | testkit `src/` is 14,608 LOC; one macro call expands 112 rules |
| `two runners` / `lease` / `single writer` (projections) | 0 | 0 | `runner.rs` has no ownership concept; `spec:5512` names the hazard |
| `disintegrate` / `cqrs-es` / `competitor` | 0 | 0 | named in `README.md:255` |
| `loom` / `miri` / `fuzz` | 0 | 0 | still absent, as at 2026-08-06 |

Three areas the 2026-08-06 census opened are now **closed and should not be re-reported**:
`SECURITY.md` exists; `RecordedAt` exists (`identity.rs:149`); retention, tenancy and
truncation are specified (ES-39/CF-27, `spec:457`, `spec:1384`). Migration is real in both
SQLite stores (`event_store.rs:191`, `projection_store.rs:122`), each with its own
`SCHEMA_VERSION` and a `tests/migration.rs`. `unwrap_used = "deny"` and `todo = "deny"` are
live at `Cargo.toml:166-190`. This repository closes its blind spots; that is why the
remaining ones are worth naming precisely.

---

## Ranked by what it costs to be wrong

### 1. The workflow that will publish four crates has never been reviewed as an artefact

**The question nobody asked:** what can the CI workflow do, and who decides what it runs?

**Evidence it was never asked.** `ci.yml` is cited seven times across the sections and
every citation quotes a *comment* in it as prose to compare against something else
(`published-surfaces…:73`, `:202`). Zero entries treat the file as a thing under review.
Concretely, at `56ef6c5`: no `permissions:` key at workflow or job level, so every job
runs with the repository-default `GITHUB_TOKEN` scope; six actions on mutable refs
(`actions/checkout@v4`, `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`,
`actions/setup-node@v4`, `obi1kenobi/cargo-semver-checks-action@v2`, and
`dtolnay/rust-toolchain@master` at `:312` — a *branch*); and `taiki-e/install-action`
downloads prebuilt `cargo-hack`, `cargo-deny` and `wasm-bindgen-cli` binaries and runs
them over the tree on all three matrix runners. All of this was reported on 2026-08-06
(F1) and none of it moved in 652 commits.

**Why it matters at 0.2.0.** This is the release where the repository stops being a
private artefact. Four crate names on crates.io, one maintainer, one long-lived token
somewhere: the blast radius of a compromised transitive action is every downstream
consumer, and it is the one defect class in this document that cannot be fixed by a later
release. There is also no release workflow at all — `RUNBOOK.md:4700` carries
`cargo publish --dry-run` as a manual checklist line, so the publish ordering across four
interdependent crates (core → testkit → happenstance → sqlite, with registry propagation
between each) is a procedure that exists only in a person's head and has never been
rehearsed.

**Cost to close: cheap.** `permissions: contents: read` at workflow level; SHA-pin all six
actions with a `# v4.2.2` comment; `@master` → a tag. The release workflow is a day and
should use crates.io Trusted Publishing (OIDC) rather than a stored token — writing it
with OIDC now is free, retrofitting it is not.

### 2. `happenstance-core` is a public dependency of three published crates, and no crate re-exports it

**The question nobody asked:** when a consumer needs to *name* the contract, where does the
name come from?

**Evidence.** `grep -rn "pub use happenstance_core;" crates/*/src/lib.rs` → nothing.
`happenstance/src/lib.rs:242` glob-re-exports core's *items* (`pub use happenstance_core::*;`,
which B-2 attacks for a different reason), and `happenstance-sqlite/src/lib.rs` carries
zero `pub use` at all (D-1 says so). Nothing in the review turns D-1's own argument on the
workspace's own crate: `idx`/`sec` counts for "public dependency" and "lockstep" are 0/0.

**Why it matters.** D-1's reasoning is exactly right and stops one crate short. A consumer
writing `fn ingest<S: EventStore>(store: S)` over both `happenstance::Event` and
`SqliteEventStore` must add `happenstance-core` to their own manifest and pick a version.
All five publishable crates take `version.workspace = true` today (`Cargo.toml:15`,
`happenstance-testkit/Cargo.toml:21` being the one with its own key at the same value), and
all depend on core at the pinned `0.2.0-alpha.1`. The day core goes to `0.3`, a consumer on
`happenstance = "0.3"` and `happenstance-sqlite = "0.2"` gets two `Event` types that print
identically — the E0308 the review already names, sourced from the workspace's own crate
rather than from `rusqlite`. And the lockstep is undecided: nothing states whether the five
versions move together, which is the question `happenstance-testkit`'s separate version key
half-answers without saying so.

**Cost to close: cheap and additive.** `pub use happenstance_core;` in `happenstance` and in
each adapter, plus one sentence per crate stating whether core's version is part of that
crate's semver contract. Both are free today and neither is free after 0.2.0.

### 3. The repository's front door was reviewed by nobody, and the gate's own lint does not read it

**The question nobody asked:** are the claims on `README.md` true?

**Evidence.** Zero of the 112 entries cite `README.md:<line>` at the repository root. The
review's docs dimension (O) covered `docs/*.md` and the per-crate READMEs; V-4 and P-6
mention the root README only to observe it has no link *into* `docs/`. Meanwhile
`xtask/src/lints.rs:1017-1022` defines `COUNT_BEARING_DOCS` as exactly four files —
`crates/happenstance-testkit/README.md`, `crates/happenstance-testkit/src/lib.rs`,
`crates/happenstance-core/src/lib.rs`, `crates/happenstance-core/Cargo.toml` — and the root
README is not one of them, nor is `crates/happenstance/README.md` or
`crates/happenstance-sqlite/README.md`. The root README carries two rule counts anyway
(`:99` "112 rules across four families", `:232` "There are 112 rules in four families. The
**event-store** family (89)"). Worse, the check's predicate is
`!counts.iter().any(|(n, _)| *n == count)` — it accepts a claim if the number matches *any*
family's count, so "the event-store family (112)" and "112 rules in four families" are both
green regardless of which is which.

This is one level below S-2, which found `lint-rule-counts` silently dropped from the story
grain. Nobody asked what the lint asserts when it *does* run, or which documents it reads.

**Why it matters.** The root README is the first artefact every evaluator opens and the one
crates.io does not render. It is also where the strongest true claims live unstated —
`unsafe_code = "forbid"` is set workspace-wide at `Cargo.toml:160` and opted into by every
member, and appears nowhere a user reads (the same observation as 2026-08-06 F2, still open).

**Cost to close: trivial.** Add the three READMEs to `COUNT_BEARING_DOCS`; tighten the
predicate to bind a claim to the family its paragraph names. Then read the file.

### 4. Observability is absent from the library, the specification and this review

**The question nobody asked:** how does an operator see what this library is doing?

**Evidence.** One `tracing` occurrence in the whole workspace's Rust, and it is a string
literal in a lint message (`xtask/src/lints.rs:1161`). No manifest declares `tracing` or
`log`, optional or otherwise. The specification's 21 hits for `observab` are all the phrase
"observable behaviour" in the conformance sense — a different word. The one place the review
touches it is inside R-2, as a sub-clause: "`run_projection` has no callback, channel or
`tracing` instrumentation" (`consumer-api-shape.md:291`). It was noticed once, at the
narrowest possible scope, and never elevated to a question about the library.

**Why it matters.** `happenstance-sqlite` ships at 0.2.0 as a production storage adapter,
and this document's own `sqlite-under-load` theme rests on facts — a 5,000 ms busy timeout,
a mutex held across `BEGIN IMMEDIATE`, `SQLITE_BUSY` counts that L2-02 says *no thread in the
tree can report* — that an operator can observe only by inference. There is also a shape
constraint specific to this port that an adapter author will get wrong unaided:
`EventStore::read` is not `async` and returns the stream at the top level (ES-13), so
`#[tracing::instrument]` on `read` opens a span that closes before a single event is read.
The attribute macro is the obvious reach and it measures nothing.

**Cost to close: cheap for the part that matters.** The contract cannot take `tracing` — it
is `no_std`-capable (`lib.rs:112`, `extern crate alloc` at `:115`) with four dependencies.
What it *can* ship is a semantic-conventions section in prose: span and field names, and one
policy sentence — `ConditionViolated` is the routine DCB outcome and belongs at `DEBUG`, not
`ERROR`. Zero API surface, zero dependencies, and it is the difference between two adapters
producing comparable traces and two adapters producing incomparable ones. Recommended in
2026-08-06 (E1); not actioned; not re-asked.

### 5. Nobody measured what the conformance suite costs the person it is aimed at

**The question nobody asked:** how long does a stranger's first `cargo test` take?

**Evidence.** Six measurements were commissioned (`gate-vacuity`,
`suite-against-wrong-adapters`, `event-clone-allocations`, `shipped-append-condition-sql`,
`one-connection-latency`, `busy-timeout-margin`) and three more are queued in
`pending-measurements.txt`. Every one prices either a *cost the library imposes at runtime*
or *whether an instrument discriminates*. None prices the adapter author's experience.
"compile time" / "build time" as a cost: 0 idx, 0 sec.

**Why it matters.** The testkit is the moat, its `src/` is 14,608 lines, and
`event_store_conformance!(MyFixture::new())` expands 112 rules across four families into one
test binary in a stranger's crate. C2-05 already establishes that the testkit forces four
`happenstance-core` features onto that build with no way to decline — so the adapter's own
`src/` compiles against a strictly larger core under `cargo test` than under `cargo build`,
twice. If that first `cargo test` takes ten minutes, the moat has a moat.

**Cost to close: an afternoon.** `cargo build --timings` on
`examples/outside-projection-adapter` — the crate that already exists to stand where a
stranger stands — plus a cold-cache wall-clock number for `cargo test` on
`happenstance-sqlite`. Publish the number next to the rule count.

### 6. Every fact about the published crates came from inside the repository

**The question nobody asked:** what does the outside world currently see?

**Evidence.** `BRIEF.md:18-21` states the three live versions were "verified against the
registry" — that is the only external observation in the entire review, and it is a version
string. Nothing checked: whether docs.rs *built* `0.2.0-alpha.1` (the gate runs a local
nightly `--cfg docsrs` build, which is a different machine, a different feature set and a
different sysroot); whether the five `[package.metadata.docs.rs]` blocks produce the pages
their comments claim (`happenstance-cloudflare/Cargo.toml:142-163` is target-overridden and
is the one most likely to fail); whether
`https://github.com/Wet-Ink-Corporation/happenstance` resolves for a stranger, which every
crate's `repository` key and the `SECURITY.md` reporting link depend on; and who owns the
four names on crates.io.

**Why it matters.** `CONTRIBUTING.md:56` and `ci.yml:51-52` both say it plainly: docs.rs
builds *after* publication and a release can be yanked but never edited. A `docsrs` build
that failed for the alpha is currently the whole public documentation of three crates, and
nobody looked.

**Cost to close: minutes.** Open five docs.rs pages and the repository URL.

### 7. Two live projection runners on one `ProjectionId` are guarded in one direction only

**The question nobody asked:** what stops two runners applying the same events?

**Evidence.** `crates/happenstance/src/runner.rs` has no lease, lock, ownership token or
single-writer statement — `grep -i "lease\|single writer\|one runner"` over it returns
nothing. The specification *names the scenario*: PS-22's `Rejects:` paragraph
(`spec:5510-5518`) describes "an old runner pod has not yet exited, two runners share an id"
— and guards only the **backwards** half, the stale runner dragging the checkpoint down.
Two runners both advancing forwards never trip `CheckpointRegression`; they interleave, and
`projection.rs:28-30` offers idempotence as an escape hatch rather than requiring it. None of
the 44 PS clauses covers it. Review counts: 0/0.

**Why it matters.** The scenario is a rolling redeploy, which is the normal way software is
released. The failure is a silently double-applied read model with a monotonic checkpoint —
no error, and the correct-looking checkpoint is the evidence that nothing went wrong.

**Cost to close: cheap as a sentence, expensive as a mechanism.** One sentence on
`run_projection` stating that exactly one runner per `(store, ProjectionId)` is the caller's
obligation, plus a RUNBOOK row. The mechanism is a real design question and belongs with the
projection-store freeze, not with 0.2.0.

---

## Ranked separately: things I could not evidence

- **Whether the roadmap's ordering is right.** The 2026-08-06 §7 argued that
  local-first-that-syncs is the position nobody else occupies, and `happenstance-sync` is
  still `publish = false` with `todo!()` bodies. No dimension asked. But "the roadmap is
  wrong" is a judgement and I have no instrument for it, so it is listed and not argued.
- **Whether the six measurements' harnesses are themselves correct.** `experiments/` holds
  thirteen directories, is deliberately out of the gate (`CLAUDE.md`), and is cited by zero
  findings as a subject. I-1 and I-3 attack ADR-0022's *numbers*, which came from there — so
  the harnesses were reached obliquely and never audited. I did not audit them either.

---

## Audit of the review's own method

**Where scoping created a seam.** Three facts were filed by three agents each, which is a
healthy sign that the dedupe worked and an unhealthy sign about the scoping. "Three of four
SQLite port methods run rusqlite on the executor thread" arrived as F2-1 (async/streams in
adapters), I-4 (sqlite performance) and J-2 (concurrency); two were excluded. CF-24's orphan
scanner arrived as L2-05 and M-4. The doc-hidden emitters arrived as C1-01 and C2-03. The
seam that stayed *empty*, by contrast, is the one between dimensions: the CI workflow sat
between "the `xtask` gate itself" (S, which owns `xtask/`) and "publish rehearsal" (U, which
owns `cargo package`), and each could reasonably have assumed the other held it. Nobody did.
The same is true of the root README, between "docs and the page standard" (O) and
"a blindfolded DX pass".

**Entries likely to be wrong.**

- **`themes.md` and `findings-index.md` disagree about L3-02's own numbers.** The index says
  "the table it documents holds 41 / 39"; `themes.md:98` says "the table holds 42 / 38". The
  truth at `mutation_coverage.rs:2380-2490` is **42 `Agreed`, 38 `Rejected`** — themes is
  right, the index is wrong. Both are then comparing the wrong quantity: the doc's claim is
  "23 Agreed … which leaves twenty-one misses", so the correct restatement is 42 Agreed and
  **40** misses (42 minus the two conformant controls), not 42/38. A finding whose content is
  "a count went stale" carries a stale count in two places.
- **`themes.md` says "all 90 rules"; five section instances say 89; `README.md:232` says the
  event-store family is 89 of 112.** One number in the headline theme is wrong.
- **Two sections state contradictory registry facts about `happenstance-cloudflare`.**
  `cost-numbers-measured-something-else.md:144` says it "**is** live at `0.2.0-alpha.1` as a
  release-bound crate"; `frozen-clauses-unmet-by-shipping-adapters.md:44` says it "declares no
  `publish = false` and **is not on the registry**". `SECURITY.md:29-31` and `ci.yml:384-385`
  settle it: cloudflare is publish-ready, deferred past `0.2.0`, and the only thing on
  crates.io under its name is `0.0.0`. The `cost-numbers` sentence is false.
- **A severity that does not match its stated outcome.** Following from the above:
  `frozen-clauses-unmet-by-shipping-adapters` is tagged `[must-settle-before-0.2.0]` and its
  headline finding Q-01 is against a crate that does not ship at 0.2.0; the
  `foreign-types…` theme's own headline calls cloudflare one of "the two crates joining the
  publishable set", which it is not. The *sections* get this right and say so; the theme
  layer does not. D-4, G-2 and Q-01 have a real deadline and it is `0.3.0`. Nothing else
  changes — they are still free now and not free later — but a launch checklist that lists
  them under 0.2.0 will spend the window on the wrong crate.

**Did "do not re-litigate the governance corpus" suppress anything?** Mostly no, and
visibly so: fifteen entries are tagged `open-question-corroborated` or
`decision-may-have-drifted`, which is the instruction working. One thing it did suppress.
The brief's precedence ladder ends at `references/evaluation/*` — "dated evidence, never a
rule" — and the practical effect was that nobody re-read
`references/evaluation/review-blind-spots.md` to ask which of its nine open questions were
closed by 652 commits and which were not. Three were not: observability (E1), the CI
`permissions` block and SHA-pinning (F1), and the README guarantees section (F2). All three
are absences, so no finder scoped to a file could have found them, and the one document that
had already found them was ranked as evidence rather than read as a checklist. That is not a
flaw in the ladder; it is a gap in the intake, and the fix is one line in the next brief:
*read the previous completeness pass first, and say for each item whether it closed.*

---

# Corrections the review made to itself

These were found by the completeness critic auditing this review's own output,
and verified independently before adoption. They are recorded rather than
silently applied, because a review that corrects itself and does not say so
teaches the reader to trust it more than it has earned.

## C-1 — three entries carried the wrong deadline

`Q-01`, `D-4` and `G-2` were tagged **must-settle-before-0.2.0**. All three are
against `happenstance-cloudflare`, which **does not ship at 0.2.0**:

- `SECURITY.md:29-32` — "`happenstance-cloudflare` is finished and packaged but
  deliberately not in the `0.2.0` release, so the only thing under its name on
  crates.io is a `0.0.0` placeholder with no functionality."
- `README.md:101` — "publish-ready, held back from `0.2.0`".
- `.github/workflows/ci.yml:384` — "publish-ready but deferred past `0.2.0`".

Their real deadline is whenever that crate ships. **The individual sections knew
this and said so; the theme layer above them did not**, which is the more useful
half of the finding: the error entered at the aggregation step, not at the
evidence step. Corrected to `must-settle-before-cloudflare-ships`.

## C-2 — a stale-count finding carrying a stale count

`L3-02` reports a doc comment stating a property its table does not have.
`findings-index.md` renders the table as 41/39; `themes.md` renders it 42/38.
The tree says **42/38** — and both comparisons are of the wrong quantity: the
correct restatement is **42 Agreed / 40 misses**. The entry is right about the
defect and wrong in the same way as its subject. Corrected in the section, and
the coincidence is left visible rather than tidied away.

## C-3 — "90 rules" where the enumeration holds 89

`themes.md` says "all 90 rules". Five sections independently say 89, and
`grep -c '^    pub async fn ' crates/happenstance-testkit/src/suite.rs` is
**89** at the pinned commit. The theme text is corrected. Note this is the same
class as the defect `stated_rule_counts` exists to catch, committed by the
review rather than by the repository.

## C-4 — two sections disagreed about the registry

One section wrote that `happenstance-cloudflare` is on crates.io; another that
it is not. `SECURITY.md:29` and `ci.yml:384` settle it: it is not, beyond a
`0.0.0` placeholder. The section carrying the error is corrected.

## C-5 — a remedy this review proposed was falsified by the measurement it asked for

`I-2` proposed binding `position > ?` into the guard's seed arm.
`experiments/shipped-append-condition-sql/` measured it at **+2.0%** with no
consistent sign across nine cells, and `EXPLAIN QUERY PLAN` shows why: the
chained subquery is uncorrelated, so the seed is the probe side rather than the
driving table. The finding's *fact* survives; its *fix* does not, and the entry
says so rather than dropping the finding.

## C-6 — a prior-art citation handed to a section was wrong

The intake cited `references/evaluation/PRESSURE-TEST.md:177-179` and
`review-blind-spots.md:626-631` as prior art for X-1. Both were read at the
pinned commit and are about unrelated matters. They are not cited. VT-23's own
`Rejects:` clause turned out to be the real prior art — and it describes the
defect verbatim, written before the code.

## C-7 — "exhausted" where the raw says "busy"

One measurement report described `rate-n64-r2` as carrying *"3 exhausted
attempts"*. The raw rows say otherwise:

    ...race_us_max=5522382  committed=5  rejected=314  busy=1  failed=0  exhausted=0
    ...race_us_max=5502321  committed=5  rejected=313  busy=2  failed=0  exhausted=0

`busy=1` and `busy=2` — three attempts that **entered** SQLite's busy handler —
and `exhausted=0` on both. No attempt ran the 5,000 ms budget out. The two
figures mean different things and only one of them happened.

The substantive claim survives and is the one to quote: **`busy > 0` was
observed at the shipped `CONTENDERS = 64`**, which is the first nonzero busy
count anywhere in this tree and is what ADR-0022 §11's falsifier — *"re-open if
any run ever reports busy > 0"* — asks for. The margin claim (1.31x–1.38x)
is unaffected; the timeout was approached, not exhausted.

Caught by reading `results/raw/rate-n64-r2.txt` rather than the summary, which
is the reason `results/raw/` is kept whole.

## C-8 — two agents were run against one experiment directory

`experiments/busy-timeout-margin/` was worked by two agents at once, because a
relaunch was issued against an agent that had not in fact failed. One `rm -rf
results/raw` destroyed a completed run mid-copy and two further runs were
corrupted before a lock was added.

The final artifact was verified independently afterwards — 28 raw files, 53
per-contender wait vectors, every file the `results/README.md` declares present,
conformance green under both handlers, and no leaked absolute-path directory
from the `HS_WAITS_DIR` bug the agent found and fixed. The two agents converged
on the same numbers, which is corroboration rather than agreement by
construction.

It is recorded because the review's own method note should not omit the one
place its orchestration was wrong.

---

# The entries

## Eight shapes in the crate a consumer installs are wrong, and no clause governs six of them

`spec/SPECIFICATION.md:102-107` puts the typed layer outside the clause space deliberately: this is the specification "for happenstance — the project, not the crate of that name, which is the typed layer and one consumer of what follows." That is right for a facade and it is no longer what `happenstance` is. It owns the codec framing region, the command loop's retry, boundary composition and the projection runner, and it is the crate `cargo add` reaches for. Nothing but rustdoc prose holds those four surfaces, and the eight entries below are places where the prose and the code disagree — two of which (an empty decision, and a model whose scope misses its own events' tags) compile, pass the crate's own `given/when/then` DSL, and fail only once the events are durable. Five need a breaking change to a crate already on crates.io; `0.2.0-alpha.1` is a pre-release, which no `^0.2` requirement resolves to, so all five are free today and permanent the day `0.2.0` ships. The theme brief priced six as breaking: the sixth is `chunk`, which sits behind `unstable-projection` and carries the semver exemption ADR-0036 records as accepted, and that correction is made in the last entry.

---

### Y-1 — `commit` has no representation for "correct, and nothing to record", and the crate's own DSL passes the decision that produces it

**Clause.** **ES-20** — *An empty batch is refused, and refused first* (`spec/SPECIFICATION.md:3542-3547`). **`[FROZEN]`** (`spec/SPECIFICATION.md:3549`; rule-table row at `:9097`). ES-20 governs the *store* and is correct. No clause governs what the typed layer does with an empty decision — `spec/SPECIFICATION.md:102-107` puts `happenstance` outside the clause space.

**Examined.** `crates/happenstance/src/command.rs:305-312`, its `# Errors` block at `:205-217`, the `Append` variant at `:111-118`, `Committed` at `:79-81`; `crates/happenstance-core/src/error.rs:219-225`; the testing DSL at `crates/happenstance/src/testing/mod.rs:220-238` and `:292-296`.

**Found.** `commit_with` puts whatever `decide` returned straight into the batch:

```
        let decided = decide(&model).map_err(CommandError::Refused)?;
        let batch = encode::<B::Event, C, S::Error, D>(&decided, codec)?;
```

`encode` (`command.rs:342`) begins `let mut batch = Vec::with_capacity(decided.len());` and its loop never runs, so an empty decision reaches `store.append(&batch, …)` as `&[]`. ES-20 requires the store to refuse it, and the caller receives `CommandError::Append`, whose own doc reads `/// The store failed the append for its own reasons.` (`command.rs:111`) over an inner variant documented `/// This is a caller bug, not a store failure.` (`error.rs:222-223`). The rendered chain is `appending the decided events failed` → `an append must contain at least one event`. `commit`'s `# Errors` list names `CommandError::Append` only as "if the store fails the append for its own reasons" (`command.rs:216-217`) — the empty decision is not among the six causes it enumerates.

The DSL disagrees. `Given::when` reads, folds, calls `decide`, and stops — `Ok(emitted) => Outcome::Emitted(emitted)` (`testing/mod.rs:223`); the decided events are never appended anywhere in `when`'s body (`testing/mod.rs:180-238`). `then` then passes:

```
            Outcome::Emitted(actual) if actual.as_slice() == expected => {}
```

so `.then(&[])` is green. `then`'s own doc says it exists to prevent the neighbouring silent pass — "comparing a refusal against `&[]` and passing is the silent pass this method exists to prevent" (`testing/mod.rs:285-286`) — and the module header claims production fidelity: "what stops a test from selecting a different set than production does" (`testing/mod.rs:67-68`).

**Why a defect.** The discriminator is that the idempotent no-op is not an edge case in this domain; it is the ordinary second call of every command a DCB handler guards. A developer writing "subscribe: if the student already holds a seat, emit nothing" has a green test, a runtime error, and an error message that tells them the store rejected their batch. The library has one answer to this question already, one file away and in the opposite direction: the projection runner refuses to commit an empty write set, with the reason stated — "Nothing was applied, so there is nothing to make durable and no checkpoint to move. Committing an empty batch here would claim the run had considered a position it never reached." (`runner.rs:509-511`). The command loop and the runner give opposite answers to the same question inside one crate.

**Semver.** Breaking. `Committed::position` is `pub position: SequencePosition` (`command.rs:81`) and any representation of "committed nothing" has to make it optional or add a second success shape. `0.2.0-alpha.1` is a pre-release and no `^0.2` requirement resolves to it, so the change costs nothing today and is permanent after `0.2.0`.

**Remediation.** Subordinate to the decision. The shape the crate already contains is `runner.rs:499-519`'s: short-circuit before `append`, and report the no-op in the success value rather than through an error variant documented as a caller bug.

**Routing.** A decision nobody has taken: *does an empty decision commit, refuse, or become a third outcome, and what does `Committed` look like afterwards?* It has no number in the RUNBOOK's ADR queue (`RUNBOOK.md:262`) — phase 7's ADR-0020 and ADR-0021 are both closed — and it is a phase 12 exit-criterion item because it is breaking (`RUNBOOK.md:4681`).

---

### P-2 — a model sees its own writes only if `DomainEvent::tags` covers `DecisionModel::scope`, and two of the crate's own rendered examples do not

**Clause.** None. `spec/SPECIFICATION.md:102-107` places the typed layer outside the clause space, so the only governing authority is **ADR-0020** ([`kb-decision-0020`](.kb/decisions/0020-fold-query-agreement.md), *accepted*), which closed the identical class of defect for event *types*.

**Examined.** `crates/happenstance/src/domain.rs:78-79` and `:172-178`; `crates/happenstance/src/boundary.rs:112-118` and `:139`; `crates/happenstance-core/src/query.rs:112-115`; the rendered examples at `crates/happenstance/src/domain.rs:141` with `:163-164`, and `crates/happenstance/src/composition.rs:132` with `:150-151`; `crates/happenstance/src/testing/mod.rs:377-392`.

**Found.** The tag set is declared twice, independently. `crates/happenstance/src/domain.rs:78-79`:

```
    /// The tags this event carries.
    fn tags(&self) -> Tags;
```

`crates/happenstance/src/domain.rs:172-178`:

```
    /// The tags every event inside this boundary carries.
    …
    fn scope(&self) -> &Tags;
```

The query is derived from the second (`boundary.rs:118`, `derive_query(M::Event::EVENT_TYPES, self.scope())`) and matched against the first, by superset (`query.rs:114`, `type_ok && tags.contains_all(&self.tags)`). Nothing relates them. `assert_domain_event`, the crate's one stated remedy for a non-compiler-enforceable agreement, checks `E::EVENT_TYPES.contains(&carried)` (`testing/mod.rs:381`) and never reads `tags()`.

Two rendered pages teach the violation. `DecisionModel`'s own example gives `Seat` the impl `fn tags(&self) -> Tags { Tags::empty() }` (`domain.rs:141`) and then scopes the model `let scope = Tags::from_pairs([("course", "c1")])?;` (`domain.rs:163`). The composition page does the same, twice: `domain.rs`-shaped events at `composition.rs:132`, and `let capacity = Counter { scope: of("course", "c1")?, seen: 0 };` / `let student = Counter { scope: of("student", "s1")?, seen: 0 };` at `composition.rs:150-151`. Both doctests pass, because both assert on `query()?.items().len()` and never fold against a store.

**Why a defect.** ADR-0020's own context names the corrupting direction and the tag half falls on it exactly: "A fold interpreting a type the query never selected narrows the log the decision is made on: the append condition protects less than the handler assumes, and that direction corrupts." A model whose scope its events do not carry reads an empty fold on every attempt, and the append condition built from that same query matches nothing — the condition is present, evaluated, and unconditional. `Query`'s own doc names that outcome and seals the *other* route to it: "An `AppendCondition` built on a query with no items is a condition nothing can ever violate — a conditional append that is silently unconditional, which is a lost update with no diagnostic anywhere" (`query.rs:126-129`). The zero-item route is structurally unreachable; the tag-mismatch route reaches the same state and is wide open.

The instrument exists and nobody is pointed at it. `Given::event` seeds with `.with_tags(event.tags())` (`testing/mod.rs:157`), so a mismatch renders as an empty `selected by the model's query:` region and a populated `seeded but NOT selected:` region — but only for an author who already wrote a test that seeds and expects the fold to see it, and the message does not name the cause. The worked example gets it right (`examples/transfers-on-sqlite/src/main.rs:294-298` carries the account tag), which is what makes this a documentation-and-check gap rather than a misunderstanding.

**Semver.** A check inside `commit_with` is behaviour-breaking: a program that used to run and silently protect nothing would return an error. Free while `0.2.0-alpha.1` is the only published version.

**Remediation.** Subordinate. Both halves are in hand at exactly one place — inside `commit_with`, between `decide` returning (`command.rs:305`) and `append` (`command.rs:311`), where `query` is already bound (`command.rs:288`) — and `Query::matches` is the predicate `Boundary::absorb` already asks of an event coming back (`boundary.rs:139`). No second filter vocabulary is needed.

**Routing.** The decision is *whether `tags`/`scope` agreement is checked, documented, or left to the author*, and it is the unclosed half of ADR-0020's question. ADR-0020 is `status: accepted` and therefore immutable; this needs a new number in the RUNBOOK's ADR queue (`RUNBOOK.md:262`). The two rendered examples are a separate, non-breaking route through the documentation pass.

---

### B-1 — `Codec` is an advertised extension point and tag resolution is a closed three-arm match

**Clause.** None; the governing authority is **ADR-0032** ([`kb-decision-0032`](.kb/decisions/0032-adr-0021-serde-attribution-correction.md), *accepted*, superseding ADR-0021), which carries forward the framing region and states what it is for.

**Examined.** `crates/happenstance/src/codec.rs:24-26`, `:258-270`, `:324-336`, `:339-366`; `crates/happenstance/src/command.rs:234`.

**Found.** The invitation, `codec.rs:24-26`:

```
/// The trait is **not sealed**: a codec of your own is a legitimate thing to
/// write, which is why this is a trait rather than an enum of the three below.
/// An enum would have been shorter and would have forbidden it.
```

`commit_with`'s page is titled `/// The command loop, with a codec of your own.` (`command.rs:234`). Writing frames the codec's `TAG` unconditionally (`frame::<C>`, `codec.rs:258-270`). Reading resolves a foreign tag through `decode_by_tag`, which is a fixed chain — `if tag == <Json as Codec>::TAG`, `<Postcard as Codec>::TAG`, `<Cbor as Codec>::TAG` — and then:

```
    Err(CodecError::UnknownTag {
        tag: tag.to_owned().into_boxed_str(),
    })
```

There is no registration seam. `decode_by_tag` and `decode_event` are private and `pub(crate)` respectively (`codec.rs:339`, `:324`), and the only public door — `Boundary::absorb<C: Codec>` — takes one codec.

**Why a defect.** ADR-0032 states the property the framing region exists to deliver: "a build with only `postcard` enabled must still read a tag written by a build with only `json`". That property holds across the three built-in codecs and is structurally unreachable for anything the "not sealed" sentence produces. ADR-0021's summary reasoned about precisely this failure for *untagged* events and closed it with a fallback, "because refusing it would make every event written before the typed layer existed unreadable with no legal repair", and left `UnknownTag` "meaning exactly one thing: a tag was written and this build cannot honour it". For a third-party codec that meaning is permanent: no build can ever become able to honour the tag. An application that adopts a custom codec, runs for months and then adds `Json` gets `CodecError::UnknownTag` from `Boundary::absorb` on every historical event, which is an empty fold and an append condition matching nothing — the P-2 failure arriving by a second road.

**Semver.** The defect is breaking for the user's data; the fix need not be. A defaulted method on `Codec` that offers the codec the chance to resolve a foreign tag is additive on a published trait. This is one of the two entries in this section that can land after `0.2.0` without becoming permanent.

**Remediation.** Subordinate, and the finder's own proposed shape was refuted in review: generalising `C: Codec` to a codec-set bound is breaking on three public signatures and needs a blanket impl that overlaps any set type unless coherence can prove otherwise. The narrower observation is that writing picks one codec and reading tries several, so the seam belongs on `Codec` rather than in the three signatures.

**Routing.** *Does `Codec` stay unsealed, and if it does, what reads a tag it did not write?* ADR-0021 and its repair ADR-0032 are both closed and immutable; this is a new number in the ADR queue. If the answer is that `Codec` should have been sealed, that decision is breaking and belongs before `0.2.0`, not after.

---

### B-4 / B-5 — the tuple `Boundary` silently requires one shared `Event` type, and the rationale defending that associated type states the semver rule backwards

**Clause.** None. Adjacent and worth naming: [`kb-open-question-disjoint-boundaries-no-clause-001`](.kb/open-questions/disjoint-boundaries-have-no-clause.md) records that the DCB independence proposition — commands sharing no consistency boundary do not conflict — is enforced by a live conformance rule and stated by no clause. This entry is the typed layer's version of the same silence: the composition the rule proves the *store* supports is the composition the *tuple impl* refuses.

**Examined.** `crates/happenstance/src/composition.rs:26-41`, the rendered doc block at `:96-157`, the module's only stated limit at `:8-10`; `crates/happenstance/src/boundary.rs:70-75`, `:112`; `crates/happenstance/src/sealed.rs:1-19`; `crates/happenstance/src/lib.rs:199`.

**Found.** The macro binds every member after the first to the first's event type, twice — on `Sealed` and on `Boundary`:

```
            $($rest: $crate::Boundary<
                Event = <$first as $crate::Boundary>::Event,
            >,)+
```

(`composition.rs:29-31` and `:39-41`). The arity-2 doc block explains the union, the non-rollback on a mid-fold `CodecError`, the rejected `compose!` macro and the ceiling of 8; it never mentions `Event`, and its worked example composes two `Counter`s over one `Seat` enum (`composition.rs:139-155`). The module's only stated limit is "The ceiling is **8**" (`composition.rs:8`).

The defence of the same associated type, `boundary.rs:70-75`:

```
    /// The one domain enum every member of this boundary folds.
    ///
    /// Present from birth rather than added later: the trait is sealed and the
    /// crate publishes from it, and growing a sealed trait a required item is
    /// a breaking change no downstream crate could have prepared for.
    type Event: DomainEvent;
```

That is the opposite of the truth here. `mod sealed;` at `lib.rs:199` carries no `pub`, so `Sealed` (`sealed.rs:17`) is unnameable downstream — "Nothing else can implement it, because nothing else can name it" (`sealed.rs:12`) — and every `Boundary` impl in existence is in this crate: the blanket at `boundary.rs:112` and seven `impl_boundary_for_tuple!` invocations. Growing `Boundary` a required item compiles in every downstream crate untouched. Being able to grow the trait freely is the principal thing sealing buys.

**Why a defect.** The failure is the motivating DCB pairing: a `Seats` model over `Enrolment` and a `Wallet` model over `Payment`, two bounded contexts checked by one append. `(seats, wallet)` does not typecheck, and the developer gets a where-clause mismatch on `Event` from inside a macro expansion, at a call site, with the composition page in front of them naming only the arity cap. Their options are to merge two bounded contexts into one enum or to drop to `happenstance-core`. The page documents the half that costs nothing — the ceiling of 8, which is a rendering decision — and not the half that is permanent. The second half of the entry is what makes the first hard to repair: a maintainer applying `boundary.rs:72-74`'s rule after `0.2.0` declines a required addition to `Boundary` that is in fact free, and a reviewer citing it blocks the relaxation this entry is about.

**Semver.** Relaxing the where-clause is breaking on a published crate; correcting the doc sentence is not. The refutation pass established the constraint is not load-bearing where it is written: `<B as Boundary>::Event` is *used* at `command.rs:229`, `:277`, `:306` and `testing/mod.rs:183`, `:186`, all of them the write path — the `decide` closure's return type and `encode` — and the read path (`Boundary::absorb`, `boundary.rs:121-148`) never names it. That is evidence about where the constraint belongs; it is not a verdict, because deleting the bound leaves `type Event = <$first as Boundary>::Event` (`composition.rs:43`) meaning something narrower than a heterogeneous tuple's caller would expect from `decide`.

**Remediation.** Subordinate. A `compile_fail` doctest beside the arity-2 impl, in the shape `boundary.rs:53-84` already uses for the seal, pins whichever answer is taken. That half is non-breaking and independent of the decision.

**Routing.** Two decisions, one owner. *Does a tuple boundary admit members over different domain enums, and what is `Boundary::Event` then?* — a new ADR number, and it must be settled before `0.2.0` because the relaxation is breaking. And *what is the sealed-trait evolution rule for this crate?* — currently stated wrongly in a doc comment and stated nowhere else; searches of `standards/rust/13-sealing-and-exhaustiveness.md`, `.kb/decisions/` and `spec/SPECIFICATION.md` return no atom on it.

---

### B-2 — the root glob mounts `happenstance-core`'s unstable projection surface at `happenstance::`, and the xtask assertion that guards that promise reads the line three above it

**Clause.** **PS-3** — *Until PS-2's bar is met the port SHOULD ship behind an off-by-default `unstable-projection` feature, with a documented exemption from semver* (`spec/SPECIFICATION.md:4880-4881`). **`[PROVISIONAL]`** (`:4882-4884`). Its stated rule is `cargo hack --feature-powerset` in `cargo xtask ci` (`:4885-4886`). Corroborates **ADR-0036** ([`kb-decision-0036`](.kb/decisions/0036-the-projection-port-ships-gated.md), *accepted*), whose accepted cost is "a consumer must name a feature to get a projection store at all".

**Examined.** `crates/happenstance/src/lib.rs:238-242`; `crates/happenstance-core/src/lib.rs:160-183`; `crates/happenstance-testkit/Cargo.toml:50-55`; `crates/happenstance/Cargo.toml:70`, `:98-112`; `crates/happenstance/src/lib.rs:140-143`; `xtask/src/main.rs:1544-1581`, `:882-892`.

**Found.** Three lines apart in the crate root:

```
#[cfg(feature = "unstable-projection")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]
pub use runner::{Progressed, Projection, ProjectionError, run_projection};

pub use happenstance_core::*;
```

(`lib.rs:238-242`). The glob re-exports whatever the *compiled* contract crate exposes, and core's projection items are gated on **core's** features (`happenstance-core/src/lib.rs:160-165`, `:167-169`, `:175-181`), not on this crate's. `happenstance-testkit` turns them on unconditionally, as a normal dependency:

```
happenstance-core = { workspace = true, features = [
  "std",
  "memory",
  "conformance",
  "unstable-projection",
] }
```

(`happenstance-testkit/Cargo.toml:50-55`; `conformance = ["unstable-projection"]` at `happenstance-core/Cargo.toml:97` implies it a second time). `happenstance` itself dev-depends on the testkit (`happenstance/Cargo.toml:70`), and its own crate root tells a consumer to do the same: "add `happenstance-testkit` as a dev-dependency" (`lib.rs:141-142`). With `happenstance`'s own `unstable-projection` off, `happenstance::Checkpoint`, `::ProjectionId`, `::ProjectionStore`, `::SendProjectionStore`, `::Authority`, `::CommitError`, `::ResetError`, `::ProjectionProbe` and — with `memory` — `::MemoryProjectionStore` all resolve in `cargo test`.

The manifest promises the opposite: "**Off by default**, and the name is the promise … these four items make no semver promise at all. A reader has to type the word `unstable` before any of them is in their build." (`happenstance/Cargo.toml:98-102`).

**Why a defect.** The gate step that would have to catch this is named and does not. `the_typed_layer_makes_no_promise_it_does_not_keep` (`xtask/src/main.rs:1544`) reads `crates/happenstance/src/lib.rs` and asserts on the literal `pub use runner::{` behind the cfg pair — `lib.rs:239` — and never inspects `lib.rs:242`. Its own comment states what it forbids: "a feature table advertising a runner nobody can name, or four `pub use`s a consumer cannot turn on" (`:1540-1542`). Neither direction covers a glob mounting a *fifth through thirteenth* item nobody turned on. PS-3's own rule, `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:884-889`), cannot see it either: `--no-dev-deps` rewrites every manifest with dev-dependencies removed, and the dev-dependency is the trigger. The user-visible outcome is an auto-import: `happenstance::Checkpoint` offered by rust-analyzer, accepted into library code, compiling under `cargo test` and failing under `cargo build`.

The second half is independent of the projection port. `pub use happenstance_core::*;` makes every future addition to the contract crate an addition to `happenstance`'s surface without review, and a name collision between a future `happenstance` item and a future core item is a hard break in a crate that did not change.

**Semver.** Breaking, in the narrow sense that anyone who reached a leaked path loses it. The leaked items themselves carry ADR-0036's documented exemption, so the honest reading is that this is free now and stays cheap; the glob's second half is a standing semver hazard on the stable surface and is free to close only while `0.2.0-alpha.1` is the published version.

**Remediation.** Subordinate: an explicit, `#[cfg]`-carrying re-export list in place of the glob, which is also what makes `cargo public-api` diffs mean anything for this crate.

**Routing.** Not a decision — PS-3 and ADR-0036 have already taken it, and the code does not implement what they took. This routes to the phase 12 release checklist (`RUNBOOK.md:4681`) as a discharge item, and to whoever owns `xtask/src/main.rs:1544`, because an assertion that names the promise and inspects the wrong line is the more expensive half.

---

### AE-3 — `after_opt` silently discards every per-guard boundary `and_guard` set, and only the safe call order is ever written down

**Clause.** **VT-30** — *An `AppendCondition` is one or more guards, each with its own boundary* (`spec/SPECIFICATION.md:1861-1867`). **`[PROVISIONAL]`** (`:1869-1872`, falsified if no adapter can push a multi-guard condition into a single statement without one self-join per guard, or if the `min()` collapse measures as immaterial; the Postgres adapter and the benchmark harness are the two named instruments and both are unbuilt).

**Examined.** `crates/happenstance-core/src/append.rs:155-176`, `:185-212`, the doc example at `:86-101`; every call site of `and_guard` in the workspace.

**Found.** `crates/happenstance-core/src/append.rs:201-212`:

```
    pub fn after_opt(self, position: Option<SequencePosition>) -> Self {
        let guards = Vec::from(self.guards)
            .into_iter()
            .map(|guard| Guard {
                query: guard.query,
                after: position,
            })
```

The `map` reconstructs every `Guard` with the new position, discarding `guard.after` without reading it. Both methods are `#[must_use]` chainable builders returning `Self` on the same type, so `AppendCondition::new(q1).and_guard(q2, Some(p2)).after(p1)` compiles and silently drops `p2`. `after_opt`'s doc explains why the scope is blanket — "Applying to every guard is what keeps a single-guard condition … behaving exactly as it always has. For per-guard boundaries, use [`and_guard`](Self::and_guard)." (`append.rs:197-199`) — and does not say that a later `after` erases what `and_guard` set. `and_guard`'s doc (`append.rs:155-169`) does not say it either.

The type's own doc example writes only the safe order:

```
/// let condition = AppendCondition::new(busy)
///     .after_opt(SequencePosition::new(9_000))
///     .and_guard(quiet, SequencePosition::new(12));
```

(`append.rs:92-94`), and so does every other call site in the tree — `crates/happenstance-core/tests/wire.rs:320`, `:896`; `crates/happenstance-sqlite/tests/append.rs:475-476`; `crates/happenstance-testkit/src/suite.rs:5052`, `:5071`, `:5228`. The hazardous order is exercised nowhere, so no test pins the behaviour and a future edit that changed it would be green.

**Why a defect.** VT-30 mandates the blanket application, so the behaviour is required and this entry does not ask for it to change. The defect is that the dangerous direction is the natural one: guards first, boundary last, reading `new(q1).and_guard(q2, p2).and_guard(q3, p3).after(p1)`. A decision model's busiest fragment usually carries the highest position, so the trailing `after` *raises* the quiet guards' boundaries and widens what the condition tolerates. Every event matching those fragments between their own boundary and `p1` loses the ability to violate the condition, so an append that should have been refused is admitted — a lost update, which is the failure VT-30's own `Rejects:` clause was written against (`spec/SPECIFICATION.md:1877-1882`). Nothing at any layer notices call order: not the type, not clippy, not the conformance suite, which only ever receives the assembled condition.

**Semver.** Renaming `after_opt` to carry its scope in the name is breaking on a published `happenstance-core` item, and free only while `0.2.0-alpha.1` stands. Adding a unit test that pins the current behaviour is neither, and is available today.

**Remediation.** Subordinate, and it is two halves that should not be merged: pinning the behaviour costs nothing and can land immediately; a name or a shape that makes the overwrite visible at the call site is the breaking half.

**Routing.** The pass that lifts VT-30's `[PROVISIONAL]` marker owns the question *does `after_opt` keep blanket scope under a name that says so?* Its two named instruments are the Postgres adapter (phase 10, `RUNBOOK.md:4544`) and the benchmark harness. ADR-0012 discharged VT-30 and is accepted and immutable, so this needs a number the queue does not currently allocate (`RUNBOOK.md:262`).

---

### AE-5 — `EventParts` derives `Debug` one method call from an `Event` whose hand-written `Debug` redacts the payload for exactly that reason

**Clause.** None; VT-33 (`spec/SPECIFICATION.md:1272-1278`, `[FROZEN]`) fixes the standard-library trait surface and does not reach `Debug` on `EventParts`. The governing authority is the constitution atom **RS-12-5** — *Treat `Debug` as API: render a payload's size, never its bytes* (`standards/rust/12-manual-impls-and-derive-traps.md:231`).

**Examined.** `crates/happenstance-core/src/event.rs:420-427`, `:429-445`, `:447-470`; `standards/rust/12-manual-impls-and-derive-traps.md:231-259`.

**Found.** `crates/happenstance-core/src/event.rs:434-445`:

```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EventParts {
    /// The event's type.
    pub event_type: EventType,
    /// The opaque payload.
    pub data: Bytes,
    …
    pub metadata: Option<Bytes>,
}
```

Twelve lines below it, the impl whose stated reason applies verbatim to the struct above:

```
impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Payloads are frequently large and rarely UTF-8; printing the length
        // keeps test failures legible.
        /// Renders a payload as its length rather than its contents.
        struct ByteLen(Option<usize>);
```

(`event.rs:447-452`), rendering `<{len} bytes>`. `bytes::Bytes` (1.12.1 in `Cargo.lock`) has no redacting `Debug` of its own, so `{:?}` on an event redacts the payload and `{:?}` on `event.into_parts()` prints it. Neither doc notes the difference, and `into_parts` names its audience: "This exists for the code that genuinely does own an event: callers, and the wire encoders in the typed layer." (`event.rs:412-413`) — the code most likely to be logging around a decode failure.

**Why a defect.** This is a named violation of a rule this repository wrote, not a preference. RS-12-5's compiled **Do** example *is* `Event`'s `Debug`, asserting `rendered.contains("<22 bytes>")` and `!rendered.contains("hunter2")` (`standards/rust/12-manual-impls-and-derive-traps.md:239-247`); its **Not** example is a struct holding a raw byte payload under `#[derive(Debug)]` (`:253-259`), which is `EventParts` field for field. The atom also names the gate step that cannot catch it: `missing_debug_implementations` is a workspace warning under `-D warnings`, "so every public type acquires a `Debug` whether or not anyone chose one" (`:233-235`). The derive satisfies the lint and defeats the rule. Within the precedence ladder a constitution atom outranks `CLAUDE.md` and prose, and within an atom the compiled example beats everything.

**Semver.** None. Removing `Debug` from a derive and hand-writing it changes rendered output, not the trait surface. This one is free at any time, which is exactly why it should not wait for a version that is not free.

**Remediation.** Subordinate: hoist `ByteLen` out of `Event::fmt`'s body — it is a nested item at `event.rs:452` reachable by nothing else — and give `EventParts` a field-for-field identical impl.

**Routing.** Not a decision; a discharge. It routes to whoever owns `standards/rust/12-manual-impls-and-derive-traps.md`'s **Evidence** block (`:225-228`), which currently cites three sites in `happenstance-core` and not this one, and to a unit test in `event.rs` asserting the parts render bounded, which is the instrument the atom's own shape implies.

---

### R-2 / Y-5 — the projection runner is unobservable between chunk commits and its one throughput knob carries no vocabulary, and both are semver-exempt

**Clause.** **PS-3** — the port SHOULD ship behind `unstable-projection` "with a documented exemption from semver" (`spec/SPECIFICATION.md:4880-4881`, **`[PROVISIONAL]`**). **ADR-0036** ([`kb-decision-0036`](.kb/decisions/0036-the-projection-port-ships-gated.md), *accepted*) states the accepted cost in terms: "`cargo-semver-checks` will not police the surface."

**Examined.** `crates/happenstance/src/runner.rs:102-126`, `:288-345`, `:409`, `:416-427`, `:460-527`; `crates/happenstance/src/command.rs:18-60`, `:530`; `crates/happenstance/src/lib.rs:238-240`; `crates/happenstance/Cargo.toml:98-112`; `examples/transfers-on-sqlite/src/main.rs:86-90`.

**Found.** The whole of `chunk`'s specification is its declaration:

```
    chunk: NonZeroUsize,
```

(`runner.rs:421`). The runner's page is 57 lines of prose (`runner.rs:288-345`) covering the flavour it binds, why it streams, why there is no failure policy, why there is one projection per call and how to rebuild; `chunk` appears in it only as the unit of the buffer — "at-most-`chunk` events between one `begin` and its `commit`, which is a bounded window rather than the log" (`runner.rs:319-320`). The doctest supplies a number and no reason: `run_projection(&events, &models, &mut sold, &Json, 64.try_into()?)` (`runner.rs:409`). Its sibling knob, the same kind of caller-supplied bound in the same crate, got a named `#[non_exhaustive]` type, two `const` constructors, a `compile_fail` fence and a doc paragraph arguing why a hidden default would be wrong (`command.rs:18-60`), plus a source-reading test `retry_has_no_default` (`command.rs:530`).

Observability: `run_projection` has no callback, channel or `tracing` instrumentation — `grep -rn "tracing::" crates/happenstance/src crates/happenstance-sqlite/src` returns nothing. The only externally observable state between start and the next chunk commit is the durable checkpoint row, reachable only by a caller who independently opens a second connection to poll `ProjectionStore::checkpoint`, which the API neither documents nor steers toward. The type meant to carry progress says so itself:

```
/// A named struct rather than a `(Option<SequencePosition>, usize)` tuple,
/// because the arity is exactly what a later observability pass wants to grow
/// and a tuple freezes it.
```

(`runner.rs:104-106`).

**Why a defect.** The wrong outcome is one outcome with two causes. An operator kicks off a rebuild against a large log — the runner's own documented use case, "run the rebuild under a **second** [`ProjectionId`] and swap the reader over when it catches up" (`runner.rs:338-345`) — and the run is indistinguishable from a hang for its entire duration, with no built-in way to tell "850,000 of 1,000,000 applied" from "wedged". The knob that decides how often that silence is broken is the one parameter the crate gives no guidance on: a caller who copies `64` from the doctest has arrived there by copying, and a caller who reasons "fewer commits is faster" and writes `1_000_000` gets one open write set holding a million buffered statements against `SqliteProjectionStore`, whose `Batch` is an owned statement list under ADR-0017 — and the bounded-window claim at `runner.rs:319-320` is still true, which is the problem: it is bounded by a number the caller invented.

**Semver.** None, and this is the correction to the theme's pricing. `run_projection`, `Progressed` and `ProjectionError` are gated (`lib.rs:238-240`), the feature is off by default, and the manifest states the exemption: "these four items make no semver promise at all" (`happenstance/Cargo.toml:100-101`). ADR-0036 records that exemption as an accepted, stated cost. Giving `chunk` a named type after `0.2.0` is therefore free, and a `run_projection_observed` entry point is additive regardless. These are the two entries in this section that can wait, and the reason they can is documented rather than assumed.

**Remediation.** Subordinate. The crate already has the shape for the observation seam once — `commit` delegating to `commit_with` with a default chosen (`command.rs:219-232`, `:265-278`) — and the shape for the knob once, in `Retry`. Whether `chunk` should have a default is the one place the two diverge: `Retry` argues a hidden default would hide a worst case nobody wrote down, and a chunk size is correctness-neutral, so the argument does not transfer unexamined.

**Routing.** Phase 6's port-freeze pass owns both, because both are questions about the port's surface rather than about the runner's body, and PS-2's bar is not yet met (ADR-0036). Neither is a `0.2.0` gate. The `chunk` measurement — wall time, peak RSS and commit count against `SqliteProjectionStore` at chunk ∈ {1, 8, 64, 1024, 65536} — belongs in `experiments/`, out of the gate, which is where `references/evaluation/research-rust-api-guidelines.md:420-422` records this codebase reaching the opposite verdict on a `NonZeroUsize` parameter elsewhere.


---

## Three `[FROZEN]` clauses are unmet by the adapters that ship, and each crate already contains its own proof

A `[FROZEN]` marker binds the design; it does not assert that anything checks it. The knowledge base already says so in those words (`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`), about two clauses whose rules are unwritten. The four entries below are the next case of that class and a worse one: here the rules *are* written, they *are* mounted against both shipping adapters, and they pass — over a state one clause forbids outright (ES-18, Q-01), over a decoder that contradicts its own error type (VT-11 and VT-8, X-3), over a documentation MUST that a gate step is scoped by construction never to reach (ES-23, Q-02), and through a rule that no store in this workspace can fail (ES-22, F2-5). Nothing here needs a new instrument to be *found* — in every case the disproof is already in the tree, written by the adapter's own author: a public error variant with a passing `wasm32` test that demonstrates the forbidden state, a correct decoder one file away from the broken one, and a `DocumentationMust` table whose derivation rule says in prose that adapter obligations are out of scope. The asymmetry is one-shot rather than semver: no fix here is a breaking change, and every one of them is available at the same price after 0.2.0. What cannot be re-run is the release. `RUNBOOK.md:4720-4740` gives phase 12 exit criteria that audit every `[PROVISIONAL]` and every `[DEFERRED]` clause on a published surface, and no criterion at all that asks whether a `[FROZEN]` one is met by the adapter being published.

### Q-01 — the reference `!Send` adapter has a reachable, tested state in which a rejected append leaves a partial batch

**Clause.** **ES-18** — *Atomicity* (`spec/SPECIFICATION.md:3392-3397`). **`[FROZEN]`** (`spec/SPECIFICATION.md:3397`). The clause admits no exception and no adapter-declared escape: *"Either every event in the batch lands or none does. A rejected append MUST leave the store byte-identical."*

**Examined.** `crates/happenstance-cloudflare/src/event_store.rs:521-552` (`write_batch`), `:645-657` (`discard_from`), `:754-780` (the `PartialBatch` variant and its doc), `:2027-2060` (the executed `wasm32` test), and `:68-96` (the module's own account of why there is no transaction). The conformance mount at `crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64-68`, and the fault seam it depends on at `crates/happenstance-cloudflare/tests/support/mod.rs:244` and `:324-379`.

**Found.** There is no transaction. `write_rows` issues one `INSERT … RETURNING position` per event and `write_batch` compensates by hand (`event_store.rs:544-551`):

```rust
match self.discard_from(from) {
    Ok(()) => Err(cause),
    Err(while_discarding) => Err(CloudflareEventStoreError::PartialBatch {
        from,
        cause: Box::new(cause),
        while_discarding: Box::new(while_discarding),
    }),
}
```

The variant's own documentation states the outcome (`event_store.rs:754-768`):

```
/// An append failed part way through **and** the rows it had already written
/// could not be discarded, so this object still holds part of a batch that
/// never succeeded.
…
/// Every other failure of `append` leaves the log as it found it,
/// so a DCB command loop may re-read and retry. This one does not: the log
/// now contains events the caller's own failed append put there, and a
/// retry would decide against them.
///
/// It is reachable, not defensive — a storage ceiling reached mid-batch
/// fails the `INSERT` and then fails the `DELETE` that would undo it.
```

`a_batch_whose_discard_also_fails_reports_both_failures` (`:2027`) executes that state on `wasm32` in the gate and asserts the surviving rows: `assert_eq!(stored_positions(&sql), vec![…from…], "`from` names where the rows the object still holds begin")` (`:2045-2050`). `CHANGELOG.md:294-296` describes it as *"the one outcome of `append` after which a retry is unsafe."*

**Why a defect.** Not that compensation is the wrong mechanism — a Durable Object rejects transaction control through `sql.exec()`, and the module says so (`event_store.rs:91-96`). The defect is that a `[FROZEN]` clause with no exception clause is contradicted by a state the adapter documents, tests and names, and no clause, ADR, or open question records the exception. `grep -rn "PartialBatch" --include=*.md` at this commit returns three hits — two backlog artefacts and `CHANGELOG.md` — and none of them is a specification amendment. ADR-0023 (`.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`), the decision that owns this adapter's shim, does not mention atomicity. The suite cannot see it either, and the reason is exact rather than general: `append_is_atomic_under_a_mid_batch_fault` (`crates/happenstance-testkit/src/suite.rs:2706-2751`) arms one fault via `Fixture::arm_mid_batch_fault`, whose contract is *"The fault fires once"* (`crates/happenstance-testkit/src/contract.rs:295`), and the Cloudflare fixture implements it as a trigger `BEFORE INSERT ON event` (`tests/support/mod.rs:368`) — which a `DELETE` does not fire. The compensation therefore always succeeds under the only fault the suite can arm, and the branch that produces `PartialBatch` is unreachable from any conformance rule.

**Semver.** None. `happenstance-cloudflare` declares no `publish = false` and is not on the registry; phase 12's publish list (`RUNBOOK.md:4700-4704`) names four crates and not this one. Removing or restructuring `PartialBatch` is free today and free at 0.2.0. The one-shot cost is reputational: the adapter the two-flavour port design was paid for is the adapter that reaches a state the specification forbids.

**Remediation.** The shape an eventual decision might start from — and it is subordinate, because the decision here is which of two things gives way. `write_rows` already ends with a single statement covering the whole batch (`event_store.rs:610-621`), and a Durable Object's one atomicity primitive that needs no transaction control is that a single statement is backed out whole; the adapter's own test support already rests on that fact (`tests/support/mod.rs:359-362`, on `RAISE(ABORT)`). The rejected alternative is recorded in place: the module notes Workers *"offers a callback form instead, which would put a second seam in the constructor"* (`:93-94`).

**Routing.** A decision, not an edit. Two live options and this document takes neither: (a) restructure the write path so no compensating statement is load-bearing, and `PartialBatch` ceases to exist; or (b) amend ES-18 to name the exception a store without transaction control is permitted, with the conformance consequence stated. Either is an ADR through `/redkiln:kb-ingest`, and the owner is whoever owns the Cloudflare adapter at RUNBOOK phase 9 (`RUNBOOK.md:4375`) jointly with phase 12's release gate. The separable and smaller half — that `Fixture::arm_mid_batch_fault` can arm a fault in the write and not in the compensation — belongs to the testkit and is what makes the choice observable either way.

### X-3 — `happenstance-sqlite` absolutises every stored position, contradicting its own error type, and the crate already contains the correct decoder

**Clause.** **VT-11** — *Positions are unique, strictly increasing in assignment order, and may have gaps* (`spec/SPECIFICATION.md:1050-1058`), **`[FROZEN]`**; and **VT-8** — *`EventId` uniqueness is a store-level guarantee* (`spec/SPECIFICATION.md:902-907`), **`[FROZEN]`**. The nearest clause covering the *threat model* is **ES-38** (`spec/SPECIFICATION.md:4370-4378`), **`[FROZEN]`** — *"A store from which events have been removed by any means outside the port MUST continue to satisfy every clause of this section"* — but it is scoped to removal, so out-of-band *insertion or repair* is governed by no clause at all. That is a gap, and it is stated here rather than papered over.

**Examined.** `crates/happenstance-sqlite/src/row.rs:75-131` (`to_event` and its `# Errors` section), `crates/happenstance-sqlite/src/event_store.rs:884-888` (the error variant), `:608-676` (`evaluate`), `:1071-1078` (`head`), `:1240-1253` (`sample_ceiling`), `:200-228` (the schema). Against `crates/happenstance-sqlite/src/projection_store.rs:892-902` and `crates/happenstance-sqlite/tests/projection.rs:294-318`.

**Found.** Six sites decode a stored `i64` with `i64::unsigned_abs()`. The two in `row.rs` are the load-bearing pair (`:105-106`, `:116-117`):

```rust
let position = SequencePosition::new(position.unsigned_abs())
    .ok_or(SqliteEventStoreError::InvalidPosition(position))?;
…
let origin = SequencePosition::new(origin_position.unsigned_abs())
    .ok_or(SqliteEventStoreError::InvalidPosition(origin_position))?;
```

`SequencePosition::new` (`crates/happenstance-core/src/event.rs:247-252`) returns `None` only for zero, so `-3` becomes `Some(3)` and the `ok_or` can catch exactly one value. The variant it never reaches documents the opposite (`event_store.rs:884-888`):

```rust
/// A stored row carried a position SQLite accepted and the contract does
/// not: [`SequencePosition`] wraps a `NonZeroU64`, so zero and negatives
/// are unrepresentable.
#[error("stored position {0} is not a valid sequence position")]
InvalidPosition(i64),
```

The schema does not exclude the value: `position INTEGER PRIMARY KEY AUTOINCREMENT` (`event_store.rs:204`) with no `CHECK`, and `origin_position INTEGER` (`:210`) with no constraint but the `UNIQUE` pair. The four remaining sites are `head` (`:1077`), the read-ceiling sample (`:1250`), the guard's conflict position (`:671`) and the write path's return (`:749`). One file away, the projection store does it correctly (`projection_store.rs:897-902`), under a doc that states the rule the event store does not follow: *"Zero and negatives are rejected rather than resolved"* (`:894-896`).

**Why a defect.** The discriminator is not "unchecked conversion is unidiomatic" — it is that the function's own `# Errors` section (`row.rs:90-94`) promises `InvalidPosition` *"if a stored position is not representable"*, and the code makes every negative representable. Two named outcomes follow, and they are inconsistent with each other, which is the severe part. A row stored at `-3` is reported to readers at position `3`, with `EventId::new(store, 3)` (`row.rs:126-130`) — colliding with a real row at `3` on both VT-11's uniqueness and VT-8's. The same row is invisible to every append condition: `evaluate` compares the raw `i64` before absolutising (`event_store.rs:667-672`), so `highest = -3` against a boundary of `0` fails `highest > boundary` and the guard reports no violation. A DCB command loop therefore folds a fact into its decision model and appends against a guard that cannot see it. Who finds out, and when: the operator, after a migration script, a partial restore, a hand-written repair or a future replication ingest — none of which this adapter controls, all of which are ordinary for a file-backed store — and the symptom is a consistency-boundary violation with no error anywhere in the trace.

**Semver.** None, in either direction. `happenstance-sqlite` is not on the registry; it joins at 0.2.0. `InvalidPosition` already exists and already documents the corrected behaviour, so the fix changes no signature and adds no variant. It changes what a program that reads a corrupt file *does* — from a silent forged position to a documented error — which is the change the error type's doc already promises. This is the one entry in this section that ships silent data corruption in the crate 0.2.0 introduces.

**Remediation.** Recessive, because the crate has already made this decision once: `projection_store.rs:897-902` is `u64::try_from(stored).ok().and_then(SequencePosition::new).ok_or(…)`, and applying it at the six `unsigned_abs` sites is the application of an existing pattern rather than a new design.

**Routing.** No decision is owed on the decoder; it is a defect against the crate's own documented error contract, and it belongs to `happenstance-sqlite`'s owner at RUNBOOK phase 8 (`RUNBOOK.md:4232`) before phase 12. Two things *are* owed a decision and go elsewhere. First, the instrument: `crates/happenstance-sqlite/tests/projection.rs:294-318` is `a_corrupt_stored_position_is_reported_rather_than_defaulted`, which corrupts a checkpoint out of band through a raw connection and asserts the store reports rather than resolves it — the event store has the identical hazard, the identical error variant and no such test, and the fixture already holds `self.path` (`tests/support/mod.rs:241`) so a raw connection is one line away. Second, the clause gap: ES-38 covers removal and nothing covers out-of-band insertion or repair, which is a question for whoever holds ES-38's phase-14 owner slot, and this entry corroborates `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` — that the completeness axis is still unowned is still costing, now in a crate about to publish.

### Q-02 — ES-23 obliges each adapter to state what a dropped `append` does; neither shipping adapter states it, and the gate step is scoped never to ask

**Clause.** **ES-23** — *Cancellation outcome is unspecified, and that is the contract* (`spec/SPECIFICATION.md:3636-3643`). **`[FROZEN]`** (`spec/SPECIFICATION.md:3643`). The clause carries two MUSTs: *"The port MUST document this in an explicit `# Cancellation` section, and each adapter MUST state which of the two it does."*

**Examined.** `crates/happenstance-core/src/store.rs:194-213` (the port half, discharged), `xtask/src/lint_narrative.rs:1290-1354` (the derivation rule) and `:1418-1424` (ES-23's row), against the whole of `crates/happenstance-sqlite/` and `crates/happenstance-cloudflare/`, plus `docs/`, `references/adapter-shapes.md` and both crate READMEs.

**Found.** `grep -rn "# Cancellation"` over the tree returns exactly one source site: `crates/happenstance-core/src/store.rs:194`. Case-insensitive `cancel` over `happenstance-cloudflare` returns nothing at all; over `happenstance-sqlite` it returns two error-variant doc comments about a blocking *read* task — *"The blocking task carrying a query panicked or was cancelled"* (`event_store.rs:872`) — neither of which is about `append` and neither of which answers ES-23's question. The port re-states the obligation to the reader (`store.rs:203-205`):

```
/// What *is* promised is `# Atomicity`, which bounds what the silence can
/// cost: whichever the adapter does, the batch is applied in full or not at
/// all and never in part. Each adapter MUST state which of the two it does.
```

and then names, as *"the shape that looks cancellation-safe and is not"*, a pooled `rusqlite` adapter working in `spawn_blocking` (`store.rs:207-213`) — which is the shape `happenstance-sqlite` is closest to. The gate step that would have to catch this is `xtask/src/lint_narrative.rs`'s `FROZEN_DOC_MUSTS` table, and it cannot, by its own stated derivation rule (`:1322-1328`): *"A candidate is **pinned** when (a) its clause is `[FROZEN]`, (b) its obligation falls on **the contract's own documentation** — not on an adapter's, not on a fixture's … "*. ES-23 appears once in that table, `Pinned` to the port site (`:1418-1424`). VT-21, VT-22 and VT-24 — whose obligations are wholly a store's — are `Excluded` with the reason written out (`:1377-1395`). ES-23 carries both halves, and the table records only the discharged one, so the undischarged half is invisible to the reader of the array as well as to the gate.

**Why a defect.** Not a missing doc heading as a matter of taste. A caller reads `store.rs:194-213`, is told in terms that the outcome of a dropped future is unspecified and that the adapter will say which it does, goes to the adapter, and finds nothing. The safe reading is the pessimistic one, so they build compensation the adapter does not need; the convenient reading is that the silence means "safe", which is exactly the error the port doc names. Both are wrong and neither surfaces until an eviction, a client disconnect or a CPU limit drops a future in production. The answer both adapters owe is available and identical, which is what makes the silence expensive rather than merely incomplete: neither `append` body contains a single `.await` — `crates/happenstance-sqlite/src/event_store.rs:1018-1052` locks the connection and calls `append_locked` inline, and `crates/happenstance-cloudflare/src/event_store.rs:812-842` says so in a comment at `:828-829` (*"this whole body contains no `.await`"*). So for both, an `append` future polled once has already run to completion and cannot be cancelled; the statement is one paragraph, and it is the same paragraph after any future move of `append` into `spawn_blocking`, because a dropped `JoinHandle` does not cancel the closure either. This corroborates ADR-0012 in writing: `references/adr/0012-append-shape-and-preconditions.md:349-364` proposes exactly the gate step that would catch this, records that *"The step does not exist and has not been written"*, and names the fallback — *"the per-adapter review at phases 8–11"*. The step was never built and the phase 8 and 9 reviews did not produce the statement, so the primary instrument and its recorded fallback are both undischarged.

**Semver.** None as an API change; this is a documentation obligation on two crates, one of which is unpublished and one of which is not on the registry. But the content of the statement becomes a promise the moment it ships, and the promise is what is one-way: an adapter that has told the world "a dropped append cannot be cancelled" has constrained its own write path. 0.2.0 is where that promise starts, not where it gets cheaper.

**Remediation.** Subordinate, and smaller than the gap: a `# Cancellation` section in each adapter's crate-root or store-module docs, saying that the body suspends nowhere and that a polled future therefore runs to completion — which does not license a caller to treat a dropped future as evidence either way, because ES-23 forbids that regardless.

**Routing.** Two routes and they are different owners. The statement itself is the adapter authors' at RUNBOOK phases 8 and 9, owed before phase 12. Whether `lint_narrative`'s `FROZEN_DOC_MUSTS` should carry a second disposition for the adapter half of a two-halved clause — or whether ES-23's row should be `Excluded` with a reason, as VT-21's is — is a decision about the gate, and its owner is whoever owns `xtask`'s narrative pin (`.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/`). ADR-0012 already records the ceiling on any such step at `:845-849`: *"The gate check proposed in §5 catches a missing `# Cancellation` section and cannot catch a section that lies."* This document takes neither decision.

### F2-5 — ES-22's rule cannot fail against any store in this workspace, and one of its two arms has never executed

**Clause.** **ES-22** — *Dropping an `append` future leaves no partial batch* (`spec/SPECIFICATION.md:3608-3614`). **`[FROZEN]`** (`spec/SPECIFICATION.md:3614`).

**Examined.** `crates/happenstance-testkit/src/suite.rs:3110-3205` (the rule), `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1660-1729` (`YieldingRowAtATimeStore`), and the `append` bodies of every store that mounts `event_store_conformance!`: `crates/happenstance-sqlite/tests/conformance.rs:87` over `crates/happenstance-sqlite/src/event_store.rs:1018-1052`, and `crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64-68` over `crates/happenstance-cloudflare/src/event_store.rs:812-842`.

**Found.** The rule's own documentation states why the reference store cannot answer the question (`suite.rs:3128-3131`):

```
/// `MemoryEventStore` passes trivially, because its `append` body contains no
/// `.await` at all and the first poll runs it to completion — which is
/// precisely why the reference store cannot answer this question and a real
/// adapter must.
```

Both real adapters answer it the same way. Neither `append` body contains a suspension point — `happenstance-cloudflare`'s says so itself (`event_store.rs:828-829`), and `happenstance-sqlite`'s locks a `std::sync::Mutex` and calls a synchronous `rusqlite` path (`event_store.rs:1042-1052`). The rule polls once, the append completes, and `assert_appended(outcome)` (`suite.rs:3172`) accepts it. Consequently the rule's second arm has never run against anything: `landed == 0` (`suite.rs:3188-3196`) — the byte-identical `snapshot_of` comparison — is reachable only by a store that suspends inside `append` and has written nothing when it does. The one store in the tree that suspends is `YieldingRowAtATimeStore`, and it writes row one *before* its first `yield_once().await` (`mutants.rs:1709-1725`), so it lands on `landed == 1` and fails at `:3177` instead.

**Why a defect, and where it is weaker than its neighbours.** CLAUDE.md's first corollary — *"A rule that no adapter can fail is decorative"* — is discharged: the testkit's own tests carry the named wrong implementation. What is not discharged is the second, *"A port is only as well-designed as the spread of what implements it… name the axis it is most likely to be wrong about and check that something in the workspace sits at the other end of it."* On the cancellation axis all four implementations sit at one end, and the clause's own commentary says a real adapter is what removes that vacuity. So the honest statement is not that ES-22 is unmet — both adapters meet it, trivially — but that ES-22 is certified against nothing but a strawman the testkit wrote to be rejected, and one of its two assertions is untested code. The wrong outcome is real but later and narrower than Q-01's or X-3's: the first store whose `append` genuinely suspends is `happenstance-postgres` or `happenstance-neon` at phase 10 (`RUNBOOK.md:4544`), a network round trip per statement, where a dropped future lands *between* statements and a partial batch is the default rather than the exotic case; whoever builds it gets a green rule whose passing arm has never been exercised and whose behaviour under a driver's own rollback-on-drop nobody has watched. This corroborates `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`'s general question — *"whether a `[FROZEN]` clause may name a rule with no owning phase at all"* — in its sharper form: whether a `[FROZEN]` clause may name a rule that has an owner, is mounted, runs, and cannot discriminate.

**Semver.** None. Adding a fixture capability or a second rule to `happenstance-testkit` is additive, and the testkit is already at `0.2.0-alpha.1` on the registry. The cost is the population one: every rule added after 0.2.0 goes red on every adapter that took a caret requirement on the testkit, so a rule that belongs at 0.2.0 is cheaper at 0.2.0 than at 0.2.1 — but this one does not belong at 0.2.0, because the instrument it needs is the phase-10 adapter that does not exist yet. **This entry's release position differs from the rest of the section, and the difference is stated rather than smoothed over.**

**Remediation.** Deliberately thin, because the substantive answer is an adapter and not a rule. A mechanical check is available now and costs nothing: assert over the workspace that at least one crate mounting `event_store_conformance!` has an `.await` inside its `EventStore::append` body — today none does, and the assertion would go from failing to passing exactly when the axis acquires a far end.

**Routing.** Not a 0.2.0 item. It belongs to the instrument-portfolio question at RUNBOOK phase 10 (`RUNBOOK.md:4544`), where the first suspending `append` in this workspace is built, and to whoever owns §6.5's far-end table. Two smaller things are separable and go with it: ES-22's `Rejects:` paragraph (`spec/SPECIFICATION.md:3620-3635`) claims `MemoryEventStore` is the reason a real adapter is needed and is now false of the two real adapters as well, which is a clause-prose correction owed to whoever next amends §4; and ADR-0012's proposed but unbuilt gate step (`references/adr/0012-append-shape-and-preconditions.md:349-364`) is the same unbuilt instrument Q-02 routes, so the two should be decided together rather than twice.


---

## Ceilings in the wrong unit

Every capacity bound this workspace states is stated in a unit that does not bound the thing that fails. The contract gives `Event` four fields, bounds three of them, and asks the "why not a fourth limit?" question three times — at `limits.rs:47`, in ADR-0015's record, and again on the testkit's `Fixture` — answering it about query items on every occasion and never about `metadata`, which is the one unbounded field an `append` actually carries (AE-4). `happenstance-sqlite` then chunks its query plan by UNION arms and not by bound parameters, at two sites, one of them inside `BEGIN IMMEDIATE`; `experiments/shipped-append-condition-sql` now shows that failing for real, with the crate's own public seam calling the unpreparable plan valid (X-1). `happenstance-cloudflare`, publishing at the same release, chunks by nothing and declares no query bound at all (X-2). And `PAGE_SIZE = 512` budgets a read page in rows, so it bounds neither the bytes resident nor the time the write connection is held — the two things it is the only knob for (R-1+J-5). What makes these defects rather than omissions is that the corpus does the arithmetic correctly in the right unit whenever it is looking at the *insert* path — `MIN_SUPPORTED_EVENTS_PER_BATCH`'s doc multiplies `128 × 64 × 3` against `SQLITE_MAX_VARIABLE_NUMBER` and reports the headroom as a percentage — and the unit never crossed to the query path, and never crossed from `data` to `metadata`. Two of the four sit in the crate joining crates.io at 0.2.0 and are free until it does; one sits on a contract already published three times.

---

### AE-4 — `Event::metadata` is a fourth unbounded payload with no floor, no `StoreLimit` variant, and therefore no conformant way for any adapter to refuse it

**Clause.** **VT-1** — *An `Event` carries what its writer knows and nothing its store knows* (`spec/SPECIFICATION.md:592-598`), **`[FROZEN]`**, which mints `metadata` as the fourth field. **VT-21** — `MIN_SUPPORTED_EVENT_DATA_LEN` (`spec/SPECIFICATION.md:1513-1526`), **`[PROVISIONAL]`**, which bounds `data` and not it. **VT-25** — *A capacity refusal is distinguishable from a store failure* (`spec/SPECIFICATION.md:1628-1637`), **`[FROZEN]`**, which forecloses the only channel left.

**Examined.** `crates/happenstance-core/src/event.rs:321-326` (the four fields); `crates/happenstance-core/src/limits.rs:17-61` (the four floors and the enum); `crates/happenstance-testkit/src/contract.rs:264-290` (the fixture's three ceilings); `crates/happenstance-sqlite/src/event_store.rs:474-496` and `crates/happenstance-cloudflare/src/event_store.rs:424-448` (both `check_ceilings`); `crates/happenstance/src/command.rs:359`; `crates/happenstance/src/codec.rs:258-269`; `references/adr/0015-validated-identifiers-and-store-limits.md:552-557`.

**Found.** `limits.rs:47-51` asks the question and answers it about the wrong candidate:

> Three variants and not four: a query-item refusal is not an append outcome, so a `QueryItems` variant would name a refusal no `append` could ever produce.

The same substitution is made twice more. `contract.rs:287-289`: *"There is deliberately no `MAX_QUERY_ITEMS` beside these three. A query-item refusal is not an append outcome…"*. `crates/happenstance-sqlite/src/event_store.rs:270-272`: *"there is no `MAX_QUERY_ITEMS` anywhere in this crate and no fourth `StoreLimit` variant to report one through…"*. And in the decision record itself, `references/adr/0015-validated-identifiers-and-store-limits.md:552-554`: *"**Three variants, not four, and that is a decision.** VT-21, VT-22 and VT-24 each say a store 'MUST refuse beyond it with `AppendError::ExceedsStoreLimit`'. VT-23 deliberately does not…"*. Across all 1,152 lines of that record, `metadata` appears once, at `:506`, and it is about a column on the tag insert. There is no `MIN_SUPPORTED_METADATA_LEN` and no `StoreLimit::MetadataLen`.

Neither shipping adapter checks it. `happenstance-sqlite`'s `check_ceilings` tests `event.data().len()`, `event.tags().len()` and `events.len()`, and nothing else. `happenstance-cloudflare`'s does the same — and its ceiling derivation says so out loud, at `crates/happenstance-cloudflare/src/event_store.rs:214-216`, listing what shares the row with `data`:

> a nullable `metadata` blob the contract does not bound at all

That sentence is why `event_data_len` is 1 MiB against a 2 MiB platform row cap: the adapter bought its safety margin out of the field the contract *does* bound, because the field it could not bound had to fit somewhere. `experiments/durable-object-limits/README.md:124` records the same reasoning at derivation time.

**Why a defect.** A preference would be "the contract should bound more things." This is a defect because the three MUSTs do not compose. `check_ceilings` accepts a 1 MiB `data` at the declared ceiling; nothing bounds `metadata`; the Durable Object's documented 2 MiB row cap is a hard wall the adapter does not own. When the row crosses it, the adapter has exactly two moves and VT-25 `[FROZEN]` forbids both readings of one of them: report through `AppendError::Store`, which VT-25 says a store MUST NOT do for a capacity refusal and which `check_ceilings`' own `# Errors` doc (`crates/happenstance-sqlite/src/event_store.rs:469-473`) explains costs a sync runner the ability to tell *"this will never fit here, park it and tell a human"* from *"the disk is full, retry"*; or mislabel it `StoreLimit::EventDataLen`, whose doc says *"One event's `data` payload was larger than the store accepts"* and whose `guaranteed_minimum()` then hands the caller 65,536 — a number about the wrong field. The named wrong outcome is E2E-42's disappearance, arriving through the exact channel VT-25 was written to close.

This is not hypothetical for the typed layer. `crates/happenstance/src/command.rs:359` writes `.with_metadata(crate::codec::frame::<C>(None))` on **every** event it appends, and `frame`'s `application` parameter (`codec.rs:258-269`) exists to copy caller-supplied causation and correlation bytes through *"untouched and never parsed"*. Today that argument is `None` at every shipped call site, so the bytes are small and fixed; the seam is built for the case that is not, and `Event::with_metadata` is public in `happenstance-core` regardless.

The discriminator against "this is already known": the corpus asked the fourth-variant question three separate times and got the enumeration of candidates wrong each time, in the same way. Query items were considered and correctly excluded. `metadata` was never in the candidate set.

**Semver.** Additive, and additive **by luck rather than by design** — which is the part that makes 0.2.0 the deadline rather than a convenience. `StoreLimit` is `#[non_exhaustive]` (`limits.rs:53-54`), so a `MetadataLen` variant lands additively; `guaranteed_minimum()` returns `usize` rather than `Option<usize>`, so a new variant needs a new floor constant and gets one without changing the signature; `Fixture::MAX_EVENT_DATA_LEN` and its two siblings are defaulted associated consts, so a fourth is additive on the already-published `happenstance-testkit`. Had any one of those three gone the other way, the fourth floor would be unreachable on a published crate. What is *not* free at any point is the conformance rule: a new rule is a testkit minor that every adapter takes on its next `cargo update`, and it turns a green suite red for a store that was conformant the day before. That is the argument for landing the floor and the rule in the same release as the crate that first pays for the gap.

**Remediation.** A `MIN_SUPPORTED_METADATA_LEN`, a `StoreLimit::MetadataLen`, a `Fixture::MAX_METADATA_LEN`, and a rule in `append_reports_exceeded_store_limits`' family that appends `data` at the store's declared ceiling with `metadata` one byte over its own. The shape is mechanical; the number is not, and neither is whether the answer is a fourth floor at all rather than a stated obligation that `data + metadata` share one bound — which is what `happenstance-cloudflare` is already doing informally by carving the margin out of `event_data_len`.

**Routing.** A new ADR, and it cannot be an amendment: ADR-0015 is `accepted` (`.kb/decisions/0015-validated-identifiers-and-store-limits.md:5`) and therefore immutable, so this needs a superseding atom and a row in `RUNBOOK.md`'s ADR queue (`RUNBOOK.md:262`), which currently has no number for it. The decision is *whether `Event::metadata` gets a floor of its own, a shared bound with `data`, or an explicit written statement that it is deliberately unbounded and that every adapter must size its other ceilings around it*; the owner is whoever owns phase 4's VT clauses. Phase 12's exit criterion at `RUNBOOK.md:4726-4730` — every `[PROVISIONAL]` clause either has a scheduled falsifier or sits behind an unstable feature — is the gate this must clear before publish, because VT-21 is `[PROVISIONAL]` and its falsifier is about `data`.

---

### X-1 — the query plan is chunked by item count, never by parameter count, and the crate's own public seam reports the unpreparable plan as valid

**Clause.** **VT-23** — `MIN_SUPPORTED_QUERY_ITEMS` *is a floor of 128* (`spec/SPECIFICATION.md:1565-1584`), **`[PROVISIONAL]`**. Its `Rejects:` line names this implementation in terms. **VT-24** (`spec/SPECIFICATION.md:1586-1608`), **`[PROVISIONAL]`**, rejects the same failure *timing* on the sibling path.

**Examined.** `crates/happenstance-sqlite/src/query_sql.rs:80-107` (`Selectivity::read_for`) and `:154-171` (`chunks`), `:192-236` (`item_sql`); `crates/happenstance-sqlite/src/event_store.rs:277` (`MAX_QUERY_ARMS_PER_STATEMENT`), `:288-291` (`planned_statement_count`), `:577` (`PARAMETER_BUDGET`), `:645-653` (`evaluate`, inside `BEGIN IMMEDIATE`), `:1301-1320` (`fetch_page`); `crates/happenstance-sqlite/tests/wide_query.rs:1-13,46`; `crates/happenstance-testkit/src/suite.rs:3880-3923`. Measured in `experiments/shipped-append-condition-sql/`.

**Found.** `chunks` partitions on one axis and one only (`query_sql.rs:159-170`): `items.chunks(max_arms)`. `item_sql` pushes one bound parameter per tag and one per type into that chunk (`query_sql.rs:220,224,232`). `Selectivity::read_for` is not chunked at all — it accumulates `wanted` across every multi-tag item of the whole query with no cap (`query_sql.rs:81-91`) and builds `"SELECT tag, events FROM tag_cardinality WHERE tag IN ({placeholders(wanted.len())})"` (`:96-101`). It is called at `event_store.rs:1301` on the read path and at `event_store.rs:647` on the append path, where line 645's enclosing transaction is `BEGIN IMMEDIATE` and the write lock is held.

The parameter axis is not unknown to this crate — it is named, given a constant with headroom, and applied to exactly one path. `event_store.rs:571-577`:

> SQLite's `SQLITE_MAX_VARIABLE_NUMBER` is 32,766. This sits below it with headroom rather than at it… `const PARAMETER_BUDGET: usize = 30_000;`

Its only consumer is `event_store.rs:760`, the tag insert. `MAX_EVENTS_PER_BATCH`'s own doc (`event_store.rs:255-260`) does the multiplication correctly for that path — *"a single multi-row tag insert would bind 98,304 of SQLite's 32,766 bound parameters — which is why the insert is chunked to the parameter budget"* — and the query path never inherited it.

`experiments/shipped-append-condition-sql/results/selectivity.md:52-55` is the measurement, and both rows are a `prepare` that actually fails:

| site | items | tags/item | parameters | limit | SQL bytes | outcome |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `Selectivity::read_for` | 300 | 128 | 38,400 | 32,766 | — | `too many SQL variables` |
| `query_sql::chunks` | 400 | 128 | 51,600 | 32,766 | 3,229,593 | `too many SQL variables` |

400 items is `MAX_QUERY_ARMS_PER_STATEMENT` exactly — the adapter's own chosen boundary — and 128 tags is `MAX_TAGS_PER_EVENT` exactly. Nothing in `happenstance-core`'s `query.rs` bounds tags per query item at all, so this is a query a conformant caller may construct. The experiment's `selectivity.md:59-61` records the half of the claim that does not depend on its transcription:

> `chunks` reports the plan as **valid**: `plan.len() == 1`, and the shipped public `SqliteEventStore::planned_statement_count(&query)` agrees it is `1`. The statement it counted cannot be prepared.

That assertion is `experiments/shipped-append-condition-sql/tests/selectivity_cost.rs:208`, against the shipped public function.

**Why a defect.** VT-23's `Rejects:` line (`spec/SPECIFICATION.md:1578-1580`) is the discriminator, and it is not a paraphrase of the finding — it is the finding, written before the code:

> `Rejects:` an adapter that generates one SQL parameter per item and silently fails past a driver limit

VT-24's `Rejects:` (`spec/SPECIFICATION.md:1600-1603`) supplies the timing: *"discovers `SQLITE_MAX_VARIABLE_NUMBER` at write time — that is, after the caller has already made its decision and taken its side effects."* The adapter avoided that on the insert path with `PARAMETER_BUDGET` and reproduced it on the guard path with a different statement.

Who breaks, how, and when they find out: an application whose append-condition guard crosses a per-tenant boundary with a per-aggregate one at a few hundred items gets `prepare` failing with `too many SQL variables`, wrapped as `AppendError::Store`, arriving inside `BEGIN IMMEDIATE` with the write lock held and the caller's decision already made — and, per AE-4 above, `happenstance-core`'s `limits.rs` gives that refusal no honest variant to travel in. They find out in production, because nothing else can find it. The conformance rule that governs this floor, `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (`crates/happenstance-testkit/src/suite.rs:3899-3903`), builds 128 items carrying **one tag each** — 128 parameters — while its own assertion message (`suite.rs:3916-3919`) says *"an adapter that sends only its first chunk of bound parameters returns an answer that is wrong rather than an error that is honest."* The rule names the unit it is not measuring in. The crate's own standing guard is item-counted for the same reason and says so at `crates/happenstance-sqlite/tests/wide_query.rs:3-5`: *"The conformance suite **cannot reach the chunk boundary on its own**: VT-23's floor is 128 items, and at one tag per item that is nowhere near SQLite's compound-`SELECT` ceiling"* — and its own `WIDE` (`:46`) is `MAX_QUERY_ARMS_PER_STATEMENT * 2 + 100` items, at one tag each.

The `read_for` row is separately worse than the `chunks` row in one respect and better in another: it fails first, at 300 items, because it runs before `chunks` on both callers — and it is the one of the two that is not chunked at all, so no width parameter exists to tune.

**Semver.** None for the finding. The fix is additive and, more to the point, `happenstance-sqlite` is **not yet on crates.io** — it joins at 0.2.0. Every public constant in it, including a second one beside `MAX_QUERY_ARMS_PER_STATEMENT`, is free to add, rename or delete today and frozen the moment 0.2.0 goes out. `planned_statement_count` is public and returning a number that is wrong in the case that matters; changing what it counts is free now and a breaking change afterwards.

**Remediation.** Partition on both of SQLite's pushdown limits at the one entry point `query_sql.rs:6-10` already exists to be, and give `Selectivity::read_for` a chunk width it currently has no concept of. `PARAMETER_BUDGET` is the constant, it already carries the arithmetic and the headroom, and it currently has one caller where it should have three. Whether `planned_statement_count` continues to mean "arms" or is repointed at the real partition is the part with a public consequence.

**Routing.** ADR-0022 is accepted (`references/adr/0022-append-condition-strategy.md`) and owns the append-condition SQL strategy; this is a gap in what it settled, not a reversal of it, so it routes to phase 8's owner as a story under the SQLite adapter with the measurement already in hand — `experiments/shipped-append-condition-sql/` is out of the gate and reproducible via its `run.sh`. Two things need a decision rather than a patch, and neither is this document's: **whether `MAX_QUERY_ARMS_PER_STATEMENT` stays public once it is no longer the partition**, and **whether the testkit's VT-23 rule is widened to cross the tag axis** — the second is a testkit change, which per AE-4's semver note is a minor that every adapter takes involuntarily, so it is the same release-timing question. No `.kb/open-questions/` atom claims this territory; the recorded prior art is VT-23's own `Rejects:` clause and the module doc at `crates/happenstance-sqlite/src/query_sql.rs:1-10`, which describes the identical defect on the *arm* axis as already-fixed history.

---

### X-2 — `happenstance-cloudflare` does no query chunking at all and declares no query ceiling, on both paths

**Clause.** **VT-23** (`spec/SPECIFICATION.md:1565-1584`), **`[PROVISIONAL]`**. Note precisely what VT-23 permits: *"A store or an ingest policy MAY refuse a larger one"* — unlike VT-21, VT-22 and VT-24, it imposes **no** documentation obligation. So this is not a MUST violation, and the entry does not claim one.

**Examined.** `crates/happenstance-cloudflare/src/query_sql.rs:42-51` (`positions_matching`), `:58-71` (`arms`), `:78-...` (`item_sql`); its two call sites, `crates/happenstance-cloudflare/src/event_store.rs:467` (inside `evaluate`) and `:1280` (inside `render_read`); the crate's published limits table at `crates/happenstance-cloudflare/src/lib.rs:156-160`; `crates/happenstance-cloudflare/src/event_store.rs:200-236` (`Ceilings`).

**Found.** `query_sql.rs:42-51`:

```rust
pub(crate) fn positions_matching(query: &Query, bindings: &mut Vec<SqlValue>) -> String {
    match query.items() {
        None => "SELECT position FROM event".to_owned(),
        Some(items) => arms(items, bindings),
    }
}
```

`arms` (`:58-71`) is an unbounded `.join(" UNION ")`. There is no `max_arms` parameter, no chunk, and no parameter budget anywhere in the crate: `MAX_QUERY`, `max_arms` and `COMPOUND` return nothing across `crates/happenstance-cloudflare/src/`. The published limits table at `lib.rs:156-160` carries three rows — `data`, tags per event, events per append — and no fourth. `Ceilings` (`event_store.rs:228-236`) carries the same three fields.

The adapter is explicitly aware of the parameter axis and reasons about it correctly for the write path. `event_store.rs:222-231`, on `events_per_batch = 1,024`:

> The bound here is not SQL: one `INSERT` per event and one per tag, each its own statement, so no parameter cap is in play.

That is true and it is the *insert*. On the query path a parameter cap is very much in play, and nothing states it.

**Why a defect.** Not a clause violation, and stated as such. It is a defect because two adapters published under one contract at the same version have materially different query capability and neither crate's documentation lets a caller discover that before deploying. `happenstance-sqlite` chunks at 400 and publishes the width as a `pub const` precisely so a test can compute the boundary; `happenstance-cloudflare` neither chunks nor declares. Named wrong outcome: an application that develops against `happenstance-sqlite` and runs against `happenstance-cloudflare` — the pairing this workspace's own local-first story recommends — with a decision model above `SQLITE_MAX_COMPOUND_SELECT`'s 500 terms, or with wide items above 32,766 parameters, works locally and fails in the Durable Object with `AppendError::Store` carrying a raw driver string, on the write path. They find out on deploy. The conformance suite cannot find it for the reason X-1 gives: `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` issues 128 single-tag items, which is 128 arms and 128 parameters, comfortably inside both walls. `crates/happenstance-cloudflare/tests/durable_object_conformance.rs` runs against real SQLite behind the `DurableObjectState` shape, so the wall is reachable in the harness the gate already runs — it is simply never approached.

**Semver.** Additive. `happenstance-cloudflare` is one of the five release-bound crates and is **not yet published**, so a `MAX_QUERY_ARMS_PER_STATEMENT` of its own, or a fourth row in the `lib.rs` limits table, costs nothing today. After 0.2.0 the ceiling becomes a published promise and the number becomes hard to move.

**Remediation.** Mirror the sibling where the sibling is right — one entry point, chunk and merge on both paths, the width published — and diverge where the Durable Object's memory ceiling makes the sibling's shape wrong. The one thing not to mirror is keeping a second, unchunked spelling beside a chunked one: `crates/happenstance-sqlite/src/query_sql.rs:6-10` records that as exactly how the sqlite write path stayed unchunked while the module doc claimed otherwise.

**Routing.** ADR-0023 owns the `SqlStorage` mapping and is accepted; this is inside its scope and not in conflict with it, so it routes to phase 9's owner. The decision it implies — *whether the two adapters are required to agree on a declared query-item ceiling, or whether VT-23's deliberate silence on documentation stands* — belongs with VT-23's own `[PROVISIONAL]` marker and its falsifier, and is the same conversation as X-1's testkit question. It corroborates the recorded open question `.kb/open-questions/no-workerd-class-runner-in-the-gate.md` (accepted) only obliquely: the shim runs real SQLite, so this particular wall is one the existing harness *can* reach, which makes it a cheaper case than most of that atom's exposure rather than an instance of it.

---

### R-1+J-5 — `PAGE_SIZE = 512` is a row count, and it bounds neither of the two things it is the only knob for

*(Merged. The intake carried these as two findings — a memory bound and a lock-hold bound — over the same constant, the same function and the same multiplier. They are one ceiling in one wrong unit with two consequences, and splitting them makes each look smaller than it is.)*

**Clause.** **No clause governs this.** Stated explicitly rather than left silent: **ES-11** — *A read is a snapshot* (`spec/SPECIFICATION.md:2976-3020`), **`[PROVISIONAL]`** — is the clause that governs multi-statement paging, and it governs its *correctness* only, requiring a ceiling *H* captured no later than the first poll. Nothing in the specification bounds a page's resident bytes or the time a paged read may hold a shared handle. The nearest thing to a promise is a rustdoc sentence, quoted below.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:136-141` (`PAGE_SIZE` and its doc), `:245` (`MAX_EVENT_DATA_LEN`), `:249-252` (`MAX_TAGS_PER_EVENT`), `:474-496` (`check_ceilings`), `:1276-1405` (`fetch_page` in full, including the mutex acquisition at `:1281-1284`), `:1293` (`budget`), `:1318` (`merged`); `crates/happenstance-core/src/store.rs:151-158`; `crates/happenstance-sqlite/src/lib.rs`; `references/adr/0022-append-condition-strategy.md:585-587`.

**Found.** `event_store.rs:136-141`, the constant and its whole doc comment:

```
/// How many rows one `spawn_blocking` hop fetches.
///
/// The point of paging at all is that a replay of a million events must not be
/// buffered, which is the promise [`EventStore::read`](happenstance_core::EventStore::read)
/// makes. The value is a placeholder until it is measured.
const PAGE_SIZE: usize = 512;
```

The port-side promise it names, `crates/happenstance-core/src/store.rs:153-155`, is also stated in events: *"That is what lets an adapter stream a million-event replay without buffering it."*

Two things then fail to hold. **First, the doc says "one hop" and the code does not.** `fetch_page` takes the connection mutex at `:1281-1284` and holds it to the end of the function at `:1405`, across `Selectivity::read_for` (`:1301`), across every chunk of the plan (`:1316`), and across the merge, sort, dedup and truncate (`:1387-1399`). `budget` is `min(remaining, PAGE_SIZE)` (`:1293`) and is applied **per chunk** as a `LIMIT` (`:1374-1375`), while `merged` accumulates across all of them (`:1318`, `:1380`) and is truncated only afterwards (`:1399`). So one hop is `ceil(arms / MAX_QUERY_ARMS_PER_STATEMENT)` statements deep and holds up to that many times `PAGE_SIZE` rows, not `PAGE_SIZE` rows.

**Second, a row is not a bounded quantity.** Each `SequencedEvent` in `merged` owns its full `data` blob, which this adapter accepts up to `MAX_EVENT_DATA_LEN = 1_048_576` (`:245`), and its `metadata`, which — per AE-4 — `check_ceilings` (`:474-496`) does not bound at all, so one row's resident bytes are bounded by SQLite's own near-gigabyte blob limit rather than by anything this crate states. The row-count budget is the only budget: no byte budget exists, and `PAGE_SIZE` is private, undocumented to callers and not configurable — nothing in `crates/happenstance-sqlite/src/lib.rs` mentions it.

The crate prices its *other* ceilings in the right currency, which is the discriminator. `event_store.rs:249-251`, on `MAX_TAGS_PER_EVENT`: *"Every tag costs a row in `event_tag` and an upsert in `tag_cardinality`, both inside the write transaction, so the number is bounded by lock hold time rather than by storage."* The reasoning exists; it was applied to the write path and not to the read page.

**Why a defect.** Not "512 is the wrong number" — that would be a preference, and ADR-0022 §15 is explicit and correct that it measured nothing about paging (`references/adr/0022-append-condition-strategy.md:585-587`: *"**It does not touch `PAGE_SIZE`.** … it stays one; nothing here measured paging."*). The defect is that the constant is stated in a unit that bounds neither consequence, so no value of it is a fix. Two named wrong outcomes. **Memory:** an operator running the local-first, embedded profile this adapter targets rebuilds a projection with default `ReadOptions` — no `limit`, which is the shape a rebuild uses — over a log of events near the 1 MiB ceiling; the very first hop allocates `chunks × 512` rows into `merged` before one row reaches the caller, and on a container capped below that the process is killed with no diagnostic pointing at a read. **Latency:** an application shares one handle between a projection catch-up and its command loop — the shape `examples/transfers-on-sqlite` demonstrates — and every `append` on that handle waits for whichever page is in flight, quantised by a number nobody measured and multiplied by a query-width factor the number does not mention. Both are found at runtime, in production, by an operator with no knob to turn.

**Semver.** None as it stands: `PAGE_SIZE` is private. Whatever replaces it is not — a byte budget, a configurable page, or a `ReadOptions`-adjacent knob all add public surface to `happenstance-sqlite`, which is unpublished until 0.2.0 and frozen after. The half of the fix that needs no measurement (taking the mutex per statement rather than per page) is internal and free at any time; the half that needs a public knob is free only now.

**Remediation.** Two separable parts, and only one needs a number. Taking the connection mutex per statement inside `fetch_page` rather than across the whole page shrinks the worst case by the query-width factor and makes the doc comment true, and it costs nothing to decide. A byte budget beside the row budget is what makes the page's cost bounded at all; what it should be, and whether it is stated by the adapter or asked of the caller, is exactly the measurement ADR-0022 §15 declined to make.

**Routing.** An `experiments/` control, out of the gate per the `references/` practice this repository already follows — a lock-hold histogram over `fetch_page` with a concurrent appender on one handle, and an RSS or counting-allocator sample over a full replay of large events at several page widths. That measurement is unowned: it is not in phase 8's story map, not in ADR-0022's falsifier list, and no `.kb/open-questions/` atom claims it. The decision it feeds — *whether a read page is budgeted in rows, in bytes, or by the caller* — belongs to phase 8's owner and needs a row in `RUNBOOK.md`'s ADR queue if it is to be settled rather than inherited, because it adds public surface to a crate that is about to be published. **Citation note, for the one permitted in-place repair:** ADR-0022 §15 cites `event_store.rs:81` for `PAGE_SIZE`; at `56ef6c5` the constant is at `:141`. The record names the right text at the wrong line.

---

## SQLite under load — measured, and two of the predictions were wrong

The theme this section was given holds, and the two experiments built for it corrected it in three places. What holds: the one connection behind one `std::sync::Mutex` is the shape everything else here is a consequence of, and the crate already contains its own counter-example — `SqliteProjectionStore` routes every SQL-touching body through `in_blocking_task` and its module doc calls the asymmetry *"the defect"* (`crates/happenstance-sqlite/src/projection_store.rs:97-98`) while the event store, one file away, does not. Under identical contention the two calls take the same time — 870.3 ms and 870.9 ms — and one of them takes a `current_thread` reactor down with it for **885.761 ms** against a 16.177 ms idle floor while the other costs **27.525 ms**. What does not hold: the theme said the busy-timeout margin was endangered by a debug build on few cores, and both halves are refuted — debug sits inside release's noise, and the core axis runs backwards, one core being about 450x *safer* than twenty because the scheduler serialises the herd before SQLite's lock has to; and it said the gate overlaps five `CONTENDERS = 64` rules where it overlaps three. What the theme did not know is the more important result: at the contender count that ships today, one launch in seven produced `busy > 0` — the first nonzero busy count anywhere in this tree, and exactly what ADR-0022 §11's own re-open trigger asks for — and the `in_blocking_task` seam that fixes `append` and `head` does not fix `read()`'s first poll, which stays at **39.3x the floor after the fix** because ES-11 puts the ceiling sample on the polling thread by requirement. Everything below decides nothing: it names a clause, a number, an owner and a route, and it proposes no value for `BUSY_TIMEOUT_MS`, for `PAGE_SIZE` or for `CONTENDERS`.

**What the two instruments are, stated once, because every figure below inherits it.** `experiments/one-connection-latency/` puts the **real** adapter in its dependency graph by path (`Cargo.toml`, `happenstance-sqlite` with `event-store` + `projection-store`): reactor-stall arms 1, 2, 4 and 5 call the shipped `SqliteEventStore::append`, `head` and `read` and the shipped `SqliteProjectionStore::commit`. The paging arms cannot — `PAGE_SIZE` is a private const, `fetch_page` a private method on a private type — so those run a drift-checked copy: `query_sql.rs` byte for byte, `row.rs` under one rename, both re-derived from the live tree by four tests that ran before any clock (`results/raw/drift.txt`, `4 passed`), and 89 conformance rules × 4 page sizes over the modified replica (`results/raw/conformance.txt`, `356 passed`). `experiments/busy-timeout-margin/` measures **no part of `happenstance-sqlite` at all** — its store is `experiments/append-condition`'s measurement candidate, same schema, pragmas, `BEGIN IMMEDIATE` and guard SQL, one differing function — and its own README says so first. Both are one machine, one run, one observation per cell, on a host that was running twenty-odd other agents; every claim here is a ratio between arms of one run and an order of magnitude, never a third significant figure. Both are outside the gate by construction (CF-34), and neither may become a gate step.

---

### J-2 — `append`, `head` and `contains_event_id` run rusqlite on the calling executor thread; the crates.io README says the opposite, and the crate's own struct doc states the principle it violates

**Clause.** **No clause governs this, and two clauses guarantee none ever will.** Stated explicitly rather than left silent. **ES-36** — *Two `append` futures on one `&self` interleave safely* (`spec/SPECIFICATION.md:4249-4254`), **`[FROZEN]`** — is the nearest, and this adapter satisfies it *because* of the defect: with no `.await` in the body the first poll runs the append to completion, so there is nothing to interleave. **CF-33** — *No conformance rule may read a clock, measure elapsed time, or assert on an operation count* (`spec/SPECIFICATION.md:8720-8721`), **`[FROZEN]`** — forecloses any rule that could see a stall, and **CF-34** (`spec/SPECIFICATION.md:8747-8752`), **`[PROVISIONAL]`**, puts the measurement outside the bar. The unstated third obligation recorded at `spec/SPECIFICATION.md:4307-4330` — that *"a `read` issued while an `append` on the same handle is suspended must complete"* — is adjacent and does not reach this either, for the same reason: this `append` never suspends.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:1018-1052` (`append`), `:1069-1078` (`head`), `:1090-1098` (`contains_event_id`), `:406-411` (`settings`), `:152-159` (the struct and its `Arc` doc), `:180-188` (the field doc quoted below); `crates/happenstance-sqlite/src/projection_store.rs:91-101` (the module doc) and `:342-361` (`in_blocking_task`); `crates/happenstance-sqlite/README.md:58`, which `Cargo.toml:16` makes the crates.io front page; `references/adr/0022-append-condition-strategy.md:387-407` (§9). Measured in `experiments/one-connection-latency/`, instruments (a) and (c).

**Found.** `append` locks and runs inline, with no `.await` anywhere in the body (`event_store.rs:1039-1052`):

```rust
let mut connection = self
    .connection
    .lock()
    .map_err(|_| AppendError::Store(SqliteEventStoreError::ConnectionPoisoned))?;

// Steps 3 and 4 are one transaction, and that is the whole of the
// atomicity claim.
Self::append_locked(
```

The published README states the opposite (`crates/happenstance-sqlite/README.md:58`):

> `rusqlite` is synchronous, so every statement runs on a blocking task, and the read stream defers its `spawn_blocking` until the first poll

Only the second clause is true. And the crate states the principle against itself, in the doc on the very field that carries the runtime handle (`event_store.rs:184-186`), explaining why ADR-0022 §9 rejected running inline:

> it keeps blocking work on an executor's thread whenever there *is* one

**The numbers.** One `current_thread` runtime, a 1 ms `tokio::time::interval` with `MissedTickBehavior::Delay`, and a second connection opened through the adapter's own `connection::open_configured` holding `BEGIN IMMEDIATE` for 750 ms — well inside the 5,000 ms busy timeout, so every arm *succeeds*:

| arm | max tick gap | against the 16.177 ms idle floor |
| --- | ---: | ---: |
| **shipped `SqliteEventStore::append`** | **885.761 ms** | **54.8x** |
| CONTROL — shipped `SqliteProjectionStore::commit`, same contention | 27.525 ms | 1.70x |
| the same append through an `in_blocking_task` seam | 34.812 ms | 2.15x |
| shipped `head()` behind an in-flight append | 717.545 ms | 44.4x |
| `head()` through the seam | 41.573 ms | 2.57x |

The two calls take 870.3 ms and 870.9 ms — the same 750 ms wait for the same lock, to within 0.7%. In the affected arms the maximum gap is 99–102% of the call: there is no partial stall and no yield point, and the ticker fired **5** times where it fired 54 idle. The 16.177 ms floor is the Windows default timer resolution and not a stall; every arm's p50 is 15.48–16.01 ms, contended or not, which is the control saying the ticker kept one cadence and what varies is a single interruption.

**Why a defect.** Not "blocking in async is unidiomatic" — that is a preference, and for a synchronous driver it is a defensible one that ADR-0022 §9 weighed. Three things make it a defect. First, the crate's published README asserts the property it does not have, on the page a 0.2.0 evaluator reads first. Second, the correct implementation is in the same crate, and its module doc names the asymmetry as the defect *in advance* — *"a second, different answer inside one crate is the defect"* (`projection_store.rs:98`) — so this is not an open design question but an unpropagated one. Third, without CONTROL 1 the 885.761 ms would be a number with no scale: a reader could conclude that waiting 750 ms for a write lock is simply what contention costs. Arm 2 says it is not.

Who breaks, how, and when they find out: a service on `#[tokio::main(flavor = "current_thread")]` — the flavour `examples/transfers-on-sqlite` demonstrates, the flavour `#[tokio::test]` defaults to, and the likeliest flavour for the local-first deployment this adapter targets — running a projection runner on a second connection to the same file. Every request-path `append` that meets the write lock stops **every timer, every I/O completion and every other task on the runtime** for the whole of the call, up to `BUSY_TIMEOUT_MS = 5_000` in the worst case. They find out in production, as unexplained multi-second latency on requests that touch no database at all, because nothing in the gate can see it: CF-33 forbids the rule and CF-34 puts the harness outside the bar.

**Semver.** None on the API. The fix moves work between threads and changes no signature. It is not free of consequence, though, and the experiment states the cost rather than measuring it: `spawn_blocking` demands a `'static` closure, so the batch must be owned — `events.to_vec()` deep-clones every `Event`, `t + 2` allocations each by AE-1's mechanism, against a shipped `append` that takes `&[Event]` and clones nothing (`Bytes` is refcounted, so the payload does not copy). The one-shot cost is the README: `happenstance-sqlite` joins the registry at 0.2.0, and `README.md:58` is what ships with it.

**Remediation.** Subordinate, and already half-written in the crate. `projection_store.rs:342-361` is the seam; transcribing it onto the event store's three synchronous bodies took the contended `append` from 885.761 ms to 34.812 ms and `head()` from 717.545 ms to 41.573 ms, both within a factor of 1.5 of the projection store's own 27.525 ms. What the seam costs in allocations is the part that makes this a decision rather than an obvious win, and it is not on any table.

**Routing.** Not an edit, because it reverses the scope of an accepted decision. ADR-0022 §9 chose the seam for the *read* path and rejected inline execution there on two grounds, one of which — *"it keeps blocking work on an executor's thread whenever there is one"* — is now measured on the write path, which §9 never considered. `.kb/decisions/0022-append-condition-strategy.md:5` is `status: accepted` and therefore immutable, so widening §9 is a superseding atom through `/redkiln:kb-ingest` and a row in `RUNBOOK.md:262`'s ADR queue, which has no number for it. The owner is `happenstance-sqlite`'s at phase 8 (`RUNBOOK.md:4232`), jointly with phase 12's release gate for the README correction. The residual the fix leaves is **J-2R** below and belongs to the same decision.

---

### J-2R — the seam does not close the read path, because ES-11 puts the ceiling sample on the polling thread by requirement

*(New. Found by the measurement this review commissioned, not by the intake. It is the surviving clause of the finding filed three times as F2-1, I-4 and J-2 and deduplicated into J-2 — the clause that says the read's ceiling sample takes the same lock on the executor thread. It is separated here because the fix J-2 routes for does not touch it, and burying it inside J-2 would let "the seam works" be read as "the problem is closed".)*

**Clause.** **ES-11** — *A read is a snapshot* (`spec/SPECIFICATION.md:2976-2981`), **`[PROVISIONAL]`** (axis: transport, far end unbuilt). The clause requires the ceiling *H* to be captured **no later than the first poll**, and its own commentary names this adapter as conformant precisely because it does so in `poll_next` (`spec/SPECIFICATION.md:2996-3001`). The lock acquisition is therefore a **requirement of the clause**, not an oversight, which is what makes this a design residual rather than a bug.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:1239-1253` (`sample_ceiling`), `:1450-1457` (the call, inside `poll_next`, with its own comment `// ES-11's sample, taken **on this thread, before the spawn**.`), `:1468-1481` (the deferred `spawn_blocking` that happens *after* it), `crates/happenstance-sqlite/src/projection_store.rs:342-361`. Measured as arms 5 and 7 of `experiments/one-connection-latency/`.

**Found.** `sample_ceiling` takes the connection mutex from inside `poll_next`, deliberately (`event_store.rs:1243-1246`):

```rust
let connection = self
    .connection
    .lock()
    .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
```

and its own doc explains the failing runs that put it there — `read_result_is_stable_under_concurrent_append` and `query_items_share_one_snapshot` failing *"roughly one run in two"* until the sample moved (`event_store.rs:1232-1234`).

Arms 5 and 7 both drive a `read()` to its first row behind an in-flight append holding the mutex. Arm 5 is the shipped adapter; arm 7 is the replica with the sibling append on its own connection:

| | shipped | through the seam | ratio |
| --- | ---: | ---: | ---: |
| `read()` first poll | 639.290 ms | **635.056 ms** | **1.007x** |

**0.7%. The seam moved nothing, and it cannot** — a seam on the *write* side changes where the mutex is **held**, not where it is **acquired** on the read side, and the read-side acquisition is on the polling thread by ES-11's requirement. **39.3x the idle floor, after the fix.** (The 0.7% agreement between two independent code paths under one construction is also a fidelity check on the replica that no textual diff could produce, alongside the 356 conformance runs.)

**Why a defect.** The discriminator is that it survives the only remedy anyone has proposed, and nothing in the tree records that. Three findings converged on `in_blocking_task` and all three would be closed by it; a reader of the closed stories would reasonably conclude the executor-stall class is gone. It is not: on a `current_thread` runtime, a caller who issues `read()` while any append holds the mutex is parked for the whole of it, up to `BUSY_TIMEOUT_MS = 5_000` in the worst case, and the arms measured here are the *good* case — the holder released at 750 ms. The worst case is arithmetic from `connection.rs:62`, not a measurement, because measuring it means measuring a failure.

**Semver.** None on the current surface. The fix is not free of shape, though, and that is the reason it needs a decision: making `sample_ceiling` a cooperative re-poll — returning `Poll::Pending` and waking when the mutex frees — changes `SqliteReadStream`'s polling contract and needs a waker registration this adapter does not have. It also does not eliminate the wait; it converts a 5,000 ms hard stall into a scheduling question. `happenstance-sqlite` is unpublished until 0.2.0, so any public consequence of that change is free today and permanent afterwards.

**Remediation.** Deliberately thin, because the shape is the decision. The two live options are a cooperative re-poll of the ceiling sample, or a second connection dedicated to the ceiling `SELECT` — which WAL makes possible, since a writer does not block readers, and the only reason these `SELECT`s serialise at all is that *the adapter* serialises them, not SQLite. Both are larger than the seam and neither is settled here.

**Routing.** The same superseding ADR as J-2, as a named residual rather than a separate record — the decision is *whether the read path's mutex acquisition is moved off the polling thread, and at what cost to the stream's shape*, and the owner is `happenstance-sqlite`'s at phase 8. It bears on ES-11's `[PROVISIONAL]` marker from an unexpected side: the clause's falsifier is about a one-shot-HTTP transport that cannot sample in one round trip, and this is a second, closer instance of the same obligation being expensive — worth recording where ES-11's marker is next reviewed.

---

### The measurement `R-1+J-5` routed for came back, and it refuted half of J-5

*(**Not an entry.** The finding is filed in *Ceilings in the wrong unit* as the merged `R-1+J-5`, whose Routing asked for exactly this instrument — "a lock-hold histogram over `fetch_page` with a concurrent appender on one handle, and an RSS or counting-allocator sample over a full replay of large events at several page widths". It is reported here rather than there because what it found is a contention property, and because it is one of the two places a measurement contradicted this review.)*

**R-1's arithmetic was right to within 0.04%.** One page of 512 events at exactly `MAX_EVENT_DATA_LEN` (`event_store.rs:245`) peaks at **537,036,800 live bytes — 512.2 MiB** — in one buffer inside a single `spawn_blocking` hop, before one row reaches the caller, and holds the connection mutex **583.2 ms** doing it, uncontended. That is a lower bound: the counter sums `layout.size()` over live allocations and excludes the system allocator's size-class rounding, the arm seeded no `metadata` (which `check_ceilings` at `:474-496` does not bound at all), and a 1,200-item query multiplies the merge buffer by three.

**J-5's premise holds and its stated mechanism is falsified.** J-5 called `PAGE_SIZE` *"the only knob controlling how long a read holds the write connection"*. Across a **32x** range — 64 to 2,048 — the per-page hold moves **1.06–1.15x**, inside the run's own noise. What moves it is query **width**: at the shipped 512, **146.6 → 203.3 → 684.2 ms** for 1, 400 and 1,200 items, because `ceil(arms / 400)` statements per page goes 1 → 1 → 3, and the merge buffer's length before `truncate(budget)` was measured at exactly `3 × PAGE` — 192, 384, 1,536, 6,144. J-5's remediation asserted that multiplier as arithmetic; here it stops being arithmetic. But `PAGE_SIZE` *does* control the number of holds, so over a 10⁶-event replay the aggregate held-mutex time scales as `1/PAGE`: **2,929.2 s at 64, 286.4 s at 512, 99.1 s at 2,048** (width 1; extrapolation, labelled one — the longest pass was 200 pages).

**And the caller sharing the handle pays at the tail only.** A quiet appender is p50 0.118 ms / p99 7.727 ms. Under a concurrent replay at the shipped page size its p50 is untouched — 0.103 to 0.148 ms across all twelve cells — while its p99 goes to 186.3 ms, 259.1 ms and **799.041 ms** at widths 1, 400 and 1,200, with a worst observed append of 945.195 ms. The cost is asymmetric: at page 512 × width 1,200 a page holds the mutex 640.7 ms and waits **1.7 ms** for it, 371x. Nothing in the median warns anyone.

**The consequence for the decision, and it is the reason this is reported and not just cited.** J-5 and R-1 pull in opposite directions on one knob. The lock-hold table says raise `PAGE_SIZE`; the residency arm says raising it is unsafe, because the quantity that scales linearly with it is the one with no ceiling — at 2,048 rows the same page would be about 2 GiB. There is one knob, in the wrong unit, and its two consequences are not co-optimisable by any value of it. That is a decision — *whether a read page is budgeted in rows, in bytes, or by the caller* — and it belongs where the ceilings section routed it: phase 8's owner, with a row in `RUNBOOK.md:262`'s ADR queue, because it adds public surface to a crate about to be published. This document takes it no further than saying the two halves cannot be settled separately.

---

### W-1 — the busy-timeout margin is 1.3x–1.4x on the plateau, and both mechanisms the finding proposed for why are refuted

**Clause.** **No clause governs the constant.** `BUSY_TIMEOUT_MS` is an adapter's number, and the specification is silent on it by design. Two clauses fence what may be done about it: **CF-33** (`spec/SPECIFICATION.md:8720-8721`), **`[FROZEN]`**, forbids any conformance rule from reading a clock, so the timeout is *the only liveness bound in the system*; **CF-34** (`spec/SPECIFICATION.md:8747-8752`), **`[PROVISIONAL]`**, keeps the measurement out of the gate. The authority for the number is ADR-0022 §11, whose own re-open trigger this measurement fires.

**Examined.** `crates/happenstance-sqlite/src/connection.rs:51-62` (the constant and its whole rationale); `references/adr/0022-append-condition-strategy.md:140-144` (the measurement's recorded conditions) and `:614-616` (the falsifier); `xtask/src/main.rs:179-190` (the gate's test invocation); `crates/happenstance-testkit/src/concurrency.rs:238` (`CONTENDERS`), `:394`, `:611`, `:691` (the three rules that open that many handles), `:508-509` and `:795` (the two that do not); `crates/happenstance-sqlite/tests/concurrency.rs:41-49`. Measured in `experiments/busy-timeout-margin/`.

**Found.** The constant's doc claims a measurement (`connection.rs:59-61`):

> Five seconds was measured to absorb 64-way contention with zero `SQLITE_BUSY` (ADR-0022 §11), which is what makes that zero mean something rather than being the zero an unbounded handler would also have produced.

The record's conditions are *"20 logical cores… `--release`"* with its own target run alone (`0022:140-142`). The gate's are not those: `xtask/src/main.rs:181-188` passes `["test", "--locked", "--workspace", "--all-features", "--", "--show-output"]` — no `--release`, no `--test-threads` cap — and libtest overlaps sibling `#[test]` fns from one binary.

In the gate's own configuration the worst per-contender **budget** wait is **3,628 ms of 5,000 — a margin of 1.38x**. The worst cell anywhere in the sweep is 8 cores at **3,828 ms, 1.31x**; 8 and 20 are not separable on this host and traded places between runs, so the defensible statement is *roughly 1.3x–1.4x on the plateau*. Every `wait_ms` figure is a **lower bound**: the counting handler resolved the same race 2.6x faster on the median than SQLite's own, uncalibrated, and across five runs the ratio was 2.18x, 2.27x, 2.40x, **0.66x** and 2.57x — directionally consistent, numerically unstable, and the inverted run is reported rather than dropped. Do not quote a wait here as exact.

**Two refutations and one correction, stated plainly.**

* **The core axis runs backwards.** 1 core → 8 ms, 2 → 628, 4 → 2,628, 8 → 3,828, 20 → 3,628. A two-core container is the *safest* configuration in the matrix, by about 450x against twenty, and `taskset -c 0,1` — the instrument the finding proposed — would have measured the safest cell and reported an 8.0x margin as though it were the hazard. The mechanism is in the retry column: 913 entries into the handler at one core, 8,817 at eight. SQLite's handler is a back-off **poll, not a queue**; the pathology needs contenders simultaneously runnable and retrying in lockstep, which is what cores buy and one core forbids.
* **The build profile barely enters.** Release at 20 cores: 3,228 ms alone, 3,328 ms with three rules concurrent, against debug's 3,628 ms — inside one band. The time is spent *asleep*, so `--release` has nothing to recover.
* **The gate overlaps three `CONTENDERS = 64` rules, not five.** `event_store_concurrency_conformance!` emits five `#[test]` fns; three open `CONTENDERS` handles (`concurrency.rs:394`, `:611`, `:691`). `k_disjoint_boundaries_admit_exactly_k_commits` opens `BOUNDARIES * PER_BOUNDARY` = 12 (`:476-479`, `:508-509`) and `a_concurrent_reader_never_sees_a_partial_batch` opens `WRITERS + 1` = 5 (`:775`, `:795`). The finding cited five and its two extra citations are those two rules.

**Why a defect, given all that.** The finding's *conclusion* survives its refuted mechanism, and the survival is the result. What is defective is not the number 5,000 but the epistemics around it: a single-cell measurement, taken under conditions the gate does not reproduce, is quoted in a shipping doc comment as though it characterised a distribution, when the distribution inside one cell has a **12x** spread and is bimodal — 640 samples at 8 cores put 197 contenders under 100 ms and nine past 3,000 ms, with a median of 328 ms. A mean would have reported about 600 ms and hidden the question. And the shipped guard strategy buys nothing here: `monotonic-guard`, which ADR-0022 §4 chose, reached 3,728 ms against the probe arm's 3,628 ms. **The margin is a property of the herd, not of the guard.** Who breaks and when: whoever runs `cargo xtask ci` on a host at the plateau — which is any developer machine or CI runner with four or more cores available to the process — and the symptom is L2-02's panic, below.

**Semver.** None. `BUSY_TIMEOUT_MS` is a `pub const` on `happenstance-sqlite`, which joins crates.io at 0.2.0, so its value and its doc are free to change today and become a published promise afterwards. Nothing about the fix is breaking in either direction.

**Remediation.** Subordinate and deliberately valueless. The measurement supplies what the doc comment lacks — a distribution, a plateau, a refuted core hypothesis, and the note that every figure is a floor — and it proposes no number. The one thing it does say about the obvious move is that raising a wall-clock timeout is a bound on the hardware and not on the store.

**Routing.** ADR-0022 §11's falsifier — *"Re-open the busy timeout if any run ever reports `busy > 0`"* (`0022:615-616`) — is fired by L2-02's evidence below, so this and L2-02 arrive at the same record together. `.kb/decisions/0022-append-condition-strategy.md` is `accepted` and immutable: re-opening §11 is a superseding atom and a row in `RUNBOOK.md:262`'s ADR queue. The `CONTENDERS` half is explicitly **not** ADR-0022's — §12 says *"it supplies a number and `concurrency-family-and-contender-count` decides"* — and that story (HS-S0041, `.bklg/from-contract-to-published-library/sqlite-durable-store/concurrency-family-and-contender-count/`) is the one that set the constant to 64. It inherits a measured margin at the value it chose, which is what its own AC-005 asked for. Owner: phase 8's, before phase 12.

---

### L2-02 — a busy store and a broken store are the same `Attempt`, and ADR-0022 §11's falsifier fired at the contender count that ships

**Clause.** **CF-33** — *No conformance rule may read a clock, measure elapsed time, or assert on an operation count* (`spec/SPECIFICATION.md:8720-8721`), **`[FROZEN]`**. Its `Rejects:` paragraph is the reason there is no watchdog, and the concurrency module restates the whole argument at `crates/happenstance-testkit/src/concurrency.rs:24-44`. No clause governs the *classification*; that is the gap.

**Examined.** `crates/happenstance-testkit/src/concurrency.rs:247-254` (`enum Attempt`, private), `:259-265` (`Attempt::of`), `:405-414` (the assertion), `:183-190` (`ConcurrentFixture` and its blanket impl); `crates/happenstance-sqlite/src/event_store.rs:1006-1015` (`append`'s own `# Errors` section, which already distinguishes contention from failure in prose); `crates/happenstance-sqlite/tests/concurrency.rs:41-49`. Measured in `experiments/busy-timeout-margin/`, the headroom and rate sweeps.

**Found.** Everything that is not `ConditionViolated` collapses to one arm (`concurrency.rs:259-265`):

```rust
fn of<E: core::fmt::Display>(result: Result<SequencePosition, AppendError<E>>) -> Self {
    match result {
        Ok(position) => Self::Committed(position),
        Err(AppendError::ConditionViolated(_)) => Self::Rejected,
        Err(other) => Self::Failed(format!("{other}")),
    }
}
```

and the rule panics on any of them (`:409-414`):

> a contender must either commit or be told `ConditionViolated`; anything else is a store failure under contention rather than the concurrency signal.

The store's own `append` doc already draws the distinction the testkit cannot receive (`event_store.rs:1014-1015`): *"including `SQLITE_BUSY` after the configured busy timeout has genuinely elapsed, which is contention reported honestly rather than a condition violation"*.

**The falsifier fired.** Under **SQLite's own** handler — so `busy` is one bit that cannot be an artefact of the instrument — debug, all cores, three racing rules, five rounds per cell:

| contenders | attempts | `busy`, by rule | rate |
| ---: | ---: | --- | ---: |
| **64** | 960 | 0, 0, 0 | 0% |
| 96 | 1,440 | 3, 15, 13 | **2.2%** |
| 128 | 1,920 | 82, 105, 75 | **13.6%** |
| 160 | 2,400 | 108, 178, 123 | **17.0%** |

Headroom above the shipped 64 is under 1.5x. And at 64 itself the answer is not "never": the shipped configuration was launched seven times, and **one produced `busy > 0`**. This is the first nonzero busy count anywhere in this tree, and it is exactly what ADR-0022 §11's re-open trigger asks for.

**Read the raw rows, not the summary.** `results/raw/rate-n64-r2.txt` lines 5 and 7:

```text
…race_us_max=5522382  committed=5  rejected=314  busy=1  failed=0  exhausted=0
…race_us_max=5502321  committed=5  rejected=313  busy=2  failed=0  exhausted=0
```

Three attempts across two rules **entered** SQLite's busy handler; `exhausted=0` on both rows — no attempt ran the 5,000 ms budget out. The experiment's own summary page says *"three contenders across two rules exhausted the full 5,000 ms"* (`results/busy-timeout-margin.md:200`) and that overstates it; the raw rows are what `results/raw/` is kept whole for. The claim to quote is **`busy > 0` at the shipped `CONTENDERS = 64`**, which is what the trigger names. `committed` stays at 5 in every row, so the *semantic* property held throughout; what failed is liveness. Note also that `busy = 0` means "not observed", not "safe": each matrix cell is 640 samples and the distribution is skewed enough to hide a rarer tail.

**Why a defect.** Not "the testkit should be more forgiving". The chain is stated in the adapter's own decision record and ends in the wrong message: `SQLITE_BUSY` → `AppendError::Store` → `Attempt::Failed` → a rule that panics saying *"a store failure under contention rather than the concurrency signal"*. That sentence tells an adapter author their store is wrong when nothing about their store is wrong, in the one place CF-33 `[FROZEN]` guarantees the suite cannot diagnose it. And there is no third classification available to say otherwise: a transient, contention-induced refusal by a store whose semantics are perfect is indistinguishable, by construction, from a semantic violation. Who finds out and when: the next out-of-tree adapter author on a plateau-class machine, on their first `cargo test`, reading a message that sends them to debug code that is correct. Worse for the population than for this workspace, because they have no ADR-0022 to tell them what they are looking at.

**Semver.** Breaking on a **published** crate, and the theme's positioning line was imprecise about where. `Attempt` is private (`concurrency.rs:247` — no `pub`), so a third arm is internal and free. What is not free is the declension: whatever lets a fixture declare that a transient busy refusal is contention rather than a defect must be public, and `ConcurrentFixture` (`:186`) has a blanket impl over every `Fixture`, so a required item there breaks it — the tolerance has to land on `Fixture` in `happenstance-testkit`, live at `0.2.0-alpha.1`, as a defaulted associated const. Additive today; the same C2-04 hazard on the next capability. And the population cost is the standing one: a rule or a classification added after 0.2.0 goes red on every adapter that took the caret requirement the crate's own onboarding tells them to write.

**Remediation.** Subordinate, and the tempting fix is the wrong one. `AppendError::Busy` in `happenstance-core` is refused on this repository's own precedent — ADR-0022 §10 declined to mint `index_arms()` in the contract crate for one adapter's benefit, quoting CLAUDE.md's *"one implementor is not a spread"*. Whatever shape this takes belongs in `happenstance-testkit` plus a dev-only override, which is what keeps it clear of all six binding constraints. The instrument that would make it checkable is a decorator over `MemoryEventStore` whose `append` refuses the first *m* callers with a transient store error — a correct store whose driver made the caller wait.

**Routing.** Two owners and one record. ADR-0022 §11's falsifier has fired, so it re-opens by superseding atom (`.kb/decisions/0022-append-condition-strategy.md:5` is `accepted`), together with W-1 above. The classification is the testkit's and belongs to the same phase-8 owner as HS-S0041, before phase 12, because it is the one change here whose cost rises the day 0.2.0 ships. The decision this document does not take is **whether the testkit grows a fixture-declared tolerance for transient contention, or whether `CONTENDERS` moves, or both** — ADR-0022 §12 already says the second is not its call.

---

### M-1 — `block_on` is a bare park-loop, the concurrency family nests it inside itself, and the collision hangs deterministically

**Clause.** **No clause governs `block_on`'s implementation.** The relevant statement is the specification's own account of what the failure looks like from outside (`spec/SPECIFICATION.md:4321-4323`): *"The executor parks and never wakes. There is no watchdog by design (CF-33), so this is a hung CI job naming no rule."* **CF-33** (`spec/SPECIFICATION.md:8720-8721`), **`[FROZEN]`**, is what forbids the diagnostic.

**Examined.** `crates/happenstance-testkit/src/registry.rs:302-308` (`ParkWaker`, whose `wake` is a bare `unpark()`), `:330-344` (`block_on`) and `:341` (`Poll::Pending => std::thread::park()`, with no re-check); the nesting at `crates/happenstance-testkit/src/concurrency.rs:890`, `:979-982` and `:1125-1133`; `crates/happenstance-sqlite/tests/concurrency.rs:41-49`. Measured in `experiments/busy-timeout-margin/tests/lost_wakeup.rs`.

**Found.** The loop has no notified flag (`registry.rs:338-342`):

```rust
match future.as_mut().poll(&mut context) {
    Poll::Ready(value) => return value,
    // Parking rather than spinning: a rule that awaits real I/O still
    // makes progress, and a rule that never wakes hangs visibly rather
    // than burning a core.
    Poll::Pending => std::thread::park(),
}
```

`park`/`unpark` carries **one** token and it belongs to the *thread*, not to the `block_on` call, so any park on that thread consumes it — including one belonging to a different, inner `block_on`. The nesting is not hypothetical: `concurrency.rs:890` calls `incomplete_batches` on the rule's own thread, `incomplete_batches` opens a second `crate::block_on` at `:980`, and the rule is itself driven by `crate::block_on` from `__emit_concurrency_blocking` at `:1130`.

| case | outer wake at | inner ready at | completes within 10 s |
| --- | --- | --- | --- |
| 1. no nesting (baseline) | 20 ms | — | **yes** |
| 2. nested, no collision | 200 ms | 20 ms | **yes** |
| 3. nested, wake lands during the inner park | 20 ms | 200 ms | **no — hung** |

Reproduced on four consecutive whole-directory runs with no observed variation. Case 1 is what makes the others evidence; case 2 is what stops the probe reading as an indictment of nesting as such.

**Why a defect.** The probe deliberately forces the ordering with sleeps, so it says the failure exists and is reachable, not how often an unforced schedule produces it — and it says nothing about whether any shipped rule hangs today. What makes it a defect rather than a latent curiosity is the diagnostic collapse it sits inside. `crates/happenstance-sqlite/tests/concurrency.rs:45-47` instructs the reader that *"If a rule hangs, that is evidence about ADR-0022's busy-timeout paragraph"* — and that instruction is only sound if the hang really was the busy timeout. Two mechanisms in this workspace produce a stopped CI job that names no rule, and CF-33 `[FROZEN]` forbids the suite the watchdog that could tell them apart. That is precisely why the lost-wakeup probe was run in the same directory as the busy-timeout measurement: measuring one of the two would let either finding be answered with *"that was probably the other one"*. Who breaks and when: an out-of-tree adapter whose read future is completed by a background thread — `MemoryEventStore` completes on the calling thread and cannot produce it; a `spawn_blocking` adapter can — running under the blocking emitter, and the symptom is a job timeout with no rule name, misdiagnosed as a busy timeout by the crate's own instruction.

**Semver.** None. `block_on` is `pub` (`registry.rs:330`) with a doctest, on a crate already at `0.2.0-alpha.1`; demoting the park token to a hint behind an `AtomicBool` changes no signature and no observable behaviour except the hang.

**Remediation.** Subordinate and mechanical: make an `AtomicBool` the truth and the park token a hint, re-checking it before and after each park. It is not implemented or measured — the experiment may not touch `crates/`.

**Routing.** `happenstance-testkit`'s owner, and it is one of the two items in this section that decides whether a stranger's first `cargo test` against the suite terminates. It is separable from every decision above and needs no ADR: nothing in `.kb/decisions/` or `spec/` settles `block_on`'s waker discipline, and the change reverses nothing. The one thing it *is* coupled to is L2-02's message: fixing this narrows what a hung job can mean, which is what makes the busy-timeout diagnosis actionable rather than a guess. The instruction at `crates/happenstance-sqlite/tests/concurrency.rs:45-47` wants revisiting in the same change, since it currently names one of two candidates as though it were the only one.

---

### J-3+J-4 — `#[derive(Clone)]` publishes connection sharing the struct's own doc disclaims, and mutex poisoning makes that sharing permanent

*(Merged. The intake carried these separately — a poisoning defect and a `Clone` defect. They are one struct declaration: `event_store.rs:152-159` is a doc paragraph saying the `Arc` is not for sharing, a `derive` that shares it, and a `Mutex` whose poisoning is sticky. Separately each reads as a nit; together they are a handle model that is wrong in what it promises and terminal in how it fails, and the `derive` is what propagates the failure to every "handle" the user believes is independent.)*

**Clause.** **ES-33** — *The fixture yields handles onto one backing store* (`spec/SPECIFICATION.md:4144`), **`[FROZEN]`**, and **ES-34** — *Two handles share one consistency boundary* (`spec/SPECIFICATION.md:4178`), **`[FROZEN]`**, define what the conformance model means by a handle: what `Fixture::connect` returns. Neither governs what a store's `Clone` means, and no clause does. Stated explicitly.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:152-159` (the doc and the derive), `:157-189` (the three fields — there is no path among them), `:288-448` (every `pub fn` on the type: `planned_statement_count`, `new`, `open`, `open_in_memory`, `store_id`, `settings`, `remint_identity` — no `connect`), `:862-870` (`ConnectionPoisoned`), and the six lock sites that map poisoning to it terminally: `:410` (`settings`), `:1042` (`append`), `:1073` (`head`), `:1094` (`contains_event_id`), `:1246` (`sample_ceiling`), `:1284` (`fetch_page`). Against `crates/happenstance-sqlite/src/projection_store.rs:353` and `:456` (the seventh site and the second public variant), `crates/happenstance-core/src/memory.rs:190-198`, and the crate's own fixture at `crates/happenstance-sqlite/tests/support/mod.rs:208-215`.

**Found.** The doc and the derive are three lines apart (`event_store.rs:152-159`):

```rust
/// The [`Arc`] is not for sharing the store; it is so that a
/// [`SqliteReadStream`] can outlive the `&self` borrow that produced it, which
/// it must, because `read` is not `async` and hands the stream back to the
/// caller.
#[derive(Debug, Clone)]
pub struct SqliteEventStore {
    connection: Arc<Mutex<Connection>>,
```

A clone shares one `rusqlite::Connection` behind one `std::sync::Mutex`. The crate's own fixture does the opposite where it matters — `connect()` calls `SqliteEventStore::open(&self.path)` for a genuine second connection (`tests/support/mod.rs:209-210`) — and the store cannot offer that itself, because it holds no path.

Poisoning is sticky and every site gives up. The workspace already contains the recovery it declines, with the rationale written out (`crates/happenstance-core/src/memory.rs:190-198`):

> A panic in another thread while holding the lock cannot have left this store inconsistent… So poisoning carries no information here and is ignored rather than propagated as a spurious failure.

**Why a defect.** Two discriminators, and the second is what the merge is for. First, `Clone` on a store type means a pool nearly everywhere else in the Rust database ecosystem, and the type's own doc says the `Arc` is *not* for sharing — so the API says one thing, the derive does another, and the conformance model's word for the thing the user wants (`Fixture::connect`, ES-33/ES-34) is unreachable from the public API. `let store = SqliteEventStore::open(…)?; for _ in 0..16 { tokio::spawn(handler(store.clone())); }` yields sixteen tasks on one connection and one mutex; throughput does not scale, and by J-2 each in-flight append stalls whatever thread polled it. Second — and this is why they are one entry — that same derive is what makes poisoning total. One panic anywhere under the lock converts *the store, every clone of it, and every `SqliteReadStream` already handed out* into a store that returns `ConnectionPoisoned` for `append`, `head`, `contains_event_id`, `settings` and every subsequent read page, for the life of the process, with the message *"the SQLite connection mutex was poisoned by a panicking thread"* and no instruction. The conservatism buys nothing: rusqlite's RAII has already restored the connection — `Transaction::drop` rolls back — and the sibling `MemoryEventStore` recovers on a documented rationale that transfers unchanged. Who finds out and when: an operator, at runtime, after any panic under the lock (a future rusqlite assert, a `to_event` decode of a row a later migration wrote), whose event store is dead until the process restarts.

**Semver.** Breaking on both halves, and **free only until `happenstance-sqlite` reaches crates.io at 0.2.0**. Removing `Clone` — or keeping it and adding a `connect()` that opens a second connection, which needs the store to remember its path — changes the type's shape. Recovering the guard makes `SqliteEventStoreError::ConnectionPoisoned` (`event_store.rs:870`) and `SqliteProjectionStoreError::ConnectionPoisoned` (`projection_store.rs:456`) unreachable, and deleting a named variant from a `#[non_exhaustive]` enum still breaks anyone who names it. This is the entry in this section whose price changes most sharply on the release date.

**Remediation.** Subordinate. Recovery is `PoisonError::into_inner` plus a *check* rather than an assumption that the connection came back clean, because `rusqlite::Transaction::drop` ignores the result of its own `ROLLBACK` — and `crates/happenstance-sqlite/src/connection.rs` is where it belongs, since `lib.rs` already gates that module on either store feature so the helper cannot compile into a build that has neither. The handle half needs the store to carry an origin it currently does not.

**Routing.** `happenstance-sqlite`'s owner at phase 8 (`RUNBOOK.md:4232`), before phase 12. Two decisions are owed and this document takes neither: **whether `Clone` continues to exist on a store type whose clone is not a handle**, and **whether the two `ConnectionPoisoned` variants are deleted or retained as documentation of a state that can no longer occur**. Both are ordinary adapter decisions rather than ADR material — no accepted decision and no clause claims this territory — but both are permanent after 0.2.0, which is what makes them dated rather than discretionary.

---

### F2-3 — the runtime `Handle` captured at construction has no escape hatch, and `NoRuntime` is unreachable for a store that outlives its runtime

**Clause.** **ES-11** (`spec/SPECIFICATION.md:2976-2981`), **`[PROVISIONAL]`**, is the clause that forces the deferred spawn and therefore the seam; its own commentary names ADR-0022 §9's capture-and-prefer as what settled it (`spec/SPECIFICATION.md:2996-3004`). No clause governs the escape hatch.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:326-332` (the capture), `:1468-1476` (the preference at poll), `:176-188` (the field doc); `crates/happenstance-sqlite/src/projection_store.rs:220` (the identical capture) and `:330-336` (the identical preference, gating `in_blocking_task` at `:342-361`, through which `begin`, `commit`, `reset` and `rollback` all pass); `references/adr/0022-append-condition-strategy.md:387-407` (§9) and `:609-612` (its re-open trigger).

**Found.** `with_store_id` captures unconditionally (`event_store.rs:326-332`):

```rust
fn with_store_id(connection: Connection, store_id: StoreId) -> Self {
    Self {
        connection: Arc::new(Mutex::new(connection)),
        store_id,
        runtime: Handle::try_current().ok(),
    }
}
```

and `poll_next` prefers the captured handle over the live one, consulting `Handle::try_current()` only when the field is `None` (`:1468-1476`). No constructor, setter or builder can clear or replace it. `SqliteEventStore::new` and `::open` (`:316`, `:357`) are the only doors in and neither takes a handle.

**Why a defect.** ADR-0022 §9 chose this with full knowledge of its cost profile and wrote its own re-open trigger for exactly this (`0022:609-610`): *"Re-open it if a deployment shows the captured `Handle` costing something the inline path does not."* This is that trigger being tripped — which is why the entry is filed as corroboration of a recorded decision's own falsifier rather than as a disagreement with it. The cost the record did not price: the capture is unconditional and irreversible, so a store built inside one runtime and served from another gets a store that `append`s and `head`s perfectly (those run inline, per J-2) and whose every `read` either hangs or yields one `Worker(task was cancelled)` item and terminates — while `SqliteEventStoreError::NoRuntime`, the variant documented for precisely this situation and which §9 kept *because* option (a) preserved its meaning, is unreachable. The shape is ordinary: `Runtime::new().block_on(migrate_and_open())` at startup then a worker runtime, or a `OnceLock<SqliteEventStore>` first initialised inside a `#[tokio::test]`.

**And the worse half is in the other module.** `projection_store.rs:220` makes the identical capture and `:330-336` prefers it the same way — but there it gates `in_blocking_task`, through which every write passes. A projection store whose construction runtime has gone does not merely read empty: its checkpoint never advances, and it reports that as `Worker(JoinError)`, which a projection runner reads as a transient driver failure and retries forever. A fix landing only in `event_store.rs` leaves that in place.

**Semver.** Additive as a finding; the fix is additive too — a constructor or builder that accepts an explicit `Handle`, or one that declines to capture. `happenstance-sqlite` is unpublished until 0.2.0, so any constructor added now is free and any added later is a permanent surface either way. This is the least dated item in the section.

**Remediation.** Subordinate: an explicit way in, and the same one in both modules. What is *not* subordinate is which module gets it — the projection store's is the writing half and the one that retries forever.

**Routing.** ADR-0022 §9's own re-open trigger, so it arrives at the same superseding record as J-2 and J-2R rather than at a new one — all three are §9's scope seen from three sides, and deciding them separately would produce three partial answers to one question. Owner: `happenstance-sqlite`'s at phase 8. This does not corroborate any `.kb/open-questions/` atom; the prior art is the ADR's own falsifier at `references/adr/0022-append-condition-strategy.md:609-612`, which describes the trigger before the code existed.

---

### R-3 — `remint_identity`'s "nothing else holding the database open" is trust-only, even for the in-process case a program can check

**Clause.** **VT-6** — *`StoreId` names a store incarnation, not a device and not a peer* (`spec/SPECIFICATION.md:813-819`), **`[PROVISIONAL]`**. VT-6 permits mint-once *"**only if** it can detect that its state was restored or cloned, **or** the deployment is documented to invoke the re-mint"* (`spec/SPECIFICATION.md:859-863`). This adapter takes the second branch, so the documented procedure is the half of the permission it rests on.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:414-447` (the procedure's documentation) and `:448-463` (its body); `:326-332` (where `store_id` is fixed, once, per handle); `spec/SPECIFICATION.md:832-837` (VT-6's `Rejects:`).

**Found.** The precondition is stated and nothing enforces it (`event_store.rs:432-434`):

> Run it with nothing else holding the database open, and *before* the restored copy accepts its first append: events already in the file keep the origin they were stamped with, which is correct — they were written by the incarnation that is being retired.

The body opens its own connection, migrates, rewrites `store_meta` under `BEGIN IMMEDIATE` and commits (`:449-462`). It does not check for, block, or detect another live `SqliteEventStore` on the same path — and a live handle keeps the `store_id` it read at construction (`:326-332`), so it keeps stamping appends under the retired incarnation after the file's persisted identity has moved on.

**Why a defect.** VT-6's own `Rejects:` paragraph names this failure class in terms — *"the one failure mode in the replication design with no error path and no observable symptom"* (`spec/SPECIFICATION.md:836-837`). The governance corpus accepts that risk for the cross-process and restore-from-backup case, which is genuinely undetectable from inside SQLite; it has never named the strictly narrower **in-process** instance, where no external actor is involved and the split is visible to the program that caused it. Who breaks and when: an operator restores a file in place while the application was not fully drained — a surviving handle from before the restore, or a health check that reopened the store early — follows the documented procedure correctly, and the stale handle keeps appending under the old incarnation. Nothing errors locally; the collision surfaces only when a replication peer sees the same `(StoreId, SequencePosition)` twice, and by then the peer's dedup has already dropped real facts.

**Semver.** Additive in both directions, and free while `happenstance-sqlite` is unpublished. Whatever detects the split adds either a public error variant or a new failure mode to an existing one, both of which are permanent after 0.2.0.

**Remediation.** Subordinate, and pointed away from where the finding pointed. Guarding inside `remint_identity` cannot work: a process-wide open-path registry needs a canonical path key that symlinks, hardlinks, `file:` URIs, UNC paths and two paths to one inode all defeat, it needs a `Drop` impl on a `Clone` type whose clones share one connection (J-3+J-4's subject), and it covers only the case the finding is narrowest about. The place the split does harm is inside the append transaction, where the persisted identity is already readable under a lock the writer already holds.

**Routing.** Not a 0.2.0 blocker and stated as such — it is the smallest item in this section and the only one not about contention. `happenstance-sqlite`'s owner at phase 8, and it bears on VT-6's `[PROVISIONAL]` marker, whose falsifier is about Durable Object eviction rather than about this. The question it raises for whoever next reviews that marker — *whether an adapter taking VT-6's documented-procedure branch owes an in-process check as part of the permission* — belongs with the clause and not with the crate; `references/adapter-shapes.md`, which VT-6 requires every adapter to record its mechanism in, is where the answer would have to appear either way.

---

## Three wrong stores cleared all 89 event-store rules, and each one is a shape a queued adapter has

The conformance suite is this workspace's moat: an adapter does not exist until it has run `event_store_conformance!` and passed. That claim was tested rather than argued. Four stores this review named were compiled against the suite as it stands at `56ef6c5` and driven through every rule of the event-store family; three of them are genuinely wrong and **all three failed nothing**, while the same harness rejected each store's injected control twenty-one times over and both conformant controls passed clean. The suite is therefore sharp where it looks and blind on three named axes — forwards paging, an over-claimed capability, and the read path's error arm — and each of those axes is occupied by an adapter the RUNBOOK has queued. Two further entries are structural rather than behavioural: the projection probe's signature locks out the one adapter shape PS-2 `[FROZEN]` exists to require, and the testkit's own growth policy makes the next projection capability a source break in every out-of-tree fixture. The release position follows from CF-29 `[FROZEN]`: a rule added after `0.2.0` turns every passing adapter's CI red, so a rule owed at `0.2.0` is paid for once by the whole population, and a rule owed later is paid for by every adapter author individually.

> **The shared instrument.** `experiments/suite-against-wrong-adapters/` — not a workspace member, not a gate step (CF-34), eleven subjects driven through `for_each_event_store_rule!`'s 89 rules, `rustc 1.97.1`, `--release`, at this commit. Conditions in `results/raw/conditions.txt`; the answer line is `results/raw/census.txt:412`. Four entries below cite it; three do not, and say so.

---

### L1-1 — no rule composes forwards `from` with `limit`, and a store that ignores the caller's budget on exactly that read is certified

**Clause:** **CF-12** — *At least one rule MUST compose `ReadOptions::from` with a multi-item query* (`spec/SPECIFICATION.md:7794-7795`), **`[FROZEN]`**; **ES-14** — *`limit` truncates the whole result, after ordering* (`spec/SPECIFICATION.md:3169-3175`), **`[FROZEN]`**; **VT-28** — *`limit(0)` returns nothing* (`spec/SPECIFICATION.md:1740-1745`), **`[FROZEN]`**.

**Examined.** Every use of `ReadOptions::from` in the event-store rule set: `crates/happenstance-testkit/src/suite.rs:897, 979, 1134, 1242, 1509, 1520, 1537, 1548` — eight call sites in five rules (`read_from_is_inclusive` `:883`, `read_backwards_from_with_limit` `:956`, `read_from_composes_with_multi_item_query` `:1118`, `read_from_and_to_bound_a_closed_window` `:1227`, `read_from_a_gap_position` `:1490`). Against them, `spec/SPECIFICATION.md:7810-7818` (CF-12's discharge) and the census at `experiments/suite-against-wrong-adapters/results/raw/census.txt:41-47`.

**Found.** The combinations issued are `from` alone, `from`+`to`, `from`+`backwards`, and `from`+`backwards`+`limit`. Forwards `from` composed with `limit` appears at no call site. CF-12's discharge paragraph reads as though it does —

> **Discharged at phase 3 stage 4, and the SHOULD was taken.** All three options are
> exercised against the same two-item query in the one rule
> (`spec/SPECIFICATION.md:7810-7811`)

— and the rule it names issues the three options as three separate reads, never composed forwards:

```rust
let resumed = read_ok(&store, &query, ReadOptions::new().from(anchor)).await;   // :1134
let reversed = read_ok(&store, &query, ReadOptions::new().backwards()).await;   // :1143
let newest = read_ok(&store, &query, ReadOptions::new().backwards().limit(2)).await; // :1155
```

The suite's own map of this coverage is also stale. `read_from_a_gap_position`'s rustdoc states:

> `read_from_is_inclusive` and `read_backwards_from_with_limit` are the only
> two rules that pass `from` at all, and both hand it a position the store
> actually assigned. (`suite.rs:1475-1477`)

Five rules pass `from` at this commit, not two.

**Measured.** `ForwardPagingBudgetStore` — `limit` applied only when `from` is absent, forward resume branch returning the whole tail, backwards branch left correct — was driven through all 89 rules:

```
SUBJECT ForwardPagingBudgetStore [Mutant]
  89 rules: 83 passed, 6 skipped, 0 FAILED
```
(`results/raw/census.txt:41,45`)

That is byte-identical to both conformant controls, `MemoryEventStore` and `LogStore`, including the six skips. The defect is visible through the port with no rule involved: `defect_is_real.rs::forward_paging_budget_is_ignored` appends five events, issues `from(2).limit(2)`, and gets **four** events back where the correct core returns two; the same store's `from(4).backwards().limit(2)` returns two, so the store is not simply broken at paging.

**Why a defect.** Not that a combination is missing — combinations are infinite — but that this one is named by three `[FROZEN]` clauses as the consumer that motivates them. VT-28's `Rejects:` is written entirely about it: *"a paging loop writing `.limit(budget - fetched)` that reaches parity did not read zero events, it read **the entire log, unbounded, silently**"* (`spec/SPECIFICATION.md:1750-1753`). A budget loop that resumes carries `from`. `ReadOptions`' own rustdoc documents the idiom — *"which is what makes `.limit(budget - fetched)` safe at parity"* (`crates/happenstance-core/src/query.rs:259-261`). And the branchy adapter that fails it is not hypothetical: it is the shape that arises when `from` is threaded into a resume path written after the paging path, which is the order every SQL adapter in this workspace will be written in. **One correction to the theme as filed:** the workspace's own runner does not issue this read. `run_projection` sets `from` and no limit, deliberately — *"One read for the whole run"* (`crates/happenstance/src/runner.rs:444-452`) — which is why nothing in-tree has met the defect and why the suite is the only thing that would.

**Semver.** Adding a rule is semver-minor on `happenstance-testkit` and a break in practice; the crate says so itself: *"Adding a rule is a semver-minor change that can turn a passing adapter's CI red. Treat that as a breaking change in practice and pin this crate exactly"* (`crates/happenstance-testkit/README.md:209-211`), and CF-29 `[FROZEN]` (`spec/SPECIFICATION.md:8625-8628`) makes it policy. `0.2.0` is the last cheap moment in the only sense that matters here: it is the release that creates the adapter population, so a rule landing with it costs nobody a red build, and the same rule landing at `0.2.1` costs every adapter one.

**Remediation, subordinate to the clause.** One rule in the `// Read options` group modelled on `read_from_and_to_bound_a_closed_window`, one registered mutant (`ForwardPagingBudgetStore` already exists, compiled, at `experiments/suite-against-wrong-adapters/src/stores.rs`), and the CHANGELOG entry CF-29 requires. Whether it is one rule or two — `from`+`limit` and `from`+`to`+`limit` are different statements — is the rule author's call and not this document's.

**Routing.** The rule-set owner at phase 3's standing obligation, taken before phase 12 (*Publish `0.2.0`*, `RUNBOOK.md:163`). CF-12 `[FROZEN]` is not amended by this: its MUST is satisfied. What is owed is the sentence at `spec/SPECIFICATION.md:7810-7811` and the rustdoc at `suite.rs:1475-1477`, both of which describe coverage the code does not have, and the repointing of those is the one in-place change this document's lifecycle permits.

---

### L2-01 — the newest read option is the least covered in both instruments, and the enum that would carry it into the generative one cannot grow without a break

**Clause:** **ES-16** — *An upper bound on a read* (`spec/SPECIFICATION.md:3289-3295`), **`[FROZEN]`**; **CF-12** (`spec/SPECIFICATION.md:7794-7795`), **`[FROZEN]`**. No clause governs the model family's generator — see the corroboration below.

**Examined.** `crates/happenstance-testkit/src/model.rs:152-179` (`Op`), `:204-212` (`any_op`), `:290-310` (`Model::select`), `:464-479` (`Model::apply`'s Read arm); every `ReadOptions::to` call site in the rule set, `crates/happenstance-testkit/src/suite.rs:1198, 1242, 1286`; the three `to` mutants at `crates/happenstance-testkit/tests/mutation_coverage.rs:645-696`; `crates/happenstance-testkit/src/lib.rs:370-435`.

**Found.** `Op::Read` is documented as carrying every read option and carries four of five:

```rust
    /// Read, with every read option in play.
    Read {
        /// What to match.
        query: Query,
        /// The inclusive `from` bound.
        from: Anchor,
        /// Whether to read newest-first.
        backwards: bool,
        /// A truncation, if any. Never zero — `ReadOptions::limit(0)` is
        /// `None` today and VT-28 owns changing that, in phase 4.
        limit: Option<usize>,
    },
```
(`model.rs:167-179`)

`any_op` generates no `to` (`model.rs:208-211`) and `Model::apply` never calls `.to(..)` (`model.rs:469-479`), so the two `to` branches of `Model::select` are dead — and the comment above them states exactly why that matters:

> The model carries `to` because a
> model that ignores an option cannot disagree with a store that
> mishandles it — it would agree with every implementation, which is the
> one thing a reference model must not do. (`model.rs:292-295`)

The enumerated suite does not make up the difference. All three `to` rules issue `Query::all()` (`suite.rs:1198`, `:1242`, `:1286`), so `to` is never composed with a filtering query — the identical gap CF-12 measured for `from` (*"no read option is exercised against a filtering query at all"*, `spec/SPECIFICATION.md:7802-7803`) and closed for `from` only. The mutant registry confirms the shape of the hole: `UnparenthesisedPredicateStore` — `WHERE a OR b AND position >= ?` — is registered and fails exactly one rule, `read_from_composes_with_multi_item_query` (`mutation_coverage.rs:753-756`). Its `to` twin, `WHERE a OR b AND position <= ?`, would fail nothing: the three registered `to` mutants (`ToBoundIgnoredStore`, `ToIsExclusiveStore`, `BackwardsToIsAnUpperBoundStore`, `mutation_coverage.rs:645-696`) are all single-predicate defects that a `Query::all()` read catches.

Separately, `Op` is declared `#[derive(Debug, Clone)] pub enum Op {` (`model.rs:151-152`) with no `#[non_exhaustive]` at either level. The testkit itself owns the compile test for the opposite decision one crate over, with a `compile_fail` doctest, a twin, and a `Rejects:` line, and states the reason in terms: *"variant-level `#[non_exhaustive]` buys over enum-level: matching is allowed and construction is not"* (`lib.rs:421-423`).

**Why a defect.** ES-16 `[FROZEN]` states the deadline itself: *"**If `to` landed at all it had to land before the first adapter shipped**, and none has"* (`spec/SPECIFICATION.md:3302-3303`). At `0.2.0` one ships. The wrong outcome is a Postgres or Neon adapter that builds its `WHERE` clause by string concatenation and gets the upper bound's precedence wrong across a disjunction — the same class as the mutant already in the registry for `from`, one bound over, invisible to every rule and to the generative model alike, and manifesting as a bounded backfill worker that reads past its window and re-delivers events the tail worker has already processed. RS-13-5 is not violated by the missing attribute at the *enum* level — `Op`'s doc says the three variants are the whole port surface (`model.rs:145-148`) and that is a correct application of the atom — but `Op::Read`'s *field set* is designed to grow, and the crate's own note in the `limit` doc says a second growth is already owed to VT-28's phase.

**Semver.** Adding a field to a struct-form variant of a `pub enum` with no `#[non_exhaustive]` is **breaking** for any downstream `match` written with a struct pattern and for any construction. `Op` is reachable as `happenstance_testkit::model::Op` whenever the `proptest` feature is on (`lib.rs:347-349`), on a crate already published at `0.2.0-alpha.1`. Applying the attribute is itself breaking, for the same reason. Both are free exactly once, at `0.2.0`.

**Remediation, subordinate to the clause.** Variant-level `#[non_exhaustive]` on `Op::Read`, a `to: Anchor` field, one `prop::sample::select` arm in `any_op`, and one `.to(..)` in `Model::apply`. **One half of the finding as filed must be dropped and is:** the wrong outcome "applies `to` after `limit`" is not observable through the port, because `to` and `limit` commute exactly — in read order `from` cuts the front and `to` and `limit` both cut the back, which is what `correct::ordered` encodes. The surviving wrong outcome is `to` against a multi-item query, and it is the one an adapter meets.

**Routing.** Two decisions, one owner each. The generator hole belongs to the rule-set owner alongside L1-1, before phase 12. The `#[non_exhaustive]` question belongs to whoever owns the testkit's public surface at phase 12 (`RUNBOOK.md:163` — *"`cargo-semver-checks` reporting against a registry baseline"*), because that is the step that will notice it and the last one that can act cheaply. **This corroborates a recorded open question:** `.kb/open-questions/model-family-rule-has-no-clause.md` (status `accepted`, last reviewed 2026-08-10, held in `UNCLAIMED_PENDING_ADR` at `xtask/src/spec_trace.rs:1968-1997`) records that `ops_agree_with_the_model` belongs to no clause and lists the seven it composes — *"ES-8, ES-9, ES-11, ES-14, ES-15, ES-18 and ES-25"*. **ES-16 is not among them**, and that omission is precisely this finding: the atom's own enumeration of what the model checks was written against a generator that cannot reach `to`. The open question is still costing, and this is what it costs.

---

### L1-2 — the capability model checks under-claiming and not over-claiming, and a fixture that lies about `REOPEN` scores higher than one that tells the truth

**Clause:** **CF-14** — *A rule MUST assert that an append acknowledged with `Ok` survives a reopen* (`spec/SPECIFICATION.md:7876-7878`), **`[DEFERRED]`**; **CF-17** — the `REOPEN` capability (`spec/SPECIFICATION.md:7992-7994`), **`[PROVISIONAL]`**; **ES-35** (`spec/SPECIFICATION.md:4208-4213`), **`[PROVISIONAL]`**. The closed analogue is **CF-39** (`spec/SPECIFICATION.md:8063-8068`), **`[PROVISIONAL]`**.

**Examined.** `crates/happenstance-testkit/src/contract.rs:184` (`const REOPEN: Capability;`), `:343-364` (the provided `reopen` body and its `# Panics` doc), `:611-640` (`COMMIT_FAULT`'s doc and constant), and `experiments/suite-against-wrong-adapters/results/raw/census.txt:49-53`.

**Found.** The whole guard is a panic in a body that an over-claiming fixture never reaches, and the trait's own rustdoc says so:

> Nothing in the trait ties
> `REOPEN: Capability::SUPPORTED` to overriding this method, so the
> reachable path an adapter author actually hits is: declare it supported,
> forget the override, run the suite. (`contract.rs:344-347`)

The identical hazard one capability over is closed by a MUST written into the clause and restated on the constant:

> So a store that can
> absorb every fault its fixture is able to arm MUST **decline** this
> capability with that as its stated reason, rather than declare it and
> contribute an `Ok`. (`contract.rs:636-639`, and CF-39 verbatim at `spec/SPECIFICATION.md:8066-8068`)

`REOPEN` has no such sentence anywhere. CF-17's own `Rejects:` line concedes it carries none — *"Rejects: nothing on its own — it is an enabling clause, and CF-14 carries the rejection"* (`spec/SPECIFICATION.md:8015-8016`) — and CF-14's rejection is delegated entirely to the fixture: it names *"a pooled rusqlite store doing its work in `spawn_blocking` and returning on the join, a Durable Object relying on output-gate semantics it does not actually have, any store with `PRAGMA synchronous = OFF`"* (`spec/SPECIFICATION.md:7893-7896`), none of which the rule can see unless the fixture's `reopen` genuinely closes something.

**Measured, and sharper than filed.** `NoopReopenFixture` — `REOPEN: Capability::SUPPORTED`, `async fn reopen(&self) {}`, over a correct but entirely volatile `Vec`:

```
SUBJECT NoopReopenFixture [Mutant]
  89 rules: 86 passed, 3 skipped, 0 FAILED
  skipped: ["append_is_atomic_under_a_mid_batch_fault",
            "arming_a_mid_batch_fault_makes_the_append_fail",
            "append_reports_exceeded_store_limits"]
```
(`results/raw/census.txt:49,52-53`)

Every other subject in the run skips six. This one skips three, because the three it converts into passes are exactly the workspace's whole durability certification: `acknowledged_writes_survive_a_reopen`, `reopened_store_does_not_reissue_an_event_id`, `recorded_time_survives_a_reopen`. `defect_is_real.rs::a_noop_reopen_closes_nothing` reads the event back *through the handle taken before the reopen*, which a store that had genuinely closed could not answer from.

**Why a defect.** The incentive runs backwards. An honest volatile fixture declines and reports `83 passed, 6 skipped`; a fixture that over-claims reports `86 passed, 3 skipped` and looks *better*. CF-18 exists to stop a decline from being a silent pass and it works; nothing plays the same role in the other direction. The named wrong outcome: an adapter author over a connection pool that "handles reconnection" declares `REOPEN` supported and writes an empty `reopen` — the honest-looking answer — ships a crate whose README says it passes `acknowledged_writes_survive_a_reopen`, and ES-35's durability claim has never been driven across a process boundary. Who finds out: the first operator to restart the service.

**Semver.** Additive to `happenstance-testkit`'s trait surface (a defaulted mechanism method and a capability's stated obligation). The *rule* that would enforce it carries CF-29's practical break, so the same `0.2.0` arithmetic as L1-1 applies. `happenstance-testkit` is already on crates.io at `0.2.0-alpha.1`.

**Remediation, subordinate to the clause, and honestly bounded.** A complete fix is structurally blocked and this should be recorded rather than promised. `MID_BATCH_FAULT` is closable because arming it has a port-observable consequence — the append must answer `Err`. `REOPEN` has none: a correct `reopen` over a durable medium and an empty one over a `Vec` produce byte-identical observations through `EventStore`. So no rule written against today's `Fixture` surface can reject `NoopReopenFixture`. What is available is CF-39's other half — a clause-level MUST on what declaring `REOPEN` commits a fixture to, plus a testkit-internal fixture registered as a `Defect` so the obligation has a named wrong implementation, plus the mechanism-statement requirement CF-39 already imposes. That converts the hazard from undetectable to *stated*, which is what CF-39 bought for the write path and no more.

**Routing.** A clause, not a code change: whoever owns CF-17's `[PROVISIONAL]` marker decides whether `REOPEN` acquires CF-39's declaration obligation, and CF-14's `[DEFERRED]` marker names the two implementations that have not answered — `happenstance-cloudflare` (HS-P0013) and `happenstance-neon` (HS-P0014), `spec/SPECIFICATION.md:7885-7888`. Both are the adapters whose "reopen" is least like closing a file, which is where an empty `reopen` is most tempting. Moving a maturity marker is an ADR's act (`CLAUDE.md`, and CF-17 says so at `spec/SPECIFICATION.md:8005-8007`); this document does not take it.

---

### L3-01 — the read path's error arm has no rule, no mutant and no fixture seam, and a store that reports a fetch failure as end-of-stream is certified

**Clause:** **ES-2** — *`read` returns the stream at the top level and is not `async`* (`spec/SPECIFICATION.md:2540`) governs the signature, and no clause governs the coverage of its `Err` arm. Stated explicitly: **no clause requires a rule to induce a read fault**, and the absence is what this entry is about.

**Examined.** The port at `crates/happenstance-core/src/store.rs:167-171`; the write-path fault seam at `crates/happenstance-testkit/src/contract.rs:218` (`MID_BATCH_FAULT`) and `:308` (`arm_mid_batch_fault`); `grep -rn 'read_fault|arm_read|READ_FAULT' crates/happenstance-testkit/src/ spec/SPECIFICATION.md` at this commit, which returns nothing; the registry's `Not covered` inventory at `crates/happenstance-testkit/tests/mutation_coverage.rs:259-329`; `results/raw/census.txt:56-62` and `:342-348`.

**Found.** The port expresses the failure per item:

```rust
    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;
```
(`store.rs:167-171`)

Nothing in the discriminator apparatus reaches that arm. `Fixture` carries `MID_BATCH_FAULT` and `arm_mid_batch_fault` for the write path and no read-path analogue; no rule induces a read fault; no mutant models one; and the registry's own six-axis `Not covered` list — the artefact ADR-0010 requires *in place of* a pass rate — does not name it (`mutation_coverage.rs:259-329`). The consumer half is already correct: `collect` returns `Poll::Ready(Err(err))` on a mid-stream `Err` (`store.rs:345-346`), so the hole is exactly where the finding puts it — nothing ever hands an adapter's read path a fault.

**Measured, and the diagnostic locates it precisely.** `SwallowedReadFaultStore` — `let Ok(page) = fetch().await else { return Poll::Ready(None) };` — fails **0 of 89** (`results/raw/census.txt:56,61`). The same store with the fault armed by hand at `open` fails **22**, including `query_all_matches_every_event`, `read_from_is_inclusive` and `event_ids_are_unique_within_a_store` (`results/raw/census.txt:342,346-348`). So the suite is **not** blind to the consequence; it has **no way to produce one**. The fixture's `arm_read_fault` had to be written as an inherent method no conformance rule can reach, and that asymmetry is the finding, measured.

**Why a defect.** The construct is not exotic — it is the most natural way to get a fallible fetch past a `poll_next` that must return a value, and both adapters that will need it are already in the tree with paged or asynchronous fetches: `happenstance-cloudflare` over `SqlStorage` and `happenstance-neon` over one-shot HTTP. The wrong outcome is silent and unrecoverable at the consumer: `defect_is_real.rs::a_swallowed_read_fault_looks_like_the_end_of_the_stream` puts five events in the store and `collect` returns `Ok` over **two**. A projection runner sees a short replay, commits the checkpoint at the truncation point, and the events above it are never applied — with `Ok` everywhere and no error to log. Who finds out: whoever reconciles the read model against the log, months later.

**Semver.** Additive in every part: a defaulted `const READ_FAULT: Capability` and a defaulted `fn arm_read_fault` on `Fixture` (defaulted, so no existing fixture breaks), one rule, one mutant. Nothing in `happenstance-core` needs to change and nothing in it may. The rule addition carries CF-29's practical break, and that is the whole of the `0.2.0` argument here.

**Remediation, subordinate to the clause.** Mirror the write side exactly: `READ_FAULT` + `arm_read_fault` defaulted-declined on `Fixture`, one rule asserting the stream yields `Err` rather than `None`, and `SwallowedReadFaultStore` registered — it is already compiled and driven through the port at `experiments/suite-against-wrong-adapters/src/stores.rs`. Whether the fault fires at a page boundary or at an item is the rule author's call.

**Routing.** Two owners. The rule-set owner takes the fixture capability and the rule, before phase 12. The specification owner decides whether a clause is minted for it — §7.4 (`spec/SPECIFICATION.md:9303`) is where a rule no clause names is disposed of, and CF-24's discipline is that a rule with no clause is as much a problem as a clause with no rule. This document names neither answer.

---

### F1-02 — `probe_read_through` is synchronous and infallible, and the only adapter shape PS-2 still needs cannot implement it

**Clause:** **PS-2** — *The port MUST NOT be frozen until … two adapters at opposite ends of the batch-shape axis have passed it* (`spec/SPECIFICATION.md:4864-4867`), **`[FROZEN]`**; **PS-12** (`spec/SPECIFICATION.md:5194-5197`), **`[PROVISIONAL]`**.

**Examined.** `crates/happenstance-core/src/projection.rs:583` (`READS_THROUGH_BATCH`), `:600-618` (`probe_read`, and its stated justification for being the one async member), `:624` (`probe_read_through`); every `const READS_THROUGH_BATCH` binding in the tree; `crates/happenstance-sqlite/src/projection_store.rs:708, 761-767`; `crates/happenstance-postgres/src/projection_store.rs:105`; `.kb/decisions/0036-the-projection-port-ships-gated.md:76-88`.

**Found.** The declaration:

```rust
    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
```
(`projection.rs:624`)

Not `async`, not fallible, `&Self::Batch` rather than `&mut`. Twenty-four lines above it, the sibling method carries the crate's own reasoning for why a probe that touches the store must be a future:

> The one asynchronous member, and not arbitrarily so: the other three
> mutate a batch the caller already holds, while this is real I/O against
> the store. (`projection.rs:602-604`)

`probe_read_through` is also real I/O against the store whenever the batch is a live transaction — and `Transaction<'static, Postgres>` is precisely what `PostgresProjectionStore` binds today (`crates/happenstance-postgres/src/projection_store.rs:105`). Reading through a `sqlx::Transaction` requires `.await` and `&mut`; answering synchronously inside an async runtime means blocking on it, which deadlocks a current-thread runtime and needs `block_in_place` plus a multi-thread one otherwise.

Every `READS_THROUGH_BATCH = true` in the workspace is an in-process store answering from a map or a buffer: `projection_memory.rs:383`, `fixtures.rs:371`, two mutation-coverage stores, two in `examples/outside-projection-adapter/src/lib.rs`, one test file. The **one adapter** that has implemented `ProjectionProbe` declares `false` and leaves the method unwritten:

```rust
    fn probe_read_through(&self, _batch: &Self::Batch, _key: &str) -> Option<u64> {
        unimplemented!(
            "SqliteBatch buffers its statements, so `READS_THROUGH_BATCH` is \
             `false` and this is never called: there is no open transaction to \
             read through, and answering from committed state is what PS-12 \
             forbids"
        )
    }
```
(`crates/happenstance-sqlite/src/projection_store.rs:761-767`, with `READS_THROUGH_BATCH = false` at `:708`)

So `probe_read_through` has never been implemented by any adapter, and PS-12's rule `batch_reads_reflect_pending_writes` has only ever executed against in-process stores.

**Why a defect.** PS-2 is `[FROZEN]` and names the unmet end explicitly: *"one adapter holding a live transaction (rusqlite or `sqlx`)"* (`spec/SPECIFICATION.md:4870-4871`). ADR-0036, accepted 2026-09-02, is the decision that the bar is unmet and that the port therefore ships gated — *"SQLite's `Batch` is an owned write set under `kb-decision-0017` … so it sits at the buffered end rather than the live-transaction end"* (`.kb/decisions/0036-the-projection-port-ships-gated.md:79-84`). The wrong outcome is not a runtime failure; it is that the adapter built to close PS-2 cannot answer the probe honestly, so it declares `READS_THROUGH_BATCH = false` — **a false statement about the store**, because a Postgres transaction reads its own uncommitted writes by construction — takes `batch_reads_reflect_pending_writes` as a reported skip, goes green, and PS-2's part 2 is recorded as met by an adapter whose defining property was never exercised. There is already drift in that direction: the story spec that will build the Postgres projection store proposes a `Vec`-of-statements batch and lists *"makes `READS_THROUGH_BATCH = true` reachable by consulting the buffer before the table"* among its reasons (`.bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-projection-store/spec.md:585-587`) — which puts the adapter at the buffered end again, beside SQLite, which is the monoculture PS-2's `Rejects:` paragraph names.

**Semver.** Breaking to `ProjectionProbe`, and **exempt**: the trait is behind `conformance`, which declares `conformance = ["unstable-projection"]` (`crates/happenstance-core/Cargo.toml:97`), and the module documents the exemption — *"exempt from this crate's semver promise for as long as the feature exists"* (`crates/happenstance-core/src/projection.rs:7`). This is the one entry in this section that is genuinely cheap after `0.2.0`. What is *not* cheap later is the ordering: the exemption ends when the port freezes, and the port cannot freeze until an adapter that cannot implement this method has passed.

**Remediation, subordinate to the clause.** Give `probe_read_through` the shape `probe_read` already has — hand-desugared RPITIT, fallible, no `Send` bound (which `probe_read`'s doc at `projection.rs:606-614` argues for at length and the same argument reaches here unchanged) — and take the batch by `&mut`, because a batch that *is* a live transaction must both `.await` and mutate to answer a read. The falsifying instrument is a `ProjectionProbe` impl whose `Batch` is an owned handle onto an async, fallible key-value store; a `tokio::sync::Mutex<HashMap<_, _>>` is enough, and it does not need a database.

**Routing.** Phase 6 (*Freeze `ProjectionStore`*, `RUNBOOK.md:156`) owns the signature, and phase 10 (*Postgres and Neon*, `RUNBOOK.md:161`) is where the cost is paid if it is not changed first. The decision — whether the probe's shape is amended before the live-transaction adapter is written, or the adapter is written against the shape and PS-2's part 2 is judged on a declined capability — belongs to whoever owns PS-2's `[FROZEN]` clause. **This corroborates ADR-0036 rather than contradicting it:** the ADR treats the unmet far end as adapter *scarcity* — a thing nobody has built yet — and this entry says the scarcity has a structural cause in the port's own signature. `grep probe_read_through` across `spec/`, `.kb/` and `references/` at this commit returns nothing outside the port and the backlog, so the cause is not recorded anywhere the freeze decision would read it.

---

### C2-04 — `ProjectionFixture`'s stated declension policy makes every future capability `error[E0046]` in every out-of-tree fixture, and §6.6 names no such class

**Clause:** **§6.6, the testkit's compatibility policy** — **CF-29** `[FROZEN]`, **CF-30** `[NON-NORMATIVE]`, **CF-31** `[FROZEN]`, **CF-32** `[FROZEN]` (`spec/SPECIFICATION.md:8618-8709`). The class this entry names is governed by none of them, and that is the finding.

**Examined.** `crates/happenstance-testkit/src/contract.rs:536` (`pub trait ProjectionFixture`), `:575` (`SECOND_HANDLE`), `:588-604` (`RESET_REFUSAL` and its policy statement), `:617-640` (`COMMIT_FAULT` and its restatement); `crates/happenstance-testkit/src/lib.rs:350-355`; `crates/happenstance-testkit/Cargo.toml:47-53`; `examples/outside-projection-adapter/tests/support/mod.rs:39-56, 85-95`; §6.6 in full.

**Found.** The policy is stated on the constant, twice, and it forbids defaulting:

> It is **required rather than defaulted**, unlike
> [`Fixture::MID_BATCH_FAULT`], and that is a deliberate difference of one
> line per fixture. A default would have to carry a *testkit-written*
> reason, and the projection family's declension policy is that the fixture
> writes the reason — a store's account of a trade only it can describe.
> (`contract.rs:588-593`, restated for `COMMIT_FAULT` at `:619-624`)

All three of `SECOND_HANDLE` (`:575`), `RESET_REFUSAL` (`:604`) and `COMMIT_FAULT` (`:640`) are required with no default. §6.6 contemplates rule addition (CF-29), rule meaning-change (CF-31) and the version key (CF-32). It does not contemplate the trait growing a required item. ADR-0036 is explicit that the port is expected to move — that is why it ships gated at all — so the next declinable capability the projection suite needs lands, by the crate's own stated policy, as a fourth required associated const.

**Why a defect.** Not the policy, which is well argued — *"why a particular store cannot make a commit fail* is not [the same sentence for every store]" (`contract.rs:622-627`) is a real distinction and the reason a testkit-written default would be worse. The defect is that the policy's consequence is unstated in the one document that governs what a testkit release may do to an adapter. The wrong outcome is concrete and already instrumented: `examples/outside-projection-adapter` exists precisely to be a stranger's crate, and it carries **two** `ProjectionFixture` impls (`tests/support/mod.rs:39` and `:85`). A fourth required const is `error[E0046]: not all trait items implemented` in both, and in every out-of-tree fixture, on a *minor* bump — which is the version an adapter author who followed the crate's own advice (*"pin this crate exactly"*, `README.md:210`) will nonetheless meet the day they unpin. The instrument that proves it is one line on a scratch branch: add `const NEW_THING: Capability;` to `ProjectionFixture` and check `examples/outside-projection-adapter`.

**Semver.** Additive today; the class is a **source break** on the next capability. And a semver characterisation the owner needs before deciding: ADR-0036's exemption names `happenstance-core` and `happenstance`, forwarded as `projection-store` on adapters (`.kb/decisions/0036-the-projection-port-ships-gated.md:57-62`). It does not name `happenstance-testkit`, whose `pub mod projection;` is **unconditional** — *"Unconditional, unlike its two nearest templates"* (`lib.rs:350-355`) — and which turns `unstable-projection` on for core unconditionally in its own manifest (`Cargo.toml:47-53`). So `ProjectionFixture` is ordinary, un-exempt, published API of a crate already on crates.io at `0.2.0-alpha.1`. Whether the exemption is intended to reach it is a question this document does not answer.

**Reshaping, recorded.** The finding is broader than filed. The base `Fixture` trait already has this shape today, not as a future risk: `SECOND_HANDLE` is a MUST under CF-16 with no default (`contract.rs:172`) and `REOPEN` is required (`:184`), so both families sit in the same bucket. `MID_BATCH_FAULT` is the exception that proves it — defaulted, and CF-39 says the default is deliberate (*"it is defaulted on the trait, so a fixture that never mentions it declines it"*, `spec/SPECIFICATION.md:8056-8058`).

**Remediation, subordinate to the clause.** Not a code change. A clause in §6.6 naming the class — a testkit release that adds a required associated const to a fixture trait is what CF-31 calls a major, or it is a case §6.6 accepts and says so — and, if the policy is to hold, a sentence on `ProjectionFixture` telling a fixture author that a minor bump may require a new line. Whether the policy should bend instead (a default carrying a testkit-written reason, which is exactly what the doc argues against) is the trade, and it is the owner's.

**Routing.** The specification owner, for §6.6. Phase 6 (`RUNBOOK.md:156`) is where the projection port's capability set next moves, and phase 12's `cargo-semver-checks` baseline (`RUNBOOK.md:163`) is what will report it as `trait_missing_associated_constant` once there is a registry baseline to compare against — which is the first release at which the tooling can see this at all.

---

### M-4 — CF-24's orphan scanner covers two of five enumerations, and the reason the clause gave for deferring the other three has been overtaken

**Clause:** **CF-24** — *No rule may exist in the suite without appearing in the enumeration* (`spec/SPECIFICATION.md:8395-8396`), **`[FROZEN]`**.

**Examined.** `crates/happenstance-testkit/src/registry.rs:352-366` and `:409-430`; `crates/happenstance-testkit/src/projection.rs:1994-2008` and `:2036-2062`; the five enumerations — `for_each_event_store_rule!` (`registry.rs:94`), `for_each_projection_store_rule!` (`projection.rs:1884`), `for_each_concurrency_rule!` (`concurrency.rs:1076`), `for_each_model_rule!` (`model.rs:712`), `for_each_event_store_benchmark!` (`bench.rs:726`); `CONTRIBUTING.md:224-241`. **Not measured** — `experiments/suite-against-wrong-adapters/` drives one family and says nothing about this.

**Found.** CF-24 records the gap and the reason it was left:

> The mechanical fix is cheap and deliberately not taken yet —
> extend `declared_rules()` to scan the two modules against their own
> enumerations, behind the same `#[cfg(test)]` — because a third family is
> plausible in phase 4 and one scanner written against three is better than three
> written one at a time. (`spec/SPECIFICATION.md:8412-8417`)

A third family did arrive. It got a **second scanner**, `no_orphan_projection_rules` (`projection.rs:2036`), whose `declared_projection_rules` (`:1995-2008`) is `registry.rs`'s `declared_rules` (`:353-366`) verbatim but for the `include_str!` argument and the function name. The clause's stated rationale for waiting is therefore spent, and the residual is three enumerations rather than two: `for_each_concurrency_rule!` (5 rules, `concurrency.rs:379, 468, 598, 669, 767`), `for_each_model_rule!` (1, `model.rs:598`) and `for_each_event_store_benchmark!` (3 scenarios, `bench.rs:532, 574, 638`). CONTRIBUTING carries the same overtaken description at the place a rule author meets it — *"that check `include_str!`s `suite.rs` and nothing else"* (`CONTRIBUTING.md:235-236`), true when written and false at this commit.

**Why a defect.** CF-24's own `Rejects:` names the outcome verbatim: *"a rule written, reviewed, merged, and never run because its registration line was forgotten"* (`spec/SPECIFICATION.md:8420-8421`). It is not caught by the adjacent check either, and CONTRIBUTING says why: *"`the_concurrency_rules_reject_exactly_what_they_claim` does not close it — that test enumerates *from* the same macro, so a rule missing from the list is missing from the check too"* (`CONTRIBUTING.md:237-239`). The concurrency family is where it costs most: five rules, driven on real threads, the only place a lost update between two OS threads is expressible, and the family that phase 10's non-serialising Postgres store exists to be measured by. A sixth rule added there and left out of the macro compiles, `cargo xtask ci` is green, and nothing runs it.

**Semver.** None. `#[cfg(test)]` throughout; no published item changes.

**Remediation, subordinate to the clause.** Generalise once rather than a fourth time — `fn declared_rules_in(source: &'static str, prefix: &str)` in `registry.rs`, with the call sites placed inside the files they scan so `include_str!` resolves relative to the scanning module. The benchmark enumeration takes a different prefix (`pub async fn` under `pub mod scenarios`) and CF-34 governs whether a benchmark scenario is "a rule" for this clause's purposes; that is a question for the clause, not for the scanner.

**Routing.** A `[FROZEN]` clause whose stated reasoning is stale, which is the specification owner's, not an implementer's — and the fix is not the whole of it: the clause's deferral paragraph and `CONTRIBUTING.md:235-236` both describe a tree that no longer exists, and repointing them is the one in-place change this document's lifecycle permits. This is filed as **corroboration**: CF-24 records the gap itself and this entry adds only the thing the clause could not know — that the third family arrived, took a duplicated scanner, and left the rationale spent. Severity is deferred by that reading, and it is the only entry in this section that does not bear on `0.2.0`.


---

## Every path an outside adapter author is told to type is API the crate declines to call API

`happenstance-testkit` is the moat: an adapter does not exist until it has run the suite, and CF-23 `[FROZEN]` makes the per-test wrapper an *adapter-supplied parameter*, which makes the twelve wrappers this crate ships the only concrete instances of that parameter anyone has. All twelve, plus `__emit_rule_names`, carry `#[doc(hidden)]` immediately above `#[macro_export]` — so the extension point is named in a table on the crate's front-door rustdoc page, written into a copy-pasteable example, invoked across a crate boundary inside this workspace, and rendered nowhere. Two of the five suite macros reach past the `__private` module the crate documents as the mechanism for exactly this, freezing `concurrency::` and `bench::` as paths a caller who wrote one line now depends on. The onboarding page's account of the adapter's feature graph is wrong in both directions at once; the one crate in the tree built to be an outside author does not write the manifest the recipe specifies; nothing ties adding a rule to the version bump the crate's only versioning statement promises; and the concurrency module tells a runtime-free adapter it must bring `tokio`. None of this is a design error — the emitter-as-parameter design is right, and the reasons for `#[doc(hidden)] #[macro_export]` are mechanical and recorded. It is a surface the crate has not yet decided to *have*. At `0.2.0` it acquires one by default, and the CI job that would notice a later break is the one the attribute switches off.

---

### C2-03 — The extension point CF-23 makes mandatory has no rendered page, no stability statement, and no semver instrument

**Clause:** **CF-23** — *the testkit MUST NOT emit any runtime-specific attribute from its own expansion; the per-test wrapper MUST be a parameter supplied by the adapter* (`spec/SPECIFICATION.md:8363-8374`). **`[FROZEN]`**. §6.6, the crate's compatibility policy (`spec/SPECIFICATION.md:8618-8700`), governs rule addition (CF-29), rule meaning-change (CF-31) and the version key (CF-32), and says nothing about the emitters.

**Examined.** The thirteen `#[doc(hidden)] #[macro_export] macro_rules!` items — `registry.rs:226,246,274,291`, `projection.rs:1925,1946,1973`, `model.rs:729,746`, `concurrency.rs:1100,1123`, `bench.rs:749,770`, all in `crates/happenstance-testkit/src/`, every one of them the same three lines in the same order. Against them: `crates/happenstance-testkit/src/lib.rs:60-77`, `crates/happenstance-testkit/README.md:207-211`, `crates/happenstance-testkit/src/lib.rs:7`, `crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64-68`, and `.github/workflows/ci.yml:346-375`.

**Found.** The page tells the reader to write the name (`crates/happenstance-testkit/src/lib.rs:60-67`):

```text
//! mean every new runtime needs a testkit release. Three emitters ship, and all
//! three are demonstrated in this crate's `tests/`:
//!
//! | Emitter | Wrapper | Adapter needs |
//! |---|---|---|
//! | `__emit_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | `__emit_blocking` | `#[test]` + [`block_on`] | nothing |
//! | `__emit_wasm` | `#[wasm_bindgen_test]` | `wasm-bindgen-test` |
```

and then hands them a program to copy (`:69-77`), `emit = happenstance_testkit::__emit_blocking`. The workspace has already taken the instruction across a crate boundary — `crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64-68`:

```rust
happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_wasm,
    emit = happenstance_testkit::__emit_wasm,
    fixture = CloudflareFixture::new()
);
```

The crate's entire statement about its own stability is three sentences in `README.md:207-211` (*"Adding a rule is a semver-*minor* change… pin this crate exactly"*), and `src/lib.rs:7` is `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` — the README is included **only** under `cfg(doctest)`, so those three sentences never reach docs.rs, and they are about the rule set rather than about the macros.

**Why a defect.** Not "an item is undocumented" — undocumented items are ordinary. `#[doc(hidden)]` plus a `__` prefix is Rust's universal declaration that an item is not public API and may change without notice, and the crate is applying that declaration to the one thing CF-23 requires every adapter author to name. The two readings are irreconcilable and the crate publishes both. The failure lands on the population CF-20 and CF-23 exist to serve: a Workers author reads the page, is told to write `__emit_wasm`, searches the rendered API for it, finds no item, no signature and no stability statement, and cannot tell whether they are allowed to depend on it. The reverse direction is worse and is measurable: `.github/workflows/ci.yml:346-375` runs `cargo-semver-checks` on every pull request, and `#[doc(hidden)]` is the marker that tool uses to exclude an item from its analysis — so the one instrument in this repository that would report a rename of `__emit_wasm` as breaking is the instrument the attribute turns off. That job also states its own second limit at `:369-373`: it compares against the pull request's base, so *"it says nothing about breaks against the last release"*.

**Semver.** The finding is none — nothing changes shape. A fix is additive: a stability paragraph in §6.6 and on the rendered page, and, if the emitters are to be supported, documented aliases. `0.2.0` is not the last cheap moment in the semver sense; it is the moment the emitters ship a **second** time, either with a stability statement or as items the crate says may change without notice. The decision that is genuinely dated is the other arm — declaring them unstable and renaming them later costs nothing today and is a major release afterwards.

**Remediation.** The obvious move — seal them — is unavailable, and the reason is mechanical rather than a matter of taste: `macro_rules!` lives in a flat crate-root textual namespace and a private helper is unreachable from a downstream expansion site (`references/evaluation/review-conformance-suite.md:356-364`), which is why the shape exists at all; `crates/happenstance-cloudflare/tests/durable_object_conformance.rs:64-68` is the in-tree proof that the cross-crate reach is already load-bearing. What remains is a written statement of what the twelve names promise, and whether they promise it. Which arm is right is not this document's call.

**Routing.** Two places, and they are already scheduled. The stability statement is a §6.6 clause — spec authorship, owned by the runbook's specification pass, not by a line edit. The instrument gap belongs to **HS-S0091** `registry-surface-diff` (`.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/spec.md`, stage `plan`), whose B12 already requires the report to name what `cargo-semver-checks` does not check; `#[doc(hidden)]` exclusion is a fourth blind spot for that list, and the emitters are what makes it material. The rendered-page half is **HS-S0094** `guarantees-and-docs-rs-presentation`, whose AC-006 is *"no feature-gated public item is invisible"* — a sentence about `doc(cfg)` pills today that this finding says needs a second half.

---

### M-3 · M-5 — The crate states an absolute about macro-expansion paths, and its own macros break it in both directions

**Clause:** **No clause governs this**, and that is the finding's shape rather than an omission in the citation. CF-22 `[FROZEN]` (`spec/SPECIFICATION.md:8218-8220`) governs where the rule set is enumerated; CF-23 `[FROZEN]` governs the emitter parameter; §6.6 governs rules, meanings and the version key. Nothing in the corpus says what a `#[macro_export]`ed macro body is permitted to name, which is why the discipline lives in a doc comment and a changelog entry instead.

**Examined.** `crates/happenstance-testkit/src/lib.rs:243-249` and `:626-631`; the five exported suite macros' expansions at `concurrency.rs:1176`, `bench.rs:845` and `:852`, `:868`, `:884`, `:892`; `lib.rs:323` and `:332`; `concurrency.rs:186`; `bench.rs:106`; `crates/happenstance-sqlite/tests/concurrency.rs:62`; `CHANGELOG.md:1590`; and, for the second half, `crates/happenstance-testkit/src/projection.rs:214-224` against its two siblings at `:136` and `:184`.

**Found.** The discipline, stated as an absolute on the onboarding page (`lib.rs:243-249`):

```text
//! That line expands in *your* crate, which is why the expansion never assumes
//! what you have in scope: it spells the fixture trait as
//! `$crate::__private::ProjectionFixture`, through a hidden module this crate
//! keeps for the purpose, so the invocation works whether or not you imported
//! the trait and whatever you renamed the dependency to.
```

The module it names (`lib.rs:626-631`):

```rust
/// Re-exports the macro expansions need to name, so an adapter is not required
/// to have this crate in scope under that exact name.
#[doc(hidden)]
pub mod __private {
    pub use crate::contract::{Fixture, ProjectionFixture};
}
```

Two items, and four fixture-shaped types exist. `event_store_concurrency_conformance!` reaches around it (`concurrency.rs:1176`):

```rust
            async fn __conformance_fixture() -> impl $crate::concurrency::ConcurrentFixture {
```

and `event_store_benchmarks!` does both things eight lines apart — `bench.rs:845` is `impl $crate::__private::Fixture`, and `bench.rs:852` is:

```rust
            fn __benchmark_params() -> $crate::bench::BenchmarkParams {
```

Neither `ConcurrentFixture` (`concurrency.rs:186`) nor `BenchmarkParams` (`bench.rs:106`) is re-exported at the crate root: `lib.rs:359-368` carries `Capability, Fixture, …, ProjectionFixture, RuleOutcome`, `FaultyStore`, `GappyMemoryStore`, `block_on` and `rules`, and nothing from `concurrency` or `bench`. The claim that the surface is complete is on the record — `CHANGELOG.md:1590`: *"why the expansion reaches the fixture trait through a hidden `__private` module you never name. **No item became `pub` and no rule changed**, so this is not a MINOR event"*. And the second direction, the one unqualified path in twenty-six macros (`projection.rs:216`):

```rust
        if !<<$fixture as $crate::ProjectionFixture>::Store as ProjectionProbe>::READS_THROUGH_BATCH
```

`ProjectionProbe` resolves against `projection.rs`'s own `use`, not against the expansion site — while `must!` (`:136`) and `require!` (`:184`), thirty and eighty lines above, qualify everything they name.

**Why a defect.** The stated rule is not aspirational: it is the reason `__private` exists, and it is what `CHANGELOG.md:1590` cites to classify a documentation change as not-MINOR. Two of five suite macros are outside it, and the consequence is not that a caller might name a wrong path — it is that a caller who named **no** path at all now depends on two. `crates/happenstance-sqlite/tests/concurrency.rs:62` is one line, `happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new());`, and it compiles only while `$crate::concurrency::ConcurrentFixture` resolves. `require_read_through!` is not `#[macro_export]`ed, so its half is latent rather than live; it earns its place here because it is the same rule broken in the opposite direction in the same crate, and because the module doc at `projection.rs:110-121` already argues that these three helper macros get copied per family — the copy is where a bare `ProjectionProbe` becomes `error[E0405]` inside a macro the author did not write.

**Semver.** The fix is additive — widen `__private`, remove nothing. The **defect** is a dated trap: after `0.2.0`, moving `ConcurrentFixture` to the crate root, demoting `pub mod concurrency` to private with selective re-exports, or relocating `BenchmarkParams` is a major break of `happenstance-testkit`, and it breaks one-line callers. `0.2.0` is therefore the last cheap moment for any of those relocations, and widening `__private` first is what makes them cheap later.

**Remediation.** Make `__private` the complete inventory rather than two thirds of one, and route every non-emitter `$crate::` item path in every exported macro through it — today six escape (`concurrency::ConcurrentFixture`, `concurrency::rules`, `bench::BenchmarkParams`, `bench::scenarios`, `model::rules`, `projection::rules`), plus `block_on`, plus the bare `ProjectionProbe`. Whether the emitter paths themselves should follow is the same open decision as C2-03's and is not settled by this.

**Routing.** The property is mechanically checkable — an `include_str!` scan over each macro-carrying file asserting that every `$crate::` type path in a `#[macro_export]`ed body is `$crate::__private::`-rooted, in the shape `xtask/src/lints.rs` already uses to read a file in a test. The scan is a backlog item; the question of whether these paths are *promises* is §6.6's, alongside C2-03's, and belongs to the runbook's specification pass. **HS-S0091** `registry-surface-diff` is where the gap becomes visible at release grain, because `cargo-semver-checks` reads item paths and not macro bodies.

---

### C2-01 · C2-05 — The onboarding page's account of the adapter's feature graph is wrong outward and silent inward

**Clause:** **PS-3** — *until PS-2's bar is met the port SHOULD ship behind an off-by-default `unstable-projection` feature, **with a documented exemption from semver*** (`spec/SPECIFICATION.md:4880-4890`). **`[PROVISIONAL]`**. The exemption is what the false sentence conceals. No clause governs a prose statement about feature implication, which is why nothing caught it.

**Examined.** `crates/happenstance-testkit/src/lib.rs:198-212` (step 1 of *Writing a projection adapter from outside this workspace*); `crates/happenstance-core/Cargo.toml:97` and `:80-96`; `crates/happenstance-core/src/lib.rs:101-106`, `:128-130`, `:171-173`; `crates/happenstance-testkit/Cargo.toml:30-55`; `Cargo.toml:2`; `crates/happenstance-testkit/README.md:166-175`.

**Found.** Outward, `crates/happenstance-testkit/src/lib.rs:209-212`:

```text
//! `conformance` is one flag on a dependency your adapter already has. It pulls
//! in no crate and implies no other feature — not `std`, not `memory` — so your
//! *normal* dependency graph does not grow at all. This crate is a
//! dev-dependency and stays one.
```

`crates/happenstance-core/Cargo.toml:97` is `conformance = ["unstable-projection"]`, and the manifest documents the implication as a deliberate choice over sixteen lines above it (`:80-96`). The contract crate's own page is already correct — `crates/happenstance-core/src/lib.rs:103-106`: *"implies exactly one thing — `unstable-projection`, because the probe is defined inside the module that feature gates. Not `std`, not `memory`."* Two rendered pages, one right, and the wrong one is the page an outside author lands on.

Inward, `crates/happenstance-testkit/Cargo.toml:49-55`:

```toml
[dependencies]
happenstance-core = { workspace = true, features = [
  "std",
  "memory",
  "conformance",
  "unstable-projection",
] }
```

with `Cargo.toml:2` `resolver = "3"`, and no testkit feature that declines any of the four. The crate's documentation names the dependency and not the features: `README.md:172-173`, *"this crate's stay `happenstance-core` and `futures-core`"*.

**Why a defect.** Both halves have a named victim and a delayed discovery, and they are different victims. Outward: an author follows step 1, writes `features = ["conformance"]` in their **normal** `[dependencies]`, and on the crate's written word believes they have taken on nothing. They have taken on `unstable-projection`, which PS-3 `[PROVISIONAL]` and ADR-0036 exempt from semver — and Cargo feature unification is global and additive, so every application in their graph compiles that surface too. Inward: an author writes `happenstance-core = { version = "0.2", default-features = false }` — the `no_std` or wasm population CF-20 exists for — and names a `memory`- or `std`-gated item in a helper in `src/`. Under resolver v3 the dev-dependency's four features unify into the same `happenstance-core` when and only when dev-dependencies are in the graph, so `cargo test` is green and `cargo build` is green for them; their consumer's build is `error[E0432]: unresolved import happenstance_core::MemoryEventStore`, because `crates/happenstance-core/src/lib.rs:171-173` gates that re-export on `memory`. The adapter's `src/` is compiled against a strictly larger contract crate under `cargo test` than under `cargo build`, and no page says so.

The manifest itself is not at fault and reads well: `crates/happenstance-testkit/Cargo.toml:30-48` explains each of the four features and why `unstable-projection` is restated rather than inherited. What it never states is the consequence for somebody else's build.

This corroborates a recorded, accepted open question. `.kb/open-questions/projection-store-in-adapter-default-features.md` names exactly this failure — *"`cargo add happenstance-sqlite` turned the unfrozen port on without the consumer naming it"* — resolved for `happenstance-sqlite` on 2026-09-02 and left open for `happenstance-neon` and `happenstance-postgres`. That atom is about manifests. This finding is the same defect one level out, in the sentence that tells a stranger what their manifest costs, and in a crate that is on the registry now.

**Semver.** None, in both directions: prose plus, if wanted, a gate step. Nothing here is dated by `0.2.0` in the semver sense. What is dated is who reads it — step 1 is the first screen of the extension surface, and `0.2.0` is when strangers start following it.

**Remediation.** The outward half is a four-line replacement against a sentence the contract crate has already written correctly. The inward half is not a feature split — `crates/happenstance-testkit/src/lib.rs:334` declares `pub mod fixtures;` unconditionally and `fixtures.rs:11-17` imports `MemoryEventStore`, `MemoryProjectionStore` and `ProjectionProbe` with no `cfg`, so all four features are genuinely required to compile the crate; what is missing is the sentence, and a check that an adapter's library still builds without its dev-dependencies.

**Routing.** **HS-S0094** `guarantees-and-docs-rs-presentation` (`…/publication-and-positioning/guarantees-and-docs-rs-presentation/spec.md`, stage `plan`) — *"what depending costs, stated where a consumer reads it"*, whose C5 already reads *"nothing gated may be invisible, and the manifest is what decides it"*. The inward half's instrument belongs to **HS-S0097** `stranger-install-smoke` (DoD 9): a scratch project outside the workspace is the only place where `cargo build` and `cargo test` disagree in the way this predicts.

---

### C2-06 — The promise a new rule arrives in a minor release is checked by nothing, and CF-29 says so about itself

**Clause:** **CF-29** — *a conformance rule MAY be added in a minor release, and MUST land in the same release as its mutant and its changelog entry* (`spec/SPECIFICATION.md:8625-8653`). **`[FROZEN]`**. **CF-32** — *`happenstance-testkit` MUST carry its own `version` key* (`:8684-8700`). **`[FROZEN]`**.

**Examined.** `crates/happenstance-testkit/Cargo.toml:21`; `xtask/src/lints.rs:312-368` (CF-32's whole check) and `:527-616` (`changelog_names_every_rule`, CF-29's); `crates/happenstance-testkit/README.md:207-211`; `crates/happenstance-testkit/src/lib.rs:205`.

**Found.** CF-32's check, in full, is a hand parse for one key and one substring — `xtask/src/lints.rs:344-366`:

```rust
    match declared {
        None => bail!( … declares no `version` key … ),
        Some(value) if value.contains("workspace") => bail!( … forbids that … ),
        Some(value) => {
            println!("CF-32: {TESTKIT_MANIFEST} carries its own `version{value}`");
            Ok(())
        }
    }
```

CF-29's other half, `changelog_names_every_rule` (`:548`), asserts that every rule name parsed from the suite appears in a `CHANGELOG.md` entry carrying 120 characters of prose, and reads no version key at all. Neither compares the rule set to the number. CF-29's own text concedes the shape (`spec/SPECIFICATION.md:8629-8631`): *"it is a `cargo xtask ci` lint step… and **it is the weakest check in the gate** — which is said here because the alternative is a clause that reads as though the obligation is mechanised."* Its `Rejects:` line (`:8649-8653`) names the exact failure: *"a rule added quietly in a patch release. An adapter author who takes a patch bump and finds their build red has no way to distinguish 'my adapter has a defect' from 'the suite changed', and the second answer erodes the first's authority permanently."*

**Why a defect.** A 90th rule can land with `Cargo.toml:21` untouched, or bumped to `0.2.1`, with a changelog entry under the existing heading — and `cargo xtask ci` is green. The victim is named by the crate's own onboarding: `crates/happenstance-testkit/src/lib.rs:205` tells an adapter author to write `happenstance-testkit = "0.2"`, a caret that takes the patch automatically, while `README.md:207-211` tells them to pin exactly — and that second sentence never reaches docs.rs (`src/lib.rs:7`). They find out from a red CI run with no way to tell whether they regressed or the bar moved. This is not a newly discovered gap; it is a gap the specification discloses about itself and nobody owns. That is why it is filed: `0.2.0` is the release that makes the caret matter, because it is the first stable number a caret can actually resolve past.

**Semver.** None. A gate step and a baseline file, neither in a published crate's compiled surface. Available at the same price before and after `0.2.0`.

**Remediation.** The shape that would work is a committed baseline — the rule set the last published version shipped, keyed by that version — and a gate step that fails when the current set differs and the version key still resolves inside the last published caret requirement. It would want a clause of its own; the corpus runs to CF-40. **This document does not write it.** ADR and clause authorship belong to the runbook's own pass, not to a review side effect.

**Routing.** A §6.6 clause plus its step, owned by the runbook's specification pass. The nearest existing instrument is **HS-S0091** `registry-surface-diff`, whose report header already carries *baseline version · feature set · verdict · findings · release-version* per crate — but it diffs the public **type** surface, which is precisely the surface a new rule does not change. Saying so in that story's *"what this did not check"* section (its B12) is the cheapest way to stop this gap reading as covered.

---

### V-3 — The falsifier does not write the manifest the recipe it exists to falsify specifies

**Clause:** **No clause governs the falsifier's manifest.** The recipe it departs from is rustdoc — `crates/happenstance-core/src/projection.rs:542-556` — and the feature it turns on unconditionally is the one PS-3 `[PROVISIONAL]` (`spec/SPECIFICATION.md:4880-4890`) exempts from semver.

**Examined.** `examples/outside-projection-adapter/Cargo.toml:20-29` and `src/lib.rs:1-10`, `:337`, `:481`; `crates/happenstance-core/src/projection.rs:535-565`; `crates/happenstance-sqlite/Cargo.toml:98`; the story's own record at `.bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/spec.md` (EC-006, NF-001) and `_extension-surface-gaps.md:148-163`.

**Found.** The recipe, published on the contract crate's page (`crates/happenstance-core/src/projection.rs:546-556`):

```toml
/// [dependencies]
/// happenstance-core = "…"
///
/// [features]
/// # Forwards to the contract crate. The `impl ProjectionProbe` lives in `src/`
/// # under `#[cfg(feature = "conformance")]`, because a crate cannot `cfg` on a
/// # dependency's feature — which is also why a `[dev-dependencies]` entry
/// # would not work: it does not exist for the lib build the impl compiles in.
/// conformance = ["happenstance-core/conformance"]
```

The real adapter obeys it: `crates/happenstance-sqlite/Cargo.toml:98`, `conformance = ["happenstance-core/conformance"]`. The falsifier does not (`examples/outside-projection-adapter/Cargo.toml:27-29`):

```toml
happenstance-core = { version = "0.2.0-alpha.1", path = "../../crates/happenstance-core", features = [
  "conformance",
] }
```

There is no `[features]` table in that manifest, and neither `impl ProjectionProbe` — `src/lib.rs:337` and `:481` — carries a `#[cfg]`. The crate's own page states the claim this falsifies (`src/lib.rs:3-6`): *"a projection adapter written **from the published documentation**, in a crate positioned so that the orphan rule, **the feature flags** and the dependency graph all behave the way they would for a stranger."*

**Why a defect.** The departure is deliberate and reasoned — the story's NF-001 requires the member to cost the feature powerset one combination rather than a multiplier, and EC-006 records that the crate *"declares **no features of its own**"* on exactly that ground. That makes it a trade, not an oversight, and it is still a defect, because the trade is not recorded where it is read. The gap record measured the feature cost and cleared it (`_extension-surface-gaps.md:163`, *"One feature asked for beyond the default: `conformance`. **The claim holds as stated.**"*) — over a `cargo tree -e features --depth 1` transcript at `:152-159` that prints `happenstance-core feature "conformance"` and `happenstance-core feature "default"` and no line for the feature `conformance` implies. So the one artefact that exists to prove the documented surface is sufficient did not exercise the one manifest shape the surface prescribes, and says nothing about not having. A third-party author now has two reference manifests in one repository, contradicting each other, with no note saying which is normative — and the falsifier is the one the docs.rs page points them at (`crates/happenstance-testkit/src/lib.rs:255-257`). Copying it publishes an adapter whose `[dependencies]` unconditionally enables `unstable-projection` for every application in its graph, which is the failure `.kb/open-questions/projection-store-in-adapter-default-features.md` records as accepted and open for two other crates in this workspace.

**Semver.** None. `examples/outside-projection-adapter` is `publish = false` and never becomes publishable (that story's NF-004). Cost is one additional feature-powerset combination, which is what NF-001 was protecting.

**Remediation.** Either the falsifier writes the manifest the recipe specifies and pays the combination, or its page states in one sentence which shape it did not test and why. The second is cheaper and it is a real answer; the first is what the crate's own opening paragraph currently claims. Note for whoever takes it: `unstable-projection` is not in `happenstance-core`'s default set (`crates/happenstance-core/Cargo.toml:35`), and the six port types the falsifier imports at `src/lib.rs:39-42` are gated on it, so the naive "move `conformance` into a `[features]` table" edit does not compile.

**Routing.** **HS-S0088** `falsifier-ledger-repair` (`…/publication-and-positioning/falsifier-ledger-repair/spec.md`, stage `plan`, DoD 12) — *"the falsifier ledger is repaired before anything reads it"* is this finding's home, and the repair is a recorded limitation rather than a code change. The choice between the two arms is the story owner's.

---

### M-2 — The concurrency page tells a runtime-free adapter it must bring `tokio`; two emitters ship and the specification says two

**Clause:** **CF-22** — *the rule set MUST be enumerated in exactly one place, as a macro taking a callback, and every harness MUST be built by invoking it* (`spec/SPECIFICATION.md:8218-8220`). **`[FROZEN]`**. Its own discussion states the emitter count at `:8348-8353`, and on the precedence ladder a clause outranks a doc comment.

**Examined.** `crates/happenstance-testkit/src/concurrency.rs:138`, `:146-155`, `:1114-1125`; `spec/SPECIFICATION.md:8348-8353`; `crates/happenstance-testkit/src/lib.rs:161-168`.

**Found.** The module page (`concurrency.rs:138`, then `:146-151`):

```text
//! # This family carries its own enumeration and its own emitter
…
//! **One emitter ships rather than three, and that is not a weakening of
//! CF-23.** The emitter is still a *parameter*; what has changed is which
//! wrappers make sense. `#[wasm_bindgen_test]` is unreachable — the module does
//! not exist on `wasm32`, which has no threads to race on — and a
//! `block_on`-only harness would be identical to the tokio one, because the
//! parallelism is in [`std::thread::scope`] rather than in the runtime.
```

The second emitter is 972 lines below it and gives the opposite reason for existing (`concurrency.rs:1114-1121`):

```rust
/// Emits one plain `#[test]` per concurrency rule, driven by
/// [`block_on`](crate::block_on).
///
/// Shipped because CF-23's content is that the wrapper is a *parameter*, and a
/// family with exactly one emitter reads as a family that forgot. It is also the
/// honest default for an adapter with no runtime at all …
```

and the specification agrees with the code, not the page (`spec/SPECIFICATION.md:8348-8351`): *"**One shipped emitter would have been enough and two are shipped anyway.**… the blocking emitter races exactly as hard as the tokio one."*

**Why a defect.** It is not a stale count in a comment: the paragraph is a cost statement, and it is the cost statement on the page an adapter author reads before deciding whether to invoke the family. Told the only wrapper is `#[tokio::test(flavor = "multi_thread")]`, a runtime-free adapter concludes the concurrency family costs it a `tokio` dev-dependency with `rt-multi-thread`. It does not — `__emit_concurrency_blocking` needs no runtime, and the crate says so about itself in the same file. The population that pays is the one the whole two-flavour design exists for. They find out by reading a file they were never expected to open.

Worth saying where the crate is right, because it makes the slip legible rather than careless: `crates/happenstance-testkit/src/lib.rs:161-168` deliberately refuses to spell any of the model, concurrency or benchmark module names as intra-doc links, because each module is absent under some configuration the crate is documented under and rustdoc treats an unresolved link as a hard error. That care is real. The count in `concurrency.rs:146` is the same class of fact with none of it applied — and `:152-153` then writes `[__emit_concurrency_tokio](crate::__emit_concurrency_tokio)` as a link to a `#[doc(hidden)]` item, while `lib.rs:65-67` writes the same class of name as plain code font. One crate, two spellings of one decision, which is C2-03's gap showing through.

**Semver.** None. A prose correction, non-breaking, available at the same price forever.

**Remediation.** The paragraph wants the count dropped as a *claim* — a two-row table in `lib.rs:63-67`'s existing shape counts itself and leaves no number for a third emitter to falsify — while keeping the two substantive sentences it earns (why a wasm emitter is unreachable, why the tokio one is `multi_thread`). The count is also mechanically assertable: `for_each_concurrency_rule!` already forces every emitter to consume one list.

**Routing.** **HS-S0095** `rendered-page-preflight` (`…/publication-and-positioning/rendered-page-preflight/spec.md`, stage `plan`) — *"the rendered pages are read before the irreversible act, and the read is dated"*. This is precisely a page-read finding, and `concurrency`'s module page is one an evaluator reaches from the rule table.


---

## Eleven rendered surfaces still describe an unpublished workspace, and the one prose-content check in the gate asks about rule counts

The prose that ships — crate-root rustdoc, the three crates.io front pages, `CHANGELOG.md`, `docs/` — was written from a vantage point that ended on 2026-08-16, when `448e1ac` published `0.2.0-alpha.1`. `happenstance-testkit`'s own docs.rs page tells its reader that nothing in this workspace is published yet; `happenstance-sqlite`'s front page says the crate carries `publish = false`, which its manifest, its README and `xtask/src/package.rs` all contradict at the same commit; the only copy-pasteable manifest fragment the testkit publishes specifies a requirement that cannot resolve against the registry today. The same failure class holds for decisions rather than dates: `SendEventStore`'s rendered page instructs every native adapter author to implement some *other* trait, because `trait_variant` copies the base trait's doc attributes verbatim and the workspace's own test relies on that copying; `store.rs` still states the read-laziness promise ADR-0011 replaced; `docs/append-conditions.md` cites as authority the one clause that refuses its claim. **None of this gets more expensive after `0.2.0` — the asymmetry here is one-shot and reputational rather than semver.** What changes is who reads it: these are the first screens of the two docs.rs pages a `0.2.0` evaluator opens, the manifest they paste, and the changelog they check. The repository has already answered this class of defect once, correctly, and the answer is why the entries below are routable rather than merely true: `xtask/src/main.rs:622-628` records that a README told crates.io the projection suite was "two rules of seventeen" through the fifteen commits that made it seventeen, that *"three audits raised it as a finding"*, and that *"a finding raised three times is a missing check, so this is the check."* That check exists, runs in the gate, and asks one question — how many rules — over prose it already knows how to extract from `.rs`, `.toml` and Markdown alike (`xtask/src/lints.rs:886-909`).

### P-4 — `happenstance-sqlite`'s docs.rs front page says the crate is not publishable, and the test written to catch exactly this misses it on a synonym

- **Clause:** No specification clause governs a crate-root doc block's statement about its own publish status. CF-32 `[FROZEN]` and its gate step (`xtask/src/main.rs:648`) govern the *manifest* half; `xtask/src/package.rs`'s `reconcile` (`:194-212`) governs the manifest-to-`PUBLISHABLE` pairing. Both are manifest checks by construction and neither reads a `//!` block.
- **Examined:** `crates/happenstance-sqlite/src/lib.rs:16-19`; `crates/happenstance-sqlite/Cargo.toml:17-27`; `crates/happenstance-sqlite/README.md:14-18`; `xtask/src/package.rs:101`; `crates/happenstance-sqlite/tests/front_page.rs:146-174`.
- **Found:** `crates/happenstance-sqlite/src/lib.rs:16-19`:

  ```
  //! Whether the crate is *published* is a different question with a different
  //! owner: it still carries `publish = false`, and lifting that is the
  //! publication pass's decision rather than this crate's. Having passed the suite
  //! and being on a registry are two claims, and only the first is made here.
  ```

  Its own manifest, six lines of comment written to be load-bearing, says the opposite — `Cargo.toml:23-24`: *"There is deliberately no `publish = false` here now, and its absence is half of an atomic pair"*. Its own README says the opposite — `README.md:14-16`: *"The `publish = false` this callout used to name is gone: the crate-set re-plan put it in the release beside `happenstance`, `happenstance-core` and `happenstance-testkit`."* And `xtask/src/package.rs:101` carries `"happenstance-sqlite"` in `PUBLISHABLE`.
- **Why a defect:** Not a stale sentence in a comment nobody renders — the crate-root `//!` block is the docs.rs front page, and it is paragraph two. `crates/happenstance/README.md:10-11` routes an application author here explicitly (*"Writing an application? This one — and a store to keep the events in: `happenstance-sqlite`"*), so the reader arrives to check the adapter is real and is told by the adapter that it is not on a registry. The discriminator between this and a preference is that four artefacts at one commit make two contradictory claims, three of them agree, and the one that disagrees is the only one a consumer sees. The instrument that should have caught it exists and runs: `tests/front_page.rs:151-174`, `fn the_front_page_no_longer_describes_a_skeleton`, written because *"Four statements shipped on docs.rs while every one of them was false"*, asserts the absence of six recognisable stale phrases including ``"is `publish = false` until it passes"``. It is green at `56ef6c5` because the surviving sentence spells the same claim differently — ``"it still carries `publish = false`"``. A phrase list catches the sentence it was written against and nothing else. `happenstance-cloudflare` got the same passage right (`crates/happenstance-cloudflare/src/lib.rs:66-73`), which is what makes this a slip rather than an unsettled question.
- **Semver:** None — no API moves. But `0.2.0` is the publication event this sentence is wrong about, and a docs.rs page is immutable per version: the correction cannot be applied to `0.2.0` after `0.2.0` is cut.
- **Remediation:** The one-line prose correction is obvious and is not the interesting half. The interesting half is whether `reconcile`'s existing both-directions pairing grows a third member — a grep of each `PUBLISHABLE` crate's crate-root `//!` block for the literal `publish = false` — so that the manifest flag, the constant and the front page cannot drift apart in any of the six directions rather than only the two they currently cannot.
- **Routing:** `RUNBOOK.md:4699-4712`, phase 12's pre-publish checklist, as a blocking item beside `cargo publish --dry-run`. The question of whether `reconcile` grows a prose half belongs to whoever owns `xtask/src/package.rs`; this document does not take it.

### C2-02 — The only copy-pasteable dependency block the testkit publishes cannot resolve against the registry, and the pinning advice that would fix it is excluded from the rendered page

- **Clause:** CF-30 `[NON-NORMATIVE]` (`spec/SPECIFICATION.md:8654-8668`), which is on point in both halves and already concedes the gap: *"The testkit's documentation should say so; nothing checks that it does."*
- **Examined:** `crates/happenstance-testkit/src/lib.rs:198-207` and `:1-7`; `crates/happenstance-testkit/README.md:207-211`; `Cargo.toml:33-41`; `examples/outside-projection-adapter/Cargo.toml:21-29`.
- **Found:** `crates/happenstance-testkit/src/lib.rs:200-207`:

  ```toml
  [dependencies]
  happenstance-core = { version = "0.2", features = ["conformance"] }

  [dev-dependencies]
  happenstance-testkit = "0.2"
  ```

  The workspace has written the warning against that first line twice already. `Cargo.toml:34-36`: *"this is the highest-cost line in the file to get wrong: `version = "0.2.0"` never matches a `0.2.0-alpha.1` release"*. `examples/outside-projection-adapter/Cargo.toml:21-26`: *"a `"0.2.0"` requirement does not match a `0.2.0-alpha.1` candidate, so the whole workspace stops resolving… An outsider adding a pre-release writes the pre-release."* The second line is a caret where `README.md:209-211` says the opposite — *"Treat that as a breaking change in practice and pin this crate exactly"* — and that advice never renders, because `lib.rs:7` includes the README only under `#![cfg_attr(doctest, …)]`.
- **Why a defect:** The failure is live, not hypothetical, and it is first contact. An adapter author on docs.rs for `happenstance-testkit 0.2.0-alpha.1` copies step 1 and `cargo` answers that no candidate matches `^0.2`, because `0.2.0-alpha.1` is the only version on the registry. That half self-heals when `0.2.0` stable ships and the other half does not: the caret on the testkit survives, and CF-30's whole argument — that a dev-dependency does not propagate, so an exact pin here costs nobody anything and buys the adapter author control of when they take a new bar — reaches no reader of the rendered page. This corroborates CF-30 exactly. The clause was demoted to `[NON-NORMATIVE]` on the ground that no adapter *behaviour* violates it; what the demotion left unowned is whether the recommendation reaches anyone, and the answer at `56ef6c5` is that it does not.
- **Semver:** None. `0.2.0` is when the first half stops being wrong by accident rather than by correction, which is the worse outcome — it removes the evidence without removing the caret.
- **Remediation:** The version strings are three characters each. The structural half is where the exact-pin paragraph lives: `README.md` is `cfg(doctest)`-only by a decision recorded at `lib.rs:1-6` (a duplicated page is worse than one page), so the advice has to be authored into the module docs rather than moved.
- **Routing:** The manifest fragment goes on phase 12's checklist with P-4. Whether CF-30 stops being prose and becomes a clause with a check — the paragraph's own *"nothing checks that it does"* is an invitation — is the specification pass's decision and its owner is the specification, not this document.

### C2-07 — A published crate's rustdoc tells its reader that nothing in this workspace is published yet

- **Clause:** None governs. CF-29 `[FROZEN]` and CF-31 `[FROZEN]` govern what a testkit release *means*; no clause governs whether the crate describes its own publication state correctly.
- **Examined:** `crates/happenstance-testkit/src/lib.rs:484-489`; `CHANGELOG.md:306`; `git log -S"factory =" -- crates/happenstance-testkit/src/lib.rs`.
- **Found:** `crates/happenstance-testkit/src/lib.rs:486-489`:

  ```
  /// The keyword was `factory =` and took a store expression. There is no
  /// deprecated arm, because nothing in this workspace is published yet and this
  /// is the last release in which that is true. Change the keyword and hand it a
  /// [`Fixture`] instead of a store.
  ```

  `happenstance-testkit` has been on crates.io at `0.2.0-alpha.1` since `448e1ac` (2026-08-16), which `CHANGELOG.md:306` records as `## [0.2.0-alpha.1] — 2026-08-16`.
- **Why a defect:** Two costs, and the second is the one that matters. The first is that the sentence is self-evidently false on the page it is rendered into, which spends the credibility of the compatibility prose around it. The second is that this sentence is the *standing written justification* for renaming a macro keyword with no deprecated arm, and it will read as licence the next time somebody wants to do that. The justification is not merely stale, it is unnecessary: `factory =` was introduced at `23fd446` (2026-08-06) and removed at `1c1a6b7` (2026-08-08), eight days before the first publish, so no published version of `happenstance-testkit` ever accepted it. The true reason is stronger than the false one.
- **Semver:** None. `0.2.0` is not the last cheap moment for this; every moment is equally cheap, and the sentence gets read by more people after each one.
- **Remediation:** Four lines for four lines. The replacement clause is a fact about two commits rather than a claim about the registry, so it cannot go stale in the direction this one did.
- **Routing:** Phase 12's checklist, with P-4 and C2-02 — three instances of one class, cheapest to walk together.

### U-3 — `CHANGELOG.md` scopes itself to three crates, and `happenstance-sqlite` is the fourth in this release

- **Clause:** CF-29 `[FROZEN]` (`spec/SPECIFICATION.md:8625`) requires a changelog entry per conformance rule and has a gate step that enforces it (`xtask/src/main.rs:607`, `lint-changelog`). No clause governs which crates the document covers, and the step reads the file only for rule names (`xtask/src/lints.rs:486-515`).
- **Examined:** `CHANGELOG.md:3` and `:25-305`; `.github/workflows/ci.yml:382-390`; `crates/happenstance-sqlite/Cargo.toml:17-27`.
- **Found:** `CHANGELOG.md:3`:

  ```
  Notable changes to `happenstance`, `happenstance-core` and `happenstance-testkit`.
  ```

  `.github/workflows/ci.yml:382-384` says the opposite at the same commit: *"Every crate the release publishes, plus the one that is publish-ready and waiting. `happenstance-sqlite` joined the release set at the crate-set re-plan"*, and `package:` at `:390` lists it. `happenstance-sqlite` appears twice in the whole of `CHANGELOG.md` (`:665`, `:1721`), both times inside historical entries about other crates; the `[Unreleased]` section carries no entry for it at all.
- **Why a defect:** `crates/happenstance/README.md` tells a consumer that *"What changed is in `CHANGELOG.md`, per release, in a caller's terms"*. A consumer who follows that and then adds `happenstance-sqlite` reads, in the document's first sentence, that it is not about the crate they just took a dependency on — and a real public surface (the two store roles behind features, the store-limit constants, the WAL and busy-timeout behaviour) has no release note anywhere. This exact drift has already happened once in this tree, in prose, and `CLAUDE.md` records it about itself: *"this sentence read 'three' through phase 9's promotion of `happenstance-cloudflare` and did not move, so a count on its own turned out to be a claim nobody re-reads."* Here the members are spelled out and it drifted anyway, which narrows the diagnosis: spelling the members is not the fix, having something read the sentence is.
- **Semver:** None. The asymmetry is one-shot: `0.2.0` is the release the entries would describe, and a changelog written after the fact is reconstructed rather than recorded — which `CHANGELOG.md:9-11` is itself a paragraph about.
- **Remediation:** A scope line and a set of `[Unreleased]` entries. The durable half is that `lint-changelog` already parses this file and already knows how to extract prose; extending it to compare the scope sentence against `PUBLISHABLE` is a comparison between two lists that both already exist in the gate's memory.
- **Routing:** Phase 12's first checklist item, `RUNBOOK.md:4699` (*"`CHANGELOG.md` finalised for 0.2.0"*), which is the correct owner and currently unchecked.

### F1-01 — `SendEventStore`'s and `SendProjectionStore`'s rendered pages instruct the reader to implement a different trait, because the derivation copies the base trait's doc attributes and the workspace relies on it doing so

- **Clause:** ES-1 `[FROZEN]` (`spec/SPECIFICATION.md:2508-2516`), which mandates both halves the pages invert: *"The `Send` flavour MUST be derived by `#[trait_variant::make(SendEventStore: Send)]` rather than hand-written… Generic code in this workspace and in adapter crates MUST bind `EventStore`, not `SendEventStore`."*
- **Examined:** `crates/happenstance-core/src/store.rs:86-141` and `:1028-1075`; `crates/happenstance-core/src/projection.rs:294-302`, `:417`; `trait-variant-0.1.3/src/variant.rs`'s `mk_variant`.
- **Found:** `crates/happenstance-core/src/store.rs:88-90`, which the derivation at `:141` copies verbatim onto `SendEventStore`:

  ```
  /// This is the `!Send` flavour and the one to use in generic bounds; see the
  /// [module documentation](self) for why. Adapters that can be `Send` should
  /// implement [`SendEventStore`] instead and get this for free.
  ```

  and `:104-105`:

  ```
  /// Bound on this trait, not [`SendEventStore`], unless you need to cross a
  /// thread boundary:
  ```

  Same shape at `projection.rs:300-302` for `SendProjectionStore`. The workspace states the copying mechanism itself at `store.rs:128-131` — *"the `trait_variant` expansion builds it with `..tr.clone()`… which copies the trait's attributes verbatim"* — and **depends** on it for documentation: `store.rs:1050-1052` reasons that *"`SendEventStore` has no doc comment of its own, so if attributes stopped being copied, `missing_docs`… would fail the build."*
- **Why a defect:** The audience is every native adapter author, which is every adapter author but the wasm one, and the module routes them onto this page on purpose (`store.rs:23-25`: *"Implement `SendEventStore` if your store can be shared across threads — which is every native adapter"*). They land on `SendEventStore`'s page and read that it is the `!Send` flavour, that they should implement `SendEventStore` "instead", and to bind "this trait, not `SendEventStore`" — three sentences, each false about the page carrying it, one of them a circular instruction. This is not a preference about doc phrasing: ES-1's binding rule is `[FROZEN]`, and the surface that teaches it teaches its inverse. The two published sites are `SendEventStore` and `SendProjectionStore`; the same shape exists at `happenstance-sync/src/ingest.rs:119` and `peer.rs:81`, which are `publish = false` and therefore not yet anyone's problem.
- **Semver:** None — no signature moves; only doc attributes. This is not cheaper before `0.2.0` than after, but it is read by more adapter authors after.
- **Remediation:** The constraint is that a doc line usable on both flavours has to be true on both, since one attribute set serves two pages. `store.rs`'s own test module (`:1028-1075`) already reads the file as text and asserts on the lines around the derivation, so an assertion that the copied block names neither `SendEventStore` nor `!Send` sits in an existing harness rather than a new one — and the same module already pins the `trait-variant` version for exactly this reason.
- **Routing:** The port's owner, as a documentation change against ES-1's existing frozen text; the clause needs no amendment, because the clause is what the pages contradict.

### T1 — `store.rs` promises a laziness ADR-0011 replaced and ES-11 refuses, and the crate's own reference adapter does not have it

- **Clause:** ES-11 `[PROVISIONAL]` (`spec/SPECIFICATION.md:2976-2983`), whose own prose settles it: *"Laziness is therefore **permitted and never required**, and building a stream and never polling it is not guaranteed to be free."*
- **Examined:** `crates/happenstance-core/src/store.rs:151-160`; `crates/happenstance-core/src/memory.rs:296-336`; `.kb/decisions/0011-read-laziness-and-isolation.md:64-75` (status `accepted`); `spec/SPECIFICATION.md:2992-3012`.
- **Found:** `crates/happenstance-core/src/store.rs:152-154`:

  ```
  /// Reads the events matching `query`, in the order `options` asks for.
  ///
  /// The returned stream is **lazy**: nothing is executed until it is first
  /// polled, and failures surface as `Err` items rather than up front.
  ```

  ADR-0011, accepted, decided otherwise (`.kb/decisions/0011-read-laziness-and-isolation.md:66-69`): *"The port's promise changes from 'lazy' to 'evaluated against one state sampled no later than the first poll': laziness is permitted, never required… a caller must not depend on whether events appended between the call and the first poll appear."* A grep of `store.rs` for the decided vocabulary — `sampled`, `no later than` — returns nothing.
- **Why a defect:** The sentence is false about the adapter shipped in the same crate, 150 lines below it. `memory.rs:301-335` takes the read guard, filters, orders, truncates and collects into a `Vec` inside `fn read`, before any poll — the comment at `:303-304` says so: *"Filter, order and truncate under the lock, then release it."* So the port's own rustdoc makes a promise its reference implementation does not keep, and the reader who relies on it is the third-party adapter author, because `store.rs` is the document they implement against. Two things follow. They may over-promise laziness in their own adapter on the strength of the port's wording; and, worse, they never meet ADR-0011's real obligation, which is absent from `store.rs` entirely — *"an adapter issuing more than one statement per `read` must capture a position ceiling no later than the first poll and bound every later statement by it"* (`:70-73`). The testkit does carry `read_result_is_stable_under_concurrent_append`, so the obligation is *checked*; what is missing is that it is nowhere *stated* on the surface the implementer reads first, which is the difference between finding out at design time and finding out when the suite goes red.
- **Semver:** None — doc only. Worth noting that ES-11's marker is `[PROVISIONAL]` on the transport axis, so this is a correction toward a clause that may still move, not a freeze.
- **Remediation:** Two sentences, not one: the laziness claim reduced to what ES-11 permits, and the ceiling obligation added, since it is the half an implementer cannot derive from the signature. The existing `RefetchingPagedStore` in the testkit's own `tests/` already fails the rule, so no new instrument is needed to demonstrate the shape the wording is about.
- **Routing:** The port's owner. ADR-0011 is accepted and immutable; this is unexecuted follow-through on an accepted decision, not a new decision, and it belongs beside F1-01 as one pass over `store.rs`'s rustdoc.

### O-1 + P-5 — `docs/append-conditions.md` cites as authority the one clause that refuses its claim, does not answer the question it declares, and both of its `file:line` citations are dead

- **Clause:** ES-40 `[PROVISIONAL]` as cited by the page; ES-25 `[FROZEN]` and ES-26 `[FROZEN]` are what the page's sentence is actually about. RP-30-2 and RP-30-3 (`standards/pages/30-citing-the-specification.md:41-92`) are the page rules it breaks.
- **Examined:** `docs/append-conditions.md:1-31` (the whole page); `spec/SPECIFICATION.md:4422-4432`; `crates/happenstance-core/src/lib.rs:103`, `:122`, `:134`, `:173`; `xtask/src/lint_pages.rs:7-11` and `:20-23`; `xtask/src/lint_narrative.rs:27-33`; `xtask/src/lint_constitution.rs:55`.
- **Found:** the page's entire answer, `docs/append-conditions.md:5-7`:

  ```
  An append condition is checked against the same boundary the query read, so a
  writer that saw a consistent view cannot be overtaken between reading and
  appending (ES-40).
  ```

  ES-40 (`spec/SPECIFICATION.md:4425-4429`, `[PROVISIONAL]`) says the reverse: *"A store that has had matching history removed MAY admit an append that would have been rejected, and the contract MUST NOT imply otherwise."* Its own commentary: *"the condition passes **vacuously**."* And `docs/append-conditions.md:19-20`:

  ```
  `memory` is a private module (`crates/happenstance-core/src/lib.rs:103`); the type is
  re-exported at `:122`, so `happenstance_core::MemoryEventStore` is the resolving path.
  ```

  `lib.rs:103` is `//!   authors running that suite against their own store; nothing in the runtime` — mid-paragraph of the `conformance` *feature* documentation. `lib.rs:122` is `mod query;` — a different private module's declaration. The real `mod memory;` is at `:134` and the real re-export at `:173`.
- **Why a defect:** The page declares `explanation` for *"Why does a write re-read what it decided on?"* (`:3`) — the central question in DCB and the one an application author arrives with. It answers it in one sentence, and the sentence's cited authority contradicts it. A reader following the page's own citation discipline lands on a clause about vacuous passes over pruned history and either loses trust in the page or, worse, walks away believing the completeness caveat does not exist. The two dead citations land the reader on unrelated lines with no way to tell — `RP-30-3`'s `Rejects` names precisely this failure one file over: *"A page citing `spec/SPECIFICATION.md:1462`. It resolves on the day it is written and points at an unrelated clause the next time the file grows."* This corroborates two enumerated blind spots the workspace has already written down, in the modules that walk this page on every gate run. `xtask/src/lint_pages.rs:7-8`: *"It checks that a need is declared, never that the page answers it."* `:20-23`: *"It does not resolve clause ids… A page can cite a real, resolving id and restate its content in the paragraph underneath, and nothing mechanical sees that either."* `xtask/src/lint_narrative.rs:29-31` says the same for fences: *"Whether its prose still describes what its fences do is a question nothing in this repository asks."* This page falls inside all three declines simultaneously. The anchored-citation machinery that would catch the dead `file:line` pair exists — `xtask/src/lint_constitution.rs` requires every citation as `path:line (anchor)` and verifies the anchor sits within a tolerance of the stated line — and is scoped to `standards/rust` (`:55`).
- **Semver:** None. `docs/` is not published to crates.io; it is the narrative tree the repository points readers at, and the page is the one named for the mechanism that makes DCB work.
- **Remediation:** Merged from two findings because they are one page and one cause. The clause correction and the two line numbers are edits; the shape question is whether the citation form the constitution already enforces (`path:line (anchor)`) is extended over `docs/`, which is a scope change to an existing checker rather than a new one, and whether a page citing a clause is checked against the clause's *content* rather than its existence — which `lint_pages.rs:20-23` currently declines by design and refers to `spec_trace`.
- **Routing:** The narrative tree's owner for the page; the two enumerated declines in `lint_pages.rs` and `lint_narrative.rs` are where a decision about widening the checkers belongs, and this document does not take it.

### O-4 — `happenstance`'s crates.io front page claims a feature parity three codec keys contradict, and the defaults it claims cannot disagree already do

- **Clause:** None governs a README's feature-parity claim. ADR-0003 governs where `serde` may live and is not violated here.
- **Examined:** `crates/happenstance/README.md:44-48`; `crates/happenstance/Cargo.toml:14-19` and `:72-112`; `crates/happenstance-core/Cargo.toml:34-97`.
- **Found:** `crates/happenstance/README.md:47-48`, under a heading called `## Guarantees`:

  ```
  - Every feature this crate has is forwarded from `happenstance-core`, so the two
    cannot disagree about what `default-features = false` means.
  ```

  The manifests at the same commit:

  | | `happenstance` | `happenstance-core` |
  | --- | --- | --- |
  | `default` | `std, memory, json` | `std, memory` |
  | forwarded | `std`, `serde`, `memory`, `unstable-projection` | the same four |
  | only here | **`json`, `postcard`, `cbor`** | — |
  | only there | — | **`conformance`** |

  `crates/happenstance/Cargo.toml:75-77` states the divergence itself: *"The three codecs. Each turns on exactly one optional dependency and one unit struct."* The same overstated claim is written a second time at `Cargo.toml:15-16`: *"every one is forwarded below, so that a consumer of the facade has exactly the switches a consumer of the contract crate has."*
- **Why a defect:** The sentence is false in exactly the way it declares impossible. `default-features = false` on `happenstance` drops `json` — a codec, a type and a dependency — and on `happenstance-core` drops nothing of the kind, so the two *do* disagree about what the flag means. The reader who is hurt is the one the sentence is written for: an integrator reasoning about a minimal or audited dependency graph, told there is nothing crate-specific to audit, who finds three optional third-party dependencies (`serde_json`, `postcard`, `ciborium`) at `cargo tree -e features`. The `ciborium` case is the sharp one, because `crates/happenstance/Cargo.toml:78-82` records that its licence subtree was read against the allowlist and could have refused — a decision that only exists because the feature is local to this crate.
- **Semver:** None to fix the sentence. The underlying divergence is correct and deliberate — ADR-0006 gave encoding to the typed layer, and codecs *belong* here. It is the claim that is wrong, not the manifest.
- **Remediation:** Two sentences to correct, not one: the README and the manifest comment state the same thing, and fixing one leaves the drift in place.
- **Routing:** The typed layer's owner, with C2-02 and U-3 as one pass over the published front pages before `0.2.0` is cut.

### N-1 — `happenstance-cloudflare`'s manifest declares `cargo deny check bans` red and escalates to an ADR about something else; `deny.toml` at the same commit already resolved it, under a different ADR

- **Clause:** None governs a manifest comment. ADR-0035 (`.kb/decisions/0035-async-trait-through-worker.md`, accepted) settles the question; ADR-0001 is the ban it amends from outside.
- **Examined:** `crates/happenstance-cloudflare/Cargo.toml:54-65`; `deny.toml:73-80`; `.kb/open-questions/deny-bans-red-on-the-worker-dependency.md:1-5` and `:118-153`; `.kb/decisions/` (directory listing).
- **Found:** `crates/happenstance-cloudflare/Cargo.toml:56-65`, present tense:

  ```
  # *And one price that is not paid yet, stated because a silent one is how a
  # guard dies.* `cargo deny check bans` is **red**: `deny.toml` bans
  # `async-trait` (ADR-0001 …) … and that decision is not this manifest's to
  # take … So the finding stands, recorded against AC-008 in
  # `worker-binding-layer/_ledger.md` and escalated to ADR-0023
  # (`adr-0023-and-atom-resolutions`), which is where the wrapper entries land if
  # they are ratified.
  ```

  `deny.toml:74-79` at the same commit already carries the ratified entry:

  ```toml
  { crate = "async-trait", wrappers = [
      "wasm-bindgen-test",
      "worker",
      "worker-macros",
  ], reason = "ADR-0001: injects `+ Send`, which forecloses wasm32; ADR-0035 exempts the `worker` subtree" },
  ```

  The open question that raised it is closed: `.kb/open-questions/deny-bans-red-on-the-worker-dependency.md:4` reads `status: superseded`, and `:128-130` reads *"**Sub-question 1 is answered `ratify`**"* and *"`cargo deny check bans` reports `bans ok`"*. ADR-0023 is `.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` — the SQLStorage mapping and the off-tokio harness, an unrelated decision.
- **Why a defect:** Three artefacts at one commit disagree, and the manifest is the only one that is wrong — on both facts. It reports a gate step as red that the brief's own baseline confirms ran and passed, and it routes the reader to an ADR number that settles something else. The person who pays is a contributor or PR reviewer touching this crate: they read a present-tense unratified finding, and either re-litigate a settled question or lose confidence in the green gate that the rest of this review's evidence rests on. They discover the contradiction only by independently opening `deny.toml` and the open-question atom. Note that the comment was *right* when written and is a good example of the house practice it names — *"a silent one is how a guard dies"*. What failed is that resolving the question did not close the record that raised it.
- **Semver:** None. A comment in a `publish = false`-free manifest that is not in the release set for `0.2.0` (`.github/workflows/ci.yml:384-385`: *"`happenstance-cloudflare` is publish-ready but deferred past `0.2.0`"*).
- **Remediation:** The comment becomes past tense and names ADR-0035. The generalisable half is that closing an open question did not sweep the sites that cite it; the atom's own `source_paths` and the `related` graph are where such a sweep would be driven from.
- **Routing:** The Cloudflare crate's owner, as follow-through on ADR-0035's execution. No decision is open here — that is the point of the entry.

### N-2 — ADR-0029 raised the MSRV on a "nothing is published" premise that no longer holds, and rejected the per-crate alternative on a `publish = false` that is also gone

- **Clause:** None governs the MSRV. ADR-0029 (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`, accepted) amends ADR-0004, whose `provisional` marker is scheduled to be lifted at phase 12.
- **Examined:** `Cargo.toml:17`; `references/adr/0029-msrv-raised-to-1-97-1.md:46-51`, `:80-84`, `:99-104`; `RUNBOOK.md:4708` and `:163`; `crates/happenstance-core/README.md:53-56`; `crates/happenstance/README.md:49-52`; `cargo metadata` over the pinned tree.
- **Found:** `references/adr/0029-msrv-raised-to-1-97-1.md:49-51`:

  > *"Nothing is published. No downstream consumer is pinned to anything. The floor costs nobody anything today, and phase 12 is where it turns into a promise."*

  and `:80-82`: *"1.97.1 is recent, and anyone on a distribution-packaged toolchain is excluded. This is the trade, and it is only defensible while nothing is published."* The rejected alternative was rejected on a fact that has since changed — `:99-101`: *"**Give `happenstance-sqlite` its own higher `rust-version`.** Honest per-package metadata: the crate is `publish = false`, so the three published crates could have kept 1.85 truthfully."* `RUNBOOK.md:4708` still carries the unchecked box *"ADR-0004 loses `provisional`; the MSRV becomes a promise."*, under a phase the table at `:163` marks `not started`.

  The floor is also stated as a `## Guarantees` bullet on two published front pages — `crates/happenstance-core/README.md:53` and `crates/happenstance/README.md:49`, both *"MSRV 1.97.1, checked in CI."* — while `cargo metadata` over the pinned tree says the maximum *declared* `rust-version` in `happenstance-core`'s non-dev closure is `trait-variant` at `1.75`, and `edition = "2024"` puts its real floor at `1.85`. Only `happenstance-sqlite`'s graph needs the higher number, and it needs it through a build script (`libsqlite3-sys`) that declares no `rust-version` at all.
- **Why a defect:** Two premises, both discharged by facts that arrived after the ADR. Three of the crates the floor applies to are on the registry; the fourth — the one whose dependency forced the floor from 1.85 to 1.97.1 — is what `0.2.0` adds, and its `publish = false` is gone. The named wrong outcome is the one ADR-0029 already reproduced once: a consumer of `happenstance-sqlite` building at the stated 1.97.1 hits the `cfg_select!` build-script break the moment their own lockfile resolves a newer `rusqlite`/`libsqlite3-sys`, because none of those crates declares a `rust-version` and therefore neither `cargo hack --rust-version` nor `resolver = "3"` can see it coming — which `CLAUDE.md` already says in as many words. They find out at `cargo build`, on a floor two published crates advertise as a guarantee and do not need. This is the *decision drifting*, not the code: the reasoning ADR-0029 wrote down is what has expired, and the ADR itself named phase 12 as the review point.
- **Semver:** Raising an MSRV after publish is breaking in practice, whatever the manifest key says; lowering one is additive. `0.2.0` is genuinely the cheap moment for this and the only one — the promise begins at the stable release, and `happenstance-sqlite`'s first published version is that release.
- **Remediation:** The decision has at least three shapes and this document takes none of them: hold 1.97.1 workspace-wide and lift ADR-0004's `provisional` as planned; state a per-package floor now that `happenstance-sqlite` is a separate published crate and the `publish = false` ground for rejecting that is gone; or lower the floor on the three crates whose graphs do not require it. The two README `## Guarantees` bullets are downstream of whichever is chosen.
- **Routing:** `RUNBOOK.md:4708`, phase 12, which already owns this and names it. The decision belongs to the ADR pass — ADR-0029 is accepted and immutable, so a change is a new atom amending ADR-0004 again, not an edit.

### U-2 — `happenstance-sqlite`'s dev-dependency ships a version requirement that reintroduces the publish-order coupling the same workspace removed, at length, one crate over

- **Clause:** CF-32 `[FROZEN]` (`spec/SPECIFICATION.md:8684-8694`) requires `happenstance-testkit` to carry its own `version` key so it can move independently, and has a gate step (`xtask/src/main.rs:648`). The clause is satisfied — `crates/happenstance-testkit/Cargo.toml:21` reads `version = "0.2.0-alpha.1"`. No clause governs how a *consumer* spells the dev-dependency, which is where the coupling comes back.
- **Examined:** `crates/happenstance-sqlite/Cargo.toml:51`; `Cargo.toml:41`; `crates/happenstance/Cargo.toml:60-70`; the generated manifests under `target/package/`.
- **Found:** `crates/happenstance-sqlite/Cargo.toml:51`:

  ```toml
  happenstance-testkit = { workspace = true, features = ["proptest"] }
  ```

  which inherits `Cargo.toml:41`'s `version = "0.2.0-alpha.1"`. `crates/happenstance/Cargo.toml:60-70` spells the identical dev-dependency deliberately differently, and explains why at length:

  ```
  # Deliberately **not** `{ workspace = true }`, and this is NF-006 rather than an
  # oversight. `[workspace.dependencies]` carries a version requirement on this
  # crate, and a dev-dependency carrying a version has to resolve from the
  # registry at publish time — which would make `happenstance-testkit` publish
  # *before* `happenstance`, a coupling CF-32's independent version number exists
  # to avoid. Cargo strips a versionless dev-dependency from the published
  # manifest entirely, so the path-only spelling is what keeps the two crates'
  # release order free.
  ```

  The generated artefacts confirm both halves: `target/package/happenstance-sqlite-0.2.0-alpha.1/Cargo.toml:118-120` carries `[dev-dependencies.happenstance-testkit] / version = "0.2.0-alpha.1"`, and `target/package/happenstance-0.2.0-alpha.1/Cargo.toml` carries no `happenstance-testkit` dev-dependency at all.
- **Why a defect:** The coupling is exactly the one the workspace diagnosed and removed, reintroduced by the crate that joined the release set after the diagnosis was written. `happenstance-testkit` versions independently by design — CF-29 `[FROZEN]` explicitly contemplates a rule landing in a minor bump, and `CHANGELOG.md`'s `[Unreleased]` section stages dozens — so the next testkit-only bump is a routine event. When it happens, whoever cuts the next `happenstance-sqlite` release finds that it cannot be published until the new testkit version is live on the registry, for a dependency that never reaches a consumer of the crate. They find out at `cargo publish`, and the explanation is already written eleven lines long in a sibling manifest, which is the tell that this is a slip rather than a different judgement.
- **Semver:** None to a consumer — a dev-dependency does not propagate. It is a release-order constraint on the maintainer, and `0.2.0` is when `happenstance-sqlite` first has a release order to constrain.
- **Remediation:** The finding sits on `crates/happenstance-sqlite/Cargo.toml:51`; the version requirement it inherits is at `Cargo.toml:41` and is load-bearing for nothing in the workspace, since every other reference to the testkit is a dev-dependency too. Which of the two lines moves is a real choice — the per-crate spelling matches the precedent that already exists and is documented, the root spelling fixes it once for every future adapter.
- **Routing:** Phase 12's publish-order item, `RUNBOOK.md:4702-4704`, which already fixes `core → testkit → happenstance → sqlite` and therefore masks this for `0.2.0` specifically. That masking is why it needs routing now rather than being discovered later.


---

## The two crates that have not shipped are assembled out of other people's types, and one of their errors can be forged

`happenstance-core` re-exports `bytes` and says why in the doc comment above it; `happenstance-postgres`, which carries `publish = false` and will never reach a consumer, re-exports `sqlx` and says why in the doc comment above it; and the workspace wrote the rule down as a constitution atom with a named diagnostic and a named rejected implementation. The two crates in `PUBLISHABLE` that are not yet on crates.io — `happenstance-sqlite`, which 0.2.0 adds, and `happenstance-cloudflare` — apply it in neither direction: between them they name `rusqlite`, `tokio` and a pre-1.0 `worker` across every construction path, every error payload and the whole projection write vocabulary, and re-export none of the three. That is not a style objection. `SqliteEventStore::new(rusqlite::Connection)` on the registry makes a `rusqlite` 0.41 bump a breaking release of `happenstance-sqlite` forever, and a `pub use rusqlite;` added afterwards does not undo it. Two smaller items in `happenstance-cloudflare` sit in the same window: `StringifiedThrow` is the one error type in the five publishable crates that any caller can mint or mutate, and it exposes a classifier that reads that mutable field; and `PartialBatch`, the variant a `[FROZEN]` clause's own violation is reported through, carries two typed causes and marks neither `#[source]`, so every `source()` walker in the ecosystem sees a leaf. The fourth item is the workspace's only SQL-text seam handed to a consumer, and it is undocumented as one.

---

### D-1 · D-4 — Both unpublished adapters name a foreign crate on their construction and error paths and re-export none of it; the two crates that get this right are the contract crate and a skeleton that will never publish

**Clause.** **No specification clause governs this**, and that is stated rather than omitted: `grep -n "re-export" spec/SPECIFICATION.md` returns four hits, none of them about a third-party dependency's visibility. The nearest clause is **ES-6 [FROZEN]** (`spec/SPECIFICATION.md:2677`, marker at `:2679`), and it points the other way — it *endorses* the wrapping, citing `SqliteEventStoreError` as "twelve real variants over `rusqlite::Error`, `JoinError`, `TryCurrentError` and the crate's own decode failures" (`spec/SPECIFICATION.md:2688-2690`) as the instrument that made ES-6 decidable at all. The rule that governs is a constitution atom: **RS-40-4**, *Name a signature's types through the defining crate's own re-export* (`standards/rust/40-public-surface-and-evolution.md:171-210`). Under the precedence ladder an atom binds below a clause and above `references/evaluation/*`; nothing above it contradicts it here.

**Examined.**
`crates/happenstance-sqlite/src/lib.rs:80-95` (the whole of the crate's export surface) · `crates/happenstance-sqlite/src/event_store.rs:128-131, 316, 857-882` · `crates/happenstance-sqlite/src/connection.rs:20, 78, 89, 186` · `crates/happenstance-sqlite/src/projection_store.rs:111, 216, 370-385, 423, 449-468` · `crates/happenstance-sqlite/README.md:33-36` · `crates/happenstance-cloudflare/src/lib.rs:454-458` · `crates/happenstance-cloudflare/src/sql_storage.rs:247, 262` · `crates/happenstance-cloudflare/src/js.rs:188, 197` · `crates/happenstance-cloudflare/README.md:44-45` · `crates/happenstance-cloudflare/Cargo.toml` (no `[features]` table) · against `crates/happenstance-core/src/lib.rs:184-186` and `crates/happenstance-postgres/src/lib.rs:76-78` · `Cargo.toml:72, 122, 129`.

**Found.** `happenstance-sqlite`'s entire export surface is three module declarations and zero `pub use`:

```rust
// crates/happenstance-sqlite/src/lib.rs:82-95
#[cfg(any(feature = "event-store", feature = "projection-store"))]
pub mod connection;

#[cfg(feature = "event-store")]
pub mod event_store;
...
#[cfg(feature = "projection-store")]
pub mod projection_store;
```

The foreign types are on the default feature set, in the error enum, in both constructors, in the configuration helpers and in the projection write vocabulary:

```rust
// crates/happenstance-sqlite/src/event_store.rs:860, 874, 882
    Sqlite(#[from] rusqlite::Error),
    Worker(#[from] JoinError),
    NoRuntime(#[from] TryCurrentError),
// (imports at :128-131 — `rusqlite::Connection`, `rusqlite::types::Value`,
//  `tokio::runtime::{Handle, TryCurrentError}`, `tokio::task::{JoinError, JoinHandle}`)

// crates/happenstance-sqlite/src/event_store.rs:316
    pub fn new(connection: Connection) -> Result<Self, SqliteEventStoreError> {

// crates/happenstance-sqlite/src/connection.rs:78, 89, 186
pub fn open_configured(path: impl AsRef<Path>) -> rusqlite::Result<Connection> {
pub fn configure(connection: &Connection) -> rusqlite::Result<()> {
    pub fn read_back(connection: &Connection) -> rusqlite::Result<Self> {

// crates/happenstance-sqlite/src/projection_store.rs:216, 423, 382
    pub fn new(connection: Connection) -> Self {
    pub fn push(&mut self, sql: impl Into<String>, params: impl IntoIterator<Item = Value>) {
    pub fn params(&self) -> &[Value] {
```

`SqliteProjectionStoreError` repeats the three `#[from]` payloads verbatim at `:452, :464, :468`. `happenstance-cloudflare` is the same shape against `worker` — every construction path into the crate names it, and so does the README's only example:

```rust
// crates/happenstance-cloudflare/src/sql_storage.rs:247, 262
    pub fn new(sql: worker::SqlStorage) -> Self {
    pub fn from_state(state: &worker::State) -> Self {
// crates/happenstance-cloudflare/src/js.rs:188, 197
    pub fn from_error(error: worker::Error) -> Self {
    pub fn error(&self) -> &worker::Error {
// crates/happenstance-cloudflare/README.md:44-45
async fn enrol(state: &worker::State) -> Result<(), Box<dyn core::error::Error>> {
    let store = CloudflareEventStore::new(SqlStorage::new(state.storage().sql()));
```

and its exports at `lib.rs:456-458` are three lines of the crate's own types, none of them `worker`. Its `Cargo.toml` declares no `[features]` table at all, so there is no build of this crate that does not link `worker` 0.8.5 (`Cargo.toml:122`).

The two crates that do it correctly say so in the doc comment:

```rust
// crates/happenstance-core/src/lib.rs:184-186
/// Re-exported so adapters and callers can name payload types without adding a
/// direct dependency on a specific `bytes` version.
pub use bytes;

// crates/happenstance-postgres/src/lib.rs:76-78
/// Re-exported so callers can build a pool without pinning their own `sqlx`
/// version against this crate's.
pub use sqlx;
```

`happenstance-postgres` carries `publish = false` (`crates/happenstance-postgres/Cargo.toml:12`).

**Why a defect.** The discriminator is RS-40-4's own *Rejects* clause, and it names the diagnostic: an adapter that adds its own copy of a dependency and "one `cargo update` later, resolves a different major than `happenstance-core` did. The two `Stream` traits print identically, so `impl EventStore for MyStore` fails with `error[E0277]: the trait bound … is not satisfied` naming a trait the author can see is implemented — a diagnostic that sends people to rewrite the adapter rather than to read `cargo tree -d`" (`standards/rust/40-public-surface-and-evolution.md:200-206`). The same sentence written against `rusqlite` is the wrong outcome here, and the finder's version of it is exact: a consumer already carrying `rusqlite` for its own tables, at a requirement that does not overlap the `0.40` this workspace pins (`Cargo.toml:72`), writes `if let SqliteEventStoreError::Sqlite(e) = err { e.sqlite_error_code() }` and gets `E0308: mismatched types` over two types that print identically. They find out at the moment they first try to *act* on a store error — which is late, because the crate's happy path is rusqlite-free and gives no warning: `SqliteEventStore::open` (`event_store.rs:357`) and `open_in_memory` (`:377`) name no foreign type, so a consumer can build, append and read for weeks before touching a signature that does. The crate's own README install block (`crates/happenstance-sqlite/README.md:33-36`) offers `happenstance-sqlite = "0.2.0-alpha.1"` and no companion `rusqlite` line, so nothing tells them which version to add.

Two corrections to the finding as filed. First, the claim that `SqliteEventStore::new` is "uncallable without naming `rusqlite`" is **wrong**: `open` and `open_configured` compose without an annotation, and the coupling bites on the bring-your-own-connection path and the error-inspection path, not on first use. Second, for `happenstance-cloudflare` the version-skew argument is *stronger*, not weaker, than for `happenstance-sqlite`: a consumer necessarily has `worker` in their own manifest, because they write the `#[durable_object]` class the README's own example takes a `&worker::State` from — so the two copies are guaranteed to exist and are guaranteed to be unified by nothing but luck. `worker` is pinned at `0.8.5`, pre-1.0, where the ecosystem treats `0.8 → 0.9` as a major and ships it without 1.0-grade caution.

The workspace has already reasoned this through once, in the contract crate, and left it unexecuted: `RUNBOOK.md:2870-2872` carries an unchecked box for "`pub use futures_core;` beside `pub use bytes;`, since `Stream` appears in `read`'s signature and every adapter is forced to name it" — the identical argument, recorded, unscheduled, and applied to none of the adapters.

**Semver.** Adding `pub use rusqlite;` / `pub use tokio;` / `pub use worker;` is **additive** and free today. What is not free is what happens if it is *not* done: `SqliteEventStore::new(rusqlite::Connection)`, `open_configured -> rusqlite::Result<Connection>` and `SqliteBatch::push(_, impl IntoIterator<Item = rusqlite::types::Value>)` on the registry make every `rusqlite` major a **breaking** release of `happenstance-sqlite`, permanently — and a re-export added in 0.3.0 does not retract the coupling a 0.2.0 consumer already compiled against. Neither crate is on crates.io today (`happenstance`, `happenstance-core` and `happenstance-testkit` are, at `0.2.0-alpha.1`), so 0.2.0 is the last moment this costs one line. `happenstance-cloudflare`'s README states its own status: "no release, no version to depend on, and no frozen API" (`crates/happenstance-cloudflare/README.md:24-25`) — the same window, closing at whichever release ships it.

**Remediation** *(subordinate)*. One `pub use` per foreign crate that appears in a signature, each carrying the doc sentence `happenstance-core` and `happenstance-postgres` already wrote, plus a `rusqlite` line in `happenstance-sqlite/README.md`'s install block so the version a caller must match is stated where they read it. Whether `happenstance-cloudflare` should additionally gain a `[features]` table isolating the `worker`-typed surface is a separate and larger question — the crate has no feature table at all, and `worker` reaches the type of every construction path, so a feature that excludes it excludes the adapter. That is a design decision, not a re-export.

**Routing.** Two decisions, one owner each, neither taken here. (1) *Whether adapter crates re-export their driver* — the same question `RUNBOOK.md:2870-2872` leaves open for `futures_core` in `happenstance-core`; it belongs to whoever owns that RUNBOOK line, decided once for the workspace rather than three times, and it wants an ADR because RS-40-4's "Evidence" line will need repointing at whatever lands. (2) *Whether `happenstance-cloudflare` gains a feature table before it publishes* — release-readiness, and a candidate for `.kb/open-questions/` if 0.2.0 ships without it. Both are pre-publication items: file against the 0.2.0 release gate, not against a phase.

---

### G-2 — `StringifiedThrow` is the one error type in the five publishable crates a caller can mint and mutate, and it exposes a classifier that reads the mutable field

**Clause.** **No specification clause governs the fabricability of an adapter error.** ES-6 [FROZEN] (`spec/SPECIFICATION.md:2677`) covers this type's `Send`/`Sync` standing and nothing else about it. The governing rule is **RS-13-3**, *Reach for `#[non_exhaustive]` with public fields when a value must be readable but not fabricable* (`standards/rust/13-sealing-and-exhaustiveness.md:100-142`), with **RS-13-1** (`:12`) as the alternative it weighs against.

**Examined.** `crates/happenstance-cloudflare/src/js.rs:268-302` and its sibling `:168-173` · `crates/happenstance-cloudflare/src/lib.rs:457` · `crates/happenstance-cloudflare/src/sql_storage.rs:120-133` · an inventory of `#[non_exhaustive]` over every error type in the five `PUBLISHABLE` crates (`xtask/src/package.rs:86-89`).

**Found.**

```rust
// crates/happenstance-cloudflare/src/js.rs:268-273
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("JavaScript threw: {message}")]
pub struct StringifiedThrow {
    /// `String(value)` applied at the boundary.
    pub message: String,
}
```

and, on the same type:

```rust
// crates/happenstance-cloudflare/src/js.rs:299-301
    pub fn is_constraint_violation(&self) -> bool {
        reads_as_constraint_violation(&self.message)
    }
```

It is re-exported at the crate root (`lib.rs:457`). Its sibling keeps the field private for exactly this reason:

```rust
// crates/happenstance-cloudflare/src/js.rs:168-173
pub struct JsThrow {
    /// The thrown value, unstringified, as `worker` classified it.
    error: Rc<worker::Error>,
}
```

The inventory is unanimous against it. Every error enum in the five publishable crates carries `#[non_exhaustive]`: `InvalidTag` (`error.rs:18`), `InvalidEventType` (`:50`), `InvalidQuery` (`:92`), `AppendError` (`:213`), `CommitError` (`projection.rs:228`), `ResetError` (`:269`), `CodecError` (`codec.rs:78`), `CommandError` (`command.rs:97`), `ProjectionError` (`runner.rs:141`), `FaultyStoreError` (`faulty.rs:86`), `SqliteEventStoreError` (`event_store.rs:856`), `SqliteProjectionStoreError` (`projection_store.rs:448`), `CloudflareEventStoreError` (`event_store.rs:699`), `SqlError` (`sql_storage.rs:132`). The only three that do not are `MemoryStoreError {}` and `MemoryProjectionStoreError {}` — uninhabited, and correct under **RS-13-5**, *Do not put `#[non_exhaustive]` on an enum designed not to grow* (`standards/rust/13-sealing-and-exhaustiveness.md:188`) — and `StringifiedThrow`.

**Why a defect.** Two failure modes, and they need separating because only one of them the attribute fixes.

The load-bearing one is semver, and it is unarguable: `StringifiedThrow` is a single-field public struct with no seal, so adding any second field after publication is a breaking change. The obvious second field is the one the crate root already argues about — the numeric SQLite code `worker` "finds no SQLite code to cache" today (`crates/happenstance-cloudflare/src/lib.rs:255-258`). If Workers ever surfaces one, the crate would want to carry it, and carrying it would be a major.

The second is RS-13-3's own hazard, and it is why the attribute is not cosmetic. `is_constraint_violation` is a substring test over a field any holder can assign. RS-13-3's *Rejects* clause describes the structurally identical failure one crate over — a fabricated `Guard.after` that "names a position no read ever returned; the store finds nothing above that position, accepts the append, and the write lands on a decision model nothing ever justified. From every log it is a successful conditional write, which is the one failure mode DCB exists to make impossible" (`standards/rust/13-sealing-and-exhaustiveness.md:134-140`). Here: a layer that reconstructs a `StringifiedThrow` — a test double, a boundary mapper, an error-normalising middleware — sets `message` to text containing `UNIQUE constraint failed`, `is_constraint_violation()` answers `true`, and a DCB command loop treats a transport fault as a lost append condition and retries a decision the store never refused. Nobody finds out, because the retry succeeds.

The honest scope: this type is not on the adapter's own control-flow path. Its in-crate roles are the recorded ES-6 alternative and the positive `Send` control the `!Send` probe module needs (`crates/happenstance-cloudflare/src/lib.rs:544-548` — "Without a positive control this whole module would also pass if the probe were simply broken"). But it is public, re-exported at the crate root, and the crate root actively documents the stringified shape as the `Send + Sync` alternative that loses "a capability, not information the caller needs" (`lib.rs:248-249`). A consumer who takes that route builds these values.

**Semver.** **Breaking**, and free only until `happenstance-cloudflare` publishes — it is in `PUBLISHABLE` (`xtask/src/package.rs:86-89`) and gate-held to publication standards today. `#[non_exhaustive]` on a struct with a public field breaks the struct literal downstream (`error[E0639]`); making the field private additionally breaks reads. Both are one-line changes now and permanent shapes the day after.

**Remediation** *(subordinate)*. Note that `#[non_exhaustive]` alone does **not** close the second hazard — RS-13-3 is explicit that it blocks the literal and forces `..` in a pattern, and does nothing to `throw.message = "…".into()` on a value already held. The shape that closes both is `JsThrow`'s: a private field with a `message()` accessor, under RS-13-1. Whether `StringifiedThrow` should be public at all, given its two in-crate roles are a recorded alternative and a test control, is a prior question and is not this document's to take.

**Routing.** A release-readiness item against whichever release first ships `happenstance-cloudflare`, and — because the choice between RS-13-1 (private field) and RS-13-3 (`#[non_exhaustive]` with a public field) is a decision with a written rule on each side and a public-surface consequence — a candidate for a decision record through `/redkiln:kb-ingest` rather than a line edit. Owner: whoever holds `happenstance-cloudflare`'s publication gate.

---

### X-4 — The workspace's only consumer-facing SQL-text seam takes `impl Into<String>`, and its one-line rustdoc frees the caller of the obligation the crate's own callers all meet

**Clause.** **No specification clause governs this.** `ProjectionStore`'s port is provisional and **ADR-0017** settled that the batch carries no universal write vocabulary — its title is literally *"What a projection batch owns, and the seam that is not a write vocabulary"* (`.kb/decisions/0017-what-a-projection-batch-owns.md:3`, and `:82`: "The write seam is split by consumer rather than given a universal vocabulary on `Batch`"). ADR-0017 answers *what shape the seam is*; it does not answer *how a consumer uses it safely*, and nothing else does either. The applicable house rule is **RS-70-1**'s rustdoc obligations (`standards/rust/70-rustdoc-obligations.md`), whose own limit RS-70-5 states at `:260`: *"Nothing in the gate reads prose."*

**Examined.** `crates/happenstance-sqlite/src/projection_store.rs:396-402, 370-385, 422-428, 572-611, 711-732` · `crates/happenstance-sqlite/Cargo.toml:54-82` · a search for parameterisation guidance across `crates/happenstance-sqlite/src/`, `crates/happenstance-sqlite/README.md` and `docs/`.

**Found.** The entire doc comment on the seam is one line:

```rust
// crates/happenstance-sqlite/src/projection_store.rs:422-423
    /// Queues a statement to run when the batch commits.
    pub fn push(&mut self, sql: impl Into<String>, params: impl IntoIterator<Item = Value>) {
```

No `# Security`, no `# Panics`, no mention of binding. The type-level doc above it frames free-form SQL as the intended use and names `params` nowhere:

```rust
// crates/happenstance-sqlite/src/projection_store.rs:400-402
/// would have to be reported for it or waived for everything. Fill one through
/// [`push`](Self::push) — queueing the application's own read-model SQL is the
/// caller's whole job — and hand it back to `commit` or `reset`.
```

`grep -rni "inject|parameteris|parameteriz|untrusted|trust boundary"` over `crates/happenstance-sqlite/src/`, that crate's README and `docs/` returns **nothing**. The transactional coupling is documented on `commit`, and is what makes the outcome permanent:

```rust
// crates/happenstance-sqlite/src/projection_store.rs:572-576
    /// Applies `batch` and moves `id`'s checkpoint to `position`, as one unit.
    ///
    /// One `BEGIN IMMEDIATE` carries all of it: the recorded checkpoint is read
    /// under the same write lock that will overwrite it, then every queued
    /// statement runs in the order it was pushed, then the checkpoint is upserted
```

**Why a defect.** The discriminator between a missing paragraph and a defect is that this is the *only* place in the workspace a consumer is handed a SQL-text seam, and the values that flow through it are precisely the bytes this library guarantees it does not inspect: `ADR-0003` makes payloads opaque `Bytes`, forwarded and validated by nothing. So the natural line — `batch.push(format!("INSERT INTO seat_map (course, seats) VALUES ('{course}', {seats})"), [])`, with `course` decoded out of an event payload — compiles, reads exactly like the doc invites, and is a SQL injection whose source is the event log. It runs inside the same `BEGIN IMMEDIATE` that advances the checkpoint, so a successful injection is recorded as *progress*: the projection never replays those events, and no later run re-derives the corrupted rows. The wrong outcome's "when do they find out" is therefore *never*, from the library's side.

Against that: the crate's own two callers of `push` both parameterise correctly (`projection_store.rs:713-722`, `:731`), so the code models the right thing — nothing propagates that discipline to the consumer, because the signature does not require it and the doc does not name it.

The finder classed this a nit. It is not a nit, but the theme's framing overstates its permanence, and the honest correction is worth printing. `push` lives behind `projection-store`, which is **off by default** and whose own manifest comment says the surface it enables makes no promise: *"**Enabling this enables an unfrozen port** … `ProjectionStore` is provisional: PS-2's bar is unmet"* (`crates/happenstance-sqlite/Cargo.toml:66-69`). A caller reaching `push` has already opted into an unfrozen port.

**Semver.** Documentation is **non-breaking** and always available. Narrowing the signature — splitting `push` into a parameterised form and a deliberately-named raw form — is **breaking**, and `happenstance-sqlite` has never been published, so it is free at 0.2.0. It is *less* costly after 0.2.0 than the rest of this section, because `projection-store` is opt-in and its manifest already disclaims semver on the surface behind it. That makes this the one entry in this theme whose window is soft.

**Remediation** *(subordinate)*. A paragraph is a control nothing enforces — this repository's own doctrine, at `standards/rust/70-rustdoc-obligations.md:260` and in `CLAUDE.md`'s *"A rule that no adapter can fail is decorative."* The type system can carry this one: the parameterised path takes `&'static str` (or a statement type minted from one) and the free-form path is named so that reaching for it is a decision. Whether that trade is worth a breaking change to an already-opt-in surface is exactly the judgement this document does not make.

**Routing.** The projection-store surface is queued for the freeze that `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` (status `superseded` by ADR-0017) and PS-2's unmet two-adapter bar govern. File this against that freeze, not against 0.2.0: whoever freezes `ProjectionStore` decides the seam's final shape, and a signature narrowed twice is worse than one narrowed once. In the interim, the rustdoc half is a documentation item for the 0.2.0 release gate.

---

### G-1 — `PartialBatch` carries two typed causes and marks neither `#[source]`, so the report of a `[FROZEN]` clause's own violation reaches every error walker as a leaf

**Clause.** **ES-18 [FROZEN]** — *Atomicity* (`spec/SPECIFICATION.md:3392`, marker at `:3397`): "Either every event in the batch lands or none does. A rejected append MUST leave the store byte-identical." `PartialBatch` exists to report the state that clause forbids; **no clause governs `Error::source()`** — `grep -n "source()|source chain|Error::source" spec/SPECIFICATION.md` returns nothing. The governing rule is **RS-30-2**, *Carry a foreign error as a type parameter with `#[source]`, never as a `String`* (`standards/rust/30-error-taxonomy.md:71-129`).

**Examined.** `crates/happenstance-cloudflare/src/event_store.rs:698-781`, `:2026-2060` · against `crates/happenstance/src/runner.rs:174-191`.

**Found.**

```rust
// crates/happenstance-cloudflare/src/event_store.rs:771-780
    #[error(
        "an append failed and its rows from position {from} could not be discarded: {cause} (while discarding: {while_discarding})"
    )]
    PartialBatch {
        /// The first position this batch was given; every row at or above it was
        /// this batch's, and is what the discard was aimed at.
        from: SequencePosition,
        /// Why the append stopped.
        cause: Box<CloudflareEventStoreError>,
        /// Why the rows it had written could not be discarded.
        while_discarding: Box<CloudflareEventStoreError>,
    },
```

`grep -n "fn source" crates/happenstance-cloudflare/src/event_store.rs` returns nothing, and the enum is `#[derive(Debug, Clone, thiserror::Error)]` (`:698`). thiserror populates `source()` from `#[from]`, `#[source]` or a field literally named `source`; `cause` and `while_discarding` are none of the three. The variant is the outlier inside its own enum — its siblings chain: `Sql(#[from] SqlError)` with `#[error(transparent)]` (`:702-703`), `StoredEventType(#[from] InvalidEventType)` (`:726`), `StoredTag(#[from] InvalidTag)` (`:730`).

The workspace already has the correct two-cause shape, in a crate that is on crates.io:

```rust
// crates/happenstance/src/runner.rs:174-191
    #[error("reading the event stream failed")]
    Read {
        /// What the run had committed before this.
        progress: Progressed,
        /// The event store's own refusal.
        #[source]
        source: R,
        /// A rollback that also failed while the chunk was being discarded.
        ///
        /// Carried beside the cause rather than replacing it: the cause is what
        /// a reader must act on, and a rollback failure reported in its place
        /// would hide it.
        rollback: Option<W>,
    },
```

The only test over `PartialBatch` asserts on `Display` (`event_store.rs:2050-2059`: `cause.to_string().contains("no space left on device")`), not on the chain.

**Why a defect.** RS-30-2's *Why* states the discriminator directly: "`#[source]` is what populates `Error::source()`, and a `dyn Error` source can be downcast back to its concrete type … the information is still printable and no longer *actionable*" (`standards/rust/30-error-taxonomy.md:73-77`). This is a stricter case than the one the atom rejects: the typed value **survives** and the chain is broken anyway, so the failure is invisible to review — the field is right there in the `Debug` output. `anyhow`'s `{:#}` and `.chain()`, `tracing-error`, Sentry-style reporters and any hand-written `while let Some(next) = e.source()` loop all report `PartialBatch` as a leaf.

The wrong outcome is sharpened by *which* variant this is. `PartialBatch` is the one failure of this adapter that a DCB command loop must not retry — the variant's own doc says so: "Every other failure of `append` leaves the log as it found it, so a DCB command loop may re-read and retry. This one does not: the log now contains events the caller's own failed append put there, and a retry would decide against them" (`event_store.rs:760-764`). It is the report of an ES-18 [FROZEN] violation, and it reaches an operator's structured logging with its root cause stripped. The remedy left is substring-matching a Display string — which is what RS-30-2's *Rejects* clause describes as the end state ("the only remedy is substring matching on a message the transport is free to reword").

**Semver.** **None.** Adding `#[source]` changes no type, no field visibility and no `Display` output — thiserror's `#[source]` and the `#[error(...)]` format string are independent, and `{cause}` keeps rendering. This one costs the same before and after publication and does not consume the breaking-change window.

**Remediation** *(subordinate)*. thiserror permits one `#[source]` per variant, so the shape available is `runner.rs`'s exactly: mark `cause` — the primary, actionable failure — and leave `while_discarding` carried beside it, documented as out-of-chain the way `rollback` is at `runner.rs:186-189`. A test asserting `Error::source(&err).and_then(|s| s.downcast_ref::<CloudflareEventStoreError>()).is_some()`, sitting beside the existing Display assertion at `event_store.rs:2050`, is what keeps it from regressing — the current test cannot fail on this.

**Routing.** No decision is owed: RS-30-2 already states the rule and `runner.rs` already applies it, so this is a defect against a rule rather than a question about one. File it as a `.bklg/` fix story against `happenstance-cloudflare`'s release-readiness, with the source-chain assertion as its acceptance criterion. It has no deadline of its own and should not be allowed to displace the breaking items above it in this section.


---

## `cost-numbers-measured-something-else` — every performance figure this workspace publishes was measured against a shape the shipped code does not have

Two experiments built for this review put the theme's thesis to a measurement, and it survives: in `happenstance-sqlite` every published append-condition figure was taken on `GROUP BY position HAVING COUNT(DISTINCT tag) = n`, and the adapter emits a correlated intersection chain instead — which, measured against that aggregate for the first time, **loses in 9 of 9 two-tag cells at 1.54x–1.86x**; in `happenstance-core` the arithmetic is simply wrong, an `Event` clone costing **66 heap allocations at VT-22's 64-tag floor** where four corpus sites say two. The thesis needed one correction and got it in public: I-2's proposed remedy — binding `position > ?` into the guard's seed arm — is **falsified**, at +2.0% with no consistent sign across nine cells, because `EXPLAIN QUERY PLAN` shows the seed is the probe side and not the driving table. The measurements also surfaced something no finding asked for and which is the theme's sharpest instance: `event_store.rs:91-96` states in *rendered* public documentation that multi-tag items "must be probed most-selective-tag-first", and on the shipped shape that policy costs **38x–44x**, replicated on two fresh stores, because it puts the larger set on the side SQLite materialises. Read every absolute below as a warm-cache figure on one host: the 50,000-event calibration did **not** reproduce ADR-0022 §1's 42,399 µs — the same SQL shape costs 14,085 µs here, 3.0x less, for page-cache reasons `experiments/shipped-append-condition-sql/README.md:220-235` sets out — so the ratios carry and the absolutes do not. Nothing in this section is fixed, no clause is amended and no ADR is written; each entry names a number, a site, and whose decision it is.

---

### I-1 — The shipped multi-tag query is an intersection chain, and it loses to the aggregate whose measurements were used to justify it

**Clause.** None governs the SQL shape. The nearest clause is **ES-27** — *A condition matches on tags, not only on types* (`spec/SPECIFICATION.md:3856-3862`), **`[FROZEN]`** (`spec/SPECIFICATION.md:9104`) — which mandates *that* tags are matched and not how, but whose `Rejects:` prose repeats the derived figure: *"measured at roughly 200x a single-tag boundary's cost at 50,000 events, which is why ADR-0022 ships `tag_cardinality` and most-selective-tag-first probing as requirements"* (`spec/SPECIFICATION.md:3902-3905`). The governing record is `references/adr/0022-append-condition-strategy.md:353-386` (§8), accepted and immutable.

**Examined.** `crates/happenstance-sqlite/src/query_sql.rs:196-235` (`item_sql`), `:34-54` (the module doc that prices it), `crates/happenstance-sqlite/src/event_store.rs:91-96`, `references/adr/0022-append-condition-strategy.md:353-386` and `:589-607` (§16's falsifiers), against `experiments/shipped-append-condition-sql/results/guard-cost.md` §2 and `experiments/shipped-append-condition-sql/results/query-plans.md` §1.

**Found.** The general form ADR-0022 measured, `references/adr/0022-append-condition-strategy.md:354-356`:

> The general superset test is `GROUP BY position HAVING COUNT(DISTINCT tag) = n`.

What the adapter emits, `crates/happenstance-sqlite/src/query_sql.rs:232-235`:

```rust
    for tag in &tags[1..] {
        params.push(Value::Text(tag.clone()));
        sql.push_str(" AND position IN (SELECT position FROM event_tag WHERE tag = ?)");
    }
```

No aggregate anywhere. `query_sql.rs:47-50` nevertheless prices the chain from the aggregate's numbers — *"Multi-tag items take an **intersection chain seeded by the most selective tag**, which keeps the boundary pushable for the same reason"* — where "the same reason" is the 1,093 µs → 556 µs single-tag measurement taken on the `GROUP BY` form.

Measured, `guard-cost.md` §2a, `guard_us` medians, 15–120 interleaved rounds per cell on one shared file, `chain-as-shipped ÷ grouped-adr0022`: 1.56x / 1.60x / **1.54x** on `accepted-2tag-at-head` at 50,000 / 500,000 / 10^6 events; 1.79x / 1.61x / 1.56x on `rejected-2tag-unbounded`; 1.75x / 1.58x / **1.86x** on `rejected-2tag-midlog`. Nine cells, one direction. At 10^6 events with the guard anchored at head that is **559,591 µs against 362,385 µs**. `query-plans.md` §1 gives the mechanism: the chained subquery is uncorrelated, so SQLite materialises it as a `LIST SUBQUERY` over the whole of the least selective tag, while `GROUP BY` compiles to a `CO-ROUTINE` and materialises nothing into a list at all.

Two facts sharpen and two qualify. Sharpening: the chain's transcription tracks the **real adapter** to within 7% in every cell above a millisecond (`guard-cost.md` §3), so this is not a straw shape; and the 200x published in ES-27's frozen prose understates badly on this host — the like-named single-tag cell `rejected-1tag-unbounded` at 50,000 events is 11 µs against the chain's 25,204 µs, a **derived** ratio of roughly 2,300x, computed across two cells of one interleaved run and not asserted by the experiment itself. Qualifying: `query_sql` is a **private** module (`crates/happenstance-sqlite/src/lib.rs:89`), so `:34-54` is read by maintainers rather than rendered on docs.rs — only `event_store.rs:91-96` (`pub mod event_store`, `lib.rs:86`) and ES-27's prose reach a stranger; and the chain **is** conformance-tested against the real store, so this is an evidential defect, not a correctness one.

**Why a defect and not a preference.** The discriminator is not that the chain is slower — a slower shape chosen with a number beside it is a trade. It is that ADR-0022 §16's falsifier for §8 is *"re-open it if a future SQLite pushes predicates through an aggregate, at which point the special case stops earning its branch"* (`references/adr/0022-append-condition-strategy.md:605-607`), and it can never fire, because the aggregate it names is not what runs. A decision whose stated re-opening condition is unreachable has stopped being a decision and become a sentence.

**Semver.** None. `query_sql` is private, `happenstance-sqlite` is unpublished and joins the registry at `0.2.0`, and the emitted SQL is not API. The prose correction is free at any time; changing the emitted shape is free only until `0.2.0` ships a schema and a plan people measure against.

**Remediation, subordinate.** Two separable pieces, and only the first is cheap: correct the four sentences that price the chain from a measurement of the aggregate, and separately decide whether the chain stays. `grouped-adr0022` is already a built, conformant arm (`experiments/shipped-append-condition-sql`, 89 rules × 4 shapes, 356 passed), so the second is a decision with evidence rather than an experiment to schedule. Note `guard-cost.md` §1: the single-tag path is 4–16 µs at every size on all four shapes, so nothing here touches the fast path.

**Routing.** A superseding decision record for ADR-0022 §8, owned by whoever holds `.kb/decisions/0022`; ADR-0022 is accepted and therefore immutable, so §8's requirement cannot be edited and §16's dead falsifier cannot be repointed by hand. ES-27's `Rejects:` prose carries the derived 200x figure inside a `[FROZEN]` clause, so repairing that number is a clause edit and goes through the same record, not through a documentation pass. `event_store.rs:91-96` and `query_sql.rs:34-54` are rustdoc and follow the record rather than leading it.

---

### CN-1 — `tag_cardinality`'s most-selective-first ordering is stated as a requirement in rendered documentation and costs the shipped chain 38x–44x

*New in this pass. It arises from MEASUREMENT 2 rather than from the finding intake, and is filed separately from I-1 because the site, the wrong outcome and the owner all differ.*

**Clause.** None mandates the ordering. **ES-27 `[FROZEN]`** (`spec/SPECIFICATION.md:9104`) is the nearest, and its `Rejects:` prose asserts the policy's provenance: *"which is why ADR-0022 ships `tag_cardinality` and most-selective-tag-first probing as requirements"* (`spec/SPECIFICATION.md:3904-3905`). The requirement itself is `references/adr/0022-append-condition-strategy.md:374-379` (§8, item 1), accepted and immutable.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:91-96` (rendered module documentation, `lib.rs:86`), `crates/happenstance-sqlite/src/query_sql.rs:114-117` and `:199`, against `experiments/shipped-append-condition-sql/results/seed-ordering.md` and its replication.

**Found.** `crates/happenstance-sqlite/src/event_store.rs:92-96`:

```
//! * **`tag_cardinality` is a requirement rather than a convenience.**
//!   Multi-tag items must be probed most-selective-tag-first and SQLite cannot
//!   supply per-value cardinality: `ANALYZE` stores an *average*, which is
//!   exactly wrong for a tag set where one value matches a third of the log and
//!   another matches one percent.
```

The implementation, `crates/happenstance-sqlite/src/query_sql.rs:114-117`, sorts ascending by count so the least-matching tag lands in the seed arm:

```rust
    fn most_selective_first(&self, mut tags: Vec<String>) -> Vec<String> {
        tags.sort_by_key(|tag| self.0.get(tag).copied().unwrap_or(0));
        tags
    }
```

Measured on one 500,000-event store, one connection, one builder, **one statement text** — `Selectivity::inverted()` flips only which parameters are bound — with the running order alternating every round and every round asserting both orderings returned the identical position: most-selective-first **511,054 µs** against least-selective-first **11,839 µs** on `accepted-2tag-at-head` (43.2x); 553,328 against 13,340 (41.5x); 634,239 against 14,399 (44.0x). An independent re-run on a fresh store: 39.5x, 41.5x, 38.1x. Six cells, two runs, one direction. `query-plans.md` §1 supplies the mechanism and `seed-ordering.md` the arithmetic — the chained subquery is what SQLite materialises, so cost is the size of the *chained* tag's history: 500,000 list entries under the shipped ordering against 5,155 under its inverse, 97x of predicted work against 38–44x of measured time.

**Why a defect and not a preference.** The sentence was measured on the `GROUP BY` form, where `tag IN (?,?)` is order-independent and the ordering question does not arise, and inherited by a shape where the planner picks the driving side itself. It is not a tuning preference stated too strongly; it is a policy whose sign is inverted on the code that ships, published as a requirement in the crate's rendered documentation, where an adapter author or a reviewer reads it as settled.

**Semver.** None — `Selectivity` and `most_selective_first` are crate-private, and `happenstance-sqlite` joins the registry at `0.2.0`. The write-lock cost is a runtime property, not a signature, so it is repairable at any version; `0.2.0` matters only because it is when the sentence starts being read by strangers.

**Remediation, subordinate.** The measurement falsifies the stated rule for the two-tag case and deliberately does **not** supply a replacement: `seed-ordering.md`'s own limits say so — with three or more tags the plan has more than one materialisation to choose between and the single-term arithmetic no longer holds — and it makes no claim about the `GROUP BY` form, where the ordering is a no-op. So "invert the sort" is a two-tag answer to an n-tag policy and should not be taken as the fix. Note also `query_sql.rs:115`'s `unwrap_or(0)`: a tag absent from `tag_cardinality` sorts as maximally selective, which is the seed arm on the shipped ordering.

**Routing.** The same superseding record I-1 routes to, because §8's items 1 and 2 are one paragraph and one of them is now measured wrong on the shipped shape. Whoever holds `.kb/decisions/0022` owns it. Until that record exists, `event_store.rs:91-96` is a rendered claim the repository's own instrument contradicts, and the review takes no view on whether the sentence, the sort, or the chain is the thing that should move.

---

### I-2 — The append guard emits no boundary predicate; the mechanism is confirmed and the finding's proposed remedy is falsified

**Clause.** None governs where the boundary is evaluated. **ES-26** — *the AC3 boundary: `after` is exclusive* (`spec/SPECIFICATION.md:3815`) — fixes the semantics, which the shipped code satisfies; nothing in the specification says the comparison must reach SQL.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:642-674` (`evaluate`), `crates/happenstance-sqlite/src/query_sql.rs:154-171` (`chunks`, which takes no boundary parameter on either caller) and `:48-50`, against `experiments/shipped-append-condition-sql/results/guard-cost.md` §2b/§2c and `results/query-plans.md` §1.

**Found.** The fact holds exactly as filed. `crates/happenstance-sqlite/src/event_store.rs:658-672`:

```rust
            let chunk: Option<i64> = connection.query_row(
                &format!("SELECT max(position) FROM ({matched})"),
                rusqlite::params_from_iter(params.iter()),
                |row| row.get(0),
            )?;
            highest = highest.max(chunk);
        }

        let boundary = guard.after.map_or(0, as_i64);
        if let Some(highest) = highest
            && highest > boundary
```

No `position > ?` reaches SQLite, on either caller, which `results/raw/emitted-sql.txt` confirms off a `sqlite3_trace_v2` callback on a real `SqliteEventStore` rather than by reading the builder. So `query_sql.rs:48` crediting the chain with *"keeps the boundary pushable"* names a mechanism the shipped strategy never exercises.

**The remedy the finding proposed is falsified, and this is worth more than the confirmation.** Binding `position > ?` into the seed arm — `chain-bounded-seed`, a built and conformance-cleared arm — costs **+2.0%** at 10^6 events on `accepted-2tag-at-head` (570,741 µs against 559,591 µs), inside both cells' bands (`us_p10=485,321`/`us_max=4,685,317` against `us_p10=516,599`/`us_max=741,044`). Across nine cells the deltas are +8.8%, −4.5%, +2.0%, −3.2%, +2.6%, +1.6%, −7.2%, −2.9%, −7.4% — no consistent sign, none outside its own spread. `query-plans.md` §1 explains why, and it is the opposite of the finding's reasoning: the two plans are identical line for line, the seed is the **probe** side, and the predicate lands on the arm already being seeked one row at a time rather than on the `LIST SUBQUERY` it would have to shrink.

The control that proves the null is real rather than a broken instrument is `chain-bounded-all-arms`, which binds the boundary into the materialised subquery too: **179 µs against 559,591 µs**, 3,126x, at a head boundary — with the two other boundaries behaving exactly as the mechanism predicts, 2.0x–2.33x at mid-log where half the range survives and 0.99x–1.02x at boundary zero where nothing can be discarded. Three predictions, three sizes, nine agreements.

**Why a defect and not a preference.** Not the missing predicate on its own — the guard is correct, and `evaluate`'s Rust-side comparison is conformant. The defect is that a rendered rationale credits a shipped choice with a property the shipped choice does not have, and the review's own proposed correction was reasoned from that same rationale and was wrong. The cost stands: `guard-cost.md` §2 puts a two-tag guard at 296 ms at 500,000 events and 560 ms at 10^6, inside `BEGIN IMMEDIATE` with every other writer queued behind it, and `query-plans.md`'s cold column records **3.97 s** for the shipped chain's first execution at 10^6 rows — the figure an application meets after a restart.

**Semver.** None; additive at most. The guard is crate-private and `happenstance-sqlite` is unpublished.

**Remediation, subordinate.** `chain-bounded-all-arms` is the only one of the four shapes that makes `query_sql.rs:48-50`'s sentence true, and it exists nowhere in the repository. It is also the shape with the least evidence behind it — two tags, one event type, a uniformly spread selective tag, warm cache — and adopting a 3,126x on that basis is precisely the move this section is about. Treat it as an arm to be built and measured against three-tag guards and a clustered tag, not as a patch.

**Routing.** Same superseding record as I-1 and CN-1 — the three are one paragraph of §8 seen from three sides. The falsification itself is a result and belongs in the record's evidence, not only in this document; the review notes explicitly that a finding it filed was refuted by the experiment it asked for.

---

### I-3 — The paged read is unmeasured, and the clause `fetch_page` carries for `Query::all` costs 846x

**Clause.** None governs paging or its cost. **ES-11** — *A read is a snapshot* (`spec/SPECIFICATION.md:2976`), **`[PROVISIONAL]`** (`spec/SPECIFICATION.md:9088`) — is what the paging exists to satisfy, and the shipped shape satisfies it.

**Examined.** `crates/happenstance-sqlite/src/event_store.rs:136-141` (`PAGE_SIZE`), `:1301-1330` (`fetch_page`), `crates/happenstance-sqlite/src/query_sql.rs:161`, `references/adr/0022-append-condition-strategy.md:570-587` (§15), against `experiments/shipped-append-condition-sql/results/query-plans.md` §2.

**Found.** The repository already records this gap in an accepted, immutable ADR — `references/adr/0022-append-condition-strategy.md:585-587`, §15 *"What this record deliberately does not do"*:

> - **It does not touch `PAGE_SIZE`.** `event_store.rs:81` calls the current 512 "a placeholder until it is measured" and it stays one; nothing here measured paging.

`crates/happenstance-sqlite/src/event_store.rs:1323` wraps every page in the arm SQL, and `crates/happenstance-sqlite/src/query_sql.rs:161` returns the bare table for `Query::all`:

```rust
            let mut sql = format!("SELECT {columns} FROM event WHERE position IN ({matched})");
```
```rust
        None => vec![("SELECT position FROM event".to_owned(), Vec::new())],
```

So a `Query::all` replay emits `WHERE position IN (SELECT position FROM event)` — a tautology over the table it selects from. Measured at page 1,000 of a 10^6-event store: **60,092 µs warm**, against **71 µs** for the identical statement with that clause deleted returning the identical 512 rows — **846x**. The plan says why: with the clause SQLite drives from `USING ROWID SEARCH ON TABLE event FOR IN-OPERATOR` instead of from the `rowid>? AND rowid<?` range the statement already carries. A full `Query::all` replay of 10^6 events **through the real adapter** issues 1,954 paged statements and takes **97.2 s**, 49.8 ms per page for everything the adapter does, against 0.14 s for 1,954 executions of the control.

Honour the experiment's own limit: the 97.2 s and the isolated 60.1 ms **bracket rather than subtract**, and removing the clause and re-measuring the replay is the experiment that would close the gap and was not run. `Query::all` is also the worst case for this clause as well as the commonest — a query with real tag items emits a genuinely selective arm, and nothing here measures that case.

**Why a defect and not a preference.** The clause cannot remove a row; it only takes the plan away from a range scan the statement already carries. And the number that chose the tag storage — §6's join table at 3.16x the canonical blob and 4.63x JSON1 — came from a single statement buffering the whole result into a `Vec`, a shape §15 states the adapter deliberately does not have. The commonest read in the library is therefore priced by a measurement of a read the adapter cannot perform, and a projection runner catching up over a large log is where it is found.

**Semver.** None. `PAGE_SIZE` and `fetch_page` are private. `happenstance-cloudflare` carries the same shape at `crates/happenstance-cloudflare/src/event_store.rs:1282` and **is** live at `0.2.0-alpha.1` as a release-bound crate, so the same clause reaches the registry either way; nothing about repairing it is a breaking change.

**Remediation, subordinate.** Omitting the wrapper when the arm is the unconstrained `SELECT position FROM event` is the obvious shape and it is one branch, but the measurement covers the statement and not the replay, so the claim it supports is "the clause is very nearly all of the 97.2 s", not "removing it makes the replay 0.14 s". `PAGE_SIZE`'s own comment still says "a placeholder until it is measured", and it still is.

**Routing.** Corroborates ADR-0022 §15's own recorded non-verdict; the record that lifts it is the same one that owns `PAGE_SIZE`. The paged-read measurement is phase-8-shaped work with no named owner — that absence is the routing, and the decision of who takes it belongs to the RUNBOOK.

---

### I-5 — Tag deduplication is quadratic on both query-planning paths, once per read page and once per append guard under the write lock

**Clause.** None governs planning cost. **VT-23** — *`MIN_SUPPORTED_QUERY_ITEMS` is a floor of 128* (`spec/SPECIFICATION.md:1565-1575`), **`[PROVISIONAL]`** (`spec/SPECIFICATION.md:9044`) — is what makes the measured shape a conformance floor rather than a corner: it requires every store to evaluate 128 items, and nothing anywhere bounds tags per item.

**Examined.** `crates/happenstance-sqlite/src/query_sql.rs:82-91` (`Selectivity::read_for`) and `:240-249` (`distinct_tags`), reached from `crates/happenstance-sqlite/src/event_store.rs:1301` (per read page) and `:647` (per append guard, inside `BEGIN IMMEDIATE`), against `experiments/shipped-append-condition-sql/results/selectivity.md` §1.

**Found.** `crates/happenstance-sqlite/src/query_sql.rs:240-249`:

```rust
fn distinct_tags(item: &QueryItem) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(item.tags().len());
    for tag in item.tags() {
        let value = tag.as_str().to_owned();
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}
```

and the same pattern one level up, at `:86`, accumulating across items: `if !wanted.contains(&tag) { wanted.push(tag); }`.

Measured at VT-23's own floor — 128 items × 128 tags, 16,384 tags presented to the planning path, 25 rounds, medians: `Selectivity::read_for`'s accumulation **257,690 µs** against a `BTreeSet`'s **6,424 µs**, **40.1x**, with the two outputs asserted byte-identical before either was timed. The `shared-tags` control separates the two quadratics: 128 items naming the *same* 128 tags collapse the cross-item cost to 6,150 µs while the per-item one barely moves (4,595 → 5,033 µs). So of the 258 ms, roughly 250 ms is `read_for`'s `wanted.contains` and about 4.6–5.0 ms is `distinct_tags`.

Honour the experiment's limit precisely: only the second is reachable through a shipped public entry point. `SqliteEventStore::planned_statement_count` passes `Selectivity::default()` (`event_store.rs:288-291`) and never reaches `read_for` at all, so the 257.7 ms is a line-for-line transcription of a crate-private function — there is no seam from outside the crate that can reach it — and the page makes no claim that any application builds this query.

**Why a defect and not a preference.** The shape is not a corner someone invented; it is the floor the specification requires every store to survive, and the append-path call happens **inside `BEGIN IMMEDIATE`** with the write lock held, so a quarter-second of pure-Rust planning is a quarter-second every other writer waits. The read-path call runs once per 512-row page, so it multiplies by the page count of a replay.

**Semver.** None; both functions are private and the fix is a container swap with output asserted identical.

**Remediation, subordinate.** Note what the finding's own remediation pass established and the measurement does not contradict: **VT-16 `[FROZEN]`** already guarantees a `Tags` value contains no duplicates, so `distinct_tags`'s dedup is dead work rather than merely quadratic — a different repair from `read_for`'s, which genuinely accumulates across items and needs a set.

**Semver overlap.** This entry shares its measurement file with X-1 (the parameter-count ceiling), which belongs to another section. The two are separate defects on one call path and are not merged here; `selectivity.md` §2 carries X-1's rows.

**Routing.** Adapter-local and inside the crate joining the registry at `0.2.0`; it needs no decision record, and the routing is a backlog row against `happenstance-sqlite` with the measurement attached. The one thing that is *not* routine is the append-path site, which fails or stalls with the write lock held — whoever owns the append path's error and latency budget should see the 257.7 ms figure before `0.2.0`.

---

### AE-1 — Four corpus sites price an `Event` clone at two allocations; it is `t + 2`, and the falsifier that decides `append`'s ownership can be satisfied by a harness structurally unable to see it

**Clause.** **ES-17** — *The batch is borrowed, not owned* (`spec/SPECIFICATION.md:3345-3353`), **`[PROVISIONAL]`** (`spec/SPECIFICATION.md:9094`), whose marker reads: *"falsified by a measurement on a real adapter showing the per-event clone is a material fraction of append cost. The named measurement is the SQLite adapter's multi-row insert benchmark, in the phase that builds it."*

**Examined.** `crates/happenstance-core/src/event.rs:404-413`, `crates/happenstance-core/src/memory.rs:29-31` and `:388-399`, `spec/SPECIFICATION.md:3370-3373`, `references/adr/0012-append-shape-and-preconditions.md:173-174`, against `crates/happenstance-core/src/tag.rs:79` and `:281` and `experiments/event-clone-allocations/results/clone-cost.md`.

**Found.** `crates/happenstance-core/src/event.rs:410-413`:

> an owning adapter clones instead, and that clone is cheap — the expensive fields are [`Bytes`], so it bumps a refcount rather than copying the payload, leaving one `Box<str>` and one boxed tag slice.

`spec/SPECIFICATION.md:3371-3373`, inside ES-17's own rationale:

> the remaining cost is one `Box<str>` and one boxed tag slice, bounded by the tag count.

The types make it `t + 2`: `pub struct Tags(Box<[Tag]>)` (`crates/happenstance-core/src/tag.rs:281`) and `pub struct Tag(Cow<'static, str>)` (`:79`), and `Tags::from_pairs` (`:304-312`) routes every tag through `Tag::new`'s `Cow::Owned` (`:93`).

Measured, one `event.clone()`, steady state, payload `Bytes::from_static` so nothing in the row is the payload: **66 heap operations requesting 2,001 bytes** at VT-22's 64-tag floor, against **1 operation and 1,536 bytes** in the `Tag::from_static` control — **66.0x**. Exact at 1, 8, 32 and 64 tags. At **zero** tags it is **1**, not 2, because `<[T]>::to_vec()` on an empty slice allocates nothing, so the published figure is wrong in both directions: it overcounts a tagless event by one and undercounts a 64-tag event by sixty-four. The instrument's own control on the unit: `Tag::new("k00:v00").clone()` is 1 heap op, `Tag::from_static(…)` is 0.

**The trap is confirmed as predicted, and it is the load-bearing half.** The `Tag::from_static` arm is flat at **1 allocation** across 1, 8, 32 and 64 tags — it does not respond to tag count at all — and it is the arm a benchmark author writes without choosing to, because `from_static` constants are what a test fixture holds. `arms_are_equivalent.rs` proves the two regimes build `==` events and encode byte-identically in `serde_json` and `postcard`, so the cheap arm is not winning by carrying less.

One thing no finding claimed, found on the way: `Bytes::clone` is a refcount bump *from the second clone onwards*. A payload built with `Bytes::from(Vec<u8>)` — the shape every decoded payload has — starts promotable, and its **first** clone allocates a shared header: 67 against 66 at 64 owned tags, 2 against 1 at 64 static tags. One extra allocation per payload, paid by any store that clones each appended event once, which `crates/happenstance-core/src/memory.rs:396` does.

**Why a defect and not a preference.** ES-17 is the clause that decides whether `EventStore::append` takes `&[Event]` or `Vec<Event>`, and its falsifier is a benchmark. A benchmark built the natural way measures a regime with a 66x cheaper clone that is flat in tag count, reports the clone immaterial, lifts the marker, and freezes a signature on evidence that could not have gone the other way. That is not an imprecise sentence; it is a falsifier that can be satisfied by construction.

**Semver.** The documentation correction is non-breaking and free. The decision it feeds is **breaking**: `EventStore::append`'s signature is on `happenstance-core`, live at `0.2.0-alpha.1`, so `0.2.0` is the last cheap moment — and ADR-0012's falsifier item 5 obliges a positive result to *add* "a cheap way to keep a copy for retry", which is additive surface on the same crate in the same window.

**Remediation, subordinate.** The instrument the finding proposed — a counting `#[global_allocator]` in a host test in `crates/` — is **illegal in this repository**: `GlobalAlloc` requires `unsafe impl` and the workspace root sets `unsafe_code = "forbid"`, which cannot be overridden from inside the crate, and a global allocator is a per-binary singleton that would install into every test binary linking the crate. That is why the measurement lives in `experiments/event-clone-allocations/` and cannot be a gate step (CF-34). The corpus also already knew the right arithmetic once: `references/evaluation/research-rust-api-guidelines.md:103-104` says *"`Tags(Box<[Tag]>)` is `n + 1` allocations… five allocations per event"*, and it did not survive into ADR-0012 or the clause.

**Routing.** Corroborates `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` (accepted), which records that ADR-0012's falsifier item 1 — *two builds of the same SQLite adapter differing only in `append`'s ownership* — is scheduled by nobody, and names two triggers: whoever next proposes lifting ES-17, or phase 12. This section adds a third obligation to that record rather than settling it: **whichever build takes that measurement must state which tag regime its fixtures are in**, because the answer differs by 66x at the specification's own floor. The clause text is `[PROVISIONAL]` and the four documentation sites are not, so three of them (`event.rs:410-413`, `memory.rs:29-31`, `references/adr/0012:173-174`) are rustdoc and record and one is a clause; correcting the clause is a spec change and goes to whoever holds ES-17. `event.rs:411` is stale twice over — it still names `Box<str>`, which ADR-0015 replaced with `Cow<'static, str>` workspace-wide (`references/adr/0015-validated-identifiers-and-store-limits.md:245`).

---

### AE-2 — Every `Serialize` impl deep-clones into an owned wire mirror; `SequencedEvent` does it twice, exactly, at every tag count in both formats

**Clause.** None governs encode-path allocation. **WF-5**'s field obligations (`crates/happenstance-core/src/event.rs:780-790` documents them on `SequencedEventWire`) govern what the mirror must carry, not how it is built.

**Examined.** `crates/happenstance-core/src/event.rs:753-762` and `:800-810`, `crates/happenstance-core/src/query.rs:377-384` and `:414-421`, `crates/happenstance-core/src/append.rs:299-311`, against `experiments/event-clone-allocations/results/clone-cost.md` §"Arms 3 and 4".

**Found.** `crates/happenstance-core/src/event.rs:753-762`:

```rust
    impl Serialize for Event {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            EventWire {
                event_type: self.event_type().clone(),
                data: self.data().clone(),
                tags: self.tags().clone(),
                metadata: self.metadata().cloned(),
            }
            .serialize(serializer)
        }
    }
```

and `:800-810`, where `SequencedEventWire` takes `event: self.event.clone()` and its derived impl then calls the above, cloning all four fields again. `crates/happenstance-core/src/query.rs:418` is the same shape (`Self::Items(items) => QueryWire::Items(items.to_vec())`), as are `query.rs:379-381` and `append.rs:305-307`.

Measured, with the encoder's own allocations cancelled by the byte-identity control (owned arm minus static arm at the same tag count and format): the delta is `t + 1` for `Serialize for Event` and `2(t + 1)` for `Serialize for SequencedEvent` — **ratio 2.00 at 0, 1, 8, 32 and 64 tags, in `postcard` and in `serde_json` alike**. In absolutes: encoding **one** 64-tag `SequencedEvent` to postcard costs **140 heap operations to produce 587 bytes**, of which **130 (93%) are transient clones that produce no output**; the same value in the borrowed regime costs 10. One 64-tag `QueryItem` costs 139 heap ops owned against 11 borrowed, for 516 bytes of output.

**Why a defect and not a preference.** The `serde` feature exists in this crate for one consumer — `crates/happenstance-core/src/lib.rs:90-92` says it is *"enabled by replication adapters that need one"* — and a replicated event is precisely one that carries tags. A sync runner pushing a batch at the crate's own two floors (128 events, 64 tags) does `2 × 128 × 66 = 16,896` transient allocations that produce no bytes, on top of the encoding, and the target it lands hardest on is the memory-constrained one.

**Semver.** None, in both directions. The wire mirrors are private inside `#[cfg(feature = "serde")] mod serde_impls`, and serde's idiom for this — one owned mirror for `Deserialize`, one borrowing mirror for `Serialize`, same derive, same renames, same field order — moves no byte.

**Remediation, subordinate.** `payload::optional::Encode<'a>` already exists at `crates/happenstance-core/src/event.rs:667` and already routes a *borrowed* `Bytes` through the human-readable branch, so the borrowing half of the pattern is partly built. The byte-identity control in `experiments/event-clone-allocations/tests/arms_are_equivalent.rs` — two formats, both value types, `postcard` framed with a sentinel — is the shape of the regression test such a change would need, and `crates/happenstance-core/tests/wire.rs` already uses the same framing trick for the same reason.

**Routing.** Adapter-free and clause-free: this is a `happenstance-core` implementation change with no decision above it, so it is a backlog row rather than a record. It belongs before `happenstance-sync` exists, not because of semver but because the crate that would notice has not been built yet — which is also why nothing found it. Note the instrument the finding proposed (a tagged `attempt` in `crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs`, comparing `peak_pages` against the tagless attempt) was **not** the one run; the wasm32 residency delta is still unmeasured, and the host allocation counts above are counts and requested bytes, not resident memory.

---

### H2 — `MemoryEventStore::read` clones the whole matched set before truncating, so `limit` buys the caller nothing

**Clause.** **ES-14** — *`limit` truncates the whole result, after ordering* (`spec/SPECIFICATION.md:3169-3180`), **`[FROZEN]`** (`spec/SPECIFICATION.md:9091`). The clause requires `limit` to apply to the filtered, ordered result set; it does not require materialising that set first.

**Examined.** `crates/happenstance-core/src/memory.rs:296-336`, and `:29-31`, against `experiments/event-clone-allocations/results/read-limit.md`.

**Found.** `crates/happenstance-core/src/memory.rs:312-325` builds the full vector in both branches, ending `.cloned() / .collect()`, and only then, at `:330-332`:

```rust
        if let Some(limit) = options.limit {
            selected.truncate(limit);
        }
```

The store's own doc, `crates/happenstance-core/src/memory.rs:30-31`, prices the snapshot: *"Cloning is cheap regardless: payloads are [`Bytes`](bytes::Bytes), so a snapshot bumps refcounts rather than copying data."*

Measured on a store of a million two-tag events filled through the owned regime, `limit(1)` against a fully-matching query: **4,000,020 heap operations requesting 229,995,520 bytes**, about 311–383 ms, to return **one** event. With `.take(limit)` above `.cloned()`: **5 allocations, 655 bytes, 2.0 µs**. At VT-22's 64-tag floor and only ten thousand events: 660,014 allocations and 22 MB, against 67 and 2,577 bytes.

The control matters more than the ratio. `limit=1` against `limit=None`: 40,014 against 40,026 heap ops at 10,000 events, and **4,000,020 against 4,000,038 at a million — 1.0000x to four significant figures.** Asking for one event out of a million costs what asking for all of them costs; the only difference is in `deallocs`, where the `limit=1` row frees 3,999,997 of the four million before returning, because `truncate` drops the tail it had just finished cloning. And the fix must be free where there is no limit, which it is: at `limit=None` both arms cost identically, 40,013 heap ops and 3,149,296 bytes.

Quote the allocation columns and not the timings. The counts are identical to the digit across four separate runs; the wall-clock medians moved by up to 40% between runs on this host, and the counting allocator adds four relaxed atomic RMWs per heap operation, so the timing ratios are an **upper bound** on the speed-up.

**Why a defect and not a preference.** The store documents itself as *"not built for scale"* two lines above the sentence under review, and nobody is entitled to be surprised that it is slow. That is not what is claimed here. What is claimed is narrower and is exactly what the doc asserts: the snapshot is priced as cheap, and `limit` is priced as a reduction, and neither is true — work is done and then thrown away. This composes with AE-1 multiplicatively, because the discarded tail pays `t + 2` per event: the 10,000-event row at 64 tags is the product.

**Semver.** None. `MemoryEventStore` is live at `0.2.0-alpha.1` but the change is behind the trait, alters no signature, and is asserted output-preserving over 160 query/option combinations on a deliberately non-uniform store.

**Remediation, subordinate.** `.take(limit)` belongs after the bounds filters and before `.cloned()`, below both filters so it cannot limit the *scanned* set (the shape ES-14 rejects) and above the clone so it does not copy a tail it is about to drop; `rev()` must stay ahead of `take()` in the backwards branch for the same frozen reason. `experiments/event-clone-allocations/src/readpath.rs` is one plausible spelling of that, and the experiment's own caveat applies: 160 equal-output combinations on one mixed store is evidence the change is behaviour-preserving on that grid, not a proof, and the conformance suite is where the real bar sits — which that crate cannot run, because it may not modify core.

**Routing.** Prior art: `references/evaluation/review-correctness.md:582-600` records this as **C13** with the same fix, so this is a rediscovery with a number attached rather than a new finding. It is a backlog row against `happenstance-core` with the measurement attached; the decision it does *not* take is whether the reference store's documented "not built for scale" makes read-path cost out of scope, which is the RUNBOOK's to answer if anyone wants to close it that way.

---

**NOTES:**

- **Thesis: upheld, and sharpened in one place.** The theme's claim — that every published performance figure was measured against a shape the shipped code does not have — holds on both halves. It is stronger than filed on the SQL side: the chain does not merely lack a measurement, it **loses** to the aggregate whose measurement justified it in 9 of 9 two-tag cells (1.54x–1.86x).
- **A measurement falsified one of my own findings and I have said so in the entry.** I-2's proposed remedy (bind `position > ?` into the seed arm) is refuted: +2.0% at 10^6 events, no consistent sign across nine cells, and `EXPLAIN QUERY PLAN` shows the two plans identical line for line because the seed is the probe side, not the driving table. The *fact* I-2 reports (no boundary reaches SQL) is confirmed off a trace callback on the real store. `chain-bounded-all-arms` (3,126x) is named as the shape that would make the crate's sentence true and explicitly **not** recommended on two-tag warm-cache evidence.
- **One new entry, CN-1**, not in the finding intake: the most-selective-first ordering costs the shipped chain 38x–44x, measured twice on two fresh stores. It came out of MEASUREMENT 2's "one thing nobody asked for". I considered merging it into I-1 — same root cause, same ADR section — and did not, because the sites, the wrong outcomes and the remediation owners differ, and merging would bury a measured 40x inside a "cost unknown" entry.
- **No merges taken.** AE-1/AE-2 and AE-1/H2 were each assessed for merge (AE-2 is AE-1's arithmetic one file over; H2 pays AE-1's cost per matched rather than per returned event) and kept separate: different clauses, different maturity markers, different semver, different owners. The composition is stated inside each entry instead.
- **Nothing dropped.** All seven assigned findings survived verification at `56ef6c5`; every `path:line` in this section was re-read at the pinned tree.
- **Precision corrections made against the briefing text.** (1) `query_sql` is a **private** module (`crates/happenstance-sqlite/src/lib.rs:89`), so `query_sql.rs:34-54` is *not* rendered public documentation — only `event_store.rs:91-96` (`pub mod event_store`) and ES-27's prose reach a stranger. The experiment README's "public module documentation" covers both files and overstates for one. (2) I-1's original "never conformance-tested" is true of the *experiment* crate only; the chain is covered by `crates/happenstance-sqlite/tests/conformance.rs`, which is why every entry here is classed evidential rather than correctness.
- **A number I derived rather than quoted, flagged as such in I-1.** ES-27's frozen prose publishes "roughly 200x" for the two-tag penalty; the like-named cells of one interleaved run give ~2,300x for the shipped chain at 50,000 events (25,204 µs against 11 µs). That is arithmetic across two scenarios of one run, not a claim the experiment makes, and it is routed as a figure needing re-derivation rather than as a settled one.
- **Caveats honoured throughout.** Absolutes are warm-cache, one host; the 50,000-event calibration did not reproduce ADR-0022 §1's 42,399 µs (14,085 µs, 3.0x less), so ratios carry and absolutes do not. Two tags, one event type, uniformly spread selective tag. The 257.7 ms `read_for` figure is a transcription of a crate-private function with no public seam. The 97.2 s replay and the 60.1 ms statement bracket rather than subtract. Allocation counts, not resident memory; the counting allocator makes timing ratios an upper bound. The anomalous 500,000-event `rejected-2tag-midlog` `APPEND` row is not cited anywhere in this section.
- **For another section's author:** the same clone experiment corrected **H3**'s reasoned layout figures. Measured on `x86_64-pc-windows-msvc` at the pinned toolchain, `Cow<'static, str>` is **24 bytes, not 32** — `String`'s capacity field carries the discriminant — so `Tag` and `EventType` are 24, `Tags` 16, `Event` 104, `SequencedEvent` 144, `QueryItem` 32. A `tests/layout_budget.rs` written from H3's numbers would have failed on the first `cargo test`. Also for whoever owns X-1: its rows share `results/selectivity.md` with I-5 and confirm at 400 items × 128 tags = 51,600 parameters against SQLite's 32,766.
- **No decision taken.** Three entries (I-1, CN-1, I-2) route to one superseding record for ADR-0022 §8, owned by whoever holds `.kb/decisions/0022`; ADR-0022 is accepted and immutable, so §8's requirement cannot be edited and §16's unreachable falsifier cannot be repointed by hand. AE-1 routes to the existing `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` and adds one obligation to it (state the tag regime) without settling who takes the measurement. No ADR text is written here and no clause is amended.

---

## Every mandatory check in this workspace asserts a property one level stronger than the mechanism it runs

This review's own baseline is one sentence — *`cargo xtask ci` is green at `56ef6c5`, all 33 steps, no OPTIONAL step skipped* — and every other judgement in this document is priced against it. The entries below measure what that sentence excludes. In each case the check runs, the check passes, and the check's own documentation states a property the check's mechanism cannot observe; the statement is written in the module doc, the help text or the constitution atom, which is where the reader stops. The flagship is `proof.rs`'s anti-vacuity apparatus: an assertion over `cargo test -- --list` cannot see `#[ignore]`, because libtest's discovery output is byte-identical with and without it, and seven places in the gate — one of them a compiled constitution atom above `CLAUDE.md` on the precedence ladder — say it can. Nothing here is semver-bound, so nothing here needs the `0.2.0` window; a gate repair costs the same the day after. It ranks where it does for a different reason. `0.2.0` is when the gate stops being a private discipline and becomes the evidence a third-party adapter author cites when deciding whether the 90-rule conformance suite means anything, and one item — Q-03 — has a deadline of its own that is not `0.2.0` at all: it must land before `ProjectionStore` is declared `[FROZEN]`, because a freeze is the one thing this repository treats as permanent.

The measurement backing three of these entries is `experiments/gate-vacuity/`, built for this review against a detached worktree of `56ef6c5`. It is untracked at the pinned commit, is not a crate, and `run.sh` is not and must not become a gate step (CF-34 `[PROVISIONAL]`). Five controls ran before any count, and all five fired — including the RED control that renames one named test and takes the same gate to exit 1 at exactly the step whose job that is.

### S-1 — `#[ignore]` silences every named proof test, and seven places in the gate say it cannot

**Clause.** CF-1 `[FROZEN]` (`spec/SPECIFICATION.md:7566-7571`) and CF-3 `[FROZEN]` (`:7603-7606`). Both name a meta-test in `crates/happenstance-testkit/tests/mutation_coverage.rs` as their rule — `every_rule_has_a_mutant` and `mutants_fail_exactly_their_declared_rules` respectively. No clause governs the `proof-artefact` step itself; the governing text is the constitution atom RS-81-4.

**Examined.** `xtask/src/proof.rs:24-31`, `:217`, `:1726`, `:1964`, `:2050`, `:2408`; `xtask/src/main.rs:398-404`; `standards/rust/81-checks-that-cannot-be-types.md:283-323`; `Cargo.toml:168`; `Cargo.lock:1985-1986`; `experiments/gate-vacuity/results/gate-vacuity.md` and `results/raw/`.

**Found.** The whole argument, at `xtask/src/proof.rs:29-30`:

> ```
> //! So the names are asserted, out of `--list`, before the tests run. That is the
> //! same argument `package.rs` makes about `cargo package --list`
> ```

The mechanism, at `xtask/src/proof.rs:1964`:

```rust
        .filter_map(|line| line.trim().strip_suffix(": test"))
```

`results/raw/list-diff.txt` is empty: libtest's discovery output for `projection_harness_parity` is byte-identical with and without an `#[ignore]` on `no_harness_lists_a_rule_by_hand`. An assertion over that listing cannot observe the attribute.

Measured, with `#[ignore = "…"]` applied to all **31 names across 9 targets** that `ARTEFACTS` holds: `cargo xtask ci` exits **0** and runs all **33** steps. The `proof-artefact` step's own log, from `results/raw/ignore-all-ignore-reason-ci.txt:7078-7086`:

```
happenstance-testkit/projection_harness_parity: 2 named tests present
running 2 tests
test projection_harness_parity::each_harness_invokes_the_suite_exactly_once ... ignored, measured by experiments/gate-vacuity
test projection_harness_parity::no_harness_lists_a_rule_by_hand ... ignored, measured by experiments/gate-vacuity
test result: ok. 0 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
```

**Four of the nine proof artefacts ran zero tests** while the step printed *"N named tests present"* about each and exited 0 — among them `mutation_coverage` (8 named, `0 passed; 8 ignored`, and with it CF-1's and CF-3's instruments), `projection_mutation_coverage` (7 named, 0 passed), `projection_harness_parity`, and `course-subscriptions/ui`, the `trybuild` compile-fail pair whose entire value is that it can fail. Taken one name at a time under the exact command `REQUIRED` runs, **31 of 31 exit 0**; no name is an exception.

Bare `#[ignore]` is refused — but not by anything the gate documents. It is `clippy::ignore_without_reason`, a *pedantic* lint enabled at `Cargo.toml:168` and promoted by `-D warnings`, whose own diagnostic ends `= help: add a reason with = ".."`. **The spelling that survives is the one clippy tells the author to reach for, in the same keystroke.**

The wasm32 half is not a second line of defence. `xtask/src/main.rs:402-404` states the division explicitly —

> ```
> // Its stated limit: it cannot say the rules *passed*, and it cannot see
> // an `#[ignore]` on a macro-generated test. Both need the runner, and
> // both are the step below's.
> ```

— and the step below, `cargo xtask wasm-conformance` over an ignored `wasm_tests::the_probe_is_not_vacuous`, reports `81 tests listed, 16 named`, then `80 passed; 0 failed; 1 ignored`, `EXIT=0`.

The claim also sits in the constitution, at `standards/rust/81-checks-that-cannot-be-types.md:321-323`:

> **Rejects.** A proof artefact truncated to its `#![cfg(…)]` attributes, or whose meta-tests have been renamed or marked `#[ignore]`.

RS-81-4's compiled example (`:283-303`) models exactly two listings — the full one and `""` — and there is no third. The rename half of that `Rejects:` line is true and the experiment's RED control demonstrates it; the `#[ignore]` half is not, and within an atom the compiled example beats the prose.

**Why a defect.** This is not a gap in coverage, which would be a preference about how much to test. It is a stated mechanism that does not hold, in the file whose subject is stated mechanisms that do not hold — `proof.rs` exists because *"a step that a deletion fails and an emptying passes is checking the filename"* (`:26-27`), and it reproduces that shape one level up. The wrong outcome is named and dated: the change that would have failed a named guard is the change that can silence it, in the same commit, with an attribute clippy suggests. Nobody finds out at review, because the diff shows a green gate; they find out when a mutant stops discriminating and CF-1's instrument has not run for some number of commits nobody can bound.

**Semver.** None. `xtask` is not published and the fix touches no public item; `0.2.0` is not the deadline. The deadline is the first third-party adapter author who cites this gate.

**Remediation.** Subordinate, and one correction to the obvious route: `cargo test -- --list --ignored` is unavailable, because the locked `wasm-bindgen-test 0.3.76` (`Cargo.lock:1985-1986`) offers `--include-ignored` and no run-only-ignored mode, so the host and wasm arms could not share one mechanism. What is left is to capture the run's own output rather than discard it and compare the reported `passed` count against `tests.len()`, or to pass `--exact` filters per name. RS-81-4's compiled example would need a third case for whatever lands.

**Routing.** Two decisions, one owner each. (1) Whether `proof.rs`'s assertion moves from discovery to execution is the `xtask` gate's own; it belongs to whoever owns the phase that wrote `ARTEFACTS`, and lands with a control in `experiments/gate-vacuity/` that goes red. (2) RS-81-4's `Rejects:` line is a constitution atom and cannot be corrected by a line edit to its prose while its compiled example says something narrower — that is a change to the atom, and `cargo xtask lint-constitution` plus `cargo test -p xtask --doc` are what hold it. Neither is taken here.

### S-5 — one prose word in a `Rule:` line switches off that clause's rule-name check, for nineteen `[FROZEN]` clauses

**Clause.** CF-38 `[FROZEN]` (`spec/SPECIFICATION.md:8813-8817`), whose first stated failure condition is *"a clause naming a rule that does not exist"*. The clauses the guard silences carry their own markers: **19 `[FROZEN]`**, 6 `[PROVISIONAL]`, 3 `[DEFERRED]`.

**Examined.** `xtask/src/spec_trace.rs:698-701`, `:1628-1634`, `:1750-1755`; `experiments/gate-vacuity/results/spec-trace-guard.md` and `results/raw/`.

**Found.** Check 4 abstains before it resolves anything (`xtask/src/spec_trace.rs:698-701`):

```rust
    for c in &clauses {
        if c.schedules_new || !has_suite(&c.id) {
            continue;
        }
```

and `schedules_new` is a substring test over the clause's `Rule:` **text** (`:1628-1634`):

```rust
    let elsewhere =
        text.contains("unit test") || text.contains("compile test") || text.contains("meta-test");
    let schedules_new = text.contains("(new)")
        || text.contains('†')
        || text.trim_start().starts_with("new ")
        || text.contains(" new `")
        || elsewhere;
```

Measured: dropping all seven terms from `schedules_new` only — leaving `elsewhere` computing exactly what it computed before, so §7.2's rendering does not move and the delta is attributable to check 4 alone — takes `cargo xtask spec-trace` from *"traceability: no problems found"* to **exit 1, 45 traceability problems**, across **28 clauses**: ES 10, VT 11, PS 6, WF 1. Nineteen of the 28 are `[FROZEN]`.

The trigger is family-agnostic. VT-13's `Rule:` line is what the guard costs at its worst:

```
`Rule:` unit test `position_next_signals_overflow`; `read_from_is_inclusive`,
`condition_after_ignores_events_at_the_boundary`; new
`read_from_a_gap_position` (ES-9's name for it)
```

Two of those four names — `read_from_is_inclusive` and `condition_after_ignores_events_at_the_boundary` — are live suite rules that resolve fine once the guard is off. The words *"unit test"* switch off the check for all four.

Three of the seven terms are **inert at this commit**: `†`, `meta-test` and the leading `new ` each fire on zero clauses. `grep -c "Rule:.*†" spec/SPECIFICATION.md` is 0, against 59 daggers in the file, all of them inside §7's generated table or the prose describing the convention.

**Why a defect.** A clause can name a conformance rule that has never existed and CF-38's own mandatory checker reports green, because the sentence explaining that the rule is unwritten is also the switch that stops it looking. The state is unrecoverable through the gate: the clause cannot move out of the guarded state by this mechanism, because the mechanism never runs on it again. The wrong outcome falls on whoever reads a green `spec-trace` as evidence that a `[FROZEN]` clause's named rule exists — which is precisely what §7.2 is published for.

This corroborates a recorded, accepted open question: `.kb/open-questions/no-ps-rule-name-is-resolved.md` (`kb-open-question-no-ps-rule-name-resolved-001`). Two corrections it is owed, both measured here. Its stated mechanism — *"`schedules_new` is set by a bare dagger in the clause's `Rule` line"* — does not hold at `56ef6c5`; the term actually doing the work on the `PS` family is `` new ` ``, and the dagger does no work at all. And its **sub-question 2** — *"If the dagger is retired from the guard, how many of the seventeen daggered `PS` clauses newly fail?"* — is answered: **zero**, `results/raw/spec-trace-drop-dagger.txt`, exit 0, no problems. Sub-question 1 therefore has a measured answer too: the dagger is doing no work `has_suite` cannot.

**Semver.** None. Not a `0.2.0` decision.

**Remediation.** Subordinate, and it is two changes with different prices. Retiring the three inert terms is free by construction — the measurement shows they fire on nothing, so the gate stays green — and is a repair under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`. Retiring the four live terms cannot land green: 45 reports arrive at once, and four of them are not rule names at all (`trait_variant` on ES-2, `compile_fail` on VT-32 and WF-12, `spec_trace` on WF-12), which `backticked_idents` would need to stop harvesting first. The honest reading of 45 is 41 names a reader would expect to resolve plus 4 parser false positives.

**Routing.** The open question's own three sub-questions, now with sub-questions 1 and 2 measured. It is `kb-open-question-no-ps-rule-name-resolved-001`'s to close and it names its own forcing event — *"the next `PS` clause that cites a rule name nobody has written"*. Whether the dagger convention is superseded by the maturity markers is its sub-question 3 and is an ADR's, not this document's.

### Q-03 + Q-04 — four checks named by five `[FROZEN]` clauses are absent from `spec_trace.rs`, and one of them is the port-freeze bar

**Clause.** CF-25 `[FROZEN]` (`spec/SPECIFICATION.md:8451-8458`), CF-26 `[FROZEN]` (`:8468-8471`), CF-36 `[FROZEN]`, CF-37 `[FROZEN]` (`:8805-8806`), and CF-38 `[FROZEN]` (`:8813-8817`), whose fourth stated failure condition is *"a case naming no clause"*.

**Examined.** `xtask/src/spec_trace.rs:1-9`, `:612-626`, and the whole file by grep; `spec/SPECIFICATION.md:8451-8471`, `:8805-8817`, `:9550-9581`; `spec/E2E-CASES.md`; `experiments/gate-vacuity/cases.py` and `results/raw/e2e-cases.txt`.

**Found.** Four claims, one checker, and none of the four is implemented.

CF-25 gates every port freeze in the document, and asserts its own instrument (`spec/SPECIFICATION.md:8457-8458`):

> Rule: `cargo xtask spec-trace` (CF-38), which reads the portfolio table and the maturity markers and fails when a `[FROZEN]` port clause has an axis with no far-end row and no named risk acceptance.

CF-26 names the same mechanism against §6.5's `Far end exists` column. Over `xtask/src/spec_trace.rs` at `56ef6c5`:

```console
$ grep -ci "portfolio" xtask/src/spec_trace.rs   → 0
$ grep -ci "far end"   xtask/src/spec_trace.rs   → 0
$ grep -ci "axis"      xtask/src/spec_trace.rs   → 0
```

CF-36 claims the checker cross-references each E2E case's level marker; `grep -c "Level" xtask/src/spec_trace.rs` returns **0**, and no other check performs the comparison under another name.

CF-37 is a flat obligation on a document — *"Every E2E case MUST name the clause or clauses it exercises."* — whose `Rule:` line is `cargo xtask spec-trace` (CF-38). Over the pinned documents: `spec/E2E-CASES.md` carries **58** `### E2E-nn` headings and **0** `Clauses:` fields of any kind; **54 of 58** case bodies contain no clause identifier anywhere. The checker reads `E2E-CASES.md` in one direction only — `collect_cases` (`xtask/src/spec_trace.rs:626`) builds a set of case identifiers so that check 5 can verify a clause's `Cases:` line resolves. There is no reverse traversal, so CF-38's fourth condition has no implementation. In the direction the checker *could* run, the result is clean: **58 of 58 cases are claimed by at least one clause's `Cases:` line — zero orphans**, once `field_line`'s continuation-line joining is reproduced.

**Why a defect.** A missing check whose absence nothing can report is worse than an unwritten one, because the gate is green and reads as evidence — `spec_trace.rs`'s own module doc says so at `:8-9`: *"a specification whose cross-references have quietly rotted is worse than one that never made them — it reads as though it is backed by tests."* Each limb has its own named wrong outcome.

CF-25/CF-26 is the one with a deadline, and it is not `0.2.0`. `ProjectionStore` is the port queued for freeze — PS-2 gates it, `projection-store-freeze` (HS-P0010) owns it, and PS-12, PS-23, PS-24 and PS-38 are `[PROVISIONAL]` on axes whose far ends are unbuilt. Whoever runs that freeze gets a green `spec-trace` and a clause telling them it means the portfolio bar was checked. It was not checked; it was asserted.

CF-37/CF-38 is the hazard §7.6 names about itself and then leaves unguarded (`spec/SPECIFICATION.md:9569-9571`): *"Single-claim cases are the ones a later edit can orphan without anyone noticing"* — E2E-14, E2E-31, E2E-50, E2E-51 — and the check that would notice is the one CF-38 lists fourth and nobody wrote. The orphan set is empty **today**; it is unguarded **tomorrow**.

CF-36 corroborates an accepted open question, `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md` (`kb-open-question-cf-36-unperformed-cross-reference-001`), which names its own second forcing event as *"phase 12, where first publish makes the specification a promise rather than a working note"*. That is a result: the atom is dated `2026-08-17` and the gap is unchanged at `56ef6c5`.

**Semver.** None. `spec/` and `xtask/` are not published artifacts. But CF-36's own atom prices its deadline at first publish, and CF-25's is earlier still.

**Remediation.** Subordinate, and cheaper than the shape suggests for two of the four: the specification already carries the inputs. §6.5's portfolio table has the `Far end exists` column CF-26 names, and the clause census `spec_trace.rs` already computes carries the maturity markers, so CF-25/CF-26 is a tenth check over documents already parsed. CF-38's stated five conditions are a **floor** (`MUST fail on`), so adding a check needs no clause edit; rewriting a `[FROZEN]` clause's `Rule:` line to stop claiming an absent instrument does, at the price `CLAUDE.md` sets — a superseding ADR whose content would be *"the workspace decided not to machine-check its own freeze bar"*.

**Routing.** Three owners, and none of the three decisions is taken here. (1) Whether the portfolio check is built before `ProjectionStore` freezes, or the freeze names the axis it accepts risk on and the ADR that accepts it — CF-25's own two permitted routes — belongs to `projection-store-freeze` (HS-P0010). (2) Whether CF-38's fourth condition reads "a case body naming no clause" (54 of 58 fail) or "a case no clause claims" (0 fail) is a reading of a `[FROZEN]` clause and settles which of CF-37 or the checker is wrong; it belongs with CF-37/CF-38, not with whoever implements. (3) CF-36's two routes — implement, or supersede via `kb-playbook-repair-frozen-clause-001` — are already enumerated in its accepted open question and stay there.

### S-2 — five statements of which file-reading lints run, no two agreeing, and the story-grain gate drops the one that exists for the change stories make

**Clause.** No SPECIFICATION clause governs the composition of the lint family. Four of its members are named by clauses — CF-33, CF-6, CF-29, CF-32 — but which entry point runs which is `xtask`'s own arrangement, and `xtask/src/main.rs:29-34` says so, naming a seventh lint deliberately excluded from the group *"because it names no clause (ADR-0016 §14)"*.

**Examined.** `xtask/src/affected.rs:26-36` and `:115-139`; `xtask/src/main.rs:29-52`, `:543-689`, `:1094`, `:1114-1120`, `:1201-1214`; `xtask/src/lints.rs:1066`; `.redkiln/config.yaml:36`, `:40`, `:48`.

**Found.** `affected::run`'s file-reading block is unconditional and complete in nine lines (`xtask/src/affected.rs:118-124`, `:130`, `:138`):

```rust
    crate::spec_trace::retired_rules()?;
    crate::lints::no_clock()?;
    crate::lints::no_position_literals()?;
    crate::lints::changelog_names_every_rule()?;
    crate::lints::testkit_version()?;
    crate::lints::core_alloc_features()?;
    crate::spec_trace::run(crate::spec_trace::Mode::Check)?;
```

`grep -rn "stated_rule_counts" xtask/src/` returns four hits and none is in `affected.rs`. The module doc immediately above claims otherwise (`xtask/src/affected.rs:30-31`):

> ```
> //! [`crate::lints`] and [`crate::spec_trace`] run on every invocation, whatever
> //! the diff touched.
> ```

and the help text for that exact command (`xtask/src/main.rs:1094`):

> ```
> println!("         The story-grain gate: the six file-reading lints and spec-trace,");
> ```

Each entry point drops something different, and only `REQUIRED` holds them all:

| entry point | omits |
| --- | --- |
| `REQUIRED` (`main.rs:516-689` plus the narrative and pages steps) | — |
| `lint_steps()` (`main.rs:1201-1214`) → `cargo xtask lints` | `lint-core-alloc-features`, `spec-trace` |
| `affected::run` (`affected.rs:118-138`) → `verify.affected_gate` | **`lint-rule-counts`**, `lint-constitution` |
| `print_help()`'s task list (`main.rs:1114-1120`) | names six of the eleven |
| `.redkiln/config.yaml:36` | says *"the five file-reading lints"* |

**Why a defect.** The dropped member is not arbitrary. `stated_rule_counts` runs in the opposite direction to every other step — it holds four *documents* to the code — and `xtask/src/lints.rs:20-24` records why it exists:

> ```
> //! [`stated_rule_counts`] runs the other way, and it exists because the failure
> //! it catches happened three times before anything could see it — a document
> //! shipped to crates.io describing a suite two thirds smaller than the one in
> //! the package beside it.
> ```

`.redkiln/config.yaml:40` wires `cargo xtask affected --base {{base}}` as `verify.affected_gate`, which runs at every story advance seam. So a story that lands a conformance rule — the exact change this lint exists for — clears its own grain with the README, both `lib.rs` front pages and `happenstance-core/Cargo.toml`'s feature comment still stating the old count. The person who breaks is the crates.io reader of `happenstance-testkit`'s front page, and the failure has already happened three times. `.redkiln/config.yaml:48` compounds it: `reachability_static` is `cargo xtask lints && cargo xtask spec-trace`, which routes through the *other* incomplete list.

**Semver.** None. `xtask` is not published; the cost is borne by `crates.io` copy, which is a `0.2.0` artefact rather than a `0.2.0` API.

**Remediation.** Subordinate. The structural fix is the one this repository already uses for the wasm family: one named list, and the entry points select from it rather than each transcribing it. That collapses the five prose counts into one place a `grep` can check.

**Routing.** Two questions, and the second is a decision. Whether `affected::run` should run `lint-rule-counts` is settled by the lint's own stated purpose and needs no owner beyond the `xtask` gate. Whether `lint-constitution` belongs on the story grain is not — `affected.rs:126-129` argues the narrative tree onto the list and the constitution off it deliberately, so reversing that is a decision with a written rationale to supersede, and it belongs to whoever owns the constitution's enforcement rather than to whoever notices the asymmetry.

### S-3 — `constitution.rs` asserts a `RUSTDOCFLAGS` behaviour this repository has now measured false twice, in the file describing how the constitution is enforced

**Clause.** No clause governs. The governing text is the constitution atom **RS-01-4** (`standards/rust/01-standard-of-evidence.md:180-188`), which sits above `CLAUDE.md` on the precedence ladder and already denies the claim.

**Examined.** `xtask/src/constitution.rs:31-36`; `xtask/src/narrative.rs:42-54`; `xtask/src/main.rs:732-745`; `standards/rust/01-standard-of-evidence.md:180-188`; `experiments/gate-vacuity/results/constitution-fence.md`.

**Found.** `xtask/src/constitution.rs:31-36`:

> ```
> //! `RUSTDOCFLAGS=-D warnings` recovers rustc's *default-on* lints inside a
> //! doctest — a probe confirmed `non_snake_case` fails the build under it — but
> //! not the workspace's `[lints]` table and not clippy, which is where
> //! `unwrap_used` lives. So the recovery is partial
> ```

`xtask/src/narrative.rs:42-53`, recording a re-measurement on the pinned 1.97.1:

> ```
> //! * **`RUSTDOCFLAGS=-D warnings` reaches nothing inside a narrative fence, and
> //!   that is measured rather than cited.** … a fence violating `non_snake_case`, a warn-by-default
> //!   rustc lint confirmed to fire on the same snippet under plain `rustc`,
> //!   compiled and ran with **no diagnostic and exit 0** — with the variable
> //!   set, with it removed
> ```

RS-01-4 is unambiguous (`standards/rust/01-standard-of-evidence.md:183-186`): *"`RUSTDOCFLAGS=-D warnings` gates the doc build — intra-doc links, `missing_docs` — not anything lexically inside a fence."*

`narrative.rs`'s measurement was taken against the narrative tree. Re-measured here against the **constitution** corpus — the tree `constitution.rs`'s own sentence is about — with two `#`-hidden, *used* lines inserted after the first fence's own `fn main() {` in `standards/rust/00-prime-directives.md`, and with both controls firing: plain `rustc --edition 2024` on the same snippet emits `warning: variable notSnakeCase should have a snake case name … #[warn(non_snake_case)] … on by default`; a type error at the identical insertion point takes the docs step to **exit 101**, naming `00-prime-directives.md - constitution::prime_directives (line 32)`. The arm itself: `RUSTDOCFLAGS="-D warnings" cargo test --locked -p xtask --doc` is **exit 0, 62 passed**, and the whole `cargo xtask ci` is **exit 0, 33 steps**.

**Why a defect.** This is not two views of an open question. RS-01-4 is a constitution atom, `constitution.rs` is a comment in a gate file, and the atom wins on the precedence ladder — the falsified sentence is the one that survives, in the file describing the corpus's enforcement, and `xtask/src/main.rs:737-739` leans on it to justify the step: *"`RUSTDOCFLAGS` is the only way `-D warnings` reaches rustdoc — clippy does not lint doctests at all, so this is the whole of what the constitution's examples are held to."* What they are actually held to is type-checking plus `lint_constitution`'s `FORBIDDEN_IN_FENCE` grep for `.unwrap()` and `.expect(`. Every rustc warn-by-default lint — `non_snake_case`, `unused_mut`, `deprecated`, `unused_imports` — is unenforced inside every fence in `standards/rust/`. The person who breaks is the next author of an atom, who writes an example believing the compiler is holding it to more than it is, in a corpus whose whole enforcement model is that the compiled example beats the prose.

**Semver.** None.

**Remediation.** Subordinate, and it is a documentation correction rather than a mechanism change: the sentence is wrong, RS-01-4 is right, and `main.rs`'s step comment inherits the correction. Whether the unenforced lints should be recovered by some other means is a separate and larger question this entry does not open.

**Routing.** `narrative.rs:52-54` already names where the primary measurement lives — its project's `_limits-evidence.md`, three transcripts with their exact commands and toolchain — and this entry adds a fourth taken against the constitution corpus. The correction belongs to whoever owns the `xtask` gate's documentation; RS-01-4 needs no change and should not be edited to accommodate the correction.

### F1-04 — ES-13's regression pin is a `compile_fail` fence in an integration test target, and cargo has never handed it to a compiler

**Clause.** ES-13 `[FROZEN]` — the clause that freezes `read` on `&Query` and names taking `Query` by value as the wrong fix.

**Examined.** `crates/happenstance-core/tests/frozen_signatures.rs:25-32`, `:221-234`; `RUNBOOK.md:3628-3643`; `experiments/gate-vacuity/results/raw/baseline-ci.txt`.

**Found.** `crates/happenstance-core/tests/frozen_signatures.rs:221` opens a bare fence with no error code, closing at `:229`, and its carrier is `:230-233`:

```rust
#[expect(
    dead_code,
    reason = "the doctest above is the artefact; this item exists to carry it"
)]
```

The file's own module doc still presents it as live (`:29-32`):

> ```
> //! What does not compile is a local, and
> //! [`escaping_cases_need_the_query_to_outlive_the_stream`] documents the four
> //! diagnostics that arrangement produces
> ```

Read out of the green control's own log — `results/raw/baseline-ci.txt`, the whole `cargo test --locked --workspace --all-features`: `grep -n escaping_cases_need_the_query_to_outlive_the_stream` returns nothing. `Doc-tests happenstance_core` runs **33 tests, every one from `src/`**, and contains exactly **one** `- compile fail` line: `event.rs - event::EventType::from_static (line 95)`. The target itself builds and runs (`Running tests\frozen_signatures.rs`); its doctest does not exist. `grep -n compile_fail crates/happenstance-core/tests/frozen_signatures.rs` returns one line, `221`, and it carries no code.

**Why a defect.** This is prior art with two unrecorded halves. `RUNBOOK.md:3628-3643` records that the doctest never runs, and records it deliberately — *"It is recorded here rather than silently fixed… an instrument that cannot fail is a finding, and a finding belongs to whoever owns the clause."* What it does not record is that **the site itself carries no marker**: a reader of `frozen_signatures.rs` — the phase-4 proof artefact for a `[FROZEN]` clause, in a crate publishing at `0.2.0` — is told by the module doc and by the `#[expect]` reason that the pin is live, and only a reader who has also read `RUNBOOK.md:3628` knows otherwise. Nor does it record that relocating the fence into `src/` would still not be enough on its own: a bare `compile_fail` with no error code passes when the arrangement stops failing for the reason it was written about. The wrong outcome falls on whoever next revisits ES-13 — a `'static` shape on `Query`, a shift in RPITIT capture rules, or the by-value change the clause's `Rejects:` line forbids — and nothing goes red. They find out when an adapter author reports the diagnostic the file claims to pin.

**Semver.** None for the fix. The clause it pins is `[FROZEN]`, so the thing it guards is exactly what `0.2.0` cannot change cheaply — the pin's absence is the cost, not its repair.

**Remediation.** Subordinate, and one correction to the obvious route. Moving the fence onto a `#[doc(hidden)]` item in `crates/happenstance-core/src/store.rs` is the whole of the defect and is not in dispute. Annotating it `compile_fail,E0597` is refused by this repository's own measured evidence: `standards/rust/62-doctests-and-harnesses.md:12` (**RS-62-1**) — *"Pair every `compile_fail` fence with a compiling one, and do not trust its error code"* — records that rustdoc 1.97.1 compares the code, finds no match, and **reports the fence as passing anyway**.

**Routing.** ES-13 is `[FROZEN]`, so its instrument belongs to whoever owns the clause, which is what `RUNBOOK.md:3639-3643` already says and is the precedent it cites (WF-12's instrument moved from a `compile_fail` doctest to a const assertion in `tests/wire.rs` for the adjacent reason). Nothing here decides where the fence lands.

### F1-03 — the two-flavour public API is emitted by a caret-resolved proc macro, and the sentence that reassures you about it is scoped to this workspace

**Clause.** No clause governs the derivation's dependency requirement. ES-2 `[FROZEN]` names `trait_variant` in its reasoning; the constraint that the two flavours exist is ADR-0001 and ADR-0035's, and CLAUDE.md's binding constraint 4 rests on the blanket impl the macro emits.

**Examined.** `Cargo.toml:130`; `Cargo.lock:1780-1784`; `crates/happenstance-core/src/store.rs:1010-1030`, `:1044-1050`; `xtask/src/main.rs` (31 `--locked` occurrences); `.github/workflows/ci.yml:13-14`, `:31`, `:140`, `:307`, `:345`, `:399`.

**Found.** `Cargo.toml:130` is a caret requirement:

```toml
trait-variant = "0.1.3"
```

`SendEventStore`, `SendProjectionStore` and the blanket impl are not written in this repository; they are emitted at compile time by that macro. `crates/happenstance-core/src/store.rs:1017-1021` states the exposure precisely —

> ```
> /// Parsed rather than pinned in the manifest: `trait-variant = "0.1.3"` is a
> /// caret requirement, so `0.1.4` would resolve without the manifest changing.
> ```

— and then, twenty-seven lines later, at `:1047-1049`, mitigates only the half that is inside this tree:

> ```
> /// repository can observe the expansion, so the version is asserted instead —
> /// the gate builds `--locked`, so the resolved version cannot move without
> /// someone changing it deliberately
> ```

That sentence carries no scope qualifier. It is true of this workspace's gate and false of the published artifact: `Cargo.lock` does not travel to a consumer, all 31 `--locked` invocations in `xtask/src/main.rs` and every CI job pin `0.1.3`, and `.github/workflows/ci.yml` has a weekly `schedule:` trigger (`:13-14`) that drives `advisories` and nothing else. No job in this repository ever resolves this requirement freshly.

**Why a defect.** The theme, one level down from the gate: an assertion written where the reader stops, whose mechanism holds for a narrower population than the sentence names. Every existing guard on this derivation is `#[cfg(test)]`, so all of them run against the locked version and none can observe a different one. The wrong outcome falls on a consumer, in their own crate, on a day nobody here pushed anything: `trait-variant 0.1.4` publishes, their fresh resolve of `happenstance-core 0.2.0` takes it, and if `mk_blanket_impl` or `transform_item` changed at all their build fails inside a macro they did not write — or, worse, compiles with a different flavour relationship than the one this workspace ever tested. They find out at `cargo build`, and the maintainer finds out from the issue.

**Semver.** None to the fix. The exposure itself is a `0.2.0` property: three of these crates are already on `crates.io` at `0.2.0-alpha.1`, so the first consumer whose resolve can float is already possible today.

**Remediation.** Subordinate, and it has two halves at different addresses. A derivation contract compiled into `happenstance-core`'s lib is the half that travels to the consumer and fails at their build rather than silently succeeding differently. A `floating-deps` job on the `schedule:` trigger that already exists is the half that makes the maintainer find out first — noting that `rm Cargo.lock` is the wrong verb for it. Neither half closes the residual: a shape change in a version nobody has published yet cannot be tested by anything.

**Routing.** Two decisions. Whether the caret requirement narrows to `=0.1.3` is a dependency-policy decision with a real cost — it forces a manual bump for every patch — and belongs with ADR-0001/ADR-0035's owner rather than with whoever notices. Whether a floating-resolve job joins CI is the gate's own; it is non-blocking by construction and green today. `store.rs:1047-1049`'s missing scope qualifier is a documentation correction and is the cheapest of the three.

### K1 + K2 — WF-10 says *every value type*, and the wire layer's instruments are narrower than that in two independent directions

**Clause.** WF-10 `[FROZEN]` (`spec/SPECIFICATION.md:2298-2310`). VT-14 `[FROZEN]` (`:1153`) is what makes `EventType` and `Tag` value types with constructor invariants in the first place.

**Examined.** `spec/SPECIFICATION.md:2299-2310`; `crates/happenstance-core/src/event.rs:320-326`, `:715-720`, `:739-751`, `:753-763`; `crates/happenstance-core/src/tag.rs:525-530`; `crates/happenstance-core/tests/wire.rs:176-178`, `:1179`, `:1260`, `:1324`, `:1399`.

**Found.** WF-10's obligation is unqualified (`spec/SPECIFICATION.md:2299-2302`):

> The `Deserialize` implementation of every value type MUST re-run its constructor's validity invariants, so that no wire value can become an in-memory value the constructor would have rejected.

Its own `Rule:` line names four tests (`:2306-2309`) — `wire::decode_rejects_a_non_canonical_tag_set`, `wire::decode_rejects_an_unconstrained_query_item`, `wire::decode_rejects_a_zero_item_query`, `wire::decode_accepts_an_over_capacity_value` — covering `Tags`, `QueryItem` and `Query`. `grep -n "fn decode_" crates/happenstance-core/tests/wire.rs` returns exactly those four.

**Direction one — no instrument for `EventType` or `Tag`.** The production code is correct today (`crates/happenstance-core/src/event.rs:715-720`):

```rust
    impl<'de> Deserialize<'de> for EventType {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let raw = String::deserialize(deserializer)?;
            Self::new(raw).map_err(serde::de::Error::custom)
        }
    }
```

and `crates/happenstance-core/src/tag.rs:525-530` is the same shape. Nothing in `wire.rs` ever feeds either one a malformed string. The proptest generator cannot: `crates/happenstance-core/tests/wire.rs:176-178` is

```rust
        pub(super) fn any_event_type() -> impl Strategy<Value = EventType> {
                .prop_map(|value| EventType::new(value).expect("a valid event type"))
```

so every value it round-trips has already passed `new()`.

**Direction two — nothing ties `Event`'s field set to `EventWire`'s.** `Event`'s private fields (`crates/happenstance-core/src/event.rs:321-326`) and `EventWire`'s (`:739-751`) are two independently declared structs, and the `Serialize` impl builds the mirror with a manual literal (`:753-761`):

```rust
    impl Serialize for Event {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            EventWire {
                event_type: self.event_type().clone(),
                data: self.data().clone(),
                tags: self.tags().clone(),
                metadata: self.metadata().cloned(),
            }
```

That literal is exhaustiveness-checked against `EventWire`, not against `Event`. The same shape holds for `SequencedEventWire`, `EventIdWire`, `GuardWire` and `QueryItemWire`.

**Why a defect.** Both directions have the same signature: the obligation is stated over a set, the instruments cover a subset, and the compiler cannot tell the difference. Direction one's wrong outcome is a contributor who "simplifies" `EventType::deserialize` to `Ok(Self(Cow::Owned(raw)))` on the reasoning that a wire value must already have been validated upstream — clippy, fmt, the round-trip proptests and every test in the workspace stay green, and a peer or a hand-crafted replication message can then put a value carrying a control character or a bidirectional override into the receiver's memory, which is exactly what VT-14's constructor exists to prevent and what WF-10's `Rejects:` line names. Direction two's is a contributor who adds a required field to `Event` for a legitimate domain reason, updates `Event::new` and the accessors because the compiler forces them, and does not update `EventWire` because nothing forces that — the crate compiles, clippy passes, and the round-trip proptests pass **vacuously**, because their generator does not vary a field they do not know about. In both cases the field or the invariant is silently dropped on the wire, and the finder is a peer, at ingest, over durable events.

**Semver.** None. Both fixes are tests plus their gate registration; no public item moves, and `0.2.0` is not the deadline.

**Remediation.** Subordinate, and direction two is more tractable than it first looks. Rust cannot assert field-set equality between two independently declared structs, which is why the finding was filed as needing a process instrument — but it does not need to. It only needs to force whoever adds a field to `Event` to stand at the wire mirror and say in code what happens to it, which `E0027` (a struct pattern with no `..`) and `E0063` (a missing field in an initializer) already do on stable. Direction one is two `wire::decode_rejects_*` tests and their citation on WF-10's `Rule:` line; whether the same obligation on VT-19 and the other `[FROZEN]` clauses imposing it is cited at the same time is part of the same pass.

**Routing.** WF-10 is `[FROZEN]` and its `Rule:` line is the record of what checks it — extending that line is a repair under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, not a clause amendment, since the set of implementations WF-10 admits does not change. Whether the exhaustive-destructuring instrument is worth its ceremony at every wire mirror or only at `Event` is a design call belonging with whoever owns §2.7's wire format, not with this document. `cargo xtask spec-trace`'s check 4 is what would hold the new names honest — subject to S-5.


---

## The suite's self-reports are checked by something weaker than the sentence, and the drift has already happened

`happenstance-testkit` is the crate that decides whether an adapter exists, and ADR-0010 makes it carry its own proof: no rule without a mutant, no pass rate ever quoted, a shape report in place of a ratio. The apparatus is real and most of it works. What has drifted is the layer above it — the sentences that tell a reader *what the apparatus found*. In five places the report is checked by something one level weaker than the claim written over it: a table's row-by-row agreement is asserted while its prose count is not, a `println!` carries an obligation that no assertion and no default `cargo test` can observe, an unfalsifiable predicate stands in for "the scenario completed", and two assertion messages cite a `[FROZEN]` clause other than the one the reachable failure evidences. The audience splits in two and the theme's framing needs correcting on that point: three of these are read by an adapter author on crates.io, docs.rs, or in a red build; the other two live in `tests/mutation_coverage.rs`, which ships in the package but renders on no documentation page, and are read by whoever prices the suite at a phase gate. Nothing here costs a consumer correctness. It costs the ability to believe the suite's own account of itself, which is the thing 0.2.0 asks strangers to take on trust.

---

### L1-3 — the reporting promise CF-18 rests on is unobservable in every repository but this one

**Clause.** **CF-18** — *a rule whose capability requirement is unmet MUST still be emitted as a test that **reports** the skip with the fixture's stated reason* (`spec/SPECIFICATION.md:8029-8032`). **`[FROZEN]`**. **ES-35**'s marker depends on the same mechanism (`spec/SPECIFICATION.md:4215`, **`[PROVISIONAL]`**).

**Examined.** The three surfaces an adapter author actually reads — `crates/happenstance-testkit/README.md:45-49` (the crates.io front page), `crates/happenstance-testkit/src/lib.rs:46-50` (the docs.rs module page), and `crates/happenstance-testkit/src/contract.rs:229-233` and `:759-764` (the rustdoc on `Capability::declined`, which is the item they call). Against the emitter itself at `crates/happenstance-testkit/src/contract.rs:934-960`, the meta-test at `crates/happenstance-testkit/tests/mutation_coverage.rs:3256-3345`, and the gate step at `xtask/src/main.rs:167-190`.

**Found.** `README.md:46-47`:

> `rule needing a capability your fixture declines still runs as a test and prints`
> `your stated reason; it is never silently dropped from the binary`

`lib.rs:48-49`:

> `//! it returns [RuleOutcome::Skipped] and the harness prints the fixture's`
> `//! stated reason.`

`contract.rs:761-763`:

> `/// The reason is not a formality. It is printed on every run for every rule`
> `/// the decision skips, so it lands in the adapter's CI log where both a`
> `/// reviewer and a user of the adapter can read it.`

The mechanism, `contract.rs:956-959`:

```rust
    pub fn report(self, rule: &str) {
        if let Some(line) = self.skip_line(rule) {
            println!("{line}");
        }
    }
```

A rule that skips is a test that **passes**, and libtest discards a passing test's stdout. The same file says so, twenty lines above, at `contract.rs:941-947`:

> `/// Natively, libtest suppresses a *passing* test's stdout unless it is run`
> `/// with --show-output (or --nocapture), which is why the gate passes it.`
> `/// That makes the line reachable by a human; it does not make anyone read`
> `/// it. The machine-checked half of the obligation is the`
> `/// mutation_coverage::capability_skips_are_reported meta-test […] which asserts on`
> `/// [RuleOutcome] *values* rather than on stdout — and not this function.`

The gate passes the flag, and says why, at `xtask/src/main.rs:169-173`. The meta-test says where it does not run, at `mutation_coverage.rs:3266-3268`:

> `// has to be closed there: this file's meta-tests never run in an`
> `// adapter's CI, which is precisely where the fixture declining a MUST`
> `// lives.`

**Why a defect.** Not a disagreement about what is true — `contract.rs:941-947` measured it and states it exactly. It is three surfaces asserting the opposite of what a fourth measured, and the three that are wrong are the only three a stranger reads. CF-18's `Rejects:` paragraph turns entirely on the trade landing *"in the adapter's CI log where a reviewer and a user of the adapter can both see it"* (`spec/SPECIFICATION.md:8042-8044`). In a third-party repository neither half of that is present: the line needs `--show-output`, which exactly one CI in the world passes, and the meta-test that checks the `RuleOutcome` values lives in the testkit's own `tests/` and never runs there. The wrong outcome has a named victim who is not the author: an adapter author declines `REOPEN` and `MID_BATCH_FAULT` — the minimal honest fixture, and legal under CF-18 — sees `N passed; 0 failed` with no `SKIP` line anywhere, and publishes a README saying the crate passes the happenstance conformance suite. The person who breaks is the *user* who chose that adapter on the strength of the claim, and they find out when an acknowledged write is not there after a restart.

The instrument is one command against a fixture already in the tree. `crates/happenstance-sqlite/tests/support/mod.rs:173` declines `MID_BATCH_FAULT`, and two rules `require!` it (`suite.rs:2709`, `suite.rs:2797`), so `cargo test -p happenstance-sqlite --test conformance 2>&1 | grep -c SKIP` yields `0`, and the same command with `-- --show-output` yields `2`. Not run in this pass, which compiles nothing; both preconditions are read from the source.

**Semver.** Correcting the four sentences is **none**, and available at the same price forever. The other half is not: no passing libtest test can emit to a default run, so any mechanism that puts the skip in front of a stranger changes what an adapter's suite *does* — which is a change to the bar, and `spec/SPECIFICATION.md:8703-8706` is explicit that *"the contract's is a promise about types, the testkit's is a promise about the bar."* `happenstance-testkit` is live at `0.2.0-alpha.1`; **0.2.0 is the last cheap moment for the mechanism half only.**

**Remediation, subordinate to the clause and the routing.** The four sentences can be brought into line with `contract.rs:941-947` — a documentation change, which settles nothing — and the machine-checked half can be moved somewhere an adapter's own CI executes. Whether the second is worth doing at all is the decision, not the sentence.

**Routing.** Two routes, and they must not be collapsed. The prose correction is a documentation story under the support initiative. The mechanism question is an ADR through the RUNBOOK's ADR pass and never a line edit, because CF-18 is `[FROZEN]`: **the decision is whether CF-18's reporting obligation is discharged by a mechanism a stranger's default `cargo test` can observe, or whether the clause is narrowed to what libtest permits.** Owner: `.kb/decisions/`, not this document.

---

### L3-02 · L3-04 — both non-vacuity reports state a property their table does not have, and the meta-tests assert the rows rather than the prose

*Merged from two findings: `L3-02` (`MODEL_COVERAGE`) and `L3-04` (`REGISTRY`). One defect in two tables — a doc comment over a `const` table making a claim the table does not support, with a meta-test that asserts row-by-row agreement and nothing about the sentence — and one remediation shape.*

**Clause.** For `REGISTRY`: **CF-2** — *mutants MUST be registered as data […] naming, per mutant, the exact set of rules it fails* (`spec/SPECIFICATION.md:7591-7593`), **`[FROZEN]`**, with **CF-1** (`:7566-7569`, **`[FROZEN]`**) supplying the argument the false sentence discounts. For `MODEL_COVERAGE`: **no clause governs**, and that is itself the finding's corroboration — see below.

**Examined.** `crates/happenstance-testkit/tests/mutation_coverage.rs:2324-2378` (the `MODEL_COVERAGE` heading and its enumeration) against the table at `:2380-2490` and the meta-test at `:3775-3831`; and `mutation_coverage.rs:249-257` (the `REGISTRY` caution) against `:341-375` and `crates/happenstance-testkit/src/suite.rs:605-621`. The generator's own definition at `crates/happenstance-testkit/src/model.rs:152-179`.

**Found — `MODEL_COVERAGE`.** `mutation_coverage.rs:2324-2334`:

> `/// # The twenty-one it does not catch are three shapes, not twenty-one`
> `///`
> `/// Twenty-three rows below are marked [ModelOutcome::Agreed]. Two of those are`
> `/// the conformant controls and *must* be, which leaves **twenty-one misses** […]`
> `/// Every one of the twenty-one`
> `/// carries a defect the model **cannot express**`

The table it documents runs `:2381-2489` and holds **80 rows: 42 `Agreed`, 38 `Rejected`.** Two of the 42 are the controls, named as such at `:2381-2388` (`GappedPositionStore`, `PagedStreamStore`). So there are **40 misses**, not 21 — the heading is stale by 19 in both numbers. (Counts, not a fraction: `spec/SPECIFICATION.md:7656-7660` forbids the ratio and asks for exactly this shape.)

The enumeration beneath the heading (`:2336-2368`) names 21 stores across six bullets, which is why the heading and its own bullets agree with each other and with nothing else. **Nineteen `Agreed` rows are named in no bullet**, and three of the axes they sit on are absent from the one-line boundary the heading offers — *"the model drives one handle, on one fixture, through a strictly sequential stream of non-empty batches of typical values, and it never reopens and never arms a fixture"* (`:2331-2335`):

- `Op::Read` has **no `to` field at all** (`model.rs:167-178`), so `ToBoundIgnoredStore`, `ToIsExclusiveStore` and `BackwardsToIsAnUpperBoundStore` cannot be reached — three unnamed rows on one axis the sentence does not mention.
- `limit` is `Option<usize>` documented `/// A truncation, if any. Never zero` (`model.rs:175-177`), which is `LimitZeroIsUnlimitedStore` — whose provenance calls it *"the crate's own former behaviour rather than an invented one"* (`mutation_coverage.rs:700-707`).
- `Op::AppendConditional` carries exactly one `query` and one `anchor` (`model.rs:159-166`), which is `MinCollapseStore` — a **liveness** defect whose own provenance says it *"passes the ENTIRE existing `condition_after_*` family"* (`:1751-1759`).

The meta-test at `:3775-3831` asserts that every driven store has a row, that every row's claimed outcome equals the observed one, and that at least one row is `Rejected`. It asserts nothing about the count, so the prose can drift arbitrarily far and stay green — which is what happened.

**Found — `REGISTRY`.** `mutation_coverage.rs:249-257`, the paragraph that tells a reviewer how to discount the shotgun mutant's 18-rule `fails` list:

> `/// Those rows evidence the anchor, not the headline property, and they are`
> `/// inflation rather than vacuity — no rule in the table is covered by that`
> `/// mutant alone.`

`untagged_events_match_query_all` appears in exactly one `fails` list in the whole event-store registry — `InnerJoinTagStore`'s, at `mutation_coverage.rs:362` — so the closing clause is false as written. The claim's *intent* survives: the rule body at `suite.rs:605-621` carries one assertion, so today it cannot fail at an anchor, and for that rule `InnerJoinTagStore` is the headline mutant rather than an inflation row.

**Why a defect.** Two different readers, two different wrong outcomes, one mechanism. For `MODEL_COVERAGE`: whoever decides at the next phase gate how much the model family is worth — or whether to spend on strengthening `Op` — reads a blind-spot inventory that understates the gap by 19 stores and characterises the residue as edge-case shapes at the boundary. The reachable conclusion is that the generator misses a small, well-understood set. The measurable answer is 40 stores, and at least five of them are ordinary read-option and condition semantics that a sequential generator *could* reach with three more fields. For `REGISTRY`: the sentence is the reassurance a reviewer uses to stop checking, and it is aimed at precisely the audit it then makes unnecessary. Nothing breaks today; it breaks the day `untagged_events_match_query_all` gains a second assertion, at which point its only registered evidence may be an anchor failure and CF-1's obligation is discharged in name only — and the paragraph told the auditor not to look. Both are defects rather than preferences on the same discriminator: each sentence is a **counted claim over a machine-readable table**, so it is falsifiable by a four-line assertion in the file it already sits in, and neither has one.

**Corroborates a recorded open question.** `.kb/open-questions/model-family-rule-has-no-clause.md` (`kb-open-question-model-family-rule-no-clause-001`) records that `ops_agree_with_the_model` checks the composition of seven clauses and belongs to none, and is held in `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1968-1997`). `MODEL_COVERAGE` is the only coverage report that family has, and it drifted by 19 rows with nothing in the gate positioned to notice — which is the deferral's cost, arriving. That is the first measured consequence of the gap the open question describes, and it should be attached to it.

**Semver.** **None**, both halves. `MODEL_COVERAGE` and `REGISTRY` are `const` tables in `tests/`, not public API, and no consumer can name them. Neither is a 0.2.0 deadline; both are, however, what an evaluator checks when deciding whether the rule count means anything, which is the argument for doing them before the numbers are read by strangers rather than after.

**Remediation, subordinate to the clause and the routing.** Both sentences can be corrected to the properties that hold, and both can be pinned by an assertion in the meta-test that already walks the table — a count for `MODEL_COVERAGE`, and for `REGISTRY` the narrower true claim plus an `expect` pin naming the assertion `InnerJoinTagStore` is supposed to trip on that rule. Whether the model family's boundary statement is re-derived at the same time, or left for whoever answers the open question, is not settled here.

**Routing.** A maintenance story on `happenstance-testkit`'s own `tests/`, with the corrected `MODEL_COVERAGE` figures attached as evidence to `kb-open-question-model-family-rule-no-clause-001` rather than filed separately. **The decision the open question already names — whether a cross-clause property rule earns a clause of its own — is untouched by this document and remains held in `UNCLAIMED_PENDING_ADR`.**

---

### L3-03 — the concurrency family's conformant control is held by convention, in the one family whose siblings both assert it

**Clause.** **CF-5** — *the testkit MUST also hold at least one conformant variant — a store that is legally different from `MemoryEventStore` and MUST pass every rule* (`spec/SPECIFICATION.md:7626-7628`). **`[FROZEN]`**.

**Examined.** `crates/happenstance-testkit/tests/mutation_coverage.rs:2523-2547` (the `Racer` struct), `:2579-2587` (`LockedStore`), and the meta-test at `:3838-3969`, against the two sibling families' structural guards at `mutation_coverage.rs:2924-2931` and `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:1374-1381`.

**Found.** `Racer` has no `kind` field. The control is identified by an empty list and a prose note, `mutation_coverage.rs:2526-2528`:

> `    /// The **exact** set of concurrency rules this store fails. Empty iff it is`
> `    /// the conformant control.`

and `:2580-2586`:

```rust
        name: "LockedStore",
        fails: &[],
        provenance: "the conformant control: one mutex held across the whole append, which is \
             the shape of rusqlite behind a connection, of a Durable Object, and of \
             `MemoryEventStore` itself",
```

The meta-test's own positive control, at `:3966-3971`, asserts the *other* direction only:

```rust
            assert!(
                RACERS.iter().any(|row| row.fails.contains(rule)),
                "no racing store fails `{rule}`, so it is a rule nothing has \
                 ever been shown to break — which is what ADR-0010 calls \
                 decorative"
            );
```

The event-store family asserts what this one does not, at `:2924-2931`:

```rust
        assert!(
            REGISTRY
                .iter()
                .any(|entry| entry.kind == Kind::ConformantVariant),
            "no conformant variant is registered, so \
             `conformant_variants_pass_everything` asserts over nothing — which \
             is the vacuity of CF-5 reintroduced one level up"
        );
```

and the projection family carries the same guard at `projection_mutation_coverage.rs:1374-1381`.

**Why a defect.** Delete `LockedStore` from `RACERS` and from `for_each_racer!` and every assertion in `the_concurrency_rules_reject_exactly_what_they_claim` still holds: the remaining five racers each fail exactly what they declare, and every rule is still failed by something. The meta-test stays green and so does the gate. The wrong outcome has a plausible author and a plausible motive — the file documents its own rendezvous flakiness at length (`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:135-165`), and `LockedStore` is the one racer that demonstrates nothing and costs five rule-runs per test, so it is the natural thing to drop while chasing an intermittent failure. After that edit the five concurrency rules have only ever been passed by stores registered as mutants, nothing anywhere says so, and CF-5's vacuity argument is reintroduced in the one family where a rule is macro-emitted and has no `REGISTRY` row to fall back on. The discriminator against preference: this is not a suggestion that a check would be nice — it is the observation that two of three families in the same file already carry it and the third does not, and the clause is `[FROZEN]`.

**Semver.** **None.** `Racer` and `RACERS` are private to a test target. Not a 0.2.0 deadline.

**Remediation, subordinate to the clause and the routing.** Keying the assertion off `fails.is_empty()` reproduces the defect one level up, because an empty `fails` list is also what a disarmed mutant looks like — and emptying a flaky row's `fails` list is the cheaper repair than fixing a rendezvous, so that spelling would *manufacture* a control. The shape that does not have this problem is the one the sibling families already use: `Racer` gains the `kind` field `Declared` has, and both assertions key off it.

**Routing.** A maintenance story on `happenstance-testkit`'s `tests/`, carrying CF-5 as its acceptance criterion. No ADR: the clause is unchanged and unchallenged — what is missing is its enforcement in one of the three places it applies.

---

### L2-03 — `report`'s documented assertion that "the scenario completed" is discharged by a predicate that cannot be false

**Clause.** **CF-34** — *performance MUST be measured by a separate harness, and that harness MUST NOT be part of the conformance bar* (`spec/SPECIFICATION.md:8749-8755`). **`[PROVISIONAL]`**, and its `Rule:` line reads *"none — the harness is not the bar, which is the clause's content"*. **No clause governs what the harness asserts about its own records**, which is stated here rather than left implicit: the promise at issue is made in rustdoc and nowhere else.

**Examined.** `crates/happenstance-testkit/src/bench.rs:386-394` and `:422-439` (`BenchmarkRecord::is_well_formed` and `report`), `:335-339` (`BenchmarkPass::is_well_formed`), `:383-396` (`BenchmarkPass::count`), and the three scenarios' unconditional pushes at `:557`, `:588` and `:667-670`.

**Found.** `bench.rs:425-426`:

> `/// One line. It asserts that the scenario completed and that the record is`
> `/// well-formed`

The assertion, `bench.rs:432-437`, checks only the second half:

```rust
        assert!(
            self.is_well_formed(),
            "{scenario}: every pass must report at least one attempt and account \
             for each of them as committed, rejected, refused or failed. Got {self:?}"
        );
```

and neither conjunct of `is_well_formed` (`:392-394`) can be false. The second, `BenchmarkPass::is_well_formed` (`:337-339`), is `self.attempts == self.committed + self.rejected + self.refused + self.failed` — an invariant of `count` (`:384-393`), which increments `attempts` and exactly one of the four on every call. The first, `!self.passes.is_empty()`, cannot be false either: all three shipped scenarios push a pass unconditionally before any early return (`:557`, `:588`, `:667`). The one test that exercises the empty case does so on a record no scenario can produce — `an_empty_record_is_not_well_formed` at `:920-926` constructs one directly through the private `new`.

The early return the assertion was supposed to catch is two lines below its own pass, at `bench.rs:590-595`:

```rust
        let Ok(boundary) = planted else {
            // A seed that never landed leaves no boundary to race for. The
            // record says so — one pass, no contended pass — rather than
            // reporting a contended run that did not happen.
            return record;
        };
```

**Why a defect.** The comment is right that the record *says so*; nothing *reads* it. An adapter author mounts `event_store_benchmarks!` and tracks `BENCH` lines across releases, which is what the harness exists for. Their boundary append starts failing — a migration, a permissions change, a pool exhausted — and `conditional_append_under_contention` prints `BENCH conditional_append_under_contention: seed attempts=1 committed=0 rejected=0 refused=0 failed=1 events=0` and **the test passes**. The contended pass, which is the entire measurement, is absent rather than zero, and the difference between "contention produced no rejections" and "contention never happened" is exactly the distinction the module docs at `bench.rs:53-57` say the record exists to preserve: *"A run in which every contender wins is a valid measurement of the wrong thing, and the record is what makes that legible instead of averaging it away."* They find out when someone compares two releases' numbers and cannot explain the gap. The discriminator: this is not a request for a stronger benchmark — CF-34 forbids that, and `is_well_formed` is honest about being a shape check on counters. It is a documented assertion (`bench.rs:425`) that the code does not perform, on the harness's most-read method.

**Semver.** **None** — correcting the finding as filed, which classed it additive. `BenchmarkRecord::new` (`bench.rs:356`) and `push` (`:364`) are **private**, so no caller can construct a record and the labels a completion check would need can be added without touching the public surface. `passes()`, `pass()`, `scenario()`, `summary()` and `is_well_formed()` all keep their signatures. **0.2.0 is not a deadline for this one**, and it should not consume any part of the window.

**Remediation, subordinate to the clause and the routing.** Either half is available: the sentence at `bench.rs:425` can be narrowed to what the code checks, or the record can carry the passes its scenario owes so the "completed" half becomes real. The second is the larger change and the one that keeps the promise; the first is a one-line edit and is not obviously wrong, because CF-34 is explicit that the harness decides nothing.

**Routing.** A story against `happenstance-testkit`'s `bench` module, behind the off-by-default feature and outside the bar. No ADR — CF-34 already places this outside conformance, and its `[PROVISIONAL]` marker turns on a different question (whether a complexity property is expressible as a deterministic assertion), which this finding neither supplies nor refutes.

---

### L1-4 · L2-04 — two assertions cite a `[FROZEN]` clause other than the one their reachable failure evidences

*Merged from two findings: `L1-4` (`query_matching_nothing_yields_empty`) and `L2-04` (`a_concurrent_reader_never_sees_a_partial_batch`). One defect at two sites — a failure message naming a defect other than the one that fired, sending an adapter author to the wrong subsystem — and one remediation shape, which the same file already demonstrates.*

**Clause.** **ES-9** — *`ReadOptions::from(p)` […] MUST NOT error and MUST NOT return empty on that ground alone* (`spec/SPECIFICATION.md:2798-2805`), **`[FROZEN]`**, which names `query_matching_nothing_yields_empty` as *"the complementary half already in the suite"* (`:2807-2810`). And **ES-18** — *Atomicity. Either every event in the batch lands or none does* (`spec/SPECIFICATION.md:3392-3397`), **`[FROZEN]`**, which names `a_concurrent_reader_never_sees_a_partial_batch` as *"this clause's first sentence asked while the append is still running"* (`:3403-3404`).

**Examined.** `crates/happenstance-testkit/src/suite.rs:829-846` and its helper `read_ok` at `:147-158`; the rule's registered mutant at `crates/happenstance-testkit/tests/mutation_coverage.rs:467-482`. And `crates/happenstance-testkit/src/concurrency.rs:979-998` with its sole consumer at `:814-821`, against the pattern the same file uses at `:544-556`.

**Found — site one.** `suite.rs:829-846`, the rule entire:

```rust
    /// A query that matches nothing yields an empty stream rather than an error.
    pub async fn query_matching_nothing_yields_empty<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("A")]).await;

        let found = read_ok(
            &store,
            &query_of_types(&["Nonexistent"]),
            ReadOptions::new(),
        )
        .await;
        assert!(found.is_empty(), "a query with no matches must not error");

        RuleOutcome::Ran
    }
```

The error half is already owned one layer down: `read_ok` panics with `"read should succeed, got {err:?}"` (`suite.rs:154-157`). So the only way this assertion can fire is a store that returned **events** — and the sentence it prints then tells the author their store errored. Its one registered mutant is `UninternedTypeStore`, whose provenance is *"the clause is dropped rather than the query being refused — so the read widens instead of narrowing"* (`mutation_coverage.rs:475-479`): the widening case, which is the case the message describes wrongly. The message also carries no `{found:?}`, against an idiom that appears on 33 lines of the same file.

**Found — site two.** `concurrency.rs:980-985`:

```rust
        let seen: Vec<SequencedEvent> = match crate::block_on(happenstance_core::collect(
            store.read(&Query::all(), ReadOptions::new()),
        )) {
            Ok(events) => events,
            Err(err) => return std::vec![format!("a concurrent read failed: {err}")],
        };
```

That string is returned into the same `Vec<String>` the helper otherwise fills with batch names (`:987-997`), and the sole consumer asserts over it under a message about atomicity alone, `concurrency.rs:814-820`:

```rust
        assert!(
            partial.is_empty(),
            "a reader observed {} batch(es) part-written: {partial:?}. Either \
             every event of a batch is visible or none is (ES-18), and a reader \
             that can see half of one can build a decision model from a command \
             that never completed",
            partial.len()
        );
```

**Why a defect.** In both cases a correct assertion prints an incorrect diagnosis, and the diagnosis names a subsystem the author will then go and read. At site one, the stranger the rule exists for is an author who interned event types — the standard way to avoid a type string per row, and precisely the shape `UninternedTypeStore` models. Their type clause is dropped, their read widens, and the one rule that catches it hands them `assertion failed: found.is_empty()` under "a query with no matches must not error". They check their error path, find no error, and conclude the testkit is wrong. At site two, an adapter whose read transiently fails under contention — `SQLITE_BUSY` past the handler's ceiling, a pool with no reader slot, a 503 from a one-shot HTTP backend — is told by name, and with a `[FROZEN]` clause ID attached, that its `append` is writing rows outside a transaction. They go looking for a missing `BEGIN` in code that has one. The helper's own doc (`concurrency.rs:975-978`) is right that a read failing under contention is a defect this rule is entitled to name; what is wrong is the **channel**, not the decision to report it.

Site one carries a second, smaller property worth recording because it bears on CF-1: the rule has no non-vacuity anchor, so a store that returns nothing from every read passes it — which is why `InnerJoinTagStore`, whose defect is exactly that for untagged events, is absent from this rule's `fails` list (`mutation_coverage.rs:341-375`). The anchor idiom the file uses elsewhere is thirty lines above, at `suite.rs:800-805`: *"the first query alone must select exactly its own event — otherwise the union assertion below holds for the wrong reason."*

**Semver.** **None**, both sites. Message text and one private helper's return type; no public signature moves and no rule is added or removed, so the bar is unchanged and no adapter re-runs anything. Not a 0.2.0 deadline — but these are what an adapter author meets on the day their build goes red, which is the day the suite's credibility is actually decided.

**Remediation, subordinate to the clause and the routing.** The pattern for site two is already in the same file, 260 lines above: `k_disjoint_boundaries_admit_exactly_k_commits` splits one assertion into two rather than describing two defects in one sentence, and says why at `concurrency.rs:544-550` — *"Zero winners and two winners are opposite defects […] and a single message describing both is a message that identifies neither."* One `Vec<String>` becoming a named pair, and one assertion becoming two, applies that verbatim. Site one is a message rewritten to the failure it can actually have, plus the `positions_of(&found)` its siblings carry, plus an anchor read.

**Routing.** A maintenance story on `happenstance-testkit`'s rule bodies. No ADR and no clause change: ES-9 and ES-18 are both `[FROZEN]` and both are correct — it is the code citing them that has the wrong one.

---

## The rendered examples implement the three hazards `DomainEvent`'s own documentation says the trait exists to remove

`happenstance` has no derive — that is ADR-0033's accepted verdict — so every field of a `DomainEvent` is hand-spelled, and the crate's doc examples are not illustrations of the API but the specification of what an implementor will actually write. In three places the rendered example implements exactly the shape the adjacent doc paragraph says the design exists to prevent. `event_type` indexes `EVENT_TYPES` by position in all thirty-one compiling impls in the tree, which is the practice `domain.rs:73-75` calls "the whole hazard this trait exists to remove"; `decode` discards its `event_type` parameter in every copy-pasteable example, and that parameter is the entire ground on which ADR-0021 Decision 3 earns the answer "no read-path hook is needed"; `tags` is rendered eight times out of eight as `Tags::empty()`, the one case that never meets the fallible constructor the method's own sibling `DecisionModel::scope` documents at length six lines away. None of this is a semver item — the fixes are doc and example edits, free before and after 0.2.0. The deadline is a different and harder one: 0.2.0 is when the copying starts, and one of the three shapes writes wrong data durably while another forecloses the only payload-evolution strategy the ADR corpus records, which is the class of defect a consumer cannot back out of. The five entries after them are the same failure one level down — the teaching material's own routing, line citations and stated compiler rules have drifted from what they name — and the last is governance residue rather than code.

A note on the **Clause** field before the entries, because it is uniform and it is itself a result. `spec/SPECIFICATION.md` has exactly three clause families — `VT`, `WF`, `ES` — and all 88 headings govern `happenstance-core`. `grep -n "DomainEvent" spec/SPECIFICATION.md` returns a single line, `:1416`, and it is prose about `EventType` interning. **No clause governs `DomainEvent`, `Retry`, `docs/` or the examples.** The typed layer's obligations live in accepted ADRs and in rustdoc, and rustdoc is the one surface no step of `cargo xtask ci` reads for truth: `spec-trace` parses `spec/SPECIFICATION.md` and `spec/E2E-CASES.md` by path, and the doc build checks that links resolve, not that sentences are true.

---

### Y-2 — every `event_type()` that compiles indexes `EVENT_TYPES` by position, and the guard ADR-0020 named cannot see a reorder

**Clause.** None. No `VT`/`WF`/`ES` clause governs the typed layer's `DomainEvent`. The governing instrument is **ADR-0020** (accepted, immutable) — atom `.kb/decisions/0020-fold-query-agreement.md:111-112`, full record `references/adr/0020-fold-query-agreement.md:345-349`.

**Examined.** `crates/happenstance/src/domain.rs:68-76`; the census `grep -rn "impl DomainEvent for"` and `grep -rn "fn event_type(&self) -> EventType"` over `crates/happenstance/` and `examples/`; `crates/happenstance/src/testing/mod.rs:379-393`; `examples/course-subscriptions/src/main.rs:186-198`; `examples/course-subscriptions/tests/runs.rs:66,200`.

**Found.** The rationale, `domain.rs:70-75`:

```
/// Returned **by value**, which costs a borrowed-`Cow` clone and no
/// allocation. `-> &'static EventType` lost: `EventType` holds a
/// `Cow<'static, str>`, so const promotion does not apply and the
/// implementation would have to index `EVENT_TYPES` by position — a second
/// place to get the mapping wrong, which is the whole hazard this trait
/// exists to remove.
```

Sixty-two lines later, in the crate's own published doctest, `domain.rs:136-139`:

```
///     fn event_type(&self) -> EventType {
///         match self {
///             Self::Taken => Self::EVENT_TYPES[0].clone(),
///             Self::Freed => Self::EVENT_TYPES[1].clone(),
```

The census is categorical, not partial. Thirty-one `impl DomainEvent` blocks across `crates/happenstance/` and `examples/`; thirty-one `event_type` bodies; **exactly one** does not index by position, and it is `domain.rs:33-34` inside a `compile_fail` fence whose `EVENT_TYPES` is `&[]` and therefore cannot be indexed. `grep -rn "EVENT_TYPES\["` returns 27 lines across 12 files. Seven of the affected sites are rendered doctests — `lib.rs:39`, `domain.rs:137-138`, `runner.rs:374`, `boundary.rs:46`, `composition.rs:128-129`, `testing/mod.rs:23`, `testing/mod.rs:364-365` — which is to say, every published model to copy.

ADR-0020 names `assert_domain_event` as the mitigation. Its body, `testing/mod.rs:381-382`:

```rust
assert!(
    E::EVENT_TYPES.contains(&carried),
```

**Why a defect.** ADR-0020 named a residual and it named it narrowly. `references/adr/0020:346-347` reads: *"a hand-written `event_type()` may return a type **absent from** `EVENT_TYPES` and no `const` sees the match arms."* `assert_domain_event` is exactly that test — a membership test. A **reorder** of `EVENT_TYPES` returns a type that is still a member, so the guard passes, and the hazard the reorder creates is not the residual ADR-0020 priced. That is the discriminator: this is not the accepted price re-reported, it is a second hazard the accepted mitigation is structurally unable to detect.

The named wrong outcome. An author alphabetises `EVENT_TYPES` — three lines, no `match` arm touched, reads as cosmetic. From that commit every event of the affected variants is appended under the **wrong** `EventType`, and `commit` returns `Ok`. Nothing goes red, and each reason is checkable: `Boundary::query` derives one item over *all* declared types (`boundary.rs:118`, `derive_query` at `:172-174`), so the query still nominates them; `absorb`'s filter is `query.matches(...)` (`boundary.rs:138`), so they still match; `decode` ignores the envelope type (see B-3), so the fold still applies the correct arm off serde's own variant identifier; and the worked example's end-to-end transcript test asserts membership only — `examples/course-subscriptions/tests/runs.rs:200`, `EVENT_TYPES.contains(&event_type)`. Discovery is at read time by a *second* consumer of the same store — a projection, a replication peer, a DCB-conformant reader — over events that are already durable.

**Semver.** The doc and example correction is **none**. The instrument is not: teaching `assert_domain_event` to check `every_variant[i].event_type() == EVENT_TYPES[i]` adds a precondition (arguments in declaration order) to a function already published in `happenstance` 0.2.0-alpha.1, turning a passing call into a panic. That is a behaviour break, and **0.2.0 is the last cheap moment for it**. A second entry point instead is additive and free later.

**Remediation.** Two shapes exist and they differ in exactly the semver dimension above: strengthen `assert_domain_event` in place, or add a sibling that checks positional agreement and leave the published one alone. Independently, and cheaply, both worked examples could invoke the guard at all — neither does today (`grep -rn assert_domain_event` finds call sites only inside `crates/happenstance`'s own tests).

**Routing.** The choice between the two shapes is a decision record's, not this document's, and ADR-0020 is accepted and immutable, so the route is a new record rather than an amendment. It belongs to the RUNBOOK's ADR pass, ahead of phase 12's first-publish audit. The census above is the input that record starts from.

---

### B-3 — every copy-pasteable `decode` discards the parameter ADR-0021 Decision 3 rests its whole answer on

**Clause.** None. The governing instruments are `DomainEvent::decode`'s own `# Errors` clause (`crates/happenstance/src/domain.rs:88-99`), `Boundary::absorb`'s (`boundary.rs:103-108`), and **ADR-0021 Decision 3** (`references/adr/0021-payload-evolution-and-codec-tag.md:223-268`), as corrected by ADR-0032.

**Examined.** `crates/happenstance/src/domain.rs:88-99`; `crates/happenstance/src/boundary.rs:103-108`; the five copyable `decode` bodies at `lib.rs:44-45`, `domain.rs:40-41`, `boundary.rs:51-52`, `composition.rs:136-137`, `testing/mod.rs:372-373`; both examples at `examples/course-subscriptions/src/main.rs:216-222` and `examples/transfers-on-sqlite/src/main.rs:306-312`; `references/adr/0021:223-268`.

**Found.** The obligation, `domain.rs:88-94`:

```
/// Decodes a payload written for `event_type`.
///
/// # Errors
///
/// Returns [`CodecError::UnknownEventType`] when `event_type` is not one
/// this type declares, and the codec's own failure when `data` is not a
/// valid payload for it.
```

`Boundary::absorb` resells it as the drift signal, `boundary.rs:105-107`:

```
/// Returns [`CodecError::UnknownEventType`] when a **nominated** event is
/// not one the domain type declares, because that is a real disagreement
/// between the declaration and the fold
```

Every copyable implementation, identical at five sites including the crate root's first program and `assert_domain_event`'s own doctest — `lib.rs:44-45`:

```
//!     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
//!         -> Result<Self, CodecError> { c.decode(d) }
```

And both worked examples spell it out longhand with the same underscore, e.g. `examples/transfers-on-sqlite/src/main.rs:306-312`.

**Why a defect.** The parameter is not decorative — it is the load-bearing half of an accepted ADR. `references/adr/0021:245-252` earns "no read-path hook is needed" on precisely this signature: *"`decode` receives the event type **and** the raw bytes and returns a `Result<Self, CodecError>`. An implementation may therefore try the current payload shape, fall back to an older one, and construct the current variant from it — entirely inside the typed layer, with no port change."* An implementation that discards `event_type` has no discriminator to branch on, so the strategy the ADR names as the reason a hook was not needed is not merely unused, it is unavailable.

Two named wrong outcomes, both discovered late. First, the drift signal is dead: a declared-but-unfolded event type surfaces to the caller as `CodecError::Decode(serde …)` rather than `UnknownEventType`, so a caller matching on `UnknownEventType` because `boundary.rs:105-107` told them what it means never sees it, and a declaration/fold disagreement passes through silently. Second, and worse: the discriminator actually in force is serde's variant identifier inside the payload. Neither example carries `#[serde(rename)]` (`examples/course-subscriptions/src/main.rs:169`), so renaming a Rust variant — a refactor, not a schema change — makes every already-written payload of that variant undecodable, permanently, while the envelope's `EventType` sits in the store holding the correct name that nothing reads. Discovery is on the first replay after the rename, over durable events.

**Semver.** **None** — every affected site is a doc comment or example source. But the shape becomes durable in consumers' stores at 0.2.0, and events written under a payload-discriminated `decode` cannot be retro-fitted with envelope dispatch without a migration.

**Remediation.** The half the crate can keep is the examples: show `decode` matching `event_type` against `EVENT_TYPES` and returning `CodecError::UnknownEventType` on no match, at least in the crate-root program and in `assert_domain_event`'s own doctest. The half it cannot keep is any implementor's `decode`, which is why the question of whether the obligation becomes *assertable* — a second loop in `assert_domain_event` over a fabricated undeclared `EventType` — is a real one and carries the same published-function semver question as Y-2.

**Routing.** The example correction is documentation work with no decision in it. The assertability question is the same decision Y-2 routes and should be taken with it, in one record, since both concern what `assert_domain_event` is for. ADR-0021 is accepted; nothing here contradicts it — it corroborates that Decision 3's falsifier has a second, closer failure mode than the cross-event upcast the record anticipated at `:255-261`.

---

### P-3 — `DomainEvent::tags` is total over a fallible constructor, and the idiom that solves it reaches no rendered surface

**Clause.** None governs the typed layer's `tags`. Corroborates the recorded open question **`.kb/open-questions/d-1-the-validated-type-has-no-total-path.md:58-65`** (`kb-open-question-d-1-no-total-path-001`, accepted, open).

**Examined.** `crates/happenstance/src/domain.rs:78-79` against `crates/happenstance/src/domain.rs:172-178`; the fallible routes at `crates/happenstance-core/src/tag.rs:90`, `:133`, `:304` and the infallible one at `:479`; the eight rendered occurrences of `fn tags(&self) -> Tags` in doc comments; `examples/course-subscriptions/src/main.rs:78-157` and `examples/transfers-on-sqlite/src/main.rs:205-244`; `docs/*.md` and `README.md`.

**Found.** The method and the whole of its documentation, `domain.rs:78-79`:

```rust
    /// The tags this event carries.
    fn tags(&self) -> Tags;
```

One line. No `# Errors`, no `# Panics`, no note on where the fallibility went. Its sibling on the sibling trait, ninety-four lines later at `domain.rs:174-177`, carries the full reasoning for the opposite choice:

```
    /// Already validated: [`Tags`] has no infallible constructor that can
    /// produce an invalid value, so returning a reference to a held value is
    /// what keeps the fallibility in the caller's constructor instead of
    /// inside an infallible signature, where it could only become an `unwrap`.
```

Every rendered example of `tags` — eight of eight, at `boundary.rs:47`, `composition.rs:132`, `domain.rs:36`, `domain.rs:141`, `lib.rs:40`, `runner.rs:375`, `testing/mod.rs:24`, `testing/mod.rs:368` — is `fn tags(&self) -> Tags { Tags::empty() }`, the one case with no tags at all.

**Why a defect.** The documentation demonstrates only the degenerate case, so the reader is not told the ordinary case has a price, let alone that it is payable. The route it leaves them is the one `DecisionModel::scope`'s doc explicitly warns about: an `unwrap` inside an infallible signature. `Tag::new` refuses an empty value, a value over `MAX_TAG_LEN`, a Unicode `Cc` control character, and seven bidirectional formatting controls (`crates/happenstance-core/src/tag.rs:83-88`) — so `Tags::from_pairs([("book", &self.book_id)]).expect("a valid tag")` over an identifier that came off an HTTP request panics on the write path, inside `commit`, after the decision has already been taken. Discovery is the first adversarial input in production.

What is right, and should be said: **the repository knows the answer.** Both examples hold an already-validated `Tag` inside their identifier newtype and `.collect()` it through the infallible `FromIterator<Tag> for Tags` (`tag.rs:479`), and both document why, inline — `examples/course-subscriptions/src/main.rs:80-85` states it in five lines, and `examples/transfers-on-sqlite/src/main.rs:207-210` restates it. The two newtype blocks cost 80 lines (`main.rs:78-157`) and 40 (`main.rs:205-244`) respectively. The defect is that the idiom lives only in example *source*: it appears in no rustdoc, no `README.md`, and no page under `docs/`. The reader who meets the trait on docs.rs never sees it.

**Semver.** **None**. The fix is a doc comment and one replaced doctest; the signature is untouched, and deliberately so — changing `fn tags(&self) -> Tags` is D-1's decision, not a doc pass's.

**Remediation.** One rendered example carrying the ordinary case: an event holding a `String`-derived identifier, the validated-`Tag` newtype, and `.collect()`. It is compiled by the existing doctest step, so it cannot rot. `# Errors`/`# Panics` prose on `tags` naming where the fallibility went would cost nothing and is the smaller half.

**Routing.** D-1 is accepted and open and this corroborates it — the deferred thing is still costing, and it now has a second, cheaper face than the API gap the OQ records. The OQ prices the *API* residual; what this adds is that the residual is currently undocumented on the rendered surface, which is repairable without settling D-1 at all. The signature question stays D-1's and is not touched here.

---

### V-5 — nine call sites convert a compile-time literal at run time against a `const fn`, and two of them are the crate's own rustdoc

**Clause.** None.

**Examined.** `crates/happenstance/src/command.rs:48-53` and its doctest at `:27-34`; `crates/happenstance/src/lib.rs:58`; `examples/course-subscriptions/src/main.rs:23,404,438,478`; `examples/transfers-on-sqlite/src/main.rs:411,436,482`.

**Found.** The constructor, `command.rs:50-53`:

```rust
    #[must_use]
    pub const fn attempts(n: NonZeroU32) -> Self {
        Self { attempts: n }
    }
```

Its own doctest, `command.rs:30` and `:33`:

```
/// let bounded = Retry::attempts(3.try_into()?);  // at most three
…
/// # Ok::<(), core::num::TryFromIntError>(())
```

The crate-root tutorial, `lib.rs:58`: `//! let retry = Retry::attempts(3.try_into()?);`. And all six handler call sites in the two examples, identically: `Retry::attempts(ATTEMPTS.try_into()?),` over `const ATTEMPTS: u32 = 3;`.

**Why a defect.** The `?` propagates a `TryFromIntError` that is statically unreachable, and it is invisible only because both examples return `anyhow::Result` and the doctest hides the fallout on a `#` line. The named wrong outcome is the reader who copies the call into a handler returning a domain error type — the ordinary case, and the one this library pushes them toward, since `CommandError<S, D>`'s domain parameter is bound `core::error::Error`. The `?` no longer compiles. Their two exits are to grow their error enum a `TryFromIntError` variant for a conversion that cannot fail, or to write `.expect("3 is nonzero")` and put a panic on the command path. Discovery is at the first compile of their own second handler, and the fix they reach for is the wrong one, because the crate's own documentation showed them the fallible spelling twice.

The crate built the door and no example walks through it: `attempts` is `const fn`, so `const ATTEMPTS: Retry = Retry::attempts(NonZeroU32::new(3).unwrap());` is a compile-time constant under the 1.97.1 MSRV, and `Retry::attempts(0)` already fails to compile (`command.rs:38-41`).

**Semver.** **None**. No API change is required, because `attempts` is already `const fn` — the fix is nine call sites and two doctests. Additive only if a separate convenience constructor is *also* wanted, which is a different question.

**Remediation.** The const spelling replaces all nine sites without touching a signature, and drops the hidden `# Ok::<(), core::num::TryFromIntError>(())` line at `command.rs:33` that exists only to make the infallible conversion compile. Whether `Retry` additionally grows a literal-taking door is separate and additive.

**Routing.** The nine-site correction has no decision in it. The convenience-constructor question, if anyone wants it, is a small API decision on a crate already at 0.2.0-alpha.1 and belongs in a record, not here.

---

### V-4 — the documentation index routes to four of six pages, and the two it omits are the only route into the worked example

**Clause.** None. Corroborates a deliberate, recorded deferral: **`.bklg/docs-that-teach/application-author-path/_integration.md:116-141`**, section *"W-1 — the one open reach gap, and why it is not counted here"*, in initiative **HS-I0007** (`status: implementing`).

**Examined.** `docs/README.md:13-22`; `ls docs/`; an inbound-link sweep over `docs/`, `README.md`, `crates/`, `examples/`, `xtask/`; `examples/course-subscriptions/tests/reach.rs:28-42,332-343`; `xtask/src/narrative.rs:141,150`.

**Found.** The page declares itself a router, `docs/README.md:13`: *"It routes rather than teaches"*. Its whole narrative table, `:17-22`:

```
| Page | Read it at |
| --- | --- |
| Appending under a condition | [`append-conditions.md`](append-conditions.md) |
| Watching a boundary refuse, in three runnable steps | [`first-encounter.md`](first-encounter.md) |
| What to read before you write an adapter | [`adapter-reading-order.md`](adapter-reading-order.md) |
| Fences the compiler never sees | [`text-fences.md`](text-fences.md) |
```

`docs/` holds six pages. The two absent — `read-the-worked-example.md` and `carry-your-invariant.md` — are the entire handoff chain into `examples/course-subscriptions`. The sweep returns no inbound link from any other `docs/` page, and `grep -n "docs/" README.md` returns **nothing**: the repository README carries no link into `docs/` at all. Both pages are compiled — `xtask/src/narrative.rs:141,150` `include_str!`s them — so the gate proves they are correct and proves nothing about whether anyone can reach them.

**Why a defect.** A reader who has run `cargo add happenstance` and opens the directory whose own README says it is for user documentation is offered four pages, none of which is the canonical DCB program. `examples/course-subscriptions/tests/reach.rs:332-343` exists specifically to keep this chain unbroken and asserts the bridge→page hop; nothing asserts the index→bridge hop, so the chain is guarded from its second link onward and dangles at its first. The obligation was never a rule — it is a per-page assertion three separate stories each hand-wrote (`xtask/tests/first_encounter.rs:480-484`, `xtask/src/lint_narrative.rs:4853-4873`, `reach.rs:332-343`), and the two pages written last got no author who wrote a fourth and fifth.

**Semver.** **None**. Documentation only.

**Remediation.** Two table rows, plus — this is the part that stops recurrence — an assertion that the index carries a row per page, in the shape of the three that already exist per-page.

**Routing.** This corroborates a deferral rather than reporting a surprise, and the deferral's grounds are written down in three places (`project.md:134-139`, `invariant-to-appendcondition-bridge/spec.md:222-223`, `reach-and-adapter-path/project.md:44-47`) — the index row was scoped out at spec time, not dropped. It belongs to **HS-P0023 `reach-and-adapter-path`** (`status: implementing`), whose AC-004 is initiative DoD-7. Two things the owner should know that the deferral note does not carry. The prior audit records the table as three rows at `:19-21`; at `56ef6c5` it is four at `:17-22`, so the table has grown once since the deferral was written and still did not pick these two up. And the story that exists, **HS-S0158** (`stage: spec`), is scoped to the `crate-root-front-door` and `readme-front-door` surfaces — the narrative table is not in its scope lock, so nothing currently owns it.

---

### V-6 — four `path:line` citations into the worked example are stale by exactly 24 lines, in the two files whose stated job is to guard against drift

**Clause.** None. `cargo xtask spec-trace` reads `spec/SPECIFICATION.md` and `spec/E2E-CASES.md` by path and nothing else, so no gate step reads these.

**Examined.** `examples/course-subscriptions/tests/ui/unhandled_variant.rs:3-4` and `examples/course-subscriptions/tests/ui/handled_variant.rs:3-4`; `.kb/open-questions/d-1-the-validated-type-has-no-total-path.md:64`; the true locations in `examples/course-subscriptions/src/main.rs`.

**Found.** Identical text in both fixtures, `unhandled_variant.rs:3-4`:

```
//! A mirror of `examples/course-subscriptions/src/main.rs:194-207` (the domain
//! enum) and `:341-379` (the `Seats` decision model and its fold).
```

At `56ef6c5`, `main.rs:194` is `Self::CourseDefined { .. } => Self::EVENT_TYPES[0].clone(),` — a match arm inside the `DomainEvent` impl, not the enum. `main.rs:341` is inside `Seats::apply`'s body and `:379` is inside `StudentSeat::apply`'s, so the second range starts and ends inside two different folds.

Every citation is off by **exactly 24 lines, in the same direction**, which is what makes this one defect and not four:

| Citation | Construct at the cited start | True range | Construct at the true start |
| --- | --- | --- | --- |
| `main.rs:194-207` (both fixtures) | a `match` arm in `event_type` | `170-183` | `enum Enrolment` |
| `main.rs:341-379` (both fixtures) | a statement in `Seats::apply` | `317-355` | `struct Seats` |
| `main.rs:102-182` (D-1, `:64`) | `CourseId::as_str` | `78-158` | the identity newtype block |

**Why a defect.** The fixture's own next sentence is *"The mirror is the thing most likely to drift, so it is named here"* — the naming is the guard, and the naming is what drifted. A contributor asked to keep the fixture in step follows `main.rs:194-207`, lands in the middle of the `DomainEvent` impl, and concludes the fixture mirrors something it does not. They find out only by reading both files whole, which is the work the citation existed to save. D-1's `:64` compounds it in the other direction: it prices the newtype cost at "81 lines" over a range that at `56ef6c5` starts mid-method and ends mid-enum, so the number and the range no longer agree.

**Semver.** **None**.

**Remediation.** Repointing four ranges is the whole of part one, and it is the one change this document's own lifecycle rule permits in place. Part two is the instrument: a text lint in the shape of the five already in `xtask/src/lints.rs`, parsing `path:START-END` out of `examples/`, `docs/` and `.kb/` and asserting the cited range's first line is not blank and is not mid-construct.

**Routing.** The repointing is documentation work with no decision in it. The instrument is not owned by anything: `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md` is the nearest recorded question but is scoped strictly to `spec-trace`'s *level-marker* comparison, so it is adjacent, not corroborated. Whether the gate acquires a general citation lint is a decision for the RUNBOOK's pass on gate scope — the same pass that has to answer CF-36 — and this document does not take it.

---

### F1-05 — a comment in the flavour-discipline test justifies a hoist with a compiler rule that does not apply to it, in the file ADR-0035 names as the load-bearing guard

**Clause.** None. The rule invoked is constitution atom **RS-22-3** (`standards/rust/22-rpitit-and-lifetime-capture.md:131-138`).

**Examined.** `crates/happenstance/tests/flavours.rs:520-523` against the signature it describes, `crates/happenstance/src/runner.rs:416-427`; the in-tree control at `flavours.rs:371`; `standards/rust/22-rpitit-and-lifetime-capture.md:131-138`; `.kb/decisions/0035-async-trait-through-worker.md:24,45,86-94`.

**Found.** `flavours.rs:521-523`:

```rust
        // Bound to a local rather than inlined: edition 2024 RPITIT captures
        // every in-scope lifetime, and an inlined temporary is E0716.
        let chunk = core::num::NonZeroUsize::new(64).expect("64 is not zero");
```

The subject, `runner.rs:416-421`:

```rust
pub async fn run_projection<S, P, C>(
    events: &S,
    models: &P::Store,
    projection: &mut P,
    codec: &C,
    chunk: NonZeroUsize,
```

**Why a defect.** Both halves of the stated reason are false, and each is falsified by the tree rather than by argument. `run_projection` is a free `pub async fn`, not a trait method, so no RPITIT is involved. Its fifth parameter is `chunk: NonZeroUsize` — a `Copy` scalar taken by value, borrowed by nothing — so an inlined temporary is moved into the future and cannot be dropped while borrowed. RS-22-3 governs a different construct entirely: `standards/rust/22-rpitit-and-lifetime-capture.md:132-138` is about `read(&self, query: &Query, ..) -> impl Stream` capturing the *reference* parameter's lifetime. The in-tree control sits 150 lines up: `flavours.rs:371` performs the **identical** binding, `let chunk = NonZeroUsize::new(64).expect("64 is not zero");`, carrying no rationale at all.

The named wrong outcome. `crates/happenstance/tests/flavours.rs` is not an ordinary test file: ADR-0035 (accepted, 2026-09-02) names it three times as the guard that makes the `worker` `async_trait` exemption safe — `.kb/decisions/0035-async-trait-through-worker.md:88-91`, *"`cargo deny` is an optional probed gate step… `crates/happenstance/tests/flavours.rs` is not optional: it instantiates every typed-layer entry point against a genuinely `!Send` store."* A reader auditing that claim — the reader ADR-0035 sends there — takes away a rule that by-value arguments must be hoisted under edition 2024, and either writes it into their own code as cargo cult or misdiagnoses a real `E0716`, which in this port always comes from `&Query` per ES-13, as a temporary-argument problem. In a repository whose stated standard is that a claim ships with the thing that compiles (`standards/rust/01-standard-of-evidence.md`), this is a claim shipping with nothing.

One correction to the record for accuracy: this entry does **not** rest on `docs/adapter-reading-order.md`. That page's step 4 (`:50-56`) routes to `standards/rust/20-two-flavour-ports.md` and ADR-0001 and stops; it never names `flavours.rs`. ADR-0035 is the true inbound route and it is the stronger one.

**Semver.** **None**. A comment.

**Remediation.** Either delete the two comment lines, matching the identical binding at `:371`, or replace them with the reason that is actually true if one is wanted. Inlining the expression compiles, which is the check.

**Routing.** No decision. It is the lightest entry in this section and is named because the file it sits in is one an accepted ADR points auditors at.

---

### Y-6 — ADR-0033's reopen condition is scoped to ceremony volume, so it excludes the soundness ground Y-2 and B-3 raise

**Clause.** None. The subject is a decision record: **ADR-0033** (`.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md:118-128`, accepted and immutable). Corroborates `.kb/open-questions/d-1-the-validated-type-has-no-total-path.md:20-22`.

**Examined.** `.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md:114-128`; the census in Y-2; `examples/transfers-on-sqlite/src/main.rs:279-313`.

**Found.** The record's verdict is sound and unchanged, and should be said plainly: the classification is published range by range over 532 of 532 lines, both extreme assignments of the contested block agree (0.50:1 and 0.12:1 against a 1.0 threshold), and nothing in the tree since has moved the ratio. The reopen condition, `:120-125`:

```
Reopen **if and only if** D-1 (`kb-open-question-d-1-no-total-path-001`) is
settled with an infallible `Tags` path. A derive that also handled tags
constructed from runtime values would reach into the contested 85 lines
rather than only the 40-line `DomainEvent` impl — but even the full 85-line
swing does not cross the 1.0 threshold from 0.50:1, so this stays a
post-0.1 question.
```

And the ledger it prices against, `:114-116`: *"What a derive would have bought, for the record: 40 lines of 532, 7.5% of the file."*

**Why a defect.** Every criterion in the record is **volume** — lines, percentages, a ratio against a threshold. AC-013's own question is whether the rewritten worked example carries more mapping ceremony than domain logic, and the record answers exactly that and no more. Y-2 and B-3 are not volume claims: they are that the hand-written mapping is spelled positionally with no compiler check, and that a documented obligation is discharged nowhere. An `if and only if` gated on D-1 excludes that ground by construction.

The wrong outcome is procedural, not runtime, and this is the only such entry in the section. It lands on the maintainer: the next person who meets Y-2 in the field opens the record, finds an accepted immutable atom whose reopen trigger their evidence does not satisfy, and either files nothing or writes a superseding record whose relationship to 0033 is unclear — because 0033 never claimed the ground being contested, and making `superseded_by` mean "extended" is the one thing that graph must not come to mean.

**Semver.** **None**, and this is the entry's strongest point: a derive is an additive item on a crate that would not exist yet, so it is **not a freeze-window decision at all**. Nothing here competes for the 0.2.0 window and nothing here should be settled inside it.

**Remediation.** ADR-0033's text can never carry the correction — it is accepted, and `redkiln validate --kb` checks accepted decision bodies against `HEAD`. The precedent for widening a record's ground without superseding it already exists in this repository: ADR-0029 **amends** ADR-0004 rather than superseding it. A second, cheap input is available first: `kb-reference-macros-ceremony-measurement-001` has never been re-taken against the second worked example, written after ADR-0033 — `examples/transfers-on-sqlite/src/main.rs:279-313` is 35 lines of `impl DomainEvent` for three variants.

**Routing.** Not into the 0.2.0 window. This document records the gap and takes no decision on it, per the house rule that ADR authorship belongs to the RUNBOOK's ADR pass and never to a review as a side effect. The owner is that pass; the input it starts from is Y-2's 31-of-31 census plus the re-taken measurement above. D-1 stays where it is — the finding does not settle it and does not need it settled.


---

## Found while writing this document

### RV-1 — the house format document's own lifecycle citation has drifted

**Clause.** None. This is the class `references/evaluation/review-citation-drift.md`
(2026-08-10, pinned `3712c9b`) exists to record, and its §1 predicted recurrence.

**Examined.** `references/evaluation/phase-7-contract-defects.md:10`;
`references/evaluation/README.md:228-231`; `references/evaluation/README.md:83-85`.

**Found.** `phase-7-contract-defects.md:10` states the lifecycle rule and cites it:

> The one permitted in-place change is repointing a `file:line` citation at the
> text it already named (`README.md:83-85`).

`README.md:83-85` is about the **erratum** exception granted to
`ps-clause-pairing-sweep.md`. The rule it means — *"The one permitted exception is
repointing a `file:line` citation at the file it already named"* — is at
`README.md:228-231`.

**Why a defect.** The citation resolves, so `cargo xtask spec-trace` cannot see
it: the checker verifies that a cited location exists, not that it says what the
citing sentence claims. That is exactly the gap `review-citation-drift.md` §1
recorded as having already recurred once after phase 2 closed it. The document
carrying the drift is the one every later evaluation document is modelled on,
including this one — so the error propagates by imitation rather than by edit.

**Semver.** None.

**Remediation.** Subordinate, and unusually cheap: this is precisely the
**one permitted in-place change** the sentence itself describes. Repoint `:83-85`
to `:228-231`. It changes no claim, only whether a reader can follow one.

**Routing.** The immutable-evidence lifecycle allows this repair without a
superseding document. It belongs to whoever owns `references/evaluation/`.
Found by this review while modelling its own header on that document — which is
also the answer to how it went unnoticed: nothing reads a citation except a
reader who follows it.

### RV-2 — a constitution counterexample is nondeterministic, and turns the gate red about once in 126 runs

**Clause.** None. The governing authority is the constitution's own bar,
`standards/rust/01-standard-of-evidence.md` (RS-01-1): *"Name the wrong
implementation, and compile it beside the rule."* This atom does exactly that,
and the assertion it uses to do it is the defect.

**Examined.** `standards/rust/12-manual-impls-and-derive-traps.md:59` (the
assertion), `:40-64` (the whole `Not` fence), compiled by
`xtask/src/constitution.rs` as one doctest module per atom, and run by the
gate's *"the constitution's examples compile"* step
(`cargo test --locked -p xtask --doc`).

**Found.** The fence builds a `Kind` whose derived `Hash` spans `arity` and
`name` while `Borrow<str>` yields only `name`, then asserts the entry cannot be
found by the borrowed probe:

```rust
assert_eq!(registry.get("CourseDefined"), None, "the entry is present and unreachable");
```

That is true of the *hash*, and not true of `HashMap::get`. `get` hashes the
probe to a bucket and then compares by equality **within** that bucket, so when
`hash("CourseDefined")` and `hash((3, "CourseDefined"))` happen to land in the
same bucket, `<Kind as Borrow<str>>::borrow` returns `"CourseDefined"`, the
equality succeeds, and `get` returns `Some(&"decode")`. `RandomState` is seeded
per `HashMap`, so the outcome is redrawn on every run.

Measured over 200,000 fresh maps on the pinned toolchain: **1,586 reachable —
0.79%**, or about one gate run in 126. Observed live: `cargo xtask ci` was
green at `56ef6c5` at the start of this review and red at the end of it with no
change to `standards/`, failing on this line with `left: Some("decode")`,
`right: None`; three immediate re-runs of `cargo test -p xtask --doc` were green.

**Why a defect and not a preference.** The *lesson* is right and the atom is
worth keeping — a key that is present and unreachable is exactly the trap
RS-12-1 exists to teach. What is wrong is that the assertion chosen to
demonstrate it is probabilistic, in a corpus whose entire premise is that a
compiled example is what makes a claim checkable, and in a gate step that is
`REQUIRED` rather than `OPTIONAL`. The named wrong outcome is worse than a
red build: a contributor whose unrelated pull request fails here learns that the
constitution's examples are flaky, and the next genuine failure of this step is
re-run rather than read.

**Semver.** None. `standards/` ships in no crate.

**Remediation.** Subordinate, and deliberately not applied here — editing a
constitution atom as a side effect of a review is the authorship this repository
separates on purpose. The deterministic form of the same lesson is to assert on
the hashes rather than on the lookup, or to insert enough entries that a
bucket collision cannot rescue the probe, or to assert the pair that is always
true: `registry.get("CourseDefined")` disagreeing with
`registry.get(&Kind { arity: 3, name: "CourseDefined".into() })`. The last is
the closest to what RS-12-1 actually teaches, since it is the *disagreement*
between the owned and borrowed forms that is the bug.

**Routing.** `standards/rust/`'s owner. It needs no ADR — nothing in
`.kb/decisions/` or `spec/` governs an atom's choice of assertion — and it is
the only entry in this review found by re-running the gate rather than by
reading anything, which is the argument for re-running it.

### RV-3 — `experiments/` is inert to `affected` and visible to `spec-trace`, so adding an experiment can redden a specification citation

**Clause.** None. This is `xtask`'s own consistency, and the two checks disagree
about what `experiments/` is.

**Examined.** `xtask/src/affected.rs:291-303` (`is_inert`, whose `INERT` list
contains `"experiments/"`); `xtask/src/spec_trace.rs:2275-2296` (the directory
walk, which skips `target`, `.git`, `.claude` and `node_modules` and **not**
`experiments/`).

**Found.** `affected.rs:302` treats the directory as reaching no package:

```rust
const INERT: &[&str] = &[
    "spec/",
    "standards/pages/",
    "references/",
    "experiments/",
```

`spec_trace.rs:2296` does not:

```rust
if name == "target" || name == ".git" || name == ".claude" || name == "node_modules"
```

So `spec-trace` indexes every experiment's sources by basename, and a bare-name
citation in `spec/SPECIFICATION.md` that previously resolved once now resolves
twice.

**Observed, in this review, by causing it.** Adding
`experiments/shipped-append-condition-sql/src/store.rs` and
`experiments/suite-against-wrong-adapters/tests/support/fixtures.rs` turned 19
citations of `store.rs:NNN` and one of `fixtures.rs:64-67` into
`is a bare name the workspace defines 2 times`, and `cargo xtask ci` went red on
`specification traceability` — with the failure reported against
`spec/SPECIFICATION.md` lines that had not changed and have nothing to do with
the experiment. Renaming the two files to `probe_store.rs` and
`wrong_fixtures.rs` restored `traceability: no problems found`.

**Why a defect.** The existing seven experiments do not collide only by
accident of naming; nothing states the constraint and no check enforces it at
the point of authorship. `store.rs`, `row.rs`, `query_sql.rs`, `lib.rs`,
`fixtures.rs`, `harness.rs`, `correct.rs` and `mod.rs` are all basenames the
current experiments already share with `crates/`, and three of them are cited
bare in the specification. Who breaks and when: the next person who writes an
experiment — which is the repository's own recommended instrument for pricing a
decision — names a file the obvious thing, and the gate blames the
specification. The error message says *"These are defects in the specification,
not in the checker"*, which in this case is false in both halves.

The same class already had one recorded instance: `spec_trace.rs:2275-2284`
documents `.claude/worktrees/` producing **153 traceability problems** for
exactly this reason, and the fix was to skip that directory. `experiments/` is
the same shape and was not added.

**Semver.** None.

**Remediation.** Subordinate: either add `experiments/` to the walk's skip list,
matching `affected.rs`, or state the naming constraint where an experiment
author will meet it. The first is one word and makes the two checks agree about
what the directory is; whether an experiment's sources *should* be citable from
the specification is the actual question, and it is not this document's to
settle — `spec/SPECIFICATION.md` does cite `experiments/wire-format/` today, so
the answer is not obviously "skip it".

**Routing.** `xtask`'s owner. Related to, and sharper than, the general
observation in *What this review did not open* that the gate's own 21,503 lines
were unreviewed until this pass. Found by breaking the gate rather than by
reading it, which is the second entry in this document with that provenance.
