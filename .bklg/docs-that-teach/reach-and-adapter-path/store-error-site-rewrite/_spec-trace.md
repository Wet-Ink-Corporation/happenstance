# `cargo xtask spec-trace`, the re-anchoring it forced, and the selector EC-008 asked for

AC-007's **third artefact**. Decision 8 forecast that this insertion would red the
citation gate and that repairing it was in scope; EC-003 said the red was expected.
This records what actually happened, which is the same shape one level finer.

## The result

```console
$ cargo xtask spec-trace
201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE),
112 conformance rules, 58 e2e cases, 401 citations checked
(80 anchored to their subject, 12 external)
traceability: no problems found; §7.1–§7.2 matches the checker
exit 0
```

Green in **check** mode over the resulting tree. `cargo xtask lint-constitution` —
the other reader of line-anchored `store.rs:NNN` citations, and the one
`spec-trace` never opens — reports `27 atoms, all consistent`.

The two lines `spec-trace` also prints, about
`k_disjoint_boundaries_admit_exactly_k_commits` and `ops_agree_with_the_model`
owing a decision, are **pre-existing and not this story's**: both are present on
`main`, neither concerns a documentation citation, and both name the ADR they are
owed.

## The re-anchoring, in two rounds

The insertion is bigger than `ANCHOR_SLACK` (**12**, `xtask/src/spec_trace.rs:391`),
exactly as Decision 8 predicted, so the repair was necessary rather than tidy. It
happened in two rounds because the story was implemented across two sessions.

**Round 1 — `+21` and `+47`, in the interrupted session (`10e99b2`).** The module
doc grew from 52 lines to 73, and the `#[doc(alias)]` rule block added 26 more
above `pub trait EventStore`. So citations landing *inside* the doc region moved by
21 and citations landing below the attributes moved by 47 — two different
constants, applied per citation rather than as one blanket shift. Reviewed here
line by line against `git show 10e99b2 -- spec/ standards/`: every changed line
differs from its predecessor in a line number and in nothing else, and each new
target lands on the same subject the old one named.

**Round 2 — `+1`, this session.** One line was added to the fence: the closing `|`
of rustc's surviving suggestion hunk, which the interrupted session's draft had cut
mid-box. Everything from `store.rs:50` down therefore moved by exactly one.

`cargo xtask spec-trace --write` was run first and **rewrote nothing**, which is
correct rather than a failure: it re-anchors a citation only when the subject has
left the window, and a one-line drift is well inside a slack that is deliberately
twelve wide *precisely* so a growing doc comment cannot red the gate
(`xtask/src/spec_trace.rs:391`). The tool declining is the tool working.

The 49 citations were bumped anyway, because "inside the slack" and "correct" are
different claims and this project is about the second one. **This is arithmetic
over a known edit, not a heuristic over subjects** — one line was inserted at a
known position in a known file — and every bump was verified before it was made:

> the five lines beginning at the old target must now begin at the new target, and
> the old target's own line must have changed.

Five consecutive lines agreeing is conclusive; one is not, because `    ///` occurs
everywhere. That check is what EC-004 asks for in the direction it can be had, and
it is why no line number was hand-edited to silence anything — nothing was failing.

**The check caught a real false positive, and that is why it was run in report mode
first.** A naive `store\.rs:` pattern also matches the tail of
`projection_store.rs:270-292` and `event_store.rs:62-67` — citations into
`happenstance-ladybug`, `happenstance-postgres`, `happenstance-neon` and
`happenstance-sqlite`. Twenty-one such matches were proposed and rejected by
anchoring the pattern and whitelisting the path; bumping them would have silently
mis-pointed four other crates' citations while every gate stayed green.

## Every changed line, and what changed in it

48 lines across 11 files. Proved mechanically rather than asserted:

```console
$ # mask every store.rs line number in the diff and compare each pair
$ lines differing in anything but a store.rs line number: 0
```

No clause text, no rule, no `Rejects:` case, no `**Evidence.**` subject and no
example changed. `references/**` also carries `store.rs:NNN` citations and is
**deliberately not repaired**: it is evidence kept for citation, binding nothing,
read by no checker, and its line numbers were already historical. Stated so the
omission is a decision rather than an oversight.

| File | Citations bumped |
| --- | --- |
| `spec/SPECIFICATION.md` | 20 |
| `spec/E2E-CASES.md` | 14 |
| `standards/rust/40-public-surface-and-evolution.md` | 4 |
| `standards/rust/70-rustdoc-obligations.md` | 4 |
| `standards/rust/20-two-flavour-ports.md` | 2 |
| `standards/rust/21-send-is-not-inherited.md`, `22-rpitit-and-lifetime-capture.md`, `30-error-taxonomy.md`, `51-features-and-no-std.md`, `92-toolchain-limits-and-dead-ends.md` | 1 each |

Spot-checked by hand at the subjects that matter most, each now landing on its own
first line rather than the line above it: `store.rs:141`
(`#[trait_variant::make(SendEventStore: Send)]`), `:149`
(`type Error: core::error::Error + 'static;`), `:167` (`fn read(`), `:261`
(`async fn append(`), `:296` (`async fn head(`), `:333`
(`pub async fn collect<S, T, E>(`), `:369` (`pub async fn read_decision_model<S>(`).

## EC-008 — the selector `_design.md` flagged as unverified

`_design.md` gives `store-module-error-site` the selector `#main-content > .docblock`,
inherited from the verified `happenstance` page and explicitly unconfirmed for
`happenstance_core`, which had not been built at design time. Built now
(`cargo doc -p happenstance-core --no-deps`), it **does not match**. The real one:

```
#main-content > details.toggle.top-doc > div.docblock
```

rustdoc wraps a *module's* top documentation in a `<details class="toggle top-doc">`
that a crate-root page does not use. Recorded here rather than in `_design.md`,
which is signed off and is not edited by an implementing story (EC-008).

**The consequence the design actually cared about is checked and is fine.** That
`<details>` carries the `open` attribute, so the module doc is expanded, not
collapsed — elements (b) and (f) are not hidden by the one renderer-owned collapse
nobody chooses. Verified on the built page:

```html
<details class="toggle top-doc" open><summary class="hideme">…</summary><div class="docblock">
```

## The rendered surface, measured

`target/doc/happenstance_core/store/index.html`, after `cargo doc -p happenstance-core --no-deps`:

| What | Measured |
| --- | --- |
| Fence element | `<pre class="language-text">` — uncompiled, as intended |
| Fence lines | 13, both `= note:` lines present, `TraitVariantBlanketType` present |
| Longest fence line | **109 characters** — the number `_design.md` recorded as F4; it overflows a 936px content box and takes a horizontal scrollbar at both 1440x900 and 1024x768, which is **expected, not a defect** |
| Last fence line | `  |` — the closing bar of the surviving suggestion hunk |
| Heading ladder | `h2 #why-there-are-two-traits`, `h3 #what-that-means-in-practice`, `h3 #import-one-flavour-not-both`, `h3 #naming` — four entries, no fifth, no level skipped |
| Section source lines | **35**, against a cap of 36 and a design projection of ~34 |
