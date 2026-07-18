import { useEffect, useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient, cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { Lab } from "@/components/research/Lab";
import { Button } from "@/components/ui/button";

// Edge slipping, made slidable. Two linked views driven by one slider (the
// allowed number of slipped edges, N):
//
//  1. the target-multiplication chart — how many near-miss boards exist at
//     score 480−N relative to perfect boards, using Max's community estimate
//     S(480−N) ≈ 2^(N−1)·C(420,N)·S(480) (groups.io msg 6390), on a log axis.
//     N=1 is a genuine gap: interior slips come in pairs (parity), and the
//     complex-theory slip table (msg 6412) puts an exact zero there.
//
//  2. the slip-gate strip — a 16×16 board in scan order with the N unlock
//     depths drawn on it, late (Verhaard/Blackwood style, depths 201–239) or,
//     for contrast, early (depths 41–79). The strip shows why late slips are
//     cheap: the earliest gate bounds how many later placements can ever sit
//     on top of a defect.
//
// Everything is computed live in log-space (no big numbers, no RNG, fully
// deterministic). The only animation is a frontier sweep across the strip —
// one cell per tick, gated on useRunWhileVisible, O(1) work per tick.

const MAX_SLIPS = 13; // 480 − 13 = 467, Verhaard's score
const CELLS = 256;
const LOG_TOP = 28; // chart headroom: multiplier at N=13 is ~10^27.8

const PAUSE_TICKS = 40;
const TICK_MS = 30;

/** log10 of Max's relative multiplier 2^(N−1)·C(420,N); NaN at N=1 (parity). */
function log10Mult(n: number): number {
  if (n === 0) return 0;
  if (n === 1) return Number.NaN;
  let l = (n - 1) * Math.log10(2);
  for (let k = 0; k < n; k++) l += Math.log10(420 - k);
  for (let k = 2; k <= n; k++) l -= Math.log10(k);
  return l;
}

const LOG_MULT = Array.from({ length: MAX_SLIPS + 1 }, (_, n) => log10Mult(n));

type GateMode = "late" | "early";

/** Unlock depths for n cumulative slips: 201–239 (late) or 41–79 (early). */
function gateDepths(n: number, mode: GateMode): number[] {
  if (n === 0) return [];
  const start = mode === "late" ? 201 : 41;
  const span = 38;
  if (n === 1) return [start + Math.round(span / 2)];
  return Array.from({ length: n }, (_, i) => start + Math.round((i * span) / (n - 1)));
}

const T = {
  en: {
    title: "The slip budget — more targets, one edge each",
    intro:
      "One slider, two consequences. Allowing N slipped edges multiplies the number of boards worth finding (left, log scale — Max's community estimate, msg 6390) while capping the score at 480 − N. Where the slips are allowed to land (right) decides whether the budget is cheap or fatal.",
    slider: "allowed slips N",
    target: (n: number) => `target score ${480 - n} / 480`,
    chartTitle: "Near-miss boards per perfect board (log scale)",
    axisNote: "relative to the 480s — Max's estimate, msg 6390",
    multPerfect: "×1 — the perfect boards themselves, never found by anyone",
    multParity:
      "×0 — a lone interior mismatch is parity-forbidden; the slip table has an exact zero here",
    mult: (mant: string, exp: number, score: number) =>
      `≈ ${mant} × 10^${exp} times more boards at score ${score} than at 480`,
    markRecord: "N = 10 → 470, the current record (Blackwood's break budget)",
    markVerhaard: "N = 13 → 467, Verhaard's prize board",
    stripTitle: "Where the slips may land (scan order, 256 placements)",
    gateLate: "Gates late (201–239)",
    gateEarly: "Gates early (41–79)",
    legendGate: "unlock depth (one more slip allowed from here on)",
    legendZone: "slip zone — mismatches may appear here",
    legendPlaced: "search frontier (animated sweep)",
    noGates: "N = 0: no gates — the search must be flawless for all 256 placements.",
    slipsUnlocked: (k: number, n: number) => `slips unlocked at the frontier: ${k} of ${n}`,
    lateMsg: (g: number, tail: number) =>
      `Earliest slip allowed at placement ${g} of 256: at most ${tail} later placements can ever sit on top of a defect, and the extra branching opens only where the tree has already narrowed to near-forced moves.`,
    earlyMsg: (g: number, tail: number) =>
      `Earliest slip allowed at placement ${g} of 256: up to ${tail} later placements inherit the defect, and the extra branching multiplies the widest part of the tree — the budget is spent where it buys least and costs most.`,
    footnote:
      "The late gates for N = 10 land on 201, 205, 209, … 239 — almost exactly Blackwood's real break indexes (201–239). Chart and strip are exact computations of the stated formulas; the formula itself is the community's rule-of-thumb estimate, not a theorem.",
    fmtMant: (x: number) => x.toFixed(1),
  },
  fr: {
    title: "Le budget de glissement — plus de cibles, une arête chacune",
    intro:
      "Un seul curseur, deux conséquences. Autoriser N arêtes glissées multiplie le nombre de plateaux qui valent la peine d'être trouvés (à gauche, échelle log — l'estimation communautaire de Max, msg 6390) tout en plafonnant le score à 480 − N. L'endroit où les glissements ont le droit d'atterrir (à droite) décide si le budget est bon marché ou fatal.",
    slider: "glissements autorisés N",
    target: (n: number) => `score visé ${480 - n} / 480`,
    chartTitle: "Plateaux quasi parfaits par plateau parfait (échelle log)",
    axisNote: "relatif aux 480 — estimation de Max, msg 6390",
    multPerfect: "×1 — les plateaux parfaits eux-mêmes, que personne n'a jamais trouvés",
    multParity:
      "×0 — un défaut intérieur isolé est interdit par parité ; la table des glissements affiche ici un zéro exact",
    mult: (mant: string, exp: number, score: number) =>
      `≈ ${mant} × 10^${exp} fois plus de plateaux au score ${score} qu'à 480`,
    markRecord: "N = 10 → 470, le record actuel (le budget de breaks de Blackwood)",
    markVerhaard: "N = 13 → 467, le plateau primé de Verhaard",
    stripTitle: "Où les glissements peuvent atterrir (ordre de balayage, 256 placements)",
    gateLate: "Portes tardives (201–239)",
    gateEarly: "Portes précoces (41–79)",
    legendGate: "profondeur de déverrouillage (un glissement de plus autorisé à partir d'ici)",
    legendZone: "zone de glissement — les défauts peuvent apparaître ici",
    legendPlaced: "front de recherche (balayage animé)",
    noGates: "N = 0 : aucune porte — la recherche doit être irréprochable sur les 256 placements.",
    slipsUnlocked: (k: number, n: number) => `glissements déverrouillés au front : ${k} sur ${n}`,
    lateMsg: (g: number, tail: number) =>
      `Premier glissement autorisé au placement ${g} sur 256 : au plus ${tail} placements ultérieurs peuvent reposer sur un défaut, et le branchement supplémentaire ne s'ouvre que là où l'arbre s'est déjà rétréci en coups quasi forcés.`,
    earlyMsg: (g: number, tail: number) =>
      `Premier glissement autorisé au placement ${g} sur 256 : jusqu'à ${tail} placements ultérieurs héritent du défaut, et le branchement supplémentaire multiplie la partie la plus large de l'arbre — le budget est dépensé là où il rapporte le moins et coûte le plus.`,
    footnote:
      "Les portes tardives pour N = 10 tombent sur 201, 205, 209, … 239 — presque exactement les vrais break indexes de Blackwood (201–239). Le graphique et la bande calculent exactement les formules énoncées ; la formule elle-même est l'estimation approchée de la communauté, pas un théorème.",
    fmtMant: (x: number) => x.toFixed(1).replace(".", ","),
  },
  es: {
    title: "El presupuesto de deslizamiento — más objetivos, una arista cada uno",
    intro:
      "Un solo control, dos consecuencias. Permitir N aristas deslizadas multiplica la cantidad de tableros que vale la pena encontrar (a la izquierda, escala logarítmica — la estimación comunitaria de Max, msg 6390) a la vez que limita la puntuación a 480 − N. Dónde se permite que caigan los deslizamientos (a la derecha) decide si el presupuesto es barato o fatal.",
    slider: "deslizamientos permitidos N",
    target: (n: number) => `puntuación objetivo ${480 - n} / 480`,
    chartTitle: "Tableros casi perfectos por tablero perfecto (escala logarítmica)",
    axisNote: "relativo a los 480 — estimación de Max, msg 6390",
    multPerfect: "×1 — los propios tableros perfectos, que nadie ha encontrado jamás",
    multParity:
      "×0 — un desajuste interior aislado está prohibido por paridad; aquí la tabla de deslizamientos marca un cero exacto",
    mult: (mant: string, exp: number, score: number) =>
      `≈ ${mant} × 10^${exp} veces más tableros con puntuación ${score} que con 480`,
    markRecord: "N = 10 → 470, el récord actual (el presupuesto de rupturas de Blackwood)",
    markVerhaard: "N = 13 → 467, el tablero premiado de Verhaard",
    stripTitle: "Dónde pueden caer los deslizamientos (orden de recorrido, 256 colocaciones)",
    gateLate: "Puertas tardías (201–239)",
    gateEarly: "Puertas tempranas (41–79)",
    legendGate: "profundidad de desbloqueo (un deslizamiento más permitido de aquí en adelante)",
    legendZone: "zona de deslizamiento — aquí pueden aparecer desajustes",
    legendPlaced: "frontera de búsqueda (barrido animado)",
    noGates: "N = 0: sin puertas — la búsqueda debe ser impecable en las 256 colocaciones.",
    slipsUnlocked: (k: number, n: number) => `deslizamientos desbloqueados en la frontera: ${k} de ${n}`,
    lateMsg: (g: number, tail: number) =>
      `Primer deslizamiento permitido en la colocación ${g} de 256: como mucho ${tail} colocaciones posteriores pueden asentarse sobre un defecto, y la ramificación adicional se abre solo donde el árbol ya se ha estrechado hasta movimientos casi forzados.`,
    earlyMsg: (g: number, tail: number) =>
      `Primer deslizamiento permitido en la colocación ${g} de 256: hasta ${tail} colocaciones posteriores heredan el defecto, y la ramificación adicional multiplica la parte más ancha del árbol — el presupuesto se gasta donde menos rinde y más cuesta.`,
    footnote:
      "Las puertas tardías para N = 10 caen en 201, 205, 209, … 239 — casi exactamente los break indexes reales de Blackwood (201–239). El gráfico y la banda calculan con exactitud las fórmulas enunciadas; la fórmula en sí es la estimación aproximada de la comunidad, no un teorema.",
    fmtMant: (x: number) => x.toFixed(1).replace(".", ","),
  },
};

export function EdgeSlipLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [n, setN] = useState(10);
  const [mode, setMode] = useState<GateMode>("late");
  const [step, setStep] = useState(0);

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setStep((s) => (s + 1) % (CELLS + PAUSE_TICKS)), TICK_MS);
    return () => clearInterval(id);
  }, [visible]);

  const gates = useMemo(() => gateDepths(n, mode), [n, mode]);

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        …
      </div>
    );
  }

  const frontier = Math.min(step, CELLS - 1);
  const unlocked = gates.filter((g) => g <= frontier + 1).length;
  const firstGate = gates[0];

  // --- chart geometry ---
  const CW = 470;
  const CH = 210;
  const PAD_L = 44;
  const PAD_B = 34;
  const PAD_T = 14;
  const plotH = CH - PAD_T - PAD_B;
  const barW = 22;
  const slot = (CW - PAD_L - 10) / (MAX_SLIPS + 1);
  const xOf = (i: number) => PAD_L + i * slot + (slot - barW) / 2;
  const yOf = (l: number) => PAD_T + plotH * (1 - l / LOG_TOP);

  const sel = LOG_MULT[n] ?? Number.NaN;
  const readout =
    n === 0
      ? t.multPerfect
      : n === 1
        ? t.multParity
        : (() => {
            const exp = Math.floor(sel);
            const mant = Math.pow(10, sel - exp);
            return t.mult(t.fmtMant(mant), exp, 480 - n);
          })();

  // --- strip geometry ---
  const CELL = 17;
  const SW = 16 * CELL;
  const gateSet = new Set(gates);

  return (
    <Lab ref={rootRef} title={t.title} intro={t.intro} note={t.footnote}>
      <div className="flex flex-wrap items-center gap-x-4 gap-y-2">
        <label className="flex min-w-56 flex-1 items-center gap-3 text-sm">
          <span className="whitespace-nowrap text-muted-foreground">{t.slider}</span>
          <input
            type="range"
            min={0}
            max={MAX_SLIPS}
            value={n}
            onChange={(e) => setN(Number(e.target.value))}
            className="w-full max-w-64"
          />
          <span className="w-6 text-right font-semibold tabular-nums">{n}</span>
        </label>
        <span className="rounded-md border bg-muted/40 px-2 py-1 text-xs font-medium tabular-nums">
          {t.target(n)}
        </span>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        {/* target-multiplication chart */}
        <div className="w-full min-w-64 max-w-lg flex-1">
          <svg viewBox={`0 0 ${CW} ${CH}`} className="w-full rounded-md border bg-muted/20" role="img" aria-label="Log-scale bar chart of near-miss boards per perfect board as the number of allowed edge slips N rises from 0 to 13">
            {/* y grid: 10^0, 10^7, 10^14, 10^21, 10^28 */}
            {[0, 7, 14, 21, 28].map((l) => (
              <g key={l}>
                <line
                  x1={PAD_L}
                  x2={CW - 6}
                  y1={yOf(l)}
                  y2={yOf(l)}
                  className="stroke-border"
                  strokeWidth={1}
                  strokeDasharray="3 4"
                />
                <text
                  x={PAD_L - 5}
                  y={yOf(l) + 3}
                  textAnchor="end"
                  className="fill-muted-foreground"
                  fontSize={9}
                >
                  {l === 0 ? "×1" : `10^${l}`}
                </text>
              </g>
            ))}
            {LOG_MULT.map((l, i) => {
              const selHere = i === n;
              if (Number.isNaN(l)) {
                // N = 1: parity zero — draw a gap marker, not a bar.
                return (
                  <g key={i}>
                    <text
                      x={xOf(i) + barW / 2}
                      y={yOf(0) - 4}
                      textAnchor="middle"
                      fontSize={9}
                      className={selHere ? "fill-rose-500 font-bold" : "fill-rose-400"}
                    >
                      ×0
                    </text>
                    <line
                      x1={xOf(i) + 3}
                      x2={xOf(i) + barW - 3}
                      y1={yOf(0)}
                      y2={yOf(0)}
                      className="stroke-rose-400"
                      strokeWidth={2}
                    />
                  </g>
                );
              }
              const h = Math.max(2, yOf(0) - yOf(l));
              const special = i === 10 || i === 13;
              return (
                <rect
                  key={i}
                  x={xOf(i)}
                  y={yOf(0) - h}
                  width={barW}
                  height={h}
                  rx={2}
                  className={cn(
                    selHere
                      ? "fill-sky-500"
                      : special
                        ? "fill-amber-500/70"
                        : "fill-muted-foreground/40",
                  )}
                />
              );
            })}
            {/* x labels: N on top row, score below */}
            {LOG_MULT.map((_, i) => (
              <g key={i} textAnchor="middle">
                <text
                  x={xOf(i) + barW / 2}
                  y={CH - PAD_B + 12}
                  fontSize={9}
                  className={i === n ? "fill-foreground font-bold" : "fill-muted-foreground"}
                >
                  {i}
                </text>
                <text
                  x={xOf(i) + barW / 2}
                  y={CH - PAD_B + 24}
                  fontSize={8}
                  className={i === n ? "fill-foreground" : "fill-muted-foreground/70"}
                >
                  {480 - i}
                </text>
              </g>
            ))}
            <text x={PAD_L} y={10} fontSize={9} className="fill-muted-foreground">
              {t.chartTitle}
            </text>
            <text x={CW - 6} y={CH - 4} textAnchor="end" fontSize={8} className="fill-muted-foreground/80">
              {t.axisNote}
            </text>
          </svg>
          <p className="mt-1 text-center text-[11px] font-medium tabular-nums">{readout}</p>
          <ul className="mt-1 space-y-0.5 text-[10px] text-muted-foreground">
            <li className="flex items-center gap-2">
              <span className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm bg-amber-500/70" />
              {t.markRecord}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm bg-amber-500/70" />
              {t.markVerhaard}
            </li>
          </ul>
        </div>

        {/* slip-gate strip */}
        <div className="w-full max-w-xs space-y-2">
          <svg viewBox={`-1 -1 ${SW + 2} ${SW + 2}`} className="w-full rounded-md border bg-muted/20">
            {Array.from({ length: CELLS }, (_, i) => {
              const x = (i % 16) * CELL;
              const y = Math.floor(i / 16) * CELL;
              const depth = i + 1; // placements are 1-based
              const inZone = firstGate !== undefined && depth >= firstGate;
              const isGate = gateSet.has(depth);
              const placed = i <= frontier;
              return (
                <rect
                  key={i}
                  x={x + 0.5}
                  y={y + 0.5}
                  width={CELL - 1}
                  height={CELL - 1}
                  rx={2}
                  className={cn(
                    isGate
                      ? "fill-amber-500"
                      : inZone
                        ? "fill-sky-500"
                        : "fill-muted-foreground",
                  )}
                  opacity={isGate ? (placed ? 0.95 : 0.55) : inZone ? (placed ? 0.55 : 0.22) : placed ? 0.4 : 0.14}
                />
              );
            })}
            {/* frontier marker */}
            <rect
              x={(frontier % 16) * CELL + 0.5}
              y={Math.floor(frontier / 16) * CELL + 0.5}
              width={CELL - 1}
              height={CELL - 1}
              rx={2}
              className="fill-foreground"
            />
          </svg>
          <p className="text-center text-[10px] text-muted-foreground">
            {t.stripTitle} — {t.slipsUnlocked(unlocked, n)}
          </p>
          <div className="flex justify-center gap-2">
            <Button
              size="sm"
              variant={mode === "late" ? "default" : "outline"}
              onClick={() => setMode("late")}
            >
              {t.gateLate}
            </Button>
            <Button
              size="sm"
              variant={mode === "early" ? "default" : "outline"}
              onClick={() => setMode("early")}
            >
              {t.gateEarly}
            </Button>
          </div>
          <ul className="space-y-0.5 text-[10px] text-muted-foreground">
            <li className="flex items-center gap-2">
              <span className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm bg-amber-500" />
              {t.legendGate}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm bg-sky-500/50" />
              {t.legendZone}
            </li>
            <li className="flex items-center gap-2">
              <span className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm bg-foreground" />
              {t.legendPlaced}
            </li>
          </ul>
          <p className="rounded-md border bg-muted/30 px-2.5 py-2 text-[11px] leading-relaxed">
            {firstGate === undefined
              ? t.noGates
              : mode === "late"
                ? t.lateMsg(firstGate, 256 - firstGate)
                : t.earlyMsg(firstGate, 256 - firstGate)}
          </p>
        </div>
      </div>
    </Lab>
  );
}
