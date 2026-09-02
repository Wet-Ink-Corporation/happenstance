---
item: "HS-S0157"
stage: report
created: "2026-08-17T13:16:14.395Z"
updated: "2026-08-17T13:16:14.395Z"
---

# Report — The observed error-site walk

## Findings Ledger

Nine acceptance criteria, all satisfied, each by real text in a real record backed by a re-runnable
check. Nothing is blocked and nothing is deferred. **Two weaknesses are disclosed rather than
discounted, and a reviewer may reasonably reject the story on the first of them** — read AC-002's row
and the two paragraphs under "What a reviewer should push on" before the rest.

**Mount point.** `.bklg/docs-that-teach/reach-and-adapter-path/project.md:359`, a five-line bullet
appended inside `## Companions`, linking the record with the visible link text *the observed
error-site walk record*. That mount is what makes project **Definition-of-done item 4** resolve
against a file instead of a memory, for one of its three walks. The diff is a pure addition: no
frontmatter line changed and no pre-existing line moved, which is the non-occlusion invariant.

**The deliverable.**
`.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md` — an append-only
failure protocol declared before the first entry, then one dated entry for the walk of **2026-08-20**
against tree `d14affd`.

| AC | Result | What proves it |
| --- | --- | --- |
| AC-001 | **satisfied** | The collision was reproduced first-hand, not quoted. Scratch crate built outside the repository at the pinned `1.97.1`; `rustc 1.97.1 (8bab26f4f 2026-07-14)` as printed; the whole stderr pasted un-reflowed at `walk-record.md:152-173` with both `= note:` lines (`:159-160`) and `TraitVariantBlanketType` (`:160`). `rg -cF '= note:'` = 6; `rg -cF 'TraitVariantBlanketType'` = 5. No file under `crates/` added. Re-runnable: the record carries the source, the manifest and the command. |
| AC-002 | **satisfied, with a disclosed weakness** | Walker named and relationship stated at `walk-record.md:36-60`; bounded-claim stanza in the entry body at `:309-315`, not a footnote. The weakness — the walker is the implementation context and both dependency commits carry a `Co-Authored-By: Claude Opus 5` trailer — is stated in the record itself at `:47-60`, in the record's own words, with the explicit sentence that a reviewer who judges it insufficient should reject the entry. |
| AC-003 | **satisfied** | Cold-start protocol at `walk-record.md:62-93`: permitted at t=0, forbidden at t=0, and **two contaminations disclosed** (`:73-88`) with what each bought. Document order checkable from the heading outline alone — protocol `:62` then reproduction `:94` then Observation A `:179` then hop table `:213`. |
| AC-004 | **satisfied** | Exactly one hop row is sourced at `store.rs` (row 2, `:222`) and its destination is the account. Arrival proved at the passage, not the page: answered-need quoted verbatim (`:243`, matching `docs/adapter-reading-order.md:3`), position markers `1 of 6` through `6 of 6` with the arrival passage named as `## 4 of 6 — why there are two flavours`, and the `MemoryEventStore` caveat confirmed **present and in position at entry 1**, naming it the conformance oracle and reference implementation and stating it is not an adapter. |
| AC-005 | **satisfied** | No pointing device used, stated at `:215-217`; both hop rows carry a filled affordance cell naming the keys. Two real search strings with results written down: `TraitVariantBlanketType` returned 20 matches in 8 files of which **9 are in `store.rs`**, against a measured baseline of six hits and **none** in `store.rs`; and rustdoc's `#[doc(alias)]` keys verified present in the generated search index, with the limitation (index read, browser not driven) stated. |
| AC-006 | **satisfied** | `store.rs` unblocks in place, before any hop. Both quotes are the file's own, not paraphrases: `rg -F 'is ambiguous because both'` hits `store.rs:53`, `rg -F 'Import only the one you are binding on'` hits `store.rs:56`. The invariant behind it was falsified deliberately (`:203-208`): with the account hypothetically deleted the fix still works, because it depends on nothing downstream. |
| AC-007 | **satisfied** | The record exists, carries the walk's own ISO date (`## Walk of 2026-08-20`, `:28`; restated as the walk's date at `:302`), and is mounted at `project.md:359` inside `## Companions`. |
| AC-008 | **satisfied** | The failure protocol is declared **before the first entry** (`:7-27`), so it binds whether or not a failure occurred — which is what makes this record capable of failing in the entry where it did not. No hop failed; three observations were recorded as **not routed** with the reason each was already a design decision, and no dependency's file was touched. |
| AC-009 | **satisfied** | Composed prose plus exactly one hop table with a header row and four filled columns (`:219-222`). The raw-HTML and See-also greps return nothing over **both** changed files. Nothing folded. Heading ladder unskipped. Copy fidelity kept over the density budget: the 109-character `= note:` line is left un-reflowed and said to take a horizontal scrollbar. |

