---
item: "HS-S0046"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The name reserved and the manifest publishable, with publishing left to someone else

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows here are proved by a **recorded transcript** rather than a re-runnable gate — AC-001 and
AC-007 — because `xtask/src/package.rs`'s `PUBLISHABLE` (`:86`) does not check an unpublished crate
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md:641`). Paste the
command line and its output verbatim; a paraphrase makes this story's central claim unfalsifiable.

Three artefacts belong in this file beyond the rows themselves, and the spec names them as evidence:
the `cargo package -p happenstance-sqlite --list --allow-dirty` transcript (AC-007), the fact-by-fact
citation table pairing every number in the README with the `src/`/`tests/` line it comes from
(AC-005), and the note recording `RUNBOOK.md:4230`'s stale `publish = false` exit checkbox as
superseded by project AC-015 (AC-008).

```yaml
- id: AC-001
  criterion: >-
    GIVEN an evaluator who has met the name `happenstance-sqlite` in `spec/SPECIFICATION.md` and
    `RUNBOOK.md` and goes looking for it, WHEN they open
    `https://crates.io/crates/happenstance-sqlite`, THEN the name resolves to this project — because
    `cargo xtask reserve happenstance-sqlite` generated the `0.0.0` placeholder, its `--dry-run` was
    verified first, and the upload was performed under explicit human authorisation, with version,
    date and registry URL recorded in `_ledger.md`. AND if no crates.io credential is available, the
    ledger row records a named handoff — who holds the token and what remains — and never a claim
    that the name was taken.
  satisfied: true
  evidence: "PERFORMED 2026-08-18 UNDER EXPLICIT HUMAN AUTHORISATION. The three facts this row was waiting on are at the END of this evidence, recorded from the upload transcript itself, which existed only in the terminal that ran it. EVERYTHING BETWEEN HERE AND THAT SECTION IS KEPT VERBATIM AS WRITTEN WHILE THE ROW WAS BLOCKED - it is the verification chain the upload rested on, and rewriting it would discard the record of what was established before the irreversible act rather than after it. It read: BLOCKED ON A HUMAN-AUTHORISED, IRREVERSIBLE ACT - and this row states the handoff rather than the name being taken, which EC-001 makes the pass condition for this outcome. EVERYTHING SHORT OF THE UPLOAD IS DONE AND VERIFIED. `cargo xtask reserve happenstance-sqlite` ran and wrote the placeholder to target/reserve/happenstance-sqlite/ carrying Cargo.toml, LICENSE-APACHE, LICENSE-MIT, README.md and src/, and it printed the two commands in the order the module intends - the dry run first, then the claim under the words 'This is irreversible - a version can be yanked, never removed'. THE DRY RUN WAS THEN EXECUTED and is green: `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml --dry-run` printed `Updating crates.io index` / `Packaging happenstance-sqlite v0.0.0` / `Packaged 7 files, 15.8KiB (6.0KiB compressed)` / `Verifying happenstance-sqlite v0.0.0` / `Compiling happenstance-sqlite v0.0.0` / `Finished dev profile` / `Uploading happenstance-sqlite v0.0.0` / `warning: aborting upload due to dry run`. So the artefact is ready and it packages and compiles. WHY THE UPLOAD DID NOT HAPPEN: a crates.io token IS present at C:/Users/ryanm/.cargo/credentials.toml, so EC-001's literal condition (no credential) does not hold - the blocker is the OTHER half of the criterion, that the upload is a human-authorised act. Publishing is irreversible and public; an implementing agent's authority does not extend to it, and the honest failure mode here is a row claiming a name was taken. THE NAMED HANDOFF: the repository owner (ryan.britton, who holds the crates.io token in $CARGO_HOME/credentials.toml) runs exactly one command from the worktree root - `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml` - regenerating the placeholder first with `cargo xtask reserve happenstance-sqlite` if target/ has been cleaned. What then remains for this row is three facts: the published version (0.0.0), the date, and https://crates.io/crates/happenstance-sqlite resolving. Nothing in the workspace tree changes: the placeholder is generated under target/ and declares its own empty [workspace]. AC-002's reconciliation is COMPLETE BEFORE that command is available to run, which is the ordering that criterion exists to guarantee - see its row, and both are in the same commit as this ledger. THE UPLOAD, PERFORMED 2026-08-18: the repository owner authorised it explicitly at the run-4 gate and the orchestrator executed it, the authority AC-001 requires being the authorisation rather than the keystroke. The artefact was regenerated with `cargo xtask reserve happenstance-sqlite` and INSPECTED BEFORE UPLOAD - version 0.0.0, description without the stale 'Not yet implemented.', LICENSE-MIT and LICENSE-APACHE both present, and both surfaces naming 0.2.0-alpha.1 (README.md line 15, src/lib.rs line 4) - then dry-run again, green. The upload transcript verbatim: `Updating crates.io index` / `Packaging happenstance-sqlite v0.0.0` / `Packaged 7 files, 15.8KiB (6.0KiB compressed)` / `Verifying happenstance-sqlite v0.0.0` / `Compiling happenstance-sqlite v0.0.0` / `Finished dev profile [unoptimized + debuginfo] target(s) in 0.28s` / `Uploading happenstance-sqlite v0.0.0` / `Uploaded happenstance-sqlite v0.0.0 to registry crates-io` / `Published happenstance-sqlite v0.0.0 at registry crates-io`. THE THREE FACTS THIS ROW OWED, taken from the registry rather than from the upload's own claim - GET https://crates.io/api/v1/crates/happenstance-sqlite returns name happenstance-sqlite, max_version 0.0.0, created_at 2026-08-18T13:16:06.844473Z, yanked false, description 'SQLite event store and projection store adapters for happenstance.' So https://crates.io/crates/happenstance-sqlite resolves to this project. NOTHING IN THE WORKSPACE TREE CHANGED: the placeholder is generated under target/ and declares its own empty [workspace]; `publish = false` and PUBLISHABLE are untouched, which is AC-015's second half. DISCLOSED RATHER THAN DISCOVERED LATER: the published README carries three links to https://github.com/Wet-Ink-Corporation/happenstance, all of which return HTTP 404 to an anonymous client because the repository is private. That is finding N-3 from HS-P0011's review, owned by publication-and-positioning (HS-P0016), and it is NOT this story's to fix - the links are identical strings that begin resolving the moment the repository is public, with no republish and no version bump. It is recorded here because this row is the reason a fourth crates.io page now carries them, and the decision to ship them was taken knowingly at the run-4 gate rather than found afterwards."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "manual, recorded: `cargo xtask reserve happenstance-sqlite` (xtask/src/reserve.rs:137-191, dispatch xtask/src/main.rs:670) then `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml --dry-run`; transcripts verbatim in this ledger + resolvable https://crates.io/crates/happenstance-sqlite"

