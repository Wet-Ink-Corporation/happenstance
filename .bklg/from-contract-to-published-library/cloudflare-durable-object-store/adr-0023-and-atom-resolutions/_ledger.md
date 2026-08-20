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
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the `/redkiln:kb-ingest` wave itself. The command carries `disable-model-invocation: true` (`commands/kb-ingest.md`), runs Stage A inline to create **its own** dedicated worktree, and gates on a human before the workflow touches anything; the workflow then commits the whole wave on that branch and the human merges it. Nothing in this slice may perform it, and performing it in *this* worktree would be the wrong tree and the wrong gate. What is staged and ready: four intake documents under `.kb/_intake/`, the proposed action plan with the wave id, the two refusals the gate must be prepared to make, and the post-wave verification list — all in `coordination-note.md`, Part 2.
  mount_point: ".kb/_governance/integration-waves/<wave-id>/03-integration-summary.md"
  verifying_test: "git log --oneline -- .kb/decisions/ (atom attributed to the ingest wave's commit) + existence of .kb/_governance/integration-waves/<wave-id>/03-integration-summary.md listing the atom"

- id: AC-003
  criterion: "GIVEN a gate reader who opens `.kb/maps/decision-map.md` six months from now having done none of this work, WHEN they ask what phase 9 settled about the Cloudflare adapter, THEN one row on that map points at a single accepted atom `.kb/decisions/0023-*.md` that states the `SqlStorage` mapping **and** the off-tokio harness as ONE question, the atom names the single body of evidence that makes the conjunction one decision (the suite executing under `workerd` against the real bindings) and cites `.kb/playbooks/one-decision-per-adr-title.md`'s own stated exception, and **no second atom splits the pair** — an atom in `.kb/decisions/` that is absent from the map is the KB's analogue of a component constructed but never rendered and fails this criterion."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave. `.kb/decisions/0023-*.md` and its row on `.kb/maps/decision-map.md` are the wave's output. The content is staged in `.kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`, including the *one question, two consequences* paragraph and its citation of `.kb/playbooks/one-decision-per-adr-title.md`'s own stated exception, and the long form is on disk at `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; ls .kb/decisions/0023-*.md returns exactly one path; rg -n \"0023\" .kb/maps/decision-map.md returns the row"

- id: AC-004
  criterion: "GIVEN the adapter author who later has to stand a fourth conformance harness up on a runtime nobody has tried, and whose stated fear is discovering late that the ground was already walked, WHEN they read ADR-0023 and its long-form record, THEN each rejected alternative is named **with the reason it lost** — at minimum `vitest-pool-workers` as its own CI job, `wasm-bindgen-test-runner` under node with no Durable Object, and a probe-gated step carrying no compensating mandatory assertion — the same treatment covers the mapping side (the ceiling capture, the DCB query rendering, and `JsThrow` versus `StringifiedThrow`), and the *winning* harness is recorded as `every-rule-under-workerd`'s finding rather than chosen here."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave, for the atom half. The rejected alternatives and their losing reasons are authored in full — `vitest-pool-workers` as its own CI job, `wasm-bindgen-test-runner` under node with no Durable Object, and a probe-gated step with no compensating mandatory assertion, plus the mapping side's ceiling capture, the `Query::index_arms()` decomposition and `JsThrow` versus `StringifiedThrow` — at `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` §2.5 and §3.3, and staged for the atom in `.kb/_intake/0023-…`. The winning harness shape is recorded as `every-rule-under-workerd`'s finding rather than chosen here.
  mount_point: ".kb/decisions/0023-*.md (and references/adr/0023-*.md)"
  verifying_test: "Review gate against .kb/decisions/README.md (state the alternatives that lost): each named alternative and its losing reason present in .kb/decisions/0023-*.md or references/adr/0023-*.md, winning shape traced by citation to every-rule-under-workerd's merged artefact"

- id: AC-005
  criterion: "GIVEN `RUNBOOK.md:4280-4282` instructing that ADR-0001's `provisional` marker be \"formally retired and this adapter cited\", and GIVEN the marker was already lifted at phase 1 with ADR-0008 recording the lift, WHEN this story discharges that instruction, THEN `.kb/decisions/0001-async-port-flavours.md` is byte-identical to `main`, the discharge takes the form of a citation *inside* ADR-0023 and the long-form record, and the record states **why the runbook's wording was read and not obeyed** — so the next reader does not attempt the edit the instruction invites."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave, for the atom half. The discharge-by-citation is written in full at `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` §4, including **why the runbook's wording was read and not obeyed**. `git diff main -- .kb/decisions/0001-async-port-flavours.md` already produces no output and `redkiln validate --kb` passes.
  mount_point: ".kb/decisions/0023-*.md"
  verifying_test: "git diff main -- .kb/decisions/0001-async-port-flavours.md produces no output; redkiln validate --kb (accepted-atom immutability against HEAD)"

- id: AC-006
  criterion: "GIVEN the evaluator deciding in a bounded sitting whether this library tells the truth about the runtime it claims, WHEN they ask whether ADR-0009's ES-6 prediction survived contact with a real `worker::Error`, THEN the answer exists as **new content** — the verdict recorded in ADR-0023 or its own atom, citing `caller-visible-error-verdict`'s committed reconstruction test by path — `.kb/decisions/0009-error-send-sync.md` is byte-identical to `main`, and any contradiction of ADR-0009 is expressed as a superseding atom rather than a clarifying line; project AC-005's older wording (\"bound added, or ADR-0009's deferral confirmed\") is corrected in the record, because ES-6 was settled rather than deferred."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave, for the atom half. The ES-6 verdict is authored as new content at `references/adr/0023-…` §5 and staged at `.kb/_intake/es-6-verdict-against-adr-0009s-prediction.md`, citing `caller-visible-error-verdict`'s four committed reconstruction tests in `crates/happenstance-cloudflare/src/lib.rs` by name, and correcting project AC-005's older *deferral* wording. `git diff main -- .kb/decisions/0009-error-send-sync.md` already produces no output.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "git diff main -- .kb/decisions/0009-error-send-sync.md produces no output; redkiln validate --kb; review gate that the verdict paragraph cites the reconstruction test's real path under crates/happenstance-cloudflare/"

- id: AC-007
  criterion: "GIVEN a reader whose value from the corpus is knowing not only what was decided but **what was unknown on the day it was decided**, WHEN CF-40's ownership question is answered, THEN `.kb/open-questions/cf-40-fixture-limits-ownership.md` still exists with its body byte-identical, its `status` moved to `withdrawn` or `superseded`, its `related` extended with the answering atom's id, its answer covering sub-question 2 (whether the fixture contract has one owning document or is amended piecemeal by whichever ADR needs the next capability), and its bullet on `.kb/maps/open-questions-index.md` **annotated in place rather than removed** — the file is never `git rm`ed and the question is never rewritten into its own answer."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave. The `status`/`related` flip on `.kb/open-questions/cf-40-fixture-limits-ownership.md`, and its annotation on `.kb/maps/open-questions-index.md`, are the wave's writes. The answering content — covering sub-question 2 with a third data point rather than a second argument — is staged at `.kb/_intake/cf-40-fixture-contract-ownership-resolution.md`, which states explicitly that the question's body is left byte-identical and that the file is never `git rm`ed.
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "git diff main -- .kb/open-questions/cf-40-fixture-limits-ownership.md shows frontmatter hunks only and zero body hunks; git diff --diff-filter=D --name-only main -- .kb/open-questions/ is empty; redkiln validate --kb"

- id: AC-008
  criterion: "GIVEN the edge developer who needs to know whether this runtime's memory ceiling forces a peer to buffer a payload it cannot hold, WHEN `wf-11-memory-ceiling-falsifier`'s finding is recorded, THEN `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` reaches a resolved state on **either** outcome — fired: WF-11's `MUST` re-scopes to formats rather than peers and the re-scoping decision is handed to `replication-identity-and-ingest` (HS-P0017); did not fire: the condition is not constructible on this runtime and the record names what *would* construct it — with the body byte-identical, the index bullet annotated in place, and **no wire-format change anywhere in this PR**."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave. The resolution is staged at `.kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md` and carries the *did not fire* branch with the measured numbers `wf-11-memory-ceiling-falsifier` produced — verdict (c), no ceiling found within 2,169 pages, and 36,604,834 bytes as the firing payload at the platform's documented ceiling. `git diff --name-only main -- crates/ spec/` is empty for this story's commit and no wire-format change appears anywhere in it.
  mount_point: ".kb/maps/open-questions-index.md"
  verifying_test: "git diff main -- .kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md shows frontmatter hunks only; git diff --name-only main -- crates/ spec/ is empty for this story's commits; redkiln validate --kb"

- id: AC-009
  criterion: "GIVEN that `spec/SPECIFICATION.md` cites line ranges existing only in `references/adr/`, and that directory is validated by nothing, WHEN ADR-0023's long-form record lands, THEN `references/adr/0023-*.md` is a **new** file carrying what a ~100-line atom cannot — the compiler transcripts, `measured-store-limits`' measurement tables, and the rejected alternatives in full — **no existing file under `references/adr/` is modified**, and `cargo xtask spec-trace` is green."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is **partly satisfied, and the missing half is the atom.** `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` is a **new** file, on disk, carrying the rejected alternatives in full, the measured dependency-reversal figures and the `cargo deny check bans` finding; `git status --short -- references/adr/` shows it as the only untracked addition and this story modifies no existing file there; `cargo xtask spec-trace` is green — *"traceability: no problems found; §7.1–§7.2 matches the checker"*. What is missing is the review-gate half this AC also names: *the atom links the record*. There is no atom until the wave runs.
  mount_point: "references/adr/0023-*.md (linked from .kb/decisions/0023-*.md)"
  verifying_test: "git diff --name-status main -- references/adr/ shows exactly one A row and zero M rows; cargo xtask spec-trace"

- id: AC-010
  criterion: "GIVEN `.kb/_governance/integration-waves/` already holds `2026-08-10-intake` and `2026-08-10-intake-2`, and GIVEN `.kb/_intake/README.md` is a README rather than an atom that the default glob nevertheless picks up, WHEN the wave runs, THEN its id carries a suffix so it writes a **new** audit-trail directory instead of overwriting an earlier wave's, `.kb/_intake/README.md` is dropped at the approval gate and is still on disk afterwards, and `.kb/_intake/` is otherwise empty — a file still sitting there after a successful run is a file the wave did not ingest."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the same wave. The wave id is proposed and suffixed — `2026-08-19-intake-phase-9`, following `2026-08-10-intake-2`'s precedent — the `.kb/_intake/README.md` drop at the approval gate is specified, and the fifth already-tracked intake file (`0034-what-the-phase-8-reconciliation-cost.md`, staged since `4ad58d0` and never ingested) is named so the gate expects it rather than being surprised by it. All in `coordination-note.md`, Part 2.
  mount_point: ".kb/_governance/integration-waves/<wave-id>/"
  verifying_test: "ls .kb/_governance/integration-waves/ shows a third distinctly-named directory with main's two intact; ls .kb/_intake/ returns README.md and nothing else; git diff --diff-filter=D --name-only main -- .kb/_governance/ is empty"

- id: AC-011
  criterion: "GIVEN a reader following `RUNBOOK.md`'s ADR queue to find out whether 0023 was ever written, WHEN they reach the queue row at `RUNBOOK.md:302`, THEN it is struck through and rewritten in the shape `RUNBOOK.md:295` already uses for ADR-0016 — pointing at the atom by path and stating in one sentence what it settled — phase 9's ADR-0023 work box (`RUNBOOK.md:4267-4268`) is ticked, and **no other `RUNBOOK.md` line is touched by this story**, because the phase 9 ledger paragraph and the CF-14/CF-27 re-reads belong to slice-mate `deferral-re-reads-and-es-32-verdict`."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the wave, and **deliberately not done before it**. Striking `RUNBOOK.md:302`'s queue row and ticking phase 9's ADR-0023 work box would point a reader at `.kb/decisions/0023-*.md`, which does not exist, and would claim an acceptance that has not happened — the same inversion the phase's proof-artefact section was rewritten to prevent. The two edits and the shape they must take (`RUNBOOK.md:295`'s ADR-0016 form) are specified at the end of `coordination-note.md`, Part 2.
  mount_point: "RUNBOOK.md:302"
  verifying_test: "git diff main -- RUNBOOK.md for this story's commits confines every hunk to the queue row and the ADR-0023 work box; cargo xtask spec-trace"

- id: AC-012
  criterion: "GIVEN this PR lands no Rust and therefore has **no compiler standing behind it**, WHEN the wave is complete and the story is proposed as done, THEN `redkiln validate --kb` is clean, `redkiln doctor` reports exactly **six** `template-drift` advisories and no seventh, `cargo xtask spec-trace` is green, `cargo xtask affected --base main` is green, and `redkiln adopt --templates` has not been run at any point in the story — the four commands are the whole of the automation available here and all four are part of the story, not of a follow-up."
  satisfied: false
  evidence: >-
    BLOCKED — the missing dependency is the wave. Three of the four commands are green **now** — `redkiln validate --kb` passes, `redkiln doctor` reports exactly six `template-drift` advisories and no seventh, `cargo xtask spec-trace` and `cargo xtask affected --base main` are green — and `redkiln adopt --templates` has not been run at any point. But this AC's own WHEN is *when the wave is complete*, and the commands have to be re-run against the tree the wave produces before it can be flipped.
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; redkiln doctor (template-drift advisory count asserted at exactly six); cargo xtask spec-trace; cargo xtask affected --base main"
```
