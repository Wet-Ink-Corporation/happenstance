# Is `DomainEvent::tags` covering `DecisionModel::scope` checked in `commit_with`, merely documented, or left to the author?

Decision record: **P-2-tags-scope-agreement**. Source: `references/evaluation/review-pre-publication-2026-09-03.md:589-624`.

---

## Why this is owed

**ADR-0020 closed one half of a two-halved question and the other half is still open.**

`kb-decision-0020` (`.kb/decisions/0020-fold-query-agreement.md`, `status: accepted`, `adr_id: ADR-0020`) exists because a DCB handler names its event set twice — once in the query, once in the fold — and nothing checked they agreed. Its long form names the corrupting direction precisely, at `references/adr/0020-fold-query-agreement.md:66`:

> | Fold interprets a type the query never selected | nothing — the arm is simply never taken | the arm is dead, and the decision is made on a **narrower** log than it believes. The append condition protects less than the handler assumes. This is the one that corrupts |

ADR-0020 made that unwritable for **event types**: `EVENT_TYPES` is declared once, the fold is exhaustive over the same enum, and the query is derived on a sealed trait the caller cannot override (`crates/happenstance/src/boundary.rs:69`, `:112-119`).

It did not touch **tags**. `Boundary::query` derives the query's tag half from `DecisionModel::scope()`; the store matches it against `DomainEvent::tags()`; the two are declared independently, in two traits, by two methods, and nothing in the workspace relates them. A model whose events do not carry its scope lands on exactly the row quoted above: it reads an empty fold on every attempt, and the append condition built from that same query matches nothing. The condition is present, evaluated, and unconditional.

That outcome is the one `happenstance-core` already names and seals *by the other route*, at `crates/happenstance-core/src/query.rs:126-129`:

> That seal is load-bearing rather than tidy. An `AppendCondition` built on a query with no items is a condition nothing can ever violate — a conditional append that is silently unconditional, which is a lost update with no diagnostic anywhere.

The zero-item route is structurally unreachable. The tag-mismatch route reaches the same state and is wide open.

**Two of the crate's own rendered pages teach the violation** (counted below), which turns this from an author's mistake into something the documentation is currently instructing.

**Which atom this supersedes: none.** ADR-0020 is `status: accepted` and therefore immutable; nothing proposed here contradicts a sentence of it. Its Decision section decides `scope()`'s *signature* (`references/adr/0020-fold-query-agreement.md:203-225`) and never the coverage relation. This is a new question that ADR-0020 makes visible — the unclosed half of the same class — and it needs a new number from the RUNBOOK's ADR queue (`RUNBOOK.md:262`), with `kb-decision-0020` as its parent rather than its target.

**What the specification says: nothing.** No clause governs it. `spec/SPECIFICATION.md:102-107` puts the typed layer outside the clause space:

> This is the normative architectural specification for happenstance — the project, not the crate of that name, which is the typed layer and one consumer of what follows.

The nearest clause subject in ADR-0020's own residual log is **VT-18** (`spec/SPECIFICATION.md:1401`, `[FROZEN]`, discharged at phase 4), and it is about `Event::new`'s conversion bounds, not about tag coverage. So there is no maturity marker to move and no frozen clause to amend — the only governing authority is a decision record that does not yet exist.

---

## What is true today

### The two declarations, ninety-odd lines apart in one file

`crates/happenstance/src/domain.rs:78-79` — the whole of it:

```rust
    /// The tags this event carries.
    fn tags(&self) -> Tags;
```

`crates/happenstance/src/domain.rs:172-178`:

```rust
    /// The tags every event inside this boundary carries.
    ///
    /// Already validated: [`Tags`] has no infallible constructor that can
    /// produce an invalid value, so returning a reference to a held value is
    /// what keeps the fallibility in the caller's constructor instead of
    /// inside an infallible signature, where it could only become an `unwrap`.
    fn scope(&self) -> &Tags;
```

"The tags **every** event inside this boundary carries" is a claim about `tags()`, written on `scope()`, and enforced nowhere.

### The bridge that makes them a pair

`crates/happenstance/src/boundary.rs:115-119`:

```rust
    fn query(&self) -> Result<Query, InvalidQuery> {
        // Reading the `const` is what evaluates it, here, once per model.
        let () = AtLeastOneType::<M::Event>::CHECKED;
        derive_query(M::Event::EVENT_TYPES, self.scope())
    }
```

`crates/happenstance/src/boundary.rs:172-175`:

```rust
pub(crate) fn derive_query(types: &[EventType], scope: &Tags) -> Result<Query, InvalidQuery> {
    let item = QueryItem::new(types.iter().cloned(), scope.clone())?;
    Ok(Query::from_item(item))
}
```

The scope becomes the item's required tag set. Matching is by superset, `crates/happenstance-core/src/query.rs:112-115`:

