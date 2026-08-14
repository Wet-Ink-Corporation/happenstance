# 60 — What a test must prove

> **Load when:** adding a conformance rule · writing a deliberately wrong store ·
> a meta-test says a mutant failed a rule it does not declare · `error[E0277]`
> naming `catch_unwind` · `error[E0038]` on a defect trait · a property test that
> has never failed
> **See also:** 01 (standard of evidence) · 61 (compile-time assertions) ·
> 62 (doctests and harnesses) · 81 (checks that cannot be types)

---

## RS-60-1. Open the subject inside the caught closure; never capture it.

**Why.** `catch_unwind`'s `UnwindSafe` bound constrains the closure's *captures*
— everything created inside is invisible to it. A non-capturing closure coerces
to `fn()`, and function pointers are unconditionally `UnwindSafe`, so the check
passes trivially and still means something.

**Do**

```rust
use std::panic::catch_unwind;

/// One rule, already bound to a store type by monomorphisation, so the fixture
/// is opened by the body rather than handed in.
fn run_probe(name: &str, run: fn() -> bool) -> String {
    match catch_unwind(run) {
        Ok(true) => format!("{name}: passed"),
        Ok(false) => format!("{name}: failed"),
        Err(_) => format!("{name}: panicked"),
    }
}

fn main() {
    let verdict = run_probe("reads_what_it_wrote", || {
        let store = std::rc::Rc::new(std::cell::RefCell::new(vec![1u8]));
        store.borrow().len() == 1
    });
    assert_eq!(verdict, "reads_what_it_wrote: passed");
}
```

**Not** — hoisting the fixture out so several probes can share it. The capture is
`&Rc<RefCell<_>>`: `error[E0277]`, *"may contain interior mutability and a
reference may not be safely transferable across a catch_unwind boundary"*. rustc
1.97.1 offers no fix — it traces `&Rc<RefCell<_>>: UnwindSafe` back to a bound in
`std::panic::catch_unwind` and stops — so `AssertUnwindSafe` is an escape hatch
the reader supplies unaided.

```rust,compile_fail,E0277
use std::cell::RefCell;
use std::panic::catch_unwind;
use std::rc::Rc;

fn main() {
    let store = Rc::new(RefCell::new(vec![1u8]));
    let _ = catch_unwind(|| store.borrow().len() == 1);
}
```

**Rejects.** The reviewer who reaches for `AssertUnwindSafe`, wraps the shared
fixture in it, and so asserts away the one check that made reuse safe: every
probe after the first panic then runs against a store the panic left half
mutated, so a mutant's verdicts become order-dependent and the registry's
exactness claim quietly stops being true. It is found — if ever — when reordering
`for_each_mutant!` changes which rules "fail".

**Evidence.** `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:454 (catch_unwind(probe.run))` ·
`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:330 (function pointers are unconditionally)` ·
[SPECIFICATION CF-3](../../spec/SPECIFICATION.md) ·
[ADR-0010](../../.kb/decisions/0010-the-suite-must-prove-itself.md)

---

## RS-60-2. Classify a caught panic by where it was raised, not by what it said.

**Why.** `catch_unwind` hands back the payload alone; a panic's location exists
only for the duration of the `set_hook` callback, so it must be stashed in a
thread-local and read back on the same thread. That location is a *positive*
claim — every rule assertion lives in one file — where a message check is a
denylist that accepts everything not on it.

**Do**

```rust
use std::cell::RefCell;
use std::panic::{self, catch_unwind};
use std::sync::Once;

thread_local! {
    static LAST_ORIGIN: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Installed once and never restored: `set_hook` is process-global and libtest
/// runs tests as concurrent threads, so a take/set bracket around each
/// `catch_unwind` races them. The previous hook is still called, so nothing an
/// unexpected panic would have printed is swallowed.
fn install_recording_hook() {
    static INSTALLED: Once = Once::new();
    INSTALLED.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            LAST_ORIGIN
                .with_borrow_mut(|slot| *slot = info.location().map(|at| at.file().to_owned()));
            previous(info);
        }));
    });
}

/// True only when the panic came out of the file every rule's assertions live
/// in. A store that falls over reports its own file — including through
/// `unwrap` and indexing, which are `#[track_caller]` and so name the caller.
fn rejected_by_a_rule(rule: fn(), rule_file: &str) -> bool {
    install_recording_hook();
    let caught = catch_unwind(rule);
    caught.is_err()
        && LAST_ORIGIN
            .with_borrow_mut(Option::take)
            .is_some_and(|file| file == rule_file)
}

