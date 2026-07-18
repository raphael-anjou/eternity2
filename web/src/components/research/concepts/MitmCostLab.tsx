import { useEffect, useState } from "react";
import type { ReactNode } from "react";
import { useT } from "@/i18n";
import { useIsClient, cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { Button } from "@/components/ui/button";

// The time-memory trade of meet in the middle, made tangible. Two views:
//
//  "The trade": a problem-size slider n drives three log-scale bars — one-sided
//  time 2^n, MITM time 2·2^(n/2), MITM memory 2^(n/2) entries — plus the tiny
//  O(n) memory of the one-sided walk. The point is to FEEL the square root
//  (every +2 on n quadruples one bar and merely doubles the others) and to
//  watch the memory bill arrive as n grows.
//
//  "The join": a micro-instance (n = 10) animated end to end. The left half
//  enumerates its 2^5 = 32 candidates and stores each in a hash table under
//  its seam signature; the right half's 32 candidates then probe the table.
//  Occupied bucket = solutions found without ever walking the full 2^10 tree.
//
// Everything is deterministic (fixed LCG seed); the animation is gated on
// useRunWhileVisible and does trivial work per tick.

const N_MIN = 8;
const N_MAX = 64;
const BYTES_PER_ENTRY = 16;
const RAM_LIMIT = 16 * 2 ** 30; // 16 GiB, a beefy laptop

// --- the join micro-instance (n = 10: two halves of 2^5 = 32) --------------
const JOIN_N = 10;
const HALF = 32; // 2^(JOIN_N/2)
const BUCKETS = 48;
const PAUSE_TICKS = 16;
const TOTAL_TICKS = HALF + HALF + PAUSE_TICKS;

function lcg(s: number): number {
  return (Math.imul(s, 1664525) + 1013904223) >>> 0;
}

/** Deterministic seam signatures for both halves (module-level, seeded). */
function makeSigs(seed: number, count: number): number[] {
  const out: number[] = [];
  let s = seed;
  for (let i = 0; i < count; i++) {
    s = lcg(s);
    out.push(s % BUCKETS);
  }
  return out;
}
const LEFT_SIG = makeSigs(0x5eed, HALF);
const RIGHT_SIG = makeSigs(0xcafe, HALF);
const FULL_COUNTS: number[] = (() => {
  const c = new Array<number>(BUCKETS).fill(0);
  for (const b of LEFT_SIG) c[b] = (c[b] ?? 0) + 1;
  return c;
})();

const T = {
  en: {
    title: "Feel the square root",
    intro:
      "One knob: the number of binary choices n. Meet in the middle never removes the exponential — it splits it into two square roots, one paid in time, one paid in memory.",
    tabTrade: "The trade",
    tabJoin: "The join",
    n: "problem size n",
    oneTime: "one-sided time",
    mitmTime: "MITM time",
    mitmMem: "MITM memory",
    oneMem: "one-sided memory",
    steps: "steps",
    entries: "entries",
    atSpeed: "at 10⁹ steps/s:",
    oneSidedTakes: "one-sided",
    mitmTakes: "MITM",
    tableCosts: "the table costs",
    ramWall: "→ bigger than a 16 GiB machine",
    ramOk: "(fits in RAM)",
    logNote: "Bars are on a log scale — each grid line is ×256. Linear bars would be useless: at n = 64 the red bar would be 2³² times longer than the blue ones.",
    joinCaption: (
      <>
        A full run of the micro-instance n = {JOIN_N}: two enumerations of 2<sup>5</sup> = {HALF} half-candidates instead of one walk of 2<sup>{JOIN_N}</sup> = 1,024 — but only if one side's table is held in memory.
      </>
    ),
    phaseStore: (i: number) => `Storing top halves: ${i} / ${HALF} — each keyed by its seam signature`,
    phaseProbe: (j: number) => `Probing with bottom halves: ${j} / ${HALF} — one hash lookup each`,
    phaseDone: `Done: ${2 * HALF} enumeration steps + ${HALF} stored entries, instead of 1,024 full walks.`,
    stored: "tops stored",
    memory: "table memory",
    probes: "probes",
    pairs: "solutions joined",
    hit: "hit",
    miss: "empty bucket",
    bucketAxis: `${BUCKETS} seam signatures (hash buckets)`,
    loading: "Loading…",
  },
  fr: {
    title: "Sentir la racine carrée",
    intro:
      "Un seul bouton : le nombre de choix binaires n. Le meet in the middle ne supprime jamais l'exponentielle — il la scinde en deux racines carrées, l'une payée en temps, l'autre en mémoire.",
    tabTrade: "L'échange",
    tabJoin: "La jonction",
    n: "taille du problème n",
    oneTime: "temps unilatéral",
    mitmTime: "temps MITM",
    mitmMem: "mémoire MITM",
    oneMem: "mémoire unilatérale",
    steps: "pas",
    entries: "entrées",
    atSpeed: "à 10⁹ pas/s :",
    oneSidedTakes: "unilatéral",
    mitmTakes: "MITM",
    tableCosts: "la table coûte",
    ramWall: "→ plus gros qu'une machine à 16 Gio",
    ramOk: "(tient en RAM)",
    logNote: "Les barres sont en échelle logarithmique — chaque graduation vaut ×256. En échelle linéaire, à n = 64 la barre rouge serait 2³² fois plus longue que les bleues.",
    joinCaption: (
      <>
        Un tour complet de la micro-instance n = {JOIN_N} : deux énumérations de 2<sup>5</sup> = {HALF} demi-candidats au lieu d'un parcours de 2<sup>{JOIN_N}</sup> = 1 024 — mais seulement si la table d'un des côtés tient en mémoire.
      </>
    ),
    phaseStore: (i: number) => `Stockage des moitiés hautes : ${i} / ${HALF} — chacune indexée par sa signature de couture`,
    phaseProbe: (j: number) => `Sondage avec les moitiés basses : ${j} / ${HALF} — une consultation de table chacune`,
    phaseDone: `Terminé : ${2 * HALF} pas d'énumération + ${HALF} entrées stockées, au lieu de 1 024 parcours complets.`,
    stored: "hauts stockés",
    memory: "mémoire de table",
    probes: "sondages",
    pairs: "solutions jointes",
    hit: "touché",
    miss: "seau vide",
    bucketAxis: `${BUCKETS} signatures de couture (seaux de hachage)`,
    loading: "Chargement…",
  },
  es: {
    title: "Sentir la raíz cuadrada",
    intro:
      "Un único mando: el número de decisiones binarias n. El meet in the middle nunca elimina la exponencial — la parte en dos raíces cuadradas, una pagada en tiempo y otra en memoria.",
    tabTrade: "El intercambio",
    tabJoin: "La unión",
    n: "tamaño del problema n",
    oneTime: "tiempo unilateral",
    mitmTime: "tiempo MITM",
    mitmMem: "memoria MITM",
    oneMem: "memoria unilateral",
    steps: "pasos",
    entries: "entradas",
    atSpeed: "a 10⁹ pasos/s:",
    oneSidedTakes: "unilateral",
    mitmTakes: "MITM",
    tableCosts: "la tabla cuesta",
    ramWall: "→ más grande que una máquina de 16 GiB",
    ramOk: "(cabe en RAM)",
    logNote: "Las barras están en escala logarítmica — cada línea de la cuadrícula vale ×256. En escala lineal serían inservibles: con n = 64 la barra roja sería 2³² veces más larga que las azules.",
    joinCaption: (
      <>
        Una ejecución completa de la micro-instancia n = {JOIN_N}: dos enumeraciones de 2<sup>5</sup> = {HALF} medios candidatos en lugar de un único recorrido de 2<sup>{JOIN_N}</sup> = 1024 — pero solo si la tabla de uno de los lados se mantiene en memoria.
      </>
    ),
    phaseStore: (i: number) => `Almacenando las mitades superiores: ${i} / ${HALF} — cada una indexada por su firma de costura`,
    phaseProbe: (j: number) => `Sondeando con las mitades inferiores: ${j} / ${HALF} — una consulta a la tabla cada una`,
    phaseDone: `Listo: ${2 * HALF} pasos de enumeración + ${HALF} entradas almacenadas, en lugar de 1024 recorridos completos.`,
    stored: "superiores almacenadas",
    memory: "memoria de tabla",
    probes: "sondeos",
    pairs: "soluciones unidas",
    hit: "acierto",
    miss: "cubeta vacía",
    bucketAxis: `${BUCKETS} firmas de costura (cubetas hash)`,
    loading: "Cargando…",
  },
};

/** "1.8×10¹⁹" as JSX (plain number below 10 000). */
function Approx({ x }: { x: number }): ReactNode {
  if (x < 1e4) return <>{Math.round(x)}</>;
  const e = Math.floor(Math.log10(x));
  const m = x / 10 ** e;
  return (
    <>
      {m.toFixed(1)}×10<sup>{e}</sup>
    </>
  );
}

function fmtSeconds(s: number): string {
  if (s < 1e-6) return `${Math.max(1, Math.round(s * 1e9))} ns`;
  if (s < 1e-3) return `${(s * 1e6).toFixed(1)} µs`;
  if (s < 1) return `${(s * 1e3).toFixed(1)} ms`;
  if (s < 120) return `${s.toFixed(1)} s`;
  const min = s / 60;
  if (min < 120) return `${min.toFixed(0)} min`;
  const h = min / 60;
  if (h < 48) return `${h.toFixed(0)} h`;
  const days = h / 24;
  if (days < 730) return `${days.toFixed(0)} d`;
  const years = days / 365.25;
  if (years < 1e4) return `${years.toFixed(0)} y`;
  return `${(years / 1e6).toFixed(1)} My`;
}

function fmtBytes(b: number): string {
  const units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
  let v = b;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i++;
  }
  return `${v >= 100 ? v.toFixed(0) : v.toFixed(1)} ${units[i] ?? "B"}`;
}

