# Plan: #441 (the refused-push check in `directional_merge_rules_compose` cannot fail), with #450's interface boundary

Branch `claude/project-thread-xxl8lr-441`, off `origin/phase-14` (`1cfa397c`). Planned 2026-11-15.
#441 is planned in full. #450 is a separate PR, and only its interface boundary and decisions are planned
here. The owner has decided (2026-11-15) that SY-7 and SY-10 freeze in 0.7.0.

## Goal

#441: `directional_merge_rules_compose` (SY-10) stops claiming a check that no store can fail. Its
hub-and-spoke walk runs in the order the shipped `Edge::step` runs, pull first and then push, and its
rustdoc says where SY-34's refusal is held and why the ingest family cannot hold it.

#450 (later): the runner gets an adjudication hook, so a non-adjudicator can be shown to write no
compensation (SY-7). The hook also lets an edge adjudicate what it pulls (SY-10). With the hook in place,
both rules are mounted and both markers move.

## Recommendation for #441: delete the check (issue option 2), and land it as its own PR before #450

Why option 1 (give the store something to do on the refused push) has nothing honest to test:

- **The store is never told that a push was refused.** In the walk, and in `Edge::step`
  (`crates/happenstance-sync/src/runner.rs:451-491`), the refusal is the far peer's answer to
  `SyncPeer::push`. The local store sees no call that differs from an accepted push.
- **No port the runner holds can remove or rewrite an event.** `EventStore` has `read`, `append`, `head`
  and `contains_event_id` (`crates/happenstance-core/src/store.rs:159-341`). `IngestStore` has `ingest`,
  `store_id` and `watermark`. `WriteGroupStore` has `read_groups`. SY-34 names one wrong implementation,
  rolling the spoke back to the last acknowledged position, and the runner cannot express it through
  those ports. So any store-side check at the refusal would only re-test whatever ran before it.
- **The one store that could fail the check** keeps an outbox and consumes it on `read_groups`, so a
  refused push loses its re-offer. That store belongs to the write-group family, and the ingest family's
  `IngestFixture::Store` is not bound by `WriteGroupStore` (`ingest_fixture.rs:55`, and
  `write_group_suite.rs:1-9` says the omission is deliberate). Making the walk read through `read_groups`
  would need either a bound on a published fixture trait (breaking) or a move to another family. That is
  #450's question, M below, and not one to settle inside #441.

Why deleting serves the #450 mount best:

1. **The shipped runner never produces this walk's refusal.** `Edge::step` always pulls before it pushes.
   A "refused push before any pull" exists only in the played runner. Once the walk runs pull, then push,
   then pull, it is a sequence of `Edge` steps, the same one `tests/runner/directions.rs:212-281` drives.
   #450 can then swap the played `MergeRule` for the shipped hook without also reordering the walk.
2. **The non-decorative refusal checks need a peer**, and SY-8 keeps peers out of the ingest family.
   They already exist at runner grain:
   - `tests/runner/directions.rs`: the declined step keeps the spoke's own claim, and the pulled claim
     lands unjudged.
   - `tests/runner/main.rs:310-338`, `a_refused_push_keeps_the_log_and_the_watermark_and_records_the_class`.

   #450 rewrites `directions.rs` around the shipped hook anyway, and that is where its new refusal check
   belongs (T-450-6 below).
3. **ADR-0107 §2.1 already recommends this fix** (`.kb/decisions/0107-phase-13s-clauses-are-frozen.md:159-162`).

**Separate PR, not folded into #450.** #441 touches only the testkit: it changes no rule verdict, adds no
API and needs no ADR, so it can merge while #450's owner-level API decisions are still open. The SY-10
marker (`spec/SPECIFICATION.md:7315`) and the ledger row (`runbook/ledgers.md:326`) both make #441 a
prerequisite of the freeze, not of the hook, so a separate PR meets that order whichever lands first.

One conflict in the runbook, flagged rather than edited. `runbook/handover.md:57` says "#450 first, then
#441", while `runbook/phases/13-sync.md:225` and ADR-0107 say #441 comes first. The two orders do not
actually clash, because #441 only has to land before the freeze. The freeze PR should make handover agree.

## Constraints and non-goals

