# CF-23 requires an adapter author to write a name the crate declares is not public API. Does `happenstance-testkit` support the emitters, or say they may change?

Short answer up front: **the contradiction is real, it is published in both
directions at one commit, and the cheap half is landed. What is left is one
sentence in §6.6, and it is a sentence this lane may not write.** The rendered
page now names all twelve emitters, discloses the `#[doc(hidden)]` and discloses
what the attribute costs; what no artefact says is whether a rename is a MAJOR
event.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** It was written by the lane implementing `C2-03`, in the same
session as the change it describes, and nobody independent argued the other side.
Read it with that discount.

---

## Why this is owed

CF-23 `[FROZEN]` (`spec/SPECIFICATION.md:8474-8485`):

> the testkit MUST NOT emit any runtime-specific attribute from its own
> expansion; the per-test wrapper MUST be a parameter supplied by the adapter

The twelve `__emit_*` macros this crate ships are the only concrete instances of
that parameter anyone has. Each carries, in this order and with nothing between
them:

```rust
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_wasm {
```

`#[doc(hidden)]` plus a `__` prefix is Rust's universal declaration that an item
is not public API and may change without notice. The crate applies that
declaration to the one thing a `[FROZEN]` clause requires every adapter author to
name. Both readings are published, at one commit, and they are irreconcilable.

The reach is not hypothetical. `crates/happenstance-cloudflare/tests/durable_object_conformance.rs`
takes it across a crate boundary inside this workspace:

```rust
happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_wasm,
    emit = happenstance_testkit::__emit_wasm,
    fixture = CloudflareFixture::new()
);
```

§6.6, the crate's compatibility policy (`spec/SPECIFICATION.md:8729-8819`),
governs rule addition (CF-29), rule meaning-change (CF-31) and the version key
(CF-32). It says nothing about the emitters.

---

## What is true today, after this lane

Landed, and none of it decides the question:

- The front page names **all twelve** emitters in a four-family table, plus
  `__emit_rule_names`, which is reachable by the same route and wraps no test.
  Before this, three were named — all of the event-store family — and the
  paragraph above the table said *"Three emitters ship"*.
- The page states that the names carry `#[doc(hidden)]`, that they therefore do
  not appear on docs.rs, and that `#[doc(hidden)]` is the marker
  `cargo-semver-checks` uses to exclude an item — so **the one instrument in this
  repository that would report a rename as breaking is the one the attribute
  switches off** (`.github/workflows/ci.yml:375-408`).
- The page states that whether the names are a promise is open, and routes the
  reader to the standing advice that covers them either way: pin this crate
  exactly (CF-30).
- `crates/happenstance-testkit/tests/emitter_surface.rs` holds the table to the
  `macro_rules!` definitions in both directions and bans a written-out count
  beside it.

**The remediation the audit called unavailable is still unavailable, and the
reason is mechanical rather than a matter of taste.** `macro_rules!` lives in a
flat crate-root textual namespace; a private helper is unreachable from a
downstream expansion site
(`references/evaluation/review-conformance-suite.md:356-364`). There is nothing
to seal them behind. `#[doc(hidden)]` is the only tool the language offers for
*"exported because it has to be"*, and its use here is not a mistake.

---

## Option A — support them: documented, named in §6.6, and a rename is MAJOR

A §6.6 clause saying that the emitter names `happenstance-testkit` ships are part
of its public surface, that renaming or removing one is a MAJOR release, and that
the names may be added to in a MINOR one — the same shape CF-29 already gives the
rule set.

- **Costs a caller:** nothing. Their one line keeps working.
- **Costs an adapter author:** nothing, and it removes the question they cannot
  answer today.
- **Costs this crate:** twelve names it can never rename cheaply, including two
  it might well want to — `__emit_rule_names` is not an emitter, and the
  `__emit_benchmark_*` pair belongs to a family CF-34 says is explicitly not a
  bar.
- **Semver:** none to make the promise. Everything after is constrained by it.
- **Instrument:** the gap does not close. `cargo-semver-checks` still cannot see
  them, because `#[doc(hidden)]` is what excludes an item and the attribute
  cannot be removed for the reason above. So the promise would be **held by
  nothing mechanical** — which is the failure mode CLAUDE.md calls decorative,
  arriving one level up. Unless: `emitter_surface.rs` is extended to pin the
  *names* against a committed list, which turns a rename into a red test naming
  the promise. That is cheap and is the version of this option worth taking.

## Option B — declare them unstable: a §6.6 sentence saying they may change

A §6.6 clause saying the emitter names are **not** covered by this crate's
compatibility policy, may be renamed or removed in any release, and that an
adapter depending on one should pin exactly — which CF-30 already recommends for
an unrelated reason.

- **Costs a caller:** a rename can break their one line on a minor bump. CF-30's
  exact pin is what makes that a choice rather than a surprise, and the exact pin
  is already the recommendation.
- **Costs an adapter author:** they must read one more sentence. Against that,
  they get an answer where today they get a contradiction.
- **Costs this crate:** nothing, and it keeps the freedom to fix the two names
  that are wrong.
- **Semver:** none. Declaring them unstable is free **today** and is a major
  release afterwards, because a name shipped twice without a disclaimer is a name
  people have relied on.

## Option C — leave it open and rely on the page

What is landed. The page discloses the attribute, the consequence and the
openness, and §6.6 stays silent.

- **Costs:** the reader gets a straight account of the situation and no answer.
  It is honest and it does not scale: the second release that ships the emitters
  without a decision is the release in which "we never said" stops being
  available.

---

## Recommendation, and the strongest argument against it

**Option B, with A's instrument.** Declare them unstable in §6.6, and extend
`emitter_surface.rs` to pin the twelve names against a committed list so that a
rename is a red test that names the clause rather than a silent break.

Three reasons. The names are *wrong* in two places already and B is the only arm
that keeps them fixable. B costs an adapter author nothing they are not already
told to do — CF-30's exact pin, now on the rendered page. And B is the arm the
audit itself calls dated: *"declaring them unstable and renaming them later costs
nothing today and is a major release afterwards"*.

**The strongest argument against it**, stated in its own terms: CF-23 makes
naming an emitter **mandatory**, and a clause that requires you to write a name
while another clause says the name may vanish is a contract that hands the
adapter author a cost with no way to avoid it. The `!Send` Workers author cannot
use the default arm — `#[tokio::test]` is not available to them — so for that
population the emitter is not a convenience they opted into; it is the only door.
Telling them the door may move is worse than telling nobody anything, because it
is a disclosed defect rather than an undiscovered one. Option A answers that
objection completely and pays for it with two names it cannot fix.

I do not think that argument wins, but it is close, and it is closer than the
audit's framing suggests. It turns on how many adapters exist at the moment the
sentence is written, which is a fact about the future.

## Cost of delay

Low, and lower than the audit estimated. `0.2.0` is not a cliff for A — a promise
can be made in any release. It **is** a cliff for B: the second release that
ships these names without a disclaimer is the release after which declaring them
unstable is itself a break. That release is `0.2.0`.

## What this does not settle

- **Whether `__emit_rule_names` belongs in the set at all.** It wraps no test and
  is not an emitter; it is in the table because it is reachable by the same
  route. If A is chosen, it is the first name to argue about.
- **The `cargo-semver-checks` blind spot as a class.** `#[doc(hidden)]`
  exclusion is a fourth entry for `HS-S0091` `registry-surface-diff`'s B12 list
  of what that tool does not check, and the emitters are what makes it material.
  This brief does not write that list.
- **Whether §6.6 acquires the clause at all**, as against a paragraph. That is
  the specification pass's call and it owns the maturity marker.
