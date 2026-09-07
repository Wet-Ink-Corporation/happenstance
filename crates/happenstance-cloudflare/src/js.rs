//! The JS boundary, **bound** rather than modelled: the live thrown value and
//! the two error shapes ES-6 has to choose between.
//!
//! # What replaced the stand-in, and what did not move
//!
//! Until phase 9 this module modelled `wasm_bindgen::JsValue` with an
//! `Rc<str>`. The model is gone; the *shape* it was built to hold is not.
//! [`JsHandle`] now carries a real [`JsValue`] and [`JsThrow`] a real
//! [`worker::Error`], and both are reached through an [`Rc`], which is the one
//! detail this module cannot give up.
//!
//! # Why the payload is `Rc`-shaped, unconditionally
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
//! builds — single-threaded wasm, no `atomics` — and on the host as well, where
//! `target_feature = "atomics"` is likewise unset. `worker` then adds two more
//! of its own (`worker-0.8.5/src/sql.rs`: `unsafe impl Send for SqlStorage {}`,
//! `unsafe impl Sync for SqlStorage {}`, and the same pair for its cursor).
//! The escape hatch is an `unsafe impl` in all four cases, and this workspace
//! sets `unsafe_code = "forbid"`, so a happenstance adapter cannot write that
//! line itself — it can only *inherit* it by holding one of those types.
//!
//! `Rc<T>` is `!Send` for every `T`, including a `T` that is `Send`. Holding
//! the real value through an `Rc` therefore keeps the thrown value **live** —
//! [`JsThrow::thrown`] hands it back, [`JsHandle::property`] reads a field off
//! it — while leaving this crate the workspace's only error type that can fail
//! a `Send + Sync` bound. That is the whole ES-6 instrument, and it is why the
//! swap this module just went through did not restore `Send`-ness by accident:
//! the probes in the crate root run on the host *and* on `wasm32`, because an
//! auto-trait leak that arrives through `cfg(not(target_feature = "atomics"))`
//! can only be observed where the code is compiled.

use std::rc::Rc;

use worker::js_sys::{Array, Reflect};
use worker::wasm_bindgen::JsValue;

/// The text SQLite puts in a constraint failure, on every binding of it.
///
/// A Durable Object surfaces SQLite's own message — `UNIQUE constraint failed:
/// event.position` — through the thrown `Error`'s `message`, and exposes no
/// numeric code. Spelled once so [`JsThrow`] and [`StringifiedThrow`] cannot
/// drift apart, and so a future change to the Workers text is one edit.
const CONSTRAINT_TEXT: &str = "constraint failed";

/// The `code` a driver would set if it set one.
///
/// Probed for completeness rather than because Workers sets it: the finding
/// recorded in the crate documentation is that it does not, and a probe that
/// only ever misses is how a finding stays checkable instead of becoming
/// folklore.
const CONSTRAINT_CODE: &str = "SQLITE_CONSTRAINT";

/// Whether a message carries SQLite's own constraint-failure text.
fn reads_as_constraint_violation(message: &str) -> bool {
    message.contains(CONSTRAINT_TEXT)
}

/// An opaque handle to a value living on the JavaScript side.
///
/// The payload is an `Rc<JsValue>` rather than a bare `JsValue`, and the `Rc`
/// is load-bearing rather than ergonomic — see this module's documentation.
/// `Rc` is what surrenders `Send`; `RefCell<T>` would not do, because it is
/// `Send` whenever `T` is, it gives up `Sync` instead, and a store built out of
/// one proves nothing about the bare flavour. That trap is recorded in
/// `happenstance-testkit`'s `tests/local_conformance.rs`; this type exists
/// partly so it is not walked into twice.
#[derive(Debug, Clone)]
pub struct JsHandle {
    /// The JS-side value, held so that the handle is `!Send` however `JsValue`
    /// is declared on the target being compiled for.
    repr: Rc<JsValue>,
}

impl JsHandle {
    /// Takes ownership of a value from the JavaScript side.
    #[must_use]
    pub fn new(value: JsValue) -> Self {
        Self {
            repr: Rc::new(value),
        }
    }

    /// The value itself, still live.
    #[must_use]
    pub fn as_js(&self) -> &JsValue {
        &self.repr
    }

