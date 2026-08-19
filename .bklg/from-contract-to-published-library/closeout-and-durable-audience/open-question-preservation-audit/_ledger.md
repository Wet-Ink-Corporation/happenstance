---
item: HS-S0130
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Zero open-question deletions, every consumed atom resolved and annotated

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: >-
    **GIVEN** the reader will not take "nothing was deleted" on trust and must be able to re-run it, **WHEN** they read the opening of `## Open-question preservation audit`, **THEN** the section states the baseline SHA `B` **derived two independent ways and shown to agree** — the parent of the first commit that added `.bklg/from-contract-to-published-library/`, and `git merge-base main <initiative-branch>` — with both commands and both outputs quoted, `ce933d8` named as the *expected* value that was re-derived rather than copied from this spec; and the closeout SHA `H` stated as **full** SHAs (never a branch name or abbreviation) and shown to be the same commit `clean-checkout-harness` recorded for the gate run, **SO THAT** every later claim in the section is scoped to a range the reader can resolve after the branch is deleted.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, opening paragraph)"
  verifying_test: "Static/process, reviewed not scripted (_decomposition.md:49): both derivation commands and outputs quoted in the record and agreeing; `git cat-file -e <B>` and `git cat-file -e <H>` resolve; `H` string-equal to the SHA in the record's clean-checkout/gate section; no abbreviated SHA present"

- id: AC-002
  criterion: >-
    **GIVEN** an atom deleted in one commit and re-added in the next is invisible to a two-point diff — it renders as `M`, or as nothing at all if the re-add is byte-identical — **WHEN** the reader looks for the deletion evidence, **THEN** the record quotes `git log --no-renames --diff-filter=D --name-status <B>..<H> -- .kb/open-questions/` **verbatim with its empty output**, states the number of commits the walk covered (from `git rev-list --count <B>..<H>`), and says in words that this is a walk over every commit in the range rather than a diff of its endpoints, **SO THAT** "nothing was deleted" is a property of the whole history and not of the two commits someone happened to compare.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, the zero-deletion result)"
  verifying_test: "Process: `git log --no-renames --diff-filter=D --name-status <B>..<H> -- .kb/open-questions/` quoted with empty output, plus `git rev-list --count <B>..<H>`; reviewed check that the quoted command uses `log` with `--no-renames` and not a two-point `git diff` (project.md:211-214; _decomposition.md:49)"

- id: AC-003
  criterion: >-
    **GIVEN** a rename is a deletion of the path every index bullet and `source_paths` entry points at, and rename detection makes it report as `R` instead of `D`, **WHEN** the reader checks that nothing left the directory under another name, **THEN** the same range is re-run with `-M` and every `R` under `.kb/open-questions/` is listed with old path → new path and either shown to have had its index bullet and referrers updated in the same commit (recorded as a deliberate rename, listed, not a finding) or routed as a finding; **and** the atom sets at `B` and at `H` are listed by filename and frontmatter `id` from `git ls-tree`, showing the baseline **nineteen** all present at `H` and naming — not merely counting — any atom the initiative added, **SO THAT** the directory is shown to have only grown, and disappearance-by-rename cannot be absorbed silently.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, rename report and on-disk parity)"
  verifying_test: "Process: the `-M` re-run quoted with output and every `R` carrying both paths; `git ls-tree -r --name-only <B> -- .kb/open-questions/` and the `<H>` form quoted, set difference computed in the record against the baseline 19 atoms (`.kb/open-questions/` excluding README.md); additions named, not counted (EC-007)"

- id: AC-004
  criterion: >-
    **GIVEN** DR-7's six filenames (`project.md:155-161`) were written at planning time and are a **prediction**, not an inventory, **WHEN** the reader asks whether the per-atom table is complete, **THEN** the section states the consumed set **before its first row** and shows how it was computed — every `open_question` atom that a decision atom this initiative wrote names in `related:` / `supersedes:` / its body, taken from the slice-mate's decision-atom audit table, **unioned** with DR-7's six — and lists every asymmetry with a reason: an atom consumed but not predicted gets a row and a note, and a DR-7 atom that turns out **still genuinely open** is recorded as still open with the reason and is **not** flipped to make the prediction true, **SO THAT** the table's coverage is checkable rather than asserted, and the plan is allowed to have been wrong.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, the consumed-set enumeration stated ahead of the table)"
  verifying_test: "Static: consumed set enumerated as atom paths before the first table row; each member traceable to a row in the slice-mate's decision-atom audit table in the same record or to DR-7 (project.md:155-161); both asymmetry directions addressed explicitly, an empty one stated as empty (EC-003, EC-010)"

