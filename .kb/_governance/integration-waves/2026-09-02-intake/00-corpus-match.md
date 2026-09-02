# Wave `2026-09-02-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus, about the
tree, and about the seven staged files.

This is the corpus's **seventh wave**, and it is unlike all six before it in one structural
respect: **four of its seven files describe a subject the corpus has never held.** Every prior
wave landed inside the contract/ports/specification neighbourhood. This one opens a brand
identity domain, populates `.kb/design/` for the first time since the layer was scaffolded, and
mints the corpus's first decision atoms that are not `ADR-` numbered.

Its hard jobs are four:

1. **Cross-file dedup across a four-file cluster that was staged as four documents and is not
   four atoms.** The trademark gate is stated twice, in two files, almost word for word. The
   lockup's geometry constants are stated twice. The *irregularity reads as an error*
   anti-pattern is stated twice. Ingested file-by-file, this wave would have produced three
   duplicated paragraphs and no owner for any of them.
2. **Deciding where a decision that is not an ADR lives.** `.kb/decisions/` is described by its
   own README as "the ADR corpus", and every one of its twenty-nine atoms carries an
   `ADR-` number. The brand records are numbered `SD-` and one staged file says in terms that
   the two sequences are deliberately separate. `02`, Adjudication 6.
3. **Two verified corrections to staged prose, both about mutable state, and both of which
   change what an atom may say.** `happenstance-neon` and `happenstance-postgres` do *not*
   carry the exposure the PS-3 file attributes to them — they carry a worse one, by a different
   mechanism — and `NeonProjectionStore` is not the `todo!()` stub that file calls it. The
   corpus's own specification already says so at `spec/SPECIFICATION.md`:392.
4. **One verification the wave cannot perform, and says so rather than papering over.** The
   brand source tree the four brand files cite — `references/brand/`, `assets/brand/`,
   `references/seeds/licensing-and-the-commercial-seam.md` — **is not present in this
   worktree.** The provenance section below.

## The corpus, as verified

