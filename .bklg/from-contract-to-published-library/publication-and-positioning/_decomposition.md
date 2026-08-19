# Briefs — 0.2.0 — where private opinions become promises

Companion artifact for `HS-P0016`. Holds this project's warranted briefs — `ux`,
`testing`, `deployment` (`../_decomposition.md`:261). **No `architecture` brief is
warranted, deliberately**: this project makes promises about a surface it does not
design (`_intake-brief.md`, Constraints). Each brief follows
`.redkiln/templates/briefs/brief.md` — Intent / Acceptance Criteria / Notes.

Read with `_grounding.md` (the verified anchors), `project.md` (AC-001…AC-016) and
`../initiative.md` (BR-05…BR-09, BR-14, BR-15, BR-17; DT-1, DT-4, DT-5, DT-6).

---

## UX brief

### Intent

Scope: **the evaluator's surface — the only "screen" this library has.** Three
rendered pages, each owned by a file in this tree:

| Surface | What renders it | Source of record |
| --- | --- | --- |
| The crates.io front page, per published crate | `readme = "README.md"` in each manifest | `crates/happenstance/README.md`, `crates/happenstance-core/README.md`, `crates/happenstance-testkit/README.md` |
| The docs.rs page, per published crate | rustdoc under `[package.metadata.docs.rs]` | `crates/*/src/lib.rs` module docs; `crates/happenstance-core/Cargo.toml:54-56` |
| The repository landing page | GitHub | `README.md` |

The repo-root `README.md` is **not** the packaged surface. `README.md:131-137`
already states why: `include_str!` cannot reach outside a package, so the root
README's examples are compiled by `xtask` and each crate's own `README.md` is what
ships in the `.crate` and what crates.io renders
(`xtask/src/package.rs:88-94`, `REQUIRED_FILES`). Any copy written only at the root
is copy an evaluator arriving from a registry search never sees. **Write to the
crate READMEs first; the root README is the third surface, not the first.**

This brief states what the experience must *be* at contract grain. It does not
choose the wording, the section order or the layout — DT-1, DT-4, DT-5 and DT-6 are
resolved in this project's `_design.md` (AC-011), and this brief is the bar that
resolution has to clear.

### Who is on the other side, and what they are trying to do

The durable `.kb/product/` layer is **structurally present and functionally empty**
— no `authority_tier: product` persona or journey atom exists
(`../initiative.md`:227-233). Personas are therefore carried from the initiative's
own distillation, `../_discovery/distillation/personas-and-journeys.md`, and are
promoted at closeout by `closeout-and-durable-audience`, not here.

**Primary: Persona 4, the evaluator (pre-adoption)**
(`../_discovery/distillation/personas-and-journeys.md`:249-313). Journey: *Decide in
one sitting* (`../initiative.md`:249-250). Two properties drive every requirement
below:

- **They are one-shot and time-boxed** in a way the other three personas are not
  (`personas-and-journeys.md`:333-338). An application author revises their
  understanding over weeks of contact with the code; the evaluator's whole decision
  window is one reading of a registry page. There is no second pass to correct a
  first impression.
- **They cannot run our suite** (`personas-and-journeys.md`:262-266) and have
  nothing external to check "DCB-compliant" against — the DCB specification defines
  no conformance process, so the label is industry-wide self-asserted
  (`personas-and-journeys.md`:272-275).

Secondary, and each arriving at the *same* pages: the application author choosing a
contract before a database, the adapter author looking for an executable definition
of correct, and the constrained-runtime developer checking whether `wasm32` is a
first-class target or a footnote (`../initiative.md`:203-220).

**Open, and not settled by this brief:** whether the evaluator is its own persona or
an earlier stage of the application author's journey
(`../_decomposition.md`:310-314; `personas-and-journeys.md`:373-377). It changes what
closeout promotes and whether DT-1's *two entry points, one per audience* option is
even coherent — so it is `_design.md`'s call under AC-011. This brief is written to
hold either way: it constrains what the surface must answer, not who is counted.

### User intents this surface must satisfy

Framed as what the person is trying to do, not as the mechanism that serves it. Each
names the project AC it discharges.

| | Intent, in the reader's words | Discharges |
| --- | --- | --- |
| **U1** | "In one screen, tell me what this is and whether it is real." | AC-011 / DT-1 |
| **U2** | "Show me what I would be trusting, and let me check it myself." | AC-009 |
| **U3** | "Tell me what happens when a read is independent of the write that produced it." | AC-010 / DT-4 |
| **U4** | "Tell me how strong each promise is, not just that one exists." | AC-003 / DT-5 |
| **U5** | "Tell me whether it runs where I run." | AC-014 |
| **U6** | "Tell me what depending on you costs me, and when that cost can change." | AC-006, AC-002 |
| **U7** | "Let me try it without cloning the repository." | AC-008 |
| **U8** | "Tell me honestly where it is not finished, before I find out." | AC-007 |

U3 is the cross-persona beat: **all four** personas share it and none of them
currently has anywhere to look (`personas-and-journeys.md`:339-345). It is also the
one with a live external trap — two DCB-labelled stores already disagree in public
on gaps-permitted vs gapless (`personas-and-journeys.md`:276-279), which is why
BR-14 exists.

### States this surface must render, and the vocabulary for them

The status table (`README.md:79-98`) is this repository's existing **state
primitive**, and its vocabulary is the thing most likely to lie at publish. It must
distinguish states that are genuinely different to a reader:

1. **Published and passes the conformance suite** — the only state that supports a
   compliance claim. `CLAUDE.md`'s rule is the bar: *an adapter that compiles but
   has not run the suite is not an adapter*.
2. **Published, no suite applies** (the testkit itself; the facade).
3. **In-tree, compiles, has not passed the suite** — must never be spelled the same
   as (1). `xtask/src/package.rs:20-22` names the live instance: `cargo package -p
   happenstance-sqlite --list` exits 0 and lists seven files, none of them required.
4. **Skeleton / `todo!()`** (`CLAUDE.md`, 🔩) — an instrument, not a target.
5. **Deliberately out of this release train** — `happenstance-sync`,
   `happenstance-sync-testkit` (`_intake-brief.md`, Non-goals). A crate absent from
   the registry with no explanation reads as abandonment; this state needs words.
