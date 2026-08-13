---
item: HS-P0013
stage: storymap
created: 2026-08-12T03:30:14.280Z
updated: 2026-08-12T03:30:14.280Z
template_sig: 1c63534a
rendered_sig: 27eb76a7
---

# Story Map — The edge store, run rather than asserted

Twelve stories across five milestones, covering `project.md`'s AC-001…AC-012. Every
claim below cites a path in this worktree; where a story and a clause in
`spec/SPECIFICATION.md` disagree, the clause wins, and where a story and an
**accepted** atom under `.kb/decisions/` disagree, the atom wins.

## Backbone

There is no screen here, so the backbone is stated in terms of the three people who
actually observe this project's output: the **adapter author** who must be able to
run the suite, the **gate reader** who must be able to see what ran, and the
**library consumer** who must be able to depend on the crate.

| # | Activity | Who observes it | Where it is observable |
|---|---|---|---|
| **A** | *Run a wasm32 conformance suite for real, from one command* | adapter author | a new named `Step` in `xtask/src/main.rs`'s `REQUIRED` array (`:105`), selected by name in `wasm_steps()` (`:784-791`) |
| **B** | *Store and replay events inside a Durable Object* | adapter author | `crates/happenstance-cloudflare/src/event_store.rs` — `read`, `append`, `head`, `contains_event_id` with no `todo!()` |
| **C** | *Prove the edge store conforms — every rule, executed* | gate reader | `event_store_conformance!` with `emit = happenstance_testkit::__emit_wasm`, shaped like `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27` |
| **D** | *Know what this runtime cannot do, and why* | gate reader | `SKIP <rule>: <reason>` lines (`crates/happenstance-testkit/src/contract.rs:473-483`), the fixture's `Capability` constants, and the atoms under `.kb/` |
| **E** | *Depend on the crate* | library consumer | `crates/happenstance-cloudflare/Cargo.toml` without `publish = false`, plus both licence files and a README |

Activity A is the one nothing in the tree does today: `xtask/src/main.rs:219-244`'s
comment says in as many words that the wasm32 harness step "does **not** run
anything", and `.github/workflows/ci.yml:196-237`'s `wasm-conformance` job — the only
place a rule actually executes on `wasm32` — is a GitHub Actions job, not an `xtask`
step, and runs on `ubuntu-latest` only (`:206`) while the `gate` job matrices across
three runners (`:34-39`). Closing that is the project's highest-risk item and is
sequenced first.

## Slices

