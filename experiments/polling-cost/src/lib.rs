//! The polling cost ES-32 imposes, as a number.
//!
//! ES-32 forbids `EventStore` from growing a tail, subscribe or notify method at
//! 0.1: consumers poll. A deployment with N views therefore performs N
//! independent reads of the same log and has the same events delivered to it N
//! times. That is the cost, and until this harness existed every argument about
//! it in this repository was an estimate.
//!
//! # What is reported, and what is not
//!
//! The headline is a **count**, not a duration: *delivery amplification*, the
//! events the store yielded across all views divided by the distinct events
//! those deliveries carried. It is dimensionless, machine-independent and
//! reproducible on any laptop. Wall-clock figures are recorded as secondary,
//! always with the machine that produced them attached.
//!
//! Nothing here asserts. The harness exits non-zero when *it* fails, never
//! because a number was large, and it holds no threshold. CF-34 is why: a
//! benchmark inside the conformance bar is a threshold whose first red build is
//! resolved by raising the threshold, after which it measures nothing and blocks
//! everything.
//!
//! # Staleness is observed, never subtracted
//!
//! `head() - checkpoint` appears nowhere — not in this code, not in a README
//! column, not in a derived field. Positions are an opaque ordering key and the
//! specification permits gaps, so the difference counts nothing. [`Observer`]
//! compares positions instead, which is sound on a store with holes.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use happenstance::{
    Codec, CodecError, DomainEvent, Event, EventStore, EventType, Json, MemoryEventStore,
    MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError, Progressed,
    Projection, ProjectionId, ProjectionStore, SequencePosition, Tags,
};
use serde::{Deserialize, Serialize};

pub mod observer;
pub mod progress;
// Test support, and the only module here allowed to `assert!`: it reads the
// harness's own source so a test can hold it to its negative requirements. It
// runs during no measurement, which is why `no_verdict.rs` excludes it from the
// sweep for thresholds rather than making an exception inside a scan.
pub mod counting;
pub mod sources;
pub mod validate;

pub use counting::CountingStore;
pub use observer::Observer;
pub use sources::harness_sources;

/// The result-record contract's version. Changing a record's shape bumps it; it
/// never silently re-interprets a file already committed.
pub const SCHEMA_VERSION: u32 = 1;

/// The fan-out axis: how many views poll the same log.
pub const FAN_OUTS: [usize; 6] = [1, 2, 4, 8, 16, 32];

/// The log-size axis, in events.
pub const LOG_SIZES: [usize; 3] = [1_000, 10_000, 100_000];

/// The poll-interval axis, in milliseconds.
pub const POLL_INTERVALS_MS: [u64; 3] = [10, 100, 1_000];

/// Both selectivity arms. Neither is optional: the overlapping arm is ES-32's
/// worst case and the one the tail-seam argument turns on, and the disjoint arm
/// is what makes the ratio mean anything by being the case where it does not
/// grow.
pub const ARMS: [Arm; 2] = [Arm::Overlapping, Arm::Disjoint];

/// Events per `begin`/`commit` pair, fixed and recorded.
pub const CHUNK: usize = 1_024;

/// Repeats per swept cell.
pub const REPEATS: usize = 2;

/// The seed every randomised choice derives from. Written into every record
/// rather than described in prose.
pub const SEED: u64 = 0x_5011_C057;

/// The log the staleness phase measures over, held small on purpose: its cost is
/// paid in real seconds at the poll interval, and the quantity under study there
/// is latency rather than volume.
pub const STALENESS_LOG: usize = 1_000;

/// How many events the staleness phase appends after the views have caught up.
pub const STALENESS_APPENDS: usize = 4;

/// The one event type the measured domain declares.
const MEASURED: EventType = EventType::from_static("Measured");

// ---------------------------------------------------------------------------
// The swept grid
// ---------------------------------------------------------------------------

