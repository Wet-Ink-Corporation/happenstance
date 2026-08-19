---
item: HS-S0051
stage: discover
created: 2026-08-12T13:02:13.185Z
updated: 2026-08-12T13:02:13.185Z
template_sig: 86ce4036
rendered_sig: 178c7453
---

# Discover — A lazy read that is still one sample: ADR-0011's ceiling-and-page

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: implement `read` as ADR-0011's ceiling-and-page — still non-`async`, stream at the top level, ceiling captured no later than the first poll, every later statement bounded by it, no cursor held across an `await` | `_storymap.md`, **Slices**, `durable-object-read-path` row | The mechanism is prescribed by an accepted atom. This story's job is to fit it to a real API, not to invent a trade-off |
| AC-001 (the read body, no `todo!()`) and AC-007(a) (the non-snapshot cursor versus the laziness requirement) | `project.md`, **Acceptance criteria** | "Honoured" is listed first of the three acceptable outcomes, and that ordering is deliberate |
| `dependsOn: worker-binding-layer` — it supplies the real `SqlStorageCursor` and the synchronous `exec` the paging shape is built on | `_storymap.md`, **Slices**; `crates/happenstance-cloudflare/src/sql_storage.rs:1-12` | `exec` being synchronous is what makes a bounded re-`exec` per page cheap: there is no connection to acquire |
| ADR-0011 is **accepted** and states the mechanism operatively: an adapter issuing more than one statement per `read` must capture a position ceiling no later than the first poll and bound every later statement by it — stated to discharge isolation across three store shapes, one named literally as *chunked cursor* | `.kb/decisions/0011-read-laziness-and-isolation.md:19-21`, `:69-75` | That is this adapter's shape, named. The atom wins over any brief that reads it as still open |
| ARCH-AC-03: implement ceiling-and-page, **or** record in ADR-0023 the compiled reason the real `SqlStorageCursor` ruled it out. `check_cursor_still_valid` is deleted or kept as belt-and-braces, never as the primary mechanism | `_decomposition.md`, Architecture brief, **Acceptance Criteria**; `crates/happenstance-cloudflare/src/event_store.rs:300-306` | Detecting invalidation after the fact is not a fix; the current skeleton only detects |
| The crate's own docs frame the non-snapshot cursor as an **open** capability trade-off between laziness and buffering | `crates/happenstance-cloudflare/src/lib.rs:96-107` | Stale against an accepted atom. `_grounding.md` §3 says so explicitly and asks for it to be treated as the leading resolution, not a fresh question |
| `read` stays non-`async` and returns the stream at the top level; two tests in `memory.rs` pin that shape and neither may be deleted, with `spawns_from_generic` the one that rejects an `async fn read` refactor | `CLAUDE.md`, binding constraint 3; `crates/happenstance-cloudflare/src/event_store.rs:148-169` | A paging state machine is exactly the kind of change that tempts an `async fn`. It must not become one |
| `SqlRowStream`'s state machine stays hand-written, because a coroutine's `Send`-ness is *inferred* while this adapter's whole value is that its `Send`-ness is decided by its fields | `crates/happenstance-cloudflare/src/event_store.rs:198-233` | The reason survives the rewrite even though every line of the body changes |
| `ReadOptions::limit` truncates **matches**, not scanned rows; registered mutants already reject the two wrong readings | `crates/happenstance-testkit/src/lib.rs:143`; `mutation_coverage.rs:709-711` (`LimitBeforeFilterStore`), `:726-728` (`LimitPerItemStore`) | Paging must not spend the caller's limit on rows a query item excluded |
| `ReadOptions` gained an inclusive `to` for exactly this ceiling — the caller-side spelling of the same mechanism | `.kb/decisions/0011-read-laziness-and-isolation.md:79-81`; mutants `ToBoundIgnoredStore` and `ToIsExclusiveStore`, `mutation_coverage.rs:643-645`, `:661-663` | The adapter composes the two bounds; it does not treat them as alternatives |
| ES-9 is `[FROZEN]`, and what it actually states is that `from` names a **position, not an index** — a range predicate over assigned positions, yielding the next matching event above or below an unoccupied `p` | `spec/SPECIFICATION.md:2748-2760`, ledger row `:8591` | Load-bearing correction: ES-9 does not carry the laziness requirement the crate docs and briefs attribute to it. See Questions |
| ES-9's clause names the paginating adapter that anchors its window on the head at the first poll as the store it rejects, and that store is registered | `spec/SPECIFICATION.md` (ES-9, `Rejects:`); `crates/happenstance-testkit/tests/mutation_coverage.rs:769-771` (`NullHeadPagingStore`), with `PagedStreamStore` at `:339-341` as the honest version | The nearest registered neighbour of the shape this story builds, and the one it must not become |
| Consumers: `durable-object-host-and-fixture` mounts a store that must replay, and `adr-0023-and-atom-resolutions` records whichever resolution this story reached | `_storymap.md`, **Slices**, `depends_on` columns | Whatever is decided here has to be legible enough to be written down as a decision with alternatives that lost |

