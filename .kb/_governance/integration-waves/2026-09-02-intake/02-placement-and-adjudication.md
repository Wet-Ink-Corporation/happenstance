# Wave `2026-09-02-intake` — placement and adjudication

The ordered action plan. Thirteen operations: **five decision atoms created, two reference atoms
created, two design atoms created (the layer's first), one playbook atom created, two open
questions created, one open question resolved.** No decision atom's body is edited, no decision
atom's `status` is flipped, and no supersession is performed anywhere in this wave.

Order is load-bearing in six places, and the rule is the one every wave uses: an atom a later op
links to is created first, and mutating ops run strictly serially.

```
Op  1  create_new     .kb/open-questions/projection-store-in-adapter-default-features.md
Op  2  create_new     .kb/decisions/0035-async-trait-through-worker.md
Op  3  create_new     .kb/decisions/0036-the-projection-port-ships-gated.md
Op  4  merge_existing .kb/open-questions/deny-bans-red-on-the-worker-dependency.md   (resolve)
Op  5  create_new     .kb/playbooks/making-the-need-a-page-answers-declared-singular-checkable.md
Op  6  create_new     .kb/open-questions/trademark-search-gates-the-commercial-layer.md
Op  7  create_new     .kb/reference/brand-mark-geometry-and-palette-2026-08.md
Op  8  create_new     .kb/design/symbol-annotates-the-wordmark.md
Op  9  create_new     .kb/design/a-radial-mark-between-two-glyph-collisions.md
Op 10  create_new     .kb/decisions/sd-0001-the-name-names-the-boundary-of-what-occurred.md
Op 11  create_new     .kb/decisions/sd-0002-standalone-svg-carries-one-colourway.md
Op 12  create_new     .kb/decisions/sd-0002-the-mark-and-the-rules-that-bind-it.md
Op 13  create_new     .kb/reference/brand-identity-source-locations-2026-08.md
```

- **Op 1 before Op 3** — ADR-0036's `related` names the question that owns its routed residual,
  rather than describing the residual a second time in an immutable body.
- **Op 2 before Op 4** — the resolution names `kb-decision-0035`.
- **Op 6 before Ops 12 and 13** — both brand atoms link the trademark gate.
- **Op 7 before Ops 8, 9 and 12** — three atoms cite one set of measurements and none restates it.
- **Ops 8 and 9 before Op 12** — the mark decision cites the design judgement it rests on.
- **Ops 10 and 11 before Op 12** — SD-0002 `depends_on` SD-0001, and `related`s the delivery rule.

**Op 4 is the wave's only mutating operation**, and it touches an `open_question`. No decision
atom in the corpus is opened for writing at any point in this wave.

---

## Standing choices, applied to every atom in the wave

### 1. One atom per record, and the long-form record keeps the evidence

Unchanged from waves 2–6, and this wave is where it stops being about ADRs alone.

- **ADR-0035** applies it cleanly: `references/adr/0035-async-trait-through-worker.md` exists on
  disk and `deny.toml`:59 already cites it by path. Link the atom; cite the record by `file:line`.
- **ADR-0036** applies it aspirationally — no `references/adr/` record exists yet. Wave 6's
  ADR-0034 is the precedent that this does not block the mint.
- **SD-0001 and SD-0002** apply it to a record the wave cannot open. Adjudication 6, and the
  provenance section of `00`.
- **Op 11 breaks the "one atom per record" half of it deliberately**, and says so on both atoms.
  Adjudication 7.

### 2. `status`, and the value the schema still does not have

ADR-0036 records a port that is deliberately *not frozen*, with a `[PROVISIONAL]` clause above
it and a bar it has not met; the brand atoms record commitments whose long-form records are not
in this tree. `KbFrontmatter`'s `status` enum still has no "accepted, provisional", and
`kb-open-question-adr-status-vocabulary-001` records exactly that gap. The convention that
question observes is followed rather than answered: `status: accepted`, with every qualification
and its falsifier folded into `summary` and into a body heading. **No edit to the
status-vocabulary atom.** Sixth wave running.

### 3. `authority_tier`, `phase`, `reversibility`

`authority_tier: decision` for Ops 2, 3, 10, 11 and 12, mirroring all twenty-nine siblings.
`authority_tier: guideline` for Op 5, mirroring all six playbooks. `authority_tier: note` for
both reference atoms and both open questions. **`authority_tier: design` for Ops 8 and 9** —
the first two atoms in the corpus to carry it, and it is what makes `design/` a layer rather
than a directory, exactly as `.kb/README.md` says.

`phase` is the frontmatter value this wave chose rather than read, and each choice is stated:

| Op | `phase` | Why |
| --- | --- | --- |
| 2 | **9** | The `worker` dependency is phase 9's (`RUNBOOK.md`'s table, *Cloudflare Durable Object*), the record's own Context opens *"Phase 9 replaced the hand-written Durable Object stand-in"*, and the call was made at HS-P0013's gate |
| 3 | **6** | PS-2 and PS-3 are phase 6's clauses and phase 6's proof artefact is *"`CheckpointOnlyStore` **failing** the projection suite, and two unlike batch shapes passing it"* — which is PS-2's bar verbatim. This verdict is that artefact evaluated and found half-discharged. The `0.2.0` milestone that forced the evaluation sits after phase 7; the *clause* is phase 6's, and a decision atom's `phase` has always named the phase that owned the question |
| 10, 11, 12 | **null** | Brand work is not a phase of this library. `RUNBOOK.md`'s fifteen phases are contract, adapters, publication and replication; none of them owns a wordmark. `null` is the schema's own value for "no phase owned this", and inventing a sixteenth phase to avoid it would be worse |

`reversibility` per op, in the table below. Two are worth flagging here: **Op 2 is `medium`, not
`low`**, and the reason is claim 4 — the exemption removes a legible guard on two edges and
leaves an unskippable compile-time one (`crates/happenstance/tests/flavours.rs`) on all of them,
so reversing it costs a `deny.toml` line and a new record rather than a redesign. **Op 12 is
`low`**, because the mark is already rendered into eight SVG files, a favicon and a wordmark
delivered as outlined paths.

### 4. `source_paths` keeps the intake path

Every atom this wave writes or amends carries its originating `.kb/_intake/…` path, even though
the ingest clears `_intake` afterwards. The path documents provenance; the git history holds the
file. Wave 1's rule, unchanged — and it is why **five ops list two intake files each** (CL-1
through CL-4, plus Op 9's cross-file anti-pattern).

`source_paths` also carries paths that do not resolve in this worktree — `references/brand/`,
`assets/brand/`, `references/seeds/licensing-and-the-commercial-seam.md`. That is deliberate and
it is not a defect: `source_paths` is free-form provenance, `validate --kb` resolves atom **ids**
and not paths, and every wave since the first has written `.kb/_intake/…` paths that the same
commit deletes. What would be a defect is an atom *quoting content* from a file the wave never
opened, and none does — the brand atoms are authored from the staged files, which carry the
content inline.

### 5. `last_reviewed: 2026-09-02` on everything this wave touches

Including Op 4's resolved question, where it is one of only four fields that move.

### 6. What is *not* extracted

**No `crates/**` change.** **No `spec/SPECIFICATION.md` edit** — PS-3 keeps `[PROVISIONAL]`,
PS-2 is `[FROZEN]` and is applied rather than amended, and the PS-3 file says both in its own
words. **No `deny.toml` edit** — the wrappers entry is already there and already cites ADR-0035.
**No `standards/pages/**` edit** — the RP- rules are that tree's, and Op 5 cites them.
**No layer README is edited**, though two claims score against `.kb/decisions/README.md`: that
README is the authority being obeyed.

Two mutable non-atom obligations are named by the intake and are **not** this wave's `.kb/` ops.
Recorded here so a reader who finds them unperformed can see they were not forgotten:

- **The brand tree itself.** `references/brand/` and `assets/brand/` are untracked and absent
  here. Op 13 is a pointer *to* them and does not create them. Landing that tree is a
  source-control step, not a KB op.
- **HS-S0055's store-limit numbers.** Adjudication 9, and `unresolved`.

### 7. Reciprocal links are the Maps phase's, not the atom author's

Every new atom is authored with **outbound-only** links. Where a mutual edge is wanted —
`kb-decision-0036` ↔ `kb-open-question-adapter-default-projection-feature-001`,
`kb-decision-sd-0002` ↔ the two design atoms — the second half is wired by the Maps phase, as in
waves 4, 5 and 6.

