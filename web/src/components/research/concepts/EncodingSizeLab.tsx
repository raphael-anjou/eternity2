import { useEffect, useMemo, useState } from "react";
import { useT } from "@/i18n";
import { Button } from "@/components/ui/button";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Math as InlineMath } from "@/components/research/Math";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// Interactive encoding-size explorer. Slide the board size n and colour count c
// and watch the direct clausal encoding blow up against Heule's seam-colour
// encoding — the counts are computed live from the formulas the page derives,
// on a log scale, with the community's practical pure-SAT ceiling (≈10×10) and
// Eternity II (16×16) marked. Below it, a six-clause unit-propagation cascade
// shows what CDCL actually feeds on: chains of forced assignments ending in a
// conflict that compresses to a short learned clause.

const T = {
  en: {
    title: "How big is the CNF? Slide and see",
    intro:
      "Both encodings are computed live from the page's own derivation: p = n² pieces, 4 rotations, n² cells give 4n⁴ placement variables. The direct encoding forbids every disagreeing pair of placements across every seam; Heule's seam encoding names the colour on each seam and forbids disagreement once per seam. The bars are log-scale — each tick is ×10.",
    boardSize: (n: number) => `Board size n = ${n} (${n}×${n})`,
    colors: (c: number) => `Edge colours c = ${c}`,
    variables: "variables",
    clauses: "clauses",
    direct: "Direct encoding (pairwise conflicts)",
    seam: "Heule seam encoding (compact)",
    breakdown: "Where the clauses go",
    rowCellEO: "each cell holds exactly one placement",
    rowPieceAMO: "each piece used at most once",
    rowConflict: "seam disagreement (pairwise, ≈ uniform colours)",
    rowImplication: "placement ⇒ its seam colours",
    rowSeamAMO: "one colour per seam",
    ceiling: "≈ practical ceiling for pure SAT on Eternity-style boards (community experiments)",
    ceilingShort: "pure-SAT ceiling",
    e2: "Eternity II",
    withinReach: "Within the range complete solvers have actually dispatched.",
    beyondReach:
      "Beyond every demonstrated pure-SAT success on Eternity-style instances — loadable, and unsolved.",
    e2Note: (v: string, cl: string) =>
      `At n = 16, c = 17 the direct form reaches ${v} variables and ≈ ${cl} conflict clauses — the same “hundreds of millions” the community reported for full-board CNFs.`,
    upTitle: "What CDCL feeds on: a unit-propagation cascade",
    upIntro:
      "Six clauses, one decision. Assigning x₁ = true forces x₂, then x₃, x₄, x₅ — each step is a clause with a single live literal — until clause 6 has none left: conflict. Walking the chain back, the whole conflict compresses to one decision, so the solver learns the short clause ¬x₁. Eternity II starves exactly this: its chains are one link long, so its learned clauses come out wide and useless.",
    step: "Step",
    reset: "Reset",
    play: "Auto-play",
    pause: "Pause",
    msg: [
      "Nothing assigned yet.",
      "Decision: x₁ = true.",
      "c2 has one live literal: x₂ = true (forced).",
      "c3 forces x₃ = true.",
      "c4 forces x₄ = true.",
      "c5 forces x₅ = true — a four-link chain from one decision.",
      "c6 has no live literal left: CONFLICT. Every link traces back to x₁ — learn the 1-literal clause ¬x₁.",
    ],
    learned: "learned clause: ¬x₁ — one literal, prunes half the tree",
    note: "Counts are exact for the constraint families shown, except seam disagreement, which treats colours as uniform (×(1−1/c)); real piece sets deviate a little. The seam encoding uses sequential at-most-one for the cell and piece constraints, as the compact encodings in Heule's paper do. The 10×10 ceiling and the full-board CNF sizes are the ones this page reports from community experiments.",
  },
  fr: {
    title: "Quelle taille fait la CNF ? Faites glisser",
    intro:
      "Les deux encodages sont calculés en direct depuis la dérivation de la page : p = n² pièces, 4 rotations, n² cases donnent 4n⁴ variables de placement. L'encodage direct interdit chaque paire de placements en désaccord sur chaque couture ; l'encodage à coutures de Heule nomme la couleur de chaque couture et interdit le désaccord une seule fois par couture. Les barres sont en échelle log — chaque graduation vaut ×10.",
    boardSize: (n: number) => `Taille du plateau n = ${n} (${n}×${n})`,
    colors: (c: number) => `Couleurs d'arête c = ${c}`,
    variables: "variables",
    clauses: "clauses",
    direct: "Encodage direct (conflits par paires)",
    seam: "Encodage à coutures de Heule (compact)",
    breakdown: "Où partent les clauses",
    rowCellEO: "chaque case porte exactement un placement",
    rowPieceAMO: "chaque pièce sert au plus une fois",
    rowConflict: "désaccord de couture (par paires, couleurs ≈ uniformes)",
    rowImplication: "placement ⇒ ses couleurs de couture",
    rowSeamAMO: "une couleur par couture",
    ceiling: "≈ plafond pratique du SAT pur sur les plateaux de type Eternity (expériences communautaires)",
    ceilingShort: "plafond SAT pur",
    e2: "Eternity II",
    withinReach: "Dans la plage que les solveurs complets ont réellement expédiée.",
    beyondReach:
      "Au-delà de tout succès démontré du SAT pur sur les instances de type Eternity — chargeable, et irrésolu.",
    e2Note: (v: string, cl: string) =>
      `À n = 16, c = 17, la forme directe atteint ${v} variables et ≈ ${cl} clauses de conflit — les mêmes « centaines de millions » que la communauté rapporte pour les CNF du plateau entier.`,
    upTitle: "Ce dont se nourrit le CDCL : une cascade de propagation unitaire",
    upIntro:
      "Six clauses, une décision. Poser x₁ = vrai force x₂, puis x₃, x₄, x₅ — chaque étape est une clause à un seul littéral vivant — jusqu'à ce que la clause 6 n'en ait plus : conflit. En remontant la chaîne, tout le conflit se compresse en une décision : le solveur apprend la clause courte ¬x₁. Eternity II affame exactement cela : ses chaînes n'ont qu'un maillon, donc ses clauses apprises sortent larges et inutiles.",
    step: "Étape",
    reset: "Réinitialiser",
    play: "Lecture auto",
    pause: "Pause",
    msg: [
      "Rien d'assigné pour l'instant.",
      "Décision : x₁ = vrai.",
      "c2 n'a qu'un littéral vivant : x₂ = vrai (forcé).",
      "c3 force x₃ = vrai.",
      "c4 force x₄ = vrai.",
      "c5 force x₅ = vrai — une chaîne de quatre maillons pour une décision.",
      "c6 n'a plus de littéral vivant : CONFLIT. Chaque maillon remonte à x₁ — on apprend la clause à 1 littéral ¬x₁.",
    ],
    learned: "clause apprise : ¬x₁ — un littéral, qui élague la moitié de l'arbre",
    note: "Les comptes sont exacts pour les familles montrées, sauf le désaccord de couture, qui traite les couleurs comme uniformes (×(1−1/c)) ; les jeux de pièces réels s'en écartent un peu. L'encodage à coutures utilise un au-plus-un séquentiel pour les contraintes de cases et de pièces, comme les encodages compacts de l'article de Heule. Le plafond 10×10 et les tailles de CNF du plateau entier sont ceux que cette page rapporte des expériences communautaires.",
  },
  es: {
    title: "¿Qué tamaño tiene la CNF? Desliza y compruébalo",
    intro:
      "Ambas codificaciones se calculan en vivo a partir de la propia derivación de la página: p = n² piezas, 4 rotaciones, n² celdas dan 4n⁴ variables de colocación. La codificación directa prohíbe cada par de colocaciones en desacuerdo sobre cada costura; la codificación por costuras de Heule nombra el color de cada costura y prohíbe el desacuerdo una sola vez por costura. Las barras están en escala logarítmica: cada marca vale ×10.",
    boardSize: (n: number) => `Tamaño del tablero n = ${n} (${n}×${n})`,
    colors: (c: number) => `Colores de arista c = ${c}`,
    variables: "variables",
    clauses: "cláusulas",
    direct: "Codificación directa (conflictos por pares)",
    seam: "Codificación por costuras de Heule (compacta)",
    breakdown: "A dónde van las cláusulas",
    rowCellEO: "cada celda contiene exactamente una colocación",
    rowPieceAMO: "cada pieza se usa como máximo una vez",
    rowConflict: "desacuerdo de costura (por pares, colores ≈ uniformes)",
    rowImplication: "colocación ⇒ sus colores de costura",
    rowSeamAMO: "un color por costura",
    ceiling: "≈ techo práctico del SAT puro en tableros de tipo Eternity (experimentos comunitarios)",
    ceilingShort: "techo del SAT puro",
    e2: "Eternity II",
    withinReach: "Dentro del rango que los solucionadores completos han despachado realmente.",
    beyondReach:
      "Más allá de todo éxito demostrado del SAT puro en instancias de tipo Eternity: cargable, y sin resolver.",
    e2Note: (v: string, cl: string) =>
      `Con n = 16, c = 17 la forma directa alcanza ${v} variables y ≈ ${cl} cláusulas de conflicto: los mismos «cientos de millones» que la comunidad reportó para las CNF del tablero completo.`,
    upTitle: "De qué se alimenta el CDCL: una cascada de propagación unitaria",
    upIntro:
      "Seis cláusulas, una decisión. Fijar x₁ = verdadero fuerza x₂, luego x₃, x₄, x₅ —cada paso es una cláusula con un solo literal vivo— hasta que la cláusula 6 no tiene ninguno: conflicto. Al remontar la cadena, todo el conflicto se comprime en una decisión, así que el solucionador aprende la cláusula corta ¬x₁. Eternity II mata de hambre justo esto: sus cadenas tienen un solo eslabón, de modo que sus cláusulas aprendidas salen anchas e inútiles.",
    step: "Paso",
    reset: "Reiniciar",
    play: "Reproducción automática",
    pause: "Pausar",
    msg: [
      "Nada asignado todavía.",
      "Decisión: x₁ = verdadero.",
      "c2 tiene un solo literal vivo: x₂ = verdadero (forzado).",
      "c3 fuerza x₃ = verdadero.",
      "c4 fuerza x₄ = verdadero.",
      "c5 fuerza x₅ = verdadero: una cadena de cuatro eslabones a partir de una decisión.",
      "c6 no tiene ningún literal vivo: CONFLICTO. Cada eslabón se remonta a x₁: se aprende la cláusula de 1 literal ¬x₁.",
    ],
    learned: "cláusula aprendida: ¬x₁ — un literal, poda la mitad del árbol",
    note: "Los recuentos son exactos para las familias de restricciones mostradas, salvo el desacuerdo de costura, que trata los colores como uniformes (×(1−1/c)); los conjuntos de piezas reales se desvían un poco. La codificación por costuras usa un al-máximo-uno secuencial para las restricciones de celda y de pieza, como las codificaciones compactas del artículo de Heule. El techo de 10×10 y los tamaños de CNF del tablero completo son los que esta página reporta a partir de experimentos comunitarios.",
  },
};

