---
item: HS-S0079
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Read-your-own-writes inside one batch, answered

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, because they decide what "evidence" may be:

- **Evidence for AC-001, AC-002, AC-003, AC-005 and AC-007 must cite a run**, by target and fully
  qualified test name out of the `ARTEFACTS` row (`xtask/src/proof.rs:133`), plus the commit. An
  answer written from `crates/happenstance-ladybug/src/projection_store.rs:49-64` is the third named
  wrong implementation and has nothing to cite (`discover.md`, *The wrong implementation*).
- **`<conformance target>` is a placeholder on purpose.** The target's file name is HS-S0078's and is
  fixed by ADR-0025's Q3 emitter arm; the implementer replaces it here with the real name once it
  exists, which is a substitution of a known-unknown, not a re-wording of a criterion.

```yaml
- id: AC-001
  criterion: >-
    GIVEN the reviewer who has been told the deferred write set is the one axis a projection can
    plausibly fail against, and who will not accept "the mechanism implies…" as an observation, WHEN
    they run `cargo xtask ci` on this branch and read the `tests` step's captured output for the
    Ladybug conformance target, THEN a projection built from two distinct, ordered `GraphStatement`s
    in one `GraphWriteSet` — the second `MATCH`ing a node only the first creates — has been executed
    through a single `commit` against the real `lbug` engine inside the target HS-S0078 registered,
    and its result is in the scroll they are already reading. A single statement doing both is not
    this case and does not discharge it.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "crates/happenstance-ladybug/tests/<conformance target>.rs — the two-statement read-your-own-writes case, run by cargo xtask ci's tests step with --show-output (xtask/src/main.rs:131-151)"

- id: AC-002
  criterion: >-
    GIVEN the same reviewer, who knows the natural Cypher spelling collapses both halves into one
    statement and would then be reading a fact about the query engine rather than about the batch,
    WHEN they read the record, THEN a single-statement control has run in the same invocation, its
    outcome is written down before the two-statement result is interpreted, and the two are compared:
    if they diverge, the case discriminates statement-local from cross-statement visibility and
    AC-003's answer stands on it; if they agree, the recorded finding is "this case cannot distinguish
    the two mechanisms and is not evidence" — never a reason to retry, reword or delete the control.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "crates/happenstance-ladybug/tests/<conformance target>.rs — the single-statement control case, on its own LadybugFixture instance, same cargo xtask ci tests-step invocation; divergence check recorded in .bklg/from-contract-to-published-library/ladybug-projection-store/read-your-own-writes-projection/"

- id: AC-003
  criterion: >-
    GIVEN the reviewer opening the freeze verdict expecting the one line that could not have been
    written from the design, WHEN they read this story's recorded outcome, THEN it says exactly one of
    PS-12's three shapes and says which: (1) the traversal observed the pending node ⇒ supported
    behaviour, PS-4's second condition survives; (2) the `Batch` exposes no read path at all ⇒ stated
    limit, PS-12's own compliant shape, with the "no read path at all" consequence spelled out rather
    than implied; (3) the traversal ran and answered from committed state ⇒ the shape PS-12 forbids,
    recorded as a finding about the port surface and routed to the verdict and to HS-P0010. No hedge,
    no two shapes at once, and no sentence that would read the same had the case never run.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "Review-tier against the emitted output of crates/happenstance-ladybug/tests/<conformance target>.rs, checked against spec/SPECIFICATION.md:5052-5057 and :5067-5073; the recorded outcome in .bklg/from-contract-to-published-library/ladybug-projection-store/read-your-own-writes-projection/"

- id: AC-004
  criterion: >-
    GIVEN the adapter author on "Learn when you are finished", who will read this record next year to
    learn what a Ladybug batch can be asked, WHEN they look for the answer, THEN it is not hiding
    inside a `Capability::declined("…")` string: a declension states what the fixture could not set up
    and a declined rule does not run, so it cannot express a store behaviour and cannot discharge an
    AC that requires execution. If the suite's vocabulary can only express this case as a declension,
    that fact is written down as a finding about the suite, raised with `projection-store-freeze` in
    this same change with its reason — and AC-005's branch carries the observation instead.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "Static: rg -n \"Capability::declined\" crates/happenstance-ladybug/tests/ reviewed against crates/happenstance-testkit/src/contract.rs:219-232 and :412-419, inside cargo xtask ci's clippy/tests steps; any raised finding recorded as prose in the story folder"

- id: AC-005
  criterion: >-
    GIVEN the suite maintainer at HS-P0010, who registered `batch_reads_reflect_pending_writes` behind
    `ProjectionProbe::READS_THROUGH_BATCH` and needs to know whether this adapter's declaration is an
    observation or a guess, WHEN they read the record, THEN the branch taken is stated: with the
    declaration `true`, the suite rule is the primary evidence and the two-statement case stands
    beside it as the control's twin; with the declaration `false`, the suite emits a reported skip —
    which executes nothing and therefore discharges nothing — and the two-statement case runs as a
    test in the same target or nothing ran at all. Either way the suite is invoked whole, never
    subsetted, and the declaration's value was written after the run said what it is, not copied from
    the module docs.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "crates/happenstance-ladybug/tests/<conformance target>.rs — the whole projection_store_conformance!(LadybugFixture::new()) invocation under --show-output, its batch_reads_reflect_pending_writes line (pass or reported SKIP) pasted verbatim; declaration value in crates/happenstance-ladybug/src/projection_store.rs against spec/SPECIFICATION.md:5059-5065"

- id: AC-006
  criterion: >-
    GIVEN the gate reader already burned once by a step that a deletion fails and an emptying passes,
    WHEN either variant is deleted, emptied, renamed or `#[ignore]`d, THEN `cargo xtask ci` fails
    before the run with a message naming the missing test — never exiting 0 on `running 0 tests` —
    because the existing `ARTEFACTS` row for this crate's conformance target now carries both
    variants' fully qualified names, copied out of `cargo test --test <target> -- --list` rather than
    guessed.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "cargo xtask proof-artefact — its own gate step (xtask/src/main.rs:170-178), check() at xtask/src/proof.rs:195-217 with the qualification requirement at :58-70; plus the by-hand #[ignore] negative control with its failure output recorded"