- id: AC-002
  criterion: >-
    GIVEN an evaluator reading the placeholder's own front page to learn when this crate becomes
    worth depending on, WHEN it names the first functional release, THEN it names the version this
    initiative will actually ship — `0.2.0-alpha.1` per BR-05
    (.bklg/from-contract-to-published-library/initiative.md:288) — because `xtask/src/reserve.rs:258`
    and `:276` were reconciled before the irreversible upload, OR the divergence is recorded in
    `_ledger.md` with a stated reason. Whichever is chosen, the six unreserved names inherit it.
  satisfied: true
  evidence: "RECONCILED, NOT DIVERGED, AND BEFORE ANY UPLOAD IS POSSIBLE. Both template sentences now name 0.2.0-alpha.1: xtask/src/reserve.rs:258 ('The first functional release will be `0.2.0-alpha.1`.') in the README template and xtask/src/reserve.rs:276 ('will be `0.2.0-alpha.1`; this version exists so that the name matches the') in the lib.rs template. `grep -n '0\\.1\\.0-alpha\\.1' xtask/src/reserve.rs` now matches only inside the new test module, where SUPERSEDED_RELEASE_CLAIM (:304) holds the string deliberately as the value a regression would reintroduce. THE REGENERATED PLACEHOLDER CARRIES IT: after re-running `cargo xtask reserve happenstance-sqlite`, `grep -n '0\\.2\\.0-alpha\\.1' target/reserve/happenstance-sqlite/README.md target/reserve/happenstance-sqlite/src/lib.rs` matches both - README.md:15 and src/lib.rs:4. A TEST NOW HOLDS IT FOR ALL TEN NAMES, not just this one: xtask/src/reserve.rs::tests::every_placeholder_names_the_release_this_project_will_ship renders readme() and lib_rs() for every RESERVABLE row and asserts both surfaces name 0.2.0-alpha.1 and neither names 0.1.0-alpha.1. IT WAS RED FIRST, on the base tree, with 'the happenstance placeholder's README.md still promises 0.1.0-alpha.1, which this initiative superseded with 0.2.0-alpha.1' - so the six unreserved names inherit the corrected claim mechanically rather than by anyone remembering. `cargo test -p xtask` green (227 passed inside `cargo xtask affected --base main`). ORDERING IS STRUCTURAL RATHER THAN CHRONOLOGICAL HERE: AC-001's upload has not happened at all, so the reconciliation cannot fail to precede it, and EC-004 is unreachable for this reservation."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "xtask/src/reserve.rs::tests::every_placeholder_names_the_release_this_project_will_ship; `rg -n \"0\\.1\\.0-alpha\\.1\" xtask/src/reserve.rs` empty; re-run `cargo xtask reserve happenstance-sqlite` and `rg -n \"0\\.2\\.0-alpha\\.1\" target/reserve/happenstance-sqlite/README.md target/reserve/happenstance-sqlite/src/lib.rs` matches both; `cargo test -p xtask` green; ledger ordering shows the reconciliation predates AC-001's upload"

