// The deployed leg: every conformance rule, then the wall probe, against the
// harness Worker deployed to a real Cloudflare account.
//
//   node deployed.mjs <base-url> <run-id>      (HARNESS_TOKEN in the environment)
//
// The local leg runs the same Worker under `workerd`; this one runs it where a
// consumer's object runs, which is the only place the platform's own limits
// (the row ceiling among them) can be observed rather than inferred.
//
// Strict, like the local leg: it exits non-zero if any rule fails, if a rule's
// name is unknown to the dispatcher, or if fewer rules ran than were listed.
import { writeFileSync } from "node:fs";

const [base, runId] = process.argv.slice(2);
const token = process.env.HARNESS_TOKEN;
if (!base || !runId || !token) {
  console.error("usage: HARNESS_TOKEN=... node deployed.mjs <base-url> <run-id>");
  process.exit(2);
}

async function call(object, route) {
  const response = await fetch(`${base}/do/${object}/${route}`, {
    headers: { authorization: `Bearer ${token}` },
  });
  return { status: response.status, body: await response.text() };
}

const listing = await call(`${runId}-rules`, "rules");
if (listing.status !== 200) {
  console.error(`listing the rules failed: ${listing.status} ${listing.body}`);
  process.exit(1);
}
const rules = listing.body.split("\n").filter((name) => name.length > 0);
console.log(`rules listed: ${rules.length}`);

const failures = [];
let executed = 0;
for (const rule of rules) {
  // One object per rule per run: every rule starts from empty storage, and a
  // re-run never inherits a previous run's log.
  const { status, body } = await call(`${runId}-${rule}`, `rule/${rule}`);
  executed += 1;
  if (status === 200) {
    console.log(`ok   ${rule}${body.startsWith("skipped\n") ? `  (${body.slice(8)})` : ""}`);
  } else {
    console.log(`FAIL ${rule}  [${status}] ${body.split("\n")[0]}`);
    failures.push({ rule, status, body });
  }
}

const probe = await call(`${runId}-probe`, "probe");
console.log(`--- deployed Durable Object SQLite walls ---\n${probe.body}`);
writeFileSync("deployed-walls.txt", probe.body);
writeFileSync("deployed-failures.json", JSON.stringify(failures, null, 2));

console.log(`executed ${executed} of ${rules.length}; ${failures.length} failed`);
if (probe.status !== 200) {
  console.error(`the probe failed: ${probe.status}`);
  process.exit(1);
}
if (executed !== rules.length || failures.length > 0) {
  process.exit(1);
}
