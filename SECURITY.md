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

The nine crates this repository publishes, at their published versions:

- `happenstance`
- `happenstance-core`
- `happenstance-testkit`
- `happenstance-sqlite`
- `happenstance-cloudflare`
- `happenstance-postgres`
- `happenstance-neon`
- `happenstance-sync`
- `happenstance-sync-testkit`

**A report against a `0.0.0` placeholder is a report against nothing**: each name
was reserved with an empty `0.0.0` before its first real release.

One note on `happenstance-cloudflare`, because it is `wasm32`-only and nothing in
the host test matrix exercises it. Its conformance runs under a `node:sqlite`-backed
shim in the gate and under `workerd` itself in a sibling CI job, `harness/workerd`,
which mounts every event-store rule and the ingest, write-group and sync peer
families in a real Durable Object (ADR-0066 §6). Findings there are especially
welcome for exactly that reason.

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

**The latest `1.x` minor is supported.** A fix lands in the next `1.x` release; an older
`1.x` minor is not patched separately, because a `1.x` minor never breaks what an
earlier one promised and moving to it is a `cargo update`. When a `2.0.0` ships, the
last `1.x` minor gets **security fixes for six months**, and then none (ADR-0066 §8).
An adapter's major follows its re-exported driver's breaking version (ADR-0044), and
this window is what lets you move on your own schedule.

| Version | Supported |
|---|---|
| latest `1.x` minor | ✅ |
| an earlier `1.x` minor | ⬆ move to the latest `1.x`; nothing in it breaks you |
| `0.x` | ❌ superseded by `1.0.0`; the `0.x` sections of [`CHANGELOG.md`](CHANGELOG.md) and the `docs/upgrading-to-0-*.md` guides are the way across |

### What semver covers

Every public item of the nine crates, and every `[FROZEN]` clause of
[`spec/SPECIFICATION.md`](spec/SPECIFICATION.md) in the behaviour it promises.
`conformance` and `ProjectionProbe` are inside the promise (ADR-0063), and so are
driver error payloads an adapter re-exports, under the driver-major limit above.
**The exemptions are these, and only these** (ADR-0066 §5):

- `#[doc(hidden)]` items;
- the text of a `Display` or `Debug` implementation;
- rustdoc prose;
- `happenstance-postgres`'s naive arm, which is not a feature but a `--cfg
  happenstance_naive_arm` a build sets for itself, and never API.

A new conformance rule that detects a violation of a clause already `[FROZEN]` is a
**minor** release of `happenstance-testkit` or `happenstance-sync-testkit`: it finds a
defect and breaks nothing that was conformant, but it can turn an adapter's CI red, so
pin a testkit exactly (CF-29, CF-31).

### Licence

The nine crates above stay **`MIT OR Apache-2.0` for all of 1.x** (ADR-0066 §8). A
crate added later may be licensed otherwise; these nine may not.

## Disclosure

Report privately, and we will agree a disclosure date with you once there is a
fix or a decision not to fix. If a report turns out to describe intended
behaviour, that answer comes with the reasoning and, where it belongs, a
specification clause — this project's habit is to write down why, not merely
what.
