---
item: HS-S0029
stage: discover
created: 2026-08-12T13:01:52.386Z
updated: 2026-08-12T13:01:52.386Z
template_sig: 86ce4036
rendered_sig: 4d3b338a
---

# Discover — The worked example, rewritten on the typed layer and actually executed

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: rewrite `examples/course-subscriptions/` onto `happenstance` — its manifest dependency moves off `happenstance-core`, `parse_capacity` and the `format!("…").into_bytes()` payload are **deleted rather than wrapped**, `commit` is deleted *into* the command loop, `main`'s observable steps survive verbatim — and add the integration test that actually **executes** the binary via `CARGO_BIN_EXE_course-subscriptions`, because nothing in the gate runs `main` today and AC-003 is an execution claim | `_storymap.md:60` (M6 row) | Two halves: the rewrite, and the instrument that makes the rewrite checkable at all |
| AC-003 — `cargo run -p course-subscriptions` completes the canonical DCB cycle with **no `todo!()` reached**, using typed events and a decision model; no `format!("…").into_bytes()` payload construction and no hand-rolled `parse_capacity` survive | `project.md:170-173`; DoD 1 at `project.md:224-225` | "Survive" is a textual claim about the file as well as a behavioural one about the run |
| `depends_on: command-loop` — supplies `commit`/`commit_with`, `Retry`, `Committed`, `CommandError`; the example's own `commit` is deleted into it | `_storymap.md:60`, `:127-129` | The example cannot be rewritten before the thing it is rewritten onto exists |
| `depends_on: decision-model-composition` — supplies the tuple `Boundary` impls the `subscribe` case needs, since that case is already a two-item query spanning two concerns | `_storymap.md:60`; `_decomposition.md:419` (AC-004 row) | This is where AC-004 is *exercised*, though `decision-model-composition` owns it |
| The four call sites the typed layer must absorb, all present today: `define_course` (`:86`), `subscribe` (`:113`), `unsubscribe` (`:177`) and the shared `commit` (`:207`) | `_decomposition.md:460-467`; `examples/course-subscriptions/src/main.rs:86,113,177,207` | `commit`'s own doc comment names its destination — *"this is the second half of every DCB command handler, and it is identical every time — which is exactly why it belongs in the typed layer"* |
| The exact code to delete: `parse_capacity` at `:231`, whose doc says it is *"hand-rolled because the contract layer stores opaque bytes and this example deliberately takes no serialisation dependency — decoding is the typed layer's job, and that layer does not exist yet"*; and the payload at `:97-100`, `format!("{{\"capacity\":{capacity}}}").into_bytes()` | `examples/course-subscriptions/src/main.rs:97-100`, `:226-233` | The comment states its own expiry condition. This story is that condition arriving |
| The hazard the rewrite closes: `subscribe` names `[COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED]` in a `QueryItem` and then folds the same three by hand in a `match` on `sequenced.event_type().as_str()`, with a `_ => {}` arm | `examples/course-subscriptions/src/main.rs:114-125`, `:140-156` | Two hand-written statements of one event set, plus a catch-all that absorbs divergence silently |
| `main`'s observable steps survive **verbatim** — they *are* the behaviour AC-003 asserts; the transcript keeps its `== … ==` markers, its 3-space indented result lines and the literal `rejected: ` prefix | `_decomposition.md:418`; `_design.md:793-800`, `:168-186` | The diff is beneath `main`, not in it |
| The example's manifest dependency moves to `happenstance`. *"An example still depending on `happenstance-core` proves nothing about the crate a user installs"* | `_decomposition.md:468-470` (Composition roots 3) | The `use` block at `examples/course-subscriptions/src/main.rs:24-27` is its whole surface today |
| **No instrument exists today that runs the binary.** `cargo test --workspace --all-features` compiles the example but never executes `main`; `xtask/src/main.rs` has no `course-subscriptions` reference at all. `std::process::Command::new(env!("CARGO_BIN_EXE_course-subscriptions"))` closes it with no new dependency | `_decomposition.md:809-826` (testing brief) | DoD 1 uses an execution verb. Until this lands, AC-003 cannot be honestly checked off |
| AC-013's measurement is taken over **this** rewritten example — boilerplate versus domain logic — not over the design's doctest; the design records a falsifiable prediction of 2.4:1 against the domain and expects the verdict to be *"in scope"* | `_design.md:1104-1111`; `RUNBOOK.md:524` | The rewrite is the measurement's substrate, and `defect-log-and-macros-verdict` reads it |
| Transcript budgets that constrain the rewrite: 80 columns, position and event type never truncate, tags wrap to a continuation line indented 8; a long event type overruns the 22-character type column and the wrap rule does not cover it (a finding the mock recorded) | `_design.md:857-864`, `:1217-1220` | A domain rename in the rewrite can break the transcript's column alignment |
| No colour, no spinner, no in-place rewriting; the example runs to completion with stdout **piped**, which is how the gate runs it | `_design.md:170-186`, `:923` | The execution test pipes stdout, so any TTY assumption fails it immediately |

