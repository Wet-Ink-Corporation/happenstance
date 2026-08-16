---
item: HS-S0023
stage: implement
created: 2026-08-12T13:46:19.582Z
updated: 2026-08-12T13:46:19.582Z
---

# Acceptance ledger — The command loop — read, decide, append, retry on ConditionViolated

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "**P1 is protected from the lost update `append`'s return value invites.** GIVEN P1 has written one `commit` call against a boundary another writer is also appending to, WHEN the loop builds the `AppendCondition` for any attempt — the first or the fourth — THEN its `after` is the last position that attempt's own `read_decision_model` actually observed (`crates/happenstance-core/src/store.rs:315-330`), never the position the previous attempt's `append` returned (`:131-145`), so no event sitting below that return is excluded from the condition"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:294 builds the condition as
    `AppendCondition::new(query).after_opt(anchor)`, where `anchor` is
    `read_decision_model`'s second return value taken at :279 inside the same attempt;
    nothing in the loop reads `append`'s return except to fill `Committed.position`
    (:302). `cargo test -p happenstance --all-features`:
    command_loop.rs::after_anchor_comes_from_the_read ... ok — a recording wrapper
    captures every submitted condition and asserts each attempt's `after` equals the
    anchor its own read returned (two attempts, `None` then `Some`, so the comparison
    is not two `None`s). The named wrong implementation is compiled beside it:
    ::threads_append_return_is_rejected ... ok shows the append-return anchor failing
    to catch an event at a position below it.

- id: AC-002
  criterion: "**P1 can build, inspect and discard a decision without consequence.** GIVEN a domain rule P1 encoded as an `Err` return from their `decide` closure, WHEN the loop reaches that refusal, THEN it returns `CommandError::Refused(d)` carrying P1's own error type, and the store is byte-identical to before the call — no partial append, no probe write, and nothing before the append performed any write at all (AC-U12, `_decomposition.md:170-178`)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:290 — `decide(&model).map_err(CommandError::Refused)?`
    returns before `encode` (:291) and before the append (:296), so the refusal path
    performs no write at all. `cargo test -p happenstance --all-features`:
    command_loop.rs::refusal_appends_nothing ... ok — compares the positions the store
    actually assigned before and after (never literal values) and asserts the recording
    wrapper saw zero `append` calls.

- id: AC-003
  criterion: "**P1 gets back a value that says what happened, not a tuple that freezes what can be said.** GIVEN an uncontended store, WHEN P1 awaits `commit`, THEN they receive `Committed { position, attempts }` where `position` is what `append` assigned and `attempts` is 1; the type is `#[non_exhaustive]` and `#[must_use]`, so a later observability field is not a breaking change and a dropped outcome does not compile quietly (AC-U06, AC-U09; `_design.md`, *Signatures*)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:77-89 — `Committed { position, attempts }`,
    carrying both `#[must_use = "…"]` (:77) and `#[non_exhaustive]` (:78); `pub use`d
    at crates/happenstance/src/lib.rs:157. The counter is incremented once per append
    submitted (:270) and is the same one `Exhausted` reports, so the two cannot
    disagree. `cargo test -p happenstance --all-features`:
    command.rs::tests::first_try_reports_one_attempt ... ok,
    command_loop.rs::commit_is_commit_with_json ... ok (attempts == 1 and
    `Committed.position` equals the position the store assigned),
    docs_composition.rs::committed_is_non_exhaustive_and_must_use ... ok.

