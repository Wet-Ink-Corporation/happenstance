# The extension surface, met from outside — the reading discipline and the gap record

This is the primary output of `documented-extension-surface` (HS-S0015). The fixture it describes is
secondary: a fixture proves that *one* author got through, and only this record says **where the
documentation ran out** for them.

Everything above the horizontal rule was written and committed **before** any line of
`examples/outside-projection-adapter/` existed. That ordering is the whole of AC-007's check — a
denylist written afterwards describes what happened; one written first constrains it.

## The arm in force, read and not chosen

DT-8 asks whose adapter-author bar the suite holds. The answer is **not** made here. It is read out
of the signed-off design record, which `projection-api-design-record` (HS-S0014) wrote, and quoted
verbatim:

> ### DT-8 — whose adapter-author bar the suite holds
>
> **Resolution: the bar is held for an author this repository did not write.** The extension surface
> is the documented pair **`projection_store_conformance!` + `ProjectionProbe`**.

— `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:359-363`.

**Arm A is therefore in force**, and with it the obligation the same file names so that it cannot be
quietly dropped:

> `documented-extension-surface` (HS-S0015) must build a projection fixture **from the documentation
> alone** — not copied from `crates/happenstance-testkit/src/fixtures.rs` — and clear the
> mutant-registry exactness check and the capability-skip rule.

— `_design.md:388-395`.

The stated cost the exercise is measuring, also read rather than assumed: *"one feature flag on a
dependency the adapter already has, and no new edge in the dependency graph"* (`_design.md:365-375`,
restating `spec/SPECIFICATION.md`'s own coherence argument for `ProjectionProbe`'s home).

## The allowlist — rendered documentation only

Read as the rendered rustdoc page an outside author actually meets (`cargo doc --all-features`), and
nothing else, while the store, the probe impl and the fixture are being written:

1. `happenstance-testkit`'s crate page — the `//!` block at `crates/happenstance-testkit/src/lib.rs`.
2. `projection_store_conformance!`'s macro page.
3. `for_each_projection_store_rule!`'s macro page, and the three emitter pages it names.
4. `happenstance_testkit::ProjectionFixture`'s page — the fixture trait, its three capability
   constants and its two provided mechanisms.
5. `happenstance_testkit::Capability` and `happenstance_testkit::RuleOutcome`, plus the four
   `&'static str` reason constants beside them.
6. `happenstance_testkit::projection`'s module page and `happenstance_testkit::projection::rules`'
   module page.
7. `happenstance_core::ProjectionStore`'s page and `happenstance_core::ProjectionProbe`'s page,
   together with `Checkpoint`, `Authority`, `CommitError`, `ResetError` and `ProjectionId`.

Two things are *allowed* that a purist might dispute, and they are named here rather than left to be
noticed. Rustdoc renders a trait's **signatures** as well as its prose, so reading `projection.rs`'s
trait declarations is reading the page. And the doctest on `ProjectionStore` — the `ToyStore`
walkthrough — is **published documentation**, on the port's own page, put there precisely so an
implementer has something to copy; copying from it is using the documented surface, not evading the
denylist. Where it was used, this record says so.

## The denylist — not opened while the fixture is written

1. **`crates/happenstance-testkit/src/fixtures.rs`** — foremost. It carries the owned-handle
   reference `ProjectionFixture` implementation, which is precisely what an author would copy.
2. `crates/happenstance-core/src/projection_memory.rs` — `MemoryProjectionStore`'s source, the
   reference *store*.
3. `crates/happenstance-testkit/tests/projection_conformance*.rs` — the testkit's own harnesses,
   including the buffering conformant variant landed by `buffering-conformant-variant` (HS-S0013).
4. `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` and its `mutants/` directory —
   the wrong stores, which include the checkpoint-only analogue this story must invent for itself.

