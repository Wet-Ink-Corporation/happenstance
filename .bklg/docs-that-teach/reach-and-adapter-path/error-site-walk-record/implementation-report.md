---
item: "HS-S0157"
stage: implement
created: "2026-08-17T13:16:14.395Z"
updated: "2026-08-17T13:16:14.395Z"
---

# Implementation Report — The observed error-site walk

## TDD Evidence

**What "test first" means here, and what it cannot mean.** This story's spec is explicit that it
adds zero gate time — no code, no test, no CI step (`spec.md`, NF-003) — and that nothing in
`cargo xtask ci` can observe a walk (`_design.md`, `## Density budget`, gap 2). There is therefore no
unit test to write, and claiming one would be the decorative check `CLAUDE.md` forbids. The
falsifiable instrument the spec *does* name is the mechanical-assertion tier of its
`## Tests and CI` table, and that tier was run **red before the record was authored and green
after**. The honest limitation is stated rather than glossed: these assertions check that the
evidence is present and correctly shaped, not that the walk happened — only the Observation tier,
read by a human, can falsify the path itself, which is why the record and not a test is the
deliverable.

| AC | The check (from the spec's mechanical tier) | Red, before | Green, after |
| --- | --- | --- | --- |
| AC-007 | `test -f .../error-site-walk-record/walk-record.md` | `ABSENT` — asserting the missing record, not a typo | `PASS` |
| AC-007 | `rg -n 'error-site-walk-record/walk-record.md' .../project.md` | exit 1, no hit — the mount did not exist | hits `project.md:359`, inside `## Companions` |
| AC-001 | `rg -cF 'TraitVariantBlanketType' walk-record.md` | no such file | `5` |
| AC-001 | `rg -cF '= note:' walk-record.md` (spec asks ≥ 2) | no such file | `6` |
| AC-001 | no file added under `crates/` | — | `git status --porcelain` shows only `.bklg/` paths |
| AC-006 | `rg -n -F 'is ambiguous because both' crates/happenstance-core/src/store.rs` | — | hits `:53`, proving the record's quote is the file's own |
| AC-006 | `rg -n -F 'Import only the one you are binding on' crates/.../store.rs` | — | hits `:56` |
| AC-009 | `rg -n '<details\|<table\|<div\|style='` over both changed files | — | no matches in either |
| AC-009 | `rg -in '^#+ *(see also\|next steps\|further reading)'` over both | — | no matches in either |
| AC-003, AC-006 | heading-order check: protocol → reproduction → Observation A → hop table | — | `:62` → `:94` → `:179` → `:213` |

**The one check a machine can settle outright** is AC-001's reproduction, and it was settled by
running it, not by pasting it. A scratch crate was created outside the repository with a path
dependency on this worktree's `happenstance-core` and its own `[workspace]` table; `cargo build` at
the pinned `1.97.1` toolchain produced `error[E0034]` with both `= note:` candidate lines and
`TraitVariantBlanketType`. That stderr is what the record carries. It was **not** quoted from
`store.rs` — and the proof that it was not is finding 1 below, where the walker's own output and the
installed fence measurably differ.

**Order of work was the instrument, and it was preserved.** The reproduction was run first; then
`store.rs` was opened cold and Observation A written; then the pointer was followed; then the
arrival was checked; and only *then* were `_design.md`'s composition tables, the density budget and
`project.md` opened, to compare the observations against what design had decided. Reversing that
order would have produced a record that verified its own expectations.

## Commits

One checkpoint commit, on `initiative/docs-that-teach`, not pushed.

| SHA | Subject |
| --- | ------- |
| *`git log -1 --format=%h --grep "Story: reach-and-adapter-path/error-site-walk-record"`* | `feat(reach-and-adapter-path): The error site walk record` |

The SHA cell is a lookup rather than a literal, following the convention the slice-mates established:
**the SHA of a commit containing this file cannot be written inside this file** — writing it changes
the tree, which changes the SHA. The commit is identified by its stable
`Story: reach-and-adapter-path/error-site-walk-record` trailer.

## Changes

| File | Shape of the change |
| --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md` | **New.** The deliverable: an append-only failure protocol declared before the first entry, then one dated entry — walker and relationship, cold-start protocol with two disclosed contaminations, the reproduction stanza, Observation A, the two-row hop table, the arrival check, three recorded-but-not-routed findings, and the bounded-claim stanza. |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | **One five-line bullet appended** to `## Companions` (`:359`). Pure addition: no frontmatter line changed, no pre-existing line moved. This is the mount. |
| `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/_ledger.md` | Nine rows flipped `false` → `true`, each with a `file:line` citation into `walk-record.md` and the command that re-checks it. No criterion re-worded. |
| `.bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/implementation-report.md`, `report.md` | **New.** This report and the findings ledger. |

**No file under `crates/`, `docs/`, `spec/`, `standards/`, `xtask/` or `.kb/` was touched**, which is
the PR boundary and also the thing that keeps the observation valid: a witness who repairs what they
are witnessing has destroyed it.

## Gates

| Gate | Result |
| --- | --- |
| `cargo xtask affected --base main` — the configured story-grain gate (`.redkiln/config.yaml:40`), which derives the affected package set itself and runs fmt, clippy `-D warnings`, tests, the five file-reading lints and `spec-trace` | **passed** (`affected gate passed`) |
| `cargo fmt --all --check` | **clean**, exit 0 |
| `cargo xtask spec-trace` — project DoD item 2 | **clean** — `traceability: no problems found; §7.1–§7.2 matches the checker`, 201 clauses and 401 citations checked |
| `cargo xtask ci --fast` — project DoD item 1 | **passed** — `all required checks passed (--fast: 4 optional step(s) not run)`, including `6 pages, all consistent` and `6 pages, 16 rules, all consistent`, which is the registration guard the walk's hop depends on |

**One gate failure was hit, investigated under EC-007, and is not this story's.** The first
`cargo xtask affected` run failed a constitution doctest at
`standards/rust/12-manual-impls-and-derive-traps.md:39`, with `left: Some("decode")` against
`right: None`. It is a **seed-dependent flake, not a regression**: the example is the deliberate
"Not" case, asserting that a `HashMap` lookup *misses* because `derive(Hash)` spans `arity` and
`name` while `Borrow<str>` yields only `name`. With one entry in the map, hashbrown still probes by
the h2 tag byte, so on roughly 1 run in 128 the tag collides, equality then succeeds, and the lookup
finds the entry the example claims is unreachable. Re-running the same doctest passed. The file was
last touched in `30dcb2d`, long before this slice, and this story changes no Rust at all, so it
cannot be the cause. **Not repaired here** — it is outside this story's PR boundary and belongs to
whoever owns the constitution corpus; it is surfaced rather than silently re-run.

## Notes

**The walker is the implementation context, and that is a real weakening of AC-002's separation.**
The spec's shape assumes a human walker recruited by someone else. This session has no facility for
spawning an independent context, so the implementer walked. It authored neither dependency — no
shared memory, no shared transcript, no access to either story's reasoning — but both dependency
commits carry a `Co-Authored-By: Claude Opus 5` trailer, so the mechanical authorship cross-check
surfaces the walker's model name on the work it witnessed. The spec's own risk table anticipates
exactly this ("`git log` attribution is fragile under squashes, co-authored trailers and pair work…
the criterion is the walker's own stated relationship to the work"), and the record states the
relationship and the weakness in full at `walk-record.md:47-60` rather than discounting it. **A
reviewer who judges this insufficient should reject the entry**, and the record says so in those
words. `EC-004` was weighed and not taken, because the walker did not author either dependency in
the sense the criterion is about.

**A second contamination was incurred and is disclosed, not hidden.** Running the authorship
cross-check AC-002 itself requires (`git show --stat af9a241`) printed that commit's file list, which
names `docs/adapter-reading-order.md` — so the walker knew the destination's path before opening
`store.rs`. Per EC-006 it is disclosed in the walker's own words with what it bought. It is not the
total contamination EC-006 voids an entry for: the walker was shown a path, not the page. The
mitigation is checkable by anyone — `store.rs:65-67` names that exact path in prose, so the surface
under test supplies the destination itself, and Observation A, made before any hop, is a property of
`store.rs` alone.

**Three findings were noticed and none was routed, which is a deviation worth naming.** The instinct
on each was to file it against a dependency; each was instead checked against `_design.md` first and
each turned out to be a decision design had already taken and written down — the dropped `help:`
hunk is an authorised density yield, the named-unlinked pointer is register row P3's sanctioned form,
and the `#[doc(alias)]` keys are declared search keys rather than pointers. They are recorded anyway
(`walk-record.md:270-298`), because a later reader who notices them and cannot find that paragraph
will re-open them as defects.

**The walk succeeded cleanly and was not enriched.** The spec warns against adding what the walker
"would have" hit. The entry is a transcript of one event: two hops, one recorded search attempt, and
the three observations above.