```rust
    pub fn matches(&self, event_type: &EventType, tags: &Tags) -> bool {
        let type_ok = self.types.is_empty() || self.types.binary_search(event_type).is_ok();
        type_ok && tags.contains_all(&self.tags)
    }
```

So an event is selected only when **its own** tags contain **all** of the model's scope. `Tags::contains_all` is `crates/happenstance-core/src/tag.rs:347`.

### Where the emitted event is tagged

`crates/happenstance/src/command.rs:356-360`, inside `encode`:

```rust
        batch.push(
            built
                .with_tags(event.tags())
                .with_metadata(crate::codec::frame::<C>(None)),
        );
```

The event goes onto the log carrying whatever `tags()` returned. Nothing compares that against `model.query()`, which is bound eleven lines earlier at `command.rs:290` and still in scope at `command.rs:309`:

```rust
        let decided = decide(&model).map_err(CommandError::Refused)?;
        let batch = encode::<B::Event, C, S::Error, D>(&decided, codec)?;

        // From the read, and only from the read.
        let condition = AppendCondition::new(query).after_opt(anchor);

        match store.append(&batch, Some(&condition)).await {
```

(`command.rs:305-311`.) Both halves are in hand at exactly one place, and neither is consulted about the other.

### Nothing checks the containment — measured

`grep -rn "contains_all" --include=*.rs crates/ examples/` returns **one** production call site: `crates/happenstance-core/src/query.rs:114`, the matcher above. Every other hit is in `happenstance-testkit`'s own tests, `happenstance-core/tests/wire.rs`, or a property test of `contains_all` itself. There is no second predicate anywhere that relates a `tags()` to a `scope()`.

### The crate's one stated remedy for a non-compiler-enforceable agreement does not cover it

`crates/happenstance/src/testing/mod.rs:378-383`:

```rust
pub fn assert_domain_event<E: DomainEvent>(every_variant: &[E]) {
    for value in every_variant {
        let carried = value.event_type();
        assert!(
            E::EVENT_TYPES.contains(&carried),
```

It never reads `tags()`, and it has no `scope` to compare against — it takes `&[E]`, not a model.

### The rendered doctests that violate it — the count

Eight rustdoc examples in the workspace impl `fn tags(&self) -> Tags`, and **all eight** return `Tags::empty()` (`boundary.rs:47`, `composition.rs:132`, `domain.rs:36`, `domain.rs:141`, `lib.rs:40`, `runner.rs:375`, `testing/mod.rs:24`, `testing/mod.rs:368`).

Of those eight, six are consistent or vacuous:

| Page | Why it is not a violation |
| --- | --- |
| `boundary.rs:47` | `compile_fail`; `Divergent` implements `Boundary` directly with `Query::all()`, no `scope` |
| `domain.rs:36` | `compile_fail`; `Empty(Tags::empty())` — empty scope |
| `lib.rs:40` | model is `Seats { scope: Tags::empty(), … }` (`lib.rs:57`) |
| `runner.rs:375` | `Projection` is `scope: Tags::empty()` (`runner.rs:405`) |
| `testing/mod.rs:24` | model is `Seats { scope: Tags::empty(), … }` (`testing/mod.rs:41`) |
| `testing/mod.rs:368` | `assert_domain_event` example; no model at all |

**Two rendered pages teach the violation, and they instantiate three such models between them.**

`crates/happenstance/src/domain.rs` — `DecisionModel`'s own page. `:141`:

```rust
///     fn tags(&self) -> Tags { Tags::empty() }
```

`:163-165`:

```rust
/// let scope = Tags::from_pairs([("course", "c1")])?;
/// let seats = Seats { scope, taken: 0 };
/// assert_eq!(seats.query()?.items().map_or(0, <[_]>::len), 1);
```

`crates/happenstance/src/composition.rs` — the composition page, twice. `:132` is the same `Tags::empty()` impl; `:150-151`:

```rust
/// let capacity = Counter { scope: of("course", "c1")?, seen: 0 };
/// let student = Counter { scope: of("student", "s1")?, seen: 0 };
```

Both doctests pass, because both assert on `query()?.items()` length (`domain.rs:165`, `composition.rs:155`) and neither ever folds against a store. A `Seats` built exactly as `domain.rs:163-164` writes it can never see a `Seat` written exactly as `domain.rs:141` writes it.

### The instrument exists and nobody is pointed at it

`crates/happenstance/src/testing/mod.rs:157` seeds with `.with_tags(event.tags())`, and `Given::when` (`testing/mod.rs:180-238`) computes the set difference between everything seeded and everything the boundary's query selected. A failed `then` renders four regions, the fourth being `seeded but NOT selected:` (`testing/mod.rs:283-285`). So a mismatch **already** renders as an empty `selected` region beside a populated `seeded but NOT selected` region — but only for an author who wrote a test that seeds and expects the fold to see it, and the message does not name tag coverage as the cause. Nothing fails if the author never writes that test.

