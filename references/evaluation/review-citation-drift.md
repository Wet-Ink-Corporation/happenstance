# Review — citation drift and stale claims

**Lens:** do the repository's own `file:line` citations and explanatory comments
still say what they claim?
**Date:** 2026-08-10 · **Repo:** `D:\repos\happenstance` @ `3712c9b` + uncommitted
working tree · **Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)`

---

## Provenance, and why that matters for how you read this

These are **byproducts**. They were surfaced by the adversarial passes that built
[`standards/rust/`](../../standards/rust/README.md), not by a review commissioned to find them, and
every one of them lies outside that work's scope. Nothing here has been changed.

The passes were run by agents, and an agent's report is a claim, not a finding.
Every item below was therefore re-verified directly before being written down —
by reading the cited line, by running the compiler, or by swapping the code and
observing what happened. **Three agent claims did not survive that check and are
recorded in §6 as refuted**, because a later reader who hears the same claim from
the same kind of source should be able to find out cheaply that it was already
tested.

Where a finding names a line in `xtask/src/main.rs`, the number is the one in
`3712c9b`. The working tree at the time of writing carries an uncommitted
32-line insertion into that file, which shifts it.

---

## 1. Six citations resolve, and point at the wrong line

`spec-trace` is green on all six.

| Citation site | Cites | Actually at |
|---|---|---|
| `SPECIFICATION.md:2460` | `memory.rs:364-391` — `send_flavour_stream_is_send_in_generic_code` | `memory.rs:614` |
| `SPECIFICATION.md:2469` | `memory.rs:393-453` — `spawns_from_generic` | `memory.rs:643` |
| `SPECIFICATION.md:4473` | `memory.rs:364-391`, same test | `memory.rs:614` |
| `SPECIFICATION.md:2569` | `store.rs:100` — `type Error: core::error::Error + 'static` | `store.rs:101` |
| `SPECIFICATION.md:2593` | `store.rs:100`, same item | `store.rs:101` |
| `docs/adr/0009-error-send-sync.md:15` | `store.rs:100`, `projection.rs:73` | `store.rs:101`, `projection.rs:90` |

`store.rs:100` is the last line of the doc comment above the item — off by one,
harmless to follow. The `memory.rs` citations are the ones that mislead: `:364-391`
lands inside `MemoryEventStore::append`'s body, 250 lines above the test it names,
where a reader finds the write lock and the condition check and could plausibly
believe that *is* the evidence for a clause about stream `Send`ness.

**The diagnosis is not new, and that is the finding.**
[`RUNBOOK.md:1713-1716`](../../RUNBOOK.md) already states it exactly:

> Note that `spec-trace` passes over all fourteen and always would:
> `check_citations` validates existence and a first-number bound, not that the
> cited line still says the cited thing.

And [`RUNBOOK.md:354-356`](../../RUNBOOK.md) records line-number rot as *already
dealt with* — "that was fixed in phase 2's own commit, fourteen citations
re-anchored". Six are stale again at `3712c9b`. So the phase-2 re-anchoring was
either incomplete or has already decayed, and it decayed silently because the
only instrument that could have noticed was the one the RUNBOOK correctly
predicted would not.

**A working implementation now exists to port.** `standards/rust/`'s citations carry a
quoted anchor beside the line number and `xtask lint-constitution` asserts the
anchor occurs within ±10 lines. It caught this class of drift three times during
that work, **twice on citations I had invalidated myself** by inserting 32 lines
into `xtask/src/main.rs` — which is precisely the mechanism that produced the six
rows above. The code is `parse_citation` and `check_citations` in
`xtask/src/lint_constitution.rs`, and it is about forty lines.

---

## 2. ES-6 is `[FROZEN]` on a conformance rule that does not exist

`SPECIFICATION.md:8407` marks ES-6 `FROZEN`, checked by
`store_error_crosses_a_join_handle`. The rule is named in `SPECIFICATION.md`,
`docs/adr/0008`, `docs/adr/0009` and `RUNBOOK.md`.

Verified at `3712c9b`:

- `ThreadSafeEventStore` occurs in **zero** `.rs` files.
- `store_error_crosses_a_join_handle` occurs in **three**, all of them comment
  lines in `happenstance-cloudflare` — `src/lib.rs:92`, `src/send_shape.rs:14`
  and `src/send_shape.rs:95`. None is a `fn`.

