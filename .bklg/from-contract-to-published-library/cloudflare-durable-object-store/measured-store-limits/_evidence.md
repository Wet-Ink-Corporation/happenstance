# Evidence package — the Durable Object store limits

Written for `adr-0023-and-atom-resolutions` (HS-S0058), which is the only story
in this project permitted to write to `.kb/`. Nothing here is an atom and nothing
here was written under `.kb/`; every item is a finding with its derivation
attached, in the terms the receiving story will need to cite it.

## 1. The three numbers, and how each was obtained

| Constant | Declared | Obtained by |
| --- | --- | --- |
| `MAX_EVENT_DATA_LEN` | `Some(1_048_576)` — 1 MiB | Cloudflare's documented 2 MiB Durable Object row cap as the **seed**, halved to leave room for the rest of the row this adapter writes: `event_type` ≤ 255 B, the canonical `tags` blob ≤ 256 KiB at the tag ceiling below, ADR-0014's `origin_store` + `origin_position` + `recorded_at`, and a `metadata` blob the contract does not bound at all. Confirmed accepted and read back byte-for-byte on the executing runtime (probe M2B), and 1,048,577 confirmed refused as `ExceedsStoreLimit { limit: EventDataLen }` |
| `MAX_TAGS_PER_EVENT` | `Some(1_024)` | Measured against **this adapter's** tag storage: probe M3B confirms 1,024 tags produce exactly 1,024 `event_tag` rows at a storage cost of 139,264 bytes, and 1,025 refused as `ExceedsStoreLimit { limit: TagsPerEvent }`. Positioned at 1,024 because at `MAX_TAG_LEN` (255) the canonical blob is then 256 KiB — an eighth of the row cap, which is what keeps the payload figure safe |
| `MAX_EVENTS_PER_BATCH` | `Some(1_024)` | **Not** a bound-parameter cap, and that was the first thing checked: this adapter renders one `INSERT` per event and one per tag, each its own statement. The real bound is that the whole batch runs inside one turn with nothing awaited between rows — which is what makes the compensating discard exact — so it is a statement count. Probe M4B measures 2,055 statements for a 1,024-event batch with one tag each, and 1,025 events refused as `ExceedsStoreLimit { limit: EventsPerBatch }` |

Each clears its floor by a wide margin: 16×, 16× and 8× respectively
(`MIN_SUPPORTED_EVENT_DATA_LEN` 65,536; `MIN_SUPPORTED_TAGS_PER_EVENT` 64;
`MIN_SUPPORTED_EVENTS_PER_BATCH` 128). No sub-floor measurement occurred, so
EC-001's escalation path was never entered.

Reproducible at `experiments/durable-object-limits/`, run twice with
byte-identical output (`results/run-1.txt`, `results/run-2.txt`), against rustc
1.97.1, `wasm-bindgen-test-runner` 0.2.126, Node v24.18.0, adapter commit
`440bbac`.

## 2. The finding ADR-0023 most needs: the physical wall was never located

**Phase A of the probe found no per-value refusal anywhere in the searchable
range.** Payloads doubled to 8 MiB, 16,384 `event_tag` rows for one event, and
8,192 consecutive event inserts in one turn were all accepted and all read back.

The executing runtime is real SQLite reached through `worker`'s real
`wasm-bindgen` bindings, but the host is a Node process rather than `workerd`, and
it does not enforce the Durable Object platform's documented caps. So the three
declared numbers are **not** search results, and the fixture, the crate
documentation and the experiment's README all say so in those words.

What they are is the adapter's own **refusal policy**: `check_ceilings` runs
before any SQL is issued, which is the design `durable-object-write-path` already
took and the reason a refusal here can name *which* ceiling was crossed. CF-40
requires only that a declared value is accepted and one more refused. It does not
require the declaration to be the physical maximum, and an unstable exact maximum
is how a green run becomes a flaky one — which is exactly the case
`measured-store-limits`' own spec anticipated in its clarification 5.

**This is the one thing a `workerd`-class runner would change**, and it is worth
recording as the standing residual: phase A would then produce real numbers, and
the declarations would either be confirmed as conservative or moved.

## 3. CF-40's `[PROVISIONAL]` falsifier did **not** fire

CF-40's marker names *a real adapter whose ceiling is not a constant* as its
falsifier, and the Cloudflare adapter as one of three instruments. Probe M5 is the
direct test: a 2 MiB row is accepted both on a fresh object and on one already
holding a megabyte. The boundary does not move with what is stored, so the
cumulative cap `SqlError::StorageLimitExceeded` reports is **not** the per-value
boundary and was not mistaken for one. No `None` declaration and no falsification
finding is owed on that axis.

## 4. CF-39's verdict: `SUPPORTED`, with the mechanism stated

This is the **first fixture in the workspace** to claim `MID_BATCH_FAULT`, so
`arming_a_mid_batch_fault_makes_the_append_fail` and
`append_is_atomic_under_a_mid_batch_fault` have now run for real somewhere — new
coverage for the *conformance suite*, not only for this adapter.

