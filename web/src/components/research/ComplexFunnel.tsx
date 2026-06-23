import { useT } from "@/i18n";
import data from "@/data/complex-theory.json";

// The E2 funnel, drawn from Brendan Owen's complex-theory numbers (McGavin's
// GMP run). The per-depth expected node count, on a log scale across all 256
// cells, shows the three regimes the community considers the key to the puzzle:
// exponential growth, a vast plateau where the tree is widest (and backtrackers
// burn ~99% of their time), and the funnel collapse where the last ~60 pieces
// each kill orders of magnitude of branches. Static SVG from committed data.

interface Point {
  depth: number;
  atDepth: number;
  cumulative: number;
}
const curve = data.curve as Point[];

const T = {
  en: {
    title: "The E2 funnel — expected branches at each depth",
    growth: "growth",
    plateau: "plateau — backtrackers burn ~99% of their time here",
    collapse: "collapse — the last ~60 pieces are tightly constrained",
    yLabel: "expected ways to extend (log scale)",
    xLabel: "depth (cells placed, of 256)",
    peak: "≈10⁴⁵ ways at the peak",
    caption:
      "Each point is the expected number of legal ways to extend a partial board at that depth, from complex theory. The tree explodes to ~10⁴⁵ wide in the middle, then funnels back down to the 14,702 expected solutions at the last cell. The plateau is the wall: a search has to cross an astronomically wide band where almost nothing prunes.",
  },
  fr: {
    title: "L'entonnoir d'E2 — branches attendues à chaque profondeur",
    growth: "croissance",
    plateau: "plateau — les backtrackers y brûlent ~99 % de leur temps",
    collapse: "effondrement — les ~60 dernières pièces sont très contraintes",
    yLabel: "façons attendues d'étendre (échelle log)",
    xLabel: "profondeur (cases posées, sur 256)",
    peak: "≈10⁴⁵ façons au pic",
    caption:
      "Chaque point est le nombre attendu de façons légales d'étendre un plateau partiel à cette profondeur, d'après la théorie complexe. L'arbre explose jusqu'à ~10⁴⁵ de large au milieu, puis se resserre vers les 14 702 solutions attendues à la dernière case. Le plateau est le mur : une recherche doit traverser une bande astronomiquement large où presque rien n'élague.",
  },
};

export function ComplexFunnel() {
  const t = useT(T);

  const W = 520;
  const H = 240;
  const padL = 36;
  const padR = 12;
  const padT = 14;
  const padB = 30;

  const maxDepth = 256;
  const logVals = curve.map((p) => Math.log10(Math.max(1, p.atDepth)));
  const maxLog = Math.max(...logVals); // ~45.7
  const x = (d: number) => padL + (d / maxDepth) * (W - padL - padR);
  const y = (logv: number) => padT + (1 - logv / maxLog) * (H - padT - padB);

  const pts = curve.map((p, i) => `${x(p.depth)},${y(logVals[i] ?? 0)}`).join(" ");

  // Regime bands.
  const bands = [
    { from: 1, to: 50, cls: "fill-emerald-400/10", label: t.growth },
    { from: 50, to: 200, cls: "fill-amber-400/15", label: t.plateau },
    { from: 200, to: 256, cls: "fill-sky-400/10", label: t.collapse },
  ];

  return (
    <div className="space-y-3">
      <h3 className="text-center text-sm font-medium">{t.title}</h3>
      <div className="mx-auto max-w-2xl">
        <svg viewBox={`0 0 ${W} ${H}`} className="w-full rounded-lg border bg-card">
          {bands.map((b) => (
            <rect
              key={b.from}
              x={x(b.from)}
              y={padT}
              width={x(b.to) - x(b.from)}
              height={H - padT - padB}
              className={b.cls}
            />
          ))}
          {/* y gridlines at powers of 10 */}
          {[0, 10, 20, 30, 40].map((lv) => (
            <g key={lv}>
              <line x1={padL} y1={y(lv)} x2={W - padR} y2={y(lv)} className="stroke-muted" strokeWidth={0.5} />
              <text x={4} y={y(lv) + 3} className="fill-muted-foreground text-[8px]">
                10^{lv}
              </text>
            </g>
          ))}
          <polyline points={pts} fill="none" className="stroke-foreground" strokeWidth={1.5} />
          {curve.map((p, i) => (
            <circle key={p.depth} cx={x(p.depth)} cy={y(logVals[i] ?? 0)} r={1.6} className="fill-foreground" />
          ))}
          {/* x ticks */}
          {[1, 64, 128, 192, 256].map((d) => (
            <text key={d} x={x(d)} y={H - padB + 14} textAnchor="middle" className="fill-muted-foreground text-[8px]">
              {d}
            </text>
          ))}
          <text x={W / 2} y={H - 2} textAnchor="middle" className="fill-muted-foreground text-[9px]">
            {t.xLabel}
          </text>
        </svg>
      </div>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-emerald-400/40" /> {t.growth}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-amber-400/50" /> {t.plateau}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-sky-400/40" /> {t.collapse}
        </span>
      </div>
      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
