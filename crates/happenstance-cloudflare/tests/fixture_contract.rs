//! The fixture's own contract, proved before the suite is aimed at it.
//!
//! An **integration** target on purpose. It is a second compilation unit, which
//! is the only place the Durable Object host's reachability constraint can
//! actually be observed — a bare `#[cfg(test)]` module in `src/lib.rs` is
//! invisible from here — and it is the same boundary the slice-mate's
//! conformance target sits on. Discovering that the host cannot be named from
//! `tests/` after the fixture merged is the failure this file exists to move
//! earlier.
//!
//! # Why the split between plain `#[test]` and `#[wasm_bindgen_test]`
//!
//! It is a property of the assertion, not a preference. `Fixture`'s five
//! associated constants are facts about this adapter that need no JavaScript
//! heap to read, so the criteria about *what the fixture declares* run in an
//! ordinary `cargo test -p happenstance-cloudflare` with no wasm toolchain
//! installed at all. The criteria about *what the fixture does* — isolation,
//! two handles onto one object, the schema seam, the store id — need a real
//! Durable Object, which exists only where a JS heap does, so they are
//! `#[wasm_bindgen_test]` cases under `#[cfg(target_arch = "wasm32")]` and are
//! executed by the gate's `wasm32 run of the conformance rules` step through
//! `xtask`'s `WASM_UNIT_TARGETS` row for this target.
//!
//! The test **names** are the contract either way (spec, *Acceptance criteria*).

mod support;

use happenstance_testkit::{Capability, Fixture};
use support::CloudflareFixture;

/// The reason string the trait's own `MID_BATCH_FAULT` default carries.
///
/// Transcribed rather than read through the trait, because reading it through
/// the trait is precisely the mistake this file is checking for: a fixture that
/// inherits the default and a fixture that restates it would then compare equal
/// to the same expression. Written out, a change to either side has something to
/// disagree with.
const INHERITED_MID_BATCH_FAULT_REASON: &str = "this fixture cannot make its store fail between two rows of one batch; \
     nothing in the port can reach inside an `append`, so the injection has \
     to come from the adapter and this one has none to offer";

/// Every word that would make a declined reason about *this* runtime.
///
/// A decline is only worth printing if it says why **this** runtime cannot, and
/// "not supported" satisfies every other check in this repository.
const RUNTIME_VOCABULARY: &[&str] = &["durable object", "isolate", "storage", "object's", "handle"];

/// Compiled proof that a type is **not** `Send`, with its own positive control.
///
/// Autoref specialisation, the only way to observe the *absence* of an auto
/// trait on stable, and the same shape `crates/happenstance-cloudflare/src/lib.rs`
/// uses. It is written out again here rather than imported because the module
/// there is `#[cfg(test)]` and therefore invisible from a second compilation
/// unit — which is the same fact `the_host_is_reachable_from_an_integration_test`
/// is about, met from the other side.
mod not_send_probe {
    use core::marker::PhantomData;

    pub(crate) struct Probe<T>(pub(crate) PhantomData<T>);

    pub(crate) trait NotSend {
        fn is_send(&self) -> bool {
            false
        }
    }

    impl<T> NotSend for Probe<T> {}

    impl<T: Send> Probe<T> {
        #[allow(clippy::unused_self)]
        pub(crate) fn is_send(&self) -> bool {
            true
        }
    }
}

/// AC-002 — the fixture's `Store` is the bare, `!Send` flavour, and stays so.
///
/// The positive control is what makes it non-vacuous: without it the whole
/// assertion also passes when the probe is simply broken and always answers
/// `false`.
#[test]
fn the_fixture_store_is_not_send() {
    use core::marker::PhantomData;

    use not_send_probe::{NotSend as _, Probe};

    assert!(
        Probe::<happenstance_core::SequencePosition>(PhantomData).is_send(),
        "the positive control: a NonZeroU64 is Send, and a probe that cannot \
         say so is reporting nothing about the negative case below"
    );
    assert!(
        !Probe::<<CloudflareFixture as Fixture>::Store>(PhantomData).is_send(),
        "`Fixture::Store` is bound on the bare `EventStore`, the flavour with \
         no `Send` requirement, and this adapter is the only one in the \
         workspace that implements it. A `Send` store here means the fixture \
         path acquired a bound the whole two-flavour design exists to keep off \
         it — and on `wasm32` without `atomics` a real `JsValue` is `Send + \
         Sync`, so it is restorable by accident and without a diagnostic."
    );
}

