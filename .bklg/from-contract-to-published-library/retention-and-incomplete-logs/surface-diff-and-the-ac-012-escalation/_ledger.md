---
item: "HS-S0124"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Acceptance ledger — The 0.2.0 surface diff, and the escalation that is a finding not a change

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN a consumer who already has happenstance-core = \"0.2\", happenstance = \"0.2\" or happenstance-testkit = \"0.2\" in a manifest and whose build must not break by surprise (initiative :338-340), WHEN this project's assembled tree is compared against what that consumer can already cargo add, THEN a dated report exists at spec/audits/surface-diff-<YYYY-MM-DD>.md, produced by a real `cargo xtask surface-diff --run --date <YYYY-MM-DD>` carrying --baseline-version per crate and never --baseline-rev, covering exactly the three crates in xtask/src/package.rs:86's PUBLISHABLE — three crates, three independent baselines, because happenstance-testkit moves on its own version key (CF-32, [FROZEN]) — and committed verbatim as the tool wrote it, never hand-edited, with a superseding capture at a new date being the only correction path so the earlier report survives as the record of what was true then."
  satisfied: false
  evidence: ""
  mount_point: "spec/audits/surface-diff-<YYYY-MM-DD>.md — the artefact directory the mandatory surface-diff check step reads, mounted in xtask/src/main.rs's REQUIRED list (:105)"
  verifying_test: "cargo xtask surface-diff (check mode) over the committed report, via xtask/src/main.rs:105; plus review of the three per-crate --run transcripts in spec/audits/surface-diff-<YYYY-MM-DD>.md against xtask/src/package.rs:86"
- id: AC-002
  criterion: "GIVEN the maintainer who must not have to remember a finding, WHEN any subsequent story in this project or the next runs its own gate, THEN the finding is read in place, inside the gate they already run — `cargo xtask ci --fast` (.redkiln/config.yaml:55) and `cargo xtask affected --base main` (:40) both consume this report through the already-mounted surface-diff check step and go red if it is stale, malformed or crate-set-mismatched — and this story adds no wiring edit to do it, because the step is mounted by HS-S0091 and an unmounted step is AC-008's halt, not a licence to mount one here."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:105 (REQUIRED list) and xtask/src/affected.rs's unconditional file-reading block — read-only; this story adds no wiring edit"
  verifying_test: "cargo xtask ci --fast (.redkiln/config.yaml:55) and cargo xtask affected --base main (.redkiln/config.yaml:40), both green; git diff empty over xtask/src/main.rs and xtask/src/affected.rs"
- id: AC-003
  criterion: "GIVEN the reviewer of this pull request, who is entitled to know which question each green check answered, WHEN the record is read, THEN both comparisons are present and distinguished by what each is blind to: the branch-point cargo-semver-checks job (.github/workflows/ci.yml:279-316, baseline-rev = base.sha) cannot see \"a break merged two pull requests ago\" (:305-307), and the registry capture cannot see anything introduced after its captured-at — and neither is presented as, relabelled as, or substituted for the other."
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml:279-316 — the PR-grain semver job, run and its output captured into this story's folder, never edited"
  verifying_test: "The semver job at .github/workflows/ci.yml:279-316 green on this PR, output captured; content review of the finding record against CONTRIBUTING.md:291-301"
- id: AC-004
  criterion: "GIVEN the adapter author whose conformance suite is green today and who will bump happenstance-testkit next week, WHEN they read this project's surface record, THEN they read two sentences in order — (1) no breaking change to happenstance-core, happenstance or happenstance-testkit against the 0.2.0 registry baseline; (2) adding a conformance rule is semver-MINOR and can still turn a passing adapter's CI red, quoted from the crate's own manifest, which is why that crate carries an independent version key — so the record is materially honest rather than only technically true, and a record that stops at sentence (1) fails this AC even though it is not false."
  satisfied: false
  evidence: ""
  mount_point: "The AC-011 finding record in .bklg/from-contract-to-published-library/retention-and-incomplete-logs/surface-diff-and-the-ac-012-escalation/, read by closeout-and-durable-audience's BR-15 audit"
  verifying_test: "Content review of the finding record against the verbatim sentence at crates/happenstance-testkit/Cargo.toml:4-13 and the both-sentences requirement at .bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:77-88"
