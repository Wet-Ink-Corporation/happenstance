// What the ingest ceiling's bisection decides, pinned without a runtime.
//
// The search lives in JavaScript (ingest-ceiling.mjs), one request per
// attempt, because a shape near the top can trap or reset the object rather
// than answer an error. Each case names the wrong search it rejects: one that
// reports the last accepted size it happened to try rather than the boundary,
// one that loses the refusal's text, one that dies with a trapped attempt
// instead of counting it, and a line format that drifts from `Wall::line`
// (src/probe.rs), which the result files are read against.
import { describe, expect, it } from "vitest";
import { SHAPES, bisectCeiling, ceilingLine, readAttempt, wallLine } from "../ingest-ceiling.mjs";
import { isPlatformMiss } from "../platform-miss.mjs";

// A runtime whose wall sits at `wall`, refusing above it with its size named.
function walled(wall: number) {
  const tried: number[] = [];
  const attempt = async (n: number) => {
    tried.push(n);
    return n <= wall
      ? { accepted: true, text: `accepted events=${n} appended=${n}` }
      : { accepted: false, text: `[422] refused class=Stop too big at ${n}` };
  };
  return { attempt, tried };
}

// A clock that advances 5 ms per reading, so a round trip is never zero.
function ticking() {
  let t = 0;
  return () => (t += 5);
}

describe("bisectCeiling", () => {
  it("finds the boundary and quotes the refusal just above it", async () => {
    const { attempt } = walled(37);
    const wall = await bisectCeiling(attempt, 1025, ticking());
    expect(wall.accepted).toBe(37);
    expect(wall.searchedTo).toBe(1025);
    expect(wall.refusal).toBe("[422] refused class=Stop too big at 38");
    expect(wall.roundTripMs).toBe(5);
  });

  it("reports no wall when the top is accepted", async () => {
    const { attempt, tried } = walled(10_000);
    const wall = await bisectCeiling(attempt, 256, ticking());
    expect(wall).toEqual({ accepted: 256, searchedTo: 256, refusal: null, roundTripMs: 5 });
    expect(tried).toEqual([1, 256]);
  });

  it("reports nothing accepted when one event is refused", async () => {
    const { attempt, tried } = walled(0);
    const wall = await bisectCeiling(attempt, 1025, ticking());
    expect(wall).toEqual({
      accepted: null,
      searchedTo: 1025,
      refusal: "[422] refused class=Stop too big at 1",
      roundTripMs: null,
    });
    expect(tried).toEqual([1]);
  });

  it("counts a thrown attempt, a trap, as a refusal with its message", async () => {
    const attempt = async (n: number) => {
      if (n > 3) {
        throw new Error("Rust panic: memory access out of bounds");
      }
      return { accepted: true, text: "accepted" };
    };
    const wall = await bisectCeiling(attempt, 64, ticking());
    expect(wall.accepted).toBe(3);
    expect(wall.refusal).toBe("Rust panic: memory access out of bounds");
  });

  it("asks for no size twice", async () => {
    const { attempt, tried } = walled(37);
    await bisectCeiling(attempt, 1025, ticking());
    expect(new Set(tried).size).toBe(tried.length);
  });
});

describe("the report lines", () => {
  it("match Wall::line byte for byte", () => {
    expect(wallLine("axis", { accepted: 600, searchedTo: 600, refusal: null })).toBe(
      "axis: accepted 600; no wall up to 600",
    );
    expect(wallLine("axis", { accepted: null, searchedTo: 600, refusal: "refused 1" })).toBe(
      "axis: accepted none; refused above it: refused 1",
    );
  });

  it("name the shape and the round trip at the boundary", () => {
    const wall = { accepted: 1024, searchedTo: 1025, refusal: "[422] refused class=Split x", roundTripMs: 412 };
    expect(ceilingLine("minimal", wall)).toBe(
      "ingest minimal: accepted 1024; refused above it: [422] refused class=Split x (round trip at 1024: 412 ms)",
    );
  });
});

describe("readAttempt", () => {
  it("accepts only a 200 that says accepted, and flattens a refusal to one line", () => {
    expect(readAttempt(200, "accepted events=4 appended=4")).toEqual({
      accepted: true,
      text: "accepted events=4 appended=4",
    });
    expect(readAttempt(422, "refused class=Split\ningest exceeds the store's events per batch limit: 1025\n")).toEqual({
      accepted: false,
      text: "[422] refused class=Split ingest exceeds the store's events per batch limit: 1025",
    });
    expect(readAttempt(404, "no route /ingest-ceiling/minimal").accepted).toBe(false);
  });

  it("refuses a 200 whose store appended fewer events than were sent", () => {
    // A store that skipped or dropped rows committed a smaller call than the
    // one asked for, so the size is not what the runtime took.
    expect(readAttempt(200, "accepted events=1024 appended=1000")).toEqual({
      accepted: false,
      text: "[200] accepted events=1024 appended=1000",
    });
    expect(readAttempt(200, "accepted").accepted).toBe(false);
    expect(readAttempt(200, "accepted events=7 appended=7 extra").accepted).toBe(false);
  });
});

describe("the searched shapes", () => {
  it("search one past the declared ceiling, and max-data to twice the isolate's memory", () => {
    expect(SHAPES).toEqual([
      { shape: "minimal", searchedTo: 1025 },
      { shape: "max-tags", searchedTo: 1025 },
      { shape: "max-data", searchedTo: 256 },
    ]);
  });
});

describe("what the deployed leg retries", () => {
  it("never treats a rule's violation as a platform miss", () => {
    expect(isPlatformMiss(422, "violated\nSY-30 violated: the group was split")).toBe(false);
    expect(isPlatformMiss(422, "refused class=Split\ningest exceeds")).toBe(false);
  });
});
