// What the deployed leg retries, pinned without a deployment.
//
// The deployed leg waits out Cloudflare's propagation answers and retries
// nothing else (platform-miss.mjs). Each case names the wrong implementation it
// rejects: a matcher that also swallows the harness's own 404, one that misses
// the HTML page a real run got, a loop that never gives up or gives up early.
import { describe, expect, it } from "vitest";
import { LISTING_ATTEMPTS, callPastPropagation, failureLine, isPlatformMiss } from "../platform-miss.mjs";

// An unclassified Cloudflare 500, in the shape of the error pages the edge
// serves: one line of doctype, then markup whose first line says nothing. The
// deployed leg met one on PR #51 and printed only that first line.
const UNCLASSIFIED_PAGE =
  '<!DOCTYPE html>\n<!--[if lt IE 7]> <html class="no-js ie6 oldie" lang="en-US"> <![endif]-->\n<head>\n<title>Worker threw exception | example.workers.dev | Cloudflare</title>\n</head><body><h1>Error 1101</h1></body></html>';

// The page run 37470989256 (attempt 1) got on the listing, cut to its marker.
const PROPAGATING_PAGE =
  '<!DOCTYPE html><html><head><title>Page not found</title></head><body><h1>There is nothing here yet</h1><p>If you expect something to be here, it may take some time.</p></body></html>';

// The page run 38076989779 (job 114285992747) got on the listing, cut to its
// title and code.
const SCRIPT_NOT_FOUND_PAGE =
  '<!DOCTYPE html><html><head><title>Script not found | happenstance-workerd-harness.example.workers.dev | Cloudflare</title></head><body><h1><span class="cf-error-type" data-translate="error">Error</span> <span class="cf-error-code">1104</span></h1><h2 class="cf-subheadline" data-translate="error_desc">Script not found</h2></body></html>';

type Answer ={ status: number; body: string };

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
  it("matches each of Cloudflare's propagation answers", () => {
    expect(isPlatformMiss(500, "Worker not found.\n")).toBe(true);
    expect(isPlatformMiss(404, PROPAGATING_PAGE)).toBe(true);
    expect(isPlatformMiss(500, "Durable Object reset because its code was updated.")).toBe(true);
    expect(isPlatformMiss(500, SCRIPT_NOT_FOUND_PAGE)).toBe(true);
  });

  it("matches the script-not-found page only as Cloudflare serves it", () => {
    // A rule's own failure that merely says the words is the rule's failure.
    expect(isPlatformMiss(500, "Rust panic: Script not found (error 1104)")).toBe(false);
    expect(isPlatformMiss(200, SCRIPT_NOT_FOUND_PAGE)).toBe(false);
  });

  it("matches the reset only as the platform words it", () => {
    // A rule whose own message merely mentions a reset is the rule's failure.
    expect(isPlatformMiss(500, "Rust panic: Durable Object reset because its code was updated. (in suite.rs)")).toBe(false);
    expect(isPlatformMiss(404, "Durable Object reset because its code was updated.")).toBe(false);
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

describe("failureLine", () => {
  // The wrong implementation each case rejects: the old first-line print, which
  // turns every HTML page into `<!DOCTYPE html>`; one that reads the title out
  // of a plain-text body; and one that drops the status.
  it("names an HTML page by its title", () => {
    expect(failureLine(500, UNCLASSIFIED_PAGE)).toBe(
      "[500] Worker threw exception | example.workers.dev | Cloudflare  (HTML page)",
    );
  });

  it("reads a title written in any case, across lines, with entities left as sent", () => {
    expect(failureLine(502, "<HTML><Title>\n  Bad gateway &amp; more\n</TITLE></HTML>")).toBe(
      "[502] Bad gateway &amp; more  (HTML page)",
    );
  });

  it("keeps the first line of a plain-text body, as a rule's own failure reports it", () => {
    expect(failureLine(500, "Rust panic: panicked at suite.rs:1:1\nstack...")).toBe(
      "[500] Rust panic: panicked at suite.rs:1:1",
    );
    // A rule message that mentions a title tag is still the rule's own text.
    expect(failureLine(500, "assertion failed: <title>x</title> was not expected")).toBe(
      "[500] assertion failed: <title>x</title> was not expected",
    );
  });

  it("falls back to the first line of an HTML page with no title", () => {
    expect(failureLine(500, "<!DOCTYPE html>\n<html><body>oops</body></html>")).toBe(
      "[500] <!DOCTYPE html>",
    );
  });

  it("does not make an unclassified page retryable", () => {
    expect(isPlatformMiss(500, UNCLASSIFIED_PAGE)).toBe(false);
  });
});
