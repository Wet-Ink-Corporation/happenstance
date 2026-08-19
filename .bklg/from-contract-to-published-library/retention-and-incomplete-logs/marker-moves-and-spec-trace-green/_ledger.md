---
item: HS-S0123
stage: implement
created: 2026-08-13T00:00:00.000Z
updated: 2026-08-13T00:00:00.000Z
---

# Acceptance ledger — Three markers leave, and the census is recomputed by a human

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Two rows have no compiled test, by construction, and both are stated as such rather than hidden.**
AC-001's *settled versus implemented* obligation and AC-004's `CF-` asymmetry (`has_suite` is false,
so `spec-trace` check 4 never reads CF-27's `Rule:` line) are carried by content review against the
mutants the discover names. Their evidence must be a citation to the reviewed text plus the review
finding — never a green gate run, which passes both failures.

**Two rows are satisfied by a deliberate falsification, not by a passing run.** AC-002 and AC-003 each
require the implementer to break the property locally, record the resulting failure, and revert. The
evidence field for those rows is the recorded failure output plus the reverted state, because a green
run alone does not distinguish a live check from an absent one.

**ES-40 has two branches (spec *Clarifications resolved during spec* §2).** AC-005 is satisfied by
discharge — the expected branch — or by a renewal naming a path under `experiments/` that exists on
disk; whichever branch is taken, the evidence states which and cites it.

```yaml
- id: AC-001
  criterion: "**GIVEN** an adapter author reading ES-39 to learn whether a store may forget history and still be conformant, **WHEN** they reach the clause's maturity marker at `spec/SPECIFICATION.md:4327-4333`, **THEN** it no longer reads `[DEFERRED]`; the replacement carries ADR-0028's answer in declaration position, states in terms that **no primitive was adopted**, names what a reader is therefore on its own against, and describes incompleteness in a way that does **not** imply a boundary position — a floor is `earliest_position()` and the clause's own argument is that a regulated purge is scattered, not a prefix (`:4336-4342`)"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:4325-4349 — ES-39's clause body, composition root 7 (`_decomposition.md:143-146`), read by `xtask/src/spec_trace.rs`"
  verifying_test: "cargo xtask spec-trace checks 1-3 (xtask/src/spec_trace.rs:650-678) plus content review against the SettledByAssertion mutant (discover.md:47), corroborated by git diff --name-only showing no change to crates/happenstance-core/src/store.rs or crates/happenstance-core/src/append.rs"

- id: AC-002
  criterion: "**GIVEN** the same reader continuing to ES-39's `Rule:` line to find the rule that would check the clause, **WHEN** they read `a_store_reports_the_history_it_does_not_hold` at `spec/SPECIFICATION.md:4344-4345`, **THEN** its `(new)` annotation is still there and the marker text above explains why — the question is answered and the rule is not writable, because writing it needs a primitive nobody adopted; the reader is not left inferring that a missing rule means a forgotten one"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:4344-4345 — ES-39's `Rule:` line, read by spec-trace check 4 via the `schedules_new` exemption"
  verifying_test: "cargo xtask spec-trace check 4 (xtask/src/spec_trace.rs:680-711, exemption at :1626-1631), with the recorded deliberate falsification: `(new)` removed locally until check 4 names the unresolvable rule, then reverted"

- id: AC-003
  criterion: "**GIVEN** an evaluator scanning §7.1's maturity census to judge how much of the contract is still moving, **WHEN** CF-27 is classified, **THEN** it is classified from a marker the clause *declares* — line-initial or line-final at `spec/SPECIFICATION.md:8036` — and no longer from `maturity_of`'s first-marker-anywhere fallback, so that replacement prose quoting another clause's marker cannot silently reclassify it"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8034-8041 — CF-27's clause body and its declared marker, classified by `maturity_of` and rendered into the generated region at :8507-8753"
  verifying_test: "cargo xtask spec-trace check 1 plus maturity_of's declared branch (xtask/src/spec_trace.rs:1441-1472), with the recorded deliberate falsification: a temporary sentence quoting another clause's bracketed marker added to CF-27's body, §7.1/§7.2 confirmed unchanged, then reverted"

- id: AC-004
  criterion: "**GIVEN** an adapter author asking whether a suffix store is distinguishable from a young one, **WHEN** they read CF-27's `Rule:` line at `spec/SPECIFICATION.md:8042-8047`, **THEN** it matches the branch HS-S0122 actually took — the `(new;` clause rewritten out as a whole sentence if the rule was written, or retained with the clause text recording in writing why the rule cannot exist if it was refused — and in neither case is it left as three deleted characters inside a sentence that no longer parses as English"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8042-8047 and its generated §7.2 row at :8738 — the row daggered by resolution in `rule_cell` (xtask/src/spec_trace.rs:1147-1183)"
  verifying_test: "cargo xtask spec-trace check 6 ownership sweep (xtask/src/spec_trace.rs:725-727) and the §7.2 CF-27 row at spec/SPECIFICATION.md:8738; plus content review read-back against the branch recorded in .bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal/spec.md"

- id: AC-005
  criterion: "**GIVEN** an evaluator deciding in one sitting whether a `[PROVISIONAL]` clause is a real hedge or a parked one, **WHEN** they read ES-40 at `spec/SPECIFICATION.md:4351-4379`, **THEN** either the marker is **discharged** — the suffix store gave the clause its assertion in HS-S0117, which is the expected branch — or it is **renewed naming an experiment that exists on disk** under `experiments/`, in the shape of `experiments/position-visibility/README.md`; a renewal reading *pending further work* is a failure of this row even though it clears check 2's twelve-character floor"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:4351-4379 — ES-40's clause body and its marker at :4359, read by spec-trace check 2"
  verifying_test: "cargo xtask spec-trace check 2 (xtask/src/spec_trace.rs:659-668) for the falsifier floor; on the renewal branch, the named path confirmed present under experiments/ (shape: experiments/position-visibility/README.md) and cited in the implementation report — rejects RenewalWithoutAnExperiment (discover.md:51)"

- id: AC-006
  criterion: "**GIVEN** an adapter author who wants to run the rule ES-40 names, **WHEN** they read its `Rule:` line at `spec/SPECIFICATION.md:4368-4371`, **THEN** `condition_over_removed_history_does_not_reject` no longer carries `(new)`, because HS-S0117 wrote it — so the checker resolves the name for real instead of skipping the clause, and the author can run it by name"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:4368-4371 — ES-40's `Rule:` line, resolved by spec-trace check 4 against crates/happenstance-testkit/src/suite.rs"
  verifying_test: "cargo xtask spec-trace check 4 (xtask/src/spec_trace.rs:680-711) resolving the name against crates/happenstance-testkit/src/suite.rs, with the rule's actual definition line in suite.rs cited as evidence"

- id: AC-007
  criterion: "**GIVEN** the repository owner reviewing this diff for the one thing `CLAUDE.md` forbids without an ADR, **WHEN** they read the hunks, **THEN** ES-38 (`spec/SPECIFICATION.md:4299-4323`) is byte-identical — `(new)` retained, reading (b) from the discover gate — no other `[FROZEN]` clause is touched, and no rule is retired or renamed anywhere in the tree"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:4299-4323 (ES-38, untouched) and its generated §7.2 cell at :8620; crates/happenstance-testkit/src/registry.rs as the registry the retired-rules lint reads"
  verifying_test: "git diff -U0 spec/SPECIFICATION.md showing no hunk inside :4299-4323; cargo xtask lints' retired-rules step (xtask/src/main.rs:342); the §7.2 ES-38 cell at spec/SPECIFICATION.md:8620 losing its dagger"

- id: AC-008
  criterion: "**GIVEN** the next contributor moving an unrelated marker three months from now, **WHEN** they run `cargo xtask spec-trace` on a tree containing this commit, **THEN** it is green on the first run: §7.1/§7.2 between `spec/SPECIFICATION.md:8507` and `:8753` are exactly what `cargo xtask spec-trace --write` produced from this commit's clause bodies, committed verbatim with no hand edit inside the markers and no trimming of rows the author did not expect, and the marker moves, the regenerated region and the census reconciliation are **one commit**"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:8507-8753 — the region between `<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->` and `<!-- END GENERATED -->`, rendered by xtask/src/spec_trace.rs"
  verifying_test: "bare cargo xtask spec-trace check 9 (xtask/src/spec_trace.rs:741-742); idempotence by re-running cargo xtask spec-trace --write (xtask/src/main.rs:671-674) post-commit for an empty git diff; git log --oneline -1 --stat showing all three regions in one commit — rejects MarkerFlip (discover.md:47)"

- id: AC-009
  criterion: "**GIVEN** the release auditor running the initiative's DoD 12 audit (`.bklg/from-contract-to-published-library/initiative.md:393-395`) — every clause's maturity reported, no provisional clause with an empty falsifier, and the report's count matching the document's own stated figure — **WHEN** they run the gate on this tree, **THEN** §1.3's sentence at `spec/SPECIFICATION.md:219-222` was recomputed **by a person** and agrees with the checker on all four maturity counts and on its own subtraction, holding total = 200, normative = 198 and `[NON-NORMATIVE]` = 2 invariant while `[DEFERRED]` goes 10 → 8 and the FROZEN/PROVISIONAL/DEFERRED triple still sums to 198; and `cargo xtask spec-trace`, `cargo xtask affected --base main` and `cargo xtask ci --fast` are all green on the result"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:219-222 — §1.3's hand census, the one figure spec-trace deliberately checks rather than writes (xtask/src/spec_trace.rs:39-57); gates wired at .redkiln/config.yaml:40 and :55"
  verifying_test: "cargo xtask spec-trace check 8 (xtask/src/spec_trace.rs:459-508) over the stated total, each of the four counts and the sentence's subtraction; cargo xtask affected --base main (.redkiln/config.yaml:40); cargo xtask ci --fast (.redkiln/config.yaml:55); git diff --name-only listing nothing outside the PR-boundary globs"
```
