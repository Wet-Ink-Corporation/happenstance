//! The watermark family (#143): where a runner keeps what a peer confirmed
//! (SY-31).
//!
//! Not a peer's bar and not an ingest store's, but a **projection store's**:
//! the store a runner keeps its per-peer watermark in, beside the read model
//! whose rows depend on it. The rule takes a
//! [`ProjectionFixture`](happenstance_testkit::ProjectionFixture), the fixture
//! the projection suite already mounts, so a projection adapter calls it over
//! the fixture it has. There is no mounting macro, as in the convergence
//! family: one rule, called directly, with no capability to decline.
//!
//! # What the rule plays
//!
//! The runner. It drives a real [`Edge`](happenstance_sync::runner::Edge) from
//! an in-memory local log to an in-memory far side that names its store, keeps
//! the edge's [`Watermark`](happenstance_sync::runner::Watermark) through the
//! fixture's store in a batch that also carries a read-model row, and reads
//! both back through a fresh handle. So what it holds the store to is what the
//! shipped runner asks of it: a checkpoint under a `sync/` id the store did
//! not mint, committed with the batch, kept apart per peer, refused when it
//! would move backwards, and seen from outside the handle that wrote it.

/// The watermark rule. See the [crate documentation](crate) for what it
/// covers.
///
/// Returns `Result<RuleOutcome, Violation>`, as every rule in this crate does.
///
/// # What the rule does not do
///
/// **Assert a local position it did not read back.** Every position compared
/// is one the local log assigned, read through
/// [`WriteGroupStore::read_groups`](happenstance_sync::WriteGroupStore::read_groups)
/// or reported by the edge.
pub mod watermark_rules {
    use alloc::format;
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use happenstance_core::{
        CommitError, Event, EventStore, ProjectionProbe, ProjectionStore, SequencePosition, StoreId,
    };
    use happenstance_sync::runner::{Edge, Watermark};
    use happenstance_sync::{MemoryIngestStore, StoreResume, StoreSyncPeer, WriteGroupStore};
    use happenstance_testkit::{ProjectionFixture, RuleOutcome};

    use crate::violation::{Violation, ensure};
    use crate::{IngestFixture, MemoryIngestFixture, MemoryIngestHandle};

    /// The rule's clause.
    const CLAUSE: &str = "SY-31";

    /// The local log's incarnation.
    const LOCAL: StoreId = StoreId::from_bytes([0x31; 16]);

    /// The most steps an edge takes before the rule decides it will never sync
    /// (H-13): far above what three small groups need.
    const MAX_STEPS: usize = 16;

    /// A page large enough for the whole local log.
    const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

    /// The read-model row each commit carries, by key.
    const ROW_WITH_THE_HUB: &str = "sync-watermark/confirmed-by-hub";
    const ROW_WITH_THE_SPOKE: &str = "sync-watermark/confirmed-by-spoke";
    const ROW_WITH_THE_STALE: &str = "sync-watermark/stale";

    /// A far side over the in-memory reference store, which names its
    /// incarnation on every answer, so the edge learns whose watermark it holds.
    type Far = StoreSyncPeer<MemoryIngestHandle, MemoryIngestHandle>;

