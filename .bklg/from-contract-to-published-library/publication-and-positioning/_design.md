---
item: HS-P0016
stage: design
created: "2026-08-12"
updated: "2026-08-12"
---

# API surface design — 0.2.0 — where private opinions become promises

The resolved **public API surface** for this project, signed off by a human before any story spec is
written. Everything below is binding on the implementer.

**Read this one differently from its siblings.** This project ships almost no Rust API. Its designed
surface is three *rendered pages* — the crates.io front page, the docs.rs page, and the repository
landing page — and the template's own framing is what makes that the right use of this stage: *"the
surface a `cargo add happenstance` user meets is this repository's screen."* For this project that
sentence is literal rather than analogical, so this file carries both halves: the small API surface
in the template's sections, and the **interaction design of the rendered pages** in `## Surfaces`
through `## States`. The design review scores the built surface against the second half.

**Two gaps in the substrate, named rather than filled** (`_grounding.md`, and the grounding pass's
own findings):

1. **There is no design-system primitive layer, and one must not be invented.** No CSS tokens, no
   component package, no `package.json` anywhere in the tree — checked. This is correct for a Rust
   library workspace: **crates.io and docs.rs own the theme, in light and dark, and neither honours
   anything we write** (UX brief, *Design-system primitives*). The primitive layer this design
   composes from is the Markdown/rustdoc vocabulary and the prose forms already standing in the tree,
   each cited by path below. Anything hand-rolled either fails to render or fails in one theme, and
   for a published version number that failure is permanent.
2. **There is no `interaction-patterns.md` for this initiative.** Confirmed absent
   (`.bklg/from-contract-to-published-library/_discovery/distillation/` holds `opportunities.md` and
   `personas-and-journeys.md` only), deliberately, at the intake gate — `userFacing` is false for the
   initiative as a whole (`initiative.md`:411). So every pattern decision below is grounded in the
   **prose-form inventory and the IQ-1…IQ-7 invariants already codified in this project's UX brief**
   (`_decomposition.md`, *UX brief*), which is the dossier this project actually has. Citations point
   there, and to `personas-and-journeys.md` for the persona claims underneath them.

**There is no capture, and that is a skip rather than a pass.** `design.capture` is undeclared in
`.redkiln/config.yaml`; there is no app and no route to screenshot. The instrument that replaces it is
named in AC-007 and owned by the `rendered-page-preflight` story: a human reads the **rendered** page
before the irreversible act. `xtask/src/package.rs:4-18` already states why nothing automated can
stand in — `cargo package --list` proves containment, never presentation. Every number in
`## Density budget` is therefore written as a budget **measured at that preflight**, not as a fact
this file verified.

---

## Items

Every public item this project adds, changes or removes. `path` is the full path a caller writes.

This project's API surface is nearly empty **by construction** — `project.md`'s *Out of scope*
reserves any API change for an upstream project and a re-plan, and no `architecture` brief is
warranted (`_decomposition.md`:261). Two entries, and the second is a manifest key rather than an
item; it is listed anyway because it is the single largest determinant of what the docs.rs surface
renders.

```yaml
- path: "happenstance/unstable-projection" # and happenstance-core/unstable-projection
  kind: "feature"
  change: "added" # CONDITIONAL — exists only if AC-012 resolves PS-3 to "ship gated"
  feature: "off-by-default"
  clause: "PS-3"
  decided_by: "projection-port-ship-shape (AC-012), on projection-store-freeze and ladybug-projection-store evidence"
  note: "This design binds the PRESENTATION either way (see Shape decision, PS-3 row); it does not decide PS-3."

- path: "crates/happenstance/Cargo.toml [package.metadata.docs.rs]"
  kind: "manifest" # extends the template's kind vocabulary: not an item, but it decides what docs.rs renders
  change: "added"
  feature: "default"
  clause: "n/a — RS-51-5, standards/rust/51-features-and-no-std.md:188-236"
  note: "Verified ABSENT today. Present at crates/happenstance-core/Cargo.toml:54-56 and crates/happenstance-testkit/Cargo.toml:56-58. The crate a `cargo add` evaluator meets is the one whose docs.rs build is unconfigured."
```

**Nothing is removed and no signature changes.** If `registry-surface-diff` (AC-002) reports
otherwise against the `0.2.0-alpha.1` baseline, that is a finding this file did not anticipate and it
blocks the release for a re-plan — it does not get absorbed here.

## Signatures

The exact signatures, as they will be written.

```rust
// The only Rust-visible change, and only in the PS-3 "ship gated" arm. The item is the SAME item;
// what the feature adds is a gate, and RS-51-5's treatment is what makes the gate render as an
// annotation on docs.rs rather than as an absence (IQ-2).

// crates/happenstance-core/src/lib.rs
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]
pub trait ProjectionStore { /* unchanged */ }
```

```toml
# crates/happenstance/Cargo.toml — the missing block, verbatim from RS-51-5
# (standards/rust/51-features-and-no-std.md:188-236), matching the two manifests that already have it.
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Surfaces

Seven rendered pages. **No DOM selector is nameable for any of them** and none is invented here:
crates.io and docs.rs are third-party renderers this project ships Markdown and rustdoc *into*, and
the repository landing page is GitHub's renderer. `selector` therefore carries the **source of
record** — the file in this tree that the page is rendered from, which is the only thing an
implementer can actually address. `route` is the post-publish URL, which does not resolve until
`publish-0-2-0` lands.

```yaml
- id: crates-io-happenstance
  route: "https://crates.io/crates/happenstance"
  selector: "crates/happenstance/README.md"
  renders_from: "readme = \"README.md\" (crates/happenstance/Cargo.toml:12)"
  pattern: "proof-led identity block"
  primary: true
  states: [first-screen-1440x900, first-screen-1024x768, full-page, images-blocked, light-theme, dark-theme]

- id: crates-io-happenstance-core
  route: "https://crates.io/crates/happenstance-core"
  selector: "crates/happenstance-core/README.md"
  renders_from: "readme = \"README.md\""
  pattern: "proof-led identity block"
  primary: false
  states: [first-screen-1440x900, first-screen-1024x768, full-page, images-blocked, light-theme, dark-theme]

- id: crates-io-happenstance-testkit
  route: "https://crates.io/crates/happenstance-testkit"
  selector: "crates/happenstance-testkit/README.md"
  renders_from: "readme = \"README.md\""
  pattern: "proof-led identity block"
  primary: false
  states: [first-screen-1440x900, first-screen-1024x768, full-page, images-blocked, light-theme, dark-theme]

- id: docs-rs-happenstance
  route: "https://docs.rs/happenstance/0.2.0/happenstance/"
  selector: "crates/happenstance/src/lib.rs (//! module docs)"
  renders_from: "rustdoc under [package.metadata.docs.rs], all-features = true"
  pattern: "module-doc ladder with doc(cfg) gate annotations"
  primary: true
  states: [first-screen-1440x900, first-screen-1024x768, full-page, all-features, light-theme, dark-theme, docs-build-failed]

