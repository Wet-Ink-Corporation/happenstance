---
item: HS-S0056
stage: discover
created: 2026-08-12T13:02:18.275Z
updated: 2026-08-12T13:02:18.275Z
template_sig: 86ce4036
rendered_sig: a5c8f301
---

# Discover — WF-11's falsifier tested where the memory ceiling is real

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: forward a payload sized against this runtime's *measured* memory ceiling through the adapter's actual non-streaming encode path under the `workerd` run, and record whether it bites — gathering evidence only, changing no wire format | `_storymap.md`, **Slices**, `wf-11-memory-ceiling-falsifier` row | Two constraints in one sentence: measured, and under the run. Either dropped makes the result worthless |
| AC-011: whether this runtime's memory ceiling forces a peer to forward a payload it cannot buffer through a human-readable encoder is answered **with evidence**, and the open-question atom reflects it. No wire format change is made in this project | `project.md`, **Acceptance criteria**, AC-011 | "With evidence" is the operative phrase — and the atom's update is a separate story's write |
| `dependsOn: every-rule-under-workerd` — the only place on this runtime where a payload can meet a real memory ceiling inside a process the gate started | `_storymap.md`, **Slices** | The dependency exists because the measurement is only meaningful on the target |
| WF-11 is `[PROVISIONAL]`, checked by `wire::payload_is_base64_in_json` and `wire::payload_is_raw_in_postcard`, and its falsifier is explicitly owned by phase 9 — this adapter | `spec/SPECIFICATION.md:2279`, ledger row `:8576`; `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`, *What forces it* | The atom names this project by phase, and says why a marker must be owned by a phase that can observe its falsifier |
| The finding is about serde's data model rather than about base64: `serialize_str` takes a `&str` and `collect_str` takes a `Display`, and both write the entire rendering before returning — so **any** human-readable payload encoding must materialise the whole payload, and hex has the same property | `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`, *What is true today* | It falsifies human-readable payload encoding *as a category*. A fix is not a better alphabet |
| The `no_std` half of the falsifier is already **closed by measurement** and left the falsifier rather than staying in it with a note attached; only the buffering half is live | same atom, *What is true today* | Precedent for what "answered" looks like here: a measurement that removes the question, not a paragraph that describes it |
| The measured wire costs against a 340 KiB payload: JSON integer array 1,243,464 bytes (3.5715x), base64-in-JSON 464,218 bytes (1.3333x), postcard 348,163 bytes (1.0000x) | same atom, *What is true today*; `references/` wire-format measurements cited there | A ready-made negative control: a run that shows no difference between the binary and human-readable paths did not exercise the encoder |
| The suite's guaranteed minimum payload is 65,536 bytes | `crates/happenstance-testkit/src/lib.rs:145` (Value edges row) | The atom's own first sub-question asks whether a payload at or above that must go through JSON here |
| The encoder lives in the sync crate, and this project ships no sync port — `happenstance-cloudflare` is an event store only | `crates/happenstance-sync/src/wire.rs`; `CLAUDE.md`, repository map; `project.md`, **Out of scope** | The falsifier's subject is a *peer*, and this crate is not one. That gap is the first thing the spec must confront |
| Evidence is gathered here; the decision is `replication-identity-and-ingest`'s (HS-P0017), and no wire-format change is made in this project | `project.md`, **Out of scope** | The output is a measurement and a statement, not a patch to `wire.rs` |
| The atom is **resolved rather than deleted**, and that write belongs to `adr-0023-and-atom-resolutions` | `project.md`, AC-008 and DR-7; `_storymap.md`, **Why these milestones and not others**, last bullet | This story produces the finding; it does not touch `.kb/` |
| ADR-0003 keeps payloads opaque `Bytes` on the way through, and the atom's own third sub-question asks whether any fix would sit adjacent to that boundary | `.kb/decisions/0003-opaque-payloads.md`; the atom, *Ordered sub-questions* 3 | The adapter never inspects a payload, which is what makes "forward" the right verb |

## Questions

**Does this adapter forward anything through a human-readable encoder at all?**
This is the first thing the spec must establish, and it is answered here as a
framing rather than as a result. `happenstance-cloudflare` is an event store, not
a replication peer; it ships no sync port and this project adds none. So the
falsifier's *condition* — a peer that must forward a payload larger than it can
buffer — may not be constructible in this crate without standing up something that
does not exist. Two outcomes are acceptable: measure the isolate's real ceiling and
drive `crates/happenstance-sync/src/wire.rs`'s own encoder against it *inside* the
wasm32 run, which needs no sync port in this adapter and is the cheap path; or
record, with evidence, that the condition cannot be met on this runtime and state
exactly what would construct it. One outcome is not acceptable: "we did not see it
fire" as a resolution.

**What is "this runtime's memory ceiling"?** Deferred to **spec**, and measured
rather than read off a documentation page — the same discipline
`measured-store-limits` applies to the fixture's three numbers, and for the same
reason: the number that matters is the one the process actually hits.