### Findings from the walk, and where they went

**None was routed, and that is the finding.** Each of the three was checked against the signed-off
`_design.md` before being written up, and each turned out to be a decision design had already taken.
They are recorded at `walk-record.md:270-298` so a later reader does not re-open them as defects.

1. The installed `text` fence carries one `help:` hunk where rustc 1.97.1 emits two — authorised in
   advance by `_design.md`'s density-budget row: *"Dropping candidate #1's `help:` hunk is yield rule
   (3) already exercised."* A recorded yield, not drift.
2. The pointer is a named path, not an activatable link — `store.rs:65-67` says so ("Named, not
   linked: it is a page, not an item") and register row P3 sanctions that form. The keyboard cost is
   recorded as a cost: a rustdoc reader must retype the path rather than press `Enter`.
3. The `#[doc(alias)]` keys land on the `EventStore` trait, one step from the module doc that carries
   the explanation — which is what `store.rs:116-119` declares them to be: search keys, not pointers.

### What a reviewer should push on

**The walker's independence is the weak joint, and it is weaker than the spec assumed.** The spec
was written for a human walker recruited by a third party; this session cannot spawn an independent
context, so the story's own implementer walked. It authored neither dependency in the sense that
matters — no shared memory, no shared transcript, no access to either story's reasoning — but the
model identity is shared with both commits' co-author trailer. The criterion is the *stated*
relationship, and the record states it in full including the weakness. This is a judgement call and
it is flagged for a human to overrule, not smoothed over.

**A second contamination was incurred mid-story.** Running the authorship cross-check AC-002 itself
requires printed the destination's path before `store.rs` was opened. Disclosed under EC-006 with
what it bought. It is partial, not total — a path, not the page — and its mitigation is checkable:
`store.rs:65-67` names that same path in prose, so the surface under test supplies its own
destination, and Observation A was made before any hop and is a property of `store.rs` alone. The
record is written down as a **weaker record than an uncontaminated walk**, in those words.

**What this evidence is not.** It is evidence that the path exists and carries a non-author. It is
**not** evidence that a stranger finds it. That claim belongs to `comprehension-evidence` (HS-P0024)
and BR-14, and the record refuses it explicitly so this project does not spend a sibling's only
instrument in advance.

### Gates

`cargo xtask affected --base main` **passed** (the configured story-grain gate: fmt, clippy
`-D warnings`, tests, the file-reading lints, `spec-trace`). `cargo fmt --all --check` clean.
`cargo xtask spec-trace` clean — *traceability: no problems found*. `cargo xtask ci --fast` passed —
*all required checks passed*. One pre-existing seed-dependent flake in a constitution doctest
(`standards/rust/12-manual-impls-and-derive-traps.md:39`) was hit once, characterised, confirmed to
pass on re-run, confirmed untouched by this slice, and deliberately **not repaired** — it is outside
this story's PR boundary.

The gates prove only that this story disturbed nothing. Nothing in `cargo xtask ci` verifies
keyboard-only reachability or self-describing link text, which is why the record — read by a human
against its own protocol — is the instrument and the gate is not.
