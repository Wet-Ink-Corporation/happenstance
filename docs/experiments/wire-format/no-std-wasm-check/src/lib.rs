//! M1 (`phase5-my-measurements.md`) — does a `no_std` + `alloc` `base64`
//! build check clean on `wasm32-unknown-unknown`?
//!
//! Backs the first half of WF-11's falsifier
//! (`SPECIFICATION.md:2105-2107`): "falsified if the base64 implementation
//! cannot be made `no_std`". This crate is the smallest thing that can
//! answer that: `#![no_std]`, one `alloc` import, `STANDARD.encode` /
//! `STANDARD.decode`, nothing else.
//!
//! There is no `#[test]` here on purpose — `wasm32-unknown-unknown` has no
//! test harness to run against by default, and the question this crate
//! answers is "does it type-check and compile for the target", not "does a
//! test pass". Run it with:
//!
//! ```console
//! $ cd docs/experiments/wire-format/no-std-wasm-check
//! $ cargo check --target wasm32-unknown-unknown
//! ```
//!
//! Measured (`phase5-my-measurements.md`, M1): `Checking base64 v0.22.1` /
//! `Finished` in ~2.7s, no warnings. The `no_std` half of WF-11's falsifier
//! is answered by measurement, not by inference.

#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

/// Encodes bytes as standard base64, using only `alloc`.
pub fn enc(b: &[u8]) -> String {
    STANDARD.encode(b)
}

/// Decodes standard base64 back to bytes, using only `alloc`.
pub fn dec(s: &str) -> Option<Vec<u8>> {
    STANDARD.decode(s.as_bytes()).ok()
}
