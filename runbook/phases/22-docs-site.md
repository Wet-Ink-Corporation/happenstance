# Phase 22 — The documentation site

**Goal.** A branded site on GitHub Pages that introduces happenstance and carries
its user documentation. Its guide is `docs/`, rendered from the same bytes the gate
compiles. Its API section is rustdoc with scraped examples. A reader arriving
without a `cargo add` behind them finds their way to the page that answers them.

**Why here.** `HS-I0007` ("From Accurate to Teachable", phase 20) put *"a
marketing site, a hosted service, or documentation addressed to a non-Rust
audience"* out of its own scope, on the ground that its reader has already run
`cargo add`. On 2026-09-29 the owner widened that. The evaluator reads for twenty
minutes before a `cargo add`, and a crate page plus a GitHub tree was all there was
to read. The brand system built for this purpose dressed nothing:
- the claude.ai *happenstance* design system;
- `references/brand/`;
- `assets/brand/`.

This phase is that site. It reuses phase 20's work and reopens none of it. The
narrative tree, its need declarations and its gate checks are the site's content
and its correctness bar, unchanged.

**Decisions it settled, with the owner, 2026-09-29.**
1. **`docs/` stays.** It is the checked narrative tree: `lint_narrative::TREE`
   pins it for the gate's checks, and crate docs cite its pages. The site shell
   lives in `site/`. Pages deploys from an Actions artefact, so no `/docs`
   publishing folder is wanted.
2. **Zola** (one binary, no Node) builds it. The landing page is bespoke Tera
   over the design system's tokens, fonts and components.
3. **Scraped examples on the site's API build only.** docs.rs is unchanged.
4. **Tracked here**, as its own phase, rather than by amending phase 20.

**Proof artefact.** The site deployed from `main`, carrying all of:
- at least one `api/` item showing *Examples found in repository*;
- a guide page rendered from a `docs/` page the same commit's gate compiled;
- a landing-page program lifted from the tutorial by fence index rather than
  pasted.

## Work

- [x] `site/` — Zola config, Tera 2 templates, the design system's components
      vendored (`site/static/css/components.css`), its `tokens.json` read by
      `load_data` so a brand re-sync is copying one file, IBM Plex with its
      licence, two highlight themes from the brand's syntax tokens.
- [x] `cargo xtask site [--serve] [--no-api]` (`xtask/src/site.rs`):
  - renders each `docs/` page into the guide, placed by its declared need, and
    reads that declaration with `lint_pages::declarations`, the one parser;
  - re-aims links: a sibling page becomes a Zola `@/` link, which Zola checks,
    and anything else in the repository becomes a `blob/main` URL;
  - drops rustdoc's hidden lines;
  - lifts the landing page's program from `docs/first-encounter.md`;
  - fails by name on a `docs/` page that is neither in the reading order nor
    excluded from it.
- [x] Scraping: `doc-scrape-examples = true` on the seven `examples/*` targets.
      Verified with `html_no_source` left in place: the scraped snippets link
      into the example crates' own source pages, which exist, so the attribute
      the earlier specs asked not to flip in passing is not flipped.
- [ ] rustdoc logo and favicon on the seven published crates — the badge, from
      `assets/brand/favicon.svg` on `main`, which the public repository serves.
      Built and seen rendering, then **withdrawn**: an attribute block above a
      crate root's items moves every line under it, and `standards/rust/` alone
      cites 19 of those lines by number (`cargo xtask lint-constitution` said so).
      It lands either as a same-line addition to an existing crate attribute —
      if `rustfmt` keeps it on one line — or together with a citation repoint,
      in a change that touches nothing else.
- [x] `.github/workflows/pages.yml` — build on a pull request, deploy from
      `main`; SHA-pinned and read-only at the top level, as `cargo xtask
      lint-workflows` requires. `site/` is inert to `cargo xtask affected`.
- [ ] Repoint the two crate-doc links into `docs/` at the site, **after the
      first deploy** — `crates/happenstance/src/lib.rs`'s front-door sentence and
      `crates/happenstance/src/domain.rs`'s bridge link. Both point at GitHub
      today, which always resolves; a link to a site that is not yet up would
      ship broken on the next docs.rs render. Same sentence, same link text: the
      pointer policy (`xtask/src/pointers.rs`) allows one front door, and a
      second sentence is the defect it names.