    /// Coerces the value to a string, as `String(value)` does in JS.
    ///
    /// This is the operation ADR-0009 turns on: it is total, it is cheap, and
    /// its result is `Send + Sync`. What it discards is everything reachable
    /// only through [`property`](Self::property).
    ///
    /// `js-sys` exposes no binding to the global `String` function, so the
    /// coercion is reached through `Array.prototype.join`, which applies
    /// exactly `String(element)` to each element. `join` renders `null` and
    /// `undefined` as the empty string, which `String` does not, so those two
    /// are answered directly.
    #[must_use]
    pub fn stringify(&self) -> String {
        let value: &JsValue = &self.repr;
        if value.is_null() {
            return "null".to_owned();
        }
        if value.is_undefined() {
            return "undefined".to_owned();
        }
        if let Some(text) = value.as_string() {
            return text;
        }
        Array::of1(value).join("").into()
    }

    /// Reads a named property, as `Reflect::get` does.
    ///
    /// This is the capability a stringified error does not have, and the whole
    /// content of the "does stringifying lose anything?" question: a caller
    /// holding the handle can still ask for `code`, `cause` or a
    /// vendor-specific field that nobody thought to put in the message.
    ///
    /// A property that is absent reads as `None` rather than as a handle onto
    /// `undefined`, because a caller branching on "is this field here" should
    /// not have to know which of the two JS spells absence.
    ///
    /// # Errors
    ///
    /// Returns [`JsThrow`] if the lookup itself throws, which `Reflect::get`
    /// can do on a revoked `Proxy`.
    pub fn property(&self, name: &str) -> Result<Option<Self>, JsThrow> {
        let key = JsValue::from_str(name);
        match Reflect::get(&self.repr, &key) {
            Ok(value) if value.is_undefined() => Ok(None),
            Ok(value) => Ok(Some(Self::new(value))),
            Err(thrown) => Err(JsThrow::new(&Self::new(thrown))),
        }
    }
}

/// A value thrown across the JS boundary, kept as thrown.
///
/// It carries a real [`worker::Error`] — the same value a Durable Object hands
/// back out of `sql.exec()` — through an `Rc`, which is what keeps the type
/// `!Send` and what makes [`Clone`] free rather than impossible
/// (`worker::Error` is not `Clone`).
///
/// `worker` classifies a thrown value on the way in: a real `Error` object
/// becomes `Error::UnknownJsError` with `name`, `message` and `code` cached and
/// **`original` retained**, a thrown string becomes `Error::JsError`, and
/// anything else becomes `Error::Internal` with the value retained. So "kept as
/// thrown" is literally true for the two variants a `SqlStorage` failure
/// actually produces.
#[derive(Debug, Clone, thiserror::Error)]
#[error("JavaScript threw: {error}")]
pub struct JsThrow {
    /// The thrown value, unstringified, as `worker` classified it.
    error: Rc<worker::Error>,
}

impl JsThrow {
    /// Wraps a thrown value, classifying it the way `worker` does.
    #[must_use]
    pub fn new(thrown: &JsHandle) -> Self {
        Self::from_error(worker::Error::from(thrown.as_js().clone()))
    }

    /// Wraps an error `worker` has already classified.
    ///
    /// This is the constructor the `SqlStorage` binding reaches for, because
    /// `worker::SqlStorage::exec` has already done the conversion by the time
    /// the adapter sees a failure.
    #[must_use]
    pub fn from_error(error: worker::Error) -> Self {
        Self {
            error: Rc::new(error),
        }
    }

    /// The classified error, which is also what [`Display`](core::fmt::Display)
    /// renders.
    #[must_use]
    pub fn error(&self) -> &worker::Error {
        &self.error
    }

    /// The live thrown value, when one survived classification.
    ///
    /// `None` for the variants `worker` builds out of Rust rather than out of a
    /// throw — there is no JS value behind those, and inventing one would be
    /// the quiet stringification this crate exists to avoid.
    #[must_use]
    pub fn thrown(&self) -> Option<JsHandle> {
        match &*self.error {
            worker::Error::UnknownJsError { original, .. } | worker::Error::Internal(original) => {
                Some(JsHandle::new(original.clone()))
            }
            _ => None,
        }
    }

    /// Whether the thrown value looks like a SQLite constraint violation.
    ///
    /// Reached by property lookup on the live value where there is one, and by
    /// the message `worker` cached at conversion where there is not. Compare
    /// `StringifiedThrow::is_constraint_violation` — named rather than linked,
    /// because that type is `pub(crate)` and a link to it from this public page
    /// is `rustdoc::private_intra_doc_links`, which the gate denies — which
    /// answers the same question from the message text alone — the comparison is the evidence
    /// ADR-0009 asks for, and the finding is that on this question the two
    /// answer identically, because Workers puts the SQLite error text in
    /// `.message` and exposes no numeric `code`.
    ///
    /// # Errors
    ///
    /// Returns itself if the property lookup throws.
    pub fn is_constraint_violation(&self) -> Result<bool, Self> {
        let Some(thrown) = self.thrown() else {
            // No live value: the cached message is all there is, which is the
            // stringified answer under another name. Recorded rather than
            // hidden — it is the case where the two shapes genuinely converge.
            return Ok(reads_as_constraint_violation(&self.error.to_string()));
        };

        if let Some(code) = thrown.property(CONSTRAINT_CODE_KEY)?
            && code.stringify() == CONSTRAINT_CODE
        {
            return Ok(true);
        }

        let message = match thrown.property(MESSAGE_KEY)? {
            Some(message) => message.stringify(),
            None => thrown.stringify(),
        };
        Ok(reads_as_constraint_violation(&message))
    }
}

