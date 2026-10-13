// The deployed leg's hibernation cells: each cross-restart rule against
// `happenstance-cloudflare` on a deployed Durable Object, with a hibernatable
// WebSocket held open over `wss://` and the object left alone until
// Cloudflare's own inactivity timer hibernates it. Alongside them, the idle
// cells (#262 item 2, test/idle.test.ts): an object holding the adapter, with
// and without a partly read stream, and the timer control, each left alone and
// then asked which instance answers.
//
//   node deployed-restart.mjs <base-url> <run-id>      (HARNESS_TOKEN in the environment)
//
// RECORD-ONLY. It prints, per rule, whether the object restarted, after how
// long, whether the wasm instance changed, and the after-half's verdict; per
// idle cell, whether the object was evicted and the two witnesses; and it
// exits 0 whatever it saw. For the idle cells this is decision D4 of #262's
// plan: nothing pinned here states when production evicts an object that holds
// no hibernatable socket, so "not evicted at 12 s" would be the platform's
// policy, not the adapter keeping the object alive. Local `workerd` already produces a real hibernation
// and runs these cells strictly (test/restart.test.ts); Cloudflare's timing is
// not documented by anything this repository pins, and a red here would be a
// timing question in the merge queue rather than an answer about the adapter.
// A usage error is the only non-zero exit.
//
// The witness is the object's own: the wake reply names the instance that
// answered, and an after-half asked to expect a restart refuses (409) if the
// before-half's instance answered it. So "restarted" below is never inferred
// from the wait.
import { callPastPropagation, failureLine } from "./platform-miss.mjs";

const [base, runId] = process.argv.slice(2);
const token = process.env.HARNESS_TOKEN;
if (!base || !runId || !token) {
  console.error("usage: HARNESS_TOKEN=... node deployed-restart.mjs <base-url> <run-id>");
  process.exit(2);
}

// How long a cell leaves the object alone before waking it, and the one longer
// wait it tries when the first did not see a restart.
const WAITS_MS = [Number(process.env.HIBERNATE_WAIT_MS ?? 20_000), 60_000];
const REPLY_MS = 15_000;
const authorization = `Bearer ${token}`;
const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function call(object, route, init = {}) {
  const response = await fetch(`${base}/do/${object}/${route}`, {
    ...init,
    headers: { authorization },
  });
  return { status: response.status, body: await response.text(), headers: response.headers };
}

// Resolves with the next message on `socket`, or rejects after `ms`.
function next(socket, ms) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`no message within ${ms} ms`)), ms);
    socket.addEventListener(
      "message",
      (event) => {
        clearTimeout(timer);
        resolve(String(event.data));
      },
      { once: true },
    );
  });
}

function opened(socket, ms) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`the socket did not open within ${ms} ms`)), ms);
    socket.addEventListener("open", () => {
      clearTimeout(timer);
      resolve();
    }, { once: true });
    socket.addEventListener("error", () => {
      clearTimeout(timer);
      reject(new Error("the socket failed to open"));
    }, { once: true });
  });
}

const field = (text, key) => new RegExp(`${key}=([0-9a-f]{16})`).exec(text)?.[1] ?? "?";

async function cell(rule) {
  const object = `${runId}-hibernated-${rule}`;
  // Node's WebSocket (undici) sends `headers` on the upgrade; the router
  // refuses an upgrade without the token like any other request.
  const socket = new WebSocket(`${base.replace(/^http/, "ws")}/do/${object}/restart/socket`, {
    headers: { authorization },
  });
  try {
    await opened(socket, REPLY_MS);
    const before = await call(object, `restart/cloudflare/${rule}/before`, { method: "POST" });
    if (before.status !== 200) {
      return { rule, note: `before ${failureLine(before.status, before.body)}` };
    }
    const instance = field(before.body, "incarnation");
    const wasm = before.headers.get("x-wasm") ?? "?";
    let waited = 0;
    let woken = { instance, wasm };
    for (const wait of WAITS_MS) {
      await pause(wait);
      waited += wait;
      const reply = next(socket, REPLY_MS);
      socket.send("wake");
      const text = await reply;
      woken = { instance: field(text, "instance"), wasm: field(text, "wasm") };
      if (woken.instance !== instance) {
        break;
      }
    }
    const restarted = woken.instance !== instance;
    const query = restarted ? "expect=restarted&socket=held" : "expect=same";
    const after = await call(object, `restart/cloudflare/${rule}/after?${query}`, {
      method: "POST",
      body: before.body,
    });
    return {
      rule,
      restarted,
      waited,
      wasmChanged: woken.wasm !== wasm,
      verdict: failureLine(after.status, after.body),
    };
  } catch (error) {
    return { rule, note: error instanceof Error ? error.message : String(error) };
  } finally {
    socket.close(1000, "cell done");
  }
}