const choose2 = (k: number): number => (k * (k - 1)) / 2;

type RowKey = "rowCellEO" | "rowPieceAMO" | "rowConflict" | "rowImplication" | "rowSeamAMO";

interface Counts {
  vars: number;
  clauses: number;
  rows: { label: RowKey; value: number }[];
}

/** Direct encoding: 4n⁴ placement vars, everything pairwise. */
function directCounts(n: number, c: number): Counts {
  const cells = n * n;
  const perCell = 4 * cells; // p·4 placements available on one cell (p = n²)
  const seams = 2 * n * (n - 1);
  const cellEO = cells * (1 + choose2(perCell));
  const pieceAMO = cells * choose2(4 * cells); // p pieces × C(4n², 2)
  const conflict = seams * perCell * perCell * (1 - 1 / c);
  return {
    vars: 4 * cells * cells,
    clauses: cellEO + pieceAMO + conflict,
    rows: [
      { label: "rowCellEO", value: cellEO },
      { label: "rowPieceAMO", value: pieceAMO },
      { label: "rowConflict", value: conflict },
    ],
  };
}

/** Heule seam encoding: seam-colour vars + sequential AMO for cells/pieces. */
function seamCounts(n: number, c: number): Counts {
  const cells = n * n;
  const perCell = 4 * cells;
  const seams = 2 * n * (n - 1);
  // Sequential (ladder) at-most-one: ~3k clauses and k−1 aux vars for k literals.
  const cellEO = cells * (1 + 3 * (perCell - 1));
  const pieceAMO = cells * 3 * (4 * cells - 1);
  const auxVars = 2 * cells * (perCell - 1);
  const implications = 4 * (4 * cells * cells); // each placement ⇒ ≤4 seam colours
  const seamAMO = seams * choose2(c);
  return {
    vars: 4 * cells * cells + seams * c + auxVars,
    clauses: cellEO + pieceAMO + implications + seamAMO,
    rows: [
      { label: "rowCellEO", value: cellEO },
      { label: "rowPieceAMO", value: pieceAMO },
      { label: "rowImplication", value: implications },
      { label: "rowSeamAMO", value: seamAMO },
    ],
  };
}

