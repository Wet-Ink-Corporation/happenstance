---
title: "replication-and-position-identity-across-store-boundaries — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "Across every mature event-log ecosystem, position/offset is treated as cluster-local and non-portable by construction; the industry's actual answer for cross-boundary identity is a stable per-event id carried in the payload, while ordering, dedup-loop prevention, and 'did the receiving side re-check what the writer asserted' are each solved ad hoc, inconsistently, and usually only after an outage taught the lesson."
sources: [
  "https://www.warpstream.com/blog/kafka-replication-without-the-offset-gaps",
  "https://cwiki.apache.org/confluence/display/KAFKA/KIP-1279:+Cluster+Mirroring",
  "https://www.conduktor.io/glossary/kafka-mirrormaker-2-for-cross-cluster-replication",
  "https://lists.apache.org/thread/2726x5pzq6zb31z6tvrjf54mb8t07309",
  "https://github.com/confluentinc/event-streaming-patterns/blob/main/docs/compositional-patterns/geo-replication.md",
  "https://discuss.kurrent.io/t/replicating-events-across-datacenters/300",
  "https://dcb.events/specification/",
  "https://dcb.events/faq/",
  "https://www.cockroachlabs.com/glossary/distributed-db/hybrid-logical-clock-hlc-timestamps/",
  "https://www.trinitylogic.co.uk/blog/kafka-consumer-idempotency-exactly-once/",
  "https://www.conduktor.io/blog/building-idempotent-consumers",
  "https://dev.to/neon-postgres/comparing-local-first-frameworks-and-approaches-1hgn"
]
---
# replication-and-position-identity-across-store-boundaries

## Findings

