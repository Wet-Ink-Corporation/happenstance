---
item: "HS-S0087"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — DT-1, DT-4, DT-5 and DT-6 are decided, each naming what lost

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story lands **no Rust and no manifest change**, and no conformance rule in
`crates/happenstance-testkit/src/suite.rs` can observe a positioning decision. `verifying_test`
therefore names the real command, the real content review or the real diff assertion that decides the
row — which is exactly the tier the testing brief assigns to project AC-011 (*static presence check
plus content review, not a compiled test*, `publication-and-positioning/_decomposition.md`:474) and
project AC-013 (*static (process) — `redkiln validate --kb` over every atom, each from the ingest
path*, `:476`).

Two `mount_point`s recur, because the story has two. The **atom** is
`.kb/decisions/00NN-first-contact-positioning.md`, written by the `/redkiln:kb-ingest` wave from
`.kb/_intake/`; the **mount** that makes it reachable is its row on `.kb/maps/decision-map.md` — an
atom with no map row passes every frontmatter check and is still invisible to the corpus. AC-001's
mount is different again and deliberately wave-independent: the committed roll-call under this
story's own folder, which is the evidence for project AC-011 whether or not the wave has run yet.

`00NN` stands for the ADR number the wave actually assigns (≥ 0030, free of the 0017–0028 sibling
allocation and of 0029). The implementer replaces it with the assigned id when citing evidence; see
EC-004 in the spec.

