# 80 — The gate

> **Load when:** adding a check to CI · `cargo xtask ci` is green and CI is not ·
> a gate step ran and proved nothing · choosing between a mandatory step and a
> probed one · a step needs an environment variable · raising the MSRV
> **See also:** 81 (checks a type cannot express) · 70 (rustdoc flags) ·
> 62 (doctests) · 92 (toolchain limits)

---

## RS-80-1. Add a check as a `Step` in `REQUIRED`, and reach it by name.

**Why.** A step is data — `name`, `program`, `args`, `env`, `probe` — so the gate
is one array, `cargo xtask ci` is the only thing CI invokes, and a green local
run means a green CI run. Anything added as a bespoke `Command` inside the runner
is invisible to `--fast` and to the `cargo xtask wasm` and `lints` subsets, all
three of which select out of that array.

**Do**

```rust
struct Step {
    name: &'static str,
    program: &'static str,
    args: &'static [&'static str],
}

const REQUIRED: &[Step] = &[
    Step { name: "formatting", program: "cargo", args: &["fmt", "--all", "--check"] },
    Step { name: "tests", program: "cargo", args: &["test", "--locked", "--workspace"] },
    Step {
        name: "wasm32 build of the contract crate",
        program: "cargo",
        args: &["check", "--locked", "--target", "wasm32-unknown-unknown"],
    },
];

/// Panicking is the right failure: the steps are compile-time constants, so a
/// name that resolves to nothing is a bug in this file, never a user error.
fn steps_named(names: &[&str]) -> Vec<&'static Step> {
    names
        .iter()
        .map(|name| {
            let Some(step) = REQUIRED.iter().find(|step| step.name == *name) else {
                panic!("REQUIRED must contain the step named {name}")
            };
            step
        })
        .collect()
}

fn main() {
    let wasm = steps_named(&["wasm32 build of the contract crate"]);
    assert_eq!(wasm.len(), 1);
    assert_eq!(wasm[0].program, "cargo");
    assert!(wasm[0].args.contains(&"wasm32-unknown-unknown"));
}
```

**Not** — the index selection this replaced, shown after two steps were inserted
above it. It compiles, it runs, and it prints green.

```rust
struct Step {
    name: &'static str,
}

const REQUIRED: &[Step] = &[
    Step { name: "formatting" },
    Step { name: "clippy (all targets, all features)" },
    Step { name: "tests" },
    Step { name: "each phase's proof artefacts" },
    Step { name: "wasm32 build of the contract crate" },
];

fn main() {
    // `cargo xtask wasm` was `&REQUIRED[3..4]`.
    let selected = &REQUIRED[3..4];
    assert_eq!(
        selected[0].name, "each phase's proof artefacts",
        "the standing guard on the !Send port flavour now runs nothing"
    );
}
```

**Rejects.** An index that still selects *a* step after an insertion above it, so
`cargo xtask wasm` reports success having compiled nothing for
`wasm32-unknown-unknown`. The one check keeping the bare port flavour honest is
then satisfied by a clippy run, and the failure surfaces at the first Workers
adapter — as a design that cannot be implemented, not as a red build.

**Evidence.** `xtask/src/main.rs:98 (struct Step)` ·
`xtask/src/main.rs:1209 (fn wasm_steps)` · `xtask/src/main.rs:1249 (fn steps_named)` ·
[CONTRIBUTING §The gate](../../CONTRIBUTING.md)

---

## RS-80-2. A probe means *skip when the tool is absent*, never *ignore when it fails*.

**Why.** `probe: None` is mandatory; `probe: Some(cmd)` runs a whole command
line and, if it succeeds, treats the step exactly as mandatory. The probe carries
its own program rather than borrowing the step's, so the skip message can name
the command that answered, and `RUSTUP_AUTO_INSTALL=0` is what makes
`cargo +nightly --version` a question rather than a download.

**Do**

```rust
struct Step {
    args: &'static [&'static str],
    probe: Option<&'static [&'static str]>,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Ran,
    Skipped,
    Failed,
}

fn run(step: &Step, tool_installed: bool, step_succeeded: bool) -> Outcome {
    if step.probe.is_some() && !tool_installed {
        return Outcome::Skipped;
    }
    if step_succeeded { Outcome::Ran } else { Outcome::Failed }
}

fn main() {
    let deny = Step {
        args: &["deny", "check"],
        probe: Some(&["cargo", "deny", "--version"]),
    };

    assert_eq!(deny.args.first(), Some(&"deny"));
    assert_eq!(run(&deny, false, false), Outcome::Skipped);
    assert_eq!(run(&deny, true, true), Outcome::Ran);
    // The half that is easy to lose: it ran, and it found something.
    assert_eq!(run(&deny, true, false), Outcome::Failed);
}
```

