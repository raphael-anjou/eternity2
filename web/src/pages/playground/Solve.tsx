import { pageMeta } from "@/seo";
// Solve a small puzzle yourself: pieces in a tray on the right, board on the
// left, timer running. When you finish, the WASM engine solves the same
// puzzle and tells you how many times it could have done it meanwhile.
//
// Drag and drop uses an HTML overlay grid on top of the SVG board (HTML5
// drag sources/targets are unreliable on SVG elements). Pieces can be
// dragged tray→board, board→board (swap), and board→tray (take back).

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { PieceSvg } from "@/components/board/PieceSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import { useT } from "@/i18n";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import type { Puzzle } from "@/lib/types";
import { rotateEdges, BORDER } from "@/lib/types";
import type { Edges } from "@/lib/bucas";
import { conflictEdges } from "@/lib/bucas";
import { formatSeconds, formatInt } from "@/lib/format";

const T = {
  en: {
    title: "Solve it yourself",
    levelsLabel: "Level",
    levels: [
      { size: 3, name: "Warm-up", note: "3×3, gentle" },
      { size: 4, name: "Getting tricky", note: "4×4" },
      { size: 5, name: "Real challenge", note: "5×5" },
    ],
    lockedLevel: {
      name: "The real one",
      note: "16×16 — never solved",
    },
    lockedTitle: "Nobody has ever finished this level",
    lockedBody: (
      <>
        The puzzle Tomy actually sold is 16×16. No person and no computer has ever placed all 256
        pieces. See{" "}
        <Link className="underline" to="/status">
          where the record stands
        </Link>
        .
      </>
    ),
    intro:
      "Pick a level, then drag pieces from the tray onto the board; drag them around the board or back to the tray to rearrange. Click a placed piece to rotate it; right-click to take it back. Grey edges go on the outside, and touching edges must match. The clock starts on your first move…",
    introTouch:
      "Tap a piece in the tray, then tap a square to place it. Tap a placed piece to rotate it; press and hold to take it back. Grey edges go on the outside, and touching edges must match. The clock starts on your first move…",
    trayHintTouch: "Now tap a square to place it.",
    newPuzzle: "New puzzle",
    mistakes: (n: number) => (n === 1 ? "1 mistake" : `${n} mistakes`),
    loadingEngine: "Loading engine…",
    piecesLeft: (n: number) => `Pieces (${n} left)`,
    pieceTitle: (n: number) => `piece ${n}`,
    trayHint: "Now drag it onto the board. Pieces land unrotated; click a placed piece to rotate it.",
    candidateHint: (n: number) =>
      n === 0
        ? "This piece fits nowhere right now (in any rotation)."
        : n === 1
          ? "The green cell is the only spot this piece fits, in some rotation."
          : `Green cells are the ${n} spots this piece could fit, in some rotation.`,
    solvedIn: (time: string) => `🎉 Solved in ${time}!`,
    machineSolved: (ms: string, attempts: string) => (
      <>
        The computer solved the same puzzle in <strong>{ms} ms</strong> ({attempts} nodes
        explored).
      </>
    ),
    couldHave: (times: string) => (
      <>
        While you played, it could have solved this puzzle about{" "}
        <strong className="text-lg">{times}×</strong>.
      </>
    ),
    beforeSecondPiece: (
      <>
        It solved the whole puzzle{" "}
        <strong>before you had even placed your second piece</strong>.
      </>
    ),
    exponential: (
      <>
        And yet nobody, human or machine, has ever solved the 16×16 one. That's the magic of
        exponential growth. See{" "}
        <Link className="underline" to="/algorithms">
          Algorithms
        </Link>
        .
      </>
    ),
    racing: "Racing the machine…",
  },
  fr: {
    title: "À vous de jouer",
    levelsLabel: "Niveau",
    levels: [
      { size: 3, name: "Échauffement", note: "3×3, tout doux" },
      { size: 4, name: "Ça se corse", note: "4×4" },
      { size: 5, name: "Vrai défi", note: "5×5" },
    ],
    lockedLevel: {
      name: "Le vrai",
      note: "16×16 — jamais résolu",
    },
    lockedTitle: "Personne n'a jamais bouclé ce niveau",
    lockedBody: (
      <>
        Le puzzle vendu par Tomy fait 16×16. Ni un humain ni un ordinateur n'a jamais posé les 256
        pièces. Voyez{" "}
        <Link className="underline" to="/status">
          où en est le record
        </Link>
        .
      </>
    ),
    intro:
      "Choisissez un niveau, puis glissez les pièces de la réserve vers le plateau ; déplacez-les sur le plateau ou ramenez-les dans la réserve pour les réorganiser. Cliquez sur une pièce posée pour la faire pivoter ; faites un clic droit pour la retirer. Les côtés gris se placent à l'extérieur, et deux côtés qui se touchent doivent correspondre. Le chrono se lance dès votre premier coup…",
    introTouch:
      "Touchez une pièce dans la réserve, puis une case pour la poser. Touchez une pièce posée pour la faire pivoter ; appuyez longuement pour la retirer. Les côtés gris se placent à l'extérieur, et deux côtés qui se touchent doivent correspondre. Le chrono se lance dès votre premier coup…",
    trayHintTouch: "Touchez maintenant une case pour la poser.",
    newPuzzle: "Nouveau puzzle",
    mistakes: (n: number) => (n === 1 ? "1 erreur" : `${n} erreurs`),
    loadingEngine: "Chargement du moteur…",
    piecesLeft: (n: number) => `Pièces (${n} restante${n === 1 ? "" : "s"})`,
    pieceTitle: (n: number) => `pièce ${n}`,
    trayHint:
      "Glissez-la maintenant sur le plateau. Les pièces se posent sans rotation ; cliquez sur une pièce posée pour la faire pivoter.",
    candidateHint: (n: number) =>
      n === 0
        ? "Cette pièce ne rentre nulle part pour l'instant (quelle que soit la rotation)."
        : n === 1
          ? "La case verte est le seul endroit où cette pièce rentre, dans une certaine rotation."
          : `Les cases vertes sont les ${n} endroits où cette pièce pourrait rentrer, dans une certaine rotation.`,
    solvedIn: (time: string) => `🎉 Résolu en ${time} !`,
    machineSolved: (ms: string, attempts: string) => (
      <>
        L'ordinateur a résolu le même puzzle en <strong>{ms} ms</strong> (en explorant {attempts}{" "}
        nœuds).
      </>
    ),
    couldHave: (times: string) => (
      <>
        Pendant que vous jouiez, il aurait eu le temps de le résoudre environ{" "}
        <strong className="text-lg">{times} fois</strong>.
      </>
    ),
    beforeSecondPiece: (
      <>
        Il avait terminé le puzzle{" "}
        <strong>avant même que vous ayez posé votre deuxième pièce</strong>.
      </>
    ),
    exponential: (
      <>
        Et pourtant, personne — ni humain ni machine — n'a jamais résolu la version 16×16. Toute la
        magie de la croissance exponentielle est là. Voir{" "}
        <Link className="underline" to="/algorithms">
          Algorithmes
        </Link>
        .
      </>
    ),
    racing: "La machine s'élance…",
  },
  es: {
    title: "Resuélvelo tú mismo",
    levelsLabel: "Nivel",
    levels: [
      { size: 3, name: "Calentamiento", note: "3×3, suave" },
      { size: 4, name: "Se complica", note: "4×4" },
      { size: 5, name: "Reto de verdad", note: "5×5" },
    ],
    lockedLevel: {
      name: "El de verdad",
      note: "16×16 — nunca resuelto",
    },
    lockedTitle: "Nadie ha terminado nunca este nivel",
    lockedBody: (
      <>
        El puzzle que Tomy realmente vendió es de 16×16. Ni una persona ni una computadora han
        colocado nunca las 256 piezas. Mira{" "}
        <Link className="underline" to="/status">
          en qué punto está el récord
        </Link>
        .
      </>
    ),
    intro:
      "Elige un nivel y arrastra las piezas desde la bandeja hasta el tablero; muévelas por el tablero o devuélvelas a la bandeja para reorganizarlas. Haz clic en una pieza colocada para girarla; haz clic derecho para retirarla. Las aristas grises van por fuera, y las aristas que se tocan deben coincidir. El reloj arranca con tu primer movimiento…",
    introTouch:
      "Toca una pieza de la bandeja y luego toca una casilla para colocarla. Toca una pieza colocada para girarla; mantén pulsado para retirarla. Las aristas grises van por fuera, y las aristas que se tocan deben coincidir. El reloj arranca con tu primer movimiento…",
    trayHintTouch: "Ahora toca una casilla para colocarla.",
    newPuzzle: "Nuevo puzzle",
    mistakes: (n: number) => (n === 1 ? "1 error" : `${n} errores`),
    loadingEngine: "Cargando el motor…",
    piecesLeft: (n: number) => `Piezas (${n} restante${n === 1 ? "" : "s"})`,
    pieceTitle: (n: number) => `pieza ${n}`,
    trayHint:
      "Ahora arrástrala hasta el tablero. Las piezas caen sin girar; haz clic en una pieza colocada para girarla.",
    candidateHint: (n: number) =>
      n === 0
        ? "Esta pieza no encaja en ningún sitio ahora mismo (en ninguna rotación)."
        : n === 1
          ? "La celda verde es el único sitio donde esta pieza encaja, en alguna rotación."
          : `Las celdas verdes son los ${n} sitios donde esta pieza podría encajar, en alguna rotación.`,
    solvedIn: (time: string) => `🎉 ¡Resuelto en ${time}!`,
    machineSolved: (ms: string, attempts: string) => (
      <>
        La computadora resolvió el mismo puzzle en <strong>{ms} ms</strong> ({attempts} nodos
        explorados).
      </>
    ),
    couldHave: (times: string) => (
      <>
        Mientras jugabas, habría podido resolver este puzzle unas{" "}
        <strong className="text-lg">{times}×</strong>.
      </>
    ),
    beforeSecondPiece: (
      <>
        Resolvió el puzzle entero{" "}
        <strong>antes de que hubieras colocado siquiera tu segunda pieza</strong>.
      </>
    ),
    exponential: (
      <>
        Y aun así nadie, ni humano ni máquina, ha resuelto jamás el de 16×16. Esa es la magia del
        crecimiento exponencial. Mira{" "}
        <Link className="underline" to="/algorithms">
          Algoritmos
        </Link>
        .
      </>
    ),
    racing: "Compitiendo contra la máquina…",
  },
};

