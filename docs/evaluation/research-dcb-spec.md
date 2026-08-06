# DCB Specification Conformance Matrix — `happenstance`

Evaluated 2026-08-05 against <https://dcb.events/specification/> (retrieved verbatim,
full HTML captured to `scratchpad/spec/specification.html`), plus:

- <https://dcb.events/topics/tags/>
- <https://dcb.events/topics/projections/>
- <https://dcb.events/topics/aggregates/>
- <https://dcb.events/examples/course-subscriptions/>
- <https://dcb.events/faq/>
- The **official reference library** linked from the worked example:
  <https://github.com/dcb-events/dcb-events.github.io/tree/main/libraries/dcb>
  (`EventStore.ts`, `DecisionModel.ts`, `InMemoryDcbEventStore.ts` — fetched to `scratchpad/ref/`)
- The Python `eventsourcing` DCB implementation:
  <https://eventsourcing.readthedocs.io/en/v9.5.0/topics/dcb.html>

**There is no changelog or revision history on dcb.events.** The site is a MkDocs
Material build (`© 2026`); the specification page carries no version marker and no
"changed in" notes. I found no evidence of a revision that folded the append condition
into the query — the current text keeps them separate, and the reference library's
`AppendCondition` type is `{ failIfEventsMatch: Query; after?: number }`, matching
happenstance field-for-field. Question 4's premise about a recent restructuring does not
hold up.

## Headline

happenstance is **conformant on every MUST in the specification**. I could not find a
clause it violates. The specification is small (22 normative statements), and
happenstance's type-level encoding is in several places *stricter* than the reference
implementation in ways that are correct.

The real exposure is not conformance — it is (a) one affordance the spec explicitly
blesses that the port cannot express, (b) an unstated invariant the whole safety
argument silently depends on, and (c) what the 27-rule suite does *not* check, which is
exactly the surface the SQLite adapter (phase 1, next) will get wrong.

---

## Part 1 — Clause-by-clause matrix

Every normative statement in the specification, verbatim, against the code and the suite.

### Reading Events

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| R1 | "... **MUST** provide a way to filter Events based on their Event Type and/or Tag (see Query)" | `EventStore::read(&self, query: &Query, options: ReadOptions)` — `store.rs:117-121` | `query_all_matches_every_event`, `query_item_types_are_or`, `query_item_tags_are_and`, `query_item_combines_types_and_tags_with_and`, `query_items_are_or` ✅ |
| R2 | "... **SHOULD** provide a way to read Events from a given starting Sequence Position" | `ReadOptions.from: Option<SequencePosition>` — `query.rs:229` | `read_from_is_inclusive` ✅ |
| R3 | "... **MAY** provide further filter options, e.g. for ordering or to limit the number of Events to load at once" | `ReadOptions.backwards`, `ReadOptions.limit` — `query.rs:231-233` | `read_backwards_reverses_order`, `read_limit_truncates`, `read_backwards_from_with_limit`, `read_defaults_to_ascending_order` ✅ |

**Answering Q5 directly:** `backwards` and `limit` are **spec features, not happenstance
inventions** — R3 names both ("for ordering or to limit the number of Events"). The spec
does **not** define `from`'s inclusivity or how `limit` composes with `backwards`; the
reference library does, and happenstance matches it exactly:

> `@param {number|null} from - an optional Sequence Position to start streaming Events
> from (depending on the `backwards` flag, this is either a _minimum_ or _maximum_
> sequence number of the resulting stream)`
> — `EventStore.ts`

happenstance: forwards keeps `event.position >= from`, backwards keeps
`event.position <= from` (`memory.rs:163-174`) — identical, and inclusive in both
directions, as documented at `store.rs:116` and pinned by `read_from_is_inclusive`.

> `@param {number|null} limit - ... limits the Event stream to a maximum number of
> Events. **This can be useful for retrieving only the last Event, for example.**`
> — `EventStore.ts`

