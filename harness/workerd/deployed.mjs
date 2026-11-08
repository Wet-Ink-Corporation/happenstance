// The deployed leg: every conformance rule, then the wall probe, against the
// harness Worker deployed to a real Cloudflare account.
//
//   node deployed.mjs <base-url> <run-id>      (HARNESS_TOKEN in the environment)
//
// The local leg runs the same Worker under `workerd`; this one runs it where a
// consumer's object runs, which is the only place the platform's own limits
// (the row ceiling among them) can be observed rather than inferred.
//
// Three families, each listed by the harness and each strict: the event-store
// rules (`/rules`, `/rule/<name>`), the ingest rules (`/ingest-rules`,
// `/ingest-rule/<name>`, #108 part E3) and the write-group rules
// (`/write-group-rules`, `/write-group-rule/<name>`, #420). It exits non-zero
// if any rule of any fails, if a rule's name is unknown to the dispatcher, if
// fewer rules ran than were listed, or if a family lists fewer than its floor.
// An ingest or write-group rule's violation answers 422, which is a failure
// line and never a platform miss.
import { writeFileSync } from "node:fs";
import { callPastPropagation, failureLine, isPlatformMiss } from "./platform-miss.mjs";

// `prefix` keeps each family's objects apart: one object per rule per run.
// `floor` is the vacuity guard the local steps in ci.yml carry, for the
// listing: today's count, a floor rather than an equality for live-neon's
// reason (the commit that adds a rule must stay green). 95 rules and CF-18's
// check; 6 rules and the ingest check; 6 rules.
const FAMILIES = [
  { name: "event store", list: "rules", run: "rule", prefix: "", floor: 96 },
  { name: "ingest", list: "ingest-rules", run: "ingest-rule", prefix: "ingest-", floor: 7 },
  {
    name: "write group",
    list: "write-group-rules",
    run: "write-group-rule",
    prefix: "write-group-",
    floor: 6,
  },
];

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

const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const past = (object, route) =>
  callPastPropagation(call, object, route, { pause, log: console.log });

const failures = [];
const tallies = [];
for (const family of FAMILIES) {
  const listing = await past(`${runId}-${family.list}`, family.list);
  if (listing.status !== 200) {
    console.error(
      `listing the ${family.name} rules failed: ${failureLine(listing.status, listing.body)}\n${listing.body}`,
    );
    process.exit(1);
  }
  const rules = listing.body.split("\n").filter((name) => name.length > 0);
  console.log(`${family.name} rules listed: ${rules.length}`);

  let executed = 0;
  for (const rule of rules) {
    // One object per rule per run: every rule starts from empty storage, and a
    // re-run never inherits a previous run's log.
    const object = `${runId}-${family.prefix}${rule}`;
    let { status, body } = await call(object, `${family.run}/${rule}`);
    if (isPlatformMiss(status, body)) {
      // Retried once, on a fresh object, and said so. See platform-miss.mjs.
      console.log(`retry ${family.prefix}${rule}  [${status}] platform miss`);
      await pause(5_000);
      ({ status, body } = await call(`${object}-retry`, `${family.run}/${rule}`));
    }
    executed += 1;
    if (status === 200) {
      console.log(`ok   ${family.prefix}${rule}${body.startsWith("skipped\n") ? `  (${body.slice(8)})` : ""}`);
    } else {
      console.log(`FAIL ${family.prefix}${rule}  ${failureLine(status, body)}`);
      failures.push({ family: family.name, rule, status, body });
    }
  }
  tallies.push({ family: family.name, executed, listed: rules.length, floor: family.floor });
}

const probe = await past(`${runId}-probe`, "probe");
console.log(`--- deployed Durable Object SQLite walls ---\n${probe.body}`);
writeFileSync("deployed-walls.txt", probe.body);
writeFileSync("deployed-failures.json", JSON.stringify(failures, null, 2));

for (const { family, executed, listed, floor } of tallies) {
  console.log(`${family}: executed ${executed} of ${listed} (floor ${floor})`);
}
console.log(`${failures.length} failed`);
if (probe.status !== 200) {
  console.error(`the probe failed: ${failureLine(probe.status, probe.body)}`);
  process.exit(1);
}
const short = tallies.filter(({ listed, floor }) => listed < floor);
for (const { family, listed, floor } of short) {
  console.error(`only ${listed} ${family} rules were listed, below the floor of ${floor}`);
}
if (
  tallies.some(({ executed, listed }) => executed !== listed) ||
  short.length > 0 ||
  failures.length > 0
) {
  process.exit(1);
}
