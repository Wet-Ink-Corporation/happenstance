# Plan: a multi-item (OR) projection scope, additive in 0.8.0

Branch `claude/project-thread-qkv7ws-scope`, cut from `origin/phase-21-prep` at `11d16bb`.
Planned 2026-11-18. Authority: the owner's D3 = A (2026-11-16), "add a multi-item (OR)
projection scope as an additive provided method in 0.8.0"
(`/mnt/project-files/notes/plan-next-batch.md` §2C item 2, D3).

## Goal

One typed projection can follow "every case I own": an OR over several tag sets. Today
`Projection::scope() -> &Tags` is one AND'd tag set and the runners derive a one-item `Query`
from it (`crates/happenstance/src/boundary.rs:172-175`), so the only answers were one
projection per case or an owner tag that goes stale when ownership moves
(`deployment-patterns-for-synced-cqrs.md` P5 gap 1).

## Constraints and non-goals

- Additive only: `scope()` keeps its type and stays required. No existing implementation
  changes behaviour or checkpoint id.
- No dependency, no MSRV move, no core change: `Query` already ORs its items (VT-31).
- Non-goal: `DecisionModel` (the append-condition boundary) keeps one scope. A multi-item
  decision boundary is a different question (it widens a consistency boundary), not asked.
- Non-goal: per-scope event types. The type filter stays the projection's `EVENT_TYPES`.
- Non-goal: making `scope()` provided (see Decisions, follow-up).

## Invariants

- I1. A projection that does not override the new method derives a byte-identical query, so its
  `checkpoint_id` and `source_checkpoint_id` are unchanged (PS-25 goldens, PS-40 pin).
- I2. Every derived item carries the projection's `EVENT_TYPES`, so no scope can nominate an event
  the projection's decode does not expect.
- I3. Both runners and both id functions derive the query from one function, so the read, the
  main key and the pin can never disagree.

## Design

```rust
// crates/happenstance/src/runner.rs, on `Projection` (and so on `SendProjection`)
fn scopes(&self) -> &[Tags] {
    core::slice::from_ref(self.scope())
}
```

- **Name and shape: `scopes(&self) -> &[Tags]`.** Borrowed slice (OWN-13): the implementor stores
  `Vec<Tags>` or an array and returns it; the default borrows `scope()` as a one-element slice,
  which costs nothing and allocates nothing. Rejected: `-> impl Iterator<Item = &Tags>`:
  trait_variant 0.1.3 adds `+ Send` to every `-> impl Trait` in the derived `SendProjection`
  (`variant.rs:146-155`), a bound nobody asked for, and an iterator is not `Debug`/indexable for a
  caller checking its scope. Rejected: `-> Vec<QueryItem>` or `Query`: lets a projection widen
  the type filter past `EVENT_TYPES` (breaks I2) and makes the default allocate on every run.
- **Event-type filter:** item *i* = `QueryItem::new(EVENT_TYPES, scopes[i])`; items OR'd. The type
  filter is applied under every scope, never once around them (I2). An empty `Tags` in the slice
  means "every event of my types", as an empty `scope()` does today.
- **An empty slice** is refused: `Query::from_items` returns `InvalidQuery::NoItems`, which reaches
  the caller as `ProjectionError::Boundary` (runners, before anything is read) and
  `InvalidCheckpointId::Query` (id functions). Rationale: `Query` cannot say "nothing" (its seal
  is load-bearing, `query.rs` on `Query`), and quietly reading everything would be the opposite of
  what an owner of zero cases means. Documented on `scopes`, and on the two error arms whose docs
  said "unreachable".
- **Placement:** `boundary.rs` gets `pub(crate) fn derive_scoped_query(types, scopes: &[Tags])`;
  `derive_query` (single scope) stays for `DecisionModel`. A `pub(crate) fn projection_query<P>`
  in `checkpoint.rs` or `runner.rs` is the one caller for the four sites (I3):
  `runner.rs:927`, `fan_out.rs:401`, `checkpoint.rs:261`, `checkpoint.rs:364`.
