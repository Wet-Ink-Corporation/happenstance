# ADR-0087 — ES-11 is met on one-shot HTTP by a read-settlement fence

- **Status:** proposed. Written 2026-10-07 while the owner is away; nothing in it is decided until
  the owner accepts it. The seven questions it cannot settle on its own, D1, D2, D6, D8, D10, D11
  and D12, are in §11.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window), lane L8.
- **Supersedes:** [ADR-0061](../../.kb/decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md),
  **on acceptance only**. Until then ADR-0061 stays `accepted` and its atom is untouched (D10).
- **The owner's standing call** (`runbook/handover.md:29-33`): *"spike the ES-11 fence on Neon (ask
  again if it fails)"*. The spike did not fail. This record says what it found and what follows.
- **Evidence:** `experiments/es-11-fence/`: its README (the rule, written before the first run and
  amended before the first counted run), `run.sh`, `results/raw/` and `results/tally.md`. Measured on
  CI's `live-neon` job against the live Neon endpoint: spike PR #50, branch
  `lane/p17-es11-fence`, head `1177cfc` (GitHub's merge ref `19b1271`, the SHA each attempt's
  `ES11-SWEEP-META` row records), run `37594816236`, attempts 1, 2 and 3.
- **What it is not:** an implementation. **The spike is not merged** and will not be. Landing the
  fence on `main` is a separate change, after acceptance, in `0.4.0`'s window (D12).

---

## 1. Context

**ADR-0061 kept ES-11 `[PROVISIONAL]` while `happenstance-neon` stood outside the release set.** It
found that ES-11's sufficiency condition for asynchronous drivers was a fact about pooled drivers
(one queue orders a read before a later append) stated as a fact about async ones, narrowed it to
*"spawned at the first poll, and ordered against a later append by something the store itself
honours"* (`references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md:105-106`), and
recorded that Neon does not satisfy ES-11 (`.kb/decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md:122-145`).
That was a reasonable keep for a crate held out of a release. It does not survive Neon being one
of 1.0's crates: ADR-0066 lists *"the ES-11 record that supersedes ADR-0061's keep, now that Neon is
promised, settling ES-11 and ES-12 together"* among phase 17's obligations
(`.kb/decisions/0066-what-1-0-promises.md:376`), and phase 17 carries it as an open item
(`runbook/phases/17-breaking-window.md:162-169`).

