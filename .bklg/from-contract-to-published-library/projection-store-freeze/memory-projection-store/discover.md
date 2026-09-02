---
item: HS-S0006
stage: discover
created: 2026-08-12T13:01:13.870Z
updated: 2026-08-12T13:01:13.870Z
template_sig: 86ce4036
rendered_sig: 342a10de
---

# Discover — MemoryProjectionStore as oracle, doctest target and cold-start fix

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: ship `MemoryProjectionStore` behind the `memory` feature as the oracle, the doctest target and the cold-start fix, implementing `ProjectionProbe` under `conformance`, with the `error[E0195]` spelling trap documented where an implementer meets it and the module named defensively so a `memory`-gated intra-doc link cannot break `cargo doc` | `../_storymap.md:58` | Four jobs in one item: reference implementation, doctest host, cold-start fix, and rustdoc hazard containment |
| **AC-012** — `MemoryProjectionStore` ships behind the `memory` feature, is the doctest target for the port, and the `E0195` spelling trap is documented where an implementer meets it (PS-34, PS-36) | `../project.md:218-220` | Sole owner of AC-012 (`../_storymap.md:90`). Also the first shape for AC-004, whose second shape lands in `buffering-conformant-variant` (`../_storymap.md:82`) |
| `dependsOn: owned-batch-port-shape, projection-probe-conformance-feature` — supplies the trait it implements and the probe it must also implement | `../_storymap.md:56-58,108` | It cannot be the oracle for a suite whose write seam does not exist yet |
| The cold-start problem this fixes, stated by the RUNBOOK: implementing the port today fails with `error[E0195]` unless the implementer spells the parameter `Self::Batch<'_>` exactly — "nothing says so, and there is nothing to copy" | `../_grounding.md:202-207`; `RUNBOOK.md:3898-3903` | The store is ~50 lines and exists as much to be **copied** as to be run |
| The trap is independently confirmed from the implementer's side: `error[E0195]: lifetime parameters or bounds on method 'commit' do not match the trait declaration`, met while writing an impl | `references/adapter-shapes.md:186-194`; `crates/happenstance-ladybug/src/live_handle.rs:68-83` | Two independent transcripts, so the documentation quotes evidence rather than describing a hazard |
| UX brief **AC-U06** — the trap's documentation lands on the port's own rustdoc *and* `MemoryProjectionStore`'s doctest, "the two pages an implementer already has open". A trap documented only in ADR-0017 is documented where the person who needs it is not | `../_decomposition.md:139-147` | Placement is an acceptance criterion, not a preference. The doctest must compile under `cargo test --doc` |
| The reference-fixture pattern to mirror already exists: `MemoryFixture`/`MemoryHandle` hand back an **owned** handle holding a refcount, which is the fix for the borrowing-GAT ICE | `crates/happenstance-testkit/src/fixtures.rs:243-292`; `crates/happenstance-testkit/src/contract.rs:97-111`; `../_decomposition.md:535-540` | "The fix is the one `MemoryFixture` already demonstrates" — copy, do not invent |
| A `memory`-gated module is a rustdoc hazard this workspace has already paid for once: an intra-doc link into a module absent on some documented configuration is a hard rustdoc error | `crates/happenstance-testkit/src/lib.rs:112-132`; `../project.md:317-321`; `../_decomposition.md:694-699` | The gate runs a `--no-default-features` doc build of `happenstance-core`, plus a nightly `--cfg docsrs` build where the toolchain is present |
| Mount points: `MemoryProjectionStore` re-exported beside `MemoryEventStore` in the contract crate's export block, gated exactly like the existing `memory` module | `../_decomposition.md:348,351`; `crates/happenstance-core/src/lib.rs` export block | "Mounted" is the export block plus the feature table; one without the other is unreachable (`../_storymap.md:46-49`) |
| Testing brief AC-012: **Unit** (the port's rustdoc example compiles and runs against `MemoryProjectionStore` under `cargo test --doc`) plus **Static** (the nightly `--cfg docsrs` build renders the trap's documentation without the intra-doc-link hazard) | `../_decomposition.md:782` | Both halves are already-existing gate steps. This story costs no new gate step |
| `CLAUDE.md`'s design-stage instruction — "a doctest in place of a mock" — is what the doctest is *for*, not decoration | `CLAUDE.md`, *Where the work lives*; `../project.md:252-254` | The doctest is DoD 7's instrument as well as AC-012's |
| PS-34 and PS-36 are the clauses AC-012 names; PS-36's content is that the `Send` flavour transitively requires `Batch: Send` and `type Batch: Send;` cannot be written because `trait_variant` copies bounds verbatim into the `!Send` flavour | `spec/SPECIFICATION.md:5601-5627`; `../_decomposition.md:374-381` | The reference store must be implementable on the bare flavour and documented so an adapter author learns the constraint rather than meeting it as a type error |

## Questions

Open questions to resolve before specifying.

1. **Which flavour does `MemoryProjectionStore` implement?** Deferred to `spec`,
   with the constraint recorded: the reference store must be usable from suite
   code that binds the weaker requirement, and the existing `memory` module's
   `MemoryEventStore` is the precedent to follow rather than re-argue.
