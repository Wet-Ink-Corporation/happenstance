# Re-derivation — the frozen documentation MUSTs, against the document as it stands

Evidence, not test code. AC-001 asks for the set to be **re-derived** rather than copied from
either of the two statements that disagree about it, and for the nine-versus-eight arithmetic
to be closed by name. This is that record; `FROZEN_DOC_MUSTS` in
`xtask/src/lint_narrative.rs` is a transcription of it, and where the two disagree **this
record wins**.

Read at `spec/SPECIFICATION.md` as it stands on `initiative/docs-that-teach`: 200 clauses, 139
`[FROZEN]`, 49 `[PROVISIONAL]`, 10 `[DEFERRED]`, 2 `[NON-NORMATIVE]` (`cargo xtask spec-trace`).

## The rule applied

A candidate is **pinned** when

* **(a)** its clause is `[FROZEN]`, **and**
* **(b)** its obligation falls on **the contract's own documentation** — not on an adapter's,
  not on a fixture's, and not on this specification's own prose,

* **(c)** the discharge **exists today**, because pinning an undischarged obligation would land
  the gate red and the only repair would be editing a `happenstance-core` doc comment —
  HS-P0023's boundary, governed by
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md`.

All three conditions are stated in `FROZEN_DOC_MUSTS`' own comment, beside the array they
classify, along with the fact that ES-26, PS-31 and PS-36 fail **only** (c). A rule stated here
and an array over there is the defect this pin exists to prevent, one level up: a reader applying
the comment's rule to the entries beneath it must get the entries' answer.

Everything the candidate scan finds and the rule does not reach is **excluded**, with a
one-line reason on the entry.

## What the two existing statements say, and why they do not close

`RUNBOOK.md:3830-3845` records "discharged **nine** documentation MUSTs".

`references/evaluation/phase-4-5-reconciliation.md:119-142` is the evidence behind it, and it
lists **eight rows**:

| # | Row | What it actually is |
| - | --- | ------------------- |
| 1 | ES-23 — an explicit `# Cancellation` section on `append` | a clause discharge |
| 2 | ES-24 — at-most-once under verbatim reissue, with all three limits | a clause discharge |
| 3 | VT-15 — equality is byte equality and nothing is normalised | a clause discharge |
| 4 | VT-17 — repeated keys are legal, stated where a reader meets `Tags` | a clause discharge |
| 5 | VT-3 / ES-17 — `into_parts` is not a clone-avoidance route for adapters | **one row, two ids** |
| 6 | ES-40 — a condition is a claim about one store's log | a clause discharge |
| 7 | — a code comment reading "ES-6 is deferred" (`memory.rs`) | **a comment repair, not a clause discharge** |
| 8 | — a comment resting on a premise phase 4 falsified (`ingest.rs`) | **a comment repair, not a clause discharge** |

and then adds, in prose: *"ES-19's correction is the ninth and is recorded by the pass rather
than re-verified here."*

**The arithmetic closes like this, and it is not what the sentence suggests.** Nine = eight
rows + ES-19. But two of the eight rows are comment repairs rather than clause discharges, and
one row carries two clause ids. So the count of *clause ids* the pass touched is eight — ES-23,
ES-24, VT-15, VT-17, VT-3, ES-17, ES-40, ES-19 — and the count of *documentation MUSTs
discharged* is six rows plus ES-19, i.e. seven. "Nine" is a count of rows-plus-one, not a count
of clauses, and nothing in either document says which it is. That ambiguity is precisely the
defect BR-10 exists to prevent, one level up: a number in prose that a reader cannot check
against an enumeration, because there is no enumeration.

## The derived candidate scan

The derived half is a scan of `spec/SPECIFICATION.md` for six phrasings — `MUST document`,
`MUST state`, `MUST say`, `documentation MUST`, `MUST be documented`, and
`belongs in the port's documentation` — over **clause bodies whose whitespace has been
collapsed first**, attributed to the clause whose declaration line precedes them.

Two mechanical findings worth recording, because each one would have silently shrunk the set:

* **A line-keyed scan misses `PS-34`.** Its obligation reads `the port MUST` / `document that
  an implementer has to spell the parameter …` across a soft wrap
  (`spec/SPECIFICATION.md:5546-5547`). Collapsing whitespace before matching is what finds it.
* **The declaration heuristic agrees with the parser exactly.** Over the real document the scan
  attributes candidates to 200 declared clauses, and `crate::spec_trace::clause_ids` resolves
  the same 200 — measured, `declared=200 ids=200 missing=[] extra=[]` — so no candidate is
  attributed to a heading the parser does not consider a declaration, and none is dropped. The
  measurement is re-derived on every test run by
  `the_declaration_scan_attributes_every_clause_the_parser_declares`, so the agreement is an
  assertion rather than a reading taken once: an attribution divergence that leaves the candidate
  set unchanged is invisible to `the_whole_pin_holds_against_the_real_tree` and is exactly what
  that test catches.

The scan finds **21** candidates:

`CF-35`, `CF-39`, `CF-40`, `ES-19`, `ES-23`, `ES-24`, `ES-26`, `ES-35`, `ES-40`, `PS-31`,
`PS-34`, `PS-36`, `SY-32`, `VT-13`, `VT-15`, `VT-17`, `VT-21`, `VT-22`, `VT-24`, `VT-32`,
`VT-33`.

## The disposition, candidate by candidate

### Pinned — `[FROZEN]`, on the contract's own documentation, discharged today

