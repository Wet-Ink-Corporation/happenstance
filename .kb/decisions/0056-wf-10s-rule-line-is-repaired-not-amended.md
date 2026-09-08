---
id: kb-decision-0056
title: WF-10's rule line gains the two value types it always covered
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0056
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  WF-10 MUSTs that every value type's Deserialize re-runs its constructor's invariants, and its Rule line named four tests covering only Tags, QueryItem and Query — EventType and Tag, the two types VT-14 gives invariants to, had none. Measured: replacing EventType::deserialize's checked construction with an unchecked one leaves the whole workspace green, because the proptest generators only ever feed already-validated values. Two decode_rejects tests are added and cited; Event's Serialize destructures exhaustively so E0027 fires symmetrically with the E0063 its Deserialize already produces. Not yet written.
depends_on:
  - kb-decision-0015
related:
  - kb-decision-0016
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/wf-10-instruments-narrower-than-the-clause.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# WF-10's rule line gains the two value types it always covered

## Decision

WF-10 `[FROZEN]` reads, unqualified: "The `Deserialize` implementation of **every** value
type MUST re-run its constructor's validity invariants, so that no wire value can become
an in-memory value the constructor would have rejected." Its `Rule:` line names four tests
— `decode_rejects_a_non_canonical_tag_set`, `decode_rejects_an_unconstrained_query_item`,
`decode_rejects_a_zero_item_query`, `decode_accepts_an_over_capacity_value` — and
`grep -n "fn decode_" crates/happenstance-core/tests/wire.rs` returns exactly those four.
They exercise `Tags`, `QueryItem` and `Query`. `EventType` and `Tag` — the two value types
VT-14 `[FROZEN]` gives constructor invariants to in the first place — have none.

**Measured, not asserted.** In a scratch worktree, `EventType::deserialize`'s checked
construction (`Self::new(raw).map_err(...)`) was replaced with unchecked construction
(`Ok(EventType(Cow::Owned(raw)))`), and `cargo test --workspace --all-features` reported
`EXIT=0` throughout. Nothing observes the change because the proptest generator that would
be the only thing to notice never produces an invalid value to begin with: `any_event_type()`
(`crates/happenstance-core/tests/wire.rs:176-178`) round-trips only values that already
passed `EventType::new()`, and `Tag`'s generator is the same shape. This is the same class
of gap the corpus's rule is written to close for every other value type, and it satisfies
CLAUDE.md's own bar — a plausible wrong implementation, written and run, not merely
proposed.

**The fix is a repair of the clause, not an amendment to it.** Two `wire::decode_rejects_*`
tests are added — one feeding `EventType::deserialize`, one feeding `Tag::deserialize`, a
string carrying a C1 control character and a bidirectional-override character, the same
invalid inputs VT-14's constructor already rejects on construction — and both are cited on
WF-10's `Rule:` line. `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` is
the governing playbook: the set of implementations WF-10 admits does not change, only the
instrument that checks it grows to match the clause's own "every" as written.

**The second direction is narrower than it first read.** The review that raised this
treated `Event`'s field set as tied to its wire mirror by nothing. That is true of
`Serialize` but false of `Deserialize`, which builds `Event` from a struct literal with no
`..`: adding a required field to `Event` produces `E0063` at both the struct's own
constructor site and, independently, at the wire-mirror deserialize site
(`crates/happenstance-core/src/event.rs:363`, `:769`) — the compiler already forces a
contributor who adds a field to stand at the mirror. What it does not do is stop them
filling both sites with `None` and walking away: that measured, `cargo test -p
happenstance-core --all-features` still reports `EXIT=0`, because the round-trip proptests
never vary a field their generator does not know about, and `Serialize for Event` is
untouched because its own literal is checked only against `EventWire`, not against
`Event`. The gap is narrower than "force the contributor to the mirror" — it is "make the
mirror's field set an assertion in both directions." `Event`'s `Serialize` is changed to
destructure `self` exhaustively (`let Event { event_type, data, tags, metadata } = self;`,
no `..`), so a new field fires `E0027` at the serialize mirror symmetrically with the
`E0063` deserialize already produces. One line, landed with a comment naming the wrong
implementation it forbids, matching the convention the crate's `serde` module already
uses elsewhere.

## Alternatives rejected

A `#[cfg(test)]` source-reading assertion that the two field sets stay equal was
considered and set aside in favour of the compiler-enforced destructuring: Rust cannot
express field-set equality directly, and a second place that knows about the mirror pairs
is a worse instrument than one line that fails to compile. Extending exhaustive
destructuring to every other wire mirror (`SequencedEventWire`, `EventIdWire`, `GuardWire`,
`QueryItemWire`) is not taken now, on the argument that those types are smaller and
frozen; `Event` is the one whose asymmetry was measured.

## What this does not settle

Neither test nor the destructuring change is written yet. Whether the proptest generators
should gain a `prop_oneof!` arm producing genuinely invalid strings for the decode
direction — a stronger instrument than two hand-written tests — is a larger change left to
whoever next touches the wire-format generators. Whether the same obligation applies to
VT-19 and other `[FROZEN]` clauses that impose constructor invariants on value types not
yet audited this way is also open.
