// "How long would YOUR computer take?": measures this browser's real
// fit-check rate, then converts the brute-force search-space sizes into
// wall-clock time. All math in log10 space (the numbers overflow doubles).

import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getOfficialPuzzle, getPath } from "@/engine";
import { formatCompact } from "@/lib/format";
import { useT } from "@/i18n";
import stats from "@/data/difficulty.json";

const SECONDS_PER_YEAR = 3.156e7;
const LOG10_UNIVERSE_AGE_YEARS = Math.log10(1.38e10);

const T = {
  en: {
    title: (
      <>
        How long would <em>your</em> computer take to try everything?
      </>
    ),
    measuring: "Measuring your machine…",
    measured: (rate: string) => (
      <>
        Your browser just measured itself at{" "}
        <strong className="text-foreground">{rate} nodes per second</strong>. At that pace,
        trying every arrangement one by one (after the corner/border deductions) would take:
      </>
    ),
    note: (
      <>
        (This is the <em>brute-force</em> ceiling — the worst case. Real solvers do far better:
        backtracking abandons doomed branches early, and the best Eternity II searches add
        smart pruning and constraint propagation that cut the work by many orders of magnitude.
        Faster hardware helps too. Even so, the gap is so vast that all of it together still
        falls short of 16×16 — which is exactly why the puzzle remains unsolved.)
      </>
    ),
    units: {
      blink: "faster than you can blink",
      milliseconds: (n: number) => `${n} milliseconds`,
      seconds: (s: string) => `${s} seconds`,
      minutes: (n: number) => `${n} minutes`,
      hours: (n: number) => `${n} hours`,
      days: (n: number) => `${n} days`,
      years: (s: string) => `${s} years`,
      universeAges: (s: string) => `${s}× the age of the universe`,
      universeAgesExp: (exp: number) => `10^${exp} ages of the universe`,
    },
  },
  fr: {
    title: (
      <>
        Combien de temps faudrait-il à <em>votre</em> ordinateur pour tout essayer ?
      </>
    ),
    measuring: "Test de votre machine en cours…",
    measured: (rate: string) => (
      <>
        Votre navigateur vient de jauger sa propre vitesse :{" "}
        <strong className="text-foreground">{rate} nœuds par seconde</strong>.
        À ce rythme, passer en revue toutes les dispositions une à une (une fois les coins
        et les bords déduits) prendrait :
      </>
    ),
    note: (
      <>
        (C'est le plafond de la <em>force brute</em>, le pire des cas. Les vrais solveurs font
        bien mieux : le retour en arrière (backtracking) abandonne au plus tôt les branches sans
        issue, et les meilleures recherches sur Eternity II y ajoutent un élagage malin et de la
        propagation de contraintes qui divisent le travail par plusieurs ordres de grandeur. Le
        matériel plus rapide aide aussi. Et pourtant l'écart est si vertigineux que tout cela
        réuni reste insuffisant pour le 16×16 — c'est précisément pour ça que le puzzle n'est
        toujours pas résolu.)
      </>
    ),
    units: {
      blink: "plus vite qu'un clignement d'œil",
      milliseconds: (n: number) => `${n} millisecondes`,
      seconds: (s: string) => `${s} secondes`,
      minutes: (n: number) => `${n} minutes`,
      hours: (n: number) => `${n} heures`,
      days: (n: number) => `${n} jours`,
      years: (s: string) => `${s} ans`,
      universeAges: (s: string) => `${s} fois l'âge de l'Univers`,
      universeAgesExp: (exp: number) => `10^${exp} fois l'âge de l'Univers`,
    },
  },
  es: {
    title: (
      <>
        ¿Cuánto tardaría <em>tu</em> ordenador en probarlo todo?
      </>
    ),
    measuring: "Midiendo tu máquina…",
    measured: (rate: string) => (
      <>
        Tu navegador acaba de medir su propia velocidad:{" "}
        <strong className="text-foreground">{rate} nodos por segundo</strong>. A ese ritmo,
        recorrer todas las disposiciones una por una (tras deducir las esquinas y los bordes)
        llevaría:
      </>
    ),
    note: (
      <>
        (Este es el techo de la <em>fuerza bruta</em>, el peor de los casos. Los solucionadores
        reales rinden mucho mejor: el backtracking abandona cuanto antes las ramas sin salida, y
        las mejores búsquedas sobre Eternity II añaden una poda inteligente y propagación de
        restricciones que reducen el trabajo en muchos órdenes de magnitud. Un hardware más rápido
        también ayuda. Aun así, la brecha es tan enorme que todo ello junto sigue quedándose corto
        para el 16×16, y esa es precisamente la razón de que el puzzle siga sin resolverse.)
      </>
    ),
    units: {
      blink: "más rápido de lo que puedes parpadear",
      milliseconds: (n: number) => `${n} milisegundos`,
      seconds: (s: string) => `${s} segundos`,
      minutes: (n: number) => `${n} minutos`,
      hours: (n: number) => `${n} horas`,
      days: (n: number) => `${n} días`,
      years: (s: string) => `${s} años`,
      universeAges: (s: string) => `${s} veces la edad del Universo`,
      universeAgesExp: (exp: number) => `10^${exp} veces la edad del Universo`,
    },
  },
};

