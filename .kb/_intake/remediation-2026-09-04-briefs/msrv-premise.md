# Does ADR-0029's raise of the MSRV to 1.97.1 survive re-derivation, now that both premises it rested on are false?

Short answer, stated up front so the rest can be read as evidence rather than as
suspense: **none of the reasoning survives, and the recommendation is to lower
the number to 1.95** — one floor, workspace-wide, measured. Every ground
ADR-0029 wrote down for 1.97.1 has since been discharged by a fact that arrived
after it — including one discharged by ADR-0029's *own* consequence. And the
measured floor of the workspace at `HEAD` is **1.95.0**, two releases below the
declared one, and **1.88.0** for four of the five publishable crates.

This brief is the premise-audit lens applied to its own origin story. N-2 is the
finding that made "audit the premise" a standing check in this remediation, so
the distinction it turns on — *true when written* versus *true now* — is drawn
explicitly for every premise below, and a premise that has since become false is
not treated as proof the conclusion is wrong.

---

## Why this is owed

**The audit entry.** `references/evaluation/review-pre-publication-2026-09-03.md:2225-2239`
(N-2), which records `Clause: None governs the MSRV` and routes the decision to
`RUNBOOK.md:4708`, phase 12. Its own remediation line declines to choose:

> *"The decision has at least three shapes and this document takes none of them"*
> — `:2238`

**ADR-0029 named its own review point.** `references/adr/0029-msrv-raised-to-1-97-1.md:80-84`:

> **Bad.** 1.97.1 is recent, and anyone on a distribution-packaged toolchain is
> excluded. This is the trade, and **it is only defensible while nothing is
> published**. **Phase 12 must revisit it**: at first publish the number stops
> being a preference, and "the newest compiler that existed when we shipped" is
> not obviously the right promise to make to a consumer who is not building
> `happenstance-sqlite` at all.

That sentence is a tripwire the ADR set for itself. It has been tripped.

### The two premises, verbatim

**Premise 1 — the ground for raising the floor.**
`references/adr/0029-msrv-raised-to-1-97-1.md:49-51`:

> *Nothing is published. No downstream consumer is pinned to anything. The floor
> costs nobody anything today, and phase 12 is where it turns into a promise.*

and the atom, `.kb/decisions/0029-msrv-raised-to-1-97-1.md:60-61`:

> *Nothing is published yet, so nothing downstream is pinned to the old number.*

**Premise 2 — the ground for rejecting a per-crate MSRV.**
`references/adr/0029-msrv-raised-to-1-97-1.md:99-104`:

> - **Give `happenstance-sqlite` its own higher `rust-version`.** Honest
>   per-package metadata: **the crate is `publish = false`, so the three published
>   crates could have kept 1.85 truthfully.** Rejected on moving parts — CI would
>   need a second toolchain installed, and `cargo test --workspace` at 1.85 would
>   still fail, so that job needs an exclusion list that then has to be
>   maintained. It also makes "the MSRV" ambiguous in conversation, which is
>   worse than making it high.

and the atom, `.kb/decisions/0029-msrv-raised-to-1-97-1.md:83-84`, same words.

---

## What is true today

### 1. Premise 1 is false. Five names are on the registry; three carry real code.

`crates.io` API, queried 2026-09-03:

| crate | newest version | published | declared `rust_version` | edition | size |
|---|---|---|---|---|---|
| `happenstance-core` | `0.2.0-alpha.1` | 2026-08-16 | **`1.97.1`** | 2024 | 126,904 B |
| `happenstance` | `0.2.0-alpha.1` | 2026-08-16 | **`1.97.1`** | 2024 | 106,499 B |
| `happenstance-testkit` | `0.2.0-alpha.1` | 2026-08-16 | **`1.97.1`** | 2024 | 407,167 B (6,079 code lines) |
| `happenstance-sqlite` | `0.0.0` | 2026-08-18 | `1.85` | 2021 | 6,161 B (2 code lines) |
| `happenstance-cloudflare` | `0.0.0` | 2026-08-20 | `1.85` | 2021 | 6,168 B (2 code lines) |

Three of those are real content, not placeholders, and **not yanked**
(`"yanked": false` on every row). The 1.97.1 floor is therefore already an
immutable, permanently downloadable artefact fact for three crates — a published
version cannot be edited, only yanked.

Two qualifiers that cut the other way and belong in the same breath:

- **Zero reverse dependencies.** `reverse_dependencies?per_page=10` returns
  `total: 0` for all three. Total downloads are 38 / 52 / 39 — consistent with
  docs.rs and the author's own CI. *Nobody is actually pinned.* Premise 1's
  operative half ("no downstream consumer is pinned to anything") is, as a
  statement about real consumers, **still true**; it is only its literal half
  ("nothing is published") that is false.
