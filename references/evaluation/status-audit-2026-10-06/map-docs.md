# Map: documentation, examples and developer experience

Audited at commit `1f92d08` (origin/main), read-only worktree, 2026-10-06. No cargo commands run.
"CLAIM" = what a document says; "SHOWS" = what the tree / CI evidence shows.

## 1. Inventory

### 1.1 `docs/` — the narrative tree (649 lines total, `wc -l docs/*.md`)

| Page | Lines | Declared need | Notes |
|---|---|---|---|
| docs/README.md | 66 | router | routes only; points contributors to spec/, runbook, standards |
| docs/first-encounter.md | 162 | `tutorial` — watch a boundary refuse a write | 3 runnable programs on `MemoryEventStore` (docs/first-encounter.md:3, 16-35) |
| docs/append-conditions.md | 74 | append under a condition | |
| docs/read-the-worked-example.md | 23 | pointer to course-subscriptions | |
| docs/carry-your-invariant.md | 220 | carry invariant into your domain | |
| docs/adapter-reading-order.md | 77 | adapter author path | |
| docs/text-fences.md | 27 | fences the compiler never sees | meta page |

- SHOWS: `grep -c -i projection docs/*.md` returns **0 for every page**. No narrative page covers projections / read models / the runner.
- CLAIM (docs/README.md:3-6): Rust examples on these pages are compiled by `cargo xtask ci` (registered in `xtask/src/narrative.rs`). Not re-run here (unverified in this audit; gate owned elsewhere).

### 1.2 Examples (all workspace members via `members = ["crates/*", "examples/*", ...]`, Cargo.toml:3; all `publish = false`)

| Example | src LOC | tests LOC | README | Demonstrates (from its own `//!` header) |
|---|---|---|---|---|
| course-subscriptions | 512 | 1103 | none (has `src/overview.md`) | canonical DCB worked example, in memory |
| transfers-on-sqlite | 663 | 836 | none | typed command loop + projection runner on happenstance-sqlite, survives reopen (src/main.rs:1-12) |
| rebuilding-read-models | 1480 | 126 | none | four views, blue/green backfill, reset, rebuild, poisoned view (src/main.rs:1-12) |
| handles-and-quotas | 942 | 116 | none (has `src/overview.md`) | unique name, per-owner quota, idempotent delivery |
| telemetry-across-codecs | 810 | 127 | none | JSON + postcard in one log, v1 upcast in `decode` (src/main.rs:1-9) |
| tickets-over-http | 1706 | 119 | none | HTTP API + projection runner over one SQLite file, 202 until caught up (src/lib.rs:1-10) |
| outside-projection-adapter | 608 | 394 | none | falsifier: projection adapter written from rendered docs only (src/lib.rs:1-10) |

- SHOWS: `find examples -name 'README*'` returns nothing. Each example's only entry doc is its crate `//!` header (or overview.md for two).
- SHOWS: 6 of 7 examples enable `unstable-projection` (`grep -l unstable-projection examples/*/Cargo.toml`) — the projection runner is still feature-gated (crates/happenstance/Cargo.toml:67; CLAUDE.md ADR-0074 note: phase 18 not built).
- All 7 carry `doc-scrape-examples` (grep count 2 per Cargo.toml).

### 1.3 Crate READMEs and crate-level rustdoc

| Crate | README lines | `//!` lines in lib.rs |
|---|---|---|
| happenstance | 149 | 129 |
| happenstance-core | 81 | 94 |
| happenstance-testkit | 260 | 419 |
| happenstance-sqlite | 152 | 85 |
| happenstance-cloudflare | 193 | 454 |
| happenstance-postgres | 149 | 179 |
| happenstance-neon | 168 | 130 |
| happenstance-sync | **none** | 132 (unpublished skeleton) |
| happenstance-ladybug | none | 158 (retired) |

- `happenstance` front door (crates/happenstance/src/lib.rs:1-46): a compiled doctest showing DomainEvent + DecisionModel + `commit` on MemoryEventStore; links to first-encounter on GitHub (lib.rs:48). Good zero-to-command-handler path.
- Runner rustdoc exists (crates/happenstance/src/runner.rs:1-5) but is behind `unstable-projection`.

### 1.4 Root-level files

| File | Lines | Observation |
|---|---|---|
| README.md | 392 | DCB in 60 s, Status table, Quick start, Writing an adapter |
| CHANGELOG.md | 3925 | `[Unreleased]` at :35 carries many BREAKING entries for 0.4.0 (:58, :86, :103, :141, :270, :283 relative grep) |
| CONTRIBUTING.md | 369 | tool install (`cargo install cargo-hack cargo-deny`, :99) |
| SECURITY.md | 108 | present |
| also HANDOVER.md, REMEDIATION-HANDOVER.md, SESSION-DECISIONS-0.2.0.md, RUNBOOK.md (5,704-line frozen monolith) at root | | internal process files visible to a stranger at repo root |

## 2. Version drift a stranger would hit

- SHOWS: workspace version is `0.4.0` (Cargo.toml:24), but README status says "`0.3.2`" (README.md:16, 139-150) and Quick start says `happenstance = "0.3"` (README.md:193-197).
- Crate READMEs pin `"0.3"`: crates/happenstance-sqlite/README.md:34-35, happenstance-postgres/README.md:48-49, happenstance-neon/README.md:50-51.
- This is correct for what is on crates.io today (0.3.2 per CLAUDE.md) but will be stale at the 0.4.0 release; no evidence of a version-bump checklist for READMEs (unverified).

