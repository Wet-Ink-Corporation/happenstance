//! The example is run, and its claims are read off the transcript it printed.
//!
//! `cargo test --workspace` compiles a binary and never calls `main`, so
//! without this target the four operations this example exists to demonstrate
//! would be checked by a compile. That is the decorative-gate shape the
//! repository names elsewhere, and it is why `xtask/src/proof.rs` carries a row
//! naming the two tests below.
//!
//! # Why the tests live in `mod runs`
//!
//! `proof.rs` matches fully-qualified test names out of `cargo test -- --list`,
//! and a bare top-level test lists without a module prefix and would never
//! match its row.
//!
//! # What is asserted here, and what is asserted in the program
//!
//! The program itself refuses to finish when a claim it makes is false: an
//! order that should have been rejected succeeding, a rebuild whose rows depend
//! on its chunk size, or the poisoned view decoding an event it cannot. Those
//! are `bail!`s in `src/main.rs`, so they fail the run rather than the diff,
//! and this file does not restate them. What it adds is the part a program
//! cannot check about itself — that it ran to the end, and that the sections
//! appear in the order the argument depends on.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod runs {
    use std::process::Command;

    /// The section markers, in the order the argument needs them.
    ///
    /// A sequence and not a set: "a view is added late" only means anything
    /// after two views have already been shown to disagree, and "rebuilt at two
    /// chunk sizes" only after something has been reset.
    const MARKERS: [&str; 7] = [
        "== the database this run writes to ==",
        "== seeding the fulfilment log ==",
        "== two views over one log, at two checkpoints ==",
        "== a third view, added after the fact ==",
        "== reset, and rebuilt at two chunk sizes ==",
        "== one poisoned view does not stall the others ==",
        "== final log ==",
    ];

    /// Runs the example and returns everything it printed.
    ///
    /// A test that asserts `status.success()` and discards the output makes
    /// every failure a re-run, so the panic carries both streams.
    fn transcript() -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_rebuilding-read-models"))
            .output()
            .expect("the example binary should be runnable");

        assert!(
            output.status.success(),
            "the example exited with {}\n--- stdout ---\n{}\n--- stderr ---\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        String::from_utf8(output.stdout).expect("the transcript should be UTF-8")
    }

    #[test]
    fn the_binary_operates_every_view() {
        let transcript = transcript();

        let mut searched_from = 0;
        for marker in MARKERS {
            let found = transcript[searched_from..].find(marker).unwrap_or_else(|| {
                panic!("`{marker}` is missing, or is out of order\n{transcript}")
            });
            searched_from += found + marker.len();
        }

        // The skew is the section's whole claim, and it is the one thing the
        // program cannot refuse on its own behalf: two views at one position is
        // a perfectly good state, just not the one being demonstrated.
        assert!(
            transcript.contains("stock_on_hand is live through 8")
                && transcript.contains("order_status is live through 7"),
            "the two views were never shown at two different checkpoints\n{transcript}"
        );

        // The promotion keeps the checkpoint, so the promoted view applies one
        // event rather than replaying the log it was backfilled over.
        assert!(
            transcript.contains("the promoted view applied 1"),
            "the promoted view replayed instead of resuming\n{transcript}"
        );
    }

    #[test]
    fn the_poisoned_view_stops_and_the_others_advance() {
        let transcript = transcript();

        // The cause, not the category. `CodecError`'s own message says only
        // that a payload would not decode; the field name is what tells a
        // reader this is a schema disagreement and not a corrupt log.
        assert!(
            transcript.contains("missing field `courier`"),
            "the decode failure did not name the field it was missing\n{transcript}"
        );

        // Committing nothing is the half that matters. A view that stopped but
        // moved its checkpoint would have skipped the event it failed on.
        assert!(
            transcript.contains("its checkpoint is never run"),
            "the poisoned view moved its checkpoint\n{transcript}"
        );

        // And the failure policy: the other three ran afterwards, in the same
        // process, against the same store.
        for view in [
            "stock_on_hand applied",
            "order_status applied",
            "daily_dispatches applied",
        ] {
            assert!(
                transcript.contains(view),
                "`{view}` never ran after the poisoned view failed\n{transcript}"
            );
        }
    }
}
