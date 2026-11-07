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

- [x] #98 · **`Projection::apply` in ADR-0074's shape**, against both batch shapes the
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
- [x] #99 · **PS-28's error type** (ADR-0074 decision 8). `type Error:
      core::error::Error + From<<Self::Store as ProjectionStore>::Error> +
      'static` on `Projection`; `apply` and `on_error` return `Self::Error`;
      `ProjectionError<R, W, A>` with `Apply { progress, position, source: A,
      rollback }` (PS-28 requires the failing position) and **no** default
      `A = W`, plus an arm for a failing `on_error`, the spike's `Policy {
      progress, position, source: A, rollback }`, named here. Whether an alias
      ships is this phase's choice. Write `pump_reports_the_failing_position`,
      defining *the last good position* as the last committed one.
- [x] #101 · **The failure-policy seam** through which *skip and record* is written
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
- [x] #103 · **The savepoint caveat on a live batch** (ADR-0074 decision 6, finding F5).
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
- [x] #107 · **PS-25's derived id** (ADR-0074 decision 9). The runner commits under an id
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
- [x] #110 · **F1 in the runner's documentation** (ADR-0074 decision 10). A generic
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
      **Built 2026-11-07** as `happenstance::run_fan_out`, catching nothing by the
      owner's call (ADR-0101, proposed), with PS-26/29/30's rules. Still open: the
      `experiments/polling-cost` fan-out pass, so the box stays unticked.
- [ ] #115 · **Optional: a foreign-`rollback` clause.** ADR-0075 left `rollback` outside
      PS-15's MUST, and the specification's sentence about it is non-normative. If
      this phase makes it normative, it does so as a new clause with its rule
      (begin on one store, `rollback` on another, assert both unchanged) and a
      mutant that mutates a store on a foreign rollback, in the same change.
- [ ] #118 · A refusable reset implemented by at least one adapter (PS-18,
      `refused_reset_changes_nothing`).
- [ ] #120 · **ADR-0070's `Chunk`.** `run_projection`'s `chunk: NonZeroUsize`
      (`crates/happenstance/src/runner.rs:858`) becomes a named `Chunk`:
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
        or graph read model, with the `RESET_REFUSAL` clause composed in. Built
        at #129, and on SQLite only, so the freeze record has to say that the
        `TRUNCATE` axis PS-16's falsifier names is held by the port rule's runs
        on Postgres, Neon and Cloudflare, not by the typed test.
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

- 2026-11-02 — #135 and #138 built on `claude/project-thread-u1dtdj-98`, for #98:
  `Projection::apply` is `async` and handed a `Delivered<E>`, `SendProjection` is
  derived with `trait_variant`, and every `impl Projection` in the tree is rewritten.
  The runner hands `apply` the `EventId` it read (SY-21's typed-layer test). The #98
  box stays open: its live-Postgres leg is #142, which also owns
  `happenstance-postgres`'s four "apply is synchronous" passages. SY-21 stays
  `[PROVISIONAL]` until #127.
- 2026-11-02 — #120: `Chunk` shipped as a plain `chunk: Chunk` parameter, not
  `impl Into<Chunk>`. The compile showed `impl Into` keeps only the sites already
  holding a `NonZeroUsize`, while the seven `x.try_into()?` sites break either way
  and break worse under `impl Into` (inference falls to `!`). The examples spell
  it `Chunk::of(CHUNK.try_into()?)`. One `Copy` type serves the fan-out runner
  too, so ADR-0070's falsifier did not fire.
- 2026-11-02 — #118: `happenstance-sqlite` implements PS-18's refusable reset (owner decision
  D4). `SqliteProjectionStore::protect_from_reset`, `allow_reset` and
  `is_protected_from_reset` write, lift and read a mark in `projection_protection`, and
  `reset` reads it inside its `BEGIN IMMEDIATE` before any of the caller's statements. The
  SQLite fixture declares `RESET_REFUSAL`, so `refused_reset_changes_nothing` runs there. CF-43
  is minted `[PROVISIONAL]` and `NoopProtectFixture` registered against it. The schema question
  went with the recommended option while it waits on the owner: the table is created on every
  open, and the marker rises to 2 only inside the first protection. That reverses the fixture's
  old argument against a protected-projections table, and this line and the rewritten fixture
  doc record it rather than a decision atom. PS-18 stays `[DEFERRED]`; its freeze and the ledger
  rows are #127's.
- 2026-11-02 — #99: `Projection` gains `type Error` (`From` the store's error) and
  `apply` returns it; `ProjectionError<R, W, A>` carries it in `Apply`, with no
  default `A = W` (a `compile_fail,E0107` fence holds that). *The last good
  position* is **the last committed one**: `pump_reports_the_failing_position`
  pins the refused event's position on the error and the checkpoint at the last
  chunk boundary, and PS-28's pairing finding is rewritten as resolved (D3).
  No alias ships (D1); an application writes its own one-line alias. The
  failing-`on_error` arm is declared now as `ProjectionError::Policy`, with its
  accessors tested, and nothing constructs it until #101 (D2). A generic caller
  that moves the error into `anyhow` now needs `P::Error: Send + Sync`, as
  `examples/rebuilding-read-models` shows. `.kb/decisions/0074…:226` still cites
  `runner.rs:158`, *"two type parameters, not three"*: its referent is the text
  this change deleted, so it stays as the record of the shipped 0.4 shape.
- 2026-11-03 — #142, the #98 box's live leg: `run_projection` drives
  `LivePostgresProjectionStore` with a projection whose `async fn apply` awaits
  `LivePostgresBatch::execute`. Three tests in `happenstance-postgres`'s
  `tests/live_projection.rs` (rows and checkpoint together across chunks; a chunk the
  server aborted discarded whole; rows already sent rolled back by a commit fault) run
  in the `live-postgres` job's existing trio, whose count derives from `--list`. D1 went
  with option A: a path-only `happenstance` dev-dependency, the first adapter → typed
  layer edge, dev-only. D2 went with (a): the docs say the live store is drivable and
  keep the buffered store the default, with the held connection and the
  no-savepoint abort (#103) stated. The four rewrites are line-neutral, so ADR-0084's
  and the spec's citations below them hold. `.kb/decisions/0074…:68` and `:331` still
  cite `live_projection_store.rs:38-47` as the *cannot be driven* paragraph; that
  text is the one replaced, so they stay as the record of what it said.
- 2026-11-03 — #107: `run_projection` reads and commits under
  `checkpoint_id(&projection)`, `name@` and 16 lowercase hex digits of a
  hand-written 64-bit FNV-1a, and a changed query starts a new checkpoint
  (`changed_query_starts_a_new_checkpoint`). The separator is `@` (D2). The
  per-item encoding is injective by `Cc` terminators VT-14 keeps out of every
  field — US after each type and tag, RS after the types, GS after the item,
  FS alone for `Query::All` — not length prefixes, whose `usize` would hash
  differently on `wasm32` (D1). Goldens were computed by two independent
  models before the Rust ran. A name over 238 bytes is refused as
  `InvalidCheckpointId::NameTooLong`, and a `sync/` watermark name as
  `Refused(Reserved)`; the runner reports either as
  `ProjectionError::CheckpointId`. Exposed as `checkpoint_id`,
  `InvalidCheckpointId` and `MAX_CHECKPOINT_NAME_LEN` (D3). `Projection::id`
  keeps its name and is documented as the name (D5); renaming it so a bare-id
  read is a type error is a pre-#132 question. No `adopt` helper (D6): the
  adoption procedure is a compiled fence on `checkpoint_id` and a CHANGELOG
  entry (D7). Ten test sites moved onto `checkpoint_id` with no expected value
  changed (D4). PS-25 stays `[PROVISIONAL]` for #127.
- 2026-11-06 — #129: PS-16's composed half, in
  `examples/rebuilding-read-models/tests/rebuild_through_reset.rs`, a second target
  in a package that already compiles the typed layer and `happenstance-sqlite`, so
  no crate gains a dependency edge (D1). A typed `Projection` writes two tables, one
  accumulating and one insert-only, is reset under `checkpoint_id(&view)` and
  rebuilt to the same rows at two chunk sizes
  (`rebuild_through_reset_restores_every_table`). The runbook procedure is written
  down and shown to fail the same predicate, a reset whose third statement fails
  leaves both tables and the checkpoint, and a protected reset through a handle
  opened before the mark is refused while the view keeps running. PS-16's `Rule:`
  line names the two rules in place, line for line, and `proof.rs` holds three of
  the five names. PS-16 stays `[PROVISIONAL]` for #127 (D2). `view.id()` passed to
  `reset` or `protect_from_reset` still type-checks and targets a row the runner
  never writes; the file's documentation says so and no API changes (D3). SQLite
  only: the `TRUNCATE` caveat is on #127's PS-16 line above (D4).
- 2026-11-06 — #110: the bounds a spawner generic over the projection writes, measured
  on the shipped runner by removing each in turn (D1, the minimal set at merge):
  `S: SendEventStore<Error: Send> + Sync + 'static`,
  `P: SendProjection<Store = St, Event: Send, Error: Send> + 'static`,
  `St: SendProjectionStore<Batch: Send> + Sync + 'static`. Each removal fails: `Store = St`
  as `P::Store` with five `E0277`s on the store's futures and batch (F1); `Batch: Send`,
  `S::Error: Send` and `P::Error: Send` with `E0277`, the errors on `Stopped` across
  `rollback`; `Sync` on either store with `E0277`, even when the stores are owned by the
  task rather than in an `Arc`; `'static` with `E0310`. `Event: Send`, which the plan
  expected only after #101, is needed now: `trait_variant`'s blanket `Projection` impl is an
  `async fn` forwarding to `SendProjection::apply`, and its state holds the
  `Delivered<P::Event>`. Not needed: `P: Send` (a supertrait), `St::Error: Send`, any
  `Error: Sync`. #101 owes the `Error: Sync` its `on_error` borrow adds. The fences sit on
  `Projection`'s page, not `run_projection`'s, because `entry_points_state_their_flavour`
  forbids the name `SendEventStore` there; `run_projection`'s page lists the bounds and
  points to them. The `compile_fail,E0277` fence (D2) is not checked for its code on stable
  rustdoc, so its five `E0277`s were read off a compiled copy. D3:
  `a_concrete_bare_projection_spawns`. `experiments/apply-shape/tests/shipped_spawn.rs`
  still binds the bare, synchronous shape and stays as the spike's record.
