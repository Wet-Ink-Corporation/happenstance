//! Composing consistency boundaries: several models, one query.
//!
//! A caller writes a tuple. There is no `compose!` for them to invoke and no
//! builder to learn — an exported macro lost because a macro in the caller's
//! face is exactly the ceremony this design is measured on. The macro below is
//! internal and emits the two impls each arity needs.
//!
//! The ceiling is **8**. rustdoc renders one impl block per arity, and sixteen
//! of them buries [`Boundary`](crate::Boundary)'s page; above eight, compose
//! tuples of tuples.

/// Emits the `Sealed` and `Boundary` impls for one tuple arity.
///
/// One arm, so RS-41-3's most-literal-first ordering is satisfied by
/// construction: there is no second arm for a fragment to shadow. Every item
/// path the expansion emits is `$crate::`- or `::`-rooted, because hygiene
/// covers locals and labels and not item paths — the expansion resolves
/// wherever it lands, and `$crate` is the only spelling that survives someone
/// moving these impls into a submodule.
///
/// Members are named twice — `B1 b1` — because tuple field access (`self.0`)
/// cannot be generated from a type-parameter repetition, and destructuring
/// needs a value identifier per member.
macro_rules! impl_boundary_for_tuple {
    ($(#[$page:meta])* $first:ident $fv:ident $(, $rest:ident $rv:ident)+) => {
        impl<$first, $($rest),+> $crate::sealed::Sealed for ($first, $($rest),+)
        where
            $first: $crate::Boundary,
            $($rest: $crate::Boundary<
                Event = <$first as $crate::Boundary>::Event,
            >,)+
        {
        }

        $(#[$page])*
        impl<$first, $($rest),+> $crate::Boundary for ($first, $($rest),+)
        where
            $first: $crate::Boundary,
            $($rest: $crate::Boundary<
                Event = <$first as $crate::Boundary>::Event,
            >,)+
        {
            type Event = <$first as $crate::Boundary>::Event;

            fn query(
                &self,
            ) -> ::core::result::Result<$crate::Query, $crate::InvalidQuery> {
                let ($fv, $($rv),+) = self;

                // Every member's query is derived before any of them is read,
                // so the *first* failing member in member order is the one
                // whose error the caller gets. A member is never dropped so
                // the rest can carry on.
                let parts = [$fv.query()?, $($rv.query()?),+];

                let mut items = ::std::vec::Vec::new();
                for part in &parts {
                    match part.items() {
                        ::core::option::Option::Some(held) => {
                            items.extend_from_slice(held);
                        }
                        // A member that matches everything makes the union
                        // match everything. Widening, never narrowing.
                        ::core::option::Option::None => {
                            return ::core::result::Result::Ok(
                                $crate::Query::all(),
                            );
                        }
                    }
                }

                $crate::Query::from_items(items)
            }

            fn absorb<CODEC: $crate::Codec>(
                &mut self,
                event: &$crate::SequencedEvent,
                codec: &CODEC,
            ) -> ::core::result::Result<(), $crate::CodecError> {
                let ($fv, $($rv),+) = self;

                // Each member is asked, and each member consults its *own*
                // query. Handing every arriving event to every member would
                // compile, pass any test over disjoint tags, and corrupt the
                // fold the moment two models share an event type.
                $fv.absorb(event, codec)?;
                $($rv.absorb(event, codec)?;)+

                ::core::result::Result::Ok(())
            }
        }
    };
}

impl_boundary_for_tuple! {
    /// Two decision models checked by one append.
    ///
    /// The composed query is the **union** of the members' own queries, so the
    /// composite selects every event either member would have selected alone.
    /// Absorption goes the other way: each member folds only what its own
    /// query nominated, so a sibling's nomination can never reach it.
    ///
    /// An exported `compose!` macro lost to the tuple: a caller who writes a
    /// macro invocation where a tuple would do has been charged the ceremony
    /// this design is being measured on. The arity ceiling is 8, and that is a
    /// *rendering* decision — rustdoc emits one impl block per arity.
    ///
    /// A member that cannot decode an event it nominated stops the composite
    /// with that member's [`CodecError`](crate::CodecError). **Earlier members
    /// have already folded and are not rolled back**: the composite is a value,
    /// not a transaction, and the recovery is to discard it and re-fold from
    /// the pristine clone the command loop keeps.
    ///
    /// ```
    /// use happenstance::{Boundary, Codec, CodecError, DecisionModel};
    /// use happenstance::{DomainEvent, EventType, Tags, bytes::Bytes};
    ///
    /// #[derive(serde::Serialize, serde::Deserialize)]
    /// enum Seat { Taken, Freed }
    ///
    /// impl DomainEvent for Seat {
    ///     const EVENT_TYPES: &'static [EventType] = &[
    ///         EventType::from_static("SeatTaken"),
    ///         EventType::from_static("SeatFreed"),
    ///     ];
    ///     fn event_type(&self) -> EventType {
    ///         match self {
    ///             Self::Taken => Self::EVENT_TYPES[0].clone(),
    ///             Self::Freed => Self::EVENT_TYPES[1].clone(),
    ///         }
    ///     }
    ///     fn tags(&self) -> Tags { Tags::empty() }
    ///     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
    ///         c.encode(self)
    ///     }
    ///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
    ///         -> Result<Self, CodecError> { c.decode(d) }
    /// }
    ///
    /// #[derive(Clone)]
    /// struct Counter { scope: Tags, seen: u32 }
    ///
    /// impl DecisionModel for Counter {
    ///     type Event = Seat;
    ///     fn scope(&self) -> &Tags { &self.scope }
    ///     fn apply(&mut self, _event: Seat) { self.seen += 1; }
    /// }
    ///
    /// let of = |k, v| Tags::from_pairs([(k, v)]);
    /// let capacity = Counter { scope: of("course", "c1")?, seen: 0 };
    /// let student = Counter { scope: of("student", "s1")?, seen: 0 };
    ///
    /// // The tuple is the boundary. No macro, no builder, no new import.
    /// let both = (capacity, student);
    /// assert_eq!(both.query()?.items().map_or(0, <[_]>::len), 2);
    /// # Ok::<(), Box<dyn core::error::Error>>(())
    /// ```
    B1 b1, B2 b2
}

impl_boundary_for_tuple!(B1 b1, B2 b2, B3 b3);
impl_boundary_for_tuple!(B1 b1, B2 b2, B3 b3, B4 b4);
impl_boundary_for_tuple!(B1 b1, B2 b2, B3 b3, B4 b4, B5 b5);
impl_boundary_for_tuple!(B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6);
impl_boundary_for_tuple!(B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7);
impl_boundary_for_tuple!(B1 b1, B2 b2, B3 b3, B4 b4, B5 b5, B6 b6, B7 b7, B8 b8);
