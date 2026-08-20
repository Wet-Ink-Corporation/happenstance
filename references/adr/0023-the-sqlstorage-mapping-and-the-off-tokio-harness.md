# ADR-0023: The `SqlStorage` mapping and the off-tokio harness are one decision, because one run settles both

- **Status:** **accepted**, 2026-08-20. The `/redkiln:kb-ingest` wave ran and minted
  the atom this record summarises: [`kb-decision-0023`](../../.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md),
  `status: accepted`, which links back here through its `source_paths`. This file
  stays the long form — the transcripts, the rejected alternatives and the cost
  tables an atom cannot hold; cite it by `file:line` and link the atom.
  This line said `proposed` until the wave ran, exactly as it promised it would.
- **Date:** 2026-08-19
- **Settles:** the ADR queue's row **0023** (`RUNBOOK.md:302`) — *"Cloudflare: the
  `SqlStorage` mapping and the `workerd` harness."* One question, two consequences
  that hang off it, and one finding it must dispose of rather than absorb.
- **Discharges no clause and amends none.** CF-39, CF-40 and WF-11 are discharged
  or resolved through their own atoms; CF-23, ES-6 and ES-9 are honoured by the
  stories that ran, not changed. Nothing `[FROZEN]` is approached, and no maturity
  marker moves.
- **Cites rather than lifts:**
  [ADR-0001](0001-async-port-flavours.md). §4 says why the runbook's *"formally
  retired"* instruction was read and not obeyed.
- **Judges rather than amends:**
  [ADR-0009](0009-error-send-sync.md). §5 records what a real `!Send` error did to
  its ES-6 prediction, as new content. That atom is byte-identical to `main`.
- **Reverses a prior manifest decision, on a measurement, and records the price it
  did not pay:** the hand-written `SqlStorage` stand-in is replaced by the real
  `worker` bindings, and `cargo deny check bans` is **red** as a result. §7.

---

## 1. The question, and why its title carries an "and"

**As queued:** *Cloudflare: the `SqlStorage` mapping and the `workerd` harness
(`vitest-pool-workers` as its own CI job).*

**As answered:** *How does an event store map onto a Durable Object's `SqlStorage`,
and what harness proves it — given that neither `tokio` nor a thread is available
to it?*

`.kb/playbooks/one-decision-per-adr-title.md` is this corpus's own finding that a
conjunction in a decision title is usually a strong decision carrying a weak one.
It states its own exception: *"It stops being worth the split when both halves are
settled by the same evidence, in which case the conjunction is describing one
decision with two consequences."*

This is that case, and the evidence is nameable in one clause: **the conformance
suite executing against the real `worker` bindings, off tokio, inside one
`cargo xtask ci`.** Every claim in §2 is a claim about what that run observed;
§3's only justification is that it is what made §2 observable at all. Split into
two atoms, each would cite the same run and neither would be readable alone.

The playbook is cited in the atom for a practical reason rather than a formal one:
the next reader will check the conjunction against it, and should find the
exception already answered rather than have to re-derive it.

---

## 2. The mapping, and the four properties it had to survive

The adapter carried a hand-written stand-in for two phases. It modelled four
properties, and this phase's contribution is that all four are now **checked
against the real bindings** rather than asserted about a model
(`crates/happenstance-cloudflare/src/sql_storage.rs:1-30`).

### 2.1 `exec` is synchronous

`worker::SqlStorage::exec` is a plain `fn` returning `Result<SqlCursor>` — no
future, no connection to acquire, because the storage is co-located with the
object. Cloudflare's own documentation says so in terms: *"SQL queries using
`ctx.storage.sql.exec()` complete synchronously."*

This is the one storage in the workspace for which
[`EventStore::read`](../../crates/happenstance-core/src/store.rs) **not** being
`async` is *free* rather than merely correct. ADR-0001's two-trait design was paid
for by this runtime; §2.1 is the first place the payment returns something.

### 2.2 The cursor is not a snapshot

Cloudflare: *"Although a cursor object can technically be held across an `await`,
it does not provide a stable snapshot of query results."* A lazy stream **is** a
cursor held across awaits, so this is a capability limit rather than a type error —
the type system will happily let you write the wrong thing.

