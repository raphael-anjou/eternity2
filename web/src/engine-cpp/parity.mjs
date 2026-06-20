// Parity test: validates the C++/wasm engine against the golden outputs the
// Rust engine produced (engine-side-quests/engine-lua/golden.txt). Runs under
// Node 22's built-in WebAssembly — no bundler, no DOM, no emscripten runtime.
//
//   node web/src/engine-cpp/parity.mjs
//
// Every assertion compares wasm output to a value the Rust engine actually
// produced: generated puzzles (RNG parity), the official set, every path
// permutation, and full solver runs DOWN TO node / attempt / backtrack counts.
// Matching the counts proves the two engines walk the identical search tree.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const goldenPath = join(here, "../../../engine-side-quests/engine-lua/golden.txt");
const wasmPath = join(here, "engine.wasm");

const PATH_KINDS = [
  "row-major", "snake", "column-major", "spiral-in", "spiral-out",
  "diagonal", "border-first", "double-snake", "random",
];

// --- instantiate -------------------------------------------------------------
const bytes = readFileSync(wasmPath);
const { instance } = await WebAssembly.instantiate(bytes, {});
const e = instance.exports;
const mem = () => new DataView(e.memory.buffer);
const i32 = () => new Int32Array(e.memory.buffer);

const WIRE = e.e2_wire_ptr() >> 2; // i32 index of the wire buffer
const SCRATCH = e.e2_scratch_ptr() >> 2; // i32 index of the scratch buffer

// Read the puzzle currently staged in the wire buffer back into a JS object.
function readWirePuzzle() {
  const m = i32();
  const width = m[WIRE + 0];
  const height = m[WIRE + 1];
  const numColors = m[WIRE + 2];
  const nPieces = m[WIRE + 3];
  let o = WIRE + 4;
  const pieces = [];
  for (let i = 0; i < nPieces; i++) {
    pieces.push([m[o], m[o + 1], m[o + 2], m[o + 3]]);
    o += 4;
  }
  const nHints = m[o++];
  const hints = [];
  for (let i = 0; i < nHints; i++) {
    hints.push({ pos: m[o], piece: m[o + 1], rot: m[o + 2] });
    o += 3;
  }
  return { width, height, numColors, pieces, hints };
}

// Write a JS puzzle into the wire buffer (for score / solver construction).
function writeWirePuzzle(p) {
  const m = i32();
  m[WIRE + 0] = p.width;
  m[WIRE + 1] = p.height;
  m[WIRE + 2] = p.numColors;
  m[WIRE + 3] = p.pieces.length;
  let o = WIRE + 4;
  for (const e4 of p.pieces) {
    m[o++] = e4[0]; m[o++] = e4[1]; m[o++] = e4[2]; m[o++] = e4[3];
  }
  m[o++] = p.hints.length;
  for (const h of p.hints) { m[o++] = h.pos; m[o++] = h.piece; m[o++] = h.rot; }
}

function readScratch(n) {
  const m = i32();
  const out = new Array(n);
  for (let i = 0; i < n; i++) out[i] = m[SCRATCH + i];
  return out;
}
function writeScratch(arr) {
  const m = i32();
  for (let i = 0; i < arr.length; i++) m[SCRATCH + i] = arr[i];
}

// --- high-level engine ops ---------------------------------------------------
function generate(size, colors, seed) {
  e.e2_generate(size, colors, seed >>> 0);
  return readWirePuzzle();
}
function official() {
  e.e2_official();
  return readWirePuzzle();
}
function buildPath(kindIdx, w, h, seed) {
  const n = e.e2_build_path(kindIdx, w, h, seed >>> 0);
  return readScratch(n);
}
function solveToEnd(puzzle, kindIdx, w, h) {
  writeWirePuzzle(puzzle);
  const n = e.e2_build_path(kindIdx, w, h, 0);
  const path = readScratch(n);
  writeWirePuzzle(puzzle);
  writeScratch(path);
  const slot = e.e2_solver_new(path.length, 1, 0, 0);
  if (slot < 0) throw new Error("solver_new failed");
  do {
    e.e2_solver_step(slot, 5_000_000);
  } while (e.e2_solver_status(slot) === 0);
  const status = e.e2_solver_status(slot);
  const placed = e.e2_solver_placed(slot);
  e.e2_solver_board(slot); // board -> scratch
  // rescore from the staged puzzle (matches the lua/rust parity flow)
  writeWirePuzzle(puzzle);
  const score = e.e2_score_board(w * h);
  const rep = {
    status: status === 1 ? "solved" : status === 2 ? "exhausted" : "running",
    placed,
    score,
    nodes: e.e2_solver_nodes(slot),
    attempts: e.e2_solver_attempts(slot),
    backtracks: e.e2_solver_backtracks(slot),
  };
  e.e2_solver_free(slot);
  return rep;
}
function officialRun(budget) {
  const p = official();
  const n = e.e2_build_path(0, 16, 16, 0); // row-major
  const path = readScratch(n);
  e.e2_official();
  writeScratch(path);
  const slot = e.e2_solver_new(path.length, 1, 0, 0);
  if (slot < 0) throw new Error("solver_new failed");
  e.e2_solver_step(slot, budget);
  const status = e.e2_solver_status(slot);
  const rep = {
    status: status === 1 ? "solved" : status === 2 ? "exhausted" : "running",
    placed: e.e2_solver_placed(slot),
    best: e.e2_solver_best_placed(slot),
    nodes: e.e2_solver_nodes(slot),
    attempts: e.e2_solver_attempts(slot),
    backtracks: e.e2_solver_backtracks(slot),
  };
  e.e2_solver_free(slot);
  void p;
  return rep;
}

