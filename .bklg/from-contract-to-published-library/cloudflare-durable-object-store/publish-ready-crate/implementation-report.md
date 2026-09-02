---
item: "HS-S0059"
stage: implement
created: "2026-08-20"
updated: "2026-08-20"
---

# Implementation Report — happenstance-cloudflare becomes a crate someone can depend on

**Ten of twelve ACs are satisfied. Two are blocked, both on things outside this diff, and
neither is blocked on anything the tree could fix.** AC-010 needs one irreversible upload
under the maintainer's crates.io identity, which `cargo xtask reserve` deliberately
refuses to perform and prints for a human instead; AC-012 needs the `deny.toml`
`async-trait` ban disposed of, which is an accepted open question whose own text names
this story as the one it forces — and which this story's PR boundary, and a review that
already reverted the tempting edit once, both forbid it to settle.

Everything else landed: the crate carries both licence texts and its own README inside the
package directory, `publish = false` is gone and the name is in `PUBLISHABLE` in the same
change, the front page says what the crate is, the README's example is compiled by the
gate, and the conformance claim it makes is tied to the fixture's own constants rather
than to prose.

## TDD Evidence

The RED step is one new host-side integration target,
`crates/happenstance-cloudflare/tests/publish_readiness.rs`, written before any production
change and run against the tree as it stood: **8 of its 9 cases failed, each on the
assertion it exists to make** — no `LICENSE-MIT` inside the crate directory, no `README.md`
to read, `publish = false` still in the manifest, a description still ending *"Not yet
implemented."*, no `readme` key, no `[package.metadata.docs.rs]`, no `cfg(doctest)` include.
The ninth passed on the first run and that is the point of it: AC-011 asks this story to
**verify** that no unimplemented body survived, not to cause it.

