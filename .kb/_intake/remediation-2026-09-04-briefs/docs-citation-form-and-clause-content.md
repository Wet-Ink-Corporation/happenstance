# Should a `docs/` citation carry an anchor, and should a page citing a clause be checked against that clause's content?

Decision record: **GATE-03-docs-citation-form**. Brief only — no ADR prose, no
atom, no change to `xtask/`, which this lane does not hold.

From `O-1 + P-5` of `references/evaluation/review-pre-publication-2026-09-03.md`
(`:2142-2166`). The page defect is fixed — `docs/append-conditions.md` now cites
ES-25 and ES-26 where its sentence is about them, cites ES-40 under a heading
that says it is the caveat, and both dead `file:line` citations are repointed.
This brief is about the two checkers that watched all three defects happen and
reported nothing, and it is owed because the repair is what made their blind
spots measurable.

It is the third brief in this directory about one shape — a green that is read
as coverage and is not coverage. Read it beside `citation-anchor-slack.md`,
which measured the same failure one tree over and found 101 of 323 constitution
citations green after drifting.

---

## Why this is owed

Three defects on one page, and one instrument for each of them either declined
the question by design or was pointed at it and said nothing.

**One.** The page's whole answer cited `ES-40` as its authority, and ES-40 says
the reverse of what the sentence claimed. `xtask/src/lint_narrative.rs`'s
`check_citations` read that line on every gate run, resolved `ES-40` against
`spec/SPECIFICATION.md`'s declared ids, found it, and was silent — correctly,
under its own stated limit (`xtask/src/lint_pages.rs:20-23`):

> *"It does not resolve clause ids… A page can cite a real, resolving id and
> restate its content in the paragraph underneath, and nothing mechanical sees
> that either."*

That limit is stated for the **restatement** case. The case that shipped is
worse and is not stated anywhere: a page can cite a real, resolving id and say
the **opposite** of it. A reader following the page's own citation discipline
lands on a clause about vacuous passes over pruned history and either loses
trust in the page or walks away believing the completeness caveat does not
exist.

**Two.** `crates/happenstance-core/src/lib.rs:103` was cited as the declaration
of the private `memory` module. It is `//!   authors running that suite against
their own store; nothing in the runtime` — mid-paragraph of the `conformance`
*feature's* documentation. `mod memory;` is at `:134`.
`citation_ranges_resolve` inside `cargo xtask lints` **read that citation on
every run** and passed it, because the range resolves and its first line is not
blank. That is the whole of what it checks
(`xtask/src/lints.rs`, `citation_resolves`):

> *"That the citation says what the sentence around it claims — only that the
> range it names exists and its first line is not obviously the wrong kind of
> place to send a reader."*

Demonstrated rather than argued. Changing the same citation to
`crates/happenstance-core/src/lib.rs:99999` fails the step by name:

```
docs/append-conditions.md:19 — cites `crates/happenstance-core/src/lib.rs:99999`,
range 99999-99999 is out of bounds (224 line(s) in the target)
xtask failed: 1 citation(s) under examples, docs, .kb whose cited range does not
resolve cleanly (V-6).
```

So the citation was inside the checker's sight the whole time, and the only
thing separating it from a report was that a wrong line number happened to be
smaller than 224.

**Three, and this is the one nobody had written down.** The second citation was
spelled `` `:122` `` — the bare relative form, meaning *the same file as the
citation before it*. `parse_citation` requires a `/` in the path and so does not
recognise it **at all**. That is a documented gap for the leading-citation case
and it was doing more work than the doc comment claims: the shorthand was not
merely wrong, it was outside the one check that could have said so. Spelling it
in full moved the scan from 76 citations to 77, and the same out-of-bounds probe
now fails on it:

```
docs/append-conditions.md:45 — cites `crates/happenstance-core/src/lib.rs:99999`,
range 99999-99999 is out of bounds (224 line(s) in the target)
```

One page, and it fell inside three declines simultaneously.

## What is true today

