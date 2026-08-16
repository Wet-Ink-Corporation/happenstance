---
item: "HS-S0018"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — ADR-0020 — fold/query agreement, and DT-2's signature answer

## Findings Ledger

> **Eight of eight ACs satisfied.** Seven were satisfied inside this story's own
> diff. **AC-008 was deferred by construction and has since been discharged by the
> handoff it named:** it asserts an accepted `.kb/decisions/0020-fold-query-agreement.md`,
> and only a **human-invoked `/redkiln:kb-ingest` wave** may author an atom — on its
> own worktree branch, which is why the evidence AC-008 asks for is a *wave commit
> sha* and could not exist inside this PR's diff. Hand-authoring it is the
> anti-pattern reverted at `0269720`, and this story's spec anticipates the deferral
> and names it correct (`spec.md`, *Clarifications resolved during spec*, item 2).
> That wave has now run and merged — `a28322b`, merged at `3fb28a1` — so the row is
> recorded satisfied against the wave's sha rather than against this diff. The row
> was flipped by a later bookkeeping pass, not by the implementer, and the atom is
> still not hand-authored.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| The hazard is real in this tree today, not hypothetical | `examples/course-subscriptions/src/main.rs:114-125` names three event types in the query; `:140-155` names the same three in the fold; the `_ => {}` arm at `:154` absorbs any divergence and nothing signals it | Closed structurally by the decision; observed by `compile-fail-proof-artefact` in M6 |
| The two divergence directions are not symmetric, and only one corrupts | Recorded as a table at `references/adr/0020-fold-query-agreement.md:60-65`. Query ⊃ fold widens the boundary (spurious conflicts, safe); fold ⊃ query narrows the log the decision is made on (the append condition protects less than the handler assumes) | None. It is the reason DR-01 and DR-02 are two rules and not one |
| Two cited ranges in the inputs were wrong; both repaired and recorded | `crates/happenstance-core/src/projection.rs:152-154`, not `:47-61` (`_decomposition.md:544-546`, `_design.md` `## Shape decision`). `RUNBOOK.md:525`, not `:524`. Recorded at `references/adr/0020-fold-query-agreement.md:381-402` | Both are **repairs** (unchanged admitted-implementation set), so neither owes a decision. The stale ranges remain in `_decomposition.md` and `_design.md`, which are signed-off artefacts this story may not edit |
| `EventStore` has four required methods, and `read_decision_model` is not one of them | `crates/happenstance-core/src/store.rs:119`, `:213`, `:248`, `:268`; `read_decision_model` is a free function at `:321` | Handed to the slice-mate, whose AC-005 turns on that surface having no hook — true either way |
| Defect candidate **D-1** is logged and routed, not repaired | `references/adr/0020-fold-query-agreement.md:268-294`: *`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs.* Nearest clause subject `VT-18` (`spec/SPECIFICATION.md:1371-1375`) | Project AC-012's defect log at closeout. Reaches the contract, if ever, through a decision record — never a line edit |
| Nothing compiled changed | `git diff --name-only` for this checkpoint lists only the two new markdown artefacts and this story's `.bklg` folder | NF-001 discharged; `cargo xtask ci --fast` unchanged by construction |

## Acceptance

| AC | Status | Verified by |
| --- | --- | --- |
| **AC-001** — staged document at the ingest mount point, composed as a record, carrying a proposed frontmatter block | **satisfied** | `test -f` red → green; `.kb/_intake/0020-fold-query-agreement.md:26-86` is the proposed frontmatter (`adr_id: ADR-0020`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, `reversibility: medium`, `depends_on: kb-decision-0003, kb-decision-0006`, `related: kb-decision-0007, kb-open-question-projection-id-unvalidated-001`, six resolving `source_paths`); `cargo xtask affected --base main` passed |
| **AC-002** — one shape, matching the signed-off design row for row | **satisfied** | `references/adr/0020-…:146-238`, five numbered decisions against `_design.md` `## Shape decision` rows 1–5 and `## Signatures` `:421-434`. No design row contradicted or re-decided |
| **AC-003** — the hazard stated against real code, resolved structurally | **satisfied** | `references/adr/0020-…:20-82`, both code blocks quoted verbatim from the worked example; DR-01/DR-02 quoted from `project.md:142-143` |
| **AC-004** — where the validation went, and what was done about the shortfall | **satisfied** | `references/adr/0020-…:240-294` in four parts plus D-1; mechanical negative `git diff --name-only` shows no `crates/`, `examples/`, `xtask/` or `spec/` path |
| **AC-005** — every alternative that lost, with the wrong implementation it admits | **satisfied** | `references/adr/0020-…:296-319`, eight rows; one-shape claim at `:314-319` in the contract crate's own words (`projection.rs:152-154`) |
| **AC-006** — DT-2's price carried honestly, as a consequence and not a second decision | **satisfied** | `references/adr/0020-…:329-345`: 11 domain lines to 26 ceremony lines, 2.4:1, and the falsifiable prediction about AC-013's verdict, explicitly owned by project closeout. `:366-367` states the record decides nothing about the derive |
| **AC-007** — the long-form record exists and is named in the proposed `source_paths` | **satisfied** | `test -f` red → green; 411 lines vs the staged document's 199; `source_paths` at `.kb/_intake/0020-…:76-83` lists both, matching `.kb/decisions/0029-…:45-47` |
| **AC-008** — the wave mints the atom, the decision map carries its row, `validate --kb` clean | **satisfied** (by the wave, not by this diff) | `/redkiln:kb-ingest` ran over this document and its slice-mate in one wave: `a28322b` (2 ops, `2026-08-15-intake`), merged at `3fb28a1`. `git log --diff-filter=A -- .kb/decisions/0020-fold-query-agreement.md` now returns exactly `a28322b` — the wave, not this story's PR. Atom accepted (`adr_id: ADR-0020`, phase 7); `.kb/maps/decision-map.md:137` and `.kb/maps/domain-map.md:174-194` carry its rows; `redkiln validate --kb` → "validate passed.", exit 0. `redkiln doctor` exits 1 on nine pre-existing `unconsumed-foundation` errors (HS-S0002/-0034/-0035/-0067/-0074/-0075/-0100/-0108/-0120) from planning commit `ae77ac4` that name neither this story nor its slice-mate and are routed as release blocker #122 (`_implementation.md:340-341`, `:399-403`) |