- id: docs-rs-happenstance-core
  route: "https://docs.rs/happenstance-core/0.2.0/happenstance_core/"
  selector: "crates/happenstance-core/src/lib.rs (//! module docs)"
  renders_from: "rustdoc under [package.metadata.docs.rs]:54-56"
  pattern: "module-doc ladder with doc(cfg) gate annotations"
  primary: false
  states: [first-screen-1440x900, first-screen-1024x768, full-page, all-features, light-theme, dark-theme, docs-build-failed]

- id: docs-rs-happenstance-testkit
  route: "https://docs.rs/happenstance-testkit/0.2.0/happenstance_testkit/"
  selector: "crates/happenstance-testkit/src/lib.rs (//! module docs)"
  renders_from: "rustdoc under [package.metadata.docs.rs]:56-58"
  pattern: "module-doc ladder with doc(cfg) gate annotations"
  primary: false
  states: [first-screen-1440x900, first-screen-1024x768, full-page, all-features, light-theme, dark-theme, docs-build-failed]

- id: github-landing
  route: "https://github.com/Wet-Ink-Corporation/happenstance"
  selector: "README.md"
  renders_from: "GitHub, repo-root README"
  pattern: "contributor landing with the full status table and the full peer statement"
  primary: false
  states: [first-screen-1440x900, first-screen-1024x768, full-page, images-blocked, light-theme, dark-theme, long-label]
