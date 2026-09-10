//! A log that outlived both its encoding and its schema.
//!
//! Every other example in this workspace writes one encoding and one event
//! shape, which is the arrangement no system that has been deployed twice is
//! ever in. This one writes JSON readings in whole degrees, upgrades, and then
//! writes postcard readings in thousandths — into the same log, on the same
//! file, read afterwards by one fold that branches on neither.
//!
//! Run with `cargo run -p telemetry-across-codecs`.
//!
//! # The two axes, and why they are different problems
//!
//! 1. **The encoding changed.** ADR-0021 puts a codec tag in a framing region
//!    at the front of `Event::metadata`, which no store parses and no adapter
//!    has to understand. A payload is decoded with the codec its own tag
//!    names, resolved against the codecs the *build* carries — so a caller
//!    holding `Json` folds a postcard event without branching, and without
//!    knowing that it did. Nothing in this file passes two codecs anywhere.
//! 2. **The event shape changed.** That the tag cannot help with, because the
//!    bytes decode perfectly into a type that means something else. It is
//!    handled the way it has always been handled — a new event *type* beside
//!    the old one, the old type kept as a wire format, and an upcast in
//!    `DomainEvent::decode`, which is handed the `EventType` precisely so it
//!    can branch on it. This is the one example where that parameter is not
//!    `_event_type`.
//!
//! # The hazard this file is built around
//!
//! A query is derived from `EVENT_TYPES` crossed with a model's scope. So the
//! **old event type must stay in `EVENT_TYPES` forever**, even though nothing
//! writes it any more: delete the name and the derived query stops nominating
//! the old events, every fold silently loses its history, and nothing in the
//! compiler, the linter or the conformance suite has an opinion about it. The
//! name is what keeps the past inside the boundary.
//!
//! # What this does not demonstrate
//!
//! It does not rewrite the log, and nothing here is a migration in the sense
//! of a backfill: the old events keep their old bytes and their old type name
//! for as long as the store holds them. Nor is the postcard payload's size
//! advantage a benchmark — two payloads are printed because the difference is
//! the reason anybody does this, and `experiments/` is where this repository
//! measures things.

#![allow(clippy::print_stdout, reason = "the transcript is the point")]

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use happenstance::bytes::Bytes;
use happenstance::{
    Boundary, Codec, CodecError, CommandError, DecisionModel, DomainEvent, EventStore, EventType,
    InvalidTag, Json, Postcard, Query, ReadOptions, Retry, SequencedEvent, Tag, Tags, collect,
    commit, commit_with, read_decision_model,
};
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use serde::{Deserialize, Serialize};

/// How many times a command may be attempted before it gives up.
///
/// Spelled at every call site for the reason `course-subscriptions` gives: a
/// loop whose only exit is success is a hang with better manners. This run is
/// single-writer, so the bound is never spent.
const ATTEMPTS: u32 = 3;

/// The device this run reports for.
const DEVICE: &str = "d-1";

/// The coldest reading this application will accept, in thousandths of a degree.
const MIN_MILLIDEGREES: i32 = -60_000;

/// The hottest reading this application will accept, in thousandths of a degree.
const MAX_MILLIDEGREES: i32 = 150_000;

/// How many thousandths there are in a degree.
///
/// Named rather than spelled `1000` inside the upcast, because it is the whole
/// content of the schema change and a reader should be able to find it.
const PER_DEGREE: i32 = 1_000;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let path = database_path();
    remove_database(&path);

    println!("== the database this run writes to ==");
    println!("   {}", path.display());

    // The run is a function so every store and connection it builds is dropped
    // before the file is removed — on Windows an open handle is what makes the
    // cleanup silently fail and the next run start on the tail of this one.
    let outcome = run(&path).await;
    remove_database(&path);
    outcome
}