happenstance applies `limit` *after* ordering and filtering (`memory.rs:176-178`), so
`backwards + limit(N)` = "the newest N matches" — same as the reference
(`filteredEvents.reverse()` then `.slice(0, limit)`). Correct. See finding
**F10** for the coverage gap on the specific no-`from` variant.

### Writing Events

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| W1 | "... **MUST** provide a way to atomically persist one or more Event(s)" | `EventStore::append` — `store.rs:141-145`; memory runs the whole operation under one write lock (`memory.rs:195`) | `append_is_atomic` — **partial**, see F-note below |
| W2 | "... **MUST** fail if the Event Store contains at least one Event matching the Append Condition, if specified" | `AppendCondition::is_violated_by` — `append.rs:95-107` | `condition_without_after_rejects_any_match`, `condition_after_rejects_events_beyond_the_boundary`, `condition_rejection_is_reported_as_condition_violated`, `racing_conditional_appends_elect_one_winner` ✅ |

**W1 partial:** `append_is_atomic` (`suite.rs:385-403`) only checks the *rejected* path —
that a refused batch leaves no partial writes. Nothing checks that a *successful*
multi-event batch is indivisible to a concurrent reader (no reader may observe events 1
and 3 of a 3-event batch without 2). The suite is sequential by construction and says so
(`suite.rs:588-595`), so this cannot be tested there. Reasonable, but it means "atomic"
is asserted for half of W1.

### Query

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| Q1 | "It **MUST** contain a set of Query Items with at least one item or represent a query that matches all Events" | `enum Query { All, Items(Box<[QueryItem]>) }` — `query.rs:144-150`; `Query::from_items` rejects empty with `InvalidQuery::NoItems` (`query.rs:164-170`) | Type-level; unit test `empty_query_is_unrepresentable` (`query.rs:369`) ✅ |
| Q2 | "All Query Items are effectively combined with an **OR**" | `items.iter().any(...)` — `query.rs:202` | `query_items_are_or`; proptest `adding_a_query_item_is_monotonic` ✅ |

happenstance's enum is a **better encoding than the reference**, which represents
match-all as `{items: []}` (`queryAll()` in `EventStore.ts`) — a value the spec says is
illegal for a filtered query, distinguished only by convention. happenstance makes the
illegal state unrepresentable.

### Query Item

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| QI1 | "the Type **MUST** match one of the provided Types of the Query Item" | `self.types.is_empty() \|\| self.types.binary_search(event_type).is_ok()` — `query.rs:114` | `query_item_types_are_or` ✅ |
| QI2 | "the Tags **MUST** contain all of the Tags specified by the Query Item" | `tags.contains_all(&self.tags)` — `query.rs:115`, merge-scan at `tag.rs:231-245` | `query_item_tags_are_and`, `query_item_tags_match_supersets`, `query_item_rejects_partial_tag_overlap`; proptests `tag_item_matches_iff_superset`, `contains_all_agrees_with_elementwise_containment` ✅ |

Empty-means-any is not stated in prose but is unambiguous from the spec's own example
(an item with only `types`, an item with only `tags`) and from the reference's
`matchesEvent`: `(!queryItem.types || queryItem.types.includes(event.type)) && (!queryItem.tags || ...)`.
happenstance matches (`query.rs:114-115`).

happenstance additionally rejects an item constraining *neither*
(`InvalidQuery::UnconstrainedItem`, `query.rs:69-71`). The spec is silent, but the
reference's TypeScript type forbids it structurally:
`{ tags: [string, ...]; types?: [...] } | { tags?: [...]; types: [string, ...] }`.
happenstance agrees with the reference. ✅

### Sequenced Event

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| SE1 | "It **MUST** contain the Sequence Position" | `SequencedEvent.position` — `event.rs:279` | Type-level ✅ |
| SE2 | "It **MUST** contain the Event" | `SequencedEvent.event` — `event.rs:281` | Type-level ✅ |
| SE3 | "It **MAY** contain further fields, like metadata defined by the Event Store" | **Not used.** `SequencedEvent` has exactly two fields (`event.rs:277-282`) | n/a — see **F7** |

