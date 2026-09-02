---
item: HS-S0153
stage: spec
created: 2026-08-17T13:16:11.865Z
updated: 2026-08-17T13:16:11.865Z
template_sig: 87bbf1d0
rendered_sig: a2476dce
---

# Spec — The playbook atom staged under .kb/_intake/, never hand-authored into .kb/

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — AC-13, DoD-15's wave |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — the HS-P0021 → HS-P0025 handoff seam |
| Project | [`.bklg/docs-that-teach/page-need-discipline/project.md`](../project.md) — AC-012, DR-11 |
| This spec | `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — architecture Note 8 ("What must not move"), testing brief AC-012 (`:940-975`); [`../_grounding.md`](../_grounding.md) tension 2 |
| Signed-off design | [`../_design.md`](../_design.md) — approved 2026-08-17 with four carried conditions; **binding** on what this atom may claim about the discipline |
| Story map row | [`../_storymap.md`](../_storymap.md), milestone `binding-beyond-this-project` |
| Roadmap pointer | **none.** `rg -n "docs-that-teach" RUNBOOK.md` is empty — this initiative is not on the phase roadmap, and its charter is the plan of record |

## One-line PR slice

Stage a `kind: playbook`, `authority_tier: guideline` atom under `.kb/_intake/` carrying the method, the rejected alternatives and the conditions under which the discipline stops holding — with a field-by-field frontmatter check in the ledger, because `redkiln validate --kb` skips `_`-prefixed directories by design.

## Executive summary

This PR lands **one new file**: `.kb/_intake/lesson-page-need-declaration-discipline.md`, an atom-grade
markdown document staged for `/redkiln:kb-ingest` and nothing else. It changes no Rust, no gate step, no
rule atom and no page. It is the last story in the project because it is the only one whose content is
*about* the other seven: an atom that records rejected alternatives and stop conditions can only be true
once the thing it describes has stopped moving ([`../_storymap.md`](../_storymap.md), "Merge order" 3.2).

**The delta against the project charter** is that DR-11 states the obligation and this spec states the
shape: which path, which frontmatter values, what the body must carry to satisfy
[`.kb/playbooks/README.md:9-25`](../../../../.kb/playbooks/README.md), what it must *not* carry to stay on
the right side of `:29-31`, and — the part that is easy to get wrong and expensive to get wrong late —
how the criterion is verified when the directory it lands in is the one directory `redkiln validate --kb`
is deliberately blind to.

**The delta against the design** is smaller still: [`../_design.md`](../_design.md) resolved DT-2, DT-3 and
DT-8 with their rejected options and their failure modes already written down. This story does not
re-decide any of them. It compresses that reasoning into a portable form and cites the tree, and where the
atom would disagree with the design or with the shipped `standards/pages/` corpus, the atom is wrong.

## Context pack

The load-bearing decisions, stated inline. Read this section and you can start; everything deeper is a
signposted anchor in the second half of this spec.

**1. Staging is the deliverable; ingest is somebody else's.** Atoms reach `.kb/` only through
`/redkiln:kb-ingest`, which runs at initiative closeout — four projects downstream of here
(`../project.md`, "How this advances the initiative"). Hand-authoring the directory layout of that process
without the process was tried in this repository once and reverted at `0269720` (`CLAUDE.md`, "Where the
work lives"). So the deliverable is a file in the ingest path, and **`git diff main -- .kb ':!.kb/_intake'`
must be empty**. That one-line check is AC-012's mechanical half and it is a negative: it proves nothing
landed in the checked tree, and it proves nothing at all about the file that did land.

**2. The frontmatter has no mechanical check, and the Definition of Done must not be read as if it did.**
`redkiln validate --kb` skips any directory whose name begins with `_`, deliberately, so that a draft you
are still staging does not have to satisfy the atom schema to sit there
([`.kb/_intake/README.md:21-27`](../../../../.kb/_intake/README.md)). The same README says outright that
nothing staged there is an atom yet and that staged files are **not held to `KbFrontmatter`** (`:7-11`).
The project's Definition of Done carries a `redkiln validate --kb` green line; that line discharges
decision (1) and *not* this one. The bar here is a **manual, field-by-field walk** of the staged file's
frontmatter against `.kb/README.md`'s required-field table (`:16-26`: `id`, `title`, `kind`, `status`,
`authority_tier`, `summary`, `depends_on`/`related`, `source_paths`, `last_reviewed`), recorded in
`_ledger.md` as its own evidence and never citing the green CLI run in its place
([`../_grounding.md`](../_grounding.md) tension 2; [`../_decomposition.md`](../_decomposition.md) testing
brief, AC-012).

**3. Author it to atom grade anyway, precisely because nothing forces you to.** The intake contract permits
raw material — a design note, a pasted transcript — and the ingest run then extracts claims and adjudicates
them, biased hard toward *merging into an existing atom* over spawning a near-duplicate
(`.kb/_intake/README.md:7-11`). Staging raw material would make HS-P0025 the **author** of this
discipline's knowledge; staging an atom-grade proposal makes it the **vehicle**, which is exactly the seam
the closeout project's own charter draws ("this project supplies the ingest vehicle, not the text" —
`.bklg/docs-that-teach/durable-audience-closeout/project.md:83-86, 109-112`). That seam is the reason this
story exists as a story rather than as a bullet on someone else's.

**4. `status: proposed`.** Not `accepted` — acceptance is conferred by the ingest wave that adjudicates the
atom against the corpus, and asserting it in the one directory nothing validates would be claiming a state
no gate granted. Not `draft` — draft invites the wave to treat the file as raw material and re-derive its
claims at full cost, which is the thing decision (3) is buying its way out of. `proposed` is the honest
reading of a finished proposal awaiting a decision that is someone else's to take.

**5. A playbook carries method, never commitment.** If it says *must*, *shall* or *must not* — if reversing
it would take a decision — it belongs in `.kb/decisions/` and filing it as a playbook strips it of the
immutability that makes it enforceable (`.kb/playbooks/README.md:29-31`). The binding rules of this
discipline stay where the gate reads them: the `RP-NN-N` rules in `standards/pages/` (created earlier in
this project; see [`../_design.md`](../_design.md), "Surfaces"). The atom **cites those rule ids and does
not restate them** — the same cite-never-restate discipline this project imposes on pages with respect to
`spec/SPECIFICATION.md` (DR-09), turned back on itself.

**6. Three obligations, and the third is the one that separates a playbook from a summary.**
`.kb/playbooks/README.md:11-20` asks for a method with its supporting evidence *and the conditions under
which it stops holding*, and for the cheaper alternatives that were discarded with the reason each lost —
because "a playbook that lists only the approach that won reads as arbitrary, and the next reader
re-derives the rejected options at full cost before trusting it". Every one of those rejected options is
already written down and already reviewed in [`../_design.md`](../_design.md): the S1 pattern's five
rejected forms, DT-2's three rejected taxonomies, DT-3's implicit-byproduct and navigation-widget options,
DT-8's reviewer-judgement and permanent-ban options, and the four rejected homes for the tree itself.
**Compress and cite; do not re-litigate and do not invent a rejection that was not taken.**

**7. Ground every claim, or drop it.** `.kb/playbooks/README.md:22-25` requires the file, the command, the
compiler error or the number, with those paths in `source_paths` — "a claim about how to work is settled
against something that compiled or something that was measured, not against how the work felt". This
repository has the numbers: `MAX_ATOM_BYTES` at `xtask/src/lint_constitution.rs:95`, the report-all-then-`bail!`
shape at `:169-198`, the vacuity `bail!` at `:176`, the generated-region equality check at `:334-421`, and
`check_summaries`' statement of why a hand-maintained index is a second copy of the truth at `:466-470`.

**8. Portability is a checkable property, not a tone.** The test is stated: does the claim survive being
read by someone working on a **different part of the system** (`.kb/playbooks/README.md:33-35`)? So the
atom's subject is the *shape* — make the answered need a declared, singular, enumerated property of a prose
artifact; check the declaration mechanically; name in the checker's own documentation the judgement it
cannot make; and supply a written non-author procedure for that judgement (RS-81-1,
`standards/rust/81-checks-that-cannot-be-types.md:11`). Happenstance's page-need discipline is the
*instance*, cited throughout. An atom that reads as this project's diary fails `:33-35` and is a
`reference` atom or nothing at all.

**9. Read the shipped tree, not only the design.** This story's three `depends_on` edges —
`fold-line-rule`, `reviewer-and-citation-procedures`, `declaration-check-seen-to-fail` — are precisely the
stories that could still have changed what the atom describes. Write the atom against
`standards/pages/` and the checker module **as they stand at this story's HEAD**. Where the shipped tree
and [`../_design.md`](../_design.md) disagree, the shipped tree is what the atom describes and the
divergence is reported in `_ledger.md` rather than smoothed over — that is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` (`kb-governance-referent-not-reasoning-001`)
applied to this story's own output.