## Questions

**Does ceiling-and-page fit the real `SqlStorageCursor`?** Leading answer: yes,
and cheaply. `exec` is synchronous (`crates/happenstance-cloudflare/src/sql_storage.rs:1-12`),
so a bounded re-`exec` per page never holds a cursor across an `await` and the
documented instability of a cursor held across one is simply never engaged.
Confirmed or refuted at **spec** against the real API. A refutation is not a
licence to buffer silently: it is a compiled reason recorded in ADR-0023 plus
either a declined capability with a stated reason or a new decision atom and a
re-plan (`project.md`, AC-007).

**How is the ceiling captured — `max(position)`, `head()`, or composition with
`ReadOptions::to`?** Deferred to **spec**; the architecture brief names it
explicitly as ADR-0023's material rather than the brief's
(`_decomposition.md`, Architecture brief Notes §6). One constraint binds the
choice: `to` composes with the internal ceiling rather than competing with it,
because both are the same mechanism spelled at different layers
(`.kb/decisions/0011-read-laziness-and-isolation.md:79-81`).

**Which clause actually carries the laziness requirement?** Answered here rather
than inherited, because the mis-attribution is in the tree. The crate docs say
"ES-9 requires the stream to be lazy" (`crates/happenstance-cloudflare/src/lib.rs:100-107`)
and the briefs repeat it, but ES-9 as written is about `from` naming a position
rather than an index (`spec/SPECIFICATION.md:2748-2760`). The sampling promise —
"evaluated against one state sampled no later than the first poll", with laziness
**permitted and never required** — is ADR-0011's. Where a brief and a clause
disagree the clause wins, so the spec cites ADR-0011 for the mechanism and ES-9
only for the `from` predicate, and correcting the crate doc is in scope for this
story with the reason given in the same change.

**Is buffering the whole result set an acceptable fallback?** Answered: no, not
silently. It defeats streaming a large replay, which is the cost `lib.rs:100-107`
already names, and it is invisible to the suite (see the mutant). If it is chosen
it is chosen out loud, as a declined capability with a stated reason and a record
in ADR-0023.

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.**
Neither is this story's; they belong to `measured-store-limits`,
`adr-0023-and-atom-resolutions` (coordinated with HS-P0012) and
`every-rule-under-workerd` respectively.

## Decision

`read` is the one port method whose signature the whole two-flavour design is bent
around — non-`async`, stream at the top level — and it is also the one this
runtime disagrees with most sharply, because Cloudflare documents that a
`SqlStorageCursor` held across an `await` is not a stable snapshot. The crate
currently models the disagreement as an open trade-off between laziness and
buffering and only *detects* invalidation after the fact. That framing is stale:
ADR-0011 is accepted, it corrects the port's promise to "one state sampled no
later than the first poll", and it prescribes the mechanism for exactly this store
shape by name — capture a position ceiling at or before the first poll and bound
every later statement by it. This slice implements that. The spec will cover: the
`StreamState` machine as bounded pages rather than one live cursor; how the
ceiling is captured and how it composes with `ReadOptions::to`; how a page's
`limit` is spent on matches rather than scanned rows; the fate of
`check_cursor_still_valid`; the correction to the crate doc's ES-9 attribution;
and the exact statement that goes into ADR-0023 — either "ceiling-and-page,
implemented" or "ruled out, and here is the compiled reason", with the declined
capability and its reason if it is the latter. ES-9 is `[FROZEN]` and is honoured
rather than amended; if this runtime genuinely could not satisfy it, that is a new
decision atom and a re-plan, not a licence for this adapter to be excused.

