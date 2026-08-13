---
title: "retention-deletion-and-incomplete-log-semantics — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "GDPR-driven deletion, storage-level retention, and concurrency-driven position gaps are three distinct, well-documented causes of an incomplete event log, and the wider ecosystem has no single agreed vocabulary for telling a reader which one it is looking at — which is exactly the silence the seed calls out as unsolved."
sources:
  - "https://www.conduktor.io/blog/gdpr-kafka-right-to-erasure"
  - "https://oneuptime.com/blog/post/2026-02-17-how-to-set-up-crypto-shredding-for-gdpr-right-to-erasure-compliance-in-google-cloud/view"
  - "https://event-driven.io/en/gdpr_in_event_driven_architecture/"
  - "https://verraes.net/2019/05/eventsourcing-patterns-throw-away-the-key/"
  - "https://verraes.net/2019/05/eventsourcing-patterns-forgettable-payloads/"
  - "https://danlebrero.com/2018/04/11/kafka-gdpr-event-sourcing/"
  - "https://docs.kurrent.io/server/v24.10/operations/scavenge"
  - "https://docs.kurrent.io/server/v22.10/streams"
  - "https://discuss.kurrent.io/t/clarification-on-deletes-and-scavenging/1825"
  - "https://docs.confluent.io/kafka/design/log_compaction.html"
  - "https://medium.com/@patrickduch93/kafka-tombstones-explained-deleting-data-in-a-compacted-topic-with-redpanda-c3ddc1bd3803"
  - "https://docs.ecotone.tech/modelling/event-sourcing/setting-up-projections/gap-detection-and-consistency"
  - "https://github.com/patchlevel/event-sourcing/issues/727"
  - "https://www.linkedin.com/pulse/ugly-event-sourcing-real-world-production-issues-dennis-doomen"
  - "https://github.com/akkadotnet/akka.net/issues/5478"
  - "https://github.com/akka/akka/issues/29685"
  - "https://dcb.events/specification/"
---

# retention-deletion-and-incomplete-log-semantics

## Findings

- **Three genuinely distinct mechanisms produce an incomplete log, and the ecosystem
  does not use one word for them.** (1) Lawful deletion under GDPR Article 17
  (right to erasure), (2) storage-level retention/scavenging done for cost or
  operational reasons with no legal trigger, and (3) transient position gaps caused
  by concurrent writers committing out of order. Each is documented separately in
  the wild, by different communities, with no shared terminology connecting them —
  which matters because a reader needs to react to them differently (see below).

- **GDPR erasure and append-only immutability are treated across the industry as a
  named, unresolved architectural tension, not a solved problem.** "Article 17
  particularly causes problems for systems based on permanent and tamper-proof data
  storage solutions, such as blockchain or immutable event stores"
  (danlebrero.com). Two competing patterns dominate practitioner writing —
  crypto-shredding and forgettable payloads — and both are documented with
  explicit, non-trivial caveats rather than as clean solutions.

- **Crypto-shredding is the most-recommended pattern but is legally contested and
  operationally leaky.** It deletes a per-entity encryption key instead of the
  data, which is attractive because "deleting the encryption key effectively
  removes the data from the backups" (oneuptime.com) — backups being exactly the
  kind of copy an event-sourced system cannot surgically edit. But Verraes'
  writeup is explicit that it does not protect against "consumers that store
  decrypted sensitive data or use it to compute new sensitive values," and flags
  that "today's unbreakable encryption could be tomorrow's infosec disaster."
  event-driven.io goes further: some legal commentary holds that GDPR does **not**
  recognize key-deletion as equivalent to data deletion at all, because the
  encrypted personal data is still "personal data" under the regulation.

- **Forgettable payloads (store a pointer, not the PII, in the event) is the other
  named pattern, and it explicitly breaks the event store's status as sole source
  of truth.** Verraes: it "requires [maintaining] a separate storage, so it breaks
  the concept of an Event Store as the Single Source of Truth" — and
  event-driven.io adds it is "not entirely foolproof" because of race conditions
  and reader-side caching of the resolved PII after the pointer target is gone.

