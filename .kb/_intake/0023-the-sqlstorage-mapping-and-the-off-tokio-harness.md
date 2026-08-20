# ADR-0023 — the `SqlStorage` mapping and the off-tokio harness, settled by one body of evidence

**Staged for `/redkiln:kb-ingest`. Not an atom.** This is the raw material for the
decision atom the ADR queue carries as row **0023**
(`RUNBOOK.md:302`) and for the long-form record at
`references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`.
Nothing here is settled until the wave writes it; nothing here may be merged into
an **accepted** decision atom's body — the only two legal shapes for this content
are a new atom or a superseding one.

**Intended layer:** `.kb/decisions/`, `kind: decision`, `authority_tier: decision`,
`adr_id: ADR-0023`, `phase: 9`, `supersedes: null`, `superseded_by: null`.
**Long-form record:** `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`
(new; no existing file under `references/adr/` is touched).

---

## The one question, and why its title carries an "and"

**The question.** *How does an event store map onto a Cloudflare Durable Object's
`SqlStorage`, and what harness proves it — given that the whole point of this
adapter is that neither `tokio` nor a thread is available to it?*

The title conjoins the mapping and the harness, which
`.kb/playbooks/one-decision-per-adr-title.md` treats as a smell. That playbook
states its own exception: *"It stops being worth the split when both halves are
settled by the same evidence, in which case the conjunction is describing one
decision with two consequences."* This is that case, and the single body of
evidence is nameable in one clause: **the conformance suite executing against the
real `worker` bindings, off tokio, inside one `cargo xtask ci`.** Split into two
atoms, each would cite the same run and neither would be readable without the
other — the mapping's every claim is a claim about what that run observed, and the
harness's only justification is that it is what made the mapping observable at all.

Say this in the atom explicitly, because the next reader will check the
conjunction against the playbook and should find the exception already cited.

---

## Consequence 1 — the mapping, and the four properties it had to survive

The stand-in this adapter carried for two phases modelled four properties. They
are now checked against the real bindings rather than asserted about a model
(`crates/happenstance-cloudflare/src/sql_storage.rs:1-30`):

1. **`exec` is synchronous.** `worker::SqlStorage::exec` is a plain `fn` returning
   `Result<SqlCursor>` — no future, no connection to acquire, because the storage
   is co-located with the object. This is the one storage in the workspace for
   which `EventStore::read` **not** being `async` is free rather than awkward,
   which is ADR-0001's two-trait design being paid off rather than tolerated.
2. **The cursor is not a snapshot.** A cursor can technically be held across an
   `await` and does not provide a stable view of its results. A lazy stream *is* a
   cursor held across awaits, so this is a capability limit rather than a type
   error. It is preserved rather than papered over: `SqlError::CursorInvalidated`
   reports it, and ADR-0011's ceiling-and-page mechanism is what keeps a cursor
   off a suspension point in the read path.
3. **Everything is `!Send` and `!Sync`.** `worker` declares
   `unsafe impl Send for SqlStorage {}` and the same for its cursor
   (`worker-0.8.5/src/sql.rs`), so holding either bare would hand this crate
   `Send`-ness through an escape hatch its own lint policy denies it. Both are held
   through an `Rc`, which is `!Send` for every payload — and the four `!Send`
   probes, including `the_probe_is_not_vacuous`, are what stop that becoming a
   claim nobody checks.
4. **The object is single-threaded and re-entrant.** Shared state is reached
   through a `RefCell` that is *tried*: a second statement issued while the first
   still holds it **reports** rather than panicking, which
   `sql_storage::tests::reentrant_borrow_is_reported_not_panicked` asserts. A plain
   `borrow_mut` takes the whole object down.

**Alternatives that lost, on the mapping side, each with its reason:**

- **The ceiling capture.** Rejected: reading a `max(position)` *after* the cursor
  opens cannot bound a cursor that is not a snapshot. ADR-0011's ceiling-and-page
  takes the ceiling **first** and pages under it, which is what makes a lazy read
  expressible on a storage that will invalidate a held cursor.
- **`Query::index_arms()` decomposition in the contract.** Rejected here for the
  same reason ADR-0022 rejected it one adapter over: it does not exist in
  `happenstance-core`, the decomposition stays adapter-private, and one adapter
  wanting it is not grounds to widen the contract.
- **`JsThrow` versus `StringifiedThrow`.** The thrown value is retained rather than
  stringified at the boundary. Stringifying early is what would have made ES-6's
  question unanswerable, because the one fact a caller must branch on — constraint
  violation versus transport fault — is recoverable only while the thrown value is
  still a value. See the ES-6 cluster, staged separately.
- **Keeping the hand-written stand-in instead of taking a `worker` dependency.**
  Reversed, on a measurement rather than on taste, and the reversal's own price is
  recorded rather than hidden: the graph gains 40 crates, `cargo deny check
  licenses advisories` stays green, and **`cargo deny check bans` is red** because
  `worker` 0.8.5 depends unconditionally on `async-trait`, which `deny.toml` bans
  under ADR-0001. That finding stands rather than being wished away by widening the
  `wrappers` list — see *The one thing this decision must not do quietly* below.

---

## Consequence 2 — the harness, and the winning shape is a finding rather than a choice

