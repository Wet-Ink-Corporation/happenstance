# Wave `2026-08-20-intake-phase-9` — claims and classification

Every claim the five staged files carry, labelled against the **accepted decision corpus** and
routed to the operation that absorbs it. The labels are the four the ingest defines:

- **aligns** — the claim is consistent with an accepted decision and adds no obligation to it.
- **extends** — the claim adds knowledge no accepted decision holds, without contradicting one.
- **conflicts** — the claim cannot be true at the same time as an accepted decision's body.
- **requires-new-decision** — the claim asks for a commitment nobody has made.

**Zero claims in this wave are labelled `conflicts`.** That is the wave's headline fact and it
survived a deliberate hunt for one: three of the five files open by warning the wave against an
edit to an accepted atom, which reads at first like a conflict being declared. It is the
opposite — each is a file telling the wave that its content sits *behind* an accepted decision
rather than against it. The one thing that genuinely disagrees with a written instruction is
`RUNBOOK.md:4394-4396`, and `RUNBOOK.md` is not a decision atom.

## Modal-signal summary

| File | must/shall/must-not in its own voice | What that resolves to |
| --- | --- | --- |
| `0023-…` | several, and all load-bearing | One decision atom. C5's *"the atom must choose one and say which"* is the exception — a demand for a commitment, with both shapes named and neither taken |
| `es-6-…` | two, both process (immutability, and describing existing caller behaviour) | Evidence behind ADR-0009, folded into ADR-0023 |
| `cf-40-…` | two process `must`s, one `may not` | The `may not` restates the corpus's own immutability rule; the resolution's own prose commits by *position* rather than by modal, which is why it still earns a decision atom |
| `wf-11-…` | one process `must`, plus a must-say / must-not-say pair addressed to the atom author | A reference atom, plus a resolution; the must-not-say list is what keeps it out of the decisions layer |
| `0034-…` | none | A reference atom and evidence on an open question. Its only modal is a meta-instruction telling the wave **not** to treat it as a decision |

## `.kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **C1** | The title's "and", and `kb-playbook-one-decision-per-adr-title-001`'s stated exception — both halves settled by one body of evidence (the conformance suite against real `worker` bindings, off tokio, in one `cargo xtask ci`) | **aligns** — the playbook states the exception; the atom invokes it. No playbook edit follows | 4 |
| **C2** | The mapping's four surviving properties: `exec` is a plain `fn`; the cursor is not a snapshot and `SqlError::CursorInvalidated` reports it; everything is held `!Send`/`!Sync` through `Rc` under `worker`'s `unsafe impl Send`, guarded by four probes including `the_probe_is_not_vacuous`; the `RefCell` is *tried*, so re-entrant borrow **reports** rather than panicking | **extends** ADR-0001 and ADR-0011. First observation of either under execution on the target rather than under `cargo check` | 4 |
| **C2b** | Four alternatives that lost on the mapping side: the ceiling capture; `Query::index_arms()` in the contract; `JsThrow` over `StringifiedThrow`; keeping the hand-written stand-in | **extends** — and the `index_arms()` half **aligns** with ADR-0022, which rejected it one adapter over. The `JsThrow` half is CL-1's join with the ES-6 file | 4 |
| **C3** | The harness's shape is a finding, not a choice: `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`, against a `node:sqlite`-backed `DurableObjectState` shim, inside one `cargo xtask ci`. Four alternatives lost; one — a `workerd`-class runner — is **not rejected on merit** and is escalated. States what the harness does **not** prove | **extends**, and it **supersedes a runbook plan** (`vitest-pool-workers` as its own CI job) rather than a decision. The escalated half is CL-2 | 4, 1 |
| **C4** | ADR-0001 is **cited, not lifted**. `RUNBOOK.md:4394-4396` instructs an edit to an accepted immutable atom; the marker was already lifted at phase 1 and ADR-0008 records it. The refusal is written into the record so the next reader does not re-attempt it | **aligns** with ADR-0001, ADR-0008 and `kb-governance-referent-not-reasoning-001`. The disagreement is with a mutable runbook line, which is not a decision atom | 4 |
| **C5** | `cargo deny check bans` is **red**: `worker` 0.8.5 and `worker-macros` depend unconditionally on `async-trait`, which `deny.toml` bans under ADR-0001. No happenstance port gains a `Send` bound from it. Two shapes named — ratify a `wrappers` entry, or refuse and let the ban stay red | **requires-new-decision.** Verified: `deny.toml`'s `wrappers` list is `["wasm-bindgen-test"]` and its own comment says a second route *"is a new wrapper this list does not carry, and the check fails until someone decides it should"* | 2 |
| **C6** | Negative scope: no clause amended, no marker moved, no wire-format change, no accepted body edited; CF-39/CF-40/WF-11 resolved elsewhere | **aligns.** Realised as what the wave does *not* do, plus one `related` edge to Op 5 | 4 |

