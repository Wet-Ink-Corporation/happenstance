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

### D-03 — `PostgresProjectionStore::Batch` could not be what the skeleton declared

**Found by trying to write the bodies.** `type Batch = sqlx::Transaction<'static,
Postgres>` (`projection_store.rs:105`) type-checked against five `todo!()`s and
was unimplementable. Three facts, each verified rather than reasoned about:
`begin` is total, synchronous and infallible; `sqlx`'s only route to a
`Transaction` is `async`, fallible, and its fields are private so the type cannot
be assembled by hand; and `probe_write` is synchronous and infallible too, so even
given a live transaction there is nowhere to issue a statement into it. `todo!()`
has type `!`, which coerces to everything — so nothing reported it.

**Options.**

| | Option | Verdict |
|---|---|---|
| A | An owned, `'static`, stamped buffered write set, transaction opened inside `commit`/`reset`. | **Chosen.** It is the shape `happenstance-sqlite` reaches by a different route, and PS-5's owned-batch evidence survives intact — the type is still owned and still `'static`, which is what the clause rests on. |
| B | Keep `Transaction` and block on `PgPool::begin` in `begin`. | Panics inside a runtime. Not an option, an outage. |
| C | Keep `Transaction` and acquire lazily behind a `OnceCell`. | Does not help. `begin` must *return* the transaction; laziness would need `commit` to be where it is acquired, which is option A wearing the wrong type. |

**And a result about a `[FROZEN]` clause, which is not this adapter's to change.**
PS-2 names a live-transaction adapter as the far end of the batch-shape axis still
to be built, offering `rusqlite` or `sqlx` as the two candidates. **Both are
refuted, each by its own mechanism**: `rusqlite`'s `Transaction<'_>` is `!Send`
and costs the `SendProjectionStore` impl; `sqlx`'s cannot be produced by a total
synchronous `begin` at all. The axis end is not unbuilt — for the two drivers the
clause names, the port's own signatures forbid it. Reported here and in the
adapter's module documentation; **the specification was not amended**, because
that belongs to PS-2's owner.

Three sub-decisions inside A, each of which the obvious answer gets wrong:

- **The regression guard is the upsert**, not a prior `SELECT … FOR UPDATE`.
  `FOR UPDATE` takes no lock on a row that does not exist, so two concurrent
  first-commits could both proceed. `ON CONFLICT … DO UPDATE … WHERE position <=
  excluded.position` is atomic and takes the row lock. But it reports a refusal as
  *zero rows* and cannot say what it refused against — `RETURNING` on `DO UPDATE`
  yields the new row and Postgres exposes no `OLD` — so the recorded position is
  re-read inside the same transaction to build `CheckpointRegression`'s pair, and
  the transaction is then **rolled back**: the caller's statements are already in
  it, and a refused commit that left the read model written is a partial
  application with no checkpoint to record it.
- **The stamp is process-global and minted once per store**, never per `begin`. A
  per-store counter incremented at `begin` hands every store the sequence
  1, 2, 3… and the foreign-batch check never fires.
- **Every `PgParam` variant carries an `Option`** rather than there being one
  `Null`. Postgres parameters are typed and `sqlx` takes the OID from the Rust
  type at bind, so an untyped null forces a concrete `None::<T>` at the bind site
  and `None::<i64>` against a `text` column is a server-side error.

**Result:** 21 of 21 green against a live PostgreSQL 17.10 on the first run — 14
rules, 3 reported skips, 4 adapter-private tests.

### D-04 — `publish = false` stays on `happenstance-postgres`, and phase 10b's criterion is the thing that gives