function CostBar({
  label,
  value,
  unit,
  colorClass,
  detail,
}: {
  label: string;
  value: number;
  unit: string;
  colorClass: string;
  detail?: ReactNode;
}) {
  // Log2 scale against the largest value the slider can produce (2^N_MAX).
  const pct = Math.max(1.2, (Math.log2(Math.max(value, 2)) / N_MAX) * 100);
  return (
    <div className="space-y-0.5">
      <div className="flex items-baseline justify-between gap-2 text-xs">
        <span className="font-medium">{label}</span>
        <span className="tabular-nums text-muted-foreground">
          <Approx x={value} /> {unit}
          {detail ? <> — {detail}</> : null}
        </span>
      </div>
      <div className="relative h-3.5 overflow-hidden rounded-sm bg-muted">
        {/* log grid: one line per 8 doublings (×256) */}
        {Array.from({ length: Math.floor(N_MAX / 8) }, (_, i) => (
          <div
            key={i}
            className="absolute inset-y-0 w-px bg-border"
            style={{ left: `${((i + 1) * 8 * 100) / N_MAX}%` }}
          />
        ))}
        <div
          className={cn("h-full rounded-sm transition-all duration-200", colorClass)}
          style={{ width: `${pct}%` }}
        />
      </div>
    </div>
  );
}

