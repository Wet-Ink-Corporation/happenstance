# ADR-0083 — Codec stays unsealed through 1.x, and CodecError::UnknownTag is not split

- **Status:** proposed.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window), the item "the breaking open questions phase 16 listed"
  (`runbook/phases/17-breaking-window.md:61-66`).
- **Decided by:** the owner, not yet. This record recommends an answer. Its status moves to
  `accepted` on the owner's call, and only then does it close the open question.
- **Answers:** [`kb-open-question-seal-the-codec-001`](../../.kb/open-questions/should-codec-be-sealed.md),
  all three ordered sub-questions (`:86-99`). Also the item ADR-0066 §10 placed in this window
  (`.kb/decisions/0066-what-1-0-promises.md:347-351`).
- **Acts on:** [ADR-0049](../../.kb/decisions/0049-a-codec-declares-the-tags-it-reads.md)'s
  *What stays open* (`:80-91`) and *What this does not decide* (`:93-103`).
- **Supersedes:** nothing. ADR-0049 stays accepted. This record takes the decision it deferred,
  and contradicts none of its text except one passing claim. That claim is that the `UnknownTag`
  split "stays additive" (`:97-100`), and §5 corrects it here rather than in ADR-0049's body.
- **Evidence:** the tree at `4fbfefa` (citations re-checked at `6235224`), and one read-only crates.io query on 2026-10-07 (§4).
  Nothing was compiled for this record. The external implementors listed in §4 are the existing
  compile-pass evidence.

---

## 1. What is published

| Item | Where | Gated? |
| --- | --- | --- |
| `pub trait Codec` | `crates/happenstance/src/codec.rs:80` | no (`lib.rs:228`) |
| `const TAG: &'static str` | `codec.rs:92` | — |
| `fn encode<T: Serialize>` | `codec.rs:100` | — |
| `fn decode<T: DeserializeOwned>` | `codec.rs:109` | — |
| `fn reads_tag(&self, &str) -> bool`, defaulted | `codec.rs:185-187` | — |
| `#[non_exhaustive] pub enum CodecError` | `codec.rs:197-249` | no |
| `Json`, `Postcard`, `Cbor` | `codec.rs:273-276`, `:299-302`, `:325-328` | `json` (default), `postcard`, `cbor` |
| `commit_with<S, B, C: Codec, …>` | `command.rs:411`, re-exported at `lib.rs:232` | no |
| `Boundary::absorb<C: Codec>` | `boundary.rs:109` | no |