**10. Do not link the staged path from anything that outlives it.** A successful ingest **clears**
`.kb/_intake/` — that is the contract, and it is what makes the directory readable
(`.kb/_intake/README.md:13-19`). So a link to `.kb/_intake/lesson-….md` from `standards/pages/README.md`,
from `docs/README.md`, or from any pinned tree is a link that dies at closeout, in a repository whose whole
initiative is about links that resolve. The handoff travels as provenance — the atom's `source_paths`, this
story's `_ledger.md`, and the seam already recorded in the closeout project's charter — not as a
cross-tree hyperlink.

**11. The wave will have exactly two files, and one of them is a README.** The default input is
`.kb/_intake/*.md` (`.kb/_intake/README.md:3-5`) and `.kb/_intake/README.md` is a real `.md` file that is
deliberately not an atom (`:29-30`). Today the directory holds only that README; after this story it holds
two files. HS-P0025 makes surviving the README an acceptance criterion of its own
(`.bklg/docs-that-teach/durable-audience-closeout/project.md:198`, AC-009) and drops it at the ingest
approval gate. This story's obligation is to name its own staged file explicitly in `_ledger.md` so that
gate is a one-line decision rather than an archaeology exercise.

**12. Nothing else under `.kb/` moves.** `.kb/maps/domain-map.md` is not edited by this project at all — a
new domain area is appended during ingest's own Maps phase (`.kb/maps/README.md:34-36`), and the charter's
stronger "an appended section, never an edit" is an inference this project carries rather than a quoted
rule ([`../_decomposition.md`](../_decomposition.md), architecture Note 8;
[`../_grounding.md`](../_grounding.md) tension 4).

**13. Give the wave somewhere to merge.** Ingest prefers merging into an existing atom over creating a
near-duplicate, so the staged atom names real, existing atom ids in `depends_on`/`related` rather than
leaving them empty: `kb-playbook-verify-referent-report-coverage-001` (a check verifies its referent and
reports its coverage — the direct ancestor of "the checker sees a declaration, never an answer"),
`kb-playbook-ratchet-gate-landing-001` (landing a stricter gate over a corpus, which is what DT-8 Part 3's
deliberately empty `PERMITTED_FOLD_MECHANISMS` list is a variant of),
`kb-playbook-anchoring-citations-001` (citations over a tree that moves — the mechanism behind
cite-never-restate), and `kb-governance-referent-not-reasoning-001`. Every id above was read out of the
tree; none is invented.

**14. The persona-journey slice.** The reader served is *the next page author* and, one step out, *the next
initiative* — initiative AC-13, "someone writing the next page can cite a written discipline for what makes
a page teach, rather than reconstructing this initiative's reasoning from its output"
(`.bklg/docs-that-teach/initiative.md:398-400`). The discipline's binding half serves the author who is
writing *inside* this repository; the playbook serves the one who is not.

## Integration contract

| | |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice: the artifact the closeout wave picks up |
| **Slice / milestone** | `binding-beyond-this-project` |
| **Slice-mates** | `governed-page-cites-the-discipline` — implemented in the same context and mounted together |
| **Mount point** | **`.kb/_intake/lesson-page-need-declaration-discipline.md`**, landing in `.kb/_intake/` — the directory that *is* `/redkiln:kb-ingest`'s default input glob (`.kb/_intake/README.md:3-5`) and the vehicle HS-P0025 operates (`.bklg/docs-that-teach/durable-audience-closeout/project.md:83-86`). There is no code composition root here and inventing one would be the anti-pattern: `redkiln validate --kb` is blind to this tree on purpose, so presence in the glob's directory is the mount, and the ledger's field-by-field walk is what makes the mount observable |
| **Wires into** | `.kb/README.md:16-26` (the required-field table the frontmatter is checked against) · `.kb/playbooks/README.md:3-5,11-25,29-38` (what a playbook is, must carry, and must not carry) · `.kb/_intake/README.md:3-5,7-11,13-19,21-27` (glob, no-schema-here, clearing contract, validator skip) · the four existing atom ids in Context pack (13) · `standards/pages/**` and the project's new `xtask/src/` checker module, both created earlier in this project, cited by the atom and edited by it in no way |
| **Renders surfaces** | **none.** This story renders no surface from [`../_design.md`](../_design.md)'s manifest (`page-need-declaration`, `discipline-router`, `rule-atom`, `lint-terminal-output`, `reviewer-procedure`). It *describes* the discipline those five surfaces embody, so the design binds it negatively: any characterisation in the atom that contradicts the signed-off design, or the shipped tree, is a defect in the atom |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** This story touches no port, no store, no envelope type and no crate under `crates/`. `happenstance-testkit`'s suite cannot observe a markdown file staged for a knowledge-base ingest, and adding a rule that could would be the decorative rule `CLAUDE.md` forbids |
| **Clause(s)** | **none.** No `spec/SPECIFICATION.md` clause is discharged, amended or restated. The atom may *cite* clause ids as evidence for the cite-never-restate method; citing is not amending, and clause ids are stable and never renumbered (`spec/SPECIFICATION.md:280`) |
| **Advances DoD scenario** | It discharges **no** initiative Definition-of-Done scenario on its own, and claiming one would be the decoration this initiative exists to refuse. It satisfies initiative **AC-13** directly, and it is the payload the single closeout ingest wave carries when HS-P0025 runs **DoD-15**'s scenario (`.bklg/docs-that-teach/initiative.md:465-470`; `durable-audience-closeout/project.md:83-86`). DoD-14 stays with the slice-mate `governed-page-cites-the-discipline` and the router story, where the discipline's *decided home* and its citation live |

**Delivered mounted, not as an isolated component.** The observable form of "mounted" here is that the file
sits inside the ingest glob under a name the wave's operator can identify at the approval gate, that its
frontmatter would survive `KbFrontmatter` if it were validated, and that `_ledger.md` records the check that
proves it — because no CI job will.

## PR boundary

**In this PR**

- `.kb/_intake/lesson-page-need-declaration-discipline.md` — new, and the whole of the product.
- `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/**` — this story's
  `_ledger.md` and its implementation report, carrying the field-by-field frontmatter walk, the recorded
  divergences (if any) between the atom and the shipped tree, and the staged file's path named for
  HS-P0025's approval gate.

Per the Integration contract, mounting this slice requires touching **no** composition-root or wiring file:
the mount is the ingest glob's directory. Any edit outside the two globs above is scope drift, not mounting.

**Explicitly not in this PR**

