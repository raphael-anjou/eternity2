--[[
  Parity test: validates the pure-Lua engine against golden outputs captured
  from the Rust engine (golden.txt, produced by `cargo run --bin golden`).

  Run:  lua spec.lua

  Every assertion compares Lua output to a value the Rust engine actually
  produced — generated puzzles (RNG parity), the official set, all path
  permutations, and full solver runs DOWN TO the exact node / attempt /
  backtrack counts. Matching the counts proves the two engines explore the
  search tree identically, not merely that they both happen to solve.
]]

-- Resolve sibling modules regardless of cwd.
local here = (debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])")) or "./"
package.path = here .. "?.lua;" .. package.path

local E = require("eternity2")
local P = require("puzzle")
local S = require("solver")

local pass, fail = 0, 0
local function check(cond, msg)
  if cond then pass = pass + 1 else fail = fail + 1; print("  FAIL: " .. msg) end
end

-- Parse golden.txt into typed records.
local golden = {}
do
  local f = assert(io.open(here .. "golden.txt", "r"), "golden.txt missing")
  for line in f:lines() do
    if line ~= "" then golden[#golden + 1] = line end
  end
  f:close()
end

local function split(s)
  local t = {}
  for tok in s:gmatch("%S+") do t[#t + 1] = tok end
  return t
end

------------------------------------------------------------------------------
print("== generator parity ==")
for _, line in ipairs(golden) do
  local t = split(line)
  if t[1] == "GEN" then
    local size, colors, seed = tonumber(t[2]), tonumber(t[3]), tonumber(t[4])
    local p = P.generate(size, colors, seed)
    local got = {}
    for _, e in ipairs(p.pieces) do
      got[#got + 1] = string.format("%d,%d,%d,%d", e[1], e[2], e[3], e[4])
    end
    -- Reconstruct expected from the remaining tokens.
    local expect = {}
    for i = 5, #t do expect[#expect + 1] = t[i] end
    local ok = #got == #expect
    if ok then for i = 1, #got do if got[i] ~= expect[i] then ok = false; break end end end
    check(ok, string.format("generate(%d,%d,%d) pieces match", size, colors, seed))
  end
end

------------------------------------------------------------------------------
print("== official set parity ==")
do
  local off = P.official_puzzle()
  for _, line in ipairs(golden) do
    local t = split(line)
    if t[1] == "OFF" then
      check(#off.pieces == tonumber((t[2]:gsub("pieces=", ""))), "official piece count")
      check(off.numColors == tonumber((t[3]:gsub("colors=", ""))), "official color count")
      check(#off.hints == tonumber((t[4]:gsub("hints=", ""))), "official hint count")
    end
  end
  -- Hints, in order.
  local hi = 0
  for _, line in ipairs(golden) do
    local t = split(line)
    if t[1] == "OFFHINT" then
      hi = hi + 1
      local hnt = off.hints[hi]
      check(hnt ~= nil
        and hnt.pos == tonumber(t[2])
        and hnt.piece == tonumber(t[3])
        and hnt.rot == tonumber(t[4]),
        "official hint " .. hi)
    end
  end
end

------------------------------------------------------------------------------
print("== path parity ==")
for _, line in ipairs(golden) do
  local t = split(line)
  if t[1] == "PATH" then
    local kind, w, h = t[2], tonumber(t[3]), tonumber(t[4])
    local path = P.build_path(kind, w, h, 1)
    local ok = path ~= nil and #path == (#t - 4)
    if ok then
      for i = 0, #path - 1 do
        if path[i + 1] ~= tonumber(t[5 + i]) then ok = false; break end
      end
    end
    check(ok, string.format("path %s %dx%d", kind, w, h))
  end
end

------------------------------------------------------------------------------
print("== solver parity (generated 4x4 seed 11, all paths) ==")
for _, line in ipairs(golden) do
  local t = split(line)
  if t[1] == "SOLVE" then
    local kind = t[2]
    local function field(tok) return tonumber((tok:gsub("^.-=", ""))) end
    local function fieldStr(tok) return (tok:gsub("^.-=", "")) end
    local e_status = fieldStr(t[3])
    local e_placed, e_score = field(t[4]), field(t[5])
    local e_nodes, e_attempts, e_backtracks = field(t[6]), field(t[7]), field(t[8])

    local p = P.generate(4, 4, 11)
    local path = P.build_path(kind, 4, 4, 0)
    local sv = assert(S.new(p, path, { use_hints = true, shuffle_pieces = false, seed = 0 }))
    local r
    repeat r = sv:step(5000000) until r.status ~= "running"
    local sc = S.score_board(p, sv:board())
    check(r.status == e_status, "solve " .. kind .. " status (" .. r.status .. " vs " .. e_status .. ")")
    check(r.placed == e_placed, "solve " .. kind .. " placed")
    check(sc == e_score, "solve " .. kind .. " score")
    check(r.nodes == e_nodes, "solve " .. kind .. " nodes (" .. r.nodes .. " vs " .. e_nodes .. ")")
    check(r.attempts == e_attempts, "solve " .. kind .. " attempts (" .. r.attempts .. " vs " .. e_attempts .. ")")
    check(r.backtracks == e_backtracks, "solve " .. kind .. " backtracks")
  end
end

------------------------------------------------------------------------------
print("== official partial-run parity (fixed step budget) ==")
for _, line in ipairs(golden) do
  local t = split(line)
  if t[1] == "OFFICIALRUN" then
    local function field(tok) return tonumber((tok:gsub("^.-=", ""))) end
    local function fieldStr(tok) return (tok:gsub("^.-=", "")) end
    local budget = field(t[2])
    local e_status, e_placed, e_best = fieldStr(t[3]), field(t[4]), field(t[5])
    local e_nodes, e_attempts, e_backtracks = field(t[6]), field(t[7]), field(t[8])

    local p = P.official_puzzle()
    local path = P.build_path("row-major", 16, 16, 0)
    local sv = assert(S.new(p, path, { use_hints = true, shuffle_pieces = false, seed = 0 }))
    local r = sv:step(budget)
    check(r.status == e_status, "official status")
    check(r.placed == e_placed, "official placed (" .. r.placed .. " vs " .. e_placed .. ")")
    check(r.bestPlaced == e_best, "official best (" .. r.bestPlaced .. " vs " .. e_best .. ")")
    check(r.nodes == e_nodes, "official nodes (" .. r.nodes .. " vs " .. e_nodes .. ")")
    check(r.attempts == e_attempts, "official attempts (" .. r.attempts .. " vs " .. e_attempts .. ")")
    check(r.backtracks == e_backtracks, "official backtracks")
  end
end

------------------------------------------------------------------------------
print(string.format("\n%d passed, %d failed", pass, fail))
os.exit(fail == 0 and 0 or 1)