- id: AC-003
  criterion: >-
    GIVEN an application author scanning crates.io search results for a SQLite event store
    (initiative AC-03), WHEN `happenstance-sqlite` appears, THEN its one-line description says what
    the adapter is and no longer ends "Not yet implemented." of a crate that now passes the
    conformance suite — `crates/happenstance-sqlite/Cargo.toml:3` carries `RESERVABLE`'s wording
    verbatim (`xtask/src/reserve.rs:88`) — AND `readme = "README.md"` is stated in the manifest
    rather than left to Cargo's auto-discovery, for the reason
    `crates/happenstance-core/Cargo.toml:12-15` gives.
  satisfied: true
  evidence: "crates/happenstance-sqlite/Cargo.toml:3 now reads `description = \"SQLite event store and projection store adapters for happenstance.\"` - character-for-character the RESERVABLE row's wording at xtask/src/reserve.rs:88, with the trailing ' Not yet implemented.' gone. `grep -n 'Not yet implemented' crates/happenstance-sqlite/Cargo.toml` returns nothing. THE COMPARISON IS NOW MECHANICAL RATHER THAN A REVIEW EYEBALL, which is the part worth more than the edit: xtask/src/reserve.rs::tests::a_crate_with_a_readme_carries_what_the_placeholder_carries asserts, for every RESERVABLE name that has a crates/<name>/README.md, that the manifest's description EQUALS the row's. It was RED on this exact assertion - `left: Some(\"SQLite event store and projection store adapters for happenstance. Not yet implemented.\") right: Some(\"SQLite event store and projection store adapters for happenstance.\")` - so the placeholder and the release can no longer describe the same crate differently without a test failing. `readme = \"README.md\"` IS STATED at crates/happenstance-sqlite/Cargo.toml:22, above `publish = false`, with a comment giving the same reason crates/happenstance-core/Cargo.toml:12-15 gives: Cargo would auto-discover the file, but what the key decides is what crates.io renders as the front page, and a silent default is a poor thing for a first impression to rest on. `cargo xtask affected --base main` green proves the manifest still parses and the feature matrix is unmoved."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "xtask/src/reserve.rs::tests::a_crate_with_a_readme_carries_what_the_placeholder_carries; `rg -n \"Not yet implemented\" crates/happenstance-sqlite/Cargo.toml` empty; `rg -n '^readme = \"README.md\"' crates/happenstance-sqlite/Cargo.toml` matches; description compared character-for-character against xtask/src/reserve.rs:88; `cargo xtask affected --base main` green (.redkiln/config.yaml:40)"

