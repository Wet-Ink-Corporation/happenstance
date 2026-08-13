---
item: HS-S0134
stage: spec
created: 2026-08-12T13:48:13.459Z
updated: 2026-08-12T13:48:13.459Z
template_sig: 87bbf1d0
rendered_sig: 81c4a46e
---

# Spec — Every finding routed to an owner, none absorbed here

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — *Out of scope* `:194-195` (incidental bugs route to `support`), *Assumptions* `:558-559`, exit criterion 8 `:580-581` |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) — the DAG, this project as the rank-6 sink, *Decisions taken at the gate* |
| Project | [`.bklg/.../closeout-and-durable-audience/project.md`](../project.md) — **AC-013** `:237-240`, **DR-12** `:179-183`, DoD item 6 `:262-263`, the risk table `:293-306` |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/findings-disposition-register/spec.md` |
| Key brief | [`.bklg/.../closeout-and-durable-audience/_decomposition.md`](../_decomposition.md) — the one warranted brief (`testing`): the **AC-013 row** `:56` (tier **Process**), *Merge-gate commands* `:93-107`, and the amended **evaluator-persona decision** `:148-243` |
| Signed-off design | [`.bklg/.../closeout-and-durable-audience/_design.md`](../_design.md) — **no public API surface**; `## Items` is `N/A` `:41-45`. This story renders no surface and claims no item. |
| Story map row | [`.bklg/.../closeout-and-durable-audience/_storymap.md`](../_storymap.md) `:62` — slice `closeout-health-and-disposition`, row 2 of 3; *Where the evidence lands* `:24-30`; *Grain notes* `:92-94`; merge order step 5 `:152-155` |
| Grounding | [`.bklg/.../closeout-and-durable-audience/_grounding.md`](../_grounding.md) — *Tensions / open items to flag, not silently resolve* `:115-145`, the standing instruction behind "record, do not repair" |
| This story's discover | [`findings-disposition-register/discover.md`](discover.md) — the signal ledger, the answered routing question, and *The wrong implementation* `:37-39` |
| Roadmap pointer | [`RUNBOOK.md`](../../../../RUNBOOK.md) — the plan of record; the ADR queue at `:262-284` that lane 3 hands work back to |

## One-line PR slice

Route every finding the re-observation surfaced — `support`, a new item against the owning
sibling, or a decision atom plus a re-plan — into one **Findings and disposition** section of
`_closeout-record.md`, and prove from git that zero of them were fixed inside this project.

## Executive summary

**What this PR lands:** the register — one row per finding, each with the story that surfaced
it, the DR-12 lane it takes, and a *concrete* destination — mounted as
`## Findings and disposition (AC-013)` in this project's single closeout artefact; plus the
two proofs that make it more than a list: a **bidirectional sweep** showing no upstream finding
is missing from it, and a **git-derived zero-fix proof** taken over the whole project's commit
range rather than over this story's own diff.

**Delta against what already exists.** Six upstream stories have each already written a findings
list with a *proposed* destination — `dod-set-re-observation-record` says so explicitly
(`../dod-set-re-observation-record/spec.md:196-197`, `:210`), and `decision-atom-audit-table`
even fixes the id series (`F-01`, …) so its entries are addressable
(`../decision-atom-audit-table/spec.md:299`, `:318-321`). Three things are new here and exist
nowhere upstream:

1. **A proposal becomes a disposition.** Upstream stories are forbidden from routing
   (`../open-question-preservation-audit/spec.md:205-206`); this story assigns the lane and
   names the owner, which is the act AC-013 actually measures.
2. **Completeness becomes checkable.** Six findings lists in six sections cannot be shown
   exhaustive from inside any one of them. The register is the first place the *union* exists,
   and the first place an omission is visible.
3. **"Zero were fixed" becomes an observation.** Each upstream story proves its own boundary was
   clean. AC-013's claim is about **this project**, and only a diff over all eleven stories'
   commits proves it.

The story writes no production code and repairs nothing — including the defects it is best
placed to repair, which is the whole point (`project.md:299`).

## Context pack

The load-bearing decisions this story must honour. Read this section and you can start; the
deeper artefacts sit behind the signposted anchors the second pass appends.

**1 — Routing *is* the deliverable, and a repair fails the story rather than completing it.**
`project.md:179-183` (DR-12) fixes exactly three lanes and no fourth: an incidental defect goes
to the `support` initiative (`.redkiln/config.yaml:5`); a genuine cross-project interaction
becomes **a new item against the owning sibling, not a fix here**; anything touching a
`[FROZEN]` clause takes **a new decision atom and a re-plan**. `project.md:237-240` (AC-013)
adds the negative half: "Zero are fixed inside this project." The project's own risk table
names why this story exists at all — "the first tree where a cross-project interaction is
visible… the project most tempted to absorb a fix" (`project.md:299`).

**2 — The inputs are six named stories, and the register is built bidirectionally over them.**
This story's `blocked_by` is not a scheduling artefact; it is the input list
(`story.md` frontmatter; `_storymap.md:62`). Walk each producing story's mounted section **and**
its own folder, and reconcile in both directions — every upstream finding entry must appear as
a register row, and every register row must name the story and entry it came from:

| Input story | Where its findings are written | Note |
| --- | --- | --- |
| `whole-gate-green-on-the-assembled-tree` (HS-S0126) | the **Gate run** section, and `gate-run.md` §*Skips and findings* | a `skipped` `cargo hack` / `cargo deny` is a finding, not a footnote (`../whole-gate-green-on-the-assembled-tree/spec.md:325`) |
| `dod-set-re-observation-record` (HS-S0127) | `## DoD re-observation (1–12, 14–15)` + its findings list; `evidence/findings-and-boundary.md` | rows carry a **proposed** destination only (`../dod-set-re-observation-record/spec.md:210`) |
| `published-tree-delta-statement` (HS-S0128) | `## DoD 13 — the published-tree delta` | escalates only in the negative case — a changed published surface (`../published-tree-delta-statement/spec.md:215`) |
| `decision-atom-audit-table` (HS-S0129) | `## Decision-atom audit (AC-005)` + findings list, ids `F-01…` | ids are stable and unique **so this story can address them** (`../decision-atom-audit-table/spec.md:299`) |
| `open-question-preservation-audit` (HS-S0130) | `## Open-question preservation audit` + findings list | its EC-009 routes a disagreement with the DoD table here rather than reconciling it (`../open-question-preservation-audit/spec.md:325`) |
| `backlog-and-kb-health-at-closeout` (HS-S0133) | its section of `_closeout-record.md` — **heading not yet fixed** | its `spec.md` is still the rendered stub at this spec's authoring time; discover the heading by reading the record, do not assume one |

A section that is absent when this story runs is a **blocking finding against that story**, not
a silent zero: the register says which input it could not read and stops claiming completeness
for it.

**3 — This story does not open items. `redkiln` does, and this story is not its writer.**
`CLAUDE.md` (*Where the work lives*) is unambiguous: "The CLI is the only writer of an item's
system frontmatter… drive every state change through `redkiln <command>`", and a `PreToolUse`
hook denies the edit. So a destination is recorded in one of exactly two states, and the
difference is visible in the row:

- **opened** — an item id that *exists on disk* and is cited by path, or
- **pending** — the exact `redkiln` invocation to open it, plus the named owner.

A fabricated id, a "will be opened" with no command, or a lane with no referent all fail the
story. This is not a weakness of the register; it is what keeps it honest about who has the
pen.

**4 — Lane 1's destination is a bare stub, and that is an observation to record, not a gap to
fill.** `.bklg/support/` contains exactly `initiative.md` and `_intake-brief.md`; the initiative
is `status: exploring`, `stage: intake`, with no project and no story under it. Lane-1 rows
therefore name the initiative (HS-I0005) and the item to be opened under it — they cannot cite
an existing project, and inventing one here would be the hand-authored-backlog failure
`0269720` was reverted for (`_grounding.md:41-50`).

