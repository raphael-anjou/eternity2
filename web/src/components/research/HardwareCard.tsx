import { useLang, pick } from "@/i18n";
import { cn } from "@/lib/utils";
import { coreHours, formatCoreHours } from "@/lib/research/hardware-cost";
import type { AcceleratorKind, HardwareInfo } from "@/lib/research/types";

// The hardware an experiment ran on, as a compact spec card. It answers three
// questions at a glance: was this a standardized cross-comparable bench or a
// native run (the badge); how much compute did it actually cost (core-hours,
// the headline number); and on exactly what kit (the spec grid). The
// core-hours figure is what lets a 1-core / 60 s bench and a 400-core cluster
// run sit honestly in the same table — raw score alone hides the effort behind
// it. core-hours is DERIVED here (cores × wall-clock hours), never authored.

const T = {
  en: {
    title: "Hardware & run",
    standardized: "Standardized bench",
    standardizedTitle:
      "one logical core, fixed time budget — a cross-comparable baseline",
    native: "Native run",
    nativeTitle:
      "run on its own hardware and budget — recorded for provenance, not directly comparable",
    coreHours: "core-hours",
    coreHoursTitle: "cores × wall-clock hours — the compute this run actually cost",
    cores: "Cores",
    nodes: "Nodes",
    threads: "Threads",
    ram: "RAM",
    gpus: "GPUs",
    cpu: "CPU",
    gpu: "GPU",
    machine: "Machine",
    wallClock: "Budget",
    runs: "Runs",
    seed: "Start",
    accel: {
      none: "CPU only",
      gpu: "GPU-accelerated",
      quantum: "Quantum",
      fpga: "FPGA",
      tpu: "TPU",
    } satisfies Record<AcceleratorKind, string>,
  },
  fr: {
    title: "Matériel & exécution",
    standardized: "Banc standardisé",
    standardizedTitle:
      "un cœur logique, budget de temps fixe — une base comparable entre toutes",
    native: "Exécution native",
    nativeTitle:
      "exécuté sur son propre matériel et budget — noté pour la traçabilité, pas directement comparable",
    coreHours: "cœurs·heure",
    coreHoursTitle:
      "cœurs × heures d'horloge — le calcul que cette exécution a réellement coûté",
    cores: "Cœurs",
    nodes: "Nœuds",
    threads: "Fils",
    ram: "RAM",
    gpus: "GPU",
    cpu: "CPU",
    gpu: "GPU",
    machine: "Machine",
    wallClock: "Budget",
    runs: "Exécutions",
    seed: "Départ",
    accel: {
      none: "CPU seul",
      gpu: "Accéléré GPU",
      quantum: "Quantique",
      fpga: "FPGA",
      tpu: "TPU",
    } satisfies Record<AcceleratorKind, string>,
  },
  es: {
    title: "Hardware y ejecución",
    standardized: "Banco estandarizado",
    standardizedTitle:
      "un núcleo lógico, presupuesto de tiempo fijo — una base comparable entre todas",
    native: "Ejecución nativa",
    nativeTitle:
      "ejecutado en su propio hardware y presupuesto — registrado para trazabilidad, no directamente comparable",
    coreHours: "núcleos·hora",
    coreHoursTitle:
      "núcleos × horas de reloj — el cómputo que esta ejecución realmente costó",
    cores: "Núcleos",
    nodes: "Nodos",
    threads: "Hilos",
    ram: "RAM",
    gpus: "GPU",
    cpu: "CPU",
    gpu: "GPU",
    machine: "Máquina",
    wallClock: "Presupuesto",
    runs: "Ejecuciones",
    seed: "Inicio",
    accel: {
      none: "Solo CPU",
      gpu: "Acelerado por GPU",
      quantum: "Cuántico",
      fpga: "FPGA",
      tpu: "TPU",
    } satisfies Record<AcceleratorKind, string>,
  },
};

const ACCEL_STYLE: Record<AcceleratorKind, string> = {
  none: "border-stone-500/40 bg-stone-500/10 text-stone-600 dark:text-stone-300",
  gpu: "border-emerald-500/40 bg-emerald-500/10 text-emerald-700 dark:text-emerald-300",
  quantum: "border-violet-500/40 bg-violet-500/10 text-violet-700 dark:text-violet-300",
  fpga: "border-amber-500/40 bg-amber-500/10 text-amber-700 dark:text-amber-300",
  tpu: "border-sky-500/40 bg-sky-500/10 text-sky-700 dark:text-sky-300",
};

function Spec({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex flex-col gap-0.5">
      <dt className="text-[0.7rem] uppercase tracking-wide text-muted-foreground">{label}</dt>
      <dd className="font-medium tabular-nums">{value}</dd>
    </div>
  );
}

export function HardwareCard({ hardware }: { hardware: HardwareInfo }) {
  const { lang } = useLang();
  const t = pick(T, lang);
  const hw = hardware;
  const accel = hw.accelerator ?? (hw.gpus && hw.gpus > 0 ? "gpu" : "none");
  const ch = coreHours(hw);
  const coresLabel =
    hw.nodes && hw.nodes > 1 ? `${hw.cores} (${hw.nodes} × nodes)` : `${hw.cores}`;

  const specs: { label: string; value: string }[] = [
    { label: t.cores, value: coresLabel },
    ...(hw.threads !== undefined ? [{ label: t.threads, value: `${hw.threads}` }] : []),
    ...(hw.ramGb !== undefined ? [{ label: t.ram, value: `${hw.ramGb} GiB` }] : []),
    ...(hw.gpus !== undefined ? [{ label: t.gpus, value: `${hw.gpus}` }] : []),
    ...(hw.cpu ? [{ label: t.cpu, value: hw.cpu }] : []),
    ...(hw.gpu && hw.gpu !== "—" ? [{ label: t.gpu, value: hw.gpu }] : []),
    ...(hw.machine ? [{ label: t.machine, value: hw.machine }] : []),
    ...(hw.wallClock ? [{ label: t.wallClock, value: hw.wallClock }] : []),
    ...(hw.runs !== undefined ? [{ label: t.runs, value: `${hw.runs}` }] : []),
    ...(hw.seedPolicy ? [{ label: t.seed, value: hw.seedPolicy }] : []),
  ];

  return (
    <div className="my-8 rounded-lg border p-4 text-sm">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <div className="font-semibold">{t.title}</div>
        <div className="flex flex-wrap items-center gap-2 text-xs">
          <span
            className={cn(
              "rounded-full border px-2 py-0.5 font-medium",
              hw.measured
                ? "border-emerald-500/40 bg-emerald-500/10 text-emerald-700 dark:text-emerald-300"
                : "border-stone-500/40 bg-stone-500/10 text-stone-600 dark:text-stone-300",
            )}
            title={hw.measured ? t.standardizedTitle : t.nativeTitle}
          >
            {hw.measured ? t.standardized : t.native}
          </span>
          <span
            className={cn("rounded-full border px-2 py-0.5 font-medium", ACCEL_STYLE[accel])}
          >
            {t.accel[accel]}
          </span>
        </div>
      </div>

      {ch != null && (
        <div className="mt-3 flex items-baseline gap-2" title={t.coreHoursTitle}>
          <span className="text-2xl font-semibold tabular-nums">{formatCoreHours(ch)}</span>
          <span className="text-xs text-muted-foreground">{t.coreHours}</span>
        </div>
      )}

      <dl className="mt-3 grid grid-cols-2 gap-x-4 gap-y-2.5 sm:grid-cols-3">
        {specs.map((s) => (
          <Spec key={s.label} label={s.label} value={s.value} />
        ))}
      </dl>
    </div>
  );
}
