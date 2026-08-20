---
item: "HS-S0055"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — The fixture's numeric limits are measurements, not guesses

## Findings Ledger

**Outcome: eleven of eleven ACs satisfied. The Cloudflare conformance row now
prints zero `SKIP` lines — 89 of 89 rules Ran and passed. Nothing blocked,
nothing deferred, no conformance rule or mutant added, no clause amended, no
`.kb/` write.** One finding about the limits of the measurement itself is flagged
for ratification, and it is the most important sentence in this report.

| AC | Result | Proved by | Mounted into |
| --- | --- | --- | --- |
| AC-001 | satisfied | `dcb_conformance_wasm::append_reports_exceeded_store_limits` (payload arm) executing in the gate; probe M2B: 1,048,576 accepted and read back, 1,048,577 refused as `ExceedsStoreLimit(EventDataLen)` | `tests/support/mod.rs:247`; enforced at `src/event_store.rs:228`/`:292`/`:418` |
| AC-002 | satisfied | same rule, tag arm; probe M3B: 1,024 tags → 1,024 `event_tag` rows, 139,264 bytes; 1,025 refused as `ExceedsStoreLimit(TagsPerEvent)` | `tests/support/mod.rs:258` |
| AC-003 | satisfied | same rule, batch arm; probe M4B: 2,055 statements for 1,024 events; 1,025 refused as `ExceedsStoreLimit(EventsPerBatch)` | `tests/support/mod.rs:270` |
| AC-004 | satisfied | **the absence**: no `NO_STORE_LIMITS` line anywhere in the row's output; `tests/fixture_contract.rs::the_three_store_limits_are_measured_not_defaulted`. Red beat: it failed naming `MAX_EVENT_DATA_LEN` | the three constants |
| AC-005 | satisfied | `::no_declared_ceiling_is_below_its_floor` (16×, 16×, 8×); the three guaranteed-minimum rules green in the same run. No sub-floor measurement occurred | the three constants |
| AC-006 | satisfied | `experiments/durable-object-limits/` — README, `tests/boundaries.rs`, `results/run-1.txt` and `run-2.txt` byte-identical; outside the workspace by a bare `[workspace]` table, so never in the gate | the experiment directory |
| AC-007 | satisfied | probe M5: a 2 MiB row accepted on a fresh object **and** on one holding a megabyte, so the boundary is a constant. CF-40's falsifier tested and **not** fired | `_evidence.md` §3 |
| AC-008 | satisfied | `MID_BATCH_FAULT = SUPPORTED` with the mechanism stated; both fault rules Ran and passed; **negative control performed** — arm removed, rule went red naming the `NoopFaultFixture` shape, then restored | `tests/support/mod.rs:218`, `:298` |
| AC-009 | satisfied | `REOPEN` confirmed, not overturned: all three reopen rules Ran and passed, the first time any has executed anywhere | `tests/support/mod.rs:180`, `:281` |
| AC-010 | satisfied | CF-29-shaped `CHANGELOG.md` entry (`lint-changelog` green); the VT-21 limits table in `src/lib.rs`; `cargo doc` clean | `CHANGELOG.md`, `src/lib.rs` |
| AC-011 | satisfied | `_evidence.md` §1–§8, including the CF-40 ownership finding and the coordination check; `git status` shows no `.kb/**` path | this story's folder |

### The finding that matters most, and it is about the measurement

**Phase A of the probe found no wall.** Payloads to 8 MiB, 16,384 tag rows for one
event, and 8,192 consecutive inserts in one turn were all accepted and all read
back. The executing runtime is real SQLite reached through `worker`'s real
bindings, but the host is a Node process rather than `workerd`, and it does not
enforce the Durable Object platform's documented caps.

So the three declared numbers are **not search results**, and nothing in this
slice claims otherwise. They are the adapter's own refusal policy — documented
platform cap minus this adapter's measured overhead, confirmed accepted on the
executing runtime — enforced by `check_ceilings` before any SQL is issued, which
is what lets a refusal name *which* ceiling was crossed. CF-40 requires a declared
value to be accepted and one more refused; the spec's own clarification 5 blesses
a conservative stated ceiling over an unstable exact one, and this is that case.

That reading is written where it cannot be missed: the fixture's doc comments, the
crate documentation's limits table, `experiments/durable-object-limits/README.md`,
and `_evidence.md` §2. A `workerd`-class runner is the one thing that would change
it, and both directions of change are loud rather than silent, because
`append_reports_exceeded_store_limits` fails in one direction or the other.

### What is new coverage for the suite, not just for this adapter

`MID_BATCH_FAULT` is claimed here for the **first time in the workspace**, so
`arming_a_mid_batch_fault_makes_the_append_fail` and
`append_is_atomic_under_a_mid_batch_fault` have now run for real somewhere. The
mechanism is stated as CF-39 requires — a real `Error` thrown on the *k*-th
`INSERT INTO event (` statement, which the adapter cannot absorb because a Durable
Object rejects transaction control through `sql.exec()` and the adapter undoes the
batch itself. The negative control was performed and the rule went red on the
`NoopFaultFixture` shape before it went green on the real one.

The three `REOPEN` rules are the same story one capability over: every other
fixture in the tree declines the capability, so until this slice they were skips
everywhere.

### For the reviewer to ratify

1. **The declarations are policy, not discovery** — see above. If the reviewer's
   reading of CF-40 is that a declared ceiling must be a *located* wall, this
   story's answer would have to become `None` for all three, and AC-004's
   observable would be unreachable on this host. The spec's clarification 5 is why
   it was not read that way.
2. **`Ceilings::MEASURED` is now what `CloudflareEventStore::new` carries**, so the
   ceilings are the adapter's, not the fixture's. That is deliberate: a ceiling a
   *fixture* declares must be one the *adapter* keeps, and wiring it through the
   `#[cfg(test)]` `with_ceilings` seam would have made the promise true only under
   test. `durable-object-write-path` left `Ceilings` as a seam for exactly this.
3. **One `#[allow(dead_code)]`** on `issued_statements`, with its reasoning at the
   attribute: a `tests/support` module is compiled once per declaring target, so
   the method is live in one and dead in the other from one source, and `expect`
   would fire in the target where it is used.
4. **Seven more `standards/rust/` citations repointed** (third time in this slice).
   Only line numbers; no normative text. A `file:line` anchor into a
   heavily-documented crate root is a maintenance edge worth naming.

### Not claimed

No new conformance rule and no new mutant — the `None`-declaration defect lives in
a fixture's *declaration*, which no store can fail, and `CLAUDE.md` forbids a rule
nothing can fail. CF-40's ownership is **narrowed and handed on**, not minted:
`.kb/open-questions/cf-40-fixture-limits-ownership.md` is unchanged, and
`sqlite-durable-store` (HS-P0012) closed at `df90dcc` without minting an answer, so
`adr-0023-and-atom-resolutions` can mint it once and cite this story's finding —
that declaring the numbers was not blocked by the ownership contradiction, because
the clause text is identical under either reading.
