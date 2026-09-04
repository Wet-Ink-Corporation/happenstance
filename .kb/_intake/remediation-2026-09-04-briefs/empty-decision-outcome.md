# Does a command decision that emits no events commit, refuse, or become a third outcome — and what does `Committed` look like afterwards?

Record: **Y-1-empty-decision**. Repository read at `D:/repos/happenstance`, working tree
at `4a7ca16`. Every `crates/happenstance/src/**` path cited below is byte-identical to
the review's pinned `56ef6c5` (`git diff --stat 56ef6c5..HEAD -- crates/happenstance/src/`
is empty), so the audit's citations and mine describe the same bytes.

---

## Why this is owed

**Audit entry Y-1** (`references/evaluation/review-pre-publication-2026-09-03.md:556-586`)
found that one crate answers the same question two ways, and that neither answer is
written down anywhere that governs.

The forcing facts, each verified independently of the audit:

1. **`commit` has no representation for "correct, and nothing to record".** An empty
   decision travels to `store.append(&[], …)` and comes back as an error whose
   top-level message blames the store (`command.rs:305-321`, below).
2. **The crate's own test DSL passes the decision that produces it.** `.then(&[])` on a
   decision that emitted nothing is green (`testing/mod.rs:295`), while `commit` on the
   identical decision returns `Err`. That two-surface disagreement is the finding, and
   it sits inside a module whose header claims production fidelity —
   *"what stops a test from selecting a different set than production does"*
   (`testing/mod.rs:67-68`).
3. **No clause governs.** `spec/SPECIFICATION.md:100-107` puts the typed layer outside
   the clause space: *"the normative architectural specification for happenstance — the
   project, not the crate of that name, which is the typed layer and one consumer of what
   follows."* ES-20 is `[FROZEN]` and governs the **store**, correctly; it says nothing
   about what a caller-side loop should do before it gets there.
4. **No decision atom governs.** `grep -rln "Committed" .kb/` returns **nothing**. The
   string does not appear anywhere in the knowledge base.
5. **The crate is live.** `happenstance`, `happenstance-core` and `happenstance-testkit`
   are on crates.io at `0.2.0-alpha.1` (`review-pre-publication-2026-09-03.md:74-80`,
   *"verified against the registry, not read off the README"*; published by `448e1ac` on
   2026-08-16 per `:2022`). `RUNBOOK.md:4186` — *"Nothing is published, so nothing
   downstream broke"* — is stale and must not be used as the premise here.

Two small citation corrections to Y-1 itself, under the review's own permitted in-place
change (*"repointing a `file:line` citation at the text it already named"*, header
`:1-12`): the `# Errors` bullet for `CommandError::Append` is at `command.rs:214-215`,
not `:216-217` (`:216` is the `Exhausted` bullet); and ES-20's `**[FROZEN]**` marker is
at `spec/SPECIFICATION.md:3548`, not `:3549`. Neither changes the entry's substance.

---

## What is true today

### The line the empty batch reaches

`crates/happenstance/src/command.rs:305-312`:

```rust
        let decided = decide(&model).map_err(CommandError::Refused)?;
        let batch = encode::<B::Event, C, S::Error, D>(&decided, codec)?;

        // From the read, and only from the read.
        let condition = AppendCondition::new(query).after_opt(anchor);

        match store.append(&batch, Some(&condition)).await {
            Ok(position) => return Ok(Committed { position, attempts }),
```

`encode` opens with `let mut batch = Vec::with_capacity(decided.len());`
(`command.rs:342`) and its loop body never executes, so **`command.rs:311` is the line**:
`store.append(&[], Some(&condition))`.

The loop does not hang. `command.rs:320-322`:

```rust
                if !err.is_condition_violated() {
                    return Err(CommandError::Append(err));
                }
```

