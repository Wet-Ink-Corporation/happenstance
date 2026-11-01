// The largest single ingest call local workerd commits, per event shape
// (#108 part E3). The runner bisects, one object per attempt, because a shape
// near the top can trap the instance rather than answer (ingest-ceiling.mjs).
//
// Strict on two axes. `minimal`: 1,024 events is the adapter's declared
// per-call ceiling, so the runtime must commit it and the attempt at 1,025
// must be the adapter's own refusal, `class=Split`. `max-tags`: 512 events of
// 1,024 tags is the declared 524,288 tags per call (ADR-0100), so the runtime
// must commit 512 and the attempt at 513 must be the adapter's own refusal,
// `class=Split`, naming the total. `max-data` is printed and copied into
// experiments/durable-object-limits/results/ when it is taken as a
// measurement. It carries its own timeout, as limits.test.ts
// does: the bisections take minutes.
import { env } from "cloudflare:workers";
import { expect, it } from "vitest";
import { SHAPES, bisectCeiling, ceilingLine, readAttempt } from "../ingest-ceiling.mjs";

let objects = 0;

async function attempt(shape: string, events: number) {
  // A fresh object per attempt: a trap or a reset takes only that one.
  objects += 1;
  const stub = env.CONFORMANCE.get(env.CONFORMANCE.idFromName(`ingest-ceiling-${shape}-${events}-${objects}`));
  const response = await stub.fetch(`https://harness/ingest-ceiling/${shape}?events=${events}`);
  return readAttempt(response.status, await response.text());
}

it("measures the ingest ceiling of local workerd", async () => {
  const lines: string[] = [];
  const walls = new Map();
  for (const { shape, searchedTo } of SHAPES) {
    const wall = await bisectCeiling((n: number) => attempt(shape, n), searchedTo);
    walls.set(shape, wall);
    lines.push(ceilingLine(shape, wall));
  }
  console.log(`--- workerd ingest ceiling ---\n${lines.join("\n")}`);

  const minimal = walls.get("minimal");
  expect(minimal.accepted, lines.join("\n")).toBe(1024);
  expect(minimal.refusal, lines.join("\n")).toContain("class=Split");

  const maxTags = walls.get("max-tags");
  expect(maxTags.accepted, lines.join("\n")).toBe(512);
  expect(maxTags.refusal, lines.join("\n")).toContain("class=Split");
  expect(maxTags.refusal, lines.join("\n")).toContain("tags per batch limit: 525312");
}, 900_000);
