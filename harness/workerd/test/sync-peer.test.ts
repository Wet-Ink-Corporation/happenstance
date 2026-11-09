// One `it()` per sync rule (#130), each against its own Durable Object: the
// object as a remote peer, every pull and push a real request to the handler
// it mounts at `/sync/pull` and `/sync/push`, served over
// `CloudflareIngestStore` built by `namespaced_from_object_state` from the
// object's real state.
//
// The names come from the harness itself (`/sync-peer-rules`), which expands
// the sync testkit's enumeration, so this file names no rule and cannot drop
// one. Every rule must answer `ran`: under `PeerLimits::minimum` this peer
// shows a page boundary and confirms what it acknowledges, so a skip is a
// regression and none is tolerated. A violation answers 422 with its clause,
// never a trap.
import { env } from "cloudflare:workers";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

const listing = await object("sync-peer-rules").fetch("https://harness/sync-peer-rules");
const rules = (await listing.text()).split("\n").filter((name) => name.length > 0);

describe("sync peer conformance under workerd", () => {
  it("enumerates the family", () => {
    // Ten rules today; a floor, so the commit that adds a rule stays green.
    expect(rules.length).toBeGreaterThanOrEqual(10);
    expect(rules).toContain("round_trip_preserves_the_whole_log");
  });

  for (const rule of rules) {
    it(rule, async () => {
      const response = await object(`sync-peer-${rule}`).fetch(
        `https://harness/sync-peer-rule/${rule}`,
      );
      const body = await response.text();
      expect(response.status, body).toBe(200);
      expect(body, "every sync rule runs; none is skipped").toBe("ran");
    });
  }

  it("names a rule it does not know", async () => {
    const response = await object("sync-peer-unknown").fetch(
      "https://harness/sync-peer-rule/no_such_rule",
    );
    expect(response.status).toBe(404);
    expect(await response.text()).toBe("no such rule");
  });

  // The mounted routes, as an application mounts them: a GET is refused with
  // 405, an unknown sync path is 404, and an empty POST is malformed (400,
  // `Stop`), never a 500 a client would retry. The round trip over a real
  // socket is `tests/sqlite_round_trip.rs`.
  it("refuses what is not a sync request", async () => {
    const stub = object("sync-peer-routes");
    const get = await stub.fetch("https://harness/sync/pull");
    expect(get.status).toBe(405);
    expect(get.headers.get("allow")).toBe("POST");
    const unknown = await stub.fetch("https://harness/sync/other", { method: "POST" });
    expect(unknown.status).toBe(404);
    const empty = await stub.fetch("https://harness/sync/push", { method: "POST" });
    expect(empty.status, await empty.text()).toBe(400);
    const reset = await stub.fetch("https://harness/sync-reset");
    expect(reset.status).toBe(200);
  });
});
