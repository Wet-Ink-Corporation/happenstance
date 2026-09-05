# Wave `2026-09-04-intake` — placement and adjudication

Fourteen operations, ordered so that every atom a later operation links to exists before the link
is written. Three staged files, twenty claims, **zero cross-file merges** — the dedup this wave
performs is intra-document and against the corpus, and `00-corpus-match.md` establishes why.

**What this wave does to the accepted decision corpus: nothing.** One decision atom is added.
No accepted body is edited, no `status` is flipped on any decision, and no supersession is signed
— third wave running. Two of ADR-0022's own falsifiers have fired and the firing is recorded
*outside* it (Op 13); ADR-0015 is named as needing a superseding atom and does not get one
(Op 8); ADR-0004 and ADR-0029 are amended from outside by a lineage they invented (Op 7).

| # | Op | Destination | Kind | Classification | Sources |
| --- | --- | --- | --- | --- | --- |
| 1 | `create_new` | `.kb/reference/event-clone-allocations-and-layout-2026-09.md` | reference | extends | review |
| 2 | `create_new` | `.kb/reference/busy-timeout-margin-2026-09.md` | reference | extends | review |
| 3 | `create_new` | `.kb/reference/nested-block-on-lost-wakeup-2026-09.md` | reference | extends | review |
| 4 | `create_new` | `.kb/playbooks/assert-a-tests-execution-not-its-discovery.md` | playbook | extends | review |
| 5 | **`merge_existing`** | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | playbook | extends | review |
| 6 | `create_new` | `.kb/governance/what-may-refute-a-finding.md` | governance | extends | review |
| 7 | `create_new` | `.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md` | decision | requires-new-decision | msrv |
| 8 | `defer_open_question` | `.kb/open-questions/event-metadata-has-no-declared-floor.md` | open_question | extends | review |
| 9 | `defer_open_question` | `.kb/open-questions/read-page-budget-is-unspecified.md` | open_question | extends | review |
| 10 | `defer_open_question` | `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md` | open_question | extends | review |
| 11 | `defer_open_question` | `.kb/open-questions/remint-identity-precondition-is-trust-only.md` | open_question | extends | review |
| 12 | `defer_open_question` | `.kb/open-questions/query-plan-parameter-chunking-incomplete.md` | open_question | extends | review |
| 13 | `defer_open_question` | `.kb/open-questions/adr-0022-falsifiers-have-fired.md` | open_question | extends | review |
| 14 | **`merge_existing`** | `.kb/open-questions/projection-store-in-adapter-default-features.md` | open_question | aligns | adapter-default |

`review` = `.kb/_intake/2026-09-03-pre-publication-review.md` · `msrv` =
`.kb/_intake/msrv-becomes-a-promise-at-0-2-0.md` · `adapter-default` =
`.kb/_intake/adapter-default-projection-feature-resolved.md`.

**Ordering rationale.** References first: Op 2 is linked by Op 13 and Op 10, and Op 3 by Op 10,
so the measurements exist before the questions that cite them. Ops 4–6 are the transferable and
governing material, independent of everything after them. Op 7 is the wave's only decision and is
placed before the open questions because Op 8 and Op 13 both describe what a superseding decision
would have to look like, and a reader arriving from the decision map should meet the atom that
*was* signed before the six that were not. Ops 8–13 create; Op 14 mutates. The Maps phase runs
after all fourteen.

---

## Op 1 — the `Event` clone cost and the layout correction

**Disposition:** `create_new` · **Destination:** `.kb/reference/event-clone-allocations-and-layout-2026-09.md`

```yaml
id: kb-reference-event-clone-allocations-001
title: Cloning an Event costs t + 2 allocations, and Cow<'static, str> is 24 bytes
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-12 measurement taken 2026-09-03 from experiments/event-clone-allocations/, on
  x86_64-pc-windows-msvc at rustc 1.97.1, of what it costs to clone an Event and of what the
  contract's small types actually weigh. Tags is Box<[Tag]> and Tag is Cow<'static, str>, so the
  cost depends entirely on which constructor built the tags: Tags::from_pairs goes through
  Tag::key_value to Tag::new and yields Cow::Owned, which clones by allocating, while
  Tag::from_static yields Cow::Borrowed, which clones free. At VT-22's 64-tag floor with a
  Bytes::from_static payload, from_pairs costs 66 allocations and 2,001 bytes against
  from_static's 1 allocation and 1,536 bytes; at 8 tags, 10 against 1; at 0 tags, 1 against 0.
  The from_static arm is flat in tag count and is the arm a fixture author writes without
  choosing to, so a benchmark built the natural way reports the clone immaterial whatever the
  truth is. Separately, Bytes::clone is a refcount bump only from the second clone onward, so a
  Bytes::from(Vec<u8>) payload - the shape every decoded payload has - allocates a shared header
  on its first clone, one extra allocation per payload for any store that clones each appended
  event once. And a correction to a figure that was reasoned rather than measured: Cow<'static,
  str> is 24 bytes and not 32, because String's capacity field carries the discriminant, giving
  Tag and EventType 24, Tags 16, Event 104, SequencedEvent 144 and QueryItem 32. A layout budget
  written from the derived numbers would have failed on its first run. Four sites in the corpus
  state the clone cost as two, one of them ES-17's own rationale; this atom records the
  measurement and rewrites none of them.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0015
  - kb-reference-wire-format-measurements-001
  - kb-open-question-es-17-two-adapter-measurement-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/event-clone-allocations/
  - crates/happenstance-core/src/tag.rs
  - crates/happenstance-core/src/event.rs
last_reviewed: 2026-09-04
```

**Why here, and why not folded.** `reference/README.md`'s first case, verbatim: *"a measurement —
what was run, on what, on what date, and what it returned."* The three parts — the allocation
table, the `Bytes::clone` refcount observation, and the layout correction — are one atom because
they are one experiment about one family of types, and because the third is what makes the first
two trustworthy: a document that measured allocations and inherited a wrong size for the type
being allocated would be reporting a number it did not check.

**The correction is the reason this cannot be a footnote in a decision.** `Cow<'static, str>` at
24 rather than 32 bytes is a fact about a compiler and a target, and it is dated for that reason.
The `related` edge to `kb-decision-0015` is the one that matters: that decision moved `EventType`
and `Tag` from `Box<str>` to `Cow<'static, str>` and added `from_static`, and this atom is the
first measurement of what that move costs on each arm. It does not amend it, and ADR-0015 is
accepted regardless.

**What the atom must not do.** Rewrite the four corpus sites that state the clone cost as two.
One of them is ES-17's rationale inside an accepted decision, three are outside `.kb/`, and
`kb-open-question-es-17-two-adapter-measurement-001` already owns the ES-17 thread. The atom
records the number and names the disagreement; the repair is not an ingest's.

