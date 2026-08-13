---
item: "HS-S0019"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0021 — payload evolution and the codec tag's home

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three things about this ledger are unusual and are unusual on purpose, so an implementer does not
"work around" them. **This story *takes* three decisions rather than recording ones already taken** —
`_design.md:674-679` hands the codec-tag home to this story explicitly — so AC-002, AC-004 and AC-005
are three independently failing rows, and a record that settles one and leaves two open flips none of
them. **The substantive rows (AC-002…AC-007) are verified by human review**, because no compiled
assertion can prove a decision was recorded correctly; the testing brief says exactly this for the
whole class (`_decomposition.md:790`, AC-012 row: *"Record, not a test"*). And **AC-008 is
handoff-verified**: the atom it asserts is minted by `/redkiln:kb-ingest` on its own worktree branch,
so its evidence is that wave's commit sha plus the `redkiln validate --kb` output — never a
hand-authored `.kb/decisions/0021-*.md`, which is the anti-pattern reverted at `0269720`. An
implementer who cannot produce the wave sha leaves AC-008 `satisfied: false`; that is the correct
outcome.

```yaml
- id: AC-001
  criterion: >-
    GIVEN the M3 implementer is about to write `Codec` and needs to know where the tag goes, WHEN
    they look for ADR-0021, THEN a staged decision document exists at
    `.kb/_intake/0021-payload-evolution-and-codec-tag.md`, composed as a decision record —
    hazard/context, the three decisions, the alternatives that lost, the consequences, the residual —
    and carrying a proposed frontmatter block for the ingest run to author from (`adr_id: ADR-0021`,
    `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, a stated
    `reversibility`, `depends_on`/`related` naming the real atom ids `kb-decision-0003`,
    `kb-decision-0006`, `kb-decision-0007`, `kb-decision-0016`, and `source_paths` that all resolve);
    it lands at no other path, because `_intake` is `/redkiln:kb-ingest`'s only input.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "test -f .kb/_intake/0021-payload-evolution-and-codec-tag.md; cargo xtask affected --base main; redkiln verify --grain story (allowed-globs fence); frontmatter reviewed against .kb/decisions/0016-the-wire-format.md:1-45"

- id: AC-002
  criterion: >-
    GIVEN an application will one day hold two encodings in one store and the M3 implementer must
    write the tag somewhere, WHEN they read this record, THEN it names exactly one home —
    `Event::metadata` (`crates/happenstance-core/src/event.rs:379`, read at `:400`) or `Tags` — never
    "either is fine", states the cost it accepted out loud, and shows the choice was decided on cost
    and not on ergonomics by recording that the public surface is invariant under it (`Codec::TAG` is
    a `&'static str` either way, `_design.md:441-445`). If `Event::metadata` wins, the record
    additionally discharges its accepted cost: how an application's own causation/correlation
    metadata coexists with a typed-layer-private structure inside a public opaque field.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "Human review at the report gate against .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md:674-679 and _decomposition.md:548-570; one-home claim checked against crates/happenstance-core/src/projection.rs:47-61. No compiled test by design (_decomposition.md:790)"

- id: AC-003
  criterion: >-
    GIVEN the record's own falsifier is "name the adapter change this choice forces", WHEN the
    maintainer asks why the losing home lost, THEN the record has applied that falsifier mechanically
    and to both candidates, and has read VT-3 in the direction that actually decides it: `metadata` is
    admissible only because nothing below the port needs to see the tag — VT-3 requires that any value
    a store, a peer, a conformance rule or a query must see be carried in `EventType` or `Tags`
    (`spec/SPECIFICATION.md:629-637`, `[FROZEN]`) — while `Tags` forces a change in every adapter by
    obliging all of them to store, index and match on it, consumes tag budget
    (`crates/happenstance-core/src/limits.rs:27`), and makes a re-encoding change which `QueryItem`s
    the event satisfies, so a consistency boundary silently changes shape as a consequence of a
    storage-format migration that was supposed to be invisible.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "Human review with spec/SPECIFICATION.md:629-637 re-opened (both halves of VT-3 used); the named mutant at discover.md:82-107 appears as a rejected alternative with its failure mode. Mechanical negative: git diff --name-only shows no spec/ path"

