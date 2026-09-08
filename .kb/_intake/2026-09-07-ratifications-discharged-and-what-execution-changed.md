# The six ratifications are discharged, and executing three of them changed what was ratified

Record: **RATIFY-0.2.0-DISCHARGED**. Staged for `/redkiln:kb-ingest` rather than
written as atoms by hand, for the reason
[[ratifications-2026-09-06-pre-publication]] gives: an accepted decision atom is
immutable and is authored by that command.

**This file is the companion that file said it needed.** It records: *"This file
records decisions, not implementations. Several of the seventeen oblige code that
is not written yet; the queue is at the foot."* That queue is now empty, and
three of the six items came back from execution meaning something different from
what was ratified. Those three are the reason this document exists — the other
three landed as written and are recorded here only so the queue can be closed in
one place.

---

## The queue, and where each landed

| Brief | Commit | Landed as ratified? |
|---|---|---|
| `op-read-non-exhaustive` / A | `9000f35` | yes |
| `stringified-throw-visibility` / B | `9225c00` | yes, **plus a consequence the brief did not name** |
| `codec-foreign-tag-resolution` / A | `2d37fc4` | yes, **narrower than the brief implied** |
| `empty-decision-outcome` / 3 + `tags-scope-agreement` / A + B | `0ae10ab` | yes, landed as one change |
| `cf-18` / B3 | `3df4b6e` | **no — the costing changed the assertion** |

Each is verified against its own wrong implementation by *building* that
implementation, not by argument. Nine wrong implementations in total; each is
caught by exactly one test, and no test catches another's defect.

---

## 1. `cf-18` / B3 — the brief's own wording was decorative

**This is the finding worth keeping.** The brief asked for a test that *"fails if
any capability is declined without an accompanying declaration"*. That describes
a state that **cannot be constructed**: `Capability::declined` asserts a non-empty
reason and `Capability::SUPPORTED` *is* the absence of one, so declining and
carrying a reason are the same act. A rule written to the brief's words would
pass on every fixture that will ever exist.

That is the decorative-rule failure `CLAUDE.md` warns about, arriving from the
direction nobody watches: not a rule too weak to catch a defect, but a rule whose
subject the type system had already made unreachable — written by a lane that had
measured the *reporting* gap correctly and then described the *mechanism* by
analogy to it.

**What was implemented instead** is declension by **inheritance**. Five
capabilities default to a declension whose words are the testkit's, so a fixture
that says nothing prints this crate's prose in an adapter's CI log as though it
were that adapter's account of its own store. CF-18's `Rejects:` paragraph had
asked for exactly that enforcement — *"requiring a non-empty reason string
alongside each `false` puts the trade in the log where a reviewer and a user of
the adapter can both see it"* — and nothing had ever stood there.

Three consequences worth carrying into the KB:

- **The clause is untouched.** CF-18's MUST is unchanged and stays `[FROZEN]`;
  ES-35's `[PROVISIONAL]` marker is undisturbed. The brief's fallback — narrowing
  CF-18 to Option A — was **not taken** and remains available.
- **The `wasm32` blind spot was closed rather than accepted.** The costing
  predicted having to document it, because the check is emitted by the suite
  macro (CF-23 makes the emitter the caller's, so an emitter-side check would be
  droppable in silence) and a plain `#[test]` is neither run nor listed by
  `wasm-bindgen-test-runner`. A `cfg_attr` pair resolves both: a false predicate
  is stripped **before name resolution**, so `::wasm_bindgen_test` is never
  resolved on a native build and no native adapter gains a dependency.
- **The reference implementation failed it.** `MemoryFixture` inherited
  `MID_BATCH_FAULT` three lines below its own comment arguing that *"the
  reference fixture is the one an adapter author copies and a capability nobody
  mentions is a capability nobody thinks about"*. The author knew the principle,
  applied it to the neighbouring constant, and missed this one. **A rule everyone
  agrees with and nothing checks holds until the second time somebody is busy.**

### The gap it found in `happenstance-postgres`, which is now owed work

`PostgresFixture` inherited `READ_FAULT`, whose default reads *"the injection has
to come from the adapter and this one has none to offer"*. That sentence is
**false about this store**, and it is the clearest possible illustration of why an
inherited declension is not a declension: the default says the same thing about
every adapter, including the ones for which it is wrong.

`PgReadStream` opens a `REPEATABLE READ` transaction, `DECLARE`s a server-side
cursor and issues a `FETCH` per chunk. It is precisely the paged adapter
`happenstance-sqlite` was pointing at when it declined the same capability saying
*"the paged adapters are where this capability has something to inject"*.