### The examples get it right, which is what makes this a gap rather than a misunderstanding

`examples/transfers-on-sqlite/src/main.rs:294-299`:

```rust
    fn tags(&self) -> Tags {
        match self {
            Self::AccountOpened { account }
            | Self::Deposited { account, .. }
            | Self::Withdrawn { account, .. } => [account.tag.clone()].into_iter().collect(),
        }
    }
```

`examples/course-subscriptions/src/main.rs:200-210` does the same with course and student tags.

### What an empty decision does today — the fact the scoring turns on

**An under-tagged model folds empty, and a fold that saw nothing is the input to `decide`.** For the class of handlers whose correct answer on an empty fold is "emit nothing" — the idempotent no-op, the conditional retraction, "unsubscribe only if subscribed" — an under-tagged model therefore **decides empty**.

That class does not reach a silent success today. **It reaches a loud error.** `commit_with` has no empty guard, and `encode`'s loop simply never runs (`crates/happenstance/src/command.rs:342-343`):

```rust
    let mut batch = Vec::with_capacity(decided.len());
    for event in decided {
```

so `&[]` is what reaches `store.append` at `command.rs:311`. The port requires the refusal — `crates/happenstance-core/src/store.rs:251-253`:

> * [`AppendError::NoEvents`] when `events` is empty. The specification defines a batch as non-empty, so there is no position to return. This check MUST precede the condition check […]

and the reference store performs it above the lock (`crates/happenstance-core/src/memory.rs:360-362`). It is **ES-20**, `[FROZEN]` (`spec/SPECIFICATION.md:3542-3548`) — *"An empty batch is refused, and refused first."* The caller receives `Err(CommandError::Append(NoEvents))`.

Two consequences follow, and the whole scoring of the options below turns on them.

**The empty-decision class cannot corrupt.** The direction ADR-0020 names as corrupting is *"the append condition protects less than the handler assumes"* (`references/adr/0020-fold-query-agreement.md:66`) — a **lost update**, which requires an append to *land*. ES-20 guarantees an empty decision writes nothing at all. So every case in which an under-tagged model's vacuous condition can actually lose an update has a **non-empty** decision — which is exactly where a check written over the decided events fires.

**What the empty-decision class loses is the diagnostic, not the write.** The chain a caller sees is `appending the decided events failed` (`command.rs:118`) → `an append must contain at least one event` (`crates/happenstance-core/src/error.rs:224`), over a variant documented *"This is a caller bug, not a store failure"* (`error.rs:222-223`). That is loud, and it is misattributed: nothing in it names tag coverage. **That misattribution is Y-1's subject, not this one** (`review-pre-publication-2026-09-03.md:554-582`); the coupling is stated under *Cost of delay*.

One property does still exceed what any run-time form can express: coverage is quantified over **every variant of the enum and every model that names it**, and a run-time loop sees only the variants one decision emitted. What that costs is *when* a violation is found — first emission of the offending variant, in production, rather than in a test — and not *whether* a bad write can land. The place to quantify over an enum is a test helper handed every variant, which is what `assert_domain_event` already is (`testing/mod.rs:378`).

---

## Options

Costs to an **adapter author** are stated for completeness and are all the same: **zero**. Every artefact in this question — `DomainEvent`, `DecisionModel`, `Boundary`, `commit_with` — lives in `happenstance`, above the contract. No store adapter, no conformance rule and no `happenstance-core` type changes under any option. If the answer needs an adapter to do anything, the answer is wrong.

### Option A — Check it in `commit_with`, against the boundary's own derived query

Between `command.rs:305` and `:311`, reject the commit when a decided event's `(event_type, tags)` is not matched by `query`. `Query::matches` (`crates/happenstance-core/src/query.rs:221`) is the same predicate `Boundary::absorb` already asks at `boundary.rs:138`, so no second filter vocabulary is invented — which is what ADR-0020's own "second place the event set is named" objection would otherwise catch (`boundary.rs:124-128`).

**What it covers.** Every append that *lands*. The check runs over the decided events — `decided` at `command.rs:305`, or the `batch` `encode` builds from it (`command.rs:343`, `for event in decided`) — so a non-empty decision whose events the boundary's own query does not select is refused before `append`. A decision that is empty because the model folded empty never reaches a successful append either: ES-20 refuses it (`spec/SPECIFICATION.md:3542-3548`, `store.rs:251-253`, `memory.rs:360-362`). Between the two, **no write produced by an under-tagged model can land**, which is the corrupting direction stated at `references/adr/0020-fold-query-agreement.md:66`.

**What it does not cover, and this is structural rather than incidental.**

