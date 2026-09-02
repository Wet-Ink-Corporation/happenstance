---
item: "HS-S0131"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Three personas and four journeys staged in `.kb/_intake/`

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes specific to this story, all from `spec.md`.

**The wave.** This story declares the two strings its slice-mate must use, and they are recorded
here rather than only in prose so `product-atom-promotion-via-kb-ingest` consumes the same wave
instead of inventing one (`spec.md` *Context pack* 5, AC-008):

- **wave id** — `2026-08-12-product-audience` (distinct from `2026-08-10-intake` at `e768413` and
  `2026-08-10-intake-2` at `72f2c9b`)
- **wave glob** — `.kb/_intake/product-*.md` (narrow on purpose: a bare run would sweep
  `.kb/_intake/README.md` into the wave, `.kb/_intake/README.md:3-5`)

**The seven staged paths** this ledger's evidence cites, per `.redkiln/config.yaml:67`:
`.kb/_intake/product-persona-application-author.md`,
`.kb/_intake/product-persona-adapter-author.md`,
`.kb/_intake/product-persona-local-first-edge-developer.md`,
`.kb/_intake/product-journey-choose-a-contract-before-a-database.md`,
`.kb/_intake/product-journey-learn-when-you-are-finished.md`,
`.kb/_intake/product-journey-event-source-at-the-edge.md`,
`.kb/_intake/product-journey-decide-in-one-sitting.md`.

**Every `verifying_test` below is a recorded command with its result, or a reviewed artefact at a
real path — not a test-framework id.** This project owns no code (`project.md` *Out of scope*), and
the `testing` brief assigns static and process as the load-bearing tiers here (`_decomposition.md`
*Test mix, summarised by tier*). Do **not** flip AC-008 on the strength of the wave id being
*recorded*: the `.kb/product/` half is observable in this PR, the consumption is the slice-mate's,
in the same slice and the same context.

`story dir/` below abbreviates
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/`.

```yaml
- id: AC-001
  criterion: "The next initiative inherits a settled audience, not a re-opened question. GIVEN the planner who will frame acceptance criteria from `.kb/product/` and must not have to re-decide whether the evaluator is a person, WHEN the approver reviews `.kb/_intake/` before the ingest, THEN they find exactly seven top-level `product-*.md` drafts and nothing else new — three `concept`/persona drafts (application author, adapter author, local-first/edge developer) and four `playbook`/journey drafts whose titles are the charter's own (`initiative.md:243-250`) — with no persona draft for the evaluator and no draft in which the evaluation beats appear as a stage inside Persona 1's journey. Both of those alternatives were live, signed-off positions until 2026-08-12, and each is now wrong (Context pack 1, B-1)."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "`ls .kb/_intake/product-*.md` returns exactly the seven names of spec.md Context pack 2; `rg -n \"^kind:\" .kb/_intake/product-*.md` returns three `concept` and four `playbook`; `rg -n \"^authority_tier:\" .kb/_intake/product-*.md` returns seven `product`. Reviewed against .bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:150-180 and .bklg/from-contract-to-published-library/publication-and-positioning/_design.md:222-235"

- id: AC-002
  criterion: "The evaluator's moment is findable by the reader it exists to serve, and the reason it stands alone is on the page. GIVEN the planner who opens `.kb/product/` looking for \"the person deciding in one sitting\" and would otherwise have to read Persona 1's journey to the end to find them, WHEN they open `product-journey-decide-in-one-sitting.md`, THEN it declares the application-author persona's atom id in `related` (and Persona 1's draft names it back, so the link survives whichever atom is read first), AND one short paragraph states the settled reason — the difference is one of mechanism, one-shot public evidence versus revisable contact with the code over weeks, which earns a journey; every distinguishing property is a property of a moment, not of a person, which is why it is not a persona — citing both amended artefacts by path. The paragraph records the decision; it does not re-argue it and does not reopen DT-1."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "Review of .kb/_intake/product-journey-decide-in-one-sitting.md (`related` contains the application-author id; the reason paragraph cites _decomposition.md DR-10's amendment and publication-and-positioning/_design.md:222-235) plus `rg -n \"decide-in-one-sitting\" .kb/_intake/product-persona-application-author.md` non-empty for the reciprocal link; cross-read against ../_discovery/distillation/personas-and-journeys.md:333-338"