function fmt(v: number): string {
  if (v >= 1e9) return `${(v / 1e9).toFixed(v >= 1e10 ? 0 : 1)} B`;
  if (v >= 1e6) return `${(v / 1e6).toFixed(v >= 1e7 ? 0 : 1)} M`;
  if (v >= 1e3) return `${(v / 1e3).toFixed(v >= 1e4 ? 0 : 1)} k`;
  return `${Math.round(v)}`;
}

const LOG_MIN = 2; // 10²
const LOG_MAX = 9; // 10⁹
const logPct = (v: number): number =>
  Math.max(0, Math.min(100, ((Math.log10(Math.max(v, 1)) - LOG_MIN) / (LOG_MAX - LOG_MIN)) * 100));

function LogBar({ value, className }: { value: number; className: string }) {
  return (
    <div className="relative h-3 flex-1 overflow-hidden rounded-sm bg-muted">
      {/* decade ticks */}
      {Array.from({ length: LOG_MAX - LOG_MIN - 1 }, (_, i) => (
        <div
          key={i}
          className="absolute inset-y-0 w-px bg-background/60"
          style={{ left: `${((i + 1) / (LOG_MAX - LOG_MIN)) * 100}%` }}
        />
      ))}
      <div
        className={cn("h-full rounded-sm transition-all duration-200", className)}
        style={{ width: `${logPct(value)}%` }}
      />
    </div>
  );
}

