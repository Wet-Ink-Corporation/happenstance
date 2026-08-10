---
id: adr-0016-the-wire-format
title: "ADR-0016: The format is happenstance's own, and an unknown version is refused before the message is read"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  The wire format is PRIVATE to happenstance, which is what makes every reversal in it
  free rather than breaking, and an unknown envelope version is refused before the
  message is read. Two frozen clauses had frozen vocabulary the code already moved past;
  several others are discharged by amending the clause rather than by satisfying its
  letter.
depends_on:
  - adr-0003-opaque-payloads
related:
  - adr-0012-append-shape-and-preconditions
  - reference-experiment-wire-format
source_paths:
  - docs/architecture/SPECIFICATION.md
  - crates/happenstance-core/tests/wire.rs
  - crates/happenstance-sync/src/wire.rs
last_reviewed: 2026-08-09
adr_id: ADR-0016
phase: 5
supersedes: []
superseded_by: null
---

# ADR-0016: The format is happenstance's own, two frozen clauses froze vocabulary the code had already moved past, and an unknown version is refused before the message is read

- **Status:** accepted. Every clause in §2.7 is discharged, but several are
  discharged by **amending the clause** rather than by satisfying its letter, and
  WF-1 is discharged by confirming that the thing it defers to does not exist.
- **Date:** 2026-08-09
- **Settles:** WF-2 – WF-12. WF-1 is settled in its *scope* half only; its
  interoperability half stays `[DEFERRED]` and the reason changes (§1). It does
  **not** settle the replication protocol: WF-8 lands a versioned envelope and
  nothing else — no message set, no negotiation, no `SyncError` extension. SY-30
  stays ADR-0027's; `SyncError` stays ADR-0026's.
- **Corrects, each against a measurement:** WF-2's two cost claims and WF-7's account
  of the same; **both** numbers in WF-6's `Rejects:` line; WF-4's and WF-6's frozen
  field names; WF-8's "first field" MUST, neither satisfiable nor violable in a
  self-describing format; WF-11's payload figure; WF-12's stated reason, wrong while
  its decision is right, **and its instrument, which is measurably decorative**;
  WF-1's account of what the DCB reference publishes, in which **both halves of the
  divergence are swapped**; and the source anchors in WF-2, WF-3, WF-4, WF-10 and
  WF-12, all five of which name lines that moved.
- **Rests on and does not settle:** [ADR-0003](0003-opaque-payloads.md). The payload
  stays opaque `Bytes` and `happenstance-core` still carries no `serde` by default.
  WF-11 adds one optional dependency *inside* the `serde` feature, priced in §10.
- **Extends:** [ADR-0012 §9](0012-append-shape-and-preconditions.md) (VT-30). The
  boundary moved into `Guard { query, after }` at commit `1b2a565`; WF-4's frozen
  text predates that and names the field the move deleted. VT-30 is itself still
  `[PROVISIONAL]`, which §2 records as a live dependency.
- **Leaves provisional, deliberately:** WF-11. Its `no_std` half **closed at phase 5,
  by measurement**; its buffering half is a property of serde's data model rather
  than of base64, so it falsifies human-readable payload encoding as a **category**
  and its owner is phase 9's Workers peer.

## The question, and the one it has quietly become

**What is the wire format, and whose format is it?** That is the queue's question
([`RUNBOOK.md:291`](../RUNBOOK.md)), and the second half answers the first: because
the format is happenstance's own, every fix here is free, and the only expensive
decisions left are about *cost* rather than compatibility.

The question it has quietly become is sharper. §2.7 was written before phase 4 ran,
and phase 4 moved two things it names. **VT-30 moved the append condition's boundary
into `Guard { query, after }`** (ADR-0012 §9, at `1b2a565`), so the field WF-4
freezes — `fail_if_events_match` — no longer exists as a field anywhere; it survives
only as `AppendCondition::new`'s parameter name (`append.rs:126`). **ADR-0014 gave
`EventId` private fields with a `store()` accessor**, recorded in VT-5's body at
`SPECIFICATION.md:770-772`, so the field WF-6 freezes — `origin` — is spelled `store`
in the code and in the clause's own neighbourhood.

So: when a `[FROZEN]` clause and the code disagree about a *noun*, which moves? The
repository has no rule for that case. Rule 3 (`RUNBOOK.md:50-52`) governs only *how*
such a clause changes — *"Changing a `[FROZEN]` clause takes a new ADR, not an edit"*
— and rule 5 (`:56`) governs a phase body against the specification and does not
reach code at all. This ADR uses rule 3's mechanism: the clause moves, by ADR, twice,
and §2 and §9 argue why each is a noun rather than an obligation. Saying instead that
"the clause wins and this ADR overrules it" would manufacture a tension the RUNBOOK
does not create, and would help rather than restrain the future reader who cites this
document for a change where the code moved because it was wrong.

One correction to the framing this ADR was handed: the brief said `store()` is named
by **VT-6**. It is `SPECIFICATION.md:770-772`, inside **VT-5**'s body; VT-6
(`:781-815`) never mentions the accessor. A wrong citation inside a document whose job
is fixing wrong citations would have been the worst possible defect.

## Context

### Where every number below comes from

The measurements are programs, and the programs are kept:
**[`docs/experiments/wire-format/`](../experiments/wire-format/README.md)**, outside
the workspace and outside the gate, per CLAUDE.md's description of that directory.
`cargo test -- --nocapture` from there regenerates every figure this document states,
seed included. **W1 – W7 is the shorthand this ADR uses for the seven questions**; the
mapping to test names is that directory's README §1, and every citation below names
the test rather than the label. W7 alone has no code and cannot have: it is a dated
reading of the DCB specification, and §1 says what it could not settle.

### What is already decided, and is transcription

**Ten of the twelve clauses are `[FROZEN]`** (`SPECIFICATION.md:8133`), so phase 5's
job is implementation and mistaking execution for design is the main hazard — the one
ADR-0015 named for its own seven. **ADR-0003 is the floor**, unchanged here.

**The private-mirror pattern is already the house style, in five modules.** Every
serde impl in `happenstance-core` is hand-written against a private `…Wire` struct
mirroring the real type field for field, so fields stay private and the encoding stays
explicit: `event.rs:538`, `query.rs:341`, `append.rs:237`, `identity.rs:182`,
`tag.rs:472` (no mirror — `Tag` and `Tags` serialise directly). `identity.rs:183`
states the division of labour: *"The derives ship here; the **encoding** is phase
5's."* That is the sentence this phase is cashing.

**WF-10 is already satisfied and needs no code.** Every `Deserialize` re-runs its
constructor's invariants today: `tag.rs:497-504`, `query.rs:368-373`,
`query.rs:385-392`, `append.rs:274-289`, `event.rs:551-556`, `tag.rs:484-489`. WF-10
costs four rules and a re-anchoring of its citations. **VT-19 and ADR-0015 §6 already
settled that a capacity limit must not reach `Deserialize`**; WF-9's decode half is
that decision from the wire side.

### What was open, and what closed it

**D1 is a defect, and the RUNBOOK says so** (`:3338-3342`). Phase 0 did not delete the
attributes, so it lands here. WF-2's and WF-7's `Rejects:` paragraphs both describe a
desynchronised stream; `w1_postcard_desynchronisation` measured something worse and
something milder at once.

Worse: **a lone `Event` does not round-trip in postcard at HEAD at all**, unless it
carries *both* tags and metadata.

```text
no tags, no metadata   7 bytes  [01 41 04 11 22 33 44]                 => Err(Hit the end of buffer)
tags, no metadata     18 bytes  [01 41 04 11 22 33 44 01 09 63 6f …]   => Err(Hit the end of buffer)
no tags, metadata     11 bytes  [01 41 04 11 22 33 44 01 02 de ad]     => Err(Hit the end of buffer)
tags AND metadata     22 bytes                                          => OK, equal
```

All four round-trip in `serde_json`. So `wire::round_trips_in_postcard` over the
all-defaults shape fails **loudly** at HEAD — which is where the RUNBOOK's exit
criterion 3 (*"a decode that returns the wrong value is a better regression test than a
decode that errors"*, `:3399-3401`) needs a neighbour in the buffer to be satisfiable
at all.

Milder, because the frightening version is the one people repeat: the silent wrong
value needs a **neighbour in the same buffer**, and the neighbour matters. Of the nine
tried after the 18-byte tagged `Event` — enumerated with their outcomes in the
experiment's README §3 — three decode to a wrong `Event` with no error, two decode to a
value *equal* to the original while silently consuming the neighbour, and four error.
The mechanism is the shape of the class: the skipped `metadata` makes the decoder read
the neighbour's `0x01` as the `Option` tag `Some` and the following `0x41` — the ASCII
`'A'` of its event type — as a length of 65, which a 70-byte payload satisfies. That
91-byte buffer decodes to an `Event` whose `metadata` is 65 bytes of the neighbour, no
error anywhere. The all-defaults `Event` before the *same* neighbour errors instead,
because with two fields skipped `tags` absorbs the misalignment first — so the
wrong-value case and the all-defaults case are two tests and neither substitutes for
the other.

**D6 is not what it says on the tin, and the difference decides a rule.**
`from_str::<AppendCondition>("{}")` already **fails** at HEAD with ``missing field
`guards` `` (`w2_option_query`); so do `"[]"` and `{"guards":[]}`, all three fixed by
ADR-0012's guard sequence. What still decodes to match-everything is one level deeper:

```text
{"guards":[{}]}              => Ok(AppendCondition { guards: [Guard { query: All, after: None }] })
{"guards":[{"query":null}]}  => Ok(AppendCondition { guards: [Guard { query: All, after: None }] })
```

and `AppendCondition::new(Query::all())` serialises today as
`{"guards":[{"query":null}]}` — the encoder's own output for the most destructive value
in the protocol is a guard object whose only key is a `null`. That is the input
`wire::empty_object_is_not_a_condition` must reject; a rule that only tries `"{}"`
rejects no implementation anyone would write.

**Whether the DCB reference has a published wire format: answered, and the answer is
no.** W7 fetched `EventStore.ts` from its canonical source and read the specification
page. The reference library contains **no serialisation code of any kind**, and the
specification's JSON snippets are labelled *"a **potential** JSON representation"*
beside *"implementations are not required to use the same terms or function/field
names"*. WF-1's phrase *"the reference implementation's published shape"* has no
referent — a finding, not a failure to measure. What W7 could **not** settle: whether
`dcb.events` has changed since the 2026-08-05 fetch recorded in
`docs/evaluation/research-dcb-spec.md`; what `JSON.stringify` would literally emit for
the reference's `Query` (not executed — the `{items: […]}` conclusion is read off the
object literal); and whether any *other* DCB implementation agrees with it. A
hypothetical reference *binary* encoding is moot, since the reference defines none.

## Decision

### 1. WF-1's scope half is confirmed; its interoperability half stays `[DEFERRED]`, and its account of the divergence is wrong in both directions

The format is private to happenstance. That is what makes every other decision here
free: WF-3 reverses a documented intent, WF-6 changes two encodings, WF-11 changes a
third, and none is a compatibility break, because there is nothing to be compatible
with.

**The clause stays `[DEFERRED]`, and the reason changes.** Its marker defers to *"a
bridge exercised against the DCB reference implementation's encoding"*. There is no
such encoding. That is a stronger reason to defer, not a weaker one: a bridge built
today would be built against an illustration the specification disclaims, and the first
real DCB peer would break it.

**Both halves of the recorded divergence are swapped**, and this correction is the one
most likely to be repeated if it is not fixed now. `SPECIFICATION.md:1875-1879` says the
reference emits a bare sequence and happenstance an `{items: […]}` value. Measured at
HEAD it is the other way round: **happenstance** is the unwrapped side (`Query::Items`
goes through `serialize_some`, `query.rs:380`, and no `items` field exists anywhere in
`query.rs`), **the reference** the wrapped one (`queryAll()` returns `{items: []}`). And
happenstance's refusal of `[]` is **semantic, not syntactic** — `Query`'s `Deserialize`
reads `Option::<Vec<QueryItem>>` (`query.rs:387`), `[]` takes the `Some` branch, and
`Query::from_items` returns `Err(InvalidQuery::NoItems)`. That mechanism has a shelf life
of one commit, which is why the amendment states it in the past tense.

The divergence WF-1 reaches for survives: **happenstance spells match-all as a value that
cannot syntactically collide with a filtered query** — `null` today, `"All"` after §7 —
**while the reference spells it as `{items: []}`, structurally identical to an illegal
empty filtered query and distinguished only by which function built it.**

### 2. WF-4 is amended: the wire field is `query`, because the MUST is about presence and not about spelling

`SPECIFICATION.md:1938-1941` requires each guard to encode `fail_if_events_match` and
`after`. At HEAD `GuardWire`'s field is `query` (`append.rs:246-250`). **The clause
changes; the code does not**, and it takes three legs, because "rename the field to
match the clause" is a one-line change that would otherwise win by being cheap.

