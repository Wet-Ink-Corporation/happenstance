# Phase 6's proof artefact: what the projection suite was shown to do

**Date:** 2026-08-15 · **Pinned to:** `7620481` —
`7620481010f7bc7a3eb50b241b1dbbea77aaa68c` on
`initiative/from-contract-to-published-library`
· **Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)`
· **Run:** one `cargo xtask ci`, whole, **exit 0**

**Immutable evidence** under [this directory's rules](README.md): dated, pinned to
the commit above, cited by `file:line` from elsewhere, and **superseded rather
than edited**. A later run is a later document that names this one.

---

## What this is, and the sentence it exists to refuse

The sentence is `cargo xtask ci`: green. It is true, it is the hardest-won
sentence in the project, and it is consistent with all three of: a
`CheckpointOnlyStore` quietly dropped from the registry, a "second batch shape"
that is the oracle wearing a hat, and a `wasm32` harness running two rules out of
sixteen. The project's own Definition of Done says so in as many words — *a green
gate is a **precondition** for looking at items 1 and 2, never a substitute for
them*.

So this document records **nouns**: the name of the test a deliberately wrong
store fails, the names of the two batch shapes that pass, which optional gate
steps ran rather than skipped, and what the run does not cover.

**It decides nothing.** No freeze verdict, no `unstable-projection` exposure
verdict, no restatement of the PS-3 batch-shape finding
([that document](projection-batch-shape-evidence.md) is linked, not summarised),
and no ratio over the mutant set in any form.

## Provenance

| | |
|---|---|
| Commit | `7620481010f7bc7a3eb50b241b1dbbea77aaa68c` |
| `git status --porcelain` at that commit | empty, before the run and after it |
| Command | `cargo xtask ci` — **not** `--fast` |
| Exit status | `0` |
| Wall clock | 2026-08-15, 01:14:59–01:15:59 PDT (about one minute, warm cache) |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, the pin in `rust-toolchain.toml` |
| Ancestry | a descendant of `984e7fd` (the `unstable-projection` gate and the PS clause disposition) and of `d6496cd` (the documented extension surface) — the two changes the run had to come after, or it would be evidence about a tree that no longer exists |

`--fast` was not used and would not have been evidence: it drops every `OPTIONAL`
step, which is two of the rows in the ledger below.

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
green tick is the **meta-test** passing; the name an adapter author cares about is
the **rule**.

`CheckpointOnlyStore` is declared to fail three rules, not one
(`crates/happenstance-testkit/tests/projection_mutation_coverage.rs:294-329`):
`commit_is_atomic_with_the_read_model`, `dropped_batch_leaves_store_usable` and
`distinct_projections_advance_independently`. A store that applies no rows fails
any rule that reads one back, and the exactness meta-test asserts **both**
directions — a mutant broken in more ways than it claims is caught too.

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

## 2. The two batch shapes, and how each was observed

Both inside the **same** `cargo xtask ci` invocation, which is what makes this one
run naming two fixtures rather than two runs a reader reconciles by hand.

| Fixture | Batch shape | How it was observed in this run |
|---|---|---|
| `happenstance_testkit::fixtures::MemoryProjectionFixture` | **apply-on-write** — each write lands in the store's own map as it is made | Drove the whole enumeration to completion through two host harnesses: `projection_conformance` (tokio) and `projection_conformance_blocking` (no async runtime at all), **16 passed, 0 failed** in each |
| `BufferingProjectionFixture` (`crates/happenstance-testkit/tests/projection_conformance_buffering.rs:52`) | **replay-at-commit** — the batch is a buffered write set replayed inside `commit` | Drove the same enumeration through `projection_conformance_buffering`, and is registered as a CF-5 conformant variant in the projection mutant registry, where `projection_conformant_variants_pass_everything` and `the_second_batch_shape_answers_every_rule_with_a_pass` assert it fails nothing |

The second fixture is a positive control rather than a second name for the oracle:
it is a structurally different store — its batch holds a pending write set that the
store cannot see until `commit` takes it — and the registry's own meta-test is what
says so mechanically.

**What this pair is not.** It is not PS-2's bar. See the limits section.

---

## 3. Sixteen rules, three emitters, and what holds that claim

`for_each_projection_store_rule!` is the single enumeration. Sixteen rules are
registered in it; a seventeenth, `fresh_projection_has_no_checkpoint`, is
specified and not yet written, and §7.2 daggers it for that reason.

Three emitters expand from that one list: `__emit_projection_tokio`,
`__emit_projection_blocking` and `__emit_projection_wasm`.

The claim *"the whole suite, not a separately maintained subset"* is held by
`crates/happenstance-testkit/tests/projection_harness_parity.rs`, added in the
commit this document is pinned to. It takes the names from the enumeration itself
and asserts that no harness source contains one as an identifier, and that each of
the three carries exactly one `projection_store_conformance!` invocation. A
harness that listed rules by hand would type-check exactly as well as a generated
one, so the mandatory `wasm32` check cannot see that failure and a reviewed `rg`
cannot fail twice.

Both of its tests are themselves named in the gate's `ARTEFACTS` table
(`xtask/src/proof.rs`), so the guard cannot be `#[ignore]`d in silence either.

