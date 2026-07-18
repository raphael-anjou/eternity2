import { useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// One destroy-and-repair iteration, walked one phase at a time. This is the loop
// the repair study takes apart, drawn as a controlled illustration (a small board
// with hand-set colors, not a live engine run) so each phase is unambiguous:
//
//   0. Working board   — a full board; seams are matched (green) or broken (red).
//   1. Destroy         — the operator lifts a set of cells (dashed holes); the
//                        seams they carried disappear.
//   2. Repair          — the hole is refilled from the lifted pieces (highlighted).
//   3. Accept / revert — score the result; keep it if it did not lose, else put
//                        the snapshot back.
//
// The reader advances the phase; a caption narrates what changed and why. The
// four phases are exactly the four axes the study varies (destroy / repair /
// accept, plus the starting board and the restart-on-stall that wrap the loop).

const G = 6; // a small illustrative grid
const CELL = 40;
const GAP = 3;

type SeamState = "match" | "break" | "gone";
// A cell: filled? part of the destroyed set? just refilled? Plus its right/down
// seam state at each phase, so the picture is exact rather than suggestive.
type Phase = 0 | 1 | 2 | 3;

// The destroyed set for the illustration: a small conflict cluster near the
// centre. Fixed, so the four phases tell one coherent story.
const HOLE = new Set([14, 15, 20, 21]);

// Right/down seam colors per phase, indexed by cell. We only hand-author the
// handful of seams the story turns on; the rest stay matched (green), the
// steady-state of a decent board.
const BROKEN_BEFORE = new Set(["13R", "14D", "15D", "21R"]); // seams around the hole, before
const BROKEN_AFTER = new Set(["21R"]); // one stubborn seam remains after repair

function seamState(key: string, phase: Phase, dir: "R" | "D", pos: number): SeamState {
  // During destroy/repair the seams incident to the hole are undefined (gone).
  const touchesHole = HOLE.has(pos) || incidentToHole(pos, dir);
  if ((phase === 1 || phase === 2) && touchesHole && phase === 1) return "gone";
  if (phase === 0) return BROKEN_BEFORE.has(key) ? "break" : "match";
  if (phase === 1) return touchesHole ? "gone" : BROKEN_BEFORE.has(key) ? "break" : "match";
  // phases 2 and 3: the hole is refilled; fewer seams broken than before.
  return BROKEN_AFTER.has(key) ? "break" : "match";
}

function incidentToHole(pos: number, dir: "R" | "D"): boolean {
  const right = dir === "R" ? pos + 1 : -1;
  const down = dir === "D" ? pos + G : -1;
  return HOLE.has(right) || HOLE.has(down);
}

const T = {
  en: {
    title: "One destroy-and-repair iteration, step by step",
    phases: ["Working board", "Destroy", "Repair", "Accept or revert"],
    caps: [
      "A full board. Every interior seam is either matched (green) or broken (red). The score is the count of matched seams; this loop tries to raise it by rebuilding a piece of the board at a time.",
      "The destroy operator lifts a set of cells (here a small cluster around the broken seams) and pools their pieces. The seams those cells carried are now gone; the board is temporarily incomplete. Which cells to lift is the study's headline axis.",
      "The repair step refills the hole from the pooled pieces, most-constrained cell first, choosing the piece that matches the most surrounding seams. The refilled cells are highlighted; one stubborn seam could not be matched, but the region is better than before.",
      "Score the rebuilt board. If it did not lose score, keep it and iterate; if it did, restore the snapshot and try a different destroy next time. The acceptance rule (keep-if-not-worse, strict, annealing, or late-acceptance) is a whole axis of its own.",
    ],
    matched: "matched seam",
    broken: "broken seam",
    destroyed: "lifted (destroyed)",
    refilled: "refilled",
    prev: "Previous",
    next: "Next",
    step: "Phase",
    busy: "Loading…",
  },
  fr: {
    title: "Une itération destruction-réparation, pas à pas",
    phases: ["Plateau de travail", "Destruction", "Réparation", "Accepter ou annuler"],
    caps: [
      "Un plateau complet. Chaque jointure intérieure est soit appariée (vert), soit cassée (rouge). Le score est le nombre de jointures appariées ; cette boucle cherche à l'augmenter en reconstruisant une partie du plateau à la fois.",
      "L'opérateur de destruction retire un ensemble de cases (ici un petit amas autour des jointures cassées) et met leurs pièces en réserve. Les jointures que portaient ces cases ont disparu ; le plateau est temporairement incomplet. Quelles cases retirer est l'axe central de l'étude.",
      "La réparation remplit le trou à partir des pièces en réserve, case la plus contrainte d'abord, en choisissant la pièce qui apparie le plus de jointures voisines. Les cases replacées sont mises en évidence ; une jointure tenace n'a pu être appariée, mais la région est meilleure qu'avant.",
      "On note le plateau reconstruit. S'il n'a pas perdu de score, on le garde et on itère ; sinon, on restaure l'instantané et on tentera une autre destruction. La règle d'acceptation (garder-si-pas-pire, stricte, recuit, ou acceptation tardive) est un axe à part entière.",
    ],
    matched: "jointure appariée",
    broken: "jointure cassée",
    destroyed: "retirée (détruite)",
    refilled: "replacée",
    prev: "Précédent",
    next: "Suivant",
    step: "Phase",
    busy: "Chargement…",
  },
  es: {
    title: "Una iteración de destrucción y reparación, paso a paso",
    phases: ["Tablero de trabajo", "Destruir", "Reparar", "Aceptar o revertir"],
    caps: [
      "Un tablero completo. Cada arista interior está coincidente (verde) o rota (rojo). La puntuación es el número de aristas coincidentes; este bucle intenta aumentarla reconstruyendo una parte del tablero cada vez.",
      "El operador de destrucción retira un conjunto de celdas (aquí un pequeño grupo alrededor de las aristas rotas) y reúne sus piezas. Las aristas que sostenían esas celdas han desaparecido; el tablero queda temporalmente incompleto. Qué celdas retirar es el eje central del estudio.",
      "La reparación rellena el hueco a partir de las piezas reunidas, primero la celda más restringida, eligiendo la pieza que hace coincidir más aristas vecinas. Las celdas rellenadas se resaltan; una arista tenaz no pudo coincidir, pero la región está mejor que antes.",
      "Se puntúa el tablero reconstruido. Si no perdió puntuación, se conserva y se itera; si la perdió, se restaura la instantánea y la próxima vez se prueba otra destrucción. La regla de aceptación (conservar-si-no-empeora, estricta, recocido o aceptación tardía) es todo un eje en sí mismo.",
    ],
    matched: "arista coincidente",
    broken: "arista rota",
    destroyed: "retirada (destruida)",
    refilled: "rellenada",
    prev: "Anterior",
    next: "Siguiente",
    step: "Fase",
    busy: "Cargando…",
  },
};

export function RepairLoopDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [phase, setPhase] = useState<Phase>(0);

  if (!isClient) {
    return (
      <div className="flex h-96 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const size = G * CELL + (G + 1) * GAP;

  const cellFill = (pos: number): string => {
    const lifted = HOLE.has(pos) && (phase === 1 || phase === 2);
    if (phase === 1 && HOLE.has(pos)) return "transparent"; // hole, dashed outline only
    if (lifted && phase === 2) return "var(--refill)";
    if ((phase === 2 || phase === 3) && HOLE.has(pos)) return "var(--refill)";
    return "var(--tile)";
  };
  const cellDashed = (pos: number) => phase === 1 && HOLE.has(pos);

  return (
    <div
      className="space-y-4"
      style={
        {
          "--tile": "color-mix(in oklab, currentColor 8%, transparent)",
          "--refill": "color-mix(in oklab, #8b5cf6 22%, transparent)",
        } as React.CSSProperties
      }
    >
      {/* phase indicator */}
      <div className="flex flex-wrap items-center gap-2">
        {t.phases.map((label, i) => (
          <button
            key={label}
            type="button"
            onClick={() => setPhase(i as Phase)}
            className={`rounded-full border px-3 py-1 text-xs font-medium transition-colors ${
              i === phase
                ? "border-transparent bg-foreground text-background"
                : "border-border text-muted-foreground hover:text-foreground"
            }`}
            aria-current={i === phase ? "step" : undefined}
          >
            {i + 1}. {label}
          </button>
        ))}
      </div>

      <div className="grid gap-5 sm:grid-cols-[auto_1fr] sm:items-start">
        {/* the board */}
        <svg
          viewBox={`0 0 ${size} ${size}`}
          className="w-full max-w-[280px] rounded-md border bg-card"
          role="img"
          aria-label={`${t.step} ${phase + 1}: ${t.phases[phase]}`}
        >
          {Array.from({ length: G * G }, (_, pos) => {
            const x = (pos % G) * (CELL + GAP) + GAP;
            const y = Math.floor(pos / G) * (CELL + GAP) + GAP;
            return (
              <rect
                key={pos}
                x={x}
                y={y}
                width={CELL}
                height={CELL}
                rx={4}
                fill={cellFill(pos)}
                stroke={cellDashed(pos) ? "#8b5cf6" : "currentColor"}
                strokeOpacity={cellDashed(pos) ? 0.8 : 0.12}
                strokeWidth={cellDashed(pos) ? 2 : 1}
                strokeDasharray={cellDashed(pos) ? "4 3" : undefined}
              />
            );
          })}
          {/* right + down seams, colored by phase */}
          {Array.from({ length: G * G }, (_, pos) => {
            const x = (pos % G) * (CELL + GAP) + GAP;
            const y = Math.floor(pos / G) * (CELL + GAP) + GAP;
            const segs: React.ReactNode[] = [];
            const col = pos % G;
            const row = Math.floor(pos / G);
            const draw = (key: string, dir: "R" | "D") => {
              const st = seamState(key, phase, dir, pos);
              if (st === "gone") return null;
              const stroke = st === "break" ? "#ef4444" : "#10b981";
              if (dir === "R") {
                const sx = x + CELL + GAP / 2;
                return <line key={key} x1={sx} y1={y + 6} x2={sx} y2={y + CELL - 6} stroke={stroke} strokeWidth={3} strokeLinecap="round" />;
              }
              const sy = y + CELL + GAP / 2;
              return <line key={key} x1={x + 6} y1={sy} x2={x + CELL - 6} y2={sy} stroke={stroke} strokeWidth={3} strokeLinecap="round" />;
            };
            if (col < G - 1) segs.push(draw(`${pos}R`, "R"));
            if (row < G - 1) segs.push(draw(`${pos}D`, "D"));
            return <g key={`s${pos}`}>{segs}</g>;
          })}
        </svg>

        {/* caption + legend + controls */}
        <div className="space-y-3">
          <p className="text-sm leading-relaxed text-muted-foreground">{t.caps[phase]}</p>
          <div className="flex flex-wrap gap-x-4 gap-y-1.5 text-xs text-muted-foreground">
            <Swatch color="#10b981" label={t.matched} />
            <Swatch color="#ef4444" label={t.broken} />
            <Swatch color="#8b5cf6" label={phase === 1 ? t.destroyed : t.refilled} />
          </div>
          <div className="flex items-center gap-2 pt-1">
            <button
              type="button"
              onClick={() => setPhase((p) => (Math.max(0, p - 1) as Phase))}
              disabled={phase === 0}
              className="rounded-md border px-3 py-1.5 text-xs font-medium disabled:opacity-40"
            >
              ← {t.prev}
            </button>
            <button
              type="button"
              onClick={() => setPhase((p) => (Math.min(3, p + 1) as Phase))}
              disabled={phase === 3}
              className="rounded-md border px-3 py-1.5 text-xs font-medium disabled:opacity-40"
            >
              {t.next} →
            </button>
            <span className="ml-1 text-xs tabular-nums text-muted-foreground">
              {t.step} {phase + 1} / 4
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

function Swatch({ color, label }: { color: string; label: string }) {
  return (
    <span className="inline-flex items-center gap-1.5">
      <span className="inline-block h-2.5 w-2.5 rounded-sm" style={{ backgroundColor: color }} aria-hidden />
      {label}
    </span>
  );
}
