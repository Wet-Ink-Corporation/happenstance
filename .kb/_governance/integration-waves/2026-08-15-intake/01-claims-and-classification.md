# Wave `2026-08-15-intake` — claims and classification

Every claim the extract pass raised, labelled against the accepted decision corpus, with the
destination it was routed to. **Fourteen claims in — sixteen rows, because ADR-0020's main claim
carries two sub-claims worth labelling separately — and two operations out**: two decision atoms
created, nothing else in `.kb/` written, edited, flipped or deleted.

That ratio is the wave's shape and it is not laziness. **Ten of the sixteen rows** are either map
maintenance (collapsed into `mapsImpact`), an explicit non-action the intake states in terms, an
argument folded into an atom it does not get to be, or a correction whose subject lives outside
`.kb/` entirely. A wave that turned each of those into an op would report ten mutations, eight of
which do not exist.

## What "against the accepted decision corpus" means this wave

Twenty-one accepted decision atoms exist (`.kb/decisions/0001`–`0019`, `0029`, `0030`; every one
`status: accepted` except `kb-decision-0002`). The labels are the ones the previous four waves used,
with the same meanings:

| Label | Meaning here |
| --- | --- |
| `aligns` | Restates or applies a rule an accepted atom or a layer README already carries. Nothing new is committed; the claim cites rather than introduces. |
| `extends` | Net-new knowledge with no owner in `.kb/`, contradicting nothing accepted. |
| `conflicts` | Contradicts something accepted, or two sources contradict each other and the wave declines to pick a winner. **Zero occurrences** — and `## Conflicts` below names the four candidates that were checked before that zero was written down. |
| `requires-new-decision` | Cannot be discharged by recording it: an ADR or a human sign-off is needed. **Zero occurrences.** Both decisions arrived already taken, with 411- and 454-line records behind them. |

**This wave authors no decision it did not receive.** ADR-0020 was taken by a human at the
`/redkiln:plan` design sign-off gate on 2026-08-12 and recorded at phase 7 *before the code it
governs exists*, which is project AC-016's whole content. ADR-0021 takes its three answers in its
own record rather than transcribing a signed-off row, because `_design.md:674-679` hands all three
to it in terms — the public surface is invariant under every one of them, `Codec::TAG` being a
`&'static str` either way. Both are transcriptions into atom form; neither is a judgement this wave
made.

## The claim table

### `.kb/_intake/0020-fold-query-agreement.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `0020-C1` | One `decision` atom for ADR-0020, `phase: 7`, `reversibility: medium`, `supersedes: null`. `query()` is **not** on `DecisionModel`; the derivation is `Boundary::query` on a **sealed** trait blanket-implemented for every `DecisionModel` and macro-implemented for tuples of arity 2..=8. It returns `Result<Query, InvalidQuery>` because `QueryItem::new` is fallible and `happenstance-core` is frozen; an *empty* `EVENT_TYPES` becomes a **compile error** via a per-monomorphisation `const` (RS-61-4). `DecisionModel::scope(&self) -> &Tags`, validation paid once through `Tags::from_pairs`. `DecisionModel: Clone`, **not** `Default`. Composition is an internal `macro_rules!` `impl Boundary for (B1, B2)…`, zero caller-visible syntax. Eight rejected alternatives, each with the wrong implementation it admits | **MUST / MUST NOT** throughout — a removal, a return type, a compile error, a borrow, a supertrait present and a supertrait absent, and *"no `unwrap` and no edit to the frozen crate"* | `extends` | **Op 1** — `kb-decision-0020` |
| `0020-C1a` | The five claims are **one shape**: there is nowhere to put a hand-maintained query. The blanket impl makes the derivation free; the **seal** makes it the only one (RS-40-2) | atomicity argument | `aligns` | Op 1, `## Decision`. Carried as the reason the atom is not split five ways (`00`, *the dedup not offered*) |
| `0020-C1b` | Defect candidate **D-1** — `happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs. Nearest clause subject VT-18 (`spec/SPECIFICATION.md:1371-1375`); routed to AC-012's defect log and to a decision record, **never a line edit** of the frozen crate | a `MUST NOT` about the route, not about the crate | `aligns` | Op 1, `## What this decision does not decide`. **No open-question atom** — `02`, Adjudication 8, and one line in `unresolved` so the refusal stays visible |
| `0020-C2` | One row on `.kb/maps/decision-map.md`: ADR-0020, atom link, title, `accepted`, phase 7, supersession `—`. The extract flags that the map already carries a same-day `## 2026-08-15 checkpoint-progress ADR (ADR-0030)` section from the earlier wave | map maintenance | `aligns` | **No op of its own.** `mapsImpact.decisionMap` on Op 1, merged with `0021-C5` into **one** wave section carrying two rows — `02`, Adjudication 4 |
| `0020-C3` | An entry on `.kb/maps/domain-map.md` *"under the typed layer"*. The extract correctly declines to resolve which section that is and hands the placement to adjudication | none — advisory | `aligns` | **No op of its own.** `mapsImpact.domainMap` on Op 1. The placement is decided in `02`, Adjudication 4: a **new** domain section, not an append to the ports domain |
| `0020-C4` | **No** `reference` atom for the 411-line record; ADR-0029 is cited as the corpus's precedent that `source_paths` alone discharges a long record | none — a proposed non-action | `aligns` | **No op.** Merged with `0021-C8` into one adjudication (`02`, Adjudication 5), which supplies a criterion rather than resting on the precedent |
| `0020-C5` | Two citations repaired before staging: the two-constructor sentence is at `projection.rs:152-154`, not `:47-61`; AC-013's row is at `RUNBOOK.md:525`, not `:524`. Framed as **repairs** — the admitted-implementation set is unchanged | none load-bearing for `.kb/` | `aligns` | **No op.** Both verified correct as repaired. The only `.kb/` consequence is that Op 1's atom cites `:152-154`; the stale ranges live on in signed-off `.bklg/` artefacts and go to `unresolved` |
| `0020-C6` | `kb-open-question-projection-id-unvalidated-001` should gain `kb-decision-0020` in its `related:` frontmatter, per *"this corpus's reciprocal-backlink convention"* | none — mechanical hygiene | `aligns` | **No op.** The convention cited does not exist in this corpus; the convention that does exist is *outbound-only*. `02`, Adjudication 7 |

