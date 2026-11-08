//! How long a run waits after a transient failure, and the seam it waits
//! through.
//!
//! # Seeded, so a test reproduces it
//!
//! The jitter comes from a seed the caller supplies, through a `SplitMix64`
//! generator written out below, rather than from a thread RNG or the clock. Two
//! reasons, one per environment. A test asserts the exact sequence a seed
//! draws, with no sleep and no flake (TST-10). And on wasm32 there is no thread
//! RNG to reach for without a dependency that knows the host; a Durable Object
//! seeds from whatever its runtime offers. A deployment seeds each device
//! differently, which is what spreads a fleet's retries after a hub restarts.
//!
//! # Equal jitter
//!
//! The n-th delay is drawn uniformly from `[c/2, c]`, where `c` is the base
//! doubled n times and capped. Full jitter, `[0, c]`, spreads a fleet slightly
//! better and can draw zero, which is a retry storm at the first failure; equal
//! jitter keeps a floor that still doubles.
//!
//! # Nothing sleeps here
//!
//! [`Sleep`] is the caller's: a tokio `sleep`, a Durable Object alarm, a test
//! that records the duration and returns at once. The runner holds no timer and
//! spawns nothing (ADR-0001: no runtime in the library).

use core::num::NonZeroU32;
use core::time::Duration;

/// Waits out a duration. The runner's only contact with time.
///
/// Defined without a `Send` bound, as the ports are, and `trait_variant`
/// derives [`SendSleep`], whose future is `Send`. Bind `Sleep` in generic code:
/// it is the weaker requirement and accepts both.
///
/// # Examples
///
/// A sleep that records what it was asked for and returns at once, as a test's
/// does:
///
/// ```
/// use core::cell::RefCell;
/// use core::time::Duration;
///
/// use happenstance_sync::runner::Sleep;
///
/// #[derive(Default)]
/// struct Recorded(RefCell<Vec<Duration>>);
///
/// impl Sleep for Recorded {
///     async fn sleep(&self, duration: Duration) {
///         self.0.borrow_mut().push(duration);
///     }
/// }
/// ```
#[trait_variant::make(SendSleep: Send)]
pub trait Sleep {
    /// Resolves once `duration` has passed, by the caller's clock.
    async fn sleep(&self, duration: Duration);
}

/// Why [`Backoff::new`] refused its arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BackoffError {
    /// The first delay would exceed the cap, so the cap would be a floor.
    #[error("the backoff's base {base:?} is above its cap {cap:?}")]
    BaseAboveCap {
        /// The base asked for.
        base: Duration,
        /// The cap asked for.
        cap: Duration,
    },
}

/// A seeded, jittered, capped exponential backoff, with a limit on attempts.
///
/// [`next_delay`](Self::next_delay) draws the next wait, or says the attempts
/// are spent; [`reset`](Self::reset) starts again after a success. The state is
/// the caller's to keep between runs, so a run that returns and is driven again
/// does not restart the doubling unless the caller says so.
///
/// # Examples
///
/// ```
/// use core::num::NonZeroU32;
/// use core::time::Duration;
///
/// use happenstance_sync::runner::Backoff;
///
/// let cap = Duration::from_secs(30);
/// let mut backoff = Backoff::new(Duration::from_millis(100), cap, 0x5EED)?
///     .with_attempts(NonZeroU32::MIN.saturating_add(2));
///
/// let drawn: Vec<Duration> = core::iter::from_fn(|| backoff.next_delay()).collect();
/// assert_eq!(drawn.len(), 3, "three attempts, then spent");
/// assert!(drawn.iter().all(|delay| *delay <= cap));
/// # Ok::<(), happenstance_sync::runner::BackoffError>(())
/// ```
#[derive(Debug, Clone)]
pub struct Backoff {
    base: Duration,
    cap: Duration,
    attempts: NonZeroU32,
    /// Delays drawn since the last reset.
    drawn: u32,
    /// `SplitMix64`'s state: the seed, advanced once per draw.
    state: u64,
}

impl Backoff {
    /// How many delays a backoff draws before it is spent, unless
    /// [`with_attempts`](Self::with_attempts) says otherwise. Ten doublings
    /// of a 100 ms base, capped at 30 s, wait about a minute in all.
    pub const DEFAULT_ATTEMPTS: NonZeroU32 = NonZeroU32::MIN.saturating_add(9);

    /// A backoff whose first delay is at most `base`, doubling to at most `cap`,
    /// jittered from `seed`, spent after [`DEFAULT_ATTEMPTS`](Self::DEFAULT_ATTEMPTS).
    ///
    /// # Errors
    ///
    /// [`BackoffError::BaseAboveCap`] if `base` exceeds `cap`.
    pub const fn new(base: Duration, cap: Duration, seed: u64) -> Result<Self, BackoffError> {
        if base.as_nanos() > cap.as_nanos() {
            return Err(BackoffError::BaseAboveCap { base, cap });
        }
        Ok(Self {
            base,
            cap,
            attempts: Self::DEFAULT_ATTEMPTS,
            drawn: 0,
            state: seed,
        })
    }

    /// The same backoff, spent after `attempts` delays.
    #[must_use]
    pub const fn with_attempts(mut self, attempts: NonZeroU32) -> Self {
        self.attempts = attempts;
        self
    }

    /// The next delay, or `None` once the attempts are spent.
    ///
    /// The n-th delay since the last reset (from zero) is drawn uniformly from
    /// `[c/2, c]` for `c = min(cap, base · 2ⁿ)`, so none exceeds the cap.
    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.drawn >= self.attempts.get() {
            return None;
        }
        let doubled = 1_u32.checked_shl(self.drawn).unwrap_or(u32::MAX);
        let ceiling = nanos(self.base.saturating_mul(doubled).min(self.cap));
        let floor = ceiling >> 1;
        let span = ceiling.saturating_sub(floor);
        // Lemire's multiply-shift: a draw in `[0, span]` without a division,
        // so no divisor can be zero. The product's high half is at most `span`.
        let wide = (u128::from(self.next_random()) * (u128::from(span) + 1)) >> 64;
        let jitter = u64::try_from(wide).map_or(span, |draw| draw.min(span));
        self.drawn = self.drawn.saturating_add(1);
        Some(Duration::from_nanos(floor.saturating_add(jitter)))
    }

    /// Starts the doubling and the attempt count again, after a success. The
    /// generator keeps its place, so the next sequence is not the last one.
    pub const fn reset(&mut self) {
        self.drawn = 0;
    }

    /// `SplitMix64` (Steele, Lea and Flood, 2014): one 64-bit state, advanced by a
    /// constant and mixed. Not for secrets; for spreading retries it is ample.
    const fn next_random(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut mixed = self.state;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        mixed ^ (mixed >> 31)
    }
}

/// A duration in nanoseconds, saturating past `u64::MAX` (about 584 years).
fn nanos(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}
