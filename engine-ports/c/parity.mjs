// Parity check: drives engine.wasm under Node (Node 22 has built-in
// WebAssembly) through the C ABI and checks every line of the Rust-produced
// golden.txt (engine-ports/lua/golden.txt). Mirrors the Lua
// spec.lua assertions: GEN / OFF / OFFHINT / PATH / SOLVE / OFFICIALRUN, down
// to the exact node / attempt / backtrack counts.
//
// Usage:  node parity.mjs
// Exits non-zero on any mismatch.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const goldenPath = join(here, "..", "lua", "golden.txt");
const wasmPath = join(here, "engine.wasm");

// --- Instantiate the wasm module. ------------------------------------------
const bytes = readFileSync(wasmPath);
const { instance } = await WebAssembly.instantiate(bytes, {});
const e = instance.exports;
const mem = () => new Uint8Array(e.memory.buffer);
const memU16 = () => new Uint16Array(e.memory.buffer);
const memI32 = () => new Int32Array(e.memory.buffer);

// --- Helpers that mirror glue.ts but stay self-contained. ------------------
function readPieces() {
  const n = e.e2_puzzle_width() * e.e2_puzzle_height();
  const ptr = e.e2_pieces_ptr();
  const b = mem();
  const out = [];
  for (let i = 0; i < n; i++) {
    const o = ptr + i * 4;
    out.push([b[o], b[o + 1], b[o + 2], b[o + 3]]);
  }
  return out;
}
function readPath(len) {
  const ptr = e.e2_path_ptr();
  const v = memU16();
  const base = ptr / 2;
  return Array.from(v.slice(base, base + len));
}
function pathKindIndex(name) {
  const n = e.e2_path_kind_count();
  const b = mem();
  for (let i = 0; i < n; i++) {
    let p = e.e2_path_kind_name(i);
    let s = "";
    for (;;) { const c = b[p]; if (!c) break; s += String.fromCharCode(c); p++; }
    if (s === name) return i;
  }
  return -1;
}
const STATUS = ["running", "solved", "exhausted"];

// --- Load golden. ----------------------------------------------------------
const lines = readFileSync(goldenPath, "utf8").split("\n").filter((l) => l.trim() !== "");

let pass = 0, fail = 0;
const check = (cond, msg) => {
  if (cond) pass++;
  else { fail++; console.log("  FAIL: " + msg); }
};
const field = (tok) => Number(tok.replace(/^.*?=/, ""));
const fieldStr = (tok) => tok.replace(/^.*?=/, "");

