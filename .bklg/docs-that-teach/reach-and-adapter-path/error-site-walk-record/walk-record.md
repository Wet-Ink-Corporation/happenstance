# Walk record — the error site and the reasoning account

Dated, first-person records of the walk project `AC-009` asks for: someone who did not write the
`store.rs` rewrite reproduces the `error[E0034]` collision at the pinned toolchain, and reaches the
adapter reasoning account from the file and the message in front of them and nothing else.

## How this file is kept

This section is here before the first entry, and it binds whether or not any entry failed. It is
what makes the entries capable of failing; a record that cannot fail is decorative.

- **Entries are appended, never edited.** A closed entry stays closed. A later walk that succeeds
  where an earlier one failed is a new dated entry citing the earlier one, never a correction of it.
- **A hop that resolves nowhere, or takes two where one was designed, closes its entry as a failed
  walk.** The finding is routed to the story slug that owns the surface — `store-error-site-rewrite`
  owns the pointer at the error site, `adapter-reasoning-account` owns the destination page — and
  nothing is repaired in the commit that records it. A witness who edits what they are witnessing
  has destroyed the observation.
- **A blocked walk is not a failed walk.** If the destination does not exist yet, the story halts
  and reports the missing dependency. It does not walk a simulated surface, and it does not record a
  scheduling fact as a teaching failure.
- **Every walk is keyboard-only, and the entry names the keys it pressed.** "Keyboard-only" with no
  key named anywhere in the entry is an assertion, not an observation.
- **The scratch that reproduces the collision never enters the repository.** It does not compile. It
  lives in the entry as verbatim source plus the command that ran it, so a third party can re-run it
  outside the tree rather than trusting the paste.

## Walk of 2026-08-20

Outcome: **completed, one hop, no dead end.** Three observations are recorded below and none is
routed as a defect; each was checked against the signed-off design and found to be a decision that
design had already taken and written down.

Tree walked: `d14affd2ab177231eed41e2e21aec9d2da494046`, working tree clean at the time of the walk.

### Walker and relationship to the work

The walker is **Claude Opus 5, running as the implementation context for story
`error-site-walk-record` (HS-S0157)**, on 2026-08-20.

**Relationship, in one sentence:** the walker authored neither `store-error-site-rewrite` (landed in
`f2c7dbe`, amended by `76e9424`) nor `adapter-reasoning-account` (landed in `af9a241`) — both were
written by separate implementation contexts whose reasoning, notes and intermediate artefacts this
context has no access to — and before t=0 it had read this story's own `spec.md`, which describes
what the destination should contain but not where it is filed, and had not opened
`crates/happenstance-core/src/store.rs`, `docs/adapter-reading-order.md`, either dependency story's
`spec.md` or `report.md`, or the composition tables in `_design.md`.

**The authorship cross-check is weaker here than the criterion, and the gap is stated rather than
left to be discovered.** Both dependency commits are authored `Ryan Britton` and carry a
`Co-Authored-By: Claude Opus 5` trailer, so `git log` over `crates/happenstance-core/src/store.rs`
and `docs/adapter-reading-order.md` surfaces the walker's model name on the work it is witnessing.
What separates walker from author here is context, not identity: no shared memory, no shared
transcript, and no access to the reasoning either dependency was written under. That is a real and
material weakening of the separation project `AC-009` asks for, and a reviewer who judges it
insufficient should reject this entry rather than discount it — the criterion is this stated
relationship, not the trailer. No uncontaminated separate walker was available to this session: it
has no facility for spawning an independent context, and `EC-004`'s alternative — halt and escalate
— was weighed and rejected because the walker did not author either dependency in the sense the
criterion is about.

### Cold-start protocol

**Permitted at t=0:** the walker's own failing build output, produced by the walker, and the path
`crates/happenstance-core/src/store.rs`. Nothing else.

**Forbidden at t=0:** the backlog, this story's sibling specs and reports, the two dependency
stories' artefacts, `_design.md`'s composition tables, the authors, and prior knowledge of where the
reasoning account was filed.

**Contamination, disclosed.** Two, both real, both stated in the walker's own words.

