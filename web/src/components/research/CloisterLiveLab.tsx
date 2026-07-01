import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import { Button } from "@/components/ui/button";
import { FRAME_BUDGET_MS, yieldToBrowser } from "@/lib/useRunWhileVisible";

// CLOISTER's idea, measured live: does anchoring the border first save work?
// We solve the same small puzzle two ways with the real engine — once border-
// first (the frame's edges constrain the interior from the start, CLOISTER-style)
// and once whole-board row-major (frame and interior solved together) — and
// compare the nodes each explored. Border-first rejects doomed interiors earlier,
// so it usually does less work. Same puzzle, same engine; only the order differs.

const SIZE = 6;
const NODE_CAP = 3_000_000; // keep each solve well under a second

interface Result {
  borderFirst: number;
  wholeBoard: number;
  bfSolved: boolean;
  wbSolved: boolean;
}

// The solve runs in time-boxed bursts (~8ms of synchronous work, measured with
// performance.now()), yielding to the browser between bursts so clicks and
// navigation stay responsive while the engine works.
async function solveCount(
  size: number,
  seed: number,
  kind: string,
  cancelled: () => boolean,
): Promise<{ nodes: number; solved: boolean }> {
  const puzzle = getGeneratedPuzzle(size, 5, seed);
  const path = getPath(kind, puzzle.width, puzzle.height, 0);
  const solver = createSolver(puzzle, path, { useHints: false });
  let r = solver.report();
  try {
    while (r.status === "running" && r.nodes < NODE_CAP && !cancelled()) {
      const deadline = performance.now() + FRAME_BUDGET_MS;
      do {
        r = solver.step(2_000);
      } while (r.status === "running" && r.nodes < NODE_CAP && performance.now() < deadline);
      if (r.status === "running" && r.nodes < NODE_CAP) await yieldToBrowser();
    }
  } finally {
    solver.free();
  }
  return { nodes: r.nodes, solved: r.status === "solved" };
}

const T = {
  en: {
    title: "Border-first vs whole-board — count the wasted work",
    intro:
      "The same small puzzle, solved twice by the real engine. Border-first fills the frame, then the interior, so the frame's edges constrain every interior placement from the start — a doomed interior is rejected early. Whole-board fills row by row, discovering frame conflicts late. Run it and compare the nodes each explored.",
    run: "Solve it both ways",
    again: "New puzzle",
    bf: "Border-first (CLOISTER)",
    wb: "Whole-board (row-major)",
    nodes: "nodes explored",
    saved: (pct: number) => `Border-first did ${pct}% less work`,
    moreWork: (pct: number) => `Border-first did ${pct}% more work on this one`,
    tie: "About the same on this puzzle",
    loading: "Loading the engine…",
    idle: "Press to solve the same puzzle both ways.",
    note: "Both runs are the real engine on the same 6×6 puzzle; only the fill order differs. The effect varies puzzle to puzzle (and a hard one may hit the node cap before solving), but border-first usually wins by rejecting interiors that can't meet the frame. The full CLOISTER goes further: it fixes a proven-perfect 16×16 frame and collects extra matched edges from interiors grown against it — value that can't be retrofitted afterwards.",
  },
  fr: {
    title: "Cadre d'abord vs plateau entier — comptez le travail gaspillé",
    intro:
      "Le même petit puzzle, résolu deux fois par le vrai moteur. « Cadre d'abord » remplit le cadre puis l'intérieur, si bien que les bords du cadre contraignent chaque pose intérieure dès le départ — un intérieur condamné est rejeté tôt. « Plateau entier » remplit rangée par rangée, découvrant les conflits de cadre tard. Lancez et comparez les nœuds explorés.",
    run: "Résoudre des deux façons",
    again: "Nouveau tirage",
    bf: "Cadre d'abord (CLOISTER)",
    wb: "Plateau entier (rangées)",
    nodes: "nœuds explorés",
    saved: (pct: number) => `Cadre d'abord a fait ${pct}% de travail en moins`,
    moreWork: (pct: number) => `Cadre d'abord a fait ${pct}% de travail en plus ici`,
    tie: "À peu près pareil sur ce puzzle",
    loading: "Chargement du moteur…",
    idle: "Appuyez pour résoudre le même puzzle des deux façons.",
    note: "Les deux exécutions sont le vrai moteur sur le même puzzle 6×6 ; seul l'ordre de remplissage change. L'effet varie d'un puzzle à l'autre (et un puzzle difficile peut atteindre le plafond de nœuds avant de résoudre), mais « cadre d'abord » l'emporte d'ordinaire en rejetant les intérieurs incompatibles avec le cadre. Le CLOISTER complet va plus loin : il fixe un cadre 16×16 prouvé parfait et collecte des bords appariés supplémentaires d'intérieurs bâtis contre lui — une valeur non récupérable après coup.",
  },
};