- Testkit only. No change to `happenstance-sync`'s `src/`, to any public item of `happenstance-sync-testkit`,
  or to any rule's verdict against any store.
- Not editing the SY-10 marker, the SY-10 ledger row, or ADR-0107. An accepted atom is immutable, and the
  marker and ledger text are rewritten by the freeze PR.
- Not writing SY-34's or SY-33's named rules. Both clauses are FROZEN, and their rules
  (`a_refused_spoke_retains_its_log` and `refusal_is_never_condition_derived`) are unwritten, shown with a
  † at `spec/SPECIFICATION.md:10686-10687`. They are reported under Risks as a separate finding.
- Not adding the relayed group to the walk's push. A real `Edge` would push the other spoke's claim back
  (`directions.rs:253`, `pushed == 2`). That fidelity belongs to #450's mount.

## Invariants

- `directional_merge_rules_compose` still fails `Mistake::AdjudicatesOnIngest`, and only that rule fails
  it, with a message from the first pull ("a claim pulled from the far end was counted as ...").
- The faithful store and every mounted fixture still pass it.
- Each topology walk ends with the same facts as before: `hub_and_spoke_facts()` and `peer_to_peer_facts()`
  are unchanged.
- The falsification test (`topology/tests.rs`) still finds that no claim stands under `HUB_RULE_ON_BOTH`.

## Design (#441)

- **Placement.** `crates/happenstance-sync-testkit/src/ingest_suite/ingest_rules/topology.rs` and the
  rule's rustdoc in `crates/happenstance-sync-testkit/src/ingest_suite.rs` (around `:1326-1386`). Every
  item stays `pub(super)` or private (ARC-05).
- **`topology.rs` changes:**
  - Delete `enum Answer`, `fn both_facts` and the refused branch of `push`.
  - `push` becomes one `carry(rule, &edge.far, claim, None, "the near end's pushed claim")`, with the
    `answer` parameter gone. It can be inlined at its single call site if clippy or readability prefers.
  - `hub_and_spoke` runs: the first claim to the hub (unchanged), `own_claim`, `pull`, `push`, `pull`.
  - `peer_to_peer` is unchanged apart from dropping `Answer::Taken`.
  - `hub_and_spoke`'s doc (`:125-139`) is rewritten to match. The walk is now "the spoke pulls the other
    claim, guard and all, and takes it unjudged; it pushes its own and the hub adjudicates; it pulls the
    reversal". Its `# Errors` loses "or if the refused push changed either log".
  - `push`'s doc loses its SY-33 and SY-34 sentences.
  - The module doc gains a short section, "# Where a refusal is held", that:
    - names the two runner tests;
    - says that the store is never told of a refusal, and that the runner holds it only through ports
      that cannot remove or rewrite an event;
    - says that the walk therefore plays no refusal, and runs in `Edge::step`'s order so that a step of
      the walk is a step of the runner.
- **`ingest_suite.rs` rustdoc:**
  - The hub-and-spoke bullet loses "its push is refused (SY-33): neither log may change" and "Its push is
    re-offered".
  - The final sentence keeps "still holds its own claim under its own identity (E2E-45)". The `holds`
    check in `judge` is a different check from the refusal check and stays.
  - `# Does not reject` gains a bullet: **a spoke rolled back on a refused push (SY-34)**. It says why the
    rollback cannot be written through the store's ports, and where the runner's half is held.
- **Types, signatures, ownership, errors:** none new. `Violation` of `SY-10` as before.
- **Docs elsewhere:**
  - `CHANGELOG.md` `[Unreleased]`, under `happenstance-sync-testkit` → Changed, one entry: "the
    hub-and-spoke walk no longer plays a refused push, a check no store could fail; SY-34's refusal is held
    by `happenstance-sync`'s runner tests (#441)".
  - The `lib.rs` table row is unchanged, since it names only the mutant.
- **Alternatives considered:**
  - Keep the refused step and add a store-observable act to it, such as re-reading the push run. Rejected:
    this is option 1, and its only failing store is outside the family (see above).
  - Keep the refused step as a no-op for the narrative. Rejected: a step that does nothing invites the
    next reviewer to add the check back.
  - Fold #441 into #450. Rejected for the sequencing reason above. Either is acceptable to the owner.

