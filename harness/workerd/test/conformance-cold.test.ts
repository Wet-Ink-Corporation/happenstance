// Every conformance rule on a cold-started object: `/prime` migrates every log
// the rule may open (minting each one's identity), the object is evicted, and
// the rule then runs in a new instance over logs a previous instance created.
//
// Weaker than the cross-restart rules (test/restart.test.ts): each rule still
// runs inside one instance. What it adds is the whole family against a store
// that opens existing state rather than creating it. The rule's response names
// the instance it ran in, and it must not be `/prime`'s.
import { env } from "cloudflare:workers";
import { evictDurableObject } from "cloudflare:test";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

const listing = await object("cold-rules").fetch("https://harness/rules");
const rules = (await listing.text()).split("\n").filter((name) => name.length > 0);

describe("event store conformance on a cold-started object under workerd", () => {
  it("enumerates the suite", () => {
    expect(rules.length).toBeGreaterThan(0);
  });

  for (const rule of rules) {
    it(rule, async () => {
      const stub = object(`cold-${rule}`);
      const primed = await stub.fetch("https://harness/prime");
      const primer = await primed.text();
      expect(primed.status, primer).toBe(200);
      const before = /instance=([0-9a-f]{16})/.exec(primer)?.[1];
      expect(before, primer).toMatch(/^[0-9a-f]{16}$/);

      await evictDurableObject(stub);

      const response = await stub.fetch(`https://harness/rule/${rule}`);
      const body = await response.text();
      expect(response.status, body).toBe(200);
      expect(response.headers.get("x-incarnation"), "the rule ran in a new instance").not.toBe(before);
      expect(response.headers.get("x-incarnation")).toMatch(/^[0-9a-f]{16}$/);
    });
  }
});