/// Everything this program does, against one file.
async fn run(path: &Path) -> Result<()> {
    let events = SqliteEventStore::open(path)?;

    the_version_that_shipped_first(&events).await?;
    the_upgrade(&events).await?;
    one_log_two_encodings(&events).await?;
    one_fold_reads_both(&events).await?;

    println!("\n== final log ==");
    print_log(&events).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// The four things this program shows
// ---------------------------------------------------------------------------

/// Writes the log the old binary would have written.
///
/// It uses `commit`, which is `commit_with` with `Json` already chosen, and a
/// domain type that has since been superseded. Nothing about this section
/// knows that anything is going to change, which is the point — the events it
/// writes are the ones a real deployment is stuck with.
async fn the_version_that_shipped_first(events: &SqliteEventStore) -> Result<()> {
    println!("\n== the version that shipped first ==");

    register(events, DEVICE, "1.4.0").await?;
    for celsius in [21, 23, 22] {
        report_celsius(events, DEVICE, celsius).await?;
    }
    println!(
        "   registered {DEVICE} on firmware 1.4.0 and wrote 3 readings, in whole degrees, as JSON"
    );

    match report_celsius(events, "d-9", 20).await {
        Ok(()) => bail!("a reading from an unregistered device should have been refused"),
        Err(err) => println!("   refused: {err}"),
    }

    Ok(())
}

/// Upgrades the firmware and starts writing the new shape, in the new encoding.
///
/// Both changes land here at once because that is how they arrive in practice
/// — a deployment changes what it writes and how it writes it in the same
/// release — and keeping them together is what makes the next two sections
/// have anything to separate.
async fn the_upgrade(events: &SqliteEventStore) -> Result<()> {
    println!("\n== the upgrade ==");

    upgrade_firmware(events, DEVICE, "2.0.0").await?;
    for millidegrees in [22_500, 21_250] {
        report_millidegrees(events, DEVICE, millidegrees).await?;
    }
    println!("   firmware 2.0.0, and 2 readings in thousandths of a degree, as postcard");

    match report_millidegrees(events, DEVICE, 900_000).await {
        Ok(()) => bail!("an implausible reading should have been refused"),
        Err(err) => println!("   refused: {err}"),
    }

    Ok(())
}

/// Prints what each event was written with, and how big its payload is.
///
/// The codec tag is read out of the framing region **only to print it**. No
/// application needs to do this and no adapter may: ADR-0021's whole point is
/// that the region is opaque to the store, and the decode path below resolves
/// it without anybody here looking. It is parsed here for the same reason a
/// debugger exists.
async fn one_log_two_encodings(events: &SqliteEventStore) -> Result<()> {
    println!("\n== one log, two encodings ==");

    let log = collect(events.read(&Query::all(), ReadOptions::new())).await?;
    let mut seen = Vec::new();

    for event in &log {
        let codec = framed_codec(event).unwrap_or_else(|| "unframed".to_owned());
        println!(
            "   {:>3}  {:<22} {:<8} {} bytes",
            event.position.get(),
            event.event_type().as_str(),
            codec,
            event.event.data().len()
        );
        if !seen.contains(&codec) {
            seen.push(codec);
        }
    }

    if seen.len() < 2 {
        bail!("the log holds only {seen:?}, so there are no two encodings to read across");
    }
    println!("   two encodings and two temperature types, in one store, side by side");

    Ok(())
}

/// Folds the whole device history with one codec in hand.
///
/// The section the other three exist for. `Json` is the only codec named at
/// this call site; the postcard readings arrive decoded anyway, because the
/// framing region says what they were written with. The v1 readings arrive
/// *upcast*, because `Telemetry::decode` was handed their event type and knew
/// what to do with it.
async fn one_fold_reads_both(events: &SqliteEventStore) -> Result<()> {
    println!("\n== one fold reads both ==");

    let device = DeviceId::new(DEVICE)?;
    let mut readings = Readings::new(&device)?;

    let query = readings.query()?;
    let (found, _) = read_decision_model(events, &query).await?;
    for event in &found {
        readings.absorb(event, &Json)?;
    }

    println!(
        "   the derived query nominated {} event(s), and one codec was named here: Json",
        found.len()
    );
    println!(
        "   firmware:   {}",
        readings.firmware.as_deref().unwrap_or("unknown")
    );
    println!("   readings:   {:?}", readings.millidegrees);

    // The upcast is the assertion. 21 degrees was written by a binary that had
    // never heard of a thousandth, and it is 21000 here without anything in
    // this function converting it.
    if !readings.millidegrees.contains(&(21 * PER_DEGREE)) {
        bail!(
            "the first JSON reading did not arrive upcast: {:?}",
            readings.millidegrees
        );
    }
    println!(
        "   21 whole degrees, written by a binary with no concept of a\n   \
         thousandth, arrives as {} — the upcast is in `decode`, and the fold\n   \
         below it has one temperature variant and no idea there were ever two",
        21 * PER_DEGREE
    );

    Ok(())
}

// ---------------------------------------------------------------------------
// Identity, validated once at the edge
// ---------------------------------------------------------------------------

/// A device, validated once into the tag its events carry.
///
/// The resolution the other examples reach, and for the same reason:
/// `DomainEvent::tags` is infallible while `Tag::key_value` can refuse, so a
/// domain type whose tags come from runtime values holds the validated form
/// instead of rebuilding it on every call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct DeviceId {
    /// The identifier as the application wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

impl DeviceId {
    /// Validates `id` into the tag every event for this device is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("device", id)?,
        })
    }

    /// The identifier as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for DeviceId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<DeviceId> for String {
    fn from(device: DeviceId) -> Self {
        device.id
    }
}

