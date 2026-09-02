---
item: "HS-S0088"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The falsifier ledger is repaired before anything reads it

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story lands **no Rust, no manifest change and no gate step** — the whole diff is `RUNBOOK.md`
plus this folder — so no compiler stands behind any row here. `verifying_test` therefore names the
real command, the real diff assertion or the real recorded artefact that decides the row, which is
the testing brief's static tier for project AC-004 (`_decomposition.md`:467) and its explicit
statement that AC-004's ledger repair is *"a one-time content edit to `RUNBOOK.md`, not a new gate
step by itself"* (`:708-714`). The mechanised set-equality check is `clause-maturity-audit`'s and
**does not exist at this story's merge** — that is the intended state, not a gap (spec, EC-007).

Three evidence artefacts are committed under this story's own folder and are cited by the rows that
depend on them: the **distillation transcript** (each new falsifier cell beside the clause marker it
was distilled from), the **per-prefix reconciliation worksheet** (`VT` 9, `WF` 1, `ES` 9, `PS` 17,
`SY` 9, `CF` 4 = 49, against `spec/SPECIFICATION.md`:8513-8519) and the **anchor sweep** (every
`RUNBOOK.md` heading slug before and after). Each carries the commit sha and the date it was
produced (spec, NF-006). A row whose only evidence is *"I read it"* is not satisfied.

Three mount points recur because the section has two halves and one inbound reference: the **table**
at `RUNBOOK.md`:588-606, the **narrative** at `:612-635`, and phase 12's **exit criterion** at
`:4492-4497` — the reference that must resolve for the repair to be mounted at all rather than merely
present.

