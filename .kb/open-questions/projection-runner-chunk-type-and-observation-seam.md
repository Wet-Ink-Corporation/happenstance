---
id: kb-open-question-projection-runner-chunk-observation-001
title: Does chunk get a named type and a default, and does the runner get an observation seam?
kind: open_question
status: superseded
authority_tier: note
summary: >-
  Two coupled design questions on run_projection's surface
  (crates/happenstance/src/runner.rs), both priced at zero cost of delay
  on code by ADR-0036's stated unstable-projection semver exemption and
  both routed to phase 6's port-freeze pass by the audit that raised them.
  Question 1 (PS-3 [PROVISIONAL]): chunk is an undefaulted NonZeroUsize
  today, documented but untyped, unlike the sibling Retry type
  (#[non_exhaustive], named constructors, a compile_fail fence, a
  source-reading test asserting no default). Recommendation: give it a
  named Chunk type and a default, but only after a measurement nobody has
  run — wall time, peak memory and commit count against
  SqliteProjectionStore at chunk in {1, 8, 64, 1024, 65536} over a large
  log — because the strongest counter-argument (a flat curve across three
  orders of magnitude makes the type ceremony) is live and unresolved.
  Question 2: the runner exposes no observation seam — no callback, no
  channel, no tracing; Progressed arrives once, at the end; the only
  operator-visible progress is polling a second handle's checkpoint.
  Recommendation: run_projection_observed, an additive second entry point
  taking a callback, mirroring this crate's existing commit/commit_with
  plain-door/full-door shape, with tracing explicitly declined at this
  layer because the workspace has zero tracing:: usage today and adopting
  it is a larger decision than the runner. Strongest counter: no operator
  has hit this gap yet, because there are no operators — 2A (documentation
  only, already landed) may be the whole correct answer until a real case
  is reported. A structural trap was also found and partially closed: three
  spec/SPECIFICATION.md citations anchor to runner.rs:401, inside the
  doctest fence, and spec-trace cannot detect a citation drift there
  because it only anchors citations whose subject it can derive from
  surrounding prose — a test now pins prose additions below the fence, but
  repointing the three citations belongs to spec/SPECIFICATION.md's owner.
  Resolved 2026-09-29 by kb-decision-0070. Question 1: 1B, a named #[non_exhaustive] Chunk with named
  constructors and no Default. It is decided now because the parameter sits in run_projection's
  signature, and phase 18 builds it before lifting unstable-projection. A default waits for the
  chunk-size measurement, and adding it later is additive. Question 2: 2A at 1.0. Progressed is
  already #[non_exhaustive], run_projection_observed is additive in any later minor, and tracing is
  declined at this layer. That departs from this atom's 1C and 2B on one argument: a freeze forces
  only the half that cannot be added later.
depends_on: []
related:
  - kb-decision-0036
  - kb-decision-0007
  - kb-open-question-ps-19-scope-narrower-001
  - kb-playbook-anchoring-citations-001
  - kb-decision-0063
  - kb-open-question-apply-synchronous-live-store-001
  - kb-decision-0070
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/projection-runner-chunk-and-observation.md
  - crates/happenstance/src/runner.rs
  - crates/happenstance/tests/projection_runner_page.rs
last_reviewed: 2026-09-29
---

# Does chunk get a named type and a default, and does the runner get an observation seam?

## What the exemption buys, and what it does not

ADR-0036 ships the projection port behind `unstable-projection`, off by
default, and records that `cargo-semver-checks` will not police the
surface. That buys time **on the code**: `run_projection`, `Progressed`
and `ProjectionError` make no semver promise, so a named `Chunk` type or
an additive observation entry point are both free to add later. It buys
**nothing on the page** — a doc comment is not semver-gated, and whoever
runs the function today is not protected by a future exemption. It also
does not touch `spec/SPECIFICATION.md:5846`'s normative-reading assertion
that the runner "writes nothing anywhere, logs nothing," which an
observation seam would have to be reconciled against, in a different
file than this one.

Documentation for both gaps has already landed on `runner.rs`'s page,
pinned by `crates/happenstance/tests/projection_runner_page.rs`. No
behaviour changed.

## Question 1 — chunk: named type, default, both, or neither?

**1A — leave it `NonZeroUsize`, documented (as landed).** Costs a caller a
paragraph to read and a number to guess; nothing enforces a sane one.
Against it: this crate already answered the analogous question the other
way for `Retry` — a `#[non_exhaustive]` type with named constructors, a
`compile_fail` fence, and a test asserting it has no default. Two
caller-supplied bounds treated oppositely in one crate reads as meaning
something it may not.

**1B — a named `Chunk` type, no default.** Mirrors `Retry`'s shape
(`Chunk::of(NonZeroUsize)`, room to grow `Chunk::adaptive()` later without
a break). Costs one more name at every call site for a guarantee
`NonZeroUsize` already gives; the counter-argument is that a newtype
adding no invariant over its inner type is ceremony, unlike `Retry`, which
has two distinct construction shapes.

**1C — a named type and a default.** `Retry`'s own page argues a hidden
default would hide a worst case nobody chose (how many times a command
re-runs). That argument does not transfer here: every chunk size produces
the *same* read model, so a default hides only a performance worst case,
not a semantic one — a weaker case, made freshly rather than borrowed.
What it buys: the doctest stops teaching a magic number (`64`) by example,
which is how such numbers propagate into consumer code.