**What payload size is the right probe?** Deferred to **spec**, anchored on two
figures already in the tree: the suite's guaranteed minimum of 65,536 bytes, which
is the smallest payload a conformant store must accept, and the atom's own 340 KiB
measurement subject, which is what the wire-format numbers were taken against.

**Does anything here move WF-11's marker or re-scope the clause?** Answered:
**no.** Marker changes and re-scoping "to formats rather than peers" belong to the
wire format, and the wire format is out of scope for this project. This story
produces a measurement and a statement; `adr-0023-and-atom-resolutions` moves the
atom to *resolved*, and HS-P0017 decides what, if anything, changes.

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.** Not
this story's, though it consumes the second: the measurement runs inside whatever
runner `every-rule-under-workerd` settles on.

## Decision

WF-11's provisional marker rests on a falsifier nobody has been able to fire,
because it needs two things in the same place — a memory ceiling that is real and a
payload big enough to hit it — and until this adapter runs, this workspace has had
neither. The falsifier is broader than it looks: serde offers no streaming entry
point for a human-readable string, so the exposure is not base64's and not hex's
but the whole *category* of human-readable payload encoding, and if it fires the
clause has to be re-scoped from peers to formats. This slice goes and looks, on the
one runtime where looking means anything. It measures the isolate's actual ceiling,
drives a payload sized against it through the real non-streaming encode path, and
records what happened — the ceiling, the size, the path, and the outcome. The spec
will cover: how the ceiling is measured; which payload sizes are probed and why
those; how the encoder is reached from inside the wasm32 conformance run without
this crate taking a sync dependency; the negative control that proves the encoder
was actually exercised; and the exact form of the finding handed to
`adr-0023-and-atom-resolutions`, including the honest answer if the condition turns
out not to be constructible here. No wire format changes, no clause marker moves,
and nothing is written to `.kb/` by this story.

## The wrong implementation

**The falsifier tested where the ceiling is not.** Write the test, run the encoder
on the host — where materialising 340 KiB is free — observe that nothing fails, and
record "WF-11's falsifier does not bite on this runtime." Every check in the tree
passes and would have passed anyway: the test is green, `cargo xtask spec-trace`
still resolves WF-11's citations and finds its marker un-stale,
`wire::payload_is_base64_in_json` and `wire::payload_is_raw_in_postcard` were
already green and stay green, and a reviewer reading the diff sees a measurement
with a number in it. The atom then moves to *resolved* on the strength of a
measurement taken on the one machine where the property under test does not exist.
This is the project's entire thesis — a claim about a runtime asserted rather than
run — reappearing inside the one story whose whole content is supposed to be a
measurement, and it is the easiest place in the project to commit it, because a
host test needs no runner, no host object and no fixture, and finishes in a second.

**A second, quieter version of the same defect:** run it on the target, but against
a payload sized by guess rather than by measurement — small enough that nothing
materialises anywhere near the ceiling. The result reads identically and is equally
void.

**Where the detector lives, and why it is not a rule.** Not in
`crates/happenstance-testkit/tests/mutation_coverage.rs`: no store fails a rule
here, and no conformance rule states this property — WF-11 is a *wire format*
clause, checked by `crates/happenstance-sync`'s own `wire` proof artefact
(`xtask/src/proof.rs:133-148`), not by the event-store suite. The detector is the
shape of the artefact itself, and it has two halves. First, the measurement must
execute inside the wasm32 conformance run that `every-rule-under-workerd` builds,
under the gate's own step, so "on the target" is a property of the gate rather than
of a claim in a commit message. Second, the recorded finding must carry a negative
control drawn from the atom's own measurement table: postcard encodes the same
payload at 1.0000x while base64-in-JSON is 1.3333x and the naive integer array
3.5715x, so a run that reports no difference between the binary and human-readable
paths did not exercise the encoder and its result is void. Without that control the
evidence is indistinguishable from a test that measured nothing — which is exactly
the vacuity `the_probe_is_not_vacuous` exists to prevent one crate over.

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

**Box 6.** No conformance rule is added here. The artefact is a measurement over
payload bytes and process memory; it asserts on sizes and on whether an encode
completed, never on a position, so the specification's permission for gaps is not
engaged.

**Box 7.** Nothing `[FROZEN]` is touched. WF-11 is `[PROVISIONAL]` and this story
neither moves its marker nor re-scopes it — re-scoping "to formats rather than
peers" is the wire format's decision and belongs to `replication-identity-and-ingest`
(HS-P0017), which this project explicitly excludes. If the falsifier fires hard
enough to require a wire-format change, that is a new decision atom and a re-plan in
another project, written first.

**Box 8.** No conformance rule here seems wrong. The clause's own rules —
`wire::payload_is_base64_in_json` and `wire::payload_is_raw_in_postcard` — are
correct about *what* is encoded and are silent about *how much memory it takes*,
which is a real gap and is the reason WF-11 carries a falsifier instead of a rule;
this story fills the gap with a measurement rather than by asking a wire rule to
assert something it cannot see.
