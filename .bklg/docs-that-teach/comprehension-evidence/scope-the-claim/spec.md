---
item: HS-S0169
stage: spec
created: 2026-08-17T13:16:21.956Z
updated: 2026-08-17T13:16:21.956Z
template_sig: 87bbf1d0
rendered_sig: c1693fd8
---

# Spec — Scope the claim to what one session supports

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project (the spine this story serves) | `.bklg/docs-that-teach/comprehension-evidence/project.md` |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/spec.md` |
| Briefs (ux + testing, one `##` each; no architecture, no deployment) | `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` |
| **Signed-off design — binding** | `.bklg/docs-that-teach/comprehension-evidence/_design.md` |
| Grounding (the cited research behind the scoped claim, verified anchors) | `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` |
| Story map (slice cut, merge order, the AC-008 coverage split) | `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` |
| This story's discover artifact | `.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/discover.md` |
| The artifact this story writes into | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` |
| Roadmap pointer | none. `RUNBOOK.md` sequences library phases, not documentation projects; this initiative's ordering is `_decomposition.md`'s dependency graph and `_storymap.md`'s merge order. |

`_design.md` is **binding and already signed off** (Ryan Britton, 2026-08-17, no
conditions). It records `hasSurface: false` explicitly rather than being skipped: this
project ships no `pub` item, no screen and no crate change, and its `## Items` block is
an empty `yaml` fence with the reason written out. What it *does* durably record is the
protocol — DT-9's persona, the scenario, the narration mode, the severity scale, the
disqualifying criteria — written there by `dt9-and-fixed-protocol`. **This story reads
none of that as something to re-decide and amends none of it.** Where this spec and
`_design.md` appear to disagree, `_design.md` wins and this spec is wrong.

## One-line PR slice

State the narrow claim — real stumbles were captured and are traceable — wherever this
evidence is summarised, and assert nothing about exhaustiveness anywhere in the log or
its summary.

## Executive summary

**What this PR lands.** Not a sentence — a *closed set of places the sentence has to be*,
and a defence of it that a stranger can run. `friction-log-skeleton` already pre-wrote
`## Scope of the claim` in the log, deliberately before any finding existed, so that the
claim could never be tuned to what the reader happened to hit
(`friction-log-skeleton/spec.md:260`). `session-run-against-pinned-tree` then filled the
log with real findings. This story is what happens next: it **adopts that pre-written
sentence verbatim as the canonical string**, puts it at every point where this evidence
is summarised rather than read in full, and makes the negative half — no exhaustiveness,
no generalisation, no verdict on the documentation — checkable rather than trusted.

**The delta.** Before: one scoped sentence, correct, sitting in section seven of a file a
downstream reader may never open past its `## Status` banner or its `## Dispositions
index`. After: the same string, byte-identical, at each of four summary points inside the
log (`## Status`, `## Dispositions index`, `## Scope of the claim`, the `## Hand-off`
scope slot) and in the one place outside it where a person meets a description of this
evidence without opening it — `project.md`'s `## Companions` row, which until now
described the file as a scaffold. Plus the negative check, stated as a vocabulary rather
than as care.

**Why this is a vertical slice and not "write a disclaimer".** Because the failure it
prevents is not lexical. `rg -i "exhaustiv"` — the check the testing brief names
(`_decomposition.md`, `## Testing brief`, AC-008 row) — is a check almost nothing would
ever fail: nobody writes "our documentation testing was exhaustive". The overclaim that
actually happens is quieter and is already in the risk table as *likely*: one reader
becomes "users", seven stumbles become a quality rate, and a log of real friction becomes
"the docs were validated" by the time it reaches promotion (`project.md`, `## Risks and
coupling notes`, row 2). Catching that needs the claim stated positively as well as
negatively, present at every summary point rather than the canonical one, and inheriting
the session's own qualifications instead of laundering them.

**What this story is not.** It does not disposition, route, escalate, fix content, decide
the second-session question, or write the hand-off note's persona, date or tree. Each is
a named sibling story. See `## PR boundary`.

## Context pack

The load-bearing decisions, distilled. Everything deeper sits behind the signposted
anchors and is opened just-in-time, not read up front.

### The persona-journey slice this realizes

**U3, the downstream actor** — a sibling-project owner, the `support` initiative, and
above all HS-P0025 `durable-audience-closeout`. The UX brief's three-user table says what
U3 is trying to do: *open the log, find the items that are theirs, and act — without
reading the whole thing and without asking the logger what an entry meant*
(`_decomposition.md`, `## UX brief`). "Without reading the whole thing" is the whole
reason this story exists: **a reader who does not read the whole thing reads a summary,
and a summary is exactly where a scoped claim gets dropped.**

The specific journey is HS-P0025's. Today every persona in this initiative carries the
qualification that *"None of these three personas has been directly observed by this
initiative"* (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`,
`## Risks`). HS-P0025 replaces that with an accurate qualification, and the bar it is
held to is `.kb/product/README.md:24` — *"a persona nobody researched is a stock photo
with a name"*. This story is what keeps the replacement from over-correcting: one
observed persona is one observed persona, and the sentence that says so must travel with
the evidence rather than be re-derived by whoever cites it.

### Decision 1 — the canonical sentence is *adopted*, not authored here

`friction-log-skeleton` wrote `## Scope of the claim` **before the session ran**, and its
spec states why in one line: *"Writing it before the session is what stops it being tuned
to the findings"* (`friction-log-skeleton/spec.md:260`). That property is destroyed the
moment this story rewrites the sentence with the findings in view — even to improve it.

