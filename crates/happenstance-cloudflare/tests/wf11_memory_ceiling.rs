//! WF-11's falsifier, taken to a machine where the memory ceiling is real.
//!
//! WF-11 has carried a `[PROVISIONAL]` marker since phase 5 because its
//! falsifier needs two things in one place — a memory ceiling that is real, and
//! a payload large enough to hit it. This target supplies both: it measures this
//! isolate's ceiling **in-process**, then drives payloads sized against it
//! through the adapter's actual non-streaming encode path, with a binary-format
//! control on the same bytes.
//!
//! It gathers evidence and changes no wire format. Nothing here is a conformance
//! rule, and none is owed: WF-11 is a *wire* clause, and no event-store rule can
//! observe how much memory an encode took.
//!
//! # The subject, and where it actually lives
//!
//! The whole-payload materialisation the falsifier is about happens in
//! `happenstance-core`'s `Event` serialisation, reachable only through that
//! crate's `serde` feature: the human-readable arm renders the payload into a
//! `String` and hands it to `serialize_str`, which writes the entire rendering
//! into the output before returning. `mod payload`'s own documentation states
//! the property and names this falsifier: serde's data model has no streaming
//! entry point for a string, so the exposure is a property of the data model
//! rather than of any one encoding.
//!
//! So the falsifier can be fired without a sync port, without a peer and without
//! an adapter-to-port-crate edge — which is what makes it constructible at all
//! in a crate that is an event store and ships no sync port.
//!
//! # The instrument, and why it is trustworthy
//!
//! `core::arch::wasm32::memory_size` and `memory_grow` are both **safe**
//! functions — this workspace forbids the other kind and this adapter has no
//! escape hatch — and `memory_grow` reports a refusal by returning `usize::MAX`
//! rather than trapping, so a staircase can find the real ceiling of *this*
//! isolate under *this* runner without aborting the process.
//!
//! Two properties make it the right instrument rather than a convenient one.
//! Linear memory **never shrinks**, so a `memory_size` reading taken before and
//! after an encode is a *high-water mark* and not a sample: the number cannot be
//! flattered by an allocator that freed early. And a ceiling discovered by
//! asking the host is the number the process actually hits, rather than one
//! copied from a documentation page.
//!
//! # Order is the design, not an accident
//!
//! Inside [`records_the_ceiling_and_the_encode_peaks`] the anchored encodes run
//! **first**, while the heap is still tight and a `memory_size` delta means
//! something; the ceiling is discovered **second**; the computed size is
//! attempted **last**. Growing memory is one-way, so any other order silently
//! zeroes every peak. Do not reorder this for readability.
//!
//! That one-way property is also why this target carries exactly **two** cases
//! rather than five: linear memory is one global resource shared by every test
//! in a target. One case owns the ordering; the other
//! ([`the_two_encode_paths_produce_the_published_size_ratio`]) is written to be
//! order-independent on purpose, comparing encoded *lengths* rather than memory,
//! so it is immune to whatever the first did.
//!
//! # This target is empty on the host, and that is correct
//!
//! `#![cfg(target_arch = "wasm32")]` means an ordinary
//! `cargo test -p happenstance-cloudflare` reports it as an empty target. That
//! is *not* the vacuity `xtask/src/proof.rs`'s row guards against — that guard
//! runs against the `wasm32` build, where the two names below are asserted out
//! of `cargo test -- --list` before anything runs. A host run of this code would
//! measure a machine where materialising 340 KiB is free, which is the wrong
//! implementation this target is defined against.

#![cfg(target_arch = "wasm32")]

use core::arch::wasm32::{memory_grow, memory_size};

use happenstance_core::{Event, MIN_SUPPORTED_EVENT_DATA_LEN};
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

// ---------------------------------------------------------------------------
// The numbers this probe is anchored on
// ---------------------------------------------------------------------------

/// One page of `wasm32` linear memory, in bytes.
///
/// The resolution limit of the whole instrument: a difference smaller than this
/// is not observable through `memory_size`, which is why the peak-separation
/// claim is made at the reference size and not at the floor.
const PAGE_BYTES: usize = 65_536;

/// The event type every probed payload rides on.
///
/// Constant across every attempt so that the only thing varying between two
/// measurements is the payload.
const EVENT_TYPE: &str = "PayloadForwarded";

