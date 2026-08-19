# ADR-0020: A decision model folds a domain enum, and its query is derived on a sealed trait the caller cannot override

**Status:** accepted · **Phase:** 7 · **Date:** 2026-08-15 ·
**Reversibility:** medium · **Amends:** nothing · **Supersedes:** nothing

The question this record was minted to answer is `RUNBOOK.md:300`'s, in the plan's
own words:

> **0020** | 7 | How does a decision model guarantee that its query and its fold
> cannot disagree?

The answer, in one sentence: **it does not guarantee it — the shape removes the
place where a disagreement could be written.** There is no hand-maintained query,
because there is nowhere to put one.

---

## Context

### The hazard, in this tree, today

A DCB handler names its event set **twice**. Once in the query it reads with, and
once in the fold that interprets what came back. Nothing checks the two agree.

That is not a hypothetical. `examples/course-subscriptions/src/main.rs:114-125`
builds the two-item query for the canonical subscribe case:

```rust
let query = Query::from_items([
    // The capacity, and everyone currently holding a seat.
    QueryItem::new(
        [COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED],
        Tags::from_pairs([("course", course)])?,
    )?,
    // This student's history with this course.
    QueryItem::new(
        [STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED],
        Tags::from_pairs([("course", course), ("student", student)])?,
    )?,
])?;
```

Thirty lines below it, at `:140-155`, the fold names the same three types a second
time:

```rust
match sequenced.event_type().as_str() {
    COURSE_DEFINED => capacity = Some(parse_capacity(sequenced.event.data())),
    STUDENT_SUBSCRIBED => { seats_taken += 1; /* … */ }
    STUDENT_UNSUBSCRIBED => { seats_taken -= 1; /* … */ }
    _ => {}
}
```

The two lists agree. Nothing in the workspace makes them agree. `cargo xtask ci`
is green if a fourth event type is added to the query and not to the fold, and it
is green if the reverse happens. The `_ => {}` arm is what absorbs the difference
silently — and it is not removable while the fold matches on a `&str`, because
`sequenced.event_type()` can carry any string a store returns.

The two failure directions are not symmetric, and both are real:

| Divergence | What the compiler says | What it costs at run time |
| --- | --- | --- |
| Query selects a type the fold ignores | nothing — `_ => {}` eats it | the append condition covers events the decision never saw. The boundary is **wider** than the decision, so writes are rejected for facts the handler did not consider — a spurious conflict, which is annoying but safe |
| Fold interprets a type the query never selected | nothing — the arm is simply never taken | the arm is dead, and the decision is made on a **narrower** log than it believes. The append condition protects less than the handler assumes. This is the one that corrupts |

**This is the defect ADR-0020 exists to make unwritable.** It is stated as two
project design rules already:

- **DR-01** — *"A domain enum, not a `SequencedEvent`, is what a decision model
  folds over, so the `match` is exhaustive and divergence between the query and
  the fold is a compile error rather than a silent DCB failure"*
  (`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md:142`).
- **DR-02** — *"A decision model's `Query` is *derived* from its `EVENT_TYPES` and
  tag constraints and is never hand-maintained"* (`project.md:143`).

DR-01 closes the fold half by making the domain a Rust `enum`: an exhaustive
`match` over an enum is checked by the compiler, so a new variant is a build
failure rather than a `_ => {}`. This record owns the **other** half — the query —
and the whole of the question is *where do you put a derivation such that a caller
cannot route around it*.

### The constructors this decision is built out of, and cannot change

`happenstance-core` is frozen (project `AC-A02`). Four of its constructors set the
shape of every answer available here, and each was re-read at the line cited:

| Item | Where | What it forces |
| --- | --- | --- |
| `QueryItem::new` | `crates/happenstance-core/src/query.rs:56` | returns `Result<Self, InvalidQuery>`. There is **no** infallible constructor, not even for values that were validated on the way in |
| `InvalidQuery::UnconstrainedItem` | `crates/happenstance-core/src/error.rs:99-102` | the error a `QueryItem` with neither types nor tags produces, raised at `query.rs:68-70` |
| `Tags::from_pairs` | `crates/happenstance-core/src/tag.rs:304` | returns `Result<Self, InvalidTag>`, and is the only route from `&str` pairs into a `Tags` |
| `EventType::from_static` | `crates/happenstance-core/src/event.rs:108` | is `const`, so an event-type list can be built at compile time — and an invalid one is a compile error at a free `const` site (`event.rs:95-100`) |

