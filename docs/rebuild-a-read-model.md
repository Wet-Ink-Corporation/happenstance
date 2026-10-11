# Rebuild a read model

> **Answers:** `how-to` — How do I rebuild a read model from the log?

A read model is derived from the log, so you can always throw it away and build
it again. How you do that depends on why you are doing it.

- **The fold is right and the rows are not.** A table was dropped, a bad deploy
  wrote nonsense, or you want a clean copy. Reset the view and replay it under
  the same id.
- **The fold itself changed.** It counts something different, or it reads
  events it used to ignore. Build the new version beside the old one under a
  second id, switch your readers to it, then retire the old one.

The first is quicker. The second keeps a correct answer on screen the whole
time, if you promote it in the order
[below](#build-the-new-version-beside-the-old) gives. Both use the same two
port calls, `reset` and `commit`, and the projection runner, `run_projection`.

The runner sits behind the `unstable-projection` feature and makes no semver
promise, because the `apply` it ships is going to change
([ADR-0074](../.kb/decisions/0074-projection-apply-is-async.md)). The port beneath it,
`ProjectionStore`, is frozen. The fences on this page call only the port, on
`MemoryProjectionStore`, so they compile without the feature.
[The rebuilding example](../examples/rebuilding-read-models/src/main.rs) runs
every step below with the runner, against SQLite, and it is the program to read
next to this page.

## Choose the rebuild

The runner always commits with `Authority::Live` (the chunk commit at
[`runner.rs:624`](../crates/happenstance/src/runner.rs) names it). It cannot
say that the rows it is writing are half built. A run that stops after one
chunk of two leaves the checkpoint at `Live`, through the last
good position: that is the chunk-of-two half of
`decode_failure_names_its_position_and_rolls_back`, in
[the runner's tests](../crates/happenstance/tests/projection_runner.rs). The
port can say *rebuilding*
([PS-24](../spec/SPECIFICATION.md#47-checkpoint-semantics), checked by
`rebuilding_is_distinguishable_from_live`), but whether the runner ever does is
still open. So while a reset view replays, anything reading it sees a partial
answer that claims to be current. If nobody reads the view during the replay,
or a partial answer is harmless, reset in place. If not, use a second id.

A changed fold always needs one of the two. The checkpoint is keyed by the
`ProjectionId` alone. Nothing notices that the events a projection asks for, or
what it does with them, have changed. A projection that starts reading a new
event type resumes past every older event of that type and never sees them. A
derived id that changes with the query is the chosen remedy
([PS-25](../spec/SPECIFICATION.md#47-checkpoint-semantics)), but it is
provisional, it is not built, and no rule checks it yet. Until it is, the new id
is yours to choose.

## Reset the view

`reset` takes a batch, the same kind `commit` takes, and you fill it with your
own deletes. The port does not know which rows belong to a view, so it cannot
clear them for you. What you get in exchange is that a crash cannot leave an
empty table whose checkpoint says it is current
([PS-16](../spec/SPECIFICATION.md#46-reset), checked by
`reset_clears_rows_and_checkpoint_together`). That clause is provisional: the
shape that would falsify it is a store whose clearing statement cannot share
the checkpoint's transaction. Resetting one view also leaves every other view in
the same store where it was, so you can rebuild one view without pausing the
rest ([PS-17](../spec/SPECIFICATION.md#46-reset), checked by
`reset_is_scoped_to_one_projection`).

After a reset the checkpoint reads `Checkpoint::NeverRun`, so the next run
applies everything again, the first event included
([PS-20](../spec/SPECIFICATION.md#46-reset); the runner's
`first_run_starts_from_the_beginning`). The shortcut that looks the same, a
commit of an empty batch at the first position, is not: what it buys you is a
first event that is never applied
([PS-19](../spec/SPECIFICATION.md#46-reset), checked by
`reset_is_not_commit_at_first`). The program below does both.
`replay_reads` mirrors the runner's resume rule
([`runner.rs:538-548`](../crates/happenstance/src/runner.rs)), so the fence
models the runner rather than exercising it, and counts what each replay would
read:

```rust
use happenstance::{Authority, Checkpoint, Event, EventStore, ProjectionId};
use happenstance::{MemoryEventStore, MemoryProjectionStore, ProjectionStore};
use happenstance::{Query, ReadOptions, SequencePosition, collect};

type Failure = Box<dyn core::error::Error>;

const STOCK: ProjectionId = ProjectionId::from_static("stock_on_hand");
const STATUS: ProjectionId = ProjectionId::from_static("order_status");

#[tokio::main]
async fn main() -> Result<(), Failure> {
    let log = MemoryEventStore::new();
    let placed = Event::new("OrderPlaced", &b"{}"[..])?;
    log.append(&[placed.clone(), placed.clone(), placed], None).await?;
    let head = log.snapshot().last().ok_or("the log is empty")?.position;

    // A view that has caught up with the log.
    let views = MemoryProjectionStore::new();
    let mut rows = views.begin().await?;
    rows.write("sku-1", 3);
    views.commit(rows, &STOCK, head, Authority::Live).await?;

    // The reset: your deletes and the checkpoint, as one unit.
    let mut deletes = views.begin().await?;
    deletes.delete_all();
    views.reset(deletes, &STOCK).await?;
    assert!(views.is_empty(), "the rows went with the checkpoint");
    assert_eq!(views.checkpoint(&STOCK).await?, Checkpoint::NeverRun);

    // WRONG - "start again" spelled as a commit at the first position.
    let empty = views.begin().await?;
    let first = SequencePosition::FIRST;
    views.commit(empty, &STATUS, first, Authority::Live).await?;
    let wound = views.checkpoint(&STATUS).await?;
    assert_ne!(wound, Checkpoint::NeverRun);
    // END WRONG.

    // The count below assumes the log starts at the first position.
    let start = log.snapshot().first().ok_or("the log is empty")?.position;
    assert_eq!(start, SequencePosition::FIRST);
    assert_eq!(replay_reads(&log, Checkpoint::NeverRun).await?, 3);
    assert_eq!(replay_reads(&log, wound).await?, 2, "the first event is lost");
    Ok(())
}

/// How many events a run would read after `checkpoint`: from the start when
/// it never ran, and strictly after the checkpoint otherwise. A model of the
/// runner's rule, not a call to the runner.
async fn replay_reads(
    log: &MemoryEventStore,
    checkpoint: Checkpoint,
) -> Result<usize, Failure> {
    let mut options = ReadOptions::new();
    if let Checkpoint::Live { through } | Checkpoint::Rebuilding { through } =
        checkpoint
    {
        options = options.from(through.next().ok_or("no position follows")?);
    }
    Ok(collect(log.read(&Query::all(), options)).await?.len())
}
```

`MemoryProjectionStore` keeps all its rows in one map, so `delete_all` is the
whole of its delete vocabulary. On a real store, the batch carries a `DELETE`
for the view's own table and nothing else. The example's `retire` does exactly
that on SQLite.

## Do not wind the checkpoint back

A stale runner left over from a redeploy cannot drag a view backwards and apply
events twice: its commit comes back as `CommitError::CheckpointRegression`
([PS-22](../spec/SPECIFICATION.md#47-checkpoint-semantics), checked by
`commit_rejects_a_regressing_position`). The same guard means you cannot
rebuild by moving the checkpoint back over rows you kept. Reset is the only way
back:

```rust
use happenstance::{Authority, Checkpoint, CommitError, Event, EventStore};
use happenstance::{MemoryEventStore, MemoryProjectionStore, ProjectionId};
use happenstance::ProjectionStore;

const STOCK: ProjectionId = ProjectionId::from_static("stock_on_hand");

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let log = MemoryEventStore::new();
    let placed = Event::new("OrderPlaced", &b"{}"[..])?;
    log.append(&[placed.clone(), placed], None).await?;
    let positions: Vec<_> =
        log.snapshot().iter().map(|event| event.position).collect();
    let [first, last] = positions[..] else {
        return Err("expected two events".into());
    };

    let views = MemoryProjectionStore::new();
    let caught_up = views.begin().await?;
    views.commit(caught_up, &STOCK, last, Authority::Live).await?;

    // Moving it back is refused: its row is not written, and the checkpoint
    // stays where it was.
    let mut back = views.begin().await?;
    back.write("sku-1", 99);
    let back = views.commit(back, &STOCK, first, Authority::Live).await;
    assert!(matches!(
        back,
        Err(CommitError::CheckpointRegression { current, attempted })
            if current == last && attempted == first
    ));
    assert_eq!(views.get("sku-1"), None, "the refused row was not written");
    let kept = views.checkpoint(&STOCK).await?;
    assert_eq!(kept, Checkpoint::Live { through: last });

    // A reset is the way back, and after it any position is accepted.
    let mut deletes = views.begin().await?;
    deletes.delete_all();
    views.reset(deletes, &STOCK).await?;
    let again = views.begin().await?;
    views.commit(again, &STOCK, first, Authority::Live).await?;
    let restarted = views.checkpoint(&STOCK).await?;
    assert_eq!(restarted, Checkpoint::Live { through: first });
    Ok(())
}
```

## Replay it

After a reset, call `run_projection` again with the same projection. It finds
`NeverRun` and replays everything its query selects, in chunks. The chunk size
changes how often the checkpoint moves, and it must not change the rows
([PS-14](../spec/SPECIFICATION.md#44-read-your-writes), checked by
`rebuild_is_chunk_size_invariant`). Your projection keeps its side of that
bargain by writing everything into the batch and keeping no running state in
the struct. The runner resumes from the checkpoint, so a total held in memory
starts from zero on every run. The example's `StockOnHand` writes a delta
against the row for this reason. Its `reset_and_rebuild` rebuilds at a chunk of
1 and a chunk of 500 and refuses to finish if the rows differ.

Run exactly one runner per `(store, ProjectionId)`. Nothing enforces it, and
two runners moving forward together apply events twice without tripping the
regression check. The runner's own documentation says so under *Exactly one
runner per projection*.

## Build the new version beside the old

When the fold changes, give the new version its own `ProjectionId` and its own
table, and run it from `NeverRun` while the old view keeps answering. The two
views advance on their own schedules, so a backfill never holds the live view
back ([PS-23](../spec/SPECIFICATION.md#47-checkpoint-semantics), checked by
`distinct_projections_advance_independently`). Readers that join the two see
two different moments of the log. The example prints that skew rather than
hiding it.

When the new view has caught up, promote it. Whichever way you switch readers,
start the same way:

1. **Stop the old view's runner, and take it off the schedule.** The old id is
   about to read `NeverRun`. A runner still scheduled under it replays the
   whole log into whatever table now carries the name it writes to.

Then take one of two routes.

**Repoint your readers.** The new view keeps its own table.

2. Point every reader at the new table.
3. Retire the old view: reset its id with a batch that deletes its rows or
   drops its table. Nobody reads it any more, so nobody sees it empty.

**Rename the new table into the old one's place.** Readers keep the name they
already use.

2. Pause the new view's runner.
3. Reset the old id with a batch that carries the swap: `DROP TABLE` on the old
   table, then `ALTER TABLE … RENAME TO` the old name. On SQLite the batch is
   replayed inside the one transaction `reset` opens, and SQLite's `DROP` and
   `ALTER` are transactional, so readers see the old table until the reset
   commits and the new one after it. A swap that fails leaves the old
   table and its checkpoint in place. Push each statement on its own: the
   batch runs one statement per `push`. No conformance rule puts DDL in a
   batch, so check your own store before you rely on this.
4. Redeploy the new view's runner under the same id, writing to the renamed
   table.

In both routes, **keep the new id.** The checkpoint belongs to the id and not
to the table. Changing the id on its own to match the promoted table throws the resume
point away and replays the whole log. And never retire the old view with a
statement that names a table after the swap: once the rename commits, the old
name belongs to the new view, and a `DELETE` against it clears the rows you
just promoted.

[The example's](../examples/rebuilding-read-models/src/main.rs)
`a_third_view_added_late` promotes a view and keeps its id. After the promotion
the view applies one new event rather than the whole log, and
`the_binary_operates_every_view`, in
[its tests](../examples/rebuilding-read-models/tests/runs.rs), checks that
from the transcript. It is one process with no scheduled runners, and it retires
and renames in two separate transactions rather than one. Follow the order
above, not the example's.

## When the replay fails

A replay can stop on an event the fold cannot read, such as a field the log
never carried. The runner stops at the first failure. It discards the chunk it
was building and returns a `ProjectionError` that names the position it
stopped at. The checkpoint stays at the last chunk it committed
([PS-28](../spec/SPECIFICATION.md#48-failure-policy);
`decode_failure_names_its_position_and_rolls_back`, in the runner's tests).
Fix the projection and run it again. It resumes from that
checkpoint and applies nothing twice. That holds only if the fix leaves the
fold alone. A fix that changes which events the fold reads, or how earlier
events fold, is a changed fold: go back to
[Choose the rebuild](#choose-the-rebuild).

Each call to `run_projection` drives one projection, so a failing view stops
only itself. Your other views keep moving as long as you keep calling them
([PS-29](../spec/SPECIFICATION.md#48-failure-policy)). The example's
`one_poisoned_view` breaks one view and runs the other three past it, and
`the_poisoned_view_stops_and_the_others_advance` checks the transcript. The
rule that would check PS-29 against a runner,
`one_poisoned_projection_does_not_stall_the_others`, is not written yet.
Noticing that a view has stopped is your job. The runner returns the error to
its caller and reports it nowhere else.

## What you cannot see while it runs

The runner reports progress once, when it returns. A long replay looks the same
as a hang from outside. What you can watch is the checkpoint: it moves once per
chunk, and another handle on the same store can read it with
`ProjectionStore::checkpoint`. To show "how far along", compare that position
with the head of the log yourself. The runner's documentation says this under
*What can be seen while it runs*; no clause or rule covers it.
