# Rebuild a read model

> **Answers:** `how-to` — How do I rebuild a read model from the log?

A [read model](event-sourcing-and-cqrs.md#cqrs-one-side-decides-the-other-side-answers)
is built entirely from the events in the log, so you can always throw it away
and build it again. You will need to when its rows go wrong, or when you change
the code that builds it. This page shows how to do either without showing your
users a wrong answer and without applying any event twice.

A few words first, because this page uses them throughout:

- A **projection** is the code that builds one read model. The function that
  turns each event into row changes is its **fold**.
- Each projection has a **`ProjectionId`**, a name that identifies it.
- A **checkpoint** is the position of the last event a projection has applied.
  It is stored next to the read model's rows, so that after a restart the
  projection carries on from where it stopped instead of starting over.
- A **batch** is a set of row changes that the projection store commits
  together with the checkpoint, as one transaction.

## Two reasons to rebuild

How you rebuild depends on why.

- **The fold is right, but the rows are wrong.** A table was dropped, a bad
  deploy wrote nonsense, or you just want a clean copy. Reset the view and
  replay it under the same id.
- **The fold itself changed.** It counts something different, or it now reads
  events it used to ignore. Build the new version beside the old one under a
  second id, switch your readers to it, then retire the old one.

The first is quicker. The second keeps a correct answer on screen the whole
time, as long as you promote it in the order
[below](#build-the-new-version-beside-the-old) gives.

Both use two methods of the `ProjectionStore` trait, `reset` and `commit`, plus
the projection runner, `run_projection`, which reads events from the log and
feeds them to your projection in chunks.

### Before you start: the runner is not yet stable

`run_projection` sits behind the `unstable-projection` feature, and makes no
promise of compatibility between releases. That is because the `apply` method
your projection implements is going to change: it is due to become `async`
([ADR-0074](../.kb/decisions/0074-projection-apply-is-async.md) says why).
The `ProjectionStore` trait underneath it is stable.

So the programs on this page call only the trait, using
`MemoryProjectionStore`, and they compile without the feature.
[The rebuilding example](../examples/rebuilding-read-models/src/main.rs) runs
every step below with the runner, against SQLite. It is the program to read
alongside this page.

## Choose the rebuild

Two facts decide which rebuild you can use.

**First: a partly rebuilt view looks current.** The runner always records its
checkpoint as `Authority::Live`, meaning "this view is up to date". It has no
way to say that the rows it is writing are only half rebuilt. If a replay stops
after one chunk of two, the checkpoint still says `Live`, up to the last chunk
it finished.

The `ProjectionStore` trait can record a "rebuilding" state
([PS-24](../spec/SPECIFICATION.md#47-checkpoint-semantics)), but whether the
runner will ever use it is still undecided. Until it does, anyone reading a view
while it replays sees a partial answer that claims to be current.

So: if nobody reads the view during the replay, or a partial answer does no
harm, reset it in place. If not, build under a second id.

**Second: a changed fold always needs one of the two.** The checkpoint is
stored under the `ProjectionId` and nothing else. Nothing notices when the
events a projection asks for, or what it does with them, have changed.

That has a sharp consequence. A projection that starts reading a new event type
resumes from its old checkpoint, past every older event of that type, and never
sees them. The planned fix is an id derived from the projection's query, which
changes when the query does
([PS-25](../spec/SPECIFICATION.md#47-checkpoint-semantics)). It is provisional,
not built yet, and nothing tests it. Until it is, choosing a new id is up to
you.

## Reset the view

A reset clears a view's rows and its checkpoint together, so the next run starts
from the beginning.

`reset` takes a batch, the same kind `commit` takes, and you fill it with your
own deletes. The store does not know which rows belong to which view, so it
cannot clear them for you.

What you get in return is safety against a crash halfway through. The deletes
and the checkpoint change commit together, so a crash cannot leave an empty
table whose checkpoint says it is up to date
([PS-16](../spec/SPECIFICATION.md#46-reset)). That rule is still provisional:
it would have to change if a store turned up that cannot clear rows in the same
transaction as its checkpoint.

A reset touches only the view you name. Every other view in the same store
stays where it was, so you can rebuild one view without pausing the rest
([PS-17](../spec/SPECIFICATION.md#46-reset)).

### Reset, not a commit at the first position

After a reset, the checkpoint reads `Checkpoint::NeverRun`, so the next run
applies every event again, the first one included
([PS-20](../spec/SPECIFICATION.md#46-reset)).

There is a shortcut that looks the same and is not: committing an empty batch at
the first position. That records "applied up to position 1", so the next run
starts *after* the first event and never applies it
([PS-19](../spec/SPECIFICATION.md#46-reset)).

The program below does both, and counts how many events each replay would read.
Its `replay_reads` function copies the rule the runner uses to decide where to
resume, so it is a model of the runner rather than a call to it:

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
only delete it offers. On a real database, the batch carries a `DELETE` for the
view's own table and nothing else. The example's `retire` function does exactly
that on SQLite when it prepares a rebuild.

## Do not wind the checkpoint back

A checkpoint only moves forward. A commit at an earlier position than the
current checkpoint is refused with `CommitError::CheckpointRegression`
([PS-22](../spec/SPECIFICATION.md#47-checkpoint-semantics)).

That protects you from an old runner left over after a redeploy: it cannot drag
a view backwards and apply events a second time. The same guard means you
cannot rebuild by moving the checkpoint back over rows you kept. A reset is the
only way back:

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
`NeverRun` and replays every event its query selects, a chunk at a time,
committing the checkpoint after each chunk.

### Keep the rows independent of the chunk size

The chunk size changes how often the checkpoint moves. It must not change the
rows you end up with
([PS-14](../spec/SPECIFICATION.md#44-read-your-writes)).

Your projection's part in that is to write everything into the batch, and keep
no running totals in the projection's own fields. The runner resumes from the
checkpoint, so a total held in memory starts from zero on every run, while the
rows in the database do not.

That is why the example's `StockOnHand` writes a
change against the stored row rather than a total. Its `reset_and_rebuild`
rebuilds at a chunk size of 1 and of 500, and refuses to finish if the rows
differ.

### Run one runner per view

Run exactly one runner per projection id in a given store. Nothing enforces
this. Two runners moving forward together each apply the same events, and
because neither moves the checkpoint backwards, the regression check above does
not catch them. The runner's own documentation says so under *Exactly one
runner per projection, and the caller owns that*.

## Build the new version beside the old

When the fold changes, give the new version its own `ProjectionId` and its own
table, and run it from `NeverRun` while the old view carries on answering
readers. This is often called a blue/green rebuild.

The two views move forward on their own schedules, so a long backfill never
holds the live view back
([PS-23](../spec/SPECIFICATION.md#47-checkpoint-semantics)). One thing to watch:
a reader that joins the two views sees two different moments of the log. The
example prints that difference rather than hiding it.

### Promote the new view

When the new view has caught up, promote it. Whichever way you switch your
readers over, start the same way:

1. **Stop the old view's runner, and take it off any schedule.** The old id is
   about to be reset to `NeverRun`. A runner still scheduled under it would
   replay the whole log into whatever table then has the name it writes to.

Then take one of two routes.

**Route A: repoint your readers.** The new view keeps its own table.

2. Point every reader at the new table.
3. Retire the old view: reset its id with a batch that deletes its rows or
   drops its table. Nobody reads it any more, so nobody sees it empty.

**Route B: rename the new table into the old one's place.** Readers keep using
the table name they already know.

2. Pause the new view's runner.
3. Reset the old id with a batch that carries the swap: a `DROP TABLE` on the
   old table, then an `ALTER TABLE … RENAME TO` giving the new table the old
   name. Push each statement on its own, because the batch runs one statement
   per `push`.
4. Redeploy the new view's runner under its own id, writing to the renamed
   table.

On SQLite, route B's swap is safe. The batch runs inside the one transaction
`reset` opens, and SQLite's `DROP` and `ALTER` are transactional, so readers see
the old table until the reset commits and the new one after it. A swap that
fails leaves the old table and its checkpoint in place. No conformance test puts
table changes like these in a batch, though, so check that your own database
behaves the same before you rely on it.

### Two mistakes to avoid when promoting

**Keep the new id, whichever route you took.** The checkpoint belongs to the id,
not to the table. If you change the new view's id to match the table it now
lives in, you throw its resume point away and it replays the whole log.

**Never retire the old view with a statement that names a table after the
swap.** Once the rename commits, the old name belongs to the new view, and a
`DELETE` against it clears the rows you just promoted.

[The rebuilding example's](../examples/rebuilding-read-models/src/main.rs)
`a_third_view_added_late` takes route B and keeps the new view's id. It first
tries the swap with the new table's name mistyped, and the old view keeps its
rows and its checkpoint. Then it promotes for real. After the promotion the view
applies one new event rather than the whole log.
[Its tests](../examples/rebuilding-read-models/tests/runs.rs) check both from
the program's output: `a_failed_promotion_leaves_the_old_view_whole` and
`the_binary_operates_every_view`.

## When the replay fails

A replay can stop on an event the fold cannot read, such as one missing a field
the fold expects. The runner stops at the first failure. It throws away the
chunk it was building and returns a `ProjectionError` naming the position where
it stopped. The checkpoint stays at the last chunk it committed
([PS-28](../spec/SPECIFICATION.md#48-failure-policy)).

Fix the projection and run it again. It resumes from that checkpoint and applies
nothing twice. That holds only if your fix leaves the fold itself alone. A fix that changes which events the fold reads, or how earlier
events fold, is a changed fold: go back to
[Choose the rebuild](#choose-the-rebuild).

### A failing view stops only itself

Each call to `run_projection` drives one projection, so a failing view stops
only itself. Your other views keep moving, as long as you keep calling them
([PS-29](../spec/SPECIFICATION.md#48-failure-policy)). The example's
`one_poisoned_view` breaks one view and runs the other three past it, and
`the_poisoned_view_stops_and_the_others_advance` checks the output. No
conformance test checks this against the runner yet.

Noticing that a view has stopped is your job. The runner returns the error to
whoever called it, and reports it nowhere else.

## What you cannot see while it runs

A long replay and a hung one look the same from outside, because the runner
reports progress only once, when it returns. What you can watch is the
checkpoint. It moves once per chunk, and another handle on the same store can
read it with `ProjectionStore::checkpoint`.

To show how far along a replay is, compare that position with the newest
position in the log yourself. The runner's documentation says this under *What
can be seen while it runs*. Nothing in the specification promises it, so treat
it as current behaviour rather than a guarantee.

## How this page is checked

Both programs above are compiled and run by this repository's tests. The rules
they rely on are checked by the projection conformance suite, which every
projection store passes:

- `rebuilding_is_distinguishable_from_live` (PS-24)
- `reset_clears_rows_and_checkpoint_together` (PS-16)
- `reset_is_scoped_to_one_projection` (PS-17)
- `reset_is_not_commit_at_first` (PS-19)
- `commit_rejects_a_regressing_position` (PS-22)
- `rebuild_is_chunk_size_invariant` (PS-14)
- `distinct_projections_advance_independently` (PS-23)

[The runner's tests](../crates/happenstance/tests/projection_runner.rs) cover
the runner itself: `first_run_starts_from_the_beginning` (PS-20), and
`decode_failure_names_its_position_and_rolls_back`, whose chunk-of-two case is
the half-built replay that still reads `Live` (PS-28). The rule that would check
PS-29 against a runner, `one_poisoned_projection_does_not_stall_the_others`, is
not written yet.
