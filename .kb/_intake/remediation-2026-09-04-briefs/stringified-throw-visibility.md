# Should `StringifiedThrow` be public at all, now that its field is not?

**Produced differently from the first thirteen.** Written by the lane implementing
`G-2` and `N-1`, in the same session as the change it describes, and **without**
the author → two independent critics → revision pass the first thirteen had. It
carries its own strongest objection and answers it, which is the form, but nobody
independent argued the other side. Read it with that discount applied.

Short answer up front so the rest reads as evidence: **this brief does not
recommend removing it, and it does not recommend keeping it either — it names the
one measurement that decides, and that measurement does not exist yet.**
Confidence **medium-low**, and the reason is stated in *What would settle it*.

---

## What has already been decided, so this is not re-opened by accident

`G-2` charged that `StringifiedThrow` was the one error type in the five
publishable crates a caller could mint *and* mutate, and that it exposes a
classifier reading the mutable field. That half **is settled and landed** in this
lane:

- `message` is private; `StringifiedThrow::message()` is the read.
- Two `compile_fail` doctests, each with a compiling twin differing by one
  expression, hold the mint and the assignment respectively
  (`crates/happenstance-cloudflare/src/js.rs`).
- `#[non_exhaustive]` was **not** applied on top, and the reason is written beside
  the type: with the field private the struct expression is already unbuildable
  downstream and no downstream pattern can be exhaustive, so a second field is
  additive either way and the attribute would restate a seal the field holds.
  **RS-13-5** is the standing argument against a decoration that buys nothing.

The audit routed the choice *between* RS-13-1 (private field) and RS-13-3
(`#[non_exhaustive]` with a public field) to a decision record rather than a line
edit. The lane took it rather than deferring it, for a reason the audit states in
its own **Remediation** paragraph: **RS-13-3 does not close the second hazard.**
The attribute blocks the literal and forces `..` in a pattern and does nothing to
`throw.message = "…".into()` on a value already held, so choosing it would have
closed the semver half and left the classifier hazard untouched. Where one of two
written rules does not do the job the finding is about, the choice is not a fork.

That is stated here so the ingest pass can see the decision was taken and on what
grounds — not so it is re-litigated.

**What was left open is the audit's own prior question**, quoted verbatim:

> Whether `StringifiedThrow` should be public at all, given its two in-crate
> roles are a recorded alternative and a test control, is a prior question and is
> not this document's to take.

---

## Why it is owed

### 1. Both of its in-crate roles are satisfied by a private type

`crates/happenstance-cloudflare/src/lib.rs`, the `!Send` probe module's positive
control:

```rust
assert_send!(
    StringifiedThrow,
    "it holds a String, which is the entire point of the stringified shape"
);
```

That is `crate::js::StringifiedThrow`, named from inside the crate. `pub(crate)`
satisfies it exactly.

The other role is documentary: it is the recorded ES-6 alternative, the shape the
port would have forced on every adapter if `Error: Send + Sync` had been added.
`crates/happenstance-cloudflare/src/sql_storage.rs` states the fork on `SqlError`
and names the type; `spec/SPECIFICATION.md:2694` and `:2701` cite `js.rs` for ES-6
and cite `JsThrow`'s declaration, not this one. A `pub(crate)` type is still
documented in the tree and still readable in the source; what it stops being is a
rendered page a consumer can build against.

### 2. Nothing in the workspace consumes it as a consumer would

Every use is inside the defining crate: the two constructors, the classifier, the
probe's positive control, and four `wasm32` unit tests. The two `references/adr/`
records and the `.bklg/` specs mention it in prose. Nothing constructs it from
outside `happenstance-cloudflare`, and after this lane's change nothing outside
can.

### 3. The crate root actively invites a consumer down this road

`crates/happenstance-cloudflare/src/lib.rs`, Finding 2, says stringifying "loses a
capability, not information the caller needs" and that on the one question the
port makes a caller ask the two shapes answer identically. A reader who takes that
sentence at face value reaches for this type. That is the strongest reason it is
public and the strongest reason the question is not free: **the page argues for a
route the API would then withdraw.**

---