The limit is *preserved* rather than papered over. `SqlError::CursorInvalidated`
reports it, and ADR-0011's ceiling-and-page mechanism is what keeps a cursor off a
suspension point in the read path: take the ceiling **first**, page under it. The
read path carries three committed negative controls for that mechanism —
`a_ceilingless_paging_read_is_rejected`, `a_cursor_held_across_a_poll_is_rejected`
and `a_null_head_ceiling_is_rejected_on_the_empty_store` — so the claim is one an
adapter can fail rather than one it can only assert.

**This property is also what decides ES-32's verdict one document over.** A tail
seam on this storage can never be a held stream; see `RUNBOOK.md`'s phase-9 session
log.

### 2.3 Everything is `!Send` and `!Sync`

Not by accident and not by inheritance. `worker` declares
`unsafe impl Send for SqlStorage {}` and the same for its cursor
(`worker-0.8.5/src/sql.rs`), so holding either of them bare would hand this crate
`Send`-ness through an escape hatch its own `unsafe_code = "forbid"` policy denies
it. Both are held through an `Rc`, which is `!Send` for every payload.

The guard is four probes, including the positive control
`the_probe_is_not_vacuous`, and they now run on `wasm32` as well as on the host —
which matters here specifically, because `wasm-bindgen`'s
`cfg(not(target_feature = "atomics"))` `unsafe impl` is live only on that target.
A `!Send` claim checked only where the leak cannot appear is a claim about the
wrong machine.

### 2.4 The object is single-threaded and re-entrant

Shared state is reached through a `RefCell` that is **tried**: a second statement
issued while the first still holds it *reports* rather than panicking, which
`sql_storage::tests::reentrant_borrow_is_reported_not_panicked` asserts. The
alternative — a plain `borrow_mut` — takes the whole object down, and it would do
so on a code path a caller can reach by accident, because a Durable Object is
re-entrant by design.

### 2.5 Alternatives that lost on the mapping side

| Alternative | Why it lost |
| --- | --- |
| **Capture the read ceiling *after* opening the cursor** | Cannot bound a cursor that is not a snapshot (§2.2). ADR-0011's ceiling-first ordering is what makes a lazy read expressible at all here, and the three negative controls are what stop the ordering being quietly reversed later. |
| **`Query::index_arms()` decomposition pushed into the contract** | It does not exist in `happenstance-core`; the decomposition stays adapter-private. Rejected one adapter over for the same reason (ADR-0022 §3), and one adapter wanting it is not grounds to widen a frozen contract. |
| **`StringifiedThrow` — stringify the thrown value at the boundary** | Loses the one fact a caller must branch on. See §5: the constraint-violation/transport-fault distinction is recoverable only while the thrown value is still a value, and no bound on `Error` would bring it back afterwards. |
| **Keep the hand-written stand-in; take no `worker` dependency** | Reversed on a measurement. See §7, including the price the reversal has *not* paid. |

---

## 3. The harness, and the winning shape is a finding rather than a choice

### 3.1 The two instructions that did not agree

`RUNBOOK.md:302` queued *"the `workerd` harness (`vitest-pool-workers` as its own
CI job)"*. The initiative's **AC-004** requires the run to be inside the gate, in
the same run as the rest of it. Reconciling those two is this record's first job,
and the reconciliation is not a compromise: one of them describes a claim about CI,
and the other describes a claim about the gate, and this repository has already
retired a job (`wasm-conformance`) for being the first kind.

### 3.2 What landed

The conformance suite executes on `wasm32-unknown-unknown` under
`wasm-bindgen-test-runner`, against a `node:sqlite`-backed `DurableObjectState`
shim shipped in `crates/happenstance-cloudflare/src/host.rs`, driven by **one row**
in `xtask/src/proof.rs`'s executed-target registry, inside one `cargo xtask ci`.
Eighty-nine rules, plus the fixture-contract cases, the `!Send` probes' `wasm32`
twins, the ES-6 reconstruction, and — since `wf-11-memory-ceiling-falsifier` — the
memory probe.

This is `every-rule-under-workerd`'s finding, and this record's job is to record
which shape won rather than to choose one.

### 3.3 Alternatives that lost on the harness side

