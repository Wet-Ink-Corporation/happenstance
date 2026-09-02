---
item: HS-S0052
stage: discover
created: 2026-08-12T13:02:14.310Z
updated: 2026-08-12T13:02:14.310Z
template_sig: 86ce4036
rendered_sig: 24e04f21
---

# Discover — The ES-6 artefact: constraint violation versus transport fault, recovered by a caller

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: commit the ES-6 artefact — a test that reconstructs, from a *caller-visible* `CloudflareEventStoreError` carrying a real `worker::Error`, the one fact a caller branches on, and fails if the error type stops carrying it | `_storymap.md`, **Slices**, `caller-visible-error-verdict` row | The deliverable is one test, and its entire value is the word *caller-visible* |
| AC-005, and DoD 4 restated at the boundary: a test in the adapter's own tree recovers the constraint-violation-versus-transport distinction from a caller-visible error, and fails if the error type stops carrying it | `project.md`, **Acceptance criteria** AC-005; **Definition of done** item 4 | A boundary-observable artefact, checkable by someone who did not do the work |
| `dependsOn: durable-object-write-path` — it supplies the classification site, the point where a thrown `worker::Error` becomes either `AppendError::ConditionViolated` or something else | `_storymap.md`, **Slices** | The classifier is the write path's; the *reconstruction* is this story's |
| DR-4: a green suite decides nothing here — every conformance rule asserts on the success path or on a store-produced `AppendError`, and none reads an adapter error's contents | `project.md`, **Derived requirements**, DR-4 | The reason this needs its own story rather than falling out of the conformance run |
| The runbook is explicit that the proof artefact has two halves precisely because the first does not decide ES-6, and that without the second half "ES-6 is decided" is discharged by assertion | `RUNBOOK.md:4276-4292` | The previous plan offered only the green suite; that is the defect this story closes |
| ES-6 is `[FROZEN]` and ADR-0009 settled it: `Error` keeps `core::error::Error + 'static` on both ports and both flavours, and the stronger property becomes a downstream blanket-implemented marker | `spec/SPECIFICATION.md:2629-2686`; `.kb/decisions/0009-error-send-sync.md` | The bound is not reopened. AC-005's "bound added, or the deferral confirmed" wording predates the atom's acceptance |
| The stale-wording flag is already recorded, twice, rather than discovered here | `_grounding.md` §1; `_decomposition.md`, Architecture brief Notes §2, "Tension (flagged)" | Reading AC-005 literally would contradict an accepted atom. The real work is (a) staying `!Send` and (b) this test |
| Finding 2: a Durable Object surfaces SQLite's own text — `UNIQUE constraint failed: event.position` — through the thrown `Error`'s `message`, and exposes no numeric code either way | `crates/happenstance-cloudflare/src/lib.rs:63-73` | Classification is text matching. What a caller can recover depends entirely on what survives into the error they hold |
| Finding 3: the DCB conflict signal never travels in `Self::Error` on *any* adapter, which is why `CloudflareEventStoreError` deliberately has no `ConditionViolated` variant | `crates/happenstance-cloudflare/src/lib.rs:75-85`; `crates/happenstance-cloudflare/src/event_store.rs:100-104` | So the fact under test is not "is it a conflict" alone — it is whether a *transport* fault is distinguishable from one |
| `ViolationAsStoreErrorStore` is registered against the portable half: a store that reports the violation through the wrong `AppendError` arm | `crates/happenstance-testkit/tests/mutation_coverage.rs:962-964` | The suite already owns the channel question. What it cannot own is the contents of one adapter's error type |
| The mock is a constructed thrown value carrying the exact message text, not a live Durable Object round trip | `_decomposition.md`, Testing brief Notes §3 | Keeps this artefact cheap and independent of whether the `workerd` runner exists yet |
| `CloudflareEventStoreError` is the only error type in the workspace that can fail a `Send + Sync` bound — the other two are free by construction | `crates/happenstance-cloudflare/src/lib.rs:12-21`, `:236-243`; `project.md`, **How this advances the initiative** | This crate is the sole instrument for ES-6, and this story is the half of it that is about information rather than about auto traits |

## Questions

**Is ES-6 reopened here?** Answered: **no.** ADR-0009 is accepted and immutable,
and `redkiln validate --kb` checks accepted atoms against `HEAD`. This story
produces the artefact ADR-0009's prediction is judged against; confirming or
refuting it is a *new* record, authored through the runbook's ADR queue, never an
edit to `.kb/decisions/0009-error-send-sync.md` (`project.md`, DR-7;
`CLAUDE.md`, **Where the work lives**).

**Does the test need a `workerd` runner?** Answered: **no**, and deliberately. It
constructs the thrown value directly with the message text a Durable Object's
SQLite produces (`_decomposition.md`, Testing brief Notes §3), so DoD 4 becomes
observable independently of — and earlier than — the harness question that
`every-rule-under-workerd` is still answering. That independence is the point: the
one exit criterion the runbook says has no artefact behind it should not be the one
waiting on the riskiest piece of infrastructure in the project.

