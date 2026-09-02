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

**Left standing because it is what was run, and corrected below: "conclusive" is the
word that was wrong.** A five-line window answers *did this citation move with the
file*, which is a displacement check. It cannot answer *is the citation pointing at
the thing the sentence names*, and it passed over four citations for which the two
answers differ. The re-run that found them, and the instrument each tree actually
has, are in the section after next.

**The check caught a real false positive, and that is why it was run in report mode
first.** A naive `store\.rs:` pattern also matches the tail of
`projection_store.rs:270-292` and `event_store.rs:62-67` — citations into
`happenstance-ladybug`, `happenstance-postgres`, `happenstance-neon` and
`happenstance-sqlite`. Twenty-one such matches were proposed and rejected by
anchoring the pattern and whitelisting the path; bumping them would have silently
mis-pointed four other crates' citations while every gate stayed green.

## Every changed line, and what changed in it

**51 lines across 10 files, carrying 52 citation tokens.** Masked comparison, and the
scope is the correction — it is run over the whole slice, `af9a241..HEAD`, not over one
commit:

```console
$ # mask every store.rs line number AND every bare `:NNN` continuation in
$ # `git diff af9a241 -- spec/ standards/`, then compare each changed line
$ # against its predecessor
$ lines differing in anything but a line number: 0
```

No clause text, no rule, no `Rejects:` case, no `**Evidence.**` subject and no example
changed. `references/**` also carries `store.rs:NNN` citations and is **deliberately not
repaired**: it is evidence kept for citation, binding nothing, read by no checker, and
its line numbers were already historical. Stated so the omission is a decision rather
than an oversight.

## What the per-commit check could not see

**The first version of this section was run per-commit, and that is why it reported
green over four broken citations.** The story landed in two commits — `10e99b2` bumped
by `+21`/`+47`, `f2c7dbe` re-bumped by `+1` — and a masked comparison of *one commit's*
changed lines cannot see a citation that the first commit bumped wrongly and the second
bumped again from the wrong base. Two independent blind spots, both structural:

* **Per-commit scope.** Only `af9a241..HEAD` compares a citation against the file it was
  written for.
* **The masking pattern.** It matched `store.rs:NNN` and not the bare `` `:NNN` ``
  continuation form, which is how these documents write a second citation into the same
  sentence. Three of the four defects are that form.

Re-run over the whole slice, with the check tightened from *the five lines beginning at
the old target now begin at the new target* to **the cited line IS the first line of the
subject the sentence names**:

| Site | Was | Now | Why the old check passed it |
| --- | --- | --- | --- |
| `E2E-CASES.md:1409` | `store.rs:114` | `store.rs:141` — `#[trait_variant::make(SendEventStore: Send)]` | bumped `+22` where the displacement was `+48`. Old `:92` and new `:114` are both `/// ``` `, and the four lines under each are doc-comment filler, so the five-line window matched and the citation still landed 27 lines from its subject |
| `E2E-CASES.md:1411` | `` `:99` `` | `` `:149` `` | bare continuation — never matched, so never bumped. The same document already cited the same subject at `:1389`, so the file contradicted itself |
| `E2E-CASES.md:351` | `` `:213` `` | `` `:377` `` — `events.last()` | the same blind spot; its sibling `store.rs:260` on `:350` was bumped and this was not |
| `SPECIFICATION.md:5909` | `` `:368` `` | `` `:369` `` — `pub async fn read_decision_model<S>(` | bare continuation, bumped `+47` instead of `+48`. Inside `ANCHOR_SLACK`, so `spec-trace` stayed green; one line off the subject all the same |

A five-consecutive-lines window is a *displacement* check. It is the right instrument for
"did this citation move with the file" and the wrong one for "is it pointing at what the
sentence names", and the two answers differ exactly where a citation was already wrong.

## Which tree was verified by which instrument

The evidence this story files must say this, because the three trees are not equally
guarded and the first draft of the boundary paragraph claimed they were:

| Tree | Instrument | What it actually checks |
| --- | --- | --- |
| `standards/rust/**` — 8 files, 15 citations | `cargo xtask lint-constitution`, a gate step | the citation's **written** `(anchor)` within `ANCHOR_SLACK = 10` lines (`xtask/src/lint_constitution.rs:674`, the constant at `:111`) |
| `spec/SPECIFICATION.md` — 20 changed lines, 21 tokens | `cargo xtask spec-trace`, a gate step | a subject **derived** from the sentence, within `ANCHOR_SLACK = 12` lines (`xtask/src/spec_trace.rs:301`, the constant at `:395`) |
| `spec/E2E-CASES.md` — 16 changed lines | **nothing** | `spec_trace.rs` opens this file at `:617` and `:723` only to resolve the E2E case ids `SPECIFICATION.md` refers to. Its own `store.rs:NNN` citations are read by no checker in this repository |

So every core `store.rs` citation in `E2E-CASES.md` was re-anchored **by hand against its
subject**, and verified by reading the cited line back out of `store.rs` — the only
instrument that exists for it. All 16 now open on the first line of the subject their
sentence names.

## The inherited approximations, repaired rather than preserved

That hand verification found nine more: citations mis-anchored *before* `af9a241`, which
both bump rounds then carried forward faithfully. They are repaired here, for two reasons.
`spec.md` now says these citations must be re-anchored by hand against their subjects, and
that sentence would have been false on the day it was written. And most of them made
`E2E-CASES.md` contradict `SPECIFICATION.md` about the **same** subject:

| Sites in `E2E-CASES.md` | Subject the sentence names | Was, post-bump | Now | `SPECIFICATION.md` |
| --- | --- | --- | --- | --- |
| `:273`, `:925`, `:1269` | `EventStore::append` | `store.rs:196-200` — the `# Cancellation` paragraph | `261-265` | `261-265` |
| `:399` | `append`'s `Option<AppendCondition>` parameter | `store.rs:199` | `264` | — |
| `:116`, `:1497` | `read_decision_model` | `store.rs:246-256` — inside `append`'s doc | `369-379` | `369-379` |
| `:217`, `:1042` | `append`'s `# Atomicity` paragraph | `store.rs:178-181` | `189-192` | `189-192` |
| `:1389` | `type Error: core::error::Error + 'static` | `store.rs:147` | `149` | `149` |
| `:92` | the `read` doc's laziness, ordering and inclusivity | `store.rs:149-169` — opening on `type Error` | `151-171` | — |
| `:149` | `read` applying one `ReadOptions` to the whole `Query` | `store.rs:166-170` | `167-171` | `167-171` |
| `:1384` | the stream at the top level of the return type | `store.rs:152-156` | `153-158` | `153-158` |

Each is a line number and nothing else, so the masked comparison above covers them too.

Spot-check retained and restated as what it is — a hand read of seven subjects, not a
proof of the set: `store.rs:141` (`#[trait_variant::make(SendEventStore: Send)]`), `:149`
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