## The wrong implementation

**The read that is a snapshot because it is not lazy.** Drain the entire result
set into a `Vec` at the first poll and hand rows out of it. Every isolation rule
passes — the sample is trivially stable. Every query-semantics and read-options
rule passes. The stream is still returned at the top level, so `spawns_from_generic`
and its sibling in `crates/happenstance-core/src/memory.rs` are untouched and
`CLAUDE.md`'s constraint 3 is satisfied. `SqlRowStream` stays `!Send` because a
`Vec<SqlRow>` changes nothing about its fields. And **nothing in the suite can see
it**, by design: no conformance rule observes *when* a store reads, and ADR-0011
deliberately makes laziness permitted rather than required
(`.kb/decisions/0011-read-laziness-and-isolation.md:19-21`). The cost lands only on
a replay large enough to exceed the isolate's memory — which is the one runtime in
this workspace where that ceiling is real, and the one this project is already
measuring in `wf-11-memory-ceiling-falsifier`.

**And the one the suite *can* catch, which is why it is the less dangerous of the
two.** A page-per-`exec` read that recomputes `max(position)` for each page rather
than capturing it once — the window re-anchored on the head at every step.
`NullHeadPagingStore` is that store, already registered
(`crates/happenstance-testkit/tests/mutation_coverage.rs:769-771`) and named by
ES-9's own clause text, with `PagedStreamStore` (`:339-341`) beside it as the
honest paging version. A ceiling-and-page read that forgets the ceiling is that
defect wearing this adapter's clothes.

**Where the first mutant's detector must live — and why it is not a new rule.**
Not in `mutation_coverage.rs`. A buffering store fails no registered rule, which is
precisely what makes it dangerous, and `CLAUDE.md`'s corollary forbids adding a
rule no adapter can fail: a rule for "it buffered" would be decorative, because the
port permits buffering. Its detector is therefore threefold and none of it is a
conformance rule: the memory measurement `wf-11-memory-ceiling-falsifier` takes on
this same runtime, which is where an accidental full materialisation shows up as a
number; the ADR-0023 record stating which of the two shapes was built and why, so a
later reader can tell a chosen buffer from an accidental one; and the crate doc,
rewritten from "the fix is either buffering the whole result set … or a rule that
says a read is a snapshot only until the first `await`" to whichever answer was
actually taken.

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

**Box 6.** No conformance rule is added here. The read path is exercised by the
existing enumeration, and the ceiling this story captures is compared against
positions the store actually assigned — never against a literal `[1, 2, 3]`,
because the specification permits gaps and a paging read that assumed density
would be the defect rather than the fix.

**Box 7.** ES-9 is `[FROZEN]` and this story touches it. It is **not changed**:
the story honours it through ADR-0011's already-accepted mechanism, and separately
corrects a *citation* — the crate doc's attribution of the laziness requirement to
ES-9 rather than to ADR-0011 — which moves no clause text and no marker. If the
real `SqlStorageCursor` API defeats ceiling-and-page, the outcome is a declined
capability with a stated reason plus a new decision atom and a re-plan, written
first; it is never an edit to a frozen clause and never a silent pass.

**Box 8.** No conformance rule here seems wrong. The one thing that does read as
wrong is in-tree prose rather than a rule — `lib.rs:96-107`'s framing of an
accepted decision as an open trade-off — and this story fixes it and gives the
reason (ADR-0011's acceptance, and `_grounding.md` §3's flag) in the same change.
