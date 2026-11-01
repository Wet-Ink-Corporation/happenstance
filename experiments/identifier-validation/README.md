# `experiments/identifier-validation`

Does any legitimate Arabic, Hebrew or Persian identifier — alone or mixed with
left-to-right text — get refused by the validator `happenstance-core` ships? And
does that validator still agree, codepoint for codepoint, with the `char` walk
ADR-0015 checked it against?

This is VT-14's falsifier, run. It is issue #86, the item in
[`runbook/phases/17b-after-the-window.md`](../../runbook/phases/17b-after-the-window.md),
and the condition `runbook/ledgers.md:293` at `97f0f36` puts on VT-14's freeze: *"An RTL
identifier corpus check (Arabic, Hebrew, Persian, with mixed LTR) comes back
empty, with the E11 reproduction added to `experiments/`."*

This is not a crate anybody depends on. It is **not a workspace member** (its
`Cargo.toml` carries an empty `[workspace]` table, the same trick every
neighbour under `experiments/` uses), it appears in no `cargo xtask ci` step, and
its only dependency is a path dependency on `happenstance-core`. The root
`Cargo.lock` is untouched.

## Why it exists

VT-14 (`spec/SPECIFICATION.md`, §2.4) says `EventType::new` and `Tag::new` reject
Unicode `Cc` and the explicit bidirectional formatting controls
(U+202A–U+202E, U+2066–U+2069), and MUST NOT reject `Cf` generally. When this
directory was written, at `4d18e771`, the clause called those controls "seven" and
its marker was `[PROVISIONAL — falsified by a legitimate event type or tag that
requires an explicit bidirectional formatting control]`. It is now `[FROZEN]` by
ADR-0089, on this run, which also corrected the count to nine.

ADR-0066 refused to freeze it (`references/adr/0066-what-1-0-promises.md:216-218`)
for a precise reason: **E11 tested how the check is built, not whether any
legitimate value needs a bidi control**, and E11 was not in `experiments/`. E11
is ADR-0015's experiment
(`references/adr/0015-validated-identifiers-and-store-limits.md:186-202`): a
standalone crate, since lost, that walked all 1,112,064 Unicode scalar values and
compared a *copy* of the byte walk against `char::is_control` plus a `matches!`
over the seven-codepoint list. What it proved was the shape, not the shipped code.

So this experiment does two things, and they answer different questions.

1. **E11, reproduced against the shipped validator.** The same oracle, but the
   verdict is read through `EventType::new` and `Tag::new` — the public
   constructors, so the walk under test is the one `validate.rs` compiles into
   the contract crate, not a copy of it. This answers *"is the check what VT-14
   says it is?"*
2. **The corpus.** Hand-written Arabic, Hebrew and Persian identifiers, each in
   the form its orthography actually takes, many mixed with Latin text and ASCII
   digits. This answers the question the marker asks: *"does a legitimate value
   get refused?"*

## Pre-registered criterion

This section was committed **before any code in this directory was written**,
as its own commit (`337d6a29`), so that what "empty" means could not be adjusted
to the outcome. One erratum followed (`043ccb70`, below): it was made **after the
harness was written but before it was run**, and it corrects a count, not a
criterion. The result is recorded below, in later commits. Since the run, this
section's substance is unchanged; this paragraph was reworded at review to say
exactly when each of those two commits was made.

### What is measured

**E11-R, the reproduction.** For every Unicode scalar value `c` (all 1,112,064;
surrogates are not `char`s), in three positions:

| position | value | why |
| --- | --- | --- |
| alone | `c` | a lead byte at index 0 |
| between ASCII | `a`, `c`, `b` | a lead byte mid-string — E11's second position |
| between Arabic | U+0628, `c`, U+0628 | **new here**: the lead byte follows a two-byte RTL sequence, so the walk enters `c` from inside multi-byte text rather than after an ASCII byte |

the verdict of `EventType::new` and the verdict of `Tag::new` are each compared
against the oracle: *refused as a control character* iff `char::is_control(c)`;
*refused as a bidirectional control* iff `c` is one of U+202A–U+202E or
U+2066–U+2069; *accepted* otherwise. An accepted value must also read back
byte-identical through `as_str`.

**The corpus.** A list of hand-written identifiers in three scripts:

- **Arabic** — letters, harakat (combining marks, `Mn`), tatweel U+0640,
  Arabic-Indic digits U+0660–U+0669, Arabic punctuation, presentation forms,
  and the Arabic Letter Mark U+061C.
- **Hebrew** — letters, final forms, niqqud (`Mn`), maqaf U+05BE, geresh U+05F3
  and gershayim U+05F4, and Hebrew presentation forms.
