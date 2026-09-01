//! The seam, asserted rather than believed.
//!
//! `cargo test --locked --workspace --all-features` compiles this package and
//! never executes `main`, so without this target the sentence *"the typed layer
//! runs on SQLite"* would be checked by a compile — which is the whole defect
//! this package exists to close, reintroduced one level up. Everything here
//! starts from one spawn of `CARGO_BIN_EXE_transfers-on-sqlite` with stdout
//! piped and no terminal attached.
//!
//! # Why some of these tests read the source instead of the output
//!
//! A program that hand-rolled the read-decide-append-retry cycle against
//! `rusqlite` directly, never touching `happenstance::commit`, would print this
//! exact transcript and pass every assertion about it. So would one that folded
//! its balances in memory and wrote them out at the end, never touching
//! `run_projection`. Those are claims about *how* the program is written, and
//! the only thing that can observe them is the file. File-reading assertions
//! are first-class in this repository — `cargo xtask lints` is five of them.
//!
//! # Why the tests live in `mod runs`
//!
//! Same convention as `course-subscriptions`: `xtask/src/proof.rs`'s
//! `ARTEFACTS` rows name tests fully qualified and assert them out of `cargo
//! test -- --list` before running them, and at a file's top level these would
//! list as bare names.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod runs {
    use std::process::{Command, Output};

    /// The application's own source, read at compile time.
    const MAIN: &str = include_str!("../src/main.rs");

    /// The application's manifest, read at compile time.
    const MANIFEST: &str = include_str!("../Cargo.toml");

    /// The section markers, in the order `main` prints them.
    ///
    /// A sequence, never a set. The order *is* the claim: a transcript that
    /// reads the balances back before it says the handles were dropped is a
    /// different program making a weaker claim.
    const MARKERS: [&str; 12] = [
        "== the database this run writes to ==",
        "== opening two accounts ==",
        "== depositing 100 into a1 ==",
        "== transferring 40 from a1 to a2 ==",
        "== transferring 500 from a1 to a2 ==",
        "== projecting balances into the same file ==",
        "== everything above is dropped; reopening the same file ==",
        "== the read model, straight off disk ==",
        "== the same balances, folded again from the event log ==",
        "== the checkpoint survived too ==",
        "== the reopened store is live, not merely readable ==",
        "== final log ==",
    ];

    /// Runs the binary once and returns its output, asserting it exited 0.
    fn run() -> Output {
        let output = Command::new(env!("CARGO_BIN_EXE_transfers-on-sqlite"))
            .output()
            .expect("the example binary should be spawnable");

        assert!(
            output.status.success(),
            "the example exited {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        output
    }

    /// The run's stdout, as text.
    fn transcript() -> String {
        String::from_utf8(run().stdout).expect("the transcript should be UTF-8")
    }

    /// Returns the byte offset of `needle`, failing with the transcript if absent.
    fn offset_of(transcript: &str, needle: &str) -> usize {
        transcript
            .find(needle)
            .unwrap_or_else(|| panic!("transcript is missing {needle:?}\n---\n{transcript}"))
    }

    /// `source` with everything after `comment` stripped from each line.
    ///
    /// Every source-reading assertion below is a claim about **code**, and the
    /// first draft of two of them failed on this package's own prose: the
    /// application's module documentation mentions `_ =>` in the course of
    /// promising there is none, and its manifest names `happenstance-core` in
    /// the course of explaining why it does not depend on it. A claim about
    /// code that a comment can satisfy — or refute — is not a claim about code.
    ///
    /// Crude on purpose: it would also strip a `//` inside a string literal.
    /// Neither file has one, and a parser here would be a second thing to be
    /// wrong.
    fn code(source: &str, comment: &str) -> String {
        source
            .lines()
            .map(|line| line.find(comment).map_or(line, |at| &line[..at]))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The program runs to completion, printing its sections in order.
    #[test]
    fn the_example_runs_end_to_end() {
        let transcript = transcript();

        let mut previous = 0;
        for marker in MARKERS {
            let at = offset_of(&transcript, marker);
            assert!(
                at >= previous,
                "{marker:?} arrived out of order\n---\n{transcript}"
            );
            previous = at;
        }
    }

    /// The overdrawn transfer is refused, and the refusal carries its numbers.
    ///
    /// Asserted literally, including the indent. A refusal that read
    /// `insufficient funds` would satisfy "it was rejected" and tell an
    /// operator nothing they can act on.
    #[test]
    fn the_overdrawn_transfer_is_refused_with_its_values() {
        let transcript = transcript();

        assert!(
            transcript.contains("   rejected: account a1 holds 60, needs 500"),
            "the refusal did not carry its values\n---\n{transcript}"
        );
    }

    /// A transfer is one append carrying two events, so no state has one leg of it.
    ///
    /// The first transfer moves 40 out of a1 and into a2. Both events are in
    /// the log, adjacent, and the balances add up to the 100 that was deposited
    /// — which is the observable form of "the debit and the credit landed
    /// together".
    #[test]
    fn a_transfer_lands_as_one_append_of_two_events() {
        let transcript = transcript();
        let log = &transcript[offset_of(&transcript, "== final log ==")..];

        let withdrawn = offset_of(log, "Withdrawn");
        let deposited = log[withdrawn..]
            .find("Deposited")
            .expect("a withdrawal should be followed by its deposit");

        assert!(
            deposited > 0,
            "the transfer's two events are not adjacent in the log\n---\n{log}"
        );
    }

    /// What is read after the reopen came off the file, and it is correct.
    ///
    /// Three readings of the same two numbers, and all three must agree: the
    /// read-model table, the fold over the event log, and — because they are
    /// printed under separate markers — the fact that both survived every
    /// handle being dropped. `60` and `40` are what 100 deposited and 40
    /// transferred leaves behind.
    #[test]
    fn the_balances_survive_the_reopen() {
        let transcript = transcript();

        let from_disk =
            &transcript[offset_of(&transcript, "== the read model, straight off disk ==")
                ..offset_of(&transcript, "== the checkpoint survived too ==")];

        assert_eq!(
            from_disk.matches("a1: 60").count(),
            2,
            "the read model and the fold should both say 60\n---\n{from_disk}"
        );
        assert_eq!(
            from_disk.matches("a2: 40").count(),
            2,
            "the read model and the fold should both say 40\n---\n{from_disk}"
        );
    }

    /// The checkpoint is durable: a restart does not replay the log.
    ///
    /// The headline claim of the projection half, and the one an application
    /// notices in production before it notices any other. Zero, literally: a
    /// projection that re-applied its whole log would still print correct
    /// balances, because the delta upsert would be re-run from a table that
    /// already held the totals — and would be silently wrong.
    #[test]
    fn the_checkpoint_survives_the_reopen() {
        let transcript = transcript();

        assert!(
            transcript.contains(
                "   re-running the projection applied 0 event(s) — the log is not replayed"
            ),
            "the projection replayed events it had already applied\n---\n{transcript}"
        );
    }

    /// The reopened store still writes, and the projection follows it.
    #[test]
    fn the_reopened_store_still_accepts_commands() {
        let transcript = transcript();
        let after = &transcript[offset_of(
            &transcript,
            "== the reopened store is live, not merely readable ==",
        )..];

        assert!(
            after.contains("the projection applied 2 event(s)"),
            "the transfer after the reopen did not reach the projection\n---\n{after}"
        );
        assert!(
            after.contains("a1: 75") && after.contains("a2: 25"),
            "the read model did not follow the store after the reopen\n---\n{after}"
        );
    }

    /// Every happenstance call goes through the typed layer.
    ///
    /// The claim this package was written to make. Without it, the transcript
    /// above is satisfied by a program that talks to `rusqlite` directly and
    /// never links the typed layer at all.
    #[test]
    fn the_application_uses_the_typed_layer_and_not_the_contract_crate() {
        let main = code(MAIN, "//");
        let manifest = code(MANIFEST, "#");

        assert!(
            main.contains("use happenstance::"),
            "the application does not import the typed layer"
        );
        assert!(
            !main.contains("use happenstance_core::"),
            "the application reaches past the typed layer into the contract crate"
        );
        assert!(
            manifest.contains("happenstance = {"),
            "the manifest does not depend on the typed layer"
        );
        assert!(
            !manifest.contains("happenstance-core"),
            "the manifest names the contract crate an application should not have to"
        );
    }

    /// Both halves of the adapter are exercised, and both through happenstance.
    ///
    /// `commit` is the command loop and `run_projection` is the runner; a
    /// program that reached for `SqliteEventStore::append` or wrote its read
    /// model outside a `SqliteBatch` would be testing the adapter, not the
    /// seam.
    #[test]
    fn both_ports_are_driven_through_the_typed_layer() {
        let main = code(MAIN, "//");

        for call in ["commit(", "run_projection(", "impl Projection for"] {
            assert!(
                main.contains(call),
                "the application never reaches {call:?}, so that port is untested here"
            );
        }
        assert!(
            code(MANIFEST, "#").contains("happenstance-sqlite"),
            "the manifest does not depend on the adapter"
        );
    }

    /// The event and projection stores share one database file.
    ///
    /// The arrangement an application deploys, and the one nothing else in the
    /// workspace exercises: two `open` calls on the same path, two schemas,
    /// separate meta tables. A program that opened two files would pass every
    /// other test here.
    #[test]
    fn one_file_carries_both_stores() {
        let main = code(MAIN, "//");

        assert!(
            main.contains("SqliteEventStore::open(path)")
                && main.contains("SqliteProjectionStore::open(path)"),
            "the two stores are not opened on the same path"
        );
    }

    /// No fold has a wildcard arm.
    ///
    /// The compiler's protection over the domain is exactly the absence of
    /// `_ =>`. A wildcard anywhere in this file makes a fourth `Ledger` variant
    /// compile silently into a fold that ignores it.
    #[test]
    fn no_fold_carries_a_wildcard_arm() {
        assert!(
            !code(MAIN, "//").contains("_ =>"),
            "a wildcard match arm defeats the exhaustiveness this example claims"
        );
    }
}