2. **Does it implement `ProjectionProbe` unconditionally or under
   `conformance`?** Answered by the storymap's own row: under `conformance`
   (`../_storymap.md:58`). Which means `memory` + `conformance` is a combination
   the powerset must cover, alongside each without the other.
3. **What exactly does the doctest demonstrate?** Deferred to `spec`. It must at
   minimum show the `Self::Batch` spelling that avoids `E0195` — that is the
   cold-start fix — and the `commit` call that carries both writes, since that
   is the invariant the module exists to hold.
4. **Is `MemoryProjectionStore` also the CF-5 conformant variant?** Answered: no.
   It is the *first* of AC-004's two shapes (apply-on-write); the second is the
   buffering replay-at-commit variant in the testkit's own `tests/`, and it is a
   separate story (`../_storymap.md:65,82`; Architecture brief AC-A03).
5. **None beyond these.** `ProjectionId::new` is not hardened here
   (`../_decomposition.md:331-335`).

## Decision

An adapter author's first act is to write `impl ProjectionStore for MyStore`, and
today that act fails with a compiler error whose fix is an exact spelling nothing
in the repository states and nothing demonstrates. This slice ships the ~50-line
reference implementation that fixes the cold start: `MemoryProjectionStore` behind
the `memory` feature, implementing `ProjectionProbe` under `conformance`,
re-exported beside `MemoryEventStore`, carrying the doctest that stands in for a
mock and the `E0195` trap's documentation on the two pages an implementer already
has open. It is also the oracle every rule in the next three slices is
differential against, and the first of AC-004's two batch shapes. The spec will
fix: the store's internal shape (owned handle holding a refcount, mirroring
`MemoryFixture`, and no borrowing GAT anywhere); the feature gating and both
mount points; the doctest's content, including the `Self::Batch` spelling and a
`commit` that carries the read-model write and the checkpoint together; where the
`E0195` documentation lands — port rustdoc *and* store doctest, per AC-U06 — and
the defensive module spelling that keeps `--no-default-features` and `--cfg
docsrs` doc builds green. It touches no `[FROZEN]` clause; PS-34 and PS-36 are
documentation obligations this story discharges rather than clause text it edits.

## The wrong implementation

**A doctest fenced ```` ```ignore ```` or ```` ```no_run ````.** It renders, it
reads correctly, `cargo test --doc` reports it as passing or skipping, and the one
thing it was written for — proving that the exact `Self::Batch` spelling compiles
— is never checked. An implementer copies text nobody compiled. This is the
cheapest possible way to satisfy AC-012's words while discarding its content, and
the spec must require the doctest be a compiled, run example. The sibling trap is
already documented in this repository for a related case:
`compile_fail,E0080` is silently ignored by rustdoc on 1.97.1 when it cannot match
the code, so "the stricter-looking spelling is the weaker check"
(`crates/happenstance-testkit/src/contract.rs:400-403`; UX brief AC-U12). Any
`compile_fail` demonstration here is spelled **bare**.

**A `MemoryProjectionStore` whose `commit` writes the read model and the
checkpoint under two separate `RefCell` borrows.** It passes every rule that
exists at the end of this slice — there are none yet — it passes
`commit_advances_the_checkpoint` when that lands, and on a single-threaded store
with no failure injection it will pass `commit_is_atomic_with_the_read_model` too,
because nothing ever interleaves. The oracle then encodes the *shape* of the bug
`CheckpointOnlyStore` exists to catch, and every later rule is calibrated against
it. This is the specific danger of a reference implementation written to make
rules pass rather than to model a real store, and the guard is that the oracle's
own structure — one unit of work, one borrow — is a stated spec requirement, not
an emergent property of a small file.

**A doc comment in ADR-0017 that explains the `E0195` trap beautifully.** The ADR
is accepted, `redkiln validate --kb` is green, `cargo xtask spec-trace` is green,
and the implementer who hits the error is looking at `projection.rs` and at
`MemoryProjectionStore`, neither of which mentions it. AC-U06 names this outcome:
"a trap documented only in ADR-0017 is documented where the person who needs it is
not" (`../_decomposition.md:146-147`). Nothing executable catches it; the spec's
placement requirement is the guard.

**And the module-naming one:** a `memory`-gated `MemoryProjectionStore` linked to
from the port's rustdoc with an intra-doc link. Green on default features, green
on `--all-features`, and a **hard rustdoc error** on the gate's
`--no-default-features` doc build of `happenstance-core` — the identical hazard
the testkit already paid for and now spells two module names plainly to avoid
(`crates/happenstance-testkit/src/lib.rs:112-132`).

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

The two judgement boxes. **Literal positions**: this story adds a store and a
doctest, not a conformance rule. The doctest does commit at a position, and the
spec should have it use a `SequencePosition` obtained rather than a literal
constant where that reads naturally — but the box itself is vacuously true here,
since no rule is added. **`[FROZEN]` clauses**: none is touched. PS-34 and PS-36
are the clauses AC-012 serves, and this story discharges the documentation
obligation they state rather than editing them.