- **Persian** — Persian letters (پ چ ژ گ ک ی), Extended Arabic-Indic digits
  U+06F0–U+06F9, and the zero-width non-joiner U+200C that Persian orthography
  requires inside words.

In each script, entries mixed with left-to-right text: Latin words, ASCII digits,
dotted and slashed namespaces, `key:value` tags with an LTR key and an RTL value
and the reverse, and entries carrying the implicit directional marks U+200E
(LRM) and U+200F (RLM) where a writer would reach for them. Each entry is tested
as an event type **and** as a tag, and each `key:value` pair also through
`Tag::key_value`.

### What "empty" means

The run comes back **empty** if and only if all of these hold:

1. **E11-R: zero disagreements** in each of the three positions, for each of the
   two constructors — `0` of `1,112,064`, six times.
2. **The refused set is the clause's set.** Alone, exactly ~~72~~ **74** scalar
   values are refused: the 65 `Cc` codepoints (U+0000–U+001F, U+007F,
   U+0080–U+009F) with the control-character variant, and the ~~seven~~ **nine**
   codepoints of U+202A–U+202E and U+2066–U+2069 with the bidirectional variant.
   None lies in an Arabic, Hebrew, Syriac, Thaana, NKo,
   Samaritan or Mandaic block, or in either RTL presentation-forms range.
3. **The corpus: zero refusals.** Every corpus entry is accepted by both
   constructors and reads back byte-identical, and every `key:value` pair is
   accepted by `Tag::key_value`. The list of refused corpus entries has length
   zero.

### What makes the run void rather than empty

An empty result from a harness that cannot see a refusal is worth nothing. The
run is **void** — neither empty nor not — if any of these fails, and it is then
rerun after the harness is fixed, never reinterpreted:

- **Negative controls.** Each of the ~~seven~~ **nine** bidirectional controls,
  and U+0085
  (a C1 control), is inserted into every corpus entry at its start, its middle
  (on a character boundary) and its end. Every such value MUST be refused, with
  the bidirectional variant and the control-character variant respectively. One
  accepted negative control voids the run.
- **Corpus shape.** Each of the three scripts has at least ten entries; every
  entry contains at least one character from its script's block; and each script
  has at least three entries that also contain an ASCII letter or digit (the
  mixed-LTR requirement).
- **Coverage.** E11-R checks exactly 1,112,064 values per position.

### Erratum, made before the run

The first commit of this section said *"exactly 72"* and *"the seven
bidirectional controls"*, taking the count from VT-14's own text. The clause's
ranges, U+202A–U+202E and U+2066–U+2069, hold **nine** codepoints, not seven
(five embeddings and overrides, four isolates), and `validate.rs` rejects all
nine — its own test was named `rejects_all_seven_bidirectional_controls` until
ADR-0089, and walks nine. The count was found wrong while writing the harness, before anything
here was run, and corrected in its own commit with the old numbers struck
through rather than deleted. The ranges were always the criterion; only the
arithmetic moved. The miscount in the clause and in the prose that repeats it is
a finding for the owner, not something this directory edits.

### What a non-empty result means

Any corpus entry refused, or any disagreement, is recorded here with its
codepoints and the variant returned, and is **not** removed from the corpus. A
refused corpus entry is a candidate falsification of VT-14 and goes to the owner
as a re-disposition of the clause; a disagreement in E11-R is a defect in
`validate.rs` against VT-14's text and goes to an issue. Neither is fixed in this
directory: validator changes are out of scope.

### What "empty" does not mean

An empty result is **necessary for VT-14's freeze, not sufficient**. The corpus
is written by one author and is a sample of what applications will name things,
not a population; an empty result says no value in it needs a bidi control, and
that the validator refuses nothing else that RTL text uses. The marker's
falsifier — a legitimate value that *requires* an explicit bidirectional control
— stays reachable by any value an application produces later. Marking VT-14
`[FROZEN]` is a change to a specification clause and is the owner's decision; this
directory does not make it.

## Result

**Empty.** Every criterion held, and no condition voided the run.