- id: AC-004
  criterion: "**P1's retry re-decides against the world as it now is, and their own inputs survive it.** GIVEN a competing writer lands an event between P1's read and P1's append, WHEN the loop retries, THEN it re-derives the query, re-reads, folds a **fresh clone of the pristine boundary** and calls `decide` again with that fresh state — it never reuses the previous fold, never re-submits the previous event batch, and never mutates the boundary value P1 passed in (AC-U13, `_decomposition.md:179-185`; `_design.md`, *Shape decision*, `DecisionModel` supertrait row)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:274 — `let mut model = boundary.clone();` at the
    top of every iteration, so the query is re-derived (:275), the store re-read (:277)
    and the fold rebuilt (:282) from the pristine value; the caller's `boundary` is
    never mutated and no batch survives an iteration. `cargo test -p happenstance
    --all-features`: command_loop.rs::retry_refolds_from_pristine_state ... ok — the
    closure records the folded `taken` it saw and the test asserts `[0, 1]`, so
    attempt 2 saw the interloper and attempt 1 did not; and
    ::retry_does_not_resubmit_the_previous_batch ... ok — the events reaching `append`
    on attempt 2 are byte-equal to attempt 2's own decision, not attempt 1's.

- id: AC-005
  criterion: "**P1's command works against a remote store, not only an in-process one.** GIVEN a store that reports `ConditionViolated` with `conflicting_position: None` — which a one-shot-HTTP adapter legitimately does (`crates/happenstance-core/src/error.rs:139-147`) — WHEN the loop decides whether to retry, THEN it decides through `AppendError::is_condition_violated()` (`error.rs:253`) and nothing else, retries normally, and still surfaces the hint it declined to branch on by carrying the `ConditionViolated` value into whatever error it eventually returns rather than discarding it"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:305 — the retry decision is
    `err.is_condition_violated()` and nothing else; `conflicting_position` appears
    nowhere in the crate's source except in the doc paragraph that says it is not
    consulted. The violation is carried into `Exhausted.source` at :311 through
    `violation()` (:351). `cargo test -p happenstance --all-features`:
    command_loop.rs::retries_when_conflicting_position_is_none ... ok (the wrapper only
    ever reports `None`, and the loop still retries and commits on attempt 2);
    ::branching_on_some_is_rejected ... ok (the wrong predicate is compiled beside it
    and answers "do not retry" for the same value); ::exhausted_carries_the_violation
    ... ok (the hint survives into the returned error and is reachable through
    `core::error::Error::source`).

- id: AC-006
  criterion: "**P1 can see the bound before they hit it, and running out is a distinct answer.** GIVEN P1 wrote `Retry::attempts(2.try_into()?)` at the call site — a required argument with no `Default` and no hidden 3 — WHEN the store violates on every attempt, THEN exactly 2 appends are attempted and the call returns `CommandError::Exhausted { attempts: 2, source }`, distinguishable from a store failure and from a refusal; `Retry::once()` attempts exactly 1 (AC-U13; `_design.md`, *Shape decision*, Retry-bound row)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:41-67 — `Retry` is a `#[non_exhaustive]` struct
    over a `NonZeroU32` with two constructors and no `Default`; `pub use`d at
    crates/happenstance/src/lib.rs:157, and a required positional argument of both
    doors. `cargo test -p happenstance --all-features`:
    command_loop.rs::exhaustion_is_bounded_and_named ... ok — `attempts(2)` submits
    exactly 2 appends and returns `Exhausted { attempts: 2, .. }`, `once()` submits
    exactly 1, and the wrapper's own append count is compared against what the error
    reports; command.rs::tests::retry_has_no_default ... ok — an inherent-vs-trait
    probe with a positive control, which is the only way to assert an impl's absence;
    command.rs::tests::once_is_one_attempt_and_attempts_is_what_it_says ... ok. The
    `attempts(0)` spelling is a compile error, paired with a compiling fence at
    command.rs:27-40 (`Retry (line 38) - compile fail ... ok`).