/// Which selectivity arm a cell runs under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arm {
    /// Every view's query selects the whole log. ES-32's worst case.
    Overlapping,
    /// Each view selects about one Nth of the log.
    Disjoint,
}

impl Arm {
    /// The spelling that goes into a record and a README row.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Overlapping => "overlapping",
            Self::Disjoint => "disjoint",
        }
    }
}

/// Which quantity a cell measures.
///
/// Two phases rather than one grid, because the two quantities do not depend on
/// the same axes and multiplying them together would buy a cross product nobody
/// reads. Replay amplification does not move with the poll interval — the
/// runner resumes past its checkpoint, so an idle poll delivers nothing — and
/// staleness does not move with the log size once the views have caught up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    /// Replay a seeded log into N views and count the deliveries.
    Amplification,
    /// Append into a caught-up deployment and time the observations.
    Staleness,
}

impl Phase {
    /// The spelling that goes into a record.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Amplification => "amplification",
            Self::Staleness => "staleness",
        }
    }
}

/// One swept cell: every condition that varies, fixed to a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// Which quantity this cell measures.
    pub phase: Phase,
    /// How many views poll the log.
    pub fan_out: usize,
    /// How many events the log holds.
    pub log_size: usize,
    /// How long the driver waits between polls, in milliseconds.
    pub poll_interval_ms: u64,
    /// Events per `begin`/`commit` pair.
    pub chunk: usize,
    /// Which selectivity arm.
    pub arm: Arm,
    /// Which repeat of this cell.
    pub repeat: usize,
    /// The seed this cell's choices derive from.
    pub seed: u64,
}

/// The whole grid, in the order it is run.
///
/// Every declared value of every declared axis appears. The amplification phase
/// sweeps fan-out × log size × arm; the staleness phase sweeps fan-out × poll
/// interval × arm at a fixed small log. The trim is recorded in the README with
/// its reason, which is what the spec permits — the *shape* is not trimmed and
/// no axis value is dropped.
#[must_use]
pub fn sweep() -> Vec<Cell> {
    let mut cells = Vec::new();
    for repeat in 0..REPEATS {
        for arm in ARMS {
            for fan_out in FAN_OUTS {
                for log_size in LOG_SIZES {
                    cells.push(Cell {
                        phase: Phase::Amplification,
                        fan_out,
                        log_size,
                        poll_interval_ms: POLL_INTERVALS_MS[0],
                        chunk: CHUNK,
                        arm,
                        repeat,
                        seed: SEED,
                    });
                }
                for poll_interval_ms in POLL_INTERVALS_MS {
                    cells.push(Cell {
                        phase: Phase::Staleness,
                        fan_out,
                        log_size: STALENESS_LOG,
                        poll_interval_ms,
                        chunk: CHUNK,
                        arm,
                        repeat,
                        seed: SEED,
                    });
                }
            }
        }
    }
    cells
}

// ---------------------------------------------------------------------------
// The measured domain and the measured projection
// ---------------------------------------------------------------------------

/// One event, carrying the shard it was tagged with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Measured {
    shard: u32,
    ordinal: u64,
}

impl DomainEvent for Measured {
    const EVENT_TYPES: &'static [EventType] = &[MEASURED];

    fn event_type(&self) -> EventType {
        MEASURED
    }

