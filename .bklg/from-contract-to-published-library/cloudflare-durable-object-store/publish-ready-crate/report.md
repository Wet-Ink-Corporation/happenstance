---
item: "HS-S0059"
stage: report
created: "2026-08-20"
updated: "2026-08-20"
---

# Report — happenstance-cloudflare becomes a crate someone can depend on

## Findings Ledger

**Outcome: ten of twelve ACs satisfied; AC-010 and AC-012 are reported BLOCKED rather than
closed, and neither is blocked on anything inside this diff.** The crate is publish-*ready*
— both licence texts and its own README inside the package directory, `publish = false`
gone and the name inside `PUBLISHABLE` in the same change, a front page that says what the
crate is, a README example the gate compiles, and a conformance claim tied to the fixture's
own constants. What is not done is the one irreversible upload that claims the crates.io
name, and the one decision that would let the *full* gate go green.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **The packaging assertion now covers a fourth crate, and the coverage was proved by making it fail.** Adding the name to `PUBLISHABLE` is the mount: it is what turns "the three files are in the tree today" into a standing gate obligation | `cargo xtask package-check` names four crates and lists all three files inside `happenstance-cloudflare`'s artifact (19 files packaged). With `LICENSE-APACHE` moved aside the same program fails naming crate and filename; both drift directions (EC-001, EC-002) were driven too and printed their messages verbatim — transcripts in `implementation-report.md`, *Gates* | none |
| **A test target now reads what the crate says about itself.** `package-check` asserts three filenames and reads no word of their contents; that gap is where "publishable and lying" lives | `crates/happenstance-cloudflare/tests/publish_readiness.rs` — nine cases, eight red before the change. It ties the README's conformance paragraph to `CloudflareFixture`'s own `Capability` constants and declared ceilings, so the claim fails if the fixture moves in either direction | A reviewer should still read the paragraph; the test cannot see a true sentence that is narrower than it sounds |
| **The README example is compiled, and the compilation was proved non-vacuous.** `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` at `src/lib.rs:19` | `cargo test -p happenstance-cloudflare --doc` runs it; substituting `CloudflareEventStore::new(state)` for the real constructor turns it red with `error[E0308]`, restoring it turns it green | none. `no_run` is EC-004's narrowest honest attribute here and the reason is written at `src/lib.rs:14-18` |
| **The docs.rs configuration was verified in the exact shape docs.rs will build it**, rather than declared and hoped for | `[package.metadata.docs.rs]` with `default-target = "wasm32-unknown-unknown"` (`Cargo.toml:159-163`); `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc -p happenstance-cloudflare --no-deps --all-features --target wasm32-unknown-unknown` green | The nightly `docs.rs configuration` `OPTIONAL` step names its crates one by one and does not name this one. Adding it there is an `xtask` `Step` change, which this story's PR boundary assigns elsewhere — a small, real follow-up |
| **The deferred `host` visibility decision was taken.** `src/lib.rs` had recorded it as `publish-ready-crate`'s to make | The module travels, `#[doc(hidden)]`, because `tests/` is a second compilation unit that can only name it if it is `pub` — removing it would take the adapter's only executed conformance run with it. Argument and both losing alternatives at `crates/happenstance-cloudflare/src/lib.rs:38-53` | none |
| **This diff moved lines the Rust constitution cites, and the first full gate run caught it.** Eight citations across four atoms | Repointed to the anchors they name; `cargo run -p xtask -- lint-constitution` → `27 atoms, all consistent` | none. This is the lint working, not a defect |
| **BLOCKED — AC-010.** The crates.io name is not held, and claiming it is a one-way public action | `cargo xtask reserve happenstance-cloudflare` wrote the placeholder; `cargo publish --dry-run` green (`Packaged 7 files, 15.8KiB`, `aborting upload due to dry run`); `cargo search happenstance-cloudflare` returns nothing, so EC-005's already-claimed branch does not apply | **The maintainer runs one command**: `cargo publish --manifest-path target/reserve/happenstance-cloudflare/Cargo.toml`. `xtask` prints it and refuses to run it — *"Prints the publish command; never publishes anything itself"* — which is a human-in-the-loop seam this repository designed, and an implementation run is not the human |
| **BLOCKED — AC-012.** The full gate is one `OPTIONAL` step short of green, on a red that predates this diff | Every REQUIRED step passes, including all seven steps `wasm_steps()` names and `spec-trace`; `cargo deny check` reports `advisories ok, bans FAILED, licenses ok, sources ok` because `worker` and `worker-macros` depend unconditionally on the `async-trait` crate `deny.toml` bans under ADR-0001. This diff adds no dependency (NF-003) | **Dispose of `kb-open-question-worker-async-trait-ban-001`** — ratify a `wrappers` entry carrying the argument, or refuse and carve the step out deliberately. Not takeable here: `deny.toml` is outside the PR boundary, the identical edit was reverted by review as finding-deletion (`2ea99fd`), and every `.kb/` write in this project belongs to `adr-0023-and-atom-resolutions` |