**`mapsImpact`:** `domainMap` — the reference sits in the contract-ports domain.

---

## Op 2 — the busy-timeout margin, and the two mechanisms it refutes

**Disposition:** `create_new` · **Destination:** `.kb/reference/busy-timeout-margin-2026-09.md`

```yaml
id: kb-reference-busy-timeout-margin-001
title: The busy-timeout margin at 64 contenders is 1.3x to 1.4x, and fewer cores is not safer
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-12 measurement taken 2026-09-03 from experiments/busy-timeout-margin/, of how much of
  ADR-0022's fixed 5,000 ms busy_timeout the shipped conformance configuration actually consumes.
  At the shipped CONTENDERS = 64 the worst per-contender wait is 3,628 ms of the 5,000 ms budget,
  a margin of 1.38x; the worst cell anywhere in the matrix is 3,828 ms at 8 cores, 1.31x. The
  defensible claim is 1.3x to 1.4x on the plateau, and not more, because two mechanisms a
  reasonable person would predict are refuted by the matrix. Fewer cores is not safer: 1 core
  gives 8 ms, 2 gives 628, 4 gives 2,628, 8 gives 3,828 and 20 gives 3,628, a roughly 450-fold
  swing, because SQLite's busy handler is a back-off poll rather than a queue and the pathology
  needs contenders that are simultaneously runnable - so constraining cores measures the safest
  cell in the matrix rather than a conservative one. And the build profile barely enters: debug's
  3,628 ms sits inside release's 3,228 and 3,328 ms band, because the time is spent asleep and
  --release has nothing to recover. Every wait_ms is a lower bound: the counting handler the
  experiment installs resolves the same race about 2.6x faster than SQLite's own, uncalibrated,
  with one of five ratios inverted. Storage was never varied, and storage is what a busy handler
  ultimately waits on. Conditions, because none of it is portable: i9-13905H (14c/20t), Windows
  11, NTFS/NVMe, rustc 1.97.1 msvc, SQLite 3.53.2, wal plus synchronous=normal read back off the
  live connection, on a host running about twenty other agent processes. This measurement records
  busy > 0 at 64 contenders; kb-reference-append-condition-experiment-001 recorded busy = 0 at the
  same count on 2026-08-16, and both are true of their own moment.
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-append-condition-experiment-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-nested-block-on-lost-wakeup-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-04
```

**Why a second reference atom rather than an extension of the first.** Scored **72** against
`kb-reference-append-condition-experiment-001` — same adapter, same pragma, same contender count
— and refused on `reference/README.md`'s dating rule, which the two atoms now demonstrate rather
than merely obey. The August atom says *"sixty-four `rusqlite` connections opened on one file
across thirty races each recorded `busy=0`."* This one says `busy>0` at the same count. **Merging
would overwrite a snapshot with a later one and destroy the only evidence that the property
changed** — and the change is precisely what Op 13 exists to report. Two dated atoms disagreeing
is the layer working; one atom silently updated is the failure mode the README names (*"a mirror
nobody maintains is worse than no mirror because it looks maintained"*).

**Why the refutations belong in the atom and not in a playbook.** *"Constraining cores measures
the safest cell"* is close to a transferable technique and is not one: it is a property of
SQLite's back-off handler, not of measurement in general, and it stops holding the moment the
contended resource has a queue rather than a poll. `playbooks/README.md` excludes the one-off in
terms. The generalisable half — *state the conditions, because none of this is portable* — is
already the reference layer's dating rule.

**`mapsImpact`:** `domainMap` — the SQLite-adapter domain.

---

## Op 3 — the nested `block_on` that loses a wakeup

**Disposition:** `create_new` · **Destination:** `.kb/reference/nested-block-on-lost-wakeup-2026-09.md`

```yaml
id: kb-reference-nested-block-on-lost-wakeup-001
title: park carries one token per thread, so a nested block_on in the testkit loses its wakeup
kind: reference
status: accepted
authority_tier: note
summary: >-
  The deterministic reproduction taken 2026-09-03 in experiments/busy-timeout-margin/tests/
  lost_wakeup.rs of a second way a conformance run can stop and name no rule. The testkit's own
  executor, crates/happenstance-testkit/src/registry.rs:338-342, drives a rule by polling and
  calling std::thread::park on Poll::Pending, with no notified flag of its own. std's park and
  unpark carry a single token per thread, so an unpark delivered while the thread is not parked
  is coalesced rather than queued: a second block_on nested inside a rule already driven by one -
  the shape at crates/happenstance-testkit/src/concurrency.rs:890 calling through to :980 - can
  consume the token the outer loop was waiting for. Measured over four runs: the baseline
  completes, the nested case without a collision completes, and the nested case with a collision
  hangs past ten seconds. This matters beyond the one call site because CF-33 is [FROZEN] and
  forbids a conformance rule a clock, a watchdog or an elapsed-time assertion, so a hang produces
  a stopped CI job that names no rule at all. spec/SPECIFICATION.md:4302-4322 already records one
  mechanism with that signature - an adapter holding an exclusive resource across its append's
  suspension point, where the read blocks on what the suspended append still holds. This is a
  second, and it lives in the suite rather than in an adapter. crates/happenstance-sqlite/tests/
  concurrency.rs:45-47 currently instructs the reader that a hang is evidence about ADR-0022's
  busy-timeout paragraph and should be escalated there, which was sound while that was the sole
  candidate and is a dated statement now.
depends_on: []
related:
  - kb-decision-0010
  - kb-decision-0022
  - kb-playbook-cold-future-hand-polling-001
  - kb-reference-busy-timeout-margin-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/tests/lost_wakeup.rs
  - crates/happenstance-testkit/src/registry.rs
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-sqlite/tests/concurrency.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
```

**Adjudication 3 — an experiment directory is not a subject.** This finding and Op 2 were
measured in the same crate; `lost_wakeup.rs` is a test file inside
`experiments/busy-timeout-margin/`. Scored **40** and split, on
`kb-playbook-one-decision-per-adr-title-001`'s deletion test, which passes in both directions:
delete the margin table and the lost wakeup is a complete finding about an executor; delete the
lost wakeup and the margin table is a complete finding about a pragma. The deciding argument is
the reader. Op 2's reader is weighing whether 5,000 ms is enough; Op 3's reader is looking at a
CI job that stopped and is deciding what to suspect. Filed together, the second reader has to
read a matrix of core counts before reaching the paragraph that helps them.

**Why `reference` and not `playbook`.** The `park`-carries-one-token property is a fact about
`std`, and the finding is about two named functions in one crate. `playbooks/README.md` excludes
*"something true of exactly one file… either a `reference` atom (if it records a state worth
citing) or nothing at all."* The transferable half — *do not nest an executor inside itself* —
is not worth a playbook on one instance.