---

## Adjudication 1 — ADR-0035 is transcribed, not signed

Wave 6 met this exact claim and refused it. The refusal is worth quoting, because what changed
is not the argument:

> Authoring that choice here would be the wave ratifying a dependency exemption against a binding
> constraint, with no record, no human, and a red gate as the forcing pressure.

All three conditions are now false. **There is a record** —
`references/adr/0035-async-trait-through-worker.md`, which names ADR-0001 as the decision it
amends, states the exemption's argument and its known gap, and carries the failing `cargo deny`
transcript. **There was a human** — the maintainer made the call at HS-P0013's gate on
2026-08-20, and the staged file records that two earlier passes (HS-S0059, and wave 6 itself)
each declined to make it for him. **The gate is green** — `deny.toml`:75-79 carries the wrappers
entry, with a `reason` string naming ADR-0035 and a fifteen-line comment above it distinguishing
this case from the `wasm-bindgen-test` one.

So Op 2 is the same class of operation as wave 6's ADR-0034: **a decision that arrives taken,
with its loser named and its cost stated, and the wave transcribes it.** The three things Op 2's
body must carry, because they are what make it a decision rather than a note:

1. **The two arguments must not be collapsed** (claim 5). `wasm-bindgen-test` is exempt because
   it is a dev-dependency in no published artifact. `worker` is a normal dependency of
   `happenstance-cloudflare` and **does ship**; it is exempt because it uses `#[async_trait]`
   for its own `DurableObject` trait, which this workspace implements but never derives a port
   from. Reusing the first sentence for the second case would be reasoning about a different
   situation with the same words — and `deny.toml`'s own comment already refuses to.
2. **The guard that matters is named** (claim 4). `cargo deny` is an optional probed gate step;
   `crates/happenstance/tests/flavours.rs` is a compile-time obligation on every target. The
   exemption removes the legible guard on two edges and leaves the unskippable one on all.
3. **The known gap is recorded rather than hidden** (claim 10). A `wrappers` entry pins a *name*,
   so a `worker` major bump that changed how it uses `async-trait` would not be caught. That is a
   property of `wrappers` and it has been true of the `wasm-bindgen-test` entry since the ban's
   first run.

**And `kb-decision-0001` receives no operation of any kind** — not a `related` edge added to its
own frontmatter, not a line, not a `last_reviewed` bump. The edge is outbound from Op 2, as a
`depends_on`, on the shape `kb-decision-0029` established against ADR-0004: an amendment carries
the edge, and the amended atom stays byte-identical with `superseded_by: null`.

---

## Adjudication 2 — the numbers

**ADR-0035 is fixed by an artefact**, twice over: the record exists, and `deny.toml` cites it.
No judgement required.

**The PS-3 verdict has no allocated number and takes ADR-0036.** Nothing in `.bklg/` or
`RUNBOOK.md` names a number for it; the staged file says only *"numbered 0036 or above"*, which
names a floor and not a number. Wave 5's rule applies: highest-taken + 1.

| Decision | Number | Fixed by |
| --- | --- | --- |
| `async-trait` through `worker` | **ADR-0035** | `references/adr/0035-…`, and `deny.toml`:59 |
| The projection port ships gated | **ADR-0036** | highest-taken + 1 |
| The name's meaning and copy rules | **SD-0001** | `references/brand/SD-0001-what-happenstance-means.md`, named by two staged files |
| The mark, and the SVG delivery rule | **SD-0002** | `references/brand/SD-0002-the-mark.md`, named by three staged files |

**0024–0028 are still not backfilled**, and all five remain reserved by planned projects, exactly
as wave 6 found them. Taking one would collide with a planned initiative.

**0035 is highest-taken even though no atom carries it**, which is new. The `.kb/decisions/`
listing tops out at 0034; `references/adr/` tops out at 0035. The record is the allocation — that
is wave 5's rule stated forward rather than backward — so ADR-0036 is the next free number and
0035 is not a gap this wave may claim for something else.

---

## Adjudication 3 — the PS-3 verdict and its routed finding are one atom

The staged file carries them as one document, and the wave agrees at **85**. The instrument is
`kb-playbook-one-decision-per-adr-title-001`'s own test — *"if I deleted one half, would the
other half's argument still be complete"* — and it answers asymmetrically, which is the answer:

- **Delete the finding.** The verdict is complete: PS-2's bar is a conjunction, part 1 is
  discharged by `CheckpointOnlyStore`, part 2 is not, so the port ships gated and PS-3 keeps its
  marker.
- **Delete the verdict.** The finding is *unintelligible*. "`cargo add happenstance-sqlite`
  turned the gate on without the consumer naming it" cannot be stated without first establishing
  that there is a gate, that it is off by default, and why. The finding is a defect **in the
  mechanism the verdict describes**.

That is the playbook's stated exception — *"one decision with two consequences, not two decisions
wearing one title"* — and it is the shape wave 6 found in ADR-0023 and its ES-6 cluster.

**Two things Op 3's body must carry that a summary would lose.**

The **struck-through paragraph stays struck through rather than deleted.** The staged file's own
sentence is the argument: *"a finding that was deferred on a mistaken cost should show the
mistake, not a tidy conclusion."* The mistaken cost was that removing `projection-store` from
`default` would stop `tests/projection.rs` running bare; the fact that refutes it is that the
file is gated on `all(projection-store, conformance)` and `conformance` was never in `default`,
so that family already ran 0 tests bare and 24 under `--all-features`. Verified at
`crates/happenstance-sqlite/tests/projection.rs`:44. This is `kb-governance-referent-not-reasoning-001`'s
discipline applied to a cost estimate rather than to a citation.

And **the feature keeps its name, deliberately.** `event-store`/`projection-store` names *roles*
and is shared by three adapters; `unstable-projection` names *maturity* and already lives on
`happenstance-core`'s own gate, which the adapter flag forwards. Renaming one adapter would
fragment a three-crate vocabulary to re-spell a signal that is not missing. That is a rejected
alternative with a stated reason, which is what `.kb/decisions/README.md` requires and what a
summary of the verdict would have dropped.

---

## Adjudication 4 — the neon/postgres residual is a question, not a paragraph

The staged file routes this residual to a backlog project and stops. Wave 6's precedent cuts
*against* minting a question for exactly that shape — WF-11's sub-question 2 was *"deferred by
name to a project. A named backlog owner is a task, so this does not mint a new open question."*
This wave mints one anyway, and the three reasons are cumulative.

**First, wave 3's rule.** A decision naming a gap is not the same knowledge as the question that
owns the gap. WF-11's residual had a home — it rode in a reference atom's body and in a resolved
question's dated section. This one has none: no existing open question concerns adapter feature
surfaces, and `kb-open-question-postgres-arm-c-structural-cost-…` is about whether arm C is
*expressible*, which is a different failure sharing one crate name.

**Second, and it is the deciding reason: the residual is a live mutable fact that is already
wrong in the staged file's own telling.** `00`, Correction 1. The file says the two crates carry
*"the identical `default`"* and infers *"the same exposure"*. The default list is identical; the
mechanism is not, and it runs the wrong way:

```
happenstance-sqlite    projection-store = ["happenstance-core/unstable-projection"]
happenstance-neon      projection-store = []
happenstance-postgres  projection-store = []
                       happenstance-core = { …, features = ["std", "unstable-projection"] }   ← in [dependencies]
```

Removing `projection-store` from `default` closed the gate on `happenstance-sqlite` because that
feature *was* the forward. It would close nothing on the other two, which enable
`happenstance-core/unstable-projection` unconditionally, outside any feature, each with a comment
explaining that the flag is stated rather than inherited because feature unification is per
build. **The lever the file names is not the lever**, and that changes the question a future
owner has to answer.

A fact in that state does not belong in an immutable decision body. Op 1 is mutable, is the
layer whose README asks for *"the verified current state, and where you verified it"*, and can be
corrected when the manifests move. Op 3 links it and says nothing about `Cargo.toml` beyond that
the residual is owned elsewhere.

