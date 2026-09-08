---
id: kb-decision-0039
title: ES-42 freezes without a Unpin bound, at the deadline the marker named
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0039
reversibility: low
phase: 12
supersedes: null
superseded_by: null
summary: >-
  EventStore::read's return type takes no + Unpin bound, and that is now [FROZEN]
  rather than [PROVISIONAL], decided at the 0.2.0 release pass the marker itself
  named as its own deadline. No consumer needs dyn erasure the hand-written
  Pin<Box<dyn Stream + 'a>> wrapper (ADR-0011's E11) cannot already provide: boxing
  suffices because Pin<Box<T>> is Unpin whatever T is, so the erasure round-trips
  against MemoryEventStore with no unsafe code and no port change. Freezing without
  the bound permanently forecloses a generator-backed chunked-cursor implementation
  of read, since a bound cannot be added to an opaque return type after publish
  without a major version. CF-25's standing qualification on freezing a port against
  a monoculture does not gate this freeze, because ES-42's rule is discharged by the
  compiler rather than by any adapter's behaviour, and no adapter axis could
  demonstrate the property either way.
depends_on:
  - kb-decision-0011
related:
  - kb-decision-0037
  - kb-open-question-cf-25-cf-26-portfolio-check-001
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/es-42-marker-earned-off-at-0-2-0.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# ES-42 freezes without a Unpin bound, at the deadline the marker named

## Decision

`EventStore::read`'s return type carries no `+ Unpin` bound, and ES-42's marker
in `spec/SPECIFICATION.md` moves from `[PROVISIONAL]` to `[FROZEN]`. This is not
an ordinary maturity promotion decided because evidence accumulated; it is a
marker that named its own deadline — *"must be re-evaluated before phase 12:
adding a bound to an opaque return type after publish is breaking, so this
clause expires rather than drifts"* — and the `0.2.0` release pass is that
re-evaluation, taken because the deadline had arrived rather than because a
falsifier fired.

## The question, asked in the clause's own terms

Does any consumer need `dyn EventStore` where the port has acquired a method
the hand-written erasure wrapper cannot box — a generic method, which is not
dyn-compatible, or a return whose lifetime the wrapper cannot name?

**The answer at `0.2.0` is no.** No such consumer exists in this workspace and
none is known to be coming. The marker is explicit that inconvenience is not
the falsifier: the erasure is roughly fifteen lines of downstream code
(`kb-decision-0011`'s E11), it compiles today, and it round-trips against
`MemoryEventStore` with no unsafe code and no change to the port.

## Why freezing is available at all

`Pin<Box<dyn Stream + 'a>>` is itself a `Stream` through the standard blanket
impl over `Pin<P>`, and `Pin<Box<T>>` is `Unpin` whatever `T` is. That is why
boxing suffices, why no pin projection is needed, and why no unsafe code
appears in a workspace that forbids it outright. A consumer who needs a trait object can get one today, over the
signature as written, without the library doing anything on their behalf.

What the rejected `+ Unpin` bound would have cost is the reason this is a real
fork and not a formality: it permanently forbids a generator-backed stream — a
`stream!` macro implementation of `read` fails to compile with `cannot be
unpinned` against such a bound — which is the natural implementation of a
chunked-cursor read. That shape has no implementation anywhere in this
workspace today and therefore no vote in this decision, which is exactly the
situation `kb-decision-0034`'s sibling qualification (CF-25, tracked in
`kb-reference-port-traits-compiled-findings-001`) exists to be careful about.

## Why CF-25 does not gate this freeze

Stated rather than assumed, because CF-25's qualification applies to every
`[FROZEN]` port clause and skipping past it silently is the failure mode it was
written against. CF-25 forbids declaring a *port* frozen while an axis of the
instrument portfolio has no passing implementation at its far end.

ES-42's rule is compile-level. There is no adapter axis that could falsify a
property of a return *type*, because an adapter cannot demonstrate that
`read`'s return does or does not need `+ Unpin` — that property is discharged
by the compiler, not by a store's behaviour under load. The evidence is a
compile, and the E11 erasure wrapper compiling without the bound is it. That is
a narrower argument than "CF-25 does not apply to ES clauses": it applies to
this clause because this clause's rule is not about behaviour at all.

## What would reopen it, and what that now costs

Unchanged, and deliberately kept in the clause body: a consumer that genuinely
needs the erasure and cannot write it without the bound. A frozen clause still
has to say what a mistake proving it wrong would look like.

What has changed is the price. While the clause was provisional, reopening it
was an edit to a still-open question. It is now `[FROZEN]` on a published
crate, so reopening costs a new ADR under this corpus's immutability
discipline, and adding the bound afterward would be a breaking change to a
published return type, requiring a major-version move under `0.x` semver
rules. That asymmetry is the entire reason the marker demanded a decision
before publication rather than after — the same shape ADR-0037 records for the
MSRV promise, where a number that cost nothing to move before publication
started costing a version bump the moment someone else could be pinned to it.

## Alternatives rejected

**Leaving the marker `[PROVISIONAL]` past phase 12.** Rejected because the
marker's own text names phase 12 as the point past which the question can no
longer be asked cheaply, and letting it drift past that point without a
decision is exactly the failure this corpus's maturity-marker discipline
exists to prevent — a marker that says "decide me by X" is a claim, not
housekeeping.

**Adding `+ Unpin` pre-emptively, to keep every option open.** Rejected
because it is not free: it forecloses the generator-backed implementation
permanently and buys nothing against a consumer need that does not exist. A
bound added "just in case" is exactly the kind of unearned constraint this
workspace's port-freezing discipline is built to refuse.

## What this does not decide

Whether a future chunked-cursor `read` implementation is ever built, or by
which adapter, is untouched — this decision only settles that such an
implementation, if it is ever wanted, arrives by major version rather than by
minor. `kb-decision-0037`'s account of what a version bump costs a consumer at
this stage applies unchanged.
