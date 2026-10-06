# Assessment — architecture and contract

Commit `1f92d08` (== origin/main), 2026-10-06. Read-only. Sources: `map-core.md`, `map-spec.md`,
`map-adapters.md`, `gate-summary.md`, plus direct spot-checks in `audit-wt` and CI run 37406385109
(GitHub Actions job list, read via MCP).

**Rating: AMBER. Completeness against the 1.0 target: about 70%.**

**Headline:** The core contract is well designed, and the code holds its binding constraints, with tests
enforcing them. The two-flavour port design is enforced, not just documented. The weak points are at the
edges: one published adapter does not meet a read-snapshot MUST, another fails a size floor on its real
platform, and the replication port is frozen on paper but has no implementation or tests. Its build is
scheduled after the 0.4.0 breaking window closes.

## Documents claim vs code and tests show

| Claim | Shown by |
|---|---|
| `EventStore::read` is not async and returns the stream at the top level | `crates/happenstance-core/src/store.rs:184-188` (`fn read(...) -> impl Stream<...>`). `append`, `head` and `contains_event_id` are `async fn` (`:283,:318,:341`). |
| Both ports are derived by `trait_variant` | `store.rs:158` `#[trait_variant::make(SendEventStore: Send)]`; `projection.rs:425` `#[trait_variant::make(SendProjectionStore: Send)]` |
| Two Send tests exist | `memory.rs:628` `send_flavour_stream_is_send_in_generic_code`; `memory.rs:657/680` `spawns_from_generic` with a generic `S: SendEventStore` |
| No `#[async_trait]` | `grep -rn '#\[async_trait' --include=*.rs crates examples harness` finds only a string at `crates/happenstance/tests/manifest_contract.rs:298` |
| serde is not a default feature of core | `crates/happenstance-core/Cargo.toml:48` `default = ["std", "memory"]`; serde is `optional = true` (`:35`) |
| Core depends on nothing in the workspace; no adapter depends on another | core `[dependencies]` contains only bytes, futures-core, thiserror, trait-variant, serde?, base64?. The only adapter-to-sync edge is a versionless **dev**-dependency in `crates/happenstance-sqlite/Cargo.toml:115`, which Cargo strips on publish (the comment at `:98-107` explains why). |
| Gate green | `gate-summary.md`: `cargo xtask ci` exits 0, with 2,965 tests passed and 0 failed. CI run 37406385109: gate ×3, live-postgres, live-neon and msrv all succeed. **The `workerd` job fails.** |
| 203 clauses: 152 frozen, 32 provisional, 12 deferred, 7 non-normative | `spec/SPECIFICATION.md:228-236` agrees with the generated table `:9791-9800`. `grep -cE '^\| [A-Z]{2}-[0-9]+ \|'` = 203. |

## Strengths

- The two-flavour design is enforced at the definition, not only at concrete types. The second test closes the `async fn read` refactor hole with a real `tokio::spawn`. The same pattern is repeated in the typed layer (`crates/happenstance/tests/flavours.rs:117,549`).
- The ports are small. `EventStore` has 4 methods; `ProjectionStore` has 5 plus `Batch`/`Error`. Neither carries a `Send` bound, and that choice is documented (`projection.rs:438-444`).
- Illegal states are pushed into types: `NonZeroU64` positions, a `NonZeroU32` `Retry` with a compile_fail proof, and `#[non_exhaustive]` errors and status types.
- There is a real spread of storage shapes behind the ports: a serialising lock (memory, sqlite, Durable Object), positions allocated outside the transaction (postgres, `xid8` frontier), and connectionless HTTP (neon). Two projection-batch shapes (buffered vs live sqlx tx) each pass the projection suite.
- The spec is machine-checked. The clause↔rule trace is regenerated and diff-checked by `cargo xtask spec-trace`. A 21-name sample resolved 21/21 (map-spec §4). The testkit proves its own rules with 84 + 18 named mutants and an `every_rule_has_a_mutant` meta-test.
- No production `unsafe`, `todo!` or `unwrap` exists in core, the typed layer or the testkit (map-core §2–4, workspace lints `unsafe_code = forbid`, `unwrap_used = deny`, `todo = deny`).

