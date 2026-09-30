+++
title = "The guide"
description = "Pages grouped by the one need each answers. Every Rust example on them is compiled by the repository's gate."
sort_by = "weight"
template = "section.html"
page_template = "page.html"
+++

Each page states, under its title, the single question it answers. If yours is
not among them, the [reference](@/reference.md) says what is checked and where,
and the [API](../api/happenstance/index.html) documents every public item.

The pages live in the repository's `docs/` directory, and this site renders
them unchanged: an example that stopped compiling would fail the build before it
reached you.
