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

mod restarts;

use core::cell::OnceCell;

use worker::{
    DurableObject, Env, Request, Response, Result, State, WebSocket, WebSocketIncomingMessage,
    WebSocketPair, durable_object, wasm_bindgen,
};

use crate::dispatch::{self, Dispatched};
use crate::incarnation::Incarnation;
use crate::probe;

/// One Durable Object per rule: the runner names each object after the rule it
/// runs, so every rule starts from empty storage.
#[durable_object]
#[derive(Debug)]
pub(crate) struct ConformanceObject {
    state: State,
    /// Minted in `new`, so a new construction answers with a new one.
    incarnation: Incarnation,
    /// The store a cross-restart rule runs against, opened by the first
    /// restart request this instance serves. A `OnceCell` because it is set
    /// once per instance and only read after, so a shared reference to it may
    /// live across an `await` without any borrow flag to hold.
    held: OnceCell<restarts::Held>,
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
        Self {
            state,
            incarnation: Incarnation::mint(),
            held: OnceCell::new(),
        }
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
                let mut response = answer(outcome)?;
                // The runner compares it with `/prime`'s to see the cold start.
                response
                    .headers_mut()
                    .set("x-incarnation", &self.incarnation.to_string())?;
                Ok(response)
            }
            (Some("prime"), None) => match dispatch::prime(&self.state) {
                Ok(()) => Response::ok(self.witness()),
                Err(fault) => Response::error(fault.to_string(), 500),
            },
            (Some("incarnation"), None) => Response::ok(self.witness()),
            (Some("restart"), Some("socket")) => self.accept_socket(),
            (Some("restart"), Some("registry")) => Response::ok(restarts::registry()),
            (Some("restart"), Some(route)) => match restarts::Route::parse(route) {
                Ok(route) => {
                    let instance = restarts::Instance {
                        state: &self.state,
                        incarnation: self.incarnation,
                        held: &self.held,
                    };
                    instance.run(route, req).await
                }
                Err(unknown) => Response::error(unknown, 404),
            },
            (Some("probe"), None) => {
                let sql = happenstance_cloudflare::SqlStorage::from_state(&self.state);
                let report = probe::report(&sql).await;
                self.state.storage().delete_all().await?;
                Response::ok(report)
            }
            _ => Response::error(format!("no route {path}"), 404),
        }
    }

    /// Any message wakes the object; the reply names the instance that woke.
    async fn websocket_message(
        &self,
        ws: WebSocket,
        _message: WebSocketIncomingMessage,
    ) -> Result<()> {
        ws.send_with_str(format!("awake {}", self.witness()))
    }

    async fn websocket_close(
        &self,
        _ws: WebSocket,
        _code: usize,
        _reason: String,
        _was_clean: bool,
    ) -> Result<()> {
        // The runner closes its end when a cell is done; the runtime finishes
        // the close handshake, and there is nothing of this object's to release.
        Ok(())
    }

    async fn websocket_error(&self, _ws: WebSocket, error: worker::Error) -> Result<()> {
        worker::console_error!("a held WebSocket failed: {error}");
        Ok(())
    }
}

impl ConformanceObject {
    /// `instance=<nonce> wasm=<nonce>`: which construction answered, and in
    /// which wasm instance.
    fn witness(&self) -> String {
        format!(
            "instance={} wasm={}",
            self.incarnation,
            Incarnation::of_wasm_instance()
        )
    }

    /// Accepts a hibernatable WebSocket: the runtime, not this instance, holds
    /// it, so the object may hibernate with it open and is woken by its next
    /// message.
    fn accept_socket(&self) -> Result<Response> {
        let pair = WebSocketPair::new()?;
        self.state.accept_web_socket(&pair.server);
        Response::from_websocket(pair.client)
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