/// The property a thrown `Error` carries its text in.
const MESSAGE_KEY: &str = "message";

/// The property a thrown `Error` would carry a driver code in, if it carried
/// one.
const CONSTRAINT_CODE_KEY: &str = "code";

/// A value thrown across the JS boundary, stringified at the boundary.
///
/// The shape `worker::Error::JsError(String)` takes, and the shape ES-6 would
/// force on every adapter if `Error: Send + Sync` were added to the port.
///
/// Unlike [`JsThrow`] this is `Send + Sync`, which is the entire trade — and it
/// is why this type stays in the tree after the real bindings landed. It is the
/// recorded alternative, and it is the positive control the crate root's
/// `!Send` probes need: without a type that the probe reports `true` for, the
/// whole probe module would also pass if the probe were simply broken.
///
/// # Readable, not mintable, and the difference is a lost update
///
/// [`is_constraint_violation`](Self::is_constraint_violation) is a substring
/// test over the message, so whoever writes the message decides what this crate
/// calls a conflict. The wrong implementation it forbids is not exotic — it is
/// an error-normalising middleware, a boundary mapper or a test double that
/// builds one of these with `UNIQUE constraint failed` somewhere in the text. A
/// DCB command loop then reads a transport fault as a lost append condition and
/// retries a decision the store never refused; the retry succeeds, so nothing in
/// any log says otherwise. That is the shape RS-13-1 names one crate over
/// (`standards/rust/13-sealing-and-exhaustiveness.md`), against a fabricated
/// `Guard`, with the message here in place of the position there. [`JsThrow`]'s
/// field has been private since it was written; this one is too.
///
/// `#[non_exhaustive]` is deliberately **not** applied on top. With the field
/// private the struct expression is already unbuildable downstream and no
/// downstream pattern can be exhaustive, so a second field — the numeric SQLite
/// code the crate root records Workers as not surfacing — is additive either
/// way. The attribute would restate a seal the field already holds, and RS-13-5
/// is the standing argument against decorating a type with one that buys
/// nothing. [`JsThrow`] carries none, for the same reason.
///
/// # Why this is `pub(crate)`, and what left with the `pub`
///
/// **Both of this type's in-crate roles are satisfied by a private type.** It
/// is the recorded alternative to [`JsThrow`], and it is
/// `the_probe_is_not_vacuous`'s positive control — the `Send` type without
/// which the crate root's whole `!Send` probe module would also pass if the
/// probe were simply broken. Neither role needs a caller.
///
/// **And nothing in the workspace ever consumed it as a caller would.** The
/// evidence for a public audience does not exist rather than being outweighed:
/// this crate has never been in anyone's hands — `happenstance-cloudflare`
/// holds a `0.0.0` placeholder on the registry, which is not a predecessor
/// because there was never anything under it — so `0.2.0` is the one release at
/// which withdrawing the name costs nobody anything. After it, the same change
/// is a major version.
///
/// **What went with the `pub` is four rendered doctests, and they are not
/// replaced in kind.** Two proved the struct literal is unbuildable downstream
/// and two that the field cannot be assigned on a value already held. Both
/// facts are still true and both are now *subsumed*: a caller who cannot name
/// the type cannot reach either seal to test it. `pub(crate)` is the strictly
/// stronger statement, checked by the compiler on every build of every crate
/// that depends on this one, which is a better instrument than a `compile_fail`
/// pair and a cheaper one. What the pair below proves is therefore the new seal
/// rather than the old ones — see `stringified_throw_is_crate_private` in the
/// crate root.
///
/// **The argument the deleted doctests carried is kept**, because it is the
/// reason the field was private before the type was: whoever writes the message
/// decides what this crate calls a conflict, so an error-normalising middleware
/// or a test double that puts `UNIQUE constraint failed` into the text makes a
/// DCB command loop read a transport fault as a lost append condition and retry
/// a decision the store never refused. The retry succeeds, so nothing in any
/// log says otherwise. That is RS-13-1's shape with the message here in place of
/// the position there, and it is why the field stays private even now that the
/// type is unreachable — the seal a future in-crate module meets is still the
/// field's.
///
/// # The consequence the brief did not name: in a shipped build, nobody uses it
///
/// Withdrawing the `pub` made the type dead code outside a test build, and
/// `-D warnings` said so immediately — *"struct `StringifiedThrow` is never
/// constructed"*, plus its four associated items. Both remaining uses are
/// `#[cfg(test)]`: the crate root's `!Send` probes, and this module's own
/// `wasm32` tests. That is not a surprise to be suppressed; it is the honest
/// arithmetic of Option B, and it is recorded here because a reader meeting the
/// `allow` below deserves to know it was reasoned about rather than reached for.
///
/// **`#[cfg(test)]` on the type would be the tidier answer and is the wrong
/// one.** The ratified decision is about *visibility*, and swapping it for a
/// compilation gate answers a different question — it would make the recorded
/// alternative to [`JsThrow`] stop existing in the artefact the crate actually
/// ships, so ES-6's fork would have one arm in the binary and one in the test
/// profile. The two shapes are meant to be comparable in the same build.
///
/// So the `allow` is scoped to `not(test)` rather than written flat. In a test
/// build the lint stays live, which means the day the probes stop using this
/// type — the day it becomes *genuinely* dead rather than dead-outside-tests —
/// `cargo test` says so. A flat `#[allow(dead_code)]` would silence that too,
/// and the only thing keeping this type honest is that something still asserts
/// on it.
///
/// **Neither of those two names is a link, and neither can be.** The `!Send`
/// probes live in `lib.rs`'s `#[cfg(test)] mod not_send_assertions`, and the
/// seal's compiled proof is its `#[cfg(doctest)] mod
/// stringified_throw_is_crate_private`. A documentation build compiles neither
/// configuration, so rustdoc resolves neither path and
/// `-D rustdoc::broken-intra-doc-links` rejects the attempt. Naming them in
/// prose is the spelling that cannot rot into a broken link — which is the same
/// trade this crate already makes one type up, where `JsThrow`'s page names
/// `StringifiedThrow::is_constraint_violation` without linking it because that
/// item is now private.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("JavaScript threw: {message}")]
#[cfg_attr(not(test), allow(dead_code, reason = "the only users are cfg(test)"))]
pub(crate) struct StringifiedThrow {
    /// `String(value)` applied at the boundary.
    ///
    /// Private, and read through [`message`](Self::message). The two doctests
    /// on the type say what that buys; the sibling seven lines of
    /// [`JsThrow`] is the same decision taken at the same boundary.
    message: String,
}