**The anchored citation form already exists, and is scoped to one tree.**
`xtask/src/lint_constitution.rs` requires every citation in `standards/rust/` as
`path:line (anchor)` and verifies the anchor sits within `ANCHOR_SLACK` of the
stated line. The scope is `standards/rust` and nothing else.
`RP-30-3`'s own `Rejects.` names this failure one tree over, for the
specification rather than for source:

> *"A page citing `spec/SPECIFICATION.md:1462`. It resolves on the day it is
> written and points at an unrelated clause the next time the file grows, and
> the reader who follows it reads the wrong rule with no way to tell — the
> failure mode stable ids exist to remove."*

`docs/` obeys the *clause* half of that rule and has no equivalent for the
`path:line` half, which is what the page was carrying.

**Two more citations went stale in this lane while nobody looked.** Inserting
one section into `docs/carry-your-invariant.md` moved everything below it by 62
lines, and three citations in `.kb/_intake/remediation-2026-09-04-briefs/after-opt-scope.md`
pointed into that region. They were found by hand and repointed by anchor. They
could not have been found by the checker: `.kb/_intake/` is excluded from
`CITATION_SCAN_DIRS`, deliberately, because it is staging that
`/redkiln:kb-ingest` clears. **A brief accumulates drift while it waits, and is
ingested carrying it.**

**The counts, so an option can be priced.** `cargo xtask lints` reports
77 `path:line` citations across 121 files in `examples`, `docs`, `.kb` after
this lane's repairs. That is the population any anchoring option pays for
once. It is a quarter of `standards/rust/`'s 323.

## Options

### Option A — Extend the anchored form over `docs/` and `examples/`

The narrow, precedented move: `citation_ranges_resolve` starts requiring
`` `path:START-END (anchor)` `` and checks the anchor sits at the cited line,
exactly as `lint_constitution` does for `standards/rust/`.

Costs one pass over 77 citations to add anchors. It is the same instrument the
repository already runs, on a corpus a quarter the size of the one it already
runs it on, so there is no new machinery and no new failure message class.

Two things against it. It inherits `ANCHOR_SLACK`, and
`citation-anchor-slack.md` measured that tolerance and found 31% of the
constitution's citations green only because of it — so Option A extends a check
whose calibration is itself an open question, and would be extending it to a
second corpus before the first is settled. And `docs/` citations are read by
*readers* rather than by a compiler; an anchor is machine ballast in prose a
person reads, which is a real cost `standards/rust/` pays willingly because its
audience is contributors and `docs/` may not.

### Option B — Require nothing new, and check the sentence instead

Leave the citation form alone and add the check neither module has: that a page
citing a clause id does not **contradict** that clause. There is no mechanical
form of this, so in practice it means a reviewer's procedure with a name and a
place — `standards/pages/40-reviewing-a-page.md`'s non-author walk, extended
with a step that opens each cited clause.

It is the only option that would have caught defect one, which is the most
serious of the three: a dead line number sends a reader somewhere useless, and a
contradicted clause sends them somewhere wrong while looking authoritative.

Against it: it is a human procedure, this repository has three of them already,
and `xtask/src/lint_pages.rs:7-11` is explicit that the reviewer's walk is the
instrument for exactly this and *"never a byte count"*. Adding a fourth step to
a walk nobody is scheduled to run is how the walk stops being run.

### Option C — Anchors on `docs/`/`examples/`, and extend the scan into `.kb/_intake/`

Option A plus closing the staging blind spot the three repointed citations fell
into. `.kb/_intake/` is excluded because it is transient, and the exclusion's
own doc comment argues that scanning it would hold the lint *"to citations
nobody is keeping in step with the tree, which is a different failure from
V-6's: noisy rather than blind"*.

The counter-evidence is now in the tree: this directory holds 23 briefs that
have been staged for a day and are cited by each other, and three of them went
stale in one afternoon under a single-section insert. Staging that lasts is not
staging.