- id: AC-004
  criterion: >-
    GIVEN a consumer who unpacks the `.crate` to exercise the choice `license = "MIT OR Apache-2.0"`
    invites them to make, WHEN they look inside the package directory, THEN both licence texts are
    there as byte-identical copies of the workspace-root files — not references, not relative paths
    outside the package, which is precisely the shape D11 shipped (`xtask/src/package.rs:1-22`).
  satisfied: true
  evidence: "COPIES INSIDE THE PACKAGE DIRECTORY, PROVED BYTE-IDENTICAL. `git diff --no-index LICENSE-MIT crates/happenstance-sqlite/LICENSE-MIT` produced no output and exited 0, and the same for LICENSE-APACHE - so the texts are identical to the workspace root's, and EC-007's CRLF hazard on this Windows checkout did not fire (both were written with `cp`, which copies bytes and does not translate). `cargo package -p happenstance-sqlite --list --allow-dirty` lists both, inside the artefact: see AC-007's transcript. THE DEFECT THIS IS NOT: D11, quoted at xtask/src/package.rs:1-22, was three crates whose `license = \"MIT OR Apache-2.0\"` was backed by licence text that lived only at the workspace root - Cargo does not follow a path outside the package directory and does not warn when the field is backed by nothing. A reference or a relative path would have looked correct in the manifest and shipped nothing. GUARDED FROM HERE ON by xtask/src/reserve.rs::tests::a_crate_with_a_readme_carries_what_the_placeholder_carries, which was RED with 'happenstance-sqlite/LICENSE-MIT is missing: the manifest offers a choice of two licences and the package would carry neither' before the copies landed, and which compares bytes rather than existence."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "xtask/src/reserve.rs::tests::a_crate_with_a_readme_carries_what_the_placeholder_carries; `cargo package -p happenstance-sqlite --list --allow-dirty` lists LICENSE-MIT and LICENSE-APACHE; `git diff --no-index LICENSE-MIT crates/happenstance-sqlite/LICENSE-MIT` and `git diff --no-index LICENSE-APACHE crates/happenstance-sqlite/LICENSE-APACHE` both produce no output"