/// The seed the payload generator is driven by, written in the source.
///
/// The same value the workspace's out-of-gate wire-format measurement
/// (`w5_payload_encodings.rs:26-42`) publishes, so these `wasm32` numbers land
/// beside comparable host numbers instead of beside nothing. The generator below
/// is **re-derived in eight lines rather than imported**: that measurement lives
/// in a tree deliberately kept out of the gate, and a gate step may not depend
/// on it.
///
/// The seed is in the source rather than described, for the reason that
/// measurement gives against its own earlier draft: a size cited with no seed is
/// reproducible in form only.
const SEED: u64 = 0x2545_F491_4F6C_DD1D;

/// The reference payload size, in bytes — 340 KiB.
///
/// The exact subject of the measurement table in
/// `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
/// and of WF-11's own clause text.
const REFERENCE_PAYLOAD_LEN: usize = 348_160;

/// The published cost of the reference payload through the human-readable path.
///
/// From the atom's own table: the payload rendered into a JSON string, quotes
/// included. A pure function of the payload's length, which is why it is
/// asserted exactly rather than within a tolerance.
const PUBLISHED_HUMAN_READABLE_LEN: usize = 464_218;

/// The published cost of the same payload through the binary path.
///
/// The raw bytes plus a three-byte length varint.
const PUBLISHED_BINARY_LEN: usize = 348_163;

/// What an empty payload still costs in the human-readable encoding.
///
/// An empty JSON string is two characters, not zero — so it has to be added back
/// when the payload's own contribution is isolated as a delta between two
/// otherwise identical events.
const EMPTY_HUMAN_READABLE_FIELD: usize = 2;

/// What an empty payload still costs in the binary encoding: a one-byte zero
/// length varint.
const EMPTY_BINARY_FIELD: usize = 1;

/// The numerator of the peak multiple, `11/3`.
///
/// Encoding P bytes through the human-readable path holds, at peak, the payload
/// itself (P), the rendered `String` (≈4P/3) and the serialiser's own output
/// buffer (≈4P/3): `1 + 4/3 + 4/3 = 11/3 ≈ 3.667`.
///
/// A **named item rather than a comment**, so that a reviewer can check the
/// predicted failure size against the measured ceiling by reading the source.
const PEAK_NUMERATOR: usize = 11;

/// The denominator of the peak multiple, `11/3`. See [`PEAK_NUMERATOR`].
const PEAK_DENOMINATOR: usize = 3;

/// Cloudflare's documented per-isolate memory limit, in bytes.
///
/// Transcribed from the platform's documentation and used for exactly one thing:
/// as the *stated basis* for a predicted failure size when this runner turns out
/// to impose no ceiling of its own. It is never reported as a measurement — the
/// whole point of this target is that a measured number and a documented one are
/// different kinds of thing and are printed beside each other rather than
/// conflated.
const PLATFORM_ISOLATE_CEILING_BYTES: usize = 128 * 1024 * 1024;

/// The architectural maximum of a `wasm32` linear memory, in pages — 4 GiB.
///
/// The other half of the configuration side of the comparison.
const WASM32_MAX_PAGES: usize = 65_536;

/// The same maximum in bytes, and it is a `u64` for a reason worth stating: it
/// does not fit in this target's `usize`, which is 32 bits. The architectural
/// ceiling of a `wasm32` linear memory is one byte past the largest address that
/// memory can name.
const WASM32_MAX_BYTES: u64 = 4 * 1024 * 1024 * 1024;

/// How many pages the ceiling staircase is willing to have granted to it.
///
/// A page beyond the platform's own documented ceiling, and the choice is the
/// whole of this probe's cost control. A runner that honours something near the
/// platform limit is *detected* — the staircase reaches a refusal — and one that
/// does not is *bounded*, because a staircase walking a 4 GiB ceiling in 64 KiB
/// steps would turn the gate into a hang, and a hang is the one failure mode
/// that teaches a team to skip a step.
const CEILING_BUDGET_PAGES: usize = (PLATFORM_ISOLATE_CEILING_BYTES / PAGE_BYTES) + 16;

/// How many grow requests the staircase may spend.
///
/// Doubling reaches the page budget in about eleven steps and the bisection that
/// follows a refusal costs about as many again; sixty-four is a ceiling on the
/// *shape* being wrong rather than a tuned number.
const CEILING_STEP_BUDGET: usize = 64;