Against it: it makes every lane that moves a line responsible for every brief
any other lane wrote, which is a coupling between concurrent worktrees that will
turn the gate red on merges rather than on defects. That is a real cost and it
lands on exactly the workflow this remediation is running.

### Option D — Do nothing mechanical, and record the three declines where a reader meets them

`xtask/src/lint_pages.rs` and `xtask/src/lint_narrative.rs` both open with a
*What this does not verify* section, and both are excellent. Neither says a page
can cite a clause that says the opposite. Adding that sentence to the two limit
lists, and a sentence to `citation_resolves`'s about the bare `:NNN` form, costs
three lines and changes nothing about what runs.

Its whole content is that it makes the blind spot legible to the next reader
instead of to the next audit. Its whole weakness is that this is the *third*
brief in this directory about a green that is not coverage, so legibility has
now been tried.

## Recommendation

**Option A, then D, and not C** — with medium confidence, and with the sequencing
mattering more than the choice, which is the same shape `citation-anchor-slack.md`
reached about its own corpus.

A first because it is precedented, mechanical, and priced: 77 citations, one
instrument that already exists, one corpus that already runs it. D second and
cheaply, because the contradiction case is genuinely not mechanical and the
honest thing is to say so in the module that would have to catch it.

**C is declined on the merge-coupling cost**, and the decline should be
re-examined the first time a brief is ingested carrying drift — which is
observable, because `kb-ingest` reads these files and a stale `path:line` in an
ingested atom *is* inside `CITATION_SCAN_DIRS`. The gap closes itself at the
moment it starts to matter, which is the argument for not paying for it now.

**B is declined as a substitute and kept as a companion.** It is the only option
that reaches the worst defect, and it is not an option that can be *taken* — a
reviewer's walk is not something a brief can install.

### The strongest argument against, in its own words

*Option A fixes the least serious of the three defects and charges 77 edits for
it. The dead line number was embarrassing and cost a reader one confused
scroll; the wrong clause was the one that could have made somebody believe a
guarantee this library does not offer, and A does nothing about it whatever. It
also extends a tolerance that a sibling brief in this same directory has just
demonstrated is miscalibrated — so the recommendation is to spend real effort
propagating a check whose own calibration is an open question, in order to catch
the cheap half of a finding. Do D, do nothing else, and settle `ANCHOR_SLACK`
first.*

The objection is right about the severity ordering and right that `ANCHOR_SLACK`
should be settled first — which is a **sequencing** correction to Option A, and
this brief takes it: A should not land before `citation-anchor-slack.md` does.

Where it is wrong is the claim that A catches only the cheap half. An anchor is
not only a drift detector; it is a statement of *what the author meant to point
at*, and the reason this page's citation survived two months is that nobody
reading it could tell what `:103` was supposed to be. The wrong-clause defect
survived for the same reason in a different medium — `(ES-40)` was a bare
parenthetical with no visible link, so no reader following the page's prose ever
opened the clause. Both are the same failure: a citation that costs a reader
something to verify does not get verified.

## Cost of delay

**Low and slowly compounding**, with the same asymmetry `citation-anchor-slack.md`
names. Nothing in `docs/` is published to crates.io, so no consumer is misled
today; what erodes is the reader's warrant for trusting a green run, and it
erodes whether or not the decision is taken.

One thing that is *not* slow: this remediation is running five concurrent
worktrees over one tree, and every one of them moves lines. The three citations
this lane repointed were found because a hard constraint told it to look. There
is no reason to think the other lanes' inserts were all above every citation
into their files.

## What this does not settle

- **`ANCHOR_SLACK`'s value**, for either corpus. That is
  `citation-anchor-slack.md`'s question and this brief defers to it entirely —
  Option A should adopt whatever that record settles rather than pick a second
  number.
- **Whether `spec_trace`'s own `ANCHOR_SLACK = 12` reconciles with either.**
  Three instruments, three tolerances, one undocumented difference; the sibling
  brief names it and neither of us prices it.