### Sequence Position

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| SP1 | "**MUST** be unique in the Event Store" | `NonZeroU64` newtype — `event.rs:119` | `positions_are_unique` — **in-process only**, see **F3** |
| SP2 | "**MUST** be monotonic increasing" | documented `event.rs:94-96` | `positions_are_strictly_monotonic` — **value only, not visibility**, see **F2** |
| SP3 | "**MAY** contain gaps" | documented as "an opaque ordering key, **not** a count of events. Never compute `end - start` and call it a length." (`event.rs:95-97`) | Suite compares against store-assigned positions throughout, never literals — e.g. `positions_of(&all[..1])` at `suite.rs:192-196` ✅ **exemplary** |

### Events (the append batch)

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| ES1 | "**MUST** not be empty" | `AppendError::NoEvents` — `error.rs:155-161`; memory at `memory.rs:211-216` | `append_rejects_empty_batch` ✅ — but see **F6** for the unpinned interaction |
| ES2 | "**MUST** be iterable, each iteration returning an Event" | `&[Event]` — `store.rs:143` | Type-level ✅ (but see **F8**) |

ES1 is a clause many implementations skip — the reference enforces it only through
TypeScript's `[Event, ...Event[]]` tuple type and does no runtime check
(`InMemoryDcbEventStore.append` has no guard). happenstance checks it at runtime **and**
gives it a dedicated error variant and a conformance rule. Credit where due.

### Event

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| E1 | "It **MUST** contain an Event Type" | `Event.event_type: EventType`, non-empty enforced — `event.rs:184`, `event.rs:40-42` | `append_preserves_event_payload` ✅ |
| E2 | "It **MUST** contain Event Data" | `Event.data: Bytes` — `event.rs:185` | `append_preserves_event_payload` ✅ |
| E3 | "It **MUST** contain Tags — in rare cases, the list could be empty, but all associated Tags must always be exposed" | `Event.tags: Tags` (always present, possibly empty) — `event.rs:186`, accessor `event.rs:234` | `append_preserves_event_payload` compares whole-event equality incl. tags ✅ |
| E4 | "It **MAY** contain further fields, like metadata defined by the client" | `Event.metadata: Option<Bytes>` — `event.rs:187` | `append_preserves_event_payload` ✅ |

**Answering Q2 directly:** the spec requires **no event identity, no timestamp, and no
recorded-at**. E1–E3 are the complete list of required fields. Both reference
implementations agree:

- reference TS: `type Event = { data: Record<string, unknown>; type: string; tags: string[] }`
  and `type SequencedEvent = Event & { position: number }` — no timestamp, no id.
- Python `eventsourcing`: `DCBEvent(type_, data_, tags_)` and
  `DCBSequencedEvent(event_, position_)` — "There is **no timestamp, recorded-at, or
  event id** field."

So the absence is **spec-permitted, and matches every reference implementation**. It is
nonetheless a practical defect for a library aiming to be a production workhorse — see
**F7**, whose force comes from timing (the schema lands two phases before the decision),
not from conformance.

### Tags / Tag

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| T1 | "It **SHOULD** not contain multiple Tags with the same value" | `Tags` sorts + dedups at construction (`tag.rs:258-266`); duplicates unrepresentable | proptests `tags_canonicalisation_is_idempotent`, `tags_equality_ignores_insertion_order` ✅ **exceeds SHOULD** |
| TG1 | "It **MAY** represent a key/value pair such as `product:123` **but that is irrelevant to the Event Store**" | `Tag(Box<str>)` opaque; `Tag::key()`/`Tag::value()` expose the convention without enforcing it (`tag.rs:39`, `tag.rs:90-98`) | Unit test `key_value_splits_on_first_colon_only` ✅ |

**Answering Q3 directly: happenstance is right, and the type must *not* enforce
`key:value`.** TG1 says the shape is "irrelevant to the Event Store" in so many words.
Enforcing it would make happenstance **non-conformant with the spec's own example
query**, which uses bare opaque tags:

```json
{ "items": [ { "types": ["EventType1", "EventType2"] },
             { "tags": ["tag1", "tag2"] },
             { "types": ["EventType2", "EventType3"], "tags": ["tag1", "tag3"] } ] }
```

`tag1` has no colon. A `Tag` type requiring `key:value` could not represent it.

The query semantics reinforce this: QI2 is *set containment over opaque strings*
(`tags.contains_all`), never a per-key lookup. Nothing in matching ever splits a tag. The
`topics/tags` article confirms the convention is advisory — "Prefixes (e.g.
`customer:c123`, `order-1234`) help disambiguate values and standardize tag structure" —
note `order-1234`, a prefix with a *hyphen*, in the spec's own guidance.

happenstance's split is exactly right: opaque storage + matching, `Tags::from_pairs` as
the ergonomic default (convention over configuration), `Tag::new` as the escape hatch.
`Tag::key_value` correctly rejects a colon in the *key* (`InvalidTag::ColonInKey`,
`tag.rs:73-75`) while permitting one in the value, keeping `key()`/`value()`
unambiguous. No change needed.

### Append Condition

| # | Spec text (verbatim) | happenstance | Suite rule |
|---|---|---|---|
| AC1 | "It **MUST** contain a `failIfEventsMatch` Query" | `AppendCondition.fail_if_events_match: Query` — `append.rs:54` | ✅ |
| AC2 | "It **MAY** contain an `after` Sequence Position" | `AppendCondition.after: Option<SequencePosition>` — `append.rs:60` | ✅ |
| AC3 | "The Event Store **MUST** ignore the Events before the specified position while checking the condition for appending events" | `match self.after { Some(after) if position <= after => false, _ => ... }` — `append.rs:103-106` (`after` **exclusive**) | `condition_after_ignores_events_at_the_boundary`, `condition_after_rejects_events_beyond_the_boundary`, `condition_after_ignores_non_matching_events` ✅ |
| AC4 | "Note: This number **can be higher than the position of the last event matching the Query**." | **No way to produce such a position.** See **F1** | **Never exercised.** See **F4** |
| AC5 | "if omitted, no Events will be ignored, effectively failing if any Event matches the specified Query" | `after: None` → falls through to `matches()` — `append.rs:105` | `condition_without_after_rejects_any_match`, `condition_without_after_allows_non_match` ✅ |

**Answering Q4 directly:** happenstance's `AppendCondition` matches the spec
**field-for-field and name-for-name**. The spec's pseudo-code is:

```
AppendCondition {
  failIfEventsMatch: Query
  after?: SequencePosition
}
```

happenstance is `{ fail_if_events_match: Query, after: Option<SequencePosition> }`. No
divergence. And the condition has **not** been folded into the query in any revision I
can find — the reference library, last updated in the same repo that publishes the site,
still carries the two-field type.

happenstance's encoding is better than the reference on one point: the reference uses
`after: 0` as the sentinel for "no boundary" (`DecisionModel.ts`:
`let lastConsumedEventPosition = 0`), then tests it with the falsy check
`if (!condition.after)` — which conflates "position 0" with "absent". `NonZeroU64` +
`Option` makes position 0 unrepresentable and the niche optimisation makes
`Option<SequencePosition>` free (`event.rs:98-102`, asserted by
`option_position_is_niche_optimised`). This is a genuinely better model of the same
thing.

---

## Part 2 — Q1 answered in full: does `read` owe a head position?

**Short answer: not per the specification, and happenstance matches the official
reference exactly. But the affordance AC4 exists to enable is unreachable, and one major
implementation exposes it.**

The spec's `read` returns only `SequencedEvents` — "some form of iterable or reactive
stream of Sequenced Events". There is **no clause requiring a head**, and no mention of
one anywhere on dcb.events.

The official reference computes the append condition like this:

```ts
// DecisionModel.ts
let lastConsumedEventPosition = 0
for (const event of eventStore.read(compositeProjection.query)) {
  state = compositeProjection.apply(state, event)
  lastConsumedEventPosition = event.position
}
const appendCondition: AppendCondition = {
  failIfEventsMatch: compositeProjection.query,
  after: lastConsumedEventPosition,
}
```

That is **the last matched position** — identical to happenstance's
`read_decision_model`:

```rust
// store.rs:205-207
let events = collect(store.read(query, ReadOptions::new())).await?;
let last = events.last().map(|event| event.position);
Ok((events, last))
```

with `after: 0` ≡ `after: None`. **So happenstance's choice is the reference behaviour,
and it is sound.** The equivalence argument holds: if the read query and the condition
query are the same, no matching event can exist between the last match and the head (it
would have been returned), so `after = last_matched` and `after = head` reject exactly
the same set of concurrent writes. Your hypothesis that they might differ when the query
matches nothing but the log is non-empty does **not** produce a correctness gap — if
nothing matched at read time, "any match at all" and "any match after the read" are the
same predicate. `after: None` is a correct guarantee.

Where it does bite is **cost and expressiveness**, which is what AC4's Note is for:

1. **Cost.** With `after: None`, AC5 obliges the store to consider the *entire log*.
   On the SQLite adapter that is an unbounded index probe on the hottest path in the
   system — every command, forever — where `after: Some(head)` would be a bounded range
   scan from `head+1`. This is the case AC4 exists to permit, and it hits precisely the
   commands whose decision model matches nothing: "define a course that must not exist",
   "claim a username", "issue an invoice number" — the entire uniqueness family, which is
   four of the seven worked examples on dcb.events.

2. **Expressiveness.** The spec explicitly endorses splitting the two queries:

   > "the query used to build a decision model (aka *SourcingCriteria*) does not have to
   > match the query used to enforce append conditions (aka *AppendCriteria*)"
   > — <https://dcb.events/topics/tags/>

   With a different append query, `after = last_matched_by_the_sourcing_query` has no
   sound justification, and happenstance offers nothing else.

The Python `eventsourcing` library solves this by returning a head:

> The `head` property represents "a 'last known position' that corresponds to the last
> recorded event in the database at the time of reading, rather than the sequence number
> of the last event it receives."

That is precedent, not obligation. Severity: **design-risk, not defect** — see **F1**.

---

## Part 3 — Q6: MUSTs the conformance suite does not check

The 27 rules (`testkit/src/lib.rs:93-129`) are well-targeted. What they miss:

| Unchecked | Clause | Why it matters | Finding |
|---|---|---|---|
| Position uniqueness/monotonicity **across a store reopen** | SP1 ("unique **in the Event Store**"), SP2 | Every rule starts from a "fresh, empty store". A file-backed adapter computing the next position as `COUNT(*)` or `MAX(rowid)+1` passes all 27 rules and reuses positions after a delete. The sqlite stub's own comment (`event_store.rs:31-34`) justifies `AUTOINCREMENT` by "the specification requires uniqueness across the store's whole lifetime" — there is no test for the property that comment defends. | **F3** |
| **Visibility** ordering of positions | (unstated in spec) | The soundness of `after` depends on it. Neither the spec nor happenstance states it. | **F2** |
| `after` **greater than the last matching position / beyond the head** | AC4 — the one clause the spec flags with an explicit "Note" | Every `condition_after` in the suite passes a position returned by a real `append` (`suite.rs:483, 501, 519, 604`). An adapter that validates `after` against stored rows breaks on a head-derived value. | **F4** |
| `Query::all()` as a **condition** query | AC1 + Q1 | Never once. The condition is the part pushed into SQL; match-all is where a generated `WHERE` clause degenerates. | **F5** |
| **Multi-item** condition query | AC1 + Q2 | Every fixture is `Query::from_item` (`fixtures.rs:65, 76, 87`) — single-item only. Multi-item OR inside a condition is where operator precedence against the `position > after` conjunct goes wrong. | **F5** |
| Empty batch **with** a condition | ES1 × W2 | Suite tests only `append(&[], None)` (`suite.rs:408`). Memory checks the condition *first* (`memory.rs:197-216`), so two conformant adapters return different errors. | **F6** |
| Atomicity of a **successful** multi-event batch | W1 | Only the rejected path is checked. Acknowledged as untestable in a sequential suite. | noted above |
| `backwards + limit` with no `from` | R3 | The reference's own documented use case ("retrieving only the last Event") and the shape its own condition probe uses. | **F10** |
| `from` beyond the head → empty, not error | R2 | Untested boundary. | minor |