Read together they say something precise: **the event-type half of a derived query
can be built at compile time and the tag half cannot.** Everything below follows
from that asymmetry.

### The audience constraint, which is not decoration

The repository owner and the readership are fluent in event sourcing, DCB,
consistency boundaries and concurrency control, and **new to idiomatic Rust**
(`CLAUDE.md`, *Who you are working with*). So a construct that is free for a Rust
expert — a sealed trait, a blanket impl, a per-monomorphisation `const` — is a
first-hour cost here, and every one of them below is justified by *what the
alternative was and why it lost* rather than by its name.

That constraint is what gives **DT-2** its weight: *how much must a caller state
before their consistency boundary is checked?* Its two poles are minimal ceremony
with late errors, and explicit declaration with early errors. `_design.md`'s
`## Shape decision` resolved it toward **explicit declaration** and a human
approved that resolution on 2026-08-12, naming it as one of three things they were
specifically being asked to accept (`_design.md` `## Sign-off`). **This record does
not re-decide it.** It writes down what was decided, what lost, and what it costs,
because an accepted atom is immutable and a record written after the code records
what was built instead of deciding it.

### The concrete form of the question

`_decomposition.md:526-546` states DT-2's technical form exactly:

> **is `DecisionModel::query()` infallible, and if so, where did the validation go?**

and then insists on one property of the answer, which is the property this record
is most at risk of violating: **the answer must be one shape, not two.** The
contract crate already condemns the failure mode in its own prose, about its own
`ProjectionId`:

> *"Adding a fallible `parse` beside this constructor would be worse than either
> choice: two constructors enforcing different rules is the defect that makes an
> invalid value reachable through the weaker one."*
> — `crates/happenstance-core/src/projection.rs:152-154`

A record that admits both an infallible and a fallible path has not resolved DT-2.
It has deferred it into the type system, where the weaker path wins by being
easier to reach.

---

## Decision

Five commitments. They are one shape; none of them is optional, and none of them
admits a second spelling.

### 1. `query()` is not on `DecisionModel`. The derivation is `Boundary::query`, on a sealed trait

```rust
pub trait DecisionModel: Clone {
    type Event: DomainEvent;
    fn scope(&self) -> &Tags;
    fn apply(&mut self, event: Self::Event);
}

/// What the command loop consumes. Sealed: blanket-implemented for every
/// `DecisionModel` and macro-implemented for tuples of arity 2..=8.
pub trait Boundary: sealed::Sealed {
    fn query(&self) -> Result<Query, InvalidQuery>;
    fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C)
        -> Result<(), CodecError>;
}
```

The derivation lives on a **separate trait**, blanket-implemented for every
`DecisionModel`, and that trait is **sealed** — its supertrait is a private
`sealed::Sealed` that no downstream crate can name, so no downstream crate can
write an `impl Boundary for MyModel`.

**Why not simply a provided method on `DecisionModel`?** Because a provided method
is a *default*, and a default exists to be overridden. The moment a caller writes
their own `fn query`, the query is hand-maintained — which is precisely DR-02's
prohibition, reintroduced by the mechanism that was supposed to enforce it. The
house rule this follows is RS-40-2, *"put a derivable convenience on the blanket
ext trait"* (`standards/rust/40-public-surface-and-evolution.md:73`), and its
stated reason is the mechanical one: `impl<S: Trait + ?Sized> Ext for S {}` covers
every type at once, so a second impl for one type is `error[E0119]` — *"the
provided body is not a default, it is the **only** body"* (`:75-79`).

For a reader new to Rust: the sealing and the blanket impl are doing two different
jobs, and both are needed. The **blanket impl** means every `DecisionModel` gets
`query()` without writing anything. The **seal** means nobody can supply a rival
one. Take either away and DR-02 becomes a convention rather than a structure.

### 2. `Boundary::query` returns `Result<Query, InvalidQuery>`, and an *empty* `EVENT_TYPES` is a compile error

