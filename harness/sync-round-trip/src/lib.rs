//! An instrument, never published: a native SQLite store replicating through
//! the Neon and Postgres sync peers (#131).
//!
//! # Why a crate of its own
//!
//! The round trip needs two adapters in one build, `happenstance-sqlite` at the
//! near end and `happenstance-neon` or `happenstance-postgres` at the far end,
//! and no adapter may depend on another. An instrument crate is not an
//! adapter, so the rule holds; `harness/workerd` is the precedent, for the
//! Durable Object's round trip (#130).
//!
//! # What it checks
//!
//! The test binary, `tests/round_trip/`, runs two cases against each far
//! end:
//!
//! * the near store's own log pushed through the peer and pulled back, group
//!   for group; then the far store's own append pulled and ingested into a
//!   second SQLite store, with every payload, metadata and tag set byte for
//!   byte;
//! * a pushed batch replayed, which changes nothing.
//!
//! # Live, and gated
//!
//! Every case is `#[ignore]`d with its reason, so a machine without a server
//! keeps a green `cargo xtask ci`. CI runs the Neon cases in its `live-neon`
//! job and the Postgres cases in `live-postgres`, each with `-- --ignored`.
//! The Neon cases read `NEON_CONNECTION` and fail, rather than pass over
//! nothing, without it; the Postgres cases read `HAPPENSTANCE_POSTGRES_URL`,
//! or start or attach to the shared conformance container.
//!
//! This library is empty: Cargo needs a target, and the crate's work is its
//! test binary.