- id: AC-007
  criterion: "**P1 matches on the vocabulary they already learned, and every error carries the value.** GIVEN any failure a caller can act on, WHEN P1 inspects it, THEN it is a `#[non_exhaustive]` `CommandError<E, D>` variant whose payload is a typed `#[source]` — the store's own `E`, P1's own `D`, a `CodecError`, an `InvalidQuery` — with no `String` payload and no `happenstance`-local re-spelling of `ConditionViolated`, and the chain is reachable through `core::error::Error::source` (AC-U05, AC-U08; `standards/rust/30-error-taxonomy.md` RS-30-2, RS-30-4)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs:96-165 — `CommandError<E, D>` is
    `#[non_exhaustive]` with seven variants and six `#[source]` payloads plus one
    `#[from]`; every payload is a type (`E`, `D`, `AppendError<E>`, `CodecError`,
    `ConditionViolated`, `InvalidQuery`) and none is a string. `pub use`d at
    crates/happenstance/src/lib.rs:157. `cargo test -p happenstance --all-features`:
    command.rs::tests::store_read_failure_chains_to_source ... ok,
    ::decode_failure_names_its_position ... ok (the message renders `12`),
    ::encode_failure_names_its_event_type ... ok (the message renders `SeatTaken`) —
    each downcasting the walked `source()` to the concrete inner type; and
    docs_composition.rs::no_string_payloads_in_command_error ... ok.

- id: AC-008
  criterion: "**P3's edge build survives P1's command loop.** GIVEN a Workers or `wasm32` caller on the `!Send` flavour and a multi-threaded caller on the `Send` flavour, WHEN either compiles `commit`/`commit_with`, THEN both work: this crate's generic code binds `EventStore` (the weaker requirement that accepts both), imports exactly one flavour name per module, uses no `#[async_trait]`, and — because `S::Error` carries no `Send` bound (ADR-0009) — the loop collapses each `AppendError` to a retry decision **before** the next read's await, so the whole future stays `Send` inside a real `tokio::spawn` (`CLAUDE.md` constraints 1 and 4; `crates/happenstance-core/src/memory.rs:643-700`)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/command.rs binds `S: EventStore` at :218 and :257 — the
    weaker requirement — imports one flavour name (`EventStore`, :10) and uses no
    `#[async_trait]`; :305-315 collapses the `AppendError` to a control decision in one
    statement and drops it before the loop's next await. `cargo test -p happenstance
    --all-features`: flavours.rs::commit_spawns_from_generic ... ok — a generic
    `S: SendEventStore + Send + Sync + 'static` awaiting the loop inside a real
    `tokio::spawn` on a multi-thread runtime, **with no `S::Error: Send` bound**, which
    is the assertion; ::commit_binds_the_weak_flavour ... ok — the same call against a
    hand-written `impl EventStore for LocalStore` (an `Rc` field) with no `error[E0119]`;
    ::the_weak_flavour_store_is_genuinely_not_send ... ok with a positive control. The
    instrument was proved non-vacuous by mutation: binding the error across the next
    await produced `error[E0277]: `<S as SendEventStore>::Error` cannot be sent between
    threads safely … required by a bound in `tokio::spawn``, and the mutation was
    reverted. `cargo xtask affected --base main` green (clippy `-D warnings`).