/// AC-005 — every declined capability names why *this runtime* cannot.
#[test]
fn every_declined_capability_names_this_runtime() {
    let declared: [(&str, Capability); 3] = [
        (
            "SECOND_HANDLE",
            <CloudflareFixture as Fixture>::SECOND_HANDLE,
        ),
        ("REOPEN", <CloudflareFixture as Fixture>::REOPEN),
        (
            "MID_BATCH_FAULT",
            <CloudflareFixture as Fixture>::MID_BATCH_FAULT,
        ),
    ];

    for (name, capability) in declared {
        let Some(reason) = capability.reason() else {
            continue;
        };
        assert!(
            !reason.trim().is_empty(),
            "{name} is declined with an empty reason. `Capability::declined` \
             rejects that in a `const fn`, but for an **associated** const the \
             rejection arrives at codegen — so `cargo check` and `cargo \
             clippy` both pass and only a build or a run catches it."
        );
        let lowered = reason.to_ascii_lowercase();
        assert!(
            RUNTIME_VOCABULARY.iter().any(|word| lowered.contains(word)),
            "{name} is declined with `{reason}`, which says *that* this \
             fixture cannot rather than *why this runtime* cannot. The reason \
             is printed on every conformance run and is the only record of the \
             trade; a generic refusal reaches a gate reader saying nothing. \
             Expected one of {RUNTIME_VOCABULARY:?} to appear in it."
        );
    }
}

/// AC-006 — `MID_BATCH_FAULT` is answered in this impl, not inherited.
///
/// The trait defaults it, and the default was written for a store with no fault
/// to inject. A Durable Object is not that store, so inheriting the default
/// would put a sentence in this run's CI log that is not about this adapter.
#[test]
fn mid_batch_fault_is_restated_not_inherited() {
    let declared = <CloudflareFixture as Fixture>::MID_BATCH_FAULT;
    assert_ne!(
        declared.reason(),
        Some(INHERITED_MID_BATCH_FAULT_REASON),
        "`MID_BATCH_FAULT` is the trait's default verbatim, so this fixture \
         has not answered it — it has let a sentence written for an in-memory \
         store speak for a Durable Object. Restate it in the impl: supported \
         with a real armed seam, or declined in this runtime's own words."
    );
}

/// AC-007 — the three CF-40 ceilings are written out in this impl.
///
/// A ceiling is a fact rather than a trade, and the failure this rejects is
/// *inheritance by omission*: three constants left off the impl read exactly
/// like three constants deliberately answered `None`, and only the source can
/// tell them apart. Reading the source is therefore the assertion.
#[test]
fn the_three_store_limits_are_stated_here() {
    const SOURCE: &str = include_str!("support/mod.rs");

    for constant in [
        "MAX_EVENT_DATA_LEN",
        "MAX_TAGS_PER_EVENT",
        "MAX_EVENTS_PER_BATCH",
    ] {
        assert!(
            SOURCE.contains(&format!("const {constant}")),
            "`{constant}` is not written in `tests/support/mod.rs`, so this \
             fixture inherited it from the trait. An inherited ceiling and a \
             deliberate `None` are indistinguishable in a CI log, and the \
             owner of the value has nowhere to be named."
        );
    }
}

/// The `None` default is a statement, and for this store it is a false one.
///
/// Nothing in the conformance suite can catch this, and the reason is worth
/// stating rather than implying: the defect is in the **fixture's declaration**,
/// not in a store's behaviour, so no store can fail a rule written about it, and
/// `CLAUDE.md` forbids adding a conformance rule nothing can fail. Leaving all
/// three at `None` makes `append_reports_exceeded_store_limits` report
/// `Skipped { capability: NO_STORE_LIMITS }`, print an honest-looking line and
/// certify nothing — a green run in which the workspace's one capacity-capped
/// runtime contributes nothing at all to the clause it was brought in to
/// discharge, which is the outcome CF-40 names in terms.
///
/// So the guard is adapter-local: this test lives in this crate, ships in no
/// suite and gates no other adapter. If it is ever deleted as redundant, the
/// quiet failure mode is fully restored.
#[test]
fn the_three_store_limits_are_measured_not_defaulted() {
    for (name, declared) in [
        (
            "MAX_EVENT_DATA_LEN",
            <CloudflareFixture as Fixture>::MAX_EVENT_DATA_LEN,
        ),
        (
            "MAX_TAGS_PER_EVENT",
            <CloudflareFixture as Fixture>::MAX_TAGS_PER_EVENT,
        ),
        (
            "MAX_EVENTS_PER_BATCH",
            <CloudflareFixture as Fixture>::MAX_EVENTS_PER_BATCH,
        ),
    ] {
        assert!(
            declared.is_some(),
            "{name} is `None`, which says this store has no ceiling on that \
             value. It is not true of a store whose backing API carries \
             `SqlError::StorageLimitExceeded` and whose adapter refuses a batch \
             before issuing any SQL, and it makes \
             `append_reports_exceeded_store_limits` skip rather than run."
        );
    }
}

