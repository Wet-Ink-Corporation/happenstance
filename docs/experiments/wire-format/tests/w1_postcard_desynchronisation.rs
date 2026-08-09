//! W1 — does `skip_serializing_if` desynchronise a `postcard` decoder?
//!
//! Backs ADR-0016 decision §3 and the WF-2/WF-7 amendments: whether D1's five
//! `#[serde(default, skip_serializing_if = ...)]` sites (`event.rs:578,580`,
//! `query.rs:352,354`, `append.rs:248` at HEAD) are safe in a
//! length-prefixed, non-self-describing format the way they are in JSON.
//!
//! **Why a local mirror, not `happenstance_core::Event` itself.** Phase 5 is
//! going to delete these five attributes — that is the whole point of the
//! ADR this experiment supports. If this test asserted against the real
//! `Event`'s `Serialize` impl, it would stop demonstrating the bug the day
//! the bug is fixed, and an experiment that stops compiling — or stops
//! *measuring the thing it was written to measure* — the moment the phase it
//! documents completes is worse than no experiment (see the parent task's
//! brief and `README.md`). `EventMirrorPreFix` below is a permanent,
//! byte-for-byte reproduction of the pre-fix shape, decoupled from the crate.
//! It will keep demonstrating this exact bug for as long as this file exists,
//! regardless of what `happenstance-core` looks like by then.
//!
//! Separately, and NOT asserted on (informational only, printed with a
//! caveat), the same scenario is run against the real `happenstance_core::Event`
//! at whatever HEAD this crate is built against, so a reader can see whether
//! the fix has landed.

use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, Tags};
use serde::{Deserialize, Serialize};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}

/// Permanent reproduction of `EventWire` as it stands at HEAD
/// (`crates/happenstance-core/src/event.rs:575-582`): four fields, `tags`
/// and `metadata` skipped when at their default. Decoupled from the real
/// crate on purpose — see the module doc.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventMirrorPreFix {
    event_type: String,
    data: Vec<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<u8>>,
}

/// The same four fields, no skip attributes — what WF-2's MUST already
/// requires and what the mirror above violates.
#[derive(Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
struct EventMirrorNoSkip {
    event_type: String,
    data: Vec<u8>,
    tags: Vec<String>,
    metadata: Option<Vec<u8>>,
}

fn mirror_tagged() -> EventMirrorPreFix {
    EventMirrorPreFix {
        event_type: "A".into(),
        data: vec![0x11, 0x22, 0x33, 0x44],
        tags: vec!["course:c1".into()],
        metadata: None,
    }
}

fn mirror_tagged_noskip() -> EventMirrorNoSkip {
    EventMirrorNoSkip {
        event_type: "A".into(),
        data: vec![0x11, 0x22, 0x33, 0x44],
        tags: vec!["course:c1".into()],
        metadata: None,
    }
}

/// The nine neighbours the ADR's "3 wrong / 2 swallowed / 4 error" split is
/// about, enumerated here rather than described. Each is postcard-encoded
/// immediately after the pre-fix mirror in one buffer, then a solo
/// `take_from_bytes::<EventMirrorPreFix>` is run over the combined buffer —
/// exactly what a batch decoder does. The outcome is one of:
///   WRONG   — decodes to Ok with the wrong value, no error
///   SWALLOWED — decodes to Ok with the *original* value, but consumed part
///               or all of the neighbour's bytes as if they were its own
///   ERROR   — decode fails
fn try_neighbour<T: Serialize + core::fmt::Debug>(name: &str, neighbour: &T) -> &'static str {
    let event = mirror_tagged();
    let buffer = postcard::to_stdvec(&(event.clone(), neighbour)).unwrap();
    let solo = postcard::to_stdvec(&event).unwrap();
    println!("  neighbour: {name}");
    println!("    whole buffer      = [{}] ({} bytes)", hex(&buffer), buffer.len());
    println!("    neighbour's bytes = [{}]", hex(&buffer[solo.len()..]));
    let verdict = match postcard::take_from_bytes::<EventMirrorPreFix>(&buffer) {
        Ok((decoded, rest)) => {
            println!("    take_from_bytes   = Ok");
            println!("    decoded           = {decoded:?}");
            println!("    remaining         = [{}] ({} bytes)", hex(rest), rest.len());
            if decoded == event {
                "SWALLOWED"
            } else {
                "WRONG"
            }
        }
        Err(e) => {
            println!("    take_from_bytes   = Err({e})");
            "ERROR"
        }
    };
    println!("    VERDICT           = {verdict}");
    println!();
    verdict
}

