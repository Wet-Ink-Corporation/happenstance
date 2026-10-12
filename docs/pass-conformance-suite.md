# Pass the conformance suite

> **Answers:** `how-to` — How do I make my adapter pass the conformance suite?

This page is for you if you have written a store adapter, a type that
implements the `EventStore` contract over your own storage, and you now want to
prove it is correct. By the end you will have a test file that runs the
conformance suite against your store, and you will know how to read what it
reports.

Why it matters: the **conformance suite** is the shared set of tests that defines
what a correct store does. Application code written against happenstance trusts
that every store behaves the same way, and the suite is how that trust is earned.
An adapter counts as an adapter only once it has run the suite against its own
storage and passed. Compiling is not enough, and the testkit's
[crate documentation](../crates/happenstance-testkit/src/lib.rs) says so at the
top.

If you have not yet built the adapter, start with
[what to read before you write an adapter](adapter-reading-order.md). This page
picks up where that one stops.

## What you write

You write one type and one line of test code.

The type is a **fixture**. The suite never takes your store directly; it asks the
fixture for one. The fixture tells the suite how to open your store, and declares
what your store can and cannot do. The line hands the fixture to the suite, which
expands into one test per rule.

A few words used on the rest of this page:

- A **rule** is one test in the suite, with a descriptive name such as
  `two_handles_observe_each_others_appends`.
- A **family** is a group of rules with its own macro. The main one checks the
  event store; others race threads, generate random operations, or check
  projections.
- A **capability** is something the suite wants to test but your store may
  honestly be unable to support, such as surviving a restart. You declare each
  one as supported, or declined with a reason.

The rest of this page covers what to put in the fixture, how to read the suite's
output, and which other families to add.

## Write the fixture

### One instance, one store

