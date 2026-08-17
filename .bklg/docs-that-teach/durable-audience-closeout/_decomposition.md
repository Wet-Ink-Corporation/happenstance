# Briefs — The Durable, Reconciled Audience (HS-P0025)

This project's briefs artifact. It carries the `architecture`, `ux` and `testing`
briefs this project warrants (`.bklg/docs-that-teach/_decomposition.md:322`), each
under its own `##` heading, in the Intent / Acceptance Criteria / Notes shape of
`.redkiln/templates/briefs/brief.md`. Read it alongside
[`project.md`](project.md) (the mandate, DR-1 … DR-14 and AC-001 … AC-018) and
[`_grounding.md`](_grounding.md) (what was verified against this worktree).

## UX brief

### Intent

Design the **durable product layer this project leaves behind** as a surface a
stranger has to use, rather than as a directory this project has to fill.

This project ships no screen. Its user-facing surface is four artefacts, and the
charter says so directly — its `_design.md` "surface is the promoted atoms
themselves — what a persona atom must state so the next initiative can frame
acceptance criteria from it" ([`project.md`](project.md), risk table, "This project
owns no design tension"):

1. **The promoted atoms** under `.kb/product/` — persona atoms (`kind: concept`)
   and journey atoms (`kind: playbook`), both `authority_tier: product`
   (`.kb/product/README.md:6-13`).
2. **The navigation surfaces** that make them findable — an appended `##` section
   in `.kb/maps/domain-map.md` (AC-010) and, for any question that becomes an
   `open_question` atom, an appended bullet in `.kb/maps/open-questions-index.md`
   (AC-017).
3. **The reconciliation record** — the written adjudication of this initiative's
   three personas against HS-S0131's four staged ones (DR-1, AC-001, AC-002).
4. **The Definition-of-Done re-observation ledger** and the DT-1 … DT-10 audit
   table (AC-014, AC-015, AC-018).

The scoping question the brief answers is *who reads these and to what end*, and
the answer is not the three documented personas. They are the **subject** of this
surface, never its audience — an application author never opens
`.kb/product/`, and an atom drifting into being documentation *for* the persona it
describes is the first sign this brief was ignored. `.kb/product/README.md:38-40`
draws the same line from the other side: this layer records who the work is for,
and stays separable from what anyone is expected to build.

### Who this is actually for, framed as intent

| | Reader | What they are trying to do | Where the intent is grounded |
| --- | --- | --- | --- |
| **U1** | The **next initiative's charter author** (primary) | "Tell me who this library is for and to what end, with the confidence attached to each claim, so I can write `GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome>` without re-running discovery." | `.kb/product/README.md:15-21` — a charter cites these atoms in `## Referenced personas & journeys` and frames its criteria from them |
| **U2** | The **closeout reviewer** — the human at the ingest approval gate and at the pull request | "Show me every adjudication you made, including the ones that changed nothing, so I can disagree with exactly one of them without reading the discovery corpus." | AC-001 ("no pair is unaddressed"), AC-006 ("removing any one is a review-blocking defect"), `.kb/_intake/README.md` — the human reviews the wave by merging its branch |
| **U3** | The **future maintainer reconciling a third persona set** | "Let me see what was decided, against which tree, on what evidence — and correct it without editing it." | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; AC-012 (name the merged tree by ref or sha) |
| **Not a user** | The application author, the adapter author, the evaluator | Nothing. They never open `.kb/product/`. | `.kb/product/README.md:38-43` |

U1 is the acceptance-shaped reader: every invariant below exists because U1 must be
able to *act* on the atom — frame a criterion, inherit a caveat — from one open
file. U2 is the falsification-shaped reader: every invariant about completeness and
non-occlusion exists because U2 must be able to find the one thing they disagree
with.

### The states this surface has, stated from intent

Each state below is a state the reader needs to *see*, not a state the tooling
needs to track. Every one of them must be a **word written in the artefact**.

| Surface | States | Why the reader needs the state named |
| --- | --- | --- |
| A persona atom's evidence | `directly observed` (naming HS-P0024's session) / `still inferred` | U1 must know which claims they may frame a criterion from and which they must re-check. AC-007 forbids the blanket "none has been directly observed" |
| A reconciliation pair | `merged` (surviving atom named) / `superseded` (superseded source named) / `kept distinct` (distinguishing goal or fear named) | DR-1: silence on a pair is a defect. U2 disagrees per-pair, so the pair is the unit |
| The merge order | counterpart **ran** / counterpart **did not run** | AC-002: the record must read coherently in the case HS-S0131 never ran and this initiative's set stands as the authored one — the case `_grounding.md` confirms is live today |
| The evaluator question | decided as `a persona in its own right` / decided as `an earlier stage of the application author's journey` | AC-003: one decision, one place, with the reasoning — not a question left open in two distillations |
| A charter open question | `answered on the record` / `open_question atom` | AC-017; index bullets state `Open` / `Withdrawn` / `Superseded` first (`.kb/maps/open-questions-index.md:136-143`) |
| A design tension DT-1 … DT-10 | `resolved` / `recorded deferral` / **`gap`** | AC-018: a tension with neither is *reported as a gap*, not passed over |
| A DoD scenario | `re-observed` with named observer, tree ref and what was seen | AC-014: "inherited from a sibling" is not a state this surface may express |
| DoD scenario 2 | two halves — `failed by name` and `recovered` — each with its own evidence | AC-015: the failing half is recorded, not just the recovery |

### Design-system primitives — compose these, do not hand-roll

There is no CSS in this repository, and pretending otherwise would produce a
decorative brief. The honest primitive layer for a text corpus is the **atom
template, the frontmatter vocabulary, and the two map formats** — and they behave
exactly like a token layer: they are the shapes `redkiln validate --kb` and the
reviewer already know how to read, and anything invented beside them is the
equivalent of bespoke CSS in a repository that has a design system.

| Primitive | Path | What it fixes |
| --- | --- | --- |
| **The atom** | `.kb/_templates/atom.md` | Frontmatter keys (`id`, `title`, `kind`, `status`, `authority_tier`, `summary`, `depends_on`, `related`, `source_paths`, `last_reviewed`) and the body skeleton `## Context` / `## Body` / `## Consequences / links` |
| **The tier tokens** | `.kb/product/README.md:6-13` | Persona → `kind: concept` + `authority_tier: product`. Journey → `kind: playbook` + `authority_tier: product` |
| **The persona content vocabulary** | `.kb/product/README.md:11-13` | The four slots: goal, context, **what they already do instead**, what they are afraid of. A journey is the moment-by-moment path, written so you can tell whether a build improved it |
| **Concept-atom exemplar** | `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | How a `concept` atom composes: a folded `summary:` that carries the whole argument, `related:` by id, `source_paths:` citing both the `.kb/_intake/…` staging file and the long-form evidence (`:23-28`) |
| **Playbook-atom exemplar** | `.kb/playbooks/one-decision-per-adr-title.md` | How a `playbook` atom composes — the same shape a journey atom takes |
| **The domain-map section** | `.kb/maps/domain-map.md:144-150` (format visible at `:105-142`) | Append a `##`; group entries by kind under bold labels (**Reference**, **Concepts**, **Governance**, **Playbooks**, **Open questions**); cite each atom's id next to its link; one sentence of orientation, because the atom is the source of truth and the map is not |
| **The index bullet** | `.kb/maps/open-questions-index.md:136-143` | Status word first, then the link, then the id, then one sentence — the grounding lives in the atom |
| **The open-question body** | `.kb/open-questions/README.md` | "What is true today / what is not decided / what forces it", with `source_paths` grounding each claim |
| **The design-atom skeleton** | `.kb/design/README.md` | Pattern / Rejected / Mitigates / Holds when / Stops holding when. Listed for completeness: this project owns no design tension (`.bklg/docs-that-teach/_decomposition.md:161-163`), so expect not to use it |
| **The only installer** | `/redkiln:kb-ingest` over `.kb/_intake/*.md` (`.kb/_intake/README.md:5-7`) | The single write path. An atom that did not come through it does not exist (AC-008; precedent `0269720`) |
| **The validators** | `redkiln validate --kb` (AC-011); `cargo xtask ci`, wired at `.redkiln/config.yaml:60` (AC-016) | The only automated readers of this surface |

