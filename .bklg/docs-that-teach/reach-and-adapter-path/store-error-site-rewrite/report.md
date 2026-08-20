---
item: "HS-S0156"
stage: report
created: "2026-08-17T13:16:13.954Z"
updated: "2026-08-17T13:16:13.954Z"
---

# Report — rustc's own E0034 output at the site where it fires

## Findings Ledger

Nine acceptance criteria, all satisfied by real text on a real rendered page, each with a test that
was **red first against the pre-story doc at `ffd0eeb7`**. Nothing is blocked and nothing is deferred.
One criterion's *verification column* diverges from what was delivered, for a mechanical reason
stated in full below rather than smoothed over — read that row before the others.

**Mount point.** `crates/happenstance-core/src/store.rs`, the module `//!` comment at `:1-74`,
mounted as its own rendered page by `crates/happenstance-core/src/lib.rs:126` (`pub mod store;` —
the spec cites `:99`, which was already stale when it was written) and built by the three rustdoc
passes in `xtask/src/main.rs`. Read at
`target/doc/happenstance_core/store/index.html`. The register row that records the one hop it
installs lives at `xtask/src/pointers.rs:250-273`, inside `cargo test` and `clippy -D warnings`
because `xtask/src/lib.rs` declares the module.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — rustc's own transcript, searchable and verbatim | **satisfied** | `store.rs:36-50`. Both `= note:` lines (`:43-44`) and `TraitVariantBlanketType` (`:44`) — on `main` a search for either returns nothing anywhere in `crates/`, which is the gap. `module_doc::the_fence_is_rustcs_own_e0034_output` pins the block character for character with `assert_eq!`; red showed 4 lines against 13. Provenance in `_reproduction.md`: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, the command, the full stderr. |
| **AC-002** — the ambiguity named in words | **satisfied** | `store.rs:52-54` names `store.read(…)`, both flavours and "in scope". `module_doc::the_diagnosis_survives_the_fence_being_deleted` deletes every fence line and still requires both trait names — and requires them *distinctly*, since `SendEventStore` contains `EventStore` and a naive `contains` cannot tell one name from two. |
| **AC-003** — unstuck without a hop | **satisfied** | The escape hatch at `store.rs:58` precedes the pointer at `:66`. `module_doc::the_section_composes_in_the_designs_binding_order` asserts **exactly four** paragraphs after the fence, which is what rejects an insertion between (c) and (d) — the weaker "fix before pointer" test does not. Falsification executed: with `docs/adapter-reading-order.md` renamed away, the section still resolves `E0034`. |
| **AC-004** — the narrow limit, not the broad one | **satisfied** | `store.rs:60-63` names the uncompiled fence *and* the compiled `rust,compile_fail,E0034` negatives at `standards/rust/20-two-flavour-ports.md:179` and `standards/rust/00-prime-directives.md:242` that really do assert the error code. Those two fences are compiled by `cargo test --locked -p xtask --doc`, green inside the affected gate. `module_doc::the_page_never_claims_that_nothing_checks_this` forbids the broad falsehood by name. |
| **AC-005** — one recessive, correctly-formed pointer | **satisfied** | `store.rs:65-67`: last sentence, no heading, link text `the adapter reading order`, named-but-unlinked (rung 3 of the ladder). `module_doc::one_recessive_pointer_closes_the_section` asserts exactly one mention, no `http(s)://`, and no `[` anywhere in the paragraph — so an intra-doc link cannot creep in. The affected gate's rustdoc builds run under `rustdoc::broken_intra_doc_links = "deny"` and are green. EC-002 did not fire: the destination exists and is registered at `xtask/src/narrative.rs:160-161`. |
| **AC-006** — register row P3 with a named guard | **satisfied** | `xtask/src/pointers.rs:250-273`. The guard names two mechanisms and **both were run against the condition they claim**: with the destination renamed away, `cargo xtask narrative` exited 1 with the reason in the message. The story also added `row_p3s_pointer_is_installed_on_the_surface_it_claims` (`:694`), which reads the row's claim back out of the surface and the registration table — closing the "path in the prose is not the path the tree registers" gap the row previously recorded as unguarded. Red first: `carries the destination path 0 times in its documentation`. |
| **AC-007** — clause pin read first, three artefacts, `spec-trace` green | **satisfied** | `_clause-pin.md`, `_reproduction.md`, `_spec-trace.md`. The pin exists and is enumerated (`xtask/src/lint_narrative.rs:1354`, 21 clauses), so EC-001 did not fire; the three entries pinned to this file (ES-19, ES-23, ES-24) are anchored by verbatim phrases at `store.rs:179`, `:194`, `:217`, all outside the edited region. `cargo xtask spec-trace` exits 0 in check mode. The subject review is mechanised, not eyeballed: **all 48 changed citation lines are identical to their predecessors once the line number is masked — 0 differ in anything else.** Repair-vs-gap verdict: **repair**; EC-009 did not fire. |
| **AC-008** — the surface obeys the signed-off design | **satisfied** | Section measures **35** source lines against a cap of **36**; heading ladder unchanged at four entries rendering h2/h3/h3/h3 with no level skipped; no `<details>`, `<div>`, `style=` or "See also" block. Built and read: the module doc sits inside `<details class="toggle top-doc" open>`, so it is **not** collapsed. The fence's longest line measures **109** characters and takes a horizontal scrollbar at both viewports — `_design.md`'s F4 measurement reproduced, expected rather than a defect. |
| **AC-009** — the search keys, by a stated rule | **satisfied, with the verification column's count corrected** | `store.rs:115-140`: the rule, its rejected alternatives, the no-register-row reason, and **two** attributes. The column says three. Three is not writable: `SendEventStore` is derived and `trait-variant-0.1.3/src/variant.rs:115-123` rebuilds it with `..tr.clone()`, copying the trait's attributes, so there is no second item to attach one to. Observed rather than argued — `trait.SendEventStore.html` renders `EventStore`'s doc comment verbatim, and a doc comment *is* a `#[doc]` attribute. Two attributes therefore yield **four** (key, item) pairs in the search index, a strict superset of the design's three. The criterion's THEN — a reader searching `E0034` or `TraitVariantBlanketType` reaches the two traits — is met and over-met. |

