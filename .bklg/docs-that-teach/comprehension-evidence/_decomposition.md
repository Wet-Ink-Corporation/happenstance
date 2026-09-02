# Briefs — Comprehension Evidence (HS-P0024)

Companion artifact, not an item file. Holds this project's warranted briefs — `ux`
and `testing` per [`../_decomposition.md`](../_decomposition.md), `## Warranted
briefs per project` (no `architecture`, no `deployment`). Each brief follows
[`.redkiln/templates/briefs/brief.md`](../../../.redkiln/templates/briefs/brief.md):
Intent / Acceptance Criteria / Notes.

Not authoritative over [`../initiative.md`](../initiative.md),
[`../_decomposition.md`](../_decomposition.md) or
[`project.md`](project.md) — where a brief and those disagree, they win. Grounding
for every citation below is [`_grounding.md`](_grounding.md).

---

## UX brief

### Intent

Make the comprehension session — and the record it leaves — an experience that a
**non-author reader can complete without being coached**, and that a **stranger
six months later can audit without asking anyone a question**.

This project builds no screen and no public API. Its user-facing surfaces are
three, and all three are real artifacts a person reads:

1. **The material the reader walks** — the assembled documentation tree merged from
   HS-P0020 through HS-P0023: [`docs/README.md`](../../../docs/README.md), the
   rendered rustdoc of `crates/happenstance-core/` and `crates/happenstance/`, and
   [`examples/course-subscriptions/`](../../../examples/course-subscriptions/).
   This project does not author that surface; it *observes a person using it*, and
   the small content fixes it may make (`project.md`, `## In scope`) must compose
   from that surface's existing primitives rather than introduce new ones.
2. **The friction log** — a dated markdown artifact whose reader is a reviewer, a
   sibling-project owner, and eventually HS-P0025. Its legibility is the whole
   deliverable: research 04's finding is that a log with no destination and no
   auditable shape is a diary
   ([`../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md`](../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md)).
3. **`_design.md`** — the protocol, fixed before recruitment, and the sole durable
   record of DT-9. `design.capture` is absent from
   [`.redkiln/config.yaml`](../../../.redkiln/config.yaml), so the perceptual
   review is a skip and the written resolution is the whole record (`CLAUDE.md`;
   [`_grounding.md`](_grounding.md), `## Confirmed mechanics`).

**No Accepted decision atom under `.kb/decisions/` governs documentation, personas
or comprehension methodology.** All seventeen concern port flavours, crate naming,
opaque payloads, MSRV, error traits, append conditions, position assignment, event
identity, validated identifiers and the wire format — the initiative charter states
this and a listing confirms it ([`_grounding.md`](_grounding.md), `## Headline
finding`). This brief therefore cites **no UX decision atom for its core subject,
because none exists**, and states that rather than manufacturing a citation. The
one atom that binds, and only at the disposition seam, is
[`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md).

#### Who the three users are, and what each is trying to do

Framed as intent, not mechanism. Persona detail is
[`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md);
it is **not** promoted to `.kb/product/` and this project must not promote it —
[`.kb/product/README.md`](../../../.kb/product/README.md) holds an unevidenced
sketch in `_discovery/` until closeout, which is HS-P0025's job.

| User | What they are trying to do | What must never happen to them |
| --- | --- | --- |
| **U1 — the recruited reader** (whichever of Persona 1 / 2 / 3 DT-9 selects) | Get *their own* job done against the library — model a cross-entity boundary, implement the storage contract, or decide inside twenty minutes whether to depend on this. They are not evaluating documentation; they are using it. | Being told the answer. Being asked to grade a page. Being made to feel the session is a test of them. Each of these converts evidence into agreement. |
| **U2 — the facilitator/logger** | Capture what actually happened — searches typed, links followed, files opened, commands run — while it is happening, without steering it. | Discovering after the session that the shape they needed (scenario, context, severity scale) was never fixed, and reconstructing it from memory. Research 04 calls retrospective reconstruction a measurably different instrument, not a late version of the same one. |
| **U3 — the downstream actor** (a sibling project owner, the `support` initiative, HS-P0025) | Open the log, find the items that are theirs, and act — without reading the whole thing and without asking the logger what an entry meant. | Reaching an item whose destination is a description rather than an id, or whose disposition is implied by silence. |

