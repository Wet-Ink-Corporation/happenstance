# Map — the original vision, its promises, and how they drifted

Reader: vision/promises mapper. Tree read: detached worktree at `1f92d08` (== `origin/main`),
read-only. Date of audit: 2026-10-06. No cargo command was run.

Convention: **CLAIM** = a document says it; **SHOWN** = code, manifest, registry or test output
shows it; **unverified** = no evidence found. All paths are relative to the worktree root.

---

## 0. A caveat on "earliest": git does not reach the beginning

The project's own history predates its git history.

| Fact | Evidence |
|---|---|
| The repository has **two root commits**, both dated September 2026, and 86 commits in total | `git rev-list --max-parents=0 HEAD` → `cee9e87` (2026-09-08), `673dcc5` (2026-09-07); `git log --oneline \| wc -l` → 86 |
| The first commits are not the start. They are status repairs to an existing, mature tree. For example, `673dcc5` is titled "Repair three status claims that had gone stale", and `cee9e87` adds hundreds of `.bklg/` files wholesale | `git show --stat 673dcc5`, `git show --stat cee9e87` |
| Commits the evaluations cite as their base, `9fd2337` (2026-08-05/06) and `2a65d76`, **do not exist in this repository** | `git cat-file -t 9fd2337` → `fatal: Not a valid object name`. Same result for `2a65d76` |
| The runbook's citations were moved "off the commit the squash left behind" | commit `db99a72`; `runbook/handover.md:144` ("both squash and rebase rewrite commit ids") |

**Consequence:** the earliest authoritative sources for the vision are the **dated documents**, not
commits. In date order:

1. ADR-0001, ADR-0002 and ADR-0005, all dated 2026-08-05 (`references/adr/0001-async-port-flavours.md:4`, `references/adr/0005-rename-to-happenstance.md:4`).
2. `review-docs-adr.md` and `research-rust-ecosystem.md`, both 2026-08-05 @ `9fd2337` (`references/evaluation/review-docs-adr.md:4`, `research-rust-ecosystem.md:3-4`).
3. `ARCHITECTURAL-EVALUATION.md`, 2026-08-06 @ `9fd2337` (`:3`).
4. `PRESSURE-TEST.md` @ `2a65d76` (`:10-11`).
5. The seeds, from 2026-08-11 onward (`references/seeds/user-documentation.md:7`).
6. The frozen `RUNBOOK.md`, frozen at `65253fc` on 2026-09-28 (`RUNBOOK.md:3`).

The pre-August-6 README is not in the tree. Its wording survives only through quotations in the reviews.

---

## (a) The vision, in the project's own words

### a.1 What the founding README said (reconstructed from quotations, 2026-08-05/06)

The original front page is gone, but four separate reviews quote it consistently:

- **"fast, efficient"** were "the first two words of the stated goal" (`references/evaluation/review-docs-adr.md:369`, `ARCHITECTURAL-EVALUATION.md:830`, `review-packaging-semver.md:342`, re-quoted in `benchmarks/README.md:81`).
- **"with batteries"** / "Comes with batteries" was a product promise on the front page (`ARCHITECTURAL-EVALUATION.md:885`, quoting the old `README.md:3-5`).
- The goal was **"widely usable, trusted, respected"** (`review-docs-adr.md:373`).
- It was to be **"a delight to lean on"** / "delight for a working Rust developer" (`review-docs-adr.md:374`, `research-rust-ecosystem.md:3`, `ARCHITECTURAL-EVALUATION.md:900`).
- The scope was "the goal of a **published, trusted 0.1**" (`ARCHITECTURAL-EVALUATION.md:4`).

### a.2 The founding *technical* motive (ADR-0001, 2026-08-05)

> "The motivating deployment needs both: a local-first native application syncing with SQLite inside
> a Durable Object behind a Rust Worker." — `references/adr/0001-async-port-flavours.md:45-46`

The same target is stated in the sync crate root, which dates from phase 2 and is still present:

> "A local-first application holds its own event store on the device and syncs with a shared
> instance — the motivating deployment being SQLite inside a Cloudflare Durable Object, reached
> through a Rust Worker." — `crates/happenstance-sync/src/lib.rs:54-57`

### a.3 The vision as stated today

