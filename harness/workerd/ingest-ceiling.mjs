// The ingest ceiling's bisection (#108 part E3), shared by
// test/ingest-ceiling.test.ts (local workerd) and deployed-ingest.mjs (the
// deployed object). Pure: the caller supplies the attempt and the clock, so
// test/ingest-ceiling-bisect.test.ts pins it without a runtime.
//
// The search lives here rather than in the object, unlike src/probe.rs's:
// each attempt is one request on its own object, because a shape near the top
// can exhaust memory or CPU, which traps or resets the object rather than
// answering an error, and an in-object bisection would die with its first
// trap. A thrown attempt is therefore a refusal, quoted by its message.

// What is searched, and how far. `minimal` and `max-tags` stop one past the
// adapter's declared 1,024 events per call, so the top answers the adapter's
// own refusal (`class=Split`); `max-tags` meets the adapter's 262,144 tags per
// call first, so its bisection finds 256 and the refusal above it names the
// total (ADR-0100). `max-data` stops at 256 MiB in one call, twice the
// deployed isolate's 128 MiB memory limit, src/probe.rs's "twice the
// platform's own ceiling" convention.
export const SHAPES = [
  { shape: "minimal", searchedTo: 1025 },
  { shape: "max-tags", searchedTo: 1025 },
  { shape: "max-data", searchedTo: 256 },
];

// Bisects `1..=searchedTo` for the largest size `attempt` accepts, assuming
// acceptance is monotone in size, which is what a limit is: try 1, try the
// top, then halve. `attempt(n)` answers `{ accepted, text }`; a throw is a
// refusal carrying its message. No size is attempted twice. The round trip is
// measured here, by `now`, because `Date.now()` inside a Worker does not
// advance during a synchronous transaction.
//
// Answers `{ accepted, searchedTo, refusal, roundTripMs }`: the largest size
// accepted (or null), the refusal at the size above it (or null when nothing
// up to the top was refused), and the round trip of the accepted attempt.
export async function bisectCeiling(attempt, searchedTo, now = Date.now) {
  const seen = new Map();
  const tryAt = async (n) => {
    if (!seen.has(n)) {
      const started = now();
      let answer;
      try {
        answer = await attempt(n);
      } catch (error) {
        answer = { accepted: false, text: error instanceof Error ? error.message : String(error) };
      }
      seen.set(n, { accepted: answer.accepted, text: answer.text, ms: now() - started });
    }
    return seen.get(n);
  };
  const wall = (accepted, refusal) => ({
    accepted,
    searchedTo,
    refusal,
    roundTripMs: accepted === null ? null : seen.get(accepted).ms,
  });

  const first = await tryAt(1);
  if (!first.accepted) {
    return wall(null, first.text);
  }
  if ((await tryAt(searchedTo)).accepted) {
    return wall(searchedTo, null);
  }
  // Invariant: `low` accepted, `high` refused.
  let low = 1;
  let high = searchedTo;
  while (high - low > 1) {
    const mid = low + Math.floor((high - low) / 2);
    if ((await tryAt(mid)).accepted) {
      low = mid;
    } else {
      high = mid;
    }
  }
  return wall(low, (await tryAt(high)).text);
}

// One line, in `Wall::line`'s exact format (src/probe.rs), so the ingest
// walls read like every other wall in experiments/durable-object-limits.
export function wallLine(axis, wall) {
  const accepted = wall.accepted === null ? "none" : String(wall.accepted);
  return wall.refusal === null
    ? `${axis}: accepted ${accepted}; no wall up to ${wall.searchedTo}`
    : `${axis}: accepted ${accepted}; refused above it: ${wall.refusal}`;
}

// The report line for one shape: its wall, and the round trip at the boundary.
export function ceilingLine(shape, wall) {
  const line = wallLine(`ingest ${shape}`, wall);
  return wall.roundTripMs === null
    ? line
    : `${line} (round trip at ${wall.accepted}: ${wall.roundTripMs} ms)`;
}

// How `/ingest-ceiling/<shape>?events=<n>` reads as an attempt: accepted only
// on a 200 whose body is exactly `accepted events=<n> appended=<n>`, every
// event sent having landed; anything else is a refusal, its status kept and
// its body flattened onto one line, verbatim otherwise. A call that committed
// fewer rows than it carried is not a call of that size.
export function readAttempt(status, body) {
  const counts = /^accepted events=(\d+) appended=(\d+)$/.exec(body.trim());
  const accepted = status === 200 && counts !== null && counts[1] === counts[2];
  return {
    accepted,
    text: accepted ? body : `[${status}] ${body.trim().replace(/\s*\n\s*/g, " ")}`,
  };
}