    fn tags(&self) -> Tags {
        shard_tags(self.shard)
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<happenstance::bytes::Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        _: &EventType,
        data: &happenstance::bytes::Bytes,
    ) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

fn shard_tags(shard: u32) -> Tags {
    Tags::from_pairs([("shard", shard.to_string().as_str())]).expect("a valid shard tag")
}

/// The view under measurement.
///
/// Deliberately near-zero-cost — one counter and one row per commit. The
/// quantity under study is the redundant *read* fan-out, and read-model work
/// would dilute it. That is a design choice **and** a stated limitation, and it
/// is written in both places.
#[derive(Debug)]
struct View {
    id: ProjectionId,
    key: String,
    scope: Tags,
    applied: u64,
}

impl View {
    fn new(index: usize, arm: Arm, fan_out: usize) -> Self {
        let scope = match arm {
            // Every view selects the whole log: the query nominates by event
            // type alone, so no tag narrows it.
            Arm::Overlapping => Tags::empty(),
            // Each view selects about one Nth: the shard tag is what narrows it,
            // and it is the same `Query` vocabulary either way.
            Arm::Disjoint => shard_tags(u32::try_from(index % fan_out.max(1)).unwrap_or(0)),
        };
        Self {
            id: ProjectionId::new(format!("view_{index}")),
            key: format!("view_{index}"),
            scope,
            applied: 0,
        }
    }
}

impl Projection for View {
    type Event = Measured;
    type Store = MemoryProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        _: Measured,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.applied += 1;
        batch.write(self.key.clone(), self.applied);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The record contract
// ---------------------------------------------------------------------------

/// One line of `results/`: a cell, every condition it ran under, and what it
/// measured.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// The record contract's version.
    pub schema_version: u32,
    /// `"record"`, so a reader can tell a record from the manifest without
    /// inspecting its fields.
    pub kind: String,
    /// Which quantity this cell measured.
    pub phase: String,
    /// How many views polled the log.
    pub fan_out: usize,
    /// How many events the log held.
    pub log_size: usize,
    /// The interval between polls, in milliseconds.
    pub poll_interval_ms: u64,
    /// Events per `begin`/`commit` pair.
    pub chunk: usize,
    /// Which selectivity arm.
    pub arm: String,
    /// Which repeat of this cell.
    pub repeat: usize,
    /// The seed this cell's choices derived from.
    pub seed: u64,
    /// Which projection store held the read models.
    pub projection_store: String,
    /// How many `read` calls the views issued between them.
    pub reads_issued: u64,
    /// How many events the store yielded, summed across every view and poll.
    pub events_delivered: u64,
    /// How many times a projection's `apply` ran, summed across views.
    pub events_applied_total: u64,
    /// How many **distinct** log events were applied by at least one view.
    ///
    /// The headline's denominator, and the reason the two arms differ. Under a
    /// derived query and a filtering store, every event delivered to a view is
    /// applied by it, so `events_delivered / events_applied_total` is
    /// identically one and tells a reader nothing. Deduplicating asks the
    /// question ES-32 is actually about: how many times was the same event
    /// delivered?
    pub events_applied_distinct: u64,
    /// `events_delivered ÷ events_applied_distinct`. The headline.
    pub delivery_amplification: f64,
    /// Median observed staleness, in nanoseconds. `null` outside the staleness
    /// phase.
    pub staleness_ns_p50: Option<u64>,
    /// 95th-percentile observed staleness, in nanoseconds.
    pub staleness_ns_p95: Option<u64>,
    /// The largest backlog seen at a poll boundary: events appended and not yet
    /// observed by a view, counted by comparing positions.
    pub pending_at_poll_max: Option<u64>,
    /// How long the cell took, in nanoseconds. Secondary, and only meaningful
    /// beside the machine in the manifest.
    pub elapsed_ns: u64,
}

/// The one manifest line at the head of a pass: the environment, written once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// The record contract's version.
    pub schema_version: u32,
    /// `"manifest"`.
    pub kind: String,
    /// The tag this pass was written under.
    pub run_tag: String,
    /// `rustc -vV`, joined onto one line.
    pub rustc: String,
    /// `cargo -V`.
    pub cargo: String,
    /// The commit the measured tree was at.
    pub git_rev: String,
    /// Whether the **measured** tree — `crates/` and the workspace manifests —
    /// had uncommitted changes.
    ///
    /// Scoped to what was measured rather than to the whole worktree on purpose:
    /// the harness measures the library, and a committed pass must be
    /// re-derivable from a commit of the library. The harness's own directory
    /// moving does not make a number irreproducible; the library's moving does.
    pub git_dirty: bool,
    /// `release` or `debug`.
    pub profile: String,
    /// The operating system the run happened on.
    pub os: String,
    /// The CPU model, as the OS reports it.
    pub cpu: String,
    /// Logical CPUs visible to the process.
    pub logical_cpus: usize,
    /// Total RAM in bytes, or `0` where the OS did not say.
    pub ram_bytes: u64,
    /// The feature set the measured crate was built with.
    pub features: String,
    /// The instant the pass started, as an RFC-3339-ish UTC string.
    pub started_at: String,
    /// Which projection store held the read models.
    pub projection_store: String,
}

