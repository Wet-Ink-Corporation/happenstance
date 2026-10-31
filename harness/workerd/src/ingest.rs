//! The ingest port on `workerd`: `happenstance-sync`'s ingest family, run
//! against `CloudflareIngestStore` inside the object (#108 part E3).
//!
//! * [`fixture`]: one isolated log per fixture instance, as the event-store
//!   family's, with the store built by the **published** constructor over the
//!   object's own `DurableObjectState`, and SY-30's fault armed by a trigger on
//!   the namespace's own event table.
//! * [`dispatch`]: the rule, by name, from the family's own enumeration.
//! * [`classes`]: F's D7 failure classes, each judged by an identical retry on
//!   the same object.
//! * [`ceiling`]: one attempt of the largest ingest call the runtime commits;
//!   the search is the runner's (`ingest-ceiling.mjs`).
//!
//! The routes are `src/object/ingesting.rs`'s, apart from the event-store
//! family's because both families name their declension check alike.

pub mod ceiling;
pub mod classes;
pub mod dispatch;
pub mod fixture;