**Host or `wasm32` target?** Inherited from `worker-binding-layer`'s open question
(`_decomposition.md`, Testing brief Notes §2) and **deferred to spec** with it, on
the same terms: wherever it lands, an ordinary `cargo test` must reach it.

**Where does the classifier live — here or in the write path?** Answered: the
classifier is `durable-object-write-path`'s, because it must run *before*
`Self::Error` is constructed. This story owns the reconstruction, and it must read
the error through the public surface a caller actually has rather than through a
private field, or it will keep passing after the information stops being
recoverable.

**Does anything here change the wire between adapter and caller?** Answered: no
new `AppendError` arm and no new adapter error variant for conflicts —
`event_store.rs:100-104`'s absence is a finding made structural and adding to it
would look like a fix and be the defect.

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.** Not
this story's; they belong to `measured-store-limits` and
`adr-0023-and-atom-resolutions` (coordinated with HS-P0012), and to
`every-rule-under-workerd` and ADR-0023.

## Decision

ES-6's real question was never "does the bound compile" — ADR-0009 answered that
by compiling four things against this crate — it is whether an adapter error a
caller actually receives still carries the one fact a caller must branch on:
was this a constraint violation, or was it a transport fault? A green conformance
suite is silent on that by construction, because every rule asserts on the success
path or on a store-produced `AppendError` and none of them reads an adapter error's
contents; the runbook says so and calls the missing artefact out as the reason
"ES-6 is decided" would otherwise be discharged by assertion. This slice commits
that artefact: a test that builds a real `worker::Error` carrying the message text
a Durable Object's SQLite produces, drives it through the same path `append` uses,
and reconstructs the distinction from what a caller can see. The spec will cover:
the exact thrown value constructed and why that text; the public surface the test
reads through and why not a private field; the assertion that fails if the error
type stops carrying the fact; where the test lives after `worker-binding-layer`
settles the host-versus-`wasm32` question; and the sentence that goes into ADR-0023
recording whether ADR-0009's asymmetry prediction bit. Nothing `[FROZEN]` is
amended — ES-6 is confirmed against a real error type, which is what the clause
asked for.

## The wrong implementation

**The error that is legible only to its author.** Two shapes, and the second is
the likely one. The blunt version: `CloudflareEventStoreError::Sql` carrying a
`StringifiedThrow` whose `Display` renders `"a SQL error occurred"`. The subtle
version, which a careful implementer arrives at honestly: a classifier that
correctly distinguishes constraint violation from transport fault *inside*
`append`, uses it to pick between `AppendError::ConditionViolated` and
`AppendError::Store(..)`, and then discards the evidence — so the `Store` arm a
caller receives on the failure path cannot be told apart from a network fault, a
storage cap, or a binding that was never wired up. Every existing check passes,
and there is a structural reason none of them can fail it, stated in this project's
own derived requirements: every conformance rule asserts on the success path or on
a store-produced `AppendError`, and none reads an adapter error's contents
(`project.md`, DR-4). The suite is green whether the error carries the fact or not,
which is exactly why `RUNBOOK.md:4276-4292` refuses the green suite as the artefact
and demands this second half.

**Where the detector must live.** In this crate's own tree, beside
`not_send_probe`: a test that constructs a thrown value carrying the Durable
Object's real text (`UNIQUE constraint failed: event.position`, named at
`crates/happenstance-cloudflare/src/lib.rs:63-73`), drives it through the same
classification path `append` uses, and asserts on what a **caller** can observe —
through the public error surface, not a private field, or the test survives the
regression it exists to catch. It does **not** belong in the testkit's registry:
`ViolationAsStoreErrorStore` (`crates/happenstance-testkit/tests/mutation_coverage.rs:962-964`)
already covers the portable half — a violation reported through the wrong
`AppendError` arm — and what remains is a property of *this* error type, which no
portable rule can state without asserting on an adapter's private contents. That
asymmetry is itself worth writing down rather than closing: it is the same
asymmetry ES-6's clause records, and it is why the workspace's sole `!Send` adapter
is the sole instrument for the clause.

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

**Box 6.** No conformance rule is added here. The one test this story commits
asserts on an error's caller-visible classification and never on a position, so the
specification's permission for gaps is not engaged at all.

**Box 7.** ES-6 is `[FROZEN]` and this story touches it. It is **not changed**:
ADR-0009 settled the clause and this story supplies the evidence the clause's own
`Rule:` line was waiting for. If the artefact refuted ADR-0009 — if a real
`worker::Error` turned out to carry nothing a caller can act on — the outcome is a
new decision atom naming the alternatives that lost, written first and authored
through the ingest path by `adr-0023-and-atom-resolutions`, never a line edit to
an accepted atom or to a frozen clause.

**Box 8.** No conformance rule here seems wrong. The one artefact that *did* seem
wrong — AC-005's "bound added, or ADR-0009's deferral confirmed", which reads ES-6
as still open — is corrected in this story's Questions rather than obeyed, with the
reason (the atom's acceptance post-dates the wording) and the two places it was
already flagged both cited.