/// The projection store this harness used.
///
/// Exactly two values are admissible and the field is never free text: HS-P0010
/// landed `MemoryProjectionStore`, so a harness-local store would be a second
/// fixture shape nobody asked for.
pub const PROJECTION_STORE: &str = "happenstance_core::MemoryProjectionStore";

/// The other admissible value, kept so `schema.rs` can assert the field is one
/// of exactly two and so the README can say which was not used.
pub const PROJECTION_STORE_LOCAL: &str = "polling-cost::LocalProjectionStore";

// ---------------------------------------------------------------------------
// Running a cell
// ---------------------------------------------------------------------------

/// Why a cell could not produce a number.
///
/// The harness fails only for its own reasons. There is no arm of this type for
/// *a number was large*, and adding one would make the harness a gate step
/// wearing a different name.
#[derive(Debug)]
pub enum HarnessError {
    /// The event store refused a seed append.
    Seed(String),
    /// The runner returned an error.
    Run(String),
    /// A pass could not be written, or its tag already existed.
    Write(String),
}

impl std::fmt::Display for HarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seed(why) => write!(f, "seeding the log failed: {why}"),
            Self::Run(why) => write!(f, "the runner failed: {why}"),
            Self::Write(why) => write!(f, "writing the pass failed: {why}"),
        }
    }
}

impl std::error::Error for HarnessError {}

/// Runs one cell and produces its record.
///
/// This is the single entry point the sweep uses, so a test that drives one cell
/// drives exactly what the recorded pass drove.
///
/// # Errors
///
/// Returns [`HarnessError`] when the harness itself fails — never because a
/// measured number was large.
pub fn run_cell(cell: &Cell) -> Result<Record, HarnessError> {
    match cell.phase {
        Phase::Amplification => amplification(cell),
        Phase::Staleness => staleness(cell),
    }
}

fn seed_log(events: &MemoryEventStore, cell: &Cell) -> Result<(), HarnessError> {
    let fan_out = u32::try_from(cell.fan_out.max(1)).unwrap_or(1);
    let mut batch = Vec::with_capacity(cell.log_size.min(1_024));
    for ordinal in 0..cell.log_size {
        let shard = u32::try_from(ordinal % cell.fan_out.max(1)).unwrap_or(0) % fan_out;
        let value = Measured {
            shard,
            ordinal: ordinal as u64,
        };
        let payload = value
            .encode(&Json)
            .map_err(|e| HarnessError::Seed(e.to_string()))?;
        let event = Event::new(MEASURED, payload)
            .map_err(|e| HarnessError::Seed(e.to_string()))?
            .with_tags(value.tags());
        batch.push(event);
        if batch.len() == 1_024 {
            append(events, &batch)?;
            batch.clear();
        }
    }
    if !batch.is_empty() {
        append(events, &batch)?;
    }
    Ok(())
}

fn append(events: &MemoryEventStore, batch: &[Event]) -> Result<SequencePosition, HarnessError> {
    block_on(EventStore::append(events, batch, None)).map_err(|e| HarnessError::Seed(e.to_string()))
}

