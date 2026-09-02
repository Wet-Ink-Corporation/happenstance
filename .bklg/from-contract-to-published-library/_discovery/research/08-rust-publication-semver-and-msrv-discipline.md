---
title: "rust-publication-semver-and-msrv-discipline — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "Publication is where a Rust library's private opinions become promises it cannot silently withdraw — and the ecosystem's own evidence is that both halves of that promise (semver and MSRV) fail quietly, routinely, and in ways manual review does not catch."
sources: [
  "https://predr.ag/blog/cargo-semver-checks-today-and-in-2023/",
  "https://predr.ag/blog/four-challenges-cargo-semver-checks-has-yet-to-tackle/",
  "https://rust-lang.github.io/rust-project-goals/2026/cargo-semver-checks.html",
  "https://github.com/obi1kenobi/cargo-semver-checks",
  "https://rustprojectprimer.com/checks/semver.html",
  "https://rustprojectprimer.com/releasing/versioning.html",
  "https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/semver.md",
  "https://github.com/rust-lang/api-guidelines/discussions/231",
  "https://rust-lang.github.io/rfcs/3537-msrv-resolver.html",
  "https://rust-lang.github.io/rfcs/2495-min-rust-version.html",
  "https://github.com/clap-rs/clap/discussions/5369",
  "https://rust-lang.github.io/api-guidelines/documentation.html"
]
---

# rust-publication-semver-and-msrv-discipline

## Findings

- **Semver breakage in Rust is common, undetected by review, and not rare.** A
  study cited by cargo-semver-checks' author, scanning over 14,000 crate
  releases, found that more than 3% carried at least one semver violation the
  tool would have caught — and roughly 1–2 times per week an accidental
  breaking change slips into a release of some popular Rust package "despite
  the maintainers' best efforts." The author's framing is pointed: "code that
  violates semver doesn't look wrong," which is exactly why manual review does
  not catch it. (predr.ag, "cargo-semver-checks today and in 2023")
- **The tool exists because "correct on inspection" was never a working
  bar.** cargo-semver-checks catalogues 30+ distinct kinds of semver violation
  it can detect today, against a tracking issue of 60+ known-but-undetected
  ways to break semver — the author calls this "far from an exhaustive list."
  High-profile crates (PyO3, clap-rs, RustCrypto utilities) have all shipped
  accidental breaks that this class of tooling caught. (predr.ag)
- **The Rust ecosystem is moving cargo-semver-checks toward becoming a
  first-party part of `cargo` itself**, with 2026 project-goal work explicitly
  aimed at resolving cross-crate re-export false positives — evidence that the
  ecosystem judges self-discipline insufficient and is investing in mechanized
  gates instead of process. (rust-lang.github.io/rust-project-goals/2026)
- **0.x versioning has its own, different, widely-relied-on contract.** Below
  1.0, cargo treats a minor bump (0.1→0.2) as the breaking-change boundary and
  a patch bump as safe for new features; a `"0.2"` requirement resolves as
  `>=0.2.0, <0.3.0`. This is a real, load-bearing convention distinct from
  post-1.0 semver, not an informal approximation of it — and callers who don't
  know the convention will misread a 0.x version number's severity.
  (rustprojectprimer.com, cargo's own semver.md)
- **MSRV bump vs. semver bump is an unresolved, still-contested ecosystem
  question — not settled trivia.** The rust-lang api-guidelines discussion
  shows genuinely opposed camps: treating an MSRV bump as *breaking* is argued
  to be counterproductive (it forces every downstream crate that wants to
  support old Rust into pinning an old major version, per @joshtriplett), but
  treating it as *non-breaking* creates "super-confusing type errors" at
  coherence boundaries when a public dependency's MSRV silently drifts (per
  @Kixunil), and even patch-level transitive MSRV bumps can silently raise a
  crate's *effective* MSRV with no version signal at all (per @jannic).
  (github.com/rust-lang/api-guidelines/discussions/231)
