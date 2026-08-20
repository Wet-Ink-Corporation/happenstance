---
item: "HS-S0156"
stage: implement
created: "2026-08-17T13:16:13.954Z"
updated: "2026-08-17T13:16:13.954Z"
---

# Implementation Report — rustc's own E0034 output at the site where it fires

## TDD Evidence

**What "test first" means here, said before the table so the table is not mistaken for something
weaker.** This story's deliverable is a doc comment, and the spec is explicit that the instrument for
a reading surface is an observed walk that a *different* story owns. That is true of the *reading*.
It is not true of the *text*: whether the fence is rustc's own output, whether the diagnosis survives
the caret being deleted, whether the fix precedes the pointer, whether the section fits its budget —
every one of those is a property of the file, and every one of them was invisible to this workspace
until this change. So `crates/happenstance-core/src/store.rs` now carries
`#[cfg(test)] mod module_doc`, seventeen tests that read the module's own `//!` comment. A doc
comment nothing checks is exactly the rot the section was rewritten to repair.

**The red step was run against a real baseline, not imagined.** The module doc was replaced with its
pre-story content from `ffd0eeb7` — the commit HS-P0023's implementation branched from — with the new
test module appended, and the suite run. Nine of seventeen failed, each on the missing behaviour and
none on a compile or import error; eight passed, and those eight are the invariants the baseline
already held (the heading ladder, the density cap, no raw HTML), which is what makes them regression
guards rather than padding.

| AC | Test | Red — against the pre-story doc at `ffd0eeb7` | Green |
| --- | --- | --- | --- |
| **AC-001** | `module_doc::the_fence_is_rustcs_own_e0034_output` | `left: [4 lines] right: [13 lines]` — the trimmed fence had no `--> `, no candidate notes and no help hunk | `store.rs:36-50` |
| **AC-001** | `module_doc::the_two_note_lines_and_the_internal_type_name_never_yield` | `= note:` count `left: 0 right: 2` | `store.rs:43-44` |
| **AC-001** | `module_doc::the_fence_is_uncompiled_plain_text_a_reader_can_paste` | passed on the baseline — a regression guard against the `rust`-fence temptation | `store.rs:36` |
| **AC-002** | `module_doc::the_ambiguity_is_named_in_words_and_not_only_by_the_caret` | `element (c) must name the method whose call is ambiguous: Import only the one you are binding on…` — the paragraph after the fence *was* the fix | `store.rs:52-54` |
| **AC-002** | `module_doc::the_diagnosis_survives_the_fence_being_deleted` | failed: with the fence removed the prose named one flavour, not both | `store.rs:52-54` |
| **AC-003** | `module_doc::the_section_composes_in_the_designs_binding_order` | paragraph count `left: 1 right: 4` | `store.rs:52-67` |
| **AC-003** | `module_doc::deleting_the_pointer_leaves_the_reader_unstuck` | failed: there was no pointer paragraph to delete | `store.rs:56-58` survives |
| **AC-004** | `module_doc::the_limit_stated_is_the_narrow_one` | panicked reaching for element (e): the section had one paragraph after the fence | `store.rs:60-63` |
| **AC-004** | `module_doc::the_page_never_claims_that_nothing_checks_this` | passed on the baseline — the falsehood was never there and now cannot arrive | — |
| **AC-005** | `module_doc::one_recessive_pointer_closes_the_section` | failed: zero mentions of the destination | `store.rs:65-67` |
| **AC-005** | `module_doc::the_pointer_is_recessive_and_the_section_grows_no_appendix` | passed on the baseline — guards the "See also" block that gets appended later | — |
| **AC-006** | `pointers::tests::row_p3s_pointer_is_installed_on_the_surface_it_claims` | `the surface … carries the destination path 0 times in its documentation` | `xtask/src/pointers.rs:694` |
| **AC-008** | `module_doc::the_heading_ladder_stays_at_four_entries` | passed on the baseline — the guard is against a fifth entry arriving with the insertion | — |
| **AC-008** | `module_doc::no_heading_carries_a_single_sentence_body` | passed on the baseline | — |
| **AC-008** | `module_doc::the_section_stays_inside_its_density_budget` | passed on the baseline at 13 lines; now measures **35** against a cap of **36** | — |
| **AC-008** | `module_doc::every_element_is_persistent_chrome` | passed on the baseline | — |
| **AC-009** | `module_doc::every_search_key_is_a_string_rustc_printed_on_this_page` | `left: [] right: ["#[doc(alias = \"E0034\")]", "#[doc(alias = \"TraitVariantBlanketType\")]"]` | `store.rs:139-140` |
| **AC-009** | `module_doc::the_search_keys_sit_where_trait_variant_copies_them_to_both_flavours` | `left: "/// }" right: "#[doc(alias = \"E0034\")]"` | `store.rs:139-141` |