| Alternative | Why it lost |
| --- | --- |
| **`vitest-pool-workers` as its own CI job** — the runbook's own wording | Loses to AC-004's *same run as the rest of the gate*. A separate CI job is a claim about CI; nobody's local `cargo xtask ci` checks it, and this repository retired exactly such a job on that argument. |
| **`wasm-bindgen-test-runner` under node with no Durable Object** | The `SqlStorage` binding cannot be satisfied without something shaped like `DurableObjectState`. The harness would compile and assert nothing about the adapter. The shim exists to satisfy that binding and no more. |
| **A probe-gated step with no compensating mandatory assertion** | Loses to the gate's own rule that *a constraint whose only check is skippable is unguarded on every machine that lacks the tool*. The shape that won pairs a probed execution step with a **mandatory** runner-free compensator that fails on an emptied, renamed or `cfg`-ed-away target. |
| **A `workerd`-class runner inside `cargo xtask ci`** | **Not rejected on merit** — escalated as a blocking finding. See §6. |

### 3.4 What the harness does not prove

Stated here because a reader who finds *"every rule green"* will otherwise supply
the missing qualifier themselves, generously. No isolate, no eviction, no
hibernation, no I/O gate, no event loop re-entering the object mid-`await`, and
none of the platform's own storage ceilings. The honest sentence is the one the
crate root already states: every rule executes on `wasm32-unknown-unknown` against
a shim shipped here, and **not** under `workerd`.

Two consequences follow, and both are recorded elsewhere rather than hidden here:
the fixture's three numeric ceilings are this adapter's **declared refusal policy**
seeded from a documented platform cap, not a located physical wall
(`measured-store-limits`); and WF-11's falsifier could not be constructed, because
this runner imposes no memory ceiling at all (`wf-11-memory-ceiling-falsifier`).

---

## 4. ADR-0001 is cited, not lifted — and why the instruction was read and not obeyed

`RUNBOOK.md:4394-4396` asks for ADR-0001's `provisional` marker to be *"formally
retired and this adapter cited"*.

**The marker was already lifted at phase 1**, by `LocalMemoryEventStore`, and
**ADR-0008 records the lift**. `.kb/decisions/0001-async-port-flavours.md` is an
accepted decision atom: its body is immutable, and `redkiln validate --kb` checks
it against `HEAD` on every run. Obeying the instruction literally would mean
editing an accepted atom to record something a later accepted atom already records
— which is the exact failure the immutability rule exists to prevent, arriving
through the front door of a runbook checkbox.

So the discharge is a **citation**, here and in the atom. The reason is written
down because an instruction that invites an edit to an immutable atom will invite
the same edit from the next reader.

What phase 9 supplies is worth stating on its own, because it is the first of its
kind in this workspace: the **bare, `!Send` flavour** of the port is now observed
*under execution*, against a real adapter, on the target the two-flavour design was
paid for — not under a `cargo check`. Until this phase, every claim about that
flavour rested on compilation.

---

## 5. ADR-0009's ES-6 prediction, judged

**ES-6 was settled, not deferred.** `Error` keeps `core::error::Error + 'static` on
both flavours and the stronger property lives in a downstream
`ThreadSafeEventStore` marker. The project's own AC wording — *"the bound added, or
ADR-0009's deferral confirmed"* — predates the atom's acceptance and is corrected
here rather than answered.

What ADR-0009 named was an **asymmetry**: an error type that is `!Send` because it
carries a live JavaScript value would be where the absent bound costs a caller
something. Phase 9 is the first runtime that can produce one.

**The verdict: ADR-0009's decision holds, and this adapter is the evidence for it
rather than the exception to it.** A caller recovers the one fact they must branch
on — constraint violation versus transport fault — from what `append` hands back,
with no `Send + Sync` bound on `Error`.

The artefact is four committed reconstruction tests in
`crates/happenstance-cloudflare/src/lib.rs`'s `es6_reconstruction` module, named in
`xtask/src/proof.rs` so they cannot be renamed or emptied in silence:
`constraint_violation_reaches_the_caller_as_condition_violated`,
`transport_fault_reaches_the_caller_distinguishably`,
`the_distinction_is_reachable_from_outside_the_crate` (which binds the **bare**
flavour and reads only `pub` items), and the negative control
`an_evidence_discarding_classifier_is_rejected`.

**Why this is not a green suite dressed up as an answer.** Every conformance rule
asserts on the success path or on a store-produced `AppendError`, and none reads an
adapter error's contents — so a green suite exists whether stringifying a `JsValue`
loses information or not. That is why the phase's proof artefact demanded a second
half, and the second half was demonstrated capable of failing: two wrong
classifiers were compiled into the real `classify_write` and the suite re-run. The
evidence-discarding shape was rejected by two tests, the flattening shape by three.

