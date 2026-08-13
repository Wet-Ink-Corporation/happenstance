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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "`rg -n \"0\\.1\\.0-alpha\\.1\" xtask/src/reserve.rs` empty; re-run `cargo xtask reserve happenstance-sqlite` and `rg -n \"0\\.2\\.0-alpha\\.1\" target/reserve/happenstance-sqlite/README.md target/reserve/happenstance-sqlite/src/lib.rs` matches both; `cargo test -p xtask` green; ledger ordering shows the reconciliation predates AC-001's upload"

- id: AC-003
  criterion: >-
    GIVEN an application author scanning crates.io search results for a SQLite event store
    (initiative AC-03), WHEN `happenstance-sqlite` appears, THEN its one-line description says what
    the adapter is and no longer ends "Not yet implemented." of a crate that now passes the
    conformance suite — `crates/happenstance-sqlite/Cargo.toml:3` carries `RESERVABLE`'s wording
    verbatim (`xtask/src/reserve.rs:88`) — AND `readme = "README.md"` is stated in the manifest
    rather than left to Cargo's auto-discovery, for the reason
    `crates/happenstance-core/Cargo.toml:12-15` gives.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "`rg -n \"Not yet implemented\" crates/happenstance-sqlite/Cargo.toml` empty; `rg -n '^readme = \"README.md\"' crates/happenstance-sqlite/Cargo.toml` matches; description compared character-for-character against xtask/src/reserve.rs:88; `cargo xtask affected --base main` green (.redkiln/config.yaml:40)"

- id: AC-004
  criterion: >-
    GIVEN a consumer who unpacks the `.crate` to exercise the choice `license = "MIT OR Apache-2.0"`
    invites them to make, WHEN they look inside the package directory, THEN both licence texts are
    there as byte-identical copies of the workspace-root files — not references, not relative paths
    outside the package, which is precisely the shape D11 shipped (`xtask/src/package.rs:1-22`).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "`cargo package -p happenstance-sqlite --list --allow-dirty` lists LICENSE-MIT and LICENSE-APACHE; `git diff --no-index LICENSE-MIT crates/happenstance-sqlite/LICENSE-MIT` and `git diff --no-index LICENSE-APACHE crates/happenstance-sqlite/LICENSE-APACHE` both produce no output"

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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "rg over crates/happenstance-sqlite/README.md finds no bare rust fence (only rust,ignore); `rg -n \"include_str\" crates/happenstance-sqlite/src/lib.rs` empty; `git diff main -- crates/happenstance-sqlite/src/` empty; line and section counts recorded here against crates/happenstance-core/README.md (67 lines, 5 sections) and crates/happenstance-testkit/README.md (130 lines)"

- id: AC-007
  criterion: >-
    GIVEN the publisher who will later add this crate to `PUBLISHABLE` and needs the packaging gate
    to pass on the first attempt rather than rediscover D11, WHEN they read this story's ledger, THEN
    it carries the verbatim output of `cargo package -p happenstance-sqlite --list --allow-dirty`,
    including the command line, showing `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` in the
    listing — the run, not a claim that it was run.
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/Cargo.toml"
  verifying_test: "`git diff main -- xtask/src/package.rs RUNBOOK.md` empty; `rg -n \"^publish = false\" crates/happenstance-sqlite/Cargo.toml` matches; `cargo xtask package-check` (xtask/src/main.rs:680) prints `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit` (xtask/src/package.rs:213-216); supersession note present in this ledger"
```

## Recorded artefacts

Filled in by the implementer, referenced by the rows above.

### `cargo package -p happenstance-sqlite --list --allow-dirty` (AC-007)

_Not yet run. Paste the command line and its complete output here, verbatim._

### README fact-by-fact citations (AC-005)

_Not yet written. One row per stated number: fact | value in the README | `file:line` it was read from._

### `RUNBOOK.md:4230` supersession note (AC-008)

_Not yet written. Record that the phase-8 exit checkbox "`publish = false` removed" is superseded by
project AC-015 (`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md:268-272`),
per the architecture brief's §8 ruling
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md:379-395`), and that
the runbook was deliberately left unedited because its exit criteria are read at phase closeout._

### Reservation record (AC-001, AC-002)

_Not yet performed. Record: the version published, the date, the resolvable registry URL, who
authorised the upload — or, if no credential was available, the named handoff and what remains._
