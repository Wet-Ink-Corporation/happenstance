---
item: HS-S0179
stage: implement
created: 2026-08-17T13:16:29.746Z
updated: 2026-08-17T13:16:29.746Z
---

# Acceptance ledger — Mount every promoted atom at all four mount points

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This project compiles no Rust, so every `verifying_test` names a real command, a real grain in
`.redkiln/config.yaml`, or a real checklist path in the tree — not a Rust test function
(`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md`, `## Testing brief`, the
four-tier mix and AC-TB-02/AC-TB-08).

**One routed gap is carried by this story and must be recorded, not fixed**: the preamble at
`.kb/maps/domain-map.md:35` — *"This is the map's first wave. One domain exists so far."* — becomes
arithmetically false when the second domain is appended. Correcting it produces a deletion line and
breaks AC-002. Record it here in prose; do not open the file to fix it.

```yaml
- id: AC-001
  criterion: "GIVEN U1 has never heard of this initiative and opens `.kb/maps/domain-map.md` looking for who this library is for, WHEN they scroll to the end of the file, THEN they find exactly one appended `##` section covering documentation and the audience it teaches, placed after the last pre-existing section, listing **every** atom the wave landed — persona atoms, journey atoms, HS-P0021's carried page-need-discipline playbook and any `open_question` atom — each with a link whose text names its destination, its atom id beside the link, and one sentence of orientation, so U1 can choose which atom to open without opening any of them."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md — the appended `##` section (mount point 1 of 4)"
  verifying_test: "Tier 2 content review against `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` `## UX brief` AC-UX-07, read beside `.kb/maps/domain-map.md:105-142`; roll-call in `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md`"

- id: AC-002
  criterion: "GIVEN U3 later cites a fact by `file:line` against the corpus and GIVEN this spec itself cites `.kb/maps/domain-map.md:35`, `:105-142` and `:144-150`, WHEN this story's diff is applied, THEN `git diff` over `.kb/maps/domain-map.md`, `.kb/maps/open-questions-index.md` and `.kb/README.md` contains **no deletion line anywhere** — the stale preamble at `.kb/maps/domain-map.md:35` is left standing and recorded as a routed gap rather than corrected, `last_reviewed` is not bumped, and if the wave's Maps phase rewrote either, that hunk is reverted before the wave commits — so no anchor anyone already cited moves."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md, .kb/maps/open-questions-index.md, .kb/README.md — the diff surface of mount point 1"
  verifying_test: "Tier 1: `git diff .kb/maps/domain-map.md` · `git diff .kb/maps/open-questions-index.md` · `git diff .kb/README.md` — zero lines beginning `-` (IQ-3 falsifier; AC-UX-06; AC-A05)"

- id: AC-003
  criterion: "GIVEN U1 has landed on a journey atom and wants the persona whose goals that journey serves, WHEN they read the journey atom's frontmatter, THEN it names that persona in `related` / `depends_on` — and GIVEN U1 started at the persona instead, WHEN they read *its* frontmatter, THEN it names its journeys back, so neither direction of the pair is a dead end; every id on both sides resolves to a real atom, and edges toward HS-P0021's carried playbook are written on **this project's** atoms so the payload's own frontmatter stays byte-identical."
  satisfied: false
  evidence: ""
  mount_point: "Reciprocal `related` / `depends_on` frontmatter on the landed `.kb/product/` atoms (mount point 2 of 4)"
  verifying_test: "Tier 1: `redkiln validate --kb` (no-dangling-links rule, `.kb/README.md` `## Rules`); Tier 2 reciprocity read recorded in `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md`; Tier 1 payload check `git diff .kb/playbooks/`"