So: whatever string sits in `## Scope of the claim` at the skeleton's commit is **the
canonical string**, and this story propagates it character for character. If it is
genuinely defective — missing the positive half of Decision 3, say — the correction is
made as a visible amendment that names the original and the reason beside it, never as an
in-place overwrite (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`:
the referent may be rewritten, the reasoning may not be erased). A silent post-session
rewrite of this section is the one edit that would make project AC-008 unfalsifiable.

### Decision 2 — "wherever this evidence is summarised" is an enumerated set, not a judgement call

Project AC-008 says the claim is scoped *"wherever it is made"*. Left as a standard, that
is unfalsifiable — a reviewer cannot check "everywhere". This story closes it into five
named locations, four inside the log and one outside, and the set is the criterion:

1. **`_friction-log.md` → `## Scope of the claim`** — the source of record, skeleton-landed.
2. **`_friction-log.md` → `## Status`** — the banner the skeleton built as the
   *anti-miscitation* line (`friction-log-skeleton/spec.md:250`). After
   `session-run-against-pinned-tree` replaces "no session has been run" with the date and
   tree, this is the first thing a citing reader sees, and it is a summary of the file's
   evidentiary standing whether or not it was designed as one.
3. **`_friction-log.md` → `## Dispositions index`** — additive and derived by
   construction, and the section a downstream owner opens *instead of* the chronological
   record (`friction-log-skeleton/spec.md:259`). A derived view read in place of the
   record is a summary.
4. **`_friction-log.md` → `## Hand-off`, its scope slot** — the slot the skeleton reserved
   for exactly this string (`friction-log-skeleton/spec.md:261`).
   `handoff-note-to-closeout` fills the persona, date and tree slots beside it and is
   *bound to carry this sentence unchanged*; `_storymap.md`'s `## Coverage` states the
   split for AC-008 in those terms.
5. **`project.md` → `## Companions`**, the row `friction-log-skeleton` added. This is the
   only description of this evidence a person meets **without opening the log at all**,
   reached from `redkiln board`. It currently says the file is a scaffold until the
   session runs; after the session that sentence is false, and the row that replaces it is
   a one-line summary of the evidence — so it carries the claim.

The set is closed on purpose. If a sixth summary point appears — a project-level review
artifact, a routing message body — it is added *here*, deliberately, before the PR, not
discovered at gate time.

### Decision 3 — the claim has two halves, and shipping only the negative half is a failure

`_grounding.md:72-76` records the basis precisely: one qualitative session finds roughly a
third of a design's problems, **and each problem a real reader hits is already proven — it
need not recur to count** (Nielsen & Landauer 1993). Both clauses matter and they point in
opposite directions.

- **The limit.** Nothing here is exhaustive, representative, or a measurement of anything.
- **The licence.** Each recorded stumble is a real stumble a real reader hit, traceable to
  an `FL-###` id, a named tree SHA and a dated session — and is already worth fixing
  without needing to recur.

A summary that carries only the limit reads as "weak evidence, discount it", which is
precisely the outcome the initiative's own framing forbids: the gate and the comprehension
record *"falsify different things and neither may stand in for the other"*
(`project.md`, `## How this advances the initiative`, quoting `initiative.md`, `## Risks`,
first row). Under-claiming is not the safe direction; it is the other way to make one
expensive session worthless.

### Decision 4 — the overclaim to defend against is not the word "exhaustive"

The testing brief's AC-008 mechanism is `rg` for the scope sentence plus *"`rg -i
"exhaustiv"` returning nothing outside a disclaiming sentence"* (`_decomposition.md`,
`## Testing brief`). Take that as the floor, not the check — on its own it is the
decorative rule `CLAUDE.md` warns about: name the plausible wrong implementation it
rejects, and there isn't one, because nobody writes that word.

The wrong implementations that are actually plausible here, each of which this story's
checks must reject:

- **The plural slide.** One reader becomes "users", "readers", "a developer coming to
  this cold". Every generalisation past the single observed person is unlicensed.
- **The count as a rate.** "Seven stumbles, two blocking" is a fact; "seven stumbles, two
  blocking — a 70% success rate", "most readers got through", or any percentage at all is
  a measurement one session cannot produce.
- **The verdict.** "The documentation is broadly usable", "the material tested well", "no
  major issues found". Absence of a stumble in one walk is not evidence of its absence,
  and a verdict on the documentation as a whole is the claim BR-14 exists to forbid
  (`initiative.md:349`).
- **The laundering hop.** The sentence is present in the log and absent from the thing
  someone actually quotes — the index preamble, the board row, the hand-off slot. This is
  the failure the enumerated set in Decision 2 exists to make impossible.

### Decision 5 — qualifying session states are inherited by the summary, never dropped

Two of the session's own recorded states change what the evidence supports, and both must
travel with the claim wherever it goes:

- **Ineligible-but-used-anyway.** If the recruited reader failed `_design.md`'s
  disqualifying criteria and was used anyway, the log says the artefact is **unmet** —
  *"a failure of the initiative, not a reason to redefine the bar"* (`project.md`,
  `## Risks and coupling notes`, row 1; `_decomposition.md`, `## UX brief`, the states
  list). `session-run-against-pinned-tree`'s spec closes the loop in this story's
  direction: *downstream summaries inherit that statement; the hand-off note may not
  quietly drop it.*
- **Abandonment.** If the reader stopped at a blocker, the walk was partial and the scope
  statement says so. IQ-6 makes the partial log evidence, not a void — but evidence of a
  shorter walk than the scenario described.

Where neither occurred, the scope statement says nothing extra; that is not a licence to
omit the qualification when one did.

### Decision 6 — one source of record, and copies that cannot drift silently

Four of the five locations hold a *copy*. The invariant that makes copies safe is the one
the log already uses for its derived index: each copy names `## Scope of the claim` as the
source of record, and the strings are byte-identical, so a divergence is a `diff`, not a
matter of interpretation. Four paraphrases that agree today are four sentences that will
disagree after the next edit, and nothing would report it.

This is also the IQ-2 shape applied to prose rather than to findings: the derived copies
are additive. Delete every one of them and the claim is still stated in full at its source
(`_decomposition.md`, IQ-2 / UX-AC-006).

### Decision 7 — this story does not wait on dispositions, and must not disturb them

`_storymap.md`'s dependency graph gives this story exactly one edge:
`session-run-against-pinned-tree`. It deliberately does **not** depend on
`disposition-every-stumble` — *"a fix that lands late must not be able to hold up the
evidence HS-P0025 needs"*. Two consequences follow.

The `## Dispositions index` may be empty or partial when this story writes its preamble
line. That is expected and is not a reason to wait. And `disposition-every-stumble` and
`route-and-escalate` will append rows **beneath** that preamble afterwards — so the line
this story writes goes in the preamble, above the table, and those stories rewrite neither
it nor the entries, in the same append-only discipline the chronological record is held to
(IQ-3, IQ-4).

### Standing constraints inherited, not re-decided

- **Nothing under `.kb/` is written by this story.** Hand-authoring atoms outside the
  ingest path is a named non-goal of the initiative and the first attempt was reverted
  (`0269720`). Promotion is HS-P0025's, at closeout.
- **No persona is promoted, reconciled, renamed or named as observed.** Naming the
  directly observed persona is `handoff-note-to-closeout`'s. This story writes the sentence
  that constrains how that name may be read.
- **No item frontmatter is touched.** `project.md`'s body prose only; the CLI is the single
  writer of `id`, `stage`, `status`, `updated` and `links`, and a `PreToolUse` hook denies
  the edit (`CLAUDE.md`, `## Where the work lives`).
- **No new heading vocabulary, no fold, no widget, no CSS or token layer.** The log's eight
  sections are fixed by `friction-log-skeleton`; this story adds text inside four of them
  and adds no section (`_decomposition.md`, `#### The primitive layer to compose from — do
  not hand-roll`).
- **One-line checkboxes.** Any checkbox this story writes stays on a single line; the gate
  parser matches line by line and a wrapped box can never match.
- **Quizzes and inline recall checks stay settled out** and are not reachable from here.
- From `CLAUDE.md`'s binding constraints: none is touched, and none may be disturbed. This
  story compiles nothing, adds no `pub` item and goes near no port.

## Integration contract

This story is delivered **mounted**. The deliverable is not a sentence in a spec or a note
in this story's folder — it is text living in the real friction log and on the real project
card, in the four log sections and the one board row named below. A scope statement that
exists only in this spec is the exact laundering failure Decision 4 names.

- **Archetype**: `capability` — a user-observable slice (`_storymap.md`, slice table). The
  observable is what U3 and HS-P0025 read at every point where this evidence is summarised.
- **Slice / milestone**: `scoped-claim-and-handoff`. **Slice-mates:**
  `second-session-decision` and `handoff-note-to-closeout` — implemented together in one
  context and mounted as one integrated surface. Order inside the slice:
  `scope-the-claim` and `second-session-decision` in either order, then
  `handoff-note-to-closeout` last (`_storymap.md`, `## Merge order`, step 4).
- **Mount point**: `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` →
  **`## Scope of the claim`**, the canonical string and the source of record, with
  `## Status`, `## Dispositions index` and the `## Hand-off` scope slot as its in-file
  propagation points and `.bklg/docs-that-teach/comprehension-evidence/project.md` →
  `## Companions` as the one propagation point outside the log. The path is
  `friction-log-skeleton`'s to fix and `_design.md`'s to bind: **if `_design.md`'s protocol
  section names a different home, that path substitutes verbatim everywhere below**, and
  the PR boundary is widened here, deliberately, before the work — never at gate time.
- **Wires into**:
  - `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — the eight-section
    shape `friction-log-skeleton` fixed and the `FL-###` entries
    `session-run-against-pinned-tree` filled. Read for the qualifying states of Decision 5;
    **the chronological record is never edited by this story**.
  - `.bklg/docs-that-teach/comprehension-evidence/_design.md` — the protocol, the log's
    bound path and heading vocabulary. Consumed, never amended.
  - `.bklg/docs-that-teach/comprehension-evidence/project.md` → `## Companions`
    (currently lines 362–372) — the project card's drill-down index, the only render path
    by which a person reaching HS-P0024 from `redkiln board` meets this evidence. Body
    prose only.
  - `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract, enforced by
    `require_ledger: true` (`.redkiln/config.yaml:67`) — which is what makes "the sentence
    is at five named locations" a checkable row rather than an assurance.
  - `.redkiln/config.yaml:40` (`affected_gate`) and `:55` (`integration_scoped`) — the
    story-grain and integration-grain commands this PR is checked by. Touching nothing under
    `crates/`, `cargo xtask affected` falls through to the file-reading lints and
    `spec-trace` rather than passing vacuously on an empty package set.
  - Read-only, as the bar the sentence is written against:
    `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` (`## Risks`,
    the "none directly observed" qualification) and `.kb/product/README.md:24`.
- **Renders surfaces**: **no `_design.md` surface id, because `_design.md` declares none** —
  its `## Items` block is an empty `yaml` fence and every shape section reads `N/A — no
  user-facing surface`, with `hasSurface: false` recorded rather than the project being
  skipped. What this story changes is the UX brief's **document surface 2, the friction
  log** — *"a dated markdown artifact whose reader is a reviewer, a sibling-project owner,
  and eventually HS-P0025"*, whose legibility the brief calls *the whole deliverable* — in
  four of its eight sections, plus the project card's `## Companions` row. It renders
  nothing else: not surface 1 (the material walked), not surface 3 (`_design.md`).
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No rule in
  `crates/happenstance-testkit/src/suite.rs` can observe a scope sentence; no port,
  signature or bound changes, so there is nothing an adapter could implement wrongly. Stated
  rather than left blank, per `CLAUDE.md`'s requirement that a story changing a port names a
  rule — this story changes no port. It adds no rule and no literal position assertion.
- **Clause(s)**: **discharges and amends none.** No `spec/SPECIFICATION.md` clause is
  edited, no `[FROZEN]` clause changes, no line-anchored citation moves, so no ADR is owed
  and `cargo xtask spec-trace` has nothing to re-anchor.