#### The states this surface has to express

Enumerated so they are designed in, not discovered as a missing column.

- **Protocol fixed, nobody recruited.** `_design.md` carries scenario, narration
  mode, severity scale and disqualifying criteria, at a commit that *precedes* the
  session date (AC-002). This state must be externally checkable — a reviewer reads
  a commit date, not an assurance.
- **Candidate declared, eligibility resolved.** Eligible / ineligible / ineligible-
  but-used-anyway. The third is a real state and must be representable: if it
  occurs, the artefact is unmet and the log says so (`project.md`, risk table, row 1).
- **Session in progress.** Append-only. Entries are added; nothing already written
  is edited.
- **Stumble recorded, undispositioned.** Legal *during* the session, illegal at
  hand-off. AC-006 is "exactly one disposition" — zero and two are both failures.
- **Dispositioned:** fixed / deliberately accepted with a reason / routed with a
  destination id. Exactly one, and the reason or id is present, not implied.
- **Escalated.** A distinct state from routed: a disposition that would reopen a
  resolved design tension carries the **DT id** and does not become a fix here
  (AC-011).
- **Second-session question answered.** Run / declined-with-reason. "Not mentioned"
  is not one of the states (AC-010).
- **Handed off.** One persona named as directly observed, with date and tree
  (AC-009). Absent this state, HS-P0025 cannot replace the blanket "none has been
  directly observed" qualification honestly.
- **No reader found.** A terminal failure state of the project, not a prompt to
  redefine the bar.

#### The primitive layer to compose from — do not hand-roll

The medium here is markdown and rustdoc, not a web app: there is **no CSS layer and
no token file, and inventing one is out of scope**. The equivalent obligation still
holds — compose from the repository's existing artifact primitives rather than
inventing a bespoke format — and the primitives are real files:

**For the log, the protocol and the hand-off note:**

- [`.redkiln/templates/briefs/brief.md`](../../../.redkiln/templates/briefs/brief.md)
  — the Intent / Acceptance Criteria / Notes spine.
- [`.redkiln/templates/_design.md`](../../../.redkiln/templates/_design.md) — its
  `## Shape decision` table (`| Item | Chosen shape | Rejected (and why) | Evidence
  | Resolves |`) is the existing primitive for recording a decision *with its
  rejected alternatives*; DT-9's resolution composes into that shape. Its
  `## Sign-off` section is the existing primitive for the human gate.
- The risk-table primitive already used by every project in this initiative:
  `| Risk | Likelihood / Impact | Mitigation |` ([`project.md`](project.md),
  `## Risks and coupling notes`).
- The routing-table primitive: two columns, "what you are looking for" against
  "where it is" — [`docs/README.md`](../../../docs/README.md) already uses it, and
  it is the shape a disposition index should reuse rather than replace.
- The **one-line checkbox** primitive from
  [`.redkiln/templates/gates/`](../../../.redkiln/templates/gates/). `CLAUDE.md` is
  explicit that the parser matches line by line and a wrapped box can never match.
  Any checklist this project writes keeps each box on one line. This is the closest
  thing in this repository to a hard token constraint, and it is load-bearing.

**For any content fix the session's dispositions produce:**

- [`standards/rust/70-rustdoc-obligations.md`](../../../standards/rust/70-rustdoc-obligations.md)
  — RS-70-2 (never write an intra-doc link that resolves in only some feature
  configurations), RS-70-3 (`doc-valid-idents`, never `allow(clippy::doc_markdown)`),
  RS-70-4 (`docsrs` + `doc(cfg)`), RS-70-5 (name the alternative that lost, once).
  The `# Errors` / `# Panics` section headings in that atom are the doc-comment
  primitives; a fix adds to them rather than inventing a new heading vocabulary.
- [`standards/rust/62-doctests-and-harnesses.md`](../../../standards/rust/62-doctests-and-harnesses.md)
  — the compiled-example primitive. A fix that adds an `ignore`-fenced snippet
  reproduces the anti-pattern the initiative exists to remove.