Two more detectors are not in that file, because their mechanism is not a `#[test]`: the
README doctest itself, and `cargo xtask package-check`. Both were driven to red on purpose
and are recorded below.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `publish_readiness::both_licence_texts_travel_inside_the_crate_directory`; `cargo xtask package-check`; `git diff --no-index` | RED: `cargo package -p happenstance-cloudflare --list` listed 15 files and none of them a licence; the test panicked on the missing `LICENSE-MIT`. GREEN: both texts copied in, 19 files packaged, and both `git diff --no-index` pairs exit 0 with no output. The test additionally rejects a pointer file — it asserts the MIT warranty disclaimer and the Apache appendix are *in* the bytes |
| AC-002 | `publish_readiness::the_readme_is_beside_the_manifest_and_named_by_it` | RED: no `README.md` inside the package and no `readme` key. GREEN: `crates/happenstance-cloudflare/README.md` written, `readme = "README.md"` stated at `Cargo.toml:20`, and `package --list` shows it inside the artifact |
| AC-003 | `publish_readiness::the_manifest_no_longer_declines_publication`; `cargo xtask package-check` | RED: `publish = false` present. GREEN: deleted, with `"happenstance-cloudflare"` added to `PUBLISHABLE` (`xtask/src/package.rs:95`) in the same change. **Both drift directions were then driven deliberately** — see *Gates* — and each produced EC-001's and EC-002's message verbatim |
| AC-004 | the packaging step's own program, run with a file removed | RED **on purpose, after green**: `LICENSE-APACHE` moved aside, `cargo run -p xtask -- package-check` fails naming the crate and the filename. GREEN once restored. That negative run is the only evidence that the crate is *inside* the check rather than adjacent to it |
| AC-005 | `publish_readiness::the_description_is_the_sentence_the_reservation_promises` | RED: the description ended *"Not yet implemented."*. GREEN: it is now `xtask/src/reserve.rs:93`'s sentence exactly, and the test cross-reads `reserve.rs` so the pair cannot drift silently |
| AC-006 | `publish_readiness::the_front_page_says_what_the_crate_is_and_declares_its_docs_rs_build`; the gate's `documentation` step; a nightly `--cfg docsrs` build | RED: no `[package.metadata.docs.rs]`, and the opening-line assertion failed. GREEN: block declared at `Cargo.toml:159-163`, front page opens on what the crate is. `rg -n "Status: not implemented" crates/happenstance-cloudflare/` was already empty — `every-rule-under-workerd` replaced that header — so this story's work here is the docs.rs declaration and the status section that says what publication the crate *is* ready for |
| AC-007 | `cargo test -p happenstance-cloudflare --doc`; `publish_readiness::the_readme_example_is_compiled_from_inside_the_package` | RED: no `cfg(doctest)` attribute, so the README compiled nowhere. GREEN: `src/lib.rs:19` carries it and the doctest runs — `src\lib.rs - (line 446) - compile ... ok`. **Proved non-vacuous rather than assumed**: substituting `CloudflareEventStore::new(state)` for the real constructor turns that same doctest red with `error[E0308]: mismatched types`, and restoring it turns it green |
| AC-008 | `publish_readiness::no_example_in_the_readme_asserts_on_a_literal_position` | RED: no README to scan. GREEN over the written example, which binds `appended` from `store.append(…)` and compares `replayed.last().map(|event| event.position)` against it. The scanner collapses `[<digits>]` indices first, so `events[0].position` is not mistaken for a literal — a scanner that fired there would be one nobody keeps |
| AC-009 | `publish_readiness::the_conformance_claim_carries_every_qualifier_the_run_produces` | RED: no README. GREEN against the fixture rather than against prose — the test reads `CloudflareFixture`'s own `Capability` constants and its three declared ceilings and requires the README to agree with them, in both directions: a declined capability must appear in the fixture's own words, and the "no `SKIP` line" sentence is *forbidden* the moment any capability is declined |
| AC-010 | — | **Blocked.** `cargo xtask reserve happenstance-cloudflare` and `cargo publish --dry-run` are green; the upload is not done. See *Notes* |
| AC-011 | `publish_readiness::no_unimplemented_body_or_scoped_allow_survives`; `cargo clippy … -D warnings` | GREEN on its first run, and that is the correct result: this story verifies rather than causes it. Every `.rs` file under `src/` is scanned with comment lines excluded, and the workspace denies `clippy::todo` with no scoped allow anywhere in the crate |
| AC-012 | `cargo xtask affected --base main`; `cargo xtask ci` | The story grain is green. The full gate found **one real regression caused by this diff and one pre-existing red**. The regression was mine — see *Gates* — and is fixed. The pre-existing red is the `cargo deny` ban, and it is not this story's to clear |

## Commits

One checkpoint on `initiative/from-contract-to-published-library`, not pushed:

- `feat(cloudflare-durable-object-store): happenstance-cloudflare becomes a crate someone
  can depend on`, carrying the trailer
  `Story: cloudflare-durable-object-store/publish-ready-crate` and this report's body.

The SHA is deliberately not transcribed here, for the reason the sibling stories' reports
give: this file is inside the commit that would name it.
`git log --grep "Story: cloudflare-durable-object-store/publish-ready-crate"` is the lookup
that cannot go stale, and `redkiln record-links` settles `story.md`'s `links.commits`.