**Leg one: the MUST is about presence.** The clause's own justification (`:1955-1960`)
is *"`after` is always present because E2E-37's ingest rule depends on seeing it … An
ingest policy can only refuse what it can see."* E2E-37 turns on `after` being a
store-local position with no referent at the receiver — `is_violated_by` compares raw
positions (`append.rs:205-214`, delegating to `Guard::is_violated_by` at `:219-233`,
where `position <= after` at `:230` is the comparison) — so `after: 288455` in hub
numbering names an unrelated event and the check passes vacuously. Every word of that
is about whether the field is **on the wire**; none is about what it is called. A
`GuardWire` emitting `query` and `after` unconditionally satisfies the obligation the
clause gives its reason for; one emitting `fail_if_events_match` and skipping `after`
when `None` satisfies the letter and fails E2E-37.

**Leg two: both literal-compliance routes cost more.** Renaming `Guard::query`
(`append.rs:114`) is a breaking change to a `pub` field ADR-0012 §9 deliberately made
readable downstream. `#[serde(rename = "fail_if_events_match")]` is cheaper and worse:
one name in the type and another on the wire, in a crate whose entire serde story is
the hand-written mirror precisely so the encoding is visible where it is produced.

**Leg three: the one positive-sounding argument is an argument against.**
`fail_if_events_match` matches the reference's `failIfEventsMatch` — confirmed by W7 —
and under WF-1 that is the *specific* pressure the clause exists to resist: a
compatibility patch arriving as tidiness. It does not even buy compatibility, since the
reference's `AppendCondition` holds one `Query`.

**A dependency this amendment carries.** VT-30 is still `[PROVISIONAL]`
(`SPECIFICATION.md:1781-1784`), both its named instruments unbuilt; §3.4's own prose
still calls it provisional (`:3425`). If VT-30 is falsified and the guard sequence
withdrawn, `query`/`after` goes with it and WF-4 returns to a single-guard spelling —
reproducing, one clause over, the defect this ADR exists to fix. That is the one event
that reopens this amendment, and it does not reopen the `after`-presence MUST, which is
E2E-37's and independent of the guard count.

**What WF-4 keeps** is the non-empty `guards` sequence, and the
`from_str::<AppendCondition>("{}")` sentence marked a **satisfied floor** rather than a
live obligation. What is added is the input that is not satisfied: `{"guards":[{}]}`.

### 3. D1's five `skip_serializing_if` attributes go, and WF-2 has both of its cost numbers wrong

The five attributes are deleted: `event.rs:578`, `:580`, `query.rs:352`, `:354`,
`append.rs:248`. That is D1 and WF-2 and is not in dispute; what is in dispute is what
the clause says it costs.

WF-2 says the skips cost *"nothing at all"* in postcard (`:1900-1902`). Measured
(`w1_postcard_desynchronisation`), restoring the **one** absent field on the tagged
`Event` takes it from 18 bytes to 19, and restoring **both** on the all-defaults
`Event` from 7 to 9 — one byte per absent field. In JSON, *"a bare `Event` goes from 35
to 61"* is right only for a four-byte payload whose every byte renders as one decimal
digit; the same shape carrying `11 22 33 44` goes 39 → 65, a zero-length payload
28 → 54, and the tagged W1 event 60 → 76 because only `metadata` is restored. The
stable claim is **+26 bytes**. The trade is unchanged and still correct — the
diagnostic format pays 26 bytes, the bulk format one byte per absent field, and the
format stops being undecodable — but a frozen clause cited as rationale must not carry
a number wrong in both directions.

### 4. `#[serde(default)]` goes with them, and that is this ADR's preference rather than a precedence rule

`#[serde(default)]` is deleted at the same five sites, reversing `RUNBOOK.md:3356`
(*"Keep `#[serde(default)]` so existing JSON still parses"*). **The reversal is this
ADR's own preference, not a precedence rule it inherits.** Rule 5 (`:56`) resolves a
phase body against the *specification*, and the only specification sentence that
disagreed with `:3356` was WF-12's "fifth hazard" claim — which this ADR withdraws.
After that withdrawal no clause forbids the attribute, so rule 5 has nothing to bite
on, and dressing a preference as a precedence rule is the failure mode §13 convicts.

**What `#[serde(default)]` actually does, measured** (`w4_serde_default_is_inert`). It
never touches serialisation: postcard encodings are byte-identical across a plain
struct, a per-field `default` and a whole-struct `default` — `[1,0]` for the default
value, `[1,1,7]` for the non-default one, in all three. Only `skip_serializing_if`
shrinks the encoding (`[1]`) and only it breaks the round trip: in a struct+trailer
decode the other three consume exactly `struct_len` bytes and leave the trailer intact,
while the skip form fails with `DeserializeBadOption`, the decoder reading the trailer's
first byte `0xEF` as an `Option` discriminant. **So the attribute is not a positional
hazard and does not violate WF-2's MUST**, which forbids exactly the thing measured not
to happen. Its only effect is on **decode**, and only in a self-describing format: on a
non-`Option` field, `from_str` of `{"id":42}` errors with ``missing field `count` ``
without it and succeeds as `{id:42, count:0}` with it, while in postcard that path is
unreachable — running out of bytes is a hard parse error, and a truncated buffer is
rejected with `DeserializeUnexpectedEnd` identically either way.

**The argument for deleting it, stated as the preference it is.** What it buys is a
decoder that accepts a document its own encoder cannot produce — measured, a
whole-struct `default` struct emits `{"id":1,"count":null}` always and yet accepts
`{"id":42}`. That is a **second, undocumented format**: one shape written, a strictly
larger set accepted, nothing describing the difference, and a peer's bug surviving a
round trip to reappear as a default value three hops later. One format per direction
beats schema tolerance in a private format with no published shape and no deployed
peers. That is the whole of the argument, and it is derived from nothing frozen.

**And WF-12's stated reason must be corrected rather than allowed to ride along.**
`SPECIFICATION.md:2134-2137` calls `ReadOptions`'s whole-struct `#[serde(default)]` *"a
fifth `skip_serializing_if`-shaped hazard … the same positional-desynchronisation class
as WF-2."* At HEAD `ReadOptionsWire` (`query.rs:394-401`) carries `#[serde(rename =
"ReadOptions", default)]` and **no per-field `skip_serializing_if` at all**, so W4's
isolation is exactly the shape in the tree and refutes the claim.

### 5. The presence obligation binds the encoder only, and buys less than it looks

**The deletion closes the read side at three of the five sites and changes nothing at
the other two.** `Event::metadata` and `Guard::after` are `Option`-typed, and serde's
derive routes a missing field through `missing_field`, which succeeds for any type
whose `Deserialize` calls `deserialize_option` — **with no attribute at all**. Measured
(`w2_option_query`) on a mirror carrying no serde attributes whatsoever: it emits
`{"a":1,"opt":null,"seq":[],"tags":[]}` and yet decodes `{"a":1,"seq":[],"tags":[]}` to
`Ok(opt: None)`, while rejecting an absent `seq` or `tags` with ``missing field``. On
this ADR's own externally tagged guard mirror, `{"guards":[{"query":"All"}]}` decodes to
`Ok(after: None)`. It is the same `missing_field` mechanism that makes `{"guards":[{}]}`
decode to match-everything at HEAD, and it survives this fix on the `Option` fields.

**So the repository owner's decision, recorded rather than left to inference: the
presence obligation binds the ENCODER ONLY.** WF-2's and WF-4's MUSTs are amended to say
so rather than to grow a read-side half, because the asymmetry is safe where it could
bite: a missing `after` decodes to `None`, and `None` checks the *whole* log, so it
fails **closed** — a spurious rejection, never a spurious acceptance. Closing it would
cost a hand-written visitor at every `Option` field to buy a stricter error for a
document nothing produces. This is the same argument §4 rejected three paragraphs
earlier, applied to the same fact and reaching the opposite answer; the Consequences
disclose that rather than leaving it to be noticed.

### 6. WF-7: the two-format matrix is the instrument, and its "maximum-size values" half was unowned

WF-7 requires the round-trip tests to run in `serde_json` **and** `postcard`, *"both
over every envelope shape including all-defaults and maximum-size values"*
(`:2012-2013`). It is decided here, before the table that lists its rules.

**Its `Rejects:` repeats the claim WF-2's does, and W1 refutes both the same way.** WF-7
says D1's attributes *"desynchronise the stream in postcard"*. Measured solo, they do
not: three of an `Event`'s four shapes error **loudly**, and the silent wrong value
needs a specific neighbour. The clause's *conclusion* — that a JSON-only matrix
certifies a broken format — is exactly right and is strengthened by the correction,
because all four shapes round-trip in `serde_json`.

**That same fact scopes the negative control, and this is the trap.** A mirror carrying
`skip_serializing_if` is the wrong implementation WF-2 and WF-7 exist to reject — but it
round-trips **cleanly** in `serde_json`, which is precisely why WF-7 demands a second
format. So the control belongs to `wire::round_trips_in_postcard` alone. Obliging
`wire::round_trips_in_json` to carry one too would write an unsatisfiable obligation
into a `[FROZEN]` clause on this ADR's authority; the amendments scope it to the
postcard rule, and WF-2's `Rule:` line names only that rule anyway.

**`wire::round_trips_in_json` needs a job the postcard rule does not already do, and the
"maximum-size values" clause is it.** Every shape round-trips in JSON at HEAD and will
after the fix, so a JSON round trip over small values rejects nothing. What is
JSON-specific and size-specific is §10's base64 arm: a payload encoder that truncates,
or a decoder that grew a length check, shows at `MIN_SUPPORTED_EVENT_DATA_LEN` and
nowhere smaller. So the rule carries an `Event` whose `data` is 65,536 bytes, and shares
its wrong implementation with `wire::decode_accepts_an_over_capacity_value`. Below that
size it is a regression fixture, stated as one rather than dressed up.

### 7. WF-3: `Query` becomes an externally tagged enum, and the rule that checks it must test `{"guards":[{}]}`

`Query` is encoded as an externally tagged enum with two variants, `All` (unit) and
`Items` (sequence). This reverses a documented intent — `query.rs:377` says *"`None` is
the match-all query; `Some(items)` is a filtered one"* — and the reversal is the
clause's, taken at `[FROZEN]`.

**The defect, measured** (`w2_option_query`). `to_string(&Some(Query::all()))` is
`"null"`. `to_string(&None::<Query>)` is `"null"`. They are equal.
`from_str::<Option<Query>>("null")` is `Ok(None)` while `from_str::<Query>("null")` is
`Ok(All)`, so `Some(Query::All)` **does not survive a round trip**. And
`Some(Query::Items(…))` is byte-identical to bare `Query::Items(…)`.

The Rust reason matters for a reader coming from C#, where the instinct is the opposite.
In serde's data model `Option` is not a wrapper a self-describing format renders:
`serialize_some(v)` is *transparent*, so `Some(T)` produces exactly the bytes `T`
produces. C#'s `Nullable<T>` is a distinct runtime type carrying its own `bool`, so `T?`
and `T` are never the same value; here they are the same *bytes*, and the distinction
lives only in the Rust type that was asked for. So encoding a two-variant enum through
`Option` does not merely lose `Some(All)` against `None` — it makes `All` occupy the
format's null, the value every buggy peer emits by accident.

**The proposed encoding, measured on both formats.**

```text
JSON      QueryWire::All        = "All"
JSON      QueryWire::Items      = {"Items":[{"types":["Enrolled"],"tags":["course:c1"]}]}
JSON      None::<QueryWire>     = null                     null != "All"      => distinguishable
postcard  QueryWire::All        = [00]
postcard  QueryWire::Items(one) = [01 01 01 08 45 6e 72 6f 6c 6c 65 64 01 09 63 6f 75 72 73 65 3a 63 31]
postcard  None::<QueryWire>     = [00]
postcard  Some(QueryWire::All)  = [01 00]                  [00] != [01 00]    => distinguishable
```

Both formats round-trip it. Externally rather than internally tagged, for the reason the
clause already gives: an internally tagged representation requires a self-describing
format and postcard is not one.

**The rule this obliges.** `wire::empty_object_is_not_a_condition` must assert on
`{"guards":[{}]}` and `{"guards":[{"query":null}]}`, not on `"{}"`. The wrong
implementation it must reject is the one in the tree today: a `Guard` whose `query`
decodes an absent or null value to `Query::All`. Measured against the externally tagged
mirror, `{"guards":[{}]}` becomes ``missing field `query` `` and
`{"guards":[{"query":null}]}` becomes `expected value`.

### 8. WF-5 is already implemented, and the phase body does not know it

`SequencedEventWire` (`event.rs:614-619`) carries `position`, `id`, `recorded_at` and
`event`, all four unconditional, no `skip_serializing_if` and no `default`. ADR-0014
landed it. **WF-5 costs this phase a round-trip test and nothing else.** It is a
decision rather than a note because the phase body's work list does not mention WF-5 at
all, so a reader working the list would either skip the clause or re-derive its
implementation. `wire::sequenced_event_round_trips` is still owed — it is what stops the
next person deciding `id` is recomputable at the receiver, which E2E-42's A—B—C topology
refutes.

### 9. WF-6: the field is `store`, the binary half is already done, and both numbers in the clause's `Rejects:` line are wrong

**The field is `store`, and the clause moves.** `EventIdWire` is `{ store: StoreId,
position: SequencePosition }` (`identity.rs:208-211`), mirroring `EventId`'s own private
fields (`:97-100`) and its `store()` accessor (`:110-113`), which VT-5's body names at
`SPECIFICATION.md:770-772`. WF-6's `origin` would put a third name on a value that
already has two agreeing ones, on the wire, where nothing else in the crate uses the
word. Same disposition as WF-4, same three legs, plus one: `identity.rs:183-188`
promises phase 5 *"changes an encoding rather than a type"*, and renaming
`EventId::store` is changing a type ADR-0014 froze.

**The binary half is already satisfied — which is a reason to write the assertion, not
to skip it.** `StoreId` serialises as `[u8; 16]` (`identity.rs:194-198`) and postcard
renders sixteen raw bytes with no length prefix. "Passes on arrival" is the wrong thing
to say at the moment the phase is adding the branch that can break it. **The wrong
implementation both WF-6 rules exist to reject is an inverted `is_human_readable`
branch**, invisible to every round-trip rule in this document, because hex and base64 are
symmetric and encode-then-decode agrees whichever arm it takes. Measured
(`decorative_inverted_human_readable_branch`) on a deliberately inverted newtype over
`[de ad be ef]`: JSON renders `[222,173,190,239]`, postcard renders the ASCII of
`"3q2+7w=="`, and **both** round trips return `Ok(true)`. So
`wire::store_id_encodes_as_hex_in_json` must assert the JSON value is the 32-character
string and `wire::store_id_encodes_as_bytes_in_postcard` must assert sixteen raw bytes
and no ASCII hex — the only two instruments that can see an inversion.

**What is owed is the human-readable half**, branching on `is_human_readable`: 32
lowercase hex digits without separators, which is what `StoreId`'s `Display` already
produces (`identity.rs:64`, measured `0f1e2d3c4b5a69788796a5b4c3d2e1f0`). The consequence
the clause states stands: the same value has two encodings, so human-readability becomes
part of the format's identity.

**Both numbers in the `Rejects:` line are wrong, and one is unreachable.** `:1993-1995`
claims a byte-sequence `StoreId` in JSON *"produces a 32-element array of integers … and
costs four times the bytes."* Measured (`w3_store_id_json_size`): a **sixteen**-element
array, because `StoreId` is `[u8; 16]` (`identity.rs:44`); and **1.7353×**, 59 bytes
against 34 of quoted hex, with an all-`0xff` ceiling of **1.9118×** and an all-`0x01`
floor of **0.9706×**, *cheaper* than hex. Four times is unreachable. The decision
survives — what carries it is illegibility rather than size — but a frozen clause cited
as rationale must not carry a count wrong by two and a size wrong by two the other way.

### 10. WF-11: base64 is taken, priced at zero new crates, and one half of the falsifier closes

`Event::data` and `Event::metadata` encode as standard-alphabet base64 in human-readable
formats and as raw byte strings otherwise. **`base64` is added as an optional dependency
of `happenstance-core`, reachable only through the `serde` feature** — `optional = true,
default-features = false`, the feature line becoming `serde = ["dep:serde",
"dep:base64", "bytes/serde", "serde/alloc"]`.

**The measurement** (`w5_payload_encodings`), on Turnstile's seat map — *"340 KB of
GeoJSON in `Event::data`"* (`docs/scenarios/README.md:1367`), read as 340 KiB = 348,160
bytes, generated by `xorshift64*` from the seed written into the test file, first sixteen
bytes `[e7 e3 a8 ea 0b 28 6c 7f e0 ab f9 1c 87 19 71 e4]`, byte-sum mod 2³² =
44,443,899:

| Encoding | Bytes | Size | Ratio to raw |
|---|---|---|---|
| `serde_json` of `bytes::Bytes` (array of decimal integers) | 1,243,464 | 1214.32 KiB | 3.5715× |
| base64 as a JSON string, quotes included | 464,218 | 453.34 KiB | 1.3333× |
| lowercase hex as a JSON string, quotes included | 696,322 | 680.00 KiB | 2.0000× |
| postcard of `bytes::Bytes` | 348,163 | 340.00 KiB | 1.0000× |

Base64-in-JSON is **2.6786× smaller** than the array of integers. One unit note, because
the clause is wrong by more than rounding: *"roughly 1.3 MB"* overstates by ~4.6%
decimal and ~9.6% binary — 1,243,464 bytes is **1.243 MB** (10⁶) and **1.186 MiB** (2²⁰).
This ADR states KiB throughout.

**The rules that check the branch.** As in §9, an inverted `is_human_readable` branch
round-trips cleanly in both formats. `wire::payload_is_base64_in_json` must assert the
JSON value is the *string* `"3q2+7w=="` for `[de ad be ef]`, and
`wire::payload_is_raw_in_postcard` must assert the postcard bytes contain `de ad be ef`
and no ASCII base64.

**What `base64` costs, measured, and it is the cheapest form a dependency has.** `base64
0.22.1` is **already in this workspace's graph** — `cargo tree --workspace -i base64
--depth 1` reports `sqlx-core` and `sqlx-postgres` pulling it for
`happenstance-postgres` — and `cargo tree -p base64` is **one line**: no transitive
dependencies at all. So the workspace pays **zero new crates**, and a consumer taking
`happenstance-core` with the `serde` feature and no sqlx pays **exactly one leaf crate
with no dependencies of its own**. That is the number ADR-0003 should be priced against,
and it answers the licence question by implication rather than a fresh run: the crate is
already in the graph `cargo deny` checks in the gate today. The **semver reading is the
second leg**: ADR-0003's protected asset is its own Consequences' *"semver surface"*, and
`base64` appears in no public signature — so a major version of it is a patch-level
change here, exactly the property `bytes` does *not* have.

**The version pinned is `"0.22"`, and the reason belongs in the manifest.** `0.23.1`
exists; taking it would put two majors of `base64` in the graph, because sqlx pins
`0.22`, and `deny.toml`'s `[bans] multiple-versions = "warn"` would add a *warning* to
the gate rather than fail it — worse than a failure, because nobody reads warnings.
Without the reason recorded, the next person upgrades it for tidiness and silently
reverses the measurement above.

**The marker stays `[PROVISIONAL]`, with a restated falsifier**, because its halves are
now in different states.

The **`no_std` half is closed, by measurement.**
`docs/experiments/wire-format/no-std-wasm-check/` is a `#![no_std]` + `alloc` crate whose
only dependency is `base64 = { version = "0.22", default-features = false, features =
["alloc"] }`, using `STANDARD.encode` and `STANDARD.decode`; `cargo check --target
wasm32-unknown-unknown` reports `Checking base64 v0.22.1` and `Finished`, no warnings.
That half leaves the falsifier rather than staying in it with a note attached, because a
falsifier that has been answered is not a falsifier.

