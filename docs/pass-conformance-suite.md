# Pass the conformance suite

> **Answers:** `how-to` — How do I make my adapter pass the conformance suite?

An adapter counts once it has run the conformance suite against its own storage
and passed. Compiling is not enough. The testkit's
[crate documentation](../crates/happenstance-testkit/src/lib.rs) states that
bar at the top. Read
[what to read before you write an adapter](adapter-reading-order.md) first. This
page starts where that one stops: your store implements the port, and you want
the suite green.

You write one type and one line. The type is a **fixture**, which tells the
suite how to get a store and what that store can do. The line hands the fixture
to the suite. Everything else on this page is what to put in the fixture, how
to read what the suite says back, and which other families to add.

## Write the fixture

The suite never takes your store directly. It takes a fixture. Give each
fixture instance a backing store of its own, such as a fresh temporary file or
a throwaway schema, and make each `connect` open a handle onto that store
([CF-15](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
`two_fixture_instances_observe_none_of_each_others_appends`). Two instances that
point at one file fail that rule.

Then answer the eight constants. Two have no default, so you cannot forget them:

- **`SECOND_HANDLE`** is not a choice. The suite opens two handles onto one
  store to catch a store that is only correct within one session
  ([CF-16](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
  `two_handles_observe_each_others_appends`). Declining it does not skip that
  rule. The rule fails and quotes your reason.
- **`REOPEN`** asks whether your fixture can show the suite what survives a
  restart. If your store keeps its data somewhere that outlives the process,
  support it and override `reopen`; the clause says what the override owes. An
  in-memory store declines, and says why
  ([CF-17](../spec/SPECIFICATION.md#63-the-fixture-contract), provisional;
  `acknowledged_writes_survive_a_reopen`).

Two more have a default, and you should still write them. `MID_BATCH_FAULT` and
`READ_FAULT` default to declined, in the testkit's own words. The suite adds a
test, `every_declined_capability_is_stated_by_this_fixture`, that fails while
either one still carries those words
([CF-18](../spec/SPECIFICATION.md#63-the-fixture-contract)). Write why *your*
store cannot be faulted part way through a batch or a read, or support the
capability and override the matching `arm_` method. Supporting
`MID_BATCH_FAULT` is a promise about where your fault fires
([CF-39](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
`arming_a_mid_batch_fault_makes_the_append_fail`), so read the clause before
you claim it.

One more is defaulted the other way. `READ_YOUR_OWN_WRITES` defaults to
supported: an append you were told succeeded shows up in your next read through
the same handle. A store whose reads can lag an acknowledged append declines
it, with its own reason. The PostgreSQL and Neon fixtures do, because a read
there waits for a server-wide snapshot frontier. Only the model family needs it,
and it reports a skip without it (`Fixture::READ_YOUR_OWN_WRITES`, read by
`ops_agree_with_the_model`).

Last come three ceilings: `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and
`MAX_EVENTS_PER_BATCH`. They are facts about your store, not trades, so they
take a number or `None` and no reason. The suite tests a number you state from
both sides of the ceiling
([CF-40](../spec/SPECIFICATION.md#63-the-fixture-contract), provisional;
`append_reports_exceeded_store_limits`). Mirror your adapter's own constants
rather than typing the numbers again, as the SQLite fixture does in
[its support module](../crates/happenstance-sqlite/tests/support/mod.rs).

The program below is a whole fixture for a store that is not the reference
store. `GappyMemoryStore` hands out positions seven apart, so it leaves gaps.
Your handle would wrap a connection or a pool where this one wraps an `Arc`.
The one macro line is the whole of what goes in your `tests/conformance.rs`:

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

In your crate, `cargo test` turns that macro line into one test per rule. A
doctest is not built as a test binary, so the tests the line emits do not exist
here. `main` runs the same rules instead, the way the `emit_blocking` harness
does, and checks that every rule passed or was skipped, either for a
capability the fixture declined or for the ceilings it states none of. It also
checks that no skip carries the testkit's default wording. One emitted test has no rule behind it and so does not run
here: `every_declined_capability_is_stated_by_this_fixture`. Your own
`cargo test` runs it.

The gaps cost nothing. Positions may have gaps
([VT-11](../spec/SPECIFICATION.md#vt-11--positions-are-unique-strictly-increasing-in-assignment-order-and-may-have-gaps)),
and no rule asserts a literal position
([CF-6](../spec/SPECIFICATION.md#61-the-suites-own-proof-obligation), checked by
`mutation_coverage::conformant_variants_pass_everything`). The testkit runs
the whole suite against this store with this stride in its own tests. If you
write tests of your own beside the suite, hold them to the same rule: compare
against positions your store returned, never against `1`, `2`, `3`.

Real fixtures to copy from: SQLite's, over a temporary file, which supports
`REOPEN` and states all three ceilings, in
[its support module](../crates/happenstance-sqlite/tests/support/mod.rs); the
reference `MemoryFixture`, in
[the testkit's fixtures](../crates/happenstance-testkit/src/fixtures.rs); and
the PostgreSQL fixture, which aims each instance at a throwaway schema, in
[its support module](../crates/happenstance-postgres/tests/support/mod.rs).

## Read a skip

A rule that needs a capability your fixture declined still runs as a test. It
passes, and prints one line naming the capability and your reason
([CF-18](../spec/SPECIFICATION.md#63-the-fixture-contract), checked by
`mutation_coverage::capability_skips_are_reported`). The suite never drops a
rule from the binary, because a missing rule looks the same in CI as one that
passed.

libtest hides what a passing test prints. A plain `cargo test` shows the pass
count and none of those lines, whatever you declined. Run it with
`-- --show-output` to read them, and do that in CI if you want anyone else to
read them. `RuleOutcome::report` records this limit where it is implemented.

Each declined capability is a trade, and the reason is the only record of it.
Write the reason for someone choosing your adapter, not for yourself. Nothing
checks that a reason is good: `"n/a"` passes.

## Read a failure

A failing rule panics, and the test that fails carries the rule's name. Look
the rule up in the testkit's `rules` module. Its documentation names the wrong
implementations it rejects, and one of them is often yours.

The program below is a common one. Every `connect` opens a store of its own, the
way every connection to SQLite's `:memory:` opens a database of its own. The
isolation rule passes, because two fixture instances really do share nothing.
The second-handle rule fails, because two handles onto one instance share
nothing either:

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
Declining one to turn a red rule green is a different act. `SECOND_HANDLE`
refuses it outright, as above. The others record it in every skip line, where
the people choosing your adapter can read it.

If you believe a rule is wrong, the fix belongs in the testkit, in the same
change as the wrong implementation the rule must still reject. Never skip it.
`CONTRIBUTING.md`, under
[Writing an adapter](../CONTRIBUTING.md#writing-an-adapter), says the same.

## Choose the harness

The one-line form wraps each rule in `#[tokio::test]`, so your
`dev-dependencies` need `tokio` with the `macros` and `rt` features. The
wrapper is a parameter you choose, not the testkit's
([CF-23](../spec/SPECIFICATION.md#64-runtime-independence)). The long form
names it:
`event_store_conformance!(mod_name = …, emit = …, fixture = …)`.

- `emit_blocking` (`happenstance_testkit::emit_blocking`) needs no runtime at
  all.
- `emit_wasm` (`happenstance_testkit::emit_wasm`) runs under `wasm-bindgen-test`, for a
  `wasm32` store. The Cloudflare adapter uses it in
  [its conformance target](../crates/happenstance-cloudflare/tests/durable_object_conformance.rs).

Those names are promised: renaming one is a major release of the testkit
([CF-41](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit),
checked by `the_promised_emitters_are_exactly_the_pinned_list`). An emitter you
write yourself is not covered by that promise. The PostgreSQL adapter writes one
to mark every rule `#[ignore]` until a server is available, in
[its conformance target](../crates/happenstance-postgres/tests/postgres_conformance.rs).
It has to name `__conformance_fixture`, which CF-41 leaves out of the promise,
so you keep it in step with each testkit release yourself.

## Add the other families

The one line runs the event-store family. Three more exist, and each has its
own macro.

- **Concurrency.** `event_store_concurrency_conformance!` races
  `concurrency::CONTENDERS` threads against one store. Among its rules,
  `exactly_one_of_n_contenders_commits` asks
  [ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)'s question of a
  store under real contention. It is opt-in, and only a store whose handle is
  `Send` can run it. Its default harness also needs `tokio`'s `rt-multi-thread`
  feature. Your fixture must open one more handle than there are
  contenders. A pool smaller than that hangs instead of failing, because no rule
  may read a clock
  ([CF-33](../spec/SPECIFICATION.md#67-benchmarks-are-not-conformance)), so
  there is no timeout to fail it.
- **Model.** `event_store_model_conformance!` generates operation sequences and
  checks your store against a model of the log. It needs the testkit's
  `proptest` feature on your dev-dependency, and it skips on a fixture that
  declines `READ_YOUR_OWN_WRITES`. The SQLite adapter mounts it beside
  the event-store family in
  [its conformance target](../crates/happenstance-sqlite/tests/conformance.rs).
- **Projections.** If your crate also implements `ProjectionStore`,
  `projection_store_conformance!` takes a `ProjectionFixture` and checks that
  port. It needs `ProjectionProbe` implemented beside your store, behind a
  `conformance` feature of your own crate. The testkit's
  [crate documentation](../crates/happenstance-testkit/src/lib.rs) walks the six
  steps and the manifest under *Writing a projection adapter from outside this
  workspace*.
  [The outside projection adapter](../examples/outside-projection-adapter/tests/outside_projection_conformance.rs)
  was written from those steps alone, and runs the family in one line.

## Pin the suite

A new rule can arrive in a minor release of the testkit
([CF-29](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit)), and
it can turn your passing build red. Pin the testkit exactly, with `=`, in your
`dev-dependencies`, so you choose when you take a new bar. That advice is
[CF-30](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit),
which is prose rather than a requirement, and it costs nothing downstream:
Cargo never resolves a library's dev-dependencies for its users. When a rule's
meaning changes, the testkit takes a major release that cites the decision
behind it ([CF-31](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit)).

## What a green run does not tell you

- **That `reopen` is honest.** Make your `reopen` really close what the
  fixture holds and read the medium again, because nothing else will catch it.
  An empty `reopen` over a store that loses everything on restart passes every
  rule, and the testkit records that in
  `reopen_over_claiming_is_undetectable_and_this_is_the_record`.
- **That your store is fast enough.** Performance is measured by a separate
  harness, which no adapter has to pass
  ([CF-34](../spec/SPECIFICATION.md#67-benchmarks-are-not-conformance)).
- **That the rules you skipped hold.** A skipped rule passed without running.
  Read the skip lines before you quote the pass count.
