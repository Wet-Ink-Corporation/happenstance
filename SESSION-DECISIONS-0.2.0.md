# Decision log — the `0.2.0` closeout session

Every decision this session took that a reader could reasonably have taken
differently, with the options that were on the table, the evidence that separated
them, and what was chosen. Written as it happened rather than reconstructed.

**This file is a session record, not a knowledge-base atom.** Decisions with
durable consequence are additionally staged as briefs in `.kb/_intake/` for a
later `/redkiln:kb-ingest`, and the ones that bind code get a long-form record in
`references/adr/`. Atoms are never hand-written.

**Scope.** RUNBOOK phases 10b, 11 and 12, to the point of *ready to publish*.
The release itself — `cargo publish` of the five crates, the `v0.2.0` tag, the
GitHub release, the repository visibility flip and the yank of `0.2.0-alpha.1` —
is deliberately not done here and is listed at the foot.

---

## Standing decisions taken by the owner before the work started

| # | Decision | Consequence |
|---|---|---|
| O-1 | The session stops at **publish-ready**. | Two of phase 12's four exit criteria cannot be met and are reported unmet rather than ticked: *crates live / docs.rs green*, and *`cargo-semver-checks` against a published baseline*. |
| O-2 | Phase 11 is **attempted**, installing CMake if `lbug`'s prebuilt path fails. | Phase 11 gates nothing; if the native build cannot be made to work it is left `blocked` with the evidence rather than skipped in silence. |
| O-3 | Neon runs against the **real endpoint**, from `NEON_CONNECTION` in the git-ignored `.env`. | The credential is read from the environment at run time only. It is never committed, never written into a fixture, never echoed into a log or a CI artefact. |
| O-4 | `happenstance-ladybug` is **claimed on crates.io** during the session. | A `0.0.0` placeholder, if a registry token is present; otherwise the placeholder is generated and the publish left to the owner. |

---

## Decisions taken during the work

### D-01 — `main` was red, and the repair edits an accepted decision atom

**Found.** The baseline `cargo xtask ci` failed at *every stated rule count matches
the suite*: `.kb/decisions/0058-…:33` cites
`crates/happenstance-sqlite/src/event_store.rs:1229-1296` and line 1229 is blank.
All three ranges in that sentence are wrong; the lint reports one because the other
two land on non-blank lines and pass the mechanical check while pointing elsewhere.

`HANDOVER.md:20` says the gate was green at `a0a925b`, and it was. The KB intake
wave (`025f300`, merged `6acdf24`) landed afterwards and is the last thing on `main`
before this lane. **Nobody re-ran the gate after the merge.**

**Cause.** The atom inherited the citation from
`.kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md:91`, written on
2026-09-04 against a tree the merge-join read path later moved. `a4616ca` repointed
the fifteen citations that merge moved — in code and docs, and in no `.kb` file,
because **`.kb/_intake` has no citation checker at all** (~1,200 citations,
`REMEDIATION-HANDOVER.md`'s own coverage count). The ingest promoted a stale
citation out of an unchecked directory into a checked one.

**Options.**

| | Option | Verdict |
|---|---|---|
| A | Repoint the three ranges in the atom body. | Violates the letter of *"an accepted decision atom is immutable"*. |
| B | Write a superseding atom carrying the correct citation. | **Does not fix the gate** — the lint reads every atom regardless of status, so the broken one stays red. And it marks a correct, unreversed decision `superseded`, which is a lie about the decision. |
| C | Suppress or narrow the lint. | This is the *"four prose words switched off a check for nineteen frozen clauses"* defect the remediation pass found. Rejected outright. |
| D | Re-run the ingest. | The intake source is cleared; the wave would author a second atom beside the first. |

**Chosen: A**, on the repository's own stated rule. `CLAUDE.md`'s ADR-0006 section
generalises it in terms — ***rewrite the referent, never the reasoning.*** *"A
superseded body records a reversal and stays verbatim; a renamed crate inside a
standing decision is not a reversal at all."* A drifted line number inside a
standing decision is the same shape. Not one word of the reasoning changed.

**And a finding about the guard itself.** `redkiln validate --kb`'s immutability
check compares the **working tree against `HEAD`**: it refuses an *uncommitted*
edit to an accepted atom and passes once that edit is committed. It is a dirty-tree
guard, not a guard against history — measurably weaker than the prose around it
implies, and it cannot distinguish a referent repair from a reversal. Staged for
the KB as an open question rather than treated as a licence.

**Left behind for the owner:** `.kb/_intake` should be inside the citation checker's
scope, or the ingest should re-anchor citations as it promotes them. Either closes
this hole; nothing else does.

---


### D-02 — Ladybug's build cost, measured rather than estimated

Phase 11's exit criteria ask for the build cost to be measured and the CI decision
recorded. Measured first, on this machine (`x86_64-pc-windows-msvc`, rustc 1.97.1),
in a scratch crate outside the workspace so nothing was disturbed:

| Fact | Value |
|---|---|
| CMake needed? | **No.** `lbug` 0.20.3's `build.rs` tries a prebuilt download first and it succeeded, so the cmake-from-source fallback the crate's own `Cargo.toml` NOTE warns about never ran. |
| Prebuilt artefact | `lbug.lib`, **1,444,941,838 bytes — 1.44 GB**, a single static archive, plus `lbug.h` and `lbug.hpp` |
| Rust-side compile | 17 crates, unremarkable |
| Link | **fails**: `LINK : fatal error LNK1181: cannot open input file 'libssl.lib'` |

**OpenSSL is not optional.** `build.rs`'s `link_libraries` calls `link_openssl()`
and then emits `cargo:rustc-link-lib=dylib=libssl` / `=libcrypto` unconditionally
in static mode, and it is called in the dylib branch too. There is no feature or
environment variable that turns it off. `OPENSSL_DIR` is honoured and only adds a
link-search path, so any directory holding `libssl.lib` and `libcrypto.lib` will do.

**The install went wrong in a way worth recording.** `winget install
ShiningLight.OpenSSL.Dev` raised a UAC prompt and sat on it. On an unattended run
nobody clicks it, so the command hung until it was killed — the machine-scope MSI
has no user-scope arm. The no-admin route taken instead is a user-space `vcpkg`
building `openssl:x64-windows-static-md`, which `lbug`'s `build.rs` already looks
for through `vcpkg::find_package("openssl")`.
*(A UAC dialog was left on the desktop by the killed installer and needs dismissing
by hand — the process behind it is gone.)*

**What this already settles about the CI question**, before the adapter exists: a
1.44 GB static archive plus an OpenSSL toolchain cannot sit on the path of
`cargo test --workspace --all-features`, which is gate step 3 and which *links*.
`cargo check` and `cargo clippy` do not link, so the cost lands on exactly one
step — but it lands hard, on every machine and all three CI platforms.

The consequence is a design constraint rather than a scheduling one: **the `lbug`
dependency has to be behind an off-by-default feature**, so the default gate never
links it, and the conformance run is gated the way the Postgres suite is. That is
the same argument the crate's own `Cargo.toml` NOTE makes for why Ladybug is a
separate crate at all, one level further in — and it was reached by measurement
here rather than by preference.

---

## Left for the owner

*(accumulated as the work reveals them)*