// --- Unit-propagation cascade -------------------------------------------------
// c1:(x1∨x5) c2:(¬x1∨x2) c3:(¬x2∨x3) c4:(¬x3∨x4) c5:(¬x2∨¬x4∨x5) c6:(¬x1∨¬x5)
// Step 1 is the decision x1 = true; steps 2..5 propagate x2..x5; step 6 conflicts.
const CLAUSES: { id: string; lits: { v: number; neg: boolean }[] }[] = [
  { id: "c1", lits: [{ v: 1, neg: false }, { v: 5, neg: false }] },
  { id: "c2", lits: [{ v: 1, neg: true }, { v: 2, neg: false }] },
  { id: "c3", lits: [{ v: 2, neg: true }, { v: 3, neg: false }] },
  { id: "c4", lits: [{ v: 3, neg: true }, { v: 4, neg: false }] },
  { id: "c5", lits: [{ v: 2, neg: true }, { v: 4, neg: true }, { v: 5, neg: false }] },
  { id: "c6", lits: [{ v: 1, neg: true }, { v: 5, neg: true }] },
];
/** Clause firing at each step (index = step − 1; null = the decision itself). */
const STEP_CLAUSE: (number | null)[] = [null, 1, 2, 3, 4, 5];
const MAX_STEP = 6;

