---
item: HS-S0130
stage: spec
created: 2026-08-12T13:48:09.371Z
updated: 2026-08-12T13:48:09.371Z
template_sig: 87bbf1d0
rendered_sig: d7b6866e
---

# Spec — Zero open-question deletions, every consumed atom resolved and annotated

## Scope lock

| Artefact | Path | What this story takes from it |
| --- | --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` | BR-15 (`:298`), DoD 14 (`:398-401`) and DoD 15 (`:402-404`) — each ends "the corresponding open-question atom reflects that resolution, and `redkiln validate --kb` passes"; exit criteria 7–8 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` | The traceability matrix that assigns the **audit** half of BR-15 to HS-P0019, and the DAG rank that puts this project last |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | **DR-7** (`:155-161`, the six named atoms), **AC-006** (`:211-214`), DoD 3 (`:253-255`), and *Out of scope* "The two silences … this project audits only that they exist and that their open-question atoms are resolved" (`:114-116`) |
| The one warranted brief | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The AC-006 row (`:49`): tier **static / process**, the `--diff-filter=D` instrument, and the six filenames cross-checked against the index |
| Grounding | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | Why the audit tables were shaped but not populatable at planning time; the `0269720` precedent on process-shaped output |
| Signed-off design | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | **No public API surface.** `## Items` is `N/A`; this story renders no surface and adds no Rust item |
| Story map | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | This story's row (`:58`), the `answers-audit` slice rationale (`:78-80`), the grain notes (`:92-99`), and *Where the evidence lands* (`:24-30`) |
| Roadmap pointer | `RUNBOOK.md:262-284` | The ADR queue whose written/reserved numbers the slice-mate enumerates and whose `related:` edges name the questions this initiative consumed |

## One-line PR slice

Prove no open-question atom was deleted — a zero-deletion diff of `.kb/open-questions/` across the
initiative — and that each consumed atom carries a resolution-bearing status and an annotated bullet
in the index.

## Executive summary

This PR lands one new, cited section — **`## Open-question preservation audit`** — appended to the
project's `_closeout-record.md`, immediately after the decision-atom audit table its slice-mate
(`decision-atom-audit-table`, HS-S0129) put there, plus this story's own `_ledger.md`. It writes
nothing under `.kb/`.

**Pointer:** the slice-mate already answers *"did every answer this initiative settled land as an
accepted decision atom that names what lost?"* (project AC-005). **Delta:** this story answers the
other half of BR-15, which is not the same question and is not implied by it — *"did the questions
survive being answered?"* An initiative can produce a perfect decision corpus and still destroy the
record of what was not known on the day each choice was made, by deleting the question atom, by
renaming it, or — the quiet one — by rewriting its body into its own answer. `.kb/open-questions/README.md:40-45`
forbids all three; nothing in the gate checks any of them, because `redkiln validate --kb`'s
immutability check is scoped to `status: accepted` **decision** atoms (`.kb/decisions/README.md:7-13`)
and an `open_question` atom is not one. This story is the check that does not otherwise exist.

The second delta is a posture: this story **measures and routes; it never repairs**. A missing status
flip or a missing index bullet is a finding handed to `findings-disposition-register` (HS-S0134,
which this story `blocks`), not something fixed here.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted anchor.

**1 — The deliverable is preservation, not resolution.** `project.md:114-116` is explicit that for the
two silences "this project audits only that they exist and that their open-question atoms are
resolved." Flipping a question's status is part of *resolving* it, and `.kb/open-questions/README.md:40-45`
assigns that act to whoever wrote the answer — the owning sibling project (HS-P0017, HS-P0018,
HS-P0010 …), on the commit where the answer landed. If this story flips a status to make its own
table green, the audit stops being able to fail: it becomes an instrument that edits the thing it
measures. That is the single most important constraint here, and it is the same instinct
`_storymap.md:92-94` states for the project as a whole ("No story fixes anything").

**2 — A resolution-bearing status is `withdrawn` or `superseded`, and nothing else.** Not a body that
now reads like an answer, not a `related:` link on its own, not an index bullet. The README's
*Resolving one* section (`:42-45`) fixes the vocabulary and adds the constraint that carries the
whole point: *"leave the body describing what was not known at the time. Do not rewrite a question
into its own answer."* Deletion-by-rewrite is a deletion that `--diff-filter=D` reports as clean, so
the audit must inspect the body diff of each consumed atom and not just its survival. Note the
adjacent, already-filed complication: `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md`
records that `KbFrontmatter`'s status enum has no value for some states the corpus needs. If a sibling
resolved a question and could not express it in the enum, that is a **finding with a real precedent**,
not a failure of this audit — record which state it wanted.

**3 — Walk the commit range; do not diff its endpoints, and turn rename detection off.** Two wrong
implementations this rule must reject. (a) `git diff B..HEAD -- .kb/open-questions/` sees only
endpoints: an atom deleted in one commit and re-added with different content in the next appears as
`M`, and one deleted and restored byte-identically appears as nothing at all. The audit therefore
walks every commit in the range (`git log --diff-filter=D --name-status`), and states the range it
walked. (b) With rename detection on, a rename — which is a delete of the path the index and every
`source_paths` reference point at — is reported as `R` and slips past a `D` filter. The audit runs
`--no-renames` so a rename surfaces as a deletion, and separately reports any `R` found with `-M` so
a deliberate rename is visible rather than silently absorbed.

**4 — The consumed set is computed, then reconciled against DR-7's six.** `project.md:155-161` names
six atoms by filename (`cf-40-fixture-limits-ownership.md`, `projection-store-batch-has-no-apply-seam.md`,
`es-38-and-gap-read-rules-are-unowned.md`, `global-versus-per-boundary-visibility-invariant.md`,
`ps-1-states-no-progress-obligation.md`, `human-readable-payload-encoding-on-a-constrained-peer.md`);
all six exist on disk today. That list was written at planning time and is a *prediction*. The audit
computes the actual consumed set from the slice-mate's decision-atom audit table — every question a
decision atom this initiative wrote names in `related:` or answers — and unions it with the six. A
member of the computed set that DR-7 did not predict gets a row and a note; a DR-7 atom that turns
out still genuinely open gets a row saying so, with the reason, and is **not** flipped to make the
prediction true. Nineteen `open_question` atoms exist at the baseline; the six are a subset, and the
audit covers deletions across all nineteen and status across the consumed set.

**5 — The index bullet is half the obligation, and it has a shape.** `.kb/maps/open-questions-index.md:12-13`
carries the standing rule in its own frontmatter `summary` — *"A withdrawn or superseded question
stays listed, annotated, rather than removed"* — and `:136-143` fixes the bullet's shape: status word
first (`Open`, `Withdrawn`, `Superseded`), then the id, then one sentence. So a consumed atom has two
observable obligations, and they can disagree: the frontmatter status and the bullet's status word.
A bullet still reading **Open** above an atom whose frontmatter says `superseded` is exactly the rot
the index exists to prevent, and it is a finding, not a typo to fix in passing.

