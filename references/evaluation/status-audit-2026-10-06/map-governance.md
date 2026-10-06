# Governance, release and process health — map

Audited worktree: `audit-wt` at `1f92d08` (== origin/main, "Phase 17 L6a: the workerd job, landed red on purpose (#34)"). Date 2026-10-06. Read-only; no cargo commands run. All counts below are from commands run in the worktree root.

Caveat: the clone is shallow (`git rev-parse --is-shallow-repository` -> `true`; oldest visible commit `673dcc5` 2026-09-07), so commit-count statistics are not representative of the project's full history.

## 1. Release state

| Item | Evidence | Status |
|---|---|---|
| Workspace version | `Cargo.toml:24` `version = "0.4.0"`; `Cargo.toml:60-61` core/happenstance requirements `0.4.0`; `crates/happenstance-testkit/Cargo.toml:36` `0.4.0` | 0.4.0 in manifests, **unreleased** |
| Last published | `CHANGELOG.md:341` `## [0.3.2] — 2026-09-20`; remote tags `v0.3.0`, `v0.3.1`, `v0.3.2` (`git ls-remote --tags origin`) | 0.3.2 is the live release |
| 0.4.0 changelog | `CHANGELOG.md:35` `## [Unreleased]` with Changed (l.37), Added (l.212), Removed (l.302) — ~305 lines pending | in progress |
| Releases in CHANGELOG | 0.2.0-alpha.1 (2026-08-16, l.2310), 0.2.0 (2026-09-10, l.457), 0.3.0/0.3.1 (2026-09-11), 0.3.2 (2026-09-20) | 5 entries; CHANGELOG is 3,925 lines |
| Publish-false crates | `crates/happenstance-sync/Cargo.toml:12`, `crates/happenstance-ladybug/Cargo.toml:12` (retired, ADR-0078) | matches CLAUDE.md claim of two lines |
| Local tags | `git tag | wc -l` -> 0 (tags only on remote) | info |

Docs CLAIM (CLAUDE.md, runbook/handover.md) seven crates published at 0.3.2 — not re-verified against crates.io here (unverified from this audit; registry not queried).

## 2. Toolchain, MSRV, deny

| Item | Evidence |
|---|---|
| MSRV | `Cargo.toml:26` `rust-version = "1.97.1"`; every crate uses `rust-version.workspace = true` |
| Toolchain pin | `rust-toolchain.toml`: `channel = "1.97.1"`, components rustfmt+clippy, target wasm32-unknown-unknown |
| MSRV == pin | yes, so the `msrv` CI job (ci.yml:1043) currently re-runs the same compiler — CLAUDE.md admits this proves nothing until they diverge |
| deny.toml | `yanked = "deny"` (l.6); license allowlist of 8 (l.10-19); `multiple-versions = "warn"` (l.23); `wildcards = "deny"` with `allow-wildcard-paths = true` (l.36); async-trait banned with 5 wrapper exemptions (worker, worker-macros, wasm-bindgen-test, testcontainers, tonic) per ADR-0001/0035/0038 (l.100-108); unknown registry/git denied (l.111-112) |

## 3. CI (.github/workflows)

Two workflows: `ci.yml` (1,246 lines) and `pages.yml`.

| Job | Line | Trigger/condition | Note |
|---|---|---|---|
| gate | ci.yml:50 | push/PR, matrix ubuntu/windows/macos, `fetch-depth: 0` | runs `cargo xtask ci` |
| backlog | ci.yml:188 | `if: false` (l.255) | disabled since redkiln retired; kept with long justification comment |
| live-postgres | ci.yml:412 | always | real Postgres |
| live-neon | ci.yml:693 | steps gated on `env.NEON_CONNECTION != ''` | secret-dependent; comment l.830 records flaky rates ("3 red in 20 over HTTP/1.1, 1 red in 40...") |
| workerd | ci.yml:876 | local leg + deployed Cloudflare leg | **RED ON MAIN BY DESIGN**: commit 1f92d08 message and `runbook/handover.md` record 97 executed / 96 passed, VT-23's 128-item rule fails against workerd's 100-parameter / 5-compound-term limits; fix owed in lane L6b (ADR-0083, not yet written) |
| msrv | ci.yml:1043 | always | redundant while MSRV == pin |
| semver | ci.yml:1087 | PR only; `continue-on-error` when the version is unpublished (l.1133) | effectively soft for 0.4.0 since 0.4.0 is unpublished |
| advisories | ci.yml:1184 | schedule/dispatch only | weekly cron `17 7 * * 2` (l.14) |
| benchmarks | ci.yml:1232 | always | compile-only |
| pages build/deploy | pages.yml:46/70 | deploy on main only | docs site (phase 22) |

