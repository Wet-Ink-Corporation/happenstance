+++
title = "The guide"
description = "From events to dynamic consistency boundaries to running code. Every Rust example in it is compiled and run by the repository's tests."
sort_by = "weight"
template = "section.html"
page_template = "page.html"
+++

## New here? Read these in order

Each page is short, and each one ends with a link to the next.

1. [Events, and the systems built on them](@/guide/event-driven-systems.md):
   what an event is, and why systems are built around them.
2. [Event sourcing, and CQRS](@/guide/event-sourcing-and-cqrs.md): what changes
   when the events become the record.
3. [Aggregates, and the boundary drawn in advance](@/guide/aggregates.md): how
   most event-sourced systems keep decisions safe, and where that runs out.
4. [Dynamic consistency boundaries](@/guide/what-is-dcb.md):
   the idea happenstance is built on.
5. [Your first encounter](@/guide/first-encounter.md): three short programs
   that end with the store refusing a write that would have broken a rule.

## Already know the background?

Every other page states, under its title, the one question it answers, and the
menu groups them by what you want to do: understand an idea, learn by running
code, or get a specific job done. The [reference](@/reference.md) says what is
checked and where, and the [API](../api/happenstance/index.html) documents every
public item.

The pages live in the repository's `docs/` directory, and this site renders
them unchanged: an example that stopped compiling would fail the build before it
reached you.