/// A ceiling below a floor is a conformance failure, never a declaration.
///
/// The trait explicitly permits a fixture to state a ceiling below one of the
/// guaranteed minima, and to then fail the corresponding rule — correctly,
/// because VT-21, VT-22 and VT-24 make those floors obligations every store
/// clears. So a measurement that comes in under a floor is not a smaller number
/// to declare; it is a schema or batching change in the adapter, or a blocking
/// finding. This test is what stops the first of those two readings.
#[test]
fn no_declared_ceiling_is_below_its_floor() {
    use happenstance_core::{
        MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_EVENTS_PER_BATCH, MIN_SUPPORTED_TAGS_PER_EVENT,
    };

    for (name, declared, floor) in [
        (
            "MAX_EVENT_DATA_LEN",
            <CloudflareFixture as Fixture>::MAX_EVENT_DATA_LEN,
            MIN_SUPPORTED_EVENT_DATA_LEN,
        ),
        (
            "MAX_TAGS_PER_EVENT",
            <CloudflareFixture as Fixture>::MAX_TAGS_PER_EVENT,
            MIN_SUPPORTED_TAGS_PER_EVENT,
        ),
        (
            "MAX_EVENTS_PER_BATCH",
            <CloudflareFixture as Fixture>::MAX_EVENTS_PER_BATCH,
            MIN_SUPPORTED_EVENTS_PER_BATCH,
        ),
    ] {
        let Some(ceiling) = declared else {
            continue;
        };
        assert!(
            ceiling >= floor,
            "{name} is {ceiling}, below the guaranteed minimum of {floor}. \
             Declaring the smaller number is not the fix: the store must clear \
             the floor, so the answer is a wider column, a chunked insert, or a \
             blocking finding."
        );
    }
}

/// AC-007 — the expression the slice-mate hands the shipped macro type-checks.
///
/// The macro's general arm hoists the fixture expression behind
/// `async fn __conformance_fixture() -> impl Fixture` and hands the opaque type
/// to every rule (`crates/happenstance-testkit/src/lib.rs:494-521`). This is
/// that obligation, written out: if `CloudflareFixture::new()` does not satisfy
/// it, the slice-mate's three-line target does not compile.
#[test]
fn the_fixture_expression_satisfies_the_macro_arm() {
    async fn conformance_fixture() -> impl Fixture {
        CloudflareFixture::new()
    }

    fn accepts_the_macros_bound<F: Fixture>(_: impl AsyncFn() -> F) {}

    accepts_the_macros_bound(conformance_fixture);

    assert!(
        <CloudflareFixture as Fixture>::SECOND_HANDLE.is_supported(),
        "CF-16 is a MUST and `two_handles_observe_each_others_appends` uses \
         `must!` — a declined `SECOND_HANDLE` fails the rule quoting this \
         fixture's own words rather than skipping it, which is the escape \
         route the clause was rewritten to close."
    );
}

/// The half that needs a real Durable Object under it.
#[cfg(target_arch = "wasm32")]
mod on_the_object {
    use core::future::poll_fn;

    use futures_core::Stream;
    use happenstance_cloudflare::host::{self, DurableObjectHost};
    use happenstance_core::{Event, EventStore, Query, ReadOptions};
    use happenstance_testkit::Fixture;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::CloudflareFixture;

    fn event(event_type: &str) -> Event {
        Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
    }