fn main() {
    assert!(rejected_by_a_rule(
        || panic!("positions must be strictly increasing"),
        file!()
    ));
    // Same message, raised somewhere else: the store fell over, and no rule
    // rejected anything.
    assert!(!rejected_by_a_rule(
        || panic!("positions must be strictly increasing"),
        "suite.rs"
    ));
}
```

**Not** — the message denylist as the *primary* check. It compiles, it is what
everyone writes first, and the closing assertion is what catches it.

```rust
/// Substrings of the standard library's own panic messages.
const RUNTIME_PANICS: &[&str] = &["index out of bounds", "already borrowed"];

fn looks_like_a_rejection(message: &str) -> bool {
    !RUNTIME_PANICS.iter().any(|needle| message.contains(needle))
}

fn main() {
    // The mutant's modelled defect has been deleted, so the *fixture contract's*
    // own unimplemented body panicked instead. The denylist waves it through and
    // the registry row still reads as proof.
    assert!(looks_like_a_rejection("`Fixture::reopen` is not implemented"));
}
```

**Rejects.** Exactly that, measured: deleting `LosingFixture`'s `reopen` override
removes its modelled defect outright, the panic then comes from
`Fixture::reopen`'s provided body in `contract.rs`, and under the substring
denylist all six meta-tests stayed green — leaving the entire durability axis
vacuous and reported as covered.

**Evidence.** `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:160 (fn is_a_rule_body)` ·
`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:222 (A denylist can never be the primary check)` ·
`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:296 (LAST_ORIGIN.with_borrow_mut)` ·
[SPECIFICATION CF-2](../../spec/SPECIFICATION.md) ·
[ADR-0010](../../.kb/decisions/0010-the-suite-must-prove-itself.md)

---

## RS-60-3. One defect is one overridden default method, selected by a `PhantomData` marker.

**Why.** A trait whose every method has a body delegating to the correct
implementation makes "wrong in exactly one step" the only thing an impl can say.
The steps take no `self` and the trait carries an associated `const`, so it is
not dyn-compatible — the marker has to be a type parameter, which is also what
keeps the store's fields ordinary.

**Do**

```rust
use core::marker::PhantomData;

mod correct {
    pub fn matching(events: &[u32], wanted: u32) -> Vec<u32> {
        events.iter().copied().filter(|e| *e == wanted).collect()
    }
    pub fn truncated(selected: Vec<u32>, limit: usize) -> Vec<u32> {
        selected.into_iter().take(limit).collect()
    }
}

/// Override nothing and this *is* the reference store; override one method and
/// the failure is attributable to that step.
trait Defect: 'static {
    /// The registry key, naming the store rather than the defect.
    const NAME: &'static str;

    fn matching(events: &[u32], wanted: u32) -> Vec<u32> {
        correct::matching(events, wanted)
    }
    fn truncated(selected: Vec<u32>, limit: usize) -> Vec<u32> {
        correct::truncated(selected, limit)
    }
}

/// `PhantomData<D>`, not a field: `D` is a marker that is never constructed, and
/// monomorphisation is what picks the step.
struct MutantStore<D: Defect> {
    events: Vec<u32>,
    defect: PhantomData<D>,
}

impl<D: Defect> MutantStore<D> {
    fn read(&self, wanted: u32, limit: usize) -> Vec<u32> {
        D::truncated(D::matching(&self.events, wanted), limit)
    }
}

/// `SELECT … LIMIT n` pushed into the scan, with the predicate applied to the
/// rows that come back. One override, and nothing else differs.
struct LimitBeforeFilterStore;
impl Defect for LimitBeforeFilterStore {
    const NAME: &'static str = "LimitBeforeFilterStore";
    fn matching(events: &[u32], wanted: u32) -> Vec<u32> {
        correct::matching(&events[..events.len().min(2)], wanted)
    }
}