- **These are pre-releases.** `Cargo.toml:6-14` is explicit that this was chosen
  for exactly that reason: *"A pre-release only resolves for a requirement that
  asks for one, so `cargo add happenstance` does not pick this up by accident.
  That is the intended behaviour, not a limitation."* A pre-release carries no
  compatibility promise.

So the honest reading is **premise 1 is false in letter and alive in spirit** —
and the spirit expires at stable `0.2.0`, not before.

### 2. Premise 2 is false without qualification.

`publish = false` is **gone** from both crates that matter to it, and its removal
was deliberate and paired with gate machinery, not an oversight:

- `crates/happenstance-sqlite/Cargo.toml:23-27` — *"There is deliberately no
  `publish = false` here now, and its absence is half of an atomic pair:
  `xtask/src/package.rs`'s PUBLISHABLE names this crate, and `reconcile()` fails
  on either half alone."*
- `crates/happenstance-cloudflare/Cargo.toml:22-29` — *"`publish = false` is
  gone, and its removal is half of a pair rather than a manifest tidy-up."*
- `xtask/src/package.rs:86-89` lists all five names in `PUBLISHABLE`.

`publish = false` survives only on the four skeletons and the non-shipping
members: `crates/happenstance-ladybug/Cargo.toml:12`,
`crates/happenstance-neon/Cargo.toml:12`,
`crates/happenstance-postgres/Cargo.toml:12`,
`crates/happenstance-sync/Cargo.toml:12`,
`examples/course-subscriptions/Cargo.toml:8`,
`examples/outside-projection-adapter/Cargo.toml:8`,
`examples/transfers-on-sqlite/Cargo.toml:8`, `xtask/Cargo.toml:7`.

And `happenstance-sqlite` is no longer the `todo!()` skeleton the ADR described:
**3,129 lines of real source** across six modules (`event_store.rs` alone is
1,508), with exactly one `todo!()` left, and `ADR-0022` — the driver/SQL decision
the rejection deferred to — is `status: accepted`.

### 3. A third premise, unnamed in the audit, is false — and ADR-0029 falsified it itself.

`references/adr/0029-msrv-raised-to-1-97-1.md:86-89`:

> **Neutral.** Nothing in the three publishable crates needed this. **All three
> build at 1.85 today** and would keep doing so; the floor is raised for workspace
> uniformity, not out of necessity. That is worth stating because **it is the
> thing a future reader will most want to know before lowering it again**.

Measured at `HEAD` with `rustup run 1.85 cargo check -p happenstance-core
--all-features --ignore-rust-version` (rustc 1.85.1):

