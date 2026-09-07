# Ratifications at the `0.2.0` release pass: the seventeen briefs whose window closes at publication

Record: **RATIFY-0.2.0**. Staged for `/redkiln:kb-ingest` rather than written as
atoms by hand, for the reason the briefs themselves are staged: an accepted
decision atom is immutable and authored by that command.

**This file records decisions, not implementations.** Several of the seventeen
oblige code that is not written yet; the queue is at the foot.

---

## How the seventeen were selected

`.kb/_intake/remediation-2026-09-04-briefs/` holds 49 briefs. Its README
classifies each by semver impact and by the version until which the decision is
free. Bucketed:

| | Count |
|---|---|
| Already ratified and landed | 1 |
| **Free only until `0.2.0`** | **17** |
| No deadline, or a deadline after this release | 31 |

**One of the seventeen was not in the README's tables.** They classify 41 of the
49; `codec-foreign-tag-resolution.md` is among the eight they miss, and it
carries an option marked *"breaking, and therefore free only before `0.2.0`"*.
That is a publish gate sitting outside the index whose job is to surface publish
gates. The other seven unclassified briefs were checked and carry no deadline —
three of them (`citation-anchor-slack`, `repository-url-and-security-channel`,
`es-22-arm-two-is-reached-the-finding-is-wrong`) are already resolved and landed.

**The README's index is therefore not sufficient on its own**, and a later wave
should not treat it as complete. Reading every brief's own `## Cost of delay` is
what found the gap.

## Method

Nine briefs were ratified **on their own recommendation**, without individual
review. That is a deliberate use of how they were produced — author (opus/xhigh)
→ two independent critics with different lenses → revision folding the criticism
back in — and of the fact that twelve of the first thirteen flipped their own
recommendation under critique, overwhelmingly toward cheaper and less
irreversible options. A brief that has already argued itself out of its first
answer is a stronger input than a fresh opinion.

Eight were reviewed individually by the repository owner, because each carries a
breaking arm and a wrong answer there costs a major version rather than a minor.

---

## Ratified on the brief's own recommendation (9)

`fixture-declension-policy` · `event-metadata-floor` · `sqlite-blocking-seam` ·
`domain-event-guard-and-decode` · `after-opt-scope` · `append-batch-ownership` ·
`msrv-premise` · `wf-10-instruments-narrower-than-the-clause` ·
`testkit-dev-dependency-version-requirement`

Each brief's `## Recommendation` is the decision. Where a brief recommends a
combination or holds a sub-option back, that shape is ratified as written.

## Reviewed and decided individually (8)

| Brief | Decision | Note |
|---|---|---|
| `empty-decision-outcome` | **Option 3** — two success shapes | The post-critique recommendation. Catches the forgot-to-`push` bug at compile time at every call site rather than at run time on the append path, and a `#[must_use]` two-armed outcome cannot be discarded by `…await?;`. Unifies `command.rs` and `runner.rs` onto the governed side (ADR-0031, PS-38 via ADR-0030). Confidence was medium and the brief said so |
| `op-read-non-exhaustive` | **Option A** — apply it now, with `to` | Not A+D: no constructor, because nobody has been shown to construct an `Op` by hand. The break is already being taken and this is the last release at which the marginal cost is zero |
| `tags-scope-agreement` | **A + B** | A closes the lost-update direction ADR-0020 named and reuses `Query::matches` rather than inventing a second filter vocabulary; B repairs two rendered doctests that currently teach the violation. `C-additive` stays held back — no clock, additive at any minor, and its remit is Y-2's |
| `query-partition-public-surface` | **1A** — both constants stay public | A caller who cannot see the arm limit cannot tell a narrow-item split from a wide-item one. The counter-argument is accepted with open eyes: both numbers are build-time constants of *this* build of bundled SQLite, so a future `rusqlite` with raised limits cannot be exploited without a major |
| `read-page-budget-rows-bytes-or-caller` | **A for `0.2.0`; B open** | A is bounded and costs a caller nothing. B — a caller-stated knob — is a public-surface decision with an ADR's worth of consequences and would have been decided on one machine's run |
| `codec-foreign-tag-resolution` | **A now; C held open until `0.2.0`** | A is a defaulted method, additive, and converts *no legal repair exists* into *a repair exists and here is its shape*. C — sealing `Codec` — stays live and is the cheaper, truer answer if no fourth codec ever appears |
| `stringified-throw-visibility` | **Option B** — `pub(crate)` | Both in-crate roles are satisfied by a private type and nothing in the workspace consumes it as a consumer would. The crate has never been in anyone's hands, so the evidence for a public audience does not exist. Breaking, and free only here |
| `cf-18-observable-skip-reporting` | **Cost B3 before publishing** | **The one decision that adds work rather than settling it** — see below |

---

## `cf-18` is the exception, and it should be read as one

The brief declined to recommend. Its fallback was: *"If the ADR pass cannot reach
B3 before `0.2.0`, take A, say why, and leave B3 as the open question rather than
shipping the overreach for another cycle."*

**The fallback was not taken.** B3 is to be costed before publication:

> One extra emitted test per suite that fails if any capability is declined
> without an accompanying declaration file — an assertion over the fixture's own
> declarations, observable in a default `cargo test` run.

The consequence is that **CF-18 stands as written** rather than being narrowed,
and `ES-35`'s `[PROVISIONAL]` marker — which depends on the same mechanism — is
not disturbed. The cost is design and implementation before the release, on a
clause the brief measured rather than argued: libtest cannot report a skip
reason, so the obligation as written has never been met outside this repository.

If B3 proves more expensive than it looks, the fallback remains available and is
already written down. Taking it later is a decision, not a drift.

---

## What this obliges, before `0.2.0` publishes

Ratifying a breaking decision is not the same as landing it. In rough order of
size:

1. **`cf-18` / B3** — new gate machinery, and the only unbounded item here.
2. **`empty-decision-outcome` / Option 3** — a second success shape on the
   command path, `#[must_use]`, and every call site and doctest that names the
   existing one.
3. **`tags-scope-agreement` / A + B** — a run-time refusal between
   `command.rs:305` and `:311`, plus two doctest repairs.
4. **`codec-foreign-tag-resolution` / A** — a defaulted method on `Codec`.
5. **`stringified-throw-visibility` / B** — a visibility change, and the crate
   root's invitation withdrawn with it.
6. **`op-read-non-exhaustive` / A** — an attribute, landing with the `to` field.

`query-partition-public-surface` (1A) and `read-page-budget` (A) ratify what is
already landed and oblige nothing.

## Related

- [[es-42-marker-earned-off-at-0-2-0]] — the other decision this release pass took.
- [[falsifier-ledger-reconciled-at-0-2-0]] — the repair that surfaced ES-42.
