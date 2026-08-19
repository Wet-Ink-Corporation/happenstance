//! Every projection rule reaches all three emitters, through the single
//! enumeration and not through a list somebody kept.
//!
//! # The hole this closes, and why the gate could not
//!
//! The mandatory *wasm32 check of the conformance harnesses* step type-checks
//! `crates/happenstance-testkit/tests/` for `wasm32-unknown-unknown`
//! (`xtask/src/main.rs`), and that is the whole of AC-016's *mechanism*. It is
//! also blind to the failure AC-016 is about: **a harness that named three rules
//! by hand would type-check exactly as well as one that expands from
//! `for_each_projection_store_rule!`.** So *"the whole suite, not a separately
//! maintained subset"* could become false with nothing in the tree to say so, and
//! the way it becomes false is ordinary — an eighteenth rule is registered, two
//! harnesses are updated, the third is forgotten.
//!
//! A reviewed `rg` over the three files would find that today. It cannot find it
//! *again*: it is not re-run when the nineteenth rule lands, which is the
//! decorative-check shape this repository keeps naming. This is a test, so it is.
//!
//! # It is the mirror of `no_orphan_rules`, not a fork of it
//!
//! `registry::no_orphan_rules` obtains the registered names from
//! `for_each_event_store_rule!(crate::__emit_rule_names)` and compares them
//! against a source scan of `suite.rs`; a rule present in the source and absent
//! from the enumeration is an orphan. This asks the same question from the other
//! side: the enumerated names must be **absent** from every harness, because a
//! harness that contains one is a harness that wrote it down instead of expanding
//! it.
//!
//! # Why `include_str!` and not a directory walk
//!
//! The sources are read at compile time. A runtime walk would look for paths that
//! do not exist inside `cargo package`'s extracted tree — this crate is
//! published — and it would make the guard depend on the working directory a test
//! binary happens to be launched from. `include_str!` resolves against *this
//! file*, so it is correct wherever the binary runs and it fails at compile time
//! if a harness is deleted, which is itself the right answer.
//!
//! Host-only. Nothing here drives a store or awaits anything, so there is no
//! reason to pay for it on a target where the point is the *other* harness.

#![cfg(not(target_arch = "wasm32"))]

mod projection_harness_parity {
    /// The three harnesses AC-016 is about, each with the name a failure message
    /// has to be able to print.
    ///
    /// The buffering harness (`projection_conformance_buffering.rs`) is
    /// deliberately absent: it is the CF-5 second batch shape, one fixture rather
    /// than one emitter, and holding it to the same rule would say the emitter set
    /// is four.
    const HARNESSES: &[(&str, &str)] = &[
        (
            "projection_conformance.rs",
            include_str!("projection_conformance.rs"),
        ),
        (
            "projection_conformance_blocking.rs",
            include_str!("projection_conformance_blocking.rs"),
        ),
        (
            "projection_conformance_wasm.rs",
            include_str!("projection_conformance_wasm.rs"),
        ),
    ];

    /// Whether `haystack` contains `needle` on identifier boundaries.
    ///
    /// A substring match would fire on prose. `commit_is_atomic_with_the_read_model`
    /// appears in `projection_conformance.rs`'s module documentation as part of an
    /// English sentence about which mutant fails it, and that sentence is exactly
    /// the kind of thing this file exists to *keep*, not to forbid. What is
    /// forbidden is the name used as an **identifier** — the shape a hand-written
    /// `async fn commit_is_atomic_with_the_read_model()` has.
    ///
    /// So a hit only counts when the character before and after the match is
    /// neither alphanumeric, `_`, nor a backtick. The backtick is what excludes
    /// documentation: every rule name mentioned in prose in this repository is
    /// written `` `like_this` ``, which is the house convention the rustdoc
    /// obligations atom already requires.
    fn contains_identifier(haystack: &str, needle: &str) -> Option<usize> {
        let bytes = haystack.as_bytes();
        for (i, _) in haystack.match_indices(needle) {
            let before = i.checked_sub(1).map(|j| bytes[j]);
            let after = bytes.get(i + needle.len()).copied();
            let boundary = |c: Option<u8>| match c {
                None => true,
                Some(b) => !(b.is_ascii_alphanumeric() || b == b'_' || b == b'`'),
            };
            if boundary(before) && boundary(after) {
                return Some(haystack[..i].lines().count());
            }
        }
        None
    }

    /// No harness writes a rule name down.
    ///
    /// The wrong implementation this rejects is the one the type checker cannot
    /// see: a harness that enumerates the rules it wants instead of expanding the
    /// registry's own list. It fails by naming the harness, the rule and the line,
    /// because a guard whose message is a count gets `#[ignore]`d — and an
    /// `#[ignore]`d guard is then caught by the gate's proof-artefact step,
    /// loudly, at the worst possible time.
    #[test]
    fn no_harness_lists_a_rule_by_hand() {
        let registered = happenstance_testkit::for_each_projection_store_rule!(
            happenstance_testkit::__emit_rule_names
        );
        assert!(
            !registered.is_empty(),
            "the enumeration is empty, so this guard would pass against a harness \
             that listed every rule by hand"
        );

        let mut offences = Vec::new();
        for (name, source) in HARNESSES {
            for rule in registered {
                if let Some(line) = contains_identifier(source, rule) {
                    offences.push(format!("{name}:{line} names `{rule}`"));
                }
            }
        }

        assert!(
            offences.is_empty(),
            "a projection harness writes a rule name down instead of expanding \
             `for_each_projection_store_rule!`, so the rule set it runs is a list \
             somebody maintains rather than the enumeration. Offending sites: \
             {offences:#?}"
        );
    }

    /// Each harness invokes the suite exactly once.
    ///
    /// Two failures wearing one shape, and the message distinguishes them. **Zero**
    /// means the harness stopped covering the port at all — it still compiles, it
    /// still exits 0, and it runs nothing. **Two** means a second, hand-scoped
    /// invocation was added beside the generated one, which is the hand-listing
    /// failure wearing a macro: the extra call is where a maintained subset goes
    /// once this file forbids the obvious spelling.
    #[test]
    fn each_harness_invokes_the_suite_exactly_once() {
        for (name, source) in HARNESSES {
            let invocations = source
                .lines()
                .filter(|line| {
                    !line.trim_start().starts_with("//")
                        && line.contains("projection_store_conformance!")
                })
                .count();

            assert!(
                invocations != 0,
                "{name} carries no `projection_store_conformance!` invocation — it \
                 compiles, it exits 0, and it covers nothing"
            );
            assert_eq!(
                invocations, 1,
                "{name} carries {invocations} `projection_store_conformance!` \
                 invocations; a second one beside the generated call is a \
                 hand-scoped rule set wearing a macro"
            );
        }
    }
}
