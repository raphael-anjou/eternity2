import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import { rotateEdges } from "@/lib/types";
import type { Edges } from "@/lib/bucas";
import { pieceFitsAt } from "@/lib/piece-fit";
import { BoardSvg } from "@/components/board/BoardSvg";
import { PieceSvg } from "@/components/board/PieceSvg";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { yieldToBrowser } from "@/lib/useRunWhileVisible";

// Piece theft, live. We fill a small board up to a frontier cell, read the
// (north, west) colours that cell now demands, and ask the real legality test
// which unused pieces can serve it. When exactly one can — a sole server — we let
// you "steal" it: place that piece somewhere else instead. The frontier cell's
// server count drops to zero and the board is dead, even though pieces remain.
// That is exactly how solvers die: a future cell starves because its only
// supplier was spent elsewhere.

interface Built {
  puzzle: Puzzle;
  board: number[];
  frontier: number;
  demandN: number; // north colour demanded
  demandW: number; // west colour demanded
  servers: number[]; // piece ids that can serve the frontier
}

function colorAt(cells: (Edges | null)[], pos: number, side: number): number {
  const c = cells[pos];
  return c ? (c[side] ?? -1) : -1;
}

const T = {
  en: {
    title: "Watch a cell starve — live",
    intro:
      "We fill a small board up to the red cell, then read what it now needs: a piece whose top matches the cell above and whose left matches the cell beside it. The engine lists every unused piece that can serve that demand. When only one can, you can 'steal' it — place it elsewhere — and watch the red cell's options drop to zero. The board still has pieces, but it can no longer be finished.",
    demand: (n: number, w: number) => `The red cell demands: top = colour ${n}, left = colour ${w}`,
    servers: (k: number) => `${k} piece${k === 1 ? "" : "s"} can serve it`,
    soleServer: "Exactly one server — a single point of failure.",
    multiServer: "More than one server here — try a new puzzle to find a sole-server cell.",
    steal: "Steal the only server (place it elsewhere)",
    stolen: "Stolen — the red cell now has 0 servers. The board is dead, but the box still looks full.",
    reset: "Reset",
    newPuzzle: "New puzzle",
    supply: "Piece supply",
    supplyHint: "Green = can serve the red cell. Red ring = the sole server. Dimmed = already placed.",
    loading: "Setting up a puzzle with the engine…",
    note: "On the real 16×16, 47 distinct (north, west) demands are served by a single piece, and a cell needs about 3 servers on average. The legality test here is the engine's. Spend a sole server in the wrong place and a future cell dies with the box still full — the trap real solvers fall into deep in the search.",
  },
  fr: {
    title: "Regardez une case mourir de faim — en direct",
    intro:
      "On remplit un petit plateau jusqu'à la case rouge, puis on lit ce qu'elle réclame : une pièce dont le haut s'accorde à la case du dessus et la gauche à la case d'à côté. Le moteur liste chaque pièce inutilisée pouvant servir cette demande. Quand une seule le peut, vous pouvez la « voler » — la poser ailleurs — et voir les options de la case rouge tomber à zéro. Le plateau a encore des pièces, mais ne peut plus être terminé.",
    demand: (n: number, w: number) => `La case rouge réclame : haut = couleur ${n}, gauche = couleur ${w}`,
    servers: (k: number) => `${k} pièce${k === 1 ? "" : "s"} peu${k === 1 ? "t" : "vent"} la servir`,
    soleServer: "Exactement un fournisseur — un point de défaillance unique.",
    multiServer: "Plus d'un fournisseur ici — essayez un nouveau tirage pour trouver une case à fournisseur unique.",
    steal: "Voler l'unique fournisseur (le poser ailleurs)",
    stolen: "Volé — la case rouge a maintenant 0 fournisseur. Le plateau est mort, mais la boîte semble pleine.",
    reset: "Réinitialiser",
    newPuzzle: "Nouveau tirage",
    supply: "Réserve de pièces",
    supplyHint: "Vert = peut servir la case rouge. Anneau rouge = l'unique fournisseur. Estompé = déjà posée.",
    loading: "Préparation d'un puzzle avec le moteur…",
    note: "Sur le vrai 16×16, 47 demandes (haut, gauche) distinctes sont servies par une seule pièce, et une case a besoin d'environ 3 fournisseurs en moyenne. Le test de légalité ici est celui du moteur. Dépensez un fournisseur unique au mauvais endroit et une case future meurt alors que la boîte est pleine — le piège où tombent les vrais solveurs au fond de la recherche.",
  },
};

