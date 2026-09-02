# 81 — Checks that cannot be types

> **Load when:** writing a `cargo xtask` lint · a check must read Rust source, a
> manifest or a changelog · a gate step passed over code it never looked at ·
> asserting on a command's output · a check found nothing on its first run
> **See also:** 80 (the gate) · 60 (what a test must prove) · 70 (documenting a
> check) · 01 (standard of evidence)

---

## RS-81-1. Prove the check's blind spot in its own tests, then state it in its own documentation.

**Why.** There is no type that says "this crate may not observe time" and no
compiler pass that reads `CHANGELOG.md`, so some checks are greps — and a check
whose limits are undocumented is read as a guarantee. The failure that follows is
not the grep missing something; it is a reader deleting the *real* instrument
because the grep looks like it already covers the ground.

**Do** — the limits are executable, so execute them.

```rust
/// The cheap second line for "no rule asserts on a literal position": an
/// integer-list literal outside index position.
///
/// # What this does not verify
///
/// A position reached through a binding, a comparison against `position.get()`,
/// or a literal reached through a helper. The enforcement is the gapped-position
/// variant store, which refutes all three behaviourally.
fn flags(code: &str) -> bool {
    let chars: Vec<char> = code.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c != '[' {
            continue;
        }
        let Some(close) = chars[i + 1..].iter().position(|c| *c == ']') else {
            continue;
        };
        let inner: String = chars[i + 1..i + 1 + close].iter().collect();
        let tokens: Vec<&str> = inner.split(',').map(str::trim).filter(|t| !t.is_empty()).collect();
        let is_list = !tokens.is_empty()
            && tokens.iter().all(|t| t.chars().all(|c| c.is_ascii_digit()));
        let indexes = chars[..i]
            .iter()
            .rev()
            .find(|c| !c.is_whitespace())
            .is_some_and(|c| c.is_alphanumeric() || *c == '_' || *c == ')' || *c == ']');
        if is_list && !indexes {
            return true;
        }
    }
    false
}

fn main() {
    assert!(flags("assert_eq!(found, [1, 2, 3]);"));
    // Legitimate: the suite anchors on positions the store assigned.
    assert!(!flags("positions_of(&all[..1])"));
    // The two documented limits, asserted rather than promised.
    assert!(!flags("let expected = 3;"));
    assert!(!flags("found[0].position.get() == 3"));
}
```

**Not** — the same check documented as the enforcement.

```rust
/// No rule asserts on a literal sequence-position value.
fn no_position_literals(code: &str) -> bool {
    !code.contains("[1, 2, 3]")
}

fn main() {
    let rule = "let expected = 3; assert_eq!(found[0].position.get(), expected);";
    assert!(
        no_position_literals(rule),
        "reported clean over a rule that hard-codes a position"
    );
}
```

**Rejects.** A reader who takes the grep for the enforcement and deletes
`GappedPositionStore` — positions from 4,096 in steps of seven — as an elaborate
duplicate of a lint that already runs. The suite stays green and the lint stays
green, and the first adapter whose store does not start at 1 fails a rule nobody
can account for, in the field rather than in the testkit.

**Evidence.** `xtask/src/lints.rs:627 (GappedPositionStore)` ·
`xtask/src/lints.rs:693 (fn is_integer_list)` ·
[SPECIFICATION CF-6](../../spec/SPECIFICATION.md) ·
[SPECIFICATION CF-33](../../spec/SPECIFICATION.md)

---

## RS-81-2. Scan code only — and hard-error on the constructs the scanner cannot lex.

**Why.** Prose about a construct reads exactly like the construct:
`concurrency.rs` uses the word `sleep` four times while explaining why the
watchdog is absent, so a raw-byte grep fires on the sentence justifying its own
existence and the repair a hurried reader reaches for is to delete the sentence.
Blanking comments and string *contents* removes that class entirely — and the
four constructs that flip the scanner's idea of "inside a string" must be a hard
error, because the alternative failure is that every match silently stops
reporting.

**Do** — each construct is detected *while the scanner is in code*, which is the
only position where it can be told from the same bytes inside a string.

```rust
/// Source with `//` comments removed and string-literal contents blanked, or an
/// error naming a construct this is not a lexer for.
fn code_only(line: &str) -> Result<String, &'static str> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        i += 1;
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                out.push('"');
            }
            continue;
        }
        if c == '\'' && chars.get(i) == Some(&'"') {
            return Err("a char literal holding a quote: this file needs a lexer");
        }
        if c == '"' {
            in_string = true;
            out.push('"');
            continue;
        }
        if c == '/' && chars.get(i) == Some(&'/') {
            break;
        }
        out.push(c);
    }

    if in_string {
        return Err("a string literal still open at end of input");
    }
    Ok(out)
}

