# Brand assets

The shipped artwork for the happenstance identity. All SVG; wordmarks are outlined
paths, so nothing here needs a font installed at the point of use.

| File | Use |
| --- | --- |
| `happenstance-lockup-horizontal.svg` | Primary lockup. Ink wordmark, for light surfaces |
| `happenstance-lockup-horizontal-reversed.svg` | Primary lockup. Paper wordmark, for ink surfaces |
| `happenstance-lockup-stacked.svg` | Square and near-square placements |
| `happenstance-wordmark.svg` | Wordmark alone |
| `happenstance-mark.svg` | Mark, primary weight. 32px and above |
| `happenstance-mark-compact.svg` | Mark, compact weight. 32px and below |
| `happenstance-mark-mono.svg` | Mark in a single inherited colour via `currentColor` |
| `favicon.svg` | Ink badge for browser chrome, avatars and app icons |

## Choosing a lockup for light and dark

**Each file carries one fixed colour.** Selection happens at the point of use, because
the page knows its own background and the file does not:

```html
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/happenstance-lockup-horizontal-reversed.svg">
  <img src="assets/brand/happenstance-lockup-horizontal.svg" alt="happenstance">
</picture>
```

Do not add a `prefers-color-scheme` media query inside an SVG here. A standalone SVG
knows the operating system's preference but not the colour of the surface it was placed
on, so a light page opened on a machine set to dark mode renders the wordmark
cream-on-cream.

## Palette

`--sun:#FFB627` · `--deep:#E08700` · `--ember:#A85B00` · `--ink:#2A211B` · `--paper:#FFFCF4`

Sun is a graphic fill. On light surfaces it measures 1.71:1 and is never used for text;
Ember is the amber text colour at 4.92:1. On ink, Sun reads 10.34:1, and 10.79:1 on
GitHub dark `#0D1117`.

## Rules that bind

Seven blocks, always. One block at twelve o'clock; the mark never rotates. Scale
uniformly — no stretching, condensing or arching. One flat fill from the palette, with
no gradients, shadows, glows or outlines. Clear space of one block length on every side.

The full specification, including construction geometry and lockup measurements, is
[`references/brand/brand-kit.html`](../../references/brand/brand-kit.html).