- 2026-11-06 — #101: `Projection` gains the provided `on_error`, handed an
  `ApplyFailure` (`Decode` or `Apply`, each carrying the `EventId` and a borrowed
  source) and the open batch, answering a `Policy` (`Halt`, `Skip`). The default
  halts; no projection in the tree was edited. The owner decisions were taken **as
  recommended** while the owner is asked, one commit each where the code allows:
  O1(a), a generic spawner adds `P::Error: Sync`, measured by the `E0277` on
  `run_projection_spawns_from_code_generic_over_the_projection` without it; O2(a),
  `Progressed::skipped`; O3(a), `const fn` constructors `ApplyFailure::decode` and
  `::apply`, which the rule's wrong runners need to live outside the crate. A skip
  counts toward `chunk`, and the checkpoint moves to a skipped event (the window
  rule): a log of nothing but poison still commits every `chunk` events.
  `skip_and_record_is_atomic` reads PS-27's *"crash injected between them"* as
  *both present after a skip, both absent after a failure injected between the
  record and the commit, and both written by the restart*; it runs against
  `run_projection` and three wrong runners in `tests/failure_policy.rs`, each
  failing the legs it names. The tripwire item above names the old test; its
  replacement is `the_failure_policy_is_the_projections_and_halts_by_default`,
  which pins the seam's presence. PS-27 stays `[DEFERRED]` for #127; PS-26's rule
  stays #112's.