```yaml
- id: AC-001
  criterion: "**GIVEN** the maintainer at the publish gate must answer *\"who falsifies ES-41, and when?\"* without opening a 9,000-line specification (IQ-1's 0-hops-to-a-claim budget, `_decomposition.md`:232-239), **WHEN** they read `RUNBOOK.md`'s provisional ledger, **THEN** a row exists whose `Clauses` cell is exactly `ES-41`, whose `Falsified by` cell is a distillation of the clause's own marker (`spec/SPECIFICATION.md`:4388-4394) naming **both** exposed axes — transport (one more round trip on a store with no connection and no cursor) and completeness (a store that cannot state what it does not hold cannot distinguish *\"no such event\"* from *\"not visible to me\"*) — and the structure VT-8 does not already oblige, and whose `Owning phase` cell names the clause's own instruments: **8 (`happenstance-sqlite`) or 9 (`happenstance-cloudflare`), whichever lands first**; **AND** no word in either cell asserts anything the clause's marker does not"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md section `The 49 [PROVISIONAL] clauses` — the ledger table at RUNBOOK.md:588-606, inside the section spanning :580-635"
  verifying_test: "distillation transcript committed under .bklg/from-contract-to-published-library/publication-and-positioning/falsifier-ledger-repair/, pairing the new ES-41 row cell-by-cell against spec/SPECIFICATION.md:4388-4394; cargo xtask spec-trace (exit 0); cargo xtask affected --base main (.redkiln/config.yaml:40) green"

- id: AC-002
  criterion: "**GIVEN** ES-42 is the only one of the four additions whose marker names *this* publish as its deadline — *\"must be re-evaluated before phase 12: adding a bound to an opaque return type after publish is breaking, so this clause expires rather than drifts\"* (`spec/SPECIFICATION.md`:3083-3088) — and no other story in this project owns it (it is `[PROVISIONAL]`, so `deferred-clause-reread` does not reach it), **WHEN** the maintainer reads its row at the gate, **THEN** the `Owning phase` cell says the clause **expires at 12 — re-evaluated at this publish**, in words, not merely `12`; the `Falsified by` cell carries the marker's own exclusion *\"inconvenience is not the falsifier\"* alongside the two shapes that do falsify it (a generic method, which is not dyn-compatible; a return whose lifetime the `Pin<Box<…>>` wrapper cannot name); **AND** the row makes the handoff **visible** without performing it — no re-evaluation verdict is written here"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md section `The 49 [PROVISIONAL] clauses` — the ledger table at RUNBOOK.md:588-606, inside the section spanning :580-635"
  verifying_test: "the same distillation transcript, ES-42 section, against spec/SPECIFICATION.md:3083-3088 and §7.5's row at :9028; content check that the Owning phase cell contains the word 'expires' and the Falsified by cell carries the marker's 'inconvenience is not the falsifier' exclusion; git diff --stat showing no .kb/ path added"

- id: AC-003
  criterion: "**GIVEN** grouping is the ledger's whole argument — folding is correct only when **one** falsifier settles both, the way PS-2 alone settles thirteen rows (`RUNBOOK.md`:583-586, `:608-609`) — and CF-39 and CF-40 are adjacent in the specification and both about what a *fixture* promises, **WHEN** the maintainer reads the `CF` family, **THEN** they find **two** rows, not one: CF-39 falsified by a real adapter whose only injectable mid-batch fault is one its driver transparently absorbs — a connection killed mid-statement behind a reconnect-and-retry pool — owned by **8 (rusqlite) and 10 (Postgres)**; CF-40 falsified by a real adapter whose ceiling is not a constant — a Postgres row whose TOAST threshold depends on what else is in the row, a KV store whose per-value cap varies with the key — owned by **8, 9 and 10**; **AND** each row carries its clause's own standing fact that *no adapter has armed a fault* / *stated a ceiling* yet, so a reader cannot mistake an unexercised promise for an exercised one"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md section `The 49 [PROVISIONAL] clauses` — the ledger table at RUNBOOK.md:588-606, inside the section spanning :580-635"
  verifying_test: "the same distillation transcript, CF-39 and CF-40 sections, against spec/SPECIFICATION.md:7637-7642 and :7678-7684; the table at RUNBOOK.md:588-606 goes 17 rows to 21 with two distinct CF rows carrying different Falsified by and Owning phase cells"

- id: AC-004
  criterion: "**GIVEN** ES-10's own frozen note instructs that *\"lifting it deletes the disclosure, and the acceptance has to move in the same act or the exposure disappears silently\"* (`spec/SPECIFICATION.md`:2823-2831), so a reader who watches position allocation vanish from the ledger and does not know where it went reads the exposure as **closed**, **WHEN** the residual-exposure row is repaired, **THEN** four coordinated edits land in that one row and none of them alone: the `Clauses` cell becomes `ES-11, ES-12, ES-35, ES-40`; the bolded framing goes from *five* to *four*, matching `spec/SPECIFICATION.md`:259-264 and `:371`; the `Falsified by` cell loses its position-allocation sentence and gains, in its place, a statement that the axis is now disclosed by ADR-0013's CF-25 acceptance — one hop, to the specific atom `.kb/decisions/0013-position-assignment-and-visibility.md`, never a repository root (IQ-1); and the `Owning phase` cell becomes `10 (ES-11, ES-12), 8 (ES-35), 14 (ES-40)`; **AND** CF-26's *\"a **fixture** instrument does not falsify any of them\"* sentence survives verbatim, because it is still true of the four that remain"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md:606 — the residual-exposure row of the ledger table at RUNBOOK.md:588-606"
  verifying_test: "git diff RUNBOOK.md showing all four edits to :606 in one hunk (clause cell, five-to-four framing, replacement disclosure pointer, phase cell) with CF-26's fixture sentence unchanged; test -f .kb/decisions/0013-position-assignment-and-visibility.md resolving the one-hop pointer"

- id: AC-005
  criterion: "**GIVEN** *\"the heading was right; the rows were short\"* (`RUNBOOK.md`:612-620) is the precise failure counting to 49 produces, **WHEN** the repaired ledger's clause IDs are bucketed by prefix, **THEN** they equal §7.1's `[PROVISIONAL]` column **per prefix, not merely in total** — `VT` 9, `WF` 1, `ES` 9, `PS` 17, `SY` 9, `CF` 4, total **49** (`spec/SPECIFICATION.md`:8511-8519) — which no compensating pair of errors can pass; the arithmetic is 46 − 1 + 4 = 49 and each term is exhibited; **AND** `cargo xtask spec-trace` is green with byte-identical output before and after the change, and `git diff --stat` shows `spec/SPECIFICATION.md` untouched, so the identity was reached by repairing the ledger and not by moving the specification"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md section `The 49 [PROVISIONAL] clauses` — the ledger table at RUNBOOK.md:588-606, inside the section spanning :580-635"
  verifying_test: "per-prefix reconciliation worksheet committed under .bklg/from-contract-to-published-library/publication-and-positioning/falsifier-ledger-repair/ (VT 9, WF 1, ES 9, PS 17, SY 9, CF 4 = 49) checked against spec/SPECIFICATION.md:8513-8519; cargo xtask spec-trace run before and after with byte-identical transcripts recorded; git diff --stat naming exactly two paths and not naming spec/SPECIFICATION.md"

- id: AC-006
  criterion: "**GIVEN** `clause-maturity-audit` will parse this table by composing with `xtask/src/spec_trace.rs`'s existing parser rather than writing a second one (`_decomposition.md`:711-714), so the table's shape is a contract with a consumer that cannot yet fail, **WHEN** the repair lands, **THEN** the header row is byte-identical (`| Group | Clauses | Falsified by | Owning phase |`), the column count stays four, clause cells stay comma-separated IDs each matching `[A-Z]{2}-\\d+`, the existing ranges keep their spaced en dash (`VT-21 – VT-24`, `PS-4 – PS-6`, `PS-22 – PS-25`), the four additions are single-clause rows introducing no new range form, each addition sits with its family (the two `ES` rows near the existing `ES` rows, the two `CF` rows near `CF-17, CF-34`), and the residual-exposure row stays **last**, where its bolded framing reads as the summary it is; **AND** the section heading `### The 49 [PROVISIONAL] clauses` is unchanged, because its slug is the anchor AC-008 repoints to"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md section `The 49 [PROVISIONAL] clauses` — the ledger table at RUNBOOK.md:588-606, inside the section spanning :580-635"
  verifying_test: "git diff RUNBOOK.md showing the header row, the separator row, the section heading and every existing row except :606 unchanged; a regex sweep of the repaired table confirming every clause cell is comma-separated [A-Z]{2}-\\d+ and the three existing ranges keep their spaced en dash"

