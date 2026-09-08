# Decision log — the `0.2.0` closeout session

Every decision this session took that a reader could reasonably have taken
differently, with the options that were on the table, the evidence that separated
them, and what was chosen. Written as it happened rather than reconstructed.

**This file is a session record, not a knowledge-base atom.** Decisions with
durable consequence are additionally staged as briefs in `.kb/_intake/` for a
later `/redkiln:kb-ingest`, and the ones that bind code get a long-form record in
`references/adr/`. Atoms are never hand-written.

**Scope.** RUNBOOK phases 10b, 11 and 12, to the point of *ready to publish*.
The release itself — `cargo publish` of the five crates, the `v0.2.0` tag, the
GitHub release, the repository visibility flip and the yank of `0.2.0-alpha.1` —
is deliberately not done here and is listed at the foot.

---

## Standing decisions taken by the owner before the work started

| # | Decision | Consequence |
|---|---|---|
| O-1 | The session stops at **publish-ready**. | Two of phase 12's four exit criteria cannot be met and are reported unmet rather than ticked: *crates live / docs.rs green*, and *`cargo-semver-checks` against a published baseline*. |
| O-2 | Phase 11 is **attempted**, installing CMake if `lbug`'s prebuilt path fails. | Phase 11 gates nothing; if the native build cannot be made to work it is left `blocked` with the evidence rather than skipped in silence. |
| O-3 | Neon runs against the **real endpoint**, from `NEON_CONNECTION` in the git-ignored `.env`. | The credential is read from the environment at run time only. It is never committed, never written into a fixture, never echoed into a log or a CI artefact. |
| O-4 | `happenstance-ladybug` is **claimed on crates.io** during the session. | A `0.0.0` placeholder, if a registry token is present; otherwise the placeholder is generated and the publish left to the owner. |

---

## Decisions taken during the work

*(appended as they are taken)*

---

## Left for the owner

*(accumulated as the work reveals them)*
