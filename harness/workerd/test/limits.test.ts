// The SQLite walls of this runtime, measured rather than read from source.
//
// Not an assertion about the adapter: the report is printed, and copied into
// experiments/durable-object-limits/results/ when it is taken as a measurement.
// It fails only if the probe itself could not run. It carries its own timeout:
// the bisections take minutes, past the 60 s the rules run under.
import { env } from "cloudflare:workers";
import { expect, it } from "vitest";

it("measures the Durable Object's SQLite walls", async () => {
  const stub = env.CONFORMANCE.get(env.CONFORMANCE.idFromName("probe"));
  const response = await stub.fetch("https://harness/probe");
  const report = await response.text();
  console.log(`--- workerd SQLite walls ---\n${report}`);
  expect(response.status, report).toBe(200);
}, 900_000);