**Not** — "optional" read as a property of the step rather than of the tool.

```rust
# #[derive(Debug, PartialEq)]
# enum Outcome { Ran, Skipped, Failed }
struct Step {
    optional: bool,
}

fn run(step: &Step, step_succeeded: bool) -> Outcome {
    if step_succeeded || step.optional {
        Outcome::Ran
    } else {
        Outcome::Failed
    }
}

fn main() {
    let deny = Step { optional: true };
    assert_eq!(
        run(&deny, false),
        Outcome::Ran,
        "cargo deny found an advisory and the gate went green"
    );
}
```

**Rejects.** A `cargo deny check` that reports a RUSTSEC advisory against a
transitive dependency and is written off as "optional, skip it". CI installs
every probed tool on every runner, so the step is not skipped there — it is
*failing* there, and a runner that swallows the failure turns the one check that
arrives without a commit to trigger it into decoration nobody reads.

**Evidence.** `xtask/src/main.rs:141 (probe is not forced to be an invocation of the thing it is probing)` ·
`xtask/src/main.rs:1306 (let Some(probe) = step.probe)` ·
`xtask/src/main.rs:1340 (fn is_available)`

---

## RS-80-3. rustdoc flags go in that step's own `env`, spelled `RUSTDOCFLAGS`.

**Why.** rustdoc does not read `RUSTFLAGS`, so CI's ambient `-D warnings` reaches
every rustc invocation in the gate and no rustdoc one. `env` is per step because
setting it in the process environment would leak into every other step, and the
obvious shell workaround — a `VAR=x cargo …` string through `sh` — is not
portable to the Windows this repository is developed on.

**Do**

```rust
struct Step {
    name: &'static str,
    args: &'static [&'static str],
    env: &'static [(&'static str, &'static str)],
}

const DOCS: Step = Step {
    name: "documentation",
    args: &["doc", "--locked", "--workspace", "--all-features", "--no-deps"],
    env: &[("RUSTDOCFLAGS", "-D warnings")],
};

fn main() {
    assert_eq!(DOCS.name, "documentation");
    assert_eq!(DOCS.args.first(), Some(&"doc"));
    assert!(
        DOCS.env
            .iter()
            .any(|(key, value)| *key == "RUSTDOCFLAGS" && value.contains("-D warnings")),
        "rustdoc reads this variable and no other"
    );
}
```

**Not**

```rust
# struct Step { name: &'static str, env: &'static [(&'static str, &'static str)] }
const DOCS: Step = Step {
    name: "documentation",
    env: &[("RUSTFLAGS", "-D warnings")],
};

fn main() {
    assert!(
        !DOCS.env.iter().any(|(key, _)| *key == "RUSTDOCFLAGS"),
        "the step denies no rustdoc lint at all"
    );
}
```

**Rejects.** The documentation step exactly as it shipped: it printed *generated
3 warnings* and exited 0 for as long as it ran. Every rustdoc lint in the
workspace — `missing_docs` renderings, `private_intra_doc_links`, the whole
rustdoc group — was unenforced while a step named "documentation" reported
success, which is worse than having no step, because the gate's summary said the
question had been asked.

**Evidence.** `xtask/src/main.rs:112 (is not portable to the Windows this repository is developed on)` · `xtask/src/main.rs:491 (because rustdoc does not read)` ·
`xtask/src/main.rs:490 (rather than the ambient)`

---

## RS-80-4. Every gate invocation that resolves dependencies carries `--locked`.

**Why.** A gate that silently updates `Cargo.lock` has tested a dependency graph
nobody committed. Write the exemption as its *reason* and never as a list of
names: `cargo fmt` resolves nothing, and both `cargo hack --no-dev-deps` steps
physically rewrite each manifest with its dev-dependencies removed, which changes
the graph and so must rewrite the lock file — those two flags are mutually
exclusive by construction. Written that way the predicate also finds the step no
reason covers: `cargo deny check` resolves the graph, accepts `--locked` as a
global option, and today carries neither the flag nor an argument for omitting
it.

**Do**