- **It finds a violation late — at first emission, not at authoring time.** The loop sees only the variants this run emitted, never the enum. If variant `X` carries the scope and `Y` does not, the first `Y` a program ever decides is where the error arrives: in production, loudly, but after the code shipped. That is a *when*, not a *whether*.
- **It is grain-limited on a composite.** `Boundary::query` for a tuple is the **union** (`composition.rs:45-73`) and `Query::matches` for `Items` is `any` (`crates/happenstance-core/src/query.rs:224`), so the check proves an event is selected by **at least one** member — never by the member whose invariant motivated it. `(capacity, student)` with an event carrying only the course tag passes, while `student`'s fold stays empty. The corrupting direction survives at that grain.
- **The empty-decision class keeps a misattributed error rather than gaining a right one.** The check adds nothing there, because ES-20 already refuses it; what the caller reads still says "an append must contain at least one event" (`error.rs:224`) and never names tag coverage. Repairing *that* message is Y-1's decision, not this one.

- **Cost to a caller.** A program that ran and silently protected nothing now returns an error — for the non-empty decisions only. Correct programs pay one `Query::matches` per decided event per attempt — a binary search over a short sorted slice plus a merge-scan over two sorted tag sets (`tag.rs:347-360`), on a path that is already about to do a network or disk write. Callers who deliberately emit an out-of-boundary event alongside the guarded one (an audit or notification event with different tags) must stop, or split into two commits, or the check must be opt-out.
- **Cost to an adapter author.** None.
- **Semver class.** **Behaviour-breaking.** A new `CommandError` variant is itself additive — `CommandError` is `#[non_exhaustive]` (`command.rs:97`) — but a call that used to return `Ok` now returns `Err`. `0.2.0-alpha.1` is a pre-release and no `^0.2` requirement resolves to it, so the change is free today and permanent after `0.2.0`.
- **What it forecloses.** Emitting an event from inside a `commit_with` that the boundary deliberately does not select — an audit or notification event with different tags now has to leave by another route, or the check needs an opt-out. It also fixes the guarantee at the union grain named above: `Boundary` exposes the union query (`composition.rs:45-73`) and per-member absorption (`:75-90`) with no way to enumerate members, so the weaker guarantee is what a later, stronger per-member check would have to be written *around*.

### Option B — Document it, and repair the two violating doctests

Prose on `tags` and `scope` naming the obligation and its consequence; replace the `Tags::empty()` impl on `domain.rs` and `composition.rs` with one that returns the model's scope, so the rendered pages stop teaching the violation.

- **Cost to a caller.** Nothing, and no protection either. The failure remains reachable, silent, and diagnosable only by a test the author may not write.
- **Cost to an adapter author.** None.
- **Semver class.** **None.** Doc comments and doctest bodies; no signature moves.
- **What it forecloses.** Nothing — it is compatible with every other option and is arguably owed under all of them. Note it is *not* the same work as **P-3** (`review-…:3598-3634`), which asks for one rendered example of the *fallible-tag* idiom and is explicitly `Semver: None`; the two doc changes touch the same eight examples and should be sequenced together, but P-3 does not repair this defect and this does not repair P-3's.

### Option C — Arm the test-time instrument instead of the run-time path

Make `Given::event` refuse (or `Decision::then` assert) a seeded event the boundary's own query does not select, and/or give `assert_domain_event` a sibling that takes a scope and every variant and checks coverage. The set difference is already computed (`testing/mod.rs:230-237`).

The two halves are separable and score differently, which matters below:

- **C-additive** — the `assert_domain_event` sibling: `fn assert_scope_covered<E: DomainEvent>(scope: &Tags, every_variant: &[E])`, asserting `value.tags().contains_all(scope)` for each. It is **enum-total** — it quantifies over every variant the author hands it, exactly as `assert_domain_event` already quantifies over `EVENT_TYPES` — so it finds the violation at authoring time rather than at first emission, and, taken a member at a time, it reaches the **per-member** grain a union-query check cannot. **Semver: additive (minor)**, not none: it is a new `pub fn` in `happenstance::testing`, and `pub mod testing` is in the crate's published default surface — `#[cfg(all(feature = "memory", feature = "json"))]` (`crates/happenstance/src/lib.rs:212-214`) over `default = ["std", "memory", "json"]` (`crates/happenstance/Cargo.toml`). Additive is cheap, but it is not free of a version number, and it enlarges a surface Y-2 is about to be asked to justify.
- **C-refusal** — `Given::event` rejecting an unselected seed. Test-surface behaviour-breaking, and it needs the pre-release window only in the weak sense that a test-only refusal does.

