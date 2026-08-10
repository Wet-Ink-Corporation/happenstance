---
id: adr-0013-position-assignment-and-visibility
title: "ADR-0013: Positions are assigned once, become visible in order, and the freeze names what it is accepting"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  A store assigns each position once, never reuses one, MAY leave gaps, and makes
  positions visible in assignment order — the invariant that makes AppendCondition's
  `after` sound. Because gaps are permitted, no conformance rule may assert a literal
  position value (CF-6); that prohibition exists to protect a MAY.
depends_on: []
related:
  - reference-experiment-position-visibility
  - question-postgres-position-visibility
source_paths:
  - docs/architecture/SPECIFICATION.md
  - docs/experiments/position-visibility/README.md
last_reviewed: 2026-08-09
adr_id: ADR-0013
phase: 4
supersedes: []
superseded_by: null
---

# ADR-0013: Positions are assigned once, become visible in order, and the freeze names what it is accepting

- **Status:** accepted
- **Date:** 2026-08-08
- **Settles:** VT-11, VT-12 (confirmed as a pointer, not restated), VT-13, ES-10
  — which this ADR lifts from `[PROVISIONAL]` to `[FROZEN]` — and ES-38
- **Also settles, because the lift forces them:** the `Rule:` line of ES-30 — one
  of the three rules it names, and its MUST is untouched — and the poll-count
  limitation, which is claimed by no clause today
- **Rests on and does not settle:** ES-9 `[FROZEN]`. VT-13's resume half is sound
  only because `from` is a threshold rather than a seek, so ES-9 is cited
  throughout this document and amended nowhere in it. The queue leaves ES-9 among
  the twenty-nine phase-4 clause IDs no ADR claims, and the reconciliation dossier
  puts it on ADR-0011's list to claim or decline; this ADR does neither, and says
  so here so that "0013 covered it" cannot be the reason it goes unclaimed.
- **Extends:** [ADR-0010](0010-the-suite-must-prove-itself.md), whose obligation —
  a rule must be demonstrated to fail something — is the standard the single rule
  behind this lift is held to, and the reason its limitation is written down here
  rather than discovered at phase 10
- **Does not settle:** which mechanism `happenstance-postgres` uses. That is
  ADR-0024's, at phase 10. This ADR needs one affordable mechanism to exist; it
  does not choose the adapter's.

## The question, and the one that travels with it

**What does a store promise about position assignment and visibility — gaps,
reuse, and the invariant that makes `AppendCondition::after` sound?**

The CF-25 axis acceptance is written here too, and that is not a second question
smuggled in. It is this question's accounting. ES-10's `[PROVISIONAL]` marker is
where the position-allocation axis's CF-25 exposure has been carried since the
specification was assembled; lifting the marker deletes the carrier. The
acceptance has to move somewhere in the same act, or the exposure disappears
silently — which is the failure CF-25 exists to prevent. So the lift and the
acceptance are one decision with two halves, and the ledger is stated in full
because a partial ledger is how the exposure gets lost.

