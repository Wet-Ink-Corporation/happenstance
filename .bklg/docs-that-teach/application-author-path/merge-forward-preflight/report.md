---
item: "HS-S0183"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — Merge forward and record the baseline before authoring

## Findings Ledger

The story's outcome as the review gate reads it. Mount point on every row:
`crates/happenstance/src/lib.rs` — the merged crate root, 237 lines, `Tags::empty()` at `:38`
and `:55`. The deliverable is
`.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md`.

| AC | Result | What proves it | Deferred / routed |
| --- | --- | --- | --- |
| AC-001 — the merge landed, is observable, occluded nothing | **met** | `_baseline.md:25-51` § Merge. `git rev-list --parents -n 1 a5c0f30` → three shas; `git merge-base --is-ancestor` exit 0; 237-line `lib.rs` with both `Tags::empty()` sites; `#main-content details.top-doc > div.docblock` located in the real `target/doc/happenstance/index.html`; `git diff --diff-filter=DRM 3fd3866 a5c0f30 -- .bklg/docs-that-teach .redkiln/telemetry/events references/seeds/user-documentation.md` empty. 11/11 assertions green in `check_baseline.py` | The merge's sibling tip was `4327ce3`, not the `3f49ec6` the spec named (EC-004) — recorded, and the reason § Anchors carries commands rather than numbers |
| AC-002 — the gate is green **on the merge commit**, and the baseline reverts in one move | **met** | `_baseline.md:53-82` § Gate. `cargo xtask ci --fast` exit 0; `cargo xtask affected --base main` exit 0; both tied to `a5c0f30` by `git diff --stat a5c0f30 HEAD -- . ':!.bklg' ':!.redkiln'` being empty, so `HEAD`'s source **is** the merge commit's. `git revert -m 1 a5c0f30` recorded as the backout | none. EC-002 did not fire |
| AC-003 — every anchor resolves from the table alone, id-first | **met** | `_baseline.md:84-120` § Anchors, 17 rows. Each row's own command was executed and its printed line matched: ES-25 `:3756`, ES-26 `:3814`, ES-27 `:3855`, VT-30 `:1861`, CF-7 `:7670`, the id-stability sentence `:300`, `const REQUIRED` `xtask/src/main.rs:120`, the `"tests"` step `:158`, and nine more. `cargo xtask spec-trace` exit 0 | Commands are `git grep -nF`, not `rg`: `rg` is not on this machine's `PATH`, and an unrunnable re-derivation is a claim. Recorded at `_baseline.md:91-95` |
| AC-004 — every contradicted design assumption carries a disposition | **met** | `_baseline.md:122-145` § Dispositions, nine rows, each three fields. The four the spec measured are rows 1–4; rows 5–9 were found while measuring and are recorded rather than absorbed. `_design.md` byte-identical (`git diff --stat 3fd3866 HEAD -- …/_design.md` empty) | Routed: the vocabulary-seam premise → `surface-course-subscriptions` (HS-S0188); the vanished `## Status:` heading → `boundary-refusal-encounter` (HS-S0185); the bracket-count invocation dependence → HS-S0185 and the `support` initiative |
| AC-005 — every composition number re-measured; zero teaching content | **met** | `_baseline.md:146-209` § Composition baseline, six invariants read off the render after `cargo doc -p happenstance --no-deps`: brackets **0** (2 under the gate's invocation), hidden lines **2** and neither carrying a forbidden construct, widest line **70** cols, height **35** lines, `##` headings over 22 chars **2 of 4**, answered-need line **absent**. No Rust fence, no answered-need line, no mapping table, no control | **Two budget breaches routed under EC-005** as conditions on `boundary-refusal-encounter`: the merged fence is 35 rendered lines against a 32-line ceiling and 70 columns against 68. Neither number was relaxed and `_design.md` was not edited |

**Nothing is blocked.** No AC is deferred, no `fixme`, no double. The two density breaches are
*findings about the inherited file*, routed to the story whose job is replacing it — not
unmet criteria: AC-005 asks for them to be measured and recorded, and they are.

**The one process discrepancy, recorded not silenced.** `redkiln verify --grain story --item
HS-S0183` returns `[ok] affected-gate`, `[ok] ledger`, `[FAIL] boundary`, `[FAIL] provenance`.
The boundary failure is EC-007 verbatim — ~230 files arrive by the merge's **second parent**,
which a glob cannot express; `git diff --name-only a5c0f30^1 a5c0f30 --
.bklg/docs-that-teach/application-author-path` is empty and the seven files the merge genuinely
resolved are each named in the fence. The provenance failure is sequencing: `links.commits` is
written by `redkiln record-links` after the checkpoint exists, and the CLI is the only writer
of item frontmatter. Neither was answered by widening the fence.

## Acceptance

| AC | Verification status |
| --- | --- |
| AC-001 | satisfied — ledger row cites `_baseline.md:25-51` and the `check_baseline.py` AC-001 block |
| AC-002 | satisfied — ledger row cites `_baseline.md:53-82`, `cargo xtask ci --fast` exit 0, `cargo xtask affected --base main` exit 0 |
| AC-003 | satisfied — ledger row cites `_baseline.md:84-120`, all 17 rows re-derived, `spec-trace` exit 0 |
| AC-004 | satisfied — ledger row cites `_baseline.md:122-145`, nine dispositions, `_design.md` byte-identical |
| AC-005 | satisfied — ledger row cites `_baseline.md:146-209`, all five numbers re-derived by the check |

Tiers 2, 3 and 4 are n/a with the reasons `spec.md` § Tests and CI states; tier 5's reviewer
walk is what this report is written for.

## Knowledge Harvest

Three candidates for `.kb/` at closeout, each earned by something that went wrong here rather
than by a preference:

1. **A record's re-derivation command must be runnable where the record is read.** Seventeen
   anchor rows named `rg -n`, the repository's convention in prose, and not one of them ran:
   `rg` is not on this machine's `PATH`. `git grep -nF` needs only the repository it is
   standing in. The general shape — *an instrument that only works in the author's environment
   is a claim wearing an instrument's clothes* — is the same argument CLAUDE.md already makes
   about decorative conformance rules, applied to documentation.
2. **A measurement read off a render must name the invocation that produced it.** The crate
   root shows zero literal `[bracket]` pairs under `cargo doc -p happenstance --no-deps` and
   **two** under the gate's `documentation` step, which exits 0 with `RUSTDOCFLAGS=-D warnings`
   and both links unresolved. Same tree, same file, two answers. Any anti-pattern phrased as a
   count on a rendered page inherits this and needs its invocation pinned.
3. **A merge commit does not fit a glob, and the fix is not a wider glob.** `redkiln verify`
   cannot distinguish "authored" from "arrived by parentage", so every merge-bearing story will
   fail its boundary check. Widening the fence to `**` would make the check permanently
   meaningless. The mechanism that *does* work is `git diff-tree --cc --name-only <merge>`,
   which reports exactly the files that differ from **both** parents — the mechanical
   definition of a conflict resolution, and the check that caught this story's own fence naming
   six of seven.

**Two of those are now carried, not just harvested.** The slice review raised this story's
EC-001 ordering — the PR-boundary fence completed *after* the conflict resolutions, twice —
as a non-blocking process finding, having verified all seven resolved files independently and
found nothing hidden by the disclosure. It and candidate 3 above are written into
`_baseline.md § Carried forward to HS-P0025` as **P1** and **P2**, each with what would close
it, and `durable-audience-closeout/merge-forward-baseline/spec.md`'s `## Anchors` table now
points at that section — so the next merge-bearing story meets them before it merges rather
than rediscovering them after. A harvest candidate that only exists in a report nobody opens at
the moment it applies is the same defect as an unreachable record.
