# Retry a command

> **Answers:** `how-to` — How do I retry a command safely, without applying it twice?

Sooner or later a command will not go through the first time. Two very
different things can cause that, and each needs its own fix. This page shows
both, and you will want both in any application that takes requests from a
network.

- **Another writer got there first.** Someone else appended an event inside
  your decision's [consistency boundary](what-is-dcb.md) between your read and
  your write, or the store was too busy to take the write. Nothing of yours was
  written. The fix is to read again, decide again and append again, a bounded
  number of times. The command loop does this for you; you choose the bound.
- **You do not know whether your write landed.** A network timeout, a dropped
  connection, or a message queue that delivers the same message twice. The same
  command arrives again, and it may already have been applied. The fix is to
  give each request an id and make the decision check for it.

The first keeps a busy boundary from failing commands it could have accepted.
The second keeps a resent command from being applied twice, which matters most
for things like payments, where "twice" is a real bug.

## Bound the loop's retry

The typed layer's command loop reads the events your decision needs, calls your
decision, and appends the result with a [condition](what-is-dcb.md#each-decision-chooses-its-own-boundary).
If the store refuses the append, it goes round again. The function is `commit`,
or `commit_with` if you want to choose the encoding yourself (`commit` uses
JSON, which is on by default).

Both take a `Retry`, the most attempts the loop may make, and it has no default.
That is deliberate: the worst case is then a number you wrote down, not one
you discovered in production. Spell it as a constant, as
[the course subscriptions example](../examples/course-subscriptions/src/main.rs)
does with `ATTEMPTS`.

`Retry::attempts` takes a `NonZeroU32`, a number the type system guarantees is
not zero, and it is a `const fn`, so it can run while your program compiles. A
zero therefore stops the build, with error `E0080`, instead of failing at run
time:

```rust,compile_fail,E0080
use core::num::NonZeroU32;
use happenstance::Retry;

const RETRY: Retry = Retry::attempts(NonZeroU32::new(0).unwrap());

fn main() {
    let _ = RETRY;
}
```

### What the loop retries, and what it does not

The loop retries two answers from the store, and both count against the same
bound:

- **A violated condition.** This is how a store says that an event matching
  your query landed after your read
  ([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).
- **A busy refusal.** The store was too busy to take the write. A store only
  gives this answer when the append wrote nothing at all
  ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)),
  which is what makes it safe to try again.

Each retry starts from scratch. The loop reads the store again, rebuilds your
decision's state from a fresh copy, and calls your decision again. It never
re-sends the events the previous attempt decided on, because those were decided
on information that is now out of date.

Any other failure comes back to you straight away and is never retried. That
includes a failed read, and an append that failed for the store's own reasons,
such as a lost connection. The last section explains why that second case is not
retried.

### When the attempts run out

When every attempt has been refused, you get `CommandError::Exhausted`. Its
`source` is the last refusal, and `is_busy()` tells you which kind it was. The
difference matters to whoever sent the command: a store that stayed busy needs
the client to back off before resending, while a boundary that stayed contended
can be resent straight away.

The program below turns each outcome into a reply for the client. The
`Unknown` case is the one the rest of this page is about.

```rust
use happenstance::{AppendError, CommandError, ConditionViolated};

/// What the client that sent the command is told.
#[derive(Debug, PartialEq, Eq)]
enum Reply {
    /// The store stayed busy on every attempt: back off, then resend.
    Busy,
    /// Writers kept landing in the boundary: resend.
    Contended,
    /// The append may have landed: resend the same request id.
    Unknown,
    /// A refusal or a failure, reported as it is.
    Failed,
}

fn reply<E, D>(err: &CommandError<E, D>) -> Reply
where
    E: core::error::Error + 'static,
    D: core::error::Error + 'static,
{
    match err {
        CommandError::Exhausted { source, .. } if source.is_busy() => Reply::Busy,
        CommandError::Exhausted { .. } => Reply::Contended,
        CommandError::Append(AppendError::Store(_)) => Reply::Unknown,
        _ => Reply::Failed,
    }
}

fn main() {
    type Error = CommandError<std::io::Error, std::io::Error>;
    let locked = || std::io::Error::other("database is locked");

    let busy: Error = CommandError::Exhausted {
        attempts: 3,
        source: AppendError::Busy(locked()),
    };
    assert_eq!(reply(&busy), Reply::Busy);

    let hot: Error = CommandError::Exhausted {
        attempts: 3,
        source: ConditionViolated::unspecified().into(),
    };
    assert_eq!(reply(&hot), Reply::Contended);

    let reset = std::io::Error::other("connection reset after COMMIT");
    let lost: Error = CommandError::Append(AppendError::Store(reset));
    assert_eq!(reply(&lost), Reply::Unknown);
}
```