### What a reviewer should look at hardest

**The AC-009 count.** It is the one place the delivered work does not match a verification cell, and
the honest reading is that the cell encoded an assumption about the source (two items to write on)
that the macro falsifies. Deliver three and one of them has to be a string rustc never printed, which
breaks the very rule the story was asked to establish. The judgement is recorded in the ledger row,
in the source at `store.rs:128-134`, and here — three places, so it cannot be found only by someone
who already suspected it.

**The citation diff.** 48 lines across 11 files is the largest part of this change by line count and
carries the least thought per line. It is line numbers only, and that claim is checkable in one
command rather than by reading: mask every `store.rs:NNN` and the two sides of the diff are equal.
The method, and the 21 false positives it rejected — `projection_store.rs` and `event_store.rs` match
a naive pattern on their tail — are in `_spec-trace.md`.

**The tests, for whether they could fail.** Two were built specifically so a weaker form could not
pass: the transcript is compared with `assert_eq!` rather than `contains` (it caught the interrupted
session's own fence, which was pasted from a different run and truncated mid suggestion box), and the
composition test counts paragraphs rather than checking an ordering (which is what rejects a
paragraph wedged between the diagnosis and the fix). Nine of seventeen were red on the baseline; the
eight that passed are named in the implementation report as regression guards, not as coverage.

### Boundary and scope

Nothing outside the story's declared paths was touched. The front door
(`crates/happenstance/src/lib.rs`, `crates/happenstance/README.md`) is untouched and the adapter
pointer is deliberately **not** installed there — that page already routes adapter authors to
`happenstance_core`, so a pointer there would contradict it. No public item, signature, feature or
manifest changed; no `SPECIFICATION.md` clause **text** changed; no `xtask` gate step was added.
`references/**` carries `store.rs:NNN` citations and was deliberately left alone: it binds nothing,
no checker reads it, and its numbers were already historical.

### Handed on

- **`error-site-walk-record`** is unblocked. It owns the observed, keyboard-only walk that closes
  initiative DoD-10, and it must be a different pair of hands — nothing in this story reports a walk.
- **ES-26** remains `Excluded` in `FROZEN_DOC_MUSTS` with a reason naming HS-P0023. Its subject is
  `append`'s error asymmetry, not this site; it is a residual with an owner, recorded in the
  implementation report's `## Notes` rather than taken here as an unasked edit inside the pinned
  surface.
- **The `doc(alias)` staleness check** the design offered to HS-P0020 and did not count as a guard now
  partly exists, in the place this story could reach it: `module_doc::every_search_key_is_a_string_rustc_printed_on_this_page`
  requires every key to still appear verbatim in the transcript it came from. What is still missing,
  and is still HS-P0020's, is the check that rustc has not renamed `TraitVariantBlanketType` upstream —
  named as residual risk at `store.rs:136-138` and counted as a guard by nobody.