Rule for both lists: **every point at which the allowlist was insufficient is a gap**, recorded below
with what was missing, which denylisted file or which source read answered it, and the documentation
change that would have prevented it. A run that finds no gaps says so in those words, because a
fixture built with no gaps found is either a triumph or a copy, and only the record tells you which
(`discover.md:89-91`).

---

<!-- Everything below this line was written after the fixture. -->

## What was built, and how far it got

`examples/outside-projection-adapter/` — a `publish = false` workspace member picked up by
`Cargo.toml:3`'s members glob with no root-manifest edit.

| | |
| --- | --- |
| Conformant store | `OutsideProjectionStore` (`src/lib.rs`), an in-process `BTreeMap` read model with a checkpoint per `ProjectionId`, both moved under **one** `Mutex` |
| Wrong store | `CheckpointOnlyStore` (`src/lib.rs`), which moves the checkpoint and drops the write set |
| Fixtures | `tests/support/mod.rs`, one per store — the only two `ProjectionFixture` impls in the crate, and they are in `tests/` because they cannot be anywhere else |
| Suite invocation | `tests/outside_projection_conformance.rs`, one line |
| Result | **16 of 16 projection rules pass**, one of them as a reported skip; the whole run is inside the gate's ordinary `tests` step |

Declared capabilities, and the one decline: `SECOND_HANDLE` supported (a handle is an `Arc` clone),
`COMMIT_FAULT` **supported** — the store applies the rows, consults the armed fault, and puts the rows
back before answering `Err`, which is what makes `failed_commit_leaves_both_unchanged` a real result
rather than a skip — and `RESET_REFUSAL` **declined**, in the store's own words
(`OutsideProjectionStore::NO_RESET_PROTECTION`). `ProjectionProbe::READS_THROUGH_BATCH` is `true`, so
this run prints exactly **one** `SKIP` line.

That combination is deliberately *not* the reference fixture's. The reference declines both
`COMMIT_FAULT` and `RESET_REFUSAL`, so before this crate existed nothing in the tree had ever run
`failed_commit_leaves_both_unchanged` outside the mutant harness.

### The rule that convicts the wrong store, named

**`commit_is_atomic_with_the_read_model`.** That is the name of the failing rule, recorded here as
AC-004 requires, reached from `happenstance_testkit::projection::rules` — the published path — rather
than from the testkit's private mutant machinery. Its message, quoted from the run:

> the read-model write and the checkpoint write must become durable together or not at all, and a
> fresh handle saw one without the other: row `Some(12)`, checkpoint `NeverRun`. A store that commits
> the checkpoint and discards the write set replays nothing and skips everything the discarded batch
> would have written

Two neighbouring tests are what make that a demonstration rather than an anecdote, and both are in
`tests/outside_projection_discrimination.rs`:

- `the_checkpoint_only_store_passes_the_rule_that_watches_only_the_checkpoint` — the **trap**, stated
  as a passing test. `commit_advances_the_checkpoint` certifies the wrong store perfectly happily.
  That is P2's fear made executable: "it compiles, and the obvious test is green".
- `commit_is_atomic_with_the_read_model_passes_the_conformant_store` — the **control**. The same rule,
  the same entry point, the right store, green. Without it the red above could have been the harness.

The failure is caught rather than emitted, because a conformance run over a deliberately wrong store
must not turn this workspace's gate red. The panic *is* how the suite reports a failure; catching it
and asserting on it reads the same signal a red CI run would have shown.

## The cost, counted rather than asserted

The claim under test is the specification's own: *"one feature flag on a dependency the adapter
already has, and no new edge in the dependency graph"* (`_design.md:365-375`).

```console
$ cargo tree -p outside-projection-adapter --edges normal
outside-projection-adapter v0.2.0 (examples/outside-projection-adapter)
└── happenstance-core v0.2.0 (crates/happenstance-core)
    ├── bytes v1.12.1
    ├── futures-core v0.3.33
    ├── thiserror v2.0.19
    │   └── thiserror-impl v2.0.19 (proc-macro)
    │       └── … proc-macro2, quote, syn, unicode-ident
    └── trait-variant v0.1.3 (proc-macro)
        └── … proc-macro2, quote, syn, unicode-ident
```

