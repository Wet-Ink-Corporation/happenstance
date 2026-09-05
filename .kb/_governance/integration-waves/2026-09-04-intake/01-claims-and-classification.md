# Wave `2026-09-04-intake` — claims and classification

Every claim the three staged files carry, labelled against the **accepted decision corpus** and
routed to the operation that absorbs it. The labels are the four the ingest defines:

- **aligns** — the claim is consistent with an accepted decision and adds no obligation to it.
- **extends** — the claim adds knowledge no accepted decision holds, without contradicting one.
- **conflicts** — the claim cannot be true at the same time as an accepted decision's body.
- **requires-new-decision** — the claim asks for a commitment nobody has made.

**Zero claims in this wave are labelled `conflicts`**, and for the first time in eight waves the
hunt for one found something that *looks* like a contradiction and is not. `kb-reference-append-
condition-experiment-001` records **`busy=0` across thirty races at sixty-four connections**
(2026-08-16). R-3 records **`busy>0` at the same contender count** (2026-09-03). Two reference
atoms about one property, disagreeing. That is not a conflict — it is two snapshots of a mutable
property, each true of its own moment, which is precisely the situation `reference/README.md`'s
dating rule exists to make legible. Neither atom is edited; both are dated; the *decision* that
rested on the earlier number is what has a question raised against it, and that question is Op 13.

Two things worth stating before the tables.

**One claim asks for a supersession and does not supply one.** The review's Q1 says in terms that
a floor for `Event::metadata` *"needs a superseding atom"* against ADR-0015. It is right that it
does. It is also a question, not an answer — nothing in the review says what the floor should be,
whether `metadata` shares `data`'s, or whether it is deliberately unbounded. KB authority rule 1
permits a supersession; it does not permit an ingest to invent one. Op 8 is an open question and
`02`, Adjudication 6 is the argument.

**The one staged file carrying a commitment in its own voice is the smallest of the three.** The
MSRV document's operative sentence contains no `must` — *"`0.2.0` requires Rust 1.97.1, and that
is a promise rather than a preference"* — and it is unambiguously a decision anyway, by
`decisions/README.md`'s own test (*"anything that commits the project"*) and by the plainer one
that a promise made to a stranger is the definition of a commitment. Wave 7 met the same shape in
PS-3's *"is not frozen"* and drew the same conclusion: **a commitment by position is still a
commitment.** The review, by contrast, carries prohibitions (`may never refute`) that are about
the *corpus* rather than the *system*, which is what routes them to `governance/` and not to
`decisions/`.

## Modal-signal summary

| File | must/shall/must-not in its own voice | What that resolves to |
| --- | --- | --- |
| `2026-09-03-pre-publication-review.md` | **none about the system.** Two prohibitions about the corpus (*"may annotate … and may never refute"*), one quasi-modal in a question (*"owe an in-process check"*), and every other `[FROZEN]` `MUST` quoted from a clause it does not amend | Three reference atoms, one new playbook, one playbook merge, one governance atom, six open questions. **Zero decision atoms** — the file's own preamble says it *"authors no atom and proposes no ADR text; the ingest wave adjudicates, and the RUNBOOK's ADR queue decides"*, and the wave honours that on all six questions |
| `msrv-becomes-a-promise-at-0-2-0.md` | two `must`, two `should`, one `may`, and the operative claim carries none of them | **One decision atom.** The `must`s are procedural (*"the wave must show zero changed bytes"*) and the `should`s belong to the consumer advice (*"pin an exact version"*). The commitment is stated as fact, which is how ADR-0036 stated its own six weeks earlier |
| `adapter-default-projection-feature-resolved.md` | one `should`, negated (*"they should not stay"*) | **One open-question resolution, and nothing else.** The `should` reports a conclusion already forced by ADR-0036 rather than making a new one, which is why this file mints no decision despite being the only one of the three that changed shipped manifests |

## `.kb/_intake/2026-09-03-pre-publication-review.md`

Twelve claims. The file is explicitly a **filter over a larger document** — *"the part of it that
is not dated — mechanisms, transferable practice, and questions that are now owed an owner"* — so
every claim below arrives already separated from the review's dated findings, and the wave does
not re-litigate that separation.