The trait's page says it is **not sealed** (`codec.rs:24-26`). The section headed `# Reading a
tag this build did not write` (`codec.rs:28-67`) states what that invitation costs.
`tests/codec_extension_point.rs:109-139` pins both, and pins their order.

## 2. What "sealed" means in Rust, and why the two directions are not symmetric

This section is for a reader who knows interface versioning from C# or Java but not Rust's rules.

**Sealing.** Rust has no `sealed` keyword for traits, so a library builds one from two rules.
`happenstance` already does this for `Boundary` (`crates/happenstance/src/sealed.rs:1-19`,
`boundary.rs:69`):

```rust
mod sealed {                // a private module: unnameable outside the crate
    pub trait Sealed {}     // `pub` so it may appear in a public trait's bounds
}
pub trait Codec: sealed::Sealed { /* … */ }
impl sealed::Sealed for Json {}
```

A downstream `impl Codec for Mine` now needs `Mine: Sealed`. Writing that impl means naming
`happenstance::sealed::Sealed`, and the module is private, so the compiler refuses with `E0603`
("module `sealed` is private"). Omitting it gives `E0277` ("the trait bound `Mine: Sealed` is not
satisfied"). `sealed.rs:3-6` calls the `pub`-in-a-private-module spelling the only one that works.
Strictly, a `pub(crate)` supertrait on a `pub` trait compiles on 1.97 and also seals; what it
costs is the warn-by-default `private_bounds` lint ("trait `Sealed` is more private than the item
`Codec`"), which this workspace's `-D warnings` turns into a failed gate. So the private-module
spelling is the only one that passes the gate, not the only one the compiler accepts.

**Why sealing later is a break and unsealing is not.** Adding the supertrait turns every existing
downstream `impl Codec` into `E0277` on the next `cargo update`. The code did not change, but the
trait it implements now asks for something it cannot provide. Removing a supertrait asks for
less. Every existing impl still satisfies the trait, and no caller's bound `C: Codec` can have
relied on `Sealed`, because `Sealed` has no items and cannot be named. So:

| Change | Downstream effect | Semver |
| --- | --- | --- |
| unsealed → sealed | every foreign `impl Codec` fails, `E0277` | **major** |
| sealed → unsealed | none | minor |
| add a required method to an unsealed trait | every foreign impl fails, `E0046` | **major** (RS-40-1, `standards/rust/40-public-surface-and-evolution.md:12-17`) |
| add a required method to a sealed trait | none: every impl is in this crate | minor (`.kb/concepts/growing-a-sealed-trait-is-not-a-breaking-change.md:8-14`) |
| add a defaulted method to either | none, barring a method-name ambiguity at a call site | minor |

`Codec`'s own page states the first row backwards. It says sealing "stays open, because it is
additive to take later and impossible to undo" (`codec.rs:64-67`). The truth is that sealing is
the change that cannot be taken later without a major, and unsealing is the additive one. The
open question restates the correct direction (`should-codec-be-sealed.md:103-108`). The doc
comment was never corrected, and §7 corrects it.

**What this record therefore costs.** Staying unsealed is the one-way choice. It gives up sealing
until `2.0`, and in `1.x` `Codec` can grow only defaulted items: methods with bodies, and
associated consts with defaults. A new associated *type* is ruled out too, because associated type
defaults are not stable Rust. A new required item would be `E0046` in every foreign impl. This
is the same constraint `EventStore` already lives under, and the reason `reads_tag` was added with
a default (`CHANGELOG.md:1419-1424`).

**Coherence, which is a separate fence.** The orphan rule says an `impl` must be in the crate
that owns the trait or the type. It is why nobody outside `happenstance` can override `reads_tag`
*for `Json`* (`codec.rs:52-59`, ADR-0049 `:59-71`). That holds whether or not `Codec` is sealed.
Sealing would forbid foreign codecs. The orphan rule only forbids changing this crate's codecs.
They are two different walls, and this record leaves both standing.

## 3. Options

- **A. Stays unsealed through 1.x, recorded.** No code change. The open question's
  sub-question 3 answer (`should-codec-be-sealed.md:96-99`), and the likely answer phase 16 named
  (`:106-108`). **Taken.**
- **C. Seal in `0.4.0`.** ADR-0049's deferred Option C (`0049…md:80-91`). Makes ADR-0032's
  "any build reads any tag a built-in wrote" property exact. Breaks every foreign implementor. §6
  records what it would take, so that a future `2.0` does not rediscover it.
- **Leave it open.** Not available. ADR-0066 §10 requires an answer in this window, because after
  1.0 the C half of the question costs a major.

## 4. Evidence

1. **The API is built around third-party codecs.**
   - `commit_with` is ungated (`lib.rs:232`), and the command loop takes `C: Codec` rather than a
     closed enum. The trait's page says why: "An enum would have been shorter and would have
     forbidden it" (`codec.rs:24-26`).
   - `commit_with`'s page is the door for a codec of your own, and a test holds its pointer to the
     limit section (`tests/codec_extension_point.rs:161-177`).
   - `reads_tag` serves only a foreign codec. The three built-ins never override it, and its
     CHANGELOG entry says so (`CHANGELOG.md:1419-1432`).
2. **Sealing leaves a valid build that compiles but cannot be used.** `default = ["std", "memory",
   "json"]`, and each codec is its own optional feature (`crates/happenstance/Cargo.toml:105`,
   `:120-127`). `default-features = false` with none of `json`, `postcard` or `cbor` turned on is
   a supported build: `decode_by_tag` carries a `cfg` arm specifically so it compiles warning-free
   (`codec.rs:482-486`). Sealed, that build has zero `Codec` implementors. `commit_with`,
   `Boundary::absorb` and `DomainEvent::decode<C: Codec>` would all compile and be uncallable.
   `cargo hack --feature-powerset` checks that each combination compiles, not that it can be
   called, so the gate would not notice.
3. **Implementors that sealing would break.** Each of these is outside the crate as the compiler
   sees it, because an integration test and a doctest each compile as a separate crate:
   - the `MyApp` doctest, `codec.rs:130-159`;
   - `Runic`, `tests/codec_extension_point.rs:203`;
   - `Elder`, `:226`;
   - `Forgetful`, `:429`;
   - a local `Json`, `tests/composition.rs:93`.

   Two in-crate implementors would survive: `src/tests.rs:101` and `src/command.rs:634`. These
   five are already the guard this record wants. A change that seals the trait fails to compile
   them, so no new test is owed.
4. **The one invariant the crate needs from a foreign codec is already enforced.** A tag must be
   non-empty. `TagIsWritable::<C>::CHECKED` is a `const` assertion, evaluated once per codec type
   when `frame::<C>` is instantiated (`codec.rs:372-384`, `:392-393`). It fails the downstream
   build at compile time and needs no seal. The other half, "no `0xFF`", is free because a
   `&'static str` is UTF-8 (`codec.rs:89-91`).
