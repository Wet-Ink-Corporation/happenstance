---
item: "HS-S0025"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — A given/when/then DSL that cannot hide its own filter

**All ten ACs are satisfied. Nothing is blocked and nothing is deferred.**

`happenstance::testing` is a real, named module declared at
`crates/happenstance/src/lib.rs:187`, carrying `given`, `Given<B>`, `Decision<E>` and
`assert_domain_event`. The crate-root page gains one region — *Testing without a database* —
between Features and the adapter pointer, exactly where the design's composition table puts
it. The four-region failure message is the deliverable, and its fourth region,
`seeded but NOT selected:`, is asserted present **and** correct against an oracle spelled
independently of the DSL's own set difference.

The retry demonstration drives `commit_with` against the slice-mate's `FaultyStore`, which
refuses an append reporting `conflicting_position: None` — so a caller's retry loop is
testable without a database, and the loop that branches on `Some` is testable *at all* for
the first time in this workspace.

## TDD Evidence

The same two-stage red the slice's first story used, and the second stage is the one that
matters. An **inert** renderer was landed first — the two filter regions omitted, which is
precisely the design's anti-pattern 12, *"a DSL assertion failure shows only
expected/actual"* — so every assertion about regions three and four failed on the string it
was written to assert rather than on a missing symbol.

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-001 | `src/testing/tests.rs::seeded_then_decided_asserts_emitted_events`, `::composed_boundary_folds_both_models` | the module did not exist; `happenstance::testing` was an unresolved path | PASS; the composed case asserts each tuple member folded 1, not 2 |
| AC-002 | `tests/dsl_failure_message.rs::four_regions_render_in_order`, `::panic_location_is_the_callers_line` | `render::tests::the_four_regions_are_in_order` FAILED at `render.rs:232` — *"`selected by the model's query:` is missing"* | PASS; three byte-offset orderings and the header, plus file + exact caller line |
| AC-003 | `::region_four_names_the_seeded_but_unselected_event` | region four did not exist, so the `Withdrawn` the model's `EVENT_TYPES` omits was nowhere on the screen | PASS; present in region four, absent from region three |
| AC-004 | `::empty_seed_keeps_region_four_and_says_so`, `::empty_selection_is_not_an_error` | `render::tests::the_empty_state_keeps_its_region` FAILED at `render.rs:245` | PASS; `(nothing was seeded)` and the region that carries it |
| AC-005 | `::overflow_truncates_selected_first_and_the_diagnosis_last`, `::long_event_type_wraps_without_truncating_type_or_position` | `render::tests::selected_truncates_first_and_the_diagnosis_last` FAILED at `render.rs:263`; `::a_long_event_type_wraps_rather_than_truncating` FAILED at `:292` | PASS; `… and 12 more` in region three, region four complete, every line ≤ 80 columns |
| AC-006 | `src/testing/tests.rs::refusal_is_asserted_through_then_refused`, `::refusal_does_not_route_through_when_err`, `tests/dsl_failure_message.rs::then_on_a_refused_decision_panics_naming_the_refusal` | the module did not exist | PASS; the boxed refusal downcasts back to the caller's own type |
| AC-007 | `::seeded_bytes_are_the_bytes_commit_writes`, `::nominated_but_undecodable_event_is_an_error`, `::non_nominated_type_is_skipped_not_an_error`, `::assert_domain_event_rejects_a_variant_outside_event_types` | as above | PASS; compared against `Json.encode` and `frame::<Json>(None)`, never a literal |
| AC-008 | `::dropping_a_decision_appends_nothing`, `::must_use_message_is_verbatim` | as above | PASS; the attribute's own literal is read back out of `mod.rs` and compared |
| AC-009 | `tests/retry_without_a_database.rs::retry_succeeds_after_an_injected_violation`, `::retry_refolds_from_a_pristine_model`, `::retry_is_bounded` | `retry_refolds_from_a_pristine_model` FAILED: `the seeding commit: Exhausted { attempts: 1, source: ConditionViolated { conflicting_position: None } }` — a real defect in the *test*, which armed the store before seeding it | PASS; seeded first, armed after, `attempts == 2` and the closure ran twice over a fold of 1 |
| AC-010 | `tests/mounted_at_the_crate_root.rs` (three cases) | the module was not declared, so the public path did not resolve | PASS; plus `cargo doc` clean with default features **and** `--no-default-features` |

The red transcript:

```
test result: FAILED. 37 passed; 5 failed   (happenstance --lib, inert renderer)
```

and after the two regions were restored, every target green. No test was weakened, skipped
or deleted between the two.

## Commits

One checkpoint commit, carrying the whole story:

```
feat(typed-layer-and-alpha-release): Given/when/then DSL
Story: typed-layer-and-alpha-release/given-when-then-dsl
```

Its short SHA is reported in this run's slice digest and is recoverable here with
`git log --grep "Story: typed-layer-and-alpha-release/given-when-then-dsl" --oneline`
— a commit cannot cite its own hash, and this report is inside it.

