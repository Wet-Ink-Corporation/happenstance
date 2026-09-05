//! The demonstration is run, and its claims are read off what it printed.
//!
//! Running it is the only way to check any of this: `cargo test --workspace`
//! compiles a binary and never calls `main`, and every property here is about
//! two processes and fourteen sockets rather than about a type. That is why
//! `xtask/src/proof.rs` carries a row naming the two tests below.
//!
//! # Why the tests live in `mod runs`
//!
//! `proof.rs` matches fully-qualified test names out of `cargo test -- --list`,
//! and a bare top-level test lists without a module prefix and would never
//! match its row.
//!
//! # What is deliberately not asserted
//!
//! How many attempts the contended commits spent, and how many polls the view
//! took to catch up. Both depend on how the machine running this interleaved
//! its threads, and a test that pinned either would pass here and fail on a
//! busier or slower machine while nothing was wrong. The demonstration prints
//! both and says the same thing about them; what is asserted is the seat
//! count, which no interleaving is allowed to change.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod runs {
    use std::process::Command;

    /// The section markers, in the order the argument needs them.
    ///
    /// A sequence and not a set: "the view's process starts" only means
    /// something after it has been shown not to be running, and that only means
    /// something after writes have been accepted without it.
    const MARKERS: [&str; 7] = [
        "== the file both processes share ==",
        "== the write side starts, and only the write side ==",
        "== 8 clients ask for the same seat at once ==",
        "== 6 clients ask for 6 different seats ==",
        "== the view's process is not running ==",
        "== the view's process starts ==",
        "== what the view holds ==",
    ];

    /// Runs the demonstration and returns everything it printed.
    ///
    /// A test that asserts `status.success()` and discards the output makes
    /// every failure a re-run, so the panic carries both streams. It matters
    /// more here than elsewhere: a failure inside a child process arrives on
    /// this process's stderr and nowhere else.
    fn transcript() -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_tickets-over-http"))
            .output()
            .expect("the demonstration binary should be runnable");

        assert!(
            output.status.success(),
            "the demonstration exited with {}\n--- stdout ---\n{}\n--- stderr ---\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        String::from_utf8(output.stdout).expect("the transcript should be UTF-8")
    }

    #[test]
    fn two_processes_share_one_file() {
        let transcript = transcript();

        let mut searched_from = 0;
        for marker in MARKERS {
            let found = transcript[searched_from..].find(marker).unwrap_or_else(|| {
                panic!("`{marker}` is missing, or is out of order\n{transcript}")
            });
            searched_from += found + marker.len();
        }

        // The write side is up and the read side is not, and the API answers
        // with a checkpoint of `none` rather than with rows that would be
        // missing the caller's own write. This one is deterministic — the
        // runner has not been started even once, so there is no timing in it.
        assert!(
            transcript.contains("-> 202, checkpoint none"),
            "the API served a view that had never run\n{transcript}"
        );

        // And it does catch up, through a second process, over the same file.
        assert!(
            transcript.contains("200 after "),
            "the view never reached the position the client wrote at\n{transcript}"
        );
    }

    #[test]
    fn nothing_is_oversold_under_contention() {
        let transcript = transcript();

        // Eight clients, one seat, one winner. The program `bail!`s if two
        // clients are told they hold it, so this asserts the shape of the
        // answer rather than re-checking the count: seven refusals and not,
        // say, seven timeouts.
        assert!(
            transcript.contains("one 201 and 7 409s"),
            "the seat was not sold exactly once, cleanly\n{transcript}"
        );

        // Six clients, six seats, five in the show, one already gone. The
        // capacity boundary is contended by every one of them at once.
        assert!(
            transcript.contains("4 sold, 2 refused as full"),
            "the capacity was not enforced as the show ran out\n{transcript}"
        );

        // The invariant no interleaving may change.
        assert!(
            transcript.contains("5 seats held out of 5"),
            "the view does not hold exactly the show's capacity\n{transcript}"
        );
    }
}