- **Cost to a caller.** Only inside `happenstance::testing`. A test that seeded an out-of-boundary event on purpose now has to say so. Callers who write no test are protected by nothing.
- **Cost to an adapter author.** None.
- **Semver class.** Split, and the two halves differ. `C-refusal` is **behaviour-breaking on the test surface** — `Given::event` already returns `Result<Self, CodecError>` (`testing/mod.rs:144`), so a refusal fits the existing signature, and the same pre-release freeness applies. `C-additive` is **additive (minor)**.
- **What it forecloses.** Little. It is the option ADR-0020 already used for the residual it could not close structurally — "`EVENT_TYPES` ↔ `event_type()` agreement is not compiler-enforced and is tested by `assert_domain_event::<E>(&[…])` — the residual is named rather than hidden" (`.kb/decisions/0020-fold-query-agreement.md`, *Consequences*). Choosing it here is consistent with that precedent, and it does not preclude Option A later.

### Option D — Repair it silently: union the model's scope into the written event's tags

In `command.rs::encode`, add the boundary's scope to every emitted event's tags, so coverage holds by construction.

**The repository evidence does not support this option, and it should be named only to be rejected.** `encode` (`command.rs:335-363`) takes `&[E]` and a codec and has no boundary in hand; supplying one is possible, but `Boundary::query` returns the **union** for a tuple (`composition.rs:54-72`) and there is no per-member scope to union in — for `(capacity, student)` there is no defensible answer to *which* scope the event acquires. It also writes tags the author never declared into a durable log whose tags are an adapter's index key (`tag.rs:251-260`), which changes what the event *means* rather than checking what it claims.

- **Semver class.** Behaviour-breaking, and worse: it silently changes the bytes of past-shaped writes.
- **What it forecloses.** Any later reading of `tags()` as the author's own statement of what an event is about.

---

## Recommendation

**Option A plus Option B, in one change. `C-additive` is endorsed on its merits but deliberately not bundled — it has no clock, and its remit is Y-2's.**

**Semver headline: behaviour-breaking (A), none (B).** Were `C-additive` bundled, the headline would be **additive (minor)** for it and none for B — not "none for both", which is what revision 2 claimed.

Confidence: **medium-high** on A + B; **medium** on holding `C-additive` back rather than bundling it.

Why A:

1. **It is the only option that closes the corrupting direction ADR-0020 named.** That direction is a *lost update* — an append that lands while the condition built beside it protects less than the handler assumes (`references/adr/0020-fold-query-agreement.md:66`). A refusal placed between `command.rs:305` and `:311` covers every decision that would land, and ES-20 already covers every decision that would not (`spec/SPECIFICATION.md:3542-3548`). Nothing else in the option set touches the run-time path at all, so nothing else closes it. *(This reason was deleted in revision 2 on the ground that A "passes vacuously on an empty decision". The premise was false: an empty decision is not a pass, it is `Err(CommandError::Append(NoEvents))`, and a batch that is never written cannot lose an update.)*
2. **It reuses the predicate the crate already asks.** `Query::matches` (`crates/happenstance-core/src/query.rs:221`) is what `Boundary::absorb` asks of an event coming back (`boundary.rs:138`), so no second filter vocabulary is invented — the objection ADR-0020's own design comment raises against exactly that (`boundary.rs:124-128`).
3. **It is the only item here with a clock, and the clock is short.** A is behaviour-breaking and free only while `0.2.0-alpha.1` is a pre-release no `^0.2` requirement resolves to (`Cargo.toml:6-15`). B costs nothing ever; `C-additive` is additive and therefore available at any minor. Spending the one window on the only change that needs it is the disciplined use of it — and the failure A prevents is invisible by construction, so the pressure to pay a major bump for it later will never arrive.
4. **B is owed under every option** and stops two rendered pages teaching the violation (`domain.rs:141`/`:163-165`, `composition.rs:132`/`:150-151`) regardless of what else is decided. It is also the only half that repairs the *documentation* defect, which A does not touch.

Why `C-additive` is endorsed but held back:

- What it uniquely buys over A is **earlier** detection (authoring time rather than first emission) and the **per-member** grain A cannot reach on a composite. Both are real, and neither expires: it is additive, so it can land at any later minor at the same cost it has today.
- Against bundling it now: it puts a second public helper into `happenstance::testing` (`lib.rs:212-214`, default features) while **Y-2** is the open question about what `assert_domain_event`'s remit *is*. Adding the sibling first hands Y-2 a two-function surface to rationalise rather than a question to answer.
- And its second use — giving the empty-decision class a diagnostic that names tag coverage — only becomes concrete once **Y-1** decides what an empty decision returns. Sequencing it after Y-1 costs nothing and lets it be shaped by that answer.

`C-refusal` (the `Given::event` refusal) is compatible with all of this and cheap; it is not required, and it can follow.

**The strongest argument against this recommendation, stated in its own words:**

*A refuses at the boundary's **union** query, and on a composite that proves only that some member selects the event. `(capacity, student)` with a `student`-blind event passes A and still decides `student`'s invariant on an empty fold — so A does not close the corrupting direction, it closes it at one grain and freezes the weaker guarantee into the crate's behaviour, which a later per-member check would have to be written around. `C-additive`, pointed at each member's scope in turn, reaches the grain A cannot, at authoring time, for a minor. Spending the irreplaceable window to buy the weaker of the two guarantees, and deferring the stronger one to a question (Y-2) that has no number, is the wrong order.*

