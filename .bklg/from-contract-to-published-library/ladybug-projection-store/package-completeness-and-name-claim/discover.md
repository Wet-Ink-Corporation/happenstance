---
item: HS-S0080
stage: discover
created: 2026-08-12T13:02:49.804Z
updated: 2026-08-12T13:02:49.804Z
template_sig: 86ce4036
rendered_sig: 5eca91d2
---

# Discover — Package-complete the crate and hold the name

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — copy both licences and a README into the crate directory, drop `publish = false`, add the crate to `xtask/src/package.rs`'s `PUBLISHABLE`, and claim `happenstance-ladybug` on crates.io behind a `cargo publish --dry-run` | `_storymap.md:46` | Four edits, three of which fail the gate together if any is done alone. |
| **AC-010** — the name is claimed on crates.io and the crate carries both licence files and a README, so `publication-and-positioning`'s publish decision is never blocked on this crate's readiness | `project.md:218-221` | The AC is about *readiness*, not publication. HS-P0016 decides whether it ships. |
| Phase 0's rule — a name is reserved when its phase starts and not before, *"the point being that by now there is a crate to justify it with"*; the per-phase claim list names `happenstance-ladybug` at phase 11 | `project.md:70-72`; `RUNBOOK.md:826-829`, `:4411-4413` | The claim is timed deliberately, and `cargo xtask reserve <name>` generates the placeholder. |
| **M5** — `PUBLISHABLE` is a hand-declared set reconciled against what the manifests actually say; removing `publish = false` **without** adding the crate fails the gate, and adding it while `publish = false` remains fails from the other direction | `_decomposition.md:173-184`; `xtask/src/package.rs:86`, `:186-202` | The two edits are one edit. The failure messages name the direction, which is what makes the coupling survivable. |
| `REQUIRED_FILES` is `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`, and they must be **copied into the crate directory** — *"Cargo will not follow a path outside it"* | `xtask/src/package.rs:94`, `:152-157` | Not referenced, not linked from the workspace root. The check's own error message says so in as many words. |
| The layout to copy is `crates/happenstance-core/`, including the explicit `readme = "README.md"` key and its rationale — stated rather than auto-discovered, because *"the key is what crates.io renders on the package page, and a silent default is a poor thing to rely on for the first impression anyone gets of the crate"* | `_decomposition.md:180-182`; `crates/happenstance-core/Cargo.toml:12-15` | The precedent covers the manifest key as well as the files, and its reason is about first contact, which is what a README is for. |
| The gate's own assertion is `cargo package --list` under the step named *"packaged artifacts carry their licences and README"* | `xtask/src/main.rs:519`; `_decomposition.md:480` | AC-010's mechanical half is already checked by an existing step; this story's job is to make that step have something to check, not to add one. |
| **docs.rs is a known, separate failure** — docs.rs itself fails to build `lbug` 0.19.1, and removing `publish = false` turns that from a note into a real consequence; it is `publication-and-positioning`'s call, and this project's obligation is to hand that project the number and the `[package.metadata.docs.rs]` question | `_decomposition.md:401-406`; `crates/happenstance-ladybug/src/lib.rs:31-32`; `crates/happenstance-core/Cargo.toml:54-56` | Phase 12's proof artefact is docs.rs green under `--all-features` (`RUNBOOK.md:162`), so an unremarked docs.rs failure is a proof obligation handed over broken. |
| Out of scope — whether this crate is published at `0.2.0`; the decomposition's working default is three crates only | `project.md:127-130` | AC-010 makes the crate ready. `verdict-ordering-and-publication-handoff` (HS-S0083) carries the facts to whoever decides. |
| Slice rationale — `packaging-and-ci-shape` is one slice because both stories are the same surface, *what the workspace pays*; `PUBLISHABLE` and `publish = false` fail the gate together | `_storymap.md:79-82` | The sibling story (`cold-build-cost-and-ci-shape`) is unordered with this one and touches the same gate file for a different reason. |
| `depends_on: fill-the-bodies-and-ps-34-disposition` — supplies a crate with no `todo!()` in it, which is the difference between claiming a name for an adapter and claiming one for a placeholder | `_storymap.md:46`; `RUNBOOK.md:4411-4413` | Phase 0's rule is precisely that there is *a crate to justify it with*. Claiming earlier would satisfy the mechanics and not the rule. |
| crates.io name-claim itself is outside any test tier — a manual, one-time `cargo publish --dry-run` plus the actual claim, recorded in the verdict or `RUNBOOK.md` | `_decomposition.md:480` | The one obligation here with no automated guard at all, which is why it must leave a written record. |

## Questions

**Does removing `publish = false` mean this crate gets published? — answered:
no.** It means Cargo *would* publish it, which is what makes `PUBLISHABLE`,
`REQUIRED_FILES` and `cargo package --list` start applying. Whether it ships at
`0.2.0` is `publication-and-positioning`'s, against a working default of three
crates only (`project.md:127-130`). The distinction is worth stating in `spec`
because the gate's failure message reads *"Cargo will publish X"*
(`xtask/src/package.rs:188-192`), which invites the opposite reading.

**Does docs.rs failing to build `lbug` block AC-010? — answered: no, and it is
handed forward rather than absorbed.** The crate can be package-complete and
name-held with a docs.rs build that fails; what it cannot be is silently so.
This story records the failure as a known fact against `lbug` 0.19.1, and
`verdict-ordering-and-publication-handoff` (HS-S0083) carries it — together with
the `[package.metadata.docs.rs]` question modelled on
`crates/happenstance-core/Cargo.toml:54-56` — to HS-P0016, whose own proof
artefact is docs.rs green under `--all-features`.