- id: AC-005
  criterion: >-
    **GIVEN** "resolved" has a fixed vocabulary and an owner — `.kb/open-questions/README.md:42-45` assigns the status flip to whoever wrote the answer, on the commit where it landed — **WHEN** the reader reads a per-atom row, **THEN** it shows that atom's frontmatter `status` **read at `H`** as `withdrawn` or `superseded` together with the `related` edge naming the answering atom; and anything else — a question this initiative answered whose atom still reads `open`, or a resolution state `KbFrontmatter`'s enum cannot express (the precedent is already filed at `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md`, and the row records **which state it wanted**) — is written down verbatim as a finding carrying one of DR-12's three destinations (`project.md:179-183`) and is left **uncorrected in this PR**, **SO THAT** the audit is able to fail, which an instrument that edits what it measures cannot.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, per-atom status column and findings list)"
  verifying_test: "Static: each row's status matched against `git show <H>:<atom>` frontmatter; every non-conforming row carries a destination from DR-12's closed three-value set (project.md:179-183); `git diff --name-only` for this story's commits contains no `.kb/` path (shared with AC-008)"

- id: AC-006
  criterion: >-
    **GIVEN** the one deletion that `--diff-filter=D` reports as clean is the atom rewritten into its own answer — forbidden by `.kb/open-questions/README.md:44-45` ("leave the body describing what was not known at the time") and guarded by nothing, because `validate --kb`'s immutability check binds `status: accepted` **decision** atoms only (`.kb/decisions/README.md:7-13`) — **WHEN** the reader asks whether the surviving atoms still say what was not known, **THEN** each consumed atom's `git diff <B>..<H> -- <path>` is inspected and classified: a change confined to frontmatter (`status`, `superseded_by`, `related`, `last_reviewed`) passes; any change to the prose body is **quoted** in the record and routed as a finding; and the passthrough keys `tracks` / `resolution_ref` are reported as neither stripped nor invented (`README.md:47-50`), **SO THAT** the record of the state of knowledge on the day each choice was made survives the initiative that made the choices.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, body-preservation column)"
  verifying_test: "Static: one body-preservation verdict per consumed atom from `git diff <B>..<H> -- <atom>`, with the hunk quoted wherever the verdict is not 'frontmatter only'; the four permitted frontmatter keys named in the record; passthrough keys reported (.kb/open-questions/README.md:47-50)"

- id: AC-007
  criterion: >-
    **GIVEN** `.kb/maps/open-questions-index.md:12-13` makes "a withdrawn or superseded question stays listed, annotated, rather than removed" a standing rule, and `:136-143` fixes the bullet's shape as status word first (`Open`, `Withdrawn`, `Superseded`), then the id, then one sentence, **WHEN** the reader cross-checks the index against the atoms, **THEN** each consumed atom's row carries **three separately observed verdicts** — the bullet still exists; it leads with a status word from that closed set; and that word **equals** the atom's frontmatter status at `H` — with the three failure modes (missing bullet, unannotated bullet, status disagreement) reported as three distinct findings each with its own destination, and a bullet still reading **Open** above an atom whose frontmatter says `superseded` called out as the specific rot the index exists to prevent rather than fixed in passing, **SO THAT** "not in this index" keeps meaning "never asked" and "Open" keeps meaning open.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, the three index columns)"
  verifying_test: "Static: `.kb/maps/open-questions-index.md` read at `<H>` — bullet presence, status word from the closed set at `.kb/maps/open-questions-index.md:141`, and word-versus-frontmatter agreement recorded as three independent columns; disagreements listed as findings and the index file absent from this story's diff"

- id: AC-008
  criterion: >-
    **GIVEN** "fourteen ledger entries in fourteen places is the failure mode the charter's DoD preamble is written against" (`_storymap.md:24-30`), and given that an instrument which repairs what it measures can no longer report a failure, **WHEN** the reader opens `_closeout-record.md` at `H`, **THEN** exactly **one** new `## Open-question preservation audit` section is present — at the same heading level as, and immediately after, the slice-mate's decision-atom audit table, with that table's rows unmoved, unreworded and un-truncated, and no sibling findings file anywhere in this story's directory standing in for it — every claim in it carrying the command that produced it and both SHAs; **and** `redkiln validate --kb` is re-run on the closeout tree and exits zero, **and** `git diff --name-only` across this story's commits shows **zero paths under `.kb/`**, with this story's `_ledger.md` citing the section by path *and* heading, **SO THAT** the reader gets one sitting and the audit's own diff is the proof of its read-only posture.
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md (## Open-question preservation audit, mounted immediately after the decision-atom audit table)"
  verifying_test: "Static/process: reviewed diff of `_closeout-record.md` — exactly one added `##` section, positioned after the slice-mate's table, slice-mate rows byte-identical; `redkiln validate --kb` exit 0 recorded (.redkiln/config.yaml:56-60); `git diff --name-only` for this story's commits contains no `.kb/` path; `redkiln verify --grain story` against the PR-boundary block and this ledger (.redkiln/config.yaml:67, :73)"
```