interface Placement {
  piece: number;
  rot: number;
}

interface MachineResult {
  ms: number;
  attempts: number;
}

export default function Solve() {
  const t = useT(T);
  const engineReady = useEngine();
  // Touch devices get tap-centric instructions and long-press removal
  // (HTML5 drag and drop does not exist on mobile browsers).
  const coarsePointer = useMemo(
    () => typeof window !== "undefined" && window.matchMedia("(pointer: coarse)").matches,
    [],
  );
  const longPress = useRef<{ timer: number; x: number; y: number } | null>(null);
  const [size, setSize] = useState(3);
  const [seed, setSeed] = useState(() => Math.floor(Math.random() * 100000));
  // Seed with a board sized to the initial `size`: the only thing that resizes
  // it afterwards is reset(), which fills a fresh empty board itself. Without
  // this the overlay grid maps over an empty array on first mount and renders
  // zero drop targets, so drag-and-drop does nothing until you click a size.
  const [board, setBoard] = useState<(Placement | null)[]>(() =>
    Array<Placement | null>(3 * 3).fill(null),
  );
  const [selected, setSelected] = useState<number | null>(null);
  const [showLocked, setShowLocked] = useState(false);
  const [startedAt, setStartedAt] = useState<number | null>(null);
  const [elapsed, setElapsed] = useState(0);
  const [finishedIn, setFinishedIn] = useState<number | null>(null);
  const [machine, setMachine] = useState<MachineResult | null>(null);

  const puzzle: Puzzle | null = useMemo(
    () => (engineReady ? getGeneratedPuzzle(size, Math.min(size + 1, 8), seed) : null),
    [engineReady, size, seed],
  );

  const reset = useCallback((newSize: number) => {
    setSize(newSize);
    setSeed(Math.floor(Math.random() * 100000));
    setBoard(Array(newSize * newSize).fill(null));
    setSelected(null);
    setStartedAt(null);
    setElapsed(0);
    setFinishedIn(null);
    setMachine(null);
  }, []);

  // No effect to clear the board on size/seed change: `reset()` is the only
  // thing that changes either, and it already sizes a fresh empty board.

  // ticking clock
  useEffect(() => {
    if (startedAt === null || finishedIn !== null) return;
    const id = setInterval(() => setElapsed((Date.now() - startedAt) / 1000), 250);
    return () => clearInterval(id);
  }, [startedAt, finishedIn]);

  const usedPieces = useMemo(() => {
    const used = new Set<number>();
    board.forEach((p) => p && used.add(p.piece));
    return used;
  }, [board]);

  const cells: (Edges | null)[] = useMemo(() => {
    if (!puzzle) return [];
    return board.map((p) => {
      if (!p) return null;
      const piece = puzzle.pieces[p.piece];
      return piece ? rotateEdges(piece, p.rot) : null;
    });
  }, [board, puzzle]);

  // What-if helper: while a tray piece is selected, which empty cells could it
  // legally go into, in SOME rotation, given the pieces already on the board and
  // the grey-rim rules? This is the "suggest pieces per position" aid the
  // by-hand community keeps asking for, run in reverse (one piece, many cells).
  const candidateCells = useMemo(() => {
    const out = new Set<number>();
    if (selected === null || !puzzle) return out;
    const piece = puzzle.pieces[selected];
    if (!piece || usedPieces.has(selected)) return out;
    const w = puzzle.width;
    const h = puzzle.height;
    for (let pos = 0; pos < board.length; pos++) {
      if (board[pos]) continue; // only empty cells
      const x = pos % w;
      const y = Math.floor(pos / w);
      // A cell edge facing the rim must be grey (BORDER); an interior edge must
      // match the touching edge of a placed neighbour, if one is there.
      const facesRim = [y === 0, x === w - 1, y === h - 1, x === 0];
      const neighbourPos = [
        y === 0 ? -1 : pos - w,
        x === w - 1 ? -1 : pos + 1,
        y === h - 1 ? -1 : pos + w,
        x === 0 ? -1 : pos - 1,
      ];
      const fits = [0, 1, 2, 3].some((rot) => {
        const e = rotateEdges(piece, rot);
        for (let d = 0; d < 4; d++) {
          if (facesRim[d]) {
            if (e[d] !== BORDER) return false; // rim edge must be grey
            continue;
          }
          if (e[d] === BORDER) return false; // interior edge must not be grey
          const np = neighbourPos[d] ?? -1;
          const ncell = np >= 0 ? cells[np] : null;
          if (ncell) {
            // the neighbour's edge that touches side d is the opposite side.
            if (ncell[(d + 2) % 4] !== e[d]) return false;
          }
        }
        return true;
      });
      if (fits) out.add(pos);
    }
    return out;
  }, [selected, puzzle, board, cells, usedPieces]);

  // Mistakes = mismatched piece-piece contacts PLUS misoriented rim pieces:
  // a non-grey edge facing the outside, or a grey edge facing inward, is
  // wrong even before its neighbors arrive.
  const allConflicts = useMemo(() => {
    if (!puzzle) return [] as [number, number][];
    const w = puzzle.width;
    const h = puzzle.height;
    const marks = new Map<string, [number, number]>();
    for (const [pos, dir] of conflictEdges({
      width: w,
      height: h,
      cells,
      pieceNumbers: null,
      hints: null,
      puzzleName: null,
    })) {
      marks.set(`${pos}:${dir}`, [pos, dir]);
    }
    cells.forEach((e, pos) => {
      if (!e) return;
      const x = pos % w;
      const y = Math.floor(pos / w);
      const facesRim = [y === 0, x === w - 1, y === h - 1, x === 0]; // URDL
      for (let d = 0; d < 4; d++) {
        if (facesRim[d] !== (e[d] === BORDER)) marks.set(`${pos}:${d}`, [pos, d]);
      }
    });
    return [...marks.values()];
  }, [cells, puzzle]);

  // Count each faulty edge once: a mismatched contact produces two half-edge
  // marks (one per piece) that collapse onto one canonical key; rim-facing
  // faults are their own key.
  const mistakeCount = useMemo(() => {
    if (!puzzle) return 0;
    const w = puzzle.width;
    const h = puzzle.height;
    const keys = new Set<string>();
    for (const [pos, d] of allConflicts) {
      const x = pos % w;
      const y = Math.floor(pos / w);
      const facesRim = [y === 0, x === w - 1, y === h - 1, x === 0][d];
      if (facesRim) {
        keys.add(`rim:${pos}:${d}`);
      } else {
        const npos = d === 0 ? pos - w : d === 1 ? pos + 1 : d === 2 ? pos + w : pos - 1;
        keys.add(`e:${Math.min(pos, npos)}:${Math.max(pos, npos)}`);
      }
    }
    return keys.size;
  }, [allConflicts, puzzle]);

  const complete =
    puzzle !== null && board.every(Boolean) && allConflicts.length === 0 && board.length > 0;

  // completion → stop the human clock. Kept separate from the machine-solve
  // effect below: this one just records elapsed time (cheap, synchronous), and
  // flipping `finishedIn` is what triggers the benchmark. Bundling them would
  // tear the benchmark's RAF loop down on the re-render that setFinishedIn
  // causes (the effect would re-run, hit the guard, and fire its cleanup).
  useEffect(() => {
    if (!complete || finishedIn !== null || startedAt === null) return;
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setFinishedIn((Date.now() - startedAt) / 1000);
  }, [complete, finishedIn, startedAt]);

  // Once the human is done, race the machine. The solve is stepped across RAF
  // frames in small (~12ms) chunks rather than one synchronous burst: a hard
  // puzzle could otherwise pin the main thread for up to the full 5s cap,
  // freezing the tab (and blocking navigation) right after the human finishes.
  // While it runs, `machine` stays null and the UI shows the "Racing the
  // machine…" message; we accumulate only the time spent inside step()
  // (excluding the idle gaps between frames) so the reported time stays
  // comparable to the old synchronous measurement. Guarded on machine === null
  // so it runs once per puzzle. This effect synchronizes with an external
  // system (the WASM solver), so its setState legitimately lives in the body.
  useEffect(() => {
    if (finishedIn === null || machine !== null || !puzzle) return;

    const path = getPath("row-major", puzzle.width, puzzle.height, 0);
    const solver = createSolver(puzzle, path, { useHints: false });
    let raf = 0;
    let solverMs = 0; // accumulated time spent inside step(), excluding idle gaps

    const frame = () => {
      const frameStart = performance.now();
      let r = solver.report();
      // Step in small node-budget chunks so the wall-clock check below is
      // actually enforced: step(budget) runs up to `budget` nodes SYNCHRONOUSLY
      // before returning, so a huge budget (e.g. 10_000_000) can pin the main
      // thread for hundreds of ms in a single call — past the 12ms slice — which
      // freezes the tab and blocks route changes. A modest chunk keeps each call
      // short enough that we re-check the clock often and yield after ~12ms.
      while (r.status === "running" && performance.now() - frameStart < 12) {
        r = solver.step(50_000);
      }
      solverMs += performance.now() - frameStart;
      if (r.status !== "running" || solverMs >= 5000) {
        if (r.status === "solved") setMachine({ ms: solverMs, attempts: r.nodes });
        return; // solver freed by the cleanup below
      }
      raf = requestAnimationFrame(frame);
    };
    raf = requestAnimationFrame(frame);

    return () => {
      cancelAnimationFrame(raf);
      solver.free();
    };
  }, [finishedIn, machine, puzzle]);

  // Start the human clock on first interaction. Wrapped so the linter sees it
  // as an event handler (not render code) — Date.now() is fine to call here.
  const touch = useCallback(() => {
    setStartedAt((prev) => (prev === null ? Date.now() : prev));
  }, []);

  const placePiece = (pos: number, piece: number) => {
    if (!puzzle || finishedIn !== null) return;
    touch();
    setBoard((prev) => {
      if (prev.some((p, i) => p?.piece === piece && i !== pos)) return prev;
      const next = [...prev];
      next[pos] = { piece, rot: 0 };
      return next;
    });
    setSelected(null);
  };

  /** Move a placed piece to another cell (swapping with any occupant). */
  const movePiece = (from: number, to: number) => {
    if (finishedIn !== null || from === to) return;
    touch();
    setBoard((prev) => {
      const moving = prev[from];
      if (!moving) return prev;
      const next = [...prev];
      next[to] = moving;
      next[from] = prev[to] ?? null;
      return next;
    });
  };

  const removePiece = (pos: number) => {
    if (finishedIn !== null) return;
    setBoard((prev) => {
      if (!prev[pos]) return prev;
      const next = [...prev];
      next[pos] = null;
      return next;
    });
  };

  const onCellClick = (pos: number) => {
    if (!puzzle || finishedIn !== null) return;
    if (selected !== null) {
      placePiece(pos, selected);
      return;
    }
    touch();
    setBoard((prev) => {
      const here = prev[pos];
      if (!here) return prev;
      const next = [...prev];
      next[pos] = { piece: here.piece, rot: (here.rot + 1) & 3 };
      return next;
    });
  };

  const handleDrop = (pos: number, data: string) => {
    if (data.startsWith("piece:")) {
      const piece = parseInt(data.slice(6), 10);
      if (!Number.isNaN(piece)) placePiece(pos, piece);
    } else if (data.startsWith("move:")) {
      const from = parseInt(data.slice(5), 10);
      if (!Number.isNaN(from)) movePiece(from, pos);
    }
  };

  // How the machine's solve time compares to yours, framed as a human image
  // instead of a division that explodes to infinity when ms ≈ 0.
  const comparison = useMemo(() => {
    if (!machine || finishedIn === null || !puzzle) return null;
    const yourMs = finishedIn * 1000;
    const perPieceMs = yourMs / (puzzle.width * puzzle.height); // your avg time per piece
    // How many of YOUR pieces' worth of time the machine fit into.
    const piecesWorth = machine.ms / perPieceMs;
    if (piecesWorth < 2) return { kind: "beforeSecondPiece" as const };
    const times = Math.floor(yourMs / machine.ms);
    if (times >= 2) return { kind: "times" as const, n: times };
    return { kind: "beforeSecondPiece" as const };
  }, [machine, finishedIn, puzzle]);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 text-muted-foreground">{coarsePointer ? t.introTouch : t.intro}</p>
      </div>

      <div className="flex flex-wrap items-center gap-2">
        {t.levels.map((lvl, i) => (
          <Button
            key={lvl.size}
            variant={lvl.size === size ? "default" : "outline"}
            onClick={() => {
              setShowLocked(false);
              reset(lvl.size);
            }}
            className="h-auto flex-col items-start gap-0.5 py-2"
          >
            <span className="text-xs font-normal opacity-70">
              {t.levelsLabel} {i + 1}
            </span>
            <span className="font-semibold">{lvl.name}</span>
            <span className="text-xs font-normal opacity-70">{lvl.note}</span>
          </Button>
        ))}
        {/* The aspirational tier: a real level you can select, but never win.
            Clicking it teaches the whole point of the site. */}
        <Button
          variant={showLocked ? "default" : "outline"}
          onClick={() => setShowLocked((v) => !v)}
          className="h-auto flex-col items-start gap-0.5 border-dashed py-2"
        >
          <span className="text-xs font-normal opacity-70">🔒 {t.levelsLabel} ∞</span>
          <span className="font-semibold">{t.lockedLevel.name}</span>
          <span className="text-xs font-normal opacity-70">{t.lockedLevel.note}</span>
        </Button>
        <div className="ml-auto flex items-center gap-3">
          {mistakeCount > 0 && (
            <Badge variant="destructive">{t.mistakes(mistakeCount)}</Badge>
          )}
          <Badge variant="secondary" className="font-mono text-base">
            ⏱ {formatSeconds(finishedIn ?? elapsed)}
          </Badge>
        </div>
      </div>

      {showLocked && (
        <Card className="border-dashed">
          <CardHeader>
            <CardTitle className="text-base">🔒 {t.lockedTitle}</CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-muted-foreground">{t.lockedBody}</CardContent>
        </Card>
      )}

      {!engineReady || !puzzle ? (
        <div className="py-24 text-center text-muted-foreground">{t.loadingEngine}</div>
      ) : (
        <div className="grid gap-6 lg:grid-cols-[1fr_300px]">
          <div>
            <div
              className="relative max-w-2xl"
              onContextMenu={(e) => e.preventDefault()}
            >
              <BoardSvg
                width={puzzle.width}
                height={puzzle.height}
                cells={cells}
                conflicts={allConflicts}
              />
              {/* HTML drag layer over the SVG: per-cell sources + targets. */}
              <div
                className="absolute inset-0 grid"
                style={{ gridTemplateColumns: `repeat(${puzzle.width}, 1fr)` }}
              >
                {board.map((p, pos) => (
                  <div
                    key={pos}
                    draggable={!!p && finishedIn === null}
                    onDragStart={(e) => {
                      if (!p) return;
                      e.dataTransfer.setData("text/plain", `move:${pos}`);
                      e.dataTransfer.effectAllowed = "move";
                    }}
                    onDragOver={(e) => e.preventDefault()}
                    onDrop={(e) => {
                      e.preventDefault();
                      handleDrop(pos, e.dataTransfer.getData("text/plain"));
                    }}
                    onClick={() => onCellClick(pos)}
                    onContextMenu={(e) => {
                      e.preventDefault();
                      removePiece(pos);
                    }}
                    onPointerDown={(e) => {
                      // Long-press removal for touch (no right-click there).
                      if (e.pointerType === "mouse" || !p) return;
                      const timer = window.setTimeout(() => {
                        removePiece(pos);
                        navigator.vibrate?.(40);
                        longPress.current = null;
                      }, 550);
                      longPress.current = { timer, x: e.clientX, y: e.clientY };
                    }}
                    onPointerMove={(e) => {
                      const lp = longPress.current;
                      if (lp && Math.hypot(e.clientX - lp.x, e.clientY - lp.y) > 12) {
                        clearTimeout(lp.timer);
                        longPress.current = null;
                      }
                    }}
                    onPointerUp={() => {
                      if (longPress.current) {
                        clearTimeout(longPress.current.timer);
                        longPress.current = null;
                      }
                    }}
                    onPointerCancel={() => {
                      if (longPress.current) {
                        clearTimeout(longPress.current.timer);
                        longPress.current = null;
                      }
                    }}
                    className={[
                      p ? "cursor-grab active:cursor-grabbing" : "cursor-pointer",
                      !p && candidateCells.has(pos)
                        ? "rounded-sm ring-2 ring-inset ring-emerald-400/70"
                        : "",
                    ].join(" ")}
                  />
                ))}
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <Card
              onDragOver={(e) => e.preventDefault()}
              onDrop={(e) => {
                const data = e.dataTransfer.getData("text/plain");
                if (data.startsWith("move:")) {
                  e.preventDefault();
                  const from = parseInt(data.slice(5), 10);
                  if (!Number.isNaN(from)) removePiece(from);
                }
              }}
            >
              <CardHeader>
                <CardTitle className="flex items-center justify-between text-base">
                  <span>{t.piecesLeft(puzzle.pieces.length - usedPieces.size)}</span>
                  <Button variant="outline" size="xs" onClick={() => reset(size)}>
                    {t.newPuzzle}
                  </Button>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-4 gap-2">
                  {puzzle.pieces.map((edges, i) =>
                    usedPieces.has(i) ? (
                      <div key={i} className="aspect-square rounded bg-muted/50" />
                    ) : (
                      <button
                        key={i}
                        draggable
                        onDragStart={(e) => {
                          e.dataTransfer.setData("text/plain", `piece:${i}`);
                          e.dataTransfer.effectAllowed = "move";
                          setSelected(i);
                        }}
                        onClick={() => setSelected(selected === i ? null : i)}
                        className="cursor-grab rounded transition-transform hover:scale-105 active:cursor-grabbing"
                        title={t.pieceTitle(i + 1)}
                      >
                        <PieceSvg edges={edges} size={58} selected={selected === i} />
                      </button>
                    ),
                  )}
                </div>
                {selected !== null && (
                  <div className="mt-2 space-y-1 text-xs text-muted-foreground">
                    <p>{coarsePointer ? t.trayHintTouch : t.trayHint}</p>
                    <p className="text-emerald-600 dark:text-emerald-400">
                      {t.candidateHint(candidateCells.size)}
                    </p>
                  </div>
                )}
              </CardContent>
            </Card>

            {complete && finishedIn !== null && (
              <Card className="border-emerald-500">
                <CardHeader>
                  <CardTitle className="text-base text-emerald-600">
                    {t.solvedIn(formatSeconds(finishedIn))}
                  </CardTitle>
                </CardHeader>
                <CardContent className="space-y-2 text-sm">
                  {machine && comparison ? (
                    <>
                      <p>
                        {t.machineSolved(
                          machine.ms < 1 ? "<1" : formatInt(machine.ms),
                          formatInt(machine.attempts),
                        )}
                      </p>
                      <p>
                        {comparison.kind === "beforeSecondPiece"
                          ? t.beforeSecondPiece
                          : t.couldHave(formatInt(comparison.n))}
                      </p>
                      <p className="text-muted-foreground">{t.exponential}</p>
                    </>
                  ) : (
                    <p className="text-muted-foreground">{t.racing}</p>
                  )}
                </CardContent>
              </Card>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export const meta = pageMeta("solve");