The counter is that the two guarantees are not substitutes: A binds every program, `C-additive` binds only the author who calls it, and `query.rs:126-129`'s "a lost update with no diagnostic anywhere" is a run-time state that no test helper removes from a shipped binary. A partial run-time refusal that catches the single-model case — which is every model in both worked examples and both violating doctests — is worth more than the same coverage available only to authors who opt in. But the composite hole is real and the decider should know they are buying the union grain.

**The discriminator, if these still read as close.** Ask whether the crate is willing to forbid emitting an out-of-boundary event from inside `commit_with` (an audit or notification event tagged differently, alongside the guarded one). If yes, take A now — it is free while `0.2.0-alpha.1` is a pre-release and never again. If that foreclosure is unacceptable without an opt-out, A needs an escape hatch designed before it lands, and the honest sequence is B now, `C-additive` after Y-1/Y-2, and A only once the escape hatch is specified — accepting that A then costs a major bump and probably does not happen.

---

## Cost of delay

**Not equally cheap forever. Of the three options worth taking, only Option A has a clock.**

- **Option A is free now and permanent at `0.2.0`.** The workspace is at `version = "0.2.0-alpha.1"` (`Cargo.toml:15`). No `^0.2` requirement resolves to a pre-release, so today a program that used to return `Ok` and silently protect nothing can be made to return `Err` at no cost to anybody. After `0.2.0` ships, the same change is a breaking release, and the realistic outcome of that is that it never happens: the failure it prevents is invisible, so the pressure to pay a major bump for it will never arrive.
- **Options B and C are cheap forever.** Doc comments and doctest bodies cost nothing at any time; `C-additive` is additive (minor) and so is available at every future minor at exactly today's price; `C-refusal` touches only the test surface. Deferring B loses only the readers who follow `domain.rs:141` in the meantime, which is why it is recommended now anyway. Deferring `C-additive` loses nothing at all, which is why it is not bundled.
- **The coupling with Y-1 is a correctness coupling, not a merge-ordering one.** Both changes want the same six lines of `commit_with` (`command.rs:305-311`), so there is a rebase either way — but that is the trivial half. The load-bearing half is that **today's loud `Err(NoEvents)` is what keeps the empty-decision class from being silent**, and Y-1 is the decision that removes it. A Y-1 outcome that converted the empty decision into a plain `Ok` would create the silent hole this brief exists to prevent — an under-tagged model returning success forever with nothing written and nothing said.

  Y-1's own brief has since landed on **Option 3, two success shapes** — a `#[must_use]` two-armed outcome the caller must match, "loud at *compile time*, at every call site" — rather than on the short-circuit-to-`Ok` shape. **That resolves the silent-`Ok` risk and does not resolve the coupling.** It resolves the risk because the nothing-to-do arm cannot be discarded by `…await?;` without a warning, so no caller can absorb the case unknowingly. It does not resolve the coupling because Y-1's own analysis concedes the arm "still does not distinguish 'deliberate no-op' from 'forgot to `push`'" — and an under-tagged model is a third thing wearing the same arm. So after Y-1 the empty-decision class is unignorable but still unexplained, and the instrument that explains it is `C-additive`, at authoring time. **That is the sequencing this brief recommends**, and it is why `C-additive` is held for after Y-1 rather than dropped. Neither Y-1 nor this question has a number in the ADR queue (`RUNBOOK.md:262`).

---

## What this does not settle

- **The projection port has the identical pair, and the recommended change does not reach it.** A check in `commit_with` is on the command path only. `crates/happenstance/src/runner.rs:433` is `derive_query(P::Event::EVENT_TYPES, projection.scope())` — the same function, the same two independent declarations, `Projection::scope()` against `DomainEvent::tags()`. The failure mode differs (no append condition, so nothing is corrupted; the projection simply applies nothing while its checkpoint advances), which is why it may deserve a different answer — but it deserves a stated one, and after A + B the two ports would be visibly unequal. `C-additive`, being a free function over a `&Tags` and a slice of variants, is the one instrument here that *can* be pointed at a `Projection::scope()` by an author who thinks to; that is a further argument for it landing eventually, and not an argument for bundling it now. ADR-0008 (`kb-decision-0008`, accepted) is the record that makes "one derivation, both ports" the standing expectation, so a check on one port and silence on the other is a departure worth naming.
- **Per-member granularity inside a composite.** A check written against `Boundary::query` proves an event is selected by the union, never by the member whose invariant motivated it. Closing that needs a way to enumerate members, which `Boundary` does not have and which would be a change to a sealed trait.
- **D-1 / P-3 — `tags()` being total over a fallible `Tags`.** Untouched here. `.kb/open-questions/d-1-the-validated-type-has-no-total-path.md` stays open and stays open-question-shaped; P-3's doc remedy overlaps this brief's Option B in the same eight examples but answers a different question.
- **Y-2 and `assert_domain_event`'s remit.** This brief endorses the scope-aware sibling and deliberately does not schedule it, precisely so Y-2 decides what the helper surface is *for* before a second function is added to it. A decider who bundles `C-additive` anyway should expect Y-2 to inherit a two-function surface and be asked what unifies it.
- **Whether the typed layer should have a clause space at all.** `spec/SPECIFICATION.md:102-107` puts `happenstance` outside it, so every question of this shape has a decision record and no clause, no maturity marker, and no `spec-trace` coverage. That is a standing condition this decision inherits and does not change.
- **Nothing is superseded.** `kb-decision-0020` stays exactly as it is. This is a new record with 0020 as its parent, taking a number from `RUNBOOK.md:262`'s queue.

