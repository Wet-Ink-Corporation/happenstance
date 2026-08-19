---
item: "HS-S0020"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — DomainEvent and DecisionModel, mounted at the crate root

## Findings Ledger

**Outcome: eight of eight ACs satisfied, one with a cited exception inside it.**
The exception is a *budget number*, not a behaviour, and it is the same number the
signed-off design's own mock already failed by more than twice as much.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| The vocabulary exists and is mounted at the crate's only render path | `crates/happenstance/src/lib.rs:120-124` — `Boundary`, `Codec`, `CodecError`, `DecisionModel`, `DomainEvent` `pub use`d beside the surviving `pub use happenstance_core::*;`; all four modules private at `:111-114` | none |
| A hand-maintained query has nowhere to live | `boundary.rs:45` (`Boundary: crate::sealed::Sealed`), `sealed.rs:17-19` (`pub` in a private module), blanket impl at `boundary.rs:88`. Doctest `boundary_cannot_be_implemented_outside_the_crate` yields `error[E0277] … Sealed is not satisfied` with rustc's own "sealed trait" note | the slice-mate adds tuple impls to the same seal; coherence probed and clean |
| An empty `EVENT_TYPES` is a compile error, with the diagnostic the AC asked for | `error[E0080]: evaluation panicked: a DomainEvent must declare at least one event type in EVENT_TYPES`, `note: … while instantiating fn <Empty as Boundary>::query`. Const item at `boundary.rs:116` (RS-61-4) | none |
| `absorb` skips by nomination, not by a failed decode | `boundary.rs:97` consults `nominates` (`:137`) before any codec is reached; `tests.rs::absorb_skips_an_unnominated_event` is the test that failed against the routing-by-arrival implementation | the tuple impls must route per member, not per composite |
| `_design.md`'s `CodecError` signature does not compile | thiserror: `error: transparent variant can't contain #[source]`. Fixed by a condition message plus the explicit typed `#[source]`; `codec.rs:75-84` documents why | worth a one-line correction to `_design.md` at project closeout; the *intent* is unchanged |
| `Boundary` needs `type Event`, which `## Signatures` omits | required by `commit`'s `Vec<B::Event>` (`_design.md:514-524`) and by the slice-mate's `B2: Boundary<Event = B1::Event>`; landed now because RS-40-1 forbids growing a sealed trait later | same closeout correction |
| **Defect D-2** — the `<= 35`-visible-line fence budget is unreachable under the signed-off `DomainEvent` | mapping ceremony is 22 lines before any domain exists; the smallest honest two-variant program is 44. `_design.md` `## Mock`, finding 1 already measured the designed program at 81 lines / 107 columns and the approver accepted it | AC-012's defect log and AC-013's ceremony verdict. **Not** to be closed by adding a derive macro here — that would destroy the measurement |
| `crates/happenstance/README.md:6` now contradicts the crate root | it still says *"this crate is currently a facade"*; the file is outside this PR's allowed globs | the `crate-readme` surface, M5–M7 |
| The residual `EVENT_TYPES` / `event_type()` agreement is tested in both directions | `tests.rs::every_variant_event_type_is_declared`; the fixture declares `TicketAudited`, which no variant returns and `decode` refuses loudly | `testing::assert_domain_event` generalises it in M4 |

## Acceptance

| AC | Status | Verified by |
| -- | ------ | ----------- |
| AC-001 | satisfied | `fold_is_exhaustive_over_domain_enum`, `scope_returns_the_models_own_tags`, `decision_model_is_clone_not_default`; signatures reviewed against `_design.md` `## Signatures` |
| AC-002 | satisfied | `query_is_derived_from_event_types_and_scope` (oracle built from the model's own declaration, never from literal positions), `query_is_a_value_a_test_can_assert_on`, `nomination_agrees_with_the_derived_query`, and the sealed-trait `compile_fail` paired with its compiling companion |
| AC-003 | satisfied | the bare `compile_fail` fence and its measured `E0080` const-eval diagnostic; `unconstrained_boundary_is_an_error_not_query_all`; `clippy -D warnings` under `unwrap_used = "deny"`; no diff under `crates/happenstance-core/` |
| AC-004 | satisfied | `absorb_skips_an_unnominated_event`, `absorb_returns_unknown_event_type_when_a_nominated_event_cannot_be_decoded`, `absorb_applies_a_decoded_event_to_the_fold`, `codec_error_carries_a_typed_source` |
| AC-005 | satisfied | root re-exports beside the surviving glob; `contract_names_are_not_shadowed`; both `cargo doc` configurations warning-free; docs.rs metadata and `doc_cfg` present |
| AC-006 | satisfied | the two bullets rewritten in place as intra-doc links, three honest promises kept, the stale "facade / adds nothing" region deleted; `# Errors` by condition on every fallible item; the rejected alternative named once on each of the four unusual constructs |
| AC-007 | satisfied, **with one budget carried as defect D-2** | five of six budgets asserted and green (`doc_density_budget_holds`, `public_identifiers_fit_the_item_table`, `the_crate_root_page_fits_above_the_fold`); the sixth — `<= 35` visible fence lines — is 44, recorded in the source and in the ledger, with the 72-column and hidden-line budgets *not* conceded |
| AC-008 | satisfied | `nothing_in_the_vocabulary_touches_a_store`, `every_variant_event_type_is_declared`; no `async fn`, no `EventStore`/`SendEventStore` bound, no new crate, no derive macro |

**Deferred, and named rather than faked:** nothing in this story. The tuple impls,
`commit`, the concrete codecs and `happenstance::testing` belong to named siblings
and are absent by boundary, not by omission.

## Knowledge Harvest

Candidates for `.kb/` at project closeout. None is authored here — atoms are the
`/redkiln:kb-ingest` wave's.

* **A sealed trait's supertrait is `pub` inside a private module, not `pub(crate)`.**
  `pub(crate)` makes the public trait "more private than" its own bound and trips
  `private_bounds`; the `pub`-in-a-private-module spelling trips `unreachable_pub`
  instead, which is a scoped `allow` carrying a reason. This workspace had no
  sealed trait before today, so `standards/rust/13-sealing-and-exhaustiveness.md`
  has no rule for it.
* **A blanket impl over a *local* trait does not conflict with tuple impls.**
  `impl<M: DecisionModel> Boundary for M` coexists with
  `impl<B1: Boundary, B2: Boundary<Event = B1::Event>> Boundary for (B1, B2)`,
  because the orphan rule forbids any downstream `impl DecisionModel for (A, B)`.
  Probed before a line of it was written; the opposite reading — a guaranteed
  `E0119` — would have forced a different composition shape on the slice-mate.
* **`#[error(transparent)]` is not a way to carry a typed `#[source]`.** It refuses
  the attribute outright, and on its own it forwards the *inner* error's source, so
  a boxed error under it disappears from the chain a caller reports from. RS-30-2
  needs that sentence attached to it.
* **The mapping-ceremony ratio, measured on real code rather than a design sketch:**
  22 lines of `DomainEvent` ceremony before a domain exists, and 44 lines for the
  smallest honest two-variant vocabulary program. This is AC-013's input, and the
  discomfort is recorded rather than acted on.
