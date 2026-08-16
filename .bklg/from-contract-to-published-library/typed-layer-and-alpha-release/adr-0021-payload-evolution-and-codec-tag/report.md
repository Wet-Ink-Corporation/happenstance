---
item: "HS-S0019"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — ADR-0021 — payload evolution and the codec tag's home

## Findings Ledger

> **Seven of eight ACs satisfied. AC-008 is not, by construction.** It asserts an
> accepted `.kb/decisions/0021-payload-evolution-and-codec-tag.md`, and only a
> **human-invoked `/redkiln:kb-ingest` wave** may author an atom, on its own
> worktree branch — which is why the evidence AC-008 asks for is a *wave commit sha*
> and cannot exist in this PR's diff. Hand-authoring it is the anti-pattern reverted
> at `0269720`.

**This story took three decisions rather than transcribing one.** `_design.md:674-679`
hands them over in terms, and no later stage would have taken them: M3 consumes the
siting, it does not choose it.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **VT-3's second half is what decides the siting, and it is the half most easily skipped** | `spec/SPECIFICATION.md:632-633`: *"Any value that a store, a peer, a conformance rule or a query must be able to see MUST be carried in the `EventType` or in `Tags`."* Used at `references/adr/0021-payload-evolution-and-codec-tag.md:61-94` via the prior question *does anything below the port need to see the codec tag?* | None. If a future change ever makes something below the port need the tag, VT-3 **requires** `Tags` and this decision is falsified — that is stated in the record as the condition, not left implicit |
| VT-3's `Rejects:` line is the mirror case, not a counter-example | `spec/SPECIFICATION.md:643-646` rejects an ingest path writing **origin identity** into `metadata` — a value a peer must see. The codec tag is a value nothing below the port may see. Handled at `references/adr/0021-…:88-94` | None |
| The falsifier is not symmetric, and that asymmetry is the argument | Six-row table at `references/adr/0021-…:104-120`. `Tags` obliges every adapter to store, index, match on and budget the tag (`crates/happenstance-core/src/limits.rs:27`, 64 tags); `Event::metadata` obliges none | None |
| **The `Tags` siting corrupts silently, and nothing mechanical rejects it** | `references/adr/0021-…:122-136`: re-encoding changes the tag set → changes which `QueryItem`s the event satisfies → changes which decision models read it. `Tags::from_pairs([("codec","json")])` validates (`crates/happenstance-core/src/tag.rs:304-312`) and the whole gate stays green | This is the named mutant (`discover.md:82-107`). It survives as a rejected alternative with its failure mode, so a future reader meets it as a fork |
| A version suffix is invisible to every query already written | Matching is exact equality — `self.types.binary_search(event_type).is_ok()` (`crates/happenstance-core/src/query.rs:113`). The example's query at `examples/course-subscriptions/src/main.rs:114-125` names `"CourseDefined"` exactly (`:29`); the first upcast makes `capacity` `None` and the handler bail at `:158-160` | Consequence-side proof is M3/M6's by design (`_decomposition.md:783`) |
| **`EventStore` has four required methods, and `read_decision_model` is not one** | `crates/happenstance-core/src/store.rs:93-269` — `read` `:119`, `append` `:213`, `head` `:248`, `contains_event_id` `:268`; `read_decision_model` is a free function at `:321`. `_design.md`, `_decomposition.md` and this spec all recite it as a fifth method | A **repair**, not an amendment: AC-005's conclusion holds under either spelling, because neither contains a hook |
| Refusing untagged events is unimplementable, not merely strict | `references/adr/0021-…:280-288`. No adapter may rewrite stored events (VT-3) and no backfill may run through a hook that does not exist, so there is no legal repair for a log the typed layer refuses to read | The fallback rule is also the mechanism any future re-siting would reuse (`:394-417`) |
| `reversibility` is `low`, and deliberately not inherited from ADR-0016 | `.kb/decisions/0016-the-wire-format.md` is `high` because its format is private and unpublished. This one is inherited by `0.2.0-alpha.1` and by every event a user writes; reversal means re-siting a tag on stored events, which VT-3 forbids any adapter doing on their behalf | NF-006 discharged |
| Nothing compiled changed | `git diff --name-only` for this checkpoint lists only the two markdown artefacts and this story's `.bklg` folder. VT-3 and every `ES-*` clause are byte-identical | NF-001 discharged; AC-A02 and AC-012's routes honoured |

## Acceptance