**5 — Lane 3 writes nothing under `.kb/`.** A finding that touches a `[FROZEN]` clause is
routed as *a new decision atom owed by a named owner, plus a re-plan* — the register states the
clause id, the commitment it moves and who owns the atom, and authors no atom.
`.kb/decisions/README.md:8-13` makes an accepted body immutable and `redkiln validate --kb`
enforces it against `HEAD`; `:20-23` draws the repair/amendment line ("filing an amendment as a
repair is how a frozen commitment quietly moves"). Writing the atom here would additionally be
hand-authoring a `.kb/` atom outside the ingest path, which DR-8 and `0269720` forbid.

**6 — The pre-registered risk shapes are swept, not just whatever happened to surface.**
`project.md:293-306` is a list, written before any of this ran, of the shapes a finding was
expected to take: DoD 13's phrase, the `!Send` flavour quietly dropped, a seventh
`template-drift` advisory, the kb-ingest glob and a second wave, thin upstream evidence,
positioning dates against a named live peer, the evaluator decision's downstream reach. Each
row gets an explicit **surfaced / did not surface** line with its evidence. A register that
only lists what the upstream stories volunteered cannot distinguish "nothing was found" from
"nobody looked", and the risk table is the repository's own answer to that.

**7 — A decision *this project* took that settled a cross-project disagreement is itself a
finding.** This is the wrong implementation this story rejects, named in its own discover
(`discover.md:37-39`): quietly "correcting" a cross-project disagreement inside this project's
planning artefacts satisfies a shallow reading of "no code was fixed" while making the
disagreement invisible to the sibling that owns it. The concrete instance is the
evaluator/DT-1 question. Its state must be **observed on both sides at register time**, not
assumed: this project's brief carries the amendment at `../_decomposition.md:148-180` and its
consequence at `:227-242`, and `publication-and-positioning`'s own DT-1 rider carries the
matching amendment at `../../publication-and-positioning/_design.md:221-243` ("Net effect on
this project: none. DT-1 is unchanged and no published surface moves"). If both sides agree,
the row records **reconciled at planning, no item owed**, citing both — a disposition, with
evidence, not an omission. If they disagree, it is a lane-2 finding against HS-P0016. Either
way the row exists; its absence is the failure.

**8 — "Zero were fixed inside this project" is measured across the project, not across this
story.** Each upstream story already proves its own boundary was clean; AC-013's subject is the
project. The proof is a diff over the commit range covering all eleven stories of HS-P0019,
against a stated allow-list: `.bklg/**` planning artefacts, and the `.kb/_intake/` →
`.kb/product/` (and the `.kb/maps/` index sync) writes that
`product-atom-promotion-via-kb-ingest` legitimately produces via `/redkiln:kb-ingest`
(`project.md:215-221`, AC-007/AC-008). **Any** path under `crates/`, `xtask/`, `spec/`,
`standards/`, `examples/`, `references/` or `.github/` is a fix, whatever its commit message
says, and any *hand-authored* write under `.kb/` outside that ingest path is the same failure
wearing a different hat.

**9 — A swept zero is a real outcome; an unstated zero is not.** If nothing surfaced, the
section says so explicitly and shows the sweep — the six inputs read, the risk rows checked,
the diff clean. `_storymap.md:20-22` sets the bar for the whole project: done means a command
was run on *this* tree and a reader can follow the citation back, never that a box was ticked.

**10 — The persona-journey slice.** The reader is the one `_storymap.md:17-22` names and the
amended DR-10 promotes as a journey atom of its own: someone who must be able to believe the
initiative closed honestly **without re-deriving the evidence**, and who cannot run the suite.
For this story that means the register answers their sharpest question — *what did this
initiative find, and who has it now?* — from the record they already have open, in one sitting,
with every destination followable.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` (`story.md` frontmatter `archetype: capability`) — the register is the thing the reader meets; there is no substrate here for a later story to build on beyond reading it. |
| **Slice / milestone** | `closeout-health-and-disposition` (`_storymap.md:61-63`), the terminal sequence. |
| **Slice-mates** (one context, one integrated surface) | `backlog-and-kb-health-at-closeout` (HS-S0133) — merges first, and is one of this story's six inputs; `initiative-closeout-readiness` (HS-S0135) — merges last and reads this register. `_storymap.md:85-88`: "findings can only be routed once every other slice has produced its findings, and the initiative can only be declared closable once both have landed." |
| **Mount point** | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — the project's single convergence artefact (`_storymap.md:24-30`), stood up by `clean-checkout-harness` in slice 1 and already carrying five sections by the time this story runs. This story **appends one** `## Findings and disposition (AC-013)` section to it. A register living only in this story's folder would satisfy every content check and reach nobody — the unmounted failure this contract forbids. |
| **Wires into** (real contracts, by path) | The five mounted sections named in Context pack decision 2, plus `backlog-and-kb-health-at-closeout`'s, read as **state contracts** — their finding ids and proposed destinations are this story's input and are never rewritten in place. `.redkiln/config.yaml:5` (`support_initiative: support`) — lane 1's destination; `:67` / `:73` (`require_ledger`, `require_commit_provenance`) — this story's `_ledger.md` is mandatory and cited and its commit is recorded via `redkiln record-links --sha`. `.bklg/support/initiative.md` (HS-I0005) — the lane-1 parent as it actually exists. `.kb/decisions/README.md:8-23` — the immutability and repair/amendment rules lane 3 defers to. `RUNBOOK.md:262-284` — the ADR queue a lane-3 finding hands work back to. `git` over the project's commit range — the instrument for decision 8. |
| **Renders surfaces** | **none.** `_design.md` records no public API surface and no screen (`## Items`: "N/A — no public surface", `:41-45`); the initiative is `userFacing: false` (`../../_decomposition.md:266`). This story claims no `path` id and adds, changes or removes zero Rust items. |
| **Conformance rule(s)** | **None, and deliberately: this is not adapter-observable.** No rule is added to `crates/happenstance-testkit/src/suite.rs` and no port changes. No conformance rule can express "a defect found at closeout has a named owner"; the instruments are `git`, the six upstream sections, and a human-readable table. A rule invented here would be one no adapter could fail — the decorative-rule failure `CLAUDE.md` names. |
| **Clause(s)** | **None discharged or amended.** `spec/SPECIFICATION.md` is not opened for edit. If a finding implicates a `[FROZEN]` clause, the register names the clause and routes lane 3 (new decision atom + re-plan); it never edits the clause or its maturity marker. |
| **Advances DoD scenario** | **`project.md` Definition of done item 6** (`:262-263`) — "Every finding is routed with a recorded destination and none was fixed here (AC-013)" — whole, this story's alone. At initiative grain it is a precondition of **exit criterion 8** (`../../initiative.md:580-581`): "what was learned has been harvested rather than left in the backlog." A finding absorbed silently here would let "every project is closed out" be true while the defect it hid stays unowned, which is the exact failure exit criterion 8 is written against, and it also guards the initiative's standing routing rule at `:194-195` and `:558-559`. |

**Delivered mounted.** The deliverable is the section inside `_closeout-record.md` plus this
story's `_ledger.md` (mandatory and cited, `.redkiln/config.yaml:67`, tied to a commit by
`:73`). Working notes may live in this story's folder; the answer may not.

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block
under this heading and fails on any file changed outside it.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/findings-disposition-register/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

Two entries, and the absence of `.bklg/support/**`, `.bklg/from-contract-to-published-library/publication-and-positioning/**`
and `.kb/**` is the point, not an oversight: the story that routes work must not be the story
that performs it, and the CLI — not this agent — is the writer of any item it routes to
(Context pack decision 3). Appending to `_closeout-record.md` is the **mount**, named in the
*Integration contract*, and is not scope drift.

**In this PR**

- The `## Findings and disposition (AC-013)` section appended to `_closeout-record.md`: the
  input sweep (six sources, each read and cited), the register table itself, the risk-shape
  sweep from `project.md:293-306`, the self-audit entry for the evaluator/DT-1 question, the
  zero-fix proof, and — if it applies — the explicit swept-zero statement.
- Stable register ids that trace back to each source's own finding id (`F-01`, …), so a reader
  can go from a row to the section that raised it and back.
- The captured command output backing the zero-fix proof, as a companion note in this story's
  own folder, cited from the section.
- This story's `_ledger.md` (second pass) and the `redkiln record-links --sha` provenance
  `.redkiln/config.yaml:73` requires.

**Explicitly not in this PR**

- **Any repair.** Not to code, not to a spec clause, not to a `.kb/` atom, not to a sibling's
  mounted section, and not to this project's own planning artefacts to make a disagreement go
  away. That last one is the wrong implementation this story is written against
  (`discover.md:37-39`).
- **Opening the items it routes to.** No `redkiln new`, no write under `.bklg/support/**` or
  any sibling project's directory. The row carries the exact invocation and the owner
  (Context pack decision 3).
- **Writing any decision atom, or staging one into `.kb/_intake/`.** Lane 3 names the atom
  that is owed; it does not author it (DR-8; `.kb/decisions/README.md:8-13`).
- **Re-litigating any upstream finding.** The register does not decide whether a finding is
  *correct* — only which lane it takes and who owns it. A disagreement with an upstream story's
  own conclusion is itself a row, not an edit to that story's section.
- **Editing a sibling's section of `_closeout-record.md`.** This story appends; it re-words,
  re-orders, truncates and displaces nothing already written there.
- **The health check itself** (`redkiln validate` / `validate --kb` / `doctor --json`, and the
  six-advisory disposition) — `backlog-and-kb-health-at-closeout`'s, AC-011/AC-012. This story
  consumes its findings.
- **Declaring the initiative closable** — `initiative-closeout-readiness`'s, AC-014.

**Merge DoD:** `_closeout-record.md` carries a findings register in which every finding raised
by the six input sources has exactly one row, every row names a DR-12 lane and a concrete
destination in a stated `opened`/`pending` state, every pre-registered risk shape is marked
surfaced or not, the evaluator/DT-1 self-audit entry is present with both sides cited, and a
git-derived proof over the project's whole commit range shows zero files changed outside the
`.bklg/**` planning artefacts and the ingest-produced `.kb/` writes.

## Behavior and interfaces

The register is compiled from the clean-checkout tree slice 1 produced
(`../_decomposition.md:103-107`: all four merge-gate commands "run against the clean-checkout
tree described in DR-1, not against the working tree these planning artefacts are written in").

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Bidirectional enumeration over six sources** | Build the row set as the union of (a) every findings entry in the six input sources and (b) every row the sweeps below add. Exactly one row per finding. An entry in (a) with no row is an omission; a row with no source is an invention. Both directions are stated, not assumed. | `_storymap.md:62` (the six `depends_on`); `../decision-atom-audit-table/spec.md:299` (stable ids exist for this purpose) |
| **A source that cannot be read is a row, not a zero** | If an input section is missing or its story has not landed, the register names the source, says it could not be read, and raises a blocking finding against that story rather than reporting zero findings for it. | `_storymap.md:85-88`; `project.md:262-263` |
| **Row shape is fixed here, and every cell is load-bearing** | *Register id* / *Finding* (what was expected, what was observed) / *Surfaced by* (story + its own finding id) / *Lane* (one of DR-12's three) / *Destination* (the concrete referent) / *State* (`opened` with an id that exists, or `pending` with the exact command + owner) / *Evidence* (`file:line` into the source section). No cell is optional; "TBD", "noted" and a blank destination each fail the row. | `project.md:179-183`; `../decision-atom-audit-table/spec.md:193` (the precedent that a row shape is fixed, not improvised) |
| **Exactly three lanes, and the lane is chosen by a stated test** | Lane 1 `support` — an incidental defect, no cross-project interaction, no committed surface moved. Lane 2 **new item against the owning sibling** — the behaviour of one project observed to interact with another's, or an inconsistency between two projects' recorded decisions. Lane 3 **decision atom + re-plan** — the finding cannot be recorded without moving something a `[FROZEN]` clause or an accepted atom committed to. The row states which test it met. | `project.md:179-183`; `.redkiln/config.yaml:5`; `.kb/decisions/README.md:20-23` (repair vs. amendment) |
| **Destinations are concrete, and their state is honest** | `opened` requires an item id whose file exists on the tree, cited by path. `pending` requires the literal invocation (`redkiln new …`, parent named) and the owner who will run it. No row asserts an id that does not exist. | `CLAUDE.md` *Where the work lives* (the CLI is the only writer of system frontmatter); `.bklg/support/initiative.md` (HS-I0005, the lane-1 parent as it exists) |
| **Lane 1 routes to an initiative, not to a project that isn't there** | `.bklg/support/` holds only `initiative.md` and `_intake-brief.md`; HS-I0005 is `status: exploring`, `stage: intake`, with no children. Lane-1 rows name HS-I0005 and the item to be opened under it, and record that the support tree was empty at closeout. | `.redkiln/config.yaml:5`; `.bklg/support/initiative.md`; `_grounding.md:41-50` |
| **Lane 3 names the atom; it never writes one** | The row states the clause id or accepted atom implicated, the commitment that would move, the owner of the new atom, and that a re-plan is owed. Zero writes under `.kb/`. | `.kb/decisions/README.md:8-13`, `:20-23`; `project.md:179-183`; `RUNBOOK.md:262-284` |
| **The risk-shape sweep is explicit, row by row** | Each of the ten rows of `project.md`'s risk table gets a **surfaced / did not surface** line with its evidence — including the ones expected to be quiet (the `!Send` flavour, a seventh drift advisory) and the one explicitly *not* a finding in its expected form (DoD 13's delta, which `published-tree-delta-statement` states as bounded and non-defective, and which appears here only if that constraint was found broken). | `project.md:293-306`; `../published-tree-delta-statement/spec.md:215` |
| **The self-audit: this project's own cross-project decisions are swept** | Every decision taken inside HS-P0019's stories that resolved a disagreement owned elsewhere gets a row. The evaluator/DT-1 case is the named instance: read **both** sides on the tree and record their agreement or disagreement with citations, never a silent correction. | `../_decomposition.md:148-180`, `:227-242`; `../../publication-and-positioning/_design.md:221-243`; `discover.md:37-39` |
| **The zero-fix proof is computed over the project, not the story** | A diff over the commit range covering all eleven HS-P0019 stories, against the stated allow-list: `.bklg/**`, and the `.kb/_intake/` → `.kb/product/` + `.kb/maps/` writes `/redkiln:kb-ingest` produced. Any path under `crates/`, `xtask/`, `spec/`, `standards/`, `examples/`, `references/`, `.github/` fails AC-013; so does a hand-authored `.kb/` write outside that ingest path. The command and its output are quoted, not summarised. | `project.md:237-240`; `project.md:215-221` (AC-007/AC-008, the legitimate `.kb/` writes); `_storymap.md:92-94` |
| **A swept zero is stated, with its sweep** | Zero findings is a permitted outcome and is written as an explicit sentence naming the six sources read, the ten risk rows checked and the clean diff — never as an absent table. | `_storymap.md:20-22`; `project.md:262-263` |
| **Upstream sections are read, never rewritten** | The register cites each source by heading and `file:line`; it re-words, re-orders and truncates nothing already in `_closeout-record.md`, and appends its own section below them. | `../published-tree-delta-statement/spec.md:164` (the append-only precedent); `_storymap.md:24-30` |
| **Register ids are stable and traceable both ways** | Each row's id is unique across the record and carries its source's own finding id, so `initiative-closeout-readiness` (HS-S0135) and any future reader can go row → source section → row. | `../decision-atom-audit-table/spec.md:299` (NF-006); `_storymap.md:63` |
| **No public API surface is added or changed** | `_design.md` returns `N/A` for `## Items`, `## Signatures` and `## Visibility and stability`; this story claims none of them. | `../_design.md:41-61` |

## Data and migrations

**N/A.** This story defines no schema, adds no table, changes no on-disk format and touches no
store — the project owns no code at all (`project.md` *Out of scope*; `_design.md:10-11`). Its
only persisted artefacts are markdown inside the PR boundary: one appended section of
`_closeout-record.md` and this story's own folder.

Two structures it depends on are nonetheless real and are **inherited, not negotiated**. The
first is the `_closeout-record.md` section layout `clean-checkout-harness` defined and five
sections already follow; this story appends in that shape rather than proposing a second one.
The second is the upstream finding-id series (`F-01`, …) that
`../decision-atom-audit-table/spec.md:318-321` fixes precisely so the two stories can address
the same entries — renaming an upstream id here would break the source it came from.

Nothing here is read by a program. `redkiln` reads item frontmatter, which the CLI alone writes
and which this story never touches; the gate reads the source tree, which this story does not
modify.

## Acceptance criteria

The reader every row is written from is the one `_storymap.md:17-22` names and the amended DR-10
promotes to a journey atom of its own (`../_decomposition.md:148-180`, `:227-232`): **someone who
must be able to believe the initiative closed honestly without re-deriving the evidence, and who
cannot run the suite.** Two further readers touch this section directly — the author of the
slice-mate `initiative-closeout-readiness` (HS-S0135, project AC-014), who must be able to say the
initiative is closable *because* nothing is unowned, and the future owner of a routed finding, who
arrives at a row cold and needs to know what to do next. Each criterion is a goal one of those three
has, crossing the whole path from the tree on disk to the record they open.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a reader who has just read five earlier sections of `_closeout-record.md` and wants to know whether the findings register covers *all* of them, **WHEN** they read the input sweep at the head of `## Findings and disposition (AC-013)`, **THEN** they find one line per input source — the six stories at `_storymap.md:62`, each named with the section or file that was read and the count of finding entries taken from it — and the register table below contains exactly one row per entry in that union, in both directions: an upstream entry with no row is an omission and a row naming no source entry is an invention. The sweep **reports its own coverage** rather than asserting completeness (`.kb/playbooks/verify-the-referent-and-report-coverage.md`) | **Bidirectional union check.** For each of the six sources, the entries are listed by their own ids and set against the register's *Surfaced by* column; the set difference is shown in both directions and is empty, or every element of it is itself a row. A statement of the form "all findings are covered" with no shown per-source count = fail — that is the "checked 84 of 338 while printing no problems found" failure the playbook is written from |
| AC-002 | **GIVEN** the future owner of a finding, arriving at a row with no memory of the project, **WHEN** they read it, **THEN** every one of the seven cells is filled — *Register id* / *Finding* (what was expected, what was observed) / *Surfaced by* (source story + that source's own finding id) / *Lane* (exactly one of DR-12's three, `project.md:179-183`) / *Destination* / *State* / *Evidence* (`file:line` into the source section) — and the row states the **test** by which its lane was chosen, so the routing is checkable rather than asserted. "TBD", "noted", "see above" and an empty cell each fail the row | **Row-shape and lane-test check.** Every row is read cell by cell; zero cells empty or placeholder; every *Lane* cell carries one of the three literal lanes plus the one-clause test it met (incidental / cross-project interaction / touches a `[FROZEN]` clause or accepted atom). Every *Evidence* `file:line` is opened and the described finding is present **at that location** — the referent, not merely the address (`.kb/playbooks/verify-the-referent-and-report-coverage.md`) |
| AC-003 | **GIVEN** the same owner, who now wants to act, **WHEN** they read *Destination* and *State*, **THEN** either the state is `opened` and the item id cited resolves to a file that exists on the tree, or the state is `pending` and the row carries the **literal `redkiln` invocation** that opens it with its parent named plus the human owner who will run it — and a lane-1 row names `support` / HS-I0005 as it actually exists (`.bklg/support/initiative.md`; `.redkiln/config.yaml:5`), never a `support` project that is not there, while a lane-3 row names the clause id or accepted atom implicated, the commitment that would move, the owner of the new atom and that a re-plan is owed, and authors nothing under `.kb/` | **Destination-resolution check.** Every `opened` id is resolved against the tree (the file exists, cited by path); every `pending` row contains a runnable command string and a named owner; `git diff --stat -- .kb/ .bklg/support/ .bklg/from-contract-to-published-library/publication-and-positioning/` over this story's commit range is **empty**. A fabricated id, a bare "will be opened", or a lane with no referent = fail |
| AC-004 | **GIVEN** a reader who cannot distinguish "nothing was found" from "nobody looked", **WHEN** they read the risk-shape sweep, **THEN** they find one line for each of the **ten** rows of `project.md:293-306` — the pre-registered list of shapes a finding was expected to take — each marked explicitly *surfaced* (with its register id) or *did not surface* (with the evidence that was checked and where), including the rows expected to be quiet (the `!Send` flavour, a seventh `template-drift` advisory) and DoD 13's delta, which appears as a finding only in the negative case its owner already bounded (`../published-tree-delta-statement/spec.md:215`) | **Pre-registered sweep check.** Row count equals ten and each maps to a distinct `project.md:293-306` risk; every *did not surface* line cites what was read to reach that conclusion (a section of `_closeout-record.md`, a command output, a path). A sweep that lists only what the upstream stories volunteered = fail; a *did not surface* with no cited evidence = fail |
| AC-005 | **GIVEN** the reader for whom "zero were fixed" must mean more than "no `.rs` file changed", **WHEN** they read the self-audit entry, **THEN** every decision taken inside this project's own stories that settled a disagreement owned elsewhere has a row, and the evaluator/DT-1 case in particular is **observed on both sides at register time** — this project's amendment (`../_decomposition.md:148-180`, `:227-242`) and `publication-and-positioning`'s own DT-1 rider (`../../publication-and-positioning/_design.md:221-243`) — recorded either as *reconciled at planning, no item owed* with both citations, or as a lane-2 finding against HS-P0016. The row exists in either case; its absence, or a quiet edit to either side to make the question go away, is the failure | **Both-sides observation check.** The entry quotes the operative sentence from each side with its `file:line`, and states the verdict. `git diff --name-only` over this story's commit range contains **neither** `../_decomposition.md` nor any path under `publication-and-positioning/` — the discrimination `.kb/governance/rewrite-the-referent-never-the-reasoning.md` draws, applied one layer out: an edit that changes what a planning artefact *asserts* is a reversal, not a repair. This is the check the named wrong implementation fails (`discover.md:37-39`) |
| AC-006 | **GIVEN** a reader who distrusts the claim rather than the author, **WHEN** they read the zero-fix proof, **THEN** they find the commit range covering **all eleven** HS-P0019 stories stated as full 40-character SHAs, the literal command quoted, its output quoted rather than summarised, and the allow-list stated *before* the result — `.bklg/**`; the `.kb/_intake/` → `.kb/product/` and `.kb/maps/` writes `/redkiln:kb-ingest` legitimately produced (`project.md:215-221`); and the `.redkiln/telemetry/events/` footprint `auto_stage_telemetry` leaves (`.redkiln/config.yaml:12`) — such that they can re-run it themselves and get the same answer. Any path under `crates/`, `xtask/`, `spec/`, `standards/`, `examples/`, `references/` or `.github/`, and any hand-authored `.kb/` write outside the ingest path, is a fix whatever its commit message says | **Re-derivable diff check.** `git diff --name-only <project-base-sha>..<closeout-head-sha>` run at the stated SHAs; every path in the output classified against the stated allow-list, with the classification shown per path prefix rather than as a total. An unstated allow-list, an abbreviated SHA, a summarised output, or a range covering only this story = fail (`project.md:237-240`; `_storymap.md:92-94`) |
| AC-007 | **GIVEN** a reader who opens the project's one closeout artefact, **WHEN** they reach the findings register, **THEN** they meet it **composed in place** in `_closeout-record.md` — a `## Findings and disposition (AC-013)` section carrying five subsections in the fixed order input sweep → register table → risk-shape sweep → self-audit → zero-fix proof, the table rendered with all seven columns, every sibling section already in that file left byte-identical, and — where nothing surfaced — an explicit swept-zero sentence naming the six sources read, the ten risk rows checked and the clean diff, rather than an absent table | **Composition + non-occlusion check.** `git diff -- .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` over this story's commit range shows **additions only**, every pre-existing line unchanged; the appended section carries five subsections in that order, a seven-column table, and no cell reading "I checked" or "yes". A register living only under `findings-disposition-register/` = unmounted = fail (`_storymap.md:24-30`: "fourteen ledger entries in fourteen places is the failure mode") |

**Traceability.** All seven rows serve **project AC-013** (`project.md:237-240`) and, through it, DoD
item 6 (`project.md:262-263`) and initiative exit criterion 8 (`../../initiative.md:580-581`). No row
discharges another project AC; AC-001's input sweep makes `initiative-closeout-readiness`'s AC-014
claim possible without re-deriving it, which is a service, not a transfer of ownership.

## Interaction quality

RFC §6.7/D6. This story renders **no screen and no public API item**: the signed-off `_design.md`
records `## Items` as "N/A — no public surface" (`../_design.md:41-45`) and the `design.capture`
perceptual review is a *declared* skip (`../_design.md:93-97`; `.redkiln/config.yaml` — `design:` is
deliberately absent). That sign-off binds here as a **prohibition**, not a licence: it forbids this
story from inventing a surface, and it leaves the composition rules for the artefact it *does*
render to be taken from the two places that actually fixed them — the convergence rule
(`_storymap.md:24-30`) and the section shape `clean-checkout-harness` established and five sections
already follow.

Every invariant below is carried by an AC row in the table above. None is stated only here.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| **In place, not a context jump** — the reader meets the register inside the record they already have open; the primary content never lives in a second file the record merely points at | AC-007 | Mount check: the section is in `_closeout-record.md`. A register under `findings-disposition-register/` cited from nowhere = fail |
| **Non-occlusion** — appending this section leaves the five sections slices 1–5 wrote untouched and still reachable, including the six findings lists this story reads | AC-005, AC-007 | `git diff` of `_closeout-record.md` shows additions only; every pre-existing line byte-identical |
| **Reversibility** — the whole change is one revertible commit, and reverting it restores every audited surface exactly, because nothing under `.kb/`, `.bklg/support/` or any sibling project was touched | AC-003, AC-005, AC-006 | `git diff --stat` empty for those three trees; commit provenance recorded per `.redkiln/config.yaml:73` |
| **Preserved reading position, selection, focus, scroll; keyboard reachability** | **N/A, declared not skipped** | There is no interactive surface to preserve state in (`../_design.md:41-49`). Recording this as an explicit N/A is the point: a silent omission and an unrendered surface look identical from the outside, which is the same failure the declared-skip convention exists to prevent |

**COMPOSITION invariants** — taken from the signed-off `_design.md`'s prohibition plus the record's
own inherited shape.

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| **Presentation exists at all** — the section is composed (an H2 heading, five ordered subsections, a real seven-column table with a header row), not a bare dump of finding sentences or a bulleted list of destinations | AC-002, AC-007 | Structure check: the five subsections present in the fixed order; the table renders with all seven columns |
| **Placement** — inside `_closeout-record.md`, appended *below* the five sections it reads, in the project directory beside the health verdict and the closeout readiness statement | AC-007 | Mount-point check (`_storymap.md:24-30`) |
| **Transience** — the register is **persistent chrome** in the record: every disposition is legible in the section itself, nothing collapsed and nothing gated behind a link the reader must follow to learn *whether* something was found. The deeper tier — the captured command output, the source sections' own bodies — is reached on demand through `file:line` citations and a companion note in this story's folder | AC-002, AC-006, AC-007 | Every conclusion legible in the section; every conclusion also citable to a deeper artefact. A row whose disposition can only be learned by opening the companion note = fail |
| **Density budget, with its numbers** — exactly **7** columns; exactly **1** row per finding; exactly **6** input-sweep lines, one per source at `_storymap.md:62`; exactly **10** risk-sweep lines, one per `project.md:293-306` row; **5** subsections, no sixth; **0** cells whose evidence is the author's assertion; section body within roughly **150** lines, transcripts excluded and linked | AC-001, AC-004, AC-007 | Row, column and line counts over the rendered section; citation sweep for the zero-assertion figure |
| **Hierarchy** — the input sweep comes **first** (it establishes what the register is over, before any row is read), the register table second, the two sweeps that *add* rows third and fourth, and the zero-fix proof **last**, because it is the conclusion the whole section is evidence for — never a preamble | AC-007 | Subsection-order check |
| **Anti-pattern: the silent repair** — resolving a cross-project disagreement inside this project's own planning artefacts and reporting a clean register (`discover.md:37-39`) | AC-005 | Both-sides observation check + `git diff --name-only` excluding `../_decomposition.md` and `publication-and-positioning/**` |
| **Anti-pattern: the volunteered-only register** — listing what the upstream sections happened to raise and calling it complete (`project.md:293-306` exists because of this) | AC-001, AC-004 | Bidirectional union check + the ten-row pre-registered sweep; either alone lets it through |
| **Anti-pattern: the destination that is not a destination** — a lane with no referent, a "will be opened", or an item id that does not exist | AC-003 | Destination-resolution check |
| **Anti-pattern: the loose companion file** — a register that exists but is not in the artefact the reader opens (`_storymap.md:24-30`) | AC-007 | Mount check |

## Error conditions

| id | Condition | Required behaviour |
| -- | --------- | ------------------ |
| EC-001 | `_closeout-record.md` does not exist at execution time — `clean-checkout-harness` (merge order 1, `_storymap.md:140-143`) has not landed | **Halt and report a blocking finding against slice 1.** Do not create a substitute landing place: an artefact per story is exactly the "fourteen ledger entries in fourteen places" failure the convergence exists to prevent (`_storymap.md:26-30`) |
| EC-002 | One of the six input sections is absent, or its story has not landed | The input sweep names the source, states that it could not be read, and raises a **blocking finding against that story**. It never reports zero findings for an unread source, and it never claims completeness for the union (`_storymap.md:85-88`) |
| EC-003 | An upstream finding entry carries no proposed destination, or its proposal does not fit any of DR-12's three lanes | The register assigns the lane itself — that is this story's job — and states the test it applied. If the entry is too thin to route (no expected/observed pair, no owner discernible), the row records that and raises a **finding against the upstream story** for thin evidence, citing the risk `project.md:303` pre-registered. It never invents a fourth lane |
| EC-004 | Two or more sources raise the same underlying defect | **One row, both source entries named** in *Surfaced by*. Two rows double-count the finding and make the bidirectional check pass on an inflated union; dropping one silently breaks AC-001's reverse direction |
| EC-005 | A lane-2 finding's owning sibling project is already closed out | Route it under `support` (HS-I0005, `.redkiln/config.yaml:5`) with the closed sibling named in the row and the reason stated. **Do not reopen a closed item**, and do not edit its closeout record — the CLI owns every transition (`CLAUDE.md` *Where the work lives*) |
| EC-006 | A finding implicates a `[FROZEN]` clause of `spec/SPECIFICATION.md` or an accepted atom under `.kb/decisions/` | Lane 3. The row names the clause id or atom, the commitment that would move, the owner of the new decision atom and that a re-plan is owed, and hands the work back to the ADR queue (`RUNBOOK.md:262-284`). **Zero writes under `.kb/`, zero edits to `spec/SPECIFICATION.md`**; the immutability rule and the repair/amendment discrimination are `.kb/decisions/README.md:8-23`'s, not this story's to apply by editing |
| EC-007 | The zero-fix diff shows a path outside the stated allow-list | **AC-013 has failed at the project grain, and that is the register's most important row.** Name the commit, the path, the owning story and what it changed; route it. Do **not** revert the commit here, and do **not** widen the allow-list to absorb it — widening the allow-list to fit the diff is the inverse of the story |
| EC-008 | A `.kb/` write in the range is hand-authored rather than produced by `/redkiln:kb-ingest` | The same failure wearing a different hat (`project.md:300`; the reverted `0269720`). The row names the commit and the atom path and routes it; the atom is neither deleted nor re-ingested here |
| EC-009 | The evaluator/DT-1 sides disagree when read at register time | A lane-2 finding against HS-P0016, quoting both sides. **Neither side is edited** to agree with the other — that is the wrong implementation this story is written against (`discover.md:37-39`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`) |
| EC-010 | Writing this section would require re-wording, re-ordering or truncating a sibling's section of `_closeout-record.md` | It does not. Append only. If a sibling's section is wrong, that is a row in this register, not an edit to that section (AC-007's additions-only check is what catches the temptation) |
| EC-011 | A destination item id was expected to exist and does not resolve on the tree | The row is `pending`, not `opened`, and carries the invocation and the owner. A cited id that does not resolve fails AC-003 outright; the honest downgrade is always available and always cheaper |
| EC-012 | Commits land inside HS-P0019 between this story's computation and its merge | The section states the exact `<base>..<head>` SHAs it computed over, so the proof is honest about *what* it measured. A commit after the stated head is the next reader's problem, not a silent inaccuracy in this one |
| EC-013 | Nothing surfaced from any of the six sources and the sweeps add nothing | A permitted and real outcome. Write the explicit swept-zero statement (AC-007) — six sources read and counted, ten risk rows checked, the diff clean at the stated SHAs — never an absent or empty table (`_storymap.md:20-22`) |

## Non-functional

| id | Requirement | Why, and how it is observed |
| -- | ----------- | --------------------------- |
| NF-001 | **Every normative cell carries a resolvable citation, and the citation is checked against its referent, not merely its address.** Zero cells whose justification is the author's assertion | The reader this project exists for cannot re-derive the evidence (`_storymap.md:17-22`); an uncited row asks them to. `.kb/playbooks/verify-the-referent-and-report-coverage.md` is the standing rule that resolving an address proves nothing about the content attributed to it. Observed by AC-002's citation sweep |
| NF-002 | **Read-only outside the mount.** The only writes are `_closeout-record.md` and this story's own folder — zero bytes under `.kb/`, `.bklg/support/`, any sibling project directory, `spec/`, or any crate | It is the property that makes the register falsifiable from the diff alone, by someone who does not read the prose. Enforced by the PR-boundary block via `redkiln verify --grain story` and asserted in the section itself |
| NF-003 | **Reproducible.** Full 40-character SHAs for both ends of the commit range — never `main..HEAD`, never an abbreviation — and every command quoted as run | The initiative branch is deleted at closeout; a range written as `main..HEAD` stops resolving the day after it is written. AC-006's re-derivable diff check |
| NF-004 | **Legible in one sitting.** The section body stays inside the density budget above, with transcripts linked from the companion note rather than pasted | The mount is shared with the whole project's closeout evidence; a section that re-derives its inputs doubles the record and halves the chance it is read (`_storymap.md:24-30`) |
| NF-005 | **Register ids are stable, unique across the whole record, and traceable in both directions** — each carries the source's own finding id so a reader goes row → source section → row | `initiative-closeout-readiness` (HS-S0135) cites these ids to claim nothing is unowned; a renamed id after that citation breaks its record. `../decision-atom-audit-table/spec.md:299` (NF-006) is the upstream half of the same contract |
| NF-006 | **No new tooling, dependency, CI step, fixture or script.** The register is `git`, six markdown sections and a table, using commands the project already declares (`../_decomposition.md:96-107`) | A project whose deliverable is re-observation must not grow an instrument only it uses; `../_decomposition.md:109-123` — nothing is mocked, and nothing new is built to check what already exists |
| NF-007 | **No stage transitions, no item-frontmatter edits, no `redkiln new`.** The implementing agent authors bodies only | `CLAUDE.md` *Where the work lives*: the CLI is the only writer of an item's system frontmatter, and a `PreToolUse` hook denies the edit. It is also *why* the `opened`/`pending` distinction exists at all (Context pack decision 3) |
| NF-008 | **Claim width.** No sentence generalises "zero were fixed inside this project" into "nothing is broken". The register asserts routing and boundary, never health | The health verdict is `backlog-and-kb-health-at-closeout`'s (AC-011/AC-012) and the gate colour is `whole-gate-green-on-the-assembled-tree`'s. Overclaiming here would make a clean register read as a clean tree |

## Implementation notes (non-prescriptive)

Shape suggestions only. Anything here loses to an AC row, to DR-12 (`project.md:179-183`), or to
`.kb/decisions/README.md`.

- **Build the union mechanically before writing any prose.** Open the six sources in the order at
  `_storymap.md:62`, list every finding entry by its own id into a scratch list in this story's
  folder, and only then draw the table. A register written finding-first drifts toward the
  volunteered-only anti-pattern, because the sources that found *nothing* are exactly the ones that
  do not announce themselves — and a source that found nothing still owes a sweep line.
- **Do the risk sweep as a second, independent pass**, straight down `project.md:293-306` without
  looking at the union. The two passes are meant to disagree: anything the risk pass turns up that
  the union did not is the finding the upstream story missed, and anything the union has that no
  risk row anticipated is worth a sentence about why the pre-registration was incomplete.
- **Read both sides of the evaluator/DT-1 question on the tree, at register time, before writing the
  self-audit row.** The two amendments were written on the same day by the same hand
  (`../_decomposition.md:150-153`; `../../publication-and-positioning/_design.md:221-222`), which
  makes agreement likely and makes assuming it exactly the failure `discover.md:37-39` describes.
  Quote the operative sentence from each; the row is short when they agree and it still exists.
- **Fix the commit range first, and state it before running anything.** The base is the SHA the
  project's first story branched from — `clean-checkout-harness`'s recorded harness row is the
  cheapest place to find it — and the head is the tree this story is computing on. Run
  `git diff --name-only <base>..<head>` once and classify the output by path prefix; classifying by
  commit message is what lets a "docs: tidy" commit through.
- **Expect `.redkiln/telemetry/events/**` in the diff and allow-list it deliberately, with its
  citation** (`.redkiln/config.yaml:12`, `auto_stage_telemetry: true`). A register that reports it
  as a fix has produced a false finding; one that quietly drops it has an unstated allow-list. Say
  it out loud, with the reason.
- **Write the destination and its state in the same pass as the row.** A row whose destination is
  left for later is the pressure point `project.md:299` names, one step removed: by the time you
  come back, "it was already handled" is the cheapest thing to write.
- **When a row is genuinely tempting to fix, write the row and stop.** The cheapest defence against
  the project's own named risk is that the register is populated *before* anything is contemplated,
  not after. A repair discovered mid-section costs the story its acceptance criterion; a routed
  finding costs a sibling twenty minutes.

## Tests and CI (merge gate)

Grounded in the testing brief's AC-013 row (`../_decomposition.md:56`, tier **Process**) and its
merge-gate command block (`../_decomposition.md:93-107`). Every project-level command runs against
the **clean-checkout tree slice 1 produced**, not the working tree these artefacts are written in
(`../_decomposition.md:105-107`), and `--fast` is never substituted (DR-2).

| Tier | Command / path | Proves |
| ---- | -------------- | ------ |
| **Process** | `git diff --name-only <project-base-sha>..<closeout-head-sha>`, output quoted, classified against the stated allow-list | The zero-fix claim over **all eleven** HS-P0019 stories rather than over this story's own diff. **AC-006**; the mechanism behind `project.md:237-240` |
| **Process** | `git log --diff-filter=A --format=%H -- .kb/product/ .kb/maps/` over the same range, cross-read against `product-atom-promotion-via-kb-ingest`'s recorded ingest commit | The `.kb/` writes inside the allow-list arrived through `/redkiln:kb-ingest` and not by hand. **AC-006**, **EC-008**; `project.md:215-221` (AC-008) |
| **Process** | `git diff --stat -- .kb/ .bklg/support/ .bklg/from-contract-to-published-library/publication-and-positioning/` over this story's commit range — empty | This story routed without performing: no atom written, no support item opened by hand, no sibling artefact edited. **AC-003**, **AC-005** |
| **Process** | `git diff -- .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — additions only | Mounted in place without occluding the five sections already there. **AC-007** |
| **Static** | Bidirectional union sweep over the six sources at `_storymap.md:62` — each source's finding entries listed by id and set against the register's *Surfaced by* column, both differences shown | Completeness computed and coverage reported, not asserted. **AC-001**; the standing rule at `.kb/playbooks/verify-the-referent-and-report-coverage.md` |
| **Static** | Citation-resolution sweep — every `file:line` in the section opened and the described finding present *at that location* | The register cites referents, not addresses. **AC-002**, **NF-001** |
| **Static** | Destination-resolution sweep — every `opened` id resolved to a file on the tree; every `pending` row carrying a runnable invocation and a named owner | No fabricated ids, no lane without a referent. **AC-003**; `.bklg/support/initiative.md` is the lane-1 parent as it exists |
| **Static** | Ten-row sweep against `project.md:293-306`, each line marked surfaced/did not surface with cited evidence | "Nothing was found" is distinguishable from "nobody looked". **AC-004** |
| **Static** | Both-sides read of `../_decomposition.md:148-242` and `../../publication-and-positioning/_design.md:221-243`, each operative sentence quoted with its `file:line` | The self-audit was performed rather than assumed, and neither side was edited. **AC-005**; the discriminator against `discover.md:37-39` |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story-grain gate. This story maps to no workspace package, which is exactly why the command also runs the five file-reading lints and `spec-trace` unconditionally (`.redkiln/config.yaml:30-40`) — a purely package-shaped gate would compile nothing and call it green |
| **Story grain** | `redkiln verify --grain story` with `_ledger.md` (`.redkiln/config.yaml:67`, `:73`) | Every AC-### has a ledger row with real cited evidence and a recorded work commit; the PR-boundary block is enforced against the changed-file set |
| **Project / terminal grain** | `cargo xtask ci` (`.redkiln/config.yaml:60`) — **not this story's to run** | The tree this story measures is the one `whole-gate-green-on-the-assembled-tree` ran the whole gate on (`_storymap.md:54`). Cited, never re-run: a second run produces a second SHA and a second answer to a question asked once |
| **Project grain** | `redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json` — **not this story's to run** | Consumed from `backlog-and-kb-health-at-closeout` (AC-011/AC-012), one of this story's six inputs. Its findings are register rows; its commands are not re-issued here |

**Merge gate for this story** = the four Process rows + the five Static rows + the two Story-grain
rows, all green, with the three-tree `git diff --stat` empty and `_closeout-record.md` additions-only.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation inside this PR |
| ---- | ------------------- | ------------------------- |
| **Pressure to fix what the register finds** — the project's own named risk (`project.md:299`), at its sharpest here because this is the last story that could quietly absorb one | High / fatal to the AC | AC-006 makes the boundary observable over the whole project rather than aspirational; AC-005 extends it to planning artefacts, which is where the temptation actually lives; EC-007 says name it, do not revert it and do not widen the allow-list |
| **The silent planning-artefact correction** — resolving a cross-project disagreement in this project's own briefs, which changes no code and therefore reads as clean | Medium / silent failure | AC-005's both-sides check plus the `git diff --name-only` exclusion of `../_decomposition.md` and `publication-and-positioning/**`. `.kb/governance/rewrite-the-referent-never-the-reasoning.md` supplies the test: does the edit change what the document *asserts*? |
| **Six inputs, six chances to be incomplete** — a register can be internally perfect and cover five sources | Medium / defeats the whole story | AC-001's bidirectional union with per-source counts; EC-002 turns an unreadable source into a blocking finding rather than a zero |
| **The mount is not there, or a sibling's section is missing** | Low / blocking | EC-001 and EC-002. The merge-order precondition is stated under *Dependencies* so it cannot be discovered at implementation time |
| **The allow-list is discovered rather than stated** — `.redkiln/telemetry/events/` and the ingest-produced `.kb/` writes both look like violations until they are named | Medium / false finding or unstated exception | AC-006 requires the allow-list *before* the result, with citations (`project.md:215-221`; `.redkiln/config.yaml:12`) |
| **Routing something whose owner is already closed** | Medium / a row with nowhere to go | EC-005: `support` with the closed sibling named. `.bklg/support/` is a bare stub (Context pack decision 4) and the row says so rather than inventing a project under it |
| **A lane-3 finding invites writing the atom** — the fastest way to "close" a frozen-clause finding | Low / immutability breach | EC-006 and the PR boundary. `.kb/decisions/README.md:8-13` makes an accepted body immutable and `validate --kb` enforces it against `HEAD`; hand-authoring an atom outside the ingest path is separately forbidden (DR-8, `0269720`) |
| **Register-id churn breaks the slice-mate.** `initiative-closeout-readiness` cites these ids to claim nothing is unowned | Medium / rework in the same slice | NF-005; both stories are implemented in one context (`_storymap.md:85-88`), so a rename is a visible contract change rather than a silent one |
| **A moving tree** — commits land inside HS-P0019 after the diff is computed | Low / accuracy | NF-003 / EC-012: both SHAs stated in the section |

**Coupling**, all read-only except the mount: the six input sections of `_closeout-record.md` (read),
`project.md` / `../_decomposition.md` / `_storymap.md` (read), `.bklg/support/initiative.md` (read),
`../../publication-and-positioning/_design.md` (read), `.kb/decisions/README.md` and the `.kb/`
governance and playbook atoms (read), `RUNBOOK.md` (read), `git` over the project's range (read),
`_closeout-record.md` (append-only). No crate, no `spec/SPECIFICATION.md` clause, no conformance
rule, no CI job, no template and no item frontmatter is touched.

## Dependencies

**Blocks-on (declared `depends_on`, `_storymap.md:62`)** — six edges, and they are the input list,
not a schedule:

| Story | What this story takes from it |
| ----- | ----------------------------- |
| `whole-gate-green-on-the-assembled-tree` (HS-S0126) | The **Gate run** section and its skips-and-findings list — a `skipped` `cargo hack` or `cargo deny` is a finding, not a footnote (`../whole-gate-green-on-the-assembled-tree/spec.md:325`) |
| `dod-set-re-observation-record` (HS-S0127) | The fourteen-row DoD table and its findings list, whose destinations are explicitly **proposals** this story converts into dispositions (`../dod-set-re-observation-record/spec.md:196-197`, `:210`) |
| `published-tree-delta-statement` (HS-S0128) | The `## DoD 13 — the published-tree delta` section; a finding only in the negative case its own EC-006 defines (`../published-tree-delta-statement/spec.md:215`) |
| `decision-atom-audit-table` (HS-S0129) | The audit table's findings list under stable `F-##` ids, made stable precisely so this story can address them (`../decision-atom-audit-table/spec.md:299`) |
| `open-question-preservation-audit` (HS-S0130) | The preservation audit's findings list, including its EC-009 disagreement-with-the-DoD-table case, which it routes here rather than reconciling (`../open-question-preservation-audit/spec.md:325`) |
| `backlog-and-kb-health-at-closeout` (HS-S0133) | The **Backlog and KB health** section — a `problem`, a `process-drift` warning, a seventh advisory or a red `validate --kb` arrives here as a finding, never as a repair (`../backlog-and-kb-health-at-closeout/spec.md:186-193`, `:262-263`) |

**Merge-order precondition, distinct from a dependency.** The mount `_closeout-record.md` is stood up
by `clean-checkout-harness` in slice 1 (`_storymap.md:53`), which merges first (`_storymap.md:140-143`).
That is a sequencing fact rather than a declared edge, and it is handled by EC-001 — not by adding a
seventh `depends_on` this story does not have.

**Unlocks**

| Story | Why it waits on this one |
| ----- | ------------------------ |
| `initiative-closeout-readiness` (HS-S0135, project AC-014) | Declares `depends_on: findings-disposition-register` (`_storymap.md:63`). It cannot claim the initiative is closable while a finding is unowned; the register is what makes "nothing is unowned" a citable statement rather than an impression (`_storymap.md:85-88`) |

Nothing outside this project waits on it. `blocks` is empty at the project grain by construction
(`project.md:286-291`), and the routed items are opened by their owners, through the CLI, after this
story merges.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these when the bound AC says to — link, do not pre-read.

| Anchor | Why it is load-bearing | When to open | Serves |
| ------ | ---------------------- | ------------ | ------ |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` (`:179-183` DR-12; `:237-240` AC-013; `:262-263` DoD 6; `:293-306` the risk table; `:215-221` AC-007/AC-008) | The three lanes and their tests, the AC's negative half, and — uniquely — the **pre-registered** list of finding shapes that makes AC-004 possible. `:215-221` is the authority for the `.kb/` half of the zero-fix allow-list | `:179-183` before assigning the first lane (AC-002/AC-003); `:293-306` as its own second pass (AC-004); `:215-221` before stating the allow-list (AC-006) | AC-002, AC-003, AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` (`:17-22` the reader and the "a command was run" bar; `:24-30` the convergence rule; `:62` the six inputs; `:85-88` the slice contract; `:92-94` no story fixes anything) | Names the six input sources authoritatively, fixes the mount, and states the bar a swept zero must meet | `:62` before building the union (AC-001); `:24-30` before mounting (AC-007) | AC-001, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` (`:56` the AC-013 tier; `:93-107` the merge-gate commands and the clean-checkout rule; `:148-180` and `:227-242` the amended evaluator decision) | Fixes the tier (**Process**) that proves AC-013 and states that every command runs on the clean-checkout tree. `:148-242` is **one of the two sides** AC-005 must read | `:93-107` before running anything; `:148-242` immediately before writing the self-audit row (AC-005) | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` (`:221-243` the DT-1 rider, incl. "Net effect on this project: none") | The **other** side of the evaluator/DT-1 question. AC-005 is unsatisfiable without it, and assuming its content instead of reading it is precisely the named wrong implementation | In the same sitting as the first side, before writing the self-audit row (AC-005) | AC-005 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/findings-disposition-register/discover.md` (`:37-39` the wrong implementation; `:30` the answered routing question) | The named wrong implementation is AC-005's acceptance test in prose — a register that passes AC-001, AC-002 and AC-006 and still fails | Before declaring the register finished (AC-005) | AC-005 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | The repository's own standing rule for any check over cross-references: verify the referent and not merely the address, and **report your own coverage**. It is why AC-001 demands per-source counts instead of a completeness claim, and why AC-002's citations are opened rather than resolved | Before writing the input sweep (AC-001); again before the citation pass (AC-002) | AC-001, AC-002 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | Supplies the mechanical test AC-005 applies one layer out from the corpus: ask whether the edit changes what the document **asserts**, not whether it changes the document. It is what distinguishes a harmless planning-artefact tidy from a silent cross-project correction | The moment an edit to a planning artefact looks harmless (AC-005) | AC-005 |
| `.kb/decisions/README.md` (`:7-18` immutability; `:20-23` repair vs. amendment; `:25-33` what belongs and the "state what lost" bar) | Lane 3's whole authority: an accepted body is never edited, `validate --kb` enforces it against `HEAD`, and "filing an amendment as a repair is how a frozen commitment quietly moves" | Before writing any lane-3 row (AC-003); before touching anything on discovering a `.kb/` gap (EC-006) | AC-003 |
| `.redkiln/config.yaml` (`:5` `support_initiative`; `:12` `auto_stage_telemetry`; `:40` the affected gate; `:60` the terminal grain; `:67`/`:73` ledger and commit provenance) | `:5` is lane 1's literal destination; `:12` is why `.redkiln/telemetry/events/` legitimately appears in the diff; `:67`/`:73` are why `_ledger.md` and a recorded work commit are mandatory rather than customary | `:5` before the first lane-1 row (AC-003); `:12` before stating the allow-list (AC-006); `:67`/`:73` before opening the PR | AC-003, AC-006 |
| `.bklg/support/initiative.md` | HS-I0005 **as it actually is** — `status: exploring`, `stage: intake`, no project and no story under it. A lane-1 row must name what exists; inventing a project here is the hand-authored-backlog failure `0269720` was reverted for | Before writing the first lane-1 destination (AC-003) | AC-003 |
| `RUNBOOK.md` (`:262-284` the ADR queue) | Where a lane-3 finding actually lands: the queue is the repository's record of which decisions are owed and to whom, and it is the referent a "new decision atom plus a re-plan" points at | While writing a lane-3 row's owner and re-plan (AC-003) | AC-003 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` (`:41-45` `## Items` N/A; `:93-100` the sign-off) | The signed-off design binds as a **prohibition**: no surface is invented here, and the perceptual review is a *declared* skip. It is what makes the Interaction-quality N/A rows legitimate rather than omissions | Before writing anything that looks like a surface (AC-007) | AC-007 |
| `.bklg/from-contract-to-published-library/initiative.md` (`:194-195` incidental bugs route to `support`; `:558-559` the assumptions; `:580-581` exit criterion 8) | The initiative-grain reason the register exists: "what was learned has been harvested rather than left in the backlog". A finding absorbed here lets "every project is closed out" be true while the defect stays unowned | Before writing the section's opening paragraph (AC-001, AC-007) | AC-001 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` (`:41-50` the `0269720` precedent; `:115-145` tensions to flag rather than silently resolve) | The standing instruction behind "record, do not repair", and the reason a bare `.bklg/support/` is an observation rather than a gap to fill | When a finding looks cheaper to fix than to route (AC-005, AC-006) | AC-006 |

## Clarifications resolved during spec

1. **How many AC-### this story carries: seven, exactly the ids the first pass fixed.** The front
   half's *Behavior and interfaces* table has fourteen rows; several state one obligation twice (row
   shape and lane test; destination concreteness and lane-1's bare parent; upstream sections read
   never rewritten, which is the same obligation as non-occlusion). They fold into AC-001…AC-007
   with nothing dropped — the read-only-outside-the-mount row becomes **NF-002** rather than an AC
   because it is a property of *how* the story is done, and "no public API surface" becomes the
   Interaction-quality prohibition rather than a criterion, because there is nothing for a reader to
   observe in the record. The ledger carries exactly these seven ids.
2. **`backlog-and-kb-health-at-closeout`'s heading is no longer unknown.** The front half (Context
   pack decision 2) recorded it as "heading not yet fixed" because that story's `spec.md` was still
   a rendered stub at authoring time. It is now written and names its mount: the **Backlog and KB
   health** section of `_closeout-record.md`, created empty-with-placeholders by
   `clean-checkout-harness` and filled by that story (`../backlog-and-kb-health-at-closeout/spec.md:186-193`).
   The instruction that produced the caution stands unchanged and is why it is stated here rather
   than edited into the front half: **discover the heading by reading the record**, because the
   spec is planning-time evidence and the record is execution-time fact.
3. **The zero-fix allow-list gains a third, explicitly cited entry.** The front half named two
   (`.bklg/**`, and the ingest-produced `.kb/` writes). `.redkiln/config.yaml:12` sets
   `auto_stage_telemetry: true`, so a `redkiln` run inside the range leaves a tracked footprint under
   `.redkiln/telemetry/events/` — which is neither a fix nor an unstated exception. AC-006 requires
   all three stated **before** the result, with citations. This is an addition to the list, not a
   widening of it: EC-007 explicitly forbids widening the allow-list to absorb a violation.
4. **Discover's first deferred question — "is the evaluator/DT-1 discrepancy the only known
   finding?"** — is resolved as a *procedure* rather than a prediction, as discover itself proposed
   (`discover.md:29`). The row shape is fixed here (AC-002); population happens at execution against
   the six sources (AC-001) and the ten pre-registered risk shapes (AC-004). The evaluator/DT-1 case
   is not carried as a pre-written row but as the **self-audit obligation** of AC-005, because its
   verdict depends on reading both sides at register time — and at planning time both sides say the
   same thing (`../_decomposition.md:148-180`; `../../publication-and-positioning/_design.md:221-243`,
   "Net effect on this project: none"), which makes *reconciled at planning, no item owed* the likely
   disposition and assuming it the likely failure.
5. **Discover's second question — "what destination does the evaluator/DT-1 finding get?"** — is
   narrowed. Discover answered "a new item against HS-P0016" (`discover.md:30`); the register can
   only reach that disposition if the two sides actually disagree when read. AC-005 therefore admits
   **two** outcomes, both of which are rows: reconciled-with-citations, or a lane-2 finding against
   HS-P0016 (falling to `support` per EC-005 if that project is already closed out). What discover
   forbade is unchanged and is the only outcome AC-005 rejects: no row at all.
6. **`_closeout-record.md` is the mount and is deliberately absent from the anchors table.** It does
   not exist on the tree at spec time — `clean-checkout-harness` creates it — so citing it as a
   progressive-disclosure anchor would hand the implementer a dead path. It is named in the
   *Integration contract*, its absence is EC-001, and the six sections this story reads are reached
   through it at execution time.
7. **No conformance rule and no `spec/SPECIFICATION.md` clause is added, and that is a decision
   rather than an omission.** No rule can express "a defect found at closeout has a named owner",
   and inventing one would be the decorative rule `CLAUDE.md` forbids — one no adapter could fail.
   The instruments are `git`, six markdown sections and a human-readable table.