6. **Clause maturity**: frozen / provisional / deferred / demoted-to-prose
   (`spec/SPECIFICATION.md:219-222`). Whether this vocabulary reaches a consumer
   surface at all is DT-5.

**Live defects in state copy, to be fixed at the publish commit** — each is a
sentence that is true today and false on the tree that ships:

- `README.md:11-16` — *"Nothing is published yet."*
- `README.md:104-109` — *`happenstance = "0.1"`* and *"That version does not resolve
  yet: the name is held on crates.io at `0.0.0`… the only way to try this is a git
  dependency."*
- `README.md:85-90` — 🔲 rows for crates this initiative's upstream projects will
  have made real, and a `happenstance` row reading *"a facade over
  `happenstance-core` today"* which `typed-layer-and-alpha-release` supersedes.
- `crates/happenstance/README.md:6-11` — *"this crate is currently a facade… adds
  nothing yet."*

### Design-system primitives — compose these, do not hand-roll

There is no CSS layer here and the implementer must not invent one: **crates.io and
docs.rs own the theme, in light and dark, and neither honours anything we write.**
The primitive layer is the rustdoc/Markdown vocabulary and this repository's own
standing forms. Compose from these, by path:

**Rustdoc primitives — `standards/rust/70-rustdoc-obligations.md`.** Load the atom;
do not re-derive it.
- RS-70-1 (`:12-89`) — `[lints] workspace = true`, then `missing_docs`,
  `# Errors`, `# Panics`. These *are* the doc-page section vocabulary.
- RS-70-2 (`:93-148`) — never write an intra-doc link that resolves in only some
  feature configurations; the gate builds docs twice for exactly this.
- RS-70-3 (`:152-191`) — `doc-valid-idents` in `clippy.toml`, never
  `allow(clippy::doc_markdown)`. This is what keeps identifiers rendered as code and
  product names rendered as prose — a typographic token, enforced.
- RS-70-5 (`:243-313`) — *name the alternative that lost, in the doc comment, once.*
  The house voice for every claim on this surface.

**Feature-visibility primitives — `standards/rust/51-features-and-no-std.md`
RS-51-5 (`:188-236`).** `#![cfg_attr(docsrs, feature(doc_cfg))]` plus
`#[cfg_attr(docsrs, doc(cfg(feature = "…")))]`, and the manifest block:

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

Present at `crates/happenstance-core/Cargo.toml:54-56` and
`crates/happenstance-testkit/Cargo.toml:56-58`. **Absent from
`crates/happenstance/Cargo.toml`** — verified by grep: the block does not exist in
that manifest. That is the crate a `cargo add` evaluator meets
(ADR-0006, `.kb/decisions/0006-bare-name-to-the-typed-layer.md`), and it is the one
whose docs.rs build is not configured. Fixing it is in scope here.

**Package-surface primitives.**
- `crates/happenstance/src/lib.rs:10` —
  `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`: the crate README's
  examples are compiled by the crate that ships them, and the prose stays out of the
  rendered page (`:1-9` says why). Every new example on a packaged surface uses this,
  not an `ignore` fence — see `standards/rust/62-doctests-and-harnesses.md` RS-62-5
  (`:230`) for the out-of-package case.
- `crates/happenstance/src/lib.rs:73` — `#![doc(html_no_source)]`. An existing,
  deliberate choice about what the docs.rs page offers; do not flip it in passing.
- `xtask/src/package.rs:88-94` — `REQUIRED_FILES`: `LICENSE-MIT`, `LICENSE-APACHE`,
  `README.md`. Containment is already asserted; presentation is not
  (`xtask/src/package.rs:4-18`), which is exactly the gap AC-007 closes by looking at
  the rendered page.

**Prose forms already in the tree — reuse the slot, do not open a new one.**
- Status callout: `README.md:11-16`.
- Status table: `README.md:79-98` — glyph **and** words in every cell.
- Quick start, with the reason it is an `xtask` doctest: `README.md:100-137`.
- Claim-with-evidence: `README.md:227-234` (the former-name section) — states the
  fact plainly and cites `ADR-0005` by path rather than asserting authority. **This
  is the form AC-009's compliance claim and AC-006's MSRV promise take.**
- Prior art: `README.md:218-225` — **DT-6's slot already exists.** If DT-6 resolves
  to *state it*, it lands here and in the crate READMEs, not in a new section.
- Disambiguation: `crates/happenstance/README.md:13-20`, *"Which crate do I want?"*
- Guarantees: `crates/happenstance/README.md:41-49` — already carries
  `forbid(unsafe_code)`, feature forwarding and MSRV-with-ADR-link. **This is U6's
  and U3's slot.**

**Forbidden on every published surface:** raw HTML, inline `style`, JavaScript,
custom colour, animated media, and any layout that assumes a viewport width. crates.io
renders a Markdown subset and docs.rs themes both light and dark; anything hand-rolled
either fails to render or fails in one theme, and the failure is permanent for that
version number.

### Accessibility floor

WCAG AA, translated honestly to a surface whose CSS we do not own. Each item is
checkable on the rendered page.

- **Colour and glyph never alone.** Every ✅/🔲/🔩 carries words in the same cell
  (`README.md:83-90` already complies — preserve it). Every badge's meaning is also
  stated in text: `README.md:8-9`'s badges are decoration, and the page must read
  correctly with images disabled or unreachable.
- **Contrast is not ours to set, so meaning must not depend on it.** No custom
  colour, no colour-coded diagram, no screenshot carrying information that is not
  also in text. Both docs.rs themes must be equally legible, which they are only if
  we add nothing.
- **Reduced motion.** No animated media at all on a published surface. crates.io
  cannot honour `prefers-reduced-motion`, so an animated demo is unstoppable for the
  reader who needs it stopped. The static form is the precedent already in the tree:
  a code block plus a `cargo run -p course-subscriptions` line (`README.md:73-77`).
- **Structure that assistive technology can navigate.** One `#` H1 per document; no
  skipped heading levels; every table has a header row; every fence declares a
  language (`rust`, `toml`, `console`) so it is announced and highlighted correctly;
  link text is meaningful on its own — never "here", "this", or a bare URL.