Key health finding: main is knowingly red on the `workerd` job (evidence: commit title of HEAD; handover "Merge #34 red, then L6b").

## 4. The gate (xtask)

`xtask/src` = 18 files + `site/`, **32,981 lines** of Rust (`wc -l xtask/src/*.rs`), plus `site/render.rs` 793. Largest: `lint_pages.rs` 5,587; `lint_narrative.rs` 5,167; `lints.rs` 4,601; `spec_trace.rs` 4,223; `proof.rs` 3,089; `main.rs` 2,211.

Subcommands (`xtask/src/main.rs:1067-1140`): ci, ci --fast, wasm, affected, reserve, spec-trace, package-check, proof-artefact, wasm-conformance, wasm-conformance-enumeration, narrative-doctests, narrative, site, lints, lint-clock, lint-testkit-version, lint-core-alloc-features, lint-changelog, lint-enumerations, lint-position-literals, lint-rule-counts, lint-retired-rules, lint-workflows, lint-kb, lint-pages, lint-constitution.

Gate steps (31 inline `name:` entries in main.rs plus `lint_workflows::STEP` and `lint_kb::STEP` at main.rs:904-905; ~33-36 total): formatting; clippy; tests; each phase's proof artefacts; wasm32 build of core / testkit check / cloudflare / neon / typed layer; wasm32 conformance non-vacuity; wasm32 run of conformance rules; documentation; specification traceability; no retired rule live; no rule reads a clock; enumerations; no literal positions; changelog per rule; stated rule counts; testkit version; core alloc features; constitution consistency; narrative doctests; constitution examples compile; narrative pages checked; every page declares one need; docs no-default-features; docs default features; package licences/README; feature powerset; wasm32 feature powerset; licences and advisories (cargo deny); docs.rs nightly; docs.rs wasm32 nightly; workflows pin actions and token scope; no accepted decision's body has changed.

Observation: roughly half of xtask's 33k lines (lint_pages, lint_narrative, lints, spec_trace) police documents and governance artefacts rather than library behaviour.

## 5. Knowledge base (.kb)

`.kb` = 301 files, 3.32 MB (`du -sb`).

### Decisions (`.kb/decisions/`)
96 atoms + README (`ls | wc -l` = 97). Status via `grep -m1 '^status:'`: **94 accepted, 2 superseded** (0002-crate-naming, 0021-payload-evolution-and-codec-tag).
Composition: 76 numbered ADRs 0001-0078 (0026 and 0027 absent — gap, unexplained in this pass), 2 `sd-` (brand/design), 17 `wi-` weigh-in decisions, 1 `standalone-svg-carries-one-colourway`.
Latest: 0078 happenstance-ladybug-is-retired. Handover refers to a future ADR-0083 (`runbook/handover.md` Next action 3), implying 0079-0082 are planned/queued but not present.

### Long-form records (`references/adr/`)
35 files, 13,802 lines total; largest 0016-the-wire-format.md 1,508 lines.

### Intake backlog (`.kb/_intake/`)
6 decision atoms staged, all `status: accepted`, not yet promoted: wi-269e5a, wi-3c4f18, wi-557b41, wi-642765, wi-8cdf0a, wi-c11a24 (`ls .kb/_intake/decisions`). CLAUDE.md says intake files are to be written into .kb/ and removed — these are pending promotion.

### Maps
`.kb/maps/`: README, decision-map.md, domain-map.md, open-questions-index.md.

### Open questions (`.kb/open-questions/`)
78 atoms: **47 accepted (= still open), 31 superseded (= resolved)**.