/// The largest payload this probe will attempt to encode, in bytes.
///
/// Eight times the store limit this adapter's own fixture declares
/// (`MAX_EVENT_DATA_LEN = 1 MiB`, `tests/support/mod.rs`), and that excess is
/// deliberate: `MAX_EVENT_DATA_LEN` is a limit on what a store will **accept**,
/// while WF-11's falsifier is about what a peer must **forward**. So the probe
/// may and should attempt sizes this store would refuse — a deliberate over-size
/// here is not a violated store limit, and the report says so.
const ATTEMPT_BUDGET_BYTES: usize = 8 * 1024 * 1024;

// ---------------------------------------------------------------------------
// The instrument
// ---------------------------------------------------------------------------

/// Linear memory's current size, in pages.
///
/// A high-water mark rather than a sample, because `wasm32` linear memory never
/// shrinks.
fn high_water_pages() -> usize {
    memory_size::<0>()
}

/// A value, and how far linear memory had to grow while it was being built.
#[derive(Debug)]
struct Measured<T> {
    /// What the closure produced.
    value: T,
    /// The growth across the closure, in pages — a lower bound on the true peak.
    peak_pages: usize,
}

/// Fills whatever free space the heap already holds, and hands it back to be
/// held.
///
/// **The instrument does not resolve anything without this, and the first run of
/// this probe proved it.** The runner starts the module with a 4 MB heap, so a
/// 340 KiB payload encoded through either path fits entirely inside free space
/// the allocator already owns: `memory_size` never moves and both readings come
/// out at zero pages. Zero against zero is not a measurement of two encoders —
/// it is a measurement of the runner's start-up.
///
/// So the free list is filled first, in half-page blocks, until the host has to
/// grant a page. From there the closure's own allocations have to be asked of
/// the host rather than served out of a hole an earlier allocation left behind,
/// and the delta is about the encode.
///
/// The ballast is returned rather than dropped because it has to stay alive
/// *across* the measurement; dropping it would hand the free space straight
/// back.
fn tighten() -> Vec<Vec<u8>> {
    let mut ballast = Vec::new();
    let before = high_water_pages();
    while high_water_pages() == before {
        ballast.push(vec![0u8; PAGE_BYTES / 2]);
    }
    ballast
}

/// Runs `build` against a tightened heap, between two readings of the high-water
/// mark.
///
/// The delta bounds the closure's true peak **from below**: what is left of the
/// free list after [`tighten`] is under one half-page, and an allocator can
/// still round a request up or coalesce two of them, so this can understate a
/// cost and can never overstate one.
fn measuring<T>(build: impl FnOnce() -> T) -> Measured<T> {
    let ballast = tighten();
    let before = high_water_pages();
    let value = build();
    let after = high_water_pages();
    drop(ballast);
    Measured {
        value,
        peak_pages: after.saturating_sub(before),
    }
}

/// `len` bytes from a xorshift64\* stream seeded by [`SEED`].
///
/// Re-derived here rather than imported; see [`SEED`] for why. Deterministic and
/// dependency-free, so a second run on this runner reproduces the bytes exactly
/// and a run on another runner reproduces them too.
fn deterministic_payload(len: usize) -> Vec<u8> {
    let mut state: u64 = SEED;
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        out.extend_from_slice(&state.to_le_bytes());
    }
    out.truncate(len);
    out
}

/// The peak a human-readable encode of `payload_len` bytes is predicted to hold.
///
/// Divided before multiplied, deliberately: `usize` is 32 bits on this target and
/// the other order overflows on any payload past about 390 MiB, which is inside
/// the range this arithmetic is asked about.
const fn predicted_peak(payload_len: usize) -> usize {
    payload_len / PEAK_DENOMINATOR * PEAK_NUMERATOR
}

/// The inverse: the payload whose predicted peak fills `budget_bytes`.
const fn payload_that_peaks_at(budget_bytes: usize) -> usize {
    budget_bytes / PEAK_NUMERATOR * PEAK_DENOMINATOR
}

/// A ratio against the payload, in basis points.
///
/// Integer arithmetic through `u64` rather than a float, because `usize` is 32
/// bits here and `464_218 * 10_000` does not fit in one.
fn basis_points(encoded: usize, payload: usize) -> u64 {
    (encoded as u64) * 10_000 / (payload as u64)
}

