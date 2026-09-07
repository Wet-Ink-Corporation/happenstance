# Seed — licensing and the commercial seam

Raw material for `/redkiln:initiative`. This states a problem and a vision. It
deliberately does **not** decompose the work, name projects, or propose a design:
that is `/redkiln:plan`'s to decide, from its own grounding. It also does not
settle the question — the choice of licence is an ADR, and ADRs are the runbook's
to author.

**Not legal advice.** Everything below is research and reasoning. Before Wet Ink
Corporation signs a commercial agreement, files a trademark, or adopts a
non-OSI licence, this needs an IP attorney who works in open source.

## Where this stands

The workspace is `MIT OR Apache-2.0` today. One declaration in
`Cargo.toml:9`, two files at the root, copyright in **Wet Ink Corporation**
(`LICENSE-MIT:3`), and a `cargo package --list` assertion in the gate that both
files and a README are inside every publishable `.crate` — added because
`cargo publish --dry-run` does not warn about a missing licence and a crates.io
release cannot be edited afterwards.

There is **no CLA**. `CONTRIBUTING.md:315-322` states inbound-equals-outbound and
leaves contributors their own copyright.

Nothing is published. `RUNBOOK.md` phase 12 puts `happenstance-core`,
`happenstance`, `happenstance-testkit` and `happenstance-sqlite` on crates.io at
`0.2.0`, and phase 12 has not started.

**That is the deadline, and it is a hard one.** A crates.io release is
irrevocable. The moment `happenstance-core 0.2.0` exists under `MIT OR
Apache-2.0`, that grant is permanent for that version — a later version can carry
any licence at all, and anyone who dislikes it forks from `0.2.0` and carries on.
So the licence is not a phase-12 checklist line. It is a phase-12 *precondition*,
and it is the last cheap moment to choose.

The goal, as the owner states it, is three things at once:

1. **pervasive** — this is a library people `cargo add`, and adoption is the
   whole point of holding the bare name (ADR-0006);
2. **permissive**, without becoming one of the projects that strangled itself
   with licence terms;
3. **guarded** against someone taking the code, closing it, and selling it, with
   an easy commercial door for organisations that cannot use copyleft;

and GitHub Sponsors has been applied for in anticipation of the third.

## What the evidence says

### The threat being guarded against does not have the shape it is assumed to have

The licences invented to stop appropriation — SSPL, BUSL, Elastic 2.0, FSL —
were all written against one specific attack: **a hyperscaler hosting your server
as a managed service and capturing the revenue.** MongoDB, Elastic, Redis,
HashiCorp, Sentry, SurrealDB. Every one of those is a *product* you run, and the
thing being taken is *the operating of it*.

happenstance is a library. There is no service to host, no console to resell, no
managed offering to undercut. What a bad actor can actually take is a snapshot of
a contract crate, which begins rotting the day `0.3.0` ships, against a
specification and a conformance suite they do not control and cannot credibly
claim to have passed.

The realistic uses of a permissive happenstance are: a consultancy vendoring it
into a client's closed product (**that is the adoption, and it costs nothing**),
an enterprise embedding it behind a policy that forbids copyleft (**same**), and
a competitor rebranding a fork (**a trademark problem, not a copyright one** —
see below). Copying the anti-hyperscaler playbook onto a library imports the
entire adoption cost of those licences and buys none of the protection they were
designed to give, because the attack they defend against cannot be run here.

### Copyleft is uniquely severe for a *Rust* library

Rust static-links and monomorphises. There is no dynamic-linking seam, which
means the LGPL compromise that made Qt's dual-licence model work for thirty years
is practically unavailable: LGPL §4 requires that a user be able to relink the
application against a modified library, and nothing in the Rust toolchain makes
that a reasonable thing to promise. The Rust community has circled this
repeatedly and landed on permissive defaults for exactly this reason.

So for happenstance the copyleft options collapse to GPL/AGPL, and on a library
those are total: every application that `cargo add`s an AGPL
`happenstance-core` becomes AGPL. That is not a guard against theft. That is a
guard against use.

And it fails a check the repository already performs on *other people's* crates.
`Cargo.toml:54-60` records `sqlx`'s `tls-rustls` feature being rejected because
`webpki-roots` is CDLA-Permissive-2.0 and `cargo deny check licenses` refused it.
That is happenstance sitting on the *consumer* side of exactly the mechanism an
AGPL happenstance would sit on the producer side of. Google's public AGPL ban is
the canonical instance and a great many corporate policies were copied from it;
the practical effect is that an AGPL crate is rejected by a scanner before a
human ever evaluates it.