**What the atom must not do.** Assert that `concurrency.rs:45-47`'s comment is wrong. It is not;
it is dated. The atom says what changed and leaves the comment to whoever next edits that file,
the same restraint Op 5 applies to the mis-anchored citation it records.

**`mapsImpact`:** `domainMap` — the conformance-suite domain.

---

## Op 4 — assert a test's execution, not its discovery

**Disposition:** `create_new` · **Destination:** `.kb/playbooks/assert-a-tests-execution-not-its-discovery.md`

```yaml
id: kb-playbook-assert-execution-not-discovery-001
title: A gate that asserts a test exists has not asserted that it ran
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  An anti-vacuity check built on test discovery cannot see a silenced test, and the fix is to
  assert on the run's own output instead. Measured 2026-09-03 in experiments/gate-vacuity/:
  libtest's cargo test --list output is byte-identical with and without #[ignore] - the recorded
  diff is an empty file - so xtask/src/proof.rs, which rests its argument on the names being
  asserted out of --list before the tests run, cannot distinguish a suite that runs from one that
  does not. All thirty-one named proof tests were annotated #[ignore = "..."] and cargo xtask ci
  ran all thirty-three steps and exited 0, with four of nine artefacts then running zero tests.
  The bare #[ignore] spelling is caught, but by clippy::ignore_without_reason, a pedantic lint
  nothing in the gate's documentation mentions - and the spelling clippy's own help text
  recommends is exactly the one that survives it, so the lint that appears to close the hole
  instructs an author through it. The repair is to compare the count or the names in the run's
  result output against the expected set, not the discovery listing. The general shape: any check
  that asserts a test's existence as a proxy for its execution has this hole, and the two are
  different program behaviours rather than two ends of one reference - which is what separates
  this from a coverage-reporting problem. Where it stops holding: a harness whose discovery and
  execution listings are the same artefact has nothing to compare, and a suite small enough to
  read has no need of the check.
depends_on: []
related:
  - kb-decision-0010
  - kb-playbook-verify-referent-report-coverage-001
  - kb-decision-0037
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/gate-vacuity/
  - xtask/src/proof.rs
last_reviewed: 2026-09-04
```

**Adjudication 2 — why this is not a merge into `kb-playbook-verify-referent-report-coverage-001`,
for the fifth wave running.** Scored **58**, inside the band, and the discriminator is that
playbook's own generalisation section, which enumerates the population it governs: *"citations,
links, ids, symbol references, schema `$ref`s, test-to-requirement traceability."* Every member is
a **cross-reference** — an address and a referent. A test suite is not one: `--list` and the run
are two behaviours of one program, not two ends of one pointer. The fixes differ accordingly.
That playbook's is *print the denominator* — make the scope legible. This one's is *change what
you assert on* — the denominator is already right, and every name in it is present. Merging would
force one atom to carry two mechanisms under a title naming one, which
`kb-playbook-one-decision-per-adr-title-001` rejects.

**Why this is a playbook rather than a reference.** It has a stated claim, a measurement, a
named repair, and a boundary — `playbooks/README.md`'s method case in full. The measurement that
grounds it is small enough to sit inline (an empty diff file and one exit code), so no companion
reference atom is minted; `00`'s refused-dedup rule for evidence splitting fires on tables of
numbers, and this has none.

**The `related` edge to Op 7 is deliberate and is the near-miss `00` records at 52.** ADR-0037
discloses that the `msrv` CI job is currently vacuous. That is the same *shape* — a green step
proving less than its name — and it is a **disclosure**, not a defect: ADR-0029 recorded it,
ADR-0037 restates it, and the condition under which it stops being vacuous is written down.
Folding them would make this playbook assert that an honest disclosure is a bug. A link says the
true thing: the shape recurs, and one instance was found by a review while the other was declared
by its own author.

**What the atom must not do.** Prescribe the repair to `xtask/src/proof.rs`. That is a task, it
belongs in `.bklg/`, and `playbooks/README.md` asks for the method and its boundary, not a work
order.

**`mapsImpact`:** `domainMap` — the gate-and-tooling domain.

---

## Op 5 — the citation that resolves and does not say what is claimed

**Disposition:** `merge_existing` · **Destination:** `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` (`kb-playbook-anchoring-citations-001`) · **Score:** 74

**What changes.** A new dated subsection, placed in the existing *"The cost, and where it stops
holding"* neighbourhood, carrying three things the atom does not hold:

1. **A second instance of the defect the playbook exists to prevent**, dated 2026-09-03.
   `references/evaluation/phase-7-contract-defects.md:10` cites `README.md:83-85` for the
   lifecycle's citation-repointing rule; `:83-85` is the *erratum exception* and the rule is at
   `:228-230`. It resolves, so `spec-trace` passes it.
2. **The propagation mechanism, which is new knowledge.** Every later evaluation document copies
   that header, so the error spreads **by imitation rather than by edit** — a class no
   line-checking tool can catch, because each copy is independently well-formed.
3. **The coverage statement.** `standards/rust/` carries explicit anchors and
   `xtask/src/lint_constitution.rs` enforces them; `references/evaluation/` and `spec/` do not.
   The playbook already priced the explicit spelling as *"an expensive retrofit onto [a corpus]
   that already exists"* — this names which corpora are on which side of that price today.

Frontmatter deltas: `source_paths` gains
`.kb/_intake/2026-09-03-pre-publication-review.md`,
`references/evaluation/review-pre-publication-2026-09-03.md` and
`references/evaluation/phase-7-contract-defects.md`; `last_reviewed` moves to `2026-09-04`;
`related` gains `kb-playbook-assert-execution-not-discovery-001`. **`id`, `title`, `kind`,
`status`, `authority_tier` and `depends_on` are unchanged**, and no existing sentence is reworded.

**Adjudication 5 — the corpus's first playbook merge, and the rule that licenses it.** Six waves
have refused to merge one playbook into another, under the standing rule that *a second instance
of a shape is a link, not a merge*. This is not that, and the distinction is worth stating because
it will be reached for again.

- **A second instance of a *shape*** is what `kb-playbook-declared-page-need-001` was against
  `kb-playbook-verify-referent-report-coverage-001`: two different subjects (documentation
  structure; cross-reference checking) exhibiting one abstract pattern. The right response is a
  `depends_on` edge, because merging them would produce an atom whose title names one subject and
  whose body covers two.
- **A second instance of *the defect a specific playbook exists to prevent*** is this. The
  merge target's entire subject is `file:line` citations drifting under a moving tree in *this*
  repository, and the incoming claim is one such citation, in this repository, drifting. There is
  no second subject to smuggle in. An atom that records how to catch citation drift and declines
  to record a fresh instance of citation drift is an atom that will be read as *"this was solved"*.

