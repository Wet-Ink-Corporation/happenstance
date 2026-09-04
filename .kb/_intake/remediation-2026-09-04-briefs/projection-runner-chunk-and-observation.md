# Does `chunk` get a named type and a default, and does the runner get an observation seam?

**Record id:** `projection-runner-chunk-and-observation`
**Would supersede:** nothing. **ADR-0036** (`.kb/decisions/0036-the-projection-port-ships-gated.md`, *accepted*) ships the projection port behind `unstable-projection` and records the accepted cost in terms — *"`cargo-semver-checks` will not police the surface"* — which is what prices both questions below at zero. **PS-3** (`spec/SPECIFICATION.md:4928-4929`, `[PROVISIONAL]`) is the clause.

**This brief did not get the two-critic pass the original thirteen had.** It was written by the lane implementing audit entry **R-2 / Y-5**, in the same session as the documentation change it describes. `README.md`'s discount applies.

**Routing, restated:** the audit routes both halves to **phase 6's port-freeze pass**, and neither is a `0.2.0` gate. This brief exists so that pass has the argument in front of it, not to pull the decision forward.

---

## What the exemption buys, and what it does not

It buys **time on the code**. `run_projection`, `Progressed` and `ProjectionError` are gated, the feature is off by default, and the manifest says these items *"make no semver promise at all"*. Giving `chunk` a named type after `0.2.0` is therefore free, and an observed entry point beside `run_projection` is additive regardless. These are genuinely deferrable.

It buys **nothing on the page**. A doc comment is not semver-gated: it is read by whoever runs the thing today, and the operator harmed by the gap is harmed now. That asymmetry is why this lane landed the documentation and left the code alone, rather than deferring both together — deferring both would have been reading the exemption as covering something it does not cover.

It also does not buy freedom from **`SPECIFICATION.md`**. `spec/SPECIFICATION.md:5735` asserts as fact that the runner *"writes nothing anywhere, logs nothing"*. An observation seam has to be reconciled with that sentence, which is a specification edit and not this lane's.

---

## What is true today, after the lane

`crates/happenstance/src/runner.rs`'s page gains two sections, both below the doctest fence (see *A trap this file sets*, below):

- **`# Choosing `chunk``** — what moves in each direction. Small: one `begin`/`commit` pair per event, the most frequent externally visible progress and the highest per-event cost. Large: a write set holding every application since the last commit — an owned statement list on `SqliteProjectionStore` — plus more re-read work on a restart, because a failed chunk is discarded whole. It states that the knob has no default and no named type, that this is a gap rather than a position, and that a measurement nobody has run is what settles it.
- **`# What can be seen while it runs`** — there is no callback, no channel and no `tracing` anywhere in this workspace's `src/`; `Progressed` arrives once, at the end; the durable checkpoint read through a second handle is the only thing an operator can poll.

`crates/happenstance/tests/projection_runner_page.rs` holds both, and was Red on all three checks before the change.

**No behaviour changed.**

---

## Question 1 — does `chunk` get a named type, a default, or both?

### Option 1A — leave it `NonZeroUsize`, documented (as landed)

- **Costs a caller:** they read a paragraph and pick a number. Nothing enforces a sane one.
- **Semver:** none, now or later.
- **Against it:** the crate already decided this question the other way one module over. `Retry` is a named `#[non_exhaustive]` type with two `const` constructors, a `compile_fail` fence and a source-reading test that it has no default. Two caller-supplied bounds in one crate with opposite treatments is a difference a reader will read as meaning something.

### Option 1B — a named `Chunk` type, no default

Mirrors `Retry` exactly: `Chunk::of(NonZeroUsize)`, `#[non_exhaustive]`, and room to grow a variant (`Chunk::adaptive()`) without a break.

- **Costs a caller:** one more name, and `64.try_into()?` becomes `Chunk::of(...)`.
- **What it buys:** somewhere for the paragraph to *live* rather than be inherited from a function's page, and the arity freedom `Progressed`'s own doc comment argues for on the return side.
- **Against it:** a newtype over `NonZeroUsize` that adds no invariant is ceremony. `Retry`'s type earns its keep because `Retry::once()` and `Retry::attempts(n)` are two shapes; `Chunk` has one.

### Option 1C — a named type **and** a default

- **Against it, and this is the argument that has to be answered rather than borrowed:** `Retry`'s page argues a hidden default would hide a worst case nobody wrote down — the number of times a command may be re-run. That argument does **not** transfer. A chunk size is correctness-neutral: every value produces the same read model. So the case against a default here has to be made freshly, and the honest version is weaker: a default hides a *performance* worst case, not a semantic one.
- **What it buys:** the doctest stops teaching a magic number by example, which is how `64` propagates.

