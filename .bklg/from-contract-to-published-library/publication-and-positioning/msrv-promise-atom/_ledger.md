---
item: HS-S0086
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The MSRV stops being a preference and becomes a promise

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Note on the instruments below: this story's gate is the **KB gate**, not the compiler. The testing
brief types project AC-006 and AC-013 as *static (process, not code)* with `redkiln validate --kb`
as the instrument (`.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md`:469).
The `verifying_test` field therefore names a real command or a real path to read, and the content
criteria (AC-003 … AC-007) name the content review that is their honest tier — inventing a compiled
test for prose would be decorative, which is the failure mode `CLAUDE.md` names directly.

```yaml
- id: AC-001
  criterion: "The promise exists as an atom the corpus minted, not one a hand wrote. GIVEN `.kb/decisions/` may only hold atoms produced by an ingest wave (DR-13, `project.md`:208-212; `CLAUDE.md`, and the reverted hand-authoring attempt `0269720`), WHEN this story lands, THEN exactly one new `decision` atom exists under `.kb/decisions/` with valid `KbFrontmatter` — `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 12`, a stated `reversibility`, `supersedes: null`, `superseded_by: null`, `depends_on: [kb-decision-0004, kb-decision-0029]` — its `source_paths` cites the staged `.kb/_intake/…` document it was authored from, and that staged document and the atom appear in the same wave commit; and the atom is real composed prose in the corpus's shape, not a schema-valid stub"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; git log --stat on the wave commit shows the staged .kb/_intake/ source and the new .kb/decisions/00NN-*.md together; .kb/_governance/integration-waves/ gains exactly one new dated directory; frontmatter read field-by-field against .kb/decisions/0029-msrv-raised-to-1-97-1.md:1-35"

- id: AC-002
  criterion: "The number and the filename are chosen once, because they become a permanent public URL. GIVEN `0017–0028` are allocated to sibling projects (`../_decomposition.md`:116-121), `0029` is on disk (`_grounding.md`:33-36), and three slice-mates stage atoms into the same wave (`_storymap.md`:73-78), WHEN the wave runs, THEN this atom's `adr_id` is `>= ADR-0030`, is none of `0017–0028` or `0029`, and is distinct from every other atom in the wave; and its filename slug is the exact target of the absolute GitHub blob URL `guarantees-and-docs-rs-presentation` will publish, in the form the existing ADR-0029 bullet already uses at `crates/happenstance/README.md`:46-49, chosen before publish because `cargo yank` leaves every rendered page exactly as it was"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "ls .kb/decisions/ before and after the wave; the four staged .kb/_intake/ filename prefixes compared in one place before the run against .bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md:73-78"

- id: AC-003
  criterion: "The evaluator learns the floor as a promise, inside their budget, before deciding to depend. GIVEN Persona 4 has followed the one link the Guarantees bullet is allowed (`_design.md`:429, `:470`), WHEN they land on the atom, THEN its opening states that the `0.2.0` release requires Rust 1.97.1, that this is now a promise to a consumer rather than an internal build setting, and plainly that a consumer on an older toolchain cannot build the release — expressed so it is quotable as AC-UX-010's first Guarantees bullet at <= 3 rendered lines with exactly 1 link, positioned as a cost read after interest exists rather than as identity copy; and `Cargo.toml`:8 and `rust-toolchain.toml`:2 both still read `1.97.1`"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "content review of .kb/decisions/00NN-*.md against .bklg/from-contract-to-published-library/publication-and-positioning/_design.md:429, :470, :591-595; git diff shows Cargo.toml and rust-toolchain.toml unchanged"

- id: AC-004
  criterion: "The evaluator's question is answered here and not one hop further on. GIVEN their hop budget is already spent, WHEN they read the atom with ADR-0004 and ADR-0029 closed, THEN the atom's own body still answers \"which compiler, and why that one\" — naming that the floor was forced by a dependency's build script rather than by this workspace's code, that five of five database crates in the workspace declare no `rust-version` at all so neither `cargo hack --rust-version` nor `resolver = \"3\"` could see it coming, and that `rust-version` and `rust-toolchain.toml` remain two different facts; ADR-0004 and ADR-0029 are cited for provenance and are not required reading"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "content review: .kb/decisions/00NN-*.md read standalone with .kb/decisions/0004-edition-and-msrv.md and .kb/decisions/0029-msrv-raised-to-1-97-1.md closed; factual agreement cross-checked against 0029:41-53"