Still open (status: accepted):
| Atom | Title (abridged) |
|---|---|
| adr-0022-falsifiers-have-fired | Two of ADR-0022's falsifiers fired; nobody has re-opened |
| adr-status-vocabulary-exceeds-the-schema | No KB status for accepted-but-provisional ADR |
| cf-17-cf-14-maturity-markers-and-the-reopen-must | CF-17/CF-14 markers did not move |
| cf-18-residuals-after-declension-by-inheritance | residuals under CF-18 |
| cf-25-cf-26-portfolio-check-does-not-exist | spec-trace does not perform the named portfolio check |
| cf-33-cf-34-scope-outside-the-testkit | timed assertions outside the testkit |
| cf-36-names-a-cross-reference-nothing-performs | 13 breaches, three repairs |
| cf-38-case-naming-no-clause-reading | two readings differ by 54 cases |
| cf-5-conformant-control-per-rule-or-per-branch | control obligation granularity |
| dcb-reference-publishes-no-wire-format | no DCB wire format exists |
| disjoint-boundaries-have-no-clause | DCB independence unstated by clause |
| docs-citation-anchor-form-and-clause-contradiction-check | doc contradicted a clause undetected |
| es-11-ceiling-sample-cost-on-sqlite-read | 635 ms read cost under contention, no remedy measured |
| es-17-two-adapter-measurement-is-unscheduled | falsifier measurement unscheduled |
| es-18-byte-identical-versus-conformance-reading | byte-identical claim vs weaker rules |
| es-23-frozen-doc-musts-adapter-half | disposition for one of two MUSTs |
| es-38-and-gap-read-rules-are-unowned | two frozen clauses name unwritten rules |
| es-6-names-an-unwritable-rule | frozen clause names unwritable rule |
| es-7-and-vt-9-provisional-markers | falsifiers can no longer falsify |
| event-type-positional-mapping-has-no-compiler-check | unchecked positional event_type mapping |
| experiment-raw-output-eaten-by-the-ignore-rule | .gitignore eats experiment output |
| gate-step-first-check-hides-its-second | one step bundles three checks |
| happenstance-facade-does-not-match-adr-0006 | facade neither re-exports nor gates as ADR-0006 said |
| is-the-dagger-convention-superseded-by-maturity-markers | dagger convention |
| model-family-rule-has-no-clause | rule belongs to no clause |
| model-only-kind-memberless-dormant-or-withdrawn | empty mutant kind |
| off-poll-adapter-visibility-defect-undetected | no instrument for off-poll visibility defect |
| one-shot-http-conformance-to-es-11 | can any one-shot HTTP adapter satisfy ES-11 |
| postgres-neon-store-id-has-no-restore-detection | StoreId mint-once lacks VT-6 branch |
| projection-batch-sql-seam-statement-type | `&'static str` vs Statement type |
| projection-id-is-unvalidated | ProjectionId::new infallible, undecided |
| read-fault-rule-has-no-clause | rule with no clause |
| read-page-budget-is-unspecified | page budget in rows |
| read-to-backwards-limit-composition-gap | backwards+window+budget unwritten |
| references-adr-in-place-correction-policy | may references/adr be corrected in place |
| reset-refusal-declension-has-no-clause | RESET_REFUSAL un-mechanised |
| rustdoc-citations-relative-or-url-shaped | citation form now repo is public |
| scope-coverage-helper-and-the-projection-port-gap | helper endorsed, unscheduled |
| should-codec-be-sealed | Codec unsealed; free window closed |
| sole-evidence-pin-requirement-generality | 15 rules share shotgun hazard |
| sync-message-set-and-format-version | FORMAT_VERSION=1 names nonexistent message set |
| then-empty-emission-idiom-and-the-nothing-to-do-channel | then(&[]) semantics |
| trademark-search-gates-the-commercial-layer | trademark search not run |
| trait-variant-caret-resolves-past-the-locked-gate | caret-pinned proc macro |
| tuple-boundary-heterogeneous-event-type | tuple Boundary bound to first member |
| vt-30-provisional-marker-is-stale-and-unscheduled | VT-30 marker stale |
| what-the-exact-anchor-rule-still-leaves-open | three questions left by exact-anchor checker |

Classification (judgement, from titles): roughly 20 of 47 concern the governance machinery itself (clause/rule bookkeeping, citation form, markers, KB schema) rather than library behaviour; the remainder are genuine API/semantics questions (ProjectionId validation, Codec sealing, tuple Boundary typing, ES-11 on HTTP adapters, sync format version, trademark).

## 6. Specification maturity

`grep -c` occurrences in `spec/SPECIFICATION.md`: `[FROZEN]` 187, `[PROVISIONAL]` 23, `[DEFERRED]` 11. These are marker occurrences, not a deduplicated clause count (CLAUDE.md CLAIMS 202 clauses; not re-verified). spec/ = 12,052 lines, 743 KB.

## 7. Runbook and plan state

`runbook/` 17 files, 233 KB; frozen `RUNBOOK.md` 5,704 lines, 402 KB (kept for ~2,250 line citations, per runbook/README.md).

Status table (`runbook/README.md:80-110`): done = 0-12, 15, 16; in progress = 17, 20, 22; not started = 17b, 18, 13, 14, 19a, 19b, 21.

Checkbox counts per phase file (`grep -c '- \[x\]'` / `'- \[ \]'`):
| Phase | done | open |
|---|---|---|
| 13 sync | 1 | 25 |
| 14 retention | 3 | 11 |
| 15 reconcile | 18 | 0 |
| 16 define 1.0 | 15 | 0 |
| 17 breaking window | 9 | 17 |
| 17b | 0 | 14 |
| 18 typed runner | 0 | 19 |
| 19 sqlite wasm | 0 | 10 |
| 20 docs | 0 | 5 |
| 21 1.0.0 | 0 | 15 |
| 22 docs site | 4 | 10 |