// The idle cells, as test/idle.test.ts runs them locally: `prefix` is the
// route (`idle-control` adds the pending interval), `wait` how long the object
// is left alone between the arm and the release.
const IDLE_CELLS = [
  { name: "plain", prefix: "idle", stream: "none", wait: 12_000 },
  { name: "held-stream", prefix: "idle", stream: "held", wait: 12_000 },
  { name: "plain", prefix: "idle", stream: "none", wait: 8_000 },
  { name: "held-stream", prefix: "idle", stream: "held", wait: 8_000 },
  { name: "timer-control", prefix: "idle-control", stream: "held", wait: 12_000 },
];

async function idleCell({ name, prefix, stream, wait }) {
  const label = `${name}@${wait / 1000}s`;
  const object = `${runId}-idle-${name}-${wait}`;
  try {
    const armed = await callPastPropagation(call, object, `${prefix}/arm?stream=${stream}`, {
      pause,
      log: console.log,
    });
    if (armed.status !== 200) {
      return { label, note: `arm ${failureLine(armed.status, armed.body)}` };
    }
    // Nothing touches the object now: any request would reset its timer.
    await pause(wait);
    const released = await call(object, `${prefix}/release`, { method: "POST" });
    if (released.status !== 200) {
      return { label, note: `release ${failureLine(released.status, released.body)}` };
    }
    const evicted = field(released.body, "instance") !== field(armed.body, "instance");
    const wasmChanged = field(released.body, "wasm") !== field(armed.body, "wasm");
    return { label, evicted, wasmChanged, released: released.body };
  } catch (error) {
    return { label, note: error instanceof Error ? error.message : String(error) };
  }
}

function printIdle(results) {
  console.log("--- deployed idle objects (record-only) ---");
  for (const r of results) {
    if (r.note !== undefined) {
      console.log(`?    ${r.label}  could not run: ${r.note}`);
    } else {
      console.log(`${r.label.padEnd(20)} evicted=${r.evicted} wasm changed=${r.wasmChanged}  ${r.released}`);
    }
  }
}

// Started before the hibernation run so the waits overlap. idleCell never
// rejects, so the table is printed on every path, the catch included.
const idle = Promise.all(IDLE_CELLS.map(idleCell));
try {
  const pastPropagation = (object, route) => callPastPropagation(call, object, route, { pause, log: console.log });
  const listing = await pastPropagation(`${runId}-restart-registry`, "restart/registry");
  if (listing.status !== 200) {
    console.log(`::warning::the restart registry did not answer: ${failureLine(listing.status, listing.body)}`);
    printIdle(await idle);
    process.exit(0);
  }
  const rules = listing.body
    .split("\n")
    .filter((line) => line.startsWith("rule "))
    .map((line) => line.slice("rule ".length));
  console.log(`hibernation cells: ${rules.length}, first wait ${WAITS_MS[0]} ms, retry wait ${WAITS_MS[1]} ms`);

  const results = await Promise.all(rules.map(cell));
  console.log("--- deployed hibernation (record-only) ---");
  for (const r of results) {
    if (r.note !== undefined) {
      console.log(`?    ${r.rule}  could not run: ${r.note}`);
    } else {
      console.log(
        `${r.restarted ? "rest" : "same"} ${r.rule}  restarted=${r.restarted} after ${r.waited} ms; ` +
          `wasm changed=${r.wasmChanged}; after-half ${r.verdict}`,
      );
    }
  }
  const observed = results.filter((r) => r.restarted).length;
  const passed = results.filter((r) => r.restarted && r.verdict.startsWith("[200]")).length;
  console.log(`restart observed on ${observed} of ${rules.length}; passed across it: ${passed}`);
  if (observed < rules.length || passed < observed) {
    console.log("::warning::the deployed hibernation cells were not all observed and passed; record-only, see the table");
  }
  printIdle(await idle);
} catch (error) {
  console.log(`::warning::the deployed hibernation run did not complete: ${error instanceof Error ? error.message : error}`);
  printIdle(await idle);
}
