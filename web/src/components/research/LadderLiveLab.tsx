import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import type { SolverHandle } from "@/engine";
import type { Puzzle } from "@/lib/types";
import { Button } from "@/components/ui/button";
import { FRAME_BUDGET_MS, yieldToBrowser } from "@/lib/useRunWhileVisible";

// LADDER, run for real. Rung 1: many cheap probes (distinct seeds/scan orders),
// each given a tiny budget; we keep the ones that reached deepest. Rung 2: those
// survivors get a bigger budget; keep the best again. Rung 3: the finalists run
// longest. The deepest start, concentrated on, tends to be the one that finishes
// — "promote the deepest" beats "restart at random." Every probe is a real engine
// run; the bars are real depths.

const SIZE = 6;
const SCANS = ["row-major", "snake", "spiral-in", "diagonal", "column-major", "spiral-out"];

interface Probe {
  id: number;
  scan: string;
  seed: number;
  solver: SolverHandle;
  depth: number;
  best: number;
  solved: boolean;
  alive: boolean;
}

const RUNGS = [
  { probes: 12, budget: 2_000, keep: 6 },
  { probes: 6, budget: 12_000, keep: 3 },
  { probes: 3, budget: 80_000, keep: 1 },
];

const T = {
  en: {
    title: "Flood, then promote the deepest — live",
    intro:
      "Rung 1 fires twelve cheap probes, each a real search with a tiny budget. We keep the six that reached deepest. Rung 2 gives those a bigger budget and keeps three; rung 3 gives the finalists the most. Concentrating effort on the deepest starts, rather than restarting at random, is what lets one of them finish.",
    run: "Run the ladder",
    again: "Run again",
    rung: (i: number) => `Rung ${i + 1}`,
    kept: "promoted",
    out: "dropped",
    solved: "solved!",
    depth: "depth reached",
    loading: "Loading the engine…",
    note: "Each bar is a real probe's depth on a 6×6 puzzle, run live. The full LADDER adds diversity dedup (don't promote near-identical prefixes) and exact endgames; on the 16×16 it reached a 451 board with no record to copy. The lever shown here is the core one: deepest-first promotion concentrates the budget where progress is real.",
    idle: "Press run to flood the board with probes.",
  },
  fr: {
    title: "Inonder, puis promouvoir les plus profonds — en direct",
    intro:
      "Le palier 1 lance douze sondes bon marché, chacune une vraie recherche à petit budget. On garde les six qui sont allées le plus profond. Le palier 2 leur donne un plus gros budget et en garde trois ; le palier 3 donne le plus aux finalistes. Concentrer l'effort sur les départs les plus profonds, plutôt que de redémarrer au hasard, est ce qui permet à l'un d'eux de finir.",
    run: "Lancer l'échelle",
    again: "Relancer",
    rung: (i: number) => `Palier ${i + 1}`,
    kept: "promu",
    out: "éliminé",
    solved: "résolu !",
    depth: "profondeur atteinte",
    loading: "Chargement du moteur…",
    note: "Chaque barre est la profondeur d'une vraie sonde sur un puzzle 6×6, en direct. Le LADDER complet ajoute une déduplication de diversité (ne pas promouvoir des préfixes quasi identiques) et des fins de partie exactes ; sur le 16×16 il a atteint un plateau 451 sans record à copier. Le levier montré ici est l'essentiel : la promotion des plus profonds concentre le budget là où le progrès est réel.",
    idle: "Appuyez pour inonder le plateau de sondes.",
  },
  es: {
    title: "Inundar, luego promover las más profundas — en vivo",
    intro:
      "El peldaño 1 lanza doce sondas económicas, cada una una búsqueda real con un presupuesto mínimo. Nos quedamos con las seis que llegaron más profundo. El peldaño 2 les da un presupuesto mayor y conserva tres; el peldaño 3 les da el máximo a las finalistas. Concentrar el esfuerzo en los arranques más profundos, en lugar de reiniciar al azar, es lo que permite que una de ellas termine.",
    run: "Ejecutar la escalera",
    again: "Ejecutar de nuevo",
    rung: (i: number) => `Peldaño ${i + 1}`,
    kept: "promovida",
    out: "descartada",
    solved: "¡resuelto!",
    depth: "profundidad alcanzada",
    loading: "Cargando el motor…",
    note: "Cada barra es la profundidad de una sonda real sobre un puzzle 6×6, en vivo. El LADDER completo añade deduplicación por diversidad (no promover prefijos casi idénticos) y finales de partida exactos; en el 16×16 alcanzó un tablero de 451 sin ningún récord que copiar. La palanca que se muestra aquí es la esencial: la promoción de las más profundas concentra el presupuesto donde el progreso es real.",
    idle: "Pulsa ejecutar para inundar el tablero de sondas.",
  },
};

