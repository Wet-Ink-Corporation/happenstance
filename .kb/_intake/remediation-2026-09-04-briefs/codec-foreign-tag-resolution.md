# Does `Codec` stay unsealed, and if it does, what reads a tag it did not write?

**Record id:** `codec-foreign-tag-resolution`
**Would supersede:** nothing. **ADR-0021** (`.kb/decisions/0021-payload-evolution-and-codec-tag.md`) sited the tag and is superseded on its serde attribution by **ADR-0032** (`.kb/decisions/0032-adr-0021-serde-attribution-correction.md`); both are `status: accepted` and therefore immutable. This is a new number. ADR-0032 states the property the framing region exists to deliver — *a build with only `postcard` enabled must still read a tag written by a build with only `json`* — and this brief is about the set that property quantifies over.

**This brief did not get the two-critic pass the original thirteen had.** It was written by the lane implementing audit entry **B-1**, in the same session as the documentation change it describes, and nobody independent argued the other side. `README.md`'s discount applies. Each option below carries its own strongest objection and an answer, which is the form, not a substitute for a critic.

---

## Why this is owed

`Codec`'s page invites the extension in as many words (`crates/happenstance/src/codec.rs:24-26`):

> The trait is **not sealed**: a codec of your own is a legitimate thing to write, which is why this is a trait rather than an enum of the three below. An enum would have been shorter and would have forbidden it.

`commit_with` is titled *"The command loop, with a codec of your own"* (`crates/happenstance/src/command.rs:235`) and takes `&C` for exactly that reason.

Writing under a codec of your own works: `frame::<C>` writes `C::TAG` unconditionally (`codec.rs:290`). Reading one back does not, the moment a second codec is in play. `decode_event` uses the codec in hand when the event carries no tag or carries the codec's own; anything else goes to `decode_by_tag` (`codec.rs:356-369`), which is a closed chain (`codec.rs:371-397`):

```rust
    #[cfg(feature = "json")]
    if tag == <Json as Codec>::TAG { … }
    #[cfg(feature = "postcard")]
    if tag == <Postcard as Codec>::TAG { … }
    #[cfg(feature = "cbor")]
    if tag == <Cbor as Codec>::TAG { … }

    Err(CodecError::UnknownTag { … })
```

There is no registration seam. `decode_by_tag` is private and `decode_event` is `pub(crate)`; the only public door is `Boundary::absorb<C: Codec>` (`crates/happenstance/src/boundary.rs:109`), which takes one codec.

So ADR-0032's property holds across the three built-ins and is **structurally unreachable** for anything the "not sealed" sentence produces. `UnknownTag`'s own page said the refusal *"means exactly one thing: a tag was written and this build cannot honour it"* — true of a built-in behind a feature that is off, where the repair is one flag, and permanently untrue of a third-party tag, where no build can ever become able to honour it.

The failure mode is not exotic. An application adopts its own codec, runs for a year, then adds `Json` — for a migration, for a console-readable log, for anything — and every historical event comes back as `UnknownTag` from `Boundary::absorb`: an empty fold, and an append condition matching nothing. That is the same end state as the audit's `P-2`, reached by a second road.

### What this lane already landed, and what it deliberately did not

Landed (`835028f`), because it is true whichever option below is taken and needs no decision:

- `Codec`'s page carries `# Reading a tag this build did not write` (`codec.rs:28`), immediately under the invitation it qualifies.
- `CodecError::UnknownTag`'s page (`codec.rs:120`) separates the two conditions it covers.
- `commit_with` points at the section rather than restating it.
- `crates/happenstance/tests/codec_extension_point.rs` asserts the invitation and its limit travel together — three checks, all Red before the change — and characterises the behaviour with a real third-party `impl Codec` written through `commit_with` and read back through `absorb`. That last test **passes on arrival and says so**; it is the one place that has to move under every option below, and it is not decorative: mutating `decode_by_tag`'s final `Err` into a fall-back to the codec in hand fails it.

**Not landed:** any change to what the code does. That is this brief's subject.

---

## The refuted shape, recorded so it is not re-proposed

The audit's own finder proposed generalising `C: Codec` to a codec-*set* bound. Review refuted it: that is breaking on three public signatures (`commit_with`, `Boundary::absorb`, the projection runner's decode path) and needs a blanket impl that overlaps any set type unless coherence can prove otherwise. The narrower observation that survived is the one the options below are built on — **writing picks one codec and reading tries several, so the seam belongs on `Codec`**, not in the three signatures.

---

## Option A — a defaulted resolution method on `Codec`

Add, with a default body that refuses:

```rust
    /// Offered a tag this codec did not write, before the built-in chain.
    fn decode_foreign<T: serde::de::DeserializeOwned>(
        &self,
        _tag: &str,
        _data: &[u8],
    ) -> Option<Result<T, CodecError>> {
        None
    }
```

`decode_event` consults the codec in hand for a foreign tag before falling through to the built-in chain.