- **Alt text.** Every image has it, or the image does not ship.
- **Plain-text legibility is the floor, not the fallback.** The evaluator's decision
  must be reachable from the text alone.

### Interaction-quality invariants

First-class and testable. The generic web vocabulary maps onto this medium exactly
once you accept that the reader's "interaction" is *navigating a claim to its
evidence and back inside one sitting*.

**IQ-1 — In place, not a context jump.** Each of U1…U6 is answerable on the page the
reader landed on. Evidence links deepen a claim; they never *carry* it. Budget: **0
hops to read a claim, ≤ 1 hop to reach its evidence**, and that hop lands on a
specific anchor — never a repository root, never "see the specification".
*Wrong implementation this rejects:* a landing page whose answer to U3 is a link to
`spec/SPECIFICATION.md`. The evaluator has one sitting
(`personas-and-journeys.md`:333-338); a 5,000-line specification is a context jump
they do not return from.

**IQ-2 — Non-occlusion: a filter must not hide what it filters.** Two concrete forms:
- *Maturity markers annotate, they never subtract.* If DT-5 resolves toward "publish
  only frozen guarantees", the provisional ones must still be visible as provisional
  — a surface that lists 139 frozen clauses and silently omits 49 provisional ones is
  the filter hiding what it filters, and it is the exact dishonesty BR-06 exists
  against (`spec/SPECIFICATION.md:213-217`: *a provisional marker with no falsifier is
  indistinguishable from a decision nobody wanted to make*).
- *Feature gating annotates, it never subtracts.* docs.rs builds
  `all-features = true` and every gated public item carries `doc(cfg)` (RS-51-5), so
  an item is shown **with its gate** rather than being absent. An item that vanishes
  because a feature is off reads as "not supported", which is a different and false
  claim.
This is the same discipline `.kb/decisions/0010-the-suite-must-prove-itself.md`
already binds in the testkit — *a declined capability still runs the rule and reports
the stated reason, never vanishes* — applied to documentation instead of test output.
`standards/rust/40-public-surface-and-evolution.md` RS-40-5 (`:212`) is the
mechanism: a capability is an associated `const` whose constructor **rejects an empty
reason**. No surface on this project may state an absence without a reason.

**IQ-3 — Preserved position: inbound anchors survive.** Every heading anchor a
published surface exposes is a deep link somebody may already hold. At the publish
commit, either the anchor survives or **every inbound reference is updated in the
same change**. Known live inbound anchors: `README.md:9` → `#licence`,
`README.md:16` → `#status`. The house precedent for identifier stability is already
normative: `spec/SPECIFICATION.md:210-211` retains a demoted clause's ID *"so that
citations resolve"*, and `cargo xtask spec-trace` enforces the specification side.
*Wrong implementation this rejects:* a positioning rewrite that renames "Status" and
silently breaks the only two intra-document links on the page — and, worse, breaks
whatever external post linked them, permanently, because a published version cannot
be edited.

**IQ-4 — Reversibility, and where it actually lives.** Publication is the one
irreversible act in this project: a yank removes a version from the resolver and
leaves the rendered page exactly as it is
(`standards/rust/51-features-and-no-std.md`:226-231;
`standards/rust/70-rustdoc-obligations.md`:230-234). So reversibility must be bought
*before* the act, in three places:
- The rendered pages are read **before** publish, not after (AC-007). `cargo package
  --list` proves containment only (`xtask/src/package.rs:4-18`).
- Every claim is worded so it can be **superseded by a later dated claim** rather
  than needing a silent rewrite: state the fact, the implementations, and the date
  (AC-009). This is `README.md:227-234`'s form, and it is why the initiative's risk
  register calls *"passed against N adapters"* a snapshot (`../initiative.md`:434).