Phase 10b's second exit criterion reads *"No `todo!()` on either path; `publish =
false` removed."* Removing it collides head-on with a decision the owner has
already taken and `HANDOVER.md` lists under **do not re-open**: `0.2.0` ships
**five crates**, and `happenstance-postgres` is not one of them.

It is not a soft collision. `xtask/src/package.rs`'s `reconcile` holds
`PUBLISHABLE` against the manifests **in both directions** — so dropping
`publish = false` without adding the crate to `PUBLISHABLE` fails the gate, and
adding it makes a sixth published crate.

**Chosen: keep `publish = false`, and treat the criterion as superseded by the
later decision rather than the decision as blocked by the earlier criterion.**
The criterion's *substance* — no stub survives — is met and is checkable: the
`todo!()`s are gone and `#![allow(clippy::todo)]` left with them, which is the
proof the phase-12 work item actually asks for. What remains is a release-set
flag, and a flag that says "do not publish this" is not evidence of unreadiness
when the release set is five by decision. The crate-root prose now says which of
the two it is, because they were reading as the same thing.

**Left for the owner:** if `happenstance-postgres` should ship after all, it is
one line in `PUBLISHABLE`, one flag, a README, two licence files and a docs.rs
metadata block — not a re-plan.

### D-05 — the clause audits are a gate step rather than a pass somebody did

Phase 12's two auditable exit criteria say to check the ledgers *"against the
ledger and `cargo xtask spec-trace`, not against prose — the previous revision
carried this criterion with nothing to check it against."*

**Options.** Write the audit up as a document (what the criterion literally asks
for); or make it mechanical. Chosen: mechanical.
`runbook_clause_ledgers_match_the_specification` reads §7.2 — generated by
`spec-trace` and equality-checked against it — and requires both ledgers to name
exactly the clauses marked that way, in both directions, with every provisional
group naming an owner.

The argument for the stronger form is the ledger's own history: it has been wrong
twice and nothing noticed either time. Five clauses were missing at phase 3's
close — the five §1.3 names as carrying CF-25's exposure, so an audit reading the
table alone would have found 41 falsifiers under a heading saying 46 and concluded
everything was owned. Phase 5's recount was then wrong in both directions at once.
Both were caught by a human reading clause by clause. A document would have been a
third such pass.

Proven non-decorative in both directions before it was committed: a dropped clause
is reported, a clause the specification no longer marks is reported, and a blank
owner cell is reported — that last being the one that matters most, because a
blank cell reads as covered.

**Today it reports 46 provisional across 20 groups and 12 deferred, all owned.**

### D-06 — phase 12's deferred-clause criterion counted one and there are seven

The criterion said *"One qualifies, and it is safe: WF-1."* Classifying all twelve
`[DEFERRED]` clauses by the crate that publishes the surface each is about gives
**seven on a published surface**: WF-1, ES-39, PS-18, PS-27, PS-30, CF-14, CF-27.
The five `SY` clauses are out of scope because `happenstance-sync` is
`publish = false`.

Each of the seven now ships under a stated disposition rather than a count — the
full table is in `RUNBOOK.md`'s phase 12. The two that needed a real decision:
**ES-39** is *accepted in writing as a possible breaking 0.2*, because reporting
history a store does not hold needs a port surface and adding a method to
`EventStore` is breaking for every implementer; **CF-14** likewise, where the
breaking arm is `Fixture::REOPEN` changing type if "durable" turns out to need
grading — a case CF-32 already anticipates by giving the testkit its own version.

**And phase 10b moved one of them.** PS-18's deferral is owned by whoever *"takes
the count when the first adapter over storage this workspace does not control
clears the projection suite."* `PostgresProjectionStore` is that adapter, it has
cleared the suite, and it **declines** `RESET_REFUSAL`. The count is one adapter
and it did not implement protection.

### D-07 — the `READ_FAULT` injection is not either of the two the fixture named

`PostgresFixture`'s declension named `pg_terminate_backend` between two `FETCH`es,
or closing the cursor beneath the reader. **Neither survives contact**, and the
reasons are worth more than the injection:

- Arming happens *before* the read starts — the trait requires it, because a
  fixture that arms at `connect` arms nothing a rule already holding a handle can
  see. So there is no reader backend in flight and no cursor to close. Both
  presuppose a read already running, which is a fixture API the port does not
  have and CF-39 says it should not acquire.
