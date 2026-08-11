# Wave `2026-08-10-intake-2` — claims and classification

Every claim the extract pass raised, labelled against the accepted decision corpus, with the
destination it was routed to. **One hundred and eleven claims in, thirty-six atoms out** across
thirty-eight operations.

## What "against the accepted decision corpus" means this wave

`.kb/decisions/` still holds **no atoms** (`00-corpus-match.md`), so for the second wave running
there is nothing in the corpus to contradict. What is different is that the *intake* is a
decision corpus: seventeen ADRs, human-signed, carrying two supersessions, one partial
supersession chain and one amendment among themselves. The labels are used as follows, and the
usage is stated so a later wave reading this table is not misled.

| Label | Meaning here |
| --- | --- |
| `aligns` | Restates a rule `.kb/` already carries — in a layer README, or in an atom the previous wave landed. Nothing new is committed; the atom grounds the rule and cites it rather than introducing it. |
| `extends` | Net-new knowledge with no owner in `.kb/` and nothing to contradict. The wave's default, and the right label for an ADR that *supersedes another ADR in the same intake batch* — the supersession is already resolved by the documents themselves and arrives as a fact to record, not as a conflict to adjudicate. |
| `conflicts` | Contradicts something already accepted, or two incoming sources contradict each other and the wave declines to pick a winner. **Four occurrences**, all intra-wave, all routed per authority rule 4. |
| `requires-new-decision` | Cannot be discharged by recording it: an ADR, or a human's sign-off, is needed. Routed to `open-questions/`. |

**The `docs/adr/` corpus is now mirrored, and that is the change from last wave.** The previous
wave read `docs/adr/` for context and treated it as non-authoritative for placement, because the
authority rules govern `.kb/` atoms and importing the ADR corpus was "a separate wave with a
human in it". This is that wave, and the human is upstream of it: every one of these seventeen
documents was already accepted and signed off in `docs/adr/` — several with sign-off events named
in their own headers (ADR-0012's two lines, 2026-08-08; ADR-0015's CF-40 placement). **This wave
authors no decision.** It transcribes seventeen that exist, and the ADR-authorship rule is
honoured exactly as last wave honoured it: where a claim needs a *new* decision, it becomes an
open question.

## The claim table

### `0001-async-port-flavours.md` — ADR-0001

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `claim-1` | One trait definition per port, `Send` flavour derived by `trait_variant`; `read` returns its stream at the top level; bind the weaker flavour; no `#[async_trait]`, ever | "must" ×1, "cannot" ×5 | `extends` | Op 5 — `kb-decision-0001` |
| `claim-2` | The lift evidence: `LocalMemoryEventStore` passes all 27 rules natively and on `wasm32`; the direct impl sits beside the blanket impl with no `error[E0119]` | none — measurement | `extends` | **Op 1** (CL-A) + Op 26 (merge into the ES-7/VT-9 question) |

### `0002-crate-naming.md` — ADR-0002

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | Publish under prefixed names; pursue the bare `eventum` name in parallel; add a facade later if it frees. **Fully superseded by ADR-0005.** | "publish now", "do not block" | `extends` | Op 6 — `kb-decision-0002`, `status: superseded` |

### `0003-opaque-payloads.md` — ADR-0003

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | `Event::data` is `bytes::Bytes`; the contract crate carries no `serde` by default and never parses a payload; the `serde` feature covers envelope types only | declarative prohibition | `extends` | Op 7 — `kb-decision-0003` |
| `c2` | "accepted — **provisional**", with a named lift condition at phase 13 | governance-shaped | **`conflicts`** | **Op 27** (CL-D) — the schema has no such status |
| `c3` | Encoding belongs to the layer above | scope language | — | folded into Op 7's body; `related` to Ops 10, 11 |

### `0004-edition-and-msrv.md` — ADR-0004

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C0` | Edition 2024, `rust-version = "1.85"`, toolchain pinned 1.97.1. Amended, not superseded, by ADR-0029 | ADR-grade | `extends` | Op 8 — `kb-decision-0004` |
| `C1`–`C2` | What the compiler is needed for; edition 2024's RPIT lifetime capture is why `read` can borrow `&self` without `+ '_` | descriptive | `extends` | Op 8, Context |
| `C3` | Let-chains avoided (1.88 > the 1.85 floor) — **rescinded in effect** by ADR-0029 | constraint, now reversed | `extends` | Op 8 verbatim; the reversal is Op 21's, `related` both ways |
| `C4`–`C7` | Consequences and the semver policy: MSRV verified only in CI; *"if that job fails, raise the MSRV rather than working around it"*; an MSRV bump is a minor bump | one imperative | `extends` | Op 8, Consequences/Policy |
| — | "accepted — **provisional**", lifting at first publish | governance-shaped | **`conflicts`** | Op 27 (CL-D) |