- **Advances DoD scenario**: **project DoD item 4** — *"The scope statement is present
  wherever the evidence is cited (AC-008), including in whatever HS-P0025 will read at
  closeout"* (`project.md`, `## Definition of done`) — which this story takes to green. At
  initiative grain it advances no scenario alone and says so rather than claiming one: it
  **guards** DoD-5 and DoD-6 (`_decomposition.md`, `### Definition of Done`, rows 5 and 6,
  both HS-P0024's) against being read as more than they are, and it is a **precondition for
  DoD-15's honesty** — the audience atoms HS-P0025 promotes carry an evidence qualification
  that this sentence is the source of. It traces initiative BR-14 (`initiative.md:349`) and
  project AC-008.

## PR boundary

### In this PR

- The **canonical scope string** adopted verbatim from `## Scope of the claim` as the
  skeleton committed it, with the positive and negative halves of Decision 3 both present —
  amended, if it genuinely lacks one, as a visible amendment naming the original beside it.
- The same string at the **four remaining summary points**: the `## Status` banner, the
  `## Dispositions index` preamble, the `## Hand-off` scope slot, and `project.md`'s
  `## Companions` row (body prose only, never frontmatter).
- The **source-of-record label** on each copy, naming `## Scope of the claim`.
- The **inherited qualification** where the session recorded ineligible-but-used-anyway or
  abandonment, carried at every one of the five locations.
- The **negative sweep** over the log and its summary points against the vocabulary in
  Decision 4, and the correction of anything it finds that is this story's to correct — a
  summary sentence, an index preamble, a board row.
- This story's own backlog folder: the `_ledger.md` with `file:line` evidence per AC, and
  any working notes.

### Explicitly not in this PR

- **Any `FL-###` entry**: no text, no severity, no timestamp, no ordering, no id. The
  chronological record is `session-run-against-pinned-tree`'s and is append-only and frozen
  at write time (IQ-3). If the sweep finds an overclaiming phrase inside a *reader's own
  quoted words*, it stays exactly as recorded — the record of what someone said is not a
  summary and is never edited to be more careful.
- **Any disposition, route, escalation or destination id** — `disposition-every-stumble`'s
  and `route-and-escalate`'s. This story writes the index's *preamble*, never its rows.
- **The second-session verdict** — `second-session-decision`'s, a slice-mate. "One session"
  is a fact this story's sentence states; whether a second is owed is that story's recorded
  call (project AC-010).
- **The hand-off note's persona, date and tree** — `handoff-note-to-closeout`'s. This story
  fills only the scope slot beside them.
- **Any content fix** to `docs/README.md`, a doc comment, `examples/course-subscriptions/`
  or any crate — `content-fixes-from-dispositions`'s.
- **Any file under `.kb/`**, including staging a persona or an open question, and any
  persona promotion or reconciliation. HS-P0025's, through the ingest path.
- **`_design.md`'s protocol** — persona, scenario, narration mode, severity scale,
  disqualifying criteria. `dt9-and-fixed-protocol`'s, and its commit date predating the
  session is what makes project AC-002 checkable.
- **Any frontmatter, anywhere.** The CLI is the single writer of item system fields.

### Merge DoD

The canonical string is byte-identical at all five named locations with each copy naming
its source of record; the negative sweep is clean against the Decision 4 vocabulary across
the log and its summary points, with any surviving hit sitting inside the disclaiming
sentence itself; the positive half of the claim is present with its basis; any
ineligible-but-used-anyway or abandonment qualification recorded by the session is carried
at every location; nothing under `## Chronological record` changed; `cargo xtask affected
--base main` green (falling through to the file lints and `spec-trace`, this story touching
no crate); `redkiln validate && redkiln doctor` clean with no new `template-drift` beyond
the six standing advisories; and the `_ledger.md` carries a real `file:line` per AC.

### Paths

`redkiln verify --grain story` reads the first fenced block under this heading and fails on
any file changed outside it. Narrow on purpose: two files this story writes into, plus its
own folder. If `_design.md` binds the log elsewhere, this block is widened here, before the
work.

```
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/comprehension-evidence/project.md
.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/**
```

## Behavior and interfaces

The observable behaviour, claim by claim. "Evidence path" is where the claim is grounded
today — the thing to read, not the thing to copy. **The log** throughout means
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`; **the canonical string**
means the text of `## Scope of the claim` as `friction-log-skeleton` committed it.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **the canonical string is adopted, never re-authored after the findings exist** | Whatever `## Scope of the claim` says at the skeleton's commit is the string this story propagates, character for character. A post-session rewrite — even an improving one — destroys the property the pre-writing bought and makes project AC-008 unfalsifiable. A genuine defect is corrected as a visible amendment naming the original and the reason beside it, never in place. | `friction-log-skeleton/spec.md:260` ("writing it before the session is what stops it being tuned to the findings") · `.kb/governance/rewrite-the-referent-never-the-reasoning.md` · `project.md` AC-008 |
| **the pre-session provenance stays checkable** | `git log -p` on the log shows the `## Scope of the claim` body unchanged since the skeleton's commit, or shows an amendment that is additive and dated. The same provenance discipline the project already applies to `_design.md` predating the session date, applied to the one sentence that is most tempting to tune. | `_decomposition.md`, `## Testing brief`, AC-002 and AC-008 rows · `friction-log-skeleton/spec.md:296` |
| **the summary set is enumerated and closed** | Five locations, named in Decision 2: `## Scope of the claim` (source), `## Status`, `## Dispositions index` preamble, `## Hand-off` scope slot, and `project.md`'s `## Companions` row. "Wherever it is made" is discharged by that closed list, not by reviewer vigilance. A sixth point is added to the list deliberately, before the PR. | `project.md` AC-008 · `friction-log-skeleton/spec.md:249-261` (the eight sections and which stories fill 7 and 8) · `project.md:362-372` (`## Companions`) |
| **every copy is byte-identical and names its source of record** | Each of the four copies reproduces the canonical string exactly and states that `## Scope of the claim` is the source of record. Paraphrases are forbidden: four sentences that agree today diverge at the next edit and nothing reports it. | Decision 6 · `_decomposition.md` IQ-2 / UX-AC-006 (a derived view is additive) · `friction-log-skeleton/spec.md:259` (the index states in its own first line that it is derived) |
| **the derived copies are deletable without loss** | Delete `## Status`, the index preamble line, the hand-off slot line and the Companions row, and the claim is still stated in full at its source. The copies exist for reach, never as the only home of the claim. | `_decomposition.md` IQ-2 (the deletion check, stated verbatim) |
| **no exhaustiveness claim, on a vocabulary wider than the word** | The sweep covers `exhaustiv`, `comprehensive`, `complete (set\|picture\|coverage)`, `all the (issues\|problems\|stumbles)`, `every (issue\|problem)`, `no (other\|further\|major) issues`, `representative`, `typical (reader\|user\|developer)`, `most (readers\|users)`, `readers/users (generally\|tend\|will)`, `%`, `success rate`, `validated`, `signed off as usable`. Any hit must sit inside the disclaiming sentence itself. The bare `exhaustiv` check is the floor, not the check — on its own it rejects no implementation anyone would write. | `_decomposition.md`, `## Testing brief`, AC-008 row (the floor) · `CLAUDE.md`, `## The rule that matters` (a rule nothing can fail is decorative) · `initiative.md:349` (BR-14) |
| **one reader is never spoken of as "users", and a count is never a rate** | Counts are counts: "seven stumbles, two blocking" is a fact about this walk. A percentage, a success rate, "most readers", or a verdict on the documentation as a whole is a measurement one session cannot produce. Absence of a stumble in one walk is not evidence of its absence. | `_grounding.md:72-76` (Nielsen & Landauer — a third of problems; each hit problem already proven) · `project.md`, risk table, row 2 · `initiative.md:503` |
| **the positive half is stated with its basis** | The claim names what the evidence *does* license: each recorded stumble is a real stumble a real reader hit, traceable to an `FL-###` id, a named tree SHA and a dated session, and already worth fixing without needing to recur. A summary carrying only the limit invites downstream to discount evidence the initiative says cannot be substituted for. | `_grounding.md:72-76` · `project.md`, `## How this advances the initiative` (the two instruments falsify different things) · `initiative.md`, `## Risks`, first row |
| **qualifying session states are inherited, never laundered** | Where `## Session record` records **ineligible-but-used-anyway**, every summary point states that the artefact is unmet — a failure of the initiative, not a moved bar. Where the session ended in `Kind: abandonment`, every summary point states the walk was partial and where it stopped. | `project.md`, risk table, row 1 · `_decomposition.md`, `## UX brief`, the states list · `session-run-against-pinned-tree/spec.md`, EC-002 ("downstream summaries inherit that statement; the hand-off note may not quietly drop it") · IQ-6 |
| **the project card stops describing a scaffold** | `## Companions`' row for the log currently says it is a scaffold until the session runs. After the session that is false. The replacement row is one line, carries the claim and any inherited qualification, and touches body prose only. | `friction-log-skeleton/spec.md:264` (the row as landed) · `project.md:362-372` · `CLAUDE.md`, `## Where the work lives` (frontmatter is the CLI's) |
| **the sentence is liftable in one hop, as plain text** | HS-P0025 can copy the string out of the `## Hand-off` slot without re-deriving it, reading the file as plain markdown — no fold, no widget, no rendering step, no script, and no new heading. Browser find and stable heading anchors only. | `_decomposition.md` UX-AC-013, UX-AC-004, `#### The accessibility floor` · `friction-log-skeleton/spec.md:261-262` |
| **the index preamble is written above rows that do not exist yet** | This story does not wait on `disposition-every-stumble`; the preamble line goes above the table, and the disposition stories append rows beneath it without rewriting it. | `_storymap.md`, `### Dependency graph` (the deliberate non-edge) and `## Merge order`, step 4 · `_decomposition.md` IQ-3 / IQ-4 |
| **nothing else moves** | No `FL-###` entry, no severity, no disposition, no destination id, no DT escalation, no second-session verdict, no persona name, no `.kb/` file, no `_design.md` amendment, no doc comment, no `docs/README.md` edit, no crate, no `SPECIFICATION.md` clause, no conformance rule, no frontmatter. A phrase inside a reader's quoted words is left exactly as recorded. | `project.md`, `## Out of scope` · `_storymap.md`, slices table · `CLAUDE.md` binding constraints |

