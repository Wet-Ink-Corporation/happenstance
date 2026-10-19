// The `transactionSync` properties the ingest store behind `cfg(test)` rests
// on (#108, part F), measured on the runtime rather than assumed.
//
// `crates/happenstance-cloudflare/src/event_store/ingest.rs` runs each ingest
// call inside one `ctx.storage.transactionSync`, and the gate checks that store
// against a `node:sqlite` model of the primitive (`ingest/model.rs`): SAVEPOINT,
// then RELEASE on a return and ROLLBACK TO on a throw. Each case below is one
// thing that model assumes. A case failing here means the model is wrong about
// workerd, and the store's gate run proves nothing about the platform.
// The cases are JavaScript routes in worker.mjs (`/transaction-probe/<case>`),
// because the `worker` crate binds no `transactionSync`.
import { env } from "cloudflare:workers";
import { describe, expect, it } from "vitest";

// Each case on its own object, so no case sees another's tables.
async function probe(name: string) {
  const stub = env.CONFORMANCE.get(env.CONFORMANCE.idFromName(`transaction-probe-${name}`));
  const response = await stub.fetch(`https://harness/transaction-probe/${name}`);
  const body = await response.text();
  expect(response.status, body).toBe(200);
  return JSON.parse(body);
}

describe("transactionSync under local workerd", () => {
  it("is a function on the object's storage", async () => {
    const answer = await probe("is-a-function");
    expect(answer.type).toBe("function");
  });

  it("rolls back every row and the AUTOINCREMENT counter on a throw, and rethrows the same value", async () => {
    const { before, after, rethrownSame } = await probe("throw-rolls-back");
    // The control: the row written before the call is there, and it put a
    // counter in place, so an unchanged counter is a measurement.
    expect(before.rows).toBe(1);
    expect(before.sequence).not.toBeNull();
    expect(after).toEqual(before);
    expect(rethrownSame).toBe(true);
  });

  it("commits a callback that returns, and answers its value", async () => {
    const { before, after, returned } = await probe("return-commits");
    expect(after.rows).toBe(before.rows + 1);
    expect(returned).toBe(7);
  });

  it("answers no row from UPDATE OR IGNORE … RETURNING on a UNIQUE conflict", async () => {
    const { conflicting, untouched, control } = await probe("update-or-ignore-returning");
    expect(conflicting).toBe(0);
    expect(untouched).toBe("b");
    // The control: the same statement with no conflict answers its row.
    expect(control).toBe(1);
  });

  it("refuses transaction control through sql.exec", async () => {
    const { savepoint, begin } = await probe("savepoint-through-exec");
    expect(savepoint.refused, JSON.stringify(savepoint)).toBe(true);
    expect(begin.refused, JSON.stringify(begin)).toBe(true);
  });

  it("names a case it does not know", async () => {
    const stub = env.CONFORMANCE.get(env.CONFORMANCE.idFromName("transaction-probe-unknown"));
    const response = await stub.fetch("https://harness/transaction-probe/no-such-case");
    expect(response.status).toBe(404);
    expect(await response.text()).toContain("no-such-case");
  });
});
