//! Validates the conformance suite against the reference implementation.
//!
//! This runs in both directions at once. It checks that
//! [`MemoryEventStore`](happenstance_core::MemoryEventStore) is DCB-compliant, and —
//! more importantly — it checks that the suite itself is sane. A rule that no
//! correct store can pass is worse than no rule at all, because an adapter
//! author will spend a day believing their code is broken.

use happenstance_core::MemoryEventStore;

happenstance_testkit::event_store_conformance!(MemoryEventStore::new());