Two small commits landed **before** it, both carrying the slice-mate's trailer because both
repair files that story owns: a redundant intra-doc link target in
`crates/happenstance-testkit/src/faulty.rs` that only `--document-private-items` sees, and
eight constitution citations in `standards/rust/` whose anchors moved 41 lines when the
testkit's crate-root doc gained a region. Both were found by `cargo xtask ci --fast`, and
keeping them out of this story's commit is what keeps its PR boundary honest.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance/src/testing/mod.rs` | **new.** `given`, `Given<B>`, `Decision<E>`, `assert_domain_event`, the private `Outcome<E>` that carries a refusal as a typed `Box<dyn Error>`, and the `seeded \ selected` difference that region four reports |
| `crates/happenstance/src/testing/render.rs` | **new.** The four-region message, in its own function so it can be asserted at the string level; the 8-row / 80-column budget, the asymmetric truncation, and a wrapper that breaks a line rather than a value |
| `crates/happenstance/src/testing/tests.rs` | **new.** The nine unit cases that assert on things a test author cannot see: the store `given` built, the typed refusal, and the `#[must_use]` attribute's own source text |
| `crates/happenstance/src/lib.rs` | **mount.** `pub mod testing;` behind `all(feature = "memory", feature = "json")` with its `doc_cfg`, the `# Testing without a database` region, and the conditional `[testing-module]` link definition |
| `crates/happenstance/Cargo.toml` | a **path-only, versionless** dev-dependency on `happenstance-testkit` |
| `crates/happenstance/tests/dsl_failure_message.rs` | **new.** The divergent model, the long event type, the drifted variant, and one `caught(fn())` helper that serialises every panicking case behind a lock |
| `crates/happenstance/tests/retry_without_a_database.rs` | **new.** The retry demonstration, the pristine re-fold, the bounded exhaustion, and EC-005 |
| `crates/happenstance/tests/mounted_at_the_crate_root.rs` | **new.** The public path, the doc-fence column budget, and the region's position on the landing page |
| `Cargo.lock` | one line: `happenstance` gains a dev-dependency edge on `happenstance-testkit`. No third-party crate enters the graph |

## Gates

| Gate | Command | Result |
| --- | --- | --- |
| Story gate | `cargo xtask affected --base main` | **affected gate passed** |
| Project gate | `cargo xtask ci --fast` | **all required checks passed (4 optional steps not run)**, including all four `wasm32` steps, both documentation builds and the packaging assertions |
| Formatter | `cargo fmt --all --check` | clean |
| Lint | `cargo clippy -p happenstance --all-targets --all-features -- -D warnings` | clean |
| Tests | `cargo test -p happenstance` | every target green |
| Doctests | `cargo test -p happenstance --doc` | 8 passed, 3 compile-fail passed |
| Feature | `cargo build -p happenstance --no-default-features --features std` | clean — the module and its three test files drop together |
| Docs | `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps`, and the same `--no-default-features` | both clean |

## Notes

Five deviations and residuals, each with its reason.

**DG-1 — the module is gated on two features, not one.** `_design.md`'s item table says
`feature: memory`, and that is incomplete rather than wrong: `Given::event` seeds through the
codec `commit` writes with, which is `Json`, which is `json`'s. A `memory`-only gate would
leave `--features std,memory` (no `json`) referring to a type that does not exist. Both
features are in `default`, so the item a `cargo add happenstance` user meets is exactly the
item the design signed off; what the second gate buys is a feature powerset that compiles.

**DG-2 — `Given::event` buffers, and `when` performs the write.** The design's signature is
sync (`pub fn event<E: DomainEvent>(self, event: E) -> Result<Self, CodecError>`) and
`MemoryEventStore::append` is `async`, so the two cannot both be true unless the append is
deferred. It is, to `when` — the one `await` a test author writes. Nothing observable turns
on it: encoding is what can fail in `event`, and encoding is what its `Result` is for.

**DG-3 — EC-002's `CodecError::UnknownEventType` is unreachable through `given`, and the test
asserts the reachable disagreement instead.** The derived query is built **from**
`EVENT_TYPES`, so a nominated event whose type the domain type does not declare cannot
occur: `Boundary::absorb` filters by the same declaration it decodes against. What *is*
reachable — and what `nominated_but_undecodable_event_is_an_error` asserts — is the other
half of `absorb`'s documented contract: a nominated payload the codec cannot decode,
surfaced as `CommandError::Decode { position, source }` rather than swallowed into a shorter
log. Recorded here rather than worked around, because the AC's wording names a variant the
type system makes unreachable.

**NF-006 — the dev-dependency route, decided rather than discovered.** `crates/happenstance`
takes `happenstance-testkit = { path = "../happenstance-testkit" }`: **path-only and
versionless**, deliberately not `{ workspace = true }`, whose `version = "0.2.0"` would have
to resolve from the registry at publish time and would therefore force
`happenstance-testkit` to publish *before* `happenstance` — a coupling CF-32's independent
version number exists to avoid. Cargo omits a versionless dev-dependency from the published
manifest, so the two crates' release order stays free. This could not be *demonstrated*
here, because `cargo package -p happenstance` cannot resolve `happenstance-core = "^0.2.0"`
from crates.io while nothing is published; the gate's own `cargo package --list` step, which
does not resolve, is green. `publish-0-2-0-alpha-1` inherits a decision rather than a
surprise.

**Anti-pattern 12's reviewable equivalent.** The design phrases it as a screenshot check —
*"a screenshot missing `seeded but NOT selected:` fails"*. There is no screenshot to take of
a panic message, so the equivalent here is `region_four_names_the_seeded_but_unselected_event`,
which asserts the region is present *and* names the right event, against an oracle written
from the values the test seeded rather than from the DSL's own difference (RS-60-4).

Nothing in `command-loop`'s signatures had to change, and nothing under
`crates/happenstance-core/src/**` was touched.
