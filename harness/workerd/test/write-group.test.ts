// One `it()` per write-group rule (#420, SY-39), each against its own Durable
// Object, through `CloudflareIngestStore::namespaced_from_object_state` over
// the object's real state: the ingest family's fixture, whose store also reads
// its log back in write groups.
//
// The names come from the harness itself (`/write-group-rules`), which expands
// the sync testkit's enumeration, so this file names no rule and cannot drop
// one. Every rule must answer `ran`: the family gates no rule on a capability,
// so no skip is possible and none is tolerated. A violation answers 422 with
// its clause, never a trap.
import { env } from "cloudflare:workers";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

const listing = await object("write-group-rules").fetch("https://harness/write-group-rules");
const rules = (await listing.text()).split("\n").filter((name) => name.length > 0);

describe("write-group conformance under workerd", () => {
  it("enumerates the family", () => {
    // Six rules today; a floor, so the commit that adds a rule stays green.
    expect(rules.length).toBeGreaterThanOrEqual(6);
    expect(rules).toContain("a_read_never_cuts_a_group");
  });

  for (const rule of rules) {
    it(rule, async () => {
      const response = await object(`write-group-${rule}`).fetch(
        `https://harness/write-group-rule/${rule}`,
      );
      const body = await response.text();
      expect(response.status, body).toBe(200);
      expect(body, "every write-group rule runs; none is skipped").toBe("ran");
    });
  }

  it("names a rule it does not know", async () => {
    const response = await object("write-group-unknown").fetch(
      "https://harness/write-group-rule/no_such_rule",
    );
    expect(response.status).toBe(404);
    expect(await response.text()).toBe("no such rule");
  });
});