- id: AC-005
  criterion: >-
    GIVEN an evaluator who wants to know what this store promises about durability and limits before
    reading a line of its source (initiative AC-09), WHEN they read the crate's front page, THEN it
    states this adapter's real, current facts: the journal mode, the `synchronous` setting and the
    finite busy timeout `schema-migration-and-identity` configured; the three ceilings `append`
    enforces (`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH`, declared per
    `crates/happenstance-testkit/src/contract.rs:253-279`); and that the conformance suite is green
    against it — as numbers read out of the landed code, never the aspirational text the module doc
    carried while the crate was an instrument.
  satisfied: true
  evidence: "EVERY NUMBER IN THE README IS PAIRED WITH THE LINE IT WAS READ OUT OF, and a fact with no citable line was not written. DURABILITY (README.md:47-57, the three-row table): `journal_mode` = WAL cites crates/happenstance-sqlite/src/connection.rs:33 `pub const JOURNAL_MODE: &str = \"wal\"`; `synchronous` = NORMAL (1) cites connection.rs:46 `pub const SYNCHRONOUS: i64 = 1` and the pragma issued at connection.rs:95; `busy_timeout` = 5,000 ms cites connection.rs:62 `pub const BUSY_TIMEOUT_MS: u64 = 5_000` and its application at connection.rs:93. All three are PUBLIC constants, which is what lets the README say 'a public constant you can assert against' rather than asking to be believed; connection.rs:174-196 is the reader that compares them back out of PRAGMA. The 'OFF is not on the menu' sentence cites connection.rs:42-45, which names it as the wrong implementation the reopen rule rejects; the '64-way contention with no SQLITE_BUSY' sentence cites connection.rs:59-61 (ADR-0022 section 11). CEILINGS (README.md:59-73): 1,048,576 cites crates/happenstance-sqlite/src/event_store.rs:245 MAX_EVENT_DATA_LEN; 128 cites event_store.rs:252 MAX_TAGS_PER_EVENT; 256 cites event_store.rs:261 MAX_EVENTS_PER_BATCH. That `append` ENFORCES them rather than merely declaring them cites event_store.rs:475, :482 and :488, and the ExceedsStoreLimit claim cites event_store.rs:1005-1008. 'Checked from both sides' cites crates/happenstance-sqlite/tests/append.rs:238-286, which appends exactly the limit and requires acceptance, then one more and requires refusal. THE GAP CLAUSE (README.md:75-78) cites the AUTOINCREMENT column at event_store.rs:49 and its 'permits gaps' paragraph. CONFORMANCE STATUS (README.md:8-14) is true on this tree: `cargo test -p happenstance-sqlite` green with conformance.rs 90 passed, concurrency.rs 9 passed and, with --all-features, projection.rs 24 passed. THE SHAPE PARAGRAPH (README.md:27-39) carries forward crates/happenstance-sqlite/src/lib.rs:21-29's portfolio-axis statement rather than inventing a second version of it. NOTHING ASPIRATIONAL SURVIVED: no sentence in this README came from the module doc the crate carried as an instrument - that text was deleted one story earlier and is not a source."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "fact-by-fact citation table in this ledger pairing every README number with its crates/happenstance-sqlite/src/ or crates/happenstance-sqlite/tests/ line; `cargo test -p happenstance-sqlite` green on the same tree; `cargo package -p happenstance-sqlite --list --allow-dirty` lists README.md"

- id: AC-006
  criterion: >-
    GIVEN an application author who arrived by following `crates/happenstance-core/README.md:14-22`'s
    "which crate do I want" pointer, WHEN they land on this crate's page, THEN it has the same shape
    its siblings have — title, a one-sentence identity linking the repository and the DCB
    specification, a status blockquote, a which-crate pointer, and a Licence section — within the
    sibling density budget (60–130 lines, 4–6 `##` sections, identity in one sentence, at most one
    fenced block above the which-crate pointer), AND every Rust fence is marked `rust,ignore`, so no
    fence advertises an example that cannot compile (D10). No `include_str!` is added to
    `crates/happenstance-sqlite/src/lib.rs`, which belongs to the predecessor story.
  satisfied: true
  evidence: "THE HOUSE SHAPE, IN ORDER: title (README.md:1), one-sentence identity linking the repository and the DCB specification (:3-6), status blockquote (:8-14), 'Which crate do I want?' pointer (:16-25), then the technical detail - the shape (:27-45), durability (:47-57), limits (:59-78) - and a Licence section last (:80-82). That is crates/happenstance-core/README.md:1-24's hierarchy with this crate's own status line, which is the line that genuinely differs: the sibling says 'every storage adapter is a documented stub' and this one says the suite is green. DENSITY, MEASURED NOT GUESSED: 81 lines and 5 `##` sections, against a budget of 60-130 lines and 4-6 sections taken from the siblings (crates/happenstance/README.md 60 lines; crates/happenstance-core/README.md 67 lines, 5 sections; crates/happenstance-testkit/README.md 130 lines). ONE FENCED BLOCK, and it sits BELOW the which-crate pointer, so the budget's 'at most one fenced block above the pointer' is satisfied with zero. EVERY RUST FENCE IS `rust,ignore`: the only fence in the file opens at README.md:41 tagged rust,ignore and closes at :45; no line in the file is an untagged Rust fence. THE CHOICE IS DELIBERATE RATHER THAN OMITTED - the alternative is crates/happenstance/src/lib.rs:1-10's `#![cfg_attr(doctest, doc = include_str!(\"../README.md\"))]`, and taking it would edit a file this story does not own. `grep -n include_str crates/happenstance-sqlite/src/lib.rs` is EMPTY and `git diff d2d1f73 -- crates/happenstance-sqlite/src/` is EMPTY: not one byte of src/ moved in this story. GUARDED: xtask/src/reserve.rs::tests::no_readme_advertises_an_example_nothing_compiles fails any README carrying a bare Rust fence in a crate whose lib.rs does not doctest that README - the D10 rule stated generally rather than as this crate's habit."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "xtask/src/reserve.rs::tests::no_readme_advertises_an_example_nothing_compiles; rg over crates/happenstance-sqlite/README.md finds no bare rust fence (only rust,ignore); `rg -n \"include_str\" crates/happenstance-sqlite/src/lib.rs` empty; `git diff main -- crates/happenstance-sqlite/src/` empty; line and section counts recorded here against crates/happenstance-core/README.md (67 lines, 5 sections) and crates/happenstance-testkit/README.md (130 lines)"

