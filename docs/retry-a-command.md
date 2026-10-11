# Retry a command

> **Answers:** `how-to` — How do I retry a command without applying it twice?

A command gets retried from two places, and they need different things from you.

- **The command loop retries on contention.** Another writer landed inside your
  boundary, or the store was too busy to take the write. The loop reads again,
  decides again, and appends again. You give it a bound.
- **Your client retries on silence.** A timeout, a dropped connection, a
  message delivered twice. The same command arrives a second time, and it may
  already have been applied. You make that safe by putting the delivery in the
  boundary.

Do both. The first keeps a busy boundary from failing commands it could have
admitted. The second keeps a redelivery from applying a command twice.

## Bound the loop's retry

Pass `commit` a `Retry`. It has no default (`retry_has_no_default`), so the
worst case is a number you wrote down. Spell it as a constant: `Retry::attempts` is a `const fn`, so a zero
fails when the program is compiled, not when it runs.

The loop retries two answers from the store, and both count against the same
bound:

- a violated condition, which is how a store reports that something matching
  your boundary landed after your read
  ([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics), checked by
  `condition_rejection_is_reported_as_condition_violated`);
- a busy refusal, which the store reports only when the batch took no effect
  ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so),
  checked by `a_busy_append_left_nothing_behind`).

Each retry reads the store again and folds a fresh copy of your boundary. It
then calls your decision again, and never re-sends the previous attempt's
events. `retry_refolds_from_pristine_state` and
`retry_does_not_resubmit_the_previous_batch` in
[the command loop's tests](../crates/happenstance/tests/command_loop.rs) check
that. A busy refusal takes the same path
(`a_busy_refusal_is_re_decided_and_the_attempts_say_so`, in
[the retry tests](../crates/happenstance/tests/retry_without_a_database.rs)).
A read failure and any other store failure come back at once and are never
retried (`an_injected_read_failure_is_not_retried_as_contention`,
`a_store_failure_is_still_not_retried`).

When the bound runs out you get `CommandError::Exhausted`, and its `source` is
the last refusal (`exhaustion_is_bounded_and_named`,
`busy_on_every_attempt_is_exhausted_and_carries_the_busy_refusal`). Ask
`is_busy()` which one it was. A boundary that stayed contended and a store that
stayed busy call for different replies:

```rust
use core::num::NonZeroU32;
use happenstance::{AppendError, CommandError, ConditionViolated, Retry};

// What you hand `commit` as its third argument. Zero does not compile.
const RETRY: Retry = Retry::attempts(NonZeroU32::new(3).unwrap());

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

    let lost: Error = CommandError::Append(AppendError::Store(locked()));
    assert_eq!(reply(&lost), Reply::Unknown);
}
```

Leave room in the bound for a redelivery that races its original. The next
section shows that its first append is refused. It is the second attempt that
reads the original and settles. With `Retry::once()` that delivery comes back as
`Exhausted` instead, which is the `Retry::once()` case of
`exhaustion_is_bounded_and_named`.

## Put the request in the boundary

Give every delivery of a command an id that the client chooses and sends again
when it retries. Then do three things with it:

1. **Tag** every event the command writes with the request id.
2. **Query** for it: add one item for this request's events to the
   boundary, beside the items your rule already needs.
3. **Fold** it first: if the read found this request's events, the command has
   already been applied, and you decide nothing.

The program below is a deposit, which has no natural key. Two deposits of the
same amount into the same account are indistinguishable in the domain, so only
the request can tell a redelivery from a second deposit. Two deliveries of one
request read before either appends, which is the race a client retry produces:

```rust
use happenstance::{AppendCondition, AppendError, Event, EventStore, Query};
use happenstance::{MemoryEventStore, QueryItem, Tags, read_decision_model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let account = Tags::from_pairs([("account", "a-1")])?;
    let request = Tags::from_pairs([("request", "r-9")])?;
    // Your rule: no deposit into a closed account.
    let closed = QueryItem::new(["AccountClosed"], account)?;
    // The step this page adds: this delivery's own events.
    let delivered = QueryItem::new(["Deposited"], request.clone())?;
    let boundary = Query::from_items([closed, delivered])?;
    let tags = Tags::from_pairs([("account", "a-1"), ("request", "r-9")])?;
    let deposit = Event::new("Deposited", &b"{}"[..])?.with_tags(tags);

    // Two deliveries of r-9, and both read before either appends.
    let (_, first_read) = read_decision_model(&store, &boundary).await?;
    let (_, second_read) = read_decision_model(&store, &boundary).await?;
    let first = AppendCondition::new(boundary.clone()).after_opt(first_read);
    store.append(&[deposit.clone()], Some(&first)).await?;
    let second = AppendCondition::new(boundary.clone()).after_opt(second_read);
    let refused = store.append(&[deposit], Some(&second)).await;
    assert!(matches!(refused, Err(AppendError::ConditionViolated(_))));

    // The retry reads again, and its fold finds r-9 already applied.
    let (seen, _) = read_decision_model(&store, &boundary).await?;
    let applied = seen.iter().any(|event| event.tags().contains_all(&request));
    assert!(applied, "the second delivery decides nothing");
    assert_eq!(store.snapshot().len(), 1, "one deposit, not two");
    Ok(())
}
```

This program uses the append condition directly, so you can see what the
request item does. With `commit`, the request is one more decision model in
the boundary tuple, scoped to the request tag. The loop retries the refused
append for you, and your decision checks that model before anything else.

Without the `delivered` item, the boundary holds only `AccountClosed`. A
deposit appended under it does not match its own condition, so the second
delivery is admitted and the account is credited twice. A store does not
protect you here: it must land that second copy
([ES-24](../spec/SPECIFICATION.md#es-24--resolving-an-unknown-outcome), checked
by `reissued_batch_conditioned_on_other_events_lands_twice`).

When a decision finds its request already applied, it has two ways to say so.
Returning no events gives you `CommandOutcome::Nothing`, which is a success and
appends nothing (`a_decision_that_emits_nothing_is_a_success`). Returning a
refusal of your own carries what the first delivery did back to the client.
Choose the refusal when the client needs that answer, and turn it into a
success where you reply.

## When you cannot tell whether it landed

A dropped future does not mean the append failed. A store may have committed it
anyway
([ES-23](../spec/SPECIFICATION.md#es-23--cancellation-outcome-is-unspecified-and-that-is-the-contract)).
`CommandError::Append` carrying the store's own error is the same situation
reported as a value. The loop does not retry it, because deciding again could
apply the command twice (`a_store_failure_is_still_not_retried`).

So resend the same request with the same id. If the first attempt landed, the
read finds its events and the decision settles as above. If it did not land, the
command is applied now. Either way it is applied once. A new id makes the resend
a second command.

## See it in a running program

[The handle registry](../examples/handles-and-quotas/src/main.rs) does all of
this against SQLite. `RequestId` is the client's id, `Request` is the decision
model scoped to it, and `settle` turns the replay refusal into a success. Its
`one_command_delivered_twice` sends one claim twice and checks that the log grew
by one event. `a_replayed_delivery_appends_nothing`, in
[its tests](../examples/handles-and-quotas/tests/runs.rs), checks the same from
the transcript.
