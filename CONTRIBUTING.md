# Contributing

## Before you start

Read [`.kb/decisions/`](.kb/decisions/). A handful of decisions shape everything else: the
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
ADR-0003 and ADR-0004 still carry theirs. That used to be cheap, because nothing
was published and nobody depended on the MSRV. It stopped being cheap at `0.2.0`:
the API surface is on crates.io, and ADR-0037 made the MSRV a promise. A design
question that would move either one is now also a semver question.

## One thing to get right before you clone, on Windows

**Clone to a short path.** `D:\hs` is fine; a checkout nested a few levels under
your user profile is not.

`experiments/` carries result transcripts whose filenames encode the condition
that produced them, and the longest of them is around 100 characters. Under
Windows' default 260-character path limit that leaves very little room for the
directory above it, and the failure is not a warning:

```text
error: unable to create file experiments/busy-timeout-margin/results/raw/waits/
waits-headroom-c20-n96-begin-immediate-probe-positions_are_unique_under_concurrent_appends.txt:
Filename too long
fatal: Could not reset index file to revision 'HEAD'
```

That is a `git worktree add` into a path that was merely *deepish*, and it leaves
a half-populated tree behind. `git config --global core.longpaths true` also
fixes it, and is the better fix if you already have the checkout where you want
it. Linux and macOS are unaffected.

## The gate

```console
cargo xtask ci
```

That is the whole thing: formatting, clippy with `-D warnings`, tests, the
`wasm32-unknown-unknown` build of `happenstance-core` and its feature powerset,
documentation — with `--all-features`, again with `--no-default-features`, on
the default features of `happenstance-core` and of `happenstance`, and once more
on nightly under `--cfg docsrs` when xtask's pinned `NIGHTLY` toolchain is installed —
specification traceability, a `cargo package --list` assertion that every
publishable crate ships both licences and a README, and, when the tools are
installed, `cargo hack` feature-powerset and `cargo deny`. It is defined once in
`xtask/src/main.rs`, and CI runs exactly the same command. If it passes locally,
it passes on CI.

**One CI job does not run on a pull request from a fork, and it is not your
fault.** *backlog and knowledge base* checks `.bklg/` and `.kb/` using the
redkiln CLI, which lives in a private sibling repository and needs a credential.
GitHub deliberately withholds secrets from workflows triggered by a fork, so the
job is skipped there rather than failed — failing it would report a problem you
have no way to fix. You will see it as *skipped* in the checks list. A maintainer
re-runs it from a same-repository branch before merging. Nothing else about the
gate changes, and `cargo xtask ci` still tells you everything it tells them.

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

One obligation the suite cannot check, because no rule can read a doc comment:
ES-23 requires each adapter to state what a dropped `append` future does. Answer
it under a `# Cancellation` heading in your store module, and keep it there with
a test target copied from one of the four that exist:
`crates/happenstance-{sqlite,cloudflare,postgres,neon}/tests/cancellation_statement.rs`.
Then add yours to ES-23's adapter-owned row in `FROZEN_DOC_MUSTS`
(`xtask/src/lint_narrative.rs`); `cargo test -p xtask` fails until you do.

## Adding a method to a port

The two-flavour derivation ([ADR-0001](.kb/decisions/0001-async-port-flavours.md),
[ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md)) constrains how a
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
  and since [ADR-0010](.kb/decisions/0010-the-suite-must-prove-itself.md) that is
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
**rendezvous** that waits for a known party, not for a count of yields: a yield
bound is a timeout and loses under load. `Shared::cohort` in `racers.rs` records
four versions of that idea and why the first three were wrong.

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

Pull requests run `cargo-semver-checks` twice over all nine publishable crates —
`happenstance-core`, `happenstance`, `happenstance-testkit`, `happenstance-sqlite`,
`happenstance-cloudflare`, `happenstance-postgres`, `happenstance-neon`,
`happenstance-sync` and `happenstance-sync-testkit`. A breaking change is fine; an
accidental one is not.

