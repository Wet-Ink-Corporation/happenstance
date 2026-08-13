---
item: "HS-S0125"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The clean-checkout seam and the closeout record it feeds

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from `spec.md`. This project owns no code
(`project.md` *Out of scope*), so every `verifying_test` below is a **recorded command with its
captured output committed at a real path** rather than a test-framework id — that is the tier the
`testing` brief assigns (`_decomposition.md` *Test mix*: static and process are load-bearing here).
And AC-003's second lock-hash reading and AC-006's post-run check are taken **after** the
slice-mate's `cargo xtask ci` run inside the same slice; do not flip either row on the strength of
the contract being recorded rather than observed (`spec.md` *Clarifications resolved during spec*,
item 6).

`_evidence/` below abbreviates
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/`.

```yaml
- id: AC-001
  criterion: "The reader can name the exact tree the closeout gate ran on, and reproduce it. GIVEN an auditor who has never had access to this machine and cannot re-run the suite, WHEN they open the Harness row of `_closeout-record.md`, THEN they find the full isolation command — `git clone https://github.com/Wet-Ink-Corporation/happenstance.git <dir>` into a directory outside `D:/repos/happenstance` and every worktree of it, followed by `git -C <dir> checkout --detach <SHA>` — with the resolved SHA and its commit date, AND running those two commands themselves yields a tree whose `git rev-parse HEAD` equals the recorded SHA. A `git worktree add` in place of the clone fails this criterion even though it produces a checkout, because the tree it produces shares the primary `.git` (B-1)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/clone-transcript.txt (git clone + git checkout --detach, captured whole) and _evidence/head-sha.txt (git rev-parse HEAD, git log -1 --format=%cI, git branch -r --contains <SHA>), reviewed against the Harness row of _closeout-record.md"

- id: AC-002
  criterion: "The reader can see for themselves that nothing was left in the tree. GIVEN the auditor asking \"how do you know a scratch file or a stale `target/` was not part of what you measured\", WHEN they open the residue evidence, THEN they find `git status --porcelain --ignored` producing empty output in the clone, captured to a file, and taken before anything was built — ordered ahead of the gate transcript, not reconstructed after it. The `--ignored` flag is part of the criterion: the plain form passes with a populated `target/`, which is the stale-artefact half of DR-1 (`project.md:129-131`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/residue-before.txt (`git status --porcelain --ignored` in the clone, zero bytes), with its command line recorded in _evidence/clone-transcript.txt after the detach and before any cargo invocation"

- id: AC-003
  criterion: "The reader can believe the gate measured the real dependency graph. GIVEN the auditor asking \"was a local copy standing in for a published crate\", WHEN they open the override scan, THEN they find zero hits for every form DR-1 forbids — a `[patch…]` section in any `Cargo.toml`, a `paths = [...]` or `[source.*] replace-with` in any cargo config, a vendored source directory — AND they find the workspace's own `version`+`path` dependencies at `Cargo.toml:24-26` listed explicitly as non-hits with the reason, so the next reader does not mistake ordinary workspace practice for the thing DR-1 rules out. `Cargo.lock`'s blob hash is recorded before the run so the post-run comparison has a recorded value to compare against."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/override-scan.txt (four scans, zero hits, plus the recorded Cargo.toml:24-26 non-hit explanation) and _evidence/lock-hash.txt (`git hash-object Cargo.lock` before, re-read after the slice's gate run), cross-checked against xtask/src/main.rs:52-54"

- id: AC-004
  criterion: "The reader can tell what the gate did not do, and why. GIVEN the auditor asking \"twenty-four steps, four of them optional — which ran\", WHEN they read the harness row, THEN they find a probe manifest captured before the run recording the exit status of `cargo hack --version`, `cargo deny --version` and `cargo +nightly --version` under `RUSTUP_AUTO_INSTALL=0` — the same spelling `is_available` uses (`xtask/src/main.rs:894-908`) — AND the transcript path the run must tee into, fixed here, so every `skipped:` line the gate prints to stdout and records nowhere else (`xtask/src/main.rs:873-878`) survives the session. A skip is then attributable to a named absent tool rather than merely observed, which is what AC-001 of `project.md:192-195` asks for."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/probe-manifest.txt (three probes with exit status under RUSTUP_AUTO_INSTALL=0, plus `rustc -Vv`), with _evidence/gate-transcript.txt declared as the tee target in the Harness row of _closeout-record.md before the slice-mate runs"

- id: AC-005
  criterion: "The reader has one place to read all of it together. GIVEN the auditor who must not be sent to fourteen ledger entries in fourteen places — the failure mode `_storymap.md:24-30` writes AC-003's word \"set\" against — WHEN they open `_closeout-record.md`, THEN it exists with all nine sections in the fixed order of B-8, every owed row present as a visible placeholder naming the AC and the story that owes it, a stated column contract requiring artefact path plus the subject string a reader should find there plus the SHA it was observed on, a preamble stating that a sibling's `_ledger.md` may be the pointer and never the evidence, and a coverage line giving rows filled against rows owed."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: "Structural review of .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md against spec.md B-8/B-9/B-10: nine sections in order, exactly fourteen DoD rows, a destination column and no fix column in Findings, the coverage line reading 1 / N, contract grounded in .kb/playbooks/verify-the-referent-and-report-coverage.md"

- id: AC-006
  criterion: "The reader can trust that no later claim was smuggled in on this observation. GIVEN the auditor reading a row about a tree that moved after this harness ran — slices 4 and 5 commit into `.kb/` by construction — WHEN they check its provenance, THEN every row carries the SHA it was observed on rather than the document carrying one SHA, the record's preamble states the rule that a story needing a later tree re-runs this procedure at its own SHA and adds a harness row rather than citing this one, and the checkout's read-only property is evidenced by a post-run tracked-file check whose non-empty result is declared a finding to route rather than something to repair."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/_evidence/residue-after.txt (`git status --porcelain`, tracked files only, empty, captured in the same slice after the gate run), read together with the Harness table's per-row SHA column and the re-run preamble rule in _closeout-record.md; xtask/src/main.rs:315-327 is why a dirty tracked file is a real signal"
```