The three additions all sit inside the existing title. Nothing about the merge widens it.

**Why the mis-citation itself is not repaired here.** The source names the repair as *"the one
permitted in-place change the sentence itself describes"* — a lifecycle rule in a tree this wave
does not write to. The KB records the instance; the edit belongs to whoever next touches
`references/evaluation/`.

**`mapsImpact`:** `domainMap` — the domain-map's documentation-and-citations entry gains the
instance date. No decision-map or open-question-index change.

---

## Op 6 — what may refute a finding

**Disposition:** `create_new` · **Destination:** `.kb/governance/what-may-refute-a-finding.md`

```yaml
id: kb-governance-what-may-refute-a-finding-001
title: What may refute a finding, and how an accepted decision's currency is computed
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  The precedence ladder this repository already declares - SPECIFICATION clause above accepted
  ADR above constitution atom above CLAUDE.md above references/evaluation/* - applied to a verb
  it was not written for. standards/rust/README.md:23-43 tells an author which document wins when
  two disagree; this atom tells an auditor what may overturn a finding, and three consequences
  follow that the ladder alone does not give. First, references/evaluation/* and RUNBOOK.md may
  annotate a finding and may never refute one: they are dated evidence rather than rules, and a
  review that lets them refute suppresses true findings using documents the repository itself
  declares non-binding - which is RS-01-3's rule read from the auditing end. Second, a refutation
  needs a quoted sentence answering the same question. A filename is not a refutation and a
  section number is not a refutation, and the absence of a citation in a corpus of roughly forty
  thousand lines is a search failure rather than a verdict. Third, an accepted decision's currency
  is computable rather than a matter of judgement: git log <adr-commit>..HEAD -- <the files the
  decision cites> returns empty when the decision was taken against exactly this code, and it
  refutes; when it returns commits, the decision refutes only if a line at HEAD still implements
  what it decided, and where that line has moved or vanished the correct verdict is that an
  accepted decision may have silently drifted - the highest-value class available and the one a
  naive "the ADR covers this" reading discards. Four candidates surfaced under that rule in the
  2026-09-03 pre-publication review. Specification clauses are exempt from the computation
  because cargo xtask spec-trace is a gate step, so their citations resolve at HEAD by
  construction.
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-verify-referent-report-coverage-001
  - kb-decision-0022
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - standards/rust/README.md
  - standards/rust/01-standard-of-evidence.md
last_reviewed: 2026-09-04
```

**Adjudication 7 — two prohibitions, and why they do not go to `decisions/`.** KB authority rule
2 routes `must`/`must not` to the decisions layer, and *"may never refute"* is prohibition-strength.
It is nonetheless not a decision atom, because `governance/README.md` draws the line by subject
rather than by modal: *"An architecture commitment. Must, shall, must not about **the system
itself** is a `decision` atom… This layer governs how the record is maintained, not what the
system does."* Nothing here constrains an adapter, a port, a wire format or a crate surface. It
constrains how an auditor may weigh two of this repository's own documents against each other,
which is the layer's second bullet — *"a rule about who may accept, supersede, or retire an atom,
and under what authority"* — read one notch wider.

**Why it is not a restatement of `standards/rust/README.md`.** The ladder is there; consequences
(2) and (3) are not, and consequence (3) is the atom's centre of gravity. *An ADR's currency is
computable* converts a judgement call into a command with three defined outcomes, one of which —
*an accepted decision may have silently drifted* — is a verdict the corpus had no name for. The
review says why it is owed an atom rather than left in a document: *"this review had to derive it
and got it wrong first"*, which is `governance/README.md`'s stated reason for existing (*"A rule
that is worked out in practice and never written down gets rediscovered, at the same cost"*).

**Why not merged into `kb-governance-referent-not-reasoning-001`.** Scored 35. That atom decides
whether a given edit to a record may be made; this one decides whether a claim about a record may
be overturned. One layer, two verbs, no shared test.

**What the atom must not do.** Name the four silent-drift candidates the review surfaced. They
are findings against specific decisions, they are dated, and the four belong in the review
document and the backlog rather than in a governance rule — an atom that carries a rule *and* its
current findings has a body that expires.

**`mapsImpact`:** `domainMap` — the governance domain gains its second atom.

---

## Op 7 — the MSRV becomes a promise at `0.2.0`

**Disposition:** `create_new` · **Destination:** `.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md`

```yaml
id: kb-decision-0037
title: The MSRV becomes a promise at 0.2.0, and the number does not move
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0037
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  0.2.0 requires Rust 1.97.1, and that is a promise rather than a preference. The number is
  unchanged from ADR-0029 and its standing is not: until this release the floor was a self-imposed
  constraint raisable at zero cost because nothing was published, and 0.2.0 puts happenstance-core,
  happenstance, happenstance-testkit and happenstance-sqlite on a registry, from which moment the
  number binds someone who may never build the crate that forced it. This is an amendment lineage
  rather than a supersession: ADR-0004 and ADR-0029 stay accepted and byte-identical, and neither
  gets a superseded_by flip, on the precedent ADR-0029 itself set against ADR-0004. The floor was
  forced by a dependency's build script rather than by this workspace's code - rusqlite 0.40 pulls
  libsqlite3-sys 0.38.1, whose build script invokes cfg_select!, unavailable before 1.88 - and is
  deliberately set higher than that evidence requires, tied to the toolchain the project is
  developed and tested on, because rusqlite, libsqlite3-sys, sqlx, sqlx-core and sqlx-postgres
  declare no rust-version at all and the exact minimum would have to be bisected against a build
  script that the next silent dependency would invalidate again. An MSRV increase is a minor
  version bump named in CHANGELOG.md; a patch release cannot move it, and below 1.0 cargo already
  treats a minor bump as incompatible, so a floor rise cannot reach a consumer without a version
  change they chose. The promise is stated no larger than its evidence: the msrv CI job installs
  1.97.1, runs cargo hack check --workspace --no-dev-deps --rust-version and then the full test
  suite at that toolchain, and it is currently vacuous because rust-toolchain.toml pins the same
  number - it proves the workspace builds at 1.97.1, which was never in doubt, and not that 1.97.1
  is the minimum, which nothing in this repository proves. It starts proving something the day the
  pin and the floor diverge, which is why it is kept. Four alternatives lost: promising only
  "latest stable", which is the absence of a floor rather than a floor; pinning rusqlite back to
  0.37, which buys a number with a security and maintenance surface and would be re-fought every
  release; a per-package rust-version, which is the strongest of the four and loses on verification
  capacity rather than on merit, since four floors need per-package MSRV checking this workspace
  does not have; and a moving N-2-stable window, which breaks a consumer's build on a date rather
  than on an upgrade they chose.
depends_on:
  - kb-decision-0004
  - kb-decision-0029
related:
  - kb-governance-referent-not-reasoning-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-playbook-assert-execution-not-discovery-001
source_paths:
  - .kb/_intake/msrv-becomes-a-promise-at-0-2-0.md
  - Cargo.toml
  - rust-toolchain.toml
  - .github/workflows/ci.yml
  - CHANGELOG.md
  - RUNBOOK.md
last_reviewed: 2026-09-04
```

