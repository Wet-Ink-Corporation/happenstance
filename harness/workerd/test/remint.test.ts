// The re-mint leg (#298, src/remint.rs) under local workerd: what can run here
// without a restore, and the pin that says why the restore itself cannot.
//
// The leg restores a SQLite bookmark (`onNextSessionRestoreBookmark`), and the
// workerd that package-lock.json pins does not implement one: it answers
// `getCurrentBookmark` with an all-zero bookmark and refuses the restore. So
// the leg runs on the deployed object only (deployed-remint.mjs), and this
// file holds the two refusals that keep a run there honest: an after-half in
// the instance that wrote the ledger (no restart), and one across an eviction
// that rolled nothing back (no restore). The first test fails on a runtime
// bump that implements a restore, which is the moment to move the leg here.
import { env } from "cloudflare:workers";
import { evictDurableObject } from "cloudflare:test";
import { describe, expect, it } from "vitest";

function object(name: string) {
  return env.CONFORMANCE.get(env.CONFORMANCE.idFromName(name));
}

type Stub = ReturnType<typeof object>;

async function call(stub: Stub, route: string, body?: string) {
  const response = await stub.fetch(`https://harness/${route}`, { method: "POST", body });
  return { status: response.status, body: await response.text() };
}

// `seed` then `lose`: the four events, and the ledger the runner carries.
async function ledger(stub: Stub) {
  const seeded = await call(stub, "remint/seed");
  expect(seeded.status, seeded.body).toBe(200);
  const lost = await call(stub, "remint/lose");
  expect(lost.status, lost.body).toBe(200);
  expect(lost.body.match(/^event=/gm)?.length, lost.body).toBe(4);
  return lost.body;
}

describe("the re-mint leg under local workerd", () => {
  it("local workerd does not implement a bookmark restore", async () => {
    const stub = object("remint-local-restore");
    await ledger(stub);
    const current = await call(stub, "bookmark/current");
    expect(current.status, current.body).toBe(200);
    const restore = await call(stub, `bookmark/restore?bookmark=${encodeURIComponent(current.body)}`);
    expect(restore.status, restore.body).toBe(501);
    expect(restore.body).toContain("does not implement point-in-time recovery");
  });

  it("an after-half in the instance that wrote the ledger refuses", async () => {
    const stub = object("remint-local-no-restart");
    const evidence = await ledger(stub);
    const after = await call(stub, "remint/after?mode=remint", evidence);
    expect(after.status, after.body).toBe(409);
    expect(after.body).toContain("no restart");
  });

  for (const mode of ["remint", "control"]) {
    it(`a ${mode} after-half across a restart that restored nothing refuses`, async () => {
      const stub = object(`remint-local-no-restore-${mode}`);
      const evidence = await ledger(stub);
      await evictDurableObject(stub);
      const after = await call(stub, `remint/after?mode=${mode}`, evidence);
      expect(after.status, after.body).toBe(409);
      expect(after.body).toContain("the restore was not observed");
      expect(after.body).toContain("B1");
    });
  }

  it("an after-half names a mode it does not know", async () => {
    const stub = object("remint-local-bad-mode");
    const after = await call(stub, "remint/after?mode=both", "incarnation=0000000000000000\n");
    expect(after.status, after.body).toBe(400);
    expect(after.body).toContain('no mode "both"');
  });
});