- **Shipped, mature event stores treat retention and deletion as first-class,
  differentiated product surface, not an afterthought.** Kurrent/EventStoreDB
  exposes per-stream `$maxAge`/`$maxCount` metadata, and distinguishes **soft
  delete** (stream can be reopened, events scavenged later) from **hard delete**
  (a tombstone event that permanently forbids reopening the stream and is itself
  never scavenged). Scavenging is explicitly documented as destructive and
  irreversible outside of backups. This is strong evidence that "what may this
  store forget" is not a hypothetical question a mature adapter target will need
  answered — it is already a shipped, versioned feature surface with its own
  vocabulary (soft/hard, age/count, tombstone).

- **Kafka's compaction model shows deletion markers can themselves expire, creating
  a second-order incompleteness a naive consumer cannot detect.** A tombstone
  (null-value record) marks a key deleted, but "delete markers... are also cleaned
  out of the log after a period of time to free up space" via
  `delete.retention.ms` (Medium/Duch, Confluent docs). A consumer that was offline
  longer than that window sees neither the original record nor the tombstone —
  it simply never learns the key was ever deleted, and may retain the pre-deletion
  value forever if it caches state. This is a concrete, named example of "a
  reader that quietly builds a wrong answer from a truncated log."

- **Position/sequence gaps from concurrent writers are a distinct, well-documented
  failure mode already treated as production-serious, independent of any deletion
  policy.** Reported pattern: "a projection can process event #11 while event #10
  isn't visible yet, causing the event to be silently skipped forever... with no
  error, no log entry, and no exception" — discovered "months later when a
  customer reports a wrong balance," with the only fix being "rebuild the
  projection from scratch" (search synthesis corroborated by
  docs.ecotone.tech and github.com/patchlevel/event-sourcing#727). Ecotone's own
  docs and Dennis Doomen's "The Ugly of Event Sourcing" post independently
  describe the same shape of bug: correct in dev (single-writer), silently wrong
  in production (concurrent writers).

- **At least one shipped framework (Ecotone) explicitly taxonomizes *why* a
  position is missing, and treats the taxonomy as operationally different
  outcomes** — out-of-order commits (transient, expected to heal, tracked as
  `"11:10"` pending catch-up), rolled-back transactions (permanent, benign gap),
  and "genuine deletions" (permanent, and the one case that is not benign). This
  is direct evidence that a single "gaps MAY exist" rule, undifferentiated by
  cause, is not what a serious implementation actually needs — it needs to know
  which of (heals itself / permanently and harmlessly empty / permanently and
  meaningfully empty) it is looking at.

- **The DCB specification itself is silent on all of this.** Direct check of
  `dcb.events/specification/`: it permits sequence positions to contain gaps, but
  documents no cause, no vocabulary, and no reader-facing distinction between a
  transient gap, a rolled-back write, and a deleted event. `happenstance`
  inherits a spec that licenses the *symptom* (gaps are legal) without saying
  anything about the *causes* a reader might need to tell apart. Any answer this
  initiative writes here is an extension of DCB, not a clarification of an
  existing DCB position — worth stating plainly rather than assuming upstream
  guidance exists.

