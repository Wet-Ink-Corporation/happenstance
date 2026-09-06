# What is `assert_domain_event` for — does it gain a positional-agreement precondition — and should every rendered `decode` match `event_type` against `EVENT_TYPES`?

Decision record **Y-2-B-3-domain-event-guard**. Sources: audit entries Y-2
(`references/evaluation/review-pre-publication-2026-09-03.md:3506-3551`) and B-3
(`:3553-3598`). This is a brief; the human decides.

**Read this first.** Two of the audit's census claims do not survive re-measurement, and
one of them changes which option is cheapest. §"What is true today" states the corrected
numbers before anything is priced on them.

**And read this second.** This brief has been revised after review. Its first version
rejected Option B, and the reason it called *decisive* was itself an unmeasured claim; it has
been removed and **the recommendation has flipped**. §"Revision record" at the foot says
exactly what changed and what changed it.

---

## Why this is owed

### The guard cannot see the hazard the code style creates

`crates/happenstance/src/domain.rs:70-75` states, as the trait's own rationale for
returning `EventType` **by value**:

```rust
/// Returned **by value**, which costs a borrowed-`Cow` clone and no
/// allocation. `-> &'static EventType` lost: `EventType` holds a
/// `Cow<'static, str>`, so const promotion does not apply and the
/// implementation would have to index `EVENT_TYPES` by position — a second
/// place to get the mapping wrong, which is the whole hazard this trait
/// exists to remove.
```

Sixty-two lines later, that same file's rendered doctest indexes `EVENT_TYPES` by
position — `crates/happenstance/src/domain.rs:135-139`:

```rust
///     fn event_type(&self) -> EventType {
///         match self {
///             Self::Taken => Self::EVENT_TYPES[0].clone(),
///             Self::Freed => Self::EVENT_TYPES[1].clone(),
```

So does every other rendered doctest, and so does every impl in `examples/`. The
documentation teaches the exact shape the trait's rationale says the trait exists to
remove.

**The hazard that opens.** An author alphabetises `EVENT_TYPES` — three lines, no `match`
arm touched, reads as cosmetic. From that commit every event of the affected variants is
appended under the wrong `EventType`, and `commit` returns `Ok`. Nothing in the workspace
goes red, and each reason is checkable in the tree:

- `Boundary::query` derives **one** `QueryItem` over *all* declared types —
  `crates/happenstance/src/boundary.rs:118` calls `derive_query(M::Event::EVENT_TYPES,
  self.scope())`, and `derive_query` at `:172-175` is `QueryItem::new(types.iter().cloned(),
  scope.clone())`. Order is not observable; the reordered set is the same set, so the query
  still nominates every affected event.
- `absorb`'s filter is `query.matches(...)` (`boundary.rs:138`), so they still match.
- `decode` ignores the envelope type at every site (below), so the fold still applies the
  correct arm off serde's own variant identifier — the state folded is *right* while the
  bytes on disk are *wrong*.
- The worked example's end-to-end transcript test asserts membership only —
  `examples/course-subscriptions/tests/runs.rs:199-202`, whose whole predicate is
  `EVENT_TYPES.contains(&event_type)` at `:200`.

**And the named guard cannot see it.** ADR-0020 is `status: accepted` and immutable
(`.kb/decisions/0020-fold-query-agreement.md:5`). It names the mitigation at
`:111-112`:

> `EVENT_TYPES` ↔ `event_type()` agreement is not compiler-enforced and is tested by
> `assert_domain_event::<E>(&[…])` — the residual is named rather than hidden.

The long form states the residual narrowly — `references/adr/0020-fold-query-agreement.md`,
the *"A price the shape does not pay for"* paragraph:

> a hand-written `event_type()` may return a type **absent from** `EVENT_TYPES` and no
> `const` sees the match arms.

That is a **membership** claim, and `assert_domain_event` is exactly a membership test
(`crates/happenstance/src/testing/mod.rs:381-382`). A reorder returns a type that is still
a member, so the guard passes. **ADR-0020's guard is structurally unable to detect a
reorder**, and that is not the accepted price re-reported — it is a second hazard.

### Every rendered `decode` throws away the parameter ADR-0021 Decision 3 rests on

`references/adr/0021-payload-evolution-and-codec-tag.md:245-252` earns *"no read-path hook
is needed"* on precisely this signature:

> `decode` receives the event type **and** the raw bytes and returns a `Result<Self,
> CodecError>`. An implementation may therefore try the current payload shape, fall back to
> an older one, and construct the current variant from it — entirely inside the typed layer,
> with no port change.

An implementation that discards `event_type` has no discriminator to branch on, so the
strategy the ADR names as the reason a hook was not needed is not merely unused — it is
unavailable to a reader who copies the crate's own examples.

---

## What is true today

### `assert_domain_event`, in full

`crates/happenstance/src/testing/mod.rs:378-394` — this is the whole function:

```rust
pub fn assert_domain_event<E: DomainEvent>(every_variant: &[E]) {
    for value in every_variant {
        let carried = value.event_type();
        assert!(
            E::EVENT_TYPES.contains(&carried),
            "`event_type()` returned `{}`, which EVENT_TYPES does not declare: \
             [{}]. The declaration and the value have drifted, and no compiler \
             can see it — this is the check that can",
            carried.as_str(),
            E::EVENT_TYPES
                .iter()
                .map(happenstance_core::EventType::as_str)
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
}
```

Signature: `pub fn assert_domain_event<E: DomainEvent>(every_variant: &[E])`. It reads
**`event_type()` only**. It never calls `tags()` and never calls `encode`/`decode`. It is a
per-element membership test with no cross-element and no positional reasoning, and the
doc-comment at `:334-345` states no ordering precondition — only *"Hand this every variant
of your domain enum."*

The published function is live: `happenstance` `0.2.0-alpha.1` was published to crates.io
on 2026-08-16
(`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/_release-log.md:33`,
*"Published | **yes** — all three live, 2026-08-16 21:36–21:37 UTC"*), and the packaged
copy carries this exact body at
`target/package/happenstance-0.2.0-alpha.1/src/testing/mod.rs:378`.

### The census — corrected

The audit reports *"Thirty-one `impl DomainEvent` blocks … thirty-one `event_type` bodies;
**exactly one** does not index by position."* Re-measured over `crates/happenstance/` and
`examples/`:

| | audit | measured |
|---|---|---|
| `impl DomainEvent` blocks | 31 | **30** |
| `event_type` bodies | 31 | **30** |
| `EVENT_TYPES[` lines / files | 27 / 12 | **27 / 12** ✅ |
| bodies that index by position | 29 (implied) | **13** |
| bodies that do **not** | 1 | **17** |

Both of the audit's "thirty-one"s are raw `grep -c` line counts. The `impl` grep counts a
string literal at `examples/course-subscriptions/tests/runs.rs:510`
(`MAIN.contains("impl DomainEvent for Enrolment")`); the `event_type` grep counts the trait
*declaration* at `crates/happenstance/src/domain.rs:76`. Net of both, 30 impls and 30
bodies — an immaterial correction.

**The material correction is the last two rows.** The claim *"the census is categorical, not
partial"* (`:3532`) is **false for the tree as a whole**. Of the 22 real (non-doctest)
impls, **six** index by position and **sixteen** use named `const EventType` items. The
named-const pattern is not hypothetical — it is the crate's own house style in its own test
corpus, e.g. `crates/happenstance/src/testing/tests.rs:27-29,45-53`:

```rust
const DEPOSITED: EventType = EventType::from_static("Deposited");
const WITHDRAWN: EventType = EventType::from_static("Withdrawn");

impl DomainEvent for Ledger {
    const EVENT_TYPES: &'static [EventType] = &[DEPOSITED, WITHDRAWN];

    fn event_type(&self) -> EventType {
        match self {
            Self::Deposited { .. } => DEPOSITED,
            Self::Withdrawn { .. } => WITHDRAWN,
        }
    }
```

That impl is **immune to a reorder of `EVENT_TYPES` by construction.** So is every one of
the other fifteen.

**What *is* categorical is the copy-pasteable surface**, and the audit is right about that:

- All **seven** rendered doctests index by position — `lib.rs:39`, `domain.rs:137-138`,
  `runner.rs:374`, `boundary.rs:46`, `composition.rs:128-129`, `testing/mod.rs:23`,
  `testing/mod.rs:364-365`. (An eighth `event_type` body, `domain.rs:33-34`, sits inside a
  `compile_fail` fence whose `EVENT_TYPES` is `&[]` and therefore cannot be indexed.)
- All **five** impls in `examples/` index by position —
  `examples/course-subscriptions/src/main.rs:193-197`,
  `examples/transfers-on-sqlite/src/main.rs:286-292`,
  `examples/transfers-on-sqlite/tests/contention.rs:408-412`, and both trybuild fixtures at
  `examples/course-subscriptions/tests/ui/{handled,unhandled}_variant.rs:46-51`.
- Exactly one in-crate impl indexes by position —
  `crates/happenstance/tests/mounted_at_the_crate_root.rs:37`.

**This matters for the decision.** The hazard is confined to the surface the crate
*publishes as a model*, and the tree already contains a proven, compiling, reorder-immune
alternative for it.

### `decode` — every copy-pasteable site discards the parameter

Confirmed at **eight** doc sites (the audit named five and missed `domain.rs:145-146`,
`testing/mod.rs:28-29` and `runner.rs:379-380`); seven are rendered, one
(`domain.rs:40-41`) is inside the `compile_fail` fence. Two quoted, as required:

**`crates/happenstance/src/lib.rs:44-45`** — the crate root's first program, the single most
copied block in the library:

```rust
//!     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
//!         -> Result<Self, CodecError> { c.decode(d) }
```

**`crates/happenstance/src/testing/mod.rs:372-373`** — inside `assert_domain_event`'s own
doctest, i.e. the guard demonstrates itself on an impl that discards the parameter:

```rust
///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
///         -> Result<Self, CodecError> { c.decode(d) }
```

The parameter is `event_type: &EventType` — the second parameter of
`DomainEvent::decode`, `crates/happenstance/src/domain.rs:95-99`. Both worked examples spell
the discard out longhand: `examples/course-subscriptions/src/main.rs:216-222` and
`examples/transfers-on-sqlite/src/main.rs:306-312` both bind it as `_event_type: &EventType`
and call `codec.decode(data)`.

**A second correction the audit does not make, and it is favourable.** The tree already
contains the remedy B-3 asks for, compiling and passing, in **six** in-crate impls.
`crates/happenstance/tests/projection_runner.rs:75-86` is the cleanest:

```rust
fn decode<C: Codec>(
    codec: &C,
    event_type: &EventType,
    data: &Bytes,
) -> Result<Self, CodecError> {
    if !Self::EVENT_TYPES.contains(event_type) {
        return Err(CodecError::UnknownEventType {
            event_type: event_type.clone(),
        });
    }
    codec.decode(data)
}
```

Same shape at `crates/happenstance/src/tests.rs:51-62`, `tests/flavours.rs:49-60`,
`tests/codec_tag.rs:62-73`, `tests/command_loop.rs:53-64`, `tests/composition.rs:46-57`. The
remedy is a lift from the test corpus into the docs, not a design exercise.

**But note precisely what that pattern buys and what it does not.** It is a *membership
guard*, and it restores the drift signal `boundary.rs:105-107` sells:

```rust
/// Returns [`CodecError::UnknownEventType`] when a **nominated** event is
/// not one the domain type declares, because that is a real disagreement
/// between the declaration and the fold
```

It does **not** make the decode envelope-*dispatched*. serde's variant identifier is still
what selects the variant. **Nothing in the workspace does envelope dispatch.** And no
`#[serde(rename)]` appears anywhere in `crates/happenstance/` or `examples/` (grep returns
nothing), so the derives are externally tagged by default and the rename hazard the audit
names at `:3590` is real: renaming a Rust variant makes every already-written payload of
that variant permanently undecodable while the envelope's `EventType` sits in the store
holding the correct name that nothing reads.

### Governing records

- **ADR-0020** — `.kb/decisions/0020-fold-query-agreement.md`, `status: accepted`,
  `superseded_by: null`, `reversibility: medium`. **Immutable.** Its residual statement
  remains true; nothing here contradicts it. It names the guard at `:111-112` and the long
  form scopes it to *absent from* `EVENT_TYPES`.
- **ADR-0021** — `.kb/decisions/0021-payload-evolution-and-codec-tag.md`, **superseded by
  ADR-0032**. **ADR-0032 is current.** It supersedes 0021 but explicitly carries all three
  decisions forward —
  `.kb/decisions/0032-adr-0021-serde-attribution-correction.md:14-19`: *"all three of
  ADR-0021's decisions carry over intact … upcasting happens at decode with no read-path
  hook added to the frozen `EventStore`."* What 0032 withdraws is one *justification for one
  rejected alternative* (the mis-attributed ADR-0003 ground against a serde-encoded framing
  region), not Decision 3. So Decision 3's substance is live and its current home is
  ADR-0032.
- **Which atom would a new record supersede? None.** ADR-0020's residual is not being
  withdrawn — it is being *extended* to a hazard 0020 did not price. ADR-0032's Decision 3 is
  not being withdrawn — B-3 **corroborates** it by showing its falsifier has a closer failure
  mode than the cross-event upcast anticipated at `references/adr/0021:255-261`. A record here
  is a **new atom depending on kb-decision-0020 and kb-decision-0032**, superseding neither.
  (Explicitly out of scope for this brief: proposing that atom's prose, status or
  frontmatter.)

### What the SPECIFICATION says

**Nothing.** `grep -n "DomainEvent" spec/SPECIFICATION.md` returns one hit,
`spec/SPECIFICATION.md:1416`, and it is prose about `happenstance-core`'s `EventType`
interning, not a clause. `assert_domain_event`, `EVENT_TYPES` and `event_type()` appear in
no clause at all. The clause space (`ES-*`, `VT-*`, `WF-*`, `PS-*`) governs
`happenstance-core`; the typed layer is governed by ADRs only. **There is no clause id and
no maturity marker to cite here**, which is itself the reason this needs a decision record
rather than a spec edit — and the reason `cargo xtask spec-trace` will not catch a
regression in it.

---

## Options

Two independent questions are entangled here, and they should be priced apart, because
**they have different deadlines**:

- **(1) the taught shape** — what the rendered docs and examples model. Semver **none**,
  but B-3's durability asymmetry gives it a real 0.2.0 deadline.
- **(2) the guard** — what `assert_domain_event` checks. This is where the semver split
  lives.

### Option A — fix the taught shape only. Named consts in every rendered doctest and both examples; membership-guarded `decode` at the copyable sites. No API change.

Rewrite the seven rendered doctests and the five `examples/` impls to the named-const
pattern already proven at `crates/happenstance/src/testing/tests.rs:27-53`, and lift the
membership-guarded `decode` from `crates/happenstance/tests/projection_runner.rs:75-86` into
at least `lib.rs`'s first program, `testing/mod.rs`'s `assert_domain_event` doctest, and
both examples.

- **Costs a caller:** nothing. No signature moves, no call is invalidated, no existing test
  starts failing. Roughly line-neutral in the doctests — the named const adds a line and
  removes the wrapped array literal and the `.clone()`. `lib.rs:37-39` goes from three lines
  to three.
- **Costs an adapter author:** nothing. `DomainEvent` lives in the typed layer; no adapter
  implements it.
- **Semver class:** **none.** Every affected site is a doc comment or example source.
- **Forecloses:** nothing. It is strictly compatible with adding a guard later, and it
  *shrinks* the residual any later guard has to cover.
- **What it does not do:** it does not detect anything. A stranger who writes positional
  indexing anyway is not caught, and a two-variant mapping swap in a named-const impl is
  still invisible to every check in the tree.

### Option B — strengthen `assert_domain_event` in place with a positional-agreement precondition

Teach the published function to check `every_variant[i].event_type() == EVENT_TYPES[i]`.

- **Costs a caller:** a call that passes today can panic tomorrow. The published doc at
  `crates/happenstance/src/testing/mod.rs:336` says only *"Hand this every variant of your
  domain enum"* — **no ordering is stated**, so a caller who passed variants in
  enum-declaration order against an `EVENT_TYPES` written in a different order is correct
  today and broken after.
- **Costs an adapter author:** nothing.
- **Costs this repository: measured, and it is zero.** The guard has **two** real call sites
  and **one** doctest call. `crates/happenstance/src/testing/tests.rs:422` passes
  `&[deposit("a1"), withdrawal("a1")]` against `EVENT_TYPES = &[DEPOSITED, WITHDRAWN]` —
  positionally in agreement. `crates/happenstance/src/testing/mod.rs:376` passes
  `&[Seat::Taken, Seat::Freed]` against an impl that indexes `[0]` and `[1]` — in agreement by
  construction. `crates/happenstance/tests/mounted_at_the_crate_root.rs:23` is
  `let _: fn(&[Ping]) = assert_domain_event::<Ping>;` — a **fn-pointer coercion, not a call**,
  so it executes no check at all. The one negative test,
  `crates/happenstance/tests/dsl_failure_message.rs:539-548`, still passes: `Drifted` declares
  `&[DEPOSITED]` and returns `WITHDRAWN`, which a positional check rejects at index 0, and the
  test asserts only that the message contains `"Withdrawn"` and `"EVENT_TYPES"` — both of which
  a positional message names. **Nothing in this tree goes red.**
- **Semver class:** **behaviour break** on a function live in `happenstance
  0.2.0-alpha.1`. And it is a *reachable* break: a consumer requiring `"0.2.0-alpha.1"` is
  matched by `0.2.0`, so the panic arrives on a routine `cargo update`. The pre-release
  marker (`_release-log.md:29`, *"a **pre-release**"*) is the licence to do it, and **0.2.0
  is the last moment that licence exists.** Note where the break lands: in
  `happenstance::testing`, so it surfaces as a **red test in a consumer's suite**, not as a
  production failure — the cheapest class a behaviour break can be in.
- **Forecloses:** the order-free formulation. Once declaration order is a documented
  precondition it cannot be relaxed without a second break.
- **The defect the evidence turns up.** The check has no independent handle on "variant
  *i*" — Rust offers no reflection over enum variants, so the only referent is the caller's
  own slice order. A failure therefore cannot distinguish *"the impl permuted"* from *"the
  caller listed the variants in a different order"*. The diagnostic is ambiguous by
  construction, and the precondition is unstated in the published doc
  (`crates/happenstance/src/testing/mod.rs:336`), so taking B means writing it.

  *A claim was removed here.* This bullet previously ended by asserting that B "fires
  spuriously on the sixteen reorder-immune named-const impls whose call sites happen not to
  match `EVENT_TYPES` order." That was never measured. Those sixteen impls have **no call
  site**: the guard is called from the three places enumerated two bullets up and from nowhere
  else, and this brief's own §"What this does not settle" already said so. The sentence priced
  a counterfactual as a measurement, and it is struck rather than reworded.

### Option C — add a sibling assertion, leave the published function alone

A second entry point (`assert_domain_event_in_order`, or whatever it is called) carrying the
positional precondition in its own doc from day one.

- **Costs a caller:** nothing until they opt in. One more name in
  `happenstance::testing`'s item table.
- **Costs an adapter author:** nothing.
- **Semver class:** **additive**, and additive *forever* — a new `pub fn` in a module is
  minor-compatible in Rust indefinitely. The audit says this itself at `:3545`: *"A second
  entry point instead is additive and free later."*
- **Forecloses:** nothing, except that the library now has two guards where a reader must
  work out which they want — the cost ADR-0020's own rationale is sensitive to
  (`projection.rs:152-154`, quoted in ADR-0020 at `:103-107`: *"two constructors enforcing
  different rules is the defect that makes an invalid value reachable through the weaker
  one"*). That argument is about *constructors*, not assertions, so it does not transfer
  cleanly — but it is the house's own instinct and a record should answer it rather than
  ignore it.
- **The same ambiguous-diagnostic defect as Option B applies**, minus the surprise: the
  precondition is stated up front, so a caller who opts in has been told.

### Option D — an order-free coverage check instead of a positional one

Check that the multiset `{v.event_type() : v in every_variant}` is a **bijection** onto
`EVENT_TYPES` — no two variants collapsing onto one declared type, no declared type left
unreturned.

- **Costs a caller:** less than B. It has no ordering precondition, so a correct call in
  any order still passes. It does still break a call that deliberately passes a *subset* of
  variants — which the doc already forbids in words but the code permits today.
- **Semver class:** **behaviour break, but a narrower one** than B, and free only until
  0.2.0 for the same reason.
- **Forecloses:** little.
- **What kills it as an answer to Y-2.** It does **not** catch the audit's named hazard. In
  the alphabetise-`EVENT_TYPES` scenario every variant still returns a *distinct declared*
  type — `Taken → "SeatFreed"`, `Freed → "SeatTaken"` — so the bijection holds and the check
  passes. It is a genuine improvement on membership, and it is orthogonal to the reorder.
  Recording it as a rejected alternative is worth more than adopting it as the answer.

### Option E — a repo-local lint forbidding `EVENT_TYPES[` in rendered documentation

`xtask` already lints source text as a gate step — `xtask/src/lint_constitution.rs`,
`lint_narrative.rs`, `lint_pages.rs`, and `cargo xtask lint-constitution` is in the gate. A
step asserting `EVENT_TYPES[` appears in no `///`, `//!` or `examples/` line is a
twenty-line addition to an existing pattern.

- **Costs a caller / adapter author:** nothing. **Semver: none.**
- **Forecloses:** nothing.
- **Its honest limit:** it holds Option A from regressing; it does not help a stranger who
  never reads this repository. It is a *ratchet*, not a fix, and it is only worth anything
  once A has landed.

---

## Recommendation

**Take Option A now, take Option E with it, and take Option B before 0.2.0. Option C is
what B degrades into if that deadline passes, not a parallel choice.**

**This recommendation flipped during review.** The first version read *"Take Option A now,
and Option E with it. Do not take Option B. Leave Option C undecided."* What flipped it is
recorded at the foot; the short form is that B's rejection rested on a cost that, when
measured, is zero.

**Why A and E regardless — unchanged, and independent of the guard.** The corrected census
is what carries this. The audit
framed the hazard as pervasive and therefore as something only a run-time guard could
reach; measurement says the opposite. Seventeen of thirty bodies are already immune, and
every immune one is *in-crate*, while every exposed one is *published as a model*. That is a
documentation defect with a documentation-sized cure, and the cure is a lift of a pattern
that already compiles in this tree at `crates/happenstance/src/testing/tests.rs:27-53` and
`crates/happenstance/tests/projection_runner.rs:75-86`. A guard added instead of A would be
a run-time detector for a hazard the library's own prose says it exists to remove
(`domain.rs:70-75`) and which the library then teaches at seven rendered sites. Fixing the
teaching is strictly upstream of detecting the consequence.

**Why B, and why now.** The first version of this brief gave three reasons against B and
called the third decisive. The third was false — see the Option B entry above, where it is
struck. What remains does not carry a rejection. **The measured in-tree cost of B is zero
broken calls**, its break lands in a test module rather than in production, and its licence —
the pre-release marker — expires at 0.2.0 and never returns. That is the whole of the case:
the one option with a hard deadline turns out to be the one whose bill this repository has
already been shown not to pay.

**Why C is now the fallback rather than an open question.** C's appeal was always that it is
*additive forever*, and that remains true — which is precisely what makes it the thing to do
*after* the deadline rather than instead of meeting it. Taking C while B is still free buys
the permanent version of a cost the brief already flagged under Option C and then waved
through: the canonical name keeps the weak check, so every caller who reaches for the obvious
name gets membership-only forever and only a caller who knows a second name exists gets the
stronger one. That is the house's own named defect — *"two constructors enforcing different
rules is the defect that makes an invalid value reachable through the weaker one"*
(`projection.rs:152-154`, quoted in ADR-0020 at `:103-107`). The first version answered that
by observing the sentence is about constructors, not assertions, and let it go. It transfers
better than that: an assertion nobody calls is as inert as a constructor nobody uses.

**The strongest argument against this recommendation.** It is the surviving half of the case
the first version made against B, and it should be read as an argument, not a residue:

> (i) It is the only option on the table carrying a behaviour break, and it buys the least
> per unit of break. (ii) Its diagnostic is ambiguous by construction — with no reflection
> over variants, the check cannot name which of the impl and the call site is wrong.

Two things extend it. **"Zero broken calls" is measured over *this* tree only.** It cannot be
measured over consumers, because the published doc states no ordering precondition
(`testing/mod.rs:336`, *"Hand this every variant of your domain enum"*), so any consumer who
listed variants in enum-declaration order against a differently-ordered `EVENT_TYPES` is
correct today and red tomorrow, and nothing here bounds how many of them there are. The
counter-argument is that the population is an eighteen-day-old alpha and the failure is a test
failure with a readable message — but that is a judgement about size, not a measurement, and
it should be recorded as one. And **B forecloses the order-free formulation permanently**:
once declaration order is documented as a precondition, relaxing it is a second break, and
Option D's bijection — which is order-free and which this brief rejects only as an *answer to
Y-2* — is the thing foreclosed. A reader who weighs the ambiguous diagnostic heavily should
take C and accept the permanent weak default, on the ground that a check whose failure does
not tell you which side moved is not worth a break to make canonical.

**A second argument against, quoted from the source that most authorises this brief.** From
`crates/happenstance/src/testing/mod.rs:336-340`, the doc that justifies
`assert_domain_event` existing at all:

> It exists because that agreement is **not** compiler-enforceable: a variant may return a
> type absent from `EVENT_TYPES` and no `const` sees the match arms. The alternative that
> lost was pretending a hand-written impl can enforce it — it cannot, and **a test that runs
> is worth more than a claim that does not.**

Read against the *first* version of this brief, that was a direct rebuttal: an A-only answer
meets a guard gap with better prose and a better-taught idiom, which is precisely *"pretending
a hand-written impl can enforce it"*, the alternative ADR-0020 already considered and
rejected. A named-const style is a convention, and a convention is a claim; the positional
check, whatever its diagnostic problems, is a test that runs. **Under the revised
recommendation this passage is no longer an argument against — it is the argument for B**, and
it is left standing here because it is the sentence that made the flip hard to resist rather
than merely permissible. One qualification survives either way: ADR-0020's sentence was
written about the *membership* residual, which `assert_domain_event` does test, not about the
reorder, which it cannot.

**On B-3's half, separately: the example correction is not optional and has no decision in
it.** It should land with A regardless of what happens to the guard, because of the
durability asymmetry below. The narrower question B-3 raises — whether the `decode`
obligation becomes *assertable*, i.e. a second loop in the guard over a fabricated
undeclared `EventType` — carries the identical published-function semver question as Y-2, and
so it now rides with **B**, under the same expiring licence and in the same change.

---

## Cost of delay

**Split, and the split is the whole point. The audit's own framing at `:3545` conflates the
two halves; measurement separates them.**

**Half one — the taught shape and the examples. Free now; not free later, and not because
of semver.** The mechanism is B-3's durability asymmetry
(`review-pre-publication-2026-09-03.md:3592`): *"the shape becomes durable in consumers'
stores at 0.2.0, and events written under a payload-discriminated `decode` cannot be
retro-fitted with envelope dispatch without a migration."* This is verified: no
`#[serde(rename)]` exists anywhere in `crates/happenstance/` or `examples/`, so the derives
are externally tagged and the payload's own variant identifier is the live discriminator.
Every consumer who copies `lib.rs:44-45` between now and the fix writes events whose only
discriminator is a Rust identifier they are free to rename. **The cost of delay here is not
paid by this repository — it is paid by strangers, in their stores, and it is not
recoverable by a later library change.** Same for the positional `event_type()`: an
application that ships the copied shape and later alphabetises its declaration mislabels
durable events. Nothing about a 0.2.0 boundary makes that cheaper or dearer; what changes at
0.2.0 is how many consumers there are.

**Half two — strengthening the published guard in place (Option B).** Genuinely free only
until 0.2.0, and permanently expensive after. `happenstance 0.2.0-alpha.1` is live
(`_release-log.md:33`); `^0.2.0-alpha.1` resolves `0.2.0`, so the panic ships on a routine
update. The pre-release marker is the licence, and it expires at 0.2.0. **This is the only
item in the record with a hard deadline — and after revision it is the item recommended
*for*, precisely because it is the only one whose price rises.** (The first version of this
brief ended this sentence *"and it is the item recommended against"*, which made the deadline
an argument for doing nothing.)

**Half three — a sibling function (Option C).** **Equally cheap forever.** Additive minor
change, today or at 1.0. No deadline exists on it, and none should be manufactured for it —
which is now the reason it is the *fallback* rather than the answer: it is exactly the option
that is still available on 0.2.1, so spending the expiring licence on B costs nothing that C
does not still cover.

**Half four — the `xtask` ratchet (Option E).** Equally cheap forever, but worthless before
A lands and cheap immediately after, when it costs one gate step and protects a fix that
otherwise regresses the next time someone writes a doctest by copying the nearest one.

---

## What this does not settle

- **Envelope *dispatch*, as distinct from an envelope *membership guard*.** The remedy
  lifted from `tests/projection_runner.rs:75-86` restores the `UnknownEventType` drift signal
  but leaves serde's variant identifier as the live discriminator, so the variant-rename
  hazard at `:3590` survives Option A intact. Making the envelope actually *select* the
  variant needs per-variant payload types and is a materially larger design question about
  what `DomainEvent` is. Not opened here.
- **Whether `#[serde(rename = "…")]` on every variant should be house style in the
  examples.** It is the cheapest partial mitigation of the rename hazard and it is not
  currently anywhere in the tree. It is a different question from either half above.
- **Whether `assert_domain_event` should also exercise `tags()`, `encode` or `decode`.**
  Today it reads `event_type()` and nothing else. B-3's *"second loop over a fabricated
  undeclared `EventType`"* is one instance of a larger question about the guard's remit that
  no record has asked.
- **Whether either worked example should call the guard at all.** Verified: `grep -rn
  assert_domain_event --include=*.rs` finds call sites only at
  `crates/happenstance/src/testing/tests.rs:422`,
  `crates/happenstance/tests/dsl_failure_message.rs:541` and
  `crates/happenstance/tests/mounted_at_the_crate_root.rs:23` — **zero in `examples/`**. The
  audit is right that this is cheap and independent (`:3547`), and it is orthogonal to every
  option above.
- **Option D's order-free bijection check** as an independent improvement to the membership
  test, on its own merits and its own (smaller) break. Rejected here only as an *answer to
  Y-2*.
- **`happenstance-macros`.** ADR-0020 carried the falsifiable prediction that the derive
  lands in scope (`.kb/decisions/0020-fold-query-agreement.md:36-39`), and ADR-0033 records
  it out of scope for 0.1. A derive would make both hazards unrepresentable rather than
  merely untaught, which is the strongest long-run answer to Y-2 — and it is settled
  elsewhere and not reopened here.
- **Any clause in `spec/SPECIFICATION.md`.** There is none to move; `spec-trace` does not
  reach the typed layer's `DomainEvent`, and whether the typed layer should acquire clause
  coverage at all is its own question.

---

## Revision record

One revision, 2026-09-03, in response to a review that did not accept the original
recommendation. Three changes, and the second follows from the first.

**1. A falsified premise was removed, not reworded.** The original Option B entry ended:
*"and it fires spuriously on the sixteen reorder-immune named-const impls whose call sites
happen not to match `EVENT_TYPES` order"*, and §Recommendation restated it as reason (iii),
labelled **decisive**: *"It would fire spuriously against the sixteen named-const impls this
repository itself writes."* Re-measured, the cost is **zero**. Those sixteen impls have no
`assert_domain_event` call site. The guard is called from two places plus one doctest —
`crates/happenstance/src/testing/tests.rs:422`,
`crates/happenstance/tests/dsl_failure_message.rs:541` and
`crates/happenstance/src/testing/mod.rs:376` — and
`crates/happenstance/tests/mounted_at_the_crate_root.rs:23`, the fourth mention, is
`let _: fn(&[Ping]) = assert_domain_event::<Ping>;`, a fn-pointer coercion that calls nothing.
All three calls satisfy a positional check today, and the one negative test
(`dsl_failure_message.rs:539-548`) asserts only that the message names `"Withdrawn"` and
`"EVENT_TYPES"`, which a positional message does. The claim priced a counterfactual as a
measurement and was struck; the Option B entry now carries the measurement in its place, and
says the sentence was removed. The brief's own §"What this does not settle" had enumerated the
three call sites all along, which is what made the error findable.

**2. The recommendation flipped: A + E, B rejected, C open → A + E + B, C the fallback.**
With reason (iii) gone, reason (ii) — the ambiguous diagnostic — is no discriminator, because
the original text conceded it applies to Option C and left C live anyway; and reason (i) is the
deadline restated as an objection to meeting it. What is left is a break with a measured
in-tree cost of zero, landing in `happenstance::testing` as a red test rather than a
production failure, under a pre-release licence that expires at 0.2.0 and does not return —
against a sibling whose cost is permanent, because the canonical name would keep the weak
check and every default caller with it. The reviewer's formulation, which this revision
adopts: *"C's cost is permanent — the weak guard keeps the canonical name and every default
caller — where B's expires at 0.2.0."*

**3. Consequential edits, listed so none of them is silent.** The preamble gained a pointer to
this section. Option B gained a measured-cost bullet and a note on where the break lands.
§Recommendation's *"Why not B specifically"* was replaced by *"Why B, and why now"*; *"Why C
stays open"* by *"Why C is now the fallback"*, which reverses the original's judgement that
ADR-0020's weaker-constructor argument *"does not transfer cleanly"* to assertions — Option C's
own bullet still carries the original wording, and is overridden there rather than edited.
The surviving reasons (i) and (ii) are now quoted verbatim under §Recommendation as the
strongest argument against, extended with the two points that genuinely limit the flip: the
zero is measured over this tree only and cannot be measured over consumers, and B forecloses
Option D's order-free formulation permanently. The ADR-0020 *"a test that runs is worth more
than a claim that does not"* passage, which the original filed as the strongest argument
*against* its recommendation, is now the argument *for* it and is labelled as such rather than
deleted. Cost-of-delay half two's closing sentence changed from *"and it is the item
recommended against"*; half three now names C as the fallback. B-3's assertability question
rides with B rather than with C.

**What did not change.** Every measurement in §"What is true today" — the corrected census,
the eight `decode` sites, the governing-records analysis and the finding that
`spec/SPECIFICATION.md` says nothing here. Options A, D and E are untouched. Nothing in
§"Cost of delay" half one moved: the durability asymmetry was never contingent on the guard
question. No file outside this brief was edited, and no repository file was read for anything
but verification.
