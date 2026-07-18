import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedSolvedPuzzleFramed } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { boardFromEngine, conflictEdges, scoreSummary } from "@/lib/bucas";
import { BoardSvg, CELL } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

// The 479 story, made clickable. An engine-generated solved framed 8×8 stands
// in for a 480 board: every scored edge matches, and the outward-facing grey
// rim is unscored, exactly like the real puzzle's 60 grey edges. Click any
// piece to turn it 180°:
//
//  - interior pieces change up/down together and left/right together, so
//    scored breaks always come in pairs (0, 2 or 4) — kubzpa's parity;
//  - a sky-ringed interior piece (one equal opposite pair) breaks exactly
//    two — psykowally's 478 construction;
//  - an emerald-ringed border piece (equal ring-facing sides) breaks exactly
//    ONE scored edge, the seam behind it, while the outward change lands on
//    the unscored rim (flashed amber) — Verhaard's 479 refutation.
//
// Score and conflicts come from the same bucas-compatible code the viewer
// uses; nothing is faked. No timers — the lab is purely interaction-driven.

const SIZE = 8;
const COLORS = 6;

type CellKind = "interior" | "border" | "corner";

const T = {
  en: {
    title: "Flip it yourself — the 478 and the 479",
    intro:
      "A solved board (engine-generated, all scored edges matched). Click any piece to turn it 180°. Sky rings mark interior pieces with one pair of equal opposite edges — the 478 move. Emerald rings mark border pieces whose two ring-facing sides share a colour — the 479 move. The grey band is the unscored rim: changes that land there cost nothing.",
    score: "matched edges",
    mismatches: "mismatches",
    stateSolved: "solved — the stand-in for 480/480",
    state479: "one single mismatch — a 479-class board, the score parity said was impossible",
    state478: "two mismatches — the 478-class board of the original argument",
    stateOther: (m: number) => `${m} mismatches`,
    click: "Click any piece to rotate it 180°.",
    reset: "Reset",
    newDraw: "New draw",
    hints: "Story moves",
    legendInterior: "478 move: interior, one equal opposite pair",
    legendBorder: "479 move: border, equal ring-facing sides",
    legendRim: "unscored grey rim",
    legendLanded: "outward change parked on the rim — free",
    moves: {
      repaired: (n: number) => `You repaired ${n} scored edge${n === 1 ? "" : "s"}.`,
      invisible:
        "Nothing broke — both opposite pairs of this piece are equal, so the 180° turn is invisible.",
      interior2:
        "Exactly two scored edges broke: one opposite pair was equal, the other was not. On the real board this is the 478 construction.",
      interior4:
        "Four scored edges broke — still an even number. Interior turns change up/down together and left/right together; they cannot break an odd count.",
      border1:
        "ONE scored edge broke — the seam behind the piece. The other change landed on the outward grey edge, which the score never reads. This is the 479 move that defeated the parity proof.",
      border3:
        "Three scored edges broke — an odd number. The rim absorbed exactly one of the four changes; the parity leak is the border's unscored edge.",
      corner2:
        "Two scored edges broke. A corner turn sends its two greys inward, so both changes are scored — corners do not leak parity.",
      generic: (d: number) =>
        `${Math.abs(d)} scored edge${Math.abs(d) === 1 ? "" : "s"} changed — flips interacting with earlier flips can offset each other.`,
    },
    qualifying: (i: number, b: number) =>
      `On this draw: ${i} interior piece${i === 1 ? "" : "s"} allow the 478 move, ${b} border piece${b === 1 ? "" : "s"} the 479 move.`,
    note: "On the official 16×16 set, 14 border pieces qualify for the 479 flip (computed by this project), so any full solution implies a 479. The rim here has 32 unscored edges; the real board has 60 — the sixty holes in kubzpa's proof.",
    loading: "Generating a solved board…",
  },
  fr: {
    title: "Retournez-le vous-même — le 478 et le 479",
    intro:
      "Un plateau résolu (généré par le moteur, toutes les arêtes comptées appariées). Cliquez sur une pièce pour la tourner de 180°. Les anneaux bleu ciel marquent les pièces intérieures à une paire de côtés opposés égaux — le coup du 478. Les anneaux émeraude marquent les pièces de bordure dont les deux côtés longeant l'anneau partagent une couleur — le coup du 479. La bande grise est le rebord non compté : ce qui y atterrit ne coûte rien.",
    score: "arêtes appariées",
    mismatches: "défauts",
    stateSolved: "résolu — l'équivalent du 480/480",
    state479: "un seul défaut — un plateau de classe 479, que la parité du score disait impossible",
    state478: "deux défauts — le plateau de classe 478 de l'argument d'origine",
    stateOther: (m: number) => `${m} défauts`,
    click: "Cliquez sur une pièce pour la tourner de 180°.",
    reset: "Réinitialiser",
    newDraw: "Nouveau tirage",
    hints: "Coups de l'histoire",
    legendInterior: "coup du 478 : intérieur, une paire opposée égale",
    legendBorder: "coup du 479 : bordure, côtés le long de l'anneau égaux",
    legendRim: "rebord gris non compté",
    legendLanded: "changement extérieur garé sur le rebord — gratuit",
    moves: {
      repaired: (n: number) => `Vous avez réparé ${n} arête${n === 1 ? "" : "s"} comptée${n === 1 ? "" : "s"}.`,
      invisible:
        "Rien ne casse — les deux paires de côtés opposés de cette pièce sont égales, la rotation de 180° est invisible.",
      interior2:
        "Exactement deux arêtes comptées cassent : une paire opposée était égale, l'autre non. Sur le vrai plateau, c'est la construction du 478.",
      interior4:
        "Quatre arêtes comptées cassent — toujours un nombre pair. Une rotation intérieure change haut/bas ensemble et gauche/droite ensemble ; elle ne peut pas casser un compte impair.",
      border1:
        "UNE seule arête comptée casse — la couture derrière la pièce. L'autre changement atterrit sur l'arête grise extérieure, que le score ne lit jamais. C'est le coup du 479 qui a vaincu la preuve de parité.",
      border3:
        "Trois arêtes comptées cassent — un nombre impair. Le rebord a absorbé exactement un des quatre changements ; la fuite de parité, c'est l'arête non comptée de la bordure.",
      corner2:
        "Deux arêtes comptées cassent. Une rotation de coin renvoie ses deux gris vers l'intérieur : les deux changements sont comptés — les coins ne fuient pas.",
      generic: (d: number) =>
        `${Math.abs(d)} arête${Math.abs(d) === 1 ? "" : "s"} comptée${Math.abs(d) === 1 ? "" : "s"} changée${Math.abs(d) === 1 ? "" : "s"} — des retournements voisins peuvent se compenser.`,
    },
    qualifying: (i: number, b: number) =>
      `Sur ce tirage : ${i} pièce${i === 1 ? "" : "s"} intérieure${i === 1 ? "" : "s"} permettent le coup du 478, ${b} pièce${b === 1 ? "" : "s"} de bordure le coup du 479.`,
    note: "Sur le jeu officiel 16×16, 14 pièces de bordure se qualifient pour le retournement du 479 (calculé par ce projet) : toute solution complète implique donc un 479. Le rebord compte ici 32 arêtes non comptées ; le vrai plateau en a 60 — les soixante trous de la preuve de kubzpa.",
    loading: "Génération d'un plateau résolu…",
  },
  es: {
    title: "Gíralo tú mismo — el 478 y el 479",
    intro:
      "Un tablero resuelto (generado por el motor, todas las aristas puntuadas coincidentes). Haz clic en cualquier pieza para girarla 180°. Los anillos azul cielo señalan las piezas interiores con un par de aristas opuestas iguales — la jugada del 478. Los anillos esmeralda señalan las piezas de borde cuyos dos lados que bordean el anillo comparten color — la jugada del 479. La banda gris es el borde no puntuado: lo que cae ahí no cuesta nada.",
    score: "aristas coincidentes",
    mismatches: "desajustes",
    stateSolved: "resuelto — el equivalente del 480/480",
    state479: "un único desajuste — un tablero de clase 479, que la paridad de la puntuación daba por imposible",
    state478: "dos desajustes — el tablero de clase 478 del argumento original",
    stateOther: (m: number) => `${m} desajustes`,
    click: "Haz clic en cualquier pieza para girarla 180°.",
    reset: "Reiniciar",
    newDraw: "Nuevo sorteo",
    hints: "Jugadas de la historia",
    legendInterior: "jugada del 478: interior, un par opuesto igual",
    legendBorder: "jugada del 479: borde, lados que bordean el anillo iguales",
    legendRim: "borde gris no puntuado",
    legendLanded: "cambio exterior aparcado en el borde — gratis",
    moves: {
      repaired: (n: number) => `Reparaste ${n} arista${n === 1 ? "" : "s"} puntuada${n === 1 ? "" : "s"}.`,
      invisible:
        "Nada se rompió — los dos pares de aristas opuestas de esta pieza son iguales, así que el giro de 180° es invisible.",
      interior2:
        "Se rompieron exactamente dos aristas puntuadas: un par opuesto era igual y el otro no. En el tablero real esta es la construcción del 478.",
      interior4:
        "Se rompieron cuatro aristas puntuadas — sigue siendo un número par. Los giros interiores cambian arriba/abajo a la vez e izquierda/derecha a la vez; no pueden romper un número impar.",
      border1:
        "Se rompió UNA sola arista puntuada — la costura detrás de la pieza. El otro cambio cayó en la arista gris exterior, que la puntuación nunca lee. Esta es la jugada del 479 que derrotó la prueba de paridad.",
      border3:
        "Se rompieron tres aristas puntuadas — un número impar. El borde absorbió exactamente uno de los cuatro cambios; la fuga de paridad es la arista no puntuada del borde.",
      corner2:
        "Se rompieron dos aristas puntuadas. Un giro de esquina manda sus dos grises hacia el interior, así que ambos cambios se puntúan — las esquinas no filtran paridad.",
      generic: (d: number) =>
        `Cambió${Math.abs(d) === 1 ? "" : "n"} ${Math.abs(d)} arista${Math.abs(d) === 1 ? "" : "s"} puntuada${Math.abs(d) === 1 ? "" : "s"} — giros que interactúan con giros anteriores pueden compensarse entre sí.`,
    },
    qualifying: (i: number, b: number) =>
      `En este sorteo: ${i} pieza${i === 1 ? "" : "s"} interior${i === 1 ? "" : "es"} permite${i === 1 ? "" : "n"} la jugada del 478, ${b} pieza${b === 1 ? "" : "s"} de borde la del 479.`,
    note: "En el juego oficial de 16×16, 14 piezas de borde cumplen los requisitos para el giro del 479 (calculado por este proyecto), así que cualquier solución completa implica un 479. El borde aquí tiene 32 aristas no puntuadas; el tablero real tiene 60 — los sesenta agujeros de la prueba de kubzpa.",
    loading: "Generando un tablero resuelto…",
  },
};