- id: AC-007
  criterion: >-
    GIVEN the publisher who will later add this crate to `PUBLISHABLE` and needs the packaging gate
    to pass on the first attempt rather than rediscover D11, WHEN they read this story's ledger, THEN
    it carries the verbatim output of `cargo package -p happenstance-sqlite --list --allow-dirty`,
    including the command line, showing `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` in the
    listing — the run, not a claim that it was run.
  satisfied: true
  evidence: "THE RUN, NOT A CLAIM THAT IT WAS RUN. Command: `cargo package -p happenstance-sqlite --list --allow-dirty`. Output verbatim, 23 lines: .cargo_vcs_info.json / Cargo.lock / Cargo.toml / Cargo.toml.orig / LICENSE-APACHE / LICENSE-MIT / README.md / src/connection.rs / src/event_store.rs / src/lib.rs / src/projection_store.rs / src/query_sql.rs / src/row.rs / tests/append.rs / tests/concurrency.rs / tests/conformance.rs / tests/front_page.rs / tests/migration.rs / tests/projection.rs / tests/read.rs / tests/shapes.rs / tests/support/mod.rs / tests/wide_query.rs. All three of README.md, LICENSE-MIT and LICENSE-APACHE are in the artefact. THE FALSIFIER IS NOT HYPOTHETICAL and is quoted at xtask/src/package.rs:20-22: the same command on the pre-PR tree exits 0, lists SEVEN files, and none of them is one of the three - which is exactly why a green exit code proves nothing here and the listing is the evidence. `--allow-dirty` because the check must work on an uncommitted tree, the only tree anyone runs it against before pushing. Reproducible by re-running; it reads the manifest and the directory, not a cache."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "manual, recorded: verbatim `cargo package -p happenstance-sqlite --list --allow-dirty` transcript in this ledger, reproducible by re-running; falsifier quoted at xtask/src/package.rs:20-22 (pre-PR: exits 0, lists seven files, none of the three)"

