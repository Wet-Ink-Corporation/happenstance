//! The command loop: read, decide, append, retry on contention.
//!
//! One private policy and two public doors. [`commit_with`] is the whole loop;
//! [`commit`] is the JSON convenience that costs a line.

use core::num::NonZeroU32;

use happenstance_core::bytes::Bytes;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventStore, EventType, InvalidQuery,
    SequencePosition, read_decision_model,
};

use crate::boundary::Boundary;
use crate::codec::{Codec, CodecError};
use crate::domain::DomainEvent;

/// How many times a command may be attempted before it gives up.
///
/// A required argument, because a loop whose only exit is success is a hang
/// with better manners, and a caller who cannot see the bound cannot budget
/// for it. A hidden three lost to it: it would have made every command's worst
/// case a number nobody wrote down.
///
/// `NonZeroU32` rather than `u32`, because `attempts(0)` has no honest meaning.
///
/// ```
/// use core::num::NonZeroU32;
/// use happenstance::Retry;
///
/// // At most three attempts, or exactly one.
/// let bounded = Retry::attempts(NonZeroU32::new(3).unwrap());
/// let no_retry = Retry::once();
/// # let _ = (bounded, no_retry);
/// ```
///
/// The unbounded spelling does not compile, which is the point:
///
/// ```compile_fail
/// // compile_fail: retry_takes_no_zero
/// let _ = happenstance::Retry::attempts(0);
/// ```
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Retry {
    attempts: NonZeroU32,
}

impl Retry {
    /// At most `n` attempts, the first one included.
    #[must_use]
    pub const fn attempts(n: NonZeroU32) -> Self {
        Self { attempts: n }
    }

    /// Exactly one attempt: submit once, and report contention as it is.
    #[must_use]
    pub const fn once() -> Self {
        Self {
            attempts: NonZeroU32::MIN,
        }
    }

    /// The bound this value carries. Crate-internal: the observable part is
    /// [`Committed::attempts`], and one reading of a bound is enough.
    pub(crate) const fn limit(self) -> u32 {
        self.attempts.get()
    }
}

/// What a committed command did: where it landed, and how hard it was.
///
/// A named struct rather than a tuple, because `attempts` is exactly the field
/// a later observability pass wants to grow and a tuple freezes the arity.
/// Readable, not fabricable: the fields are public and `#[non_exhaustive]`
/// blocks the struct literal, so a fourth field is not a breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "a Committed says where the events landed and how many attempts it took"]
#[non_exhaustive]
pub struct Committed {
    /// The position the store assigned the **last** appended event.
    pub position: SequencePosition,
    /// How many appends were submitted, including the one that succeeded.
    ///
    /// `1` on an uncontended commit. This is the observable proof that the
    /// retry ran, and the only part of the loop's per-attempt state that is
    /// public.
    pub attempts: u32,
}

