import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// LADDER as a tournament. Round 1: many short probes reach different depths.
// Each round keeps only the deepest and gives them more time, so the survivors
// climb higher while the field shrinks. Deterministic depths (no RNG at render).

// Per-probe depth in round 0 (0..1 fraction of the board), hand-picked to look
// like a realistic spread; survivors carry forward and improve.
const ROUND0 = [0.32, 0.55, 0.18, 0.71, 0.41, 0.62, 0.27, 0.49, 0.66, 0.38, 0.58, 0.44];

const T = {
  en: {
    round: "Round",
    kept: "kept",
    of: "of",
    caption: "Each round keeps only the deepest starts and gives them a longer search, so the survivors climb while the field shrinks. The last one standing is promoted to a full run.",
    depth: "depth reached",
    busy: "Loading…",
  },
  fr: {
    round: "Tour",
    kept: "gardés",
    of: "sur",
    caption: "Chaque tour ne garde que les départs les plus profonds et leur accorde une recherche plus longue, si bien que les survivants montent pendant que le peloton se réduit. Le dernier en lice est promu vers une exécution complète.",
    depth: "profondeur atteinte",
    busy: "Chargement…",
  },
  es: {
    round: "Ronda",
    kept: "conservados",
    of: "de",
    caption: "Cada ronda conserva solo los inicios más profundos y les concede una búsqueda más larga, de modo que los supervivientes ascienden mientras el grupo se reduce. El último en pie asciende a una ejecución completa.",
    depth: "profundidad alcanzada",
    busy: "Cargando…",
  },
};

export function LadderDiagram() {
  const t = useT(T);
  const isClient = useIsClient();
  const [round, setRound] = useState(0);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setRound((r) => (r + 1) % 4), 1300);
    return () => clearInterval(id);
  }, [visible]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  // Survivors per round: keep top half each time, and bump survivors' depth up.
  const sorted = [...ROUND0].map((d, i) => ({ d, i })).sort((a, b) => b.d - a.d);
  const keepCount = [ROUND0.length, 6, 3, 1][round] ?? 1;
  const survivors = new Set(sorted.slice(0, keepCount).map((x) => x.i));
  const bump = [0, 0.08, 0.16, 0.22][round] ?? 0;

  const H = 150;
  const barW = 22;
  const gap = 8;
  const W = ROUND0.length * (barW + gap);

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto" style={{ maxWidth: W }}>
        <svg viewBox={`0 0 ${W} ${H + 20}`} className="w-full" role="img" aria-label="Tournament bar chart where each round keeps only the deepest probes and gives survivors a longer search, so kept bars climb while eliminated ones grey out">
          {ROUND0.map((d0, i) => {
            const alive = survivors.has(i);
            const d = Math.min(1, d0 + (alive ? bump : 0));
            const h = d * H;
            return (
              <g key={i}>
                <rect
                  x={i * (barW + gap)}
                  y={H - h}
                  width={barW}
                  height={h}
                  rx={3}
                  className={alive ? "fill-primary" : "fill-muted"}
                  style={{ transition: "height 300ms, y 300ms, fill 300ms" }}
                />
              </g>
            );
          })}
          <line x1={0} y1={H} x2={W} y2={H} className="stroke-border" strokeWidth={1} />
        </svg>
      </div>
      <div className="flex items-center justify-center gap-3 text-sm text-muted-foreground">
        <span className="font-medium text-foreground">
          {t.round} {round + 1}
        </span>
        <span className="tabular-nums">
          {keepCount} {t.kept} {t.of} {ROUND0.length}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
