//! `Retry::attempts` is spelled with the named constant at every call site.
//!
//! # Why this lives in `tests/` rather than beside the code it checks
//!
//! It was a `#[cfg(test)]` test inside `src/command.rs`, and it reads two of the
//! workspace's example crates. `include_str!` resolves at compile time against
//! the file's own directory, so those paths reach **outside this package** — and
//! a file under `src/` ships in the `.crate` whether or not its tests compile
//! there. No `exclude` can reach it, because the file *is* the library.
//!
//! Nothing caught that: `cargo package --list` never compiles, and the
//! verification build compiles the library, not its tests. So the tarball built
//! cleanly while carrying a `#[cfg(test)]` block that cannot, and the person who
//! finds it is whoever runs `cargo test` on an unpacked crate.
//!
//! Moving it here preserves the assertion exactly — same needles, same four
//! sources — and puts it in a file `Cargo.toml` excludes. The alternative
//! considered and rejected was a runtime `fs::read_to_string` with a skip, which
//! makes the test vacuous in the workspace the moment a path is wrong.
//!
//! `tests/boundary_refusal.rs` states the rule this obeys: "`include_str!`
//! cannot cross the package boundary of a published crate".

/// No call site converts a literal at run time when const would do.
///
/// `Retry::attempts` is `const fn` (above), so nothing that hands it a
/// literal needs `.try_into()?` at run time — that spelling converts a
/// value the compiler already knows is nonzero, through a path that can
/// fail, for a failure that cannot happen. This walks the source of
/// every call site this crate owns (its own two doctests, and the two
/// worked examples) and fails if any of them still reach for the
/// fallible spelling where the const one is available. Needles are built
/// with `format!` rather than written as one literal, so this test's own
/// source text never contains the pattern it is searching for.
#[test]
fn call_sites_use_the_const_spelling_of_retry_attempts() {
    let doctest_literal = format!("Retry::attempts({}.try_into()?)", 3);
    let example_constant = format!("Retry::attempts({}.try_into()?)", "ATTEMPTS");
    let test_literal = format!("Retry::attempts({}.try_into()", 7);

    let this_command_rs = include_str!("../src/command.rs");
    let this_lib_rs = include_str!("../src/lib.rs");
    let course_subscriptions = include_str!("../../../examples/course-subscriptions/src/main.rs");
    let transfers_on_sqlite = include_str!("../../../examples/transfers-on-sqlite/src/main.rs");

    let sources: [(&str, &str, &[&str]); 4] = [
        (
            "crates/happenstance/src/command.rs",
            this_command_rs,
            &[&doctest_literal, &test_literal],
        ),
        (
            "crates/happenstance/src/lib.rs",
            this_lib_rs,
            &[&doctest_literal],
        ),
        (
            "examples/course-subscriptions/src/main.rs",
            course_subscriptions,
            &[&example_constant],
        ),
        (
            "examples/transfers-on-sqlite/src/main.rs",
            transfers_on_sqlite,
            &[&example_constant],
        ),
    ];

    let mut offenders = Vec::new();
    for (path, src, needles) in sources {
        for needle in needles {
            if src.contains(needle) {
                offenders.push(format!("{path}: still contains {needle:?}"));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "call site converts a compile-time literal at run time against a \
         const fn; use the const spelling instead:\n{}",
        offenders.join("\n")
    );
}
