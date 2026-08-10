# Phase 4–5 reconciliation

**Lens:** after phases 4 and 5 rewrote the contract, does `SPECIFICATION.md` still
describe the tree — and does anything in the gate notice when it stops?
**Date:** 2026-08-10 · **Repo:** `D:\repos\happenstance`, working tree at
`84dcc67` — the last of six commits, `3c704d3` through `84dcc67`, on `3712c9b`
· **Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)`

---

## What this is

The evidence behind [`RUNBOOK.md:3777-3842`](../RUNBOOK.md), *Between 5 and 6 —
the reconciliation nothing owned*. That section is the one-paragraph verdict; this
is what was measured to reach it, and it is **immutable evidence** under
[this directory's rules](README.md): dated, pinned, cited by `file:line` from
elsewhere, superseded rather than edited.

It records what was found and when. It decides nothing, schedules nothing and
recommends nothing — seven findings needed an ADR and are recorded below without
one, which is the same posture
[`phase-4-reconciliation.md`](phase-4-reconciliation.md) took and for the same
reason.

Unlike the original fourteen, it needs no rename substitution: it was written
after `7d6c1b0` and uses today's crate names.

Two conventions, both load-bearing for a document about citations.

**Every number below was re-derived against the working tree on the date in the
header**, either by running `cargo xtask spec-trace` or by reading the cited line.
Where a figure comes from the pass's own record rather than from a re-measurement,
the sentence says so and names where the record sits.

**Every `file:line` here is the working tree's on that date.** Line numbers in
`crates/**` and `xtask/**` moved several times during the pass itself — that is
one of its findings — so a reader following one of these into a later tree should
expect the item, not the number.

---

## What was audited, and how

The corpus is `SPECIFICATION.md`: 9,070 lines, 200 numbered clauses, and every
`file:line` citation in them. Three instruments, in this order.

1. **Two parallel citation audits.** The same corpus, split and worked
   independently, filing one row per citation: what the sentence attributes to the
   location, what is actually there, and whether the two are the same thing. The
   pass records 326 rows between them. The corpus size they worked against is
   recorded in the checker's own docs — "the document carries 338 citations and
   only 84 satisfy both" (`xtask/src/spec_trace.rs:2189-2191`).
2. **Adversarial refutation.** Every row re-checked by a reader whose job was to
   overturn it. The pass records that the refuter overturned rows in **both**
   audits, and two further claims in a later wave. That is the reason for
   instrument 3: an audit that cannot be shown to have produced a false positive
   has not been calibrated, and one that produced several has.
3. **A mechanical check, written afterwards, over the whole corpus.** The audits
   were people reading; the anchor check in `check_citations`
   (`xtask/src/spec_trace.rs:297-372`) is the part that survives the pass. It is
   what makes the next occurrence of this defect a red gate rather than a reading
   exercise.

Today's run of the third instrument, verbatim:

```console
$ cargo xtask spec-trace
200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE),
95 conformance rules, 58 e2e cases,
358 citations checked (69 anchored to their subject, 12 external)
```

§1.3's hand-written census (`SPECIFICATION.md:219-222`) states the same five
clause counts — 200 IDs; 139, 49, 10 and two — and check 8 compares the two on
every run.

---

## What was found, by class

### 1. Citations that resolve, pass the gate, and point somewhere else

The largest class. `RUNBOOK.md:3833-3835` records the count: of the citations
audited, **128 pointed at a different item than the sentence citing them
claimed**, and roughly 130 were re-anchored.

None of them was a broken link. Every one resolved to a file that exists at a line
that exists, which is the whole of what `check_citations` verified before this
pass. The failure mode is insertion above a cited line: a doc comment grows, an
item moves, and the citation now lands in a neighbour's body while still
resolving.

The sharpest example is one the sibling document happened to measure at
`3712c9b`, before any of these commits:
[`review-citation-drift.md:36`](review-citation-drift.md) records
`SPECIFICATION.md:2460` citing `memory.rs:364-391` for
`send_flavour_stream_is_send_in_generic_code`, a test that was by then at
`memory.rs:614`. The cited span landed 250 lines above it, inside
`MemoryEventStore::append`'s body — where a reader finds a write lock and a
condition check and could plausibly take *that* for the evidence for a clause
about a stream's `Send`ness. That is the whole class in one row: green, resolving,
and pointing at something that reads like an answer.

### 2. Clauses whose prose describes an implementation that no longer exists

Sixteen. The MUST was untouched in every case; the supporting paragraph was
written against phase-3 code. Three, re-verified against the tree today:

| The clause said | The tree says |
|---|---|
| `AppendCondition` has public fields | `guards` is private (`crates/happenstance-core/src/append.rs:118`) behind an accessor (`:181`) |
| `ReadOptions` carries no upper bound | `ReadOptions::to` shipped at phase 4 (`crates/happenstance-core/src/query.rs:327`) |
| `SequencePosition::next` is `saturating_add` | `checked_add` (`crates/happenstance-core/src/event.rs:278`), which is what lets it signal overflow at all |

The third is the one worth keeping in mind, because a `[FROZEN]` clause — VT-13 —
requires `next()` to return `None` on overflow, and `saturating_add` cannot. The
clause's own prose was quoting the code that falsified it.

### 3. Documentation MUSTs that `[FROZEN]` clauses impose and the code never met

Nine. These are not prose defects: a clause of the form "the port's documentation
MUST state X" is discharged by a doc comment or it is not discharged. Eight
re-verified at their discharge sites today:

| Clause | Obligation | Discharged at |
|---|---|---|
| ES-23 | an explicit `# Cancellation` section on `append` | `crates/happenstance-core/src/store.rs:146-165` |
| ES-24 | at-most-once under verbatim reissue, **with all three limits** | `store.rs:167-193` (the limits at `:180-193`) |
| VT-15 | equality is byte equality and nothing is normalised, NFC/NFD named | `crates/happenstance-core/src/tag.rs:29-47` |
| VT-17 | repeated keys are legal, stated where a reader meets `Tags` | `tag.rs:255-260` |
| VT-3 / ES-17 | `into_parts` is not a clone-avoidance route for adapters | `crates/happenstance-core/src/event.rs:404-418` |
| ES-40 | a condition is a claim about one store's log, not about the world | `crates/happenstance-core/src/append.rs:29-47` |
| — | a code comment reading "ES-6 is deferred" | `crates/happenstance-core/src/memory.rs:682-689`, which now states ADR-0009's settlement |
| — | a comment resting on a premise phase 4 falsified | `crates/happenstance-sync/src/ingest.rs:38-60`, which now records the move rather than the original obstruction |

ES-19's correction is the ninth and is recorded by the pass rather than
re-verified here.

The last two are a different animal from the first six: not an undischarged
obligation but a comment asserting something false about the specification. They
are counted together because the repair is the same and the reader's exposure is
the same.

### 4. Tests that clauses name and the tree did not contain

Five. Each was observed **red** against a named wrong implementation before being
made green, per CLAUDE.md's decorative-rule bar. Three of the five are in two new
test targets and carry the rejected implementation in their own doc comments:

- `event_new_accepts_a_held_event_type` and
  `command_handler_composes_validation_errors`, both VT-18, in
  `crates/happenstance-core/tests/constructor_ergonomics.rs:62` and `:117`. The
  first rejects the equality-constrained bound `TryInto<EventType, Error =
  InvalidEventType>`, which excludes the one conversion that cannot fail
  (`error[E0271]`); the second rejects an `InvalidQuery` carrying no
  `Tag(#[from] InvalidTag)` variant, which breaks the bare `?` in every downstream
  command handler.
- `append_does_not_accept_a_foreign_identity`, VT-10, in
  `crates/happenstance-testkit/tests/foreign_identity.rs:51`. It lives in the
  testkit because the vantage is a replicating peer's, which is downstream of the
  contract crate, and the rejected shape is an `append` taking
  `&[(Event, Option<EventId>)]`.

The remaining two are `query_items_is_not_constructible_downstream`
(`crates/happenstance-testkit/src/lib.rs:263`) and
`provided_method_future_is_send_in_generic_code`
(`crates/happenstance-core/src/memory.rs:777`).

The first is a `#[cfg(doctest)]` module rather than a `#[test]`, and it is in the
testkit rather than the contract crate because the property it asserts —
`error[E0639]` on `Query::Items { .. }` — is only observable from *downstream* of
`happenstance-core`. Writing it turned up a falsehood in `query.rs`, corrected in
the same commit: the variant's doc named `Query::Items(..)` as the spelling that
still matches downstream, and that is the one spelling that does not. A tuple
pattern resolves through the variant's constructor, and `#[non_exhaustive]` is
what makes that constructor crate-private.

The second is the one to read carefully, because its family has already produced
a wrong test here. Its bound is written at the definition —
`fn f<S: SendEventStore>(…)` — so the obligation is discharged before
monomorphisation. Asserted against a concrete store it would pass by auto-trait
leakage whatever the trait said, which is the defect that retired
`read_stream_is_send` and which `SPECIFICATION.md` §3.8 records.

**An earlier draft of this section named two other tests here** —
`a_borrowed_and_an_owned_tag_are_one_value` and
`a_map_keyed_by_event_type_is_probed_by_str`, VT-33's pair — reached by
elimination against the clauses that name unit tests rather than by reading
`git show e551cdf`. Both pre-date this pass. The error is recorded rather than
silently corrected because it is this document's own subject in miniature: an
inference that resolves plausibly, against a real file, and is wrong.

### 5. The checker's own coverage

`check_citations` required a citation to be path-qualified **and** name a `.rs` or
a `.toml`. Of 338 citations, 84 satisfied both; 200 were bare file names and 56
named a `.md`. Neither form was ever parsed. The step was not a weak check over
the corpus — it was a correct check over a quarter of it, reporting "no problems
found" about the other three quarters.

### 6. Conformance rules no clause claimed

Check 6 swept `suite.rs` alone. `RULE_FILES` has three entries
(`xtask/src/spec_trace.rs:85-89`), and the other two — `model.rs` and
`concurrency.rs` — hold six rules between them that no clause had ever named:
five in `concurrency.rs` and one in `model.rs`. Counted today: 89 `pub async fn`
in `suite.rs`, 5 in `concurrency.rs`, 1 in `model.rs`, which is the 95 the summary
line now prints.

Four of the six were attribution errors — the clause already named the wrong
implementation by hand and simply never named the rule that rejects it. Two were
not, and are gaps 1 and 2 below.

---

## What was repaired, by commit

Six commits, each with `cargo xtask ci` green.

| Commit | What it did |
|---|---|
| `3c704d3` | Discharged the nine documentation obligations of class 3 |
| `e551cdf` | Wrote the five tests of class 4, each observed red first |
| `a843b99` | Widened `citations()` from 84 of 338 to the whole corpus, and made the step state its own coverage |
| `52105d2` | Widened check 6 from `suite.rs` to all three `RULE_FILES` |
| `89bb966` | Re-tensed sixteen clauses, recounted six self-referential numbers, refreshed §5's inventory, re-anchored ~130 citations |
| `84dcc67` | Added the content anchor: a citation must be about the thing the sentence attributes to it |

Four of them are worth more than a table row.

**`a843b99` — the widening.** The old filter is quoted in the function's own docs
now (`xtask/src/spec_trace.rs:2186-2198`), which is the honest place for it: a
check's history belongs beside the check. Two things had to be got right for the
widening not to produce noise. A bare name may be ambiguous — fourteen manifests
are called `Cargo.toml`, and bare `lib.rs` means the **sync** crate at both of its
sites, so a "prefer `happenstance-core`" rule would have resolved both to the
wrong file and passed; the resolution is a table of evidence
(`BARE_NAME_MAP`, `:2037-2047`) and an unmapped collision is a hard failure naming
the candidates. And dropping the `contains('/')` requirement let whole backticked
*sentences* through, because prose ending in `.md:688-693` satisfies
`rsplit_once(':')`; the replacement rejects any candidate path containing
whitespace (`:2232-2241`).

**`52105d2` — check 6.** The four attribution errors are now claimed: VT-11 by
`positions_are_unique_under_concurrent_appends` (`SPECIFICATION.md:1030`), ES-18
by `a_concurrent_reader_never_sees_a_partial_batch` (`:3341`), ES-19 by
`append_returns_the_callers_own_last_position` (`:3452`), ES-25 by
`exactly_one_of_n_contenders_commits` (`:3724`). The remaining two went into
`UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1968-1997`), which is a ratchet
and not an allowlist: every entry prints on every green run, and an entry whose
rule *becomes* claimed is itself a failure (`:770-775`), so the array can only
shrink and deleting the last entry deletes the mechanism.

**`89bb966` — §5's inventory.** One example, because it is the shape of the work
and every field of it is checkable. `SPECIFICATION.md:5761-5777` now records that
phase 5 added a fifth module to `happenstance-sync`: `wire.rs`, **359 lines**,
declared by `pub mod wire` at **`lib.rs:147`**, with the crate's `tests/` now
holding **four** files. Re-measured today: `wc -l` gives 359, the declaration is
on line 147, and the directory holds exactly `cursor_shape_probe.rs`,
`ingest_reaches_a_foreign_store.rs`, `real_peer_shapes.rs` and `wire.rs`. The
paragraph also states what did *not* move — no new `SyncError` variant, no derives
on `PushBatch`/`EventGroup`/`ReplicatedEvent` — which is the half an inventory
usually omits and the half that keeps §5's bound decisions honest.

**`84dcc67` — the content anchor.** A citation is now evidence *for* the nearest
backticked identifier before it, and the check asserts that identifier occurs
within `ANCHOR_SLACK` lines of the cited range
(`xtask/src/spec_trace.rs:336-365`). It anchors 69 of 358 citations today. The
other 289 are counted and not anchored, which is the honest outcome: a `.md`
target is evidence for a *passage* rather than a definition, and a sentence with
no backticked subject has no derivable anchor. The design constraint that decided
its shape is in [the lessons](#what-the-measurements-taught).

---

## What was found and deliberately not repaired

Two citations were confirmed not to resolve to what a naive reader would expect,
and both were left exactly as they are. Neither is a defect, and in both cases
"repairing" it would have destroyed the record it exists to keep.

### The declared-historical region

§6 opens by pinning its own measurement to a commit that is not this one:

> Measured at the start of phase 3, it was worth less than it says. The
> measurement is of *that* tree, `b4b593d`, line numbers included — which is why
> those numbers do not resolve against the working copy and are not meant to.
> That covers the rest of this paragraph as well as §6.1 and §6.2 below…
> (`SPECIFICATION.md:7033-7036`)

Re-anchoring those citations would make a phase-3 measurement describe a phase-5
tree, which is the same error as rewriting a dated review's vocabulary — the rule
[`README.md:75-85`](README.md) states for this whole directory.

So the anchor check exempts the span, and it finds it **by content rather than by
line number**: from the line containing "line numbers included" to the `### 6.3`
heading (`xtask/src/spec_trace.rs:406-424`). Today that is
`SPECIFICATION.md:7034-7501`, and **14** citation-shaped spans sit inside it. A
hard-coded span would come to cover the wrong section silently, which is the
failure the whole check exists to prevent, one level up. If either anchor
disappears the function returns nothing rather than guessing, so the region's
citations get reported — loud and correct — instead of a possibly-wrong span being
exempted.

The prose's own count of "eight citations that look current and are not" is the
`query_of_types` list at `:7051-7052`; seven of its eight are the bare `:447`,
`:465`, … continuation form, which the parser does not treat as citations at all.

### The citation quoted inside a sentence that calls it false

One entry, and it is the reason `UNANCHORED_CITATIONS`
(`xtask/src/spec_trace.rs:389-392`) is a mechanism rather than a special case.
ES-3's `Rejects:` bullet reads:

> That leg carries the clause on its own, which is as well, because the second leg
> this bullet used to offer is false: it said `memory.rs:154` and
> `crates/happenstance-sqlite/src/event_store.rs:202` "both write `+ Send` and
> would both need `+ Send + Sync`", and they would not.
> (`SPECIFICATION.md:2549-2553`)

The number is part of the quotation. Repairing it, or anchoring it, would falsify
the record of what was once claimed — and the claim is recorded precisely because
it is the argument a reader reconstructs unprompted. Flipping the attribute and
running `cargo check --workspace --all-features` produced zero errors, which is
the measurement the bullet keeps.

---

## The seven gaps

Recorded, not decided. Each needs an ADR; none got one.

### 1. `k_disjoint_boundaries_admit_exactly_k_commits` has no clause

The central DCB independence proposition — commands sharing no consistency
boundary do not conflict — is enforced by a rule at
`crates/happenstance-testkit/src/concurrency.rs:436` and stated by no clause. The
word **"disjoint" occurs zero times** in `SPECIFICATION.md`; re-derived today with
a case-insensitive count.

ES-25's *only if* half forbids the false-positive direction and does not say this,
so claiming it there would assert that a `[FROZEN]` clause contains a proposition
it does not. Held in `UNCLAIMED_PENDING_ADR`
(`xtask/src/spec_trace.rs:1969-1978`) and printed on every green run.

**What would settle it:** an ADR that either widens ES-25 (`SPECIFICATION.md:3695`)
or mints a clause.

### 2. `ops_agree_with_the_model` has no clause

`crates/happenstance-testkit/src/model.rs:598`. It replays a generated sequence of
appends, conditional appends and reads against a model and compares every answer,
so what it enforces is the *composition* of ES-8, ES-9, ES-11, ES-14, ES-15, ES-18
and ES-25 over inputs no clause enumerates. §6.4 names it, but only as CF-22's
illustration of a per-family enumeration. Claiming it under any one of the clauses
it exercises would say that clause is what it checks.

**What would settle it:** minting the clause the model family has never had — a
store agrees with the contract over arbitrary operation sequences, not only over
the examples the suite enumerates — or deciding that check 6's bar is per-clause
and a cross-clause rule is disposed of some other way. In
`UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1979-1996`).

### 3. PS-1's MUST is a coupling, not a progress obligation

PS-1 (`SPECIFICATION.md:4733-4735`, `[FROZEN]`) says the read-model write and the
checkpoint write become durable together **or not at all**. §4.11's table
(`:5661`) assigns it `commit_advances_the_checkpoint`. A `commit` that returns `Ok`
and makes *neither* durable satisfies the MUST through the "or not at all" arm,
passes `commit_is_atomic_with_the_read_model` — both-absent is one of the two
states that rule permits — and fails the other one. That a successful commit
*advances* anything is stated by no clause's MUST in the document.

Recorded in the clause body at `:4743-4753`; phase 6 owns the repair, and it is an
ADR's rather than an edit's because the clause is `[FROZEN]`.

### 4. PS-19's MUST is scoped "after a successful reset"

PS-19 (`SPECIFICATION.md:5218-5221`, `[FROZEN]`) is about what `checkpoint(id)`
returns **after a successful `reset`**. `fresh_projection_has_no_checkpoint`,
which §4.11 (`:5660`)
also assigns it, asks about an id that has never been seen. A store that writes an
explicit `NeverRun` sentinel on reset and resolves a missing row with
`.unwrap_or(Live { through: FIRST })` satisfies the clause verbatim and fails the
rule — and that is the natural shape rather than a contrivance.

Recorded in the clause body at `:5230-5239`; phase 6 owns whether the clause widens
or a new one says it.

### 5. ES-6 is `[FROZEN]` naming a rule that cannot exist

ES-6 (`SPECIFICATION.md:2629-2631`) is the strongest maturity marker in the
vocabulary attached to the weakest evidence. Its `Rule:` line (`:2670`) names
`store_error_crosses_a_join_handle`; §7.2's row (`:8588`) renders it `†`, which
the legend defines as "must be written".

Verified today: the name occurs as **no `fn` anywhere in the workspace**. Its three
`.rs` occurrences are comments in `happenstance-cloudflare` —
`src/lib.rs:92`, `src/send_shape.rs:14`, `src/send_shape.rs:95` — and
`src/lib.rs:88-94` states the reason in its own words: the rule "is unwritable
against today's port for *every* adapter, not merely for this one". The
demonstration sits beside it: `SendStoreWithLocalError` (`send_shape.rs:123`, the
impl at `:137`) satisfies every `Send` obligation the derived flavour states,
carries a `!Send` `Error`, and compiles.

**Check 4 cannot catch it, and the reason is structural.** The check skips any
clause that declares its rules new (`xtask/src/spec_trace.rs:684-687`), and
`schedules_new` is set by the literal `(new)` or `†` in the `Rule:` field
(`:1616-1620`). ES-6 carries `(new)`. This is the exact inverse of what `52105d2`
fixed: that commit made every rule have a clause; this is a clause whose rule
cannot be written.

Found independently by [`review-citation-drift.md`](review-citation-drift.md) §2,
on the same date, from different work.

**What would settle it:** writing the rule, which requires a port change; or
demoting the clause and recording why. Either is an ADR, because ES-6 is
`[FROZEN]`.

### 6. ES-7's and VT-9's `[PROVISIONAL]` markers look discharged

A `[PROVISIONAL]` marker is a standing bet with a named falsifier. Both of these
have had their falsifier reached without being falsified, which is a different
state from "still open" and is not what the marker communicates.

**ES-7** (`SPECIFICATION.md:2685`) is provisional against "`error[E0119]:
conflicting implementations` on a downstream `impl EventStore for LocalType`"
(`:2690-2693`).
Its named instrument, `LocalMemoryEventStore`
(`crates/happenstance-testkit/tests/local_conformance.rs:198`), is a direct impl in
a genuinely downstream crate that sits beside the blanket impl without `E0119` and
passes every rule natively and on `wasm32` — the clause says so itself at
`:2698-2703`.

**VT-9** (`SPECIFICATION.md:903`) is provisional against "a target that cannot
supply a wall clock at append time" (`:912-914`). `crates/happenstance-core/src/memory.rs:263-274` says
outright that `wasm32-unknown-unknown` **is one** — `SystemTime::now()` does not
error there, it aborts — and stamps every event `RecordedAt::from_millis(0)`. VT-9's
MUST is satisfied anyway, because its rules assert that a recorded time is
*stable*, never that it is recent. So the falsifier is satisfiable without
falsifying.

**What would settle it:** moving a marker, which is an ADR's.

### 7. Nothing owned a specification reconciliation after phase 4 or phase 5

The last one was phase 3's — ADR-0008's four amendments plus phase 2's fourteen
falsified claims — pooled there because that phase was already opening §6 and §7
(`RUNBOOK.md:3801-3804`). Phases 4 and 5 were the two largest changes to the
contract in the plan and neither carried an equivalent item.

The rule that would have caught it was already written down at the end of the ADR
queue (`RUNBOOK.md:334-336`): *a phase's clause range and the union of its ADRs'
clause ranges are two numbers, and nothing checks that they are equal. Compute both
at the phase's exit.* Nothing implements it.

It is now a standing exit criterion for every phase from 6 onward
(`RUNBOOK.md:3810-3820`). The numeric comparison it asks for is still performed by
no tool.

---

## Corroboration — `review-citation-drift.md`

[`review-citation-drift.md`](review-citation-drift.md) carries the same date and
is pinned to `3712c9b`, the commit this pass started from. It was written
independently, as a byproduct of building [`docs/rust/`](../rust/README.md), by
work with no brief to audit anything.

It found six stale citations, diagnosed the identical root cause —
insertion-above-a-cited-line, invisible to a check that verifies existence and a
line bound — and recommended the identical remedy: port `parse_citation` and
`check_citations` from `xtask/src/lint_constitution.rs`, "about forty lines"
(`review-citation-drift.md:63-69`).

The overlap and the residue:

- **Five of its six `SPECIFICATION.md` citations are discharged by this pass.**
- **The `docs/adr/0009` row is discharged too**, and by an edit this pass did not
  plan to make. A sweep agent scoped to `SPECIFICATION.md` repaired three
  citations in `docs/adr/0009-error-send-sync.md` as well —
  `store.rs:100 → :101`, `projection.rs:73 → :90`, and `memory.rs:143-145 →
  :291`, two of them now carrying the `(anchor)` form. The edits are correct and
  are exactly the drift `review-citation-drift.md` §1 names. They are kept rather
  than reverted, under ADR-0006's own rule — *rewrite the referent, never the
  reasoning* — which is what distinguishes a citation repair from an amendment to
  an accepted decision. No reasoning changed; the ADR still argues what it argued.
  Recorded here because two earlier drafts of this document asserted the file was
  untouched, and an out-of-scope edit that nobody writes down is the kind that is
  discovered later by someone who cannot tell whether it was deliberate.
- **Its §4.1** — `query.rs` naming `Query::Items(..)` as the pattern spelling that
  works downstream, when that is the one spelling that does not — is discharged by
  `e551cdf`.
- **Its §2 is gap 5 above** and remains open.

That document also bounds itself explicitly: "It is not a citation audit… assume
there are more" (`review-citation-drift.md:256-259`). This pass is the audit that
assumption asked for, and it found 128 where the byproduct found six.

**Two independent passes converging on the same root cause and the same remedy is
itself the evidence** that the defect is structural rather than incidental. Neither
was in a position to be influenced by the other; the recommendation each reached
is the same forty lines.

---

## What the measurements taught

Each of these is a measurement, not a preference. Where the number survives in the
tree, the citation points at it.

**A check that does not state its own coverage is indistinguishable from one that
sees everything.** `check_citations` was correct over 84 of 338 citations and
printed "no problems found". The summary line now carries the coverage
(`xtask/src/spec_trace.rs:291-296`), which is the half of that change worth
keeping.

**Deriving an anchor from prose is only worth it where the derivation is certain.**
Four attempts. The first walked back up to four backticked spans to find the
sentence's subject; the function's own comment records the result —
"reported seventy failures out of two hundred and sixty-two"
(`xtask/src/spec_trace.rs:2109-2111`) — nearly all of one shape: a sentence with no
backticked subject at all, where reaching back far enough always finds *some*
identifier and it belongs to the previous sentence. Restricting to the
immediately-preceding span on the citation's own line or the one above
(`:2116-2118`), declining anything that is not a plain identifier (`:2120-2130`),
declining upper-case names because a type is cited at its behaviour and declared
four hundred lines away (`:2135-2142`), excluding `.md` targets (`:2074-2080`), and
searching the whole cited *range* rather than around its first line
(`:2246-2256`) took it to two reports. Both were real, and both were citations the
sweep had repaired into the wrong place hours earlier.

**The discriminator that made it work.** If the subject appears nowhere in the
cited file, the derivation picked the wrong word and the check declines
(`:340-352`). If it appears but far away, that is drift and the check reports it.
Only the second is worth reporting, and separating them is the difference between
a check that reports drift and one that reports its own guesses.

**A windowed search is what makes a content check survivable.** Inserting lines
above a cited item must not red the gate. A content hash fails that property on
every ordinary edit, and its refresh command becomes a reflex nobody reads. The
slack constant carries that reasoning at its definition
(`xtask/src/spec_trace.rs:374-381`).

**A ratchet, not an allowlist.** `UNCLAIMED_PENDING_ADR` prints every entry on
every green run, and an entry whose rule becomes claimed is itself a failure
(`:770-775`). So it can only shrink, and deleting the last entry deletes the
mechanism. It cannot be used to make a problem go away quietly, which is the
property that distinguishes it from the drift allowlist it replaced.

**Repair versus gap.** A correction to a `[FROZEN]` clause is a *repair* if the set
of implementations it admits is unchanged, and a *change* otherwise; only the
second needs an ADR. For a MUST that turns out to be already discharged, the safe
form is to keep it and record the discharge with its evidence — never to delete it.
§7.5 states the same rule for defect lists: two entries "are kept in the table with
their closures recorded, because a defect list that deletes its own entries cannot
be audited" (`SPECIFICATION.md:9009-9011`).

**Two documents that agree can both be wrong**, and then a precedence rule gives no
arbitration. §7.5 said five clauses name no case; corrected to three; and three was
also wrong, because §7.2 is *generated* and showed two the authored table had never
held. The honest count is five with different members, and §7.5 now says so and
says why the unchanged headline is a coincidence (`SPECIFICATION.md:9013-9021`,
`:9035-9040`). The general form: a defect list assembled by hand beside a
machine-generated one answering the same question will drift, and comparing it
against itself cannot find that.

**Line-numbered citations across documents are a standing tax.** Every edit to
`crates/**` or `xtask/**` in this pass broke citations in `docs/rust/` — three
separate times, each caught by `cargo xtask lint-constitution`, which is green
today over 27 atoms. The tax is not an argument against the citations; it is the
argument for the check, because the alternative is the same breakage discovered by
a reader.

---

## Re-measured while writing this document

Four observations made in the course of grounding the claims above. They are
recorded rather than repaired, because this document changes no code.

**Three clauses render `†` for tests that exist.** §7.2's second reading convention
promises that where a clause points at "a unit or compile test living in the crate
the clause constrains", the cell carries the clause's own words and **no dagger** —
"a dagger there would assert an absence nothing checked"
(`SPECIFICATION.md:8477-8481`). The trigger for that branch is a literal phrase in
the `Rule:` field: `unit test`, `compile test` or `meta-test`
(`xtask/src/spec_trace.rs:1614-1615`). ES-3 (`SPECIFICATION.md:2541`) and ES-5
(`SPECIFICATION.md:2620`) — and ES-4, which names ES-3's rule — describe their
instrument in other words, "a static assertion in `happenstance-core`'s own
tests", so all three fall through to the name-list branch and render `†` at
`SPECIFICATION.md:8585-8587`. Both tests exist — one of them created by this
pass, in `e551cdf`, and daggered anyway:
`provided_method_future_is_send_in_generic_code` at
`crates/happenstance-core/src/memory.rs:777`, and
`error_bound_is_identical_on_both_flavours` at
`crates/happenstance-core/src/store.rs:367`. §3.8
(`SPECIFICATION.md:4453-4458`) already states that these live outside the suite
and are deliberately not counted. This is the same
escape hatch as gap 5, pointing the other way: `(new)` suppresses check 4's lookup,
so neither the false `†` nor ES-6's impossible one can be noticed.

**The anchor slack is twelve here and ten there.** `xtask/src/spec_trace.rs:376`
says its constant is "the same twelve `docs/rust`'s own citation lint uses".
`xtask/src/lint_constitution.rs:111` is `const ANCHOR_SLACK: usize = 10`. The
*reason* is the same and the number is not — which is a citation-drift defect
inside the comment that installed the citation-drift check, and is recorded here
for that reason rather than for its consequence, which is none.

**`RUNBOOK.md:3828` cites a file that does not exist.** It points the remaining
gaps at `.kb/_intake/gaps-owed-a-decision.md`. As of this date `.kb/_intake/`
contains `README.md` and nothing else, so this document is the only record of gaps
3, 4, 5, 6 and 7; gaps 1 and 2 are additionally held by the gate.

**One apparent contradiction, refuted.** `store.rs:127-129` reads "Returns the
position assigned to the **last** appended event. The specification does not
require this", while ES-19 (`SPECIFICATION.md:3437`, `[FROZEN]`) makes exactly that
a MUST. It is not a defect: `happenstance-core`'s doc comments use "the
specification" for the **DCB** specification throughout — "the specification permits
gaps" (`event.rs:271`), "the specification's own term" (`event.rs:317`) — and DCB
does not require `append` to return a position. Recorded so it is not re-derived.

---

## What this document does not check

Stated because a review that does not bound itself gets read as exhaustive.

- **It does not audit `docs/adr/`.** The audit's corpus was `SPECIFICATION.md`.
  Three citations in `docs/adr/0009` were repaired incidentally — see above — and
  that is the whole of the ADRs' coverage: seventeen files carrying `file:line`
  citations under exactly the same exposure, with no check over them at all.
  `check_citations` is handed `SPEC`'s contents and nothing else
  (`xtask/src/spec_trace.rs:297-307`, called at `:723`). That the three repairs
  happened by accident rather than by a sweep is the point — nothing would have
  found the other sixteen files' drift, and nothing yet will.
- **It checks no clause's truth.** A clause whose MUST is wrong about the domain
  passes every check described here, provided its prose and its citations describe
  the code beside them.
- **The anchor covers 69 of 358 citations.** The other 289 are counted and
  unanchored by design, so a `.md` citation or one whose sentence names no subject
  can still drift silently. The coverage is printed on every run so that this
  sentence cannot quietly stop being true.
- **Nothing pins this document.** If a repaired citation moves tomorrow, this file
  becomes wrong with no step going red — the same property every other document in
  this directory has, and [`README.md`](README.md) explains why that is deliberate.