    /// Drains a stream through `poll_next` and nothing else.
    ///
    /// Hand-rolled for the reason `src/event_store.rs`'s own tests give: this
    /// crate carries no `futures-util`, and reaching the next item through
    /// `poll_next` is what a caller of the bare, `!Send` flavour actually does.
    async fn drain<S: Stream>(stream: S) -> Vec<S::Item> {
        let mut stream = Box::pin(stream);
        let mut out = Vec::new();
        while let Some(item) = poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
            out.push(item);
        }
        out
    }

    /// Every event type the store holds, in position order.
    async fn types_in<S: EventStore>(store: &S) -> Vec<String> {
        drain(store.read(&Query::all(), ReadOptions::new()))
            .await
            .into_iter()
            .map(|item| {
                item.expect("every item of the read decodes")
                    .event
                    .event_type()
                    .as_str()
                    .to_owned()
            })
            .collect()
    }

    /// AC-001 — the host exists, is reachable from a second compilation unit,
    /// and answers.
    ///
    /// Compiling at all is half the proof and it is the half that catches
    /// EC-005: a `#[cfg(test)]` module in `src/lib.rs` cannot be named from
    /// here. The other half is that the host is a *host* — one append driven
    /// through `CloudflareEventStore::new(sql)`, the one construction seam,
    /// against storage taken off the object's own state.
    #[wasm_bindgen_test]
    async fn the_host_is_reachable_from_an_integration_test() {
        let host = DurableObjectHost::new();
        let store = happenstance_cloudflare::CloudflareEventStore::new(host.storage());
        store.migrate().expect("the schema applies");

        store
            .append(&[event("HostedByTheObject")], None)
            .await
            .expect("the append lands");

        assert_eq!(
            types_in(&store).await,
            ["HostedByTheObject"],
            "the host has to answer, not merely link: an entrypoint that \
             compiles and stores nothing is the position this project started \
             in"
        );
    }

    /// AC-003 — two fixture instances share nothing.
    ///
    /// Both instances are constructed **before** either appends, which is what
    /// distinguishes isolation from clearing: a fixture that reuses one object
    /// and wipes it between instances passes every sequential arrangement and
    /// fails only this one.
    #[wasm_bindgen_test]
    async fn two_instances_alive_at_once_observe_none_of_each_others_appends() {
        let first = CloudflareFixture::new();
        let second = CloudflareFixture::new();

        let one = first.connect().await;
        let other = second.connect().await;

        one.append(&[event("OnlyInTheFirst")], None)
            .await
            .expect("the first instance accepts its append");
        other
            .append(&[event("OnlyInTheSecond")], None)
            .await
            .expect("the second instance accepts its append");

        assert_eq!(
            types_in(&one).await,
            ["OnlyInTheFirst"],
            "a fresh fixture instance is a fresh Durable Object. Pointing every \
             instance at one object's storage is the adapter mistake no test \
             the testkit writes about its own fixture could ever observe."
        );
        assert_eq!(
            types_in(&other).await,
            ["OnlyInTheSecond"],
            "and the sharing has to be invisible in both directions"
        );
    }

    /// AC-004 — `connect()` is a second handle onto the same object.
    #[wasm_bindgen_test]
    async fn two_handles_from_one_instance_observe_each_others_appends() {
        let fixture = CloudflareFixture::new();
        let one = fixture.connect().await;
        let other = fixture.connect().await;

        one.append(&[event("ThroughTheFirstHandle")], None)
            .await
            .expect("the first handle accepts its append");

        assert_eq!(
            types_in(&other).await,
            ["ThroughTheFirstHandle"],
            "CF-16: `connect()` must hand back one more handle onto *that* \
             object — the aliasing `SqlStorage: Clone` already models — not a \
             second object wearing the same name"
        );
    }

    /// AC-004 — the schema seam runs once per instance, never per handle.
    ///
    /// Read off the host's own statement log, because the difference between
    /// one migration per object and one per connection is invisible in
    /// everything else: `CREATE TABLE IF NOT EXISTS` is idempotent, so a
    /// per-handle migration is correct and still wrong. It would also mask a
    /// fixture whose two "handles" are actually two objects, because each would
    /// have been given a schema on the way out.
    #[wasm_bindgen_test]
    async fn migrate_runs_once_per_instance_not_per_connect() {
        let fixture = CloudflareFixture::new();
        let _first = fixture.connect().await;
        let _second = fixture.connect().await;

        let migrations = fixture
            .issued_statements()
            .into_iter()
            .filter(|statement| statement.contains("CREATE TABLE IF NOT EXISTS event ("))
            .count();

        assert_eq!(
            migrations,
            1,
            "`migrate()` is the schema seam and belongs to the *instance*. \
             Calling it from `connect()` makes schema creation a per-connection \
             effect. Statements issued: {:?}",
            fixture.issued_statements()
        );
    }

    /// AC-004 — a second handle does not mint a second incarnation.
    ///
    /// ADR-0014 governs store-id incarnation: mint once, re-mint only on a
    /// detectable restore or clone. A fixture that re-minted per handle would
    /// change what `contains_event_id` answers about the store's *own* past,
    /// which is the one question the identity columns exist to answer.
    #[wasm_bindgen_test]
    async fn a_second_handle_does_not_re_mint_the_store_id() {
        let fixture = CloudflareFixture::new();
        let one = fixture.connect().await;

        one.append(&[event("MineNotYours")], None)
            .await
            .expect("the first handle accepts its append");
        let ids: Vec<_> = drain(one.read(&Query::all(), ReadOptions::new()))
            .await
            .into_iter()
            .map(|item| item.expect("the item decodes").id)
            .collect();
        let id = *ids.first().expect("the append produced one event");

        let other = fixture.connect().await;
        assert!(
            other
                .contains_event_id(id)
                .await
                .expect("the probe runs on the second handle"),
            "a second handle onto one object is the same store, so it must \
             still recognise the object's own events. A fixture that re-minted \
             the incarnation per handle answers `false` here while passing \
             every other rule."
        );
    }

    /// AC-006 — nothing declared supported is claimed without its method.
    ///
    /// The trait ties no `SUPPORTED` constant to its override and the provided
    /// bodies **panic**, so the reachable author path is *declare it supported,
    /// forget the override, run the suite*. Reading the constant is not enough:
    /// the panic has to be reached, or the whole failure mode is untested.
    #[wasm_bindgen_test]
    async fn a_supported_capability_has_its_method_overridden() {
        let fixture = CloudflareFixture::new();

        if <CloudflareFixture as Fixture>::REOPEN.is_supported() {
            let store = fixture.connect().await;
            store
                .append(&[event("BeforeTheReopen")], None)
                .await
                .expect("the append lands");

            // The provided body panics; reaching this line at all is the
            // assertion. What follows is the second half: an override that
            // discards handle state without discarding what was committed.
            fixture.reopen().await;

            let reopened = fixture.connect().await;
            assert_eq!(
                types_in(&reopened).await,
                ["BeforeTheReopen"],
                "`REOPEN` is declared supported, so `reopen` must discard \
                 handle state and leave what was durably committed — not \
                 discard the events with it, and not do nothing"
            );
        }

        if <CloudflareFixture as Fixture>::MID_BATCH_FAULT.is_supported() {
            let store = fixture.connect().await;
            fixture.arm_mid_batch_fault(2).await;
            let batch = [event("Armed1"), event("Armed2"), event("Armed3")];
            assert!(
                store.append(&batch, None).await.is_err(),
                "CF-39: `MID_BATCH_FAULT` is declared supported, so an armed \
                 fault must make the write of the third event fail inside the \
                 store's own write path by a mechanism the store cannot \
                 absorb. An override with an empty body reaches this line and \
                 reports a green atomicity result for a store nothing faulted."
            );
        }
    }

    /// The host's arming hook is a real seam, not a decoration.
    ///
    /// Not an acceptance criterion of its own — it is the control that makes
    /// `a_supported_capability_has_its_method_overridden`'s
    /// `MID_BATCH_FAULT` branch mean something, by showing the same host under
    /// the same statement failing exactly once and then working.
    #[wasm_bindgen_test]
    fn the_hosts_arming_hook_fires_once_and_disarms() {
        let host = DurableObjectHost::new();
        let sql = host.storage();
        sql.exec("CREATE TABLE IF NOT EXISTS probe (v INTEGER)", &[])
            .expect("the schema applies");

        host::arm_throw_after(&sql, "INSERT INTO probe", "armed", 1);

        assert!(
            sql.exec("INSERT INTO probe (v) VALUES (1)", &[]).is_ok(),
            "the skip count is honoured: the first matching statement runs"
        );
        assert!(
            sql.exec("INSERT INTO probe (v) VALUES (2)", &[]).is_err(),
            "and the next one throws"
        );
        assert!(
            sql.exec("INSERT INTO probe (v) VALUES (3)", &[]).is_ok(),
            "and the arming disarms as it fires, so one arming is one throw"
        );
    }
}