Green: `cargo test -p happenstance-core --lib module_doc` — **17 passed, 0 failed**.
`cargo test -p xtask --lib pointers` — **20 passed, 0 failed**.

**Two tests deserve their construction explained, because a weaker form of each would have passed on
work that was wrong.**

`the_fence_is_rustcs_own_e0034_output` compares the fence to a pinned `TRANSCRIPT` with `assert_eq!`
rather than with a handful of `contains` calls. Every `contains` form passes on a block somebody has
tidied, and tidying is precisely the defect: the reader's move is to paste their own terminal beside
this one and look for a difference. This is the one place in the workspace where pinning a paragraph
is right, and the test's doc comment says so, because the house rule is the opposite everywhere else.
It earned its keep immediately — it was red against the *interrupted session's* fence too, which was
pasted from a run whose scratch file put the call on line 7 and which ended one line early, mid
suggestion box.

`the_section_composes_in_the_designs_binding_order` asserts the fence is followed by **exactly four**
paragraphs. A test that merely checked "the fix appears before the pointer" is satisfied by a section
with a new paragraph wedged between the diagnosis and the fix — which is anti-pattern 12 and the one
thing `_design.md` calls a hard constraint. Counting is what rejects it.

**Two falsifications were executed rather than asserted.** `docs/adapter-reading-order.md` was
renamed out of the working tree, and (a) the section at `store.rs:52-58` still carries the diagnosis,
the import rule and the fully-qualified escape hatch, so a reader who never follows the pointer is
still unstuck — UX invariant 1; and (b) `cargo xtask narrative` exited 1 with
``xtask/src/narrative.rs — `mod adapter_reading_order` names no page in docs``, which is row P3's
guard firing on the condition it claims to catch. The page was restored and the step re-run: *6
pages, all consistent*.

## Commits

One checkpoint commit, on `initiative/docs-that-teach`, not pushed.

| SHA | Subject |
| --- | ------- |
| *`git log -1 --format=%h --grep "Story: reach-and-adapter-path/store-error-site-rewrite"`* | `feat(reach-and-adapter-path): The store error site rewrite` |

The SHA cell is a lookup rather than a literal, for the reason the slice-mates recorded first: **the
SHA of a commit containing this file cannot be written inside this file** — writing it changes the
tree, which changes the SHA. The commit is identified by its stable
`Story: reach-and-adapter-path/store-error-site-rewrite` trailer; the literal belongs on the item, via
`redkiln record-links HS-S0156 --sha <sha>` before the advance that files this report, which is what
`require_commit_provenance: true` reads.

**One earlier commit is part of this story's history and is deliberately not its checkpoint.**
`10e99b2` is an interrupted session's partial work, committed without a `Story:` trailer precisely so
the resume window would not count the story as done. This session re-entered the story, treated that
work as unreviewed starting material, and everything above was written on top of it.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-core/src/store.rs` | The section `## Import one flavour, not both` (`:31-67`) runs (a) the surviving cause sentence, (b) rustc 1.97.1's own transcript, (c) the ambiguity named in words, (d) the untouched in-place fix, (e) the narrow limit, (f) one recessive pointer — the design's binding order, 35 source lines. The fence was re-pasted from this session's reproduction: line number and the closing `\|` of the suggestion hunk, which the earlier draft had cut mid-box. Two `#[doc(alias)]` attributes at `:139-140` under a stated rule at `:115-138`. A new `#[cfg(test)] mod module_doc` at `:436` with seventeen tests. |
| `xtask/src/pointers.rs` | Row **P3** filed in `POINTER_REGISTER` (`:250-273`). Its guard was rewritten to name two mechanisms instead of one and to drop the sentence recording an unguarded gap, because the gap is now closed. New test `row_p3s_pointer_is_installed_on_the_surface_it_claims` (`:694`) reads the row's claim back out of the surface and out of `narrative.rs`'s registration table. No policy in that file was re-decided. |
| `spec/SPECIFICATION.md` (20), `spec/E2E-CASES.md` (14), nine `standards/rust/*.md` (15) | **Line numbers and nothing else.** 48 changed lines, every one identical to its predecessor once the `store.rs:NNN` number is masked. Detail, method and the false positives the method rejected are in `_spec-trace.md`. |
| `.bklg/…/store-error-site-rewrite/_clause-pin.md`, `_reproduction.md`, `_spec-trace.md` | AC-007's three artefacts. |
| `.bklg/…/store-error-site-rewrite/_ledger.md` | Nine rows flipped with cited evidence. |