/// Drives every view to the end of the seeded log and counts the deliveries.
fn amplification(cell: &Cell) -> Result<Record, HarnessError> {
    let inner = MemoryEventStore::new();
    seed_log(&inner, cell)?;
    let events = CountingStore::new(inner);
    let models = MemoryProjectionStore::new();

    let mut views: Vec<View> = (0..cell.fan_out)
        .map(|index| View::new(index, cell.arm, cell.fan_out))
        .collect();

    let started = Instant::now();
    let mut applied_total = 0u64;

    for view in &mut views {
        loop {
            let progress = run_once(&events, &models, view, cell)?;
            if progress.applied == 0 {
                break;
            }
            applied_total += u64::try_from(progress.applied).unwrap_or(u64::MAX);
        }
    }
    let elapsed = started.elapsed();

    // Both figures are **observed**, by the instrument, at the moment the store
    // handed each event over. The denominator is the distinct positions those
    // deliveries carried: every one of them was applied, because the runner
    // applies what it is handed and the query is derived rather than supplied.
    let delivered = events.delivered();
    let distinct_applied = events.distinct_delivered();
    Ok(Record {
        schema_version: SCHEMA_VERSION,
        kind: "record".to_owned(),
        phase: cell.phase.as_str().to_owned(),
        fan_out: cell.fan_out,
        log_size: cell.log_size,
        poll_interval_ms: cell.poll_interval_ms,
        chunk: cell.chunk,
        arm: cell.arm.as_str().to_owned(),
        repeat: cell.repeat,
        seed: cell.seed,
        projection_store: PROJECTION_STORE.to_owned(),
        reads_issued: events.reads(),
        events_delivered: delivered,
        events_applied_total: applied_total,
        events_applied_distinct: distinct_applied,
        delivery_amplification: ratio(delivered, distinct_applied),
        staleness_ns_p50: None,
        staleness_ns_p95: None,
        pending_at_poll_max: None,
        elapsed_ns: u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX),
    })
}

/// Appends into a caught-up deployment and times what the views observe.
fn staleness(cell: &Cell) -> Result<Record, HarnessError> {
    let inner = MemoryEventStore::new();
    seed_log(&inner, cell)?;
    let events = CountingStore::new(inner);
    let models = MemoryProjectionStore::new();

    let mut views: Vec<View> = (0..cell.fan_out)
        .map(|index| View::new(index, cell.arm, cell.fan_out))
        .collect();

    // Catch every view up first: staleness is about what an already-running
    // deployment sees, not about a cold replay. The counters are then reset, so
    // the steady-state figures are not inflated by the replay that preceded
    // them.
    for view in &mut views {
        while run_once(&events, &models, view, cell)?.applied > 0 {}
    }
    events.reset();

    let started = Instant::now();
    let mut observers: Vec<Observer> = (0..cell.fan_out).map(|_| Observer::new()).collect();
    let mut pending_max = 0u64;
    let interval = std::time::Duration::from_millis(cell.poll_interval_ms);

    for ordinal in 0..STALENESS_APPENDS {
        let shard = u32::try_from(ordinal % cell.fan_out.max(1)).unwrap_or(0);
        let value = Measured {
            shard,
            ordinal: (cell.log_size + ordinal) as u64,
        };
        let payload = value
            .encode(&Json)
            .map_err(|e| HarnessError::Seed(e.to_string()))?;
        let event = Event::new(MEASURED, payload)
            .map_err(|e| HarnessError::Seed(e.to_string()))?
            .with_tags(value.tags());
        let position = append(&events.inner, std::slice::from_ref(&event))?;
        let at = started.elapsed();
        // Only the views that **nominated** the event are told about it. In the
        // disjoint arm an event belongs to one shard, so the other views never
        // select it and never should have: an observer told about an event its
        // view's query excludes reports a staleness that is really the harness
        // waiting for something that was never coming.
        for (view, observer) in views.iter().zip(observers.iter_mut()) {
            if nominates(view, &value) {
                observer.appended(position, u64::try_from(at.as_nanos()).unwrap_or(u64::MAX));
            }
        }

        // Poll until every view that nominated the event has observed it, at
        // the cell's interval. Bounded so a view whose query never selects the
        // event cannot hang the sweep.
        for _ in 0..8 {
            std::thread::sleep(interval);
            let now = u64::try_from(started.elapsed().as_nanos()).unwrap_or(u64::MAX);
            let mut outstanding = false;
            for (view, observer) in views.iter_mut().zip(observers.iter_mut()) {
                let _ = run_once(&events, &models, view, cell)?;
                let checkpoint = block_on(models.checkpoint(view.id()))
                    .map_err(|e| HarnessError::Run(e.to_string()))?;
                observer.observe(checkpoint, now);
                let pending = observer.pending(checkpoint);
                pending_max = pending_max.max(u64::try_from(pending).unwrap_or(u64::MAX));
                if pending > 0 {
                    outstanding = true;
                }
            }
            if !outstanding {
                break;
            }
        }
    }
    let elapsed = started.elapsed();

    let mut samples: Vec<u64> = observers
        .iter()
        .flat_map(|observer| observer.staleness_ns().to_vec())
        .collect();
    samples.sort_unstable();

    let delivered = events.delivered();
    Ok(Record {
        schema_version: SCHEMA_VERSION,
        kind: "record".to_owned(),
        phase: cell.phase.as_str().to_owned(),
        fan_out: cell.fan_out,
        log_size: cell.log_size,
        poll_interval_ms: cell.poll_interval_ms,
        chunk: cell.chunk,
        arm: cell.arm.as_str().to_owned(),
        repeat: cell.repeat,
        seed: cell.seed,
        projection_store: PROJECTION_STORE.to_owned(),
        reads_issued: events.reads(),
        events_delivered: delivered,
        events_applied_total: views.iter().map(|view| view.applied).sum(),
        events_applied_distinct: events.distinct_delivered(),
        delivery_amplification: ratio(delivered, events.distinct_delivered()),
        staleness_ns_p50: percentile(&samples, 50),
        staleness_ns_p95: percentile(&samples, 95),
        pending_at_poll_max: Some(pending_max),
        elapsed_ns: u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX),
    })
}

