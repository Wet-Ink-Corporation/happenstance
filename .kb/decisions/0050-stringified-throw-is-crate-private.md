---
id: kb-decision-0050
title: StringifiedThrow is crate-private, and narrowing it made the compiler look
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0050
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  StringifiedThrow becomes pub(crate): both its in-crate roles — the !Send probe's positive
  control and ES-6's documentary alternative — are satisfiable by a private type, and no
  public-audience evidence exists. The consequence nobody named: pub methods on a pub type are a
  configuration dead_code can never fire in, so narrowing revealed message() and new() had no
  caller anywhere. The two remaining users sit in disjoint cfg configurations, which is why the
  two allows are gated differently; cfg(test) on the whole type is rejected because it would put
  ES-6's recorded alternative in the test profile.
depends_on: []
related:
  - kb-decision-0023
  - kb-decision-0009
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-cloudflare-feature-gate-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/stringified-throw-visibility.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# StringifiedThrow is crate-private, and narrowing it made the compiler look

## Decision

`StringifiedThrow` (`crates/happenstance-cloudflare/src/js.rs`) becomes
`pub(crate)`, and the crate root's invitation to build a `Send + Sync` error
shape out of it is withdrawn with it. Landed at `9225c00`, ratified as Option B
against Options A (keep it `pub`) and C (keep it `pub`, drop the classifier
method that gives it a hazard). The type's field, `message`, was already made
private in the same lane with a public `message()` reader; this decision is
about the type's own visibility, which the earlier audit routed separately
because neither Option A nor C for the field closed it.

## Why: two in-crate roles, no public-audience evidence

`StringifiedThrow` has exactly two jobs inside `happenstance-cloudflare`, and
both are satisfied by a private type. First, it is the `!Send` probe module's
positive control — `assert_send!(StringifiedThrow, "it holds a String, which is
the entire point of the stringified shape")` — named from inside the crate,
which `pub(crate)` answers exactly. Second, it is the recorded alternative for
ADR-0009's ES-6 fork: the shape every adapter would have been forced into had
`Error: Send + Sync` been added to the port. `spec/SPECIFICATION.md` cites
`js.rs` for that fork and names `JsThrow`'s declaration, not this one — a
`pub(crate)` type stays fully documented in the tree and readable in source; it
stops being a page a consumer can build against.

Nothing in the workspace consumes it as a consumer would. Every use — the two
constructors, the classifier, the probe's positive control, four `wasm32` unit
tests — is inside the defining crate. The crate root's own prose had argued for
the opposite: it told a reader that stringifying "loses a capability, not
information the caller needs" and that the two error shapes answer one question
identically, which invites exactly the public use nothing in the tree exercises.
That invitation is the strongest argument the type *should* stay public, and
also the reason the question was not free: the page argued for a route the API
would then withdraw. It was decided against because the crate has never
published a version with this type reachable — the registry carries only a
`0.0.0` name-reservation placeholder — so there is no audience the change can be
shown to have hurt.

## The consequence the brief did not name

Narrowing visibility made `dead_code` able to fire for the first time. A `pub`
method on a `pub` type is a configuration in which the lint can never trigger,
because an exported item always has a hypothetical external caller — so the
lint had been structurally silent about this type since it was written.
`pub(crate)` removed that shield, and the compiler immediately found `message()`
and `new()` had no real caller in any crate, any test, or either target.
`message()`'s own justification had been written entirely in terms of a
caller outside the crate — precisely the caller this change removes — so it had
outlived its own argument by one commit before anyone looked.

A second, sharper fact fell out of the same narrowing: the type's two remaining
users are not in the same build configuration. The host `cfg(test)` build
reaches the *type* — the `!Send` probe names it — and none of its methods; the
methods are reached only by `js.rs`'s `#[cfg(all(test, target_arch =
"wasm32"))]` tests. The two `#[allow(dead_code)]`s are gated differently for
that reason, each staying live in exactly the configuration that can perform the
check. `#[cfg(test)]` on the whole type was considered and rejected: it would
put ES-6's recorded alternative in the test profile and its counterpart
(`JsThrow`) in the shipped binary, making the two shapes the fork compares no
longer comparable within one build.

## What this does not decide

Whether `JsHandle` and `JsThrow` should be public — a different question with a
different answer available, since both are genuinely reachable from
`SqlError::Thrown` on a caller's error path and `StringifiedThrow` was not.
Whether `happenstance-cloudflare` publishes a feature table before it ships.
Nothing here amends ADR-0009 or ADR-0023; both are cited and untouched.
