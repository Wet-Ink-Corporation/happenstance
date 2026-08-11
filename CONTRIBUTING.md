# Contributing

## Before you start

Read [`docs/adr/`](docs/adr). A handful of decisions shape everything else: the
two-flavour async ports, opaque payloads, the edition and MSRV, and the crate
naming. Changing one is fine — but it means writing a new ADR that supersedes
the old one, not working around it in code. ADR-0002/ADR-0005 and
ADR-0005/ADR-0006 are worked examples of that: the superseded body is left
factually intact rather than rewritten.

**Some ADRs are marked provisional.** ADRs 0001, 0003 and 0004 were authored in
one sitting alongside the initial scaffold, before the code they constrain
existed — and ADR-0002 was superseded thirty-two minutes after it was written.
Each provisional ADR states what would have to happen for it to become
precedent. Until that happens, treat it as a recorded intention: contradicting
one still needs a superseding ADR, but you do not owe deference to a decision
the code has not yet voted on.

ADR-0001 is the worked example of a marker being *earned off*: it lifted when
`LocalMemoryEventStore` — the first `!Send` implementer of either port — passed
the conformance suite, which is the exact condition its banner had named.
ADR-0003 and ADR-0004 still carry theirs. Do not let an unpublished API surface, or an
MSRV nobody depends on, decide a design question on its own.

## The gate

```console
cargo xtask ci
```

That is the whole thing: formatting, clippy with `-D warnings`, tests, the
`wasm32-unknown-unknown` build of `happenstance-core` and its feature powerset,
documentation — with `--all-features`, again with `--no-default-features`, and
once more on nightly under `--cfg docsrs` when a nightly toolchain is present —
specification traceability, a `cargo package --list` assertion that every
publishable crate ships both licences and a README, and, when the tools are
installed, `cargo hack` feature-powerset and `cargo deny`. It is defined once in
`xtask/src/main.rs`, and CI runs exactly the same command. If it passes locally,
it passes on CI.

Three of those need a word on why they exist, because each was added after
something passed that should not have. The docs build runs twice because a broken
intra-doc link is a hard rustdoc error rather than a warning, and three links
resolved only with `--all-features` on — the `no_std` configuration had been
failing for as long as the gate had existed, and nothing built it. The
`cargo package --list` assertion exists because `cargo publish --dry-run` does
not warn about a missing licence file; the first time you learn is when the crate
is on crates.io and cannot be edited. And specification traceability
(`cargo xtask spec-trace`) checks that `SPECIFICATION.md`'s cross-references
still resolve, on the principle that a specification whose citations have rotted
is worse than one that never made any, because it reads as though it is backed by
tests.

The plain `wasm32` check is load-bearing rather than decorative: it is the only
thing keeping the `!Send` port flavour honest until a Cloudflare adapter exists.
It stays mandatory and separate from the `wasm32` powerset beside it precisely so
that the constraint-1 guard cannot become skippable. If you add a step to the
gate, note that `cargo xtask wasm` looks its step up by **name** and not by
index, deliberately — an index is silent about what it selects, and getting it
wrong leaves that check running nothing while still printing green.

Optional tools, if you want the full gate locally:

```console
cargo install cargo-hack cargo-deny --locked
```

## Writing an adapter

1. Implement `SendEventStore` if your store can be shared across threads — every
   native adapter can. Implement `EventStore` only when you genuinely cannot, as
   on `wasm32`. Implementing the former gives you the latter for free.
2. Write a `Fixture` for it. A fixture instance is one **isolated backing
   store**; each `connect()` on it returns one **handle** onto that store. Say
   so honestly — a file-backed fixture that points every instance at the same
   temporary path fails
   `two_fixture_instances_observe_none_of_each_others_appends`, which is the
   whole reason that rule exists. Declare the two capability constants that have
   no default: `SECOND_HANDLE` if you can open a second handle onto one store,
   `REOPEN` if you can discard process state and read back only what was durably
   committed. A third, `MID_BATCH_FAULT`, defaults to declined — override it only
   if your store can be made to fail while writing the *k*-th event of a batch,
   which nothing outside the adapter can arrange. Decline with
   `Capability::declined("…")` and a real reason; it is printed on every run.
   `happenstance_testkit::fixtures::MemoryFixture` is the worked example.
