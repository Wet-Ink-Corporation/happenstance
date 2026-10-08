//! Does `EventStore::append` keep `&[Event]`, or take `Vec<Event>`?
//!
//! The ES-17 two-build measurement, taken against `happenstance-cloudflare` —
//! the one adapter in the workspace whose write path copies each payload into an
//! owned row value (`write_rows`, `crates/happenstance-cloudflare/src/event_store.rs`)
//! and so the one ADR-0012 item 4 says could benefit. See `README.md` for the
//! decision rule, which was written before any of this ran.
//!
//! # The arms
//!
//! * **B0**, [`cloudflare::B0`]: `write_rows` + `write_tag_rows` + the identity
//!   stamp, verbatim, through the published `SqlStorage::exec`. Held to the real
//!   `CloudflareEventStore::append`'s allocation count before it is trusted.
//! * **B1**, [`cloudflare::B1`]: the same statements and values, bound without
//!   the `to_binding` clone (`sql_storage.rs:81-82`): borrowed, one Rust copy
//!   per payload, and the arm O1 is judged against. It is not the floor for a
//!   borrowed batch: `worker`'s `SqlStorage::exec_raw` (`worker-0.8.5/src/sql.rs:220`)
//!   could bind `Uint8Array::from(&[u8])` and `JsValue::from_str(&str)` straight
//!   from the borrow, with no Rust copy at all and fewer than O1 makes. Not
//!   measured; a stronger borrowed arm could only widen the gap O1 must clear.
//! * **O1**, [`cloudflare::O1`]: the batch owned; each payload and metadata
//!   buffer *moved* into its binding.
//!
//! [`memory_arms`] is the host-side control and the caller-side axis.
//!
//! # The instrument
//!
//! The counting `#[global_allocator]` is `event-clone-allocations`'s, linked by
//! path: that crate's `lib.rs` installs it in every binary that links it. This
//! crate writes no `unsafe` of its own. [`measured`] is its region timer
//! re-exported, so a reader sees the one instrument named in one place; a test
//! that asserts an exact count measures through [`region::hold`] instead, which
//! keeps parallel tests out of its region.

pub mod batch;
#[cfg(target_arch = "wasm32")]
pub mod cloudflare;
pub mod contention;
pub mod exact;
pub mod memory_arms;
pub mod poll;
pub mod region;
pub mod stats;

/// The instrument: `measure` (counts a region), `snapshot` and `Counts`.
pub use event_clone_allocations::counting as measured;
