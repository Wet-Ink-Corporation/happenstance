#!/usr/bin/env python3
"""The six edits this experiment makes to a scratch checkout, and nothing else.

Every one of them is applied inside ``$WT`` -- a detached ``git worktree`` of the
pinned commit -- never to the repository's own working tree, and every one is
undone by ``git -C "$WT" checkout -- .`` between arms.  Nothing here writes
inside ``D:\\repos\\happenstance`` except this experiment's own ``results/``.

    mutate.py ignore <file> <fn>          add `#[ignore]` to one test
    mutate.py ignore-reason <file> <fn>   add `#[ignore = "…"]` to one test
    mutate.py rename <file> <fn> <new>    rename one test  (the CONTROL)
    mutate.py guard  <file>               delete spec_trace's seven guard terms
    mutate.py guard1 <file> <term-key>    delete exactly one of them
    mutate.py fence  <file>               put a `non_snake_case` binding in a fence
    mutate.py fence-typeerror <file>      a type error in the same place (the CONTROL)

**Line endings are load-bearing and that is why this is not a `sed` one-liner.**
`core.autocrlf` is `true` on the measuring machine, so every file in the checkout
is CRLF while the blob in git is LF.  `cargo fmt --check` is the gate's *first*
step and rustfmt's default `newline_style = "Auto"` infers the ending from the
file, so an inserted bare ``\\n`` turns the gate red at step 0 -- a red that says
nothing whatever about `#[ignore]`.  Getting the answer "red" for the wrong
reason is the exact failure mode this experiment exists to name, so it is not
allowed to happen inside the instrument.  Every write below reproduces the
ending the file already uses.

The `ignore` edit also reproduces the *indentation* of the `fn` line it sits
above, for the same reason.
"""

from __future__ import annotations

import re
import sys


def read(path: str) -> tuple[str, str]:
    """The file's text with newlines normalised, and the ending it really uses."""
    raw = open(path, "rb").read().decode("utf-8")
    eol = "\r\n" if "\r\n" in raw else "\n"
    return raw.replace("\r\n", "\n"), eol


def write(path: str, text: str, eol: str) -> None:
    open(path, "wb").write(text.replace("\n", eol).encode("utf-8"))


# The two spellings, and the difference between them is the headline of count 1.
#
# A bare `#[ignore]` is refused by `clippy::ignore_without_reason`, which this
# workspace enables through `pedantic = "warn"` (`Cargo.toml:168`) and `-D
# warnings` promotes to an error.  Clippy's own help line on that diagnostic is
# `help: add a reason with = ".."`, so the *second* spelling is the one an author
# reaches in the same keystroke, and it is the one the count is about.
IGNORE_SPELLING = {
    "ignore": "#[ignore]",
    "ignore-reason": '#[ignore = "measured by experiments/gate-vacuity"]',
}


def op_ignore(path: str, fn: str, attr: str, guard: str | None = None) -> None:
    """Insert `attr` immediately above `fn`'s signature.

    `guard` is an attribute line that must sit directly above the signature, and
    it exists because `the_probe_is_not_vacuous` is spelled three times in
    `happenstance-cloudflare/src/lib.rs` — once as the probe, once as the host
    `#[test]` twin and once as the `#[wasm_bindgen_test]` twin. Editing the wrong
    one measures the wrong target, so the ambiguity is refused rather than
    resolved by counting occurrences.
    """
    text, eol = read(path)
    head = r"^([ \t]*)" if guard is None else r"^[ \t]*" + re.escape(guard) + r"\n([ \t]*)"
    pattern = re.compile(
        head + r"((?:pub )?(?:async )?fn " + re.escape(fn) + r"\()", re.M
    )
    replacement = (
        (lambda m: f"{m[1]}{attr}\n{m[1]}{m[2]}")
        if guard is None
        else (lambda m: f"{m[1]}{guard}\n{m[1]}{attr}\n{m[1]}{m[2]}")
    )
    text, n = pattern.subn(replacement, text)
    if n != 1:
        sys.exit(f"mutate: `fn {fn}(` matched {n} times in {path}, wanted 1")
    write(path, text, eol)


def op_rename(path: str, fn: str, new: str) -> None:
    text, eol = read(path)
    text, n = re.subn(r"\bfn " + re.escape(fn) + r"\(", f"fn {new}(", text)
    if n != 1:
        sys.exit(f"mutate: `fn {fn}(` matched {n} times in {path}, wanted 1")
    write(path, text, eol)


