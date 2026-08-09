# Exploration: composing a Crux application that does not collapse under its own weight

Status: **exploration notes**. Companion to `research-crux-integration.md`. Nothing here is normative.
Date: 2026-08-07.
Grounded against: `crux_core` 0.20.0 source and book; `examples/weather` measured; Proton's
`ProtonMail/proton-rust-nation-2026` demo and the Rust Nation talk; and the accumulated record from
Elm, TCA, Redux, Elmish, Iced, Bloc, Yew and Dioxus.

---

## 0. Thesis

Your instinct is right on both counts. A single Crux `App` does become unwieldy — but **not for the
reason people assume**, and the framework's documented answer makes it worse rather than better.

The documented answer is nested state machines: the parent owns a match arm per child, lifts the
child's events with `map_event`, lifts its effects with `map_effect`, and interprets a `Transition`
type on the way back up. Measured on `examples/weather` — a **seven-screen toy** — that is 25–38
lines per delegation arm, ~40–60 lines per parent/child pair, eleven `Event` enums, nine
`Transition` enums, five levels of nesting, boxed variants with hand-written constructors to hide
the boxing, and one `#[allow(clippy::too_many_lines)]`. The `Outcome`/`Status`/`Started` protocol
carrying all of it is **205 lines of example code you copy into your project**; it is not in
`crux_core`. The book never mentions the cost.

Every Elm-shaped ecosystem has arrived at the same place and then spent years arguing about it. The
common root is not the framework. It is this: **children share nothing, so the parent must mediate
everything.** TCA's action bubbling, Elmish's interception, Halloy's eight `&mut` parameters,
cosmic-settings' `Vec<u8>` escape hatch, Bloc's seven-year unresolved argument — all of it is the
cost of siblings with no common substrate.

**You have a common substrate. It is the log.** And DCB is what makes it a *cheap* one, because two
slices that never share a tag never share a consistency boundary and never need to coordinate at
all. The composition story at the code level and the consistency story at the data level turn out to
be the same story.

So the design below has one governing move — **stop composing state machines and start composing
around the log** — plus one mechanical trick that removes almost all remaining wiring: make a slice
*generic over* the app's `Event` and `Effect` types rather than owning its own and being mapped.

---

## 1. What Crux actually gives you

Two methods. That is the entire composition surface of `crux_core` 0.20.0, under a source comment
reading `// Mapping for composition`:

```rust
pub fn map_effect<F, NewEffect>(self, map: F) -> Command<NewEffect, Event>
where F: Fn(Effect) -> NewEffect + Send + Sync + 'static, ...;

pub fn map_event<F, NewEvent>(self, map: F) -> Command<Effect, NewEvent>
where F: Fn(Event) -> NewEvent + Send + Sync + 'static, ...;
```

There is no `App` combinator, no sub-app registry, and nothing in the newer work that helps.
`effects::EffectRouter` (0.19+) is scoped out of app composition **by explicit design decision** —
its RFC states the goal as "keep `App` implementations unaware of handling mechanics."
`middleware::Layer` is being deprecated. `docs/src/guide/composing.md` existed as a stub reading
`TO DO.` from November 2022 until it was deleted in November 2025.

Two things worth knowing about the documented pattern beyond its size:

- The book's nested-state-machines chapter **never uses `map_event`/`map_effect` by name** —
  `grep "Command::map_event" docs/` returns nothing. The documented pattern and the framework's own
  composition primitives are taught separately.
- The single place the repo admits app composition is costly is the *Command RFC*, describing the
  design it replaced: *"Any transformations involved in making an app a 'child' of another app has
  to be done up front… typically with a `From` implementation which can't make contextual
  decisions."* The RFC declares the problem solved by `map_effect`/`map_event`. There has been no
  follow-up. (§6 argues the RFC dismissed the right answer.)

---

## 2. What Proton actually did — the record is better than the folklore

Worth correcting, because the talk is the main artefact people cite and the demo shows something
simpler than "micro-frontends" implies.

**One `Core`, not one per frontend.** From the talk: *"we still have one core here because the core
is the engine that runs the show and we want just one of those."* Features are child crates
implementing `App`, pulled in and composed — *"that lumo app is exactly the same Lumo app that
powers the standalone Lumo top level app. So we're just like pulling in the crate."*