// --- golden parsing & checks -------------------------------------------------
const lines = readFileSync(goldenPath, "utf8").split("\n").filter((l) => l.trim() !== "");
let pass = 0, fail = 0;
function check(cond, msg) {
  if (cond) pass++;
  else { fail++; console.log("  FAIL: " + msg); }
}
const field = (tok) => Number(tok.replace(/^.*?=/, ""));
const fieldStr = (tok) => tok.replace(/^.*?=/, "");
let officialHintIdx = 0;

for (const line of lines) {
  const t = line.split(/\s+/);
  switch (t[0]) {
    case "GEN": {
      const [size, colors, seed] = [Number(t[1]), Number(t[2]), Number(t[3])];
      const p = generate(size, colors, seed);
      const got = p.pieces.map((e4) => e4.join(","));
      const expect = t.slice(4);
      let ok = got.length === expect.length;
      if (ok) for (let i = 0; i < got.length; i++) if (got[i] !== expect[i]) { ok = false; break; }
      check(ok, `generate(${size},${colors},${seed}) pieces`);
      break;
    }
    case "OFF": {
      const off = official();
      check(off.pieces.length === field(t[1]), "official piece count");
      check(off.numColors === field(t[2]), "official color count");
      check(off.hints.length === field(t[3]), "official hint count");
      break;
    }
    case "OFFHINT": {
      // OFFHINT lines appear in order; track an index.
      const off = official();
      const h = off.hints[officialHintIdx];
      check(
        h && h.pos === Number(t[1]) && h.piece === Number(t[2]) && h.rot === Number(t[3]),
        `official hint ${officialHintIdx}`,
      );
      officialHintIdx++;
      break;
    }
    case "PATH": {
      const kind = t[1];
      const w = Number(t[2]), h = Number(t[3]);
      const kindIdx = PATH_KINDS.indexOf(kind);
      const path = buildPath(kindIdx, w, h, 1);
      const expect = t.slice(4).map(Number);
      let ok = path.length === expect.length;
      if (ok) for (let i = 0; i < path.length; i++) if (path[i] !== expect[i]) { ok = false; break; }
      check(ok, `path ${kind} ${w}x${h}`);
      break;
    }
    case "SOLVE": {
      const kind = t[1];
      const eStatus = fieldStr(t[2]);
      const ePlaced = field(t[3]), eScore = field(t[4]);
      const eNodes = field(t[5]), eAttempts = field(t[6]), eBacktracks = field(t[7]);
      const kindIdx = PATH_KINDS.indexOf(kind);
      const p = generate(4, 4, 11);
      const r = solveToEnd(p, kindIdx, 4, 4);
      check(r.status === eStatus, `solve ${kind} status (${r.status} vs ${eStatus})`);
      check(r.placed === ePlaced, `solve ${kind} placed`);
      check(r.score === eScore, `solve ${kind} score (${r.score} vs ${eScore})`);
      check(r.nodes === eNodes, `solve ${kind} nodes (${r.nodes} vs ${eNodes})`);
      check(r.attempts === eAttempts, `solve ${kind} attempts (${r.attempts} vs ${eAttempts})`);
      check(r.backtracks === eBacktracks, `solve ${kind} backtracks (${r.backtracks} vs ${eBacktracks})`);
      break;
    }
    case "OFFICIALRUN": {
      const budget = field(t[1]);
      const eStatus = fieldStr(t[2]);
      const ePlaced = field(t[3]), eBest = field(t[4]);
      const eNodes = field(t[5]), eAttempts = field(t[6]), eBacktracks = field(t[7]);
      const r = officialRun(budget);
      check(r.status === eStatus, `official status (${r.status} vs ${eStatus})`);
      check(r.placed === ePlaced, `official placed (${r.placed} vs ${ePlaced})`);
      check(r.best === eBest, `official best (${r.best} vs ${eBest})`);
      check(r.nodes === eNodes, `official nodes (${r.nodes} vs ${eNodes})`);
      check(r.attempts === eAttempts, `official attempts (${r.attempts} vs ${eAttempts})`);
      check(r.backtracks === eBacktracks, `official backtracks (${r.backtracks} vs ${eBacktracks})`);
      break;
    }
    default:
      break;
  }
}

void mem;

console.log(`\n${pass} passed, ${fail} failed`);
process.exit(fail === 0 ? 0 : 1);