- **Anything under `.kb/` outside `_intake/`.** `git diff main -- .kb ':!.kb/_intake'` is empty. No
  `.kb/playbooks/` file, no `.kb/product/` atom, no `.kb/maps/domain-map.md` edit
  ([`../_decomposition.md`](../_decomposition.md), architecture Note 8).
- **Running `/redkiln:kb-ingest`.** The wave, the adjudication, the clearing of `_intake/` and the
  `domain-map.md` append are HS-P0025's (`durable-audience-closeout/project.md:83-89`).
- **`standards/pages/**`.** The rules tree, its router, its rule atoms, the fold-line rule and the reviewer
  procedures were landed by this story's three dependencies. This story reads them and edits none of them —
  and if reading them reveals a defect, the defect is reported, not patched here.
- **`xtask/**`.** No new gate step, no change to the checker, no change to `main.rs`, `affected.rs` or
  `lint_constitution.rs` ([`../_decomposition.md`](../_decomposition.md), architecture Note 8).
- **HS-P0020's pages tree and its citation of the discipline.** That is the slice-mate
  `governed-page-cites-the-discipline`, delivered in the same context under its own spec.
- **Any link from a durable tree into `.kb/_intake/`** — Context pack (10).
- **Any ADR.** No Accepted decision atom constrains this project and none is written by it
  ([`../_grounding.md`](../_grounding.md) tension 1).

