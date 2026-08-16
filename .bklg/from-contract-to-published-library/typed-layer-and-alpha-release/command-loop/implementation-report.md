---
item: "HS-S0023"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The command loop: read, decide, append, retry on ConditionViolated

**All ten ACs are satisfied.** `commit`, `commit_with`, `Retry`, `Committed` and
`CommandError` land in `crates/happenstance/src/command.rs`, are `pub use`d at the crate
root beside the surviving glob, and the module doc's `**The command loop**` bullet is now
an intra-doc link to `commit` — rendered, in the generated HTML, as
`<a href="fn.commit.html"><strong>The command loop</strong></a>`.

The half-implementation this absorbs — `examples/course-subscriptions/src/main.rs:207`'s
private `commit`, which bails where a retry loop should go — is untouched here. Deleting
it *into* this function is M6's, and it can now start.

## TDD Evidence

Written in the order the spec's risk table demands: `flavours.rs` first, because the
`Send` trap compiles until it does not and every single-threaded test passes either way.

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-009 | `docs_composition.rs::roadmap_bullet_became_a_link` | `panicked … the command-loop bullet is not a link: * **The command loop** *(planned)* — read, decide, append, retry on` | ok |
| AC-009 | `docs_composition.rs::no_planned_heading_survives` | `panicked … the command loop is still advertised as planned` | ok |
| AC-009 | `docs_composition.rs::retry_policy_is_on_commit_itself` | `panicked … the crate's own source is readable: … NotFound` (there was no `command.rs`), then `panicked … \`commit\`'s own page does not state \`re-read\`` | ok |
| AC-003, AC-006, AC-007, AC-010 | `docs_composition.rs::committed_is_non_exhaustive_and_must_use`, `::no_string_payloads_in_command_error`, `::retry_has_no_default`, `::density_budget_holds`, `::docsrs_metadata_is_present` | all five failed on the missing file | ok |
| AC-001, AC-002, AC-004, AC-005, AC-006 | `command_loop.rs` (9 tests) | unresolved imports `happenstance::{commit, commit_with, Retry, Committed, CommandError}` — the items this story exists to add | ok |
| AC-008 | `flavours.rs` (3 tests) | same unresolved imports | ok |

**The `Send` assertion was proved non-vacuous by mutation, not by argument.** With the
loop written as the spec's named wrong implementation — bind the `AppendError`, await,
*then* inspect it — `cargo test -p happenstance --test flavours` fails to compile with:

```
error[E0277]: `<S as SendEventStore>::Error` cannot be sent between threads safely
note: required because it appears within the type `AppendError<<S as SendEventStore>::Error>`
note: required because it's used within this `async` fn body
note: required by a bound in `tokio::spawn`
```

The mutation was reverted. `commit_spawns_from_generic` carries **no** `S::Error: Send`
bound, so its bounds are the assertion rather than an escape hatch — the spec's own
sketch allowed that bound, and it turned out not to be needed once the error is collapsed
in one statement.

**AC-004's pristine-clone half was proved non-vacuous the same way, and it was vacuous
first.** Driven from an empty store, attempt 1 folds nothing and the closure records `0`
whether or not the loop re-clones — the sound loop and the hoisted-clone loop both record
`[0, 1]`, so the assertion rejected only *"the loop never re-reads"*. `Contended::seed`
now lands one subscription before the call, so attempt 1 folds `1` and a pristine re-fold
records `[1, 2]` where a stale one double-counts to `[1, 3]`. With
`let mut model = boundary.clone();` hoisted out of the loop:

```
---- retry_refolds_from_pristine_state stdout ----
assertion `left == right` failed: attempt 2 folded a store that had moved onto
a model that had not been rebuilt: a hoisted clone records [1, 3] here
  left: [1, 3]
 right: [1, 2]
test result: FAILED. 9 passed; 1 failed
```

Nine of ten tests in the file passed under that mutation, which is the measurement that
matters: this test is the only thing standing between the hoisted clone and a green gate.
The mutation was reverted.

## Commits

* `feat(typed-layer-and-alpha-release): Command loop` — the SHA is recorded in the slice
  digest; this story's whole diff plus these two reports.
* Slice-mate, landed first in the same context:
  `feat(typed-layer-and-alpha-release): Codec and feature forwarding` (`da530cd`).

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance/src/command.rs` | **New, 596 lines.** `Retry` (`NonZeroU32`, no `Default`), `Committed` (`#[must_use]` + `#[non_exhaustive]`), `CommandError<E, D>` (seven variants, six typed `#[source]`s, one `#[from]`), `commit_with` (the whole policy), `commit` (the `json`-gated JSON door), the private `encode` and `violation` helpers, and a `#[cfg(test)] mod tests` with six unit tests over a module-local one-variant domain (`Turnstile`/`Gate`) and a module-local `Wire` codec, so the unit tier drives the real loop against a bare `MemoryEventStore` rather than asserting over hand-built values. |
| `crates/happenstance/src/codec.rs` | One line: the `#[allow(dead_code)]` on `frame` is gone, because the command loop is now its caller. |
| `crates/happenstance/src/lib.rs` | The mount. `mod command;`, the five `pub use`s beside the glob, region 2's first program rewritten to call `commit`, region 4's bullet rewritten in place as a reference-style link, and the two conditional `doc = "[command-loop]: …"` definitions that make it resolve in every feature state. |
| `crates/happenstance/Cargo.toml` | `[dev-dependencies]` only: `tokio` gains `rt-multi-thread` (the spawn test needs a real work-stealing runtime) and `futures-core` arrives because naming `Stream` is unavoidable in a hand-written `impl EventStore`. The `[features]` block is the slice-mate's and was not touched by this story. |
| `crates/happenstance/tests/command_loop.rs` | New. The recording/violating wrapper (three knobs: violate next *n*, record every condition and batch, land an interloper between read and append), a fourth affordance `seed` that puts a *previous* command's event in place before the loop runs — bypassing the watch, because it is nobody's attempt — and ten tests. |
| `crates/happenstance/tests/flavours.rs` | New. The spawn test, a hand-written `!Send` `impl EventStore`, and a Send probe with a positive control. |
| `crates/happenstance/tests/docs_composition.rs` | New. Nine source-reading assertions over the rendered surface. |

