import { useState } from "react";
import { MotifSwatch } from "@/components/board/MotifSwatch";
import { useT } from "@/i18n";
import { motifFor } from "@/lib/motifs";
import data from "@/data/rare-color-geography.json";

const T = {
  en: {
    pickPrompt: "Rare colours (border-only) — pick one:",
    rareLabel: (c: number) => `Rare colour ${c}`,
    vsCommon: "vs a common one:",
    commonLabel: (c: number) => `Common colour ${c}`,
    boardLabel: (c: number, frame: number, interior: number) =>
      `Board map: where colour ${c} lives — ${frame} frame edges, ${interior} interior`,
    colour: (c: number) => `Colour ${c}`,
    rare: "rare",
    common: "common",
    onFrame: "on the frame ring",
    inInterior: "in the interior",
    totalEdges: "total edges",
    rareNote:
      "Every one of this colour's edges sits on the frame ring. Not a single one appears in the 14×14 interior — that empty middle is the whole finding.",
    commonNote:
      "A common colour fills the interior and barely touches the frame — the mirror image of the rare five.",
  },
  fr: {
    pickPrompt: "Couleurs rares (bordure uniquement) — choisissez-en une :",
    rareLabel: (c: number) => `Couleur rare ${c}`,
    vsCommon: "contre une commune :",
    commonLabel: (c: number) => `Couleur commune ${c}`,
    boardLabel: (c: number, frame: number, interior: number) =>
      `Carte du plateau : où vit la couleur ${c} — ${frame} arêtes de cadre, ${interior} à l'intérieur`,
    colour: (c: number) => `Couleur ${c}`,
    rare: "rare",
    common: "commune",
    onFrame: "sur l'anneau du cadre",
    inInterior: "à l'intérieur",
    totalEdges: "arêtes au total",
    rareNote:
      "Chaque arête de cette couleur se trouve sur l'anneau du cadre. Pas une seule n'apparaît dans l'intérieur 14×14 — ce centre vide, c'est tout le résultat.",
    commonNote:
      "Une couleur commune remplit l'intérieur et ne touche presque pas le cadre — l'image inverse des cinq couleurs rares.",
  },
  es: {
    pickPrompt: "Colores raros (solo en el borde) — elige uno:",
    rareLabel: (c: number) => `Color raro ${c}`,
    vsCommon: "frente a uno común:",
    commonLabel: (c: number) => `Color común ${c}`,
    boardLabel: (c: number, frame: number, interior: number) =>
      `Mapa del tablero: dónde vive el color ${c} — ${frame} aristas del marco, ${interior} en el interior`,
    colour: (c: number) => `Color ${c}`,
    rare: "raro",
    common: "común",
    onFrame: "en el anillo del marco",
    inInterior: "en el interior",
    totalEdges: "aristas en total",
    rareNote:
      "Cada arista de este color se encuentra en el anillo del marco. Ni una sola aparece en el interior de 14×14 — ese centro vacío es todo el hallazgo.",
    commonNote:
      "Un color común llena el interior y apenas toca el marco — la imagen inversa de los cinco colores raros.",
  },
};

// The spatial companion to the rare-colour bar chart: a 16×16 board where the
// frame ring is drawn apart from the interior, so the finding — the five rare
// colours live *only* on the ring, never inside — is visible as a shape, not
// just a number. Pick a rare colour and its 24 edges light up around the frame
// while the 14×14 interior stays blank; pick a common colour and the interior
// fills instead. The counts come from the same committed data the chart uses.

interface ColorRow {
  color: number;
  total: number;
  frame: number;
  interior: number;
  rare: boolean;
}

const COLORS = data.colors as ColorRow[];
const RARE = COLORS.filter((c) => c.rare).map((c) => c.color);

const GRID = 16;
const CELL = 20;
const GAP = 1.5;
const SIZE = GRID * CELL;

/** Frame cells are the outer ring (row/col 0 or 15). */
function isFrame(r: number, c: number): boolean {
  return r === 0 || r === GRID - 1 || c === 0 || c === GRID - 1;
}

