// An editable board grid for the format converter: every cell's four edges are
// editable inputs, validated per-edge, with a red message on the offending cell.
// The board renders live from the edited cells and drives every output on the
// page, so a hand-built or hand-corrected board round-trips through all formats.
//
// Editing model: the board is a flat list of cells (URDL edge quads, null =
// empty). Clicking a cell in the preview selects it; the side panel then edits
// that cell's four edges. An edge is valid when it is a whole number in
// [0, maxColor]; anything else flags the cell and names the fault. `a` (0) is
// the grey border, and a cell whose four edges are all 0 is treated as empty.

import { useMemo, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import type { Edges } from "@/lib/bucas";

// Letters decode a..z, so 25 is the widest colour a board_edges string can
// carry. The official puzzle uses 0..21; we validate against the format ceiling,
// not the official palette, so the converter stays puzzle-agnostic.
export const MAX_COLOR = 25;

const DIR_KEYS = ["u", "r", "d", "l"] as const;

export type CellFault = { pos: number; message: string };

// A raw editable edge is the string the user typed; we keep the text so a
// half-typed or invalid value is preserved in the field rather than snapping.
type RawCell = [string, string, string, string];

function edgesToRaw(cell: Edges | null): RawCell {
  if (!cell) return ["0", "0", "0", "0"];
  return [String(cell[0]), String(cell[1]), String(cell[2]), String(cell[3])];
}

// Parse one edge field. Returns the number, or null with a reason if invalid.
function parseEdge(raw: string): { value: number | null; reason: "empty" | "nan" | "range" | null } {
  const s = raw.trim();
  if (s === "") return { value: null, reason: "empty" };
  if (!/^\d+$/.test(s)) return { value: null, reason: "nan" };
  const n = Number(s);
  if (n < 0 || n > MAX_COLOR) return { value: null, reason: "range" };
  return { value: n, reason: null };
}

export interface BoardEditorLabels {
  editorTitle: string;
  editHint: string;
  selectPrompt: string;
  cellLabel: (row: string, col: number) => string;
  dir: { u: string; r: string; d: string; l: string };
  clearCell: string;
  faultNan: string;
  faultRange: (max: number) => string;
  faultsSummary: (n: number) => string;
  noFaults: string;
  reset: string;
}

function rowLetter(row: number): string {
  return String.fromCharCode(65 + row);
}

export function BoardEditor({
  width,
  height,
  cells,
  onChange,
  onReset,
  labels,
}: {
  width: number;
  height: number;
  cells: (Edges | null)[];
  onChange: (cells: (Edges | null)[]) => void;
  onReset?: () => void;
  labels: BoardEditorLabels;
}) {
  const [selected, setSelected] = useState<number | null>(null);

  // The faults over the whole board, and per-cell, computed from the committed
  // cells (already-parsed Edges). An edge that fails to parse never reaches
  // `cells` — the field-level editor below keeps the raw text and blocks the
  // commit — so a fault here means an out-of-range value that slipped in via a
  // pasted board (which decodeBucas would have dropped) — defensively surfaced.
  const faults = useMemo<CellFault[]>(() => {
    const out: CellFault[] = [];
    cells.forEach((cell, pos) => {
      if (!cell) return;
      for (const e of cell) {
        if (!Number.isInteger(e) || e < 0 || e > MAX_COLOR) {
          out.push({ pos, message: labels.faultRange(MAX_COLOR) });
          break;
        }
      }
    });
    return out;
  }, [cells, labels]);

  const faultPositions = useMemo(() => faults.map((f) => f.pos), [faults]);

  const setCell = (pos: number, next: Edges | null) => {
    const copy = cells.slice();
    copy[pos] = next;
    onChange(copy);
  };

  return (
    <div className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_minmax(0,280px)]">
      <div>
        <BoardSvg
          width={width}
          height={height}
          cells={cells}
          highlight={selected != null ? [selected] : undefined}
          conflicts={faultPositions.flatMap<[number, number]>((p) => [
            [p, 0],
            [p, 1],
            [p, 2],
            [p, 3],
          ])}
          onCellClick={(pos) => setSelected(pos)}
          coordinates
          className="max-w-full"
        />
        <p className="mt-2 text-xs text-muted-foreground">{labels.editHint}</p>
      </div>

      <div className="space-y-3">
        <div className="flex items-center justify-between gap-2">
          <h3 className="text-sm font-semibold">{labels.editorTitle}</h3>
          {onReset && (
            <Button variant="ghost" size="sm" onClick={() => { setSelected(null); onReset(); }}>
              {labels.reset}
            </Button>
          )}
        </div>

        {selected == null ? (
          <p className="text-sm text-muted-foreground">{labels.selectPrompt}</p>
        ) : (
          <CellEditor
            key={selected}
            row={rowLetter(Math.floor(selected / width))}
            col={(selected % width) + 1}
            initial={cells[selected] ?? null}
            onCommit={(next) => setCell(selected, next)}
            labels={labels}
          />
        )}

        <p className="text-xs text-muted-foreground">
          {faults.length === 0 ? labels.noFaults : labels.faultsSummary(faults.length)}
        </p>
      </div>
    </div>
  );
}