Each fixture instance must have a backing store of its own, such as a fresh
temporary file or a throwaway database schema. Each call to `connect` opens a new
handle onto that same store
([CF-15](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
`two_fixture_instances_observe_none_of_each_others_appends`).

This separation is what lets the suite run rules side by side without them
seeing each other's events. Two instances that point at one file fail that rule.

### The two capabilities you must declare

A fixture declares eight constants. Two have no default, so the compiler makes
you answer them:

- **`SECOND_HANDLE`** is not really a choice. The suite opens two handles onto
  one store, to catch a store that is only correct within a single connection
  ([CF-16](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
  `two_handles_observe_each_others_appends`). Declining it does not skip that
  rule: the rule fails and quotes your reason.
- **`REOPEN`** asks whether your fixture can close the store and open it again,
  so the suite can check that acknowledged writes survive a restart. If your
  store keeps its data somewhere that outlives the process, support it and
  override `reopen`; the clause says what the override must do. An in-memory
  store declines, and says why
  ([CF-17](../spec/SPECIFICATION.md#63-the-fixture-contract), provisional;
  `acknowledged_writes_survive_a_reopen`).

### The two fault capabilities, which you should still write

`MID_BATCH_FAULT` and `READ_FAULT` ask whether the suite can make your store fail
part way through writing a batch, or part way through a read. Both default to
declined, with a reason written by the testkit.

Do not leave the default. The suite adds a test,
`every_declined_capability_is_stated_by_this_fixture`, that fails while either
one still carries the testkit's wording
([CF-18](../spec/SPECIFICATION.md#63-the-fixture-contract)). The point is that
the reason should be about *your* store. Write why it cannot be faulted there,
or support the capability and override the matching `arm_` method.

Supporting `MID_BATCH_FAULT` is a promise about where your fault fires
([CF-39](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
`arming_a_mid_batch_fault_makes_the_append_fail`), so read the clause before you
claim it.

### Read-your-own-writes

`READ_YOUR_OWN_WRITES` defaults the other way, to supported. It means an append
you were told succeeded shows up in your next read through the same handle.

A store whose reads can briefly lag an acknowledged append should decline it,
with its own reason. The PostgreSQL and Neon fixtures do, because a read there
only shows rows below a server-wide snapshot point. Only the model family needs
this capability, and it reports a skip without it
(`Fixture::READ_YOUR_OWN_WRITES`, read by `ops_agree_with_the_model`).

### The four size limits

Last come four ceilings: `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`,
`MAX_EVENTS_PER_BATCH` and `MAX_METADATA_LEN`. These are facts about your store
rather than trade-offs, so each takes a number, or `None` for no limit, and no
reason. The metadata ceiling is the one with no floor: any number is legal.

When you state a number, the suite tests both sides of it: an append within the
ceiling must be accepted, and one beyond it must be refused
([CF-40](../spec/SPECIFICATION.md#63-the-fixture-contract), provisional;
`append_reports_exceeded_store_limits`). Use your adapter's own constants rather
than typing the numbers again, as the SQLite fixture does in
[its support module](../crates/happenstance-sqlite/tests/support/mod.rs), so the
two can never disagree.

### A whole fixture

The program below is a complete fixture for a store that is deliberately not the
reference in-memory store. `GappyMemoryStore` hands out positions seven apart,
so it leaves gaps between them. Your handle would wrap a connection or a pool
where this one wraps an `Arc`.

The single macro line, `event_store_conformance!`, is everything that goes in
your `tests/conformance.rs`:

```rust
use core::num::NonZeroU64;
use std::collections::BTreeSet;
use std::sync::Arc;

use futures_core::Stream;
use happenstance_core::{AppendCondition, AppendError, Event, EventId, Query};
use happenstance_core::{ReadOptions, SendEventStore, SequencePosition, SequencedEvent};
use happenstance_testkit::{Capability, Fixture, GappyMemoryStore, NO_STORE_LIMITS};
use happenstance_testkit::{RuleOutcome, UNSTATED_MID_BATCH_FAULT, UNSTATED_READ_FAULT};
use happenstance_testkit::{block_on, rules};

/// One handle onto a fixture's store.
#[derive(Clone)]
struct Handle(Arc<GappyMemoryStore>);

impl SendEventStore for Handle {
    type Error = <GappyMemoryStore as SendEventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.0.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.0.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.0.contains_event_id(id).await
    }
}

/// One instance is one store. Every `connect` is a handle onto it.
struct StridedFixture(Arc<GappyMemoryStore>);

impl StridedFixture {
    fn new() -> Self {
        let stride = NonZeroU64::new(7).expect("seven is not zero");
        Self(Arc::new(GappyMemoryStore::with_stride(stride)))
    }
}

impl Fixture for StridedFixture {
    type Store = Handle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(
        "the events live in a Vec in this process, so nothing outlives a reopen",
    );
    const MID_BATCH_FAULT: Capability = Capability::declined(
        "a batch is written under one lock, so no fault can fire between two rows",
    );
    const READ_FAULT: Capability = Capability::declined(
        "a read iterates memory the store already holds, so there is no fetch to fail",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(Handle(Arc::clone(&self.0)))
    }
}

happenstance_testkit::event_store_conformance!(StridedFixture::new());

fn main() {
    // Every rule the suite enumerates, driven the way `emit_blocking` does.
    // A rule that fails panics, so reaching the end means none failed.
    macro_rules! run_each {
        ($($rule:ident),* $(,)?) => {
            [$((stringify!($rule), block_on(rules::$rule(async || StridedFixture::new())))),*]
        };
    }
    let outcomes = happenstance_testkit::for_each_event_store_rule!(run_each);

    let mut skipped = BTreeSet::new();
    for (rule, outcome) in outcomes {
        if let RuleOutcome::Skipped { capability, reason } = outcome {
            skipped.insert(capability);
            // Each reason is this fixture's own, never the testkit's default.
            assert!(![UNSTATED_MID_BATCH_FAULT, UNSTATED_READ_FAULT].contains(&reason), "{rule}");
            if capability == "REOPEN" {
                assert_eq!(Some(reason), StridedFixture::REOPEN.reason(), "{rule}");
            }
        }
    }
    let declined = ["MID_BATCH_FAULT", NO_STORE_LIMITS, "READ_FAULT", "REOPEN"];
    assert_eq!(skipped, BTreeSet::from(declined));
}
```

In your crate, `cargo test` turns that macro line into one test per rule. This
page's example is a doctest, and a doctest is not built as a test binary, so the
generated tests do not exist here. Instead, `main` runs the same rules itself,
the way the `emit_blocking` harness does.

`main` then checks three things. Every rule either passed or was skipped, for a
capability the fixture declined or a ceiling it left as `None`. No skip carries
the testkit's default wording. And one generated test,
`every_declined_capability_is_stated_by_this_fixture`, is left to your own
`cargo test`, because it has no rule behind it to call here.

### Gaps in positions are fine

Positions may have gaps
([VT-11](../spec/SPECIFICATION.md#vt-11--positions-are-unique-strictly-increasing-in-assignment-order-and-may-have-gaps)),
and no rule checks for a literal position
([CF-6](../spec/SPECIFICATION.md#61-the-suites-own-proof-obligation), checked by
`mutation_coverage::conformant_variants_pass_everything`). The testkit runs the
whole suite against this exact store, stride and all, in its own tests.

If you write tests of your own beside the suite, hold them to the same rule:
compare against positions your store returned, never against `1`, `2`, `3`.

### Real fixtures to copy from

- SQLite's, over a temporary file, which supports `REOPEN` and states all three
  ceilings, in [its support module](../crates/happenstance-sqlite/tests/support/mod.rs).
- The reference `MemoryFixture`, in
  [the testkit's fixtures](../crates/happenstance-testkit/src/fixtures.rs).
- PostgreSQL's, which gives each instance a throwaway schema, in
  [its support module](../crates/happenstance-postgres/tests/support/mod.rs).

## Read a skip

When a rule needs a capability your fixture declined, it still runs as a test.
It passes, and prints one line naming the capability and your reason
([CF-18](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
`mutation_coverage::capability_skips_are_reported`).

The suite never removes a rule from the test binary instead. In CI, a missing
test looks exactly like a passing one, so removing it would hide the gap.

You have to ask to see those lines. Rust's test runner hides what a passing test
prints, so a plain `cargo test` shows only the pass count. Run it with
`-- --show-output` to read them, and do that in CI if you want anyone else to
see them. `RuleOutcome::report` notes this limit where it is implemented.

Each declined capability is a trade-off, and your reason is the only record of
it. Write it for someone deciding whether to use your adapter, not for yourself.
Nothing checks that a reason is any good: `"n/a"` passes.

## Read a failure

A failing rule panics, and the failing test carries the rule's name. Look the
rule up in the testkit's `rules` module. Its documentation lists the wrong
implementations it is designed to catch, and yours is often one of them.

The program below shows a common mistake. Every `connect` opens a brand-new
store, the way every connection to SQLite's `:memory:` opens a database of its
own. The isolation rule passes, because two fixture instances really do share
nothing. The second-handle rule fails, because two handles onto one instance
share nothing either:

```rust
use std::panic::{AssertUnwindSafe, catch_unwind};

use happenstance_core::MemoryEventStore;
use happenstance_testkit::{Capability, Fixture, RuleOutcome, block_on, rules};

// WRONG: every `connect` opens a new, empty store.
struct StorePerConnection;

impl Fixture for StorePerConnection {
    type Store = MemoryEventStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined("nothing outlives this process");

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(MemoryEventStore::new())
    }
}
// END WRONG.

fn main() {
    let open = async || StorePerConnection;

    let isolated = block_on(rules::two_fixture_instances_observe_none_of_each_others_appends(open));
    assert_eq!(isolated, RuleOutcome::Ran);

    let shared = catch_unwind(AssertUnwindSafe(|| {
        block_on(rules::two_handles_observe_each_others_appends(open))
    }));
    let failure = shared.expect_err("the second handle saw nothing, so the rule fails");
    let message = failure.downcast_ref::<String>().map_or("", String::as_str);
    assert!(message.contains("second handle"), "{message}");
}
```

Decline a capability only when your store genuinely lacks it, and say why.
Declining one just to turn a failing rule green is a different thing.
`SECOND_HANDLE` refuses it outright, as above. The others record it in every skip
line, where the people choosing your adapter can read it.

If you believe a rule is wrong, fix it in the testkit, in the same change as an
example of the wrong implementation the rule must still catch. Never skip it.
`CONTRIBUTING.md`, under
[Writing an adapter](../CONTRIBUTING.md#writing-an-adapter), says the same.

## Choose the harness

Each rule is an `async` function, so something has to run it. The thing that
wraps each rule into a test is called an **emitter**, and you choose it to suit
your runtime
([CF-23](../spec/SPECIFICATION.md#64-runtime-independence)).

The one-line form uses tokio, wrapping each rule in `#[tokio::test]`, so your
`dev-dependencies` need `tokio` with the `macros` and `rt` features. The long
form names the emitter:
`event_store_conformance!(mod_name = …, emit = …, fixture = …)`.

- `emit_blocking` (`happenstance_testkit::emit_blocking`) needs no async runtime
  at all.
- `emit_wasm` (`happenstance_testkit::emit_wasm`) runs under
  `wasm-bindgen-test`, for a store that targets `wasm32`. The Cloudflare adapter
  uses it in
  [its conformance target](../crates/happenstance-cloudflare/tests/durable_object_conformance.rs).

Those names are a stable promise: renaming one would be a major release of the
testkit
([CF-41](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit),
checked by `the_promised_emitters_are_exactly_the_pinned_list`).

You can also write an emitter of your own, but it is not covered by that promise.
The PostgreSQL adapter writes one to mark every rule `#[ignore]` until a database
server is available, in
[its conformance target](../crates/happenstance-postgres/tests/postgres_conformance.rs).
It has to name `__conformance_fixture`, which CF-41 leaves out of the promise, so
you keep it in step with each testkit release yourself.

## Add the other families

The one line runs the event-store family. Three more exist, each with its own
macro. Add the ones that fit your store.

- **Concurrency.** `event_store_concurrency_conformance!` races
  `concurrency::CONTENDERS` threads against one store. Among its rules,
  `exactly_one_of_n_contenders_commits` checks that when many writers race on the
  same condition, exactly one wins
  ([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

  It is opt-in, and only a store whose handle is `Send` (safe to move between
  threads) can run it. Its default emitter also needs `tokio`'s `rt-multi-thread`
  feature.

  Your fixture must be able to open one more handle than there are contenders. A
  connection pool smaller than that hangs rather than failing, because no rule
  may read a clock
  ([CF-33](../spec/SPECIFICATION.md#67-benchmarks-are-not-conformance)), so
  there is no timeout to fail it.
- **Model.** `event_store_model_conformance!` generates random sequences of
  operations and checks your store against a simple model of the log. It needs
  the testkit's `proptest` feature on your dev-dependency, and it skips on a
  fixture that declines `READ_YOUR_OWN_WRITES`. The SQLite adapter runs it beside
  the event-store family in
  [its conformance target](../crates/happenstance-sqlite/tests/conformance.rs).
- **Projections.** If your crate also implements `ProjectionStore`, the store
  for read models, `projection_store_conformance!` takes a `ProjectionFixture`
  and checks that contract. It needs `ProjectionProbe` implemented beside your
  store, behind a `conformance` feature of your own crate.

  The testkit's [crate documentation](../crates/happenstance-testkit/src/lib.rs)
  walks the six steps and the manifest under *Writing a projection adapter from
  outside this workspace*.
  [The outside projection adapter](../examples/outside-projection-adapter/tests/outside_projection_conformance.rs)
  was written from those steps alone, and runs the family in one line.

## Pin the suite

A new rule can arrive in a minor release of the testkit
([CF-29](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit)), and
it can turn your passing build red. So pin the testkit exactly, with `=`, in your
`dev-dependencies`. Then you choose when to take on a new bar.

That advice is
[CF-30](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit), which
is a recommendation rather than a requirement. It costs your users nothing,
because Cargo never resolves a library's dev-dependencies for the people who
depend on it.

When an existing rule's meaning changes, the testkit takes a major release that
links the decision behind it
([CF-31](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit)).

## What a green run does not tell you

A passing suite is strong evidence, but it has blind spots you should know about.

- **That `reopen` is honest.** Make your `reopen` really close what the fixture
  holds and read the storage again, because nothing else will catch it. An empty
  `reopen` over a store that loses everything on restart passes every rule. The
  testkit records that blind spot in
  `reopen_over_claiming_is_undetectable_and_this_is_the_record`.
- **That your store is fast enough.** Performance is measured by a separate
  benchmark harness, which no adapter has to pass
  ([CF-34](../spec/SPECIFICATION.md#67-benchmarks-are-not-conformance)).
- **That the rules you skipped hold.** A skipped rule passed without running.
  Read the skip lines before you quote the pass count.