1. **The walker read this story's `spec.md` before t=0**, because it is the implementation context
   for the story and the spec is its source of truth. That spec states what the destination *should*
   carry — an answered-need line, `n of 6` position markers, a `MemoryEventStore` caveat — and does
   not state where the destination is. What it bought: the arrival check below knew what to look
   for, so it is a *verification* of those three properties rather than a discovery of them.
2. **The walker saw the destination's path before opening `store.rs`.** Running the authorship
   cross-check that this record's own walker-identity stanza requires (`git show --stat` over
   `af9a241`) printed that commit's file list, which names `docs/adapter-reading-order.md`. This
   happened before t=0 and was not anticipated. What it bought: it removed "could the destination be
   found at all?" from the walk. What mitigates it, and is checkable by anyone: `store.rs:65-67`
   names that exact path in prose, so the surface under test supplies the destination itself — the
   walker did not need the leaked knowledge and can demonstrate that the pointer would have carried
   a walker who never had it. What it cannot have touched: Observation A, which is entirely a
   property of `store.rs`, was made before any hop, and is quoted verbatim so a third party can
   re-read the file and disagree.

The honest consequence is that this entry is a **weaker record than an uncontaminated walk**, and it
is recorded as one. It is not void: the contamination is partial and disclosed, and the two
observations the walk exists to make — does the file unblock in place, and does the pointer resolve
in one hop — are both properties of files that a reader can re-check.

### Reproduction

The collision was reproduced first-hand, outside the repository, at the pinned toolchain. It is not
quoted from `store.rs`.

The scratch lives in a temporary directory with a path dependency on the worktree's
`happenstance-core` and its own `[workspace]` table, so it is never absorbed into the workspace and
never lands under `crates/`. It is reproduced here verbatim and was not committed.

`Cargo.toml`:

```toml
[package]
name = "e0034-walk"
version = "0.0.0"
edition = "2024"

[dependencies]
happenstance-core = { path = "PATH-TO-THIS-WORKTREE/crates/happenstance-core" }

[workspace]
```

`rust-toolchain.toml`, matching the repository's pin:

```toml
[toolchain]
channel = "1.97.1"
```

`src/main.rs`:

```rust
// Writing an adapter, so both flavours of the port are in scope.
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, SendEventStore};

fn main() {
    let store = MemoryEventStore::new();
    let _stream = store.read(&Query::all(), ReadOptions::new());
}
```

The exact invocation, run from the scratch directory:

```console
$ cargo build
```

The toolchain, as `rustc -V` printed it in that directory:

```console
rustc 1.97.1 (8bab26f4f 2026-07-14)
```

The whole `error[E0034]` stderr, pasted as rustc emitted it. Long lines are left long: the second
`= note:` line is 109 characters and takes a horizontal scrollbar rather than a reflow, because a
transcript edited to fit stops matching what the reader has on screen.

```text
error[E0034]: multiple applicable items in scope
 --> src\main.rs:6:25
  |
6 |     let _stream = store.read(&Query::all(), ReadOptions::new());
  |                         ^^^^ multiple `read` found
  |
  = note: candidate #1 is defined in an impl of the trait `SendEventStore` for the type `MemoryEventStore`
  = note: candidate #2 is defined in an impl of the trait `EventStore` for the type `TraitVariantBlanketType`
help: disambiguate the method for candidate #1
  |
6 -     let _stream = store.read(&Query::all(), ReadOptions::new());
6 +     let _stream = SendEventStore::read(&store, &Query::all(), ReadOptions::new());
  |
help: disambiguate the method for candidate #2
  |
6 -     let _stream = store.read(&Query::all(), ReadOptions::new());
6 +     let _stream = EventStore::read(&store, &Query::all(), ReadOptions::new());
  |

For more information about this error, try `rustc --explain E0034`.
```

Both `= note:` candidate lines are present, and `TraitVariantBlanketType` is the type named in
candidate #2 — the string a reader would search, and the one the measured baseline recorded as
absent from `store.rs`.

### Observation A: in place, before any hop

This observation was made with `store.rs` open and nothing followed. No link was taken, no search
was run for the reasoning account, and the destination page was not opened until after this section
was written.

**Did `store.rs` alone name which call was ambiguous, and how to resolve it? Yes, both.**

Which call, quoted verbatim from `crates/happenstance-core/src/store.rs:52-54`:

> In words, because a caret does not survive a search hit or a screen reader: `store.read(…)` is
> ambiguous because both [`EventStore`] and [`SendEventStore`] are in scope and each of them
> supplies a `read`.

How to resolve it, quoted verbatim from `crates/happenstance-core/src/store.rs:56-58`:

> Import only the one you are binding on — [`EventStore`] in almost every case. If you genuinely
> need both in one module, disambiguate with fully-qualified syntax:
> `SendEventStore::read(&store, &query, options)`.

Both passages sit **above** the pointer to the reasoning account, which is the last thing in the
section at `store.rs:65-67`. The reader is unstuck before the offer is made.

**Falsification, run deliberately.** With the reasoning account hypothetically deleted, does the
quoted sentence still unblock the reader? Yes. It names the method (`read`), names both traits, says
which of the two to import, and gives the fully-qualified escape hatch for the case where both are
genuinely needed. Nothing in the fix depends on the destination existing. The page works without the
link, which is the shape invariant 1 asks for.

The plain-words naming also does not depend on the caret line. The caret row is spatial and
linearises to nothing under a screen reader or in a search-result snippet; the sentence at `:52-54`
carries the same information as prose, and it is that sentence, not the caret, that answered the
question.

### Hops

Keyboard-only. No pointing device was used at any point in this walk: no mouse, no trackpad, no
touch. Every affordance below was reached by typing at a prompt or in a search field and pressing
`Enter`.

| # | Surface on screen | Affordance used, and the keys | Destination passage reached |
| --- | --- | --- | --- |
| 1 | the walker's own `cargo build` stderr, in the terminal | typed the string rustc printed into a workspace search — `rg -F 'TraitVariantBlanketType'` — at the shell prompt, `Enter`. No pointing device | `crates/happenstance-core/src/store.rs:44`, the `= note:` candidate-#2 line inside `## Import one flavour, not both`. Returned 20 matches in 8 files, of which **9 are in `store.rs`**; the measured baseline for this string recorded six workspace hits and **none** in `store.rs`. Run as a measurement of the search path, not as how the file was located — the cold start supplied the path |
| 2 | `crates/happenstance-core/src/store.rs`, module doc, section `## Import one flavour, not both` | the pointer at `store.rs:65-67`, which names its destination as a path in prose rather than as an activatable link; keyboard: `/` to search within the open file for `docs/adapter-reading-order.md`, then the same path typed into the editor's open-file prompt, `Enter` | `docs/adapter-reading-order.md`, arriving at the page head — its answered-need line — and from there the section `## 4 of 6 — why there are two flavours`, which is the passage that answers the question the stall arrived with |

**One hop from `store.rs`, and it is row 2.** Exactly one row above is sourced at `store.rs`, and
its destination is the reasoning account. There was no intermediate page, no redirect and no second
lookup.

**A third search affordance, recorded as an attempt rather than a hop.** `store.rs:139-140` carries
`#[doc(alias = "E0034")]` and `#[doc(alias = "TraitVariantBlanketType")]` on `pub trait EventStore`.
The walker built the documentation locally with `cargo doc -p happenstance-core --no-deps` and
confirmed that both alias strings are present in the generated search index under
`target/doc/search.index/`, so a reader who focuses rustdoc's search box with `S` or `/` and types
either string does get a hit. The limitation is stated rather than glossed: the walker read the
generated index, and did not drive a browser, so what is verified is that the keys are indexed —
not what the rendered result list looks like.

### Arrival check

Checked against the destination as rendered, not assumed from the pointer.

**The stated answered-need**, verbatim from `docs/adapter-reading-order.md:3`:

> **Answers:** `explanation` — In what order do I read what already exists, to build an adapter?

**Position marker of the section landed on.** The pointer carries no fragment, so the arrival is at
the page head rather than mid-sequence: the answered-need line is the first thing below the H1, and
the six-entry reading order is above the first section. The passage answering the question the
walker arrived with is headed `## 4 of 6 — why there are two flavours`, and every one of the six
section headings carries its position in the sequence — `## 1 of 6` through `## 6 of 6`. A reader
who did land mid-sequence could name where they were from the heading alone, without scrolling up.

**Walk-backable.** Entry 4 states what it assumes was read first, in its own words: "Fourth, not
first: you meet this as `error[E0034]` before you meet it as a rule." The hop can be retraced from
the destination without browser history.

