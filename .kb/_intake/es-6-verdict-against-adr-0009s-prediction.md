# ES-6 — ADR-0009's prediction, judged against a real `!Send` error

**Staged for `/redkiln:kb-ingest`. Not an atom.** This is a **verdict on a
prediction**, recorded as new content. `.kb/decisions/0009-error-send-sync.md` is
accepted and immutable and must be **byte-identical to `main`** after the wave:
if any claim here is read as contradicting it, the only legal mechanism is a
**superseding** atom, never a clarifying line in its body.

**Intended layer.** Either a section of the ADR-0023 decision atom or its own
`kind: note` atom under `.kb/reference/` — the adjudicator's call, biased per
`.kb/_intake/README.md` toward folding it into ADR-0023 rather than spawning a
near-duplicate, since it is settled by the same run.

---

## What ADR-0009 predicted, and what the project's own AC wording got wrong

ES-6 was **settled, not deferred**: `Error` keeps `core::error::Error + 'static`
on both flavours, and the stronger `Send + Sync` property lives in a downstream
`ThreadSafeEventStore` marker. Project AC-005's wording — *"the bound added, or
ADR-0009's deferral confirmed"* — predates the atom's acceptance and is wrong on
its face; **correct it in the record** rather than answering the question it asks.

What ADR-0009 actually named was an **asymmetry**: an error type that is `!Send`
because it carries a live JavaScript value would be the case where the absent
bound costs a caller something. Phase 9 is the first runtime that can produce one.

## The verdict

**ADR-0009's decision holds, and this adapter is the evidence for it rather than
the exception to it.** A caller recovers the one fact they must branch on —
constraint violation versus transport fault — from what `append` hands back,
without `Error` carrying a `Send + Sync` bound.

The artefact is committed and is `caller-visible-error-verdict`'s
(HS-S0052): four reconstruction tests in
`crates/happenstance-cloudflare/src/lib.rs`'s `es6_reconstruction` module, executed
on `wasm32-unknown-unknown` and named in `xtask/src/proof.rs`'s executed-target
registry so they cannot be renamed or emptied in silence.

- `constraint_violation_reaches_the_caller_as_condition_violated` — asserts
  `is_condition_violated()` **and** the absence of a `Store` arm.
- `transport_fault_reaches_the_caller_distinguishably` — the
  `Store(Sql(Thrown(..)))` shape plus the recovered text, read through `Display`
  and the public `source` chain.
- `the_distinction_is_reachable_from_outside_the_crate` — a
  `classify_like_a_consumer<S: EventStore>` that binds the **bare** flavour and
  reads only `pub` items.
- `an_evidence_discarding_classifier_is_rejected` — the negative control.

## Why this is not a green suite dressed up as an answer

Every conformance rule asserts on the success path or on a store-produced
`AppendError`, and **none of them reads an adapter error's contents**. A green
suite exists whether stringifying a `JsValue` loses information or not, which is
exactly why the phase's proof artefact demanded a second half. The second half is
what was built, and it was demonstrated to be capable of failing: two wrong
classifiers were compiled into the *real* `classify_write` and the suite re-run —
the evidence-discarding shape was rejected by two tests and the flattening shape by
three.

## The consequence for the mapping, which is why this belongs beside ADR-0023

The thrown value is **retained rather than stringified at the boundary**
(`JsThrow`, not `StringifiedThrow`). That is a mapping decision, and this verdict
is the evidence for it: stringify early and the caller-visible distinction is gone,
and no bound on `Error` — `Send + Sync` included — would bring it back.

## What is explicitly **not** claimed

`store_error_crosses_a_join_handle`, ES-6's own named rule, is still **unwritten
and unowned**. Writing it is a testkit change against ADR-0009's marker and it is
not this project's; the open question
`.kb/open-questions/es-6-names-an-unwritable-rule.md` already records it and is
**not** resolved by this verdict. Do not let the wave read this document as
answering that question.

No public item was added to reach this verdict — a fallback accessor was provided
for in case the real bindings left the fact unreachable from outside the crate, and
it was not needed, which is the strongest available answer to a design that records
no surface.
