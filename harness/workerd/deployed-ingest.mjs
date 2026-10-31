// The deployed leg's ingest ceiling and failure census (#108 part E3).
//
//   node deployed-ingest.mjs <base-url> <run-id>      (HARNESS_TOKEN in the environment)
//
// RECORD-ONLY. It bisects each shape of ingest-ceiling.mjs, one object per
// attempt, runs each case of src/ingest/classes.rs, prints both, and writes
// them to deployed-ingest.txt for experiments/durable-object-limits/results/.
// The platform's memory and CPU walls are pinned by nothing here, which is the
// hibernation cells' reason for recording rather than asserting. It exits
// non-zero only when a route answers `no route`: then the deployed Worker is
// not this commit's, and the record would be about something else.
import { writeFileSync } from "node:fs";
import { SHAPES, bisectCeiling, ceilingLine, readAttempt } from "./ingest-ceiling.mjs";
import { callPastPropagation, failureLine } from "./platform-miss.mjs";

const CASES = ["trigger-abort", "missing-schema", "storage-full"];

const [base, runId] = process.argv.slice(2);
const token = process.env.HARNESS_TOKEN;
if (!base || !runId || !token) {
  console.error("usage: HARNESS_TOKEN=... node deployed-ingest.mjs <base-url> <run-id>");
  process.exit(2);
}

let stale = false;

async function call(object, route) {
  const response = await fetch(`${base}/do/${object}/${route}`, {
    headers: { authorization: `Bearer ${token}` },
  });
  const body = await response.text();
  if (response.status === 404 && body.startsWith("no route ")) {
    stale = true;
  }
  return { status: response.status, body };
}

const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

let objects = 0;
const lines = [];
for (const { shape, searchedTo } of SHAPES) {
  const wall = await bisectCeiling(async (events) => {
    objects += 1;
    const { status, body } = await call(
      `${runId}-ceiling-${shape}-${events}-${objects}`,
      `ingest-ceiling/${shape}?events=${events}`,
    );
    return readAttempt(status, body);
  }, searchedTo);
  lines.push(ceilingLine(shape, wall));
}
const ceiling = `--- deployed ingest ceiling ---\n${lines.join("\n")}`;
console.log(ceiling);

const census = ["--- deployed ingest classes ---"];
for (const name of CASES) {
  const { status, body } = await callPastPropagation(call, `${runId}-class-${name}`, `ingest-class/${name}`, {
    pause,
    log: console.log,
  });
  census.push(status === 200 ? `${name}:\n${body}` : `${name}: ${failureLine(status, body)}`);
}
console.log(census.join("\n"));

writeFileSync("deployed-ingest.txt", `${ceiling}\n${census.join("\n")}\n`);

if (stale) {
  console.error("a route answered `no route`: the deployed Worker is not this commit's");
  process.exit(1);
}