```
ls .kb/decisions      →  29 atoms (0001–0023, 0029–0034) — 27 accepted, 2 superseded (0002, 0021)
ls .kb/open-questions →  26 atoms, all authority_tier: note — 20 accepted, 6 superseded
ls .kb/reference      →  10 · playbooks 6 · maps 3 · concepts 1 · governance 1
ls .kb/design         →   0 atoms, README only  ← this wave is the layer's first
ls .kb/product        →   0 atoms, README only  ← nothing routes there this wave
                         76 atoms + 9 layer READMEs, before this wave
ls references/adr     →  26 records: 0001–0023, 0029, 0030, 0035
ls references/brand   →  ABSENT from this worktree
ls assets/brand       →  ABSENT from this worktree
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **29** (27 accepted) | **Five are added** — ADR-0035, ADR-0036, SD-0001, SD-0002 and SD-0002's delivery rule. **No body is edited, no `status` is flipped, and no decision is superseded anywhere in this wave** |
| `open-questions/` | 26 (20 accepted) | **Two are created**; **one is resolved** (the `deny.toml` ban — metadata plus a dated section, body verbatim). No other question is touched |
| `reference/` | 10 | **Two are added**, both brand. No existing reference atom is extended — the dating rule, below |
| `design/` | **0** | **Two are added.** The layer has existed since the KB was scaffolded and has never held an atom; its README's template is followed literally |
| `playbooks/` | 6 | **One is added.** `one-decision-per-adr-title` is **cited three times as an instrument and merged into zero times** — sixth wave running |
| `maps/` | 3 | All three inherit work — `decision-map` five rows across two sections, `domain-map` two new domains, `open-questions-index` three bullet changes |
| `governance/`, `concepts/` | 1 each | No bearing. `kb-governance-referent-not-reasoning-001` is cited by the incoming playbook's own frontmatter and is not edited |
| `product/` | 0 | Nothing routes there. The brand material is not a persona and not a journey; `product/README.md` excludes a market segment and a positioning claim in terms |

For authority purposes:

- **Twenty-seven accepted decision atoms exist and the intake touches five of them by name** —
  `kb-decision-0001` (whose `deny.toml` exemption set ADR-0035 amends), `kb-decision-0005` and
  `kb-decision-0006` (the rename, which SD-0001 explicitly declines to supersede),
  `kb-decision-0017` (the owned batch, which is why SQLite sits at the buffered end of PS-2's
  axis) and `kb-decision-0023` (which introduced the `worker` dependency). **Not one of them is
  edited, flipped, or superseded.**
- **Zero supersessions.** Second wave running. Two staged files invite an edit to an accepted
  atom and both refuse it in their own voice: the ADR-0035 file's *"Do not amend or merge into
  `.kb/decisions/0001-async-port-flavours.md`"*, and the SD-0001 file's *"It does not supersede
  ADR-0005, which remains correct about why the rename happened."*
- **One claim asks for a verdict the wave declines to sign**, and it is the same claim wave 6
  declined for the same reason one layer down: the three store-limit numbers. Carried in
  `unresolved`. `02`, Adjudication 9.

## Provenance check

Every path the seven files name was tested against this worktree. Unlike wave 6's check, this
one did not come back clean, and the failures are informative rather than fatal.

```
references/adr/0035-async-trait-through-worker.md                ✓ exists — ADR-0035's number is fixed by an artefact
deny.toml:75-79                                                  ✓ wrappers = [wasm-bindgen-test, worker, worker-macros], citing ADR-0035
deny.toml:57-66                                                  ✓ the different-argument distinction, already in the file's own comment
crates/happenstance/tests/flavours.rs                            ✓ exists — the load-bearing guard
.kb/open-questions/deny-bans-red-on-the-worker-dependency.md      ✓ kb-open-question-worker-async-trait-ban-001, status: accepted
spec/SPECIFICATION.md:4880-4884                                  ✓ PS-3, [PROVISIONAL], SHOULD with the condition, verbatim
spec/SPECIFICATION.md:392                                        ✓ the impl census — and it disagrees with one staged sentence, below
crates/happenstance-testkit/tests/…/mutants.rs:44                ✓ CheckpointOnlyStore, NAME = "CheckpointOnlyStore"
crates/happenstance-sqlite/tests/projection.rs:44                ✓ #![cfg(all(feature = "projection-store", feature = "conformance"))] — TWO
crates/happenstance-sqlite/tests/projection.rs:226               ✓ projection_store_conformance!(SqliteProjectionFixture::new())
crates/happenstance-sqlite/Cargo.toml                            ✓ default = ["event-store"] — the fix landed, and the comment cites PS-3
crates/happenstance-neon/Cargo.toml                              ✗ NOT what the intake says — Correction 1
crates/happenstance-postgres/Cargo.toml                          ✗ same
crates/happenstance-neon/src/projection_store.rs:174-225         ✗ four port methods carry real bodies — Correction 2
.bklg/…/postgres-and-neon-stores/                                ✓ exists — the routed owner is a real project
.bklg/…/deskeleton-and-package-readiness/discover.md             ✗ no such project directory; it is a story of postgres-and-neon-stores
standards/pages/{README,00,10,20,40}.md, xtask/src/lint_pages.rs ✓ every path the page-need playbook cites
references/brand/, assets/brand/                                 ✗ ABSENT from this worktree
references/seeds/licensing-and-the-commercial-seam.md            ✗ ABSENT; references/seeds/ holds two files, neither of them this one
.kb/decisions/                                                   ✓ no 0035-*.md, no 0036-*.md, no sd-*.md; highest ADR atom is 0034
```

### Correction 1 — the neon and postgres exposure is not the sqlite exposure

The PS-3 file's routed residual says *"`happenstance-neon` and `happenstance-postgres` carry the
identical `default = ["event-store", "projection-store"]`."* The default list is identical. The
**mechanism is not**, and the difference runs the wrong way:

```
happenstance-sqlite    projection-store = ["happenstance-core/unstable-projection"]   ← a gated forward
happenstance-neon      projection-store = []
happenstance-postgres  projection-store = []
                       happenstance-core = { …, features = ["std", "unstable-projection"] }  ← unconditional, in [dependencies]
