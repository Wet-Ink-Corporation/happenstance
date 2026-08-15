---
item: "HS-S0015"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — DT-8's arm discharged — the suite's bar held for the author it was chosen for

## Findings Ledger

**Verdict: eight of eight ACs satisfied. Nothing blocked, nothing deferred, nothing stubbed.**
`cargo xtask ci` green, run whole. Arm **A** (outside-author) was in force, read out of
`_design.md:359-363` and not chosen here.

### AC by AC

| AC | result | what proves it | where it is mounted |
| --- | --- | --- | --- |
| **AC-001** | **Met** | Reviewed diff, plus the commit graph: `546a8fe` carries the arm quoted verbatim from `_design.md:359-363` and the reading discipline, and touches no crate. EC-001 did not fire. | `_extension-surface-gaps.md:15-34` |
| **AC-002** | **Met** | `cargo doc --workspace --all-features` with `-D warnings`; `cargo test --doc -p happenstance-testkit --all-features` (19 passed, 4 ignored, 3 compile-fail). Density measured: 64/70 doc lines, one new `#` of seven, six steps, one code block, a one-line invocation. | `crates/happenstance-testkit/src/lib.rs:175-239` |
| **AC-003** | **Met** | `examples/outside-projection-adapter/tests/outside_projection_conformance.rs` — **16 of 16 rules pass**, inside `cargo xtask ci`'s ordinary `tests` step. The macro expanded in a foreign crate through `__private`. | `examples/outside-projection-adapter/` (workspace member via `Cargo.toml:3`) |
| **AC-004** | **Met** | `.../outside_projection_discrimination.rs` — `commit_is_atomic_with_the_read_model` fails `CheckpointOnlyStore` by name; the trap test and the control beside it. Reached via the published `projection::rules` path. | `examples/outside-projection-adapter/src/lib.rs:368-506` |
| **AC-005** | **Met** | `.../outside_projection_capability_skip.rs:27` asserts the returned `RuleOutcome::Skipped` value with the reason read off the fixture's own const; `:66` proves a declared capability still runs its rule. Never stdout. | `tests/support/mod.rs:50-51` against `src/lib.rs:135` |
| **AC-006** | **Met** | `cargo tree --edges normal` (no testkit node), `cargo tree -e features --depth 1` (one feature: `conformance`), and two verbatim transcripts — `error[E0117]` and `error[E0432]` — both reverted. | `examples/outside-projection-adapter/Cargo.toml` |
| **AC-007** | **Met** | Commit order: `546a8fe` precedes the fixture commit. Nine gaps recorded with cause, answer and preventing change. | `_extension-surface-gaps.md` |
| **AC-008** | **Met** | `cargo xtask ci`: `all checks passed`. `redkiln verify --item HS-S0015 --grain story --base 6ce1cf3`: `boundary` **green** over the amended block. Two gaps fixed and cited (G6, G9); three reported, not absorbed (G1, G2, G5). `Cargo.lock` committed. | `contract.rs:891-905`, `standards/rust/`, `CLAUDE.md`, `CHANGELOG.md`, `spec.md:256-257` |

### The tests that carry it

- `examples/outside-projection-adapter/tests/outside_projection_conformance.rs` — the whole suite,
  one line, sixteen passes.
- `examples/outside-projection-adapter/tests/outside_projection_discrimination.rs` — the named
  failure, its trap and its control.
- `examples/outside-projection-adapter/tests/outside_projection_capability_skip.rs` — the
  `RuleOutcome` value assertions, both directions.
- `cargo test --doc -p happenstance-testkit --all-features` — the crate-doc section compiles.
- `cargo run -p xtask -- lint-constitution` — `27 atoms, all consistent`.

### What this story found, which is its actual output

Nine gaps. Two fixed here, three handed onward, two recorded as standing tensions the page should
*not* change, and two pre-authorised failures that did not occur and are stated as findings anyway.
Plus one finding that is deliberately **not** a `G`: `D1` is an error in the signed-off record this
story reads, not a point where the allowlist was insufficient, and numbering it `G10` would have made
the nine-gap count answer a different question than the one it was asked.

