//! What one `Event` costs to clone, and what the encode path costs on top.
//!
//! Four documentation sites in this repository state the cost of cloning an
//! [`Event`](happenstance_core::Event) as two heap allocations:
//!
//! * `crates/happenstance-core/src/event.rs:404-413` — "leaving one `Box<str>`
//!   and one boxed tag slice";
//! * `crates/happenstance-core/src/memory.rs:30-31` — "Cloning is cheap
//!   regardless: payloads are `Bytes`, so a snapshot bumps refcounts rather than
//!   copying data";
//! * `spec/SPECIFICATION.md:3370-3373` (ES-17, `[PROVISIONAL]`) — the same
//!   sentence, in the clause that owns `append`'s ownership decision;
//! * `references/adr/0012-append-shape-and-preconditions.md:173` — "`Bytes` are
//!   refcounted, so `event.clone()` bumps a counter rather than copying a
//!   payload".
//!
//! This crate answers whether that is true, and it answers it **twice**, because
//! there are two regimes and only one of them is what an application is in. See
//! [`arms`] for the split and [`readpath`] for the second question — whether
//! `MemoryEventStore::read` pays that cost once per *matched* event or once per
//! *returned* event.
//!
//! # The instrument, and why it lives in `experiments/`
//!
//! [`counting::Counting`] is a `#[global_allocator]`. It cannot live in a
//! workspace member for two independent reasons, either of which alone would be
//! sufficient: `GlobalAlloc` requires `unsafe impl` and the workspace root sets
//! `unsafe_code = "forbid"`, which no inner attribute can waive; and a global
//! allocator is a per-binary singleton, so declaring one in `happenstance-core`
//! would install it into every test binary that links the crate. `Cargo.toml`
//! carries an empty `[workspace]` table so cargo never adopts this crate, and
//! nothing in `cargo xtask ci` or `.redkiln/config.yaml` invokes `run.sh`
//! (CF-34).
//!
//! # Reading the numbers
//!
//! Conformance first. `tests/arms_are_equivalent.rs` establishes that the arms
//! are the same value encoded the same bytes before `tests/measure_clone.rs` or
//! `tests/measure_read.rs` prints anything, because an arm that is cheap by
//! virtue of doing less is not a faster arm, it is a different arm.

pub mod arms;
pub mod counting;
pub mod readpath;

/// The counting allocator, installed for every binary that links this crate —
/// which is this crate's own tests, and nothing else in the repository.
#[global_allocator]
static ALLOCATOR: counting::Counting = counting::Counting;
