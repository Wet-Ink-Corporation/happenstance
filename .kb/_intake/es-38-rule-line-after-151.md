# Proposal for #163's freeze record: ES-38's `Rule:` line, now that the rule exists

Raised by #151, 2026-11-15. Raw material for the phase 14 freeze record (#163),
not an atom and not an ADR. Nothing here binds anything until the owner adopts
or rejects it.

## Why the frozen text is stale

ES-38 is `[FROZEN]` (`spec/SPECIFICATION.md:4859-4883`), so its body changes only
through an ADR. Its `Rule:` bullet (`spec/SPECIFICATION.md:4869-4879`, eleven
lines) still describes the world before phase 14:

- "**(new)**" — the rule now exists:
  `positions_are_not_reused_after_removal` in
  `crates/happenstance-testkit/src/suite.rs`, enumerated in `registry.rs`.
- "That instrument does not exist" — CF-27's instrument is `WithholdingStore`
  (#144), and `PrunedMemoryFixture` grants `REMOVAL` by forwarding to
  `withhold` (#151).
- "`Fixture` declares no such capability" — `Fixture::REMOVAL` and
  `remove_outside_port` landed in #147.
- "**Owner: phase 14.**" — phase 14 has done it.

## The proposed replacement (11 lines for 11, line-neutral)

```
- **Rule:** `positions_are_not_reused_after_removal` (#151), run against CF-27's
  instrument (`PrunedMemoryFixture` over `WithholdingStore`, where it passes by
  delegation) and against every fixture that grants `Fixture::REMOVAL` (#147):
  SQLite, Postgres, Neon, the Durable Object over the shim and under workerd. It
  removes the tail and then everything, and requires each next append above every
  position ever assigned, `head` to name a held event, and the removed identity to
  stay unheld. It rejects `RowidReuseFixture`, the named wrong implementation (a
  rowid table without `AUTOINCREMENT` reusing the deleted tail's highest rowid),
  `SequenceHeadFixture` and `TruncatingWhenEmptyFixture`. It stands on the guard
  `removal_outside_the_port_removes_exactly_the_named_events`. Amended by the
  phase 14 freeze record (#163): the phase 4 note that no capability existed is gone.
```

Eleven lines replace eleven, so no `SPECIFICATION.md:N` citation below the
clause moves. `cargo xtask spec-trace --write` regenerates §7.1 afterwards.

## Why this needs the ADR and is not a citation repair

`kb-governance-referent-not-reasoning-001` lets a citation's referent be
repointed without a decision. This is not that: the replacement changes what
the clause says checks it, and removes the clause's own statement that nothing
does. That is a change of reasoning, so it belongs to the freeze record that
#163 writes, beside ES-40's marker move.

## Evidence the rule is real (measured at #151)

- The wrong stores in
  `crates/happenstance-testkit/tests/mutation_coverage/removed_history.rs`, and
  the three existing removal mutants the rule also rejects, fail exactly their
  declared rules, asserted by `mutants_fail_exactly_their_declared_rules`.
- `happenstance-sqlite`'s
  `a_rowid_table_without_autoincrement_hands_out_the_deleted_tails_position`
  shows the named wrong implementation on real SQLite.
- Not measured locally: Postgres, Neon and workerd run the rule only in their
  live CI jobs.
