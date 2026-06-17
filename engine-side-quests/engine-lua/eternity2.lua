--[[
  Eternity II engine — pure Lua port of the Rust crate in ../engine/src.

  Why this file exists: the Rust core is fast and correct, but Rust is a steep
  language to pick up for maintenance. This is the SAME engine — same search,
  same scoring, same generator, same official set, same path orders — written
  in plain Lua, the simplest embeddable language there is. It has no
  dependencies, no build step, and no toolchain: `lua spec.lua` runs it, and
  it loads in a browser through Fengari (lua.vm.js) behind the exact same
  boundary the WASM build uses. If you can read this file, you can maintain
  the engine.

  Conventions kept identical to the Rust/JS contract so outputs are byte-for-
  byte comparable:
    * Edge order is URDL (up, right, down, left), bucas convention.
    * Color 0 is the grey border; interior colors are 1..22.
    * A board is `cell -> piece*4 + rotation`, or -1 for an empty cell, where
      `cell` and `piece` are 0-based indices (so the JSON matches the Rust
      serde output exactly).
    * Rotation r = clockwise quarter-turns.

  Lua is 1-indexed, the contract is 0-indexed. To avoid a swamp of off-by-one
  bugs the rule in this file is: DOMAIN indices (cells, pieces, colors,
  rotations, board encodings) are 0-based integers stored as VALUES; Lua
  tables that hold them are accessed with an explicit +1 only at the table
  boundary, via the small helpers below. Read those first.
]]

local M = {}

M.BORDER = 0

------------------------------------------------------------------------------
-- 0-based array helpers. `arr` is a Lua sequence (1..n); `i` is a 0-based
-- index. Keeping this in one place is what makes the rest of the port read
-- like the Rust.
------------------------------------------------------------------------------

-- Build a 0-based-friendly array from a generator f(i) for i in 0..n-1.
function M.gen0(n, f)
  local t = {}
  for i = 0, n - 1 do
    t[i + 1] = f(i)
  end
  return t
end

------------------------------------------------------------------------------
-- Edge rotation (mirrors types.rs `rotated`).
-- e is a 4-array {up, right, down, left}; r clockwise quarter-turns.
-- new[i] = old[(i + 4 - r) % 4]. Written out per-rotation so it is obviously
-- total and allocation-light.
------------------------------------------------------------------------------

function M.rotated(e, r)
  local u, ri, d, l = e[1], e[2], e[3], e[4]
  local k = r % 4
  if k == 1 then
    return { l, u, ri, d }
  elseif k == 2 then
    return { d, l, u, ri }
  elseif k == 3 then
    return { ri, d, l, u }
  else
    return { u, ri, d, l }
  end
end

-- Lexicographically minimal cyclic rotation: the bucas canonical form.
function M.canonical(e)
  local best = e
  for r = 1, 3 do
    local c = M.rotated(e, r)
    -- 4-tuple lexicographic compare
    for i = 1, 4 do
      if c[i] < best[i] then
        best = c
        break
      elseif c[i] > best[i] then
        break
      end
    end
  end
  return best
end

-- Max matched-edge score for a w×h board: 2wh - w - h (480 for 16×16).
function M.max_score(w, h)
  return 2 * w * h - w - h
end

------------------------------------------------------------------------------
-- XorShift RNG — identical algorithm and seeding to generator.rs so that
-- `generate(size, colors, seed)` produces the SAME puzzle as the Rust engine
-- for every (size, colors, seed). This is the load-bearing parity guarantee.
--
-- Rust uses u64 wrapping arithmetic; Lua 5.3+ integers are 64-bit and wrap on
-- overflow with the same two's-complement semantics, and the bitwise
-- operators (<<, >>, ~, &, |) match. `>>` in Lua is logical (unsigned), which
-- is what we want for the `x >> k` mixing steps.
------------------------------------------------------------------------------

local XorShift = {}
XorShift.__index = XorShift
M.XorShift = XorShift

function XorShift.new(seed)
  -- seed is u32; splitmix64 once to avoid the all-zero state, exactly as Rust.
  local z = (seed & 0xFFFFFFFF) + 0x9E3779B97F4A7C15
  z = (z ~ (z >> 30)) * 0xBF58476D1CE4E5B9
  z = (z ~ (z >> 27)) * 0x94D049BB133111EB
  local state = (z ~ (z >> 31)) | 1
  return setmetatable({ state = state }, XorShift)
end

function XorShift:next_u32()
  local x = self.state
  x = x ~ ((x << 13) & 0xFFFFFFFFFFFFFFFF)
  x = x ~ (x >> 7)
  x = x ~ ((x << 17) & 0xFFFFFFFFFFFFFFFF)
  self.state = x & 0xFFFFFFFFFFFFFFFF
  -- (x >> 32) as u32
  return (self.state >> 32) & 0xFFFFFFFF
end

-- Uniform in [0, n).
function XorShift:below(n)
  return self:next_u32() % n
end

-- In-place Fisher–Yates over a 0-based-length-n array (Lua sequence 1..n),
-- matching the Rust `for i in (1..len).rev()` order so shuffles agree.
function XorShift:shuffle(arr)
  local len = #arr
  for i = len - 1, 1, -1 do
    local j = self:below(i + 1) -- 0..i
    arr[i + 1], arr[j + 1] = arr[j + 1], arr[i + 1]
  end
end

return M
