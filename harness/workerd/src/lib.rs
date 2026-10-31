//! Every event-store conformance rule, and the ingest family, run by `workerd`
//! inside a real Durable Object — the runtime `happenstance-cloudflare` exists for, rather than the
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
//! # Across a restart
//!
//! A testkit rule runs inside one request, and an object cannot restart itself
//! mid-request. [`restart`] holds rules phased across two requests instead: a
//! before-half writes and returns its evidence as text, the runner restarts the
//! object (an eviction it drives, or a hibernation through `workerd`'s own
//! inactivity timer), and an after-half asserts. Each construction of the object
//! names itself with an [`incarnation::Incarnation`], so an after-half refuses
//! to run across a restart that did not happen. Three wrong stores,
//! [`restart::InstanceLog`], [`restart::FreshHighWater`] and
//! [`restart::MintsPerOpen`], fail exactly the rules the registry lists for
//! them, and only when the restart happens.
//!
//! [`idle`] asks the converse: that an idle object holding the adapter, with or
//! without a partly read stream, keeps nothing pending, so `workerd`'s
//! inactivity timer evicts it. [`idle::IdleHold`] parks the store and the
//! stream inside one future the object owns and polls by hand, and the runner
//! compares the instance that armed it with the one that answers later.
//!
//! [`remint`] runs `CloudflareEventStore::remint_identity` across a real
//! bookmark restore (`onNextSessionRestoreBookmark`), on a deployed object
//! only, because local `workerd` does not implement a restore. Beside it runs
//! the control that skips the re-mint, where an `EventId` must be reissued.
//!
//! [`ingest`] runs the other port, `happenstance-sync`'s ingest family, in the
//! same object (#108 part E3): every ingest rule against
//! `CloudflareIngestStore`, built by the published
//! `namespaced_from_object_state` over the object's own `DurableObjectState`,
//! so each call is one real `ctx.storage.transactionSync`. Beside the family it
//! holds a census of F's D7 failure classes, each judged by an identical retry
//! on the same object, and one attempt of the ingest ceiling, which the runner
//! bisects.
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
pub mod idle;
pub mod incarnation;
pub mod ingest;
mod object;
pub mod probe;
pub mod remint;
pub mod restart;