```rust
struct Step {
    name: &'static str,
    args: &'static [&'static str],
}

const STEPS: &[Step] = &[
    Step { name: "formatting", args: &["fmt", "--all", "--check"] },
    Step { name: "clippy", args: &["clippy", "--locked", "--workspace", "--all-targets"] },
    Step { name: "documentation", args: &["doc", "--locked", "--workspace", "--no-deps"] },
    Step {
        name: "feature powerset",
        args: &["hack", "check", "--feature-powerset", "--no-dev-deps"],
    },
    // `--locked` is a global option of `cargo deny`, so it goes before the
    // subcommand. The gate's step does not carry it yet.
    Step { name: "licences and advisories", args: &["deny", "--locked", "check"] },
];

fn main() {
    for step in STEPS {
        // The exemptions written as their reasons rather than as two names.
        let exempt = step.args.first() == Some(&"fmt") || step.args.contains(&"--no-dev-deps");
        assert!(
            exempt || step.args.contains(&"--locked"),
            "{} resolves dependencies without --locked",
            step.name
        );
    }
}
```

**Not**

```rust
# struct Step { name: &'static str, args: &'static [&'static str] }
const ADDED: Step = Step {
    name: "wasm32 build of the Neon adapter",
    args: &["check", "-p", "happenstance-neon", "--target", "wasm32-unknown-unknown"],
};

fn main() {
    assert!(
        !ADDED.args.contains(&"--locked"),
        "free to resolve, and to commit nothing about what it resolved"
    );
}
```

**Rejects.** A new adapter step copied from a sibling with the flag dropped. It
passes on a machine whose lock file cargo has just quietly bumped, the bump is
never committed because `cargo xtask ci` is run before `git add`, and the next
contributor's identical command resolves a different graph — so a break lands in
a run that changed no source at all and is attributed to the commit that happened
to be under it.

**Evidence.** `xtask/src/main.rs:147 (resolves no dependencies)` ·
`xtask/src/main.rs:903 (must rewrite the lock file)` ·
[cargo-hack README](https://raw.githubusercontent.com/taiki-e/cargo-hack/main/README.md) *(checked 2026-08-09, rustc 1.97.1)*

---

## RS-80-5. Spend the floor the MSRV bought; move the floor only in an ADR.

**Why.** `cargo hack --rust-version` does not compare manifests — it reads each
selected package's `rust-version`, strips the patch, and runs
`rustup run 1.<minor> cargo` (never `cargo +1.<minor>`, which trips a rustup bug)
— so it exercises whatever floor the manifest currently states, including one
just raised. `resolver = "3"` only *prefers* dependency versions
declaring a compatible `rust-version`, a dependency declaring none carries no
signal at all, and when nothing matches the resolver picks an incompatible
version rather than erroring. Neither says anything about whether this
workspace's own code compiles at the floor, which is why CI's `msrv` job also
runs a full `cargo test --all-features` at the pinned number.

**Do** — the floor is 1.97.1, so let-chains (stable in 1.88) are available, and
`clippy.toml`'s `msrv` key is what turned `collapsible_if` back on when it moved.

```rust
fn first_positive(values: &[u64]) -> Option<u64> {
    if let Some(first) = values.first()
        && *first > 0
    {
        return Some(*first);
    }
    None
}

fn main() {
    assert_eq!(first_positive(&[7, 9]), Some(7));
    assert_eq!(first_positive(&[0]), None);
    assert_eq!(first_positive(&[]), None);
}
```

**Not** — a bump taken in passing to reach one new API. Nothing objects: the
`msrv` job reads the floor out of the line that was just edited, then installs
that toolchain and agrees with itself.

```toml
[workspace.package]
rust-version = "1.99.0"
```

**Rejects.** A floor raised silently in a workspace where ADR-0004 still carries
its provisional marker until first publish. Every mechanical check agrees,
because every one of them derives the floor from the manifest; the pinned
toolchain is newer than both, so the local gate is green too. The first party to
disagree is a consumer on the old compiler, after publication, when the number
has stopped being a note and become a promise.

**Evidence.** `Cargo.toml:26 (rust-version = "1.97.1")` · `clippy.toml:1 (msrv = "1.97.1")` ·
`xtask/src/main.rs:1299 (A let-chain, and the first in the workspace)` ·
[ADR-0029](../../.kb/decisions/0029-msrv-raised-to-1-97-1.md) ·
[ADR-0004](../../.kb/decisions/0004-edition-and-msrv.md) ·
[cargo rust-version resolution](https://doc.rust-lang.org/cargo/reference/resolver.html) *(checked 2026-08-09, rustc 1.97.1)*
