+++
title = "The guide"
description = "Keep your data true on devices, at the edge and in the cloud: where to start, what to build, and how it works. Every Rust example in it is checked by the repository's tests."
sort_by = "weight"
template = "section.html"
page_template = "page.html"
+++

## Start from what you want

- **See it work.** [Your first encounter](@/guide/first-encounter.md): three
  short programs that end with the store refusing a write that would have broken
  a rule.
- **Plan your app.** [Choose a topology](@/guide/choose-a-topology.md): where
  each log, view and decision lives across devices, the edge and the cloud.
- **Pick a database.** [Choose a store](@/guide/choose-a-store.md): SQLite,
  Postgres, Neon or a Durable Object, and what each one refuses.
- **Run at the edge.** [Run on a Durable Object](@/guide/run-on-a-durable-object.md):
  the event store inside Cloudflare, next to your users.

## By pillar

| You want your data to stay… | Read |
| --- | --- |
| **True offline** | [Choose a topology](@/guide/choose-a-topology.md), then [Sync a device to a central store](@/guide/sync-a-device.md) |
| **True to your rules** | [Appending under a condition](@/guide/append-conditions.md), [carrying your rule into your domain](@/guide/carry-your-invariant.md), [retrying a command safely](@/guide/retry-a-command.md) |
| **True to its history** | [Rebuilding a read model](@/guide/rebuild-a-read-model.md), [the worked example](@/guide/read-the-worked-example.md) |
| **True wherever it lives** | [Choose a store](@/guide/choose-a-store.md), [run on a Durable Object](@/guide/run-on-a-durable-object.md), [bring your own store](@/guide/adapter-reading-order.md) |

## New to event sourcing?

Four short background pages, each ending with a link to the next. Read them if
the pages above use words you have not met.

1. [Events, and the systems built on them](@/guide/event-driven-systems.md):
   what an event is, and why systems are built around them.
2. [Event sourcing, and CQRS](@/guide/event-sourcing-and-cqrs.md): what changes
   when the events become the record.
3. [Aggregates, and the boundary drawn in advance](@/guide/aggregates.md): how
   most event-sourced systems keep decisions safe, and where that runs out.
4. [Dynamic consistency boundaries](@/guide/what-is-dcb.md): how happenstance
   checks a rule across as many records as it needs.

## Everything else

Every page states, under its title, the one question it answers, and the menu
groups them by what you want to do: understand an idea, learn by running code,
or get a specific job done. The [reference](@/reference.md) says what is checked
and where, and the [API](../api/happenstance/index.html) documents every public
item.

The pages live in the repository's `docs/` directory, and this site renders
them unchanged: an example that stopped compiling would fail the build before it
reached you.