// ---------------------------------------------------------------------------
// The domain, in the shape the old binary had
// ---------------------------------------------------------------------------

/// The event set the previous version of this application shipped.
///
/// **Kept rather than deleted, and it is not dead code.** It is the wire
/// format of every reading written before the upgrade, and `Telemetry::decode`
/// still reaches for it — an upcast is "decode with the type that wrote these
/// bytes, then map forward", and the type that wrote them has to still exist
/// for that sentence to be executable.
///
/// It writes two event types. `Telemetry` declares four, because it must
/// nominate these two as well as its own.
#[derive(Debug, Serialize, Deserialize)]
enum Legacy {
    /// A device came online. The same shape the new version writes.
    DeviceRegistered {
        /// The device now known.
        device: DeviceId,
        /// What it was running.
        firmware: String,
    },
    /// A reading, in whole degrees Celsius.
    TemperatureReported {
        /// Which device reported.
        device: DeviceId,
        /// The reading, in whole degrees.
        celsius: i32,
    },
}

/// The four names this log has ever carried, declared once.
///
/// Two of them mean the same *reading* at different schema versions, and that is
/// the whole point of this example — so they are named rather than indexed.
/// Indexing the declaration list gave two subscripts that differ by one
/// character and mean "what is written now" and "what may still be read";
/// `TEMPERATURE_REPORTED_V2` and `TEMPERATURE_REPORTED` cannot be confused by a
/// reader, or reordered into a lie by an editor.
const DEVICE_REGISTERED: EventType = EventType::from_static("DeviceRegistered");
/// See [`DEVICE_REGISTERED`].
const FIRMWARE_UPGRADED: EventType = EventType::from_static("FirmwareUpgraded");
/// The **v1** reading. Read-only: nothing writes this name any more.
const TEMPERATURE_REPORTED: EventType = EventType::from_static("TemperatureReported");
/// The **v2** reading, and what a reading written now carries.
const TEMPERATURE_REPORTED_V2: EventType = EventType::from_static("TemperatureReportedV2");

impl DomainEvent for Legacy {
    const EVENT_TYPES: &'static [EventType] = &[DEVICE_REGISTERED, TEMPERATURE_REPORTED];

