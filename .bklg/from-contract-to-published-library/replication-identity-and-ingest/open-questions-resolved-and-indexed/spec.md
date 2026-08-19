---
item: HS-S0100
stage: spec
created: 2026-08-12T13:47:39.147Z
updated: 2026-08-12T13:47:39.147Z
template_sig: 87bbf1d0
rendered_sig: 9a42d580
---

# Spec — Both phase-13 open questions resolved in place, not deleted

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — AC-13 (`:344-346`), DoD 14 (`:398-401`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-013 (`:243-246`), DR-10 (`:178-180`) |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` — the intake row (`:46`), AC-A09 (`:563-567`), the AC-013 tier (`:842`) |
| Story map row | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md:57` |
| Design | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` — signed off 2026-08-12 as **no user-facing surface**; this story renders none |
| Roadmap pointer | `RUNBOOK.md:4593-4595` (WF-1's disposition belongs in ADR-0026's envelope section), `RUNBOOK.md:4516-4620` (phase 13 in full) |

## One-line PR slice

Resolve both phase-13-owned open-question atoms in place —
`sync-message-set-and-format-version` and `dcb-reference-publishes-no-wire-format` —
annotate rather than delete, update `.kb/maps/open-questions-index.md`, and leave
`redkiln validate --kb && redkiln doctor` green.

## Executive summary

Two `open_question` atoms were filed by the 2026-08-10 ADR-import wave and both name
**phase 13** as their owner. This PR is the moment phase 13 answers them — and the
whole of the answer already exists by the time this story starts, because ADR-0026 and
ADR-0027 landed first as its slice-mates. So this PR writes **no new reasoning**. It
performs a disposition: for each atom, the frontmatter status and outbound links move
to reflect what the merged ADR actually did to the question, the atom's prose body is
left byte-identical, the two bullets in `.kb/maps/open-questions-index.md` are rewritten
in place so a reader entering from the map is not told a settled question is still open,
and `redkiln validate --kb && redkiln doctor` stays green over the result.

The delta against the project charter is one word, and it is the word that makes this a
story rather than a chore: **dispositioned**, not **answered**. The two atoms will very
probably not receive the same treatment. The message-set question is expected to be
*settled* by ADR-0027 and to become `superseded`. The DCB question is expected to be
*renewed* — `RUNBOOK.md:4593-4595` asks ADR-0026 for "named, deferred, with the
experiment being a specific external implementation to interoperate with. Not silence"
— and a renewed question is still open, so flipping its status would be a lie told to
make the diff symmetrical. This PR is what makes both outcomes legible from the index,
and it is the first time this repository has closed an open question at all: no atom in
`.kb/open-questions/` carries `withdrawn` or `superseded` today (verified by grep over
the directory), so the shape this story lands is the precedent every later resolution
copies.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted
anchor; nothing below needs an anchor opened to be actionable.

**1. The answer is not this story's to write.** ADR-0026 and ADR-0027 are the answers,
and both merge before this story does — `_storymap.md:57` makes them this story's
`depends_on`, and the map's shaping fact 1 states the ordering: the decisions gate the
code, and every later slice depends on slice 1. If a fact you need to disposition an
atom is *absent* from the merged ADR, that is a defect in the ADR, not a gap you close
by reasoning here. Raise it; do not fill it. An open question closed by an argument
that lives only in a backlog item is a decision with no authority behind it
(`.kb/decisions/README.md:41-43`).

**2. A resolution is a status disposition plus an index annotation. It is never a
deletion and never a rewrite of the question's prose.** `.kb/open-questions/README.md:40-45`
is explicit: "the answer is a **new atom** — and the record of the question stays. Link
the two (`related`), move this atom's `status` to `withdrawn` or `superseded`, and leave
the body describing what was not known at the time. Do not rewrite a question into its
own answer: the value of the record is that it shows the state of knowledge on the day
the choice was made." The index atom says the same from the other side: "A withdrawn or
superseded question stays listed, annotated, rather than removed — the record that it
was once open is itself worth keeping" (`.kb/maps/open-questions-index.md:7-13`, which
is also project DR-10 at `project.md:178-180`).

**3. The corpus already has the test for whether an edit is allowed, and it is not
"did the file change".** `.kb/governance/rewrite-the-referent-never-the-reasoning.md:52-56`:
"Ask whether the edit changes what the document **asserts**, not whether it changes the
document." Its third worked instance is exactly this story's shape — ADR-0007 corrected
ADR-0006, and "ADR-0006's frontmatter gained a 'partly superseded by' note while its
body … stayed verbatim … it changed as metadata, not as prose"
(`:80-85`). The `decision` layer states the same rule mechanically: "That metadata flip
is the only edit an accepted decision ever receives; its body is never reworded"
(`.kb/decisions/README.md:9-13`). Open questions carry `authority_tier: note` and are
not under the decision-immutability check, which is precisely why the discipline has to
be applied on purpose here rather than enforced for you.

**4. The two atoms get different dispositions, and the branch is chosen from evidence
in the merged ADR — not from a preference for a tidy diff.**

- `kb-open-question-sync-message-set-undesigned-001` ("`FORMAT_VERSION = 1` names a
  message set that does not exist yet") asks three *ordered* sub-questions
  (`.kb/open-questions/sync-message-set-and-format-version.md:83-91`): does phase 13
  draft a message set at all; if so is the version negotiated per connection or carried
  per message; and if negotiation wins, is removing the per-message field a breaking
  wire-format change. ADR-0027 owns the message set and the derives on
  `PushBatch`/`EventGroup`/`ReplicatedEvent` (`_storymap.md:56`), and the slice-mate
  story `message-set-on-the-envelope` settles `FORMAT_VERSION`'s disposition
  (`_storymap.md:63`). If all three sub-questions are answered, the atom becomes
  `superseded` with an outbound link to the answering decision atom. If ADR-0027 named
  the message set but left sub-question 3 standing, the atom stays open and is
  **amended** — the residue named, so the next reader inherits a narrowed question
  rather than a stale one.
- `kb-open-question-dcb-no-published-format-001` ("There is no DCB wire format to
  interoperate with") is the one expected to stay open. `RUNBOOK.md:4593-4595` and the
  architecture brief (`_decomposition.md:336-343`) both put its disposition inside
  ADR-0026's envelope section as a **renewal against a named experiment**, and the atom
  itself says the deferral is "unfalsifiable by construction" until some DCB
  implementation publishes an encoding
  (`.kb/open-questions/dcb-reference-publishes-no-wire-format.md:78-85`). A renewal
  leaves `status` alone and lands as an annotation naming ADR-0026 and the experiment.
  Its sub-question 3 — whether re-checking the 2026-08-05 W7 finding is phase 13's
  standing task (`:97-100`) — is the one part of it phase 13 can settle outright, and
  whichever way ADR-0026 settled it must be visible in the annotation.

**5. `resolved` in project AC-013 means *dispositioned*, not *answered*.** The bar is
that neither atom is left claiming a question phase 13 has in fact acted on, and that
the action is legible from the index. The precedent for the open-but-amended shape is
already in the file: `.kb/maps/open-questions-index.md:56-61` carries "**Open** — …
Amended 2026-08-10: … the question is still open because neither assigns an owning
phase." Copy that shape; do not invent a new one.

**6. Nothing under `.kb/` is hand-authored.** `CLAUDE.md` (*Where the work lives*) and
the project's architecture brief both bind this: the intake row at
`_decomposition.md:46` routes "the two resolutions" through `.kb/_intake/` →
`/redkiln:kb-ingest`, and AC-A09 (`_decomposition.md:563-567`) states it as a criterion.
Hand-writing atoms produces "the directory layout of the process without the process",
and the first attempt at it was reverted (`0269720`). So this story authors an intake
document per atom, runs the ingest wave, and *verifies* what the wave wrote — the wave
performs the mutating ops, syncs the map atom, and clears `.kb/_intake/`.

**7. Links are outbound-only, from the question to the answer. No accepted decision
atom is touched.** `redkiln validate --kb` enforces that every `depends_on` / `related`
/ `supersedes` / `superseded_by` id resolves to a real atom (`.kb/README.md:45-46`) —
it does **not** require reciprocity. Adding a back-link to ADR-0026 or ADR-0027 would be
an edit to an accepted decision that `.kb/decisions/README.md:9-13` does not sanction, so
`.kb/decisions/**` is deliberately outside this story's PR boundary and the boundary
check is what makes the rule mechanical rather than remembered.

**8. Do not invent tracking keys.** `KbFrontmatter` is a `.passthrough()` object, so an
imported corpus's own `tracks` / `resolution_ref` keys survive validation — and
`.kb/open-questions/README.md:46-50` says in the same breath: "do not strip them to make
an atom 'conform', and do not invent them on an atom you are authoring." Neither of
these two atoms carries either key today (verified by reading both files). The pointer
to the answer travels on `related`, and on `superseded_by` only where `status` actually
became `superseded` — because a `superseded_by` whose referent does not resolve is a
dangling link and fails the gate.

**9. This story changes no specification clause and no maturity marker.** WF-1's
`[DEFERRED]` marker (`spec/SPECIFICATION.md:1892-1900`) is a referent here, not a
target. If the DCB atom's disposition implies WF-1's falsifier text should move, that
belongs to `clause-arithmetic-and-deferral-renewals` under project AC-010
(`_storymap.md:70`), which is a different slice with a different gate. `spec/SPECIFICATION.md`
is therefore outside this story's PR boundary on purpose.

**10. The journey this serves has no persona atom yet, and that is stated rather than
faked.** `.kb/product/` holds only its README (verified), because the personas are
initiative AC-15's and arrive at closeout. The reader served here is the one the KB's
own contract names: a maintainer who arrives at `.kb/maps/open-questions-index.md` to
find out what is still open. "A question nobody can find from the map is a question that
gets asked again from scratch" (`.kb/open-questions/README.md:28-29`). That sentence is
this story's acceptance bar in one line — and its inverse, a *settled* question still
listed as `Open`, costs the next reader the same way.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (the KB's own resolution record),
  consumed by every later reader of the index and by this project's own exit criteria.
  No double, no flag, no `todo!()`.
- **Slice / milestone**: `decisions-of-record`. Slice-mates, implemented and mounted in
  one context: `adr-0026-peer-ingest-and-transport`, `adr-0027-merge-compensation-and-message-set`,
  and this story. Both mates are hard predecessors — this story's inputs are their outputs
  (`_storymap.md:55-57`, merge order at `_storymap.md:119-121`).
- **Mount point**: **`.kb/maps/open-questions-index.md`** — the real composition root of
  the open-questions layer and the only render path a reader enters from. An atom whose
  status changed and whose index bullet did not is exactly the "constructed but not
  mounted" failure: the resolution exists on disk and nobody meets it. The index's own
  *Adding an entry* section (`:136-143`) is the composition contract — status word
  first, then the id, then one sentence; the atom carries the structure, the index does
  not repeat it.
- **Wires into**:
  - `.kb/open-questions/sync-message-set-and-format-version.md` and
    `.kb/open-questions/dcb-reference-publishes-no-wire-format.md` — the two atoms whose
    frontmatter this disposition writes.
  - The two answering decision atoms landed by the slice-mates under `.kb/decisions/`
    (ids in the corpus's existing `kb-decision-00NN` shape, cf. `kb-decision-0016` in
    both atoms' `related` lists) — referenced outbound only, never edited.
  - `.kb/_intake/` + `/redkiln:kb-ingest` — the only writer of `.kb/` atoms
    (`_decomposition.md:46`, `CLAUDE.md` *Where the work lives*). The wave also writes
    its own record under `.kb/_governance/integration-waves/`, which is why that path is
    inside the boundary.
  - `redkiln validate --kb` (`KbFrontmatter` conformance, accepted-decision immutability,
    no dangling links) and `redkiln doctor` — the merge gate this project's testing brief
    assigns to AC-013 (`_decomposition.md:842`, `:651-653`).
- **Renders surfaces**: **none.** `_design.md` records **N/A — no user-facing surface**
  for the whole project, signed off 2026-08-12, and its `## Items` block is `N/A`. This
  story implements no `path` id because the design declares none, and it does not
  re-open that determination.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** No port, type or
  trait changes here; no peer can behave differently because of this diff, so a rule
  would be decorative under `CLAUDE.md`'s "a rule that no adapter can fail" corollary.
  The check that *can* fail is `redkiln validate --kb && redkiln doctor`, and it is a
  real one: a dangling `superseded_by`, an edited accepted decision, or an invented
  status value all fail it.
- **Clause(s)**: **none discharged, none amended.** WF-1 is cited as a referent only
  (`spec/SPECIFICATION.md:1892-1900`); its marker is `clause-arithmetic-and-deferral-renewals`'
  under project AC-010. No `[FROZEN]` clause is edited, which is project DoD 7 and the
  initiative's standing exclusion.
- **Advances DoD scenario**: initiative **DoD 14** (`initiative.md:398-401`) — "an
  accepted decision atom answers whether ingest re-checks a writer's asserted
  conditions … the corresponding open-question atom reflects that resolution, and
  `redkiln validate --kb` passes." The ADRs land the first clause; **this story is the
  only thing in the initiative that lands the second and third**. It also closes project
  DoD 5 (`project.md:271-274`) and is the sole claimant of project AC-013
  (`_storymap.md:91`).

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced
block below and fails on any file changed outside it.

Three exclusions are load-bearing rather than incidental, and each is a rule made
mechanical by leaving a path out: `.kb/decisions/**` is out because links are
outbound-only and an accepted decision's body is immutable (context pack 7);
`spec/SPECIFICATION.md` is out because clause and marker work is a different slice's
(context pack 9); `crates/**` is out because this story compiles nothing.

```
.kb/_intake/**
.kb/open-questions/**
.kb/maps/**
.kb/_governance/integration-waves/**
.bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/**
```

**In this PR**

- One `.kb/_intake/` document per atom, stating the disposition and the evidence in the
  merged ADR that forces it, then `/redkiln:kb-ingest` consuming and clearing them.
- The frontmatter disposition the wave writes on both open-question atoms: `status`
  where it moved, the outbound `related` / `superseded_by` link to the answering
  decision atom, and `last_reviewed` bumped to the wave's date.
- Both bullets in `.kb/maps/open-questions-index.md:122-134` rewritten in place, in the
  index's own *Adding an entry* form, with a dated amendment annotation where the
  question stayed open.
- This story's own `_ledger.md` and its evidence citations (`require_ledger: true`,
  `.redkiln/config.yaml:67`).

**Explicitly not in this PR**

- Any new reasoning about the message set, `FORMAT_VERSION`'s per-message versus
  per-connection fork, or DCB interoperability. Those are ADR-0027's and ADR-0026's, and
  a sentence of it appearing here is scope drift with the authority stripped off.
- Any edit to `.kb/decisions/**`, including a reciprocal `related` back-link.
- Any edit to `spec/SPECIFICATION.md`, including WF-1's `[DEFERRED]` marker.
- Deleting, renaming or moving either open-question atom, or removing either index
  bullet.
- Any `.rs` file, any `xtask` gate wiring, any `CHANGELOG.md` entry — this story adds no
  conformance rule, so CF-29 owes nothing.

**Merge DoD, one line**: both atoms carry a disposition that matches what the merged
ADRs actually decided, both index bullets say so, both bodies are byte-identical below
the frontmatter, and `redkiln validate --kb && redkiln doctor` is green with exactly the
six expected `template-drift` advisories and no new finding.

The implementer may also touch the composition-root and wiring files named in the
Integration contract — `.kb/maps/open-questions-index.md` and the ingest wave's own
record — to mount this slice; that is not scope drift.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The disposition is read off the merged ADR, never derived here** | Before writing anything, read what ADR-0027 landed against the message-set atom's three ordered sub-questions and what ADR-0026's envelope section landed against WF-1's interoperability half. The disposition is a function of those two readings. A sub-question the ADR did not reach stays open and is named as residue. | `.kb/open-questions/sync-message-set-and-format-version.md:83-91`; `.kb/open-questions/dcb-reference-publishes-no-wire-format.md:87-100`; `RUNBOOK.md:4593-4595`; `_decomposition.md:336-343` |
| **Settled → `superseded`, with a resolving link** | Where the answering ADR closes the question, `status` becomes `superseded` and the atom carries an outbound `superseded_by` (plus `related`) naming the answering decision atom's id. Both must resolve — `validate --kb` fails a dangling link — so the ADR atom must already be merged when the wave runs. | `.kb/open-questions/README.md:40-45`; `.kb/README.md:45-46` |
| **Renewed or narrowed → stays open, amended** | Where the ADR renewed a deferral against a named experiment, or answered some sub-questions and not others, `status` stays `accepted`, a `related` link to the ADR is added, and the index bullet gains a dated `Amended <date>:` clause naming the ADR, the experiment, and what is still open. The expected case is the DCB atom: `RUNBOOK.md` asks for a renewal, not a settlement. | `.kb/maps/open-questions-index.md:56-61` (the ES-6 precedent); `.kb/open-questions/dcb-reference-publishes-no-wire-format.md:78-85`; `RUNBOOK.md:4593-4595` |
| **Both bodies stay byte-identical below the frontmatter** | The question's prose is the record of what was not known on the day the choice was made. `git diff -- .kb/open-questions/` must show hunks inside the YAML block only. No "How it was resolved" section is appended to either atom: the answer lives in the decision atom, and the pointer travels as metadata. | `.kb/open-questions/README.md:40-45`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md:52-56`, `:80-85` |
| **No new frontmatter vocabulary** | `status` takes a value from the documented enum (`draft` · `proposed` · `accepted` · `superseded` · `withdrawn`) and nothing else; `tracks` and `resolution_ref` are not invented on atoms that do not carry them; `authority_tier: note` is unchanged on both. | `.kb/README.md` *Atoms* table; `.kb/open-questions/README.md:46-50` |
| **Links are outbound only** | The question points at the decision. The decision is not edited to point back. `validate --kb` requires resolvable ids, not reciprocity — and `.kb/decisions/**` being outside the PR boundary is what makes this fail loudly rather than quietly. | `.kb/README.md:45-46`; `.kb/decisions/README.md:9-13` |
| **The index is updated in place, in its own form** | Each of the two bullets at `.kb/maps/open-questions-index.md:122-134` keeps its position under *Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)*, keeps its file link and its `kb-open-question-…` id, and changes only its leading status word and its sentence. No bullet is removed; no new `##` section is started, because both questions' domain already has one. | `.kb/maps/open-questions-index.md:77-83`, `:122-134`, `:136-143` |
| **The write path is the ingest path** | Both dispositions are staged as `.kb/_intake/` documents and applied by `/redkiln:kb-ingest`, which performs the mutating ops serially, syncs the map atom, records the wave under `.kb/_governance/integration-waves/`, and clears `.kb/_intake/`. Hand-editing the atoms is the failure mode that was reverted once already. | `_decomposition.md:46`, `:563-567`; `CLAUDE.md` *Where the work lives* (`0269720`) |
| **The gate this story is measured by** | `redkiln validate --kb && redkiln doctor` is AC-013's whole tier — `S`, static. `doctor` must report exactly the six expected `template-drift` advisories and no new finding. `cargo xtask lints && cargo xtask spec-trace` still runs at the story grain even though this story maps to no package, and must stay green. | `_decomposition.md:842`, `:651-653`, `:784`; `.redkiln/config.yaml:40` (and its comment), `:48` |
| **This story is the corpus's first open-question resolution** | No atom in `.kb/open-questions/` carries `withdrawn` or `superseded` today. The shape landed here is the one every later resolution copies, so it follows the two READMEs literally rather than improvising a house form. | `.kb/open-questions/` (grep for `status:` across the directory); `.kb/open-questions/README.md:40-50` |

## Data and migrations

**N/A — no schema, no database, no data migration.** This story writes no code and
touches no persisted application state; `crates/**` is outside its PR boundary.

Two things resemble a migration closely enough to be worth naming, so that neither is
mistaken for one and handled with the wrong instincts:

- **The frontmatter status transition on two atoms** is a state change on a versioned
  markdown file, not a migration. It has no forward/back script, no ordering constraint
  beyond "the answering decision atom must already exist so the link resolves", and it is
  reverted by reverting the commit. `redkiln validate --kb` is its schema check.
- **The `.kb/_intake/` staging directory** is emptied by `/redkiln:kb-ingest` as part of
  the wave (`CLAUDE.md`, repository map: "staging. `/redkiln:kb-ingest` consumes and
  clears it"). An intake document surviving in the tree at exit means the wave did not
  complete, not that a file was forgotten — treat it as a failed run, not as leftover
  scaffolding to delete by hand.

## Acceptance criteria

The persona is the one the KB's own contract names and the one this initiative has not
yet written a persona atom for (context pack 10): **a maintainer who arrives at
`.kb/maps/open-questions-index.md` to find out what phase 13 left open.** Every criterion
below is that maintainer's journey crossing the full stack — index bullet → atom
frontmatter → answering decision atom — not a capability stated in isolation. "Verified
by" names the command or diff that produces the evidence the `_ledger.md` row must cite;
this story compiles nothing, so its verifying tests are the gate's file-reading checks,
which is exactly what `.redkiln/config.yaml`'s `affected_gate` comment says such a story
is entitled to (`:36-40`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a maintainer who needs to know whether the sync message set is still an open design question, **WHEN** they follow the `sync-message-set-and-format-version` bullet from the index into the atom after this PR, **THEN** the atom's frontmatter carries the disposition ADR-0027 actually forced against its three ordered sub-questions (`.kb/open-questions/sync-message-set-and-format-version.md:83-91`) — `status: superseded` plus an outbound `superseded_by` and `related` naming the answering decision atom where all three were reached, or `status` left as-is plus a `related` link and a named residue where they were not — and no status value outside the documented enum is invented. | `redkiln validate --kb` (status enum + every id resolves); `git diff -- .kb/open-questions/sync-message-set-and-format-version.md` shows hunks inside the YAML block only; the ledger row cites the ADR-0027 atom id and the sub-question the evidence came from |
| AC-002 | **GIVEN** the same maintainer asking whether happenstance's wire format now has to interoperate with anything, **WHEN** they open `dcb-reference-publishes-no-wire-format.md` after this PR, **THEN** the atom is still **open** — a renewal, not a settlement — carrying an outbound `related` link to ADR-0026 and an annotation naming the experiment the renewal is measured against (`RUNBOOK.md:4593-4595`: "named, deferred, with the experiment being a specific external implementation to interoperate with. Not silence"), and its sub-question 3 (whether re-checking the W7 finding is phase 13's standing task, `:97-100`) is visibly settled whichever way ADR-0026 settled it. | `redkiln validate --kb`; `git diff -- .kb/open-questions/dcb-reference-publishes-no-wire-format.md` shows frontmatter-only hunks and an unchanged `status`; the ledger row cites the ADR-0026 line that names the experiment |
| AC-003 | **GIVEN** a maintainer who wants to know what was *not* known on the day the deferral was taken, **WHEN** they read either atom's prose after this PR, **THEN** it is byte-identical to its pre-PR body below the closing `---` — no "How it was resolved" section appended, no sentence reworded into its own answer, neither file deleted, renamed or moved — because the value of the record is that it shows the state of knowledge at the time (`.kb/open-questions/README.md:40-45`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md:52-56`). | `git diff -- .kb/open-questions/` inspected hunk by hunk: every hunk lies between the opening and closing `---`; `git diff --name-status -- .kb/open-questions/` shows exactly two `M` entries and no `D`/`R` |
| AC-004 | **GIVEN** a maintainer who enters *only* from the index and never opens an atom, **WHEN** they read the two bullets at `.kb/maps/open-questions-index.md:122-134` after this PR, **THEN** neither tells them a question phase 13 has acted on is untouched: each bullet keeps its position under its existing `##` section, its file link and its `kb-open-question-…` id, and changes only its leading status word and its one sentence — in the index's own *Adding an entry* form, status word first, then id, then one sentence, with a dated `Amended <date>:` clause where the question stayed open, copying the existing precedent at `:56-61` rather than inventing a house form. | `rg -n "kb-open-question-sync-message-set-undesigned-001|kb-open-question-dcb-no-published-format-001" .kb/maps/open-questions-index.md` returns both ids, each on a bullet whose leading bold word is `Open`, `Withdrawn` or `Superseded`; `git diff -- .kb/maps/open-questions-index.md` shows zero removed bullets, zero added bullets and zero new `##` headings |
| AC-005 | **GIVEN** the repository's rule that nothing under `.kb/` is hand-authored, **WHEN** this story's changes land, **THEN** both dispositions arrived through `.kb/_intake/` documents consumed by `/redkiln:kb-ingest` — the wave's record exists under `.kb/_governance/integration-waves/`, `.kb/_intake/` is back to holding only its own README at exit, and the diff touches no file under `.kb/decisions/**` (links are outbound only) and none under `spec/SPECIFICATION.md` (clause work is `clause-arithmetic-and-deferral-renewals`'). | `git diff --name-only` intersected with the PR-boundary block above returns the empty set outside it; `ls .kb/_intake/` shows no staged document; the wave record path is cited in the ledger |
| AC-006 | **GIVEN** a maintainer who trusts the KB because the gate checks it, **WHEN** the merge gate runs on this PR, **THEN** `redkiln validate --kb && redkiln doctor` is green — no dangling `superseded_by`, no accepted decision atom mutated, exactly the six expected `template-drift` advisories and no new finding — the unconditional `cargo xtask lints && cargo xtask spec-trace` stays green even though this story maps to no package, and `_ledger.md` says which of those commands produced the evidence for each AC rather than resting on one green run. | `redkiln validate --kb && redkiln doctor`; `cargo xtask lints && cargo xtask spec-trace`; `cargo xtask affected --base main`; `redkiln verify --grain story` (`require_ledger: true`, `.redkiln/config.yaml:62-67`) |

Project **AC-013** (`project.md:243-246`) is covered end to end: "both atoms reflect a
resolved state" is AC-001 + AC-002 with AC-003 policing *how*; "the index is updated in
place" is AC-004; "`redkiln validate --kb` passes" is AC-006; AC-005 is the write-path
constraint the project's own architecture brief adds (`_decomposition.md:46`, `:563-567`).

## Interaction quality

This story renders **no user-facing surface** — `_design.md` records `N/A — no
user-facing surface` for the whole project, signed off 2026-08-11, and its `## Items`
block is `N/A`. That does not make this section vacuous, and it is not discharged by
saying so. The document layer this story writes into *has* a composed surface with a
signed-off form of its own: the index's *Adding an entry* contract
(`.kb/maps/open-questions-index.md:136-143`) is the composition spec a bullet is checked
against, exactly as a design atom would be for a screen. Every invariant below is carried
by an **AC row in the table above** — none is stated only here, because `redkiln verify`
extracts ACs from table cells and bullets in this section would never be gated.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, never a context jump** — the resolution annotates where the question already lives; it does not relocate the reader to a new file or a new section | AC-003 (atoms not moved or renamed), AC-004 (bullets keep their position under the existing `##` section, no new heading) | `git diff --name-status` shows two `M` and no `D`/`R`; the index diff adds no `##` |
| **Non-occlusion** — the answer never covers the question. The record of what was not known stays fully legible beside the disposition that resolved it | AC-003 | every `.kb/open-questions/` hunk lies inside the YAML block |
| **Preserved anchors** (the document analogue of preserved focus/scroll/selection) — the reader's handles are the bullet's ordinal position, its file link and its `kb-open-question-…` id; all three survive the edit | AC-004 | `rg` finds both ids on bullets in their original section; diff shows zero bullets removed |
| **Reversibility** — the whole change is undone by reverting one commit. No forward script, no back script, no ordering constraint beyond "the answering decision atom already exists" | AC-005 (nothing outside the boundary), AC-006 (`validate --kb` green before and after) | the *Data and migrations* section above states the non-migration explicitly; ledger cites the wave commit |
| **Reachability** (replacing keyboard reachability, which has no referent here) — the disposition is reachable from the mount point in one hop, because "a question nobody can find from the map is a question that gets asked again from scratch" (`.kb/open-questions/README.md:28-29`) | AC-004 | index bullet → atom → answering decision atom, each link resolving under `validate --kb` |

**COMPOSITION invariants** — taken from the index's own form, since `_design.md` declares
no surfaces to take them from:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — each bullet is a real composed entry (bold status word, linked filename, parenthesised id, one sentence), never a bare id, a bare path, or a status word with no sentence | AC-004 | `git diff` on the index read against `:136-143`; the two rewritten bullets match the shape of the twenty already there |
| **Composition and placement** — both bullets stay under *Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)*; no question is promoted to its own section for having been resolved | AC-004 | zero new `##` headings in the index diff |
| **Transience** — the question record is persistent chrome and is never revealed-then-removed; the only transient layer is `.kb/_intake/`, which must be **gone** at exit, and a surviving intake document means the wave failed rather than that a file was forgotten | AC-003 (persistence), AC-005 (transience discharged) | `ls .kb/_intake/`; the wave record under `.kb/_governance/integration-waves/` |
| **Density budget, with the real numbers** — exactly **2** atoms modified, **2** index bullets rewritten, **0** bullets removed, **0** bullets added, **0** new `##` sections, **0** lines changed below either atom's closing `---`, **0** files under `.kb/decisions/**`; at most **2** outbound link ids added per atom (`related`, and `superseded_by` only where `status` actually moved); **1** sentence per bullet plus at most **1** dated `Amended <date>:` clause | AC-003, AC-004, AC-005 | counted directly off `git diff --stat` and the two file diffs |
| **Hierarchy** — status word first, then the id, then the sentence; the atom carries the "what is true today / what is not decided / what forces it" structure and the index does not repeat it | AC-004 | `:136-143` read against the two rewritten bullets |
| **Anti-pattern: rewriting a question into its own answer** (`.kb/open-questions/README.md:40-45`) | AC-003 | frontmatter-only hunks |
| **Anti-pattern: deletion or de-listing** — a withdrawn or superseded question stays listed, annotated (project DR-10, `project.md:178-180`) | AC-003, AC-004 | no `D` in `--name-status`; no removed bullet |
| **Anti-pattern: symmetry for tidiness** — giving both atoms the same disposition because the diff reads better. The DCB atom is expected to stay open; matching it to the other one is a lie told to the next reader | AC-002 | `status` unchanged on the DCB atom, with the renewal's named experiment cited in the ledger |
| **Anti-pattern: hand-authoring under `.kb/`** — the directory layout of the process without the process (`0269720`) | AC-005 | the wave record exists and `.kb/_intake/` is clear |
| **Anti-pattern: the reciprocal back-link** — editing an accepted decision atom to point back at the question. `validate --kb` wants resolvable ids, not reciprocity | AC-005 | `.kb/decisions/**` absent from `git diff --name-only` |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The merged ADR does not reach one of the atom's ordered sub-questions (most likely sub-question 3 of the message-set atom: whether removing the per-message `format_version` is a breaking change). | The atom **stays open** and is amended with the residue named, so the next reader inherits a narrowed question rather than a stale one. Do **not** close the gap by reasoning inside this story — an open question closed by an argument that lives only in a backlog item is a decision with no authority behind it. Raise it against ADR-0027 instead. |
| EC-002 | A `superseded_by` or `related` id does not resolve — usually because the answering decision atom's id differs from the one assumed, or its slice-mate has not merged. | `redkiln validate --kb` fails, and that is the correct outcome. Fix by using the **real** id from the merged atom, never by deleting the link to make the gate green and never by hand-creating the referent. If the mate genuinely has not merged, this story is not startable: it is a hard predecessor (`_storymap.md:119-121`). |
| EC-003 | `/redkiln:kb-ingest` finishes but `.kb/_intake/` still holds a staged document. | Treat as a **failed wave**, not as leftover scaffolding. Do not delete the file by hand and do not proceed: the wave is the writer, and a partial wave means some mutating op did not apply. |
| EC-004 | The ingest wave proposes an op that edits an atom under `.kb/decisions/**` (e.g. adding a reciprocal `related` back-link) or reaches outside the PR boundary. | Decline the op. Accepted decision atoms receive exactly one kind of edit — the supersession metadata flip — and this story is not that (`.kb/decisions/README.md:9-13`). The boundary check failing is the intended mechanism, not an obstacle to route around. |
| EC-005 | `redkiln doctor` reports a seventh `template-drift` advisory or any new finding. | Halt and report. Six is the asserted set (`CLAUDE.md`, *Where the work lives*); a seventh is a template changed without a decision and is never silenced from inside this story. Never run `redkiln adopt --templates`. |
| EC-006 | A status value outside the documented enum (`draft` · `proposed` · `accepted` · `superseded` · `withdrawn`) is proposed — e.g. `resolved`, `renewed`, `closed`. | Rejected by `validate --kb`; the correct expression of "renewed" is `status` unchanged plus a dated index annotation (AC-002), and of "settled" is `superseded` (AC-001). The vocabulary is not extended by this story. |
| EC-007 | An intake document is tempted to invent `tracks` or `resolution_ref` on an atom that does not carry them. | Forbidden by `.kb/open-questions/README.md:46-50` — `KbFrontmatter` passes unknown keys through, so this would validate and still be wrong. Neither atom carries either key today; the pointer travels on `related` / `superseded_by`. |

## Non-functional

| id | Requirement | Why, and how it is judged |
| --- | --- | --- |
| NF-001 | **Zero compile surface.** This story maps to no workspace package; `cargo xtask affected --base main` will therefore compile nothing, and the five file-reading lints plus `spec-trace` are what actually run. Both must be green. | `.redkiln/config.yaml:36-40` names precisely this case — "a story whose whole deliverable is an edit to SPECIFICATION.md maps to no package, and a purely package-shaped gate would compile nothing, read nothing, and call it green". |
| NF-002 | **One hop to the answer.** From the index bullet, the maintainer reaches the disposition in one link and the answering decision atom in two, with no intermediate document that only forwards. | The failure this avoids is the one `.kb/open-questions/README.md:28-29` names: a question re-asked from scratch because the map did not carry its state. |
| NF-003 | **Precedent quality.** No atom in `.kb/open-questions/` is `withdrawn` or `superseded` today, so whatever shape lands here is the shape every later resolution copies. It must be derivable from the two READMEs alone, with no invented convention a later author would have to reverse-engineer from this diff. | Judged by reading the result against `.kb/open-questions/README.md:40-50` and `.kb/maps/open-questions-index.md:136-143` and finding no rule that is only in this PR. |
| NF-004 | **Revert-clean.** The change is undone by reverting one commit, with no data migration, no ordering constraint beyond link resolution, and no state left in `.kb/_intake/`. | Stated in *Data and migrations* above; verified by AC-005's empty intake and by `validate --kb` being green on both sides of the revert. |
| NF-005 | **Authority hygiene.** Not one sentence of new reasoning about the message set, the per-connection/per-message fork, or DCB interoperability appears anywhere in this PR — including in the intake documents, whose job is to state the disposition and cite the ADR line that forces it. | An intake document that argues is an ADR written in the wrong place; reviewed by reading each intake doc for a claim with no `file:line` behind it. |

## Implementation notes (non-prescriptive)

Shape, not instructions. Any route that satisfies the ACs is acceptable.

1. **Read the two merged ADRs before writing anything.** For the message-set atom, walk
   ADR-0027 against the three ordered sub-questions in order and write down, per
   sub-question, the `file:line` that answers it or the fact that nothing does. For the
   DCB atom, read ADR-0026's envelope section for WF-1's interoperability half and for
   the named experiment. The disposition is a *function* of those two readings; if you
   find yourself constructing an argument, you have crossed into EC-001.
2. **Also check what the slice-mate story `message-set-on-the-envelope` did to
   `FORMAT_VERSION`** (`_storymap.md:63`). It is in a later slice, so at this story's
   merge point it will normally be unlanded — which is itself evidence about
   sub-question 2's residue, not a reason to wait.
3. **Stage one intake document per atom**, each stating: the atom id, the proposed
   `status` (or "unchanged"), the exact link ids to add, the one-sentence index bullet
   to write, and the ADR `file:line` forcing each. Nothing else. Then run
   `/redkiln:kb-ingest` and let the wave write.
4. **Verify the wave rather than trusting it.** `git diff -- .kb/open-questions/` hunk by
   hunk for frontmatter-only changes; `git diff --name-only` against the PR boundary;
   `ls .kb/_intake/`; then the gate.
5. **Expect asymmetry and do not launder it.** The most likely correct outcome is one
   `superseded` atom and one still-open, amended atom. A reviewer seeing two identical
   dispositions should be more suspicious, not less.
6. **Ledger last.** Fill `_ledger.md` from evidence you actually produced — the atom
   `file:line` and the command whose output you read — not from a green gate run, per
   the testing brief's warning that a green `ci --fast` never on its own proves a
   specific AC's assertion ran (`_decomposition.md`, *Merge-gate commands*).

## Tests and CI (merge gate)

Grounded in the project testing brief's tier table and *Merge-gate commands*
(`_decomposition.md`, *Testing brief*; AC-013's row is **S** only, `:842`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| S (story grain, unconditional) | `cargo xtask affected --base main` | The story-grain gate runs even though the diff maps to no package; the five file-reading lints and `spec-trace` are what execute (`.redkiln/config.yaml:36-40`) — NF-001 |
| S (reachability) | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | No clause cites a rule that does not exist and no rule is orphaned, unchanged by this diff — NF-001, and the guard that this story did not perturb `spec/SPECIFICATION.md` |
| S (**the AC-013 tier**) | `redkiln validate --kb` | `KbFrontmatter` conformance on both atoms, every `related` / `superseded_by` id resolves, no accepted decision atom mutated, no status outside the enum — AC-001, AC-002, AC-006, EC-002, EC-004, EC-006 |
| S (**the AC-013 tier**) | `redkiln doctor` | Exactly the six expected `template-drift` advisories and no new finding — AC-006, EC-005 |
| S (diff shape) | `git diff -- .kb/open-questions/` · `git diff --name-status -- .kb/open-questions/` | Every hunk inside the YAML block; exactly two `M`, no `D`/`R` — AC-003, and the non-occlusion / transience invariants |
| S (diff shape) | `git diff -- .kb/maps/open-questions-index.md` | Zero bullets removed, zero added, zero new `##`, both ids still present with a valid leading status word — AC-004, and the composition / density / hierarchy invariants |
| S (boundary) | `git diff --name-only` read against the PR-boundary block | Nothing under `.kb/decisions/**` or `spec/SPECIFICATION.md`, nothing in `crates/**` — AC-005, EC-004 |
| S (write path) | `ls .kb/_intake/` · the wave record under `.kb/_governance/integration-waves/` | The dispositions were written by `/redkiln:kb-ingest`, and its staging is cleared — AC-005, EC-003 |
| S (ledger) | `redkiln verify --grain story` (`require_ledger: true`, `.redkiln/config.yaml:62-67`) | Every AC-### in this spec has a ledger row, satisfied, with non-placeholder evidence — AC-006 |
| I (project ceiling, not this story's alone) | `cargo xtask ci --fast` (`.redkiln/config.yaml:50-55`) | Project DoD 1. This story cannot break it — it compiles nothing — but the slice does not merge without it |

**No new conformance rule, and none is owed.** Nothing here is adapter-observable, so a
rule would be decorative under `CLAUDE.md`'s "a rule that no adapter can fail" corollary;
CF-29's changelog-per-rule obligation is therefore not triggered.

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Containment |
| --- | --- | --- |
| **The ADR did not actually answer the question** — the story is scheduled as if the answer exists, and an implementer under slice pressure fills the gap with plausible reasoning. | This is the single highest-probability failure, and it produces a decision with no ADR behind it that later readers will cite as settled. | EC-001 and context pack 1 make "raise it, do not fill it" the required behaviour; NF-005 makes an arguing intake document a review defect. |
| **Symmetry pressure** — two atoms, two dispositions, and one of them looks unfinished. | `RUNBOOK.md:4593-4595` asks the DCB question to be *renewed*, so the correct diff is asymmetric and looks like an oversight to a reviewer skimming it. | AC-002 states the renewal as the criterion; the *Executive summary* and this table both name the trap so review sees it as intended. |
| **Hard predecessor coupling** — both dispositions depend on ids that do not exist until the two ADR stories merge. | Starting early yields dangling links (EC-002) and a red gate that looks like a KB problem rather than a sequencing one. | Merge order 1 (`_storymap.md:119-121`) puts this story third in its own slice; the gate failure mode is documented so it is read correctly. |
| **The ingest wave over-reaches** — a merge/amend op touching a decision atom or an unrelated open question. | The wave is the writer and is designed to prefer merge/amend; the boundary is the only thing stopping it. | EC-004 plus the PR-boundary block, which `redkiln verify --grain story` enforces mechanically. |
| **Precedent lock-in** — this is the corpus's first resolution, so any improvisation becomes convention. | Cheap to get right now, expensive to unwind across twenty atoms later. | NF-003 requires the shape be derivable from the two READMEs; AC-004 requires copying the existing `:56-61` amendment precedent rather than inventing one. |
| **Coupling out** — none into `crates/**`. | This story blocks the slice, not any compilation unit. | `crates/**` is outside the PR boundary; no `.rs` file, no gate wiring, no `CHANGELOG.md` entry. |

## Dependencies

**Blocks on** (hard predecessors, both in this same `decisions-of-record` slice, both
merged before this story starts — `_storymap.md:55-57`, merge order at `:119-121`):

- `adr-0026-peer-ingest-and-transport` — supplies WF-1's interoperability disposition in
  its envelope section and the named experiment the DCB renewal is measured against, plus
  the answer to that atom's sub-question 3. Without it AC-002 has nothing to cite.
- `adr-0027-merge-compensation-and-message-set` — supplies the message set and the derives
  on `PushBatch`/`EventGroup`/`ReplicatedEvent`, which is what the message-set atom's three
  ordered sub-questions are asked against. Without it AC-001 has nothing to read.

Both dependencies are *evidence* dependencies, not build dependencies: this story cannot
be written correctly before them, and cannot be usefully started early, because its whole
output is a function of what they landed.

**Unlocks:**

- The slice itself, and therefore the merge order's rule that "nothing in a `.rs` file
  that a clause constrains may merge before this slice does" (`_storymap.md`, *Merge
  order* 1) — the next slice, `ingest-seam-and-memory-oracle`, follows.
- Project **AC-013** (`project.md:243-246`), of which this story is the sole claimant
  (`_storymap.md:91`), and project **DoD 5** (`project.md:271-274`).
- Initiative **DoD 14** (`initiative.md:398-401`) — the ADRs land its first clause; this
  story is the only thing in the initiative that lands its second and third.

No story depends on this one for a symbol, a type or a file it imports.

## Anchors (progressive disclosure)

Deferred depth, signposted and AC-bound. Link and open at the stated moment; never paste
in bulk. Every path verified present in the worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.kb/open-questions/sync-message-set-and-format-version.md` | The atom being dispositioned; `:83-91` holds the three ordered sub-questions that ADR-0027 must be walked against, in order — the disposition is read off them one at a time, not off the ADR's summary. | First, before drafting the message-set intake document. | AC-001 |
| `.kb/open-questions/dcb-reference-publishes-no-wire-format.md` | The atom expected to stay open; `:78-85` explains why the deferral is "unfalsifiable by construction", and `:97-100` is sub-question 3, the one part phase 13 can settle outright. | Before drafting the DCB intake document, and again when writing its index annotation. | AC-002 |
| `.kb/open-questions/README.md` | The binding contract for *how* a question is resolved: `:40-45` (new atom, record stays, status moves, body untouched), `:46-50` (do not invent `tracks` / `resolution_ref`), `:28-29` (findability from the map is the point). | Before the first intake document, and again before accepting the wave's output. | AC-001, AC-002, AC-003, EC-007 |
| `.kb/maps/open-questions-index.md` | The mount point *and* the composition spec: `:136-143` is the *Adding an entry* form, `:56-61` is the existing open-but-amended precedent to copy, `:122-134` is where the two bullets live. | While writing each bullet — the form is checked against this file, not against taste. | AC-004 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | `:52-56` gives the test for whether an edit is allowed ("does it change what the document *asserts*"), and `:80-85` is the worked ADR-0006/0007 instance whose shape this story reproduces: metadata changed, prose verbatim. | When tempted to add a "How it was resolved" paragraph, or when reviewing the diff for AC-003. | AC-003 |
| `.kb/decisions/README.md` | `:9-13` states that the supersession metadata flip is the only edit an accepted decision ever receives, and `:41-43` that a decision needs an atom behind it — together they are why `.kb/decisions/**` is outside the boundary and why no back-link is added. | Before adding any link, and if the ingest wave proposes touching a decision atom. | AC-005, EC-004 |
| `.kb/README.md` | `:45-46` — `validate --kb` requires every link id to resolve but does **not** require reciprocity; the *Atoms* table carries the `status` enum this story may not extend. | When choosing between `related` and `superseded_by`, and on any `validate --kb` failure. | AC-001, AC-006, EC-002, EC-006 |
| `RUNBOOK.md` | `:4593-4595` is the instruction that WF-1's disposition be a **renewal against a named experiment** ("Not silence"); `:4516-4620` is phase 13 in full, including what the phase owes beyond this story. | Before writing the DCB disposition — it is the sentence that makes AC-002 a renewal rather than a settlement. | AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | `:46` and `:563-567` (AC-A09) bind the write path to `.kb/_intake/` + `/redkiln:kb-ingest`; `:336-343` places the DCB disposition inside ADR-0026's envelope section; `:842` and `:651-653` fix AC-013's tier as **S** = `validate --kb` + `doctor`. | Before staging intake documents, and when filling the ledger's `verifying_test`. | AC-005, AC-006, AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md` | `:55-57` are the three slice rows and this story's `depends_on`; `:119-121` is the merge order that makes both ADRs hard predecessors; `:91` records this story as AC-013's sole claimant. | If the sequencing is ever in doubt, or if a dependency looks unmerged. | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` | `:243-246` is project AC-013 verbatim (what "resolved" is measured as); `:178-180` is DR-10 (annotate, never remove); `:271-274` is DoD 5. | When judging whether the disposition clears the project bar rather than merely satisfying the gate. | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:398-401` is DoD 14, the initiative-level scenario this story is the only claimant of clauses two and three of. | At exit, when writing the ledger's closing evidence. | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | Records `N/A — no user-facing surface` for the whole project with its `## Items` block `N/A` — the authority for this story rendering nothing, and for the composition invariants being taken from the index's own form instead. | Only if someone proposes a rendered surface here. | AC-004 |
| `spec/SPECIFICATION.md` | `:1892-1900` is WF-1 with its `[DEFERRED]` marker — cited as a **referent** for the DCB atom's context, never as a target. Its marker belongs to `clause-arithmetic-and-deferral-renewals` under project AC-010. | Read-only, when confirming what WF-1 currently says. Never edit from this story. | AC-002, AC-005 |
| `.redkiln/config.yaml` | `:36-40` is why a story mapping to no package still gets a real gate; `:48` is `reachability_static`; `:62-67` is `require_ledger`. | When running the gate and when filling `_ledger.md`. | AC-006, NF-001 |
| `.kb/_governance/integration-waves/` | Where `/redkiln:kb-ingest` records the wave; the record's existence is the evidence that the ingest path — not a hand edit — wrote the atoms. | After the wave, to cite the record path in the ledger. | AC-005 |

## Clarifications resolved during spec

1. **The AC set is exactly the six the front half decided** — AC-001 … AC-006. None added,
   none dropped. AC-005 (the write path) and AC-006 (the gate) were already stated as
   normative in the *Behavior and interfaces* table and the *Integration contract*; they
   are enumerated here as ACs so that `redkiln verify` can extract and gate them, since a
   claim stated only in prose gets no ledger row.
2. **"Resolved" in project AC-013 means *dispositioned*, not *answered*.** Settled by
   context pack 5 and made mechanical here: AC-001 admits both the `superseded` and the
   still-open-with-residue outcome, and AC-002 requires the DCB atom to remain open. A
   spec that demanded two closures would have forced a false one.
3. **Interaction quality is not skipped for a story with no screen.** `_design.md`
   declares no surfaces, so the composition family is taken from
   `.kb/maps/open-questions-index.md:136-143`, which is the signed-off form for the one
   composed artefact this story writes. Every invariant is bound to an AC row rather than
   left as prose, per RFC §6.7/D6's extraction rule.
4. **No persona atom is cited, because none exists.** `.kb/product/` holds only its README;
   the personas are initiative AC-15's and arrive at closeout. The maintainer-at-the-index
   reader is named from `.kb/open-questions/README.md:28-29` instead of from an invented
   persona id — a fabricated citation would be worse than an honest absence.
5. **No conformance rule and no clause change**, confirmed against both corollaries in
   `CLAUDE.md`: nothing here is adapter-observable, and WF-1's marker belongs to
   `clause-arithmetic-and-deferral-renewals` (project AC-010), not to this story.
6. **The index bullets' line range is cited as `:122-134`** to stay consistent with the
   front half's *Behavior and interfaces* table. The range is the two bullets plus the
   blank line before *Adding an entry*; the implementer should locate them by id
   (`rg -n "kb-open-question-dcb-no-published-format-001"`) rather than by line, since the
   slice-mates may have appended entries above them.