The **buffering half is not about base64 at all** — and this paragraph is **read from
serde's `Serializer` API rather than measured**, which the reader is owed because its
neighbour is now a measurement. `serialize_str` takes a `&str`, and serde's data model
offers no streaming entry point for a human-readable string; `collect_str` takes a
`Display` and still writes the whole rendering into the output. So **any** human-readable
payload encoding must materialise the whole payload; hex has the same property. The
falsifier therefore falsifies **human-readable payload encoding as a category**, and if
it fires, WF-11's MUST becomes unsatisfiable on that peer and the clause is re-scoped to
formats rather than peers. Its owner is **phase 9**'s Workers peer: recorded as phase
5's, it would be a marker phase 5 can neither lift nor fail — the shape ADR-0015 §7 names
and CF-38 exists to prevent.

### 11. WF-8: a minimal generic envelope in `happenstance-sync`, with a hand-written `Deserialize`, and a witness that proves it is hand-written

What ships is a `wire` module in `happenstance-sync` containing exactly four things: a
`FORMAT_VERSION` constant, `Envelope<T> { format_version: u16, message: T }` generic in
`T`, a **hand-written** `Deserialize` that reads the version and refuses before touching
the message, and its **own** error enum.

**`SyncError` is not extended.** It is ADR-0026's enum (`peer.rs:391-421`). A
wire-version refusal is a decoding failure, not a runner failure, and adding a variant
would take a phase-13 design decision inside an encoding phase. **SY-30 is untouched.**
**Generic in `T`, and not an enum of message kinds**, because an `enum Message { Push(…),
Pull(…) }` would put the message set beside the version check and the message set is
exactly what phase 13 has not designed.

**Why the hand-written `Deserialize` is structurally necessary.** This is the
Rust-specific part and the C#/JS instinct gets it wrong invisibly. A
`#[derive(Deserialize)]` generates a visitor that reads **every** field into a local
`Option` and only then constructs the struct. There is no hook between "field one has
been read" and "field two is about to be read": the derive offers `default`,
`deny_unknown_fields` and `flatten`, and none is "stop here and decide". So a derived
`Envelope<T>` **decodes the message before anything examines the version** — the partial
decode WF-8's MUST NOT forbids. In C# you would reach for a `JsonConverter` that peeks
the first token; the hand-written `Deserialize` is that, and it is the only way to get
it, because serde's derive is a code generator with no interception point.

**The MUST is the defect, not the rule name — and the rule that would have caught a
derive is not the one either name suggests.** Three measured findings.

- **"First field" is not assertable in a self-describing format, so the MUST is amended
  rather than the rule renamed.** Key order carries no meaning in JSON:
  `{"message":null,"format_version":999}` decodes to `Ok` (`w6_envelope_varint`).
  `serde_json`'s writer emits declaration order, but that is an assertion about
  `serde_json`. So the MUST is scoped: **positional formats keep the positional
  obligation**, and **self-describing formats discharge it by an explicit pre-message
  check.**
- **The byte-position spelling was rejected on a varint measurement.** Postcard encodes a
  `u16` as a varint: `1 => [01]`, `127 => [7f]`, `128 => [80 01]`, `384 => [80 03]`,
  `65535 => [ff ff 03]`. What holds for every value is that `take_from_bytes::<u16>`
  succeeds on the front of the buffer and returns a remainder identical regardless of
  version — measured `[03 aa bb cc]` for versions 1 and 384 alike. So "first byte" is not
  the spelling. **It is not the rule either**, which is the sharper finding.
- **Neither the error nor the framing distinguishes a derive from the hand-written impl**
  (`decorative_envelope_witness_count`). `EnvelopeDerived<T>` — derive plus a version
  check *after* decoding — against `EnvelopeHand<T>`: at version 999 both return `Err` in
  JSON and in postcard, and the framing is byte-identical: `[01 07]` at version 1,
  `[80 03 07]` at 384, same `take_from_bytes::<u16>` value and `[07]` remainder. So the
  version-refusal rule and the front-of-buffer rule **both pass against the
  implementation WF-8's MUST NOT forbids.** The one separating observation is a
  **witness**: a `T` whose `Deserialize` records that it ran. At an unknown version the
  derived envelope calls it **once** in both formats; the hand-written envelope calls it
  **zero** times. So `wire::version_is_readable_before_the_message` is specified as that
  witness test — which matters because the hand-written impl is this ADR's own
  recommendation, and as previously specified nothing would have caught someone replacing
  it with a derive.

**What WF-8 rejects is unchanged and is its best argument:** per-type versioning costs
bytes on every event in the log and still cannot express a change to the *relationship*
between two types — for instance VT-30's move of `after` into `Guard`, which changed no
single type's shape in isolation.

### 12. `RUNBOOK.md:3402-3404` is two exit criteria wearing one sentence, and its second half is already done

The criterion reads: *"The envelope carries a format version, and a peer built at an
older bound rejects an over-bound message with a variant that means 'refused, park this'
rather than a parse failure."* Those are two criteria about two clauses. The first is
**WF-8** (§11). The second is **WF-9**, and it is about a **store limit**, not a format
version — a bound change is explicitly *not* a wire change, which is WF-9's whole MUST,
so a version bump is the wrong instrument. **WF-9's second half already exists**:
`AppendError::ExceedsStoreLimit { limit, len }` at `error.rs:237-244` (ADR-0015 §8), and
`append_reports_exceeded_store_limits` at `suite.rs:4282`, registered at
`registry.rs:187`. Both verified in the tree. **No peer is required.**

What WF-9 still owes is the **decode** half, and the clause must say what ceiling the
value exceeds or the rule is decorative: `happenstance-core` enforces no store ceiling —
`MIN_SUPPORTED_EVENT_DATA_LEN` (65,536 bytes, `limits.rs:22`) and its neighbours are
floors an adapter must *support*, not limits the contract crate applies — so a rule that
decodes a large `Event` and asserts `Ok` passes against every implementation, including a
lazy one. The amendment names the ceiling and the wrong implementation: a `Deserialize`
that grew a length check, a live hazard precisely because WF-10 tells the same impls to
re-run their constructor's invariants.

