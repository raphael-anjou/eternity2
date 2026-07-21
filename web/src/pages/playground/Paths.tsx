import { pageMeta } from "@/seo";
import { LocalizedLink } from "@/components/LocalizedLink";
// Build your own search path (the order the solver fills cells) by clicking
// or dragging on a grid, then race it against the classics on the same
// puzzle. Path order routinely changes nodes-to-solve by orders of magnitude.

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { PathComplexity } from "@/components/learn/PathComplexity";
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
  getGeneratedPuzzleFramed,
  getGeneratedSolvedPuzzleFramed,
  getMaxColors,
  getPath,
  getPathKinds,
} from "@/engine";
import { Switch } from "@/components/ui/switch";
import type { SolverHandle } from "@/engine";
import { createBlockSolver, type Block } from "@/engine-extras/block-solver";
import { useT } from "@/i18n";
import type { Hint, Puzzle, SolverReport } from "@/lib/types";
import { rotateEdges } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";
import { formatCompact, formatInt } from "@/lib/format";
import { cn } from "@/lib/utils";

const GRID_SIZES = [2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16];

// Brush footprints offered on the path grid (Dan Karlsson's list). 1×1 is the
// classic single-cell path; the rest build a block path the macro-piece solver
// can fill block-by-block.
const BRUSH_SHAPES: { w: number; h: number }[] = [
  { w: 1, h: 1 },
  { w: 2, h: 1 },
  { w: 1, h: 2 },
  { w: 2, h: 2 },
  { w: 3, h: 3 },
  { w: 4, h: 4 },
  { w: 4, h: 1 },
];

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
    gridHelpBlock:
      "Click to stamp a block of cells at once · right-click a cell to cut the path at its block. Blocks fill as a unit when you race.",
    brushLabel: "Block",
    blocksBadge: (k: number) => `${k} block${k === 1 ? "" : "s"}`,
    blockMode: "Block mode",
    blockModeHelp:
      "Your path is built from blocks. Race it and the solver commits each block as one atomic move — a whole valid sub-assembly at a time — instead of one piece.",
    raceSetup: "Race setup",
    colorsLabel: (n: number) => `Colors: ${n}`,
    newPuzzle: (seed: number) => `New puzzle (seed ${seed})`,
    framed: "Frame-restricted colors",
    framedHint:
      "Like the real Eternity II: confine some colors to the border band, the rest to the deep interior (needs size ≥ 4, colors ≥ 2).",
    raceWithCustom: "Race your path vs the classics",
    raceClassics: "Race the classics",
    stop: "Stop",
    finishPath: (done: number, total: number) =>
      `Finish your path (${done}/${total} cells) to enter it in the race.`,
    yourPath: "Your path",
    finishedBadge: (rank: number | undefined, checks: string) =>
      rank === 1 ? `🏆 ${checks} checks` : `#${rank} · ${checks} checks`,
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
    gridHelpBlock:
      "Cliquez pour poser tout un bloc de cases d'un coup · clic droit sur une case pour couper le parcours à son bloc. Les blocs se remplissent d'un seul tenant pendant la course.",
    brushLabel: "Bloc",
    blocksBadge: (k: number) => `${k} bloc${k === 1 ? "" : "s"}`,
    blockMode: "Mode bloc",
    blockModeHelp:
      "Votre parcours est constitué de blocs. Lancez la course et le solveur valide chaque bloc comme un coup atomique — un sous-assemblage valide complet à la fois — au lieu d'une seule pièce.",
    raceSetup: "Réglages de la course",
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    newPuzzle: (seed: number) => `Nouveau puzzle (graine ${seed})`,
    framed: "Couleurs réservées au cadre",
    framedHint:
      "Comme le vrai Eternity II : confine certaines couleurs à la bande de bordure, les autres à l'intérieur profond (nécessite taille ≥ 4, couleurs ≥ 2).",
    raceWithCustom: "Lancer votre parcours contre les classiques",
    raceClassics: "Lancer la course des classiques",
    stop: "Arrêter",
    finishPath: (done: number, total: number) =>
      `Complétez votre parcours (${done}/${total} cases) pour l'engager dans la course.`,
    yourPath: "Votre parcours",
    finishedBadge: (rank: number | undefined, checks: string) =>
      rank === 1 ? `🏆 ${checks} tests` : `#${rank} · ${checks} tests`,
    racingBadge: () => "en cours…",
    nodesLabel: "nœuds explorés",
    depthTip: "profondeur maximale atteinte par la recherche",
    laneStats: (placed: number, total: number, deepest: number, backtracks: string) =>
      `placées ${placed}/${total} · profondeur max ${deepest} · retours en arrière ${backtracks}`,
  },
  es: {
    title: "Inventa tu propio camino de búsqueda",
    intro: (
      <>
        El solucionador rellena el tablero en el orden que tú decidas. Haz clic (o arrastra) sobre
        las celdas en el orden en que quieres que se rellenen y luego haz que tu camino compita
        contra los clásicos en el mismo puzzle. Sorpresa: el orden importa <em>muchísimo</em>.
        También puedes <em>revelar</em> algunas piezas como pistas y ver cómo se desploma la
        búsqueda, igual que con las pistas reales del puzzle.{" "}
        ¿Quieres predecir qué orden gana antes de la carrera? Eso es exactamente lo que estima la{" "}
        <LocalizedLink className="underline" to="/research/why/complex-theory">
          teoría compleja
        </LocalizedLink>{" "}
        — el número de nodos esperado de un camino, nivel de profundidad a nivel de profundidad.
      </>
    ),
    clear: "Limpiar",
    autoFinish: "Completar el camino",
    modePath: "Trazar camino",
    modeHint: "Revelar pista",
    gridHelpHint:
      "Haz clic en una celda para revelar su pieza real como pista. Las celdas con pista quedan fijadas y salen del camino. Vuelve a hacer clic para ocultarlas.",
    hintsBadge: (k: number) => `${k} pista${k === 1 ? "" : "s"}`,
    gridHelp:
      "Clic izquierdo o arrastrar para añadir celdas en orden · clic derecho sobre una celda para cortar el camino ahí.",
    gridHelpBlock:
      "Haz clic para estampar un bloque de celdas de una vez · clic derecho sobre una celda para cortar el camino en su bloque. Los bloques se rellenan como una unidad durante la carrera.",
    brushLabel: "Bloque",
    blocksBadge: (k: number) => `${k} bloque${k === 1 ? "" : "s"}`,
    blockMode: "Modo bloque",
    blockModeHelp:
      "Tu camino se construye a partir de bloques. Lanza la carrera y el solucionador confirma cada bloque como un movimiento atómico — un subensamblaje válido completo a la vez — en lugar de una sola pieza.",
    raceSetup: "Configuración de la carrera",
    colorsLabel: (n: number) => `Colores: ${n}`,
    newPuzzle: (seed: number) => `Nuevo puzzle (semilla ${seed})`,
    framed: "Colores reservados al marco",
    framedHint:
      "Como el Eternity II real: confina algunos colores a la banda de borde y el resto al interior profundo (requiere tamaño ≥ 4, colores ≥ 2).",
    raceWithCustom: "Compite tu camino contra los clásicos",
    raceClassics: "Lanzar la carrera de los clásicos",
    stop: "Detener",
    finishPath: (done: number, total: number) =>
      `Completa tu camino (${done}/${total} celdas) para inscribirlo en la carrera.`,
    yourPath: "Tu camino",
    finishedBadge: (rank: number | undefined, checks: string) =>
      rank === 1 ? `🏆 ${checks} comprobaciones` : `#${rank} · ${checks} comprobaciones`,
    racingBadge: () => "compitiendo…",
    nodesLabel: "nodos explorados",
    depthTip: "máxima profundidad alcanzada por la búsqueda hasta ahora",
    laneStats: (placed: number, total: number, deepest: number, backtracks: string) =>
      `colocadas ${placed}/${total} · profundidad máx ${deepest} · retrocesos ${backtracks}`,
  },
};

