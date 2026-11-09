//! The restore rules, their one enumeration, and the two emitters.
//!
//! Private: what a caller reaches is [`restore_rules`], re-exported at the
//! crate root, and the macros, which `#[macro_export]` puts there regardless.

/// The restore rules. See the [crate documentation](crate) for what each
/// covers.
///
/// Each rule is an async function taking `impl AsyncFn() -> F` — how to make a
/// [`RestoreFixture`](crate::RestoreFixture), as in the other families — and returning
/// `Result<RuleOutcome, Violation>`. No rule decodes a payload (SY-35): an
/// event is compared whole, and found by its type.
pub mod restore_rules {
    use alloc::collections::BTreeMap;
    use alloc::collections::btree_map::Entry;
    use alloc::format;
    use alloc::vec::Vec;
    use core::slice;

    use happenstance_core::{Event, EventId, EventType};
    use happenstance_sync::{ReplicatedEvent, SyncPeer, Waiting};
    use happenstance_testkit::RuleOutcome;

    use crate::RestoreFixture;
    use crate::violation::{Violation, ensure};

    /// The clause every rule here reports under.
    const VT_6: &str = "VT-6";

    /// The most pulls the rule makes before it decides the peer will never
    /// say it has caught up: a peer that loops fails rather than hangs (H-13).
    const MAX_PULLS: usize = 64;

    /// Written before the backup: the restore brings it back.
    const KEPT: EventType = EventType::from_static("KeptByTheBackup");
    /// Written after the backup: the restore takes it away, and the identity
    /// it was issued is the one a store that keeps its identity issues again.
    const LOST: EventType = EventType::from_static("LostToTheRestore");
    /// Written after the restore and the documented procedure.
    const WRITTEN_AFTER: EventType = EventType::from_static("WrittenAfterTheRestore");

    /// VT-6: a restored store does not issue an `EventId` it already issued
    /// for a different event.
    ///
    /// The rule writes one event, takes a backup, writes a second, and pulls
    /// the peer's whole log. Then it restores the backup, which takes every
    /// step the adapter documents for after a restore (VT-6's re-mint on a
    /// mint-once adapter), connects a fresh peer, writes a third event, and
    /// pulls the whole log again. No identity may name one event in the first
    /// pull and a different event in the second, or two events in either.
    ///
    /// It then checks that the restore was one: the first event is back
    /// under the identity it was issued, the second is gone, and the third is
    /// there. A fixture whose restore did nothing would otherwise pass for a
    /// store that never reissues. No position is compared: an identity is
    /// matched whole, against what the peer itself handed back.
    ///
    /// # Rejects
    ///
    /// The adapter VT-6's `Rejects:` line names: one that keys its
    /// `StoreId` on the device, or keeps the one a migration minted, and so
    /// after a restore hands Monday's identities to new events. Every peer that
    /// pulled the lost event dedupes the new one away as already seen. The same
    /// sequence with the documented re-mint left out is the negative control:
    /// `tests/wrong_restores.rs` holds the device-keyed peer, a restore that
    /// restores nothing, and a store that stamps its current incarnation on
    /// every event it reads back, so a re-mint rewrites the history it
    /// restored; `happenstance-postgres` runs its own store with the re-mint
    /// skipped.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of VT-6 if the fixture cannot write, back up or
    /// restore; if a pull fails or never ends; if one identity names two
    /// events; or if the restore did not bring back the backup.
    pub async fn restored_peer_does_not_reissue_identities<F: RestoreFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        let fixture = open().await;
        let (kept, lost, written_after) = (
            one_event(KEPT)?,
            one_event(LOST)?,
            one_event(WRITTEN_AFTER)?,
        );

        fixture.append_local(slice::from_ref(&kept)).await?;
        fixture.back_up().await?;
        fixture.append_local(slice::from_ref(&lost)).await?;
        let before = whole_log(&fixture.connect().await, "before the restore").await?;
        let kept_id = issued(&before, &kept, "before the restore")?;
        issued(&before, &lost, "before the restore")?;

        fixture.restore().await?;
        fixture
            .append_local(slice::from_ref(&written_after))
            .await?;
        let after = whole_log(&fixture.connect().await, "after the restore").await?;

