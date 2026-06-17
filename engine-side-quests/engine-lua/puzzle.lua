--[[
  Puzzle sources: the seeded generator (generator.rs), the official Eternity II
  set (official.rs), and the built-in cell-visit orders / "paths" (paths.rs).

  A Puzzle is a plain table matching the Rust serde output (camelCase keys):
    { name=string, width=int, height=int, numColors=int,
      pieces = { {u,r,d,l}, ... },        -- 0-based piece i at pieces[i+1]
      hints  = { {pos=int, piece=int, rot=int}, ... } }
  Edges are URDL in rotation 0.
]]

local E = require("eternity2")

local P = {}

------------------------------------------------------------------------------
-- Generator (generator.rs)
------------------------------------------------------------------------------

-- Interior edge count of an n×n board: 2·n·(n−1).
function P.interior_edge_count(size)
  return 2 * size * (size - 1)
end

-- Largest usable color count for a size (≤ 22 renderable motifs).
function P.max_colors(size)
  local c = P.interior_edge_count(size)
  return (c < 22) and c or 22
end

-- Pieces in solution order/orientation: piece i belongs at cell i, rot 0.
function P.generate_solved(size, colors, seed)
  assert(size >= 2, "size must be >= 2")
  local maxc = P.max_colors(size)
  if colors < 1 then colors = 1 elseif colors > maxc then colors = maxc end
  local s = size
  local n_edges = P.interior_edge_count(size)
  local rng = E.XorShift.new(seed)

  -- One color per interior adjacency, every color present at least once.
  local palette = E.gen0(n_edges, function(i)
    if i < colors then
      return i + 1
    else
      return rng:below(colors) + 1
    end
  end)
  rng:shuffle(palette)

  -- vert[y][x] = color between (x,y)-(x+1,y) ; horiz[y][x] between (x,y)-(x,y+1).
  -- palette is split: first s*(s-1) verticals, rest horizontals. Access 0-based.
  local function vert(x, y) return palette[(y * (s - 1) + x) + 1] end
  local function horiz(x, y) return palette[(s * (s - 1) + y * s + x) + 1] end

  local pieces = {}
  for y = 0, s - 1 do
    for x = 0, s - 1 do
      local up    = (y == 0)     and E.BORDER or horiz(x, y - 1)
      local down  = (y == s - 1) and E.BORDER or horiz(x, y)
      local left  = (x == 0)     and E.BORDER or vert(x - 1, y)
      local right = (x == s - 1) and E.BORDER or vert(x, y)
      pieces[#pieces + 1] = { up, right, down, left }
    end
  end

  return {
    name = string.format("generated_%dx%d_c%d_s%d", size, size, colors, seed),
    width = size, height = size, numColors = colors,
    pieces = pieces, hints = {},
  }
end

-- Solvable puzzle: build the solved board, then shuffle piece order and
-- rotate each piece randomly (generator.rs `generate`).
function P.generate(size, colors, seed)
  local puzzle = P.generate_solved(size, colors, seed)
  local rng = E.XorShift.new(seed ~ 0xA5A5A5A5)
  rng:shuffle(puzzle.pieces)
  for i = 1, #puzzle.pieces do
    puzzle.pieces[i] = E.rotated(puzzle.pieces[i], rng:below(4))
  end
  return puzzle
end

------------------------------------------------------------------------------
-- Official set (official.rs). CSV: line 1 = size; then per piece
-- `top,right,bottom,left,x,y,rotation`, colors as 16-bit binary words
-- (65535 = grey border = 0). Non-zero (x,y,rot) marks a clue piece.
------------------------------------------------------------------------------

local function parse_color_word(s)
  local v = tonumber((s:gsub("%s", "")), 2)
  assert(v, "invalid binary color word in official CSV")
  if v == 65535 then return E.BORDER else return v end
end

-- path defaults to ../../engine/data relative to this module's directory
-- (engine-side-quests/engine-lua/ -> ../../engine/data).
function P.official_puzzle(csv_path)
  csv_path = csv_path or (P._dir() .. "../../engine/data/official_eternity2.csv")
  local f = assert(io.open(csv_path, "r"), "cannot open official CSV: " .. csv_path)
  local lines = {}
  for line in f:lines() do
    if line:gsub("%s", "") ~= "" then lines[#lines + 1] = line end
  end
  f:close()

  local size = tonumber(lines[1])
  local pieces, hints = {}, {}
  local max_color = 0
  for id = 0, #lines - 2 do
    local line = lines[id + 2]
    local cols = {}
    for tok in line:gmatch("[^,]+") do cols[#cols + 1] = tok end
    local edges = {
      parse_color_word(cols[1]), parse_color_word(cols[2]),
      parse_color_word(cols[3]), parse_color_word(cols[4]),
    }
    for i = 1, 4 do if edges[i] > max_color then max_color = edges[i] end end
    pieces[#pieces + 1] = edges
    if #cols >= 7 then
      local x = tonumber(cols[5]) or 0
      local y = tonumber(cols[6]) or 0
      local rot = tonumber(cols[7]) or 0
      if x ~= 0 or y ~= 0 or rot ~= 0 then
        hints[#hints + 1] = { pos = y * size + x, piece = id, rot = rot }
      end
    end
  end

  return {
    name = "official_eternity2", width = size, height = size,
    numColors = max_color, pieces = pieces, hints = hints,
  }
end

-- Directory of this source file (so the default CSV path resolves regardless
-- of the caller's cwd).
function P._dir()
  local src = debug.getinfo(1, "S").source:sub(2)
  return src:match("(.*[/\\])") or "./"
end

------------------------------------------------------------------------------
-- Paths (paths.rs). A path is the cell-visit order: a permutation of
-- 0..w*h-1 (row-major cell indices) returned as a 0-based-value array.
------------------------------------------------------------------------------

P.PATH_KINDS = {
  "row-major", "snake", "column-major", "spiral-in", "spiral-out",
  "diagonal", "border-first", "double-snake", "random",
}

-- Returns the path array (sequence of 0-based cell indices) or nil for an
-- unknown kind.
function P.build_path(kind, width, height, seed)
  local w, h = width, height
  local function idx(x, y) return y * w + x end
  local out = {}
  local function push(v) out[#out + 1] = v end

  if kind == "row-major" then
    for y = 0, h - 1 do for x = 0, w - 1 do push(idx(x, y)) end end

  elseif kind == "snake" then
    for y = 0, h - 1 do
      if y % 2 == 0 then
        for x = 0, w - 1 do push(idx(x, y)) end
      else
        for x = w - 1, 0, -1 do push(idx(x, y)) end
      end
    end

  elseif kind == "column-major" then
    for x = 0, w - 1 do for y = 0, h - 1 do push(idx(x, y)) end end

  elseif kind == "spiral-in" or kind == "spiral-out" then
    local x0, y0, x1, y1 = 0, 0, w, h
    while x0 < x1 and y0 < y1 do
      for x = x0, x1 - 1 do push(idx(x, y0)) end
      for y = y0 + 1, y1 - 1 do push(idx(x1 - 1, y)) end
      if y1 > y0 + 1 then
        for x = x1 - 2, x0, -1 do push(idx(x, y1 - 1)) end
      end
      if x1 > x0 + 1 then
        for y = y1 - 2, y0 + 1, -1 do push(idx(x0, y)) end
      end
      x0, y0, x1, y1 = x0 + 1, y0 + 1, x1 - 1, y1 - 1
    end
    if kind == "spiral-out" then
      local rev = {}
      for i = #out, 1, -1 do rev[#rev + 1] = out[i] end
      out = rev
    end

  elseif kind == "diagonal" then
    for d = 0, (w + h - 2) do
      for y = 0, h - 1 do
        if d >= y and d - y < w then push(idx(d - y, y)) end
      end
    end

  elseif kind == "border-first" then
    for x = 0, w - 1 do push(idx(x, 0)) end
    for y = 1, h - 1 do push(idx(w - 1, y)) end
    if h > 1 then
      for x = w - 2, 0, -1 do push(idx(x, h - 1)) end
    end
    if w > 1 then
      for y = h - 2, 1, -1 do push(idx(0, y)) end
    end
    for y = 1, h - 2 do for x = 1, w - 2 do push(idx(x, y)) end end

  elseif kind == "double-snake" then
    local y = 0
    while y < h do
      local rows = (y + 1 < h) and { y, y + 1 } or { y }
      if math.floor(y / 2) % 2 == 0 then
        for x = 0, w - 1 do for _, r in ipairs(rows) do push(idx(x, r)) end end
      else
        for x = w - 1, 0, -1 do for _, r in ipairs(rows) do push(idx(x, r)) end end
      end
      y = y + 2
    end

  elseif kind == "random" then
    for c = 0, w * h - 1 do push(c) end
    E.XorShift.new(seed):shuffle(out)

  else
    return nil
  end

  assert(#out == w * h, "path length mismatch for " .. kind)
  return out
end

return P