## Test list (#441, drive in this order)

Deleting the check removes behaviour, so there is no new red test. The evidence is that every existing pin
stays green and exact, before and after.

- [ ] Before editing, record a baseline with `cargo test -p happenstance-sync-testkit --all-features`:
  - `wrong_ingest_stores` (every pin, in particular
    `directional_merge_rules_compose_fails_a_store_that_adjudicates_on_ingest` and
    `every_rule_reports_a_store_that_cannot_read_its_identity`);
  - the `topology::tests` 3/3;
  - the memory, local and wasm mounts.

  **Disk:** run one test target at a time with `CARGO_INCREMENTAL=0`, never the workspace.
- [ ] After the deletion and the reorder, the same set is green. The `AdjudicatesOnIngest` message still
  comes from the first pull.
- [ ] `hub_and_spoke_leaves_the_seat_with_the_first_claim_to_the_hub` still sees exactly `{OTHER_SPOKE@1}`
  standing.
- [ ] `the_hubs_rule_on_the_spoke_leaves_no_claim_standing` still records the reversal authors as
  `[near, far]` in that order. This is the reorder's sharpest risk: under `HUB_RULE_ON_BOTH`, the spoke's
  reversal must still be written before the hub's.
- [ ] `cargo clippy -p happenstance-sync-testkit --all-targets --all-features -- -D warnings`, rustdoc with
  `-D warnings` on the testkit, and `cargo xtask spec-trace`. No spec line moves.

Wrong implementations the kept checks reject, as the house rule requires:

- **Per-ingest counts** (`carry` and `pull`) reject `AdjudicatesOnIngest`, which is already pinned.
- **`judge`'s facts comparison** rejects the same mutant, plus any store that drops or adds a fact.
- **`judge`'s own-claim `holds` check** rejects a store that gives up its own fact when a peer's claim on
  the same tag arrives, such as an upsert keyed by tag. **Not verified** that a mutant exists for it in
  `tests/wrong_ingest_stores.rs`. #450 should add one if it keeps the check, and #441 does not widen into
  it.

## Interface boundary for #450 (next PR; decisions, not code)

### What exists

- The pull is ingested with no compensation, always (`runner.rs:429-437`).
- `StoreSyncPeer::push` ingests a push with no compensation (`store_peer.rs:318-360`), and says that the
  hook is "planned with SY-7".
- The only adjudicating far side is test-local: `Hub` in `tests/runner/directions.rs`.
- The testkit plays the runner with a private `MergeRule { Adjudicate, Accept }` (`topology.rs:63-98`).
  `adjudication.rs` does the same with an `Assignment`.

### Where the hook mounts

There are two ingest sites, one per direction of an edge, which is what SY-10 asks for:

- **Pull** (the local side ingests what it pulled): `Edge::pull_and_ingest`. This is the ADR-0107 gap:
  a hub that drives its own edge cannot adjudicate what it pulls.
- **Push** (the far side ingests what was pushed): `StoreSyncPeer::push`, and through it `serve_sync`.
  This is needed for a hub served over HTTP, such as the Durable Object hub.

A rule's direction is given by where it is mounted. No role parameter appears anywhere (SY-9).

### Recommended shape (each item is subject to D-450-1..6 below)

- **Placement.** A new module `crates/happenstance-sync/src/runner/adjudicate.rs`, re-exported from
  `runner`. The hook is runner configuration (SY-7), so it lives above the port, not in `peer.rs` (SY-8).
- **Trait** (CLAUDE.md constraint 1: `trait_variant`, never `async_trait`):
  ```rust
  #[trait_variant::make(SendAdjudicate: Send)]
  pub trait Adjudicate {
      type Error: core::error::Error + Classify + 'static;
      /// The compensation `group` lands with, from what the local store holds and from `earlier`,
      /// the groups of the same batch that land before it in the same ingest call. Empty: unjudged.
      async fn compensation(&self, group: &EventGroup, earlier: &[EventGroup])
          -> Result<Vec<Event>, Self::Error>;
  }
  /// Judges nothing: every group lands unjudged. The default on every edge and store peer.
  #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
  pub struct Accept;               // Error = core::convert::Infallible
  ```