function TradeView({ t }: { t: (typeof T)["en"] }) {
  const [n, setN] = useState(40);
  const half = n / 2;
  const oneTime = 2 ** n;
  const mitmTime = 2 * 2 ** half;
  const mitmEntries = 2 ** half;
  const mitmBytes = mitmEntries * BYTES_PER_ENTRY;
  const overRam = mitmBytes > RAM_LIMIT;

  return (
    <div className="space-y-4">
      <label className="flex items-center gap-3 text-sm">
        <span className="whitespace-nowrap text-muted-foreground">{t.n}</span>
        <input
          type="range"
          min={N_MIN}
          max={N_MAX}
          step={2}
          value={n}
          onChange={(e) => setN(Number(e.target.value))}
          className="w-full max-w-64"
        />
        <span className="w-8 text-right font-semibold tabular-nums">{n}</span>
      </label>

      <div className="space-y-2.5">
        <CostBar
          label={`${t.oneTime} — 2^${n}`}
          value={oneTime}
          unit={t.steps}
          colorClass="bg-rose-500"
        />
        <CostBar
          label={`${t.mitmTime} — 2·2^${half}`}
          value={mitmTime}
          unit={t.steps}
          colorClass="bg-sky-500"
        />
        <CostBar
          label={`${t.mitmMem} — 2^${half}`}
          value={mitmEntries}
          unit={t.entries}
          colorClass={overRam ? "bg-rose-500" : "bg-amber-500"}
          detail={
            <span className={overRam ? "font-semibold text-rose-600 dark:text-rose-400" : undefined}>
              {fmtBytes(mitmBytes)} {overRam ? t.ramWall : t.ramOk}
            </span>
          }
        />
        <CostBar label={`${t.oneMem} — n`} value={n} unit={t.entries} colorClass="bg-stone-400" />
      </div>

      <p className="text-sm tabular-nums text-muted-foreground">
        {t.atSpeed} {t.oneSidedTakes} <span className="font-semibold text-foreground">{fmtSeconds(oneTime / 1e9)}</span> · {t.mitmTakes}{" "}
        <span className="font-semibold text-foreground">{fmtSeconds(mitmTime / 1e9)}</span> · {t.tableCosts}{" "}
        <span className={cn("font-semibold", overRam ? "text-rose-600 dark:text-rose-400" : "text-foreground")}>
          {fmtBytes(mitmBytes)}
        </span>
      </p>
      <p className="text-xs text-muted-foreground">{t.logNote}</p>
    </div>
  );
}

