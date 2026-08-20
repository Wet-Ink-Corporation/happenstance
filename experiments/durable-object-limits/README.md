# The Durable Object store-limit measurements

`CloudflareFixture` states three numbers — `MAX_EVENT_DATA_LEN`,
`MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH` — and CF-40 makes each of them a
promise checked at **both** ends on every gate run: exactly that many accepted,
one more refused as `AppendError::ExceedsStoreLimit` naming the matching
`StoreLimit`, with nothing of the refused value left in the log.

This directory is where those numbers came from, kept so that the next person can
re-derive them without asking anybody. It exists because the alternative — three
literals in a fixture with a doc comment saying "measured" — is a label rather
than a citation, and six months from now it is indistinguishable from a guess.
The enforcing constant was in fact called `Ceilings::MEASURED` until 2026-08-20,
which is that failure arriving from inside this very work: the name told a reader
who never opened this file the one thing *The finding* below says is not true. It
is `Ceilings::DECLARED` now.
That is the failure [`experiments/wire-format`](../wire-format/README.md) was
written about, one measurement over.

## Running it

```console
$ cd experiments/durable-object-limits
$ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
    cargo test --target wasm32-unknown-unknown -- --nocapture
```

It needs the same two things every `wasm32` run in this repository needs, and
both fail confusingly when missing: a `wasm-bindgen-test-runner` matching the
locked `wasm-bindgen` exactly, and Node 22.5 or newer, because the Durable Object
host reaches `node:sqlite` through `process.getBuiltinModule`.

**This is not a crate the workspace knows about.** `Cargo.toml` opens with a bare
`[workspace]` table, so cargo does not walk up and adopt it as a member; the root
`members` list is unchanged, the root `Cargo.lock` is never touched, and nothing
here runs under `cargo xtask ci`. CLAUDE.md's repository map is what that is
honouring — `experiments/` is *"measurements. reproducible, and not in the gate."*

What the gate checks on every invocation is the **promise**, not the search:
`append_reports_exceeded_store_limits` against the declared constants. Re-deriving
a boundary on every CI run would make the gate's outcome depend on a search rather
than on a declaration, which is the difference between a stable red and a flaky
one.

## Two phases, and running only one of them is how this goes wrong

**Phase A — where the runtime stops.** Statements issued through
`SqlStorage::exec` against the schema `CloudflareEventStore::migrate` created,
with the adapter's own ceiling check deliberately bypassed. Answers *is there a
wall, and where*.

**Phase B — where the adapter stops.** The same shapes through
`CloudflareEventStore::append`, which refuses above its declared ceilings before
issuing any SQL. Answers *is the declaration where it claims to be, and does the
refusal name the right `StoreLimit`*.

The first cut of this probe had only phase B — and it stopped being a measurement
the moment the adapter declared its ceilings. It dutifully reported 1 MiB, 1,024
and 1,024: the declaration, read back to itself. A probe that can only rediscover
the number it was given has the reproducible *form* of a measurement without the
thing itself.

## What was measured

Recorded verbatim in [`results/`](results/), from two consecutive runs whose
output is byte-identical.

| Environment | |
| --- | --- |
| rustc | 1.97.1 (8bab26f4f 2026-07-14) |
| `wasm-bindgen-test-runner` | 0.2.126 |
| Node | v24.18.0 |
| Adapter commit | `440bbac` (`every-rule-under-workerd`) |
| Host | `crates/happenstance-cloudflare/src/host.rs` — a `DurableObjectState`-shaped JS object over `node:sqlite`, reached through `worker`'s real bindings |

**M1 — the row's cost besides its payload.** `databaseSize` does not move at all
for one event carrying a 247-byte `event_type` and eight tags (28,672 → 28,672:
the first page has room), and moves by 139,264 bytes for 64 KiB of payload.
`databaseSize` is page-granular, so both are read as bounds rather than as byte
counts, and every declaration below treats them that way.