- id: AC-008
  criterion: >-
    GIVEN the maintainer of `publication-and-positioning` (HS-P0016), whose project owns whether this
    crate ships at all, WHEN this PR merges, THEN the decision is still theirs —
    `crates/happenstance-sqlite/Cargo.toml:12`'s `publish = false` stands, `xtask/src/package.rs` is
    byte-identical to its pre-PR state, `cargo xtask package-check` prints the same three-crate
    reconciliation as before, AND `RUNBOOK.md:4230`'s stale "`publish = false` removed" checkbox is
    recorded in `_ledger.md` as superseded by AC-015 rather than edited or silently diverged from.
  satisfied: true
  evidence: "THE DECISION IS STILL HS-P0016's. `git diff d2d1f73 -- xtask/src/package.rs RUNBOOK.md` is EMPTY - neither file changed by one byte in this story (d2d1f73 is this story's base; `git diff main` is the wrong comparison on this branch, where predecessor stories legitimately edited other files). `grep -n '^publish = false' crates/happenstance-sqlite/Cargo.toml` matches at :22, now carrying a comment stating WHY it stays rather than leaving the next reader to rediscover it: PUBLISHABLE still names three crates, so deleting this line would fail reconcile() on a promoted-but-unreconciled crate, which is the failure xtask/src/package.rs was written to produce (EC-005 made unreachable by explanation rather than by hope). `cargo xtask package-check` printed, after this story's changes, `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit` followed by the same three per-crate lines as before - THE INVARIANCE IS THE EVIDENCE: this crate is asked nothing, exactly as before. RUNBOOK.md:4230 IS RECORDED AS SUPERSEDED, NOT EDITED: its phase-8 exit checkbox reads '`publish = false` removed', and project AC-015, architecture brief AC-A05 (_decomposition.md:55-57) and the initiative decomposition all say the opposite and are the current authority. The checkbox is therefore knowingly not ticked, and the runbook is left untouched because it is the plan of record and its phase-8 exit criteria are read at phase closeout by the runbook's owner. See the *Recorded artefacts* note below, which states the supersession in prose so a closeout audit reading phase 8 finds a decision rather than a silence. NOTED AND NOT FIXED, per the spec: the generated placeholder's `rust-version = \"1.85\"` (xtask/src/reserve.rs:221) binds nothing - the placeholder has no dependencies and no code - and the MSRV promise is BR-08's, owned by publication-and-positioning."
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "`git diff main -- xtask/src/package.rs RUNBOOK.md` empty; `rg -n \"^publish = false\" crates/happenstance-sqlite/Cargo.toml` matches; `cargo xtask package-check` (xtask/src/main.rs:680) prints `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit` (xtask/src/package.rs:213-216); supersession note present in this ledger"
```

## Recorded artefacts

Filled in by the implementer, referenced by the rows above.

### `cargo package -p happenstance-sqlite --list --allow-dirty` (AC-007)

```console
$ cargo package -p happenstance-sqlite --list --allow-dirty
.cargo_vcs_info.json
Cargo.lock
Cargo.toml
Cargo.toml.orig
LICENSE-APACHE
LICENSE-MIT
README.md
src/connection.rs
src/event_store.rs
src/lib.rs
src/projection_store.rs
src/query_sql.rs
src/row.rs
tests/append.rs
tests/concurrency.rs
tests/conformance.rs
tests/front_page.rs
tests/migration.rs
tests/projection.rs
tests/read.rs
tests/shapes.rs
tests/support/mod.rs
tests/wide_query.rs
```

Twenty-three files. `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` are all in the artefact, which is
the whole of what this row asserts. On the pre-PR tree the same command exits `0` and lists **seven**
files, none of them one of the three — the falsifier `xtask/src/package.rs:20-22` already quotes, and
the reason a green exit code is not evidence here.

### README fact-by-fact citations (AC-005)

Every number the README states, and the line it was read out of. A fact with no citable line was not
written rather than softened.

| Fact | Value in the README | Read from |
| --- | --- | --- |
| journal mode | `WAL` (`README.md:55`) | `crates/happenstance-sqlite/src/connection.rs:33` — `pub const JOURNAL_MODE: &str = "wal"`; applied at `:111-152` |
| `synchronous` | `NORMAL` (`1`) (`README.md:56`) | `src/connection.rs:46` — `pub const SYNCHRONOUS: i64 = 1`; the pragma issued at `:95` |
| why not `OFF` | "the specification names it as the wrong implementation the reopen rule exists to reject" (`README.md:56`) | `src/connection.rs:42-45` |
| busy timeout | `5,000` ms (`README.md:57`) | `src/connection.rs:62` — `pub const BUSY_TIMEOUT_MS: u64 = 5_000`; applied at `:93` |
| "64-way contention with no `SQLITE_BUSY`" | `README.md:57` | `src/connection.rs:59-61` (ADR-0022 §11) |
| the three settings are readable back | "a public constant you can assert against" (`README.md:49-51`) | `src/connection.rs:174-196`, the reader that compares them against `PRAGMA` |
| max bytes of `data` on one event | `1,048,576` (`README.md:66`) | `src/event_store.rs:245` — `pub const MAX_EVENT_DATA_LEN: usize = 1_048_576` |
| max tags on one event | `128` (`README.md:67`) | `src/event_store.rs:252` |
| max events in one append | `256` (`README.md:68`) | `src/event_store.rs:261` |
| `append` **enforces** them | `README.md:61-62` | `src/event_store.rs:475`, `:482`, `:488`; the error contract at `:1005-1008` |
| "checked from both sides" | `README.md:70-73` | `crates/happenstance-sqlite/tests/append.rs:238-286` — the exact limit accepted, one more refused |
| positions may gap | `README.md:75-78` | `src/event_store.rs:49` (`AUTOINCREMENT`, "gaps allowed") and `:100-104` |
| conformance status | `README.md:8-14` | `cargo test -p happenstance-sqlite`: `tests/conformance.rs` 90 passed, `tests/concurrency.rs` 9 passed; `--all-features` `tests/projection.rs` 24 passed |
| the shape it represents | `README.md:27-39` | `src/lib.rs:21-29`, carried forward rather than re-invented |

### `RUNBOOK.md:4230` supersession note (AC-008)

`RUNBOOK.md:4230`'s phase-8 exit checkbox reads **"`publish = false` removed"**. It is **superseded**,
not skipped and not silently diverged from.

Project **AC-015** (`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md:268-272`)
requires the opposite — the crate becomes publishable while publishing "stays someone else's
decision" — and the architecture brief's §8
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md:379-395`) settles
the conflict in this project's favour with a mechanism rather than a preference: with `PUBLISHABLE`
(`xtask/src/package.rs:86`) still naming only `happenstance-core`, `happenstance` and
`happenstance-testkit`, deleting `publish = false` here makes `reconcile()` fail the gate on a
promoted-but-unreconciled crate. Architecture **AC-A05** (`_decomposition.md:55-57`) says the same.