`NoEvents` is not the condition signal, so control leaves at `:321` on the first attempt.
That is the one thing already right, and ES-20's precedence rule is why
(`spec/SPECIFICATION.md:3550-3554`: without it, *"reporting the violation puts a correct
client into a loop that never terminates"*).

### How the store refuses it

`crates/happenstance-core/src/memory.rs:343` and `:360-361`:

```rust
        // Emptiness first, and **above the lock** — ES-20 `[FROZEN]`.
        …
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
```

`crates/happenstance-core/src/error.rs:219-225`:

```rust
    /// The caller passed an empty batch.
    ///
    /// The specification defines a batch as a non-empty collection of events,
    /// so there is no position an adapter could honestly return. This is a
    /// caller bug, not a store failure.
    #[error("an append must contain at least one event")]
    NoEvents,
```

**ES-20**, `spec/SPECIFICATION.md:3542-3548`, `**[FROZEN]**`:

> `append(&[], _)` MUST return `AppendError::NoEvents`. The emptiness check MUST
> precede the condition check, so that an empty batch under a condition that would
> have been violated still returns `NoEvents`.

Nothing proposed below touches ES-20. The store's answer is correct and the clause stays
where it is.

### The contradiction the caller reads

The wrapper's doc (`command.rs:111`) and the inner variant's doc (`error.rs:222-223`)
state opposite things about the same value:

| | text |
| --- | --- |
| `command.rs:111` | `/// The store failed the append for its own reasons.` |
| `error.rs:222-223` | `/// This is a caller bug, not a store failure.` |

The rendered chain is `appending the decided events failed` → `an append must contain at
least one event`. And `commit`'s `# Errors` list at `command.rs:214-215` offers only
*"if the store fails the append for its own reasons"* — the empty decision is not among
the causes it enumerates (`:207-216`).

### `Committed`, and why its shape is the question

`command.rs:76-88`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "a Committed says where the events landed and how many attempts it took"]
#[non_exhaustive]
pub struct Committed {
    /// The position the store assigned the **last** appended event.
    pub position: SequencePosition,
    /// How many appends were submitted, including the one that succeeded.
    …
    pub attempts: u32,
}
```

`#[non_exhaustive]` blocks the struct literal, so **adding** a field is additive
(`command.rs:73-75` says so in terms). **Changing `position`'s type is not** — the field
is `pub` and read by construction-free code. Note also that `attempts`' own doc — *"How
many appends were submitted"* — becomes false in a no-op that submits none; any
success-shaped option has to re-document that field, not just add beside it.

`CommandError` is `#[non_exhaustive]` (`command.rs:97`), so **adding a variant is
additive**.

### The library's other answer, one file away

`crates/happenstance/src/runner.rs:499-519`, the arm the audit points at:

```rust
        match last {
            // The rows and the checkpoint in the port's **single** `commit`.
            Some(position) => {
                models.commit(batch, projection.id(), position, Authority::Live)…
            }
            // Nothing was applied, so there is nothing to make durable and no
            // checkpoint to move. Committing an empty batch here would claim
            // the run had considered a position it never reached.
            None => {
                models.rollback(batch)…
            }
        }
```

and the success value it reports into, `runner.rs:115-120`:

```rust
pub struct Progressed {
    /// Where the checkpoint was last moved to, or `None` if it did not move.
    ///
    /// `None` after a completed run is an idle one: the query nominated
    /// nothing above the checkpoint, so there was nothing to commit.
    pub through: Option<SequencePosition>,
```

**Does the shape transfer? Partly — and the part that does not is load-bearing.**
The runner's `None` is a fact about the *world*: the stream was exhausted, nobody wrote
code saying "emit nothing", and there is no reachable caller bug that produces it. The
command loop's empty batch is a *return value the caller wrote*, and it has two possible
meanings that the type cannot distinguish: (a) a deliberate "nothing to do", and
(b) a fold arm that forgot to `push`. Importing `Option<SequencePosition>` imports a
silent answer to (b): a decide closure missing a push would report `Ok` and the caller's
program would carry on. Today (b) is loud — with the wrong message, but loud. That
asymmetry is the strongest reason the runner's shape is a precedent rather than a
derivation, and it is not stated in Y-1's Remediation.

### The DSL, and what would move if it changed

`testing/mod.rs:222-225` — `when` folds, decides, and stops; nothing is appended:

```rust
        let outcome = match decide(&boundary) {
            Ok(emitted) => Outcome::Emitted(emitted),
            Err(refusal) => Outcome::Refused(Box::new(refusal)),
        };
```

`testing/mod.rs:293-295` — the green arm:

```rust
    pub fn then(self, expected: &[E]) {
        match &self.outcome {
            Outcome::Emitted(actual) if actual.as_slice() == expected => {}
```

`then`'s own doc (`testing/mod.rs:284-286`) says it exists to stop the *neighbouring*
silent pass — *"comparing a refusal against `&[]` and passing is the silent pass this
method exists to prevent"* — and `then_on_a_refused_decision_panics_naming_the_refusal`
(`tests/dsl_failure_message.rs:515-530`) is the test that holds it. The empty-**emission**
pass is the case that doc does not cover.

Five `.then(&[])` sites exist in the tree. Two are inside `caught(|| …)` and expect a
panic for other reasons (`tests/dsl_failure_message.rs:384`, `:525`). **Three are green
passes that would have to be rewritten** if `then(&[])` started rejecting an empty
emission — and in all three, `&[]` is being used as *"I am not asserting on the
decision"* rather than as a claim about it:

- `src/testing/tests.rs:286-289` — `seeded_bytes_are_the_bytes_commit_writes`
- `src/testing/tests.rs:346-350` — `non_nominated_type_is_skipped_not_an_error`
- `tests/dsl_failure_message.rs:405-411` — `.expect("an empty selection is a well-formed test, not a failure").then(&[])`

That `&[]` currently carries two meanings is itself part of the finding.

### What the workspace's own domain code does

There is **no production site in the workspace where a decision returns `Ok(vec![])`.**
The canonical DCB example routes the idempotent second call to a *typed refusal*,
`examples/course-subscriptions/src/main.rs:445-450`:

```rust
            if seat.subscribed {
                return Err(Refusal::AlreadySubscribed {
                    student: student.id.clone(),
                    course: course.id.clone(),
                });
            }
```

with the variant documented at `:244-250` as *"Invariant 3: a student may not subscribe
to the same course twice."* This cuts against Y-1's discriminator — *"the idempotent
no-op is not an edge case in this domain; it is the ordinary second call of every command
a DCB handler guards"* (`:579`). In the one worked domain this repository owns, the
ordinary second call **refuses**. That is one author's modelling choice rather than a
measurement of what users will do, so it weakens the discriminator without dissolving it.

### Governance

- **No atom would be superseded.** Nothing in `.kb/` mentions `Committed`.
  `.kb/decisions/0012-append-shape-and-preconditions.md` (`kb-decision-0012`, `status:
  accepted`) governs the **store** side and transcribes ES-20 — *"an empty batch is
  refused before any condition is evaluated"* — and is **untouched** by every option
  below. This bullet's headline survives review; its supporting claim did not.

  > **Removed (falsified).** An earlier draft of this bullet read: *"`0020-fold-query-agreement.md`
  > and `0021-…` are the two phase-7 typed-layer decisions; neither reaches `Committed`
  > (verified by grep over `references/adr/0020-*.md` and `0021-*.md`)."* That is false at
  > `HEAD`. `grep -H '^phase:' .kb/decisions/*.md` returns **five** phase-7 atoms — 0020,
  > 0021, **0031**, **0032**, **0033** — all `status: accepted`, all `superseded_by: null`.
  > The grep offered as proof was scoped to the two files it was proving exhaustive, so it
  > could not have found the other three. The claim is removed rather than rewritten,
  > because what it was doing in the brief was licensing reason 3 below to treat the
  > runner's answer as ungoverned.