It now declines **by scope**, naming the injection that would work —
`pg_terminate_backend` against the reader's own backend between two `FETCH`es, or
closing the cursor beneath it, both reachable from the second pooled connection
the fixture already opens for `SECOND_HANDLE`. Building it is **phase 10's
remainder**, alongside Neon. It was deliberately not built in the change that
discovered the gap: a rule this adapter has never run plus a fault path it has
never had is not something to ship to satisfy a check added in the same commit.

## 2. `codec-foreign-tag-resolution` / A — the repair is narrower than the brief implied

`Codec::reads_tag` is a defaulted method by which a codec declares the tags it can
decode, consulted before the built-in chain. Additive, as ratified.

**What the brief did not say, and the trait's page now does:** the repair is *per
codec*, and the orphan rule makes that a hard boundary. Nobody outside
`happenstance` can override the method for `Json`, `Postcard` or `Cbor`. So the
migration it serves is *"my codec also reads the tag I used to write"* — a rename,
or one codec that knows several encodings. The migration it does **not** serve is
the one `Codec`'s own page had been using as its cautionary story: an application
that wrote under its own codec for a year and then adopted `Json` still gets
`UnknownTag`, because nothing in that build knows how those bytes were written.

A **registry** would have answered the second migration too, and costs global
mutable state, an initialisation order, and a log that reads differently depending
on what has been registered yet. **Sealing the trait** stays open and is the
cheaper, truer answer if no fourth codec ever appears — additive to take later,
impossible to undo.

**A test in the tree documented itself as failing under every option and does not
fail under this one.** `a_codec_of_your_own_writes_a_tag_no_other_codec_can_read`
claimed to fail "under every option on the table: a resolution seam on `Codec`, a
registry, or sealing the trait". True of a registry, false of a per-codec method —
and the reason it survives is exactly what a reader needs to understand about what
landed. Corrected in place rather than deleted.

## 3. `stringified-throw-visibility` / B — narrowing found two methods with no user

`StringifiedThrow` is `pub(crate)` and the crate root's invitation is withdrawn
with it, as ratified.

**The consequence the brief did not name:** `pub` methods on a `pub` type are a
configuration in which `dead_code` can **never** fire, because an exported item
always has a hypothetical caller. Narrowing the type made the compiler look, and
`message()` and `new()` had no real caller in any crate, any test or either
target. `message` was the read half of the field seal and its justification was
written entirely in terms of a caller — precisely the caller this change
withdrew — so it outlived its own argument by one commit.

A second, sharper fact: the type's two remaining users are not even in the same
configuration. The host `cfg(test)` build reaches the **type** (the `!Send` probe
names it) and none of its methods; the methods are reached only by `js.rs`'s
`#[cfg(all(test, target_arch = "wasm32"))]` tests. So the two `allow(dead_code)`s
are gated differently, each staying live in the configuration that can actually
perform the check.

`#[cfg(test)]` on the type would have been tidier and is wrong: it would put
ES-6's recorded alternative in the test profile and its counterpart in the shipped
binary, so the two shapes the fork compares would no longer be comparable in one
build.

---

## Two things that generalise beyond these six

**A citation anchor that matches many lines is already broken, and only a line
number is hiding it.** Twenty-one citations in `standards/rust/` moved during this
work. Fourteen `lint-constitution` repointed by naming the line. Seven it refused,
and **six of those were already ambiguous before any of this** — the two suite
macros in `happenstance-testkit` carry byte-identical comments, so anchors like
`Listed first` and `qualified by the caller` matched both all along, and the
citations passed only because a line number happened to land on one of them. The
lint's refusal is the feature: sharpening to an anchor that names *which* macro,
*which* trait or *which* of two prefixes is the repair, and repointing the number
alone would have left the ambiguity for whoever shifts it next.

**A count in a document nobody re-reads is a claim, not a fact.** `CLAUDE.md` said
`.kb/decisions/` held "the seventeen atoms" when it held thirty-two; `RUNBOOK.md`
said 48 briefs awaited ratification three days after seventeen were ratified;
`ci.yml`'s gated-test arithmetic said 101. The repair applied in each case was not
a new number: the count was removed where a command answers it (`ls
.kb/decisions/`), the assertion was left a **floor** rather than an equality where
a number must exist (an equality fails on the commit that adds a rule, which
trains whoever adds one to edit the number rather than read what it is for), and
the publishable-crate list is spelled with its members because a bare count had
already drifted once.

## Related

- [[ratifications-2026-09-06-pre-publication]] — the seventeen, and the queue this
  discharges
- [[es-42-marker-earned-off-at-0-2-0]] — the other decision the release pass took
- [[f2-5-holds-the-release-for-phase-10]] — discharged the same day it was taken
