import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// Algorithm X on Knuth's own 7-item exact-cover instance, animated. Each frame
// shows the live matrix: choose the column with fewest 1s, cover it (its rows
// detach — links fade), recurse, hit a dead end, and uncover in reverse order
// (the same two pointer writes, undone — the links dance back). Depth meter,
// solutions counter, and a narration line explain each move. The whole search
// is precomputed (it is tiny and deterministic); the component replays frames.

const COLS = ["A", "B", "C", "D", "E", "F", "G"];

/** Knuth's example from the Dancing Links paper: 6 options over 7 items.
 *  Unique solution: R1 {C,E,F} + R4 {A,D} + R5 {B,G}. */
const ROWS: number[][] = [
  [2, 4, 5], // R1: C E F
  [0, 3, 6], // R2: A D G
  [1, 2, 5], // R3: B C F
  [0, 3], // R4: A D
  [1, 6], // R5: B G
  [3, 4, 6], // R6: D E G
];

type DNote =
  | { k: "start" }
  | { k: "choose"; col: number; size: number }
  | { k: "dead"; col: number }
  | { k: "cover"; row: number; cols: number[] }
  | { k: "uncover"; row: number; cols: number[] }
  | { k: "solution"; n: number; rows: number[] }
  | { k: "done"; n: number };

interface DFrame {
  aliveCols: boolean[];
  aliveRows: boolean[];
  solution: number[];
  activeCol: number;
  activeRow: number;
  depth: number;
  solutions: number;
  note: DNote;
}

function buildFrames(): DFrame[] {
  const frames: DFrame[] = [];
  const aliveC: boolean[] = Array.from({ length: COLS.length }, () => true);
  const aliveR: boolean[] = Array.from({ length: ROWS.length }, () => true);
  const stack: number[] = [];
  let solutions = 0;

  const snap = (note: DNote, activeCol = -1, activeRow = -1): void => {
    frames.push({
      aliveCols: [...aliveC],
      aliveRows: [...aliveR],
      solution: [...stack],
      activeCol,
      activeRow,
      depth: stack.length,
      solutions,
      note,
    });
  };

  const colSize = (c: number): number =>
    ROWS.reduce((acc, cols, r) => acc + ((aliveR[r] ?? false) && cols.includes(c) ? 1 : 0), 0);

  const search = (): void => {
    const alive = COLS.map((_, c) => c).filter((c) => aliveC[c] ?? false);
    if (alive.length === 0) {
      solutions++;
      snap({ k: "solution", n: solutions, rows: [...stack] });
      return;
    }
    let best = alive[0] ?? 0;
    for (const c of alive) if (colSize(c) < colSize(best)) best = c;
    const size = colSize(best);
    snap({ k: "choose", col: best, size }, best);
    if (size === 0) {
      snap({ k: "dead", col: best }, best);
      return;
    }
    for (let r = 0; r < ROWS.length; r++) {
      if (!(aliveR[r] ?? false) || !(ROWS[r] ?? []).includes(best)) continue;
      // Cover: remove each column of this row, and every row touching them.
      const coveredCols = (ROWS[r] ?? []).filter((c) => aliveC[c] ?? false);
      const hiddenRows: number[] = [];
      stack.push(r);
      for (const c of coveredCols) {
        aliveC[c] = false;
        for (let r2 = 0; r2 < ROWS.length; r2++) {
          if ((aliveR[r2] ?? false) && (ROWS[r2] ?? []).includes(c)) {
            aliveR[r2] = false;
            hiddenRows.push(r2);
          }
        }
      }
      snap({ k: "cover", row: r, cols: coveredCols }, best, r);
      search();
      // Uncover in exact reverse order: the links dance back.
      for (const r2 of hiddenRows) aliveR[r2] = true;
      for (const c of coveredCols) aliveC[c] = true;
      stack.pop();
      snap({ k: "uncover", row: r, cols: [...coveredCols].reverse() }, best, r);
    }
  };

  snap({ k: "start" });
  search();
  snap({ k: "done", n: solutions });
  return frames;
}

const FRAMES: DFrame[] = buildFrames();
const END = FRAMES.length - 1;

// --- Layout -------------------------------------------------------------------

const X0 = 52;
const DX = 36;
const Y0 = 72;
const DYR = 34;
const SW = X0 + COLS.length * DX + 6;
const SH = Y0 + ROWS.length * DYR + 2;

function rowName(r: number): string {
  return `R${r + 1}`;
}
function rowItems(r: number): string {
  return (ROWS[r] ?? []).map((c) => COLS[c] ?? "?").join(" ");
}