/// What one staircase found out about this isolate.
#[derive(Debug)]
struct Ceiling {
    /// Linear memory's size in pages when the walk started.
    started_at_pages: usize,
    /// Linear memory's size in pages when it stopped.
    ///
    /// A hard **lower** bound on this isolate's ceiling: the host granted every
    /// page of it.
    reached_pages: usize,
    /// The smallest total page count the host refused to reach, if it refused.
    ///
    /// A hard **upper** bound, and `None` means the walk never found one — in
    /// which case this isolate's ceiling was not measured at all, only bounded
    /// from below, and the verdict has to say so.
    refused_at_pages: Option<usize>,
    /// Grow requests spent, against [`CEILING_STEP_BUDGET`].
    steps: usize,
    /// Whether the walk stopped on the probe's own budget rather than on a
    /// refusal by the host.
    stopped_on_budget: bool,
}

/// Walks linear memory upwards until the host refuses, or the budget runs out.
///
/// Doubling first, then a bisection of the interval the doubling left behind —
/// EC-002's shape, and the reason a runner that permits growth toward the 4 GiB
/// architectural limit costs this step a bounded number of requests instead of
/// sixty-five thousand.
///
/// A successful grow moves the base, so the bisection re-bases its bracket after
/// each grant rather than re-scanning from the start.
fn walk_to_the_ceiling() -> Ceiling {
    let started_at_pages = high_water_pages();
    let mut granted = 0usize;
    let mut steps = 0usize;
    let mut refused_at_pages: Option<usize> = None;
    let mut delta = 1usize;

    // `memory_grow` returns the *previous* size, or `usize::MAX` on refusal. It
    // is a refusal signal, not an error to unwrap — and it does not trap, which
    // is exactly what makes it usable as a probe here.
    while steps < CEILING_STEP_BUDGET && granted.saturating_add(delta) <= CEILING_BUDGET_PAGES {
        steps += 1;
        if memory_grow::<0>(delta) == usize::MAX {
            refused_at_pages = Some(high_water_pages().saturating_add(delta));
            break;
        }
        granted += delta;
        delta = delta.saturating_mul(2);
    }

    if refused_at_pages.is_some() {
        // `delta` is a refused request; the largest grantable one is below it.
        let mut headroom = delta;
        while steps < CEILING_STEP_BUDGET && headroom > 1 && granted < CEILING_BUDGET_PAGES {
            let mid = headroom / 2;
            steps += 1;
            if memory_grow::<0>(mid) == usize::MAX {
                headroom = mid;
                refused_at_pages = Some(high_water_pages().saturating_add(mid));
            } else {
                granted += mid;
                headroom -= mid;
            }
        }
    }

    Ceiling {
        started_at_pages,
        reached_pages: high_water_pages(),
        refused_at_pages,
        steps,
        stopped_on_budget: refused_at_pages.is_none(),
    }
}

/// One payload size, taken through both encode paths and measured.
#[derive(Debug)]
struct Attempt {
    /// The payload this attempt carried, in bytes.
    payload_len: usize,
    /// The human-readable encoding's length, in bytes.
    human_readable_len: usize,
    /// Linear memory's growth across the human-readable encode, in pages.
    human_readable_peak_pages: usize,
    /// The binary encoding's length, in bytes.
    binary_len: usize,
    /// Linear memory's growth across the binary encode, in pages.
    binary_peak_pages: usize,
}

