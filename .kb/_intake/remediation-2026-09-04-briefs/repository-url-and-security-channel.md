# May `0.2.0` ship with a `repository` URL that 404s anonymously, taking `SECURITY.md`'s only reporting channel with it?

Decision record: **PUB-01-repository-visibility**. Brief only — no ADR prose, no atom, no
code. Found during the pre-publication remediation; **not in the audit**, which examined
manifest *contents* and not whether the URLs in them resolve.

---

## Why this is owed

Three crates are already on crates.io and two more join at `0.2.0`. Every one of them
carries `repository = "https://github.com/Wet-Ink-Corporation/happenstance"` through the
workspace inheritance at `Cargo.toml:19`. That URL does not resolve for an anonymous
reader.

The consequence is not cosmetic, and it is not confined to a broken link on a crate page.
`SECURITY.md`'s **only** vulnerability-reporting channel is a GitHub private-advisory
link on the same host:

> **Use GitHub's private vulnerability reporting:** open the
> [Security tab](https://github.com/Wet-Ink-Corporation/happenstance/security/advisories)
> and choose *Report a vulnerability*. That opens a private thread with the
> maintainers, and it is the only channel that gets you a fix before the problem is
> public.

A reporter who finds a soundness bug in the conformance suite — precisely the class the
document says is *"what this library is for"* — follows that link, gets a 404, and has
nowhere else to go. The document's own next paragraph tells them **not** to open a
public issue, so the instruction it gives when its channel is unreachable is *silence*.

## What is true today

**Verified 2026-09-04, anonymously, and not taken from the audit.**

- `https://github.com/Wet-Ink-Corporation` — **200**. The organisation exists, is
  located in the USA, has no public members, and shows exactly **one** public
  repository: `praecepta`, a Python repository last updated 2026-03-05.
- `https://github.com/Wet-Ink-Corporation/happenstance` — **404**. Since the org
  resolves and one sibling repository is public, the 404 is a *visibility* answer and
  not a *typo* answer: the repository is private, or under a different name.
- `Cargo.toml:19` — `repository = "https://github.com/Wet-Ink-Corporation/happenstance"`,
  inherited by all nine crate manifests via `repository.workspace = true`.
- Registry state, read from the crates.io API the same day:
  `happenstance-core` carries `0.2.0-alpha.1` and `0.0.0`; `happenstance` and
  `happenstance-testkit` likewise at `0.2.0-alpha.1`; `happenstance-sqlite` and
  `happenstance-cloudflare` are `0.0.0` placeholders only. **So three crates are
  publishing this URL to the public today**, not at some future release.
- `SECURITY.md` — the advisory link is the sole channel. There is no email address, no
  alternate contact, and no fallback sentence anywhere in the file.

**The repository is the project's evidentiary base, and the crates say so.** This is not
a project where the source link is a courtesy. `README.md`, `CHANGELOG.md` and the crate
rustdoc route the reader to `spec/SPECIFICATION.md`, to `.kb/decisions/`, to
`references/adr/` and to `standards/rust/` for the reasoning behind essentially every
non-obvious choice, and `SECURITY.md` itself says *"Everything else in this project is
deliberately in the open — the specification, the backlog, the evidence behind every
decision — and this is the one exception."* Anonymously, none of it is open. A crate
whose documentation is built on citations into a repository nobody can read is a
different product from the one that documentation describes.

## Options

### Option A — Publish the repository before `0.2.0`

The URL resolves, the advisory link resolves, the citations resolve, and **no file in
the tree changes**. It also makes true the sentence `SECURITY.md` already asserts.

Costs: whatever the privacy is currently buying. Publishing takes the whole tree with
it — `.bklg/`, the committed telemetry under `.redkiln/`, and the fourteen evaluation
documents under `references/evaluation/`, which include this remediation's own audit and
its record of what was found wrong. That is a business judgement this brief cannot take,
and it is why the question is routed to the owner rather than answered here.

### Option B — Keep it private, and give `SECURITY.md` an out-of-band channel

A one-paragraph edit: an email address (or a `security.txt`-style contact) presented as
the channel that works today, with the advisory link retained as the preferred route
once the repository is public. **Closes the reporting hole and nothing else** — the
`repository` link on three crate pages still 404s, and every `file:line` citation in the
published rustdoc still leads nowhere. Requires an address the owner is willing to
publish and monitor; nothing in the tree supplies one.

### Option C — Point `repository` somewhere that resolves

Change `Cargo.toml:19` to a public mirror, or drop the key. Dropping it is worse than
wrong: it removes the reader's only signal that a source exists at all. A mirror splits
the project's identity and creates a second thing to keep in sync, which this
repository's own citation-drift experience says will fail quietly.

### Option D — Ship `0.2.0` as-is and fix it afterwards

Available, and it is what happens by default. The asymmetry is that a published crate
version is immutable: `0.2.0`'s `repository` link is wrong on crates.io forever, and the
window in which a reporter with a real finding cannot reach anyone opens the moment the
release lands and closes only when someone notices.

## Recommendation

**A if the repository can be made public, and B in the same change if it cannot** — and
B regardless, because it is cheap and it is the only option that survives GitHub's
advisory UI being unavailable for any other reason.

The argument for treating this as a release blocker rather than a follow-up is the
immutability above. Every other finding in this remediation is a code or documentation
defect that a `0.2.1` fixes completely. This one publishes a permanent artefact: the
`repository` field of `happenstance 0.2.0` is a fact about that version for as long as
the registry exists, and it will be wrong.

### The strongest argument against, in its own words

*Nobody is reading these crates yet. `happenstance-core 0.2.0-alpha.1` is a pre-release
Cargo will not resolve without an explicit pre-release requirement, so the population of
affected readers is approximately zero, and a fix at `0.2.1` reaches all of them.*

That is correct about today and wrong about the release. `0.2.0` is not a pre-release,
and it is the release whose entire purpose is to create the reader population. The cost
is not being paid now because the audience does not exist yet; it starts being paid at
the exact moment the audience does — which is the same arithmetic this remediation
accepted for `L1-1`'s rule and `D-1`'s re-exports, and it should not be accepted there
and refused here.

## Cost of delay

Zero until `0.2.0`, then permanent for that version and recurring for every version
published before it is fixed. Option B's cost never changes and is one paragraph.

## What this does not settle

- **Whether the private-advisory link should remain the *preferred* channel.** It is the
  right mechanism when it works; the question is only what stands beside it.
- **Whether the published rustdoc's `file:line` citations should be relative-path or
  URL-shaped.** They are load-bearing here, and `spec-trace` and `lint-constitution` keep
  them honest *inside* the repository. Whether a reader on docs.rs can act on them is a
  separate question with its own answer, and it becomes live under Option B.
- **Whether the `0.0.0` placeholder reservations should be yanked before a real
  release.** Raised in `adapter-driver-reexport-policy.md` and still open; the same
  registry read that produced this brief's version table is the evidence for it.