for (const line of lines) {
  const t = line.split(/\s+/);
  switch (t[0]) {
    case "GEN": {
      const [size, colors, seed] = [Number(t[1]), Number(t[2]), Number(t[3])];
      e.e2_generate(size, colors, seed);
      const got = readPieces().map((p) => p.join(","));
      const expect = t.slice(4);
      let ok = got.length === expect.length;
      if (ok) for (let i = 0; i < got.length; i++) if (got[i] !== expect[i]) { ok = false; break; }
      check(ok, `generate(${size},${colors},${seed}) pieces`);
      break;
    }
    case "FRAMEDGEN": {
      const [size, colors, seed] = [Number(t[1]), Number(t[2]), Number(t[3])];
      e.e2_generate_framed(size, colors, seed, 1);
      const got = readPieces().map((p) => p.join(","));
      const expect = t.slice(4);
      let ok = got.length === expect.length;
      if (ok) for (let i = 0; i < got.length; i++) if (got[i] !== expect[i]) { ok = false; break; }
      check(ok, `generateFramed(${size},${colors},${seed},true) pieces`);
      break;
    }
    case "OFF": {
      e.e2_official();
      check(e.e2_puzzle_width() * e.e2_puzzle_height() === field(t[1]), "official piece count");
      check(e.e2_puzzle_colors() === field(t[2]), "official color count");
      check(e.e2_puzzle_hints() === field(t[3]), "official hint count");
      break;
    }
    case "OFFHINT": {
      // OFFHINT lines come in order; map by matching pos.
      e.e2_official();
      const nh = e.e2_puzzle_hints();
      const pos = Number(t[1]), piece = Number(t[2]), rot = Number(t[3]);
      let found = false;
      for (let i = 0; i < nh; i++) {
        if (e.e2_hint_pos(i) === pos) {
          found = e.e2_hint_piece(i) === piece && e.e2_hint_rot(i) === rot;
          break;
        }
      }
      check(found, `official hint pos=${pos}`);
      break;
    }
    case "PATH": {
      const kind = t[1], w = Number(t[2]), h = Number(t[3]);
      const ki = pathKindIndex(kind);
      // golden builds PATH lines with seed 1 (matches spec.lua).
      const len = e.e2_build_path(ki, w, h, 1);
      const got = readPath(len);
      const expect = t.slice(4).map(Number);
      let ok = got.length === expect.length;
      if (ok) for (let i = 0; i < got.length; i++) if (got[i] !== expect[i]) { ok = false; break; }
      check(ok, `path ${kind} ${w}x${h}`);
      break;
    }
    case "SOLVE": {
      const kind = t[1];
      const eStatus = fieldStr(t[2]);
      const ePlaced = field(t[3]), eScore = field(t[4]);
      const eNodes = field(t[5]), eAttempts = field(t[6]), eBacktracks = field(t[7]);
      // generated 4x4 seed 11, path seed 0, hints on
      e.e2_generate(4, 4, 11);
      const ki = pathKindIndex(kind);
      const len = e.e2_build_path(ki, 4, 4, 0);
      const rc = e.e2_solver_new(len, 1, 0, 0);
      check(rc === 0, `solve ${kind} construct`);
      let status;
      do { e.e2_solver_step(5000000); status = STATUS[e.e2_solver_status()]; } while (status === "running");
      const sc = e.e2_solver_score();
      check(status === eStatus, `solve ${kind} status (${status} vs ${eStatus})`);
      check(e.e2_solver_placed() === ePlaced, `solve ${kind} placed`);
      check(sc === eScore, `solve ${kind} score (${sc} vs ${eScore})`);
      check(e.e2_solver_nodes() === eNodes, `solve ${kind} nodes (${e.e2_solver_nodes()} vs ${eNodes})`);
      check(e.e2_solver_attempts() === eAttempts, `solve ${kind} attempts (${e.e2_solver_attempts()} vs ${eAttempts})`);
      check(e.e2_solver_backtracks() === eBacktracks, `solve ${kind} backtracks`);
      break;
    }
    case "OFFICIALRUN": {
      const budget = field(t[1]);
      const eStatus = fieldStr(t[2]);
      const ePlaced = field(t[3]), eBest = field(t[4]);
      const eNodes = field(t[5]), eAttempts = field(t[6]), eBacktracks = field(t[7]);
      e.e2_official();
      const ki = pathKindIndex("row-major");
      const len = e.e2_build_path(ki, 16, 16, 0);
      e.e2_solver_new(len, 1, 0, 0);
      e.e2_solver_step(budget);
      check(STATUS[e.e2_solver_status()] === eStatus, "official status");
      check(e.e2_solver_placed() === ePlaced, `official placed (${e.e2_solver_placed()} vs ${ePlaced})`);
      check(e.e2_solver_best_placed() === eBest, `official best (${e.e2_solver_best_placed()} vs ${eBest})`);
      check(e.e2_solver_nodes() === eNodes, `official nodes (${e.e2_solver_nodes()} vs ${eNodes})`);
      check(e.e2_solver_attempts() === eAttempts, `official attempts (${e.e2_solver_attempts()} vs ${eAttempts})`);
      check(e.e2_solver_backtracks() === eBacktracks, "official backtracks");
      break;
    }
    default:
      break;
  }
}

console.log(`\n${pass} passed, ${fail} failed`);
process.exit(fail === 0 ? 0 : 1);
