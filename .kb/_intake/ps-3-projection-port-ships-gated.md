# PS-3's verdict: the projection port ships behind `unstable-projection` at `0.2.0`

**Decided 2026-09-02.** Staged for `/redkiln:kb-ingest` as an accepted decision
atom, numbered 0036 or above. `.kb/decisions/` atoms are authored by ingest and
never by hand, so this file is the input and not the record.

## The decision

`ProjectionStore` **is not frozen at `0.2.0`.** It ships behind the off-by-default
`unstable-projection` feature in both `happenstance-core` and `happenstance`, with
its semver exemption documented on the module itself.

PS-3 is a *SHOULD with a condition* — *"Until PS-2's bar is met"*
(`spec/SPECIFICATION.md`:4880-4881). The condition has not been met. This is that
evaluation, taken part by part rather than asserted, and it is the first time
anything in the tree has applied PS-2's bar to a real adapter set.

## PS-2's bar, evaluated

PS-2 (`spec/SPECIFICATION.md`:4864-4867) is `[FROZEN]` and is a **conjunction of
two parts**. Both must hold. One does.

### Part 1 — a hostile store must *fail* the suite. **MET.**

*"the conformance suite has failed a store that commits the checkpoint and
discards the read-model write"*

`CheckpointOnlyStore` exists at
`crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`:44,
registered as a `Defect` with `NAME = "CheckpointOnlyStore"`, and
`crates/happenstance-testkit/tests/projection_conformance.rs`:15 records that it
is discharged elsewhere and by name. The mutant registry asserts that every mutant
fails exactly the rules its row claims, so this is a rule that demonstrably
rejects something rather than a rule nothing can fail.

### Part 2 — two adapters at opposite ends of the batch-shape axis must pass. **NOT MET.**

*"two adapters at opposite ends of the batch-shape axis have passed it"*, where
the ends are named: *"one adapter holding a live transaction (rusqlite or `sqlx`)
and one that cannot hold anything across an await (Workers `SqlStorage` or Neon
over one-shot HTTP)."*

Exactly **one storage adapter** has run `projection_store_conformance!`:
`SqliteProjectionStore`, at `crates/happenstance-sqlite/tests/projection.rs`:226,
against a real temporary file. `spec/SPECIFICATION.md`:392 states the rest
plainly: of the five impls, *"the other four still run against nothing."*

It fails on **both halves**, not one:

- **Neither named end is represented by a passing adapter.** SQLite's `Batch` is
  an *owned* write set — statements pushed and replayed at `commit`, with the
  lifetime removed by ADR-0017 — so it sits at the buffered end, not the
  live-transaction end. The live-transaction impls are
  `PostgresProjectionStore` (`todo!()` throughout) and the `live_handle`
  experiment (`checkpoint` still outstanding). Neither has run the suite.
- **No cannot-hold-across-await adapter has passed either.** `NeonProjectionStore`
  carries real bodies in all four port methods and runs against nothing; there is
  no Workers projection store at all.

What *has* passed beside SQLite are testkit fixtures — `MemoryProjectionFixture`,
`BufferingProjectionFixture`, the blocking and `wasm32` harnesses — and
`examples/outside-projection-adapter`. Those are worth having and they are not
adapters. Counting them would be the precise error PS-2's *Rejects* clause names:
*"the schedule that freezes this port against `MemoryProjectionStore` and an
in-process rusqlite transaction… the exact monoculture CLAUDE.md's spread rule
exists to catch."*

## The arm that lost, and what it costs

**Freezing the port at `0.2.0`.** Rejected because part 2 is unmet, and the cost
of being wrong is asymmetric and unrecoverable: a frozen port is a semver promise,
a published version can be yanked but never removed, and the axis the port is most
likely to be wrong about — whether a batch can be a live borrowed handle — is the
one no passing adapter currently occupies. Freezing now would freeze against one
storage shape and call it a contract.

The cost of the arm taken is real and is accepted: a consumer must write the word
`unstable-projection` to get a projection store, the surface makes no semver
promise, and `cargo-semver-checks` will not police it.

## What this does not decide

- **The marker does not move.** PS-3 keeps `[PROVISIONAL]`. Its falsifier is
  *PS-2's bar met before 0.1*, which has not happened; a satisfied SHOULD is not a
  moved marker.
- **PS-2 is untouched.** It is `[FROZEN]`, and nothing here amends it. It was
  applied, not edited — which is the point of writing the verdict against it.
- **`happenstance-sqlite`'s default features are not this decision's to change**,
  and the story that took this verdict says so in terms: its own scope table
  records *"Conformance rule(s): none, and it is not adapter-observable."* The
  finding below is routed rather than resolved here.

## Finding, routed rather than absorbed

**`cargo add happenstance-sqlite` turns the unfrozen port on without the consumer
naming it.** The adapter's `default = ["event-store", "projection-store"]` and
`projection-store = ["happenstance-core/unstable-projection"]`, so the gate that
`happenstance-core` and `happenstance` both hold off-by-default is walked around
by the third crate in the release set. PS-3's *"off-by-default"* is true of two
crates out of three, and false for the one a consumer installs to get a store.

The forwarding itself is right and its manifest comment argues it well — *"one
capability, one switch… a crate with two switches for one capability is a crate
where the weaker one wins silently."* What is in question is only whether that
switch belongs in `default`.

It is not free to change. `crates/happenstance-sqlite/tests/projection.rs` is
gated on `feature = "projection-store"`, so removing it from `default` means a
bare `cargo test -p happenstance-sqlite` stops running the projection family —
the exact defect that manifest already warns about for `proptest`: *"a family
that needs an extra flag to appear is a family that silently does not run."* The
gate itself is unaffected, because it runs `--all-features`.

**Owner: `publication-and-positioning`, before the `0.2.0` publish.** It is a
crate-surface decision with a consumer consequence, and it should be taken
deliberately rather than inherited from a default nobody re-read.

## Evidence

- `spec/SPECIFICATION.md`:4864-4877 (PS-2), :4880-4891 (PS-3), :392 (the impl
  census and what has run against what), :4897 (the phase-6 discharge)
- `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`:44
- `crates/happenstance-sqlite/tests/projection.rs`:226
- `references/evaluation/projection-batch-shape-evidence.md`:1 — recorded as
  evidence for this clause and deliberately not a verdict on it
- `crates/happenstance-sqlite/Cargo.toml`, `[features]` — the routed finding