### Recommendation on question 1

**1C, at the freeze, and only after the measurement.** The default is the half that actually removes the hazard the audit describes — a caller copying `64`, or reasoning "fewer commits is faster" and writing `1_000_000`. But a default that is a guess is worse than none, because it carries the crate's authority. So: measure first.

**The measurement**, which nobody has run and which belongs in `experiments/`, out of the gate: wall time, peak resident memory and commit count against `SqliteProjectionStore` at chunk ∈ {1, 8, 64, 1024, 65536} over a log large enough that the curve is visible. `references/evaluation/research-rust-api-guidelines.md:420-422` records this codebase reaching the *opposite* verdict on a `NonZeroUsize` parameter elsewhere, which is the reason to measure rather than to reason from the sibling.

**Strongest argument against the recommendation:** if the curve is flat across three orders of magnitude, 1A is right and everything above is ceremony. That is a live possibility and the measurement is what settles it.

---

## Question 2 — does the runner get an observation seam?

### Option 2A — nothing; the checkpoint is the API (as today)

- **What it buys:** the runner stays a function with no policy surface, which is the same argument that beat `on_error: SkipPolicy` on this page and beat it well.
- **Against it:** the checkpoint is observable only to a caller who knows to open a second handle and poll, which the API never says. The page now says it, which converts this from a hidden gap to a documented one — a real improvement, and not the same as a seam.

### Option 2B — `run_projection_observed`, a second entry point taking a callback

`commit`/`commit_with` is the shape this crate already uses for exactly this: the plain door with a choice made, and the full door beside it.

- **Semver:** additive, and free under the exemption.
- **Costs a caller:** nothing; the plain door is unchanged.
- **Against it:** a callback invoked between chunks is a synchronous hook in an async loop, and a caller who does I/O in it stalls the runner. That is documentable and it is a real foot-gun. Also, two entry points is two pages.

### Option 2C — `tracing` spans

- **Against it:** it puts a dependency and an ecosystem choice into the typed layer, and `happenstance-core` has none. The whole workspace has zero `tracing::` today; making this the first is a decision much larger than the runner.
- **What it buys:** the thing operators actually have tooling for.

### Recommendation on question 2

**2B, at the freeze, with 2C explicitly declined at this layer.** A callback is the smallest thing that closes the operator gap, it matches a shape this crate has already justified once, and it leaves the choice of `tracing`-or-not with the application, which is where a library-vs-binary split usually puts it.

**Strongest argument against:** nobody has asked. The gap was found by an audit reading the page, not by an operator hitting it — there are no operators. 2A plus the documentation this lane landed may be the whole correct answer, and the freeze should ask for a reported case before adding a second entry point.

---

## A trap this file sets, recorded because it cost this lane a measurement

Three sentences of `spec/SPECIFICATION.md` cite `run_projection` at `crates/happenstance/src/runner.rs:401` — a line **inside the doctest fence**. Prose added above that fence moves the call down and leaves the three citations pointing at whatever now occupies 401.

`runner.rs` carried a comment saying `spec-trace` catches this. **It does not.** Measured: moving the new `chunk` section above the fence puts the call 39 lines from the cited line, and `spec-trace` reports the identical `401 citations checked (80 anchored to their subject, 12 external)` and exits 0. The checker anchors only the citations whose subject it can derive from the prose beside them, and these three are not among the 80.

Both the comment and the test now say so, and `prose_added_to_this_page_stays_below_the_fence` is what actually holds the line. **The residue is not fixable from this lane:** the three citations point at a line the checker cannot anchor, in `spec/SPECIFICATION.md`, which is another lane's file. Whoever owns it can make them anchorable — cite the `pub async fn run_projection` line, whose subject the prose already names — and then the gate would carry the check the comment claimed.

## Cost of delay

**Zero on code, by ADR-0036's stated exemption.** Non-zero and already accruing on the page, which is why the page moved first. The measurement in question 1 has no deadline either, but it gates 1C, so scheduling it is the only thing that has to happen before the freeze rather than at it.

## What this does not settle

- Whether `Progressed` grows fields. Its own doc comment says the arity is what a later observability pass wants to grow, and 2B would be that pass.
- Whether `SPECIFICATION.md:5735`'s *"logs nothing"* is a normative claim or an observation. 2C would contradict it and 2B arguably would not; the specification's owner decides which.
- Anything about the port itself. Both questions are about `run_projection`'s surface, which is the typed layer's, not `ProjectionStore`'s.
