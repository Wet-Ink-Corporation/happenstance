# Assessment: docs and developer experience (docs-dx)

Commit `1f92d08` (== origin/main), 2026-10-06. Read-only. Primary inputs were map-docs.md, map-vision.md and gate-summary.md. Every load-bearing claim below was spot-checked in the worktree or against GitHub Actions.

**Rating: AMBER. About 40% complete against the 1.0 docs target.**

**Headline:** A competent Rust developer can get from `cargo add` to a working event store with conditional appends using the docs alone. Projections, operations, choosing an adapter and upgrading still have to be pieced together from long example sources and rustdoc. The documentation site has never deployed, and no reader other than the author has tested the docs.

## Can a stranger adopt it today?

| Journey step | Covered by | Verdict |
|---|---|---|
| First append, read back, conditional append | `docs/first-encounter.md` (3 compiled programs), `docs/append-conditions.md`, README "DCB in sixty seconds" | **Yes**, and good |
| Typed events, decision model, `commit` | `crates/happenstance/src/lib.rs` front-door doctest | **Yes** |
| Carrying an invariant into your own domain | `docs/carry-your-invariant.md` (220 lines) and the `course-subscriptions` example | **Yes** |
| A durable store (SQLite, Postgres) | crate READMEs and `examples/transfers-on-sqlite` | Partial: reference material only, no guide |
| Projections and read models | none of the 7 `docs/` pages; runner rustdoc (on docs.rs via `all-features`, but behind `unstable-projection`); examples of 663–1,480 lines | **Gap** |
| Rebuilding, resetting and backfilling a view | `examples/rebuilding-read-models` only | **Gap** |
| Choosing an adapter | README status table; the planned `choose-a-store` page is absent | Partial |
| Upgrading from 0.3 to 0.4 | CHANGELOG only | **Gap** |
| Cloudflare / Durable Object | Cloudflare README and its 454-line crate docs; the planned how-to is absent | Partial, and the workerd limit is not stated in user terms |

## Strengths

- **The docs that exist are compiled.** The narrative pages are registered in `xtask/src/narrative.rs` and compiled by the mandatory narrative-doctests step. Crate READMEs are compiled as doctests (for example `crates/happenstance/src/lib.rs:10`). The measured gate passed all of it (gate-summary.md: "narrative docs checks" green).
- **The tutorial is a good one.** `docs/first-encounter.md` has three standalone programs. Each shows its expected output and links to the clause it relies on. The pages under `docs/` themselves are low in internal jargon: 0 ADR or phase references across the 7 pages.
- **The examples are broad and run.** Seven examples cover the canonical DCB case, SQLite durability, rebuild and backfill, uniqueness and quota, codec evolution, HTTP plus a runner, and an outside-author falsifier. All 5 that were run as binaries exited 0 (gate-summary.md).
- **The rustdoc is rich.** The typed crate's front door has a feature table and a compiled first program (`crates/happenstance/src/lib.rs:19-130`). Every published crate sets `[package.metadata.docs.rs] all-features = true`, so the gated runner still renders on docs.rs.
- **The project is honest about maturity.** The README's "Read the ✅ rows narrowly" note (`README.md:155-158`) and the Cloudflare README's "It is not `workerd`" section (`crates/happenstance-cloudflare/README.md:106-113`) say this plainly.
- **The site machinery is built.** There is a Zola shell, `cargo xtask site`, and examples scraped into the API docs. `pages.yml` builds successfully; only the deploy fails.

## Findings

### F1 (high): The documentation site has never deployed
- Claim: phase 22's goal is "the site deployed from `main`" (`runbook/README.md:109`).
- What the evidence shows: there have been 3 of 3 Pages runs on `main` (36810591479, 36863802395 and 37406385128), and all of them failed.
- Log of run 37406385128, deploy job 112085167260: `Failed to create deployment (status: 404) ... Ensure GitHub Pages has been enabled`.
- The build job succeeds. The fix is a setting only the owner can change: the unchecked item "The owner sets the repository's Pages source to *GitHub Actions*" (`runbook/phases/22-docs-site.md:84`).
- The proof artefact URL is unchecked (`:88`).

### F2 (high): No guide page covers projections or read models
- `grep -c -i projection docs/*.md` gives 0 for all 7 pages. The same grep for "read model" also gives 0.
- The only routes to a working projection are:
  - the runner rustdoc (`crates/happenstance/src/runner.rs:1-30`), which sits behind `unstable-projection` (`crates/happenstance/Cargo.toml:148`);
  - example sources of 663 to 1,480 lines.
- The planned `rebuild-a-read-model` how-to is unchecked (`runbook/phases/22-docs-site.md:75-78`).
- Read models are half of event sourcing, so this is the biggest adoption gap.

### F3 (medium-high): The five planned how-to pages and the bridge page do not exist
- `runbook/phases/22-docs-site.md:75-80` lists these, all unchecked:
  - `choose-a-store`
  - `rebuild-a-read-model`
  - `retry-a-command`
  - `pass-the-conformance-suite`
  - `run-on-a-durable-object`
  - `boundaries-not-aggregates.md`
