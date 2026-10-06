// What the deployed leg retries, pinned without a deployment.
//
// The deployed leg waits out Cloudflare's propagation answers and retries
// nothing else (platform-miss.mjs). Each case names the wrong implementation it
// rejects: a matcher that also swallows the harness's own 404, one that misses
// the HTML page a real run got, a loop that never gives up or gives up early.
import { describe, expect, it } from "vitest";
import { LISTING_ATTEMPTS, callPastPropagation, isPlatformMiss } from "../platform-miss.mjs";

// The page run 37470989256 (attempt 1) got on the listing, cut to its marker.
const PROPAGATING_PAGE =
  '<!DOCTYPE html><html><head><title>Page not found</title></head><body><h1>There is nothing here yet</h1><p>If you expect something to be here, it may take some time.</p></body></html>';

type Answer = { status: number; body: string };

// A `call` that answers from a script, one entry per request, and counts them.
function scripted(answers: Answer[]) {
  let calls = 0;
  const call = async () => {
    const answer = answers[Math.min(calls, answers.length - 1)];
    calls += 1;
    return answer;
  };
  return { call, calls: () => calls };
}

const quiet = { pause: async () => {}, log: () => {} };

describe("isPlatformMiss", () => {
  it("matches both of Cloudflare's propagation answers", () => {
    expect(isPlatformMiss(500, "Worker not found.\n")).toBe(true);
    expect(isPlatformMiss(404, PROPAGATING_PAGE)).toBe(true);
  });

  it("never matches the harness's own answers", () => {
    expect(isPlatformMiss(404, "no such rule")).toBe(false);
    expect(isPlatformMiss(404, "no route /x")).toBe(false);
    expect(isPlatformMiss(404, "expected /do/<object>/<route>")).toBe(false);
    expect(isPlatformMiss(500, "Rust panic: panicked at suite.rs:1:1")).toBe(false);
    expect(isPlatformMiss(200, PROPAGATING_PAGE)).toBe(false);
  });
});

describe("callPastPropagation", () => {
  it("waits out a miss that clears and returns the answer behind it", async () => {
    const edge = scripted([
      { status: 404, body: PROPAGATING_PAGE },
      { status: 500, body: "Worker not found." },
      { status: 200, body: "rule_a\n" },
    ]);
    const result = await callPastPropagation(edge.call, "o", "rules", quiet);
    expect(result).toEqual({ status: 200, body: "rule_a\n" });
    expect(edge.calls()).toBe(3);
  });

  it("does not retry the harness's own failure", async () => {
    const edge = scripted([{ status: 404, body: "no such rule" }]);
    const result = await callPastPropagation(edge.call, "o", "rules", quiet);
    expect(result.status).toBe(404);
    expect(edge.calls()).toBe(1);
  });

  it("gives up after its attempts and returns the last answer", async () => {
    const edge = scripted([{ status: 404, body: PROPAGATING_PAGE }]);
    const result = await callPastPropagation(edge.call, "o", "rules", quiet);
    expect(result.status).toBe(404);
    expect(edge.calls()).toBe(LISTING_ATTEMPTS);
    expect(LISTING_ATTEMPTS).toBe(6);
  });
});
