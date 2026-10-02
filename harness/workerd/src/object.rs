//! The Durable Object class.
//!
//! The Worker's entry point is `worker.mjs`, in JavaScript. A failing rule traps
//! the wasm instance, and any Rust awaiting that instance traps with it; an entry
//! point that owns no wasm outlives the trap and reports the object's error.
//!
//! The class is `pub(crate)` inside a private module, and that is load-bearing:
//! `#[durable_object]` generates `pub` methods with no documentation, and the
//! workspace denies `missing_docs`. An item unreachable
//! from the crate root is not public API, so the lint has nothing to say —
//! without an `#[allow]` and without touching the lint table. `wasm-bindgen`
//! exports the class to JavaScript whatever its Rust visibility.

use worker::{DurableObject, Env, Request, Response, Result, State, durable_object, wasm_bindgen};

use crate::dispatch::{self, Dispatched};
use crate::probe;

/// One Durable Object per rule: the runner names each object after the rule it
/// runs, so every rule starts from empty storage.
#[durable_object]
#[derive(Debug)]
pub(crate) struct ConformanceObject {
    state: State,
}

/// The global `worker.mjs` installs to hear about a panic.
const PANIC_RELAY: &str = "__harnessPanic";

/// Hands every panic's message to JavaScript before the instance traps.
///
/// A failing rule fails by panicking, as it does under every emitter. Under
/// `panic = "abort"` the panic traps the wasm instance inside a microtask, so the
/// promise the request is waiting on never settles: without this, a red rule
/// reads as a request timeout a minute later, with the assertion only in a log.
/// The hook runs *before* the trap, while JavaScript can still be called, and
/// `worker.mjs` races each request against the relay so the request rejects at
/// once with the assertion's own text.
///
/// Installed once per wasm instance. A trap discards the instance and the shim
/// builds a fresh one, whose first object installs it again.
fn relay_panics() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            previous(info);
            let relay = worker::js_sys::Reflect::get(
                &worker::js_sys::global(),
                &wasm_bindgen::JsValue::from_str(PANIC_RELAY),
            );
            if let Ok(relay) = relay
                && let Ok(relay) = wasm_bindgen::JsCast::dyn_into::<worker::js_sys::Function>(relay)
            {
                // A relay that throws leaves the request to time out, which is
                // where it would have been without one; there is nowhere else to
                // report it from inside a panic hook.
                if let Err(thrown) =
                    relay.call1(&wasm_bindgen::JsValue::NULL, &info.to_string().into())
                {
                    worker::console_error!("the panic relay threw: {thrown:?}");
                }
            }
        }));
    });
}

impl DurableObject for ConformanceObject {
    fn new(state: State, _env: Env) -> Self {
        relay_panics();
        Self { state }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        let path = req.path();
        let mut segments = path.trim_start_matches('/').splitn(2, '/');
        match (segments.next(), segments.next()) {
            (Some("rules"), None) => Response::ok(dispatch::rule_names().join("\n")),
            (Some("rule"), Some(name)) => {
                let outcome = dispatch::run(&self.state, name).await;
                // A deployed object is billed for what it stores; a rule's log
                // is worthless once the rule has answered.
                self.state.storage().delete_all().await?;
                answer(outcome)
            }
            (Some("probe"), None) => {
                let sql = happenstance_cloudflare::SqlStorage::from_state(&self.state);
                let report = probe::report(&sql).await;
                self.state.storage().delete_all().await?;
                Response::ok(report)
            }
            _ => Response::error(format!("no route {path}"), 404),
        }
    }
}

fn answer(outcome: Dispatched) -> Result<Response> {
    match outcome {
        Dispatched::Ran => Response::ok("ran"),
        Dispatched::Skipped(line) => Response::ok(format!("skipped\n{line}")),
        Dispatched::Unknown => Response::error("no such rule", 404),
        Dispatched::HarnessFault(faults) => Response::error(faults.join("\n"), 500),
    }
}