### 13. WF-12: `ReadOptions` leaves the wire, on the clause's decision, not on its stated reason, and not on its stated instrument

`ReadOptions` loses `Serialize` and `Deserialize`; `query.rs:394-425` is deleted. The
argument that carries it (`:2127-2132`) is sound: `from` is a store-local position with
no meaning at the sender, and `backwards`/`limit` are traversal options a peer has no
business setting. The argument that does **not** carry it is the `#[serde(default)]`
sentence, refuted in §4.

**The clause's stated instrument is measurably decorative.** A `compile_fail` doctest
passes whenever the snippet fails to compile **for any reason**. Measured — four
spellings of the same assertion, doctests D1 – D4 in
`docs/experiments/wire-format/src/lib.rs`, against `happenstance-core` at HEAD with
`std,serde,memory` — the honest one **FAILED** with *"Test compiled successfully, but
it's marked `compile_fail`"*, correctly reporting that `ReadOptions` is serialisable
today, while a type-name typo (`ReadOptionz`), a misspelt trait (`serde::Serialise`) and
a wrong crate path (`happenstance::ReadOptions`) **all reported ok**. Three of four green
against a false assertion. `RUNBOOK.md:3112-3117` already recorded half of this, finding
`compile_fail,E0080` to be *"the same check wearing a claim"*. And
`crates/happenstance-core/tests/` cannot host a `compile_fail` assertion at all: an
integration test that fails to compile fails the build.

**The replacement is an inherent-impl detection**, worth a Rust digression for a reader
who would reach for specialisation here and find it unstable. Rust resolves an
**inherent** associated constant before a trait one, so give a marker type both:

```rust
use core::marker::PhantomData;

pub struct Detect<T>(PhantomData<T>);

/// The default answer, available for every `T`.
pub trait NotSerialize { const IS_SERIALIZE: bool = false; }
impl<T> NotSerialize for Detect<T> {}

/// The inherent answer, available only where the bound holds — and inherent
/// items win, which is the whole mechanism.
impl<T: serde::Serialize> Detect<T> { pub const IS_SERIALIZE: bool = true; }

/// The `Deserialize` half is NOT a transcription of the line above it.
pub trait NotDeserialize { const IS_DESERIALIZE: bool = false; }
impl<T> NotDeserialize for Detect<T> {}
impl<T: serde::de::DeserializeOwned> Detect<T> { pub const IS_DESERIALIZE: bool = true; }

const _: () = assert!(
    !Detect::<happenstance_core::ReadOptions>::IS_SERIALIZE,
    "WF-12: ReadOptions must not implement Serialize",
);
```

`Detect::<T>::IS_SERIALIZE` names the inherent constant when `T: Serialize` and falls
back to the trait's `false` when it does not; `const _` turns the answer into a compile
error at const-evaluation. Measured at HEAD: `error[E0080]: evaluation panicked: WF-12:
ReadOptions must not implement Serialize`. With the type name misspelt: `error[E0425]:
cannot find type ReadOptionz in crate happenstance_core` — a **build failure**, not a
green test, which is the property the doctest lacks. Against a non-`Serialize` type:
compiles clean.

**The `Deserialize` half needs a different bound, and it is the Rust-specific step not to
hand-wave.** `Deserialize<'de>` is generic over the lifetime of the data it borrows
*from*, so `serde::Deserialize` is not a trait you can name without one: `impl<T:
serde::Deserialize> Detect<T>` is `error[E0106]: missing lifetime specifier`, measured.
What is wanted is a **higher-ranked trait bound** — `for<'de> serde::Deserialize<'de>`,
read as "for *every* lifetime `'de`, `T: Deserialize<'de>`". It is a quantifier over
lifetimes, the construct a C# or TypeScript reader has no syntax for; the compiler
suggests `impl<'a, T: Deserialize<'a>>` beside it, which is the wrong repair — that makes
the *impl* generic over one caller-chosen lifetime rather than requiring the bound to
hold for all of them. `serde::de::DeserializeOwned` is shorthand for exactly the HRTB and
is the spelling the test should use. Measured, in a throwaway crate at this revision:
both spellings compile and both answer correctly — `true` for a derived type, `false` for
a type with no impl. One honest limit, also measured: a **borrowing** `Deserialize` (a
type holding `&'a str`) does not fall back to `false`; it is a hard *"implementation of
`Deserialize` is not general enough"*. That does not bite here, because `ReadOptions`
owns every field, but it means `Detect` answers *"does this deserialise from owned
data"*, not *"does this implement `Deserialize` at all"*.

**So WF-12's `Rule:` line is amended too**: the rule is a **const-evaluation assertion in
an integration test**, not a compile test. A `const _` is not a `#[test]`, so `cargo test
--list` cannot assert it by name, and what enforces it is that the target must **build**,
which the `proof.rs` step already does in order to list its neighbours.

### 14. D12 gets an xtask manifest lint, not a WF-13 — and §1.3's census does not move

**D12 was closed at phase 0**, by commit `927d291` (2026-08-06), whose message says
*"**D12** — `serde/alloc` arrived by accident … Now stated."* The fix is at
`crates/happenstance-core/Cargo.toml:36` and `CHANGELOG.md:1091` records it.

**Its failure is not reproducible at HEAD**, which is the whole argument for the
instrument. `bytes` 1.12.1 — the version `Cargo.lock:92-93` pins — declares its own
optional `serde` with `features = ["alloc"]` (read from `bytes-1.12.1/Cargo.toml:120-124`
in the local registry, not inferred), so deleting `serde/alloc` today **would** still
compile, *by the manifest* — **not measured**, because no build was run with the token
removed, and because the point of the lint is that nothing observable changes. A
**behavioural** rule would therefore be decorative: it observes nothing in every
configuration on this lockfile, and starts observing something only on the day `bytes`
changes its mind — the day the lint was supposed to have prevented.

The failure mode is a manifest edit, so the check is a manifest assertion — **the CF-32
pattern**, whose own `Rule:` line records that its check *"found nothing on its first run
and that is expected"* (`SPECIFICATION.md:7835-7841`). **And it is not a clause.** CF-32
earned one because it constrains the *testkit's packaging contract*, which adapter
authors depend on; D12's lint constrains one feature line in one crate against one
transitive accident, and is a gate step in the shape `lints::testkit_version` already has
(`xtask/src/lints.rs:285`, wired at `main.rs:413-430`). WF-13 would put a clause in a
frozen section to describe a `grep`.