| criterion | held | the number |
| --- | --- | --- |
| 1. E11-R, zero disagreements | yes | `0` of `1,112,064`, in each of 3 positions × 2 constructors |
| 2. refused set is the clause's ranges | yes | 74 refused alone, both constructors: U+0000–U+001F and U+007F–U+009F as `ControlCharacter`, U+202A–U+202E and U+2066–U+2069 as `BidirectionalControl`; 0 inside any of the 19 blocks in `src/blocks.rs`, counted over every refusal rather than over the recorded list |
| 3. corpus, zero refusals | yes | 51 entries (Arabic 21, Hebrew 16, Persian 14) × 2 constructors, and 10 pairs through `Tag::key_value`: 0 refused, 0 altered |
| void: negative controls | none escaped | 1,530 per constructor (51 entries × 10 inserts × 3 positions) |
| void: corpus shape | no problem | ≥ 10 entries per script; every entry carries its script; mixed-LTR entries: Arabic 6, Hebrew 6, Persian 5 |
| void: coverage | full | 1,112,064 values per position |

### Conditions

| | |
| --- | --- |
| Tree | `7bc79b4e` — the harness after the first review's fixes. The first run, at `163ba0aa`, also came back empty, and its output is in this file's history |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, LLVM 22.1.6, `cargo 1.97.1` |
| Host | `x86_64-unknown-linux-gnu` |
| Build | `--release`, `--locked` |
| Command | `./run.sh` |
| Wall clock | a few seconds, almost all of it compilation |

`results/raw/conditions.txt` is `rustc --version --verbose` read back by the run
itself; `results/raw/tests.txt` is the test binary's output, one thread, and
`results/raw/run.txt` is the binary's report, whose last line is
`== Result: EMPTY`. The binary exits non-zero on anything else, so `run.sh` would
have stopped there.

### How the harness was shown not to be blind

Two of the tests hand the same harness a validator VT-14's `Rejects:` line
names, and both are caught:

- `sweep_catches_a_validator_that_bans_only_ascii_controls` — a C0-and-DEL-only
  check disagrees with the oracle on exactly 41 values between ASCII letters: the
  32 C1 controls and the nine bidi controls.
- `corpus_catches_a_validator_that_bans_the_joiners` — a check that refuses
  U+200C and U+200D (the two `Cf` characters the clause says a blanket `Cf` ban
  would take) refuses exactly the seven corpus entries that carry one, all
  Persian, each as `OtherRefusal`, and nothing else.

And `negative_controls_catch_a_validator_that_refuses_nothing` confirms all 1,530
controls escape a validator that accepts everything, so the zero escapes above are
the shipped validator's, not the harness's. Likewise
`rtl_block_count_covers_refusals_past_the_recorded_cap` shows that a validator
refusing everything is counted inside the RTL blocks for every one of their
codepoints, far beyond the 256-entry recorded list, so criterion 2's zero does not
depend on that cap.

### What the run also showed

- **VT-14 said "seven" and meant nine.** At `4d18e771`, the clause, ADR-0015,
  `validate.rs`'s doc comments and the `BidirectionalControl` variant's docs all
  said *the seven explicit bidirectional formatting controls* (ADR-0089 corrected
  all but ADR-0015, whose body is immutable); the ranges they give hold nine,
  and the shipped check refuses nine. Nothing was wrong with the behaviour; the
  count in the prose was wrong (see the erratum above). Correcting a clause's
  text was not this directory's to do.
- **RTL text uses `Cf`, and the narrow list lets it.** The corpus carries
  U+200C ZWNJ (six Persian entries and two pairs), U+200D ZWJ, U+200E LRM, U+200F RLM,
  U+061C ALM, U+0600 ARABIC NUMBER SIGN and U+06DD ARABIC END OF AYAH, all `Cf`,
  all accepted. The implicit marks LRM, RLM and ALM are the plain-text tools a
  writer reaches for to fix a mixed-direction rendering, and they are not on the
  closed list; nothing in the corpus needed an *explicit* embedding, override or
  isolate to be written. Isolates are what a display layer wraps around an
  identifier when it prints one, which is outside the identifier's bytes.
- **The byte bound bites RTL text at half the characters.** Arabic, Hebrew and
  Persian letters are two bytes in UTF-8, so the 255-byte bound admits 127 of
  them (`constructors_report_empty_and_too_long_as_their_own_verdicts` refuses
  128 U+0628 as `TooLong`). That is VT-20's subject, not VT-14's, and no corpus
  entry came near it.

### What this licenses

VT-14's freeze condition in `runbook/ledgers.md:293` at `97f0f36` is met: the RTL corpus check
came back empty and the E11 reproduction is in `experiments/`. Per *What "empty"
does not mean* above, that is necessary and not sufficient, and marking the clause
`[FROZEN]` is the owner's decision. This directory does not edit
`spec/SPECIFICATION.md`.

## Running it

```console
./run.sh                     # conditions, the tests, then the report, into results/raw/
cargo test --release         # the criteria alone
cargo run --release          # the report alone; exits non-zero unless EMPTY
```