/// Why a command did not commit.
///
/// The vocabulary is the contract's, extended by two type parameters and never
/// duplicated: `E` is the store's own error and `D` is the caller's own
/// refusal, both carried as typed sources. There is no `happenstance`-local
/// re-spelling of `ConditionViolated`, and no payload is a string.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CommandError<E: core::error::Error + 'static, D: core::error::Error + 'static> {
    /// The boundary constrains neither event types nor tags.
    ///
    /// Absorbed with `#[from]`, so a first program writes no extra `?` for it.
    #[error(transparent)]
    Boundary(#[from] InvalidQuery),

    /// The store failed while the decision model was being read.
    ///
    /// Not retried: a read failure is not the DCB concurrency signal.
    #[error("reading the decision model failed")]
    Read(#[source] E),

    /// The store failed the append for its own reasons.
    ///
    /// Only a violated condition routes to a retry; everything else is this.
    /// A message plus `#[source]` rather than `#[error(transparent)]`, because
    /// transparent forwards the *inner* error's source and would drop the
    /// `AppendError` itself out of the chain a caller reports from.
    #[error("appending the decided events failed")]
    Append(#[source] AppendError<E>),

    /// An event the read nominated could not be decoded.
    ///
    /// The position names *which* event, because a disagreement between a
    /// domain type's `EVENT_TYPES` and the fold that reads them is otherwise
    /// invisible.
    #[error("decoding the event at position {position} failed")]
    Decode {
        /// Where the offending event sits in the store's order.
        position: SequencePosition,
        /// The codec's own refusal.
        #[source]
        source: CodecError,
    },

    /// An event the decision produced could not be encoded.
    ///
    /// **Nothing was appended.** Encoding happens before the single
    /// irreversible act.
    #[error("encoding a `{event_type}` payload failed")]
    Encode {
        /// The event type whose payload could not be written.
        event_type: EventType,
        /// The codec's own refusal.
        #[source]
        source: CodecError,
    },

    /// The decision refused, and nothing was appended.
    ///
    /// Not an error of this library: the caller's own type, carried faithfully.
    #[error("the decision refused")]
    Refused(#[source] D),

    /// Every attempt was contended.
    ///
    /// A distinct outcome from a store failure and from a refusal. The last
    /// violation travels along, hint and all, so nothing is swallowed.
    #[error("the boundary was contended on all {attempts} attempts")]
    Exhausted {
        /// The bound that was reached.
        attempts: u32,
        /// The last violation, carried for reporting.
        #[source]
        source: ConditionViolated,
    },
}

/// Reads, decides, appends under a condition, and retries on contention.
///
/// JSON payloads. For any other encoding use [`commit_with`], which is this
/// function with the codec spelled out: there is one implementation of the
/// policy below and both doors go through it.
///
/// Bound on [`EventStore`], the flavour that does **not** require `Send`, so a
/// store held through an `Rc` on a single-threaded edge runtime is accepted
/// here. It is the weaker requirement of the two, and the blanket impl means a
/// `Send` store satisfies it as well — binding the other way round would have
/// locked out `wasm32` for no gain.
///
/// # The retry policy
///
/// On a violated condition the loop goes back to step one. It re-derives the
/// query, re-reads the store, folds a **fresh clone** of the boundary you
/// handed it, and calls your closure again with that new state. It never
/// re-uses a stale fold, never re-submits the previous attempt's events, and
/// never mutates your value. The bound is `retry`, it is a required argument,
/// and running out is [`CommandError::Exhausted`] rather than a hang.
///
/// **This is not the retry-safety of a verbatim resubmission, and collapsing
/// the two is a lost update.** Re-sending the *same* events under the *same*
/// condition asks "did my write land?"; this loop asks "what should I decide
/// now?", and the answer is allowed to differ. A loop that replayed the first
/// attempt's events against a store that had moved would overwrite a decision
/// somebody else had already committed.
///
/// Every attempt's condition is anchored on **that attempt's own read**, never
/// on what a previous `append` returned: positions may have gaps, and another
/// writer may hold one below a returned value that this caller never saw.
///
/// The alternative that lost: branching the retry on
/// `ConditionViolated::conflicting_position` being `Some`. It works against an
/// in-process store and never retries against one reached over one-shot HTTP,
/// which reports `None` conformantly. The hint is carried into the returned
/// error and never consulted.
///
/// # Errors
///
/// * [`CommandError::Boundary`] if the boundary constrains nothing.
/// * [`CommandError::Read`] if the store fails during the read.
/// * [`CommandError::Decode`] if a nominated event cannot be decoded.
/// * [`CommandError::Refused`] if your closure returns `Err`. Nothing is
///   appended.
/// * [`CommandError::Encode`] if a decided event cannot be encoded. Nothing is
///   appended.
/// * [`CommandError::Append`] if the store fails the append for its own
///   reasons.
/// * [`CommandError::Exhausted`] if every attempt was contended.
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub async fn commit<S, B, D, F>(
    store: &S,
    boundary: B,
    retry: Retry,
    decide: F,
) -> Result<Committed, CommandError<S::Error, D>>
where
    S: EventStore,
    B: Boundary + Clone,
    D: core::error::Error + 'static,
    F: FnMut(&B) -> Result<Vec<B::Event>, D>,
{
    commit_with(store, boundary, &crate::codec::Json, retry, decide).await
}

/// The command loop, with a codec of your own.
///
/// The ungated door, and the whole policy: `commit` is this function with
/// `Json` already chosen, and exists beside it so that a first program names a
/// domain before it names an encoding. One entry point always taking `&C` lost
/// for exactly that reason.
///
/// A codec of your own is welcome here and carries one limit worth reading
/// before you commit a log to it: [`Codec`]'s *Reading a tag this build did
/// not write*. In short, the events it tags are readable by a build holding
/// that codec and by nothing else — which makes it safe as a log's only
/// codec, and not as one of several.
///
/// Bound on [`EventStore`], the flavour that does **not** require `Send`, so an
/// `Rc`-backed store on a single-threaded edge runtime is accepted — and a
/// `Send` store is too, through the blanket impl. Spawning this loop onto a
/// multi-threaded runtime works without any further bound: the store error is
/// collapsed to a control decision before the next read's await, so nothing
/// unbounded is alive across a suspension point.
///
/// On a violated condition it re-derives the query, re-reads the store, folds
/// a **fresh clone** of the boundary you handed it and calls your closure
/// again — never re-using a stale fold, never re-submitting the previous
/// attempt's events, and never mutating your value. The bound is `retry`, and
/// running out is [`CommandError::Exhausted`] rather than a hang. **That is not
/// the retry-safety of a verbatim resubmission, and collapsing the two is a
/// lost update.** Every attempt's condition is anchored on that attempt's own
/// read.
///
/// The links to this crate's other pages are deliberately spelled without
/// backtick links where the target is feature-gated: an intra-doc link that
/// resolves in only some configurations is a hard rustdoc error.
///
/// # Errors
///
/// The full set is on [`CommandError`], and every variant is actionable at
/// this call site.
pub async fn commit_with<S, B, C, D, F>(
    store: &S,
    boundary: B,
    codec: &C,
    retry: Retry,
    mut decide: F,
) -> Result<Committed, CommandError<S::Error, D>>
where
    S: EventStore,
    B: Boundary + Clone,
    C: Codec,
    D: core::error::Error + 'static,
    F: FnMut(&B) -> Result<Vec<B::Event>, D>,
{
    let mut attempts = 0_u32;

    loop {
        attempts = attempts.saturating_add(1);

        // A fresh clone every attempt. `DecisionModel: Clone` exists for this:
        // re-using the previous attempt's value with a "reset" is how a stale
        // field survives a retry. `retry_refolds_from_pristine_state` is what
        // rejects hoisting this line out of the loop — and it only does so
        // because its store is seeded, so the fold has a state to be stale.
        let mut model = boundary.clone();
        let query = model.query()?;

        let (events, anchor) = read_decision_model(store, &query)
            .await
            .map_err(CommandError::Read)?;

        for event in &events {
            model
                .absorb(event, codec)
                .map_err(|source| CommandError::Decode {
                    position: event.position,
                    source,
                })?;
        }

        let decided = decide(&model).map_err(CommandError::Refused)?;
        let batch = encode::<B::Event, C, S::Error, D>(&decided, codec)?;

        // From the read, and only from the read.
        let condition = AppendCondition::new(query).after_opt(anchor);

        match store.append(&batch, Some(&condition)).await {
            Ok(position) => return Ok(Committed { position, attempts }),
            Err(err) => {
                // The error is collapsed to a control decision here, and
                // dropped before the next read's await. `S::Error` carries no
                // `Send` bound (ADR-0009), so binding it across that suspension
                // point would make the whole future `!Send` — and every
                // single-threaded test would still pass. `tests/flavours.rs` is
                // what rejects that shape.
                if !err.is_condition_violated() {
                    return Err(CommandError::Append(err));
                }
                if attempts >= retry.limit() {
                    return Err(CommandError::Exhausted {
                        attempts,
                        source: violation(err),
                    });
                }
            }
        }
    }
}

/// Encodes one decision's events, tagging each with the codec that wrote it.
fn encode<E, C, S, D>(decided: &[E], codec: &C) -> Result<Vec<Event>, CommandError<S, D>>
where
    E: DomainEvent,
    C: Codec,
    S: core::error::Error + 'static,
    D: core::error::Error + 'static,
{
    let mut batch = Vec::with_capacity(decided.len());
    for event in decided {
        let event_type = event.event_type();
        let data: Bytes = event.encode(codec).map_err(|source| CommandError::Encode {
            event_type: event_type.clone(),
            source,
        })?;

        // Unreachable for a well-formed `DomainEvent`: `event_type()` hands
        // back an already-validated `EventType` and the conversion is the
        // infallible identity. It is absorbed rather than `unwrap`ed, for the
        // reason `Boundary::query` keeps its own unreachable `Err` arm.
        let built = Event::new(event_type, data).map_err(InvalidQuery::from)?;

        batch.push(
            built
                .with_tags(event.tags())
                .with_metadata(crate::codec::frame::<C>(None)),
        );
    }
    Ok(batch)
}

/// The violation inside an error the predicate has already answered for.
fn violation<E>(err: AppendError<E>) -> ConditionViolated {
    match err {
        AppendError::ConditionViolated(violation) => violation,
        // Unreachable: every caller asks the predicate first. Spelled as the
        // violation that names no conflict rather than as an `unwrap`, because
        // a contention signal must not become a process failure inside
        // somebody else's request handler.
        _ => ConditionViolated::unspecified(),
    }
}

#[cfg(test)]
mod tests {
    use core::convert::Infallible;
    use core::num::NonZeroU32;

    use happenstance_core::bytes::Bytes;
    use happenstance_core::{EventType, MemoryEventStore, SequencePosition, Tags};

    use super::{Retry, commit_with};
    use crate::{Codec, CodecError, DecisionModel, DomainEvent};

    // -----------------------------------------------------------------------
    // The smallest domain a command can be run against
    // -----------------------------------------------------------------------

    /// One declared type, one variant, one fold arm.
    ///
    /// Deliberately neither `src/tests.rs`'s `Ticket` nor the doctests' `Seat`:
    /// a change to one fixture must not quietly repair another.
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    enum Turnstile {
        Passed,
    }

    const PASSED: EventType = EventType::from_static("TurnstilePassed");

    impl DomainEvent for Turnstile {
        const EVENT_TYPES: &'static [EventType] = &[PASSED];

        fn event_type(&self) -> EventType {
            PASSED
        }

        fn tags(&self) -> Tags {
            gate_tags()
        }

        fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
            codec.encode(self)
        }

        fn decode<C: Codec>(
            codec: &C,
            _event_type: &EventType,
            data: &Bytes,
        ) -> Result<Self, CodecError> {
            codec.decode(data)
        }
    }

    #[derive(Debug, Clone)]
    struct Gate {
        scope: Tags,
        passed: u32,
    }

    impl DecisionModel for Gate {
        type Event = Turnstile;

        fn scope(&self) -> &Tags {
            &self.scope
        }

        // No `_ =>` arm: a second variant is a compile error here.
        fn apply(&mut self, event: Self::Event) {
            match event {
                Turnstile::Passed => self.passed += 1,
            }
        }
    }

    fn gate_tags() -> Tags {
        Tags::from_pairs([("gate", "north")]).expect("a valid tag pair")
    }

    /// A real codec, so the loop's encode step encodes something.
    ///
    /// Tagged `wire` rather than `json` so this module needs no feature the
    /// contract's in-memory store does not already need.
    struct Wire;

    impl Codec for Wire {
        const TAG: &'static str = "wire";

        fn encode<T: serde::Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
            serde_json::to_vec(value)
                .map(Bytes::from)
                .map_err(|err| CodecError::Encode(Box::new(err)))
        }

        fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
            serde_json::from_slice(data).map_err(|err| CodecError::Decode(Box::new(err)))
        }
    }

    /// An uncontended commit reports `1`, and reports it from the loop.
    ///
    /// Driven through `commit_with` rather than asserted off a `Committed`
    /// literal, which would only prove that field assignment works. This
    /// rejects a counter read before its increment (`0`), one seeded at the
    /// retry bound (`Retry::once()` here, so a bound-shaped answer is still
    /// `1` — hence the second assertion), and a `position` that is anything
    /// other than what the store assigned.
    #[tokio::test]
    async fn first_try_reports_one_attempt() {
        let store = MemoryEventStore::new();
        let gate = Gate {
            scope: gate_tags(),
            passed: 0,
        };

        let done = commit_with(&store, gate, &Wire, Retry::once(), |gate: &Gate| {
            assert_eq!(
                gate.passed, 0,
                "an empty store folded into a non-zero state"
            );
            Ok::<_, Infallible>(vec![Turnstile::Passed])
        })
        .await
        .expect("an uncontended commit");

        assert_eq!(done.attempts, 1, "an uncontended commit is one attempt");
        // Compared against the position the store actually assigned, never a
        // literal: the specification permits gaps.
        let held = store.snapshot();
        let first = held.first().expect("one event landed");
        assert_eq!(first.position, done.position);
    }

    // An inherent method shadows a trait method when the bound holds, and
    // falls through to it when it does not — a compile-time decision read as a
    // bool, which is the only way to assert the *absence* of an impl.
    mod default_probe {
        use core::marker::PhantomData;

        pub(super) struct Probe<T>(pub(super) PhantomData<T>);

        pub(super) trait NotDefault {
            fn has_default(&self) -> bool {
                false
            }
        }

        impl<T> NotDefault for Probe<T> {}

        impl<T: Default> Probe<T> {
            #[allow(clippy::unused_self)]
            pub(super) fn has_default(&self) -> bool {
                true
            }
        }
    }

    #[test]
    fn retry_has_no_default() {
        use core::marker::PhantomData;
        use default_probe::{NotDefault as _, Probe};

        // Positive control, so this cannot pass on a broken probe.
        assert!(
            Probe::<u32>(PhantomData).has_default(),
            "probe is broken: u32 has a Default"
        );
        assert!(
            !Probe::<Retry>(PhantomData).has_default(),
            "`Retry` has a default, so the bound is a hidden number again"
        );
    }

    #[test]
    fn once_is_one_attempt_and_attempts_is_what_it_says() {
        assert_eq!(Retry::once().limit(), 1);
        assert_eq!(
            Retry::attempts(NonZeroU32::new(7).expect("7 is not zero")).limit(),
            7
        );
    }

    /// No call site converts a literal at run time when const would do.
    ///
    /// `Retry::attempts` is `const fn` (above), so nothing that hands it a
    /// literal needs `.try_into()?` at run time — that spelling converts a
    /// value the compiler already knows is nonzero, through a path that can
    /// fail, for a failure that cannot happen. This walks the source of
    /// every call site this crate owns (its own two doctests, and the two
    /// worked examples) and fails if any of them still reach for the
    /// fallible spelling where the const one is available. Needles are built
    /// with `format!` rather than written as one literal, so this test's own
    /// source text never contains the pattern it is searching for.
    #[test]
    fn call_sites_use_the_const_spelling_of_retry_attempts() {
        let doctest_literal = format!("Retry::attempts({}.try_into()?)", 3);
        let example_constant = format!("Retry::attempts({}.try_into()?)", "ATTEMPTS");
        let test_literal = format!("Retry::attempts({}.try_into()", 7);

        let this_command_rs = include_str!("command.rs");
        let this_lib_rs = include_str!("lib.rs");
        let course_subscriptions =
            include_str!("../../../examples/course-subscriptions/src/main.rs");
        let transfers_on_sqlite = include_str!("../../../examples/transfers-on-sqlite/src/main.rs");

        let sources: [(&str, &str, &[&str]); 4] = [
            (
                "crates/happenstance/src/command.rs",
                this_command_rs,
                &[&doctest_literal, &test_literal],
            ),
            (
                "crates/happenstance/src/lib.rs",
                this_lib_rs,
                &[&doctest_literal],
            ),
            (
                "examples/course-subscriptions/src/main.rs",
                course_subscriptions,
                &[&example_constant],
            ),
            (
                "examples/transfers-on-sqlite/src/main.rs",
                transfers_on_sqlite,
                &[&example_constant],
            ),
        ];

        let mut offenders = Vec::new();
        for (path, src, needles) in sources {
            for needle in needles {
                if src.contains(needle) {
                    offenders.push(format!("{path}: still contains {needle:?}"));
                }
            }
        }

        assert!(
            offenders.is_empty(),
            "call site converts a compile-time literal at run time against a \
             const fn; use the const spelling instead:\n{}",
            offenders.join("\n")
        );
    }

    /// A store error, so the chain below ends somewhere concrete.
    #[derive(Debug, PartialEq, Eq)]
    struct Disk;

    impl core::fmt::Display for Disk {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("the disk is full")
        }
    }

    impl core::error::Error for Disk {}

    #[test]
    fn store_read_failure_chains_to_source() {
        let err: super::CommandError<Disk, Disk> = super::CommandError::Read(Disk);

        let source = core::error::Error::source(&err).expect("a typed source");
        assert!(
            source.downcast_ref::<Disk>().is_some(),
            "the store's own error did not survive: {source}"
        );
        assert!(err.to_string().contains("reading the decision model"));
    }

    #[test]
    fn decode_failure_names_its_position() {
        let position = SequencePosition::new(12).expect("12 is not zero");
        let err: super::CommandError<Disk, Disk> = super::CommandError::Decode {
            position,
            source: CodecError::UnknownTag {
                tag: "protobuf".into(),
            },
        };

        assert!(
            err.to_string().contains("12"),
            "the message does not name which event: {err}"
        );
        let source = core::error::Error::source(&err).expect("a typed source");
        assert!(source.downcast_ref::<CodecError>().is_some());
    }

    #[test]
    fn encode_failure_names_its_event_type() {
        let err: super::CommandError<Disk, Disk> = super::CommandError::Encode {
            event_type: EventType::from_static("SeatTaken"),
            source: CodecError::UnknownTag {
                tag: "protobuf".into(),
            },
        };

        assert!(
            err.to_string().contains("SeatTaken"),
            "the message does not name the event type: {err}"
        );
        let source = core::error::Error::source(&err).expect("a typed source");
        assert!(source.downcast_ref::<CodecError>().is_some());
    }
}