// Same math as before, with the unit phrases supplied by the active language.
function describeLogSeconds(logSec: number, u: typeof T.en.units): string {
  if (logSec < -3) return u.blink;
  if (logSec < 0) return u.milliseconds(Math.round(10 ** (logSec + 3)));
  const sec = logSec < 7 ? 10 ** logSec : Infinity;
  if (sec < 60) return u.seconds(sec.toFixed(sec < 10 ? 1 : 0));
  if (sec < 3600) return u.minutes(Math.round(sec / 60));
  if (sec < 86400) return u.hours(Math.round(sec / 3600));
  if (sec < 3.156e7) return u.days(Math.round(sec / 86400));
  const logYears = logSec - Math.log10(SECONDS_PER_YEAR);
  // Plain years all the way up to one age of the universe, so the next unit
  // always starts at >= 1x (never "0x the age of the universe").
  if (logYears < LOG10_UNIVERSE_AGE_YEARS) return u.years(formatCompact(10 ** logYears));
  if (logYears < LOG10_UNIVERSE_AGE_YEARS + 6) {
    const ages = 10 ** (logYears - LOG10_UNIVERSE_AGE_YEARS);
    return u.universeAges(formatCompact(ages));
  }
  const agesExp = Math.round(logYears - LOG10_UNIVERSE_AGE_YEARS);
  return u.universeAgesExp(agesExp);
}

interface SpaceRow {
  size: number;
  log10Naive: number;
  log10Refined: number;
}

const ROWS = (stats.searchSpace as SpaceRow[]).filter((r) =>
  [3, 4, 5, 6, 8, 10, 12, 16].includes(r.size),
);

export function TimeToSolve() {
  const engineReady = useEngine();
  const t = useT(T);
  const [rate, setRate] = useState<number | null>(null);

  useEffect(() => {
    if (!engineReady || rate !== null) return;
    // Measure real fit-check throughput on the official puzzle for ~150ms.
    const puzzle = getOfficialPuzzle();
    const solver = createSolver(puzzle, getPath("row-major", 16, 16, 0), { useHints: true });
    const t0 = performance.now();
    let r = solver.report();
    while (performance.now() - t0 < 150) r = solver.step(20_000);
    const elapsed = (performance.now() - t0) / 1000;
    solver.free();
    // Terminal result of a one-shot WASM benchmark — the whole point of the
    // effect, not a cascading render.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setRate(r.nodes / elapsed);
  }, [engineReady, rate]);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">{t.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        {rate === null ? (
          <p className="text-sm text-muted-foreground">{t.measuring}</p>
        ) : (
          <>
            <p className="text-sm text-muted-foreground">{t.measured(formatCompact(rate))}</p>
            <div className="grid gap-1.5">
              {ROWS.map((row) => {
                const logSec = row.log10Refined - Math.log10(rate);
                const label = describeLogSeconds(logSec, t.units);
                const dire = logSec > 9; // beyond ~30 years
                return (
                  <div
                    key={row.size}
                    className="flex items-baseline justify-between rounded-md border px-3 py-1.5 text-sm"
                  >
                    <span className="font-mono font-semibold">
                      {row.size}×{row.size}
                    </span>
                    <span className={dire ? "font-semibold text-red-500" : "text-emerald-600"}>
                      {label}
                    </span>
                  </div>
                );
              })}
            </div>
            <p className="text-xs text-muted-foreground">{t.note}</p>
          </>
        )}
      </CardContent>
    </Card>
  );
}