**The `MemoryEventStore` caveat, checked where `memory.rs` is first sequenced. Present, and in
position.** `memory.rs` is sequenced at entry 1, and the caveat is at entry 1 — not deferred to a
closing note. From `## 1 of 6 — the runnable loop, and what it is not`:

> **And it is not an adapter.** That file's own three-reasons list says what `MemoryEventStore` is:
> the oracle the conformance suite is validated against, the thing that makes this crate's examples
> runnable, and the store application code is written against before a real one exists.

It names both properties the criterion asks for — conformance oracle and reference implementation —
and states plainly that it is not an adapter. It is also paired in the same breath with a store at
the other end of the storage axis, the `PgStore` in `standards/rust/91-adapter-authoring-recipe.md`,
and with `references/adapter-shapes.md`. The numbered list entry above it carries the same warning
in six words: "First because it runs, caveated because it is not an adapter."

### Findings, routed by slug

**No finding is routed, and no hop failed.** Three things were noticed and each was checked against
the signed-off design before being written up; all three are decisions that design had already taken
and recorded. They are written down anyway, because a later reader who notices them and cannot find
this paragraph will re-open them as defects.

1. **The installed fence carries one `help:` hunk; rustc 1.97.1 emits two.** The walker's own stderr
   above has a `help:` hunk for candidate #1 and one for candidate #2; the fence at `store.rs:36-49`
   carries only candidate #2's. The error code, both `= note:` lines and `TraitVariantBlanketType`
   match exactly. **Not routed:** `_design.md`'s density-budget row for `store-module-error-site`
   authorises precisely this cut in advance — "Dropping candidate #1's `help:` hunk is yield rule
   (3) already exercised; nothing further may be cut." It is a recorded yield, not drift, and the
   arrow line's path and column differ only because the walker's scratch is a binary where the
   fence's was a library.
2. **The pointer is a named path, not an activatable link.** `store.rs:65-67` says so in its own
   words — "Named, not linked: it is a page, not an item." The keyboard cost is real and is recorded
   as a cost: a reader in rendered rustdoc cannot press `Enter` on it and must retype the path into
   an editor or a repository browser. **Not routed:** the pointer register's row P3 in `_design.md`
   sanctions "the named-unlinked form", guarded by the registration check that the page exists at
   that path — which it does. The hop still resolves, and still resolves in one.
3. **The `#[doc(alias)]` keys land on the trait, not on the module doc that carries the
   explanation.** Both attributes sit on `pub trait EventStore` at `store.rs:139-140`, so a rustdoc
   searcher typing `E0034` arrives at the trait page and reaches `## Import one flavour, not both`
   one step further on, at the module page. **Not routed:** `store.rs:116-119` states the rule this
   was installed under — an alias "is a *search key* and not a pointer: it moves a reader who is
   already searching and does nothing for the reader who is reading, so it never substitutes for the
   cross-reference at the stall in this module's documentation." The walk's designed start is the
   reader who already has the file open, and for that reader the explanation is zero hops away.

### What this record does and does not claim

**It claims:** that on 2026-08-20, at tree `d14affd`, a walker who wrote neither dependency
reproduced `error[E0034]` first-hand at the pinned 1.97.1 toolchain, was unblocked by
`crates/happenstance-core/src/store.rs` alone before following anything, and reached the adapter
reasoning account in exactly one keyboard-only hop, arriving at a passage it could name and
verifying the answered-need line, the position markers and the `MemoryEventStore` caveat in place.
The path exists and it carries a non-author.

**It does not claim that a stranger finds it.** This walker is an insider. Insider knowledge routes
a reader around rough spots that would stop someone else, and two contaminations are disclosed above
that make this particular walk easier than a clean one. Nothing here is evidence about a new user, a
first-time reader, an outsider or an evaluator meeting this repository for the first time. That
claim needs a different instrument and belongs to `comprehension-evidence` (HS-P0024) and its
friction log; spending it here in one enthusiastic sentence would pre-empt a sibling project's only
method, and it cannot be un-spent.

**It does not claim the gate checked any of this.** Nothing in `cargo xtask ci` verifies
keyboard-only reachability or self-describing link text. The gate commands run for this story prove
only that the tree was not disturbed by writing this record.