Leave at least one spare attempt in the bound. When a resent request races its
own original, the next section shows that the resend's first append is refused,
and it is the second attempt that sees the original and settles. With
`Retry::once()`, that resend comes back as `Exhausted` instead.

## Put the request in the boundary

To make a resend safe, the decision has to be able to see whether this exact
request has already been applied. The way to let it see that is to put the
request into the events, and into what the decision reads.

Have the client choose an id for each request, and send the same id again when
it retries. Then do three things with it:

1. **Tag** every event the command writes with the request id, for example
   `request=r-9`.
2. **Read** it. With `commit`, add a decision model for the request, scoped to
   the request tag, beside your rule's own decision models in the boundary you
   pass in (the boundary can be a tuple of several models).
3. **Check it first.** If the read found events from this request, the command
   has already been applied, and the decision appends nothing.

Put the request tag on the command's own events and nowhere else. Every request
has a different id, so it is the tag with the most distinct values you will
ever write, and that has a storage and indexing cost. The specification weighs
that cost when it rules out putting an identity tag on every event
([ES-24](../spec/SPECIFICATION.md#es-24--resolving-an-unknown-outcome)).

### A worked example: a deposit sent twice

The program below is a bank deposit, which is the hard case. Two deposits of
the same amount into the same account look identical, so nothing in the domain
can tell a resend from a genuine second deposit. Only the request id can.

`attempt` is one pass of the command loop, written out by hand with the append
condition so you can see each step. Its rule already reads `Deposited` events,
so the request's own event is inside the boundary already. When your rule does
not read the events it writes, add the request's decision model from step 2.

Two deliveries of one request race, the loser is retried, and then the client
resends once more:

```rust
use happenstance::{AppendCondition, AppendError, Event, EventStore, Query};
use happenstance::{MemoryEventStore, QueryItem, Tags, read_decision_model};

type Failure = Box<dyn core::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Failure> {
    let store = MemoryEventStore::new();
    let account = Tags::from_pairs([("account", "a-1")])?;
    let request = Tags::from_pairs([("request", "r-9")])?;
    // Your rule's boundary: the account, closed or not, and its deposits.
    let types = ["AccountClosed", "Deposited"];
    let boundary = Query::from_items([QueryItem::new(types, account)?])?;
    let tags = Tags::from_pairs([("account", "a-1"), ("request", "r-9")])?;
    let deposit = Event::new("Deposited", &b"{}"[..])?.with_tags(tags);

    // Two deliveries of r-9 race: the second reads before the first lands.
    let (_, stale) = read_decision_model(&store, &boundary).await?;
    assert_eq!(attempt(&store, &boundary, &request, &deposit).await?, 1);
    let guard = AppendCondition::new(boundary.clone()).after_every_guard_opt(stale);
    let refused = store.append(&[deposit.clone()], Some(&guard)).await;
    assert!(matches!(refused, Err(AppendError::ConditionViolated(_))));

    // The loser's retry, then a client's late resend. Both read r-9.
    assert_eq!(attempt(&store, &boundary, &request, &deposit).await?, 0);
    assert_eq!(attempt(&store, &boundary, &request, &deposit).await?, 0);
    assert_eq!(store.snapshot().len(), 1, "one deposit, not two");
    Ok(())
}

/// One attempt: read, fold, decide, and append under the read.
async fn attempt(
    store: &MemoryEventStore,
    boundary: &Query,
    request: &Tags,
    deposit: &Event,
) -> Result<usize, Failure> {
    let (seen, upto) = read_decision_model(store, boundary).await?;
    // The request first: if r-9 already landed, append nothing.
    if seen.iter().any(|event| event.tags().contains_all(request)) {
        return Ok(0);
    }
    let guard = AppendCondition::new(boundary.clone()).after_every_guard_opt(upto);
    store.append(&[deposit.clone()], Some(&guard)).await?;
    Ok(1)
}
```

### Why both the condition and the check are needed

The condition and the request check do different jobs, and neither can do the
other's.

- **The condition stops the race.** The second delivery read before the first
  one landed, so its append is refused rather than added beside the first.
- **The check stops the repeat.** After the refusal, the loser's retry reads
  again and now sees the first deposit. So does the client's late resend.
  Neither has anything to conflict with, so the condition would let both
  through. Only the check sees that `r-9` is already done.

Delete the request check from `attempt` and the account is credited three
times.

### A note on resending a batch unchanged

The specification also guarantees that resending exactly the same append, byte
for byte, under exactly the same condition, lands at most once, provided the
condition's query matches the events being appended
([ES-24](../spec/SPECIFICATION.md#es-24--resolving-an-unknown-outcome)). That is
a different situation from this page's. The command loop never resends a batch
unchanged: every attempt reads again and decides again, so it relies on the
request check, not on that guarantee.

### Telling the client what happened

In [the handle registry example](../examples/handles-and-quotas/src/main.rs),
linked again below, the request's decision model is scoped to the request tag
alone. It finds this request's events whatever your rule's own query covers, so
it works even when the rule does not read the event type it writes.

When a decision finds its request already applied, it can say so in two ways:

- **Return no events.** `commit` then returns `CommandOutcome::Nothing`, which
  is a success and appends nothing.
- **Return a refusal of your own** that carries what the first delivery did, so
  the client gets the same answer it would have got the first time. Turn that
  refusal into a success where you build the reply.

Choose the refusal when the client needs the original answer, for example the
id of the thing the first delivery created.

## When you cannot tell whether it landed

A timeout or a dropped connection does not mean the append failed. The store
may have committed it just before the connection went. The specification says
outright that the outcome of a cancelled append is unknown
([ES-23](../spec/SPECIFICATION.md#es-23--cancellation-outcome-is-unspecified-and-that-is-the-contract)).
`CommandError::Append` carrying the store's own error is the same situation,
reported as a value instead of a dropped future.

This is why the loop does not retry it. If the first append did land, deciding
again without a request id would apply the command a second time.

So resend the same request with the same id:

- If the first attempt landed, the read finds its events, and the decision
  settles as in the example above.
- If it did not land, the command is applied now.

Either way it is applied once. Resending with a new id would make it a second
command.

## See it in a running program

[The handle registry example](../examples/handles-and-quotas/src/main.rs) does
all of this against SQLite. `RequestId` is the client's id, `Request` is the
decision model scoped to it, and `settle` turns the "already applied" refusal
into a success.

Its `one_command_delivered_twice` sends one claim twice and
checks that the log grew by one event.
`a_replayed_delivery_appends_nothing`,
in [its tests](../examples/handles-and-quotas/tests/runs.rs), checks the same
from the program's output.

## How this page is checked

Every program above is compiled and run by this repository's tests. The
behaviour the prose describes is also covered by named tests, if you want to
read them:

- `retry_has_no_default`, in the command loop's own source
  ([`command.rs`](../crates/happenstance/src/command.rs)).
- In [the command loop's tests](../crates/happenstance/tests/command_loop.rs):
  `retry_refolds_from_pristine_state`,
  `retry_does_not_resubmit_the_previous_batch`,
  `exhaustion_is_bounded_and_named` (including its `Retry::once()` case) and
  `a_decision_that_emits_nothing_is_a_success`.
- In [the retry tests](../crates/happenstance/tests/retry_without_a_database.rs):
  `a_busy_refusal_is_re_decided_and_the_attempts_say_so`,
  `an_injected_read_failure_is_not_retried_as_contention`,
  `a_store_failure_is_still_not_retried` and
  `busy_on_every_attempt_is_exhausted_and_carries_the_busy_refusal`.
- In the conformance suite every store passes:
  `condition_rejection_is_reported_as_condition_violated` (ES-25),
  `a_busy_append_left_nothing_behind` (ES-43) and
  `reissued_conditional_batch_lands_once` (ES-24).
