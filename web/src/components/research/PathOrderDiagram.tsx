import { useT, useLang } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// The fill orders the DFS study compares, each drawn as an arrowed path that
// traces the order cells are visited, in the same arrow style the learn
// section's PathDiagrams uses (per-segment red-to-green hue ramp, a start dot,
// a shared arrowhead marker). The path is shown on a small representative grid
// (the shape of each order is identical in character at any size, and a 256-cell
// path is unreadable), so the direction of travel is legible: a row-major sweep,
// an inward spiral, the border ring then the interior, or Verhaard's comb. The
// sequences are the exact orders the engine's `path.rs` produces, recomputed
// here so the picture cannot drift from the code. Verhaard's comb and the
// bottom-up order are study-specific, so the paths are computed here rather than
// pulled from the engine's built-in path kinds.

const G = 8; // representative grid side (the real board is 16x16)
const N = G * G;
const CELL = 22;
const PAD = CELL / 2; // centre offset

type Order = { key: string; en: string; fr: string; seq: number[] };

function rowMajor(): number[] {
  return Array.from({ length: N }, (_, i) => i);
}
function rowMajorBottomUp(): number[] {
  const v: number[] = [];
  for (let r = G - 1; r >= 0; r--) for (let c = 0; c < G; c++) v.push(r * G + c);
  return v;
}
function spiralIn(): number[] {
  let top = 0,
    bottom = G - 1,
    left = 0,
    right = G - 1;
  const v: number[] = [];
  while (top <= bottom && left <= right) {
    for (let c = left; c <= right; c++) v.push(top * G + c);
    for (let r = top + 1; r <= bottom; r++) v.push(r * G + right);
    if (top < bottom) for (let c = right - 1; c >= left; c--) v.push(bottom * G + c);
    if (left < right) for (let r = bottom - 1; r > top; r--) v.push(r * G + left);
    top++;
    bottom--;
    left++;
    right--;
  }
  return v;
}
function spiralOut(): number[] {
  return spiralIn().reverse();
}
function borderFirst(): number[] {
  const isBorder = (r: number, c: number) => r === 0 || r === G - 1 || c === 0 || c === G - 1;
  const border: number[] = [];
  const interior: number[] = [];
  for (let r = 0; r < G; r++)
    for (let c = 0; c < G; c++) (isBorder(r, c) ? border : interior).push(r * G + c);
  return border.concat(interior);
}
function verhaardComb(horiz = Math.round(G * 0.6)): number[] {
  const v: number[] = [];
  for (let r = 0; r < horiz; r++) for (let c = 0; c < G; c++) v.push(r * G + c);
  for (let c = 0; c < G; c++) for (let r = horiz; r < G; r++) v.push(r * G + c);
  return v;
}

const ORDERS: Order[] = [
  { key: "row-major", en: "Row-major (best)", fr: "Ligne par ligne (meilleur)", seq: rowMajor() },
  { key: "bottom-up", en: "Bottom-up", fr: "De bas en haut", seq: rowMajorBottomUp() },
  { key: "spiral-in", en: "Spiral-in", fr: "Spirale entrante", seq: spiralIn() },
  { key: "spiral-out", en: "Spiral-out", fr: "Spirale sortante", seq: spiralOut() },
  { key: "border-first", en: "Border-first", fr: "Bordure d'abord", seq: borderFirst() },
  { key: "comb", en: "Verhaard comb", fr: "Peigne de Verhaard", seq: verhaardComb() },
];

const xy = (pos: number) => ({ x: (pos % G) * CELL + PAD, y: Math.floor(pos / G) * CELL + PAD });

const T = {
  en: {
    caption:
      "The six fill orders the study compares, each traced as the path the search walks, from red at the start to green at the end (shown on a small grid; the real board is 16x16). Row-major keeps a constant two-neighbour frontier and reaches deepest. The spirals and border-first drive that frontier through the hard corners early, which is why, without a heuristic, they stall. The comb runs a band of rows, then vertical teeth.",
    busy: "Loading…",
  },
  fr: {
    caption:
      "Les six ordres de remplissage comparés par l'étude, chacun tracé comme le chemin parcouru par la recherche, du rouge au départ au vert à la fin (sur une petite grille ; le vrai plateau est 16x16). Le parcours ligne par ligne garde un front à deux voisins constant et va le plus profond. Les spirales et la bordure-d'abord poussent ce front à travers les coins difficiles très tôt, d'où leur blocage sans heuristique. Le peigne remplit une bande de lignes, puis des dents verticales.",
    busy: "Chargement…",
  },
};

export function PathOrderDiagram() {
  const t = useT(T);
  const { lang } = useLang();
  const isClient = useIsClient();

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  return (
    <div className="space-y-3">
      <div className="grid grid-cols-2 gap-4 sm:grid-cols-3">
        {ORDERS.map((o) => {
          const markerId = `arr-${o.key}`;
          const first = xy(o.seq[0] ?? 0);
          return (
            <figure key={o.key} className="space-y-1.5">
              <svg
                viewBox={`0 0 ${G * CELL} ${G * CELL}`}
                className="aspect-square w-full rounded-md border bg-card"
                role="img"
                aria-label={`${o.en} fill order, drawn as the path the search walks across the grid, from red at the start to green at the end`}
              >
                <defs>
                  <marker
                    id={markerId}
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                  >
                    <path d="M0,0 L10,5 L0,10 z" fill="currentColor" />
                  </marker>
                </defs>
                {/* faint grid lines */}
                {Array.from({ length: G + 1 }, (_, i) => (
                  <g key={i} className="stroke-border">
                    <line x1={i * CELL} y1={0} x2={i * CELL} y2={G * CELL} strokeWidth={1} />
                    <line x1={0} y1={i * CELL} x2={G * CELL} y2={i * CELL} strokeWidth={1} />
                  </g>
                ))}
                {/* one arrow per step, hued red (start) to green (end) */}
                {o.seq.slice(0, -1).map((c, i) => {
                  const next = o.seq[i + 1];
                  if (next === undefined) return null;
                  const hue = (i / (o.seq.length - 2)) * 120;
                  const p1 = xy(c);
                  const p2 = xy(next);
                  const dx = p2.x - p1.x;
                  const dy = p2.y - p1.y;
                  const len = Math.hypot(dx, dy) || 1;
                  const trim = Math.min(8, len / 4);
                  return (
                    <line
                      key={i}
                      x1={p1.x + (dx / len) * trim}
                      y1={p1.y + (dy / len) * trim}
                      x2={p2.x - (dx / len) * trim}
                      y2={p2.y - (dy / len) * trim}
                      stroke={`hsl(${hue} 75% 45%)`}
                      strokeWidth={2}
                      markerEnd={`url(#${markerId})`}
                      color={`hsl(${hue} 75% 45%)`}
                    />
                  );
                })}
                <circle cx={first.x} cy={first.y} r={4} fill="hsl(0 75% 45%)" />
              </svg>
              <figcaption className="text-center text-xs font-medium text-muted-foreground">
                {lang === "fr" ? o.fr : o.en}
              </figcaption>
            </figure>
          );
        })}
      </div>
      <p className="text-sm leading-relaxed text-muted-foreground">{t.caption}</p>
    </div>
  );
}
