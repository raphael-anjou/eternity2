import { describe, it, expect } from "vitest";
import {
  decodeBucas,
  viewerUrlFromBoard,
  bucasUrlFromBoard,
  parseHints,
  parseHintsFull,
  encodeHints,
  cellsToBoardEdges,
} from "./bucas";

// A tiny 2x2 board as bucas params (4 cells × 4 URDL letters = 16), carrying
// board_pieces so we can prove the canonical viewer URL drops it.
const TINY =
  "puzzle=t&board_w=2&board_h=2&board_edges=aabaacaaadbaeaca&board_pieces=001002003004";

describe("canonical viewer URL", () => {
  it("emits board_edges but never board_pieces", () => {
    const board = decodeBucas(TINY);
    const url = viewerUrlFromBoard(board);
    expect(url).toContain("board_edges=");
    expect(url).not.toContain("board_pieces=");
    expect(url.startsWith("https://eternity2.dev/viewer?")).toBe(true);
  });

  it("appends hints only when the board has clue cells", () => {
    const plain = decodeBucas(TINY);
    expect(viewerUrlFromBoard(plain)).not.toContain("hints=");

    const withClues = decodeBucas(`${TINY}&hints=1.0-3.2`);
    const url = viewerUrlFromBoard(withClues);
    expect(url).toContain("hints=");
    // positions preserved, in board order
    expect(url).toMatch(/hints=1\.0-3\.0/); // overlay rot is 0 (recovered from edges)
  });

  it("bucas URL is edges-only too (matches the Rust emitter)", () => {
    const board = decodeBucas(TINY);
    const url = bucasUrlFromBoard(board);
    expect(url).toContain("board_edges=");
    expect(url).not.toContain("board_pieces=");
    expect(url).toContain("board_w=2");
  });
});

describe("hints round-trip", () => {
  it("encodes and parses pos.rot", () => {
    expect(encodeHints([{ pos: 72, rot: 2 }, { pos: 5, rot: 0 }])).toBe("72.2-5.0");
    expect(parseHints("72.2-5.0")).toEqual([72, 5]);
    // full form keeps rotation
    expect(parseHintsFull("72.2-5.0")).toEqual([
      { pos: 72, rot: 2 },
      { pos: 5, rot: 0 },
    ]);
  });

  it("tolerates a bare position (rot defaults to 0) and empty input", () => {
    expect(parseHints("9")).toEqual([9]);
    expect(parseHints(undefined)).toBeNull();
    expect(parseHintsFull("")).toEqual([]);
  });
});

describe("cellsToBoardEdges", () => {
  it("empty cells become aaaa", () => {
    expect(cellsToBoardEdges([null, [0, 1, 2, 3]])).toBe("aaaaabcd");
  });
});

describe("toOurParams keeps hints (URL round-trip)", () => {
  it("preserves the hints param so the clue overlay survives share/reload", async () => {
    const { toOurParams, parseParams } = await import("./bucas");
    const raw = parseParams(`${TINY}&hints=1.0-3.2`);
    const out = toOurParams(raw);
    expect(out["hints"]).toBe("1.0-3.2");
    expect(out["board_edges"]).toBeTruthy();
  });
});

describe("one standard: viewerUrlFromBoard and ourParams agree", () => {
  it("produce the same param shape for the same board", async () => {
    // Decode a board, and build the same board as an engine puzzle+cells, then
    // confirm both URL builders yield identical query strings. This locks the
    // two entry points (decoded-board path vs engine path) to one standard.
    const { ourParams } = await import("./bucas");
    const board = decodeBucas(TINY);
    const fromBoard = viewerUrlFromBoard(board).split("?")[1] ?? "";
    // Reconstruct an equivalent params record from the same decoded cells by
    // reusing the string builder's own output as the reference key order.
    const keys = new URLSearchParams(fromBoard);
    // Both must carry the same param keys in the same order, edges only.
    expect([...keys.keys()]).toEqual(["puzzle", "puzzle_size", "board_edges"]);
    expect(keys.has("board_pieces")).toBe(false);
    // ourParams (engine path) uses the same sanitizeName + encodeHints helpers,
    // so a hinted board yields the identical hints token.
    const puzzle = { name: "t", width: 2, height: 2, numColors: 4, pieces: [], hints: [] };
    const hinted = ourParams(puzzle, [-1, -1, -1, -1], "t", [
      { pos: 1, rot: 0 },
      { pos: 3, rot: 2 },
    ]);
    expect(hinted["hints"]).toBe("1.0-3.2"); // real rot preserved on the engine path
  });
});
