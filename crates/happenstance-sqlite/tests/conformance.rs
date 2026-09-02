//! The bar, run whole against a real file on disk.
//!
//! An adapter that compiles is not an adapter. This target is where
//! `happenstance-sqlite` stops being an instrument: one `SqliteFixture`, one
//! macro invocation, and every rule in `for_each_event_store_rule!` emitted as
//! its own `#[tokio::test]` driven against SQLite.
//!
//! The fixture itself lives in [`support`] — shared with
//! `tests/concurrency.rs`, and under `tests/support/` rather than at
//! `tests/support.rs` because Cargo compiles every file *directly* under
//! `tests/` as its own binary. Read that module for what the fixture is made
//! of; three of this project's criteria are won or lost in its body rather than
//! in any rule's output.
//!
//! # The model family runs here too, and it is the one that reads generated SQL
//!
//! `event_store_conformance!` is a list of examples somebody wrote down. The
//! model family is not: it generates a `query × from × backwards × limit ×
//! condition × position policy` space and asks whether this adapter's SQL agrees
//! with an independently-written model of the contract on sequences nobody
//! chose. Against a store whose statements were written to satisfy named
//! examples, that is the first instrument in the workspace capable of finding a
//! *generated* disagreement — a page boundary, a chunk merge, a `resume_from`
//! inclusivity — that is right on every example.
//!
//! Both families mount **here**, sharing one [`support::SqliteFixture`]
//! definition, because they answer one question — *has this adapter passed the
//! bar* — and a reader should meet them in one output. The racing family is the
//! one that does not, and `tests/concurrency.rs` says why.
//!
//! ## Why there is no `#![cfg(feature = "proptest")]` here
//!
//! `event_store_model_conformance!` exists only behind the testkit's
//! off-by-default `proptest` feature, and the reference harness
//! `crates/happenstance-testkit/tests/memory_model_conformance.rs:27` gates
//! itself on exactly that. **That form is unavailable here.**
//! `cfg(feature = "…")` names the *current* crate's features, so this crate
//! cannot `cfg` on a dependency's feature; the feature is enabled at the
//! `[dev-dependencies]` site instead and the invocation is gated on
//! `not(target_arch = "wasm32")` alone, which the crate-level attribute below
//! already supplies. `cargo test -p happenstance-sqlite` with no flags is
//! therefore the whole command, which is what the project's testing brief
//! promises.
//!
//! ## Why only the tokio emitter, and why that is a decision rather than an
//! omission
//!
//! The reference harness invokes **both** shipped emitters, as CF-23's
//! demonstration that the wrapper is still a parameter. Copying that is wrong
//! here for a mechanical reason: `__emit_model_blocking` drives the rule under
//! the testkit's own `block_on` with **no tokio runtime anywhere**, and this
//! adapter's read stream defers a `spawn_blocking` into `poll_next` — a hop that
//! resolves through the handle captured at construction and, failing that,
//! `Handle::try_current()`. Under the blocking emitter both are absent, so every
//! generated `read` would fail with `SqliteEventStoreError::NoRuntime`: a
//! *runtime* fact, not a conformance defect, arriving as a red rule. CF-23 is
//! satisfied for this family inside the testkit, by the harness that can
//! honestly run both.
//!
//! ## The pass column, and where it is honestly discharged
//!
//! `RUNBOOK.md`'s phase-8 proof artefact asks for *"the phase-3 mutant harness
//! re-run with `SqliteEventStore` in the pass column"*, and the pass-column
//! meta-test is `mutation_coverage::conformant_variants_pass_everything` — whose
//! subjects are `Rc`/`RefCell` primitives built inside the **testkit's own** test
//! binary. Registering this adapter there was considered and **refused**, for two
//! reasons: it would make a published crate dev-depend on its own consumer, and
//! it would drag `rusqlite` and a tokio runtime into a harness deliberately built
//! to need neither.
//!
//! What that meta-test actually asserts is *every registered rule driven against
//! a conformant store*, and that is precisely what this file is — the sequential
//! family, the model family, and `tests/concurrency.rs`'s racing family, all
//! against one conformant `SqliteFixture` on a real file. The refusal is recorded
//! here, where a reviewer holding the runbook meets it, rather than only in a
//! ledger a code reader will never open.
//!
//! The tokio harness, so: native only.

#![cfg(all(feature = "event-store", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

mod support;

use support::SqliteFixture;

happenstance_testkit::event_store_conformance!(SqliteFixture::new());

happenstance_testkit::event_store_model_conformance!(SqliteFixture::new());
