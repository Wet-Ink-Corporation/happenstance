# ADR-queue escalation — the three durability markers, and what an ADR would have to decide

**Not an ADR.** This is the note AC-005 requires: a maturity marker's *level*
moves in an ADR pass, never as a side effect of a code change
(`.kb/open-questions/es-7-and-vt-9-provisional-markers.md:14-15`; `CLAUDE.md`,
*Open questions, deliberately unresolved*). This story recorded a **verdict**
inside each marker's existing level and corrected the sentences that had become
factually false. Nothing below was decided here.

Evidence for all three is the same run and it is now in the tree:
`cargo test -p happenstance-sqlite --test conformance` — 90 passed, 0 failed —
with `acknowledged_writes_survive_a_reopen`, `recorded_time_survives_a_reopen` and
`reopened_store_does_not_reissue_an_event_id` all `Ran` and green across a real
close-and-reopen of a real file, and both `MID_BATCH_FAULT`-gated rules reported
as `SKIP` carrying `SqliteFixture`'s own stated reason.

## 1. ES-35 — durability, `[PROVISIONAL]`

**Verdict written (level unchanged):** the *adapter* far end is now supplied and
the *fault* far end is not. The falsifier is restated as **a store that loses a
write to a fault rather than to an instruction**, and the clause's closing
paragraph no longer says the axis has "nothing at the other end", because that
half became false the moment `SqliteFixture` ran the reopen rules over a file.

**What an ADR would have to decide, if anyone proposes promoting it.** Whether
"durability is claimed" can be frozen on a far end that is still *modelled*.
Every failing control on this axis — `LosingFixture`, and now
`RestampingFixture` — lives in one process and loses its data to a pointer swap.
An adapter fixture that arms a real fault against a real medium (a killed
process, an `fsync` that lied) does not exist in the workspace and is not on any
project's charter. Promoting ES-35 before one exists would freeze a clause whose
`Rejects:` paragraph names three real adapters and whose registry rejects none of
them.

**Escalate to:** the runbook's ADR queue, at whichever phase first proposes an
adapter fixture that can fail *unarmed*.

## 2. CF-17 — the `REOPEN` capability, `[PROVISIONAL]`

**Verdict written (level unchanged):** **the rule shape is confirmed.** CF-17's
falsifier was *a legitimate adapter that is durable and cannot express even a
reopen through this contract*. `happenstance-sqlite` is durable and file-backed,
and it expressed the reopen with nothing added to the contract — close every
`rusqlite::Connection`, checkpoint the write-ahead log with `TRUNCATE`, leave the
file alone. The clause's own pre-emption of the `restart` split therefore held
against the first adapter that could have forced it. The sentence naming
`DurableFixture` as *the only fixture in the workspace that supplies this
capability* was corrected; there are two now, and the second is the first over a
medium outside the process.

**What an ADR would have to decide.** Whether one adapter is the *spread* that
freezes a capability. CLAUDE.md's own rule says it is not — *a port is only as
well-designed as the spread of what implements it* — and the two implementations
CF-17's marker was written against have not answered. A Durable Object's "reopen"
is storage surviving an isolate eviction; a one-shot HTTP client has no
connection to close at all. Either could still show that the weaker operation
needs grading.

**Escalate to:** the runbook's ADR queue, gated on `cloudflare-durable-object-store`
(HS-P0013) and `postgres-and-neon-stores` (HS-P0014). Promotion is plausible after
either; it is not this story's to take.

## 3. CF-14 — the durability rule's obligation, `[DEFERRED]`

**Verdict written (level unchanged):** the deferral is **confirmed and narrowed**.
What is deferred is not the rule — that landed early, as the named exception the
clause records — but the *experiment*: whether one `reopen` shape serves rusqlite,
a Durable Object and a one-shot HTTP client, or whether "durable" needs grading.
One of the three has now answered with the one shape and needed no grading. The
falsifier is narrowed to the two that have not, with their owning projects named
in the marker.

**What an ADR would have to decide.** Nothing yet. The deferral is doing its job:
it names an experiment, one third of it has run, and the marker now says which
two thirds remain and who owns them. The ADR arrives when the second or third
implementation answers — and if either answers *no*, the decision is whether
`REOPEN` becomes graded (a capability with levels) or whether the adapter that
cannot express it declines the capability and says so, which is CF-18's existing
mechanism.

**Escalate to:** the runbook's ADR queue, gated on the same two projects as CF-17.

## What this story deliberately did not do

- It did not move any marker's level. The clause-status rows still read
  `ES-35 PROVISIONAL`, `CF-14 DEFERRED`, `CF-17 PROVISIONAL`, and
  `cargo xtask spec-trace` reports the same clause census as before the edits —
  `201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE)`, and
  `389 citations checked` with no problems found.
- It did not touch VT-9's own provisional marker, which is phase 9's
  (`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`).
- It did not author an ADR or a `.kb/decisions/` atom. Recording the gap is the
  deliverable; deciding it is the runbook's ADR pass.