- id: AC-009
  criterion: "**P4 lands on the crate root and meets the loop as a working thing, not a plan.** GIVEN P4's one bounded, one-shot sitting (`_decomposition.md`, UX brief persona table), WHEN they open `docs.rs/happenstance`, THEN region 2's first program calls `commit` and compiles; region 4's `**The command loop**` bullet is an intra-doc link to that function **in place**, out from under the word *Planned* (`crates/happenstance/src/lib.rs:35,47-48`); `commit`, `commit_with`, `Retry`, `Committed` and `CommandError` are reachable from the crate root beside the surviving `pub use happenstance_core::*;` (`:75`) and none of them shadows a core name; and `commit`'s **own** item page carries the retry policy, the verbatim-resubmission distinction and the rejected alternative as persistent prose — a doc comment that says *see the specification* fails this (AC-U10, AC-U14, AC-U15; `_design.md`, *Composition* regions 2 and 4, *Transience policy* rows for the retry policy and for `Boundary`/`Committed`/`CommandError`; anti-patterns 1, 2, 14)"
  satisfied: true
  evidence: >-
    crates/happenstance/src/lib.rs:95-98 — the bullet is rewritten in place, still the
    fourth entry of region 4, now `[**The command loop**][command-loop]` with the
    reference resolved at :126-127; the rendered page carries
    `<li><a href="fn.commit.html" title="fn happenstance::commit"><strong>The command
    loop</strong></a>` (target/doc/happenstance/index.html), so the link *is* the
    emphasis. Region 2's first program calls `commit` and compiles (`lib.rs - (line
    154) ... ok`). All five items are `pub use`d at :154-157 beside the surviving glob
    at :160. `commit`'s own page carries the policy at command.rs:167-210. `cargo test
    -p happenstance --all-features`: docs_composition.rs::roadmap_bullet_became_a_link,
    ::no_planned_heading_survives, ::no_item_shadows_a_core_name,
    ::retry_policy_is_on_commit_itself (the three sentences present, no "see the
    specification" deferral, and "The alternative that lost" named once),
    ::landing_page_names_only_the_persistent_four — all ok. `cargo doc -p happenstance
    --no-deps` clean with `rustdoc::broken_intra_doc_links` denied.
  verifying_test: "crates/happenstance/tests/docs_composition.rs::roadmap_bullet_became_a_link, ::no_planned_heading_survives, ::no_item_shadows_a_core_name, ::retry_policy_is_on_commit_itself, ::landing_page_names_only_the_persistent_four, plus cargo test -p happenstance --doc and cargo doc -p happenstance --no-deps with rustdoc::broken_intra_doc_links denied"

- id: AC-010
  criterion: "**P4 can read the page without scrolling sideways and can tell what a feature turns on.** GIVEN the same one sitting, at 1024x768, WHEN P4 reads the rendered page, THEN the composed presentation holds its budget: doc prose at most 80 columns, code inside a doc fence at most 72 columns, the first sentence of every item this story adds at most 80 characters and a complete claim, every added identifier at most 24 characters, and the crate-root module doc at most 130 lines (`_design.md`, *Density budget*); AND the feature story is legible without opening `Cargo.toml` — `commit` is the JSON convenience gated `#[cfg(feature = \"json\")]` and renders a `doc_cfg` gate badge on docs.rs, `commit_with` is ungated and reachable with the gate off, and no intra-doc link on the page resolves in only some feature configurations (AC-U14; RS-70-2, RS-70-4, RS-51-5; anti-patterns 3, 5, 6)"
  satisfied: true
  evidence: >-
    The budget holds against the tests M2 left standing plus this story's own:
    doc_budget.rs's seven (80-column prose, 72-column fences, ≤130 module-doc lines,
    first sentences, the glob, the fence position) and
    src/tests.rs::the_crate_root_page_fits_above_the_fold (the first program is 35
    visible lines with exactly 2 hidden, both harness — the `#[tokio::main]` opener and
    the `Ok::<…>` closer), plus docs_composition.rs::density_budget_holds ... ok over
    `command.rs`. `commit` is gated and badged at command.rs:211-212, `commit_with` is
    ungated at :257; the rendered badge is in target/doc/happenstance/fn.commit.html
    (`stab portability">Available on <strong>crate feature <code>json`).
    docs_composition.rs::docsrs_metadata_is_present ... ok. No link resolves in only
    some configurations: the vocabulary reference is conditional (lib.rs:126-127) and
    `commit_with`'s page names `commit` in plain text, so `cargo doc -p happenstance
    --no-deps` is clean under default features **and** `--no-default-features`, and
    `cargo check -p happenstance --no-default-features` and `--features std` both
    compile with `commit` gone and `commit_with` standing.
  verifying_test: "crates/happenstance/tests/docs_composition.rs::density_budget_holds and ::docsrs_metadata_is_present, plus cargo check -p happenstance --no-default-features (and --features std) and cargo doc -p happenstance --no-deps in both feature states"
```
