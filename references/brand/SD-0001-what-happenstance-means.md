# SD-0001 — "Happenstance" names the boundary drawn by what occurred

**Status:** accepted
**Date:** 2026-08-18
**Decides:** what the project's name means, for the purpose of every sentence
written about it
**Supersedes:** nothing
**Related:** [ADR-0005](../adr/0005-rename-to-happenstance.md) (the rename),
[ADR-0006](../adr/0006-bare-name-to-the-typed-layer.md) (the bare name's
allocation), [`naming.md`](naming.md) (the usable forms)

---

## The decision

**"Happenstance" names the thing DCB does: it draws the consistency boundary
around what actually occurred, rather than around a structure chosen before
anyone knew what would occur.**

The word is "happen" + "circumstance," a mid-nineteenth-century American blend.
**Its dominant dictionary sense is chance** — Merriam-Webster gives "a
circumstance especially that is due to chance" — and this record does not pretend
otherwise. The sense it claims is the narrower one carried by the phrase *as it
happened*: what turned out to be the case, as against what was arranged in
advance.

That is the axis DCB moves along. A classical aggregate fixes the consistency
boundary when the schema is written, before anyone knows which decisions will be
made against it. A DCB handler reads exactly the events its decision depends on
and appends conditioned on nothing matching that query having appeared since —
so the boundary is not declared ahead of time and then defended, it is whatever
that query happened to match.

**The boundary is deliberate but not pre-declared, and the difference matters.**
An earlier draft of this record glossed the name as "arising from what took place
rather than from design," which is wrong in a way worth recording: the handler
designs its query on purpose. What it does not do is inherit a boundary someone
else drew earlier. *Unplanned* is the wrong word; *unarranged in advance* is the
right one.

Classical event sourcing asks the opposite. It makes you draw consistency
boundaries before you know what decisions you will make, commit to them as
aggregates, and then pay a read model plus a saga every time a decision refuses
to fit inside one. That is a boundary by prior design. happenstance's is a
boundary by happenstance.

## What this record does not claim

**The name was not chosen for this reason.** ADR-0005 renamed the project from
`eventum` on 2026-08-05 because the bare `eventum` crate name was held by an
unrelated crate dormant since 2020, and the entire recorded justification for
`happenstance` is availability: "the crates.io endpoint for it returns 404, and a
prefix search returns no `happenstance*` crates"
(`references/adr/0005-rename-to-happenstance.md:28-30`).

So this record assigns a meaning after the fact. That is legitimate and worth
being explicit about rather than quietly retrofitting — a name acquires its
meaning from use in any case, and the choice here is whether that meaning is
chosen or left to accumulate. ADR-0005's own closing observation is that "a naming
decision is settled by being made, not by being tested." True, and it leaves the
meaning unsettled. This settles the meaning, not the name.

## Why it lost, for each option not taken

**Leave it meaningless.** The status quo: the name is a string that was free. This
is not costless. Every piece of copy that wants to say what the project is about
starts from zero, because the name contributes nothing and sometimes actively
works against the sentence it sits in. An evaluator who asks "why is it called
that?" — and they do ask, it is the cheapest possible question — receives a shrug,
which in a project whose entire posture is *every claim names what would falsify
it* reads as the one place nobody thought. Rejected because the cost is paid
continuously and the fix is paid once.

**Name it after the mechanism directly** — some construction on *boundary*,
*condition*, *query*, *append*. This is what the market already did: `cqrs-es`,
`eventually`, `disintegrate`, `eventsourcing`. It is legible on first contact and
it is also the reason none of those names can carry an argument. A mechanism name
describes the how and leaves the *why* homeless, and happenstance's differentiator
is a why — the boundary is drawn later, by the decision, from what happened.
Rejected as unavailable in any case (the rename was forced by a collision) and
unhelpful even if it had been.

**Claim the edge capability instead** — lean the identity on `!Send`, `wasm32`,
Durable Objects, the offline-tolerant scenarios. This is the strongest uncontested
differentiator in the project: no surveyed Rust crate can reach a Durable Object
at all, and `cqrs-es` has no path to a `!Send` target
([`references/evaluation/research-rust-ecosystem.md:75`](../evaluation/research-rust-ecosystem.md)).
But it is a *capability* claim, and the name is not where capability claims belong
— they belong where they can be checked, which for this project means the
conformance suite and HS-P0013. Rejected as a naming argument, retained as a
positioning one.

## Bad.

Three costs, and the first is real enough to have nearly sunk this.

**"Happenstance" colloquially means accident, and nobody wants an accidental
database.** The dominant everyday sense is chance, luck, coincidence — the opposite
of the reliability an event store must project, and it is the *primary* dictionary
sense rather than a fringe reading, so a cold reader will supply it by default.
This decision does not make that go away. It only ensures that wherever the name
is introduced at length, the intended sense arrives with it — and because the
colloquial reading is the default, the introduction has to **name and displace it**
rather than quietly assert the other one. The README now does this in its first
sentence on the subject. Copy that skips the displacement and simply asserts the
intended sense will read as a stretch, because it is one.

**The meaning is retroactive and a hostile reader can say so.** ADR-0005 is public
and says availability. Anyone can read it. The mitigation is not to hide it but to
be first to say it, which is what the section above does, and which is the same
move ADR-0006 made when it dismantled its own predecessor's reasoning four hours
after that reasoning was written.

**It does not translate and it does not abbreviate.** Twelve letters, three
syllables, no natural short form, and a word many non-native English speakers will
not know. `hs` is taken by everything; `happenstance` in a dependency list is long.
This is a permanent, unfixable cost of a name already chosen and shipped to
crates.io under four crate names, and it is recorded here so that nobody proposes
solving it with a rename, which would cost far more than it recovers.

## What follows from this

- The README's *Former name* section explains the rename but not the name. It
  should carry one sentence of meaning. **Done 2026-08-18.**
- Copy may now use the boundary-by-happening reading as a shared premise rather
  than re-deriving it. [`naming.md`](naming.md) holds the usable forms.
- The lead-claim question is unblocked on one axis: whatever the lead claim turns
  out to be, the name is no longer neutral about it.

## Evidence

| Claim | Source |
|---|---|
| The name was justified only by availability | [`references/adr/0005-rename-to-happenstance.md:28-30`](../adr/0005-rename-to-happenstance.md) |
| A naming decision is settled by being made | [`references/adr/0005-rename-to-happenstance.md`](../adr/0005-rename-to-happenstance.md) |
| The bare name went to the typed layer, not the contract | [`.kb/decisions/0006-bare-name-to-the-typed-layer.md`](../../.kb/decisions/0006-bare-name-to-the-typed-layer.md) |
| DCB draws the boundary per decision, from what the handler read | `README.md:20-28`; [`spec/SPECIFICATION.md`](../../spec/SPECIFICATION.md) |
| No surveyed Rust crate reaches a `!Send` target | [`references/evaluation/research-rust-ecosystem.md:75`](../evaluation/research-rust-ecosystem.md) |