```

**The ordering is load-bearing and it is the UX brief's, not a convenience.** The repo-root README is
the **third** surface, not the first: `include_str!` cannot reach outside a package
(`README.md`:131-137), so the root README is compiled by `xtask` and is *not* what ships in the
`.crate`. Copy written only at the root is copy the evaluator arriving from a registry search never
sees. Write `crates/*/README.md` first.

## Pattern decision

One row of reasoning per surface family, then the four owned tensions. Every citation is to this
project's UX brief (`_decomposition.md`, *UX brief*) and to `personas-and-journeys.md`, because
**no interaction-pattern dossier exists for this initiative** and this file must not pretend one does.

### The patterns, per surface family

| Surface family | Pattern chosen | Rejected, and why | Grounding | Known failure mode → mitigation |
| --- | --- | --- | --- | --- |
| crates.io (×3) | **Proof-led identity block**: H1 + one-line identity → status callout → disambiguation triad, then evidence below the fold | *Feature tour first* — rejected: the evaluator's decision is "is this real and is it for me", and a feature list answers neither in the one sitting they have (`personas-and-journeys.md`:333-338). *Badge wall first* — rejected: badges are images, and the page must read correctly with images blocked (UX brief, *Accessibility floor*) | UX brief IQ-1 (0 hops to a claim, ≤ 1 to its evidence); existing forms at `README.md`:11-16 (callout) and `crates/happenstance/README.md`:13-20 (triad) | **Truncation upstream of the page.** crates.io shows only the manifest `description` in search results and the crate card — the README never renders there. Mitigation: the `description` field is treated as the lead claim in miniature and budgeted in `## Density budget`; it is re-read for truth at the publish commit like any other published sentence (IQ-7) |
| docs.rs (×3) | **Module-doc ladder with `doc(cfg)` gate annotations**: one-line summary → `# Status` → `# Using it today` (runnable) → the rest, every gated item shown *with* its gate | *Bare API index* (no module prose) — rejected: the sidebar is an index of names, and a name is not an answer to "is this real". *Prose-only module doc with no runnable example* — rejected by RS-70-1 and by the template's own argument: a runnable example is the one artifact that cannot rot | RS-51-5 (`standards/rust/51-features-and-no-std.md`:188-236); the existing ladder at `crates/happenstance/src/lib.rs`:11-71 | **Silent absence.** A feature-gated item built without `--cfg docsrs` simply is not on the page, which reads as "not supported" — a different and false claim (IQ-2). Mitigation: `all-features = true` on all three manifests (the `happenstance` one is missing today and is in scope) and `doc(cfg)` on every gated public item; the docs build is green under all features before publish (AC-UX-007). Second failure mode: **the docs build fails and the reader lands on a docs.rs error page.** Mitigation: the crates.io README is self-sufficient — no claim on it depends on docs.rs rendering |
| GitHub root | **Contributor landing**: the full 8-row status table, the full peer statement, repo-relative links | *Mirror the packaged README* — rejected: the audiences differ (contributor vs evaluator) and, decisively, **link resolution differs**. Every link on a packaged surface must be absolute; every link on the root must be repo-relative. The tree already knows this — `crates/happenstance/README.md`:48,54 use absolute GitHub URLs while `README.md`:83-90 uses relative paths | `README.md`:79-98, :218-225; UX brief AC-UX-011 | **Dead links across the boundary.** A relative link copied onto a packaged README resolves against crates.io and 404s. Mitigation: it is anti-pattern AP-6, checked mechanically at the publish commit (AC-UX-011), not read |

### DT-1 — which claim leads on first contact

**Options on the table** (`initiative.md`:420): (a) lead with storage-agnostic proof; (b) lead with
the edge story; (c) two entry points, one per audience.

**Resolved: (a), with the proof stated as an act the reader can perform rather than as an adjective.**
The packaged `happenstance` README's first screen leads with *what this is and how you can check it*:
a contract for storage plus a **published** conformance suite that decides who meets it — the claim
`README.md`:3-6 already makes, hardened by the compliance block below the fold that names the
implementations the suite ran against and the date (AC-009).

**(b) the edge story lost** because it narrows the perceived audience to a runtime most evaluators are
not on, and it is a differentiator only *after* the contract claim has been accepted. It also costs
nothing to demote: the constrained-runtime reader can self-identify from **one line** in the
Guarantees block (AC-UX-009), whereas the general evaluator cannot self-identify from an edge lead at
all. The asymmetry decides it.

**(c) two entry points lost, and it lost twice.** Structurally: crates.io renders exactly one README
per crate and there is no route parameter, so "two entry points" in this medium means *two crates'
front pages* — and the crate split ADR-0006 already made is by **role** (application author vs adapter
author), not by runtime. A runtime-shaped second entry point would need a fourth crate or a landing
page nobody navigates to. And on the persona question below, it splits one person in half.

**The persona question this turns on, resolved here because it had to be** (`_decomposition.md`:310-314;
`personas-and-journeys.md`:373-377): **the evaluator is the first fifteen minutes of the application
author's journey, not a fifth persona.** One entry point per *crate role*, which is what the existing
disambiguation triad already encodes. Reason: every property that distinguishes Persona 4 —
time-boxed, one-shot, cannot run the suite — is a property of a *moment*, not of a person; the same
human is an application author twenty minutes later. Persona 4 is also the least-evidenced of the
four (`personas-and-journeys.md`:349-355), which is a reason to fold it rather than enshrine it.

> **Amended 2026-08-12, by the repository owner, at the `/redkiln:plan` spec stage.** The
> conclusion above stands — the evaluator is **not** a fourth persona atom — but the *consequence*
> originally written here said `closeout-and-durable-audience` (BR-16) should promote it as a
> **journey stage** on the application author's atom. It is promoted as **its own journey atom,
> linked to Persona 1**, instead.
>
> Why: `closeout-and-durable-audience`'s DR-10 argued the other way and made one point this
> section never rebutted — the difference is in the *mechanism* of trust-building (one-shot public
> evidence versus revisable contact with the code over weeks), not a difference of degree within
> one journey. That is compatible with everything above: it is still a moment, not a person, and a
> moment with its own mechanism is exactly what a **journey** atom is for. A stage buried inside
> Persona 1's journey would make that mechanism unfindable to the reader it exists to serve, which
> is the failure `.kb/product/` is being populated to prevent.
>
> DR-10's third argument — that folding would retroactively make DT-1's option (c) incoherent — was
> **discounted**: option (c) lost, and this section's own reasoning for rejecting it was partly that
> it splits one person in half. Preserving a rejected option's premise is not a reason to shape the
> durable knowledge base.
>
> Net effect on this project: none. DT-1 is unchanged and no published surface moves. The change is
> to what `closeout-and-durable-audience` promotes, and it is recorded there too
> (`../closeout-and-durable-audience/_decomposition.md`, DR-10).

*Resolves:* DT-1, AC-011, AC-UX-001, U1.

### DT-4 — where the positions-and-gaps promise is stated

**Options** (`initiative.md`:423): (a) on the crate landing page; (b) in the specification only;
(c) both, with the landing page pointing.

**Resolved: (c), in a specified shape** — the promise is **stated in full**, in the caller's register,
inside the **Guarantees** block of `crates/happenstance/README.md`:41-49 and
`crates/happenstance-core/README.md`:43-54 (the UX brief names Guarantees as U3's and U6's slot), as
one bullet of ≤ 3 rendered lines plus one nested line, carrying **exactly one** link into
`spec/SPECIFICATION.md`.

**(b) specification-only lost** on IQ-1's budget: 0 hops to read a claim. The evaluator has one
sitting; a ~5,000-line specification is a context jump they do not return from, and U3 is the beat
**all four** personas share and none currently has anywhere to look
(`personas-and-journeys.md`:339-345).

**(a) landing-only lost** because a promise with no citation is exactly what two DCB-labelled stores
already disagreeing in public can each write (`personas-and-journeys.md`:276-279). The citation *is*
the difference, and it is why BR-14 exists.

**Failure mode of "both": divergence.** Two copies of a promise drift, and the published one cannot be
edited. Three mitigations, all structural rather than procedural:
- **Different registers, so they cannot silently duplicate.** The README states the *consequence for
  the caller* — positions are unique and increasing, **gaps are permitted, and code that assumes
  `n + 1` is wrong against a conformant store**. The specification states the clause. Neither is a
  paraphrase of the other.
- **One link, into a clause ID, not a heading.** `cargo xtask spec-trace` already guards clause IDs
  and `spec/SPECIFICATION.md`:210-211 retains even a demoted ID *"so that citations resolve"*. A
  renamed heading breaks the gate rather than the page.
- **The absence is stated with its reason.** The copy must **not** imply ownership of
  `read_from_a_gap_position`, which `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
  records as named by two accepted decisions and owned by neither. The nested line says so and links
  the open question. This is IQ-2 and RS-40-5 applied to prose: **no surface may state an absence
  without a reason** (`standards/rust/40-public-surface-and-evolution.md`:212).

*Resolves:* DT-4, AC-010, AC-UX-003, U3.

### DT-5 — is the clause maturity vocabulary published

**Options** (`initiative.md`:424): (a) publish the ledger; (b) publish only frozen guarantees;
(c) publish a summary that links the ledger.

**Resolved: (c), as a single dated census sentence that defines all four words inline.** Form, sited
immediately after the status callout on the packaged `happenstance` and `happenstance-core` READMEs:

> As published at `0.2.0` (YYYY-MM-DD), the specification carries 200 clause IDs: **139 frozen**
> (changing one takes a new decision record), **49 provisional** (each carries the falsifier that
> would settle it), **10 deferred** (each names the experiment and the phase that owns it), and **2**
> demoted to prose. → *the ledger*

**(a) publish the ledger lost** on density: 200 rows on the one screen an evaluator reads buries every
other claim on the page — twenty-five times the height of the entire status table, on the surface
whose whole budget is ~14 rendered lines above the fold at 1024×768.

**(b) frozen-only lost outright — it is forbidden, not merely worse.** Listing 139 frozen guarantees
while suppressing the existence of 49 provisional ones is IQ-2's filter hiding what it filters, and it
is the exact dishonesty BR-06 exists against (`spec/SPECIFICATION.md`:213-217).

**This also closes IQ-6's live defect.** `crates/happenstance/README.md`:53-56 already tells a reader
the specification carries *"a maturity marker"* per clause while the vocabulary that word belongs to is
defined nowhere a consumer reaches. The census sentence defines it in the same sentence that uses it.
"Leave it as is" was never an option.

**Failure mode of the summary pattern: staleness.** Four numbers on a page that cannot be edited after
publish. Mitigations: the four numbers are taken from `spec/SPECIFICATION.md`:219-222 **by the AC-003
clause audit at the publish commit**, not typed from memory; the sentence is **dated and scoped to the
version** so it reads as a snapshot rather than a standing claim (IQ-4's wording discipline, the form
of `README.md`:227-234); and **no other sentence on any surface repeats a count**, so a future release
edits one sentence per README rather than N. The count is 49, never 46 — `RUNBOOK.md`:4495's anchor is
the stale artifact (UX brief, *Notes*).

*Resolves:* DT-5, AC-003, AC-011, AC-UX-004, U4.

### DT-6 — explicit comparison to the nearest live peers

**Options** (`initiative.md`:425): (a) state it; (b) let the proof stand alone.

**Resolved: (a) state it** — in full in the existing **Prior art** slot (`README.md`:218-225) on the
GitHub surface, and in a **one-sentence, one-link reduced form** on the packaged `happenstance` README
per AC-UX-005. Not a new section on either.

**(b) lost, and the reason is stronger than "silence reads as an oversight."** The section already
exists and already names both peers honestly, including the sentence that sends a reader *away*
(`README.md`:180: *"if you want DCB on Postgres today, `disintegrate` below is the mature choice"*).
Choosing (b) would mean **deleting existing honest copy** in a release whose thesis is that a claim
ships next to what proves it. That is a strictly worse move than dating what is already there.

**Failure mode of comparison copy: it dates badly and turns adversarial.** The nearest live peer
shipped the day before intake (`initiative.md`:440). Three mitigations, each checkable on a
screenshot:
- **No feature matrix, ever.** Prose only, one sentence per peer. A matrix implies completeness, and a
  row goes stale invisibly; a stale sentence reads as a stale sentence. This is AP-5.
- **Each sentence says what the peer is good at, then what happenstance's *different bet* is — never a
  ranking.** The existing copy is the model, not a thing to rewrite.
- **"As of YYYY-MM-DD", re-read at the publish commit rather than at decision time** (AC-UX-005), and
  re-read again at every subsequent release because it lives in the section the release checklist
  already points at.

*Resolves:* DT-6, AC-011, AC-UX-005.

## Composition

Where each region sits **relative to the others**. Regions are numbered in render order, top to
bottom. `[FS]` marks regions inside the first-screen budget; everything else is on the same page,
reached by scrolling — which is not a hop.

### `crates-io-happenstance` (the primary surface)

| # | Region | Occupant | Shares space with |
| --- | --- | --- | --- |
| R1 `[FS]` | **Identity** | `# happenstance` + one-line what-it-is (DT-1's lead claim, the `description` field's twin) | nothing — full width, alone |
| R2 `[FS]` | **State** | The status callout, blockquote, dated, true of the tree that shipped | directly under R1, no heading between them |
| R3 `[FS]` | **Audience** | *"Which crate do I want?"* triad — application author → this crate; adapter author → `happenstance-core`; measuring an adapter → `happenstance-testkit` | directly under R2 |
| R4 | **Maturity census** | DT-5's one dated sentence, four counts, four definitions, one link | immediately below the fold at 1024×768; above it at 1440×900 |
| R5 | **Badges** | CI + licence, decoration only | **below R4, not under the H1** — see *Density budget* |
| R6 | **Compliance claim** | AC-009's claim-with-evidence block: "DCB-compliant", the implementations the suite ran against, the date, one link | its own `##` heading; the first heading a scrolling reader meets |
| R7 | **Quick start** | The `toml` fence + the `rust` fence that is this crate's own doctest (`crates/happenstance/src/lib.rs`:10) and the same text `stranger-install-smoke` runs | directly under R6, because R6 earns the try |
| R8 | **What DCB buys you** | The existing explanatory prose (`crates/happenstance/README.md`:22-39) | under R7 |
| R9 | **Guarantees** | `forbid(unsafe_code)`; feature forwarding; **DT-4's positions-and-gaps promise**; the MSRV promise + minor-bump-is-breaking (AC-UX-010); the `wasm32` self-identification line (AC-UX-009) | one bulleted list, ≤ 7 bullets |
| R10 | **Prior art pointer** | DT-6's one sentence + one link to the root Prior art section | under R9 |
| R11 | **Design** | The spec link, rewritten so it no longer uses "maturity marker" as orphan vocabulary (R4 now defines it) | under R10 |
| R12 | **Licence** | MIT OR Apache-2.0 | last |

**The three decisions in that table that no brief made, stated plainly.** (1) Badges move **below**
the census, not under the H1 — they are the only first-screen region whose entire content is images,
and the first screen is 14 rendered lines at 1024×768. (2) The compliance block sits **above** the
quick start: the evaluator's order is *is it real → do I try it*, not the reverse. (3) The **8-row
status table does not appear on any packaged surface at all** — it is one hop away on the GitHub
landing page. It costs more than the entire first-screen budget, its content is repo-shaped (paths
into `crates/`), and its links would be relative on a surface where relative links die. The
disambiguation triad is what carries "which crate" on crates.io; the table is what carries "how far
along is each crate" on GitHub.

### `crates-io-happenstance-core` and `crates-io-happenstance-testkit`

Same skeleton, two deliberate differences. **`happenstance-core`**: R3's triad points *away* to
`happenstance` for application authors — that is its job, and it already does it
(`crates/happenstance-core/README.md`:14-22). It carries R4's census (it is the crate whose contract
the clauses describe) and DT-4's promise in its own Guarantees block. **`happenstance-testkit`**:
R2's callout is already sharper than a census (`crates/happenstance-testkit/README.md`:9-22 states
both what changed and what is still early), so R4 is **demoted below the fold** there rather than
repeated in the first screen — the callout already carries the maturity signal in a form specific to
the suite. R6's compliance block is the same text on all three, because it is the same claim.

### `docs-rs-*`

The module doc is a ladder, and the sidebar is generated from its `#` headings — so heading choice is
layout. Order: one-line summary (1–2 lines) → `# Status` → `# Using it today` (the runnable example)
→ everything else. The first `#` heading must appear within 12 rendered lines so the sidebar's section
list is meaningful rather than a single entry. Gated items render **with** their `doc(cfg)` pill,
never absent.

### `github-landing`

Unchanged skeleton (`README.md`), four edits: badges stay at the top (contributor audience, no crate
metadata block pushing content down), the status callout and status table are corrected to the tree
that shipped, the Quick start's version and "does not resolve yet" copy is corrected, and the Prior
art section gains its date.

## Transience policy

Every control gets a policy and a reason. The three axes translate to this medium exactly once you
accept the UX brief's framing that the reader's "interaction" is *navigating a claim to its evidence
and back inside one sitting*:

- **persistent chrome** → in the first screen of the surface, before any scroll
- **revealed on scroll** (this medium's revealed-on-hover/focus) → on the same page, 0 hops, below the
  fold
- **opened on demand** → one hop, behind a link

| Control | Policy | Reason |
| --- | --- | --- |
| H1 + one-line identity | **persistent chrome**, all 7 surfaces | It is the only region that answers "what am I looking at". Nothing else may occupy line 1 |
| Status callout | **persistent chrome**, all 7 surfaces | U8 and IQ-7: the honest statement of where this is not finished must not be reachable only by scrolling past the good news |
| Disambiguation triad | **persistent chrome** on the 3 crates.io surfaces; **revealed on scroll** on GitHub | On crates.io it is the "is this for me" answer and there is no other. On GitHub the status table does that job and the audience is already oriented |
| Maturity census (DT-5) | **revealed on scroll** at 1024×768, **persistent chrome** at 1440×900, on `happenstance` and `happenstance-core`; **revealed on scroll** always on `happenstance-testkit` | It is the first region whose absence from the first screen costs a *scroll* rather than a *claim*, which is precisely why it is the one that yields (see *Density budget*) |
| The 200-clause ledger | **opened on demand** | 200 rows cannot be persistent anything. Inlining it is DT-5 option (a), which lost |
| Compliance claim block | **revealed on scroll** on the 3 crates.io surfaces | It is 6 lines and it is read deliberately, not glanced at. IQ-1 is satisfied — a scroll is not a hop |
| The compliance evidence (report, ADR-0010) | **opened on demand**, exactly 1 hop, landing on a specific anchor | IQ-1's ≤ 1 hop, and IQ-5: the evidence must be a reachable artefact, never "run our CI" |
| Quick-start fence | **revealed on scroll** | 15+ lines cannot be glanced at, and the decision it serves ("will I try this") comes *after* the identity decision |
| Positions-and-gaps promise (DT-4) | **revealed on scroll**, inside Guarantees | It is a promise a reader seeks deliberately once they are past "is this real". The clause is 1 hop |
| The `read_from_a_gap_position` non-ownership | **revealed on scroll**, nested under the promise it qualifies | RS-40-5 / IQ-2: an absence ships with its reason attached, in the same place as the thing it qualifies — never on a separate page |
| MSRV promise | **revealed on scroll**, in Guarantees; the atom is 1 hop | U6 is a cost question, asked after interest exists |
| `wasm32` self-identification | **revealed on scroll**, one line in Guarantees | DT-1's loser: it is one line for the reader who is looking for it, not the lead for the reader who is not |
| Peer statement (DT-6) | **opened on demand** from the packaged surfaces (1 sentence + link); **revealed on scroll** on GitHub | Exactly one full copy exists, so exactly one thing goes stale. That is the whole maintenance mitigation |
| Badges | **persistent but non-load-bearing** on GitHub; **revealed on scroll** on crates.io | They are images. The page must read correctly with images blocked, so nothing may depend on them; and on crates.io they cost first-screen budget the triad needs |
| Status table (8 rows) | **opened on demand** from every packaged surface (1 hop to GitHub); **revealed on scroll** on GitHub | It exceeds the entire first-screen budget, and its relative links do not survive the packaging boundary |

**Nothing on this list is persistent because nobody decided otherwise.** Three regions were actively
demoted from the first screen — the badges, the census at 1024×768, and the status table off the
packaged surfaces entirely — and each demotion has a stated cost and a stated reason above.

## Density budget

Numbers, and the method that makes them falsifiable. **Every figure below is a budget checked at the
`rendered-page-preflight` (AC-007), not a measurement this file performed** — no capture harness
exists (`design.capture` undeclared) and `xtask/src/package.rs`:4-18 is explicit that containment is
not presentation. A budget that is missed is a finding for that story, not a silent pass.

### Rendered area, per surface, per viewport

"Rendered lines" = 16px body text at ~1.5 line-height ≈ 24 CSS px per line; blank lines count.

| Surface | 1440×900 | 1024×768 |
| --- | --- | --- |
| crates.io | content column ≈ 800 px wide (~95 chars/line). Above the fold after browser + site + crate-metadata chrome: ≈ 500 px ≈ **21 rendered lines** | metadata stacks above the README: ≈ 340 px ≈ **14 rendered lines** ← **the binding constraint** |
| docs.rs | content ≈ 850 px beside a ~200 px sidebar. Above the fold ≈ 700 px ≈ **29 rendered lines** | sidebar collapses to a top bar; ≈ 550 px ≈ **22 rendered lines** |
| GitHub | content ≈ 900 px. Above the fold ≈ 570 px ≈ **24 rendered lines** | ≈ 418 px ≈ **17 rendered lines** |

**The design is dimensioned against 14, not 21.** First-screen allocation on
`crates-io-happenstance`: identity 3 + blank 1 + callout 3 + blank 1 + triad 6 = **14**. That is the
whole budget, exactly, and it is why the badges and the census sit below it.

### Per-item budgets

| Item | Budget | Rationale |
| --- | --- | --- |
| `description` (Cargo.toml) | **≤ 120 characters**, one sentence, true standalone | It is the *only* text crates.io shows in a search result and in the crate card; the README never renders there. Today's is ~131 chars and claims typed events, decision models and projection runners — **re-read it for truth at the publish commit** (IQ-7), because it is a published sentence like any other |
| Identity line | ≤ 2 rendered lines | Leaves 12 of 14 for callout + triad |
| Status callout | ≤ 3 rendered lines | 4 lines pushes the triad's last bullet off the 1024×768 fold |
| Disambiguation triad | ≤ 6 rendered lines including its `##` heading; ≤ 3 entries | Three entries × 1–2 lines. A fourth entry means a fourth published crate, which AC-DEP-001 has decided against |
| Census sentence (DT-5) | ≤ 3 rendered lines; exactly 4 counts, 4 inline definitions, 1 link | Definitions may not move to the link target — IQ-6 forbids orphan vocabulary |
| Compliance block (AC-009) | ≤ 6 rendered lines: claim 1, implementations 1–2, date 1, link 1 | Longer and it stops reading as a single checkable claim |
| Guarantees list | **≤ 7 bullets**, each ≤ 3 rendered lines, **exactly 1 link per bullet** | It absorbs DT-4, the MSRV promise and the `wasm32` line on top of the 3 bullets it already has (`crates/happenstance/README.md`:41-49). Seven is the ceiling before it stops being scannable |
| Positions-and-gaps bullet | ≤ 3 lines + 1 nested line for the non-ownership | The nested line is structurally subordinate so it cannot be read as the promise |
| Quick-start fence | **≤ 20 source lines** | 20 × ~21 px mono ≈ 420 px: one screen at 1440×900 with no inner scrollbar. Today's is 9 (`crates/happenstance/README.md`:30-39); the root's is 18 |
| Status table (GitHub only) | ≤ 8 rows, 3 columns; Role cell ≤ 90 chars; Status cell = glyph **+ 2–5 words** | 4 columns wrap at 1024×768 and the Crate column starves first |
| Peer statement, packaged surfaces | 1 sentence, ≤ 2 rendered lines, 1 link | AP-5: prose only, never a matrix |
| docs.rs lead paragraph | ≤ 2 rendered lines; first `#` heading within 12 rendered lines | The sidebar is generated from the headings; a doc whose first heading is 40 lines down has a one-entry sidebar |

### What yields first when the budget is exceeded

Nothing on this medium is truncated — the reader scrolls — so **"yields" means loses first-screen
position**, and the order is fixed:

1. **Badges** — removed from the first screen outright. They are the only region whose content is
   entirely decoration, and with images blocked they render as nothing anyway.
2. **The maturity census** — moves below the disambiguation triad. It is 0 hops either way, and its
   definitions may **never** be dropped to make it fit (IQ-6).
3. **The identity line compresses from two sentences to one** — the second sentence goes to R8.
4. **The status callout and the disambiguation triad never yield.** If those three moves are not
   enough, something new was added that should not have been; delete the addition rather than the
   callout.

### Minimum legible size for the primary label

We do not own the CSS, so the rule is stated as a prohibition rather than a value. The primary label
is the **H1 rendered by the host at the host's own base size** — crates.io ≈ 28 px, docs.rs ≈ 24 px,
GitHub 2em ≈ 32 px. Therefore: **no primary label may be carried by anything smaller than the host's
16 px body text**, and specifically no `<small>`, no `<sub>`/`<sup>`, no raw HTML at all, no image, and
no badge. A badge glyph renders at ~20 px, and at zero when images are blocked. Every heading level is
the host's; nothing on these surfaces sets a size.

## Hierarchy

Per region: primary / secondary / recessive, and **what carries the distinction** — remembering that
the carriers available are heading level, vertical position, blockquote, bold, table, code fence and
link, and that colour is not one of them.

| Region | Rank | Carried by |
| --- | --- | --- |
| Identity (R1) | **Primary** | `#` H1 — the largest type on the page, set by the host. Position: line 1. Nothing else may use `#` |
| Status callout (R2) | **Primary** | Blockquote + leading **bold** phrase (`README.md`:11's form). It is the only blockquote in the first screen, so the indent rail *is* the signal — and it survives with images off and in both themes |
| Disambiguation triad (R3) | **Primary** | `##` heading phrased as the reader's own question, plus a bulleted list with the **audience in bold** as the bullet's first words — the reader scans for themselves, not for us |
| Compliance block (R6) | **Primary below the fold** | `##` heading + the claim as the first sentence + the date on its own line. It is the only block on the page whose four parts (claim / implementations / date / link) are all mandatory |
| Maturity census (R4) | **Secondary** | Prose paragraph, no heading, bold **only** on the four numbers. It must not compete with R2 — one blockquote per first screen |
| Quick start (R7) | **Secondary** | `##` heading + fenced code. The fence's monospace block is loud enough that it needs no further emphasis |
| Guarantees (R9) | **Secondary** | `##` heading + flat bulleted list. **No nested bullets except the DT-4 non-ownership line**, whose subordination is the point |
| Status table (GitHub) | **Secondary** | Table with a header row; the Status column carries glyph **and** words — never glyph alone (`README.md`:83-90 already complies; preserve it) |
| Prior art / peer statement (R10) | **Recessive** | Plain prose under a `##`, no bold, no table, near the bottom. Recessive **by design**: it is honest to include and dangerous to elevate (DT-6's failure mode) |
| Badges (R5) | **Recessive** | Images, below the census, meaning duplicated in text. Their rank is *decoration* — the page must be complete without them |
| Design / spec link (R11), Licence (R12) | **Recessive** | Short prose under `##`, last. One link each |

**One rule across all three surface families:** at most **one** primary-ranked element per screenful.
The first screen's primary is the identity block; the compliance block is the primary of the screen it
lands in. A screen with two things shouting has none.

## Shape decision

Per item: the shape chosen, the alternatives rejected, and why each lost.

| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |
| --- | --- | --- | --- | --- |
| `unstable-projection` (PS-3) | **Presentation is bound here, the verdict is not.** Whichever arm AC-012 picks, the port renders on docs.rs *with its gate*: `#![cfg_attr(docsrs, feature(doc_cfg))]` + `#[cfg_attr(docsrs, doc(cfg(…)))]`, `all-features = true` | *Ship gated without `doc(cfg)`* — rejected: the item vanishes from the page, which reads as "not supported" and is a **false** claim (IQ-2). *Decide PS-3 here* — rejected: it is AC-012's, on `projection-store-freeze` and `ladybug-projection-store` evidence, and this stage may not pre-empt it | RS-51-5 (`standards/rust/51-features-and-no-std.md`:188-236); `spec/SPECIFICATION.md`:4777-4784; `_storymap.md`:110-114 | PS-3's *presentation*; AC-UX-007 |
| `crates/happenstance/Cargo.toml` docs.rs block | Add it, byte-identical to the two manifests that have it | *Leave it and rely on default docs.rs behaviour* — rejected: without `--cfg docsrs` the `doc(cfg)` attributes are inert, so gated items render with no gate; without `all-features` they do not render at all | Verified absent by grep; present at `crates/happenstance-core/Cargo.toml`:54-56 and `crates/happenstance-testkit/Cargo.toml`:56-58 | AC-UX-007 |
| The quick-start snippet | It **is** `crates/happenstance/README.md`'s fence, compiled as that crate's own doctest via `crates/happenstance/src/lib.rs`:10, and it **is** the text `stranger-install-smoke` runs | *A separate `examples/` file* — rejected: the reader copies what they see, and a second text is a second thing to keep true. *A `rust,ignore` fence* — rejected outright: RS-62-5's out-of-package pattern exists precisely so a packaged example need not be `ignore`d | `standards/rust/62-doctests-and-harnesses.md`:230; `README.md`:131-137 | AC-UX-012, AC-008 |
| The `description` field | Treated as a designed surface with a 120-char budget and an IQ-7 truth re-read | *Treat it as metadata* — rejected: it is the only text in a crates.io search result, so it is the lead claim for every reader who never reaches the README | `crates/happenstance/Cargo.toml`:3 | DT-1's truncation mitigation |

**Provisional clauses carried into implementation:** none by this file. PS-3 is explicitly *not*
resolved here — this design binds only how its outcome renders, either way. That distinction is the
point: a clause carried into implementation still provisional is how a port gets frozen by accident,
and the reverse mistake — a design stage quietly settling a clause a sibling project owns — is the
same failure with the arrow reversed.

## Placement and re-export

**The packaging boundary is the placement decision here, and it behaves exactly like coherence.**
`include_str!` resolves against the file tree, and a path reaching outside the package does not exist
inside a packaged `.crate` (`README.md`:131-137). So:

- Copy that must reach a registry reader lives in **`crates/<name>/README.md`**, which is what
  `readme = "README.md"` packages and what `xtask/src/package.rs`:88-94's `REQUIRED_FILES` asserts is
  contained. Copy written only at the repo root is invisible to the evaluator.
- The repo-root `README.md` is compiled by `xtask`, which is never published. It is the third surface.
- Each crate's README is compiled by **that crate** (`crates/happenstance/src/lib.rs`:10), where the
  relative path stays inside the package. Attaching the root README to `happenstance` would make
  `cargo test` fail for anyone who ran it.

**Link resolution is the second placement rule, and it is absolute.** Every link on a packaged surface
is an **absolute** URL; every link on the GitHub root is **repo-relative**. The tree already splits
this way (`crates/happenstance/README.md`:48,54 vs `README.md`:83-90). Copying a region across the
boundary without rewriting its links produces a dead link on a page that can never be edited — which
is why AC-UX-011 checks it mechanically rather than by reading.

**Nothing is re-exported into a prelude and nothing moves modules.** `pub use happenstance_core::*;`
(`crates/happenstance/src/lib.rs`:76) is unchanged, and `#![doc(html_no_source)]`
(`:73`) is an existing deliberate choice about what the docs.rs page offers — **do not flip it in
passing.**

## Visibility and stability

| Item | Visibility | `#[non_exhaustive]` / sealed | Feature | Semver promise |
| --- | --- | --- | --- | --- |
| `ProjectionStore` (PS-3, gated arm) | `pub` | unchanged by this project | `unstable-projection`, **off by default** | **Documented exemption from semver** while gated — stated on the item, not only in an ADR. If AC-012 freezes it instead, the exemption text does not ship |
| `unstable-projection` feature name | public feature | n/a | off by default | Under 0.x the **minor bump is the breaking-change signal**; the feature's removal at stabilisation is a minor bump like any other |
| Everything else `happenstance` re-exports | `pub`, unchanged | unchanged | `default = ["std", "memory"]`, every feature forwarded from `happenstance-core` | `0.2.0`; minor-bump-is-breaking under 0.x. The facade defines no feature of its own, so the two crates cannot disagree about `default-features = false` (`crates/happenstance/Cargo.toml`:14-26) |

**The stability promise itself is a designed surface object, not just a policy.** AC-UX-010 requires
it to be *stated where a consumer reads it* — the Guarantees block — in three parts: the MSRV floor as
a promise, what an MSRV bump means to a consumer, and that under 0.x the minor bump is the
breaking-change signal. Each links the **new** atom AC-006 authors. It does **not** edit
`.kb/decisions/0004-edition-and-msrv.md` or `0029-msrv-raised-to-1-97-1.md`; both are accepted and
immutable, and `redkiln validate --kb` checks accepted atoms against `HEAD`.

## What it costs a caller

- **The docs.rs manifest block costs a caller nothing** — it is build metadata for one renderer.
- **The `unstable-projection` gate, if PS-3 takes that arm, costs a caller one line in their manifest
  and buys them a stated exemption from semver.** The cost of the *other* arm is larger and less
  visible: freezing a port whose conformance suite does not exist yet.
- **Both port flavours are unaffected.** Nothing in this project adds a bound, so the `!Send` flavour
  and the `Send` flavour cost identically — which is the only acceptable answer under ADR-0001. AC-014
  asserts this rather than assuming it: the four mandatory `wasm32` steps run against the **published**
  tree and the **published** feature set.
- **The MSRV becomes a promise at this release**, which is the one real new cost. 1.97.1 is the floor;
  it equals `rust-toolchain.toml`'s pin today, and ADR-0004's `provisional` marker comes off at first
  publish by its own Policy section. A consumer on an older toolchain cannot build this release — that
  is now a promise rather than a preference, and it is why AC-006 exists.
- **The prose budget's cost is the honest one to name:** the Guarantees block has 7 bullets and this
  project spends 4 of them. A later release wanting an eighth must demote something, and the demotion
  order is in `## Density budget`.

## What a user meets first

A `cargo add happenstance` reader meets, in this order and nothing else in the first screen: **the H1
and one line saying what this is**, **a dated status callout saying how far along it actually is**, and
**a three-way "which crate do I want"**. That is the whole first screen at 1024×768, and AC-UX-001 is
the bar it clears: a reader who stops there can state what the library is and who it is for.

**What is deliberately *not* on the first screen**, each with the region that carries it instead: the
compliance evidence (R6 — it is read, not glanced at), the quick start (R7 — the try decision comes
after the identity decision), the maturity census (R4 at 1024×768 — it is the first thing that yields),
the badges (R5 — decoration), the positions-and-gaps promise (R9 — sought deliberately), the peer
comparison (R10, and one hop from the packaged surfaces — recessive by design), and the 8-row status
table (GitHub only — it costs more than the entire first-screen budget).

On docs.rs the reader meets the one-line summary, `# Status`, and the runnable example, in that order —
and the runnable example is the same one on the crates.io page, which is the same one the stranger
install runs.

## States

Six states, in this medium's terms, plus two the medium adds.

| State | What it renders |
| --- | --- |
| **Empty** — a crate with nothing yet to claim (`happenstance`'s typed layer; the testkit's projection suite) | **Words with a reason, never an omission.** The status callout states the absence *and* why it is absent, in the form of `crates/happenstance-testkit/README.md`:9-22. RS-40-5 is the mechanism this borrows: a declined capability's constructor rejects an empty reason (`standards/rust/40-public-surface-and-evolution.md`:212), so **no surface here may state an absence without one** |
| **Loading** — docs.rs build queued or failed | The reader lands on a docs.rs error page carrying our crate name. Mitigations: the docs build is green under all features *before* publish (AC-UX-007), and **the crates.io README is self-sufficient** — no claim on it depends on docs.rs rendering. A page whose evidence link is a docs.rs URL that does not build has no evidence |
| **Error** — a dead link, a dead intra-document anchor, a 404 | It renders as **nothing visible**, which is exactly why it is checked mechanically rather than read (AC-UX-011). Two live inbound anchors are known and must survive or be updated in the same change: `README.md`:9 → `#licence`, `README.md`:16 → `#status` (IQ-3). The house precedent is `spec/SPECIFICATION.md`:210-211 — an ID is retained *"so that citations resolve"* |
| **Overflow** — the 8-row status table at 1024×768 | 3 columns, header row, Role cell ≤ 90 chars. At 4 columns the Crate column starves first. The table exists on the GitHub surface only |
| **Long label** — the longest crate name beside the longest role text (`happenstance-testkit` / *"Conformance suite adapters must pass"*) | Wraps inside its cell; the Status cell never wraps because it is glyph + 2–5 words. If a role string cannot fit 90 chars, shorten the role — do not add a column |
| **Narrow viewport** — 1024×768 and below | crates.io stacks the crate metadata above the README, cutting the first screen to ~14 rendered lines; docs.rs collapses its sidebar to a top bar. **The 14-line figure is the number the whole budget is set from**, so narrow is the design case rather than the degraded one |
| **Images blocked** (medium-specific) | Every badge renders as nothing. The page must read correctly: no badge's meaning may exist only in the image, and no diagram, screenshot or animated media appears on a published surface at all |
| **Theme** (medium-specific) | docs.rs renders light, dark and ayu; crates.io renders light and dark. We set no colour, so both are legible only if we add nothing. Any colour-coded meaning fails in at least one theme, permanently, for that version number |

### The states the API must express

The Rust surface itself must make representable: **gated-but-present** (`doc(cfg)`, never absent —
IQ-2), **absent-with-a-stated-reason** (RS-40-5's non-empty reason, which is also the prose rule
above), and **provisional-with-a-named-falsifier** (`spec/SPECIFICATION.md`:213-217; CF-38 already
makes an empty falsifier a build failure). No new enum, no new variant: all three already have their
mechanism, and this project's contribution is to apply the same three to *documentation*, where
nothing enforced them before.

## Anti-patterns

Concrete forbidden moves. Each is phrased so it can be checked against a **screenshot of the rendered
page** by someone who cannot read the code.

- **AP-1** — A crates.io first screen that does not say **who the crate is for**. If the
  "Which crate do I want?" block is not visible without scrolling at 1024×768, this fails.
- **AP-2** — A status cell, badge or marker whose only signal is a **glyph or a colour** (✅ 🔲 🔩)
  with no words in the same cell.
- **AP-3** — **Two different maturity counts visible on one surface**, or any count that is not
  attached to a date and a version. The four numbers appear exactly once per page.
- **AP-4** — A list of **frozen guarantees visible with no mention that 49 clauses are provisional**.
  Filtering that hides what it filters.
- **AP-5** — A **feature-comparison table or matrix naming a peer project**. Prose only, one sentence
  per peer, dated.
- **AP-6** — A link on a crates.io or docs.rs page that renders as a **relative path** or lands on a
  404. (Corollary: a heading renamed from `#status` or `#licence` with no surviving anchor.)
- **AP-7** — Any **raw HTML, inline `style`, JavaScript, custom colour, image carrying meaning, or
  animated media** on a published surface.
- **AP-8** — A docs.rs page where a feature-gated public item appears **without its `doc(cfg)` gate
  annotation**, or is **absent entirely**.
- **AP-9** — A published page containing any of the four known-stale strings: *"Nothing is published
  yet"*, `happenstance = "0.1"` with *"does not resolve yet"*, *"the only way to try this is a git
  dependency"*, or *"this crate is currently a facade… adds nothing yet"*.
- **AP-10** — A **"DCB-compliant" claim on a screen with no date and no named implementations in the
  same block**. The claim and its evidence are one visual unit or they are a defect.
- **AP-11** — The **8-row status table on a packaged crates.io page**. It belongs on GitHub only.
- **AP-12** — A quick-start code fence that **scrolls inside itself** at 1440×900 (> 20 source lines),
  or that differs by one character from the text the stranger-install smoke ran.
- **AP-13** — A positions-and-gaps statement that implies the library **owns
  `read_from_a_gap_position`**, or that states any absence without a reason next to it.
- **AP-14** — **Badges above the status callout** on a packaged crates.io page. They are decoration and
  they cost the first screen's budget.
- **AP-15** — More than **one primary-ranked element in a single screenful** (two blockquotes, or a
  blockquote competing with a table, above the fold).

Standing, and never re-litigated here: no `#[async_trait]`; no `serde` in `happenstance-core`'s
defaults; `read` returns the stream at the top level; generic code binds `EventStore`, not
`SendEventStore`; no `unwrap`/`expect` in library code.

## The doctest

The runnable example a user would copy. It is **the same text three times over**: the fence in
`crates/happenstance/README.md`, the doctest `crates/happenstance/src/lib.rs`:10 compiles from it, and
the program `stranger-install-smoke` runs against the registry version. AC-UX-012 is exactly that
identity, and it is why no separate example file is created.

```rust
use happenstance::{Event, EventStore, MemoryEventStore, Query, ReadOptions, Tags, collect};

async fn define_course() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();

    store
        .append(
            &[Event::new("CourseDefined", &br#"{"capacity":2}"#[..])?
                .with_tags(Tags::from_pairs([("course", "c1")])?)],
            None,
        )
        .await?;

    // Positions are unique and increasing. Gaps are permitted: do not assume `n + 1`.
    let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    assert_eq!(events.len(), 1);
    Ok(())
}
```

19 source lines, inside the 20-line budget. It is a write-then-read cycle because that is what AC-008
requires a stranger to be able to run, and the one comment in it is DT-4's promise stated where a
copier will see it.

**No `compile_fail` case.** Nothing in this project's surface has an arrangement worth pinning with a
compiler error; the arrangements this design forbids are *presentational*, and the instrument for those
is the rendered-page read (AC-007), not the compiler. Saying so explicitly is the point — a
`compile_fail` here would be decorative.

## Mock

| Mock | Path | Built by | Notes |
| --- | --- | --- | --- |
| Static sign-off mock | `.bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html` | design stage | All **seven** surfaces (not only the three packaged READMEs), **51 frames**, self-contained — no network fetch, no webfonts, badges inlined as `data:image/svg+xml` so `images-blocked` is a real state rather than a claim. Judged by the rendered-page discipline, not by a DOM capture: there is no app route and `design.capture` is undeclared |

**Viewports:** `1440x900`, `1024x768` — both drawn at true width for every surface.
**Themes:** `light` (the capture run's list). `dark-theme` is declared on every surface but sits outside that
list; it is drawn **once**, supplementary and labelled as such, on `crates-io-happenstance`, so that the Theme
row in `## States` — *we set no colour, so both are legible* — is falsifiable rather than merely asserted.

**States drawn.** Each declared state at each viewport, except the two that name their own viewport:
`first-screen-1440x900`, `first-screen-1024x768`, `full-page`, `images-blocked` (crates.io ×3 and GitHub),
`all-features` and `docs-build-failed` (docs.rs ×3), `long-label` (GitHub). `light-theme` is the theme axis of
every frame rather than a frame of its own. 50 declared cells + 1 supplementary = 51.

**What the mock composes from, and the gap it confirms.** There is no design-system primitive layer to compose
from — re-verified while building it: no `package.json`, no `*.css`, no token or theme file anywhere in the
tree. So the primitive set is the one `## Surfaces` names: the rendered-Markdown block vocabulary plus rustdoc's
own `doc(cfg)` pill. Where a host emits a **stable class contract** the real one is reproduced —
rustdoc's `.rustdoc / .sidebar / .main-content / .docblock / .stab.portability / .item-table`, and
github-markdown-css's `.markdown-body`. crates.io's DOM classes are build-hashed, so nothing claims to mirror
them; its type scale and geometry are mirrored instead. Every host chrome band is **dimensioned to
`## Density budget`** (1440×900 → 500 px above the fold; 1024×768 → 340 px), which is what puts the fold rule on
each clipped frame where the budget says it falls.

**One finding the mock produced, carried to the approver rather than absorbed here.** Drawn at the real block
metrics a Markdown renderer uses — `margin-bottom: 16px` on every block, an H1 at 35 px, an H2 at 26 px plus its
rule and its 24 px top margin — the five first-screen regions measure **≈343 px against the 340 px budget**. They
fit, but only after each status callout was tightened from three rendered lines to two and each triad bullet to
one line. Either the budget's line model gains a heading surcharge (an H1 ≈ 2.1 lines, an H2 ≈ 2.8 including
margins) or the first screen is understood to be **full** at `0.2.0`: any sixth region, or a callout that grows
back to three lines, pushes the triad's third bullet across the fold and **AP-1** fails. This does not change any
decision above; it says the demotion order in `## Density budget` will be needed sooner than the arithmetic
implies.

**Reference captures the design review compares against**, one per surface per viewport:

| Surface | 1440×900 | 1024×768 |
| --- | --- | --- |
| `crates-io-happenstance` | `.bklg/from-contract-to-published-library/publication-and-positioning/design/reference/crates-io-happenstance@1440x900.png` | `.bklg/from-contract-to-published-library/publication-and-positioning/design/reference/crates-io-happenstance@1024x768.png` |
| `crates-io-happenstance-core` | `…/design/reference/crates-io-happenstance-core@1440x900.png` | `…/design/reference/crates-io-happenstance-core@1024x768.png` |
| `crates-io-happenstance-testkit` | `…/design/reference/crates-io-happenstance-testkit@1440x900.png` | `…/design/reference/crates-io-happenstance-testkit@1024x768.png` |
| `docs-rs-happenstance` | `…/design/reference/docs-rs-happenstance@1440x900.png` | `…/design/reference/docs-rs-happenstance@1024x768.png` |
| `docs-rs-happenstance-core` | `…/design/reference/docs-rs-happenstance-core@1440x900.png` | `…/design/reference/docs-rs-happenstance-core@1024x768.png` |
| `docs-rs-happenstance-testkit` | `…/design/reference/docs-rs-happenstance-testkit@1440x900.png` | `…/design/reference/docs-rs-happenstance-testkit@1024x768.png` |
| `github-landing` | `…/design/reference/github-landing@1440x900.png` | `…/design/reference/github-landing@1024x768.png` |

`…` abbreviates `.bklg/from-contract-to-published-library/publication-and-positioning`. Capture is **manual** at
this stage and stays manual at AC-007: `design.capture` is undeclared, there is no route, and
`xtask/src/package.rs:4-18` is explicit that `cargo package --list` proves containment and never presentation.
These paths are where the rendered-page preflight drops its reads, so the review compares the *published* page
against the frame that was signed off rather than against a description of it.

## Sign-off

**Approved.** Ryan Britton (repository owner), 2026-08-12, at the `/redkiln:plan` design
sign-off gate, having looked at the mock rendered in a browser.

**Conditions: none.** All three pushback items below were accepted as put, and so was the
density finding this mock produced: the 14-line first screen is understood to be **full at
0.2.0**. Drawn at real Markdown block metrics the five regions measure ≈343 px against a
340 px budget, so any sixth region, any callout growing back to three rendered lines, or the
triad's third bullet crossing the fold makes **AP-1** fail. The demotion order already in
`## Density budget` names what yields first; this sign-off accepts that it will be needed
sooner than the 24 px-per-line arithmetic implies.

Three things the approver should push back on if they disagree, because each is a call this file made
that no brief made: **(1)** folding the evaluator into the application author's journey rather than
promoting a fourth persona — it is what makes DT-1's option (c) incoherent, and reversing it reopens
DT-1; **(2)** moving the badges below the maturity census on the packaged surfaces; **(3)** keeping the
8-row status table off every packaged surface, one hop away on GitHub.
