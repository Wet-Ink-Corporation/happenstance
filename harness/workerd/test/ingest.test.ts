// One `it()` per ingest rule (#108 part E3), each against its own Durable
// Object, through `CloudflareIngestStore::namespaced_from_object_state` over
// the object's real state and its real `ctx.storage.transactionSync`.
//
// The names come from the harness itself (`/ingest-rules`), which expands the
// sync testkit's enumeration, so this file names no rule and cannot drop one.
// Every rule must answer `ran`: the fixture supports MID_GROUP_FAULT and
// declares all three per-event ceilings, so no skip is expected and none is
// tolerated. A violation answers 422 with its clause, never a trap.
import { env } from "cloudflare:workers";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

const listing = await object("ingest-rules").fetch("https://harness/ingest-rules");
const rules = (await listing.text()).split("\n").filter((name) => name.length > 0);

describe("ingest conformance under workerd", () => {
  it("enumerates the family", () => {
    // Six rules and CF-18's declension check.
    expect(rules.length).toBeGreaterThanOrEqual(7);
    expect(rules).toContain("every_declined_capability_is_stated_by_this_fixture");
  });

  for (const rule of rules) {
    it(rule, async () => {
      const response = await object(`ingest-${rule}`).fetch(`https://harness/ingest-rule/${rule}`);
      const body = await response.text();
      expect(response.status, body).toBe(200);
      expect(body, "every ingest rule runs; none is skipped").toBe("ran");
    });
  }

  it("names a rule it does not know", async () => {
    const response = await object("ingest-unknown").fetch("https://harness/ingest-rule/no_such_rule");
    expect(response.status).toBe(404);
    expect(await response.text()).toBe("no such rule");
  });
});