const T = {
  en: {
    title: "Algorithm X with dancing links — cover, recurse, uncover",
    intro:
      "Knuth's own example: 7 items A–G, 6 options R1–R6, exactly one exact cover. Step through the search: the column with fewest 1s is chosen (fail-first), covering it detaches columns and rows from the lattice of links, a dead end triggers the dance — the same pointer writes, undone in reverse — and the search resumes where it left off.",
    step: "Step",
    play: "Play",
    pause: "Pause",
    reset: "Reset",
    depth: "depth",
    solutions: "solutions",
    partial: "Partial solution",
    empty: "(empty)",
    legend: "Columns = items to cover, rows = options. Faded = detached from the link lattice. The count under each letter is the number of live options covering it.",
    nStart: "The full matrix: every option linked into every item it covers. No choice made yet.",
    nChoose: (col: string, size: number) =>
      `Choose column ${col} — with ${size} live option${size === 1 ? "" : "s"}, it is the most constrained item. Branching here keeps the tree smallest (fail-first).`,
    nDead: (col: string) =>
      `Column ${col} has no live options left: this item can never be covered from here. Dead end — unwind.`,
    nCover: (row: string, items: string, cols: string) =>
      `Try ${row} {${items}}: cover column${cols.length > 1 ? "s" : ""} ${cols} — two pointer writes per link detach them, and every row touching them drops out of the lattice.`,
    nUncover: (row: string, cols: string) =>
      `Backtrack from ${row}: uncover ${cols} in exact reverse order. The removed nodes kept their own pointers, so the same two writes splice everything back — the links dance.`,
    nSolution: (n: number, rows: string) =>
      `Every column covered exactly once — solution #${n}: ${rows}. The search continues, hunting for more.`,
    nDone: (n: number) =>
      `Search exhausted: ${n} solution${n === 1 ? "" : "s"} in the entire tree, each state visited once, nothing copied — only pointers moved.`,
  },
  fr: {
    title: "L'algorithme X et les dancing links — couvrir, récurser, découvrir",
    intro:
      "L'exemple de Knuth lui-même : 7 éléments A–G, 6 options R1–R6, exactement une couverture exacte. Suivez la recherche pas à pas : la colonne au moins de 1 est choisie (l'échec d'abord), la couvrir détache colonnes et lignes du treillis de liens, une impasse déclenche la danse — les mêmes écritures de pointeurs, défaites en ordre inverse — et la recherche reprend là où elle s'était arrêtée.",
    step: "Pas",
    play: "Lecture",
    pause: "Pause",
    reset: "Réinitialiser",
    depth: "profondeur",
    solutions: "solutions",
    partial: "Solution partielle",
    empty: "(vide)",
    legend: "Colonnes = éléments à couvrir, lignes = options. Estompé = détaché du treillis de liens. Le compte sous chaque lettre est le nombre d'options vivantes qui la couvrent.",
    nStart: "La matrice complète : chaque option est chaînée dans chaque élément qu'elle couvre. Aucun choix encore.",
    nChoose: (col: string, size: number) =>
      `On choisit la colonne ${col} — avec ${size} option${size === 1 ? "" : "s"} vivante${size === 1 ? "" : "s"}, c'est l'élément le plus contraint. Brancher ici garde l'arbre le plus petit (l'échec d'abord).`,
    nDead: (col: string) =>
      `La colonne ${col} n'a plus aucune option vivante : cet élément ne pourra jamais être couvert d'ici. Impasse — on remonte.`,
    nCover: (row: string, items: string, cols: string) =>
      `On essaie ${row} {${items}} : couvrir ${cols.length > 1 ? "les colonnes" : "la colonne"} ${cols} — deux écritures de pointeurs par lien les détachent, et chaque ligne qui les touche sort du treillis.`,
    nUncover: (row: string, cols: string) =>
      `Retour arrière depuis ${row} : on découvre ${cols} en ordre exactement inverse. Les nœuds retirés ont gardé leurs propres pointeurs : les deux mêmes écritures recousent tout — les liens dansent.`,
    nSolution: (n: number, rows: string) =>
      `Chaque colonne couverte exactement une fois — solution n°${n} : ${rows}. La recherche continue, en quête d'autres.`,
    nDone: (n: number) =>
      `Recherche épuisée : ${n} solution${n === 1 ? "" : "s"} dans tout l'arbre, chaque état visité une fois, rien copié — seuls des pointeurs ont bougé.`,
  },
  es: {
    title: "El algoritmo X con dancing links — cubrir, recurrir, descubrir",
    intro:
      "El propio ejemplo de Knuth: 7 elementos A–G, 6 opciones R1–R6, exactamente una cobertura exacta. Recorre la búsqueda paso a paso: se elige la columna con menos unos (fallo primero), cubrirla desprende columnas y filas del entramado de enlaces, un callejón sin salida desata la danza — las mismas escrituras de punteros, deshechas en orden inverso — y la búsqueda se reanuda donde se había quedado.",
    step: "Paso",
    play: "Reproducir",
    pause: "Pausar",
    reset: "Reiniciar",
    depth: "profundidad",
    solutions: "soluciones",
    partial: "Solución parcial",
    empty: "(vacío)",
    legend: "Columnas = elementos por cubrir, filas = opciones. Atenuado = desprendido del entramado de enlaces. El número bajo cada letra es la cantidad de opciones vivas que la cubren.",
    nStart: "La matriz completa: cada opción está enlazada a cada elemento que cubre. Aún no se ha tomado ninguna decisión.",
    nChoose: (col: string, size: number) =>
      `Se elige la columna ${col} — con ${size} opción${size === 1 ? "" : "es"} viva${size === 1 ? "" : "s"}, es el elemento más restringido. Ramificar aquí mantiene el árbol lo más pequeño posible (fallo primero).`,
    nDead: (col: string) =>
      `La columna ${col} ya no tiene opciones vivas: este elemento no podrá cubrirse desde aquí. Callejón sin salida — se retrocede.`,
    nCover: (row: string, items: string, cols: string) =>
      `Se prueba ${row} {${items}}: cubrir ${cols.length > 1 ? "las columnas" : "la columna"} ${cols} — dos escrituras de punteros por enlace las desprenden, y toda fila que las toca sale del entramado.`,
    nUncover: (row: string, cols: string) =>
      `Retroceso desde ${row}: se descubre ${cols} en orden exactamente inverso. Los nodos retirados conservaron sus propios punteros, así que las dos mismas escrituras lo recomponen todo — los enlaces danzan.`,
    nSolution: (n: number, rows: string) =>
      `Cada columna cubierta exactamente una vez — solución n.º ${n}: ${rows}. La búsqueda continúa, en busca de más.`,
    nDone: (n: number) =>
      `Búsqueda agotada: ${n} solución${n === 1 ? "" : "es"} en todo el árbol, cada estado visitado una vez, nada copiado — solo se movieron punteros.`,
  },
};