| SHA | Subject |
| --- | ------- |
| see `git log --grep "Story: cloudflare-durable-object-store/publish-ready-crate"` | `feat(cloudflare-durable-object-store): happenstance-cloudflare becomes a crate someone can depend on` |

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance-cloudflare/LICENSE-MIT`, `LICENSE-APACHE` | **New.** Byte-identical copies of the repository root's, inside the package directory because Cargo will not follow a path outside it |
| `crates/happenstance-cloudflare/README.md` | **New.** What the crate is and which runtime, before any caveat; a compiled `no_run` usage example; the conformance paragraph with the run's own qualifiers; the declared ceilings and the 2^53 position ceiling; where it runs; the licence |
| `crates/happenstance-cloudflare/Cargo.toml` | `description` corrected to the reservation's sentence; `readme = "README.md"` stated; `publish = false` deleted, with the reason for the pair written where it stood; `[package.metadata.docs.rs]` declared with `default-target = "wasm32-unknown-unknown"` |
| `crates/happenstance-cloudflare/src/lib.rs` | The `cfg(doctest)` README include and its reasoning at the top; the status header now reads *bound, implemented, packaged — and not yet released*; a new status section stating precisely what publication the crate is and is not ready for; **the deferred `host` decision taken** — the module travels, hidden — with the argument and the two alternatives that lost. The four findings, the capability limits, the `Targets` note and `not_send_probe` are untouched |
| `crates/happenstance-cloudflare/tests/publish_readiness.rs` | **New.** Nine host-side cases over the crate's own claims, `#![cfg(not(target_arch = "wasm32"))]` because it reads files |
| `xtask/src/package.rs` | One `PUBLISHABLE` entry, and a comment saying why the pair lands together |
| `RUNBOOK.md` | Phase 9's exit box left **unticked**, with the halves recorded rather than averaged |
| `standards/rust/25-…`, `50-…`, `52-…`, `61-…` | Eight `file:line` citations repointed. Not scope drift — the citations point *into* `crates/happenstance-cloudflare/`, and this diff moved those lines. See *Notes* |

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p happenstance-cloudflare --test publish_readiness` | 9 passed, 0 failed (RED first: 8 failed) |
| `cargo test -p happenstance-cloudflare --doc` | 3 passed, 0 failed, README block among them |
| `cargo run --locked --quiet -p xtask -- package-check` | zero exit; `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit, happenstance-cloudflare` and `happenstance-cloudflare: 19 files packaged, including LICENSE-MIT, LICENSE-APACHE, README.md` |
| `cargo clippy -p happenstance-cloudflare --all-targets --all-features -- -D warnings` | clean |
| `cargo fmt --all --check` | clean |
| `cargo run -p xtask -- lint-constitution` | `27 atoms, all consistent` — **after** the citation repair below |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci` | **Red at one `OPTIONAL` step, green everywhere else.** Transcript below |

**The three negative runs, verbatim.** Each was run deliberately and reverted; they are the
evidence that the checks can fail.

EC-003 / AC-004 — one required file removed:

```text
happenstance-cloudflare: `LICENSE-APACHE` is NOT in the packaged artifact
xtask failed: 1 file(s) promised by manifest metadata but absent from the artifact:
happenstance-cloudflare is missing LICENSE-APACHE. Copy the file into the crate directory
— Cargo will not follow a path outside it.
```

EC-001 — the manifest flag deleted, the `PUBLISHABLE` entry withheld:

```text
xtask failed: the publishable set has drifted from the manifests. Cargo will publish
happenstance-cloudflare but this step does not check it. If the crate was promoted on
purpose, copy LICENSE-MIT, LICENSE-APACHE, README.md into its directory and add its name
to PUBLISHABLE in xtask/src/package.rs; if not, its `publish = false` has gone missing.
```

EC-002 — the entry added, `publish = false` restored:

```text
xtask failed: the publishable set has drifted from the manifests. PUBLISHABLE names
happenstance-cloudflare but Cargo will not publish it — the crate has gained a `publish =
false`, or the name here is stale.
```

**The full `cargo xtask ci` step list, transcribed from the run** and checked name by name
against `wasm_steps()` (`xtask/src/main.rs:1050-1060`), which names **seven** at merge time
— not the four the spec was written against, because `wasm-execution-gate-step` has since
landed two more and the fifth arrived at phase 7. Every one of the seven is green:

```text
=== formatting ===                                   ok
=== clippy (all targets, all features) ===           ok
=== tests ===                                        ok
=== each phase's proof artefacts ===                 ok
=== wasm32 build of the contract crate ===           ok   (wasm_steps 1/7)
=== wasm32 check of the conformance harnesses ===    ok   (wasm_steps 2/7)
=== wasm32 build of the Cloudflare adapter ===       ok   (wasm_steps 3/7)
=== wasm32 build of the Neon adapter ===             ok   (wasm_steps 4/7)
=== wasm32 build of the typed layer ===              ok   (wasm_steps 5/7)
=== wasm32 conformance targets are non-vacuous ===   ok   (wasm_steps 6/7)
=== wasm32 run of the conformance rules ===          ok   (wasm_steps 7/7)
=== WF-11 falsifier probe ===                        ok
=== documentation ===                                ok
=== specification traceability ===                   ok
=== no retired rule is still live ===                ok
=== no conformance rule reads a clock ===            ok
=== no literal position values in the suite ===      ok
=== every conformance rule has a changelog entry === ok
=== every stated rule count matches the suite ===    ok
=== the testkit carries its own version ===          ok
=== happenstance-core names serde/alloc … ===        ok
=== the Rust constitution is internally consistent = ok   (red on the first run; see below)
=== the constitution's examples compile ===          ok
=== documentation (no default features) ===          ok
=== documentation (default features) ===             ok
=== packaged artifacts carry their licences and README === ok
=== feature powerset ===                             ok   (OPTIONAL)
=== wasm32 feature powerset ===                      ok   (OPTIONAL)
=== licences and advisories ===                      FAILED (OPTIONAL) — see below
=== docs.rs configuration (nightly) ===              not reached
```

**The one red, and why it is not this story's.** `cargo deny check` reports
`advisories ok, bans FAILED, licenses ok, sources ok`:

```text
error[banned]: crate 'async-trait = 0.1.91' is explicitly banned
warning[unmatched-wrapper]: direct parent 'worker = 0.8.5' of banned crate
  'async-trait = 0.1.91' was not marked as a wrapper
warning[unmatched-wrapper]: direct parent 'worker-macros = 0.8.5' of banned crate
  'async-trait = 0.1.91' was not marked as a wrapper
```

That is red at the base of this story as well as at its head — this diff adds no
dependency and no feature (NF-003), and `cargo tree -e normal` is unchanged by it. It is
`.kb/open-questions/deny-bans-red-on-the-worker-dependency.md`, accepted and deliberately
open, whose *What forces it* section names this story by name. See *Notes*.

**The regression this diff did cause, and its repair.** The first full run failed at the
REQUIRED step `the Rust constitution is internally consistent`, with eight citations in
four constitution atoms pointing at lines this diff moved — adding nineteen lines at the
top of `src/lib.rs` and about thirty in its middle shifted every cited anchor below them by
51 or 52 lines. The eight were repointed at the anchors they name (`can only ever
*inherit*`, `mod not_send_probe`, `struct Probe`, `fn the_probe_is_not_vacuous`, and so on),
which is the repair the lint is *for*: it matches the quoted text, not the number, so it
can say which citation moved and where it should now point.

## Notes

**AC-010 — the reservation is done up to, and not including, the irreversible step, and
that boundary is deliberate.** The placeholder exists and is verified:

```text
Placeholder for `happenstance-cloudflare` written to:
  …/target/reserve/happenstance-cloudflare

$ cargo publish --manifest-path …/target/reserve/happenstance-cloudflare/Cargo.toml --dry-run
   Packaging happenstance-cloudflare v0.0.0
    Packaged 7 files, 15.8KiB (6.0KiB compressed)
   Verifying happenstance-cloudflare v0.0.0
    Finished `dev` profile [unoptimized + debuginfo] target(s)
   Uploading happenstance-cloudflare v0.0.0
warning: aborting upload due to dry run
```

