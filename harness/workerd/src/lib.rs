//! Every event-store conformance rule, run by `workerd` inside a real Durable
//! Object — the runtime `happenstance-cloudflare` exists for, rather than the
//! `node:sqlite` shim the gate runs it on.
//!
//! # Which kind of `publish = false` this is
//!
//! An **instrument**, never a release candidate. It is neither the unfinished
//! kind (`happenstance-sync`) nor the finished-and-held-back kind the workspace
//! has had and no longer has: it is a test harness that compiles to a Worker,
//! and nothing here is a consumer's API.
//!
//! # The shape, and the one thing it adds
//!
//! `object` (private, for the reason its own documentation gives) holds a
//! `#[durable_object]` class built the way a production class
//! is built — `SqlStorage::from_state` over the object's own state, handed to
//! `CloudflareEventStore` — and a fetch handler that runs one rule by name.
//! [`dispatch`] is the emitter, written under CF-23's extension point: it expands
//! [`for_each_event_store_rule!`](happenstance_testkit::for_each_event_store_rule)
//! into a `match` from rule name to rule. The names the test runner iterates come
//! from the **same** enumeration through a different callback, so a rule dropped
//! from the dispatch table is a red `no such rule` rather than a silent pass.
//!
//! The one thing this harness needs that the shim harness does not is a second
//! isolated store inside one object, because a Durable Object has exactly one
//! database and two rules open two fixture instances at once. Each fixture
//! instance therefore keeps its log under its own
//! [`TableNamespace`](happenstance_cloudflare::TableNamespace), through the
//! adapter's public constructor — the store under test is the store a consumer
//! builds, not a test-only variant of it.
//!
//! [`probe`] is the other half of the job: the platform's SQLite walls, measured
//! on the runtime that imposes them (`experiments/durable-object-limits`).
//!
//! # Running it
//!
//! See `harness/workerd/README.md`. The CI job is `workerd` in
//! `.github/workflows/ci.yml`, a sibling of `gate` and never a step inside it.

pub mod dispatch;
pub mod fixture;
mod object;
pub mod probe;