export function CloisterLiveLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [result, setResult] = useState<Result | null>(null);
  const [running, setRunning] = useState(false);
  const [seed, setSeed] = useState(1);
  const cancelRef = useRef(false);

  // Stop any in-flight chunked solve when the component unmounts (navigation).
  useEffect(() => {
    cancelRef.current = false;
    return () => {
      cancelRef.current = true;
    };
  }, []);

  const run = useCallback(async () => {
    if (!engineReady || running) return;
    setRunning(true);
    setResult(null);
    const cancelled = () => cancelRef.current;
    const bf = await solveCount(SIZE, seed, "border-first", cancelled);
    const wb = await solveCount(SIZE, seed, "row-major", cancelled);
    if (cancelRef.current) return;
    setResult({
      borderFirst: bf.nodes,
      wholeBoard: wb.nodes,
      bfSolved: bf.solved,
      wbSolved: wb.solved,
    });
    setRunning(false);
  }, [engineReady, running, seed]);

  const verdict = useMemo(() => {
    if (!result) return null;
    const { borderFirst, wholeBoard } = result;
    if (borderFirst === 0 || wholeBoard === 0) return null;
    const diff = (wholeBoard - borderFirst) / wholeBoard;
    if (Math.abs(diff) < 0.08) return { kind: "tie" as const, pct: 0 };
    if (diff > 0) return { kind: "saved" as const, pct: Math.round(diff * 100) };
    return { kind: "more" as const, pct: Math.round(-diff * 100) };
  }, [result]);

  if (!engineReady) {
    return (
      <div className="flex h-48 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const max = result ? Math.max(result.borderFirst, result.wholeBoard, 1) : 1;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      {result && (
        <div className="space-y-2">
          {(
            [
              { label: t.bf, nodes: result.borderFirst, color: "bg-emerald-500" },
              { label: t.wb, nodes: result.wholeBoard, color: "bg-sky-500" },
            ] as const
          ).map((row) => (
            <div key={row.label} className="space-y-1">
              <div className="flex items-baseline justify-between text-xs">
                <span className="font-medium">{row.label}</span>
                <span className="tabular-nums text-muted-foreground">
                  {row.nodes.toLocaleString()} {t.nodes}
                </span>
              </div>
              <div className="h-3 w-full overflow-hidden rounded-full bg-muted">
                <div
                  className={"h-full rounded-full " + row.color}
                  style={{ width: `${(row.nodes / max) * 100}%`, transition: "width 400ms" }}
                />
              </div>
            </div>
          ))}
          {verdict && (
            <p
              className={
                "rounded-md border px-3 py-2 text-center text-sm " +
                (verdict.kind === "saved"
                  ? "border-emerald-300 bg-emerald-500/10"
                  : verdict.kind === "more"
                    ? "border-amber-300 bg-amber-500/10"
                    : "")
              }
            >
              {verdict.kind === "saved"
                ? t.saved(verdict.pct)
                : verdict.kind === "more"
                  ? t.moreWork(verdict.pct)
                  : t.tie}
            </p>
          )}
        </div>
      )}

      <div className="flex flex-wrap items-center justify-center gap-3">
        <Button size="sm" onClick={() => void run()} disabled={running}>
          {t.run}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setSeed((s) => (s * 7 + 13) % 100000);
            setResult(null);
          }}
          disabled={running}
        >
          {t.again}
        </Button>
        {!result && !running && <span className="text-xs text-muted-foreground">{t.idle}</span>}
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