**And the composition is flat delegation, not the book's protocol.** The entire parent `update`:

```rust
fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
    match event {
        Event::Initialize => Command::event(Event::LumoFeature(LumoEvent::Configure { .. })),
        Event::ArticlesFeature(e) => Articles.update(e, &mut model.articles_feature)
            .map_event(Event::ArticlesFeature)
            .map_effect(|effect| match effect {
                ArticlesEffect::Render(r) => Effect::Render(r),
                ArticlesEffect::Http(r)   => Effect::Http(r),
            }),
        Event::LumoFeature(e) => Lumo.update(e, &mut model.lumo_feature)
            .map_event(Event::LumoFeature)
            .map_effect(|effect| match effect { .. }),
    }
}
```

No `Outcome`, no `Status`, no `Transition`, no `mem::take`. Children signal nothing upward except
through the shared `Model`. **This is roughly ten times cheaper than the documented pattern**, and it
is what a production team actually shipped.

Three findings from their material that shape the design below:

1. **Ludovico's constraint on effects is the real load-bearing rule.** *"When we compose features the
   top level app needs to be able to resolve the effects that come from all the different composed
   features… the more these effects will look like low-level IO primitives — here's an HTTP request,
   here's a database request — the more we increase the chances that it will be common to more
   features."* The parent `Effect` is the union of every child's; the way to keep the union small is
   for effects to be **infrastructure, never domain**.
2. **There is no shared auth/session/navigation story.** The talk's abstract advertises it; neither
   the talk body nor the repo demonstrates it. `Model` is two disjoint child models with no shared
   fields. Navigation lives entirely on the shell.
3. **A tension nobody has written down.** The weather example threads shared context (`&ApiKey`) as
   an extra argument to every child `update` — possible only because those children are plain
   methods. **Once a child is a real `App`, `App::update`'s fixed signature takes that escape hatch
   away.** This is a direct argument against making slices `App` impls, and §7 uses it.

Also worth knowing: their shell-side `Feature`/`FeatureCore`/`FeatureCoreMap` lens abstraction
(Swift and Kotlin) is genuinely good and solves a real problem — a feature's binary payload differs
depending on how deeply it is nested, so the shell needs the mirror of `map_event`/`map_effect`.
Their typegen across N packages is hand-maintained relative paths per language per namespace, and
their own demo contains a visible copy-paste error in one of them. That is the ergonomic reality of
N-way typegen today.

---

## 3. What five other ecosystems already paid for

Compressed to the transferable claims. Each is sourced in the research behind these notes.

| Ecosystem | What they learned | Transfers as |
|---|---|---|
| **Elm** | Feldman, 96k LOC: *"thinking in terms of 'parent', 'child', and 'component' … leads to a worse and worse experience as the code base scales. In contrast, thinking only in terms of 'caller' tends to yield better results."* `elm-spa-example` nests at **exactly one level** — the page boundary — and Feldman's own header comment calls that layer accidental. | Split once, at the top. Below that, functions with narrow signatures. **Narrowing the argument type substitutes for splitting the state machine.** |
| **TCA** | The most developed answer, and its bill: compile-time pathology from overload explosion; ~6 ms per *no-op* action in a large app; a measured **10× win from cutting one subtree out of the global store** (210k events/9.61 s → 17k/0.91 s). Every serious shop independently reinvented `case delegate(Delegate)` plus lint rules, because *"Enums in Swift don't have any access control."* | Reified actions buy debuggability and cost encapsulation — a genuine dichotomy, not a bug. **Rust module privacy is the thing Swift lacked; spend it.** And never assume one root store. |
| **Redux** | The three Style Guide rules are one argument: model actions as *events not setters*; allow *many reducers to respond to one action*; therefore *never dispatch sequentially*. Sibling communication is `extraReducers` — a slice listening to another slice's action **with no parent, no routing table, no registration**. | **Broadcast beats routing.** And `addCase` is a runtime string match where a Rust `match` on a foreign variant is compiler-checked — adding a variant *forces* every listener to be revisited. That is the single largest correctness advantage available here. |
| **Bloc** | Spent three years teaching direct bloc-to-bloc subscription, then forbade it, never justified the reversal, and never lint-enforced it. The answer that survived: *"Two blocs can listen to a stream from a repository and update their states independent of each other."* Write to the shared store; let both readers re-derive. | **This is exactly the architecture below** — and happenstance is a far stronger reactive repository than anything in that thread. Their unresolved half (shared *logic*, not shared *state*) is answered by composed `Decider`s. |
| **Iced / Elmish** | Iced deprecated its `Component` for *"encapsulated state… hampers single source of truth"*, shipped no replacement, and its ecosystem reconverged on `update(&mut self, …deps) -> Action` within a year. Its best argument: a child with **30 messages** needs an upward channel with **2 variants** — the parent should see 2, not 30. Elmish's interception idiom is spelled as a match-with-wildcard, *"and the catch-all is precisely what destroys exhaustiveness — the compiler you were recruiting is disarmed by the very idiom used to intercept."* | If you keep a parent/child channel at all, make it a **first-class, exhaustively-matched, narrow type**. §8 gets this property differently and more cheaply. |