### `0005-rename-to-happenstance.md` — ADR-0005

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | Rename `eventum` → `happenstance`. **Stands.** | executed commitment | `extends` | Op 9 — `kb-decision-0005` |
| `c2` | Allocate the bare name to the contract crate. **Reversed by ADR-0006.** | executed commitment | `extends` | Op 9, with the reversal stated in the summary's first clause |
| `c3` | Supersede, do not rewrite: a superseded ADR's body stays verbatim, because rewriting a rename into it inverts a factual claim into a falsehood | normative, meta | **`aligns`** | **Op 22** (CL-B) — `.kb/decisions/README.md` states the immutability rule; this generalises it |
| `c4` | Consequences: one fewer crate, facade has no name left, per-crate README owed before publish | "must" ×1, "should" ×1 | `extends` | Op 9 |
| `c5` | Follow-up: reserve the name before someone else registers it | conditional | `extends` | Op 9's body; **no question atom** (CL-I) — carried in `unresolved` |

### `0006-bare-name-to-the-typed-layer.md` — ADR-0006

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | Invert ADR-0005: `happenstance-core` is the contract, `happenstance` is the typed layer. *Everything depends on `happenstance-core`; it depends on nothing in this workspace.* | bolded commitment | `extends` | Op 10 — `kb-decision-0006` |
| `C2` | Who-imports-what, and three ecosystem precedents (`serde_core`, `futures-core`, `tracing-core`) | none | `extends` | Op 10, Context |
| `C3` | The serde boundary moves with the contract, not the name; the contract crate's docs **must** say so and `happenstance` **must** point at it | "must" ×2 | `extends` | Op 10 |
| `C4` | The projection runner moves into the contract crate — **corrected by ADR-0007** | none | `extends` | Op 10, marked as the reversed half |
| `C5`–`C6` | Consequences; three alternatives rejected | none | `extends` | Op 10 |
| `C7` | Why 0001/0003/0004 are provisional and this one is not: *"a naming decision is settled by being made, not by being tested"* | classification | `extends` | Op 10; `related` to Op 27 |
| `C8` | **Rewrite the referent, never the reasoning** — a rename that preserves meaning may be rewritten in place; reasoning inside a standing decision never is | normative, meta | **`aligns`** | **Op 22** (CL-B) |
| `C9` | Provenance footnote: three "phase 3" citations went stale when RUNBOOK.md was rewritten, and are recorded rather than repointed | none | `extends` | Op 10; `related` to `kb-playbook-anchoring-citations-001` |

### `0007-projection-runner-decodes.md` — ADR-0007

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | Split the runner at the decode boundary: the checkpoint pump in `happenstance-core`, the `Projection` trait and the decoding runner in `happenstance`. Partly supersedes ADR-0006 | commitment | `extends` | Op 11 — `kb-decision-0007` |
| `c2`–`c4` | A projection nominates events with `Query` (no second filtering vocabulary); `Projection::Store` is an associated type so a two-store projection is unrepresentable; checkpoints stay per `(store, ProjectionId)` and callers **must** tolerate the skew | "must" ×1 | `extends` | Op 11 |
| `c5` | Falsifier: if the core pump has no independent caller when phase 7 exits, collapse it upward and supersede | conditional | `extends` | Op 11 |
| `c6` | Third time an "and" in an ADR title concealed a second, weaker decision | observational | `extends` | **Op 23** (CL-C) — `playbook` |
| `c7` | `ProjectionStore::Batch` carries no trait bounds, so generic code can begin and commit a batch but cannot write to it — there is no `apply` | deferral | `requires-new-decision` | **Op 28** |
| `c8` | ADR-0006's body kept verbatim; only the runner allocation moves | bookkeeping | `aligns` | Op 11 + Op 22 |

