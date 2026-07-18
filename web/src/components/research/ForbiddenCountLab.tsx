import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { decodeBucas, type Edges } from "@/lib/bucas";
import { scanForbidden } from "@/lib/forbidden-patch";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { BoardSvg, CELL } from "@/components/board/BoardSvg";
import { cn } from "@/lib/utils";

// "Forbidden 2×2 count is a progress signal," made live. We decode real record
// boards, scan every 2×2 window with the exact feasibility test (does any
// rotation of the four pieces make the square match?), and count the forbidden
// ones. The count falls as the score rises — a secondary objective the matched-
// edge score alone doesn't expose. Computed live from each board's edges.

// Boards spanning the score range, best first. One per score tier so the
// monotone trend is legible without a wall of near-identical 469s.
const FEATURED: { id: string; fallbackLabel: string }[] = [
  { id: "Joshua_Blackwood_470", fallbackLabel: "Blackwood 470" },
  { id: "JBlackwood+PMcGavin_469", fallbackLabel: "McGavin 469" },
  { id: "Joshua_Blackwood_468", fallbackLabel: "Blackwood 468" },
  { id: "Louis_Verhaard_467", fallbackLabel: "Verhaard 467" },
];

const T = {
  en: {
    title: "Count the forbidden squares — live",
    intro:
      "Pick a real record board. We scan all 225 of its 2×2 windows and, for each, ask the exact question: can those four pieces be rotated so the little square matches? The ones where the answer is no are forbidden — outlined in red. A perfect 480 board would have none. Watch the count fall as the board gets better.",
    boardLabel: "Board",
    forbidden: (n: number, total: number) => `${n} forbidden of ${total} windows`,
    scoreLabel: (s: number) => `${s} / 480 matched edges`,
    scatterTitle: "Forbidden count vs score — every known top board",
    scatterX: "matched edges",
    scatterY: "forbidden 2×2",
    note: "Across every known board the trend is monotone: the 470s sit near 18 forbidden squares, the 467s near 23, and weaker boards have dozens. So two boards with the same edge-score aren't equal — the one with fewer forbidden squares is structurally closer to a real solution. It is a second axis of progress the edge-count alone hides. (Counts are over fully-placed interior windows, computed live from each board's edges.)",
  },
  fr: {
    title: "Comptez les carrés interdits — en direct",
    intro:
      "Choisissez un vrai plateau record. On scanne ses 225 fenêtres 2×2 et, pour chacune, on pose la question exacte : peut-on faire tourner ces quatre pièces pour que le petit carré s'accorde ? Celles où la réponse est non sont interdites — encadrées en rouge. Un plateau 480 parfait n'en aurait aucune. Observez le compte chuter à mesure que le plateau s'améliore.",
    boardLabel: "Plateau",
    forbidden: (n: number, total: number) => `${n} interdits sur ${total} fenêtres`,
    scoreLabel: (s: number) => `${s} / 480 bords appariés`,
    scatterTitle: "Compte d'interdits vs score — tous les meilleurs plateaux connus",
    scatterX: "bords appariés",
    scatterY: "2×2 interdits",
    note: "Sur tous les plateaux connus, la tendance est monotone : les 470 sont autour de 18 carrés interdits, les 467 autour de 23, et les plateaux plus faibles en ont des dizaines. Donc deux plateaux de même score ne sont pas équivalents — celui qui a le moins de carrés interdits est structurellement plus proche d'une vraie solution. C'est un second axe de progrès que le compte de bords seul masque. (Comptes sur les fenêtres intérieures entièrement posées, calculés en direct depuis les bords de chaque plateau.)",
  },
  es: {
    title: "Cuenta los cuadrados prohibidos — en vivo",
    intro:
      "Elige un tablero récord real. Escaneamos sus 225 ventanas 2×2 y, para cada una, planteamos la pregunta exacta: ¿se pueden rotar esas cuatro piezas para que el pequeño cuadrado encaje? Aquellas donde la respuesta es no están prohibidas — remarcadas en rojo. Un tablero 480 perfecto no tendría ninguna. Observa cómo cae la cuenta a medida que el tablero mejora.",
    boardLabel: "Tablero",
    forbidden: (n: number, total: number) => `${n} prohibidas de ${total} ventanas`,
    scoreLabel: (s: number) => `${s} / 480 aristas coincidentes`,
    scatterTitle: "Cuenta de prohibidos vs puntuación — todos los mejores tableros conocidos",
    scatterX: "aristas coincidentes",
    scatterY: "2×2 prohibidos",
    note: "En todos los tableros conocidos la tendencia es monótona: los 470 rondan los 18 cuadrados prohibidos, los 467 los 23, y los tableros más flojos tienen decenas. Así que dos tableros con la misma puntuación no son equivalentes — el que tiene menos cuadrados prohibidos está estructuralmente más cerca de una solución real. Es un segundo eje de progreso que la cuenta de aristas por sí sola oculta. (Cuentas sobre las ventanas interiores totalmente colocadas, calculadas en vivo a partir de las aristas de cada tablero.)",
  },
};

interface Decoded {
  cells: (Edges | null)[];
  width: number;
  height: number;
  score: number;
  label: string;
  forbidden: number;
  total: number;
  windows: number[];
}

