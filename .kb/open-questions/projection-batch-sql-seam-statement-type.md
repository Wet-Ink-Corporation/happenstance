---
id: kb-open-question-projection-batch-sql-statement-type-001
title: Is &'static str the projection batch's final SQL seam, or does a minted Statement type follow?
kind: open_question
status: accepted
authority_tier: note
summary: >-
  X-4 of the pre-publication review found SqliteBatch::push took impl
  Into<String> with a one-line doc comment and no documented trust
  boundary, over the same seam ADR-0003 makes opaque (payloads are
  unvalidated Bytes). The remediation landed the type-level fix rather
  than a documentation-only one, per this repository's own doctrine that a
  rule nothing enforces is decorative: push now takes sql: &'static str,
  with push_raw_sql (impl Into<String>) kept as an explicit escape hatch,
  and a compile_fail,E0308 doctest holds the narrowing. This is Option A of
  three considered. &'static str proves provenance (the literal is in the
  source) but not shape — it does not stop a crafted literal, and
  Box::leak defeats it for anyone determined to. Option B (a minted
  Statement newtype via a sql! macro, giving a home for a future
  placeholder/parameter-arity check) and Option C (documentation only,
  rejected before writing since it is a control nothing enforces) were
  both declined for now. The surface is opt-in (behind projection-store,
  an unfrozen, semver-exempt port per PS-3/ADR-0036) and the crate is not
  yet on crates.io, so the narrowing was free; a second narrowing to a
  minted type after 0.2.0 would not be. Recommendation: keep Option A and
  defer the final shape to whoever freezes ProjectionStore under PS-2,
  since ADR-0017 already settled that the batch carries no universal write
  vocabulary across happenstance-sqlite, happenstance-neon and
  happenstance-ladybug, so the arity-check obligation cannot be answered
  once for all three by construction — only stated once, which it is not
  yet.
depends_on: []
related:
  - kb-decision-0036
  - kb-decision-0017
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/projection-batch-sql-seam.md
last_reviewed: 2026-09-07
---

# Is &'static str the projection batch's final SQL seam, or does a minted Statement type follow?

## What is true today

The pre-publication review's `X-4` found `SqliteBatch::push` accepting
`impl Into<String>`, documented by a single line ("Queues a statement to
run when the batch commits") with no `# Security` section, no mention of
binding, and no occurrence anywhere in the crate's `src/`, README or
`docs/` of `inject`, `parameteris(z)e`, `untrusted`, or `trust boundary`.
This is the one place in the workspace a consumer hands the library
free-form SQL text, and the values flowing through the surrounding
`params` are exactly the bytes ADR-0003 makes opaque and forwards
unvalidated.

The remediation landed the type-level fix, citing this repository's own
rule that a control nothing enforces is decorative
(`standards/rust/70-rustdoc-obligations.md` RS-70-5: "Nothing in the gate
reads prose"). `crates/happenstance-sqlite/src/
projection_store.rs:473` now reads:

```rust
pub fn push(&mut self, sql: &'static str, params: impl IntoIterator<Item = Value>) {
    self.push_raw_sql(sql, params);
}
```

with `push_raw_sql` (still `impl Into<String>`) kept beside it as a named,
opt-in escape hatch for statements whose shape is computed (an `IN (…)`
list sized by a variable key count, for instance). A `compile_fail,E0308`
doctest on `push` holds the narrowing, checked by `cargo test -p
happenstance-sqlite --all-features`. Every in-tree caller — the crate's
own `probe_write`, `probe_delete_all`, and `examples/transfers-on-
sqlite/src/main.rs:562` — already passed a literal, so the narrowing cost
nothing today; that is evidence about current usage, not proof no
consumer will ever need `push_raw_sql`.

Three facts bound the window: the surface is opt-in, behind
`projection-store`, whose manifest comment states the port is
`[PROVISIONAL]` and makes no semver promise; every in-tree caller already
complies; and `happenstance-sqlite` itself is not yet on crates.io (only a
`0.0.0` placeholder is), so this narrowing was free and a second one after
`0.2.0` would not be.

## The question

Does the seam stop at `&'static str` + `push_raw_sql`, or does it grow a
minted `Statement` newtype (e.g. a `sql!(...)` macro yielding `Statement`,
which `push` would take instead) that gives a home for guarantees
`&'static str` cannot express — chiefly a placeholder/parameter-count
check against the literal's text?

**Option A (landed).** Costs a caller nothing for a literal; a computed
statement moves to `push_raw_sql`. Breaking (compiler-caught, `E0308`) but
exempt under the port's semver waiver. Proves provenance, not shape: a
`const BAD: &str = "… WHERE k = 'x' OR 1=1"` still compiles, correctly,
because it is source-literal; a `Box::leak`'d computed string defeats the
type for anyone who wants past it. It is a guard rail, not a proof.

**Option B — a minted `Statement` type.** Same provenance guarantee, plus
room for an arity check nothing today performs. Costs every caller an
import and a wrapper at each call site, forever, for a guarantee
`&'static str` already gives. It is exported macro surface
(`standards/rust/41-declarative-macros.md` applies in full — `$crate`
paths, hygiene, a `compile_fail` per refusal). Breaking again if it lands
after Option A: two narrowings instead of one.

**Option C — documentation only.** Rejected before being written up, as
the paragraph-only fix the review's own remediation calls a control
nothing enforces.

## Recommendation

Option A, and then stop until PS-2's bar is met and `ProjectionStore`
freezes. The seam's final shape belongs with whoever does that freeze,
because `NeonProjectionStore` and `LadybugProjectionStore` have the same
seam in their own SQL dialects, and ADR-0017 already settled that the
batch carries no universal write vocabulary across the three — so the
arity-check obligation cannot be resolved once for all three by
construction. It can be *stated* once, though, and is not yet.

The strongest argument against waiting: the semver-exempt window is soft
here specifically (the manifest already disclaims the promise), unlike
the rest of this crate's public surface where the review's other findings
applied, so "wait for the freeze" costs less here than it would elsewhere
in the same theme.

## Cost of delay

Low and asymmetric. Option A already closed the injection-shaped finding;
delaying Option B costs an arity check nobody has asked for yet. Delaying
Option A would have cost the whole finding, which is why it was not
deferred.

## What this does not settle

Whether `PendingStatement::sql` stays `Box<str>` or becomes the minted
type; whether the same narrowing is owed by `NeonProjectionStore`'s and
`LadybugProjectionStore`'s write vocabularies (ADR-0017 forecloses
answering this once for all three, not whether the obligation should be
stated); and whether `happenstance-core` should document a trust-boundary
statement for `metadata`/`data` reaching a projection store at all, which
is a contract-level question this atom does not reach.
