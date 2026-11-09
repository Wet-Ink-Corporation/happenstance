//! The backoff a run waits out between transient failures: seeded, jittered and
//! capped (#244, SY-36's "retry, with jittered and capped backoff").
//!
//! Pure arithmetic over a seed, so nothing here sleeps (TST-10).

use core::num::NonZeroU32;
use core::time::Duration;

use happenstance_sync::runner::{Backoff, BackoffError};

const BASE: Duration = Duration::from_millis(100);
const CAP: Duration = Duration::from_secs(30);

fn backoff(seed: u64, attempts: u32) -> Backoff {
    let attempts = NonZeroU32::new(attempts).expect("a test asks for at least one attempt");
    Backoff::new(BASE, CAP, seed)
        .expect("the base is below the cap")
        .with_attempts(attempts)
}

fn delays(backoff: &mut Backoff) -> Vec<Duration> {
    core::iter::from_fn(|| backoff.next_delay()).collect()
}

/// The ceiling the n-th delay is drawn under: the base doubled n times, capped.
fn ceiling(attempt: u32) -> Duration {
    BASE.saturating_mul(1_u32.checked_shl(attempt).unwrap_or(u32::MAX))
        .min(CAP)
}

/// Rejects a backoff with no jitter, which draws the same sequence from every
/// seed and so lines every device up behind one restarted hub, and one that
/// reads a clock or a thread RNG, which no seed reproduces.
#[test]
fn the_same_seed_repeats_and_a_different_seed_does_not() {
    let first = delays(&mut backoff(7, 12));
    let again = delays(&mut backoff(7, 12));
    let other = delays(&mut backoff(8, 12));

    assert_eq!(first, again, "a seed reproduces its sequence");
    assert_eq!(first.len(), 12);
    assert_ne!(first, other, "a second seed jitters differently");
}

/// Rejects a backoff that never doubles (a fixed retry interval), one whose
/// jitter can reach zero (full jitter: a retry storm at the first failure), and
/// one that is uncapped or overflows past many attempts.
#[test]
fn every_delay_lies_in_the_upper_half_of_its_ceiling_and_never_above_the_cap() {
    for seed in 0..64 {
        let drawn = delays(&mut backoff(seed, 200));

        for (attempt, delay) in (0_u32..).zip(&drawn) {
            let high = ceiling(attempt);
            assert!(
                *delay >= high / 2 && *delay <= high,
                "seed {seed}, attempt {attempt}: {delay:?} outside [{:?}, {high:?}]",
                high / 2
            );
            assert!(*delay <= CAP, "seed {seed}, attempt {attempt}: {delay:?}");
        }
        assert!(
            drawn.iter().skip(20).all(|delay| *delay >= CAP / 2),
            "seed {seed}: the ceiling reached the cap and stayed there"
        );
    }
}

/// Rejects a backoff with no attempt cap, which retries a dead peer for ever
/// (the one #244 audited), and one whose reset does not restart the
/// count or the doubling.
#[test]
fn it_gives_up_after_its_attempts_and_a_reset_starts_again() {
    let mut paced = backoff(3, 4);

    let spent = delays(&mut paced);
    assert_eq!(spent.len(), 4);
    assert_eq!(paced.next_delay(), None, "spent stays spent");

    paced.reset();
    let first = paced
        .next_delay()
        .expect("a reset backoff has attempts again");
    assert!(first <= BASE, "a reset restarts the doubling: {first:?}");
}

/// Rejects a constructor that accepts a base above its cap, which would make
/// the cap the floor and the base a lie.
#[test]
fn a_base_above_the_cap_is_refused() {
    let refused = Backoff::new(CAP, BASE, 0);

    assert!(
        matches!(refused, Err(BackoffError::BaseAboveCap { base, cap }) if base == CAP && cap == BASE),
        "{refused:?}"
    );
}