- **Ownership per parameter and return:**
  - `&self`: the hook is shared configuration. Any state it keeps is the implementor's, and it holds its
    own read handle onto the local store, as `directions.rs`'s `Hub` does, so `Edge::step`'s store bound
    (`WriteGroupStore + IngestStore`) does not grow.
  - `&EventGroup` and `&[EventGroup]`: borrowed from the pulled batch the runner owns (OWN-01).
  - `Vec<Event>` is owned in the return because the runner keeps it alive until `IngestGroup::new(events,
    &compensation)` borrows it for the one ingest call (OWN-02, OWN-13).
- **Decide-only.** The hook returns compensation and never writes. The runner lands the group and its
  compensation in one `IngestGroup` (SY-2, SY-30), so a compensation committed apart from its group cannot
  be expressed at all (API-02).
- **`earlier`** exists because a batch lands in one ingest call. Without it, a hook that reads the store
  misses a contest inside the batch: two claims on one seat in one pulled batch. `directions.rs`'s `Hub`
  has exactly this blind spot today.
- **Edge.** `pub struct Edge<R, A = Accept>`:
  - `new`, `resuming`, `restored` and `Default` stay on `impl<R> Edge<R>`, that is `Edge<R, Accept>`.
  - New `pub fn with_adjudicator<B>(self, rule: B) -> Edge<R, B>`.
  - `step` and `run` move to `impl<R, A: Adjudicate> Edge<R, A>`.

  **Spiked** on rustc 1.97.1 in the scratchpad: `Edge::new()` still infers `A` from the impl (the
  `HashMap::new` precedent); `fn user(e: &mut Edge<u32>) -> Result<u8, StepFailure<Pp, u8>>` compiles
  unchanged; a type alias with a defaulted parameter is accepted.
- **Errors.**
  - `StepError<P, R, I, A = Infallible>` gains `Adjudicate(#[source] A)`, which is allowed because the
    enum is `#[non_exhaustive]`.
  - `Phase` gains `Adjudicate`, also allowed by `#[non_exhaustive]`.
  - `StepFailure<P, S, A = Accept>`.
  - `impl Classify for Infallible`, which is additive: coherence keeps any downstream crate from writing
    it.
  - A hook failure is reacted to by its class, like every other failure (SY-36). The pull token does not
    advance, because nothing was ingested.
- **Store peer (if D-450-2 is A).**
  - `StoreSyncPeer<S, I, A = Accept>` with `with_adjudicator`.
  - `StorePeerError<R, W, A = Infallible>` gains `Adjudicate(A)`, under `#[non_exhaustive]`.
  - The hook runs after `admit`, so a refused push writes nothing, and before the single ingest.
- **Send flavour.** `tests/flavours.rs` gains a hooked edge whose step future is `Send` under
  `tokio::spawn` in generic code, together with a `!Send` hook driving a bare edge (SY-17).

### Test list for #450 (in order; FanRunner precedent: `crates/happenstance/tests/fan_out/rules.rs`)

- [ ] T-450-1: `Edge::new()` with `Accept` behaves as today. The existing `tests/runner/*` suite passes
  unchanged, which is the semver canary.
- [ ] T-450-2: a hub-driven edge, `Edge::new().with_adjudicator(SeatRule)`, pulls a spoke's contested
  claim and lands it with one reversal under the hub's identity. Its red step is a compile error against
  `with_adjudicator`.
- [ ] T-450-3: the same stores under `Accept` land the claim with no reversal. This is SY-7's runner half
  on the shipped runner.
- [ ] T-450-4: two contested claims in **one** pulled batch earn exactly one reversal, handed through
  `earlier`. It rejects a runner that judges each group against the log as it was before the batch.
- [ ] T-450-5: a hook failure of each class parks, stops or retries as SY-36 says, the token does not
  advance, and nothing lands.
- [ ] T-450-6, the refusal check #441 hands over: a declined push to an adjudicating store peer leaves the
  far log unchanged, through `admit`, the only refusal `StoreSyncPeer` makes. It rejects a store peer that
  adjudicates before it admits.