One more, from Dioxus, because it inverts the usual instinct: *"Notice how we started by splitting
our UI **first** and **then** state. It's generally better to centralize our state primitives and
pass down derived values where possible."* Nested TEA does the opposite — splitting a component
*forces* you to split the model and the message enum with it.

---

## 4. The hinge: siblings that share a log do not need a parent

Every mediation cost in §3 exists because sibling A and sibling B have no common ground, so all
communication travels up to a parent and back down. That parent then knows both children's
internals, which is the coupling you objected to — and you are right that it is coupling *created
by* the composition mechanism, not removed by it.

Here, A and B share the log. Which means:

- **A appends a fact; B's projection sees it.** No code edge between A and B at all. Not a weak
  edge — *no* edge. This is Bloc's reactive-repository answer with a much stronger contract behind
  it, and Redux's "model actions as events" with compile-checked listeners instead of string matches.
- **DCB is what makes it cheap.** With aggregates, A and B would have to agree on aggregate
  boundaries, and any invariant spanning them needs a saga. With DCB, A and B tag facts in their own
  namespaces, never conflict, and get parallel writes for free. A genuine cross-slice invariant is a
  *composed `Decider`* — a tuple whose `query()` concatenates both boundaries — not a process
  manager. That is the reduced bounded-context tax you suspected, and it is real.
- **The integration surface is the tag namespace, not a type.** Which means it is versionable,
  greppable, and does not appear in anyone's `Event` enum.

So the rule that follows, and it is the whole design in one line:

> **Cross-slice communication is a fact in the log. If it cannot be a fact, it is not domain
> communication, and it belongs to the shell or to an explicitly named shared context.**

---

## 5. Four seams, and the discipline is knowing which one a problem belongs to

| Seam | What composes | Mechanism | Cost |
|---|---|---|---|
| **1. Domain** | `Decider`s, `Fact`s, `Projection`s, `Namespace`s | plain Rust; query union | **zero** |
| **2. Slice** | one vertical feature inside one `App` | flat dispatch, generic over `Event`/`Effect` | ~4 lines per slice |
| **3. Cross-slice** | features reacting to each other | a fact in the log | zero, or one match arm |
| **4. Core** | independently owned/deployed products | separate `Core` + `Bridge`, shared store | high — org boundary only |

**Seam 1 is the largest lever and it is easy to miss.** Charypar says it himself: *"a lot of the
lower level logic and state should be unaware of Crux."* The weather example's problem is that
*everything* is in the state machine. If most of your logic lives in Crux-free domain crates —
deciders, facts, projections, all plain functions over a store — then the composition problem shrinks
to the size of the thin Crux-facing shell around it. **A Crux `App` that is mostly a dispatch table
does not become unwieldy.** It becomes unwieldy when it holds logic.

---

## 6. The slice contract: generic over `Event` and `Effect`, not mapped into them

Here is the mechanical trick, and I think it is the part worth spiking first.

