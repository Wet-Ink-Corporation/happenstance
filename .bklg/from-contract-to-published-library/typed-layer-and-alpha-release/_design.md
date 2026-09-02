---
item: HS-P0011
stage: design
created: 2026-08-12T05:12:00.000Z
updated: 2026-08-12T05:12:00.000Z
---

# API surface design — The typed layer, the worked example, and 0.2.0-alpha.1

The resolved **public API surface** for HS-P0011, plus the composition of the three
text surfaces it renders into. Everything below is binding on the implementer.

**Read the medium before reading the decisions.** There is no screen. A repo-wide
search for a token file, a theme file or a shared UI package returns nothing, the
initiative declares `userFacing: false`, and `design.capture` is deliberately absent
from `.redkiln/config.yaml:75-82` so the perceptual review is a declared *skip* and
not a silent pass. What a human meets here is a **type surface** and **four text
surfaces**, and this project is the one the initiative's own decomposition calls
*"the primary `ux` brief in the portfolio"* because *"this is the API a `cargo add`
user meets"* (`.bklg/from-contract-to-published-library/_decomposition.md:274`).

**The dossier this stage normally cites does not exist.**
`.bklg/from-contract-to-published-library/_discovery/distillation/` holds
`opportunities.md` and `personas-and-journeys.md` only; no `interaction-patterns.md`
was ever commissioned
(`.bklg/from-contract-to-published-library/initiative.md:411-412,479-482` records
that as a deliberate intake-gate decision). Every "citation" column below therefore names the
distillation artefact that *does* exist, or the in-tree file the pattern is copied
from. Where neither exists, the row says so rather than inventing a precedent.

**The specification is silent about this layer.** `spec/SPECIFICATION.md`'s clause
namespaces are `VT-`, `ES-`, `PS-`, `SY-`, `WF-`, `CF-` — value types, event store,
projection store, sync, workflow, cross-cutting. There is no typed-layer namespace.
So most items below carry **no clause**, and their only contract is this file plus
ADR-0020 and ADR-0021. That is a finding, not an omission: it is why DoD 5 makes
this file human-approved before a single story spec is written.

---

## Surfaces

Machine-read manifest. `route` is the artefact a reviewer opens; `selector` is the
addressable region inside it. No capture harness runs (there is no app), so
`selector` is the anchor a *human* reviewer navigates to, and each is verified to
exist in the tree today or is explicitly marked as created by this project.

```yaml
- id: crate-root-rustdoc
  pattern: progressive-disclosure-landing
  route: crates/happenstance/src/lib.rs
  selector: "//! module doc, lines 11-71 (rendered: docs.rs/happenstance/*/happenstance/ #main-content > .docblock)"
  states: [default-features, no-default-features, all-features-docsrs, narrow-1024]

- id: crate-readme
  pattern: decision-first-readme
  route: crates/happenstance/README.md
  selector: "## Stability (new, replaces the blockquote at lines 6-11)"
  states: [rendered-on-crates-io, compiled-as-doctest, narrow-1024]

- id: first-program-doctest
  pattern: complete-runnable-first-program
  route: crates/happenstance/src/lib.rs
  selector: "the first ```rust fence in the module doc"
  states: [compiles-default, compiles-no-default-features, rendered-hidden-lines-elided]

- id: worked-example-transcript
  pattern: text-first-sectioned-transcript
  route: examples/course-subscriptions/src/main.rs
  selector: "stdout of `cargo run -p course-subscriptions`, the `== … ==` section markers"
  states: [accepted, rejected, final-log, piped-non-tty, narrow-80col]

- id: dsl-failure-message
  pattern: non-occluding-assertion-failure
  route: crates/happenstance/src/testing/
  selector: "the panic message of a failed `then(...)` assertion, on stderr"
  states: [expected-vs-actual, seeded-but-not-selected, empty-selection, long-label]

- id: compile-fail-diagnostic
  pattern: pinned-stderr-snapshot
  route: crates/happenstance/tests/ui/ (created by this project; blocked on trybuild)
  selector: "the .stderr fixture's first error block and its `-->` span"
  states: [protection-present-fails-to-compile, negative-control-compiles, blocked-no-instrument]
```

---

## Pattern decision

One row per surface: the pattern chosen, what was rejected, the evidence, and the
tension it settles. **DT-2 is this project's only owned tension** and it is resolved
in the first row and again, concretely, in *Shape decision*.

### `crate-root-rustdoc` — progressive-disclosure landing

**Chosen.** One-sentence summary → the complete first program → the discriminator
(what arrives here, what stays below) → the five former roadmap bullets, now
intra-doc links to real items → the feature table → the downward pointer for adapter
authors. The example moves **above** the prose.

**Rejected: keep today's order** (status → discriminator → roadmap → example last,
`crates/happenstance/src/lib.rs:11-71`). It was right for a facade with nothing to
show. After the alpha it puts 40 lines of positioning ahead of the artefact P4 came
for, and P4's beat *"is one-shot and time-boxed — they do not get a second pass"*
(`_decomposition.md`, UX brief persona table, citing
`_discovery/distillation/personas-and-journeys.md`, *Persona 4*).

**Rejected: a `#[doc(inline)]` prelude module as the landing.** A prelude is a second
place a name can come from, and the crate already promises *"everything the contract
crate exports is available here under the same paths"* (`lib.rs:53-56`). Two import
paths for `Query` is the ambiguity AC-A01 forbids.

**Known failure mode of the chosen pattern, and its mitigation.** A landing that
leads with code truncates: the reader takes the snippet and never reads the
discriminator, then files an issue asking why `happenstance-core` exists. Mitigated
by the last region — the adapter-author pointer stays *last and short*, exactly as
today (`lib.rs:69-72`), and the discriminator paragraph sits immediately under the
example rather than above it, so the first scroll lands on it.

**Resolves:** DT-2's rendered half — the first program is the artefact the ceremony
budget is judged against (AC-U01).

### `crate-readme` — decision-first readme

**Chosen.** `# happenstance` → the DCB one-liner (**untouched**) → `## Which crate do
I want?` → **`## Stability`** → `## What DCB buys you` (first code block) →
`## Guarantees` → `## Design` → `## Licence`.

**Rejected: mount `## Stability` between `## Guarantees` (:41) and `## Design`
(:51)**, which is what the architecture brief's *Composition roots* item 7 and
DEP-003 both say. It is below the first code block, and AC-U18 requires the
stability posture to be legible *"before the reader commits… above the first code
block a reader will copy."* **Two briefs disagree and this file settles it: AC-U18
wins on ordering.** The brief's line is a mount-point convenience; the ordering is a
composition decision, which is this stage's to make and no brief's.

**Rejected: keep the status blockquote (:6-11) and add `## Stability` beneath it.**
The blockquote asserts *"this crate is currently a facade… adds nothing yet"*, which
is **false the moment this project ships**. Two stability claims on one page, one of
them wrong, is worse than either. The blockquote is deleted and `## Stability` takes
its position.

**Boundary respected:** lines 1-4 (the title and the DCB positioning sentence) are
**not touched**. DT-1 — which claim leads on first contact — is HS-P0016's, and the
`## Stability` section is this project's only claim on that page (`_decomposition.md`,
UX brief, *Deliberately not decided here*).

### `first-program-doctest` — complete runnable first program

**Chosen.** Every line that names a `happenstance` item is **visible**. Hidden `# `
lines are permitted for exactly two things and nothing else: the `#[tokio::main]`
wrapper (RS-62-2 requires an `.await` example to get a runtime) and the closing
`# Ok::<(), Box<dyn core::error::Error>>(())`.

