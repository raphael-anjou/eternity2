import { useState } from "react";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";
import data from "@/data/entropy-area-law.json";

// The area law, made tangible. For each block size n, two exact counts on the
// real E2 interior pieces: how many colour-valid n×n blocks exist if pieces may
// repeat (Variant A, the matching grammar) versus if each piece is used once
// (Variant B, true E2). Their ratio B/A is the scarcity penalty — it starts at 1
// and drops as the block grows, because every added cell is another chance to
// need a piece already spent. That decline, compounding over the AREA of a
// patch, is the wall. Numbers are exact (A(2), B(2) verified in-repo; A(3) from
// the vault's broken-profile DP counter), not a toy simulation — a small
// generated board would not reproduce E2's real scarcity, so we show the real
// counts instead.

const blocks = data.blockCounts as {
  n: number;
  cells: number;
  reusable: number;
  distinct: number | null;
  ratio: number | null;
}[];
const density = data.entropyDensityByN as { n: number; sOverN2: number }[];

const T = {
  en: {
    title: "Watch scarcity bite, block by block",
    intro:
      "Two exact counts on the real Eternity II pieces. Reusable: how many colour-valid n×n blocks exist if pieces may repeat — the richness of the matching rules. Distinct: how many survive the use-each-piece-once rule. Step n up and watch the gap open.",
    sizeLabel: "Block size",
    reusable: "colour-valid blocks (pieces may repeat)",
    distinct: "…that use distinct pieces",
    ratio: "survive the distinct rule",
    densityLabel: "freedom per cell (entropy density)",
    densityHint:
      "Even with reusable pieces, each added cell adds less freedom than the last — the grammar alone tightens as blocks grow.",
    n3note: "The distinct count at 3×3 is past exact enumeration here; the reusable count is 1.96×10¹¹.",
    note: "The distinct-realizable fraction collapses with the AREA of the patch, not its perimeter — and area grows as n². By around eighty cells it has dropped below one in a thousand: the size of the smallest moves between the best known boards. The hardness isn't in matching colours (that grammar stays rich); it's in the quiet once-each rule, whose cost compounds over area.",
  },
  fr: {
    title: "Voir la rareté mordre, bloc par bloc",
    intro:
      "Deux comptages exacts sur les vraies pièces d'Eternity II. Réutilisables : combien de blocs n×n valides en couleurs existent si les pièces peuvent se répéter — la richesse des règles d'accord. Distinctes : combien survivent à la règle « chaque pièce une fois ». Augmentez n et regardez l'écart se creuser.",
    sizeLabel: "Taille de bloc",
    reusable: "blocs valides en couleurs (pièces répétables)",
    distinct: "…utilisant des pièces distinctes",
    ratio: "survivent à la règle de distinction",
    densityLabel: "liberté par cellule (densité d'entropie)",
    densityHint:
      "Même avec des pièces répétables, chaque cellule ajoutée apporte moins de liberté que la précédente — la grammaire seule se resserre quand les blocs grandissent.",
    n3note: "Le compte distinct en 3×3 dépasse l'énumération exacte ici ; le compte réutilisable est 1,96×10¹¹.",
    note: "La fraction réalisable distincte s'effondre avec l'AIRE du patch, pas son périmètre — et l'aire croît en n². Vers quatre-vingts cellules elle est tombée sous un sur mille : la taille des plus petits mouvements entre les meilleurs plateaux connus. La difficulté n'est pas dans l'accord des couleurs (cette grammaire reste riche) ; elle est dans la règle discrète « une fois chacune », dont le coût se compose sur l'aire.",
  },
};

function fmt(x: number): string {
  if (x < 1e4) return Math.round(x).toLocaleString();
  const exp = Math.floor(Math.log10(x));
  const mant = x / 10 ** exp;
  return `${mant.toFixed(2)}×10^${exp}`;
}

export function EntropyScarcityLab() {
  const t = useT(T);
  const [n, setN] = useState(2);
  const b = blocks.find((x) => x.n === n) ?? blocks[1];
  if (!b) return null;
  const dens = density.find((x) => x.n === n)?.sOverN2 ?? null;
  const ratioPct = b.ratio !== null ? Math.round(b.ratio * 100) : null;

  return (
    <div className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex items-center justify-center gap-2">
        <span className="text-xs font-medium text-muted-foreground">{t.sizeLabel}:</span>
        {blocks.map((x) => (
          <button
            key={x.n}
            onClick={() => setN(x.n)}
            className={cn(
              "rounded border px-3 py-1 text-sm font-medium tabular-nums transition-colors",
              x.n === n
                ? "border-primary bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted",
            )}
          >
            {x.n}×{x.n}
          </button>
        ))}
      </div>

      <div className="mx-auto max-w-md space-y-3">
        {/* Reusable count bar (full width = log reference). */}
        <CountRow label={t.reusable} value={b.reusable} valueStr={fmt(b.reusable)} max={b.reusable} tone="grammar" />
        {b.distinct !== null ? (
          <CountRow label={t.distinct} value={b.distinct} valueStr={fmt(b.distinct)} max={b.reusable} tone="distinct" />
        ) : (
          <p className="text-center text-[11px] text-muted-foreground">{t.n3note}</p>
        )}

        {ratioPct !== null && (
          <div className="rounded-md border bg-muted/30 px-3 py-2 text-center">
            <div className="text-2xl font-bold tabular-nums">{ratioPct}%</div>
            <div className="text-[11px] text-muted-foreground">{t.ratio}</div>
          </div>
        )}

        {dens !== null && (
          <div className="flex items-center justify-between rounded-md border px-3 py-2">
            <span className="text-xs text-muted-foreground">{t.densityLabel}</span>
            <span className="text-lg font-bold tabular-nums">{dens.toFixed(2)}</span>
          </div>
        )}
        <p className="text-[11px] text-muted-foreground">{t.densityHint}</p>
      </div>

      <p className="text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}

function CountRow({
  label,
  value,
  valueStr,
  max,
  tone,
}: {
  label: string;
  value: number;
  valueStr: string;
  max: number;
  tone: "grammar" | "distinct";
}) {
  // Bar width by log10 so the two counts are visually comparable across scales;
  // the distinct bar sits just under the reusable one to show the shaved-off gap.
  const pct = max > 1 ? (Math.log10(Math.max(value, 1)) / Math.log10(max)) * 100 : 100;
  return (
    <div className="space-y-0.5">
      <div className="flex items-baseline justify-between text-xs">
        <span className="text-muted-foreground">{label}</span>
        <span className="font-semibold tabular-nums">{valueStr}</span>
      </div>
      <div className="h-2.5 overflow-hidden rounded-sm bg-muted">
        <div
          className={cn("h-full rounded-sm", tone === "grammar" ? "bg-sky-500" : "bg-emerald-500")}
          style={{ width: `${Math.max(2, pct)}%` }}
        />
      </div>
    </div>
  );
}