fn main() {
    assert_eq!(code_only("// why the watchdog is absent: no sleep"), Ok(String::new()));
    match code_only("panic!(\"a rule may not sleep\");") {
        Ok(code) => assert!(!code.contains("sleep")),
        Err(why) => panic!("{why}"),
    }
    match code_only("thread::sleep(delay);") {
        Ok(code) => assert!(code.contains("sleep")),
        Err(why) => panic!("{why}"),
    }
    assert!(code_only("let quote = '\"';").is_err());
}
```

**Not** — the best-effort scanner. It compiles, it reports success, and the
assertion below is the whole defect.

```rust
fn code_only_lossy(line: &str) -> String {
    let mut out = String::new();
    let mut in_string = false;
    for c in line.chars() {
        if c == '"' {
            in_string = !in_string;
            out.push('"');
            continue;
        }
        if !in_string {
            out.push(c);
        }
    }
    out
}

fn main() {
    let rule = "let quote = '\"'; thread::sleep(delay);";
    assert!(
        !code_only_lossy(rule).contains("sleep"),
        "one char literal blanks the rest of the file and every match stops reporting"
    );
}
```

**Rejects.** The demonstrated case: one `let quote` holding a quote character,
inserted into a rule, made both the clock lint and the position-literal lint
green over a rule body that read a clock *and* asserted on `[1, 2, 3]`. The
module used to *promise* that such a failure would be loud; it would not have
been, because a lint that stops reporting prints its success line and exits 0 —
the quietest failure available — and the promise told the next reader not to look.

**Evidence.** `xtask/src/lints.rs:116 (fn code_lines)` ·
`xtask/src/lints.rs:102 (for it. It was demonstrated: one)` · `xtask/src/lints.rs:129 (let unlexable = |line: usize, what: &str| -> anyhow::Error {)` ·
[SPECIFICATION CF-33](../../spec/SPECIFICATION.md)

---

## RS-81-3. Match whole identifiers, and scope the scan to the directory whose behaviour the check constrains.

**Why.** `str::contains` is a substring test and conformance rule names nest, so a
longer rule's changelog entry silently discharges a shorter rule's obligation and
inflates any per-rule arithmetic built on the same match. Scope is the mirror
image: `crates/happenstance-testkit/src` and not the whole crate, because
`tests/` holds wrong implementations that may legitimately use the constructs the
rules may not — and a check that fires there teaches the next person that the
remedy is to widen the exclusions, which is the direction that ends with the
check switched off.

**Do**

```rust
/// Whether an entry names a rule as a whole identifier.
fn names_rule(text: &str, rule: &str) -> bool {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    text.match_indices(rule).any(|(at, _)| {
        let before = text[..at].chars().next_back().is_none_or(|c| !ident(c));
        let after = text[at + rule.len()..].chars().next().is_none_or(|c| !ident(c));
        before && after
    })
}

fn main() {
    let entry = "- append_is_atomic_under_a_mid_batch_fault rejects a torn batch";
    assert!(names_rule(entry, "append_is_atomic_under_a_mid_batch_fault"));
    assert!(!names_rule(entry, "append_is_atomic"));
}
```

**Not**

```rust
fn main() {
    let entry = "- append_is_atomic_under_a_mid_batch_fault rejects a torn batch";
    assert!(
        entry.contains("append_is_atomic"),
        "and so a rule with no entry of its own was reported as satisfied"
    );
}
```

**Rejects.** Exactly what happened to `append_is_atomic` — the only rule in the
suite that rejects the write-then-check shape. It went a whole phase with no
changelog entry while the step reported it satisfied by the longer rule's entry,
so an adapter author taking the minor bump had no sentence anywhere telling them
which defect had just started failing their build.

**Evidence.** `xtask/src/lints.rs:515 (fn names_rule)` ·
`xtask/src/lints.rs:53 (TESTKIT_SRC)` · `xtask/src/lints.rs:246 (CLOCK_CONSTRUCTS)` ·
[SPECIFICATION CF-29](../../spec/SPECIFICATION.md)

---

## RS-81-4. Parse the command's output and assert names; a zero exit status is evidence of nothing.

**Why.** `cargo test --test foo` exits 0 on `running 0 tests`, so naming a target
checks the filename: a deletion fails the step and an *emptying* passes it. The
same argument covers `cargo package --list`, which exits 0 while listing an
artifact that carries neither licence — running a command and discarding its
output is the decorative-gate shape. A generated document region is the same
check once more: hold it to equality with what the checker computes, because the
generator is not the check.

**Do**

```rust
/// The tests a clause cites, asserted out of `--list` before they are run.
const EXPECTED: &[&str] = &[
    "mutation_coverage::every_rule_has_a_mutant",
    "mutation_coverage::mutants_fail_exactly_their_declared_rules",
];

fn missing(listing: &str) -> Vec<&'static str> {
    EXPECTED
        .iter()
        .copied()
        .filter(|name| {
            !listing
                .lines()
                .any(|line| line.trim().trim_end_matches(": test") == *name)
        })
        .collect()
}