function JoinView({ t, visible }: { t: (typeof T)["en"]; visible: boolean }) {
  const [step, setStep] = useState(0);

  useEffect(() => {
    if (!visible) return;
    const id = setInterval(() => setStep((s) => (s + 1) % (TOTAL_TICKS + 1)), 140);
    return () => clearInterval(id);
  }, [visible]);

  const inserted = Math.min(step, HALF);
  const probed = Math.min(Math.max(step - HALF, 0), HALF);
  const done = step >= HALF + HALF;

  // Table contents after `inserted` insertions.
  const counts = new Array<number>(BUCKETS).fill(0);
  for (let i = 0; i < inserted; i++) {
    const b = LEFT_SIG[i] ?? 0;
    counts[b] = (counts[b] ?? 0) + 1;
  }
  // Join tally over the first `probed` right-hand candidates (table is full
  // by the time probing starts, exactly like the real phase split).
  let pairs = 0;
  for (let j = 0; j < probed; j++) pairs += FULL_COUNTS[RIGHT_SIG[j] ?? 0] ?? 0;
  const lastProbe = probed > 0 ? (RIGHT_SIG[probed - 1] ?? 0) : null;
  const lastHit = lastProbe !== null && (FULL_COUNTS[lastProbe] ?? 0) > 0;

  const BAR_W = 7;
  const GAP = 3;
  const W = BUCKETS * (BAR_W + GAP);
  const BASE = 96;

  return (
    <div className="space-y-3">
      <svg viewBox={`0 0 ${W} 128`} className="w-full rounded-md border bg-muted/20" role="img" aria-label="Histogram of stored seam signatures across hash buckets, with the currently probed bucket outlined green on a hit or rose on a miss">
        {Array.from({ length: BUCKETS }, (_, b) => {
          const c = counts[b] ?? 0;
          const h = c * 22;
          const x = b * (BAR_W + GAP) + GAP / 2;
          const probedHere = lastProbe === b;
          return (
            <g key={b}>
              <rect x={x} y={BASE - 88} width={BAR_W} height={88} className="fill-muted" rx={1} />
              {c > 0 && (
                <rect
                  x={x}
                  y={BASE - h}
                  width={BAR_W}
                  height={h}
                  rx={1}
                  className="fill-sky-500"
                  opacity={0.9}
                />
              )}
              {probedHere && (
                <>
                  <rect
                    x={x - 1}
                    y={BASE - 90}
                    width={BAR_W + 2}
                    height={92}
                    fill="none"
                    strokeWidth={1.5}
                    stroke={lastHit ? "#10b981" : "#f43f5e"}
                    rx={2}
                  />
                  <circle cx={x + BAR_W / 2} cy={BASE + 10} r={4} fill={lastHit ? "#10b981" : "#f43f5e"} />
                </>
              )}
            </g>
          );
        })}
        <line x1={0} y1={BASE} x2={W} y2={BASE} className="stroke-border" strokeWidth={1} />
        <text x={W / 2} y={124} textAnchor="middle" fontSize={9} className="fill-muted-foreground">
          {t.bucketAxis}
        </text>
      </svg>

      <p
        className={cn(
          "text-center text-sm font-medium",
          done ? "text-emerald-700 dark:text-emerald-400" : "text-muted-foreground",
        )}
      >
        {done ? t.phaseDone : probed > 0 ? t.phaseProbe(probed) : t.phaseStore(inserted)}
      </p>

      <div className="grid grid-cols-2 gap-2 text-center sm:grid-cols-4">
        <Stat label={t.stored} value={`${inserted}`} />
        <Stat label={t.memory} value={fmtBytes(inserted * BYTES_PER_ENTRY)} />
        <Stat label={t.probes} value={`${probed}`} />
        <Stat label={t.pairs} value={`${pairs}`} accent={pairs > 0} />
      </div>

      <p className="text-xs text-muted-foreground">{t.joinCaption}</p>
    </div>
  );
}

function Stat({ label, value, accent }: { label: string; value: string; accent?: boolean }) {
  return (
    <div className="rounded-md border px-2 py-1.5">
      <div
        className={cn(
          "text-lg font-bold tabular-nums",
          accent ? "text-emerald-600 dark:text-emerald-400" : "",
        )}
      >
        {value}
      </div>
      <div className="text-[10px] text-muted-foreground">{label}</div>
    </div>
  );
}

export function MitmCostLab() {
  const t = useT(T);
  const isClient = useIsClient();
  const [view, setView] = useState<"trade" | "join">("trade");
  const { ref: rootRef, visible } = useRunWhileVisible();

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.loading}
      </div>
    );
  }

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>
      <div className="flex gap-2">
        <Button
          size="sm"
          variant={view === "trade" ? "default" : "outline"}
          onClick={() => setView("trade")}
        >
          {t.tabTrade}
        </Button>
        <Button
          size="sm"
          variant={view === "join" ? "default" : "outline"}
          onClick={() => setView("join")}
        >
          {t.tabJoin}
        </Button>
      </div>
      {view === "trade" ? <TradeView t={t} /> : <JoinView t={t} visible={visible} />}
    </div>
  );
}