> "An opinionated, storage-agnostic event sourcing library for Rust, built on the Dynamic
> Consistency Boundary specification: a contract for storage, and a published conformance suite
> that decides who meets it." — `README.md:3-6`

> "happenstance's bet is different: ports plus a *published* conformance suite, so that 'storage
> agnostic' is a claim anyone can check and third parties can ship adapters against." —
> `README.md:359-361`

> "A library an application author can `cargo add` and rely on, and that an adapter author can
> implement against with a bar that tells them when they are done." —
> `references/seeds/remaining-runway.md:70-71`

> "…a contract that has been used, not just specified; adapters that have run the conformance suite
> and passed it, across storage shapes that genuinely disagree with each other; a published release
> whose promises are real; and honest answers — not silence — on replication and on what happens
> when a store no longer holds its whole history." — `references/seeds/remaining-runway.md:73-76`

> "The local-first and edge case specifically … That constraint is load-bearing and shapes
> everything; work that quietly drops it has changed the product." —
> `references/seeds/remaining-runway.md:87-90`

The typed crate's self-description is "DCB-compliant event sourcing with batteries: typed events,
decision models and projection runners over a storage-agnostic contract." It appears at
`crates/happenstance/Cargo.toml:3` and `crates/happenstance/src/lib.rs:19`.

### a.4 The positioning question was raised and never settled in the README

The 2026-08-06 evaluation (N5, `ARCHITECTURAL-EVALUATION.md:879`) argued for a different lead:

> "**DCB is the mechanism, local-first-that-syncs is the position.** … 'the event sourcing library
> for local-first Rust applications that sync' is a position nobody occupies."

`revised-runway.md:312-318` repeated it.

What the README does instead (**SHOWN**):

- It leads with DCB and the conformance suite (`README.md:3-6`, `:40-91`).
- It contains **zero** occurrences of "local-first". `grep -rn -i local-first README.md docs crates/*/README.md` finds no hits in any of them. The only hits in crate roots are `happenstance-sync/src/lib.rs:27,54` and `happenstance-sqlite/src/lib.rs:51`.
- The owner's 2026-09-28 decision to put sync inside 1.0 rests on the premise that **"replication is the headline for first adopters"** (`.kb/decisions/wi-40b321-…md:8,29`).

The headline the owner decided on and the headline the README shows therefore disagree.

---

## (b) Explicit promises, goals and non-goals

Status column: **kept** (still binding), **met** (delivered, with evidence), **open**, **dropped**
or **changed**.

### Identity and positioning

| # | Promise / goal | Source (earliest first) | Status today |
|---|---|---|---|
| P1 | **DCB-compliant**: implement the dcb.events specification's MUSTs | `README.md:3-4`; the evaluation found it "conforms to every MUST in the DCB specification" (`ARCHITECTURAL-EVALUATION.md:18`) | **CLAIM kept.** The spec in `spec/SPECIFICATION.md` carries 203 clause IDs: 152 FROZEN / 32 PROVISIONAL / 12 DEFERRED / 7 NON-NORMATIVE (`spec/SPECIFICATION.md:228-230`). Whether tests pass now is not verified by this reader (no cargo run). |
| P2 | **Storage-agnostic**: a contract crate plus adapter crates, never feature flags | ADR-0002/0005 crate tables (`references/adr/0005…md:36-44`); `README.md:267-270` | **kept.** Today 7 crates are published at 0.3.2 (registry, see P13). |
| P3 | **A published conformance suite is the differentiator and the bar.** "An adapter that compiles but has not run the suite is not an adapter." | `ARCHITECTURAL-EVALUATION.md` §2 ("This is the project's best idea"); `README.md:311-341`; CLAUDE.md "The rule that matters" | **kept and strengthened.** The README CLAIMS 116 rules in four families (`README.md:333-341`). ADR-0010 requires every rule to have a mutant that fails it (`.kb/decisions/0010-…md:12-28`). |
| P4 | **"fast, efficient"** | founding README, quoted at `review-docs-adr.md:369` | **dropped / changed.** "This README makes no performance claim, and that is deliberate" (`README.md:285-288`). A measured record exists out of the gate (`benchmarks/`, `README.md:290-309`). |
| P5 | **"with batteries"**: typed events, codec, decision model, command loop with retry, projection runner | founding README (`ARCHITECTURAL-EVALUATION.md:885-917`) | **softened, then partly delivered.** Phase 0 ticked "Soften the README's 'with batteries' tagline" (`RUNBOOK.md:1102-1106`), but the phrase survives in `crates/happenstance/Cargo.toml:3`. See the batteries ledger in (c). |
| P6 | **"widely usable, trusted, respected"** and a **"delight"** to use | `review-docs-adr.md:373-374` | **open.** Trust artefacts exist (SECURITY.md, MSRV promise, CHANGELOG). The usability evidence, a reader who is not the author, is not built: phase 20's `HS-P0024` comprehension-evidence is "not yet built" (`runbook/phases/20-docs-that-teach.md:19-22`). |
| P7 | **Local-first, edge and wasm32 as a load-bearing constraint**: a `!Send` port flavour so that a Durable Object adapter is possible | ADR-0001 (`references/adr/0001…md:36-46`); `remaining-runway.md:87-90` | **kept in the code; absent from the positioning.** The two-flavour ports are a binding constraint (CLAUDE.md constraints 1 and 3). `happenstance-cloudflare` is published (registry). See (c) D10 for the platform walls workerd found. |

