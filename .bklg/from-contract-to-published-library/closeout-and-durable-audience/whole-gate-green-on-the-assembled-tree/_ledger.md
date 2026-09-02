---
item: "HS-S0126"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The whole gate green on the assembled tree, with its SHA and every skip accounted for

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Note for the implementer, per spec *Clarifications resolved during spec* item 4: **AC-001 is
satisfied by an observed exit code, not by a zero.** A red gate is routed as a finding
(`project.md` DR-12, AC-013) and does not block this row; a repair, a `--fast` substitution or an
unaccounted skip does.

```yaml
- id: AC-001
  criterion: "GIVEN the residue-free checkout `clean-checkout-harness` produced, WHEN the implementer runs `cargo xtask ci` with no flags from its root and lets it finish, THEN the run reaches its own end (`REQUIRED` then `OPTIONAL`, xtask/src/main.rs:828-831) and the record states the observed exit code — zero, or non-zero naming the first failing step from `run_steps`'s `{step.name} failed with {status}` bail — AND the string `--fast: ` appears nowhere in the captured transcript, so the reader can tell the release bar from the sibling bar without trusting a claim."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the Gate run section"
  verifying_test: "cargo xtask ci (verify.e2e, .redkiln/config.yaml:60) run whole from the clean checkout; transcript recorded at .bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/gate-run.md section 'Command and exit', with a grep of the capture for `--fast: ` returning nothing"

- id: AC-002
  criterion: "GIVEN the evaluator wants to re-derive the run rather than accept it, WHEN they open the record, THEN it names the 40-character commit SHA the gate ran against and cites the slice-mate's residue evidence for that same SHA by path, SO THAT they can check out that commit themselves and find the tree the gate saw — not 'a recent tree', not 'the closeout branch'."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the Gate run section"
  verifying_test: "gate-run.md section 'Tree under test' carries `git rev-parse HEAD` and `git status --porcelain` captured immediately before the run and cites clean-checkout-harness's residue artefact by path; SHA equality check between .bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/gate-run.md and .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"

- id: AC-003
  criterion: "GIVEN the evaluator wants to know what 'the gate was green' actually covered, WHEN they read the record's step table, THEN every one of the gate's 24 steps — the 20 in `REQUIRED` (xtask/src/main.rs:105-534) and the 4 in `OPTIONAL` (:535-637) — appears as its own row carrying its verbatim `Step::name` and one of `passed` / `failed` / `skipped`, SO THAT a step added, renamed or dropped upstream shows as a mismatch instead of blending into a summary sentence."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the Gate run section"
  verifying_test: "Row-for-row comparison of gate-run.md section 'Steps' against the `name:` fields in xtask/src/main.rs:105 (REQUIRED) and :535 (OPTIONAL) — 24 rows, no paraphrase, no roll-up — cross-checked against every `=== <name> ===` banner (xtask/src/main.rs:864) in the captured transcript"

- id: AC-004
  criterion: "GIVEN BR-12's promise that the `!Send` port flavour is 'exercised end to end by everything that ships', WHEN the evaluator looks for it at closeout rather than at cloudflare-durable-object-store's own merge, THEN the record shows `wasm32 build of the contract crate`, `wasm32 check of the conformance harnesses`, `wasm32 build of the Cloudflare adapter` and `wasm32 build of the Neon adapter` each named individually and each `passed` — a count such as '4/4 wasm32 green' fails this criterion."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the Gate run section"
  verifying_test: "The four named rows in gate-run.md section 'Steps', matched literally against `wasm32_steps` at xtask/src/main.rs:769-793, plus their four `=== … ===` banners in the captured transcript; these steps carry `probe: None` so any `skipped` or absent row is a failure (.kb/decisions/0001-async-port-flavours.md)"

- id: AC-005
  criterion: "GIVEN the evaluator cannot distinguish 'green' from 'green with the advisory scan switched off' unless the record tells them, WHEN any step prints `skipped: <probe> did not succeed` (xtask/src/main.rs:876) or the run exits non-zero, THEN the record quotes that line verbatim and names the absent tool it identifies (`cargo hack --version`, `cargo deny --version`, `cargo +nightly --version`), AND — because CLAUDE.md Commands records that cargo-hack and cargo-deny both resolve on this machine — a skip of either, or any failing step, is written into the record as a finding with a destination for findings-disposition-register (HS-S0134) rather than repaired, with the diff confined to the two globs in PR boundary."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the Gate run section"
  verifying_test: "gate-run.md section 'Skips and findings' — one verbatim quoted `skipped:` line per skipped step each mapped to its probe, or the explicit sentence 'no step printed skipped'; plus the boundary proof `git diff --stat <base>..HEAD` touching only .bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/** and .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md, with zero paths under crates/, xtask/, spec/ or .github/ (checked by `redkiln verify --grain story`)"

- id: AC-006
  criterion: "GIVEN the evaluator arrives at `_closeout-record.md` and nowhere else, WHEN they read its Gate run section, THEN they reach the SHA, the exit code, the four `wasm32` outcomes and the skip accounting from that section — by reading it or by following its one cited link to `gate-run.md` — AND the section states the width the green was proven at (host OS, the `rust-toolchain.toml` pin that ran, and that the MSRV floor is checked by CI's `msrv` job, not by this run), SO THAT nothing in it reads as a claim about a consumer's compiler."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md — the Gate run section, in the shape clean-checkout-harness defined"
  verifying_test: "`rg -n \"gate-run.md\" .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` returns a hit and the cited path resolves; `rg` over the same section returns the host-OS, rust-toolchain.toml-pin and MSRV-is-CI's-msrv-job sentences (CLAUDE.md Commands; .kb/decisions/0029-msrv-raised-to-1-97-1.md; .github/workflows/ci.yml:241)"
```
