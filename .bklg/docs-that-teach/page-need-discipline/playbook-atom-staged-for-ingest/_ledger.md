---
item: HS-S0153
stage: implement
created: 2026-08-17T13:16:11.865Z
updated: 2026-08-17T13:16:11.865Z
---

# Acceptance ledger — The playbook atom staged under .kb/_intake/, never hand-authored into .kb/

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Two things about this ledger in particular**, both from the spec and both easy to violate by
accident:

1. **`redkiln validate --kb` green is AC-010's evidence and nothing else.** The validator skips every
   `_`-prefixed directory by design (`.kb/_intake/README.md:21-27`), so it is silent about the staged
   file's own frontmatter. Citing it under AC-002, AC-003 or AC-005 is EC-005 and is rejected at
   review.
2. **Six of the eleven rows are discharged by command transcripts, and two require a non-author**
   (AC-007's rejection cross-walk and AC-009's portability walk). Capture the transcripts as the
   commands are run, and schedule the two walks early — they are a scheduling constraint on the
   slice, not a formatting one.

The evidence sections referenced below (`§Mount`, `§Frontmatter walk`, `§Mechanical negatives`,
`§Summary adjudication`, `§Resolution transcript`, `§Non-occlusion`, `§Method`, `§Rejection
cross-walk`, `§Commitment audit`, `§Portability`, `§Divergence`, `§Composition`) are written into the
implementation report and cited from the `evidence` field by name plus `file:line`.

`<atom>` stands for `.kb/_intake/lesson-page-need-declaration-discipline.md` throughout.

```yaml
- id: AC-001
  criterion: "GIVEN the ingest operator runs /redkiln:kb-ingest with no argument at initiative closeout, WHEN the default glob .kb/_intake/*.md is expanded (.kb/_intake/README.md:3-5), THEN exactly one new payload file — .kb/_intake/lesson-page-need-declaration-discipline.md — is swept in beside the directory's README, under a `lesson-` name the operator reads as payload rather than scaffolding, and it is registered in no manifest, index or config anywhere else"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "git diff --name-status main -- .kb/_intake (exactly one A line, and it is <atom>) + ls .kb/_intake (exactly two files) — transcripts in the implementation report §Mount, which also names <atom> in full for HS-P0025's approval gate"

- id: AC-002
  criterion: "GIVEN the ingest operator adjudicating this file against the corpus, WHEN they read its first six frontmatter lines and nothing else, THEN they see kind: playbook, authority_tier: guideline and status: proposed — a finished proposal awaiting a decision that is theirs to take, neither claiming an acceptance no gate conferred nor inviting a full re-derivation from raw material"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "rg -n '^(kind|status|authority_tier):' .kb/_intake/lesson-page-need-declaration-discipline.md — transcript verbatim in §Frontmatter walk rows 3-5, each carrying the one-sentence reason `accepted` and `draft` lost (spec Context pack 4)"

- id: AC-003
  criterion: "GIVEN that redkiln validate --kb skips every _-prefixed directory by design and is therefore silent about this file (.kb/_intake/README.md:21-27), WHEN the implementer closes the story, THEN _ledger.md carries a field-by-field walk of all ten required fields against .kb/README.md:16-26 — id, title, kind, status, authority_tier, summary, depends_on, related, source_paths, last_reviewed — each row citing the staged file's own line number, and no row of that walk cites the green validate --kb run as its evidence"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Frontmatter walk has ten rows, each with a <atom>:NN cite; the AC-003 evidence string contains no `validate --kb` (EC-005). The green CLI run is recorded separately in §Mechanical negatives, labelled as AC-010's evidence only"

- id: AC-004
  criterion: "GIVEN the ingest operator at the approval gate, who must decide merge-or-spawn before opening the body, WHEN they read summary alone, THEN it adjudicates: it states the problem, the properties that decide the design, the options ruled out and the discriminator, composed as the corpus's playbook summaries are — a paragraph in the folded-block form, not a one-line label and not a restatement of the title"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Summary adjudication — side-by-side against .kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:7-15 and .kb/playbooks/anchoring-citations-in-a-long-lived-document.md:7-15, with the four elements quoted out of the drafted summary and its sentence count recorded"

- id: AC-005
  criterion: "GIVEN a reader who does not yet trust the atom, WHEN they follow any entry in source_paths or any id in depends_on / related, THEN every path resolves in the tree at merge and every id resolves to an atom that exists — so the wave's merge bias has somewhere real to land rather than a near-duplicate to spawn"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Resolution transcript — one `test -f` line per source_paths entry (all present) and one `rg -n \"^id: <id>\" .kb` line per depends_on/related id (each a single hit), including the four pre-resolved ids at .kb/playbooks/verify-the-referent-and-report-coverage.md:2, .kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:2, .kb/playbooks/anchoring-citations-in-a-long-lived-document.md:2, .kb/governance/rewrite-the-referent-never-the-reasoning.md:2"

