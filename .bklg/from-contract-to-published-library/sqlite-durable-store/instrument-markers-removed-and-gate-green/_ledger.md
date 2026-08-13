---
item: HS-S0045
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The last todo!() and the scoped allow go together, and the gate is green

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows in this ledger are discharged by a **named review item** rather than by a command, and that
is stated rather than hidden: AC-003 (no lint in this repository reads documentation for truth) and
the "what `--fast` did not run" half of AC-005. Their evidence must name the reviewer's reading and
the base-revision diff, not a green transcript that proves something else.

```yaml
- id: AC-001
  criterion: "GIVEN the adapter author has taken all four suites green against a real file and comes back to the crate root to learn whether they are finished, WHEN they open crates/happenstance-sqlite/src/lib.rs and read `git show` for the commit that closed the work, THEN the final todo!() on a SQLite path and the #![allow(clippy::todo)] at :76-80 — its four-line justification included — have left in that one commit, clippy::todo covers the crate with no exception, and no other crate's scoped allow moved with them."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/lib.rs"
  verifying_test: "cargo xtask ci --fast clippy step (xtask/src/main.rs:105-130, `cargo clippy --workspace --all-targets --all-features -- -D warnings`), plus `rg -n 'todo!' crates/happenstance-sqlite/src` and `rg -n 'allow\\(clippy::todo\\)' crates/happenstance-sqlite` returning nothing, plus one-commit provenance recorded via `redkiln record-links --sha`"

- id: AC-002
  criterion: "GIVEN the adapter author's fear is a port that quietly assumed something their storage cannot provide, WHEN every SQLite path that used to be todo!() is executed by the conformance, model, concurrency and projection targets, THEN each one runs real SQL — no unimplemented!(), unreachable!(), panic!(\"not implemented\") or fabricated Err variant stands in for work not done — and where head and contains_event_id are written here they obey the constraints already stated on them: head re-reads max(position) from the database on every call and is never a cached field an append updates, with NULL mapping to None; contains_event_id reads the UNIQUE(origin_store, origin_position) pair."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/lib.rs"
  verifying_test: "cargo test -p happenstance-sqlite (crates/happenstance-sqlite/tests/conformance.rs, tests/concurrency.rs, tests/projection.rs, tests/shapes.rs) green inside cargo xtask ci --fast; head's freshness proven by two_handles_observe_each_others_appends (crates/happenstance-testkit/src/contract.rs); `rg -n 'unimplemented!|unreachable!|not implemented' crates/happenstance-sqlite/src` empty"

- id: AC-003
  criterion: "GIVEN the evaluator has one sitting and a rendered docs page in which to decide whether \"DCB-compliant\" and \"storage-agnostic\" are real claims, WHEN they land on this crate's front page — in the tree or from cargo doc — THEN it describes an adapter that has passed the conformance suite against a real file, not the skeleton it used to be: the Status: an instrument, not yet an adapter banner and the \"every operation that touches SQL is todo!()\" paragraph are gone, the publish = false sentence is replaced by the true and different statement that publication is publication-and-positioning's decision, Open decisions is closed against ADR-0022 or narrowed to what it genuinely left open with the open-question atom cited, and no other status marker in the tree contradicts the front page — event_store.rs:3, projection_store.rs:3, tests/shapes.rs:3 and CLAUDE.md:33's skeleton row all corrected — while the three sections that carry compiled results survive."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/lib.rs"
  verifying_test: "Named review item — the reviewer reads the rewritten crates/happenstance-sqlite/src/lib.rs against its base revision and reads event_store.rs's `# Intended schema` block object by object against migration 1; machine half is `rg -n 'instrument, not yet an adapter|bodies unimplemented|publish = false until|skeleton' crates/happenstance-sqlite CLAUDE.md` clean outside CLAUDE.md:66-73's definition paragraph and the other five skeleton rows; the docs step of cargo xtask ci --fast (xtask/src/main.rs:290-315) proves link resolution only and MUST NOT be cited as truth evidence"

- id: AC-004
  criterion: "GIVEN the specification is the artefact that tells both personas which clause each behaviour discharges, WHEN the doc rewrite of AC-003 shifts every line below a rewritten header and cargo xtask spec-trace runs, THEN every file:line citation into this crate still resolves and still anchors within ANCHOR_SLACK = 12 lines of its derived subject — repaired by moving citation line numbers, and by nothing else: no clause text, no maturity marker, no §7.3–§7.6 judgement, and no hand-edit of the generated §7.1/§7.2 regions."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/lib.rs"
  verifying_test: "cargo xtask spec-trace (REQUIRED step, xtask/src/main.rs:315-341; also reachability_static, .redkiln/config.yaml:48) green, with `git diff spec/SPECIFICATION.md` read to confirm only citation digits changed"

- id: AC-005
  criterion: "GIVEN the project's bar is cargo xtask ci --fast and the adapter author must be able to tell a partial bar from a full one, WHEN the gate is run on the tree exactly as this PR leaves it, THEN every REQUIRED step passes end to end — fmt, clippy at -D warnings, cargo test --workspace --all-features, the proof-artefact step, all four wasm32 steps, docs, spec-trace, the five file-reading lints, both constitution checks, the no-default-features doc build and package-check — and the ledger records, by name, what --fast did not run, so a green --fast is never presented as a green gate."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/lib.rs"
  verifying_test: "cargo xtask ci --fast (integration_scoped, .redkiln/config.yaml:55) green whole, its closing line quoted from xtask/src/main.rs:853-861; the evidence must additionally name the four dropped OPTIONAL steps (xtask/src/main.rs:535-640) and the two bars outside it (the MSRV job, cargo xtask ci whole at .redkiln/config.yaml:60), and state that DR-08 is re-checked not discharged (_storymap.md:150-152)"

- id: AC-006
  criterion: "GIVEN publishing this crate is a decision publication-and-positioning owns and RUNBOOK.md:4230's exit checkbox is stale, WHEN the adapter author diffs this PR looking for scope that leaked, THEN publish = false in crates/happenstance-sqlite/Cargo.toml, PUBLISHABLE in xtask/src/package.rs:86 and the crate description are byte-identical to base — the crate becomes describable as an adapter here and publishable somewhere else."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/src/lib.rs"
  verifying_test: "`git diff crates/happenstance-sqlite/Cargo.toml xtask/src/package.rs` showing no change to publish/description/PUBLISHABLE (an empty diff for package.rs entirely), plus package-check inside cargo xtask ci --fast (xtask/src/package.rs:86, :163-180) still naming exactly three publishable crates; the ledger note records RUNBOOK.md:4230 as stale rather than skipped"
```