### Target runtimes and storage backends

| # | Promise / goal | Source | Status today |
|---|---|---|---|
| P8 | **Native tokio (`Send`) and `wasm32-unknown-unknown` on Cloudflare Workers (`!Send`)** | `references/adr/0001…md:38-43` | **met for the port.** "CI builds `happenstance-core` for `wasm32-unknown-unknown` on every commit" (`README.md:263-265`, CLAIM). |
| P9 | **SQLite as the first durable adapter**, rusqlite | eventum crate table (`references/adr/0005…md:41`); phase 8 | **met, host only.** Published 0.3.2 (registry). "native means host only" (`crates/happenstance-sqlite/src/lib.rs:26`; `README.md:15` of that crate). The local-first *on-device browser* store is unbuilt (phase 19a/19b "not started", `runbook/README.md` status table). |
| P10 | **A Cloudflare Durable Object event store** | ADR-0001; phase 9 | **met at the shim level, and not yet on the real runtime.** It passes under a `node:sqlite` shim, not workerd (`README.md:177-183`; `SECURITY.md:44-50`). ADR-0066 §6 promises workerd conformance before 1.0. |
| P11 | **A graph projection store (Ladybug)** | original crate set `eventum-ladybug` (`references/adr/0005…md:42`); ADR-0025 | **dropped.** Retired at phase 17 (ADR-0078); see (c) D3. |
| P12 | **Postgres and Neon, as instruments rather than flagships** | `revised-runway.md:302-318`; `README.md:272-281`; phase 2 instrument portfolio (`RUNBOOK.md:799-846`) | **added, then promoted to the release set** at `e597c34`. Published 0.3.2 (registry). |

### Release, compatibility and 1.0