- **Checkpoint (PS-25) and pin (PS-40, ADR-0109, #485):** unaffected in mechanism. The digest
  already sorts and dedups item encodings (`checkpoint.rs` "The encoding"), so scope order and a
  repeated scope do not move the id; adding or dropping a scope does, and the view replays (PS-25,
  correct: a newly owned case's history must be applied). The pin is derived from the same digest,
  so it follows. One doc sentence in `checkpoint.rs` ("The runner only ever derives one item")
  becomes false and is corrected.
- **Errors:** no new error type or arm (ERR-01): the existing `InvalidQuery::NoItems` is exact.

## Spec

New clause **PS-41**, `[PROVISIONAL]`, in §4.7 after PS-40, with E2E-60 in `spec/E2E-CASES.md`.
Rule `several_scopes_are_one_query`, a typed-layer test (no adapter can fail it, so not a
conformance rule, as PS-40). Falsifier: a deployment needing different event types under
different scopes. `cargo xtask spec-trace --write` regenerates §7.1–7.2; §7.6 prose updated.

## Test list (drive in this order)

New file `crates/happenstance/tests/projection_scopes.rs`; a rule fn held against three runners.

1. [x] `several_scopes_are_one_query`: cases c1, c2 owned, c3 not; log c1, c3, other-type-tagged-c1,
   c2, c1+c2; `run_projection` applies exactly c1, c2, c1+c2 once each in log order (leg A), commits
   under `checkpoint_id` through the last nominated position (leg B), and a non-`EVENT_TYPES` event
   tagged c1 is never nominated (the run would halt on decode) (leg C).
2. [x] `several_scopes_are_one_query_in_a_fan_out`: the same rule through `run_fan_out`.
3. [x] `the_scopes_rule_rejects_a_runner_reading_only_scope`: wrong runner = a wrapper that forwards
   `scope()` only, then the shipped runner. Fails A and B.
4. [x] `one_scope_keeps_its_checkpoint_id`: default `scopes` and an explicit one-element override
   give the same `checkpoint_id` and `source_checkpoint_id` as before (golden); order and repeats
   do not move it; adding a scope does.
5. [x] `no_scopes_is_refused_before_anything_is_read`: empty slice: `ProjectionError::Boundary(NoItems)`,
   nothing applied, no checkpoint or pin written; `checkpoint_id` returns `Query(NoItems)`.

## Decisions (H-16)

- Public API addition `Projection::scopes`: pre-approved by D3 = A.
- Made without the owner (recorded here, reversible before 0.8.0 ships): the name `scopes`;
  `&[Tags]` over an iterator or items; empty slice refused rather than reading nothing; type
  filter per item; `scope()` stays required.
- Follow-up, not taken: making `scope()` provided so a multi-scope projection need not implement
  it. Additive too, but beyond D3's wording.

## Risks

- A wrapper projection that forwards `scope()` but not `scopes()` silently narrows a multi-scope
  inner projection. Documented on `scopes`; it is the wrong runner the rule rejects.
- trait_variant copies the default body into `SendProjection`; `self.scope()` there must resolve to
  `SendProjection::scope`. Checked by compiling a `SendProjection` impl in the test.

## Progress

- Red 1 (compile, against the not-yet-written API):
  `cargo test -p happenstance --test projection_scopes` →
  `error[E0407]: method 'scopes' is not a member of trait 'Projection'` (projection_scopes.rs:131),
  and E0782 at the two `SendProjection::scopes` calls.
- Green: `Projection::scopes` + `derive_scoped_query` + `projection_query`, four call sites moved.
  6 passed.
- Red 2 (behavioural, mutation): `projection_query` reverted to `from_ref(projection.scope())`:
  `several_scopes_are_one_query` and `..._in_a_fan_out` fail legs A and C (applied 4, incl. `c3`),
  `the_ids_follow_the_set_of_scopes` and `no_scopes_is_refused_before_anything_is_read` fail.
  This showed my leg C reasoning was wrong (a scope-only runner still filters by type), so a second
  wrong runner, `DropsTheTypes`, was added for I2, and the leg docs corrected.
- Red 3 (mutation): item types dropped in `derive_scoped_query` (`take(0)`): both
  `several_scopes_are_one_query` tests fail. Reverted.
- Test 4 renamed `the_ids_follow_the_set_of_scopes` (one scope through `scopes` = same id as
  through `scope()`; order/repeat stable; adding moves both ids). I1's golden is already held by
  `the_runner_commits_under_the_derived_id` (`van_stock@dc525f3f5d262b19`) and `checkpoint_id`'s
  doctest, both through the default `scopes`.
- Added `the_send_flavour_provides_scopes` (a `SendProjection` impl overriding only `scope`).
- Spec: PS-41 `[PROVISIONAL]` in §4.7, E2E-60 in group B, §1.3 hand count 214→215 / 11→12,
  §7.6 prose, six `Unresolvable::Elsewhere` declarations in `xtask/src/spec_trace.rs`,
  §7.1–7.2 regenerated with `spec-trace --write`.
- Docs: rustdoc on `scopes` with a doctest; `ProjectionError::Boundary` and
  `InvalidCheckpointId::Query` docs; `checkpoint_id` docs; fan-out's one-runner-per-view note;
  `docs/rebuild-a-read-model.md`; CHANGELOG Added / Changed / trace note.
- Review of `cec1714` (changes requested), addressed:
  - F1: red `the_real_documents_orphan_no_case` failed `left: 60, right: 59` at
    `xtask/src/spec_trace.rs:4177`; the count and the quoted §7.6 sentence are now 60.
  - F3: `scopes` rustdoc and the rebuild how-to say a scope-set change re-applies every
    still-followed scope onto the rows already built and leaves a dropped scope's rows, and
    name the three remedies (clear rows in the reset batch, a new id, an idempotent fold).
  - F4: the companion test's citation is `spec_trace.rs:3853-3869` (the fn through its
    closing brace; `3873` would run past it).
  - F6: the default's doc states the contract, not the history.
  - F7: `the_send_flavour_forwards_an_overridden_scopes` (characterization: green on first run).
  - F5 left as a follow-up and F2 kept as built, per the coordinator.
