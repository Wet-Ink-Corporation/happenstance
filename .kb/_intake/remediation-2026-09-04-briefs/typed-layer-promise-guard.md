# What should `the_typed_layer_makes_no_promise_it_does_not_keep` assert, now that the promise it names has a second half?

**Record id:** `typed-layer-promise-guard`
**Would supersede:** nothing. This is about one assertion in `xtask/src/main.rs` and the checks in `crates/happenstance/tests/` that stood beside it. No ADR and no clause names either; **PS-3** (`spec/SPECIFICATION.md:4928-4929`, and its discharge note at `:4945-4946`) and **ADR-0036** (`.kb/decisions/0036-the-projection-port-ships-gated.md`, *accepted*) already took the decision the code failed to implement, which is why audit entry **B-2** routes its code half to the release checklist rather than to an ADR.

**This brief did not get the two-critic pass the original thirteen had.** It was written by the lane implementing B-2, in the same session as the change it describes, and nobody independent argued the other side. `README.md`'s discount applies.

**It is a handoff, not only a question.** The lane that found this holds `crates/happenstance/` and `crates/happenstance-core/`; `xtask/` is another lane's. The code half of B-2 is landed and the guard is untouched.

---

## Why this is owed

`the_typed_layer_makes_no_promise_it_does_not_keep` (`xtask/src/main.rs:1551`; the audit cites `:1544`, which is where it sat before the tree moved) reads `crates/happenstance/src/lib.rs` and asserts on the literal `pub use runner::{` behind its `#[cfg]`/`#[cfg_attr]` pair — line 239 in the tree it was written against. The item it was actually about sat three lines below, at 242:

```rust
pub use happenstance_core::*;
```

The glob re-exported whatever the **compiled** contract crate exposed. `happenstance-core` gates its projection items on **its own** `unstable-projection`; `happenstance-testkit` enables that feature unconditionally and is `happenstance`'s dev-dependency, so under `cargo test` ten contract names — `Checkpoint`, `ProjectionId`, `ProjectionStore`, `SendProjectionStore`, `Authority`, `CommitError`, `ResetError`, `ProjectionProbe`, the `projection` module and `MemoryProjectionStore` — resolved at `happenstance::` with this crate's `unstable-projection` **off**.

Demonstrated before the fix, in the tree:

```
running 1 test
test the_unstable_projection_surface_is_reachable_without_its_feature ... ok
```

and, in the same tree:

```
test tests::the_typed_layer_makes_no_promise_it_does_not_keep ... ok
```

The guard stays green in a state it should reject. Constructed and confirmed: a crate root carrying `pub use happenstance_core::{Authority, Checkpoint, ProjectionId, ProjectionStore};` with **no gate at all**, four items of the unfrozen port handed to every caller, passes the assertion unchanged — as do `doc_budget`, `doc_surface`, `docs_composition` and `manifest_contract`. Nothing in the workspace could see below line 239.

### The four checks that made it look like policy

This is the part worth carrying forward, because it is a shape rather than a slip. Four green assertions required the glob **by name**. Every line number in this table is read at `dbc0376`, the Red commit — the state before the repair, which is the state they describe:

| Check | File |
|---|---|
| `the_glob_reexport_survives_and_nothing_shadows_it` | `crates/happenstance/tests/doc_budget.rs:256`, assertion at `:260` |
| `crate_root_renders_the_codec_surface` | `crates/happenstance/tests/doc_surface.rs:42`, assertion at `:133` |
| `crate_root_renders_the_projection_surface` | `crates/happenstance/tests/doc_surface.rs:253`, assertion at `:326` |
| `no_item_shadows_a_core_name` | `crates/happenstance/tests/docs_composition.rs:194`, assertion at `:197` |

Each asked for the *mechanism* where the promise it stood for is that the contract's paths still resolve through the facade. The story that landed them says so in its own acceptance criterion: *"Nothing shadows a contract name, and `pub use happenstance_core::*;` survives"* (`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md:424`). Requiring a mechanism rather than a property is how a defect acquires four defenders, and it is the same failure as an assertion reading the wrong line: both are green for as long as they are wrong.

---

## What is true today, after the lane

`crates/happenstance/src/lib.rs:264-312` is an explicit, `#[cfg]`-carrying list of every contract item, each behind the contract's own gate. `ProjectionProbe` is off it entirely — the contract gates it on `conformance`, which `happenstance` does not forward. `crates/happenstance/tests/contract_surface.rs` derives each item's gate from `happenstance-core`'s crate root, compares it against the hand list, and rejects four wrong implementations, each demonstrated by mutation:

