# What the number settles, claim by claim

Every verdict below is read off `results/raw/census.txt` and
`results/raw/defects.txt`. Where a claim is **qualified** rather than confirmed,
that is stated first, because a measurement that corrects the review is a better
result than one that agrees with it.

## L1.L1-1 — no read composes forwards `from` with `limit` · **CONFIRMED**

`ForwardPagingBudgetStore` — `limit` applied only when `from` is absent, the
backwards branch left correct — **fails 0 of 89 rules**.

The claim predicted this store passes "all 45 rules in this half and all 90 in
the suite". Measured: it passes all 89 the enumeration holds, with 6 skips it
shares with both conformant controls.

The defect is not theoretical.
`defect_is_real.rs::forward_paging_budget_is_ignored` appends five events and
issues the read a projection runner issues — `from(2)`, `limit(2)`. The correct
core returns 2 events; this store returns **4**, and reports success. The same
store's backwards read of `from(4).backwards().limit(2)` returns 2, so the defect
really is confined to the forward branch and the store is not simply broken at
paging.

## L1.L1-2 — over-claiming a capability is unchecked · **CONFIRMED, and sharper than filed**

`NoopReopenFixture` — `REOPEN: Capability::SUPPORTED`, `async fn reopen(&self) {}`,
over a completely correct but entirely volatile store — **fails 0 of 89 rules**.

The sharpening is in the skip counts, and it is visible in the raw output with no
interpretation. Every other subject in the run skips **6** rules. This one skips
**3**, because the three it does not skip are the three a declined `REOPEN` would
have removed:

* `acknowledged_writes_survive_a_reopen`
* `reopened_store_does_not_reissue_an_event_id`
* `recorded_time_survives_a_reopen`

They ran. They passed. There is no durable medium anywhere in the subject: after
`fixture.reopen().await`, `defect_is_real.rs::a_noop_reopen_closes_nothing` reads
the event back **through the handle taken before the reopen**, which a store that
had genuinely closed could not answer from.

So the fixture that over-claims does not merely go unpunished — it converts three
skips into three green rules. An adapter author reading `86 passed` has a
*stronger*-looking result than the honest fixture beside it, which declines the
capability and gets `83 passed, 3 skipped`. The incentive points the wrong way.

## L3.L3-01 — the read path's error arm has no rule, no mutant and no fixture seam · **CONFIRMED, and the diagnostic locates it precisely**

`SwallowedReadFaultStore` —
`let Ok(page) = fetch().await else { return Poll::Ready(None) };` — **fails 0 of
89 rules**.

The diagnostic row is what makes this a claim about the *seam* rather than about
the rules. The identical store with the fault armed by hand at `open` fails **22**
rules — `query_all_matches_every_event`, `read_from_is_inclusive`,
`event_ids_are_unique_within_a_store`,
`store_accepts_the_guaranteed_minimum_batch_size` and eighteen more. So:

* the suite is **not** blind to the consequence of a swallowed read fault; and
* the suite has **no way to produce one**.

`Fixture` carries `MID_BATCH_FAULT` and `arm_mid_batch_fault` for the write path
and no read-path analogue, so `SwallowedReadFaultFixture::arm_read_fault` had to
be written as an *inherent* method that no conformance rule can reach. That
asymmetry is the claim, measured: the missing thing is one `Capability` and one
defaulted `fn`, and everything downstream of it already works.

`defect_is_real.rs::a_swallowed_read_fault_looks_like_the_end_of_the_stream`
shows what a consumer meets: five events in the store, `collect` returns `Ok` over
**two**, and the port's per-item `Err` arm — which exists exactly for this — is
never used.

## F2.F2-5 — ES-22 is answered trivially by every adapter in the tree · **CONFIRMED, and the rule itself is exonerated**

This is the entry the count flatters and the detail corrects, so read the detail.

`StagedCommitStore` fails 0 of 89, and that is the *right* answer: it is not a
wrong implementation. It suspends inside `append` between two statements and
commits atomically at the end — the interactive-transaction shape done correctly,
which is what `happenstance-postgres` and `happenstance-neon` will be.

