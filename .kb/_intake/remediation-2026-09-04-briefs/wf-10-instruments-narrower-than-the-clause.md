# WF-10 says *every value type*. Two mutations prove the instruments do not. Which instrument does each direction get?

Short answer up front: **both directions are confirmed by construction in this
working tree, and one of them is half-guarded in a way the review did not
credit — `E0063` already forces a contributor who adds a field to `Event` to
stand at the wire mirror, and then lets them fill it with `None` and walk away.**
Neither fix is inside this lane's writable surface; both are `crates/` and one is
a `Rule:` line in `spec/SPECIFICATION.md`.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

WF-10 `[FROZEN]`, unqualified:

> The `Deserialize` implementation of every value type MUST re-run its
> constructor's validity invariants, so that no wire value can become an
> in-memory value the constructor would have rejected.

Its `Rule:` line names four tests, and `grep -n "fn decode_"
crates/happenstance-core/tests/wire.rs` returns exactly those four:

```
1179:    fn decode_rejects_a_non_canonical_tag_set()
1260:    fn decode_rejects_an_unconstrained_query_item()
1324:    fn decode_rejects_a_zero_item_query()
1399:    fn decode_accepts_an_over_capacity_value()
```

They cover `Tags`, `QueryItem` and `Query`. `EventType` and `Tag` — the two value
types VT-14 `[FROZEN]` gives constructor invariants to in the first place — have
none.

---

## Direction one: measured, and the whole workspace stays green

The production code is correct today. `crates/happenstance-core/src/event.rs`:

```rust
    impl<'de> Deserialize<'de> for EventType {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let raw = String::deserialize(deserializer)?;
            Self::new(raw).map_err(serde::de::Error::custom)
        }
    }
```

Replacing that second line with `Ok(EventType(Cow::Owned(raw)))` — the
"simplification" a contributor reaches for on the reasoning that a wire value was
validated upstream — was applied in a scratch worktree and measured:

```
### clippy          EXIT=0
### test core       EXIT=0
### test testkit    EXIT=0
### cargo test --workspace --all-features   EXIT=0
```

Nothing in the workspace observes it. The proptest generator cannot, and says so
in its own shape (`crates/happenstance-core/tests/wire.rs:176-178`):

```rust
        pub(super) fn any_event_type() -> impl Strategy<Value = EventType> {
                .prop_map(|value| EventType::new(value).expect("a valid event type"))
```

Every value it round-trips has already passed `new()`. `Tag` is the same shape at
`:164`.

The wrong outcome is a peer or a hand-crafted replication message putting a value
carrying a control character or a bidirectional override into a receiver's memory
— which is what VT-14's constructor exists to prevent and what WF-10's `Rejects:`
line names.

---

## Direction two: half-guarded, and the half that holds is not the half that matters

The review reads `Event`'s field set as tied to `EventWire`'s by nothing. That is
right about `Serialize` and **wrong about `Deserialize`**, which builds `Event`
with a struct literal and no `..`:

```rust
    impl<'de> Deserialize<'de> for Event {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = EventWire::deserialize(deserializer)?;
            let mut event = Self {
                event_type: wire.event_type,
                data: wire.data,
                tags: wire.tags,
                metadata: None,
            };
```

Measured: adding a required `causation: Option<Bytes>` field to `Event` produces

```
error[E0063]: missing field `causation` in initializer of `Event`
   --> crates\happenstance-core\src\event.rs:363:12
error[E0063]: missing field `causation` in initializer of `Event`
   --> crates\happenstance-core\src\event.rs:769:29
```

— and `:769` **is** the wire mirror. So the compiler already does the first half
of what the review's proposed instrument would do: it makes the contributor stand
there.

It does not do the second half. Filling both sites with `causation: None` — the
obvious way past an `E0063` — leaves the field absent from the wire in **both**
directions, and:

```
### cargo check -p happenstance-core --all-features   EXIT=0
### cargo test  -p happenstance-core --all-features   EXIT=0
```

The round-trip proptests pass vacuously, because their generator does not vary a
field it does not know about. `Serialize for Event` is not touched at all, since
its literal is exhaustiveness-checked against `EventWire`.

That correction matters for choosing the instrument: the ceremony the review
prices is partly already paid, and what is missing is narrower than "force the
contributor to the mirror". It is "make the mirror's field set an assertion".

---

## The question, and the options

**Q1 — direction one.** Two `wire::decode_rejects_*` tests, feeding
`EventType::deserialize` and `Tag::deserialize` a string carrying a C1 control
and a bidirectional override, plus their citation on WF-10's `Rule:` line.

That is a repair under `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`,
not a clause amendment: the set of implementations WF-10 admits does not change.
Whether the same obligation on VT-19 and the other `[FROZEN]` clauses imposing it
gets cited in the same pass is part of it.

Note the coupling to S-5: `spec-trace`'s check 4 is what would hold the new names
honest, and **it now does** — the prose guard that would have silenced WF-10's
`Rule:` line (it contains the words "compile test") is retired, and every name it
cites is either resolved or declared in `UNRESOLVABLE_RULE_NAMES`.

**Q2 — direction two.** Three shapes, in increasing ceremony:

- **A. Destructure the source exhaustively in `Serialize`.** `let Event {
  event_type, data, tags, metadata } = self;` with no `..` — `E0027` then fires on
  a new field at the *serialize* mirror too, matching what `E0063` already does at
  the deserialize one. One line, no new test, no new dependency.
- **B. A `#[cfg(test)]` assertion that the two field sets are equal.** Rust cannot
  express it directly; it would be a source-reading check, and this lane's
  `lint-enumerations` shows the shape works. Cost: a second place that knows about
  the mirror pairs.
- **C. Both, at `Event` only**, leaving `SequencedEventWire`, `EventIdWire`,
  `GuardWire` and `QueryItemWire` on the argument that they are smaller and
  frozen.

**Recommendation: Q1 as stated, and A for Q2.** A is one line per mirror and it
closes the direction that is genuinely open, given that `E0063` already closes
the other. The strongest argument against A: it makes the `Serialize` impl
mention every field twice and reads as noise to anyone who has not been shown
this failure — which is the argument that ends with someone re-adding `..` during
a tidy-up. That argument is real, which is why A should land with a comment
naming the wrong implementation it forbids, the way the rest of this crate's
`serde` module already does.

---

## Cost of delay

Not `0.2.0`, and both fixes are additive. What delay costs is that the finder is
a peer, at ingest, over durable events — which is the most expensive place in
this design to find anything.

---

## What this does not settle

Whether the exhaustive-destructuring instrument is worth its ceremony at every
wire mirror or only at `Event`; that is a design call belonging with whoever owns
§2.7's wire format. Whether the proptest generators should gain a
`prop_oneof!` arm that produces *invalid* strings for the decode direction, which
would be a stronger instrument than two hand-written tests and a larger change.