```

Both crates name `unstable-projection` **in their dependency table, outside any feature**, each
with a comment explaining that feature unification is per build and the flag is stated rather
than inherited. So changing `default` on those two crates does not close the gate the way it
closed on `happenstance-sqlite`: `--no-default-features` still enables
`happenstance-core/unstable-projection` in that build. **The residual is real and is larger than
the file describes, and it is a live mutable fact.** That is the strongest single argument for
Op 1 minting an open question rather than the residual riding inside ADR-0036's immutable body,
and Op 1's body is where the corrected mechanism is recorded. `02`, Adjudication 4.

### Correction 2 — `NeonProjectionStore` is not a `todo!()` stub

The same paragraph says *"Both are stubs whose projection bodies are `todo!()`."* True of
`happenstance-postgres` (`projection_store.rs`:112–138, every port method). **False of
`happenstance-neon`**: `begin`, `checkpoint`, `commit`, `reset` and `rollback` at
`projection_store.rs`:174–225 all carry real bodies, and the `todo!()`s sit in three private
request builders (:127, :132, :143) and three free decode functions (:238, :243, :248).
`spec/SPECIFICATION.md`:392 already states this correctly and in more detail than any atom
should restate. **No atom this wave writes asserts an impl census of its own**; Op 1 cites the
specification line and says what it establishes. This is the reference README's pointer rule
applied to a claim rather than to a measurement: the evidence is large, already written down,
and two copies is one that goes stale.

### The brand tree is not in this worktree, and what follows from that

`references/brand/` and `assets/brand/` are untracked in the repository and absent here; so is
`references/seeds/licensing-and-the-commercial-seam.md`. The four brand files therefore cite a
tree the wave cannot open.

Three consequences, and only the third is a limitation:

1. **It does not block the mint.** Wave 6 minted ADR-0034 with no `references/adr/` record at
   all, on wave 5's precedent. An absent long-form record has never blocked an atom in this
   corpus, and the four brand files are **self-contained** in a way ADR-0034's intake was not:
   each carries the decision, the alternatives that lost, the costs acknowledged, and the
   measured evidence inline.
2. **It changes the reference atom's job rather than cancelling it.** `reference/README.md`'s
   pointer case is *"an index into evidence that lives somewhere else"*, and its dating rule is
   *"every atom here is a statement about a moment. Say which moment."* Op 13 is therefore
   authored as a pointer **dated 2026-08-18 and true of the working tree on that date**, which
   is what its own staged file says of itself in its first sentence. That is honest whether or
   not the tree lands, and it is not a mirror anyone must keep current.
3. **The wave verified the brand content against the intake files and against nothing else.**
   Every other claim in this wave was checked against a file in the tree. These were not, and
   could not be. Recorded in `unresolved`.

## Scoring method

Unchanged from all six previous waves — a threshold that moves between waves is not a threshold.
A 0–100 judgement of **subject identity**: would a reader looking for one claim expect to find
the other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms,
  linked, and the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Rules carried forward, each with what it did this wave:

- **A README is not a merge target** (wave 1). Exercised twice — the ADR-0035 file's claim 8
  against `.kb/decisions/README.md` at lexical 80, and the brand-pointer file's SD-numbering
  claim against the same README at 30. Both are the authority being obeyed. **No operation.**
- **A supersession chain is never a merge** (wave 2). Not exercised — no supersession.
- **A decision naming a gap is not the same knowledge as the question that owns the gap**
  (wave 3). Exercised once, and it is what produces Op 1 rather than a sixth paragraph inside
  ADR-0036.
- **An accepted decision atom is not a merge target, however high it scores** (wave 4).
  Exercised on `kb-decision-0001` (ADR-0035 claim 2, **85**) and `kb-decision-0005` (SD-0001
  claim 3, **65**). **Two atoms scored against, zero operations on either.**
- **An ADR number is allocated by the artefact that already carries it** (wave 5). ADR-0035's
  record exists on disk and `deny.toml`:59 already cites it by path. Nothing allocates a number
  for the PS-3 verdict, so it is highest-taken + 1 = **ADR-0036**.
- **An intake file's numeric prefix is a staging sequence and never an ADR number** (wave 6).
  Not exercised destructively — `.kb/_intake/0035-…` happens to name the number it mints, which
  is the first time the prefix has been accurate and is not a reason to start trusting it.
- **New this wave: a record number may yield more than one atom, and the split is stated on
  both.** `SD-0002` holds two decisions — the mark, and a delivery rule that generalises past
  logos entirely. Two atoms, one `adr_id`, adjacent filenames. `02`, Adjudication 7.
- **New this wave: a design atom's boundary is not automatically an open question.** The
  seven-element defence is unfalsified and names the check that would settle it. That belongs in
  the design atom's own boundary section, on `kb-decision-0034`'s precedent of a position naming
  its own superseding trigger inside itself. `02`, Adjudication 8, and it is the wave's most
  contestable call.

## Merge candidates against the existing corpus

| Existing atom | Incoming claim | Score | Disposition |
| --- | --- | --- | --- |
| `kb-decision-0001` | ADR-0035 claims 1, 2, 6 — the ban is a proxy for a property `worker` does not violate | **85** on subject, **0 actionable** | **no operation.** Accepted and immutable, and the staged file's own *"Two things the wave must NOT do"* forbids the merge in terms. ADR-0035 amends the exemption set **from outside**, on `kb-decision-0029`'s precedent against ADR-0004. `depends_on` on Op 2 |
| `kb-decision-0029` | ADR-0035 claim 8 — the amend-without-editing shape | **70** | **link only.** Same *shape*, unrelated *subject*: one is a compiler floor, the other a dependency ban. This is the structural precedent Op 2 cites, not knowledge Op 2 shares |
| `kb-open-question-worker-async-trait-ban-001` | ADR-0035 claim 7 — this resolves it | **95** | **resolve, not merge-in** — Op 4. The answer is a new atom; the question keeps its body verbatim and moves to `superseded`, with a dated section appended. `.kb/open-questions/README.md`:41-45, and wave 6's cf-40 precedent |
| `kb-decision-0023` | ADR-0035 claims 1, 3 — the dependency this exempts, and `flavours.rs` | 55 | **link only.** ADR-0023 took the `worker` dependency and recorded the red gate as its price; ADR-0035 pays it. Same subject, different settling condition — squarely 50–79 |
| `kb-decision-0015`, `kb-decision-0023` | ADR-0035 claim 9 — the three store-limit numbers | 35, 30 | **no operation.** The staged file forbids reading itself as resolving them and routes them to a backlog record. `unresolved` |
| `kb-decision-0017` | PS-3 C1 — SQLite's owned batch is why it sits at the buffered end | **72** | **link only.** ADR-0017 decided the batch's shape; ADR-0036 is the first document to notice that the decision *placed every passing adapter at one end of PS-2's axis*. Same subject, different settling evidence. `depends_on` on Op 3 |
| `kb-decision-0030` | PS-3 C1 — PS-38 and the checkpoint's progress obligation | 30 | **link only.** Adjacent clause family, unrelated question |
| `kb-open-question-postgres-arm-c-structural-cost-…` | PS-3 C3 — Postgres is one of the unbuilt ends | 40 | **link only.** That question asks whether arm C is *expressible*; Op 1 asks what a crate's `default` feature set exposes. Different failure, one crate name in common |
| `kb-open-question-projection-store-batch-has-no-apply-seam` | PS-3 C1 | 45 | **link only, and it is already `superseded`.** No operation on a superseded question |
| `kb-playbook-verify-referent-report-coverage-001` | page-need claim D — the pairing recurs | **40** | **link only, and the staged file asks for exactly that.** Its own frontmatter carries `depends_on: [kb-playbook-verify-referent-report-coverage-001]` and its body calls that atom *"the earlier instance"*. A second instance of a shape has never merged into the first here — waves 4, 5 and 6 each refused it once |
| `kb-playbook-ratchet-gate-landing-001` | page-need claim E — a corpus nobody may edit | 45 | **link only.** The staged file names it as a distinct problem, *"not this one"* |
| `kb-playbook-anchoring-citations-001` | page-need claim B — citing rather than repeating | 35 | **link only** |
| `kb-decision-0005` | SD-0001 claim 3 — the name was **not** chosen for this reason | **65** | **link only, and the non-supersession is the point.** ADR-0005 is accepted; the whole of claim 3 is a statement that this atom does not touch it. Wave 4's rule, applied to the atom that scores highest against an incoming claim it must not receive |
| `kb-decision-0006` | SD-0001 — the bare name went to the typed layer | 30 | **link only** |
| `kb-concept-torn-read-append-boundary-001` | SD-0001 claim 1 — what a DCB boundary is | **50** | **link only, and the direction matters.** That concept explains a boundary hazard; SD-0001 asserts what the *word* claims. Borderline at the band edge, and two atoms is the safe side of it: a copy rule collapsing into a concurrency concept would bury both |
| `.kb/decisions/README.md` | ADR-0035 claim 8; brand-pointer claim 4 | 80, 30 lexical | **no operation, twice.** The authority being obeyed. Wave 1's rule |
| `.kb/open-questions/README.md` | ADR-0035 claim 7 — the resolution procedure | 95 lexical | **no operation.** The procedure is executed as Op 4 rather than recorded |
| `.kb/design/README.md` | brand-lockup C1, C2 — the Pattern/Rejected/Holds-when template | 90 lexical | **no operation.** Ops 8 and 9 are the template filled in, not a claim about it |

## Cross-file clusters

Seven files, and **four genuine cross-file clusters — every one of them inside the brand
group.** Three would have produced duplicated knowledge if the files had been ingested
independently, and that is this wave's single most important finding.

| Cluster | Files | Claims | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-1** — the trademark gate | `brand-identity-commitments`, `brand-where-the-identity-lives` | *Open, and gating* / *Not yet true* | **95** | **one open question, sourced from both.** Op 6 |
| **CL-2** — irregularity reads as an error | `brand-identity-commitments`, `brand-symbol-wordmark-lockup-pattern` | *The decision* / anti-pattern 1 | **88** | **one design atom holds it; the decision cites it.** Ops 9 and 12 |
| **CL-3** — the lockup's constants | `brand-identity-commitments`, `brand-symbol-wordmark-lockup-pattern` | *Geometry* / *Pattern* | **85** | **one reference atom holds the numbers; the design atom cites them.** Ops 7 and 8 |
| **CL-4** — the hex-value colour lesson | `brand-identity-commitments`, `brand-symbol-wordmark-lockup-pattern` | *Palette* / anti-pattern 3 | **80** | **one decision atom holds the lesson, one reference atom holds both hex values.** Ops 7 and 12 — and **not** the design atoms. `02`, Adjudication 5 |

### CL-1 — the wave's clearest collapse

`brand-identity-commitments.md`'s closing section:

> **The trademark search on "happenstance" has not been run.** … That search gates the
> commercial layer, and no part of this identity should be filed, registered or applied to
> physical goods until it returns.

`brand-where-the-identity-lives.md`'s closing section:

> The trademark search on "happenstance" has not been run, and gates any filing, registration or
> application to physical goods.

Two files, one sentence, two staging documents produced the same afternoon. There is no argument
for two atoms, and no argument for folding it into either file's own atom either: the first
file's atom is a set of binding commitments and the second's is a pointer, and an unresolved
gating question is neither. `.kb/open-questions/README.md`'s second bullet — *"a known gap the
work uncovered and did not close, with what is actually true today"* — names it exactly. Op 6
carries both files in `source_paths`.

### CL-2 and CL-3 — the same claim wearing two jobs

The mark file states the equality of the blocks as the **reason for a commitment**:

> Equality of the blocks is load-bearing. An earlier exploration varied the block lengths … and
> it failed — **irregularity does not read as discovery, it reads as an error**.

The lockup file states it as an **anti-pattern that proved real**, with the rendering that found
it. Same knowledge, two altitudes. The corpus already has a rule for this and it is
`.kb/decisions/README.md`'s: *"What does not belong here: the evidence."* The commitment (seven
equal blocks, always) is the decision's; the observation and its rendering are the design atom's;
the decision cites the design atom by id and does not restate the paragraph.

CL-3 is the same move on numbers instead of prose. `0.40em diameter, 0.57× cap height, top edge
0.06em above the cap line, left edge 0.06em inside the advance` appears in both files verbatim.
It lands **once**, in Op 7's reference atom, dated 2026-08-18 with the rest of the geometry; Op 8
and Op 12 both cite it. Three copies of one measurement would have been three things to update
and two that quietly went stale.

## The dedups that were refused, and the ones that were not offered

| Group | Score | Ruling |
| --- | --- | --- |
| The two design atoms (lockup; radial mark) as one "the mark and its lockup" atom | 40 | **Refused, and the instrument is the corpus's own.** `kb-playbook-one-decision-per-adr-title-001`'s test — *"if I deleted one half, would the other half's argument still be complete"* — passes in **both** directions: the footnote-marker mechanism is about a wordmark's final letter, the seven-element resolution is about a symbol standing alone at 16px, and neither needs the other to be complete. The staged file's own title carries the "and" the playbook warns about |
| The SVG delivery rule folded into the mark decision | **70** | **Refused, inside the 50–79 band, with the reason written down.** The source itself says the rule *"generalises past logos to any SVG asset shipped for use on a surface it does not control."* A general commitment filed inside a brand-mark atom is a commitment nobody looking for it will find. Different settling conditions: the mark decision moves if the mark changes; the delivery rule moves if a renderer learns its own surface colour. Ops 11 and 12, adjacent filenames under one `adr_id`. `02`, Adjudication 7 |
| SD-0001 (the name) and SD-0002 (the mark) as one brand-identity atom | 45 | **Refused.** Two records, two subjects — verbal identity and copy rules against visual identity and geometry — and the second cites the first as a premise rather than containing it. `depends_on`, not a merge |
| The geometry and palette measurements inside the mark decision | **75** | **Refused on the decisions README's own sentence**: *"What does not belong here: the evidence. The measurement … is a `reference` atom that this one cites."* The WCAG ratios are dated 2026-08-18 and will be recomputed if a token moves; the commitment *Sun is never used for text on a light surface* will not. Wave 5 made the identical split between ADR-0022 and its experiment |
| The PS-3 verdict and its routed `happenstance-sqlite` finding as two atoms | **85** | **Refused — they are one atom.** The bundling test again: delete the finding and the verdict is complete; delete the verdict and the finding cannot be stated, because it is a defect *in the gate the verdict describes*. One decision with two consequences, which is the playbook's own stated exception, and the staged file carries it as one document under a *"Finding, routed rather than absorbed"* heading |
| The neon/postgres residual riding inside ADR-0036's body | 60 | **Refused, and Correction 1 is why.** Wave 3's rule (a decision naming a gap is not the question that owns the gap) is the standing reason; the deciding one is that the residual is a statement about **mutable manifests that are already wrong in the staged file's own telling**, and an immutable decision body is the worst place in the corpus to keep a fact that is moving. Op 1 |
| The page-need playbook merged into `kb-playbook-verify-referent-report-coverage-001` | 40 | **Refused, and the staged file refuses it first** — it links that atom as `depends_on` and calls itself the second instance. Two instances of one shape is when a corpus writes the shape down as a `concept`, which nothing this wave stages does; it is never when it overwrites the first instance with the second |
| The unfalsified seven-element defence as its own open question | 55 | **Refused — folded into Op 9's boundary section.** `02`, Adjudication 8, and the wave's most contestable call. `kb-decision-0034` is the precedent: a position that names its own superseding trigger inside itself, rather than shipping a companion question that says the same thing one file over |
| A `product/` persona or journey from any brand file | — | **Not offered, and the wave does not invent one.** `product/README.md` excludes a market segment, a positioning claim and an unevidenced sketch in terms. The brand files carry copy rules and geometry, and no researched audience |
| The three store-limit numbers as an atom of any kind | — | **Not offered by any file, and the wave does not invent one.** Second wave running. The ADR-0035 file names the contradiction — numbers *"read off a platform page"* against HS-S0055 AC-001's *"never read off a platform page"* — and says in terms that *"no document has adjudicated that"* and that it surfaces in a backlog record rather than here. `unresolved` |
| Any edit to `crates/**`, `deny.toml`, `spec/SPECIFICATION.md` or `standards/pages/**` | — | **No operation.** `deny.toml` already carries the ratified wrappers entry and cites ADR-0035 by path; `happenstance-sqlite`'s `default` already reads `["event-store"]`. In both cases the code landed first and the KB is catching up, which is the ordinary shape and not a defect |