**The consequence for §2's mapping**, which is why this sits inside ADR-0023 rather
than beside it: the thrown value is **retained rather than stringified**. That is a
mapping decision, and this verdict is its evidence.

**Not claimed.** `store_error_crosses_a_join_handle` — ES-6's own named rule — is
still unwritten and unowned; it is a testkit change against ADR-0009's marker and
`.kb/open-questions/es-6-names-an-unwritable-rule.md` remains open.

---

## 6. The blocking finding this record must not absorb

The queue row assumed a `workerd`-class runner. There is not one inside
`cargo xtask ci`, and it is escalated rather than quietly re-scoped.

Every other gate tool is either pinned by `rust-toolchain.toml` or `cargo
install`ed at a version resolved from `Cargo.lock`. A `workerd`-class runner —
`wrangler`, `miniflare` or `vitest-pool-workers` — is a different artefact in every
dimension that matters: a Node lockfile this repository does not own, an external
binary with no Windows-native story, and a version nothing in the gate can derive.
The measured cost is recorded in
`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/implementation-report.md`.

Two consequences are named rather than left to be inferred: the fixture's numeric
ceilings are a declared refusal policy rather than located walls, and WF-11's
falsifier is not constructible from this gate. Both are recorded in their own
stories and neither is repaired by this record.

---

## 7. The dependency reversal, and the price it has not paid

The stand-in was justified on the argument that `worker` drags a large
`wasm-bindgen`/`js-sys`/`web-sys` surface and its own licence graph through the
gate *"for no type-checking benefit"*. That argument was true of a crate whose
every body was `todo!()` and false of one that talks to a Durable Object:
`worker::Error` is the real thrown value ES-6 is a claim about, and
`worker::SqlStorage` is the API §2's four properties were modelling. **An
instrument that models the runtime cannot be falsified by the runtime.**

**Measured rather than feared.** The graph gains 40 crates (`http`, `matchit`,
`strum`, `serde-wasm-bindgen`, `wasm-streams`, `web-sys`, `url` and their leaves);
`wasm-bindgen`, `js-sys` and `wasm-bindgen-futures` were already present as
`wasm-bindgen-test`'s transitive dependencies, so those three are new *edges* to
existing *nodes*. `cargo deny check licenses advisories` is green against
`deny.toml`'s unmodified eight-licence allowlist, and `worker`'s declared
`rust-version = "1.75"` sits well under ADR-0029's 1.97.1 floor.

**And one price that is not paid, stated because a silent one is how a guard
dies.** `cargo deny check bans` is **red**: `deny.toml` bans `async-trait` under
ADR-0001 — it injects `+ Send`, which forecloses `wasm32` — and `worker` 0.8.5
depends on it unconditionally, as does `worker-macros`. The ban's own comment says
such a route *"fails until someone decides it should"* be a wrapper.

**That decision is this record's, and it is stated rather than slipped through a
manifest.** What a red ban does **not** mean: `worker` uses `#[async_trait]` for
its own `DurableObject` trait, and no happenstance port gains a bound from it.
`happenstance-core` still declares each port once without a `Send` bound and lets
`trait_variant` derive the second flavour; the substantive guards are unmoved.

Two shapes are available and the atom must choose one and say which:

- **Ratify.** Add `worker` and `worker-macros` to `deny.toml`'s `wrappers` list,
  with the paragraph above written beside the entry so the exception carries its
  reason rather than its convenience.
- **Refuse.** Leave the ban red and record the exception here, accepting that
  `cargo deny check bans` is a step that fails for a known reason — which is a
  standing cost on every run and, historically, how a guard learns to be ignored.

Leaving it undecided while the gate is red is not one of the shapes.

---

## 8. What this record does not touch

No clause is amended and no maturity marker moves. No wire format changes — that is
`replication-identity-and-ingest`'s (HS-P0017), even though WF-11's falsifier was
fired at in this phase. No accepted decision atom's body is edited: ADR-0001,
ADR-0008, ADR-0009, ADR-0011, ADR-0012, ADR-0015 and ADR-0016 are read and cited,
never rewritten. No existing file under `references/adr/` is modified; this is a
new file, because `spec/SPECIFICATION.md` cites line ranges into that directory and
nothing validates it.