- The ecosystem's own interaction primitives — rustdoc search, `#[doc(alias)]`,
  intra-doc links, `[+]`/`[-]` collapse — catalogued in
  [`../_discovery/distillation/interaction-patterns.md`](../_discovery/distillation/interaction-patterns.md).
  Its anti-pattern list names the failure directly: **a bespoke navigation widget
  added on top of a medium that already renders the equivalent for free**. If the
  session finds the evaluator's "nowhere for the second question to go," the fix is
  almost certainly a *missing link*, not a missing component.

#### The accessibility floor

WCAG 2.2 AA is the bar, translated into what this medium can actually violate.

- **Colour is never the only carrier.** Research 04's stoplight convention is
  red/yellow/green *by name*; a markdown log rendered on a diff view has no colour
  at all. Every severity mark is therefore a **text token** — the word, or a
  number on the named scale (Nielsen 0–4 is one candidate; the choice is
  `_design.md`'s, per AC-002) — and never an emoji or colour swatch carrying the
  meaning alone. A screen reader and a `git diff` must both read the severity.
- **Static text is the evidence of record; motion is never required.** A screen or
  audio recording may exist as a supplement, but AC-004's six elements must be
  tickable against the markdown alone. No reader of this evidence is ever obliged
  to watch a recording, and nothing in the artefact autoplays, animates or depends
  on transition to convey meaning. This is the reduced-motion floor stated for a
  medium that has no animation to reduce: the obligation is that the text is
  complete, not that an animation is polite.
- **Keyboard reachability, both sides.** The session must be completable using only
  what the ecosystem's own medium provides — rustdoc's keyboard search, browser
  find, intra-doc links. If the reader could not reach something without a pointer,
  that is a finding, logged, not a session defect to work around. And the log
  itself is navigable by browser find and stable heading anchors; it must not
  require a widget, a script or a rendering tool to be readable.
- **The reader's own context is part of the record.** AC-004 already requires
  logger context; that includes platform, toolchain, browser and any assistive
  technology in use, because a stumble is only reproducible against the context
  that produced it.
- **A diagram, if a disposition adds one, carries a text equivalent.** The
  numbered-list / mapping-table narration device is independently evidenced in
  `interaction-patterns.md` as a working substitute for a diagram, which makes the
  text equivalent a first-class alternative here rather than a grudging alt
  attribute.

#### Interaction-quality invariants

These are testable requirements, not aspirations. Each is checkable by a reviewer
against a named artifact.

- **IQ-1 — In place, not a context jump.** A stumble's disposition is readable at
  the stumble. An index or summary table may exist *in addition*, and a routed item
  may point outward by id, but a reviewer must never have to leave the entry to
  learn whether it was resolved. One hop maximum, and the hop is optional.
- **IQ-2 — Non-occlusion: a filter must not hide what it filters.** A severity
  roll-up, a "blocking items only" view or a per-owner extract is additive. The
  chronological record stays complete and visible-by-default, and no item appears
  *only* in a filtered view. This is the same rule
  `interaction-patterns.md` states as an anti-pattern for the documentation surface
  — a load-bearing item behind a fold, an inactive tab or a collapsed admonition,
  with no visible-by-default counterpart — applied to this project's own artifact.
  Checkable exactly as that dossier states it: **if the collapsed or filtered view
  were deleted, would the record still carry every stumble and its disposition?**
- **IQ-3 — Preserved position: ids and order are stable once written.** Stumbles
  carry stable ids assigned in the order they occurred, and the chronological
  section is append-only during the session. Renumbering after the fact, reordering
  by severity in place of chronology, or re-heading a section breaks every citation
  a sibling project has already made into the log. A later reader landing on a
  cited anchor must find the item that was cited.
- **IQ-4 — Reversibility.** Every disposition is revisitable. A changed disposition
  is recorded *as a change*, with the earlier one still legible and the reason
  attached — never an in-place overwrite. This is
  [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
  applied to dispositions: the referent may be rewritten, the reasoning may not be
  erased. It is also what makes AC-011's escalation safe — an escalation that later
  proves unnecessary can be withdrawn without the trail vanishing.
- **IQ-5 — Non-interference during the session.** The facilitator does not answer
  the reader's question, does not point at the file, and does not narrate on the
  reader's behalf. Research 04 treats non-authorship as the *mechanism* — insiders
  unconsciously route around rough spots — and a facilitator who rescues the reader
  reintroduces exactly that. If the session must be unblocked to continue, the
  intervention is itself logged as an entry with its timestamp.
- **IQ-6 — Abandonment is a valid end state.** The reader may stop at any point and
  the partial log is still evidence. Nielsen & Landauer's basis for the single
  session is that each problem a real reader hits is already proven; a session that
  ends at the first blocker has produced a finding, not a void. The log records
  where and why it stopped rather than being discarded or re-run silently.
- **IQ-7 — Destinations are ids, not descriptions.** "Route to the docs team" is
  not a destination. `HS-P0022`, `HS-P0023`, the `support` initiative
  (`.redkiln/config.yaml`, `support_initiative: support`, line 5) or a named
  deferral are. A routed item without an id fails AC-006 as surely as an
  undispositioned one.

### Acceptance Criteria

Project-grain, checkable by a reviewer reading a named artifact. These refine
`project.md`'s AC-001…AC-011 along the UX axis; they do not replace them, and where
they appear to disagree, `project.md` and `../initiative.md` win.

- **UX-AC-001 — The persona choice reads as an intent, not a label.** `_design.md`'s
  DT-9 resolution states which persona the session walks **in terms of what that
  reader is trying to accomplish** (model a cross-entity boundary / implement the
  contract / decide within twenty minutes), names the comprehension technique that
  fits that intent from the published paraphrase / plus-minus / task-based taxonomy,
  and states in the same section that the other two personas' evidence remains
  inferred. Composed into the `## Shape decision` table shape from
  `.redkiln/templates/_design.md`, with the rejected two personas and why each lost.
  (Refines AC-001.)
- **UX-AC-002 — The scenario is stated in the reader's own words and is one to two
  sentences.** It names the reader's goal, not the documentation's structure. A
  reviewer can read it without knowing this repository exists. Research 04 calls the
  scenario the most crucial part and names both failure directions — too narrow
  proves nothing, too esoteric is dismissed as an edge case. (Refines AC-002.)
- **UX-AC-003 — Every severity mark is a text token on a named scale.** No mark
  carries its meaning by colour or emoji alone; the scale is named in `_design.md`
  before the session and applied inline as the log is written, never retrofitted.
  Checkable by reading the log as plain text with all styling stripped. (Refines
  AC-002/AC-004; the accessibility floor above.)
- **UX-AC-004 — The log is complete as static text.** All six elements of AC-004 —
  scenario; logger identity, context and date; chronological record including search
  terms and links followed; reactions as they occurred; inline severity — are
  tickable against the markdown alone, with no recording, tool or rendering step
  required. Logger context includes platform, toolchain and any assistive technology.
- **UX-AC-005 — Disposition is readable at the stumble (IQ-1).** For every
  severity-marked item, a reviewer reading the entry can see its disposition without
  navigating away. Any summary or per-owner index is additive.
- **UX-AC-006 — No stumble exists only inside a filtered or collapsed view (IQ-2).**
  Deleting every roll-up, fold and extract from the log leaves the full set of
  stumbles and dispositions intact. Checkable by deletion.
- **UX-AC-007 — Stumble ids and chronological order are stable (IQ-3).** Ids are
  assigned in occurrence order and are never reassigned; the chronological section is
  append-only during the session. A citation made by a sibling project into a
  stumble id still resolves to the same item after the log is finalised.
- **UX-AC-008 — A revised disposition is recorded as a revision (IQ-4).** Where any
  disposition changed after first being written, both the earlier disposition and the
  reason for the change are legible. Nothing is silently overwritten.
- **UX-AC-009 — Facilitator interventions are logged, not invisible (IQ-5).** Any
  moment the facilitator answered, pointed or unblocked is an entry with its
  timestamp. A session log containing zero interventions asserts that none occurred.
- **UX-AC-010 — The session was completable keyboard-only, or the failure to be is a
  finding.** The log records whether the reader reached what they needed using the
  medium's own affordances (rustdoc search, browser find, intra-doc links). A
  pointer-only reach is logged as a stumble, not smoothed over.
- **UX-AC-011 — Every routed item names an id (IQ-7).** `HS-P0022`, `HS-P0023`, the
  `support` initiative per `.redkiln/config.yaml`, or a named deferral staged for
  later ingest. A prose destination fails. (Refines AC-006/AC-007.)
- **UX-AC-012 — Any content fix composes from the existing primitive layer.** A fix
  arising from a disposition uses `standards/rust/70-rustdoc-obligations.md`'s
  doc-comment sections and intra-doc links, `docs/README.md`'s routing-table shape,
  or a compiled example per `standards/rust/62-doctests-and-harnesses.md`. It
  introduces no new navigation widget, no new page-structure convention, and no
  `ignore`-fenced snippet. Where the fix looks like it needs a new convention, that
  is a routed item for HS-P0021 `page-need-discipline`, not an invention here.
- **UX-AC-013 — The hand-off note is legible to someone who has not read the log.**
  It states one directly-observed persona, the date, the tree walked, and the scope
  sentence ("real stumbles were captured and are traceable"; never exhaustiveness) in
  a form HS-P0025 can lift without re-deriving anything. (Refines AC-008/AC-009.)

### Notes

**Deviation from an Accepted ADR: none, because none applies.** Stated explicitly
per the grounding pass rather than left as an absence a reviewer has to verify. The
only KB atom binding on this project's work is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`, and it binds at two
seams: IQ-4 above, and any disposition that rewrites a `happenstance-core` doc
comment already discharging a specification clause.

**Escalation has a shape already on record — cite it, do not invent one.** Two
sibling projects name HS-P0024 reciprocally as their disposition destination, and
their wording is the expected shape for AC-011/IQ-7:

- HS-P0023's risk table: *"AC-008 may turn out to be insufficient… If the friction
  log shows the surfaced version does not carry an adapter author, a new account is
  owed and is dispositioned through HS-P0024 — not absorbed here."* Its risk table
  separately concedes its DoD-9 evaluator second-questions are author-chosen and
  that *"HS-P0024 is the instrument that can falsify the choice."*
- HS-P0022's `## Out of scope`: *"If the log shows this project's bridge does not
  land, the disposition routes through HS-P0024, not back into this charter
  silently."*

The symmetry matters: this project routes findings *out* in exactly the shape those
two projects declared they would accept them.

**Quizzes and inline comprehension checks are settled out, not a design option.**
`interaction-patterns.md`'s anti-pattern list rules them out on evidence — they
measure recall of the author's own prose and inherit the author's blind spots, which
is the bias the non-author requirement exists to route around. Do not reopen this as
a lighter-weight alternative to the session.

**The `support` initiative is real but structurally empty.** `.redkiln/config.yaml`
line 5 sets `support_initiative: support` and `.bklg/support/initiative.md` exists
(`HS-I0005`), but it is an unfilled template at `stage: intake` with zero projects.
Routing a library bug there is correct; describing it as an active backlog with
stories to slot into is not.

**A deferral is staged material, not a file this project writes.** The third AC-007
destination — "a recorded open question" — is `.kb/open-questions/`-shaped, and
`CLAUDE.md`'s binding rule is that `.kb/` atoms are authored only through
`/redkiln:kb-ingest` or closeout; the first attempt at hand-authoring was reverted
(`0269720`). A deferral disposition therefore stages the question for ingest or for
HS-P0025's promotion pass, with the id of that hand-off recorded. This project edits
nothing under `.kb/`.

**`examples/outside-projection-adapter/` is not a citable anchor here.** It appears
in this initiative's discovery corpus (Opportunity 5) as precedent for a
non-author-built artefact, and `../initiative.md` itself flags it as *"asserted and
unverifiable from this worktree."* A listing of `examples/` does not surface it. If
the precedent is wanted, cite the `opportunities.md` paragraph, not the path.

**DT-9 is deliberately unresolved here.** The three personas call for genuinely
different comprehension checks — paraphrase for Persona 1's mental model, a
findability check for Persona 2's `E0034` gap in
[`crates/happenstance-core/src/store.rs`](../../../crates/happenstance-core/src/store.rs),
plus-minus for Persona 3's twenty-minute trust decision. This brief supplies the
intent framing and the invariants each would be held to; the choice belongs to
`_design.md` and its human sign-off gate.

**Contract grain, deliberately.** Nothing above prescribes a file layout for the
log, a heading vocabulary, or which severity scale wins. Those are `_design.md`'s,
and the brief would be overreaching to pre-empt the one stage this repository
reserves for a human to disagree cheaply.

---

## Testing brief

### Intent

Say, honestly, what "tested" means for a project whose proof artifacts are a
dated markdown log, a `_design.md` sign-off and a hand-off note rather than a
crate. `../_decomposition.md`'s warranted-briefs table marks this project
`testing: yes` (`## Warranted briefs per project`) but — unlike HS-P0020 and
HS-P0022, whose rows it explains in the paragraph immediately below the table —
gives no stated rationale for HS-P0024's own `yes`. This brief supplies that
rationale rather than leaving it implied: HS-P0024 earns `testing` for two
reasons the ACs below make concrete, not because it produces library code.

1. **Several of this project's own ACs are provenance claims, not opinions**, and
   a provenance claim is mechanically checkable even when the thing it is about
   is qualitative. "This commit precedes that date" (AC-002), "this destination
   id resolves to a real path" (AC-007), "this DT id exists in the ownership
   table" (AC-011) are `git`/`test -f`/`rg` facts, not reviewer judgment calls —
   distinguishing them from AC-003/004/008/009/010, which stay genuinely
   reviewer-read, is the point of the tier split below.
2. **The one code-shaped obligation this project carries is real**: "small
   content fixes arising from dispositions" is an explicit in-scope bullet
   (`project.md`, `## In scope`), and DoD-7 binds this non-terminal project to
   `cargo xtask ci --fast` (`.redkiln/config.yaml:55`,
   `integration_scoped: "cargo xtask ci --fast"`) the same as every code-touching
   project in this initiative. A content fix that skips that gate is not
   "documentation, so untested" — it is untested.

What this project does **not** own: HS-P0023 took no `testing` brief because its
DoD items "are all observed walks rather than automated checks"
(`../_decomposition.md`, `## Warranted briefs per project`). This project's
central instrument — a real non-author reader walking the real tree — is the
same kind of observed walk, and stays one. Nothing below converts the session
itself into an automated test; IQ-5's non-interference rule
(`_decomposition.md` §UX brief above) and research 04's non-authorship-as-
mechanism finding are exactly why a scripted or simulated "reader" would not be
evidence of anything. The tiers below cover the artifacts the session
*produces*, and the code the session's findings may touch — never the session.

### Acceptance Criteria

Every project `AC-###` (`project.md`, `## Acceptance criteria`) mapped to the
tier(s) that actually check it, and the real command or mechanism each tier
runs. "Artifact-evidence" is not a euphemism for "someone eyeballs it": it is
the ledger discipline `.redkiln/config.yaml`'s `require_ledger: true` already
imposes on every story under this project — each story's `_ledger.md`
(`.redkiln/templates/_ledger.md`) cites `evidence` as a real `file:line` and, if
one exists, a `verifying_test` id per criterion, and `redkiln verify --grain
story` blocks `implement → report` until every row satisfies that (`
.redkiln/templates/_ledger.md`, lines 10–17). For this project, most `evidence`
values will be `file:line` citations into the log, `_design.md` or the hand-off
note themselves, because those *are* the proof artifacts — not a gap in
rigor, the shape rigor takes when the deliverable is a record rather than a
function.

| AC | Tier(s) | Mechanism | What it actually proves |
| --- | --- | --- | --- |
| AC-001 — persona decided, not defaulted | Artifact-evidence | Ledger cites `_design.md`'s `## Shape decision` table row (`.redkiln/templates/_design.md` shape) naming the chosen persona, technique and the two rejected personas with reasons | The choice exists and is reasoned, not silent |
| AC-002 — protocol fixed before recruitment | Static (provenance) + Artifact-evidence | `git log --format=%aI -- <path to _design.md>` predates the session date recorded in the log — a real date comparison, not an assurance | The protocol could not have been written to fit what the reader did |
| AC-003 — logger verifiably non-author/non-insider | Artifact-evidence | Ledger cites the log's declaration section against the disqualifying criteria `_design.md` fixed under AC-002 | The declaration is checkable without asking the reader anything further, per the AC's own text |
| AC-004 — dated log, six elements present | Artifact-evidence, Static for structure | Heading/section presence is `rg`-checkable (scenario, logger identity/context/date, chronological record, reactions, inline severity); content adequacy is ledger-cited, reviewer-read | All six tick against the file alone, no recording required (UX brief, accessibility floor) |
| AC-005 — log names the tree it walked | Static (provenance) | The cited commit/merge point resolves via `git show`/`git log` from this worktree — the same discipline `_grounding.md` used to *fail* `examples/outside-projection-adapter/` as unreachable, applied here to confirm rather than refute a citation | A later reader can tell whether a stumble still applies |
| AC-006 — every stumble exactly one disposition | Artifact-evidence | Ledger-cited; reviewer reads the log end to end and counts. See Notes — this is the AC most tempting to over-automate | Zero undispositioned, zero double-dispositioned items |
| AC-007 — routing lands correctly | Static (existence) + Artifact-evidence | `test -f .bklg/support/initiative.md`, `test -f .bklg/docs-that-teach/application-author-path/project.md`, `test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md` confirm each destination id resolves to a real item; which id is the *correct* one per item stays reviewer-read | A routed item's destination is real and reachable, not a description |
| AC-008 — claim scoped in writing | Static (grep) + Artifact-evidence | `rg` for the required scope sentence in the log and its summary; `rg -i "exhaustiv"` returning nothing outside a disclaiming sentence | The claim never overreaches past what one session supports (BR-14) |
| AC-009 — hand-off to closeout explicit | Artifact-evidence | Ledger cites the hand-off note's named section (one persona, date, tree) | HS-P0025 can lift the qualification without re-deriving it |
| AC-010 — second session is a decision | Artifact-evidence | Ledger cites the assessment and its verdict (run / declined-with-reason) | Silence never stands in for a decision |
| AC-011 — no settled decision reopened in passing | Static (id existence) + Artifact-evidence | Any escalation's DT id is checked against `../_decomposition.md`'s `## Design tension ownership` table (`rg "^\| DT-" ../_decomposition.md`); the escalation's substance stays reviewer-read | An "escalation" citing a DT id that does not exist is caught mechanically, not trusted |

**Content-fix tier, orthogonal to the table above.** Where a disposition
produces a small content fix (rustdoc comment, `docs/README.md` prose), it is
proven by the ordinary code tiers this repository already runs, not a new one:

- **Doctest tier** (stands in for "unit" — this project has no functions):
  `cargo test --workspace --all-features` compiles every doc-comment example the
  fix touches. Governed by
  [`standards/rust/62-doctests-and-harnesses.md`](../../../standards/rust/62-doctests-and-harnesses.md)
  — RS-62-1 through RS-62-5 — and
  [`standards/rust/70-rustdoc-obligations.md`](../../../standards/rust/70-rustdoc-obligations.md)'s
  RS-70-2/70-3/70-5, already cited in the UX brief's primitive layer.
- **Static tier**: `cargo fmt`, `cargo clippy -D warnings`, and the two
  cheap-tripwire checks `.redkiln/config.yaml`'s `reachability_static` wires —
  `cargo xtask lints && cargo xtask spec-trace` — catch a fix that breaks a
  `SPECIFICATION.md` clause citation or a doc-comment lint before the full gate
  runs.
- **Integration tier — this project's actual merge gate**: `cargo xtask ci
  --fast`, `.redkiln/config.yaml:55`'s `integration_scoped`, matching DoD-7
  verbatim ("the non-terminal integration bar is green... this project is
  `terminal: false`, so the whole-gate `e2e` bar is HS-P0025's, not this one's").
  Runs fmt, clippy `-D warnings`, tests, the four wasm32 steps, docs,
  `spec-trace`, the `--no-default-features` doc build and the package-list
  assertion (`CLAUDE.md`, `## Commands`). Any content fix must clear this before
  the story it belongs to can advance.
- **This project's own backlog hygiene**: `redkiln validate --kb && redkiln
  doctor` clean, with no new `template-drift` beyond the six standing advisories
  CLAUDE.md documents (`project.md`, DoD-8).
- **E2E tier — explicitly not owned here.** `.redkiln/config.yaml:60`'s `e2e:
  "cargo xtask ci"` is the terminal bar, and DoD-7 states plainly it is
  "HS-P0025's, not this one's." This project's nearest analogue to an
  end-to-end proof is the friction-log session itself — a real reader against
  the real assembled tree — which is a research instrument, not a CI step, and
  the initiative's own framing is explicit that the two "falsify different
  things and neither may stand in for the other" (`project.md`, `## How this
  advances the initiative`, quoting `../initiative.md`, `## Risks`, first row).
  Do not fold the session into this project's own merge gate as if a passing
  `cargo xtask ci --fast` said anything about comprehension.

### Notes

**`cargo xtask affected --base <base>`** is the story-grain check
(`.redkiln/config.yaml`'s `affected_gate`) each of this project's stories runs
before the integration-grain checks above: it maps the diff to workspace
packages plus their dependents and runs fmt/clippy/tests for that set, or —
because a story here more often edits `SPECIFICATION.md`, a KB atom or nothing
under `crates/` at all — falls through to the five file-reading lints and
`spec-trace` unconditionally, so a story whose whole deliverable is the friction
log still gets a real check rather than a vacuous pass on an empty package set
(`.redkiln/config.yaml`, `affected_gate` comment block).

**Do not automate AC-006 or AC-009 into a check that nothing can fail.**
CLAUDE.md's own corollary — "a rule that no adapter can fail is decorative;
before adding one, name a plausible wrong implementation it rejects" — applies
here one level up. A script that merely confirms a "Disposition:" line exists
under every stumble heading would pass a disposition that says nothing
("noted") as readily as one that says "routed to HS-P0022, see stumble #4" —
it does not reject the wrong implementation research 04 is worried about. If
this project's own stories want to lighten AC-006/AC-009's reviewer burden, the
check has to assert on content (a destination id pattern, an intervention
timestamp), not merely on structural presence, or it belongs in the
Artifact-evidence tier and not in a new automated one.

**Fixtures and seams — what stays real, what may stand in.**

- **U1, the recruited reader, and U2, the facilitator, are never mocked, scripted
  or simulated.** This is not a testing-convenience gap to close later; it is
  the mechanism. Research 04's finding is that *insiders* — not authors
  specifically — unconsciously route around rough spots, which is why AC-003's
  disqualifying criteria and IQ-5's non-interference rule exist. A stand-in
  reader is not a cheaper version of this instrument; it is a different,
  useless one.
- **The optional cognitive-walkthrough pre-screen is the one sanctioned
  stand-in**, and it is explicitly scoped as "a hypothesis generator, never the
  proof artefact" (`project.md`, `## In scope`, last-but-two bullet). Treat any
  content fix the pre-screen alone justifies as provisional until the real
  session either confirms or supersedes it.
- **The pinned tree is this project's nearest analogue to a testkit `Fixture`.**
  CLAUDE.md's conformance-suite rule states the shape directly: "one fixture
  instance is one isolated backing store; each `connect()` on it is one handle
  onto that store" (`CLAUDE.md`, `## The rule that matters`). Applied here: one
  commit or merge point is one session's fixture (AC-005 records which), and a
  second session (AC-010) opens a new fixture instance rather than reopening
  the first one's — the discipline is the same, the noun that gets pinned is
  different.
- **A content fix's own seams are whatever RS-62 already governs** — no new
  mocking pattern is warranted for a doc-comment example. If a fix needs an
  `.await`d call, RS-62-2's hidden-`main` pattern is the existing primitive; if
  it needs to show usage that must not compile, RS-62-1's paired-fence pattern
  is, never a lone `compile_fail` fence trusting its error code.