    fn event_type(&self) -> EventType {
        match self {
            Self::DeviceRegistered { .. } => DEVICE_REGISTERED,
            Self::TemperatureReported { .. } => TEMPERATURE_REPORTED,
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::DeviceRegistered { device, .. } | Self::TemperatureReported { device, .. } => {
                [device.tag.clone()].into_iter().collect()
            }
        }
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

// ---------------------------------------------------------------------------
// The domain, as it is now
// ---------------------------------------------------------------------------

/// Everything that can happen to a device, in this version's vocabulary.
///
/// One temperature variant, in thousandths. The fold does not know there were
/// ever two shapes, and that is the whole reason the upcast lives in `decode`
/// rather than in every `apply` that has to deal with a reading.
#[derive(Debug, Serialize, Deserialize)]
enum Telemetry {
    /// A device came online.
    DeviceRegistered {
        /// The device now known.
        device: DeviceId,
        /// What it was running.
        firmware: String,
    },
    /// A device's firmware changed.
    FirmwareUpgraded {
        /// The device upgraded.
        device: DeviceId,
        /// What it is running now.
        firmware: String,
    },
    /// A reading, in thousandths of a degree.
    TemperatureReported {
        /// Which device reported.
        device: DeviceId,
        /// The reading, in thousandths of a degree.
        millidegrees: i32,
    },
}

impl DomainEvent for Telemetry {
    /// Four names for three variants.
    ///
    /// `TemperatureReported` is the **old** name and nothing writes it any
    /// more. It is declared because the query is derived from this list, so
    /// removing it would stop every boundary nominating the readings written
    /// before the upgrade — silently, and with no test, lint or conformance
    /// rule that has an opinion. The list is where a log's past is kept alive.
    const EVENT_TYPES: &'static [EventType] = &[
        DEVICE_REGISTERED,
        FIRMWARE_UPGRADED,
        TEMPERATURE_REPORTED,
        TEMPERATURE_REPORTED_V2,
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::DeviceRegistered { .. } => DEVICE_REGISTERED,
            Self::FirmwareUpgraded { .. } => FIRMWARE_UPGRADED,
            // The v2 name, never the v1 one. A reading written now is a v2
            // reading; the v1 name is read-only. This used to be a subscript
            // into the list above, carried by a comment saying "index 3, never
            // index 2" — the comment a named constant makes unnecessary.
            Self::TemperatureReported { .. } => TEMPERATURE_REPORTED_V2,
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::DeviceRegistered { device, .. }
            | Self::FirmwareUpgraded { device, .. }
            | Self::TemperatureReported { device, .. } => {
                [device.tag.clone()].into_iter().collect()
            }
        }
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    /// Decodes a payload written for `event_type`, upcasting the old shape.
    ///
    /// **The one place in this workspace where that parameter is used.** The
    /// codec tag in the framing region has already decided *how* these bytes
    /// were encoded; what it cannot decide is what they meant, because a v1
    /// reading deserialises perfectly into a number that is a thousand times
    /// too small. Only the event type distinguishes them.
    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        // The named v1 constant, not the literal `"TemperatureReported"`: one
        // declaration, so there is no second spelling to get wrong.
        if *event_type == TEMPERATURE_REPORTED {
            return Ok(upcast(codec.decode::<Legacy>(data)?));
        }

        codec.decode(data)
    }
}

/// Maps an event written by the previous version into this one's vocabulary.
///
/// Total, and deliberately so: an upcast that can fail is a log that can stop
/// being readable, and there is nothing a caller could do about it at the
/// moment it happens. Whole degrees multiply into thousandths exactly, so the
/// conversion loses nothing — `i32` covers the plausible range this
/// application accepts several hundred times over.
fn upcast(old: Legacy) -> Telemetry {
    match old {
        Legacy::DeviceRegistered { device, firmware } => {
            Telemetry::DeviceRegistered { device, firmware }
        }
        Legacy::TemperatureReported { device, celsius } => Telemetry::TemperatureReported {
            device,
            millidegrees: celsius.saturating_mul(PER_DEGREE),
        },
    }
}

