---
item: "HS-S0188"
stage: report
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Report — The worked example surfaced in its own words

## Findings Ledger

**Seven of seven criteria are satisfied.** The example's own explanation is now one file that
two render paths share by construction, and it is one hop from the page whose reader is looking
for it. Nothing is deferred, nothing is stubbed, and no assertion in this story passes on a tree
where the thing it guards is missing — every one of the seven was run red first.

| AC | Result | What proves it | Where it lives |
| --- | --- | --- | --- |
| AC-001 — surfaced, not summarised | **Met** | `overview.md` is the only copy; `main.rs:5` includes it and carries no `//!` line; the stripped block compares **IDENTICAL** to the new file; `cargo doc -p course-subscriptions` under `-D warnings` renders the same text in the same `top-doc` position; `runs.rs` → 10 passed | `examples/course-subscriptions/src/overview.md`, `src/main.rs:1-6` |
| AC-002 — a checked page, not an orphan file | **Met** | One module, one `include_str!` (asserted as a count); `every narrative page is checked — 5 pages, all consistent`. Red first on the missing registration | `xtask/src/narrative.rs:144-151` |
| AC-003 — one answered-need, above everything | **Met** | `orientation`, the first block element under the H1, in HS-P0021's notation; both `reach.rs::exactly_one_answered_need` and HS-P0021's own step green, the latter also carrying RP-10-3's one-per-directory ceiling | `docs/read-the-worked-example.md:3` |
| AC-004 — oriented, and anchored, before departure | **Met** | Two sentences of orientation; the DT-1 anchor cited at a byte offset before the outbound link; **zero** occurrences of the four prior-model phrases, which is stricter than the criterion asks and keeps HS-S0187's AC-005 intact | `docs/read-the-worked-example.md:5-12` |
| AC-005 — the seam named once, and true post-merge | **Met** | One sentence, between the anchor and the link; `happenstance::commit` once; `happenstance_core` never; no sentence of the page appears verbatim in `overview.md` | `docs/read-the-worked-example.md:14-17` |
| AC-006 — the chain holds, and fails by name when it breaks | **Met** | Three hops, every target resolved with `Path::exists`, `main.rs` asserted last, the bridge's anchor heading asserted by its own text. Falsified once on a dangling target and restored | `docs/carry-your-invariant.md:157-158` → `docs/read-the-worked-example.md:19,22` |
| AC-007 — budgets held, nothing hidden, nothing introduced | **Met** | 31-character path, 22-character H1, 24 source lines, longest paragraph 267, zero fences, zero images, zero markers, zero affordances; HS-P0020's `HIDDEN_MARKERS` rejection is the second instrument | `docs/read-the-worked-example.md`; `reach.rs::page_holds_its_budgets` |

**Mount points.** Two, and the story is not done with either one missing.
`xtask/src/narrative.rs:144-151` is what makes `docs/read-the-worked-example.md` a *checked*
page rather than a markdown file under a directory. `examples/course-subscriptions/src/main.rs:5`
is the second render path for `overview.md`'s bytes, and it is what makes "verbatim" a property
of the build rather than of a reviewer diffing prose. The reach itself is mounted at
`docs/carry-your-invariant.md:157-158`, in HS-P0020's single closing-pointer slot.

**The falsification that makes this story's own checks non-decorative.** An unstyled, unreviewed
render satisfies AC-001, AC-002 and AC-006 completely — identical bytes, a compiling module,
resolving links — and is still a bare link with nothing telling the reader what is behind it.
AC-003, AC-004, AC-005 and AC-007 are the rows that fail on that page, and each is a position or
a count assertion rather than a reviewer's impression: the answered-need line must be the first
block element, the orientation must be at most two sentences and must precede the anchor, the
anchor must precede the seam, and the seam must precede the link.

**What a reviewer should look at first.** The seam sentence. It is the one thing on this page no
compiler can check, it is read at the moment the reader is leaving, and the signed-off design's
own version of it is *already false* post-merge — which is the proof that the failure mode is
real rather than hypothetical. The sentence shipped names the seam that exists: the example holds
no `Query` and no `AppendCondition` because `happenstance::commit` derives both. That was
confirmed against the merged file, not against the design's premise.

**Deferred, and why none of it is a gap.**

- **`docs/README.md` gains no row.** The front-door signpost table and every pointer policy are
  HS-P0023's DT-10. This story makes the material reachable *from the bridge*; it does not decide
  where the front-door pointer lives.
- **Stale `main.rs:1-19` / `:1-28` citations across the planning corpus** are a costed
  consequence of the move, routed to the closeout's reference reconciliation. No gate reads them,
  so nothing breaks, and chasing them would mean editing signed-off stage artifacts.
- **A substrate gap is recorded for HS-P0020**: a narrative page cannot include a source file's
  module doc, which is the whole reason `overview.md` exists. If that ever becomes possible, the
  extraction should be dropped — `_design.md:736-741` pre-authorises exactly that.
- **`reach.rs` is deliberately not added to `xtask/src/proof.rs`'s `ARTEFACTS`.** That file is
  outside this story's PR boundary and its list is an expectation set, not an index.
- **Two project ACs are properties of the page *set*** — the fence inventory (this page
  contributes zero fences and zero clause citations) and the answered-need and anchor walk (one
  `orientation` declaration, one DT-1 citation). Both belong to `page-set-assurance`.

**Gate at the checkpoint.** `cargo xtask affected --base main` → `affected gate passed`.
`233 passed; 0 failed; 2 ignored` doctests — unchanged, because this page authors no fence, which
is itself AC-007 — plus `7 passed` in the new `reach` target and `10 passed` in `runs`.
`cargo fmt --all -- --check` green, with the formatter's write pass run last.