    /// **SY-31.** A runner's per-peer watermark is kept through the projection
    /// port, under the reserved id, in the same commit as a read model.
    ///
    /// The rule appends two groups to a local log and syncs an edge with a
    /// hub, then commits the edge's watermark through the fixture's store in a
    /// batch carrying one probe row. Through a fresh handle it requires:
    ///
    /// 1. the row and the watermark both, so they landed in one commit;
    /// 2. an edge restored from that watermark to push nothing more;
    /// 3. after a third group and a second edge, to a spoke, committed with a
    ///    row of its own, each peer's watermark its own;
    /// 4. a hub watermark below the kept one refused as
    ///    [`CommitError::CheckpointRegression`], with neither its row nor the
    ///    watermark changed.
    ///
    /// # Rejects
    ///
    /// `tests/wrong_watermark_stores.rs` carries four projection stores, each
    /// wrong only for a `sync/` id: one that refuses the reserved prefix as an
    /// application id it does not accept, one that keeps sync ids in its
    /// handle rather than its store, one that keys every peer's watermark
    /// under the bare prefix, and one that upserts a watermark into a side
    /// table without the port's regression check.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-31 if the fixture declines a second handle, if the
    /// store refuses or loses a watermark, keeps it apart from its batch,
    /// shares it between peers or lets it move backwards, or if the reference
    /// edge fails to sync.
    pub async fn watermark_advances_transactionally_with_a_read_model<F: ProjectionFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        if let Some(reason) = F::SECOND_HANDLE.reason() {
            return Err(Violation::new(
                CLAUSE,
                format!(
                    "a watermark kept through the port is a claim about what a fresh handle \
                     reads, and this fixture cannot open one ({reason}); SECOND_HANDLE is a MUST"
                ),
            ));
        }
        let fixture = open().await;
        let writer = fixture.connect().await;
        let local = MemoryIngestStore::with_store_id(LOCAL);
        let hub_store = MemoryIngestFixture::new();
        let hub = far_side(&hub_store).await;

        append(&local, 2).await?;
        append(&local, 1).await?;
        let first = synced_watermark(&mut Edge::new(), &hub, &local).await?;
        commit_with_row(&writer, &first, ROW_WITH_THE_HUB).await?;
        let observer = fixture.connect().await;
        kept_with_its_row(&observer, &first, ROW_WITH_THE_HUB).await?;
        restored_pushes_nothing(&first, &hub, &local).await?;

        append(&local, 1).await?;
        let spoke_store = MemoryIngestFixture::new();
        let second =
            synced_watermark(&mut Edge::new(), &far_side(&spoke_store).await, &local).await?;
        commit_with_row(&writer, &second, ROW_WITH_THE_SPOKE).await?;
        let observer = fixture.connect().await;
        kept_with_its_row(&observer, &second, ROW_WITH_THE_SPOKE).await?;
        kept(&observer, &first, Requirement::EachPeersOwn).await?;