# The seven substrings `rules_of` tests for, keyed by the name this experiment
# reports them under.  Transcribed from `xtask/src/spec_trace.rs:1628-1634`.
GUARD_TERMS = {
    "unit-test": 'text.contains("unit test")',
    "compile-test": 'text.contains("compile test")',
    "meta-test": 'text.contains("meta-test")',
    "paren-new": 'text.contains("(new)")',
    "dagger": "text.contains('\u2020')",
    "leading-new": 'text.trim_start().starts_with("new ")',
    "space-new-tick": 'text.contains(" new `")',
}

ALL_KEYS = (
    "paren-new",
    "dagger",
    "leading-new",
    "space-new-tick",
    "unit-test",
    "compile-test",
    "meta-test",
)


def rewrite_guard(text: str, drop: set[str]) -> str:
    """Rebuild `rules_of`'s `schedules_new`, and *only* it, without `drop`'s terms.

    **`elsewhere` is left computing exactly what it computed before, and that is
    the whole design of this edit.**  Three of the seven substrings reach
    `schedules_new` through `elsewhere`, but `elsewhere` is also read on its own
    by check 7 and by the §7.2 renderer (`spec_trace.rs:1158`, `:1383`).
    Deleting a term from `elsewhere` would therefore change two checks at once,
    and the delta in the report could no longer be attributed to check 4 — which
    is the only check this experiment is asking about.  So the three are
    *inlined* into `schedules_new` as their own `text.contains(…)` calls and
    dropped from there, leaving `elsewhere` byte-identical.

    With every term dropped the statement becomes `let schedules_new = false;`,
    written out longhand rather than deleted, so the surrounding code compiles
    unedited.
    """
    kept = [GUARD_TERMS[k] for k in ALL_KEYS if k not in drop]
    schedules = "\n        || ".join(kept) if kept else "false"

    old = re.compile(r"    let schedules_new = .*?\|\| elsewhere;\n", re.S)
    text, n = old.subn(lambda _m: f"    let schedules_new = {schedules};\n", text)
    if n != 1:
        sys.exit(f"mutate: the `schedules_new` statement matched {n} times, wanted 1")
    return text


def main() -> None:
    op, path = sys.argv[1], sys.argv[2]
    if op in IGNORE_SPELLING:
        guard = sys.argv[4] if len(sys.argv) > 4 else None
        op_ignore(path, sys.argv[3], IGNORE_SPELLING[op], guard)
    elif op == "rename":
        op_rename(path, sys.argv[3], sys.argv[4])
    elif op == "guard":
        text, eol = read(path)
        write(path, rewrite_guard(text, set(GUARD_TERMS)), eol)
    elif op == "guard1":
        key = sys.argv[3]
        if key not in GUARD_TERMS:
            sys.exit(f"mutate: unknown guard term {key}")
        text, eol = read(path)
        write(path, rewrite_guard(text, {key}), eol)
    elif op in ("fence", "fence-typeerror"):
        # One binding, inside the first fence of one atom, violating exactly one
        # warn-by-default rustc lint and nothing else.  It is *used*, so
        # `unused_variables` cannot fire and confuse the attribution: if the docs
        # step goes red, `non_snake_case` is the only lint that can have done it.
        #
        # It goes in as two `#`-hidden doctest lines immediately after the
        # fence's own hidden `fn main() {`, and both halves of that are chosen
        # rather than convenient.  *Inside main*, because a `let` hoisted above
        # the fence's hidden preamble would land at module scope and produce a
        # compile error -- a red that says nothing about lints.  *Hidden*,
        # because the step before this one in `REQUIRED` is `lint-constitution`,
        # which reads these files as text; leaving the rendered fence
        # byte-identical is what keeps the docs step the only thing this edit can
        # possibly be answered by.
        # `fence-typeerror` is the control for `fence`, and it is not optional.
        # A green docs step under `fence` has two explanations -- the lint did
        # not fire, or the edit never reached a compiler at all -- and only a
        # deliberate type error in the *same* position tells them apart.
        body = (
            "# let _typeerror: u8 = \"not a u8\";\n"
            if op == "fence-typeerror"
            else "# let notSnakeCase = 1u8;\n# assert_eq!(notSnakeCase, 1);\n"
        )
        text, eol = read(path)
        text, n = re.subn(
            r"^# fn main\(\) \{\n",
            "# fn main() {\n" + body,
            text,
            count=1,
            flags=re.M,
        )
        if n != 1:
            sys.exit(f"mutate: no hidden `# fn main() {{` fence in {path}")
        write(path, text, eol)
    else:
        sys.exit(f"mutate: unknown op {op}")


if __name__ == "__main__":
    main()
