Two devices sell the last seat while offline. Sync brings the oversell to a
central store, which keeps its rule with an ordinary append, and three files
end up agreeing on what happened.

```console
cargo run -p two-devices-one-seat
```

# The story

1. **Central** schedules a show with three seats and sells two. A box-office
   **tablet** and a **phone**, each a `SqliteEventStore` on its own file, sync
   down. One seat is left, everywhere.
2. **The signal drops.** Each device sells the last seat with
   `happenstance::commit`, a conditional append against the history that
   device holds. Both succeed, because each decision is right about what its
   device knew. A sync attempted now fails as a *transient* failure, and the
   sale waits on the device.
3. **The signal returns.** Each device syncs through its own
   `happenstance_sync::runner::Edge`. Ingest never refuses (SY-1), so central
   now holds four sales for three seats. The oversell is visible, not lost.
4. **Central keeps its rule.** It runs a decision over the whole history it
   holds and appends one `SaleRefunded`, an ordinary conditional append
   carrying its reason (SY-2). Running the rule again finds nothing to do.
5. **Everyone converges.** The devices sync the refund down. A read model,
   [`Timeline`], is built on all three files and shows the same thing on each:
   four sales, one refund, three of three seats held, and an audit line saying
   why. Every handle is dropped and the files are read back: three logs with
   the same event identities, three identical read models.
6. **The phone tries again**, reopened and offline. It refuses with *sold out*
   and no round trip, because the history it holds is now the complete one.

# What it shows

- **Offline is real.** A device decides alone, with the same decision model
  central uses, and a failed sync loses nothing.
- **One set of rules.** [`sell_seat`] runs on every device, and
  [`adjudicate`] on central over the history no device had alone.
- **History is kept.** Nothing is overwritten. The oversell is in every log,
  and so is the refund that answered it, with its reason as data.
- **Three places, one answer.** The read model converges because it is written
  to give the same table in any order, not because the orders happened to
  match.

# Which sale is refunded

The rule is *the first sale to reach central keeps the seat*, by central's own
log order. The refund is an event, so every peer converges on the outcome
whatever order its own log holds the sales in. Two alternatives lost. The
lowest event id is deterministic across peers (SY-23), but store ids are
minted at random, so the winning device would change from run to run. A
device's clock is not an ordering key.

# Central adjudicates by convention, for now

`StoreSyncPeer` never adjudicates: a push ingests and writes no compensation.
No published runner yet writes a compensation beside an ingest; that hook is
#450. So central applies its rule **after** ingest, with an ordinary `commit`.
That is SY-2's compensation by an ordinary append, and SY-7, which is still
provisional, makes one peer its author. Here that peer is central, by this
application's convention. When the hook lands, this example moves to it.

# The transport

Each device talks to central through `RemoteSyncPeer` over [`Uplink`], an
`Exchange` that calls `serve_sync` over central's `StoreSyncPeer`. Every pull
and push goes through the real wire envelope, so replacing `Uplink` with an
HTTP client changes the transport and nothing else. The guide page
`docs/sync-a-device.md` walks the same shape.

# Swapping central for Postgres or Neon

Neither is compiled here.

- **Postgres** (`happenstance-postgres`, feature `sync`). What changes, all of
  it:
  - `Central` becomes `PostgresEventStore`. `CentralError`, `CentralPeer`,
    `Uplink`, `UplinkError` and `StoryError`'s `Schedule` and `Adjudicate`
    variants name central only through it, and follow.
  - Central's file. `Scenario::open` opens central with the same private
    `Site` the devices use, which opens all three roles on one SQLite file.
    Central needs its own: a `PostgresEventStore`, a
    `PostgresProjectionStore` and the application's connection for the read
    model's table, all from one pool.
  - Central's read model. `Timeline` writes through a `SqliteBatch`, so a
    Postgres central needs a `Projection` over `PostgresProjectionStore`'s
    batch, with the same SQL.
  - `StoryError`'s `Open`, `OpenModel`, `ReadModel`, `Sell`, `Read` and
    `Project` variants name SQLite's errors for every place, central
    included. Central's failures need variants of their own.

  What does not change: the domain and the rule, `StoreSyncPeer`,
  `serve_sync`, `RemoteSyncPeer`, the edges, and both devices.
- **Neon** is not a one-line change. `NeonSyncPeer` *is* the peer, since the
  device talks to Neon directly over one-shot HTTP with no server between, so
  it replaces `RemoteSyncPeer<Uplink>`. The application supplies the
  `SqlTransport`, and central's rule commits through a `NeonEventStore`.

# What is not shown

One interleaving of one story, not a proof of convergence. The proof that a
projection declared convergent is order-independent is
`happenstance-sync-testkit`'s interleaving rule. No peer set above one edge,
no adjudicator running on its own schedule, and no network.