| Clause | Discharge site | Anchor (verbatim) |
| ------ | -------------- | ----------------- |
| VT-13 | `crates/happenstance-core/src/event.rs:261-263` | ``resumes with `ReadOptions::from(checkpoint.next()?)`, and that is sound on a store with gaps`` |
| VT-15 | `crates/happenstance-core/src/tag.rs:29` | `# Equality is byte equality, and nothing is normalised` |
| VT-17 | `crates/happenstance-core/src/tag.rs:255-256` | `a key may legally appear more than once` |
| VT-32 | `crates/happenstance-core/src/event.rs:88-90` | ``an invalid one that nothing reads survives `check`, `clippy`, `build` and `test``` |
| VT-33 | `crates/happenstance-core/src/tag.rs:467` | ``Not the amortised O(1) `Extend` usually implies.`` |
| ES-19 | `crates/happenstance-core/src/store.rs:131` | ``It is not a sound `after` for a follow-up condition`` |
| ES-23 | `crates/happenstance-core/src/store.rs:146` | `# Cancellation` |
| ES-24 | `crates/happenstance-core/src/store.rs:169` | `at-most-once under verbatim reissue` |

Every anchor is a **phrase**, never the clause id, and that is forced rather than preferred:
the shipping half of each of those three files — `#[cfg(test)]` excluded — names no clause id
at all, so an id-as-anchor pin would fail on every entry the day it landed. `guard_pin` rejects
one, and `an_anchor_equal_to_its_clause_id_is_a_hard_error` also re-checks the premise against
the real files, so the rule fails loudly if that ever stops being true.

### Excluded — with the reason on the entry

| Clause | Why the rule does not reach it |
| ------ | ----------------------------- |
| VT-21, VT-22, VT-24 | (b). The obligation is on a **store** — "a store MAY accept more, MUST document its actual limit" (`:1487`, `:1517`, `:1560`) — not on the contract's documentation. |
| ES-26 | (a) and (b) are both satisfied and the obligation is **not discharged**. "The asymmetry is deliberate and MUST be documented as such" (`:3757`): `store.rs:118` says `from` is inclusive and `append.rs:135` says `after` is exclusive, and nothing anywhere says the asymmetry is deliberate. Anchoring it needs a doc-comment edit — HS-P0023's. **A finding, not a formality.** |
| ES-35 | `[PROVISIONAL]`, and the obligation is on an adapter that declines durability. |
| ES-40 | `[PROVISIONAL]`. Its discharge at `append.rs:29` is present today and would anchor cleanly on `# A claim about one store's log, not about the world`; this becomes a pinned entry the day the clause freezes. |
| PS-31 | `[FROZEN]` and on the port's own documentation, and **not discharged**: `projection.rs` does not say that an event-emitting projection is out of scope at 0.1. HS-P0023's edit. |
| PS-34 | `[PROVISIONAL — contingent on PS-5]`, and dead the moment `type Batch;` lands. |
| PS-36 | `[FROZEN]` and on the port's own documentation, and **not discharged**: `projection.rs` does not document that the `Send` flavour transitively requires `Batch: Send`. HS-P0023's edit. |
| SY-32 | `[DEFERRED]`, and `SyncPeer` is `happenstance-sync`'s port rather than the contract's. |
| CF-35 | (b). The obligation is on this specification's own clauses and is discharged by `cargo xtask spec-trace` (CF-38), not by a doc comment. |
| CF-39, CF-40 | (b). The obligation is on a **fixture**. |

### Not candidates at all, and deliberately absent from the enumeration

**VT-3** and **ES-17** are the evidence table's row 5, and neither words a documentation
obligation in any of the six phrasings, so neither is a candidate. Putting either into the
enumeration would fire assertion 3's *the enumeration is stale* arm on a green tree.

* **VT-3** states its correction under a heading that says what it is: *"**Prose, not a
  clause:** `Event::into_parts` … used to document itself as 'avoiding a clone in adapter write
  paths'. That was false as written … ES-17 owns the correction"* (`:681-688`). It is a repair
  recorded as prose, not a MUST.
* **ES-17** is `[PROVISIONAL]` (`:3290-3295`), so it fails (a) regardless. Its discharge at
  `event.rs:404-413` — *"Not a clone-avoidance route for adapters"* — is present and would
  anchor cleanly; it becomes a candidate only if the clause is ever reworded into a MUST, and a
  pinned entry only if it also freezes.

## The answer

**The set was recorded as nine, the evidence supports at most eight clause ids, and the
re-derivation puts the pinned count at eight — a *different* eight.**

* Kept from the evidence table: ES-19, ES-23, ES-24, VT-15, VT-17 — five.
* Dropped from it: VT-3 (not a MUST), ES-17 (`[PROVISIONAL]`), ES-40 (`[PROVISIONAL]`) — three.
* Found by the scan and absent from it: VT-13, VT-32, VT-33 — three, each `[FROZEN]`, each on
  the contract's own documentation, each discharged today and none of them previously written
  down anywhere.

Three `[FROZEN]` documentation MUSTs on the contract's own documentation are **not discharged
at all** — ES-26, PS-31, PS-36 — and this record is the first place that has been said. They
are excluded with that reason rather than pinned, because pinning them would land the gate red
and the only repair is a `happenstance-core` doc-comment edit, which belongs to HS-P0023 under
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`. They are the concrete follow-up
this story hands forward.

The count is not written into the `const`'s comment, and it is not written into the module.
`UNCLAIMED_PENDING_ADR` already refused that trade — its count *"is computed and printed rather
than written here, so this comment cannot come to disagree with the array beneath it"*
(`xtask/src/spec_trace.rs:1975-1978`) — and `the_pin_holds_no_written_count` holds this pin to
the same bar.
