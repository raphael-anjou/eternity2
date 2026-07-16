import { useMemo, useState } from "react";
import {
  Bar,
  BarChart,
  Cell,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { BoardSvg } from "@/components/board/BoardSvg";
import { decodeBucas, conflictEdges } from "@/lib/bucas";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { RECORD_BOARDS } from "@/data/record-boards";

// The mismatch-geometry finding, computed live from real boards. We take an
// actual record board, find its mismatched edges with the exact same scoring
// rule the engine uses (conflictEdges), and bin them by row. Community boards
// (built border-first / bottom-up) pile their few mismatches in the TOP rows;
// this project's from-scratch boards (built top-down) mirror them into the
// BOTTOM rows. Nothing is hand-placed — the per-row counts and the highlighted
// band are derived from the board data every render.

type BoardChoice = {
  id: string;
  params: string;
  camp: "community" | "scratch";
};

// A pair from each camp, chosen because their geometry is the cleanest mirror
// (verified from the board data: McGavin 469 → rows 0–4; our boards → rows 11–15).
const COMMUNITY = new Map(KNOWN_BOARDS.map((b) => [b.id, b.params]));
const SCRATCH = new Map(RECORD_BOARDS.map((b) => [b.id, b.viewerParams]));

const CHOICES: BoardChoice[] = (
  [
    { id: "JBlackwood+PMcGavin_469", params: COMMUNITY.get("JBlackwood+PMcGavin_469") ?? "", camp: "community" },
    { id: "Louis_Verhaard_467", params: COMMUNITY.get("Louis_Verhaard_467") ?? "", camp: "community" },
    { id: "keyring-460", params: SCRATCH.get("keyring-460") ?? "", camp: "scratch" },
    { id: "gauntlet-458", params: SCRATCH.get("gauntlet-458") ?? "", camp: "scratch" },
  ] satisfies BoardChoice[]
).filter((c) => c.params);

const T = {
  en: {
    pick: "Board",
    community: "Community (top-concentrated)",
    scratch: "This project (bottom-concentrated)",
    labels: {
      "JBlackwood+PMcGavin_469": "McGavin 469",
      Louis_Verhaard_467: "Verhaard 467",
      "keyring-460": "KEYRING 460",
      "gauntlet-458": "GAUNTLET 458",
    } as Record<string, string>,
    rowAxis: "row (0 = top)",
    countAxis: "mismatches",
    total: (n: number) => `${n} mismatched edges`,
    band: (a: number, b: number) => `all in rows ${a}–${b}`,
    perfect: "every other row is perfect",
    busy: "Computing…",
    caption:
      "The shaded band is where the mismatches actually fall, highlighted on the real board; the bars count them per row. Switch boards: the community's pile sits at the top, this project's at the bottom — the same puzzle, opposite scan directions.",
  },
  fr: {
    pick: "Plateau",
    community: "Communauté (concentré en haut)",
    scratch: "Ce projet (concentré en bas)",
    labels: {
      "JBlackwood+PMcGavin_469": "McGavin 469",
      Louis_Verhaard_467: "Verhaard 467",
      "keyring-460": "KEYRING 460",
      "gauntlet-458": "GAUNTLET 458",
    } as Record<string, string>,
    rowAxis: "rangée (0 = haut)",
    countAxis: "défauts",
    total: (n: number) => `${n} bords non appariés`,
    band: (a: number, b: number) => `tous en rangées ${a}–${b}`,
    perfect: "toutes les autres rangées sont parfaites",
    busy: "Calcul…",
    caption:
      "La bande ombrée est l'endroit où tombent réellement les défauts, surlignée sur le vrai plateau ; les barres les comptent par rangée. Changez de plateau : le tas de la communauté est en haut, celui de ce projet en bas — le même puzzle, des sens de balayage opposés.",
  },
};

export function MismatchGeometryLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const [boardId, setBoardId] = useState(CHOICES[0]?.id ?? "");

  const choice = CHOICES.find((c) => c.id === boardId) ?? CHOICES[0];
  const params = choice?.params ?? "";
  const camp = choice?.camp;

  const decoded = useMemo(() => (params ? decodeBucas(params) : null), [params]);

  const { conflicts, perRow, total, band, bandCells } = useMemo(() => {
    if (!decoded) return { conflicts: [], perRow: [], total: 0, band: null, bandCells: [] };
    const w = decoded.width;
    const h = decoded.height;
    const confl = conflictEdges(decoded);
    // Each mismatched edge contributes two half-edges; weight 0.5 each so a
    // shared edge counts once. Bin by the row of the cell each half sits on.
    const rows = new Array<number>(h).fill(0);
    for (const [cell] of confl) {
      const r = Math.floor(cell / w);
      rows[r] = (rows[r] ?? 0) + 0.5;
    }
    const per = rows.map((count, row) => ({ row, count }));
    const tot = Math.round(rows.reduce((a, b) => a + b, 0));
    const hit = per.filter((r) => r.count > 0).map((r) => r.row);
    const bandRange = hit.length ? ([Math.min(...hit), Math.max(...hit)] as [number, number]) : null;
    // Cells to highlight: the whole band of affected rows, lightly, so the
    // concentration reads at a glance even before the conflict marks.
    const cells: number[] = [];
    if (bandRange) {
      for (let r = bandRange[0]; r <= bandRange[1]; r++) {
        for (let c = 0; c < w; c++) cells.push(r * w + c);
      }
    }
    return { conflicts: confl, perRow: per, total: tot, band: bandRange, bandCells: cells };
  }, [decoded]);

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-1.5">
        {CHOICES.map((c) => (
          <button
            key={c.id}
            type="button"
            onClick={() => setBoardId(c.id)}
            className={`rounded-full border px-3 py-1 text-xs font-medium transition-colors ${
              c.id === boardId
                ? "border-primary bg-primary text-primary-foreground"
                : "hover:bg-muted"
            }`}
          >
            {t.labels[c.id] ?? c.id}
          </button>
        ))}
      </div>

      <div className="grid gap-4 md:grid-cols-[1fr_320px]">
        <div>
          {decoded && (
            <BoardSvg
              width={decoded.width}
              height={decoded.height}
              cells={decoded.cells}
              conflicts={conflicts}
              highlight={bandCells}
              className="max-w-md"
            />
          )}
        </div>
        <div className="space-y-2">
          <div className="text-sm">
            <span className="font-semibold tabular-nums">{t.total(total)}</span>
            {band && (
              <span className="text-muted-foreground">
                {" — "}
                {t.band(band[0], band[1])}; {t.perfect}.
              </span>
            )}
          </div>
          <div className="h-44 rounded-lg border p-2" role="img" aria-label={t.caption}>
            {isClient ? (
              <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
                <BarChart data={perRow} margin={{ top: 6, right: 10, bottom: 16, left: 4 }}>
                  <XAxis
                    dataKey="row"
                    fontSize={9}
                    interval={1}
                    label={{ value: t.rowAxis, position: "insideBottom", offset: -8, fontSize: 10 }}
                  />
                  <YAxis fontSize={10} width={22} allowDecimals={false} />
                  <Tooltip
                    formatter={(v) => [String(v), t.countAxis]}
                    labelFormatter={(r) => `${t.rowAxis.split(" ")[0]} ${String(r)}`}
                  />
                  <Bar dataKey="count" radius={[3, 3, 0, 0]} isAnimationActive={false}>
                    {perRow.map((r) => (
                      <Cell
                        key={r.row}
                        fill={r.count > 0 ? (camp === "community" ? "#f59e0b" : "#38bdf8") : "#e5e7eb"}
                      />
                    ))}
                  </Bar>
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <div className="flex h-full items-center justify-center text-xs text-muted-foreground">
                {t.busy}
              </div>
            )}
          </div>
        </div>
      </div>

      <p className="text-xs leading-relaxed text-muted-foreground">{t.caption}</p>
    </div>
  );
}