- id: AC-005
  criterion: "The evaluator learns what a bump would cost them, as one statement and not two facts to compose. GIVEN U6 asks when the cost can change and DR-6 asks explicitly whether the policy survives now a real consumer is bound (`project.md`:176-183), WHEN they read the atom's policy section, THEN it states that an MSRV bump is a minor version bump called out in the changelog and that under 0.x the minor bump is itself the breaking-change signal, joined in one statement rather than left to the reader — and it says whether that policy stands unchanged now that it binds someone; the statement is quotable at <= 3 rendered lines with exactly 1 link for AC-UX-010's remaining bullets"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "content review of .kb/decisions/00NN-*.md against .bklg/from-contract-to-published-library/publication-and-positioning/project.md:176-183, .kb/decisions/0004-edition-and-msrv.md:83-86, and _design.md:566-580"

- id: AC-006
  criterion: "What is actually checked is stated, so the promise is never larger than its evidence. GIVEN `crates/happenstance/README.md`:46 today says \"MSRV 1.97.1, checked in CI\" while the `msrv` job at `.github/workflows/ci.yml`:241-260 runs on a toolchain pinned to the floor itself, and this tree has already shipped a stale MSRV sentence (`CHANGELOG.md`:1031-1033), WHEN the atom describes verification, THEN it names the job, says in words that the job is currently vacuous because the pin equals the floor, names the condition (pin != floor) under which it starts proving something, and does not restate the unbacked \"checked in CI\" phrasing; the status is carried by words, never by a glyph or a badge alone"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "content review of .kb/decisions/00NN-*.md against .github/workflows/ci.yml:241-260 and .kb/decisions/0029-msrv-raised-to-1-97-1.md:63-69"

- id: AC-007
  criterion: "A reader a year from now can tell this was a fork, not an accident. GIVEN `.kb/decisions/README.md`:31-33 requires the rejected options, WHEN the atom is read, THEN it names at minimum four live alternatives and why each lost — keep the floor a preference / promise only \"latest stable\"; lower the published crates to 1.85 by pinning `rusqlite` 0.37; a per-package `rust-version` so the three published crates promise less than the workspace; and a moving-window policy such as N-2 stable in place of a fixed floor — with the two ADR-0029 already rejected (`:76-92`) re-weighed against a consumer rather than inherited"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "content review of .kb/decisions/00NN-*.md against .kb/decisions/README.md:31-33 and .kb/decisions/0029-msrv-raised-to-1-97-1.md:76-92 — a resolution stating a winner without stating what lost fails"

- id: AC-008
  criterion: "The two atoms that already carry the number come through byte-identical. GIVEN both are `status: accepted` and `redkiln validate --kb` checks accepted bodies against `HEAD` (`.kb/decisions/README.md`:7-13; `.kb/maps/decision-map.md`:30-34), WHEN the wave lands, THEN a diff against the branch point shows zero changed bytes in `.kb/decisions/0004-edition-and-msrv.md` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — body and frontmatter, including any `superseded_by` flip, because this is an amendment lineage and not a supersession (`.kb/maps/decision-map.md`:76-79) — and every existing inbound `file:line` citation into either atom still lands where it did"
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "git diff --exit-code <branch point> -- .kb/decisions/0004-edition-and-msrv.md .kb/decisions/0029-msrv-raised-to-1-97-1.md; redkiln validate --kb; redkiln doctor (exactly six template-drift advisories, no dependency-cycle)"

- id: AC-009
  criterion: "The record is reachable by someone navigating the corpus, and nothing still calls this future work. GIVEN an atom with no index row is the KB equivalent of a component constructed but never rendered, WHEN the wave lands, THEN the atom has a row on `.kb/maps/decision-map.md` in ADR-number order under a new `##` wave section with its 0004/0029 dependency named in the relationship column and no existing row deleted or displaced (`:81-86`); `.kb/_intake/` is cleared to its README alone, with that README not ingested; and `CLAUDE.md`'s fifth binding constraint names the new atom in the past tense instead of promising the conversion \"at phase 12\""
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "read .kb/maps/decision-map.md for the new row and the unchanged prior rows; ls .kb/_intake/ shows README.md only; git diff -- CLAUDE.md; redkiln doctor"
```