### Fair Source does not survive being a dependency

FSL and BUSL trade OSI status for a noncompete. For a self-hosted *product* that
trade is arguable — the buyer reviews the licence once, at adoption. For a
*dependency* the review is charged to every adopter, every time, and the
noncompete language ("the same or substantially similar functionality") is fuzzy
enough that a compliance team cannot clear it quickly. HashiCorp's BUSL produced
exactly this: GitLab's legal team concluded it was exposed, and deprecated the
Terraform CI/CD templates in 18.0 rather than keep reasoning about it.

The instructive detail is that **SurrealDB, which does use BUSL, does not use it
on its libraries.** BUSL covers the database; the libraries and SDKs are
Apache-2.0 or MIT. The company that most plausibly could have put a restrictive
licence on a Rust client crate looked at the same problem and split by role.

### The famous disasters are relicensings, not initial choices

Terraform MPL→BUSL in August 2023 produced the OpenTF manifesto within days, a
fork within two weeks, and Linux Foundation adoption within a month; IBM bought
HashiCorp for $6.4bn and the licence did not come back. Redis→SSPL produced
Valkey. Elastic→SSPL produced OpenSearch, and Elastic partially reversed to AGPL
in 2024. Akka→BUSL produced Pekko.

The lesson is not "restrictive licences fail." It is that **a licence is a
promise, and the value destroyed is in the revocation, not in the terms.**
Which cuts both ways for happenstance: choosing permissive now and tightening
later would be the same betrayal at a smaller scale, and choosing restrictive now
simply means the adoption never arrives to be betrayed. Either way, the decision
wants to be made once, before publish, and then kept.

### The dual-licence businesses that do work share three properties

iText (AGPL + commercial), Ghostscript/Artifex (AGPL + commercial exception), Qt
(LGPL + commercial), MySQL (GPL + commercial) — all libraries, all genuinely
profitable, and all with:

1. **consolidated copyright**, via a CLA that assigns or broadly licenses
   contributions;
2. **a product expensive enough to route around** that paying beats reimplementing;
3. **willingness to enforce.** This is the part that is usually left out. Dual
   licensing converts to revenue only where non-compliance has a consequence, so
   the business is, in practice, a licence-compliance operation with a library
   attached. Artifex litigated to establish that the GPL is an enforceable
   contract. If letters are never going to be sent, AGPL costs adoption and
   returns nothing.

happenstance has (1) available and does not have (2) or (3) — and (3) is a
choice about what the owner wants to spend their life doing, not a fact about the
code.

There is a corollary worth stating plainly, because it eliminates an option that
looks attractive: **MPL-2.0 does not produce a commercial-licence business.**
Its file-level copyleft is mild enough that a company can ship it inside a closed
product, so nobody ever needs to buy their way out of it. MPL is the correct
choice if the goal is *reciprocity on the code* — changes to our files come back
— and the wrong choice if the goal is revenue. Those are two different goals and
they want two different instruments.

## The vision

**Draw the commercial seam somewhere other than the licence of the contract.**

The observation that makes this tractable is that an enterprise blocked by policy
does not want to *buy permission* — under Apache-2.0 it already has permission,
for free, and that is a selling point rather than a gap. What it wants to buy is
**indemnification, a warranty, a support commitment, and a named counterparty to
put on a purchase order.** That is a commercial *agreement*, not a commercial
*licence*. It converts better, it needs no CLA, it is what procurement is
actually shaped to buy, and it is entirely compatible with the crate staying
`MIT OR Apache-2.0` forever.

Three layers, and the first two follow a seam the architecture already draws:

**The interface stays permissive, permanently, and says so.** The contract
(`happenstance-core`), the typed layer (`happenstance`), the conformance suite
(`happenstance-testkit`) and the specification. These are what "pervasive" means,
and `happenstance-testkit` in particular *must* be frictionless — a bar that
adapter authors have to seek permission to clear is not a bar. Apache-2.0 also
carries an express patent grant with a termination clause, which is real
protection of a kind permissive licences are routinely and wrongly said to lack.