| # | finding | disposition |
| --- | --- | --- |
| **G1** | `crates/happenstance-core/src/projection.rs:3-11` — the first page an adapter author lands on — says *"the conformance suite does not cover it yet"*. False since merge position 6, and the most expensive sentence on the surface: an author who believes it never looks for `projection_store_conformance!` at all. | **Reported → HS-S0016.** Not repairable here: `spec/SPECIFICATION.md:4566-4569` quotes the paragraph verbatim and the specification is outside this story's boundary. Repairing one without the other trades a false sentence for a rotted citation. |
| **G2** | `cargo xtask spec-trace` **existence-checks 290 of its 359 citations** and content-anchors only 69. Measured twice: deleting the phrase the spec quotes from `projection.rs:3-11` kept it green, and inserting 65 lines above `testkit/src/lib.rs:238` — cited by `SPECIFICATION.md:7807`, and already pointing at the wrong line before this story — kept it green too. | **Reported → HS-S0016 / runbook.** The fix already exists in the tree: `lint-constitution`'s ±10-line phrase anchor. |
| **G5** | The mutant-registry exactness discipline — the thing this repository treats as the difference between a suite and a decoration — is **internal-only**. `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` sit over a registry private to the testkit's test binary. `_design.md:388-395`'s obligation is therefore half discharged. | **Reported → HS-S0016.** EC-005 pre-authorised exactly this. Neither forbidden remedy taken: no registration in the testkit's mutation binary, no registry re-implemented in an example crate. |
| **G6** | `RuleOutcome::Skipped`'s field docs named only `Fixture`, not `ProjectionFixture`, and did not say the value is the associated const's own identifier. The sentence that says so lived on a private macro. | **Fixed** — `crates/happenstance-testkit/src/contract.rs:891-905`. No item became `pub`. |
| **G9** | Adding a section to a crate-doc block moved eight constitution citations into it, and `lint-constitution` failed the gate on all eight, each naming the phrase that had moved. | **Fixed** — `+65` across three `standards/rust/` files. The mirror image of G2, and the reason G2 has a known fix. |
| **G3** | `crates/happenstance-testkit/src/lib.rs:52-53` points an author at `fixtures::MemoryFixture`, the first file on this exercise's denylist. | **Recorded, deliberately unchanged.** For a real author that advice is correct; withholding it would be worse documentation. What was genuinely absent is a projection-side equivalent — which this change supplies. |
| **G4** | The new section's one Rust block is compiled trivially by the page's `# macro_rules! ignore` idiom, so the invocation is never type-checked. Measured by breaking the hidden line. | **Recorded.** A typed example needs a store the page cannot name without G3's circularity, plus ~60 lines against a 70-line budget. |
| **G7** | Two pre-authorised failures **did not occur**: `__private` carried the foreign expansion first time (EC-003), and `projection::rules` was already public (AC-005's gap #1). | **Stated as a finding.** No testkit item became `pub`; CF-32's MINOR event does not fire. |
| **G8** | The three reads that went beyond the allowlist, listed in full — one rule body, the port's own `ToyStore` doctest (which was copied from, and is published to be), and the suite's own failure messages. | **Stated.** Ten of the sixteen first-run failures were repaired **from the assertion message alone**, no source read. The strongest positive result in the record. |
| **D1** | `_design.md:379-383`'s cost snippet puts `happenstance-core` with `features = ["conformance"]` under `[dev-dependencies]`, contradicting the argument the same section makes at `:369-375` — the orphan rule forces the `ProjectionProbe` impl into `src/`, and `src/` cannot see a dev-dependency. The `error[E0432]` transcript in the record is that fact measured. | **Reported → HS-S0016 / runbook.** The decision is right and only the snippet is wrong; the diff follows the decision (`examples/outside-projection-adapter/Cargo.toml:20-22`, `[dependencies]`). `_design.md` is signed off and out of boundary by AC-001 and AC-008, so it is **not** edited here — but the next reader copies the snippet, not the paragraph above it. |

### Deferred, blocked, or not claimed

- **Nothing is blocked.** No AC is deferred and no test is skipped or fixme'd.
- **Initiative DoD 9 is explicitly not claimed.** A workspace member depends by path; a stranger
  installing from the registry belongs to `publication-and-positioning`. The limit is written into
  the record rather than implied.
- **Half of `_design.md:388-395` cannot be discharged by anyone outside this workspace** — the
  mutant-registry exactness check. That is G5, and saying so is the deliverable.
- **Two boundary widenings, admitted into the fenced block rather than argued past it**:
  `standards/rust/**` (G9's three atoms, forced by the mount point the spec chose and unavoidable by
  any placement, scoped to line-number re-pointing with the **+8/-8** property asserted) and
  `Cargo.lock` (compelled by EC-007's `--locked` requirement, which the spec's own
  Data-and-migrations section already assumed the block covered). Both follow `aef8990`'s shape, and
  `redkiln verify --item HS-S0015 --grain story --base 6ce1cf3` reports `boundary` **green** over the
  amended block — a widening recorded only in prose is one the gate cannot enforce.

### For the reviewer, in one line

The fixture passing is the cheap half. Read `_extension-surface-gaps.md` — specifically G1, G2 and
G5 — because those are three things nothing already in this tree could have found, and they are what
the story was commissioned to produce.
