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
        (Real solvers don't try <em>everything</em>: backtracking abandons doomed branches
        early, which is why small boards solve instantly. But the wall moves only a little:
        even the smartest known search dies long before 16×16.)
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
    measuring: "Mesure de votre machine…",
    measured: (rate: string) => (
      <>
        Votre navigateur vient de mesurer sa propre vitesse :{" "}
        <strong className="text-foreground">{rate} nœuds par seconde</strong>.
        À ce rythme, essayer tous les arrangements un par un (après les déductions
        coins/bords) prendrait :
      </>
    ),
    note: (
      <>
        (Les vrais solveurs n'essaient pas <em>tout</em> : le retour en arrière abandonne
        très tôt les branches condamnées, et c'est pourquoi les petits plateaux sont résolus
        instantanément. Mais le mur recule à peine : même la recherche la plus maligne connue
        s'effondre bien avant le 16×16.)
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
  if (logYears < 6) return u.years(formatCompact(10 ** logYears));
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