- id: AC-004
  criterion: >-
    GIVEN `EventType::from_static` is `const` and accepts `"CourseDefined.v2"` happily
    (`crates/happenstance-core/src/event.rs:108-119`) so the naive "yes" compiles, round-trips and is
    indistinguishable from a correct decision at review time, WHEN the record answers whether
    `EventType` carries a version suffix, THEN it answers and carries the query-naming consequence:
    every `Query` already written names its types exactly, including
    `examples/course-subscriptions/src/main.rs:114-125`, so a suffix is invisible to them and the
    first upcast makes existing decision models read an empty log — capacity resolves to `None`, the
    handler bails with "course does not exist", and the append condition that was to protect the
    boundary matches nothing. A suffix is therefore admissible only together with a stated rule for
    how a query names versions; a bare "yes" fails this row.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "Human review with examples/course-subscriptions/src/main.rs:114-125 open; the record either answers no-suffix and names what carries evolution instead, or answers yes and states the query-naming rule. Consequence-side proof deferred to M3/M6 per _decomposition.md:783"

- id: AC-005
  criterion: >-
    GIVEN `EventStore` is `[FROZEN]` and offers `read`, `append`, `head` and `read_decision_model` and
    no hook of any kind (`crates/happenstance-core/src/store.rs:110-159`), so "no hook is needed" is
    the answer that requires no work and leaves no trace, WHEN the record answers the upcaster
    question, THEN the answer is earned: "no" names the upcasting strategy that makes it true —
    decode-time tolerance, which `DomainEvent::decode(codec, event_type, data) -> Result<Self,
    CodecError>` already permits with no port change (`_design.md:396-408`) — and "yes" is a defect
    entry naming the clause ID and routed to a decision record under project AC-012
    (`project.md:204-206`), never a line edit of the frozen crate and never a proposed amendment.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "Human review against crates/happenstance-core/src/store.rs:110-159 and _design.md:396-408; a \"no\" with no named strategy fails (discover.md:121-128). Mechanical negative: git diff --name-only shows no crates/, examples/, xtask/ or spec/ path"

- id: AC-006
  criterion: >-
    GIVEN the codec tag exists so that one store can hold more than one encoding — which makes this a
    migration decision by construction — and events already written carry no tag at all, WHEN the M3
    implementer meets an untagged event, THEN the record has already said what a reader does (a stated
    default, a refusal, or a rule), rather than leaving M3 to invent one, and has bounded that answer
    by the two things this PR may not change: no adapter rewrites or reinterprets stored events (VT-3,
    `spec/SPECIFICATION.md:629-637`) and no backfill runs through a hook `EventStore` does not have
    (`crates/happenstance-core/src/store.rs:110-159`). An answer that needs either is the AC-012
    defect entry, not an amendment.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "Human review: the untagged-event rule is a sentence a decoder can implement, and CodecError::UnknownTag (_design.md:461-462) is either the stated outcome or explicitly not; read back against .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/codec-and-feature-forwarding/discover.md:50"

- id: AC-007
  criterion: >-
    GIVEN a successful ingest clears `_intake` (`.kb/_intake/README.md`), so the staged document is
    not a durable home for the reasoning, WHEN the wave runs, THEN the long-form record already exists
    at `references/adr/0021-payload-evolution-and-codec-tag.md` carrying the cost comparison, the
    rejected homes with their failure modes and the compiler-facing detail a ~100-line atom cannot
    hold; the staged document names it in the `source_paths` it proposes; and the record states the
    seam against ADR-0016 — which owns the replication wire format
    (`.kb/decisions/0016-the-wire-format.md`) — citing it as `related` so the corpus does not acquire
    two competing answers to "how is a payload encoded".
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "test -f references/adr/0021-payload-evolution-and-codec-tag.md; proposed source_paths lists both files, matching .kb/decisions/0029-msrv-raised-to-1-97-1.md; ADR-0016 seam checked by reading .kb/decisions/0016-the-wire-format.md:12-38"

- id: AC-008
  criterion: >-
    GIVEN AC-016 is a sequencing obligation — the record must exist before the code it governs — and
    atoms are authored by the ingest path and never by hand (`0269720` is the reverted attempt), WHEN
    a human runs `/redkiln:kb-ingest` over this document and its slice-mate in one wave on its own
    worktree branch with a suffixed wave id, THEN
    `.kb/decisions/0021-payload-evolution-and-codec-tag.md` exists as an accepted atom,
    `.kb/maps/decision-map.md` carries its row, `redkiln validate --kb` and `redkiln doctor` are
    clean, and `git log --diff-filter=A` shows that atom added by the wave's commit and not by this
    story's PR.
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "redkiln validate --kb && redkiln doctor on the ingest branch; git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md names the wave commit; .kb/maps/decision-map.md carries the row"
```
