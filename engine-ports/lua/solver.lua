--[[
  Step-able backtracking DFS over an arbitrary cell-visit order — the pure-Lua
  twin of solver.rs. Same explicit machine: call `step(budget)` repeatedly and
  read the board between calls (exactly what the animated browser UI needs).
  One "step" is one placement OR one backtrack.

  Board encoding matches the contract: board[cell] (0-based) = piece*4 + rot, or
  -1 when empty. The report fields and statuses are identical to the Rust
  Report so a UI can swap engines without noticing.

  No clocks here; wall-time is the caller's business.
]]

local E = require("eternity2")
local P = require("puzzle")

local S = {}
S.__index = S

local U32_MAX = 0xFFFFFFFF -- sentinel for "no piece placed in this frame"

-- Construct a solver. `path` is a 0-based-value array of cell indices.
-- opts: { use_hints=true, shuffle_pieces=false, seed=0 }.
-- Returns solver, err (err is a string on failure).
function S.new(puzzle, path, opts)
  opts = opts or {}
  local use_hints = opts.use_hints
  if use_hints == nil then use_hints = true end
  local shuffle_pieces = opts.shuffle_pieces or false
  local seed = opts.seed or 0

  local w, h = puzzle.width, puzzle.height
  local n_cells = w * h
  local n_pieces = #puzzle.pieces
  if n_pieces ~= n_cells then
    return nil, string.format("puzzle has %d pieces for %d cells", n_pieces, n_cells)
  end
  if #path ~= n_cells then
    return nil, string.format("path covers %d of %d cells", #path, n_cells)
  end
  do
    local seen = {}
    for _, c in ipairs(path) do
      if c >= n_cells or seen[c] then
        return nil, "path is not a permutation of the cells"
      end
      seen[c] = true
    end
  end

  -- Rotated-edge table for every (piece, rotation), piece-id major.
  -- table0[piece*4 + r] (0-based) = {edges}. `distinct0` marks rotations that
  -- duplicate a lower rotation of the same piece.
  local table0, distinct0 = {}, {}
  for id = 0, n_pieces - 1 do
    local base = puzzle.pieces[id + 1]
    for r = 0, 3 do
      local re = E.rotated(base, r)
      local row = id * 4 + r
      table0[row] = re
      distinct0[row] = true
      for prev = 0, r - 1 do
        local pe = table0[id * 4 + prev]
        if pe[1] == re[1] and pe[2] == re[2] and pe[3] == re[3] and pe[4] == re[4] then
          distinct0[row] = false
          break
        end
      end
    end
  end

  -- Piece iteration order (optionally seed-shuffled).
  local piece_order = E.gen0(n_pieces, function(i) return i end)
  if shuffle_pieces then
    E.XorShift.new(seed):shuffle(piece_order)
  end

  -- board0[cell] 0-based, -1 empty. used[piece]=true once placed.
  local board0 = {}
  for c = 0, n_cells - 1 do board0[c] = -1 end
  local used = {}
  local hint_count = 0
  if use_hints then
    for _, hnt in ipairs(puzzle.hints) do
      local pos = hnt.pos
      if pos >= n_cells or hnt.piece >= n_pieces or used[hnt.piece] then
        return nil, string.format("invalid hint at position %d", pos)
      end
      board0[pos] = hnt.piece * 4 + (hnt.rot % 4)
      used[hnt.piece] = true
      hint_count = hint_count + 1
    end
  end

  -- One frame per non-hint cell, in visit order.
  local frames = {}
  for _, c in ipairs(path) do
    if board0[c] == -1 then
      frames[#frames + 1] = { pos = c, cursor = 0, placed = U32_MAX }
    end
  end

  local best_board = {}
  for c = 0, n_cells - 1 do best_board[c] = board0[c] end

  return setmetatable({
    width = w, height = h, n_pieces = n_pieces,
    table0 = table0, distinct0 = distinct0, piece_order = piece_order,
    board0 = board0, used = used, frames = frames,
    depth = 0, hint_count = hint_count,
    status = "running",
    nodes = 0, attempts = 0, backtracks = 0,
    best_placed = hint_count, best_board = best_board,
  }, S)
end

-- Would table-row `e` fit at cell `pos`? (rim/interior + placed neighbors)
function S:fits(pos, e)
  local w, h = self.width, self.height
  local x, y = pos % w, pos // w
  local top, right, bottom, left = e[1], e[2], e[3], e[4]
  local B = E.BORDER
  local b = self.board0

  if (y == 0)     ~= (top == B)    then return false end
  if (y == h - 1) ~= (bottom == B) then return false end
  if (x == 0)     ~= (left == B)   then return false end
  if (x == w - 1) ~= (right == B)  then return false end

  if y > 0 then
    local n = b[pos - w]
    if n >= 0 and self.table0[n][3] ~= top then return false end
  end
  if y < h - 1 then
    local n = b[pos + w]
    if n >= 0 and self.table0[n][1] ~= bottom then return false end
  end
  if x > 0 then
    local n = b[pos - 1]
    if n >= 0 and self.table0[n][2] ~= left then return false end
  end
  if x < w - 1 then
    local n = b[pos + 1]
    if n >= 0 and self.table0[n][4] ~= right then return false end
  end
  return true
end

function S:placed()
  return self.hint_count + self.depth
end

-- Run up to `budget` steps (placements + backtracks); returns a report table.
function S:step(budget)
  local remaining = budget
  while remaining > 0 and self.status == "running" do
    remaining = remaining - 1

    if self.depth == #self.frames then
      self.status = "solved"
      self.best_placed = self:placed()
      for c = 0, self.width * self.height - 1 do self.best_board[c] = self.board0[c] end
      break
    end

    local frame = self.frames[self.depth + 1]
    local pos = frame.pos
    local cursor = frame.cursor
    local limit = self.n_pieces * 4
    local placed_row = U32_MAX

    while cursor < limit do
      local oi = cursor // 4
      local r = cursor % 4
      cursor = cursor + 1
      local pid = self.piece_order[oi + 1]
      if self.used[pid] then
        cursor = (cursor + 3) & ~3 -- skip this piece's remaining rotations
      else
        local row = pid * 4 + r
        if self.distinct0[row] then
          self.attempts = self.attempts + 1
          if self:fits(pos, self.table0[row]) then
            placed_row = row
            break
          end
        end
      end
    end

    if placed_row ~= U32_MAX then
      local pid = placed_row // 4
      self.board0[pos] = placed_row
      self.used[pid] = true
      frame.cursor = cursor
      frame.placed = placed_row
      self.depth = self.depth + 1
      self.nodes = self.nodes + 1
      local placed = self:placed()
      if placed > self.best_placed then
        self.best_placed = placed
        for c = 0, self.width * self.height - 1 do self.best_board[c] = self.board0[c] end
      end
    else
      frame.cursor = 0
      if self.depth == 0 then
        self.status = "exhausted"
        break
      end
      self.depth = self.depth - 1
      local prev = self.frames[self.depth + 1]
      local row = prev.placed
      prev.placed = U32_MAX
      self.board0[prev.pos] = -1
      self.used[row // 4] = nil
      self.backtracks = self.backtracks + 1
    end
  end
  return self:report()
end

function S:report()
  return {
    status = self.status,
    nodes = self.nodes,
    attempts = self.attempts,
    backtracks = self.backtracks,
    placed = self:placed(),
    bestPlaced = self.best_placed,
  }
end

-- Current board as a 0-based array copy (cell -> piece*4+rot | -1).
function S:board()
  local out = {}
  for c = 0, self.width * self.height - 1 do out[c] = self.board0[c] end
  return out
end

function S:best_board_copy()
  local out = {}
  for c = 0, self.width * self.height - 1 do out[c] = self.best_board[c] end
  return out
end

-- Matched interior edges of a board (board[cell] 0-based = piece*4+rot | -1).
-- Grey-grey interior contacts do NOT score (bucas convention).
function S.score_board(puzzle, board0)
  local w, h = puzzle.width, puzzle.height
  local B = E.BORDER
  local function edges_of(row)
    if row == nil or row < 0 then return nil end
    local pid = row // 4
    local r = row % 4
    local base = puzzle.pieces[pid + 1]
    if base == nil then return nil end
    return E.rotated(base, r)
  end
  local score = 0
  for y = 0, h - 1 do
    for x = 0, w - 1 do
      local here = edges_of(board0[y * w + x])
      if x + 1 < w then
        local b = edges_of(board0[y * w + x + 1])
        if here and b and here[2] == b[4] and here[2] ~= B then score = score + 1 end
      end
      if y + 1 < h then
        local b = edges_of(board0[(y + 1) * w + x])
        if here and b and here[3] == b[1] and here[3] ~= B then score = score + 1 end
      end
    end
  end
  return score
end

return S