**The commercial layer is what an organisation needs and an individual does
not** — operational adapters, migration tooling, certification against the
conformance suite, priority support, an SLA, indemnity. This is open core, drawn
at the port boundary that already exists, and it is subject to the rule the
evidence repeats without exception: **the enterprise tier must add to the free
one, never subtract from it.** A capability that is free today and paid tomorrow
is the Terraform mistake in miniature.

**The anti-appropriation lever is trademark, not copyright.** Apache-2.0 §6
grants no trademark rights, and this is exactly the protection the owner is
reaching for: a fork may take every line of the code and still may not call
itself happenstance, and may not claim conformance. The name is Wet Ink
Corporation's regardless of what the source licence says. Registering it,
publishing a trademark policy, and making a testkit pass the thing that gets
*certified* costs adoption nothing and defends the thing actually worth
defending, which was never the source text.

GitHub Sponsors belongs in this picture, but not as a licensing instrument. It is
a donation channel governed by GitHub's terms with no contract between sponsor
and maintainer; no procurement department will accept a sponsorship tier as a
grant of rights. It is excellent for individuals and goodwill, and a
"commercial supporter" tier bundling listed benefits is fine. The organisational
door is a real order form against a real entity.

## What has to be true for this to be worth doing

- **Phase 12 does not start until the licence question is closed.** Not because
  the answer is likely to be "change it" — the recommendation above is that
  almost nothing changes — but because publishing is the moment the answer stops
  being reversible, and the runbook currently treats the licence as an assertion
  about file presence rather than as a decision.
- **The decision is written down as an ADR**, including the option not taken and
  why, because "why isn't this AGPL?" will be asked by every serious evaluator
  and the answer should be a citation rather than a conversation.
- **The no-CLA position is examined once, deliberately, rather than inherited.**
  It is probably right — it is the lowest-friction posture and it is what the
  Rust ecosystem expects. Note the nuance that makes it safe: permissive inbound
  licensing already permits sublicensing, so `CONTRIBUTING.md`'s current position
  does *not* close the commercial door. A CLA would only be needed to offer
  others' contributions proprietarily *while* preventing them from doing the
  same, which is a commitment to a licence-compliance business and should be
  entered on purpose or not at all.
- **A trademark search on "happenstance" comes back clean enough to file.** The
  word is common; this is a real risk and it is checkable cheaply. If the mark is
  unavailable, the third layer above loses its instrument and the whole strategy
  needs rethinking — so this is the first question, not the last.
- **The public statement exists.** A `LICENSING.md` that says, in the project's
  own voice, what is permissive and will stay permissive, what commercial
  offerings exist, and what the trademark policy is. Adoption of a young library
  by a company is a bet on the maintainer's future behaviour; the licences that
  destroyed value did it by proving that bet unsafe. Saying the promise out loud,
  early, is how the bet gets made.

## Sources

Research conducted 2026-08-16.

- Google's AGPL ban: <https://opensource.google/documentation/reference/using/agpl-policy>
- Heather Meeker, *AGPL In the Light of Day*: <https://heathermeeker.com/2023/10/13/agpl-in-the-light-of-day/>
- OpenTofu fork and the Terraform relicensing: <https://en.wikipedia.org/wiki/OpenTofu>,
  <https://opentofu.org/manifesto/>, <https://spacelift.io/blog/terraform-license-change>
- SurrealDB's split (BUSL core, Apache-2.0/MIT libraries and SDKs):
  <https://github.com/surrealdb/license/blob/main/README.md>
- Functional Source License: <https://fsl.software/>; Armin Ronacher's argument for it:
  <https://lucumr.pocoo.org/2024/9/23/fsl-agpl-open-source-businesses/>; the
  counter-case that fair-source noncompetes are legally fuzzy:
  <https://techcrunch.com/2024/09/22/some-startups-are-going-fair-source-to-avoid-the-pitfalls-of-open-source-licensing/>
- iText's dual-licence history: <https://itextpdf.com/how-buy/AGPLv3-license>,
  <https://pdfa.org/a-perfect-blend-of-open-source-and-commercial-driving-development-and-innovation-for-all/>
- MPL-2.0's file-level copyleft: <https://www.mozilla.org/en-US/MPL/2.0/FAQ/>,
  <https://fossa.com/blog/open-source-software-licenses-101-mozilla-public-license-2-0/>
- LGPL and Rust static linking: <https://users.rust-lang.org/t/crate-organization-and-metadata-for-bindings-to-an-lgpl-library/40523>
- What licences Rust crates actually choose: <https://lawngno.me/blog/2025/03/06/crate-licences.html>
