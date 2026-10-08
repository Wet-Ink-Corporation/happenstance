# ADR-0082 — ProjectionId is validated: VT-14's set, a 255-byte bound, and happenstance/ and sync/ reserved

- **Status:** accepted.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window), lane L10.
- **Decided by:** the owner's default for phase 17 ("`ProjectionId` refuses the full ADR-0015 set
  with a generic reserved prefix", `runbook/handover.md`), and two Weigh-In calls: `wi-2155ac`
  (the reserved set is `["happenstance/", "sync/"]`, matched as an exact byte prefix) and
  `wi-279dbb` (`sync_watermark(StoreId)`, infallible, with no hidden unchecked constructor). This
  record states those answers and takes D4, D5 and D6 itself.
- **Answers:** [`kb-open-question-projection-id-unvalidated-001`](../../.kb/open-questions/projection-id-is-unvalidated.md).
- **Supersedes:** [ADR-0015](../../.kb/decisions/0015-validated-identifiers-and-store-limits.md)
  **§10 only**, where it declined to validate `ProjectionId`. ADR-0015 stays accepted.
- **Clauses:** VT-35 `[FROZEN]` and PS-39 `[PROVISIONAL]`, minted here; SY-31 and PS-25 amended
  by cross-reference.
- **Plan:** `.temper/plans/p17-l10.md`; research brief `.temper/plans/p17-l10-brief.md`.

---

## 1. What was true before

`crates/happenstance-core/src/projection.rs` at `6235224` defined

```rust,ignore
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionId(Box<str>);

impl ProjectionId {
    pub fn new(value: impl Into<String>) -> Self { … }
}
```

`new` was infallible and checked nothing: `ProjectionId::new("")` succeeded and the empty string
became a checkpoint row's primary key. Its two siblings, `EventType` and `Tag`, both validate
through `validate::check` and return a `Result`. There was no `const` constructor, and no
`TryFrom`, `FromStr`, `AsRef` or `Borrow<str>`. The type had shipped unchanged since `0.2.0`, and
`v0.3.2` carries the same `new`.

Two store-side failures were already reachable:

- Postgres `text` refuses U+0000 ("invalid byte sequence for encoding UTF8: 0x00"), so
  `ProjectionId::new("a\0b")` failed at `commit` on Postgres and on Neon, far from the constructor
  that accepted it.
- A Postgres btree entry is limited to about 2,704 bytes, so a very long id failed at insert.

Those are the forcing event the open question named: *a projection store whose backing table
rejects a key the constructor accepts*.

## 2. Why ADR-0015 §10 declined, and why both reasons are gone

ADR-0015 §10 gave two reasons.

1. **No instrument.** `ProjectionStore` was provisional and had no conformance suite, so a
   validating constructor would have been checked by nothing: the decorative-rule hazard
   `CLAUDE.md` names. Since then the projection suite has grown to seventeen rules with a mutant
   registry (`crates/happenstance-testkit/tests/projection_mutation_coverage/`), and ADR-0063
   froze the port. A store-side rule over the id's bytes can now be written and failed, and PS-39
   is that rule.
2. **A second door would be worse.** A fallible `parse` beside an infallible `new` reproduces
   defect D2, the VT-16 / VT-26 hole: two constructors with different rules, and an invalid value
   reachable through the weaker one. **That reason still binds.** It is why this record replaces
   the only door rather than adding a second, and why no infallible conversion survives.

## 3. The decision

### D1 — one checking door, two spellings of it

```rust,ignore
pub fn new(value: impl Into<String>) -> Result<ProjectionId, InvalidProjectionId>;
pub const fn from_static(value: &'static str) -> ProjectionId; // panics on refusal
pub fn sync_watermark(peer: StoreId) -> ProjectionId;           // D3
```

`new` and `from_static` call one private `const fn refusal(&str) -> Option<InvalidProjectionId>`.
It runs `validate::check(value, MAX_PROJECTION_ID_LEN)` and then a private `reserved_prefix`, a
`while`-loop byte compare over `RESERVED` (RS-11-3: `starts_with` is not `const`).
`TryFrom<&str>`, `TryFrom<String>` and `FromStr` forward to `new`. **`From<&str>` and
`From<String>` are deliberately absent**, and a `compile_fail` doctest on `ProjectionId` pins it.

The field moves from `Box<str>` to `Cow<'static, str>` so that `from_static` can hold a borrowed
literal in a `const`. That costs 8 bytes (24 against 16), and nothing stores ids in bulk. It is the
trade VT-32 records for `EventType`. `Eq`, `Ord` and `Hash` are written by hand over `as_str()`,
and `Borrow<str>` and `AsRef<str>` are added, for VT-33's reason: a map keyed by `ProjectionId`
must be probeable by `&str` without losing entries.

### D2 — what is refused, in what order