### `0008-one-derivation-for-both-ports.md` — ADR-0008

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1`–`C2` | One derivation scheme for both ports. Three rules: hand-desugar a provided method to `-> impl Future`; `where Self: Sync` at the point of use, never in the attribute; one body compiles against both flavours | 3 hard rules | `extends` | Op 12 — `kb-decision-0008` |
| `C3` | The evidence, compiled: the three-copy expansion, `LocalMemoryEventStore` with no `E0119`, `RefCell` is `Send` and `Rc` is the hazard, `#[tokio::test]` drives a `!Send` store, five failed spellings for a per-flavour associated type | none — measurement | `extends` | **Op 1** (CL-A) |
| `C4`–`C5` | Where the ports diverge: `ProjectionStore`'s GAT (`E0311`), `Self: Sync` makes a provided method uncallable on a `RefCell` store, provided methods are second-class on the bare flavour | "should" ×2, "may not" ×1 | `extends` | Op 12 |
| `C6` | ES-6 stays deferred; the rule, when written, **must** assert on the future's `Output`, not the future | "must" ×1 | `requires-new-decision` | **Op 25** — merge into `kb-open-question-es-6-unwritable-rule-001` |
| `C7`–`C8` | Three alternatives rejected; four amendments owed to the specification (ES-2, ES-3, ES-5, PS-36), none changing a normative MUST | none | `extends` | Op 12; `related` to `kb-playbook-repair-frozen-clause-001` |

### `0009-error-send-sync.md` — ADR-0009

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | `Error` keeps `core::error::Error + 'static` on both ports and both flavours; the strength moves to a downstream `ThreadSafeEventStore` marker trait. Four compiled findings; four alternatives rejected | "must" ×3 | `extends` | Op 13 — `kb-decision-0009`; the four findings to **Op 1** |
| `C2` | ES-6's premise is half wrong and is corrected; ES-6 becomes writable, and PS-35 with it | "must" ×2 | `requires-new-decision` | **Op 25** |
| `C3` | Extends ADR-0008; settles ES-6 and PS-35's second half | metadata | `extends` | Op 13's `depends_on` |

### `0010-the-suite-must-prove-itself.md` — ADR-0010

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C0`–`C1` | *A rule may not be added to the suite until a store exists that fails it.* Every rule owes a mutant with stated provenance; three meta-tests enforce it; no pass rate is ever quoted over the mutant set | "may not", "is ever" | `extends` | Op 14 — `kb-decision-0010` |
| `C2` | The fixture becomes a factory that can be asked more than once for a handle onto the same store; a declined capability **must** be reported, never silent | "must" ×1 | `extends` | Op 14 |
| `C3` | Runtime wrapper stays a parameter; two corrections — `#[tokio::test]` already drives a `!Send` store, and `RefCell` is `Send` so CF-28 must name `Rc` | "must"/"should" | `extends` | Op 14 for the ratification; the two corrections to **Op 1** (CL-A) |
| `C4` | §4 amended five times inside one document: zero of three rules retired, `append_is_atomic` un-retired, `MID_BATCH_FAULT` minted. *"A `Retires:` line is a claim to re-examine, not a filing."* | "must not" ×1 | `extends` | Op 14; `related` to `kb-playbook-repair-frozen-clause-001` |
| `C5` | Consequences and four rejected alternatives, including `cargo-mutants` over the reference store | none | `extends` | Op 14 |

