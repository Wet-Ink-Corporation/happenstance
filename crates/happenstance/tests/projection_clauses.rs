//! The counts four `PS` clause verdicts rest on, executed.
//!
//! A verdict that says *"nothing calls this"* is a claim about the tree at one
//! instant. Written into `spec/SPECIFICATION.md` and checked nowhere, it decays
//! into the thing this project exists to prevent: a marker whose condition has
//! already changed, with no instrument able to notice
//! (`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`).
//!
//! So each test below pins **both halves** — the verdict text in the clause and
//! the fact about the tree it was derived from. Grow the runner a fan-out path
//! and the clause that says there is none goes red; take away the projection's
//! failure policy and PS-27's, which now pins its presence, goes red too.
//! Revert a clause to an unevaluated marker and the same test goes red from
//! the other side. Neither half can drift without the other noticing.
//!
//! No conformance rule is added, and that is the correct answer: PS-33 is a
//! phase gate no adapter can fail, and CF-36 forbids a clause backed only by
//! integration-level cases from naming an adapter rule.
#![cfg(all(feature = "unstable-projection", feature = "memory", feature = "json"))]
// A test target may unwrap; the library under test is held to
// `unwrap_used = "deny"` regardless.
#![allow(clippy::unwrap_used)]

use core::num::NonZeroUsize;
use std::collections::BTreeMap;
use std::panic::AssertUnwindSafe;
use std::path::{Path, PathBuf};

use happenstance::bytes::Bytes;
use happenstance::{
    Checkpoint, Chunk, Codec, CodecError, Delivered, DomainEvent, Event, EventStore, EventType,
    Json, MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, Projection, ProjectionId, ProjectionStore, ResetError, Tags,
    checkpoint_id, run_fan_out, run_projection,
};
use happenstance_testkit::block_on;

// ---------------------------------------------------------------------------
// Reading the tree the verdicts were counted over
// ---------------------------------------------------------------------------

fn workspace() -> PathBuf {
    // The same two-level climb `manifest_contract.rs` already makes to reach
    // `deny.toml` and the workspace manifest.
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(workspace().join(rel))
        .unwrap_or_else(|e| panic!("reading {rel}: {e}"))
        .replace("\r\n", "\n")
}

