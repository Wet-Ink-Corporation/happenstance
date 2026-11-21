# Review follow-ups, 2026-11-18

## Goal
Four small follow-ups from tonight's reviews, one commit each, on
`claude/project-thread-qkv7ws-followups` (base 65a5513). No push.

## Items
1. Scope F1: `.kb/reference/spec-trace-has-suite-family-switch.md:84` cites
   `xtask/src/spec_trace.rs:3853-3869`; the fn runs 3854-3870 now (fn line to closing
   brace). Line 81's `3838-3845` checked: correct.
2. Scope F5 / PERF-02: `plan_read` dedup is O(n^2). `QueryItem` derives only
   `Debug, Clone, PartialEq, Eq` (no `Hash`/`Ord`; adding them is an H-16 API change,
   not taken). Its parts do: `EventType: Hash + Eq`, `Tags: Hash + Eq` (derived), and
   `QueryItem`'s `PartialEq` is field-wise over exactly `(types, tags)`, both readable
   through `types()` / `tags()`. So key a `HashSet<(&[EventType], &Tags)>` beside the
   Vec: O(n), first-seen order kept by the Vec.
   Order observer: nothing asserts on item order in a read query (memory store
   filters by membership); keep first-seen regardless.
   Test gap: no test pins that lanes sharing a scope put their item in the read once.
   Add `Instrumented` query recording + `views_sharing_a_scope_ask_for_its_item_once`
   (characterisation; must fail against a no-dedup `plan_read`, checked by mutation).
3. #379 F1: tie `COMPENSATION_EVENTS` to the builder by typing the compensation built
   in `build_unchecked` as `[Event; COMPENSATION_EVENTS]`; a third event becomes a
   compile error, not a stale count. No behaviour change.
4. #379 F2: `Removed` is `{Deleted, Nothing}`, no payload; changing it is a testkit
   public API change (H-16), not taken. `WorkerdFixture::remove_outside_port` already
   records each SqlError in `HarnessFaults` via `exec`. Only
   `WorkerdIngestFixture::remove_outside_port` drops it (`.is_ok()`, H-10). Fix: body
   becomes a `Result<(), String>` helper naming the statement and the SqlError; the
   trait method logs it with `worker::console_error!` (as the ingest dispatcher does for
   its faults) and answers `Nothing`. Check with wasm32 `cargo check`.

## Test list
- [x] views_sharing_a_scope_ask_for_its_item_once (fan_out/runs.rs)
- [x] existing fan_out, sync-testkit unit + wrong_ingest_stores stay green

## Progress
- 12d2524 item 1: citation repointed to 3854-3870; line 81 unchanged (correct).
- item 2 red: characterisation test green on base; mutation (no dedup) failed at
  runs.rs:169 with left [{shard:1},{shard:0},{shard:1},{shard:0}]. 8314ae3 green, 37/37 fan_out.
- 15edeee item 3: array typed by COMPENSATION_EVENTS; sync-testkit lib 70, wrong_ingest_stores 14,
  memory_ingest_conformance 45 green.
- 00aa62b item 4: wasm32 clippy -D warnings clean; Removed unchanged.
- spec-trace: no problems. lints: green. temper gate GREEN a64e5c0003fe41e7.