**Adjudication 1 — the number, and why nothing on disk allocated it.** Wave 5's rule is that an
ADR number is allocated by the artefact that already carries it. `references/adr/` stops at 0035,
the highest decision atom is 0036, and no file anywhere in the tree mentions ADR-0037. So the
number is highest-taken + 1 = **ADR-0037**, which is also what the staged file proposes. This is
the first time the file's proposal and the derivation have agreed with no artefact between them,
and the derivation is what the wave relies on.

**Adjudication 10 — amendment, not supersession, and the corpus invented this shape for exactly
this lineage.** ADR-0029 amended ADR-0004 without editing it, and
`kb-governance-referent-not-reasoning-001` generalised why: an edit that does not change what a
document *asserts* is not the test — the test is whether the assertion moved. ADR-0004 asserts
that the floor is a preference until first publish; ADR-0029 asserts the number is 1.97.1. **Both
assertions remain true after ADR-0037**, which asserts a third thing neither made: that the
preference has become a promise. Nothing is corrected, so nothing is superseded. `supersedes:
null`, `superseded_by: null` on all three, and the wave shows zero changed bytes in the two
predecessors — which `redkiln validate --kb` checks against `HEAD` rather than taking on trust.

**Adjudication 11 — `reversibility: medium`, against a real argument for `low`.** The content of
this decision is that a number stops being free to move, which reads like `low`. It is set to
`medium` on two grounds. First, consistency: ADR-0004 (which *states* the bump-is-a-minor-bump
policy) and ADR-0029 (which *exercises* it) are both `medium`, and a lineage whose third member
prices the same reversal differently invites the reader to think the mechanism changed. Second,
the mechanism genuinely has not: reversal is defined, cheap in process (a minor bump and a
changelog line), and below 1.0 cannot reach a consumer who did not choose it. What is expensive
is the trust, and this corpus's `reversibility` field has never priced trust. Recorded here
because it is a judgement rather than a derivation.

**The two corrections from `00`, and what they do to the body.**