### `0011-read-laziness-and-isolation.md` — ADR-0011

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `core-signature-decision` | ES-13 stands: `read` keeps `query: &Query`; an escaping stream owns its query as a *parameter* | commitment | `extends` | Op 15 — `kb-decision-0011` |
| `sampling-instant-not-laziness` | The promise is "one state sampled no later than the first poll", not "lazy"; three normative consequences | MUST/MUST NOT ×3 | `extends` | Op 15 |
| `position-ceiling-mechanism` | An adapter issuing more than one statement per `read` **MUST** capture a ceiling H and bound every later statement by it | MUST ×2 | `extends` | Op 15 |
| `es-11-es-12-stay-provisional` | Both stay `[PROVISIONAL]`; ES-12's rule is shown *not* to be blocked | marker retention | `extends` | Op 15; `related` Op 27 |
| `no-unpin-bound` | No `+ Unpin`; new `[PROVISIONAL]` clause with a phase-12 deadline. **Corrects ADR-0001's `dynosaur` consequence (`error[E0277]`)** | MUST NOT | **`conflicts`** | Op 15 states the correction; the compiled fact to **Op 1**. ADR-0001's atom body is *not* edited |
| `readoptions-to-and-limit-fix` | `ReadOptions` gains `to`; `limit` becomes `Option<usize>` so `limit(0)` yields nothing | normative | `extends` | Op 15 |
| `readoptions-declines-after` | Exactly one lower bound, `from`, inclusive; an exclusive `after` **MUST NOT** be added. Minted `[FROZEN]` with no falsifier by design | MUST/MUST NOT | `extends` | Op 15 |
| `no-eventstoreext-or-prelude` | Neither ships at 0.1; declined as prose because an absence is not rule-checkable | declination | `extends` | Op 15 |
| `vt-31-rule-ownership-declined` | `query_union_is_item_concatenation` is owed and owned by nobody. *"The one disposition in this ADR that a human should confirm rather than inherit."* | none | `requires-new-decision` | **Op 29** |
| `torn-read-append-condition-mechanism` | Why a torn read is not caught by the append condition: the caller derives the boundary *from* the read, so a torn read becomes a silently **accepted** append | none — mechanism | `extends` | **Op 4** — `concept` |
| `compiled-experiments-e10-e11-e12` | E10 escaping-stream matrix, E11 hand-written `dyn` erasure, E12 `Unpin` forbids a generator stream | none | `extends` | E11 to **Op 1**; E10/E12 named in Op 15 with a pointer |
| `cross-adr-linking-and-corrections` | Extends 0008 and 0010; routes VT-26 to 0015, head/count to 0013, VT-30 to 0012 | metadata | `extends` | Op 15's `depends_on`/`related` |
| — | Two clauses stay `[PROVISIONAL]`, one new clause minted `[PROVISIONAL]` | governance-shaped | **`conflicts`** | Op 27 (CL-D) |