- id: AC-004
  criterion: "GIVEN U2 opens `.bklg/docs-that-teach/durable-audience-closeout/project.md` at the pull request and asks what this project actually harvested, WHEN they read HS-P0025's `links.kb`, THEN it lists every atom the wave landed — written by exactly one `redkiln record-links HS-P0025 --atom <comma-separated ids>` invocation run **after** `redkiln validate --kb` came back green, never by hand and never one id per invocation, and never on this story's own item."
  satisfied: false
  evidence: ""
  mount_point: "`links.kb` on HS-P0025, `.bklg/docs-that-teach/durable-audience-closeout/project.md` (mount point 3 of 4)"
  verifying_test: "Tier 3 mount-point walk: `links.kb` read off `.bklg/docs-that-teach/durable-audience-closeout/project.md` and compared to the landed atom set, with the single `redkiln record-links HS-P0025 --atom` transcript cited in ordering sequence (merge-gate steps 1 and 4)"

- id: AC-005
  criterion: "GIVEN the project later reaches `closeout` and `closure.md` is rendered for the first time, WHEN whoever runs that stage opens this story's folder, THEN they find the `## Knowledge Harvest` rows already written in `.redkiln/templates/closure.md`'s exact three columns (`\\| KB id \\| Kind \\| Summary \\|`), one row per landed atom, ready to transcribe rather than re-derive — and THEN they find that this story advanced no stage and wrote no `closure.md`, because that file does not exist yet and claiming a row was written into it would be a fabricated record."
  satisfied: false
  evidence: ""
  mount_point: "`.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_harvest-rows.md` — the authored `## Knowledge Harvest` rows for transcription into `closure.md` at `closeout` (mount point 4 of 4)"
  verifying_test: "Tier 2: `_harvest-rows.md` header compared character for character to `.redkiln/templates/closure.md:18-19`, one row per landed id; Tier 1: no `closure.md` in the diff (`.redkiln/processes/project.yaml:75-79`)"

- id: AC-006
  criterion: "GIVEN the wave already proved `redkiln validate --kb` green on the atoms **as landed**, WHEN the reciprocal edges and the appended map section have changed the frontmatter and content the validator reads, THEN `redkiln validate --kb` is re-run on that tree and exits zero — this story's own obligation, not a second reading of the wave's — and `redkiln doctor` still reports **exactly six** `template-drift` advisories, a seventh or a missing one being flagged as a routed gap rather than silenced, and `redkiln adopt --templates` never run."
  satisfied: false
  evidence: ""
  mount_point: "The mounted tree itself — `.kb/maps/domain-map.md` plus the edged `.kb/product/` atoms, validated after mounting"
  verifying_test: "Tier 1: `redkiln validate --kb && redkiln doctor` (merge-gate steps 1 and 2, `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` `## Testing brief`), both transcripts cited with the tree sha"

- id: AC-007
  criterion: "GIVEN a reader who has never heard of this initiative opens `.kb/README.md` — the corpus front door — WHEN they follow links whose text names each destination, THEN they reach **every** atom this wave landed in at most two hops, and the walk is recorded in prose naming the starting file, each hop and each atom reached; and GIVEN `.kb/README.md` carries no markdown link to `.kb/maps/domain-map.md` today, WHEN that gap still holds at implementation time, THEN one bullet naming `maps/domain-map.md` as the subject-matter index is appended there (addition only) so the walk is genuinely two hops rather than `ls .kb/product/`."
  satisfied: false
  evidence: ""
  mount_point: ".kb/README.md — the corpus front door, hop 1 of the two-hop walk into `.kb/maps/domain-map.md`"
  verifying_test: "Tier 3: `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) plus the recorded walk in `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md` checked for set equality against the landed atoms (IQ-5, AC-UX-08)"

- id: AC-008
  criterion: "GIVEN U3 wants to correct something this closeout promoted, WHEN they read this story's diff to learn the route, THEN they find that mounting never became authoring: **no new file appears under `.kb/`** in this story's diff, `.kb/decisions/` is untouched in every form (no addition, no modification, no `related` edge written into one), HS-P0021's carried playbook body and frontmatter are unedited, `.kb/_intake/README.md` is byte-identical and `.kb/_intake/` stays cleared, no atom id is renamed or reused — so the only correction route the tree offers is a **new atom that supersedes**, never an edit."
  satisfied: false
  evidence: ""
  mount_point: "The whole `.kb/` diff surface — the boundary between mounting and authoring"
  verifying_test: "Tier 1: `git diff --stat .kb/` · `git diff .kb/decisions/` · `git diff .kb/_intake/` · `git diff .kb/playbooks/`, plus `redkiln validate --kb`'s accepted-decision check against `HEAD` (AC-A01, AC-A09, AC-A10, T5)"