export function RareColorRing() {
  const t = useT(T);
  // Default to the first rare colour so the finding shows on first paint.
  const [selected, setSelected] = useState<number>(RARE[0] ?? 1);
  const row = COLORS.find((c) => c.color === selected);
  const motif = motifFor(selected);
  const isRare = row?.rare ?? false;

  return (
    <div className="space-y-4">
      {/* colour picker */}
      <div className="space-y-1.5">
        <div className="text-xs font-medium text-muted-foreground">{t.pickPrompt}</div>
        <div className="flex flex-wrap gap-1.5">
          {RARE.map((c) => (
            <button
              key={c}
              onClick={() => setSelected(c)}
              className={`rounded-md border p-0.5 transition-shadow ${
                selected === c ? "ring-2 ring-foreground" : "hover:shadow"
              }`}
              aria-pressed={selected === c}
              aria-label={t.rareLabel(c)}
            >
              <MotifSwatch color={c} width={26} />
            </button>
          ))}
          <span className="mx-1 self-center text-xs text-muted-foreground">{t.vsCommon}</span>
          {COLORS.filter((c) => !c.rare)
            .slice(0, 4)
            .map((c) => (
              <button
                key={c.color}
                onClick={() => setSelected(c.color)}
                className={`rounded-md border p-0.5 opacity-70 transition-shadow ${
                  selected === c.color ? "opacity-100 ring-2 ring-foreground" : "hover:shadow"
                }`}
                aria-pressed={selected === c.color}
                aria-label={t.commonLabel(c.color)}
              >
                <MotifSwatch color={c.color} width={26} />
              </button>
            ))}
        </div>
      </div>

      <div className="flex flex-col items-center gap-4 sm:flex-row sm:items-start sm:justify-center">
        {/* the board */}
        <svg
          viewBox={`0 0 ${SIZE} ${SIZE}`}
          className="w-full max-w-[280px] rounded-lg border bg-card"
          role="img"
          aria-label={t.boardLabel(selected, row?.frame ?? 0, row?.interior ?? 0)}
        >
          {Array.from({ length: GRID }, (_, r) =>
            Array.from({ length: GRID }, (_, c) => {
              const frame = isFrame(r, c);
              // Cell is "lit" if the selected colour appears in that region:
              // rare colours only light the frame; common ones only the interior.
              const lit = isRare ? frame : !frame;
              return (
                <rect
                  key={`${r}-${c}`}
                  x={c * CELL + GAP}
                  y={r * CELL + GAP}
                  width={CELL - GAP * 2}
                  height={CELL - GAP * 2}
                  rx={2}
                  fill={lit ? motif.bg : "transparent"}
                  className={lit ? "" : "stroke-muted"}
                  strokeWidth={lit ? 0 : 0.75}
                  opacity={lit ? 0.92 : 1}
                />
              );
            }),
          )}
          {/* ring outline to name the frame */}
          <rect
            x={GAP}
            y={GAP}
            width={SIZE - GAP * 2}
            height={SIZE - GAP * 2}
            rx={3}
            fill="none"
            className="stroke-foreground/30"
            strokeWidth={1}
          />
          <rect
            x={CELL + GAP}
            y={CELL + GAP}
            width={SIZE - 2 * CELL - GAP * 2}
            height={SIZE - 2 * CELL - GAP * 2}
            rx={2}
            fill="none"
            className="stroke-foreground/20"
            strokeDasharray="3 3"
            strokeWidth={0.75}
          />
        </svg>

        {/* readout */}
        <div className="w-full max-w-[16rem] space-y-2 text-sm">
          <div className="flex items-center gap-2">
            <MotifSwatch color={selected} width={28} />
            <span className="font-semibold">{t.colour(selected)}</span>
            <span
              className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                isRare
                  ? "bg-amber-500/15 text-amber-700 dark:text-amber-300"
                  : "bg-muted text-muted-foreground"
              }`}
            >
              {isRare ? t.rare : t.common}
            </span>
          </div>
          <dl className="space-y-1 tabular-nums">
            <div className="flex justify-between border-b pb-1">
              <dt className="text-muted-foreground">{t.onFrame}</dt>
              <dd className="font-semibold">{row?.frame ?? 0}</dd>
            </div>
            <div className="flex justify-between border-b pb-1">
              <dt className="text-muted-foreground">{t.inInterior}</dt>
              <dd className={`font-semibold ${row?.interior === 0 ? "text-amber-600 dark:text-amber-400" : ""}`}>
                {row?.interior ?? 0}
              </dd>
            </div>
            <div className="flex justify-between">
              <dt className="text-muted-foreground">{t.totalEdges}</dt>
              <dd className="font-semibold">{row?.total ?? 0}</dd>
            </div>
          </dl>
          <p className="text-xs leading-relaxed text-muted-foreground">
            {isRare ? t.rareNote : t.commonNote}
          </p>
        </div>
      </div>
    </div>
  );
}
