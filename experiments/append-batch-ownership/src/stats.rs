//! Median and interquartile range, and the clock they are taken with.
//!
//! The quartiles are the nearest-rank kind on the sorted sample: no
//! interpolation, so every reported figure is a time that was actually observed.

use std::fmt;

/// The three quartiles of a sample of wall times, in microseconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    /// How many samples it was taken over.
    pub n: usize,
    /// First quartile.
    pub q1: f64,
    /// Median.
    pub median: f64,
    /// Third quartile.
    pub q3: f64,
}

impl Summary {
    /// Summarises `samples`, or `None` when there are none.
    #[must_use]
    pub fn of(samples: &[f64]) -> Option<Self> {
        let mut sorted = samples.to_vec();
        sorted.sort_by(f64::total_cmp);
        let at = |numerator: usize| -> Option<f64> {
            let last = sorted.len().checked_sub(1)?;
            sorted.get(last * numerator / 4).copied()
        };
        Some(Self {
            n: sorted.len(),
            q1: at(1)?,
            median: at(2)?,
            q3: at(3)?,
        })
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "median_us={:<11.1} q1_us={:<11.1} q3_us={:<11.1} n={}",
            self.median, self.q1, self.q3, self.n
        )
    }
}

/// A monotonic reading in microseconds.
///
/// `Instant` on the host. On wasm32 there is no `Instant`, so it is
/// `performance.now()` read through `js_sys::Reflect` — which Node provides as
/// a global — and `0.0` if a host ever lacks it, which turns every time into
/// zero rather than into a panic and is visible in every row.
#[must_use]
pub fn now_us() -> f64 {
    clock::now_us()
}

#[cfg(not(target_arch = "wasm32"))]
mod clock {
    use std::sync::LazyLock;
    use std::time::Instant;

    static ORIGIN: LazyLock<Instant> = LazyLock::new(Instant::now);

    pub(super) fn now_us() -> f64 {
        ORIGIN.elapsed().as_secs_f64() * 1e6
    }
}

#[cfg(target_arch = "wasm32")]
mod clock {
    use happenstance_cloudflare::worker::js_sys::{self, Function, Reflect};
    use happenstance_cloudflare::worker::wasm_bindgen::{JsCast, JsValue};

    pub(super) fn now_us() -> f64 {
        let global = js_sys::global();
        let Ok(performance) = Reflect::get(&global, &JsValue::from_str("performance")) else {
            return 0.0;
        };
        let Ok(now) = Reflect::get(&performance, &JsValue::from_str("now")) else {
            return 0.0;
        };
        now.dyn_ref::<Function>()
            .and_then(|now| now.call0(&performance).ok())
            .and_then(|millis| millis.as_f64())
            .map_or(0.0, |millis| millis * 1e3)
    }
}