**Recommendation: 1C, at the freeze, after the measurement** — wall time,
peak resident memory and commit count against `SqliteProjectionStore` at
chunk sizes {1, 8, 64, 1024, 65536} over a log large enough to show the
curve, to live in `experiments/`, outside the gate. The strongest
objection: if the curve is flat across three orders of magnitude, 1A was
right all along and everything above is ceremony — a live, currently
untested possibility.

## Question 2 — does the runner get an observation seam?

**2A — nothing; the checkpoint is the API (as today, now documented).**
The runner stays a pure function with no policy surface, the same
argument that already beat an `on_error: SkipPolicy` knob on this page.
Against it: the checkpoint was observable only to a caller who already
knew to open a second handle and poll — a hidden gap, now at least a
documented one, which is real progress but not the same as a seam.

**2B — `run_projection_observed`, a second entry point taking a
callback.** Mirrors this crate's existing `commit`/`commit_with`
plain-door/full-door shape. Additive and free under the exemption; the
plain door is unchanged. Against it: a synchronous callback in an async
loop is a foot-gun for any caller who does I/O in it, stalling the
runner — documentable, not eliminable. Also doubles the page to maintain.

**2C — `tracing` spans.** Puts a dependency and an ecosystem choice into
the typed layer; the workspace has zero `tracing::` usage today, and this
would be the first. Declined at this layer as a decision larger than the
runner's own surface, though it is "the thing operators actually have
tooling for."

**Recommendation: 2B at the freeze, with 2C explicitly declined at this
layer.** Strongest counter: nobody has asked, because there are no
operators yet — the gap was found by an audit reading the page, not by
anyone hitting it in practice. 2A plus the landed documentation may be the
whole correct answer until a real reported case exists.

## The citation trap this file sets

Three `spec/SPECIFICATION.md` sentences cite `run_projection` at
`crates/happenstance/src/runner.rs:401`, a line inside the doctest fence.
Prose added above the fence shifts the call downward and leaves those
citations pointing at whatever now occupies line 401 — and `spec-trace`
does not catch this: it only anchors citations whose subject it can derive
from adjacent prose, and these three are not among the anchored set (a
prior in-file comment claiming `spec-trace` catches this was wrong,
confirmed by measurement: moving new prose above the fence produced an
identical "401 citations checked" summary and exit 0). A test,
`prose_added_to_this_page_stays_below_the_fence`, now holds the line from
this side; repointing the three citations to the `pub async fn
run_projection` line itself belongs to `spec/SPECIFICATION.md`'s owner.

## Cost of delay

Zero on code, by ADR-0036's exemption. Non-zero and already accruing on
the page for undocumented callers, which is why the documentation landed
first and the type/seam questions did not. The chunk-size measurement has
no deadline of its own but gates 1C, so scheduling it is the one thing
that must happen before the freeze rather than at it.

## What this does not settle

Whether `Progressed` grows fields (its own doc comment reserves that for a
later observability pass, which 2B could be); whether `spec/
SPECIFICATION.md:5846`'s "logs nothing" is normative or observational —
2C would contradict it, 2B arguably would not, and the specification's
owner decides which; and anything about `ProjectionStore` itself, since
both questions are about the typed layer's `run_projection` surface, not
the port.

## Closed — 2026-09-29

`kb-decision-0070` decides both questions. It departs from this atom's recommendations on a timing
rule rather than on any of the analysis above: a freeze forces only the half that cannot be added
later.

**Question 1: 1B.** `chunk` becomes a named `Chunk`. It is `#[non_exhaustive]`, has a private
representation, and is built through named constructors, starting with `Chunk::of(NonZeroUsize)`.
That is the shape `Retry` already has in the same crate. The parameter sits in `run_projection`'s
signature (`crates/happenstance/src/runner.rs:514-520`, `chunk` at `:519`), and that signature
becomes published surface the moment phase 18 lifts `unstable-projection`. So the type is decided
now, and phase 18 builds it before the lift. The fan-out runner phase 18 also builds (PS-30) takes
the same `Chunk`. Whether the parameter is `Chunk` or `impl Into<Chunk>` is left to phase 18's
compile. **There is no `Default`.** PS-14 requires the same read model at every chunk size, so a
default would hide only a cost, and the measurement this atom asked for — wall time, peak memory
and commit count at {1, 8, 64, 1024, 65536} — still does not exist. Adding `impl Default` later is
a minor change. That measurement is therefore the one residue of question 1, and it has no phase.
It is `kb-decision-0070`'s reopen condition, not an obligation.

**Question 2: 2A at 1.0.** The checkpoint is the API. `Progressed` is already `#[non_exhaustive]`
and can grow fields. `run_projection_observed` (2B) is additive in any later minor, and it waits for
an operator who needs progress mid-run that polling a second handle cannot give. `tracing` (2C) is
declined at this layer. That keeps `spec/SPECIFICATION.md:6104`'s "writes nothing anywhere, logs
nothing" true, so the question in "What this does not settle" — whether that sentence is normative
or observational — does not need an answer yet.

**The citation trap, re-read.** The three specification sentences this atom found anchored inside
the doctest fence have since been repointed. They now cite `runner.rs:524` (`spec/SPECIFICATION.md:6104`,
`:6045`, `:6173`), which is inside `run_projection`'s signature (the function starts at `:514`, and
`:524` is `C: Codec,`), so they are no longer in the fence, but they are still not on the `pub async
fn` line. Phase 18 rewrites this signature to take `Chunk`, and the citations drift again unless
they are repointed in that change. `prose_added_to_this_page_stays_below_the_fence`
(`crates/happenstance/tests/projection_runner_page.rs:142`) still holds the page's side. Closed by
hand in phase 16. No accepted decision was edited.
