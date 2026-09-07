# The name, and how to use it

Established by [`SD-0001`](SD-0001-what-happenstance-means.md). This file is the
usable form — what to write when the name needs explaining, at each of the lengths
it gets asked for.

## The premise, once

The everyday sense is **chance**, and that is the primary dictionary sense, not a
fringe one. The sense meant here is the narrower one in *as it happened*: **what
turned out to be the case, as against what was arranged in advance.**

That is the axis DCB moves along. A classical aggregate fixes the consistency
boundary when the schema is written. A DCB boundary is chosen by the handler —
deliberate, but not pre-declared — and is then whatever its query happened to
match.

**Deliberate but not pre-declared.** Hold that phrase precisely. The handler writes
its query on purpose; what it does not do is inherit a boundary someone drew
earlier. Copy that implies the boundary is unplanned is wrong and hands the
argument to the colloquial reading.

## At four lengths

**Six words.** *The boundary is not declared in advance.*

**One sentence, for a crate doc opener.**
> A DCB boundary is not fixed when the schema is written — it is whatever the
> handler's query happened to match.

**Two sentences, for an evaluator who asked why the name.**
> The everyday sense of happenstance is chance, which is the wrong idea here. The
> one meant is *as it happened*: a classical aggregate fixes the consistency
> boundary before anyone knows which decisions will be made, and DCB lets it be
> whatever the handler's query turned out to match.

**A paragraph, for a written piece or a talk.** Use the README's *The name*
section, which is the canonical published form, or SD-0001's opening if more
length is wanted.

## Rules for using it

- **Never assert the name is clever.** Introduce the sense and let the reader make
  the connection. The project's whole register is showing the evidence and
  declining to editorialise; a name that announces its own aptness breaks it.
- **Name the colloquial reading and displace it; do not talk past it.** Chance is
  the default sense and it is exactly wrong for a durable event store. Copy that
  simply asserts the intended meaning reads as a stretch, because against the
  dictionary it is one. Copy that says "the everyday sense is chance, which is the
  wrong idea here" and then gives the right one is doing the actual work.
- **Lower-case in prose**, matching the crate name, including at the start of a
  sentence where the sentence can be recast to avoid it.
- **Do not extend the metaphor.** No chance, luck, dice, serendipity, fortune or
  coincidence imagery anywhere — in copy, in examples, in error messages, in a
  logo. Every one of those reinforces the reading this decision exists to
  displace. This is the single easiest way to undo SD-0001 and it will be
  tempting, because the imagery is right there.
- **The name is not the lead claim.** It supports whichever claim positioning
  settles on; it does not substitute for one.

## The crate names

ADR-0006 set the rule, and it is a two-audience statement:

| Crate | Who pins it |
|---|---|
| `happenstance-core` | Adapter authors. Ports, types, errors, the in-memory reference store. Low churn |
| `happenstance` | Application authors. The typed layer — `Codec`, `DomainEvent`, `DecisionModel`, the command loop |
| `happenstance-testkit` | Adapter authors. Versioned independently, because adding a rule can turn a passing adapter red |
| `happenstance-<backend>` | Adapters. `-sqlite`, `-postgres`, `-neon`, `-cloudflare`, `-ladybug`, `-sync` |

Precedent, and the argument that decided it: `serde_core`/`serde`,
`futures-core`/`futures`, `tracing-core`/`tracing` all converged independently on
the bare name going to what applications import.

**Undecided:** whether a commercial tier or any non-adapter crate breaks the
`happenstance-<thing>` pattern. Do not set precedent by accident — if one is
needed, decide it as an `SD-` first.

## The acknowledged costs

From SD-0001, repeated here because copy decisions keep meeting them:

- The colloquial sense is wrong and is the default reading.
- The meaning is retroactive; ADR-0005 is public and says availability. Be first
  to say so rather than be caught.
- Twelve letters, three syllables, no short form, no clean translation. Permanent,
  and not a reason to rename.