1. empty → `Empty`;
2. more than 255 bytes → `TooLong { len }`, carrying the rejected length, not the bound;
3. a `Cc` character → `ControlCharacter`, or one of U+202A–U+202E, U+2066–U+2069 →
   `BidirectionalControl`, **whichever occurs first reading left to right**;
4. a value beginning with `happenstance/` or `sync/` → `Reserved { prefix }`.

Nothing else is refused. `Cf` in general is accepted, so U+200B, U+200C and U+200D are legal.

The left-to-right clause of item 3 was found, not designed. The lane's property test first used an
oracle that checked every `Cc` before any bidirectional control, and proptest shrank a
counterexample to `"\u{202E}\0"`: the validator answers `BidirectionalControl`, because
`validate::check` is one walk over the value and reports the first offending character. That is
`EventType`'s and `Tag`'s behaviour too, so the contract is stated as it is; `validate.rs` is not
changed. The seed is kept in `crates/happenstance-core/tests/projection_id.proptest-regressions`.

The reserved prefixes are matched as **exact bytes**. `Sync/x`, `SYNC/x`, `" sync/x"`, `sync`,
`syncope`, `sync_jobs` and `x/sync/y` are all valid ids. Case folding would be a second, unstated
equality beside VT-15's byte equality, and a trim would make the stored id differ from the one
checked.

### D3 — the watermark constructor

`sync_watermark(peer)` is `Self(Cow::Owned(format!("sync/{peer}")))` over `StoreId`'s `Display`:
32 lowercase hex digits, no dashes. Its output is 37 bytes, passes every VT-14 rule, and is refused
by `new` only as `Reserved { prefix: "sync/" }`, which a unit test asserts: because the reservation
is checked last, a `Reserved` verdict proves the value passed everything before it.

The format is frozen with VT-35. A watermark is persisted as a checkpoint key, so changing the
rendering orphans every peer's progress. A `Debug` rendering (`StoreId([..])`), a UUID-dashed one
and an uppercase one are each named by a test as wrong.

### D4 — the error type

```rust,ignore
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidProjectionId {
    Empty,
    TooLong { len: usize },
    ControlCharacter,
    BidirectionalControl,
    Reserved { prefix: &'static str },
}
impl From<core::convert::Infallible> for InvalidProjectionId { … }
```

It lives in `projection.rs`, beside `CommitError` and `ResetError`, the port's own errors, rather
than in `error.rs`. That keeps `error.rs` and `lib.rs` line-neutral for the constitution's exact
citations. It does not hand the rejected `String` back (ERR-14), for parity with
`InvalidEventType` and `InvalidTag` and to stay `Clone + Eq` and allocation-free.

### D5 — phase 18's derived id

See the atom for the rule as decided. The proof that the separator rule keeps a derived id out of
the reserved namespace, written out:

> Let `d = name ++ SEP ++ h`, where `name` is a value `ProjectionId::new` accepts, `SEP` is one
> byte, and `h` is 16 lowercase hex digits. Suppose `d` begins with a reserved prefix `p`.
>
> - If `len(p) ≤ len(name)`, then `p` is a prefix of `name`, so `name` begins with `p` and `new`
>   refuses it. Contradiction.
> - Otherwise `p` covers all of `name` and at least `SEP`, so `SEP` is a byte of `p`. The rule
>   chooses `SEP` outside every byte of every `RESERVED` entry. Contradiction.
>
> So `d` begins with no reserved prefix. It contains no `Cc` and no bidirectional control, since
> `name` has none and `SEP` and `h` are printable ASCII; it is non-empty; and its length is
> `len(name) + 17`, which is at most 255 exactly when `len(name) ≤ 238`.

The premise is *accepted by `new`*, not *valid*, and the difference is one constructor. A
`sync_watermark` id is a valid `ProjectionId` — VT-35 mints it — and it begins with `sync/`, so a
derived id over it would begin with `sync/` too, and the first case above would not be a
contradiction. Phase 18 therefore MUST build the derived id by passing `name ++ SEP ++ h` to
`ProjectionId::new` and surfacing its refusal as the derivation's typed error, never through a
constructor that skips the reservation. A name the premise excludes
is then refused at derivation rather than minting an id inside the reserved namespace, and the
proof's premise is checked by the same validator it relies on.

`/` fails the rule (it is in both prefixes). So does every letter of `s`, `y`, `n`, `c`, `h`, `a`,
`p`, `e`, `t`. `@` is recommended; `#` lost because `tickets-over-http` exposes ids in URLs, where
`#` begins a fragment, and `:` lost because it is the tag `key:value` convention.

### D6 — maturity

VT-35 is `[FROZEN]`. PS-39 is `[PROVISIONAL]`, freeze-by-17b, falsified by a backing store whose
key column cannot hold a 255-byte UTF-8 key byte-faithfully and cannot be configured to. At this
record it is green against `MemoryProjectionStore`, `happenstance-sqlite`, and both of
`happenstance-postgres`'s stores against a live PostgreSQL 17.10. Neon runs it in CI's live job,
which is not a required check until L8.

