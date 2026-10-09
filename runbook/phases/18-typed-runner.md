# Phase 18 — The typed runner leaves its gate

**Goal.** `happenstance`'s projection runner is a stable, published API: no
`unstable-projection` feature between an application author and it.

**Why here.** The port under it has been `[FROZEN]` since `0.3.0` (ADR-0063), and
`happenstance-core`'s feature of the same name gates nothing. What still holds the
runner back is one layer up, and it is three things: `Projection::apply` is
synchronous (`crates/happenstance/src/runner.rs:95-99`), so a projection can push into
a buffered batch and cannot issue a statement into a live one; the runner halts on
the first failure and has no failure-policy seam, so PS-27's *skip and record*
has a count of zero; and there is no fan-out runner, so PS-30 binds nothing.
Phase 17 decided `apply`
([ADR-0074](../../.kb/decisions/0074-projection-apply-is-async.md)); this phase
builds it.

**Decisions it settles.** None new — it implements ADR-0074, phase 17's `apply`
record, and ADR-0070's `Chunk`. Discharges PS-18, PS-27 and PS-30 from `[DEFERRED]`, or
renews them with the disposition phase 16 gave.

**Work**

- [ ] #98 · **`Projection::apply` in ADR-0074's shape**, against both batch shapes the
      port was frozen on: a buffered batch, and `LivePostgresProjectionStore`'s
      live `sqlx` transaction. The spike in `experiments/apply-shape/src/shape.rs`
      is the reference, not the code to copy:
      - #135 · `#[trait_variant::make(SendProjection: Send)] pub trait Projection`, with
        `async fn apply(&mut self, event: Delivered<Self::Event>, batch: &mut
        <Self::Store as ProjectionStore>::Batch) -> Result<(), Self::Error>`.
        Inside the trait the store's types are spelled as explicit projections,
        never through the `StoreBatch<P>` / `StoreError<P>` aliases: those go
        through `<P as Projection>`, which `trait_variant`'s generated
        `SendProjection` can satisfy only with `Self: Sized` (six `E0277`s on
        1.97.1). Signatures outside the trait keep the aliases. `trait-variant`
        becomes a direct dependency of `happenstance`, under the pin ES-7's record
        sets.
      - **`Delivered<E>`**: `#[non_exhaustive]`, private fields, `id() -> EventId`,
        `event()`, `into_event()`, a **public** `Delivered::new(id, event)`, and no
        position accessor, held by a `compile_fail,E0599` doctest beside a
        compiling control fence. `recorded_at()` is optional; if it is added, its
        rustdoc repeats VT-9's "not an ordering key". **Decide it before the gate
        lifts**: after 1.0 a `recorded_at` must be `Option`-typed or arrive with a
        builder or second constructor, because `new(id, event)` keeps its two
        arguments.
      - A typed-layer test that a buffered run is ready at its first poll with no
        runtime (the spike's `tests/memory.rs:151` is the model).
      - #138 · Every `impl Projection` in the tree rewritten — the research brief counts
        twelve plus the `runner.rs` doctest, across `benchmarks/`, `examples/`,
        `crates/happenstance/tests/` and `experiments/polling-cost` — and
        `runner.rs`'s *"Why `apply` is synchronous"* rustdoc rewritten rather than
        left contradicting the record.
      - #142 · `crates/happenstance-postgres/src/live_projection_store.rs`'s paragraph
        saying `run_projection` cannot drive it removed, and a row-writing
        projection run through it against a real server.
- [ ] #99 · **PS-28's error type** (ADR-0074 decision 8). `type Error:
      core::error::Error + From<<Self::Store as ProjectionStore>::Error> +
      'static` on `Projection`; `apply` and `on_error` return `Self::Error`;
      `ProjectionError<R, W, A>` with `Apply { progress, position, source: A,
      rollback }` (PS-28 requires the failing position) and **no** default
      `A = W`, plus an arm for a failing `on_error`, the spike's `Policy {
      progress, position, source: A, rollback }`, named here. Whether an alias
      ships is this phase's choice. Write `pump_reports_the_failing_position`,
      defining *the last good position* as the last committed one.
- [ ] #101 · **The failure-policy seam** through which *skip and record* is written
      atomically with the checkpoint (PS-27), in ADR-0074's shape: a provided
      `on_error(&mut self, failure: &ApplyFailure<'_, Self::Error>, batch: &mut
      <Self::Store as ProjectionStore>::Batch) -> impl Future<Output =
      Result<Policy, Self::Error>>`,
      defaulting to `Policy::Halt`. `ApplyFailure` covers `Decode` and `Apply`,
      each carrying the `EventId`. The default is spelled `-> impl Future` with a
      block, and its parameters are named and discarded with `let _ = x;` (F2: an
      `_`-prefixed name fails pedantic clippy through `trait_variant`'s blanket
      impl). The runner's tripwire `no_skip_and_record_path_is_offered`
      (`crates/happenstance/tests/projection_clauses.rs`) is turned round in the
      same change. The Kestrel Motor shred case in
      `references/scenarios/README.md` §4 is the workload that needs it.
- [ ] #103 · **The savepoint caveat on a live batch** (ADR-0074 decision 6, finding F5).
      A server-refused statement aborts the live transaction, so a `Skip` after it
      writes nothing. The runner cannot issue a `SAVEPOINT` itself, because that
      is the generic write PS-9 forbids. Owed: `on_error`'s rustdoc states the
      limit; a live-batch test in which `apply` wraps its statement in a
      savepoint, the server refuses it, and the skip record commits with the
      checkpoint — the test that confirms PS-9's freeze on this leg, because the
      runner-issued savepoint was declined by design and the application's has
      not yet run; and a session-log line recording whether `LivePostgresBatch`
      gains an inherent savepoint helper (additive) or the pattern stays the
      application's.
- [ ] #107 · **PS-25's derived id** (ADR-0074 decision 9). The runner commits under an id
      derived from the projection's name and a digest of its derived `Query`: a
      hand-written 64-bit FNV-1a, in 16 lowercase hex digits, over each item's
      canonical encoding, the encodings sorted and **deduplicated**, then hashed.
      The per-item encoding must be **injective** — length-prefixed fields with a
      type/tag section marker, or a `Cc` separator VT-14 keeps out of every
      `EventType` and `Tag` — and the rustdoc says why. Zero dependencies. Pin
      the function and the encoding with golden values, because they become a
      1.0 promise. **Expose the derived id** (`happenstance::checkpoint_id`, or
      the name chosen here), and move `tickets-over-http` and
      `rebuilding-read-models`' `print_checkpoints` onto it. Its format is bound
      by [ADR-0082](../../.kb/decisions/0082-projection-id-is-validated-and-sync-is-reserved.md)
      §D5: VT-35's 255 bytes, so a name of at most 238 bytes, refused at
      derivation and never truncated or hashed; and a one-byte printable ASCII
      separator in no reserved prefix (`@` recommended), pinned here. Document
      the **adoption procedure**: `begin`, then `commit(empty_batch, derived_id,
      through, authority)` at the old checkpoint's position (PS-21), for a
      deployment whose query did not change. It is a behaviour change: a
      checkpoint under a bare id reads `NeverRun` after the upgrade, so the
      release **needs a `CHANGELOG.md` entry** stating the rebuild and the
      adoption procedure. Write `changed_query_starts_a_new_checkpoint`.
- [ ] #110 · **F1 in the runner's documentation** (ADR-0074 decision 10). A generic
      spawner writes `P: SendProjection<Store = St>` with `St` a type parameter,
      or every store future is `!Send`. This applies to the shipped
      `run_projection` too. The *what spawning it costs* rustdoc states the
      bounds, `Store = St` first
      (`experiments/apply-shape/tests/common/mod.rs:22-25`), with a compiled
      example.
- [ ] #112 · A fan-out runner holding N views over one log (PS-30, E2E-32), measured with
      `experiments/polling-cost` — a benchmark, not a conformance rule (CF-34).
      Over an async `apply`, `panicking_apply_rolls_back` needs `catch_unwind`
      around **each poll**, not around the call — a hand-written `poll_fn`
      wrapper, since the runner avoids `futures-util` (ADR-0074 decision 11).
- [ ] #115 · **Optional: a foreign-`rollback` clause.** ADR-0075 left `rollback` outside
      PS-15's MUST, and the specification's sentence about it is non-normative. If
      this phase makes it normative, it does so as a new clause with its rule
      (begin on one store, `rollback` on another, assert both unchanged) and a
      mutant that mutates a store on a foreign rollback, in the same change.
- [ ] #118 · A refusable reset implemented by at least one adapter (PS-18,
      `refused_reset_changes_nothing`).
- [ ] #120 · **ADR-0070's `Chunk`.** `run_projection`'s `chunk: NonZeroUsize`
      (`crates/happenstance/src/runner.rs:519`) becomes a named `Chunk`:
      `#[non_exhaustive]`, private representation, `Chunk::of(NonZeroUsize)` its
      first constructor — the shape `Retry` already has
      (`crates/happenstance/src/command.rs:43-62`). The fan-out runner above takes
      the same type. No `Default` and no observation seam: both are additive
      later, and ADR-0070 says what reopens each. Whether the parameter is
      `Chunk` or `impl Into<Chunk>` is this phase's compile to settle, and the
      session log records which. If `Chunk` cannot serve both runners without a
      second type, say so before the gate lifts — that is ADR-0070's falsifier.