- [ ] T-450-7: rules written over a test-local `EdgeRunner` trait run against the shipped `Edge` **and**
  against `MiniEdge`. `MiniEdge` is built over the public API with one `Defect` each:
  - `AdjudicatesUnderAccept`: the rule SY-7 forbids, every peer adjudicating;
  - `AdjudicatesWhatItPushes`: the push-side rule applied to the local side;
  - `IgnoresEarlier`;
  - `CompensatesApart`: compensation as a second write.

  Each defect fails exactly its rule, and the shipped `Edge` passes them all. This is the "wrong runner
  that compensates when it should not" that #450 asks for.
- [ ] T-450-8: the testkit mount (per D-450-4). `topology.rs`'s and `adjudication.rs`'s private rule
  enums are replaced by a testkit `SeatRule: Adjudicate` and the shipped `Accept`. The existing pins stay
  exact: `AdjudicatesOnIngest` fails SY-10 only, and `FilesCompensationUnderAPlaceholder` fails SY-7 only.
  The falsification tests keep their findings.
- [ ] T-450-9: `cargo semver-checks check-release -p happenstance-sync --baseline-version 0.6.0` reports
  nothing, or only lints with a T row in the 0.7.0 trace.
- [ ] Freeze, after the owner's yes: a new ADR atom that moves SY-7 and SY-10 to FROZEN, with the spec
  markers, the ledger rows, handover and the CHANGELOG updated.

## Decisions (H-16)

| ID | Decision | Options | Recommendation | Door | Decided by | Outcome |
|---|---|---|---|---|---|---|
| D-441-1 | Delete the refused-push check from a conformance rule (a test deletion, H-12/H-16) | A) delete it, and reorder the walk to `Edge::step`'s order; B) keep a refused step and give the store an act at it; C) keep the no-op step without the check | **A**. No store can fail the check (see above). ADR-0107 §2.1 recommends A. | two-way (testkit-internal, restorable) | owner | open |
| D-441-2 | Separate PR or fold into #450 | A) separate, land first; B) first commit of #450's PR | **A**. No API and no ADR, so it is not held up by #450's owner decisions | two-way | owner | open |
| D-450-1 | How the hook attaches to `Edge` | A) a defaulted type parameter `Edge<R, A = Accept>` with `with_adjudicator` (one `step`/`run`, and the role shows in the type); B) per-step methods `step_with(peer, store, &rule)` and `run_with`, which leave `Edge` untouched but double the step API and let one edge be stepped under two rules | **A**: assignment is per-edge configuration (SY-7), and the spike shows existing code compiles. B is the fallback if semver-checks flags A | **one-way** once published (public type shape) | owner | open |
| D-450-2 | Mount on the push side too (`StoreSyncPeer`, `serve_sync`) | A) yes, in #450; B) `Edge` only, with `StoreSyncPeer` as a follow-up | **A**. Without it, a hub served over `serve_sync` cannot adjudicate, and SY-10 would freeze with one direction of a served edge configurable only by a hand-written wrapper peer. Note that `serve_sync<S, I>` names `StoreSyncPeer<S, I>`, so generalising it to `<S, I, A>` adds a generic parameter to a pub fn. That is a turbofish break, likely flagged by semver-checks. The non-breaking route is a new `serve_sync` sibling, or acceptance as a 0.7.0 break with a T row (0.x minor) | **one-way** | owner | open |
| D-450-3 | Hook contract | A) decide-only, per group with `earlier`, returning `Vec<Event>`; B) per batch, returning one compensation per group; C) write-capable (the hook appends) | **A**. C makes SY-2's forbidden state representable. B pairs results by index, which is fragile | **one-way** (published trait) | owner | open |
| D-450-4 | What "mounts the rule" means for the freeze | A) the testkit rules express their merge rules through the shipped `Adjudicate`/`Accept`, and the runner half is proven in `happenstance-sync/tests` by shipped-vs-`MiniEdge`; B) move `directional_merge_rules_compose` to drive a real `Edge` over `StoreSyncPeer`, in the write-group family (whose store is `WriteGroupStore + IngestStore`) | **A**. It keeps SY-8 ("a rule that needed two peers would be testing the runner") and keeps the rule in the macro adapters already mount. B removes a rule from `ingest_store_conformance!`, a testkit change consumers see | **one-way** for B, two-way for A | owner | open |
| D-450-5 | Ship a family-keyed assignment type (tag key → rule) | A) no, the hook impl decides which families it holds; B) ship `ByFamily` | **A** now. B is additive later | two-way | owner | open |
| D-450-6 | Additive-only, or use 0.7.0's 0.x-minor licence to break | A) additive only (the brief's rule); B) allow breaks with T rows | **A**. Flag any lint semver-checks raises | one-way | owner | open |

