# Certification review — `happenstance` 0.2.0

**Commit:** `6c7a7a8`, branch `lane/0.2.0-closeout` · **Date:** 2026-09-08 · **Scope:** the whole
workspace at the release commit, plus the live behaviour of every adapter that has a backend to run
against.

**What this is.** A certification review answering four questions the owner asked — is this library
performant, stable, elegantly architected, and fit for purpose — with one worry stated explicitly:
that ADRs written at the outset, or when SQLite was the only adapter, may be *opinionated in a way
that hamstrings future adapters*.

**What it is not.** It changes nothing. Findings that imply an ADR, a specification amendment or a
KB ingest are written as recommendations naming the clause and the atom; none is applied.

**Method,** in full at [Appendix A](#appendix-a--method). Three exploration passes; seven live
verification runs including the full gate, live PostgreSQL, live Neon repeated ten times, wasm32
conformance, and a complete benchmark re-run; twenty finder lenses each pipelined into three
adversarial verifiers, plus a second pass; nine per-axis verdicts; and a completeness critic.
110 agents, 108 findings raised, 95 confirmed by two or three of three verifiers, clustered into
23 themes. **[Read §8 before quoting any number in this report](#8-evidentiary-standing--what-was-measured-and-what-was-reasoned):**
some findings rest on a live run and some on reading a clause, and the report says which.

---

## 1. Verdict

| Axis | Verdict |
| --- | --- |
| Performant | **PASS WITH CONDITIONS** |
| Stable | **PASS WITH CONDITIONS** |
| Elegantly architected | **PASS WITH CONDITIONS** |
| Adapter-neutral *(the central question)* | **PASS WITH CONDITIONS** |
| Test and gate integrity | **PASS WITH CONDITIONS** |
| Rust idiom and API design | **PASS WITH CONDITIONS** |
| Developer experience | **PASS WITH CONDITIONS** |
| Documentation and artifact currency | **PASS WITH CONDITIONS** |
| Release readiness | **GO WITH CONDITIONS — do not publish today** |

No axis fails. No axis passes unconditionally. That uniformity is not hedging, and it is the single
most useful thing this review found:

> **The library is in better shape than the apparatus around it.** Every axis is held below an
> unconditional pass by the same class of defect — documents, gates and decision records that
> describe a version of this project that no longer exists — and almost none by the code.

Nine of the twenty-three themes block the release. Every one of them is cheap now and permanent
after `cargo publish`, which is the whole argument of §5.

---

## 2. The question you actually asked

> *"I am concerned that some of our ADRs may have been written at the outset, or when the only
> adapter was sqlite — and as a result they are opinionated in a way that hamstrings future
> adapters."*

**On the `EventStore` side, that worry is largely not borne out, and the evidence is unusually
strong.** Five prospective adapters were designed against the frozen contract, method by method, by
agents whose brief was to find the clause that blocks them:

| Prospective adapter | Result |
| --- | --- |
| Object store (S3/GCS, CAS on a manifest) | **No frozen clause blocks it.** ES-10 is *free* here via compare-and-swap, where Postgres had to buy it with `xid8`. ES-11/ES-12 are met better than on `happenstance-neon`. The port's shape costs it nothing. |
| Append-only log (Kafka/Kinesis) | Two blockers, **both the specification being correctly strict**: DCB itself requires one total sequence, and these systems have no compare-and-set of any kind. Saying so plainly is a service, not a defect. |
| Conditional-write KV (DynamoDB/Cosmos) | ES-10, ES-18, ES-19 and ES-26 all survive the walk cleanly. What blocks it is not a port method — see the two real findings below. |
| Distributed transactional (FoundationDB/Spanner) | ES-23 and ES-24 *"describe FoundationDB's `commit_unknown_result` and its verbatim-reissue resolution better than most databases document it themselves."* Two hard blockers, below. |
| Embedded KV + document store, against `ProjectionStore` | The port is well designed and four adapters are real evidence it holds. The defects are in the **freeze condition**, not the trait. |

That is a good result and it should be read as one. The ADR corpus was audited in three cohorts —
the 27 decisions taken with **no adapter in existence**, the SQLite-only window, and the
multi-adapter phases — and the cohort-A audit's conclusion was that *"the EventStore-side cohort-A
freezes were validated by the adapters that arrived later"*: ADR-0013's four CF-25 risk acceptances
now have three of four axes filled by real adapters; ADR-0001's two-flavour derivation survived a
genuine `!Send` adapter and a genuine no-connection one; ADR-0011's ceiling-and-page is what
`happenstance-cloudflare`'s hand-written cursor is built on, and it cites it approvingly.

**What is real is sharper than the worry, and it is this:**

> What SQLite silently supplied to every freeze was not an *invariant*. It was two *mechanisms* — a
> dense 64-bit counter, and a server-side predicate engine — and both were priced into frozen
> clauses by five adapters that are all SQL engines.

Three consequences, each a concrete finding:

**(a) The capacity floors compose into an atomic append nothing can refuse.** VT-21, VT-22 and VT-24
are three independent MUSTs a store must satisfy *simultaneously*, so the smallest append every
conformant store is obliged to accept atomically is 128 × 65,536 = **8,388,608 bytes** of payload
alone, and 10,477,568 with tags at `MAX_TAG_LEN`. ES-18 makes that atomic. That is larger than a
FoundationDB transaction (10 MB) and past DynamoDB's `TransactWriteItems` budget — and
`StoreLimit`'s three variants cannot express the refusal, while VT-25 forbids reporting it through
`AppendError::Store`. The workspace **already solved this on the other port**:
`crates/happenstance-sync/src/peer.rs:290-292` carries `max_batch_bytes`. Both `StoreLimit` and
`AppendError` are `#[non_exhaustive]`, so a `BatchBytes` variant is additive and free today.

**(b) `SequencePosition` is 32 bits narrower than the token three frozen clauses jointly demand.**
VT-11 + ES-19 + ES-10 together require a store-wide value that is unique, strictly increasing,
strictly ascending *within* one append in slice order, and ordered identically to visibility. That
is a commit-order token plus a sub-order — 80 to 96 bits in the one system class that ships exactly
that primitive. ES-10 itself is *correctly strict*; the reviewers were clear about that. It is the
64-bit dense counter, not the invariant, that came from SQLite.

**(c) There is no "condition evaluation" axis in the instrument portfolio.** CF-25 obliges the
portfolio to name the axis a port is most likely to be wrong about before freezing it
(`spec/SPECIFICATION.md:8785-8797`). Seven axes are named and *who evaluates the append condition* is
not one of them — yet all five `EventStore` implementations, the reference store included, answer
the DCB predicate by handing an arbitrary type/tag expression to an engine that evaluates it
atomically with the insert. **That is one storage shape wearing five hats on precisely the property
ES-25, ES-27 and ES-18 jointly demand.** And the two conformance rules that would reject a
keyed-CAS adapter's wrong construction are blind: no rule anywhere puts two tags in an append
condition, and the disjoint-boundaries rule uses boundaries disjoint in every tag.

**On the `ProjectionStore` side the worry lands squarely, and it is worse than "shaped by SQLite" —
it was shaped by `rusqlite` specifically.** `crates/happenstance-core/src/projection.rs:70-84`
justifies dropping the GAT with *"a transaction cannot outlive its connection"*, which is a fact
about `rusqlite::Transaction<'a>` and nothing else. sqlx refuted it; Neon refuted it; Ladybug refuted
the *reasoning* while agreeing with the outcome, with a compiling counter-example preserved at
`experiments/live-handle-projection-batch/`. Postgres then produced the strongest result: PS-2's own
"live transaction" far end is **forbidden by the port's signatures**, because `begin` is total,
synchronous and infallible and no real driver acquires a transaction that way. The embedded-KV lens
found a **third independent mechanism** (PS-6's infallibility) that the tree has not recorded.

So PS-2's freeze condition — two adapters at opposite ends of the batch-shape axis — is **not
satisfiable as written**, and the port's own status page gives a gating reason four adapters have
falsified. That is the one place where an early decision is genuinely holding the project still.

---

## 3. What is genuinely good

A certification that lists only defects is not a certification. These survived deliberate attempts
to break them, and several are better than the field norm.

- **`unsafe_code = "forbid"` holds, with zero `unsafe` anywhere in workspace code.** The only
  `unsafe` mentions are doc comments explaining why `worker`'s hatch is *not* inherited.
- **The gate is real.** 36 of 37 steps green in 256 s: zero clippy warnings under `-D warnings`,
  zero rustdoc warnings across **five** documentation configurations including the nightly
  `--cfg docsrs` builds and the middle-of-the-feature-range case a consumer actually stands in,
  `cargo deny` clean, feature powerset 242/242 on host and clean on wasm32, all 13 out-of-workspace
  trees compiling.
- **Four adapters over storage this workspace does not control pass their suites against live
  backends** — measured today, not asserted: Postgres 131/132, Neon 105/105 in each of seven
  completed repetitions, Cloudflare 94/94 on wasm32.
- **The falsifier discipline works.** ES-11's marker predicted a failure, on the axis it named, on
  the adapter it named. It got the *mechanism* wrong and the tree recorded that too. This is the
  strongest evidence in the repository that the maturity markers are not decoration.
- **The mutant registry is real engineering.** Every rule in every family is held to a registered
  wrong implementation *in both directions* — a mutant must fail exactly its declared rules and pass
  or state a skip on all the others — with mandatory provenance so a saboteur cannot satisfy CF-1.
- **The Rust is in the top few percent.** The idiom lens's words: `Borrow<str>` on `EventType`/`Tag`
  with hand-written `Hash`/`Ord` that stay consistent with `str`'s; the `NonZeroU64` niche justified
  by where `Option<SequencePosition>` actually appears; `checked_add` with the reason for rejecting
  `saturating_add` written at the line; `#[non_exhaustive]` used precisely where a value must be
  readable but not fabricable and **deliberately not** on `CommandOutcome`, which is designed not to
  grow. *"That last restraint is the rarest of the lot."*
- **`xtask` is not an unreviewed liability.** 30,591 lines carrying **446 tests**, run by the
  workspace test step, with explicit *what this does not verify* headers per module. One real defect
  in a body that large is a good result.
- **The decode surface does not panic.** The completeness critic built the fuzzer the tree lacks and
  ran 400,000 iterations of random and mutated bytes through `postcard` and `serde_json` for
  `Event`, `SequencedEvent`, `Query`, `Tags` and `AppendCondition`: **zero panics**, and no
  speculative allocation on a tampered length prefix. **Do not chase this gap.**
- **Every adapter documents its own costs**, several better than the databases underneath them
  document theirs. `happenstance-neon`'s honest section — *"the `Stream` impl below is a shape, not
  a capability"* — is the kind of thing most libraries would hide.

---

## 4. Six root causes

Ninety-five findings in a repository this disciplined are not ninety-five mistakes. **Five of the
six causes are process failures around an architecture that largely held.**

**1 — A decision burst whose last mile was never walked.** Roughly 23 atoms (0037–0059) landed on
2026-09-07, authored by `/redkiln:kb-ingest` from remediation briefs written as *recommendations*.
The ingest promoted brief prose into **immutable** decision bodies with no check that the remedy had
been executed. Three atoms now assert in the past tense edits that never happened, and ADR-0059
decides four actions with a *"before 0.2.0"* deadline of which zero were taken. Because accepted
atoms are immutable, the false record cannot be edited out — only superseded.

**2 — A release decision that outran its own prose, guarded by a check that reads one sentence.**
Commit `e597c34` made the release set seven crates in the manifests and in `xtask/src/package.rs`,
whose `reconcile` is a genuine two-way check — and therefore stayed green while nine documents kept
saying five. The instrument for exactly this class already exists (`COUNT_BEARING_DOCS`, built after
a README shipped "two rules of seventeen" for fifteen commits) and its four entries are all rule
counts. **The tree built the right check for the right defect and then let the same defect recur
along a different axis.**

**3 — The honesty machinery describes designs the code replaced.** `spec-trace` verifies that a
citation resolves and a marker is well formed; `lint_pages` verifies a page is registered;
`lints.rs` verifies counts in four named files. **Nothing verifies a claim.** So a capability
declension states a read path the crate's own `Send` argument proves impossible; a PROVISIONAL
marker names two shipped crates as unbuilt; §6.5's prose census contradicts the table three lines
above it.

**4 — The checks that would have caught causes 1–3 are individually off, absent or vacuous.** Three
CI jobs or steps carry `if: false`; `cargo-semver-checks` runs 0 of 254 lints and exits 0 at the
exact release it exists for; the wasm32 half of the proof gate never reads its own transcript. *The
pattern is not laziness — every exclusion was argued on its own merits, and the arguments are among
the best prose in the repository. No artefact holds the aggregate, so the green tick keeps arriving
over a shrinking set of guarantees.*

**5 — The portfolio's axes were drawn from the adapters that exist.** §2(c) above. This is the
correct restatement of the owner's worry, and it is sharper than "clauses frozen against one storage
shape".

**6 — Nothing in this repository reasons about two components at the same time.** *This one was not
on the candidate list and it is the most important.* **The typed layer has never been run against
postgres, neon or cloudflare** — every example and every test binds it to `MemoryEventStore` or
sqlite. Every one of the highest-severity live defects is a pair of individually correct,
individually documented decisions failing at an untested seam. The workspace's discipline is
per-artefact and excellent per artefact. **What has no owner is the join** — and it is also the
cheapest thing to fix: most composition defects need a paragraph in the right rustdoc plus one
additive seam taken while additive is still free.

---

## 5. Release conditions

**Recommendation: do not run `cargo publish` today.** Not because the library is unready — it is
release-worthy — but because six of the artefacts publishing makes permanent are currently wrong
about the release they describe, and one required CI job is red on `main` at the release content.
Everything below is hours, not weeks.

### Blocking, in the order they must be done

1. **Ingest the three unratified ADRs** (`/redkiln:kb-ingest` on the staged 2026-09-08 briefs).
   `spec/SPECIFICATION.md` cites ADR-0025, ADR-0060 and ADR-0061 as amending authority in **eleven**
   passages including PS-2, which is `[FROZEN]` — and all three read `Status: proposed`, have no
   atom, and sit in the one directory whose in-place-correction policy is itself an open question.
   The check that would notice is the `backlog` job at `if: false`. **(T-03)**
2. **Execute or supersede the phase-12 atoms that claim work that was not done** — ADR-0055's four
   clone-cost sites plus a fifth uncensused one, ADR-0058's crates.io sentence, ADR-0059 in full or
   its own written fallback. Do not tag with ADR-0059's deadline silently missed. **(T-02)**
3. **Reconcile the crate set across the nine documents that still say five** — including
   `crates/happenstance-postgres/src/lib.rs:23-24` and `crates/happenstance-neon/src/lib.rs:11-16`,
   which will otherwise tell a docs.rs reader that the crate they are reading is `publish = false`.
   **(T-01)**
4. **Fix the gates.** Give `wasm_run`/`wasm_unit_run` the host path's shape so the 94 wasm32 rules
   cannot be silenced with the gate green, and correct the false claim at
   `xtask/src/main.rs:426-428`. Add `--exclude happenstance-ladybug` to `ci.yml:895`. Re-enable the
   semver job — **and see §7, because the documented one-line re-enable does not cover the two new
   crates.** **(T-04, T-09)**
5. **Take the additive API changes while they are still free** — drop `PartialOrd, Ord` from
   `RecordedAt`; add `StoreLimit::BatchBytes`; add `AppendError::Busy`. All three are additive on
   `#[non_exhaustive]` types today and breaking changes after publication. **(T-05, T-08, T-12)**
6. **Correct the four crate pages that state properties their own source refutes** — most sharply
   `happenstance-postgres`'s *"writers are never serialised"*, refuted by its own retry machinery
   600 lines away. **(T-06)**
7. **Discharge ES-23** — it is `[FROZEN]` and obliges every adapter to state what a dropped `append`
   future does. Postgres and Neon state nothing, and Postgres's real behaviour is verbatim the
   clause's own `Rejects:` shape. **(T-07)**
8. **Fix the two drop-boundary defects**: `PgCursor`'s wrong runtime handle, and Ladybug's three
   decode paths that return with the transaction open. **(T-07)**
9. **Fold the CHANGELOG** — the dated `## [0.2.0]` section describes a five-crate release and the
   Neon ES-11 disclosure is stranded under `[Unreleased]`. **(T-01)**
10. **Correct `run_projection`'s rustdoc**, which states as a *port* property something two shipped
    adapters do not have, and state the pool arithmetic where the caller stands. **(T-13)**

### Then publish, and add one instrument afterwards

The critic's observation, which no individual lens could make: **six confirmed findings share one
syntactic shape** — an accepted ADR asserting in the past tense that an edit was made. Every one is
a claim of the form *"file X now says Y"*, and every one is false at HEAD. **That is a lintable
class. One `xtask` module would have caught six findings.**

---

## 6. Findings by theme

95 confirmed findings, clustered into 23 themes, none left unmerged. Ranked by severity × blast
radius × how hard it is to fix later.

| # | Theme | Sev | Effort | Blocks |
| --- | --- | --- | --- | --- |
| T-01 | The release set moved from five crates to seven and the prose did not follow — including on two crates.io front pages | critical | small | ● |
| T-02 | Accepted, immutable atoms assert corrections never applied — one with a deadline expiring at this tag | critical | medium | ● |
| T-03 | The frozen specification and two published crate roots rest on three ADRs nobody accepted | critical | medium | ● |
| T-04 | Gates that are green because what they check is switched off, absent, or vacuous | critical | medium | ● |
| T-08 | No byte budget composes: the contract and three adapters bound size per-factor, and nothing multiplies | critical | medium | ● |
| T-05 | Public API decisions that are free today and breaking after publication | high | medium | ● |
| T-06 | Crate documentation states properties the code refutes, on pages that become permanent | high | medium | ● |
| T-07 | Adapter resource and cancellation defects at the drop boundary, incl. an undischarged FROZEN doc MUST | high | small | ● |
| T-09 | The publish sequence is exercised for three of seven crates, and the leaks it would catch are in the tree | high | small | ● |
| T-13 | `run_projection` composes with real adapters in ways neither port describes | high | medium | ● |
| T-10 | Skips and declensions dressed as facts: three instruments certify nothing and say otherwise | high | medium | |
| T-11 | The specification's own coverage ledger contradicts itself and understates what shipped | high | medium | |
| T-12 | No vocabulary for "contended, retry me", so the caller-facing retry loop is non-convergent | high | medium | |
| T-14 | Decisions argued from in-memory and SQLite shapes whose evidence a later adapter refuted | high | medium | |
| T-15 | The portfolio's axes were drawn from the adapters that exist | high | medium | |
| T-16 | The `ProjectionStore` freeze condition is unreachable, and its status page gives a falsified reason | high | medium | |
| T-17 | Large, measured, avoidable costs on the typed layer's fold and inside SQLite's write lock | high | medium | |
| T-18 | Frozen clauses that exclude whole storage families | high | large | |
| T-19 | User docs teach the layer the user did not install and route adapter authors to a skeleton | high | medium | |
| T-20 | The two live conformance runs are not reproducible as documented | medium | small | |
| T-21 | Entry-point documents state as settled what the specification records as falsified | medium | small | |
| T-22 | Benchmark documentation is false at HEAD in three places | medium | small | |
| T-23 | Two Rust-idiom divergences where the workspace has two answers to one question | low | trivial | |

### The five that matter most beyond the release checklist

**T-08 — nothing multiplies.** The clearest single expression of root cause 5. Beyond the ~8–10 MB
atomic-append floor, the same per-factor habit runs through the read side: ADR-0053 gave
`happenstance-sqlite` a public 8 MiB page budget and looked at no other adapter, so Postgres buffers
1,024 rows with **no byte bound** (~128× its sibling's published promise), Cloudflare pages on row
count alone inside a hard 128 MiB isolate where one page at its own declared ceilings is *exactly*
that cap, and Neon's 64 MiB response ceiling exceeds the whole memory budget of the wasm32 runtime
the crate advertises. *"Every limit in this workspace was derived from the constraint of one thing
at a time, and no artefact anywhere holds a product."*

**T-12 — no channel for "contended, retry me".** `AppendError` has four variants and no transient
one, so an adapter over optimistic storage has two conformant moves and both are wrong: retry
internally with invisible latency, or return `Store(E)`, which `commit_with` treats as terminal.
`happenstance-postgres` does **both**. Worse, its read is frontier-bounded and its append condition
deliberately is not — so `commit_with`'s re-read after a rejection is the one view that *cannot*
contain the event the append was refused for, and the loop burns its whole budget in microseconds
discovering that. `Retry` is a budget with no spacing, *which is precisely the defect HEAD's own
commit diagnosed and fixed one layer down inside the adapter on the same day.*

**T-16 — the projection port cannot reach its own freeze bar.** §2 above. The recommendation is to
rewrite the status page to state what is true — four adapters over storage this workspace does not
control clear the suite, and the port stays gated because all four are owned buffered write sets
after the borrowed end proved undischargeable — and to replace PS-2's axis rather than wait on it.

**T-17 — two large, measured, avoidable costs.** `Boundary::absorb` rebuilds and discards a whole
`Query` for **every event it folds** — measured at 100% of the fold's heap traffic and 5.2–6.3× its
own wall clock — while `commit_with` is still holding the byte-identical query it derived one line
earlier. And SQLite's `bump_cardinality` runs one statement per tag per event **inside
`BEGIN IMMEDIATE`**: 2.97 ms of write-lock hold per batch at the specification's own two floors,
against 137 µs — while its immediate neighbour on the identical data is chunked to the parameter
budget.

**T-19 — `docs/` teaches the wrong crate.** Across seven pages it names `happenstance-neon`,
`-cloudflare`, `-testkit` and `-ladybug` **zero times each**, contains zero occurrences of
*projection*, *read model* or *checkpoint*, and teaches `happenstance-core`'s untyped API rather
than the typed layer the bare crate is. The critic put the number beside it: the tree carries
roughly **1.5 lines of prose per line of code** — ~93,420 lines across `.kb/`, `references/`, the
specification and `standards/` — of which the artefact aimed at users is **649 lines, or 0.6%**.

---

## 7. One finding this review got wrong

**A finding this review's own verifiers rejected 0-of-3 is true, and it is release-blocking.**

`release`'s claim was that the semver net gives zero signal *and* that its documented one-line
re-enable leaves two crates uncovered. Three adversarial verifiers voted it down. The completeness
critic checked it in `ci.yml` and it is correct:

- the **enabled** rev-baseline step lists five packages (`ci.yml:970`): `happenstance`,
  `-core`, `-testkit`, `-sqlite`, `-cloudflare`;
- the **disabled** registry-baseline step lists seven (`ci.yml:1006`), behind `if: false` at
  `ci.yml:1003`;
- so **neither `happenstance-postgres` nor `happenstance-neon` is watched by any semver instrument
  that runs** — and the comment above the enabled step (`ci.yml:946-951`) says *"Every crate the
  release publishes — all five of them… That foresight is why this line needed no edit at the
  release."* The comment asserts a coverage the line beneath it does not provide.

Combined with today's live measurement — 0 of 254 lints, exit 0, all seven crates — the honest
statement is: **at the 0.2.0 release commit, no semver check covers two of the seven crates being
published, and the file says otherwise.**

This is recorded here rather than quietly promoted because it bears on how the rest of the report
should be read. **The 95/108 confirmation rate measures agreement, not rigour.** The vote is noisy
in both directions: three lenses raised the same frontier/retry mechanism and it was confirmed twice
and rejected once; a `critical` on PS-2's freeze condition was rejected 0-of-3 while a `high` making
the same argument from the DX angle was confirmed. Do not read the confirmation count as a quality
signal.

---

## 8. Evidentiary standing — what was measured and what was reasoned

**Measured, against live systems, today.** The gate (37 steps declared, 36 run, 1 probe-skipped,
exit 0, 256 s). Live PostgreSQL 17.10 (132 listed, 132 run, 131 pass at `--test-threads=2`; 27 of
105 failing at the *documented* invocation on a 20-CPU host, proven from 51× `FATAL: sorry, too many
clients already` in the server's own log; flake curve 27/132 → 1/132 → 0/132 across 20/2/1 threads).
Live Neon (105/105 in each of 7 completed repetitions; 363 schemas created per run against a
documented "around ninety"; the 512 MB project exhausted at run 8). wasm32 (94/94 under a Node
`node:sqlite` shim). The full benchmark harness. `cargo deny`, `cargo hack` ×2, MSRV at 1.97.1,
five documentation configurations, `cargo-semver-checks`. All 13 out-of-workspace trees.

**Reasoned from the code, not executed.** Every finding from the five prospective-adapter lenses —
eleven confirmed findings — is of this kind *by construction*: no append-only-log, KV, distributed,
object-store or embedded adapter exists, so their verifiers could only check whether the finder read
the clause correctly. **A 3-of-3 vote on a prospective-adapter argument is not the same evidentiary
object as a 3-of-3 vote on `PgCursor`'s runtime handle.** Treat §2's (a), (b) and (c) as
well-reasoned arguments to be tested by building something, not as measurements.

**Two severities this review's own critic corrected downward.**

- **T-08's `critical` is probably a `high`.** The arithmetic is right, but `StoreLimit` is
  `#[non_exhaustive]`, so the remedy is additive and a patch release can carry it, and the victim is
  a prospective adapter class with no member in the workspace.
- **`RecordedAt: Ord`'s mechanism is over-read.** VT-9 binds *implementations* and the contract's
  *statements*, not the presence of a trait impl — and VT-9 is `[PROVISIONAL]`, not frozen, which the
  finding's "0.2.0 freezes it" elides. The right, smaller finding still stands: an unused derive on a
  public type creates an affordance the specification spends four paragraphs discouraging, and
  removing it later is breaking.

**Where the tree's reasoning is better than a finding credited it.** ADR-0059's own frontmatter
anticipates the miss — *"the additive sibling is the fallback if that deadline passes"* — so it is a
predicted slip with a written fallback, not an unexecuted decision. And `RUNBOOK.md:608` already
states the Postgres read-your-own-writes cost in the ledger; the defect is that the *consumer-facing*
surface does not.

---

## 9. What this review did not do

Named so the certification is not read as broader than it is.

- **No `workerd` run.** 94/94 under a Node shim establishes that the SQL is correct and the `!Send`
  types compose. It establishes nothing about the 128 MiB isolate cap, the frozen-clock behaviour
  VT-9's own falsifier names, or Durable Object storage-gate semantics. The tree says so itself at
  `SECURITY.md:44-49`. **The two Cloudflare findings are exactly the two a workerd run under the
  memory limit would convert from argument into measurement.**
- **No Ladybug run.** Its CI job is `if: false` — frozen on 2026-09-08, the release day, with its own
  comment recording *"This job had never run when it was frozen"* — and today's local gate
  probe-skipped it. **`happenstance-ladybug` is the only adapter in the workspace whose conformance
  has never been observed by a second machine.**
- **No soak.** Nothing ran longer than a few minutes. Three findings point at unbounded growth or
  leakage (`PgCursor`'s `mem::forget`, `run_projection` pinning a connection, the Neon sweeper) and
  all three are inferred from code. **A four-hour projection replay against Postgres with
  `pg_stat_activity` sampled would settle all three at once — the highest-value missing modality
  after `workerd`.**
- **No `cargo-mutants` run against the adapters.** The instrument exists (a 4,000-line mutant
  registry) and has never been pointed at `happenstance-core`'s condition evaluator or
  `happenstance-sqlite`'s SQL. **The single highest-yield unrun experiment in the tree, and it costs
  one command.**
- **No real docs.rs render, and no fresh-machine resolve from crates.io** — the latter impossible
  today, since nothing is published at 0.2.0.
- **ES-11 was not reproduced, and is not refuted.** 46 clean samples, 0 failures — but the shipped
  transport is the HTTP/2 arm documented at 1-in-40, and P(0 in 46) at that rate is ≈0.31. What *is*
  a finding: the 3-of-20 and 1-of-40 figures appear in six files including ADR-0061 and the
  specification, and **no committed raw log backs them anywhere**, unlike every study under
  `experiments/`.
- **`happenstance-sync` was out of scope** except where it constrains core — which it does, via the
  frozen WF clauses.

**Side effects of this review.** The benchmark harness rewrote `benchmarks/results/raw/*` and added
`benchmarks/results/history/2026-09-08-6c7a7a8-dirty.json`. The Neon endpoint was left near its
512 MB limit. Nothing else in the working tree was modified.

---

## Appendix A — method

| Stage | Agents | What it did |
| --- | --- | --- |
| Explore | 3 | Mapped the public API surface, the five adapters, and the ADR/spec/testkit/docs corpus. |
| Ground & measure | 7 | The full gate; live Postgres; live Neon ×10; wasm32/Cloudflare; the benchmark harness; supply chain; the out-of-workspace trees. |
| Find | 20 | Five per-adapter port-cost lenses; five prospective-adapter falsification lenses; three ADR cohort audits; seven cross-cutting lenses. |
| Verify | 60 | Three perspective-diverse adversarial verifiers per lens: *reproduce in the code* / *is the tree already aware* / *does it matter at 0.2.0*. Majority required. |
| Second pass | 12 | What the first pass missed on adapters, prospective adapters and ADRs, plus verification. |
| Certify | 11 | Clustering and root causes; nine per-axis verdicts; a completeness critic. |

Finders were instructed that this repository documents itself adversarially, that a finding merely
restating something the tree already says is not a finding, and that an empty result from a lens
that genuinely looked is worth more than six padded ones. Findings were capped at six per lens.

Raw material: 108 findings with `file:line` evidence, 23 themes, 23 lens summaries, and the full
verifier verdicts, retained in this session's scratchpad.

## Appendix B — findings that did not survive verification

Listed for honesty, not as findings. **One of them is true — see §7.**

- *(0/3)* PS-2's freeze condition is unsatisfiable in principle, and ADR-0060's proposed remedy
  cannot reach it — **superseded by T-16, which makes the same argument and was confirmed.**
- *(0/3)* **The semver net leaves `happenstance-postgres` and `happenstance-neon` covered by neither
  baseline — SEE §7. THIS ONE IS TRUE.**
- *(0/3)* PS-6's infallible `begin` refutes the embedded ordered-KV driver family.
- *(0/3)* `happenstance-neon` is exempted from the decorrelation argument on a false ground —
  latency is spacing, not decorrelation.
- *(1/3)* `pub mod send_shape` ships two panicking instrument types as semver-binding public API.
- *(1/3)* VT-24's 128-event batch floor exceeds the transaction-operation cap of every
  conditional-write KV store.
- *(1/3)* `StoreLimit` has no variant for a query-item refusal on the append path.
- *(1/3)* No adapter in the workspace lacks a server-side query engine — **partly carried into T-15.**
- *(1/3)* The suite has no outcome for "passed, uninformatively".
- *(1/3)* The projection port has no seam for a read model whose store has no transaction.
- *(1/3)* ADR-0001 never records that the two flavours are mutually exclusive per type.
- *(1/3)* PS-6's PROVISIONAL marker names a phase that completed without evaluating it.
- *(1/3)* ES-25 evaluates the condition over what a store *holds*; ES-10/ES-11 govern what a `read`
  *shows* — **the same mechanism as two confirmed findings; see §7 on vote noise.**
