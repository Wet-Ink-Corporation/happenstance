# happenstance

An opinionated, storage-agnostic event sourcing library for Rust, built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

## Which crate do I want?

- **Writing an application?** This one — and a store to keep the events in.
  Four ship at `0.2.0`, and the choice is about where the log lives rather than
  about features:
  [`happenstance-sqlite`](https://crates.io/crates/happenstance-sqlite) for one
  file on disk;
  [`happenstance-postgres`](https://crates.io/crates/happenstance-postgres) for a
  server whose writers do not queue behind each other;
  [`happenstance-neon`](https://crates.io/crates/happenstance-neon) for the same
  Postgres with no connection to hold, over one-shot HTTP; and
  [`happenstance-cloudflare`](https://crates.io/crates/happenstance-cloudflare)
  for a Durable Object on `wasm32`.
- **Writing a storage adapter?** Depend on
  [`happenstance-core`](https://crates.io/crates/happenstance-core) instead. It is
  the smaller semver surface, and it is what
  [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit) measures
  you against.

## Stability

- **`0.2.0` is the first stable release**, and what it promises is exact: the
  `EventStore` clauses marked `[FROZEN]` in
  [the specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)
  are semver-binding from here.
- **`ProjectionStore` is not part of that promise.** It ships behind the
  off-by-default `unstable-projection` feature and is exempt from semver. The
  reason is not that nothing implements it — four adapters clear its suite — but
  that its freeze condition asks for two adapters at opposite ends of an axis
  whose far end the port's own signatures make unreachable. The contract crate's
  `projection` module carries the mechanism.
- **Pin [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit)
  exactly** if you depend on it. It carries its own version, and adding a
  conformance rule is a semver-*minor* change there that can turn a passing
  adapter red — `0.2.0` does exactly that, twice.
- **What changed is in [`CHANGELOG.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/CHANGELOG.md)**, per release, in a caller's terms.

## What DCB buys you

There are no aggregates. An event carries a type, opaque bytes and a set of tags;
a query selects across them; a command reads what it needs, folds it, decides, and
appends conditioned on nothing new having appeared. The consistency boundary is
whatever that query selects — chosen per decision, and free to span what
aggregates would have separated without reaching for a saga.

One enum of events, one struct that folds them, and one call that reads,
decides, appends and retries — with the boundary enforced by the store rather
than by the code remembering to check:

```rust
use happenstance::{bytes::Bytes, Codec, CodecError, DecisionModel};
use happenstance::{DomainEvent, EventType, MemoryEventStore, Retry};
use happenstance::{Tags, commit};
use std::error::Error;

// Named once and returned by name. Indexing `EVENT_TYPES` in a match arm
// couples the arm to a position in a separate list, so reordering that list
// silently relabels the event.
const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");

#[derive(serde::Serialize, serde::Deserialize)]
enum Seat { Taken }

impl DomainEvent for Seat {
    const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
    fn event_type(&self) -> EventType { SEAT_TAKEN }
    fn tags(&self) -> Tags { Tags::empty() }
    fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
        c.encode(self)
    }
    fn decode<C: Codec>(c: &C, t: &EventType, d: &Bytes)
        -> Result<Self, CodecError> {
        // The event type is a guard, not decoration: without it a payload
        // written under another type decodes silently into this one.
        if !Self::EVENT_TYPES.contains(t) {
            let event_type = t.clone();
            return Err(CodecError::UnknownEventType { event_type });
        }
        c.decode(d)
    }
}

#[derive(Clone)]
struct Seats { scope: Tags, taken: u32 }

impl DecisionModel for Seats {
    type Event = Seat;
    fn scope(&self) -> &Tags { &self.scope }
    fn apply(&mut self, event: Seat) {
        match event { Seat::Taken => self.taken += 1 }
    }
}

# #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
let store = MemoryEventStore::new();
let seats = Seats { scope: Tags::empty(), taken: 0 };
let retry = Retry::attempts(core::num::NonZeroU32::new(3).unwrap());

// The closure sees a model folded from a fresh read on every attempt, so a
// decision is never taken against state a concurrent writer has moved.
let take = |seats: &Seats| {
    Ok::<_, core::convert::Infallible>(
        if seats.taken < 5 { vec![Seat::Taken] } else { vec![] },
    )
};

let done = commit(&store, seats, retry, take).await?;
assert_eq!(done.committed().expect("a seat was taken").attempts, 1);
# Ok::<(), Box<dyn Error>>(()) }
```

## Guarantees

- `#![forbid(unsafe_code)]`, workspace-wide.
- Every switch `happenstance-core` has is re-declared here and forwards to it
  unchanged in meaning — `std`, `serde`, `memory`, `unstable-projection`. Three
  are this crate's own and forward nothing: the `json`, `postcard` and `cbor`
  codecs, one optional dependency each (`serde_json`, `postcard`, `ciborium`).
  Encoding is what the typed layer is for, so that is where they belong. It does
  mean `default-features = false` is not the same act on both crates: `json` is
  in these defaults, so it drops a codec and a type here and nothing of the kind
  there. If you are auditing a minimal graph, those three are what to look at —
  and `conformance` is the one switch that exists only in the contract crate.
- MSRV 1.97.1, checked in CI. Raised from 1.85 at phase 2 by a *dependency's*
  build script rather than by this crate's own code —
  [ADR-0029](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0029-msrv-raised-to-1-97-1.md)
  records the measurement and the trade.

## Design

The design is specified rather than described:
[`spec/SPECIFICATION.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)
carries the normative clauses, each with a maturity marker, the conformance rule
that checks it, and the wrong implementation it forbids.

## Licence

MIT OR Apache-2.0.