`RUNBOOK.md` is therefore **left unedited**. It is the plan of record, its phase-8 exit criteria are
read at phase closeout by its owner, and correcting a checkbox in it is not this story's act. What
this story owes is that a closeout audit reading that line finds a recorded decision rather than a
silence — which is this note.

### Reservation record (AC-001, AC-002)

**Not performed. AC-001 stays `satisfied: false`, and this is a handoff rather than a claim.**

| Field | Value |
| --- | --- |
| Placeholder generated | yes — `cargo xtask reserve happenstance-sqlite`, written to `target/reserve/happenstance-sqlite/` with `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`, `src/lib.rs` |
| Dry run | **green** — `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml --dry-run`: `Packaged 7 files, 15.8KiB (6.0KiB compressed)`, `Verifying happenstance-sqlite v0.0.0`, `Compiling happenstance-sqlite v0.0.0`, `warning: aborting upload due to dry run` |
| Version claim reconciled first | **yes** — `0.2.0-alpha.1` in both templates before any upload was possible (AC-002) |
| Version published | — |
| Date | — |
| Registry URL | — (unresolved; the name is **not** claimed) |
| Who authorises | the repository owner, who holds the crates.io token in `$CARGO_HOME/credentials.toml` |
| What remains | one command from the worktree root: `cargo publish --manifest-path target/reserve/happenstance-sqlite/Cargo.toml` — re-running `cargo xtask reserve happenstance-sqlite` first if `target/` has been cleaned. Then fill the three empty fields above and flip AC-001 |

A token **is** present on this machine, so `EC-001`'s literal condition (no credential) does not hold.
The blocker is the other half of the criterion: the upload is irreversible, public, and explicitly a
human-authorised act. Nothing in the tracked tree depends on it — the placeholder lives under
`target/` and declares its own empty `[workspace]` — so every other criterion in this story lands
regardless, which is what `EC-001` prescribes.
