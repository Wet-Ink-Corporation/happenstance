// Every write-group rule on a cold-started object (#420):
// test/ingest-cold.test.ts, one family over. `/prime` migrates every log a
// rule may open, so the write-group columns are added by a previous instance;
// the object is evicted, and the rule then runs in a new instance, whose
// `migrate` must find both columns and add neither (`ALTER TABLE … ADD
// COLUMN` has no `IF NOT EXISTS`, so a second add would throw).
import { env } from "cloudflare:workers";
import { evictDurableObject } from "cloudflare:test";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

const listing = await object("cold-write-group-rules").fetch("https://harness/write-group-rules");
const rules = (await listing.text()).split("\n").filter((name) => name.length > 0);

describe("write-group conformance on a cold-started object under workerd", () => {
  it("enumerates the family", () => {
    expect(rules.length).toBeGreaterThanOrEqual(6);
  });

  for (const rule of rules) {
    it(rule, async () => {
      const stub = object(`cold-write-group-${rule}`);
      const primed = await stub.fetch("https://harness/prime");
      const primer = await primed.text();
      expect(primed.status, primer).toBe(200);
      const before = /instance=([0-9a-f]{16})/.exec(primer)?.[1];
      expect(before, primer).toMatch(/^[0-9a-f]{16}$/);

      await evictDurableObject(stub);

      const response = await stub.fetch(`https://harness/write-group-rule/${rule}`);
      const body = await response.text();
      expect(response.status, body).toBe(200);
      expect(body).toBe("ran");
      expect(response.headers.get("x-incarnation"), "the rule ran in a new instance").not.toBe(before);
      expect(response.headers.get("x-incarnation")).toMatch(/^[0-9a-f]{16}$/);
    });
  }
});