- `ls docs` shows only the 7 existing pages. The site has no search (`:83`).

### F4 (medium): No upgrade guide for 0.3 to 0.4, although 0.4 is breaking
- The workspace version is already `0.4.0` (`Cargo.toml:24`).
- `CHANGELOG.md`'s `[Unreleased]` section (`:35-340`) contains 8 "breaking" mentions:
  - core's `unstable-projection` removed (`crates/happenstance/Cargo.toml:140`);
  - postgres `naive-arm` removed;
  - `commit` now retries `Busy`;
  - and others.
- There is no consolidated migration page: `ls docs | grep -iE 'migrat|upgrad'` returns nothing. There are only scattered per-entry "Migration:" notes.

### F5 (medium): The crate READMEs shown on crates.io contradict the current release
- `crates/happenstance/README.md:11` says "Four ship at `0.2.0`", and `:29` says "**`0.2.0` is the first stable release**". The current release is 0.3.2, and 0.4.0 is in the tree.
- `crates/happenstance-core/README.md:10-14` says "93-rule event-store conformance suite" and lists "an embedded graph database for the projection role" among the five adapters. That is Ladybug, which was retired by ADR-0078 and is no longer in the workspace.
- Meanwhile the root README says 116 rules (`README.md:16`).
- These are the front pages a stranger reads first on crates.io and docs.rs, for the bare-name crate and the contract crate.
- The root README and the adapter READMEs pin `"0.3"` (`README.md:194`), which is correct today but will be stale at 0.4.0.

### F6 (medium): Cloudflare's real-runtime limit is not stated in terms a user can act on
- The PR #34 commit (`1f92d08`) says: "the adapter evaluates at most 5 query items. VT-23's 128-item rule fails". `runbook/handover.md:44-48` adds "at most 5 query items and 45 tags in one item".
- The Cloudflare README gives only the platform's raw numbers: "5 compound-`SELECT` terms, 100 bound parameters" (`crates/happenstance-cloudflare/README.md:119-120`).
- Neither that README nor the root README's Cloudflare note (`README.md:177-183`) tells a user that a query with more than 5 items, or more than 45 tags in one item, fails on real workerd. They also do not say that one conformance rule is red there.
- So a user reading the docs would expect broader query support than the runtime gives.

### F7 (medium): No reader other than the author has tested the docs
- Phase 20's goal requires "a reader who is not the author has used it and been listened to" (`runbook/phases/20-docs-that-teach.md:3-5`).
- HS-P0024 (comprehension evidence) is "not yet built" (`:19-22`).
- The only friction-log artefact is a skeleton: `find . -iname '*friction*'` returns `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton`.
- The partial counter-evidence is `examples/outside-projection-adapter`, which was written from the rendered docs. But it was written by an agent inside the project, and its gap record is filed under `.bklg/` (`examples/outside-projection-adapter/src/lib.rs:10`).
- Phase 22 itself says of its personas: "None has been observed".

### F8 (low-medium): Prose density and internal clutter at the front door
- The root README is 392 lines. It carries 5 ADR references, 6 phase references, and 6 pointers into `.kb/`, `runbook/` or `RUNBOOK.md`. Its status block discusses the release-set decision commit `e597c34` (`README.md:171-172`).
- The repo root shows a stranger `HANDOVER.md`, `REMEDIATION-HANDOVER.md`, `SESSION-DECISIONS-0.2.0.md` and the 5,704-line frozen `RUNBOOK.md`.
- Both `README.md:185` and `docs/README.md:30` send readers to `RUNBOOK.md` as "the plan". That file is frozen: its own header says "the live runbook is `runbook/README.md`" (`RUNBOOK.md:3`).
- Half of `docs/README.md` (`:26-55`) describes gate mechanics to a user audience.
- The `docs/` pages themselves are clean (F-strengths). The density is in the READMEs and crate docs.

### F9 (low): The examples have no READMEs and cannot be installed from crates.io
- `find examples -iname 'README*'` returns nothing.
- Each example's entry point is its `//!` header. Those headers are good: for example, `examples/transfers-on-sqlite/src/main.rs:1-16` says what the example proves and gives `cargo run -p ...`.
- All examples are `publish = false` workspace members, so a stranger has to clone the repo to run them.
- The scraped-examples API pages would help here, once the site deploys (F1).

## Claims versus evidence

| Claim | What the evidence shows |
|---|---|
| "Rust examples on these pages are compiled by `cargo xtask ci`" (`docs/README.md:3-6`) | **Confirmed**: the gate passed the narrative-doctests step (gate-summary.md) |
| The site is deployed (the phase 22 goal) | **False today**: 3 of 3 deploys failed with a Pages 404 |
| "0.2.0 is the first stable release" (happenstance README) | **Stale**: 0.3.2 is current, and 0.4.0 is in the tree |
| "Five adapters … an embedded graph database" (core README) | **Stale**: Ladybug was retired (ADR-0078) |
| Cloudflare "passes the suite on wasm32" (`README.md:150`) | True under the node shim. On real workerd, 96 of 97 pass locally and 93 of 96 deployed (PR #34). User docs do not surface the 5-item limit |