- id: AC-003
  criterion: "What the approver reads is what the promoted atom will carry. GIVEN the approver who can only approve what the draft says, and an ingest that re-authors rather than copies and is \"biased hard towards merging into an existing atom\" (`.kb/_intake/README.md:7-11`), WHEN they open any of the seven drafts, THEN it opens with a YAML block naming the proposed `id` (`kb-concept-…-001` / `kb-playbook-…-001`), `title`, `kind`, `status`, `authority_tier: product`, `summary`, `source_paths` and `related`, in the shape the corpus already uses — so no field the promoted atom is later checked for exists only as prose an extraction pass can paraphrase away. This cannot fail validation where it sits, by design (`.kb/_intake/README.md:21-27`), which is exactly why it must be checked by eye here."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "Field-by-field review of each of the seven drafts' `---`-fenced frontmatter block (all eight keys) against .kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30; plus `redkiln validate --kb` exiting zero with the drafts staged, recorded as the `_`-prefix SKIP it is (.kb/_intake/README.md:21-27) and never cited as proof the drafts are well-formed"

- id: AC-004
  criterion: "A reader who reads only the frontmatter cannot mistake an inference for an observation. GIVEN the planner about to frame an acceptance criterion from an atom, WHEN they read its `summary` and stop there, THEN the last sentence is in that atom's own voice, names that atom's own evidence classes (download counts for Persona 1; the project's six skeletons and the surveyed conformance regimes for Persona 2; one blog post and one issue thread for Persona 3; research framing rather than a named individual's account for the evaluation journey), and contains both literal strings `secondary evidence` and `directly observed`. All seven carry it, journeys included. A copy-pasted collective \"All four personas rest on secondary evidence…\" fails twice: there are three personas, and it reads as someone else's claim inside an atom's own summary (DR-9, `project.md:166-170`; B-4)."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "`rg -c \"secondary evidence\" .kb/_intake/product-*.md` and `rg -c \"directly observed\" .kb/_intake/product-*.md` each returning 7 files with >=1 hit; `rg -n \"All four personas\" .kb/_intake/product-*.md` returning zero; reviewer confirmation per file that both strings fall inside the `summary` scalar and that the sentence names that draft's own evidence classes"

- id: AC-005
  criterion: "Every claim can be walked back to the artefact it came from, and the artefact really says it. GIVEN the planner who distrusts an inherited persona and wants to check one sentence of it, WHEN they follow a draft's `source_paths`, THEN each path exists and contains the attributed content — the distillation file plus the specific upstream artefacts bearing on that persona or journey, narrowed from the distillation's own `sourcePaths` (`personas-and-journeys.md:8-21`) rather than copied wholesale, so a path list is a claim about provenance and not a decoration. An address that resolves says nothing about whether the claim is there (`.kb/playbooks/verify-the-referent-and-report-coverage.md`). The extra `.kb/_intake/…` entry the ingest adds on promotion (`README.md:13-19`) is expected and is not a defect."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "`test -f` over every `source_paths` entry in all seven drafts, followed by the referent check with coverage reported (n checked of n cited) per .kb/playbooks/verify-the-referent-and-report-coverage.md; plus the narrowing check — no two drafts carry an identical `source_paths` list and none reproduces all fourteen entries of ../_discovery/distillation/personas-and-journeys.md:8-21"