```yaml
- id: AC-001
  criterion: "GIVEN the repository owner signed this project's `_design.md` off on 2026-08-12 with no conditions (`_design.md`:772-789), WHEN the release needs project AC-011 (\"None is left unowned at release\", `project.md`:266-269) to be an observed fact rather than an assumption, THEN each of DT-1, DT-4, DT-5 and DT-6 is confirmed to carry a written resolution in `_design.md` that names its rejected option(s) AND the reason each lost, and each closes with a `*Resolves:*` line binding it to the project ACs it discharges (`:245`, `:282`, `:318`, `:345`); AND that confirmation is a dated, committed roll-call artefact under this story's folder — the testing brief's own tier for AC-011 is static presence check plus content review, not a compiled test (`publication-and-positioning/_decomposition.md`:474) — so DoD item 3's committed artefact with a date, not a remembered observation (`project.md`, Definition of done) is satisfied; AND `_design.md` itself is byte-identical afterwards, because a story that edits the artifact it was measured against has destroyed its own evidence"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/first-contact-design-resolutions/ — the committed dated roll-call artefact (wave-independent evidence for project AC-011)"
  verifying_test: "content review: the committed roll-call cites _design.md:191-245, :247-282, :284-318, :320-345 and the four *Resolves:* lines at :245/:282/:318/:345, and records the sign-off date (:772-789) and the amendment date (:222-243); git diff --quiet -- .bklg/from-contract-to-published-library/publication-and-positioning/_design.md at HEAD"

- id: AC-002
  criterion: "GIVEN those four answers live only under `.bklg/`, which is archived when the initiative closes, WHEN a maintainer two releases later asks what does this crate say on first contact, and why, THEN one accepted decision atom at a number >= 0030 (free of the 0017-0028 sibling allocation and of 0029) carries all four winners in their decided shape, not as headlines — DT-1 (a) storage-agnostic proof stated as an act the reader can perform rather than as an adjective; DT-4 (c) both places, the promise stated in full and in the caller's register inside the Guarantees block, as one bullet of <= 3 rendered lines plus one nested line carrying exactly one link into a clause ID rather than a heading; DT-5 (c) a single dated census sentence of <= 3 rendered lines with exactly 4 counts, 4 inline definitions and 1 link, sited immediately after the status callout; DT-6 (a) stated in the existing Prior art slot in full plus a one-sentence, <= 2-rendered-line, one-link reduced form on the packaged README, and never as a feature matrix — AND the atom states why it is one record and not four (the four interlock: DT-1 (c)'s rejection rests on the persona call, DT-5's siting on DT-1's first screen, DT-4's slot on DT-1's demotion, DT-6's reduced form on that screen being full); AND it is valid KbFrontmatter in the decision shape of record (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33): `kind: decision`, `authority_tier: decision`, `status: accepted`, `adr_id`, `phase: 12`, a stated `reversibility`, `supersedes: null`, `superseded_by: null`, `related` carrying `kb-decision-0006` and `kb-open-question-es-38-and-gap-read-unowned-001`, and `source_paths` naming the intake file and `_design.md`"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-first-contact-positioning.md (authored by the /redkiln:kb-ingest wave from .kb/_intake/), reached from .kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb (exit 0, new atom present); content review of .kb/decisions/00NN-*.md for all four winners with their shape clauses (act-not-adjective; Guarantees block, caller's register, <=3+1 lines, exactly one clause-ID link; <=3-line dated census with 4 counts + 4 inline definitions + 1 link, sited after the status callout; Prior art slot in full plus <=2-line one-link reduced form, no matrix), the one-record rationale, and the nine frontmatter fields; git ls-files .kb/decisions/ showing exactly one added path whose number is >= 0030 and outside 0017-0028"

- id: AC-003
  criterion: "GIVEN the named wrong implementation is an atom that states four winners, validates cleanly, is correctly numbered and quietly drops the what-lost-and-why half (`discover.md`:43), WHEN the next maintainer reads the atom's rejected-options sections, THEN all eleven options the initiative's DT table put on the table (`initiative.md`:420, :423, :424, :425) are accounted for — 4 winners and 7 losers — and each loser carries its reason at the strength `_design.md` states it, not a shorter paraphrase: DT-1 (b) lost on the asymmetry (the edge reader self-identifies from one Guarantees line; the general evaluator cannot self-identify from an edge lead at all); DT-1 (c) lost twice (crates.io renders one README per crate and there is no route parameter; ADR-0006's split is by role, not runtime; and it splits one person in half); DT-4 (b) lost on IQ-1's 0-hop budget — a ~5,000-line specification is a context jump the one-sitting reader does not return from; DT-4 (a) lost because the citation is the difference, since a promise with no citation is what two DCB-labelled stores already disagreeing in public can each write; DT-5 (a) lost on density: 200 rows against a ~14-rendered-line first screen at 1024x768, twenty-five times the height of the status table; DT-5 (b) is forbidden, not merely worse — listing 139 frozen guarantees while suppressing 49 provisional ones is IQ-2's filter hiding what it filters and the exact dishonesty BR-06 exists against (AP-4); DT-6 (b) lost because choosing it would mean deleting existing honest copy, including the sentence that sends a reader away"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-first-contact-positioning.md — the rejected-options sections, one per tension"
  verifying_test: "option roll-call reconciling 4 + 7 = 11 against initiative.md:420/423/424/425; content review that each of the seven loser entries carries its distinguishing reason token (asymmetry; one README per crate + role; 0 hops / one sitting; citation; 200 + 14; forbidden / filter hiding what it filters; deleting existing honest copy) traceable to _design.md:202-212, :258-265, :298-303, :328-332; file:line resolution at HEAD of every citation in those sections"

- id: AC-004
  criterion: "GIVEN `_decomposition.md`:310-314 flagged the evaluator-vs-application-author question as open and DT-1 could not be decided without it, AND the owner amended the consequence in place at the spec gate (`_design.md`:222-243), WHEN `closeout-and-durable-audience` later populates `.kb/product/` and reads this atom for the disposition, THEN the atom carries the amended form and only it: the evaluator is a moment in the application author's journey and not a fifth persona atom (the surviving conclusion), and the consequence is that the evaluation path is promoted as its own journey atom linked to Persona 1 — explicitly not a journey stage buried inside Persona 1's journey, and not a fourth persona atom; AND the atom names the argument that carried the amendment (the difference is in the mechanism of trust-building — one-shot public evidence versus revisable contact with the code over weeks — which is what a journey atom is for) and the argument that was discounted (that folding would retroactively make DT-1's option (c) incoherent — circular, because option (c) lost partly on that same premise); AND both sides of the record are cited, so a reader landing on either finds the other"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-first-contact-positioning.md — the persona-disposition section (read by closeout-and-durable-audience when it populates .kb/product/)"
  verifying_test: "content review that the atom contains the journey-atom-linked-to-Persona-1 consequence, the mechanism argument and the discounted circularity argument; a NEGATIVE check that the superseded wording (journey stage on the application author's atom) appears nowhere except as the thing amended; file:line resolution of _design.md:214-220, :222-243 and .bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:148-179"

- id: AC-005
  criterion: "GIVEN AP-3 forbids two maturity counts on one surface and any count not attached to a date and a version (`_design.md`:650-652), AND AP-13 forbids a positions-and-gaps statement implying the library owns `read_from_a_gap_position` (:670-672), AND the project's risk register names an open question resolved in passing by the release rather than by decision as medium/high (`project.md`:351), WHEN the atom is read, THEN it records the census sentence's form (dated, version-scoped, four counts, four inline definitions, one link) and contains none of the four numbers itself, stating instead that they are taken from `spec/SPECIFICATION.md`:219-222 by the AC-003 clause audit at the publish commit — a count in the atom is AP-3 relocated from the page into the corpus; AND it carries DT-4's nested absence-with-a-reason in substance (the promise must not imply ownership of `read_from_a_gap_position`), links `kb-open-question-es-38-and-gap-read-unowned-001` as `related`, and leaves that open-question atom open and untouched; AND it cites rather than copies `_design.md`'s composition (:347-437), density budget (:439-498), hierarchy (:500-616) and AP-1...AP-15 (:641-679) — no region table, no budget table and no anti-pattern list in the atom, because two copies of a budget is the two copies drift failure DT-4's own resolution is engineered against (`_design.md`:268-270)"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/00NN-first-contact-positioning.md — the DT-4 and DT-5 sections and the atom's related edge set; .kb/open-questions/es-38-and-gap-read-rules-are-unowned.md left standing"
  verifying_test: "content review that the atom contains no maturity count and does contain both the non-ownership qualification and the spec/SPECIFICATION.md:219-222 provenance sentence; git diff --quiet -- .kb/open-questions/ and redkiln validate --kb showing that atom byte-identical to HEAD; structural check that no table in the atom is copied from _design.md"

- id: AC-006
  criterion: "GIVEN atoms are authored by `/redkiln:kb-ingest` and never by hand (`CLAUDE.md`; reverted once at 0269720), AND an atom with no map row is one the corpus cannot navigate to, WHEN the wave lands, THEN the atom reached `.kb/decisions/` from a staged document under `.kb/_intake/` through the ingest path — no file was written directly under `.kb/decisions/` by this story — with `.kb/_intake/` cleared afterwards and `.kb/_intake/README.md` dropped at the approval gate rather than ingested (`.kb/_intake/README.md`:13-19); AND `.kb/maps/decision-map.md` carries one row for it in ADR-number order under the wave's own `##` section with no superseded row deleted (:81-86); AND `.kb/decisions/0001`...`0016` and `0029` are byte-identical, `redkiln validate --kb` and `redkiln doctor` are clean with exactly six template-drift advisories and zero dependency-cycle, and the diff is bounded by the PR boundary — no published surface, no `Cargo.toml`, no `RUNBOOK.md`, no `spec/SPECIFICATION.md`, no `_design.md`; AND each of `landing-copy-and-status-truth` (HS-S0092), `compliance-claim-and-gaps-promise` (HS-S0093) and `guarantees-and-docs-rs-presentation` (HS-S0094) can read its own instruction out of the atom alone — which claim leads and in what form, which slot the gaps promise sits in and what it may not imply, what the census sentence must contain, and where the peer statement lives in full versus reduced — without re-deciding anything"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md — the wave section's row for 00NN (the composition root that makes the atom navigable)"
  verifying_test: "redkiln validate --kb && redkiln doctor (exit 0; exactly six template-drift advisories, zero dependency-cycle); git ls-files .kb/_intake/ free of this story's document with .kb/_intake/README.md still present; decision-map row-count assertion (rows added only, none deleted); git diff --name-only against the merge base a subset of the spec's PR boundary; read-back answering all four downstream questions from the atom's text alone; cargo xtask affected --base main (.redkiln/config.yaml:40) green"
```