**What the runbook queued** was *"the `SqlStorage` mapping and the `workerd`
harness (`vitest-pool-workers` as its own CI job)"*. The initiative's own AC-004
requires the run to be **inside the gate, in the same run as the rest of it**.
Those two are not compatible, and reconciling them is this decision's first job.

**What actually landed**, and it is `every-rule-under-workerd`'s finding rather
than this record's choice: the conformance suite executes on
`wasm32-unknown-unknown` under `wasm-bindgen-test-runner`, against a
`node:sqlite`-backed `DurableObjectState` shim shipped in
`crates/happenstance-cloudflare/src/host.rs`, driven by **one row** in
`xtask/src/proof.rs`'s executed-target registry, inside one `cargo xtask ci`.

**Alternatives that lost, with the reason each lost:**

- **`vitest-pool-workers` as its own CI job** — the runbook's own wording. Loses to
  the initiative's *same run as the rest of the gate*: a separate CI job is a claim
  about CI, not about the gate, and this repository has already retired one such
  job (`wasm-conformance`) precisely because a claim the gate cannot make is a
  claim nobody's local run checks.
- **`wasm-bindgen-test-runner` under node with no Durable Object at all** — loses
  because the `SqlStorage` binding cannot be satisfied without something shaped
  like `DurableObjectState`; the harness would compile and assert nothing about the
  adapter. The shim exists to satisfy exactly that binding and no more.
- **A probe-gated step with no compensating mandatory assertion** — loses to the
  gate's own stated rule that *a constraint whose only check is skippable is
  unguarded on every machine that lacks the tool*. The shape that won pairs a
  probed execution step with a **mandatory** runner-free compensator that fails on
  an emptied, renamed or `cfg`-ed-away target.
- **A `workerd`-class runner inside `cargo xtask ci`** — the shape the queue row
  assumed, and it is **not rejected on merit**. It is an escalated blocking
  finding: `workerd` is an external binary with no Windows-native story, versioned
  by a Node lockfile this repository does not own, where every other gate tool is
  either rustup-pinned or `cargo install`ed from `Cargo.lock`. The cost was
  measured rather than feared and is recorded in
  `.bklg/…/every-rule-under-workerd/implementation-report.md`.

**What the harness therefore does not prove, stated in the atom rather than left
to be discovered.** No isolate, no eviction, no hibernation, no I/O gate, no event
loop re-entering the object mid-`await`, and none of the platform's own storage
ceilings. The honest sentence — the one the crate root already states — is that
every rule executes on `wasm32-unknown-unknown` against a shim shipped here, and
**not** under `workerd`.

---

## Consequence 3 — ADR-0001 is **cited, not lifted**, and the record says why

`RUNBOOK.md:4394-4396` instructs that ADR-0001's `provisional` marker be *"formally
retired and this adapter cited"*. **Do not obey that instruction.** The marker was
already lifted at phase 1 by `LocalMemoryEventStore`, and ADR-0008 records the
lift. `.kb/decisions/0001-async-port-flavours.md` is accepted and immutable, and
`redkiln validate --kb` checks it against `HEAD`.

What phase 9 supplies is the **real-runtime evidence behind an already-accepted
decision**, and the discharge is therefore a citation *inside* ADR-0023 and its
long-form record. Write the reason down: an instruction that invites an edit to an
immutable atom will invite the same edit from the next reader unless the record
says it was read and not obeyed.

The evidence itself is worth stating, because it is the first of its kind: the
bare, `!Send` flavour of the port is now observed **under execution** against a
real adapter on the target the two-flavour design was paid for, rather than under
a `cargo check`. Eighty-nine conformance rules, plus the fixture-contract cases and
the four `!Send` probes' `wasm32` twins.

---

## The one thing this decision must not do quietly

**`cargo deny check bans` is red, and widening `deny.toml`'s `wrappers` list is
this decision's to ratify or refuse — not a manifest's to slip through.**
`deny.toml` bans `async-trait` because ADR-0001 forbids it: it injects `+ Send`,
which forecloses `wasm32`. `worker` 0.8.5 and `worker-macros` depend on it
unconditionally.

What it does **not** mean, since a red ban invites the wrong inference: `worker`
uses `#[async_trait]` for its own `DurableObject` trait, and no happenstance port
gains a bound from it. `happenstance-core` still declares each port once without a
`Send` bound and lets `trait_variant` derive the second flavour.

The finding is recorded against `worker-binding-layer`'s AC-008 and escalated
here. Two shapes are available and the atom must choose one and say which:
**ratify** a `wrappers` entry naming `worker` and `worker-macros` with the argument
above written into `deny.toml` beside it, or **refuse** and let the ban stay red
with the exception recorded. Either is a decision; leaving it undecided while the
gate is red is not.

---

## What this decision does **not** touch

- No clause is amended and no maturity marker moves. CF-39, CF-40 and WF-11 are
  discharged or resolved elsewhere; CF-23, ES-6 and ES-9 are honoured by the
  upstream stories rather than changed. Nothing `[FROZEN]` is approached.
- No wire format changes. That is `replication-identity-and-ingest`'s (HS-P0017).
- No accepted decision atom's body is edited. ADR-0001, ADR-0008, ADR-0009,
  ADR-0011, ADR-0012, ADR-0015 and ADR-0016 are read and cited, never rewritten.