5. **No visible user either way.** On 2026-10-07 crates.io reported 0 reverse dependencies for
   `happenstance`, 133 downloads in total, and `max_stable_version` `0.3.2`. Private dependents are
   invisible to that count. So "nobody would be broken by sealing" is unproven, and "nobody is
   inconvenienced by staying open" is certain. The open question asked which reading of the
   silence to take (`should-codec-be-sealed.md:65-75`). This record takes the reading whose error
   costs nothing.
6. **`Boundary` is sealed and `Codec` is not, and that is consistent.** `Boundary` is sealed to
   protect an invariant: the query is derived and cannot be overridden (ADR-0020,
   `.kb/decisions/0020-fold-query-agreement.md:75-78`). `Codec` has no such invariant to protect
   beyond item 4's, which is enforced without a seal. A seal is a tool for an invariant. It is not
   a default.

## 5. Sub-question 1: the `UnknownTag` split — declined

`CodecError::UnknownTag` (`codec.rs:221-225`) is produced by three conditions. The open question
and ADR-0049 named two.

| # | Condition | Producer | Repair |
| --- | --- | --- | --- |
| 1 | A framing region is present but unreadable: a framing version this build does not know (the magic `hpst` followed by a version other than `\x01`), no terminator, or a tag that is not UTF-8 | `unreadable()`, `codec.rs:438-445`, from `:422-433` (version: `:422-424`, `:360-363`) | an unknown version: upgrade `happenstance` to the release that wrote it; otherwise none, the bytes are damaged or not ours |
| 2 | A tag naming a built-in codec whose feature is off in this build | `decode_by_tag`, `codec.rs:501-503` | turn the feature on in `Cargo.toml` |
| 3 | A tag no codec in this build claims | the same line | pass a codec whose `reads_tag` claims it |