export function LadderLiveLab() {
  const t = useT(T);
  const engineReady = useEngine();
  const [probes, setProbes] = useState<Probe[]>([]);
  const [rung, setRung] = useState<number>(-1);
  const [running, setRunning] = useState(false);
  const cancelRef = useRef(false);
  const TOTAL = SIZE * SIZE;

  // Abort any in-flight ladder run on unmount so no loop survives navigation.
  useEffect(() => {
    cancelRef.current = false;
    return () => {
      cancelRef.current = true;
    };
  }, []);

  const run = useCallback(async () => {
    if (!engineReady || running) return;
    setRunning(true);
    setProbes([]);
    setRung(-1);

    const puzzle: Puzzle = getGeneratedPuzzle(SIZE, 5, 1);

    // Rung 1: spawn fresh probes (distinct scan × seed).
    const rung1 = RUNGS[0];
    if (!rung1) {
      setRunning(false);
      return;
    }
    let live: Probe[] = [];
    for (let i = 0; i < rung1.probes; i++) {
      const scan = SCANS[i % SCANS.length] ?? "row-major";
      const seed = i + 1;
      const path = getPath(scan, puzzle.width, puzzle.height, seed);
      const solver = createSolver(puzzle, path, { useHints: false, shufflePieces: true, seed });
      live.push({ id: i, scan, seed, solver, depth: 0, best: 0, solved: false, alive: true });
    }

    // Free every solver still alive (used at the end and on cancellation).
    const freeLive = () => {
      live.forEach((p) => {
        if (p.alive) {
          p.alive = false;
          p.solver.free();
        }
      });
    };

    for (let r = 0; r < RUNGS.length; r++) {
      const rungCfg = RUNGS[r];
      if (!rungCfg) break;
      setRung(r);
      const { budget, keep } = rungCfg;
      // Run each live probe for this rung's budget, in time-boxed bursts
      // (~8ms of synchronous work) with yields in between so the main thread
      // stays free for clicks and navigation.
      for (const probe of live) {
        if (cancelRef.current) break;
        let rep = probe.solver.report();
        let spent = 0;
        while (rep.status === "running" && spent < budget && !cancelRef.current) {
          const deadline = performance.now() + FRAME_BUDGET_MS;
          do {
            const chunk = Math.min(1_000, budget - spent);
            rep = probe.solver.step(chunk);
            spent += chunk;
          } while (rep.status === "running" && spent < budget && performance.now() < deadline);
          if (rep.status === "running" && spent < budget) await yieldToBrowser();
        }
        probe.best = Math.max(probe.best, rep.bestPlaced);
        probe.depth = rep.bestPlaced;
        probe.solved = rep.status === "solved";
      }
      if (cancelRef.current) {
        freeLive();
        return;
      }
      // show this rung's result
      setProbes(live.map((p) => ({ ...p })));
      // let the UI paint between rungs
      await new Promise((res) => setTimeout(res, 700));
      if (cancelRef.current) {
        freeLive();
        return;
      }

      // keep the deepest `keep`; free the rest
      const ranked = [...live].sort((a, b) => b.depth - a.depth);
      const survivors = ranked.slice(0, keep);
      const survivorIds = new Set(survivors.map((p) => p.id));
      live.forEach((p) => {
        if (!survivorIds.has(p.id)) {
          p.alive = false;
          p.solver.free();
        }
      });
      setProbes(live.map((p) => ({ ...p })));
      // survivors carry forward; non-survivors are already freed above.
      live = survivors;
      if (survivors.some((p) => p.solved)) break;
      await new Promise((res) => setTimeout(res, 500));
      if (cancelRef.current) {
        freeLive();
        return;
      }
    }
    // free the survivors that are still alive
    freeLive();
    setRunning(false);
  }, [engineReady, running]);

  const maxDepth = useMemo(() => Math.max(TOTAL, ...probes.map((p) => p.depth)), [probes, TOTAL]);

  if (!engineReady) {
    return (
      <div className="flex h-64 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  const H = 150;
  const barW = 24;
  const gap = 8;
  const W = Math.max(1, probes.length) * (barW + gap);

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      {probes.length > 0 && (
        <div className="mx-auto" style={{ maxWidth: W }}>
          <svg viewBox={`0 0 ${W} ${H + 20}`} className="w-full" role="img" aria-label="Bar chart of the depth each live probe reached, with survivors promoted and dropped probes greyed out at each ladder rung">
            {probes.map((p, i) => {
              const h = (p.depth / maxDepth) * H;
              const fill = p.solved
                ? "fill-emerald-500"
                : p.alive
                  ? "fill-primary"
                  : "fill-muted";
              return (
                <g key={p.id}>
                  <rect
                    x={i * (barW + gap)}
                    y={H - h}
                    width={barW}
                    height={h}
                    rx={3}
                    className={fill}
                    style={{ transition: "height 300ms, y 300ms, fill 300ms" }}
                  />
                </g>
              );
            })}
            <line x1={0} y1={H} x2={W} y2={H} className="stroke-border" strokeWidth={1} />
          </svg>
        </div>
      )}

      <div className="flex flex-wrap items-center justify-center gap-3">
        <Button size="sm" onClick={() => void run()} disabled={running}>
          {probes.length ? t.again : t.run}
        </Button>
        {rung >= 0 && (
          <span className="rounded-md border px-2 py-1 text-xs font-medium">
            {t.rung(rung)}
            {probes.some((p) => p.solved) ? " · " + t.solved : ""}
          </span>
        )}
        {!probes.length && !running && (
          <span className="text-xs text-muted-foreground">{t.idle}</span>
        )}
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
