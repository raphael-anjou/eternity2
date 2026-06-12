// Build your own search path (the order the solver fills cells) by clicking
// or dragging on a grid, then race it against the classics on the same
// puzzle. Path order routinely changes nodes-to-solve by orders of magnitude.

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getMaxColors, getPath, getPathKinds } from "@/engine";
import type { SolverHandle } from "@/engine";
import { useT } from "@/i18n";
import type { Puzzle, SolverReport } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { formatCompact } from "@/lib/format";
import { cn } from "@/lib/utils";

const GRID_SIZES = [4, 5, 6, 8, 10];

const T = {
  en: {
    title: "Invent your own search path",
    intro: (
      <>
        The solver fills the board in whatever order you choose. Click (or drag) cells in the
        order you want them filled, then race your path against the classics on the same
        puzzle. Spoiler: the order matters <em>a lot</em>.
      </>
    ),
    clear: "Clear",
    autoFinish: "Auto-finish",
    gridHelp:
      "Left-click/drag to add cells in order · right-click a cell to cut the path there.",
    raceSetup: "Race setup",
    colorsLabel: (n: number) => `Colors: ${n}`,
    newPuzzle: (seed: number) => `New puzzle (seed ${seed})`,
    raceWithCustom: "Race your path vs the classics",
    raceClassics: "Race the classics",
    stop: "Stop",
    finishPath: (done: number, total: number) =>
      `Finish your path (${done}/${total} cells) to enter it in the race.`,
    yourPath: "Your path",
    finishedBadge: (rank: number | undefined, checks: string) =>
      rank === 1 ? `🏆 ${checks} nodes` : `#${rank} · ${checks} nodes`,
    racingBadge: (checks: string) => `racing… ${checks} nodes`,
    laneStats: (placed: number, total: number, deepest: number, backtracks: string) =>
      `placed ${placed}/${total} · deepest ${deepest} · backtracks ${backtracks}`,
  },
  fr: {
    title: "Inventez votre propre chemin de parcours",
    intro: (
      <>
        Le solveur remplit le plateau dans l'ordre que vous choisissez. Cliquez sur les cases
        (ou faites-les glisser) dans l'ordre où vous voulez les remplir, puis faites courir
        votre chemin contre les classiques sur le même puzzle. Spoiler : l'ordre compte{" "}
        <em>énormément</em>.
      </>
    ),
    clear: "Effacer",
    autoFinish: "Compléter automatiquement",
    gridHelp:
      "Clic gauche ou glisser pour ajouter des cases dans l'ordre · clic droit sur une case pour couper le chemin à cet endroit.",
    raceSetup: "Préparation de la course",
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    newPuzzle: (seed: number) => `Nouveau puzzle (graine ${seed})`,
    raceWithCustom: "Lancer la course : votre chemin contre les classiques",
    raceClassics: "Lancer la course des classiques",
    stop: "Arrêter",
    finishPath: (done: number, total: number) =>
      `Terminez votre chemin (${done}/${total} cases) pour l'inscrire à la course.`,
    yourPath: "Votre chemin",
    finishedBadge: (rank: number | undefined, checks: string) =>
      rank === 1 ? `🏆 ${checks} nœuds` : `#${rank} · ${checks} nœuds`,
    racingBadge: (checks: string) => `en course… ${checks} nœuds`,
    laneStats: (placed: number, total: number, deepest: number, backtracks: string) =>
      `placées ${placed}/${total} · profondeur max ${deepest} · retours en arrière ${backtracks}`,
  },
};

function rankColor(rank: number, total: number): string {
  const t = total <= 1 ? 0 : rank / (total - 1);
  const hue = 0 + t * 120; // red → green
  return `hsl(${hue} 75% 45%)`;
}

interface Lane {
  id: string;
  label: string;
  path: Uint16Array;
  report: SolverReport | null;
  cells: (Edges | null)[] | null;
  finishedAttempts: number | null;
}