**Therefore §1.3's census is unchanged: 200 clause IDs, 198 normative, 139 `[FROZEN]`, 49
`[PROVISIONAL]`, 10 `[DEFERRED]`, two `[NON-NORMATIVE]`** (`SPECIFICATION.md:219-221`,
matching §7.1's totals row at `:8138`). This ADR adds no clause, removes none and moves
no marker. **Recomputed by hand under ADR-0015 §13's rule. Unchanged.**

### 15. `cargo xtask spec-trace` is fixed in this phase, and the fix is three changes, not one

`backticked_idents` (`xtask/src/spec_trace.rs:1379-1392`) accepts a backticked chunk only
if every character is an ASCII lowercase letter, an underscore or a digit. A `::` fails
that test, so every `::`-qualified name a clause mentions is **silently dropped**. Its own
neighbourhood knows this — `:1323-1328` records that the same defect dropped
`mutation_coverage::every_rule_has_a_mutant` and that *"a clause naming one parsed with an
empty rule list and §7.2 rendered a cell that looked checked"*. **Ten of the twelve WF
clauses name nothing but `wire::`-qualified tests**, so WF-1 – WF-8, WF-10 and WF-11 parse
to an empty rule list and render their own prose, claiming nothing.

**WF-9 and VT-19 are the two that lie**, and §7.2's reading conventions record it at
`SPECIFICATION.md:8104-8107`. Both name `append_reports_exceeded_store_limits` beside
`wire::` rules; the plain one survives, is found in the registry, and gets no `†`. WF-9's
row at `:8193` and VT-19's at `:8164` therefore read as fully checked, with the rules
covering their wire halves not merely unwritten but **absent from the row**. §6 already
records the obligation this violates (`:8017-8023`). The conventions document the defect
honestly and then ask the reader to work around it — *"Read those two clauses, not their
rows"* — which is a note where an instrument belongs.

**The fix is three changes, and the first alone turns the gate red.** That matters more
than the fix does: a phase landing only the obvious half breaks a `cargo xtask ci` step
and cannot repair it by doing more of the work the clauses ask for.

**One — `backticked_idents` accepts `:` as a path character, and keeps the whole qualified
string.** A `module::name` chunk parses to `wire::query_all_is_unambiguous`, not to
`query_all_is_unambiguous`. The prefix is load-bearing twice: it routes the name to the
right resolution source in change two, and a bare last segment would collide silently with
a suite rule of the same name, because `collect_rules` yields unqualified names.

**Two — a second resolution source, consulted only for `wire::`-prefixed names.** Check 4
(`:527-539`) resolves every `WF-`/`VT-`/`ES-` clause's rule names against
`collect_rules(SUITE)`, and `SUITE` is `crates/happenstance-testkit/src/suite.rs` alone
(`:69`); `RULE_FILES` (`:85-89`) is three files, every one inside `happenstance-testkit`.
Neither wire test file is in either list. **So change one on its own makes check 4 report
WF-1 – WF-11 and VT-19 as naming rules that do not exist, and writing the seventeen tests
does not clear it.** The second source parses `#[test] fn` names out of
`crates/happenstance-core/tests/wire.rs` and `crates/happenstance-sync/tests/wire.rs`,
qualifying each with its enclosing modules, so `mod wire { #[test] fn
query_all_is_unambiguous() … }` yields `wire::query_all_is_unambiguous`. Check 4 consults
it for **any** name carrying the `wire::` prefix, whatever clause family names it — which
is what makes VT-19 work as well as WF-1 – WF-11. The inner `mod wire` is not decoration:
it is the trick `tests/mutation_coverage.rs` already uses so `cargo test --list` prints the
qualified name the clauses cite, and `proof.rs` asserts on that listing.

**The second source feeds `rule_cell` too, and that is not a separate change but the same
one.** `rule_cell` (`:897-919`) daggers a name against exactly the set check 4 resolves
against, and it is handed `known_rules` today. Wiring the new source into check 4 alone
would leave seventeen *written* tests rendering `†` under a legend (`:8096-8097`) that
defines `†` as *"was looked for in `suite.rs` and not found … it must be written"* — which
would replace the silence §15 is fixing with a false statement, and would falsify this
section's own closing sentence. So the function grows **two** sets rather than one:
`known_rules`, the suite's own rules, which is what check 6 keeps sweeping; and
`resolvable = known_rules ∪ wire_rules`, which is what check 4 and `rule_cell` consult. The
legend is amended in the same change to say what the checker then actually does.

**Three — check 6 (`:553-563`) keeps iterating over the suite set only.** It requires every
rule in the suite to be claimed by a clause or retired by one. Sweeping the wire tests into
that set would demand that every helper `#[test]` in `wire.rs` be claimed by a clause, and
that is not the obligation: a conformance rule is something an *adapter* must pass, and a
round trip of this crate's own encoding is not an adapter obligation — §6 says so.
Resolution is one-way: clauses may name wire tests; wire tests need not be named by
clauses.

**`wire::` must not be added to the `elsewhere` trigger list** (`:1329-1330`) — the point
of the fix is that `wire::` names become resolvable, so an unwritten one must render `†`
rather than the clause's own prose. That makes the reading-conventions paragraph false, and
it is amended below.

**WF-12 stays `elsewhere`, and after this ADR it does so by accident.** It is marked by the
phrase "compile test", and §13 rewrites that line to *"a const-evaluation assertion …
**not** a compile test"* — the phrase survives only as a negation. The outcome is right and
the mechanism is not: `read_options_is_not_serialisable` is a `const _` and not a `#[test]`,
so no resolution source can ever find it and it must stay `elsewhere` permanently. So the
amended `Rule:` line keeps those two words deliberately, `spec_trace.rs:1329-1330` gains a
comment recording that WF-12 depends on them, and WF-12's row (`:8196`) joins the
regeneration prediction below, because its `Rule:` text changes entirely.

**A `Rule:` field may backtick rule names and nothing else.** `rules_of` (`:1305-1341`)
hands the **whole** field to `backticked_idents`, so any backticked all-lowercase token
containing an underscore becomes a name the checker will try to resolve — and after change
one it will try harder, not less. `skip_serializing_if`, `serde_json` and
`version_is_the_first_field` are all such tokens, and this ADR's own first draft put all
three inside `Rule:` amendments, where they would have produced up to four
`names rule X, which is not in suite.rs` problems that no test could ever clear. Below,
`version_is_the_first_field` and serde_json are unbackticked and the attribute is written
out in full; the amendments say so at each site, rather than leaving the next author to
rediscover why a token lost its backticks. This is the same convention `retires_of`'s doc comment
(`:1354-1371`) already records for `Retires:`, one field over; the difference is that
`Retires:` documents it and `Rule:` does not, so `rules_of` gains the matching comment.
Where the attribute itself must be named, the full spelling `#[serde(skip_serializing_if =
"…")]` is safe, because `#` is not a path character in any version of the parser.

**And this phase must not ship the fix without the list it exposes:** the moment `::`
resolves, all eighteen distinct `wire::` rules the WF clauses name become visible in §7.2,
including eleven no generated row has ever shown. The table under "Owed to the code"
enumerates every one against the wrong implementation it rejects, so the regenerated
section renders no `†` nobody owns.

## Consequences

**Good.** The format is decodable. At HEAD a lone `Event` with no tags — what every doctest
in the crate builds — cannot be read back in postcard at all. The crate had **354 lines** of
hand-written wire format across five modules and zero round-trip assertions; after this
phase it has a proptest over both a self-describing and a non-self-describing format. And
the most destructive value in the protocol stops being the value a buggy peer produces by
accident. Three frozen clauses stop carrying wrong numbers and one stops carrying a
decorative instrument.

**Bad, and this is the sharpest cost, because the reason is the one this ADR gave for the
opposite decision.** §4 deletes `#[serde(default)]` because it buys "a decoder that accepts
a document its own encoder cannot produce" — a second, undocumented format. §5 then accepts
exactly that at `Event::metadata` and `Guard::after`: measured,
`{"guards":[{"query":"All"}]}` decodes to `Ok(after: None)` against an encoder that always
writes `after`. **The stated reason for the deletion is the reason the deletion is declined
at two of the five sites**, and since the deletion rests on this ADR's own preference rather
than on anything frozen, the preference is self-limited and says so here rather than leaving
a reader to find it. The two cases are not equally bad — the survivor is on `Option` fields
only and fails **closed**, so the omission can only cost a spurious rejection — but "less
bad" is not "consistent". The residual cost is that "every field is always present" is true
of what happenstance writes and not of what it accepts.

**Bad, but narrower than it looks.** Deleting the five attributes leaves
`wire::round_trips_in_postcard` with no *current* type that can fail it. It does not make
the rule decorative: the wrong implementation it rejects is this ADR's own starting point,
so it is a regression test, and the negative control is belt-and-braces rather than the
whole of its power below maximum size. The control is nevertheless named and listed in
`xtask/src/proof.rs`, because a test nothing asserts by name is a test someone deletes as
dead code.

**Bad.** `base64`'s *graph* cost is measured — zero new crates for the workspace, one leaf
crate for a downstream consumer, and a `no_std` + `alloc` build that checks clean on
`wasm32-unknown-unknown` — but nobody measured its **compiled size** or its **build time**
on that target. Adding the first new dependency to `happenstance-core` since the crate
existed on a graph argument alone is still the thing a reviewer should push hardest on, and
if a constrained peer cannot take the size, the decision reverses to hex rather than the
clause bending.

**Bad.** WF-8 freezes a version field against a protocol nobody has designed, and
`w6_envelope_varint` measured that even a `u16` is not free — postcard spends one, two or
three bytes on it. If phase 13 negotiates per connection rather than per message, the
field is dead weight on every message and removing it is a format break.

**Bad.** Two `[FROZEN]` clauses have their nouns changed here, and what a future reader
sees is a precedent: *the clause moved because the code had already moved*. Both
amendments are ADR output rather than edits, both preserve every MUST, and both name the
alternative and its cost — but WF-4's new noun tracks **VT-30, which is still
`[PROVISIONAL]`** (§2). The failure mode to guard against is somebody citing this ADR for
a change where the code moved because it was **wrong**, and neither amendment is that.

**Neutral.** No signature changes, no port changes, no new error variant, `SyncError`
untouched — which is what keeps phase 5 schedulable anywhere between phase 4 and phase 12,
the property `RUNBOOK.md:3326-3332` claims for it.

## Alternatives rejected

Each is argued where it was rejected; this is the index, not a second copy.

- **Rename `Guard::query` to `Guard::fail_if_events_match`,** or `#[serde(rename = …)]`,
  and likewise `rename = "origin"` on `EventIdWire::store` (§2, §9).
- **Keep `#[serde(default)]`,** which `RUNBOOK.md:3356` asks for (§4): rejected on W4 and
  on this ADR's own preference rather than on a clause.
- **Close the read side properly, with a hand-written mirror deserialiser at every `Option`
  field** (§5) — the honest alternative, and the one that would make "every field is always
  present" true in both directions. Rejected by the repository owner.
- **Add WF-13 for D12** (§14): a behavioural rule would pass in every configuration forever
  and start failing on the day it was meant to have prevented.
- **Hex rather than base64 for WF-11** (§10): 696,322 bytes against base64's 464,218. Its
  real advantage — no dependency, a hex encoder being fifteen lines — is the argument to
  revisit if compiled size proves unaffordable.
- **Leave `Bytes` as an array of decimal integers,** the cheap alternative
  `RUNBOOK.md:3379-3382` offers. Rejected on 1,243,464 bytes and 3.5715×.
- **Extend `SyncError` with an `UnknownFormatVersion` variant** (§11): a phase-13 design
  decision inside an encoding phase, where the envelope's own error enum costs six lines.
- **A derived `Deserialize` on `Envelope<T>` with a post-hoc version check** — the
  alternative that nearly escaped, since only §11's witness test rejects it. Likewise
  **asserting `version_is_the_first_field` literally**.
- **Keep `read_options_is_not_serialisable` as a `compile_fail` doctest** (§13): three of
  four spellings report green while `ReadOptions` remains fully serialisable. A `trybuild`
  snapshot is the other repair and pulls in a dependency phase 6 owns.
- **Write `wire::empty_object_is_not_a_condition` against `"{}"`,** as WF-4 literally
  specifies. Measured, `"{}"`, `"[]"` and `{"guards":[]}` all already fail at HEAD.
- **Fix `spec-trace` in a later phase** (§15). It is why the defect survived: it is nobody's
  clause. Rejected because phase 5 makes it acute — ten of the twelve clauses it hides are
  this phase's.

## What this ADR leaves open

- ~~**Whether a `no_std` base64 build actually exists.**~~ **Closed at phase 5, by
  measurement** — `docs/experiments/wire-format/no-std-wasm-check/` checks clean on
  `wasm32-unknown-unknown` (§10). Recorded as closed rather than deleted, because §10's
  marker text and WF-11's falsifier both changed on the strength of it. What is **not**
  closed is compiled size and build time on that target, which is the whole of the fourth
  **Bad.** above.

- **Whether decision §15's `spec-trace` fix can be made green in one phase.** The three
  changes are specified from a reading of `xtask/src/spec_trace.rs` at HEAD; **no
  implementation was run in this pass.** Change one alone is known to turn the gate red for
  WF-1 – WF-11 and VT-19, and changes two and three are what is claimed to clear it.
  **Refuted by:** `cargo xtask spec-trace` still failing after all three land and the
  seventeen wire tests are written. If it cannot be made green, **decision §15 is
  withdrawn**: `wire::` returns to the `elsewhere` trigger list, the
  `SPECIFICATION.md:8098-8107` amendment is **not** applied, §7.2's reading conventions
  stand as written, and the defect reopens for a later phase with the failure recorded.
  **Owner: phase 5's own implementation run.**

- **Whether human-readable payload encoding is available on a memory-limited peer at all.**
  This one is **read from serde's `Serializer` API rather than measured** — the same
  qualifier §10's buffering paragraph carries, stated here because its neighbour above is
  now a measurement and the asymmetry would otherwise read as an omission. **Refuted by:** a
  Workers peer that must forward a payload it cannot buffer, which makes WF-11's MUST
  *unsatisfiable on that peer* and forces the clause to be re-scoped to formats rather than
  peers. **Owner: phase 9.**

- **Whether the DCB reference acquires a published wire format.** W7 found none and could
  not check whether `dcb.events` has changed since 2026-08-05. **Refuted by:** a DCB
  implementation that publishes an encoding. **Owner: phase 13**, per `RUNBOOK.md:546`.

- **What the sync message set is, and therefore what `FORMAT_VERSION = 1` names.** **Refuted
  by:** a phase-13 design in which versions are negotiated per connection rather than
  carried per message. **Owner: phase 13.**

- **Whether the negative controls that replace D1's attributes survive.** **Refuted by:**
  `cargo xtask ci` staying green after a negative control is deleted from
  `crates/happenstance-core/tests/wire.rs`. **Owner: phase 5**, by the `proof.rs` refactor
  under "Owed to xtask".

- **Whether `wire::round_trips_in_postcard` should be a proptest, a table, or both.**
  `RUNBOOK.md:3384-3393` asks for a proptest; W1 shows the interesting failures *at HEAD*
  need a specific neighbour in the buffer, which a generator will not reliably produce.
  **Refuted by:** the proptest passing against the negative-control mirror struct, which
  would mean the generator never reaches a sparse shape and the table is load-bearing.
  **Owner: phase 5.**

## Amendments this decision owes the specification

Recorded rather than applied, because this run writes the ADR only and because most touch
`[FROZEN]` clauses (`RUNBOOK.md:50-52`). **Where a clause below is marked frozen, this ADR
is the authority.** Each entry gives the actionable text; the argument is in the decision
section cited and is not repeated.

**No normative MUST is *reversed* here.** Two are **scoped to serialisation** (WF-2 and
WF-4, §5). One is **scoped by format** (WF-8, §11). One **gains two refusal obligations**
(WF-4: `{"guards":[{}]}` and `{"guards":[{"query":null}]}` MUST fail — the only behavioural
change these amendments make). Two **change a noun** (WF-4's `fail_if_events_match` →
`query`, WF-6's `origin` → `store`).

**WF-1 — `[DEFERRED]`; stays deferred** (§1). Two edits.

- **`:1856-1859`, the marker.** Add: *the DCB specification and its reference TypeScript
  library publish **no** wire format: `EventStore.ts` contains no serialisation code, and
  the specification's JSON snippets are labelled a "potential JSON representation" beside an
  explicit disclaimer that "implementations are not required to use the same terms or
  function/field names". There is nothing to build a bridge against, which makes the
  deferral stronger rather than weaker (ADR-0016 §1, reading of 2026-08-05).*
- **`:1875-1879`, the recorded divergence** (*"The reference emits a query as a bare sequence
  … where happenstance cannot parse `[]` at all"*) **becomes**: *"Both halves of this were
  recorded backwards. **happenstance** emits a bare sequence — `Query::Items` goes through
  `serialize_some` (`query.rs:380`) — while **the reference** is the `{items: […]}` side:
  its `Query` is `{ items: QueryItem[]; matchesEvent(…); merge(…) }` and `queryAll()`
  returns `{items: []}`. happenstance did not fail to *parse* `[]` before ADR-0016 §7
  either: it parsed to an empty item list and rejected it semantically through
  `Query::from_items`. Under the externally tagged encoding `[]` is refused by the
  representation itself, which strengthens the divergence — happenstance spells match-all as
  a value that cannot syntactically collide with a filtered query, the reference as `{items:
  []}`, structurally identical to an illegal empty filtered query. Three further facts a
  bridge needs: the reference's `QueryItem` tags are bare opaque strings, not key/value
  `Tags`; its `Query` carries methods and is not serialisable without a step somebody must
  define; and its `AppendCondition` is a single `{failIfEventsMatch, after?}` guard rather
  than a sequence."*

**WF-2 — `[FROZEN]`** (§3, §5). Five edits.

- **`:1883-1885`, the MUST.** Add: *"**This obligation binds the encoder.** A `Deserialize`
  that accepts an absent `Option` field does not violate it: serde's `missing_field`
  succeeds for any type whose `Deserialize` calls `deserialize_option`, with or without
  `#[serde(default)]` (measured, ADR-0016 §5), so the read side is asymmetric at
  `Event::metadata` and `Guard::after`. The asymmetry is deliberate and fails closed — a
  missing `after` decodes to `None`, which checks the whole log. `#[serde(default)]` is
  nevertheless deleted everywhere, on ADR-0016 §4's own preference."*
- **`:1891-1892`, the five attribute anchors** — `event.rs:342`, `:344`, `query.rs:281`,
  `:283`, `append.rs:121` **become** `event.rs:578`, `:580`, `query.rs:352`, `:354`,
  `append.rs:248`.
- **`:1900-1903`, the cost sentence** (*"…from 35 to 61 — and nothing at all in postcard"*)
  **becomes**: *"The cost is **+26 bytes in JSON** for the two restored fields —
  `,"tags":[]` is 10 and `,"metadata":null` is 16 — which for a bare `Event` with a
  four-byte single-digit payload is 35 → 61, for the same shape carrying `11 22 33 44` is
  39 → 65, and for a zero-length payload 28 → 54; and **one byte per absent field in
  postcard** — measured, the tagged single-field case is 18 → 19 and the all-defaults
  two-field case 7 → 9. That is still the correct direction to trade."*
- **`:1893-1898`, the `Rejects:` paragraph.** Add: *"Measured: a **lone** `Event` fails to
  round-trip in postcard for three of its four shapes, erroring with `Hit the end of buffer`
  at 7, 18 and 11 bytes; only the all-fields-present shape (22 bytes) survives. The
  **silent** wrong value needs a neighbouring value in the same buffer, and the neighbour
  matters: of nine tried after the tags-only `Event`, three decode to a wrong `Event` with
  no error, two decode to a value equal to the original while consuming the neighbour, and
  four error. The rule needs both a solo shape and a framed pair."*
- **`:1888-1889`, the `Rule:` line** — append the negative-control sentence given under WF-7,
  **with its two deliberate spellings intact** (§15). It applies unchanged here because this
  line names only `wire::round_trips_in_postcard`, which is the rule the control belongs to.

**WF-3 — `[FROZEN]`.** One citation edit; the MUST and the argument are untouched.
`(`query.rs:304-321`)` at `:1916` becomes `(`query.rs:375-392`)`, and *"`query.rs:306`
states the intent"* at `:1925` becomes `query.rs:377`.

**WF-4 — `[FROZEN]`** (§2, §7). Three edits.

- **`:1938-1941`, the MUST** **becomes**: *"…Each guard MUST **serialise** both `query` and
  `after`, both always written, with `after` an explicit `Option`; as in WF-2 the presence
  obligation binds the encoder, and a decoder that accepts an absent `after` fails closed to
  `None`. `serde_json::from_str::<AppendCondition>("{}")` MUST fail — satisfied since
  ADR-0012's guard sequence landed, and retained as a floor rather than a live obligation —
  **and so MUST `{"guards":[{}]}` and `{"guards":[{"query":null}]}`, which are the inputs
  the rule actually tests.** The field is named `query` because VT-30 moved the boundary
  into `Guard { query, after }` (ADR-0012 §9), and this clause's earlier
  `fail_if_events_match` named a field that move deleted; the MUST is about presence
  (E2E-37) and not about spelling. **VT-30 is itself `[PROVISIONAL]` (`:1781-1784`) with
  both instruments unbuilt, so this noun tracks a provisional clause's landed
  implementation: falsifying VT-30 withdraws the guard sequence and returns this clause to a
  single-guard spelling. It does not reopen the `after`-presence obligation, which is
  E2E-37's and independent of the guard count."*
- **`:1947-1953`, the `Rejects:` opening** **becomes**: *"the input that still decodes to
  match-everything after ADR-0012's guard sequence landed. Measured at HEAD, `"{}"`, `"[]"`
  and `{"guards":[]}` all already error. What is **not** closed is `{"guards":[{}]}`, which
  returns `Ok(AppendCondition { guards: [Guard { query: All, after: None }] })`, as does
  `{"guards":[{"query":null}]}` — and `AppendCondition::new(Query::all())` serialises today
  as exactly that (`append.rs:246-250`). It succeeds only because `Query`'s `Deserialize` is
  `Option`-shaped; once WF-3 makes `Query` explicitly tagged it becomes a hard error
  (measured: ``missing field `query` `` and `expected value`), and the two changes must land
  in one commit."* The `after` paragraph at `:1955-1960` keeps its argument and re-anchors
  one citation: `is_violated_by` is at **`append.rs:205-214`**, delegating to
  `Guard::is_violated_by` at **`:219-233`**, where `position <= after` at `:230` is the
  comparison — not `append.rs:95-107`.
- **`:1944-1945`, the `Rule:` line** — drop **`wire::condition_round_trips`**. It rejects
  nothing that `wire::round_trips_in_json` and `wire::round_trips_in_postcard` do not
  already reject — both are specified "over every envelope shape", and `AppendCondition` is
  one. Dropped rather than left for §7.2 to render as a `†` nobody owns (§15).

**WF-5 — `[FROZEN]`; no change** (§8). Recorded so the phase does not re-derive it: the MUST
is **already satisfied** by `SequencedEventWire` (`event.rs:614-619`), landed by ADR-0014.
What is owed is `wire::sequenced_event_round_trips` and nothing else.

**WF-6 — `[FROZEN]`** (§9). Two edits.

- **`:1984-1987`** (*"`EventId` MUST encode as a struct of `origin` and `position`"*)
  **becomes**: *"…`EventId` MUST encode as a struct of **`store`** and `position`. **The
  field is `store` because that is `EventId`'s own field and accessor
  (`identity.rs:97-100`, `:110-113`), named as such by VT-5's body at `:770-772`.** The
  binary half is already satisfied at HEAD: `StoreId` serialises as `[u8; 16]`
  (`identity.rs:194-198`) and postcard renders sixteen raw bytes with no length prefix.
  **The wrong implementation both rules exist to reject is an inverted `is_human_readable`
  branch, invisible to every round-trip rule** — measured, an inverted branch round-trips
  cleanly in both formats — so `wire::store_id_encodes_as_hex_in_json` asserts the JSON
  value is the 32-character string and `wire::store_id_encodes_as_bytes_in_postcard` asserts
  sixteen raw bytes and no ASCII hex."*
- **`:1993-1995`** (*"a 32-element array of integers … four times the bytes"*) **becomes**:
  *"…a **sixteen**-element array of integers — `StoreId` is `[u8; 16]` (`identity.rs:44`),
  one element per byte — that no operator can read. Measured: 59 bytes against 34 of quoted
  hex, a factor of **1.7353**; the arithmetic maximum, an all-`0xff` id, is 65 bytes =
  1.9118×, and an all-`0x01` id is 33 bytes = 0.9706×, i.e. **cheaper** than hex. "Four
  times the bytes" is not reachable, and the argument that carries this clause is
  illegibility rather than size."* The UUID half (`:1995-1998`) is untouched.

**WF-7 — `[FROZEN]`** (§6). Two edits.

- **`:2015-2020`, the `Rejects:`** (*"…desynchronise the stream in postcard"*) **becomes**:
  *"…and in postcard are **undecodable for three of an `Event`'s four shapes** — `Hit the
  end of buffer` at 7, 18 and 11 bytes — and silently decode to a **wrong** value when a
  neighbour follows them in the buffer (measured). A JSON-only matrix certifies a broken
  format either way; the correction strengthens the clause, because all four shapes
  round-trip in `serde_json`."*
- **`:2012-2013`, the `Rule:` line.** Add: *"After ADR-0016 §3 no type in
  `happenstance-core` can fail these rules, because every wire struct writes every field.
  **`wire::round_trips_in_postcard` is therefore paired with a negative control** — a mirror
  struct carrying `#[serde(skip_serializing_if = "…")]`, asserted to fail the same round
  trip — and that control is the whole of its discriminating power below maximum size:
  deleting it is deleting the rule. **`wire::round_trips_in_json` takes no such control, and
  must not be given one:** the same mirror round-trips cleanly in serde_json — measured, all
  four `Event` shapes do — which is the very fact this clause exists to state. Its own job is
  the **maximum-size** half, shared with WF-9's decode rule: an `Event` whose `data` is
  `MIN_SUPPORTED_EVENT_DATA_LEN` (65,536 bytes, `limits.rs:22`), which is where a truncating
  payload encoder or a `Deserialize` that grew a length check first shows."* The postcard
  sentence is appended to WF-2's `Rule:` line.

  Two spellings in that text are deliberate and must survive editing, for the reason §15
  gives: **serde_json is unbackticked** and the attribute is written in full rather than as
  a bare `skip_serializing_if`, because a `Rule:` field may backtick rule names and nothing
  else. Backticking either would make `cargo xtask spec-trace` report a rule that no
  resolution source can supply and no test can write — in the two clauses whose rules this
  ADR exists to make resolvable. `MIN_SUPPORTED_EVENT_DATA_LEN` is safe as it stands: the
  parser rejects uppercase.

**WF-8 — `[FROZEN]`** (§11). Three edits.

- **`:2027-2030`, the MUST** **becomes**: *"Every replication message MUST carry a
  `format_version`, and a receiver MUST read and check it **before any part of the message
  is decoded**. In a positional format the version MUST be the first field, so that
  `take_from_bytes::<u16>` on the front of the buffer yields it; in a self-describing format
  field order carries no meaning — measured, `{"message":null,"format_version":999}` decodes
  to `Ok` — so the obligation there is discharged by an explicit check in a hand-written
  `Deserialize` and not by position. The version MUST be bumped whenever the shape of any
  wire type changes. A receiver that does not implement a version MUST refuse the entire
  message with a distinguishable error and MUST NOT attempt a partial decode."*
- **`:2039-2042`, the `Rejects:`** **becomes**: *"Also rejects, **in a positional format**, a
  version field placed anywhere but first: a decoder that has already consumed two fields
  incorrectly cannot recover to read a version, so a trailing version is a version nobody
  can read when they need it. In a self-describing format the equivalent refusal is a
  decoder that reads the message before checking the version — key order is not observable
  there, so position is not the property and a pre-message check is."*
- **`:2033-2034`, the `Rule:` line** **becomes**:
  *"`wire::rejects_an_unknown_format_version`;
  `wire::version_is_readable_before_the_message`, replacing the unwritten
  wire::version_is_the_first_field — **unbackticked deliberately**, because a `Rule:`
  field may backtick rule names and nothing else (§15).
  **The replacement is a witness test, not a byte-layout test**: measured, a derived
  `Deserialize` with a post-hoc version check returns the same `Err` in both formats *and*
  produces byte-identical postcard framing — `[01 07]` at version 1, `[80 03 07]` at 384,
  same `take_from_bytes::<u16>` value and `[07]` remainder — so neither the error nor the
  front-of-buffer remainder distinguishes it from the hand-written impl this clause
  requires. The rule decodes an `Envelope<Witness>` at an unknown version, where `Witness`'s
  `Deserialize` records that it ran, and asserts the count is **zero** in both formats; the
  derive records **one**. The byte-position spelling was rejected separately, because a
  `u16` varint is one, two or three bytes."* **`Retires:` is the other disposition and is
  declined**: that field disposes of a rule that *exists* — `retired_rules` reads it
  against `collect_rules`, and CF-38 discharges it by deleting the rule in the same change
  — and this one was never written, so the line would name nothing, could never be
  discharged, and would falsify the two live statements that no `Retires:` field survives
  in the document (`SPECIFICATION.md:7978`, `:8549-8553`).

**WF-9 — `[FROZEN]`; no change to the MUST** (§12). One addition, **appended to the
`Rejects:` paragraph at `:2060-2065`** rather than to the `Rule:` line at `:2057-2058` —
it describes a wrong implementation, which is that field's job, and `Rejects:` is also the
field `rules_of` never reads (§15). The `AppendError::ExceedsStoreLimit` half is **already
implemented and already checked** (`error.rs:237-244`; `append_reports_exceeded_store_limits`
at `suite.rs:4282`, registered at `registry.rs:187`). Add: *"The rule decodes an `Event`
whose `data` exceeds
`MIN_SUPPORTED_EVENT_DATA_LEN` (65,536 bytes, `limits.rs:22`) — a floor adapters must
support, not a limit the contract crate applies — and asserts `Ok`. The wrong implementation
it rejects is a `Deserialize` that grew a length check, a live hazard precisely because
WF-10 asks the same impls to re-run their constructor's invariants; that implementation is
written into `crates/happenstance-core/tests/wire.rs` as a negative control."*

**WF-10 — `[FROZEN]`.** One citation edit at `:2088-2091`: *"(`tag.rs:317-324`
re-canonicalises, `query.rs:297-302` re-validates through `QueryItem::new`)"* becomes
*"(`tag.rs:497-504` re-canonicalises, `query.rs:368-373` re-validates through
`QueryItem::new`, `query.rs:385-392` routes `Query` through `from_items`, and
`append.rs:274-289` rejects an empty guard sequence)"*.

**WF-11 — `[PROVISIONAL]`; stays provisional** (§10). Three edits.

- **`:2105-2107`, the marker** **becomes**: *"`[PROVISIONAL — falsified if a peer must
  forward a payload too large to buffer, which would make this MUST unsatisfiable on that
  peer and force the clause to be re-scoped to formats rather than to peers. **This is a
  property of serde's data model rather than of base64** — read from serde's `Serializer`
  API rather than measured: `serialize_str` takes a `&str`, so **any** human-readable
  payload encoding, hex included, must materialise the whole payload. It therefore falsifies
  human-readable payload encoding as a category, and its instrument is the Workers peer
  under a memory limit at **phase 9**. The `no_std` half of the earlier falsifier is
  **closed**: measured at phase 5, a `#![no_std]` + `alloc` crate depending on `base64` with
  `default-features = false, features = ["alloc"]` checks clean for `wasm32-unknown-unknown`
  (ADR-0016 §10)]`"*
- **`:2110-2113`** (*"340 KB seat map becomes roughly 1.3 MB of JSON"*) **becomes**: *"so
  Turnstile's seat map (`docs/scenarios/README.md:1367`), taken as 340 KiB = 348,160 bytes,
  becomes **1,243,464 bytes** of JSON that no human can read — 1.243 MB in 10⁶ units, 1.186
  MiB in 2²⁰, and 3.5715× the raw payload. Base64 is 464,218 bytes (453.34 KiB, 1.3333×) and
  2.6786× smaller than the array of integers; lowercase hex is 696,322 bytes (680.00 KiB,
  2.0000×); postcard is 348,163 bytes (1.0000×). Measured; the programs are in
  `docs/experiments/wire-format/`."*
- **`:2115-2117`, the dependency paragraph.** Add: *"The cost is measured and it is the
  smallest a dependency has: `base64 0.22.1` is already in this workspace's graph, pulled by
  `sqlx-core` and `sqlx-postgres`, and has no transitive dependencies of its own — zero new
  crates for the workspace, exactly one leaf crate for a consumer taking the `serde` feature
  without sqlx. `base64` also appears in no public signature, so it does not touch the
  semver surface ADR-0003 protects. The feature line becomes `serde = ["dep:serde",
  "dep:base64", "bytes/serde", "serde/alloc"]`. Both named rules exist to reject an
  **inverted `is_human_readable` branch**, invisible to WF-7's rules: measured, `[de ad be
  ef]` must render as the JSON string `"3q2+7w=="` and as four raw bytes in postcard, and an
  inverted implementation renders `[222,173,190,239]` and the ASCII of `"3q2+7w=="`
  respectively while passing both round trips."*

**WF-12 — `[FROZEN]`; the MUST is untouched** (§13). Two edits.

- **`:2124`, the `Rule:` line** (*"compile test `read_options_is_not_serialisable`"*)
  **becomes**: *"`read_options_is_not_serialisable`, a **const-evaluation assertion in
  `crates/happenstance-core/tests/wire.rs`**, and **not a compile test** — that phrase is
  kept deliberately, because `spec_trace`'s `elsewhere` trigger reads it and a `const _` is
  not a `#[test]` any rule resolver could find. A `compile_fail` doctest passes whenever the
  snippet fails to compile for any reason: measured, of four spellings only the honest one
  detected that `ReadOptions` is serialisable at HEAD, while a type-name typo, a misspelt
  trait and a wrong crate path all reported green — and `RUNBOOK.md:3112-3117` already
  recorded that `compile_fail,E0080` is "the same check wearing a claim". The assertion is
  `const _: () = assert!(!Detect::<ReadOptions>::IS_SERIALIZE, …)`, where an inherent
  `impl<T: Serialize> Detect<T> { const IS_SERIALIZE: bool = true; }` shadows a defaulted
  trait constant of `false`, because inherent associated items win over trait ones; a
  misspelt type is then `error[E0425]`, a build failure rather than a green test. The
  `Deserialize` half needs `impl<T: serde::de::DeserializeOwned> Detect<T>`, not `impl<T:
  serde::Deserialize>` — the latter is `error[E0106]`, since `Deserialize<'de>` is
  lifetime-parameterised. What enforces the whole is that the test target must build, which
  `xtask/src/proof.rs` already does in order to list it."*

  That text backticks `spec_trace` and `compile_fail`, which `rules_of` will read as rule
  names. They are **inert only because this clause is `elsewhere`**: check 4 skips it,
  `rule_cell` renders its prose, and neither name is ever resolved. It is the one place in
  §15's convention that is satisfied by a second property rather than by the spelling, and
  the two properties are coupled — the day WF-12 stops being `elsewhere` it acquires two
  rules that cannot exist. The comment §15 puts on `spec_trace.rs:1329-1330` is what a
  reader arrives at from there.
