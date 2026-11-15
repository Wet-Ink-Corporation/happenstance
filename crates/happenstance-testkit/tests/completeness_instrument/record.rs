//! The suite half of the rule: every rule of every family, run against the
//! pruned fixture, and the outcome of each recorded in `pass_list.txt`.
//!
//! `run` is a function pointer, not a boxed closure: a non-capturing closure
//! coerces to `fn()`, which is unconditionally `UnwindSafe`, and the fixture is
//! opened inside it (RS-60-1). Nothing is asserted away to reach
//! `catch_unwind`.

use std::any::Any;
use std::panic;

use happenstance_testkit::RuleOutcome;

use super::fixture::PrunedMemoryFixture;

/// The opener every probe hands its rule.
async fn pruned_fixture() -> PrunedMemoryFixture {
    PrunedMemoryFixture::pruned().await
}

/// One rule, bound to the pruned fixture.
#[derive(Clone, Copy)]
struct Probe {
    family: &'static str,
    name: &'static str,
    run: fn() -> RuleOutcome,
}

/// What one rule did against the pruned fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Recorded {
    Ran,
    Skipped { capability: &'static str },
    Failed { message: String },
}

/// One line of the record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) family: &'static str,
    pub(crate) name: &'static str,
    pub(crate) recorded: Recorded,
}

fn event_store_probes() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    family: "event-store",
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::rules::$rule(pruned_fixture)
                    ),
                }
            ),* ]
        };
    }
    happenstance_testkit::for_each_event_store_rule!(probe)
}

fn concurrency_probes() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    family: "concurrency",
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::concurrency::rules::$rule(pruned_fixture)
                    ),
                }
            ),* ]
        };
    }
    happenstance_testkit::for_each_concurrency_rule!(probe)
}

#[cfg(feature = "proptest")]
fn model_probes() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    family: "model",
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::model::rules::$rule(pruned_fixture)
                    ),
                }
            ),* ]
        };
    }
    happenstance_testkit::for_each_model_rule!(probe)
}

/// Every family compiled into this build. The model family needs `proptest`.
fn probes() -> Vec<Probe> {
    let mut probes = event_store_probes();
    probes.extend(concurrency_probes());
    #[cfg(feature = "proptest")]
    probes.extend(model_probes());
    probes
}

/// Whether a family's lines are compiled into this build.
fn compiled(family: &str) -> bool {
    family != "model" || cfg!(feature = "proptest")
}

/// Runs every rule of every compiled family against the pruned fixture.
///
/// No quiet panic hook: a panic here is unexpected, and should print.
pub(crate) fn run_families() -> Vec<Entry> {
    probes()
        .into_iter()
        .map(|probe| Entry {
            family: probe.family,
            name: probe.name,
            recorded: match panic::catch_unwind(probe.run) {
                Ok(RuleOutcome::Ran) => Recorded::Ran,
                Ok(RuleOutcome::Skipped { capability, .. }) => Recorded::Skipped { capability },
                Err(payload) => Recorded::Failed {
                    message: panic_message(payload.as_ref()),
                },
            },
        })
        .collect()
}

fn panic_message(payload: &(dyn Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(ToString::to_string)
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "a panic with a non-string payload".to_owned())
}

/// The record, one line per rule in registry order. A failed rule renders as
/// `failed`, which no committed record carries.
pub(crate) fn render(entries: &[Entry]) -> Vec<String> {
    entries
        .iter()
        .map(|entry| match &entry.recorded {
            Recorded::Ran => format!("{} ran {}", entry.family, entry.name),
            Recorded::Skipped { capability } => {
                format!("{} skipped {} {capability}", entry.family, entry.name)
            }
            Recorded::Failed { .. } => format!("{} failed {}", entry.family, entry.name),
        })
        .collect()
}

/// The committed record's lines, without comments or blank lines, and only
/// for the families this build compiles.
pub(crate) fn expected(committed: &str) -> Vec<String> {
    committed
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter(|line| line.split_whitespace().next().is_some_and(compiled))
        .map(ToOwned::to_owned)
        .collect()
}

/// The committed file's header: every line up to the first record line.
pub(crate) fn header(committed: &str) -> String {
    committed
        .lines()
        .take_while(|line| line.is_empty() || line.starts_with('#'))
        .fold(String::new(), |mut header, line| {
            header.push_str(line);
            header.push('\n');
            header
        })
}