3. Invoke the conformance suite, handing it an expression that builds a fixture:
   ```rust
   happenstance_testkit::event_store_conformance!(MyFixture::new());
   ```
4. Make it pass.

Step 4 is not a formality. An adapter that compiles but has not run the suite is
not an adapter, and will not be merged as one. If a rule looks wrong, say so and
fix the rule — a bad rule costs every future adapter author a day.

## Adding a method to a port

The two-flavour derivation ([ADR-0001](docs/adr/0001-async-port-flavours.md),
[ADR-0008](docs/adr/0008-one-derivation-for-both-ports.md)) constrains how a
*provided* method is written, and gets it wrong in a way that is easy to
misdiagnose. Three rules, in the order you will meet them.

**A provided method is never `async fn`.** `trait_variant` clones the default
body into the derived trait while clearing `asyncness`, and does not rewrite the
body — so an `async fn` default becomes a non-async function containing `.await`
and fails with `error[E0728]: await is only allowed inside async functions and
blocks`. Hand-desugar it instead:

```rust
fn head(&self) -> impl Future<Output = Result<Option<SequencePosition>, Self::Error>>
where
    Self: Sync,
{
    async move { /* … */ }
}
```

**`Self: Sync` goes at the point of use, never in the attribute.** A body that
holds `&self` across an `await` holds a `&Self`, and `&Self: Send` holds exactly
when `Self: Sync`. Taking it on the method leaves the stream alone. Putting
`Sync` in the `trait_variant` attribute instead applies the whole bound list to
`read`'s stream as well, so an adapter whose stream hides a `Cell` or an `Rc`
stops compiling with `error[E0277]: cannot be shared between threads safely` —
at the adapter, not here. Note also that rustc's own suggestion for the missing
bound is `#[trait_variant::make(SendEventStore: Send where Self: Sync)]`, which
does not parse: `trait_variant` accepts only `+`-separated trait bounds, and
following the suggestion yields a confusing second error.

**The trait-side and impl-side rules are opposites, and both are enforced.** The
trait forbids `async fn`; an impl that *overrides* such a method is required to
use it, because `clippy::manual_async_fn` is on and the gate denies warnings.
Reading only the first rule and applying it to your impl is a build failure.

One consequence worth knowing before you reach for a provided method: `Self:
Sync` makes it uncallable on a `RefCell`-backed store, which is the shape the
bare flavour exists to serve. If a method must be reachable there, either take
its arguments as parameters so the future captures owned values rather than
`&self`, or make it a required method.

## Conformance rules

When adding one:

