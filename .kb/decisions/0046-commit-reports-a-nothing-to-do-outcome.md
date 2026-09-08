---
id: kb-decision-0046
title: commit reports a nothing-to-do outcome rather than an error
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0046
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  commit returns a two-armed, must_use success outcome — committed, or nothing to do carrying
  attempts — instead of Committed directly, so an empty decision stops travelling to the store
  as AppendError::NoEvents under a doc comment that contradicts itself. This unifies the
  command path onto the shape the runner side already has under PS-38, catches the
  forgot-to-push bug at compile time via a must_use two-armed outcome that cannot be discarded
  by await?, and costs no clause: ES-20 and ADR-0012 stay exactly as written. Landed at
  0ae10ab, in the last release where the return type could change for free.
depends_on:
  - kb-decision-0012
  - kb-decision-0030
  - kb-decision-0031
related:
  - kb-open-question-then-empty-emission-idiom-001
  - kb-decision-0047
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/empty-decision-outcome.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# commit reports a nothing-to-do outcome rather than an error

## Decision

`commit` and `commit_with` (`crates/happenstance/src/command.rs`) return a two-armed,
`#[must_use]` success outcome instead of `Committed` directly: one arm carries the position and
attempt count a normal commit already reported, the other carries only `attempts` and means the
decision emitted nothing. Before this change a decision that folded to "nothing to do" produced
an empty batch, which travelled to `store.append(&[], …)` and came back as
`Err(CommandError::Append(AppendError::NoEvents))` — a chain whose outer doc comment said "the
store failed the append for its own reasons" over an inner variant documented "this is a caller
bug, not a store failure," two contradictory claims about the identical value. The crate's own
test DSL disagreed with its production path on the same fact: `.then(&[])` on a decision that
emitted nothing passed, while `commit` on the identical decision returned `Err`. Landed at
`0ae10ab`, exactly as ratified, alongside the tags/scope check this decision shares a commit
with (`kb-decision-0047`).

## Why a new arm rather than an optional position

`crates/happenstance/src/runner.rs` already answers the identical shape one file over: a
projection run that committed nothing returns `Progressed { through: None, … }`, governed by
ADR-0031 (`kb-decision-0031`, the decision that put `run_projection` in that file) and PS-38 via
ADR-0030 (`kb-decision-0030`) — *"a `ProjectionId` no successful `commit` has named MUST read as
`Checkpoint::NeverRun`."* The command path's answer, by contrast, stood on nothing but a doc
comment on an enum variant. Importing the runner's `Option<SequencePosition>` shape directly was
rejected: the runner's `None` is a fact about the *world* — the stream was exhausted, and no
reachable caller bug produces it — while the command loop's empty batch is a value the caller's
own closure returned, with two possible causes the type cannot otherwise distinguish: a
deliberate no-op, or a fold arm that forgot to `push`. A `Result<T, E>` position field would make
case (b) a silent success. The two-armed outcome keeps case (b) loud, and louder than before: it
becomes a compile-time obligation at every call site rather than a runtime error on one path,
because a `#[must_use]` two-armed value cannot be discarded by `…await?;` without a warning — the
same guard `Committed` already carried. This reasoning reversed the brief's own first
recommendation (a new `CommandError` variant, additive on the type surface but silent at the
common call site) after review showed the loudness argument favored the opposite of what it was
first credited with.

## What this leaves unresolved on purpose

`error.rs:222-223`'s premise for treating an empty append as a caller bug was "there is no
position an adapter could honestly return" — a constraint on the **store port**, where every
success carries a `SequencePosition`. This decision lifts exactly that constraint one layer up,
giving the typed layer's outcome an arm that carries no position, without touching the
constraint where it is actually true. `AppendError::NoEvents` and ES-20 `[FROZEN]` — "an empty
batch is refused, and refused first" — are untouched: the store still refuses an empty batch
exactly as before, and `kb-decision-0012`, which transcribes that clause and is `accepted`
and therefore immutable, is neither amended nor superseded by any option this decision
considered. What remains genuinely open is named as its own atom rather than folded in here: the
outcome still does not distinguish "deliberate no-op" from "forgot to push," and how the test
DSL's `.then(&[])` should read against the new arm — whether it grows a dedicated
`then_nothing()` or keeps its double meaning — is a design question the option chosen does not
answer by itself.