| # | Promise / goal | Source | Status today |
|---|---|---|---|
| P13 | **First publish was "0.1"**, later **0.2.0** | `ARCHITECTURAL-EVALUATION.md:4`, `:919-948`; `RUNBOOK.md:4294-4315` | **met as 0.2.0** (2026-09-10). **SHOWN on the registry** (crates.io API, 2026-10-06): `happenstance`, `-core` and `-testkit` each have `0.3.2, 0.3.1, 0.3.0, 0.2.0, 0.2.0-alpha.1 (yanked), 0.0.0`. `-sqlite`, `-cloudflare`, `-postgres` and `-neon` have `0.3.2, 0.3.1, 0.3.0, 0.2.0, 0.0.0`. |
| P14 | **Payloads are opaque bytes; no serde in the core's defaults; replication forwards bytes byte-for-byte** | ADR-0003 (`.kb/decisions/0003…md:12-24`) | **kept, and still provisional on its own terms.** ADR-0003 "lifts at phase 13 when happenstance-sync round-trips an event … without deserialising its payload" (`.kb/decisions/0003…md:13-14`). Phase 13 has not started. |
| P15 | **The typed layer gets the bare name**; the contract is `happenstance-core` | ADR-0006 (`.kb/decisions/0006…md:12-24`) | **met.** Both are published (registry). |
| P16 | **Sync / replication**: a third port (`SyncPeer`, `IngestStore`), peer-to-peer *and* hub-and-spoke, plus a `happenstance-sync-testkit` | `crates/happenstance-sync/src/lib.rs:17-67`; ADR-0005 crate table | **open: a skeleton.** `publish = false` (`crates/happenstance-sync/Cargo.toml:12`), and its description reads "Not yet implemented" (`:3`). On crates.io neither `happenstance-sync` nor `happenstance-sync-testkit` exists (API: "does not exist"). Phase 13 is "not started". It is **inside 1.0** by D-1 (`wi-40b321`). |
| P17 | **The MSRV is a promise from 0.2.0**; after 1.0, rises are bounded | ADR-0004 → ADR-0029 (1.97.1) → ADR-0037 → ADR-0067 | **kept.** `README.md:203-210`. |
| P18 | **Licence**: `MIT OR Apache-2.0`, and the nine 1.0 crates stay so "for all of 1.x" | `README.md:389-392`; ADR-0066 §8 (`.kb/decisions/0066…md:327-328`) | **kept**, and a commitment for 1.x. |
| P19 | **Security**: an email channel, with scope limited to the published crates | ADR-0041; `SECURITY.md:3-35` | **kept.** "Pre-1.0, and honestly so. There is no long-term support branch and no backporting" (`SECURITY.md:77-80`). |
| P20 | **What 1.0 means** | ADR-0066 (phase 16, 2026-09-29) | **defined, not reached.** Details follow this table. |
| P21 | **Documentation for strangers**: compiled narrative docs, with "a reader who is not the author getting through it" | `references/seeds/user-documentation.md:134-145`; phase 20 | **open.** Phase 20 is "in progress", and 3 of 6 projects are not built (`runbook/phases/20-docs-that-teach.md:19-22`). A falsifier exists: `examples/outside-projection-adapter` is "written from the rendered documentation alone" (CLAUDE.md map). |
| P22 | **No performance claim gates a merge**: an adapter that is slow is still conformant (CF-34) | `README.md:306-309` | **kept** (CLAIM). |

ADR-0066 defines 1.0 as follows:

- **Nine crates**: core, happenstance, testkit, sqlite, cloudflare, postgres, neon, sync and sync-testkit (`:13-16`, `:118-125`).
- Every non-frozen clause gets one disposition (`:142-166`).
- `1.0.0` ships in lockstep, and the crates version independently after that (`:238-241`).
- Cloudflare conformance must run on workerd (`:279-295`).
- The rc soak has five conditions and no time floor (`:310-322`).

Two explicit non-goals sit beside these promises:

- **NG1 — no derive macros at 0.1.** ADR-0033 measured the ceremony ratio at 0.50:1 against a 1:1 threshold (`.kb/decisions/0033…md:12-30`), and the roadmap keeps the macros off it (`runbook/roadmap.md:148-150`).
- **NG2 — no DCB wire interoperability (WF-1).** The deferral holds because DCB publishes no wire format (`runbook/roadmap.md:151-152`).
- **Also not a goal: teaching readers who know neither event sourcing nor DCB.** "**Not** the reader who knows neither event sourcing nor DCB" (`references/seeds/user-documentation.md:163-165`).

### Promised audiences (personas)

`.kb/product/` holds **no persona atoms**: `ls .kb/product` → `README.md` only. Its README says
personas are promoted "at closeout", and that never happened before redkiln was retired.

The personas exist only in the frozen backlog:

- **HS-I0006** (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`) has four:
  1. Application author (`:42`)
  2. Adapter author (`:114`)
  3. **Local-first / edge Rust developer** (`:182`)
  4. Evaluator (`:249`)
- **HS-I0007** (`.bklg/docs-that-teach/…/personas-and-journeys.md`) has three: application author (`:56`), adapter author (`:148`) and evaluator (`:225`). The **local-first / edge persona is gone**: grepping for "local-first|edge" in that file returns no persona heading.

---

## (c) How the vision drifted

| # | Drift | Kind | Evidence |
|---|---|---|---|
| D1 | **The project was renamed from `eventum` to `happenstance` on 2026-08-05.** The crates.io collision went away, and the bare name went first to the contract and four hours later to the typed layer. | changed | `README.md:374-387`; ADR-0002 → ADR-0005 → ADR-0006 |
| D2 | **`happenstance-runtime` was dissolved.** The typed layer took the bare name. The projection runner then split across the decode seam (ADR-0007), and ADR-0031 later collapsed it upward. | changed | `.kb/decisions/0006…md:17-20` ("happenstance-runtime ceases to exist"); `.kb/decisions/0007…md`; `0031-the-runner-collapses-upward.md` |
| D3 | **The Ladybug graph projection store was in the original crate set and was retired at phase 17.** It passed the projection suite at phase 11 but could never be published, because `lbug` does not build on docs.rs. ADR-0066 put it outside 1.0. ADR-0078 then excluded it from the workspace: "there are just too many issues with it". The loss is the only non-SQL batch shape. | dropped | `references/adr/0005…md:42`; `.kb/decisions/0078…md:12-25`; `runbook/roadmap.md:111-116`. **Contradiction:** ADR-0078 says "The crates.io `0.0.0` reservation is not yanked. The name stays claimed" (`.kb/decisions/0078…md:25,158`). The crates.io API on 2026-10-06 returns "crate `happenstance-ladybug` does not exist". Commit `cee9e87` already recorded that the reservation (O-4) "was not executed", with the publish "left with the owner". |
| D4 | **Postgres and Neon were added as falsification instruments**, not as targets. Both were then promoted into the release set at `e597c34`, which re-opened a set the owner had settled at five. | added → promoted | `RUNBOOK.md:799-846`; `revised-runway.md:302-318`; CLAUDE.md; commit `e597c34` |
| D5 | **The first release moved from 0.1.0 to 0.2.0**, because phase 4's breaking changes were detected by `cargo-semver-checks` even though nothing had been published. | changed | `RUNBOOK.md:4294-4315` |
| D6 | **The ordering was inverted.** The original runbook built adapters before the contract freeze. The evaluation and pressure test re-sequenced the work "by blast radius": `!Send` proof first, skeletons, then the freezes, the typed layer and the adapters. | changed (method) | `ARCHITECTURAL-EVALUATION.md:13-31`, `:919-960`; `RUNBOOK.md:5-46` |
| D7 | **Performance claims were withdrawn.** "fast, efficient" became "no performance claim"; benchmarks were built but kept out of the gate. | dropped claim → measured record | `README.md:283-309`; `benchmarks/README.md:75-83`; ADR-0066 §9 (`:332-333`) |
| D8 | **The batteries scope was narrowed** (detail in the batteries ledger after this table). | narrowed | `RUNBOOK.md:1102-1106` |
| D9 | **Sync moved from "off the 0.1 path" to inside 1.0.** The evaluation decoupled publication from replication (`ARCHITECTURAL-EVALUATION.md:964-966`). Then D-1 (2026-09-28) put sync inside 1.0 against the agent's recommendation, which the owner overrode, adding about 17 working days. Sync has gone from never started to on the critical path. | changed (raised) | `.kb/decisions/wi-40b321…md:8,29-30`; `runbook/roadmap.md:90-95`; status table: phase 13 "not started" |
| D10 | **The edge target met real platform walls.** The workerd job (PR #34, merged as `1f92d08`) went red on purpose: 97 rules executed and 96 passed. The failure is VT-23's 128-item rule against workerd's 100-parameter and 5-compound-term limits, and the adapter "serves at most 5 query items and 45 tags in one item". This narrows what the Cloudflare adapter can promise at 1.0. | discovered constraint | `runbook/handover.md:40-50`; `git log -1 1f92d08` ("landed red on purpose") |
| D11 | **SQLite on wasm32 (the browser local-first store) was added as phases 19a/19b**, after 2026-09-28. It is off the 1.0 path and not started. | added | `references/seeds/sqlite-on-wasm.md:92-118`; `runbook/roadmap.md:73-74` |
| D12 | **A branded documentation site (phase 22) was added on 2026-09-29.** It contradicts the docs seed's constraint that "neither is a bespoke site". The owner widened scope on the ground that the evaluator reads before running `cargo add`. | added; contradicts an earlier constraint, with a recorded owner decision | `references/seeds/user-documentation.md:175-177`; `runbook/phases/22-docs-site.md:8-30`; commit `09854b3` |
| D13 | **The local-first / edge persona was lost** between HS-I0006 and HS-I0007, and none was ever promoted to `.kb/product/`. | dropped (audience) | see the persona section above |
| D14 | **Tracking tool retired.** Redkiln and `.bklg/` were frozen on 2026-09-28: 190 stories, none at done. The runbook became a directory, and the monolith was frozen at `65253fc`. | process change | CLAUDE.md "Where the work lives"; `runbook/roadmap.md:96-102`; `RUNBOOK.md:3` |
| D15 | **The 1.0 timeline grew.** The original critical path to 0.1 was "≈ 8 weeks" (`ARCHITECTURAL-EVALUATION.md:940`). The 1.0 path is now "about 62–75 working days" solo, up from 34–43 (`runbook/roadmap.md:78-79`). Phase 17 alone went from 5–8 days to 25–30 (`runbook/roadmap.md:68`; ADR-0072 re-estimated about 275 hours). | estimate drift | as cited |
| D16 | **A Crux application framework above the library was explored.** The four `research-crux-*` documents are explicitly "mutable speculation", "constrain nothing" and are "not an ADR". | added (speculative, non-binding) | `references/evaluation/README.md:13-35` |
| D17 | **Some front-page status text is stale relative to D3 and the release set.** The core README still lists "an embedded graph database for the projection role" among five adapters (`crates/happenstance-core/README.md:14`). The happenstance README says "Four ship at `0.2.0`" and "`0.2.0` is the first stable release" while 0.3.2 is current (`crates/happenstance/README.md:11,29`). The root README handles it correctly ("until its adapter was retired at phase 17", `README.md:21`). | doc staleness | as cited |
| D18 | **`happenstance-sync`'s manifest still reads "Not yet implemented."** It has been a phase-2 skeleton since mid-August, with real traits and a `MemorySyncPeer`, but no runner and no testkit. | open | `crates/happenstance-sync/Cargo.toml:3`; `crates/happenstance-sync/src/lib.rs:3-15`; `runbook/roadmap.md:19-22` |

### The batteries ledger (2026-08-06 vs today)

| Battery | 2026-08-06 status (`ARCHITECTURAL-EVALUATION.md:889-907`) | Today | Evidence |
|---|---|---|---|
| Typed events + codec | planned | **present** | `crates/happenstance/src/codec.rs`, `domain.rs` |
| A decision model whose query and fold cannot disagree | "the single most important battery" | **present**, with a trybuild proof | phase 7 proof (`runbook/README.md` row 7) |
| Composing decision models | planned | **present** | `crates/happenstance/src/composition.rs` |
| Command loop with retry | planned | **present** | `crates/happenstance/src/command.rs` |
| given/when/then testing DSL | absent | **present** | `crates/happenstance/src/testing/mod.rs:1-7` |
| Fault injection (`FaultyStore`, `GappyMemoryStore`) | absent | **present** | `crates/happenstance-testkit/src/faulty.rs:248` |
| Projection runner | planned | **present but gated** behind `unstable-projection`; phase 18 "not started" | `README.md:30-32`; ADR-0066 §5 |
| Live subscriptions / tail | absent | **deferred**: "ES-32: absent, and stated as absent", and the runner polls | `RUNBOOK.md:586` |
| Snapshots | absent | **deferred past 1.0** | `runbook/ledgers.md:72` |
| `tracing` / metrics | absent | **deferred past 1.0** ("purely additive") | `runbook/ledgers.md:73` |
| Derive macro | out of 0.1 | **out** (ADR-0033) | `runbook/roadmap.md:148-150` |
| Benchmarks | absent | **present, out of gate** | `README.md:283-309` |

---

## (d) Gaps and concerns for the synthesis

1. **The positioning is inconsistent.** The founding motive (ADR-0001) and the owner's 1.0 premise (wi-40b321) are local-first sync and replication. The README, the docs and the HS-I0007 personas never mention local-first, and the piece that would deliver it (sync) has not started.
2. **The registry contradicts ADR-0078 on the ladybug name** (D3). The record claims a held reservation that crates.io says does not exist. The name is unprotected.
3. **The sync names are unclaimed.** ADR-0066 flags the claim as on the critical path; the registry confirms both are absent.
4. **Edge promises are narrower in fact than in the frozen spec.** VT-23's 128-item rule fails on real workerd (D10). How that is resolved will shape Cloudflare's 1.0 claim.
5. **The "trusted, delight" goal still lacks its evidence**: no outside-reader pass yet (P6, P21).
6. **The early record is not in git** (section 0). Every claim about August 2026 rests on documents that quote a commit this repository cannot show.