---

## Revision record

**Revision 3 — 2026-09-03.** A premise audit against the tree at `HEAD` **falsified the fact revision 2's flip rested on**. Revision 2 is therefore reversed in substance, and every sentence built on the falsified premise is removed rather than rewritten around.

**What was falsified, and by what evidence.** Revision 2 claimed that Option A's check *"passes vacuously on an empty decision"*, and treated that as a silent hole: an under-tagged model folds empty, decides empty, the loop runs zero times, and the commit "reports success". The last step is false. `commit_with` has no empty guard, so `&[]` reaches `append` (`crates/happenstance/src/command.rs:342-343`, `:311`); `crates/happenstance-core/src/store.rs:251-253` requires `AppendError::NoEvents` and requires it *before* the condition check; `crates/happenstance-core/src/memory.rs:360-362` returns it above the lock; and `spec/SPECIFICATION.md:3542-3548` is **ES-20**, `[FROZEN]` — *"An empty batch is refused, and refused first."* The caller receives `Err(CommandError::Append(NoEvents))`, not `Ok`. Every citation here was re-read at `HEAD` for this revision.

The consequence that reverses the flip: ADR-0020's corrupting direction is a **lost update** — *"the append condition protects less than the handler assumes"* (`references/adr/0020-fold-query-agreement.md:66`) — and a lost update requires an append to land. ES-20 guarantees the empty-decision class writes nothing. So the class Option A's loop cannot see is precisely the class that cannot corrupt, and every case that *can* corrupt has a non-empty decision, where Option A fires.

**What was removed, and why — not rewritten around:**

1. The *ordering fact* section's claim that a run-time check "passes vacuously on exactly the handlers whose empty fold is the symptom — today, unconditionally". **Removed as false.** Replaced by *What an empty decision does today*, which states the ES-20 outcome and separates the two things the empty-decision class actually loses: not the write (nothing is written), but the *diagnostic* (the error says "an append must contain at least one event" and never names tag coverage, `error.rs:222-224`).
2. The italic paragraph in that section retracting revision 1's Y-1 attribution. **Removed** — it retracted a true-ish claim on a false analysis, and the whole paragraph is now moot because the vacuity it was arguing about does not exist.
3. Option A's first *What it does not cover* bullet, *"An empty decision passes vacuously."* **Removed as false.** Option A's coverage block now says what it does cover (every append that lands) and what it genuinely does not: late detection rather than no detection, and the composite **union** grain (`crates/happenstance-core/src/query.rs:224` — `Items` matching is `any`), which is the real residual and was previously buried under a false one.
4. The *"This recommendation flipped"* paragraph in the Recommendation. **Removed**, together with the flip it announced.
5. Revision-record item 1 of revision 2 — the deletion of recommendation reason 1, *"It is the only option that closes the corrupting direction ADR-0020 named."* **The deletion is reversed and the reason is restored**, because it is true as stated: Option A plus ES-20 between them prevent any write produced by an under-tagged model from landing, and no other option touches the run-time path.
6. Revision-record item 3 of revision 2 — the deletion of the Y-1 sentence in *Cost of delay* on the ground that "no ordering of the two changes the outcome". **Removed as backwards.** The coupling is a **correctness** coupling: today's loud `NoEvents` is the only thing keeping the empty-decision class from being silent, and Y-1 is the decision that removes it. Y-1's own brief has since landed on **Option 3, two success shapes** — a `#[must_use]` two-armed outcome, loud at compile time — which resolves the silent-`Ok` risk but not the coupling, because Y-1's own analysis concedes the arm does not distinguish a deliberate no-op from a bug. That is now stated in *Cost of delay*.

Revision 2's remaining removal — item 2, which deleted *"deferring A costs the option itself"* as a reason **for** A — is also reversed, on its own ground rather than on the falsified one. The observation that only A has a clock is true; revision 2 turned it against A by pairing it with *"a check that is partial by construction"*, and that pairing is what the audit removes. With the partiality claim gone, the clock argues for A, and it is restored as recommendation reason 3.