- **Every mature log system treats position/offset as scoped to one cluster, and none of the standard replication tools make it survive a boundary crossing unchanged.** Kafka's own replication tooling (MirrorMaker 2) does not attempt to preserve offsets — it builds a lossy translation map instead, because storing an exact mapping "for every single record" is too expensive to maintain. A newer alternative (WarpStream's Orbit) exists specifically because that lossiness was judged unacceptable, and its entire pitch is "every record... maintains its exact offset, including any offset gaps" — which only works because it constrains the *topology* (dedicated pass-through) rather than solving translation in general. Two competing philosophies exist in production today — translate-and-accept-loss vs. preserve-and-constrain-topology — and neither is "positions just work across stores." (WarpStream: https://www.warpstream.com/blog/kafka-replication-without-the-offset-gaps; Conduktor on MM2: https://www.conduktor.io/glossary/kafka-mirrormaker-2-for-cross-cluster-replication)

- **Offset translation failure has a named, observed failure mode: unbounded duplicate delivery on failover.** The Kafka mailing list discussion of MM2's checkpoint connector states plainly that when a consumer fails over to the destination cluster, "there is an unfixed amount of duplicate consumption of records as the last offset mapping... could be much smaller than the last actually-committed consumer group offset." This is not a hypothetical edge case list item — it is the documented, expected behavior of the mainstream tool. (https://lists.apache.org/thread/2726x5pzq6zb31z6tvrjf54mb8t07309)

- **The field's actual answer to "what identifies an event across a boundary" is not the position — it's a stable identifier embedded in the event itself.** Multiple independent idempotent-consumer writeups converge on the same advice: using a log-coordinate composite (e.g., topic+partition+offset) as a dedup key "is fragile if topics get compacted or consumers need to reprocess historical events," and the fix is "embedding the idempotency key in the event payload itself as a stable UUID, generated at source, [which] survives redelivery unchanged" and lets "any downstream consumer... use it as a deduplication key without caring about partition or offset." (https://www.trinitylogic.co.uk/blog/kafka-consumer-idempotency-exactly-once/; https://www.conduktor.io/blog/building-idempotent-consumers)

- **Practitioners running the closest analog to happenstance — an event-sourcing-native store replicated across datacenters — report that ordering is the first thing sacrificed, not an incidental cost.** In a Kurrent/EventStoreDB community thread with EventStoreDB's own creator participating, the cross-datacenter "shovel" pattern (subscribe on one node, push to another) is described as explicitly giving up global ordering: "you will not have the same ordering and consistency." The thread also surfaces a second, easy-to-miss problem: without tagging each event's datacenter of origin in its metadata, a bidirectional shovel replicates its own replicated events back and forth forever — the fix offered was ad hoc, application-level provenance tagging, not anything the store did for them. (https://discuss.kurrent.io/t/replicating-events-across-datacenters/300)

- **The DCB specification — the contract happenstance is implementing — is silent by scope on everything cross-boundary.** Sequence Position is defined only as unique-within-the-store, monotonic, and permitted to have gaps; the specification's Append Condition mechanism (`after`, ignoring events before a remembered position) is defined entirely in terms of one store's position space. There is no clause addressing what a position means, or whether it means anything, once it has crossed into a second store. This is not an oversight happenstance's authors introduced — it is the boundary of what the wire-level protocol standard itself claims to answer, confirmed directly against the spec text. (https://dcb.events/specification/)

- **Where the industry has actually built cross-node position identity that means something, it did so by inventing a new kind of clock, not by extending offsets.** Hybrid Logical Clocks (physical time + logical counter, used by CockroachDB, MongoDB, and YugabyteDB) exist precisely because a bare sequence number has no meaning once more than one node can assign one, and because a bare wall clock isn't precise or synchronized enough to order events either. The pattern that generalizes is: causal, cross-node comparable position is a *different artifact* from a single-store monotonic counter, engineered in for that purpose, not a byproduct you get by replicating the counter. (https://www.cockroachlabs.com/glossary/distributed-db/hybrid-logical-clock-hlc-timestamps/)

- **The adjacent local-first / sync-engine ecosystem is having the identical argument under a different name, unresolved as of 2026.** A survey of local-first sync frameworks frames the live design question as "which sync engine boundary — replicate Postgres rows, replicate document ops, or replicate event logs — and what that choice does to schema migrations and *multi-device identity*." That is happenstance's open question (hub-and-spoke vs. peer-to-peer, and what a peer must re-verify) restated by a completely different community working on completely different storage, which suggests it isn't a happenstance-specific gap to be embarrassed about closing quickly — it's an open problem the wider field is still actively contesting. (https://dev.to/neon-postgres/comparing-local-first-frameworks-and-approaches-1hgn)

## Evidence & citations

| Claim | Source |
| --- | --- |
| MM2 offset translation is a lossy map, not exact preservation, because per-record mapping is too costly | Conduktor MM2 glossary: https://www.conduktor.io/glossary/kafka-mirrormaker-2-for-cross-cluster-replication |
| WarpStream Orbit's differentiator is exact offset (incl. gap) preservation, contrasted with MM2's approach | https://www.warpstream.com/blog/kafka-replication-without-the-offset-gaps |
| MM2 does not translate offsets stored outside the Kafka consumer-group protocol (e.g. Flink, Spark) | https://www.conduktor.io/glossary/kafka-mirrormaker-2-for-cross-cluster-replication |
| Failover after imprecise offset mapping causes an "unfixed amount of duplicate consumption" | Kafka users mailing list: https://lists.apache.org/thread/2726x5pzq6zb31z6tvrjf54mb8t07309 |
| Recommended idempotency key is a payload-embedded stable UUID generated at source, not the log coordinate | https://www.trinitylogic.co.uk/blog/kafka-consumer-idempotency-exactly-once/ and https://www.conduktor.io/blog/building-idempotent-consumers |
| EventStoreDB cross-datacenter "shovel" replication explicitly sacrifices global ordering | Kurrent/EventStore Discuss forum thread (Greg Young, Thomas Weiss, Igor Berger participating): https://discuss.kurrent.io/t/replicating-events-across-datacenters/300 |
| Preventing replication loops in that setup requires application-level datacenter-of-origin tagging in event metadata | same thread |
| Kafka cluster/geo replication is asynchronous by default: "an event that is recorded in the source may not be immediately available at the destination" | Confluent event-streaming-patterns geo-replication doc: https://github.com/confluentinc/event-streaming-patterns/blob/main/docs/compositional-patterns/geo-replication.md |
| DCB spec defines Sequence Position as unique + monotonic + gap-permitting, scoped to "the Event Store" (singular), with no cross-store clause | https://dcb.events/specification/ |
| DCB's Append Condition (`after`) is defined entirely in one store's position space | https://dcb.events/specification/ |
| Hybrid Logical Clocks were built because neither a bare counter nor a bare wall clock gives meaningful cross-node event order; adopted by CockroachDB, MongoDB, YugabyteDB | https://www.cockroachlabs.com/glossary/distributed-db/hybrid-logical-clock-hlc-timestamps/ |
| The live 2026 local-first-sync design question is explicitly framed as "which sync boundary... and what that does to... multi-device identity" | https://dev.to/neon-postgres/comparing-local-first-frameworks-and-approaches-1hgn |

## Implications for the idea

- **The initiative's framing — "a `SequencePosition` is meaningful only within one store and cannot be replicated as-is" — is not a hedge; it is the field's consensus, arrived at the hard way.** Every mainstream replication tool researched here either accepts lossy position translation (and documents the resulting duplicate-delivery failure mode) or narrows its topology specifically to dodge the problem. That strengthens the brief's framing that this is a real open question owed a decision, not a corner that can be silently assumed away.

- **The industry's converged answer to "how does a reader recognize the same event on both sides of a boundary" is identity, not position** — a stable per-event id carried in the event itself. This is independent evidence (from a completely different ecosystem: Kafka consumer patterns) that a store already separating "the event's own identity" from "its position in this store's log" is aligned with how the rest of the industry solved the adjacent problem, whatever happenstance ultimately decides about replication mechanics. It is not, on its own, an answer to whether *ingest re-checks the append condition a writer asserted* — that remains genuinely open and was not resolved by anything found here.

- **"Does replication preserve ordering" and "does replication re-verify what was asserted" are separable questions, and the evidence says most existing systems answer the first one "no" by default and treat that as acceptable.** The EventStoreDB practitioners explicitly traded away global ordering for cross-datacenter replication and called it "somewhat acceptable." That is useful grounding for framing the initiative's open question honestly: refusing to promise ordering across a replication boundary is a legitimate, precedented answer, not an evasion — but *silently* doing so, versus documenting it as a stated decision, is exactly the gap the intake brief already flags ("silence is not an outcome").

- **Loop-prevention in bidirectional replication is presently solved everywhere by manual, application-level provenance tagging** (datacenter name in event metadata), not by anything a store contract does for the writer. That is evidence this is a real, recurring operational problem class for anyone who replicates event logs both directions — worth naming explicitly as a scenario the eventual decision record should at least acknowledge, even if it stays out of scope for this initiative's first pass.

- **The DCB specification itself takes no position (pun unavoidable) on cross-store semantics**, which means happenstance cannot resolve this open question by re-reading the wire protocol more carefully — there is nothing there to find. Whatever gets decided will be an addition to what DCB specifies, not a clarification of it, and should be framed and evidenced that way rather than presented as "implementing what DCB already says."

- **This is not a problem unique to event-sourcing storage — the local-first/sync-engine community is contesting the structurally identical question in 2026 under different vocabulary** ("sync boundary," "multi-device identity"). That is useful context for scoping expectations: an initiative that produces "a decision or an explicit, reasoned refusal" here is producing something the wider field has not yet converged on either, which argues for treating the eventual ADR as a considered position taken under genuine uncertainty rather than a gap being closed on a known answer.
