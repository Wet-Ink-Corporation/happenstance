---
item: HS-S0090
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Every deferral is re-read at publish and says why, dated

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Every `mount_point` is `xtask/src/main.rs` — the `REQUIRED` step list (`:105`ff) and the module doc
(`:8-24`), per the spec's Integration contract. This story registers no `Step` of its own; its checks
run inside the mandatory clause-audit step `clause-maturity-audit` adds to that list, which is why the
mount is the list rather than a new file. `xtask/src/clause_audit.rs` is that slice-mate's module: if it
lands under a different filename, every `verifying_test` path below follows the real name, and the
implementer records the substitution in the evidence field.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator who has read the published census sentence and been told ten clauses are deferred, WHEN they open any one of WF-1, ES-39, PS-33, SY-14, SY-18, SY-27, SY-28, SY-32, CF-14, CF-27 in spec/SPECIFICATION.md, THEN they read — at that clause, without following a link or opening a second file — a single `Re-read: <version>, <YYYY-MM-DD> — <reason>` line whose reason is written in the caller's register (what depending on 0.2.0 costs them while the question is open), distinct from the marker's experiment-and-phase, and all ten clauses carry one"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff) — checks run inside the mandatory clause-audit step; records live at the ten clauses in spec/SPECIFICATION.md"
  verifying_test: "xtask/src/clause_audit.rs::tests::rejects_deferred_clause_with_no_reread"

- id: AC-002
  criterion: "GIVEN a maintainer who adds the record to a clause that already has `Rule:` and `Cases:` lines, WHEN spec-trace parses the document, THEN field_line(body, \"Re-read\") returns the record and the `Re-read:` line terminates the field above it instead of being absorbed as continuation — so rules_of never sees the reason's backticked identifiers, check 4 reports no non-existent rule, and no `Rule:`/`Cases:` value gains a character"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff) — via xtask/src/spec_trace.rs's field_head allowlist (:1541-1555)"
  verifying_test: "xtask/src/clause_audit.rs::tests::reread_line_terminates_the_field_above_it"

- id: AC-003
  criterion: "GIVEN the evaluator was told \"10 deferred\" by the published census, WHEN the gate runs on the tree that publishes that sentence, THEN three sets are proven equal — the clauses carrying `Re-read:`, the clauses spec-trace parses as DEFERRED, and §1.3's hand-computed 10 (spec/SPECIFICATION.md:219-222) — so a record on a non-deferred clause, a deferred clause without one, and an eleventh deferral added later each fail by name"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff)"
  verifying_test: "xtask/src/clause_audit.rs::tests::rejects_reread_on_a_non_deferred_clause and ::rejects_an_eleventh_deferral_that_was_never_reread"

- id: AC-004
  criterion: "GIVEN the maintainer at the next release, who would otherwise inherit ten reasons written for this one, WHEN workspace.package.version moves past 0.2.0 (Cargo.toml:5-6), THEN all ten records are stale by construction and the gate fails until each is re-read and re-stamped — the version comparison is exact, the date is checked for YYYY-MM-DD shape only, and the audit reads no clock, so two runs on one commit can never disagree"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff)"
  verifying_test: "xtask/src/clause_audit.rs::tests::rejects_a_record_stamped_at_a_previous_version, ::rejects_a_malformed_date, ::verdict_is_a_function_of_the_tree"

- id: AC-005
  criterion: "GIVEN a maintainer under release pressure, for whom the cheapest fake re-read is pasting the marker's experiment across into the new field, WHEN the record's prose — normalised (whitespace collapsed, backticks and punctuation stripped, case-folded) — equals, or is contained in, the marker text falsifier_of returns for that same clause, or falls below the stated minimum length, THEN the gate fails that clause by name rather than counting it re-read"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff) — compared against xtask/src/spec_trace.rs's falsifier_of (:1475-1489)"
  verifying_test: "xtask/src/clause_audit.rs::tests::rejects_a_reason_that_restates_the_marker and ::rejects_a_reason_below_the_minimum_length"

- id: AC-006
  criterion: "GIVEN the maintainer running the gate, WHEN the clause-audit step runs, THEN it runs mandatorily — inside REQUIRED in xtask/src/main.rs:105ff, never behind a probe: that would convert found a problem into skipped (:89-102) — it reports rather than merely passes, printing one line per clause (id, date, the reason's first line: ten lines, no more) into the dated report the slice commits, every failure names the clause id, the file and the field so the reader reaches the offending spot from the message alone, and xtask/src/main.rs:8-24 gains, in this same change, the sentence stating the gate now proves every deferral was re-read at the published version, alongside the audit's own module doc stating the axis it does not check"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff, and the module doc at :8-24)"
  verifying_test: "xtask/src/clause_audit.rs::tests::failure_message_names_the_clause_and_the_field"

- id: AC-007
  criterion: "GIVEN anyone who cites this specification — spec-trace's own line-range cross-references, CLAUDE.md, the ADR corpus — WHEN this change lands, THEN nothing normative has moved: the ten clauses' sentences, [DEFERRED] markers, `Rule:`, `Cases:` and `Rejects:` fields are byte-stable, no [FROZEN] or [PROVISIONAL] clause appears in the diff, no clause is renumbered or reordered, `cargo run -p xtask -- spec-trace --write` leaves the tree unchanged (the generated §7.1/§7.2 region opened at :8507 does not move a cell), and `cargo xtask spec-trace` stays green so every existing citation still resolves"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff) — the spec-trace step; subject is spec/SPECIFICATION.md"
  verifying_test: "cargo run -p xtask -- spec-trace --write && git diff --exit-code spec/SPECIFICATION.md (run as part of cargo xtask affected --base main)"

- id: AC-008
  criterion: "GIVEN the maintainer must be able to believe a green gate, WHEN the parser silently stops recognising the field — the failure mode a compiled-once unit test cannot see, because it proves the parser worked when cargo test ran and not on the tree being published — THEN a DEFERRAL_PROBE fixture clause, parsed on every gate run in the RETIRES_PROBE/WIRE_PROBE shape (xtask/src/spec_trace.rs:849-868, :954-967, :1802-1848), fails the gate, and the audit additionally carries #[cfg(test)] mod tests in the xtask/src/package.rs:408-457 shape where every test names in a doc comment the wrong tree it rejects"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED step list, :105ff) — probe evaluated inside the spec-trace step"
  verifying_test: "DEFERRAL_PROBE in xtask/src/spec_trace.rs, evaluated by cargo run -p xtask -- spec-trace; plus cargo test -p xtask over xtask/src/clause_audit.rs::tests"
```