**6 — Findings route to HS-S0134, with a named destination.** `.redkiln/config.yaml:5` names the
`support` initiative; `project.md` DR-12 (`:179-183`) fixes the three destinations: incidental defect
→ `support`; a genuine cross-project interaction → a new item against the **owning sibling** (the
project that resolved the question, not this one); anything touching a `[FROZEN]` clause → a new
decision atom plus a re-plan. Every non-conformance this audit records carries one of those three.

**7 — Mount: one section in the shared record, not a document of its own.** `_storymap.md:24-30`
decides this for the whole project: the stories that must be read together converge on
`_closeout-record.md`, because *"fourteen ledger entries in fourteen places is the failure mode the
charter's DoD preamble is written against."* The decision table and this preservation audit are read
in one sitting by one reader, so this story appends to that file rather than creating a sibling one.
`_closeout-record.md` is stood up by `clean-checkout-harness` (`_storymap.md:53`) and will already
exist and already carry the slice-mate's table when this story runs (merge order, `_storymap.md:147`).

**8 — The persona is the reader of the closed initiative.** `_storymap.md:16-22` names them: the
reader "who must be able to believe the initiative closed honestly without re-deriving the evidence."
Their journey slice here is narrow and specific — they open `_closeout-record.md`, want to know
whether anything was quietly dropped while the initiative was busy being decisive, and must be able
to follow every claim back to a command they can re-run and an atom path they can open. A row that
says "checked, fine" without the SHA range and the command is a row that fails this persona even when
it is true.

**9 — This story is downstream of a record that may already contradict it.** `dod-set-re-observation-record`
(slice 2, merges first) re-observes DoD 14 and 15, whose second clause is precisely
"the corresponding open-question atom reflects that resolution." If this deeper audit disagrees with
that row, the disagreement is itself a finding routed to HS-S0134 — this story does not edit the
earlier record, and it does not soften its own result to match it.

## Integration contract

- **Archetype**: `capability` (`story.md` frontmatter `archetype: capability`). The user-observable
  slice is a section a reader opens and can act on; there is no substrate here for someone else to
  consume.
- **Slice / milestone**: **`answers-audit`**. Slice-mate: **`decision-atom-audit-table`** (HS-S0129),
  which this story `blocked_by`. The two are implemented in one context and land as one integrated
  surface — `_storymap.md:78-80`: "they share the same 'what did this initiative settle'
  enumeration, and the second is meaningless without the first's list."
- **Mount point**:
  `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — the
  project's convergence artefact (`_storymap.md:24-30`), stood up by `clean-checkout-harness`
  (`_storymap.md:53`). This story appends **one** `## Open-question preservation audit` section
  directly after the slice-mate's decision-atom audit table. A findings-only file in this story's own
  directory would satisfy every check and reach nobody; that is the unmounted failure this contract
  forbids.
- **Wires into** (the real contracts it consumes, by path):
  - `.kb/open-questions/` — the nineteen `open_question` atoms at the baseline, including DR-7's six.
  - `.kb/open-questions/README.md:40-50` — the resolution protocol: new atom, `related` link, status
    to `withdrawn`/`superseded`, body left describing what was not known; and the `.passthrough()`
    note that `tracks` / `resolution_ref` survive validation and must be neither stripped nor invented.
  - `.kb/maps/open-questions-index.md:12-13, 136-143` — the never-removed rule and the bullet shape.
  - `.kb/decisions/README.md:7-13` — the immutability rule, cited here for its **scope**: it binds
    `status: accepted` *decision* atoms, which is why an `open_question` body has no automated guard
    and needs this audit.
  - The slice-mate's decision-atom audit table inside `_closeout-record.md` — the input that turns
    DR-7's predicted six into a computed consumed set.
  - `git` over the initiative commit range — the instrument; and `redkiln validate --kb`, re-run to
    confirm the atoms this audit reports on still validate.
  - `.redkiln/config.yaml:67` (`require_ledger`) and `:73` (`require_commit_provenance`) — this
    story's `_ledger.md` is mandatory and cited, and its commit is recorded via `redkiln record-links --sha`.
- **Renders surfaces**: **none.** `_design.md` declares no public API surface for this project
  (`## Items`: "N/A — no public surface"), and the initiative is `userFacing: false`. This story adds,
  changes and removes zero Rust items.
- **Conformance rule(s)**: none, and deliberately. This behaviour is not adapter-observable: no port,
  no clause and no `suite.rs` rule can express "an atom recording an unanswered question still
  exists." The instruments are `git`, `redkiln validate --kb` and a human-readable table.
- **Clause(s)**: none discharged or amended in `spec/SPECIFICATION.md`. Several consumed atoms *name*
  clauses (ES-38, PS-1, CF-40, WF-11); this story reads those names and changes no clause and no
  maturity marker. A finding that would require moving a `[FROZEN]` marker routes as a new decision
  atom plus a re-plan (DR-12), never as an edit here.
