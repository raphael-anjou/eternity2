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