/// One poll of one view, through the **real** runner.
fn run_once(
    events: &CountingStore,
    models: &MemoryProjectionStore,
    view: &mut View,
    cell: &Cell,
) -> Result<Progressed, HarnessError> {
    let chunk = std::num::NonZeroUsize::new(cell.chunk).unwrap_or(std::num::NonZeroUsize::MIN);
    block_on(happenstance::run_projection(
        events, models, view, &Json, chunk,
    ))
    .map_err(|e| HarnessError::Run(e.to_string()))
}

/// Whether a view's derived query selects this event.
///
/// The same two inputs the runner derives from — the domain type's event set
/// and the projection's scope — asked here rather than restated: an empty scope
/// nominates everything, and a shard scope nominates its own shard.
fn nominates(view: &View, event: &Measured) -> bool {
    view.scope == Tags::empty() || view.scope == event.tags()
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn percentile(sorted: &[u64], p: usize) -> Option<u64> {
    if sorted.is_empty() {
        return None;
    }
    let index = (sorted.len().saturating_sub(1)) * p / 100;
    sorted.get(index).copied()
}

// ---------------------------------------------------------------------------
// Writing a pass
// ---------------------------------------------------------------------------

/// Writes one pass under `root/<tag>/`, refusing to overwrite an existing one.
///
/// Re-runs are **additive**: a re-measurement writes beside the pass it is being
/// compared with rather than over it, so undoing a re-run is deleting one
/// directory and a committed pass is never edited in place.
///
/// # Errors
///
/// Returns [`HarnessError::Write`] if the tag already exists, or if the files
/// cannot be written.
pub fn write_pass(
    root: &Path,
    tag: &str,
    manifest: &Manifest,
    records: &[Record],
) -> Result<PathBuf, HarnessError> {
    let dir = root.join(tag);
    if dir.exists() {
        return Err(HarnessError::Write(format!(
            "the pass `{tag}` already exists at {}. A re-measurement writes a new \
             tag beside it; nothing here is ever edited in place. Set HS_TAG to a \
             name that is free, or delete that directory deliberately.",
            dir.display()
        )));
    }
    std::fs::create_dir_all(&dir).map_err(|e| HarnessError::Write(e.to_string()))?;

    let mut body = String::new();
    let head = serde_json::to_string(manifest).map_err(|e| HarnessError::Write(e.to_string()))?;
    let _ = writeln!(body, "{head}");
    for record in records {
        let line = serde_json::to_string(record).map_err(|e| HarnessError::Write(e.to_string()))?;
        let _ = writeln!(body, "{line}");
    }
    let path = dir.join("records.ndjson");
    std::fs::write(&path, body).map_err(|e| HarnessError::Write(e.to_string()))?;
    Ok(path)
}

// ---------------------------------------------------------------------------
// The environment block
// ---------------------------------------------------------------------------

/// Captures the conditions a pass ran under.
///
/// Every field here is a condition of the number, not metadata about it: a
/// figure without them is an estimate wearing a decimal point.
#[must_use]
pub fn environment(run_tag: &str, workspace: &Path) -> Manifest {
    Manifest {
        schema_version: SCHEMA_VERSION,
        kind: "manifest".to_owned(),
        run_tag: run_tag.to_owned(),
        rustc: command("rustc", &["-vV"]).replace('\n', "; "),
        cargo: command("cargo", &["-V"]),
        git_rev: command("git", &["rev-parse", "HEAD"]),
        git_dirty: measured_tree_is_dirty(workspace),
        profile: if cfg!(debug_assertions) {
            "debug".to_owned()
        } else {
            "release".to_owned()
        },
        os: std::env::consts::OS.to_owned(),
        cpu: cpu_model(),
        logical_cpus: std::thread::available_parallelism().map_or(0, std::num::NonZeroUsize::get),
        ram_bytes: 0,
        features: "unstable-projection,memory,json".to_owned(),
        started_at: command("git", &["log", "-1", "--format=%cI"]),
        projection_store: PROJECTION_STORE.to_owned(),
    }
}

/// Whether the **measured** tree has uncommitted changes.
///
/// `crates/`, the workspace manifest and the lockfile — not the harness's own
/// directory. What makes a number irreproducible is the library moving under it;
/// the experiment's README gaining a paragraph does not.
fn measured_tree_is_dirty(workspace: &Path) -> bool {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(workspace)
        .args([
            "status",
            "--porcelain",
            "--",
            "crates",
            "Cargo.toml",
            "Cargo.lock",
        ])
        .output();
    match out {
        Ok(out) => !String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        Err(_) => true,
    }
}

fn command(program: &str, args: &[&str]) -> String {
    std::process::Command::new(program)
        .args(args)
        .output()
        .map_or_else(
            |_| "unknown".to_owned(),
            |out| String::from_utf8_lossy(&out.stdout).trim().to_owned(),
        )
}

fn cpu_model() -> String {
    for key in ["PROCESSOR_IDENTIFIER", "CPU_MODEL"] {
        if let Ok(value) = std::env::var(key)
            && !value.trim().is_empty()
        {
            return value.trim().to_owned();
        }
    }
    if let Ok(info) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in info.lines() {
            if let Some(rest) = line.strip_prefix("model name") {
                return rest.trim_start_matches([':', ' ']).trim().to_owned();
            }
        }
    }
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

// ---------------------------------------------------------------------------
// A future driver, so the harness takes no async runtime
// ---------------------------------------------------------------------------

/// Drives one future to completion on this thread.
///
/// Hand-written rather than pulling in an async runtime: the harness is a
/// single-threaded sweep, and a runtime in the dependency graph of a *measuring*
/// crate is one more thing between the number and the code it is about.
pub fn block_on<F: Future>(future: F) -> F::Output {
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    struct Park(std::thread::Thread);
    impl Wake for Park {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    let waker = Waker::from(Arc::new(Park(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::park(),
        }
    }
}