## 3. Migration guide 0.3 -> 0.4

- SHOWS: no migration guide. `grep -rniE 'migrat(ion|ing) (guide|from)|upgrading' CHANGELOG.md docs README.md` returns only CHANGELOG.md:399 ("Read the first entry below before upgrading an adapter", in the 0.3.0 section).
- Inline "Migration:" notes exist inside individual CHANGELOG entries (CHANGELOG.md:1440, :1534) but the `[Unreleased]` 0.4.0 breaking set (testkit emitters public, `commit` retries busy store, removal of core's `unstable-projection`, postgres `naive-arm` removed, sqlite busy timeout) has no consolidated upgrade page.

## 4. Documentation site (phase 22)

- Shell exists: `site/config.toml`, templates (base/index/page/section/404/components), `site/content/guide/_index.md`, `site/content/reference.md`, brand tokens, 2 highlight themes, 10 static files. Landed in `09854b3` (#29).
- Workflow: `.github/workflows/pages.yml` builds on PR, deploys from `main` only.
- **SHOWS: the site has never deployed.** `gh run list --workflow pages.yml --branch main` -> 3 runs, all `failure` (36810591479 on 2026-10-01 for #29; 36863802395; 37406385128 on 2026-10-06). `gh run view 37406385128`: build ✓, deploy ✗ — "Failed to create deployment (status: 404) ... Ensure GitHub Pages has been enabled". PR builds succeed (e.g. 37419424003).
- Cause matches the open runbook item "The owner sets the repository's Pages source to *GitHub Actions*" (runbook/phases/22-docs-site.md, Work list, unchecked). Owner action, not code.
- Phase 22 open items (runbook/phases/22-docs-site.md): rustdoc logo/favicon (withdrawn); repoint two crate-doc links after first deploy; five how-to pages (`choose-a-store`, `rebuild-a-read-model`, `retry-a-command`, `pass-the-conformance-suite`, `run-on-a-durable-object`) — **none exist in docs/**; `boundaries-not-aggregates.md` — absent; search — not built; all four exit criteria unchecked. Session log has one entry (2026-09-29).
- The phase's own persona table says "None has been observed" (22-docs-site.md, "The audience, reconciled").

## 5. Phase 20 (docs that teach) status

- runbook/phases/20-docs-that-teach.md: all exit criteria unchecked; Session log (line 54) is empty; estimate "Not yet made"; HS-P0023/24/25 not built, HS-P0020–22 "at review" when .bklg froze.
- Comprehension evidence: only `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton` exists (find -iname '*friction*'). No dated friction log from a non-author reader -> the initiative's "witnessed reader" bar is unmet.
- ES-23 adapter-half pointer (carried from phase 16): open.

## 6. Against references/seeds/user-documentation.md

- Seed's problem (seed :36 ff): the gate proves docs compile and links resolve, "None of it reads a sentence." Its vision requires a reader who learns from the docs.
- Status vs seed: compile-checking and need-declaration machinery has grown (narrative doctests, lint-pages); the human-comprehension half remains unevidenced (section 5).

## 7. Can a new Rust developer go from zero to an event store + projection?

| Step | Covered by | Verdict |
|---|---|---|
| Install + first append/read | README Quick start; docs/first-encounter.md step 1 | yes (in memory) |
| Conditional append / DCB decision | first-encounter steps 2-3, append-conditions.md, happenstance lib.rs doctest | yes, good |
| Typed events + decision model + `commit` | crates/happenstance/src/lib.rs:7-46 | yes |
| Durable store (SQLite) | sqlite README; transfers-on-sqlite example | partial — reference + example, no guide page |
| Projection / read model | **no docs/ page**; runner rustdoc behind `unstable-projection`; transfers-on-sqlite, rebuilding-read-models examples | **gap** — must reverse-engineer from 663-1480-line example sources with no README |
| Operations (rebuild, reset, backfill, migrations, schema) | rebuilding-read-models example only | gap — no operations guide |
| Choosing an adapter | README Status table | partial — the planned `choose-a-store` how-to is absent |
| Upgrading 0.3 -> 0.4 | CHANGELOG only | gap |
| Edge/Durable Object | cloudflare README (193) + 454-line rustdoc | partial — planned how-to absent |

Overall: an experienced Rust developer can reach a working **event store with conditional appends** from docs alone. Reaching a **projection** requires examples + rustdoc on an unstable feature; no tutorial or how-to exists. The writing is dense with internal references (ADRs, phase numbers, clause IDs, `.bklg` paths — e.g. outside-projection-adapter/src/lib.rs:10 cites a `.bklg` path), which the target "new to Rust" reader will find heavy.

## 8. Gaps (ranked)

1. Pages never deployed — owner must enable Pages (Actions source); 3/3 main deploys failed.
2. No projection/read-model guide page; runner still `unstable-projection` (phase 18 pending).
3. Five planned how-tos + `boundaries-not-aggregates` not written.
4. No 0.3 -> 0.4 migration guide despite ≥6 BREAKING entries in [Unreleased].
5. No README in any example directory.
6. No witnessed-reader friction log (phase 20 core proof).
7. README / crate READMEs say 0.3 / 0.3.2 while workspace is 0.4.0 (correct now, stale at release).
8. Root clutter (HANDOVER.md, REMEDIATION-HANDOVER.md, SESSION-DECISIONS-0.2.0.md, RUNBOOK.md) faces strangers.
9. No search on the site; no rustdoc logo.
