# Security

## Reporting a vulnerability

**Email <security@wet-ink.net>.** That is the channel that works today, and it
reaches the maintainers privately.

**If you would rather use GitHub's private vulnerability reporting**, open the
[Security tab](https://github.com/Wet-Ink-Corporation/happenstance/security/advisories)
and choose *Report a vulnerability*. It is the better mechanism where it is
available: it keeps the report, the fix and the advisory in one thread.

**Please do not open a public issue for a vulnerability**, and please do not send
a proof of concept to a public discussion. Everything else in this project is
deliberately in the open — the specification, the backlog, the evidence behind
every decision — and this is the one exception.

Expect an acknowledgement within a few days. This is a small project and there is
no on-call rota; if you have had no reply within a week, assume the notification
was missed rather than ignored, and say so — on the same thread, or by email.

## What is in scope

The seven crates this repository publishes, at their published versions:

- `happenstance`
- `happenstance-core`
- `happenstance-testkit`
- `happenstance-sqlite`
- `happenstance-cloudflare`
- `happenstance-postgres`
- `happenstance-neon`

The last two joined this list at `0.2.0`. Before that the only thing under either
name on crates.io was a `0.0.0` placeholder with no functionality and no
dependencies — it never linked a driver and never put anything in front of a
consumer. **A report against a `0.0.0` placeholder is a report against nothing**;
if you are looking at one, you are looking at a version that predates the crate.

Two notes on `happenstance-cloudflare`, because it is the member of that list
whose evidence is thinnest and you should know that before you weigh a finding
against it. It passes the conformance suite under a `node:sqlite`-backed shim
rather than under `workerd` itself — `.kb/open-questions/no-workerd-class-runner-in-the-gate.md`
is the standing record of what that does and does not establish — and it is
`wasm32`-only, so nothing in the host test matrix exercises it. Findings there
are especially welcome for exactly that reason.

Things worth reporting, because they are what this library is *for*:

- **A conformance rule that passes an unsound store.** The suite is the thing
  that decides whether an adapter is safe to depend on, so a rule that can be
  satisfied by an implementation which loses a write, admits two conflicting
  appends, or reports a position it did not assign, is a defect in the guarantee
  itself and not merely a weak test.
- **An append condition that admits a write it should reject**, or a read that
  returns events a query did not nominate.
- **A projection commit that separates the read-model write from its
  checkpoint**, since the whole point of that port is that they are one unit of
  work.
- Anything that lets untrusted event payloads affect control flow. Payloads are
  opaque bytes to `happenstance-core` by design; if that has stopped being true
  somewhere, it is a bug worth hearing about.

## What is out of scope

- **Denial of service through deliberate resource exhaustion** — an unbounded
  query against a store you control, a batch sized to exhaust memory. The limits
  a store enforces are documented per adapter and are a configuration question
  rather than a vulnerability.
- **`todo!()` in a crate marked a stub** in [`README.md`](README.md)'s status
  table. Those crates are not published and are documented as unfinished.
- **The testkit's own fault-injection stores.** `FaultyStore`, `GappyMemoryStore`
  and the mutant registry exist to misbehave; that is their job.

## Supported versions

Pre-1.0, and honestly so. There is no long-term support branch and no
backporting: a fix lands in the next release, and the release before it is not
patched. Pin an exact version and read [`CHANGELOG.md`](CHANGELOG.md) at each
upgrade.

`0.2.0` is the first stable release, and what that does and does not promise is
worth being exact about. The `EventStore` clauses marked `[FROZEN]` in
[`spec/SPECIFICATION.md`](spec/SPECIFICATION.md) are semver-binding from here.
`ProjectionStore` was **not** at `0.2.0`: it shipped behind an off-by-default
`unstable-projection` feature with a documented semver exemption. Adapters at
both ends of its batch-shape axis have since passed its conformance suite and
ADR-0063 lifted the gate, so from the next release the `[FROZEN]` `PS` clauses
are semver-binding too. What remains exempt is the typed projection *runner*
behind `happenstance`'s `unstable-projection` feature, for a reason that crate
names; a breaking change there is not a violation of this table.

| Version | Supported |
|---|---|
| `0.2.0` | ✅ current |
| `0.2.0-alpha.*` | ❌ yanked at the `0.2.0` release |
| anything earlier | ❌ |

## Disclosure

Report privately, and we will agree a disclosure date with you once there is a
fix or a decision not to fix. If a report turns out to describe intended
behaviour, that answer comes with the reasoning and, where it belongs, a
specification clause — this project's habit is to write down why, not merely
what.