**Hand-rolling, explicitly forbidden here** — each of these is the text-corpus
equivalent of writing bespoke CSS next to a token layer:

- a frontmatter key that is not in `.kb/_templates/atom.md`;
- a persona section vocabulary that is not the product README's four slots (a
  "Persona card", a "TL;DR", a bespoke table per atom);
- a status column carried by an emoji, a tick, a colour word or an empty cell;
- a subdirectory under `.kb/product/` — the corpus is flat per layer;
- a new map format, or a new `##` in `open-questions-index.md` when the question's
  domain already has a section (`.kb/maps/open-questions-index.md:136-140`);
- an atom written by hand into `.kb/`, whatever its shape.

**The single highest-probability defect in this brief's whole surface** is the
authority tier. Every atom an author will look at while composing — the concept
exemplar (`:6`), the map atoms (`:6`) — carries `authority_tier: note` or
`guideline`. The product layer's own README requires `product`. Copy the tier from
`.kb/product/README.md:6-13`, never from the neighbour you are imitating.

### Accessibility floor

This project renders no HTML and authors no colour, so state the floor and its
trigger condition rather than skipping it silently:

- **WCAG AA contrast** — no colour is authored by this project's diff; the
  obligation transfers unchanged to any rendered page, and the rendered
  documentation surface is HS-P0020's (`.bklg/docs-that-teach/_decomposition.md:317`,
  the only project earning a `deployment` brief). If this project ever adds a
  rendered page or a diagram, AA applies to it and this line stops being N/A.
- **Colour, glyph and position never alone** — this one binds *today*, in
  markdown. Every state in the table above is a word. No ✅/❌, no 🟢, no
  strikethrough-as-status, no "a blank cell means fine", no state inferable only
  from a row's absence. Corpus precedent is direct: the open-questions index states
  `Open` / `Withdrawn` / `Superseded` as literal words, first
  (`.kb/maps/open-questions-index.md:136-143`).
- **Reduced motion** — N/A, no motion surface exists; same transfer condition as
  contrast.
- **Link purpose from link text alone** (WCAG 2.4.4) — every link names the atom,
  the path or the section it leads to. Never "here", "this", "see above". The
  corpus already reads this way throughout the two maps.
- **Heading structure** — one `#` per file, sections at `##`, no skipped level.
  This is also what makes an atom's `## Context` / `## Body` skeleton legible.
- **Linear readability** — every row of the DoD ledger and the DT audit is
  self-contained: it names its scenario or tension, its observer, its tree ref and
  its outcome, with no row that only makes sense read after the one above. A
  reader consuming one row at a time must lose nothing.
- **Plain-text equivalence for load-bearing claims** — a verdict that exists only
  as a table cell is a verdict that will be lost in a quote or a diff. Where a
  reconciliation outcome or a DT gap is load-bearing, it is also stated in a
  sentence.

### Interaction-quality invariants

These are requirements, not aspirations, and each carries the falsifier that makes
it testable.

**IQ-1 — In place, not a context jump.** A persona atom answers *who, to what end,
and with what confidence* on first load. Links carry evidence and provenance; they
never carry one of the four required slots. **Falsifier:** an atom whose "what they
are afraid of" or "what they already do instead" reads "see
`_discovery/distillation/personas-and-journeys.md`". U1's whole reason for existing
is that they should not have to open the discovery corpus.

**IQ-2 — Non-occlusion: a filter must not hide what it filters.** Four concrete
obligations, because this surface has four places to violate it:

- *The ledger, not the summary.* The reconciliation record lists **every** pair,
  including pairs that resolved to no change and pairs kept distinct. A summary
  sentence may precede the ledger; it may never replace a row (DR-1, AC-001).
- *The map indexes, it does not gate.* Every promoted atom stays reachable by
  directory listing and by grep; the domain-map section is one path to it, not the
  path. The map's own rule says as much: "the atom itself is the source of truth,
  not this map" (`.kb/maps/domain-map.md:148-150`).
- *Supersession annotates, it does not delete.* The corpus already does this: a
  withdrawn or superseded question "stays listed, annotated, rather than removed —
  the record that it was once open is itself worth keeping"
  (`.kb/maps/open-questions-index.md:11-13`). A superseded persona is named in the
  record, not erased from it.
- *Clearing `.kb/_intake/` must not hide the evidence.* A successful ingest clears
  the directory (AC-009, `.kb/_intake/README.md:14-19`). Non-occlusion is preserved
  by `source_paths` citing **both** the `.kb/_intake/…` staging path and the
  `.bklg/docs-that-teach/_discovery/…` artefact (AC-005) — exactly the pattern
  `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:23-28` uses.

**IQ-3 — Preserved position (the text analogue of preserved focus and scroll).**
Nothing this project lands may move an anchor someone else already cited. Maps are
appended to, never edited (AC-010); no existing atom id is renamed or reused; no
pre-existing section is reflowed in a way that shifts a cited `file:line`. This is
also why the AC-013 spec-trace re-run is looking for *additions* rather than
renumbering — clause ids are stable names (`spec/SPECIFICATION.md:280`).
**Falsifier:** `git diff .kb/maps/domain-map.md` showing a `-` line anywhere
outside the appended region.

