// An idle object holding a `CloudflareEventStore`, with and without a partly
// consumed read stream, is evicted by `workerd`'s own 10 s inactivity timer
// (#262 item 2, src/idle.rs). Every cell arms its own object, leaves it alone,
// and then asks which instance answers:
//
// * evicted: a store held alone, and a store with a stream opened and advanced
//   by one event, are both gone by 12 s. A new instance answers, and the
//   release finds nothing held.
// * not yet: the same two at 8 s. The instance that armed answers the release,
//   so it is the timer that evicts, not the arming or the wait itself. The
//   held stream then yields every event it had left, so what was parked was a
//   live stream and not a closed one.
// * the positive control: the held stream plus a pending JavaScript interval,
//   of the kind a write-behind store keeps for its flush (worker.mjs,
//   `/idle-control/`). It is NOT evicted at 12 s, so the measurement can tell
//   an object holding something pending from one holding only memory.
//
// Waiting is the point here (the platform's timer is what is measured), and the
// witness, not the wait, decides each outcome: nothing touches an object
// between its arm and its check, since a request would reset its timer.
// "Evicted" means the instance nonce changed and the wasm nonce did not; a
// changed wasm nonce is a trap, which is a harness fault, never an eviction.
import { env } from "cloudflare:workers";
import { afterAll, describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

type Stub = ReturnType<typeof object>;

// Past workerd's 10 s timer, with 2 s for the shutdown itself; and short of it.
const EVICTED_BY_MS = 12_000;
const NOT_YET_MS = 8_000;
// What `/idle/arm` appends (src/idle.rs `EVENTS`), as its answer states it.
const APPENDED = /appended=(\d+)/;

const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

// `instance=<nonce> wasm=<nonce>`, as every idle answer begins.
function witness(text: string) {
  const match = /instance=([0-9a-f]{16}) wasm=([0-9a-f]{16})/.exec(text);
  if (match === null) {
    throw new Error(`not a witness: ${text}`);
  }
  return { instance: match[1], wasm: match[2] };
}

async function call(stub: Stub, route: string) {
  const response = await stub.fetch(`https://harness/${route}`, { method: "POST" });
  const body = await response.text();
  expect(response.status, body).toBe(200);
  return body;
}

const table: string[] = [];

interface Cell {
  name: string;
  prefix: "idle" | "idle-control";
  stream: "none" | "held";
  waitMs: number;
  evicted: boolean;
}

const cells: Cell[] = [
  { name: "plain", prefix: "idle", stream: "none", waitMs: EVICTED_BY_MS, evicted: true },
  { name: "held-stream", prefix: "idle", stream: "held", waitMs: EVICTED_BY_MS, evicted: true },
  { name: "plain", prefix: "idle", stream: "none", waitMs: NOT_YET_MS, evicted: false },
  { name: "held-stream", prefix: "idle", stream: "held", waitMs: NOT_YET_MS, evicted: false },
  { name: "timer-control", prefix: "idle-control", stream: "held", waitMs: EVICTED_BY_MS, evicted: false },
];

describe.concurrent("an idle object and workerd's inactivity timer", () => {
  for (const cell of cells) {
    const label = `${cell.name}@${cell.waitMs / 1000}s`;
    it(`${label} ${cell.evicted ? "is evicted" : "is not evicted"}`, async () => {
      const stub = object(`idle-${cell.name}-${cell.waitMs}`);
      const armed = await call(stub, `${cell.prefix}/arm?stream=${cell.stream}`);
      const first = witness(armed);
      const appended = Number(APPENDED.exec(armed)?.[1]);
      expect(appended, armed).toBeGreaterThan(1);

      // Nothing touches the object now: any request would reset its timer.
      await pause(cell.waitMs);
      const released = await call(stub, `${cell.prefix}/release`);
      const now = witness(released);
      const evicted = now.instance !== first.instance;
      table.push(`${label.padEnd(20)} evicted=${evicted} wasm changed=${now.wasm !== first.wasm}  ${released}`);

      expect(now.wasm, "a changed wasm instance is a trap, not an eviction").toBe(first.wasm);
      expect(evicted, released).toBe(cell.evicted);
      if (cell.evicted) {
        // The instance that held the store is gone, and its hold with it.
        expect(released).toMatch(/ held=nothing$/);
      } else if (cell.stream === "held") {
        // The stream was live: it yields everything after the one event it
        // was advanced by.
        expect(released).toMatch(new RegExp(` held=held events=${appended - 1}$`));
      } else {
        expect(released).toMatch(new RegExp(` held=none events=${appended}$`));
      }
    }, 60_000);
  }
});

afterAll(() => {
  console.log(["--- idle objects under local workerd ---", ...table.sort()].join("\n"));
});