**The mechanism**, in the terms CF-39 requires a claimant to state: the Durable
Object host is armed to throw a real `Error` on the *k*-th statement whose text
contains `INSERT INTO event (` — the adapter's own per-event insert. The trailing
paren is load-bearing: `INSERT INTO event` alone is a prefix of
`INSERT INTO event_tag`, so without it the fault lands in the tag pass rather than
between two event rows.

**Why the store cannot absorb it.** A Durable Object rejects transaction control
through `sql.exec()`, so there is no `SAVEPOINT`; the turn's implicit transaction
commits when the handler returns normally, which converting a throw into `Err(…)`
does. The adapter therefore undoes a failed batch itself by deleting the positions
it had assigned — and the compensating `DELETE FROM event …` is deliberately not
matched by the arming substring, so the discard runs.

**The negative control was performed.** With the arm removed and the override left
in place — the registered `NoopFaultFixture` shape —
`arming_a_mid_batch_fault_makes_the_append_fail` goes red with:

```text
CF-39: a fixture declaring `MID_BATCH_FAULT` supported MUST, when armed at
k < events.len(), cause the write of the k-th event to fail inside the store's own
write path, by a mechanism the store cannot absorb, so that the append returns
`Err`. This one armed a fault at row 2 of a three-event batch and the append
succeeded … Got Ok(SequencePosition(6))
```

Restored, both rules pass.

## 5. CF-17's verdict: `REOPEN` survived contact

Declared `SUPPORTED` by `durable-object-host-and-fixture` on the trait's own
Durable-Object reasoning, and confirmed here against the executed suite rather
than overturned. `acknowledged_writes_survive_a_reopen`,
`reopened_store_does_not_reissue_an_event_id` and `recorded_time_survives_a_reopen`
all **Ran** and passed — the first time any of the three has executed anywhere in
this workspace, because every other fixture declines the capability.

The mechanism is `DurableObjectHost::storage()`: a second binding taken off the
same retained `state`, which discards the previous binding's cursor generation and
its cached store-id incarnation and touches no row. EC-006 was never entered.

## 6. The CF-40 ownership finding

`.kb/open-questions/cf-40-fixture-limits-ownership.md` records that ADR-0015 both
claims and disclaims the clause, that ADR-0012 is the other claimant by adjacency,
and that phase 8 is the forcing phase.

**The finding this story contributes is narrower than the question, and it is a
narrowing rather than an answer.** Declaring three numbers here was **not** blocked
by not knowing which document owns CF-40, because the clause's normative text is
identical under either reading: the three `Option<usize>` constants, the
both-directions promise, and the `ExceedsStoreLimit` channel do not change
depending on which ADR is held to own them. What that leaves is the atom's own
sub-question 2 — *whether the fixture contract has one owning document at all* —
which is a question about the shape of the decision record rather than about the
clause.

**Coordination check.** `sqlite-durable-store` (HS-P0012) merged one position ahead
and is closed (`df90dcc`, *close HS-P0012 sqlite-durable-store*). No `.kb/` atom
resolving CF-40's ownership exists in the tree at the time of writing —
`.kb/open-questions/cf-40-fixture-limits-ownership.md` is unchanged — so the answer
has **not** been minted twice and HS-S0058 is free to mint it once, citing this
finding. EC-008 was checked and did not apply.

## 7. Residuals handed on, not acted on

Two `[PROVISIONAL]` markers in `spec/SPECIFICATION.md` carry falsifier notes that
this slice makes false. They are quoted here verbatim and **not edited**: a
provisional marker's falsifier list belongs to the clause's owning decision, and
for CF-40 that owner is the very thing the open question has not settled.
`cargo xtask spec-trace` is green either way.

- **CF-40**, `spec/SPECIFICATION.md` — the sentence to the effect that *no adapter
  has stated a ceiling yet*. `CloudflareFixture` now states all three.
- **CF-39**, `spec/SPECIFICATION.md` — the sentence to the effect that *no adapter
  has armed a fault yet*. `CloudflareFixture` now arms one, with the mechanism
  stated.

Also handed on, from the slice as a whole: the runner shape that worked
(`wasm-bindgen-test-runner` over Node with the `node:sqlite`-backed
`DurableObjectState` shim — no `wrangler`, no standalone `workerd`), the fact that
all 89 event-store rules pass against this adapter with **no** wasm-only
divergence, and the two `xtask` seam repairs recorded in the slice's
implementation reports.

## 8. What was not done here

No `.kb/` file was created, edited or deleted; `git diff --name-only` over this
slice shows no `.kb/**` path. No clause text and no `[PROVISIONAL]` marker was
edited. No conformance rule and no mutant was added — the `None`-declaration
defect is in a *fixture's declaration*, which no store can fail, so `CLAUDE.md`'s
rule forbids a suite rule for it and the guard is the adapter-local
`the_three_store_limits_are_measured_not_defaulted` plus the CF-29-shaped changelog
entry.
