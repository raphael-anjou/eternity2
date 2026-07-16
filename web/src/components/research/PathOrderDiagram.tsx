import { useT, useLang } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// The fill orders the DFS study compares, drawn as small 16x16 grids. Each cell
// is shaded by WHEN it is filled: light for early in the sequence, dark for
// late, using the same single-hue emerald ramp the leaderboard heatmap uses, so
// the pages read as one system. The sequences are the exact orders the engine's
// `path.rs` produces (row-major, bottom-up, spiral-in, spiral-out, border-first,
// Verhaard comb), recomputed here so the picture cannot drift from the engine.

const W = 16;
const H = 16;
const N = W * H;
const CELL = 7;

type Order = { key: string; en: string; fr: string; seq: number[] };

function rowMajor(): number[] {
  return Array.from({ length: N }, (_, i) => i);
}
function rowMajorBottomUp(): number[] {
  const v: number[] = [];
  for (let r = H - 1; r >= 0; r--) for (let c = 0; c < W; c++) v.push(r * W + c);
  return v;
}
function spiralIn(): number[] {
  let top = 0,
    bottom = H - 1,
    left = 0,
    right = W - 1;
  const v: number[] = [];
  while (top <= bottom && left <= right) {
    for (let c = left; c <= right; c++) v.push(top * W + c);
    for (let r = top + 1; r <= bottom; r++) v.push(r * W + right);
    if (top < bottom) for (let c = right - 1; c >= left; c--) v.push(bottom * W + c);
    if (left < right) for (let r = bottom - 1; r > top; r--) v.push(r * W + left);
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
  const isBorder = (r: number, c: number) => r === 0 || r === H - 1 || c === 0 || c === W - 1;
  const border: number[] = [];
  const interior: number[] = [];
  for (let r = 0; r < H; r++)
    for (let c = 0; c < W; c++) (isBorder(r, c) ? border : interior).push(r * W + c);
  return border.concat(interior);
}
function verhaardComb(horiz = 10): number[] {
  const v: number[] = [];
  for (let r = 0; r < horiz; r++) for (let c = 0; c < W; c++) v.push(r * W + c);
  for (let c = 0; c < W; c++) for (let r = horiz; r < H; r++) v.push(r * W + c);
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

// order index -> emerald alpha over the card surface (early light, late dark).
function shade(orderIndex: number): string {
  const t = orderIndex / (N - 1); // 0..1
  const alpha = 0.12 + 0.82 * t;
  return `rgba(16, 185, 129, ${alpha.toFixed(3)})`;
}

const T = {
  en: {
    caption:
      "The six fill orders the study compares, each on the 16x16 board. A cell's shade shows when it is filled: pale for the first placements, deep green for the last. Row-major keeps a constant two-neighbour frontier and reaches deepest; the spirals and border-first drive that frontier through the hard corners early, which is why, without a heuristic, they stall.",
    busy: "Loading…",
    early: "filled first",
    late: "filled last",
  },
  fr: {
    caption:
      "Les six ordres de remplissage comparés par l'étude, chacun sur le plateau 16x16. La teinte d'une case indique quand elle est posée : pâle pour les premières, vert foncé pour les dernières. Le parcours ligne par ligne garde un front à deux voisins constant et va le plus profond ; les spirales et la bordure-d'abord poussent ce front à travers les coins difficiles très tôt, ce qui explique leur blocage sans heuristique.",
    busy: "Chargement…",
    early: "posée en premier",
    late: "posée en dernier",
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
          // pos -> order index (seq is a full permutation, so every pos is set)
          const orderOf = new Array<number>(N).fill(0);
          o.seq.forEach((pos, i) => (orderOf[pos] = i));
          return (
            <figure key={o.key} className="space-y-1.5">
              <svg
                viewBox={`0 0 ${W * CELL} ${H * CELL}`}
                className="w-full rounded border bg-card"
                role="img"
                aria-label={`${o.en} fill order on a 16 by 16 board, shaded from pale (filled first) to deep green (filled last)`}
              >
                {Array.from({ length: N }, (_, pos) => {
                  const r = Math.floor(pos / W);
                  const c = pos % W;
                  return (
                    <rect
                      key={pos}
                      x={c * CELL}
                      y={r * CELL}
                      width={CELL}
                      height={CELL}
                      fill={shade(orderOf[pos] ?? 0)}
                      stroke="rgba(0,0,0,0.06)"
                      strokeWidth={0.4}
                    />
                  );
                })}
              </svg>
              <figcaption className="text-center text-xs font-medium text-muted-foreground">
                {lang === "fr" ? o.fr : o.en}
              </figcaption>
            </figure>
          );
        })}
      </div>
      <div className="flex items-center justify-center gap-2 text-xs text-muted-foreground">
        <span>{t.early}</span>
        <span className="inline-block h-2.5 w-24 rounded" style={{ background: "linear-gradient(to right, rgba(16,185,129,0.12), rgba(16,185,129,0.94))" }} aria-hidden />
        <span>{t.late}</span>
      </div>
      <p className="text-sm leading-relaxed text-muted-foreground">{t.caption}</p>
    </div>
  );
}
