---
id: kb-open-question-es-6-unwritable-rule-001
title: ES-6 is frozen and names a rule that cannot be written
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-6 is FROZEN and names store_error_crosses_a_join_handle as its rule, marked (new) and
  rendered with † where the legend reads "does not exist yet". The identifier occurs as no fn
  anywhere in the workspace — only in prose and in comments, one of which states the rule is
  unwritable against today's port for every adapter, with SendStoreWithLocalError as the probe.
  spec-trace's check 4 skipped clauses whose rule was marked (new) or †, and that exemption had no
  expiry, so a rule scheduled forever was indistinguishable from one scheduled for next week. ADR-0009 is accepted and makes the rule writable, so this may be a scheduling gap rather
  than a design gap — but the clause is frozen and names an unwritten rule. Settled by writing the
  rule against ADR-0009's marker, or by deciding that a † with no owning phase is a hard failure.
  Found independently the same day by references/evaluation/review-citation-drift.md §2. ADR-0008
  and ADR-0009 are now imported as decision atoms: ADR-0008 adds that the rule, when written, must
  assert on the future's Output and not on the future, since a future-only check is decorative
  against a !Send error, and ADR-0009 supplies the ThreadSafeEventStore marker the rule's bound
  would name. Narrowed and not closed on 2026-08-20 by ADR-0023: ADR-0009's prediction was judged
  against a real !Send error carrying a live JavaScript value — the first runtime that can produce
  one — and it held, a caller recovering constraint violation from transport fault without Error
  carrying Send + Sync. store_error_crosses_a_join_handle is still unwritten, still unowned, and
  still named by a FROZEN clause; the scheduling gap this atom describes is unchanged.
  Updated 2026-09-07 on three counts. The escape hatch changed shape: schedules_new was retired
  from spec-trace, check 4 now guards on has_suite alone, and every unresolvable rule name must be
  declared in UNRESOLVABLE_RULE_NAMES with a reason — ES-6's own entry says the rule is unwritable
  against today's port, "a finding rather than a schedule", so sub-question 2 is half-answered:
  the permanent silence is gone, an expiry or an owning phase is still not required. A fourth
  sub-question arrives from the driver re-export pass: ES-6 endorses wrapping a driver error but
  says nothing about whether the wrapped type is part of the promise, which is the fact a sealing
  option depended on and ADR-0044 did not supply. And the §7.2 citation is repointed to the live
  table row rather than by the intake's mechanical offer, which was anchored on the wrong line.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-decision-0008
  - kb-decision-0009
  - kb-reference-port-traits-compiled-findings-001
  - kb-open-question-es-38-and-gap-read-unowned-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-decision-0023
  - kb-reference-spec-trace-unresolved-declarations-001
  - kb-open-question-no-ps-rule-name-resolved-001
  - kb-open-question-dagger-convention-vs-maturity-markers-001
  - kb-decision-0044
  - kb-decision-0045
  - kb-playbook-anchoring-citations-001
  - kb-open-question-exact-anchor-residue-001
  - kb-decision-0050
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/0008-one-derivation-for-both-ports.md
  - .kb/_intake/0009-error-send-sync.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/src/send_shape.rs
  - references/evaluation/review-citation-drift.md
  - references/adr/0008-one-derivation-for-both-ports.md
  - references/adr/0009-error-send-sync.md
  - .kb/_intake/es-6-verdict-against-adr-0009s-prediction.md
  - .kb/_intake/remediation-2026-09-04-briefs/prose-guard-retired-and-what-it-owes.md
  - .kb/_intake/remediation-2026-09-04-briefs/stated-only-defects-and-the-reopen-must.md
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
last_reviewed: 2026-09-07
---

# ES-6 is frozen and names a rule that cannot be written

## What is true today

ES-6 (`spec/SPECIFICATION.md:2677`) is `[FROZEN]` and its `Rule:` field names
`store_error_crosses_a_join_handle`, marked **(new)** (`SPECIFICATION.md:2720`). §7.2's generated
table renders it with `†` (`SPECIFICATION.md:9271`), where the legend defines `†` as "does not
exist yet". That identifier occurs as no `fn` anywhere in the workspace. It occurs only in prose:
twice in `RUNBOOK.md`, once in `.kb/decisions/0008`, three times in `.kb/decisions/0009`, three times in
comments in `happenstance-cloudflare`, and in the specification itself. One of those comments
(`crates/happenstance-cloudflare/src/lib.rs:92`) states the rule "is unwritable against today's
port for *every* adapter, not merely for this one," and the probe backing that claim is
`SendStoreWithLocalError` (`crates/happenstance-cloudflare/src/send_shape.rs:94-98`), whose doc
comment reads: "If this compiles — and it does — then the *derived* flavour does not imply a
`Send` error either."

The gate did not catch this, because check 4 skipped any clause whose rule was declared new or
marked `†` — `if c.schedules_new || !has_suite(&c.id) { continue; }` — where `schedules_new` was a
substring test over the clause's own `Rule:` prose. That escape hatch was deliberate and correct in
general, a clause may legitimately schedule a rule the current phase has not written yet, but it
had no expiry, so a rule scheduled forever printed identically to one scheduled for next week.

