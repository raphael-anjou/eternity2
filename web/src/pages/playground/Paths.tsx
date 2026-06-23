import { pageMeta } from "@/seo";
import { LocalizedLink } from "@/components/LocalizedLink";
// Build your own search path (the order the solver fills cells) by clicking
// or dragging on a grid, then race it against the classics on the same
// puzzle. Path order routinely changes nodes-to-solve by orders of magnitude.

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useEngine } from "@/engine/useEngine";
import {
  createSolver,
  getGeneratedPuzzle,
  getGeneratedSolvedPuzzle,
  getMaxColors,
  getPath,
  getPathKinds,
} from "@/engine";
import type { SolverHandle } from "@/engine";
import { useT } from "@/i18n";
import type { Hint, Puzzle, SolverReport } from "@/lib/types";
import { rotateEdges } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { formatCompact, formatInt } from "@/lib/format";
import { cn } from "@/lib/utils";

const GRID_SIZES = [2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16];

const T = {
  en: {
    title: "Invent your own search path",
    intro: (
      <>
        The solver fills the board in whatever order you choose. Click (or drag) cells in the
        order you want them filled, then race your path against the classics on the same
        puzzle. Spoiler: the order matters <em>a lot</em>. You can also <em>reveal</em> a few
        pieces as hints and watch the search collapse, just like the puzzle's real clues.{" "}
        Want to predict which order wins before racing? That's exactly what{" "}
        <LocalizedLink className="underline" to="/research/why/complex-theory">
          complex theory
        </LocalizedLink>{" "}
        estimates — the expected node count of a path, depth by depth.
      </>
    ),
    clear: "Clear",
    autoFinish: "Auto-finish",
    modePath: "Draw path",
    modeHint: "Reveal hint",
    gridHelpHint:
      "Click cells to reveal their true piece as a hint. Hinted cells are pinned and drop out of the path. Click again to un-reveal.",
    hintsBadge: (k: number) => `${k} hint${k === 1 ? "" : "s"}`,
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
    racingBadge: () => "racing…",
    nodesLabel: "nodes explored",
    depthTip: "deepest the search has placed so far",
    laneStats: (placed: number, total: number, deepest: number, backtracks: string) =>
      `placed ${placed}/${total} · deepest ${deepest} · backtracks ${backtracks}`,
  },
  fr: {
    title: "Inventez votre propre parcours de recherche",
    intro: (
      <>
        Le solveur remplit le plateau dans l'ordre que vous lui dictez. Cliquez sur les cases
        (ou faites-les glisser) dans l'ordre où vous voulez les voir remplies, puis lancez
        votre parcours dans la course face aux classiques, sur le même puzzle. Surprise :
        l'ordre change tout, <em>vraiment</em>. Vous pouvez aussi <em>révéler</em> quelques
        pièces en guise d'indices et voir la recherche s'effondrer, exactement comme avec les
        vrais indices du puzzle.{" "}
        Vous voulez prédire quel ordre gagne avant de lancer la course ? C'est précisément ce
        qu'estime la{" "}
        <LocalizedLink className="underline" to="/research/why/complex-theory">
          théorie complexe
        </LocalizedLink>{" "}
        — le nombre de nœuds attendu d'un parcours, profondeur par profondeur.
      </>
    ),
    clear: "Effacer",
    autoFinish: "Compléter le parcours",
    modePath: "Tracer le parcours",
    modeHint: "Révéler un indice",
    gridHelpHint:
      "Cliquez sur une case pour révéler sa vraie pièce en indice. Les cases avec indice sont figées et sortent du parcours. Recliquez pour les masquer.",
    hintsBadge: (k: number) => `${k} indice${k === 1 ? "" : "s"}`,
    gridHelp:
      "Clic gauche ou glissé pour ajouter des cases dans l'ordre · clic droit sur une case pour couper le parcours à cet endroit.",
    raceSetup: "Réglages de la course",
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    newPuzzle: (seed: number) => `Nouveau puzzle (graine ${seed})`,
    raceWithCustom: "Lancer votre parcours contre les classiques",
    raceClassics: "Lancer la course des classiques",
    stop: "Arrêter",
    finishPath: (done: number, total: number) =>
      `Complétez votre parcours (${done}/${total} cases) pour l'engager dans la course.`,
    yourPath: "Votre parcours",
    finishedBadge: (rank: number | undefined, checks: string) =>
      rank === 1 ? `🏆 ${checks} nœuds` : `#${rank} · ${checks} nœuds`,
    racingBadge: () => "en cours…",
    nodesLabel: "nœuds explorés",
    depthTip: "profondeur maximale atteinte par la recherche",
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
  const [mode, setMode] = useState<"path" | "hint">("path");
  // Cells whose true piece is revealed as a hint before the race starts.
  const [hintCells, setHintCells] = useState<Set<number>>(new Set());

  const [colors, setColors] = useState(6);
  const [seed, setSeed] = useState(1);
  const [lanes, setLanes] = useState<Lane[]>([]);
  const [racing, setRacing] = useState(false);
  const solversRef = useRef<Map<string, SolverHandle>>(new Map());
  const lanesRef = useRef<Lane[]>([]); // current lanes for the rAF loop
  const puzzleRef = useRef<Puzzle | null>(null);
  const rafRef = useRef(0);

  const n = size * size;

  // Tear down a running race: stop the loop, free the WASM solvers. Defined
  // up here (before the effects that call it) and memoized so it can be an
  // effect dependency without re-subscribing every render.
  const stopRace = useCallback(() => {
    setRacing(false);
    cancelAnimationFrame(rafRef.current);
    solversRef.current.forEach((s) => s.free());
    solversRef.current.clear();
  }, []);

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
    setOrder(Array.from(getPath(kind, size, size, seed)).filter((c) => !hintCells.has(c)));
  };

  const completeRowMajor = () => {
    setOrder((prev) => {
      const seen = new Set(prev);
      const rest = [];
      for (let c = 0; c < n; c++) if (!seen.has(c) && !hintCells.has(c)) rest.push(c);
      return [...prev, ...rest];
    });
  };

  // Marking a cell as a hint removes it from the drawn path: a hint is a
  // known piece, so it is not part of the order the solver must work out.
  const toggleHint = useCallback((cell: number) => {
    setHintCells((prev) => {
      const next = new Set(prev);
      if (next.has(cell)) next.delete(cell);
      else next.add(cell);
      return next;
    });
    setOrder((prev) => prev.filter((c) => c !== cell));
  }, []);

  useEffect(() => {
    // Size changes the cell count, so both path and hints must reset. This is
    // a reset-on-key-change effect that ALSO frees the running race's WASM
    // solvers (stopRace) — that teardown can't happen during render, so the
    // accompanying state resets live here too.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setOrder([]);
    setHintCells(new Set());
    stopRace();
    setLanes([]);
  }, [size, stopRace]);
  // Note: seed/colors changes keep both the path AND the hints. Both are
  // positional ("this cell"), not piece-bound: the true piece at each hinted
  // cell is looked up fresh from the solved puzzle at race time.

  // Build engine hints for the revealed cells: look up the true piece+rotation
  // at each hinted cell from the solved puzzle, matched against the shuffled
  // race puzzle's pieces (greedy by cell, each piece used once, which is safe
  // because the solved board is a valid tiling).
  const buildHints = (shuffled: Puzzle, cells: Set<number>): Hint[] => {
    if (cells.size === 0) return [];
    const solved = getGeneratedSolvedPuzzle(size, colors, seed);
    const used = new Array(shuffled.pieces.length).fill(false);
    const hints: Hint[] = [];
    // Assign every cell first (keeps the multiset consistent), then keep only
    // the revealed ones as hints.
    const assignment: (Hint | null)[] = [];
    for (let cell = 0; cell < n; cell++) {
      const want = solved.pieces[cell];
      if (!want) {
        assignment.push(null);
        continue;
      }
      let hit: Hint | null = null;
      for (let pid = 0; pid < shuffled.pieces.length && !hit; pid++) {
        if (used[pid]) continue;
        const piece = shuffled.pieces[pid];
        if (!piece) continue;
        for (let r = 0; r < 4; r++) {
          const e = rotateEdges(piece, r);
          if (e[0] === want[0] && e[1] === want[1] && e[2] === want[2] && e[3] === want[3]) {
            used[pid] = true;
            hit = { pos: cell, piece: pid, rot: r };
            break;
          }
        }
      }
      assignment.push(hit);
    }
    for (const cell of cells) {
      const h = assignment[cell];
      if (h) hints.push(h);
    }
    return hints;
  };

  // ---- race machinery -----------------------------------------------------

  const startRace = (laneSpecs: { id: string; label: string; path: Uint16Array }[]) => {
    stopRace();
    const puzzle = getGeneratedPuzzle(size, colors, seed);
    puzzle.hints = buildHints(puzzle, hintCells);
    puzzleRef.current = puzzle;
    const useHints = puzzle.hints.length > 0;
    // The engine wants a full-coverage path (it pre-fills hint cells and
    // skips them). Append any cells the visible path omits — the hints first,
    // then any stragglers — so every lane's path is a permutation of all n.
    const completePath = (path: Uint16Array): Uint16Array => {
      const seen = new Set(path);
      const full = Array.from(path);
      for (let c = 0; c < n; c++) if (!seen.has(c)) full.push(c);
      return Uint16Array.from(full);
    };
    const newLanes: Lane[] = laneSpecs.map((spec) => {
      const solver = createSolver(puzzle, completePath(spec.path), { useHints });
      solversRef.current.set(spec.id, solver);
      return {
        ...spec,
        report: solver.report(),
        cells: null,
        finishedAttempts: null,
      };
    });
    lanesRef.current = newLanes;
    setLanes(newLanes);
    setRacing(true);
  };

  useEffect(() => {
    if (!racing) return;
    const tick = () => {
      const puzzle = puzzleRef.current;
      if (!puzzle) return;

      // Step the WASM solvers HERE (never inside a setState updater: React
      // Strict Mode double-invokes updaters, which would step solvers twice
      // and desync them). We mutate the lanes ref, accumulate the new public
      // state, then hand React a pure snapshot.
      //
      // CRITICAL: the per-frame WASM budget is shared across ALL lanes, not
      // spent per-lane. A per-lane budget meant total synchronous work scaled
      // with lane count (N lanes × 10ms), pinning the main thread every frame
      // and starving the browser of the idle slice it needs to commit a route
      // change — so clicking a nav link while racing appeared to do nothing.
      // One shared budget keeps total work bounded regardless of lane count,
      // leaving the thread responsive enough to navigate away mid-race.
      const FRAME_BUDGET_MS = 8;
      let anyRunning = false;
      const runningCount = lanesRef.current.filter((l) => l.finishedAttempts === null).length;
      // Split the frame budget evenly; at least a sliver each so every lane moves.
      const perLaneMs = runningCount > 0 ? Math.max(1, FRAME_BUDGET_MS / runningCount) : 0;
      const snapshot = lanesRef.current.map((lane) => {
        if (lane.finishedAttempts !== null) return lane;
        const solver = solversRef.current.get(lane.id);
        if (!solver) return lane;
        // A slice of the shared frame budget, so fast lanes still finish quickly
        // while every lane gets visible motion each frame.
        const laneStart = performance.now();
        let r = lane.report ?? solver.report();
        while (r.status === "running" && performance.now() - laneStart < perLaneMs) {
          r = solver.step(20_000);
        }
        if (r.status !== "running") {
          return {
            ...lane,
            report: r,
            finishedAttempts: r.nodes,
            cells: boardFromEngine(puzzle, solver.board()).cells,
          };
        }
        anyRunning = true;
        // Live working board each frame: it thrashes, which is the point: you
        // can watch the search churn. The node counter carries true progress.
        return { ...lane, report: r, cells: boardFromEngine(puzzle, solver.board()).cells };
      });

      lanesRef.current = snapshot;
      setLanes(snapshot);

      if (anyRunning) {
        rafRef.current = requestAnimationFrame(tick);
      } else {
        setRacing(false);
      }
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [racing]);

  useEffect(() => stopRace, [stopRace]);

  const pathTarget = n - hintCells.size;
  const customComplete = order.length === pathTarget && pathTarget > 0;
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
                {/* Show the closed selector as "Y×Y", matching the menu items
                    (SelectValue alone would render the bare value, e.g. "6"). */}
                <SelectValue>{`${size}×${size}`}</SelectValue>
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
              {order.length}/{pathTarget}
            </Badge>
            {hintCells.size > 0 && (
              <Badge variant="outline" className="border-amber-500 text-amber-600">
                📌 {t.hintsBadge(hintCells.size)}
              </Badge>
            )}
          </div>

          <div className="inline-flex rounded-md border p-0.5 text-sm">
            <button
              onClick={() => setMode("path")}
              className={cn(
                "rounded px-3 py-1 font-medium transition-colors",
                mode === "path" ? "bg-primary text-primary-foreground" : "text-muted-foreground",
              )}
            >
              {t.modePath}
            </button>
            <button
              onClick={() => setMode("hint")}
              className={cn(
                "rounded px-3 py-1 font-medium transition-colors",
                mode === "hint" ? "bg-amber-400 text-amber-950" : "text-muted-foreground",
              )}
            >
              📌 {t.modeHint}
            </button>
          </div>

          {/* Pointer events + elementFromPoint so drag-painting works with a
              finger as well as a mouse (touch never fires mouseenter). */}
          <div
            className="grid select-none touch-none rounded-lg border bg-border p-0.5"
            style={{
              gridTemplateColumns: `repeat(${size}, minmax(0, 1fr))`,
              gap: size > 8 ? "1px" : "2px",
            }}
            onPointerDown={(e) => {
              if (e.pointerType === "mouse" && e.button !== 0) return;
              const cell = (e.target as HTMLElement).dataset["cell"];
              if (cell === undefined) return;
              const c = parseInt(cell, 10);
              if (mode === "hint") {
                toggleHint(c);
                return;
              }
              if (hintCells.has(c)) return; // can't route a path through a hint
              setDrawing(true);
              appendCell(c);
            }}
            onPointerMove={(e) => {
              if (!drawing || mode === "hint") return;
              const el = document.elementFromPoint(e.clientX, e.clientY) as HTMLElement | null;
              const cell = el?.dataset["cell"];
              if (cell !== undefined && !hintCells.has(parseInt(cell, 10))) {
                appendCell(parseInt(cell, 10));
              }
            }}
            onPointerUp={() => setDrawing(false)}
            onPointerLeave={() => setDrawing(false)}
            onPointerCancel={() => setDrawing(false)}
          >
            {Array.from({ length: n }, (_, cell) => {
              const rank = rankOf.get(cell);
              const isHint = hintCells.has(cell);
              return (
                <div
                  key={cell}
                  data-cell={cell}
                  onContextMenu={(e) => {
                    e.preventDefault();
                    if (isHint) toggleHint(cell);
                    else truncateFrom(cell);
                  }}
                  className={cn(
                    "flex aspect-square cursor-crosshair items-center justify-center rounded-sm text-xs font-semibold",
                    isHint
                      ? "bg-amber-400 text-amber-950 ring-1 ring-amber-600"
                      : rank === undefined
                        ? "bg-background text-muted-foreground/40"
                        : "text-white",
                  )}
                  style={
                    !isHint && rank !== undefined
                      ? { backgroundColor: rankColor(rank, n) }
                      : undefined
                  }
                >
                  {isHint ? "📌" : rank !== undefined && size <= 9 ? rank + 1 : ""}
                </div>
              );
            })}
          </div>
          <p className="text-xs text-muted-foreground">{mode === "hint" ? t.gridHelpHint : t.gridHelp}</p>

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
                    onValueChange={(v) => setColors(singleSliderValue(v))}
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
                            {t.racingBadge()}
                          </Badge>
                        )}
                      </CardTitle>
                    </CardHeader>
                    <CardContent>
                      {lane.cells && (
                        <BoardSvg width={size} height={size} cells={lane.cells} />
                      )}
                      {lane.finishedAttempts === null && (
                        <div className="mt-2 flex items-baseline justify-between font-mono text-sm">
                          <span className="tabular-nums font-semibold">
                            {formatInt(lane.report?.nodes ?? 0)}
                          </span>
                          <span className="text-xs text-muted-foreground">{t.nodesLabel}</span>
                        </div>
                      )}
                      {lane.finishedAttempts === null && lane.report && (
                        <div
                          className="mt-1 h-1.5 overflow-hidden rounded bg-muted"
                          title={t.depthTip}
                        >
                          <div
                            className="h-full bg-primary transition-[width]"
                            style={{ width: `${(lane.report.bestPlaced / n) * 100}%` }}
                          />
                        </div>
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

export const meta = pageMeta("paths");
