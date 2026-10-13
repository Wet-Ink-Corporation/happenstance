// The Worker's entry point: `worker-build`'s Rust Durable Object, with each
// request raced against a Rust panic, behind a token-guarded router.
//
// Both halves are JavaScript on purpose. A failing conformance rule panics, and
// under `panic = "abort"` that traps the wasm instance inside a microtask: the
// promise the request awaits never settles, and anything else awaiting the same
// instance — an entry point written in Rust, for one — dies with it.
// `relay_panics` (src/object.rs) calls `globalThis.__harnessPanic` with the
// panic's message before the trap, so the object's request rejects at once with
// the assertion's own text, and this router, which owns no wasm, reports it.
// The shim reinitialises the instance on the next call.
import { ConformanceObject as RustObject } from "./build/index.js";

export class ConformanceObject {
  constructor(ctx, env) {
    this.inner = new RustObject(ctx, env);
  }

  fetch(request) {
    return relayed(() => this.inner.fetch(request));
  }

  // A hibernatable WebSocket's events arrive here, on whichever construction of
  // the object is current, including one built to receive this very message
  // after a hibernation.
  webSocketMessage(ws, message) {
    return relayed(() => this.inner.webSocketMessage(ws, message));
  }

  webSocketClose(ws, code, reason, wasClean) {
    return relayed(() => this.inner.webSocketClose(ws, code, reason, wasClean));
  }

  webSocketError(ws, error) {
    return relayed(() => this.inner.webSocketError(ws, error));
  }
}

// Races one call into the Rust object against the panic relay.
async function relayed(call) {
  const panicked = new Promise((_, reject) => {
    globalThis.__harnessPanic = (message) => reject(new Error(`Rust panic: ${message}`));
  });
  try {
    return await Promise.race([call(), panicked]);
  } finally {
    globalThis.__harnessPanic = undefined;
  }
}

// `/do/<object>/<route>`, forwarded to the object named `<object>`.
//
// Only the deployed leg comes through here; the local runner reaches the
// binding directly. It refuses everything unless `HARNESS_TOKEN` (a var or a secret) is
// set and the request carries it as a bearer token: an open harness would be a
// public endpoint for running arbitrary-width SQL against a billed account.
export default {
  async fetch(request, env) {
    if (!env.HARNESS_TOKEN) {
      return new Response("the harness token is not configured", { status: 403 });
    }
    if (request.headers.get("authorization") !== `Bearer ${env.HARNESS_TOKEN}`) {
      return new Response("forbidden", { status: 403 });
    }
    const match = /^\/do\/([^/]+)\/(.+)$/.exec(new URL(request.url).pathname);
    if (!match) {
      return new Response("expected /do/<object>/<route>", { status: 404 });
    }
    const [, object, route] = match;
    const stub = env.CONFORMANCE.get(env.CONFORMANCE.idFromName(object));
    // The original request, re-addressed: its method, body and `Upgrade`
    // header go through, and the object's answer comes back unchanged, so a
    // `101` carries its WebSocket. The query string goes with the route.
    const target = `https://harness/${route}${new URL(request.url).search}`;
    try {
      return await stub.fetch(new Request(target, request));
    } catch (error) {
      // The message only — it carries the panic's assertion text, which the
      // runner reports; the error itself would also carry its stack.
      const message = error instanceof Error ? error.message : "the object failed without an Error";
      return new Response(message, { status: 500 });
    }
  },
};
