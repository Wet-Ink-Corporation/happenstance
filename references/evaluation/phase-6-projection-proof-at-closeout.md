# Phase 6's proof artefact at closeout: what the projection suite was shown to do

**Date:** 2026-08-15 · **Pinned to:** `04b1f4d` —
`04b1f4d0b63f02796317b753160946b4e25aceb9` on
`initiative/from-contract-to-published-library`
· **Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)`
· **Run:** one `cargo xtask ci`, whole, **exit 0**

**Supersedes [`phase-6-projection-proof.md`](phase-6-projection-proof.md)**
(2026-08-15, pinned to `7620481`), which is left exactly as it was written. That
document is not wrong about its own subject: at `7620481` the enumeration held
sixteen rules and §7.2 daggered the seventeenth, and it says so. It stopped
describing *this* tree at `dc363f4`, when `fresh_projection_has_no_checkpoint`
landed against PS-38 and the daggers came off. Under [this directory's
rules](README.md) the repair for that is a later dated document naming the
earlier one, never an edit — so this is that document, and the delta it exists
to record is precisely the deliverable the project's disclosed hold was about.

**Immutable evidence**, on the same terms: dated, pinned to the commit above,
cited by `file:line` from elsewhere, and superseded rather than edited.

---

## What this is, and the sentence it exists to refuse

The sentence is `cargo xtask ci`: green. It is true, and it is consistent with
all of: a `CheckpointOnlyStore` quietly dropped from the registry, a "second
batch shape" that is the oracle wearing a hat, and a `wasm32` harness running
two rules where the host harnesses run seventeen. A green gate is a
**precondition** for looking at those, never a substitute for them.

So this document records **nouns**: the name of the test a deliberately wrong
store fails, the names of the fixtures that pass, which optional gate steps ran
rather than skipped, and what the run does not cover.

**It decides nothing.** No freeze verdict, no `unstable-projection` exposure
verdict, no restatement of the PS-3 batch-shape finding
([that document](projection-batch-shape-evidence.md) is linked, not summarised),
and no ratio over the mutant set in any form.

## Provenance

| | |
|---|---|
| Commit | `04b1f4d0b63f02796317b753160946b4e25aceb9` |
| `git status --porcelain` at that commit | empty, before the run and after it |
| Command | `cargo xtask ci` — **not** `--fast` |
| Exit status | `0`, closing line `all checks passed` |
| Wall clock | 2026-08-15, 11:01 PDT, warm cache |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, the pin in `rust-toolchain.toml` |
| Ancestry | a descendant of `dc363f4` (the seventeenth rule), of `a930e12` and `d0d3db2` (the specification's own prose brought into agreement with its generated §7.2, and the citations that moved with it), and of `263b7e6` (the gate step that holds a shipped document's rule count to the enumeration) |

**Where this document lands, stated rather than left to be inferred.** It is
committed *after* the tree it is pinned to, together with the index entry in
[`README.md`](README.md) and a one-line change in `xtask/src/proof.rs` pointing
that file's `#[cfg(test)]` guards at this document instead of the superseded
one. Nothing else moves in that commit, and the gate is re-run whole afterwards
— the same shape the superseded document used, where `7620481` ran and `674c459`
recorded it.

`--fast` was not used and would not have been evidence: it drops every
`OPTIONAL` step, which is four of the rows in the ledger below.

---

## 1. The failing test, and there are two names

`CheckpointOnlyStore` is a deliberately wrong projection store: it commits the
checkpoint and discards the read-model write. The specification names it by hand
(§4.11) as one of the three stores the projection suite owes.

| | Name | What it is |
|---|---|---|
| **The conformance rule it fails** | `commit_is_atomic_with_the_read_model` | The rule an adapter author would fail. PS-1's coupling: write a probe row into a batch, commit at position *P*, then read the row and the checkpoint through fresh handles — both present or both absent, never one |
| **The meta-test that asserts it fails exactly there** | `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` | The test the **gate** runs. It is why a green exit means something: the harness catches the mutant's panic, so the suite exits 0 *because* the mutant failed where it was declared to, and would exit non-zero if it failed anywhere else — or nowhere |

