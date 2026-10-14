// The deployed leg's bookmark-restore cells for
// `CloudflareEventStore::remint_identity` (#298, src/remint.rs): one cell that
// re-mints after the restore, and the control that does not.
//
//   node deployed-remint.mjs <base-url> <run-id>      (HARNESS_TOKEN in the environment)
//
// Per cell, on its own object: append A1 and A2, take a bookmark, append B1
// and B2 and keep the object's ledger of all four, restore the bookmark (the
// object restarts), then on the new instance check that exactly A1 and A2
// remain, re-mint or not, append C1 to C3, and judge. The re-mint cell passes
// when no `EventId` names a different event than it did before the restore
// and a lost position was handed out again; the control passes when an
// `EventId` IS reissued, which is the hazard the re-mint exists for.
//
// Deployed only: local workerd does not implement a restore
// (test/remint.test.ts pins that). ASSERTED WHEN THE RESTORE RUNS, RECORDED
// WHEN IT CANNOT. A cell whose after-half answers 422 ran, observed the
// restore, and failed the judge, and the script exits 1. That is a
// `remint_identity` regression, a control that no longer shows the hazard, or
// a remint cell with reused-positions=0: the restore did not roll the
// position counter back, so the instrument is blind and cannot tell a re-mint
// from its absence. Anything
// else short of a pass (a cell that could not run, a restore that did not
// land, a restart that never came) is printed with a warning and exits 0,
// like deployed-restart.mjs, because nothing pinned here documents how fast
// a restore lands on Cloudflare. A usage error exits 2.
import { callPastPropagation, failureLine } from "./platform-miss.mjs";

const [base, runId] = process.argv.slice(2);
const token = process.env.HARNESS_TOKEN;
if (!base || !runId || !token) {
  console.error("usage: HARNESS_TOKEN=... node deployed-remint.mjs <base-url> <run-id>");
  process.exit(2);
}

const authorization = `Bearer ${token}`;
const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// The text `ctx.abort` is given in worker.mjs: a restore's request fails with
// it, and any other failure there is a fault.
const RESTORING = "restoring a bookmark";

// How often, and how far apart, an after-half is retried while the object
// still answers from the instance that wrote the ledger.
const AFTER_ATTEMPTS = 6;
const AFTER_DELAY_MS = 5_000;

async function call(object, route, init = {}) {
  const response = await fetch(`${base}/do/${object}/${route}`, {
    method: "POST",
    ...init,
    headers: { authorization },
  });
  return { status: response.status, body: await response.text() };
}

async function cell(mode) {
  const object = `${runId}-remint-${mode}`;
  try {
    const seeded = await callPastPropagation((o, r) => call(o, r), object, "remint/seed", {
      pause,
      log: console.log,
    });
    if (seeded.status !== 200) {
      return { mode, note: `seed ${failureLine(seeded.status, seeded.body)}` };
    }
    const bookmark = await call(object, "bookmark/current");
    if (bookmark.status !== 200) {
      return { mode, note: `bookmark ${failureLine(bookmark.status, bookmark.body)}` };
    }
    const lost = await call(object, "remint/lose");
    if (lost.status !== 200) {
      return { mode, note: `lose ${failureLine(lost.status, lost.body)}` };
    }
    const restore = await call(object, `bookmark/restore?bookmark=${encodeURIComponent(bookmark.body)}`);
    if (!restore.body.includes(RESTORING)) {
      return { mode, note: `restore ${failureLine(restore.status, restore.body)}` };
    }
    let after;
    for (let attempt = 1; attempt <= AFTER_ATTEMPTS; attempt += 1) {
      after = await call(object, `remint/after?mode=${mode}`, { body: lost.body });
      if (!(after.status === 409 && after.body.includes("no restart"))) {
        break;
      }
      console.log(`retry ${mode} after-half (attempt ${attempt + 1}): the writing instance answered`);
      await pause(AFTER_DELAY_MS);
    }
    // The whole body rather than its first line: the lines after it name
    // each reissued identity, which is the finding when a cell fails.
    const verdict = after.body.trimStart().startsWith("<")
      ? failureLine(after.status, after.body)
      : `[${after.status}] ${after.body.trim().split("\n").join("; ")}`;
    return { mode, bookmark: bookmark.body, verdict, passed: after.status === 200, judgedFailed: after.status === 422 };
  } catch (error) {
    return { mode, note: error instanceof Error ? error.message : String(error) };
  }
}

const results = await Promise.all(["remint", "control"].map(cell));
console.log("--- deployed re-mint across a bookmark restore (asserted when the restore runs) ---");
for (const r of results) {
  if (r.note !== undefined) {
    console.log(`?    ${r.mode}  could not run: ${r.note}`);
  } else {
    console.log(`${r.passed ? "pass" : "fail"} ${r.mode}  bookmark ${r.bookmark}; after-half ${r.verdict}`);
  }
}
const judgedFailed = results.filter((r) => r.judgedFailed === true).map((r) => r.mode);
if (judgedFailed.length > 0) {
  console.log(`::error::the restore ran and the judge failed: ${judgedFailed.join(", ")}; a regression, or reused-positions=0 (the restore kept the position counter, so the cell is blind); see the table`);
  process.exit(1);
}
if (!results.every((r) => r.passed === true)) {
  console.log("::warning::the deployed re-mint cells did not all run; recorded, not asserted, see the table");
}
