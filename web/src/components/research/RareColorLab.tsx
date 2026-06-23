import { useMemo, useState } from "react";
import { useT } from "@/i18n";
import { useEngine } from "@/engine/useEngine";
import { getOfficialPuzzle } from "@/engine";
import { MotifSwatch } from "@/components/board/MotifSwatch";
import { colorToLetter } from "@/lib/motifs";

// Pick a colour; we scan the real official 256 pieces in the browser and split
// its edges by the kind of piece they sit on — corner, border, or interior.
// Choose one of the five rare colours and the interior bar is empty: those
// colours never appear inside the board, only on the frame. Choose a common
// colour and it is almost all interior. Live, from the official set — the same
// numbers the page's chart reports, here you can poke at them colour by colour.

interface Split {
  corner: number;
  border: number;
  interior: number;
  total: number;
}

// Classify a piece by its grey (border) edge count: 2 = corner, 1 = border,
// 0 = interior — the standard Eternity II split.
function pieceKind(edges: readonly number[]): "corner" | "border" | "interior" {
  const greys = edges.filter((c) => c === 0).length;
  if (greys >= 2) return "corner";
  if (greys === 1) return "border";
  return "interior";
}

function Bar({
  value,
  label,
  fill,
  maxTotal,
  edgesLabel,
}: {
  value: number;
  label: string;
  fill: string;
  maxTotal: number;
  edgesLabel: string;
}) {
  return (
    <div className="flex items-center gap-2">
      <span className="w-24 shrink-0 text-right text-xs text-muted-foreground">{label}</span>
      <div className="relative h-5 flex-1 overflow-hidden rounded bg-muted">
        <div
          className="h-full rounded transition-[width]"
          style={{ width: `${(value / maxTotal) * 100}%`, background: fill }}
        />
      </div>
      <span className="w-16 shrink-0 text-xs tabular-nums">{edgesLabel}</span>
    </div>
  );
}

const T = {
  en: {
    title: "Where does each colour live?",
    intro:
      "Pick a colour. We scan the real official pieces and count its edges by the kind of piece they sit on. The five rare colours light up only the frame — zero in the interior.",
    pick: "Colour",
    corner: "corner pieces",
    border: "border pieces",
    interior: "interior pieces",
    edges: (n: number) => `${n} edge${n === 1 ? "" : "s"}`,
    rare: "rare · frame-only",
    common: "common",
    interiorZero: "Not once in the interior — a rare colour, fenced to the frame.",
    interiorMost: "Overwhelmingly interior — a common colour doing the board's work.",
    loading: "Loading the piece set…",
  },
  fr: {
    title: "Où vit chaque couleur ?",
    intro:
      "Choisissez une couleur. On parcourt les vraies pièces officielles et on compte ses bords selon le type de pièce. Les cinq couleurs rares n'éclairent que le cadre — zéro à l'intérieur.",
    pick: "Couleur",
    corner: "pièces de coin",
    border: "pièces de bord",
    interior: "pièces intérieures",
    edges: (n: number) => `${n} bord${n === 1 ? "" : "s"}`,
    rare: "rare · cadre uniquement",
    common: "commune",
    interiorZero: "Pas une seule fois à l'intérieur — une couleur rare, confinée au cadre.",
    interiorMost: "Très majoritairement intérieure — une couleur commune qui fait le travail du plateau.",
    loading: "Chargement du jeu de pièces…",
  },
};

export function RareColorLab() {
  const t = useT(T);
  const ready = useEngine();
  const [color, setColor] = useState(1);

  // For every colour 1..22, split its edge occurrences by piece kind.
  const splits = useMemo<Split[] | null>(() => {
    if (!ready) return null;
    const puzzle = getOfficialPuzzle();
    const out: Split[] = Array.from({ length: 23 }, () => ({ corner: 0, border: 0, interior: 0, total: 0 }));
    for (const piece of puzzle.pieces) {
      const kind = pieceKind(piece);
      for (const c of piece) {
        if (c === 0) continue;
        const s = out[c];
        if (!s) continue;
        s[kind]++;
        s.total++;
      }
    }
    return out;
  }, [ready]);

  if (!splits) {
    return (
      <section className="mx-auto max-w-3xl rounded-xl border bg-muted/20 p-5">
        <div className="flex h-40 items-center justify-center text-sm text-muted-foreground">{t.loading}</div>
      </section>
    );
  }

  const s = splits[color] ?? { corner: 0, border: 0, interior: 0, total: 0 };
  const isRare = s.interior === 0 && s.total > 0;
  const maxTotal = Math.max(...splits.slice(1).map((x) => x.total), 1);

  return (
    <section className="mx-auto max-w-3xl space-y-4 rounded-xl border bg-muted/20 p-5">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{t.title}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap gap-1.5">
        {Array.from({ length: 22 }, (_, i) => i + 1).map((c) => {
          const rare = (splits[c]?.interior ?? 0) === 0 && (splits[c]?.total ?? 0) > 0;
          return (
            <button
              key={c}
              type="button"
              onClick={() => setColor(c)}
              title={`'${colorToLetter(c)}'`}
              className={`rounded-md border p-0.5 transition-all ${
                c === color ? "ring-2 ring-primary" : "hover:scale-110"
              } ${rare ? "border-amber-500" : "border-transparent"}`}
            >
              <MotifSwatch color={c} width={26} />
            </button>
          );
        })}
      </div>

      <div className="grid gap-4 sm:grid-cols-[140px_1fr] sm:items-center">
        <div className="flex flex-col items-center gap-1.5">
          <MotifSwatch color={color} width={72} />
          <span className="font-mono text-sm font-bold">'{colorToLetter(color)}'</span>
          <span
            className={`rounded-full px-2 py-0.5 text-[10px] font-medium uppercase tracking-wide ${
              isRare
                ? "bg-amber-500/15 text-amber-700 dark:text-amber-300"
                : "bg-muted text-muted-foreground"
            }`}
          >
            {isRare ? t.rare : t.common}
          </span>
        </div>
        <div className="space-y-2">
          <Bar value={s.corner} label={t.corner} fill="#94a3b8" maxTotal={maxTotal} edgesLabel={t.edges(s.corner)} />
          <Bar value={s.border} label={t.border} fill="#f59e0b" maxTotal={maxTotal} edgesLabel={t.edges(s.border)} />
          <Bar value={s.interior} label={t.interior} fill="#38bdf8" maxTotal={maxTotal} edgesLabel={t.edges(s.interior)} />
          <p className="pt-1 text-sm leading-relaxed text-muted-foreground">
            {isRare ? t.interiorZero : t.interiorMost}
          </p>
        </div>
      </div>
    </section>
  );
}