Not changed, and each is a decision rather than an oversight: `crates/happenstance/**` (the front
door — `front-door-pointer`'s, and forbidden here), `crates/happenstance-testkit/**`, any adapter
crate, any `SPECIFICATION.md` clause **text**, any `xtask` gate **step**, and `references/**`, whose
`store.rs:NNN` citations bind nothing and were already historical.

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p happenstance-core --lib module_doc` | 17 passed, 0 failed |
| `cargo test -p xtask --lib pointers` | 20 passed, 0 failed |
| `cargo fmt --all -- --check` | clean — run last, after the one clippy fix |
| `cargo clippy -p happenstance-core -p xtask --all-targets --all-features -- -D warnings` | clean (one finding fixed: `clippy::manual_contains`) |
| `cargo xtask spec-trace` | exit 0 — 401 citations checked, *traceability: no problems found* |
| `cargo xtask lints` | green — 27 atoms consistent, 6 pages consistent, 6 pages one need each |
| `cargo xtask narrative` | 6 pages, all consistent |
| `cargo doc -p happenstance-core --no-deps` | built; page read at `target/doc/happenstance_core/store/index.html` |
| **`cargo xtask affected --base main`** — `.redkiln/config.yaml`'s `affected_gate` | **exit 0, `affected gate passed`** — 233 passed, 0 failed, 2 ignored. Run twice: once after green, once after the citation re-anchoring. |

Affected packages: `happenstance-core`, `xtask`, and their dependents — the scope
`cargo xtask affected` computes for itself from the diff, which is why the command takes a base ref
rather than a package filter.

## Notes

**The transcript was re-pasted, and the compiler decided it.** EC-010 says the compiler wins when the
reproduction differs from what the design's mock reproduced. It also settles a smaller question the
spec did not anticipate: the interrupted session's fence carried `--> src/lib.rs:7:19` and ended at
the `+` line. This session's reproduction put the call on line 6 and printed a closing `|`. Rather
than keep a block that matched no recorded run, the fence is now this run's output and `_reproduction.md`
records the run it came from — so AC-001's "diff the rendered fence against that same stderr" is a
check somebody can actually perform. The first run also printed `src\lib.rs` because cargo hands
rustc a Windows path; invoking rustc directly with the path spelled `src/lib.rs` gives the same
diagnostic platform-neutrally, and that is the run the fence came from. The difference is one
character and it is the invoker's, not the compiler's.

**Three attributes were asked for; two are writable, and they are worth four.** `SendEventStore` is
derived, and `trait-variant-0.1.3/src/variant.rs:115-123` rebuilds it with `..tr.clone()`, copying the
trait's attributes — so there is no second item to write a variant-only attribute on. This is
observed rather than reasoned: `trait.SendEventStore.html` renders `EventStore`'s doc comment
verbatim, and a doc comment is a `#[doc]` attribute. Two attributes therefore produce four
(key, item) pairs in the search index, a strict superset of the three the design specified. AC-009's
verification column says "exactly three"; it is a proxy the mechanism falsifies, and the divergence is
written into the ledger row rather than papered over.

**The citation repair went one round further than the tool would.** `spec-trace --write` was run and
rewrote nothing, correctly: a one-line drift sits well inside `ANCHOR_SLACK`, which is twelve
*precisely* so a growing doc comment cannot red the gate. The 49 drifted citations were bumped anyway,
each verified by requiring the five lines at the old target to now begin at the new one — arithmetic
over a known single-line insertion, not a heuristic over subjects, which is the thing
`xtask/src/spec_trace.rs` is built to decline. Running it in report mode first was not caution for its
own sake: a naive `store\.rs:` pattern also matches the tail of `projection_store.rs:270-292` and
`event_store.rs:62-67`, and 21 such citations into four other crates were proposed and rejected before
anything was written.

**The design's selector was wrong for this crate, as it said it might be.** EC-008 fired.
`#main-content > .docblock` does not match `happenstance_core::store`; the real selector is
`#main-content > details.toggle.top-doc > div.docblock`, because rustdoc wraps a *module's* top
documentation in a toggle that a crate-root page does not use. Recorded in `_spec-trace.md` rather
than in the signed-off design. The consequence the design cared about is fine: the toggle carries
`open`, so nothing is collapsed.

**Carried forward, with an owner, and not taken here.** `FROZEN_DOC_MUSTS` marks **ES-26** `Excluded`
with a reason that names this project: *"both halves are documented but nothing says the asymmetry is
deliberate. Anchoring it needs a doc-comment edit, which is HS-P0023's."* Its subject is `append`'s
error asymmetry, not the `E0034` site; no acceptance criterion of this story asks for it, and making
an unasked edit inside the pinned surface is the failure mode the pin exists to prevent. It belongs to
a later story or a project residual.

**The observed walk is not here, and must not be.** DoD-10 closes on `error-site-walk-record`, which
must be a different pair of hands. Nothing in this report claims a walk; the falsifications above are
file-level and are labelled as such.
