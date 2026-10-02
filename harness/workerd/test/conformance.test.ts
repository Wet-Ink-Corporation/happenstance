// One `it()` per conformance rule, each against its own Durable Object.
//
// The names come from the harness itself (`/rules`), which expands the
// testkit's enumeration — so this file names no rule, and cannot drop one.
import { env } from "cloudflare:workers";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

const listing = await object("rules").fetch("https://harness/rules");
const rules = (await listing.text()).split("\n").filter((name) => name.length > 0);

describe("event store conformance under workerd", () => {
  it("enumerates the suite", () => {
    expect(rules.length).toBeGreaterThan(0);
  });

  for (const rule of rules) {
    it(rule, async () => {
      const response = await object(rule).fetch(`https://harness/rule/${rule}`);
      const body = await response.text();
      if (body.startsWith("skipped\n")) {
        console.log(body.slice("skipped\n".length));
      }
      expect(response.status, body).toBe(200);
    });
  }
});