---

## 4. The run ledger — every step, ran or skipped

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
| wasm32 check of the conformance harnesses | ran, green — this is the whole of AC-016's *mechanism* |
| wasm32 build of the Cloudflare adapter | ran, green |
| wasm32 build of the Neon adapter | ran, green |
| documentation | ran, green |
| specification traceability | ran, green — *201 clauses (139 FROZEN, 50 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 111 conformance rules, 58 e2e cases, 379 citations checked*; §7.1–§7.2 matches the checker |
| no retired rule is still live | ran, green — 0 dispositions against 111 rules in 4 files |
| no conformance rule reads a clock | ran, green |
| no literal position values in the suite | ran, green |
| every conformance rule has a changelog entry | ran, green — *all 111 rules in 4 file(s) have a changelog entry* |
| the testkit carries its own version | ran, green |
| happenstance-core names serde/alloc and base64/alloc | ran, green |
| the Rust constitution is internally consistent | ran, green — *27 atoms, all consistent* |
| the constitution's examples compile | ran, green |
| documentation (no default features) | ran, green |
| documentation (default features) | ran, green |
| packaged artifacts carry their licences and README | ran, green — `happenstance-core`, `happenstance`, `happenstance-testkit` |

### `OPTIONAL`

| Step | Probe | Outcome |
|---|---|---|
| feature powerset | `cargo hack --version` | **ran**, green — 62 workspace combinations |
| wasm32 feature powerset | `cargo hack --version` | **ran**, green — `happenstance-core`, `happenstance-neon`, `happenstance-testkit` |
| licences and advisories | `cargo deny --version` | **ran**, green |
| docs.rs configuration (nightly) | `cargo +nightly --version` | **ran**, green |

The two powersets are the only steps that would catch `conformance` being silently
coupled to `memory`, or `unstable-projection` failing to compile with the port
gated off — `happenstance-core` went from eight feature combinations to
thirty-two in the commit before this one, and both steps exercised the widened set.

### What the proof-artefact step printed

```text
happenstance-testkit/mutation_coverage: 8 named tests present, 79 registry rows
happenstance-testkit/projection_mutation_coverage: 7 named tests present, 19 registry rows
happenstance-testkit/projection_harness_parity: 2 named tests present
happenstance-core/wire: 3 named tests present
happenstance-sync/wire: 2 named tests present
```

Two registries, two counts, each printed beside the target it is about. Before the
commit this document is pinned to, the count was selected by *package*, so a second
`happenstance-testkit` row would have printed `79` beside the projection target — a
number that is not about that target at all.

### The skeletons

`happenstance-postgres`, `happenstance-ladybug` and `happenstance-sqlite` compiled
inside this run, in the `tests`, `clippy` and both powerset steps, with every
affected method body still `todo!()`. **No skeleton body was changed** by the
commit this document is pinned to or by the one before it.

---

## 5. What this run does **not** cover

The whole value of this document to a reader who does not trust summaries is that
it says where the evidence stops.

**The MSRV.** The local gate never checks it. CI's `minimum supported Rust version`
job does — `cargo hack check --no-dev-deps --rust-version` on a pinned 1.97.1
toolchain, plus a full `cargo test --workspace --all-features` at 1.97.1, because
`--no-dev-deps` is exactly the flag that hides `proptest` and `tokio`
(`.github/workflows/ci.yml:241-278`). The floor currently **equals** the
`rust-toolchain.toml` pin, so that job proves nothing until the two diverge again;
ADR-0029 explains why it is kept rather than deleted.

**`wasm32` *execution*.** The gate *type-checks* the harnesses on that target and
does not run them. What runs them is CI's `conformance on wasm32` job, which
resolves `wasm-bindgen-cli` out of `Cargo.lock` and runs
`cargo test --locked -p happenstance-testkit --target wasm32-unknown-unknown` under
`wasm-bindgen-test-runner` (`.github/workflows/ci.yml:204-238`). The new projection
wasm harness is picked up by that job through cargo target auto-discovery, with no
workflow edit. **This document does not report a result for that job**: it records
that the local run type-checked the harness, and that execution is attributed to a
CI run a reader should look up for the same commit. Claiming otherwise would be the
artefact's own failure mode.

**PS-2's bar.** Two structurally unlike batch shapes passing is **not** the same as
two *adapters* at opposite ends of the batch-shape axis. Both shapes named in
section 2 are testkit-side instruments this workspace wrote, and PS-2's *Rejects*
field names the schedule that freezes the port against `MemoryProjectionStore` and
one in-process transaction as the monoculture to refuse. The port therefore ships
behind `unstable-projection`, and what would clear the bar — a projection adapter
over storage this workspace does not control — is stated in the module's own
header.

**CI-side advisories and semver.** `cargo deny check` ran here; the workflow's own
advisory and packaging jobs are CI's and are not reported in this ledger.

**Durability, restarts and real connections.** Every store in the projection mutant
registry is a `BTreeMap` behind an `Rc`, so nothing in this run models a defect
whose observation needs a real restart, a connection pool or a transaction manager.
That limitation is the event-store family's too, and it is stated so a green run
here is not read as evidence about durability.

---

## 6. What this document deliberately does not say

- **No freeze verdict.** Whether the port is frozen is `ladybug-projection-store`'s
  to write, from this evidence.
- **No `unstable-projection` exposure verdict.** Whether 0.1 ships with the gate
  closed is `publication-and-positioning`'s at phase 12.
- **No restatement of the PS-3 finding.** Whether the two batch shapes disagreed,
  and where, is [`projection-batch-shape-evidence.md`](projection-batch-shape-evidence.md).
- **No ratio over the mutant set, in any form.** ADR-0010 forbids it because the
  denominator is a choice: a fraction reports how representative the author was
  while reading as though it said how good the suite is. *"Sixteen rules across
  three emitters"* is a statement about the enumeration and is allowed; *"n of m
  mutants caught"* is the forbidden sentence and appears nowhere above.

  The prohibition is enforced rather than remembered:
  `proof::tests::the_artefact_quotes_no_pass_rate_over_the_mutant_set` in
  `xtask/src/proof.rs` reads this document and fails on a ratio-shaped sentence
  near any line mentioning a mutant. It fired once while this section was being
  written, on an earlier wording of this very bullet — which is the check working,
  and is recorded rather than tidied away.

---

## Citations

Every `file:line` above is the working tree's at `7620481`, the commit named in the
header. A reader following one into a later tree should expect the **item**, not
the number — the subject is quoted alongside every range for exactly that reason.
This directory's own recorded failure mode is citations that resolve, pass
`spec-trace`, and point at the wrong line.