        no_identity_names_two_events(&before, &after)?;
        let restored_id = issued(&after, &kept, "after the restore")?;
        ensure(restored_id == kept_id, VT_6, || {
            format!(
                "the event the backup held came back as {restored_id}, not the {kept_id} it was \
                 issued: the restore did not keep the identities it restored (VT-5), so a peer \
                 that pulled it takes it again as a new fact"
            )
        })?;
        ensure(!after.iter().any(|event| event.event == lost), VT_6, || {
            "the event written after the backup is still there after the restore: the fixture's \
             restore did not put the storage back, so nothing was restored for the rule to judge"
                .into()
        })?;
        issued(&after, &written_after, "after the restore")?;
        Ok(RuleOutcome::Ran)
    }

    /// An event of `event_type` with a payload no other event of the rule
    /// carries: its own type's name, as opaque bytes.
    ///
    /// `Event::new` cannot refuse an `EventType` already built; the error is
    /// mapped rather than unwrapped all the same.
    fn one_event(event_type: EventType) -> Result<Event, Violation> {
        let data = event_type.as_str().as_bytes().to_vec();
        Event::new(event_type, data).map_err(|error| {
            Violation::new(VT_6, format!("the rule's own event is invalid: {error}"))
        })
    }

    /// Every event `peer` hands back from the start, following its tokens.
    async fn whole_log<P: SyncPeer>(
        peer: &P,
        when: &str,
    ) -> Result<Vec<ReplicatedEvent>, Violation> {
        let mut events = Vec::new();
        let mut from: Option<P::Resume> = None;
        for _ in 0..MAX_PULLS {
            let pulled = peer.pull(from.as_ref()).await.map_err(|error| {
                Violation::new(VT_6, format!("a pull {when} was refused: {error}"))
            })?;
            events.extend(
                pulled
                    .batch
                    .groups
                    .into_iter()
                    .flat_map(|group| group.events),
            );
            if pulled.waiting == Waiting::Nothing {
                return Ok(events);
            }
            from = Some(pulled.resume);
        }
        Err(Violation::new(
            VT_6,
            format!("the peer {when} never said nothing was waiting in {MAX_PULLS} pulls"),
        ))
    }

    /// The identity the one event in `log` equal to `event` carries.
    fn issued(log: &[ReplicatedEvent], event: &Event, when: &str) -> Result<EventId, Violation> {
        let mut found = log.iter().filter(|held| held.event == *event);
        match (found.next(), found.next()) {
            (Some(held), None) => Ok(held.id),
            (None, _) => Err(Violation::new(
                VT_6,
                format!(
                    "the rule's local `{}` event is not in the log the peer handed back {when}",
                    event.event_type()
                ),
            )),
            (Some(first), Some(second)) => Err(Violation::new(
                VT_6,
                format!(
                    "the rule's one local `{}` event came back twice {when}, as {} and {}",
                    event.event_type(),
                    first.id,
                    second.id
                ),
            )),
        }
    }

    /// No identity names one event in `before` and another in `after`, or two
    /// events within either.
    fn no_identity_names_two_events(
        before: &[ReplicatedEvent],
        after: &[ReplicatedEvent],
    ) -> Result<(), Violation> {
        let mut named: BTreeMap<EventId, &Event> = BTreeMap::new();
        for held in before.iter().chain(after) {
            match named.entry(held.id) {
                Entry::Vacant(slot) => {
                    slot.insert(&held.event);
                }
                Entry::Occupied(slot) => ensure(*slot.get() == &held.event, VT_6, || {
                    format!(
                        "the store issued {} to a `{}` event and again to a `{}` event: after a \
                         restore it reissued an identity it had already given away, and every \
                         peer that holds the first will drop the second as already seen",
                        held.id,
                        slot.get().event_type(),
                        held.event.event_type()
                    )
                })?,
            }
        }
        Ok(())
    }
}

/// Hands the complete restore rule set to `$callback`.
///
/// This family's single enumeration. `$callback` is the path of a macro that
/// accepts a comma-separated list of identifiers, and it is invoked once, with
/// every rule in [`restore_rules`].
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_restore_rule!(rule_names);
/// assert!(names.contains(&"restored_peer_does_not_reissue_identities"));
/// ```
#[macro_export]
macro_rules! for_each_restore_rule {
    // Raw token trees, for the reason `for_each_sync_peer_rule!` records.
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- A restore does not reissue an identity (VT-6) --------------
            restored_peer_does_not_reissue_identities,
        }
    };
}

/// Emits one `#[tokio::test]` per restore rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`. The default emitter of
/// [`restore_conformance!`](crate::restore_conformance).
#[macro_export]
macro_rules! emit_restore_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::restore_rules::$name(__restore_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one plain `#[test]` per restore rule, driven by
/// [`happenstance_testkit::block_on`]: no runtime and no `Send` bound.
#[macro_export]
macro_rules! emit_restore_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::block_on(
                    $crate::__private::restore_rules::$name(__restore_fixture),
                )?
                .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Expands to a `[&str; N]` of the rule names, for the orphan meta-test.
#[cfg(test)]
macro_rules! restore_rule_names {
    ($($name:ident),* $(,)?) => { [ $( ::core::stringify!($name) ),* ] };
}

/// No rule exists without appearing in the enumeration, and none is
/// enumerated without existing: `no_orphan_sync_rules`, one family over, over
/// the same four-space prefix `cargo xtask spec-trace` reads.
#[cfg(test)]
#[test]
fn no_orphan_restore_rules() {
    const SOURCE: &str = include_str!("restore_suite.rs");
    const PREFIX: &str = "    pub async fn ";
    let declared: Vec<&str> = SOURCE
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .filter_map(|rest| {
            rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
        })
        .collect();
    let mut registered = crate::for_each_restore_rule!(restore_rule_names).to_vec();

    assert!(
        !declared.is_empty(),
        "the source scan matched no rule at all"
    );
    let mut declared = declared;
    declared.sort_unstable();
    registered.sort_unstable();
    assert_eq!(
        declared, registered,
        "the rules declared in `restore_suite.rs` and the rules `for_each_restore_rule!` \
         enumerates must be the same set"
    );
}