**That mechanism no longer exists, and what replaced it says ES-6's quiet part out loud.** The
prose guard was retired: check 4 now guards on `!has_suite(&c.id)` alone
(`xtask/src/spec_trace.rs:822`), `schedules_new` is gone with it, and its own doc comment records
why the switch could not be repaired in place — the sentence explaining that a rule was unwritten
*was* the switch that stopped the checker looking (`xtask/src/spec_trace.rs:795-813`). A clause's
rule name is now either resolvable or declared in `UNRESOLVABLE_RULE_NAMES`
(`xtask/src/spec_trace.rs:1006`) with the reason nothing can find it. ES-6 has an entry, and it is
one of the fifteen `Scheduled` ones whose owner is not a phase:

> `store_error_crosses_a_join_handle` — "nothing, yet:
> `crates/happenstance-cloudflare/src/send_shape.rs` argues it is unwritable against today's port,
> which is a finding rather than a schedule" (`xtask/src/spec_trace.rs:1167-1174`).

`kb-reference-spec-trace-unresolved-declarations-001` carries the census the entry came out of, and
is where the other fourteen live. This moves sub-question 2 without closing it. The permanent
*silence* is gone — the exemption is now data in the gate's own binary, has to be written by a
human, and names ES-6 as a finding rather than a queue position. What is still absent is the thing
the sub-question actually asks for: nothing requires an entry to carry an expiry or an owning
phase, so ES-6's declaration can stand forever exactly as the `†` did, one grep away rather than
zero.

A partial resolution already exists and has not been executed: ADR-0009 (accepted, in `.kb/decisions/`)
settles the underlying question that `Error` keeps `core::error::Error + 'static` on both ports and
both flavours, with the stronger property becoming a marker trait declared downstream. That makes
the rule writable, and the wrong implementation it must reject already exists in the tree. So this
may be a scheduling gap rather than a design gap — but the clause is `[FROZEN]` today and still
names an unwritten rule.

That premise has since been observed under execution rather than only reasoned about. Phase 9's
Cloudflare adapter is the first runtime in the workspace that can produce the case ADR-0009 named —
an error type that is `!Send` because it carries a live JavaScript value — and `kb-decision-0023`
records the verdict: the decision holds, and this adapter is the evidence for it rather than the
exception to it. Four reconstruction tests in `crates/happenstance-cloudflare/src/lib.rs`'s
`es6_reconstruction` module, executed on `wasm32-unknown-unknown` and pinned in
`xtask/src/proof.rs`'s executed-target registry, show a caller recovering the one fact they must
branch on — constraint violation versus transport fault — from what `append` hands back, with no
`Send + Sync` bound on `Error` and with no public item added to reach it. The suite was demonstrated
capable of failing: two wrong classifiers were compiled into the real `classify_write`, and the
evidence-discarding shape was rejected by two tests, the flattening shape by three.

`references/evaluation/review-citation-drift.md` §2 reports the same finding independently, written
the same day from the `standards/rust/` work with no knowledge of this pass, and records the
identifier occurring in three comment locations in the crates — convergent evidence that this is a
real gap and not an artifact of one reading.

One citation in the paragraph above was repaired here rather than by the repoint offered to it, and
the difference is worth recording because it is the hazard `kb-decision-0045` and
`kb-playbook-anchoring-citations-001` are about. The `[REOPEN]` lane's brief listed this atom's
`SPECIFICATION.md:8588` as drifting to `8625`, anchored on the line `nothing. Batch shape's tick is
the *one-sided* one`, and reported the anchor as unique and the repoint as mechanical. Both are
true and the result is still wrong: that line is §7's batch-shape prose, not §7.2's table row, and
checking the revision the citation was written against (`76e9424`, 2026-08-19) shows the ES-6 row
at `9080` while `8588` already held unrelated prose. The citation was mis-anchored before it
drifted, so following the anchor faithfully preserved the wrong target. The row is at `9271` today
and that is what the body now cites.

## What is not decided

Whether the rule gets written against ADR-0009's marker — after which ES-6's `(new)` and `†` come
off and its `UNRESOLVABLE_RULE_NAMES` entry is deleted, which is what check 4 now asks for by name
— or whether a broader decision is taken about clauses whose rules have no owning phase at all.

