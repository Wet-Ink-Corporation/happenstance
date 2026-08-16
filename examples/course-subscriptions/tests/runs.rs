//! The instrument the initiative's first Definition-of-Done item was missing:
//! something that actually **runs** the example.
//!
//! `cargo test --locked --workspace --all-features` compiles this package and
//! never executes `main`, so until this target existed the sentence *"the worked
//! example runs end to end"* was checked by a compile. Everything here starts
//! from one spawn of `CARGO_BIN_EXE_course-subscriptions` with stdout piped and
//! no terminal attached — the state the gate actually runs it in.
//!
//! # Why half of these tests read the source instead of the output
//!
//! A program that does the same appends and prints only its final log exits 0,
//! reaches no `todo!()`, and satisfies every "it ran" assertion. So does one
//! that hand-rolls the read-decide-append-retry cycle, or names its event set
//! twice, or scrapes a capacity out of bytes. Those are claims about *how* the
//! example is written, and the only thing that can observe them is the file.
//! File-reading assertions are first-class in this repository — `cargo xtask
//! lints` is five of them — and keeping these beside the execution test rather
//! than adding a sixth `xtask` lint keeps one target answering for one story.
//!
//! # Why the tests live in `mod runs`
//!
//! `xtask/src/proof.rs`'s `ARTEFACTS` row names the tests inside this target,
//! fully qualified, and asserts them out of `cargo test -- --list` before it
//! runs them. At the file's top level these would list as bare names and the
//! row could never match. Same convention as `happenstance-core`'s `wire`
//! target and the testkit's `mutation_coverage`.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod runs {
    use std::process::{Command, Output};

    /// The example's own source, read at compile time.
    const MAIN: &str = include_str!("../src/main.rs");

    /// The example's manifest, read at compile time.
    const MANIFEST: &str = include_str!("../Cargo.toml");

    /// The seven section markers, in the order `main` prints them.
    ///
    /// A sequence, never a set: the order is the behaviour AC-001 asserts, and
    /// a transcript carrying all seven in the wrong order is a different run.
    const MARKERS: [&str; 7] = [
        "== defining course c1 with capacity 2 ==",
        "== defining course c1 again ==",
        "== subscribing s1 and s2 ==",
        "== subscribing s1 again ==",
        "== subscribing s3, which would exceed capacity ==",
        "== s1 unsubscribes, freeing a seat ==",
        "== final log ==",
    ];

    /// The three refusals the run must produce, with their carried values.
    ///
    /// Asserted literally, including the three-space indent and the `rejected: `
    /// prefix, because a refusal that reads as a category (`capacity exceeded`)
    /// tells a reader nothing they can act on.
    const REFUSALS: [&str; 3] = [
        "   rejected: course c1 is already defined",
        "   rejected: student s1 is already subscribed to c1",
        "   rejected: course c1 is full (2/2)",
    ];

    /// Every event type the log may hold.
    const EVENT_TYPES: [&str; 3] = ["CourseDefined", "StudentSubscribed", "StudentUnsubscribed"];

    /// The three command handlers, by the header each is declared under.
    const HANDLERS: [&str; 3] = [
        "async fn define_course(",
        "async fn subscribe(",
        "async fn unsubscribe(",
    ];

    /// The transcript's line budget, from the signed-off design.
    const MAX_LINES: usize = 45;
    /// The column budget for any transcript line.
    const MAX_COLUMNS: usize = 80;
    /// The column budget for a section marker.
    const MAX_MARKER_COLUMNS: usize = 60;
    /// The width of the final log's event-type field.
    const TYPE_FIELD: usize = 22;

    // -----------------------------------------------------------------------
    // The one spawn every test reads from
    // -----------------------------------------------------------------------

    /// Runs the compiled binary with stdout piped, and captures everything.
    fn execute() -> (Output, String) {
        let output = Command::new(env!("CARGO_BIN_EXE_course-subscriptions"))
            .output()
            .expect("cargo builds the example's binary before this target runs");
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        (output, stdout)
    }

    /// Fails with the whole captured run attached.
    ///
    /// A test that asserts `status.success()` and discards the output makes
    /// every failure a re-run. The diagnosis belongs in the message.
    fn report(output: &Output, stdout: &str, what: &str) -> ! {
        panic!(
            "{what}\n\n--- status ---\n{}\n\n--- stdout ---\n{stdout}\n--- stderr ---\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // -----------------------------------------------------------------------
    // Source scanners
    // -----------------------------------------------------------------------

    /// The `open`/`close`-balanced group that follows `opening`.
    fn group<'a>(source: &'a str, opening: &str, open: u8, close: u8) -> &'a str {
        let at = source.find(opening).unwrap_or_else(|| {
            panic!("`{opening}` is not in examples/course-subscriptions/src/main.rs")
        });
        let start = at
            + source[at..]
                .bytes()
                .position(|byte| byte == open)
                .unwrap_or_else(|| panic!("`{opening}` opens no group"));

        let mut depth = 0usize;
        for (offset, byte) in source[start..].bytes().enumerate() {
            if byte == open {
                depth += 1;
            } else if byte == close {
                depth -= 1;
                if depth == 0 {
                    return &source[start..=start + offset];
                }
            }
        }
        panic!("`{opening}`'s group is never closed");
    }

    /// The brace-matched body that follows the first `header`.
    fn block<'a>(source: &'a str, header: &str) -> &'a str {
        group(source, header, b'{', b'}')
    }

    /// Every brace-matched body declared under `header`.
    fn blocks<'a>(source: &'a str, header: &str) -> Vec<&'a str> {
        let mut found = Vec::new();
        let mut from = 0usize;
        while let Some(offset) = source[from..].find(header) {
            let start = from + offset;
            found.push(block(&source[start..], header));
            from = start + header.len();
        }
        found
    }

    /// The `== final log ==` region: `{:>3}  {:<22} {:?}`, one row per event.
    ///
    /// The region is *recessive* by column alignment rather than by prose, so
    /// the assertions are column assertions: the position and the event type
    /// each occupy their own field and neither is ever truncated to fit. It is
    /// also the non-occlusion check — the log prints every event the store
    /// holds, not the subset any one boundary selected, so a fold that
    /// disagreed with its query would show up here as a row nobody accounted
    /// for.
    fn assert_log_region(rows: &[&str]) {
        assert!(
            !rows.is_empty(),
            "`== final log ==` is the last thing printed"
        );

        let mut previous = 0u64;
        for row in rows {
            assert!(
                row.starts_with("   "),
                "a log row is not indented three: {row:?}"
            );

            let position: u64 = row
                .get(3..6)
                .unwrap_or_else(|| panic!("a log row has no position column: {row:?}"))
                .trim()
                .parse()
                .unwrap_or_else(|_| panic!("a log row's position is not a number: {row:?}"));
            assert!(
                position > previous,
                "the log is not in the store's own order: {row:?}"
            );
            previous = position;

            assert_eq!(
                row.get(6..8),
                Some("  "),
                "the position column is not padded"
            );

            let field = row
                .get(8..8 + TYPE_FIELD)
                .unwrap_or_else(|| panic!("a log row has no event-type column: {row:?}"));
            let event_type = field.trim_end();
            assert!(
                EVENT_TYPES.contains(&event_type),
                "`{event_type}` is not one of this example's event types, or it was truncated"
            );

            assert_eq!(
                row.get(8 + TYPE_FIELD..9 + TYPE_FIELD),
                Some(" "),
                "the event-type column is not padded to {TYPE_FIELD}"
            );
            let tags = &row[9 + TYPE_FIELD..];
            assert!(
                tags.starts_with('{') && tags.ends_with('}'),
                "a log row's tag set is not rendered: {row:?}"
            );
        }

        assert_eq!(
            rows.len(),
            5,
            "the log does not hold every event the run accepted"
        );
    }

    // -----------------------------------------------------------------------
    // AC-001 — the gate executes the binary
    // -----------------------------------------------------------------------

    /// The whole canonical DCB cycle, run by the gate rather than by a human.
    ///
    /// The wrong implementation it rejects is the one that shipped for two
    /// phases: a workspace test run that *builds* the example and never runs
    /// it, under a Definition-of-Done item written with execution verbs.
    #[test]
    fn the_binary_completes_the_dcb_cycle() {
        let (output, stdout) = execute();

        if !output.status.success() {
            report(&output, &stdout, "the example did not exit 0");
        }
        if !output.stderr.is_empty() {
            report(&output, &stdout, "the example wrote to stderr");
        }

        let mut cursor = 0usize;
        for marker in MARKERS {
            match stdout[cursor..].find(marker) {
                Some(at) => cursor += at + marker.len(),
                None => report(
                    &output,
                    &stdout,
                    &format!("`{marker}` is missing, or arrives out of order"),
                ),
            }
        }

        for refusal in REFUSALS {
            if !stdout.lines().any(|line| line == refusal) {
                report(&output, &stdout, &format!("`{refusal}` never appeared"));
            }
        }
    }

    // -----------------------------------------------------------------------
    // AC-002 — the transcript is the signed-off surface, inside its budget
    // -----------------------------------------------------------------------

    /// The composition at `main.rs`'s observable steps, asserted literally.
    ///
    /// The unstyled render this rejects is a program that does the same appends
    /// and prints only the final log: it satisfies every assertion in
    /// [`the_binary_completes_the_dcb_cycle`] and none of these.
    #[test]
    fn the_transcript_is_the_designed_composition() {
        let (output, stdout) = execute();
        let lines: Vec<&str> = stdout.lines().collect();

        let marker_at: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.starts_with("=="))
            .map(|(index, _)| index)
            .collect();

        if marker_at.len() != MARKERS.len() {
            report(
                &output,
                &stdout,
                &format!(
                    "{} section markers, expected {}",
                    marker_at.len(),
                    MARKERS.len()
                ),
            );
        }

        for (nth, (&at, marker)) in marker_at.iter().zip(MARKERS).enumerate() {
            assert_eq!(lines[at], marker, "marker {nth} is not the designed one");
            if nth == 0 {
                assert_eq!(at, 0, "the first marker is not the first line");
            } else {
                assert_eq!(
                    lines[at - 1],
                    "",
                    "no blank line precedes the marker `{marker}`"
                );
            }
        }

        let final_log = marker_at[MARKERS.len() - 1];

        // Everything above the log: a result line, indented exactly three.
        for line in &lines[..final_log] {
            if line.is_empty() || line.starts_with("==") {
                continue;
            }
            assert!(
                line.starts_with("   ") && !line[3..].starts_with(' '),
                "a result line is not indented exactly three spaces: {line:?}"
            );
        }

        let refused: Vec<&str> = lines
            .iter()
            .copied()
            .filter(|line| line.trim_start().starts_with("rejected: "))
            .collect();
        assert_eq!(
            refused, REFUSALS,
            "the refusals are not the three the design signed off, with their values"
        );

        // The log region: one row per event the store holds, column-aligned.
        assert_log_region(&lines[final_log + 1..]);
    }

    /// The density budget, counted over the real captured stdout.
    #[test]
    fn the_transcript_holds_its_budget() {
        let (_, stdout) = execute();

        assert!(
            !stdout.contains('\u{1b}'),
            "anti-pattern 9: the transcript carries an ANSI escape"
        );
        assert!(
            !stdout.contains('\r'),
            "anti-pattern 9: the transcript rewrites a line in place"
        );

        let lines: Vec<&str> = stdout.lines().collect();
        assert!(
            lines.len() <= MAX_LINES,
            "the transcript is {} lines, over its budget of {MAX_LINES}",
            lines.len()
        );

        for line in &lines {
            let columns = line.chars().count();
            assert!(
                columns <= MAX_COLUMNS,
                "a transcript line runs to {columns} columns: {line:?}"
            );
            if line.starts_with("==") {
                assert!(
                    columns <= MAX_MARKER_COLUMNS,
                    "a section marker runs to {columns} columns: {line:?}"
                );
            }
        }

        for name in EVENT_TYPES {
            assert!(
                name.len() <= TYPE_FIELD,
                "`{name}` is wider than the {TYPE_FIELD}-column event-type field"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-003 — the example installs the crate a user installs
    // -----------------------------------------------------------------------

    /// The manifest names `happenstance`, and nothing below it.
    ///
    /// Read with the comment lines dropped, because the manifest is allowed to
    /// *explain* the move off `happenstance-core` — what it may not do is still
    /// declare it.
    #[test]
    fn the_example_depends_only_on_the_typed_layer() {
        let entries: Vec<&str> = MANIFEST
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect();

        assert!(
            !entries
                .iter()
                .any(|line| line.contains("happenstance-core")),
            "the example still depends on the crate adapter authors pin"
        );

        let dependency = entries
            .iter()
            .find(|line| line.trim_start().starts_with("happenstance "))
            .expect("the manifest names `happenstance`");
        for feature in ["\"std\"", "\"memory\"", "\"json\""] {
            assert!(
                dependency.contains(feature),
                "the `happenstance` dependency does not name {feature}: {dependency}"
            );
        }

        assert!(
            entries
                .iter()
                .any(|line| line.trim_start().starts_with("serde")),
            "the domain enum needs `serde`, and the manifest does not name it"
        );

        assert!(
            !MAIN.contains("happenstance_core"),
            "a `happenstance_core::` path survives in the example's source"
        );
    }

    // -----------------------------------------------------------------------
    // AC-004 — the three constructs are deleted, not wrapped
    // -----------------------------------------------------------------------

    /// `parse_capacity`, the hand-built payload and the private `commit`.
    #[test]
    fn the_deleted_constructs_are_gone() {
        for gone in [
            "parse_capacity",
            "into_bytes",
            r#"\"capacity\":"#,
            r#"b"{}""#,
            "async fn commit",
        ] {
            assert!(
                !MAIN.contains(gone),
                "`{gone}` survives in the example — it was renamed or wrapped, not deleted"
            );
        }
    }

    /// The capacity the refusal prints is the one that was decoded.
    ///
    /// This is the behavioural discriminator for the deletion: an
    /// `unwrap_or(0)`-shaped decode makes the *first* subscribe refuse `(0/0)`,
    /// so a wrong implementation cannot reach the right transcript.
    #[test]
    fn capacity_refusal_carries_the_decoded_capacity() {
        let (output, stdout) = execute();
        if !stdout.contains("rejected: course c1 is full (2/2)") {
            report(
                &output,
                &stdout,
                "the capacity refusal does not carry the decoded capacity",
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 — the event set is named once, in the user's own file
    // -----------------------------------------------------------------------

    /// The query is derived; the fold is exhaustive over the domain enum.
    #[test]
    fn the_event_set_is_named_once() {
        for gone in ["QueryItem::new", "Query::from_items", "Query::from_item("] {
            assert!(
                !MAIN.contains(gone),
                "`{gone}` survives: the event set is still named a second time by hand"
            );
        }

        assert!(
            !MAIN
                .lines()
                .any(|line| line.contains("match ") && line.contains("event_type()")),
            "the fold still matches on a stringly-typed event type"
        );

        let folds = blocks(MAIN, "fn apply(");
        assert_eq!(
            folds.len(),
            3,
            "the example does not declare three decision-model folds"
        );
        for fold in folds {
            assert!(
                !fold.contains("_ =>"),
                "a fold carries a wildcard arm, so a new variant would be silent"
            );
        }
    }

    /// The enum and its folds are ordinary code in the file the user owns.
    ///
    /// The slice-mate's compile-fail diagnostic has to point at *this* file's
    /// `match` arm; an implementation that moved the fold behind a macro or a
    /// helper trait would send the `-->` span somewhere the reader cannot act.
    #[test]
    fn the_fold_lives_in_the_users_file() {
        assert!(
            MAIN.contains("enum Enrolment"),
            "the domain enum is not declared here"
        );
        assert!(
            MAIN.contains("impl DomainEvent for Enrolment"),
            "the `DomainEvent` impl is not written here"
        );
        assert_eq!(
            MAIN.matches("impl DecisionModel for ").count(),
            3,
            "the example does not declare three decision models"
        );
        assert!(
            !MAIN.contains("macro_rules!"),
            "the mapping is hidden behind a local macro, which AC-013 is measured against"
        );
    }

    // -----------------------------------------------------------------------
    // AC-006 — `subscribe` composes two models
    // -----------------------------------------------------------------------

    /// Two decision models, passed as a tuple, with no macro in the caller's face.
    #[test]
    fn subscribe_composes_two_models() {
        for model in [
            "impl DecisionModel for Seats",
            "impl DecisionModel for StudentSeat",
        ] {
            assert!(MAIN.contains(model), "`{model}` is missing");
        }

        let body = block(MAIN, "async fn subscribe(");
        let boundary = group(body, "let boundary = (", b'(', b')');
        assert!(
            boundary.contains("Seats::new"),
            "the seats model is not in `subscribe`'s boundary"
        );
        assert!(
            boundary.contains("StudentSeat::new"),
            "the per-student model is not in `subscribe`'s boundary: \
             the two concerns were collapsed into one"
        );
        assert_eq!(
            body.matches("commit(").count(),
            1,
            "`subscribe` does not commit exactly once"
        );
    }

    // -----------------------------------------------------------------------
    // AC-007 — the command loop absorbed every handler
    // -----------------------------------------------------------------------

    /// No handler re-derives read, decide, append or retry.
    #[test]
    fn the_loop_absorbs_every_handler() {
        for gone in ["AppendCondition", "read_decision_model", "AppendError"] {
            assert!(
                !MAIN.contains(gone),
                "`{gone}` survives: a handler still spells the cycle out by hand"
            );
        }

        assert_eq!(
            MAIN.matches("commit(").count(),
            3,
            "the example does not have exactly three commit call sites"
        );

        for handler in HANDLERS {
            let body = block(MAIN, handler);
            assert!(
                body.contains("commit("),
                "`{handler}` does not go through the command loop"
            );
            assert!(
                body.contains("Retry::"),
                "`{handler}` does not spell its retry bound at the call site"
            );
        }
    }
}