- 2026-11-06 — #123: the convergence declaration is `happenstance::ConvergentProjection`,
  `pub trait ConvergentProjection {}`, behind `unstable-projection`, in its own
  `src/convergence.rs`. D7's letter, `ConvergentProjection: Projection`, compiled for
  concrete projections but not for code generic over a `SendProjection` that also spawns
  the run: the supertrait's where-clause shadows `trait_variant`'s blanket impl (`E0271`
  on `P::Store`, `E0277` on `apply`'s future, `E0034` on `id()`), and return-type
  notation is `E0658` at 1.97.1. The owner calls were taken **as recommended** while the
  owner is asked: O1(B), one marker with no supertrait (the likeliest alternative, two
  markers, is a swap inside `convergence.rs`); O2, non-convergence is declared by not
  implementing it, recorded in SY-22's prose, so the SY-22 rule is a `compile_fail`
  beside a control rather than a runtime filter. `tests/convergence.rs` holds both
  flavours declaring it, two interleavings committing one read model (and diverging
  over re-minted identities), and #110's spawner with `+ ConvergentProjection`, which
  is `E0271` under D7's spelling. Phase 13's rule (#141) needs `happenstance` as a
  dependency of `happenstance-sync-testkit`: a new inward edge, and phase 13's call.
- 2026-11-06 — #103: the savepoint caveat. `Policy::Skip`'s page carries it in full
  (`policy.rs`, which has no line citations): a buffered batch meets its database only at
  commit, so no policy is asked; a live transaction aborts on a statement the server
  refuses, and the record `on_error` writes is refused with it, unless `apply` set a
  `SAVEPOINT` and issued `ROLLBACK TO SAVEPOINT` before returning the refusal.
  `on_error`'s rustdoc states the limit and links there in a line-neutral rewrite, as
  are `live_projection_store.rs`'s two passages, the README's, #142's CHANGELOG line and
  PS-27's prose, so no `runner.rs:N`, `live_projection_store.rs:N` or
  `SPECIFICATION.md:N` citation moved. `tests/live_projection.rs` runs the pattern
  through `run_projection` against PostgreSQL 17.10 with a `CHECK` cap the server
  enforces after the event's journal row has landed:
  `a_savepoint_in_apply_lets_skip_record_a_server_refusal` commits the writes either
  side of the refusal, none of the refused event's, its skip record with SQLSTATE
  `23514` and the checkpoint at the head; its control,
  `a_server_side_failure_leaves_nothing_for_on_error_to_write_into` (ADR-0074's name),
  runs the same `apply` with no savepoint and stops with `ProjectionError::Policy`,
  whose source is `on_error`'s own write refused as `25P02`, having committed nothing.
  An application-issued `SAVEPOINT` and `ROLLBACK TO SAVEPOINT` work through
  `sqlx::query` on the live batch, so the plan's risk R1 did not arise. The decision
  line, taken as recommended while the owner is asked (O1(a)): **`LivePostgresBatch`
  gains no savepoint helper: three `execute` calls in `apply` suffice and are run live;
  a helper stays an additive option.** PS-9 is `[FROZEN]` and its text is not edited
  (O2(b)): its narrative still says the savepoint pattern "has not been run", and
  updating that evidence to name the test above is left for #127's reconcile.
- 2026-11-07 — #112: `happenstance::run_fan_out`, with `FanOut` and `ViewError`, drives
  N views of one projection type from one read. Each view keeps its own chunk window,
  opens its write set lazily and decodes for itself, so it ends with the rows,
  checkpoint and `Progressed` `run_projection` alone would give it; an exhaustive
  sweep (chunk 1–5 × poison position × policy, three views) holds that, and
  `skip_and_record_holds`' leg B runs unchanged through a one-view fan-out. A view's
  failure stops it alone, as `ViewError::Stopped`, and a failed read stops every
  running view once, in `FanOut::read`. Two calls are **the owner's**, each made on a
  card: **O1 = B, "Catch nothing"** (Ryan, 2026-11-07 04:38Z), so a panic unwinds out
  of the call and the dropped write sets roll back; ADR-0101 records it as
  `proposed` and reads ADR-0074 decision 11 as binding only a catching runner. And
  **(A) for AC-012** (Ryan, 04:58Z): `no_local_projection_store` refused a local
  recording store, so the instrument is `happenstance-testkit`'s new
  `RecordingProjectionStore`, a public fixture beside `FaultyStore`, and the test is
  unchanged. PS-30's rule fails three catching runners; PS-26's fails one that asks
  the first view's policy for all; PS-29's (O3) fails one that stops every view.
  A spawned fan-out asks `run_projection`'s bounds plus `St::Error: Send`, measured;
  setup derives every key before its first await, so `P: Sync` is not asked. Spec
  hunks are line-neutral, no FROZEN MUST moved, and PS-30 stays `[DEFERRED]` for
  #127. PS-38 is exercised by `fan_out_resumes_each_view_from_its_own_checkpoint`
  (N ids on one handle). Not done: the `experiments/polling-cost` fan-out passes
  002/003, left for a follow-up.