fn main() {
    let store = MutantStore::<LimitBeforeFilterStore> {
        events: vec![7, 7, 9, 9],
        defect: PhantomData,
    };
    // `read_limit_applies_after_filtering` is the rule that rejects this, and
    // the only rule that does.
    assert_eq!(
        store.read(9, 2),
        Vec::<u32>::new(),
        "{} scans two rows and then filters",
        LimitBeforeFilterStore::NAME
    );
}
```

**Not** — a defect chosen at run time, so one store type can wear any of them.
`error[E0038]`: an associated `const` and receiverless associated functions are
not dispatchable.

```rust,compile_fail,E0038
# trait Defect: 'static {
#     const NAME: &'static str;
#     fn matching(events: &[u32], wanted: u32) -> Vec<u32> { let _ = wanted; events.to_vec() }
# }
# struct LimitBeforeFilterStore;
# impl Defect for LimitBeforeFilterStore { const NAME: &'static str = "LimitBeforeFilterStore"; }
fn main() {
    let _registry: Vec<Box<dyn Defect>> = vec![Box::new(LimitBeforeFilterStore)];
}
```

**Rejects.** Each mutant written longhand as a whole second store — the shape
everyone reaches for, and the shape the workspace's 66 mutants would each take.
It drifts from the
reference implementation, so a mutant silently acquires a second, undeclared
defect; `mutants_fail_exactly_their_declared_rules` then reports the *instrument*
as broken, the registry stops being a map from rules to bugs, and no one can tell
which store is lying.

**Evidence.** `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:110 (pub(crate) trait Defect)` ·
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:107 (Box<dyn Defect>)` ·
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:266 (defect: PhantomData)` ·
[SPECIFICATION CF-3](../../spec/SPECIFICATION.md) ·
[ADR-0010](../../.kb/decisions/0010-the-suite-must-prove-itself.md)

---

## RS-60-4. Spell the oracle in a direction that shares no subroutine with the implementation.

**Why.** An oracle that calls the implementation's own helper asserts
`f(x) == f(x)`: every case passes, `proptest` shrinks nothing, and the assertion
still type-checks — nothing in the type system distinguishes the two spellings,
so independence has to be written into the source. The available move is
inversion: `all` of `any` against a merge-scan, a linear scan against a
`binary_search`. Take the generators from
`happenstance_testkit::fixtures::strategies` rather than writing your own —
[CF-21](../../spec/SPECIFICATION.md) is why they are exported and why the
alphabet is the size it is.

**Do**

```rust
use proptest::prelude::*;

/// A canonical, sorted tag set — the shape an adapter uses as an index key.
#[derive(Debug)]
struct Tags(Vec<String>);

impl Tags {
    fn new(mut tags: Vec<String>) -> Self {
        tags.sort();
        tags.dedup();
        Self(tags)
    }
    /// The implementation: one merge-scan over two sorted slices, which is the
    /// kind of code that is wrong only at a boundary.
    fn contains_all(&self, needles: &Tags) -> bool {
        let mut held = self.0.iter();
        needles
            .0
            .iter()
            .all(|wanted| held.any(|have| have == wanted))
    }
}

/// A stand-in for the exported strategy, with the exported alphabet.
fn any_tag() -> impl Strategy<Value = String> {
    prop::sample::select(vec!["a", "b", "c", "d", "e"]).prop_map(str::to_owned)
}

fn any_tags() -> impl Strategy<Value = Tags> {
    prop::collection::vec(any_tag(), 0..6).prop_map(Tags::new)
}

fn main() {
    proptest!(|(haystack in any_tags(), needles in any_tags())| {
        // The oracle, spelled in the inverted direction so that it shares no
        // subroutine with the merge-scan: `all` of `any`, not one pass.
        let naive = needles
            .0
            .iter()
            .all(|wanted| haystack.0.iter().any(|have| have == wanted));
        prop_assert_eq!(haystack.contains_all(&needles), naive);
    });
}
```

**Not** — the oracle routed through the implementation. It compiles, it passes
256 cases, and the two assertions below it are what expose the store it certified.

```rust
struct Tags(Vec<String>);

impl Tags {
    /// Unsorted, because the constructor forgot to sort — the defect.
    fn new(tags: Vec<String>) -> Self {
        Self(tags)
    }
    fn contains(&self, tag: &str) -> bool {
        self.0.binary_search_by(|have| have.as_str().cmp(tag)).is_ok()
    }
    fn contains_all(&self, needles: &Tags) -> bool {
        needles.0.iter().all(|wanted| self.contains(wanted))
    }
}

fn main() {
    let held = Tags::new(vec!["c".to_owned(), "b".to_owned(), "a".to_owned()]);
    let wanted = Tags::new(vec!["c".to_owned()]);

    // An oracle that calls `contains` agrees with `contains_all` on every input,
    // because `contains_all` *is* `contains`. The property is green:
    let oracle = wanted.0.iter().all(|tag| held.contains(tag));
    assert_eq!(held.contains_all(&wanted), oracle);

    // And the store is wrong. One hand-written line says so.
    assert!(!held.contains_all(&wanted), "\"c\" is held, and binary_search missed it");
}
```

**Rejects.** Exactly the `Tags` in the second fence: a constructor that stopped
sorting while `contains` stayed a `binary_search`, under a property whose oracle
calls `contains`. The two agree on every generated input because they are one
expression, so the property is green, shrinks nothing, and certifies the store
that drops rows for a tag it holds. The author reads a passing proptest as
coverage of the merge-scan; the defect is found by whoever reads the projection's
output, and the failing input was in the sample all along.

**Evidence.** `crates/happenstance-testkit/tests/properties.rs:56 (must not share a subroutine)` ·
`crates/happenstance-testkit/tests/properties.rs:62 (Which level is independently written)` ·
`crates/happenstance-testkit/src/fixtures.rs:521 (pub fn any_tag)` ·
`crates/happenstance-testkit/src/fixtures.rs:472 (pub use proptest)` ·
[SPECIFICATION CF-21](../../spec/SPECIFICATION.md)