### The acceptance criteria this story enumerates

Seven, in the order the behaviour above lands them, framed from the intent of the person
each serves — U3 the downstream actor, and HS-P0025 as the specific downstream reader:
**AC-001** (the canonical string is the pre-session one, adopted verbatim, with its
provenance intact) · **AC-002** (the string is present at all five enumerated summary
points, byte-identical) · **AC-003** (no exhaustiveness claim anywhere in the log or its
summary, on the wider vocabulary) · **AC-004** (no generalisation past the one observed
reader; no count presented as a rate; no verdict on the documentation) · **AC-005** (the
positive licence is stated with its basis, so the scoping reads as a licence and not only
a hedge) · **AC-006** (ineligible-but-used-anyway and abandonment qualifications are
inherited at every summary point) · **AC-007** (one source of record, copies labelled as
copies and deletable without loss, liftable in one hop as plain text).

## Data and migrations

**N/A — no schema, no store, no migration, and nothing that compiles.** This story adds
prose to four sections of one markdown file and rewrites one row of another. It defines no
type, adds no `pub` item, touches no crate under `crates/`, opens no connection and writes
no row; `_design.md` records `hasSurface: false` and an empty items block for exactly this
reason, and `happenstance-core`'s no-`serde`-by-default constraint is nowhere near
anything here.

Two things are worth naming as *near*-data rather than left to inference.

**The canonical string behaves like a fixture value with four references.** Its "schema" is
that the four copies are byte-identical to the source, and its only legal "migration" is an
additive amendment that leaves the original legible beside it — the same shape
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` imposes on a disposition
revision and `friction-log-skeleton` imposes on the `Revisions` slot. There is no
propagation mechanism and no template expansion: a divergence is found by comparison, which
is why the copies name their source of record rather than merely agreeing with it.

**The `FL-###` id space is read here and never written.** `session-run-against-pinned-tree`
fixed it as append-only, occurrence-ordered and frozen at write time, and sibling projects
will foreign-key into those anchors through `_ledger.md`'s `evidence: "file:line"` rows.
This story cites ids when the positive half of the claim points at traceability, and
renumbering, reordering or re-heading remains the forbidden operation — a broken citation
with no error message (`_decomposition.md`, IQ-3).

## Acceptance criteria

Seven criteria, framed from the intent of the person each serves crossing the whole
artifact — **U3, the downstream actor**, and **HS-P0025 `durable-audience-closeout`** as
the one downstream reader whose input this project cannot manufacture. A criterion framed
as a bare capability ("the log has a scope sentence") is satisfied by a sentence sitting in
section seven that nothing downstream ever reads, which is precisely the failure Decision 4
names as the laundering hop. The personas are `_decomposition.md`'s `## UX brief`
three-user table; the journey they cut through is `_storymap.md`'s backbone A5.

Throughout: **the log** is
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` (or the path `_design.md`
fixes, per `## Integration contract`); **the canonical string** is the text of
`## Scope of the claim` as `friction-log-skeleton` committed it, before any finding existed;
**the five locations** is Decision 2's enumerated, closed set.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the scope sentence was written before the session so it could not be tuned to what the reader happened to hit, **WHEN** U3 six months later asks whether the claim was trimmed to fit the findings, **THEN** `## Scope of the claim`'s body is the string the skeleton committed, character for character — and where it was genuinely defective, the correction sits **beside** the original as a dated additive amendment naming what it replaces and why, never as an in-place overwrite | Static provenance: `git log -p --follow -- <log>` across this story's commits shows the `## Scope of the claim` body either unchanged since the skeleton's commit or extended by an append-only amendment hunk with no deletion inside the section; the skeleton's commit date precedes the session date recorded in `## Session record`. Rejects the wrong implementation Decision 1 names — a post-session rewrite that *improves* the sentence, destroys the property the pre-writing bought, and makes project AC-008 unfalsifiable because the claim now provably fits the findings |
| **AC-002** | **GIVEN** U3 meets this evidence at whichever point they happen to land on — the board card, the `## Status` banner, the dispositions index, the hand-off slot — **WHEN** they read only that point and never open the chronological record, **THEN** the canonical string is there, byte-identical, at every one of the five enumerated locations, each rendered as a composed line in that section's existing shape rather than a paragraph dropped under a heading | Static presence + byte-identity: `rg -F -n "<canonical string>" <log> .bklg/docs-that-teach/comprehension-evidence/project.md` returns a hit inside each of the five named sections, and a span-by-span comparison of the five extracted strings shows zero divergence — a `diff`, not a judgement of whether two sentences "say the same thing". Rejects the laundering hop: the claim present in the log and absent from the thing people actually quote; and rejects four agreeing paraphrases, which are four sentences that diverge at the next edit with nothing to report it |
| **AC-003** | **GIVEN** a promotion reviewer at closeout is deciding what may honestly be written into a persona atom, **WHEN** they scan the log and everything that summarises it for any assertion of coverage, **THEN** they find none on the full Decision 4 vocabulary — not merely the word "exhaustive" — and every hit the sweep does return sits inside the disclaiming sentence itself | Static, `rg -i` over `<log>` and `project.md`'s `## Companions` for the whole vocabulary: `exhaustiv`, `comprehensive`, `complete (set\|picture\|coverage)`, `all the (issues\|problems\|stumbles)`, `every (issue\|problem)`, `no (other\|further\|major) issues`, `representative`. The bare `exhaustiv` check from `_decomposition.md`'s `## Testing brief` AC-008 row is run as the **floor**, and named as such: on its own it rejects no implementation anyone would write, which is exactly the decorative rule `CLAUDE.md`'s `## The rule that matters` forbids. The plausible wrong implementation this row rejects is a summary asserting "no major issues found" — absence of a stumble in one walk read as evidence of absence |
| **AC-004** | **GIVEN** exactly one reader walked the material exactly once, **WHEN** any summary of this evidence describes that walk, **THEN** it speaks of that one reader and never of "users" or "readers" in general, presents counts as counts and never as a rate or a percentage, and issues no verdict on the documentation as a whole | Static, `rg -i` for the plural slide (`users`, `readers`, `a developer coming to this cold`, `typical (reader\|user\|developer)`, `most (readers\|users)`, `readers (generally\|tend\|will)`), the rate (`%`, `success rate`, `pass rate`) and the verdict (`validated`, `broadly usable`, `tested well`, `signed off as usable`) — each surviving hit reviewer-adjudicated against the disclaiming sentence and ledger-cited. Rejects the risk table's *likely* failure verbatim (`project.md`, `## Risks and coupling notes`, row 2): one reader becoming "users" and seven stumbles becoming a quality rate on the way to promotion. **A phrase inside a reader's own quoted words in an `FL-###` entry is not a summary** and is out of scope here — see EC-005 |
| **AC-005** | **GIVEN** HS-P0025 must decide whether this evidence licenses anything at all, **WHEN** they read the scope statement at any of the five locations, **THEN** the positive half is there with its basis — each recorded stumble is a real stumble a real reader hit, traceable to an `FL-###` id, a named tree SHA and a dated session, already worth fixing without needing to recur — so the scoping reads as a **licence with a boundary**, not as "weak evidence, discount it" | Artifact-evidence, ledger-cited `file:line` at the sentence, plus static presence of the traceability triple (an `FL-###` reference, the tree identifier from `## Session record`, and the session date) reachable from the claim. Checked against `_grounding.md:72-76` (Nielsen & Landauer 1993 — ~1/3 of problems, and each problem a real reader hits already proven). Rejects the hedge-only summary, which is the *other* way to make one expensive session worthless and is forbidden by the initiative's own framing that the gate and the comprehension record "falsify different things and neither may stand in for the other" (`project.md`, `## How this advances the initiative`) |
| **AC-006** | **GIVEN** the session recorded **ineligible-but-used-anyway**, or ended in `Kind: abandonment`, **WHEN** U3 reads any single one of the five locations and no other, **THEN** that qualification is stated there too — the artefact is unmet, or the walk was partial and stopped where it stopped — and **GIVEN** neither occurred, no location invents a qualification that the session did not record | Conditional static: if `rg -n "ineligible-but-used-anyway" <log>` hits inside `## Session record`, then the unmet statement is present at each of the five locations; if `rg -n "Kind: abandonment" <log>` hits, then the partial-walk statement is present at each. Both directions ledger-cited. Rejects the failure `session-run-against-pinned-tree`'s EC-002 names in this story's own direction — *"downstream summaries inherit that statement; the hand-off note may not quietly drop it"* — and rejects its mirror, a qualification manufactured at summary time that the session record does not carry |
| **AC-007** | **GIVEN** a reviewer deletes every derived copy of the claim from the log and the project card, **WHEN** they re-read what is left, **THEN** the claim is still stated in full at `## Scope of the claim` and nothing has been lost — because each copy named that section as its source of record before it was deleted — and **GIVEN** HS-P0025 wants the string, they lift it in one hop as plain text: browser find and stable heading anchors, no fold, no widget, no script, no rendering step | The deletion check stated verbatim in `_decomposition.md`'s IQ-2, applied to prose: in a scratch copy delete `## Status`'s claim line, the index preamble line, the hand-off slot line and the Companions row, and confirm `## Scope of the claim` still carries the whole claim including its positive half and any inherited qualification. Plus static: each copy carries the source-of-record label; `rg -n "<details>\|<summary>\|<script>" <log>` returns nothing; `rg -n "^## " <log>` returns the skeleton's eight headings unchanged and in order. Rejects a claim whose only complete statement is a derived view, and a "clearer" restatement in the hand-off slot that quietly becomes the real home of the claim |

