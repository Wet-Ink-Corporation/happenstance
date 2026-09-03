# The nested `block_on` hangs

`happenstance_testkit::block_on` (`crates/happenstance-testkit/src/registry.rs:330-344`)
loses a wakeup when a second `block_on` runs on the same OS thread while the
outer one is waiting. Reproduced deterministically, on every run.

Every figure is a row in [`raw/lost-wakeup.txt`](raw/lost-wakeup.txt).

## Why this probe is in a crate about busy timeouts

Because **from outside, a hang and a timeout exhaustion are the same event.**
Both end as a CI job that stopped, naming no rule. CF-33 is `[FROZEN]` and
forbids the conformance suite from carrying the watchdog that could tell them
apart (`crates/happenstance-testkit/src/concurrency.rs:36-44`), and
`crates/happenstance-sqlite/tests/concurrency.rs:41-49` restates the prohibition:
*"If a rule hangs, that is evidence about ADR-0022's busy-timeout paragraph."*

That sentence is only sound if the hang really was the busy timeout. This
directory measures the busy timeout, and a run in which it is not exhausted
([`busy-timeout-margin.md`](busy-timeout-margin.md)) leaves the other candidate
standing. Measuring only one of the two would let either finding be answered
with *"that was probably the other one"*.

## The mechanism

`block_on` is a park loop with no notified flag:

```text
Poll::Pending => std::thread::park(),
```

and `ParkWaker::wake` (`registry.rs:302-308`) is a bare `self.0.unpark()`.
`park`/`unpark` carries **one** token and it belongs to the *thread*, not to the
`block_on` call. So any park on that thread consumes it — including a park
belonging to a different, inner `block_on`.

The nesting is not invented for this file. The concurrency family does it:
`concurrency.rs:890` calls `incomplete_batches` on the rule's own thread,
`incomplete_batches` opens a second `crate::block_on` at `:980`, and the rule is
itself being driven by `crate::block_on` from `__emit_concurrency_blocking` at
`:1130`.

## The three cases

An outer future hands its waker to a helper thread, then runs a nested
`block_on` on the same thread. Only the *timing* of the outer wake differs
between cases 2 and 3.

| case | outer wake at | inner ready at | completes within 10 s |
| --- | --- | --- | --- |
| 1. no nesting (baseline) | 20 ms | — | **yes** |
| 2. nested, no collision | 200 ms | 20 ms | **yes** |
| 3. nested, wake lands during the inner park | 20 ms | 200 ms | **no — hung** |

Case 1 is what makes cases 2 and 3 evidence: if the baseline could not be woken
at all, nothing below would be about nesting. Case 2 is what stops the probe
being read as an indictment of nesting *as such* — the inner `block_on` returns
long before the outer waker fires, so there is no inner park for the outer's
token to land in. Case 3 arranges the collision and it hangs, every time.

## What the run recorded

```text
WAKEUP  case=plain                 completed=true
WAKEUP  case=nested-no-collision   completed=true
WAKEUP  case=nested-collision      completed=false  deadline_s=10  verdict=HUNG-past-deadline
```

Reproduced on four consecutive whole-directory runs while this experiment was
being built, with no observed variation.

## What this does **not** show

* **Not that any shipped rule hangs today.** It shows that the ordering which
  would hang is available on the thread `__emit_concurrency_blocking` drives, and
  that `concurrency.rs:890`/`:980` puts a nested `block_on` on exactly that
  thread. Whether a real adapter's read future is ever completed by a background
  thread at the moment the inner loop is parked is a property of the adapter, not
  of the testkit — `MemoryEventStore` completes on the calling thread and cannot
  produce it. A `spawn_blocking` adapter can.
* **Not a measurement of probability.** Case 3 forces the ordering with sleeps.
  It says the failure exists and is reachable; it says nothing about how often an
  unforced schedule produces it.
* **Nothing about the fix.** M-1's remediation (an `AtomicBool` notified flag
  demoting the park token to a hint) is not implemented or measured here — this
  directory may not touch `crates/`.
* **Case 3 asserts nothing**, deliberately. A failing assertion in an experiment
  reads as the experiment being broken; the row is the result.