function cellKind(pos: number): CellKind {
  const x = pos % SIZE;
  const y = Math.floor(pos / SIZE);
  const onX = x === 0 || x === SIZE - 1;
  const onY = y === 0 || y === SIZE - 1;
  if (onX && onY) return "corner";
  if (onX || onY) return "border";
  return "interior";
}

interface MoveInfo {
  kind: CellKind;
  delta: number; // change in broken scored edges (positive = newly broken)
}

export function ParityFlipLab() {
  const t = useT(T);
  const ready = useEngine();
  const [seed, setSeed] = useState(3);
  // 180° flip toggles per cell (0 or 2 rotation offset on the solved layout).
  const [flips, setFlips] = useState<Set<number>>(new Set());
  const [last, setLast] = useState<MoveInfo | null>(null);
  const [showHints, setShowHints] = useState(true);

  const puzzle = useMemo<Puzzle | null>(
    () => (ready ? getGeneratedSolvedPuzzleFramed(SIZE, COLORS, seed, true) : null),
    [ready, seed],
  );

  const engineCells = useMemo(
    () =>
      puzzle
        ? puzzle.pieces.map((_, i) => i * 4 + (flips.has(i) ? 2 : 0))
        : [],
    [puzzle, flips],
  );

  const board = useMemo(
    () => (puzzle ? boardFromEngine(puzzle, engineCells) : null),
    [puzzle, engineCells],
  );

  const summary = board ? scoreSummary(board) : null;
  const conflicts = board ? conflictEdges(board) : [];

  // Which pieces enable the story moves (solved layout: piece i at cell i, rot 0).
  const story = useMemo(() => {
    const interior: number[] = [];
    const border: number[] = [];
    if (!puzzle) return { interior, border };
    puzzle.pieces.forEach((e, pos) => {
      const kind = cellKind(pos);
      const vertEq = e[0] === e[2];
      const horizEq = e[1] === e[3];
      if (kind === "interior") {
        if (vertEq !== horizEq) interior.push(pos); // exactly one equal pair → 2 breaks
      } else if (kind === "border") {
        const grey = e.indexOf(0);
        if (grey >= 0 && e[(grey + 1) % 4] === e[(grey + 3) % 4]) border.push(pos); // equal laterals → 1 break
      }
    });
    return { interior, border };
  }, [puzzle]);

  const flip = (pos: number) => {
    if (!puzzle || !summary) return;
    const next = new Set(flips);
    if (next.has(pos)) next.delete(pos);
    else next.add(pos);
    // Measure the move honestly: score the board before and after.
    const nextBoard = boardFromEngine(
      puzzle,
      puzzle.pieces.map((_, i) => i * 4 + (next.has(i) ? 2 : 0)),
    );
    const after = scoreSummary(nextBoard);
    setLast({ kind: cellKind(pos), delta: summary.score - after.score });
    setFlips(next);
  };

  const reset = () => {
    setFlips(new Set());
    setLast(null);
  };

  const newDraw = () => {
    setSeed((s) => (s * 7 + 13) % 100000);
    setFlips(new Set());
    setLast(null);
  };

  if (!puzzle || !board || !summary) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const broken = summary.max - summary.score;
  const state =
    broken === 0
      ? t.stateSolved
      : broken === 1
        ? t.state479
        : broken === 2
          ? t.state478
          : t.stateOther(broken);

  const moveMessage = (m: MoveInfo): string => {
    if (m.delta < 0) return t.moves.repaired(-m.delta);
    if (m.delta === 0) return m.kind === "interior" ? t.moves.invisible : t.moves.generic(0);
    if (m.kind === "interior") {
      if (m.delta === 2) return t.moves.interior2;
      if (m.delta === 4) return t.moves.interior4;
      return t.moves.generic(m.delta);
    }
    if (m.kind === "border") {
      if (m.delta === 1) return t.moves.border1;
      if (m.delta === 3) return t.moves.border3;
      return t.moves.generic(m.delta);
    }
    if (m.delta === 2) return t.moves.corner2;
    return t.moves.generic(m.delta);
  };

  // Overlay: unscored rim band, story rings, amber "parked on the rim" flashes.
  const RIM = 30;
  const BOARD_PX = SIZE * CELL;
  const cx = (pos: number) => (pos % SIZE) * CELL + CELL / 2;
  const cy = (pos: number) => Math.floor(pos / SIZE) * CELL + CELL / 2;
  const rimLanded: { pos: number; grey: number }[] = [];
  for (const pos of flips) {
    if (cellKind(pos) !== "border") continue;
    const e = puzzle.pieces[pos];
    if (!e) continue;
    const grey = e.indexOf(0);
    if (grey >= 0) rimLanded.push({ pos, grey });
  }

  const overlay = (
    <g style={{ pointerEvents: "none" }}>
      {/* unscored rim band */}
      <rect x={0} y={0} width={BOARD_PX} height={RIM} fill="#a8a29e" opacity={0.35} />
      <rect x={0} y={BOARD_PX - RIM} width={BOARD_PX} height={RIM} fill="#a8a29e" opacity={0.35} />
      <rect x={0} y={0} width={RIM} height={BOARD_PX} fill="#a8a29e" opacity={0.35} />
      <rect x={BOARD_PX - RIM} y={0} width={RIM} height={BOARD_PX} fill="#a8a29e" opacity={0.35} />
      {/* outward change parked on the rim (flipped border pieces) */}
      {rimLanded.map(({ pos, grey }) => {
        const x = (pos % SIZE) * CELL;
        const y = Math.floor(pos / SIZE) * CELL;
        const thick = 40;
        const r =
          grey === 0
            ? { x, y, width: CELL, height: thick }
            : grey === 2
              ? { x, y: y + CELL - thick, width: CELL, height: thick }
              : grey === 3
                ? { x, y, width: thick, height: CELL }
                : { x: x + CELL - thick, y, width: thick, height: CELL };
        return <rect key={pos} {...r} fill="#f59e0b" opacity={0.85} rx={6} />;
      })}
      {/* story rings */}
      {showHints &&
        story.interior.map((pos) => (
          <circle
            key={`i${pos}`}
            cx={cx(pos)}
            cy={cy(pos)}
            r={CELL * 0.36}
            fill="none"
            stroke="#38bdf8"
            strokeWidth={16}
            opacity={0.9}
          />
        ))}
      {showHints &&
        story.border.map((pos) => (
          <circle
            key={`b${pos}`}
            cx={cx(pos)}
            cy={cy(pos)}
            r={CELL * 0.36}
            fill="none"
            stroke="#10b981"
            strokeWidth={16}
            opacity={0.9}
          />
        ))}
    </g>
  );

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-80 space-y-2">
          <BoardSvg
            width={SIZE}
            height={SIZE}
            cells={board.cells}
            conflicts={conflicts}
            overlay={overlay}
            onCellClick={flip}
            className="cursor-pointer"
          />
          <p className="text-center text-[11px] text-muted-foreground">{t.click}</p>
        </div>

        <div className="min-w-56 max-w-sm flex-1 space-y-3">
          <div
            className={cn(
              "rounded-md border px-3 py-2 text-center",
              broken === 0
                ? "border-emerald-300 bg-emerald-500/10"
                : broken === 1
                  ? "border-amber-300 bg-amber-500/10"
                  : "border-rose-300 bg-rose-500/10",
            )}
          >
            <div className="text-2xl font-bold tabular-nums">
              {summary.score} / {summary.max}
              <span className="ml-2 text-sm font-normal text-muted-foreground">{t.score}</span>
            </div>
            <div className="text-[11px] leading-snug text-muted-foreground">{state}</div>
          </div>

          {last && (
            <p className="rounded-md border bg-muted/30 px-3 py-2 text-xs leading-relaxed">
              {moveMessage(last)}
            </p>
          )}

          <ul className="space-y-1 text-[11px] text-muted-foreground">
            <li className="flex items-center gap-2">
              <span className="inline-block h-3 w-3 shrink-0 rounded-full border-2 border-sky-400" />
              {t.legendInterior}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-3 w-3 shrink-0 rounded-full border-2 border-emerald-500" />
              {t.legendBorder}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-3 w-3 shrink-0 rounded-sm bg-stone-400/60" />
              {t.legendRim}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-3 w-3 shrink-0 rounded-sm bg-amber-500" />
              {t.legendLanded}
            </li>
          </ul>

          <p className="text-[11px] text-muted-foreground">
            {t.qualifying(story.interior.length, story.border.length)}
          </p>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button size="sm" variant="outline" onClick={reset} disabled={flips.size === 0}>
          {t.reset}
        </Button>
        <Button size="sm" variant="outline" onClick={newDraw}>
          {t.newDraw}
        </Button>
        <Button
          size="sm"
          variant={showHints ? "default" : "outline"}
          onClick={() => setShowHints((h) => !h)}
        >
          {t.hints}
        </Button>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