Not a run-time error. A `const` item, evaluated once per monomorphisation, carries
the claim *"a `DomainEvent` must declare at least one event type"*, so a
`DomainEvent` with `EVENT_TYPES: &'static [EventType] = &[]` fails to build. The
mechanism is RS-61-4, *"put the claim in a `const` item, so the compiler is the
thing that checks it"* (`standards/rust/61-compile-time-assertions.md:249`), whose
stated reason is that *"a generic function's obligations are discharged when it is
**instantiated**"* — a `const _: () = …;` item instantiates without calling, which
is what makes the check fire for a type the program never actually runs against.

The reader is meant to see a **post-monomorphisation const-eval error naming the
event-type set**, not a trait-resolution error, and the paired `compile_fail`
doctest on `DomainEvent`'s item page is spelled bare `compile_fail` and never
`compile_fail,E0080` — rustdoc on 1.97.1 silently ignores an error code it cannot
match, so the stricter-looking spelling is the weaker check. The contract crate
already says exactly this about the same construct
(`crates/happenstance-core/src/event.rs:102-106`).

### 3. `DecisionModel::scope(&self) -> &Tags` — the model *holds* validated tags

Returning a borrow of a `Tags` the model already owns is what pays the validation
**once**, in the constructor the caller was already writing:

```rust
let seats = Seats {
    scope: Tags::from_pairs([("course", "c1")])?,   // the only `?` the scope costs
    capacity: None,
    taken: 0,
};
```

`Tags::from_pairs` is fallible and is the only way in (`tag.rs:304`). So a `Tags`
that exists is a `Tags` that is valid, and `scope()` cannot fail because there is
nothing left to check.

### 4. `DecisionModel: Clone`, not `Default`

`Clone` is what lets the command loop re-fold from the **pristine** model on
retry: a conditional append that loses the race must re-read and re-decide, and
re-decide means starting from a model that has absorbed nothing. `Default` is the
alternative that looks equivalent and is not — it would require `scope`'s
validated `Tags` to be default-constructible, which is a second, weaker way to
obtain a `Tags`, and that is `projection.rs:152-154`'s defect exactly.

### 5. Composition is `impl Boundary for (B1, B2)`… by an *internal* `macro_rules!`, arity 2..=8

A caller composes two consistency boundaries by writing a tuple. There is no
exported macro and no caller-visible syntax at all, because a macro in the
caller's face is precisely the ceremony DT-2 is being measured on. The macro is
internal, `$crate`-qualified and most-literal-arm-first (RS-41-1 / RS-41-3), and
the arity ceiling is **8** because rustdoc renders one impl block per arity and
sixteen of them buries the page.

---

## Where the fallibility went, and the residual

`Boundary::query` builds a `QueryItem` from values that are **already** validated —
`EVENT_TYPES` is `const`-constructed and `Tags` has no infallible constructor —
and it still meets a `Result`. This is the single most likely thing for an
implementer to "fix" wrongly, so the disposition is written down in full.

**1. `Err` is kept, and given a real meaning.** `QueryItem::new` returns
`InvalidQuery::UnconstrainedItem` when an item constrains neither types nor tags
(`query.rs:68-70`, variant at `error.rs:99-102`). Read at this altitude that says:
*this boundary constrains nothing, which is `Query::all()` and must be said out
loud.* Reading every event in the store is a legitimate thing to want and an
illegitimate thing to arrive at by accident, so the refusal is worth making.

**2. The other route to that `Err` becomes a compile error.** An empty
`EVENT_TYPES` on a model with an empty `scope()` is the other way to reach
`UnconstrainedItem`, and it is decidable at compile time. Decision 2 decides it
there. Leaving it to the first read would be DT-2's minimal-ceremony pole wearing
the explicit-declaration pole's clothes: the ceremony is paid *and* the error
still arrives late.

**3. `commit`/`commit_with` absorb the `Result`** into `CommandError::Boundary`,
so the first program writes no extra `?` for a derivation it did not ask for.

**4. No `unwrap`. No edit to `happenstance-core`.** `unwrap`/`expect` in library
code is on the standing anti-pattern list (`_design.md` `## Anti-patterns`), and
editing the frozen crate is what `AC-A02` forbids.

