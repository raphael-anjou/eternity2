--[[
  Tiny command-line demo of the Lua engine. No arguments: generates a small
  solvable puzzle and solves it; then loads the official set and runs a short
  bounded search to show progress. This is just a human-facing smoke test —
  the real correctness check is `lua spec.lua`.

  Usage:  lua demo.lua [size] [seed]
]]

local here = (debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])")) or "./"
package.path = here .. "?.lua;" .. package.path

local E = require("eternity2")
local P = require("puzzle")
local S = require("solver")

local size = tonumber(arg[1]) or 6
local seed = tonumber(arg[2]) or 42

print(string.format("Eternity II — pure Lua engine (Lua %s)", _VERSION:match("%d+%.%d+")))
print(string.rep("-", 52))

-- 1. Generate and fully solve a small puzzle.
local colors = P.max_colors(size)
local p = P.generate(size, colors, seed)
local path = P.build_path("double-snake", size, size, 0)
local sv = assert(S.new(p, path))
local r
repeat r = sv:step(2000000) until r.status ~= "running"
print(string.format("generated %dx%d (c=%d, seed=%d):", size, size, colors, seed))
print(string.format("  status=%s  placed=%d/%d  score=%d/%d",
  r.status, r.placed, size * size, S.score_board(p, sv:board()), E.max_score(size, size)))
print(string.format("  nodes=%d  attempts=%d  backtracks=%d", r.nodes, r.attempts, r.backtracks))

-- 2. Official set: short bounded search (it does not finish — nothing does).
local off = P.official_puzzle()
local opath = P.build_path("row-major", 16, 16, 0)
local osv = assert(S.new(off, opath))
local orep = osv:step(50000)
print("official Eternity II (16x16, 50,000-step probe):")
print(string.format("  status=%s  placed=%d  best_placed=%d  attempts=%d",
  orep.status, orep.placed, orep.bestPlaced, orep.attempts))
print(string.rep("-", 52))
print("OK")