The crates are named rather than counted, because the count has been wrong here
**twice**. This paragraph said *three* from the release that had three, through
two phases that each added one; it then said *five* through the release that made
the set seven. A number in a document nobody re-reads goes stale silently, and
`xtask/src/package.rs`'s `PUBLISHABLE` is the list actually held against the
manifests. `stated_crate_counts` in `xtask/src/lints.rs` reads this paragraph on
every run, and it was written because of this sentence.

The two runs answer different questions. The **rev baseline** diffs against the
commit the branch started from. It catches a break introduced by *this pull
request*, and it cannot see a break merged two pull requests ago, because that
break is already in its baseline. The **registry baseline** diffs against the
newest compatible version on crates.io. It catches a break against what a consumer
can actually install, and it does not say which pull request introduced it. The
registry baseline has run on every pull request since `86a410c`, the commit that
turned it on once `0.2.0` was published. Until then the registry had nothing to
diff against. `happenstance-postgres` and `happenstance-neon` were left out of the
rev baseline until phase 15. Before `0.2.0` their surface had no published
predecessor, so every difference was a break against nothing.

A separate weekly job runs `cargo deny check advisories` and nothing else. A new
advisory against a dependency nobody has touched is the only failure that arrives
without a commit to trigger it, so it gets a schedule; licences and bans change
only when a manifest does, and the gate already covers those on every push.

## Design principles

These are the reasons behind the contract's shape. They used to open the README,
and they moved here when the README started leading with what the library is
for.

**Illegal states are unrepresentable.** `Query` is an enum, not a vector,
because the specification allows "at least one item" or "match everything" and
nothing else — an empty query cannot be constructed. `SequencePosition` wraps a
`NonZeroU64`, so position zero does not exist and `Option<SequencePosition>`
costs no more than a bare one. `Tags` is canonically sorted at construction, so
it can never be observed out of order and set equality is just `==`.

**Payloads are opaque.** `Event` holds `Bytes`, and `happenstance-core` has no
`serde` dependency by default — the optional feature covers the envelope types
only, for replication. Adapters need no domain knowledge, and replication can
forward events byte-for-byte without deserialising them. Encoding belongs to the
layer above, which is what `happenstance` itself is for.

**The concurrency signal is in the type system.** `append` returns
`AppendError`, which separates `ConditionViolated` — routine under contention,
and a cue to retry — from adapter-specific failures. No caller has to match on a
string to tell "retry" from "something broke".

**Two flavours of each port.** `EventStore` carries no `Send` bound, so it can
be implemented on `wasm32` where futures are `!Send` — which is what makes a
Cloudflare Durable Object adapter possible at all. `SendEventStore` is derived
from it for native use, and implementing it gives you both. Generic code binds
the weaker one and accepts either. CI builds `happenstance-core` for
`wasm32-unknown-unknown` on every commit so this stays true.

**Adapters are separate crates, not feature flags.** `rusqlite` bundles a C
library; LadybugDB's `lbug`, the retired graph adapter's driver, compiled C++ through `cmake`; Postgres needs a
network and a container to test against. Nobody who wants one should pay for the
other, and a third-party adapter is a first-class citizen rather than a fork.

