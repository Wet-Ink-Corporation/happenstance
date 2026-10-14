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

// The idle measurement's positive control: `/idle-control/<route>` is
// `/idle/<route>` with a timer pending in between.
const CONTROL = "/idle-control/";

// The bookmark calls of the re-mint leg (#298, src/remint.rs), which the
// `worker` crate does not bind: `/bookmark/current` and
// `/bookmark/restore?bookmark=<b>`.
const BOOKMARK = "/bookmark/";

// The reason `ctx.abort` is given, and so the text a restore's request fails
// with: the runner tells this expected failure from any other by it.
const RESTORING = "restoring a bookmark";

// How a runtime with no point-in-time recovery refuses a restore (local
// `workerd` 1.20260815.1), and the fixed text the object answers in its place.
const NO_PITR = "does not implement point-in-time recovery";
const NO_PITR_ANSWER = "this runtime does not implement point-in-time recovery";

export class ConformanceObject {
  constructor(ctx, env) {
    this.ctx = ctx;
    this.inner = new RustObject(ctx, env);
  }

  fetch(request) {
    const url = new URL(request.url);
    if (url.pathname.startsWith(CONTROL)) {
      return this.control(url, request);
    }
    if (url.pathname.startsWith(BOOKMARK)) {
      return this.bookmark(url);
    }
    return relayed(() => this.inner.fetch(request));
  }

  // The positive control for the idle measurement (#262 item 2,
  // test/idle.test.ts): the same hold as `/idle/...`, plus a pending timer of
  // the kind a write-behind store would keep for its flush. An object with a
  // timer pending is not idle, so the platform must NOT evict it, which is
  // what shows that the measurement can tell a busy object from an idle one.
  // The interval is owned: set only once Rust has armed, cleared on release.
  async control(url, request) {
    const route = url.pathname.slice(CONTROL.length);
    if (route === "release") {
      clearInterval(this.flush);
      this.flush = undefined;
    }
    url.pathname = `/idle/${route}`;
    const response = await relayed(() => this.inner.fetch(new Request(url, request)));
    if (route === "arm" && response.status === 200 && this.flush === undefined) {
      this.flush = setInterval(() => {}, 1_000);
    }
    return response;
  }

  // The re-mint leg's bookmark calls. `current` answers the bookmark as text.
  // `restore` arranges for the next session to start from the bookmark and
  // then ends this one, because a restore applies only when the object
  // restarts: its request therefore fails, with RESTORING as the message.
  // A runtime that does not implement a restore throws instead, and that
  // throw is answered as a 501 with NO_PITR_ANSWER, which is what local
  // `workerd` does (test/remint.test.ts pins it). Any other throw is a 500.
  async bookmark(url) {
    const route = url.pathname.slice(BOOKMARK.length);
    try {
      if (route === "current") {
        return new Response(await this.ctx.storage.getCurrentBookmark());
      }
      if (route !== "restore") {
        return new Response(`no bookmark route ${route}`, { status: 404 });
      }
      const bookmark = url.searchParams.get("bookmark");
      if (!bookmark) {
        return new Response("bookmark= is required", { status: 400 });
      }
      await this.ctx.storage.onNextSessionRestoreBookmark(bookmark);
    } catch (error) {
      // The caller gets a fixed text, never the error itself: the detail goes
      // to the Worker's log. The one refusal the runners act on, a runtime
      // that implements no restore, is recognised here and named.
      console.error("a bookmark call threw", error);
      const message = error instanceof Error ? error.message : "";
      return message.includes(NO_PITR)
        ? new Response(NO_PITR_ANSWER, { status: 501 })
        : new Response("the bookmark call failed; the Worker's log has the error", { status: 500 });
    }
    this.ctx.abort(RESTORING);
    // Not reached: `abort` throws into this request.
    return new Response("the object did not restart", { status: 500 });
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

// Every call into the Rust object that is still in flight. A panic traps the
// one wasm instance they all share, so the relay rejects every one of them.
// A set rather than one slot: calls overlap (the hibernation cells run
// concurrently), and each call removes only its own entry when it settles, so
// one call finishing never disarms another's relay.
const inFlight = new Set();
globalThis.__harnessPanic = (message) => {
  for (const reject of inFlight) {
    reject(new Error(`Rust panic: ${message}`));
  }
};

// Races one call into the Rust object against the panic relay.
async function relayed(call) {
  let entry;
  const panicked = new Promise((_, reject) => {
    entry = reject;
  });
  inFlight.add(entry);
  try {
    return await Promise.race([call(), panicked]);
  } finally {
    inFlight.delete(entry);
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