What it measures is the branch.
`defect_is_real.rs::the_es22_arm_a_store_takes_depends_on_whether_it_suspends`
reproduces `dropped_append_future_leaves_no_partial_batch`'s own sequence — build
the append future, poll it once, drop it — outside the rule, and reads the counts:

| Store | first poll | events landed | arm ES-22 takes |
|---|---|---:|---|
| `LogStore` (correct core, no `.await` in `append`) | completed | 3 | `landed == batch.len()` — the membership loop |
| `StagedCommitStore` (suspends between statements) | suspended | **0** | `landed == 0` — `snapshot_of(&after) == snapshot_of(&before)` |

The sharpened claim was that the `landed == 0` arm "has never executed against any
store in the workspace". This experiment executed it. **The arm is correct; it had
simply never run** — which is a better outcome for the remediation than the claim
assumed, because it means the fix is a store to point the rule at rather than a
repair to the rule.

The other half stands unchanged and was not in question: the testkit's own
`YieldingRowAtATimeStore` already fails this rule, so ES-22 is non-vacuous in the
rejection direction and vacuous only in the *acceptance* direction — which is
where a real adapter lands.

## L2.L2-01 — the model family never generates `ReadOptions::to` · **NOT ADDRESSED**

Out of scope for this instrument, and it should not be read as touched by the
number. The census drives the **event-store** family (89 rules); the model family
is a separate enumeration behind the testkit's `proptest` feature, and none of the
four subjects here is a `to`-vs-`limit` ordering defect. That claim's own verifier
already concluded half of it — "applies `to` after `limit`" — is not observable
through the port at all.

What this experiment *does* corroborate is its premise: `to` is thin on the
ground. Of the 21 rules the injected controls fail, none involves `to`, and the
read-option rules that do (`read_to_is_inclusive`,
`read_from_and_to_bound_a_closed_window`,
`read_to_under_backwards_bounds_the_older_end`) appear only in the armed
diagnostic's list, where they fall for a reason unrelated to `to`.

## L3.L3-03 — the concurrency family's conformant control is a convention · **NOT ADDRESSED, and corroborated indirectly**

Also out of scope: the concurrency family is a third enumeration
(`for_each_concurrency_rule!`), driven on real threads, and this crate's subjects
are `Rc`-backed and single-threaded by design.

It is corroborated in one narrow, honest way. This experiment had to *build* the
control-by-construction that L3-03 says the concurrency family lacks: `Kind` is an
associated const on `Subject`, so a control cannot be silently reclassified by
deleting a row, and `tests/census.rs` asserts over `Kind::Control` and
`Kind::InjectedControl` separately rather than over "the ones with no failures".
That is the same repair L3-03's remediation proposes for `Racer`, arrived at
independently by someone who needed it in order to trust their own number.

## What the number does not show

* **It measures one family.** 89 rules of the event-store family. It says nothing
  about the projection, model or concurrency families, and a store that passes
  here is not certified by anything those would catch.
* **Four subjects, not a survey.** These are the four the review named. "Four of
  four pass" is not "the suite catches nothing": the same harness rejects the
  injected arms 21 times over, and the testkit's own registry holds forty mutants
  it does catch.
* **In-memory, single-threaded, `Rc`-backed.** Every subject is `MemoryEventStore`
  wearing a different hat — the exact hazard `CLAUDE.md` names about freezing a
  port against four adapters of one storage shape. A real adapter can fail these
  rules for reasons no store here can express.
* **(d) is not evidence of a hole.** Counting `StagedCommitStore` among "four that
  pass" is arithmetically true and rhetorically misleading. Three genuinely wrong
  stores pass; the fourth is a correct store whose value is the arm it executes.
* **`arm_read_fault` is armed by the experiment, not by the store.** Subject (c)
  with no fault armed is byte-for-byte a correct store, and a reader is entitled
  to say it therefore passes trivially. That *is* the claim — there is no seam for
  a rule to arm it through — and the diagnostic row is what keeps it from being
  circular.
* **It is a census, not a proof of maximality.** "Fails nothing" is measured over
  the 89 rules that exist at `56ef6c5`. It is not a claim that no rule *could*
  catch these stores.