- [ ] The first how-to pages in `docs/`, under `standards/pages/`, each
      registered in `xtask/src/narrative.rs` and `site.rs`'s reading order:
      `choose-a-store`, `rebuild-a-read-model`, `retry-a-command`,
      `pass-the-conformance-suite`, `run-on-a-durable-object`. On a follow-up
      branch, after the site merges (`wi-269e5a`).
- [ ] `boundaries-not-aggregates.md`, the prior-model bridge. It argues against
      the DDD aggregate, matching the landing page's second beat (`wi-3c4f18`,
      the owner, 2026-09-29).
- [ ] Search. Zola builds the index; the client library has to be vendored.
- [ ] The owner sets the repository's Pages source to *GitHub Actions*.

## Exit criteria

- [ ] The proof artefact exists at `https://wet-ink-corporation.github.io/happenstance/`.
- [ ] `pages.yml` has been seen to fail on a deliberately broken guide link, not
      only to pass.
- [ ] `cargo xtask ci` green with every new `docs/` page in it.
- [ ] The two crate-doc links repointed, and the phase-20 friction log (HS-P0024)
      told that the site is where its reader now starts.

## The audience, reconciled — a draft, deliberately not promoted

Two initiatives each distilled personas and neither was promoted:
- `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` has
  three;
- `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
  has four.

They agree on three personas and differ by one, so the site is written for their
union: four personas. **None has been observed.** `.kb/product/README.md` keeps
an unevidenced sketch out of the product layer ("promoting it early gives a
guess the standing of a finding"), so they stay here until phase 20's friction
log walks one. The two source drafts carry the evidence; this is the
reconciliation.

| Persona | Goal | Fear | The site's first page for them |
|---|---|---|---|
| **Application author** — has a rule spanning two entities | model the boundary right the first time, in the library's words | a model that compiles, runs and is quietly wrong | `first-encounter` → the worked example → `carry-your-invariant` |
| **Evaluator** — twenty minutes before a `cargo add` | decide whether *storage-agnostic* and *DCB-compliant* are real | adopting, or rejecting, on a first impression that does not generalise | the landing page, then `reference`: each claim beside what checks it |
| **Edge developer** — a `!Send` runtime, `wasm32`, a Durable Object | event-source on the edge without hand-building the layer | building on a niche that gets orphaned, as Thalo was | `reference#where-it-runs`; a Durable Object how-to is owed |
| **Adapter author** — their storage, this contract | know why the port is shaped so, to tell "the port is wrong for me" from "I have not understood it" | a wall that could be their misunderstanding or the port's limit | `adapter-reading-order`, then the conformance suite |

**Not a reader of this site:** someone who knows neither event sourcing nor DCB.
HS-I0007 settled that on evidence, and the site's copy assumes event-sourcing
vocabulary throughout.

## The narrative spine

One domain carries the site — the canonical worked example's: a course holds
thirty seats, a student takes at most five courses. The landing page runs it in
three beats and then proves it:

1. **The rule** — one decision, two limits, about two different things.
2. **The answer you know** — choose a boundary when the schema is written
   (`Course` or `Student`), and pay a read model and a saga for the other limit.
   Named once and honestly.
3. **The answer here** — read exactly what this decision rests on, and append on
   condition that nothing matching that read has appeared since.
4. **Proof, not claim** — the tutorial's refusal program, lifted by fence index,
   with what it prints; then the conformance suite, the specification, and the
   gate that compiles the guide.

Voice follows the design system's brand book and `SD-0001`:
- lowercase `happenstance`, and no sentence that opens with it;
- the *chance* reading named and displaced, once, where the name is explained;
- no chance imagery anywhere;
- candid status in a note;
- every claim beside the instrument that checks it.

## Session log

- **2026-09-29** — *uncommitted, `lane/docs-site`.* Phase opened with the owner.
  Built the shell, the assembly and the workflow. Seen in a browser at desktop
  width in both themes, and at 375 px in Night; five pages measured at 375 px
  with no horizontal scroll. Scraped examples verified on
  `happenstance::read_decision_model`. Two things found on
  the way:
  - Zola 0.23 runs Tera 2, which has no macros; the templates use components.
  - The design system's `bundle.css` references `var(--space-3.5)`, which is
    invalid CSS; the vendored copy escapes it and says so.
