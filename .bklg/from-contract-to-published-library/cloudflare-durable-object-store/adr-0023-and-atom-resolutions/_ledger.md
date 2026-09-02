---
item: "HS-S0058"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0023 accepted, and CF-40 and WF-11 resolved rather than deleted

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story lands no Rust, so no compiler stands behind any row. `verifying_test` therefore names the
real command or the real diff assertion that decides the row — the testing brief's **process-gate
tier** (`_decomposition.md`, Testing brief Notes §1: *"No test runner is the right tool for 'is this
decision atom accepted and immutable'"*). Every `mount_point` is a real path a reader reaches the
capability through, not a code path.

```yaml
- id: AC-001
  criterion: "GIVEN `sqlite-durable-store` (HS-P0012) merges one position ahead of this project and may already have answered CF-40, and GIVEN the forbidden outcome is two independent mintings, WHEN the implementer takes their **first** action on this story — before authoring a single intake document — THEN they look in `.kb/decisions/`, `.kb/maps/decision-map.md` and HS-P0012's merged specs, and record in this story's own folder which branch holds (A: a resolution exists, so this story *cites* it and names the owning atom; B: none exists, so this story stages the resolution and records that HS-P0012 cites it) together with the evidence for that reading — so that exactly one minting of CF-40's answer exists across both projects."
  satisfied: true
  evidence: >-
    `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/adr-0023-and-atom-resolutions/coordination-note.md`, Part 1 — written **before a single intake document**, which is the order this AC fixes. **Branch B**, with the evidence quoted in the note: `ls .kb/decisions/` returns no `0023-*` and no atom minting CF-40's answer, and `rg -n "CF-40" .kb/decisions .kb/maps` returns five hits, none of them a resolution. The decisive one is `sqlite-durable-store`'s own decision: `.kb/decisions/0022-append-condition-strategy.md:93-94` records CF-40 as a non-verdict with a named owner — *"CF-40's clause home … stays open at `kb-open-question-cf-40-ownership-001`"* — and its frontmatter summary repeats it at `:30`. `.kb/maps/open-questions-index.md:171-173` still carries the bullet as **Open**. So HS-P0012 deliberately did not mint it; this story stages the resolution (`.kb/_intake/cf-40-fixture-contract-ownership-resolution.md`) and HS-P0012 cites it, and exactly one minting exists across both projects. `git diff --name-only main -- .kb/decisions/` for this story is empty — nothing was typed into `.kb/decisions/` by hand.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -n \"CF-40\" .kb/decisions .kb/maps (output quoted in the committed coordination note under .bklg/from-contract-to-published-library/cloudflare-durable-object-store/adr-0023-and-atom-resolutions/) + diff review for a second minting"

- id: AC-002
  criterion: "GIVEN that **no automated check in this repository can distinguish an ingested atom from a hand-written one** — `redkiln validate --kb` checks conformance and immutability, never provenance — and GIVEN that hand-authoring `.kb/decisions/` was already done once here and reverted (`CLAUDE.md`, **Where the work lives**, commit `0269720`), WHEN this story produces its knowledge-base content, THEN every new atom and every open-question metadata flip arrives through a `/redkiln:kb-ingest` wave staged from `.kb/_intake/*.md` and gated by a human, and the branch's history shows the wave's own commit introducing them — never a file typed directly into `.kb/decisions/`."
  satisfied: true
  evidence: >-
    The wave ran, and its provenance is the evidence. `git log --oneline --
    .kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` returns **exactly one** commit —
    `6b0fe33 kb: ingest wave 2026-08-20-intake-phase-9 — ADR-0023, ADR-0034 and phase 9's evidence` — so the
    atom is attributed to the wave's own commit and to no hand-edit, and this story's staging commit `d3030c6`
    shows `A references/adr/0023-…` and nothing at all under `.kb/decisions/`. The wave was performed by
    `/redkiln:kb-ingest` in its **own** worktree behind the human gate and merged at `3ac4bf1`, exactly as
    `coordination-note.md` Part 2 asked. Its audit trail is on disk at
    `.kb/_governance/integration-waves/2026-08-20-intake-phase-9/` — `00-corpus-match.md`,
    `01-claims-and-classification.md`, `02-placement-and-adjudication.md`, `03-integration-summary.md` — and
    `03-integration-summary.md:16-17` lists both new atoms by path and id.
  mount_point: ".kb/_governance/integration-waves/<wave-id>/03-integration-summary.md"
  verifying_test: "git log --oneline -- .kb/decisions/ (atom attributed to the ingest wave's commit) + existence of .kb/_governance/integration-waves/<wave-id>/03-integration-summary.md listing the atom"

- id: AC-003
  criterion: "GIVEN a gate reader who opens `.kb/maps/decision-map.md` six months from now having done none of this work, WHEN they ask what phase 9 settled about the Cloudflare adapter, THEN one row on that map points at a single accepted atom `.kb/decisions/0023-*.md` that states the `SqlStorage` mapping **and** the off-tokio harness as ONE question, the atom names the single body of evidence that makes the conjunction one decision (the suite executing under `workerd` against the real bindings) and cites `.kb/playbooks/one-decision-per-adr-title.md`'s own stated exception, and **no second atom splits the pair** — an atom in `.kb/decisions/` that is absent from the map is the KB's analogue of a component constructed but never rendered and fails this criterion."
  satisfied: true
  evidence: >-
    One row, one atom, and no split. `ls .kb/decisions/0023-*.md` returns exactly one path,
    `.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`, and `rg -n "0023"
    .kb/maps/decision-map.md` returns its row at `.kb/maps/decision-map.md:211` — *"ADR-0023 | kb-decision-0023
    | The SqlStorage mapping and the off-tokio harness, settled by one body of evidence | accepted | 9"* — with
    the wave's own section at `:198-200` and its note at `:26`. The atom states the conjunction as **one**
    question and names the single body of evidence that makes it one — the conformance suite executing against
    the real `worker` bindings, off tokio, inside one `cargo xtask ci` (`:15-18`) — and cites
    `kb-playbook-one-decision-per-adr-title-001`'s own stated exception at `:81`, with the playbook in `related`
    at `:61`. No second atom splits the pair: the wave's other new atom,
    `.kb/decisions/0034-the-fixture-contract-has-no-single-owner.md`, is CF-40's resolution, a different
    question with its own open-question home. `redkiln validate --kb` passes, so the atom is neither absent from
    the map nor unreachable from it.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; ls .kb/decisions/0023-*.md returns exactly one path; rg -n \"0023\" .kb/maps/decision-map.md returns the row"

- id: AC-004
  criterion: "GIVEN the adapter author who later has to stand a fourth conformance harness up on a runtime nobody has tried, and whose stated fear is discovering late that the ground was already walked, WHEN they read ADR-0023 and its long-form record, THEN each rejected alternative is named **with the reason it lost** — at minimum `vitest-pool-workers` as its own CI job, `wasm-bindgen-test-runner` under node with no Durable Object, and a probe-gated step carrying no compensating mandatory assertion — the same treatment covers the mapping side (the ceiling capture, the DCB query rendering, and `JsThrow` versus `StringifiedThrow`), and the *winning* harness is recorded as `every-rule-under-workerd`'s finding rather than chosen here."
  satisfied: true
  evidence: >-
    Each rejected alternative is named **with the reason it lost**, on both sides. Harness side,
    `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md:153-160`: `vitest-pool-workers` as
    its own CI job (*"loses to AC-004's same run as the rest of the gate … a separate CI job is a claim about
    CI"*, `:157`), `wasm-bindgen-test-runner` under node with **no** Durable Object (*"the `SqlStorage` binding
    cannot be satisfied without something shaped like `DurableObjectState`"*, `:158`), a probe-gated step with
    no compensating mandatory assertion (`:159`), and a `workerd`-class runner recorded as **not rejected on
    merit** but escalated (`:160`). Mapping side, `:118-125`: the ceiling captured after the cursor opens,
    `Query::index_arms()` pushed into the contract, `StringifiedThrow`, and keeping the hand-written stand-in.
    The atom carries the same set in short form at `:155-164`. The **winning** shape is recorded as
    `every-rule-under-workerd`'s finding rather than chosen here — *"This is `every-rule-under-workerd`'s
    finding, and this record's job is to record which shape won rather than to choose one"* (`:150-151`).
  mount_point: ".kb/decisions/0023-*.md (and references/adr/0023-*.md)"
  verifying_test: "Review gate against .kb/decisions/README.md (state the alternatives that lost): each named alternative and its losing reason present in .kb/decisions/0023-*.md or references/adr/0023-*.md, winning shape traced by citation to every-rule-under-workerd's merged artefact"

- id: AC-005
  criterion: "GIVEN `RUNBOOK.md:4280-4282` instructing that ADR-0001's `provisional` marker be \"formally retired and this adapter cited\", and GIVEN the marker was already lifted at phase 1 with ADR-0008 recording the lift, WHEN this story discharges that instruction, THEN `.kb/decisions/0001-async-port-flavours.md` is byte-identical to `main`, the discharge takes the form of a citation *inside* ADR-0023 and the long-form record, and the record states **why the runbook's wording was read and not obeyed** — so the next reader does not attempt the edit the instruction invites."
  satisfied: true
  evidence: >-
    Discharged by citation, with the record saying why the instruction was read and not obeyed. `git diff main
    -- .kb/decisions/0001-async-port-flavours.md` produces **no output** — the atom is byte-identical to `main`
    — and the discharge lives inside ADR-0023: the atom at `:131` (*"ADR-0001, cited and not lifted.
    `RUNBOOK.md` instructs that ADR-0001's provisional marker be retired…"*) and its summary at `:40-42` (*"its
    provisional marker was already lifted at phase 1 and ADR-0008 records it, so RUNBOOK.md's instruction to
    retire it again is read and not obeyed"*), with the long form at `references/adr/0023-…` §4. `redkiln
    validate --kb` passes, which is the accepted-atom immutability check against `HEAD`.
  mount_point: ".kb/decisions/0023-*.md"
  verifying_test: "git diff main -- .kb/decisions/0001-async-port-flavours.md produces no output; redkiln validate --kb (accepted-atom immutability against HEAD)"

- id: AC-006
  criterion: "GIVEN the evaluator deciding in a bounded sitting whether this library tells the truth about the runtime it claims, WHEN they ask whether ADR-0009's ES-6 prediction survived contact with a real `worker::Error`, THEN the answer exists as **new content** — the verdict recorded in ADR-0023 or its own atom, citing `caller-visible-error-verdict`'s committed reconstruction test by path — `.kb/decisions/0009-error-send-sync.md` is byte-identical to `main`, and any contradiction of ADR-0009 is expressed as a superseding atom rather than a clarifying line; project AC-005's older wording (\"bound added, or ADR-0009's deferral confirmed\") is corrected in the record, because ES-6 was settled rather than deferred."
  satisfied: true
  evidence: >-
    The ES-6 verdict exists as **new content**, and ADR-0009 is untouched. `git diff main --
    .kb/decisions/0009-error-send-sync.md` produces **no output**. The verdict is folded into ADR-0023 rather
    than into ADR-0009 — the shape `coordination-note.md` Part 2 named as preferred, and the wave's first
    refusal — and reads at `.kb/decisions/0023-…:106-112` and in the summary at `:28-33`: four reconstruction
    tests executed on `wasm32-unknown-unknown` and pinned in `xtask/src/proof.rs`'s registry show a caller
    recovering constraint violation from transport fault **without** `Error` carrying `Send + Sync`, so
    ADR-0009's decision holds and this adapter is the evidence for it rather than the exception to it. That is
    the correction project AC-005's older *"bound added, or ADR-0009's deferral confirmed"* wording needed,
    because ES-6 was **settled** at phase 2 and has been `[FROZEN]` since phase 3, not deferred. The wave's own
    adjudication records the refusal to read the cluster as a clarification of ADR-0009 and deliberately leaves
    `kb-open-question-es-6-unwritable-rule-001` at `accepted`
    (`.kb/_governance/integration-waves/2026-08-20-intake-phase-9/03-integration-summary.md:38`). `redkiln
    validate --kb` passes.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "git diff main -- .kb/decisions/0009-error-send-sync.md produces no output; redkiln validate --kb; review gate that the verdict paragraph cites the reconstruction test's real path under crates/happenstance-cloudflare/"

- id: AC-007
  criterion: "GIVEN a reader whose value from the corpus is knowing not only what was decided but **what was unknown on the day it was decided**, WHEN CF-40's ownership question is answered, THEN `.kb/open-questions/cf-40-fixture-limits-ownership.md` still exists with its body byte-identical, its `status` moved to `withdrawn` or `superseded`, its `related` extended with the answering atom's id, its answer covering sub-question 2 (whether the fixture contract has one owning document or is amended piecemeal by whichever ADR needs the next capability), and its bullet on `.kb/maps/open-questions-index.md` **annotated in place rather than removed** — the file is never `git rm`ed and the question is never rewritten into its own answer."
  satisfied: true
  evidence: >-
    The question still exists, its original body is untouched, and its bullet is annotated in place.
    `.kb/open-questions/cf-40-fixture-limits-ownership.md` moved `status: accepted → superseded`, gained
    `kb-decision-0034`, `kb-decision-0023` and `kb-decision-0022` in `related`, gained the answering intake in
    `source_paths`, and moved `last_reviewed` to `2026-08-20`; `git diff --diff-filter=D --name-only main --
    .kb/open-questions/` is empty, so nothing was `git rm`ed. Sub-question 2 is answered in the negative — the
    fixture contract has **no** single owning document, and a `CF-` clause is minted by the decision that first
    needs the capability — by `kb-decision-0034`, from outside, editing none of ADR-0015, ADR-0012 or ADR-0022.
    The index bullet is annotated **in place** at `.kb/maps/open-questions-index.md:201-210`, still carrying its
    original two-claimant statement with the resolution appended. **One drafting note for a later reviewer:**
    the verification line below says *"frontmatter hunks only and zero body hunks"*, and the shape that actually
    landed is **append-only** — the original body is byte-identical and a dated `## Resolved 2026-08-20` section
    is appended below it, which is what `.kb/open-questions/README.md:42-45` requires (*"leave the body
    describing what was not known at the time … do not rewrite a question into its own answer"*). The intent the
    criterion states is met; the literal `zero body hunks` command is the wording that drifted. Recorded in
    `report.md`, *The append-not-modify shape, and a verification line that drifted*.
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "git diff main -- .kb/open-questions/cf-40-fixture-limits-ownership.md shows frontmatter hunks only and zero body hunks; git diff --diff-filter=D --name-only main -- .kb/open-questions/ is empty; redkiln validate --kb"

- id: AC-008
  criterion: "GIVEN the edge developer who needs to know whether this runtime's memory ceiling forces a peer to buffer a payload it cannot hold, WHEN `wf-11-memory-ceiling-falsifier`'s finding is recorded, THEN `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` reaches a resolved state on **either** outcome — fired: WF-11's `MUST` re-scopes to formats rather than peers and the re-scoping decision is handed to `replication-identity-and-ingest` (HS-P0017); did not fire: the condition is not constructible on this runtime and the record names what *would* construct it — with the body byte-identical, the index bullet annotated in place, and **no wire-format change anywhere in this PR**."
  satisfied: true
  evidence: >-
    The *did not fire* branch, resolved on the measurement rather than on silence.
    `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` moved `status: accepted →
    superseded`, gained `kb-reference-wf-11-memory-ceiling-verdict-001` and
    `kb-open-question-workerd-runner-absent-001` in `related`, and its original body is byte-identical with a
    dated `## Answered 2026-08-20` section appended — the same append-not-modify shape recorded under AC-007.
    The answering atom is `.kb/reference/wf-11-memory-ceiling-verdict-2026-08.md`, carrying
    `wf-11-memory-ceiling-falsifier`'s measured numbers: no ceiling found within 2,169 pages = 142,147,584
    bytes, past Cloudflare's documented 128 MiB, because the runner is `wasm-bindgen-test-runner` over Node and
    a Node isolate has no per-isolate cap; the firing payload at the documented ceiling is **36,604,834 bytes**,
    thirty-five times this adapter's declared 1 MiB `MAX_EVENT_DATA_LEN`. The missing runtime property is handed
    to `kb-open-question-workerd-runner-absent-001` and sub-question 2 to `replication-identity-and-ingest`;
    **WF-11 stays `[PROVISIONAL]`, unmoved**. No wire-format change anywhere in this story: `git show
    --name-status d3030c6 -- crates/ spec/` and the same over `6b0fe33` both return nothing. Index bullet
    annotated in place at `.kb/maps/open-questions-index.md:215-226`.
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "git diff main -- .kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md shows frontmatter hunks only; git diff --name-only main -- crates/ spec/ is empty for this story's commits; redkiln validate --kb"

- id: AC-009
  criterion: "GIVEN that `spec/SPECIFICATION.md` cites line ranges existing only in `references/adr/`, and that directory is validated by nothing, WHEN ADR-0023's long-form record lands, THEN `references/adr/0023-*.md` is a **new** file carrying what a ~100-line atom cannot — the compiler transcripts, `measured-store-limits`' measurement tables, and the rejected alternatives in full — **no existing file under `references/adr/` is modified**, and `cargo xtask spec-trace` is green."
  satisfied: true
  evidence: >-
    Both halves are now closed. `git show --name-status d3030c6 -- references/adr/` returns **exactly one `A`
    row and zero `M` rows** — `A references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` — so
    the record is a new file and this story modified no existing file under `references/adr/`; the only later
    touch is Amendment ADR-0023-A moving the record's own status line `proposed → accepted` (`f8391b8`), which
    is what that status line promised would happen when the wave ran. It carries what a 164-line atom cannot:
    319 lines including the rejected alternatives in full (`:118-125`, `:153-160`), `measured-store-limits`'
    figures, and the `cargo deny check bans` finding. The half that was missing — *the atom links the record* —
    is satisfied: `.kb/decisions/0023-…:66` names
    `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` in `source_paths`. `cargo xtask
    spec-trace` is green: *"401 citations checked … traceability: no problems found; §7.1–§7.2 matches the
    checker"*.
  mount_point: "references/adr/0023-*.md (linked from .kb/decisions/0023-*.md)"
  verifying_test: "git diff --name-status main -- references/adr/ shows exactly one A row and zero M rows; cargo xtask spec-trace"

- id: AC-010
  criterion: "GIVEN `.kb/_governance/integration-waves/` already holds `2026-08-10-intake` and `2026-08-10-intake-2`, and GIVEN `.kb/_intake/README.md` is a README rather than an atom that the default glob nevertheless picks up, WHEN the wave runs, THEN its id carries a suffix so it writes a **new** audit-trail directory instead of overwriting an earlier wave's, `.kb/_intake/README.md` is dropped at the approval gate and is still on disk afterwards, and `.kb/_intake/` is otherwise empty — a file still sitting there after a successful run is a file the wave did not ingest."
  satisfied: true
  evidence: >-
    The wave wrote a **new** audit-trail directory and consumed the intake. `ls
    .kb/_governance/integration-waves/` returns seven entries — `2026-08-10-intake`, `2026-08-10-intake-2`,
    `2026-08-13-projection-adrs`, `2026-08-15-adr-0030-checkpoint-progress`, `2026-08-15-intake`,
    `2026-08-17-adr-0022-append-condition` and the new `2026-08-20-intake-phase-9` — so `main`'s directories are
    intact and none was overwritten; `git diff --diff-filter=D --name-only main -- .kb/_governance/` is empty.
    The id is the proposed one dated the day the wave actually ran (`2026-08-20-…` rather than `2026-08-19-…`)
    and still carries the `-phase-9` suffix this criterion asks for. `ls .kb/_intake/` returns `README.md` **and
    nothing else**: all five staged documents were ingested — the four this story staged plus
    `0034-what-the-phase-8-reconciliation-cost.md`, named in advance in `coordination-note.md` Part 2 so the
    gate expected it rather than being surprised by it — and `README.md` was dropped at the approval gate and is
    still on disk.
  mount_point: ".kb/_governance/integration-waves/<wave-id>/"
  verifying_test: "ls .kb/_governance/integration-waves/ shows a third distinctly-named directory with main's two intact; ls .kb/_intake/ returns README.md and nothing else; git diff --diff-filter=D --name-only main -- .kb/_governance/ is empty"

- id: AC-011
  criterion: "GIVEN a reader following `RUNBOOK.md`'s ADR queue to find out whether 0023 was ever written, WHEN they reach the queue row at `RUNBOOK.md:302`, THEN it is struck through and rewritten in the shape `RUNBOOK.md:295` already uses for ADR-0016 — pointing at the atom by path and stating in one sentence what it settled — phase 9's ADR-0023 work box (`RUNBOOK.md:4267-4268`) is ticked, and **no other `RUNBOOK.md` line is touched by this story**, because the phase 9 ledger paragraph and the CF-14/CF-27 re-reads belong to slice-mate `deferral-re-reads-and-es-32-verdict`."
  satisfied: true
  evidence: >-
    The post-wave pass is done, and it is confined to the two edits this criterion names. The ADR-0023 queue row
    — now at `RUNBOOK.md:304`, two rows below the `:302` the criterion cites, because two unscheduled rows were
    inserted above it since — is struck through and rewritten in the shape `RUNBOOK.md:295` uses for ADR-0016:
    *"**Written**, as [ADR-0023](.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md), and
    accepted"*, pointing at the atom **by path** and stating in one sentence what it settled — one question with
    two halves, one body of evidence, the mapping holding every binding `!Send` and retaining the thrown value,
    the harness recorded as a finding. Phase 9's ADR-0023 work box at `RUNBOOK.md:4401-4413` is **ticked**, and
    its `(vitest-pool-workers as its own CI job)` parenthetical is replaced by what actually landed: the
    conformance suite on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner` against a
    `node:sqlite`-backed `DurableObjectState` shim, **one row** in `xtask/src/proof.rs`'s executed-target
    registry, inside one `cargo xtask ci`, with the losing shape cited at `references/adr/0023-…:157` and the
    `workerd`-class runner left open at `kb-open-question-workerd-runner-absent-001`. **No other `RUNBOOK.md`
    line is touched by this story:** this story's commit confines every `RUNBOOK.md` hunk to those two sites.
    The CF-14 and CF-27 cells and phase 9's session log belong to slice-mate
    `deferral-re-reads-and-es-32-verdict` and are edited in that story's own separate commit, which is the
    division this criterion itself draws. `cargo xtask spec-trace` green.
  mount_point: "RUNBOOK.md:302"
  verifying_test: "git diff main -- RUNBOOK.md for this story's commits confines every hunk to the queue row and the ADR-0023 work box; cargo xtask spec-trace"

- id: AC-012
  criterion: "GIVEN this PR lands no Rust and therefore has **no compiler standing behind it**, WHEN the wave is complete and the story is proposed as done, THEN `redkiln validate --kb` is clean, `redkiln doctor` reports exactly **six** `template-drift` advisories and no seventh, `cargo xtask spec-trace` is green, `cargo xtask affected --base main` is green, and `redkiln adopt --templates` has not been run at any point in the story — the four commands are the whole of the automation available here and all four are part of the story, not of a follow-up."
  satisfied: true
  evidence: >-
    All four commands re-run against the **post-wave** tree, which is this criterion's own WHEN. `redkiln
    validate --kb` → *"redkiln: validate passed."* `redkiln doctor` → exactly **six** `template-drift`
    advisories and no seventh (`discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`, `_design.md`,
    `_intake-brief.md`), which is the set the `backlog` CI job asserts; the remaining `doctor` lines are
    pre-existing foundation-story advisories this story does not touch. `cargo xtask spec-trace` → *"201 clauses
    (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE), 112 conformance rules, 58 e2e cases, 401
    citations checked … traceability: no problems found; §7.1–§7.2 matches the checker"*, exit 0. `cargo xtask
    affected --base main` → *"affected gate passed"*. `redkiln adopt --templates` has **not** been run at any
    point in this story — had it been, the six advisories above would have become zero and the `backlog` job
    would fail on the absence it created.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; redkiln doctor (template-drift advisory count asserted at exactly six); cargo xtask spec-trace; cargo xtask affected --base main"
```