### `0012-append-shape-and-preconditions.md` — ADR-0012

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C0` | The ADR as a whole; ES-17 stays `[PROVISIONAL]`; two lines signed off by a human 2026-08-08 | dozens of MUST | `extends` | Op 16 — `kb-decision-0012` |
| `C1` | `append` keeps `events: &[Event]`; ES-17's lift is declined this phase, with five stated preconditions phase 8 must produce | MUST-shaped | `extends` | Op 16 |
| `C2`–`C5` | Empty batch, self-conflict, cancellation (a caller **MUST NOT** treat a dropped future as evidence), reissue idempotency, returned position in slice order | MUST/MUST NOT/MAY | `extends` | Op 16 |
| `C6` | `conflicting_position` is a hint; `Display` renders a fixed message; callers **MUST NOT** parse it | MUST NOT ×3 | `extends` | Op 16 |
| `C7` | `AppendCondition` becomes a non-empty sequence of `Guard`s with a **private** `guards` field — the ADR's own flagged sign-off item, signed 2026-08-08 | dense MUST | `extends` | Op 16 |
| `C8`–`C9` | ES-25–ES-29 generalise by conjunction across guards; **CF-39** minted, `[PROVISIONAL]` | MUST-heavy | `extends` | Op 16 |
| `C10` | Thirteen amendments owed to the specification, plus a RUNBOOK correction list | to-do ledger | `extends` | Op 16; `related` to the repair playbook |
| — | CF-39 establishes the fixture-capability precedent ADR-0015's CF-40 then contests | — | **`conflicts`** | **Op 35** (CL-H) |

### `0013-position-assignment-and-visibility.md` — ADR-0013

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `0013-1` | ES-10 lifts to `[FROZEN]`; the global visibility invariant is stated once; three caveats, including that the invariant stays **global**, not per-boundary, and that `head()` reports the frontier | MUST/MUST NOT | `extends` | Op 17 — `kb-decision-0013` |
| `0013-2` | VT-13 discharge: `checked_add` not `saturating_add`; `checkpoint.next()` is the sound resume idiom over gaps | two frozen MUSTs | `extends` | Op 17 |
| `0013-3` | Flag: is the `checked_add` fix already in the tree? | — | — | **Resolved by reading the tree** (`event.rs:278`, 2026-08-10). No atom; recorded in Op 17 and in `00` |
| `0013-4` | The phase-2 Postgres experiment: arm C is the only arm that passes the inversion detector unserialised; ratios 0.987–1.026; staleness 0.688 ms unloaded, 4010.719 ms behind an unrelated 5 s write | none — measurement | `extends` | **Op 2** — `reference` |
| `0013-5` | Hand-polling two cold futures out of order to model a late-publishing transaction, with `POLL_BUDGET` considered and rejected | none | `extends` | **Op 24** — `playbook` |
| `0013-6` | Global vs per-boundary invariant, closed by decision and explicitly reversible | none | `requires-new-decision` | **Op 30** |
| `0013-7` | Whether a real Postgres adapter can express arm C cleanly through `sqlx`, and at what structural cost | none | `requires-new-decision` | **Op 31** |
| `0013-8` | The sole rule checking the ES-10 lift is unbounded against an adapter needing 3+ polls | none | `requires-new-decision` | **Op 32** |
| `0013-9` | ES-38's rule cannot be written today; `read_from_a_gap_position` is named by two ADRs and scheduled by neither | none | `requires-new-decision` | **Op 33** (CL-G, with ADR-0011) |

### `0014-event-identity-and-recorded-time.md` — ADR-0014

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `0014-core-decision` | `StoreId`, `EventId`, `RecordedAt` in `happenstance-core`; the store mints identity and time and the caller supplies neither; the incarnation rule; `RecordedAt` is signed `i64` ms with no `now()` in core; ES-41 `contains_event_id` ships `[PROVISIONAL]`; `SequencedEvent::new` superseded not widened; `Event::into_parts` returns `EventParts` | dense MUST/MAY | `extends` | Op 18 — `kb-decision-0014` |
| `0014-provisional-exposure` | Four named provisional parts with falsifiers and owning phases; nothing has been run on a store that survives its own process | descriptive | `extends` | Op 18 + **Op 26** (VT-9's owner) + Op 27 |
| `0014-alternatives-and-amendments` | Eight alternatives rejected; nine amendments owed to the specification and four RUNBOOK cells | none | `extends` | Op 18 |

### `0015-validated-identifiers-and-store-limits.md` — ADR-0015

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1`–`C2` | One `const fn` byte walk validates both `new` and `from_static`; `Cow<'static, str>` backing; hand-written `Eq`/`Hash`/`Ord`; `Borrow<str>` and `FromStr`; `Event` **MUST NOT** implement `Hash`. VT-32 and VT-33 minted `[FROZEN]` | dense MUST | `extends` | Op 19 — `kb-decision-0015` |
| `C3`–`C4` | `values_of` ships and `value_of` is declined (VT-17's MUST NOT); equality is byte equality, no normalisation, with two worked failures | MUST | `extends` | Op 19 |
| `C5` | `ProjectionId::new` stays infallible and unvalidated; phase 4 declines to validate and declines to call it "deliberately opaque" | explicit non-decision | `requires-new-decision` | **Op 34** |
| `C6`–`C9` | `InvalidQuery` composition and the seam phase 7 owns; validity bound vs capacity limit (**a capacity limit MUST NOT be enforced in a constructor or in `Deserialize`**); four public floor constants; `ExceedsStoreLimit`; **CF-40** minted | the ADR's load-bearing MUST NOT | `extends` | Op 19 |
| `C10` | CF-40's ownership: the intro says "settled at sign-off, it lands here", the Consequences say the question "is not settled" and that decision 8 "declines to choose" | none | **`conflicts`** | **Op 35** (CL-H) — an internal contradiction in an accepted ADR; not resolved by this wave |
| `C11` | This ADR is a worked instance of the repair-versus-amendment test | pattern note | **`aligns`** | Op 19 `related` → `kb-playbook-repair-frozen-clause-001` |
| — | VT-14 and VT-21–VT-24 stay `[PROVISIONAL]`, two with falsifiers no scheduled phase can reach | governance-shaped | **`conflicts`** | Op 27 (CL-D) |

### `0016-the-wire-format.md` — ADR-0016

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | The wire format is happenstance's own, private, with no compatibility obligation to the DCB reference. WF-2–WF-12 discharged; two `[FROZEN]` clauses amended by new ADR rather than by edit; six measured numbers corrected | dozens of MUST | `extends` | Op 20 — `kb-decision-0016` |
| `C2` | Rests on ADR-0003, extends ADR-0012 §9, informed by ADR-0014; does not settle the replication protocol | metadata | `extends` | Op 20's `depends_on` |
| `C3` | The measurements: postcard round-trip failures, corrected byte deltas, the 340 KiB payload table (raw 1,243,464 B / base64 464,218 B / hex 696,322 B / postcard 348,163 B), base64's zero-new-crate cost, and W7's finding that the DCB reference publishes no format | none — measurement | `extends` | **Op 3** — `reference` |
| `C4` | serde's derived `Deserialize` reads every field into a local `Option` before constructing, so a version-gated envelope is *structurally required* to hand-write its impl | structural MUST | `extends` | Op 20, as WF-8's rationale |
| `C5` | A `compile_fail` doctest passes whenever the snippet fails to compile for *any* reason — 3 of 4 deliberately-broken spellings reported green. Replaced by an inherent-associated-const detection trick inside `const _: () = assert!(…)` | instrument replacement | `extends` | Op 20, as WF-12's rationale; measurement to Op 3 |
| `C6` | WF-1's interoperability half stays `[DEFERRED]`: the DCB reference publishes no wire format at all | none | `requires-new-decision` | **Op 36** |
| `C7` | serde offers no streaming entry point for a human-readable string, so this falsifies human-readable payload encoding *as a category* | none | `requires-new-decision` | **Op 37** |
| `C8` | What the sync message set is, and therefore what `FORMAT_VERSION = 1` names, is undesigned | none | `requires-new-decision` | **Op 38** |
| `C9` | Three phase-5 implementation follow-ups with owners and refuters | none | `extends` | Op 20's body; **not** open questions — the open-questions README bars a task |

### `0029-msrv-raised-to-1-97-1.md` — ADR-0029

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | The MSRV is 1.97.1 and `rusqlite` stays at 0.40. Amends ADR-0004. `rust-version` and `rust-toolchain.toml` stay two different facts. Let-chains are available; CLAUDE.md's constraint 5 is rewritten. The `msrv` CI job is kept though vacuous. **Phase 12 must revisit the floor at first publish.** Three alternatives rejected | "must" ×1, declaratives | `extends` | Op 21 — `kb-decision-0029` |
| `c2` | The evidence: `libsqlite3-sys 0.38.1`'s build script uses `cfg_select!`; five database crates declare no `rust-version`, so `cargo hack --rust-version` and `resolver = "3"` cannot see the break; the break is outside our own code and is recoverable | none | `extends` | Op 21's Context — **not** a separate reference atom: it grounds exactly one decision and is 120 words |

## The four conflicts, and what was done with each

| # | Conflict | Resolution |
| --- | --- | --- |
| 1 | **CL-D** — nine ADRs use `status` values `KbFrontmatter` does not have ("accepted — provisional", "partly superseded by") | A working convention is applied for this wave and *stated* (`02`, "Standing choices"); the residual gap is Op 27, an `open_question`. Not silently normalised. |
| 2 | **CL-H** — ADR-0015 contradicts itself about whether it or ADR-0012 owns the fixture's numeric-limit capability, and says in terms that neither ADR may resolve it unilaterally | Op 35, an `open_question`. Both decision atoms record the contradiction and link to it. Authority rule 4: no winner is picked. |
| 3 | ADR-0011 corrects a stated consequence of ADR-0001 (`dynosaur` does not erase the port — `error[E0277]`) | Both atoms are created this wave. **ADR-0001's body is not edited**, per authority rule 1 and per the corpus's own rewrite-the-referent rule (Op 22): a correction to reasoning is the correcting document's, never a patch to the corrected one. The compiled fact lands in Op 1, which both atoms cite. |
| 4 | ADR-0010 and ADR-0008 both correct CF-28's wording, one day apart, in the same words | Not a disagreement — a duplicate. Merged into Op 1 (CL-A), which owns the finding; both decision atoms cite it. |

## Counts

| | |
| --- | --- |
| claims classified | **111** |
| `aligns` | 5 |
| `extends` | 90 |
| `conflicts` | 4 |
| `requires-new-decision` | 12 |
| accepted decision atoms **edited** | **0** |
| accepted decision atoms superseded (frontmatter flip) | **0** — the one fully-superseded ADR arrives already superseded and is authored that way (`02`) |
| decision atoms authored | **17** — transcribed from `docs/adr/`, none newly decided |
| reference atoms | 3 |
| concept atoms | 1 |
| governance atoms | 1 |
| playbook atoms | 2 |
| open_question atoms created | 12 |
| existing atoms amended | 2 (both `open_question`) |
| atoms out | **36**, across 38 operations |