## The options

### Option A — keep it `pub`, as it is now

**Costs a caller:** nothing new. They gain `message()` and lose a constructor they
should not have had.

**Costs an adapter author:** nothing.

**Semver:** none — this is the state as of this lane.

**Against it:** it publishes a type whose only two jobs are internal, and every
public item is a promise that costs something to keep. It also leaves the crate
publishing *two* error shapes for one boundary, which a reader has to be told how
to choose between; the crate root does tell them, at length, which is itself
evidence the surface is doing work prose has to undo.

### Option B — `pub(crate)`

**Costs a caller:** a consumer who took Finding 2's invitation and built the
`Send + Sync` route themselves loses the type they built it out of. They can
rebuild it in three lines — a `String` and a `contains` — which is the honest
measure of what it was giving them.

**Costs an adapter author:** nothing.

**Semver:** **breaking**, and free on the same window as everything else here: the
crate holds a `0.0.0` placeholder on the registry, so no released version carries
it. It stops being free at the release that first ships this crate.

**Against it:** it deletes the artefact ES-6's record points at. The specification
cites `js.rs` for the fork; making one arm of the fork unreachable from outside
makes the record harder to check by a reader who has only docs.rs. That is a real
cost and it is the reason this brief does not simply recommend B.

### Option C — keep it `pub` and remove `is_constraint_violation` from it

The classifier is the only reason the fabrication mattered. Without it the type is
an inert `String` wrapper and the hazard is gone by construction rather than by
seal.

**Costs a caller:** the comparison the crate root makes — *the two shapes answer
identically on the one question* — stops being something a caller can run. It
becomes a claim in prose backed by a test that no longer has a public counterpart.

**Semver:** breaking, same window.

**Against it, and it is decisive for this lane:** the classifier is the *evidence*
for ADR-0009's finding, not a convenience on top of it. `js.rs`'s
`a_unique_violation_reads_the_same_live_and_stringified` compares the two
classifiers directly and is the assertion Finding 2 rests on. Removing the public
half to remove a hazard the private field already removed trades a measurement for
nothing.

---

## Recommendation

**None, deliberately** — and unlike `transient-contention-tolerance.md`, which
declines because the deciding instrument does not exist, this one declines because
the deciding *evidence* is a fact about people rather than about types.

The question is: **does anyone outside this crate ever want a `Send + Sync` error
shape from a Workers adapter?** If yes, Option A and the crate root's Finding 2
are one coherent offer. If no, Option B and the type is an internal instrument
that leaked. Nothing in the tree answers that, because the crate has never been
published and has no consumers to observe.

If forced to choose today the lane would take **A**, on the weakest of the three
arguments: it is the state that already holds, the hazard that made the question
urgent is closed, and B and C are both still free at the same moment A is. Taking
a breaking change now to reach a state that is equally reachable later is spending
a window for nothing.

**The strongest argument against that**, stated in its own words: *"free until
publication" has been the reasoning behind every deferral in this directory, and
`N-2` is the finding that reasoning of exactly this shape had already gone stale
once — ADR-0029 raised the MSRV on a "nothing is published" premise that no longer
held. A window that everything is deferred into is a window that will be crowded
when it arrives, and the release that ships this crate is the release that has to
decide seventeen of these at once.* That objection is not answered here. It is why
the confidence is medium-low.

## Cost of delay

Low until the release that first ships `happenstance-cloudflare`, and then
permanent. The crate is `publish`-eligible and gate-held to publication standards
today but explicitly deferred past `0.2.0` (`.github/workflows/ci.yml`), so there
is more room here than on the `0.2.0` items in this directory — which cuts both
ways, per the objection above.

## What this does not settle

- Whether `JsHandle` and `JsThrow` should be public, which is the same question
  with a different answer available: both are genuinely reachable from
  `SqlError::Thrown` on a caller's error path, and `StringifiedThrow` is not.
- Whether `happenstance-cloudflare` publishes a feature table before it ships,
  which the audit's `RS-40` entry routes separately.
- Anything about ADR-0009 or ADR-0023. Neither is amended, quoted as amended, or
  needed to read this.