/// Announces a size, then attempts it through both paths on identical bytes.
///
/// **Announce, then attempt**, and the order is the policy rather than a habit:
/// an allocation failure on this target aborts the isolate instead of returning
/// an error, so a size that kills the process has to be on the transcript
/// *before* the attempt that kills it. An abort then leaves the diagnosis behind
/// rather than collapsing to an exit code.
///
/// The **binary** path runs first, deliberately. Linear memory never shrinks, so
/// whichever path runs first gets first refusal on whatever free space the heap
/// already holds: running the cheaper one first can only understate the
/// difference this measurement is about, never manufacture it.
fn attempt(payload_len: usize, why: &str) -> Attempt {
    let predicted = predicted_peak(payload_len);
    console_log!(
        "WF-11 attempt: payload = {} bytes ({}); predicted peak = {} bytes = {} pages",
        payload_len,
        why,
        predicted,
        predicted.div_ceil(PAGE_BYTES),
    );

    let payload = deterministic_payload(payload_len);
    let event = Event::new(EVENT_TYPE, payload.clone()).expect("a valid event type");

    let binary = measuring(|| postcard::to_stdvec(&event).expect("an Event encodes to postcard"));
    let binary_len = binary.value.len();
    let binary_peak_pages = binary.peak_pages;
    drop(binary);

    let human = measuring(|| serde_json::to_vec(&event).expect("an Event encodes to JSON"));
    let human_readable_len = human.value.len();
    let human_readable_peak_pages = human.peak_pages;

    // The round trip is what a re-implementation cannot fake. It runs *after*
    // both peaks are read, so its own allocations cannot reach either of them.
    let decoded: Event =
        serde_json::from_slice(&human.value).expect("the encoded form decodes back to an Event");
    assert!(
        decoded == event,
        "the payload did not survive the round trip, so whatever was measured was \
         not the encoder a peer would call"
    );
    assert!(
        decoded.data()[..] == payload[..],
        "the decoded payload is not byte-identical to the input"
    );
    drop(human);

    console_log!(
        "  human-readable: {} bytes, grew {} pages ({} bytes) | binary: {} bytes, grew {} pages \
         ({} bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing \
         early cannot flatter | round trip byte-identical",
        human_readable_len,
        human_readable_peak_pages,
        human_readable_peak_pages * PAGE_BYTES,
        binary_len,
        binary_peak_pages,
        binary_peak_pages * PAGE_BYTES,
    );

    Attempt {
        payload_len,
        human_readable_len,
        human_readable_peak_pages,
        binary_len,
        binary_peak_pages,
    }
}

// ---------------------------------------------------------------------------
// The verdict
// ---------------------------------------------------------------------------

/// The three admissible answers, and only those three.
///
/// *We did not see it fire* is made **unrepresentable** rather than discouraged:
/// there is no fourth variant and no catch-all, so the run has to choose one of
/// these and the story that consumes this measurement cites a choice rather than
/// interpreting a paragraph.
#[derive(Debug)]
enum Verdict {
    /// (a) It fires: at this payload size the human-readable encode reached or
    /// passed the measured ceiling.
    FiresAt {
        /// The payload size at which it fired.
        payload_len: usize,
        /// Where linear memory stood when it did, in pages.
        peak_pages: usize,
        /// How the failure presented.
        failure: String,
    },
    /// (b) It does not fire, with measured headroom stated as a number.
    DoesNotFire {
        /// The largest payload this runtime could be made to attempt.
        largest_attempted: usize,
        /// Bytes between the high-water mark and the measured ceiling.
        headroom_bytes: usize,
        /// The payload size the arithmetic predicted would fire.
        predicted_failure_payload: usize,
    },
    /// (c) The condition is not constructible here.
    ///
    /// A verdict rather than a shrug, because it names what is missing.
    NotConstructibleHere {
        /// The exact element this runtime does not supply.
        missing: String,
        /// What would supply it.
        would_supply: String,
        /// What *was* observed, so the answer is bounded rather than empty.
        observed: String,
    },
}

impl Verdict {
    /// The one line this probe exists to emit.
    fn line(&self) -> String {
        match self {
            Self::FiresAt {
                payload_len,
                peak_pages,
                failure,
            } => format!(
                "WF-11 VERDICT: (a) FIRES at a payload of {payload_len} bytes — linear memory \
                 stood at {peak_pages} pages and the failure presented as: {failure}"
            ),
            Self::DoesNotFire {
                largest_attempted,
                headroom_bytes,
                predicted_failure_payload,
            } => format!(
                "WF-11 VERDICT: (b) DOES NOT FIRE — the largest payload this runtime could be \
                 made to attempt was {largest_attempted} bytes, leaving {headroom_bytes} bytes of \
                 measured headroom below the ceiling; the arithmetic predicted it would fire at \
                 {predicted_failure_payload} bytes"
            ),
            Self::NotConstructibleHere {
                missing,
                would_supply,
                observed,
            } => format!(
                "WF-11 VERDICT: (c) NOT CONSTRUCTIBLE HERE — missing: {missing}; what would \
                 supply it: {would_supply}; what was observed: {observed}"
            ),
        }
    }
}

