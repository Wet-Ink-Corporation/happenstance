---
item: "HS-S0060"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Claim both crate names on crates.io

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. **Evidence for the off-tree rows is a transcription, not a link** —
crates.io is not readable by any gate in this repository, so AC-005 and AC-006 are evidenced by the
API response values written into this ledger and into the phase-10 session-log entry, and AC-007 is
the row that makes them readable in-tree afterwards. And **no row's evidence may contain registry
credential material** (AC-010, NF-002): cite the command, never its authenticated output.

```yaml
- id: AC-001
  criterion: "GIVEN the maintainer is claiming a name for the first time in weeks, and a procedure run that rarely from memory drifts, WHEN the placeholder for each of happenstance-postgres and happenstance-neon is produced, THEN it is produced by `cargo xtask reserve <name>` into `target/reserve/<name>/` and no part of the published artefact was hand-assembled — the procedure is the committed command, not recollection."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/reserve.rs — RESERVABLE rows at :98-109 and run() at :137-191, invoked as `cargo xtask reserve <name>`; artefact under target/reserve/<name>/"
  verifying_test: "`cargo xtask reserve` (no argument) plus `cargo xtask reserve happenstance-postgres` and `cargo xtask reserve happenstance-neon`; generated tree matched against xtask/src/reserve.rs:150-169"
- id: AC-002
  criterion: "GIVEN defect D11 once shipped licence metadata with no licence text, permanently, WHEN each placeholder is packaged, THEN the .crate that would be uploaded contains LICENSE-MIT, LICENSE-APACHE and README.md inside the crate directory, so anyone who downloads it gets the licences the manifest promises."
  satisfied: false
  evidence: ""
  mount_point: "The packaged artefact for each name: target/reserve/<name>/, whose licence copies are written by xtask/src/reserve.rs:158-165"
  verifying_test: "`cargo package --manifest-path target/reserve/happenstance-postgres/Cargo.toml --list` and the same for happenstance-neon; both listings show LICENSE-MIT, LICENSE-APACHE, README.md"
- id: AC-003
  criterion: "GIVEN the evaluator of Decide in one sitting, whose entire look at this project may be one registry page, WHEN they open either crates.io page, THEN the rendered README leads with the sentence 'This 0.0.0 is a placeholder. It contains no functionality.', names what the crate will be, links spec/SPECIFICATION.md and RUNBOOK.md, and states the first functional release will be 0.1.0-alpha.1 — the generator's text, unedited."
  satisfied: false
  evidence: ""
  mount_point: "The rendered crates.io page for each name, whose body is generated verbatim by xtask/src/reserve.rs:242-269 (README) and :271-287 (lib.rs STATUS const)"
  verifying_test: "Byte comparison of each published README against readme()'s output (xtask/src/reserve.rs:242-269), plus a recorded human look at both rendered pages"
- id: AC-004
  criterion: "GIVEN a publish that cannot be undone and a 0.0.0 that is a permanent first impression, WHEN either name is claimed, THEN `cargo publish --dry-run` has been run against that exact manifest and exited zero before the real publish, and neither publish used --no-verify."
  satisfied: false
  evidence: ""
  mount_point: "The command sequence xtask/src/reserve.rs:174-186 prints: dry run first, irreversible publish second"
  verifying_test: "`cargo publish --manifest-path target/reserve/<name>/Cargo.toml --dry-run` for both names, exit status and ordering recorded in this ledger ahead of the AC-005 evidence"
- id: AC-005
  criterion: "GIVEN the stranger of initiative DoD 9 who will one day run `cargo add happenstance-postgres` against a scratch project, WHEN this story lands, THEN both happenstance-postgres and happenstance-neon resolve on crates.io at 0.0.0 under this project's ownership, and no other version of either exists."
  satisfied: false
  evidence: ""
  mount_point: "crates.io, recorded in-tree at RUNBOOK.md — the phase-10 work item (RUNBOOK.md:4346-4348) and the phase-10 Session log block"
  verifying_test: "`cargo info happenstance-postgres` and `cargo info happenstance-neon`, plus GET https://crates.io/api/v1/crates/<name> showing max_version 0.0.0 and exactly one version each"
- id: AC-006
  criterion: "GIVEN phase 0 verified six registry facts for the first three names rather than assuming them, WHEN both names are claimed, THEN the same six are checked through the crates.io API for each crate — description, repository, MIT OR Apache-2.0, README present, rust-version, not yanked — and the values written down are the ones the API returned, not the ones that were intended."
  satisfied: false
  evidence: ""
  mount_point: "The phase-10 Session log block in RUNBOOK.md, in the form phase 0 used at RUNBOOK.md:1076-1083"
  verifying_test: "GET https://crates.io/api/v1/crates/<name> and its version endpoint for both names; the six field values transcribed verbatim into this ledger and the session-log entry"
- id: AC-007
  criterion: "GIVEN deskeleton-and-package-readiness and HS-P0016's publication audit read this repository rather than the registry, WHEN the claim is complete, THEN RUNBOOK.md's phase-10 work-item checkbox is ticked and a dated phase-10 session-log entry names both crates, the version, and the six verified facts — so the off-tree fact is readable in-tree."
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md:4346-4348 (the phase-10 work item) and the phase-10 Session log block that closes the phase section"
  verifying_test: "`git diff main -- RUNBOOK.md` showing the ticked work item and the new session-log entry, plus `cargo xtask affected --base main` green (the five file-reading lints and spec-trace read the file)"
- id: AC-008
  criterion: "GIVEN the generated manifest hardcodes edition 2021 and rust-version 1.85 while the workspace floor is 1.97.1, and the standing instruction is to weigh the floor rather than obey it, WHEN the placeholders are published, THEN the published manifests match a decision that is written down — either published as generated for parity with the three placeholders already on the registry, or the generator was changed first and here is the diff — and neither outcome is reached silently."
  satisfied: false
  evidence: ""
  mount_point: "This ledger row plus the phase-10 Session log entry in RUNBOOK.md; if the generator changes, xtask/src/reserve.rs:214-240"
  verifying_test: "The recorded decision and its reason in this row; where the generator changed, the diff to xtask/src/reserve.rs:214-240 with `cargo xtask affected --base main` green; the rust_version value captured under AC-006 matching the recorded decision"
- id: AC-009
  criterion: "GIVEN the adapter author whose next story is the real de-skeleton, and a package-check that fails in both directions, WHEN this story merges, THEN publish = false is still on both crate manifests, PUBLISHABLE still names exactly three crates, and no file under crates/ changed — a name claim did not quietly half-do the packaging story."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/Cargo.toml:12 and crates/happenstance-neon/Cargo.toml:12, reconciled against xtask/src/package.rs:86 and :172-212"
  verifying_test: "`cargo xtask package-check` (run inside `cargo xtask affected --base main`) and an empty `git diff --stat main -- crates/ xtask/src/package.rs`"
- id: AC-010
  criterion: "GIVEN a publish requires a live registry token and a leaked one costs a revocation and a rewritten history, WHEN this story's tree, its commits and its ledger are inspected, THEN no token or fragment of one appears in any of them, and .gitignore's .cargo/credentials entries are unchanged."
  satisfied: false
  evidence: ""
  mount_point: ".gitignore:33-34 (the .cargo/credentials entries) and this story's whole diff, commit messages and ledger"
  verifying_test: "`git diff main` reviewed for credential material, and a grep over the story's changed files for credentials / cargo login / CARGO_REGISTRY_TOKEN returning only documentation prose"
- id: AC-011
  criterion: "GIVEN a story whose deliverable is mostly a record, and a package-shaped gate that would compile nothing and call it green, WHEN the story grain runs, THEN `cargo xtask affected --base main` is green including the five file-reading lints and `cargo xtask spec-trace`."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml, verify.affected_gate — the story grain redkiln runs whether or not anyone types it"
  verifying_test: "`cargo xtask affected --base main`, output recorded in this ledger"
```