**Why the split is not "additive whichever way sealing goes".** The atom (`:110-111`) and
ADR-0049 (`:97-100`) both say it is, because `CodecError` is `#[non_exhaustive]`.
`#[non_exhaustive]` makes *adding a variant* compile-compatible: every downstream `match` already
has a wildcard arm. It does not make moving an existing input into the new variant
behaviour-compatible. A caller who wrote `Err(CodecError::UnknownTag { tag }) => …` to catch case 2
would, after the split, fall into its wildcard arm instead. It compiles, and it does something
else. Phase 17 applies the same reasoning to ADR-0022 §9 ("which variant a stranded read reports",
`runbook/phases/17-breaking-window.md:112-115`) and to `AppendError::Busy` (ADR-0077's summary, `.kb/decisions/0077-appenderror-busy.md:31-32`:
"moving a refusal from Store to Busy after 1.0 silently breaks every caller matching on the old
variant"). So the split is a `0.4.0` change or it does not happen in `1.x`.

**Why decline it rather than take it.**

- **The strongest in-tree argument for the split, and why it does not carry.** The test that pins
  the variant's page says "A caller writing a recovery path needs to know which one they have"
  (`tests/codec_extension_point.rs:141-146`). What the test then asserts is a sentence in the
  rustdoc (`:152-158`), so the tree already chose to answer that need in documentation rather than
  in a variant. This record keeps that choice; it does not invent it.
- **The distinction has no runtime consumer.** Case 2's repair is a `Cargo.toml` edit and a
  rebuild. A program that matched `CodecDisabled` at run time could do nothing with it except
  report it, and `UnknownTag`'s message already carries the tag. Case 3's repair is also a code
  change.
- **Taking it costs more than one variant.** `<Json as Codec>::TAG` cannot be named in a build
  where `json` is off, so the tags would have to be hoisted into ungated consts. The test would
  need a `cfg(all(feature = "postcard", not(feature = "json")))` gate, and it would need a control
  asserting that a foreign tag still yields `UnknownTag`.
- **A two-way split would still leave case 1 mixed in.** Doing it properly means three variants
  for one error that no caller branches on.

**What is done instead (two-way).** The variant's rustdoc says "two conditions" (`codec.rs:207-211`)
and is rewritten to name all three in the same five lines (§7). Case 1 is not one condition with
one repair: an unknown framing *version* (`codec.rs:360-363`, `:422-424`) is repaired by
upgrading, and only damaged bytes are a dead end, so the rewrite does not call it unrepairable.
The test that pins that page, `the_refusal_distinguishes_a_feature_from_a_dead_end` (`tests/codec_extension_point.rs:147-159`),
asserts only `no build`, and the rewrite keeps it.

## 6. If Option C is ever taken (a `2.0` note, not a plan)

- **It must not reuse the existing seal.** `crate::sealed::Sealed` carries
  `impl<M: DecisionModel> Sealed for M {}` (`sealed.rs:19`), a *blanket* impl. Under
  `pub trait Codec: crate::sealed::Sealed`, any downstream type that implements `DecisionModel`
  already satisfies the supertrait and can write `impl Codec for MyModel`. The seal would seal
  nothing. Option C needs its own marker, `pub trait SealedCodec {}` in the private module,
  implemented only for `Json`, `Postcard` and `Cbor` under their `cfg`s.
- **Its proof would be a `compile_fail` doctest** that implements `Codec` for a downstream
  `DecisionModel` type. That is the wrong implementation it rejects.
- **It would delete the five external implementors** in §4.3 and the "not sealed" test, whose own
  panic message says that is the test to delete (`tests/codec_extension_point.rs:116`).
- **It would rewrite `commit_with`'s page** and carry a CHANGELOG entry marking the break.

## 7. Consequences

- No signature changes. No `0.4.0` trace-table row is owed, because nothing a semver tool can see
  moves.
- **`codec.rs:61-67` is rewritten.** The backwards sentence is replaced with the correct
  asymmetry and a pointer to this record. ADR-0066 §5 exempts rustdoc prose from semver
  (`0066…md:262-265`), so the edit is two-way. It keeps its seven lines, so no `codec.rs:N`
  citation moves. It is below the heading, so the order `not sealed` (`:24`) then
  `# Reading a tag this build did not write` (`:28`) that `tests/codec_extension_point.rs:117-129`
  asserts is untouched. So are the five strings the section must contain (`:132`).
- **`codec.rs:207-211` is rewritten** to name three conditions, in five lines, keeping `no build`.
- `kb-open-question-seal-the-codec-001` is closed on acceptance. The phase 17 item's line is struck
  through.
- In `1.x`, `Codec` grows only by defaulted items (§2).

## 8. A discrepancy, recorded so nobody re-derives it

ADR-0049's atom shows the method it added as:

```rust
fn reads_tag(&self, tag: &str) -> bool {
    false
}
```

(`.kb/decisions/0049-a-codec-declares-the-tags-it-reads.md:37-41`). It also says "the default
refuses" (`:45`). What shipped, and what `0.3.2` publishes, defaults to `tag == Self::TAG`
(`codec.rs:185-187`). `decode_event` also checks `tag == C::TAG` before consulting the method
(`codec.rs:467-471`), so an override can widen what a codec reads and cannot narrow it. The test
`an_override_that_forgets_its_own_tag_still_reads_what_it_wrote`
(`tests/codec_extension_point.rs:420-429`) holds that ordering. Under either default, a codec that
says nothing reads exactly its own tag, because of the short-circuit. The visible difference is a
direct call: `MyCodec.reads_tag(MyCodec::TAG)` returns `true`. The code is authoritative. The atom
is accepted and immutable, so it is not edited. This paragraph is the correction.

## 9. Accepted costs

- **The tag namespace is unpoliced.** Nothing stops a foreign codec from declaring
  `TAG = "json"`. Its events would be framed `json`. A reader holding the real `Json` would take
  the `tag == C::TAG` branch and decode them as JSON, which ends in `CodecError::Decode` at best
  and a misread at worst. Option C would have made the built-in tags unforgeable. A one-sentence
  warning on `Codec::TAG`'s rustdoc is additive and two-way, so any phase may add it. This record
  does not require it.
- **Nothing proves no foreign codec exists**, and nothing proves one does (§4.5). This record
  chooses the answer whose mistake costs nobody.

## 10. Falsifiers

- A foreign codec, through a tag collision or a `reads_tag` that claims a built-in's tag, produces
  a silent misread in the wild that documentation could not have prevented. That is the evidence
  Option C lacked, and it reopens the question as a `2.0` item.
- A change seals the trait without superseding this record. The five external implementors in
  §4.3 fail to compile, which is the existing guard.
- A caller is found who branches at run time on "feature off" versus "foreign". That reopens §5,
  and after 1.0 only as a `2.0` item or as a new method on `CodecError`, such as an additive
  `fn names_a_builtin(&self) -> bool`, that moves no input between variants.

## 11. Out of scope

- A registry of codecs, which ADR-0049 rejected (`:50-57`).
- A rustdoc warning about tag collisions (§9).
- Whether the projection runner's decode path, behind `unstable-projection`
  (`runner.rs:652`), should report foreign tags differently. It uses the same `decode_event`, so
  this record applies to it unchanged.
- Any change to `happenstance-core`.