- Trace it to a MUST in the [specification](https://dcb.events/specification/),
  or to a property an adapter can plausibly get wrong.
- Put the reason in the assertion message. An adapter author reading a failure
  should learn what rule they broke, not just that something was `false`.
- **Never assert on literal position values.** The specification permits gaps in
  the sequence, so `assert_eq!(positions, [1, 2, 3])` fails a perfectly
  conformant adapter. Compare against positions the store actually assigned.
- Register it in `for_each_event_store_rule!`. That is the only place the set is
  written down, and `registry::no_orphan_rules` fails if you forget.
- A rule takes `impl AsyncFn() -> F`, not a fixture — *how to make one*, so it
  can make two when isolation is the thing under test — and returns a
  `RuleOutcome`. If, and only if, the **whole** rule needs a capability that a
  fixture may honestly decline, gate it with `require!(F: REOPEN)` and let it
  return `Skipped`. A rule that merely strengthens itself when a second handle
  exists has run, and must say `Ran`. `SECOND_HANDLE` is a MUST rather than a
  trade, so a rule needing it uses `must!` and fails; `require!` on a MUST buys
  a declining adapter a green suite and one `SKIP` line, which is the whole
  reason the two macros are separate.
- **Write the wrong implementation.** A rule no adapter can fail is decorative,
  and since [ADR-0010](docs/adr/0010-the-suite-must-prove-itself.md) that is
  enforced rather than reviewed: adding a rule fails
  `mutation_coverage::every_rule_has_a_mutant` until a mutant declares it. Three
  deliberately separate edits, all under
  `crates/happenstance-testkit/tests/mutation_coverage/`:

  1. `mutants.rs` — an `impl Defect` overriding **one** step of the correct
     store in `correct.rs`. One step, because a store broken in several ways
     proves the rule catches *something*, not that it catches this.
  2. `for_each_mutant!` in `tests/mutation_coverage.rs` — the enumeration.
  3. `REGISTRY` in the same file — the exact set of rules it fails, its
     `provenance` (the real adapter shape that makes it plausible; a saboteur is
     rejected by `every_mutant_states_its_provenance`), and its `FailureMode`.

  The claim lives apart from the store on purpose. Put it on the `impl` as an
  associated const and a reviewer edits the claim in the same keystroke as the
  bug it describes, at which point the claim has stopped being a check.

  If a rule genuinely has no plausible failing implementation, ADR-0010's honest
  response is to **retire the rule** and say so in its clause — not to invent a
  store no author would ship.
- **Add a `CHANGELOG.md` entry naming the defect the rule detects** (CF-29).
  Adding a rule is a semver-*minor* change that turns a passing adapter's CI red,
  and an adapter author who takes the bump needs to tell "my adapter has a
  defect" from "the suite changed". Nothing mechanical checks this; it is the one
  step on this list that only review catches, and the clause says so.

### The model family

`event_store_model_conformance!` is a second rule family, in
`crates/happenstance-testkit/src/model.rs`, and the list above applies to it with
two substitutions. Its enumeration is `for_each_model_rule!` rather than
`for_each_event_store_rule!` (CF-22 wants one list per *family*, and there is no
`no_orphan_rules` on this side because `spec-trace` does not read the file). And
its wrong implementations are not declared in `REGISTRY`, which is keyed on named
suite rules: they are declared in `MODEL_COVERAGE`, an **exhaustive** table of
what the model does to every store in the proof artefact, held by
`mutation_coverage::the_model_rule_rejects_exactly_what_it_claims`.

Exhaustive rather than a list of successes, because the question worth asking of
a model-based test is what it is *blind* to. If your change moves a store from
`Agreed` to `Rejected`, the model got stronger and the table should say so; if it
moves one the other way, the table is the only thing that will notice.

Two things not to do here. Do not reach for `proptest!` — it generates a
synchronous test body, so a rule inside one has to choose a `block_on` inside the
testkit, which is CF-23's prohibition one level down. And do not make the RNG
non-deterministic: a conformance rule that fails one run in twenty teaches an
adapter author to rerun CI until it is green.

### The concurrency family

`event_store_concurrency_conformance!` is the third family, in
`crates/happenstance-testkit/src/concurrency.rs`. Its enumeration is
`for_each_concurrency_rule!`, its wrong stores live in
`tests/mutation_coverage/racers.rs`, and its table is `RACERS` — a separate one
rather than more `REGISTRY` rows, because every store in it fails **none** of the
event-store family's named rules and `mutant_registry_is_exhaustive` rejects a
row with an empty `fails` list.

**There is no `no_orphan_rules` on this side either**, for the same reason the
model family has none: that check `include_str!`s `suite.rs` and nothing else, so
a `pub async fn` added here and left out of `for_each_concurrency_rule!` compiles,
never runs, and is reported by nothing.
`the_concurrency_rules_reject_exactly_what_they_claim` does not close it —
that test enumerates *from* the same macro, so a rule missing from the list is
missing from the check too. Add the name to the macro in the same change as the
rule. `SPECIFICATION.md` CF-24 records the gap and what the mechanical fix would
be.

Five rules specific to it, and the first two are the ones that will bite.

**No clock, no sleep, no timeout — including the one that looks like a safety
feature.** CF-33 is `[FROZEN]` and a watchdog is a clock. Liveness rests on the
CI job timeout; the cost is that a deadlocking adapter hangs rather than naming a
rule, and that cost is accepted rather than overlooked. If you find yourself
wanting `thread::sleep` to make a race reproducible, what you want is a
**rendezvous**: an atomic counter and `std::thread::yield_now`, bounded by a
number of yields so nothing hangs. `Shared::wait_for_company` in `racers.rs`
records three versions of that idea and why the first two were wrong.

**Start the contenders at a barrier.** Spawning N threads in a loop does not
start them together, and it was measured: the first contenders finished before
the last were spawned, and a wrong store was joined by *pairs* rather than by
everybody, so the rule meant to reject it passed about half the time. `race` in
`concurrency.rs` owns that barrier; a rule that spawns its own threads has to
think about it again.

**Hand each contender its own handle, by value.** The bound is
`F::Store: EventStore + Send` and it stays that way only while nothing is shared
by reference — `&F::Store: Send` means `F::Store: Sync`, which would exclude
adapters for nothing.

**Collapse each contender's result before its thread ends.**
`EventStore::Error` carries no `Send` bound, so a `Result<_, AppendError<S::Error>>`
cannot leave the thread that produced it. `Attempt` is that collapse.

**If a rule has a thread waiting on a flag, set the flag before you propagate a
panic.** `std::thread::scope` joins every scoped thread even while unwinding, so
`resume_unwind` taken *before* the flag is stored leaves the waiter spinning and
the scope never returns — and with no watchdog anywhere, that turns a nameable
store panic into a silent CI timeout. Join everything into a `Vec` first, set the
flag, join the waiter, and only then walk the results. `observe_while_writing` in
`concurrency.rs` is the one place this arises and it says so in a comment.
The same function opens with a read on the rule's own thread before the scope,
for the mirror-image reason: a spawned thread is not a scheduled one, and on a
loaded host every writer can finish before the reader runs once.

## Style

Two things to know before your first commit:

- Public items are documented, and fallible functions get an `# Errors` section.
- No `unwrap`/`expect` in library code. Test modules opt out locally with
  `#![allow(clippy::unwrap_used)]`.

Everything else — and the reasoning behind both of those — is in
[`standards/rust/`](standards/rust/README.md), the Rust constitution. It is organised so
you load one to three files for the task in front of you rather than reading it
through: start at the router's trigger table. The gate checks it like anything
else, so its examples compile and its citations resolve.

## Commits and pull requests

Keep the reasoning in the commit message. A diff shows what changed; the message
should say what constraint made that the right change.

Pull requests run an extra `cargo-semver-checks` job over all three publishable
crates, with `--baseline-rev` pointed at the commit the branch started from. A
breaking change is fine — an accidental one is not.

Know what that job proves and what it does not. It proves *this pull request*
does not break the API it branched from. It proves nothing about the last
released version: a break merged two pull requests ago is part of the baseline
and so is invisible. The registry baseline that would catch it is not available
yet — the three reserved names sit at `0.0.0`, and Cargo treats every `0.0.x`
version as incompatible with every other, so there is no compatible predecessor
to compare against. Phase 12 keeps both baselines once a real `0.1.0` exists.

A separate weekly job runs `cargo deny check advisories` and nothing else. A new
advisory against a dependency nobody has touched is the only failure that arrives
without a commit to trigger it, so it gets a schedule; licences and bans change
only when a manifest does, and the gate already covers those on every push.

## Licensing of contributions

Unless you state otherwise, any contribution you intentionally submit for
inclusion in this project — as defined in Apache-2.0 — is licensed under
`MIT OR Apache-2.0`, with no additional terms or conditions.

This is the Rust ecosystem's "inbound equals outbound" convention: what you send
in is licensed on the same terms the project ships out. There is no CLA and
nothing to sign. It exists so the project can keep dual-licensing without having
to track down past contributors for permission — which is a problem that is
trivial to prevent and expensive to fix, because the fix is asking every
contributor you have ever had.

Copyright in the project is held by Wet Ink Corporation (see
[`LICENSE-MIT`](LICENSE-MIT)); you keep the copyright in what you write, and this
section is the licence you grant, not an assignment.