/// A source file with every `//`, `///` and `//!` comment removed.
///
/// Load-bearing rather than tidy. `run_projection`'s own page names
/// `on_error: SkipPolicy` and `tokio::spawn` — that is where it states which
/// alternatives lost — so a `contains` over the raw text would find the words
/// the documentation uses to say they are absent and conclude they are present.
fn code(rel: &str) -> String {
    read(rel)
        .lines()
        .map(|line| match line.find("//") {
            Some(at) => line[..at].to_owned(),
            None => line.to_owned(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The body of one `PS-nn` clause: its sentence, marker, rule and rejects, up
/// to the next clause heading.
fn clause(id: &str) -> String {
    let spec = read("spec/SPECIFICATION.md");
    let start = spec
        .find(&format!("**{id} —"))
        .unwrap_or_else(|| panic!("`{id}` is not a clause in the specification"));
    let rest = &spec[start + 2..];
    let end = rest.find("\n\n**PS-").or_else(|| rest.find("\n\n### "));
    let body = match end {
        Some(at) => &rest[..at],
        None => rest,
    };
    format!("**{body}")
}

/// Every module-level `pub` free function in the contract crate.
///
/// Module level matters: a `pub fn` inside an `impl` is a method and cannot be
/// the pump ADR-0007 allocates, which is a free function taking the store.
fn contract_free_functions() -> Vec<String> {
    let mut out = Vec::new();
    let dir = workspace().join("crates/happenstance-core/src");
    for entry in std::fs::read_dir(&dir).expect("the contract crate's src/ is readable") {
        let path = entry.expect("a readable entry").path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("valid utf-8");
        for line in source.lines() {
            // Column zero: anything indented is inside an `impl`, a trait or a
            // `mod tests`, and none of those is a free function.
            let Some(rest) = line
                .strip_prefix("pub async fn ")
                .or_else(|| line.strip_prefix("pub fn "))
                .or_else(|| line.strip_prefix("pub const fn "))
            else {
                continue;
            };
            out.push(
                rest.chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect(),
            );
        }
    }
    out.sort();
    out
}

// ---------------------------------------------------------------------------
// One domain, one projection, and one that panics
// ---------------------------------------------------------------------------

const SHREDDED: EventType = EventType::from_static("PartShredded");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Part {
    Shredded { vin: String },
}

impl DomainEvent for Part {
    const EVENT_TYPES: &'static [EventType] = &[SHREDDED];

    fn event_type(&self) -> EventType {
        SHREDDED
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// Counts shredded parts. `panic_on` makes it the PS-30 subject.
#[derive(Debug)]
struct Shreds {
    id: ProjectionId,
    scope: Tags,
    counts: BTreeMap<String, u64>,
    panic_on: Option<String>,
}

impl Shreds {
    fn new(panic_on: Option<&str>) -> Self {
        Self {
            id: ProjectionId::from_static("shreds"),
            scope: Tags::empty(),
            counts: BTreeMap::new(),
            panic_on: panic_on.map(ToOwned::to_owned),
        }
    }
}

impl Projection for Shreds {
    type Event = Part;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Part>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        let Part::Shredded { vin } = event.into_event();
        assert!(
            self.panic_on.as_deref() != Some(vin.as_str()),
            "the PS-30 subject: `apply` panics part-way through a chunk"
        );
        let count = self.counts.entry(vin.clone()).or_insert(0);
        *count += 1;
        batch.write(vin, *count);
        Ok(())
    }
}

fn chunk(n: usize) -> Chunk {
    Chunk::of(NonZeroUsize::new(n).expect("a non-zero chunk"))
}

fn shredded(vin: &str) -> Event {
    let payload = Json
        .encode(&Part::Shredded {
            vin: vin.to_owned(),
        })
        .expect("the fixture encodes");
    Event::new(SHREDDED, payload).expect("a valid event type")
}

/// An event of a declared type whose payload no codec here can read.
fn poisoned() -> Event {
    Event::new(SHREDDED, &b"shredded, and so is the payload"[..]).expect("a valid event type")
}

// ---------------------------------------------------------------------------
// PS-33 — ADR-0007's falsifier, counted
// ---------------------------------------------------------------------------

#[test]
fn no_checkpoint_pump_exists_in_the_contract_crate() {
    // The count, taken rather than recalled. ADR-0007 allocates a checkpoint
    // pump to `happenstance-core`; the contract crate publishes exactly two
    // free functions and neither of them is one.
    assert_eq!(
        contract_free_functions(),
        vec!["collect".to_owned(), "read_decision_model".to_owned()],
        "the contract crate grew a free function: if it is a checkpoint pump, \
         PS-33's verdict is now wrong and needs recounting rather than editing"
    );

    // And nothing in the workspace's Rust code calls one, because there is
    // none to call. The word survives in this crate only where the runner's
    // own page explains why the pump signature could not have worked.
    assert!(
        !code("crates/happenstance-core/src/projection.rs").contains("pump"),
        "the port grew a pump; PS-33's count is stale"
    );

    // The verdict, in the clause.
    let ps33 = clause("PS-33");
    assert!(
        ps33.contains("[NON-NORMATIVE"),
        "PS-33 still carries an unevaluated maturity marker: {ps33}"
    );
    assert!(
        !ps33.contains("[DEFERRED"),
        "PS-33 is still deferred after the phase that owned its evaluation exited"
    );
    // Branch two of exactly two admissible forms: the superseding ADR.
    //
    // This half used to pin the STAGED file, `.kb/_intake/0032-adr-0031-…`, and
    // that was a fact with an expiry date: staging exists to be consumed, so the
    // assertion went red on 2026-08-17 the moment the ingest wave it was waiting
    // for succeeded and cleared `_intake`. A test that fails when the thing it
    // wants finally happens is pinned to the wrong half — the module doc above
    // asks for the fact the verdict was derived from, and that fact is the
    // ACCEPTED atom, which is durable. Nothing is weakened by the move: an atom
    // in `.kb/decisions/` at `status: accepted` is strictly more than a draft
    // sitting in `_intake`, which is what blocked this verdict for four waves.
    let atom = "0031-the-runner-collapses-upward";
    assert!(
        ps33.contains(atom),
        "PS-33's verdict cites no superseding ADR: {ps33}"
    );
    let rel = format!(".kb/decisions/{atom}.md");
    assert!(
        workspace().join(&rel).is_file(),
        "PS-33's verdict cites a superseding ADR that is not in `.kb/decisions/`"
    );
    assert!(
        read(&rel).contains("status: accepted"),
        "PS-33's superseding ADR exists but is not accepted, so the falsifier's \
         verdict is recorded against a decision nobody has signed"
    );
}

// ---------------------------------------------------------------------------
// PS-27 — skip-and-record, offered
// ---------------------------------------------------------------------------

#[test]
fn the_failure_policy_is_the_projections_and_halts_by_default() {
    // The seam is on the trait, provided, and its default halts. Comments
    // stripped first: the runner's own page names the rejected spelling in
    // order to say it lost.
    let runner = code("crates/happenstance/src/runner.rs");
    for present in ["fn on_error(", "Ok(Policy::Halt)"] {
        assert!(
            runner.contains(present),
            "the runner lost `{present}`; PS-27's verdict is now wrong"
        );
    }
    // And it is not the loop's: no policy type and no sixth argument, which
    // is PS-26's *Rejects* executed.
    assert!(
        !runner.contains("SkipPolicy"),
        "the runner grew a per-run `SkipPolicy`; PS-26 rejects it"
    );
    let signature = runner
        .split("pub async fn run_projection")
        .nth(1)
        .and_then(|rest| rest.split(") ->").next())
        .expect("`run_projection` is declared in `runner.rs`");
    assert_eq!(
        signature.matches(": ").count(),
        5,
        "`run_projection` grew a parameter; a run-level failure policy is \
         the alternative PS-26 rejects: {signature}"
    );

    // And the behaviour: `Shreds` does not override `on_error`, so a poisoned
    // event halts the run. A skipping default would have advanced the
    // checkpoint *past* it; that does not happen.
    let events = MemoryEventStore::new();
    let models = MemoryProjectionStore::new();
    block_on(async {
        EventStore::append(
            &events,
            &[shredded("VIN1"), poisoned(), shredded("VIN2")],
            None,
        )
        .await
        .expect("the fixture seeds");

        let mut shreds = Shreds::new(None);
        let error = run_projection(&events, &models, &mut shreds, &Json, chunk(1))
            .await
            .expect_err("the poisoned event stops the run");

        let poison = events.snapshot()[1].position;
        assert_eq!(error.position(), Some(poison));

        // The checkpoint sits at the last GOOD position, not past the poison.
        let good = events.snapshot()[0].position;
        assert_eq!(
            models
                .checkpoint(&checkpoint_id(&shreds).expect("a derivable id"))
                .await
                .unwrap(),
            Checkpoint::Live { through: good },
            "the checkpoint advanced past the poisoned event under the \
             default policy, which is a skip nobody asked for"
        );
        // The third event was never reached: a skipping default would have
        // applied it.
        assert_eq!(models.get("VIN2"), None);
    });

    // The clause says the seam exists and names its rule. Freezing it is
    // #127's, so the marker word stays.
    let ps27 = clause("PS-27");
    assert!(
        ps27.contains("[DEFERRED"),
        "PS-27's marker moved; freezing it is #127's, with its ledger row: {ps27}"
    );
    for required in ["on_error", "skip_and_record_is_atomic", "halt", "HS-P0010"] {
        assert!(
            ps27.contains(required),
            "PS-27's verdict does not state `{required}`: {ps27}"
        );
    }
    assert!(
        !ps27.contains("the count is zero"),
        "PS-27 still says no skip-and-record path exists: {ps27}"
    );
}

// ---------------------------------------------------------------------------
// PS-30 — the fan-out runner, counted
// ---------------------------------------------------------------------------

#[test]
fn no_fan_out_runner_catches_a_panic() {
    // No task, no unwind catcher, anywhere in the typed layer's own code.
    for file in [
        "crates/happenstance/src/runner.rs",
        "crates/happenstance/src/lib.rs",
        // #112's fan-out runner, which catches nothing either (O1, option B).
        "crates/happenstance/src/fan_out.rs",
    ] {
        let source = code(file);
        for absent in ["catch_unwind", "AssertUnwindSafe", "spawn("] {
            assert!(
                !source.contains(absent),
                "{file} grew `{absent}`; PS-30's verdict is now wrong"
            );
        }
    }

    // And the behaviour the absence produces: the panic is *not* caught, and
    // an uncommitted batch takes the half-applied rows down with it.
    let events = MemoryEventStore::new();
    let models = MemoryProjectionStore::new();
    block_on(async {
        EventStore::append(&events, &[shredded("VIN1"), shredded("BOOM")], None)
            .await
            .expect("the fixture seeds");
    });

    let unwound = std::panic::catch_unwind(AssertUnwindSafe(|| {
        block_on(async {
            let mut shreds = Shreds::new(Some("BOOM"));
            // One chunk over both events, so the first is applied into the
            // batch the second one panics inside.
            let _ = run_projection(&events, &models, &mut shreds, &Json, chunk(8)).await;
        });
    }));
    assert!(
        unwound.is_err(),
        "the runner caught the panic, so it is doing the fan-out runner's job"
    );

    // The id the runner would have committed under. The bare name reads
    // `NeverRun` whatever the runner did, which would make this vacuous.
    let key = checkpoint_id(&Shreds::new(Some("BOOM"))).expect("a derivable id");
    block_on(async {
        assert!(
            models.is_empty(),
            "a panicking `apply` left rows behind: the batch outlived its chunk"
        );
        assert_eq!(
            models.checkpoint(&key).await.unwrap(),
            Checkpoint::NeverRun,
            "a panicking `apply` moved the checkpoint"
        );
    });

    fan_out_does_not_catch_a_panic_either();

    let ps30 = clause("PS-30");
    assert!(
        ps30.contains("[DEFERRED"),
        "PS-30 still reads as provisional after its evaluation exit: {ps30}"
    );
    // `#112` is the build that left the MUST binding only a catching runner.
    for required in ["polling-cost", "CF-34", "#112"] {
        assert!(
            ps30.contains(required),
            "PS-30's verdict does not name `{required}` as its contingency: {ps30}"
        );
    }
}

/// The same two-event fixture through `run_fan_out`, over a quiet view and
/// the panicking one: the unwind escapes, and neither view keeps a row or a
/// moved checkpoint, because both write sets were open when it began.
fn fan_out_does_not_catch_a_panic_either() {
    let events = MemoryEventStore::new();
    let models = MemoryProjectionStore::new();
    block_on(async {
        EventStore::append(&events, &[shredded("VIN1"), shredded("BOOM")], None)
            .await
            .expect("the fixture seeds");
    });
    let quiet = || Shreds {
        id: ProjectionId::from_static("quiet"),
        ..Shreds::new(None)
    };
    let mut views = [quiet(), Shreds::new(Some("BOOM"))];

    // Test-only: the assertion is that the panic escapes, so it is stopped
    // here, outside the runner. Only the fresh stores are read afterwards.
    let unwound = std::panic::catch_unwind(AssertUnwindSafe(|| {
        block_on(async {
            let _ran = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;
        });
    }));
    assert!(
        unwound.is_err(),
        "the fan-out runner caught the panic; PS-30's verdict is now wrong"
    );
    let keys = [quiet(), Shreds::new(Some("BOOM"))]
        .map(|view| checkpoint_id(&view).expect("a derivable id"));
    block_on(async {
        assert!(
            models.is_empty(),
            "a panicking view left rows behind in a fan-out run"
        );
        for key in &keys {
            assert_eq!(
                models.checkpoint(key).await.unwrap(),
                Checkpoint::NeverRun,
                "a fan-out panic moved {key:?}'s checkpoint"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// PS-18 — reset refusal, counted
// ---------------------------------------------------------------------------

#[test]
fn reset_refusal_has_a_mechanism_and_an_adapter() {
    // The subject exists — which it did not when this story was specified.
    // Naming the variant is the compile-level half of the count.
    let refused: ResetError<MemoryProjectionStoreError> = ResetError::Refused;
    assert!(matches!(refused, ResetError::Refused));

    // The rule exists too, and is registered in the projection suite.
    let suite = read("crates/happenstance-testkit/src/projection.rs");
    assert!(
        suite.contains("pub async fn refused_reset_changes_nothing"),
        "PS-18's named rule has vanished from the testkit"
    );

    // The reference fixture still declines, with the store's own reason:
    // `MemoryProjectionStore` holds no protection.
    let fixtures = read("crates/happenstance-testkit/src/fixtures.rs");
    assert!(
        fixtures.contains("const RESET_REFUSAL: Capability = Capability::declined("),
        "the reference fixture now claims RESET_REFUSAL; PS-18's count has \
         changed and the verdict needs retaking"
    );

    // The adapter is `happenstance-sqlite` (#118): its fixture declares the
    // capability, so the rule runs there rather than skipping, and PS-18 and
    // the fixture clause it leans on (CF-43) both say so.
    let sqlite = read("crates/happenstance-sqlite/tests/projection.rs");
    assert!(
        sqlite.contains("const RESET_REFUSAL: Capability = Capability::SUPPORTED;"),
        "happenstance-sqlite's fixture no longer declares RESET_REFUSAL, so PS-18 \
         has no adapter again and its text needs retaking"
    );
    assert!(
        read("spec/SPECIFICATION.md").contains("**CF-43.** A projection fixture declaring"),
        "CF-43, the obligation a fixture takes on by declaring RESET_REFUSAL, \
         is missing from the specification"
    );

    let ps18 = clause("PS-18");
    assert!(
        ps18.contains("happenstance-sqlite"),
        "PS-18 does not name the adapter that implements it: {ps18}"
    );
    assert!(
        ps18.contains("[DEFERRED"),
        "PS-18 still reads as provisional after its evaluation exit: {ps18}"
    );
    assert!(
        ps18.contains("HS-P0010"),
        "PS-18's exclusion names no owner for the eventual count: {ps18}"
    );
}

// ---------------------------------------------------------------------------
// PS-32, PS-33, PS-35 — out of the clause space, IDs retained
// ---------------------------------------------------------------------------

#[test]
fn three_ids_are_retained_rather_than_deleted() {
    let spec = read("spec/SPECIFICATION.md");
    for id in ["PS-32", "PS-33", "PS-35"] {
        assert!(
            spec.contains(&format!("**{id} —")),
            "`{id}` was deleted rather than retained; every citation to it now \
             resolves to nothing"
        );
        let body = clause(id);
        assert!(
            body.contains("[NON-NORMATIVE"),
            "`{id}` did not leave the clause space: {body}"
        );
    }

    // CF-30 is the worked example the three were moved to match, and it must
    // still be there to be one.
    assert!(
        spec.contains("CF-30 is the worked example"),
        "the pattern these three follow is no longer documented"
    );
}