- **Correction 1.** The atom **names** the four crates rather than counting them, and states in
  one clause that `xtask/src/package.rs`'s `PUBLISHABLE` holds five because packaging and
  publication are different questions — `happenstance-cloudflare` is checked for licence files
  and a README without being in phase 12's release set. CLAUDE.md already records what a bare
  count did here once (*"this sentence read 'three' through phase 9's promotion of
  `happenstance-cloudflare` and did not move"*), and an immutable atom is the worst place in the
  corpus to repeat it.
- **Correction 2.** The atom is written in the source's own future tense. `Cargo.toml` reads
  `0.2.0-alpha.1`, `CHANGELOG.md`:306 records that alpha as the only version cut, and
  `RUNBOOK.md`:163 has phase 12 at `not started`. A past-tensed atom would put three atoms and a
  RUNBOOK row into disagreement about a date, inside the immutable layer, and would do it while
  `kb-open-question-es-17-two-adapter-measurement-001` uses the same phase-12 boundary as its own
  deadline.

**What the atom must not do — the source's own three exclusions, kept.** It does not restate
`crates/happenstance/README.md`:49's *"MSRV 1.97.1, checked in CI"*; that sentence claims more
than the atom supports, and its correction is a backlog story
(`guarantees-and-docs-rs-presentation`), not an atom.
`open-questions/README.md` excludes *"work someone is expected to do"*, so it gets no open
question either. It does not touch ADR-0004 or ADR-0029. And it does not change the number.

**`mapsImpact`:** `decisionMap` — one new row, and a new wave section, since no existing section
covers a phase-12 publication decision. `domainMap` — the toolchain-and-edition domain gains its
third atom.

---

## Op 8 — `Event::metadata` has no declared floor

**Disposition:** `defer_open_question` · **Destination:** `.kb/open-questions/event-metadata-has-no-declared-floor.md`

```yaml
id: kb-open-question-event-metadata-no-floor-001
title: Event::metadata is a fourth opaque payload with no floor and no refusal path
kind: open_question
status: accepted
authority_tier: note
summary: >-
  happenstance-core ships four MIN_SUPPORTED_* constants - EVENT_DATA_LEN, TAGS_PER_EVENT,
  QUERY_ITEMS and EVENTS_PER_BATCH - and a StoreLimit enum with three variants, EventDataLen,
  TagsPerEvent and EventsPerBatch. Event::metadata appears in neither set. It is a fourth opaque
  payload carrying, among other things, the codec tag ADR-0021 put there, and a store that cannot
  accept a metadata blob has no conformant way to say so: VT-25 is [FROZEN] and forecloses
  AppendError::Store for a capacity refusal, so the only distinguishable refusal is
  ExceedsStoreLimit and there is no variant to name. happenstance-cloudflare is already carving
  its margin out of data to compensate, which is a workaround chosen by one adapter rather than a
  property of the contract. What is not decided is which of three answers is right: metadata gets
  its own floor and its own StoreLimit variant, metadata shares data's floor and the two are
  budgeted together, or metadata is declared deliberately unbounded and the specification says so.
  ADR-0015 is accepted and immutable and minted the four floors and the three variants, so the
  first two answers require a superseding decision atom and the third requires a clause; the
  2026-09-03 pre-publication review names the question and supplies none of them. Forced by phase
  12, where a public constant set and a frozen error variant become a promise, and independently
  by the first adapter that has to refuse a metadata blob.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0021
  - kb-decision-0003
  - kb-open-question-cf-40-ownership-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-core/src/limits.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
```

**Adjudication 6 — a claim that names a supersession it does not supply.** This is the wave's
sharpest test of KB authority rule 1. The rule *permits* superseding an accepted decision; the
review says in terms that a resolution *"needs a superseding atom"* against ADR-0015. It is
right. It is also silent on what the atom would say — the review does not choose among the three
answers, does not measure a metadata payload, and does not name a rejected alternative. A
supersession minted here would be an ingest inventing a commitment on a decision's behalf, which
is the one thing rule 1 forbids more clearly than editing: *"never silent drift."*

`decisions/README.md` settles it in one sentence: *"A decision not yet taken. That is an
`open_question`… Filing a live question here gives it an authority nobody granted it."*

**What the atom must not do.** State a preferred answer, or read as a recommendation that
`metadata` get a floor. All three arms are live, and the third — *declared deliberately
unbounded* — is the one a reader will discount and is the cheapest of the three. The atom
enumerates and stops.

**`mapsImpact`:** `openQuestionIndex` (new bullet), `domainMap` (the contract-limits domain).

---

## Op 9 — the read page budget is unspecified

**Disposition:** `defer_open_question` · **Destination:** `.kb/open-questions/read-page-budget-is-unspecified.md`

```yaml
id: kb-open-question-read-page-budget-001
title: A read page is budgeted in rows, and the two costs of that knob pull opposite ways
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0011 settled what read promises - one state sampled no later than the first poll, a position
  ceiling bounding every later statement, an inclusive to and an Option<usize> limit - and settled
  nothing about what a page costs. PAGE_SIZE appears nowhere in the knowledge base before this
  atom. The 2026-09-03 pre-publication review measured both of its consequences and they are not
  co-optimisable on one knob: one page of 512 ceiling-sized events is 512.2 MiB resident, which
  says the page is already too large, while raising PAGE_SIZE is the measured win for aggregate
  lock time, 2,929 seconds down to 99 over a one-million-event replay, which says it is too small.
  What is not decided is whether a page is budgeted in rows, in bytes, or by the caller. A row
  budget is what ships and is what the residency measurement indicts; a byte budget bounds
  residency and makes the number depend on payload size, which is exactly the quantity a store
  cannot know before reading; a caller-supplied budget moves the choice to the only party that
  knows both the payload distribution and the memory it has, at the cost of a port surface change
  that phase 12 closes. Forced by phase 12, after which the read surface is a promise, and
  independently by the first deployment that replays a log large enough for the lock time to
  matter on a machine small enough for the residency to.
depends_on: []
related:
  - kb-decision-0011
  - kb-reference-wf-11-memory-ceiling-verdict-001
  - kb-reference-projection-fan-out-cost-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
```

**Why an open question and not a reference atom carrying the two numbers.** The numbers are real
and the review measured them, so a reference atom was available. It was refused because the
numbers' whole significance is the **tension**, and `reference/README.md` excludes *"a conclusion
drawn from the measurement"* — an atom holding 512.2 MiB and 2,929 → 99 s with no statement that
they oppose each other on one constant would be two facts nobody joins. Recording the fork is
what makes them worth keeping, and a fork with both tines named and neither chosen is the
open-questions layer by definition.

**Why it is not a task.** `open-questions/README.md` excludes *"work someone is expected to do"*.
Nothing here is a work order: the three budget shapes are three different port surfaces, and
choosing among them is a decision with a phase-12 deadline, not a fix.

**`mapsImpact`:** `openQuestionIndex`, `domainMap`.

---

## Op 10 — no fixture tolerance for transient contention

**Disposition:** `defer_open_question` · **Destination:** `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md`

```yaml
id: kb-open-question-testkit-contention-tolerance-001
title: A busy store and a broken store are the same Attempt, and the suite cannot tell them apart
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-33 is [FROZEN] and forbids a conformance rule a clock, an elapsed-time measurement or an
  assertion on an operation count, which guarantees the suite cannot distinguish a store that is
  momentarily contended from one that is wrong: both surface as the same failed Attempt. ADR-0022
  section 12 says explicitly that the contender count is not its call, so no decision owns the
  question either. What the 2026-09-03 busy-timeout measurement adds is that the case is no longer
  hypothetical - busy > 0 was observed at the shipped CONTENDERS = 64, one launch in seven, the
  first nonzero busy count in this tree - so an adapter can now be contended inside the gate's own
  configuration. What is not decided is whether the fixture contract grows a declared tolerance
  for transient contention, and if so what shape: a Capability the way SECOND_HANDLE and REOPEN
  are, a numeric associated constant the way CF-40's limits are, or nothing at all, on the ground
  that a suite which tolerates a busy store has stopped being a conformance suite. ADR-0034
  already answers who would mint it rather than whether it should exist - the fixture contract has
  no single owning document, and a CF- clause is minted by the decision that first needs the
  capability - so this question is about the capability, not about its home. Forced by the first
  adapter whose conformance run fails on contention rather than on conformance, and by whoever
  next proposes changing CONTENDERS.
depends_on: []
related:
  - kb-decision-0034
  - kb-decision-0022
  - kb-decision-0010
  - kb-open-question-poll-count-rule-strength-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-busy-timeout-margin-001
  - kb-reference-nested-block-on-lost-wakeup-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - crates/happenstance-testkit/src/concurrency.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
```

**Adjudication 8 — the wave's most contestable refusal, at 55.** This question and Op 13 both
descend from one observation: `busy > 0` at 64 contenders. Folding them was live and is refused
on three grounds, and the third is the deciding one.

- **Different owners.** Op 13's subject is `happenstance-sqlite`'s pragma, decided by ADR-0022.
  This one's subject is the fixture contract, which ADR-0034 says has no single owning document.
- **Different answer shapes.** Op 13 resolves into a superseding decision atom, or into a
  documented refusal to re-open. This one resolves into a `CF-` clause and a fixture constant, or
  into a recorded position that the suite deliberately will not tolerate contention.
- **Different lifetimes, and this is the one that settles it.** Op 13 closes the moment somebody
  answers whether ADR-0022 re-opens — a bounded question with a named document. This one survives
  that answer in either direction: a suite that cannot distinguish busy from broken is still
  unable to, whatever the SQLite adapter's busy timeout becomes, and would be unable to on a
  Postgres adapter with no busy handler at all. One atom would close half of itself and leave the
  other half filed under a resolved question.

Recorded as contestable because a reader who weights *provenance* over *lifetime* would fold them,
and that reading is not unreasonable — it is the same call wave 7 flagged at its Adjudication 8.

**`mapsImpact`:** `openQuestionIndex`, `domainMap` (conformance-suite domain).

---

## Op 11 — `remint_identity`'s precondition is trust-only

**Disposition:** `defer_open_question` · **Destination:** `.kb/open-questions/remint-identity-precondition-is-trust-only.md`

```yaml
id: kb-open-question-remint-precondition-trust-only-001
title: VT-6's documented-procedure branch asks a program to trust a document it could check
kind: open_question
status: accepted
authority_tier: note
summary: >-
  VT-6 requires that a StoreId be minted when a store's persistent state is created and never
  derived from anything surviving a restore, and ADR-0014 grants an adapter three ways to satisfy
  it: mint once and re-mint only when it can detect a restore or a clone, mint fresh on every
  open, or re-mint under a procedure the deployment documents. The third branch is the one this
  question is about. SqliteEventStore::remint_identity is that branch's implementation and its
  precondition is trust-only: the caller asserts that this database file is a restore or a clone,
  and nothing in the adapter checks the assertion, including in the case a program could see -
  the same file, opened twice, in one process. What is not decided is whether an adapter taking
  the documented-procedure branch owes an in-process check, and if so what it may check against
  without inventing a durability claim of its own. ADR-0014 is accepted and grants the branch
  without addressing the obligation, so this is a gap inside a branch a decision opened rather
  than a disagreement with one. Forced by phase 5's identity work and by phase 13, where sync
  makes a duplicated StoreId a peer's problem rather than a local one - a re-minted identity that
  should not have been re-minted is exactly the collision VT-6 spends a clause preventing.
depends_on: []
related:
  - kb-decision-0014
  - kb-decision-0022
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
```

**Why this scores 62 against an accepted decision and still produces no operation on it.** ADR-0014
grants the branch in one clause — *"or if the deployment documents when re-minting is invoked"* —
and the question is what that branch obliges. Wave 3's rule reads cleanly here and this is its
cleanest instance in the corpus: a decision that opens a door is not the same knowledge as the
question of what is behind it. `kb-decision-0014` is accepted, is not edited, and gains a
`related` edge only from this side.

**`mapsImpact`:** `openQuestionIndex`, `domainMap` (identity-and-time domain).

---

## Op 12 — the query plan chunks one axis of two

**Disposition:** `defer_open_question` · **Destination:** `.kb/open-questions/query-plan-parameter-chunking-incomplete.md`

```yaml
id: kb-open-question-query-plan-parameter-chunking-001
title: PARAMETER_BUDGET governs one of the two axes the query plan chunks on
kind: open_question
status: accepted
authority_tier: note
summary: >-
  happenstance-sqlite's query planner chunks on two independent axes and applies its parameter
  budget to one of them. crates/happenstance-sqlite/src/event_store.rs:577 declares
  PARAMETER_BUDGET = 30_000 and carries the arithmetic that derives a statement's row capacity
  from it; :760 is its only caller, sizing the tag-row insert. The other axis is UNION arm width,
  chunked through query_sql::chunks at a width of 400 on both the read path and the wide-guard
  path, documented as chunked and merged, never refused, because there is no MAX_QUERY_ITEMS to
  refuse against. Those two decompositions are computed independently, so a query whose arm count
  is inside the width and whose bound-parameter count is not has nothing bounding it. What is not
  decided is whether the parameter budget is owed on the arm-chunking paths as well - which would
  make the effective chunk width the minimum of an arm bound and a parameter bound - or whether
  the arm width of 400 is already conservative enough that the parameter bound can never bind
  first, in which case the answer is an assertion and a comment rather than two more callers.
  Nobody has measured which. Forced by phase 12, after which MIN_SUPPORTED_QUERY_ITEMS is a
  public promise a store must clear, and by any adapter raising the arm width.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0015
  - kb-open-question-query-union-rule-unowned-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-04
```

**Adjudication 9 — the question, and the task it is not.** The staged phrasing — *"`PARAMETER_BUDGET`
… has one caller where it should have three"* — is a defect statement, and a defect statement is a
backlog row. `open-questions/README.md` excludes it in terms: *"Work that someone is expected to
do is a backlog item in `.bklg/`, not a KB atom."* The atom is therefore written around the
**question** the defect implies, which is genuinely undecided and which the review states first:
*does the query plan chunk on bound parameters as well as UNION arms?* Two answers are live and
they differ in what ships — a minimum of two bounds, or an assertion that one can never bind. The
second answer adds no callers at all, which is why *"add two callers"* is not a restatement of
the question.

This is `es-17`'s standing shape, and that atom's own closing sentence is the model: *"The KB half
of this obligation is this open-question atom; the corresponding backlog row belongs in `.bklg/`
and is not authored here."*

**`mapsImpact`:** `openQuestionIndex`, `domainMap` (SQLite-adapter domain).

---

## Op 13 — ADR-0022's falsifiers have fired and nobody has re-opened

**Disposition:** `defer_open_question` · **Destination:** `.kb/open-questions/adr-0022-falsifiers-have-fired.md`

```yaml
id: kb-open-question-adr-0022-falsifiers-fired-001
title: Two of ADR-0022's falsifiers have fired, a third cannot fire as written, and nobody has re-opened
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0022 wrote three conditions under which it would be re-opened. The 2026-09-03 pre-publication
  review checked all three, two have fired and the third is unfireable as written, and no
  re-opening has happened. Section 11 said re-open if any run ever reports busy > 0; busy > 0 was
  observed at the shipped CONTENDERS = 64, one launch in seven, the first nonzero busy count
  anywhere in this tree, and the shape matters - busy = 1 and busy = 2 with exhausted = 0, meaning
  attempts that entered the handler rather than attempts that ran the 5,000 ms budget out, so the
  margin held while the premise did not. Section 9 said re-open if a deployment shows the captured
  tokio Handle costing something the inline path does not; the capture is unconditional and
  irreversible, which makes SqliteEventStoreError::NoRuntime unreachable for any store outliving
  the runtime it was constructed on, so the variant ADR-0022 kept in order to preserve a real
  meaning has lost it by a different route than the one that section anticipated. Section 16's
  falsifier for section 8 cannot fire as written, because the shape it names as the re-open
  trigger - a GROUP BY with HAVING COUNT(DISTINCT tag) aggregate - is not what the adapter emits;
  measured, the chain that does ship loses to that aggregate in nine of nine two-tag cells, which
  is the finding the falsifier was written to catch and which its own wording excludes. ADR-0022 is
  accepted and immutable, so a fired falsifier cannot amend it: what is not decided is whether it
  is superseded, re-opened with a scoped amendment, or explicitly ratified as still correct with
  the firings recorded against it - and who takes that call. Forced by phase 12, after which the
  pragma set is a documented property of a published adapter.
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-open-question-testkit-contention-tolerance-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - references/adr/0022-append-condition-strategy.md
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-04
```

**Adjudication 4 — the wave's single most important application of KB authority rule 1.** ADR-0022
promised, in its own body, to re-open under three named conditions. Two have now been met. The
temptation an ingest faces here is not to edit the decision's *argument* — nobody would — but to
record the firing *inside* it, as a "status" line or a dated note, on the reasoning that a
falsifier firing is the decision's own machinery working. That is exactly the edit rule 1 forbids,
and it is worse than an ordinary body edit: **the thing being written into an immutable record
would be the news that its own promise came due.** A reader of the amended atom could no longer
tell whether the decision was signed with that knowledge or acquired it afterwards, which is the
one distinction a decision record exists to preserve.

So the firing lands outside, and `kb-open-question-es-17-two-adapter-measurement-001` is the
precedent in every respect: same decision, same failure class (a falsifier whose evidence is owed
and whose owner is nobody), same closing move of naming the forcing phase rather than an assignee.

**Adjudication 4a — one atom for three falsifiers, at 85.** They are separable as *events* and
inseparable as a *question*. What is undecided is singular — does ADR-0022 re-open, and who says
so — and three atoms would be three records of one unanswered question, three bullets on the
open-questions index, and a reader obliged to reassemble them before understanding that the
answer is one call. The §16 finding in particular cannot stand alone: *"a falsifier that cannot
fire as written"* is only interesting beside two that did.

**What the atom must not do.** Recommend a resolution. All three arms are live, and the third —
ratify ADR-0022 as still correct, with the firings recorded against it from outside — is
defensible: §11's firing was `busy > 0` with `exhausted = 0`, which is a premise breaking rather
than a margin breaking, and a decision may reasonably survive that. The atom states the shape of
the observation precisely enough that the third arm is visible, and chooses nothing.

**`mapsImpact`:** `openQuestionIndex`, `domainMap` (SQLite-adapter domain). **No `decisionMap`
change** — ADR-0022's row is untouched, because nothing about its status has changed.

---

## Op 14 — the adapter-default projection question is resolved

**Disposition:** `merge_existing` · **Destination:** `.kb/open-questions/projection-store-in-adapter-default-features.md` (`kb-open-question-adapter-default-projection-feature-001`) · **Score:** 95

**What changes.**

- **Frontmatter.** `status: accepted` → `status: superseded`. `summary` gains a trailing
  resolution clause in the corpus's established form — the shape
  `kb-open-question-cf-40-ownership-001` carries — naming the date, what was done, and that
  ADR-0036 was not touched. `source_paths` gains
  `.kb/_intake/adapter-default-projection-feature-resolved.md`. `last_reviewed` → `2026-09-04`.
  `related` is unchanged: `kb-decision-0036` is already there, which is the atom the resolution
  enforces.
- **Body.** Every existing section stays **verbatim**. One new dated section is appended,
  *"Resolved 2026-09-03"*, carrying claims 1–4: the shape both manifests now hold, the
  eight-check verification, why the two halves had to move together, what did not change, and why
  it was closed here rather than by the owning project.
- **No `superseded_by` key is added.** It is a decision-only field in the frontmatter contract,
  and neither of this layer's two prior resolutions carries one — `cf-40` and the `deny.toml`
  question both name their resolver in `related` and in the summary and nowhere else.

**Adjudication 12 — the wave's most contestable call: an amendment where the README says a new
atom.** `open-questions/README.md` is explicit: *"When the question is answered, the answer is a
**new atom** — and the record of the question stays… Do not rewrite a question into its own
answer."* The staged file is equally explicit in the other direction: *"the ingest wave should
amend `kb-open-question-adapter-default-projection-feature-001` to record that it is answered and
how, rather than mint a sibling atom that restates it."*

The wave sides with the file, and the argument is that the README's rule and the file's request
are not actually opposed — the README governs **where the answer's knowledge lives**, and here
there is no knowledge to house.

- **Both prior resolutions in this layer produced a new atom because one already existed for
  independent reasons.** `cf-40` was resolved by ADR-0034, a decision that settled a position
  about the whole fixture contract. The `deny.toml` question was resolved by ADR-0035, which
  ratified an exemption written into `deny.toml`. In both cases the new atom carried a
  commitment; the question's flip merely pointed at it.
- **Here, no new atom is available, and minting one would require inventing a commitment.**
  ADR-0036 already commits `ProjectionStore` to an off-by-default gate. Two adapters honouring
  that gate adds no obligation, names no rejected alternative, and settles no fork —
  `decisions/README.md` calls a decision recorded without its rejected options *"indistinguishable
  from an accident"*, and this one has none because ADR-0036 foreclosed them.
- **A non-decision sibling would be a file whose whole content is "the question above was right."**
  That is the shape KB authority rule 3 exists to stop: *merge and link over creating a new atom*,
  and *never copy paragraphs into a second atom*. A reference-kind census — *as of 2026-09-03,
  both manifests read X* — was the strongest alternative and is refused for the same reason wave
  7 refused mirroring mutable manifests: `reference/README.md` excludes *"a mirror that someone is
  expected to keep current"*, and a three-line manifest shape is exactly that.

**What the appended section preserves, and why the body is not rewritten.** The question's *"Why
the exposure is not the sqlite exposure, and runs the wrong way"* section made a prediction — that
trimming `default` alone would close nothing — before the fix existed, and the resolution
confirms it held *exactly as written*. Rewriting that section into the past tense would destroy
the corpus's clearest instance of an open question's diagnosis being verified. The README's own
reason for the verbatim rule says so: *"the value of the record is that it shows the state of
knowledge on the day the choice was made."*

**`status: superseded` rather than `withdrawn`.** The README permits either. Both prior
resolutions in this layer used `superseded`, and `withdrawn` reads as *the question was retracted*
rather than *the question was answered*. Consistency and accuracy point the same way.

**`mapsImpact` — and this is where claim 5 lands.** `openQuestionIndex`: the bullet at
`.kb/maps/open-questions-index.md`:288-292 flips from **Open** to **Resolved 2026-09-03**, and
its description — which currently ends *"still carry `default = ["event-store",
"projection-store"]`"* — describes manifests that no longer exist and must be rewritten to the
resolved form the index uses for the `deny.toml` question two bullets above it. `domainMap`: the
parenthetical at `.kb/maps/domain-map.md`:245-248 carries the same stale manifest description and
needs the same treatment. **No `decisionMap` change** — ADR-0036 is untouched and its
`[PROVISIONAL]` marker on PS-3 does not move, which claim 3 states and this wave honours by
absence.

---

## The five things this wave deliberately does not do

1. **It does not supersede ADR-0015**, though a claim says a resolution needs one. Op 8.
2. **It does not record ADR-0022's fired falsifiers inside ADR-0022.** Op 13, Adjudication 4.
3. **It does not flip ADR-0004's or ADR-0029's frontmatter.** Op 7, Adjudication 10 — and the
   wave is expected to show zero changed bytes in both, which `redkiln validate --kb` checks
   against `HEAD` rather than taking on trust.
4. **It does not repair `crates/happenstance/README.md`:49, `xtask/src/proof.rs`, or
   `references/evaluation/phase-7-contract-defects.md`:10.** Three known defects, three backlog
   rows, zero KB atoms — `open-questions/README.md` excludes work someone is expected to do, and
   Ops 4, 5 and 7 each record the finding without prescribing the fix.
5. **It does not rewrite the four corpus sites stating the `Event` clone cost as two.** Op 1
   records the measurement; one of the four is inside an accepted decision and the repair is not
   an ingest's.