- id: AC-006
  criterion: "A promotion dated at closeout is not a pre-initiative guess wearing a closeout date. GIVEN the reader who knows `.kb/product/README.md:23-24`'s bar (\"a persona nobody researched is a stock photo with a name\") and reads the source's own instruction that it \"should be re-checked at closeout once real contact… exists to confirm or correct them\" (`personas-and-journeys.md:361-369`), WHEN they open any draft, THEN it carries one short *What closeout contact showed* note saying confirmed, corrected (with what changed), or untouched by this initiative, each naming the artefact from this initiative it rests on. An honest \"untouched\" passes; a silent omission does not; and no download figure, adoption number or user quotation appears that is not already in a cited artefact."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "`rg -n \"What closeout contact showed\" .kb/_intake/product-*.md` returning 7; each note's cited artefact opened and confirmed to bear on the sketch; a reviewer sweep for numerals and quoted speech with each traced to a cited artefact (anything untraceable is EC-4, manufactured evidence, and fails)"

- id: AC-007
  criterion: "The audience layer stays usable after the API and the registry page move. GIVEN the planner reading a journey two initiatives from now, WHEN they read any draft end to end, THEN it says what the persona is trying to accomplish and in what order, and contains no screen, flow or interaction pattern, no crate, trait, method or feature-flag name, no acceptance criterion or scope statement, and no market sizing — the four exclusions of `.kb/product/README.md:26-44`, in the Rust-shaped form they take here. The distillation's own `scope: problem-space-only` (`personas-and-journeys.md:7`) is inherited, not relaxed; the crates.io and docs.rs surfaces belong to `publication-and-positioning`'s `_design.md`."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "`rg -n \"EventStore|event_store_conformance|SendEventStore|happenstance-core|happenstance-testkit|feature =|#\\[cfg\" .kb/_intake/product-*.md` returning zero hits, plus a read-through of all seven for the two exclusions a grep cannot catch (an acceptance criterion stated in prose, a market-sizing claim), reviewed against .kb/product/README.md:26-44"

- id: AC-008
  criterion: "The ingest that follows has one wave to consume and nothing pre-empted. GIVEN the slice-mate about to run `/redkiln:kb-ingest`, and a bare run that would sweep `.kb/_intake/README.md` into the wave and clear the directory on success (`README.md:3-5`, `:13-19`), WHEN they read this story's `_ledger.md`, THEN it declares the wave id `2026-08-12-product-audience` — distinct from `2026-08-10-intake` (`e768413`) and `2026-08-10-intake-2` (`72f2c9b`), whose commit subjects and telemetry run files are keyed on theirs — and the narrowing glob `.kb/_intake/product-*.md`, AND `.kb/product/` is byte-identical to its pre-PR state (one file, `README.md`), so AC-008's provenance check downstream still has something to observe. Every draft is at the top level of `_intake/`: a subdirectory is not read by the wave glob and is therefore not mounted."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/"
  verifying_test: "`git diff --stat main -- .kb/product/` empty and `ls .kb/product/` returning README.md only; `ls .kb/_intake/` showing the seven drafts plus README.md and no subdirectory; the wave id and glob present verbatim in the preamble of this ledger; `git log --oneline` confirming 2026-08-10-intake (e768413) and 2026-08-10-intake-2 (72f2c9b) are the taken ids"

- id: AC-009
  criterion: "The initiative closes without entrenching a contradiction in a durable layer. GIVEN the reader of the closeout record who must be able to see that a cross-project disagreement was found, not absorbed (`project.md` DR-12, `:179-183`; risk row `:306`), WHEN they open `_closeout-record.md`'s *Findings* section, THEN one row records the DR-10/DT-1 discrepancy — the two artefacts, the stages and dates they were signed off at, the synthesis that settled it, and the residual question of whether HS-P0016's published positioning copy is consistent with a three-persona layer — pointing at the finding written under this story's folder, with the destination cell left owed to `findings-disposition-register`. The row records; it does not repair, does not reopen DT-1, and does not edit either signed-off artefact beyond the owner's amendment already in them."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Review of .bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/_finding-dr10-dt1.md (both artefacts with file:line, both sign-off dates and stages, the synthesis, the residual question) against exactly one appended row in the *Findings* section of _closeout-record.md whose destination cell reads as owed; plus `git diff main -- .bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md .bklg/from-contract-to-published-library/publication-and-positioning/_design.md` empty in this PR"
```