---

## Part 4 — Q7: spec concepts with no representation in happenstance

Only one, and it is a MAY:

- **SE3 — "[a Sequenced Event] MAY contain further fields, like metadata defined by the
  Event Store."** `SequencedEvent` carries `position` and `event` and nothing else
  (`event.rs:277-282`). This is the slot for store-assigned data: a recorded-at
  timestamp, a store-assigned identity, a transaction id. The RUNBOOK already commits to
  putting `EventId` there ("Lamport pair `(origin, origin_position)` as `EventId` on
  `SequencedEvent`; store-assigned", `docs/RUNBOOK.md:67`, phase 3) but tracks no
  timestamp — `grep -rin "timestamp\|recorded_at\|occurred_at\|created_at"` over
  `crates/` and `docs/` returns **zero hits**. See **F7**.

Everything else in the specification has a representation. The spec is small and
happenstance covers it.

---

## Part 5 — What is right

Stated so it does not get re-litigated:

- **Tag model** (`tag.rs`) — opaque string with an advisory `key:value` convention is
  exactly TG1. Enforcing the shape would be a conformance regression.
- **`AppendCondition`** — field-for-field with the spec, with a strictly better
  encoding of "absent" than the reference's `0` sentinel.
- **`after` exclusive at the boundary** — correct, and the best-tested area of the suite
  (`condition_after_ignores_events_at_the_boundary` pins the exact off-by-one that
  would cause a livelock).
- **`Query` as an enum** — models Q1 exactly; the reference cannot.
- **`from` inclusive in both directions, `limit` applied after ordering** — matches the
  reference precisely, and the spec leaves it undefined, so documenting it
  (`store.rs:112-116`) is a real contribution.
- **`backwards` / `limit` are R3 MAY features**, not inventions, and are correctly framed
  that way.
- **Gap discipline** — the suite never asserts literal positions and the house rule says
  so. `SequencePosition`'s doc ("Never compute `end - start` and call it a length",
  `event.rs:96-97`) is the right warning in the right place.
- **ES1 enforced at runtime with a dedicated variant and a rule** — the reference does
  not check this at all.
- **`Tags` canonicalisation** exceeds T1's SHOULD by making duplicates unrepresentable,
  and the proptests pin the merge-scan against a naive oracle.
- **`AppendError::ConditionViolated` separated from `Store(E)`** — not a spec
  requirement, but it is the difference between a usable and an unusable retry loop.

---

## Part 6 — Findings

Full detail in the structured output. Ordered by severity.

| # | Severity | Kind | Title |
|---|---|---|---|
| F1 | high | design-risk | No head position: AC4's affordance is unreachable, forcing whole-log condition checks |
| F2 | high | gap | The position-*visibility* invariant that makes `after` sound is stated nowhere |
| F3 | medium | gap | Position uniqueness is never tested across a store reopen |
| F4 | medium | gap | AC4 — `after` beyond the last match — is never exercised |
| F5 | medium | gap | `Query::all()` and multi-item queries are never used as *condition* queries |
| F6 | medium | defect | Empty-batch vs condition-check ordering is unpinned; the oracle picks the surprising order |
| F7 | medium | design-risk | No store-assigned timestamp, and the schema lands two phases before the decision |
| F8 | medium | ergonomics | `append(&[Event])` forces a clone per event; `into_parts` is unreachable from the port |
| F9 | low | ergonomics | `read_decision_model` cannot stream |
| F10 | low | gap | `backwards + limit` with no `from` — the reference's own use case — is untested |
