---
item: "HS-S0005"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — ProjectionProbe behind happenstance-core's conformance feature

## Findings Ledger

**Six of seven ACs satisfied. AC-007 is a split result, deliberately left
`satisfied: false`.**

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the five members the specification publishes, bare flavour, no `SendProjectionProbe` | **Met** | `probe_shape_matches_the_specification` coerces every member to an explicitly written signature through a generic `P: ProjectionProbe`, so a rename is `E0599` and a re-type is `E0308`. `the_probe_is_a_supertrait_of_the_bare_flavour` calls `ProjectionStore::begin` from a bound of `ProjectionProbe` alone. `rg -n "SendProjectionProbe" crates/` finds only prose forbidding one. **One spelling deviation, forced and recorded** — see below |
| **AC-002** — `conformance = []`, one flag and no new graph edge | **Met** | `crates/happenstance-core/Cargo.toml:60-79` is literally an empty list; `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps` compiles all 16 combinations, including `--no-default-features --features conformance` |
| **AC-003** — reachable *and* announced, at both mounts | **Met** | Export block at `lib.rs:132-134` with the `#[cfg]` + `#[cfg_attr(docsrs, doc(cfg(…)))]` pair copied from the `memory` items; feature-flags row at `lib.rs:83-92`. Proven by compilation: the test imports `happenstance_core::ProjectionProbe` through the crate root, and the RED transcript this story started from is exactly `no ProjectionProbe in the root` |
| **AC-004** — the seam is generic, and no member is dead on arrival | **Met** | `writes_are_visible_through_the_trait_alone` runs `begin` → `probe_write` → `commit` → `probe_read == Some(7)` with `TestStore` named only at the instantiation site. `all_five_members_are_reachable_generically` drives `probe_delete_all` through `reset` and `probe_read_through` under `READS_THROUGH_BATCH`. The instantiating store is defined in the test file and the word `memory` never appears in it |
| **AC-005** — independent of `memory`, `no_std`-clean, `wasm32`-clean | **Met** | 16 host combinations and 25 `wasm32` combinations, all green, plus `cargo xtask wasm`'s mandatory `--no-default-features` `wasm32` build. The named wrong implementation — a `conformance` surface that only compiles alongside `memory` — is rejected by combination 3 of 16 |
| **AC-006** — the rendered documentation, and no inbound link | **Met** | `missing_docs`/`missing_errors_doc` under `-D warnings`; `# Errors` on `probe_read` names the condition, not the type; `probe_read_through`'s doc carries the `unimplemented!()` spelling verbatim; the trait's own doc carries D1's orphan-rule argument and names its falsifier. `RUSTDOCFLAGS="-D warnings" cargo doc --no-default-features` green — the step where an inbound link is a hard error |
| **AC-007** — boundary held **and** `cargo xtask ci` green whole | **Split — not satisfied** | Boundary half **met** in full. Whole-gate half **not met**, for a cause inherited from the slice-mate and outside this story's control. See below |

### AC-007, stated precisely

**Met:** `spec/SPECIFICATION.md` is not edited; PS-11 and PS-12 remain
`[PROVISIONAL]`; no conformance rule, fixture, mutant, registry entry or
`MemoryProjectionStore` was added; `crates/happenstance-testkit/` is untouched;
no adapter skeleton moved; no ADR was written as a side effect. The commit
touches four source paths and this story's backlog folder, and nothing else.
EC-007's reshape trigger was not met — no projection rule that can observe the
read model without the probe was found.

**Not met:** `cargo xtask ci` is not green. The cause is entirely
`owned-batch-port-shape`, whose ADR-0017-mandated move of
`crates/happenstance-ladybug/src/live_handle.rs` and whose growth of
`projection.rs` invalidated 7 `file:line` citations in `spec/SPECIFICATION.md`
and 16 in `standards/rust/`, and broke 3 compiled constitution examples that
implement the port with the GAT. Both corpora sit outside both stories' PR
boundaries. Full inventory with the exact re-points:
`../owned-batch-port-shape/_reviewed-diff.md` §7.

The row is left `false` because the criterion says green **whole**, and it is
not. Reading it as "green apart from the parts that are not" is the ledger entry
this project exists to make impossible.

### The one deviation, and why it is not a liberty

`probe_read` is spelled `-> impl Future<Output = …>` where the specification's
fenced block writes `async fn`. It is not a preference: the gate rejects the
`async fn` spelling outright with `-D async-fn-in-trait`, because the lint fires
on any publicly reachable trait **not** under `#[trait_variant::make]`, and this
trait is deliberately not one. RS-22-1
(`standards/rust/22-rpitit-and-lifetime-capture.md:12`) requires the hand-written
desugaring in exactly this case and names the tempting wrong fix —
`#[allow(async_fn_in_trait)]`, *"which silences the one line where a reader could
have seen that the future has no `Send` bound and never will have one."* That
allow was not taken. Semantics are identical, and the desugaring puts the absence
of `+ Send` where a reader meets it.

**Nothing was stubbed, skipped or fixture-pinned.** The trait has no `todo!()`,
the test store has real bodies, and every one of the five members is driven by a
passing test rather than merely declared.

## Acceptance

**Recommended: accept the story on its own merits; AC-007's open half is a
slice-level decision, not rework here.**

The seam this story exists to add is real and reachable: an adapter author can
write `impl ProjectionProbe for TheirStore` today, against a dependency they
already have, with one feature flag and no new edge in their dependency graph.
Before this PR, generic code holding an adapter's batch could only commit it or
roll it back, so the rule carrying the port's entire reason for existing could
not be written at all.

The two claims most likely to be wrong here are both instrumented rather than
asserted. That the seam is *generic* is checked by a helper bound on
`ProjectionProbe` alone with the concrete store named only at instantiation.
That the feature is *independent* is checked by 16 host and 25 `wasm32`
combinations — and specifically by combination 3 of 16, `--no-default-features
--features conformance`, which is the one a `conformance` surface silently
coupled to `memory` would fail while staying green on defaults and on
`--all-features`.

What the reviewer is being asked to decide is not about this story's code: it is
whether to authorise the `spec/SPECIFICATION.md` citation re-point inside this
slice or to pull `unstable-projection-gate-and-clause-disposition` forward, and
separately who disposes of the three constitution rules whose subject the port
change deleted.

## Knowledge Harvest

**A placement decision nothing in the workspace can falsify needs its argument
written at the item, not only in an ADR.** `ProjectionProbe` lives in the
contract crate rather than the testkit for a coherence reason — an adapter's own
`tests/` directory is a third crate where neither a testkit trait nor the
adapter's type is local, so the orphan rule rejects the impl. Every fixture in
*this* workspace already depends on the testkit, so the wrong placement would be
green in every step of the gate and would fail only for an outside author, five
slices away. The argument is therefore written on the trait, with its falsifier
named, so it cannot be "simplified" on diff-size grounds by someone who only sees
the green gate.

**An empty feature list is a load-bearing assertion, and the manifest should say
so.** `conformance = []` *is* the economic argument — one flag, no new dependency
edge. A future `dep:` entry there would void the placement decision without
looking like it changed anything, so the comment beside it says that outright.

**The house rule caught its own named wrong implementation, first try.** RS-22-1
exists because `async fn` in a hand-written public trait fires under `-D
warnings`, and it names `#[allow(async_fn_in_trait)]` as the fix that looks
right and destroys the information. Copying the specification's fenced block
verbatim walked straight into it. That is a constitution atom earning its keep,
and it is worth recording that the *specification* carries the spelling the gate
forbids — a small drift between the two documents that `ps-clause-pairing-sweep`
could usefully pick up.
