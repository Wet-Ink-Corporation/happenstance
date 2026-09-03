# The MSRV becomes a promise at `0.2.0`, and the number does not move

**Decided 2026-09-03.** Staged for `/redkiln:kb-ingest` as an accepted decision atom, ADR-0037 —
the next free number, with `0035` and `0036` on disk. `.kb/decisions/` atoms are authored by ingest
and never by hand, so this file is the input and not the record.

**This is an amendment lineage, not a supersession.** ADR-0004 and ADR-0029 stay `status: accepted`
and byte-identical, frontmatter included. Neither gets a `superseded_by` flip. They said what the
number is and why it moved; this one says what it now *binds*.

## The decision

**`0.2.0` requires Rust 1.97.1, and that is a promise rather than a preference.**

The number is unchanged — `Cargo.toml`'s `rust-version` and `rust-toolchain.toml` both say 1.97.1
and keep saying it. What changed is its standing. Until this release the floor was a self-imposed
constraint, raisable at zero cost because nothing was published and nobody was bound by it.
`0.2.0` puts four crates on a registry, and from that moment the number is a commitment to someone
who may never build the crate that forced it.

## Which compiler, and why that one

Answered here rather than one hop further on, because the surface that links this atom is allowed
exactly one link and a reader who has spent it should not be sent looking.

**The floor was forced by a dependency's build script, not by this workspace's code.**
`happenstance-sqlite` depends on `rusqlite 0.40`, which pulls `libsqlite3-sys 0.38.1`, whose
**build script** invokes the `cfg_select!` macro. That macro does not exist before Rust 1.88.
Nothing in `happenstance`, `happenstance-core` or `happenstance-testkit` needs anything newer than
the 1.85 this project started on.

**The promised floor is higher than the forcing evidence strictly requires, and that is deliberate.**
`cfg_select!` needs 1.88; the floor says 1.97.1. The exact minimum is not discoverable from any
metadata — `rusqlite`, `libsqlite3-sys`, `sqlx`, `sqlx-core` and `sqlx-postgres` declare **no
`rust-version` at all**, five for five — so finding it would mean bisecting toolchains against a
build script, and the next dependency that declares nothing would invalidate the answer again. The
floor is therefore tied to the toolchain this project is developed and tested on. A number with a
reason beats a number with a bisection behind it that stops being true.

## What a bump costs you

**An MSRV increase is a minor version bump and is named in the changelog.** One statement, not two
facts to compose: if the floor rises, the version's minor component rises with it, and
`CHANGELOG.md` says so in the entry for that release. A patch release will not move the floor.

Pin an exact version and read the changelog at each upgrade. Below `1.0`, cargo already treats a
minor bump as incompatible, so a floor rise cannot reach you without a version change you chose.

## What is actually verified, stated so the promise is not larger than its evidence

There is an `msrv` job in `.github/workflows/ci.yml`. It installs 1.97.1, runs
`cargo hack check --workspace --no-dev-deps --rust-version`, and then runs the whole test suite at
that toolchain with dev-dependencies included — the second half exists because `--no-dev-deps` is
exactly the flag that hides `proptest` and `tokio`, both of which declare `rust-version = "1.85"`
and leave no headroom.

**That job is currently vacuous, and this atom says so rather than implying otherwise.**
`rust-toolchain.toml` pins the same 1.97.1 the job installs, so the job compiles the workspace with
the compiler every other gate step already uses. It cannot fail for an MSRV reason while that is
true, and it therefore proves the workspace builds at 1.97.1 — which was never in doubt — and not
that 1.97.1 is the *minimum*. Nothing in this repository proves the latter.

**The condition under which it starts proving something is: the pin and the floor diverge.** The
day `rust-toolchain.toml` moves ahead of `rust-version`, the job becomes the only step compiling
what a consumer compiles, and every step above it becomes blind to a floor break. It is kept for
that day rather than deleted, and ADR-0029 says the same.

The `--rust-version` half has a second limit worth naming: it reads declared metadata, and the five
database crates above declare none. `cargo hack --rust-version` cannot protect a floor against a
dependency that states no floor, and neither can `resolver = "3"`. Only running the compiler finds
those, which is what the second command in that job does.

## Alternatives, and why each lost

Four were live. Two of them ADR-0029 already rejected as a *preference*, and they are re-weighed
here against a consumer who is now bound.

**1. Keep the floor a preference, and promise only "latest stable".** Cheapest to maintain and
honest about how the project is actually developed. Rejected because it moves the entire cost onto
the consumer: a library that promises nothing about its compiler cannot be depended on by anyone
with a pinned toolchain, which is most people shipping anything. "Latest stable" is not a floor, it
is the absence of one.

**2. Lower the published floor to 1.85 by pinning `rusqlite` back to 0.37.** Would let the three
non-SQLite crates promise the number this project started on. Rejected on cost and direction:
`rusqlite 0.37` is older, and pinning a dependency backwards to protect a floor trades a real
security and maintenance surface for a number — and it would have to be re-fought at every
`rusqlite` release. ADR-0029 rejected it when nothing was published; being published makes the
maintenance burden worse, not better, because the pin then has consumers.

**3. A per-package `rust-version`, so the published crates promise less than the workspace.** The
most tempting of the four, and the one with a real argument: a consumer who installs `happenstance`
alone never links `libsqlite3-sys`, so 1.97.1 charges them for a dependency they do not have.
Rejected for now, deliberately and not by default — it is a real cost, recorded rather than denied.
Four floors across four crates is four numbers to keep true, four ways for the `msrv` job to be
checking the wrong one, and a matrix the single job above does not have. The honest reading is that
this loses on *verification capacity*, not on merit: it should be revisited if the workspace ever
gains the per-package MSRV checking that would make four numbers as trustworthy as one.

**4. A moving window, such as N-2 stable, in place of a fixed floor.** Rejected because a floor
that moves with the calendar is one a consumer cannot plan against: it breaks builds on a date
rather than on an upgrade they chose, and the breakage arrives through no commit of theirs. The
policy in *What a bump costs you* is the opposite property, and is the one worth having.

## What this does not decide

- **The number.** 1.97.1 is unchanged; this atom converts its status, not its value.
- **The shipped wording.** `crates/happenstance/README.md`:49 currently reads *"MSRV 1.97.1, checked
  in CI"*, which claims more than the section above supports. Correcting that surface belongs to
  `guarantees-and-docs-rs-presentation`, and this atom deliberately does not restate the phrase.
- **ADR-0004 and ADR-0029.** Untouched, and this is checked: `redkiln validate --kb` holds accepted
  bodies against `HEAD`, and the wave must show zero changed bytes in either.

## Evidence

- `.kb/decisions/0004-edition-and-msrv.md` — *Policy*: the bump-is-a-minor-bump rule, and *"a
  preference until first publish (phase 12) and a promise to downstream consumers afterward"*
- `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — the forcing evidence, the five crates declaring no
  `rust-version`, and the standing charge *"Phase 12 must revisit the floor at first publish"*
- `.github/workflows/ci.yml` — the `msrv` job, both of its commands
- `Cargo.toml`, `rust-toolchain.toml` — the number, in the two places that carry it
- `crates/happenstance/README.md`:49 — the sentence that overstates the check