/// Chooses between the three, from the measurements and nothing else.
fn verdict(ceiling: &Ceiling, largest: &Attempt, predicted_failure_payload: usize) -> Verdict {
    let Some(refused_at_pages) = ceiling.refused_at_pages else {
        return Verdict::NotConstructibleHere {
            missing: format!(
                "a memory ceiling this isolate can be made to reach. The host granted every one \
                 of the {} pages this probe asked for, taking linear memory to {} pages = {} \
                 bytes — past the platform's own documented per-isolate limit of \
                 {PLATFORM_ISOLATE_CEILING_BYTES} bytes — and refused nothing, so no ceiling was \
                 measured and no payload the arithmetic can name would exhaust one",
                ceiling
                    .reached_pages
                    .saturating_sub(ceiling.started_at_pages),
                ceiling.reached_pages,
                ceiling.reached_pages * PAGE_BYTES,
            ),
            would_supply: format!(
                "an isolate that actually enforces the platform's documented ceiling — a real \
                 Workers isolate rather than this runner, or a runner flag capping the linear \
                 memory a test module may grow to. At the platform's {PLATFORM_ISOLATE_CEILING_BYTES}-byte \
                 limit the arithmetic puts the firing payload at {predicted_failure_payload} bytes"
            ),
            observed: format!(
                "every attempted encode succeeded, the largest at a payload of {} bytes — which \
                 the human-readable path rendered into {} bytes, growing linear memory by {} \
                 pages ({} bytes), against the binary path's {} bytes and {} pages ({} bytes) on \
                 identical bytes",
                largest.payload_len,
                largest.human_readable_len,
                largest.human_readable_peak_pages,
                largest.human_readable_peak_pages * PAGE_BYTES,
                largest.binary_len,
                largest.binary_peak_pages,
                largest.binary_peak_pages * PAGE_BYTES,
            ),
        };
    };

    let ceiling_bytes = refused_at_pages.saturating_mul(PAGE_BYTES);
    let standing_bytes = high_water_pages().saturating_mul(PAGE_BYTES);
    match ceiling_bytes.checked_sub(standing_bytes) {
        Some(headroom_bytes) if headroom_bytes > 0 => Verdict::DoesNotFire {
            largest_attempted: largest.payload_len,
            headroom_bytes,
            predicted_failure_payload,
        },
        _ => Verdict::FiresAt {
            payload_len: largest.payload_len,
            peak_pages: high_water_pages(),
            failure: format!(
                "the high-water mark reached the ceiling the host refused to grow past \
                 ({refused_at_pages} pages) while rendering {} bytes of payload into {} bytes of \
                 human-readable output, with no headroom left below it",
                largest.payload_len, largest.human_readable_len,
            ),
        },
    }
}

// ---------------------------------------------------------------------------
// Reporting
//
// Every number reaches a reader through `console_log!` and never through
// `println!`, which is a silent discard on this target — so a probe that
// reported through the ordinary macro would run, measure, and tell nobody. Three
// functions rather than three blocks inside the case below, because the case's
// *order* is the thing a reader has to be able to see at a glance and a hundred
// lines of formatting buries it.
// ---------------------------------------------------------------------------

/// What the instrument is, and what the configuration claims, side by side.
///
/// The two are printed together and labelled, rather than one being quoted as
/// though it were the other: a measured page count and a documented limit are
/// different kinds of thing, and conflating them is how a probe like this stops
/// being evidence.
fn report_entry() {
    let entry_pages = high_water_pages();
    console_log!(
        "=== WF-11 falsifier probe ===\n  page = {} bytes; linear memory at entry = {} pages = {} \
         bytes\n  measured in-process, never read off a documentation page. Beside it, for \
         comparison rather than conflation, what the configuration claims: the platform's \
         documented per-isolate limit is {} bytes, and the architectural maximum of a wasm32 \
         linear memory is {} pages = {} bytes. The runner this executes under declares no maximum \
         of its own.",
        PAGE_BYTES,
        entry_pages,
        entry_pages * PAGE_BYTES,
        PLATFORM_ISOLATE_CEILING_BYTES,
        WASM32_MAX_PAGES,
        WASM32_MAX_BYTES,
    );
}

/// What the staircase found, including what it did *not* find.
///
/// A walk that stopped on the probe's own budget is reported as such rather than
/// as a ceiling: *at least X, the probe's budget* is honest, and a budget
/// reported as a measurement is not.
fn report_ceiling(ceiling: &Ceiling) {
    console_log!(
        "WF-11 ceiling walk: started at {} pages, reached {} pages ({} bytes), refused at {}, {} \
         of {} grow requests spent, stopped on {}",
        ceiling.started_at_pages,
        ceiling.reached_pages,
        ceiling.reached_pages * PAGE_BYTES,
        ceiling.refused_at_pages.map_or_else(
            || "nothing — the host granted every request".to_owned(),
            |pages| format!("{pages} pages = {} bytes", pages * PAGE_BYTES),
        ),
        ceiling.steps,
        CEILING_STEP_BUDGET,
        if ceiling.stopped_on_budget {
            "the probe's own page budget"
        } else {
            "a refusal by the host"
        },
    );
}