**Rejected: elide the `DomainEvent` impl behind `# `.** It halves the apparent
ceremony and falsifies the measurement AC-U01 exists to take — and AC-013's verdict
(*"if the rewritten example carries more mapping boilerplate than domain logic, the
derive is in scope"*, `RUNBOOK.md:524`) is read off this same text. Hiding the
mapping would answer AC-013 by concealment.

**Failure mode and mitigation.** A complete program is long, and length at the top of
a landing page is its own failure. Mitigated by the density budget below: the program
is capped at **35 visible lines and 72 columns**, and if it exceeds either, the thing
that yields is the *domain* (fewer variants, one course), never the ceremony — because
the ceremony is what is being measured.

**Resolves:** DT-2, on the record, in the medium AC-014 names.

### `worked-example-transcript` — text-first sectioned transcript

**Chosen.** Preserved verbatim from `examples/course-subscriptions/src/main.rs:37-79`:
blank line, `== what is about to happen ==`, then indented result lines, with refusals
carrying the literal `rejected: ` prefix. No colour, no spinner, no in-place rewrite.

**Rejected: add ANSI colour to distinguish accepted from rejected.** There is no
colour dependency anywhere in the workspace (`termcolor`/`owo-colors`/`anstream`/
`NO_COLOR` all return nothing), and the strongest available satisfaction of *colour is
never alone* is no colour at all (AC-U16).

**Rejected: a progress indicator for the projection runner's polling.** ES-32 makes
polling the mechanism; AC-010 makes its cost **a recorded number in `experiments/`**,
not a performance. An animation would also break the piped-stdout state the gate runs
in.

**Failure mode and mitigation.** A pure-text transcript with no colour relies entirely
on structure, so structure that overflows the terminal width destroys it. Mitigated by
the 80-column budget and the yield order in *Density budget*.

### `dsl-failure-message` — non-occluding assertion failure

**Chosen.** A failed `then(...)` renders four labelled regions in this order:
`expected:`, `actual:`, `selected by the model's query:`, `seeded but NOT selected:`.
The fourth region is the point.

**Rejected: `assert_eq!`-style expected/actual only.** It hides the filter. When a
model's `EVENT_TYPES` and its fold disagree, the events that *were* seeded and *were
not* selected are the entire diagnosis, and a message that omits them re-opens exactly
the hazard ADR-0020 closes (AC-U11).

**Failure mode and mitigation.** Four regions is a lot of output, and a seeded log of
200 events would bury the assertion. Mitigated: each region prints at most **8 events**
then `… and N more`, and the truncation is applied to `selected` first and to
`seeded but NOT selected` last, because the last region is the one carrying the
information.

**Citation:** no dossier row exists. The in-tree precedent is
`crates/happenstance-testkit/src/contract.rs:500-507`'s `skip_line` — a declined
capability prints its reason rather than vanishing — and this is the same principle
applied to a filter rather than to a capability.

### `compile-fail-diagnostic` — pinned stderr snapshot

**Chosen.** A `trybuild` `.stderr` fixture whose first error block's `-->` span points
into `examples/course-subscriptions/`, at the user's own `match` arm.

**Rejected: a `compile_fail` doctest.** Not an equivalent instrument and this is
settled, not re-argued: rustdoc collects doctests from the lib target only, and on
1.97.1 it *silently ignores* an unmatched error-code annotation, so the **negative
control cannot discriminate** — and the negative control is the whole of AC-002
(`RUNBOOK.md:3614-3627`; `spec/SPECIFICATION.md:8772`; the same warning is written into
`crates/happenstance-core/src/event.rs:95-106`'s own doc comment).

**Blocked, and named as blocked.** `trybuild` is absent from `Cargo.toml` and
`Cargo.lock` (verified), and its adoption is HS-P0010's decision. This surface is
**specified but unbuildable** until that input arrives. If HS-P0010 declines it,
escalate; do not substitute.

---

## Items

Every public item this project adds, changes or removes. `path` is the full path a
caller writes. `clause` is `—` wherever the specification has no clause for it, which
is most of them (see the header).

```yaml
- path: happenstance::DomainEvent
  kind: trait
  change: added
  feature: default
  clause: "VT-3 (payload and metadata stay opaque to every store)"

- path: happenstance::DecisionModel
  kind: trait
  change: added
  feature: default
  clause: "—"

- path: happenstance::Boundary
  kind: trait          # sealed; blanket impl for DecisionModel + tuple impls 2..=8
  change: added
  feature: default
  clause: "—"

- path: happenstance::Boundary::query
  kind: method
  change: added
  feature: default
  clause: "—"

- path: happenstance::Codec
  kind: trait
  change: added
  feature: default
  clause: "VT-3"

- path: happenstance::Json
  kind: type
  change: added
  feature: json
  clause: "—"

- path: happenstance::Postcard
  kind: type
  change: added
  feature: postcard
  clause: "—"

- path: happenstance::Cbor
  kind: type
  change: added
  feature: cbor          # conditional on a licence verdict — see Visibility and stability
  clause: "—"

- path: happenstance::CodecError
  kind: type
  change: added
  feature: default
  clause: "—"

- path: happenstance::commit
  kind: fn
  change: added
  feature: json          # the Json convenience; commit_with is ungated
  clause: "ES-10 (the visibility invariant the after-anchor rests on)"

- path: happenstance::commit_with
  kind: fn
  change: added
  feature: default
  clause: "—"

- path: happenstance::Retry
  kind: type
  change: added
  feature: default
  clause: "—"

- path: happenstance::Committed
  kind: type
  change: added
  feature: default
  clause: "—"

- path: happenstance::CommandError
  kind: type
  change: added
  feature: default
  clause: "—"

- path: happenstance::Projection
  kind: trait
  change: added
  feature: unstable-projection
  clause: "PS-33 (the falsifier this trait's existence evaluates)"

- path: happenstance::run_projection
  kind: fn
  change: added
  feature: unstable-projection
  clause: "PS-27, PS-30"

- path: happenstance::testing::given
  kind: fn
  change: added
  feature: memory
  clause: "—"

- path: happenstance::testing::Decision
  kind: type
  change: added
  feature: memory
  clause: "—"

- path: happenstance::testing::assert_domain_event
  kind: fn
  change: added
  feature: memory
  clause: "—"

- path: happenstance_testkit::FaultyStore
  kind: type
  change: added
  feature: default
  clause: "—"

- path: happenstance_testkit::SendFaultyStore
  kind: type
  change: added
  feature: default
  clause: "—"

- path: happenstance_testkit::GappyMemoryStore
  kind: type
  change: added
  feature: memory
  clause: "—"

- path: "happenstance [features] json / cbor / postcard / unstable-projection"
  kind: feature
  change: added
  feature: default        # json only; the rest off by default
  clause: "PS-3 (the projection port's gating)"

- path: happenstance (crate module doc)
  kind: const             # not code: the render surface itself
  change: signature-changed
  feature: default
  clause: "—"
```

---

## Signatures

The exact signatures, as they will be written.

```rust
// --- the domain vocabulary ------------------------------------------------

pub trait DomainEvent: Sized {
    /// Every event type a value of this type can carry.
    const EVENT_TYPES: &'static [EventType];

    fn event_type(&self) -> EventType;
    fn tags(&self) -> Tags;

    /// # Errors
    /// Returns the codec's error if the payload cannot be encoded.
    fn encode<C: Codec>(&self, codec: &C) -> Result<bytes::Bytes, CodecError>;

    /// # Errors
    /// Returns the codec's error if `data` is not a valid payload for
    /// `event_type`, or `CodecError::UnknownEventType` if it is not ours.
    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &bytes::Bytes,
    ) -> Result<Self, CodecError>;
}

pub trait DecisionModel: Clone {
    type Event: DomainEvent;

    /// The tags every event inside this boundary carries. Already validated:
    /// `Tags` has no infallible constructor that can produce an invalid value.
    fn scope(&self) -> &Tags;

    fn apply(&mut self, event: Self::Event);
}

/// What the command loop consumes. Sealed: blanket-implemented for every
/// `DecisionModel` and macro-implemented for tuples of arity 2..=8.
pub trait Boundary: sealed::Sealed {
    /// The derived query. Never hand-maintained — there is nowhere to put one.
    ///
    /// # Errors
    /// `InvalidQuery::UnconstrainedItem` when the boundary constrains neither
    /// event types nor tags, which is `Query::all()` and must be said out loud.
    fn query(&self) -> Result<Query, InvalidQuery>;

    /// # Errors
    /// Returns `CodecError` if a nominated event cannot be decoded.
    fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C)
        -> Result<(), CodecError>;
}

// --- encoding -------------------------------------------------------------

pub trait Codec {
    /// The value written into an event so a reader knows how to decode it.
    /// ADR-0021 owns *where* it is written; this constant is invariant under
    /// that choice.
    const TAG: &'static str;

    /// # Errors
    /// Returns `CodecError` if the value cannot be serialised.
    fn encode<T: serde::Serialize>(&self, value: &T) -> Result<bytes::Bytes, CodecError>;

    /// # Errors
    /// Returns `CodecError` if the bytes are not a valid `T`.
    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError>;
}

#[cfg(feature = "json")]     pub struct Json;
#[cfg(feature = "cbor")]     pub struct Cbor;
#[cfg(feature = "postcard")] pub struct Postcard;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("no codec is registered for tag `{tag}`")]
    UnknownTag { tag: alloc::boxed::Box<str> },
    #[error("`{event_type}` is not an event of this domain type")]
    UnknownEventType { event_type: EventType },
    #[error(transparent)]
    Encode(#[source] alloc::boxed::Box<dyn core::error::Error + Send + Sync>),
    #[error(transparent)]
    Decode(#[source] alloc::boxed::Box<dyn core::error::Error + Send + Sync>),
}

// --- the command loop -----------------------------------------------------

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Retry { /* private */ }

impl Retry {
    #[must_use] pub const fn attempts(n: core::num::NonZeroU32) -> Self;
    #[must_use] pub const fn once() -> Self;   // no retry; one attempt
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Committed {
    pub position: SequencePosition,
    pub attempts: u32,
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CommandError<E: core::error::Error + 'static, D: core::error::Error + 'static> {
    #[error(transparent)]
    Boundary(#[from] InvalidQuery),
    #[error("reading the decision model failed")]
    Read(#[source] E),
    #[error(transparent)]
    Append(#[source] AppendError<E>),
    #[error("decoding the event at position {position} failed")]
    Decode { position: SequencePosition, #[source] source: CodecError },
    #[error("encoding a `{event_type}` payload failed")]
    Encode { event_type: EventType, #[source] source: CodecError },
    #[error("the decision refused")]
    Refused(#[source] D),
    #[error("the boundary was contended on all {attempts} attempts")]
    Exhausted { attempts: u32, #[source] source: ConditionViolated },
}

/// Reads, decides, appends under a condition, and retries on
/// `ConditionViolated` up to `retry`. JSON payloads.
///
/// # Errors
/// See [`CommandError`]. Every variant is actionable at this call site.
#[cfg(feature = "json")]
pub async fn commit<S, B, D, F>(
    store: &S,
    boundary: B,
    retry: Retry,
    decide: F,
) -> Result<Committed, CommandError<S::Error, D>>
where
    S: EventStore,
    B: Boundary,
    D: core::error::Error + 'static,
    F: FnMut(&B) -> Result<alloc::vec::Vec<B::Event>, D>;

/// As [`commit`], with a caller-supplied codec.
pub async fn commit_with<S, B, C, D, F>(
    store: &S,
    boundary: B,
    codec: &C,
    retry: Retry,
    decide: F,
) -> Result<Committed, CommandError<S::Error, D>>
where /* as above, plus */ C: Codec;

// --- the projection runner (off by default) -------------------------------

#[cfg(feature = "unstable-projection")]
pub trait Projection {
    type Event: DomainEvent;
    type Store: ProjectionStore;

    fn id(&self) -> &ProjectionId;
    fn scope(&self) -> &Tags;

    /// Applies one decoded event into the adapter's open batch.
    ///
    /// # Errors
    /// Returns the adapter's error; the runner rolls the batch back.
    fn apply(
        &mut self,
        event: Self::Event,
        batch: &mut <Self::Store as ProjectionStore>::Batch<'_>,
    ) -> Result<(), <Self::Store as ProjectionStore>::Error>;
}

/// Streams events matching the projection's derived query and commits the
/// read-model write and the checkpoint as one unit, in chunks of `chunk`.
///
/// # Errors
/// See [`ProjectionError`].
#[cfg(feature = "unstable-projection")]
pub async fn run_projection<S, P, C>(
    events: &S,
    models: &P::Store,
    projection: &mut P,
    codec: &C,
    chunk: core::num::NonZeroUsize,
) -> Result<Progressed, ProjectionError<S::Error, <P::Store as ProjectionStore>::Error>>
where S: EventStore, P: Projection, C: Codec;

// --- the testing surface --------------------------------------------------

#[cfg(feature = "memory")]
pub fn given<B: Boundary>(boundary: B) -> Given<B>;

#[cfg(feature = "memory")]
impl<B: Boundary> Given<B> {
    /// # Errors
    /// Returns `CodecError` if a seeded event cannot be encoded.
    pub fn event<E: DomainEvent>(self, event: E) -> Result<Self, CodecError>;

    /// # Errors
    /// Returns the store's error, or the decision's own.
    pub async fn when<D, F>(self, decide: F) -> Result<Decision<B::Event>, CommandError<MemoryStoreError, D>>
    where D: core::error::Error + 'static, F: FnOnce(&B) -> Result<alloc::vec::Vec<B::Event>, D>;
}

#[must_use = "a Decision is not appended until it is committed; dropping it \
              discards the events the decision produced"]
#[non_exhaustive]
pub struct Decision<E> { /* private */ }

impl<E: DomainEvent + PartialEq + core::fmt::Debug> Decision<E> {
    /// Panics with the four-region message specified under
    /// `dsl-failure-message` if the emitted events differ.
    pub fn then(self, expected: &[E]);
    pub fn then_refused(self);
}

/// Asserts every value's `event_type()` is in `EVENT_TYPES`. The check a derive
/// would make unnecessary — see AC-013.
#[cfg(feature = "memory")]
pub fn assert_domain_event<E: DomainEvent>(every_variant: &[E]);

// --- happenstance-testkit -------------------------------------------------

pub struct FaultyStore<S: EventStore> { /* private */ }
pub struct SendFaultyStore<S: SendEventStore> { /* private */ }

impl<S: EventStore> FaultyStore<S> {
    #[must_use] pub fn new(inner: S) -> Self;
    /// Fails the next `n` appends with `ConditionViolated`, reporting
    /// `conflicting_position: None` — which a remote store legitimately does.
    #[must_use] pub fn violate_next(self, n: u32) -> Self;
    #[must_use] pub fn fail_next_read(self, n: u32) -> Self;
}

#[cfg(feature = "memory")]
pub struct GappyMemoryStore { /* private */ }

#[cfg(feature = "memory")]
impl GappyMemoryStore {
    /// Assigns positions with a stride, so `position + 1` is never the next one.
    #[must_use] pub fn with_stride(stride: core::num::NonZeroU64) -> Self;
}
```

---

## Shape decision

| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |
| ---- | ------------ | ------------------ | -------- | -------- |
| `DecisionModel::query` | **Not on the trait.** The derivation is `Boundary::query`, on a *sealed* trait blanket-implemented for every `DecisionModel` | A provided method on `DecisionModel` — a caller can override it, and an overridden derivation is a hand-maintained query, which is exactly DR-02's prohibition. A free `derive_query::<M>()` — the caller can ignore it and build their own | RS-40-2 (*put a derivable convenience on the blanket ext trait*), `standards/rust/40-public-surface-and-evolution.md:73` | DR-02 structurally: there is nowhere to put a divergent query |
| `Boundary::query` return type | **`Result<Query, InvalidQuery>`**, and an *empty* `EVENT_TYPES` is a **compile error**, not a runtime one | Infallible `Query` — impossible without an `unwrap`, because `QueryItem::new` is fallible and `happenstance-core` is frozen (see *the residual*, below). Fallible *and* no const assertion — that is DT-2's (a) pole and pushes a compile-time-decidable mistake to the first read | `crates/happenstance-core/src/query.rs:56-76`; `InvalidQuery::UnconstrainedItem` at `crates/happenstance-core/src/error.rs:99-102`; RS-61-4 (*put the claim in a `const` item*), `standards/rust/61-compile-time-assertions.md:249` | **DT-2** |
| `DecisionModel::scope` | **`fn scope(&self) -> &Tags`** — the model *holds* validated tags | `fn scope(&self) -> Tags` built on each call: a fallible construction inside an infallible signature forces an `unwrap`. `&[(&str, &str)]`: revalidates on every read and moves the error to the read | `crates/happenstance-core/src/tag.rs:304` (`Tags::from_pairs` is the only way in, and it is fallible) | **DT-2** — the validation is paid once, in the constructor the caller already writes |
| `DecisionModel` supertrait | **`Clone`**, not `Default` | `Default` would force `scope`'s validated `Tags` into a default-constructible field, which re-opens the "how do I get an invalid Tags" hole. `Clone` lets the loop re-fold from the *pristine* model on retry, which is AC-U13 | `crates/happenstance-core/src/projection.rs:47-61` — *"two constructors enforcing different rules is the defect that makes an invalid value reachable through the weaker one"* | AC-U13 |
| Composition | **`impl Boundary for (B1, B2)`… by an internal `macro_rules!`, arity 2..=8** | An exported `compose!` macro: the caller writes a macro invocation where a tuple would do, and a macro in the caller's face is ceremony DT-2 is being measured on. Arity ceiling 8 because rustdoc renders one impl block per arity and 16 of them buries the page | RS-41-1/41-3 apply to the internal macro (`$crate`-qualified, most-literal-first) | AC-004; **DT-2** (zero caller-visible syntax) |
| `DomainEvent::event_type` | **`fn event_type(&self) -> EventType`** (by value; a `Cow::Borrowed` copy, no allocation) | `-> &'static EventType`: `EventType` carries a `Cow<'static, str>`, so const promotion does not apply and the impl would have to index `EVENT_TYPES` **by position** — two places to get wrong, which is the hazard AC-001 exists to close | `crates/happenstance-core/src/event.rs:38-58` (`Cow` payload), `:108` (`from_static` is `const`) | AC-001 |
| `EVENT_TYPES` ↔ `event_type()` agreement | **Not compiler-enforced. Tested**, by `assert_domain_event::<E>(&[…every variant…])` | Pretending a hand-written impl can enforce it. It cannot: a variant may return a type absent from `EVENT_TYPES`, and no `const` sees the match arms | `RUNBOOK.md:524` — this residual **is** AC-013's measurement | AC-013 gets a number instead of an opinion |
| Codec selection | **`commit` (Json) + `commit_with` (any `C: Codec`)** | One entry point taking `&C` always: every first program then names a codec before it names a domain. A `Codec` *enum* instead of a trait: shorter, but forbids a user-supplied encoding at the alpha for no gain | RS-40-1 (*grow a port with a new trait, never a new required method*) — the same instinct applied to entry points | **DT-2** |
| `Codec::Error` | **Concrete `CodecError` with a boxed `#[source]`**, not an associated type | An associated `Error` type adds a third type parameter to `CommandError`, `Boundary::absorb`, `Decision` and the runner. `String`: forbidden outright | RS-30-2 (`standards/rust/30-error-taxonomy.md:71`) — a box is still a typed `#[source]`; a `String` is not | AC-U05 |
| Retry bound | **`Retry` is a required argument**, `NonZeroU32` inside | A default of 3 hidden in the loop: *"a loop whose only exit is success is a hang with better manners"* and AC-U13 requires the bound to be **visible to the caller**. `u32` instead of `NonZeroU32`: `attempts(0)` has no honest meaning | AC-U13; `crates/happenstance-core/src/event.rs:247` is the workspace's `NonZero`-shaped precedent | AC-U13 |
| What the loop returns | **`Committed { position, attempts }`, `#[non_exhaustive]`** | `(SequencePosition, u32)`: a tuple freezes the arity, and `attempts` is exactly the field a later observability pass wants to grow | RS-13-4 / RS-40-3 (`standards/rust/13-sealing-and-exhaustiveness.md:144`) | AC-U06 |
| The `after` anchor | **From `read_decision_model`'s second return value**, never from `append`'s | Threading `append`'s return into the next condition. The port's own doc forbids it in terms | `crates/happenstance-core/src/store.rs:131-145` | The defect the loop exists to make unwritable |
| Projection runner gating | **`unstable-projection`, off by default**, forwarded to `happenstance-core/unstable-projection` **if** HS-P0010 lands it, gating only this crate's items if it does not | Shipping it on by default: the port beneath it is *"not yet frozen"* in its own module doc, and `CHANGELOG.md:19-22` **already asserts** the feature exists while no manifest has it. This project must not publish that discrepancy | `crates/happenstance-core/src/projection.rs:1-11`; `CHANGELOG.md:19-22`; RS-51-1 (*a feature adds*) | PS-3's *gating* (the **verdict** stays HS-P0016's) |
| `wasm32` claim | **A fifth `wasm32` step is added by name** to `xtask/src/main.rs::REQUIRED` (`:105`) and to `wasm_steps()` (`:784`), compiling `happenstance` with `--no-default-features --features std,json` | Recording "no claim". The typed layer is what a Workers application `cargo add`s; a claim nothing compiles is the silence AC-A06 exists to forbid. The step is nearly free — `happenstance` adds no dependency `happenstance-core` does not already build on that target, plus `serde`/`serde_json`, both of which do | AC-A06; `xtask/src/main.rs:203,231,252,271` are the four that exist | AC-A06, AC-015 |
| Testkit version | **`happenstance-testkit` publishes `0.2.0-alpha.1`** — its in-tree `0.2.0` moves down | Publishing a stable `0.2.0`. CF-32 gives the testkit an independent *number*, not an independent *maturity*, and a stable conformance suite over a port that is still moving is precisely the promise this project refuses to make (`project.md` risk row 1) | `crates/happenstance-testkit/Cargo.toml:4-13`; DEP-001's flag, which assigns the decision here | The deployment brief's open item |
| `FaultyStore` | **Two types**, `FaultyStore<S>` and `SendFaultyStore<S>` | One type with both impls: `trait_variant`'s blanket impl makes that `error[E0119]` | RS-20-4 (`standards/rust/20-two-flavour-ports.md:203-253`) | — |

**The residual, and it is a recorded defect not a shrug.** `Boundary::query` builds a
`QueryItem` from values that are *already* validated — `EVENT_TYPES` is
`const`-constructed and `Tags` has no infallible constructor — yet `QueryItem::new`
still returns `Result`, because `happenstance-core` offers no infallible path for
pre-validated inputs (`crates/happenstance-core/src/query.rs:56`). We do **not** edit
the frozen crate (AC-A02) and we do **not** `unwrap`. Instead:

1. `Err` is kept and given a real meaning — `UnconstrainedItem` is *"this boundary
   constrains nothing, which is `Query::all()` and must be said out loud"*, which is a
   refusal worth making;
2. the *other* way to reach it, an empty `EVENT_TYPES`, is turned into a **compile
   error** by a `const` item evaluated per-monomorphisation (RS-61-4);
3. `commit`/`commit_with` absorb the `Result` into `CommandError::Boundary`, so the
   first program writes **no extra `?`** for it.

**Defect candidate D-1**, for AC-012's log: *`happenstance-core` has no infallible
`QueryItem` constructor for pre-validated inputs; every derived query therefore
carries a `Result` that is unreachable for well-formed models.* Clause: none (the
constructors are `VT-18`'s subject — *"constructors accept values the caller already
holds, and their errors compose"* — which is the clause this partially contradicts).
Routed to a decision record, never a line edit.

**Open, and deliberately not settled here: ADR-0021's codec-tag home.** The two
admissible homes are `Event::metadata` (`crates/happenstance-core/src/event.rs:379`)
and `Tags`. The public surface above is **invariant under that choice** — `Codec::TAG`
is a `&'static str` either way and no signature changes — so the design does not wait
on it, and the M1 story decides it. The one architectural constraint that binds
regardless: **no adapter may need to understand the tag.**

---

## Placement and re-export

Everything lands in `crates/happenstance/src/`, in modules of the implementer's
choosing, and every item is `pub use`d **at the crate root**. Callers write
`happenstance::DomainEvent`, never `happenstance::domain::DomainEvent`. Module paths
are private structure; the root is the surface.

Two exceptions, both deliberate:

- **`happenstance::testing`** is a real, named module rather than root re-exports.
  `given`, `Decision` and `assert_domain_event` are test-time vocabulary, and mixing
  them into the root's item table doubles the page a P1 reader scans for the four
  names they actually need. Gated `#[cfg(feature = "memory")]`, which is on by default.
- **`FaultyStore` / `SendFaultyStore` / `GappyMemoryStore` live in
  `happenstance-testkit`**, at *its* crate root. They are instruments for other
  people's tests; putting them in `happenstance` couples an application's dependency
  graph to a test double, and the testkit is the crate with an independent version for
  exactly this class of change (CF-32, asserted at `xtask/src/main.rs:425`).

**`pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs:75`) stays, and
every new item is added beside it.** No new item may shadow a contract name — no
`happenstance::Query`, no `happenstance::EventStore` of our own. The crate's module
doc makes this a promise to readers (`:53-56`), and a shadowing name is a silent
breaking change to a published facade plus an ambiguity in every existing doctest
(AC-A01).

**What coherence forbids, stated before the compiler says it.** `trait_variant` emits
a blanket `impl<T: SendPort> Port for T`, so **one type cannot carry both flavours** —
that is why `FaultyStore` is two types and why every generic bound in this crate is
`EventStore`, the weaker requirement that accepts both (RS-20-2, RS-20-4). Import one
flavour name per module and reach the other by full path (RS-20-3). And there is no
`dyn Boundary` or `dyn DomainEvent`: both carry generic methods, so neither is
object-safe. That is a cost, accepted below, and it mirrors RS-20-5's *"there is no
`dyn EventStore`"*.

**Manifest changes.** `crates/happenstance/Cargo.toml` gains
`[package.metadata.docs.rs] all-features = true` and
`rustdoc-args = ["--cfg", "docsrs"]`, and `lib.rs` gains
`#![cfg_attr(docsrs, feature(doc_cfg))]`. **Neither exists today** — verified: the
block is present in `happenstance-core/Cargo.toml:54-56` and
`happenstance-testkit/Cargo.toml:56-58` and absent from `happenstance`'s. Without it
every feature-gated item this project adds renders on docs.rs with **no gate badge at
all**, which is AC-U14's failure and RS-51-5/RS-70-4's.

---

## Visibility and stability

| Item | Visibility | `#[non_exhaustive]` / sealed | Feature | Semver promise |
| ---- | ---------- | ---------------------------- | ------- | -------------- |
| `DomainEvent` | `pub` | not sealed — applications implement it | default | none until 0.2.0 stable; adding a required method before then is expected |
| `DecisionModel` | `pub` | not sealed — applications implement it | default | as above |
| `Boundary` | `pub` | **sealed** (private supertrait) | default | sealed *because* the tuple impls and the blanket impl must stay the only ones; a third implementor could hand-maintain a query |
| `Boundary::query` / `absorb` | `pub` | — | default | — |
| `Codec` | `pub` | not sealed — a user codec is legitimate | default | none |
| `Json` / `Postcard` | `pub` unit struct | — | `json` (default) / `postcard` | — |
| `Cbor` | `pub` unit struct | — | `cbor` | **conditional**: ships only if `cargo deny check licenses` passes for the chosen crate. `ciborium` is **not** in `[workspace.dependencies]` today; if it is refused, the alpha ships `json` + `postcard` and `cbor` is deferred with the reason recorded |
| `CodecError` | `pub` | `#[non_exhaustive]` | default | a fourth variant is not breaking |
| `commit` | `pub` | — | `json` | — |
| `commit_with` | `pub` | — | default | — |
| `Retry` | `pub` | `#[non_exhaustive]`, private fields | default | a policy field can be added |
| `Committed` | `pub`, public fields | `#[non_exhaustive]` | default | readable, not fabricable (RS-13-3) |
| `CommandError` | `pub` | `#[non_exhaustive]` | default | — |
| `Projection` / `run_projection` / `Progressed` / `ProjectionError` | `pub` | `#[non_exhaustive]` on the two types | **`unstable-projection`, off by default** | **explicitly none.** The port beneath is provisional; the feature name is the promise |
| `testing::given` / `Given` / `Decision` | `pub` | `Decision` is `#[non_exhaustive]` + `#[must_use]` | `memory` (default) | none at the alpha |
| `testing::assert_domain_event` | `pub` | — | `memory` | none |
| `FaultyStore` / `SendFaultyStore` | `pub` (testkit) | private fields, builder methods | default | testkit's own CF-32 number |
| `GappyMemoryStore` | `pub` (testkit) | private fields | `memory` | as above |
| *nothing* | `#[doc(hidden)]` | — | — | **No item in this project is `doc(hidden)`.** An item worth shipping is worth documenting; an item not worth documenting is `pub(crate)` |

**The `pub`-because-nobody-decided check.** Three items were considered and are
**not** public: the internal composition `macro_rules!` (an implementation detail of
the tuple impls — a caller writes a tuple); `sealed::Sealed`; and the per-attempt
retry state inside `commit` (a `Committed.attempts` count is the observable part, and
that is enough).

---

## Composition

The arrangement — regions in order, and where each sits relative to the others.

### `crate-root-rustdoc` (top to bottom)

| # | Region | Occupies | Shares space with |
| --- | --- | --- | --- |
| 1 | Summary | one line — `//! DCB-compliant event sourcing, with batteries.` (unchanged, `lib.rs:11`) | nothing |
| 2 | **The first program** | one ```` ```rust ```` fence, ≤ 35 visible lines | nothing — it is alone above the fold |
| 3 | `# What arrives here, and what stays below` | the discriminator paragraph, verbatim from `lib.rs:27-33` | immediately under the fence, so the first scroll lands on it |
| 4 | `# The vocabulary` | the five former roadmap bullets (`lib.rs:35-52`), **same order, same discriminator prose**, each bullet's bold term now an intra-doc link to the real item | one bullet per line |
| 5 | `# Features` | a 4-row table: `json` (default), `cbor`, `postcard`, `unstable-projection` — what each turns on, in one clause | below the vocabulary; a reader who does not care never reaches it |
| 6 | `# Testing without a database` | three lines pointing at `happenstance::testing` and at `happenstance-testkit` | — |
| 7 | Adapter-author pointer | two lines, verbatim from `lib.rs:69-72` | **last**, and it stays last |

Region 1 of the old page (`# Status: a facade over happenstance_core`, `lib.rs:13-25`)
is **deleted**. Its ADR-0006 paragraph moves into region 3, where the discriminator
already lives; keeping a "Status" heading that no longer says "facade" would be a
heading whose only content is that it used to matter.

### `crate-readme` (top to bottom)

`# happenstance` → the DCB sentence (**untouched**, lines 3-4) → `## Which crate do I
want?` → **`## Stability`** → `## What DCB buys you` (its code block **replaced** by
the first program, so the README and the crate root show the same text) →
`## Guarantees` → `## Design` → `## Licence`. The blockquote at 6-11 is deleted.

`## Stability` carries three things and nothing else: the phase at which the API stops
moving, in the reader's terms; a pointer to `CHANGELOG.md`; and the yank policy —
*"only one pre-release resolves at a time; each alpha is yanked when the next lands."*

### `worked-example-transcript` (top to bottom, unchanged shape)

Seven `== … ==` sections in the order at `main.rs:37-68`, each preceded by a blank
line (except the first) and followed by indented result lines; then `== final log ==`
and one row per event. **`main`'s observable steps survive verbatim** — they *are*
the behaviour AC-003 asserts. What changes is beneath them: `parse_capacity`
(`:231`), the `format!("…").into_bytes()` payload (`:96-99`) and `commit` (`:207`) are
deleted rather than wrapped.

### `dsl-failure-message` (top to bottom)

`assertion failed: the decision emitted different events` → `expected:` (≤ 8 rows) →
`actual:` (≤ 8) → `selected by the model's query:` (≤ 8) → `seeded but NOT selected:`
(≤ 8, truncated **last**) → one line naming the derived query.

---

## Transience policy

Every control gets a disposition and a reason. The library reading of the three
categories: **persistent chrome** = on the crate-root page or in the caller's first
program; **revealed on hover/focus** = reachable in one rustdoc click (an item page, a
sidebar entry) but not on the landing page; **opened on demand** = requires an
explicit act — enabling a feature, following an off-site link, opening a second crate.

| Control | Disposition | Why |
| --- | --- | --- |
| `DomainEvent`, `DecisionModel`, `commit`, `Retry` | **persistent chrome** | the four names the first program cannot be written without. Nothing else qualifies |
| The first program itself | **persistent chrome** | AC-U01; it is the measurement |
| The discriminator paragraph | **persistent chrome** | it is the answer to *"why are there two crates"*, which is the first question the page provokes |
| `Boundary`, `Committed`, `CommandError`, `CodecError` | **revealed** — item pages, linked from `commit`'s signature | a reader meets them through the function that returns them, at the moment they need them. Putting four types on the landing page to explain one function inverts the ratio |
| `Json` / `Postcard` / `Cbor` | **revealed** | `commit` uses JSON; a reader who wants another codec is already looking for one |
| Tuple composition `(B1, B2)` | **revealed**, via one line in the vocabulary region | it is a *capability* of a type already on the page, not a name of its own |
| `happenstance::testing` (`given`/`when`/`then`) | **revealed** — one region on the landing page pointing to the module | test vocabulary belongs one click from the code it tests, not interleaved with it. It is not opened-on-demand because Beat 2 (*"does the domain model work"*) is the beat this project exists to separate from *"pick a database"* |
| `assert_domain_event` | **revealed**, inside `testing` | it exists because of a residual (AC-013); advertising it on the landing page advertises the residual |
| `Projection` / `run_projection` | **opened on demand** — `unstable-projection`, off by default | the port beneath it is not frozen. A reader must perform an act that names its instability before the item is in their build |
| `FaultyStore` / `GappyMemoryStore` | **opened on demand** — a second crate (`happenstance-testkit`) | an application should not carry a test double in its graph. CF-32 |
| The five feature rows | **revealed** — a table on the landing page, plus a `doc_cfg` badge on every gated item | *"a reader must be able to tell what a feature turns on without opening `Cargo.toml`"* (AC-U14) |
| ADR links, `spec/SPECIFICATION.md` links | **opened on demand** — off-site | RS-70-5 asks for the alternative that lost **in the doc comment, once**; the long record is a click away, and only for readers who want it |
| The retry *policy* prose | **persistent** on `commit`'s own item page | AC-U10: *"a doc comment that says 'see the specification' for the policy fails this"* |
| `CHANGELOG.md` | **opened on demand**, linked from `## Stability` | — |

---

## Density budget

Real numbers, in the units this medium actually has. The two named viewports are
where the *rendered* surfaces are read: docs.rs and crates.io at 1440x900 and
1024x768, and a terminal at 80 columns.

**Authored source (what the gate can check).**

| Budget | Number | What yields first when exceeded |
| --- | --- | --- |
| Code line width | **100 columns** — rustfmt's default; `rustfmt.toml` sets only `edition = "2024"`, verified | rustfmt wraps; nothing to decide |
| Doc-comment prose (`//!`, `///`) | **80 columns**, matching the existing facade (`lib.rs:29-33` measures 72-76) | the sentence is split; a doc line is never allowed to run long because rustfmt does not wrap comments and a 120-column doc line renders as one unbroken paragraph in a 700px column |
| Code **inside** a doc fence | **72 columns** | the *domain* yields — a shorter variant name, one course instead of two. Rationale: rustdoc gives `pre` blocks `overflow-x: auto`, so an over-wide line does not wrap, it **scrolls**, and a first program a reader must scroll horizontally is the single most avoidable failure on this page. 72 leaves headroom at 1024x768, where the content column is roughly 700 CSS px ≈ 83 monospace characters |
| Code inside a README fence | **80 columns** | as above; crates.io renders a comparably narrow column |
| First program | **≤ 35 visible lines**, ≤ 2 hidden | the domain yields, never the ceremony — the ceremony is the measurement (AC-U01) |
| Crate-root module doc | **≤ 130 lines** total (today: 61 lines of `//!`) | region 5 (Features) collapses to a list; region 7 never yields |
| **Distance to the first fence** | **≤ 12 prose lines** from the top | this is the above-the-fold rule: at 1440x900 the docs.rs docblock shows roughly 26 lines of body after ~250px of page chrome, so 12 lines guarantees the fence *starts* on screen |
| **First sentence of every doc comment** | **≤ 80 characters**, and a complete claim | rustdoc's item table renders the first sentence in a column roughly 55-65% of the content width and truncates with an ellipsis; at 1024x768 that is about 80 characters. This is the **minimum-legible-label rule**: the item's *name* plus a truncated fragment is a row that has starved its own title |
| Public item identifier | **≤ 24 characters** | so name + summary share one table row at 1024x768 without the summary collapsing. Every name above complies (`assert_domain_event`, 20, is the longest) |

**Terminal transcript (`worked-example-transcript`).**

| Budget | Number | What yields first |
| --- | --- | --- |
| Line width | **80 columns** | the **tag list** wraps to a continuation line indented 8 spaces. The position and the event type **never** truncate — they are the identity of the row. Today's widest line is `main.rs:71-76`'s log row: 3 indent + 3 position + 2 + 22 type + 1 + a two-tag `Debug` (~42) ≈ 73 columns, so the current format fits with 7 to spare and the wrap rule is the headroom for a third tag |
| Section marker | **≤ 60 columns**, always `== … ==` | the marker is rewritten shorter; it is never dropped, because it is the only navigation the surface has |
| Total transcript | **≤ 45 lines** | the `== final log ==` region is the only unbounded one and is capped at the 6 events the example produces. If a future step adds events, the log truncates with `… and N more`, and the sections above it do not |
| Colour, motion | **zero** | — (AC-U16) |

**Assertion failure (`dsl-failure-message`).** Four regions × **≤ 8 event rows** each,
each row ≤ 80 columns. Over budget, `selected by the model's query:` truncates first
and `seeded but NOT selected:` truncates **last**, because the last region carries the
diagnosis. Every truncation prints `… and N more`; none is silent.

---

## Hierarchy

Per region: primary / secondary / recessive, and what carries the distinction.

**`crate-root-rustdoc`.** *Primary:* the first program (region 2) and the four names
inside it. Carried by **position** (first, above the fold) and by **being the only
fence on the page**. *Secondary:* the discriminator and the vocabulary bullets (3-4),
carried by `#` headings and by bold lead-in terms that are also the intra-doc links —
the link *is* the emphasis, so nothing is bolded that is not also reachable.
*Recessive:* features, testing pointer, adapter pointer (5-7), carried by position
(below), by table/list form rather than prose, and by absence of any link into
region 2.

**`crate-readme`.** *Primary:* `## Stability`. It is the only region whose heading a
P4 reader is scanning for and it sits above the first fence. *Secondary:* `## Which
crate do I want?` and `## What DCB buys you`. *Recessive:* `## Guarantees`,
`## Design`, `## Licence` — reference material, carried by position and by being
lists of links.

**`worked-example-transcript`.** *Primary:* the `== … ==` markers, carried by the
delimiter glyphs, by a preceding blank line, and by being the only unindented lines.
*Secondary:* the result lines, carried by 3-space indent and by the literal
`rejected: ` prefix on every refusal. *Recessive:* the final log rows, carried by
column alignment (`{:>3}  {:<22}`) which reads as a table rather than as prose.

**`dsl-failure-message`.** *Primary:* the `seeded but NOT selected:` region — it is
last, which in a panic message dumped to a terminal is the position nearest the
reader's eye, and it truncates last. *Secondary:* expected/actual. *Recessive:* the
derived-query line, one line, at the bottom.

**Rustdoc's own hierarchy, which we control by three levers and nothing else.**
Position in the module doc; whether an item is re-exported at the root (it appears in
the crate's item table) or only in a module (it does not); and `doc_cfg` badges, which
mark a gated item visually without demoting it. We do not use `#[doc(hidden)]`, and we
do not fight the rustdoc theme with inline HTML (AC-U17).

---

## States

What each surface renders in each state. (The *type-level* state set is the next
section; this one is what a reader sees.)

| State | `crate-root-rustdoc` | `worked-example-transcript` | `dsl-failure-message` |
| --- | --- | --- | --- |
| **Empty** | n/a | the store before the first append: `== defining course c1 …` prints before any log row exists, and `== final log ==` over an empty store prints the header and no rows — never a blank screen | `given(…)` with no seeded events prints `seeded but NOT selected: (nothing was seeded)` — the region **stays**, and says so |
| **Loading** | n/a | there is none, and that is a decision: the runner polls, and polling renders **nothing** (no spinner, no in-place rewrite). Its cost is a number in `experiments/`, not a performance (AC-010, AC-U16) | n/a |
| **Error** | n/a | `   rejected: {err}` — the literal prefix at `main.rs:44,55,61`, preserved. The message renders the **carried value** (`course c1 is full (2/2)`), never a category (RS-30-4/30-5) | the four-region message; a store error renders as `the store failed: {err}` with `#[source]` chained |
| **Overflow** | the Features table is the only growable region; a sixth feature adds a row, it does not reflow the page | the final log truncates with `… and N more`; the `== … ==` sections never do | ≤ 8 rows per region, `… and N more`, `selected` yields first |
| **Long label** | an over-long item name breaks the ≤ 24-character budget and is renamed — that is the fix, not truncation | a long tag list wraps to a continuation line indented 8; position and type never truncate | a long event type wraps; the `not selected` marker column stays left-aligned |
| **Narrow viewport** | at 1024x768 rustdoc collapses its sidebar; the 72-column fence budget means the first program still does not scroll horizontally. We author no responsive CSS and must not | 80 columns is the floor; nothing assumes a TTY, and the example runs to completion with stdout piped, which is how the gate runs it | 80 columns |
| **Feature off** | `--no-default-features`: `commit` and the codecs disappear; **every intra-doc link on the page still resolves**, because no link points at a gated item without the gate (RS-70-2). `unstable-projection` off is the default state and the page must read completely without it | unchanged — the example uses default features | unchanged |
| **docs.rs** | `all-features = true` + `--cfg docsrs`: every gated item renders with a `doc_cfg` badge naming its feature (RS-70-4, RS-51-5) | n/a | n/a |

---

## The states the API must express

The set the *types* must make representable, enumerated here so they are designed in
rather than discovered as a missing variant.

- **Empty** — a boundary that selected no events. `read_decision_model` returns
  `(vec![], None)`, and `AppendCondition::after_opt(None)` is the well-formed
  "nothing existed" anchor. The model is simply never `apply`ed. Not an error.
- **Absent** — an event type in the log that this `DomainEvent` does not know.
  `Boundary::absorb` **skips** it silently *by nomination* (the query did not name
  it) but returns `CodecError::UnknownEventType` if it *was* nominated and cannot be
  decoded, because that is a real disagreement between `EVENT_TYPES` and the fold.
- **Conflicting** — `AppendError::ConditionViolated`, matched through
  `is_condition_violated()` (`crates/happenstance-core/src/error.rs:253`), never
  through a `happenstance`-local duplicate enum (AC-U05).
  `ConditionViolated::conflicting_position` is `Option` and **`None` is legitimate**
  for a store with no interactive transaction (`error.rs:135-147`); the loop must not
  branch on `Some`, or it works in-process and fails against `happenstance-neon`.
- **Exhausted** — `CommandError::Exhausted { attempts, source }`. A bounded retry
  that ran out is a distinct outcome from a store failure and from a refusal.
- **Refused** — `CommandError::Refused(D)`, the caller's own domain error, carried as
  a type parameter with `#[source]` (RS-30-2), never flattened to a string.
- **Partially applied** — the projection runner's chunk boundary.
  `Progressed { through, applied }` names how far the checkpoint moved; a panic inside
  `apply` rolls the batch back (PS-30) and a skipped event is recorded in the same
  transaction as the checkpoint (PS-27). The port offers no `apply()`/`set_checkpoint()`
  pair on purpose (`crates/happenstance-core/src/projection.rs:20-26`).
- **Not yet run** — `ProjectionStore::checkpoint` returns `Option<SequencePosition>`
  and `None` means never. Resuming means advancing **past** the checkpoint, because
  `ReadOptions::from` is inclusive (`projection.rs:104-106`), and
  `SequencePosition::next` returns `Option` (`event.rs:272`) whose `None` arm is real.
- **Gapped** — positions may have gaps. Nothing computes `head - checkpoint` as a lag
  (`store.rs:240`), and no test asserts literal positions. `GappyMemoryStore` is the
  instrument.
- **Declined** — a testkit capability that cannot be offered says why, through an
  associated `const` whose constructor rejects an empty reason (RS-40-5). Silence is
  the failure mode this repository has already paid for once.

---

## Anti-patterns

Concrete forbidden moves. Each is phrased so it can be checked against a **rendered
page or a terminal screenshot** by someone who cannot read the code.

1. **The crate-root page shows a bulleted list under the word "Planned".** The
   roadmap is gone; a reader landing after the alpha must not meet one (AC-U03).
2. **The first code block on the crate-root page is not the first thing below the
   summary** — anything other than one sentence sits above it.
3. **The first code block needs horizontal scrolling** at a 1024px-wide window. Any
   scrollbar under a fence on that page fails.
4. **The rendered first program is materially shorter than the one in this file**,
   which means required ceremony was hidden behind `# `. Only a `#[tokio::main]`
   wrapper and an `Ok::<(), E>(())` closer may be hidden.
5. **An item in the crate's item table shows a first-sentence column ending in an
   ellipsis.** The summary was written past its budget.
6. **A feature-gated item's page shows no gate badge** while `Cargo.toml` gates it —
   the docs.rs metadata or the `doc_cfg` attribute is missing.
7. **The README shows two stability claims**, or shows the words "currently a facade"
   anywhere, or puts `## Stability` below the first code block.
8. **The README's opening two lines changed.** That is DT-1 and it is HS-P0016's.
9. **The terminal transcript shows colour, a spinner, a percentage, or a line that
   rewrites in place.**
10. **A transcript line runs past 80 columns**, or a log row whose event type or
    position is truncated to fit.
11. **A refusal in the transcript is missing the literal `rejected: ` prefix**, or
    reads as a category (`capacity exceeded`) rather than carrying the value
    (`course c1 is full (2/2)`).
12. **A DSL assertion failure shows only expected/actual.** The two filter regions are
    the point; a screenshot missing `seeded but NOT selected:` fails.
13. **The compile-fail fixture's `-->` span points into a file the user did not
    write** — a macro body, or anything under `crates/`. It must point at the
    example's own `match` arm.
14. **`happenstance`'s page lists a `Query`, `EventStore`, `Tags` or `Event` that is
    not the contract crate's re-export** (two entries with the same name).
15. **The projection runner appears on the default docs.rs page without a
    `unstable-projection` badge**, or the changelog claims a feature the manifest does
    not have.

Standing, and not re-litigated here: no `#[async_trait]`; no `serde` in
`happenstance-core`'s defaults; `read` returns the stream at the top level; generic
code binds `EventStore`, not `SendEventStore`; no `unwrap`/`expect` in library code.

---

## The doctest

The complete first program, with **no elisions**. This is the artefact DT-2 is judged
on. Two hidden lines are permitted and both are harness, not API: the runtime and the
closer.

```rust
use happenstance::{
    Codec, DecisionModel, DomainEvent, EventType, MemoryEventStore, Retry, Tags,
    commit,
};
use serde::{Deserialize, Serialize};

// The domain. One enum; the compiler makes the fold exhaustive over it.
#[derive(Debug, Serialize, Deserialize)]
enum Enrolment {
    Defined { capacity: u32 },
    Subscribed { student: String },
}

impl DomainEvent for Enrolment {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("CourseDefined"),
        EventType::from_static("StudentSubscribed"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::Defined { .. } => Self::EVENT_TYPES[0].clone(),
            Self::Subscribed { .. } => Self::EVENT_TYPES[1].clone(),
        }
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<happenstance::bytes::Bytes, happenstance::CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        _event_type: &EventType,
        data: &happenstance::bytes::Bytes,
    ) -> Result<Self, happenstance::CodecError> {
        codec.decode(data)
    }
}

// The decision model. It holds its own validated scope, and folds the enum.
#[derive(Clone)]
struct Seats {
    scope: Tags,
    capacity: Option<u32>,
    taken: u32,
}

impl DecisionModel for Seats {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::Defined { capacity } => self.capacity = Some(capacity),
            Enrolment::Subscribed { .. } => self.taken += 1,
        }
    }
}

let store = MemoryEventStore::new();
let seats = Seats {
    scope: Tags::from_pairs([("course", "c1")])?,
    capacity: None,
    taken: 0,
};

let committed = commit(&store, seats, Retry::attempts(3.try_into()?), |seats| {
    Ok::<_, core::convert::Infallible>(vec![Enrolment::Subscribed {
        student: "s1".into(),
    }])
})
.await?;

assert_eq!(committed.attempts, 1);
```

**Count, because AC-U01 and AC-013 are the same measurement read at two altitudes.**
Domain logic — the enum, the fold's two arms, the decision — is **11 lines**. Mapping
ceremony — `EVENT_TYPES`, `event_type`, `tags`, `encode`, `decode` — is **26 lines**.
That is 2.4:1 against the domain for a two-variant enum, and it will worsen, not
improve, with a third. **The prediction this design records: AC-013's verdict is
"`happenstance-macros` is in scope for 0.1"**, and the implementer should expect to
record it as such. That prediction is falsifiable and must be checked against the
*rewritten example*, not against this doctest (`RUNBOOK.md:524`).

**The `compile_fail` companion**, per RS-62-1 (pair it with a compiling one, do not
trust its error code). It belongs on `DomainEvent`'s item page:

```rust
// This must NOT compile: a boundary that constrains nothing.
struct Nothing;
impl DomainEvent for Nothing {
    const EVENT_TYPES: &'static [EventType] = &[];
    // …
}
// The const assertion fires: "a DomainEvent must declare at least one event type".
```

The reader is meant to see a **post-monomorphisation const-eval error naming the
event-type set**, not a trait-resolution error. Spelled bare `compile_fail`, never
`compile_fail,E0080` — rustdoc on 1.97.1 silently ignores an unmatched code, so the
stricter-looking spelling is the weaker check (`crates/happenstance-core/src/event.rs:95-106`
says exactly this about the same construct). **This doctest is not AC-002's
instrument.** AC-002's instrument is the `trybuild` fixture, and it remains blocked.

---

## Mock

| Artefact | Path | Status |
| --- | --- | --- |
| Static mock | [`design/mock.html`](design/mock.html) | **built.** 28 frames, 6 surfaces, self-contained (no network fetch of any kind — verified: zero non-`data:` `url()`, zero `<script>`, zero `@font-face`) |
| Viewports | `1440x900`, `1024x768` | each frame is drawn at its real declared width, and the frame's bottom edge **is** the fold |
| Themes | `light` | the only declared theme. rustdoc's light palette is used as-authored, via its own `:root:not([data-theme])` custom properties |

**The named gap turned out to be narrower than this file claimed, and closing it
changed the design.** The paragraph that used to sit here said nothing in the
repository mechanically captures what a rustdoc page renders as. That is true of
*this* crate, but not of the medium: rustdoc is itself the design system, and it
can be run against a doc comment that has no implementation behind it. The mock
does exactly that — a throwaway crate carrying this file's module doc, item set,
summaries and feature gates was compiled three times (default,
`--no-default-features`, and `--all-features --cfg docsrs` on nightly) and the
generated HTML is embedded verbatim in the frames, with rustdoc's own stylesheet,
its own light-theme custom properties and its own class contracts
(`.docblock`, `.item-table`, `.example-wrap`, `.stab.portability`,
`pre.rust.rust-example-rendered`). Four of the six surfaces have no substrate at
all — a terminal, a panic message, `trybuild`'s stderr, crates.io's markdown — and
each frame says so in its own `substrate` row rather than borrowing authority it
does not have.

| Surface | Frames | Provenance |
| --- | --- | --- |
| `crate-root-rustdoc` | F-01 … F-06 | **generated** — real rustdoc output, all three feature states |
| `crate-readme` | F-07 … F-09 | **authored** (mock-local markdown CSS; crates.io's is not obtainable offline) |
| `first-program-doctest` | F-10 … F-13 | **generated** (rendered) / **projected** (the two compile states) |
| `worked-example-transcript` | F-14 … F-21 | **captured** — real stdout from `cargo run -q -p course-subscriptions` |
| `dsl-failure-message` | F-22 … F-25 | **projected** — this file's four-region rule applied |
| `compile-fail-diagnostic` | F-26 … F-28 | **projected**, and one frame **blocked** (`trybuild` absent) |

**Reference captures the design review will compare against**, one per surface per
viewport:

```
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/crate-root-rustdoc@1440x900.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/crate-root-rustdoc@1024x768.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/crate-readme@1440x900.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/crate-readme@1024x768.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/first-program-doctest@1440x900.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/first-program-doctest@1024x768.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/worked-example-transcript@1440x900.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/worked-example-transcript@1024x768.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/dsl-failure-message@1440x900.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/dsl-failure-message@1024x768.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/compile-fail-diagnostic@1440x900.png
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/design/reference/compile-fail-diagnostic@1024x768.png
```

`design.capture` stays absent from `.redkiln/config.yaml` and the perceptual review
stays a declared skip — there is still no app. What the mock replaces is the *manual*
`cargo doc --open` pass this section used to ask a reviewer to perform from memory:
the three feature states are now rendered side by side, at both viewports, with the
density budget measured off the rendered artefact rather than asserted.

**Five things the mock found that no textual review had.** They are recorded here,
against the frame that shows each one, and are **not** silently applied to the
sections above — each is a decision the approver is being asked to take.

1. **The first program is 81 visible lines and 107 columns** (budgets: 35 and 72),
   so region 2 is not "alone above the fold" — the discriminator paragraph lands
   roughly 1,100px below it — and the fence scrolls horizontally at 1024, which is
   anti-pattern 3 (F-01, F-02, F-11). This is DT-2's measurement, taken; it is also
   the strongest evidence yet for AC-013's verdict.
2. **Intra-doc links to gated items do not resolve when the gate is off.** The
   vocabulary and Features regions as specified emitted three rustdoc warnings under
   default features, and `commit_with`'s summary two more under
   `--no-default-features`. RS-70-2 is satisfiable, but only with conditional doc
   lines (`#![cfg_attr(not(feature = "json"), doc = …)]`), which no brief asked for.
   All three builds are now warning-free (F-03, F-05).
3. **Anti-pattern 5 is unfixable inside this project.** Three item summaries on the
   crate-root page exceed the 80-character budget and three item names exceed the
   24-character one — `ConditionViolated`, `EventId`, `read_decision_model`,
   `MIN_SUPPORTED_EVENTS_PER_BATCH` and friends — and **every one arrives through
   `pub use happenstance_core::*`**, so every one is frozen by AC-A02. The budget was
   written against this project's own items; the page it is checked on is not this
   project's alone. Either the anti-pattern gains a scope or it is not checkable.
4. **The Features table is four rows or five, and this file says both** —
   *Composition* region 5 says four, *Transience policy* says "the five feature rows".
   The frames render five, because `memory` gates `testing::given` (F-05).
5. **A long event type overruns the transcript's 22-character type column**, so the
   tag set starts late and the column alignment the recessive hierarchy rests on
   breaks for that row. The wrap rule covers the tags; nothing covers the type
   (F-20).

---

## Sign-off

**Approved.**

| Who | When | Conditions |
| --- | --- | --- |
| Ryan Britton (repository owner) | 2026-08-12 | None. All three items below accepted as put, including DT-2's resolution toward explicit declaration and the 2.4:1 ceremony ratio recorded as a falsifiable prediction about AC-013's verdict. |

Recorded at the `/redkiln:plan` design sign-off gate, against the mock at
`design/mock.html` (opened locally; not published as an artifact — it embeds a
vendored rustdoc stylesheet and is 928 KB, which is a defect worth fixing the next
time that file is touched, not a blocker on this design).

One citation was repaired before sign-off: this file's *"no `interaction-patterns.md`
was ever commissioned"* claim cited `_decomposition.md:411-412,479-482`; the real
source is `initiative.md` at those same ranges, which is what every sibling project's
`_design.md` cites. Both ranges were re-read and say exactly what was claimed.

Three things the approver is specifically being asked to accept, because each is a
decision this file made that a brief had left open or had decided differently:

1. **DT-2 resolves toward explicit declaration**, with the ceremony paid in `const`
   items the compiler checks and one `?` the caller was already writing — and the
   doctest above is the whole of the evidence. If the ceremony reads as too much, the
   answer is not to relax the shape; it is to accept AC-013's derive.
2. **`## Stability` goes above the first code block**, overriding the architecture
   and deployment briefs' `## Guarantees`/`## Design` placement (AC-U18 wins on
   ordering).
3. **`happenstance-testkit` publishes `0.2.0-alpha.1`**, not a stable `0.2.0`.

And two inputs this design **cannot** supply, both of which must be resolved before
the stories that depend on them start:

- **`trybuild`** — AC-002's only instrument, absent from the workspace, HS-P0010's
  decision. `compile-fail-diagnostic` is specified and unbuildable without it.
  Escalate rather than substitute.
- **`MemoryProjectionStore`** — HS-P0010's AC-012. `run_projection`'s transactional
  test has nothing to run against until it lands, and writing a throwaway here would
  freeze a fixture shape this project does not own.