- **`:2134-2137`, the "fifth hazard" paragraph** **becomes**: *"The impls at
  `query.rs:394-425` are on no wire path that exists and are maintenance the crate does not
  owe. **The claim that their whole-struct `#[serde(default)]` is a fifth hazard of WF-2's
  class is withdrawn**, measured: it is byte-for-byte inert in postcard on both encode and
  decode, because postcard treats running out of bytes as a hard parse error rather than an
  end-of-sequence signal, so the derive's default-fallback path never executes. Its only
  observable effect is in a self-describing format, where it lets the decoder accept a
  document the encoder cannot produce — a reason to delete it everywhere (ADR-0016 §4), but
  **not** the positional class, and `ReadOptionsWire` carried no `skip_serializing_if` at
  all."*

**VT-30 — `[PROVISIONAL]`; body only, no change to the MUST.** The clause describes the
pre-`1b2a565` world in the present tense, which §2 depends on being false.

- **`:1795-1797`** **becomes**: *"**Before commit `1b2a565`**, `AppendCondition` was `{
  fail_if_events_match: Query, after: Option<SequencePosition> }` with public fields on a
  `#[non_exhaustive]` struct. It is now `{ guards: Box<[Guard]> }` (`append.rs:84-99`) with
  `pub struct Guard { pub query: Query, pub after: Option<SequencePosition> }` (`:111-121`);
  the clause landed at phase 4 and stays `[PROVISIONAL]` until its two instruments are
  built."*