export default function Paths() {
  const t = useT(T);
  const engineReady = useEngine();
  const [size, setSize] = useState(6);
  const [order, setOrder] = useState<number[]>([]);
  const [drawing, setDrawing] = useState(false);

  const [colors, setColors] = useState(6);
  const [seed, setSeed] = useState(1);
  const [lanes, setLanes] = useState<Lane[]>([]);
  const [racing, setRacing] = useState(false);
  const solversRef = useRef<Map<string, SolverHandle>>(new Map());
  const puzzleRef = useRef<Puzzle | null>(null);
  const rafRef = useRef(0);

  const n = size * size;
  const rankOf = useMemo(() => {
    const m = new Map<number, number>();
    order.forEach((cell, i) => m.set(cell, i));
    return m;
  }, [order]);

  const appendCell = useCallback(
    (cell: number) => {
      setOrder((prev) => (prev.includes(cell) ? prev : [...prev, cell]));
    },
    [],
  );

  const truncateFrom = (cell: number) => {
    setOrder((prev) => {
      const i = prev.indexOf(cell);
      return i >= 0 ? prev.slice(0, i) : prev;
    });
  };

  const loadDefault = (kind: string) => {
    setOrder(Array.from(getPath(kind, size, size, seed)));
  };

  const completeRowMajor = () => {
    setOrder((prev) => {
      const seen = new Set(prev);
      const rest = [];
      for (let c = 0; c < n; c++) if (!seen.has(c)) rest.push(c);
      return [...prev, ...rest];
    });
  };

  useEffect(() => {
    setOrder([]);
    stopRace();
    setLanes([]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [size]);

  // ---- race machinery -----------------------------------------------------

  const stopRace = () => {
    setRacing(false);
    cancelAnimationFrame(rafRef.current);
    solversRef.current.forEach((s) => s.free());
    solversRef.current.clear();
  };

  const startRace = (laneSpecs: { id: string; label: string; path: Uint16Array }[]) => {
    stopRace();
    const puzzle = getGeneratedPuzzle(size, colors, seed);
    puzzleRef.current = puzzle;
    const newLanes: Lane[] = laneSpecs.map((spec) => {
      const solver = createSolver(puzzle, spec.path, { useHints: false });
      solversRef.current.set(spec.id, solver);
      return {
        ...spec,
        report: solver.report(),
        cells: null,
        finishedAttempts: null,
      };
    });
    setLanes(newLanes);
    setRacing(true);
  };

  useEffect(() => {
    if (!racing) return;
    const tick = () => {
      const puzzle = puzzleRef.current;
      if (!puzzle) return;
      let anyRunning = false;
      // Time-boxed round-robin so every lane advances fairly each frame.
      setLanes((prev) => {
        const next = prev.map((l) => ({ ...l }));
        const frameStart = performance.now();
        let progressed = true;
        while (performance.now() - frameStart < 12 && progressed) {
          progressed = false;
          for (const lane of next) {
            if (lane.finishedAttempts !== null) continue;
            const solver = solversRef.current.get(lane.id);
            if (!solver) continue;
            const r = solver.step(4000);
            lane.report = r;
            if (r.status !== "running") {
              lane.finishedAttempts = r.attempts;
              lane.cells = boardFromEngine(puzzle, solver.board()).cells;
            } else {
              progressed = true;
            }
          }
        }
        for (const lane of next) {
          if (lane.finishedAttempts === null) {
            const solver = solversRef.current.get(lane.id);
            if (solver) lane.cells = boardFromEngine(puzzle, solver.board()).cells;
            anyRunning = true;
          }
        }
        return next;
      });
      if (anyRunning) {
        rafRef.current = requestAnimationFrame(tick);
      } else {
        setRacing(false);
      }
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [racing]);

  useEffect(() => stopRace, []);

  const customComplete = order.length === n;
  const ranking = useMemo(() => {
    const done = lanes.filter((l) => l.finishedAttempts !== null);
    done.sort((a, b) => (a.finishedAttempts ?? 0) - (b.finishedAttempts ?? 0));
    return new Map(done.map((l, i) => [l.id, i + 1]));
  }, [lanes]);

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 text-muted-foreground">{t.intro}</p>
      </div>

      <div className="grid gap-6 lg:grid-cols-[minmax(0,420px)_1fr]">
        <div className="space-y-3">
          <div className="flex flex-wrap items-center gap-2">
            <Select value={String(size)} onValueChange={(v) => v && setSize(parseInt(v, 10))}>
              <SelectTrigger className="w-24">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {GRID_SIZES.map((s) => (
                  <SelectItem key={s} value={String(s)}>
                    {s}×{s}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button variant="outline" size="sm" onClick={() => setOrder([])}>
              {t.clear}
            </Button>
            <Button variant="outline" size="sm" onClick={completeRowMajor} disabled={customComplete}>
              {t.autoFinish}
            </Button>
            <Badge variant={customComplete ? "default" : "secondary"}>
              {order.length}/{n}
            </Badge>
          </div>

          {/* Pointer events + elementFromPoint so drag-painting works with a
              finger as well as a mouse (touch never fires mouseenter). */}
          <div
            className="grid select-none touch-none gap-0.5 rounded-lg border bg-border p-0.5"
            style={{ gridTemplateColumns: `repeat(${size}, minmax(0, 1fr))` }}
            onPointerDown={(e) => {
              if (e.pointerType === "mouse" && e.button !== 0) return;
              const cell = (e.target as HTMLElement).dataset["cell"];
              if (cell === undefined) return;
              setDrawing(true);
              appendCell(parseInt(cell, 10));
            }}
            onPointerMove={(e) => {
              if (!drawing) return;
              const el = document.elementFromPoint(e.clientX, e.clientY) as HTMLElement | null;
              const cell = el?.dataset["cell"];
              if (cell !== undefined) appendCell(parseInt(cell, 10));
            }}
            onPointerUp={() => setDrawing(false)}
            onPointerLeave={() => setDrawing(false)}
            onPointerCancel={() => setDrawing(false)}
          >
            {Array.from({ length: n }, (_, cell) => {
              const rank = rankOf.get(cell);
              return (
                <div
                  key={cell}
                  data-cell={cell}
                  onContextMenu={(e) => {
                    e.preventDefault();
                    truncateFrom(cell);
                  }}
                  className={cn(
                    "flex aspect-square cursor-crosshair items-center justify-center rounded-sm text-xs font-semibold",
                    rank === undefined ? "bg-background text-muted-foreground/40" : "text-white",
                  )}
                  style={
                    rank !== undefined ? { backgroundColor: rankColor(rank, n) } : undefined
                  }
                >
                  {rank !== undefined ? rank + 1 : ""}
                </div>
              );
            })}
          </div>
          <p className="text-xs text-muted-foreground">{t.gridHelp}</p>

          <div className="flex flex-wrap gap-1.5">
            {(engineReady ? getPathKinds() : []).map((k) => (
              <Button key={k} variant="secondary" size="sm" onClick={() => loadDefault(k)}>
                {k}
              </Button>
            ))}
          </div>
        </div>

        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-base">{t.raceSetup}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-1.5">
                  <Label>{t.colorsLabel(colors)}</Label>
                  <Slider
                    min={2}
                    max={engineReady ? getMaxColors(size) : 10}
                    step={1}
                    value={colors}
                    onValueChange={(v) => setColors(Array.isArray(v) ? v[0] : v)}
                  />
                </div>
                <div className="flex items-end gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setSeed(Math.floor(Math.random() * 100000))}
                  >
                    {t.newPuzzle(seed)}
                  </Button>
                </div>
              </div>
              <div className="flex flex-wrap gap-2">
                <Button
                  disabled={!engineReady || racing}
                  onClick={() => {
                    const specs = [
                      ...(customComplete
                        ? [{ id: "custom", label: t.yourPath, path: Uint16Array.from(order) }]
                        : []),
                      { id: "row-major", label: "row-major", path: getPath("row-major", size, size, seed) },
                      { id: "spiral-in", label: "spiral-in", path: getPath("spiral-in", size, size, seed) },
                      { id: "border-first", label: "border-first", path: getPath("border-first", size, size, seed) },
                      { id: "diagonal", label: "diagonal", path: getPath("diagonal", size, size, seed) },
                    ];
                    startRace(specs);
                  }}
                >
                  {customComplete ? t.raceWithCustom : t.raceClassics}
                </Button>
                {racing && (
                  <Button variant="destructive" onClick={stopRace}>
                    {t.stop}
                  </Button>
                )}
              </div>
              {!customComplete && order.length > 0 && (
                <p className="text-xs text-muted-foreground">{t.finishPath(order.length, n)}</p>
              )}
            </CardContent>
          </Card>

          {lanes.length > 0 && (
            <div className="grid gap-4 sm:grid-cols-2">
              {lanes.map((lane) => {
                const rank = ranking.get(lane.id);
                return (
                  <Card
                    key={lane.id}
                    className={cn(rank === 1 && "border-amber-400 shadow-amber-200/50 shadow-lg")}
                  >
                    <CardHeader className="pb-2">
                      <CardTitle className="flex items-center justify-between text-sm">
                        <span>{lane.label}</span>
                        {lane.finishedAttempts !== null ? (
                          <Badge variant={rank === 1 ? "default" : "secondary"}>
                            {t.finishedBadge(rank, formatCompact(lane.finishedAttempts))}
                          </Badge>
                        ) : (
                          <Badge variant="outline" className="animate-pulse">
                            {t.racingBadge(formatCompact(lane.report?.attempts ?? 0))}
                          </Badge>
                        )}
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      {lane.cells && (
                        <BoardSvg width={size} height={size} cells={lane.cells} />
                      )}
                      <p className="mt-1.5 text-xs text-muted-foreground">
                        {t.laneStats(
                          lane.report?.placed ?? 0,
                          n,
                          lane.report?.bestPlaced ?? 0,
                          formatCompact(lane.report?.backtracks ?? 0),
                        )}
                      </p>
                    </CardContent>
                  </Card>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