### Candidate reference atoms

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **R-1** | `cargo test --list` is byte-identical with and without `#[ignore]`, so `xtask/src/proof.rs`'s anti-vacuity check cannot see a silenced test: all 31 named proof tests can be `#[ignore = "…"]`-ed with `cargo xtask ci` running all 33 steps and exiting 0, four of nine artefacts then running zero tests. The bare spelling is caught only by `clippy::ignore_without_reason`, a **pedantic** lint the gate's documentation does not mention, and the spelling clippy's own help text recommends survives it. Generalises: any check asserting a test's *existence* as a proxy for its *execution* has this hole; assert on the run's output | **extends.** Verified where it matters: `experiments/gate-vacuity/results/raw/list-diff.txt` is **0 bytes**. `kb-decision-0010` commits the suite to mutants and three meta-tests — which *run* — and says nothing about the xtask step that asserts names. Not a conflict with it; a defect in a different instrument motivated by the same principle | 4 |
| **R-2** | Cloning an `Event` costs `t + 2` allocations through `Tags::from_pairs` (`Cow::Owned`) and 0 through `Tag::from_static` (`Cow::Borrowed`); at VT-22's 64-tag floor, 66 allocs / 2,001 B against 1 alloc / 1,536 B. The `from_static` arm is **flat in tag count** and is the arm a fixture author writes without choosing to, so a benchmark built the natural way reports the clone immaterial regardless of the truth. Also: `Bytes::clone` is a refcount bump *from the second clone onward*, so a `Bytes::from(Vec<u8>)` payload — every decoded payload's shape — allocates once more on first clone than assumed. **Correction to a reasoned figure:** `Cow<'static, str>` is **24 bytes, not 32** on `x86_64-pc-windows-msvc`/1.97.1, giving `Tag`/`EventType` 24, `Tags` 16, `Event` 104, `SequencedEvent` 144, `QueryItem` 32 | **extends**, and it is **evidence**. `reference/README.md`'s measurement case. Verified structurally at `crates/happenstance-core/src/tag.rs` — `Tags` is `Box<[Tag]>`, `Tag` is `Cow<'static, str>`. The four corpus sites stating the cost as two are named by the review and are **not** rewritten by this wave; three of the four are outside `.kb/` and the fourth is ES-17's rationale in an accepted decision | 1 |
| **R-3** | Worst per-contender wait at the shipped `CONTENDERS = 64` is **3,628 ms of 5,000, a 1.38x margin**; worst cell anywhere 3,828 ms (1.31x) at 8 cores; defensible claim **1.3x–1.4x on the plateau**. Two predicted mechanisms **refuted**: fewer cores is not safer but ~450x *worse-behaved as an instrument* (1 core → 8 ms, 8 → 3,828, 20 → 3,628) because SQLite's handler is a back-off poll needing simultaneously-runnable contenders, so constraining cores measures the safest cell; and the build profile barely enters (debug 3,628 sits inside release's 3,228/3,328 band) because the time is spent asleep. Every `wait_ms` is a **lower bound** — the counting handler resolves the same race ~2.6x faster than SQLite's own, uncalibrated, one of five ratios inverted. **Storage was never varied, and storage is what a busy handler ultimately waits on** | **extends** `kb-decision-0022`, which fixed `busy_timeout` at 5,000 ms and recorded *"which absorbed 64-way write contention with zero `SQLITE_BUSY` errors."* This is the first measurement of the **margin** that pragma buys. It does not contradict ADR-0022's body: that sentence is true of the 2026-08-16 experiment it cites, and stays true of it | 2 |
| **R-4** | `park`/`unpark` carries **one token per thread**: `crates/happenstance-testkit/src/registry.rs:338-342` parks on `Poll::Pending` with no notified flag, so a second `block_on` nested inside a rule already driven by one (`concurrency.rs:890` → `:980`) loses the wakeup. Measured deterministically over four runs — baseline completes, nested-without-collision completes, nested-with-collision **hangs past 10 s**. Durable because CF-33 `[FROZEN]` forbids the suite a watchdog, so this workspace now has **two** mechanisms producing a stopped CI job naming no rule, and `crates/happenstance-sqlite/tests/concurrency.rs:45-47` instructs the reader that a hang is evidence about the busy timeout — sound only while that is the sole candidate | **extends**, and every anchor verified verbatim. `spec/SPECIFICATION.md`:4302-4322 already names the *first* mechanism (an adapter holding an exclusive resource across its append's suspension point) and says *"There is no watchdog by design (CF-33), so this is a hung CI job naming no rule."* R-4 names the second, and it is inside the testkit rather than inside an adapter. CF-33 is quoted, not amended | 3 |

### Candidate playbook

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **citation** | `cargo xtask spec-trace` verifies that a cited location **resolves**, not that it **says what the citing sentence claims**, and `references/evaluation/review-citation-drift.md` §1 already recorded that gap recurring once. A fresh instance: `references/evaluation/phase-7-contract-defects.md:10` cites `README.md:83-85` for the lifecycle citation-repointing rule; `:83-85` is the *erratum* exception and the rule is at `:228-230`. It resolves, so nothing catches it — and because every later evaluation document copies that header, **the error propagates by imitation rather than by edit.** Transferable: an anchored citation (`path:line (anchor-text)`, the form `xtask/src/lint_constitution.rs` already enforces for `standards/rust/`) is checkable where a bare `path:line` is not; `standards/` has this, `references/evaluation/` and `spec/` do not | **extends `kb-playbook-anchoring-citations-001`, and it is the wave's clearest merge.** That atom already carries the explicit/derived split and the cost of each; what it does not carry is a *second dated instance of the defect it exists to prevent*, the propagation-by-imitation mechanism, or the statement of which corpora in this repository are anchored and which are not. Scores **74**, and the merge target is a `guideline`, not an immutable decision | 5 |

### Candidate governance atom

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **ladder** | The precedence ladder — SPECIFICATION clause > accepted ADR > constitution atom > `CLAUDE.md` > `references/evaluation/*` — with three consequences. (1) `references/evaluation/*` and `RUNBOOK.md` **may annotate a finding and may never refute one**; a review that lets them refute suppresses true findings using documents the repository declares non-binding. (2) **A refutation needs a quoted sentence answering the same question**: a filename is not a refutation, a section number is not a refutation, and absence of a citation in a ~40,000-line corpus is a search failure, not a verdict. (3) **An ADR's currency is computable, not a matter of judgement** — `git log <adr-commit>..HEAD -- <cited files>`: empty refutes; non-empty refutes only if a line at HEAD still implements it, and where that line moved or vanished the verdict is *an accepted decision may have silently drifted*, four of which surfaced under this rule. Specification clauses are exempt because `spec-trace` resolves their citations at HEAD by construction | **extends**, and the ladder's *first* two rungs are already stated — `standards/rust/README.md`:23-43, verified verbatim, with RS-01-3's worked example. What is new is the ladder applied to a **different verb**: that README tells an *author* which document wins; this tells an *auditor* what may overturn a finding, and consequences (2) and (3) appear nowhere in the corpus. The review says why it is worth an atom: *"this review had to derive it and got it wrong first"* | 6 |

**Why this is `governance` and not `decisions`, despite two prohibitions.** KB authority rule 2
sends `must`/`must not` to the decisions layer — for **architecture** commitments. `may never
refute` is a rule about how this corpus's own records are weighed against each other, which
`governance/README.md` claims in terms (*"where the corpus's rules about itself live"*) and
excludes from `decisions/` in the same breath (*"This layer governs how the record is maintained,
not what the system does"*). `02`, Adjudication 7.

### Questions now owed an owner

The review states these as *"Named, not answered"* with *"none has a number in `RUNBOOK.md`:262's
ADR queue"*, which is `open-questions/README.md`'s **deferral** bullet almost word for word. All
five become open questions; none becomes a decision.

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **Q1** | Does `Event::metadata` get a floor, share one with `data`, or is it declared deliberately unbounded? A fourth opaque payload with no `MIN_SUPPORTED_*`, no `StoreLimit` variant and therefore **no conformant refusal path** — VT-25 `[FROZEN]` forecloses `AppendError::Store`. `happenstance-cloudflare` is already carving its margin out of `data` to compensate. ADR-0015 is accepted and immutable, so any resolution needs a superseding atom | **extends `kb-decision-0015`, and does not supply the supersession it names.** Verified at `crates/happenstance-core/src/limits.rs`: four `MIN_SUPPORTED_*` constants (`EVENT_DATA_LEN`, `TAGS_PER_EVENT`, `QUERY_ITEMS`, `EVENTS_PER_BATCH`) and three `StoreLimit` variants (`EventDataLen`, `TagsPerEvent`, `EventsPerBatch`). `metadata` appears in neither set, and the string does not appear in ADR-0015's atom at all. **Scores 65 against an accepted atom and receives no operation on it** | 8 |
| **Q2** | Is a read page budgeted in rows, in bytes, or by the caller? One page of 512 ceiling-sized events is **512.2 MiB resident**; raising `PAGE_SIZE` is the measured win for aggregate lock time (**2,929 s → 99 s** over a 10⁶ replay) and is exactly what the residency measurement says is unsafe. One knob, two consequences, not co-optimisable | **extends.** `PAGE_SIZE` appears **nowhere in `.kb/`** before this wave. `kb-decision-0011` decided what `read` promises — one sample, a ceiling, `&Query`, an inclusive `to` and a `limit` — and never what a page costs. The two numbers pull in opposite directions on one constant, which is a fork with both tines named and neither chosen: the exact condition wave 6 used to refuse minting a decision | 9 |
| **Q3** | Does the testkit grow a fixture-declared tolerance for transient contention? **A busy store and a broken store are currently the same `Attempt` outcome**, and CF-33 `[FROZEN]` guarantees the suite cannot tell them apart. ADR-0022 §12 says the contender count is not its call | **extends**, and `kb-decision-0034` is the atom that says *how* it would be answered rather than *whether* it should be: *"a CF- clause is minted by the decision that first needs the capability."* Scores 58 against it and receives no operation. `kb-open-question-poll-count-rule-strength-001` is a sibling capability question, not this one | 10 |
| **Q4** | Does an adapter taking VT-6's documented-procedure branch owe an in-process check? `remint_identity`'s precondition is **trust-only even for the case a program can see** | **extends `kb-decision-0014`, and it is a gap inside a branch that decision granted.** Verified: the atom's summary reads *"An adapter may mint a store id once and re-mint only if it can detect a restore or clone, **or if the deployment documents when re-minting is invoked**"* — the branch exists, and nothing says whether a program must verify what a document asserts. `SqliteEventStore::remint_identity` exists at `event_store.rs`:448 and is exercised only by `tests/migration.rs`:421 | 11 |
| **Q5** | Does the query plan chunk on bound parameters as well as UNION arms? `PARAMETER_BUDGET` exists, carries the arithmetic, and **has one caller where it should have three** | **extends**, and both halves verified. `crates/happenstance-sqlite/src/event_store.rs`:577 declares `PARAMETER_BUDGET = 30_000`; :760 is its only use, sizing the tag-row insert. The **second axis is real and independently implemented**: :266-290 and :622-664 chunk `UNION` arms through `query_sql::chunks` at a width of 400, *"chunked and merged, never refused"*. Two chunking axes, one budget, applied to one of them | 12 |

### Falsifiers that fired

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **falsifiers** | Three of ADR-0022's own named falsifiers, checked. **§11 — *"re-open if any run ever reports `busy > 0`"* — FIRED:** `busy > 0` observed at the shipped `CONTENDERS = 64`, one launch in seven, the first nonzero busy count anywhere in this tree, and noted precisely as `busy=1`/`busy=2` with `exhausted=0` — attempts that *entered* the handler, not attempts that ran the budget out. **§9 — *"re-open if a deployment shows the captured `Handle` costing something the inline path does not"*:** the capture is unconditional and irreversible, making `NoRuntime` unreachable for a store that outlives its construction runtime. **§16's falsifier for §8 cannot fire as written**, because the shape it names to re-open on — the `GROUP BY … HAVING COUNT(DISTINCT tag)` aggregate — is not what the adapter emits; measured, the chain that does ship loses to that aggregate in **9 of 9** two-tag cells | **extends `kb-decision-0022` and may not be written into it.** The falsifiers are quoted verbatim from an accepted decision; two have fired and one is inapplicable as written; **nobody has re-opened.** Scores 68 against the decision and receives no operation on it — the single most important application of wave 4's rule in this wave, because the thing being suppressed would be *the decision's own promise to re-open*. `kb-open-question-es-17-two-adapter-measurement-001` is the shape precedent: an ADR-0022 falsifier whose evidence is owed and whose owner is nobody | 13 |

## `.kb/_intake/msrv-becomes-a-promise-at-0-2-0.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **c1** | **`0.2.0` requires Rust 1.97.1, and that is a promise rather than a preference.** The number is unchanged; its **standing** changes. Forced by `rusqlite 0.40` → `libsqlite3-sys 0.38.1`'s build script invoking `cfg_select!` (needs 1.88), with the floor deliberately set higher and tied to the dev/CI toolchain because five database crates declare no `rust-version` at all and the exact minimum would have to be bisected against a build script. An MSRV increase is a **minor** version bump named in `CHANGELOG.md`; a patch release cannot move it, and cargo's pre-1.0 semantics mean a floor rise cannot reach a consumer without a version bump they chose. The `msrv` CI job is **currently vacuous** and starts proving something only when the pin and the floor diverge; `cargo hack --rust-version` reads declared metadata and the five crates declare none, so only running the compiler finds those — which is what the job's second command does. Four alternatives rejected: promise only "latest stable"; pin `rusqlite` back to 0.37; a per-package `rust-version`; a moving N-2-stable window | **requires-new-decision**, and it arrives **taken**, with four losers named and the per-package alternative's real cost conceded rather than denied. Every anchor verified: `Cargo.toml`:15-17, `rust-toolchain.toml`, `.github/workflows/ci.yml`:307-321 with the divergence comment already spelled out. **Two framing corrections apply** — the four crates are phase 12's release set and `PUBLISHABLE` holds five, and the promise is made *ahead of* a release RUNBOOK.md:163 records as `not started`. `00`, Corrections 1 and 2 | 7 |
| **c2** | *"This is an amendment lineage, not a supersession."* ADR-0004 and ADR-0029 stay `status: accepted` and byte-identical, frontmatter included; neither gets a `superseded_by` flip. They said what the number is and why it moved; the new atom says what it now **binds** | **aligns** with `.kb/decisions/README.md`:9-13 and with `kb-decision-0029`'s own execution of the same shape against ADR-0004. **No operation** — a README is not a merge target, and neither are the two accepted atoms it protects. Realised as **authoring constraints on Op 7's frontmatter** (`depends_on: [kb-decision-0004, kb-decision-0029]`, `supersedes: null`) and as an absence: two accepted bodies untouched, which `redkiln validate --kb` checks against `HEAD` | 7 |
| **c3** | What this does not decide: the **number** (1.97.1 is unchanged; the atom converts its status, not its value); the **shipped wording** at `crates/happenstance/README.md`:49 — *"MSRV 1.97.1, checked in CI"* — which claims more than the atom supports and whose correction belongs to `guarantees-and-docs-rs-presentation`; and **ADR-0004 and ADR-0029**, untouched and checked | **aligns**, and it is the `Consequences`/scope-fence a decision atom owes. Verified: `crates/happenstance/README.md`:49 reads exactly that. The README repair is a **task** — `open-questions/README.md` excludes *"work someone is expected to do"* in terms — so it gets no atom of any kind and stays a backlog row. Folded into Op 7 as its closing section | 7 |

**All three claims are one atom.** c2 is frontmatter and c3 is a scope fence; neither states
knowledge that survives being detached from c1. Splitting them would produce two files whose
entire content is *what the third file is not*.

## `.kb/_intake/adapter-default-projection-feature-resolved.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **1** | **No — they should not stay.** Both crates now read `happenstance-core = { workspace = true, features = ["std"] }`, `default = ["event-store"]`, `projection-store = ["happenstance-core/unstable-projection"]` — the shape `happenstance-sqlite` took on 2026-09-02. Verified: all four feature combinations compile for each crate, eight checks | **aligns** with `kb-decision-0036`, and it is **application rather than commitment**: ADR-0036 already ships `ProjectionStore` behind an off-by-default gate, so making two adapters honour it adds no obligation. **Confirmed live against this worktree** — both manifests read exactly that, verbatim. This is the first staged claim in this corpus's history to be checked against the same two manifests that produced a *correction* one wave earlier and to come back clean | 14 |
| **2** | The open question's central observation held up **exactly as written**: changing `default` alone closes nothing, because `--no-default-features` still enables the core gate through the unconditional dependency-table entry. `happenstance-sqlite` already forwarded the flag through its own feature, so trimming `default` sufficed there; these two named the capability without gating it, so trimming `default` alone would have produced *"a crate that looks gated and is not"* — worse than the state it started in, because it would read as fixed. Both halves had to move together | **aligns**, and it is the **retrospective confirmation of a prospective diagnosis**. The open question's *"Why the exposure is not the sqlite exposure, and runs the wrong way"* section stated this before the fix existed. Nothing new to place: it is the resolution's own narrative and belongs in the dated section Op 14 appends | 14 |
| **3** | What this does not change: neither crate is published and both remain `publish = false` skeletons with `todo!()` projection bodies, so **the exposure closed was latent, not live**; `spec/SPECIFICATION.md`'s impl census is untouched and is not restated; **PS-2's bar is unaffected** and ADR-0036 stands exactly as written, its `[PROVISIONAL]` marker on PS-3 unmoved | **aligns.** Verified: `publish = false` on both. Realised as what the wave does **not** do — no `spec/SPECIFICATION.md` edit, no marker moved, no clause amended, and no operation of any kind on `kb-decision-0036`. Wave 7's ps-3 C4 in the same position | 14 |
| **4** | Why it was resolved here rather than by the owning project: the open question named `postgres-and-neon-stores` as owner and *"before either crate is published"* as the deadline, and that project's stories are all at `plan`. Three manifest lines per crate, no Rust touched, mechanical verification — *"leaving a known walk-around in place until a project starts is how a default nobody re-read becomes a consumer's problem"*, and the owning project inherits it as **done rather than as owed** | **aligns**, and it is the claim that licenses closing the question **now**. It does not contradict the open question's ownership statement — it discharges it early, which is what `open-questions/README.md`'s resolution procedure contemplates and what wave 6 did for `cf-40` | 14 |
| **5** | *(inferred, not stated in the intake body)* The open-questions index and the domain map both list `kb-open-question-adapter-default-projection-feature-001` as **Open**, with descriptions ending *"`happenstance-neon` and `happenstance-postgres` have not followed"* and *"still carry `default = ["event-store", "projection-store"]`"*. Both entries are now stale | **extends**, and it is **`mapsImpact`, not an operation.** Verified at `.kb/maps/open-questions-index.md`:288-292 and `.kb/maps/domain-map.md`:245-248, both of which now describe manifests that no longer exist. Recorded on Op 14 rather than minted as a fourteenth-and-a-half operation: the Maps phase owns map edits, and inventing an op for one would be the only op in eight waves whose destination is a map and whose source is an inference | 14 |

## Classification roll-up

| Label | Claims | Where they land |
| --- | --- | --- |
| **aligns** | 8 | Ops 7 and 14 — **plus one claim that correctly produces no operation at all** (MSRV c2, the non-supersession, realised as an absence and two `depends_on` edges) |
| **extends** | 11 | Ops 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13 |
| **conflicts** | **0** | — (two reference atoms will disagree about `busy` at 64 connections, and both are dated and both are true; `01`'s opening) |
| **requires-new-decision** | 1 | MSRV c1 → Op 7 |

**One `requires-new-decision` claim, and it arrives taken, which is why it becomes a decision atom
rather than an open question.** This is the discriminator wave 6 drew and wave 7 applied five
times: a claim that hands the wave a fork with both tines named and neither chosen becomes an open
question; a claim that arrives with its losers named, its costs conceded and its evidence attached
becomes a decision. The MSRV file names four alternatives and rejects each with a reason, concedes
that the per-package option *"loses on verification capacity, not on merit"*, and discloses that
the CI job backing the promise is currently vacuous. Nothing about it is a fork.

**The six claims that do hand over forks all become open questions, and the review says so first.**
Its own preamble — *"It authors no atom and proposes no ADR text; the ingest wave adjudicates, and
the RUNBOOK's ADR queue decides"* — is the file declining to sign six decisions it had the evidence
to argue for, and this wave declines them in the same place. That is six new open questions in one
wave, the layer's largest single intake, and it is the correct outcome rather than an abdication:
a review that surfaces six unowned forks and a wave that mints six answers to them would be one
document's opinion promoted to the immutable layer without a human between.