**Coverage of the traced project AC.** All seven serve `project.md` **AC-008**, and
specifically the half `_storymap.md`'s `## Coverage` assigns here — *"the sentence and its
presence in the log and its summary"*. The other half, the hand-off note carrying the same
sentence into what HS-P0025 reads, is `handoff-note-to-closeout`'s and is not claimed by
any row above; AC-002's fifth location is the *slot* the skeleton reserved, not the note's
persona, date or tree. AC-006 consumes project AC-003's recorded verdict and AC-010's input
without claiming either: it inherits states, it does not judge them.

## Interaction quality

RFC §6.7/D6. **Every invariant below is already an `AC-###` row in the table above** — this
section says only *which* row carries it and how the row is checked, and introduces
nothing new. That placement is deliberate and mechanical: `redkiln verify` extracts ACs by
matching a leading `| AC-001 |` table cell or an `- AC-001:` bullet, so an invariant stated
only as a prose bullet here would carry no ledger row, be gated by nothing, and be tested
by nobody.

**Where the composition family comes from, given `hasSurface: false`.** `_design.md` is
signed off and declares no surface id — its `## Items` block is an empty `yaml` fence. What
it *does* bind, in that section's prose, is the **primitive layer**: *"there is no CSS
layer and no token file, and inventing one is out of scope"*, and everything these
artifacts need is drawn from named repository document primitives that already exist. The
composition invariants below are taken from that clause, from the UX brief's
`#### The primitive layer to compose from — do not hand-roll` and
`#### The accessibility floor`, and from the section shape `friction-log-skeleton` fixed.
None of them is re-decided here.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump (IQ-1)** — U3 reading a summary point learns the claim *there*; the hop to `## Scope of the claim` is one hop and is optional | **AC-002**, **AC-007** | Each of the five locations carries the claim itself, not a pointer to it. Static: the `rg -F` presence check returns the string, not a cross-reference, inside each section |
| **Non-occlusion (IQ-2)** — a derived view never becomes the only home of what it derives | **AC-007** | The deletion check: remove all four copies from a scratch copy; the claim survives in full at its source |
| **Preserved position (IQ-3)** — nothing already written moves: the eight headings, the `FL-###` ids and their frozen heading lines, and every citation a sibling has already made into the log | **AC-001**, **AC-007** | `rg -n "^## " <log>` returns the eight headings unchanged and in order; `git diff` shows zero hunks under `## Chronological record` and no renumbered or re-headed entry |
| **Reversibility (IQ-4)** — a correction to the canonical string appends beside the original with its reason; the original stays legible | **AC-001** | `git log -p --follow` shows no deletion inside `## Scope of the claim`; an amendment, if any, is an added dated block naming what it replaces. Authority: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **Preserved selection, in this medium** — the reader's own quoted words in an `FL-###` entry are never edited to be more careful; the record of what someone said is not a summary | **AC-004** | The negative sweep's hits inside `Kind: reaction` / quoted spans are adjudicated and left as recorded (EC-005); `git diff` shows no entry-body hunk |
| **Keyboard reachability** — the claim is reachable with the medium's own affordances only | **AC-007** | Browser find and stable heading anchors; `rg` for `<details>` / `<summary>` / `<script>` returns nothing |
| **Abandonment is a valid end state (IQ-6)** — a partial walk is evidence of a shorter walk, never a void whose summary omits the fact | **AC-006** | The conditional inheritance check on `Kind: abandonment` |
| **Non-disturbance of work in flight** — the index preamble is written above rows that do not exist yet, and the disposition stories append beneath it without rewriting it | **AC-002**, **AC-007** | The preamble line sits above the table; `_storymap.md`'s deliberate non-edge means an empty index is expected, not a reason to wait |