function litText(l: { v: number; neg: boolean }): string {
  return `${l.neg ? "¬" : ""}x${l.v}`;
}

export function EncodingSizeLab() {
  const t = useT(T);
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [n, setN] = useState(16);
  const [c, setC] = useState(17);
  const [step, setStep] = useState(0);
  const [auto, setAuto] = useState(true);

  const direct = useMemo(() => directCounts(n, c), [n, c]);
  const seam = useMemo(() => seamCounts(n, c), [n, c]);
  const e2Direct = useMemo(() => directCounts(16, 17), []);

  // Auto-advance the cascade while visible; pause a beat on the conflict.
  useEffect(() => {
    if (!auto || !visible) return;
    const id = setInterval(() => {
      setStep((s) => (s >= MAX_STEP ? 0 : s + 1));
    }, 1300);
    return () => clearInterval(id);
  }, [auto, visible]);

  // Assignment state per variable after `step` propagations (1..5 → x1..x5).
  const assigned = (v: number): boolean => step >= v && v <= 5;
  const conflict = step >= MAX_STEP;

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="grid gap-4 sm:grid-cols-2">
        <div className="space-y-1.5">
          <Label>{t.boardSize(n)}</Label>
          <Slider min={4} max={16} step={1} value={n} onValueChange={(v) => setN(singleSliderValue(v))} />
          {/* reach strip: green up to the 10×10 ceiling, amber beyond */}
          <div className="relative mt-1 h-2 rounded-sm bg-muted">
            <div
              className="absolute inset-y-0 left-0 rounded-l-sm bg-emerald-500/50"
              style={{ width: `${((10 - 4) / (16 - 4)) * 100}%` }}
            />
            <div
              className="absolute inset-y-0 rounded-r-sm bg-amber-500/40"
              style={{ left: `${((10 - 4) / (16 - 4)) * 100}%`, right: 0 }}
            />
            <div
              className="absolute -inset-y-0.5 w-0.5 bg-foreground"
              style={{ left: `${((n - 4) / (16 - 4)) * 100}%` }}
            />
          </div>
          <div className="flex justify-between text-[10px] text-muted-foreground">
            <span>4</span>
            <span className="text-emerald-600" title={t.ceiling}>
              10 — {t.ceilingShort}
            </span>
            <span className="text-amber-600">16 — {t.e2}</span>
          </div>
        </div>
        <div className="space-y-1.5">
          <Label>{t.colors(c)}</Label>
          <Slider min={4} max={22} step={1} value={c} onValueChange={(v) => setC(singleSliderValue(v))} />
          <p className={cn("text-[11px]", n <= 10 ? "text-emerald-600" : "text-amber-600")}>
            {n <= 10 ? t.withinReach : t.beyondReach}
          </p>
        </div>
      </div>

      <div className="space-y-2">
        {(
          [
            { name: t.direct, counts: direct, bar: "bg-rose-500" },
            { name: t.seam, counts: seam, bar: "bg-emerald-500" },
          ] as const
        ).map(({ name, counts, bar }) => (
          <div key={name} className="space-y-1 rounded-md border p-2.5">
            <p className="text-xs font-medium">{name}</p>
            <div className="flex items-center gap-2 text-[11px]">
              <span className="w-16 shrink-0 text-muted-foreground">{t.variables}</span>
              <LogBar value={counts.vars} className={cn(bar, "opacity-60")} />
              <span className="w-14 shrink-0 text-right tabular-nums">{fmt(counts.vars)}</span>
            </div>
            <div className="flex items-center gap-2 text-[11px]">
              <span className="w-16 shrink-0 text-muted-foreground">{t.clauses}</span>
              <LogBar value={counts.clauses} className={bar} />
              <span className="w-14 shrink-0 text-right tabular-nums">{fmt(counts.clauses)}</span>
            </div>
          </div>
        ))}
        <details className="text-[11px] text-muted-foreground">
          <summary className="cursor-pointer select-none">{t.breakdown}</summary>
          <div className="mt-2 grid gap-3 sm:grid-cols-2">
            {(
              [
                { name: t.direct, counts: direct },
                { name: t.seam, counts: seam },
              ] as const
            ).map(({ name, counts }) => (
              <table key={name} className="w-full">
                <caption className="mb-1 text-left font-medium text-foreground">{name}</caption>
                <tbody>
                  {counts.rows.map((r) => (
                    <tr key={r.label}>
                      <td className="pr-2">{t[r.label]}</td>
                      <td className="text-right tabular-nums">{fmt(r.value)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ))}
          </div>
          <div className="mt-2 space-y-1">
            <InlineMath>{`V_{\\text{direct}} = n^2 \\cdot p \\cdot 4 = 4n^4, \\qquad C_{\\text{conflict}} \\approx 2n(n-1)\\,(4n^2)^2\\,(1 - 1/c)`}</InlineMath>
            <br />
            <InlineMath>{`V_{\\text{seam}} = 4n^4 + 2n(n-1)\\,c + \\text{aux}, \\qquad C_{\\text{seam}} \\approx 16n^4 + 2n(n-1)\\binom{c}{2}`}</InlineMath>
          </div>
        </details>
        <p className="text-[11px] text-muted-foreground">
          {t.e2Note(fmt(e2Direct.vars), fmt(e2Direct.rows[2]?.value ?? 0))}
        </p>
      </div>

      <div className="space-y-2 rounded-md border p-3">
        <h4 className="text-xs font-semibold">{t.upTitle}</h4>
        <p className="text-[11px] leading-relaxed text-muted-foreground">{t.upIntro}</p>
        <div className="flex flex-wrap gap-1.5">
          {CLAUSES.map((cl, ci) => {
            const firing = step >= 1 ? (STEP_CLAUSE[step - 1] ?? null) : null;
            const active = firing === ci;
            const isConflictClause = conflict && ci === 5;
            return (
              <span
                key={cl.id}
                className={cn(
                  "rounded-md border px-2 py-1 font-mono text-[11px] transition-colors",
                  isConflictClause
                    ? "border-rose-500 bg-rose-500/15"
                    : active
                      ? "border-amber-500 bg-amber-500/10"
                      : "",
                )}
              >
                {cl.id}: (
                {cl.lits.map((l, li) => {
                  // Literal status under the current partial assignment.
                  const val = assigned(l.v) ? !l.neg : null; // all assignments are "true"
                  return (
                    <span
                      key={li}
                      className={cn(
                        val === true && "font-bold text-emerald-600",
                        val === false && "text-rose-500 line-through",
                      )}
                    >
                      {li > 0 && " ∨ "}
                      {litText(l)}
                    </span>
                  );
                })}
                )
              </span>
            );
          })}
        </div>
        <p
          className={cn(
            "rounded-md border px-2 py-1.5 text-[11px]",
            conflict ? "border-rose-300 bg-rose-500/10" : "text-muted-foreground",
          )}
        >
          {t.msg[step] ?? ""}
          {conflict && <span className="mt-0.5 block font-medium text-emerald-600">{t.learned}</span>}
        </p>
        <div className="flex flex-wrap gap-2">
          <Button
            size="sm"
            variant="outline"
            onClick={() => {
              setAuto(false);
              setStep((s) => (s >= MAX_STEP ? 0 : s + 1));
            }}
          >
            {t.step}
          </Button>
          <Button size="sm" variant="outline" onClick={() => setAuto((a) => !a)}>
            {auto ? t.pause : t.play}
          </Button>
          <Button
            size="sm"
            variant="outline"
            onClick={() => {
              setAuto(false);
              setStep(0);
            }}
          >
            {t.reset}
          </Button>
        </div>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}
