// The deployed leg's hibernation cells: each cross-restart rule against
// `happenstance-cloudflare` on a deployed Durable Object, with a hibernatable
// WebSocket held open over `wss://` and the object left alone until
// Cloudflare's own inactivity timer hibernates it.
//
//   node deployed-restart.mjs <base-url> <run-id>      (HARNESS_TOKEN in the environment)
//
// RECORD-ONLY. It prints, per rule, whether the object restarted, after how
// long, whether the wasm instance changed, and the after-half's verdict, and it
// exits 0 whatever it saw. Local `workerd` already produces a real hibernation
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

try {
  const pastPropagation = (object, route) => callPastPropagation(call, object, route, { pause, log: console.log });
  const listing = await pastPropagation(`${runId}-restart-registry`, "restart/registry");
  if (listing.status !== 200) {
    console.log(`::warning::the restart registry did not answer: ${failureLine(listing.status, listing.body)}`);
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
} catch (error) {
  console.log(`::warning::the deployed hibernation run did not complete: ${error instanceof Error ? error.message : error}`);
}