**M2A — the runtime's payload wall: not found.** Payloads doubled from 64 KiB to
8 MiB were all accepted and all read back byte-for-byte. No refusal at or below
four times the documented row cap.

**M2B — the adapter's declared boundary, kept.** 1,048,576 bytes accepted and
read back as 1,048,576; 1,048,577 refused as
`ExceedsStoreLimit { limit: EventDataLen }`.

**M3A — the runtime's tag wall: not found.** 16,384 `event_tag` rows for one
event, written directly, all accepted.

**M3B — the adapter's declared boundary, kept.** 1,024 tags accepted, producing
exactly 1,024 `event_tag` rows at a storage cost of 139,264 bytes; 1,025 refused
as `ExceedsStoreLimit { limit: TagsPerEvent }`.

**M4A — the runtime's batch wall: not found.** 8,192 consecutive event inserts in
one turn, all accepted.

**M4B — the adapter's declared boundary, kept.** A 1,024-event batch with one tag
each is accepted and issues **2,055** statements; 1,025 events refused as
`ExceedsStoreLimit { limit: EventsPerBatch }`.

**M5 — the boundary is a constant.** A 2 MiB row is accepted both on a fresh
object and on one already holding a megabyte. CF-40's `[PROVISIONAL]` falsifier —
*a real adapter whose ceiling is not a constant* — did **not** fire.

## The finding, stated rather than buried

**On this host, no per-value wall exists inside the searchable range.** The
executing runtime is real SQLite reached through `worker`'s real bindings, but it
is a Node process rather than `workerd`, and it does not enforce the Durable
Object platform's documented caps. So the three declared numbers are *not* search
results, and this README will not pretend they are.

What they are is the adapter's own **declared refusal policy** — the phrase the
fixture, the crate documentation, `CHANGELOG.md`, the evidence package and the
enforcing constant `Ceilings::DECLARED` all use, deliberately in the same words —
derived as *documented platform cap minus this adapter's measured overhead* and
confirmed accepted on the executing runtime:

| Constant | Value | Derivation |
| --- | --- | --- |
| `MAX_EVENT_DATA_LEN` | 1,048,576 (1 MiB) | Half the documented 2 MiB row cap. The other half is the rest of the row: `event_type` ≤ 255 B, the canonical `tags` blob ≤ 256 KiB at the tag ceiling below, ADR-0014's `origin_store` + `origin_position` + `recorded_at`, and a `metadata` blob the contract does not bound at all. 16× the `MIN_SUPPORTED_EVENT_DATA_LEN` floor |
| `MAX_TAGS_PER_EVENT` | 1,024 | At `MAX_TAG_LEN` (255) the canonical blob is then 256 KiB — an eighth of the row cap, which is what keeps the payload figure above safe. Measured cost: exactly one `event_tag` row per tag. 16× the `MIN_SUPPORTED_TAGS_PER_EVENT` floor |
| `MAX_EVENTS_PER_BATCH` | 1,024 | Not a parameter cap: one `INSERT` per event and one per tag, each its own statement. The bound is that the batch runs inside **one turn with nothing awaited between rows**, which is what makes the compensating discard exact — so it is a statement count, measured at 2,055 for 1,024 events. 8× the `MIN_SUPPORTED_EVENTS_PER_BATCH` floor |

CF-40 requires only that a declared value is accepted and one more refused; it
does not require the declaration to be the physical maximum, and an unstable exact
maximum is how a green run becomes a flaky one. These are **stated ceilings**, and
saying so is part of the declaration rather than an apology for it.

**Where this needs re-running.** Two events invalidate the table above and both
are loud rather than silent, because `append_reports_exceeded_store_limits` fails
in one direction or the other: a schema change in `durable-object-write-path`'s
file (a wider column, a different tag storage, a different batch rendering), or a
`workerd`-class runner becoming available — which would turn phase A from *not
found* into a real number, and is the one measurement this host genuinely cannot
supply.