- id: AC-005
  criterion: "GIVEN the evaluator and the closeout auditor, who must be able to re-derive the verdict rather than take it on trust, WHEN they read the record, THEN every additive item this project's merged diff added is enumerated by name and by class — each new pub fn rule body re-exported through crates/happenstance-testkit/src/lib.rs:188, each for_each_event_store_rule! registration (crates/happenstance-testkit/src/registry.rs:94), any defaulted Fixture associated const and defaulted method (DA-4) with the note that a required item there would have been breaking, and each item under crates/happenstance-testkit/tests/ or crates/happenstance-sync/ recorded as not public API with its reason — the verdict for each crate read from the tool's own output and not from the architecture brief's Surface class column, which is the expectation and not the authority; the implied release is 0.2.x with no 0.3.0 implied or required, and no manifest version key moves in this PR."
  satisfied: false
  evidence: ""
  mount_point: "The AC-011 finding record's enumeration table in this story's folder, derived from spec/audits/surface-diff-<YYYY-MM-DD>.md"
  verifying_test: "Row-by-row cross-check of `git diff <project-base>..HEAD -- crates/` pub items against the enumeration; `git diff <project-base>..HEAD -- Cargo.toml crates/happenstance-testkit/Cargo.toml` shows no version-key change; the report's per-crate release-version is 0.2.x and matches what cargo xtask surface-diff check mode recomputes"
- id: AC-006
  criterion: "GIVEN an honest answer that may need a port surface the contract does not have, WHEN the record is read at the initiative's gate, THEN exactly one branch is taken explicitly: the escalation branch names the surface by path (method, type or variant), states which of DA-7's four rows it is, prices it as a minor bump under 0.x — a 0.3.0 this initiative's exit criteria do not contemplate — and reproduces the full four-row table so the reader sees what lost; or the no-escalation branch states in terms that none is owed, names the ADR-0028 branch that made it so, and dispatches every \"missing surface\" finding the two reader stories recorded (decision-model-and-ingest-observed, projection-runner-across-the-hole) either to a DA-7 row or to the record as reachable inside AC-011. Nothing is left behind in a sibling story's ledger, and an empty section is not a branch."
  satisfied: false
  evidence: ""
  mount_point: "The AC-012 escalation record in this story's folder, raised to the initiative through the project's own review roll-up (the initiative charter is not edited here)"
  verifying_test: "Content review that exactly one branch is present and complete against .bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:363-379; mechanical cross-read of decision-model-and-ingest-observed/_ledger.md and projection-runner-across-the-hole/_ledger.md for every AC-A08 \"missing surface\" finding, asserting each is dispatched here"
- id: AC-007
  criterion: "GIVEN the consumer relying on the [FROZEN] ES-37 (spec/SPECIFICATION.md:4274-4297), WHEN this project's merged range is diffed, THEN no public signature in crates/happenstance-core/src/store.rs or crates/happenstance-core/src/append.rs has changed — including in the branch where this story concludes a change is needed, because AC-012 exists to make AC-011 a decision rather than a suppression — and any rustdoc-only change there (ES-40's obligation, DA-5) is recorded as not a semver surface with that distinction stated rather than assumed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs and crates/happenstance-core/src/append.rs — read-only subjects of the confirmation; both are outside this story's PR boundary"
  verifying_test: "`git diff <project-base>..HEAD -- crates/happenstance-core/src/store.rs crates/happenstance-core/src/append.rs` reviewed for signature changes (expected none, or rustdoc only), corroborated by spec/audits/surface-diff-<YYYY-MM-DD>.md in which such a change would surface as a breaking finding"
- id: AC-008
  criterion: "GIVEN the reader of a green gate, who must never be able to mistake an unrun instrument for a clean surface, WHEN any precondition fails — cargo-semver-checks absent, the registry unreachable, 0.2.0 not published because publish-0-2-0 has not shipped, surface-diff absent from xtask/src/main.rs's REQUIRED list, or spec/audits/ absent — THEN this story halts and reports, naming the path, the value actually read and the owner, and records no verdict: it does not relabel the PR-grain job as the registry comparison, does not hand-write a report, and does not copy the seam map's expectation into the record as a finding."
  satisfied: false
  evidence: ""
  mount_point: "The halt record in this story's folder; structurally guarded by the surface-diff check step at xtask/src/main.rs:105, which fails a report with no capture behind it"
  verifying_test: "Content review of the halt record (path, value read, owner named) against .kb/decisions/0010-the-suite-must-prove-itself.md; assertion that no committed spec/audits/surface-diff-*.md exists without a --run transcript behind it"
```
