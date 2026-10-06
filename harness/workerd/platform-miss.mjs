// What the deployed leg treats as Cloudflare still propagating a fresh Worker,
// and how long it waits that out. Its own module so that
// test/platform-miss.test.ts can pin both without a deployment.
//
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
export function isPlatformMiss(status, body) {
  return (
    (status === 500 && body.trim() === "Worker not found.") ||
    (status === 404 && body.includes("<h1>There is nothing here yet</h1>"))
  );
}

export const LISTING_ATTEMPTS = 6;
export const LISTING_DELAY_MS = 10_000;

// The listing and the probe touch no rule, so they wait out propagation a little
// longer than a rule does: up to LISTING_ATTEMPTS attempts, LISTING_DELAY_MS
// apart, each retry said so, the last answer returned as it came. `call`,
// `pause` and `log` are the caller's, so a test drives this without a network
// or a clock.
export async function callPastPropagation(call, object, route, { pause, log }) {
  let result = await call(object, route);
  for (
    let attempt = 2;
    attempt <= LISTING_ATTEMPTS && isPlatformMiss(result.status, result.body);
    attempt += 1
  ) {
    log(`retry ${route} (attempt ${attempt})  [${result.status}] platform miss`);
    await pause(LISTING_DELAY_MS);
    result = await call(object, route);
  }
  return result;
}