- **The two sides of the split are not governed symmetrically, and the asymmetry runs the
  opposite way to the one this brief originally assumed.** The `runner.rs` answer stands on
  accepted, immutable governance: **ADR-0031** (`kb-decision-0031`, `status: accepted`,
  `phase: 7`) is the decision that put `run_projection` in `crates/happenstance/src/runner.rs`
  in the first place — *"One runner, in `happenstance`"* — and **ADR-0030**
  (`kb-decision-0030`, `status: accepted`) mints **PS-38** (`spec/SPECIFICATION.md:5564-5566`,
  `[PROVISIONAL]`), whose second sentence governs exactly what a run that committed nothing
  reports: *"a `ProjectionId` no successful `commit` has named MUST read as
  `Checkpoint::NeverRun`."* The clause's own prose says so in terms
  (`spec/SPECIFICATION.md:5560-5562`): *"A rebuild that has committed nothing reads as
  `NeverRun` rather than `Rebuilding`, which is correct."* The `command.rs` side has an enum
  variant's **doc comment** (`error.rs:222-223`) and nothing else: no clause, no atom, no rule.
  Both answers are still in tension, but one of them is written down where it governs and the
  other is not.
- Because ADR-0012 is `status: accepted`, it is immutable: were this decision to be
  written as a change to ES-20 (which nothing here requires), it would have to be a new
  superseding atom, never an edit.
- **The RUNBOOK's ADR queue has no row for it.** `RUNBOOK.md:296-309` lists phase 7 as
  ADR-0020 and ADR-0021, both written and on disk. `RUNBOOK.md:270` — *"0008–0028 are
  free"* — is stale: 0029–0036 now exist. Phase 12 (`RUNBOOK.md:4681`) is where a
  breaking change would have to land, and its exit criteria (`:4720-4738`) do not
  currently name this.

---

## Options

Every option carries a companion obligation on the DSL, because the disagreement is
between two surfaces and fixing one alone leaves it. That companion is priced inside each
option.

### Option 1 — Refuse, by a name that says whose bug it is

Short-circuit above `store.append`; return a new `CommandError` variant naming the empty
decision. `Committed` unchanged. `then(&[])` on an empty emission is made to panic
pointing at the same fact.

- **Costs a caller.** Nothing structural — the empty decision is still an error, with a
  message that names their own code instead of the store's. Three in-tree DSL assertions
  rewrite; a downstream user matching `CommandError::Append(AppendError::NoEvents)` stops
  matching (they already need a wildcard: `CommandError` is `#[non_exhaustive]`). A
  caller who *wants* "nothing to do" as a success must express it through their own
  refusal type `D` and match `CommandError::Refused` — which is what
  `course-subscriptions` already does.
- **Costs an adapter author.** Nothing. No port, clause or conformance rule moves.
- **Semver.** **Additive** on the type surface (new variant on a `#[non_exhaustive]`
  enum; `cargo-semver-checks` reports nothing). The *behaviour* change — an error that was
  `Append` becoming something else, and a green `then(&[])` becoming a panic — is a
  behaviour break, free now and permanent at `0.2.0`.
- **Forecloses.** The cheap version of Options 2 and 3: once `0.2.0` ships, changing
  `Committed` or `commit`'s return type is a real break. What it does **not** foreclose is
  a *second entry point* returning a richer type, which is additive at any time — see the
  Recommendation.

### Option 2 — Optional position on `Committed` (the runner's shape)

Short-circuit above `append`; `Committed::position` becomes
`Option<SequencePosition>`; the empty decision returns `Ok(Committed { position: None, … })`.

- **Costs a caller.** Every existing read of `committed.position` — including the
  overwhelming majority that can never see an empty decision — gains a match or an
  `expect`. The "forgot to `push`" bug becomes a silent success. `attempts`' doc
  (*"How many appends were submitted"*) has to be restated, because none were.
- **Costs an adapter author.** Nothing.
- **Semver.** **Breaking.** A public field's type changes; `#[non_exhaustive]` blocks
  construction, not field reads, so it does not help here.
- **Forecloses.** Going back is breaking again. It also settles, permanently and in the
  type, that "nothing to do" is a *success*, which is a domain-modelling opinion the
  library would be imposing on every consumer.

### Option 3 — Two success shapes

`commit` returns a two-armed outcome — committed, or nothing-to-do carrying `attempts` —
instead of `Committed` directly.