**Merge DoD.** `cargo xtask ci --fast` and `cargo xtask affected --base main` green, `redkiln validate --kb`
and `redkiln doctor` green with exactly six `template-drift` advisories, `git diff main -- .kb ':!.kb/_intake'`
empty, and `_ledger.md` carrying the manual frontmatter walk that the green `validate --kb` run explicitly
does not stand in for.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The staged file exists at a pinned path** | Exactly one new file, `.kb/_intake/lesson-page-need-declaration-discipline.md`. The `lesson-` prefix follows the two staged sources this repository has already ingested, so the wave's operator reads the name as payload rather than as scaffolding | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` (`source_paths`), `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` (`source_paths`), `.kb/_intake/README.md:3-5` |
| **It is swept by the default glob** | `/redkiln:kb-ingest` with no argument ingests all of `.kb/_intake/*.md`; the file needs no registration anywhere and must not be registered anywhere | `.kb/_intake/README.md:3-5` |
| **`kind: playbook`, `authority_tier: guideline`** | Both fixed by DR-11 and by the layer README. `authority_tier` is `z.string().min(1)` in the schema, so nothing enforces the vocabulary — the value is a convention this repository already keeps | `../project.md` DR-11; `.kb/playbooks/README.md:3-5`; `.kb/README.md:31-35` |
| **`status: proposed`** | Not `accepted` (acceptance is the ingest wave's to confer), not `draft` (which invites re-derivation from raw material) | Context pack (4); `.kb/README.md:21`; `.kb/_intake/README.md:7-11` |
| **Every required frontmatter field present and well-formed** | `id`, `title`, `kind`, `status`, `authority_tier`, `summary`, `depends_on`, `related`, `source_paths`, `last_reviewed`. `id` follows the corpus's `kb-playbook-<slug>-001` shape and is proposed, not promised — ingest may merge the atom into an existing one under a different id | `.kb/README.md:16-26`; `.kb/_templates/atom.md:1-18`; `.kb/playbooks/*.md` frontmatter |
| **`summary` is a paragraph, not a label** | The corpus's playbook summaries state the problem, the two properties that decide the design, the options ruled out and the discriminator — they are what a reader adjudicates on before opening the body | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:7-15`; `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:7-15` |
| **`source_paths` resolve, and are the evidence** | Every entry is a path that exists in the tree at merge: the shipped `standards/pages/` corpus, the project's `xtask/src/` checker module, `xtask/src/lint_constitution.rs`, `.bklg/docs-that-teach/page-need-discipline/_design.md`, `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `.kb/playbooks/README.md:22-25` |
| **`depends_on`/`related` name real atom ids** | The four ids in Context pack (13), each read out of `.kb/` rather than invented, so ingest's merge bias has somewhere to land | `.kb/playbooks/verify-the-referent-and-report-coverage.md:2`; `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:2`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:2`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md:2` |
| **The body carries a method with its evidence and its stop conditions** | The method: declare the answered need as a singular, enumerated, human-visible and machine-read property; check the declaration mechanically; state the check's blind spot in its own documentation; supply a non-author procedure for the judgement the check cannot make. The stop conditions are named, not implied — a medium that does not render CommonMark blockquotes in document order, a corpus small enough that the router costs more than it saves, a subject whose reference surface is *not* already owned elsewhere (which is what let `reference` be dropped from the need set) | `.kb/playbooks/README.md:11-12`; `../_design.md` "S1 — `page-need-declaration`" (hosting assumption), "S1 vocabulary — DT-2" |
| **The body names the discarded alternatives and why each lost** | Compressed from the design, not re-decided: the four rejected homes for the tree; front matter, HTML comment, filename convention, sidecar manifest and badge for the declaration form; literal four-box adoption, an open set, and a persona-keyed taxonomy for the need set; implicit-byproduct and a bespoke nav widget for findability; reviewer judgement and a permanent ban for the fold line | `../_design.md` (each rejection with its stated cost); `.kb/playbooks/README.md:15-20` |
| **The body prescribes; it does not explain and it does not commit** | No `must` / `shall` / `must not` carrying a commitment — those live as `RP-NN-N` rules in the gate-read tree and are *cited*. An atom that only described how the discipline works would be a `concept`, not a playbook | `.kb/playbooks/README.md:29-31,37-38` |
| **The body is portable** | Claims are about the shape; happenstance is the instance. At least one recurrence outside this initiative is named, so the atom passes the "different part of the system" test rather than reading as a project diary | `.kb/playbooks/README.md:33-35` |
| **The atom describes the tree that shipped** | Written against `standards/pages/` and the checker module at this story's HEAD, not against the design alone. Divergences from [`../_design.md`](../_design.md) are recorded in `_ledger.md`, never silently reconciled | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; [`../_storymap.md`](../_storymap.md) "Merge order" 3.2 |
| **Nothing lands in `.kb/` proper** | `git diff main -- .kb ':!.kb/_intake'` is empty. `.kb/maps/domain-map.md` untouched | [`../_decomposition.md`](../_decomposition.md) testing brief AC-012, architecture Note 8; `.kb/decisions/README.md:9-13` |
| **The staged path is referenced by nothing durable** | `rg -n "_intake/lesson-page-need" -g '!.bklg/**'` finds no hit outside `.kb/_intake/` itself, because a successful ingest deletes the file | `.kb/_intake/README.md:13-19` |
| **The frontmatter check is manual and recorded** | A field-by-field walk against `.kb/README.md:16-26`, written into `_ledger.md` as its own evidence. `redkiln validate --kb` green is recorded separately and labelled as proving only the negative | `.kb/_intake/README.md:21-27`; [`../_grounding.md`](../_grounding.md) tension 2; [`../_decomposition.md`](../_decomposition.md) testing brief AC-012 |
| **The wave's approval gate is set up, not left to chance** | `_ledger.md` names this story's staged file explicitly, so HS-P0025 can drop `.kb/_intake/README.md` at the approval gate rather than after the fact | `.bklg/docs-that-teach/durable-audience-closeout/project.md:198,246` |

## Data and migrations

**N/A — no schema, no store, no migration.** This story adds one markdown file and touches no crate, no
table, no serialised type and no wire format. The only structured data in the diff is the staged file's
YAML frontmatter, and it is validated by a human against `.kb/README.md:16-26` rather than by a migration
or a schema check.

Two state transitions are worth naming so nobody plans for them here:

- **The promotion is HS-P0025's, and it is a delete.** A successful ingest writes real atoms into
  `.kb/playbooks/` (or merges into an existing one) *and removes the staged source*, committing both
  together on the wave's own branch (`.kb/_intake/README.md:13-19`). Nothing in this PR anticipates that
  by writing the destination, and nothing in this PR depends on the source surviving.
- **A playbook is not immutable, and that is why the commitment may not live in it.** The
  never-edit-an-accepted-atom rule is a `decision`-atom rule (`.kb/decisions/README.md:9-13`), so a
  playbook can be corrected in place after ingest. That is precisely why the binding half of this
  discipline stays in `standards/pages/` where the gate reads it: filing a commitment as a playbook strips
  it of the immutability that makes it enforceable (`.kb/playbooks/README.md:29-31`).

## Acceptance criteria

Each criterion is a reader completing something. Three readers recur, and they are the ones this
story actually serves: **the next page author** (initiative AC-13,
`.bklg/docs-that-teach/initiative.md:398-400`), who is writing a page and wants a written discipline
rather than this initiative's output to reverse-engineer; **the practitioner on a different part of
the system**, who is the portability test made a person (`.kb/playbooks/README.md:33-35`); and **the
ingest operator**, HS-P0025's, standing at the approval gate deciding merge-or-spawn for every file
the glob swept (`.bklg/docs-that-teach/durable-audience-closeout/project.md:83-86`). The three
initiative personas — application author, adapter author, evaluator
(`.bklg/docs-that-teach/initiative.md:244-268`) — are served **transitively**: they read the pages
this discipline shapes, and none of them ever opens this file. Saying otherwise would be the
decoration this initiative exists to refuse.

`<atom>` below stands for `.kb/_intake/lesson-page-need-declaration-discipline.md` throughout.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the ingest operator runs `/redkiln:kb-ingest` with no argument at initiative closeout, **WHEN** the default glob `.kb/_intake/*.md` is expanded (`.kb/_intake/README.md:3-5`), **THEN** exactly one new payload file — `<atom>` — is swept in beside the directory's README, under a `lesson-` name the operator reads as payload rather than scaffolding, and it is registered in no manifest, index or config anywhere else | `git diff --name-status main -- .kb/_intake` shows exactly one `A` line and it is `<atom>`; `ls .kb/_intake` lists exactly two files; both transcripts pasted into `_ledger.md` §Mount, which also names `<atom>` in full so HS-P0025's operator drops `README.md` at the gate and nothing else |
| **AC-002** | **GIVEN** the ingest operator adjudicating this file against the corpus, **WHEN** they read its first six frontmatter lines and nothing else, **THEN** they see `kind: playbook`, `authority_tier: guideline` and `status: proposed` — a finished proposal awaiting a decision that is theirs to take, neither claiming an acceptance no gate conferred nor inviting a full re-derivation from raw material | `rg -n '^(kind\|status\|authority_tier):' <atom>` transcript verbatim in `_ledger.md` §Frontmatter walk rows 3–5, each row carrying the one-sentence reason the two rejected values (`accepted`, `draft`) lost — Context pack (4) |
| **AC-003** | **GIVEN** that `redkiln validate --kb` skips every `_`-prefixed directory by design and is therefore **silent** about this file (`.kb/_intake/README.md:21-27`), **WHEN** the implementer closes the story, **THEN** `_ledger.md` carries a field-by-field walk of all ten required fields against `.kb/README.md:16-26` — `id`, `title`, `kind`, `status`, `authority_tier`, `summary`, `depends_on`, `related`, `source_paths`, `last_reviewed` — each row citing the staged file's own line number, and **no** row of that walk cites the green `validate --kb` run as its evidence | `_ledger.md` §Frontmatter walk has ten rows, each with a `<atom>:NN` cite; a reviewer greps the AC-003 evidence string for `validate --kb` and finds no hit. The green CLI run is recorded separately, in §Mechanical negatives, labelled as proving only AC-010's negative — `_grounding.md` tension 2; `_decomposition.md` testing brief AC-012 |
| **AC-004** | **GIVEN** the ingest operator at the approval gate, who must decide merge-or-spawn **before** opening the body, **WHEN** they read `summary` alone, **THEN** it adjudicates: it states the problem, the properties that decide the design, the options ruled out and the discriminator, composed as the corpus's playbook summaries are — a paragraph in the folded-block form, not a one-line label and not a restatement of the title | Side-by-side against `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:7-15` and `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:7-15`, recorded in `_ledger.md` §Summary adjudication: the four elements quoted out of the drafted `summary`, and its sentence count (the two corpus exemplars run to five and four) |
| **AC-005** | **GIVEN** a reader who does not yet trust the atom, **WHEN** they follow any entry in `source_paths` or any id in `depends_on` / `related`, **THEN** every path resolves in the tree at merge and every id resolves to an atom that exists — so the wave's merge bias has somewhere real to land rather than a near-duplicate to spawn | `_ledger.md` §Resolution transcript: one `test -f` line per `source_paths` entry, all present; one `rg -n "^id: <id>" .kb` line per id, each a single hit. The four ids of Context pack (13) are pre-resolved at `.kb/playbooks/verify-the-referent-and-report-coverage.md:2`, `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:2`, `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:2`, `.kb/governance/rewrite-the-referent-never-the-reasoning.md:2` |
| **AC-006** | **GIVEN** the practitioner on a different part of the system, who has this problem and twenty minutes, **WHEN** they open the atom in a GitHub blob view and read top to bottom **clicking nothing**, **THEN** the four-step method, the evidence that grounds each step, **and** a named section stating the conditions under which the discipline stops holding are all visible in document order — none of the three behind a `<details>`, a fold, a tab, or a link to another file | `rg -n '<details>\|<summary>\|<!--' <atom>` returns no hit, transcript in `_ledger.md` §Non-occlusion. §Method records the heading that carries the stop conditions (the corpus shape is `## Where this is the wrong instrument`, `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:98`) and quotes at least two named conditions verbatim — `.kb/playbooks/README.md:11-12` |
| **AC-007** | **GIVEN** the next page author asking why the discipline is shaped the way it is, **WHEN** they reach the discarded-alternatives section, **THEN** each of the five rejection families is present with the reason it lost — the four rejected homes for the tree, the five rejected declaration forms, DT-2's three rejected need taxonomies, DT-3's two rejected findability options, DT-8's two rejected fold policies — each **compressed and cited** to `_design.md` rather than re-litigated, and **no rejection appears that the design did not take** | `_ledger.md` §Rejection cross-walk: five rows, each naming the family, the atom's line and a `_design.md` line cite; a non-author confirms every rejection in the atom maps to a row and that no sixth family was invented — `.kb/playbooks/README.md:15-20` |
| **AC-008** | **GIVEN** a reader looking for what actually binds them, **WHEN** they search the atom for a commitment, **THEN** they find none: no `must` / `shall` / `must not` whose reversal would take a decision, and every binding claim appears instead as a cited `RP-NN-N` rule id whose text is **not** restated — so the enforceable half stays where the gate reads it and a commitment is not stripped of the immutability that makes it enforceable | `rg -n '\b(must\|shall\|MUST\|MUST NOT)\b' <atom>` transcript in `_ledger.md` §Commitment audit, adjudicated line by line (each surviving hit annotated with why it is method, not commitment); every `RP-` id in the atom checked against the shipped `standards/pages/` rule text for restatement — `.kb/playbooks/README.md:29-31,37-38`; `../project.md` DR-11 |
| **AC-009** | **GIVEN** the practitioner outside this initiative applying the atom to their own corpus, **WHEN** they read it, **THEN** the subject is the *shape* — declare the answered need as a singular enumerated property; check the declaration mechanically; name the check's blind spot in its own documentation; supply a written non-author procedure for the judgement it cannot make — with happenstance as the cited instance and **at least one recurrence outside this initiative named**; and the atom describes `standards/pages/` and the checker module **as they stand at this story's HEAD**, any divergence from `_design.md` being recorded rather than smoothed over | A non-author performs the portability test of `.kb/playbooks/README.md:33-35` and records the verdict, the named recurrence and their identity in `_ledger.md` §Portability. §Divergence lists every place the atom's description differs from `_design.md`, or states "none observed" explicitly — never blank — per `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **AC-010** | **GIVEN** an initiative whose whole thesis is links that resolve, and an ingest that **deletes** this file on success (`.kb/_intake/README.md:13-19`), **WHEN** closeout clears `_intake/`, **THEN** nothing that outlives the file points at it and nothing was written into the checked knowledge base ahead of the wave: `git diff main -- .kb ':!.kb/_intake'` is empty, `.kb/maps/domain-map.md` is untouched, and the handoff travels as provenance in `source_paths` and `_ledger.md` rather than as a cross-tree hyperlink | `_ledger.md` §Mechanical negatives: `git diff main -- .kb ':!.kb/_intake'` empty; `git diff --stat main -- .kb/maps` empty; `rg -n "_intake/lesson-page-need" -g '!.bklg/**'` returns only `<atom>` itself. `redkiln validate --kb && redkiln doctor` green with **exactly six** `template-drift` advisories, recorded here and labelled as proving this AC and not AC-003 |
| **AC-011** | **GIVEN** the same atom opened in three places the corpus is actually read — a GitHub blob view, a plain-text pager, and an editor at 100 columns — **WHEN** it renders in each, **THEN** it is *composed*, not merely marked up: the corpus's playbook shape (an H1, `##` section headings, run-in bold markers, monospace `path:line` citations), meaning that survives greyscale (no colour, icon, emoji or badge carries any distinction), **no** `> **Answers:**` declaration line — this is a `.kb` atom, not a governed page, and a declaration here would claim a governance the checker's pinned trees do not extend to it — and a body inside the measured corpus envelope: **≤ 8,192 bytes** and **≤ 100 source columns** outside tables | `_ledger.md` §Composition: the atom's heading inventory; `wc -c <atom>` against the measured corpus (six existing playbook atoms run 5,161–7,137 bytes, largest `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`); `awk '{if(length($0)>m)m=length($0)}END{print m}' <atom>` against the corpus maximum of 100 (measured across the three exemplar atoms); `rg -n '^> \*\*Answers:\*\*' <atom>` no hit; greyscale legibility confirmed by inspection — `../_design.md` "Density budget", "Hierarchy", anti-patterns 2 and 3 |

**Coverage of the traced project AC.** Project **AC-012** ("the playbook atom is staged, not
hand-authored"; `../project.md:242-244`) is this story's sole property (`../_storymap.md:119`), and
it splits exactly as the testing brief splits it: its **mechanical negative** half is AC-010, its
**frontmatter** half is AC-002 + AC-003 + AC-005, and its unstated-but-real *"is this actually a
playbook"* half is AC-004 + AC-006 + AC-007 + AC-008 + AC-009 + AC-011, which is what DR-11 asks
for and what no command checks. No other project AC is traced by this story and none is partially
touched.

## Interaction quality

This story renders **no surface** from `../_design.md`'s manifest — the Integration contract says so
and it stays true. But the artifact it lands *is a document a human reads*, and the design's
composition vocabulary is written for a text medium on purpose ("the three transience classes map
onto a text medium exactly", `../_design.md` "Transience policy"). So the invariants below are the
design applied to this artifact, not borrowed from a screen it does not have. Every one of them is
carried by an **AC row in the table above** — none is a bullet here, because a bullet in this
section gets no ledger row, is never gated, and is never tested.

**State invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — the method, its evidence and its stop conditions are readable in one document without following a link out | AC-006 | `rg` for fold markup returns nothing; the stop-condition heading and two conditions quoted in `_ledger.md` §Method |
| **Non-occlusion** — nothing load-bearing is hidden until something is clicked; the summary adjudicates before the body is opened | AC-004, AC-006, AC-007 | §Non-occlusion transcript; §Summary adjudication quotes the four elements out of `summary` alone |
| **Preserved focus / scroll / selection** | **N/A, and stated rather than left blank.** A staged markdown file has no interactive state to preserve: no view, no selection, no scroll position the artifact itself owns. The nearest real analogue is the reader's *place in the corpus*, which AC-006 protects by keeping the method in one document | — |
| **Reversibility** | AC-010 | The PR is exactly one added path (AC-001), nothing durable references it, and ingest's own success condition is deleting it — so `git revert` restores the tree exactly and closeout's delete breaks nothing |
| **Keyboard reachability** | AC-006, AC-011 | The atom authors no affordance that needs a pointer — no `<details>`, no tab strip, no accordion (design anti-pattern 6, `PERMITTED_FOLD_MECHANISMS` empty). In a plain-text pager every word is reachable, which is the floor the design's own "minimum legible size" argument leans on |

**Composition invariants**, taken from the signed-off `../_design.md`.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the file is composed in the corpus's playbook shape, not a wall of bare markup or an unstructured brain-dump (which is what `_intake` *permits*, and what decision (3) refuses) | AC-011, AC-004 | Heading inventory in `_ledger.md` §Composition, compared against `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:32-98`'s section run |
| **Composition and placement** — `summary` before the body; method before its rejected alternatives; stop conditions last, as the corpus places them | AC-011, AC-006 | §Composition records the section order |
| **Transience** — every load-bearing region is **persistent chrome**; nothing in this artifact is "revealed on hover" or "opened on demand". The one exception the design would permit, a long aside behind a link, is not used | AC-006, AC-007 | §Non-occlusion |
| **Density budget, with its real numbers** — **≤ 8,192 bytes** (the design's own router ceiling, half `MAX_ATOM_BYTES` at `xtask/src/lint_constitution.rs:95`) against a measured corpus of 5,161–7,137; **≤ 100 source columns** outside tables against a measured corpus maximum of 100 | AC-011 | `wc -c` and the `awk` column measure, both transcripts in §Composition |
| **Hierarchy** — carried by position, heading level, block type, the `**Bold.**` run-in marker and monospace. **Colour is never a carrier** | AC-011 | Greyscale legibility check; §Composition records that no icon, emoji or badge appears |
| **Anti-pattern 2** — a distinction shown as a colour, icon or emoji with no word | AC-011 | Greyscale check |
| **Anti-patterns 5 and 6** — a load-bearing claim invisible until something is clicked; any `<details>`, tab strip or accordion at all | AC-006, AC-007 | `rg` fold-markup transcript |
| **Anti-pattern 15** — a procedure step containing "consider", "use judgement", "as appropriate", "if it seems". The atom *describes* the reviewer procedure; describing it with soft verbs reintroduces exactly the judgement DT-8 removed | AC-008, AC-009 | §Commitment audit greps the four phrases; each hit is either removed or annotated as quoting a *rejected* option |
| **Anti-pattern 16** — a fifth need token, or a `reference` declaration. The atom must not quietly re-open the closed set by naming a token the shipped `NEEDS` does not carry | AC-009 | §Divergence: the atom's token list is diffed against the shipped `NEEDS`, and any difference is a divergence, not a liberty |

**One invariant this artifact must deliberately *not* satisfy.** A governed page's first block after
its H1 is a `> **Answers:**` declaration (design anti-pattern 1). This file is a `.kb` atom, outside
both pinned trees, and carrying a declaration would assert a governance no checker extends to it —
so its absence is asserted, in AC-011, rather than left to chance. That inversion is the reason this
section exists at all instead of a blanket "renders no surface".

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The atom is written into `.kb/playbooks/` (or any layer under `.kb/`) instead of `.kb/_intake/` — the failure the project risk register scores Medium/High (`../project.md:314`) and the one this repository already made and reverted at `0269720` | AC-010's `git diff main -- .kb ':!.kb/_intake'` is non-empty and the story does not close. The repair is to move the file, not to add an exception |
| **EC-002** | A `depends_on` / `related` id does not resolve — it was misremembered, or the atom it named was itself superseded | The story halts on AC-005 and the id is re-read out of `.kb/` rather than guessed. An unresolvable id defeats the merge bias the atom is shaped to exploit (Context pack 13) and reads to the wave as a spawn instruction |
| **EC-003** | A `source_paths` entry points at a path that a dependency story moved after the design was signed off | Re-anchor to the shipped path and record the move in `_ledger.md` §Divergence. Do **not** delete the claim to make the citation go away — an ungrounded claim fails `.kb/playbooks/README.md:22-25` more expensively than a stale path does |
| **EC-004** | The shipped `standards/pages/` tree or the checker module disagrees with `../_design.md` on something the atom describes | The **shipped tree** is what the atom describes, and the divergence is written into `_ledger.md` §Divergence rather than reconciled in prose. This is `.kb/governance/rewrite-the-referent-never-the-reasoning.md` turned on this story's own output — Context pack (9) |
| **EC-005** | The ledger cites a green `redkiln validate --kb` run as evidence for AC-002/AC-003/AC-005 | Rejected at review. The validator is blind to `_`-prefixed directories by design (`.kb/_intake/README.md:21-27`); the green run is AC-010's evidence and is labelled as such in a different ledger section |
| **EC-006** | A durable tree — `standards/pages/README.md`, `docs/README.md`, a rule atom's `Evidence.` list — links `.kb/_intake/lesson-…` | AC-010's `rg` sweep finds it and the link is removed. A successful ingest deletes the target, so the link is a dangling link on a delay, in an initiative whose subject is links that resolve |
| **EC-007** | The atom carries a binding `must` / `shall` / `must not` | It moves out of the atom and into a cited `RP-NN-N` rule, or it was never a commitment and the sentence is rewritten as method. Filing a commitment as a playbook strips it of immutability (`.kb/playbooks/README.md:29-31`) |
| **EC-008** | The atom names a rejected alternative the design never took, or attributes a reason `../_design.md` does not state | The row fails AC-007's cross-walk. Invented rejections are worse than absent ones: they teach a false history that the next reader cannot check against anything |
| **EC-009** | `redkiln doctor` reports other than exactly six `template-drift` advisories | The gate fails and the cause is found before merge. A seventh is a template changed without deciding to; a missing one is a customisation reverted by `adopt --templates`, which this repository never runs (`CLAUDE.md`, "Where the work lives") |
| **EC-010** | `.kb/_intake/` holds a third file at merge — a stray note, a second draft | AC-001's `ls` shows it. The wave's approval gate is a one-line decision only while the directory is `README.md` + this payload (Context pack 11; `../../durable-audience-closeout/project.md:198`) |

## Non-functional

| id | requirement | why, and how it is met |
| --- | --- | --- |
| **NF-001** | **Citation durability.** Every citation in the atom survives the tree moving under it | Line-anchored cites rot. Where the target is a heading or a named item, cite `path` plus the heading or the symbol; reserve `path:line` for the cases where the line *is* the evidence (a constant, a `bail!`, a message string). This is `kb-playbook-anchoring-citations-001` applied to a file that will be read after this initiative closes |
| **NF-002** | **Zero gate cost.** The PR adds no CI step, no build input, no dependency and no measurable time to `cargo xtask ci` | The file is outside every pinned tree the gate reads and outside every crate. `cargo xtask affected --base main` should treat the change as inert; if it does not, something in the diff is outside the PR boundary |
| **NF-003** | **Blast radius of one file.** The change is revertible by `git revert` with no follow-up, and its eventual deletion by the ingest wave breaks nothing | Guaranteed by AC-001 (one added path) and AC-010 (nothing durable references it). It is what lets this story merge last in the slice without becoming a dependency of anything |
| **NF-004** | **Load cost.** An agent or a person opening this atom pays for all of it, so it is sized to be worth loading whole | The ≤ 8,192-byte ceiling of AC-011, and its stated reason, are the same argument `lint_constitution.rs:503-508` already makes for the constitution's atoms — two trees teaching two different numbers for one idea is its own defect (`../_design.md`, "Density budget") |
| **NF-005** | **The wave's cost.** The atom is authored so the ingest run *adjudicates* rather than *derives* | Raw material makes HS-P0025 the author of this discipline's knowledge; an atom-grade proposal makes it the vehicle (Context pack 3). AC-002's `status: proposed` and AC-004's adjudicable `summary` are what make the difference observable at the approval gate |

## Implementation notes (non-prescriptive)

**Read before writing, and read the tree rather than the design.** The three `depends_on` stories are
exactly the ones that could have changed what the atom describes. Open the shipped `standards/pages/`
router and rule atoms, the checker module, and the two failing runs `declaration-check-seen-to-fail`
recorded, *then* open `../_design.md` to recover the reasoning. Doing it in the other order is how a
divergence gets smoothed over instead of recorded (EC-004).

**A shape that already works is in the tree.** `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`
runs `## The situation` → `## The three options` (each with **A —** / **B —** / **C —** run-in markers
and the reason each lost) → the properties that make it work → two "why it had to be this way"
sections → `## Where this is the wrong instrument`. That last section is the stop-conditions
obligation in the corpus's own words, and reusing the shape costs nothing and buys AC-006 and AC-007
most of the way.

**Compressing five rejection families into one section is the hard part.** They are not equally
interesting: the four rejected homes for the tree and DT-3's two findability options are one
sentence each; DT-2's need taxonomy and DT-8's fold policy carry the transferable reasoning and are
worth a short paragraph. The test is AC-007's cross-walk, not word count.

**On the `id`.** `kb-playbook-<slug>-001` matches the corpus and is a *proposal* — the wave may merge
this atom into an existing one under a different id, which is the outcome Context pack (13) is
actively courting. Nothing downstream depends on the id surviving, and the `_ledger.md` row should
say so, so nobody later treats a merged-away id as a broken promise.

**On `last_reviewed`.** The date the atom was written against the shipped tree — which is this
story's merge date, not the design's sign-off date. It is the field a future reader uses to decide
whether the tree has moved since, and back-dating it to the design would make it lie by a slice.

**What "one recurrence outside this initiative" can honestly be.** The pattern — a mechanical check
that verifies a *declaration* and cannot verify the *answer*, paired with a written non-author
procedure for the part it cannot see — already recurs in this repository at RS-81-1
(`standards/rust/81-checks-that-cannot-be-types.md:11`) and in `kb-playbook-verify-referent-report-coverage-001`.
Naming one of those is honest; inventing an industry-wide claim is not, and would fail the same
evidence bar the atom is asserting (`.kb/playbooks/README.md:22-25`).

**Write the ledger as you go, not at the end.** Six of the eleven ACs are discharged by transcripts
of commands that are cheapest to capture at the moment they are run, and two (AC-007's cross-walk,
AC-009's portability walk) need a **non-author**, which is a scheduling constraint, not a formatting
one.

## Tests and CI (merge gate)

No tier here is a `#[test]`, and that is the testing brief's own categorisation of AC-012 rather than
a shortcut: this story adds no Rust, so `cargo test -p xtask` has nothing new to run and adding a
Rust test that reads a markdown file in `.kb/_intake/` would be the decorative check `CLAUDE.md`
forbids. Every row below is either a deterministic command whose transcript is the evidence, or a
procedural walk whose *identity of walker* and verdict are the evidence
(`../_decomposition.md`, testing brief AC-012 and its "Merge-gate commands" block).

| tier | command / path | proves |
| --- | --- | --- |
| **Gate state (mechanical negative)** | `git diff main -- .kb ':!.kb/_intake'` — empty | AC-010. Nothing landed in the checked knowledge base. This is the one-line shape the testing brief pairs with AC-002's |
| **Gate state** | `git diff --name-status main -- .kb/_intake` — exactly one `A`, and `ls .kb/_intake` — exactly two files | AC-001, EC-010. The mount, and that the approval gate stays a one-line decision |
| **Gate state** | `git diff --stat main -- .kb/maps` — empty | AC-010. `domain-map.md` is appended during ingest's own Maps phase, not here (`.kb/maps/README.md:34-36`) |
| **Static (repository sweep)** | `rg -n "_intake/lesson-page-need" -g '!.bklg/**'` — hits only `<atom>` | AC-010, EC-006. No durable tree links a path that closeout deletes |
| **Static (file inspection)** | `rg -n '^(kind\|status\|authority_tier\|id\|title\|summary\|depends_on\|related\|source_paths\|last_reviewed):' <atom>` | AC-002, AC-003. Every required field of `.kb/README.md:16-26` present, at a cited line |
| **Static (file inspection)** | `rg -n '<details>\|<summary>\|<!--' <atom>` — no hit; `rg -n '^> \*\*Answers:\*\*' <atom>` — no hit | AC-006, AC-011. Non-occlusion, and the deliberate absence of a governed-page declaration |
| **Static (file inspection)** | `rg -n '\b(must\|shall\|MUST)\b' <atom>`, every hit adjudicated in the ledger | AC-008, EC-007. No commitment filed as a playbook |
| **Static (measurement)** | `wc -c <atom>` ≤ 8192; `awk '{if(length($0)>m)m=length($0)}END{print m}' <atom>` ≤ 100 | AC-011, NF-004. The density budget, against a corpus measured at 5,161–7,137 bytes and 100 columns |
| **Static (resolution)** | `test -f` per `source_paths` entry; `rg -n "^id: <id>" .kb` per `depends_on`/`related` id | AC-005, EC-002, EC-003. Every citation and every merge target resolves |
| **Procedural (non-author)** | The rejection cross-walk: five families, each mapped to a `../_design.md` line, walked by someone who did not draft the atom | AC-007, EC-008. Compression is faithful and no rejection was invented |
| **Procedural (non-author)** | The portability walk of `.kb/playbooks/README.md:33-35`, verdict and walker identity recorded | AC-009. The atom is not this project's diary |
| **Procedural** | The summary adjudication against the two corpus exemplars; the composition/heading inventory; the greyscale legibility check | AC-004, AC-011. The composed presentation the unstyled-render check cannot see |
| **Backlog / KB gate** | `redkiln validate --kb && redkiln doctor` — green, **exactly six** `template-drift` advisories | AC-010, EC-009 **only**. Explicitly *not* AC-003: the validator skips this directory by design |
| **Story-grain gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml` `verify.affected_gate`) | NF-002. The diff touches nothing the gate compiles; a non-inert result means the PR boundary leaked |
| **Project-grain gate** | `cargo xtask ci --fast` (`verify.integration_scoped`; `../project.md` DoD) | The bar this non-terminal project is held to. It cannot see the atom, and saying so plainly is the point of the row |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | containment |
| --- | --- | --- |
| **The green `validate --kb` line is read as discharging the frontmatter half.** The project's Definition of Done carries that line, and it is one sentence away from AC-012's frontmatter obligation | High / High | AC-003 forbids the citation and EC-005 rejects it at review; the ledger keeps the two in *different sections* with different labels. This is `_grounding.md` tension 2 made mechanical |
| **The atom is written from `../_design.md` alone**, because the design is complete, reviewed and much easier to read than three merged stories' output | Medium / High | AC-009 requires the description to match the shipped tree and AC-007 requires a `_design.md` line cite per rejection family — the two together make the implementer open both. EC-004 says which wins |
| **Compression becomes re-litigation.** Five rejection families is a lot of reasoning, and re-deriving is easier than compressing | Medium / Medium | The design already carries every rejection with its stated cost; AC-007's cross-walk is a mapping exercise, not a judgement one. A sixth family in the atom is a defect by construction |
| **It reads as a project diary** and fails `.kb/playbooks/README.md:33-35` on arrival at the wave | Medium / Medium | AC-009's non-author portability walk is the only instrument that catches this, and it is the reason that walk is non-author. Context pack (8) states the subject/instance split before a line is written |
| **A dependency's tree moves between this story's HEAD and merge**, stranding a citation | Low / Medium | NF-001 prefers heading-anchored cites; EC-003 requires re-anchoring plus a §Divergence entry rather than deletion. The story merges last in the slice precisely to shrink this window (`../_storymap.md:151-153`) |
| **Scope drift into HS-P0025's work** — running the ingest, writing the destination atom, appending to `domain-map.md` | Low / High | The PR boundary forbids all three and AC-010's two `git diff` checks catch the last two mechanically. The first is caught by `.kb/_intake/` being non-empty at merge, which is the *desired* state here |
| **Coupling: none outbound.** No story, project or gate step consumes this file | — | This is why it merges last and blocks nothing. The single consumer is HS-P0025's wave, four projects downstream, and it consumes a *directory*, not a path (`../../durable-audience-closeout/project.md:83-86`) |

## Dependencies

**Blocks on** — all three landed and merged before this story starts, because the atom describes what
they shipped:

- **`fold-line-rule`** — DT-8's resolution, including the deliberately empty `PERMITTED_FOLD_MECHANISMS`
  list. It is one of the five rejection families AC-007 must carry, and the atom cannot state the fold
  policy's rejected options until the policy has stopped moving.
- **`reviewer-and-citation-procedures`** — the non-author verdict walk (DR-07) and the cite-never-restate
  rule (DR-09). AC-009's method step four *is* that procedure, and AC-008's "cite, never restate" is that
  rule applied to the atom itself.
- **`declaration-check-seen-to-fail`** — the two recorded failing runs. They are the grounded evidence
  AC-006 requires: `.kb/playbooks/README.md:22-25` asks for the command and the message, and this is the
  story that produced them.

**Unlocks** — **nothing inside this project or this initiative.** It is the terminal story of the
terminal slice (`../_storymap.md:151-153`). Its consumer is **HS-P0025** (`durable-audience-closeout`),
four projects downstream, whose ingest wave picks the file up out of the glob; that is a handoff across
an initiative-level seam, not a story dependency, and HS-P0025 does not name this story — it names the
directory.

**Slice-mate**, implemented in the same context and mounted together, but with no dependency edge in
either direction: **`governed-page-cites-the-discipline`**. That story makes a page cite the discipline
(AC-011 of the project, DoD-14); this one makes the discipline portable (AC-012, initiative AC-13). They
share a milestone because both are about the discipline binding *beyond* the tree it lives in.

## Anchors (progressive disclosure)

Everything above is sufficient to start. Open these when the moment named in the third column arrives —
each is load-bearing for exactly the criterion in the fourth, and none should be read in bulk up front.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| [`.kb/playbooks/README.md`](../../../../.kb/playbooks/README.md) | The layer contract in full: the three things a playbook must carry (`:11-20`), the evidence bar (`:22-25`), what does not belong — commitment, one-off, explanation (`:29-38`). Every content AC is this file made checkable | Before drafting the body; again before the commitment audit | AC-006, AC-007, AC-008, AC-009 |
| [`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`](../../../../.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md) | The closest structural exemplar in the corpus: a folded-block `summary` that adjudicates (`:7-15`), option run-ins with the reason each lost (`:46-56`), and `## Where this is the wrong instrument` (`:98`) — the stop-conditions obligation in the corpus's own words | While drafting `summary` and the section skeleton | AC-004, AC-006, AC-007, AC-011 |
| [`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`](../../../../.kb/playbooks/anchoring-citations-in-a-long-lived-document.md) | Both a merge target (`kb-playbook-anchoring-citations-001`, `:2`) and the method NF-001 asks the atom to follow in its own citations — the atom practises what it names | When choosing citation form; when filling `related` | AC-005, NF-001 |
| [`.kb/README.md`](../../../../.kb/README.md) | The required-field table (`:16-26`) that AC-003's walk is performed *against*, plus the `authority_tier` note (`:31-35`) explaining why the value is a convention nothing enforces | At the frontmatter walk, with the file open beside it | AC-002, AC-003 |
| [`.kb/_intake/README.md`](../../../../.kb/_intake/README.md) | Four separate contracts in one short file: the default glob (`:3-5`), "not held to `KbFrontmatter`" (`:7-11`), a successful ingest **clears** the directory (`:13-19`), and the deliberate `_`-prefix validator skip (`:21-27`). Misreading any one produces a different wrong story | Before the first write, and again before writing the ledger's evidence labels | AC-001, AC-003, AC-006, AC-010 |
| [`.kb/_templates/atom.md`](../../../../.kb/_templates/atom.md) | The frontmatter skeleton to copy, so field order and block style match the corpus rather than being re-invented | At the moment the file is created | AC-002, AC-003, AC-011 |
| [`.bklg/docs-that-teach/page-need-discipline/_design.md`](../_design.md) | The signed-off source of every rejection AC-007 must carry, each with its stated cost — and the composition, transience, density and hierarchy vocabulary AC-011 applies to this artifact. Binding: where the atom disagrees with it *and* with the tree, the atom is wrong | Section by section while writing the rejected-alternatives section; again for the composition check | AC-007, AC-011 |
| [`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`](../_decomposition.md) | The testing brief's AC-012 paragraph states the two-half split this spec's ACs implement, and says in as many words that the ledger must not cite the green `validate --kb` run. Architecture Note 8 is the "what must not move" list AC-010 enforces | Before writing `_ledger.md`; before touching anything under `.kb/` | AC-003, AC-010 |
| [`.bklg/docs-that-teach/page-need-discipline/_grounding.md`](../_grounding.md) | Tension 2 is the validator-blindness trap in its original form; tension 4 records that "appended, never edited" for `domain-map.md` is an inference this project carries, not a quoted rule | When you are tempted to treat a green gate as evidence | AC-003, AC-010 |
| [`.bklg/docs-that-teach/durable-audience-closeout/project.md`](../../durable-audience-closeout/project.md) | The downstream consumer's charter: the ingest-vehicle seam (`:83-86`, `:109-112`) and its own AC-009 about surviving the intake README at the approval gate (`:198`). It defines what "handed off cleanly" means from the receiving end | Once, before writing `_ledger.md` §Mount | AC-001, AC-010 |
| [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md) | The governance atom (`kb-governance-referent-not-reasoning-001`, `:2`) behind EC-004: when the shipped tree and the reasoning disagree, you rewrite the referent and record the divergence — you do not quietly restate the reasoning to fit | The moment the tree and the design disagree | AC-009, EC-004 |
| [`.kb/playbooks/verify-the-referent-and-report-coverage.md`](../../../../.kb/playbooks/verify-the-referent-and-report-coverage.md) | `kb-playbook-verify-referent-report-coverage-001` (`:2`) — the direct ancestor of "the checker sees a declaration, never an answer", and the strongest candidate for the recurrence AC-009 requires the atom to name | When naming the outside-this-initiative recurrence; when filling `depends_on` | AC-005, AC-009 |
| [`standards/rust/81-checks-that-cannot-be-types.md`](../../../../standards/rust/81-checks-that-cannot-be-types.md) | RS-81-1 (`:11`) is the rule that a check must state in its own documentation the judgement it cannot make — method step three, and the second honest candidate for AC-009's recurrence | Alongside the anchor above | AC-009 |
| [`xtask/src/lint_constitution.rs`](../../../../xtask/src/lint_constitution.rs) | The numbers the atom is grounded in rather than the code it describes: `MAX_ATOM_BYTES` (`:95`), report-all-then-`bail!` (`:169-198`), the vacuity `bail!` (`:176`), the generated-region equality check (`:334-421`), and `check_summaries`' "a hand-maintained index is a second copy of the truth" (`:466-470`) | When grounding a claim that needs a number | AC-006, AC-011 |
| [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) | AC-13 (`:398-400`) is the reader this atom serves, and the personas section (`:244-268`, `:275-302`) is why they are served transitively rather than directly | Once, if the framing of "who reads this" is in doubt | AC-009 |

## Clarifications resolved during spec

1. **An eleventh criterion was added: AC-011.** The first pass enumerated ten, all of them about
   *content* — path, frontmatter, method, rejections, grounding, negatives. None of them fails an atom
   that is correct and unreadable: a single 20,000-byte paragraph with valid frontmatter satisfies
   AC-001 through AC-010 completely. The design's composition, density and hierarchy vocabulary is
   written for a text medium (`../_design.md`, "Transience policy"), so it applies here, and the
   interaction-quality obligation is that every applicable invariant is an **AC row**, not a prose
   bullet. AC-011 is that row. The ledger carries eleven.
2. **Which personas the criteria are framed against.** The initiative's three personas — application
   author, adapter author, evaluator — never open this file. Framing the ACs against them would be
   traceability theatre. The criteria are framed against the three readers who do open it: the next
   page author (initiative AC-13), the practitioner on a different part of the system (the portability
   test made a person), and HS-P0025's ingest operator. The three initiative personas are served
   transitively, and the acceptance table says so rather than implying otherwise.
3. **The density ceiling is 8,192 bytes, and it is derived, not invented.** `MAX_ATOM_BYTES` is 16,384
   (`xtask/src/lint_constitution.rs:95`) but that governs `standards/rust/` atoms, not `.kb/` ones. The
   `.kb` corpus was measured instead: six playbook atoms run 5,161–7,137 bytes, and the largest is
   `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`. 8,192 — the design's own router
   ceiling, half `MAX_ATOM_BYTES` — sits above the whole measured corpus with room, and is the number
   AC-011 checks. The 100-column figure is likewise measured, not asserted: the three exemplar atoms
   max at 99, 99 and 100 source columns.
4. **The atom must *not* carry a `> **Answers:**` declaration.** This came up because the discipline
   this story documents is about declarations, and "practise what you document" is a tempting instinct.
   It is wrong here: the declaration is a claim of governance, the checker's pinned trees do not
   include `.kb/`, and a declaration nothing checks is exactly the decoration this initiative refuses.
   The absence is asserted in AC-011 rather than left implicit.
5. **`verifying_test` values are commands and ledger sections, not `#[test]` ids.** No tier in this
   story is a Rust test, and the testing brief classifies AC-012 as static/gate-state plus procedural
   for exactly this reason. Writing a `#[test]` in `xtask` that reads a file in `.kb/_intake/` would be
   a rule no adapter can fail, and would additionally couple the Rust gate to a directory that is
   deleted at closeout.
6. **No AC was dropped.** All ten ids from the first pass survive with their intent intact; AC-011 is
   additive. The ledger's ids are AC-001 through AC-011 and match this table exactly.
7. **Two ACs require a non-author** (AC-007's rejection cross-walk, AC-009's portability walk). That is
   a scheduling constraint on the slice, not a formatting one, and it is stated in the implementation
   notes so it is discovered before the last day rather than on it.