Conflating the two is the easiest way to make this section useless. The gate's
green tick is the **meta-test** passing; the name an adapter author cares about
is the **rule**.

`CheckpointOnlyStore` is declared to fail three rules, not one
(`crates/happenstance-testkit/tests/projection_mutation_coverage.rs:300-330`):
`commit_is_atomic_with_the_read_model`, `dropped_batch_leaves_store_usable` and
`distinct_projections_advance_independently`. A store that applies no rows fails
any rule that reads one back, and the exactness meta-test asserts **both**
directions — a mutant broken in more ways than it claims is caught too.

The same store is reproduced **outside this workspace**, in
`examples/outside-projection-adapter`, where `outside_projection_discrimination`
convicts it by name and a positive control asserts the conformant sibling passes.

### Re-run it yourself

The meta-test, which is what the gate runs:

```console
cargo test --locked -p happenstance-testkit --all-features \
  --test projection_mutation_coverage \
  projection_mutants_fail_exactly_their_declared_rules -- --exact --show-output
```

The whole projection proof artefact, exactly as the gate reaches it — the names
are asserted out of `-- --list` **before** anything runs, so a renamed or emptied
test fails here rather than exiting 0 with nothing to say:

```console
cargo run --locked -p xtask -- proof-artefact
```

Neither command assumes any state beyond a clone at the pinned commit.

---

## 2. Seventeen rules, three emitters, five harnesses

`for_each_projection_store_rule!`
(`crates/happenstance-testkit/src/projection.rs:1884-1919`) is the single
enumeration, and **all seventeen rules are in it**. That is the change this
document exists for: the superseded artefact recorded sixteen and a specified
seventeenth, and `fresh_projection_has_no_checkpoint` has since landed —
against PS-38, which ADR-0030 minted, rather than by widening `[FROZEN]` PS-19,
which is byte-identical across the whole episode.

Three emitters expand from that one list: `__emit_projection_tokio`,
`__emit_projection_blocking` and `__emit_projection_wasm`.

| Harness | Fixture | Emitter | Observed |
|---|---|---|---|
| `projection_conformance` | `happenstance_testkit::fixtures::MemoryProjectionFixture` | tokio | **17 passed, 0 failed**, two reported skips |
| `projection_conformance_blocking` | `MemoryProjectionFixture` | blocking, no async runtime at all | **17 passed, 0 failed** |
| `projection_conformance_buffering` | `BufferingProjectionFixture` (`crates/happenstance-testkit/tests/projection_conformance_buffering.rs`) | tokio | **18 passed, 0 failed, no skip** — the enumeration plus `the_second_batch_shape_answers_every_rule_with_a_pass` |
| `outside_projection_conformance` | `OutsideFixture` (`examples/outside-projection-adapter/tests/support/mod.rs`) | tokio | **17 passed, 0 failed**, one skip carrying that fixture's own stated reason |
| `projection_conformance_wasm` | `MemoryProjectionFixture` | wasm | **type-checked in this run; executed separately** — see the limits section |

The claim *"the whole suite, not a separately maintained subset"* is held by
`crates/happenstance-testkit/tests/projection_harness_parity.rs`. It takes the
names from the enumeration itself and asserts that no harness source contains one
as an identifier, and that each of the three carries exactly one
`projection_store_conformance!` invocation. A harness that listed rules by hand
would type-check exactly as well as a generated one, so the mandatory `wasm32`
check cannot see that failure and a reviewed `rg` cannot fail twice. Both of its
tests are named in the gate's `ARTEFACTS` table (`xtask/src/proof.rs`), so the
guard cannot be `#[ignore]`d in silence either.

### The two batch shapes, and how each was observed

Both inside the **same** `cargo xtask ci` invocation, which is what makes this
one run naming two fixtures rather than two runs a reader reconciles by hand.

