# Wave `2026-08-17-adr-0022-append-condition` — retrospective

Fifteen agents, zero errors, `degraded: []` — nothing dropped, no isolation trip, no retry
exhausted. The wave produced every operation it planned and then **stopped without committing**, on
a gate failure it had already proved was not its own.

## The gate that stopped it, for the third consecutive wave

`redkiln doctor` exits 1 on nine errors, all one shape:

> `foundation story 'HS-S####' is consumed by no capability slice in initiative 'from-contract-to-published-library'`

Story ids `HS-S0002`, `HS-S0034`, `HS-S0035`, `HS-S0067`, `HS-S0074`, `HS-S0075`, `HS-S0100`,
`HS-S0108`, `HS-S0120` — all under `.bklg/`, none under `.kb/`, and **none touched by any of this
wave's fourteen operations or six intake documents**. There is no KB-side fix in scope; the
"fix the offending atom, re-run" step does not apply, and a re-run reproduces the identical nine.

It is upstream **[redkiln#122](https://github.com/Wet-Ink-Corporation/redkiln/issues/122)**: the
`diagnoseUnconsumedFoundations` predicate answers a **reachability** question with a **single-hop**
test, so a foundation story feeding capability work through one intermediate foundation story is
indistinguishable from one feeding nothing. Seven of the nine are false positives by transitive
reachability.

**Verified pre-existing, independently, for the third time.** The 2026-08-15 wave's `04` records the
identical nine ids, confirmed by stash-and-reset to `HEAD` and by a second worktree at the same
commit. This wave's orchestrator confirmed it again by a different route: `redkiln doctor --cwd`
against the **initiative worktree**, which carries none of this wave's `.kb/` changes — exit 1,
same nine.

Both prior waves were finished by hand under an explicit human decision to treat the failure as out
of scope. **This is the third.** The 2026-08-13 retrospective predicted that a gate which can
establish innocence but cannot act on it would be routed around every time; three waves in five days
is that prediction closing.

**And the severity has risen since that prediction.** CI's `backlog` job
(`.github/workflows/ci.yml:177`) asserts the `unconsumed-foundation` list is **empty**. So #122 is
not tidiness — it is a **release blocker for the initiative**, to be settled before
`publication-and-positioning`, not at the PR. Two of the nine (`HS-S0034`, `HS-S0035`) are
`sqlite-durable-store`'s own stories, so the initiative is now generating the errors it must clear.

The six `template-drift` warnings on `discover.md`, `gates/discover.md`, `gates/intake.md`,
`spec.md`, `_design.md` and `_intake-brief.md` are the permanent expected set per `CLAUDE.md`, not
findings.

## What the wave declined to do, and was right to

**Three obligations were refused as KB atoms because they are tasks, not knowledge.**
`.kb/open-questions/README.md:37-38` says so in terms — *"a task … is a backlog item in `.bklg/`,
not a KB atom"* — and the wave carried each forward visibly rather than letting the refusal read as
an omission:

- **N2** — `read_through` is dead code in eight `wasm32` feature combinations
  (`crates/happenstance-core/src/projection_memory.rs:233`); CI's ambient `RUSTFLAGS=-D warnings`
  makes it a failure there, and it pre-dates phase 7. **Owed: one backlog item.**
- **N3** — a private module can shadow a glob-re-exported one (`mod projection;` in `happenstance`
  silently shadowed `happenstance_core::projection` through `pub use happenstance_core::*`, warning
  only `hidden_glob_reexports`). `_design.md`'s anti-pattern 14 is written about *type* names; this
  arrived through a *module* name. Destination is `standards/rust/`, explicitly not `.kb/`.
  **Owed: a candidate rule atom in the constitution.**
- **ES-17** — ADR-0012's falsifier item 1 (two builds of one adapter differing only in `append`'s
  ownership) is *not* discharged by the phase-8 experiment, which varies strategy rather than
  ownership, and no story in the current map is scheduled to take that measurement. The KB half is
  recorded as `kb-open-question-es-17-two-adapter-measurement-001`; the queue row is not an atom's.
  **Owed: one `redkiln new`, or a recorded deferral with a named owner.**

**One placement decision was flagged rather than guessed** — the `domain-map.md` section for phase 8
/ adapter storage. See `03`.

## What this wave finally performed

**PS-32's correction to `references/adr/0007-projection-runner-decodes.md`.** Three waves — ADR-0007's
own, ADR-0017's, and ADR-0030's — closed the record of the obligation while the false sentence stood.
The wave itself correctly refused it again (it is not a `.kb/` op, and the file is not an atom), and
flagged that closing Op 5 while the sentence stood *"would be the fourth wave to carry the substance
forward unperformed."* It was performed by hand at the finish gate, with `RUNBOOK.md:4025`'s derived
claim alongside it.

**The mechanism is worth keeping, because it generalises.** The correction could not be inserted
where a reader would most want it: five line-anchored citations point into that record, three of
them from `spec/SPECIFICATION.md`, and `spec-trace` is a gate step. An insertion above the anchors
silently re-points all five — the exact cross-reference class this initiative has now hit six times
(`cargo doc` default features, eight rustdoc→spec citations, the constitution's `file:line`
evidence, `spec_trace.rs`, and twice here). The correction was therefore written *into an existing
line* at `:38` and appended *below* the last citation, with all five anchors verified byte-identical
afterwards.

**A rule that falls out of it:** in this repository, an append-only edit to a cited long-form record
is safe and an insertion is not. Nothing enforces that, and nothing checks a rustdoc- or
backlog-into-`references/` citation at all.

## What went well

- **`degraded: []` on the largest wave yet.** Six documents, fifteen agents, fourteen operations,
  zero drops. The 2026-08-13 wave took three isolation trips on four documents.
- **Cross-file dedup actually fired.** Op 8 merged a defect from *two* different intake documents
  into one open question, on the intake's own observation that C2 is *"D-1's other face"*. The bias
  toward MERGE over new held: eleven creations against four mutations, with every creation justified
  by a layer no existing atom owned.
- **The immutability contract held under pressure.** An accepted decision was superseded, not
  edited — three frontmatter lines, zero body lines — and the serial ordering of mutating ops made a
  double-write observable (Op 6 had already flipped two of the three) instead of turning it into a
  conflict.
- **The wave corrected a dangling id from its own intake** rather than propagating it:
  `kb-open-question-cf-40-fixture-limits-ownership-001` did not exist, and the atom it meant did.

## What is owed, carried forward

1. **redkiln #122** — release blocker, unchanged and now three waves deep. Settle before
   `publication-and-positioning`.
2. **The `domain-map.md` phase-8 section** — one placement decision.
3. **N2, N3, ES-17** — two backlog items and one constitution rule atom, per above.
4. **The narrow supersession spelling for ADR-0031 ↔ `kb-decision-0007`** is confirmed and stands,
   but it is confirmed *by a human at this wave's gate*, not by anything mechanical. If a later wave
   wants the full spelling it is one frontmatter edit — which is precisely why the narrow reading was
   chosen.
