// F's D7 failure classes, revisited on the runtime (#108 part E3,
// src/ingest/classes.rs). Each case provokes one ingest failure through the
// production store, retries the identical call on the same object, and the
// object judges the class by the retry: `Holds` when they agree.
//
// Strict on purpose (D-E3-1): a `Falsifies` or a `MarkerMissed` verdict fails
// here and is recorded, and the adapter's classes are changed, if at all, in
// a change of their own. `storage-full` may be `Unreachable`, because the
// object's authoriser decides whether `PRAGMA max_page_count` runs at all.
import { env } from "cloudflare:workers";
import { describe, expect, it } from "vitest";

async function census(name: string) {
  const stub = env.CONFORMANCE.get(env.CONFORMANCE.idFromName(`ingest-class-${name}`));
  const response = await stub.fetch(`https://harness/ingest-class/${name}`);
  const body = await response.text();
  console.log(`--- ingest class ${name} ---\n${body}`);
  return { status: response.status, body };
}

describe("the ingest failure classes under local workerd", () => {
  it("trigger-abort: a RAISE inside transactionSync is Stop, and recurs", async () => {
    const { status, body } = await census("trigger-abort");
    expect(status, body).toBe(200);
    expect(body.split("\n")[0], body).toBe("verdict=Holds");
    // The failure must be the throw the case is about, on both calls.
    expect(body, body).toContain("\nfirst=class=Stop variant=Sql(Thrown) ");
    expect(body, body).toContain("\nretry=class=Stop variant=Sql(Thrown) ");
  });

  it("missing-schema: no such table is Stop, and recurs", async () => {
    const { status, body } = await census("missing-schema");
    expect(status, body).toBe(200);
    expect(body.split("\n")[0], body).toBe("verdict=Holds");
    expect(body, body).toContain("\nfirst=class=Stop variant=Sql(Thrown) ");
    expect(body, body).toContain("\nretry=class=Stop variant=Sql(Thrown) ");
  });

  it("storage-full: a full object is StorageLimitExceeded and Stop, or cannot be staged", async () => {
    const { status, body } = await census("storage-full");
    expect(status, body).toBe(200);
    expect(body, body).toMatch(/^verdict=(Holds|Unreachable)( |\n|$)/);
  });

  it("names a case it does not know", async () => {
    const { status, body } = await census("no-such-case");
    expect(status).toBe(404);
    expect(body).toContain("no-such-case");
  });
});