**No `happenstance-testkit` node appears anywhere on a normal edge.** Everything under
`happenstance-core` was already there for any consumer of the port.

```console
$ cargo tree -p outside-projection-adapter -e features --depth 1
outside-projection-adapter v0.2.0
├── happenstance-core feature "conformance"
└── happenstance-core feature "default"
[dev-dependencies]
├── happenstance-testkit feature "default"
├── tokio feature "default"
├── tokio feature "macros"
└── tokio feature "rt"
```

One feature asked for beyond the default: `conformance`. **The claim holds as stated.**

**The claim is about the *non-dev* graph, and this record says so plainly**, because a naive reading
reports a false failure. The testkit and `tokio` are both real costs and both appear above — under
`[dev-dependencies]`, which is exactly where the testkit's own crate page says to put them
(`crates/happenstance-testkit/src/lib.rs`, *Usage*: *"`tokio` with the `macros` and `rt` features in
`dev-dependencies`"*). A dev edge is not a new edge in a consumer's graph and must not be reported as
one.

Two deliberate manifest choices, both stated so a reviewer does not have to infer them:

1. **This crate does not use `[workspace.dependencies]`.** It is the one manifest in the repository
   where `workspace = true` would falsify the measurement: the workspace entry sets
   `default-features = false` on `happenstance-core` for reasons internal to this tree, which would
   have forced `std` to be spelled here as though an outsider had to ask for it, and reported a
   two-feature cost against a claim of one.
2. **`thiserror` is not in `[dependencies]`.** The port asks only for `core::error::Error + 'static`,
   and the `ToyStore` walkthrough on `ProjectionStore`'s own page hand-writes its `Display` and
   `Error` impls. Deriving would have put a second crate on a normal edge for two lines — a real new
   edge, in a record whose whole subject is edges.

## The counterfactual, convicted by transcript

`ProjectionProbe`'s placement in `happenstance-core` rather than in `happenstance-testkit` is an
argument about a crate boundary that nothing inside this workspace can fail. Two transcripts, both
produced deliberately here and then reverted, are what convict it. Together they show there would be
**no legal home at all** for the impl if the trait lived in the testkit.

**Transcript 1 — the impl in `tests/`, where an author would naturally put it.**

```console
$ cargo test -p outside-projection-adapter --test orphan_probe_placement
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> examples\outside-projection-adapter\tests\orphan_probe_placement.rs:5:1
  |
5 | impl ProjectionProbe for OutsideProjectionStore {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^----------------------
  |                          |
  |                          `OutsideProjectionStore` is not defined in the current crate
  |
  = note: impl doesn't have any local type before any uncovered type parameters
  = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
  = note: define and implement a trait or new type instead
```

**Transcript 2 — the obvious escape, closed.** "Then put it in `src/`" is the answer, and it is exactly
the answer that is unavailable when the trait lives in a dev-dependency:

```console
$ cargo check -p outside-projection-adapter --lib      # with `use happenstance_testkit::ProjectionFixture;` in src/lib.rs
error[E0432]: unresolved import `happenstance_testkit`
   --> examples\outside-projection-adapter\src\lib.rs:505:5
    |
505 | use happenstance_testkit::ProjectionFixture;
    |     ^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `happenstance_testkit`
```

A dev-dependency does not exist for the library build. So: trait in the testkit ⇒ the impl cannot be
in `src/` (transcript 2) and cannot be in `tests/` (transcript 1) ⇒ the testkit must be promoted to a
**non-dev** dependency and feature-gated. Trait in the contract crate ⇒ the impl sits in `src/` beside
the store, and costs the one flag counted above. That is the specification's argument, and this is the
first time anything in the tree has been able to run it.

The permanent artefact is these transcripts. Neither file is in the committed tree: the deliberate
violation was performed, captured, and reverted, which is the discipline `references/adapter-shapes.md`
uses for the six skeletons.

## The gaps

Numbered, each with what was missing, what answered it, and the documentation change that would have
prevented it. **Fixed** means fixed in this change and cited from here; **Reported** means the fix is a
port change, a new rule, a clause amendment or a specification edit, and belongs to
`unstable-projection-gate-and-clause-disposition` (HS-S0016) or the runbook's ADR pass.

### G1 — The port's own page tells an adapter author the suite does not exist. **Reported.**

*What was missing.* Nothing was missing; something was **false**, which is worse.
`crates/happenstance-core/src/projection.rs:3-11` — the first paragraph of the first page an adapter
author lands on — still reads *"the conformance suite does not cover it yet — and a port without a
conformance suite is a guess"*. That stopped being true at merge position 6. An outside author who
believed it would never look for `projection_store_conformance!` at all, which makes it the single
most expensive sentence on the documented surface.

*What answered it.* The testkit's own crate page, which describes the projection family in detail —
so the two pages an author reads first disagree with each other about whether the thing exists.

*The change that would have prevented it, and why it is not made here.* One clause: *"and
`happenstance_testkit::projection_store_conformance!` now covers it"*, with the provisional marker and
everything else in the paragraph left exactly as it is. It is **not made here** because
`spec/SPECIFICATION.md:4566-4569` opens §4 by quoting that paragraph verbatim — *"The port declares
itself provisional in its own first paragraph"* — and this story's PR boundary excludes the
specification. Repairing one without the other replaces a false sentence with a rotted citation.
Owner: HS-S0016, which owns the PS-1 – PS-37 maturity sweep and is the story that will be editing that
paragraph's status anyway.

### G2 — `spec-trace` does not check that a quoted citation still says what it quotes. **Reported.**

*What was missing.* A guard. This was measured twice rather than assumed, both probes reverted:

- The phrase *"a port without a conformance suite is a guess"* was **deleted** from
  `projection.rs:3-11` and `cargo xtask spec-trace` stayed green, though
  `spec/SPECIFICATION.md:4568` quotes it verbatim at that exact citation.
- This change inserts 65 documentation lines above `crates/happenstance-testkit/src/lib.rs:238`, which
  `spec/SPECIFICATION.md:7807` cites for `event_store_conformance!`. That citation was **already**
  pointing at the closing brace of a `pub use` block before this story touched anything; it now points
  into prose. `spec-trace` stayed green through both states.

*What answered it.* `spec-trace`'s own summary line, which says so out loud: *"359 citations checked
(69 anchored to their subject, 12 external)"* — 69 of 359, and the remainder are existence-checked.

*The change that would have prevented it.* Either anchor the two citations above to their subject, or
widen the check. Both are edits to `spec/SPECIFICATION.md` and to `xtask`'s trace checker, and neither
is this story's. Recorded here because this story is what moved the line, and a mover who says nothing
is how a citation rots silently.

### G3 — The crate page's most useful sentence points at the file this exercise may not open. **Recorded, deliberately not changed.**

`crates/happenstance-testkit/src/lib.rs:52-53` tells an author that
`fixtures::MemoryFixture` *"is the reference implementation and the one to read before writing your
own"*. It is the single most useful sentence on the page, and it is the first entry on this exercise's
denylist.

The tension is **not** a defect in the page and the sentence is left alone: for a real outside author,
reading the reference implementation is exactly the right advice, and a documentation set that
withheld it to satisfy an experiment would be worse documentation. What the exercise did establish is
that the projection family had **no equivalent**: no ordered account of what to write, in what order,
with the feature flag and the orphan-rule trap named. That absence is what this change fills — the new
`# Writing a projection adapter from outside this workspace` section — and the section closes by
pointing at `examples/outside-projection-adapter/`, which is a reference an author may read without
the circularity, because it *is* the page's own output.

### G4 — The one compiled example is compiled trivially. **Recorded; the honest limit of a house idiom.**

The new section's single Rust block uses the page's existing `# macro_rules! ignore { … }` hidden-line
idiom, as its two neighbours already do. **Measured, not assumed:** breaking the hidden line turned the
doctest red, which proves the block is collected and compiled — and proves in the same breath that
what `cargo test --doc` type-checks is the hidden line, never
`projection_store_conformance!(MyFixture::new());`.

*The change that would have prevented it.* A genuinely typed example needs a store the page can name,
which on the projection side means `MemoryProjectionStore` or `MemoryProjectionFixture` — i.e. G3's
circularity, plus roughly sixty lines against a density budget of seventy. Not fixed here. If a later
story wants the stronger example, the cheapest honest form is a doctest on
`fixtures::MemoryProjectionFixture`'s own page, where the store is already in scope.

### G5 — The mutant-registry exactness check is not part of the documented extension surface. **Reported. Expected (EC-005), and not worked around.**

`_design.md:388-395` asks this story to clear *"the mutant-registry exactness check and the
capability-skip rule"*. The capability-skip rule is cleared — `tests/outside_projection_capability_skip.rs`
asserts on the returned `RuleOutcome::Skipped` value. The exactness check is **not reachable from
outside**: `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` live in
`crates/happenstance-testkit/tests/projection_mutation_coverage.rs`, over a `Declared`/`Kind` registry
private to that test binary. An outside author has no path to it.

So the discipline that a rule must have a store that fails it — the discipline this repository treats
as the difference between a suite and a decoration — **is an internal discipline**, held for rules
written here and unavailable to an adapter author asking the same question about their own store.
Saying so is a better outcome than the two remedies that were available and were **not** taken:
registering this crate's fixture in the testkit's mutation binary, which would invert the very
dependency edge this story exists to count, and re-implementing the registry inside an example crate.

What an outside author *can* do is what this crate does: name a rule, drive a store they believe is
wrong through it, and assert the failure — `tests/outside_projection_discrimination.rs`. That is a
weaker instrument than the registry and it is the strongest one the published surface offers.

### G6 — `RuleOutcome::Skipped::capability` named only one of the two fixture traits. **Fixed.**

*What was missing.* The field's documentation read *"The name of the [`Fixture`] associated const,
e.g. `"REOPEN"`"*. `tests/outside_projection_capability_skip.rs` asserts
`capability: "RESET_REFUSAL"`, which is a `ProjectionFixture` const — so the page named the wrong
trait, and did not say that the value is the identifier itself rather than a description of it.

*What answered it.* A source read: `crates/happenstance-testkit/src/projection.rs`'s `require!` macro,
whose own documentation states it carries *"`stringify!` of the associated const's own identifier"*.
That is the sentence that belongs on the public type, not on a private macro an outside author cannot
see.

*The change that would have prevented it.* Exactly the change made:
`crates/happenstance-testkit/src/contract.rs`, `RuleOutcome::Skipped`'s two field docs now name both
fixture traits, give an example from each, and point at the two values that are paths rather than
consts. **No item became `pub`**, so this is not a MINOR event under CF-32 and the testkit's version
does not move; it is recorded in `CHANGELOG.md` as documentation.

### G7 — No gap where two were expected. **Stated explicitly, because "no gap" is a finding.**

Two failures were pre-authorised by the spec and **neither occurred**:

- **EC-003 did not fire.** `projection_store_conformance!` expanded in a foreign crate first time. The
  `__private` re-export carries `ProjectionFixture` and the expansion resolved with no
  `error[E0433]` and no `error[E0603]`. This is the first invocation in the workspace that could have
  failed that way — every other one lives in a crate that already has the trait in scope.
- **AC-005's "gap #1" did not fire.** `happenstance_testkit::projection::rules` is already a public
  path, and the `rules` module's own page says outright that a rule may be called directly when
  debugging a single failure. No `pub use` had to be added, so **nothing in this change made a testkit
  item `pub`** and NF-005's MINOR obligation does not arise.

### G9 — Adding a section to a crate-doc block moved eight citations into it. **Fixed — and it is G2's mirror image.**

*What happened.* Inserting 65 documentation lines at `crates/happenstance-testkit/src/lib.rs:175`
shifted everything below it by 65, including four macro definitions the Rust constitution cites.
`cargo xtask lint-constitution` — a gate step — went red immediately, on all eight, each naming the
phrase that had moved:

```console
standards/rust/41-declarative-macros.md:48 — `crates/happenstance-testkit/src/lib.rs:384` no longer
  has `qualified by the caller` within 10 lines; the citation points at the wrong place
```

*The repair.* `+65` on eight citations across `standards/rust/41-declarative-macros.md`,
`62-doctests-and-harnesses.md` and `91-adapter-authoring-recipe.md`; `lint-constitution` back to
*"27 atoms, all consistent"*.

*Admitted rather than merely stated: those three files were **outside this story's PR boundary as
first written**, and the block was amended rather than argued past.* The boundary named the mount
point but not what cites into it, and there is no way to add a section to that page without moving
them — appending to the end of the `//!` block, the only placement that avoids nothing, is the
anti-pattern AC-002 names and would have shifted the same eight lines anyway. So `standards/rust/**`
is now an entry in the fenced block at `spec.md:256`, with the argument at `:261-275` scoping it to
**line-number re-pointing only** — rule text, evidence selection, retirement and new atoms stay out —
and asserting the equal-insertions/deletions property this diff has: **+8/-8**, 4/4 + 3/3 + 1/1. That
is the shape `aef8990` set for the same class of compelled repair, after `redkiln verify` bounced two
sibling stories for it. A widening recorded only here would be a widening the gate cannot read; the
change is mechanical, is entirely line numbers, and is now checkable rather than merely disclosed.

*Why this is the record's most useful pair.* **Two citation checkers run over this repository against
the same kind of claim, and only one of them works.** `lint-constitution` anchors every citation to a
phrase within ±10 lines and caught all eight instantly. `spec-trace` existence-checks 290 of its 359
citations and caught neither of G2's two instances. The instrument that works already exists in this
tree, in `xtask`, applied to a different corpus — which makes G2 a gap with a known fix rather than an
open design question.

### G8 — The reads that went beyond the allowlist, listed in full.

Three, and no others. This list is the answer to "was it copied", and it is offered as a list rather
than as an assurance.

1. **`crates/happenstance-testkit/src/projection.rs`** — the module page, the `must!` / `require!` /
   `require_read_through!` macro documentation, and the **body of one rule**,
   `refused_reset_changes_nothing`, read to establish what a fixture that *declares* `RESET_REFUSAL`
   would be obliged to do. Nothing from it was copied into the fixture; the fixture declines that
   capability. The one thing it answered that no public page did is G6, which is fixed above.
2. **The suite's own failure output.** Ten of the sixteen rules failed on the first run against a
   deliberately incomplete store, and **every one of the ten repairs was made from the assertion
   message alone** — no source read, no reference implementation. This is the most positive finding in
   the record and deserves to be said as plainly as the negative ones: the rules' messages do not say
   "assertion failed", they say what a wrong store looks like and what it costs. *"the checkpoint is a
   high-water mark of consideration, not of application"* rewrote `commit` correctly on its own.
3. **`crates/happenstance-core/src/projection.rs`** in full — allowlisted, since rustdoc renders a
   trait's signatures and its `# Implementing it` doctest. The `ToyStore` walkthrough there **was**
   copied from, for the impl skeleton and for the hand-written `Display`/`Error` pair. It is published
   documentation put there to be copied, and it did its job; recorded so the record is complete.

The denylist held. `crates/happenstance-testkit/src/fixtures.rs`,
`crates/happenstance-core/src/projection_memory.rs`, the testkit's `tests/projection_conformance*.rs`
harnesses and `tests/projection_mutation_coverage.rs` were not opened while this crate was written.

## The stated limits of the exercise

Written down rather than implied, because an unstated limit is how a simulation gets quoted later as a
proof.

- **A workspace member is not a stranger.** This crate depends by path. It cannot `cargo add
  happenstance-core`, because nothing is published, so **initiative DoD 9 — "a stranger can install
  it" from a scratch project outside this workspace (`initiative.md:383-386`) — is explicitly not
  claimed here** and remains `publication-and-positioning`'s. What the position *does* reproduce
  faithfully is exactly what the claim under test is about: the orphan rule, the feature flags, and
  the non-dev dependency graph. It also declines `[workspace.dependencies]`, so the one convenience a
  workspace member gets that an outsider does not is not taken.
- **"Written from the documentation" is unenforceable and stays unenforced.** No tool can distinguish
  this crate from the same crate copied out of `fixtures.rs`. The controls are the allowlist, the
  denylist, the commit ordering above the rule, and this list. That residual risk is accepted and
  stated rather than claimed away.
- **One author, one reading.** Everything above is what *one* attempt found. A second author would
  find a different set, and the number of gaps here is not a measurement of the surface.

## Findings handed onward

For `unstable-projection-gate-and-clause-disposition` (HS-S0016) and the runbook's ADR pass. No `.kb/`
atom is minted here; a decision record is never a side effect of a story.

One row carries a `D` rather than a `G`. The `G` series is reserved for AC-007's question — points at
which the **allowlist** was insufficient — and D1 is not one of those: it is an error found *in the
signed-off record this story reads*, while conforming to it. Numbering it `G10` would have made the
nine-gap count answer a different question than the one it was asked.

| # | finding | owner |
| --- | --- | --- |
| G1 | The port module's first paragraph says the conformance suite does not cover it. False since merge position 6, and coupled to `spec/SPECIFICATION.md:4566-4569`, which quotes it | HS-S0016 |
| G2 | `spec-trace` existence-checks 290 of its 359 citations, so documentation inserted above a cited line rots the citation silently. Two measured instances, one of them pre-existing (`SPECIFICATION.md:7807` → `testkit/src/lib.rs:238`). **The fix already exists in the tree**: `lint-constitution`'s ±10-line phrase anchor, which caught all eight of G9 | HS-S0016 / runbook ADR pass |
| G5 | The mutant-registry exactness discipline is internal-only and is not part of the documented extension surface. Either that is stated on the page as a limit, or the machinery is published | HS-S0016 |
| G4 | The page's compiled examples are compiled trivially by a hidden-line idiom used three times. Worth a decision rather than a habit | runbook ADR pass |
| D1 | `_design.md:379-383`'s cost snippet puts `happenstance-core` with `features = ["conformance"]` under **`[dev-dependencies]`**, which contradicts the argument the same section makes six lines above (`:369-375`): the orphan rule forces `impl ProjectionProbe for MyStore` into `src/`, and `src/` cannot see a dev-dependency — the `error[E0432]` transcript quoted above is that fact measured. The correct section is `[dependencies]`, which is what `examples/outside-projection-adapter/Cargo.toml:20-22` does and what AC-003 and AC-006 specify; the decision is right and only the snippet is wrong. `_design.md` is signed off and **explicitly out of this story's PR boundary** (AC-001, AC-008), so it is reported rather than repaired — but an uncorrected snippet in a signed-off record is the copy the next author pays for | HS-S0016 / runbook ADR pass |