## Acceptance

| AC | Status | Verified by |
| -- | ------ | ----------- |
| AC-001 | **satisfied** | `publish_readiness::both_licence_texts_travel_inside_the_crate_directory`; `cargo xtask package-check`; `git diff --no-index` on both pairs, exit 0 |
| AC-002 | **satisfied** | `publish_readiness::the_readme_is_beside_the_manifest_and_named_by_it`; `README.md` inside the packaged artifact; `readme = "README.md"` at `Cargo.toml:20` |
| AC-003 | **satisfied** | `cargo xtask package-check` exits zero naming four crates; EC-001 and EC-002 both reproduced deliberately |
| AC-004 | **satisfied** | the negative run: one file removed, the step fails naming it; restored, it passes |
| AC-005 | **satisfied** | `publish_readiness::the_description_is_the_sentence_the_reservation_promises`, which cross-reads `xtask/src/reserve.rs:93` |
| AC-006 | **satisfied** | `publish_readiness::the_front_page_says_what_the_crate_is_and_declares_its_docs_rs_build`; the gate's `documentation` step; the nightly `--cfg docsrs` wasm32 build |
| AC-007 | **satisfied** | `cargo test -p happenstance-cloudflare --doc`, proved non-vacuous by a deliberate break |
| AC-008 | **satisfied** | `publish_readiness::no_example_in_the_readme_asserts_on_a_literal_position`, plus the stated review obligation discharged in `implementation-report.md`, *Notes* |
| AC-009 | **satisfied** | `publish_readiness::the_conformance_claim_carries_every_qualifier_the_run_produces`, read against the fixture's constants; diffed against the run's zero `SKIP` lines |
| AC-010 | **BLOCKED** | placeholder generated and dry-run green; the irreversible `cargo publish` is left to the maintainer |
| AC-011 | **satisfied** | `publish_readiness::no_unimplemented_body_or_scoped_allow_survives`; `clippy … -D warnings` with `clippy::todo` denied workspace-wide. Verified, not caused |
| AC-012 | **BLOCKED** | every REQUIRED step green including all seven `wasm32` steps; `cargo deny check bans` red on an accepted open question this story may not settle |

The traced project AC — **AC-012, the crate is publish-ready and the gate is green** — is
therefore **half closed**: publish-*readiness* is done and evidenced; the green-gate half
and the name reservation are open, each behind one named, owned action.

## Knowledge Harvest

Three candidates for `.kb/` at closeout. None is written here — every `.kb/` write in this
project belongs to `adr-0023-and-atom-resolutions`.

1. **The manifest flag and the publishable list are one artefact with two halves.** Deleting
   `publish = false` is a *registration*, not a manifest tidy-up: the crate is either inside
   the step that guarantees its licences ship or outside it, and `reconcile` failing in both
   directions is what makes the pair impossible to half-land. It generalises to any workspace
   whose release set is both derived and declared — the value is that the failure message can
   say *which* of the two moved.
2. **A packaging gate that reads filenames cannot see a crate that lies.** The detector that
   closes the gap is a test target reading the crate's own front matter against artefacts
   that move on their own — fixture constants, manifest keys, the repository's licence bytes
   — rather than against strings the test also writes. Worth stating beside the D11 lesson,
   which stopped one level short of it.
3. **`kb-open-question-worker-async-trait-ban-001` is now forced and still unanswered.** Its
   own text says this story is what forces it; this story has run the full gate, confirmed
   the red is exactly that ban and nothing else, and confirmed the diff adds no dependency.
   The next decision has all the evidence it needs and no remaining reason to wait.
