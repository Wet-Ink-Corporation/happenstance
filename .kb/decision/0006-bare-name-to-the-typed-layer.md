---
id: adr-0006-bare-name-to-the-typed-layer
title: "ADR-0006: The bare name goes to the typed layer; the contract becomes `happenstance-core`"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  `happenstance` is the typed layer an application reaches for; the contract crate is
  `happenstance-core` and `happenstance-runtime` ceases to exist. PARTLY superseded by
  ADR-0007, which corrects where the projection runner lives.
depends_on: []
related:
  - adr-0005-rename-to-happenstance
  - adr-0007-projection-runner-decodes
source_paths:
  - Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-09
adr_id: ADR-0006
supersedes: []
superseded_by: null
---

# ADR-0006: The bare name goes to the typed layer; the contract becomes `happenstance-core`

- **Status:** partly superseded by [ADR-0007](0007-projection-runner-decodes.md)
- **Date:** 2026-08-05
- **Partly supersedes:** [ADR-0005](0005-rename-to-happenstance.md)

> **Partly superseded.** Like the ADR it corrects, this one bundles two decisions
> under one "and". The first — the bare name to the typed layer, the contract to
> `happenstance-core` — **stands**, on exactly the reasoning below. The second —
> moving the projection runner into the contract crate, on the grounds that it
> "never decodes a payload" — is corrected by
> [ADR-0007](0007-projection-runner-decodes.md). That claim was asserted before
> any runner existed, and a projection runner that never decodes withholds the
> half applications actually need.
>
> The body is kept verbatim, per the rule this ADR applied to ADR-0005.

## Context

[ADR-0005](0005-rename-to-happenstance.md) made two decisions under one "and":

1. rename the project from `eventum` to `happenstance`;
2. make `happenstance` the **contract** crate rather than a facade over a
   `-core`.

The first is unforced and well evidenced, and it stands. The second rode in on
its momentum, and the argument given for it does not survive inspection:

> *"a contract crate is the thing users import constantly, while a facade is a
> convenience that may never be built."*

Two problems. It is an empirical claim about the behaviour of users who do not
exist, asserted without evidence. And the facade it says "may never be built" is
**phase 3 of [`docs/RUNBOOK.md`](../RUNBOOK.md)** — scheduled, with dependencies,
an exit criterion, and the worked example rewritten on top of it. The two
documents, written the same afternoon, contradict each other.

ADR-0005 also recorded the cost of the choice at the time, under **Bad**:

> *"A batteries-included facade now has no natural name left. If one is ever
> wanted, it must be `happenstance-full` or similar, or the contract crate has to
> be renamed at that point."*

That bill has arrived. Ledger row 6 asks whether `happenstance-runtime` is the
right name for the typed layer, and no good answer exists — `-runtime`,
`-domain`, `-typed`, `-model`, `-app` are all either wrong or vague. A naming
problem that will not resolve is usually a decomposition problem.

### Who imports what

| Audience | Imports | Population |
|---|---|---|
| Applications | typed layer + one adapter | the overwhelming majority |
| Adapter authors | contract + testkit | small, and sophisticated |

Even this repository's own worked example migrates: `course-subscriptions` uses
the contract with hand-rolled bytes today, and phase 3's exit criterion is
rewriting it against the typed layer.

### Ecosystem precedent

Three major crates converged independently on the opposite allocation to
ADR-0005's, and for this exact shape:

| Trait crate | Batteries crate |
|---|---|
| `serde_core` | `serde` |
| `futures-core` | `futures` |
| `tracing-core` | `tracing` |

In every case the **bare name goes to what applications import**, and `-core` to
the low-churn trait crate that implementers pin. `futures-core` is already a
dependency of this workspace.

ADR-0005 rejected `-core` as *"a suffix which existed for no reason other than
the collision"*. That was true of `eventum-core`, and is false here: there is a
reason, and it is the one `serde`, `futures` and `tracing` each arrived at
separately.

## Decision

Invert the allocation:

| Crate | Contents |
|---|---|
| **`happenstance-core`** | ports, types, errors, in-memory reference store, projection runner. No `serde`. `no_std` + `alloc`. Low churn — this is what adapter authors pin. |
| **`happenstance`** | `Codec`, `DomainEvent`, `DecisionModel`, the command loop. Re-exports the contract; feature-gates the adapters. What an application `cargo add`s. |

`happenstance-testkit`, `happenstance-sqlite`, `happenstance-ladybug` and
`happenstance-sync` keep their names and depend on `happenstance-core`.
`happenstance-runtime` ceases to exist; its contents become `happenstance`.

The dependency rule is unchanged in substance and clearer in form: **everything
depends on `happenstance-core`; `happenstance-core` depends on nothing in this
workspace.**

### The `serde` boundary moves with the contract, not with the name

ADR-0003's constraint — never put `serde` in the contract crate's default
features — is unaffected. It attaches to the crate that defines the ports, which
is now `happenstance-core`. The principle was always about the contract, not
about the string on the front of it.

### The projection runner moves into the contract crate

The discriminator for what belongs in the typed layer is **encoding**, not
orchestration. The projection runner pumps an `EventStore` into a
`ProjectionStore` and never decodes a payload — projections write through the
adapter's own `Batch`, which is adapter-specific by construction. It therefore
belongs beside `read_decision_model`, which is already an async orchestration
helper living in the contract crate.