**What form does the name claim take? — deferred to `spec` in mechanism, fixed in
substance.** `cargo publish --dry-run` first, then the claim, then a written
record of when and at what version. `cargo xtask reserve <name>` generates the
placeholder and `cargo xtask reserve` with no argument lists every name and its
phase (`RUNBOOK.md:830-832`), so the mechanism exists; what `spec` settles is
whether the placeholder or the real crate is what holds the name at this point.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — not this
story's, and it bears on it only through the crate's public description.** A crate
that blocks its executor under some configurations should say so where a stranger
meets it, which is the README, not a doc comment three modules down. That is a
content requirement on the README, not a re-opening of Q3.

**What counts as "structurally unlike"? — not this story's.** It is on disk from
`preflight-and-unlike-axes` (HS-S0074) and is read by the verdict, not by the
package metadata. Named here only so the boundary is explicit: nothing in the
README or the manifest restates or reinterprets the axes.

**Deferred to `spec`:** whether `description` at
`crates/happenstance-ladybug/Cargo.toml:3` — currently *"Not yet implemented."* —
is updated in this story or in the swap story, and what `keywords`/`categories`
inherit from the workspace.

## Decision

At the moment this story opens, `happenstance-ladybug` is a real adapter that
Cargo refuses to publish, carries no licence files, has no README, and holds no
name on crates.io — so the publication project would inherit a crate it cannot
ship for four unrelated reasons, discovered one at a time and late. This story
makes it ready and holds the name: both licence files and a README copied *into*
the crate directory where Cargo will find them, `publish = false` removed and the
crate added to `xtask/src/package.rs`'s `PUBLISHABLE` in the same change because
either alone fails the gate, an explicit `readme = "README.md"` key on the
`happenstance-core` precedent, and the name claimed behind a `cargo publish
--dry-run` with the claim recorded. The spec will cover the four coupled edits,
the `cargo package --list` assertion they exist to satisfy, the README's required
content — projection store only and why, the port's provisional status, the C++
build cost, and the docs.rs failure — and the written record of the name claim,
which has no automated guard. Nothing `[FROZEN]` is touched: PS-3, whether the
port ships behind `unstable-projection`, is `publication-and-positioning`'s to
decide (`RUNBOOK.md:601`), and this story neither anticipates it in the manifest
nor describes the port's maturity in the README as anything other than what
`spec/SPECIFICATION.md` currently marks it.

## The wrong implementation

**A README that is package-complete and untrue.** `crates/happenstance-ladybug/README.md`
exists, `cargo package --list` finds it, the gate step passes, crates.io renders
it — and it describes a LadybugDB adapter for happenstance in the same confident
register as `happenstance-core`'s, saying nothing about the three things a
stranger most needs to know before typing `cargo add`: that this crate is a
**projection store only** and will never offer an event store, and why
(`crates/happenstance-ladybug/src/lib.rs:10-22`); that the projection port itself
is provisional; and that adding this dependency compiles LadybugDB's C++ through
`cxx` and `cmake`, with docs.rs unable to build `lbug` 0.19.1 at all. Every
automated check in the workspace passes, because no check reads English — and the
first impression the project gives of this crate is a promise it has not earned.
This is the mutant with no test, and its guard is a content requirement written
into `spec`: each README claim traceable to `spec/SPECIFICATION.md`'s current
markers or to `lib.rs`'s own framing, checked in the same change. The automated
form of this check exists, but it belongs to a sibling project —
`publication-and-positioning`'s `stranger-install-smoke` — which is a reason to
get the content right here, not a reason to defer it.

**Licences added as symlinks or as `include` paths to the workspace root.**
`ls crates/happenstance-ladybug/` shows `LICENSE-MIT` and `LICENSE-APACHE`, a
local `cargo package --list` on a machine where the symlink resolves lists them,
and the crate looks complete. Cargo will not follow a path outside the crate
directory (`xtask/src/package.rs:152-157`), so on the machine that matters the
published `.crate` ships without its licences — a legal defect, not a cosmetic
one. The existing guard is real and its error message names the fix in as many
words (`package.rs:150-157`), so this story's obligation is to run the whole gate
rather than to add a check; what `spec` must say is that the files are **copied**,
which is a one-word difference that decides the outcome.

**Half of M5.** `publish = false` removed and `PUBLISHABLE` left alone — caught by
the gate with the message at `xtask/src/package.rs:188-195` — or `PUBLISHABLE`
extended while `publish = false` stays, caught from the other side at `:200-202`.
Both are already rejected, and naming them is not decorative: the tempting
intermediate state is to remove `publish = false` in the *driver swap* story
(`_decomposition.md:76` lists it under that crate's `Cargo.toml`) and leave the
registry edit for this one, which reds the gate for a whole slice and invites
someone to put `publish = false` back "temporarily". The rule for `spec` is that
`publish = false` survives until this story and then leaves with `PUBLISHABLE`'s
edit in the same commit.

**The name claimed for a crate that does not exist yet.** Reserving
`happenstance-ladybug` with a `cargo xtask reserve` placeholder before the bodies
are filled in satisfies AC-010's letter — the name is held — and breaks phase 0's
stated rule that a name is reserved when its phase starts, *"the point being that
by now there is a crate to justify it with"* (`RUNBOOK.md:4411-4413`). The
`depends_on` edge on `fill-the-bodies-and-ps-34-disposition` is what enforces it,
and it is a real ordering constraint rather than a convenience.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
