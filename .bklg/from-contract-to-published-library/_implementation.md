# Implementation ledger — From Contract to Published Library

The per-project progress ledger across `/redkiln:implement` runs. Body-only
companion file: it carries no item frontmatter, and nothing here is written by
`redkiln`. The CLI remains the single writer of every item's stage and status.

- **Initiative:** HS-I0006 · `from-contract-to-published-library`
- **Branch:** `initiative/from-contract-to-published-library`
- **Worktree:** `.claude/worktrees/from-contract-to-published-library`
- **Terminal (DoD-owning) project:** HS-P0019 `closeout-and-durable-audience`
  (`dodOwner: true` in the item index; the only project wired to `verify.e2e`)

`entry_baseline: green @ ae77ac4a1a94fdedb4f4a248210140c86e20a16a`

Observed 2026-08-13, before any mutation: `cargo xtask ci` — the **full** bar, not
`--fast` — exited 0 with `all checks passed` on a clean tree at that commit. Later
runs skip the full-suite step on the strength of that line; per-project runs are
affected-scoped inside the workflow, so anything red after this point is
attributable to the initiative rather than inherited.

## Projects, in merge order

| # | Project | Id | Depends on | State | Verdict / blocker |
|---|---------|----|------------|-------|-------------------|
| 1 | `projection-store-freeze` | HS-P0010 | — | pending | |
| 2 | `typed-layer-and-alpha-release` | HS-P0011 | 1 | pending | |
| 3 | `sqlite-durable-store` | HS-P0012 | 1, 2 | pending | |
| 4 | `cloudflare-durable-object-store` | HS-P0013 | — | pending | |
| 5 | `postgres-and-neon-stores` | HS-P0014 | 1 | pending | |
| 6 | `ladybug-projection-store` | HS-P0015 | 1 | pending | |
| 7 | `publication-and-positioning` | HS-P0016 | 2, 3, 4, 5, 6 | pending | |
| 8 | `replication-identity-and-ingest` | HS-P0017 | 7 | pending | |
| 9 | `retention-and-incomplete-logs` | HS-P0018 | 8 | pending | |
| 10 | `closeout-and-durable-audience` | HS-P0019 | 9 | pending | |

Positions 4, 5 and 6 are mutually unordered; the serial trunk is
1 → 2 → 3 → 7 → 8 → 9 → 10.

## Run log

_No project runs yet._
