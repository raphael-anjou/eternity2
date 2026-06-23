import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  Area,
  AreaChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import type { Puzzle, SolverHandle } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { countCandidates } from "@/lib/piece-fit";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";

// LODESTONE made visible: the supply of scarce pieces drains as a board fills.
// We watch a real solver fill a small board, and after every placement we ask,
// for each still-empty cell, how many unused pieces can legally serve the
// (north,west) colours it now demands — the exact "demand server" count the
// method is built on. Cells served by only one or two remaining pieces are the
// scarce demands; their number climbs sharply toward the endgame, which is why a
// from-scratch search wants to commit the rare pieces early, before they're
// stolen. Real engine; the scarce counts are computed with the same legality
// test the solver uses, so nothing is staged.

const SIZE = 6;
const COLORS = 5;
const SEED = 4;

interface Snapshot {
  placed: number;
  unique: number; // empty cells with exactly 1 serving piece left
  scarce: number; // empty cells with <= 2
}

const T = {
  en: {
    title: "Watch the rare pieces drain away",
    intro:
      "A real solver fills this 6×6 board. After each piece, we count the still-empty cells whose colours can be met by only one or two of the pieces left in the box — the scarce demands. Watch them pile up as the board fills: the rare pieces a cell needs get spent elsewhere, and by the endgame almost every open cell is down to its last server or two. That scarcity, learned from strong boards, is the signal LODESTONE feeds the search.",
    speed: (n: string) => `Speed: ${n} steps/s`,
    run: "Run",
    pause: "Pause",
    reset: "Reset",
    placed: "pieces placed",
    scarce: "scarce cells (≤2 servers)",
    unique: "down to one server",
    chartScarce: "≤ 2 servers",
    chartUnique: "exactly 1",
    xAxis: "pieces placed",
    busy: "Drawing…",
    loading: "Loading the engine…",
    legendScarce: "≤ 2 servers left",
    legendUnique: "exactly 1 left (a unique server)",
  },
  fr: {
    title: "Regardez les pièces rares se raréfier",
    intro:
      "Un vrai solveur remplit ce plateau 6×6. Après chaque pièce, on compte les cases encore vides dont les couleurs ne peuvent être satisfaites que par une ou deux des pièces restantes — les demandes rares. Regardez-les s'accumuler à mesure que le plateau se remplit : les pièces rares dont une case a besoin sont dépensées ailleurs, et à la fin de partie presque chaque case ouverte n'a plus qu'un ou deux serveurs. Cette rareté, apprise des bons plateaux, est le signal que LODESTONE fournit à la recherche.",
    speed: (n: string) => `Vitesse : ${n} étapes/s`,
    run: "Lancer",
    pause: "Pause",
    reset: "Réinitialiser",
    placed: "pièces posées",
    scarce: "cases rares (≤2 serveurs)",
    unique: "réduites à un serveur",
    chartScarce: "≤ 2 serveurs",
    chartUnique: "exactement 1",
    xAxis: "pièces posées",
    busy: "Tracé…",
    loading: "Chargement du moteur…",
    legendScarce: "≤ 2 serveurs restants",
    legendUnique: "exactement 1 restant (serveur unique)",
  },
};