- [ ] #123 · **The convergence declaration SY-20 needs.** The way a projection declares
      itself convergent, attached to the `Projection` trait ADR-0074 fixes, with
      SY-22's placement for it. Phase 13's
      `convergent_projection_is_interleaving_independent` consumes it, and phase
      13 runs after this phase for that reason — it is in 13's dependency row.
- [ ] #127 · **The clauses phase 16 gave this phase to freeze.** Each `freeze-by-18` row
      in [the 1.0 dispositions](../ledgers.md), and what freezes it:
      - **PS-16** — a typed-runner rebuild through `reset` against a multi-table
        or graph read model, with the `RESET_REFUSAL` clause composed in.
      - **PS-18** — the refusable reset above, a CF-39-shaped clause, and a
        `NoopProtectFixture` mutant. If this phase cannot deliver it, PS-18 is
        renewed past 1.0 as additive, by a record rather than by default.
      - **PS-25** — the derived id above, chosen by ADR-0074, built before the
        gate lifts.
      - **PS-27** — the failure-policy seam above and `skip_and_record_is_atomic`,
        with a mutant.
      - **PS-30** — the fan-out runner above and `panicking_apply_rolls_back`,
        with a mutant. **If this phase does not build fan-out, PS-30 becomes
        outside 1.0's surface**: a conditional MUST on a runner 1.0 does not ship.
        ADR-0066 states that fallback, so taking it is a session-log line and a
        ledger edit, not a new decision.
      - **PS-38** — against the fan-out runner. PS-23 is already frozen
        (ADR-0075), and the no-lagging-replica obligation is already documented on
        `ProjectionStore::checkpoint` and in `happenstance-neon`.
      - **SY-21** — `Projection::apply` built in ADR-0074's shape, frozen here on
        that signature — a convergent projection is handed an `EventId` and has no
        parameter through which the **local** `SequencePosition` could reach it
        (`EventId::position()` is the origin's, and is allowed) — and a
        typed-layer test that holds it. Phase 13 runs after this phase, and its
        sync rule exercises the same surface through replication.
- [ ] #132 · `unstable-projection` removed from `happenstance`, and every example that
      enabled it compiled without it. Removing a Cargo feature is itself a break
      (`references/adr/0063-the-projection-port-is-frozen.md:48-56`), and this
      phase runs after `0.4.0`'s window has closed, so the removal ships in the
      next breaking release — `1.0.0`, if nothing comes between — or the name
      stays declared and empty, as `happenstance-core`'s did until phase 17. The
      session log says which. If the gate cannot lift at all, ADR-0066 declares
      the runner exempt from semver at 1.0.

**Proof artefact.** `examples/rebuilding-read-models` compiled and run with no
`unstable-projection` anywhere in its dependency graph, and PS-18, PS-27 and
PS-30 each with a rule and a mutant that fails it.

**Exit criteria**

- [ ] No `unstable-projection` feature in `happenstance`.
- [ ] PS-18, PS-27 and PS-30 are out of `[DEFERRED]`, or renewed per phase 16.
- [ ] A projection can write into a live batch, demonstrated against a real
      database rather than a buffer.
- [ ] `run_projection` takes ADR-0070's `Chunk`, and so does the fan-out runner.
- [ ] Every `freeze-by-18` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions — PS-16, PS-18, PS-25, PS-27, PS-30, PS-38 and SY-21 — is
      `[FROZEN]`, or re-dispositioned by a record that says why. PS-30's
      outside-1.0 fallback needs only the session-log line and the ledger edit,
      since ADR-0066 already records it.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** 5–8 days.

**Session log**
