---
item: "{{item}}"
stage: design
created: "{{created}}"
updated: "{{updated}}"
---

# API surface design — {{title}}

The resolved **public API surface** for this project, signed off by a human before any story spec is
written. Everything below is binding on the implementer.

**This is the bundled design stage, repurposed.** Redkiln ships it because the pipeline had no stage
that decided what a screen looks like: discovery is barred from prescribing design, the UX brief is
held to contract grain, and a spec's invariants cover state preservation rather than composition. So
layout was whatever fell out of the implementer's first guess, and no downstream gate could perceive
the result.

A library has exactly that hole, in a different medium. Every other check here is satisfied by an API
that is correct and unusable: the conformance suite tests behaviour, clippy tests style, `missing_docs`
tests that a sentence exists, and none of them can see that a caller must name `Self::Batch<'_>` to
implement a trait, or that the one type a user meets first is buried behind a feature they will not
think to enable. The surface a `cargo add happenstance` user meets is this repository's screen.

**No capture runs.** `design.capture` is undeclared in `.redkiln/config.yaml` — there is no app to
screenshot — so the perceptual review is skipped rather than silently passed. What stands in its place
is `## The doctest`: a runnable example is the one artifact that shows the surface as a user meets it
*and* cannot rot, because the gate compiles it.

**If this project changes no public API**, say so explicitly in `## Items` with the reason, and leave
the remaining sections as a single "N/A — no public surface" line. Do not delete them, and do not leave
this file as the unauthored template: the engine blocks the stage exit on a stub, and an unexamined
"this project has no surface" claim is how a `pub` item nobody designed reaches a published crate.

## Items

Every public item this project adds, changes or removes. `path` is the full path a caller writes.

```yaml
- path: "" # e.g. happenstance_core::ProjectionStore::commit
  kind: "" # trait | type | fn | method | assoc-type | const | macro | feature
  change: "" # added | signature-changed | renamed | removed | stabilised
  feature: "" # the feature gating it, or "default"
  clause: "" # the SPECIFICATION.md clause this surface is bound by, e.g. PS-9
```

## Signatures

The exact signatures, as they will be written. Not prose about them — the code, so review is against
the thing rather than a description of it.

```rust
```

## Shape decision

Per item: the shape chosen, the alternatives rejected, and why each lost. `CLAUDE.md` requires this of
every unusual construct, and it is required here rather than in the commit message because the person
who can still cheaply disagree is reading this file.

Name the **open question or provisional clause** this resolves. A clause carried into implementation
still provisional is how a port gets frozen by accident.

| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |
| ---- | ------------ | ------------------ | -------- | -------- |

## Placement and re-export

Where each item lives, what re-exports it, and what a caller has to import to use it. The decision
recorded here is the **arrangement** — which module, which crate, what the prelude carries — not the
list of items, which is above.

State what coherence forbids where it bears on the choice: an orphan-rule problem discovered during
implementation is a placement decision made by the compiler after the fact.

## Visibility and stability

Per item: `pub` / `pub(crate)` / `#[doc(hidden)]`, whether it is `#[non_exhaustive]`, sealed, or behind
an off-by-default feature — and what semver promise it therefore carries.

An item that is `pub` because nobody decided otherwise is the failure this section exists to catch. It
is the same failure as a control that is persistent chrome because nobody decided otherwise, and it is
more expensive: a screen can be rearranged in a patch release, and a `pub` item cannot be withdrawn
from one.

| Item | Visibility | `#[non_exhaustive]` / sealed | Feature | Semver promise |
| ---- | ---------- | ---------------------------- | ------- | -------------- |

## What it costs a caller

The real budget, in numbers where numbers exist. An allocation per call, a `Send` bound that closes off
a target, an added dependency, a raised MSRV floor, a lifetime that forces a caller to restructure, a
trait that is no longer object-safe. "It will be fine" is not a budget.

State the cost on **both port flavours** where the item touches a port: a shape that is free on the
bare flavour and impossible on the `Send` one is not a shape this workspace can take (ADR-0001).

## What a user meets first

The one or two items a reader of the rendered docs lands on, and what carries them there — a crate-level
example, a re-export, a `#[doc(inline)]`. Everything cannot be on the front page; say what is not.

## The states the API must express

The full set this surface has to make representable: empty, absent, conflicting, exhausted, refused,
partially applied. Enumerate them here so they are designed into the types rather than discovered as a
missing enum variant during implementation.

## Anti-patterns

Concrete forbidden moves for this surface, each stated so it can be checked against a diff. Draw from
the binding constraints in `CLAUDE.md`, and add the ones specific to this project.

Standing, and never re-litigated here: no `#[async_trait]`; no `serde` in `happenstance-core`'s
defaults; `read` returns the stream at the top level; generic code binds `EventStore`, not
`SendEventStore`; no `unwrap`/`expect` in library code.

## The doctest

The runnable example a user would copy, written out here before it is written in the source. This is
the substitute for a mock, and it is a better one: the gate compiles it, so it cannot drift from the
API it demonstrates.

If the example must not compile — a `compile_fail` case pinning an arrangement the design forbids —
say which, and say what error the reader is meant to see.

```rust
```

## Sign-off

Who approved this surface, when, and any conditions attached. The human gate is the point: no automated
check in this pipeline can answer "would a person reach for this, and reach for it correctly?"