// Gated on the *wasm32* test build rather than on `test`, because that is
// where this impl's users actually are and a lint that is off where its subject
// is used is not a lint. The host `cfg(test)` build reaches the type — the
// `!Send` probe names it — and none of these four methods; only this module's
// `#[cfg(all(test, target_arch = "wasm32"))]` tests call them. So under
// `cargo test` on the host they are dead, under `cargo test` on wasm32 they are
// not, and the lint is live in exactly the second case. Widening this to
// `not(test)` would match the attribute on the type above and would switch the
// check off in the one configuration that can perform it.
#[cfg_attr(
    not(all(test, target_arch = "wasm32")),
    allow(dead_code, reason = "the callers are this module's wasm32 tests")
)]
impl StringifiedThrow {
    // `message(&self) -> &str` and `new(&JsHandle)` stood here and are gone,
    // and their absence is a result rather than tidying. Both were `pub` on a
    // `pub` type, which is a configuration in which `dead_code` can never fire:
    // an exported item always has a hypothetical caller. Narrowing the type to
    // `pub(crate)` made the compiler look, and it found that neither had a real
    // one — not in this crate, not in its tests, not on either target.
    //
    // `message` was the read half of the field seal and its whole justification
    // was written in terms of a caller ("a caller normalising errors, logging
    // one or matching on its text has everything they had when the field was
    // `pub`"). That caller is exactly who Option B withdrew, so the method
    // outlived its argument by one commit. `new` stringified a `JsHandle`
    // directly and was the same operation as `from_throw` one input earlier;
    // nothing ever reached for it. The `Display` impl reads the field, not the
    // accessor, so the type still renders its text and ES-6's comparison is
    // undamaged.
    //
    // Keeping them behind the `allow` below was the alternative and it loses:
    // that attribute exists to say "the users are in another configuration",
    // which would have been false of these two in every configuration there is.

    /// Stringifies a classified throw at the boundary.
    #[must_use]
    pub(crate) fn from_throw(throw: &JsThrow) -> Self {
        Self {
            message: throw.error().to_string(),
        }
    }