- **`:1804`** — `is_violated_by`'s anchor becomes **`append.rs:205-214`**, delegating to
  `Guard::is_violated_by` at `:219-233`.

**§3.4 (`:3415-3422`) — non-normative preamble**, and the worse of the two stale
descriptions, because a reader arriving at ES-25 through §3.4 is told the opposite of §2's
premise.

- **Current**: *"The shape shown here is the one on disk today:"* and the
  `fail_if_events_match` struct block.
- **Required**: *"The shape on disk since commit `1b2a565` is `pub struct AppendCondition {
  guards: Box<[Guard]> }` with `pub struct Guard { pub query: Query, pub after:
  Option<SequencePosition> }` (`append.rs:84-99`, `:111-121`). VT-30 landed at phase 4."*
- The paragraph that follows (`:3424-3431`) is rewritten in the past tense — *"VT-30 replaced
  it with…"* — and **keeps its own** *"written for one guard and generalises to N by
  conjunction"* sentence, which is why that sentence is deliberately absent from the Required
  text above. Writing it in both places produces it twice in adjacent paragraphs.

**§7.2's legend and reading conventions (`:8096-8107`) — non-generated prose**, false after
§15. Three edits, and the first is easy to miss because it sits in the legend rather than in
the conventions.

- **`:8096-8097`, the `†` legend** (*"was looked for in `suite.rs` and not found … it must be
  written"*) **becomes**: *"…was looked for in `crates/happenstance-testkit/src/suite.rs`,
  and — for a `wire::`-qualified name — in `crates/happenstance-core/tests/wire.rs` and
  `crates/happenstance-sync/tests/wire.rs`, and not found in any of them. It must be
  written."* Without this edit the legend describes a narrower search than the checker
  performs, and every wire rule the phase writes would be certified against a sentence that
  does not cover it.
- **`:8104-8107`** — delete (*"the checker's name parser does not accept a `::`-qualified
  rule … Read those two clauses, not their rows"*). It is the workaround §15 replaces with an
  instrument.
- **`:8098-8101`** — amend to name only compile and unit tests as `elsewhere`; §2.7's wire
  tests leave that category.

**All three are conditional on §15 landing green** — see the second open question.

**§1.3, the census (`:219-221`) — unchanged, and recomputed.** 200 clause IDs, 198 normative,
**139 `[FROZEN]`**, **49 `[PROVISIONAL]`**, **10 `[DEFERRED]`**, two `[NON-NORMATIVE]`,
under ADR-0015 §13's rule.

**§7.1 and §7.2 — regenerated, not edited.** The `WF` rows change because the clauses'
`Rule:` text changes and because §15 changes what `backticked_idents` can see, and that
region sits inside the generated markers (`:8126`, `:8372`). **Run `cargo xtask spec-trace
--write`.** Expect:

- WF-1 – WF-8, WF-10 and WF-11 to move from rendering their own prose to a rule list, **and
  for that list to carry no `†`** — the commit sequence writes the tests before it changes
  the parser, and `rule_cell` resolves against `known_rules ∪ wire_rules` (§15). A `†` on any
  of them is the signal that a rule the clauses name was not written, and the eighteen-rule
  table below is what it should be read against;
- **VT-19's row (`:8164`)** to gain **both** `wire::` rules its clause names and its row
  currently omits — `wire::decode_rejects_a_non_canonical_tag_set` and
  `wire::decode_accepts_an_over_capacity_value` (`Rule:` at `:1424-1426`);
- **WF-9's row (`:8193`)** to gain `wire::decode_accepts_an_over_capacity_value`;
- **WF-12's row (`:8196`)** to change, because its `Rule:` text is rewritten in full — it
  stays `elsewhere` and stays free of a `†`.

That is the visible proof that the fix landed. §7.1's `WF` row and the totals row are
unchanged.

**§7.5.** Two edits, one line apart. The **WF-12 row is at `:8616`** — not `:8617`, which is
CF-32's row and a verdict of **Acceptable** rather than a hole. Its verdict cell ends *"It is
a compile test, which is why the clause reached for one"*, which §13 refutes; that becomes
*"Its instrument is a const-evaluation assertion, not a compile test (ADR-0016 §13)."*
Separately, `:8623-8625`'s *"So: one genuine hole (WF-12, which is phase 5's)"* becomes zero
once the E2E case below is written. §7.6 (`:8629`) moves from *"All 57 cases"* to 58.

### Owed to the code, not to the specification

- **`event.rs:578`, `:580`, `query.rs:352`, `:354`, `append.rs:248`** — delete
  `skip_serializing_if` **and** `#[serde(default)]` at all five sites (§3, §4).
- **`query.rs:375-392`** — `Query`'s impls become an externally tagged two-variant encoding:
  `"All"` / `{"Items":[…]}` in JSON, `[00]` / `[01 …]` in postcard. The comment at `:377`
  documenting the `Option`-shaped intent is deleted, and the deletion is the reversal WF-3
  requires (§7).
- **`query.rs:394-425`** — delete `ReadOptionsWire` and both `ReadOptions` impls (§13).
- **`identity.rs:194-204`** — `StoreId`'s impls branch on `is_human_readable`: 32 lowercase
  hex digits without separators in the human-readable arm, `[u8; 16]` in the other.
  `EventIdWire` (`:208-211`) is unchanged; the clause moves to it (§9).
