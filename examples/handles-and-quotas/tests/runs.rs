//! The example is run, and its claims are read off the transcript it printed.
//!
//! `cargo test --workspace` compiles a binary and never calls `main`, so
//! without this target the three rules this example exists to demonstrate
//! would be checked by a compile. That is why `xtask/src/proof.rs` carries a
//! row naming the two tests below.
//!
//! # Why the tests live in `mod runs`
//!
//! `proof.rs` matches fully-qualified test names out of `cargo test -- --list`,
//! and a bare top-level test lists without a module prefix and would never
//! match its row.
//!
//! # What is asserted here, and what is asserted in the program
//!
//! `src/main.rs` `bail!`s when a decision it expects to be refused succeeds,
//! and when a command delivered twice appends twice. Those failures stop the
//! run rather than the diff, and this file does not restate them. What it adds
//! is what the program cannot check about itself: that the sections appear in
//! the order the argument depends on, and that each refusal names a value
//! rather than a category.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod runs {
    use std::process::Command;

    /// The section markers, in the order the argument needs them.
    ///
    /// A sequence and not a set. "A released name is claimable again" only
    /// means something after a name has been shown to be held by exactly one
    /// owner, and the boundary has to be printed before any of it.
    const MARKERS: [&str; 7] = [
        "== the database this run writes to ==",
        "== the boundary is a query, not an entity ==",
        "== a name is held by one owner ==",
        "== a released name is claimable again ==",
        "== a plan caps what an owner holds ==",
        "== one command, delivered twice ==",
        "== what the log actually says ==",
    ];

    /// Runs the example and returns everything it printed.
    ///
    /// A test that asserts `status.success()` and discards the output makes
    /// every failure a re-run, so the panic carries both streams.
    fn transcript() -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_handles-and-quotas"))
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
    fn the_binary_takes_every_decision() {
        let transcript = transcript();

        let mut searched_from = 0;
        for marker in MARKERS {
            let found = transcript[searched_from..].find(marker).unwrap_or_else(|| {
                panic!("`{marker}` is missing, or is out of order\n{transcript}")
            });
            searched_from += found + marker.len();
        }

        // The three scopes are the whole claim of the opening section: one
        // event set crossed with three different tag sets, which is what makes
        // three rules one boundary. A run that printed one item would still be
        // a working program and would have stopped demonstrating anything.
        for scope in ["{\"handle:alice\"}", "{\"owner:u-1\"}", "{\"request:r-1\"}"] {
            assert!(
                transcript.contains(scope),
                "the derived query never showed the scope {scope}\n{transcript}"
            );
        }

        // Each refusal carries values. `alice is held by u-1` is actionable
        // where it is printed; `conflict` would not be.
        for refusal in ["refused: alice is held by u-1", "refused: u-2 holds 2 of 2"] {
            assert!(
                transcript.contains(refusal),
                "`{refusal}` never happened\n{transcript}"
            );
        }
    }

    #[test]
    fn a_replayed_delivery_appends_nothing() {
        let transcript = transcript();

        // A replay restates the first delivery's outcome. Reporting it as a
        // conflict would be the defect: the client asked for a thing to be
        // true, and it is true.
        assert!(
            transcript.contains("already applied — claimed dave; nothing appended"),
            "the second delivery did not restate the first's outcome\n{transcript}"
        );

        // Seven decisions were taken and seven events were written — one of
        // the nine deliveries was refused and one was a replay, so a log of
        // eight would mean the replay had landed.
        assert!(
            transcript.contains("  7  HandleClaimed") && !transcript.contains("  8  "),
            "the log is not the seven events the run should have written\n{transcript}"
        );
    }
}