/// Why a decision refused.
///
/// Every variant carries the value the transcript prints — `900000 is outside
/// -60000..=150000` and never `invalid reading` — so a reader can act on the
/// refusal where it is printed.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    /// Nothing can be reported for a device nobody registered.
    #[error("device {device} is not registered")]
    NotRegistered {
        /// The unknown device.
        device: String,
    },
    /// A device registers once.
    #[error("device {device} is already registered")]
    AlreadyRegistered {
        /// The device already known.
        device: String,
    },
    /// A reading outside the range this application accepts.
    #[error("{millidegrees} is outside {MIN_MILLIDEGREES}..={MAX_MILLIDEGREES}")]
    Implausible {
        /// The reading that was refused.
        millidegrees: i32,
    },
}

// ---------------------------------------------------------------------------
// The decision models
// ---------------------------------------------------------------------------

/// One device's history, as the old binary saw it.
///
/// It exists to write the old log honestly. Its query is derived from
/// `Legacy::EVENT_TYPES`, which is the two names that version knew — which is
/// exactly the boundary the old binary had, and exactly why the new one has to
/// declare four.
#[derive(Debug, Clone)]
struct LegacyDevice {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// Whether the device has been registered.
    registered: bool,
}

impl LegacyDevice {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(device: &DeviceId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("device", device.as_str())])?,
            registered: false,
        })
    }
}

impl DecisionModel for LegacyDevice {
    type Event = Legacy;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Legacy::DeviceRegistered { .. } => self.registered = true,
            // A reading changes nothing this model decides on. The arm exists
            // because the fold is exhaustive over the enum, and that
            // exhaustiveness is the whole guarantee.
            Legacy::TemperatureReported { .. } => {}
        }
    }
}

/// One device's history, in this version's vocabulary.
///
/// Its `millidegrees` holds every reading the boundary nominated, from both
/// eras, in one unit. There is no branch in `apply` that knows about the
/// upgrade, because by the time an event reaches `apply` it has already been
/// upcast.
#[derive(Debug, Clone)]
struct Readings {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// Whether the device has been registered.
    registered: bool,
    /// What it is running now, once anything has said.
    firmware: Option<String>,
    /// Every reading the boundary nominated, in thousandths of a degree.
    millidegrees: Vec<i32>,
}

impl Readings {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(device: &DeviceId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("device", device.as_str())])?,
            registered: false,
            firmware: None,
            millidegrees: Vec::new(),
        })
    }
}