**Third, `00`'s Correction 2 makes the same point about the other supporting fact.** The file
calls both crates *"stubs whose projection bodies are `todo!()`"*. That is true of
`happenstance-postgres` and false of `happenstance-neon`, whose five port methods carry real
bodies at `projection_store.rs`:174-225. **Op 1 asserts no impl census of its own** — it cites
`spec/SPECIFICATION.md`:392, which already carries one, in more detail than an atom should
restate. The reference README's pointer rule, applied to a claim.

The owner the file names is real and is kept: `.bklg/from-contract-to-published-library/postgres-and-neon-stores/`
exists. The path the file cites for the precedent — `deskeleton-and-package-readiness/discover.md`
— is a *story* of that project rather than a project directory, and Op 1 names the project.

---

## Adjudication 5 — CL-4, and the anti-pattern that is not a design atom's

Three anti-patterns arrive in the lockup file's *Anti-patterns that proved real* section, and
they do not all go to the same place. `design/README.md` says anti-patterns proved real in the
build belong inside the pattern's own design atom — but that rule presumes the anti-pattern is
*about the pattern*, and one of the three is not.

| Anti-pattern | Home | Why |
| --- | --- | --- |
| **Irregularity to signal discovery** | Op 9 (design) | It is about the radial mark's element geometry and was found by rendering it. **CL-2**: the mark file states the same finding as the *reason for a commitment*, and Op 12 cites Op 9 rather than restating the paragraph |
| **Judging a symbol at display size only** | Op 9 (design) | It is about *"both collisions"* — it has no meaning detached from the two it describes. Not a playbook: `playbooks/README.md` excludes the one-off, and this is one build with two instances |
| **Deriving a colour from its hex value** | **Op 12 (decision) for the lesson, Op 7 (reference) for the numbers** | It is about colour perception, not glyph geometry, and it is the stated reason one palette token has the value it has |

The third is the wave's finest cut, and the argument for it is in the mark file's own Palette
section, which states the lesson **beside the commitment it justifies**: Ink was `#191512` in
the first cut and read as plain black, because seven points of red over blue is below the
threshold at which hue is perceived in text — *"A colour justified by its hex value rather than
by being looked at is not yet a decision."* That sentence is why the token is `#2A211B`.

So: the **lesson** goes to Op 12, one sentence, as the justification of a palette the decision
adopts. Both **hex values and the measured ratios** go to Op 7, where `reference/README.md`'s
dating rule can hold them. And **neither design atom receives it**, because a design atom
records the pattern chosen for a *class of surface*, and "look at the colour" is not a pattern
for a class of surface — it is one calibration finding attached to one token, which is the
`reference`-or-nothing test `playbooks/README.md` names.

---

## Adjudication 6 — where a decision that is not an ADR lives

This is the wave's structural question, and three placements were available.

**Rejected: a new `.kb/brand/` layer.** `.kb/README.md` is explicit that the pipeline routes into
layers by name and that *"a layer that has to be invented mid-wave gets invented differently each
time"*. Every scaffolded layer ships a README stating its contract; a layer invented in an
adjudication file has none, and the next wave would have to guess at it. Rejected without
argument on the other side.

**Rejected: `concepts/` or `playbooks/`.** Both READMEs exclude a commitment in identical terms,
and `.kb/decisions/README.md` states the reason: *"those two carry no immutability and a
commitment that can be edited is not a commitment."* *"Seven blocks, always"* and *"a standalone
SVG must carry one fixed colour"* are commitments by the corpus's own test — a future change
would need a decision to reverse them.

**Taken: `.kb/decisions/`, with `adr_id: SD-0001` / `SD-0002` and an `sd-` filename prefix.**

The decisions README calls the layer "the ADR corpus" and says each atom carries `adr_id`. It
does not say `adr_id` must match `ADR-\d{4}`, and the schema types it `string`. What licenses
the value is the repository's own statement, in `brand-where-the-identity-lives.md`:

> Brand decisions are numbered `SD-` and live in `references/brand/`, deliberately outside the
> `ADR-` sequence in `references/adr/`. … `SD-` records carry the same structure — the decision,
> what it does not claim, why each alternative lost, the costs, and an evidence table — **and are
> cited from `.kb/` atoms exactly as ADRs are.**

That is a decision about numbering already taken, by the same hand that took the brand
decisions, and this wave's job is to honour it rather than to relitigate it. `adr_id` names *the
record*, and the record is `SD-0001`. The filename prefix keeps the numeric ADR sequence
unpolluted in a directory listing — `sd-0001-…`, `sd-0002-…` sort after `0036-…` and read as a
parallel series, which is what they are.

**What this costs, stated rather than discovered later.** `kb-map-decision-001`'s own summary
describes itself as *"one row per decision atom in `.kb/decisions/`, its ADR number, status,
phase"*, and three of this wave's five decision rows will carry an `SD-` number and a null
phase. The map gains a section for them rather than a new column; the Maps phase inherits that,
and it is why `mapsImpact.decisionMap` is set on Ops 10, 11 and 12 as well as on 2 and 3. The
alternative — a second map atom for brand decisions — would split the supersession graph across
two files on the day the first brand supersession happens, which is the failure the decision map
exists to prevent.

---

## Adjudication 7 — one record, two atoms

`SD-0002` yields **two** decision atoms, and this is the first time any record in this corpus has.
The `00` table scores the fold at **70**, inside the 50–79 band where the corpus's own rule is
"two atoms, linked, and the reason written down". Here is the reason.

**The scope differs, and the source says so.** The mark decision binds *anything carrying the
name*. The delivery rule's own closing sentence is *"This generalises past logos to any SVG asset
shipped for use on a surface it does not control."* A repository-wide rule about SVG assets,
filed inside a brand-mark atom, is a rule nobody looking for it will find — and this repository
ships SVG from `assets/` into a README, a docs.rs header and a crates.io card, none of which is
a logo question.

**The settling conditions differ.** The mark decision moves if the mark changes. The delivery
rule moves if a rendering context learns the colour of the surface it was placed on, which is a
platform fact and not a brand fact.

**The deletion test passes both ways.** Delete the delivery rule: the mark decision's eighth
commitment — *assets ship one fixed colour per file* — still stands as a commitment; only its
reason moves to a cited atom, which is what citation is for. Delete the mark decision: the
delivery rule is complete, because its failure mode (a light page on a dark-mode machine
rendering the wordmark cream on cream, invisible, with no error) and its rejected alternative
(the embedded `@media (prefers-color-scheme: dark)`) are both stated in its own terms.

**What the split must not become.** Two atoms sharing one `adr_id` is honest only if both say so.
Op 11's body opens by naming the record and the half of it this atom carries; Op 12's rules list
carries the eighth commitment and cites Op 11 by id for its reasoning. The two filenames are
adjacent — `sd-0002-standalone-svg-carries-one-colourway.md`, `sd-0002-the-mark-and-the-rules-that-bind-it.md`
— so a directory listing shows the relationship a frontmatter key cannot.

---

## Adjudication 8 — the unfalsified boundary stays inside the design atom

The wave's most contestable call, and the extract offered both shapes.

The seven-element defence against the brightness-glyph collision is, in the staged file's own
words, *"reasoned and rendered but not tested on a stranger"*, and it names the check that would
settle it: show the mark at 16px to developers who have seen neither it nor the record, and ask
what they think it is. Until then the collision is *"mitigated and unfalsified rather than
closed."*

**The case for an open question**, which is real: `open-questions/README.md`'s job is to make an
absence legible, and an unfalsified defence against a near-universal OS glyph is a genuine
absence with a named falsifier.

**Why it loses, on three grounds.**

`kb-decision-0034` is the precedent, and it is close: an accepted position taken on three data
points, carrying `reversibility: high` and naming phase 10's `POLL_BUDGET` as *"the evidence that
would supersede this decision"* — **inside its own body**, not in a companion question. A
position that names its own falsifier is stronger than a position plus a separate atom saying
the position might be wrong, because the two can drift and only one of them is read by someone
who arrives at the position.

`design/README.md` asks for the boundary as a *required section* of every design atom — *"the fit
conditions … and where it stops holding"* — and calls the rejected alternative and the boundary
*"the load-bearing parts"*. Moving the boundary out would leave the layer's first atom missing
the half its own README says matters most.