- id: AC-007
  criterion: "**GIVEN** the narrative at `RUNBOOK.md`:622-635 currently ends *\"That is five edits and it is the next pass's, not this one's\"* — and this PR **is** that pass, so leaving it instructs the phase-12 auditor to redo finished work, which is the same class of defect as the short table it complains about (IQ-7, truth at the publish commit), **WHEN** an auditor reads `:612-635` after the change, **THEN** the phase-3 paragraph at `:612-620` survives — it is the argument for why the table is *audited* rather than trusted — and the phase-5 paragraph is replaced by a **dated repair record**: what was short (ES-41, ES-42, CF-39, CF-40), what was lifted (ES-10, and where its disclosure went), the arithmetic 46 − 1 + 4 = 49, the date and commit of the repair, and the standing, explicit fact that **the set-equality check is `clause-maturity-audit`'s and does not exist yet**; **AND** the record is worded so a later pass can supersede it with a further dated entry rather than rewriting it (IQ-4's wording discipline); **AND** the stale citation in that paragraph — `SPECIFICATION.md:8138` for §7.1's totals row, which now lands in §6.6 — is corrected to the live line"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md:612-635 — the narrative beneath the ledger table, inside the section spanning :580-635"
  verifying_test: "git diff RUNBOOK.md showing :612-620 unchanged and :622-635 replaced by a dated repair record naming the four added IDs, the lifted ID, the arithmetic 46 - 1 + 4 = 49 and clause-maturity-audit as the check that does not yet exist; every file:line citation in the rewritten paragraph resolved at the commit, including the corrected §7.1 totals line"

- id: AC-008
  criterion: "**GIVEN** phase 12's own exit criterion instructs the auditor to *\"audit it against [the provisional ledger] … and `cargo xtask spec-trace`, not against prose\"* while linking `#the-46-provisional-clauses`, an anchor dead since the heading was corrected to 49 (`RUNBOOK.md`:4492-4497), so the one criterion whose entire instruction is *reach the ledger* cannot reach it, **WHEN** that criterion is read after this change, **THEN** its link resolves to the live heading; the string `#the-46-provisional-clauses` appears **nowhere** in `RUNBOOK.md`; **AND** every heading anchor `RUNBOOK.md` exposed before the edit still exists (IQ-3 — the repair may not buy one live anchor by breaking another); **AND** the four backlog and grounding files that quote the stale slug as *evidence of staleness* (`clause-maturity-audit/spec.md`:198, `crate-set-decision/spec.md`:82 and `:416`, `crate-set-decision/discover.md`:21, `_decomposition.md`:604, `_grounding.md`:117 and `:183`) are **not** edited — they are citations of a historical defect, not live links"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md:4492-4497 — phase 12's exit criterion, the inbound reference that must resolve to the ledger section at :580"
  verifying_test: "rg -n \"the-46-provisional-clauses\" RUNBOOK.md returning no match; anchor sweep transcript committed under this story's folder listing every heading slug in RUNBOOK.md before and after with an empty removed-set; git diff --stat showing no .bklg/** path outside this story's folder"
```