## `.kb/_intake/es-6-verdict-against-adr-0009s-prediction.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **claim-1** | ES-6 was **settled, not deferred**. `Error` keeps `core::error::Error + 'static` on both flavours; `Send + Sync` lives in the downstream `ThreadSafeEventStore` marker. ADR-0009's decision **holds**, and this adapter is the evidence for it rather than the exception to it. Project AC-005's wording predates ADR-0009's acceptance and is wrong on its face | **aligns** with `kb-decision-0009` — and this is the most important label in the wave. ADR-0009 named an asymmetry (a `!Send` error carrying a live JavaScript value) as the case where the absent bound would cost a caller something; phase 9 is the first runtime that can produce one, and it does not cost them | 4 |
| **claim-2** | The artefact: four reconstruction tests in `crates/happenstance-cloudflare/src/lib.rs`'s `es6_reconstruction`, executed on `wasm32-unknown-unknown` and named in `xtask/src/proof.rs`'s registry so they cannot be renamed or emptied silently | **extends.** Verified at `xtask/src/proof.rs:854-857` — all four names present, spelled exactly as claimed | 4 |
| **claim-3** | A green suite alone proves nothing: every conformance rule asserts on the success path or a store-produced `AppendError` and none reads an adapter error's contents. Two wrong classifiers were compiled into the **real** `classify_write` and rejected — two tests and three tests respectively | **extends**, and it discharges `CLAUDE.md`'s *"a rule that no adapter can fail is decorative"* against its own instrument | 4 |
| **claim-4** | `JsThrow`, not `StringifiedThrow` — and this verdict is the evidence for that mapping decision. Stringify early and the distinction is gone; **no bound on `Error`, `Send + Sync` included, would bring it back** | **aligns**, and **CL-1's join**: near-verbatim ADR-0023's own bullet at `0023-…:80-84` | 4 |
| **claim-5** | `store_error_crosses_a_join_handle` is **still unwritten and unowned**. The verdict does **not** resolve `kb-open-question-es-6-unwritable-rule-001`. *"Do not let the wave read this document as answering that question"* | **aligns**, and it is an explicit non-claim. `_implementation.md:1716-1718` makes the same refusal a condition of the wave's gate | 8 |
| **claim-6** | No public item was added to reach the verdict — a fallback accessor was provided for and was not needed | **aligns.** *"the strongest available answer to a design that records no surface"* | 4 |

## `.kb/_intake/cf-40-fixture-contract-ownership-resolution.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **c1** | Process: this is a staged answer, not an atom. The answer is a **new atom**; the question's record stays; `related` both ways; `status` → `withdrawn` or `superseded`; the body untouched; the map bullet **annotated in place, never removed**. Its `git diff` must show frontmatter-only hunks | **aligns** with `.kb/open-questions/README.md:41-45`. Executed as Ops 5 and 6 rather than recorded as content | 5, 6 |
| **c2** | Branch B coordination: HS-P0012 merged one position ahead and **did not** mint the answer — ADR-0022 records CF-40 as a non-verdict with a named owner, twice. Verified: `kb-decision-0022:30` and `:93-94`, and `open-questions-index.md:170-173` still reads **Open**. Exactly one minting across both projects | **aligns.** Provenance for Op 5's Context, not an atom | 5 |
| **c3** | The resolution. **(1)** CF-40 is ADR-0015's clause — its header and decision-8 prose assert it, and the hedge in its own Consequences is *superseded by use rather than by argument*, since ADR-0022 and ADR-0023 both cite CF-40 to ADR-0015 and neither claims it. **(2)** The fixture contract has **no single owning document**, and that is now a recorded position: a `CF-` clause is minted by the decision that first needs the capability and carries the reason there — with the cost stated, that a reader locating a fixture-contract clause reads §6 by subject rather than one ADR. **(3)** `CloudflareFixture` is the third data point: the first fixture in the workspace to declare all three of CF-40's ceilings *and* claim CF-39's `MID_BATCH_FAULT`, where every prior fixture left all three at `None` and the rule certified nothing | **requires-new-decision**, and this is the wave's second-hardest label. It resolves an accepted decision's **self-contradiction by adjudication rather than by edit**, and it commits a *pattern* for every future `CF-` clause. Rule 2 sends a commitment to the decisions layer | 5 |
| **c4** | Sub-question 3 stays open: phase 10's `POLL_BUDGET` question (ADR-0013 §8) is the next test of (2), and if it collides that is the evidence that would supersede this resolution | **extends** `kb-open-question-poll-count-rule-strength-001`, whose body already quotes ADR-0013 saying the call *"should be made by whoever owns the fixture contract"* — a premise Op 5 answers | 9 |
| **c5** | Neither ADR-0015 nor ADR-0012 nor ADR-0022 may be edited. All three are accepted. The resolution is a new atom that cites them | **aligns** with `.kb/decisions/README.md:9-13`. **No operation** — a README is not a merge target | — |