Two neighbouring clauses are touched only as far as the lift reaches into them,
and no further. **ES-30's `Rule:` line** is amended because arm C fails one of the
three rules it names *as parenthetically described*, while satisfying the clause's
MUST — see
[decision §4](#4-caveat-three-head-is-a-frontier-and-maxposition-is-the-wrong-answer).

Whose ES-30 is it otherwise? Not this ADR's, and the answer is worth stating
precisely because the two sources disagree. `RUNBOOK.md:442`'s ledger assigns the
`head`/`count` row — "settled — ES-30, ES-31" — to **ADR-0012**, while the
reconciliation dossier counts ES-30, ES-31 and ES-32 among the twenty-nine phase-4
clause IDs that no queued ADR brief claims. Either way they are not 0013's, and
the parenthetical amended below is the whole of what this document takes. One
coordination note for whoever writes 0012: that ledger cell also says `head` is
**provided**, and ES-30 `[FROZEN]` says it MUST be **required**. The clause wins
(RUNBOOK "How to use this" rule 5) and the cell is owed a correction, but the
correction is 0012's and is recorded here only so it is not lost.

## Context

### What is already decided, and is transcription

**VT-11 `[FROZEN]`.** Positions are unique within a store, strictly increasing in
assignment order, and **gaps are permitted**. A position is not a count and the
difference between two positions is not a number of events.

**VT-12 `[NON-NORMATIVE]`.** The visibility invariant used to be stated here *and*
at ES-10, and within one editing pass the two copies acquired different maturity
markers. VT-12 is now a pointer, deliberately. This ADR does not restate the
invariant in `SPECIFICATION.md` §2 and neither may the code: where §2 needs it,
it cites ES-10.

**ES-38 `[FROZEN]`.** A store from which events have been removed by any means
outside the port keeps every promise about the events it still holds: no position
is ever reused, the survivors stay unique and monotonic, and `Query::all()` still
means "everything this store holds". The implementation it rejects — renumbering
on compaction — is the obvious move for a device pruning to save space, and it
invalidates every checkpoint and every replicated `after` naming that store.

### What was open, and what closed it

**ES-10 `[PROVISIONAL]`,** on cost rather than on correctness, with a lift
condition its own marker names: *"one affordable answer lifts this clause to
`[FROZEN]` at phase 4."* The three candidate answers were `xid8` +
`pg_snapshot_xmin`, transaction-scoped advisory locks, and a serialised sequence
table. The marker also names the consequence of a bad result: ES-25 and ES-26 are
sound only where visibility order agrees with position order, so three
unaffordable answers reopen them, which is a contract change and not a scheduling
one.

Phase 2 ran the probe against a real PostgreSQL with `fsync=on`
([`docs/experiments/position-visibility/`](../experiments/position-visibility/README.md)).
Arm C — `xid8` + `pg_snapshot_xmin` — is the only arm that both passes the
inversion detector on both writer pairs **and** leaves writers unserialised.
Throughput ratio to a bracketing baseline: **0.987 / 0.993 / 1.015 / 1.026** at
1 / 8 / 32 / 64 clients.

Cite those four numbers from **two files or a reader will conclude the opposite**.
The 8-, 32- and 64-client figures are `results/ratios.csv`. The one-client figure
is `results/ratios-c1long.csv`, the separate 90-second pass, because at 30 seconds
the baseline spread at one client was 3.70× — larger than every effect in the row.
`ratios.csv` records arm C at **0.628** at one client against a baseline that
drifted 3.70×, and anyone chasing the series in that file alone will read arm C as
37% slower at one writer. `README.md:248-256` explains the substitution.

The instrument is known to be capable of returning a negative, which is the
property that makes a positive worth anything: both declared positive controls
fired. The baseline reproduces the inversion, and arm A at 64 clients collapses to
0.062× of baseline with p99 60× worse.

## Decision

### 1. ES-10 lifts to `[FROZEN]`. ES-25 and ES-26 are not reopened.

The condition ES-10's marker states is met, on arm C's evidence and on nothing
weaker. The invariant, which is the sentence the port must carry and the only
place it is stated:

> Once any reader has observed an event at position *P*, no subsequent read
> against that store may yield an event at a position ≤ *P* that was not already
> visible. An adapter MUST NOT make an event visible at a position below one it
> has already exposed.

And the warning that has to travel in the same paragraph, because without it the
sentence reads as free: **an adapter that allocates positions before it commits
does not satisfy this.** A position taken from a sequence at the start of a
transaction is released at commit, so a transaction that started later can become
visible earlier. The caller then conditions on `after: Some(100)` while 99 is
still invisible; when 99 lands, `is_violated_by` evaluates `99 <= 100`, reports no
violation, and the guard that exists to catch exactly this stops enforcing. There
is no error, no rejected append and no failing test — Wattline's nineteen
milliseconds, at `SPECIFICATION.md:2415-2423`.

Three caveats travel with the lift. Each is a thing the clause's current text does
not carry, and each is stated here because a freeze that carries only its good
news is not a freeze.

### 2. Caveat one: the lift moves the position-allocation axis *into* the acceptance list

This is the opposite of what a casual reading of phase 4's sixth exit criterion
suggests, and it is worth being blunt about. The `[PROVISIONAL]` marker was
*itself* the CF-25 disclosure for that axis: it named the axis, named the unbuilt
far end, and named the falsifier. Freezing the clause deletes that disclosure. The
experiment says so in its own limitations section (`README.md:364-366`): *"Nothing
here is an adapter … This measures four SQL strategies, not four implementations
of `SendEventStore`."*

So the ledger after this ADR is:

The column headings are §6.5's, not new ones: an *instrument at the far end* is
what CF-25 counts, and a skeleton is not one — `todo!()` has type `!` and coerces
to anything, so a body of them falsifies a signature and never a semantic.

| Axis | Instrument at the far end today | Who carries the CF-25 exposure |
|---|---|---|
| **Position allocation** | Fixture (`PreCommitPositionStore`, CF-13). No adapter | **Accepted by this ADR** |
| Transport | None. `happenstance-neon` is a phase-2 skeleton, which falsifies a signature and is not a far end | ES-11, ES-12 `[PROVISIONAL]` |
| **Async flavour** | Fixture (`LocalMemoryEventStore`, CF-28). No adapter | **Accepted by this ADR** |
| **Batch shape** (`ProjectionStore`) | None. Skeletons at *both* ends and a passing implementation at neither, with no projection suite to run any of them against (`SPECIFICATION.md:7186`) | **Accepted by this ADR, pro forma** — see decision §7 |
| **Handle multiplicity** | Fixture (`Fixture::connect` + `SECOND_HANDLE`, CF-16 is the seam; CF-19 is the rule obligation; `CachedHeadFixture` fails it). No adapter | **Accepted by this ADR** |
| Durability | Fixture (`DurableFixture` supplies `REOPEN`, CF-17; `LosingFixture` beside it fails). No adapter | ES-35 `[PROVISIONAL]` |
| Completeness | None at either end, and nothing planned before phase 14 | ES-40 `[PROVISIONAL]` |

**Four accepted, three carried by their own clauses.** That is a partition of
seven and it is stated as a partition rather than as an integer, because the
integer is where both documents went wrong (decision §7).

What is being accepted **as risk** on the three fixture-instrumented axes is
precisely the half CF-26 says a fixture does not buy: *implementability*. The
distinction is not pedantic, and §6.5 says why row by row. On handle
multiplicity, `MemoryFixture`, `LocalFixture` and `CachedHeadFixture` all hand out
refcount clones of one in-process object, so **no connection has ever been opened
twice** in this workspace (`SPECIFICATION.md:7188`). On async flavour,
`LocalMemoryEventStore` proves the `!Send` rules can be run and can fail, and says
nothing about whether a Durable Object with a real `SqlStorage` can pass them. On
position allocation, `PreCommitPositionStore` suspends between allocating and
publishing because a rule polled it that way, and has never met a transaction.
Fixtures buy falsifiability. Adapters buy implementability, and the adapter column
is empty on all seven rows.

**That caveat outranks the frozen list as well as the moving one** — §1.7 closes
with "the honest caveat that outranks all of them" and *them* is both of its
lists (`SPECIFICATION.md:437-448`). Freezing ES-10 does not soften it.

### 3. Caveat two: this ADR keeps the invariant **global**, and says so out loud

Arm B-tag — a transaction-scoped advisory lock keyed by tag — is nearly free
(0.935 at 64 clients against arm C's 1.026) and buys a **per-boundary** invariant
where ES-10 states a **global** one. On disjoint tags it reproduces the baseline
inversion byte-for-byte. The experiment measured that the two are not the same
property and deliberately refused to settle which one happenstance needs
(`README.md:402-412`), because DCB evaluates an `AppendCondition` against a
boundary and ES-10 may therefore be stronger than its consumers require. Lifting
ES-10 as written would close that silently, so it is closed out loud, with the
argument the experiment did not have to hand:

**A per-boundary invariant would make `AppendCondition` sound and the projection
checkpoint unsound, and the checkpoint is the harder consumer.** `head()` is
deliberately *not* query-scoped (ES-30), and the stated reason is that a narrow
projection's problem is advancing past events it examined and did not match — the
global head is what lets it checkpoint past them. So a runner resumes from a
global position covering boundaries it never reads. Under a per-boundary
invariant, a runner can observe position 100 on boundary X, checkpoint, and later
have 99 become visible on boundary Y; `from: checkpoint.next()` then skips 99
permanently, with no error anywhere. B-tag buys the condition and loses the
checkpoint. Replication has the same shape from the other side: `SY` clauses cite
ES-10, and a boundary-scoped invariant would need the boundary to travel with the
position, which `SequencePosition` does not carry.

This is a decision with a real cost — roughly 9% of throughput at 64 writers,
measured — and it is taken deliberately rather than by default. Its falsifier and
owner are in [What this ADR leaves open](#what-this-adr-leaves-open).

### 4. Caveat three: `head()` is a frontier, and `max(position)` is the wrong answer

Under arm C a reader's visibility is a predicate rather than an identity — the
read is `WHERE … AND xact_id < pg_snapshot_xmin(pg_current_snapshot())`
(`schema/arm-c.sql:47`), so a row whose writing transaction's `xid8` is at or
above the frontier is not visible however low its *position* is — and `head()`
reports the **frontier** rather than `max(position)` over the table. ES-30 is
`[FROZEN]` and its owed rule is named `head_is_the_highest_visible_position`.
Those two things have been sitting in the tree unreconciled, and the
reconciliation is not a compromise: **the frontier is the only value consistent
with ES-10 and ES-30 at the same time.**

A `head()` returning `max(position)` under arm C may return a position that is
*not visible* — and will, whenever the highest-positioned row's transaction has
not yet passed the frontier, which is the ordinary case under load. A caller that
then reads from it gets nothing, which is precisely the
pagination failure ES-11 names when it calls `head()` the primitive that makes
correct pagination possible. "Highest visible position" is not a softening of
"highest position" — it is the clause's own words, and the frontier is what they
denote wherever visibility is a predicate rather than an identity.

What the frontier really costs is elsewhere, and it is surprising enough to be
worth a sentence in the port's own documentation: **`append` returning `Ok(P)`
does not promise that the next `head()` is at or above `P`.** Read-your-own-writes
does not hold under arm C, and staleness is bounded by the longest open write
transaction *anywhere in the cluster* — measured at **0.688 ms** with no holder
and **4010.719 ms** behind an unrelated five-second write in an unrelated database
(`results/staleness_pinned.txt`). An adapter choosing arm C is choosing a store
whose freshness is coupled to the health of every other workload on the same
server. That is a documented capability limit, not a tuning parameter.

That sentence is about the *pair* `append` and `head`, and neither method is this
ADR's. It is stated here on an explicit hand-off rather than by encroachment:
ADR-0011 lists *"what `head()` returns on a Postgres adapter that allocates
positions outside the transaction"* among the questions it contributes an argument
to and deliberately does not settle, and assigns it to **ADR-0013**. It is
recorded below as an amendment to **ES-10's** body rather than to any `append` or
`head` clause, and it touches no signature. If ADR-0012 says anything about
`head()` following `append`, this is the sentence it has to agree with.

The collision is therefore **not** in ES-30's MUST, which says "the highest
position currently visible" and is correct as written. It is in the parenthetical
describing one of its three unwritten rules —
`head_is_the_highest_visible_position`, *"(append, then assert `head()` equals the
position `append` returned)"* — which asserts read-your-own-writes and which arm C
fails. That parenthetical is amended here (see the amendments section); the other
two rules ES-30 names, `head_of_an_empty_store_is_none` and
`head_advances_across_two_handles`, are untouched.

The portable form asserts `head()` is **not below** the highest position a read of
the whole store yielded. Two things have to be said about what that costs,
because the tempting claim — "it still rejects two of ES-30's three
implementations" — is false and would have been a preservation argument nobody
could check.

**It rejects the default-query-scoped head, and only if the rule is written to
give it something to miss.** A `head` reporting the highest position matching some
implicit default query fails as soon as the store holds a higher event that query
does not match, so the rule must seed one — a second event type, or a tag the
default would filter on — and must read with `Query::all()`. A rule that appends
one uniform batch cannot fail it, which is CLAUDE.md's decorative-rule bar applied
to a parenthetical.

**It does not reject a cached last-written position, and neither did the equality
form.** Against a single handle, a store answering `head()` from a cached
last-written value returns exactly the position `append` returned and exactly the
highest position the read yields; it passes both spellings. That implementation is
rejected by `head_advances_across_two_handles` — ES-30's third rule, which opens
the second handle CF-16 supplies and CF-19 obliges a rule to use — and it always
was. Nothing is lost by the amendment because nothing was there to lose. The third
rejected implementation, the `backwards().limit(1)` workaround as a universal
answer, was never this rule's either: ES-30's own text hangs it on E2E-13, which
is unaffected. **ES-30's `Rejects:` set survives intact across the three rules**,
which is the claim worth making and the only one that is true.

The equality form cannot be recovered by "assert on a quiescent store" either,
because arm C's staleness bound is set by transactions the fixture does not own —
including transactions in a different database (the 4010.719 ms above) — and
CF-33 forbids the rule from waiting on a clock.

### 5. VT-13: `next()` uses `checked_add`, so the method whose sole purpose is signalling overflow can do it

`event.rs:142-144` is:

```rust
pub const fn next(self) -> Option<Self> {
    Self::new(self.0.get().saturating_add(1))   // event.rs:143
}
```

and the doc comment three lines above it promises `None` on overflow. It cannot
deliver one. `saturating_add` clamps to `u64::MAX`, `u64::MAX` is non-zero, so
`new` wraps it in `Some` and the caller is handed a position that is not the next
one. VT-13 `[FROZEN]` has already decided this — its `Rejects:` names
`event.rs:138-144` in terms — so what follows is the reasoning behind a decided
clause and its owed code change, not a new decision.

Nothing here is asserted from memory. Compiled and **run** on rustc 1.97.1
(`8bab26f4f`, the pin in `rust-toolchain.toml`) against a faithful copy of the
type, both spellings side by side:

```text
MAX.next_saturating()  = Some(18446744073709551615)
MAX.next_checked()     = None
FIRST.next_checked()   = Some(2)
const OVERFLOW         = None          // evaluated at compile time
size_of::<SequencePosition>()          = 8
size_of::<Option<SequencePosition>>()  = 8
```

The replacement stays inside the newtype instead of unwrapping to `u64` and back:

```rust
pub const fn next(self) -> Option<Self> {
    match self.0.checked_add(1) {
        Some(value) => Some(Self(value)),
        None => None,
    }
}
```

Three Rust-specific things make this a repair rather than a trade, and they are
worth spelling out because the shape is unfamiliar coming from C# or Python.

**The `Option` is free, so `saturating_add` bought nothing.** `SequencePosition`
is a newtype over `NonZeroU64`, not over `u64`, and forbidding zero leaves the
zero bit-pattern unused. rustc spends that unused pattern — the *niche* — on
`None`, so `Option<SequencePosition>` and `SequencePosition` are **both 8 bytes**,
which is the last two lines of the probe above and is already asserted as a
doctest at `event.rs:112-116`. The C# instinct is the right one to correct here:
`long?` is a `Nullable<long>`, a struct carrying a separate `bool` alongside the
value and therefore wider than the `long`, so an `Option`-shaped return there is a
real cost and the habit of avoiding it is well earned. In Rust over a `NonZero`
it costs nothing at all, and clamping to avoid a `None` traded a correct answer
for a saving that does not exist.

**`match` rather than `?` or `map`, because the function is `const`.** Neither is
usable in a `const fn` on this toolchain, and both were compiled to check rather
than assumed. `?` reports `error[E0658]: '?' is not allowed on
Option<NonZero<u64>> in constant functions`, with the underlying reason named
separately — *"`Try` is not yet stable as a const trait"*. `Option::map` reports
`error[E0658]: cannot call conditionally-const method Option::<NonZero<u64>>::map
… in constant functions`, and *"`Option::<T>::map` is not yet stable as a const
fn"*. Both are the same limitation seen twice: the const subset of the language
cannot yet call a trait method or a closure-taking combinator, so the desugared
`match` is the only spelling available. It is not stylistic noise.

`const` is load-bearing rather than decorative here: it is what lets
`SequencePosition::FIRST.next()` appear in a `const` initialiser at all, and the
probe evaluates exactly that (`const SECOND: Option<SequencePosition> =
SequencePosition::FIRST.next_checked();`, and its overflow twin at `MAX`, which is
the `const OVERFLOW = None` line above — a compile-time `None`, not a runtime
one). `NonZeroU64::checked_add` is itself const-callable, which is the whole
reason the repair is available without giving that capability up.

**The `Option` costs the caller nothing in practice** because house style forbids
`unwrap` in library code, so every call site was already going to branch. The one
that matters is `ProjectionStore::checkpoint`'s resume path
(`projection.rs:84-93`), and it is where the wrong answer would have gone
unnoticed longest: a checkpoint at `u64::MAX` resuming from `u64::MAX` reads the
same event forever, in the one place nobody will test. That is a claim about
*detectability* and not about likelihood — VT-13 is right that the blast radius is
nil in practice, because nothing reaches `u64::MAX`. The reason to fix it is that
the fix is free, not that the bug is dangerous.

### 6. VT-13's other half: `checkpoint.next()` is the resume idiom on every adapter, because ES-9 makes `from` a threshold

VT-13 `[FROZEN]` has two MUSTs, and the overflow fix above is only the first. The
second is that *"the contract MUST document that `checkpoint.next()` is therefore
the correct resume idiom and that it is sound on a store with gaps."* That
sentence is undischarged in the code today, and discharging it is this ADR's
because VT-13 is.

What makes it sound is ES-9, which is `[FROZEN]`, is nobody's to reopen and is not
reopened here: `ReadOptions::from(p)` is a range predicate over assigned
positions. When nothing occupies `p`, a forward read yields the next matching
event above it and a backward read the next below; it does not error and does not
return empty on that ground alone. The implementation ES-9 rejects is `from` as an
equality seek or a `rowid` offset — plausible wherever positions came from a dense
counter and the author assumed density, and **fatal after the first purge**.

The three clauses lock together and none of them is sound alone. VT-11 permits
gaps. ES-26 makes `AppendCondition::after` **exclusive** and `ReadOptions::from`
**inclusive**, deliberately, because they answer different questions: `after` means
"everything I did not see", `from` means "start where I stopped". VT-13 fixes
`next()`. Put together, `checkpoint.next()` is the resume idiom and it is correct
even when the very next position does not exist — because `from` is a threshold.
Under exact-seek semantics the same recipe stalls silently on any gapped log while
passing every rule in the suite, which is 55 today, not the 27 the RUNBOOK's
phase-4 body still cites.

One consequence for the code, which no clause names and which is where an adapter
author will pick up the wrong belief: `next()`'s own doc comment
(`event.rs:138-141`) says the method is *"only meaningful for adapters that
allocate positions densely"*. That is the exact belief that produces an exact-seek
`from`. ES-26 already quotes it — neutrally, as the caveat a reader of the resume
path runs into, alongside `projection.rs:84-93`'s instruction to "advance past"
the checkpoint by hand — and under VT-13 it is now false. The method is the resume
idiom on **every** adapter, and its soundness over gaps is ES-9's threshold
semantics rather than the adapter's density.

### 7. The CF-25 partition, and why "six" is in two documents

`RUNBOOK.md:3032` says ADR-0013 accepts "the remaining **six** axes";
`SPECIFICATION.md:260` says "The remaining six axes are accepted in the ADR that
lands this document." Because the two documents *agree*, RUNBOOK rule 5 gives no
arbitration and both need correcting.

Six is not arithmetic nonsense — it has a traceable derivation. §1.3's first
bullet takes position allocation out as **measured rather than accepted**, and
7 − 1 = 6. The defect is that §1.3's *second* bullet then names five `ES` clauses
spanning **four** axes and the exit criterion states the same split as
**exclusive** — "they are not part of this freeze and need no acceptance" — under
which the remainder is three, and four once ES-10 lifts. Two internally consistent
numbers, from two partitions, in two documents.

So this ADR states the partition in decision §2's table and takes the exclusive
reading, which is the exit criterion's own wording: **an axis is either carried by
a live `[PROVISIONAL]` clause naming it, or accepted here by name. Nothing is
both, and nothing is neither.** Four accepted, three carried.

**Batch shape, and the reading this ADR takes on CF-25's scope.** CF-25 is worded
*"before any port in this specification is declared `[FROZEN]`, every axis in the
portfolio table"*, and batch shape is a `ProjectionStore` axis while phase 4
freezes `EventStore`. The literal reading is taken — the axis is named and
accepted here, so it is not left claimed by neither phase — but the acceptance is
**pro forma and non-transferable**: it records that the axis has a *passing*
implementation at neither end — skeletons on both sides and a suite for neither —
and it certifies nothing about the batch shape, because no `EventStore` clause
depends on it and phase 4 has no authority over `ProjectionStore::Batch`. CF-25's
obligation on batch shape falls again in full at phase 6, and phase 6 may not
treat this row as discharged. CF-25 gains one sentence saying so, because an
acceptance a later phase must repeat is a signature rather than a decision, and
the difference should be in the clause rather than in an ADR nobody rereads.

### 8. The poll-count limitation, which is this ADR's and is claimed by no clause

The entire ES-10 lift is checked by **one rule**,
`nothing_below_an_observed_position_appears_later`. That rule creates two `append`
futures from one handle and polls them by hand in the order A, B, B, A, reading
after every step, so that the writer which started second is resumed first — the
reversal is what models the slow transaction that took a low number and published
it late, and CF-13 records that plain alternation was measured not to work.

Why a rule can *do* that is Rust-specific and worth one paragraph, because the
C#/JS intuition points the wrong way. A Rust future is **cold**: calling an `async
fn` runs none of its body and returns a value; nothing happens until something
calls `poll`. So constructing both futures is not starting two writers, it is
building two suspended state machines, and the rule owns the schedule completely
(`suite.rs:2766-2772`). `pin!` is what makes them pollable in place without a heap
allocation, since `poll` needs `Pin<&mut Self>`. The consequence is that this rule
needs **no executor, no thread and no clock** — which is what lets it be a
portable conformance rule at all under CF-20's no-`Send` fixture bound and CF-33's
prohibition on time. In C# the equivalent test would need a custom
`SynchronizationContext` to get the same determinism, because a `Task` is already
running by the time you hold it.

`Fixture` declares exactly three capability constants — `SECOND_HANDLE`
(`contract.rs:149`), `REOPEN` (`:161`) and `MID_BATCH_FAULT` (`:195`) — and none
of them is a poll budget. The rule therefore drains whatever the schedule left
unfinished, and its own comment states the consequence
(`suite.rs:2799-2800`): *"an adapter needing three polls is not let off. Asserting
'two polls was enough' would be over-specification."* That is the right call and
it has a price: **against a store whose `append` needs three polls, the
interleaving window never opens where the rule looks, and the rule cannot fail.**
Its strength varies silently with the adapter. `MemoryEventStore` passes it
trivially because its `append` body contains no `.await` at all — correctly, since
a store with no window cannot have a window bug — but the same mechanism means a
real store's window can sit outside the schedule.

This ADR settles it as follows, and settles it by naming the limitation rather
than by removing it.

**No poll budget is added at phase 4.** The obvious repair — a `POLL_BUDGET`
associated constant on `Fixture` that the schedule consumes — would be the first
capability that is not a yes/no answer, in a contract five of whose six clauses
(CF-16, CF-18, CF-19, CF-20, CF-21) are already `[FROZEN]`. And it sits close
enough to CF-33's line to need care: CF-33 forbids a rule from *asserting* on an
operation count, and a budget the schedule *consumes* is arguably not an
assertion — but that distinction is exactly the kind of reasoning CF-33 exists to
keep out of the suite, and it should be made by whoever owns the fixture contract,
with an adapter in front of them.

**The limitation is recorded in ES-10 itself**, so that the clause the freeze
rests on states the strength of the only rule that checks it. A freeze that does
not say that is a freeze over a rule whose power is unknown.

**The instrument that would settle it is named, and it is cheap**: a poll-padding
decorator over `PreCommitPositionStore` that inserts *n* `Pending` returns into
`append`, and the observation is whether the rule still rejects it. What that
decorator cannot supply is the right *n* — an author choosing *n* is the
reference-store failure mode with one more step — so the calibration waits for an
adapter with real I/O. **Owner: phase 10** (ADR-0024) rather than phase 8, because
what the calibration wants is a store whose `append` genuinely suspends more than
once, and phase 10 is where the `sqlx`-backed one lands; if phase 8's adapter
turns out to suspend, phase 8 may take it earlier. If it fires, ES-10 stays frozen
and **the rule** is what changes; the invariant is not in question, only the
instrument.

## Consequences

**Good.** The highest-ranked axis in the portfolio — the one the pressure test and
all six scenarios independently put first — now has a number rather than a
preference behind it, and the number came from a real PostgreSQL with two positive
controls that fired. ES-25 and ES-26 keep their ground.

**Good.** `AppendCondition::after` means something, and it is now stated exactly
once, at ES-10. VT-12 stays a pointer, which is the whole reason it survives as an
ID.

**Good.** `next()` stops returning a wrong answer through a signature that has a
way to say "I cannot", and it stops doing so at zero representational cost. The
blast radius was nil in practice; the reason to fix it is that a contract crate
should not lie in the one direction its type system had already made free.

**Bad, and the reason this ADR is long.** Freezing ES-10 *removes* a disclosure.
Anyone reading the specification after this change sees one fewer `[PROVISIONAL]`
marker and one more line in an ADR, and ADRs are history while the specification
is current truth. The mitigation is that ES-10's frozen text now carries the
acceptance, the frontier semantics and the rule's limitation in its own body —
three sentences that were not there — rather than relying on this document being
reread.

**Bad.** `head()` on an arm-C adapter does not satisfy read-your-own-writes, and
that is the single most surprising thing in this ADR for anyone who has used an
event store before. It is a real capability difference between adapters, it is
measured (0.688 ms to 4010.719 ms), and it is coupled to workloads outside the
application entirely.

**Bad.** The rule the whole lift rests on has a strength nobody has bounded.
That is stated rather than fixed, with the instrument named and an owner.

**Neutral.** Nothing in arm C pushes back toward the contract: no new information
on `AppendCondition`, no lock-key derivation, no change to `append`'s signature.
Arm B-tag is the arm that *would* have required a contract change — it must derive
a key from a type the adapter currently only forwards — and it is also the arm
that does not implement the clause. The mechanism that works is the one that
leaves the port alone, which is why phase 2 could own the measurement at all.

## Alternatives rejected

- **Leave ES-10 `[PROVISIONAL]`.** The conservative-looking option, and it is
  available: the marker's condition is a judgement about affordability and one
  could always want more evidence. Rejected because the marker names its own lift
  condition in terms and the condition is met, and a clause whose stated condition
  is satisfied but which stays provisional anyway teaches every future reader that
  the markers are decoration. ADR-0009 rejected the mirror-image move for the same
  reason.

- **Weaken ES-10 to a per-boundary invariant** and take arm B-tag's 0.935 at 64
  writers. Loses to the projection checkpoint (decision §3): a runner checkpoints on a
  global position covering boundaries it never reads, and a per-boundary invariant
  lets an event become visible below a checkpoint that has already passed it. The
  saving is real and the failure is silent, which is the wrong side of that trade.

- **Serialise position allocation** — arm A, or arm B-const's single constant
  lock. Both are correct and both are available today. They are rejected on the
  measurement: 0.062× and 0.033× of baseline at 64 clients, flat from 8 clients
  upward, which is the signature of a total serialisation point. They buy ES-10 by
  deleting the reason `happenstance-postgres` is in the workspace, which is to sit
  at the far end of an axis every other adapter shares. A fifth adapter with the
  same shape as the other four is not an instrument.

- **Declare `head()` to be `max(position)` and rule arm C non-conformant**, which
  would reopen the lift. Rejected because it is incoherent with ES-10 itself:
  `max(position)` under arm C names a position no read will yield, so a caller
  paginating from it reads nothing. ES-30's own words are "highest position
  currently **visible**", and the frontier is what those words denote wherever
  visibility is a predicate.

- **Add a `POLL_BUDGET` capability to `Fixture` now.** It is the direct repair for
  the one soft spot in this ADR's evidence. Rejected at phase 4 for three reasons:
  it would be the first non-boolean capability in a contract that is five-sixths
  frozen; it sits close to CF-33's prohibition and the distinction between a
  consumed budget and an asserted count deserves better than a passing paragraph;
  and its calibration needs an adapter that does not exist. Deferred to phase 10
  with the instrument named, which is the difference between deferring and
  forgetting.

- **Fix `next()` by widening to `u128` internally**, or by returning
  `SequencePosition` and documenting saturation. Both were available. The first
  doubles the type's size and destroys the niche that makes
  `Option<SequencePosition>` free; the second keeps a method whose entire purpose
  is signalling overflow and removes its ability to signal.

## What this ADR leaves open

Each with the observation that would refute it and the phase that makes the
observation. Nothing here is left open by omission.

- **Whether the global invariant is the one happenstance needs.** Closed *by
  decision* in decision §3, not by evidence, and the decision is reversible. **Refuted by:**
  a projection checkpoint that is boundary-scoped rather than global, which would
  remove the argument decision §3 rests on and make arm B-tag's 9% real money. **Owner:
  phase 6**, which freezes `ProjectionStore` and with it the checkpoint's shape.
  If that shape changes, this section reopens and needs its own ADR.

- **Whether a real Postgres adapter can pass the rule, and at what structural
  cost.** The mechanism is settled; its structural costs are not, and the
  experiment measured four SQL strategies rather than four implementations of
  `SendEventStore`. **Refuted by:** an adapter that cannot express arm C cleanly
  through `sqlx`, or whose frontier staleness is unacceptable under a real
  workload. **Owner: phase 10**, ADR-0024.

- **The strength of `nothing_below_an_observed_position_appears_later`.**
  **Refuted by:** a poll-padding decorator over `PreCommitPositionStore` that the
  rule fails to reject, calibrated against an adapter whose `append` genuinely
  takes more than two polls. **Owner: phase 10.** If it fires, the rule changes and
  ES-10 does not.

- **ES-38's rule.** The clause is `[FROZEN]` and
  `positions_are_not_reused_after_removal` cannot be written today: it needs a
  store that can remove events, `Fixture` declares no such capability, and the
  completeness instrument ES-38 names is CF-27, which is `[DEFERRED]` with nothing
  planned before phase 14. This ADR transcribes ES-38 into the port's
  documentation and records that its rule waits. **Owner: phase 14.** A `[FROZEN]`
  marker binds the design; it does not assert that anything checks it.

- **ES-35 (durability) and ES-40 (completeness)** stay `[PROVISIONAL]` and are not
  phase 4's. ES-35's falsifier is owned by phase 8; CF-27, which ES-40 is one
  decision with, is unplanned before phase 14.

- **ES-11 and ES-12** stay `[PROVISIONAL]` on the transport axis, which is empty
  at both ends. They are ADR-0011's to restate and nobody's to lift at phase 4.

## Amendments this decision owes the specification

Recorded rather than applied, because this run writes ADRs only and because two of
them touch `[FROZEN]` clauses, where this repository's rule is that a frozen clause
changes by ADR and not by edit. **No normative MUST is overturned by this ADR.**

**ES-10 — `[PROVISIONAL]` → `[FROZEN]`.** This ADR is the authority. Two separate
edits, and applying only the first leaves the clause contradicting itself.

*Edit one — the marker* (`SPECIFICATION.md:2364`). The whole
`**[PROVISIONAL — axis: **position allocation** … reopened with it.]**` block
becomes `**[FROZEN]**`, followed by three sentences the clause does not carry
today:

1. that freezing this clause moves the position-allocation axis **into**
   ADR-0013's CF-25 acceptance list rather than out of it, because this marker
   was that axis's disclosure;
2. that a conformant adapter buying the invariant with `xid8` +
   `pg_snapshot_xmin` reports a **frontier** from `head()`, therefore does not
   satisfy read-your-own-writes, and has staleness bounded by the longest open
   write transaction anywhere in the cluster — 0.688 ms unloaded and 4010.719 ms
   behind an unrelated five-second write in an unrelated database, both measured
   (`docs/experiments/position-visibility/results/staleness_pinned.txt`);
3. that `nothing_below_an_observed_position_appears_later` has a strength that
   varies with the adapter's poll shape, because `Fixture` cannot express a poll
   budget, and that the bounding instrument is owed by phase 10.

*Edit two — the body paragraph the marker's condition lives in*
(`SPECIFICATION.md:2372-2383`), which currently reads *"It is provisional on cost,
not on correctness … One affordable answer lifts this clause to `[FROZEN]` at
phase 4. Three unaffordable answers do not make the clause wrong…"*. A `[FROZEN]`
clause stating the condition under which it becomes frozen is a clause arguing
with itself. It becomes the past tense: the probe was run at phase 2, arm C is the
affordable answer, ES-25 and ES-26 are not reopened, and the sentence about what
three unaffordable answers would have meant is kept as *history* — because the
counterfactual is what tells a later reader that this freeze had a way to fail.

Everything else in the clause stands. In particular the `Rejects:` line — the
`nextval()` adapter and Wattline's nineteen milliseconds — is unchanged and is
what the freeze now protects unconditionally, and the "sole statement of the
visibility invariant" paragraph keeps VT-12 pointing here.

**ES-30 — the `Rule:` line only, and within it one parenthetical. `[FROZEN]`;
this ADR is the authority.** The clause names three rules; only
`head_is_the_highest_visible_position` is touched, and
`head_of_an_empty_store_is_none` and `head_advances_across_two_handles` are
unchanged.

- **Current text** (`SPECIFICATION.md:3311-3314`): ``head_is_the_highest_visible_position` **(new)** (append, then assert `head()` equals the position `append` returned)`
- **Required text**: ``head_is_the_highest_visible_position` **(new)** (append a batch holding at least one event a narrower default query would not match, read the whole store with `Query::all()`, and assert `head()` is not below the highest position the read yielded)`

**The clause's MUST is untouched** — "the highest position currently visible" is
correct as written and the frontier is what it denotes wherever visibility is a
predicate. The seeding requirement is part of the required text rather than a note
on it, because without it no implementation can fail the rule and CLAUDE.md's
decorative-rule bar is not met.

What ES-30's `Rejects:` set loses: **nothing**, and the reason is that the
attribution in the old parenthetical was already wrong. Against a single handle, a
cached last-written position returns exactly the position `append` returned — so
the *equality* form never rejected it either; `head_advances_across_two_handles`
is the rule that does, using CF-16's second handle under CF-19's obligation, and
that rule is untouched here. A default-query-scoped head is rejected by the
replacement, by construction, given the seeding above. And the
`backwards().limit(1)` workaround as a universal answer is hung on E2E-13 by
ES-30's own text, not on any of its three rules. Three rejected implementations,
three homes, all three still occupied.

The equality form is not recoverable by demanding quiescence, because arm C's
staleness is set by transactions no fixture owns — including transactions in
another database entirely — and CF-33 forbids the rule from waiting.

**CF-25 — one additive sentence. `[FROZEN]`; this ADR is the authority.** Add,
after the existing MUST at `SPECIFICATION.md:7096-7099`: *"An acceptance recorded
for an axis on which the frozen port does not sit is pro forma: it names the
exposure and does not discharge it for the port that does sit on that axis."*
Nothing is removed and no MUST is weakened — CF-25's `Rejects:` line names no axis
at all, it names four adapters of one storage shape being used to freeze
`EventStore`, so the sentence cannot subtract from what it was defending. Without
it, phase 6 inherits a discharged-looking batch-shape row it never examined, which
is the same failure CF-25 exists to prevent, one port over.

**§1.3, the census (`:218-221`).** `135 [FROZEN]` → **136**, `46 [PROVISIONAL]` →
**45**. The clause-ID total (193) and the normative total (191) are unchanged by
this ADR. **Apply in one pass with ADR-0012's decision on `MID_BATCH_FAULT`:** if
that ADR adds a CF clause the totals move too, and the census is the one count in
this document a human computes by reading. Two edits landing separately leave it
wrong in between.

**§1.3, the discharge split (`:239-262`).** The third bullet's "The remaining
**six** axes are accepted in the ADR that lands this document" becomes the
partition of decision §2's table: four accepted by ADR-0013 by name — position
allocation, async flavour, handle multiplicity, and batch shape pro forma — and
three carried by ES-11/ES-12, ES-35 and ES-40. The second bullet's "Five `ES`
clauses carry the residual exposure" becomes four and drops ES-10 from its list.
The first bullet's "position allocation is
measured, not accepted" is now **wrong in both halves' relationship**: it was
measured *and* it is now accepted, and the bullet must say both or the axis
vanishes from the ledger.

**`:362`** (the `EventStore` row of the port table) — "Five `ES` clauses hold the
residual and are `[PROVISIONAL]` for it (ES-10, ES-11, ES-12, ES-35, ES-40)"
becomes four, without ES-10, and the "four have a fixture instrument" sentence
keeps position allocation in its list while moving it to the accepted column.

**`:437-448`** (the honest caveat) — unchanged in substance and **must stay
unchanged**: seven axes, no adapter instrument at any far end. Only the sentence
apportioning which are fixture-instrumented and which are accepted needs to agree
with §1.3.

**§6.5's portfolio prose (`:7216-7221`)** — "ES-10 stays `[PROVISIONAL]` against
exactly that measurement" is now false. The measurement was made; the row's `Far
end exists` column stays **"Fixture yes, adapter no"**, and CF-26's `Rejects:` —
reading the fixture tick as coverage — applies with more force after the lift, not
less.

**§7.1 and §7.2 — regenerated, not edited.** ES-10's maturity cell at `:7666`
reads `PROVISIONAL` and must read `FROZEN`, but that row sits inside
`<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->` … `<!-- END GENERATED -->`
(`SPECIFICATION.md:7584-7823`). It is produced by `cargo xtask spec-trace --write`
from the parsed clauses, and the gate compares the committed region against a
freshly computed one on every run, so a hand edit is either redundant or a
conflict. **Run the tool.** §7.1's per-section summary moves in the same pass —
the `ES` row and the totals row both shift one clause from `[PROVISIONAL]` to
`[FROZEN]` — and §1.3's census is checked *against* that computation rather than
generated from it, which is the property that makes the two agreeing worth
anything. §7.3 through §7.6 are authored and are not touched.

### Owed to the code, not to the specification

Listed here because they are this decision's and would otherwise be nobody's.

- `event.rs:143` — `saturating_add` → `NonZeroU64::checked_add`, in the `match`
  form above so the function stays `const`. Unit test
  `position_next_signals_overflow`, which VT-13 names and which does not exist.
- `event.rs:138-141` — the doc comment claiming `next()` is *"only meaningful for
  adapters that allocate positions densely"* is false and is the belief that
  produces an exact-seek `from`. It becomes: this is the resume idiom on every
  adapter, and it is sound over gaps because `ReadOptions::from` is a threshold
  (ES-9), not a seek. This is VT-13's second MUST discharged, not ES-9's clause
  restated.
- `store.rs` — the visibility invariant, the pre-commit warning, and the
  frontier/read-your-own-writes caveat go on the port. ES-38's sentence goes there
  too, with its rule recorded as waiting on CF-27.
- `read_from_a_gap_position` is **not** claimed here. It is ES-9's owed rule, and
  VT-13 names it only parenthetically, as *"(ES-9's name for it)"*. ADR-0011 takes
  the twenty-nine-ID extension and claims ES-9 — but it schedules no owner for
  this rule, so as of this ADR the rule is named by two documents and owned by
  neither. It is listed here so that fact is written down before phase 4 closes,
  not to claim it. ADR-0010 §1's one-sided saboteur asymmetry for its sibling
  `reading_an_empty_store_yields_nothing` is already recorded at
  `SPECIFICATION.md:2334-2345`.

### Owed to the RUNBOOK

Recorded, not applied; the run that writes the code applies them.

- **`:3030-3032`** — the exit criterion's if-and-only-if is resolved in favour of
  lifting, so two edits land together: `:3030`'s "ES-10, ES-11, ES-12, ES-35 and
  ES-40 are already `[PROVISIONAL]` … not part of this freeze" drops ES-10, and
  `:3032`'s "ADR-0013 accepts the remaining **six** axes" becomes the partition
  above. Applying only the second leaves ES-10 counted on both sides.
- **`:2838`** — the position-visibility work item cites "VT-12, ES-10". VT-12 is
  `[NON-NORMATIVE]` and is a pointer; citing it as a source of the invariant is the
  exact duplication VT-12 exists to prevent. The same correction is owed at
  **`:445`**, where the ledger's "settled — VT-12, ES-10, ES-38" repeats it.
- **`:2843-2846`** — ES-9's item says exact-seek semantics stall "while passing all
  27 rules". The suite carries 55 event-store rules today.
- **`:2866-2877`** — "Name the owner of freezing CF-16 – CF-21" is stale: five of
  the six are already `[FROZEN]` and CF-17 is phase 8's. The poll-count half of
  that item is live, is verbatim correct, and is settled in
  [decision §8](#8-the-poll-count-limitation-which-is-this-adrs-and-is-claimed-by-no-clause)
  above rather than by adding a capability.
- **`:442`, and not this ADR's to apply.** The ledger's `head`/`count` row says
  ES-30 settled `head` as a **provided** method; ES-30 `[FROZEN]` says required.
  The clause wins under rule 5. Recorded here because ADR-0013 is what walked past
  it, and left to ADR-0012, which the same row names as its owner.