- **Costs a caller.** A match at every call site, including the common one. The no-op
  becomes unignorable, which is the point and also the ceremony. Same `attempts`
  re-documentation as Option 2.
- **Costs an adapter author.** Nothing.
- **Semver.** **Breaking.** `commit` and `commit_with`'s return type changes.
- **Forecloses.** Least of the three by design — it keeps both meanings expressible. It
  is the most honest and the most verbose, and it still does not distinguish "deliberate
  no-op" from "forgot to push".

### Option 4 — Make the empty decision unrepresentable

Change the closure bound from `F: FnMut(&B) -> Result<Vec<B::Event>, D>`
(`command.rs:229`, `:277`) to a non-empty collection.

- **Costs a caller.** Ceremony at every decide site, for a case most of them never hit.
  It also structurally decides that "nothing to do" is a refusal — the caller's only
  remaining home for it is `D`.
- **Costs an adapter author.** Nothing.
- **Semver.** **Breaking**, and the widest-blast-radius of the four: it changes the
  signature every consumer writes against.
- **Forecloses.** Any future success-shaped no-op, permanently. Listed for completeness
  and because it makes explicit what Option 1 does implicitly; I do not think the
  ergonomics survive contact.

---

## Recommendation

**Option 3**, with medium confidence.

**This recommendation flipped.** The brief first recommended Option 1, on four reasons.
Review falsified one of them outright, inverted a second, and showed a third to be
underpriced; what survives no longer discriminates. The flip and its causes are recorded
in full at the foot of this document.

Why it beats the others:

1. **It is loud earlier and more completely than Option 1.** Option 1 catches the
   forgot-to-`push` bug at *runtime*, on the append path, and only when that branch
   actually runs. Option 3 catches it at *compile time*, at every call site, by forcing the
   caller to name which arm they are in — and a `#[must_use]` two-armed outcome cannot be
   discarded by `…await?;` without a warning, the same guard `Committed` already carries
   (`command.rs:136`). This is what the brief's own Option 3 cost line says — *"The no-op
   becomes unignorable"* — and it is the reverse of what the old reason 2 asserted.
