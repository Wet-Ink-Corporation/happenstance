Consistency boundaries with nothing to hang them on: a handle registry.

Three rules, and not one of them belongs to an entity:

1. a handle is held by at most one owner at a time;
2. an owner may hold no more handles than their plan allows;
3. a command sent twice does the work once.

Rule 1 is about a *name*. There is no Handle to be an aggregate — a handle
that nobody has claimed is not an object that exists in some unclaimed state,
it is a string nobody has written yet, drawn from a set nobody can enumerate.
Classical event sourcing answers this with a unique index, which lives outside
the log and therefore commits separately from it, or with a Handles aggregate
holding every name ever claimed, which is one stream that every signup in the
system contends on.

Rule 2 spans a set that is not known until it is read: which handles this
owner holds. Rule 3 spans something that is not domain state at all — whether
a particular *request* has already been applied.

DCB draws all three with one mechanism. Each is a query; the three queries are
composed into one boundary by writing them as a tuple; the read notes where it
read to, and the append is conditioned on nothing matching any of those three
queries having appeared since. One append condition, three boundaries, no
aggregate and no index.

The third rule is what makes a retry safe rather than merely likely to be
harmless. Because the request's own boundary is part of the same condition,
two concurrent deliveries of one command cannot both win: the second reads the
first's event, or is rejected by the condition and re-reads and finds it.

Run with `cargo run -p handles-and-quotas`.

# What this does not claim

The boundary is dynamic, and a released handle becomes claimable again — which
a unique index over claimed names cannot express without a delete. That is the
argument being made. It is *not* an argument that a unique index is slower, or
that this is cheaper: nothing here is measured, and `experiments/` is where
measurement lives in this repository.

Nor is it a durability or a concurrency demonstration. One process, one
writer, and the retry bound is never spent. `transfers-on-sqlite` owns the
reopen, and `tickets-over-http` races real clients over a real socket.