And `open-questions/README.md` excludes a task in terms: *"Work that someone is expected to do is
a backlog item in `.bklg/`, not a KB atom."* The settling check here **is** a task — show a
rendering to some developers — with no fork, no two named shapes, and no decision waiting on it.
Op 9 carries it as the boundary it is, with the fallback the file already names (tangential
elements, rejected on the brief, kept as the documented fallback if seven proves insufficient),
so a reader who runs the check knows what the answer would change.

**The falsifiable version of this call.** If the stranger test comes back and the mark reads as a
brightness glyph, that is evidence a *new* design atom records, superseding Op 9. It is not
evidence that Op 9 should have been a question — a question would have recorded the same
uncertainty with less of the reasoning attached to it.

---

## Adjudication 9 — the two claims the wave declines

**The three store-limit numbers**, for the second wave running. The ADR-0035 file names the
contradiction — HS-P0013's run-2 review found the numbers read off a platform page, while
HS-S0055 AC-001 says in terms *"never read off a platform page"* — and then says outright that
*"no document has adjudicated that, ADR-0023 did not, and ADR-0035 does not. It is still owed,
and it surfaces in HS-S0055's own record rather than here."*

Nothing staged in this wave adjudicates it either, and nothing staged supplies the evidence that
would. Wave 6 carried it in `unresolved` and so does this one. Two waves is not yet a pattern
worth minting a question for, and minting one now would give the KB a home for a dispute the
backlog has already assigned an owner — which is the *"a resolved decision wearing a question
mark"* failure inverted: an owned backlog item wearing an open question's clothes.

**The brand tree's absence.** The wave verified every brand claim against the staged files and
against nothing else, because `references/brand/`, `assets/brand/` and
`references/seeds/licensing-and-the-commercial-seam.md` are not in this worktree. That does not
block any op — `00`'s provenance section gives the reasoning and the ADR-0034 precedent — but it
is a verification the wave did not perform and could not, and it is carried in `unresolved`
rather than left for a reader to discover from a broken link.

---

## The operations

### Op 1 — `create_new` · `.kb/open-questions/projection-store-in-adapter-default-features.md`

**Classification:** extends · **Sources:** `ps-3-projection-port-ships-gated.md` (C3)
**Maps:** open-questions index ✓ · domain map ✓ (contract ports & conformance) · decision map ✗

```yaml
id: kb-open-question-adapter-default-projection-feature-001
title: Whether the projection port stays in happenstance-neon's and happenstance-postgres's default features
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0036 ships ProjectionStore behind an off-by-default unstable-projection gate, and
  happenstance-sqlite left projection-store out of its default set on 2026-09-02 so that
  cargo add would stop walking around it. happenstance-neon and happenstance-postgres still
  carry default = ["event-store", "projection-store"], and what is not decided is whether
  they should. The exposure is not the one happenstance-sqlite had, and the difference runs
  the wrong way: both crates name happenstance-core = { features = ["std",
  "unstable-projection"] } in their dependency table, outside any feature, so unlike
  happenstance-sqlite's gated forward, changing default closes nothing on its own —
  --no-default-features still enables the core gate in that build. Latent rather than live
  today, because neither crate is published; spec/SPECIFICATION.md:392 is the impl census and
  this atom does not restate it. Owner: the postgres-and-neon-stores project, whose
  deskeleton work already contemplates shipping happenstance-neon with projection-store off
  by default. Forced before either crate is published, which is when a default nobody
  re-read becomes a consumer's problem.
depends_on: []
related:
  - kb-decision-0017
source_paths:
  - .kb/_intake/ps-3-projection-port-ships-gated.md
  - crates/happenstance-neon/Cargo.toml
  - crates/happenstance-postgres/Cargo.toml
  - crates/happenstance-sqlite/Cargo.toml
  - crates/happenstance-core/Cargo.toml
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-02
```

**Body shape:** *What is true today* (the three manifests, quoted, with the mechanism
difference) · *What is not decided* (whether the flag leaves `default`, and whether the
unconditional dependency-table enablement is the thing that actually needs changing) · *What
forces it* (publication of either crate) · *Ordered sub-questions* (1. is `default` even the
lever, given the unconditional feature on the dependency line; 2. if the two are changed, does
`happenstance-sqlite`'s forwarding shape become the pattern for all three; 3. does the answer
differ for a crate whose projection body is real (`happenstance-neon`) and one whose is
`todo!()` (`happenstance-postgres`)).

**Rationale:** Adjudication 4. Created rather than folded into Op 3 because the state it records
is mutable and partly wrong in the staged file's telling, and an accepted decision body is the
one place in the corpus that cannot be corrected.

---

### Op 2 — `create_new` · `.kb/decisions/0035-async-trait-through-worker.md`

**Classification:** requires-new-decision (arriving taken) · **Sources:** `0035-async-trait-through-worker.md`
**Maps:** decision map ✓ · domain map ✓ (contract ports & conformance) · open-questions index ✓ (via Op 4)

```yaml
id: kb-decision-0035
title: async-trait is exempted where it is reached through worker
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0035
reversibility: medium
phase: 9
supersedes: null
superseded_by: null
summary: >-
  deny.toml's async-trait ban gains two wrappers, worker and worker-macros, amending
  ADR-0001's exemption set without touching ADR-0001's body — the shape ADR-0029 used against
  ADR-0004. The ban is a proxy for a property, not an end in itself: ADR-0001 forbids
  #[async_trait] because it injects + Send and forecloses the wasm32/Workers target, and
  worker does not violate that property, using the macro for its own DurableObject trait,
  which this workspace implements without deriving any happenstance port from it. The two new
  wrappers rest on a different argument from the existing wasm-bindgen-test entry and the
  distinction is kept rather than collapsed: that one is a dev-dependency present in no
  published artifact, while worker is a normal dependency of happenstance-cloudflare and does
  ship. The load-bearing guard was never cargo deny, which is an optional probed gate step,
  but crates/happenstance/tests/flavours.rs, which instantiates every typed-layer entry point
  against a genuinely !Send store and stops compiling the moment a Send bound reaches the
  chain; the exemption removes the legible guard on two edges and leaves the unskippable one
  on all. Exempting by name is what keeps the ban real — a third route into the graph still
  fails the gate until someone decides it should not. Known gap, recorded rather than hidden:
  a wrappers entry pins a name, so a worker major bump changing how it uses async-trait would
  not be caught, which is a property of wrappers and has been true of the wasm-bindgen-test
  entry since the ban's first run. Resolves
  kb-open-question-worker-async-trait-ban-001. Decided at HS-P0013's gate on 2026-08-20,
  after two earlier passes named both shapes and declined to choose.
depends_on:
  - kb-decision-0001
related:
  - kb-decision-0023
  - kb-decision-0029
  - kb-open-question-worker-async-trait-ban-001
source_paths:
  - .kb/_intake/0035-async-trait-through-worker.md
  - references/adr/0035-async-trait-through-worker.md
  - deny.toml
  - crates/happenstance/tests/flavours.rs
  - xtask/src/main.rs
last_reviewed: 2026-09-02
```