- id: AC-006
  criterion: "GIVEN the practitioner on a different part of the system, who has this problem and twenty minutes, WHEN they open the atom in a GitHub blob view and read top to bottom clicking nothing, THEN the four-step method, the evidence that grounds each step, and a named section stating the conditions under which the discipline stops holding are all visible in document order — none of the three behind a <details>, a fold, a tab, or a link to another file"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "rg -n '<details>|<summary>|<!--' .kb/_intake/lesson-page-need-declaration-discipline.md returns no hit — transcript in §Non-occlusion; §Method names the heading carrying the stop conditions (corpus shape: `## Where this is the wrong instrument`, .kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:98) and quotes at least two named conditions verbatim"

- id: AC-007
  criterion: "GIVEN the next page author asking why the discipline is shaped the way it is, WHEN they reach the discarded-alternatives section, THEN each of the five rejection families is present with the reason it lost — the four rejected homes for the tree, the five rejected declaration forms, DT-2's three rejected need taxonomies, DT-3's two rejected findability options, DT-8's two rejected fold policies — each compressed and cited to _design.md rather than re-litigated, and no rejection appears that the design did not take"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Rejection cross-walk — five rows, each naming the family, the atom's line and a .bklg/docs-that-teach/page-need-discipline/_design.md line cite; walked by a NON-AUTHOR who confirms every rejection in the atom maps to a row and that no sixth family was invented (.kb/playbooks/README.md:15-20)"

- id: AC-008
  criterion: "GIVEN a reader looking for what actually binds them, WHEN they search the atom for a commitment, THEN they find none: no must / shall / must not whose reversal would take a decision, and every binding claim appears instead as a cited RP-NN-N rule id whose text is not restated — so the enforceable half stays where the gate reads it and a commitment is not stripped of the immutability that makes it enforceable"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "rg -n '\\b(must|shall|MUST)\\b' .kb/_intake/lesson-page-need-declaration-discipline.md — transcript in §Commitment audit, adjudicated line by line (each surviving hit annotated as method, not commitment); every RP- id in the atom checked against the shipped standards/pages/ rule text for restatement (.kb/playbooks/README.md:29-31,37-38; project.md DR-11)"

- id: AC-009
  criterion: "GIVEN the practitioner outside this initiative applying the atom to their own corpus, WHEN they read it, THEN the subject is the shape — declare the answered need as a singular enumerated property; check the declaration mechanically; name the check's blind spot in its own documentation; supply a written non-author procedure for the judgement it cannot make — with happenstance as the cited instance and at least one recurrence outside this initiative named; and the atom describes standards/pages/ and the checker module as they stand at this story's HEAD, any divergence from _design.md being recorded rather than smoothed over"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Portability — a NON-AUTHOR performs the portability test of .kb/playbooks/README.md:33-35 and records the verdict, the named recurrence and their own identity; §Divergence lists every place the atom's description differs from _design.md, or states 'none observed' explicitly and never blank (.kb/governance/rewrite-the-referent-never-the-reasoning.md)"

- id: AC-010
  criterion: "GIVEN an initiative whose whole thesis is links that resolve, and an ingest that deletes this file on success (.kb/_intake/README.md:13-19), WHEN closeout clears _intake/, THEN nothing that outlives the file points at it and nothing was written into the checked knowledge base ahead of the wave: git diff main -- .kb ':!.kb/_intake' is empty, .kb/maps/domain-map.md is untouched, and the handoff travels as provenance in source_paths and _ledger.md rather than as a cross-tree hyperlink"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Mechanical negatives — git diff main -- .kb ':!.kb/_intake' empty; git diff --stat main -- .kb/maps empty; rg -n \"_intake/lesson-page-need\" -g '!.bklg/**' returns only <atom>; redkiln validate --kb && redkiln doctor green with exactly six template-drift advisories, labelled as proving THIS AC and not AC-003"

- id: AC-011
  criterion: "GIVEN the same atom opened in three places the corpus is actually read — a GitHub blob view, a plain-text pager, and an editor at 100 columns — WHEN it renders in each, THEN it is composed, not merely marked up: the corpus's playbook shape (an H1, ## section headings, run-in bold markers, monospace path:line citations), meaning that survives greyscale (no colour, icon, emoji or badge carries any distinction), no `> **Answers:**` declaration line — this is a .kb atom, not a governed page, and a declaration here would claim a governance the checker's pinned trees do not extend to it — and a body inside the measured corpus envelope: <= 8,192 bytes and <= 100 source columns outside tables"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/lesson-page-need-declaration-discipline.md"
  verifying_test: "§Composition — the atom's heading inventory; wc -c <atom> <= 8192 against the measured corpus (six playbook atoms run 5,161-7,137 bytes, largest .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md); awk '{if(length($0)>m)m=length($0)}END{print m}' <atom> <= 100 against the corpus maximum of 100; rg -n '^> \\*\\*Answers:\\*\\*' <atom> no hit; greyscale legibility confirmed by inspection (_design.md 'Density budget', 'Hierarchy', anti-patterns 2 and 3)"
```