- **`event.rs` (`Event`'s impls)** — `data` and `metadata` branch on `is_human_readable`:
  base64 in the human-readable arm, `serialize_bytes` in the other (§10).
- **`crates/happenstance-core/Cargo.toml:36`** — the `serde` feature gains `dep:base64`;
  `base64` is added `optional = true, default-features = false`. The workspace manifest gains
  a `base64` entry beside `serde_json` (`Cargo.toml:41`), **pinned `"0.22"` with the reason
  in a comment**: `0.23.1` exists, sqlx pins `0.22`, and taking `0.23` would put two majors
  in the graph, which `deny.toml`'s `[bans] multiple-versions = "warn"` turns into a gate
  *warning* rather than a failure — the worse outcome, and the one that silently reverses
  §10's zero-new-crates measurement. `postcard` is added under **dev / tooling only**
  (`:69-71`).
- **`crates/happenstance-sync/src/wire.rs`** — new. `FORMAT_VERSION`, `Envelope<T> {
  format_version: u16, message: T }`, a hand-written `impl<'de, T: Deserialize<'de>>
  Deserialize<'de> for Envelope<T>` refusing an unknown version before calling
  `T::deserialize`, and its own error enum. **`peer.rs:391-421`'s `SyncError` is not
  touched.** The module doc at `lib.rs:78-86` is rewritten, and the manifest comment at
  `crates/happenstance-sync/Cargo.toml:15-18` records that phase 5 settled the
  versionless-format question.
- **`crates/happenstance-core/tests/wire.rs`** and
  **`crates/happenstance-sync/tests/wire.rs`** — new, `#[cfg(feature = "serde")]`, each with
  an **inner `mod wire`** so `cargo test --list` prints the qualified names the clauses cite
  (§15). They carry the proptest `RUNBOOK.md:3384-3393` specifies, over `serde_json` **and**
  `postcard`, plus the table a generator will not reliably reach: the four solo `Event`
  shapes, and the 70-byte-payload framed pair — kept as a **regression fixture**, not a
  discriminating one, because measured post-fix the decoded value equals the original and the
  `take_from_bytes` remainder equals the neighbour's own encoding. Its job is a remainder
  assertion that notices a reintroduced skip *in a batch*; until then it passes trivially,
  and that is what it is for. `happenstance-core`'s file also carries both `Detect<T>` pairs
  (§13).
- **Three negative controls, named so `proof.rs` can list them** — an unnamed test cannot be
  asserted present:
  `wire::negative_controls::skipped_event_wire_fails_the_postcard_round_trip` (a mirror
  carrying `skip_serializing_if`, backing WF-2's and WF-7's postcard rule);
  `wire::negative_controls::option_shaped_query_is_indistinguishable_from_none` (a mirror
  reproducing HEAD's `serialize_none`/`serialize_some`, backing
  `wire::query_all_is_unambiguous`); and
  `wire::negative_controls::length_checked_event_wire_rejects_a_65_kib_payload` (a mirror
  whose `Deserialize` grew a length check, backing
  `wire::decode_accepts_an_over_capacity_value`). Measured at HEAD, the first two wrong
  implementations are live and observable.
- **`docs/experiments/wire-format/src/lib.rs`** — add the `Deserialize` half of `Detect<T>`
  beside the `Serialize` one, so §13's HRTB finding stops resting on a throwaway crate.

**The eighteen `wire::` rules, all of them**, against the wrong implementation each rejects.
Eleven were never assigned to anybody, because until §15 lands the parser drops them. Phase 5
writes every one except where the last column says otherwise.

| Rule (`wire::`) | Named by | Wrong implementation it rejects | Phase 5 |
|---|---|---|---|
| `query_all_is_unambiguous` | WF-1, WF-3 | HEAD's `serialize_none` for `All`: `Some(All)` and `None` both `null`. Control: `option_shaped_query_is_indistinguishable_from_none` | yes |
| `option_query_round_trips` | WF-3 | the same from the `Option<Query>` side: `Some(All)` returns as `None` | yes |
| `empty_object_is_not_a_condition` | WF-1, WF-4 | a `Guard` decoding absent-or-`null` `query` to `All`; input `{"guards":[{}]}` | yes |
| `round_trips_in_postcard` | WF-2, WF-7 | any `skip_serializing_if` — this ADR's own starting point, held by the negative control | yes |
| `round_trips_in_json` | WF-7 | at maximum size only: a truncating payload encoder, or a `Deserialize` with a length cap (§6) | yes |
| `sequenced_event_round_trips` | WF-5 | **already satisfied** by `SequencedEventWire` (§8); a regression pin against a wire form that drops `id` as recomputable | yes |
| `condition_after_is_visible_to_an_ingest_policy` | WF-4 | a `GuardWire` that skips `after` when it is `None` | yes |
| `condition_round_trips` | WF-4 | **nothing the two round-trip rules do not already reject** — dropped from the clause by amendment | no |
| `store_id_encodes_as_hex_in_json` | WF-6 | an inverted `is_human_readable` branch (§9) | yes |
| `store_id_encodes_as_bytes_in_postcard` | WF-6 | the same inversion, other arm | yes |
| `payload_is_base64_in_json` | WF-11 | an inverted branch on the payload (§10) | yes |
| `payload_is_raw_in_postcard` | WF-11 | the same inversion, other arm | yes |
| `decode_rejects_a_non_canonical_tag_set` | WF-10, **VT-19** | **already satisfied** (`tag.rs:497-504`); a regression pin against a `Deserialize` derived straight onto private fields | yes |
| `decode_rejects_an_unconstrained_query_item` | WF-10 | **already satisfied** (`query.rs:368-373`); the same pin, `QueryItem`'s invariant | yes |
| `decode_rejects_a_zero_item_query` | WF-10 | **already satisfied** (`query.rs:385-392`); the same pin, `Query::from_items`' invariant | yes |
| `decode_accepts_an_over_capacity_value` | WF-9, WF-10, **VT-19** | a `Deserialize` that grew a length check at `MIN_SUPPORTED_EVENT_DATA_LEN` (§12) | yes |
| `rejects_an_unknown_format_version` | WF-8 | an envelope that reads the version and does not act on it | yes |
| `version_is_readable_before_the_message` | WF-8 | a derived `Deserialize` plus a post-hoc check — witness count 1 against 0 (§11) | yes |

The four marked *already satisfied* are regression pins by admission rather than by apology:
§6's obligation is that a rule name a wrong implementation it *could* reject, and a pin names
one already removed. What keeps them from being decorative is that three of them guard a
`Deserialize` a future contributor could legitimately re-derive.

Two more sit beside them and are not `wire::`: `append_reports_exceeded_store_limits` (WF-9,
VT-19) already exists at `suite.rs:4282`, and `read_options_is_not_serialisable` (WF-12) is
re-specified in §13 as a const-evaluation assertion.

### Owed to the RUNBOOK

- **`:3356`** — *"Keep `#[serde(default)]` so existing JSON still parses"* is **reversed, on
  this ADR's own reasoning and not under rule 5** (§4). The same edit records that the
  deletion closes the read side at three of the five sites only.
- **`:3356-3357`** — *"the cost is a bare `Event` going from 35 to 61 bytes of JSON"* becomes
  **+26 bytes**, with 35 → 61, 39 → 65 and 28 → 54 as worked instances.
- **`:3348-3349`** — the five `skip_serializing_if` sites are cited as `event.rs:342`,
  `:344`, `query.rs:281`, `:283`, `append.rs:121`. They are at `event.rs:578`, `:580`,
  `query.rs:352`, `:354`, `append.rs:248`.
- **`:3362-3366`** — the D6 item says `AppendCondition::Wire.fail_if_events_match` carries no
  `#[serde(default)]`, so `{}` becomes a hard error. The field is `GuardWire::query`, `"{}"`
  **already** errors at HEAD, and the live input is `{"guards":[{}]}`.
- **`:3381`** — *"A 1 KiB payload is currently ~4 KiB of unreadable JSON"*. Measured, the
  ratio is **3.5715×**, so ~3.6 KiB.
- **`:3402-3404`** — the exit criterion is **two criteria wearing one sentence** (§12). Split
  it: *"The envelope carries a format version"* (WF-8) and *"a peer built at an older bound
  rejects an over-bound message…"* (WF-9, whose variant and suite rule both already exist;
  what is owed is `wire::decode_accepts_an_over_capacity_value`, core-only, no peer).
- **`:3405-3407`** — the WF-12 exit criterion ends *"It is a compile test, which is why the
  clause reached for one"*, the same sentence `SPECIFICATION.md:8616` carries and the one §13
  refutes. **This is the line a phase-5 implementer ticks**, so it must not survive: it
  becomes *"Its instrument is a const-evaluation assertion in
  `crates/happenstance-core/tests/wire.rs`, not a compile test — three of four `compile_fail`
  spellings were measured green against a false assertion (ADR-0016 §13)."*
- **`:3346-3382`, the work list** — it names only WF-1, WF-3, WF-4, WF-8 and WF-11. WF-2,
  WF-5, WF-6, WF-7, WF-9, WF-10 and WF-12 appear nowhere in it, and of those only WF-12
  appears in the exit criteria (`:3405`). Add WF-2's clause ID to the D1 item, WF-5 as
  *already implemented, costs a round-trip test* (§8), WF-6's hex/bytes split with its
  inverted-branch rules (§9), WF-7's two-format matrix and its maximum-size half (§6), WF-9's
  decode half (§12), WF-10's four rules, and WF-12's deletion (§13).
- **`:591`** — the `[PROVISIONAL]` ledger row for WF-11 names in its "falsified by" cell the
  **problem the clause solves** (*"a JSON payload nobody can read in a log"*), not a falsifier
  — the same defect ADR-0012 §9 found for VT-30, one column over; that VT-30 row was at
  `:561` when ADR-0012 was written and is at `:590` now, one line above this one. WF-11's
  cell becomes: *falsified by a peer that cannot buffer a payload through any human-readable
  encoder*, and the owning phase becomes **9**. The `no_std` half is **not** in the new cell,
  because it closed at phase 5 by measurement (§10) — which is what makes a single owning
  phase the right answer for a single cell.
- **`:576`** — the heading *"The 46 `[PROVISIONAL]` clauses"* disagrees with the
  specification's 49 (`:219-221`, `:8138`). It was reconciled at phase 3 against 193 clauses
  (`:2693-2703`) and phase 4 moved it: CF-39, CF-40, ES-41 **and ES-42** added, ES-10 lifted
  — 46 − 1 + 4 = **49**. Not this ADR's arithmetic to land; recorded because this ADR walked
  past it.
- **`:480`** — the ledger's wire row says *"settled — WF-1 – WF-12 (D1 critical, D6, D12)"*.
  WF-1 is settled in its scope half only and stays `[DEFERRED]`; D12 was closed at phase 0 by
  `927d291` and phase 5 adds a lint rather than a fix.

### Owed to xtask

- **`xtask/src/spec_trace.rs`** — the **three** changes of §15, which must land together:
  `backticked_idents` (`:1379-1392`) accepts `:` and keeps the qualified string; check 4
  (`:527-539`) gains a second resolution source, scoped to `wire::` names, parsing `#[test]
  fn` names out of `crates/happenstance-core/tests/wire.rs` and
  `crates/happenstance-sync/tests/wire.rs`, **and `rule_cell` (`:897-919`) is handed the same
  union**, or seventeen written tests render `†`; check 6 (`:553-563`) keeps iterating over
  the suite set only, because sweeping the wire tests in would demand a clause for every
  helper test. Two sets, then: `known_rules` for check 6, `resolvable` for check 4 and
  `rule_cell`. **Change one alone turns `cargo xtask spec-trace` red for WF-1 – WF-11 and VT-19, and
  writing the seventeen tests does not clear it** — that is why the change is three and not
  one. **Do not add `wire::` to the `elsewhere` trigger list** (`:1329-1330`); do add a
  comment there recording that WF-12 depends on the words "compile test" surviving in its
  `Rule:` line as a negation, because a `const _` can never be resolved. And `rules_of`
  (`:1305-1341`) gains the comment `retires_of` (`:1354-1371`) already carries one field
  over: **a `Rule:` field may backtick rule names and nothing else**, because the whole field
  is handed to `backticked_idents` and every backticked snake_case token in it becomes a name
  the checker will demand. That comment is load-bearing rather than tidy — it is the rule this
  ADR's own amendments broke three times before landing (§15). **If the three changes cannot
  be made green, §15 is withdrawn** — see the second open question.
- **`xtask/src/proof.rs`** — the instrument the negative-control open question needs, and it
  exists already, but it is a **three-const single-artefact checker** (`PACKAGE`, `TARGET`,
  `META_TESTS` at `:47-80`; `run()` at `:106-143`), not a table. So this is a small refactor
  rather than an added row: generalise the three consts into a list of `{ package, target,
  tests }` and loop, keeping `registry_len()` as testkit-only reporting. **Two entries are
  added, not one** — `happenstance-core`'s `wire` target with the three negative controls,
  and `happenstance-sync`'s `wire` target with WF-8's
  `wire::rejects_an_unknown_format_version` and
  `wire::version_is_readable_before_the_message`, which nothing else asserts by name and
  which are otherwise the same dead-code hole the open question exists to close. The refactor
  changes the module's stated job from "the suite's own proof artefact" to "each phase's", so
  its module doc (`:1-37`) moves with it. Its subset-not-equality argument (`:30-35`)
  transfers unchanged and is cited rather than re-argued: a ninth wire test must not need a
  gate edit to land.
- **`xtask/src/lints.rs`** — a new `serde_alloc()` lint, modelled line-for-line on
  `testkit_version()` (`:285`): read `crates/happenstance-core/Cargo.toml`, find the
  `serde = [...]` line in `[features]`, and fail if it does not name `serde/alloc`. The
  failure message must say what CF-32's says in its own idiom: this compiles today only
  because `bytes` 1.12.1 declares its optional `serde` with `features = ["alloc"]`, so the
  check asserts a manifest fact no test and no compile can see (D12, §14).
- **`xtask/src/main.rs`** — wire the new lint as a gate step beside *"the testkit carries its
  own version"* (`:413-430`), with a comment naming D12.
- **No new clause.** The lint is a gate step, not WF-13 (§14).

### Owed to `E2E-CASES.md`

- **A new case for WF-12**, which `SPECIFICATION.md:8616` and `:8623-8625` record as the
  section's one remaining genuine hole: assert that `ReadOptions` implements neither
  `Serialize` nor `Deserialize`. Its instrument is a const-evaluation assertion rather than
  the compile test the clause reached for (§13), and the case text should say so rather than
  repeat the mechanism the amendment corrects.

  **Where it goes is arithmetic, and ADR-0015 §15 already worked the rule out.** The index
  table is at `:39-46` and its groups are contiguous numeric ranges; **E is E2E-52 … E2E-57**
  (`:45`), and E2E-57 is the last case (`:1502`). So the new case is **E2E-58**, it extends
  **group E**, and the index row's range moves to E2E-52 … E2E-58. Filing it topically in
  group C — where the other wire cases live, E2E-33 … E2E-45 (`:43`) — would require
  renumbering thirteen cases and every citation to them. The house pattern is to append
  numerically, which is why E2E-56 and E2E-57 already sit in "Edge and `!Send`".

- **`SPECIFICATION.md:8629`** — *"None. All 57 cases are claimed by at least one clause"*
  becomes 58. That sentence lives in `SPECIFICATION.md`, which is why it is listed with the
  specification's amendments as well as here.

- Note, as ADR-0015 §15 did, that `spec_trace::check_citations` reads only
  `SPECIFICATION.md`, so **nothing checks this file's cross-references**. The new case's
  `Spans` and clause citations are proofread by a human or not at all.