## Questions

**Answered here.**

- *Deleted or wrapped?* Deleted. `parse_capacity` and the `format!` payload go away
  entirely, and `commit` is deleted *into* the library rather than reimplemented around it
  (`_storymap.md:60`; `_decomposition.md:460-467`). A wrapper preserves the behaviour and
  loses the point.
- *Does `main` change?* Its observable steps survive verbatim — they are the behaviour
  AC-003 asserts (`_decomposition.md:418`). The rewrite happens beneath them.
- *How is AC-003 actually observed?* By an integration test under
  `examples/course-subscriptions/tests/` that runs the binary through
  `env!("CARGO_BIN_EXE_course-subscriptions")` and asserts on exit status **and** stdout. No
  new dependency is needed; `assert_cmd` is absent from the workspace and none is required
  (`_decomposition.md:809-826`).
- *Does the example use composition?* Yes — `subscribe` is already a two-item query spanning
  the course's seats and this student's history (`examples/course-subscriptions/src/main.rs:114-125`),
  which is the multi-model case AC-004 names.

**Deferred to `spec`.**

- *Whether the execution test lands in `tests/` (collected by `cargo test`, therefore already
  inside `cargo xtask ci`'s existing test step) or as an explicit new `xtask` step.*
  `_design.md`'s call per the testing brief (`_decomposition.md:822-826`). Discovery's
  position: `tests/` needs no gate wiring and is therefore harder to forget.
- *How many decision models the rewrite composes.* Two is the shape already present; three
  would exercise a wider tuple arity. Spec's call, constrained by the transcript budget.
- *What the execution test asserts about stdout.* Discovery fixes the floor — exit status
  zero, every `== … ==` section marker present in order, and the literal `rejected: ` prefix
  on the refusal steps — and leaves the exact matching to spec.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.
`compile-fail-proof-artefact` depends on **this** story — its diagnostic must land on the
example's own `match` arm — so a rewrite that leaves no `match` over a domain enum would
strand the sibling.

## Decision

The problem this slice solves is that the canonical DCB worked example is currently written
against the contract crate, which means it demonstrates the thing this library is *not*
supposed to make people do: build payloads with `format!`, parse them back with a
hand-rolled function whose own doc comment apologises for existing, name the event set once
in a `Query` and again in a `match` with a catch-all arm, and reimplement the
read-decide-append loop per handler without a retry. This story rewrites it onto the typed
layer so that the example demonstrates the library, deletes the four pieces of scaffolding
that only existed because the typed layer did not, and — because nothing in the gate has
ever executed `main` — adds the integration test that actually runs the binary, since DoD 1
and AC-003 are execution claims and a compile is not one. The spec for this story covers the
manifest move to `happenstance`, the domain enum and its `DomainEvent` impl, the decision
models and their composition for the `subscribe` case, the deletion of `parse_capacity`, the
`format!` payload and the per-handler `commit`, the preservation of `main`'s observable steps
and the transcript's exact shape within its 80-column budget, and the
`CARGO_BIN_EXE_course-subscriptions` test with what it asserts about exit status and stdout.
No `[FROZEN]` clause is amended; the example consumes the contract only through
`happenstance`, which is the point of the move.

## The wrong implementation

**The mutant: wrapping instead of deleting.**

```rust
impl DomainEvent for Enrolment {
    fn encode<C: Codec>(&self, _codec: &C) -> Result<Bytes, CodecError> {
        match self {
            // "the codec is overkill for two fields"
            Self::Defined { capacity } =>
                Ok(format!("{{\"capacity\":{capacity}}}").into_bytes().into()),
            // …
        }
    }
    fn decode<C: Codec>(_c: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        Ok(Self::Defined { capacity: parse_capacity(data) })   // moved, not deleted
    }
}
```

The example now uses `DomainEvent`, `DecisionModel` and `commit`. It compiles. `cargo run -p
course-subscriptions` completes the full DCB cycle with no `todo!()` reached. The transcript
is byte-identical to today's, so the new execution test — asserting exit status and stdout —
**passes**. `cargo xtask ci` is green. AC-003 reads as satisfied on every clause a machine
can check: typed events, a decision model, a completed cycle.

It is wrong because the two things AC-003 explicitly requires to *not survive* have survived,
relocated. The criterion says so in those words — *"no `format!("…").into_bytes()` payload
construction and no hand-rolled `parse_capacity` survive"* (`project.md:170-173`) — and the
storymap says *deleted rather than wrapped* (`_storymap.md:60`). The reason is not tidiness:
the example is the artefact P4 evaluates the library by in one bounded sitting, and an
example that hand-encodes inside `DomainEvent::encode` teaches the reader that the codec is
optional decoration. It also silently invalidates AC-013's measurement, which is taken over
*this file* — hand-rolled encoding is exactly the mapping boilerplate the ratio is counting
(`_design.md:1104-1111`), so the mutant biases the derive verdict by moving boilerplate into
a place the counter may not look.

The discriminator is textual and belongs in the spec as an explicit acceptance step: after
the rewrite, `parse_capacity` and the string `into_bytes()` do not appear anywhere under
`examples/course-subscriptions/`, and `Enrolment::encode` delegates to `codec.encode(self)`.
That is checkable by reading the diff, which is the only instrument that works here — no test
can distinguish two implementations that produce identical bytes.

**A second mutant: an execution test that executes nothing useful.**

```rust
#[test]
fn the_example_runs() {
    let status = Command::new(env!("CARGO_BIN_EXE_course-subscriptions")).status().unwrap();
    assert!(status.success());
}
```

This is a real improvement over today — it does run `main`, which nothing currently does —
and it will pass forever, including after someone replaces `main`'s body with `println!("ok")`.
It cannot see a missing step, a swallowed refusal, or a `rejected:` line that stopped being
printed, and those are precisely the observable steps AC-003 says survive verbatim. The test
that earns its place captures stdout and asserts every `== … ==` marker appears in order and
that the refusal steps carry the literal `rejected: ` prefix (`_design.md:793-800`) — which
also makes it the only guard in the repository against the transcript quietly losing its
structure.

**A third mutant: keeping a hand-written `Query` beside the decision model.**

```rust
let seats = Seats::new(course)?;
let query = Query::from_items([ /* the same three event types, by hand */ ])?;
let (events, last) = read_decision_model(store, &query).await?;
```

It compiles, it runs, and the numbers come out right — because the hand-written query
happens to agree with the model's derived one *today*. It reintroduces the exact defect the
whole project exists to eliminate, in the one file a reader copies from, and it does so
invisibly: the two statements agree at the moment they are written and diverge the first time
a variant is added. If the rewrite still contains a `Query::from_items` call outside the
library, the rewrite has not happened.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule. Its one
new test asserts on the binary's exit status and on stdout text; the transcript's final-log
rows do print positions, so the assertion is written against the *structure* of those rows —
the markers, the prefix, the ordering — and never against a literal position value, which
also keeps the test from breaking the first time the store's allocation changes. Ticked,
checked rather than assumed. **Frozen clauses:** the example moves from consuming
`happenstance-core` directly to consuming it through `happenstance`; no clause text, marker
or rule is touched, and nothing under `crates/happenstance-core/src/**` is edited. Any
awkwardness the rewrite exposes in the frozen contract is logged with its clause ID and
routed under AC-012 by `defect-log-and-macros-verdict`, which is the box's rule rather than
an exception to it.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