### Defect candidate D-1

> **`happenstance-core` has no infallible `QueryItem` constructor for pre-validated
> inputs; every derived query therefore carries a `Result` that is unreachable for
> well-formed models.**

Its nearest clause subject is **VT-18** — *"Constructors accept values the caller
already holds, and their errors compose"* (`spec/SPECIFICATION.md:1371-1375`). The
clause's first half is discharged for `Event::new`; the same courtesy does not
exist for `QueryItem::new`, which is why a caller holding a `const EventType` and a
validated `Tags` still gets a `Result` back.

D-1 is **named here as context and routed, not repaired.** It goes into AC-012's
defect log with its clause ID and reaches the contract, if it ever does, through a
decision record — never a line edit of a frozen clause (project `AC-012`,
`AC-A02`). Three "fixes" are refused by name, because each of them is the obvious
first move:

- adding an `unwrap` — forbidden by the standing anti-pattern set;
- adding an infallible `QueryItem` constructor to `happenstance-core` — an
  unrecorded change to a frozen contract, and a second constructor enforcing
  different rules, which is the defect `projection.rs:152-154` names;
- editing VT-18 or any other `[FROZEN]` clause — an amendment filed as a repair,
  which is how a frozen commitment quietly moves (`.kb/decisions/README.md`, *The
  immutability rule*).

---

## Alternatives rejected

Each row names the **wrong implementation it admits** — the thing it lets someone
write, not merely the thing it fails to prevent. A decision recorded without its
rejected options is indistinguishable from an accident.

| Rejected | The wrong implementation it admits |
| --- | --- |
| **A provided method `query()` on `DecisionModel`** | A caller writes their own `fn query`, and the "derived" query is hand-maintained again. DR-02 becomes a convention. The failure is silent: the override compiles, the tests pass, and the boundary is whatever the override says |
| **A free function `derive_query::<M>()`** | The caller simply does not call it. Nothing requires the query passed to `read_decision_model` to be the one the model implies, so a hand-built query and a derived model coexist with no signal |
| **An infallible `query() -> Query`** | Unreachable without an `unwrap` inside library code, because `QueryItem::new` is fallible (`query.rs:56`) and `happenstance-core` is frozen. The `unwrap` is a panic in someone else's process, on a path the type system said could not fail |
| **Fallible `query()` with no `const` assertion** | An empty `EVENT_TYPES` compiles, and the failure arrives at the first read as `UnconstrainedItem` — a compile-time-decidable mistake deferred to run time, which is DT-2's minimal-ceremony pole while still charging explicit declaration's ceremony |
| **`scope(&self) -> Tags` by value** | The model holds `&str` pairs and builds the `Tags` on each call, so a fallible construction sits inside an infallible signature and forces an `unwrap` — the same panic as above, one method along |
| **`scope(&self) -> &[(&str, &str)]`** | Revalidates on every read and moves the error to the read. It also makes an invalid scope constructible, which is the hole `Tags`'s single fallible constructor exists to close |
| **`DecisionModel: Default`** | Forces `scope`'s validated `Tags` into a default-constructible field, re-opening "how do I get an invalid `Tags`". A second, weaker way to obtain a validated value is `projection.rs:152-154`'s defect verbatim |
| **An exported `compose!` macro for composition** | The caller writes a macro invocation where a tuple would do. It is caller-visible ceremony in the exact place DT-2 is being measured, and it puts a second syntax beside the one the tuple impls already provide |

**Exactly one path is licensed.** There is no infallible sibling of
`Boundary::query`, no second way to obtain a `Tags` for `scope`, and no second
composition syntax. That is the whole of the one-shape claim, and it is made in
the contract crate's own words because the repository already condemns the failure
mode it prevents.

---

## Consequences

**The divergence becomes unwritable rather than detected.** There is no place to
write a query that disagrees with a fold, because the query is not written. That
is what makes this structural rather than a lint: a lint can be silenced and a
review can be skipped, but a trait with no overridable body and no rival impl has
nothing to silence.