export function LodestoneRarityLab() {
  const t = useT(T);
  const ready = useEngine();
  const isClient = useIsClient();
  const [speedExp, setSpeedExp] = useState(1); // 10^1 = 10 placements/s
  const [running, setRunning] = useState(false);

  const [cells, setCells] = useState<(Edges | null)[] | null>(null);
  const [scarceCells, setScarceCells] = useState<number[]>([]);
  const [series, setSeries] = useState<Snapshot[]>([]);
  const [placed, setPlaced] = useState(0);
  const [done, setDone] = useState(false);

  const puzzle = useMemo<Puzzle | null>(
    () => (ready ? getGeneratedPuzzle(SIZE, COLORS, SEED) : null),
    [ready],
  );

  const solverRef = useRef<SolverHandle | null>(null);
  const rafRef = useRef(0);

  // Recompute the scarcity snapshot from the current board.
  const measure = useCallback((p: Puzzle, boardCells: Int32Array) => {
    const view = boardFromEngine(p, Array.from(boardCells));
    const used = new Set<number>();
    boardCells.forEach((v) => {
      if (v >= 0) used.add(v >> 2);
    });
    let unique = 0;
    let scarce = 0;
    const scarcePos: number[] = [];
    const n = SIZE * SIZE;
    for (let pos = 0; pos < n; pos++) {
      if (view.cells[pos]) continue; // occupied
      const x = pos % SIZE;
      const y = Math.floor(pos / SIZE);
      // only cells that already have a north or west neighbour place a demand
      const hasN = y > 0 && !!view.cells[pos - SIZE];
      const hasW = x > 0 && !!view.cells[pos - 1];
      if (!hasN && !hasW) continue;
      const servers = countCandidates(p.pieces, pos, view.cells, used, SIZE, SIZE).size;
      if (servers <= 2) {
        scarce++;
        scarcePos.push(pos);
      }
      if (servers === 1) unique++;
    }
    return { cells: view.cells, unique, scarce, scarcePos, placedCount: used.size };
  }, []);

  const rebuild = useCallback(() => {
    solverRef.current?.free();
    if (!puzzle) return;
    const path = getPath("snake", SIZE, SIZE, 0);
    const s = createSolver(puzzle, path, { useHints: false });
    solverRef.current = s;
    const snap = measure(puzzle, s.board());
    setCells(snap.cells);
    setScarceCells(snap.scarcePos);
    setSeries([{ placed: 0, unique: snap.unique, scarce: snap.scarce }]);
    setPlaced(0);
    setDone(false);
    setRunning(false);
  }, [puzzle, measure]);

  useEffect(() => {
    // rebuild constructs the WASM solver (external) and seeds the view.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    rebuild();
    return () => {
      solverRef.current?.free();
      solverRef.current = null;
    };
  }, [rebuild]);

  // Step the solver one *placement* at a time (so the scarcity series advances
  // by piece, not by raw DFS step), paced by the speed slider.
  useEffect(() => {
    if (!running || !puzzle) return;
    const perSec = Math.pow(10, speedExp);
    let acc = 0;
    let last = 0;
    const tick = (now: number) => {
      const s = solverRef.current;
      if (s) {
        const dt = last ? (now - last) / 1000 : 1 / 60;
        last = now;
        acc += perSec * dt;
        let advanced = false;
        while (acc >= 1) {
          acc -= 1;
          const before = s.report().placed;
          // run until the placed count changes or the search ends
          let guard = 0;
          let r = s.report();
          while (r.status === "running" && r.placed === before && guard < 5000) {
            r = s.step(1);
            guard++;
          }
          advanced = true;
          if (r.status !== "running") {
            setRunning(false);
            setDone(true);
            break;
          }
        }
        if (advanced) {
          const snap = measure(puzzle, s.board());
          setCells(snap.cells);
          setScarceCells(snap.scarcePos);
          setPlaced(snap.placedCount);
          setSeries((prev) => {
            const last2 = prev[prev.length - 1];
            if (last2 && last2.placed === snap.placedCount) return prev;
            return [...prev, { placed: snap.placedCount, unique: snap.unique, scarce: snap.scarce }];
          });
        }
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [running, speedExp, puzzle, measure]);

  const placeholder = useMemo<(Edges | null)[]>(() => Array<Edges | null>(SIZE * SIZE).fill(null), []);
  const speedLabel = Math.pow(10, speedExp).toLocaleString();
  const latest = series[series.length - 1];

  return (
    <section className="mx-auto max-w-3xl space-y-4 rounded-xl border bg-muted/20 p-5">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>

      {puzzle ? (
        <div className="grid gap-5 sm:grid-cols-[minmax(0,240px)_1fr] sm:items-start">
          <div className="space-y-2">
            <BoardSvg
              width={SIZE}
              height={SIZE}
              cells={cells ?? placeholder}
              highlight={scarceCells}
              className="max-w-[240px]"
            />
            <div className="flex flex-wrap gap-x-4 gap-y-0.5 text-[11px] text-muted-foreground">
              <span>
                <span className="font-semibold text-foreground tabular-nums">{placed}</span> {t.placed}
              </span>
              <span>
                <span className="font-semibold text-amber-600 tabular-nums">{latest?.scarce ?? 0}</span>{" "}
                {t.scarce}
              </span>
            </div>
          </div>

          <div className="space-y-3">
            <div className="space-y-1.5">
              <Label>{t.speed(speedLabel)}</Label>
              <Slider min={0} max={3} step={1} value={speedExp} onValueChange={(v) => setSpeedExp(singleSliderValue(v))} />
            </div>
            <div className="flex items-center gap-2">
              <Button size="sm" onClick={() => setRunning((r) => !r)} disabled={done}>
                {running ? t.pause : t.run}
              </Button>
              <Button size="sm" variant="outline" onClick={rebuild}>
                {t.reset}
              </Button>
            </div>

            <div className="h-40 rounded-lg border bg-background p-2">
              {isClient ? (
                <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
                  <AreaChart data={series} margin={{ top: 6, right: 10, bottom: 16, left: 0 }}>
                    <XAxis
                      dataKey="placed"
                      type="number"
                      domain={[0, SIZE * SIZE]}
                      fontSize={10}
                      label={{ value: t.xAxis, position: "insideBottom", offset: -8, fontSize: 10 }}
                    />
                    <YAxis fontSize={10} width={24} allowDecimals={false} />
                    <Tooltip
                      labelFormatter={(v) => `${String(v)} ${t.placed}`}
                      formatter={(val, name) => [String(val), name === "scarce" ? t.chartScarce : t.chartUnique]}
                    />
                    <Area
                      type="monotone"
                      dataKey="scarce"
                      stroke="#f59e0b"
                      fill="#f59e0b"
                      fillOpacity={0.18}
                      isAnimationActive={false}
                    />
                    <Area
                      type="monotone"
                      dataKey="unique"
                      stroke="#ef4444"
                      fill="#ef4444"
                      fillOpacity={0.25}
                      isAnimationActive={false}
                    />
                  </AreaChart>
                </ResponsiveContainer>
              ) : (
                <div className="flex h-full items-center justify-center text-xs text-muted-foreground">{t.busy}</div>
              )}
            </div>
            <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs text-muted-foreground">
              <span className="flex items-center gap-1.5">
                <span className="inline-block h-2.5 w-2.5 rounded-sm" style={{ background: "#f59e0b" }} />
                {t.legendScarce}
              </span>
              <span className="flex items-center gap-1.5">
                <span className="inline-block h-2.5 w-2.5 rounded-sm" style={{ background: "#ef4444" }} />
                {t.legendUnique}
              </span>
            </div>
          </div>
        </div>
      ) : (
        <div className="flex h-44 items-center justify-center text-sm text-muted-foreground">{t.loading}</div>
      )}
    </section>
  );
}
