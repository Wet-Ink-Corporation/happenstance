# Staged: correcting ADR-0021's serde attribution

**Staged 2026-08-15** for the next `/redkiln:kb-ingest` wave. **Not an atom.**
Run it on its own worktree branch with a **suffixed wave id** (the `2026-08-15-intake`
id is taken by the wave this document corrects), and **do not sweep `README.md` into
the default glob** at the approval gate.

Long-form record to amend in the same wave:
[`references/adr/0021-payload-evolution-and-codec-tag.md`](../../references/adr/0021-payload-evolution-and-codec-tag.md).
That file is **not** immutable and may be edited directly; the atom may not.

---

## The defect

`.kb/decisions/0021-payload-evolution-and-codec-tag.md:141-144` rejects a
`serde`-encoded framing region on three grounds, and **the third one states a binding
constraint backwards**:

> a `serde`-encoded framing region (unreadable under `--no-default-features`, and a
> `postcard`-only build could not read a `json`-only build's tag — **also barred by
> ADR-0003, which forbids `serde` in `happenstance-core`'s default features**)

The framing region is written by **`happenstance`**, the typed layer — one crate above
the port. ADR-0003 does not merely permit `serde` there, it *positively assigns*
encoding to that crate:
[`.kb/decisions/0003-opaque-payloads.md:17-18`](../decisions/0003-opaque-payloads.md) —
*"Encoding and decoding — a `Codec`, a `DomainEvent` mapping — belong to
`happenstance`, the layer above."* ADR-0003 constrains `happenstance-core` only, and
after ADR-0006's rename the crate name in that constraint says the opposite of what it
used to; `CLAUDE.md`'s binding constraint 2 warns about exactly this reading.

So the attribution inverts the constraint it cites. Reading the atom as written, an M3
implementer would conclude that `serde` is barred from the crate whose entire job is
encoding — forbidding the thing the ADR-0006 split exists to allow.

**The decision itself survives.** The other two grounds carry the rejection on their
own, and neither depends on ADR-0003. Nothing about where the tag lives changes.

## Provenance — this is an ingest defect, not a story defect

The story's staged deliverable was correct. At `e33dc9f`,
`.kb/_intake/0021-payload-evolution-and-codec-tag.md:180-181` gave **only** the two
sound reasons:

> **a `serde`-encoded framing region** — unreadable under `--no-default-features`, and
> a `postcard`-only build could not read a `json`-only build's tag;

and `:202-208` (*"`serde`, in the right direction"*) stated the boundary correctly,
in terms. The ADR-0003 clause was added during distillation at wave `a28322b`. This is
EC-004 in the ADR-0021 spec (*"Rewrite before the wave — after it the atom is
immutable"*) and NF-004, arriving one step later than the edge case anticipated.

## The op: supersede `kb-decision-0021`

Not an edit. `.kb/decisions/README.md:9-13` makes an accepted decision's body immutable
and `redkiln validate --kb` checks each one against `HEAD`, so rewording the atom fails
the gate by design. The corpus's own route is a superseding atom plus a metadata flip on
the old one.

Two ops:

1. **Mint** `.kb/decisions/0031-adr-0021-serde-attribution-correction.md` carrying
   `supersedes: [kb-decision-0021]`. The number `0031` is the next free slot in both
   `.kb/decisions/` and `references/adr/` (0030 is the highest taken); the wave assigns
   the final id.
2. **Flip** `kb-decision-0021`'s frontmatter to `status: superseded` and
   `superseded_by: kb-decision-0031`. That metadata flip is the only edit it receives —
   its body stays verbatim, because the older commits' reasoning only makes sense with it.

This is a **repair**, not an amendment, on `README.md:22-25`'s mechanical test: the set
of implementations the decision admits is unchanged. All three of ADR-0021's decisions,
its falsifier, its reversibility and its untagged-event rule carry over intact. Only a
justification for one rejected alternative is withdrawn.

### What the superseding atom must say

- **Restate the rejection of a `serde`-encoded framing region on its two sound
  grounds**, and on those alone:
  - **codec-feature independence** — reading the framing region is *how* the payload
    codec is discovered, so the region cannot be encoded by a `Codec` and must not
    depend on any codec feature: a build with only `postcard` enabled must still read a
    tag written by a build with only `json`;
  - **no added dependency** — the region must be readable with no default features and
    on `wasm32`, so it is not `serde`-encoded and is not JSON.
- **Drop the ADR-0003 attribution entirely.** Do not replace it with a corrected
  ADR-0003 clause: ADR-0003 has nothing to say about this alternative in either
  direction, and a corrected-but-present citation would keep inviting the same
  misreading. `depends_on: [kb-decision-0021]`, `related: [kb-decision-0003,
  kb-decision-0006]` is where the relationship belongs.
- **State the boundary in the right direction, once**, so the atom that supersedes this
  one cannot re-acquire the inversion: ADR-0003 constrains `happenstance-core`, whose
  `serde` feature covers envelope types only; `happenstance` is the typed layer whose
  job *is* encoding, and `serde` there is the split working. Payloads stay `Bytes` at
  the port, and the framing region is not `serde`-encoded at all.
- Carry `phase: 7` and `reversibility: low`, matching the atom it supersedes.
- `source_paths` lists this document and
  `references/adr/0021-payload-evolution-and-codec-tag.md`.

### Maps to re-sync

- `.kb/maps/decision-map.md:138` — ADR-0021's row moves to `superseded`, with
  `superseded by ADR-0031` in the last column, and a row for ADR-0031 is added.
  The partial-supersession chain section gains this lineage.
- `.kb/maps/domain-map.md:189-194` — the typed-layer domain's ADR-0021 bullet points at
  the superseding atom.

## Also in this wave: one long-form tightening, already applied

Rule 3 of Decision 1 (`references/adr/0021-…:156-158`) claimed the framing region's
no-dependency property is covered by `--no-default-features` as a gate step. Both
`--no-default-features` steps in `cargo xtask ci` are scoped `-p happenstance-core`
(`xtask/src/main.rs:216-223` and `:532-544`), and the framing region is `happenstance`'s,
one crate above — neither step ever builds it. It holds via the workspace-wide
`cargo hack check --workspace --feature-powerset --no-dev-deps` step (`:605-615`)
instead, plus the `wasm32` powerset (`:623-652`), both behind a `cargo hack` probe.

**This edit is already made in the long-form record** and needs no op. It was made
**line-count-neutral on purpose**: rule 3 stays three lines and now points to a new
appendix section, *"Which instrument covers rule 3"*, appended after `## Provenance`.
Roughly a dozen `references/adr/0021-…:NNN` citations in this story's `report.md`,
`_ledger.md` and `implementation-report.md` index into that file, and rewriting rule 3
in place would have shifted every one of them past line 158. Sections at `:419`, `:436`
and `:449` keep their line numbers; the file grows 454 → 499 lines purely by appending.
Noted so the wave does not read the growth as drift.

## A citation the reviewer flagged that is in fact correct — do not "fix" it

A review pass proposed widening `crates/happenstance-core/src/projection.rs:152-154` to
`:152-155` in `references/adr/0020-…:133`, `.kb/decisions/0020-…:104-105` and
`.kb/decisions/0021-…:139`, on the grounds that the quoted sentence runs one line
further. **It does not.** Verified against `HEAD` and against wave commit `a28322b`:

```
152:    /// fallible `parse` beside this constructor would be worse than either
153:    /// choice: two constructors enforcing different rules is the defect that
154:    /// makes an invalid value reachable through the weaker one.
155:    pub fn new(value: impl Into<String>) -> Self {
```

`:152-154` covers the quoted text exactly; line 155 is the function signature. Widening
to `:155` would pull code into a range that quotes prose — a worse citation than the one
it replaces. The records' repair of the stale `:47-61` range (correct at planning commit
`ae77ac4`, where the sentence ended at line 61) stands as made.

**No atom needs an edit for this, and none should get one.** It is recorded here and in
the long-form record (*"A second citation checked — and found correct"*) so the finding
is disposed of rather than re-raised and applied. The superseding atom should carry no
citation change.
