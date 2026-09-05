# Wave `2026-09-04-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus, about the
tree, and about the three staged files.

This is the corpus's **eighth wave**, and it is the inverse of the seventh in the respect that
matters most to an ingest. Wave 7 staged seven files that were **four subjects**, and its hardest
job was collapsing them. This one stages three files that are **three subjects**, and there is
not a single cross-file cluster in it. The dedup pressure has moved entirely inside one document
and against the corpus, which changes what this file has to argue: not *which two files say the
same thing*, but **which of one review's twelve claims are one claim, and which of them have an
owner already**.

Its hard jobs are four:

1. **Refusing the collapse of three findings that share an experiment directory and not a
   subject.** `experiments/busy-timeout-margin/` produced both R-3 (SQLite's busy handler under
   contention) and R-4 (the testkit's own executor losing a wakeup) — `lost_wakeup.rs` is
   literally a test in the busy-timeout experiment crate. One directory, two readers, two atoms.
   `02`, Adjudication 3.
2. **Separating a measurement from what it does to an accepted decision.** The same run that
   produced R-3's margin table is the run that fired ADR-0022 §11's *"re-open if any run ever
   reports `busy > 0`"*. `reference/README.md` forbids the reference atom from carrying the
   conclusion, and `decisions/README.md` forbids the conclusion from being written into
   ADR-0022's body. Two atoms, one measurement. `02`, Adjudication 4.
3. **Deciding what a resolved open question actually receives.** The adapter-default file asks
   in its own voice for the existing open question to be **amended** rather than answered by a
   sibling atom — and `open-questions/README.md` says *"the answer is a **new atom**"* and *"do
   not rewrite a question into its own answer."* This wave sides with the file, and the argument
   is `02`, Adjudication 12. It is the wave's most contestable call.
4. **Two corrections to a staged framing, both about publication, and both of which change what
   an atom may assert.** The MSRV file's operative claim is future-tensed and correct; the shape
   in which it is easiest to absorb it is past-tensed and false. Corrections 1 and 2, below.

## The corpus, as verified

```
ls .kb/decisions      →  34 atoms (0001–0023, 0029–0036, SD-0001, SD-0002 ×2) — 32 accepted, 2 superseded (0002, 0021)
ls .kb/open-questions →  28 atoms, all authority_tier: note — 21 accepted, 7 superseded
ls .kb/reference      →  12 · playbooks 7 · maps 3 · design 2 · concepts 1 · governance 1
ls .kb/product        →   0 atoms, README only  ← nothing routes there this wave
                         88 atoms + 10 layer READMEs, before this wave
ls references/adr     →  26 records: 0001–0023, 0029, 0030, 0035
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **34** (32 accepted) | **One is added** — ADR-0037. **No body is edited, no `status` is flipped, and no decision is superseded anywhere in this wave.** Five accepted atoms are named by incoming claims and none receives an operation: `kb-decision-0004`, `kb-decision-0029`, `kb-decision-0022`, `kb-decision-0015`, `kb-decision-0036` |
| `open-questions/` | 28 (21 accepted) | **Six are created** — the largest single-wave addition this layer has taken — and **one is resolved** (the adapter-default question: metadata plus a dated section, body verbatim) |
| `reference/` | 12 | **Three are added.** No existing reference atom is extended — the dating rule, fifth wave running |
| `playbooks/` | 7 | **One is added and one is extended.** `kb-playbook-anchoring-citations-001` is the first playbook in this corpus's history to receive a merge rather than a link |
| `governance/` | **1** | **One is added.** The layer has held exactly one atom since it was scaffolded; this is its second |
| `maps/` | 3 | All three inherit work — `decision-map` one row, `domain-map` two sections, `open-questions-index` seven bullet changes (six added, one resolved) |
| `design/`, `concepts/`, `product/` | 2, 1, 0 | No bearing. Nothing this wave stages is a persona, a journey, an interaction pattern or a durable idea needing explanation |

For authority purposes:

- **Thirty-two accepted decision atoms exist and the intake touches five of them by name.**
  ADR-0004 and ADR-0029 are the lineage ADR-0037 amends *from outside*; ADR-0022 has two of its
  own falsifiers fired at it; ADR-0015 is named as needing a superseding atom it does not get
  this wave; ADR-0036 is the decision the adapter-default resolution enforces. **Not one of them
  is edited, flipped, or superseded.**
- **Zero supersessions.** Third wave running. Two staged files refuse the edit in their own
  voice: the MSRV file's *"ADR-0004 and ADR-0029 stay `status: accepted` and byte-identical,
  frontmatter included"*, and the adapter-default file's *"ADR-0036 stands exactly as written and
  its `[PROVISIONAL]` marker does not move."*
- **One claim asks for a supersession this wave declines to sign.** The review's Q1 says in terms
  that a floor for `Event::metadata` *"needs a superseding atom"* against ADR-0015. It does — and
  the review does not supply one, because it names the question rather than answering it. That is
  an open question, not a decision. `02`, Adjudication 6.

## Provenance check

Every path the three files name was tested against this worktree. Unlike wave 7's check, this one
came back clean on every file — the brand-tree absence that limited that wave has no analogue
here — and the two corrections below are about *framing*, not about missing artefacts.

```
references/evaluation/review-pre-publication-2026-09-03.md      ✓ exists — the source, pinned to 56ef6c5
experiments/gate-vacuity/results/raw/list-diff.txt              ✓ exists and is 0 bytes — R-1's load-bearing evidence
experiments/event-clone-allocations/results/raw/                ✓ clone.txt, conditions.txt, conformance.txt, read.txt
experiments/busy-timeout-margin/results/raw/                    ✓ cores-debug-c{1,2,4,8,20}.txt and the release/control set
experiments/busy-timeout-margin/tests/lost_wakeup.rs            ✓ exists — R-4's deterministic reproduction
xtask/src/proof.rs                                              ✓ exists — the --list anti-vacuity check
crates/happenstance-core/src/tag.rs                             ✓ exists — Tags is Box<[Tag]>, Tag is Cow<'static, str>
crates/happenstance-testkit/src/registry.rs:338-342              ✓ block_on parks on Poll::Pending with no notified flag
crates/happenstance-sqlite/tests/concurrency.rs:45-47            ✓ "If a rule hangs, that is evidence about ADR-0022's busy-timeout paragraph"
crates/happenstance-core/src/limits.rs                          ✓ four MIN_SUPPORTED_* constants, three StoreLimit variants — Q1 confirmed
crates/happenstance-sqlite/src/event_store.rs:577,760            ✓ PARAMETER_BUDGET declared once, called once — Q5 confirmed
crates/happenstance-sqlite/src/event_store.rs:266-290,622-664    ✓ the second chunking axis: UNION arms, width 400, via query_sql::chunks
crates/happenstance-sqlite/src/event_store.rs:448                ✓ remint_identity exists — Q4 confirmed
spec/SPECIFICATION.md:813-830 (VT-6), :4302-4322 (CF-33 hang)    ✓ both verbatim as cited
standards/rust/README.md:23-43                                  ✓ the precedence ladder, verbatim, plus the RS-01-3 worked example
references/evaluation/phase-7-contract-defects.md               ✓ exists — the mis-anchored header the citation claim names
crates/happenstance-neon/Cargo.toml                             ✓ NOW MATCHES the intake exactly — see below
crates/happenstance-postgres/Cargo.toml                         ✓ same
Cargo.toml, rust-toolchain.toml                                 ✓ version 0.2.0-alpha.1, rust-version 1.97.1, channel 1.97.1
.github/workflows/ci.yml:307-321                                ✓ the msrv job, toolchain "1.97.1" spelled out with the divergence comment
crates/happenstance/README.md:49                                ✓ "MSRV 1.97.1, checked in CI" — the sentence the atom declines to restate
xtask/src/package.rs:86-107                                     ✗ PUBLISHABLE names FIVE, not four — Correction 1
RUNBOOK.md:163, :4681-4703                                      ✗ phase 12 is `not started` — Correction 2
references/adr/0037-*.md                                        ✗ absent; nothing on disk allocates the number — wave 5's rule applies
.kb/decisions/                                                  ✓ no 0037-*.md; highest ADR atom is 0036
```

### The adapter-default file is the first staged file in this corpus to be verified *true* where its predecessor was verified *false*

Wave 7's Correction 1 found `happenstance-neon` and `happenstance-postgres` carrying
`happenstance-core = { features = ["std", "unstable-projection"] }` unconditionally in
`[dependencies]`, with `projection-store = []` forwarding nothing — a feature naming a capability
while doing none of the gating. That correction is what produced
`kb-open-question-adapter-default-projection-feature-001` rather than a paragraph inside
ADR-0036's immutable body, and the argument for the split was that the residual was *"a statement
about mutable manifests"*.

Both manifests now read, verbatim:

```
happenstance-core = { workspace = true, features = ["std"] }
default          = ["event-store"]
projection-store = ["happenstance-core/unstable-projection"]
```

Which is `happenstance-sqlite`'s shape exactly, and which is the arrangement the open question
said would be needed — *"whether the `happenstance-core` dependency declaration should itself
move `unstable-projection` behind a forwarded feature … rather than enabling it unconditionally,
and whether the two changes should land together or separately given that the second is the one
that actually restores off-by-default."* Both landed together. **The wave-7 split is vindicated
by the thing it predicted**: had that residual ridden inside ADR-0036's body, an immutable
decision would now be carrying a manifest description that stopped being true in two days.

### Correction 1 — "four crates" is right about the release and wrong about the packaging assertion

The MSRV file says *"`0.2.0` puts four crates on a registry."* Checked in both directions:

```
RUNBOOK.md:4683    "happenstance-core, happenstance, happenstance-testkit and happenstance-sqlite
                    on crates.io, rendering on docs.rs"                                    ← four
xtask/src/package.rs:86-107  PUBLISHABLE = core, happenstance, testkit, sqlite, cloudflare  ← five
crates/*/Cargo.toml          five manifests carry no `publish = false`                      ← five
```

Four is **correct about what phase 12 publishes** and **wrong as a description of the workspace's
publishable set**, and the two numbers are one crate apart for a reason that is recorded:
`happenstance-cloudflare` gained its packaging surface at phase 9 and lost its `publish = false`
in the same change that added it to `PUBLISHABLE`, which is `reconcile`'s whole point. CLAUDE.md
already warns about exactly this arithmetic — *"this sentence read 'three' through phase 9's
promotion of `happenstance-cloudflare` and did not move, so a count on its own turned out to be a
claim nobody re-reads."*

**Consequence for Op 7:** the atom names the four crates rather than counting them, and says in
one clause that `PUBLISHABLE` holds five because packaging and publication are different
questions. A count with its members spelled out cannot drift the way a bare number did.

### Correction 2 — the promise is made ahead of the release, not after it

The MSRV file is careful: *"`0.2.0` puts four crates on a registry, and from that moment the
number is a commitment."* Future-tensed, and true. The past-tensed paraphrase — *"now that four
crates are published"* — is the shape the claim most easily collapses into during extraction, and
it is false against the tree:

```
Cargo.toml:15       version = "0.2.0-alpha.1"
CHANGELOG.md:306    ## [0.2.0-alpha.1] — 2026-08-16      ← the only version cut
RUNBOOK.md:163      | 12 | Publish 0.2.0 | 7, 8 | not started |
```

**Consequence for Op 7:** the atom is written in the tense the source uses. This is not
pedantry — ADR-0004's own Policy section says the floor *"becomes a promise rather than a
preference at first publish, phase 12"*, and `kb-open-question-es-17-two-adapter-measurement-001`
uses the same phase-12 boundary as its own deadline. An atom asserting that publication has
happened would put three atoms and a RUNBOOK row into disagreement about a date, and would do it
inside the immutable layer.

## Scoring method

Unchanged from all seven previous waves — a threshold that moves between waves is not a
threshold. A 0–100 judgement of **subject identity**: would a reader looking for one claim expect
to find the other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms, linked,
  and the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Rules carried forward, each with what it did this wave:

- **A README is not a merge target** (wave 1). Exercised twice —
  `.kb/open-questions/README.md` against the adapter-default resolution at 90 lexical, and
  `.kb/decisions/README.md` against the MSRV file's amendment-lineage claim at 60. Both are the
  authority being obeyed. **No operation.**
- **A supersession chain is never a merge** (wave 2). Not exercised — no supersession.
- **A decision naming a gap is not the same knowledge as the question that owns the gap**
  (wave 3). Exercised **twice**, and it is this wave's most-used rule: it is what stops ADR-0022's
  fired falsifiers from being written into ADR-0022, and what stops Q1's metadata floor from
  being written into ADR-0015.
- **An accepted decision atom is not a merge target, however high it scores** (wave 4). Exercised
  on `kb-decision-0029` (MSRV claim c1, **82**) and `kb-decision-0022` (falsifiers, **68**).
  **Two atoms scored against, zero operations on either.**
- **An ADR number is allocated by the artefact that already carries it** (wave 5). Nothing on
  disk carries 0037 — `references/adr/` stops at 0035 and the highest atom is 0036 — so
  highest-taken + 1 = **ADR-0037**, which is what the staged file itself proposes and the first
  time the file and the derivation have agreed without an artefact between them.
- **An intake file's numeric prefix is a staging sequence and never an ADR number** (wave 6). Not
  exercised — no staged file this wave carries a numeric prefix at all.
- **A record number may yield more than one atom, and the split is stated on both** (wave 7). Not
  exercised — ADR-0037 is one atom.
- **New this wave: an experiment directory is not a subject.** Three findings from
  `experiments/busy-timeout-margin/` are two atoms and one open question, split on who reads them
  rather than on where they were run. `02`, Adjudication 3.
- **New this wave: a second instance *of the same defect the playbook exists to prevent* is a
  merge, where a second instance *of the playbook's shape* is a link.** Six waves have refused to
  merge one playbook into another. This wave performs the corpus's first playbook merge, and the
  discrimination that licenses it is `02`, Adjudication 5.

## Merge candidates against the existing corpus

| Existing atom | Incoming claim | Score | Disposition |
| --- | --- | --- | --- |
| `kb-decision-0029` | MSRV c1 — 1.97.1, the forcing `cfg_select!` evidence, the five crates declaring no `rust-version`, the vacuous `msrv` job | **82** on subject, **0 actionable** | **no operation.** Accepted and immutable, and the staged file's own opening forbids the merge in terms. ADR-0037 amends from outside on **ADR-0029's own precedent against ADR-0004** — the corpus's second use of a shape it invented for exactly this lineage. `depends_on` on Op 7 |
| `kb-decision-0004` | MSRV c1, c2 — the bump-is-a-minor-bump policy, and *"a preference until first publish (phase 12) and a promise afterward"* | **75** | **no operation, and it is the sentence ADR-0037 executes.** ADR-0004 wrote the trigger; ADR-0037 is the trigger firing. Same subject, different settling condition — squarely 50–79. `depends_on` on Op 7 |
| `.kb/decisions/README.md` | MSRV c2 — amendment lineage, no `superseded_by` flip | 60 lexical | **no operation.** Wave 1's rule. The README is the authority being obeyed, and `kb-governance-referent-not-reasoning-001` is the atom that already generalises it. `related` on Op 7 |
| `kb-governance-referent-not-reasoning-001` | MSRV c2 — the repair/amendment discrimination | 45 | **link only.** That atom decides *whether an edit may be made*; ADR-0037 makes none. The edge is the precedent, not the knowledge |
| `kb-playbook-anchoring-citations-001` | citation claim — spec-trace resolves an address but does not check the claim; a fresh instance at `phase-7-contract-defects.md:10`; `references/evaluation/` and `spec/` carry no anchors where `standards/rust/` does | **74** | **MERGE — Op 5, and it is the corpus's first playbook merge.** This is not a second instance of a *shape*; it is a second instance of **the exact defect this playbook exists to prevent**, plus a coverage statement about the very corpora it names. `02`, Adjudication 5 |
| `kb-playbook-verify-referent-report-coverage-001` | citation claim (55); R-1 (**58**) | 55, 58 | **link only, twice.** Its generalisation section enumerates its own population — *"citations, links, ids, symbol references, schema `$ref`s, test-to-requirement traceability"* — and a test suite is not a cross-reference. Refused for the fifth wave running, and the reason is written down at `02`, Adjudication 2 |
| `kb-decision-0010` | R-1 — the anti-vacuity gate rests on `--list` | 40 | **link only.** ADR-0010 commits the suite to mutants and three meta-tests, which *run*; R-1 is about the xtask step that asserts named proof tests *exist*. Adjacent motivation, different mechanism, and the atom is accepted regardless |
| `kb-decision-0022` | falsifiers §11, §9, §16; R-3's margin table | **68** | **no operation.** Wave 4's rule, and the wave's sharpest application of it: two of this decision's *own* re-open triggers have fired, and the one thing that may not happen is the firing being written into the body that promised to re-open. Op 13, `depends_on` |
| `kb-reference-append-condition-experiment-001` | R-3 — the 2026-09 margin measurement | **72** | **link only, and the dating rule is why.** That atom is dated 2026-08-16 and records `busy=0` across thirty races at 64 connections; this one is dated 2026-09 and records `busy>0` at the same contender count. **They disagree, and both are true of their own moment.** Merging would overwrite a snapshot with a later one and destroy the evidence that the property changed. `reference/README.md`: *"Every atom here is a statement about a moment"* |
| `kb-reference-wf-11-memory-ceiling-verdict-001` | Q2 — 512 ceiling-sized events is 512.2 MiB resident | 35 | **link only.** Both are memory arithmetic; one is a `wasm32` isolate's growth ceiling and the other a host read path's page residency. One shared unit, no shared question |
| `kb-decision-0011` | Q2 — is a read page budgeted in rows, bytes, or by the caller | 50 | **link only, at the band edge.** ADR-0011 decided what `read` *promises* (one sample, a ceiling, `&Query`); Q2 asks what a page *costs*. The atom is accepted regardless, so the only live question was whether Q2 is knowledge ADR-0011 already holds, and it is not: `PAGE_SIZE` appears nowhere in `.kb/` |
| `kb-decision-0015` | Q1 — `Event::metadata` has no `MIN_SUPPORTED_*`, no `StoreLimit` variant | **65** | **no operation, and the non-supersession is the point.** Verified at `limits.rs`: four floors, three variants, `metadata` in neither set. The claim says it *"needs a superseding atom"* — and does not supply one. Wave 4's rule plus `decisions/README.md`'s *"a decision not yet taken is an `open_question`"*. Op 8 |
| `kb-decision-0021` | Q1 — the codec tag lives in `Event::metadata` | 40 | **link only.** ADR-0021 put a payload *in* metadata; Q1 asks what bounds metadata. The dependency runs one way and is worth an edge |
| `kb-decision-0034` | Q3 — a fixture-declared tolerance for transient contention | **58** | **link only, and it is the atom that says how Q3 would be answered.** ADR-0034's durable half — *"a CF- clause is minted by the decision that first needs the capability"* — is the *procedure* for minting what Q3 asks about. Procedure and question are not one atom; `es-17` is the standing precedent for a question that cites the decision naming its own gap |
| `kb-open-question-poll-count-rule-strength-001` | Q3 | 48 | **link only.** ADR-0034 names `POLL_BUDGET` as its position's next test; Q3 would be a *second* capability-shaped test of the same position. Two instances, two records — the corpus has never merged the second into the first |
| `kb-open-question-cf-40-ownership-001` | Q3 | 35 | **link only, and it is already `superseded`.** No operation on a superseded question |
| `kb-decision-0014` | Q4 — `remint_identity`'s precondition is trust-only | **62** | **no operation.** Verified: ADR-0014 grants the branch — *"or if the deployment documents when re-minting is invoked"* — and says nothing about whether a program must check. Q4 is a gap **inside a branch the decision granted**, which is the cleanest instance of wave 3's rule this corpus has produced. Op 11 |
| `kb-open-question-query-union-rule-unowned-001` | Q5 — parameter chunking | 25 | **link only.** That question is owed a *conformance rule* in the testkit; Q5 is about a chunking axis in one adapter's query planner. One word in common, no shared failure |
| `kb-open-question-es-17-two-adapter-measurement-001` | falsifiers — a named re-open condition nobody has acted on | 45 | **link only, and it is the shape precedent.** Same document (ADR-0022), same failure class (a falsifier with no owner), different falsifier. `es-17`'s own body is the template Op 13 follows |
| `kb-open-question-adapter-default-projection-feature-001` | adapter-default claims 1–4 | **95** | **RESOLVE — Op 14.** Not a merge into a peer and not a new sibling: metadata flip to `superseded`, body kept verbatim, dated section appended. Wave 6's `cf-40` and wave 7's `worker-async-trait-ban` executions, third running. `02`, Adjudication 12 |
| `kb-decision-0036` | adapter-default claims 1, 3 | **70** | **no operation.** Accepted, and the staged file's *"ADR-0036 stands exactly as written and its `[PROVISIONAL]` marker does not move"* forbids the merge in its own voice. `related` on Op 14, where it already is |
| `kb-governance-referent-not-reasoning-001` | governance claim — the precedence ladder | 35 | **link only.** One governs *how a record may be edited*; the other governs *what may overturn a finding about it*. Adjacent layer, different verb |
| `kb-playbook-cold-future-hand-polling-001` | R-4 — nested `block_on` loses a wakeup | 40 | **link only.** That playbook is about *driving* futures by hand to own a schedule; R-4 is about the testkit's `block_on` being unable to be driven twice on one thread. The second is the failure mode waiting for anyone who reaches for the first inside a rule already running under it, which is worth an edge and is not the same knowledge |
| `.kb/open-questions/README.md` | adapter-default claim 4 — resolve now rather than at the owning project | 90 lexical | **no operation.** The procedure is executed as Op 14 rather than recorded. Wave 1's rule |
| `.kb/reference/README.md` | R-2, R-3, R-4 — the dating rule | 85 lexical | **no operation, three times.** Ops 1, 2 and 3 are the rule obeyed, not a claim about it |

## Cross-file clusters

**None.** Three files, three subjects, zero cross-file collapses — the first wave since the
corpus's second to produce no cluster at all, and it is a property of the staging rather than an
oversight. Every pairing was scored:

| Pairing | Highest-scoring shared claim | Score | Ruling |
| --- | --- | --- | --- |
| pre-publication review × MSRV | the review's R-2 measurement conditions name `1.97.1`; the MSRV file names the same compiler | **15** | **Not a cluster.** A toolchain appearing as a *measurement condition* in one document and as a *commitment* in another is one string, not one claim. R-2's number would be re-measured on a different compiler; ADR-0037's would not |
| pre-publication review × MSRV | R-1's *"a check that asserts existence as a proxy for execution"* against the MSRV file's *"that job is currently vacuous"* | **52** | **Refused inside the band, and it is the near-miss worth naming.** Both are a green step whose evidence is narrower than its name. But R-1's is a **defect** with a stated repair, and the MSRV job's is a **disclosed property** ADR-0029 already recorded and ADR-0037 restates as scope-fencing. Fold them and the atom would assert that a decision's own honest disclosure is a bug. Op 4 links Op 7 and stops there |
| pre-publication review × adapter-default | none | **0** | Nothing in the review touches `unstable-projection`, feature defaults, or either skeleton crate |
| MSRV × adapter-default | both are about the surface a consumer installs at first publish | **28** | **Not a cluster.** One is a compiler floor promised to a consumer; the other is a feature default on two crates that will not be in the release at all (`publish = false`, both). The shared word is *publish* and the shared date is the deadline |

### Why the absence is worth a section rather than a sentence

Wave 7's headline finding was that **ingested file-by-file, it would have produced three
duplicated paragraphs and no owner for any of them.** The instinct that follows is to hunt for
clusters in every wave and, failing to find one, to manufacture the nearest thing. The 52-point
pairing above is exactly what that instinct would have taken. It is refused with its reason
written down, because a wave that collapses a defect into a disclosure has not deduplicated
anything — it has invented a claim neither document makes.

## The dedups that were refused, and the ones that were not offered

| Group | Score | Ruling |
| --- | --- | --- |
| R-3 and R-4 as one *"the busy-timeout experiment, 2026-09"* atom | 40 | **Refused, and it is the wave's first hard call.** They share a directory: `lost_wakeup.rs` is a test in `experiments/busy-timeout-margin/`. `kb-playbook-one-decision-per-adr-title-001`'s deletion test passes both ways — delete the margin table and the lost wakeup still stands; delete the lost wakeup and the margin table is complete. And the readers are disjoint: R-3's is someone weighing a 5,000 ms pragma, R-4's is someone staring at a CI job that stopped and named no rule. `02`, Adjudication 3 |
| R-3's margin table and ADR-0022 §11's falsifier firing as one atom | **70** | **Refused, inside the band, on the `reference` README's own sentence:** *"A conclusion drawn from the measurement… does not belong here."* The number is the reference atom's; what it does to an accepted decision's re-open trigger is a question nobody has answered. Wave 5 made the identical split between ADR-0022 and its experiment. Ops 2 and 13 |
| The three ADR-0022 falsifiers as three open questions | 85 | **Refused — they are one atom.** All three are re-open triggers written into one accepted decision by its own author, and the question they pose is singular: *does ADR-0022 get re-opened, and by whom.* Three atoms would be three records of one unanswered question, and the map would carry three bullets a reader has to reassemble. Op 13 |
| Q3 (a fixture tolerance for transient contention) folded into Op 13's falsifier question | **55** | **Refused inside the band, and it is this wave's most contestable refusal.** Both descend from `busy > 0`. But they have different owners and different answer shapes: Op 13's answer is a superseding ADR against `happenstance-sqlite`'s pragma, Q3's is a CF- capability on the fixture contract that ADR-0034 already says how to mint. `02`, Adjudication 8 |
| Q1 (metadata floor) folded into ADR-0015 as a fifth floor | — | **Not available.** ADR-0015 is accepted. The claim's own text says it *"needs a superseding atom"*, and this wave has none to write |
| R-1 merged into `kb-playbook-verify-referent-report-coverage-001` | 58 | **Refused, fifth wave running, and the population is the discriminator.** That playbook enumerates what it covers and a test suite is not in the list. Its fix is *print the denominator*; R-1's is *assert on the run's output*. Same family, different mechanism |
| R-1 and the citation claim as one *"checks that verify a proxy"* playbook | 50 | **Refused.** The citation claim has an owner at 74 and R-1 has none; folding them would make the merge target's subject strictly wider than its title, and `kb-playbook-one-decision-per-adr-title-001` is the instrument that rejects exactly that |
| The governance ladder merged into `kb-governance-referent-not-reasoning-001` | 35 | **Refused, and below the band.** One layer, two verbs. Editing a record and refuting a finding about one are not the same discipline |
| A new decision atom for the adapter-default resolution | — | **Not offered, and the staged file forbids it first**: *"This is a **resolution of an existing open question**, not a new decision atom."* Verified independently — ADR-0036 already commits the gate, so making two adapters honour it commits nothing new, and a decision atom with no rejected alternative is what `decisions/README.md` calls *"indistinguishable from an accident"* |
| A `product/` or `design/` atom from anything staged | — | **Not offered, and the wave does not invent one.** Nothing here is a persona, a journey, or an interaction pattern for a class of surface |
| Any edit to `crates/**`, `spec/SPECIFICATION.md`, `RUNBOOK.md`, `Cargo.toml` or `references/evaluation/**` | — | **No operation.** Both manifests already carry the shape Op 14 records, and the `crates/happenstance/README.md`:49 wording the MSRV file names as overstated is routed by that file to a backlog story. The KB is catching up to landed code, which is the ordinary shape and not a defect |
| The `phase-7-contract-defects.md:10` mis-citation repaired in place | — | **Out of scope for `.kb/`, and the source says why**: the repair is *"the one permitted in-place change the sentence itself describes"*, in a tree this wave does not write to. Op 5 records the instance; the edit belongs to whoever next touches that document |