`happenstance-cloudflare/src/lib.rs:92` states the reason in its own words: the
rule "is unwritable against today's port". `send_shape.rs` is the demonstration —
`SendStoreWithLocalError` implements `SendEventStore` with a `Send` store, a
`Send` stream and a `Send` future but a `!Send` `Error`, **and compiles**. The
obstruction is the port's shape, so it is not specific to that adapter.

This is CLAUDE.md's decorative-rule bar applied to a clause: ES-6 is frozen
against an instrument nothing can run. It is owed an explicit disposition —
either the rule gets written, or the clause is demoted and the reason recorded.
A `FROZEN` marker citing a rule that cannot exist is the strongest maturity
marker in the vocabulary attached to the weakest evidence.

---

## 3. `SequencePosition` relies on a niche it does not pin

`crates/happenstance-core/src/event.rs:219-221` promises:

> Backed by `NonZeroU64`, which makes position zero unrepresentable and lets
> `Option<SequencePosition>` occupy the same eight bytes as a bare
> `SequencePosition`.

A doctest at `:233-237` asserts it. The type at `:239-240` carries `#[derive(…)]`
and **no `#[repr(transparent)]`** — verified: zero occurrences of
`repr(transparent)` anywhere in `happenstance-core/src/`.

The niche is guaranteed for `NonZeroU64` itself. For a `struct` wrapping it,
default `repr(Rust)` makes the layout an unguaranteed — if entirely reliable —
optimisation. The doctest will keep passing either way, so it does not
distinguish "guaranteed" from "currently true". If the doc comment is making a
promise, `#[repr(transparent)]` is the attribute that makes it one.

Low severity, and deliberately so: this is a documentation-vs-attribute mismatch,
not a defect. It is recorded because the sentence reads as a layout guarantee to
an adapter author sizing a wire format.

---

## 4. Three comments that no longer match the code

### 4.1 `query.rs:157-158` names the one pattern spelling that does not compile

The comment on `Query::Items`:

> Inside this crate the variant is ordinary; outside it, it can be matched only
> as `Query::Items(..)` and constructed not at all.

The second clause is right. **The first is backwards**, and this repository
already contains the measurement that refutes it — a `#[cfg(doctest)]` block at
`crates/happenstance-testkit/src/lib.rs:248-256`, measured on 1.97.1:

> `error[E0603]: tuple variant `Items` is private` downstream, because a tuple
> pattern resolves through the variant's *constructor* and `#[non_exhaustive]`
> is what makes that constructor private outside the crate. Braces reach the
> fields directly and never name the constructor, so `{ .. }` and `{ 0: …, .. }`
> both compile.

So `Query::Items(..)` — the exact spelling `query.rs` offers as the one that
works — is the spelling that fails downstream. Two comments in one workspace
contradict each other, and the wrong one sits on the public type. An adapter
author who follows it writes a pattern that compiles in-crate and breaks at their
own crate boundary.

*(Note: an agent reported this as "the second half is wrong". It is the first
half. The finding is real; the direction was inverted in the report.)*

### 4.2 `testkit/src/lib.rs:313-314` states a mechanism that is not the mechanism

> The general form. Listed first so that arm matching never has to back out of
> `fixture = $fixture:expr` to reach it.

**Measured, not reasoned.** I swapped the two arms — putting the short form at
`:343` first and the general form second — and rebuilt:

```console
cargo test -p happenstance-testkit --test memory_conformance --test local_conformance
test result: ok. 91 passed; 0 failed
```

Every call site still compiles, including `local_conformance.rs:475`, which uses
the short form, and `:464`/`:524`, which use the general one. The arms are
disambiguated by a literal token — `emit` versus `fixture` — before either
reaches a fragment specifier, so no arm ever commits to parsing an `expr` and no
backing out is required in either order.

The `expr`-fragment hazard the comment invokes is real in general: once an arm
commits to an `expr`, an error inside it is fatal and later arms are not tried.
It simply is not reachable here. The ordering may still be worth keeping; the
stated reason is not why. Reverted immediately after measuring.

### 4.3 `xtask/src/main.rs` carries two stale MSRV references