### Composition invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every copy is a composed line in its section's existing shape: the `## Status` banner line, the index's derived-from preamble line, the hand-off's named slot, the `## Companions` list-item row — never a bare paragraph dropped under a heading and never raw markup | **AC-002** | Each location's line matches the shape already used there; the Companions row keeps the existing `- [`file`](path) — description` list style |
| **Composition and placement** — the log keeps the skeleton's eight sections in their fixed order; this story adds text inside four of them and adds, renames and reorders none | **AC-002**, **AC-007** | `rg -n "^## " <log>` returns the eight headings in order; the diff adds no heading |
| **Transience — persistent chrome vs revealed vs opened-on-demand** | **AC-002**, **AC-007** | *Persistent chrome*: all five statements are visible by default at all times — the `## Status` line, the index preamble, `## Scope of the claim`, the hand-off slot, the Companions row. *Revealed*: nothing. *Opened on demand*: nothing — no fold, no collapsed admonition, no inactive tab, no external file required to read the claim |
| **Density budget, with its real numbers** | **AC-002**, **AC-005**, **AC-006** | The canonical string is exactly as committed, unedited — its length is the skeleton's, not this story's, to set. Each derived copy is **one line** plus its source-of-record label. The `## Companions` row is **exactly one list item**. An inherited qualification adds **at most one clause** per location, not a second paragraph. Any checkbox this story writes is **one line, never wrapped** — the gate parser matches line by line and a wrapped box can never match |
| **Hierarchy** — `## Scope of the claim` is the source of record and every copy says so; the chronological record stays authoritative over both | **AC-007** | Each copy carries the label; the index preamble still states in its own first line that the index is derived and the record is authoritative |
| **Anti-pattern — colour or emoji as the sole carrier** of the qualification or the claim | **AC-002** | Plain-text read of `git show HEAD:<log>`: every statement is words |
| **Anti-pattern — a load-bearing item behind a fold, an inactive tab or a collapsed admonition** with no visible-by-default counterpart (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`) | **AC-007** | The deletion check plus the fold/widget scan |
| **Anti-pattern — a bespoke navigation widget over a medium that renders the equivalent for free**, a new heading vocabulary, a CSS layer or a token file | **AC-007** | The diff introduces no script, no widget, no style file, no new section heading |
| **Anti-pattern — a paraphrase presented as the claim.** Four sentences that agree today disagree after the next edit and nothing reports it | **AC-002** | Byte-identity, not semantic agreement |
| **Anti-pattern — the hedge-only summary.** A statement carrying the limit and not the licence, which invites downstream to discount evidence the initiative says cannot be substituted for | **AC-005** | The positive half and its traceability triple are present at the claim |
| **Anti-pattern — the scaffold row left standing.** `## Companions` still describing the log as a scaffold after the session ran is a false one-line summary on the only surface reached from `redkiln board` | **AC-002** | `rg -n "scaffold" project.md` returns nothing in the log's row |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The canonical string genuinely lacks a half of Decision 3 — most plausibly the positive licence, since a pre-written scope sentence is written defensively | Correct it as a **visible additive amendment** dated and naming the original beside it, with the reason. Never an in-place overwrite: the pre-session property is the whole basis of AC-001, and a silent rewrite makes project AC-008 unfalsifiable. The amendment is itself part of the diff a reviewer reads |
| **EC-002** | `## Status` still says no session has been run, or `## Chronological record` holds no `FL-###` entry | **This story does not start.** Its one dependency is `session-run-against-pinned-tree` and there is no claim to scope about a session that did not happen. Record the blocked state in this story's own folder and stop; do not write a scope sentence for a hypothetical walk |
| **EC-003** | `## Session record` resolves the reader as **ineligible-but-used-anyway** | AC-006 governs: every one of the five locations states that the artefact is **unmet** — a failure of the initiative, not a reason to redefine the bar (`project.md`, risk table, row 1). This story has no discretion to soften it, and the softening is exactly what `session-run-against-pinned-tree`'s EC-002 forbids downstream |
| **EC-004** | The session ended in `Kind: abandonment`, possibly minutes in | Not an error and not a void (IQ-6). Every location states that the walk was partial and where it stopped. The evidence is real and smaller than the scenario described; whether a second session follows is `second-session-decision`'s recorded verdict, and this story does not pre-empt it |
| **EC-005** | The negative sweep hits an overclaiming phrase **inside a reader's own quoted words** in an `FL-###` entry — "I guess it's all fine then" | Leave it exactly as recorded. The record of what someone said is not a summary and is never edited to be more careful; editing it would breach IQ-3 and the append-only discipline, and would corrupt the one thing the session was run to capture. If the phrase is likely to be quoted onward as a verdict, note that in this story's own notes for `route-and-escalate`, which owns destinations |
| **EC-006** | The sweep finds an overclaim **outside the PR boundary** — a sibling project's risk table, a review artifact, an initiative-level summary | Do not edit it. The `## PR boundary` fenced block is read by `redkiln verify --grain story` and a file changed outside it fails the story grain. Record the finding and hand it to `route-and-escalate` with a destination id; a boundary widened at gate time to accommodate a file already written is the failure the boundary exists to prevent |
| **EC-007** | `## Dispositions index` is empty or partial when this story writes its preamble line | Expected, and not a reason to wait. `_storymap.md`'s dependency graph deliberately gives this story no edge to `disposition-every-stumble` — *"a fix that lands late must not be able to hold up the evidence HS-P0025 needs"*. The line goes in the preamble, above the table; the disposition stories append rows beneath it and rewrite neither it nor each other |
| **EC-008** | `_design.md`'s protocol section binds the log to a path other than `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` | `_design.md` wins. Substitute the path verbatim everywhere in this spec and widen the `## PR boundary` fenced block **here, deliberately, before the work** — never after a file has been written outside it |
| **EC-009** | A **sixth** summary point turns out to exist — a project review artifact, the body of a routing message, an initiative rollup that quotes this evidence | Add it to Decision 2's enumerated set in this spec, deliberately, before the PR, and give it a copy with its source-of-record label. The set being closed is what makes "wherever it is made" checkable; discovering the sixth at gate time converts the criterion back into reviewer vigilance |
| **EC-010** | `handoff-note-to-closeout`, writing later in the same slice, judges the canonical string too weak or too long and wants to restate it | It may not paraphrase, strengthen or trim it unilaterally — that would make the hand-off slot the real home of the claim and defeat AC-007's hierarchy. A genuine defect reopens *this* story and is corrected here as EC-001's visible amendment, which the note then carries unchanged |
| **EC-011** | An edit to `project.md` is about to touch frontmatter — `updated`, `links`, `status` | Forbidden, and a `PreToolUse` hook denies it. The CLI is the single writer of item system fields; this story edits body prose in `## Companions` only (`CLAUDE.md`, `## Where the work lives`) |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | The claim is **legible without opening another file**: each of the five locations carries the sentence itself, not a reference to where the sentence lives | A pointer is not a claim. The failure this project's risk table calls *likely* happens at exactly the moment someone summarises without opening the log, and a cross-reference does nothing at that moment |
| **NF-002** | The claim is readable with **no tool, script, rendering step or network access** — plain markdown, browser find, stable heading anchors, and a `git diff` view with no colour at all | `_decomposition.md`, `#### The accessibility floor`; UX-AC-004 and UX-AC-013. AC-007 states it as a criterion; NF-002 states it as a standing property for every later reader |
| **NF-003** | The story introduces **no new dependency, tool, format, convention, heading, CSS layer, token file or widget** | `_design.md`, `## Items`: inventing a primitive layer is out of scope. The primitives are the named existing files, and the log's eight sections are `friction-log-skeleton`'s |
| **NF-004** | Repository hygiene is unchanged: `redkiln validate --kb && redkiln doctor` clean at **exactly** the six standing `template-drift` advisories, no more and no fewer | `CLAUDE.md`, `## Where the work lives` — a seventh is a template changed without deciding to, a missing one is a customisation reverted. This story changes no template and must move neither number |
| **NF-005** | The diff is **additive with respect to the record**: zero hunks under `## Chronological record`, no `FL-###` id renumbered, no heading line re-worded | IQ-3. A sibling project that has already cited `FL-004` must land on the item it cited; a broken anchor here produces no error message anywhere |
| **NF-006** | The canonical string is **quotable as one contiguous span** — copy it out of the hand-off slot and it pastes as the claim, with no footnote markers, no intra-doc link syntax and no reflowing artefacts | UX-AC-013's bar: HS-P0025 lifts it *without re-deriving anything*. A sentence broken by markup is a sentence that gets retyped, and a retyped sentence is a paraphrase |
| **NF-007** | Nothing this story writes adds a personal detail beyond what the reader's declaration already required — no name beyond the recorded identity, no credential, no private path | The log is committed to a repository that will be public, and the summary points are the surfaces most likely to be read out of context |
| **NF-008** | The evidence is legible to a stranger six months out: someone who has never met the logger can read any one of the five locations and know exactly what this evidence does and does not support | The UX brief's stated bar for U3, and the reason the positive half is required alongside the limit |

## Implementation notes (non-prescriptive)

Not instructions — the reasoning behind the shape, so the implementer can make the hundred
small calls the criteria do not reach.

**Read the string before planning the edit, and adopt it rather than improve it.** The
strongest pull in this story is to tighten a sentence written before anyone knew what the
session would find. Resist it. If the sentence is genuinely defective, EC-001's visible
amendment is the legal outlet, and it costs three lines; an in-place improvement costs the
one property that makes the sentence worth propagating.

**Run the negative sweep before writing anything, and again after.** Before, because it
tells you what you are actually defending against in *this* log — the vocabulary in
Decision 4 is general and the real hits are specific. After, because the four copies and
the Companions row are themselves new prose and are as capable of overclaiming as anything
they replace.

**Put the sweep in the PR as a real invocation, not as a claim that it was run.** The
`rg` command with its full alternation is evidence a reviewer can re-run; "I checked" is
not. This is the same reason the ledger takes a `file:line` rather than a tick.

**Keep the source-of-record label in the same sentence as the copy.** A label on its own
line reads as chrome and gets deleted by the next person tidying the section; a label in
the sentence travels with the claim when someone copies the line out.

**Write the `## Companions` row last.** It is the only edit outside the log and the only
one that a `redkiln board` reader meets first, so it should be written once you know
exactly what the log now says — including any inherited qualification. Body prose only,
and the row is one list item in the existing style.

**Prefer one clause to a second sentence for an inherited qualification.** "…and the walk
was partial, ending at FL-004" attached to the claim survives copying; a separate sentence
beneath it gets left behind at the first summary.

**If the sweep tempts you to edit an `FL-###` entry, stop.** That is EC-005, and it is the
one unrecoverable edit available in this PR. The record is append-only and frozen at write
time; the entry that reads badly is a finding for `route-and-escalate`, not a sentence to
soften.

**The index preamble goes above the table, and says nothing about rows.** It will be
written while the table is empty or half-filled. A preamble that describes the rows will be
wrong within a slice; a preamble that states the claim and the source of record stays true
however many rows arrive beneath it.

**Assume the reader of the Companions row will never open the log.** That is not a
pessimistic assumption; it is the described behaviour of U3 in the UX brief's own table —
*"without reading the whole thing"* — and it is the whole reason this story exists.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, `## Testing brief` — its AC/tier table and its
content-fix tier — and in `.redkiln/config.yaml`'s wired grains. `<log>` is
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` (or the path `_design.md`
fixed, per `## Integration contract`). The brief's warning governs every row below: a check
that would pass `"noted"` as readily as a real value rejects no wrong implementation, so
every static check here asserts on **content**, not on structural presence.

| tier | command / path | proves |
| --- | --- | --- |
| **Provenance (static)** | `git log -p --follow -- <log>` across this story's commits, inspecting the `## Scope of the claim` hunks | AC-001 — the string is the pre-session one; any change inside that section is an append with no deletion. This is the check that makes project AC-008 falsifiable rather than assertable |
| **Provenance (static)** | `git log --format=%aI` on `friction-log-skeleton`'s commit, compared against the session date in `## Session record` | AC-001 — the sentence predates the findings. The same date comparison `_decomposition.md`'s `## Testing brief` AC-002 row specifies for `_design.md`, applied to the one sentence most tempting to tune |
| **Presence (static, `rg -F`)** | `rg -F -n "<canonical string>" <log> .bklg/docs-that-teach/comprehension-evidence/project.md` | AC-002 — a hit inside each of the five enumerated sections. A fixed-string search, not a regex: a paraphrase must not be able to satisfy it |
| **Byte-identity (static)** | Extract the five spans and compare them pairwise (`diff`) | AC-002 — divergence is a diff, not a matter of interpretation. Rejects four agreeing paraphrases |
| **Negative sweep (static, `rg -i`)** | The Decision 4 vocabulary over `<log>` and `project.md`'s `## Companions`, run as one alternation and recorded in the PR verbatim; `rg -i "exhaustiv"` run separately as the named floor | AC-003, AC-004 — no exhaustiveness claim, no plural slide, no rate, no verdict. Every surviving hit is adjudicated against the disclaiming sentence and ledger-cited |
| **Conditional inheritance (static)** | `rg -n "ineligible-but-used-anyway" <log>` and `rg -n "Kind: abandonment" <log>`; where either hits, confirm the corresponding statement at each of the five locations | AC-006 — the qualification is inherited, not laundered; and where neither hits, none was invented |
| **Deletion check (static)** | In a scratch copy, delete the `## Status` claim line, the index preamble line, the hand-off slot line and the Companions row; re-read `## Scope of the claim` | AC-007 / IQ-2 — the claim, its positive half and any qualification survive in full at the source. `_decomposition.md`'s own stated method, applied verbatim to prose |
| **Plain-text legibility (static)** | `git show HEAD:<log>` read with all styling stripped; `rg -n "<details>\|<summary>\|<script>" <log>` | AC-007, NF-002 — no fold, no widget, no script; a screen reader and a `git diff` both read the claim |
| **Structure untouched (static)** | `rg -n "^## " <log>` (the eight headings, in order); `git diff --stat` scoped to `## Chronological record`; `rg -n "^### FL-" <log>` before and after | AC-001, AC-007, NF-005 — no section added, renamed or reordered; zero hunks in the record; no id renumbered |
| **Frontmatter untouched (static)** | `git diff` on `project.md` shows no hunk above the closing `---` | EC-011 — the CLI is the single writer of item system fields, and the `PreToolUse` hook is the belt to this check's braces |
| **Artifact-evidence (ledger)** | `.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/_ledger.md`, enforced by `redkiln verify --grain story` with `require_ledger: true` (`.redkiln/config.yaml:67`) | Every AC row carries a real `file:line` into the log or the project card. This is the tier that carries AC-004's adjudication and AC-005's basis, both of which are genuinely reviewer-read and which the brief forbids over-automating |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The repository gate for this diff. Touching no crate, it falls through to the five file-reading lints and `spec-trace` unconditionally rather than passing vacuously on an empty package set |
| **Integration grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7's non-terminal bar. Expected green and unchanged — this story alters no crate, so a failure here is the signal that the PR boundary leaked |
| **Backlog hygiene** | `redkiln validate --kb && redkiln doctor` | NF-004 — clean at exactly the six standing `template-drift` advisories, and no `.kb/` atom authored |
| **Explicitly not run** | `cargo xtask ci` (`.redkiln/config.yaml:60`, `e2e`) | The terminal bar is HS-P0025's, and DoD-7 says so plainly. A green gate says nothing about whether a claim is honestly scoped — the two instruments falsify different things |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation, inside this PR |
| --- | --- | --- |
| The canonical string is quietly improved with the findings in view, and the pre-session property is destroyed without anyone noticing | Medium / High | AC-001's provenance check is a `git log -p` on the section, not an assurance; EC-001 gives the genuine defect a legal, visible outlet so the improving edit has somewhere to go that is not an overwrite |
| The claim ships only as a hedge, and downstream discounts evidence the initiative says cannot be substituted for | Medium / Medium | AC-005 makes the positive half a criterion with its own basis (`_grounding.md:72-76`) and its own traceability triple. Under-claiming is not the safe direction and the spec says so twice |
| The negative sweep is run as `rg -i "exhaustiv"` alone, passes, and rejects nothing | High / Medium | AC-003 names the floor *as* the floor and carries the wider vocabulary; the PR records the actual alternation so a reviewer can re-run it. This is `CLAUDE.md`'s decorative-rule corollary applied one level up |
| The sentence lands in the log and never reaches the thing people quote — the board row or the hand-off slot | Medium / High | Decision 2's set is closed and enumerated, and AC-002 checks all five with a fixed-string search. The laundering hop is the failure the enumeration exists to make impossible |
| An edit strays into an `FL-###` entry while sweeping for overclaims | Medium / High | EC-005 and NF-005; the `## PR boundary` fenced block plus a `git diff` scoped to `## Chronological record`. The record is append-only and frozen at write time, and this is the one unrecoverable edit available here |
| `disposition-every-stumble` or `route-and-escalate` later rewrites the index preamble while appending rows | Medium / Medium | EC-007 fixes the preamble above the table and states the append-only discipline; `_storymap.md`'s merge order puts this story before the note and independent of the dispositions |
| `handoff-note-to-closeout` restates the claim "more clearly" and becomes its real home | Medium / High | EC-010 forbids it and AC-007's hierarchy check catches it: the note carries the string unchanged, and `_storymap.md`'s `## Coverage` already binds it to do so |
| The session recorded ineligible-but-used-anyway and the qualification is dropped at one of the five points out of tidiness | Low / High | AC-006's conditional check applies at **every** location, not the log as a whole; `session-run-against-pinned-tree`'s EC-002 already forbade the drop in this story's direction |
| The log's home differs from `_friction-log.md` and this spec's paths go stale | Low / Low | `friction-log-skeleton`'s spec binds `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` and mounts it at `project.md`'s `## Companions`, so the two agree today; EC-008 covers divergence and the boundary glob already covers the whole story folder |
| The `## Companions` edit touches frontmatter | Low / Medium | EC-011, the `PreToolUse` hook, and a `git diff` check that no hunk sits above the closing `---` |

**Coupling, stated once.** Upstream, this story is worthless without a session that
happened: `session-run-against-pinned-tree` supplies the findings the claim is *about* and
the two qualifying states the claim must inherit. Downstream, it is a precondition for the
honesty of everything HS-P0025 promotes — `handoff-note-to-closeout` carries this sentence
unchanged into the one input closeout cannot manufacture, and `.kb/product/README.md:24`'s
bar (*"a persona nobody researched is a stock photo with a name"*) is cleared by exactly
one observation, no more. Laterally, this story must not disturb
`disposition-every-stumble` and `route-and-escalate`, which append beneath the preamble it
writes.

## Dependencies

**Blocks on** — `_storymap.md`'s dependency graph gives this story exactly one edge, and
the absence of a second is deliberate:

| Story slug | What it must have landed before this story starts |
| --- | --- |
| `session-run-against-pinned-tree` | A session that actually happened, with `## Status` carrying the date and tree, `## Chronological record` carrying real `FL-###` entries with their severity tokens, and `## Session record` resolving the eligibility verdict and the end state (completion or `Kind: abandonment`). Without it there is no claim to scope and EC-002 halts the story: a scope sentence for a hypothetical walk is the one thing worse than an unscoped one |

Transitively, through that story: `friction-log-skeleton` (which pre-wrote the canonical
string and fixed the eight sections and the `## Companions` row this story rewrites),
`non-insider-recruitment` (whose declaration produces the eligibility state AC-006
inherits), and `dt9-and-fixed-protocol` (whose `_design.md` protocol binds the log's path
and heading vocabulary).

**Explicitly not depended on:** `disposition-every-stumble`. The non-edge is stated in
`_storymap.md`'s `### Dependency graph` — *"a fix that lands late must not be able to hold
up the evidence HS-P0025 needs"* — and EC-007 is its consequence in this story.

**Unlocks** — directly:

| Story slug | What it takes from this story |
| --- | --- |
| `handoff-note-to-closeout` | The canonical string, byte-identical and already labelled with its source of record, together with any inherited qualification. `_storymap.md`'s `## Coverage` splits project AC-008 exactly here: this story owns the sentence and its presence in the log and its summary; the note is bound to carry the same sentence because it is the summary HS-P0025 actually reads |

And transitively, through that note: **HS-P0025 `durable-audience-closeout`**, whose
persona promotion may say precisely as much as this sentence licenses and no more.

`second-session-decision` is a slice-mate, not a dependent: the two may land in either
order (`_storymap.md`, `## Merge order`, step 4). "One session" is a fact this story's
sentence states; whether a second is owed is that story's recorded verdict.

## Anchors (progressive disclosure)

Load-bearing depth, deferred but not optional. The `## Context pack` above is
self-sufficient to begin; open these at the stated moment. **Link, never paste in bulk.**
Every path below was confirmed present in this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | Fixes what this story adopts rather than authors: the eight sections in order, the `## Status` banner as the *anti-miscitation* line, the `## Dispositions index` as derived-and-additive, the `## Hand-off` slots, the `## Companions` row as landed, and — at `:260` — the reason `## Scope of the claim` was pre-written and why that must not be undone | **First, before any edit.** Its `## Behavior and interfaces` table is the section-by-section contract this story writes inside | AC-001, AC-002, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/spec.md` | The upstream story's EC-002 is this story's obligation stated from the other side — *"downstream summaries inherit that statement; the hand-off note may not quietly drop it"* — and its AC set defines the end states (completion, `Kind: abandonment`) and the tree/date fields the positive half's traceability triple points at | Before writing the qualification clause, and whenever the end state is ambiguous | AC-005, AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief's three-user table (U3's *"without reading the whole thing"* is why this story exists), the seven IQ invariants, the accessibility floor in this medium's own terms, the primitive layer that may not be hand-rolled, UX-AC-006 (the deletion check) and UX-AC-013 (liftable without re-deriving), plus the testing brief's AC-008 row and its warning against a check nothing can fail | Open the UX brief when composing a copy or judging a summary point; open the testing brief when deciding how an AC is actually verified | AC-002, AC-003, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | The traced criterion AC-008 verbatim, derived requirement 9 (the small-sample basis supports the narrow claim *only*), risk-table row 1 (ineligible-but-used-anyway means the artefact is unmet) and row 2 (the overclaim at promotion), DoD item 4 that this story takes to green, and the `## Companions` block this story rewrites | When writing the ledger's `criterion` values, and immediately before editing the Companions row | AC-004, AC-005, AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | The verified basis for both halves of the claim at `:72-76` — Nielsen & Landauer 1993: one session finds ~1/3 of problems, **and** each problem a real reader hits is already proven and need not recur to count. Also `## Headline finding`: no Accepted decision atom governs this subject, which is why this spec cites none | Before writing the positive half, and whenever a methodological claim needs its source rather than a paraphrase | AC-003, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | The AC-008 coverage split between this story and the hand-off note, the deliberate non-edge to `disposition-every-stumble` and its stated reason, and the merge order inside `scoped-claim-and-handoff` | When the scope of "wherever it is summarised" feels like it should include the hand-off note's own fields, and when the empty dispositions index invites waiting | AC-002, AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md` | The slice-mate that must carry this string unchanged. Reading it is how you check that the sentence you propagate is one the note can lift verbatim, and how EC-010's boundary is kept from both sides | After the four copies are written, before the slice's final story starts | AC-002, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off design (Ryan Britton, 2026-08-17, no conditions). It declares `hasSurface: false` with an empty `## Items` fence and states the reason, and it binds the **primitive layer** every composition invariant here is drawn from — *"there is no CSS layer and no token file, and inventing one is out of scope"*. It is also where the log's path and heading vocabulary are fixed | Before the first edit, to confirm the bound path (EC-008). **Read-only — amending it retroactively destroys project AC-002's provenance check** | AC-002, AC-007 |
| `.bklg/docs-that-teach/initiative.md` | BR-14 at `:349` in its own words — the claim scoped to real stumbles captured and traceable, never exhaustiveness — and the `## Risks` first row, the two instruments that falsify different things and may not substitute for one another | When judging whether a phrasing over- or under-claims; the charter settles both directions | AC-003, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The `## Risks` section carrying *"None of these three personas has been directly observed by this initiative"* — the exact qualification HS-P0025 will replace, and the reason this story's sentence must constrain how the replacement may be read. **Not to be promoted, renamed or edited — that is HS-P0025's** | Before writing the positive half, to see precisely what one observation is replacing | AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The named anti-patterns this story's composition invariants cite: a load-bearing item behind a fold or a collapsed admonition with no visible-by-default counterpart, and a bespoke widget over a medium that renders the equivalent for free. Also the settled ruling against quizzes, which is not reopenable here | When tempted to add a roll-up, a fold or a "clearer" navigational device to make the claim more prominent | AC-007 |
| `.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md` | The primary evidence behind the scoped claim and behind routing: the small-sample basis, and the finding that a log with no destination is a diary. It is the argument that answers the pull to either overclaim or discount | When a reviewer challenges the claim in either direction and a citation is owed | AC-003, AC-005 |
| `.kb/product/README.md` | `:24` — *"a persona nobody researched is a stock photo with a name"* — the bar this evidence exists to clear, and the rule that an unevidenced sketch stays in `_discovery/` until closeout promotes it. It is why one observation is worth stating positively and why it is worth stating narrowly | When writing the positive half, and before any temptation to describe the observation as more than one person | AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom binding near this story. It is the authority for EC-001's shape: the referent may be rewritten, the reasoning may not be erased — so a defective canonical string is amended beside the original, never over it | The moment the canonical string looks like it wants improving | AC-001 |
| `.redkiln/config.yaml` | The wired gate grains this PR is actually checked by: `affected_gate` (line 40), `integration_scoped` (line 55), `e2e` (line 60, explicitly not this project's), `require_ledger` (line 67), and `support_initiative` (line 5) | When running the merge gate, and when a sweep finding turns out to belong to another owner | AC-003, AC-007 |
| `.redkiln/templates/_ledger.md` | The ledger shape and its rules: planning authors every row `satisfied: false`; the implementer may only flip a row with real `file:line` evidence, and may never re-word a criterion or flip a satisfied row back | When filling `_ledger.md` after the five locations are written | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-007 |

## Clarifications resolved during spec

**The AC set is exactly the seven the front half enumerated.** None added, none dropped.
AC-001…AC-007 above carry the same ids, in the same order, with the same subjects as
`## Behavior and interfaces`' closing paragraph, promoted to full GIVEN/WHEN/THEN criteria
framed from U3's and HS-P0025's intent. The `_ledger.md` carries exactly these seven rows.

**"Wherever it is made" is discharged by a closed list, and that is a deliberate narrowing
of project AC-008's wording.** The project criterion says *"the log and its summary each
state…"*, which as a standard is unfalsifiable — a reviewer cannot check "everywhere".
Decision 2 converts it into five named locations, and EC-009 makes the sixth an explicit,
pre-PR amendment rather than a gate-time discovery. If a reviewer believes a sixth point
already exists, that is a spec change made here, not a judgement made at review.

**Where the composition invariants come from, given `hasSurface: false`.** `_design.md`
declares no surface id and an empty `## Items` fence, so there is no signed-off surface to
bind to in the usual sense. Rather than treat that as an exemption, this spec takes the
composition family from what `_design.md` *does* bind in its `## Items` prose — the named
document primitive layer and the explicit ruling that inventing a CSS or token layer is out
of scope — joined to the UX brief's accessibility floor and the section shapes
`friction-log-skeleton` fixed. Nothing in `## Interaction quality` is a new decision, and
every invariant there is carried by an `AC-###` row in the table above.

**Project AC-003 and AC-010 are consumed, not co-owned.** AC-006 inherits the eligibility
verdict `non-insider-recruitment` recorded and the end state
`session-run-against-pinned-tree` recorded; it neither writes nor re-judges either. The
second-session verdict is `second-session-decision`'s (project AC-010) and this story's
sentence states only the fact that one session was run.

**"Verifying test" means a real check against a real path, not a test file.** This project
has no functions and no test binary, and the testing brief says so directly. Each AC's
verification is a real `git`/`rg`/deletion check plus the ledger's `file:line` discipline —
and per the brief's own warning, every static check asserts on content rather than
structural presence, so none is a check that nothing can fail. The two rows that stay
genuinely reviewer-read, AC-004's adjudication of surviving sweep hits and AC-005's
sufficiency of the stated basis, are named as such rather than dressed up as automated.

**The negative sweep's vocabulary is fixed here and is part of the spec, not the
implementer's discretion.** It is written out in AC-003, AC-004 and the merge-gate table so
that a reviewer re-runs the same alternation the implementer ran. Extending it is welcome;
narrowing it is a spec change.

**The log did not exist in the worktree when this spec was written**, because
`friction-log-skeleton` had not yet been implemented. Every anchor in the table above was
confirmed present; `_friction-log.md` is deliberately absent from it and appears only as
the mount point, which is the correct place for a path this story's dependency creates.