**ADR-0061's own falsifier 2 has fired, and that alone forces a successor.** It said the figure
*"104 of 105 observed"* would become *"103 of 105"* if `query_items_share_one_snapshot` went red on
Neon (`references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md:233-235`). It went red:
in PR #20 (2026-09-28), in CI run `37504851570` (2026-10-06, PR #36) and in CI run `37570009097`,
passing on re-run each time (ES-12's marker records the first two, `spec/SPECIFICATION.md:3214`).
So the 104-of-105 figure is false by ADR-0061's own arithmetic. An accepted record cannot be edited
to say so; only a successor can.

**The race was a working cost, not only a clause.** This session the live Neon job went red on
exactly these two rules on three main-line PRs: #40, #44 (at `5955982`) and #46 (at `4e0f7de`).
The total number of live-Neon runs this session was not recorded, so no rate is drawn from those
three. The owner made the job non-required until L8 (`wi-0f1291`; §9).

**The shape of both racing rules.** `read_result_is_stable_under_concurrent_append`
(`crates/happenstance-testkit/src/suite.rs:6104`) and `query_items_share_one_snapshot`
(`crates/happenstance-testkit/src/suite.rs:6208`) do the same thing on one handle: seed, read
`before`, build a stream and poll it **once**, append a late event, drain, and assert that the drain
equals `before` and does not hold the late event. On Neon the first poll moves the stream from
`Unsent` to `InFlight` by calling `round_trip` (`crates/happenstance-neon/src/event_store.rs:1409`);
the test transport spawns the HTTPS request at call time
(`crates/happenstance-neon/tests/support/transport.rs:407`). `append`
(`crates/happenstance-neon/src/event_store.rs:818`) then spawns a second, independent request.
Nothing orders the read's snapshot against the append's commit, and when the commit wins, the read's
visibility frontier admits the row. The failure is always in that direction.

## 2. The mechanism

### 2.1 The fence

**Hold an `append` until every read the same transport dispatched before it has been answered.**
On the spike that is `SqlTransport::reads_settled`, a new **required** trait method, and three types
that do the bookkeeping, `ReadLedger`, `ReadTicket` and `ReadsSettled` (spike files
`crates/happenstance-neon/src/transport.rs` and `src/transport/ledger.rs` at `1177cfc`; they do not
exist on `main`, so no line is cited here). `NeonEventStore::append` awaits it once, after its
ceiling checks and before its first attempt; retries do not wait again.

**Why this orders the operations at the store, which ADR-0061 asked for.** The append's request is
handed to the transport only after the endpoint has **answered** the read. An answer is causally
after the statement executed, so the read's snapshot existed before the append's request was
written, and before its transaction began or committed. The store honours the chain *a response
follows execution*. No assumption about proxy routing, backend affinity or HTTP/2 stream order is
needed.

The ledger is a `std::sync::Mutex` and a table of `Waker`s: no executor, no timer, no atomics, no
tokio. Reads are ordered by **generation**, not counted: a waiter captures a horizon when it is
created and waits only for reads dispatched before it, so a steady stream of later reads cannot
starve an append. Wakers are collected under the lock and woken after it is released, so a waker
never runs executor code under the ledger's lock. A poisoned lock is recovered with
`PoisonError::into_inner`, which is sound because no user code runs under it and each mutation is one
map operation.

For a reader coming from C#: the closest picture is a `SemaphoreSlim`-like gate that the **I/O
completion** releases, not the caller. The Rust-specific part is *who* can release it. A Rust future
does nothing unless something polls it, so a release that lives inside a future the caller has set
aside never happens. That is the whole design problem, and §2.2 is why.

### 2.2 Why the obvious fence deadlocks the rule, and this one does not

**F0, a count on the handle released by the stream, deadlocks the rule itself.** The rule runs on one
task, T. Under F0 the stream registers at its first poll and releases on `Ready` or `Drop`. The stream
learns that its response arrived only when T polls its `InFlight` future. But T is parked inside
`append`, waiting for that release:

> append waits for the release → the release waits for T to poll the stream → T waits for append.

Eager dispatch does not help. The response may well have arrived; the only code that can observe it
is a future nobody polls.

**F1 moves the release to an agent other than T: whatever drives the transport's I/O.**

- **On the host**, the live transport moves the `ReadTicket` into the task it spawns for the request.
  When that task ends (answered, errored, timed out or panicked) the ticket drops, the ledger
  settles, and the waker T's `ReadsSettled` stored is woken. A tokio worker does this; T's stream is
  never involved. The spike's live transport also bounds every round trip at 30 s, so a hung read
  settles on its timeout.
- **On `wasm32`, without tokio**, a `fetch` transport would call `fetch()` inside `round_trip` (which
  runs at the stream's first poll, inside a polled context) and hand the ticket to the promise's
  settlement callback. The JavaScript event loop resolves the fetch while T sits in `Pending`; the
  callback drops the ticket and wakes T through `wasm-bindgen-futures`' local executor. **This is
  reasoning, not measurement** (§3.5): no `fetch` transport exists in this tree. What is checked is
  that the ledger and the adapter's wait compile for `wasm32-unknown-unknown` with no tokio
  (`cargo xtask wasm` on the spike), where std's mutex is the single-threaded one.

### 2.3 The obligation, written on the trait as a MUST

The spike's trait documentation states it. A transport **MUST**:

- register a request whose `SqlRequest::read_only` is set **before `round_trip` returns** (dispatch
  is eager);
- settle it when the endpoint answers, the send fails or the transport's own timeout fires, **driven
  by the transport and never by polling `round_trip`'s future**;
- settle it when that future is dropped unanswered, or leave it to settle when the abandoned request
  does.

A transport whose I/O advances only while its `round_trip` future is polled **deadlocks** an append
behind a read the caller polled once and set aside, exactly as F0 does. The adapter cannot detect
that at run time. It is the residual risk, stated on the trait, and the offline test that rejects
F0's shape (A2 below) is written as a downstream transport author would write one.

`head` and `contains_event_id` are `read_only`, so a transport registers them too (D13, taken as the
plan's default). Not fenced: `ProbeThenWriteStore`, the `#[cfg(test)]` ingest path, and
`NeonProjectionStore`.

### 2.4 The ordering domain

**One transport value.** Clones of a `ReadLedger` share it, so every store built over clones of one
transport shares one ordering domain. The conformance fixture builds a fresh transport per
`connect()`, so the domain the rules see is the rule's one handle, and the concurrency family's
contenders, which connect separately, do not wait on each other's reads. ES-11's MUST is not
changed. What the rules check is the same-handle case; cross-handle real-time order is not
observable through the port, and this record does not claim it.

**So the conformance claim this record proposes is scoped, and the scope is the owner's call (D3).**
A read on one handle and an append on a second handle with its own transport are not ordered by
the fence: the second transport's ledger cannot see the first's outstanding read, so the append can
commit before the read's snapshot is taken. ES-11's MUST is written of the store, not of a handle.
Whether "met for operations sharing one transport, and stated as such" satisfies it, or whether
Neon's claim must carry the cross-handle case as a named exception, is not settled by the
measurement and is not settled here. A process-wide ledger would widen the domain to one process
and still leave two processes unordered (plan D3).

### 2.5 The offline evidence

These run in the gate on the spike branch, drive futures by hand with a counting waker, and use no
`#[tokio::test]`, which is part of the evidence that nothing in the fence needs an executor. Each
was red first where a red was available; the lane's plan, which is not committed, records each
failing line under "Red runs".

- Ledger: ready when nothing is outstanding; waits for an earlier read; ignores a later read (red
  against a count-to-zero mutant); waits for the oldest under out-of-order settlement; a dropped
  ticket settles; a re-polled waiter holds one slot; the types are `Send + Sync`; a waker runs
  outside the lock (red against a wake-under-the-lock mutant); a saturated generation counter waits
  rather than releasing early (red against a mutant without the guard); two horizons release
  independently (red against two mutants); a poisoned ledger keeps working (red against a
  reset-on-poison mutant).
- Adapter, through the public API with a fake transport: **A1** an append after a first poll waits
  for the read to be answered (red with the method present and the adapter not awaiting it: the
  write was dispatched on the append's first poll); **A2** the fence releases without the stream
  being polled (rejects F0); **A3** a dropped in-flight stream still lets an append through, over a
  parked and over a cancelling transport; **A4** concurrent appends are not queued behind each
  other; **A5** an unpolled stream costs an append nothing; **A6** a failed read settles; **A7** a
  read dispatched while an append waits does not extend the wait; and the same wait behind an
  in-flight `head` and `contains_event_id` (red against a mutant that did not mark them read-only).
- A `compile_fail` doctest with an impl that omits `reads_settled`, and a passing twin with the
  method added. Stable rustdoc does not check a `compile_fail` error code (verified on rustc 1.97.1),
  so the twin is what shows the omission, and not some other error, is what fails.

## 3. The measurement

### 3.1 The rule, as pre-registered and as amended

The rule was written into `experiments/es-11-fence/README.md` on 2026-10-07 before any trial ran,
and amended **the same day, before the first counted run** (`experiments/es-11-fence/README.md:184-230`).
The reason: a review of the spike at `81eab3b` (finding W1) found the first rule could score a
visibility-frontier lag as a falsifier. Every read carries the `xmin` frontier, which CI has seen
hide committed rows; a lagged `before` then made `drained != before`, which was scored red, and
under the fence its timings would have classed it C3, **a false falsifier**. W2 to W5 tightened the
pooling, the V1 line count, the C3 tie and the README's own description. The rule of record is the
amended one (`experiments/es-11-fence/README.md:105-182`), applied mechanically by `run.sh`:

> **Size.** 250 trials per arm per shape per job attempt, so 1,000 trials per attempt. Only rows of
> `schema` 2 are judged. […] So is the pilot attempt by name (run `37591126575`, attempt 1).
>
> **Void attempts.** An attempt is **void** […] when any of these holds: its extracted row count is
> not exactly 1,000 […]; it has more than 25 `error` rows; it has more than **50 `anchor` rows**
> (5% of the attempt); its log does not carry exactly **one** result line […] for each of the two
> racing rules (V1 below).
>
> **Pooling.** […] only the **first 3** are pooled.
>
> **What counts.** A trial counts as `red` only when its `before` was complete. `anchor` and `error`
> rows are reported […] and are neither `pass` nor `red`.
>
> **The instrument is live** if the pooled `baseline` arm shows **at least 3** `red` trials […]. If
> it shows fewer after 3 pooled attempts […] the verdict is **inconclusive**.
>
> **The fence works** if all three hold: the instrument is live; the pooled `fence` arm has **0
> `red`** trials, of any class; V1: the conformance rules `read_result_is_stable_under_concurrent_append`
> and `query_items_share_one_snapshot` are green in every pooled attempt.
>
> **The fence fails** if any `fence` row is `red`. Each red is classed by client-side order:
> **C3, causal violation**: `t_append_dispatch_us >= t_read_answer_us` […]. **C1, client reorder**:
> `t_append_send_us < t_read_send_us`. **C2, reorder after send**: the read reached `hyper` first,
> and the append was dispatched before the read was answered.

**Who saw the pilot.** One attempt ran on `81eab3b` under the first rule (run `37591126575`,
attempt 1). The amendment's content was fixed by the review and handed to the agent that wrote it
before the pilot's log was fetched, and that agent never read the pilot's rows. The session that ran
the spike read them afterwards. The README discloses this (`experiments/es-11-fence/README.md:191-196`).
The pilot is excluded by name and by its row schema, whatever it shows. The ordering of rule and run
is **self-attested**, as `wi-95d2b2` was for ADR-0080: nothing but the commit history vouches for it.

**One trial** follows the racing rules' shape on one handle, with differences the README lists
(`experiments/es-11-fence/README.md:59-76`): one schema shared by the attempt, a query scoped by a
per-trial tag, and an `anchor` outcome where the rules would fail the test. **Two arms**, same
binary, same client, same endpoint: `baseline`, a transport that registers nothing and whose
`reads_settled` is ready at once (the adapter as shipped), and `fence`. Arms and shapes are
interleaved per iteration with the order rotated, so drift in the endpoint falls on both alike.

### 3.2 The result: the fence works

From `experiments/es-11-fence/results/tally.md:14-25`, generated by `run.sh tally` from
`results/raw/` and regenerated for this record with no difference:

| Arm | Shape | Pass | Red | Anchor | Error | Red, by class |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| baseline | `es11` | 658 | 92 | 0 | 0 | all C2, all `late_in_drained` |
| baseline | `es12` | 670 | 80 | 0 | 0 | all C2, all `late_in_drained` |
| fence | `es11` | 750 | 0 | 0 | 0 | — |
| fence | `es12` | 750 | 0 | 0 | 0 | — |

- **Baseline: 172 red in 1,500 judged trials.** Clopper–Pearson 95%: **9.90% to 13.19%**. The
  instrument is live by a wide margin (the rule asked for 3).
- **Fence: 0 red in 1,500 judged trials.** Rule-of-three 95% upper bound: **0.20% pooled**, and
  about **0.4% per shape** (3 / 750).
- **0 anchor and 0 error rows** in any pooled attempt; **no attempt void**. Each attempt has exactly
  1,000 rows.
- **V1:** both racing conformance rules are `ok` in all three attempts (six result lines, none
  `FAILED`; `results/raw/37594816236-*.v1.txt`). The whole live `neon_conformance` target passed in
  each attempt, 109 of 109 by the test harness's own summary line; that line is in the job logs, not
  in a committed file.
- **The pilot** (excluded): baseline 27 of 250 red per shape, fence 0 of 250 per shape.

**Verdict under the rule: THE FENCE WORKS.**

### 3.3 What the classes say

Every baseline red is **C2**: the read's task reached `hyper` first, and the append was dispatched
before the read was answered. There is **no C1**, so in 172 reds the append's task never reached the
client library before the read's. That partly answers ADR-0061's falsifier 3 (a client-side
reordering): none was seen **before** `hyper`. C2 takes in reordering anywhere after that point —
inside `hyper` and `h2`, at the proxy, or at the backend — and the sweep cannot tell those apart
(`experiments/es-11-fence/README.md:166-169`). So falsifier 3 is not fully answered, and the fence
does not need it answered: it does not depend on where after dispatch the reordering happens.

There is **no C3** in either arm. Under the fence C3 is the only class possible, and it is the
falsifier (§8).

### 3.4 What the fence costs, recorded and not claimed

Each fence-arm row records `fence_wait_us`. Over the 1,500 fence trials its median is about 120 ms
and its 95th percentile about 229 ms (`jq` over `results/raw/37594816236-*.jsonl`). The sweep polls
the stream once and appends at once, so every fenced append waits for a full read round trip; that is
the worst case by construction, not a typical one. No performance claim rests on it, as the README
says (`experiments/es-11-fence/README.md:263-264`). An append with no read in flight on the same
transport does not wait.

### 3.5 What was not measured, and must be said

- **A `wasm32` `fetch` transport.** None exists in this tree; the crate owns no HTTP client. That a
  `fetch` transport can settle a read from its promise callback, without its future being polled, is
  **reasoning** (§2.2). What is checked is only that the fence compiles for `wasm32-unknown-unknown`
  without tokio. Falsifier 2 is the open end.
- **The second-handle and cross-process case.** Two handles over independent transports are two
  ordering domains and are not ordered against each other. No rule observes that through the port,
  and neither does the sweep, which uses one handle per arm.
- **The concurrency family's contention under the fence.** The family's rules (`CONTENDERS = 64`)
  **passed** in all three attempts as part of the 109, and no `Busy` exhaustion was seen. But
  contention itself was **not measured**: nothing compared how many contenders actually overlapped
  with and without the fence. The design argument is that appends never wait on appends and
  contenders hold separate ledgers (offline test A4), so the race stays a race; that is the argument,
  not a measurement. Falsifier 3 is the open end.
- **The anchor read's frontier lag, as a rate.** `READ_YOUR_OWN_WRITES` is declined; the sweep
  counted anchors (none) but is not a measurement of the lag.
- **Throughput** (§3.4).

## 4. Decision (the fence works)

Proposed, for the owner to accept:

1. **ES-11's sufficiency paragraph is restated, line-neutrally**, in the change that lands the
   fence: a read spawned at its first poll and *ordered against a later append by something the store
   itself honours — one queue, or the read's answer preceding the append's dispatch*. ADR-0061's
   narrowing stands; this names the second way to meet it. It is still a narrowing, not a widening:
   nothing an adapter must do gets easier.
2. **`happenstance-neon` meets ES-11 and ES-12 through `SqlTransport::reads_settled`.** Its shape is
   F1, a required trait method, and that is a **semver-major break of `SqlTransport`** (D1). It lands
   in `0.4.0` (D12) as its own implementation change, **not the spike**.
3. **ES-11 and ES-12 move to `[FROZEN]` when the fence is on `main`**, not when this record is
   accepted (D6). A clause frozen on a record whose mechanism is not yet in the tree would be frozen
   against a crate that still fails it.
4. **The ordering domain is stated**: one transport value (§2.4). ES-11's MUST is unchanged, and the
   rules check the same-handle case.
5. **ADR-0061's *"Postgres gets both halves"* is corrected** (D11). `happenstance-postgres` does not
   order a read against an append by anything the store honours: both are spawned onto one
   multi-thread runtime and enter one `PgPool`, and the read's task was enqueued first
   (`crates/happenstance-postgres/src/read_stream.rs:94-98`). That is order by queue position, which
   passes by margin rather than by a guarantee. **This is reasoning, not a measurement**; nothing here
   reproduced a Postgres red, and none has been seen. Nothing changes in the Postgres adapter.
6. **ADR-0061's figure is corrected**: *"104 of 105 observed"* was false once
   `query_items_share_one_snapshot` went red (§1), 103 of 105 by its own arithmetic. With the fence,
   both racing rules pass and the live suite passed whole in each counted attempt (§3.2). No new
   standing figure is stated here, because the suite's size moves with every phase; the tool's count
   is the figure.

## 5. What would have followed had the fence failed

Not taken, recorded so the branch is visible. A C1 or C2 red under the fence would have been a spike
defect, fixed and re-run. A **C3** red would have fired falsifier 1: the plan said stop and ask the
owner (D7), with three options — a named, documented ES-11/ES-12 exception in Neon's conformance
claim; dropping Neon's claim on those clauses from 1.0; or keeping `[PROVISIONAL]` past 1.0 with
ADR-0066 amended. §4's corrections to ADR-0061's figure and its Postgres claim would have stood
either way. None of that is needed: the fence arm had no red of any class.

## 6. Alternatives refused

- **F0**, a count on the handle released by the stream: deadlocks the rule (§2.2).
- **F2**, the handle owns the in-flight read and `append` drives it: needs `T: Clone + 'static` on
  the `EventStore` impl, and a non-`Send` `dyn Future` field makes `NeonEventStore` lose `Send` and
  `Sync` on the host. Two semver-major breaks, and `unsafe` is forbidden.
- **F3**, filter this handle's own later appends out of the read: leaves a hole below the observed
  maximum position, the silent corruption ES-11 exists to forbid.
- **F4**, a server-side ticket: reads become writes, it needs a second migration, and an append hangs
  on a lost read.
- **F5**, the same mechanism as a documented obligation on the transport with no signature change:
  conformance would then hang on an obligation the compiler never shows an implementor, an
  incompatible behaviour behind an unchanged signature. **F5 is the fallback if the owner will not
  break the trait** (D1): the sweep measured the shared mechanism, so its verdict carries to F5.
- **A defaulted `reads_settled`** returning a ready future: additive in name, and silently
  non-conformant in every downstream transport that does not override it.
- **From ADR-0061, still standing:** carving one-shot HTTP out of ES-11's scope, and minting a
  `Capability` so the rule reports a skip.

## 7. Published-API impact

| Shape | What changes | Class |
| --- | --- | --- |
| **F1 (proposed)** | `SqlTransport` gains the **required** `reads_settled`, so every downstream transport breaks. New public `ReadLedger`, `ReadTicket`, `ReadsSettled` in `happenstance_neon::transport` (additive; D2). `NeonEventStore::append` may wait up to one read round trip, documented. `NeonEventStore`, `NeonReadStream` and `NeonError` do not change, and neither do their auto traits | **breaking**, `0.4.0` |
| F1 with a default | an additive method, silently non-conformant | compatible in name, broken in behaviour |
| F2 | a tightened bound on the `EventStore` impl, and `Send`/`Sync` lost on `NeonEventStore` | breaking, twice |
| F5 | no signature; a new MUST on implementors that `cargo semver-checks` cannot see | breaking, invisibly |

`happenstance-core`, the testkit and every rule are untouched by any shape. The dependency direction
does not change, and `happenstance-neon`'s `[dependencies]` do not change: no tokio, no new crate,
no new feature.

## 8. Falsifiers

Any of these reopens this record:

1. **A C3 red** — a fence-arm sweep red, or a red in either racing rule with the fence in place,
   where the append was dispatched no earlier than the read was answered. The endpoint would then not
   honour response-before-dispatch (for example, a read routed to a lagging replica).
2. **A conformant `SqlTransport` proves impossible on `wasm32`**: a `fetch` transport cannot observe
   settlement without its future being polled, so the obligation cannot be met on a target the crate
   claims.
3. **The concurrency family regresses with the fence**: a `Busy` exhaustion or a family-rule red
   attributable to the added waits, or a measured loss of contention (the race becoming a queue).
4. **Test A1 passes against an unfenced adapter**: the offline evidence would be vacuous.
5. **A second-handle or cross-process ordering that callers depend on turns out to be checkable
   through the port**, which would make the stated ordering domain too narrow.

**Not a falsifier:** green runs, carried forward from ADR-0061
(`references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md:241-242`). The claim rests on
the mechanism and the offline tests; the sweep shows the instrument sees the race and bounds the
residual.

## 9. Consequences for `wi-0f1291`, the required check

`wi-0f1291` made the `conformance against a live Neon endpoint` check non-required, with the flip
condition *"When L8's ES-11 record lands, re-add the check to the Protect main ruleset"*
(`.kb/_intake/decisions/wi-0f1291-the-live-neon-job-is-a-required-check-and.md:25`).

- **The race persists on `main` until the fence itself lands.** Restoring the required check when this
  *record* lands would re-arm the flake against every PR, on the evidence above at a baseline rate of
  roughly one trial in nine.
- **Recommended reading (D8): the flip condition means "when the fence lands on `main`".** At that
  point the owner re-adds the check to ruleset 22926481 and confirms it is required.
- The strict job then means what ADR-0061 wanted: no allowlist, and red only on a real failure.

## 10. Consequences for documents

None of these is edited by this record. All are applied in the change that lands the fence, and all
line-neutrally where they are cited by line:

- `spec/SPECIFICATION.md`: ES-11's marker (`spec/SPECIFICATION.md:3087`), its sufficiency paragraph
  (`spec/SPECIFICATION.md:3146-3150`, restated in exactly five lines), its `Rejects:` bullet
  (`spec/SPECIFICATION.md:3186-3193`), ES-12's marker (`spec/SPECIFICATION.md:3214`), §6.5's
  transport row (`spec/SPECIFICATION.md:9295`) and the two trace rows
  (`spec/SPECIFICATION.md:9874-9875`, `PROVISIONAL` to `FROZEN`).
- `happenstance-neon`'s README section on the rule it does not pass, its crate root's status prose,
  and the read stream's documentation (the spike already rewrote these, line-neutrally, for its own
  branch).
- `runbook/ledgers.md`: the residual-exposure row (`runbook/ledgers.md:187`); the ES-11 and ES-12
  disposition rows (`runbook/ledgers.md:289-290`) now name this record, and leave the table when the
  clauses freeze.
- `runbook/phases/17-breaking-window.md:162-169`, ticked when the fence lands, not before.
- `.kb/open-questions/one-shot-http-conformance-to-es-11.md`: amended to say this record answers it,
  pending acceptance, and closed on acceptance.
- `.kb/decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md`: on acceptance, and only then,
  `status: superseded` and `superseded_by: kb-decision-0087`. Its body is immutable.

## 11. Not decided here: the owner's

Each was taken as the plan's default so the spike could run, and none is acted on beyond the spike
branch. The owner decides each.

- **D1 — the fence's shape.** F1, a required `SqlTransport::reads_settled`: a **semver-major break**
  of a published trait, visible to the compiler. F5 is the no-break fallback with the same mechanism
  and the same verdict, at the cost of an obligation no signature shows.
- **D2 — publishing `ReadLedger`, `ReadTicket` and `ReadsSettled`.** Recommended public: every real
  transport needs exactly this bookkeeping, it is where the lost-wakeup and starvation bugs live, and
  it is the no-tokio answer for `wasm32`. Additive, but a public surface 1.0 then carries.
- **D3 — the ordering domain, and what the claim says about it.** One transport value (clones
  share), as built. Two handles over separate transports, or two processes, are not ordered (§2.4).
  Recommended: accept the transport domain and state the cross-handle case as a named limitation of
  Neon's ES-11 claim; the alternative is a process-wide ledger, which still leaves processes
  unordered. This is the question review raised against the record, and it gates the claim.
- **D6 — when ES-11 and ES-12 freeze.** Recommended: when the fence lands on `main`, not on this
  record's acceptance. Freezing is one-way.
- **D8 — when `wi-0f1291`'s required check comes back.** Recommended: when the fence lands on `main`
  (§9).
- **D10 — when ADR-0061 is marked superseded.** Recommended, and followed here: on acceptance only.
- **D11 — correcting ADR-0061's *"Postgres gets both halves"*.** Recommended: correct it here,
  labelled as reasoning (§4 item 5).
- **D12 — landing the break in `0.4.0`.** Recommended: `0.4.0`'s window, as its own lane. Deferring it
  past `0.4.0` means it needs a major after 1.0.