- **Costs a caller:** nothing unless they want it. A caller with one codec never sees the method.
- **Costs a third-party codec author:** they must make their codec resolve *its own historical tags*, which means a codec that is really a small dispatcher. That is the honest shape of what they are doing, and it is what the option asks them to notice.
- **Semver:** **additive** on a published trait — a defaulted method breaks no implementor. This is the only option that stays free after `0.2.0`.
- **What it buys:** the invitation becomes true. A `Runic`-then-`Json` application reads its history by passing a codec that knows about `Runic`.
- **Strongest objection:** it does not deliver ADR-0032's property, it delegates it. The reader must be holding a codec that knows the old tag; if they hold plain `Json` they still get `UnknownTag`, which is exactly the failure this brief opened with. **Answer:** that is a real narrowing and should be written into the method's page. The difference it makes is that a *repair exists*: today the application has no legal move at all, and under A it has one it can write itself. Also, a generic method on a trait that a `dyn` caller might want is not object-safe — nothing in this crate takes `&dyn Codec` today, and A would foreclose that.

## Option B — a registration seam separate from `Codec`

A `CodecSet`/registry value threaded through the read path, with a blanket impl making a single `Codec` a set of one.

- **Costs a caller:** a second concept, and a decision at every read site about which registry they are in.
- **Semver:** **breaking** on `Boundary::absorb`'s signature unless the registry arrives as a defaulted second door, in which case the crate carries two read paths.
- **What it buys:** it is the only option that actually delivers ADR-0032's property for third-party tags: a build holding `Json` *and* a registry naming `Runic` reads both, with neither codec knowing about the other.
- **Strongest objection:** it is the refuted shape wearing a different coat. The coherence problem is the same — a blanket `impl CodecSet for C: Codec` overlaps any user set type — and the review already found this costs three public signatures. **Answer:** the overlap is avoidable by *not* writing the blanket impl and asking callers to write `set![Json, Runic]`, which is more ceremony and no coherence problem; the honest summary is that B is a larger design than this crate has evidence for, and its cost is real ceremony for a case nobody has yet reported.

## Option C — seal `Codec` and withdraw the invitation

`Codec` becomes sealed, the three built-ins are the three codecs, and `codec.rs:24-26` is deleted.

- **Costs a caller:** the ability to write one. Nobody in this workspace does, and nobody outside it can yet — `0.2.0-alpha.1` is on crates.io behind an explicit pre-release requirement.
- **Semver:** **breaking**, and therefore free only before `0.2.0`. This is the option with a deadline.
- **What it buys:** every promise on the page becomes true, immediately, with no new API. ADR-0032's property becomes exactly true rather than true-of-a-subset. `decode_by_tag`'s closed match stops being a defect and becomes the design.
- **Strongest objection:** it forecloses payload encodings this crate will never ship — protobuf, Avro, a house format with a schema registry — and those are ordinary things for an event-sourced application to want. Sealing to make a documentation sentence true is fixing the wrong end. **Answer:** it is not only the sentence. Sealing is also what buys free trait evolution (see `tuple-boundary-event-type.md` on the same mechanism for `Boundary`), and `Codec` is a trait this crate is likely to want to grow. Against *that*: a sealed `Codec` and an unsealed one differ in who can add an encoding, and "we may want to add a method" is a weaker interest than "an application may want its own wire format".

---

## Recommendation

**A**, and hold C open until `0.2.0`.

A is additive, so taking it now costs nothing that cannot be revisited, and it converts *no legal repair exists* into *a repair exists and here is its shape* — which is the difference that matters to the one application this actually hurts. It leaves ADR-0032's property honestly narrowed rather than pretended, and the narrowing is documentable on the method itself.

C is the only option with a deadline and the only one that makes every current sentence true, so it must be decided before `0.2.0` regardless of whether A lands: A and C are not exclusive in the wrong order — A then C would be adding a method to a trait and then sealing it, which is coherent but wasteful.

**The strongest argument against this recommendation:** nobody has reported the failure, no crate in the wild implements `Codec`, and A adds a generic method to a trait for a user who may not exist — which is speculative generality. This repository's standing discipline is hostile to it in a form that applies directly: *a rule that no adapter can fail is decorative* (`CLAUDE.md`), and a defaulted method no implementor overrides is the same shape one trait over. `RS-40-1` (`standards/rust/40-public-surface-and-evolution.md:12`) does **not** forbid it — it governs *required* methods and a defaulted one is precisely its escape — so the objection here is judgement, not a rule. If the answer to *"has anyone asked for a fourth codec?"* is no, C is cheaper, truer and smaller, and the documentation this lane landed becomes the deletion notice rather than a caveat. **This brief does not have the evidence to settle that question**, and says so rather than manufacturing it: the evidence is a user, and there are none yet.

## Cost of delay

**Bounded by `0.2.0`, and only for C.** A stays free indefinitely. C stops being free the moment a stable version publishes. Delay past `0.2.0` therefore does not lose the repair — it loses the *cheapest* repair and leaves the crate carrying an extension point it half supports.

## What this does not settle

- Whether `Boundary::absorb` is the right public door for reading at all. It is the only one, and every option above works through it.
- Whether `CodecError::UnknownTag` should split into two variants. It is `#[non_exhaustive]`, so that is additive; this lane documented the distinction rather than encoding it, because encoding it presumes the answer above.
- What a projection runner does with a foreign tag. The same `decode_event` serves it, so the same answer applies, but the runner is behind `unstable-projection` and exempt from semver, so it does not constrain the choice.