## 4. The store half: PS-39 and its mutants

`projection_ids_round_trip_by_bytes` commits seven pairs of ids, each distinct by bytes and equal
under one plausible lossy key mapping, at strictly increasing positions, reads every checkpoint
back through a fresh handle, then resets the second id of each pair and asserts the first did not
move. Each batch carries a probe row, so no commit is a position the batch applied nothing from
(PS-21's question). The rule does **not** assert that the reset id reads `NeverRun`: that is PS-16
and PS-19's, and asserting it here convicted `TwoStatementResetStore` under the wrong name during
the lane.

| Pair | Ids (after the rule's prefix) | Equal under |
| --- | --- | --- |
| case | `Van_Stock` / `van_stock` | `COLLATE NOCASE`, `citext` |
| the bound | 255 bytes ending `👩` / `👨` | `VARCHAR(64)` truncation |
| Persian | `می‌خواهم` with U+200C / U+200D | a `latin1` column, lossy |
| emoji ZWJ | `👩‍💻` / `👨‍💻` | a `latin1` column, lossy |
| slug | `slug.van-stock` / `slug.van_stock` | an identifier-safe slug |
| pad | `pad.van_stock` / `pad.van_stock ` (a trailing space) | `CHAR(n)`, `PAD SPACE`, a trim |
| nfc | `nfc.café` with U+00E9 / with `e` + U+0301 | NFC, a nondeterministic ICU collation |

| Mutant | Models | Fails PS-39 because |
| --- | --- | --- |
| `CaseFoldingKeyStore` | `COLLATE NOCASE`, `citext`, a nondeterministic ICU collation | the case pair shares a row |
| `TruncatingKeyStore` | `VARCHAR(64)` under a truncating server | the two 255-byte ids share their first 64 bytes |
| `AsciiOnlyKeyStore` | `latin1` / ASCII with lossy conversion | the Persian, emoji and bound pairs each become `?`s |
| `SluggedKeyStore` | an id mapped onto a SQL identifier or file name | the slug pair, and every non-ASCII pair |
| `TrailingSpaceKeyStore` | a `CHAR(n)` column, a `PAD SPACE` collation, or a trim before binding | the pad pair shares a row |
| `CanonicalEquivalenceKeyStore` | NFC before binding, or a nondeterministic ICU collation | the nfc pair shares a row; the one composition is written by hand, with no new dependency |
| `SingleRowCheckpointStore` (existing) | a table with no key column | every pair shares the one row |
| `UncommittedTransactionStore` (existing) | a commit that makes nothing durable | the first read sees `NeverRun` |
| `TruncatingResetStore` (existing) | a reset with no `WHERE` | resetting one id clears its neighbour |

The three existing rows gained the rule in `fails`, each with a one-line reason; the conformant
variants pass it. **Not rejectable:** a store keyed on a cryptographic or wide hash of the id. It is
lossy and PS-39 forbids it, but showing that needs two ids that collide, which a fixed id set cannot
supply. `REGISTRY`'s "not covered" list names it.

## 5. Call-site census at `6235224`

Every call site of `ProjectionId::new` was migrated: literal ids in tests and the testkit's rules
to `from_static`; doctests whose `main` returns a boxed error to `new(…)?`; examples to a free
`const` (`tickets-over-http`'s `PROJECTION`, `transfers-on-sqlite`'s `ACCOUNT_BALANCE`,
`rebuilding-read-models`'s five); a computed id (`experiments/polling-cost`'s `view_{index}`) to
`new(format!(…))?`, propagated; and `benchmarks/src/domain.rs`'s `LazyLock` to a `static` built by
`from_static`, since both `id` methods there return a borrow of it.

| Area | Sites |
| --- | --- |
| `happenstance-core` (`src/projection.rs`, `src/projection_memory.rs`, `tests/`) | 28 |
| `happenstance-testkit` (`src/projection.rs`, `tests/`) | 21 |
| adapters' tests (`-sqlite`, `-postgres`, `-neon`) | 10 |
| `happenstance` (`src/runner.rs` doctest, `src/tests.rs`, three test files) | 7 |
| examples | 9 |
| outside the workspace (`benchmarks/`, three `experiments/`) | 15 |

No adapter's production code changed: adapters only call `as_str()`.

## 6. Rejected alternatives

- **A hidden `new_reserved` / `new_unchecked`.** VT-32's `Rejects:` names it, and a hidden
  function is callable.
- **Only `happenstance/`, with the watermark at `happenstance/sync/<peer>`.** Possible, since SY-31
  is provisional; the owner's default names `sync/`.
- **A variant on `validate::Refusal`.** It would change `EventType` and `Tag` for a rule that is
  not theirs.
- **A lower bound than 255.** D5 fits a derived id under 255 with a 238-byte name.
- **Case-insensitive prefixes.** A second equality beside VT-15's.
- **A shared identifier error type.** Three types refuse different sets; one enum would carry a
  `Reserved` arm `EventType` can never produce.