That leaves the typed layer with a charter statable in one sentence: *this is
where Rust types meet opaque bytes.*

## Consequences

**Good.** `cargo add happenstance` gives an application the thing it actually
wants. The unresolvable naming question in ledger row 6 disappears rather than
being answered badly. The layout matches three independent ecosystem
precedents. ADR-0005's recorded cost — no name left for a facade — is paid off
rather than deferred.

**Good.** `happenstance-core` is exactly the kind of crate that benefits from a
`-core` suffix: tiny dependency graph, `no_std`, and a semver surface that
should almost never move. The suffix now carries information.

**Bad.** A second rename, four hours after the first. It is free — nothing is
published — but it is churn, and the repository's history will show two naming
ADRs in one day. Recording *why* the first was wrong is the mitigation.

**Bad.** Someone arriving to learn how happenstance maps to the DCB
specification now starts at `happenstance-core` rather than at the obvious name.
The contract crate's documentation must say so in its first paragraph, and
`happenstance` must point at it.

**Neutral.** The crates.io reservation follow-up from ADR-0005 is still
outstanding and now covers **two** names: `happenstance` and
`happenstance-core`. Both are free today; neither is reserved.

## Alternatives rejected

- **Keep `happenstance` as the contract and rename the typed layer to
  `happenstance-domain`.** The cheapest option, and it fixes the immediate
  complaint that `-runtime` collides with Rust's established meaning of the word
  — a collision this repository's own vocabulary already relies on ("Workers
  runtime", "non-tokio runtime"). But it leaves the bare name on the crate that
  fewer people import, and leaves the facade problem unsolved.

- **Defer the allocation until phase 3, when the typed layer exists.** Rejected
  on inspection: phase 3 cannot produce the information the deferral claims to
  wait for. There would still be no users to observe, so the same judgment call
  would be made with the same evidence — only later, after the README, the
  doctests and the rewritten worked example had all been written against the
  names being deferred. The cost of deciding rises monotonically and the
  information does not arrive.

- **`happenstance-full` as the facade name**, as ADR-0005 contemplated. Concedes
  the good name to the smaller audience and reads like an afterthought.

## Note on ADRs 0001, 0003 and 0004

Those three now carry a **provisional** marker. They were authored on the same
afternoon as the initial scaffold, before the code they constrain existed, and
ADR-0002 was superseded thirty-two minutes after it was written. A corpus in
that state records intentions; it has not yet settled anything. The marker says
what would have to happen for each to become precedent, so that new work is not
made to argue against decisions the code has not voted on.

This ADR is subject to the same standard. It is not marked provisional only
because a naming decision is settled by being made, not by being tested.

## On the historical record

*Added when this ADR was executed, in phase 0 of
[`docs/RUNBOOK.md`](../RUNBOOK.md), commit `7d6c1b0`.*

ADR-0001, ADR-0003, ADR-0004, ADR-0007 and CLAUDE.md's binding constraints had
their crate names **rewritten** to `happenstance-core`, rather than left as
period spelling under a note. The same pass rewrote ADR-0003's two references to
`happenstance-runtime`, the crate this ADR dissolved, to `happenstance`. ADR-0007
was missed by that pass and corrected after it: its link to `ProjectionStore`
still pointed into `crates/happenstance/`, which holds only `lib.rs`, and now
points at `crates/happenstance-core/src/projection.rs`, where the trait is.

That is not falsifying the record, and the distinction that makes it legitimate
is a narrow one. Every rewritten sentence was always a statement about the crate
that **defines the ports**; the rename moved the string and left the referent
where it was. Rewriting them preserves the meaning. Leaving them would have
preserved the spelling and inverted the meaning — and ADR-0003 is the case that
proves it rather than merely illustrating it. Untouched, it forbids `serde` to
whatever crate is called `happenstance`, which is now the typed layer whose
entire job is encoding. It would have forbidden the thing this ADR exists to
allow, while permitting `serde` into the contract crate, where ADR-0003 spent
its whole argument keeping it out.

ADR-0002 and ADR-0005 are handled the opposite way — superseded banner, body
factually intact — and the two treatments do not conflict, because the rule is
**rewrite the referent, never the reasoning**. A superseded ADR's body records a
decision that was taken and then reversed, so rewriting it erases the reversal:
ADR-0002 with `eventum` rewritten to `happenstance` would assert that the name
is taken, which is precisely the claim ADR-0005 overturned. A corrected name
inside a decision that still stands is not a reversal at all. Nothing was
overturned, so an intact body would preserve nothing except a misdirection.

### Stale in the body above, recorded here rather than edited

The body cites **phase 3 of `docs/RUNBOOK.md`** three times: for the typed
layer's schedule, for its exit criterion of rewriting `course-subscriptions`
against it, and for the rejected alternative of deferring the allocation until
it. That runbook has since been replaced (`8f2ce15`) by a fifteen-phase plan in
which the typed layer is **phase 7** and phase 3 is the conformance suite.

The argument is unaffected — what it needed was that the facade was scheduled,
with dependencies and an exit criterion, and it still is. The number stays wrong
in the body because this is not the case the rule above covers: a rename leaves
the referent intact, but the runbook was rewritten rather than renamed, so there
is no phase 3 that *became* phase 7. There is a different plan. Repointing the
citation would put a reference to a document that did not exist into an argument
made before it did.