        stale_is_refused(&fixture, &local, &first).await?;
        Ok(RuleOutcome::Ran)
    }

    /// Requirement 4: a watermark below the kept one, in a batch with a row,
    /// is refused as a regression, and leaves the row out and the watermark
    /// where it was.
    async fn stale_is_refused<F: ProjectionFixture>(
        fixture: &F,
        local: &MemoryIngestStore,
        kept_one: &Watermark,
    ) -> Result<(), Violation> {
        let lowest = first_group(local).await?;
        ensure(lowest < kept_one.confirmed(), CLAUSE, || {
            format!(
                "the rule's own arrangement: the first local group, at {lowest}, is not below \
                 the watermark {}",
                kept_one.confirmed()
            )
        })?;
        let stale = Watermark::new(kept_one.peer(), lowest);
        let writer = fixture.connect().await;
        let mut batch = begin(&writer).await?;
        probe_write(&writer, &mut batch, ROW_WITH_THE_STALE).await?;
        let refused = stale.commit(&writer, batch).await;
        ensure(
            matches!(
                refused,
                Err(CommitError::CheckpointRegression { current, attempted })
                    if current == kept_one.confirmed() && attempted == lowest
            ),
            CLAUSE,
            || {
                format!(
                    "a watermark of {lowest} for a peer whose kept watermark is {} must be \
                     refused as a checkpoint regression, the port's answer for any projection; \
                     the store answered {refused:?}. A watermark that moves backwards re-pushes \
                     what the peer already holds, and one written around the port's check is \
                     one written around its transaction",
                    kept_one.confirmed()
                )
            },
        )?;
        let observer = fixture.connect().await;
        kept(&observer, kept_one, Requirement::UnmovedByARegression).await?;
        let row = probe_read(&observer, ROW_WITH_THE_STALE).await?;
        ensure(row.is_none(), CLAUSE, || {
            format!(
                "the refused commit's row is in the read model ({row:?}): a refused watermark \
                 must leave the batch it rode in uncommitted"
            )
        })
    }

    /// A far side over a fresh reference store, named on every answer.
    async fn far_side(store: &MemoryIngestFixture) -> Far {
        StoreSyncPeer::new(store.connect().await, store.connect().await)
    }

    /// Appends one group of `events` opaque events to `local`.
    async fn append(local: &MemoryIngestStore, events: usize) -> Result<(), Violation> {
        let group = (0..events)
            .map(|_| Event::new("WatermarkWritten", b"opaque".to_vec()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                Violation::new(CLAUSE, format!("the rule's own event is invalid: {error}"))
            })?;
        EventStore::append(local, &group, None)
            .await
            .map(|_| ())
            .map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!("the reference local log refused: {error:?}"),
                )
            })
    }

    /// Steps `edge` until it is synced, and hands back its watermark.
    async fn synced_watermark(
        edge: &mut Edge<StoreResume>,
        peer: &Far,
        local: &MemoryIngestStore,
    ) -> Result<Watermark, Violation> {
        for _ in 0..MAX_STEPS {
            edge.step(peer, local).await.map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!("the reference edge failed to step: {error:?}"),
                )
            })?;
            if edge.status().is_synced() {
                return edge.watermark().ok_or_else(|| {
                    Violation::new(
                        CLAUSE,
                        format!(
                            "the reference edge synced with a far side that names itself and \
                             holds no watermark: {:?}",
                            edge.status()
                        ),
                    )
                });
            }
        }
        Err(Violation::new(
            CLAUSE,
            format!(
                "the reference edge did not sync in {MAX_STEPS} steps: {:?}",
                edge.status()
            ),
        ))
    }

    /// Begins a batch, writes one probe row into it, and commits it with
    /// `watermark`.
    async fn commit_with_row<S: ProjectionProbe>(
        store: &S,
        watermark: &Watermark,
        key: &str,
    ) -> Result<(), Violation> {
        let mut batch = begin(store).await?;
        probe_write(store, &mut batch, key).await?;
        watermark.commit(store, batch).await.map_err(|error| {
            Violation::new(
                CLAUSE,
                format!(
                    "the store refused to commit a watermark under `{}` with its batch: {error}. \
                     Every id but a sync watermark's refuses the `sync/` prefix (ADR-0082), so a \
                     store that refuses it too has nowhere to keep a runner's watermark",
                    watermark.id()
                ),
            )
        })
    }

    /// Requirement 1's two halves, through `observer`: the row, and the
    /// watermark.
    async fn kept_with_its_row<S: ProjectionProbe>(
        observer: &S,
        watermark: &Watermark,
        key: &str,
    ) -> Result<(), Violation> {
        let row = probe_read(observer, key).await?;
        ensure(row == Some(PROBE_VALUE), CLAUSE, || {
            format!(
                "the row committed with a watermark reads back as {row:?} through a fresh \
                 handle; the watermark and the read model must land in one commit"
            )
        })?;
        kept(observer, watermark, Requirement::KeptWithItsRow).await
    }

    /// Which numbered requirement a [`kept`] check serves, so a violation says
    /// which one failed.
    #[derive(Debug, Clone, Copy)]
    enum Requirement {
        /// 1: read back with the row it was committed with.
        KeptWithItsRow,
        /// 3: one peer's watermark untouched by another's.
        EachPeersOwn,
        /// 4: unchanged by a refused regression.
        UnmovedByARegression,
    }

    impl core::fmt::Display for Requirement {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(match self {
                Self::KeptWithItsRow => "requirement 1, a watermark kept with its row",
                Self::EachPeersOwn => "requirement 3, each peer's watermark its own",
                Self::UnmovedByARegression => {
                    "requirement 4, a watermark unmoved by a refused regression"
                }
            })
        }
    }

    /// `watermark` is what `observer` loads for its peer, as `requirement`
    /// asks.
    async fn kept<S: ProjectionStore>(
        observer: &S,
        watermark: &Watermark,
        requirement: Requirement,
    ) -> Result<(), Violation> {
        let loaded = Watermark::load(observer, watermark.peer())
            .await
            .map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!(
                        "{requirement}: a fresh handle could not read a kept watermark: {error:?}"
                    ),
                )
            })?;
        ensure(loaded == Some(*watermark), CLAUSE, || {
            format!(
                "{requirement}: a fresh handle loads {loaded:?} under `{}` where {watermark:?} was \
                 committed: a \
                 watermark is a checkpoint of the store, kept per peer, not of the handle that \
                 wrote it nor one shared by every peer",
                watermark.id()
            )
        })
    }

    /// Requirement 2: an edge restored from the kept watermark, which
    /// [`kept`] has just read back, pushes nothing the hub already
    /// acknowledged.
    async fn restored_pushes_nothing(
        watermark: &Watermark,
        hub: &Far,
        local: &MemoryIngestStore,
    ) -> Result<(), Violation> {
        let mut restored = Edge::restored(*watermark);
        let stepped = restored.step(hub, local).await.map_err(|error| {
            Violation::new(
                CLAUSE,
                format!("the restored edge failed to step: {error:?}"),
            )
        })?;
        ensure(stepped.pushed == 0, CLAUSE, || {
            format!(
                "an edge restored from the kept watermark pushed {} events the hub had \
                 already acknowledged",
                stepped.pushed
            )
        })
    }

    /// The first position of the local log's first write group.
    async fn first_group(local: &MemoryIngestStore) -> Result<SequencePosition, Violation> {
        let page = WriteGroupStore::read_groups(local, None, WHOLE_LOG)
            .await
            .map_err(|error| {
                Violation::new(CLAUSE, format!("the reference local log failed: {error:?}"))
            })?;
        page.groups
            .first()
            .map(happenstance_sync::WriteGroup::first)
            .ok_or_else(|| Violation::new(CLAUSE, "the reference local log holds no group"))
    }

    /// The value every probe row carries.
    const PROBE_VALUE: u64 = 31;

    async fn begin<S: ProjectionStore>(store: &S) -> Result<S::Batch, Violation> {
        store.begin().await.map_err(|error| {
            Violation::new(
                CLAUSE,
                format!("the store could not begin a batch: {error}"),
            )
        })
    }

    async fn probe_write<S: ProjectionProbe>(
        store: &S,
        batch: &mut S::Batch,
        key: &str,
    ) -> Result<(), Violation> {
        store
            .probe_write(batch, key, PROBE_VALUE)
            .await
            .map_err(|error| {
                Violation::new(CLAUSE, format!("the store refused a probe row: {error}"))
            })
    }

    async fn probe_read<S: ProjectionProbe>(
        store: &S,
        key: &str,
    ) -> Result<Option<u64>, Violation> {
        store.probe_read(key).await.map_err(|error| {
            Violation::new(
                CLAUSE,
                format!("the store could not read a probe row: {error}"),
            )
        })
    }
}

/// Hands the complete watermark rule set to `$callback`.
///
/// This family's single enumeration (CF-22), beside the rule it names. No
/// mounting macro reads it: a projection adapter calls the rule over its own
/// fixture.
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_watermark_rule!(rule_names);
/// assert_eq!(names, ["watermark_advances_transactionally_with_a_read_model"]);
/// ```
#[macro_export]
macro_rules! for_each_watermark_rule {
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- The watermark is kept through the port (SY-31) ---------------
            watermark_advances_transactionally_with_a_read_model,
        }
    };
}