## Gates

```
cargo test -p happenstance                          # default features
cargo test -p happenstance --all-features           # 26 lib + 9 + 3 + 9 + … + 9 doctests
cargo check -p happenstance --no-default-features
cargo check -p happenstance --no-default-features --features std
cargo doc -p happenstance --no-deps                 # and again --no-default-features
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc -p happenstance --no-deps --all-features
cargo hack check --feature-powerset -p happenstance
cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json
cargo fmt --all --check
cargo xtask affected --base main                    # the wired story gate
cargo xtask ci --fast                               # the project's declared integration bar
```

All green. `cargo xtask affected --base main` is the one the spec names as the merge gate;
it runs `cargo fmt --all --check`, `clippy --all-targets --all-features -D warnings` and
`cargo test --all-features` over `happenstance`, `happenstance-testkit`,
`course-subscriptions` and `xtask`, plus the five file-reading lints and `spec-trace`.

## Notes

* **The vocabulary bullet is a *reference-style* link, and that is the RS-70-2 fix.**
  `[**The command loop**](commit)` is a **hard rustdoc error** in every build without
  `json`, not a warning — `cargo doc --no-default-features` proved it on the first try.
  The bullet stays one `//!` line and only the link *definition* is conditional
  (`lib.rs:126-127`), so the page's composition is unchanged in both feature states and
  the rendered HTML resolves to `fn.commit.html` under the defaults.
* **`commit_with`'s page states the policy rather than pointing at `commit`'s.** The
  obvious spelling — "see [`commit`]" — is a gated link with the same failure. Since
  `commit_with` is the door a `--no-default-features` reader has, it carries the retry
  policy, the bound and the verbatim-resubmission distinction in full; `commit`'s page
  additionally names the alternative that lost (RS-70-5, once, where the reader is).
* **`B: Boundary + Clone`, which the design's signature block does not spell.** The
  design's own *Shape decision* requires the loop to "re-fold from the pristine model on
  retry" and gives `DecisionModel` a `Clone` supertrait for exactly that — but `Boundary`
  is sealed and carries no `Clone`, so the bound has to be written at the entry points.
  It costs a caller nothing (`DecisionModel: Clone` already, and tuples of `Clone` are
  `Clone`) and is the only way AC-004 is satisfiable without editing M2's sealed trait.
* **The first program was rebuilt to 35 visible lines with 2 hidden.** Adding a real
  `commit` call and an async runtime to a program that was already at 34 lines meant the
  *domain* yielded, never the ceremony: the scope is now `Tags::empty()` and the `use`
  block is three lines instead of four. Both hidden lines are harness and both are what
  `src/tests.rs::the_crate_root_page_fits_above_the_fold` already permits — the
  `#[tokio::main]` opener and the `Ok::<…>` closer.
* **The unreachable `Err` on `Event::new` is absorbed, not `unwrap`ed.** `event_type()`
  hands back an already-validated `EventType` and the conversion is the infallible
  identity, but the constructor is still `Result`. It maps into
  `CommandError::Boundary` through `InvalidQuery::from` — the same treatment
  `Boundary::query`'s own unreachable arm gets, and the same residual as **defect
  candidate D-1**, which is inherited unchanged and still routed to AC-012's log. No new
  defect was found and nothing under `crates/happenstance-core/src/**` moved.
* **The crate root's first program is deliberately the JSON-default one, and the source
  now says so.** It calls `commit`, which is `#[cfg(feature = "json")]`, so the fence
  compiles only with the defaults on. No gate step catches that today and none needs to:
  the two feature-state checks the spec names are `cargo check` and `cargo hack check`,
  neither of which builds a doctest, and docs.rs builds `all-features`. The choice is
  recorded in a comment beside the module doc (`crates/happenstance/src/lib.rs:11-18`)
  rather than left for the next reader to mistake for an oversight, and it names the fix
  if a gate step ever does compile doctests with `json` off: a second fence behind a
  `#[cfg(not(feature = "json"))]` doc line calling `commit_with`, the way the
  `[command-loop]` reference at `:134-135` already is. The comment sits *above* the `//!`
  block on purpose — `doc_budget.rs::first_sentences_fit_the_item_table` resets its
  sentence accumulator on any non-doc line, so a `//` comment inserted mid-block would
  make it start accumulating the doctest's own body.
* **One edit in this story's commit belonged in the slice-mate's.**
  `standards/rust/90-skeletons-and-todo.md:269` is a one-line citation repair
  (`./Cargo.toml:125` → `:136`) forced by the codec commit's workspace-manifest edit;
  without it `cargo xtask lint-constitution` goes red. It is kept — reverting it would
  break the gate it exists to satisfy — and recorded here as a commit-boundary slip
  rather than substantive drift. The repository has precedent for such repairs
  (`b77cffb`).
* **No conformance rule was added.** This is a caller-side composition of unchanged port
  methods; `happenstance-testkit`'s suite observes adapters, not consumers, and it passes
  untouched.
* **The test wrapper stays private to this target.** `happenstance-testkit` is not in
  `happenstance`'s non-dev graph, early or otherwise; M4's `FaultyStore<S>` replaces this
  wrapper when it lands.