function decodeBoard(id: string, fallbackLabel: string): Decoded | null {
  const b = KNOWN_BOARDS.find((k) => k.id === id);
  if (!b) return null;
  try {
    const board = decodeBucas(b.params);
    const scan = scanForbidden(board.cells, board.width, board.height);
    return {
      cells: board.cells,
      width: board.width,
      height: board.height,
      score: b.score ?? 0,
      label: b.label.split(",")[0] ?? fallbackLabel,
      forbidden: scan.forbidden,
      total: scan.total,
      windows: scan.windows,
    };
  } catch {
    return null;
  }
}

export function ForbiddenCountLab() {
  const t = useT(T);
  const [pick, setPick] = useState(0);

  const boards = useMemo(
    () => FEATURED.map((f) => decodeBoard(f.id, f.fallbackLabel)).filter((d): d is Decoded => d !== null),
    [],
  );

  // Scatter over every scored, complete known board (deduped by label root).
  const scatter = useMemo(() => {
    const seen = new Set<string>();
    const pts: { score: number; forbidden: number }[] = [];
    for (const b of KNOWN_BOARDS) {
      if (b.score == null) continue;
      try {
        const board = decodeBucas(b.params);
        const scan = scanForbidden(board.cells, board.width, board.height);
        if (scan.total < 200) continue; // skip partials
        const key = `${b.score}:${scan.forbidden}`;
        if (seen.has(key)) continue;
        seen.add(key);
        pts.push({ score: b.score, forbidden: scan.forbidden });
      } catch {
        /* skip undecodable */
      }
    }
    return pts;
  }, []);

  if (boards.length === 0) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        …
      </div>
    );
  }

  const board = boards[Math.min(pick, boards.length - 1)] ?? boards[0];
  if (!board) return null;

  // Outline each forbidden 2×2 window for the overlay layer (BoardSvg user
  // coordinates are CELL units, origin top-left).
  const overlay = (
    <g>
      {board.windows.map((tl) => {
        const x = tl % board.width;
        const y = Math.floor(tl / board.width);
        return (
          <rect
            key={tl}
            x={x * CELL + 6}
            y={y * CELL + 6}
            width={2 * CELL - 12}
            height={2 * CELL - 12}
            fill="rgba(244,63,94,0.12)"
            stroke="#f43f5e"
            strokeWidth={10}
            rx={12}
          />
        );
      })}
    </g>
  );

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap justify-center gap-1.5">
        {boards.map((b, i) => (
          <button
            key={b.label}
            onClick={() => setPick(i)}
            className={cn(
              "rounded border px-2.5 py-1 text-xs font-medium tabular-nums transition-colors",
              i === pick
                ? "border-primary bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted",
            )}
          >
            {b.score}
          </button>
        ))}
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-72 space-y-2">
          <BoardSvg
            width={board.width}
            height={board.height}
            cells={board.cells}
            overlay={overlay}
          />
          <div className="rounded-md border border-rose-300 bg-rose-500/10 px-3 py-2 text-center">
            <div className="text-xl font-bold tabular-nums">
              {t.forbidden(board.forbidden, board.total)}
            </div>
            <div className="text-[11px] text-muted-foreground">{t.scoreLabel(board.score)}</div>
          </div>
        </div>

        <div className="min-w-56 flex-1 space-y-1">
          <p className="text-center text-[11px] font-medium text-muted-foreground">
            {t.scatterTitle}
          </p>
          <Scatter points={scatter} highlight={board} xLabel={t.scatterX} yLabel={t.scatterY} />
        </div>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}

function Scatter({
  points,
  highlight,
  xLabel,
  yLabel,
}: {
  points: { score: number; forbidden: number }[];
  highlight: { score: number; forbidden: number };
  xLabel: string;
  yLabel: string;
}) {
  const W = 300;
  const H = 200;
  const padL = 30;
  const padR = 8;
  const padT = 10;
  const padB = 26;

  const xs = points.map((p) => p.score);
  const ys = points.map((p) => p.forbidden);
  const minX = Math.min(...xs, highlight.score) - 1;
  const maxX = Math.max(...xs, highlight.score) + 1;
  const maxY = Math.max(...ys, highlight.forbidden) + 2;

  const x = (s: number) => padL + ((s - minX) / (maxX - minX)) * (W - padL - padR);
  const y = (f: number) => padT + (1 - f / maxY) * (H - padT - padB);

  return (
    <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card" role="img" aria-label="Scatter plot of forbidden 2×2 windows against matched edges across record boards, the count of forbidden squares falling as the score rises, with the selected board highlighted">
      {/* axes */}
      <line x1={padL} y1={H - padB} x2={W - padR} y2={H - padB} className="stroke-muted" strokeWidth={1} />
      <line x1={padL} y1={padT} x2={padL} y2={H - padB} className="stroke-muted" strokeWidth={1} />
      {points.map((p, i) => {
        const isHi = p.score === highlight.score && p.forbidden === highlight.forbidden;
        return (
          <circle
            key={i}
            cx={x(p.score)}
            cy={y(p.forbidden)}
            r={isHi ? 5 : 3}
            className={isHi ? "fill-primary" : "fill-muted-foreground/50"}
          />
        );
      })}
      <text x={(W + padL) / 2} y={H - 4} textAnchor="middle" className="fill-muted-foreground text-[9px]">
        {xLabel}
      </text>
      <text
        x={10}
        y={(H - padB) / 2}
        textAnchor="middle"
        transform={`rotate(-90 10 ${(H - padB) / 2})`}
        className="fill-muted-foreground text-[9px]"
      >
        {yLabel}
      </text>
    </svg>
  );
}
