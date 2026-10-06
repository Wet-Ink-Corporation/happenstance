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

// Cloudflare's own answers when a request reaches a location the freshly
// deployed Worker has not propagated to yet. Both are raised before any harness
// or adapter code runs, so they say nothing about a rule:
// - `500 Worker not found.`, which the first deployed run saw on two rules that
//   pass everywhere else;
// - a `404` HTML page headed "There is nothing here yet", which a run on PR #36
//   (37470989256, attempt 1) got on the rule listing one request after the
//   readiness probe had seen a 200.
// Matched exactly. The harness's own 404s (`no such rule`, `no route …`,
// `expected /do/…`) are plain text and never match, so a rule's own failure (a
// panic, an unknown rule, a harness fault) is never retried and the strict
// failure list stays strict.
function isPlatformMiss(status, body) {
  return (
    (status === 500 && body.trim() === "Worker not found.") ||
    (status === 404 && body.includes("<h1>There is nothing here yet</h1>"))
  );
}

const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// The listing and the probe touch no rule, so they wait out propagation a little
// longer than a rule does: up to six attempts, ten seconds apart, each retry said
// so, the last answer reported as it came.
async function callPastPropagation(object, route) {
  let result = await call(object, route);
  for (let attempt = 2; attempt <= 6 && isPlatformMiss(result.status, result.body); attempt += 1) {
    console.log(`retry ${route} (attempt ${attempt})  [${result.status}] platform miss`);
    await pause(10_000);
    result = await call(object, route);
  }
  return result;
}

const listing = await callPastPropagation(`${runId}-rules`, "rules");
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
  let { status, body } = await call(`${runId}-${rule}`, `rule/${rule}`);
  if (isPlatformMiss(status, body)) {
    // Retried once, on a fresh object, and said so. See isPlatformMiss.
    console.log(`retry ${rule}  [${status}] platform miss`);
    await pause(5_000);
    ({ status, body } = await call(`${runId}-${rule}-retry`, `rule/${rule}`));
  }
  executed += 1;
  if (status === 200) {
    console.log(`ok   ${rule}${body.startsWith("skipped\n") ? `  (${body.slice(8)})` : ""}`);
  } else {
    console.log(`FAIL ${rule}  [${status}] ${body.split("\n")[0]}`);
    failures.push({ rule, status, body });
  }
}

const probe = await callPastPropagation(`${runId}-probe`, "probe");
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