    /// Whether the message looks like a SQLite constraint violation.
    ///
    /// A Durable Object surfaces SQLite's own text — `UNIQUE constraint failed:
    /// event.position` — through the thrown `Error`'s `message`, so this is a
    /// substring test rather than a code comparison. It is uglier than
    /// [`JsThrow::is_constraint_violation`] and, on the evidence, no less
    /// capable.
    ///
    /// # The input is not the caller's to write
    ///
    /// The type's own documentation says why, and since the type became
    /// `pub(crate)` there is no caller who could write it. The narrower fact the
    /// deleted doctest pair proved is still worth stating, because it is about
    /// the *field* rather than the type and it is what a future in-crate module
    /// meets: `#[non_exhaustive]` blocks the struct literal and forces `..` in a
    /// pattern, and does nothing at all to an assignment on a value already held
    /// (RS-13-1, and RS-13-3's own "Remediation" note). A private field is what
    /// closes both, which is why the field stays private rather than being
    /// relaxed now that the type is unreachable.
    #[must_use]
    pub(crate) fn is_constraint_violation(&self) -> bool {
        reads_as_constraint_violation(&self.message)
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;
    use worker::js_sys;
    use worker::wasm_bindgen::JsValue;

    use super::{JsHandle, JsThrow, StringifiedThrow};
    use crate::host::durable_object;
    use crate::sql_storage::{SqlError, SqlValue};

    /// The exact text a Durable Object's SQLite puts in the thrown `Error`'s
    /// `message` when an append condition implemented as a unique index loses.
    const UNIQUE_VIOLATION: &str = "UNIQUE constraint failed: event.position";

    fn thrown(message: &str) -> JsThrow {
        let error: JsValue = js_sys::Error::new(message).into();
        JsThrow::new(&JsHandle::new(error))
    }

    /// The property lookup is real: the thrown value is still there and still
    /// answers, which is the capability [`StringifiedThrow`] gives up.
    #[wasm_bindgen_test]
    fn the_thrown_value_stays_live() {
        let throw = thrown(UNIQUE_VIOLATION);
        let live = throw
            .thrown()
            .expect("the thrown Error survived classification");

        let message = live
            .property("message")
            .expect("the lookup does not throw")
            .expect("an Error carries a message");
        assert_eq!(message.stringify(), UNIQUE_VIOLATION);

        assert!(
            live.property("code")
                .expect("the lookup does not throw")
                .is_none(),
            "and the finding holds: Workers exposes no numeric code to switch on"
        );
    }

    /// Finding 2, as an assertion: on the one question the port makes a caller
    /// ask, the live value and the stringified one answer identically.
    #[wasm_bindgen_test]
    fn a_unique_violation_reads_the_same_live_and_stringified() {
        let throw = thrown(UNIQUE_VIOLATION);
        assert!(
            throw
                .is_constraint_violation()
                .expect("the property lookup does not throw"),
            "the live value classifies it"
        );
        assert!(
            StringifiedThrow::from_throw(&throw).is_constraint_violation(),
            "and so does the string"
        );
    }

    /// The negative control. A classifier that answered `true` for everything
    /// would satisfy the row above and route every transport fault into
    /// `ConditionViolated`.
    #[wasm_bindgen_test]
    fn an_unrelated_failure_is_not_a_constraint_violation() {
        let throw = thrown("no such table: event");
        assert!(
            !throw
                .is_constraint_violation()
                .expect("the property lookup does not throw"),
            "an unrelated failure is not a conflict"
        );
        assert!(!StringifiedThrow::from_throw(&throw).is_constraint_violation());
    }

    /// The same classification, driven by SQLite rather than by a message this
    /// test wrote — the throw arrives out of `worker::SqlStorage::exec`.
    #[wasm_bindgen_test]
    fn a_real_unique_index_violation_classifies() {
        let sql = durable_object();
        sql.exec("CREATE TABLE probe (position INTEGER PRIMARY KEY)", &[])
            .expect("ddl");
        sql.exec("INSERT INTO probe VALUES (?)", &[SqlValue::Integer(1)])
            .expect("first insert");

        let conflict = sql
            .exec("INSERT INTO probe VALUES (?)", &[SqlValue::Integer(1)])
            .expect_err("the second insert violates the primary key");

        let SqlError::Thrown(throw) = conflict else {
            panic!("a constraint violation arrives as a throw");
        };
        assert!(
            throw
                .is_constraint_violation()
                .expect("the property lookup does not throw"),
            "and SQLite's own text is what says so: {throw}"
        );
    }
}
