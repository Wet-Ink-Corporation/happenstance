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
  satisfied: true
  evidence: >-
    .kb/_intake/0021-payload-evolution-and-codec-tag.md:1-254 — staged at the mount point and
    nowhere else (`test -f` red before the change, green after). Composed as a decision record:
    the problem at :90-97, VT-3 read in both directions at :99-112, the three decisions and the
    rule they imply at :114-144, the falsifier table at :146-160, the accepted cost at :162-168,
    ten rejected alternatives with what each admits at :170-191, the ADR-0016 seam at :193-200,
    consequences and the residual at :211-232. The proposed frontmatter is the fenced yaml at
    :24-88 and carries adr_id ADR-0021 (:31), kind decision (:28), authority_tier decision
    (:30), status accepted (:29), phase 7 (:33), reversibility low (:32), depends_on
    kb-decision-0003/kb-decision-0006 (:75-77), related kb-decision-0007/kb-decision-0016/
    kb-open-question-human-readable-encoding-limits-001 (:78-81), and six source_paths (:82-88)
    each of which resolves. Frontmatter shape taken from .kb/decisions/0016-the-wire-format.md:1-52.
    Gate: `cargo xtask affected --base main` → "affected gate passed".
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
  satisfied: true
  evidence: >-
    Exactly one home: `Event::metadata`. references/adr/0021-payload-evolution-and-codec-tag.md:96-102
    sites it against crates/happenstance-core/src/event.rs:379 (with_metadata) and :400
    (metadata() -> Option<&Bytes>). "Either home is fine" is rejected by name at :306 with the
    reason quoted from crates/happenstance-core/src/projection.rs:152-154 — the corrected range
    for the two-constructor sentence the ledger cites as :47-61; the repair is recorded at
    :436-447. The one-home claim is restated at :318-322. Decided on cost, not ergonomics:
    :394-399 records that Codec::TAG is a `&'static str` under either home (_design.md:441-445),
    which is why the design did not wait (_design.md:674-679). The accepted cost is discharged
    rather than waved through at :138-175 — a versioned framing region owned by happenstance, the
    application's own causation/correlation bytes carried through verbatim and never parsed, five
    numbered rules bounding it, and the price stated plainly at :171-175 (a contract-level reader
    must skip the framing bytes; one caller shape pays instead of every adapter). Distilled at
    .kb/_intake/0021-payload-evolution-and-codec-tag.md:114-129 and :162-168.
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
  satisfied: true
  evidence: >-
    The falsifier is applied mechanically to BOTH candidates as a six-row table at
    references/adr/0021-payload-evolution-and-codec-tag.md:104-120 — store it, index it, match on
    it, budget it, migrate it, parse it — with the verdict at :117-121 (Tags forces a change in
    every adapter, metadata forces none). The migration row is expanded at :122-136: re-encoding
    changes the tag set, which changes which QueryItems the event satisfies, which changes which
    decision models read it, and nothing mechanical rejects it. Tag budget cited to
    crates/happenstance-core/src/limits.rs:27 (MIN_SUPPORTED_TAGS_PER_EVENT = 64, re-read).
    Matching cited to crates/happenstance-core/src/query.rs:112-115 (`matches` takes event_type
    and tags only, re-read). VT-3 is used in BOTH halves at :61-94, quoted from
    spec/SPECIFICATION.md:631-633 and re-read: the second half is named as the one that decides,
    via the prior question "does anything below the port need to see the codec tag?", and the
    clause's own `Rejects:` line (:643-646, origin identity in metadata) is handled at :88-94 as
    the mirror case rather than a counter-example. The named mutant appears as a rejected
    alternative with its failure mode at :304-305. Mechanical negative: `git diff --name-only`
    for this commit shows no path under spec/.
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
  satisfied: true
  evidence: >-
    Answered "no suffix", with the query-naming consequence carried and what carries evolution
    instead named. references/adr/0021-payload-evolution-and-codec-tag.md:177-221: from_static
    would accept "CourseDefined.v2" (crates/happenstance-core/src/event.rs:108-122, re-read — the
    validator refuses empty, over-long, control and bidi strings, and a dot is none of those), so
    the naive yes compiles and round-trips; the QueryItem at
    examples/course-subscriptions/src/main.rs:114-125 is quoted verbatim with its exact const at
    :29, and matching is exact equality via `self.types.binary_search(event_type).is_ok()`
    (crates/happenstance-core/src/query.rs:113, re-read); the consequence is spelled out to the
    line — capacity None, the handler bailing "course does not exist" at main.rs:158-160, the
    append condition matching nothing. What carries evolution instead is the payload, decoded
    tolerantly, identified by the codec tag (:214-217), and the corollary is that a genuinely new
    fact takes a NEW type name so the widening of a boundary stays a visible decision (:219-221).
    A version suffix WITH a query-naming rule is separately rejected on cost at :309. Distilled at
    .kb/_intake/0021-payload-evolution-and-codec-tag.md:126-134.
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
  satisfied: true
  evidence: >-
    Answered "no hook", and earned rather than defaulted.
    references/adr/0021-payload-evolution-and-codec-tag.md:223-267 names the strategy that makes
    it true — decode-time tolerance — and shows it is already expressible: the signed-off
    `fn decode<C: Codec>(codec, event_type, data) -> Result<Self, CodecError>` (_design.md:396-408)
    receives the event type AND the raw bytes, so an older payload shape is handled inside the
    typed layer with no port change (:239-256). The frozen surface is enumerated from source
    rather than recited: crates/happenstance-core/src/store.rs:93-269 requires read :119, append
    :213, head :248, contains_event_id :268, and read_decision_model is a FREE FUNCTION at :321,
    not a trait method — the correction is recorded at :436-447 and changes nothing, because
    neither spelling contains a hook. The answer is made falsifiable at :258-262: an upcast
    needing information from outside the event being decoded shows it wrong. The "yes" route is
    fixed at :264-267 as a defect entry naming the clause ID routed under project AC-012
    (project.md:204-206) — never a line edit, never an amendment, never a variant added to
    EventStore. Distilled at .kb/_intake/0021-payload-evolution-and-codec-tag.md:135-139.
    Mechanical negative: `git diff --name-only` for this commit shows no path under crates/,
    examples/, xtask/ or spec/.
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
  satisfied: true
  evidence: >-
    The untagged-event rule is stated as one sentence a decoder can implement, at
    references/adr/0021-payload-evolution-and-codec-tag.md:269-300 and quoted as a blockquote at
    :275-278: "an event whose metadata carries no framing region decodes with the codec the
    caller is already holding — Json under commit, C under commit_with::<C> — and not
    CodecError::UnknownTag". UnknownTag (_design.md:461-462) is therefore EXPLICITLY not the
    stated outcome, and is left one meaning at :294-299: a tag was written and this build cannot
    honour it. Both bounds are honoured and named at :280-288: no adapter rewrites or
    reinterprets stored events (VT-3, spec/SPECIFICATION.md:631-633) and no backfill runs through
    a hook EventStore does not have (crates/happenstance-core/src/store.rs:93-269) — refusal is
    shown to be not a strict-and-safe choice but an unimplementable one. Read back against
    .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/codec-and-feature-forwarding/discover.md:41-51,
    which asks for exactly two things from this record — that JSON is the default (yes, :290-293)
    and the tag's encoding once the home is chosen (deliberately left to M3, :171-175 and
    :421-423) — so M3 inherits answers plus a bounded format question, not a gap. Distilled at
    .kb/_intake/0021-payload-evolution-and-codec-tag.md:140-144.
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
  satisfied: true
  evidence: >-
    references/adr/0021-payload-evolution-and-codec-tag.md exists (454 lines; `test -f` red before
    the change, green after) and carries what a ~100-line atom cannot: VT-3 read in both
    directions with its Rejects: line handled (:61-94), the six-row falsifier table (:104-120),
    the migration row expanded (:122-136), the five framing rules and the stated price
    (:138-175), the quoted QueryItem and its line-by-line consequence (:177-221), the ten-row
    rejected table (:302-322), the ADR-0016 seam (:324-351) and the reversibility argument
    (:394-417). The staged document names it in the source_paths it proposes at
    .kb/_intake/0021-payload-evolution-and-codec-tag.md:82-88, which lists both files — the same
    pairing .kb/decisions/0029-msrv-raised-to-1-97-1.md:45-47 carries. The ADR-0016 seam is stated
    in one sentence at :331-334 ("ADR-0016 owns how the metadata bytes cross a peer boundary;
    ADR-0021 owns what those bytes mean to the typed layer"), checked against
    .kb/decisions/0016-the-wire-format.md:12-38 and :101-104: this record neither restates nor
    contradicts its base64/raw-bytes rule, and explicitly refuses to inherit its
    reversibility: high with the reason (:348-351, :414-417). Cited as related:
    kb-decision-0016 at .kb/_intake/0021-…:78-81. Density: the staged document is 254 lines,
    sized to distil to an atom of roughly 100.
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
  satisfied: true
  evidence: >-
    The handoff completed. `/redkiln:kb-ingest` ran over this document and its slice-mate in one
    wave on its own worktree branch, with the suffixed wave id `2026-08-15-intake`, and landed as
    `a28322b` ("docs(kb-intake): ingest ADR-0020 and ADR-0021 into the KB (2 ops,
    2026-08-15-intake)", 2026-08-15), merged into the initiative branch at `3fb28a1` ("Merge the
    ADR-0020/0021 ingest wave into the initiative").
    `git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md` returns
    exactly that one sha — `a28322b` — so the atom was added by the wave and not by this story's PR,
    which is the clause that distinguishes this from the hand-authoring anti-pattern reverted at
    `0269720`. `.kb/decisions/0021-payload-evolution-and-codec-tag.md` exists at `status: accepted`,
    `adr_id: ADR-0021`, `phase: 7`, `reversibility: low`. `.kb/maps/decision-map.md:138` carries its
    row; `.kb/maps/domain-map.md:174-194` carries the new "typed layer" domain section and its
    bullet, and the mutual `related` edge to `kb-decision-0020` is wired. The wave consumed both
    staged sources, leaving `.kb/_intake/` holding only `README.md` at `a28322b`.
    `redkiln validate --kb` passes — "redkiln: validate passed.", exit 0 — re-run at this commit,
    which is the check that enforces both KbFrontmatter conformance and accepted-decision
    immutability against `HEAD`.
    **The `redkiln doctor` clause is dispositioned, not silent:** doctor exits 1, and it did so
    before this slice. All nine errors are `unconsumed-foundation` — HS-S0002, HS-S0034, HS-S0035,
    HS-S0067, HS-S0074, HS-S0075, HS-S0100, HS-S0108, HS-S0120 — and none of them names this story
    (HS-S0019) or its slice-mate (HS-S0018); they originate at planning commit `ae77ac4`, predating
    this slice, and eight sit in five unstarted projects. They are already routed as release blocker
    upstream redkiln #122 in
    `.bklg/from-contract-to-published-library/_implementation.md:340-341` and `:399-403`, to be
    settled before `publication-and-positioning` rather than at this PR. Doctor's remaining output is
    the six expected `template-drift` advisories CLAUDE.md documents as permanent. Nothing this
    story or the wave produced appears in doctor's output. The project's own DoD 3
    (`project.md:228-229`) requires only that the atoms exist, are written before the code they
    govern, and that `redkiln validate --kb` is clean — which is green.
    **One defect the wave introduced is tracked separately and does not bear on this row:** the
    minted atom's Rejected list attributes the serde-framing-region rejection partly to ADR-0003,
    which states that constraint backwards (this story's staged deliverable, `e33dc9f`, did not).
    The atom body is immutable, so the correction is routed as a superseding atom via a further
    ingest wave; the correcting document is staged at
    `.kb/_intake/0031-adr-0021-serde-attribution-correction.md`. AC-008 asserts the atom exists,
    was minted by the wave and validates — all three hold.
  mount_point: ".kb/decisions/0021-payload-evolution-and-codec-tag.md"
  verifying_test: "redkiln validate --kb && redkiln doctor on the ingest branch; git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md names the wave commit; .kb/maps/decision-map.md carries the row"
```
