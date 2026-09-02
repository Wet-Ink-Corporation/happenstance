# Grounding — Comprehension Evidence (HS-P0024)

Companion file, not an item file. Written for `/redkiln:plan`'s briefs stage
(`forge-plan-briefs`) so `project.md`, `_design.md` and the briefs cite real repo
paths rather than invented ones. Not authoritative over `initiative.md`,
`_decomposition.md` or this project's `_intake-brief.md` — where this note and
those disagree, they win.

## Headline finding: no ADR governs this project directly

**None of the seventeen decision atoms under `.kb/decisions/` concerns
documentation** — `initiative.md` states this explicitly ("Vision and
narrative"), and a listing of `.kb/decisions/` confirms it: ADR-0001 through
ADR-0016 plus ADR-0029 cover port flavours, crate naming, opaque payloads,
MSRV, error traits, append conditions, position assignment, event identity,
validated identifiers and the wire format — none is about narrative prose, a
friction log, or persona evidence. This project's `_design.md` and briefs
therefore have **no Accepted decision atom to bind to or to flag tension
against** for its core subject matter. That absence is itself the grounding
finding, not a gap in this search.

The one governance atom that is relevant applies only indirectly, at the
routing seam (see below):

- **`.kb/governance/rewrite-the-referent-never-the-reasoning.md`** — the test
  applied to any `happenstance-core` doc-comment rewrite. This project does not
  rewrite doc comments itself, but AC-011 ("no settled decision was reopened in
  passing") is the same discipline one level up: a disposition that would
  change a resolved design tension owned by a sibling project must be escalated
  by DT id, never absorbed here as a silent fix. That is this atom's principle
  — preserve reasoning that still stands, change the referent through the
  proper channel — applied to design tensions instead of doc comments.

## Where DT-9 sits and what it binds

`comprehension-evidence` owns exactly one design tension, **DT-9** — "which
persona the comprehension session walks first" — named in both:

- `.bklg/docs-that-teach/initiative.md`, "Open design tensions" table (row
  DT-9): options (a) application author, (b) adapter author, (c) evaluator;
  trade-off is that "the three call for genuinely different comprehension
  checks, and the session is expensive enough that 'all three' is not free;
  whichever is chosen, the others' evidence stays inferred."
- `.bklg/docs-that-teach/_decomposition.md`, "Design tension ownership" table:
  same row, owner `HS-P0024 comprehension-evidence`. The decomposition also
  notes: "`comprehension-evidence` owns one tension and `durable-audience-closeout`
  owns none. That is deliberate: they carry owned requirements rather than
  unresolved interaction choices."

This is AC-001's subject. Since `design.capture` is absent from
`.redkiln/config.yaml` (confirmed below), the perceptual review is a skip and
`_design.md`'s written resolution of DT-9 is the **only** record this choice
will ever have — same discipline as every sibling project.

## The friction-log method: cited research, not invention

`.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md`
is the load-bearing research angle for AC-002/AC-004/AC-008. Key cited
findings this project's protocol brief should ground against directly:

- **The friction-log template** (Hammerly, developerrelations.com): a
  one-to-two-sentence scenario stated in user terms, logger identity/context/
  date, a **chronological** record including search terms typed and links
  followed, reactions captured **as they occur** (concurrent, not
  retrospective), and a stoplight (red/yellow/green) severity mark applied
  inline. This is the direct source for AC-004's "auditable shape."
- **Non-authorship is a methodological requirement, not a courtesy** — insiders
  "unconsciously route around" rough spots. Direct source for AC-003.
- **A log is evidence only once routed to someone who can act** — the source's
  own final, non-optional step. Direct source for AC-007's routing
  requirement.
- **Nielsen & Landauer (1993)**: one qualitative session finds ~1/3 of
  problems, and each problem a real reader hits is already proven — it need
  not recur to count. This is the grounding for BR-14/AC-008's scoped claim
  ("real stumbles were captured and are traceable," never exhaustiveness) and
  for AC-010's "was one session enough" assessment.
- **Nielsen's 0–4 severity scale**, converging with ~3 raters, combinable with
  the stoplight convention — a candidate concrete severity scale for the
  protocol brief's AC-002 to name (or the stoplight, or a stated local
  variant — the choice is this project's to record, not this note's to make).
- **Concurrent vs. retrospective think-aloud** (Hertzum 2024 meta-analysis):
  concurrent narration surfaces different, typically more findings but
  inflates task time ~17–20%. Direct source for AC-002's "narration mode" line
  item.
- **The three-technique taxonomy** (Jarrett & Redish: paraphrase / plus-minus /
  task-based, each keyed to what a document is *for*) maps onto the three
  personas and is offered as one way to justify whichever persona DT-9 picks.

No code path or ADR grounds this project's protocol; it is grounded entirely in
this cited external research plus the initiative's own distillation. Treat the
research file's citations as the anchors for the protocol brief's normative
claims about *why* the shape is auditable.

## Routing destinations for AC-007 — verified against real project files

AC-007 requires stumbles to route correctly: library bugs to `support`, page/
structure defects to "the sibling project owning the requirement," deliberate
deferrals to a recorded open question. All three destinations are real and
already describe themselves as routing targets from this project:

- **`support` initiative** (`.bklg/support/initiative.md`, id `HS-I0005`). Its
  routing is asserted directly by `.redkiln/config.yaml:5` —
  `support_initiative: support` — and by `CLAUDE.md`'s own non-goal:
  "Incidental bugs found in passing... route to the `support` initiative per
  `.redkiln/config.yaml`, and are not absorbed as in-scope fixes." The support
  initiative itself is currently an unfilled template (`stage: intake`,
  `status: exploring`) with no projects yet — real, but structurally empty; the
  briefs should say so rather than imply a populated backlog exists there.
- **`HS-P0022 application-author-path`** and **`HS-P0023 reach-and-adapter-path`**
  are the two sibling content projects, and both project.md files already
  name `HS-P0024` as *their* disposition destination reciprocally, which
  confirms the routing edge is real and bidirectional, not asserted only from
  this side:
  - HS-P0022's "Out of scope" section: "Recruiting the non-author reader,
    running the friction-log session, and dispositioning what it finds.
    HS-P0024 `comprehension-evidence`, which also owns DT-9. If the log shows
    this project's bridge does not land, the disposition routes through
    HS-P0024, not back into this charter silently."
  - HS-P0022 DoD item 9: "Anything this project found and did not fix is
    routed: incidental bugs to the `support` initiative per
    `.redkiln/config.yaml`, comprehension doubts to HS-P0024, pointer and
    reach gaps to HS-P0023. Nothing is absorbed silently."
  - HS-P0023's "Out of scope" table: friction-log recruiting/running/
    dispositioning, "including any verdict that the surfaced adapter account
    does not carry an adapter author," is HS-P0024's.
  - HS-P0023's risk table: "AC-008 may turn out to be insufficient... If the
    friction log shows the surfaced version does not carry an adapter author,
    a new account is owed and is dispositioned through HS-P0024 — not
    absorbed here." This is the concrete precedent for AC-011: a stumble that
    would reopen HS-P0023's AC-05 scope decision must be an escalation
    carrying that decision's provenance, not a silent fix inside this
    project.
  - HS-P0023's risk table also flags its own DoD-9 second questions as
    author-chosen and possibly wrong: "the evaluator has not been observed...
    HS-P0024 is the instrument that can falsify the choice." That is a
    second, explicit self-declared HS-P0024 dependency to ground the routing
    brief against.
- **HS-P0020 `checked-documentation-surface`** is not named as a disposition
  destination in either sibling's scope section, but is the project that pins
  the narrative tree by path in `xtask/src/` (its own project.md, "One-line
  objective") — relevant to AC-005 ("the log names the tree it walked...
  through HS-P0020 through HS-P0023"), not to routing.
- **A recorded open question** — the third AC-007 destination — is
  `.kb/open-questions/` atom-shaped (nineteen exist today, e.g.
  `dcb-reference-publishes-no-wire-format.md`), but per `CLAUDE.md` and the
  initiative's own non-goal, `.kb/` atoms are authored only through
  `/redkiln:kb-ingest` or closeout — never hand-authored inside a project. A
  "deliberate deferral routed to a recorded open question" from this project's
  friction log is therefore itself a disposition that stages material for
  ingest (or for `HS-P0025 durable-audience-closeout`'s promotion pass), not a
  file this project writes directly. The briefs should reflect that mechanism
  rather than imply this project edits `.kb/open-questions/` itself.

## The design/product layer admission rules — relevant to what this project may and may not write

- **`.kb/product/README.md`**: personas/journeys are `concept`/`playbook` atoms
  at `authority_tier: product`. Explicitly **not** promoted mid-initiative — an
  "unevidenced sketch... stays in that initiative's
  `_discovery/distillation/personas-and-journeys.md` until closeout promotes
  it." This project cites the initiative's own
  `_discovery/distillation/personas-and-journeys.md` for persona detail (three
  personas, `Persona 1` app author at line ~56, `Persona 2` adapter author at
  line ~148, `Persona 3` evaluator at line ~225 in that file) and does **not**
  author or promote a `.kb/product/` atom — that is `HS-P0025
  durable-audience-closeout`'s job per BR-17/AC-14, confirmed by the
  decomposition's dependency graph (HS-P0024 → HS-P0025, one-directional).
- **`.kb/design/README.md`**: interaction-pattern decisions at
  `authority_tier: design`, harvested from a project's `_design.md` "at
  closeout." Same rule applies to DT-9's resolution: it is written in this
  project's own `_design.md` and stays there; promotion (if any) is a closeout
  concern, not this project's.

## Confirmed mechanics for `_design.md` and the perceptual-review skip

`.redkiln/config.yaml` (read directly, lines 1–20+): `support_initiative:
support` (line 5, matching every citation above), and the file's own comment
block states `design.capture` is commented out / absent — matching
`CLAUDE.md`'s "`design.capture` is deliberately absent from
`.redkiln/config.yaml`" and every sibling project's identical statement that
the perceptual review is a skip. This project's `_design.md` resolving DT-9 is
therefore the sole record of that choice, same as every other project in this
initiative — no special case to design around here.

## Existing code/precedent patterns worth citing, with a caution

- `examples/course-subscriptions/` — the one real worked example in this
  worktree, `publish = false`
  (`examples/course-subscriptions/Cargo.toml:8`), owned by HS-P0022, not this
  project — relevant only as background for what the friction-log scenario
  will actually walk if DT-9 resolves to the application-author persona.
- **Caution, verified directly**: `initiative.md`'s own open-questions section
  flags `examples/outside-projection-adapter/` (cited in
  `_discovery/distillation/opportunities.md`, Opportunity 5, as an existing
  precedent for a non-author-built artefact) as **"asserted and unverifiable
  from this worktree."** A `Glob`/listing check from this worktree root did
  not surface that path under `examples/`; do not cite it in this project's
  briefs as a reachable anchor. If a brief wants Opportunity 5's precedent, cite
  the opportunities.md paragraph itself, not the unverified example path.
- **This project's own item files do not yet exist** beyond the intake-brief
  stub: `.bklg/docs-that-teach/comprehension-evidence/project.md`,
  `_design.md` and `_storymap.md` are not yet authored (only
  `_intake-brief.md` and this note exist in that directory as of this
  grounding pass). Briefs should treat AC-001 through AC-011 (quoted verbatim
  in the task prompt from stage-1 decomposition) as the spine to expand into
  derived requirements, following the shape HS-P0022 and HS-P0023's already-
  authored `project.md` files use (`## Derived requirements` numbered DR-n,
  traced back to a BR).

## Summary of tensions/risks to carry into the briefs

1. **No Accepted ADR binds this project's subject matter.** State this
   explicitly in `_design.md` rather than searching for a citation that does
   not exist — a false citation would be worse than none.
2. **AC-011's escalation discipline has a live precedent already on record**
   in HS-P0023's own risk table (AC-008 insufficiency → routes through
   HS-P0024, not absorbed) and HS-P0022's own scope section (bridge failure →
   routes through HS-P0024). The briefs can point at those two passages as
   the concrete shape "escalation carrying a DT id" must take, symmetrically.
3. **The `support` initiative is real but currently empty** — routing there is
   correct per config, but the briefs should not describe it as an active
   backlog with existing stories to slot into.
4. **`examples/outside-projection-adapter/` is not confirmed reachable** in
   this worktree despite being cited as precedent elsewhere in the discovery
   corpus — do not treat it as a citable anchor for this project's own briefs.
5. **DT-9's resolution is genuinely open** — this note deliberately does not
   pre-select a persona; that is `_design.md`'s decision to make and record,
   grounded in the research/04 taxonomy and the three persona write-ups in
   `_discovery/distillation/personas-and-journeys.md`.