impl DecisionModel for Readings {
    type Event = Telemetry;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Telemetry::DeviceRegistered { firmware, .. } => {
                self.registered = true;
                self.firmware = Some(firmware);
            }
            Telemetry::FirmwareUpgraded { firmware, .. } => self.firmware = Some(firmware),
            Telemetry::TemperatureReported { millidegrees, .. } => {
                self.millidegrees.push(millidegrees);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The command handlers, in two generations
// ---------------------------------------------------------------------------

/// Registers a device, as the old binary did: `commit`, so JSON.
async fn register(events: &SqliteEventStore, device: &str, firmware: &str) -> Result<()> {
    let device = DeviceId::new(device)?;
    let boundary = LegacyDevice::new(&device)?;

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |known: &LegacyDevice| {
            if known.registered {
                return Err(Refusal::AlreadyRegistered {
                    device: device.id.clone(),
                });
            }
            Ok(vec![Legacy::DeviceRegistered {
                device: device.clone(),
                firmware: firmware.to_owned(),
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Reports a reading in whole degrees, as the old binary did: `commit`, so JSON.
async fn report_celsius(events: &SqliteEventStore, device: &str, celsius: i32) -> Result<()> {
    let device = DeviceId::new(device)?;
    let boundary = LegacyDevice::new(&device)?;

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |known: &LegacyDevice| {
            if !known.registered {
                return Err(Refusal::NotRegistered {
                    device: device.id.clone(),
                });
            }
            Ok(vec![Legacy::TemperatureReported {
                device: device.clone(),
                celsius,
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Records a firmware upgrade, in the new encoding.
async fn upgrade_firmware(events: &SqliteEventStore, device: &str, firmware: &str) -> Result<()> {
    let device = DeviceId::new(device)?;
    let boundary = Readings::new(&device)?;

    commit_with(
        events,
        boundary,
        &Postcard,
        Retry::attempts(ATTEMPTS.try_into()?),
        |known: &Readings| {
            if !known.registered {
                return Err(Refusal::NotRegistered {
                    device: device.id.clone(),
                });
            }
            Ok(vec![Telemetry::FirmwareUpgraded {
                device: device.clone(),
                firmware: firmware.to_owned(),
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Reports a reading in thousandths, in the new encoding.
///
/// `commit_with` and an explicit `&Postcard`. It is the ungated entry point —
/// `commit` is this function with `Json` already chosen — and naming the codec
/// here is the only difference between this handler and the one above it that
/// wrote JSON. The boundary it reads is a `Readings`, so this decision is taken
/// against a fold that has already absorbed the *JSON* events written before
/// the upgrade.
async fn report_millidegrees(
    events: &SqliteEventStore,
    device: &str,
    millidegrees: i32,
) -> Result<()> {
    let device = DeviceId::new(device)?;
    let boundary = Readings::new(&device)?;

    commit_with(
        events,
        boundary,
        &Postcard,
        Retry::attempts(ATTEMPTS.try_into()?),
        |known: &Readings| {
            if !known.registered {
                return Err(Refusal::NotRegistered {
                    device: device.id.clone(),
                });
            }
            if !(MIN_MILLIDEGREES..=MAX_MILLIDEGREES).contains(&millidegrees) {
                return Err(Refusal::Implausible { millidegrees });
            }
            Ok(vec![Telemetry::TemperatureReported {
                device: device.clone(),
                millidegrees,
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

/// The codec tag an event was framed with, read out of its metadata.
///
/// **Nothing needs this.** The framing region is documented as opaque to
/// stores and adapters, and the decode path resolves it without any caller
/// looking; it is parsed here so the transcript can show that two encodings
/// really are sitting in one log. Every step is fallible and none of it
/// panics: an event written by something that does not frame its payloads at
/// all is a legitimate thing to meet, and it renders as `unframed`.
fn framed_codec(event: &SequencedEvent) -> Option<String> {
    let metadata = event.event.metadata()?;
    let after_magic = metadata.strip_prefix(b"hpst\x01")?;
    let end = after_magic.iter().position(|byte| *byte == 0xFF)?;

    core::str::from_utf8(&after_magic[..end])
        .ok()
        .map(str::to_owned)
}

/// Prints every event in the store, in the order the store assigned.
async fn print_log(events: &SqliteEventStore) -> Result<()> {
    let log = collect(events.read(&Query::all(), ReadOptions::new())).await?;

    for event in log {
        println!(
            "   {:>3}  {:<22} {:?}",
            event.position.get(),
            event.event_type().as_str(),
            event.tags()
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// The file this run owns
// ---------------------------------------------------------------------------

/// Where this run's database lives.
///
/// Under the process id, so a `cargo run` and the test that spawns the binary
/// never meet on one file.
fn database_path() -> PathBuf {
    std::env::temp_dir().join(format!("happenstance-telemetry-{}.db", std::process::id()))
}

/// Removes the database and the two files WAL mode keeps beside it.
///
/// Deleting only the `.db` and leaving `-wal` behind is how a "fresh" run
/// starts on the tail of the last one.
fn remove_database(path: &Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut name = path.as_os_str().to_owned();
        name.push(suffix);
        drop(std::fs::remove_file(PathBuf::from(name)));
    }
}

/// Renders a command failure as the message the transcript prints.
///
/// `CommandError::Refused` carries the handler's *own* refusal, and that is
/// what a reader must see: `CommandError`'s own `Display` says "the decision
/// refused", which is a category rather than a value.
fn rejected(err: CommandError<SqliteEventStoreError, Refusal>) -> anyhow::Error {
    match err {
        CommandError::Refused(refusal) => refusal.into(),
        other => other.into(),
    }
}