Both at `3712c9b`:

- **`:578`** justifies the nightly docs.rs step with "1.85 stable and no
  contributor should need a second toolchain to run". The MSRV is 1.97.1 since
  [ADR-0029](../../docs/adr/0029-msrv-raised-to-1-97-1.md).
- **`:825`** explains a suppression via "`clippy.toml`'s `msrv = 1.85`".
  `clippy.toml:1` reads `msrv = "1.97.1"`.

Neither changes behaviour. Both would mislead someone auditing why the step
exists, which is the only reason either comment was written.

---

## 5. `CLAUDE.md:209-210` attributes an MSRV to the wrong crate

> `--no-dev-deps` is exactly the flag that hides `proptest` and `tokio` — both of
> which declare `rust-version = "1.85"`, leaving no headroom at all.

Resolved from `Cargo.lock` via `cargo metadata`:

| Crate | Version | Declared `rust-version` |
|---|---|---|
| `proptest` | 1.11.0 | **1.85** |
| `tokio` | 1.53.1 | **1.71** |

`proptest` is as described. `tokio` is not, and at 1.71 it is nowhere near the
floor. The sentence's conclusion — that the job proves nothing while the pin and
the floor are equal — survives, because `proptest` alone carries it. Only the
"both" is wrong.

---

## 6. Three agent claims that did not survive verification

Recorded so they are not re-derived.

**`sync/src/ingest.rs` is stale about what `SequencedEvent` carries — refuted.**
The comment at `:45-46` says it "now carries `position`, `id`, `recorded_at` and
`event`". `event.rs:484-496` defines exactly those four fields, in that order.
The comment is not stale; it is a self-correction that already records the
phase-4 change. The agent appears to have read `:42-44` — which describes the
*superseded* state, and says so in the same sentence — as the claim.

**`ci.yml` repeats the 1.85 attribution — refuted.** The passage at `:262-264`
names `proptest` and `tokio` as the crates `--no-dev-deps` hides, and states no
version number at all. §5's defect is confined to `CLAUDE.md`.

**`query.rs`'s second clause is wrong — refuted, but the finding stands.** See
§4.1: the defect is in the first clause.

---

## 7. Open judgment calls inside `standards/rust/`

Eight rules whose repairs the verifying pass marked unsound. All are
clause-attribution judgments — whether an atom should cite a given
`SPECIFICATION.md` clause — and none is a claim about Rust. They are listed here
rather than settled because the specification's author owns the boundary, and
because each further adversarial round found overlap subtler than the last, which
is the signature of a question that has stopped being empirical.

| Rule | Question |
|---|---|
| `RS-13-4` | Cites VT-4, which governs `SequencedEvent`'s store-assigned fields; the rule is about `EventParts`, whose fields are writer-known. The spec labels `into_parts` "Prose, not a clause". Probably belongs with no citation. |
| `RS-25-3` | Cites ES-3 for `Arc<T>: Send requires T: Send + Sync` — a `std` fact ES-3 does not carry. |
| `RS-23-5` | Its named wrong implementation is not in the tree: `grep "impl Drop" crates/` returns nothing. Does a hypothetical author error clear the decorative-rule bar? |
| `RS-20-2`, `RS-24-3`, `RS-62-1`, `RS-62-3`, `RS-91-3` | Residual clause overlap, of decreasing severity. |

`RS-91-2` and `RS-92-4` were deleted during that pass. Their IDs are retired and
must not be reused; `lint-constitution` C11 enforces uniqueness but cannot know
about a retirement it is not told.

---

## What this document does not check

Stated because a review that does not bound itself gets read as exhaustive.

- **It is not a citation audit.** The six in §1 are the ones the constitution
  work happened to touch. `SPECIFICATION.md` carries hundreds; none of the rest
  was examined, and the mechanism that produced these six is
  insertion-above-a-cited-line, which is indiscriminate. Assume there are more.
- **It checks no clause's truth**, only whether citations land and comments
  describe the code beside them.
- **It is not pinned by any gate.** Nothing cites this file, and if the six
  citations in §1 are re-anchored tomorrow, this document becomes wrong with no
  step going red. That is the same property every other document in this
  directory has, and [`README.md`](README.md) explains why it is deliberate.