**IQ-4 — Reversibility.** The whole wave — new atoms plus the removal of the staged
sources — lands as one commit on its own branch, and the human reviews it by
merging (`.kb/_intake/README.md`, "A successful ingest clears this directory"). A
promoted atom later found wrong is corrected by a **new atom that supersedes it**,
never by editing an accepted one
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`), and never by an ADR
written as a closeout by-product ([`project.md`](project.md), "Out of scope"). The
one act review cannot undo cheaply — sweeping `.kb/_intake/README.md` in as if it
were an atom — is prevented **at the ingest approval gate, before the commit**, not
repaired afterwards (AC-009; the glob is literally `.kb/_intake/*.md`,
`.kb/_intake/README.md:5-7`).

**IQ-5 — Reachable without prior knowledge (the keyboard-reachability analogue).**
A reader who has never heard of this initiative reaches every atom this project
lands, starting from `.kb/README.md`, in at most two hops, with link text that
names the destination at each hop. **Falsifier:** an atom findable only by knowing
the initiative slug, or only by `ls .kb/product/`.

**IQ-6 — Preserved selection: prior citations keep meaning what they meant.**
Promotion does not fork the audience. The discovery distillation is not deleted, it
is cited — every promoted atom names
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` in
`source_paths` (AC-005), so a charter that already cited the distillation and a
charter that cites the atom are pointed at the same audience rather than two.

**IQ-7 — The qualification travels with the claim.** The three evidence
qualifications (DR-4, AC-006) sit adjacent to the claims they qualify — inside the
persona's `summary:` and body — not collected in a trailing "caveats" section a
reader who scans the four slots will never reach. Hiding a caveat behind a
disclosure is how `.kb/product/README.md:23-24, 28-31` says a guess acquires the
standing of a finding.

**IQ-8 — Every state change is legible at the moment of reading.** No state in the
table above is expressed by presence, absence, ordering or omission. This is IQ-2
and the colour-never-alone floor meeting at the same place, and it is the invariant
AC-007 turns into a review-blocking defect.

### Acceptance Criteria

Prefixed `AC-UX-` to keep them distinct from `project.md`'s AC-001 … AC-018; each
names the project criterion it serves. Every one is observable by a reviewer with
the tree in front of them.

- **AC-UX-01** *(serves AC-004, AC-006)* — Opened alone, with no other file, each
  persona atom yields all four slots `.kb/product/README.md:11-13` names — goal,
  context, what they already do instead, what they are afraid of — plus its
  observation-status word. A slot that requires following a link is a defect.
- **AC-UX-02** *(AC-004)* — Every non-README file under `.kb/product/` carries
  `authority_tier: product`, and personas carry `kind: concept` while journeys
  carry `kind: playbook`. Checkable with a single `rg "^(kind|authority_tier):"
  .kb/product`.
- **AC-UX-03** *(AC-001, AC-002)* — The reconciliation record is a complete ledger:
  one row per persona pair and per journey pair on both sides, each carrying a
  state word (`merged` / `superseded` / `kept distinct`) and a named reason. The
  merge-order case is stated in prose immediately above the ledger, and the ledger
  reads correctly under the case that actually held. No summary line stands in for
  a row.
- **AC-UX-04** *(AC-006, AC-007)* — Each of the three evidence qualifications
  appears adjacent to the claim it qualifies. No atom collects its qualifications
  only in a trailing caveats block, and no atom carries an unqualified blanket
  "none has been directly observed".
- **AC-UX-05** *(AC-005)* — Every promoted atom's `source_paths` cites both its
  `.kb/_intake/…` staging path and at least one artefact under
  `.bklg/docs-that-teach/_discovery/`, and every cited path resolves in the tree at
  closeout.
- **AC-UX-06** *(AC-010, IQ-3)* — `git diff` over `.kb/maps/domain-map.md` and
  `.kb/maps/open-questions-index.md` contains **no deletion lines**. New content is
  an appended `##` section in the domain map and appended bullets under the
  appropriate existing section in the index.
- **AC-UX-07** *(AC-017)* — Every index bullet this project adds states the status
  word (`Open` / `Withdrawn` / `Superseded`) first, then the link, then the atom
  id, then one sentence — the shape `.kb/maps/open-questions-index.md:136-143`
  fixes. The domain-map section groups its entries by kind under bold labels and
  cites each atom's id beside its link.
- **AC-UX-08** *(IQ-5)* — A recorded two-hop reachability walk exists: starting
  file `.kb/README.md`, the hops named, and every atom this project landed reached.
  Each hop's link text names its destination.
- **AC-UX-09** *(a11y floor, IQ-8)* — No state anywhere in this project's diff is
  carried by colour, emoji, glyph, strikethrough, ordering or an empty cell. This
  covers the promoted atoms, the reconciliation record, the DoD ledger and the DT
  audit table alike.
- **AC-UX-10** *(AC-014, AC-018)* — Every row of the DoD re-observation ledger and
  of the DT-1 … DT-10 audit is self-contained: scenario or tension id, the owning
  project, the observer, the tree ref or sha, and the outcome, with no row that
  depends on the row above to be understood.
- **AC-UX-11** *(AC-014, AC-015)* — Scenario 2 occupies two named halves in the
  ledger — the gate `failed by name` and the gate `recovered` — each with its own
  evidence. The string "inherited from a sibling" appears nowhere in an outcome
  column; its presence is a review-blocking defect.
- **AC-UX-12** *(AC-012, IQ-4)* — Every observation in the ledger and both
  post-merge re-checks name the tree they were performed against by ref or sha, so
  a later reader can re-run them; and where a promoted atom is later found wrong,
  the record states the route as *supersede*, never *edit*.

### Notes

**No Accepted ADR governs this surface, and that is a finding, not a gap.** All
sixteen accepted decision atoms (`0001`–`0016`, `0029`; `0002` is superseded per
`.kb/decisions/0002-crate-naming.md:5`) govern the Rust contract — async port
flavours, opaque payloads, the wire format, the MSRV. None binds persona content,
KB promotion or documentation structure; the charter says it outright
(`.bklg/docs-that-teach/initiative.md:524-525`). Do not go hunting for a
documentation ADR to cite: what binds this surface is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`, the single-write-path
rule (`/redkiln:kb-ingest` from `.kb/_intake/`, precedent `0269720`), and the
closeout-only `harvest_kb: true` stage (`.redkiln/processes/project.yaml:76-80`).
No invariant in this brief deviates from an Accepted ADR, because none of them
reaches this surface.

**"Design system" is being used honestly, not by analogy stretched thin.** The atom
template, the tier table and the two map formats are a real shared vocabulary with
a real validator behind it (`redkiln validate --kb`), which is what makes
"compose, do not hand-roll" a testable instruction here rather than a metaphor.
What this brief does *not* claim is a component library, tokens with values, or a
rendered theme — there is none, and inventing one would be exactly the bespoke
authoring the brief forbids.

**The clean-checkout requirement is an instruction to implementation, not a state
already satisfied.** AC-014's "clean checkout" must be a fresh clone or a
`git worktree add` off the **merged** branch. This planning pass runs inside
`.claude/worktrees/docs-that-teach`, mid-merge; nothing observed here counts as a
re-observation (`_grounding.md`, "Tensions and risks worth carrying").

**HS-S0131 does not exist in this tree.** Confirmed by directory listing, not only
by prose: `.bklg/` holds `docs-that-teach` and `support` and nothing else
(`_grounding.md`, "Reconciliation counterpart does not exist in this tree"). The
reconciliation record's default drafting case must therefore be the *unrun* one —
AC-UX-03 requires the merge-order case to be stated, and the case most likely to
hold is "this initiative's set stands as the authored one".

**The ingest README risk is live.** `.kb/_intake/` currently contains only
`README.md`, and the default glob is `.kb/_intake/*.md`
(`.kb/_intake/README.md:5-7`). IQ-4 puts the fix at the approval gate deliberately:
after the wave commits, removing a README that was ingested as an atom is a
supersession problem rather than a deletion.

**`redkiln validate --kb` was not exercised by this brief.** It needs a real ingest
wave to run against, which is implementation. AC-011 and AC-UX-02 are real bars and
neither has been checked here; the testing brief should own how they are run.

**Every invariant above is testable by reading, which is the point.** This surface
has no automated reader beyond `redkiln validate --kb`'s frontmatter conformance —
nothing checks whether a persona atom is *usable*. That is precisely why IQ-1
through IQ-8 are written as falsifiers a reviewer can apply in one pass, and why
`_design.md` for this project is not vacuous: with `design.capture` deliberately
absent from `.redkiln/config.yaml`, the written resolution is the only record there
will ever be ([`project.md`](project.md), DR-14).

## Architecture brief

### Intent

Land this initiative's audience as durable `.kb/product/` atoms **through the one write
path the corpus permits**, mounted into the indexes and item links that make an atom
reachable, and re-observe the whole initiative's Definition of Done on the merged tree.
This project ships **no Rust**: nothing under `crates/`, `examples/`, `spec/` or
`standards/` is in its diff. Its architecture is therefore not a module design but a
**pipeline and a set of mount points** — which writer produces `.kb/product/`, which
index each atom is registered in, which item field records it, and which command proves
the result. The failure this brief exists to prevent is a well-formed atom that is
reachable from nothing: it validates, and the next initiative never finds it, which is
the same outcome as never having promoted it.

### Acceptance Criteria

Architecture-grain, each traced to the project criterion it serves. These constrain
*how* the work is built; [`project.md`](project.md)'s AC-001 … AC-018 remain the
observable bar.

- **AC-A01 — One writer.** Every file this project adds under `.kb/` was written by a
  `/redkiln:kb-ingest` run consuming a document this project staged in `.kb/_intake/`.
  No `.kb/` atom appears in the diff without a corresponding staged source in the same
  wave's history. *(AC-008, DR-7; `.kb/_intake/README.md:3-5`, `:13-19`; precedent
  `0269720`.)*
- **AC-A02 — The wave is narrowed at invocation.** The ingest is invoked against an
  explicit file list or a glob that excludes `.kb/_intake/README.md`; the default
  `.kb/_intake/*.md` is not used unmodified. After the run that README is still present
  and unchanged, and no atom cites it in `source_paths`. *(AC-009, DR-8;
  `.kb/_intake/README.md:5`, `:29-31`.)*
- **AC-A03 — Atoms carry the product layer's shape, not the nearest neighbour's.**
  Persona atoms are `kind: concept`, journey atoms `kind: playbook`, **both
  `authority_tier: product`** — read off `.kb/product/README.md:6-9`, not copied from a
  sampled atom. *(AC-004, DR-3.)*
- **AC-A04 — Every promoted atom is mounted at all four points.** For each atom:
  (a) an entry in the appended `##` section of `.kb/maps/domain-map.md`; (b) reciprocal
  `related` / `depends_on` links to and from the atoms it sits beside; (c) its id in
  `links.kb` on HS-P0025, written by `redkiln record-links <id> --atom`; (d) a row in
  `closure.md`'s `## Knowledge Harvest` table. An atom missing any one is unmounted.
  *(AC-010, AC-011, DR-9; `.kb/maps/domain-map.md:144-150`,
  `.redkiln/templates/_retrospective.md:59`, `.redkiln/templates/closure.md`.)*
- **AC-A05 — Maps are appended, never edited.** `git diff` on `.kb/maps/domain-map.md`
  and `.kb/maps/open-questions-index.md` shows added lines only; no pre-existing `##`
  section is modified. *(AC-010, DR-9, DR-13; `domain-map.md:144-150`,
  `open-questions-index.md:136-143`.)*
- **AC-A06 — Nothing is observed against a pre-merge tree.** The merge-forward of
  `initiative/from-contract-to-published-library` precedes reconciliation, the clause-id
  re-check, the ingest wave and the DoD re-observation. Every one of those records names
  the merged commit sha. *(AC-012, AC-013, DR-2, DR-12;
  `.bklg/docs-that-teach/_decomposition.md:89-103`, `:116-123`.)*
- **AC-A07 — The re-observation runs somewhere this planning did not.** The fifteen DoD
  scenarios are run from a checkout created fresh off the **merged** branch (`git clone`
  or a new `git worktree add`), not from the worktree the planning and authoring happened
  in. The ledger names the checkout and its sha. *(AC-014, DR-11.)*
- **AC-A08 — Validation runs in the stated order, on the final tree.**
  `redkiln validate --kb` green, then `cargo xtask ci` green, both on the tree that
  already carries the atoms, the maps, the reconciliation record and the ledger — not on
  an intermediate state. *(AC-011, AC-016, DR-10, DR-11; `.redkiln/config.yaml:60`.)*
- **AC-A09 — No decision atom is authored or touched.** The diff contains no addition to
  `.kb/decisions/` and no modification of any file in it. Anything reconciliation
  surfaces that wants a decision is written down as a routed gap, in prose, in the
  reconciliation record. *([`project.md`](project.md) non-goals;
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md`.)*
- **AC-A10 — No sibling's content is edited by this project.** The `.kb/playbooks/`
  page-need discipline atom rides this wave as **payload**; its text is HS-P0021's. This
  project's diff may add it via ingest and must not rewrite it.
  *(`.bklg/docs-that-teach/_decomposition.md:106-114`.)*

### Notes

#### 1. The seam, stated as a diff surface

Per CLAUDE.md's repository map, this project touches exactly:

| Path | What happens to it | Writer |
| --- | --- | --- |
| `.kb/_intake/*.md` (new) | staged persona/journey documents, plus carried sibling payload | this project, by hand — staged material is **not** an atom (`.kb/_intake/README.md:7-11`) |
| `.kb/product/*.md` (new) | the promoted persona and journey atoms | `/redkiln:kb-ingest` only |
| `.kb/open-questions/*.md` (new, conditional) | any charter question that could not be answered | `/redkiln:kb-ingest` only |
| `.kb/maps/domain-map.md` | one appended `##` section | ingest's map-sync step |
| `.kb/maps/open-questions-index.md` | appended bullets, conditional on the above | ingest's map-sync step |
| `.bklg/docs-that-teach/durable-audience-closeout/*` | reconciliation record, DoD ledger, stage artefacts | this project |
| the whole tree | merge-forward of the sibling branch | `git merge` |

Everything else — `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` — is **read only**
here. `cargo xtask spec-trace` (DR-12) *runs over* `spec/SPECIFICATION.md`; it does not
edit it, and pinning a newly-discovered documentation MUST is HS-P0020's, not this
project's ([`project.md`](project.md), out of scope). If a change to any read-only path
becomes necessary, that is a routed gap rather than a scope extension — incidental bugs
go to the `support` initiative (`.redkiln/config.yaml:5`).

#### 2. The composition root, and the render path each atom must mount into

There is no application here, but there is an exact equivalent, and it is the reason
this brief exists. **The "running app" is the knowledge base as the next initiative
reads it**, and the next initiative reads it through the maps, the item links and the
closure — not by listing `.kb/product/`. A promoted atom that is not registered at each
of the points below is a component rendered into no tree.

**The composition root is `/redkiln:kb-ingest`.** It is the only writer of
`.kb/product/`, and it is reached only from `.kb/_intake/`. Staging is the *input*;
ingest is the *mount*. Two consequences implementers get wrong:

- `redkiln validate --kb` **skips every `_`-prefixed directory**
  (`.kb/_intake/README.md:21-27`). A staged file is therefore invisible to validation:
  a green `validate --kb` says nothing about whether `.kb/_intake/` was cleared, which
  is precisely why AC-009 is a separate, separately-observed criterion and not a
  by-product of AC-011.
- Ingest's own contract is that a successful run **clears the directory**
  (`.kb/_intake/README.md:13-19`), and the whole wave — new atoms *and* the removal of
  the staged sources — commits together. So `.kb/_intake/README.md` surviving the wave
  is not an accident to check afterwards; it is a filter applied at the invocation.

**The mount points, in the order the pipeline reaches them:**

1. **`.kb/maps/domain-map.md` — the index the corpus renders through.** Today two
   domains, both architectural; documentation/product has no section. Append one `##`
   section, group entries by kind, cite each atom's id next to its link, one orientation
   sentence per atom (`:144-150`). Editing an existing section to make room violates the
   map's own rule and is directly observable in `git diff` (AC-010).
2. **`.kb/maps/open-questions-index.md`** — only if DR-13 lands an `open_question` atom.
   Append the bullet under the domain section the question belongs to, **status word
   first** (`Open` / `Withdrawn` / `Superseded`), then the id, then one sentence; the
   grounding lives in the atom, never in the index (`:136-143`).
3. **Reciprocal links between atoms.** The exemplar map atom carries
   `related: [kb-map-domain-001, kb-map-decision-001]` in its own frontmatter
   (`.kb/maps/open-questions-index.md:15-17`): links are two-way in this corpus. A
   journey atom names its persona in `depends_on` / `related`, and the persona names its
   journeys back.
4. **`links.kb` on HS-P0025.** Written **only** by
   `redkiln record-links <id> --atom` (`.redkiln/templates/_retrospective.md:59`). The
   CLI is the single writer of item system frontmatter (CLAUDE.md); hand-editing
   `links: kb:` is denied by a `PreToolUse` hook and would in any case be the wrong
   record.
5. **`closure.md`'s `## Knowledge Harvest` table** (`.redkiln/templates/closure.md`) —
   the project artefact the `closeout` stage produces, and the *only* project stage
   carrying `harvest_kb: true` (`.redkiln/processes/project.yaml:76-80`). That is the
   mechanical reason promotion cannot happen at an earlier stage of this project, and is
   not a scheduling preference.
6. **The initiative's own closure.** `.redkiln/processes/initiative.yaml:32-36` and
   `:37-41` both carry `harvest_kb: true`; the charter's
   `## Referenced personas & journeys` currently records the layer as *"structurally
   present and functionally empty"* and cites the distillation instead
   (`.bklg/docs-that-teach/initiative.md:275-288`). Once the atoms exist that citation
   has a real referent — but the charter is the *initiative's* artefact, and redirecting
   it is `/redkiln:closeout`'s reconciliation step, which this project **feeds** with
   atom ids rather than performs.
7. **The gate.** `.redkiln/config.yaml:60` wires `e2e: "cargo xtask ci"` as the terminal
   grain, and this is the only project held to it
   (`.bklg/docs-that-teach/_decomposition.md:306-308`). It is a precondition for reading
   the DoD evidence, never a substitute for it.

**The integration tripwire the story map should carry:** for each promoted atom, name the
map line, the `related` edge, the `links.kb` entry and the `Knowledge Harvest` row that
carry it. Four references, or it is not mounted.

#### 3. What actually constrains this work — and the ADR that does not exist

Do not go looking for an Accepted decision atom that governs persona content or KB
promotion. There is not one. Sixteen decisions are `status: accepted` and one
(`.kb/decisions/0002-crate-naming.md:5`) is superseded; every one of them governs the
Rust contract — async port flavours, opaque payloads, error bounds, the wire format, the
MSRV. The charter says it outright: *"none of the seventeen decisions concerns
documentation"* (`.bklg/docs-that-teach/initiative.md:524-525`). Citing an ADR here would
be manufacturing authority, which is the same defect as manufacturing a persona.

What binds instead, in authority order:

1. **`.kb/governance/rewrite-the-referent-never-the-reasoning.md`** (`status: accepted`,
   `authority_tier: guideline`) — a correction to a standing decision is a **new atom
   that supersedes**, never an edit. This is what makes "no ADR as a side effect of
   promotion" load-bearing rather than stylistic, and it cuts both ways: it forbids
   editing a decision *and* forbids inventing one as closeout exhaust.
2. **The single-write-path rule** — CLAUDE.md, `.kb/_intake/README.md:3-5`, and the
   revert `0269720` that is its concrete precedent.
3. **`.redkiln/processes/project.yaml:76-80`** — `harvest_kb: true` on `closeout` only.
4. **`.redkiln/config.yaml:60`** — the terminal DoD command already exists; this project
   invents no new gate step.
5. **`spec/SPECIFICATION.md:280`** — *"Clause IDs are stable and are never renumbered."*
   That is why DR-12's risk is an **incomplete** pin, not a stale one: the sibling can
   only *add* documentation MUSTs, never renumber HS-P0020's set out from under it.

**Tensions the implementer must handle explicitly, not discover:**

- **T1 — authority-tier drift.** Every atom in the corpus sampled at grounding carries
  `authority_tier: note` or `guideline`. The product layer requires `product`
  (`.kb/product/README.md:6-9`). An author who imitates the nearest exemplar ships the
  wrong tier, and only a reviewer or `redkiln validate --kb` catches it. Read the layer
  README, not the neighbour.
- **T2 — "clean checkout" versus this worktree.** AC-014 is a literal instruction to the
  implementation stage: a fresh clone or `git worktree add` off the **merged** branch.
  The worktree this planning ran in does not satisfy it and cannot be made to.
- **T3 — the counterpart may never exist.** `HS-S0131` / `HS-P0019` /
  `from-contract-to-published-library` are **not present anywhere under `.bklg/` in this
  tree** — confirmed by directory listing, not inferred from prose
  ([`_grounding.md`](_grounding.md)). DR-2 / AC-002 require the reconciliation record to
  read coherently in the case where the sibling story never ran, stating plainly that
  this initiative's set stands as the authored one. That case must be the record's
  **default shape**, with the adjudication table degrading to "no counterpart staged"
  rows — not a paragraph bolted on if the merge turns out empty.
- **T4 — the glob risk is live, not theoretical.** `.kb/_intake/` today contains exactly
  `README.md`, and the default input is `.kb/_intake/*.md`. Handle it at the invocation
  (AC-A02); an unfiltered run reaches the approval gate with the README already treated
  as content.
- **T5 — validation is a tripwire on decision immutability.** `redkiln validate --kb`
  checks each accepted decision atom against `HEAD` (CLAUDE.md), so an accidental touch
  of `.kb/decisions/` fails the run. That is the desired behaviour; do not work around
  it.

#### 4. The contracts an atom must satisfy

**Frontmatter** (`KbFrontmatter`). The schema itself lives in the redkiln plugin, not in
this repo — there is no `src/` tree here, so validate by *running the command*, not by
reading a type. The shape both exemplars carry is: `id`, `title`, `kind`, `status`,
`authority_tier`, `summary` (a folded `>-` block carrying the whole argument, not a
label), `depends_on`, `related`, `source_paths`, `last_reviewed`. Model persona atoms on
`.kb/concepts/torn-reads-and-the-append-condition-boundary.md` and journey atoms on
`.kb/playbooks/one-decision-per-adr-title.md` — for **shape only**; the tier differs
(T1). `KbFrontmatter` is `.passthrough()` (`.kb/open-questions/README.md`), so an
imported corpus's own keys survive validation: do not strip one to make an atom conform,
and **do not invent tracking keys** on an atom you are authoring.

**Persona body** — goal, context, what they already do instead, and what they are afraid
of (`.kb/product/README.md:11-13`). Four things, all four present, per persona (AC-004).

**Journey body** — the moment-by-moment path through a task, written so you can tell
whether a build improved it (`:11-13`). Explicitly **not** a screen, a flow or an
interaction pattern (`:33-36`): a journey that transcribes today's surface goes stale
when the surface moves. The four journeys the charter names are at
`.bklg/docs-that-teach/initiative.md:289-301`.

**What must not travel into an atom** (`.kb/product/README.md:26-43`): an unevidenced
sketch presented without its caveat; requirements, scope or acceptance criteria (those
stay in `.bklg/`); a market segment. That last exclusion matters for the reconciliation
record specifically — the record is an *adjudication*, and the temptation is to promote
it wholesale as an atom. It is not durable knowledge about an audience; it is a project
artefact. Its **outcome** travels into the atoms' bodies and `source_paths`; the record
itself stays under `.bklg/docs-that-teach/durable-audience-closeout/`.

**`source_paths`** must resolve at closeout (AC-005) and must cite real discovery
evidence. The verified set in this tree is
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`,
`…/distillation/opportunities.md`, `…/distillation/interaction-patterns.md`,
`…/grounding/product-functional-alignment.md`, `…/grounding/backlog-adjacency.md`,
`…/grounding/vocabulary-and-conventions.md`, and the five `…/research/0*.md` digests.
Per `.kb/_intake/README.md:17-19` each atom also cites the `.kb/_intake/…` path that fed
it — which is why clearing the directory loses nothing: the staged source stays in git
history.

**The three qualifications** (AC-006, DR-4) are body text on the atoms, not a footnote on
the record: per-persona observation status; the adapter author's "no third-party adapter
exists to read" resting on the seed's internal audit alone
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:337-343`); and
the HS-S0131 overlap (`:344-359`). Per DR-5 / AC-007 the blanket *"none of these three
personas has been directly observed"* at `:319-327` is **corrected, not copied** — name
the persona HS-P0024's session actually walked, and mark the rest inferred. The
distillation's own fourth-persona note (`:328-336` — the friction-log reader is not
automatically identical to any of the three) is why that correction needs a stated
choice behind it rather than an assumption.

#### 5. Data flow, and the orderings that are load-bearing

```
  merge-forward ──► re-read counterpart ──► reconciliation record
        │                                          │
        └──────────────► sha recorded ◄────────────┘
                                 │
        stage .kb/_intake/*  ◄───┘
                 │
                 ▼
        /redkiln:kb-ingest   (narrowed glob — README excluded)
                 │
        ┌────────┼─────────────────────┬──────────────────┐
        ▼        ▼                     ▼                  ▼
  .kb/product/  .kb/open-questions/  maps appended   _intake cleared
                 │
                 ▼
        redkiln validate --kb
                 │
                 ▼
        record-links --atom  +  closure Knowledge Harvest
                 │
                 ▼
        clean checkout ──► 15 DoD re-observations ──► ledger committed
                 │
                 ▼
             cargo xtask ci
```

Four orderings are not negotiable:

1. **Merge before everything.** Reconciliation against this worktree's copy measures a
   stale tree (`.bklg/docs-that-teach/_decomposition.md:89-103`). The merge-forward is a
   *sequencing* obligation, not a blocking dependency — the gate resolved that explicitly
   (`:116-123`) — but every observation downstream of it is worthless without it.
2. **Ingest before validation.** `validate --kb` cannot see `.kb/_intake/`
   (`.kb/_intake/README.md:21-27`), so it can only judge what landed.
3. **Validation before the links.** `record-links --atom` should record ids that exist
   and validate; recording first and fixing afterwards leaves an item pointing at an atom
   that changed shape.
4. **Everything committed before the final gate run.** AC-016 requires `cargo xtask ci`
   green on *the exact tree carrying every artefact*. A gate run taken before the ledger
   and the reconciliation record are in the tree proves the gate, not the deliverable.

**Scenario 2 contains a mutation, and the mutation must not survive.** DoD-2
(`.bklg/docs-that-teach/initiative.md:416-420`) requires a page to be deliberately broken,
the gate to **fail by name**, and the edit then reverted to green; AC-015 requires both
halves recorded. Architecturally: the failing half's evidence is a captured transcript in
the ledger, and the revert is proven by the subsequent green run — the broken edit is
never a committed state of the tree the final `cargo xtask ci` (ordering 4) is taken on.

**DR-12's re-check is a read, not a write.** `cargo xtask spec-trace` is already a named
step inside the gate (`xtask/src/main.rs:315-326`; `"spec-trace"` at `:324`), so it runs
as part of AC-016 regardless. What AC-013 adds on top is a *written statement* comparing
the merged tree's documentation MUSTs against HS-P0020's pinned set, with any addition
either pinned by HS-P0020 or explicitly routed. Because clause ids are stable
(`spec/SPECIFICATION.md:280`), the comparison is a set difference over ids, never a diff
over line numbers.

#### 6. Deliberately not prescribed

These are the implementer's, and the story map / testing brief should record whichever way
they go rather than inheriting a guess from here:

- **How many staged intake documents.** One per persona, one per journey, or a single
  adjudicated document the wave splits — ingest merges and splits either way. What is
  fixed is that everything staged is narrowed at invocation (AC-A02) and cleared by the
  run.
- **Where the reconciliation record lives.** A companion under this project's folder, or
  the body of `closure.md`. Fixed: it is **not** a `.kb/` atom (§4), and it names the
  merged sha (AC-A06).
- **How the DoD ledger is shaped.** `.redkiln/templates/_ledger.md` exists and
  `require_ledger: true` (`.redkiln/config.yaml:62-67`) binds *stories* declaring
  `AC-###` — so the natural home for the fifteen re-observations is a story-level
  `_ledger.md` rather than an invented file. Confirm that against the story map before
  inventing a format. Fixed: per-scenario evidence naming who ran it and what was seen,
  and `"inherited from a sibling"` is not an admissible evidence value (AC-014).
- **Whether the evaluator resolves as its own persona atom or as a stage of the
  application author's journey.** Genuinely open
  (`.bklg/docs-that-teach/_decomposition.md:134-139`); AC-003 requires only that the atoms
  record the answer *once*, with reasoning, and that the charter's corresponding open
  question at `.bklg/docs-that-teach/initiative.md:538-540` is marked answered.
- **Whether the fourth-persona question becomes an `open_question` atom.** If it does it
  follows `.kb/open-questions/README.md`'s "what is true today / what is not decided /
  what forces it" shape with grounded `source_paths`, and gets its indexed bullet
  (AC-A05).
- **What the appended domain section is called.** `.kb/product/` is flat with a README
  (verified by listing) — do not invent sub-directories. The section's *name* is a
  judgement call; that it is appended rather than merged into an existing section is not.

## Testing brief

### Intent

Prove [`project.md`](project.md)'s eighteen acceptance criteria against the artefacts the
UX and architecture briefs describe, using the test mix the repository already runs rather
than inventing a parallel one. This project compiles no new Rust and mounts no page, so
"test" here means four different things depending on what is being checked: a machine
reading frontmatter and diffs, a human reading a claim against its four/three required
slots, a corpus-wide reachability sweep, and a from-scratch re-run of the whole initiative's
Definition of Done. The failure this brief exists to prevent is the one named twice already
in this document — "fifteen re-observations become fifteen ticked boxes" (`project.md`,
risk table) and "nothing checks whether a persona atom is usable" (this file, "## UX
brief", Notes) — by naming, for every AC, which of the four tiers actually catches a wrong
implementation of it, rather than assuming the gate being green says so.

### The test mix, and what each tier can and cannot see

| Tier | What runs it | What it can see | What it cannot see |
| --- | --- | --- | --- |
| **1 — Static / shape** | A machine, no compiler, no ingest run: `redkiln validate --kb`, `rg` over frontmatter, `git diff` over the two map files, `test -f` over `source_paths` | Frontmatter conformance (`kind`, `authority_tier`, required keys); accepted-decision immutability against `HEAD` (CLAUDE.md, "Where the work lives"); whether a diff line was an addition or a deletion; whether a cited path resolves | Whether an atom's *content* is true, complete, or usable; whether a link is reciprocal; whether a scenario actually ran |
| **2 — Content review** (the "unit" analogue) | A human, one claim at a time, against a named checklist | Whether a persona atom states all four required slots on first load; whether all three evidence qualifications survive; whether the reconciliation ledger has a row for every pair; whether the evaluator decision is stated once with reasoning | Anything the checklist does not enumerate — which is why the checklist is IQ-1…IQ-8 and AC-UX-01…AC-UX-12 above, not a fresh list invented here |
| **3 — Integration / mount-point** | A machine and a human together: `cargo xtask lints && cargo xtask spec-trace` (the `reachability_static` grain, `.redkiln/config.yaml:48`), plus a manual walk of the four mount points the architecture brief names | Whether an atom is *reachable* — from the domain map, from a reciprocal `related` edge, from `links.kb`, from the closure table — and whether the specification's cross-references still resolve corpus-wide | Whether the atom's content is correct in isolation (Tier 2's job); whether the fifteen DoD scenarios pass end to end |
| **4 — E2E / whole-tree** | A human running the initiative's own scenarios from a clean checkout, then `cargo xtask ci` | Whether the assembled tree — atoms, maps, links, ledger, reconciliation record, merged sibling — behaves as the initiative promised, including the deliberately-broken-page failure and recovery | Nothing it cannot see; it is the terminal grain and the repository's Definition of Done (`.redkiln/config.yaml:60`; CLAUDE.md, "Commands") |

Tier 2 needs its own justification because it is the tier a Rust-shaped testing brief would
be tempted to skip. The UX brief's own Notes section says why it cannot be: *"This surface
has no automated reader beyond `redkiln validate --kb`'s frontmatter conformance — nothing
checks whether a persona atom is *usable*"* (this file, "## UX brief"). A frontmatter-valid
persona atom whose "what they are afraid of" slot links out to the discovery corpus passes
Tier 1 and fails the thing the atom exists for. Tier 2 is therefore not optional colour on
top of the machine tiers — for roughly half of this project's ACs (AC-001, AC-002, AC-003,
AC-006, AC-007, AC-018) it is the *only* tier that can fail a plausible wrong
implementation, which is the bar CLAUDE.md's "rule that matters" corollary sets for any
check this repository keeps: *"A rule that no adapter can fail is decorative"* — the
document-corpus analogue is a checklist item nothing can fail, and Tier 2's checklist is
the falsifiers already written into the UX brief (IQ-1…IQ-8, AC-UX-01…AC-UX-12) rather than
a second list invented here, so that there is exactly one place a reviewer needs to have
open.

**A named wrong implementation Tier 2 exists to reject**, per the same corollary: a persona
atom that states "goal: use the library" and "context: an application" and links `see
_discovery/distillation/personas-and-journeys.md` for the remaining two slots. It carries
valid `kind`, `authority_tier` and every required key — Tier 1 passes it — and it fails
IQ-1 the moment a reviewer opens it alone.

### Merge-gate commands, in the order the architecture brief's data-flow fixes

Reusing "## Architecture brief", "Data flow, and the orderings that are load-bearing"
(orderings 1–4) rather than restating them: testing runs *after* each ordering point, never
instead of enforcing it.

1. `redkiln validate --kb` — DR-10, AC-011's literal bar. Zero-argument invocation walks
   every non-`_`-prefixed directory under `.kb/`; it cannot see `.kb/_intake/`
   (`.kb/_intake/README.md:21-27`), so a green result says nothing about AC-009 and must not
   be read as covering it.
2. `redkiln doctor` — paired with the above in CLAUDE.md's own command list ("`redkiln
   validate --kb && redkiln doctor` # the backlog and knowledge base check"). Expected to
   keep reporting the six `template-drift` advisories CLAUDE.md names as permanent; a
   *seventh* advisory or a missing one is this project's problem to flag, not silence,
   because "the backlog CI job asserts the set is **exactly** those six" (CLAUDE.md,
   "Where the work lives").
3. `cargo xtask lints && cargo xtask spec-trace` — the `reachability_static` grain
   (`.redkiln/config.yaml:48`). Runs corpus-wide, not diff-scoped, which is exactly why
   it is Tier 3: it is the only automated check that can catch a promoted atom nothing links
   to.
4. `redkiln record-links HS-P0025 --atom` — writes `links.kb`, the single-writer path for
   item frontmatter (CLAUDE.md; `.redkiln/templates/_retrospective.md:59`). Not a test by
   itself, but a precondition Tier 3's mount-point walk checks for.
5. `cargo xtask ci` — the terminal `e2e` grain (`.redkiln/config.yaml:60`), Tier 4, and the
   only command this project is held to that a sibling project is not
   (`.bklg/docs-that-teach/_decomposition.md:306-308`). Must run **last**, on the tree that
   already carries the atoms, the maps, `links.kb`, the reconciliation record and the DoD
   ledger — a run taken earlier "proves the gate, not the deliverable" (this file, "##
   Architecture brief", ordering 4).

`.redkiln/config.yaml`'s two other declared grains do not apply to this project directly:
`affected_gate` (`cargo xtask affected --base {{base}}`) is the *story*-grain check any
story this project's story map produces still runs; `integration_scoped` (`cargo xtask ci
--fast`) is what every **non**-terminal project is held to, and this is the one project the
decomposition exempted from it in favour of the whole gate. `require_ledger: true`
(`.redkiln/config.yaml:67`) and `require_commit_provenance: true`
(`.redkiln/config.yaml:73`) bind at story grain: any
story the story map produces that declares one of `project.md`'s AC-### must carry a
`_ledger.md` with cited evidence per row, distinct from the whole-project fifteen-scenario
DoD ledger this brief describes next — the former is per-story machine-checked evidence,
the latter is the terminal re-observation this project alone performs. `_storymap.md` is
still the unpopulated template (`.bklg/docs-that-teach/durable-audience-closeout/_storymap.md`)
at the time this brief is written, so which stories carry which AC-### is the story-map
stage's decision, not this brief's.

### Fixtures and seams — what this project isolates, since nothing here is mocked

There is no running application, so there is nothing to inject a test double into in the
usual sense. The seams worth naming are the boundaries a wrong implementation could blur:

- **Staged input vs. ingested output.** `.kb/_intake/*.md` is the one seam every atom must
  cross (DR-7, AC-A01). Test both sides independently: Tier 2 reads the *staged* documents
  before ingest (do the four/three slots exist in the source material at all?); Tier 1 reads
  the *ingested* atoms after (did the shape survive?). A pass on one side is not evidence
  about the other.
- **The clean-checkout seam.** AC-014's fifteen-scenario re-run and AC-A07 both require a
  tree this planning worktree cannot supply — a fresh `git clone` or `git worktree add` off
  the **merged** branch, never `.claude/worktrees/docs-that-teach` itself
  (`_grounding.md`, "Tensions and risks worth carrying"; T2 in "## Architecture brief").
  Tier 4 does not start until that checkout exists; treat "which checkout" as the fixture
  and record its path and sha in the ledger alongside every scenario's evidence.
- **Scenario 2's fault injection.** DoD-2 requires a page broken deliberately, the gate
  failing **by name**, then a revert back to green (AC-015). This is a live edit, not a
  mock, and per the architecture brief's data-flow note it must never become a committed
  state the final `cargo xtask ci` (merge-gate step 5) runs against: stage the break,
  capture the failing run's output verbatim as the ledger's evidence for the failing half,
  revert before committing anything, then re-run green as the second half's evidence.
  "Recovered" without a captured failing transcript is the exact gap AC-UX-11 already flags
  as a review-blocking defect in the record — the same discipline applies to the ledger.
- **The unrun counterpart.** HS-S0131 does not exist in this tree (`_grounding.md`,
  "Reconciliation counterpart does not exist in this tree"; confirmed again for this brief:
  no `HS-P0019`/`HS-S0131`/`from-contract-to-published-library` directory anywhere under
  `.bklg/`). DR-2/AC-002 require the record to read coherently under *either* case. Tier 2's
  check is therefore parametric rather than a single read: verify the record's stated case
  matches what `ls .bklg` shows at the time of reading, **and** that the prose would still
  parse sensibly under the other case — the nearest thing to a table-driven test this corpus
  has, run by a human rather than a harness because there is no harness that can evaluate
  prose coherence.
- **The ingest glob.** `.kb/_intake/*.md` today matches only `README.md`
  (`.kb/_intake/README.md:3-5`; confirmed by listing). The seam to isolate is the
  *invocation*, not a post-hoc filter: AC-A02 requires the wave be invoked against an
  explicit file list or a glob that excludes the README, checked by reading the ingest
  invocation itself before the wave runs, and re-checked by Tier 1 (`.kb/_intake/`
  contents and `source_paths` on every landed atom) afterward.

### Acceptance Criteria

Prefixed `AC-TB-` to keep them distinct from `project.md`'s AC-001…AC-018 and from the two
briefs above; each governs how testing itself is carried out.

- **AC-TB-01** — Every row of the mapping table below names at least one tier, and no tier
  is claimed for an AC it cannot actually falsify (Tier 1 is never cited alone for AC-001,
  AC-002, AC-003, AC-006, AC-007 or AC-018 — see the mapping table's tier column).
- **AC-TB-02** — Every command named in "Merge-gate commands" resolves to a real script in
  this tree (`.redkiln/config.yaml`, `xtask/src/main.rs`) or a documented CLI verb
  (CLAUDE.md, "Commands"). No invented tooling.
- **AC-TB-03** — The five merge-gate steps run in the stated order on the final tree; a
  `cargo xtask ci` run taken before the reconciliation record, the ledger and `links.kb` are
  all present in the tree does not count as AC-016's evidence.
- **AC-TB-04** — The fifteen-scenario DoD ledger records, per scenario: who ran it, the
  checkout path and tree sha, and what was seen. The string "inherited from a sibling"
  appears nowhere in it (AC-014; mirrors AC-UX-11 exactly, because it is the same defect
  looked at from the evidence side rather than the record side).
- **AC-TB-05** — Scenario 2's ledger entry carries the failing run's captured output, not
  only the fact that it failed, and the broken edit is never present in the tree at the
  moment merge-gate step 5 runs (AC-015).
- **AC-TB-06** — Tier 2 review is run against the UX brief's own checklist (IQ-1…IQ-8,
  AC-UX-01…AC-UX-12); this brief does not maintain a second, divergent checklist for the
  same atoms.
- **AC-TB-07** — `redkiln validate --kb` and `cargo xtask ci` both exit green, read in that
  order, on the tree that already carries every artefact `project.md`'s Definition of Done
  names (AC-011, AC-016).
- **AC-TB-08** — No claim in this brief implies new Rust test coverage exists or is required.
  `cargo xtask ci`'s Rust-grain steps (fmt, clippy, `cargo test --workspace --all-features`,
  the four wasm32 builds) still run as part of merge-gate step 5 because AC-016 binds the
  whole gate unconditionally — they function here as a regression net proving `crates/`,
  `examples/`, `spec/` and `standards/` stayed untouched (the architecture brief's "seam,
  stated as a diff surface" table), not as tests of new behaviour.

### AC-### → tier mapping

Every project acceptance criterion, mapped to the tier(s) that can actually catch a wrong
implementation of it. "Primary" is the tier without which the AC could pass on a defective
artefact; "also" tiers add coverage but are not sufficient alone.

| AC | Criterion (short) | Primary tier | Also | Where proven |
| --- | --- | --- | --- | --- |
| AC-001 | Complete reconciliation ledger, no pair unaddressed | 2 — content review | — | Read every row against `_discovery/distillation/personas-and-journeys.md` and the reconciliation record side by side; AC-UX-03 |
| AC-002 | Merge-order case stated, reads coherently under it | 2 — content review | 1 (`ls .bklg` cross-check) | The parametric read described under "The unrun counterpart" above |
| AC-003 | Evaluator question decided once, with reasoning | 2 — content review | 1 (`rg` for a second, contradicting statement) | One read of the promoted atoms plus the charter's open-question line (`initiative.md:538-540`) |
| AC-004 | Atom shape: `kind`, `authority_tier`, four persona slots | 1 — static (`rg "^(kind\|authority_tier):" .kb/product`) | 2 (slot content) | AC-UX-01, AC-UX-02 |
| AC-005 | `source_paths` cite real, resolving discovery evidence | 1 — static (`test -f` over every cited path) | — | AC-UX-05 |
| AC-006 | All three evidence qualifications present | 2 — content review | — | Checklist item in the same pass as AC-UX-04 |
| AC-007 | Correction named, not the blanket "none observed" | 2 — content review | 1 (`rg` for the forbidden blanket phrase) | AC-UX-04, DR-5 |
| AC-008 | Every `.kb/` atom came from an ingest run | 1 — static (diff/log: the atom's introducing commit is an ingest wave, not a hand-write) | 3 (no atom outside the wave's history) | AC-A01 |
| AC-009 | `.kb/_intake/` cleared, README survives unchanged | 1 — static (`ls .kb/_intake`, `git diff` on the README) | — | validate --kb cannot see this tier (`.kb/_intake/README.md:21-27`) — do not substitute it |
| AC-010 | Domain map: appended section only, no edits | 1 — static (`git diff` shows no `-` line outside the appended region) | — | AC-UX-06, AC-A05 |
| AC-011 | `redkiln validate --kb` exits zero | 1 — static (the command itself) | — | Merge-gate step 1 |
| AC-012 | Merged tree named by ref/sha in both records | 1 — static (`rg` for a sha string in each record) | 4 (the sha is real, i.e. it resolves) | AC-A06 |
| AC-013 | `spec-trace` passes post-merge; completeness statement written | 3 — integration (`cargo xtask spec-trace`, corpus-wide) | 2 (the written completeness statement itself) | Merge-gate step 3; DR-12 |
| AC-014 | All fifteen DoD scenarios re-observed, clean checkout | 4 — e2e | — | The fixtures section's clean-checkout seam; AC-A07 |
| AC-015 | Scenario 2: fail by name, then recover, both recorded | 4 — e2e | — | The fixtures section's fault-injection seam |
| AC-016 | `cargo xtask ci` green on the final tree | 4 — e2e | — | Merge-gate step 5 |
| AC-017 | Every charter open question answered or atomised | 2 — content review | 1 (`git diff` on the index is append-only) | AC-UX-07 |
| AC-018 | DT-1…DT-10 audited: resolved / deferred / gap | 2 — content review | — | Cross-project read of every sibling's `_design.md`; no automated reader exists for this table (DR-14) |

No AC in `project.md` maps to Tier 1 alone where its failure mode is about *content* rather
than *shape* — AC-001, AC-002, AC-003, AC-006, AC-007 and AC-018 all carry Tier 2 as
primary for exactly that reason, per AC-TB-01.

### Notes

**No Accepted ADR governs testing strategy for this surface either.** Same finding as the
two briefs above, for the same reason: the sixteen accepted decisions govern the Rust
contract, and this project's diff never touches `crates/`. Restated once rather than at
length — see "## Architecture brief", "#3. What actually constrains this work — and the
ADR that does not exist".

**`redkiln validate --kb`'s decision-immutability check is a free assertion this brief does
not need to write.** Because it checks every accepted decision atom against `HEAD`
(CLAUDE.md), an accidental touch to `.kb/decisions/` — which would violate AC-A09 and the
project's own non-goal against writing an ADR as a closeout by-product — fails merge-gate
step 1 structurally. Tier 1 already covers AC-A09 as a side effect; no dedicated check is
needed for it.

**The clean-checkout requirement is a testing precondition, not a testing step.** Tier 4
cannot start in `.claude/worktrees/docs-that-teach`; this is the same transfer condition
the UX brief states for WCAG AA ("## UX brief", "Accessibility floor") — true today only
once the trigger condition (here: a fresh checkout off the merged branch) is met.

**The story-map is empty at the time this brief is written.** `_storymap.md` still carries
its unpopulated template. This brief therefore binds tiers to `project.md`'s AC-### only,
not to story ids; whichever stories the story-map stage produces inherit their tier
assignments from the row of the mapping table above that carries their declared AC-###, and
each such story's `_ledger.md` (`require_ledger: true`) is where that evidence is actually
recorded — this brief names the tier, the story's ledger names the file:line.
