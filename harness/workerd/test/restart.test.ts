// The cross-restart rules (src/restart.rs), each with its two halves on either
// side of a real restart of the Durable Object.
//
// * evicted: `before`, then `evictDurableObject` from here, then `after`.
//   Every subject: the adapter must pass, and each wrong store must fail
//   exactly the (rule, check) pairs the registry lists.
// * control: the same cells with nothing between the halves. Every subject
//   passes every rule, so a wrong store's rejection above is evidence that the
//   rule ran across the restart and depended on it.
// * hibernated: the adapter only, with a hibernatable WebSocket held open and
//   the object left alone past `workerd`'s 10 s inactivity timer, then woken by
//   a message on that socket. Waiting is the point here, and the witness, not
//   the wait, decides the outcome: slack can slow a run, never green one.
//
// The object names its own incarnation, and every after-half checks it before
// the rule runs (409 if the restart did not happen). This file checks it again,
// and checks that the wasm instance did NOT change: locally, a restart tears
// down the object and keeps the module instance, so a changed wasm nonce means
// a trap, which is a harness fault, never a restart.
//
// The rule, subject and registry names come from `/restart/registry`.
import { env } from "cloudflare:workers";
import { evictDurableObject } from "cloudflare:test";
import { afterAll, describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

type Stub = ReturnType<typeof object>;

const listing = await (await object("restart-registry").fetch("https://harness/restart/registry")).text();
const rules: string[] = [];
const rejects = new Map<string, Map<string, string>>();
for (const line of listing.split("\n")) {
  const [kind, name, , ...pairs] = line.split(" ");
  if (kind === "rule") {
    rules.push(name);
  } else if (kind === "subject") {
    rejects.set(name, new Map(pairs.filter((p) => p.length > 0).map((p) => p.split(":") as [string, string])));
  }
}
const subjects = [...rejects.keys()];
const expectedRejections = [...rejects.values()].reduce((sum, row) => sum + row.size, 0);
let observedRejections = 0;

const HIBERNATE_WAIT_MS = 12_000;

// `instance=<nonce> wasm=<nonce>`, as `/incarnation` and a wake reply say it.
function witness(text: string) {
  const match = /instance=([0-9a-f]{16}) wasm=([0-9a-f]{16})/.exec(text);
  if (match === null) {
    throw new Error(`not a witness: ${text}`);
  }
  return { instance: match[1], wasm: match[2] };
}

async function incarnation(stub: Stub) {
  return witness(await (await stub.fetch("https://harness/incarnation")).text());
}

async function before(stub: Stub, subject: string, rule: string) {
  const response = await stub.fetch(`https://harness/restart/${subject}/${rule}/before`, { method: "POST" });
  const evidence = await response.text();
  expect(response.status, evidence).toBe(200);
  const instance = /^incarnation=([0-9a-f]{16})$/m.exec(evidence)?.[1];
  const wasm = response.headers.get("x-wasm");
  expect(instance, evidence).toMatch(/^[0-9a-f]{16}$/);
  expect(wasm).toMatch(/^[0-9a-f]{16}$/);
  return { evidence, instance, wasm };
}

async function after(stub: Stub, subject: string, rule: string, evidence: string, query: string) {
  const response = await stub.fetch(`https://harness/restart/${subject}/${rule}/after?${query}`, {
    method: "POST",
    body: evidence,
  });
  return { status: response.status, body: await response.text() };
}

describe("cross-restart rules, evicted between the halves", () => {
  it("lists the rules, the subjects and a wrong store for every rule", () => {
    expect(rules.length).toBeGreaterThan(0);
    expect(subjects).toContain("cloudflare");
    expect(rejects.get("cloudflare")?.size).toBe(0);
    for (const rule of rules) {
      expect([...rejects.values()].some((row) => row.has(rule)), rule).toBe(true);
    }
  });

  for (const subject of subjects) {
    for (const rule of rules) {
      it(`${subject} ${rule}`, async () => {
        const stub = object(`evicted-${subject}-${rule}`);
        const first = await before(stub, subject, rule);
        await evictDurableObject(stub);
        const now = await incarnation(stub);
        expect(now.instance, "the eviction made a new object instance").not.toBe(first.instance);
        expect(now.wasm, "a changed wasm instance is a trap, not an eviction").toBe(first.wasm);

        const { status, body } = await after(stub, subject, rule, first.evidence, "expect=restarted");
        const check = rejects.get(subject)?.get(rule);
        if (check === undefined) {
          expect(status, body).toBe(200);
        } else {
          expect(status, body).toBe(422);
          expect(body.split("\n")[0]).toBe(`REJECTED ${rule} ${check}`);
          observedRejections += 1;
        }
      });
    }
  }
});

describe("cross-restart rules, the control: nothing between the halves", () => {
  for (const subject of subjects) {
    for (const rule of rules) {
      it(`${subject} ${rule}`, async () => {
        const stub = object(`control-${subject}-${rule}`);
        const first = await before(stub, subject, rule);
        const { status, body } = await after(stub, subject, rule, first.evidence, "expect=same");
        expect(status, body).toBe(200);
      });
    }
  }

  it("an after-half that expects a restart that did not happen is refused", async () => {
    const [rule] = rules;
    const stub = object(`guard-${rule}`);
    const first = await before(stub, "cloudflare", rule);
    const { status, body } = await after(stub, "cloudflare", rule, first.evidence, "expect=restarted");
    expect(status, body).toBe(409);
    expect(body).toMatch(/^restart not observed/);
  });
});

// Wakes the object with a message on its hibernated socket, and answers the
// witness its reply carries. Bounded: no reply in 10 s is a failure.
function wake(socket: WebSocket): Promise<{ instance: string; wasm: string }> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("no reply to the wake within 10 s")), 10_000);
    socket.addEventListener("message", (event) => {
      clearTimeout(timer);
      try {
        resolve(witness(String(event.data)));
      } catch (error) {
        reject(error);
      }
    });
    socket.send("wake");
  });
}

describe.concurrent("cross-restart rules, hibernated and woken by the platform's timer", () => {
  for (const rule of rules) {
    it(`cloudflare ${rule}`, async () => {
      const stub = object(`hibernated-${rule}`);
      const upgrade = await stub.fetch("https://harness/restart/socket", { headers: { Upgrade: "websocket" } });
      expect(upgrade.status).toBe(101);
      const socket = upgrade.webSocket;
      if (socket === null) {
        throw new Error("the upgrade carried no WebSocket");
      }
      socket.accept();
      try {
        const first = await before(stub, "cloudflare", rule);
        // Nothing touches the object now: any request would reset its timer.
        await new Promise((resolve) => setTimeout(resolve, HIBERNATE_WAIT_MS));
        const woken = await wake(socket);
        expect(woken.instance, "the object hibernated and a new instance woke").not.toBe(first.instance);
        expect(woken.wasm, "a changed wasm instance is a trap, not a hibernation").toBe(first.wasm);

        const { status, body } = await after(stub, "cloudflare", rule, first.evidence, "expect=restarted&socket=held");
        expect(status, body).toBe(200);
      } finally {
        socket.close(1000, "cell done");
      }
    }, 60_000);
  }
});

afterAll(() => {
  // The CI step greps for this line with M = M.
  console.log(`mutant rejections observed: ${observedRejections} of ${expectedRejections}`);
});