- **Advances DoD scenario**: initiative **DoD 14** (`initiative.md:398-401`) and **DoD 15**
  (`initiative.md:402-404`) — the
  second clause of each ("the corresponding open-question atom reflects that resolution, and
  `redkiln validate --kb` passes") is what this audit observes at closeout. Through project **DoD 3**
  (`project.md:253-255`) with its slice-mate, it closes the **audit half of BR-15**.

## PR boundary

**In this PR**

- A new `## Open-question preservation audit` section appended to the mount point
  `_closeout-record.md`, carrying: the baseline and closeout SHAs and how each was derived; the exact
  commands run; the zero-deletion result over the walked range; the per-atom consumed-set table
  (status, `related` edge, body-preservation verdict, index-bullet verdict); and a findings list with
  one routed destination per non-conformance.
- This story's `_ledger.md` and `implementation-report.md` in its own directory, citing that section
  by path and heading (`.redkiln/config.yaml:67`).
- Raw command transcripts, if kept, as companion notes inside this story's own directory.

**Explicitly not in this PR**

- **Any write under `.kb/`.** No status flip, no `related:` edge, no index bullet, no new atom, no
  README edit. Every one of those is the resolving sibling's act (Context pack decision 1); doing it
  here makes the audit unfalsifiable and, for a new atom, reproduces exactly what `0269720` was
  reverted for (`_grounding.md`, *Accepted decision atoms*).
- **The decision-atom side of the audit** — every-answer-has-an-atom, alternatives-that-lost, and the
  reserved-but-unwritten ADR numbers — is `decision-atom-audit-table` (HS-S0129, project AC-005).
  This story consumes its output; it does not restate or re-derive it.
- **Creating `_closeout-record.md`** — `clean-checkout-harness` owns the file's existence and its
  preamble (`_storymap.md:53`); this story appends one section to it.
- **Routing the findings anywhere** other than into the record's findings list. Opening the `support`
  item or the item against the owning sibling is `findings-disposition-register` (HS-S0134).
- **Any code, crate, spec clause or conformance rule.** The project owns none (`project.md` *Out of
  scope*).

**Boundary as globs** — `redkiln verify --grain story` reads the first fenced block below and fails
on any file changed outside it. Two entries only, and the absence of `.kb/**` is the point:

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/open-question-preservation-audit/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

**Merge DoD one-liner** — the record's new section states a derived baseline SHA, a walked commit
range with zero deletions and zero renames under `.kb/open-questions/`, one row per consumed atom
with its status, body-preservation and index-bullet verdicts, and a routed destination for every
non-conformance; `redkiln validate --kb` is green; and `git diff --stat` for this story shows zero
files touched under `.kb/`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Derive the baseline, do not hard-code it** | The initiative's start commit `B` is the parent of the first commit that added `.bklg/from-contract-to-published-library/`, cross-checked against `git merge-base main <initiative-branch>`. Both derivations must agree; at spec time both resolve to `ce933d8` ("Point every ADR reference at the atom", 2026-08-11), which is stated as the *expected* value and re-derived rather than trusted. The closeout SHA `H` is the one `clean-checkout-harness` recorded for the gate run, so the audit and the gate describe the same tree. | `_storymap.md:53`; `project.md:192-195` (AC-001 records the SHA); the record's harness section |
| **Zero deletions, walked not sampled** | `git log --no-renames --diff-filter=D --name-status B..H -- .kb/open-questions/` produces no output. The walk is over every commit in the range, not a two-point diff, so a delete-then-re-add cannot hide. The command and its empty output are quoted in the record. | `project.md:211-214` (AC-006); brief `_decomposition.md:49` |
| **Zero silent renames** | The same range is re-run with `-M` (`--find-renames`); any `R` status under `.kb/open-questions/` is listed with its old and new path and treated as a deletion of the path every `source_paths` and index link points at, unless the record shows the index and referrers were updated in the same commit. | `.kb/maps/open-questions-index.md:41-134` (every bullet is a relative link into `../open-questions/`) |
| **On-disk parity at the closeout tree** | The set of atom ids present at `B` is a subset of the set present at `H`, checked by listing `.kb/open-questions/*.md` at both revisions (`git ls-tree`) and comparing filenames and frontmatter `id`s. Nineteen atoms exist at spec time; the closeout count is stated, and any increase is listed rather than merely counted. | `project.md` *Context anchors* (`:322`, "the nineteen open questions"); `.kb/open-questions/` |
| **Compute the consumed set** | Consumed = every `open_question` atom named by a decision atom this initiative wrote (from the slice-mate's table, via `related:` / `supersedes:` / the atom's own body), **unioned** with DR-7's six filenames. Members predicted by DR-7 but not actually consumed, and members consumed but not predicted, are each listed with a reason. The set is stated before any per-atom row, so the table's completeness is checkable. | `project.md:155-161` (DR-7); the slice-mate's table in `_closeout-record.md`; `RUNBOOK.md:262-284` |
| **Resolution-bearing status per consumed atom** | Each consumed atom's frontmatter `status` is read at `H` and must be `withdrawn` or `superseded`, with `related` naming the answering atom. Anything else — including a still-`accepted` atom whose question was in fact answered, or a state the enum cannot express — is recorded verbatim as a finding with its destination, never corrected. | `.kb/open-questions/README.md:42-45`; `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md` |
| **Body preservation (deletion-by-rewrite)** | For each consumed atom, `git diff B..H -- <atom>` is inspected: changes confined to frontmatter (`status`, `superseded_by`, `related`, `last_reviewed`) pass; any change to the prose body — in particular one that turns "what is not decided" into the answer — is a finding. This check exists because no tool performs it: `validate --kb`'s immutability check binds accepted **decision** atoms only. | `.kb/open-questions/README.md:44-45`; `.kb/decisions/README.md:7-13` |
| **Index bullet, annotated and agreeing** | Each consumed atom has a bullet in `.kb/maps/open-questions-index.md` that (a) still exists, (b) leads with a status word, and (c) whose status word matches the atom's frontmatter status. Missing bullet, missing annotation, and status disagreement are three distinct findings and are reported as such. | `.kb/maps/open-questions-index.md:12-13, 136-143` |
| **The audit repairs nothing** | `git diff --name-only` for this story's commits shows zero paths under `.kb/`, enforced by the PR-boundary block above and asserted explicitly in the record. Every non-conformance carries one of DR-12's three destinations and is handed to `findings-disposition-register` (HS-S0134). | `project.md:179-183` (DR-12); `.redkiln/config.yaml:5`; `_storymap.md:92-94` |
| **Validation still green** | `redkiln validate --kb` is re-run on the closeout tree after the audit and exits zero, so the atoms this audit reports on are reported as they validate. A failure here is a finding about the KB, not a licence to edit an atom into shape. | `.redkiln/config.yaml:56-60`; `project.md:229-232` (AC-011's instrument, re-used read-only here) |
| **Mounted, cited, one section** | The section lands in `_closeout-record.md` under `## Open-question preservation audit`, immediately after the slice-mate's table, and this story's `_ledger.md` cites it by path and heading per criterion. | `_storymap.md:24-30`; `.redkiln/config.yaml:67`, `:73` |

## Data and migrations

**N/A — no schema, no migration, no persisted state.** This story writes no code and no database
lives anywhere in its path. The nearest thing to "data" it touches is the `.kb/` atom corpus, and it
touches it **read-only by design**: the frontmatter it reads (`status`, `related`, `superseded_by`,
`source_paths`) is `KbFrontmatter` owned and validated by `redkiln validate --kb`, and every write to
it belongs to the sibling project that resolved the question (Context pack decision 1). The two
outputs are markdown — a section in `_closeout-record.md` and this story's `_ledger.md`.

One shape decision worth stating so it is not re-litigated during implementation: the per-atom rows
are a **markdown table inside the shared record**, not a generated JSON or a script committed to
`xtask/`. A checker would be the better artefact if this ran repeatedly; it runs once, at closeout,
and `project.md` *Out of scope* gives this project no code (`_design.md`: "No public API surface").
The commands are therefore quoted in the record so the reader can re-run them by hand — which is what
the persona in Context pack decision 8 actually needs.

## Acceptance criteria

Eight criteria, each written from the intent of the reader `_storymap.md:16-22` names — the person
"who must be able to believe the initiative closed honestly without re-deriving the evidence" — and
each crossing the full stack this story has: a command run on the closeout tree, an observation, and a
row in `_closeout-record.md` a reader can follow back. Project **AC-006** (`project.md:211-214`) is
covered by AC-002/AC-003 (the deletion half) and AC-004…AC-007 (the resolution-and-annotation half);
AC-001 and AC-008 make both halves reproducible and mounted. Nothing below is proven by a new
test-framework assertion, because this story adds no code (`_design.md`: "N/A — no public surface").

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the reader will not take "nothing was deleted" on trust and must be able to re-run it, **WHEN** they read the opening of `## Open-question preservation audit`, **THEN** the section states the baseline SHA `B` **derived two independent ways and shown to agree** — the parent of the first commit that added `.bklg/from-contract-to-published-library/`, and `git merge-base main <initiative-branch>` — with both commands and both outputs quoted, `ce933d8` named as the *expected* value that was re-derived rather than copied from this spec; and the closeout SHA `H` stated as **full** SHAs (never a branch name or abbreviation) and shown to be the same commit `clean-checkout-harness` recorded for the gate run, **SO THAT** every later claim in the section is scoped to a range the reader can resolve after the branch is deleted. | Static/process, reviewed in the PR diff: both derivation commands present with output; `git cat-file -e <B>` and `git cat-file -e <H>` resolve; `H` string-equal to the SHA in the record's clean-checkout/gate section; no abbreviated SHA in the section. |
| AC-002 | **GIVEN** an atom deleted in one commit and re-added in the next is invisible to a two-point diff — it renders as `M`, or as nothing at all if the re-add is byte-identical — **WHEN** the reader looks for the deletion evidence, **THEN** the record quotes `git log --no-renames --diff-filter=D --name-status <B>..<H> -- .kb/open-questions/` **verbatim with its empty output**, states the number of commits the walk covered (from `git rev-list --count <B>..<H>`), and says in words that this is a walk over every commit in the range rather than a diff of its endpoints, **SO THAT** "nothing was deleted" is a property of the whole history and not of the two commits someone happened to compare. | Static/process: the quoted command contains `log`, `--diff-filter=D` and `--no-renames` (a quoted `git diff` form fails this AC); output shown empty; commit count present and re-derivable; brief `_decomposition.md:49` is the tier. |
| AC-003 | **GIVEN** a rename is a deletion of the path every index bullet and `source_paths` entry points at, and rename detection makes it report as `R` instead of `D`, **WHEN** the reader checks that nothing left the directory under another name, **THEN** the same range is re-run with `-M` and every `R` under `.kb/open-questions/` is listed with old path → new path and either shown to have had its index bullet and referrers updated in the same commit (recorded as a deliberate rename, listed, not a finding) or routed as a finding; **and** the atom sets at `B` and at `H` are listed by filename and frontmatter `id` from `git ls-tree`, showing the baseline **nineteen** all present at `H` and naming — not merely counting — any atom the initiative added, **SO THAT** the directory is shown to have only grown, and disappearance-by-rename cannot be absorbed silently. | Static/process: both `git ls-tree -r --name-only <B> -- .kb/open-questions/` and the `<H>` form quoted with output; set difference computed in the record, baseline count stated as 19 excluding `README.md`; the `-M` re-run quoted; every `R` row carries both paths. |
| AC-004 | **GIVEN** DR-7's six filenames (`project.md:155-161`) were written at planning time and are a **prediction**, not an inventory, **WHEN** the reader asks whether the per-atom table is complete, **THEN** the section states the consumed set **before its first row** and shows how it was computed — every `open_question` atom that a decision atom this initiative wrote names in `related:` / `supersedes:` / its body, taken from the slice-mate's decision-atom audit table, **unioned** with DR-7's six — and lists every asymmetry with a reason: an atom consumed but not predicted gets a row and a note, and a DR-7 atom that turns out **still genuinely open** is recorded as still open with the reason and is **not** flipped to make the prediction true, **SO THAT** the table's coverage is checkable rather than asserted, and the plan is allowed to have been wrong. | Static/process: the consumed set enumerated as a list of paths ahead of the table; each member traceable to either the slice-mate's table row or DR-7; both asymmetry directions explicitly addressed (an empty one stated as empty, not omitted); no `.kb/` path in this story's diff (see AC-008). |
| AC-005 | **GIVEN** "resolved" has a fixed vocabulary and an owner — `.kb/open-questions/README.md:42-45` assigns the status flip to whoever wrote the answer, on the commit where it landed — **WHEN** the reader reads a per-atom row, **THEN** it shows that atom's frontmatter `status` **read at `H`** as `withdrawn` or `superseded` together with the `related` edge naming the answering atom; and anything else — a question this initiative answered whose atom still reads `open`, or a resolution state `KbFrontmatter`'s enum cannot express (the precedent is already filed at `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md`, and the row records **which state it wanted**) — is written down verbatim as a finding carrying one of DR-12's three destinations (`project.md:179-183`) and is left **uncorrected in this PR**, **SO THAT** the audit is able to fail, which an instrument that edits what it measures cannot. | Static: each row's status value matched against the atom's frontmatter at `H` (`git show <H>:<path>`); each non-conforming row carries a destination drawn from the closed three-value set; `git diff --name-only` for this story shows no `.kb/` path (shared with AC-008). |
| AC-006 | **GIVEN** the one deletion that `--diff-filter=D` reports as clean is the atom rewritten into its own answer — forbidden by `.kb/open-questions/README.md:44-45` ("leave the body describing what was not known at the time") and guarded by nothing, because `validate --kb`'s immutability check binds `status: accepted` **decision** atoms only (`.kb/decisions/README.md:7-13`) — **WHEN** the reader asks whether the surviving atoms still say what was not known, **THEN** each consumed atom's `git diff <B>..<H> -- <path>` is inspected and classified: a change confined to frontmatter (`status`, `superseded_by`, `related`, `last_reviewed`) passes; any change to the prose body is **quoted** in the record and routed as a finding; and the passthrough keys `tracks` / `resolution_ref` are reported as neither stripped nor invented (`README.md:47-50`), **SO THAT** the record of the state of knowledge on the day each choice was made survives the initiative that made the choices. | Static: one body-preservation verdict per consumed atom with the diff hunk quoted where the verdict is anything but "frontmatter only"; the four permitted frontmatter keys named in the record; `redkiln validate --kb` green (AC-008) so the reported frontmatter is the validated frontmatter. |
| AC-007 | **GIVEN** `.kb/maps/open-questions-index.md:12-13` makes "a withdrawn or superseded question stays listed, annotated, rather than removed" a standing rule, and `:136-143` fixes the bullet's shape as status word first (`Open`, `Withdrawn`, `Superseded`), then the id, then one sentence, **WHEN** the reader cross-checks the index against the atoms, **THEN** each consumed atom's row carries **three separately observed verdicts** — the bullet still exists; it leads with a status word from that closed set; and that word **equals** the atom's frontmatter status at `H` — with the three failure modes (missing bullet, unannotated bullet, status disagreement) reported as three distinct findings each with its own destination, and a bullet still reading **Open** above an atom whose frontmatter says `superseded` called out as the specific rot the index exists to prevent rather than fixed in passing, **SO THAT** "not in this index" keeps meaning "never asked" and "Open" keeps meaning open. | Static: three columns per consumed atom in the record, each independently observable; every status word matched against the closed three-value set at `.kb/maps/open-questions-index.md:141`; disagreements listed as findings, and the index file absent from this story's diff. |
| AC-008 | **GIVEN** "fourteen ledger entries in fourteen places is the failure mode the charter's DoD preamble is written against" (`_storymap.md:24-30`), and given that an instrument which repairs what it measures can no longer report a failure, **WHEN** the reader opens `_closeout-record.md` at `H`, **THEN** exactly **one** new `## Open-question preservation audit` section is present — at the same heading level as, and immediately after, the slice-mate's decision-atom audit table, with that table's rows unmoved, unreworded and un-truncated, and no sibling findings file anywhere in this story's directory standing in for it — every claim in it carrying the command that produced it and both SHAs; **and** `redkiln validate --kb` is re-run on the closeout tree and exits zero, **and** `git diff --name-only` across this story's commits shows **zero paths under `.kb/`**, with this story's `_ledger.md` citing the section by path *and* heading, **SO THAT** the reader gets one sitting and the audit's own diff is the proof of its read-only posture. | Static/process: reviewed diff of `_closeout-record.md` — one added `##` section, position after the slice-mate's table, slice-mate's rows byte-identical; `redkiln validate --kb` exit 0 recorded; `git diff --name-only <B>..<H>` filtered to this story's commits contains no `.kb/` path; `redkiln verify --grain story` against the PR-boundary block and the ledger. |

## Interaction quality

This story **renders no user-facing surface**. `_design.md` records `N/A — no public surface` for every
block and was approved on that determination (`_design.md:41-100`), so there is no signed-off
composition, transience policy or density budget to inherit, and none is invented here. What it does
ship is a *read* artefact with a named reader (`_storymap.md:16-22`) and a real navigation path into a
shared document, so the invariants below are the ones that genuinely apply, taken from the mount
point's own convergence rule (`_storymap.md:24-30`) and from the index genre's standing bullet shape
(`.kb/maps/open-questions-index.md:136-143`).

**Every invariant that applies is already a row in the acceptance-criteria table above.** Nothing is
introduced here as a prose bullet, because a bullet in this section gets no ledger row, is never gated
and is never checked.

**State family**

| Invariant | Applies? | Carried by | How verified |
| --- | --- | --- | --- |
| In-place vs context-jump | Yes | **AC-008** | The audit lands *inside* `_closeout-record.md`, immediately after the slice-mate's table — the reader does not jump to a second document to learn whether the questions survived. A findings-only file in this story's own directory satisfies every other check in this spec and reaches nobody; it is the named failure this invariant forbids. |
| Non-occlusion | Yes | **AC-008** | Appending must not displace, re-word, re-order or truncate the slice-mate's decision-atom audit table, or the harness section's SHA above it. The diff shows added lines only within `_closeout-record.md`. |
| Reversibility | Yes | **AC-005**, **AC-008** | The section is purely additive and removable without touching a single `.kb/` byte — because the story writes none. Conversely, an atom flipped or a bullet annotated here would be an irreversible edit to another project's record, which is why AC-005 requires findings to be *recorded* rather than corrected. |
| Preserved focus / scroll / selection | N/A | — | No interactive surface; there is no selection or scroll state. The nearest analogue — a reader's place in the record — is served by the in-place invariant above. |
| Keyboard reachability | N/A | — | No control surface; the artefact is static markdown in a git tree. |

**Composition family** — inherited from the mount point and the KB index genre, since `_design.md`
declares no composition of its own.

| Invariant | Applies? | Carried by | How verified |
| --- | --- | --- | --- |
| Presentation exists at all | Yes | **AC-001**, **AC-002** | Every claim is *composed* evidence — a quoted command, its quoted output, and two full SHAs — not a prose sentence that happens to contain the fact. A row reading "checked, fine" fails this persona even when it is true (Context pack decision 8), and AC-001/AC-002 are what make it fail. |
| Composition / placement | Yes | **AC-008** | One `## Open-question preservation audit` section, at the record's section level, directly after the slice-mate's table. Placement is load-bearing because the consumed set is *computed from* that table: the reader meets the input immediately before the thing computed from it. |
| Transience | Yes | **AC-003**, **AC-004** | Persistent chrome, not revealed-on-trigger content: the rename report and both asymmetry directions of the consumed-set reconciliation are present and **answered** even when empty ("no `R` entries in the range"; "no consumed atom outside DR-7's six"). A heading that appears only when something went wrong makes the section's *shape* carry the result, which is exactly the deletion-by-omission this story exists to detect. |
| Density budget | Yes | **AC-003**, **AC-004**, **AC-007** | The real numbers: **one** section; **one** consumed-set enumeration stated before the table; **one** row per consumed atom — expected **6 to ~12** rows (DR-7's six as the floor, the computed union as the actual); **19** baseline atoms named in the parity check, not counted; **three** separately observed index verdicts per row; **three** permitted destinations per finding; **two** full SHAs. Each is a hard count — a table shorter than the enumerated consumed set fails AC-004 regardless of how it reads. |
| Hierarchy | Yes | **AC-002**, **AC-005** | The zero-deletion result outranks everything and is stated first with its command; the per-atom resolution table sits beneath it; findings sit last with their destinations. Ownership is explicit in every finding row — the *owning sibling*, never this project — so a finding can never be misread as work this story took on. |
| Named anti-patterns | Yes | **AC-002**, **AC-005**, **AC-006**, **AC-008** | Four, each with a structural guard rather than a request for diligence: the two-point diff standing in for a walk (AC-002); the audit that flips a status to make its own table green (AC-005, AC-008's zero-`.kb/`-paths assertion); the atom rewritten into its own answer and passed by a clean `D`-filter (AC-006); and `discover.md:36-38`'s own named wrong implementation — the story that runs the diff, finds it empty, and stops (AC-004 through AC-007 are jointly what stop it). |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | The two baseline derivations disagree — `git merge-base main <branch>` and the parent of the first `.bklg/from-contract-to-published-library/` commit resolve to different SHAs. | Record **both** SHAs and both commands, state which was used for the walk and why, and route the discrepancy as a finding. Never silently pick one; a range chosen after seeing the result is not a range. |
| **EC-002** | `_closeout-record.md` does not exist, or does not yet carry the slice-mate's table, when this story runs. | Halt and report. The file's existence and preamble belong to `clean-checkout-harness` (`_storymap.md:53`); creating it here would take that story's deliverable and hide its absence. |
| **EC-003** | The slice-mate's decision-atom audit table exists but is empty or incomplete (e.g. reserved ADR numbers unresolved). | Fall back to DR-7's six as the consumed set, state **explicitly in the section** that coverage is degraded and why, and route the incompleteness as a finding. A silent fallback would make AC-004's reconciliation vacuous. |
| **EC-004** | The `--diff-filter=D` walk is **not** empty: an atom was deleted somewhere in the range. | The story still completes — this is the audit working. Record the commit, the path, the author and the commit message, quote the deleting hunk, and route to the owning sibling per DR-12. Do **not** restore the file; restoration is the owning project's act and would erase the evidence of when it went. |
| **EC-005** | `redkiln validate --kb` fails on the closeout tree. | Record the failure verbatim with the failing atom, route it, and state that the audit's frontmatter readings are therefore reported against an unvalidated corpus. Never edit an atom to make validation green — that is the AC-005 posture applied to the tool rather than the table. |
| **EC-006** | A consumed atom was resolved in a way `KbFrontmatter`'s status enum cannot express. | Record the state it wanted, cite `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md` as the already-filed precedent, and route as a finding against that question's owner. Do not invent a passthrough key to express it (`.kb/open-questions/README.md:47-50`). |
| **EC-007** | The closeout tree contains `open_question` atoms the baseline did not. | Not a finding. List them by name in the parity check as growth — a directory that only grows is the property AC-003 is asserting, and an unnamed increment is a count, not evidence. |
| **EC-008** | A rename is found and the index bullet plus every referrer were updated in the same commit. | Record it as a deliberate rename with old → new path and the updating commit, listed rather than absorbed. It is not a finding, but an unlisted rename is indistinguishable from a deletion at the next reading. |
| **EC-009** | This audit contradicts the DoD 14 / DoD 15 rows already recorded by `dod-set-re-observation-record`. | The disagreement is itself a finding routed to `findings-disposition-register` (HS-S0134). Do not edit the earlier section, and do not soften this story's result to agree with it (Context pack decision 9). |
| **EC-010** | A DR-7 atom is found still genuinely open because the question was never actually settled. | Record as still open with the reason, and route a finding against the sibling that was expected to settle it. Flipping it, or quietly dropping the row, both turn a real gap into a green table. |

## Non-functional

| id | Requirement | Why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **Reproducible by hand.** Every command in the section runs from a clean checkout with no alias, no helper script and no `xtask` subcommand, and is copy-pasteable as written. | The persona re-runs the audit rather than trusting it (Context pack decision 8). A checker committed to `xtask/` would be the better artefact if this ran repeatedly; it runs once, and this project owns no code (*Data and migrations*). |
| **NF-002** | **Durable citations.** Full 40-character SHAs, never branch names or abbreviations; atom references as repo-relative paths plus frontmatter `id`, so a later rename does not orphan the citation. | The initiative branch is deleted at closeout; a range written as `main..HEAD` stops resolving the day after it is written. |
| **NF-003** | **Read-only against `.kb/`.** Zero bytes written under `.kb/` by this story, enforced by the PR-boundary block and asserted in the record itself. | It is the property that makes the audit falsifiable (AC-005, AC-008), and it is checkable from the diff alone by someone who does not read the prose. |
| **NF-004** | **Legible in one sitting.** The section stays within the density budget above and does not restate the slice-mate's decision corpus; it links to it. | The mount point is shared with the whole project's closeout evidence (`_storymap.md:24-30`); a section that re-derives its input doubles the record and halves the chance it is read. |
| **NF-005** | **No gate regression.** A markdown-only diff must leave `cargo xtask ci` — this project's terminal grain (`.redkiln/config.yaml:60`) — exactly as green as `whole-gate-green-on-the-assembled-tree` recorded it. | If the gate colour changes across a docs-only diff, something outside this story moved and that is itself a finding. |

## Implementation notes (non-prescriptive)

A workable order, not a mandate. The obligations are the ACs above.

- **Derive the range before reading anything else**, and write both derivations into the record as you
  get them. Deciding the range after seeing which atoms look awkward is the failure EC-001 exists for.
- The commands that do the work are ordinary `git`; nothing here needs tooling:
  `git rev-list --count <B>..<H>`, `git log --no-renames --diff-filter=D --name-status <B>..<H> -- .kb/open-questions/`,
  the same with `-M` for renames, `git ls-tree -r --name-only <rev> -- .kb/open-questions/` at both
  ends, `git show <H>:<atom>` for frontmatter, and `git diff <B>..<H> -- <atom>` for body preservation.
  Prefer `git log` over `git diff` everywhere the question is "did this ever happen" rather than "is it
  different now" — that distinction is the whole of AC-002.
- **Read the slice-mate's table first, then DR-7 — not the other way round.** Starting from the six
  filenames makes the union feel complete and the computed set feel like a formality; starting from
  what the initiative actually settled is what surfaces a seventh.
- Consider capturing raw transcripts as a companion note inside this story's own directory and citing
  it from the record, rather than pasting hundreds of lines into the shared file (NF-004). The boundary
  block already permits this story's directory.
- When a row is a finding, write the destination in the same sentence as the observation. A findings
  list assembled at the end is where destinations get generalised into "route to support".
- If you find yourself opening an editor on anything under `.kb/`, stop: that is the instrument editing
  what it measures (Context pack decision 1), and the boundary block will fail the story anyway.

## Tests and CI (merge gate)

The testing brief classifies AC-006 as **static / process** (`_decomposition.md:49`). Nothing below is
a new test-framework assertion; this project writes no production code and introduces no fixtures
(`_decomposition.md:109-123`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static — range** | Reviewed diff: both baseline derivations quoted with output and agreeing; `git cat-file -e <B>` and `git cat-file -e <H>` resolve; `H` string-equal to the SHA in the record's clean-checkout / gate section | AC-001, EC-001, NF-002 |
| **Process — the walk** | `git log --no-renames --diff-filter=D --name-status <B>..<H> -- .kb/open-questions/` (empty) and `git rev-list --count <B>..<H>`, both quoted in the record | AC-002, EC-004 |
| **Process — renames and parity** | The same range with `-M`; `git ls-tree -r --name-only <B> -- .kb/open-questions/` versus the `<H>` form, set difference computed in the record against the baseline nineteen | AC-003, EC-007, EC-008 |
| **Static — consumed set** | The enumerated consumed set cross-read against the slice-mate's decision-atom audit table in `_closeout-record.md` and against DR-7 (`project.md:155-161`); both asymmetry directions addressed | AC-004, EC-003, EC-010 |
| **Static — frontmatter** | `git show <H>:<atom>` per consumed atom; `status` in {`withdrawn`, `superseded`} with a `related` edge, or a finding row carrying one of DR-12's three destinations (`project.md:179-183`) | AC-005, EC-006, EC-010 |
| **Static — body preservation** | `git diff <B>..<H> -- <atom>` per consumed atom, classified frontmatter-only versus body-touching, hunks quoted where not frontmatter-only; `tracks` / `resolution_ref` neither stripped nor invented | AC-006 |
| **Static — index** | `.kb/maps/open-questions-index.md` read at `<H>`: bullet present, status word from {`Open`, `Withdrawn`, `Superseded`} (`:141`), word equal to frontmatter status | AC-007 |
| **Gate — knowledge base** | `redkiln validate --kb` on the closeout tree, exit zero, recorded | AC-008, EC-005 |
| **Gate — read-only posture** | `git diff --name-only` across this story's commits contains **no** `.kb/` path; `redkiln verify --grain story` against the PR-boundary block above | AC-005, AC-008, NF-003 |
| **Gate — backlog** | `redkiln verify --grain story` against this story's `_ledger.md` (`.redkiln/config.yaml:67`) and `redkiln record-links --sha` for commit provenance (`:73`) | Every AC row satisfied with cited evidence; no file changed outside the boundary |
| **Gate — whole** | `cargo xtask ci` (**not** `--fast`; `.redkiln/config.yaml:60` is this project's terminal grain, `_decomposition.md:93-107`) | NF-005 — no regression from a markdown-only diff |

## Risks and coupling (PR-scoped)

- **The temptation to fix, and it will be one line.** A consumed atom sitting at `status: open` with the
  answering ADR already on disk is a two-character edit away from a green table, and every instinct
  says to make it. That edit destroys the audit: it makes the instrument write the value it reports
  (Context pack decision 1) and it takes the resolving sibling's act without its context. The boundary
  block and AC-008's zero-`.kb/`-paths assertion are the structural guards; the risk is that someone
  reads them as bureaucracy.
- **The empty-diff illusion.** `discover.md:36-38` names this exactly: run the diff, find it empty,
  report AC-006 satisfied. A diff cannot distinguish "resolved and correctly left on disk" from "never
  touched and correctly left on disk" — both pass identically. AC-004 through AC-007 exist because of
  this and are the half most likely to be shortened under time pressure.
- **The consumed set depends on another story's output.** If `decision-atom-audit-table` (HS-S0129)
  lands thin — reserved ADR numbers unresolved, or a settled answer with no atom — this story's
  computed set inherits the gap (EC-003). Both are in the same slice and the same context, which makes
  the coupling manageable and also makes it easy to blur: the enumeration is the slice-mate's
  deliverable, this story consumes it and does not re-derive it.
- **The mount file belongs to a story outside this slice.** `clean-checkout-harness` creates
  `_closeout-record.md` and is not in this story's `depends_on` (it is upstream by merge order,
  `_storymap.md:140-155`). If slice ordering changes, EC-002 fires and this story blocks rather than
  improvising a file.
- **`ce933d8` is a spec-time guess.** It is stated as the expected baseline and re-derived on the day.
  If the branch is rebased, squashed or re-cut between now and closeout, the expected value is wrong
  and the derivation is right — write down the disagreement (EC-001) rather than quietly using the
  spec's number.
- **Nineteen is also a spec-time count.** It is the baseline figure at authoring, taken from
  `.kb/open-questions/` excluding `README.md`. Sibling projects may add atoms before closeout; AC-003
  names additions rather than asserting a fixed total, so growth is expected and a *shrink* is the
  signal.
- **Findings routed here reach HS-S0134, which this story blocks.** Under-recording a finding here
  removes it from the register entirely; there is no later pass that rediscovers it. The record's
  findings list is the only channel.

## Dependencies

**Blocks on** (`story.md` frontmatter `blocked_by: [HS-S0129]`):

- **`decision-atom-audit-table`** (HS-S0129) — supplies the enumeration of what this initiative
  actually settled, which is the input AC-004 computes the consumed set from. `_storymap.md:79-80`:
  "they share the same 'what did this initiative settle' enumeration, and the second is meaningless
  without the first's list." Same slice (`answers-audit`), implemented in one context, merged first
  (`_storymap.md:147`).

**Upstream by merge order, not by a `depends_on` edge** — stated because EC-002 depends on it:

- **`clean-checkout-harness`** (slice 1) creates `_closeout-record.md` and its preamble
  (`_storymap.md:53`); **`whole-gate-green-on-the-assembled-tree`** records the closeout SHA this story
  reuses as `H` (AC-001).

**Unlocks** (`story.md` frontmatter `blocks: [HS-S0134]`):

- **`findings-disposition-register`** (HS-S0134) — routes every finding this audit records to `support`,
  to a new item against the owning sibling, or to a decision atom plus a re-plan (project AC-013).
  It depends on this story by name (`_storymap.md:62`).
- Transitively, **`initiative-closeout-readiness`**, through HS-S0134: project DoD 3 cannot close while
  the audit half of BR-15 is unrecorded.

## Anchors (progressive disclosure)

Link, do not paste. Each row says why the artefact is load-bearing and the moment to open it. Every
path was confirmed to exist at spec time.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.kb/open-questions/README.md` | `:40-45` is the resolution protocol in the corpus's own words — new atom, `related` link, status to `withdrawn`/`superseded`, **body left describing what was not known**. `:47-50` is the `.passthrough()` note: `tracks` / `resolution_ref` survive validation and must be neither stripped nor invented. This is the rule the whole story audits against; it is not a project-local convention. | Before writing the status column, and again before classifying any body diff. | AC-005, AC-006 |
| `.kb/maps/open-questions-index.md` | `:12-13` carries the never-removed rule inside the atom's own `summary`; `:136-143` fixes the bullet shape — status word first, from a closed three-value set, then the id, then one sentence. Both halves of AC-007 are read off these two ranges. | When building the index column, one atom at a time. | AC-007 |
| `.kb/decisions/README.md` | `:7-13` is the immutability rule, cited here for its **scope**: it binds `status: accepted` *decision* atoms. That scope is exactly why an `open_question` body has no automated guard and why AC-006 has to be a human check. | Once, when you are tempted to assume `validate --kb` already covers body preservation. | AC-006 |
| `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md` | The already-filed precedent that `KbFrontmatter`'s status enum does not cover every state the corpus needs. Turns "the status is wrong" into "the status could not be expressed, and here is what it wanted" — a different finding with a different destination. | Only if a consumed atom's status looks wrong rather than missing. | AC-005 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | `:155-161` is DR-7 with the six filenames AC-004 reconciles against; `:179-183` is DR-12's three destinations, the closed set every finding must draw from; `:211-214` is project AC-006 verbatim; `:114-116` is the *Out of scope* sentence that limits this project to auditing the two silences. | `:155-161` before enumerating the consumed set; `:179-183` before writing the first finding. | AC-004, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | `:49` is the AC-006 testing row — tier **static / process**, the `--diff-filter=D` instrument, and the six filenames cross-checked against the index. `:93-107` fixes the merge-gate commands and why `--fast` is not this project's bar; `:109-123` records that the project introduces no fixtures and mocks nothing. | Before choosing instruments, and again before writing the Tests table's evidence into the ledger. | AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | `:24-30` is the convergence decision that makes `_closeout-record.md` the mount point and names the fourteen-places failure mode; `:53` assigns the file's creation to `clean-checkout-harness`; `:78-80` is the slice rationale; `:92-94` is "no story fixes anything"; `:140-155` is the merge order EC-002 depends on. | Before appending anything, and any time the shape of the deliverable is in question. | AC-008 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/decision-atom-audit-table/spec.md` | The slice-mate's own spec — the definition of the table this story computes its consumed set from, including which questions it treats as settled and how it handles a reserved-but-unwritten ADR number. Reading it is how AC-004 avoids re-deriving the enumeration. | Immediately before computing the consumed set; the two stories are implemented in one context. | AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/open-question-preservation-audit/discover.md` | `:36-38` names the wrong implementation in full — the story that runs the diff, finds it empty, and stops — and `:27-30` records the three questions deferred to this spec (diff sufficiency, the baseline SHA, and whether atoms outside DR-7's six were also consumed). All three are answered above; this is where the reasoning that produced them lives. | First, once, before writing anything into the record. | AC-002, AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | Why the audit tables were shaped but not populatable at planning time, and the `0269720` precedent — process-shaped output authored without the process — which is the reason this story writes no atom even when writing one would be convenient. | If the read-only posture starts to feel like an obstacle. | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | `:41-100` records the approved no-surface determination and the sign-off. It is why this story owes no `## Items` row, no doctest and no perceptual review, and why *Interaction quality* above inherits its composition from the mount point instead. | If anyone asks for a rendered surface or a design review of this story. | AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:398-401` and `:402-404` are DoD 14 and DoD 15, whose second clause — "the corresponding open-question atom reflects that resolution, and `redkiln validate --kb` passes" — is precisely what this audit observes at closeout. BR-15 at `:298` is the requirement both halves of the slice serve. | When writing the section's opening sentence, so the record names what it is discharging. | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | The initiative-level traceability matrix that assigns the **audit** half of BR-15 to HS-P0019, and the DAG rank that puts this project last — the reason this story reads every sibling's output and writes into none of them. | If the boundary between "audit" and "resolve" is ever unclear. | AC-004 |
| `RUNBOOK.md` | `:262-284` is the ADR queue whose written and reserved numbers the slice-mate enumerates; its `related:` edges are where the questions this initiative consumed are named. Read it through the slice-mate's table rather than independently. | Only if the slice-mate's table leaves a settled answer unattributed (EC-003). | AC-004 |
| `.redkiln/config.yaml` | `:5` names the `support` initiative — one of DR-12's three destinations; `:67` (`require_ledger`) makes this story's `_ledger.md` mandatory and cited; `:73` (`require_commit_provenance`) ties it to a recorded commit; `:60` is the terminal grain NF-005 protects. | When writing findings destinations, and when filling the ledger. | AC-008 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | One of DR-7's six, and the one most likely to be resolved by a *different* project than the one that raised it (fixture capability limits versus the freeze). A useful first row to work: it exercises the status check, the `related` edge and the index bullet together. | When writing the first per-atom row, as a worked example. | AC-005, AC-006, AC-007 |
| `.kb/open-questions/ps-1-states-no-progress-obligation.md` | Another of DR-7's six, and the one whose question names a clause with a maturity marker — the case where a finding routes as "a new decision atom plus a re-plan" rather than to `support`. | When a finding looks like it touches a frozen clause, before choosing its destination. | AC-005 |

## Clarifications resolved during spec

1. **The baseline SHA, left informal at discover** (`discover.md:28`). Resolved: `B` is **derived, not
   declared** — two independent derivations that must agree (`git merge-base main <branch>`, and the
   parent of the first commit adding `.bklg/from-contract-to-published-library/`), with `ce933d8` named
   as the expected value and re-derived on the day (AC-001). Disagreement is EC-001, not a coin toss.
2. **Whether the diff alone discharges AC-006** (`discover.md:27`). Resolved: no, and the spec splits
   the obligation into four separately observable checks — survival (AC-002/AC-003), status (AC-005),
   body preservation (AC-006) and index annotation (AC-007) — because a diff cannot distinguish
   "resolved and correctly left on disk" from "never touched and correctly left on disk".
3. **Whether atoms outside DR-7's six were consumed** (`discover.md:29`). Resolved: the consumed set is
   **computed** from the slice-mate's decision corpus and **unioned** with DR-7's six, with both
   asymmetry directions reported (AC-004). DR-7 is treated as a prediction that the audit is permitted
   to find wrong in either direction.
4. **Who writes the index annotation.** `discover.md:38` reads as though this story authors "the
   specific index bullet text added". Resolved against it: **this story writes nothing under `.kb/`.**
   `.kb/open-questions/README.md:40-45` assigns the status flip and the annotation to whoever wrote the
   answer, on the commit where it landed; a missing bullet is a finding routed to the owning sibling,
   not a bullet added here (Context pack decision 1, AC-005, AC-008). The discover text is superseded on
   this point and is preserved unedited, as the record of what was believed before the protocol was read.
5. **A new, unnamed detection requirement: rename.** Not present in AC-006's text, in DR-7 or in the
   testing brief, all of which say `--diff-filter=D`. Added as AC-003 because a rename **is** a deletion
   of the path every index bullet and `source_paths` entry resolves against, and rename detection
   reports it as `R` — so the instrument the brief names would report a clean result over the exact
   failure the criterion is about. `--no-renames` on the primary walk, plus a separate `-M` pass to make
   deliberate renames visible, is the resolution.
6. **Deletion-by-rewrite is in scope.** Also not in AC-006's text. Added as AC-006 because
   `.kb/open-questions/README.md:44-45` forbids it explicitly ("Do not rewrite a question into its own
   answer") and nothing checks it: `validate --kb`'s immutability check binds `status: accepted`
   **decision** atoms only (`.kb/decisions/README.md:7-13`). An atom rewritten into its answer passes
   every deletion filter and destroys precisely what keeping the atom was for.
7. **The AC count is unchanged at eight.** AC-001…AC-008 as decided in the front half; none added,
   none dropped. AC-003 and AC-006 above are the two that go beyond the brief's literal instrument, and
   both were already reserved in the front half's *Behavior and interfaces* table rather than invented
   here.
8. **The 19-atom baseline is a measurement, not a constant.** Taken at spec time from
   `.kb/open-questions/` (twenty files, of which `README.md` is not an atom). AC-003 asserts the
   baseline set is a **subset** of the closeout set and requires additions to be named, so a sibling
   adding an open question before closeout is expected behaviour (EC-007), not a failed count.
