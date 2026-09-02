# Wave `2026-09-02-intake` — retrospective

## What merged vs. created

One merge, twelve creates. The single merge —
`.kb/open-questions/deny-bans-red-on-the-worker-dependency.md` — is the wave's only mutating
operation, and it is a resolution: frontmatter hunks plus one appended dated section, the
existing body left untouched. No decision atom's body was opened for writing at any point in
this wave, and no supersession was performed. Full reasoning for every placement is in
`02-placement-and-adjudication.md`'s nine adjudications; `03-integration-summary.md` is the
roll-up.

Two structural firsts: `.kb/design/` received its first two atoms (`kb-design-symbol-annotates-
the-wordmark-001`, `kb-design-radial-mark-collisions-001`), turning it from a scaffolded layer
into a populated one; and `.kb/decisions/` received its first `SD-`-numbered records, filed
alongside the `ADR-`-numbered ones on the strength of the repository's own stated intent
(`brand-where-the-identity-lives.md`: *"cited from `.kb/` atoms exactly as ADRs are"*) rather than
a decision this wave took itself.

## Unresolved claims

Two claims were surfaced and deliberately declined, per Adjudication 9 — not fixed here, not
lost either:

- **The three store-limit numbers** (HS-P0013 vs. HS-S0055 AC-001). Second wave running that this
  surfaces without adjudication. No document in this tree has resolved it, including this one;
  it stays owned by HS-S0055's own record.
- **The brand tree's absence.** `references/brand/`, `assets/brand/` and
  `references/seeds/licensing-and-the-commercial-seam.md` are cited by `source_paths` but are not
  present in this worktree, so the wave verified every brand claim against the four staged
  `.kb/_intake/` files and against nothing else. Landing that tree is a source-control step, not
  a KB op, and it is not this wave's to perform.

## Follow-ups

- **`kb-open-question-adapter-default-projection-feature-001`** (Op 1) is forced before either
  `happenstance-neon` or `happenstance-postgres` publishes — its three ordered sub-questions are
  the postgres-and-neon-stores project's to answer.
- **`kb-open-question-trademark-search-001`** (Op 6) gates registration, filing and physical
  application of the brand identity; the identity itself continues to ship unaffected.
- **The unfalsified boundary in `kb-design-radial-mark-collisions-001`** (Op 9) names its own
  settling check — show the mark at 16px to developers who have seen neither it nor the record —
  and stays a decision-body boundary rather than a companion open question, per Adjudication 8.

## Doctor problems set aside as out of scope

`redkiln doctor --json` reported two problems. Both are `kind: unconsumed-foundation`, both are
under `.bklg/from-contract-to-published-library/`, and neither `file` falls under this wave's
`.kb` path set — this wave's writes are entirely under `.kb/decisions/`, `.kb/open-questions/`,
`.kb/playbooks/`, `.kb/reference/`, `.kb/design/`, `.kb/maps/`, `.kb/concepts/` and
`.kb/governance/`, and it never opened `.bklg/` at all. Set aside rather than repaired, because
repairing a `.bklg/` foundation-story consumption gap here would smuggle an unrelated backlog
change into a KB-intake commit:

- `unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/story.md` (foundation story `HS-S0108`, consumed by no capability slice)
- `unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/story.md` (foundation story `HS-S0100`, consumed by no capability slice)

`redkiln validate --kb` exited 0 with no problems of any kind. The combined, scoped verdict for
this wave is **pass**.