// The four-edge editor for one cell. Each edge is its own input with its own
// red message; the cell only commits parsed Edges (or null when cleared/all-grey)
// back up, so an invalid keystroke shows an error without corrupting the board.
function CellEditor({
  row,
  col,
  initial,
  onCommit,
  labels,
}: {
  row: string;
  col: number;
  initial: Edges | null;
  onCommit: (next: Edges | null) => void;
  labels: BoardEditorLabels;
}) {
  const [raw, setRaw] = useState<RawCell>(() => edgesToRaw(initial));

  const parsed: ReturnType<typeof parseEdge>[] = raw.map(parseEdge);
  const anyInvalid = parsed.some((p) => p.reason === "nan" || p.reason === "range");

  const commit = (nextRaw: RawCell) => {
    setRaw(nextRaw);
    const p = nextRaw.map(parseEdge);
    if (p.some((x) => x.reason === "nan" || x.reason === "range")) return; // hold on invalid
    const vals = p.map((x) => x.value ?? 0) as Edges;
    // An all-grey cell is an empty cell, matching the decoder's convention.
    onCommit(vals.every((v) => v === 0) ? null : vals);
  };

  const setEdge = (i: number, value: string) => {
    const next = raw.slice() as RawCell;
    next[i] = value;
    commit(next);
  };

  return (
    <div className="rounded-lg border p-3">
      <p className="text-sm font-medium">{labels.cellLabel(row, col)}</p>
      <div className="mt-2 grid grid-cols-2 gap-2">
        {DIR_KEYS.map((dk, i) => {
          const p: ReturnType<typeof parseEdge> = parsed[i] ?? { value: null, reason: null };
          const invalid = p.reason === "nan" || p.reason === "range";
          return (
            <label key={dk} className="flex flex-col gap-1 text-xs">
              <span className="text-muted-foreground">{labels.dir[dk]}</span>
              <input
                inputMode="numeric"
                value={raw[i] ?? ""}
                onChange={(e) => setEdge(i, e.target.value)}
                aria-invalid={invalid}
                className={
                  "h-8 rounded-md border bg-transparent px-2 text-sm outline-none focus-visible:ring-3 focus-visible:ring-ring/50 " +
                  (invalid ? "border-red-500 focus-visible:border-red-500" : "border-input focus-visible:border-ring")
                }
              />
              {p.reason === "nan" && <span className="text-red-500">{labels.faultNan}</span>}
              {p.reason === "range" && (
                <span className="text-red-500">{labels.faultRange(MAX_COLOR)}</span>
              )}
            </label>
          );
        })}
      </div>
      <div className="mt-3 flex items-center justify-between gap-2">
        <span className="text-xs text-muted-foreground">
          {anyInvalid ? "" : `${row}${col}`}
        </span>
        <Button
          variant="outline"
          size="sm"
          onClick={() => commit(["0", "0", "0", "0"])}
        >
          {labels.clearCell}
        </Button>
      </div>
    </div>
  );
}