`MemoryProjectionFixture` is **apply-on-write**: each write lands in the store's
own map as it is made. `BufferingProjectionFixture` is **replay-at-commit**: the
batch holds a pending write set the store cannot see until `commit` takes it, and
it holds no handle, transaction or lock between `begin` and `commit`. The second
is a positive control rather than a second name for the oracle, and the registry
meta-tests `projection_conformant_variants_pass_everything` and
`the_second_batch_shape_answers_every_rule_with_a_pass` are what say so
mechanically.

**What this pair is not.** It is not PS-2's bar. See the limits section.

---

## 3. The run ledger — every step, ran or skipped

**Four of four `OPTIONAL` steps ran. Nothing was skipped.** A green run with
skipped steps is a weaker claim than a green run with none, and both `cargo-hack`
and `cargo-deny` resolve on this machine, so a skip would have been a signal to
investigate rather than a default to accept.

### `REQUIRED`

| Step | Outcome |
|---|---|
| formatting | ran, green |
| clippy (all targets, all features) | ran, green |
| tests | ran, green |
| each phase's proof artefacts | ran, green — five targets, see the table below |
| wasm32 build of the contract crate | ran, green |
| wasm32 check of the conformance harnesses | ran, green |
| wasm32 build of the Cloudflare adapter | ran, green |
| wasm32 build of the Neon adapter | ran, green |
| documentation | ran, green |
| specification traceability | ran, green — *201 clauses (139 FROZEN, 50 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 112 conformance rules, 58 e2e cases, 380 citations checked (70 anchored to their subject, 12 external)*; §7.1–§7.2 matches the checker |
| no retired rule is still live | ran, green — 0 dispositions against 112 rules in 4 files |
| no conformance rule reads a clock | ran, green |
| no literal position values in the suite | ran, green |
| every conformance rule has a changelog entry | ran, green — *all 112 rules in 4 file(s) have a changelog entry* |
| **every stated rule count matches the suite** | ran, green — *6 stated rule count(s) checked against 1 (model.rs), 5 (concurrency.rs), 17 (projection.rs), 89 (suite.rs), 112 (all four rule files)*. New in `263b7e6`, and the only step that reads a document for content rather than reading code against one |
| the testkit carries its own version | ran, green |
| happenstance-core names serde/alloc and base64/alloc | ran, green |
| the Rust constitution is internally consistent | ran, green — *27 atoms, all consistent* |
| the constitution's examples compile | ran, green |
| documentation (no default features) | ran, green |
| documentation (default features) | ran, green |
| packaged artifacts carry their licences and README | ran, green — `happenstance-core` 26 files, `happenstance` 8, `happenstance-testkit` 41, each including both licences and `README.md` |

### `OPTIONAL`

| Step | Probe | Outcome |
|---|---|---|
| feature powerset | `cargo hack --version` | **ran**, green |
| wasm32 feature powerset | `cargo hack --version` | **ran**, green — `happenstance-core`, `happenstance-neon`, `happenstance-testkit` |
| licences and advisories | `cargo deny --version` | **ran**, green — *advisories ok, bans ok, licenses ok, sources ok* |
| docs.rs configuration (nightly) | `cargo +nightly --version` | **ran**, green |

### What the proof-artefact step printed

```text
happenstance-testkit/mutation_coverage: 8 named tests present, 79 registry rows
happenstance-testkit/projection_mutation_coverage: 7 named tests present, 21 registry rows
happenstance-testkit/projection_harness_parity: 2 named tests present
happenstance-core/wire: 3 named tests present
happenstance-sync/wire: 2 named tests present
```

Two registries, two counts, each printed beside the target it is about. The
projection registry's rows are wrong stores and legal ones together: the wrong
stores each declare the exact rule set they fail, and the conformant variants
declare that they fail nothing, which is what makes the exactness meta-test able
to convict in both directions.

### The skeletons

`happenstance-postgres`, `happenstance-ladybug` and `happenstance-sqlite`
compiled inside this run, in the `tests`, `clippy` and both powerset steps, with
every affected method body still `todo!()`. **No skeleton body was changed** by
the commit this document is pinned to or by any commit since the superseded
artefact was written.

---

## 4. What this run does **not** cover

The whole value of this document to a reader who does not trust summaries is that
it says where the evidence stops.

**The MSRV.** The local gate never checks it. CI's `minimum supported Rust version`
job does — `cargo hack check --no-dev-deps --rust-version` on a pinned 1.97.1
toolchain, plus a full `cargo test --workspace --all-features` at 1.97.1, because
`--no-dev-deps` is exactly the flag that hides `proptest` and `tokio`
(`.github/workflows/ci.yml:241-278`). The floor currently **equals** the
`rust-toolchain.toml` pin, so that job proves nothing until the two diverge again;
ADR-0029 explains why it is kept rather than deleted.

**`wasm32` *execution*, and it was executed — separately.** The gate
*type-checks* the harnesses on that target and does not run them. What runs them
in CI is the `conformance on wasm32` job, which resolves `wasm-bindgen-cli` out
of `Cargo.lock` (`.github/workflows/ci.yml:204-238`). This document reports **no
result for that job**. What it does report, as a separate command and not as part
of the pinned run, is that

```console
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  cargo test --locked -p happenstance-testkit \
  --target wasm32-unknown-unknown --test projection_conformance_wasm
```

gave **17 passed, 0 failed, 0 ignored** at this same commit, every rule named
individually. Two claims, kept apart on purpose: the gate type-checked the
harness, and a hand-run under the CI runner's own tool executed it.

**PS-2's bar.** Two structurally unlike batch shapes passing is **not** the same
as two *adapters* at opposite ends of the batch-shape axis. Both shapes named in
section 2 are testkit-side instruments this workspace wrote — the outside-author
fixture is a third instrument and not a storage adapter either — and PS-2's
*Rejects* field names the schedule that freezes the port against
`MemoryProjectionStore` and one in-process transaction as the monoculture to
refuse. The port therefore ships behind `unstable-projection`, and what would
clear the bar — a projection adapter over storage this workspace does not
control — is stated in the module's own header.

**CI-side advisories and semver.** `cargo deny check` ran here; the workflow's own
advisory and packaging jobs are CI's and are not reported in this ledger.

**Durability, restarts and real connections.** Every store in the projection
mutant registry is a `BTreeMap` behind an `Rc`, so nothing in this run models a
defect whose observation needs a real restart, a connection pool or a transaction
manager. That limitation is the event-store family's too, and it is stated so a
green run here is not read as evidence about durability.

---

## 5. What this document deliberately does not say

- **No freeze verdict.** Whether the port is frozen is `ladybug-projection-store`'s
  to write, from this evidence.
- **No `unstable-projection` exposure verdict.** Whether 0.1 ships with the gate
  closed is `publication-and-positioning`'s at phase 12.
- **No restatement of the PS-3 finding.** Whether the two batch shapes disagreed,
  and where, is [`projection-batch-shape-evidence.md`](projection-batch-shape-evidence.md).
  That document is pinned to `cfd9231` and is **not** superseded here: it answers
  one question off one run, and the question it answers has not changed.
- **No ratio over the mutant set, in any form.** ADR-0010 forbids it because the
  denominator is a choice: a fraction reports how representative the author was
  while reading as though it said how good the suite is. *"Seventeen rules across
  three emitters"* is a statement about the enumeration and is allowed; a fraction
  over the registry is the forbidden sentence and appears nowhere above.

  The prohibition is enforced rather than remembered:
  `proof::tests::the_artefact_quotes_no_pass_rate_over_the_mutant_set` in
  `xtask/src/proof.rs` reads this document and fails on a ratio-shaped sentence
  near any line mentioning a mutant.

---

## Citations

Every `file:line` above is the working tree's at `04b1f4d`, the commit named in
the header. A reader following one into a later tree should expect the **item**,
not the number — the subject is quoted alongside every range for exactly that
reason. This directory's own recorded failure mode is citations that resolve,
pass `spec-trace`, and point at the wrong line.