| AC | Status | Verified by |
| --- | --- | --- |
| **AC-001** — staged document at the ingest mount point, composed as a record, with a proposed frontmatter block | **satisfied** | `test -f` red → green; `.kb/_intake/0021-…:24-88` (`adr_id: ADR-0021`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, `reversibility: low`, `depends_on: kb-decision-0003, kb-decision-0006`, `related: kb-decision-0007, kb-decision-0016, kb-open-question-human-readable-encoding-limits-001`, six resolving `source_paths`); `cargo xtask affected --base main` passed |
| **AC-002** — exactly one home, its cost stated, decided on cost not ergonomics | **satisfied** | `Event::metadata`. `references/adr/0021-…:96-102`, `:138-175` (the two-region split, five rules, the stated price), `:306` ("either home is fine" rejected by name), `:318-322` (one-home claim), `:394-399` (surface invariant, `Codec::TAG` a `&'static str` either way) |
| **AC-003** — the falsifier applied mechanically to **both** candidates, VT-3 read in both directions | **satisfied** | `references/adr/0021-…:104-136` (six-row table + the migration row) and `:61-94` (both halves of VT-3, with the `Rejects:` line handled). Mechanical negative: no `spec/` path in the diff |
| **AC-004** — the version-suffix answer, with the query-naming consequence | **satisfied** | "No suffix." `references/adr/0021-…:177-221`, with the query quoted verbatim and the failure traced to `main.rs:158-160`; what carries evolution instead is named at `:214-221` |
| **AC-005** — the hook answer, earned | **satisfied** | "No hook", with decode-time tolerance named as the strategy (`references/adr/0021-…:239-256`), a falsifier stated (`:258-262`) and the "yes" route fixed as an AC-012 defect entry (`:264-267`). Mechanical negative: no `crates/`, `examples/`, `xtask/` or `spec/` path in the diff |
| **AC-006** — the untagged-event rule, bounded by VT-3 and the absent hook | **satisfied** | `references/adr/0021-…:275-278` states it as one implementable sentence; `UnknownTag` is **explicitly not** the outcome and is left one meaning at `:294-299`; both bounds named at `:280-288`; read back against `codec-and-feature-forwarding/discover.md:41-51` |
| **AC-007** — the long-form record exists, is named in `source_paths`, and states the ADR-0016 seam | **satisfied** | `test -f` red → green, 454 lines against the staged document's 254; `source_paths` at `.kb/_intake/0021-…:82-88`; the seam in one sentence at `references/adr/0021-…:331-334`, with `related: kb-decision-0016` and an explicit refusal to inherit its `reversibility` |
| **AC-008** — the wave mints the atom, the decision map carries its row, `validate --kb` clean | **NOT satisfied** | Blocked on the human-invoked `/redkiln:kb-ingest` wave. `git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md` returns nothing. Ledger row left `satisfied: false` with empty evidence, deliberately |

**The one dependency that is missing, stated exactly:** a human must run
`/redkiln:kb-ingest` over `.kb/_intake/0020-fold-query-agreement.md` **and**
`.kb/_intake/0021-payload-evolution-and-codec-tag.md` in **one** wave, on its own
worktree branch, with a suffixed wave id, dropping `.kb/_intake/README.md` from the
default glob at the approval gate. Until then project **DoD 3** is undischarged for
both M1 stories, and M3 (`codec-and-feature-forwarding`) is reading a document that
is about to be deleted from `_intake`.

## Knowledge Harvest

- **The atom this story stages**, ADR-0021, is the harvest. It is not promoted here:
  atoms are the ingest path's to author, never a story's.
- **A transferable rule, candidate for a playbook atom at closeout:** *a "MUST NOT
  parse" clause licences nothing on its own; read the paired positive obligation, and
  turn the siting into a prior question about who must see the value.* VT-3's first
  half would permit hiding anything in `metadata`; its second half is what makes the
  hiding legitimate here and illegitimate for replication identity. The same clause,
  read once, gives opposite answers to two siting questions.
- **A second one:** *state a "no" with the strategy that makes it true and the shape
  that would break it.* "No hook is needed" is the answer that requires no work and
  leaves no trace; attaching decode-time tolerance and an out-of-event upcast turns
  it from an assumption into a claim someone can refute.
- **A third, about `reversibility`:** the field is not inheritable from a
  neighbouring decision on the same subject. ADR-0016 and ADR-0021 both concern
  payload encoding and their reversibility differs by two grades, because one format
  is unpublished and the other is inherited by every event a user writes.
- **Nothing under `.kb/open-questions/` is resolved by this story.**
  `kb-open-question-human-readable-encoding-limits-001` is cited as `related` to mark
  a seam and is left untouched; it is phase 9's.