2. **It unifies the crate's two answers instead of entrenching the split, and it unifies
   them onto the governed side.** `runner.rs`'s `None` arm stands on ADR-0031 (accepted;
   it is the decision that put `run_projection` in that file) and on PS-38 by way of
   ADR-0030 (*"a `ProjectionId` no successful `commit` has named MUST read as
   `Checkpoint::NeverRun`"*, `spec/SPECIFICATION.md:5564-5566`). `error.rs:222-223`'s
   *"caller bug, not a store failure"* is a doc comment on an enum variant, governed by
   nothing. Option 3 is the shape that lets one crate give one answer.
3. **`error.rs:222-223`'s conclusion does not travel without its premise, and Option 3 is
   the option that supplies the missing home.** The variant's reasoning is *"there is no
   position an adapter could honestly return"* — a constraint on the **store port**, where
   every success carries a `SequencePosition`. Option 3 lifts precisely that constraint at
   the typed layer by giving the outcome an arm that carries no position. ES-20 and
   `AppendError::NoEvents` remain exactly right one crate down; nothing about them obliges
   the layer above to spell the same fact as an error.
4. **The window is real and this is the option that needs it.** Option 3's break is free
   today — `0.2.0-alpha.1` is a pre-release and no `^0.2` requirement resolves to it — and
   permanent after `0.2.0`. Option 1's escape hatch does not price out as cheaply as the
   brief claimed: a no-op-tolerant *second door* has to exist in both codec arms, taking the
   crate from two public entry points to four, and the axis those doors would differ on
   (how an empty return is treated) is far less legible than the axis the existing pair
   differs on (`command.rs:234-239` records the two-door choice as a deliberate one, made
   on a codec; it is evidence the pattern is defensible on *that* axis, not that it
   multiplies freely).
5. **It costs no clause and no atom.** ES-20 stays frozen and untouched; ADR-0012 stays
   accepted and untouched; no accepted atom is superseded. True of every option here, so it
   is a floor rather than a discriminator — it is kept because it bounds the blast radius,
   not because it favours anything.

**The strongest argument against it, in its own words:**

> The empty batch has two possible meanings that the type cannot distinguish: (a) a
> deliberate "nothing to do", and (b) a fold arm that forgot to `push`. Option 3 does not
> separate them either — its own cost line concedes *"it still does not distinguish
> 'deliberate no-op' from 'forgot to push'"*. What it does is convert (b) from an error
> into a success value with an extra arm, and a caller who matches that arm with `_ => ()`
> has bought the ceremony and none of the safety. The runner's precedent does not carry:
> its `None` is a fact about the world — the stream was exhausted, and no reachable caller
> bug produces it — while the command loop's empty batch is a value the caller's own
> closure returned. Importing the runner's shape imports a silent answer to a bug the
> runner cannot have. Meanwhile Option 1 is additive on the type surface, spends no
> one-shot window, and leaves a success-shaped no-op reachable later; Option 3 spends the
> window on a shape whose central weakness it admits.

That argument is not answered, only outweighed. Its (a)/(b) point stands on its own and is
the reason the confidence here is medium rather than high: the case for Option 3 is that a
warning at every call site beats an error on one path, not that Option 3 makes the bug
impossible. A decider who judges compile-time noise at the common call site to be the
worse cost, and who is willing to pay four entry points later, should still take Option 1
— but they should take it knowing the runner's answer is the governed one and theirs is
not.

---

## Cost of delay

**The deadline is real, and it binds the recommended option.** Y-1's Semver field says the
change *"costs nothing today and is permanent after `0.2.0`"* — that is true of Options 2,
3 and 4, which includes the option recommended above; it is not true of Option 1, whose
type-surface change is additive forever. Splitting it:

| Piece | Class | Free until |
| --- | --- | --- |
| New `CommandError` variant (`#[non_exhaustive]`, `command.rs:97`) | additive | **forever** |
| The behaviour change it causes (an error that was `Append` is not) | behaviour break | `0.2.0` |
| `then(&[])` on an empty emission starts panicking | behaviour break on a published test surface | `0.2.0` |
| `Committed::position` → `Option<…>` (Option 2) | **breaking** | `0.2.0` |
| `commit`'s return type gains a second shape (Option 3) | **breaking** | `0.2.0` |
| Closure bound change (Option 4) | **breaking** | `0.2.0` |
| Correcting `command.rs:214-215`'s `# Errors` list and `:111`'s doc | none | forever |
| A *second, additive* entry point with a richer success type | additive | **forever** |

`0.2.0-alpha.1` is a pre-release, so no `^0.2` requirement resolves to it
(`review-pre-publication-2026-09-03.md:78-80`) — the type-level breaks are close to free
today, which is what makes the recommended Option 3 affordable *now* and not later. The
last row remains true — the success-shaped no-op is not permanently lost by choosing
Option 1, only its cheap in-place spelling is — but it is worth less than the brief first
priced it at: buying it back costs a second door in both codec arms, four public entry
points where there are two.

The doc corrections are free forever and should not wait on this decision either way.

---

## What this does not settle

- **How `then(&[])` reads afterwards.** Whether the DSL grows a `then_nothing()`, whether
  `then(&[])` panics with a pointer at this decision, or whether the three in-tree "don't
  care" call sites get a different idiom entirely, is a design question that follows from
  the option but is not answered by it. The DSL's *deadline* (a published surface, a
  behaviour break) is the same either way.
- **Whether `decide` should be able to say "nothing to do" and "refused" as distinct
  things.** Options 1 and 4 push both into the caller's `D`; Options 2 and 3 give the
  first its own channel. Neither is settled by picking an outcome for the empty batch.
- **ES-20, `AppendError::NoEvents`, and the store port.** All untouched. No option here
  needs a clause amendment, and none needs a change to `.kb/decisions/0012-…`, which is
  `accepted` and therefore immutable.
- **Whether this decision needs a clause at all.** `spec/SPECIFICATION.md:100-107` puts
  the typed layer outside the clause space, so there is nothing to hang a maturity marker
  on. ADR-0020 and ADR-0021 are precedent for a typed-layer decision with no clause; that
  precedent is not the same as a rule, and someone has to say which it is.