- **This is not hypothetical for real crates — it produces public friction.**
  clap 4.5.0 raised its MSRV without a major bump and without changelog
  justification; the ensuing discussion contains a downstream user's plain
  verdict ("I think this is a breaking change though, and bumping msrv so
  often causes problems") set directly against the maintainer's exhaustion at
  being asked to justify each bump ("one of the more draining interactions is
  'justifying MSRV bumps'... There is no predictable way for everyone... to
  walk away satisfied"). Both are evidence-backed positions with real costs on
  each side — this is not a solved etiquette question. (github.com/clap-rs/clap
  discussions/5369)
- **MSRV mismatch used to fail silently until you tried to build, and the
  ecosystem had to build new tooling to stop that.** RFC 3537 (the MSRV-aware
  resolver) documents the pre-fix experience concretely: adding one dependency
  on an older toolchain could cascade into resolving seven separately
  incompatible transitive packages by trial and error. Its adoption data is
  the sharper point: of ~61,758 recently published packages at the time, only
  ~13.8% declared a `rust-version` field at all — while crates.io's own
  traffic showed 74% of requests coming from Rust N-2 or older. Declaration
  was rare; usage on old toolchains was common. That gap is exactly where
  builds broke without warning. (rust-lang.github.io/rfcs/3537-msrv-resolver)
- **Publishing metadata is a small, well-defined, checkable set — but it is
  a set, and an incomplete one silently degrades the docs.rs / crates.io
  experience rather than failing loudly.** `license` (SPDX expression),
  `description`, and `readme` (rendered on the crate page) are the
  registry-facing fields; docs.rs builds documentation itself, on nightly, and
  only needs a `documentation` field set if hosted elsewhere. None of these
  are enforced by `cargo publish` beyond `license` and `description` being
  required at all — a crate can publish "successfully" while looking
  unfinished on its own landing page. (rust-lang.github.io/api-guidelines,
  crates.io docs)

## Evidence & citations

- 3%+ of 14,000+ scanned releases had a semver violation cargo-semver-checks
  would catch; 1–2/week accidental breaks in popular crates; "code that
  violates semver doesn't look wrong" — https://predr.ag/blog/cargo-semver-checks-today-and-in-2023/
- 30+ detected violation kinds vs. 60+ known-undetected ways to break semver,
  "far from an exhaustive list"; PyO3/clap-rs/RustCrypto named as real
  incident sources — https://predr.ag/blog/cargo-semver-checks-today-and-in-2023/
  and https://predr.ag/blog/four-challenges-cargo-semver-checks-has-yet-to-tackle/
- cargo team's 2026 goal to merge cargo-semver-checks into cargo itself, work
  on cross-crate re-export false positives —
  https://rust-lang.github.io/rust-project-goals/2026/cargo-semver-checks.html
- 0.x semver convention (minor = breaking boundary below 1.0; `"0.2"` resolves
  `>=0.2.0,<0.3.0`) — https://rustprojectprimer.com/releasing/versioning.html
  and https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/semver.md
- MSRV-as-breaking-change debate, with named, opposed, evidence-backed
  positions (joshtriplett, matklad, BurntSushi vs. Kixunil, jannic) —
  https://github.com/rust-lang/api-guidelines/discussions/231
- clap 4.5.0 MSRV bump without major-version bump or changelog rationale;
  downstream complaint and maintainer's fatigue quote —
  https://github.com/clap-rs/clap/discussions/5369
- MSRV-resolver motivating scenario (cascading transitive downgrades on
  `cargo add clap` at Rust 1.64) and adoption gap (13.8% of packages declare
  `rust-version`; 74% of crates.io traffic is N-2-or-older Rust) —
  https://rust-lang.github.io/rfcs/3537-msrv-resolver.html
- Registry-facing publish metadata (`license`, `description`, `readme`,
  `documentation`) and docs.rs's independent nightly build —
  https://rust-lang.github.io/api-guidelines/documentation.html

## Implications for the idea

This initiative's desired outcome #3 — "a published release whose promises
are real," with every provisional clause audited "against the ledger rather
than against prose" — is not a caution invented for this repository. It is
the documented, load-bearing failure mode of the Rust publishing ecosystem at
large: semver breaks that "don't look wrong" on review, and MSRV
commitments that are either undeclared (the RFC's 13.8% figure) or contested
even when declared (the clap thread, the api-guidelines discussion). Both are
problem-space facts this initiative inherits the moment `happenstance-core`
gets a crates.io page, independent of anything about DCB or event sourcing:

- **The gap between "compiles" and "promised" is the same gap the intake
  brief already names for `todo!()` bodies, one layer up.** A crate that
  publishes without a semver-diff instrument is trusting the same kind of
  unverified good faith the brief rejects for adapters — "an adapter that
  compiles but has not run the conformance suite is not an adapter" has a
  direct publication-side counterpart: a release that has not been diffed
  against its own last published baseline is not a promise, it is an opinion
  with a version number on it. The brief already names this instrument
  (`cargo-semver-checks` against a registry baseline, `RUNBOOK.md:4108-4124`)
  and the community evidence above is why that instrument, not review, is the
  bar the ecosystem itself converged on.
- **MSRV is a promise with an audience, not a build setting.** The clap
  thread shows that even a well-regarded, widely-used crate drew public
  frustration for raising MSRV silently — and the maintainer's own account of
  that friction ("draining," "no predictable way... to walk away satisfied")
  is evidence that there is no neutral default here: every choice (declare
  and justify every bump vs. treat MSRV like any other maintenance detail) has
  a real, felt cost on one side of the relationship. `happenstance`'s current
  MSRV stance is explicitly provisional until phase 12's first publish
  (CLAUDE.md: "it loses the marker at phase 12, when first publish turns the
  MSRV into a promise") — the research above is evidence that this transition
  is not a formality but the exact point at which a project's felt experience
  changes from "raise the floor whenever it's convenient" to "raise the floor
  and answer for it."
- **The 0.x severity convention is a second audience-facing contract that
  is easy to get quietly wrong.** Because a 0.x minor bump *is* the
  ecosystem's breaking-change signal, a project that plans `0.2.0-alpha.1`
  then `0.2.0` (as this brief's desired outcome #3 does) is making a specific,
  externally-legible claim about where the breaking boundary sits between
  those releases — worth treating as a promise with a known reader, not an
  arbitrary label, once anyone outside the workspace can `cargo add` it.
- **Registry-facing completeness (license, README, docs.rs) is a low-cost,
  high-visibility signal that fails silently, not loudly** — `cargo publish`
  does not stop a maintainer from shipping a crate that looks unfinished on
  its own landing page. This is orthogonal to, and easy to overlook next to,
  the harder API-surface and conformance questions this initiative is mostly
  about — worth naming as its own distinct kind of "done" rather than
  assuming it falls out of the harder work.
- **None of the above is settled by writing more code.** The api-guidelines
  discussion is explicitly still open, and the RFC's fix (the MSRV-aware
  resolver) addresses discoverability of mismatch, not the prior question of
  whether an MSRV bump *is* a breaking change. Whatever this initiative
  decides about its own MSRV-bump policy is a decision this project will have
  to make and state for itself — the ecosystem has not made it for anyone.