/// The arithmetic, and the basis it was applied to — printed *before* the
/// attempt that tests it.
///
/// The basis is labelled because it changes what the predicted size means: from
/// a measured ceiling it is a prediction about this runtime, and from the
/// platform's documented limit it is a prediction about the runtime this one is
/// standing in for.
fn report_arithmetic(ceiling: &Ceiling, headroom_bytes: usize, predicted: usize) {
    console_log!(
        "WF-11 arithmetic: peak = payload * {}/{}. Against {} bytes of {} headroom the predicted \
         firing payload is {} bytes. This probe's own attempt budget is {} bytes, which is eight \
         times the {} byte MAX_EVENT_DATA_LEN this adapter's fixture declares — deliberately over \
         that store limit, because the falsifier is about forwarding a payload rather than \
         appending one.",
        PEAK_NUMERATOR,
        PEAK_DENOMINATOR,
        headroom_bytes,
        if ceiling.refused_at_pages.is_some() {
            "measured"
        } else {
            "documented-platform (no ceiling was measured)"
        },
        predicted,
        ATTEMPT_BUDGET_BYTES,
        ATTEMPT_BUDGET_BYTES / 8,
    );
}

// ---------------------------------------------------------------------------
// The two cases
// ---------------------------------------------------------------------------

/// The measurement: this isolate's ceiling, and what an encode costs against it.
///
/// The order inside this function is load-bearing — see the module documentation
/// — and is: anchored encodes, ceiling, computed size.
#[wasm_bindgen_test]
fn records_the_ceiling_and_the_encode_peaks() {
    report_entry();

    // 1 — the anchors, while the heap is still tight and a delta means something.
    let floor = attempt(
        MIN_SUPPORTED_EVENT_DATA_LEN,
        "floor anchor: MIN_SUPPORTED_EVENT_DATA_LEN, the smallest payload every conformant store \
         must accept, so 'a store took it but a peer cannot forward it' is a real contradiction",
    );
    let reference = attempt(
        REFERENCE_PAYLOAD_LEN,
        "reference anchor: the 340 KiB payload the published measurement table is about",
    );

    assert_eq!(
        floor.payload_len, MIN_SUPPORTED_EVENT_DATA_LEN,
        "the floor anchor was not attempted, so the smallest size at which the contradiction is \
         real went unmeasured"
    );
    assert_eq!(
        reference.payload_len, REFERENCE_PAYLOAD_LEN,
        "the reference anchor was not attempted, so these numbers land beside nothing"
    );

    // The memory claim, at the one size where 64 KiB granularity can resolve it.
    // At the floor the difference is about 21 KiB — under a single page — which
    // is why that anchor is carried by the size ratio and not by this assertion.
    assert!(
        reference.human_readable_peak_pages > reference.binary_peak_pages,
        "the human-readable encode grew linear memory by {} pages and the binary encode by {} on \
         identical bytes. No separation at the reference size is an instrument finding — the \
         allocator's growth policy rather than the encoder — and the page arithmetic belongs in \
         the report rather than in a raised size chosen until a difference appeared.",
        reference.human_readable_peak_pages,
        reference.binary_peak_pages,
    );

    // 2 — the ceiling, second, because growing memory is one-way.
    let ceiling = walk_to_the_ceiling();
    report_ceiling(&ceiling);

    assert!(
        ceiling.reached_pages >= ceiling.started_at_pages,
        "linear memory reported as having shrunk, which it cannot do — the instrument is not \
         measuring what it claims"
    );
    assert!(
        ceiling.reached_pages > 0,
        "a ceiling of zero pages is not a measurement"
    );
    assert!(
        ceiling.steps > 0,
        "the staircase spent no grow requests, so nothing was asked of the host"
    );

    // 3 — the computed size, last, because attempting it may end the isolate.
    let headroom_bytes = ceiling.refused_at_pages.map_or_else(
        || PLATFORM_ISOLATE_CEILING_BYTES,
        |refused| refused.saturating_sub(ceiling.reached_pages) * PAGE_BYTES,
    );
    let predicted_failure_payload = payload_that_peaks_at(headroom_bytes) + 1;
    report_arithmetic(&ceiling, headroom_bytes, predicted_failure_payload);

    let target = predicted_failure_payload.min(ATTEMPT_BUDGET_BYTES);
    let mut largest = reference;
    let mut size = REFERENCE_PAYLOAD_LEN.saturating_mul(2);
    while size <= target {
        largest = attempt(size, "staircase toward the predicted firing size");
        size = size.saturating_mul(2);
    }

    let verdict = verdict(&ceiling, &largest, predicted_failure_payload);
    let line = verdict.line();
    assert!(
        line.starts_with("WF-11 VERDICT: "),
        "the verdict line is the artefact the next story cites by shape; it was not produced"
    );
    console_log!("{}", line);
}