**The adapter portfolio is deliberately unlike itself.** Adapters are not only
targets to support — they are the instrument that keeps the contract honest, and
that only works if they disagree. A store that serialises its writers under a
lock, one that assigns positions outside the transaction, and one reached over
one-shot HTTP with no interactive transaction at all will each refuse a different
part of a badly-shaped port, and a portfolio where all three behave alike proves
nothing. So Postgres is here for what it *breaks* — it is the only target on the
roadmap that can violate the position-visibility invariant, which is what makes
that invariant testable rather than decorative. It is not a flagship, and if you
want DCB on Postgres today, [`disintegrate`](https://github.com/disintegrate-es/disintegrate) is the mature choice.

## Performance claims

**Every performance figure the README or the site quotes comes from one measured
record, and nothing is quoted that the record does not hold.** An earlier README
opened with the words "fast, efficient" while nothing in the repository
supported them — three of the fourteen reviews say so — and the numbers came out
until there was something to cite.

The record is kept where it can be checked:

* [`benchmarks/results/GRADES.md`](benchmarks/results/GRADES.md) — what the
  abstraction costs over raw SQL, throughput, the DCB guard, the typed layer,
  the projection runner, and exact allocation counts. Every figure is written by
  hand from `benchmarks/results/raw/`, which is committed beside it.
* [`benchmarks/README.md`](benchmarks/README.md) — the conditions each figure was
  taken under, and a **What none of this shows** section that is the first thing
  to read if you are about to quote one.

Two of its own rules are worth repeating here, because they are what make the
rest usable. Read the figures as **orders of magnitude and as ratios between arms
of one run**; nothing there should be quoted to a third significant figure. And
prefer the allocation counts to the timings — they are reproducible to the digit
where the wall clock is not.

`benchmarks/` is not a workspace member, appears in no `cargo xtask ci` step, and
contains no threshold on any budget. CF-34 forbids a benchmark result gating a
merge: an adapter that is slow is conformant. To re-derive the whole record from
a clean checkout, `cd benchmarks && ./run.sh`.

## Why the MSRV is where it is

**1.97.1**, and it is a promise from `0.2.0` rather than a preference: raising it
is a breaking change and needs a decision record, not a commit message. The
floor moved to 1.97.1 at phase 2 for a dependency's build script rather than for
anything in this workspace — five of the five database crates here declare no
`rust-version` at all, so neither `cargo hack --rust-version` nor `resolver = "3"`
can protect a floor against them, and only running the compiler finds it.

After `1.0.0` a rise ships only in a minor release, only to a Rust stable at
least six months old, and always with a `CHANGELOG.md` entry
([ADR-0067](.kb/decisions/0067-msrv-after-1-0-rises-are-bounded.md)). Each
published crate's README states its own floor and the same rule, and
`cargo xtask lints` holds them to the manifests.

## Why the README's examples are `xtask` doctests

The two Rust blocks in `README.md` are compiled by CI, so neither can drift
from the API the way a README example normally does. They are doctests of
`xtask`, not of a published crate: `include_str!` resolves against the file tree,
and a path reaching outside the package does not exist inside a packaged
`.crate` — so attaching `README.md` to `happenstance` would make `cargo test` fail
for anyone who ran it. `xtask` is never published. Each crate's own `README.md`
is compiled by that crate, where the relative path stays inside the package.

## The name

The everyday sense of the word is chance, which is the wrong idea for a durable
event store and worth displacing early. The sense meant here is the older half of
the compound: a happenstance is what *happened to be the case*, as against what was
arranged in advance. That distinction is the one DCB draws. A classical aggregate
fixes the consistency boundary when the schema is written, before anyone knows
which decisions will be made against it. A DCB boundary is deliberate but not
pre-declared — the handler chooses its query, and the boundary is then whatever
that query happened to match.

The project was called **eventum** until 2026-08-05. It was renamed because the
bare `eventum` name on crates.io belongs to an unrelated crate, dormant since
2020, which forced an awkward layout — prefixed crates only, and a `-core` suffix
that existed for no reason but the collision. `happenstance` was free. Nothing had
been published, so no release was affected; see
[ADR-0005](.kb/decisions/0005-rename-to-happenstance.md).

The bare name went to the contract crate for four hours, and then to the typed
layer where it belongs: `-core` is what an adapter author pins, and the bare name
is what an application installs — the allocation `serde_core`/`serde`,
`futures-core`/`futures` and `tracing-core`/`tracing` each arrived at
independently. See
[ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md), which supersedes
the second half of ADR-0005 and explains why that half was wrong.

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