**Body shape:** *Context* (the phase-9 `worker` dependency, and the red `cargo deny check bans`
transcript inline — four lines, not a table) · *Decision* (the two wrappers, and the amendment
that does not touch ADR-0001) · *Why the exemption is safe* (claims 2, 3, 4 — the proxy, the
property, and `flavours.rs`) · *Why the two arguments are not collapsed* (claim 5) · *Known gap*
(claim 10) · *Alternatives rejected* (refuse and let the ban stay red, with the exception
recorded — the shape wave 6's question named and this decision declines, and its cost: a gate
step that can never be green while the dependency stands, which `publish-ready-crate`'s AC-012
and its project's DoD both forbid).

**Rationale:** Adjudication 1. `depends_on: [kb-decision-0001]` carries the amendment edge, on
`kb-decision-0029`'s precedent; ADR-0001 stays byte-identical with `superseded_by: null`.

---

### Op 3 — `create_new` · `.kb/decisions/0036-the-projection-port-ships-gated.md`

**Classification:** requires-new-decision (arriving taken) · **Sources:** `ps-3-projection-port-ships-gated.md`
**Maps:** decision map ✓ · domain map ✓ (contract ports & conformance) · open-questions index ✗

```yaml
id: kb-decision-0036
title: The projection port is not frozen at 0.2.0 and ships behind unstable-projection
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0036
reversibility: medium
phase: 6
supersedes: null
superseded_by: null
summary: >-
  ProjectionStore is not frozen at 0.2.0. It ships behind the off-by-default
  unstable-projection feature in happenstance-core and happenstance, forwarded as
  projection-store on adapters, with its semver exemption documented on the module. This is
  PS-3's SHOULD evaluated against PS-2's [FROZEN] two-part bar, part by part, and it is the
  first time anything in the tree has applied that bar to a real adapter set. Part 1 is met:
  CheckpointOnlyStore exists as a registered Defect and demonstrably fails the suite, so the
  rule rejects something. Part 2 is not: exactly one storage adapter has run
  projection_store_conformance!, SqliteProjectionStore against a real temporary file, and it
  fails both halves — SQLite's Batch is an owned write set under ADR-0017, so it sits at the
  buffered end rather than the live-transaction end, and no cannot-hold-across-await adapter
  has passed at all. Testkit fixtures and the outside-projection-adapter example are worth
  having and are not adapters; counting them is the monoculture PS-2's Rejects clause names.
  The rejected arm is freezing now, and its cost is asymmetric and unrecoverable: a frozen
  port is a semver promise, a published version can be yanked but never removed, and the axis
  the port is most likely to be wrong about is the one no passing adapter occupies. The cost
  of the arm taken is accepted and stated: a consumer must name a feature to get a projection
  store at all, and cargo-semver-checks will not police the surface. PS-3 keeps [PROVISIONAL]
  — a satisfied SHOULD is not a moved marker — and PS-2 is applied, not amended. Carries the
  routed finding that happenstance-sqlite's default set forwarded the gate on, defeating
  off-by-default for the one crate a consumer installs; resolved 2026-09-02, default is now
  ["event-store"] alone, and the flag keeps its name because event-store/projection-store
  names roles across three adapters while unstable-projection names maturity.
depends_on:
  - kb-decision-0017
related:
  - kb-decision-0030
  - kb-open-question-adapter-default-projection-feature-001
source_paths:
  - .kb/_intake/ps-3-projection-port-ships-gated.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs
  - crates/happenstance-sqlite/tests/projection.rs
  - crates/happenstance-sqlite/Cargo.toml
  - crates/happenstance-core/Cargo.toml
  - references/evaluation/projection-batch-shape-evidence.md
last_reviewed: 2026-09-02
```

**Body shape:** *Decision* · *PS-2's bar, evaluated* (part 1 met, part 2 not, with the two named
ends and which impl sits where) · *The arm that lost, and what it costs* · *What this does not
decide* (the marker does not move; PS-2 is untouched; the other two adapters' defaults are not
this decision's) · *Finding, routed rather than absorbed* (the `happenstance-sqlite` default,
the struck-through cost kept struck, the same-day resolution, and the naming argument) ·
*Evidence* (cited by `file:line`, not reproduced).

**Rationale:** Adjudication 3 for the bundling; Adjudication 2 for the number; Adjudication 4 for
what is deliberately *not* in the body.

---

### Op 4 — `merge_existing` (resolve) · `.kb/open-questions/deny-bans-red-on-the-worker-dependency.md`

**Classification:** aligns · **Sources:** `0035-async-trait-through-worker.md` (claim 7)
**Maps:** open-questions index ✓ · domain map ✗ · decision map ✗

The corpus's resolution procedure, executed rather than described:
`.kb/open-questions/README.md`:41-45, on wave 6's cf-40 and WF-11 precedent. **Frontmatter-only
hunks, plus one appended dated section.** The existing body is not reworded — what it describes
was true when it was written, and that is the whole value of the record.

| Field | From | To |
| --- | --- | --- |
| `status` | `accepted` | `superseded` |
| `related` | `[kb-decision-0001, kb-decision-0029, kb-decision-0023]` | `+ kb-decision-0035` |
| `source_paths` | as imported | `+ .kb/_intake/0035-async-trait-through-worker.md`, `+ references/adr/0035-async-trait-through-worker.md` |
| `last_reviewed` | `2026-08-20` | `2026-09-02` |
| `summary` | as imported | `+` a dated resolution paragraph, appended |

**No `superseded_by`.** That key is decision-only, and wave 6 set none on either question it
resolved — `kb-open-question-cf-40-ownership-001` carries the resolving atom in `related` and
nothing else. Mirrored exactly.

**Appended body section** (the cf-40 shape): `## Resolved 2026-09-02`, naming `kb-decision-0035`,
recording that the **ratify** shape was taken and the **refuse** shape declined, and answering
the three sub-questions the atom itself ordered — (1) ratify, and the entry covers `worker` and
`worker-macros` as two names in one `wrappers` list rather than two entries; (2) not reached, and
stops being this atom's, because the ban is no longer red; (3) yes, and the precedent it sets is
the one `kb-decision-0034` would recognise — the exemption was minted by the adapter that first
needed it, with the argument written into `deny.toml` beside the entry, and no umbrella ADR over
dependency exceptions was required.

---

### Op 5 — `create_new` · `.kb/playbooks/making-the-need-a-page-answers-declared-singular-checkable.md`

**Classification:** extends · **Sources:** `lesson-page-need-declaration-discipline.md`
**Maps:** domain map ✓ (**new domain** — documentation & prose standards) · others ✗

```yaml
id: kb-playbook-declared-page-need-001
title: Making the need a page answers a declared, singular, checkable property
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  A prose tree teaches unevenly and nothing sees it: the need each page answers is implicit
  and therefore arguable. The method makes it explicit — one need per page, declared in the
  page's own visible body text, spelled from a closed enumerated set, checked mechanically —
  then the check documents the judgement it cannot make, and a written non-author walk
  supplies it. Front matter, an HTML comment, a filename convention, a sidecar manifest and a
  badge all lost as declaration forms; an open set lost because no check tests membership in
  one; a persona taxonomy lost because a persona is a property of the reader. The
  discriminator is that a check sees a declaration and never an answer, so any form that
  hides it from the reader or makes it unenumerable hands the discipline back to judgement.
depends_on:
  - kb-playbook-verify-referent-report-coverage-001
related:
  - kb-playbook-ratchet-gate-landing-001
  - kb-playbook-anchoring-citations-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/lesson-page-need-declaration-discipline.md
  - standards/pages/README.md
  - standards/pages/00-one-need.md
  - standards/pages/10-the-need-set.md
  - standards/pages/20-the-fold-line.md
  - standards/pages/40-reviewing-a-page.md
  - xtask/src/lint_pages.rs
  - .bklg/docs-that-teach/page-need-discipline/_design.md
  - .bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md
last_reviewed: 2026-09-02
```

**What changes from the staged file:** `status` `proposed` → `accepted`; `last_reviewed`
`2026-08-18` → `2026-09-02`; `source_paths` gains the `.kb/_intake/…` path per standing choice 4.
**Everything else is kept as staged** — the id, the title, the summary, and all four link edges,
each of which resolves to a real atom (verified against `.kb/playbooks/` and `.kb/governance/`).
This is the only file in the wave that arrived with valid `KbFrontmatter` already on it, and the
wave does not rewrite what is already right.

**Body:** as staged. All nine cited paths were verified to exist, including the `NEEDS` /
`MAX_NEEDS` constants and the `const _: () = assert!(…)` in `xtask/src/lint_pages.rs`.