- id: AC-007
  criterion: >-
    GIVEN a stranger who did not write this adapter and is asked to re-take the snapshot — DR-9's bar,
    because "passed against N adapters" is a snapshot and a snapshot has to be re-takeable — WHEN they
    read the record alone, THEN it names the crate, the target, both fully qualified test names, the
    exact command, the toolchain and the commit SHA, and carries both variants' emitted output
    verbatim rather than a pass/fail bit — so that a case which hung or timed out on the blocking
    bridge is visibly a different event from a traversal that ran and saw nothing, and so that a claim
    written from the module docs would have nothing to cite.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "Process-tier: the committed run record in .bklg/from-contract-to-published-library/ladybug-projection-store/read-your-own-writes-projection/, cross-checked against the ARTEFACTS row's names (xtask/src/proof.rs:133, :58-70) and cargo xtask ci's captured tests output; discipline per project.md:151-153 (DR-4), :172-174 (DR-9)"

- id: AC-008
  criterion: >-
    GIVEN the maintainers of the port, the suite and the specification, none of whom accepted a change
    in this PR, WHEN they read the whole diff, THEN nothing under `crates/happenstance-testkit/**`,
    `crates/happenstance-core/src/projection.rs`, `spec/SPECIFICATION.md` or `spec/E2E-CASES.md` was
    touched; no clause marker moved and `cargo xtask spec-trace` is green; `happenstance-ladybug`
    gains no new `pub` item; no rule was `#[cfg]`-ed out, `#[ignore]`d, retried until green or made to
    pass by weakening the fixture or the case; and no assertion anywhere names a literal position
    value — the case asserts on graph contents and, where a position appears, on the position the
    store was handed.
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — the ARTEFACTS constant at :133, the existing row for this crate's conformance target, mounting crates/happenstance-ladybug/tests/<conformance target>.rs"
  verifying_test: "Static/boundary: git diff --stat showing no path under crates/happenstance-testkit/, crates/happenstance-core/, spec/, .kb/ or references/; cargo xtask ci (whole gate, not --fast) including spec-trace and clippy --workspace --all-targets --all-features -D warnings (xtask/src/main.rs:116-129); rg -n over crates/happenstance-ladybug/tests/ for #[cfg] and #[ignore]"
```