- **Snapshotting and deletion are coupled in at least one mature framework,
  and only partially.** Akka Persistence's snapshot-then-delete-prior-events
  capability is reported as currently working only "when using a snapshot
  strategy based on a counter and retention criteria," with the predicate-based
  variant explicitly *not* supporting paired deletion
  (github.com/akka/akka#29685, github.com/akkadotnet/akka.net#5478). That is
  evidence that "snapshot implies it's safe to forget everything before it" is a
  claim mature frameworks have only partially made good on, and only along
  specific, narrower policies than the general case.

## Evidence & citations

| Claim | Source |
| --- | --- |
| GDPR erasure vs. immutable/append-only log is a named, industry-wide unresolved tension | danlebrero.com/2018/04/11/kafka-gdpr-event-sourcing; event-driven.io/en/gdpr_in_event_driven_architecture |
| Crypto-shredding: mechanism, backup benefit, and "does not protect decrypted copies downstream" caveat | verraes.net/2019/05/eventsourcing-patterns-throw-away-the-key; oneuptime.com crypto-shredding GCP post |
| Crypto-shredding may not satisfy GDPR's legal definition of erasure | event-driven.io/en/gdpr_in_event_driven_architecture |
| Forgettable payloads pattern and its "breaks single source of truth" + race-condition caveats | verraes.net/2019/05/eventsourcing-patterns-forgettable-payloads; event-driven.io/en/gdpr_in_event_driven_architecture |
| EventStoreDB/Kurrent soft delete vs. hard delete (tombstone) vs. scavenging, `$maxAge`/`$maxCount` | docs.kurrent.io/server/v24.10/operations/scavenge; docs.kurrent.io/server/v22.10/streams; discuss.kurrent.io/t/clarification-on-deletes-and-scavenging/1825 |
| Kafka tombstones themselves expire (`delete.retention.ms`), producing a second-order silent gap for lagging consumers | docs.confluent.io/kafka/design/log_compaction.html; medium.com/@patrickduch93 Kafka tombstones article |
| Concurrent-writer position gaps cause silent, undetected projection divergence discovered only via a downstream symptom | docs.ecotone.tech gap-detection-and-consistency; github.com/patchlevel/event-sourcing/issues/727; linkedin.com/pulse "The Ugly of Event Sourcing" (Dennis Doomen) |
| Ecotone's three-way gap taxonomy (out-of-order/transient, rolled-back/benign, deleted/meaningful) and its `"pos:gap"` tracking format | docs.ecotone.tech/modelling/event-sourcing/setting-up-projections/gap-detection-and-consistency |
| DCB specification permits position gaps but is silent on cause or reader-facing vocabulary for them | direct fetch of dcb.events/specification/ |
| Akka Persistence couples snapshot + delete-prior-events only for counter/retention-based strategies, not predicate-based | github.com/akka/akka/issues/29685; github.com/akkadotnet/akka.net/issues/5478 |

## Implications for the idea

- **The open question this angle was assigned to — "what may a store forget, and
  how does a reader find out?" — is not a speculative worry invented for this
  initiative; it is a named, independently-solved-differently-everywhere problem
  across the mature ecosystem.** Every shipped system this research touched
  (Kurrent, Kafka, Akka) answers it in its own vocabulary and none of them agree
  with each other. That is evidence the honest-answer-or-refusal bar the desired
  outcome sets is calibrated correctly: silence here would put `happenstance` in
  the position every other mature store had to leave behind.

- **"A gap" is not one thing, and treating it as one thing is the exact failure
  mode the seed names.** The research surfaces at least four distinguishable
  states a missing position can represent — transient/will-heal (concurrent
  in-flight write), permanent/benign (rolled-back write), permanent/meaningful
  but physically-still-present-and-shredded (crypto-shredded), and
  permanent/actually-gone (hard-deleted or scavenged). A reader that cannot tell
  these apart is the "quietly builds a wrong answer" failure the seed and brief
  both flag — and at least one shipped framework (Ecotone) has already found it
  necessary to name and track this distinction operationally, which is corroborating
  evidence this is a real design surface and not decorative.

- **Because DCB's own specification is silent on gap *cause*, this initiative
  cannot treat "what a reader may assume about the log" as a question DCB has
  already answered upstream.** Whatever this project settles here extends the DCB
  contract rather than implements an existing DCB position — which is exactly the
  kind of decision the seed says needs a written answer or an explicit, reasoned
  refusal, not silence, and probably its own ADR rather than something folded
  quietly into an existing frozen clause.

- **The load-bearing local-first/edge audience and the "storage shapes that
  disagree" axis both intersect this question in ways the wider research
  surfaces but does not resolve.** Per-entity crypto-shredding is the pattern the
  industry converges on, and it is naturally cheap when a store already isolates
  one tenant's data (a Durable Object, a per-tenant SQLite file) — but the
  research gives no evidence either way about whether it stays cheap against a
  shared multi-tenant store, which is the shape this project already carries at
  its other end. That asymmetry is itself a finding worth carrying into planning,
  not a reason to assume the answer generalizes.

- **The Kafka tombstone-expiry case is a specific, concrete cautionary example
  worth keeping in view when this initiative writes its own answer**: even a
  system that does everything "right" (issues a real, honest tombstone) can still
  produce a permanently wrong reader if the reader was absent long enough — the
  failure is not only "did we delete correctly" but "can every reader, including
  a slow or resumed one, learn that we did." That is a behavior/contract question,
  not an implementation detail, and belongs in the same place as the rest of this
  initiative's problem-space framing.

- **Snapshot-coupled deletion (Akka's counter/retention-only support) is evidence
  that "we snapshot, so it's safe to forget the rest" is a claim mature systems
  have only partly earned, and only under specific policies.** Any assumption that
  a snapshot mechanism automatically licenses forgetting prior events should be
  treated as its own claim needing its own justification, not inherited for free
  from the existence of snapshots.