Both ADRs are now imported as decision atoms — `kb-decision-0008` and `kb-decision-0009` — and
between them they narrow this question without closing it. ADR-0008 was deliberately neutral on
ES-6 but recorded the shape of the rule for whoever writes it: the derived flavour genuinely admits
a `!Send` error, the failure lands at a call site demanding `F::Output: Send` rather than at the
declaration, and therefore `store_error_crosses_a_join_handle` **must assert on the future's
`Output`** — a rule that only checks the future is `Send` passes against a `!Send` error and is
decorative. ADR-0009 then supplies the bound the rule would name,
`ThreadSafeEventStore: SendEventStore<Error: Send + Sync>` with a blanket impl, compiled downstream
so the contract crate need not grow anything; it also names the rule's obligation as belonging to
an **opt-in marker-bound rule group** and moves ES-6 from `[DEFERRED]` to `[FROZEN]`. That is the
provenance of the `[FROZEN]`-with-an-unwritten-rule state this atom describes: the freeze and the
rule arrived in the same decision, and only the freeze was executed. Neither ADR writes the rule,
neither assigns it an owning phase — ADR-0009 explicitly leaves the suite to phase 3 and the
surface question of shipping the marker in `happenstance-core` to phase 4 — so the gap is a
scheduling gap with a named remedy and no owner, which is exactly the condition sub-question 2 is
about.

`kb-decision-0023` narrows the question the same way and closes none of it, because what it judged
was ADR-0009's **prediction** and not ES-6's **rule**. The prediction — that the absent bound would
cost a caller something on the first runtime able to produce a `!Send` error — was tested and held.
`store_error_crosses_a_join_handle` is still unwritten, still unowned, and still named by a
`[FROZEN]` clause: writing it is a testkit change against ADR-0009's marker, phase 9 did not take
it, and ADR-0023 says so in its own terms rather than leaving it to be inferred. If anything the
remedy is now more attractive rather than less — the derived flavour's tolerance of a `!Send` error
has been exercised on a real one, so the rule has a live subject as well as
`SendStoreWithLocalError` as its negative probe — but the missing owning phase and both general
sub-questions below are exactly where phase 8 left them. Only the exemption's *mechanism* has
moved, from a word in the clause's prose to an entry in the gate's own table.

A second thing ES-6 does not settle surfaced from a direction that had nothing to do with the
missing rule. The driver re-export pass went looking for a clause governing whether a published
adapter re-exports the driver crate whose types appear in its public signatures, and found ES-6 as
the nearest one — pointing the *other* way, because ES-6 endorses the wrapping, citing
`SqliteEventStoreError`'s "twelve real variants over `rusqlite::Error`, `JoinError`,
`TryCurrentError` and the crate's own decode failures" (`spec/SPECIFICATION.md:2688-2690`) as the
instrument that made the clause decidable at all. What ES-6 does not say is whether the *wrapped
type* is part of the promise the `#[non_exhaustive]` error enum makes. That is not idle: the
sealing option in that pass — wrap `rusqlite::Error` behind an opaque value enum and stop naming it
publicly — depended entirely on the answer, and `kb-decision-0044` settled the re-export question
without supplying it, so the option was ruled out on cost-of-delay rather than on ES-6's meaning.
`#[non_exhaustive]` protects the *addition* of variants, not the *change* of a variant's payload
(`crates/happenstance-sqlite/src/event_store.rs:1202`), which is why the question has a semver
consequence and not only a documentation one.

## What forces it

Nothing enforces a deadline today; the exemption's lack of expiry is exactly the mechanism that
lets this sit indefinitely without failing the gate. The intake's own judgment is that this is
worth deciding regardless of ES-6 specifically: **a `†` with no owning phase should probably be a
hard failure, and today it is silence.** That is a standing risk for every future clause marked
`(new)` or `†`, not only this one.

The prose guard's retirement lowers the cost of that risk without removing it. An exemption is now
a written entry a human had to add, carrying a reason a reader can attack, rather than a `†` in a
sentence — and ES-6's entry attacks itself, saying in the gate's own source that the schedule is a
finding. But an entry with a reason and no owner still never fails, and there are fifteen of them.
Whether the dagger convention survives at all, now that the guard it switched is gone, is
`kb-open-question-dagger-convention-vs-maturity-markers-001`'s, not this atom's.

## Ordered sub-questions

1. Is ES-6 specifically closed first — write `store_error_crosses_a_join_handle` against
   ADR-0009's marker trait, using `SendStoreWithLocalError` as the negative probe it must reject —
   before the general policy question is settled?
2. Separately, should an `UNRESOLVABLE_RULE_NAMES` entry — the declaration that replaced
   `schedules_new` — be required to carry an expiry or an owning phase, so an exemption with
   neither becomes a hard failure rather than a permanent, well-documented pass?
3. If the general policy changes, does it retroactively require an owning phase for every other
   clause in the specification currently marked `(new)` or `†` — and, now that the list is
   enumerated rather than implicit, for the other fourteen `Scheduled` entries — and who audits
   it once the policy exists?
4. Is the *wrapped* driver type part of what ES-6 promises? The clause endorses wrapping and cites
   `SqliteEventStoreError`'s variants over `rusqlite::Error` as the instrument that made it
   decidable, but says nothing about whether a caller may rely on that payload staying. Answering
   yes closes the sealing option `kb-decision-0044` left unruled-on; answering no makes a
   `#[non_exhaustive]` adapter error's payload changeable at a minor version, and both answers are
   an ADR against a `[FROZEN]` clause rather than a note.
