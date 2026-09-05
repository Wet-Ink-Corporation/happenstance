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

---

## Ratified in part, and landed — 2026-09-04

**Option B is taken.** The repository owner supplied `security@wet-ink.net`, and
`SECURITY.md` now leads with it as the channel that works today, with the GitHub
private-advisory link kept as the better mechanism *where it is available*.

Three things were written rather than left implicit, and each is there because a
reporter meets the document in a different state than its author does.

1. **The 404 is stated, not discovered.** The document now says the link does not
   resolve for everyone and that nothing is wrong with their report if it 404s for
   them. Without that sentence a reporter follows a broken link, reads *"please do
   not open a public issue"* immediately after, and correctly concludes there is
   nowhere to go — which was the whole finding.
2. **"Report against the source in this repository" was amended.** A reader who
   cannot reach the repository cannot report against its source. It now says to
   report against the source rather than against the `0.0.0` placeholder, and adds
   that while the repository is private they should describe what they found by
   email and will not be asked for a link they cannot reach.
3. **The acknowledgement paragraph gained "or by email"**, since its original
   *"say so on the same thread"* presumes the thread exists.

**What this does not close, and it is the larger half.** The `repository` field
still 404s on three crates that are on crates.io today, and `homepage` and
`documentation` remain `null`, so a reader who wants the specification, the ADRs
or the evidence behind any decision still has exactly one door and it is still
locked. Option B was always scoped to the reporting channel. **Option A — publish
the repository — remains open and is the only thing that closes the rest**, and
the asymmetry from the recommendation above is unchanged: `repository` on
`happenstance 0.2.0` is immutable on the registry for as long as the registry
exists.

**Still recommended and not taken:** `documentation = "https://docs.rs/happenstance"`
in the workspace manifest. One line, works under either answer to Option A, and
gives the reader a door that opens. Not slipped in with this change, because it is
a manifest edit rather than the security channel the address was given for.

---

## Option A ratified — 2026-09-04

**The repository owner has approved publishing the repository.** Option B landed
earlier and closed the reporting channel; Option A closes the rest, and it is the
only option that does. This section records what that decides and what it costs,
measured rather than estimated.

**What it fixes that B could not.** `repository` resolves, so the link on three
crates already on the registry — and two more at `0.2.0` — points somewhere. More
importantly, the `file:line` citations that this project's rendered documentation
is *built out of* become followable. A crate whose rustdoc cites `spec/`,
`references/adr/` and `standards/` for the reasoning behind essentially every
non-obvious choice, into a repository nobody can read, is a different product from
the one that documentation describes.

**What becomes public, counted at `53abda9`:**

| tree | files | lines |
|---|---:|---:|
| `.bklg/` | 1,318 | 223,694 |
| `experiments/` | 459 | 103,778 |
| `.kb/` | 189 | 31,724 |
| `references/` | 59 | 26,840 |
| — of which `references/evaluation/` | 29 | 15,094 |
| `.redkiln/` (config, process pack, telemetry) | 44 | 6,139 |

About 390,000 lines of process artefact against a library — including this
remediation's own audit and its record of what was wrong.

**Why the middle path was rejected rather than deferred.** Publishing a subset was
costed and is worse than it looks: the publishable surface cites the process trees
**115 times** from `crates/`, `spec/`, `standards/`, `docs/`, `examples/` and the
READMEs — `experiments/` 64, `references/evaluation/` 32, `.bklg/` 16,
`.redkiln/` 3. Excluding a tree does not hide it; it converts working citations
into dangling ones on precisely the pages that exist to show their work. And
`spec-trace` cites `references/adr/` by line range, so that tree could not be
excluded at all. Turning 115 citations into dead references to satisfy a privacy
boundary is the same move as amending a clause to match an implementation.

**Pre-publication sweep, run before the switch rather than after**, because
publishing a private repository exposes its whole history at once:

- No credential patterns in tracked files — keys, tokens, `BEGIN … PRIVATE KEY`,
  cloud access ids, Slack tokens.
- No leaked absolute local paths (`C:\Users\…`, `D:\repos\…`): **zero** tracked
  files.
- The owner's personal email appears in **no tracked file**. It is in git commit
  metadata, which is ordinary for an open repository and is the author's own
  choice of identity.
- `.redkiln/telemetry/` is the one tree whose content is not argument. Its records
  are `{ts, actor, worktree, branch, event}` — session starts and ends. It
  discloses **working patterns**: which days and hours work happened, over months.
  Nothing secret; it is simply the only thing here that is data about a person
  rather than about the software. Filenames carry the actor name.

**The one thing left to decide, and it is small.** Whether `.redkiln/telemetry/`
ships. Everything else in the exposure table is evidence a reader may want to
check. Telemetry is the exception, and it is the exception on a different axis —
not sensitive, but not argument either. Removing it from the published tree costs
three citations from the publishable surface, which is the smallest severance in
the table by two orders of magnitude.

**When the repository is public, `SECURITY.md` needs one edit.** Its paragraph
beginning *"That link does not resolve for everyone"* becomes false and should go,
leaving the advisory link as the primary channel with `security@wet-ink.net`
beside it. That edit is deliberately **not** made here: writing that the repository
is public before it is would be the same class of untrue-but-green claim this
remediation spent its length removing.