- id: AC-009
  criterion: "GIVEN U2 must be able to find the one atom that is missing one of its four references, WHEN they open this story's mount-point roll-call, THEN every landed atom is named once with all four of its registrations quoted — its domain-map line, its reciprocal edge, its `links.kb` entry and its Knowledge Harvest row — with the **same id string** in all four, and THEN every one of those ids was read off the landed frontmatter rather than predicted by this spec, so no reference dangles; an atom missing any one of the four is reported as unmounted rather than counted."
  satisfied: false
  evidence: ""
  mount_point: "`.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md` — the four-reference roll-call across all four mount points"
  verifying_test: "Tier 3 mount-point walk: a four-column table with one row per atom and no empty cell (AC-A04, `four references, or it is not mounted`), cross-checked by `rg -n \"^id:\" .kb/product .kb/open-questions .kb/playbooks` and `redkiln validate --kb`"

- id: AC-010
  criterion: "GIVEN U2 reads this story's diff and GIVEN half the corpus's readers will meet it as plain text in a quote or a diff hunk rather than in a renderer, WHEN they read any entry, bullet or row this story added, THEN every state it expresses is a **literal word** — no emoji, tick, colour word, strikethrough, empty cell, ordering or absence carries meaning; a superseded atom is annotated in words and kept, never removed; index bullets state `Open` / `Withdrawn` / `Superseded` first, then the link, then the id, then one sentence, appended under the question's existing domain section rather than a new `##`; the map section groups entries by kind under bold labels and composes the map's existing format rather than a new one; and no `.kb/product/` subdirectory, no frontmatter key outside `.kb/_templates/atom.md` and no bespoke per-atom table appears anywhere."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/domain-map.md and .kb/maps/open-questions-index.md — the composed presentation of the mounted entries"
  verifying_test: "Tier 2 review against `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` `## UX brief` `### Accessibility floor`, AC-UX-07, AC-UX-09 and the `Hand-rolling, explicitly forbidden here` list, read beside `.kb/maps/open-questions-index.md:136-143`; Tier 1 sweeps `rg -n \"✅|❌|🟢|~~\" .kb/maps .kb/README.md` and `rg -n \"^(kind|authority_tier):\" .kb/product`"
```

## Routed gaps carried by this story

Recorded here rather than fixed, because fixing either would break an acceptance criterion or
leave this project's declared diff surface.

| Gap | Where it lives | Why it is routed rather than fixed | Who should own it |
| --- | --- | --- | --- |
| `.kb/maps/domain-map.md:35` reads *"This is the map's first wave. One domain exists so far."* and is false once the second `##` section is appended. | `.kb/maps/domain-map.md:35` | Correcting it is a deletion line, which is IQ-3's literal falsifier and fails AC-002. One stale sentence costs less than "append-only" ceasing to be decidable by `git diff`. | A future `/redkiln:kb-ingest` wave's Maps phase, or a dedicated map-maintenance story that can afford a non-append edit. |
| The map atom's `last_reviewed: 2026-08-10` is not bumped by this story. | `.kb/maps/domain-map.md` frontmatter | Rewriting a scalar is a deletion plus an addition. Adding an id to a `related:` list is an addition and is fine; changing a date is not. | The same future wave that owns the preamble. |
| The fourth mount point's row cannot be written into `closure.md`, which does not exist yet. | `.redkiln/processes/project.yaml:75-79` — `closeout` is the only stage that produces it | This story advances no stage, and every stage transition belongs to the orchestrating command. AC-005 converts the write into an authored, transcribable companion instead. | HS-P0025's `closeout` stage, transcribing `_harvest-rows.md`. |