fn main() {
    let full = "mutation_coverage::every_rule_has_a_mutant: test\n\
                mutation_coverage::mutants_fail_exactly_their_declared_rules: test\n";
    assert!(missing(full).is_empty());

    // The target emptied to its attributes. `cargo test` exits 0 over this.
    assert_eq!(missing("").len(), 2);
}
```

**Not** — abridged from the listing a real crate in this workspace produces
(`cargo package -p happenstance-sqlite --list`), against a step that only reads
the exit status.

```rust
const LISTING: &str = "Cargo.toml\nCargo.toml.orig\nsrc/lib.rs\n";

fn main() {
    let exited_zero = true;
    assert!(exited_zero, "which is all a step that discards the output learns");
    assert!(
        !LISTING.lines().any(|line| line == "LICENSE-MIT"),
        "the artifact ships neither licence and the gate is green"
    );
}
```

**Rejects.** A proof artefact truncated to its `#![cfg(…)]` attributes, or whose
meta-tests have been renamed or marked `#[ignore]`. Nothing else in the gate
would notice: the traceability checker reads only the rule file, so it never
resolves the meta-tests' names, and `cargo test --workspace` is just as happy
with one fewer target as with one more. The suite's own proof that it
discriminates would be gone, and the suite would keep certifying adapters.

**Evidence.** `xtask/src/proof.rs:15 (running 0 tests)` ·
`xtask/src/package.rs:20 (not hypothetical)` ·
`xtask/src/spec_trace.rs:451 (hand count against what this run computed)`

---

## RS-81-5. Derive the fact, keep the hand-written intention, and make the failure say which one moved.

**Why.** A derived set is what the tool will actually do; a hand list is what we
meant it to do. Deriving alone is smaller code and keeps the step honest, but it
reports one message for two unrelated bugs — a crate promoted on purpose whose
files were never copied, and a `publish = false` deleted by accident in a crate
nobody meant to ship. Only the difference between the two artefacts can name
which.

**Do**

```rust
use std::collections::BTreeSet;

/// What this workspace intends to publish.
const PUBLISHABLE: [&str; 3] = ["happenstance-core", "happenstance", "happenstance-testkit"];

/// Fails naming the direction, because the two directions are two bugs.
fn reconcile(derived: &BTreeSet<&str>) -> Result<(), String> {
    let declared: BTreeSet<&str> = PUBLISHABLE.into_iter().collect();
    let promoted: Vec<&&str> = derived.difference(&declared).collect();
    let withdrawn: Vec<&&str> = declared.difference(derived).collect();

    if !promoted.is_empty() {
        return Err(format!("cargo will publish {promoted:?}; a publish = false has gone missing"));
    }
    if !withdrawn.is_empty() {
        return Err(format!("the list names {withdrawn:?}; cargo will not publish it"));
    }
    Ok(())
}

fn main() {
    let derived: BTreeSet<&str> = PUBLISHABLE.into_iter().collect();
    assert!(reconcile(&derived).is_ok());

    let mut promoted = derived.clone();
    promoted.insert("happenstance-sqlite");
    match reconcile(&promoted) {
        Err(why) => assert!(why.contains("happenstance-sqlite")),
        Ok(()) => panic!("a stub that became publishable must be named"),
    }
}
```

**Not** — the hand list alone, which is the shape that stays green and silent.

```rust
const PUBLISHABLE: [&str; 3] = ["happenstance-core", "happenstance", "happenstance-testkit"];

fn main() {
    // What cargo will actually publish, after one line was deleted from a stub.
    let derived = [
        "happenstance-core",
        "happenstance",
        "happenstance-testkit",
        "happenstance-sqlite",
    ];
    assert_eq!(PUBLISHABLE.len(), 3);
    assert!(
        derived.contains(&"happenstance-sqlite"),
        "and nothing in the gate compares the two sets"
    );
}
```

**Rejects.** The one-line deletion of a skeleton's `publish = false` during a
tidy-up. The crate ships with neither licence text nor README while a hand list
naming three crates stays green, and the derivation that would have caught it
cannot be replaced by reading `crates/*/Cargo.toml` either — the workspace
members are globs, so a member added under a new path is invisible to precisely
the check that exists to notice new publishable members.

**Evidence.** `xtask/src/package.rs:86 (PUBLISHABLE)` ·
`xtask/src/package.rs:194 (fn reconcile)` · `xtask/src/package.rs:36 (The derivation is the fact)`