Estimates (`runbook/roadmap.md:64-83`): solo 1.0 path 62-75 working days (was 34-43 before phase 17 re-estimate, ADR-0072). Remaining on path after done 15/16: 17 (25-30, in progress), 17b (8-10), 18 (5-8), 13 (12), 14 (5), 21 (3-5 + soak) = **~58-70 working days**, with phase 20 "to be estimated" and 19a/19b/22 off-path. The roadmap itself notes estimates in this repo have been wrong (l.82-83); phase 17 alone grew ~4-5x (5-8 -> 25-30 days).

Handover staleness: `runbook/handover.md` "As of" says PR #34 "Not merged", but HEAD `1f92d08` is the #34 merge — stale by one step (expected, since written in-lane). Owed follow-up for PR #33 review (`wi-13bd3b`) listed as waiting on owner.

## 8. Frozen trackers

| Path | Size | State |
|---|---|---|
| `.bklg/` | 1,318 files, 30.2 MB, 259,120 lines | frozen 2026-09-28 (last commit touching it: 2026-09-28); 3 initiatives (docs-that-teach, from-contract-to-published-library, support) |
| `.redkiln/` | 55 files, 1.36 MB | frozen |
| CI `backlog` job | ci.yml:255 `if: false` | disabled |

`.bklg` alone is ~3.7x the bytes of all Rust in the repo (30.2 MB vs 8.17 MB).

## 9. Documentation-to-code ratio and process overhead

Measurements (excluding `target/`, `node_modules/`, `.git/`):

| Measure | Value |
|---|---|
| All `*.md` | 1,859 files, 386,053 lines, 34.2 MB |
| `*.md` excluding .bklg/.redkiln | 137,029 lines, 8.79 MB |
| All `*.rs` | 396 files, 191,821 lines, 8.17 MB |
| Published-library `src/` (7 crates excl. testkit; incl. inline tests; excl. ladybug) | 39,409 lines, 1.72 MB |
| ... of which comment lines (`^\s*//`) vs other non-blank | 18,389 vs 18,175 (~1:1) |
| testkit (all rs) | 39,150 lines |
| xtask (all rs) | 34,808 lines |
| experiments rs / md | 30,629 / 8,947 |
| user docs `docs/` | **7 files, 649 lines, 30.6 KB** |

Process/governance artefacts (bytes, `du -sb`): .kb 3.32 MB; references 3.44 MB; spec 0.74 MB; RUNBOOK.md 0.40 MB; standards 0.36 MB; CHANGELOG 0.26 MB; runbook 0.23 MB; CLAUDE.md 0.03 MB => **~8.8 MB**, plus 31.6 MB frozen .bklg/.redkiln.

Ratios:
- Governance prose (8.8 MB) : published library source (1.72 MB) ~ **5.1 : 1** (excluding frozen backlog); ~23 : 1 including it.
- Non-frozen markdown bytes : all Rust bytes ~ 1.08 : 1.
- Tooling that polices artefacts (xtask 34.8k lines) is ~88% the size of the published library's src (39.4k lines).
- User-facing docs are ~0.35% of the governance corpus by bytes (30.6 KB vs 8.8 MB).
- CHANGELOG (3,925 lines) for 4 stable releases; CLAUDE.md alone is 467 lines (31 KB) and loads on every task.
- 109 `Co-Authored-By: Claude` trailers in 86 visible commits — the corpus is largely AI-generated, which plausibly explains its volume.

Candid judgement: for a solo-maintained library at 0.x with ~18k lines of non-comment shipped code, the process apparatus is disproportionate. Evidence of the overhead feeding on itself: 47 open questions of which roughly 20 are about the governance machinery; ~half of xtask's 33k lines lint documents; RUNBOOK.md cannot be shrunk because of ~2,250 line citations; accepted-ADR immutability plus line-anchored citations means corrections spawn new atoms rather than edits; phase 17 estimate grew 4-5x. Strengths are real: a conformance-suite-first discipline, a multi-OS gate, live Postgres/Neon/workerd legs, cargo-deny and semver checks, pinned actions. But the user documentation (`docs/`, 649 lines) is the thinnest artefact in the repo, while it is the one a 1.0 adopter reads; phase 20 ("docs that teach") has 0/5 boxes ticked and no estimate.

## 10. Gaps and unverified items
- crates.io publication state not queried (unverified here).
- ADR numbers 0026/0027 missing; reason not checked.
- Exact gate step count approximate (33-36) because some steps are composed by name.
- Commit history shallow; authorship/commit-volume stats partial.