- **Where it goes in the RUNBOOK.** `RUNBOOK.md:296-309`'s queue has no row for it and
  `:270`'s "0008–0028 are free" is stale (0029–0036 exist). Phase 12's exit criteria
  (`:4720-4738`) do not name it. Numbering and scheduling are the RUNBOOK's, not this
  brief's.
- **Anything about `happenstance-sync`'s ingest path.** Whether an ingested empty batch
  raises the same question is unexamined here; nothing is built.

---

## Revision record

Revised after adversarial review of the first draft. Two critiques were filed; neither
accepted the original recommendation, and both were checked against the tree at `4a7ca16`
before anything here moved.

**1. The recommendation flipped: Option 1 → Option 3.**
The original four reasons did not survive. Reason 2 — *"Options 2 and 3 convert it into a
success value that most callers will ignore"* — was **falsified for Option 3** and
contradicted by the brief's own Option 3 cost line (*"The no-op becomes unignorable"*): the
loudness discriminator was measured on Option 2 and carried to Option 3 unchecked. A
two-armed `#[must_use]` outcome is checked at compile time at every call site, which is
earlier and broader than Option 1's runtime error on one branch. Reason 3 was **inverted**
(see item 2). Reason 1's escape hatch was **underpriced**: a no-op-tolerant second door
needs both codec arms, taking two public entry points to four. Reason 4 turned out to be
true of every option, so it discriminates nothing and is now labelled a floor. With three
of four reasons gone, the case the brief itself called "close" is the one that stands, and
the option it favours costs a break that is free only until `0.2.0`.

**2. A falsified claim was removed from Governance, not rewritten around.**
The bullet *"`0020-fold-query-agreement.md` and `0021-…` are the two phase-7 typed-layer
decisions; neither reaches `Committed` (verified by grep over `references/adr/0020-*.md`
and `0021-*.md`)"* is false at `HEAD`. `grep -H '^phase:' .kb/decisions/*.md` returns five
accepted phase-7 atoms: 0020, 0021, 0031, 0032, 0033. The proof cited was scoped to the two
files it was claiming to be exhaustive. The claim is deleted and the deletion is stated in
place, because its function in the brief was to license reason 3's treatment of the runner
as ungoverned. **ADR-0031** is the accepted decision that placed `run_projection` in
`crates/happenstance/src/runner.rs` — the very file the brief uses as its counter-precedent
— and **ADR-0030** mints **PS-38**, whose second sentence governs what a run that committed
nothing reports. A replacement bullet states the asymmetry the right way round: accepted
atoms plus a clause on the runner side, a doc comment on the `error.rs` side.

**3. Reason 3's residue was re-stated with its premise attached.**
The original reason propagated `error.rs:222-223`'s conclusion (*"a caller bug, not a store
failure"*) while dropping its stated premise (*"there is no position an adapter could
honestly return"*). That premise is a **store-port** constraint — every store success
carries a `SequencePosition` — and Option 3 is precisely the option that lifts it at the
typed layer by giving the outcome an arm carrying no position. The reason now appears
under its own premise, and now argues for Option 3 rather than against it.

**4. The old "strongest argument against" was accepted, so a new one was written.**
The block arguing that Option 1 entrenches the split, quoting `runner.rs:511-513`, is no
longer an argument against the recommendation — it is substantially the argument that
caused the flip, and its governance is now stated in Governance. The Recommendation
section carries a new strongest-argument-against, built from the part of the original
analysis that review did **not** touch: the (a)/(b) ambiguity at lines 186-195, which
stands unchanged and is why confidence remains medium rather than rising.

**What did not change.** The headline governance claim — no option supersedes an accepted
atom, ES-20 stays frozen, ADR-0012 stays untouched — survives review and is unedited. The
options and their cost lines are unedited. The Cost-of-delay table is unedited; only its
framing paragraphs moved, because the recommended option is now one the deadline binds.
Nothing in "What is true today" moved. No file other than this brief was touched.