`target/reserve/happenstance-cloudflare/` holds `Cargo.toml`, `LICENSE-MIT`,
`LICENSE-APACHE`, `README.md` and `src/`; its manifest carries the same description as the
crate (AC-005's other half) and an empty `[workspace]` table, which is the honest
description as well as the mechanical one — it shares nothing with this repository but its
metadata.

EC-005's already-claimed branch does not apply: `cargo search happenstance-cloudflare`
returns nothing, while `cargo search happenstance-` lists `happenstance`,
`happenstance-core`, `happenstance-testkit` and `happenstance-sqlite = "0.0.0"`. The name
is unheld, so the reservation is a first claim rather than a re-claim.

What remains is exactly one command, and `xtask` prints it rather than running it:

```console
$ cargo publish --manifest-path target/reserve/happenstance-cloudflare/Cargo.toml
```

`reserve.rs` says why it stops there — *"Then claim it. This is irreversible — a version
can be yanked, never removed"* (`xtask/src/reserve.rs:180-184`) — and the tool's own help
text is blunter: *"Prints the publish command; never publishes anything itself."* That is a
human-in-the-loop seam the repository designed on purpose, and an automated implementation
run is not the human. The upload happens under the maintainer's crates.io identity, cannot
be undone, and needs no part of this diff to be true, so it is left for them with the
command and the green dry-run transcribed above. **AC-010 stays `satisfied: false` in the
ledger until it is run.**

**AC-012 — the gate is one `OPTIONAL` step short of green, and closing it is a decision
this story is forbidden to take.** Three independent prohibitions, any one of which is
enough:

1. `deny.toml` is outside this story's PR boundary, which names six paths and does not
   include it.
2. The exact edit that would make it green — two `wrappers` entries for `worker` and
   `worker-macros` — was made once by `worker-binding-layer` and **reverted by review**
   (`2ea99fd`), whose message states the reason: *"AC-008's own final conjunct forbids
   exactly that … such an edit deletes the finding it is there to produce."*
3. The disposition is `kb-open-question-worker-async-trait-ban-001`, and every `.kb/` write
   in this project belongs to `adr-0023-and-atom-resolutions` (`_storymap.md:94-102`).

The open question's own text anticipates this exact report: *"publish-ready-crate … cannot
claim a green gate while cargo deny check bans is red, and the project's Definition of Done
says the same."* So this story does what it can do — it makes every other step green, it
runs the full gate rather than the fast one so the claim is measured rather than assumed,
and it hands the decision back with both shapes still available. **Ratify** (a `wrappers`
entry carrying the argument that no happenstance port gains a bound from it) or **refuse**
(leave the ban red and carve out the step deliberately) are both decisions somebody can
point at; what is not available is this story quietly taking the first one to tick its own
box.

`RUNBOOK.md`'s phase-9 exit box is left unticked for the same reason, with both halves
written out beneath it — the manifest flag *is* removed, every `wasm32` step *is* green,
and the sentence says `cargo xtask ci`.

**AC-008's review obligation, discharged rather than delegated.** The README's example
contains no numeral in any position context. `appended` is bound from `store.append(…)`;
the assertion compares `replayed.last().map(|event| event.position)` to `Some(appended)`;
`ReadOptions::new().from(appended)` takes the same binding. The only digits in the file are
the three declared capacity limits, the `2^53` ceiling, the course capacity `2` in the
decision branch, and the identifiers `c1`/`s1` — none of which is a position.

**AC-009's review obligation — "true but narrower than it sounds".** Read the conformance
section against what an evaluator would conclude, sentence by sentence. *The event-store
family runs in full, on the target, inside the gate*: true, and the sentence names the
target and the runner rather than leaving "the gate" to do the work. *It is not `workerd`*:
stated as a heading, not a footnote, and followed by the list of what that leaves
uncovered — no isolate, no eviction, no hibernation, no event loop re-entering the object
mid-`await`, none of the platform's storage ceilings. *The fixture declines nothing*: true
at slice HEAD, and the README says what that channel would report rather than implying
there is nothing it could. *The declared ceilings*: the paragraph beneath the table says in
terms that they are a refusal policy and that no physical wall was located, which is the
one sentence a reader could otherwise get wrong. The residual risk is the reader who takes
"conformance suite passes" to mean "certified on Cloudflare Workers"; the two headings and
the status blockquote are the answer to that, and they are the first and third things on
the page.

**One thing this story deliberately did not do.** The nightly `docs.rs configuration`
`OPTIONAL` step names its crates one by one (`-p happenstance-core -p
happenstance-testkit`) and does not name `happenstance-cloudflare`. Adding it there is a
change to an `xtask` `Step`, which the PR boundary assigns elsewhere, so the configuration
was verified by running that exact command against this crate by hand instead — green,
under `--cfg docsrs -D warnings`, on `wasm32-unknown-unknown`, which is the target the
manifest now declares. A follow-up that adds the crate to that step's list would make the
verification standing rather than a fact about today.