## Risks

- **The reorder could change the falsification test's reversal order** (`topology/tests.rs:132-137`
  asserts `[near, far]`). It should not. The walk was refused push (a no-op), pull, push, pull. It becomes
  pull, push, pull: the same ingests in the same order, with only the no-op gone. The topology test is the
  check.
- **`AdjudicatesOnIngest`'s message could move** if the first ingest it trips on changes. The exact pin
  (`fails_exactly`) catches this, and the message assertion at `wrong_ingest_stores.rs:1363` shows which
  one it is.
- **SY-34 and SY-33 are FROZEN with unwritten rules** (`spec/SPECIFICATION.md:8389`, `:10686-10687`).
  After #441, nothing in the testkit even claims SY-34. The two runner tests hold it, and its rollback
  cannot be expressed through the ports. A FROZEN clause's Rule line changes only by ADR, so this is
  **reported for a new issue, not fixed here**. The question to put to the owner: point SY-34's Rule line
  at the runner tests by ADR, or write `a_refused_spoke_retains_its_log` at runner grain.
- **#450, `earlier`:** a hook that ignores `earlier` reintroduces the in-batch blind spot. T-450-4 and
  the `IgnoresEarlier` defect are the checks.
- **#450, semver:** moving `step` and `run` to a wider impl, and the defaulted parameters. The spike
  covers inference. Run semver-checks against 0.6.0 before the PR (handover's step-2 command, with
  `--baseline-version 0.6.0`).

## Size (lower bounds)

- **#441:** 3 files. `topology.rs` loses about 35 lines of code and gains about 15 of docs.
  `ingest_suite.rs` changes about 12 rustdoc lines. `CHANGELOG.md` gains about 3. The net is roughly
  −20 lines.
- **#450:** at least 12 files and about 900 lines.
  - `happenstance-sync`: `runner/adjudicate.rs` (about 150 lines), `runner.rs` (about 60), `error.rs`
    (about 30), `classify.rs` (about 10), `store_peer.rs` (about 80, if D-450-2 is A), `serve.rs`
    (about 20).
  - Tests: `directions.rs` rewritten (about 150), a new `tests/runner/adjudication.rs` with `MiniEdge`
    (about 250), `flavours.rs` (about 30).
  - Testkit: `topology.rs` and `adjudication.rs` mount (about 80).
  - Spec, ledger, handover, CHANGELOG, and the freeze ADR atom (about 115).

## Progress

- 2026-11-15: explored and planned. No source edited. The only build was the scratchpad spike
  `scratchpad/spike/s.rs` (rustc, no cargo), which compiled clean.

## #450 implementation record (2026-11-15)

Owner decisions recorded (binding): D-450-1 A, D-450-2 A **through a new sibling** (`serve_sync_adjudicated`,
`StoreSyncPeer::with_adjudicator`; `serve_sync` keeps its two type parameters), D-450-3 A, D-450-4 A,
D-450-5 A, D-450-6 A (additive only). #441 merged as d136ddc3.

Extra from #441's review: a `wrong_ingest_stores` mutant that upserts its own claim, so only `judge`'s
`holds` check in `topology.rs` fails it.

### Shape as built
- `runner/adjudicate.rs`: `Adjudicate` / `SendAdjudicate` (trait_variant), `Accept` (`SendAdjudicate`,
  `Error = Infallible`). `impl Classify for Infallible` in `classify.rs`.
- `Edge<R, A = Accept>`, field `rule: A`; constructors stay on `Edge<R>`; accessors/builders on
  `impl<R, A>`; `step`/`run` on `impl<R, A: Adjudicate>`; `with_adjudicator` on `Edge<R>`.
- `StepError<P, R, I, A = Infallible>` + `Adjudicate(#[source] A)`; `Phase::Adjudicate`;
  `StepFailure<P, S, A = Accept>`.