## Findings

### A1 (high): A published adapter does not satisfy ES-11 ("a read is a snapshot")
`happenstance-neon` fails `read_result_is_stable_under_concurrent_append` intermittently. A read and an
append are independent HTTP requests. The clause says this itself: `spec/SPECIFICATION.md:3087` ("its
falsifier has fired … ADR-0061 … states that `happenstance-neon` does not satisfy this clause"), and so do
`crates/happenstance-neon/src/lib.rs:17-24`. ES-11 and ES-12 are only `[PROVISIONAL]`. The phase-17 item
that settles them is still unchecked (`runbook/phases/17-breaking-window.md:162`). The required live-neon
check is strict, so it flakes (it passed in run 37406385109). The open question is whether a conformant
connectionless shape exists at all. If one does not, either the MUST weakens, which changes the contract
every consumer relies on, or the crate leaves the release set.

### A2 (high): Cloudflare fails a spec floor on its real platform, and its published limits are wrong there
CI run 37406385109, job "conformance under workerd, local and deployed", reports
`conclusion: failure`. Rule `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (VT-23, 128
query items) fails both locally (96/97) and on a deployed Durable Object (95/96), with "too many SQL
variables". The adapter publishes `MAX_QUERY_ARMS_PER_STATEMENT = 400` and
`MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000`
(`crates/happenstance-cloudflare/src/event_store.rs:402,423`), both justified by stock-SQLite defaults.
The measured platform walls are 5 compound terms and 100 bound parameters
(`experiments/durable-object-limits/results/`). The gate's Node shim never enforced these, so the gate
stayed green. The fix (L6b, `json_each`, ADR-0083) is planned but not in the tree. `workerd` is not a
required check, so `main` merged red.

### A3 (high): The replication port is frozen on design only, and its build is scheduled after the breaking window
21 SY clauses are `[FROZEN]`, but 32 of 35 SY rule cells are † (no rule exists)
(map-spec §4). `happenstance-sync-testkit` does not exist (`ls crates`). §1.3 admits that "`SY` clauses bind
only the design" (`SPECIFICATION.md:198-199`). `crates/happenstance-sync` is 1,573 lines and `publish = false`.
Phase 13 depends on 17 and 18 (`runbook/README.md:104`), so it runs after `0.4.0`, the planned breaking
window. Seventeen non-frozen clauses wait for phase 13 (VT-6 StoreId, VT-9, VT-21, VT-24, SY-7/10/14/18/20/22/23/27–31,
CF-40), and four of them touch `happenstance-core` value types. Running sync after the window makes it
the most likely source of a post-0.4 breaking change to core. That breaks the project's own rule that a
port must be frozen only once something sits at the far end of its riskiest axis (CLAUDE.md, CF-25).

### A4 (medium): The typed layer ships the design an accepted ADR rejected
`crates/happenstance/src/runner.rs:50-59` argues "Why `apply` is synchronous", and `:95-99` declares
`fn apply(&mut self, event: Self::Event, batch: &mut StoreBatch<Self>) -> Result<(), StoreError<Self>>`.
ADR-0074 (`.kb/decisions/0074-projection-apply-is-async.md`, `status: accepted`) chose the opposite: an async
apply handed a `Delivered<E>`, failing with the projection's own error, under `trait_variant`.
`grep -rn Delivered crates/happenstance/src` finds nothing. Phase 18 is not started. The runner is behind
`unstable-projection`, so this is not a semver hazard. It is still a live contradiction between the
rustdoc and the decision record. Six runner-level PS clauses (PS-25–30) have no rules ("Scheduled", map-spec §4).

### A5 (medium): Several breaking-change candidates are still open inside the window
Unchecked in `runbook/phases/17-breaking-window.md:61-136`: the ADR-0022 §9 reproduction (a runtime
`Handle` captured at construction, `crates/happenstance-sqlite/src/event_store.rs:512`,
`crates/happenstance-postgres/src/event_store.rs:314`; the remedy "may change two published adapters'
constructors"); ES-17 `events: &[Event]` borrowed vs owned (`store.rs:283-287`, ADR-0055 measurement owed);
`ProjectionId` unvalidated together with SY-31's reserved prefix; whether `Codec` is sealed; the SQL batch
statement type; and VT-6 restore detection for Postgres/Neon. Each one can change a published signature.
None is answered yet, and `0.4.0` is unreleased (`Cargo.toml:24` `version = "0.4.0"`; CHANGELOG
`[Unreleased]`).

### A6 (medium): The two-flavour derivation rests on a caret-pinned proc macro
`trait-variant = "0.1.3"` (`Cargo.toml:151`) emits `SendEventStore`, `SendProjectionStore` and the
bridging blanket impl that constraint 4 depends on. Every shape guard is `#[cfg(test)]` or reads
`Cargo.lock` (`crates/happenstance-core/tests/trait_variant_pin.rs`), so it checks only this workspace's
`--locked` resolve. A consumer's resolve can float to 0.1.x versions that were never compiled here
(`.kb/open-questions/trait-variant-caret-resolves-past-the-locked-gate.md`, accepted open question,
classified additive and deferred). `store.rs:147-155` also notes that a doc alias depends on the macro's
internal name, `TraitVariantBlanketType`.

### A7 (medium): Some frozen clauses have no executable backing, and one is argued unwritable
ES-6 (`[FROZEN]`, `SPECIFICATION.md:2728-2730`) fixes `Error` with no `Send`/`Sync` bound. Its rule
`store_error_crosses_a_join_handle` is argued **unwritable against today's port**
(`crates/happenstance-cloudflare/src/send_shape.rs:14,95`; `.kb/open-questions/es-6-names-an-unwritable-rule.md`,
sub-questions 1–3 open). ES-29, ES-31, ES-38 and PS-26/28/29 are frozen with † "Scheduled" rules. That
leaves 9 frozen ES/PS clauses with nothing executing them, beyond the SY family.

### A8 (low): Conformance coverage is uneven across the real adapters
The property-based `ops_agree_with_the_model` is skipped on Postgres and Neon, because both decline
`READ_YOUR_OWN_WRITES` (map-adapters §3, CI log). It runs only against memory and SQLite.
`LivePostgresProjectionStore`, the instrument ADR-0063 used to freeze the projection port, is not run by
any CI job (`grep -n -- '--test ' .github/workflows/ci.yml` has no `live_projection`). The projection port
is therefore frozen against a far end that CI does not re-check. Cloudflare implements no
`ProjectionStore` at all.

### A9 (low): Spec prose drifts from the generated facts
`SPECIFICATION.md:9775` says "`ES` is 76 % frozen" (the generated table gives 35/43 = 81%). `:9782` says "`PS`
is 51 % frozen because it has no implementation at all" (actually 26/38 = 68%, and four adapters pass the
projection suite). Other drifts: "eighty-nine rules" (`:409`) vs 95 rule fns in `suite.rs`; CLAUDE.md's
"five-line facade" vs 5,027 src lines; and a stale `todo!()` comment at
`crates/happenstance-postgres/tests/postgres_conformance.rs:153`. The generated sections are reliable. The
hand-written prose around them is not, and readers use that prose to judge maturity.

## Where the design is weakest or most likely to need a breaking change (ranked)
1. The snapshot semantics of ES-11/ES-12 vs connectionless transports (A1).
2. Core value types and wire format once sync is built after 0.4.0 (A3).
3. Adapter constructors under ADR-0022 §9, and `&[Event]` in `append` (A5).
4. The typed `Projection` trait moving to async apply, which is gated and so not semver-breaking (A4).
5. The Cloudflare adapter's limit constants and query rendering (A2). This is additive or behavioural, not a port change.