**Rationale:** No modal in its own voice; the `must`s live in `standards/pages/`'s RP- rules,
which this atom cites and does not import — so `playbooks/`, not `decisions/`. Not merged into
`kb-playbook-verify-referent-report-coverage-001` (40; `00`'s refused-dedup table): the staged
file names that atom as *"the earlier instance"* of a shape and carries the `depends_on` edge
itself, and a second instance has never overwritten a first in this corpus.

---

### Op 6 — `create_new` · `.kb/open-questions/trademark-search-gates-the-commercial-layer.md`

**Classification:** extends · **Sources:** `brand-identity-commitments.md` **and**
`brand-where-the-identity-lives.md` — **CL-1, the wave's clearest cross-file collapse**
**Maps:** open-questions index ✓ · domain map ✓ (**new domain** — brand identity) · decision map ✗

```yaml
id: kb-open-question-trademark-search-001
title: The trademark search on "happenstance" has not been run, and it gates the commercial layer
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Two of the four staged brand documents close on the same unresolved gate, in almost the same
  words, and this atom is both of them. What is true today is that no trademark search on
  "happenstance" has been run. What makes it load-bearing rather than administrative is that
  the anti-appropriation lever the commercial seam rests on is trademark and not copyright,
  because Apache-2.0 §6 grants no trademark rights — so a permissive licence protects the code
  and protects the name not at all. What is not decided is whether the name is clear, and
  nothing downstream of that can be: no part of the identity should be filed, registered or
  applied to physical goods until the search returns, and the commercial layer waits on the
  same answer. The identity itself is unaffected and continues to ship — the wordmark, the
  mark and the palette are decided (SD-0001, SD-0002) and are used in the repository today;
  what is gated is registration, filing and physical application, which is a narrower set than
  use. Forced by the first of those three, whichever comes first.
depends_on: []
related: []
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - .kb/_intake/brand-where-the-identity-lives.md
  - references/seeds/licensing-and-the-commercial-seam.md
  - references/brand/SD-0002-the-mark.md
last_reviewed: 2026-09-02
```

**Body shape:** *What is true today* · *What is not decided* · *What forces it* · *Ordered
sub-questions* (1. classes and jurisdictions the search must cover, given a crates.io name and a
GitHub organisation are already public; 2. whether an unregistered mark is enough for the
anti-appropriation lever the seam actually needs; 3. what happens to `SD-0002`'s artwork if the
word is not clear — which is why this atom is `related` **from** the brand decisions rather than
merged into either).

**Rationale:** CL-1 at 95. Not folded into Op 12 (a commitment set) or Op 13 (a pointer), because
an unresolved gating question is neither, and both files stage it as their own closing section
rather than as part of their subject. `related: []` is outbound-only per standing choice 7 — Ops
12 and 13 carry the edges.

---

### Op 7 — `create_new` · `.kb/reference/brand-mark-geometry-and-palette-2026-08.md`

**Classification:** extends · **Sources:** `brand-identity-commitments.md` **and**
`brand-symbol-wordmark-lockup-pattern.md` — **CL-3** · **Maps:** domain map ✓ (brand identity)

```yaml
id: kb-reference-brand-geometry-palette-001
title: The mark's geometry and the palette's measured contrast, 2026-08-18
kind: reference
status: accepted
authority_tier: note
summary: >-
  The construction numbers behind SD-0002 and the contrast ratios that justify the palette,
  measured 2026-08-18 and true of that date. Geometry: a 100-unit square build grid, blocks at
  a pitch of 360°/7 = 51.4286° with the first at 0°; display weight disc r15, block 9 x 13,
  corner radius 3.6, block centre at radius 32; compact weight for 32px and below, disc r22,
  block 15 x 20, corner radius 5.5, block centre at radius 37; hub gap 10.5 units so blocks
  never touch the disc; gap-to-block-width ratio 2.19 : 1. Wordmark: IBM Plex Sans SemiBold
  600 at -2.7% tracking over the font's own kerning, delivered as outlined paths. The mark in
  the lockup: 0.40em diameter (0.57x cap height), top edge 0.06em above the cap line, left
  edge 0.06em inside the word's advance — the constants two staged documents both carried and
  which live here once. Palette, WCAG 2.1 relative luminance computed 2026-08-18: Sun #FFB627
  at 1.71:1 on paper and 10.34:1 on ink; Deep #E08700 at 2.68:1 and 6.60:1; Ember #A85B00 at
  4.92:1 and 3.13:1; Ink #2A211B at 15.38:1 on paper; Paper #FFFCF4 at 15.38:1 on ink. Sun on
  GitHub dark #0D1117 is 10.79:1 and on docs.rs white 1.75:1. Ink was #191512 in the first cut
  and measured warm while reading as plain black. This atom records the numbers and draws no
  conclusion from them; kb-decision-sd-0002 is where the conclusions are.
depends_on: []
related: []
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - references/brand/SD-0002-the-mark.md
  - references/brand/brand-kit.html
  - assets/brand/
last_reviewed: 2026-09-02
```

**Body shape:** a dated header sentence (*"true of 2026-08-18"* — `reference/README.md`'s dating
rule, in the body and not only in `last_reviewed`) · *Geometry* · *Wordmark and lockup* ·
*Palette, with measured contrast* (the five-row table verbatim) · *Two surfaces measured
separately* (GitHub dark, docs.rs white) · *What this atom does not say* (which token may be used
where — that is `kb-decision-sd-0002`'s).

**Rationale:** `.kb/decisions/README.md`: *"What does not belong here: the evidence."* CL-3 for
the deduplicated lockup constants. Refused as an append to any existing reference atom — none
concerns brand, and none shares a measurement date.

---

### Op 8 — `create_new` · `.kb/design/symbol-annotates-the-wordmark.md`

**Classification:** extends · **Sources:** `brand-symbol-wordmark-lockup-pattern.md` (C1)
**Maps:** domain map ✓ (brand identity) · **the `design/` layer's first atom**

```yaml
id: kb-design-symbol-annotates-the-wordmark-001
title: A symbol beside a wordmark is set as an annotation marker, not as a second object
kind: concept
status: accepted
authority_tier: design
summary: >-
  The resolved pattern for any lockup pairing a symbol with a wordmark. Set the symbol small
  and raised, overlapping the word's advance, nested into the open counter-space at the top
  right of the final letter; for happenstance that is 0.40em diameter with the top edge 0.06em
  above the cap line and the left edge 0.06em inside the advance
  (kb-reference-brand-geometry-palette-001 holds the constants). Two arrangements were
  rejected by rendering rather than by argument: the symbol to the left with a decorative
  element closing the right, which read as two objects bracketing a word with neither clearly
  the subject; and the symbol to the right at lockup scale with a normal gap, which read as a
  logo tacked onto a word — the size was not the problem, the gap was. The mechanism is what
  makes it portable: a glyph sitting clear of a word after a gap is read as a second object
  however well proportioned it is, while a small raised glyph overlapping the advance is
  parsed as a footnote marker, which is a typographic relationship rather than a
  compositional one. Holds when the wordmark's final letter is round or open — e, o, c, a —
  leaving a pocket at its top right. Stops holding when the final letter is a full-height
  vertical — l, k, t, d — where no pocket exists, the symbol collides or floats, and a
  conventional lockup gap is the better answer.
depends_on: []
related:
  - kb-reference-brand-geometry-palette-001
source_paths:
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - references/brand/SD-0002-the-mark.md
  - references/brand/logo-explorations-v3-collision.html
  - assets/brand/
last_reviewed: 2026-09-02
```

**Body shape:** `design/README.md`'s template literally — **Pattern** (and the class of surface:
any symbol-plus-wordmark lockup) · **Rejected** ×2, each with the property that ruled it out ·
**Mitigates** (the *logo tacked onto a word* failure, found by rasterising) · **Holds when** /
**Stops holding when**.

**Rationale:** No modal anywhere in the source; a resolved pattern for a class of surface, with
its rejected alternatives and its boundary. That is `design/`'s contract and not `decisions/`'s.
Split from Op 9 on the one-decision-per-title deletion test — `00`'s refused-dedup table.

---

### Op 9 — `create_new` · `.kb/design/a-radial-mark-between-two-glyph-collisions.md`

**Classification:** extends · **Sources:** `brand-symbol-wordmark-lockup-pattern.md` (C2–C4, C6)
**and** `brand-identity-commitments.md` (**CL-2**) · **Maps:** domain map ✓ (brand identity)

```yaml
id: kb-design-radial-mark-collisions-001
title: A radial mark sits between the gear and the brightness glyph, and an odd count is what separates it
kind: concept
status: accepted
authority_tier: design
summary: >-
  The resolved pattern for any radial symbol intended to survive a favicon. A ring of elements
  around a filled centre sits between two of the most crowded shapes in software, and both
  were found by rasterising at 16 and 32px rather than by reasoning about the vector. The gear
  was anticipated and is mitigated by five measurable constraints: seven elements rather than
  twelve or more, gaps 2.19x the element width, a 10.5-unit hub gap so elements never touch
  the centre, corner radius 0.4 of element width, and a solid centre with no bore or aperture;
  squaring the corners was tested and rejected because it buys nothing against the second
  collision and walks back into this one. The display-brightness glyph was not anticipated and
  is the more dangerous, because near-universal eight-fold radial symmetry is that glyph and
  the symbol stands alone exactly where it is weakest — favicon, registry avatar,
  organisation icon. Resolved by an odd count: seven defeats the eight-fold reading, no icon
  set uses seven, and the sun-like character survives; one element at twelve o'clock places
  the gap at six and keeps the mark symmetric about the vertical axis, because an odd count
  arranged that way reads as chosen while the same count rotated so the gap falls at the top
  reads as a piece that fell off. Rejected on cost: turning each element 90 degrees to lie
  tangentially removes the brightness reading completely and holds better at 16px, but stops
  being a sun, which was the brief — it remains the documented fallback, with its own ceiling
  at ten or more elements where the dashes close into a milled edge and read as a loading
  spinner. Rejected: seven and tangential together, because tangential needs even spacing to
  read as a sequence and at seven it tiles badly. Two anti-patterns proved real: irregularity
  to signal discovery reads as a mistake rather than as intent, and judging a symbol at
  display size only hides both collisions. The defence is reasoned and rendered but not tested
  on a stranger, and until it is the collision is mitigated and unfalsified rather than
  closed.
depends_on: []
related:
  - kb-design-symbol-annotates-the-wordmark-001
  - kb-reference-brand-geometry-palette-001
source_paths:
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - .kb/_intake/brand-identity-commitments.md
  - references/brand/logo-explorations-v3-collision.html
  - references/brand/SD-0002-the-mark.md
last_reviewed: 2026-09-02
```

**Body shape:** **Pattern** (and the class: any radial symbol that must survive a favicon) ·
**The two collisions** (the gear, with its five constraints; the brightness glyph, and why it is
worse) · **Rejected** ×3 (squared corners; tangential elements, on the brief, kept as the named
fallback with its ten-element ceiling; seven *and* tangential, which interfere) ·
**Anti-patterns that proved real** ×2 (irregularity; display-size-only judgement) ·
**Holds when / Stops holding when** · **The boundary: unfalsified** (the stranger test, and what
its failure would change).

**Rationale:** CL-2 — the irregularity finding lands here once and Op 12 cites it. Adjudication 5
sends the third anti-pattern (hex-value colour) to Ops 12 and 7 instead. Adjudication 8 keeps
the unfalsified defence here as a boundary rather than minting a companion open question.

---

### Op 10 — `create_new` · `.kb/decisions/sd-0001-the-name-names-the-boundary-of-what-occurred.md`

**Classification:** requires-new-decision (arriving taken) · **Sources:** `brand-the-name-and-what-it-means.md`
**Maps:** decision map ✓ (**new `SD-` section**) · domain map ✓ (brand identity)

```yaml
id: kb-decision-sd-0001
title: "Happenstance" names the boundary drawn by what occurred, and what that places on copy
kind: decision
status: accepted
authority_tier: decision
adr_id: SD-0001
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  The name means the thing DCB does: it draws the consistency boundary around what actually
  occurred, rather than around a structure chosen before anyone knew what would occur. The
  word is happen plus circumstance, and the sense claimed is not the dominant dictionary one —
  chance — but the narrower one carried by the phrase as it happened: what turned out to be
  the case, as against what was arranged in advance. That is the axis DCB moves along, because
  a classical aggregate fixes its boundary when the schema is written while a DCB handler
  reads exactly the events its decision depends on and appends conditioned on nothing matching
  that query having appeared since. The boundary is deliberate but not pre-declared —
  designed on purpose, not inherited. Five rules follow and they bind copy rather than code:
  name the colloquial reading and displace it rather than talking past it; never assert that
  the name is clever; do not extend the metaphor anywhere, in copy, examples, error messages
  or artwork, with no chance, luck, dice, serendipity, fortune or coincidence imagery; keep
  the name lower-case in prose, recasting a sentence rather than capitalising it; and do not
  make the name the lead claim. What this does not claim is why the name was chosen: ADR-0005
  renamed the project from eventum on 2026-08-05 because the bare eventum name was
  unavailable, and availability is the entire recorded justification. This assigns a meaning
  after the fact, states that it is doing so, and does not supersede ADR-0005, which stays
  correct about why the rename happened. The costs are permanent and accepted: the colloquial
  sense is wrong and is the default reading, the meaning is retroactive against a public
  record, and the word is twelve letters and three syllables with no natural short form — none
  of which is a reason to propose a rename of four crate names already shipped.
depends_on: []
related:
  - kb-decision-0005
  - kb-decision-0006
  - kb-concept-torn-read-append-boundary-001
source_paths:
  - .kb/_intake/brand-the-name-and-what-it-means.md
  - references/brand/SD-0001-what-happenstance-means.md
  - references/adr/0005-rename-to-happenstance.md
last_reviewed: 2026-09-02
```

**Body shape:** *The claim* (the sense, and the DCB axis it names — with *"the boundary is
deliberate but not pre-declared"* held precisely, and *unarranged in advance* preferred over
*unplanned*) · *Rules this places on copy* (the five, verbatim) · *What this does not claim*
(ADR-0005, and the explicit non-supersession) · *Costs, acknowledged* · *Usable forms* (the three
lengths, as a trailing section).

**Rationale:** Rule 2 — five prescriptions, four of them prohibitions, is a commitment set.
Kept as one atom because a rule forbidding an imagery family is unintelligible without the sense
it protects. **`kb-decision-0005` receives no operation**, at 65: the whole of claim 3 is a
statement that it stays as written, and honouring that means an outbound `related` edge and
nothing more.

---

### Op 11 — `create_new` · `.kb/decisions/sd-0002-standalone-svg-carries-one-colourway.md`

**Classification:** requires-new-decision · **Sources:** `brand-identity-commitments.md`
**Maps:** decision map ✓ (`SD-` section) · domain map ✓ (brand identity)

```yaml
id: kb-decision-standalone-svg-one-colourway-001
title: A standalone SVG carries one fixed colour, and the surface is selected at the point of use
kind: decision
status: accepted
authority_tier: decision
adr_id: SD-0002
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  A standalone SVG must carry one fixed colour, and a prefers-color-scheme media query must
  not be placed inside one. An earlier cut of the lockups embedded @media
  (prefers-color-scheme: dark) so a single file could serve both surfaces, and it failed
  silently: a standalone SVG knows the operating system's preference but not the colour of
  the surface it was placed on, so a light page opened on a machine set to dark mode rendered
  the wordmark cream on cream, invisible, with no error anywhere. The rule that replaces it is
  ship one file per colourway and select at the point of use, where the background is known —
  a picture element with a source carrying the media query and an img carrying the light
  default. The page knows its own background; the file does not. This is recorded as its own
  atom rather than as a clause of the mark decision because its scope is wider than the mark:
  it generalises past logos to any SVG asset shipped for use on a surface it does not
  control, which in this repository includes README artwork, a docs.rs header and a crates.io
  card. It shares SD-0002's record with kb-decision-sd-0002, which carries the mark itself and
  cites this atom for the reasoning behind its eighth binding rule.
depends_on: []
related: []
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - references/brand/SD-0002-the-mark.md
  - assets/brand/
last_reviewed: 2026-09-02
```

**Body shape:** *One record, two atoms* (opening sentence naming SD-0002 and which half this is) ·
*The rule* · *The failure it was found by* (cream on cream, no error — a silent failure, which is
why a convention would not have held) · *What replaces it* (the `<picture>`/`<source media>`
snippet, inline) · *Why this is not a brand rule* (the generalisation sentence) · *Alternative
rejected* (the embedded media query, and the property that ruled it out — a file cannot know its
surface).

**Rationale:** Adjudication 7. `related: []` outbound-only; Op 12 carries the edge.

---

### Op 12 — `create_new` · `.kb/decisions/sd-0002-the-mark-and-the-rules-that-bind-it.md`

**Classification:** requires-new-decision (arriving taken) · **Sources:**
`brand-identity-commitments.md` **and** `brand-symbol-wordmark-lockup-pattern.md` (**CL-2, CL-4**)
**Maps:** decision map ✓ (`SD-` section) · domain map ✓ (brand identity)

```yaml
id: kb-decision-sd-0002
title: Seven equal blocks and a lowercase wordmark, and the rules that bind anything carrying the name
kind: decision
status: accepted
authority_tier: decision
adr_id: SD-0002
reversibility: low
phase: null
supersedes: null
superseded_by: null
summary: >-
  Seven equal blocks radiate from a solid disc: the blocks are records in an append-only log —
  same size, same pitch, no privileged first one — and the disc is the boundary a query drew
  across them. The mark states in the only grammar a logo has what kb-decision-sd-0001 says
  the name means. Equality of the blocks is load-bearing and was established by rendering
  rather than by argument, kb-design-radial-mark-collisions-001 holding the evidence:
  irregularity does not read as discovery, it reads as an error, and order is the one property
  an event store may never look casual about. The wordmark is set lowercase always, with the
  mark placed as a small annotation marker at the top right of the final e —
  kb-design-symbol-annotates-the-wordmark-001 for why that composition and not a lockup gap.
  Eight rules bind anything carrying the name, and they are commitments rather than
  preferences: seven blocks always; one block at twelve o'clock and the mark never rotates;
  scale uniformly, with no stretching, condensing, arching or perspective; one flat fill from
  the palette, with no gradients, shadows, glows, outlines or strokes; clear space of one
  block length on every side at every size; the wordmark stays lowercase, so recast a sentence
  rather than capitalise the name; the mark follows the word and is never placed before it;
  and assets ship one fixed colour per file, for the reason
  kb-decision-standalone-svg-one-colourway-001 carries. One palette commitment sits with them:
  Sun is a graphic colour and is never used for text on a light surface, which is why Ember
  exists — it is the amber value that meets AA. The geometry and the measured contrast are
  kb-reference-brand-geometry-palette-001's and are cited rather than restated. One
  calibration finding is recorded with the palette because it is the reason a token has the
  value it has: Ink was #191512 in the first cut and read as plain black, and a colour
  justified by its hex value rather than by being looked at is not yet a decision. Registration
  and physical application are gated by kb-open-question-trademark-search-001.
depends_on:
  - kb-decision-sd-0001
related:
  - kb-decision-standalone-svg-one-colourway-001
  - kb-design-radial-mark-collisions-001
  - kb-design-symbol-annotates-the-wordmark-001
  - kb-reference-brand-geometry-palette-001
  - kb-open-question-trademark-search-001
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - references/brand/SD-0002-the-mark.md
  - references/brand/brand-kit.html
  - assets/brand/
last_reviewed: 2026-09-02
```

**Body shape:** *The decision* (the mark, what its parts mean, and the premise it rests on) ·
*Rules that bind anything carrying the name* (the eight, verbatim, with the eighth citing Op 11) ·
*The palette commitment* (Sun is never text on a light surface; Ember exists for that reason;
the Ink calibration finding, one sentence) · *Alternatives rejected* (varied block lengths, on
the rendering; the numbers and the collision reasoning cited to Ops 7 and 9 rather than
reproduced) · *What is gated* (Op 6).

**Rationale:** Rule 2 — eight always/never rules, self-labelled *"commitments, not
preferences"*. Adjudication 6 for the layer and the `adr_id`; Adjudication 7 for the split from
Op 11; Adjudication 5 for the Ink sentence being here rather than in a design atom. **The
irregularity paragraph is cited, not copied** — CL-2, and `.kb/README.md`'s *"prefer merge-and-link
over new files"* applied to a paragraph rather than to a file.

---

### Op 13 — `create_new` · `.kb/reference/brand-identity-source-locations-2026-08.md`

**Classification:** extends · **Sources:** `brand-where-the-identity-lives.md`
**Maps:** domain map ✓ (brand identity)

```yaml
id: kb-reference-brand-source-locations-001
title: Where the brand identity lives, as of 2026-08-18
kind: reference
status: accepted
authority_tier: note
summary: >-
  A pointer, not a copy: the brand material is large, already written down, and should be
  cited by path. True of the working tree on 2026-08-18. assets/brand/ holds eight SVG files
  with a README stating their use, the light/dark selection pattern, the palette tokens and
  the binding rules — two horizontal lockups (ink and reversed), a stacked lockup, a
  wordmark, a mark for 32px and above, a compact mark for 32px and below, a currentColor
  monochrome mark and a favicon; the wordmarks are outlined paths, so no font is required at
  the point of use. references/brand/ holds the manual and the records: brand-kit.html, the
  applied specification in ten sections and the thing to hand to anyone applying the identity;
  showcase.html, the identity across the surfaces it will actually meet; SD-0001 and SD-0002,
  the records; and logo-explorations-v3-collision.html, the rendered comparison behind the
  element count. Which source answers which question is the half a directory listing cannot
  supply: the brand kit is the authority for how do I apply this, the SD- records for why is
  it like that, and the showcase is evidence rather than instruction. Brand decisions are
  numbered SD- and sit deliberately outside the ADR- sequence, because that corpus records
  contract, port and wire-format decisions and interleaving artwork into it makes the
  architecture sequence harder to read for no gain; SD- records carry the same structure as
  ADRs and are cited from .kb/ atoms exactly as ADRs are, which is what
  kb-decision-sd-0001 and kb-decision-sd-0002 do. The identity was developed in a separate
  strategy workspace holding earlier exploration rounds not copied here, and where the two
  disagree the repository copies are the authority: the workspace holds process, the
  repository holds the settled result. Not yet true as of that date: no PNG raster fallbacks
  have been generated, and the trademark search has not been run
  (kb-open-question-trademark-search-001).
depends_on: []
related:
  - kb-decision-sd-0001
  - kb-decision-sd-0002
  - kb-decision-standalone-svg-one-colourway-001
  - kb-reference-brand-geometry-palette-001
  - kb-open-question-trademark-search-001
source_paths:
  - .kb/_intake/brand-where-the-identity-lives.md
  - references/brand/
  - assets/brand/
last_reviewed: 2026-09-02
```

**Body shape:** a dated header (*"true of the working tree on 2026-08-18"*, the staged file's own
first sentence) · *The artwork* · *The documents* · *What each source establishes* · *Numbering
convention* · *Provenance and drift* (the repository-wins precedence rule, as a subsection —
detached from the sources it ranks it means nothing) · *Not yet true* (PNG fallbacks; the
trademark search, pointing at Op 6 rather than restating it).

**Rationale:** `reference/README.md`'s pointer case in terms — *"the atom names it and says what
it establishes; it does not reproduce it"* — with the dating rule supplying the honesty the
absent tree requires (`00`, provenance). Claim 7 is **not** here: it is CL-1's, and it is Op 6.

---

## Roll-up

| Op | Disposition | Layer | Kind | Decision map | Domain map | OQ index |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | create_new | `open-questions/` | open_question | — | ✓ | ✓ |
| 2 | create_new | `decisions/` | decision | ✓ | ✓ | ✓ |
| 3 | create_new | `decisions/` | decision | ✓ | ✓ | — |
| 4 | merge_existing | `open-questions/` | open_question | — | — | ✓ |
| 5 | create_new | `playbooks/` | playbook | — | ✓ | — |
| 6 | create_new | `open-questions/` | open_question | — | ✓ | ✓ |
| 7 | create_new | `reference/` | reference | — | ✓ | — |
| 8 | create_new | `design/` | concept | — | ✓ | — |
| 9 | create_new | `design/` | concept | — | ✓ | — |
| 10 | create_new | `decisions/` | decision | ✓ | ✓ | — |
| 11 | create_new | `decisions/` | decision | ✓ | ✓ | — |
| 12 | create_new | `decisions/` | decision | ✓ | ✓ | — |
| 13 | create_new | `reference/` | reference | — | ✓ | — |

**What the Maps phase inherits.** `decision-map`: two sections — a `2026-09-02 wave (ADR-0035,
ADR-0036)` section, and a **new `SD-` series section** for three rows carrying a non-`ADR`
`adr_id` and `phase: null`, with a line explaining the parallel numbering. No existing row's
status or supersession cell changes anywhere. `domain-map`: **two new domains** — brand identity
(seven atoms across four layers, the corpus's first cross-layer domain that touches no port) and
documentation & prose standards (one atom) — plus additions to the existing contract-ports domain
for ADR-0035, ADR-0036 and Op 1. `open-questions-index`: two new bullets (Ops 1 and 6) and one
flip to **Superseded** (Op 4), which stays listed and annotated rather than removed.

**After this wave:** 89 atoms. `decisions/` 34 (32 accepted, 2 superseded), `open-questions/` 28
(21 accepted, 7 superseded), `reference/` 12, `playbooks/` 7, `design/` 2, `maps/` 3,
`concepts/` 1, `governance/` 1.