- **Whether a `docs/` citation should carry a line number at all.** `RP-30-3`
  says a *clause* citation must be a stable id and never a line, and every
  argument it makes applies to a source citation too — an anchor is the closest
  thing source has to a stable id. Nothing above asks whether `docs/` should
  cite `crates/happenstance-core/src/lib.rs (mod memory)` and drop the number.
- **What catches a page that cites a clause and contradicts it.** Option B names
  a procedure; nothing here makes it mechanical, and the honest reading of
  `lint_pages.rs:7-11` is that nothing can.

## Provenance

Author: the rendered-pages remediation lane, 2026-09-04, in the same session as
the page repair it describes, and **without** the author → two-critic → revision
pass the original thirteen briefs in this directory had. Nobody independent
argued the other side; the strongest objection is stated and answered above,
which is the form, and this directory's `README.md` applies a discount to the
later briefs that applies to this one.

The two failing-checker transcripts are reproducible: temporarily rewrite the
cited line number in `docs/append-conditions.md` to `99999` and run
`cargo xtask lints`. The 76 → 77 count change is from that step's own `V-6` row
before and after `ed08f6e`.

---

## Measured after the fact — the decline's own trigger has already fired

**Added by the wave integrator, 2026-09-04, after five lanes ran concurrently.**
This section adds a measurement and takes no decision; the recommendation above
stands as its author wrote it.

Option C is declined above *"on the merge-coupling cost"*, with the decline to be
**"re-examined the first time a brief is ingested carrying drift"**, on the
argument that *"the gap closes itself at the moment it starts to matter."*

The gap has not closed, and it started to matter earlier than that sentence
allows. Measured across the whole of `.kb/_intake/` at `bd11598`, against the
line content each citation named at `9b06836` — the only honest test available
for a citation form that carries no anchor to re-find itself by:

| | count |
|---|---:|
| bare `path:line` citations resolvable at both commits | **699** |
| whose cited line's **content changed** | **77** |
| of those, into `references/` (pinned evidence — must **not** be repointed) | 20 |
| **into live files** | **57** |
| of the 57, mechanically recoverable (the old text is findable elsewhere in the file) | 25 |

Spread over **sixteen** briefs. The heaviest are `event-metadata-floor.md` (13),
`after-opt-scope.md` (12) and `adapter-driver-reexport-policy.md` (9); the
heaviest targets are `happenstance-sqlite/src/event_store.rs` (19),
`spec/SPECIFICATION.md` (16) and `happenstance-cloudflare/src/event_store.rs` (8).

**The argument this refutes is narrow and worth stating precisely.** It is not
that the drift is large — 57 of 699 is under 9%, and drift of that order is what
`citation-anchor-slack.md` measures everywhere else in this repository. It is the
*timing* claim. "The gap closes itself at the moment it starts to matter" assumes
the moment that matters is **ingest**, when a brief's citations land inside
`CITATION_SCAN_DIRS`. But a decision brief's citations are load-bearing at a
strictly earlier moment: **when the owner reads it to ratify the decision.** That
is the one moment the brief exists for, and it is the one moment no checker
covers. Twenty-four findings currently sit unratified behind these briefs.

Two qualifications, so this does not read as stronger than it is. **A drifted
citation is not necessarily a wrong one** — 25 of the 57 have their original text
intact elsewhere in the same file, which is drift rather than falsehood, and some
of the remaining 32 name text that was legitimately rewritten by the very work
the brief describes. And **the merge-coupling cost the decline names is real**:
this measurement exists precisely *because* five worktrees moved lines under one
staging directory at once, which is the condition the decline was protecting
against, not a refutation of it.

What the measurement does establish is that the re-examination the decline itself
schedules is owed **now** rather than at first ingest, and that whatever is
decided, the 57 want repointing before anyone reads these briefs to ratify them.
That repointing is an integration task and is recorded as one; it is deliberately
not done per-lane, because a lane repointing a citation into a file another lane
is still moving produces the same drift one commit later.