```text
error[E0658]: `let` expressions in this position are unstable
   --> crates\happenstance-core\src\projection_memory.rs:329:12
    |
329 |         if let Some(current) = considered_through(recorded)
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

That is `crates/happenstance-core/src/projection_memory.rs:329-330`, and it is a
**let-chain** — the construct ADR-0029 itself unlocked
(`references/adr/0029-msrv-raised-to-1-97-1.md:66-70`: *"Good, and immediate:
let-chains are now available... nothing is rewritten here, because a decision and
a refactor should not land in one commit."*). The refactor landed later, and it
consumed the headroom the ADR told a future reader they had.

The sentence a future reader was told to rely on before lowering the floor is now
false, and the decision that wrote it is the reason.

Let-chains are now used in the library source of **three published crates**:
`crates/happenstance-core/src/projection_memory.rs:329`;
`crates/happenstance-sqlite/src/event_store.rs:559`, `:719` and
`src/projection_store.rs:835`; `crates/happenstance-cloudflare/src/js.rs:237` and
`src/event_store.rs:901`, `:901`. (`happenstance` and `happenstance-testkit`
have none in `src/`; they inherit the floor through `happenstance-core`.)

### 4. The measured floor of every crate, bisected

Declared floor is `Cargo.toml:17` — `rust-version = "1.97.1"` — inherited by
every member via `rust-version.workspace = true`. Measured by running the
compiler, which `references/adr/0029-msrv-raised-to-1-97-1.md:25-31` correctly
identifies as the only instrument that works here:

| crate | declared | **measured floor** | slack | what pins it |
|---|---|---|---|---|
| `happenstance-core` | 1.97.1 | **1.88.0** | 9 releases | let-chain, `projection_memory.rs:329` |
| `happenstance` | 1.97.1 | **1.88.0** | 9 | inherited from `-core` |
| `happenstance-testkit` | 1.97.1 | **1.88.0** | 9 | inherited from `-core` |
| `happenstance-cloudflare` (wasm32) | 1.97.1 | **1.88.0** | 9 | own let-chains + `-core` |
| **`happenstance-sqlite`** | 1.97.1 | **1.95.0** | **2** | `libsqlite3-sys` build script |
| `-postgres` / `-ladybug` / `-sync` | 1.97.1 | 1.88.0 | — | `publish = false` |

Bisection transcript for the one crate that is different — `cargo check -p
happenstance-sqlite --all-features --ignore-rust-version`:

```text
1.88.0   error: cannot find macro `cfg_select` in this scope   (libsqlite3-sys build script)
1.91.1   error[E0658]: use of unstable library feature `cfg_select`
1.93.1   error[E0658]: use of unstable library feature `cfg_select`
1.94.1   error[E0658]: use of unstable library feature `cfg_select`
1.95.0   Finished `dev` profile ... in 7.03s
1.96.1   Finished `dev` profile ... in 8.11s
```

**`cfg_select` stabilised in 1.95.** The atom's summary
(`.kb/decisions/0029-msrv-raised-to-1-97-1.md:17-18`) and the long form's context
both describe it as *"unavailable before 1.88"*, which is off by seven releases —
harmless at the time, because the number chosen was above both, but it is the
figure a future reader would use.

So: **no crate in this workspace needs 1.97.1.** The declared floor is two
releases above the highest real requirement and nine above what four of the five
publishable crates require. The number is now simply `rust-toolchain.toml:2`'s
pin — *"a number tied to the pin is at least a number with a reason"*
(`:110-111`) — but the reason it was tied to is gone.

### 5. The five metadata-silent database crates: confirmed, and there are more

`cargo metadata --all-features` over the pinned tree confirms the ADR's central
evidence exactly. `libsqlite3-sys 0.38.1`, `rusqlite 0.40.1`, `sqlx 0.8.6`,
`sqlx-core 0.8.6` and `sqlx-postgres 0.8.6` all report `rust-version = None` —
five of five, as claimed. `sqlx-macros` and `sqlx-macros-core` are silent too, so
it is seven of seven in the sqlx family. **79 of 255 packages** in the full graph
declare no `rust-version`. This premise is **true when written and true now**, and
it is the one load-bearing fact ADR-0029 established that has not moved.

It also confirms the audit's `:2235` measurement: the maximum *declared*
`rust-version` in `happenstance-core`'s non-dev closure is **`trait-variant` at
1.75**, and that closure contains **zero** packages declaring nothing. The
audit's inference from there — *"`edition = "2024"` puts its real floor at 1.85"*
— is the one place N-2 is itself understated: it measured dependencies and not
the crate's own source, and the real floor is 1.88.

### 6. ADR-0004's `provisional` marker: the event that lifts it has already half-happened

`references/adr/0004-edition-and-msrv.md:3` — `**Status:** accepted — provisional`.
`.kb/decisions/0004-edition-and-msrv.md:83-86`:

> An MSRV bump is a minor version bump, and is called out in the changelog. This
> is a preference until first publish (phase 12) and a promise to downstream
> consumers afterward — nothing is published yet, so `rust-version = "1.85"` was,
> at the time, a self-imposed constraint raisable at zero cost to anyone, not a
> hard limit to weigh a dependency against.

`RUNBOOK.md:4708` still carries the unchecked box:

```text
- [ ] ADR-0004 loses `provisional`; the MSRV becomes a promise.
```

under phase 12, which `RUNBOOK.md:163` marks `not started`.

**Has the event happened?** *Partly, and in the way that is least useful.* The
artefacts are live and immutable, so the floor has been *advertised*; but it has
been advertised on pre-releases that resolve only to an explicit pre-release
requirement, to zero dependants. The clean reading is:

> **Publication has happened; the promise has not.** ADR-0004's `provisional`
> marker should stay until stable `0.2.0`, because the marker tracks the
> *promise*, not the upload. But the window in which the floor can be moved
> *without anyone noticing* is the window that is closing, and phase 12's box
> is now the last cheap moment rather than a scheduled one.

Note the subsidiary drift, which is not this brief's to settle: `RUNBOOK.md:158`
places the `0.2.0-alpha.1` release after phase 7, and `RUNBOOK.md:156` marks
phase 6 `not started` — yet the alpha shipped on 2026-08-16. The status table and
the registry disagree.

### 7. The CI job, and the two README bullets

`.github/workflows/ci.yml:307-326`, with its own comment admitting the problem:

```yaml
  msrv:
    name: minimum supported Rust version
    ...
          # 1.97.1 as of ADR-0029, raised from 1.85. Kept spelled out here rather
          # than deferring to `rust-toolchain.toml` so that this job fails loudly
          # if the two ever disagree — the pin and the promise are different
          # facts that currently happen to be the same number.
          toolchain: "1.97.1"
```

`CLAUDE.md`'s claim that the job proves nothing until the two diverge is
**verified**: `Cargo.toml:17` and `rust-toolchain.toml:2` both read `1.97.1`, so
`cargo hack check --workspace --no-dev-deps --rust-version` runs the gate's own
compiler against the gate's own floor. ADR-0029 called this *"the real cost of
this ADR"* (`:72-78`). **Any option below that lowers the number restores the
job's meaning as a side effect.**

Two published front pages state the floor as a `## Guarantees` bullet —
`crates/happenstance-core/README.md:53-56` and
`crates/happenstance/README.md:49-52`, both:

> - MSRV 1.97.1, checked in CI. Raised from 1.85 at phase 2 by a *dependency's*
>   build script rather than by this crate's own code

Every clause of that bullet is now false for those two crates: they are not
raised by a dependency's build script (nothing in their closure needs more than
1.75 declared / 1.88 measured), they are raised by *their own code's* let-chain,
and it is not meaningfully checked in CI. This is the **second** occurrence of
this exact drift class — `CHANGELOG.md:1784-1791` records the first, where the
same two READMEs promised *"MSRV 1.85, checked in CI"* twelve minor versions
below the manifest, and notes *"Nothing checks a README's prose against a
manifest, which is why it survived a phase."* Nothing checks it now either.

### 8. Governing records and specification

- **Decision atoms:** `ADR-0004` (`.kb/decisions/0004-edition-and-msrv.md`,
  accepted, `provisional`) and `ADR-0029`
  (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`, accepted,
  `depends_on: [kb-decision-0004]`, `supersedes: null`, `superseded_by: null`).
  `ADR-0035` mentions `worker`'s `rust-version = 1.75` in passing only. No other
  atom of the 36 governs the MSRV.
- **What a change would supersede:** *nothing, if the house pattern is followed.*
  ADR-0029 amended ADR-0004 without superseding it, and said why
  (`:57-61`: ADR-0004's *reasoning* is what licensed the move). A re-derivation
  is the same shape one level on: **a new atom that amends ADR-0029, which in
  turn amends ADR-0004** — leaving both `superseded_by: null`. Choosing formal
  supersession instead would be a departure from the precedent ADR-0029 set and
  should be a conscious one. Either way, both accepted bodies are immutable; the
  correction is a new atom, never an edit.
- **Specification:** `grep -in "msrv|rust-version|minimum supported"
  spec/SPECIFICATION.md` returns **no clause**. The only hits are prose:
  `spec/SPECIFICATION.md:147-154` cites `0004:5-12` as one of three ADRs marked
  *"accepted — provisional"* whose provisionality *"is discharged here, clause by
  clause"* — which is a claim about ADR-0004's *edition and lifetime-capture*
  content, not its MSRV number; the remaining hits (`:1680`, `:4698`, `:6001`,
  `:9270`) name 1.97.1 only as the toolchain a measurement was taken on. **The
  audit is right that no clause governs the MSRV**, and no clause maturity marker
  is at stake in this decision.

---

## Options

### Option A — Hold 1.97.1 workspace-wide; lift ADR-0004's `provisional` at phase 12 as planned

Do nothing but record that the premises were re-derived and the number stands on
uniformity alone.

- **Costs a caller:** the highest of any option. Excludes every toolchain below
  1.97.1 for a library four-fifths of which needs 1.88. A consumer of
  `happenstance` — the crate `Cargo.toml:6-14` says most people will
  `cargo add` — is turned away by nine releases of margin their build would never
  have touched. ADR-0029's own words for this, `:80-81`: *"anyone on a
  distribution-packaged toolchain is excluded."*
- **Costs an adapter author:** an outside author writing an adapter against
  `happenstance-core` inherits 1.97.1 as their own crate's minimum, because both
  `resolver = "3"`'s MSRV-aware resolution and `cargo hack --rust-version` read
  the dependency's declared floor. They cannot honestly declare 1.88 for a crate
  that needs 1.88. **This cost is inferred, not observed:** the earlier draft
  cited `examples/outside-projection-adapter/` as the instrument that keeps this
  persona visible, and that claim has been **removed** — see the revision record.
  The example declares `rust-version.workspace = true`
  (`examples/outside-projection-adapter/Cargo.toml:6`), so it inherits whatever
  the workspace says under every option and can never register the difference
  between a floor that is honest for it and one that is not. Nothing in this
  repository measures this cost.
- **Semver class:** **none.** Nothing changes; `0.2.0-alpha.1` already ships this
  number.
- **Forecloses:** little formally — lowering later is additive and always
  available. But it fixes 1.97.1 as what the stable `0.2.0` READMEs promise, and
  it permanently forfeits the `msrv` CI job, which stays vacuous by construction
  for as long as the two numbers are equal. It also leaves two `## Guarantees`
  bullets stating three things that are each false.

### Option B — One number, lowered to the workspace's measured maximum: **1.95**

`Cargo.toml:17` → `rust-version = "1.95"`. Every member keeps
`rust-version.workspace = true`. `rust-toolchain.toml:2` stays at 1.97.1.

- **Costs a caller:** two releases cheaper than today for everyone, and honest in
  the sense *"no crate in this workspace builds below this."* Still seven
  releases above what a caller of `happenstance` actually needs.
- **Costs an adapter author:** the same seven-release inheritance as Option A,
  reduced by two. Still requires 1.95 to build a crate whose closure's highest
  declared floor is `trait-variant` at 1.75.
- **Semver class:** **additive.** Lowering an MSRV never breaks a build.
- **Costs the repository:** two tokens. `Cargo.toml:17`'s number, and the
  `toolchain: "1.97.1"` string at `.github/workflows/ci.yml:319`. Nothing else
  moves: the `msrv` job's second step, `cargo test --workspace --all-features`
  (`ci.yml:339`), runs **unexcluded** at 1.95, because 1.95 is the measured floor
  of `happenstance-sqlite` — the bisection above records `1.95.0 Finished`. So
  every crate in the workspace, `happenstance-sqlite`'s 3,129 lines included,
  stays covered by the MSRV job at the number the job pins.
- **Forecloses:** nothing. Option C stays available at any time and remains
  additive. Keeps "the MSRV" a single unambiguous number — ADR-0029's only
  surviving objection to per-package is answered by construction. **Restores the
  `msrv` CI job**: 1.95 ≠ 1.97.1, so `cargo hack check --no-dev-deps
  --rust-version` on a pinned 1.95 starts checking something the day it lands.

### Option C — Per-package: **1.88** workspace-wide, `happenstance-sqlite` alone at **1.95**

`Cargo.toml:17` → `rust-version = "1.88"`; `crates/happenstance-sqlite/Cargo.toml`
replaces `rust-version.workspace = true` with `rust-version = "1.95"`. Exactly one
crate deviates. This is ADR-0029's rejected alternative, re-derived against facts
it did not have.

- **Costs a caller:** the lowest of any option, and the only one that is honest
  per-crate. A caller of `happenstance` or `happenstance-core` needs 1.88; a
  caller of `happenstance-sqlite` needs 1.95, which is what it genuinely needs.
- **Costs an adapter author:** the least — an outside adapter author binding
  `happenstance-core` may truthfully declare 1.88 for their own crate. **But this
  is an inference about a persona the repository cannot observe** (see Option A's
  bullet, and the revision record): the one crate written to model that persona
  inherits the workspace floor, and the registry puts the real population at zero
  — 0 reverse dependencies, 38 / 52 / 39 downloads, pre-releases that resolve
  only to an explicit pre-release requirement.
- **Semver class:** **additive** for four crates, **none** for the fifth (1.95 is
  a lowering from 1.97.1 there too — so additive across the board).
- **Forecloses:** nothing, but it makes the floor a per-crate fact that must be
  re-derived whenever a crate's dependency graph moves — and this workspace's own
  evidence is that the dependencies which move it declare nothing (79 of 255).
- **What ADR-0029's three objections cost now, measured rather than assumed:**
  - *"CI would need a second toolchain installed"* — **still true, and cheap.**
    `cargo hack --rust-version` is documented as *"Perform commands on
    `package.rust-version`"* (cargo-hack 0.6.45 `--help`), i.e. it runs each
    package at its own declared floor automatically. The cost is one extra
    `dtolnay/rust-toolchain` step in `.github/workflows/ci.yml`. Installing 1.88
    and 1.95 here took under two minutes each.
  - *"`cargo test --workspace` at 1.85 would still fail, so that job needs an
    exclusion list that then has to be maintained"* — **true, and it costs more
    than the earlier draft charged it.** Three corrections, all of which cut
    against Option C:

    **The measurement was a compile, not a run.** `rustup run 1.88 cargo test
    --workspace --all-features --no-run --exclude happenstance-sqlite --exclude
    transfers-on-sqlite` → `Finished 'test' profile ... in 3m 38s`, exit 0. The
    earlier draft leaned on that as C's evidence; `--no-run` means every test was
    *built* and **none was executed**, nine releases below the pinned compiler.
    The claim that C's test leg is green at 1.88 is therefore **removed** — what
    is established is that it compiles. Option B's leg is two releases below the
    pin rather than nine, and runs.

    **The `-A dead_code` caveat's stated reason was self-contradictory and is
    removed.** It read *"it needs `-A dead_code`, because 1.88 does not flag
    `xtask`'s unused `MAX_NEEDS` const that 1.97.1 does"* — which, if true, is a
    reason the flag is *not* needed. What survives is the fact without the
    explanation: the 1.88 leg required a blanket `-A dead_code` to reach exit 0,
    and why is unresolved. A blanket lint allowance on a leg that also carries a
    `-D warnings` workspace bar is not a rounding error.

    **The exclusion list has a hole nobody costed.** Excluding
    `happenstance-sqlite` and `transfers-on-sqlite` from the 1.88 leg removes
    from MSRV coverage the crate with the **highest** floor and **3,129 lines**
    of real code — the one crate most likely to move. Closing it needs a *second*
    `cargo test --workspace --all-features` at 1.95 restricted to that pair, i.e.
    a second toolchain step, a second cache, a second job leg and an exclusion
    list maintained in two directions. Under Option B that whole apparatus is
    one unexcluded command. The repository does run a two-name list of this kind
    honestly — `xtask/src/package.rs`'s `reconcile()` fails in both directions —
    but that is one constant checked by one function, not a CI matrix.
  - *"It makes 'the MSRV' ambiguous in conversation, which is worse than making
    it high"* — **untouched by any measurement, and the only surviving argument
    for the status quo.**

### Option D — 1.88 workspace-wide, bought by pinning `rusqlite` back to 0.37

ADR-0029's *first* rejected alternative (`:93-97`), re-derived. Would give one
number, at the true floor of everything else.

- **Costs a caller:** the lowest possible, with no per-crate ambiguity.
- **Costs an adapter author:** nothing beyond Option C.
- **Semver class:** **additive** on the MSRV; but `rusqlite` is in
  `happenstance-sqlite`'s public error surface, so a major downgrade of it may be
  **breaking** for that crate independently of the floor. Unassessed here.
- **Forecloses:** the reason ADR-0029 gave for rejecting it has **expired** —
  *"phase 8, which owns the driver decision through ADR-0022, would inherit a
  version choice made by an MSRV constraint"* (`:95-97`). Phase 8 has landed:
  ADR-0022 is accepted and `happenstance-sqlite` carries 3,129 lines of real
  code. But a **new** objection has arrived in the other direction, and it did
  not exist when the crate was a `todo!()` skeleton: pinning a C-binding
  dependency back three minor versions on a *published* adapter means shipping an
  older SQLite amalgamation to real users for a reason that has nothing to do
  with SQLite. That is a supply-chain argument, not an ergonomics one.
- **Unverified.** ADR-0029 verified `rusqlite 0.37` / `libsqlite3-sys 0.35`
  building at both toolchains (`:38-40`) against a skeleton whose *"every body is
  `todo!()`"*. It has **not** been verified against the adapter that exists now,
  and it would need to be before this option is real.

---

## Recommendation

**Option B — one number, lowered to 1.95.**

**This recommendation flipped.** The earlier draft recommended Option C, and it
rested C on the outside adapter author, citing
`examples/outside-projection-adapter/` as the instrument that keeps that persona's
experience visible. That instrument does not work: the example declares
`rust-version.workspace = true` (`examples/outside-projection-adapter/Cargo.toml:6`),
so it inherits the workspace floor under every option and cannot register the
difference between C and B at all. With that premise gone, C's headline advantage
is an inference about a population the registry measures at **zero** — 0 reverse
dependencies, 38 / 52 / 39 downloads, pre-releases that resolve only to an
explicit pre-release requirement. C's advantage is denominated in compiler
releases; B's and C's costs are denominated in CI runs, and only one of those two
currencies has anyone spending it.

What is left once that is removed:

- **C's benefit is unobserved and, today, unpopulated.** Seven releases of
  inherited floor is a real difference to a stranger — but no stranger exists,
  nothing in the tree would notice if one did, and lowering later is additive, so
  the first stranger to appear can be answered then at zero semver cost.
- **C's costs are real, recurring and larger than the earlier accounting.** Not
  *"one toolchain step, two names"*: a blanket `-A dead_code` on the 1.88 leg for
  a reason that is unresolved; a 1.88 leg whose tests were only ever **compiled**,
  never run, nine releases below the pin; and an exclusion list that removes
  `happenstance-sqlite`'s 3,129 lines — the highest floor in the workspace — from
  MSRV coverage entirely unless a second toolchain, cache and test leg at 1.95 is
  added, which nothing in the earlier draft costed.
- **B is a two-token diff.** `Cargo.toml:17` and `.github/workflows/ci.yml:319`.
  The `msrv` job's `cargo test --workspace --all-features` (`ci.yml:339`) then
  runs **unexcluded** at 1.95, because 1.95 is the measured floor of the only
  crate that differs. Every crate stays covered, at a number two releases below
  the pin rather than nine.
- **B repairs the vacuous `msrv` job exactly as well as C does.** 1.95 ≠ 1.97.1,
  which is the whole of what ADR-0029 called *"the real cost of this ADR"*
  (`:72-78`). That repair was never C's to claim over B.
- **B keeps one number**, so ADR-0029's only objection that no measurement has
  touched — ambiguity in conversation — never arises.

B still beats Option A for the reason C did: **every ground ADR-0029 gave for
1.97.1 is now false**, including one that ADR-0029's own consequence falsified,
and holding a number whose entire recorded justification has expired is the
failure mode this remediation is named after. It beats Option D because D is
unverified against the adapter that now exists and buys its elegance with an
older SQLite amalgamation shipped to real users.

**The strongest argument against it, verbatim:**

> Take Option B (1.95, one number). C's advantage is denominated in compiler
> releases, never in consumers, and the brief's own evidence puts that population
> at zero (0 reverse deps, 38/52/39 downloads, pre-release-only resolution). C's
> costs recur every CI run and exceed its "one toolchain step, two names"
> accounting: the 1.88 leg needs a blanket -A dead_code (its stated reason is
> self-contradictory), and excluding happenstance-sqlite and transfers-on-sqlite
> leaves the crate with the highest floor and 3,129 lines of real code covered by
> no MSRV job unless a second cargo test at 1.95 is added, which is never costed.
> B is a two-token diff (Cargo.toml:17, ci.yml's toolchain string) and the
> existing cargo test --workspace --all-features runs unexcluded, since sqlite is
> green at 1.95. B repairs the vacuous msrv job identically, is additive, and
> leaves C available later at zero cost.

That is the argument that flipped this recommendation, so it is recorded here as
the case *for* B rather than against it. The strongest argument **against B** is
the one C was built on, minus the instrument that could not carry it: a caller of
`happenstance` — the crate `Cargo.toml:6-14` says most people will `cargo add` —
is still turned away by seven releases their build would never have touched, and
B's answer to that is *"nobody is currently being turned away"*, which is an
argument from an empty registry and expires the moment it stops being empty. If
the reader weighs a future stranger's inherited floor above CI simplicity,
**Option C remains the defensible second** — it is additive, available at any
time, and the gap between them is one number's worth of honesty rather than a
correctness question.

**What this brief does *not* recommend:** it does not recommend lifting ADR-0004's
`provisional` marker now. The marker tracks the *promise*, and the promise begins
at stable `0.2.0`, not at a pre-release with zero dependants.
`RUNBOOK.md:4708`'s box is correctly still unchecked.

---

## Cost of delay

**Free now; not free for long; and not equally cheap forever.**

- **Today the change costs nothing.** Zero reverse dependencies, total downloads
  in the tens, and only pre-releases on the registry — which resolve solely to an
  explicit pre-release requirement (`Cargo.toml:12-14`). Lowering a floor is
  additive in any case, so even a late change breaks no build.
- **The asymmetry is what makes the window real.** Lowering is additive forever;
  **raising is breaking in practice, whatever the manifest key says.** So the
  cheap moment is not about permission to change the number — it is about
  *choosing the number you can still raise from*. Set it at a measured floor now
  — 1.95 under the recommendation — and every future dependency that drags it
  upward is a documented minor bump from a defensible base. Leave it at 1.97.1
  and the only direction left that costs nothing is down, from a base nobody
  chose.
- **`0.2.0` is the genuinely cheap moment, and `happenstance-sqlite` is why.**
  The audit's `:2237` is right and worth restating: *"`0.2.0` is genuinely the
  cheap moment for this and the only one — the promise begins at the stable
  release, and `happenstance-sqlite`'s first published version is that release."*
  The only crate whose floor differs from the rest has never shipped real code to
  the registry (`0.0.0`, 2 code lines, and it currently advertises `1.85`). Once
  it ships at 1.97.1, a later split to 1.95 is still additive but is now a
  correction visible in a changelog rather than a first statement.
- **Doing nothing has a compounding documentation cost.** The two `## Guarantees`
  bullets are false in three clauses each and are the artefact `cargo package`
  ships to a reader who has not cloned the repository. `CHANGELOG.md:1784-1791`
  records that this exact drift *"survived a phase"* the first time because
  *"nothing checks a README's prose against a manifest."* Nothing checks it now.
  Whichever option is chosen, those two bullets are downstream of it and should
  land in the same change.

---

## What this does not settle

1. **Whether the number should be 1.88, 1.95 or 1.97.1.** This is a brief; the
   human decides. The measurements above are offered so the choice is made
   against the floor the compiler reports rather than against the pin.
2. **Whether Option D is viable at all.** `rusqlite 0.37` / `libsqlite3-sys 0.35`
   was verified by ADR-0029 against a `todo!()` skeleton and has not been checked
   against the 3,129-line adapter that exists now, nor against ADR-0022's SQL
   strategy, nor for a breaking change in `happenstance-sqlite`'s public error
   surface. It needs one `cargo check` and one API diff before it is a real
   option.
3. **The exact floor after any refactor.** 1.88 is what `HEAD` measures. It is a
   *measured* fact about today's source, not a *protected* one: the same
   let-chain refactor that consumed ADR-0029's headroom can happen again, and
   `cargo hack --rust-version` cannot see it coming any more than it saw
   `cfg_select`. If 1.88 is chosen, something has to run the compiler at 1.88 —
   which is precisely what a non-vacuous `msrv` job would do.
4. **Whether a floor should be a `## Guarantees` bullet at all.** Both READMEs
   state it as a guarantee; nothing checks a README against a manifest, and this
   is the second time the pair has gone stale. Whether that gap is closed with a
   gate step, a generated line, or by demoting the claim to prose is a separate
   decision and is not this one.
5. **ADR-0004's `provisional` marker and phase 12's ordering.** The
   recommendation is to leave it, but the reasoning above — publication has
   happened, the promise has not — is a judgement about what the marker tracks,
   and phase 12 owns it.
6. **The `RUNBOOK` / registry disagreement.** `RUNBOOK.md:158` places
   `0.2.0-alpha.1` after phase 7 and `RUNBOOK.md:156` marks phase 6 `not started`,
   while three crates shipped on 2026-08-16. That is a status-table drift, noted
   as evidence here and routed nowhere by this brief.
7. **The atom summary's `cfg_select` figure.** `.kb/decisions/0029-...:17-18`
   describes it as unavailable before 1.88; it stabilised in 1.95. ADR-0029 is
   accepted and immutable, so this cannot be edited — it is recorded here so a
   new atom can carry the corrected measurement.

### One remedy the evidence does not support

The audit's `:2238` offers as its third shape: *"lower the floor on the three
crates whose graphs do not require it."* Taken literally — **three** crates, and
on the evidence of their **graphs** — that is not supportable at `HEAD`, for two
independent reasons this brief measured:

- It is **four**, not three. `happenstance-cloudflare` also builds at 1.88
  (verified on `wasm32-unknown-unknown`), and it is in `PUBLISHABLE`
  (`xtask/src/package.rs:86-89`) with `publish = false` removed
  (`crates/happenstance-cloudflare/Cargo.toml:22-29`). The "three published
  crates" framing is inherited from ADR-0029's own era and has drifted the same
  way `CLAUDE.md` records the crate count drifting.
- Their **graphs** are not what sets their floor. The audit's own measurement at
  `:2235` is right that `happenstance-core`'s non-dev closure tops out at
  `trait-variant` 1.75 — but its inference that *"`edition = "2024"` puts its real
  floor at 1.85"* is falsified by the compiler: the floor is 1.88, set by
  `happenstance-core`'s **own source**, at `projection_memory.rs:329-330`. A
  remedy that lowered these crates to 1.85 on graph evidence would not compile.

The shape of the remedy is sound; both of its numbers are wrong, and the correct
ones are 1.88 across four crates and 1.95 for the fifth.

---

## Revision record

Revision 2, after adversarial review. Three things changed; the measurements in
*What is true today* are untouched.

1. **A falsified premise, and the claims removed with it.** The earlier draft
   rested Option C on the outside adapter author and named
   `examples/outside-projection-adapter/` as the instrument keeping that persona
   visible. That manifest declares `rust-version.workspace = true`
   (`examples/outside-projection-adapter/Cargo.toml:6`), so it inherits the
   workspace floor under every option and can never register the difference the
   argument depends on. **The claim that this repository can observe the cost
   Option C removes is removed**, in Option A's *costs an adapter author* bullet,
   in Option C's, and from the Recommendation. What survives is the narrower,
   inferred fact — a declared floor is inherited by a dependent crate — flagged
   as unmeasured, alongside the registry evidence that the affected population is
   currently zero.

2. **A second falsified premise: C's 1.88 test leg was never run.** The
   `3m 38s`, exit 0 measurement used `--no-run`: the tests were compiled and none
   executed, nine releases below the pinned compiler. **The claim that C's
   `cargo test` half is green at 1.88 is removed** and restated as what it is — a
   compile. In the same bullet, the `-A dead_code` caveat's stated reason
   (*"1.88 does not flag ... that 1.97.1 does"*) is self-contradictory as a reason
   for needing the flag; **the explanation is removed** and the bare fact kept,
   marked unresolved. Added in the same place: the exclusion list's uncosted
   hole — `happenstance-sqlite`'s 3,129 lines fall out of MSRV coverage under C
   unless a second toolchain and test leg at 1.95 is added.

3. **The recommendation flipped, C → B.** What flipped it: the falsified premise
   above removed C's headline benefit as an observable, the registry puts its
   beneficiary population at zero, and C's costs are larger and more recurrent
   than the earlier accounting charged — while B is a two-token diff whose
   `msrv` job runs unexcluded at 1.95 and repairs the vacuous job identically.
   The argument that flipped it is quoted verbatim under *Recommendation*, and
   the strongest remaining case for C is stated there as the argument against B.
   Option C's own section is unchanged apart from the corrections in (1) and (2)
   and remains the defensible second. Two sentences downstream of the flip moved
   with it: the opening summary, which read *"the number survives"* and
   contradicted both this recommendation and the previous one, and the *Cost of
   delay* bullet that named 1.88 as the base to set now.

Nothing else in the brief moved, and no file outside it was touched.