function rankColor(rank: number, total: number): string {
  const t = total <= 1 ? 0 : rank / (total - 1);
  const hue = 0 + t * 120; // red → green
  return `hsl(${hue} 75% 45%)`;
}

// A distinct, stable colour per block. Adjacent block indices get well-separated
// hues (golden-angle stepping) and alternating lightness so neighbouring blocks
// stay visually distinct even when their hues land close.
function blockColor(blockIndex: number): string {
  const hue = (blockIndex * 137.508) % 360;
  const light = blockIndex % 2 === 0 ? 45 : 38;
  return `hsl(${hue} 60% ${light}%)`;
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
  // The drawn path as a list of BLOCKS (each block = the cells it covers, in
  // internal placement order). A single-cell click is a 1×1 block. `order` (the
  // flat cell sequence the single-piece solver + estimate consume) is just
  // blocks.flat(). Block mode (any block bigger than 1×1) additionally lets the
  // race run the macro-piece BlockSolver, which commits whole blocks atomically.
  const [blockGroups, setBlockGroups] = useState<number[][]>([]);
  const order = useMemo(() => blockGroups.flat(), [blockGroups]);
  // Block mode = the path uses at least one multi-cell block. Drives whether the
  // race runs the macro-piece BlockSolver for the custom lane.
  const blockMode = useMemo(() => blockGroups.some((g) => g.length > 1), [blockGroups]);
  // Brush footprint: w×h cells laid per click. 1×1 = the classic single-cell
  // path. Larger footprints build a block path.
  const [brush, setBrush] = useState<{ w: number; h: number }>({ w: 1, h: 1 });
  const [drawing, setDrawing] = useState(false);
  const [mode, setMode] = useState<"path" | "hint">("path");
  // Cells whose true piece is revealed as a hint before the race starts.
  const [hintCells, setHintCells] = useState<Set<number>>(new Set());

  const [colors, setColors] = useState(6);
  const [seed, setSeed] = useState(1);
  // Frame-restricted colours: like the real Eternity II, confine some colours to
  // the border band and the rest to the deep interior (needs size ≥ 4). Ported
  // from the board generator / Viewer.
  const [framed, setFramed] = useState(false);
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

  // cell -> the index of the block it belongs to (for colouring/labelling the
  // grid by block when a block path is drawn).
  const blockOf = useMemo(() => {
    const m = new Map<number, number>();
    blockGroups.forEach((g, bi) => g.forEach((cell) => m.set(cell, bi)));
    return m;
  }, [blockGroups]);

  // Lay the brush footprint anchored at `cell` (top-left), as one block. Cells
  // already in the path, hinted, or off-board are clipped out; the block keeps
  // only the fresh cells, in row-major internal order. A 1×1 brush appends a
  // single-cell block, reproducing the classic behaviour. Dragging extends the
  // most recent block when the brush is 1×1; with a larger brush each press
  // stamps a new block (drag-painting whole blocks would overlap confusingly).
  const stampBlock = useCallback(
    (cell: number) => {
      setBlockGroups((prev) => {
        const placed = new Set(prev.flat());
        const ax = cell % size;
        const ay = Math.floor(cell / size);
        const fresh: number[] = [];
        for (let dy = 0; dy < brush.h; dy++) {
          for (let dx = 0; dx < brush.w; dx++) {
            const x = ax + dx;
            const y = ay + dy;
            if (x >= size || y >= size) continue;
            const c = y * size + x;
            if (placed.has(c) || hintCells.has(c)) continue;
            fresh.push(c);
          }
        }
        if (fresh.length === 0) return prev;
        return [...prev, fresh];
      });
    },
    [size, brush, hintCells],
  );

  // Continue a 1×1 drag stroke by one cell. Each dragged cell is appended as its
  // OWN single-cell (1×1) block — NOT merged into one big group. A 1×1 stroke is
  // a plain search path, so it must stay a run of singleton blocks: that keeps
  // `blockMode` false (it triggers only on a group bigger than one cell), so the
  // grid labels cells by visit rank (1, 2, 3, …) and colours them by rank rather
  // than collapsing the whole stroke into block 0 (which rendered "1" on every
  // dragged cell and flipped the grid into block-colouring mid-drag).
  const extendLastBlock = useCallback(
    (cell: number) => {
      setBlockGroups((prev) => {
        const placed = new Set(prev.flat());
        if (placed.has(cell) || hintCells.has(cell)) return prev;
        return [...prev, [cell]];
      });
    },
    [hintCells],
  );

  // Right-click a cell to cut the path there: drop the block containing it and
  // every block after it.
  const truncateFrom = (cell: number) => {
    setBlockGroups((prev) => {
      const i = prev.findIndex((g) => g.includes(cell));
      return i >= 0 ? prev.slice(0, i) : prev;
    });
  };

  const loadDefault = (kind: string) => {
    // A built-in path is a sequence of single-cell (1×1) blocks.
    const cells = Array.from(getPath(kind, size, size, seed)).filter((c) => !hintCells.has(c));
    setBlockGroups(cells.map((c) => [c]));
  };

  const completeRowMajor = () => {
    setBlockGroups((prev) => {
      const seen = new Set(prev.flat());
      const rest: number[][] = [];
      for (let c = 0; c < n; c++) if (!seen.has(c) && !hintCells.has(c)) rest.push([c]);
      return [...prev, ...rest];
    });
  };

  // Marking a cell as a hint removes it from the drawn path: a hint is a known
  // piece, so it is not part of the order the solver must work out. Drop it from
  // whichever block holds it (and drop now-empty blocks).
  const toggleHint = useCallback((cell: number) => {
    setHintCells((prev) => {
      const next = new Set(prev);
      if (next.has(cell)) next.delete(cell);
      else next.add(cell);
      return next;
    });
    setBlockGroups((prev) =>
      prev.map((g) => g.filter((c) => c !== cell)).filter((g) => g.length > 0),
    );
  }, []);

  useEffect(() => {
    // Size changes the cell count, so both path and hints must reset. This is
    // a reset-on-key-change effect that ALSO frees the running race's WASM
    // solvers (stopRace) — that teardown can't happen during render, so the
    // accompanying state resets live here too.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setBlockGroups([]);
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
    const solved = getGeneratedSolvedPuzzleFramed(size, colors, seed, framed);
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

  const startRace = (
    laneSpecs: { id: string; label: string; path: Uint16Array; blocks?: number[][] }[],
  ) => {
    stopRace();
    const puzzle = getGeneratedPuzzleFramed(size, colors, seed, framed);
    // The single-piece lanes honour the user's hints. A block lane partitions
    // the whole board and can't pre-pin hint cells, so it runs hint-free — but
    // that must not strip hints from the OTHER lanes, so give the block solver
    // its own hint-free puzzle clone rather than mutating the shared one.
    puzzle.hints = buildHints(puzzle, hintCells);
    puzzleRef.current = puzzle;
    const useHints = puzzle.hints.length > 0;
    const blockPuzzle = { ...puzzle, hints: [] as typeof puzzle.hints };
    // The engine wants a full-coverage path (it pre-fills hint cells and
    // skips them). Append any cells the visible path omits — the hints first,
    // then any stragglers — so every lane's path is a permutation of all n.
    const completePath = (path: Uint16Array): Uint16Array => {
      const seen = new Set(path);
      const full = Array.from(path);
      for (let c = 0; c < n; c++) if (!seen.has(c)) full.push(c);
      return Uint16Array.from(full);
    };
    // Complete a block list into a partition of all n cells: any cell not in a
    // drawn block becomes its own 1×1 block, appended in row-major order.
    const completeBlocks = (blocks: number[][]): Block[] => {
      const seen = new Set(blocks.flat());
      const out: Block[] = blocks.map((cells) => ({ cells }));
      for (let c = 0; c < n; c++) if (!seen.has(c)) out.push({ cells: [c] });
      return out;
    };
    const newLanes: Lane[] = laneSpecs.map((spec) => {
      const solver = spec.blocks
        ? createBlockSolver(blockPuzzle, completeBlocks(spec.blocks))
        : createSolver(puzzle, completePath(spec.path), { useHints });
      solversRef.current.set(spec.id, solver);
      return {
        id: spec.id,
        label: spec.label,
        path: spec.path,
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
            // Rank on `attempts` (per-cell fit checks), which is commensurable
            // across the single-piece and block solvers — unlike `nodes`, which
            // counts pieces in one and committed blocks in the other.
            finishedAttempts: r.attempts,
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
            <Button variant="outline" size="sm" onClick={() => setBlockGroups([])}>
              {t.clear}
            </Button>
            <Button variant="outline" size="sm" onClick={completeRowMajor} disabled={customComplete}>
              {t.autoFinish}
            </Button>
            <Badge variant={customComplete ? "default" : "secondary"}>
              {order.length}/{pathTarget}
            </Badge>
            {blockMode && (
              <Badge variant="outline" className="border-primary text-primary">
                {t.blocksBadge(blockGroups.length)}
              </Badge>
            )}
            {hintCells.size > 0 && (
              <Badge variant="outline" className="border-amber-500 text-amber-600">
                📌 {t.hintsBadge(hintCells.size)}
              </Badge>
            )}
          </div>

          <div className="inline-flex rounded-md border p-0.5 text-sm">
            <button
              onClick={() => setMode("path")}
              aria-pressed={mode === "path"}
              className={cn(
                "rounded px-3 py-1 font-medium transition-colors",
                mode === "path" ? "bg-primary text-primary-foreground" : "text-muted-foreground",
              )}
            >
              {t.modePath}
            </button>
            <button
              onClick={() => setMode("hint")}
              aria-pressed={mode === "hint"}
              className={cn(
                "rounded px-3 py-1 font-medium transition-colors",
                mode === "hint" ? "bg-amber-400 text-amber-950" : "text-muted-foreground",
              )}
            >
              📌 {t.modeHint}
            </button>
          </div>

          {mode === "path" && (
            <div className="flex flex-wrap items-center gap-1.5">
              <span className="text-xs font-medium text-muted-foreground">{t.brushLabel}:</span>
              {BRUSH_SHAPES.map((s) => {
                const active = brush.w === s.w && brush.h === s.h;
                return (
                  <button
                    key={`${s.w}x${s.h}`}
                    onClick={() => setBrush({ w: s.w, h: s.h })}
                    aria-pressed={active}
                    aria-label={`${t.brushLabel} ${s.w}×${s.h}`}
                    className={cn(
                      "rounded border px-2 py-1 text-xs font-medium tabular-nums transition-colors",
                      active
                        ? "border-primary bg-primary text-primary-foreground"
                        : "text-muted-foreground hover:bg-muted",
                    )}
                  >
                    {s.w}×{s.h}
                  </button>
                );
              })}
              {blockMode && (
                <Badge variant="outline" className="border-primary text-primary" title={t.blockModeHelp}>
                  {t.blockMode}
                </Badge>
              )}
            </div>
          )}

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
              stampBlock(c);
            }}
            onPointerMove={(e) => {
              if (!drawing || mode === "hint") return;
              // Drag-painting only extends the stroke for the 1×1 brush; with a
              // block brush each press stamps one block (dragging would overlap).
              if (brush.w !== 1 || brush.h !== 1) return;
              const el = document.elementFromPoint(e.clientX, e.clientY) as HTMLElement | null;
              const cell = el?.dataset["cell"];
              if (cell !== undefined && !hintCells.has(parseInt(cell, 10))) {
                extendLastBlock(parseInt(cell, 10));
              }
            }}
            onPointerUp={() => setDrawing(false)}
            onPointerLeave={() => setDrawing(false)}
            onPointerCancel={() => setDrawing(false)}
          >
            {Array.from({ length: n }, (_, cell) => {
              const rank = rankOf.get(cell);
              const bi = blockOf.get(cell);
              const isHint = hintCells.has(cell);
              // In block mode, colour each cell by its block (so a block reads as
              // one tile) and label it with the block number. Otherwise colour by
              // visit rank as before.
              const bg =
                !isHint && rank !== undefined
                  ? blockMode && bi !== undefined
                    ? blockColor(bi)
                    : rankColor(rank, n)
                  : undefined;
              const label = isHint
                ? "📌"
                : rank === undefined
                  ? ""
                  : blockMode && bi !== undefined
                    ? size <= 12
                      ? bi + 1
                      : ""
                    : size <= 9
                      ? rank + 1
                      : "";
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
                  style={bg ? { backgroundColor: bg } : undefined}
                >
                  {label}
                </div>
              );
            })}
          </div>
          <p className="text-xs text-muted-foreground">
            {mode === "hint"
              ? t.gridHelpHint
              : brush.w === 1 && brush.h === 1
                ? t.gridHelp
                : t.gridHelpBlock}
          </p>

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
                    aria-label={t.colorsLabel(colors)}
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
              <div className="space-y-1.5">
                <div className="flex items-center gap-2">
                  <Switch
                    id="paths-framed"
                    checked={framed}
                    disabled={size < 4 || colors < 2}
                    onCheckedChange={setFramed}
                  />
                  <Label htmlFor="paths-framed" className="cursor-pointer">
                    {t.framed}
                  </Label>
                </div>
                <p className="text-xs text-muted-foreground">{t.framedHint}</p>
              </div>
              <div className="flex flex-wrap gap-2">
                <Button
                  disabled={!engineReady || racing}
                  onClick={() => {
                    const specs = [
                      ...(customComplete
                        ? [
                            {
                              id: "custom",
                              label: blockMode ? `${t.yourPath} (${t.blockMode})` : t.yourPath,
                              path: Uint16Array.from(order),
                              // In block mode the custom lane runs the macro-piece
                              // solver over the drawn blocks.
                              ...(blockMode ? { blocks: blockGroups } : {}),
                            },
                          ]
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

          <PathComplexity
            size={size}
            colors={colors}
            seed={seed}
            order={order}
            framed={framed}
            hints={Array.from(hintCells)}
          />

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