1. the glob restored — `happenstance/src/lib.rs:242: a glob re-export — this file exists to forbid it`;
2. the explicit list with one gate dropped — `` `Authority` … behind {} … and written in `happenstance-core` behind {"unstable-projection"} ``;
3. an ungated contract item silently dropped from the facade — `` `happenstance-core` publishes `EventId` unconditionally … and `happenstance` does not re-export it ``;
4. `ProjectionProbe` mounted under the nearest plausible gate — `` gates on ["conformance"] — features this crate does not declare ``.

The four checks above were repointed at the property; their shadowing halves are untouched.

**The xtask guard is unchanged**, and it is still aimed at line 239.

---

## The question, and the options

`contract_surface.rs` now holds the property, in the crate the property is about. Does the xtask guard change, and if so how?

### Option A — leave the guard exactly as it is

- **Costs:** nothing today. The leak it missed cannot recur while `contract_surface.rs` lives.
- **What it buys:** no edit to a file another lane holds.
- **Against it, in its own words:** the guard's doc comment claims a scope it does not have — *"a feature table advertising a runner nobody can name, or four `pub use`s a consumer cannot turn on"*. The second clause is exactly what the glob did, for nine names rather than four, and the guard could not see it. A comment that overstates a check is how the next reader concludes the check already covers their case. **This is the strongest argument against A** and the reason this brief exists rather than a note.

### Option B — narrow the guard's own documentation to what it checks

Delete the second clause from the comment, and name `crates/happenstance/tests/contract_surface.rs` as the owner of the surface half.

- **Costs:** a comment edit in `xtask/src/main.rs`. No behaviour change, so no test moves.
- **What it buys:** the guard stops claiming the half it never had, and the pointer survives a reader who arrives at the xtask file first — which is where somebody looking for "what stops the typed layer over-promising" will look, because that is what the test is named.
- **Against it:** it documents a division of labour rather than enforcing one. Nothing stops a future author deleting `contract_surface.rs` and leaving the guard's pointer dangling. Answer: `lint-constitution` and the package assertion have the same exposure, and the repository's answer to it everywhere else has been a named pointer rather than a second check.

### Option C — teach the guard the surface half as well

Have the xtask assertion read `crates/happenstance/src/lib.rs` for the glob, and fail on it.

- **Costs:** a real edit to `xtask/src/main.rs`, in a file another lane holds, duplicating one line of `contract_surface.rs`.
- **What it buys:** the check runs in `cargo test -p xtask`, which is a gate step, without depending on `happenstance`'s own test targets being built. That is not nothing: `cargo hack check --no-dev-deps` — PS-3's own named rule — strips the dev-dependencies, and `happenstance`'s integration tests are not compiled at all under it.
- **Against it:** two checks for one fact, in two crates, is precisely the shape RS-81-5 warns about — the derived fact and the hand-written intention should sit together, and here the fact is `happenstance-core`'s crate root, which `contract_surface.rs` can `include_str!` and xtask would have to re-parse. The duplicate would also be the *weaker* of the two: a substring test for `::*;` catches the glob and none of the other three wrong implementations.

---

## Recommendation

**B**, and not C.

The half the guard was written for — the manifest passthrough and the runner surface agreeing — is genuinely its own, and it checks it correctly. The half it missed belongs where the fact it must be compared against lives, which is `happenstance-core`'s crate root, one directory from `contract_surface.rs` and four from xtask. What is actually broken in `xtask/src/main.rs` is a **sentence**, and the cheapest honest repair is to fix the sentence.

**The strongest argument against this recommendation:** the gate configuration where `contract_surface.rs` does not run is real. `cargo hack check --workspace --feature-powerset --no-dev-deps` rewrites every manifest without dev-dependencies, and an integration test target is not built at all under it. If the day comes that `cargo test --workspace --all-features` is dropped from the gate in favour of the powerset — not proposed, but the powerset is the step PS-3 names as its rule — the surface check silently stops running and nothing says so. If that trade is ever made, C becomes correct and this brief should be reread.

## Cost of delay

**Low, and bounded by one release.** The defect is fixed; only the guard's description is wrong. It costs nothing until somebody reads the comment and believes it. The natural moment is phase 12's release checklist (`RUNBOOK.md:4681`), where B-2 is already routed as a discharge item.

## What this does not settle

- Whether `happenstance` should re-export the contract's surface at all, rather than asking consumers to name `happenstance_core`. The facade promise is ADR-0006's and is not reopened here.
- Whether `cargo public-api` joins the gate. The explicit list is what would make its diff mean anything for this crate, which is B-2's own remediation note; nobody has decided to add the tool.
- The two example manifests whose comments still describe the glob — `examples/course-subscriptions/Cargo.toml:15` and `examples/transfers-on-sqlite/Cargo.toml:22`. Prose only, in a directory this lane does not hold.
