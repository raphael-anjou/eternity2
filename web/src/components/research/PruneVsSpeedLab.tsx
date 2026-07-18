import { useState } from "react";
import { useT } from "@/i18n";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";

// An interactive feel for why pruning beats speed. The search tree has ~b^d
// leaves. A speedup divides the work by a constant; a cut to the branching
// factor divides it by (b'/b)^d — exponential in depth. The two sliders let you
// trade a raw speedup against a small per-level prune and watch the prune win by
// orders of magnitude. Numbers are illustrative (b and d chosen to be E2-like),
// not a claim about a specific solver.

const B = 5; // illustrative effective branching factor
const D = 64; // illustrative depth (a quarter of E2's 256, kept legible)

function fmtPow(log10: number): string {
  if (!isFinite(log10)) return "∞";
  if (log10 < 3) return Math.round(10 ** log10).toLocaleString();
  return `10^${Math.round(log10)}`;
}

const T = {
  en: {
    title: "Speed divides; pruning divides exponentially",
    speedLabel: (x: string) => `Raw speedup: ${x}× faster`,
    pruneLabel: (p: number) => `Prune: −${p}% branching factor, every level`,
    baseline: "Baseline work to exhaust the tree",
    withSpeed: "…divided by the speedup",
    withPrune: "…divided by the prune",
    verdict: (ratio: string) =>
      `At this setting the prune does the work of a ${ratio}× speedup.`,
    caption:
      "A speedup is a constant divisor — it buys you a fixed multiple, no matter how deep the search. A prune lowers the branching factor at every one of the ~64 levels here, so its effect compounds: (reduced ⁄ original) to the 64th power. Even a few percent off the branching factor dwarfs a large raw speedup. That is why record solvers win on what they prune, not on clock rate — and why a puzzle engineered to resist pruning is so hard.",
    depthNote: `Illustrative tree: branching factor ${B}, depth ${D}. Eternity II's is far deeper (256), so the gap is far larger still.`,
  },
  fr: {
    title: "La vitesse divise ; l'élagage divise exponentiellement",
    speedLabel: (x: string) => `Accélération brute : ${x}× plus rapide`,
    pruneLabel: (p: number) => `Élagage : −${p}% de facteur de branchement, à chaque niveau`,
    baseline: "Travail de base pour épuiser l'arbre",
    withSpeed: "…divisé par l'accélération",
    withPrune: "…divisé par l'élagage",
    verdict: (ratio: string) =>
      `À ce réglage, l'élagage fait le travail d'une accélération de ${ratio}×.`,
    caption:
      "Une accélération est un diviseur constant — elle offre un multiple fixe, quelle que soit la profondeur. Un élagage abaisse le facteur de branchement à chacun des ~64 niveaux ici, donc son effet se compose : (réduit ⁄ original) à la puissance 64. Même quelques pour cent retirés au facteur de branchement écrasent une grosse accélération brute. C'est pourquoi les meilleurs solveurs gagnent sur ce qu'ils élaguent, pas sur la fréquence — et pourquoi un puzzle conçu pour résister à l'élagage est si dur.",
    depthNote: `Arbre illustratif : facteur de branchement ${B}, profondeur ${D}. Celui d'Eternity II est bien plus profond (256), donc l'écart est encore bien plus grand.`,
  },
  es: {
    title: "La velocidad divide; la poda divide exponencialmente",
    speedLabel: (x: string) => `Aceleración bruta: ${x}× más rápido`,
    pruneLabel: (p: number) => `Poda: −${p}% del factor de ramificación, en cada nivel`,
    baseline: "Trabajo base para agotar el árbol",
    withSpeed: "…dividido por la aceleración",
    withPrune: "…dividido por la poda",
    verdict: (ratio: string) =>
      `Con este ajuste, la poda hace el trabajo de una aceleración de ${ratio}×.`,
    caption:
      "Una aceleración es un divisor constante: aporta un múltiplo fijo, sin importar la profundidad de la búsqueda. Una poda reduce el factor de ramificación en cada uno de los ~64 niveles de aquí, así que su efecto se compone: (reducido ⁄ original) elevado a la 64. Incluso restar unos pocos por ciento al factor de ramificación empequeñece una gran aceleración bruta. Por eso los solucionadores récord ganan por lo que podan, no por la frecuencia de reloj, y por eso un puzzle diseñado para resistir la poda es tan difícil.",
    depthNote: `Árbol ilustrativo: factor de ramificación ${B}, profundidad ${D}. El de Eternity II es mucho más profundo (256), así que la diferencia es aún mayor.`,
  },
};

export function PruneVsSpeedLab() {
  const t = useT(T);
  const [speedExp, setSpeedExp] = useState(3); // 10^x speedup
  const [prunePct, setPrunePct] = useState(5); // % off branching factor

  const speedup = 10 ** speedExp;
  const bPrime = B * (1 - prunePct / 100);

  // log10 of leaves = D * log10(b)
  const baseLog = D * Math.log10(B);
  const speedLog = baseLog - speedExp; // dividing by 10^speedExp
  const pruneLog = D * Math.log10(bPrime);

  // The speedup that would match the prune: 10^(baseLog - pruneLog)
  const equivSpeedupLog = baseLog - pruneLog;

  return (
    <div className="mx-auto max-w-2xl space-y-5 rounded-lg border bg-card p-5">
      <h3 className="text-center text-base font-semibold">{t.title}</h3>

      <div className="space-y-4">
        <div className="space-y-1.5">
          <Label>{t.speedLabel(speedup.toLocaleString())}</Label>
          <Slider
            min={0}
            max={9}
            step={1}
            value={speedExp}
            onValueChange={(v) => setSpeedExp(singleSliderValue(v))}
          />
        </div>
        <div className="space-y-1.5">
          <Label>{t.pruneLabel(prunePct)}</Label>
          <Slider
            min={0}
            max={40}
            step={1}
            value={prunePct}
            onValueChange={(v) => setPrunePct(singleSliderValue(v))}
          />
        </div>
      </div>

      <div className="grid grid-cols-3 gap-3 text-center">
        <div className="rounded-md border p-3">
          <div className="text-[11px] uppercase tracking-wide text-muted-foreground">
            {t.baseline}
          </div>
          <div className="mt-1 text-lg font-bold tabular-nums">{fmtPow(baseLog)}</div>
        </div>
        <div className="rounded-md border p-3">
          <div className="text-[11px] uppercase tracking-wide text-muted-foreground">
            {t.withSpeed}
          </div>
          <div className="mt-1 text-lg font-bold tabular-nums text-amber-500">
            {fmtPow(speedLog)}
          </div>
        </div>
        <div className="rounded-md border p-3">
          <div className="text-[11px] uppercase tracking-wide text-muted-foreground">
            {t.withPrune}
          </div>
          <div className="mt-1 text-lg font-bold tabular-nums text-emerald-500">
            {fmtPow(pruneLog)}
          </div>
        </div>
      </div>

      <p className="rounded-md border border-emerald-300 bg-emerald-500/10 px-3 py-2 text-center text-sm">
        {t.verdict(fmtPow(equivSpeedupLog))}
      </p>

      <p className="text-xs leading-relaxed text-muted-foreground">{t.caption}</p>
      <p className="text-[11px] text-muted-foreground">{t.depthNote}</p>
    </div>
  );
}