Stories sharing a Milestone are implemented in one context and mounted as ONE
integrated surface. Cross-milestone `depends_on` edges are acyclic.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `wasm-execution-seam` | `wasm-execution-gate-step` | foundation | Add a named, non-skippable `Step` to `xtask` that *executes* `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` on `wasm32` inside one `cargo xtask ci`, guarded by a fourth `xtask/src/proof.rs` `Artefact` row so an emptied target fails instead of passing on `running 0 tests`, and settle the fate of `.github/workflows/ci.yml`'s standalone `wasm-conformance` job. | — | AC-004 |
| `real-worker-bindings` | `worker-binding-layer` | foundation | Replace the `worker`-free stand-in (`crates/happenstance-cloudflare/src/js.rs`, `src/sql_storage.rs`) with real Durable Object `SqlStorage` bindings, keeping all four modelled properties true and every type `Rc`-shaped so the `!Send` probes at `src/lib.rs:137-259` still hold — and price `worker` against ADR-0029's MSRV floor and `cargo deny` on the way in. | — | AC-001, AC-005 |
| `real-worker-bindings` | `durable-object-write-path` | capability | Implement `migrate`, `append`, `head` and `contains_event_id` against the real bindings — identity columns (`origin_store`, `origin_position`, `event_store.rs:187-195`), ADR-0014 store-id incarnation, DCB conflicts classified into `AppendError::ConditionViolated` *before* `Self::Error` exists, capacity refusals into `AppendError::ExceedsStoreLimit`, and the 2^53 ceiling surfaced as `CloudflareEventStoreError::StoredPosition` rather than by truncation. | `worker-binding-layer` | AC-001, AC-007 |
| `real-worker-bindings` | `durable-object-read-path` | capability | Implement `read` as ADR-0011's ceiling-and-page — still non-`async`, stream at the top level, ceiling captured no later than the first poll, every later statement bounded by it, no cursor held across an `await` — so ES-9's laziness and the isolation clauses both hold on a cursor Cloudflare documents as not a snapshot. | `worker-binding-layer` | AC-001, AC-007 |
| `real-worker-bindings` | `caller-visible-error-verdict` | capability | Commit the ES-6 artefact: a test that reconstructs, from a *caller-visible* `CloudflareEventStoreError` carrying a real `worker::Error`, the one fact a caller branches on — constraint violation versus transport fault — and fails if the error type stops carrying it. | `durable-object-write-path` | AC-005 |
| `durable-object-conformance-run` | `durable-object-host-and-fixture` | capability | Land the Durable Object host (a test-and-example surface, not a second public API) and `impl Fixture for CloudflareFixture` — one instance is one object's storage, each `connect()` one handle onto it, `SECOND_HANDLE` `SUPPORTED`, and every declined `Capability` carrying a real non-empty reason — all reaching the store through the one `CloudflareEventStore::new(sql)` constructor. | `durable-object-write-path`, `durable-object-read-path` | AC-003, AC-008 |
| `durable-object-conformance-run` | `every-rule-under-workerd` | capability | Add the crate's conformance target invoking the shipped `event_store_conformance!` with `__emit_wasm`, register it in the gate step from `wasm-execution-seam` so one `cargo xtask ci` executes it under a real Durable Object runtime, and state the concurrency family's non-invocation as a documented reason rather than an absence. | `wasm-execution-gate-step`, `durable-object-host-and-fixture` | AC-002, AC-003, AC-004 |
| `durable-object-conformance-run` | `measured-store-limits` | capability | Measure rather than guess `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH`, and decide `REOPEN` and `MID_BATCH_FAULT` honestly against the real runtime, so CF-39/CF-40's rules pass at the boundary in both directions. | `durable-object-host-and-fixture`, `every-rule-under-workerd` | AC-008, AC-007 |
| `evidence-and-verdicts` | `wf-11-memory-ceiling-falsifier` | capability | Test WF-11's falsifier where the ceiling is real: forward a payload sized against this runtime's measured memory ceiling through the adapter's actual non-streaming encode path under the `workerd` run, and record whether it bites — gathering evidence only, changing no wire format. | `every-rule-under-workerd` | AC-011 |
| `evidence-and-verdicts` | `deferral-re-reads-and-es-32-verdict` | capability | Write the per-clause statement of whether CF-14's and CF-27's deferrals still hold on this runtime (handing CF-27's completeness half onward), and the one-paragraph ES-32 tail-seam verdict in `RUNBOOK.md`'s ledger — recorded, not acted on — with `cargo xtask spec-trace` still green. | `every-rule-under-workerd` | AC-009, AC-010 |
| `evidence-and-verdicts` | `adr-0023-and-atom-resolutions` | capability | Stage `.kb/_intake/` and run the ingest path so ADR-0023 lands as an accepted atom stating the `SqlStorage` mapping and the off-tokio harness as one question with the alternatives that lost, and so CF-40's ownership and WF-11's atom move to *resolved rather than deleted*. | `durable-object-read-path`, `caller-visible-error-verdict`, `every-rule-under-workerd`, `measured-store-limits`, `wf-11-memory-ceiling-falsifier` | AC-005, AC-006, AC-008, AC-011 |
| `publish-readiness` | `publish-ready-crate` | capability | Make the crate depend-on-able: `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` beside `Cargo.toml`, `publish = false` removed so `cargo xtask package-check` covers it, the name reserved per `xtask/src/reserve.rs`, and `cargo xtask ci` green including every `wasm32` step. | `worker-binding-layer`, `every-rule-under-workerd`, `measured-store-limits` | AC-012 |

### Why these milestones and not others

- **`wasm-execution-seam` is a foundation, and it is consumed inside this project.**
  It lands real in-tree substrate — a `Step` row, a `wasm_steps()` name, an
  `xtask/src/proof.rs` `Artefact` row — not a double and not a fixme, and it is
  demonstrated the day it merges against a target that already exists and already
  passes (`crates/happenstance-testkit/tests/memory_conformance_wasm.rs`).
  `every-rule-under-workerd` is its consumer, so it is not an unproven island. It is
  sequenced first because the intake brief's own blocking question — *whether a
  `workerd` runner can be made to exist in CI at acceptable cost* — is answered here
  or escalated here, and answering it after the adapter is written is answering it
  too late.
- **`worker-binding-layer` is the other foundation.** Swapping the stand-in is not
  user-observable on its own, but it is real substrate consumed by the three
  capability stories in its own milestone, and it carries risks that must not be
  smeared across them: `worker`'s MSRV, `cargo deny`'s widened graph, and the finding
  that a real `JsValue` is `Send + Sync` on non-`atomics` `wasm32`
  (`crates/happenstance-cloudflare/src/lib.rs:33-61`) — which is exactly how
  `Send`-ness could be restored by accident through an escape hatch this workspace's
  `unsafe_code = "forbid"` denies the adapter itself.
- **The read and write paths are separate stories inside one milestone, not separate
  milestones.** They land in the same file against the same bindings and are
  meaningless apart — a store that appends but cannot replay conforms to nothing —
  but they fail for different reasons (ADR-0011's ceiling versus ADR-0013's ceiling)
  and deserve separate diffs.
- **The fixture, the host and the conformance target are one milestone.** A fixture
  with nothing to mount it on, or a conformance target with no host to reach, is the
  "build it" / "wire it in" split this map is required not to make. They are mounted
  together or not at all.
- **`measured-store-limits` is a story rather than a checkbox** because the numbers
  are facts, not trades (`crates/happenstance-testkit/src/contract.rs:214-279`):
  stating one promises that exactly that many bytes are accepted and one more is
  refused as `ExceedsStoreLimit`, and a guessed number fails
  `append_reports_exceeded_store_limits` in one direction or the other.
- **`adr-0023-and-atom-resolutions` owns every KB write, and nothing else does.**
  The evidence is gathered by the stories that can gather it; the atoms are minted
  once, through `/redkiln:kb-ingest`, never hand-written into `.kb/decisions/`. Two
  standing hazards live here: **ADR-0001 must not be edited** — its provisional marker
  was already lifted at phase 1 and the atom is accepted and immutable, so this
  project's real-runtime evidence is cited *from* ADR-0023 and the long-form record
  under `references/adr/` — and **CF-40 must not be minted twice**, so this story
  coordinates with `sqlite-durable-store` (HS-P0012), which merges one position
  ahead: whichever reaches the answer first owns it and the other cites it.

## Coverage

Every project AC in `project.md` is claimed by at least one story, and no two stories
own the same responsibility for one AC — where an AC appears twice, the stories own
*different halves* of it, named in the last column.

| Project AC | Stories | Split of responsibility |
|---|---|---|
| AC-001 (the adapter is real) | `worker-binding-layer`, `durable-object-write-path`, `durable-object-read-path` | bindings / write bodies / read body. The last `todo!()` and the scoped `#![allow(clippy::todo)]` (`crates/happenstance-cloudflare/src/lib.rs:121-126`) go with whichever body lands last |
| AC-002 (every rule runs and passes) | `every-rule-under-workerd` | sole owner |
| AC-003 (declined capabilities report a reason) | `durable-object-host-and-fixture`, `every-rule-under-workerd` | the fixture *declares* the reasons / the run *emits* them and documents the concurrency family's non-invocation |
| AC-004 (inside the gate, not beside it) | `wasm-execution-gate-step`, `every-rule-under-workerd` | the step exists, is named not indexed, and is non-vacuous / the Cloudflare target is what it runs |
| AC-005 (ES-6 decided with an artefact) | `worker-binding-layer`, `caller-visible-error-verdict`, `adr-0023-and-atom-resolutions` | the error type stays genuinely `!Send` / the committed reconstruction test / the record |
| AC-006 (ADR-0023 accepted) | `adr-0023-and-atom-resolutions` | sole owner |
| AC-007 (the two non-type-error limits) | `durable-object-read-path`, `durable-object-write-path`, `measured-store-limits` | (a) the non-snapshot cursor versus ES-9 / (b) the 2^53 ceiling reported as `StoredPosition` / (b) the boundary observed by the suite |
| AC-008 (CF-39, CF-40, ownership) | `durable-object-host-and-fixture`, `measured-store-limits`, `adr-0023-and-atom-resolutions` | the fixture's shape / the measured numbers / the atom resolved, coordinated with HS-P0012 |
| AC-009 (CF-14, CF-27 re-read) | `deferral-re-reads-and-es-32-verdict` | sole owner |
| AC-010 (ES-32 verdict on disk) | `deferral-re-reads-and-es-32-verdict` | sole owner |
| AC-011 (WF-11's falsifier) | `wf-11-memory-ceiling-falsifier`, `adr-0023-and-atom-resolutions` | the evidence / the atom resolved |
| AC-012 (publish-ready, gate green) | `publish-ready-crate` | sole owner |

**No AC is orphaned: AC-001 through AC-012 all appear above.** Two exclusions are
deliberate and are not gaps. The initiative-level DoD 1–15 re-observation belongs to
`closeout-and-durable-audience` (HS-P0019), the only project wired to the terminal
`verify.e2e` grain; and actually publishing this crate at a version belongs to
`publication-and-positioning` (HS-P0016) — `publish-ready-crate` stops at
*publish-ready* plus a `0.0.0` name reservation.

This project's own DoD items map onto the same set rather than adding work: DoD 1 is
AC-004, DoD 2 is AC-003, DoD 3 is AC-001's grep, DoD 4 is AC-005, DoD 5 is AC-006 and
AC-008, DoD 6 is AC-010, and DoD 7 is the per-story `cargo xtask affected --base main`
and per-project `cargo xtask ci --fast` grain that every story below runs.

## Merge order

Slice by slice, foundations before their consumers.

1. **`wasm-execution-seam`** — `wasm-execution-gate-step`.
   First, and deliberately so. It has no dependencies, it is the project's only
   blocking-finding risk, and it converts the hardest question into a merged answer
   before any adapter code depends on it. Its own proof is a target that already
   exists, so it cannot be "finished" by moving a goalpost.
2. **`real-worker-bindings`** — `worker-binding-layer` → then
   `durable-object-write-path` and `durable-object-read-path` (unordered with respect
   to each other; both depend only on the bindings) → then
   `caller-visible-error-verdict`.
   Independent of milestone 1 by construction — no edge is declared between them, and
   inventing one would be dishonest — but sequenced second because a bound adapter
   with nowhere to run it is the position this project starts in.
3. **`durable-object-conformance-run`** — `durable-object-host-and-fixture` →
   `every-rule-under-workerd` → `measured-store-limits`.
   The first milestone that needs both of the ones above. `every-rule-under-workerd`
   is where AC-004 actually closes and where the project's proof artefact first
   exists.
4. **`evidence-and-verdicts`** — `wf-11-memory-ceiling-falsifier` and
   `deferral-re-reads-and-es-32-verdict` (unordered with respect to each other) →
   `adr-0023-and-atom-resolutions` last, because it records what the other four
   milestones found.
5. **`publish-readiness`** — `publish-ready-crate`.
   Last. Its dependencies place it after milestone 3, and AC-012's "`cargo xtask ci`
   green including all four `wasm32` steps" is only a meaningful claim once the fifth
   one from milestone 1 is in the array too.

Milestones 4 and 5 have no edge between them and could merge in either order; the
listing above puts publication last so that the final green gate is observed against
a tree whose atoms are already accepted and whose `redkiln validate --kb` is clean.

## Standing detectors these stories must not silence

Carried forward from the architecture brief's Notes §7, restated here because a story
map is what an implementer reads. If any of these goes quiet, the change is wrong
even when the gate is green:

1. The four `!Send` probe tests including `the_probe_is_not_vacuous`
   (`crates/happenstance-cloudflare/src/lib.rs:180-259`) — owned by
   `worker-binding-layer`.
2. `send_shape::send_flavour::SendStoreWithLocalError` still compiling
   (`crates/happenstance-cloudflare/src/lib.rs:87-94`).
3. The two `read`-shape tests in `crates/happenstance-core/src/memory.rs`; neither may
   be deleted (`CLAUDE.md`, constraint 3) — `spawns_from_generic` is the one that
   rejects an `async fn read` refactor.
4. `registry::no_orphan_rules` and the single `for_each_event_store_rule!` enumeration
   (`crates/happenstance-testkit/src/lib.rs:84-89`) — a `wasm`-only rule list anywhere
   in the tree fails AC-002 by construction.
5. `cargo xtask spec-trace` — CF-14, CF-23, CF-27, CF-39, CF-40, ES-6, ES-7, ES-9,
   ES-17, ES-32 and WF-11 all carry citations these stories touch.
