---
item: "HS-S0046"
stage: implement
created: "2026-08-18"
updated: "2026-08-18"
---

# Implementation Report — The name reserved and the manifest publishable, with publishing left to someone else

> **STATUS: seven of eight ACs satisfied. AC-001 is a recorded handoff, not a
> claim.** The crate is packageable: `cargo package -p happenstance-sqlite
> --list --allow-dirty` lists 23 files including `README.md`, `LICENSE-MIT` and
> `LICENSE-APACHE`, where the pre-PR tree listed seven and none of the three.
>
> **The name is not held.** The placeholder is generated and its `--dry-run` is
> green, but `cargo publish` is irreversible, public, and — by this story's own
> criterion — a human-authorised act. It was not performed. AC-001 stays
> `satisfied: false` with a named handoff, which is what `EC-001` makes the pass
> condition for this outcome and what the criterion's own last clause demands.
>
> The durable gain beyond the four files is that **three of this story's claims
> became tests** in `xtask/src/reserve.rs`, so the six unreserved names inherit
> them mechanically rather than by anyone remembering in six weeks.

## TDD Evidence

| AC | Test | Red, and for what reason | Green |
| --- | --- | --- | --- |
| **AC-002** | `reserve::tests::every_placeholder_names_the_release_this_project_will_ship` | Red on the base tree, on the assertion: *"the happenstance placeholder's README.md still promises 0.1.0-alpha.1, which this initiative superseded with 0.2.0-alpha.1"*. It renders `readme()` **and** `lib_rs()` for all ten `RESERVABLE` rows, because a reader deciding when to depend may meet either surface | green after `reserve.rs:258` and `:276` |
| **AC-003** | `reserve::tests::a_crate_with_a_readme_carries_what_the_placeholder_carries` | Red on the description, with both strings printed: *left* `"… for happenstance. Not yet implemented."*, *right* `"… for happenstance."`. The trigger is the presence of `README.md`, so it fired the moment this story gave the crate a front page | green after `Cargo.toml:3` |
| **AC-004** | the same test, second assertion | Red next, once the description matched: *"happenstance-sqlite/LICENSE-MIT is missing: the manifest offers a choice of two licences and the package would carry neither"*. It compares **bytes**, not existence, so a re-wrapped or CRLF-translated copy fails too | green after both `cp`s |
| **AC-006** | `reserve::tests::no_readme_advertises_an_example_nothing_compiles` | A guard rather than a red-first test, and it says so: it fails any README carrying an untagged Rust fence in a crate whose `lib.rs` does not doctest that README — the D10 rule stated generally rather than as this crate's habit | green |
| **AC-001** | — | Proved by transcript, not by test. `cargo publish --dry-run` green; the upload not performed | **handoff** |
| **AC-005** | — | Proved by a fact-by-fact citation table in `_ledger.md`: fourteen rows, each pairing a README number with the `src/` or `tests/` line it was read from. **Nothing compiles a README**, so this is the only instrument there is, and `EC-006` says so | green |
| **AC-007** | — | The `cargo package --list` transcript, verbatim in `_ledger.md` | green |
| **AC-008** | — | `git diff d2d1f73 -- xtask/src/package.rs RUNBOOK.md` empty; `package-check` prints the same three-crate reconciliation | green |

**Why the tests live in `xtask/src/reserve.rs` and not beside the crate.** The
story's PR boundary is five paths and `crates/happenstance-sqlite/tests/**` is
not one of them — AC-006 verifies `git diff -- crates/happenstance-sqlite/src/`
is empty, and a test target added to that crate would be scope leak by the
story's own definition. `reserve.rs` is in boundary, and the invariant is
genuinely its own: `reserve.rs:40-43` already states that the placeholder and the
release must describe the same thing, and `:158-165` already copies the licences
for the reason D11 exists. These tests are that comment, enforced.

## Commits

- `12ac09f` — `feat(sqlite-durable-store): The name reserved and the manifest publishable`

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/Cargo.toml` | `description` drops `" Not yet implemented."` and becomes `RESERVABLE`'s wording verbatim (`:3`). `readme = "README.md"` stated with the reason (`:22`). `publish = false` kept, now with a comment saying **why** it stays — `PUBLISHABLE` names three crates, so deleting it fails `reconcile()` on a promoted-but-unreconciled crate |
| `crates/happenstance-sqlite/README.md` | **New**, 81 lines, 5 `##` sections. Title, one-sentence identity, status blockquote, which-crate pointer, then the shape, the three durability settings, the three enforced ceilings and the gap warning. One fenced block, `rust,ignore`, below the pointer |
| `crates/happenstance-sqlite/LICENSE-MIT`, `LICENSE-APACHE` | **New**, byte-identical copies of the workspace-root texts |
| `xtask/src/reserve.rs` | `:258` and `:276` reconciled to `0.2.0-alpha.1`. A `#[cfg(test)] mod tests` added carrying the three tests above |

Untouched, and verified untouched: `xtask/src/package.rs`, `RUNBOOK.md`,
`crates/happenstance-sqlite/src/**`, and the crate's feature matrix.

## Gates

| Gate | Grain | Result |
| --- | --- | --- |
| `cargo test -p xtask --bins reserve` | inner loop | 3 passed (2 red before the change, in two separate red steps) |
| `cargo package -p happenstance-sqlite --list --allow-dirty` | story instrument | 23 files, all three present |
| `git diff --no-index LICENSE-MIT crates/happenstance-sqlite/LICENSE-MIT` | story instrument | no output, exit 0; same for `LICENSE-APACHE` |
| `cargo xtask package-check` | story instrument | unchanged three-crate reconciliation |
| `cargo xtask affected --base main` | story (`affected_gate`) | `affected gate passed` (227 tests) |
| `cargo fmt --all -- --check` | formatter | clean |
| `cargo xtask ci --fast` | integration (`integration_scoped`) | `all required checks passed (--fast: 4 optional step(s) not run)` |

**One self-heal.** `cargo xtask affected` went red on the *new test's own doc
comment*: `clippy::doc_markdown` reported *"backticks are unbalanced"* on a line
containing a literal triple-backtick Rust fence tag. Fixed by describing the
fence rather than spelling it. No test was weakened; the assertion is unchanged.

## Notes

**Deviation 1 — AC-001 is not satisfied, deliberately.** A crates.io token *is*
present on this machine, so `EC-001`'s literal condition (no credential) does not
hold. The blocker is the criterion's other half: the upload is irreversible and
explicitly human-authorised. Publishing under an agent's own authority would make
the one outcome this story treats as a defect — a ledger row claiming a name that
resolves to nothing, or worse, an unwanted permanent registry entry. The
placeholder, the green dry run and the one remaining command are recorded in
`_ledger.md` under *Reservation record*.

**Deviation 2 — the description's stale half was left by the previous story on
purpose, and is fixed here.** `instrument-markers-removed-and-gate-green` could
see `" Not yet implemented."` and was forbidden to touch it. That hand-off worked:
the string survived one story and was removed by the story that owns it, with a
test that now makes its return a failure.

**A finding, routed rather than fixed.** `crates/happenstance-core/README.md:8-9`
still reads *"every storage adapter is a documented stub"*. That is now false —
this crate is not a stub — but that file is outside this story's PR boundary in
both directions. It belongs to `publication-and-positioning` (HS-P0016), which
owns the registry-facing pass, and is recorded here rather than absorbed.

**Noted, not fixed, per the spec.** The generated placeholder's
`rust-version = "1.85"` (`reserve.rs:221`) binds nothing — the placeholder has no
dependencies and no code — and the MSRV *promise* is BR-08's.
