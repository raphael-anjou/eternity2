import { useT } from "@/i18n";

// A schematic map of the record-board "archipelago". When you connect every
// strong board to every other one it lies within a short hop of (Hamming < 100
// cells different), the ≥455 records fall into 47 components — mostly tiny
// islands. A few small families cluster (reachable from each other by small
// piece-cycles); the very best boards, including McGavin's 469, sit alone, ~247
// cells from anything else. This is an illustration of that documented structure
// (vol-65 / vol-118 analysis), not a live computation — the positions are
// schematic, the Hamming numbers are the measured ones.

const CLUSTER_A = [
  { x: 78, y: 86 },
  { x: 120, y: 64 },
  { x: 150, y: 104 },
  { x: 104, y: 128 },
  { x: 64, y: 132 },
];
// edges among Cluster A (small-cycle reachable, Hamming 34–44)
const A_EDGES: [number, number][] = [
  [0, 1],
  [1, 2],
  [2, 3],
  [3, 4],
  [4, 0],
  [0, 3],
];

const T = {
  en: {
    clusterA: "a small family — reachable from each other by small swaps (≈40 cells apart)",
    clusterB: "another 459, isolated",
    mcgavin: "McGavin 469",
    mcgavinNote: "≥247 cells from anything else",
    far: "≈251 cells apart — a board-spanning swap",
    legendIsland: "a record board",
    legendHop: "small swap (reachable)",
    caption:
      "Connect every strong board to every other one within a short hop, and the ≥455 records fall into 47 separate islands — most of them singletons. A handful form small families; the very best boards, McGavin's 469 among them, sit alone, a whole-board swap away from anything else. There is no path of small steps between islands.",
  },
  fr: {
    clusterA: "une petite famille — atteignables entre elles par de petits échanges (≈40 cellules d'écart)",
    clusterB: "un autre 459, isolé",
    mcgavin: "McGavin 469",
    mcgavinNote: "≥247 cellules de tout le reste",
    far: "≈251 cellules d'écart — un échange à l'échelle du plateau",
    legendIsland: "un plateau record",
    legendHop: "petit échange (atteignable)",
    caption:
      "Reliez chaque bon plateau à tout autre situé à un court saut, et les records ≥455 se répartissent en 47 îles distinctes — la plupart isolées. Une poignée forment de petites familles ; les tout meilleurs plateaux, dont le 469 de McGavin, sont seuls, à un échange de tout le plateau de quoi que ce soit d'autre. Il n'existe aucun chemin de petits pas entre les îles.",
  },
};

export function BasinArchipelagoDiagram() {
  const t = useT(T);
  return (
    <figure className="mx-auto max-w-2xl space-y-3">
      <svg viewBox="0 0 360 240" className="w-full rounded-lg border bg-muted/20" role="img" aria-label={t.caption}>
        {/* Cluster A edges */}
        {A_EDGES.map(([a, b], i) => {
          const pa = CLUSTER_A[a];
          const pb = CLUSTER_A[b];
          if (!pa || !pb) return null;
          return (
            <line
              key={i}
              x1={pa.x}
              y1={pa.y}
              x2={pb.x}
              y2={pb.y}
              className="stroke-sky-400"
              strokeWidth={1.5}
            />
          );
        })}
        {/* Cluster A nodes */}
        {CLUSTER_A.map((p, i) => (
          <circle key={i} cx={p.x} cy={p.y} r={7} className="fill-sky-500" />
        ))}
        <text x={107} y={168} textAnchor="middle" className="fill-current text-sky-700 dark:text-sky-300" fontSize={8}>
          <title>{t.clusterA}</title>
          Cluster A
        </text>

        {/* isolated second 459 (Cluster B) */}
        <circle cx={262} cy={70} r={7} className="fill-violet-500" />
        <text x={262} y={52} textAnchor="middle" className="fill-current text-violet-700 dark:text-violet-300" fontSize={8}>
          {t.clusterB}
        </text>

        {/* McGavin singleton */}
        <circle cx={300} cy={170} r={9} className="fill-amber-500" />
        <text x={300} y={196} textAnchor="middle" className="fill-current text-amber-700 dark:text-amber-300" fontSize={9} fontWeight={700}>
          {t.mcgavin}
        </text>
        <text x={300} y={207} textAnchor="middle" className="fill-muted-foreground" fontSize={7}>
          {t.mcgavinNote}
        </text>

        {/* the great gulf: dashed long-distance non-edges */}
        <line x1={150} y1={104} x2={262} y2={70} className="stroke-muted-foreground" strokeWidth={1} strokeDasharray="3 4" />
        <line x1={150} y1={104} x2={291} y2={166} className="stroke-muted-foreground" strokeWidth={1} strokeDasharray="3 4" />
        <text x={210} y={150} textAnchor="middle" className="fill-muted-foreground" fontSize={7.5} fontStyle="italic">
          {t.far}
        </text>

        {/* legend */}
        <g transform="translate(12,214)">
          <circle cx={6} cy={0} r={5} className="fill-sky-500" />
          <text x={16} y={3} className="fill-muted-foreground" fontSize={8}>
            {t.legendIsland}
          </text>
          <line x1={120} y1={0} x2={140} y2={0} className="stroke-sky-400" strokeWidth={1.5} />
          <text x={146} y={3} className="fill-muted-foreground" fontSize={8}>
            {t.legendHop}
          </text>
        </g>
      </svg>
      <figcaption className="text-xs leading-relaxed text-muted-foreground">{t.caption}</figcaption>
    </figure>
  );
}