function noteText(note: DNote, t: (typeof T)["en"]): string {
  switch (note.k) {
    case "start":
      return t.nStart;
    case "choose":
      return t.nChoose(COLS[note.col] ?? "?", note.size);
    case "dead":
      return t.nDead(COLS[note.col] ?? "?");
    case "cover":
      return t.nCover(rowName(note.row), rowItems(note.row), note.cols.map((c) => COLS[c] ?? "?").join(", "));
    case "uncover":
      return t.nUncover(rowName(note.row), note.cols.map((c) => COLS[c] ?? "?").join(", "));
    case "solution":
      return t.nSolution(note.n, note.rows.map(rowName).join(" + "));
    case "done":
      return t.nDone(note.n);
  }
}

export function DancingLinksLab() {
  const t = useT(T);
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [idx, setIdx] = useState(0);
  const [playing, setPlaying] = useState(false);

  const frame = FRAMES[Math.min(idx, END)];
  const atEnd = idx >= END;

  useEffect(() => {
    if (!playing || !visible) return;
    const id = setInterval(() => {
      setIdx((cur) => Math.min(cur + 1, END));
    }, 950);
    return () => clearInterval(id);
  }, [playing, visible]);

  useEffect(() => {
    // Stop the player once the trace is exhausted.
    if (playing && atEnd) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setPlaying(false);
    }
  }, [playing, atEnd]);

  if (!frame) return null;

  const solutionSet = new Set(frame.solution);
  const liveSize = (c: number): number =>
    ROWS.reduce((acc, cols, r) => acc + ((frame.aliveRows[r] ?? false) && cols.includes(c) ? 1 : 0), 0);
  const isSolutionFrame = frame.note.k === "solution";

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-80">
          <svg viewBox={`0 0 ${SW} ${SH}`} className="w-full">
            {/* column headers, counts and vertical links */}
            {COLS.map((label, c) => {
              const x = X0 + c * DX + DX / 2;
              const alive = frame.aliveCols[c] ?? false;
              const isActive = frame.activeCol === c;
              const last = ROWS.reduce((m, cols, r) => (cols.includes(c) ? r : m), -1);
              return (
                <g key={c} opacity={alive ? 1 : 0.22} style={{ transition: "opacity 250ms" }}>
                  <text
                    x={x} y={26} fontSize={13} fontWeight={700} textAnchor="middle"
                    className={isActive && alive ? "fill-primary" : "fill-foreground"}
                  >
                    {label}
                  </text>
                  {isActive && alive && (
                    <rect x={x - 11} y={32} width={22} height={2.5} rx={1} className="fill-primary" />
                  )}
                  <text x={x} y={48} fontSize={9} textAnchor="middle" className="fill-muted-foreground">
                    {alive ? liveSize(c) : "—"}
                  </text>
                  {last >= 0 && (
                    <line
                      x1={x} y1={54} x2={x} y2={Y0 + last * DYR}
                      className="stroke-muted-foreground" strokeWidth={1} opacity={0.35}
                    />
                  )}
                </g>
              );
            })}

            {/* rows: label, horizontal link, 1-cells */}
            {ROWS.map((cols, r) => {
              const y = Y0 + r * DYR;
              const alive = frame.aliveRows[r] ?? false;
              const inSolution = solutionSet.has(r);
              const isActive = frame.activeRow === r;
              const first = cols[0] ?? 0;
              const last = cols[cols.length - 1] ?? 0;
              const fade = !alive && !inSolution;
              return (
                <g key={r} opacity={fade ? 0.16 : 1} style={{ transition: "opacity 250ms" }}>
                  <text
                    x={X0 - 32} y={y + 4} fontSize={10} fontWeight={600}
                    className={inSolution ? "fill-emerald-600 dark:fill-emerald-400" : "fill-muted-foreground"}
                  >
                    {rowName(r)}
                  </text>
                  <line
                    x1={X0 + first * DX + DX / 2} y1={y}
                    x2={X0 + last * DX + DX / 2} y2={y}
                    className="stroke-muted-foreground" strokeWidth={1} opacity={0.35}
                  />
                  {cols.map((c) => {
                    const x = X0 + c * DX + DX / 2;
                    const colAlive = frame.aliveCols[c] ?? false;
                    return (
                      <rect
                        key={c}
                        x={x - 10} y={y - 10} width={20} height={20} rx={5}
                        className={cn(
                          inSolution
                            ? "fill-emerald-500"
                            : colAlive && alive
                              ? "fill-primary"
                              : "fill-muted-foreground",
                        )}
                        fillOpacity={inSolution ? 0.85 : colAlive && alive ? 0.75 : 0.35}
                        stroke={isActive ? "currentColor" : "none"}
                        style={{ transition: "fill-opacity 250ms" }}
                      />
                    );
                  })}
                  {isActive && (
                    <rect
                      x={X0 - 6} y={y - 14} width={COLS.length * DX + 8} height={28} rx={7}
                      fill="none"
                      className={frame.note.k === "uncover" ? "stroke-rose-500" : "stroke-amber-500"}
                      strokeWidth={1.5}
                    />
                  )}
                </g>
              );
            })}
          </svg>
          <p className="mt-1 text-center text-[11px] text-muted-foreground">{t.legend}</p>
        </div>

        <div className="min-w-56 flex-1 space-y-3">
          <div className="flex flex-wrap gap-2 text-[11px]">
            <span className="rounded-md border px-2 py-1">
              {t.depth}{" "}
              <span className="font-semibold tabular-nums">{frame.depth}</span>{" "}
              <span aria-hidden>
                {Array.from({ length: 3 }, (_, i) => (i < frame.depth ? "●" : "○")).join("")}
              </span>
            </span>
            <span
              className={cn(
                "rounded-md border px-2 py-1 tabular-nums",
                frame.solutions > 0 && "border-emerald-300 bg-emerald-500/10",
              )}
            >
              {frame.solutions} {t.solutions}
            </span>
          </div>

          <div className="rounded-md border px-3 py-2">
            <div className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
              {t.partial}
            </div>
            <div className="mt-1 flex flex-wrap gap-1 text-xs">
              {frame.solution.length === 0 ? (
                <span className="text-muted-foreground">{t.empty}</span>
              ) : (
                frame.solution.map((r) => (
                  <span
                    key={r}
                    className={cn(
                      "rounded border px-1.5 py-0.5 tabular-nums",
                      isSolutionFrame && "border-emerald-400 bg-emerald-500/10",
                    )}
                  >
                    {rowName(r)} {"{"}
                    {rowItems(r)}
                    {"}"}
                  </span>
                ))
              )}
            </div>
          </div>

          <p className="min-h-16 rounded-md border bg-muted/40 px-3 py-2 text-xs leading-relaxed text-muted-foreground">
            {noteText(frame.note, t)}
          </p>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setPlaying(false);
            setIdx((cur) => Math.min(cur + 1, END));
          }}
          disabled={atEnd}
        >
          {t.step}
        </Button>
        <Button size="sm" onClick={() => setPlaying((p) => !p)} disabled={!playing && atEnd}>
          {playing ? t.pause : t.play}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setPlaying(false);
            setIdx(0);
          }}
        >
          {t.reset}
        </Button>
      </div>
    </div>
  );
}