**The dependency this story was blocked on has since been met.** A human ran
`/redkiln:kb-ingest` over `.kb/_intake/0020-fold-query-agreement.md` **and**
`.kb/_intake/0021-payload-evolution-and-codec-tag.md` in **one** wave, on its own
worktree branch, with the suffixed wave id `2026-08-15-intake`, dropping
`.kb/_intake/README.md` from the default glob at the approval gate. The wave cleared
both staged sources. Project **DoD 3** — the atoms exist under `.kb/decisions/`,
written before the code they govern, with `redkiln validate --kb` clean
(`project.md:228-229`) — is therefore discharged, and M2
(`domain-event-and-decision-model`) now reads the accepted atom rather than a
document about to be deleted from `_intake`.

## Knowledge Harvest

- **The atom this story stages**, ADR-0020, is the harvest. It is not promoted here:
  atoms are the ingest path's to author, never a story's.
- **A transferable rule, candidate for a playbook atom at closeout:** *a derivation
  that a caller can override is not a derivation.* The mechanism — a **sealed** trait
  carrying the derived method, blanket-implemented over the trait a caller does write
  — is RS-40-2 applied to an invariant rather than to a convenience, and it is what
  turns DR-02 from a review obligation into a structure. The same shape is available
  anywhere a "derive this, do not hand-maintain it" rule exists.
- **A second, smaller one:** *record the citation repair, do not silently apply it.*
  Two ranges in signed-off inputs pointed at the wrong lines. Repairing them in place
  would have been invisible; recording them made the difference between a repair and
  an amendment checkable by `.kb/decisions/README.md`'s mechanical test.
- **Nothing under `.kb/open-questions/` is resolved by this story**, and nothing
  there has this subject. Whether `happenstance-macros` ships is a *prediction* in the
  record and a *verdict* at project closeout (AC-013) — deliberately not settled here.