export function PieceTheftLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [seed, setSeed] = useState(4);
  const [built, setBuilt] = useState<Built | null>(null);
  const [stolen, setStolen] = useState(false);
  const seedRef = useRef(seed);

  const SIZE = 6;
  const COLORS = 5;

  const build = useCallback(async (token: { cancelled: boolean }) => {
    if (!engineReady) return;
    const cancelled = () => token.cancelled;
    // Look for a seed/fill where the frontier cell has exactly one server.
    // Each (seed, fill) probe is a small bounded solve; we yield to the browser
    // between probes so the hunt never blocks clicks or navigation.
    for (let attempt = 0; attempt < 14; attempt++) {
      const s = (seedRef.current + attempt * 37) % 100000;
      const puzzle = getGeneratedPuzzle(SIZE, COLORS, s);
      const path = getPath("row-major", puzzle.width, puzzle.height, 0);
      // try several fill depths to hit a sole-server frontier
      for (const fill of [10, 12, 14, 16, 8, 18]) {
        await yieldToBrowser();
        if (cancelled()) return;
        const solver = createSolver(puzzle, path, { useHints: false });
        let report = solver.report();
        for (let g = 0; g < 5000 && report.placed < fill; g++) {
          report = solver.step(1);
          if (report.status !== "running") break;
        }
        const board = Array.from(solver.board());
        solver.free();
        if (report.placed >= puzzle.width * puzzle.height) continue;
        const frontier = path[report.placed] ?? 0;
        const cells = boardFromEngine(puzzle, board).cells;
        const used = new Set(board.filter((v) => v >= 0).map((v) => v >> 2));
        const x = frontier % puzzle.width;
        const y = Math.floor(frontier / puzzle.width);
        if (x === 0 || y === 0) continue; // need top & left present to read demand
        const demandN = colorAt(cells, frontier - puzzle.width, 2); // above's bottom
        const demandW = colorAt(cells, frontier - 1, 1); // left's right
        const servers: number[] = [];
        puzzle.pieces.forEach((piece, i) => {
          if (used.has(i)) return;
          if (pieceFitsAt(piece, frontier, cells, puzzle.width, puzzle.height)) servers.push(i);
        });
        if (servers.length === 1) {
          setBuilt({ puzzle, board, frontier, demandN, demandW, servers });
          setStolen(false);
          return;
        }
      }
    }
    // fallback: accept whatever the first config gives (multi-server)
    await yieldToBrowser();
    if (cancelled()) return;
    const puzzle = getGeneratedPuzzle(SIZE, COLORS, seedRef.current);
    const path = getPath("row-major", puzzle.width, puzzle.height, 0);
    const solver = createSolver(puzzle, path, { useHints: false });
    let report = solver.report();
    for (let g = 0; g < 5000 && report.placed < 12; g++) {
      report = solver.step(1);
      if (report.status !== "running") break;
    }
    const board = Array.from(solver.board());
    solver.free();
    const frontier = path[report.placed] ?? 0;
    const cells = boardFromEngine(puzzle, board).cells;
    const used = new Set(board.filter((v) => v >= 0).map((v) => v >> 2));
    const servers: number[] = [];
    puzzle.pieces.forEach((piece, i) => {
      if (!used.has(i) && pieceFitsAt(piece, frontier, cells, puzzle.width, puzzle.height))
        servers.push(i);
    });
    setBuilt({
      puzzle,
      board,
      frontier,
      demandN: colorAt(cells, frontier - puzzle.width, 2),
      demandW: colorAt(cells, frontier - 1, 1),
      servers,
    });
    setStolen(false);
  }, [engineReady]);

  useEffect(() => {
    seedRef.current = seed;
    // Cancellation token: flipped on unmount or seed change so no in-flight
    // search loop survives navigation.
    const token = { cancelled: false };
    void build(token);
    return () => {
      token.cancelled = true;
    };
  }, [build, seed]);

  const display = useMemo(() => {
    if (!built) return null;
    if (!stolen) return built.board;
    // Steal the sole server: blank the frontier's would-be supply by placing the
    // server piece at a far empty cell (any empty cell that isn't the frontier).
    const board = [...built.board];
    const empty = board.findIndex((v, i) => v < 0 && i !== built.frontier);
    const server = built.servers[0];
    if (empty >= 0 && server !== undefined) {
      // place with rotation 0 — it's a wrong placement, the point is it's spent
      board[empty] = server * 4;
    }
    return board;
  }, [built, stolen]);

  if (!engineReady || !built || !display) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const cells = boardFromEngine(built.puzzle, display).cells;
  const used = new Set(display.filter((v) => v >= 0).map((v) => v >> 2));
  const soleServer = built.servers.length === 1;
  const liveServers = stolen ? 0 : built.servers.length;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-56 space-y-2">
          <BoardSvg
            width={built.puzzle.width}
            height={built.puzzle.height}
            cells={cells}
            highlight={[built.frontier]}
          />
          <div className="space-y-1 text-center text-xs">
            <p className="text-muted-foreground">{t.demand(built.demandN, built.demandW)}</p>
            <p
              className={cn(
                "rounded-md border px-2 py-1 font-medium",
                stolen
                  ? "border-red-300 bg-red-500/10 text-red-600 dark:text-red-300"
                  : soleServer
                    ? "border-amber-300 bg-amber-500/10"
                    : "border-muted",
              )}
            >
              {t.servers(liveServers)}
              {stolen ? " · " + t.stolen : soleServer ? " · " + t.soleServer : " · " + t.multiServer}
            </p>
          </div>
        </div>

        <div className="space-y-1.5">
          <p className="text-xs font-medium">{t.supply}</p>
          <div className="grid w-fit gap-1" style={{ gridTemplateColumns: "repeat(7, max-content)" }}>
            {built.puzzle.pieces.map((piece, i) => {
              const isServer = built.servers.includes(i);
              return (
                <div
                  key={i}
                  className={cn(
                    "rounded p-0.5 transition-all",
                    used.has(i) && "opacity-20",
                    isServer && !stolen && "ring-2 ring-emerald-500",
                    isServer && soleServer && "ring-2 ring-red-500",
                  )}
                >
                  <PieceSvg edges={rotateEdges(piece, 0)} size={20} />
                </div>
              );
            })}
          </div>
          <p className="max-w-56 text-xs text-muted-foreground">{t.supplyHint}</p>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        {soleServer && (
          <Button size="sm" onClick={() => setStolen((v) => !v)} disabled={stolen}>
            {t.steal}
          </Button>
        )}
        <Button size="sm" variant="outline" onClick={() => setStolen(false)}>
          {t.reset}
        </Button>
        <Button size="sm" variant="outline" onClick={() => setSeed((s) => (s * 7 + 13) % 100000)}>
          {t.newPuzzle}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