#[test]
fn w1_postcard_desynchronisation() {
    println!("=== W1(a) the pre-fix mirror, solo, at HEAD's shape ===");
    let event = mirror_tagged();
    let solo = postcard::to_stdvec(&event).unwrap();
    let noskip = postcard::to_stdvec(&mirror_tagged_noskip()).unwrap();
    println!("event (mirror)  = {event:?}");
    println!("postcard         = [{}]  ({} bytes)", hex(&solo), solo.len());
    println!(
        "no-skip postcard = [{}]  ({} bytes)   delta = +{}",
        hex(&noskip),
        noskip.len(),
        noskip.len() - solo.len()
    );
    assert_eq!(solo.len(), 18, "the tagged, one-absent-field shape is 18 bytes");
    assert_eq!(
        noskip.len(),
        19,
        "restoring the one absent field (metadata) costs exactly one byte"
    );
    println!(
        "solo round-trip (mirror)  = {:?}",
        postcard::from_bytes::<EventMirrorPreFix>(&solo).map(|v| v == event)
    );
    println!();

    println!("=== W1(a2) the all-defaults shape: no tags AND no metadata (two absent fields) ===");
    let bare = EventMirrorPreFix {
        event_type: "A".into(),
        data: vec![0x11, 0x22, 0x33, 0x44],
        tags: vec![],
        metadata: None,
    };
    let bare_noskip = EventMirrorNoSkip {
        event_type: "A".into(),
        data: vec![0x11, 0x22, 0x33, 0x44],
        tags: vec![],
        metadata: None,
    };
    let bare_pc = postcard::to_stdvec(&bare).unwrap();
    let bare_noskip_pc = postcard::to_stdvec(&bare_noskip).unwrap();
    println!("postcard         = [{}] ({} bytes)", hex(&bare_pc), bare_pc.len());
    println!(
        "no-skip postcard = [{}] ({} bytes)   delta = +{}",
        hex(&bare_noskip_pc),
        bare_noskip_pc.len(),
        bare_noskip_pc.len() - bare_pc.len()
    );
    assert_eq!(bare_pc.len(), 7, "all-defaults shape is 7 bytes");
    assert_eq!(bare_noskip_pc.len(), 9, "restoring both absent fields costs exactly two bytes");
    println!(
        "solo round-trip (mirror)  = {:?}",
        postcard::from_bytes::<EventMirrorPreFix>(&bare_pc).map(|v| v == bare)
    );
    println!();

    println!("=== W1(b) nine neighbours, one buffer each, decoded as a solo EventMirrorPreFix ===");
    println!("(the tagged shape from (a) is the value under test in every case; each neighbour");
    println!(" matches probe-encoding's original nine, byte-for-byte, per the module doc's");
    println!(" encoding-equivalence argument)");
    let neighbour_long = EventMirrorPreFix {
        event_type: "A".into(),
        data: (0u8..70).collect(),
        tags: vec![],
        metadata: None,
    };
    let neighbour_short = EventMirrorPreFix {
        event_type: "B".into(),
        data: vec![0xaau8, 0xbb],
        tags: vec![],
        metadata: None,
    };
    let mut counts = (0u32, 0u32, 0u32); // wrong, swallowed, error
    let mut tally = |v: &'static str| match v {
        "WRONG" => counts.0 += 1,
        "SWALLOWED" => counts.1 += 1,
        "ERROR" => counts.2 += 1,
        _ => unreachable!(),
    };

    tally(try_neighbour(
        "1. second Event mirror, event_type \"A\", 70-byte payload, no tags, no metadata",
        &neighbour_long,
    ));
    tally(try_neighbour(
        "2. second Event mirror, event_type \"B\", 2-byte payload",
        &neighbour_short,
    ));
    tally(try_neighbour("3. bool false", &false));
    tally(try_neighbour("4. bool true", &true));
    tally(try_neighbour("5. Option<Vec<u8>> = Some(vec![0xde, 0xad])", &Some(vec![0xdeu8, 0xad])));
    tally(try_neighbour("6. Option<Vec<u8>> = None", &None::<Vec<u8>>));
    tally(try_neighbour("7. u64 = 1  (stands in for SequencePosition(1))", &1u64));
    tally(try_neighbour("8. a tuple of five u8: (1, 2, 0xaa, 0xbb, 5)", &(1u8, 2u8, 0xaau8, 0xbbu8, 5u8)));
    tally(try_neighbour("9. String \"hello\"", &String::from("hello")));

    println!(
        "=== TALLY: {} WRONG, {} SWALLOWED, {} ERROR (of 9) ===",
        counts.0, counts.1, counts.2
    );
    assert_eq!(
        counts,
        (3, 2, 4),
        "the load-bearing split: 3 wrong-value, 2 swallowed, 4 error, of 9 neighbours"
    );

    println!();
    println!("=== W1(c) informational only: the REAL happenstance_core::Event at this build's HEAD ===");
    println!("(not asserted — this is expected to change the day D1 lands; see README.md)");
    let real = Event::new("A", Bytes::from_static(&[0x11, 0x22, 0x33, 0x44]))
        .unwrap()
        .with_tags(Tags::from_pairs([("course", "c1")]).unwrap());
    let real_pc = postcard::to_stdvec(&real).unwrap();
    println!("real Event postcard = [{}] ({} bytes)", hex(&real_pc), real_pc.len());
    println!(
        "real solo round-trip = {:?}",
        postcard::from_bytes::<Event>(&real_pc).map(|v| v == real)
    );
    if real_pc.len() == solo.len() && real_pc == solo {
        println!("=> byte-identical to the mirror: D1 has NOT landed yet in this build.");
    } else {
        println!("=> differs from the mirror: D1 has landed (or the shape changed some other way).");
    }
}