The 40–60 lines per child exist to bridge **type** boundaries: the child owns `ChildEvent` and
`ChildEffect`, so every crossing needs `map_event` and `map_effect`. But vertical slices need
**code** boundaries and **state** boundaries — they do not need type boundaries. `Effect` is
infrastructure and is genuinely shared (Ludovico's rule). `Event` is the app's intent vocabulary, and
sharing it is precisely what enables Redux-style broadcast.

Crux already has the pattern for this. It is how *capabilities* stay reusable — generic over the
effect type, bounded by `From<Request<Op>>`:

```rust
pub fn get_location<Effect, Event>() -> RequestBuilder<Effect, Event, impl Future<Output = Option<Location>>>
where Effect: Send + From<Request<LocationOperation>> + 'static, Event: Send + 'static
```

Nobody applies it to a whole slice. Applied to a slice, and with one bound added, it eliminates the
mapping entirely:

```rust
// ---- in the slice crate: catalog/src/lib.rs ----

pub enum Intent { Search(String), Select(ProductId), /* … */ }

#[derive(Default)]
pub struct Model { /* slice-local state, projections */ }

pub fn update<Ef, Ev>(intent: Intent, model: &mut Model, ctx: &impl HasSession) -> Command<Ef, Ev>
where
    Ef: Send + From<Request<StoreOperation>> + From<Request<RenderOperation>> + 'static,
    Ev: Send + From<Intent> + 'static,
{
    match intent {
        Intent::Search(q) => {
            model.query = q.clone();
            store::read(catalog::search_query(&q))
                .then_send(|r| Ev::from(Intent::Loaded(r)))   // ← no map_event
        }
        // …
    }
}
```

```rust
// ---- in the app ----

#[derive(From)]                       // derive_more, or three hand-written impls
pub enum Event {
    Catalog(catalog::Intent),
    Basket(basket::Intent),
    Checkout(checkout::Intent),
}

pub struct Model { catalog: catalog::Model, basket: basket::Model, checkout: checkout::Model }

fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
    match event {
        Event::Catalog(i)  => catalog::update(i, &mut model.catalog, &model.session),
        Event::Basket(i)   => basket::update(i, &mut model.basket, &model.session),
        Event::Checkout(i) => checkout::update(i, &mut model.checkout, &model.session),
    }
}
```

**Total wiring per slice: one `From` impl (derivable), one `Model` field, one match arm.** No
`map_event`, no `map_effect`, no `Outcome`, no `Status`, no `Transition`, no `mem::take`, no
let-else guard that silently no-ops on a routing mismatch.

Note what `Ev: From<Intent>` replaces. The Command RFC dismissed `From` because *"a `From`
implementation can't make contextual decisions"* — true for lifting a *whole child app's* event type
after the fact, but the slice is constructing its own events at the point of emission, where the
context is already in hand. **The RFC rejected the right answer for the wrong shape.**

Four properties fall out that are worth having:

- **The effect requirement is a compile-checked contract.** A slice that needs the store says so in
  its bounds; the app's `#[effect]` macro generates the `From<Request<Op>>` impls automatically. Add
  a slice needing an operation the app doesn't have and you get a trait error naming it, not a
  runtime surprise.
- **A slice is testable against a *minimal* effect enum.** Instantiate the generics with a test-local
  `enum TestEffect { Store(Request<StoreOperation>) }` containing only what this slice uses. Nothing
  in the surveyed ecosystems can do this; it falls out of the bounds being requirements rather than
  a concrete type.
- **Slices are reusable across apps** — the Proton goal — without either app naming the other's
  types.
- **Monomorphisation depth is 1.** TCA's compile-time pathology comes from `Scope<Scope<Scope<…>>>`
  and a single giant inferred expression. There is no nesting here to accumulate.

Cost, stated honestly: any slice can construct any `Event`, because `Event` is one type. That is
TCA's encapsulation loss. §8 is how Rust pays it back, and Rust has what Swift didn't.

---

## 7. Shared state that isn't a fact

This is the part Proton didn't demo and the part everyone gets wrong. Navigation, selection, modal
stack, focus, scroll, draft text, session. Four answers, in order of preference:

1. **Make it a fact if it is one.** `SignedIn { subject, at }` is a fact. So, arguably, is
   `WorkspaceOpened`. Anything you would want on another device, or in an audit, or replayed, belongs
   in the log — and then §4 applies and there is no problem.
2. **Give navigation to the shell.** Proton does, and it is right: navigation is platform-idiomatic,
   deeply tied to `NavHost`/`NavigationStack`, and putting it in the core buys nothing. This is a
   real answer, not a dodge. (TCA's Tripadvisor migration went the other way *because* their
   coordinator web was unmanageable — worth knowing the counter-case exists.)
3. **Name the shared context and let each slice declare what it needs.** Halloy — the largest real
   Iced app — threads **eight** `&mut` parameters through every child `update`, and every new shared
   resource is a breaking change to every child. The fix is not fewer parameters; it is one parameter
   whose *type is a bound*:

   ```rust
   pub fn update<Ef, Ev>(intent: Intent, model: &mut Model, ctx: &(impl HasSession + HasClock))
       -> Command<Ef, Ev>
   ```

   Each slice states its own requirement, compile-checked and minimal; adding a new shared resource
   breaks nobody who doesn't use it. This is Feldman's "narrowest possible arguments" made
   type-level, and it is **only available at seam 2** — `App::update`'s fixed signature forecloses it
   the moment a slice becomes an `App` (§2, finding 3).
4. **A `Session` slice that others read but never write.** `&Model.session` passed as `ctx`. Simple,
   and the one-writer discipline is by convention rather than by the compiler.

---

## 8. Encapsulation, paid for with module privacy

TCA shops all converged on the same three buckets — external / internal / delegate — enforced by
naming conventions and custom linters, because *"Enums in Swift don't have any access control."*
Rust does, at the payload:

```rust
pub enum Intent {
    Search(String),                 // public: anyone may construct
    Select(ProductId),
    Internal(Internal),             // variant public, payload not constructible outside
}

pub struct Internal(pub(crate) InternalKind);   // or: private fields
```

The variant is visible and matchable — so exhaustiveness still works, and the app can still *see*
that an internal event occurred — but no other module can *construct* one. That is Zabłocki's
taxonomy with the compiler doing the enforcing rather than a lint, which is what he actually wanted
and couldn't have.

Two mechanical notes:

- Internal variants get `#[serde(skip)] #[facet(skip)]` on the variant and `#[facet(opaque)]` on the
  payload, so they never reach typegen. This is also the cheapest way to shrink derive cost (§9).
- Iced's 30:2 argument — the parent should see 2 things, not 30 — is answered here by privacy rather
  than by a separate `Action` type. The public variants *are* the narrow channel. You get the
  compression without the extra enum and without the mapping.

---

## 9. Enum growth: what is actually true

The fear is real but misaimed. Measured, on `rustc 1.97.1`:

- **Length is nearly free.** A flat enum with an exhaustive single-column match fits `O(n^1.23)`:
  3,200 variants check in 0.66 s; 6,400 in 1.67 s. rust-analyzer at **4,000 variants costs 2.7 ms of
  incremental re-analysis**. A 300-variant `Event` is a non-event.
- **Derives are 70% of the cost**, not the match. At 3,200 variants, `Debug + Clone` = 0.63 s;
  removed = 0.19 s. `Serialize + Deserialize` is a further ~2.5× linear multiplier. **So the lever is
  `#[serde(skip)]`/`#[facet(skip)]` on internal variants, not splitting the enum.**
- **Width is exponential, and this is the one shape to forbid.** `match (event, state)` over tuple
  columns with wildcards is NP-hard in principle and pathological in practice: 20 tuple columns
  = 1.02 s, roughly 2× per column added. Every real blowup in Rust (#118437), Elm (#1362) and Swift
  (SR-7907) is this shape misfiled by its reporter as a length problem. It is also exactly what
  `elm-spa-example`'s `Main.update` does. **Dispatch on one axis, then the other.**
- `size_of::<Event>()` is the largest variant, permanently. `clippy::large_enum_variant` and
  `-Z print-type-sizes` are the tools; box at the boundary, as weather already does.

And the deflation worth remembering, from the Elm discourse: *"nesting reallocates variants, it does
not reduce them."* Nesting is a **boundary** decision. Nobody in any ecosystem argues it on
compile-time grounds.

---

## 10. When to reach for separate `Core`s

The micro-frontend literature is unanimous that this is an **organisational** decision. Joel Denning
(creator of single-spa) gives the crispest gate: *"Avoid micro-frontends when… your monolith is
working well for you. There is only one dev on the project. Separate deployments cause more pain than
benefit… **If the micro-frontends are talking to each other all the time, perhaps you should not be
using micro-frontends.**"* Mezzalira adds the asymmetry: *"You're always on time to move from a
coarse-grained to a fine-grained implementation. The other way around though is not true."*

The gate, then: **three teams, three deploy cadences, three nameable domains.** If any of those lists
has one entry, use seam 2.

But note the thing that makes seam 4 work better here than anywhere in that literature: **separate
Cores can share one store.** Two `Core`s in one binary, both talking to the same Rust store handler,
both appending facts, both projecting. No shell mediation for domain state — which is the failure
mode every micro-frontend postmortem describes. What they still cannot share is transient UI state,
so §7 gets harder, and §2's finding 3 bites: as `App` impls they lose the context parameter.

The Crux-specific taxes, so they are priced: `Bridge` is per-`App`, so N Cores means N bridges and N
independent `EffectId` spaces; several Cores cannot share one `EffectRouter` (it owns
`core: Core<App>` by value); and typegen becomes N packages cross-referenced by hand-written relative
paths, per language, per namespace.

Apply Rappl's shutdown test as the acceptance criterion: **can you delete or disable slice X and ship
the rest?** If no, for any X, it is a distributed monolith on paper.

---

## 11. What to forbid, explicitly

Worth writing down, because each is an attractive local decision with a bad global consequence.

1. **`match (event, state)`.** §9. The only genuinely exponential shape, and it is what a state
   machine tempts you into. Dispatch on the event, then on the state, inside the arm.
2. **A wildcard arm that swallows a sibling's events.** Elmish's lesson: interception is spelled as
   match-with-wildcard, and the wildcard disarms the compiler you adopted this architecture to
   recruit. Prefer `Command::done()` in an explicitly enumerated arm.
3. **Domain-shaped effects.** `SubscribeStudentOperation` is a smell; `StoreOperation` is not. The
   parent's `Effect` is the union of every slice's, and Ludovico's rule is what keeps the union
   small. This also keeps the FFI surface stable as features churn.
4. **Slices naming each other's types.** Cross-slice coordination is a fact in the log (§4). If two
   slices must know each other, they are one slice, or the coupling belongs in the composition root
   — the app's `update` — which is the only place allowed to know both names.
5. **`Set*`-shaped intents emitted in sequence.** Dan Abramov's mechanical test: *"If your action
   creator names start with `set*` and you often call multiple in a row, you might be missing the
   point."* In an event-sourced core this is worse than in Redux, because a setter intent has no
   corresponding fact and produces a log that records nothing meaningful.
6. **Closures as event payloads.** Tempting as an escape from enum growth, and fatal here: not
   `Debug`, not `Clone`, not `Serialize`, and the Elm compiler's error message says it best —
   *"Functions cannot be serialized, nor can values that contain functions."* Your whole FFI and
   your whole DST harness depend on events being data.

---

## 12. Where I'd spike this

1. **Two slices, generic over `Ef`/`Ev`, one `From` impl each** (§6). Prove the bound arrangement
   compiles, that `Ev::from(...)` at emission genuinely replaces `map_event`, and that the app's
   `#[effect]` macro supplies the `From<Request<Op>>` impls. *Falsifies:* the core proposal. Half a
   day, and everything else rests on it.
2. **A slice tested against a minimal test-local effect enum.** *Falsifies:* whether the "test with
   only the effects this slice uses" property is real or whether typegen/`#[effect]` forces a
   concrete type.
3. **Cross-slice reaction through the log**, with no code edge — slice A appends, slice B's
   projection updates, both render. *Falsifies:* §4, the governing claim.
4. **The `ctx: &impl HasSession` arrangement with three slices declaring different bounds.**
   *Falsifies:* §7's claim that this beats Halloy's eight parameters without becoming its own
   ceremony.
5. Only then, and only if the org gate in §10 is met: two `Core`s over one store.

---

## Open questions

1. **Is `Ev: From<Intent>` enough, or do slices need to emit sibling intents?** If a slice ever needs
   `Ev: From<OtherSliceIntent>`, the bound list grows and the slice is no longer independent. My
   expectation is that this never happens because the answer is always "append a fact" — but it is
   the first thing that would falsify §4, and it is worth watching for in the spike.
2. **Where does the shell-side lens live?** Proton's `Feature`/`FeatureCoreMap` solves a real problem
   and is per-platform hand-written. With flat slices the view model is already
   `struct ViewModel { catalog: catalog::View, … }` and a plain field access may be enough — but if
   you take the invalidation/direct-query read path from the companion notes, the shell reads
   projections directly and this may not arise at all.
3. **Does a slice own a tag namespace, or does a domain crate?** §4 makes the namespace the real
   integration surface. If a slice and a domain crate can disagree about it, G3 in the companion
   notes gets harder rather than easier.
4. **One `Model` or one per slice, for rehydration?** `Core::new_with` takes the whole model. If
   slices rehydrate at different rates — one from the log, one from a projection store, one not at
   all — the app-lifecycle state machine returns, and that is the one place nesting may genuinely
   earn its keep.

---

## Sources

- [Nested state machines](https://redbadger.github.io/crux/part-2/nested_state_machines.html) · [RFC: Command ("App composition is not very flexible")](https://redbadger.github.io/crux/rfcs/command.html) · [RFC: Effect Router](https://redbadger.github.io/crux/rfcs/effect-router.html) · [crux issue #79](https://github.com/redbadger/crux/issues/79) · [examples/weather/ARCHITECTURE.md](https://github.com/redbadger/crux/blob/master/examples/weather/ARCHITECTURE.md)
- [ProtonMail/proton-rust-nation-2026](https://github.com/ProtonMail/proton-rust-nation-2026) · [Talk: Crux Micro-Frontends](https://www.youtube.com/watch?v=B5gPuHygthQ)
- Elm: [guide — Structure](https://guide.elm-lang.org/webapps/structure.html) · [Feldman on r/elm](https://old.reddit.com/r/elm/comments/5xdl9z/elm_architecture_with_a_reduxlike_store_pattern/dehrcx8/) · [Scaling Elm Apps](https://www.youtube.com/watch?v=DoA4Txr4GUs) · [Elm Radio 19](https://elm-radio.com/episode/scaling-elm-apps/)
- TCA: [Performance.md](https://pointfreeco.github.io/swift-composable-architecture/main/documentation/composablearchitecture/performance/) · [FAQ](https://www.pointfree.co/blog/posts/141-composable-architecture-frequently-asked-questions) · [Zabłocki on boundaries](https://merowing.info/posts/boundries-in-tca/) · [Zabłocki on multi-store](https://merowing.info/posts/multi-store-tca/) · [Tripadvisor migration](https://www.infoq.com/news/2025/06/tripadvisor-tca-migration/)
- Redux: [Style Guide](https://redux.js.org/style-guide/) · [createSlice#extraReducers](https://redux-toolkit.js.org/api/createSlice#extrareducers) · [Beyond combineReducers](https://redux.js.org/usage/structuring-reducers/beyond-combinereducers)
- Bloc: [Architecture — bloc-to-bloc communication](https://bloclibrary.dev/architecture/#bloc-to-bloc-communication) · [issue #3816](https://github.com/felangel/bloc/issues/3816)
- Iced: [`Component` deprecation 9426418](https://github.com/iced-rs/iced/commit/9426418adbaac40f584fe16b623521a3a21a1a4c) · [Unofficial Iced Guide](https://jl710.github.io/iced-guide/) · [discourse](https://discourse.iced.rs/t/questions-about-reusable-components-and-app-design-pattern/546)
- Elmish: [parent-child](https://elmish.github.io/elmish/docs/parent-child.html) · [The Elmish Book — scaling](https://zaid-ajaj.github.io/the-elmish-book/#/chapters/scaling/intent) · [Mangel's tips](https://medium.com/@MangelMaxime/my-tips-for-working-with-elmish-ab8d193d52fd)
- Micro-frontends: [Mezzalira](https://www.buildingmicrofrontends.com/) · [arXiv anti-pattern catalog](https://arxiv.org/html/2411.19472v1) · [Thoughtworks Radar](https://www.thoughtworks.com/radar)
- Dioxus: [hoisting](https://dioxuslabs.com/learn/0.7/essentials/basics/hoisting) · [stores](https://dioxuslabs.com/learn/0.7/essentials/state/stores)
- Enum growth: [rust-lang#118437](https://github.com/rust-lang/rust/issues/118437) · [rust-analyzer#5397](https://github.com/rust-analyzer/rust-analyzer/issues/5397) · [swiftlang#53934](https://github.com/swiftlang/swift/issues/53934) · [Compiling Rust is NP-hard](https://compilercrim.es/rust-np/)