**Semver corrections.** Revision 2 scored `C-additive` as **Semver: None**. That is wrong: it proposes a new `pub fn` in `happenstance::testing`, and `pub mod testing` is in the crate's published default surface (`crates/happenstance/src/lib.rs:212-214`, gated on `memory` + `json`, both in `default = ["std", "memory", "json"]`). It is **additive (minor)**. Option B alone — doc bodies and doctest bodies — is genuinely **none**. Corrected in the Option C section, in *Cost of delay*, and in the Recommendation's headline, which now reads *behaviour-breaking (A), none (B)*, and notes that a bundled `C-additive` would be additive (minor) rather than none.

**Where revision 3 lands, and why it is not a reflex.** **Option A plus Option B**, with `C-additive` endorsed and sequenced after Y-1/Y-2. Re-derived rather than restored: on the corrected facts A is the only option that binds every program, it is the only item with a clock (`Cargo.toml:6-15` — free only while `0.2.0-alpha.1` is a pre-release), and `C-additive`'s two genuine advantages over it (authoring-time detection, and the per-member grain A cannot reach on a composite) are both permanently available at a minor and so lose nothing by waiting for the question that governs the helper surface. Revision 2's own strongest reason for `C-additive` — enum-totality — survives, but demoted: it buys *when* a violation is found, not *whether* a bad write can land. Confidence is **medium-high** on A + B and **medium** on holding `C-additive` back; the composite union grain is a real hole in A and is stated as the strongest argument against, with an explicit discriminator for a human who finds the two close.

**Unchanged from revisions 1 and 2, re-verified at `HEAD` for this revision:** the whole *What is true today* evidence base; the eight rendered `fn tags` impls and their two-violating / six-vacuous split; ADR-0020's `accepted`/immutable status and the citations drawn from it; Option D's rejection; and the finding that `contains_all` has exactly one production call site (`query.rs:114`) and nothing in the workspace relates a `tags()` to a `scope()`.

---

**Revision 2 — 2026-09-03.** *(Superseded in substance by revision 3; kept because the reasoning it removed had to be restored by name.)* Two critiques rejected revision 1's recommendation of Option A. Both were verified against the source before this edit; both hold.

**The falsified premise.** **[Itself falsified — revision 3. The paragraph below is retained as the record of what revision 2 believed; every claim in it that an empty decision "passes" or "reports success" is wrong, because ES-20 makes it `Err(NoEvents)`.]** Revision 1 credited Option A with closing the corrupting direction ADR-0020 named. It cannot. The check A proposes can only iterate the events this invocation decided — `decided` at `command.rs:305`, and `encode`'s `for event in decided` at `command.rs:344` — so an empty decision passes it vacuously. Revision 1's own *ordering fact* section already stated that an under-tagged model in the idempotent-no-op class **decides empty**; it then attributed the resulting hole to Y-1's empty-decision short-circuit landing first, and offered "land A first" as the mitigation. The hole is unconditional today, so the mitigation was empty and the attribution was wrong.

**What was removed, not rewritten:**

1. Recommendation reason 1, *"It is the only option that closes the corrupting direction ADR-0020 named."* Deleted. It is false as stated, and A closes that direction only for non-empty decisions, which excludes the class the brief names as the symptom.
2. Recommendation reason 3's scoring, *"deferring them costs nothing and deferring A costs the option itself."* Deleted as a reason **for** A. Only A needs the breaking window; B and `C-additive` need none, so the same fact now argues against spending the one free window on a check that is partial by construction. The window's asymmetry survives as a caveat under the strongest-argument-against, because it is a real fact a decider may still weigh.
3. The Y-1 mitigation sentence in *Cost of delay*. Deleted, with the correction stated in place.

**What flipped.** **[Reversed by revision 3; `C-additive` is also mis-scored below — it is additive (minor), not none.]** The recommendation is now **Option B plus `C-additive`** — the enum-total `assert_domain_event` sibling — with the pre-release breaking window left unspent for D-1. What flipped it: coverage is a per-variant, per-model quantified property, and revision 1's own counter-argument conceded that framing while recommending a run-time loop that cannot quantify over either. The matching check is enum-total, additive, needs no window, and is the precedent ADR-0020 set for its own residual.

**Also changed by the flip:** Option A gained a *What it does not cover* block stating the vacuity; Option C was split into `C-additive` and `C-refusal` because they score differently; the *ordering fact* section now attributes the vacuity correctly; and two *What this does not settle* bullets (the projection port, Y-2) were corrected where they asserted a consequence of the old recommendation.

**Unchanged and still true:** the evidence sections (*What is true today*), the doctest count, Option D's rejection, and the finding that nothing in the workspace relates a `tags()` to a `scope()`. No critique touched them.
