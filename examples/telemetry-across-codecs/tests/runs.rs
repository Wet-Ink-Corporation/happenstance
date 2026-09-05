//! The example is run, and its claims are read off the transcript it printed.
//!
//! `cargo test --workspace` compiles a binary and never calls `main`, so
//! without this target the two migrations this example exists to demonstrate
//! would be checked by a compile. That is why `xtask/src/proof.rs` carries a
//! row naming the two tests below.
//!
//! # Why the tests live in `mod runs`
//!
//! `proof.rs` matches fully-qualified test names out of `cargo test -- --list`,
//! and a bare top-level test lists without a module prefix and would never
//! match its row.
//!
//! # Why the transcript is squeezed before it is matched
//!
//! The encoding table is column-aligned with `{:<22}`, so the number of spaces
//! between an event type and its codec depends on the length of the *longest*
//! type name. Asserting on that spacing would make adding a fifth event type
//! break a test that has nothing to do with event types. Collapsing runs of
//! spaces first keeps the assertion about what it is about.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod runs {
    use std::process::Command;

    /// The section markers, in the order the argument needs them.
    ///
    /// A sequence and not a set: "one fold reads both" is only a claim after
    /// two encodings have been shown to exist, and they only exist after a
    /// version has shipped and then been upgraded.
    const MARKERS: [&str; 6] = [
        "== the database this run writes to ==",
        "== the version that shipped first ==",
        "== the upgrade ==",
        "== one log, two encodings ==",
        "== one fold reads both ==",
        "== final log ==",
    ];

    /// Runs the example and returns everything it printed.
    ///
    /// A test that asserts `status.success()` and discards the output makes
    /// every failure a re-run, so the panic carries both streams.
    fn transcript() -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_telemetry-across-codecs"))
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

    /// The transcript with every run of spaces collapsed to one.
    fn squeezed(transcript: &str) -> String {
        transcript
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn the_binary_writes_two_encodings_and_reads_both() {
        let transcript = transcript();
        let table = squeezed(&transcript);

        let mut searched_from = 0;
        for marker in MARKERS {
            let found = transcript[searched_from..].find(marker).unwrap_or_else(|| {
                panic!("`{marker}` is missing, or is out of order\n{transcript}")
            });
            searched_from += found + marker.len();
        }

        // Two encodings in one store is the premise. The program `bail!`s if
        // it finds fewer than two, so this asserts *which* two — a run that
        // wrote postcard twice would satisfy the program's own check.
        for framed in [
            "DeviceRegistered json",
            "TemperatureReported json",
            "FirmwareUpgraded postcard",
            "TemperatureReportedV2 postcard",
        ] {
            assert!(
                table.contains(framed),
                "the log never held `{framed}`\n{transcript}"
            );
        }

        // One codec is named at the read, and it is the one that did *not*
        // write the postcard events.
        assert!(
            transcript.contains("one codec was named here: Json"),
            "the fold was not shown reading with a single codec\n{transcript}"
        );
    }

    #[test]
    fn the_old_shape_arrives_upcast() {
        let transcript = transcript();

        // The whole schema-migration claim in one line. The first three
        // readings were written in whole degrees by a type that has no
        // `millidegrees` field; the last two were written in thousandths. They
        // come back in one unit, in order, from one fold.
        assert!(
            transcript.contains("readings:   [21000, 23000, 22000, 22500, 21250]"),
            "the readings did not arrive in one unit\n{transcript}"
        );

        // The old event type is still in the log under its old name. If a
        // migration had rewritten it, the upcast would be demonstrating
        // nothing.
        assert!(
            transcript.contains("TemperatureReportedV2"),
            "nothing was written under the new event type\n{transcript}"
        );
    }
}