- The reader's own exit must be cheap and must be said to be cheap: `cargo add
  happenstance` is reversible without touching domain code, because the facade
  forwards every feature rather than defining its own
  (`crates/happenstance/Cargo.toml:14-26`).

**IQ-5 — Reachable without running anything.** No claim on a published surface may
have "run our CI" or "check out the repo" as its only evidence. The evaluator cannot
run the suite (`personas-and-journeys.md`:262-266). Evidence must be a reachable
artefact: a published crate's own docs, a committed report, a linked ADR path.
*Corollary for U7:* the quick start must be copy-pasteable into an empty project and
work with **no path dependency and no workspace feature unification** — which is
precisely AC-008's stranger-install smoke, so the two must be the same text.

**IQ-6 — No orphan vocabulary.** A term used on a consumer surface is defined or
linked on that surface. *Live defect:* `crates/happenstance/README.md:53-56` already
tells the reader the specification carries *"a maturity marker"* per clause, while
the vocabulary that word belongs to is defined nowhere a consumer reaches. DT-5 must
close this in one of two directions — publish a summary that defines the marks and
links the ledger, or stop using the word on the packaged surface — and "leave it as
is" is not one of them.

**IQ-7 — Truth at the publish commit.** No sentence on any published surface may be
false on the tree that shipped. This is an observable property of the diff, checked
against the four stale strings listed under *States* above. It is the documentation
analogue of AC-015: a page that describes a previous tree is the same class of defect
as a clause that describes a previous port.

### Acceptance Criteria

- **AC-UX-001 — the lead claim is chosen, and the first screen carries it.** DT-1 has
  a written resolution in `_design.md` naming the option that lost; the published
  `happenstance` crates.io page leads with it; a reader who stops after the first
  screenful can state what the library is and who it is for. *(AC-011, U1)*
- **AC-UX-002 — the compliance claim ships with its evidence attached, in one
  block.** On the **packaged** surface (`crates/*/README.md`), "DCB-compliant"
  appears together with the implementations the suite was run against and the date it
  was run, plus a reachable link to the evidence — in the form of `README.md:227-234`.
  Verified on the rendered crates.io page, not on the source file. *(AC-009, U2,
  IQ-5)*
- **AC-UX-003 — the positions-and-gaps promise is findable and does not overstate.**
  A newcomer can learn, in plain language and without reading source, what the library
  promises about sequence positions and gaps, at the location DT-4 chooses, ≤ 1 hop
  from the landing entry. The statement says what is **true today**: it must not imply
  ownership of `read_from_a_gap_position`, which
  `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` records as named by two
  accepted decisions and owned by neither. *(AC-010, DT-4, U3, IQ-1)*
- **AC-UX-004 — the maturity vocabulary is decided and does not occlude.** DT-5 is
  resolved in `_design.md` naming the option that lost. Whichever wins: no maturity
  term appears on a consumer surface without a consumer-reachable definition (IQ-6),
  and no treatment presents frozen guarantees while suppressing the existence of the
  49 provisional ones (IQ-2). *(AC-003, AC-011, U4)*
- **AC-UX-005 — peer positioning is decided, dated and sited in the existing slot.**
  DT-6 is resolved in `_design.md`. If stated, it lands in the Prior art slot
  (`README.md:218-225`) and the crate READMEs, is factual and dated, and is **re-read
  at the publish commit** rather than at decision time — the nearest live peer shipped
  the day before intake (`../initiative.md`:440). *(AC-011, DT-6)*
- **AC-UX-006 — the accessibility floor holds on every rendered page.** For all three
  published crates, on the rendered crates.io and docs.rs pages: one H1, no skipped
  heading levels, every table has a header row, every fence declares a language, every
  link has meaningful text, every image has alt text, no raw HTML/CSS/JS, no animated
  media, and no meaning carried by colour or glyph alone. Recorded as a dated check.
  *(AC-007, U8)*
- **AC-UX-007 — nothing is invisible on docs.rs.** All three published crates carry
  `[package.metadata.docs.rs]` with `all-features = true` — including
  `crates/happenstance/Cargo.toml`, which does not today — the docs build is green
  under all features, and every feature-gated public item carries
  `#[cfg_attr(docsrs, doc(cfg(…)))]` per RS-51-5. *(AC-007, IQ-2)*
- **AC-UX-008 — no published sentence is false at the publish commit.** The four
  named stale strings (`README.md:11-16`, `README.md:104-109`, `README.md:85-90`,
  `crates/happenstance/README.md:6-11`) are corrected, and the status vocabulary
  distinguishes *passes the conformance suite* from *compiles* for every row.
  *(AC-007, IQ-7, U8)*
- **AC-UX-009 — the constrained-runtime reader can self-identify.** From the landing
  surface alone, a `wasm32` reader can tell whether they are supported and which
  feature set to use, without reading source. `README.md:159-164` is the existing copy;
  it must be carried to the packaged surface and be true of the **published** tree.
  *(AC-014, U5, ADR-0001)*
- **AC-UX-010 — the cost of depending is stated where a consumer reads it.** The
  Guarantees slot (`crates/happenstance/README.md:41-49`) states the MSRV as a promise,
  what an MSRV bump means to a consumer, and that under 0.x the minor bump is the
  breaking-change signal — each linking the new decision atom AC-006 authors. It does
  **not** edit `.kb/decisions/0004-edition-and-msrv.md` or
  `0029-msrv-raised-to-1-97-1.md`. *(AC-006, U6)*
- **AC-UX-011 — every link resolves and every held anchor survives.** At the publish
  commit, no published surface contains a dead link or a dead intra-document anchor,
  and any renamed heading either keeps its old anchor or ships with all inbound
  references updated in the same change. Checked mechanically, not read. *(IQ-1, IQ-3)*
- **AC-UX-012 — the page's promise and the proof artefact are the same text.** The
  quick-start snippet on the packaged `happenstance` README compiles as that crate's
  own doctest via `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`
  (`crates/happenstance/src/lib.rs:10`), and the stranger-install smoke (AC-008) runs
  that same snippet against the registry version. A reader who copies what they see
  gets what the smoke proved. *(AC-008, U7, IQ-5)*

### Notes

**What this brief does not decide.** DT-1, DT-4, DT-5, DT-6 and the
evaluator-persona question are `_design.md`'s under AC-011
(`.redkiln/templates/_design.md` is the repurposed design stage). Nothing in `.kb/`
resolves any of them — grepped, no hits — so they are not inheritable and must not be
defaulted. This brief sets the bar each resolution clears; it does not pick a winner.

**Blocked on the deployment brief, and it matters more here than elsewhere.** The
crate set is disputed: `RUNBOOK.md:4450-4451` names **four** crates (including
`happenstance-sqlite`), while `CLAUDE.md` and `xtask/src/package.rs:86`'s
`PUBLISHABLE` constant name **three**
(`["happenstance-core", "happenstance", "happenstance-testkit"]`). The decomposition
already flagged this and deferred it to the deployment brief
(`../_decomposition.md`:302-309). The UX consequence is direct: the status table
(`README.md:79-98`), the *"Which crate do I want?"* block
(`crates/happenstance/README.md:13-20`) and AC-UX-002's named-implementations list all
read differently at three crates than at four, and a fourth crate needs its own
README and licence pair to render at all (`xtask/src/package.rs:88-94`). **This brief
is written against neither number.** The deployment brief must decide explicitly and
name the rejected alternative; the working default remains three, and `RUNBOOK.md`'s
phase-12 section is the stale artifact.

**Cite 49, never 46.** `RUNBOOK.md:4495` still links a `#the-46-provisional-clauses`
anchor; the ledger heading was corrected following the phase-4 recount
(`RUNBOOK.md:622-635`) and the specification's own census says **139 `[FROZEN]`, 49
`[PROVISIONAL]`, 10 `[DEFERRED]`** and two demoted (`spec/SPECIFICATION.md:219-222`).
Any consumer-facing sentence about maturity counts takes the figure from
`spec/SPECIFICATION.md:219-222`.

**No architecture brief, and this brief must not become one.** `../_decomposition.md`:261
warrants `ux`, `testing`, `deployment` only. Where a UX requirement above would need
an API change to satisfy — for example, if the U3 statement cannot be made true
without a port change — that is an upstream project's change or a new decision atom
and a re-plan, per `project.md`'s Out of scope. It is not a change made here.

**Personas are carried, not adjudicated.** All four rest on secondary evidence and
none has been directly observed (`personas-and-journeys.md`:361-369). Persona 4 is
explicitly the least evidenced — *"directionally right but not over-specified"*
(`:349-355`). Do not derive fine-grained behaviour from it beyond the intents in the
table above; promotion into `.kb/product/` is `closeout-and-durable-audience`'s
(BR-16).

**Traceability.** AC-UX-001…012 → project AC-003, AC-006, AC-007, AC-008, AC-009,
AC-010, AC-011, AC-014 → initiative BR-08, BR-09, BR-12, BR-14, BR-17 and AC-03,
AC-08, AC-09, AC-10. AC-UX-006, AC-UX-011 and AC-UX-012 are the ones the testing
brief must find an instrument for; the rest are observations on a rendered page,
which is deliberate — `xtask/src/package.rs:4-18` is explicit that the metadata
cannot stand in for how the page reads.

---

## Testing brief

### Intent

This project ships almost no new library code. What it ships is: two new
`xtask` instruments (a registry surface-diff and a publish-time clause-maturity
audit), a repair to a hand-written ledger, one new decision atom, edits to
already-shipped README/rustdoc prose, and the release event itself. The test
mix follows that shape — it is weighted toward **static** checks and **unit**
tests on the new `xtask` modules, with **exactly one** true end-to-end
instrument (AC-008's stranger-install smoke), because that is the only claim
in this project a compiled test suite cannot stand in for.

The governing discipline is `CLAUDE.md`'s corollary on the conformance suite,
applied one level up, to the tools that gate *this* release rather than to a
port implementation: *"a rule that no adapter can fail is decorative — before
adding one, name a plausible wrong implementation it rejects, and write that
implementation into the testkit's own `tests/`."* Neither new instrument here
is a conformance rule, but both are new gate steps, and the same standard
applies via the precedent already in the tree at `xtask/src/package.rs:408-457`
(`mod tests`, `#[cfg(test)]`): `scan_publishable`'s three tests each carry a
named wrong shape in a doc comment — `publish = false` reaching JSON as `[]`,
a decoy `packages` array nested inside workspace `metadata`, a `publish` value
the scanner has never seen. The surface-diff and clause-audit modules this
project adds must carry the same shape: a fixture string, a named wrong
implementation, and an assertion that the wrong implementation is rejected —
not a happy-path test that could not fail. `xtask/src/spec_trace.rs`'s own
`#[cfg(test)]` blocks (`:1809-1825`) are the second precedent for the same
discipline applied to a parser.

### Test mix, mapped to every project AC

| AC | Tier(s) | Instrument | Wrong implementation it must reject |
| --- | --- | --- | --- |
| AC-001 | unit + static | `xtask/src/package.rs`'s `reconcile`, extended for the deployment brief's crate-set decision; `package-check` gate step | `PUBLISHABLE` names a crate `cargo metadata` does not derive (or the reverse) — `reconcile` already fails this by design |
| AC-002 | unit + integration (new gate step) | new surface-diff module, following the `package.rs`/`spec_trace.rs` module-doc-plus-`mod tests` shape; run against the `0.2.0-alpha.1` registry baseline | a seeded breaking change (a removed public item, a changed signature) that the diff does not flag — CLAUDE.md's "cannot fail is decorative" rule, applied directly, is AC-002's own text: *"Running it against a deliberately breaking change fails it."* |
| AC-003 | unit + static (extends `spec-trace`) | a new audit reading every clause's maturity marker and reconciling the total against `spec/SPECIFICATION.md`'s §1.3 hand-computed census; composes with, does not duplicate, `xtask/src/spec_trace.rs`'s existing parser (`:39-57`, check-but-never-generate §1.3) | a seeded disagreement between the parsed count and §1.3's stated figure that the check does not fail — AC-003's own text names this: *"A seeded disagreement between the two fails the check."* |
| AC-004 | unit + static | an equality check between `RUNBOOK.md`'s ledger clause-ID set and `spec-trace`'s parsed `[PROVISIONAL]` list, replacing the hand count `RUNBOOK.md:618-620` already documents as rotting | a ledger row added or removed without updating the check — "checked rather than counted by hand" is AC-004's own wording |
| AC-005 | static | extends the existing empty-falsifier prohibition (CF-38, already in `spec-trace`) to require each of the ten `[DEFERRED]` clauses carry a **re-read, dated** reason, not an inherited one | a deferred clause whose reason string is unchanged from a prior phase's audit — the "re-read" requirement is the axis the existing CF-38 check does not cover today |
| AC-006 | static (process, not code) | `redkiln validate --kb` (KbFrontmatter conformance) over the new MSRV decision atom | an atom that edits `.kb/decisions/0004-edition-and-msrv.md` or `0029-msrv-raised-to-1-97-1.md` in place — `validate --kb` already checks accepted atoms against `HEAD` and fails on exactly this |
| AC-007 | human observation (rendered page) + static (containment only) | the rendered crates.io and docs.rs pages for all three crates, read and recorded per the UX brief's AC-UX-006/AC-UX-007; `cargo package --list` for containment only | `xtask/src/package.rs:4-18` is explicit that this check cannot be replaced by an automated one — presentation is not metadata |
| AC-008 | **e2e**, the project's proof artefact | a scratch project outside this workspace, `cargo add`-ing the published crate from the registry, running a write-then-read cycle | a smoke that resolves a path dependency or inherits workspace feature unification instead of the published registry version — this is exactly what "not a path dependency" in AC-008's own text forecloses |
| AC-009 | human observation (rendered page) | the compliance claim block on the packaged README, checked for the named implementations and the date, against `README.md:227-234`'s existing claim-with-evidence form | a claim with no date, or one naming implementations broader than what actually ran the suite — `ADR-0010`'s provenance discipline is what makes "passed" mean something to check |
| AC-010 | human observation (rendered page) + AC-UX-011's link check | the positions-and-gaps statement, wherever DT-4 sites it, checked against `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` for overstatement | a statement implying ownership of `read_from_a_gap_position`, which no accepted decision actually claims |
| AC-011 | static (presence check) | confirms `_design.md` exists and each of DT-1, DT-4, DT-5, DT-6 has a written resolution naming the option that lost — a content review, not a compiled test | a `_design.md` that states a winner without stating what lost, which fails the same bar `RS-70-5` sets for doc comments (name the alternative, once) |
| AC-012 | static (citation check) | PS-3's verdict, recorded as prose citing `projection-store-freeze` and `ladybug-projection-store`'s evidence, not re-derived | a verdict asserted without citing either sibling project's report |
| AC-013 | static (process) | `redkiln validate --kb` over every atom this project authors; each must come from the `/redkiln:kb-ingest` path per `CLAUDE.md`'s hand-authoring prohibition (`0269720`) | an atom hand-written directly under `.kb/decisions/` rather than staged through `.kb/_intake/` |
| AC-014 | integration (regression only — no new instrument) | the four mandatory `wasm32` steps already in `cargo xtask ci`, run against the published tree and published feature set | a feature-flag change that silently drops the `!Send` flavour from the default feature set — caught because the wasm32 build is mandatory, not optional, in `xtask/src/main.rs` |
| AC-015 | static (property of the AC-003 audit's own diff) | the clause-audit report from AC-003, read for any `[FROZEN]` clause whose text differs from the tree this project received | a frozen clause edited to match a wrong implementation rather than blocking release — DR-15 makes this an explicit blocker, not silent |
| AC-016 | **e2e**, the whole gate | `cargo xtask ci` (not `--fast`) run on the exact commit that is published | running only `--fast` and treating it as sufficient — DoD item 5 and `.redkiln/config.yaml:50-60`'s comment on what `--fast` drops (the two feature powersets, `cargo deny`, the nightly `--cfg docsrs` build) are the check against exactly this mistake |

### Merge-gate commands

This project is **not** the terminal project — `closeout-and-durable-audience`
is the one wired to `verify.e2e` in `.redkiln/config.yaml:60`. So the commands
that fire automatically at each redkiln stage transition, per
`.redkiln/config.yaml:29-60`, are:

- **Story grain**: `cargo xtask affected --base {{base}}` — maps the diff to
  workspace packages plus dependents, runs fmt/clippy/tests for that set, and
  runs the five file-reading lints and `spec-trace` unconditionally (so a
  story that only edits `SPECIFICATION.md` or `RUNBOOK.md` — most of AC-004
  and AC-005's stories — is still gated on something).
- **Integration grain (non-terminal)**: `cargo xtask ci --fast` — everything
  in the mandatory gate except the two feature powersets, `cargo deny`, and
  the nightly `--cfg docsrs` build.

**AC-016 and DoD item 5 sit outside that automatic wiring on purpose.** They
require the *full* `cargo xtask ci` on the literal publish commit, which is
not something a non-terminal project's `integration_scoped` gate runs. This
must be a deliberately recorded, manually-invoked step at the moment of
release — the proof artefact is the committed gate output, not a passing CI
badge from an earlier commit. `redkiln validate --kb && redkiln doctor` gates
AC-006, AC-012 and AC-013 (the decision-atom work) and must report the
expected six `template-drift` advisories and zero `dependency-cycle`
advisories, per `CLAUDE.md`'s backlog-CI assertion.

### Fixtures and seams to mock

- **No `EventStore` fixture is exercised by this project.** It ships no
  adapter, so `happenstance_testkit::fixtures::MemoryFixture` and
  `event_store_conformance!` are out of scope here — cited only to say the
  rule does not apply; sibling adapter projects already discharged it.
- **Reuse the string-fixture seam `package.rs` already established**
  (`package.rs:417-423`, the `METADATA` constant): the new surface-diff and
  clause-audit modules should unit-test against synthetic `cargo-semver-checks`
  output and synthetic `SPECIFICATION.md`-shaped strings, not a real registry
  round-trip or a real 200-clause document, for the same reason `package.rs`
  gives at `:38-58` (schema drift must fail the gate, not be guessed at, and a
  unit test should not need a network call to run).
- **AC-008's smoke is the one seam that must not be mocked.** It needs a real
  `cargo add` against the real registry, which only resolves after publish —
  it is inherently a post-publish verification, not a pre-merge gate step, and
  should be recorded as such rather than folded into `cargo xtask ci`.

### Acceptance Criteria

- **AC-TEST-001** — every project AC-001…AC-016 above has at least one test
  tier assigned in the mapping table, and no tier is asserted without naming
  the wrong implementation it rejects (CLAUDE.md's decorative-gate corollary).
- **AC-TEST-002** — the two new `xtask` instruments (surface-diff,
  clause-audit) each carry a `#[cfg(test)] mod tests` following the
  `package.rs:408-457` / `spec_trace.rs:1809-1825` shape: a fixture string, a
  named wrong implementation, and a failing assertion against it.
- **AC-TEST-003** — `redkiln validate --kb` and `redkiln doctor` are clean at
  every checkpoint this project's stories commit, not only at the end.
- **AC-TEST-004** — the full `cargo xtask ci` run that discharges AC-016 is a
  committed artefact (its output, dated) distinct from any `--fast` run
  recorded earlier in the project's integration stage.

### Notes

**What this brief does not test.** The public-surface *content* — whether
DT-1's lead claim is the right one, whether DT-6 should compare to a named
peer — is a judgement `_design.md` records under AC-011, not a thing a test
tier can pass or fail. This brief only tests that a resolution exists and
names what lost.

**AC-007's rendered-page checks and AC-UX-006/007's accessibility floor stay
human observation, not automation**, because `xtask/src/package.rs:4-18` is
explicit that `cargo package --list` proves containment, never presentation —
building an automated Markdown-renderer-diff for this would be inventing a new
mechanism where the grounding explicitly says to point at the existing one and
stop.

---

## Deployment brief

### Intent

The release/rollout strategy for the one truly irreversible act in this
project: publishing `0.2.0` to crates.io. Everything else this project does —
the surface diff, the clause audit, the ledger repair — exists to make that
one act safe to take once. There is no rollout in the usual sense (no
progressive traffic shift, no canary): a crate version either resolves from
the registry or it does not, and once it resolves, a `yank` removes it from
future resolution but **cannot un-render a page anyone already read**
(`standards/rust/51-features-and-no-std.md:226-231`,
`standards/rust/70-rustdoc-obligations.md:230-234` — cited in the UX brief's
IQ-4). So "deployment" here means: what is decided before the act, what gates
it, and what the path is if a later version needs to change something this
one got wrong.

### The crate-set decision (resolves the grounding's flagged tension)

**Decision: three crates ship at `0.2.0` — `happenstance-core`, `happenstance`,
`happenstance-testkit`.** This matches `xtask/src/package.rs:86`'s
`PUBLISHABLE` constant and `CLAUDE.md`'s Commands section
(*"a `cargo package --list` assertion that each of the **three** publishable
crates..."*), and it is the working default the decomposition already carried
forward (`_decomposition.md:302-309`, restated in `_grounding.md`'s "registry
crate-set tension" section).

**Rejected alternative, named explicitly per AC-001: four crates, adding
`happenstance-sqlite`**, as stated at `RUNBOOK.md:4450-4451`
("`happenstance-core`, `happenstance`, `happenstance-testkit` **and
`happenstance-sqlite`** on crates.io"). Rejected because:

- `xtask/src/package.rs:86` and `CLAUDE.md` already encode three as the
  intention list `reconcile` checks against; shipping a fourth is a change to
  that constant, not a change this project's briefs can make unilaterally
  without the deployment brief saying so — which is the whole point of this
  section.
- A fourth crate needs its own `LICENSE-MIT`, `LICENSE-APACHE` and `README.md`
  inside the package directory to pass `package-check`
  (`xtask/src/package.rs:88-94`, `REQUIRED_FILES`) and its own status-table row,
  disambiguation entry and docs.rs configuration on the UX surface — real,
  uncosted scope this project's `_intake-brief.md` does not carry.
  `sqlite-durable-store` is upstream of this project in the DAG
  (`project.md`'s Dependencies section), but "upstream and conformant" is not
  the same claim as "has a publish-ready package surface," and nothing in this
  project's scope adds that surface.
- `RUNBOOK.md`'s phase-12 section is independently known stale — it also cites
  a superseded `#the-46-provisional-clauses` anchor
  (`RUNBOOK.md:4495`) against the ledger heading corrected to 49 following the
  phase-4 recount (`RUNBOOK.md:622-635`, `spec/SPECIFICATION.md:220`) — so it
  is the source overridden here, not `CLAUDE.md`/`xtask/src/package.rs`.

**If new information changes this before the publish commit**, it is a
re-plan that updates `PUBLISHABLE`, `_intake-brief.md`'s scope and this brief
together — not a silent expansion picked up mid-implementation, per
`_decomposition.md:302-309`'s explicit flag that this must be decided early
because it changes adapter-project scope.

### Feature flags / config gating

**PS-3 — the projection port's ship shape.** Decided here on evidence supplied
by `projection-store-freeze` and `ladybug-projection-store`
(AC-012), between the two outcomes `spec/SPECIFICATION.md:4777-4784` already
writes: frozen outright, or shipped behind an `unstable-projection` feature
flag carrying a documented semver exemption. `RUNBOOK.md:601` records the
ledger's own placeholder verdict — *"Ship behind `unstable-projection` | PS-3 |
the two batch shapes disagreeing at phase 6 | 6, decided at 12"* — which this
project inherits as its default absent evidence from the two sibling projects
overriding it. Whichever way it resolves, it is written into this project's
own decision atom (AC-013), not into this brief.

If the port ships behind a feature flag, `standards/rust/51-features-and-no-std.md`
RS-51-5's `doc(cfg)` treatment (already required by the UX brief's
feature-visibility primitives) applies: the flag makes the port **visible with
its gate**, never absent — an unstable item that vanishes from docs reads as
"not supported," which AC-UX-... IQ-2's non-occlusion invariant forbids on any
consumer surface.

**The `!Send` / `wasm32` flavour (AC-014).** No feature-gating change in this
project's scope may remove `!Send` port support from the published default
feature set. The four `wasm32` steps in `cargo xtask ci` are already mandatory
(`xtask/src/main.rs:9-14`), and this project's obligation is to run them
against the *published* tree and published feature set rather than to invent
a new check — `README.md:159-164` is the existing claim being extended to a
registry-installed crate rather than a workspace checkout.

### Migration, backfill and rollback posture

**Migration / backfill: N/A, stated explicitly.** This is the first `0.2.0`
publish of these three crates — there is no prior production data shape for
any consumer to migrate, and this library defines no schema of its own to
backfill (`MemoryEventStore` and the adapters under construction each own
their own storage schema, and none of that is this project's scope). The
reserved names currently sitting at `0.0.0`
(`CONTRIBUTING.md:298-300`, *"Cargo treats every `0.0.x` version as
incompatible with every other, so there is no compatible predecessor"*) is a
registry-reservation fact, not a migration scenario — there is nothing at
`0.0.0` a consumer could have depended on in a way `0.2.0` must be compatible
with.

**Rollback: the irreversibility is structural, so the posture is
front-loaded, not back-loaded.** A crates.io publish cannot be un-shipped.
`cargo yank` removes a version from future dependency resolution but leaves
every already-rendered docs.rs and crates.io page exactly as it was
(`standards/rust/51-features-and-no-std.md:226-231`,
`standards/rust/70-rustdoc-obligations.md:230-234`) — so yanking is a tool for
"stop new consumers from picking this up," never a tool for "undo what was
said." Given that, this project's entire instrument set — the surface diff
(AC-002), the clause audit (AC-003), the rendered-page read (AC-007), the
stranger-install smoke (AC-008) — **is** the rollback strategy: it moves the
cost of being wrong to before the act, where a mistake blocks a release
instead of requiring a correction after one. AC-015's explicit rule for the
one failure mode that slips through anyway — a `[FROZEN]` clause found wrong
at audit time — is the same posture at the level of a single clause: **block
and re-plan, never quietly amend to make the date**, which DR-15 states
directly.

**If a published version later needs to change**, the correction ships as a
new, forward-dated `0.(2+n).0` (see Release path below), and the UX brief's
IQ-4/AC-UX-... form governs how the change is worded: *"state the fact, the
implementations, and the date"* (`README.md:227-234`'s form) rather than a
silent rewrite of what an earlier version's page said. A yank is reserved for
the case where the published artefact itself is broken (fails to build,
license-missing per `xtask/src/package.rs`'s D11), not for correcting prose.

### CI implication

The two new instruments this project adds (surface-diff for AC-002,
clause-audit for AC-003/AC-004) become **mandatory** steps in
`xtask/src/main.rs`'s gate, not optional ones. `xtask/src/main.rs:33-38`
already draws the Mandatory/Optional line on a specific basis: optional means
*skipped when the probed tool is absent* (`cargo-hack`, `cargo-deny`, a
nightly toolchain) — it never means *skipped when the check would fail*. Both
new instruments always have their inputs available once written (there is no
external tool to probe for), so gating them behind a probe would be
inventing an escape hatch CLAUDE.md's decorative-gate rule exists to forbid.
Concretely:

- Both steps are added to the Mandatory list, which means both `cargo xtask
  ci --fast` (this project's own non-terminal `integration_scoped` gate,
  `.redkiln/config.yaml:50-56`) and the full `cargo xtask ci` pick them up
  automatically — `--fast` only drops the two feature powersets, `cargo deny`,
  and the nightly `--cfg docsrs` build (`.redkiln/config.yaml`'s comment at
  that key), and neither new step is one of those four.
- This has a consequence for the DAG: `AC-002`'s surface-diff needs the
  `0.2.0-alpha.1` registry baseline that only exists once
  `typed-layer-and-alpha-release` has shipped it. Until that baseline exists,
  the new step must fail closed with a stated reason (never silently pass),
  matching `.kb/decisions/0010-the-suite-must-prove-itself.md`'s "a skip is
  reported, never silent" discipline, cited by the grounding as the pattern
  to reuse.
- AC-004's ledger repair and AC-005's deferred-clause re-read are one-time
  content edits to `RUNBOOK.md` and `spec/SPECIFICATION.md`, not new gate
  steps by themselves — but AC-004's checked-not-counted requirement (DR-4)
  does need a small extension to `xtask/src/spec_trace.rs`'s existing parser
  (reuse, per the grounding's "code patterns to follow": compose with the
  existing split rather than write a second parser), which *is* a permanent
  gate addition and belongs in the same Mandatory list.

Because both new steps land in Mandatory, `cargo xtask ci`'s own module
documentation (`xtask/src/main.rs:8-23`) needs an update in the same change
that adds them — the doc comment is the one place in this repository that
already states, in prose, exactly what the gate proves, and this project's
whole thesis is that a claim must ship next to what proves it.

### Release path if a published version changes

Under `0.x`, the minor version **is** the breaking-change boundary — this is
already the project's own stated non-goal boundary (`project.md`'s "Any `1.0`
or post-1.0 semver commitment" exclusion) and the promise the UX brief's
Guarantees slot states to a consumer (`crates/happenstance/README.md:41-49`).
Concretely:

- Every future change to the three published crates ships as a new
  `0.(2+n).0`; there is no patch-vs-minor ambiguity to resolve for a
  pre-1.0 crate, and no change is a silent patch.
- `cargo-semver-checks`, already wired at `CONTRIBUTING.md:291-296` with
  `--baseline-rev` at the PR's branch point, keeps gating every future pull
  request's diff from where it branched. It proves *that PR* did not break
  what it branched from; it proves nothing about drift across several merged
  PRs (`CONTRIBUTING.md:295-300` states this limit in its own words).
- AC-002's registry-baseline surface diff, once it exists as a mandatory gate
  step, is what closes that gap for every release **after** this one too —
  each future release re-runs it against the *then-current* published
  version, not against a branch point, which is the check that would have
  caught a break accumulated across several already-merged PRs.
- The MSRV promise (AC-006's new decision atom) states what an MSRV bump
  means to a consumer now that a real one exists downstream — the atom is the
  artefact of record; this brief only states the release-path consequence:
  an MSRV bump is a minor-version event under the same 0.x rule, recorded and
  justified in the atom rather than silently rolled into an unrelated release.

### Acceptance Criteria

- **AC-DEP-001** — the crate set for `0.2.0` is exactly `happenstance-core`,
  `happenstance`, `happenstance-testkit`, matching `xtask/src/package.rs:86`;
  the four-crate alternative from `RUNBOOK.md:4450-4451` is recorded as
  rejected, with the reason, in this brief and in AC-013's decision atom.
  *(AC-001)*
- **AC-DEP-002** — PS-3's ship-shape verdict (frozen vs. `unstable-projection`)
  is recorded, citing `projection-store-freeze` and
  `ladybug-projection-store`'s evidence by report, not re-derived. If
  feature-gated, the gate carries `doc(cfg)` treatment per RS-51-5. *(AC-012)*
- **AC-DEP-003** — the four mandatory `wasm32` steps in `cargo xtask ci` pass
  against the published tree and published default feature set. *(AC-014)*
- **AC-DEP-004** — migration/backfill is recorded as N/A with the reason
  stated (first publish, no prior compatible version at `0.0.0`); rollback
  posture is recorded as front-loaded (the gate blocks) with `yank`'s actual,
  limited effect stated rather than assumed. *(supports AC-015, AC-016)*
- **AC-DEP-005** — the surface-diff (AC-002) and clause-audit (AC-003/AC-004)
  instruments are added to `xtask/src/main.rs`'s **Mandatory** step list, not
  gated behind a tool probe, and the module's own doc comment is updated in
  the same change to state what the gate now proves. *(AC-002, AC-003, AC-004)*
- **AC-DEP-006** — the release path for any future `0.x` change is stated:
  minor-bump-is-breaking, `cargo-semver-checks` at the PR grain, the new
  registry-baseline diff at the release grain, and the MSRV atom's promise —
  each citing where it already runs or will run. *(AC-006, supports AC-002)*

### Notes

**Nothing here amends `.kb/decisions/0004-edition-and-msrv.md` or
`0029-msrv-raised-to-1-97-1.md`.** Both are accepted and immutable; the MSRV
promise is a new, sibling atom per AC-006/DR-6, and this brief only states the
release-path consequence of that atom existing, not its content.

**This brief does not decide DT-1, DT-4, DT-5 or DT-6.** Those are positioning
decisions the UX brief scopes and `_design.md` resolves under AC-011; this
brief's PS-3 and crate-set decisions are the only two first-contact-adjacent
calls that belong to *deployment* rather than *positioning*, because both
determine what actually ships rather than how what ships is described.

**No architecture brief exists, and this brief does not become one.** Where
the CI-implication section above touches `xtask/src/main.rs`, it is scoping a
release-gate instrument, not the library's public API surface —
`project.md`'s Out of scope section reserves any API surface change for an
upstream project and a re-plan.