/// The control: was the encoder exercised at all?
///
/// A **size** comparison rather than a memory one, and deliberately: the
/// published table is a table of sizes, and a size is deterministic,
/// order-independent and runner-independent where a memory reading can be
/// flattened by an allocator. This case constructs its own payload, reads no
/// global state and asserts nothing about linear memory, so it passes
/// identically whether it runs before or after the staircase.
///
/// A run in which the two lengths agree means the encoder was never exercised
/// and the whole measurement is **void** — a defect to diagnose, not a verdict to
/// publish.
#[wasm_bindgen_test]
fn the_two_encode_paths_produce_the_published_size_ratio() {
    let payload = deterministic_payload(REFERENCE_PAYLOAD_LEN);
    assert_eq!(
        payload.len(),
        REFERENCE_PAYLOAD_LEN,
        "the generator did not produce the reference payload"
    );

    // One `Event`, and its twin carrying nothing, so that *the same bytes* is a
    // fact about the code rather than a claim in a report. The payload's own
    // contribution to each encoding is the delta between the two, plus what an
    // empty payload still costs — which is how the published figures, taken
    // against a bare payload, are reproduced from an envelope-carrying event.
    let carrying = Event::new(EVENT_TYPE, payload.clone()).expect("a valid event type");
    let bare = Event::new(EVENT_TYPE, &b""[..]).expect("a valid event type");

    let human_readable = serde_json::to_vec(&carrying)
        .expect("an Event encodes to JSON")
        .len()
        - serde_json::to_vec(&bare)
            .expect("an Event encodes to JSON")
            .len()
        + EMPTY_HUMAN_READABLE_FIELD;
    let binary = postcard::to_stdvec(&carrying)
        .expect("an Event encodes to postcard")
        .len()
        - postcard::to_stdvec(&bare)
            .expect("an Event encodes to postcard")
            .len()
        + EMPTY_BINARY_FIELD;

    console_log!(
        "WF-11 control: at a payload of {} bytes the human-readable path costs {} bytes \
         ({} basis points of the payload) and the binary path {} bytes ({} basis points). \
         Published: {} and {}.",
        REFERENCE_PAYLOAD_LEN,
        human_readable,
        basis_points(human_readable, REFERENCE_PAYLOAD_LEN),
        binary,
        basis_points(binary, REFERENCE_PAYLOAD_LEN),
        PUBLISHED_HUMAN_READABLE_LEN,
        PUBLISHED_BINARY_LEN,
    );

    assert_ne!(
        human_readable, binary,
        "the two paths reported the same cost, so the encoder was not exercised and this run's \
         result is void rather than a verdict"
    );
    assert_eq!(
        human_readable, PUBLISHED_HUMAN_READABLE_LEN,
        "the human-readable cost does not reproduce the published figure; the length of that \
         encoding is a pure function of the payload's length, so a mismatch means the bytes did \
         not go through the branch the falsifier is about"
    );
    assert_eq!(
        binary, PUBLISHED_BINARY_LEN,
        "the binary cost does not reproduce the published figure"
    );

    // The ratios, with the tolerance stated: five basis points either side, which
    // is one part in two thousand.
    let human_bp = basis_points(human_readable, REFERENCE_PAYLOAD_LEN);
    let binary_bp = basis_points(binary, REFERENCE_PAYLOAD_LEN);
    assert!(
        (13_328..=13_338).contains(&human_bp),
        "the human-readable path came out at {human_bp} basis points of the payload, against the \
         published 13,333 (4/3)"
    );
    assert!(
        (9_995..=10_005).contains(&binary_bp),
        "the binary path came out at {binary_bp} basis points of the payload, against the \
         published 10,000 (1.0000)"
    );
}