## `.kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **C1** | Process, identical in shape to CF-40's. Plus a hard prohibition: **no wire-format change belongs anywhere in this wave** — that decision is HS-P0017's and *"needs a decision record first"* | **aligns.** Executed as Ops 3 and 7 | 3, 7 |
| **C2** | The verdict is **(c): the condition is not constructible on this runtime.** The staircase asked for 2,047 pages; the host granted every one, taking linear memory to 2,169 pages = 142,147,584 bytes, past Cloudflare's documented 128 MiB per-isolate limit, refusing nothing. The walk stopped on the **probe's own budget**, not a refusal. Cause: `wasm-bindgen-test-runner` over Node with a `node:sqlite` shim, and a Node isolate has no per-isolate memory cap. What would supply it: a real Workers isolate, or a runner flag capping linear memory growth | **extends** `kb-decision-0016` and answers `kb-open-question-human-readable-encoding-limits-001`'s *"What forces it"* on the runtime the question assigned it to. The causal half is **CL-2** | 3, 1 |
| **C3** | At 128 MiB the firing payload is **36,604,834 bytes** (peak = payload × 11/3), **35×** the 1 MiB `MAX_EVENT_DATA_LEN` this fixture declares. So on this runtime — *even a real one* — no payload this store would accept can fire the falsifier; it can only fire on a payload another store accepted and this peer is asked to **forward**. Sharpens sub-question 1: a 65,536-byte `MIN_SUPPORTED_EVENT_DATA_LEN` payload round-trips through the human-readable path in five 64 KiB pages | **extends.** This is the claim that makes (c) a verdict rather than an absence — the condition is not merely unobserved, it is out of reach by two orders of magnitude on the store's own declared ceiling | 3 |
| **C4** | The category finding is **untouched and re-confirmed**: serde has no streaming entry point for a string, re-observed at six sizes. The published cost table reproduced exactly on `wasm32` from the published seed — 464,218 bytes human-readable against 348,163 binary at a 348,160-byte payload. Memory: 31 pages against 17, on identical bytes, with the cheaper path run first so the comparison is handicapped against the claim | **aligns** with `kb-reference-wire-format-measurements-001` — and **does not merge into it.** The dating rule; `00`, the refused dedup | 3 |
| **C5** | What changed is only the **peer condition**: it now has a measured answer on the runtime that was supposed to supply it, and the answer is that this runtime cannot | **extends.** This sentence is what licenses Op 7's `superseded` | 7 |
| **C6** | Sub-question 2 (streaming scheme / declared capability / JSON scoped to diagnostics) is **not reached** and is not this project's to choose — HS-P0017's, and it needs a decision record | **extends**, deferred by name to a project. A named backlog owner is a task, so this does **not** mint a new open question; the residual rides in Op 3's body and Op 7's dated section | 3, 7 |
| **C7** | Sub-question 3 (bearing on ADR-0003's opaque-payload boundary) is answered: **none**. The probe forwards bytes it never inspects — *"the boundary is observed here, not tested"* | **aligns** with `kb-decision-0003`. **No operation on ADR-0003**; a `related` edge from Op 3 | — |
| **C8** | Authoring guidance for the resolution — must say the condition is not constructible **and what would construct it** (*"(c) is a verdict because it names what is missing"*); must **not** say WF-11 is safe, that its marker moves, that its `MUST` re-scopes, or anything about the wire format | **aligns.** This list is the shape of Op 3's body, and it is the reason Op 3 is a `reference` atom rather than a decision: a document forbidden to move a marker or state a commitment is evidence | 3 |

## `.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md`

| Claim | Subject | vs the corpus | Op |
| --- | --- | --- | --- |
| **c1** | The metrics of one hand-run of the standing criterion (`RUNBOOK.md:3810-3820`) at baseline `53a4764`: 8 clauses in range plus 15 further passages; 3 unchanged / 5 repaired in range / 15 repaired out of range; **6 stale-but-resolving citations pointing at the wrong subject**; 4 factually false counts; 1 falsifier naming an event that had already happened. **Finding 1:** of 401 citations checked, only **80** are anchored (against 69 of 358 at phase 4/5), because `subject_before` declines whenever the nearest span is a type name, a quoted phrase or another citation — so six wrong-subject citations in `happenstance-sqlite/event_store.rs` went unreported *while the tool behaved exactly as documented*. Two mechanisations floated, **neither implemented** | **extends** `kb-reference-phase-4-5-spec-reconciliation-001` as a **sibling census**, not an append. `checked` is a coverage number; `anchored` is the only quality number | 10 |
| **c2** | **Finding 2:** the criterion's arithmetic bullet had nothing to close against — `kb-decision-0022` states **no clause range in any form**, and `RUNBOOK.md:301`'s row for it is the only one with no parenthesised range, so the comparison became a number against itself. Phase-4 precedent of the identical defect from the other side (35 named, 64 discharged, 29 invisible). Proposes a `clauses:` frontmatter key `redkiln validate --kb` could require — and **declines to answer** the sub-question it feeds | **extends** `kb-open-question-post-phase-reconciliation-001` sub-question 3. **No operation on `kb-decision-0022`**: the finding is that an immutable atom lacks a key, which is a schema question, not a defect an edit could fix | 11 |
| **c3** | **Finding 3:** of 20 repairs, only **6** were citation line numbers a machine could plausibly catch; **14** were sentences that were simply false. *"No conceivable extension of `spec-trace` catches 'seven' when the answer is twelve."* Cuts both ways — against believing a gate step could replace the pass, and for the pass having a named owner, because the failure mode is silent, durable, and reads as authoritative | **extends** the same question's sub-questions 1 and 5, and its *"Why this is a question and not a task"* tradeoff directly | 11 |
| **c4** | What was deliberately **not** done: no committed citation baseline (it would answer sub-question 2 against `kb-playbook-ratchet-gate-landing-001`'s no-count-beside-a-list rule); no gate step, no `xtask` check, no widened `ANCHOR_SLACK`; no edit to the open question; no clause amended, no marker moved, no ADR written | **aligns.** Mirrors `kb-reference-phase-4-5-spec-reconciliation-001`'s own *"What was not done, and the residual defect"* section, which is why it belongs in the same atom as c1 rather than anywhere else | 10 |

## Classification roll-up

| Label | Claims | Where they land |
| --- | --- | --- |
| **aligns** | 13 | Ops 3, 4, 6, 7, 8, 10 — plus four claims that produce **no operation at all**, correctly (`cf40-c5`, `wf-11 C7`, `0023-C1`'s playbook citation, and the two README procedures) |
| **extends** | 12 | Ops 1, 3, 4, 9, 10, 11 |
| **conflicts** | **0** | — |
| **requires-new-decision** | 2 | `0023-C5` → Op 2 (an open question, because the wave cannot sign it). `cf40-c3` → Op 5 (a decision atom, because the verdict arrives taken, with its losers named and its cost stated) |

**The two `requires-new-decision` claims are treated differently, and the difference is the
whole of wave 5's Adjudication 4 applied again.** `cf40-c3` arrives with the alternatives
named (a single named owner against piecemeal minting), the evidence counted (three ADRs,
three phases, no collision), the cost stated, and a reopen trigger identified. The wave
transcribes a decision that was taken. `0023-C5` arrives with two shapes named — ratify a
`wrappers` entry, or refuse and let the ban stay red — and **neither chosen**, by a document
whose own sentence is *"the atom must choose one and say which."* Authoring that choice here
would be the wave ratifying a dependency exemption against a binding constraint, with no
record, no human, and a red gate as the forcing pressure. It becomes an open question with the
forcing event named, which is what `.kb/decisions/README.md:41-43` requires of a decision not
yet taken.