### `.kb/_intake/0021-payload-evolution-and-codec-tag.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `0021-C1` | The codec tag lives in **`Event::metadata`**, inside a **versioned framing region** the typed layer owns; everything after it is the application's, carried through unmodified and never parsed by `happenstance`. The framing region **must** be readable without knowing the payload codec, **must** add no dependency (so it survives `--no-default-features` and `wasm32`), and **must** be distinguishable from metadata that is wholly the application's. Exact bytes are M3's | **must**, three times, on the framing region's shape | `extends` | **Op 2** — `kb-decision-0021`, Decision 1 |
| `0021-C2` | **`EventType` carries no version suffix.** Matching is exact equality (`query.rs:113`), so `"CourseDefined.v2"` is invisible to every query already written and the first upcast makes existing decision models read an empty log with their append conditions matching nothing. An event type is a **stable identity**; the payload is what evolves; a genuinely new fact takes a new type name | stated as decided fact, decision-grade | `extends` | **Op 2**, Decision 2 — same atom, per the record's *"one decision atom, three answers"* framing |
| `0021-C3` | **No read-path hook is needed**, and the strategy that earns the conclusion is **decode-time tolerance**: `DomainEvent::decode(codec, event_type, data)` already receives the type and the raw bytes. `EventStore` is `[FROZEN]` with four required methods and no hook. The falsifier is an upcast needing information from outside the event being decoded; if reached, the route is a defect entry under AC-012, never a line edit | `[FROZEN]`, plus a `MUST NOT` inherited from VT-3 | `extends` | **Op 2**, Decision 3 |
| `0021-C4` | An event whose metadata carries **no** framing region decodes with the codec in hand — `Json` under `commit`, `C` under `commit_with::<C>` — and **not** `CodecError::UnknownTag`. Refusing would make every pre-typed-layer event unreadable with no legal repair: no adapter may rewrite stored events (VT-3), and no backfill hook exists. `UnknownTag` is left meaning exactly one thing | a compatibility rule with a `MUST NOT` behind it | `extends` | **Op 2**, the implied rule — **not** a fourth atom. `00`, *the dedup not offered* |
| `0021-C5` | One row on `decision-map.md`; an entry on `domain-map.md` **beside ADR-0020's** | none — procedural | `aligns` | **No op of its own.** Merged with `0020-C2` and `0020-C3`; the phrase *"beside ADR-0020's"* is what confirms both entries belong in one section |
| `0021-C6` | `kb-open-question-human-readable-encoding-limits-001` is `related` **only**, marking a seam (wire-format encoding on a memory-limited peer, phase 9's). Not resolved, edited or deleted | none — explicit non-action | `aligns` | **No op.** Cited outbound by Op 2. The extract's `defer_open_question` label is declined: in this plan's vocabulary that disposition **creates** an atom, and the intake asks for the opposite. `02`, Adjudication 7 |
| `0021-C7` | The seam against ADR-0016: **ADR-0016 owns how the metadata bytes cross a peer boundary; ADR-0021 owns what those bytes mean to the typed layer.** 0016 moves them and never reads them; 0021 reads them and never moves them. A replicated event's framing region travels as opaque bytes and `happenstance-sync` never learns it exists. **What does not transfer is ADR-0016's `reversibility: high`**, which rests on the wire format being private and unpublished; ADR-0021 is `low` | a scope boundary, no new modal | `extends` | **Op 2**, `## The seam against ADR-0016`, plus `related: kb-decision-0016`. **Not** a supersession and not a partial one — `02`, Adjudication 1 |
| `0021-C8` | **No** `reference` atom for the 454-line record; same ADR-0029 precedent, same deferral to the wave | none — explicit non-claim | `aligns` | **No op.** Merged with `0020-C4` into one adjudication |

## Conflicts

**Zero — and the number is only worth printing beside what was checked to reach it.** Four
candidates read as conflicts on a first pass. Each was verified against the accepted atom's own
body, not its title.

| # | Candidate | Why it is not a conflict |
| --- | --- | --- |
| 1 | ADR-0021 sites a codec tag in `Event::metadata`, and **VT-3 `[FROZEN]`** says the contract layer, a store adapter and a peer MUST NOT parse `data` or `metadata` | VT-3's first half forbids *parsing*, and nothing below the port parses the framing region — decoding is strictly above it (ADR-0007), and `QueryItem::matches` reads `event_type` and `tags` only (`query.rs:112-115`, verified). VT-3's **second** half is the one that decides: *anything a store, a peer, a conformance rule or a query must see MUST be carried in `EventType` or `Tags`* — and nothing below the port needs to see the tag. Had the answer been yes, VT-3 would have **required** `Tags` and the whole design would already be violated. VT-3's own `Rejects:` line — an ingest path writing origin identity into `metadata` (`:643-646`, verified) — is the mirror case, not a counter-example: replication identity **is** something a peer must see |
| 2 | ADR-0021 vs **`kb-decision-0016`**, which already legislates `Event::metadata` — base64 in human-readable formats, raw bytes otherwise | Same field, different question, and the record states the split in terms (`0021-C7`). ADR-0016 decides how the bytes are *encoded for transport*; ADR-0021 decides what a prefix of them *means to the typed layer*. Neither claim constrains the other, and a framing region rides through base64 as opaque bytes exactly as an application's own metadata does. `related`, 55, two atoms |
| 3 | ADR-0021 puts a `happenstance`-owned structure inside a field **`kb-decision-0003`** calls opaque, in a crate ADR-0003 forbids `serde` in | ADR-0003 constrains **`happenstance-core`**, and the framing region is written and read by `happenstance`, above the port. The record goes further than avoiding the conflict — it rejects a `serde`-encoded framing region **by name**, because that would be unreadable under `--no-default-features` and a `postcard`-only build could not read a `json`-only build's tag. `depends_on`, and the dependency runs the way ADR-0006 says it should |
| 4 | ADR-0020 derives a `Query` for decision models, and **`kb-decision-0007`** decided that a projection nominates events with `Query`, *"the same type a decision model uses, so there is no second filtering vocabulary"* | ADR-0020 derives that same `Query`; it does not mint a second one. The vocabulary is untouched — what changes is who writes it down, and the answer becomes *nobody, it is derived*. Strictly, ADR-0020 makes ADR-0007's claim more true rather than less. `related` |

Two more near-misses worth recording because a later wave will meet them again:

| # | Near-miss | Why it is not a conflict |
| --- | --- | --- |
| 5 | ADR-0020 forbids a **provided method** on `DecisionModel`, while **`kb-decision-0008`** legislates what a provided method owes | Two different traits in two different crates, and two different senses of "derivation". ADR-0008 binds provided bodies on `EventStore`/`ProjectionStore` under `trait_variant`'s `Send`-flavour cloning; ADR-0020 rejects an *overridable* body on an application-facing trait in `happenstance`. Neither reaches the other. `00` scores it 40 lexical / 25 subject and refuses even the `related` edge |
| 6 | ADR-0021's title carries three answers, and **`kb-playbook-one-decision-per-adr-title-001`** warns that an "and" in a title is usually two decisions of different strength bundled together | The playbook's test is whether the halves have different *strength* — whether one is likely to be reversed while the other stands. Here all three answers share one falsifier structure, one reversibility (`low`), and one cause: the record says the design gate handed all three over together *because the public surface is invariant under all three*. The bar is cleared on its own terms. `02`, Adjudication 6 |

## Counts

| | |
| --- | --- |
| claims classified | **14** (6 + 8), labelled across **16 rows** |
| `aligns` | **10** |
| `extends` | **6** |
| `conflicts` | **0** (four candidates checked and cleared, plus two near-misses) |
| `requires-new-decision` | **0** |
| accepted decision atoms **edited** | **0** |
| accepted decision atoms superseded (frontmatter flip) | **0** |
| decision atoms authored | **2** — transcribed from `references/adr/0020-…` and `0021-…`, not newly decided |
| `open_question` atoms created | **0** |
| `open_question` atoms resolved or amended | **0** |
| `reference` atoms created | **0** — declined twice, on one criterion |
| existing atoms otherwise amended | **0** |
| map atoms inheriting work | **2** (`decision-map`, `domain-map`), via `mapsImpact` rather than ops. `open-questions-index` inherits nothing |
| atoms out | **2 new**, across 2 operations |
| rows producing no operation, deliberately | **10** (`0020-C1a`, `C1b`, `C2`, `C3`, `C4`, `C5`, `C6`; `0021-C5`, `C6`, `C8` — folded into an atom, declined, or map-bound) |
| cross-file clusters collapsed | **3** of 4 (the map rows, the reference question, the untouched open questions); **CL-1, the two ADRs, refused at 38** |
