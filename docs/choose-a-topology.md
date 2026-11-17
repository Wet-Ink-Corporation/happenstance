# Choose a topology

> **Answers:** `how-to` — Where should each log, view and decision live in my app?

An app built on happenstance can keep its data in more than one place: a SQLite
file on each device, a Durable Object at the edge, and a central Postgres or
Neon database. This page shows how to arrange those places for the app you are
building. It names the four choices you make, the rule each arrangement keeps,
and the shapes that work today.

[Choose a store](choose-a-store.md) answers a narrower question, which one
database a single log should live in. Read it first if you only have one.

## The four choices

Every arrangement is four independent choices.

| Choice | Your options | What decides it |
| --- | --- | --- |
| **Where each log lives** | `SqliteEventStore` on a device or host, `CloudflareEventStore` in a Durable Object, `PostgresEventStore` or `NeonEventStore` centrally | Where the writes happen, and whether that place is ever offline |
| **Which logs sync with which** | One `happenstance-sync` edge per pair of stores, run by the runner | Who needs to see whose events |
| **Where each view lives** | A projection reads one store and writes its read model into any projection store | How big the view is, and who queries it |
| **Where each decision is made** | Wherever a decision model runs its query and appends under a condition | Which store holds every event the decision reads |

The first three are free. The fourth is not, and the next section says why.

## The rules every arrangement keeps

**A decision belongs where its history is complete.** A conditional append
checks the events the store holds. If the store is missing some of the events
the decision's query names, the check passes without them, which is what
[ES-40](../spec/SPECIFICATION.md) records. So run a decision on a store that
holds everything its query reads: a central store, a store that holds a whole
workspace, or a device that has not pruned that history.

**Positions belong to one store.** Each store numbers its events in its own
order, and a replicated event is numbered by when it arrived
([SY-19](../spec/SPECIFICATION.md)). A checkpoint or a resume point is only
meaningful against the store that assigned it. Event identity is what crosses
between stores. The typed runner holds you to this: a view's checkpoint is
pinned to the store it was built from, and a run against another store stops
with `ProjectionError::SourceChanged` and applies nothing
([PS-40](../spec/SPECIFICATION.md)). Build each device's views from that
device's own store.

**Nothing is refused, and nothing is overwritten.** A store taking in another
store's events never refuses one ([SY-1](../spec/SPECIFICATION.md)). When two
places acted on the same thing, the fix is an ordinary new event
([SY-2](../spec/SPECIFICATION.md)), so the history keeps both what happened
and how it was put right. A device whose push is refused keeps its own log
([SY-34](../spec/SPECIFICATION.md)).

**Subsets come from how you cut stores.** Sync carries a whole log
([SY-27](../spec/SPECIFICATION.md)). To give a client only part of the data,
give that part its own store.

## The shapes, and when to use each

### Work offline, sync when back

**Use it when** each device must keep working with no network, and a central
database is the system of record.

Give each device a `SqliteEventStore` on its own file. Run one sync edge from
each device to the central store: `NeonSyncPeer` for Neon over HTTP, or
`serve_sync` over a `StoreSyncPeer` on your server, reached with a
`RemoteSyncPeer` over your own HTTP client. Every local write syncs up, and
everyone else's syncs down.

**Status:** works today, one edge per device, with your code driving the sync
loop.
[Sync a device to a central store](sync-a-device.md) walks through it, and
the [`two-devices-one-seat`](../examples/two-devices-one-seat) example runs it
end to end, including two devices selling the last seat offline.

### Many clients, one source of truth

**Use it when** many devices write, and one place must settle conflicts.

This is the shape above with many devices. Make the central store the place
that decides: it holds every device's events, so a decision over the whole
history is sound there. When two devices acted on the same thing offline, the
central store sees both events and appends the correction, which then syncs
back to everyone. Only one place should write corrections for a given fact
([SY-7](../spec/SPECIFICATION.md)).

**Status:** works today. From `0.7` the deciding store can adjudicate as it
syncs, through a merge rule mounted on its edge or its peer
(`runner::Adjudicate`); without one, it decides by convention after a sync.

### Each client holds only its workspace

**Use it when** clients should hold only the workspaces, cases or tenants they
belong to.

Give each workspace its own store centrally: a Neon schema each, or a Durable
Object each. A client runs one edge for each workspace it holds. Decisions about
one workspace are sound on the client, because its copy of that workspace is
complete. A decision that spans workspaces belongs on the server, because no
single client holds them all.

**Status:** works today.

### Huge views stay on the server

**Use it when** a view is too large for a device, such as a million customer
records.

Run the projection on the server, reading the central store and writing its
read model into the same database. Clients query it over HTTP. The
[`tickets-over-http`](../examples/tickets-over-http) example does this end to
end, including telling a client that has just written whether the view has
caught up yet.

**Status:** works today.

### A small local view of a big central store

**Use it when** a device needs fast local reads of a slice of the central data,
such as the cases one person owns.

Run a projection on the device that reads the central store and writes its read
model into local SQLite. Only the events the projection's query names cross the
network, and the view can be thrown away and rebuilt from the central store at
any time. Writes still go through one of the other shapes.

**Status:** works today. A view whose scope is "any of these tags", such as
"every case I own" in one projection, is a candidate for `1.0`; today it takes
one projection per case.

### Thin clients for the big decisions

**Use it when** a command needs the whole history, or spans slices no client
holds.

Send the command to the server and let it decide and append there.
[`tickets-over-http`](../examples/tickets-over-http) and
[`handles-and-quotas`](../examples/handles-and-quotas) both work this way. It
mixes freely with the offline shape: a client can keep its own log for local
work and forward the commands that need more.

**Status:** works today.

### A lean device and a complete cloud

**Use it when** devices should keep recent history only, while the central
database keeps everything.

The device keeps the events the central store has not confirmed, plus whatever
recent history its decisions need. The central store keeps the whole log.
Pruning happens outside the store's interface ([ES-37](../spec/SPECIFICATION.md)),
and a pruned store must still behave correctly for what it holds
([ES-38](../spec/SPECIFICATION.md)). A decision on the device stays sound only
over history it still holds in full.

**Status:** in part. From `0.7` the conformance suite holds every store to
ES-38 after rows are removed outside its interface, a condition over removed
history passes rather than refusing
([ES-40](../spec/SPECIFICATION.md)), and an edge whose far side has forgotten
history it has not reached stops with a retention gap rather than converging
on a subset ([SY-32](../spec/SPECIFICATION.md)). Pruning a device that syncs is
not yet safe: a pruned event can be pulled back in, and pruning one of the
device's own events can stop its edge
([#483](https://github.com/Wet-Ink-Corporation/happenstance/issues/483),
[#484](https://github.com/Wet-Ink-Corporation/happenstance/issues/484)).

### Edge hubs close to your users

**Use it when** devices should sync with something nearby, which in turn syncs
with the central database.

Give each tenant a Durable Object that serves sync to its devices and runs its
own edge to Neon. The Durable Object passes the sync conformance suite today.
A runner that links more than one edge is the missing piece.

**Status:** on the roadmap to `1.0`.

## Putting it together

A typical app mixes shapes. A field-service app might give each technician a
device log that syncs to Neon, keep a local view of that technician's jobs, keep
the full customer view on the server, and send scheduling commands to the server
because they span every technician. Each piece follows the rules above, so the
whole stays consistent.

Next: [run the event store inside a Durable Object](run-on-a-durable-object.md)
for the edge, or the [`happenstance-sync` API](https://docs.rs/happenstance-sync)
for the sync types named on this page.