- `StoreSyncPeer<S, I, A = Accept>` + `with_adjudicator`; `StorePeerError<R, W, A = Infallible>` +
  `Adjudicate(A)`; hook after `admit`, before the one ingest. `ServeError<R, W, A = Infallible>`;
  `serve_sync_adjudicated`. `SendPeer<StoreSyncPeer<S, I, A>>` for `A: SendAdjudicate + Sync`.

### Test list progress
- [x] T-450-1 semver canary: existing runner suite unchanged + `tests/runner/unchanged.rs` compile test
- [x] T-450-2 hub-driven edge adjudicates what it pulls
- [x] T-450-3 same under Accept: no reversal
- [x] T-450-4 two contested claims in one batch: one reversal via `earlier`
- [x] T-450-5 hook failure per class; token does not advance; nothing lands
- [x] T-450-6 declined push to an adjudicating store peer leaves the far log unchanged
- [x] T-450-7 rules over `EdgeRunner` vs shipped `Edge` and `MiniEdge` (4 defects)
- [x] T-450-8 testkit mount through `Adjudicate`/`Accept`; pins exact
- [x] upsert-own-claim mutant fails SY-10 only via `holds`
- [x] flavours: hooked edge spawnable in generic code; `!Send` hook on a bare edge
- [ ] T-450-9 semver: cargo-semver-checks not installed here; reasoned additive (see report), CI/owner to run

### Progress
- Red (T-450-1..7, compile): `cargo test -p happenstance-sync --all-features --test runner --no-run` →
  `error[E0432]: unresolved import happenstance_sync::runner::Adjudicate` and
  `unresolved import happenstance_sync::serve_sync_adjudicated`.
- Green: runner target 67/67 after `Edge<R, A>`, `StoreSyncPeer<S, I, A>`, `serve_sync_adjudicated`.
- Red (T-450-4 on the shipped runner, mutation): `compensations` handed `groups.get(..at.min(0))` →
  `the_shipped_edge_passes_every_rule` "one batch of two claims earned 0 reversals". Restored, green.
- flavours: `an_edge_steps_under_a_not_send_rule`, `a_spawned_hooked_edge_steps_a_hooked_store_peer_from_generic_code` green (11/11).
- T-450-8: testkit `seat_rule.rs` (`SeatRule: Adjudicate`); `topology.rs` and `adjudication.rs` mount it or the shipped `Accept`.
- Red (upsert mutant): with `judge`'s `holds` check disabled, `directional_merge_rules_compose_fails_a_store_that_remints_its_own_fact_when_contested`
  → "must reject its mutant left: Ok(Ran) right: Err(\"SY-10\")". Restored, green (35/35).
- Testkit: every test target green one at a time (25 targets), lib 53/53.
- Gate subset (disk 7.5 GB < 15 GB, so not the full `cargo xtask ci`): fmt ok; clippy -D warnings
  (sync + sync-testkit, all targets, all features; sync default features) ok; rustdoc -D warnings
  (all features, and sync default) ok; wasm32 check of sync, sync-testkit, cloudflare+sync ok;
  `cargo xtask spec-trace` ok (spec edits line-neutral); `cargo xtask lints` ok; `cargo test -p xtask` ok.
- Review fixes (approved with minors):
  - F1: red `error[E0432]: unresolved import super::judge` for
    `every_group_is_judged_once_with_its_own_prefix_and_paired_with_its_answer` (0, 1, 5 groups);
    green with `judge` -> `Vec<Judged>` (group paired with its answer, prefix and group from one
    `split_at_checked`), both ingest sites building `IngestGroup`s from the pairs. lib 61/61.
  - F2: `unchanged.rs` adds an unannotated `Edge::new()` stepped in generic code, `resuming`/`restored`
    chained with `with_responder`/`with_buffer`, and `SendPeer<StoreSyncPeer<S, I>>` under 0.6.0's bounds.
  - F4: `Phase`'s history moved to a `//` comment. F5: `with_adjudicator` says Debug/Send follow the rule.
  - clippy and rustdoc -D warnings (sync + sync-testkit) ok; runner 67, flavours 11, wrong_ingest_stores 35, doc 22+2.
