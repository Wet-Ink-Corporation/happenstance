//! The JS boundary, modelled: an opaque handle and the two error shapes ES-6
//! has to choose between.
//!
//! # Why there is a stand-in here at all
//!
//! The real type is [`wasm_bindgen::JsValue`]. Depending on it would pull
//! `wasm-bindgen`, `js-sys` and `worker` through `cargo deny` on every run of
//! the gate to buy one property — that a value from the JS heap is a handle, not
//! data. [`JsHandle`] has that property and nothing else.
//!
//! [`wasm_bindgen::JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
//!
//! # The one place the stand-in is deliberately stricter than the original
//!
//! `wasm-bindgen` 0.2.126 declares (`src/lib.rs:168-176`):
//!
//! ```text
//! pub struct JsValue {
//!     idx: u32,
//!     _marker: PhantomData<*mut u8>, // not at all threadsafe
//! }
//!
//! #[cfg(not(target_feature = "atomics"))]
//! unsafe impl Send for JsValue {}
//! #[cfg(not(target_feature = "atomics"))]
//! unsafe impl Sync for JsValue {}
//! ```
//!
//! So a real `JsValue` **is** `Send + Sync` on the target Workers actually
//! builds — single-threaded wasm, no `atomics` — and stops being so the moment
//! wasm threads are switched on. The escape hatch is an `unsafe impl`, and this
//! workspace sets `unsafe_code = "forbid"`, so a happenstance adapter cannot
//! write that line itself; it can only inherit it by holding a `JsValue`.
//!
//! [`JsHandle`] holds an `Rc<str>` instead, which is `!Send` unconditionally.
//! That is on purpose: this crate is the workspace's ES-6 instrument, and an
//! instrument whose `!Send`-ness is a `cfg` away from evaporating cannot falsify
//! anything. What it costs is stated rather than hidden — see the crate
//! documentation's ES-6 section for what that means for the bound.

use std::rc::Rc;

/// An opaque handle to a value living on the JavaScript side.
///
/// Stands in for `wasm_bindgen::JsValue`. The payload is an `Rc<str>` because
/// `Rc` is what surrenders `Send` — `RefCell<T>` is `Send` whenever `T: Send`,
/// it gives up `Sync`, and a store built out of one proves nothing about the
/// bare flavour. That trap is recorded in `happenstance-testkit`'s
/// `tests/local_conformance.rs:49-55`; this type exists partly so it is not
/// walked into twice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsHandle {
    /// The JS-side representation. In the real thing this is a `u32` index into
    /// a table owned by the generated glue code; here it is the text that table
    /// would have yielded, so [`stringify`](Self::stringify) can be real rather
    /// than `todo!()`.
    repr: Rc<str>,
}

impl JsHandle {
    /// Wraps a JS-side representation.
    #[must_use]
    pub fn new(repr: &str) -> Self {
        Self {
            repr: Rc::from(repr),
        }
    }

    /// Coerces the value to a string, as `String(value)` does in JS.
    ///
    /// This is the operation ADR-0009 turns on: it is total, it is cheap, and
    /// its result is `Send + Sync`. What it discards is everything reachable
    /// only through [`property`](Self::property).
    #[must_use]
    pub fn stringify(&self) -> String {
        self.repr.to_string()
    }

    /// Reads a named property, as `Reflect::get` does.
    ///
    /// This is the capability a stringified error does not have, and the whole
    /// content of the "does stringifying lose anything?" question: a caller
    /// holding the handle can still ask for `code`, `cause` or a
    /// vendor-specific field that nobody thought to put in the message.
    ///
    /// # Errors
    ///
    /// Returns [`JsThrow`] if the lookup itself throws, which `Reflect::get`
    /// can do on a revoked `Proxy`.
    pub fn property(&self, _name: &str) -> Result<Option<Self>, JsThrow> {
        todo!("phase 9: Reflect::get through js-sys")
    }
}

/// A value thrown across the JS boundary, kept as thrown.
///
/// The shape `worker::Error::Internal(JsValue)` and
/// `worker::Error::UnknownJsError { original: JsValue, .. }` take.
#[derive(Debug, Clone, thiserror::Error)]
#[error("JavaScript threw: {}", .thrown.stringify())]
pub struct JsThrow {
    /// The thrown value, unstringified.
    pub thrown: JsHandle,
}

impl JsThrow {
    /// Wraps a thrown value.
    #[must_use]
    pub fn new(thrown: JsHandle) -> Self {
        Self { thrown }
    }

    /// Whether the thrown value looks like a SQLite constraint violation.
    ///
    /// Reached by property lookup on the live value. Compare
    /// [`StringifiedThrow::is_constraint_violation`], which answers the same
    /// question from the message text alone — the comparison is the evidence
    /// ADR-0009 asks for, and the finding is that on this question the two
    /// answer identically, because Workers puts the SQLite error text in
    /// `.message` and exposes no numeric `code`.
    ///
    /// # Errors
    ///
    /// Returns itself if the property lookup throws.
    pub fn is_constraint_violation(&self) -> Result<bool, Self> {
        todo!("phase 9: probe `.message` and `.code` on the live value")
    }
}

/// A value thrown across the JS boundary, stringified at the boundary.
///
/// The shape `worker::Error::JsError(String)` takes, and the shape ES-6 would
/// force on every adapter if `Error: Send + Sync` were added to the port.
///
/// Unlike [`JsThrow`] this is `Send + Sync`, which is the entire trade.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("JavaScript threw: {message}")]
pub struct StringifiedThrow {
    /// `String(value)` applied at the boundary.
    pub message: String,
}

impl StringifiedThrow {
    /// Stringifies a thrown value at the boundary.
    #[must_use]
    pub fn new(thrown: &JsHandle) -> Self {
        Self {
            message: thrown.stringify(),
        }
    }

    /// Whether the message looks like a SQLite constraint violation.
    ///
    /// A Durable Object surfaces SQLite's own text — `UNIQUE constraint failed:
    /// event.position` — through the thrown `Error`'s `message`, so this is a
    /// substring test rather than a code comparison. It is uglier than
    /// [`JsThrow::is_constraint_violation`] and, on the evidence, no less
    /// capable.
    #[must_use]
    pub fn is_constraint_violation(&self) -> bool {
        self.message.contains("constraint failed")
    }
}