**It costs ceremony, and the number is on the record.** In the signed-off first
program (`_design.md` `## The doctest`), domain logic — the enum, the fold's two
arms, the decision — is **11 lines**, and mapping ceremony — `EVENT_TYPES`,
`event_type`, `tags`, `encode`, `decode` — is **26 lines**. That is **2.4:1 against
the domain for a two-variant enum**, and it worsens with a third
(`_design.md:1103-1109`).

**The prediction this decision carries, as a consequence and not as a second
decision:** *AC-013's verdict will land "`happenstance-macros` is in scope for
0.1"*. It is falsifiable, it must be checked against the **rewritten worked
example** rather than against the doctest, and it is the project closeout's to
record either way (`RUNBOOK.md:525` — *"Is `happenstance-macros` in scope for 0.1 |
7 | open — the criterion is stated in phase 7 and evaluated in its session log"*).
This record does not decide the derive's fate on its behalf, and states no *must*
about it.

**A price the shape does not pay for.** `EVENT_TYPES` ↔ `event_type()` agreement
is **not** compiler-enforced — a hand-written `event_type()` may return a type
absent from `EVENT_TYPES` and no `const` sees the match arms. That residual is
tested rather than proven, by `assert_domain_event::<E>(&[…every variant…])`, and
it is the second input to AC-013's measurement.

**Downstream.** `domain-event-and-decision-model` and `decision-model-composition`
build this shape; `compile-fail-proof-artefact` observes the empty-`EVENT_TYPES`
refusal; initiative DoD 2 (*"@smoke — the compiler protects the domain"*) is what
the whole chain is for.

**Reversibility: medium.** The shape is a public API of a crate that will be
published at `0.2.0-alpha.1`, so reversing it after publication is a breaking
change to `happenstance` — but only to `happenstance`. No stored data encodes it,
no adapter observes it, and `happenstance-core` does not move. Reversal is a
superseding atom plus a major version, not a migration.

---

## What this decision does not decide

- **ADR-0021's subject matter** — payload evolution, a version suffix on
  `EventType`, upcasting, and the codec tag's home. Staged in the same ingest wave;
  the public surface here is invariant under every answer it takes.
- **Whether `happenstance-macros` ships.** A prediction, above, is not a decision.
- **D-1's resolution.** Logged and routed; the contract is not touched.
- **The `trybuild` dependency**, which is AC-002's only instrument and is HS-P0010's
  to settle. The `compile_fail` doctest above is *not* AC-002's instrument.
- **Anything below the port.** `DecisionModel`, `Boundary` and the derived `Query`
  live entirely in `happenstance`. No store can observe whether a query was derived
  or hand-written, so there is no conformance rule here that any adapter could fail,
  and a rule no adapter can fail is decorative (`CLAUDE.md`, *The rule that
  matters*).

---

## Citations repaired before this record was written

Per `.kb/decisions/README.md` and this story's EC-008, a citation that does not say
what it is claimed to say blocks sign-off. Two were re-opened and corrected here,
and both are recorded rather than quietly fixed:

1. **`crates/happenstance-core/src/projection.rs:47-61`** is cited by both
   `_decomposition.md:544-546` and `_design.md`'s `## Shape decision` as the source
   of the two-constructor sentence. That range is the module's *"The invariant that
   drives the design"* prose about read-model/checkpoint atomicity. The sentence is
   at **`crates/happenstance-core/src/projection.rs:152-154`**, in `ProjectionId::new`'s
   doc comment, and that is the citation this record uses.
2. **`RUNBOOK.md:524`** is cited for AC-013's falsifiable prediction. `:524` is the
   testkit-instrument row for a store holding only a suffix of its own log. The
   `happenstance-macros` row is at **`RUNBOOK.md:525`**.

Both originals were re-read at the ranges given. Neither correction changes any
decision — the sentences say what was claimed, at different lines — so both are
**repairs** and not amendments, by the mechanical test in
`.kb/decisions/README.md`: the set of implementations admitted is unchanged.

---

## Provenance

Decided at the `/redkiln:plan` design sign-off gate on 2026-08-12 (approved by the
repository owner, no conditions) and recorded here at phase 7, **before** the code
it governs exists — which is project AC-016's whole content. The design is the
input; this record is the durable artefact. Where this record and
`spec/SPECIFICATION.md` ever disagree, the specification wins: an ADR is history,
and is never updated to match the code.