- Terminating an *idle* pooled backend is absorbed: `sqlx` tests a connection
  before handing it out. That is CF-39's named hazard — a connection killed behind
  a reconnect-and-retry pool — one step earlier, and a fixture whose arming is
  silently absorbed passes the rule **vacuously**, which is what CF-39 forbids.
- A cursor is session-local; `pg_cursors` shows only the current session's.

**Chosen:** rename `event` aside and replace it with a view whose `WHERE` calls a
`plpgsql` function that raises above position 2. The `DECLARE` plans and opens
normally and the raise arrives while the `FETCH` is producing rows — which is the
failure the capability is about. A read that failed at name resolution would be a
different fault wearing the same name.

`arming_a_read_fault_makes_the_stream_yield_an_error` passes: the stream yields an
`Err` **item** rather than ending, so this adapter does not report a fetch failure
as the end of the log.

---

### D-08 — the ADR number for Ladybug is 0025, and this session got that wrong first

**Recorded because the error was mine and the correction came from a check I did
not do.** I read `ls .kb/decisions/` as "0001–0059 all taken, so the next free
number is 0060", and wrote that into the plan. It is false: the two directories
together hold **0001–0024 and 0029–0059**, and **0025, 0026, 0027 and 0028 are
free** — reserved by `RUNBOOK.md`'s ADR queue for phases 11, 13, 13 and 14
respectively, and left empty because those phases have not run.

The scan that produced the wrong answer had a real bug in it — it zero-padded an
already-padded loop variable, so every number came back "missing" and I took the
`ls` output instead without diffing the two. The lesson is the ordinary one: a
gap in a sequence is invisible to a listing and obvious to a diff.

**Consequence.** Phase 11's record is **ADR-0025**, which is what
`RUNBOOK.md:392`, `:585`, `:4802` and `:4810` all already say. Taking 0060 would
have orphaned four rows of a queue whose entire purpose is that a number is
allocated before the question is answered. Filling a reserved slot late is this
repository's own established move: `references/adr/0024` was written a month
after `0029`.

Everything else this session numbered — nothing — is unaffected; the two
knowledge-base documents it produced are **briefs**, which carry no ADR number by
construction.

---

## Left for the owner

1. **A UAC dialog is open on the desktop.** `winget install
   ShiningLight.OpenSSL.Dev` raised it and sat there; the installer behind it was
   killed, so the dialog is inert and needs dismissing by hand. OpenSSL was
   obtained without admin instead, through a user-space `vcpkg`.

2. **`.env` holds a live Neon password in plaintext** in the working tree. It is
   git-ignored (`.gitignore:75`) and nothing in this session committed, logged or
   echoed it. Rotating it afterwards is your call, and worth taking.

3. **The release itself, and the order is forced.** `cargo publish` for the five
   crates as core → testkit → happenstance → sqlite → cloudflare, because both
   adapters dev-depend on the testkit at the workspace version. Then the `v0.2.0`
   tag, the GitHub release, yanking `0.2.0-alpha.1`, and flipping the repository
   from `INTERNAL` to public.

   Then, and only then, five things that cannot be done before the registry
   carries `0.2.0`: run `scripts/stranger-install-smoke.sh`; add the
   `cargo-semver-checks` **registry** baseline beside `--baseline-rev`; fold
   `CHANGELOG.md`'s `[Unreleased]` into `[0.2.0]` and correct the date; mark
   phase 12 `done`; and add the `0.2.0` milestone row to the status table —
   **in that order**, because `runbook_status_matches_the_registry` requires a
   milestone row's whole dependency closure to read `done`, so the row must come
   after the flip and not before it.

4. **`happenstance-postgres` is finished and deliberately not published.** That is
   the five-crate decision rather than a readiness judgement; D-04 records what
   shipping it would take if you want it in.

5. **Set "require approval for first-time contributors"** in Actions settings
   before going public. `pull_request` carries no filter, so every fork PR fires
   the full three-OS matrix.

6. **The `.kb/_intake` briefs this session staged are owed an ingest.** Atoms are
   authored by `/redkiln:kb-ingest`, never by hand, so they are staged rather than
   written.
