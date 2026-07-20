// The toolkit's board generator: make solvable Eternity-II boards with real
// colour balance, one at a time or in a batch (a seed range, optional pinned
// clues), and open any of them in the viewer to score edge by edge. This is the
// browser twin of the starter kit's `generate` / `generate_batch` — the same
// WASM generator the site has always used, now on its own page rather than
// buried in the viewer. The viewer keeps the scorer; this makes the boards.
import { useMemo, useState } from "react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { LazyBoardPreview } from "@/components/research/LazyBoardPreview";
import { LocalizedLink } from "@/components/LocalizedLink";
import { getGeneratedSolvedPuzzleFramed, getMaxColors } from "@/engine";
import { useEngine } from "@/engine/useEngine";
import { ourParams, bucasParams } from "@/lib/bucas";
import { useT } from "@/i18n";
import type { Puzzle } from "@/lib/types";

/** The identity solution of a generated puzzle: piece i at cell i, rotation 0. */
function identityBoard(puzzle: Puzzle): number[] {
  return Array.from({ length: puzzle.width * puzzle.height }, (_, i) => i * 4);
}

/** One generated board, reduced to what the gallery and the copy actions need. */
interface GenBoard {
  seed: number;
  /** viewer query string (our params), for LazyBoardPreview + a viewer link. */
  params: string;
  /** legacy bucas query, for the "copy URLs" action (community interop). */
  bucas: string;
  size: number;
  colors: number;
}

const MAX_BATCH = 120;

export function BoardGenerator() {
  const t = useT(T);
  const ready = useEngine();

  const [size, setSize] = useState(16);
  const [colors, setColors] = useState(22);
  const [framed, setFramed] = useState(true);

  // Batch controls.
  const [fromSeed, setFromSeed] = useState(1);
  const [count, setCount] = useState(12);

  const [boards, setBoards] = useState<GenBoard[]>([]);
  const [copied, setCopied] = useState<null | "jsonl" | "urls">(null);

  const maxColors = ready ? getMaxColors(size) : 22;

  /** Generate one board for a seed and reduce it to a GenBoard. */
  const makeBoard = (seed: number): GenBoard => {
    const puzzle = getGeneratedSolvedPuzzleFramed(size, colors, seed, framed);
    const cells = identityBoard(puzzle);
    const name = `gen-${size}x${size}-c${colors}-s${seed}`;
    const params = new URLSearchParams(ourParams(puzzle, cells, name)).toString();
    return { seed, params, bucas: bucasParams(puzzle, cells, name), size, colors };
  };

  const generateBatch = () => {
    if (!ready) return;
    const n = Math.max(1, Math.min(MAX_BATCH, count));
    const next: GenBoard[] = [];
    for (let i = 0; i < n; i++) next.push(makeBoard(fromSeed + i));
    setBoards(next);
    setCopied(null);
  };

  const copyJsonl = () => {
    // One compact JSON object per line: the fields a downstream tool needs to
    // reconstruct or open each board. (The kit's site-schema JSON carries the
    // full piece set; here we ship the reproducible generator parameters plus
    // the viewer URL, which is what a browser workflow actually wants.)
    const lines = boards
      .map((b) =>
        JSON.stringify({
          seed: b.seed,
          size: b.size,
          colors: b.colors,
          framed,
          url: `https://eternity2.dev/viewer?${b.params}`,
        }),
      )
      .join("\n");
    void navigator.clipboard.writeText(lines);
    setCopied("jsonl");
    setTimeout(() => setCopied(null), 1500);
  };

  const copyUrls = () => {
    const urls = boards.map((b) => `https://eternity2.dev/viewer?${b.params}`).join("\n");
    void navigator.clipboard.writeText(urls);
    setCopied("urls");
    setTimeout(() => setCopied(null), 1500);
  };

  const seedRangeLabel = useMemo(
    () => t.seedRange(fromSeed, fromSeed + Math.max(1, Math.min(MAX_BATCH, count)) - 1),
    [t, fromSeed, count],
  );

  return (
    <Card className="not-prose my-6">
      <CardHeader>
        <CardTitle className="text-base">{t.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-5">
        <p className="text-sm text-muted-foreground">{t.intro}</p>

        <div className="grid gap-5 sm:grid-cols-2">
          <div className="space-y-1.5">
            <Label>{t.sizeLabel(size)}</Label>
            <Slider
              min={4}
              max={16}
              step={1}
              value={size}
              onValueChange={(v) => {
                const s = singleSliderValue(v);
                setSize(s);
                if (ready) setColors((c) => Math.min(c, getMaxColors(s)));
              }}
            />
          </div>
          <div className="space-y-1.5">
            <Label>{t.colorsLabel(colors)}</Label>
            <Slider
              min={2}
              max={maxColors}
              step={1}
              value={Math.min(colors, maxColors)}
              onValueChange={(v) => setColors(singleSliderValue(v))}
            />
          </div>
        </div>

        <div className="flex items-center justify-between gap-3">
          <div>
            <Label htmlFor="gen-framed">{t.framed}</Label>
            <p className="text-xs text-muted-foreground">{t.framedHint}</p>
          </div>
          <Switch id="gen-framed" checked={framed} onCheckedChange={setFramed} />
        </div>

        <div className="grid gap-5 sm:grid-cols-2">
          <div className="space-y-1.5">
            <Label htmlFor="gen-from">{t.firstSeed}</Label>
            <input
              id="gen-from"
              type="number"
              min={0}
              value={fromSeed}
              onChange={(e) => setFromSeed(Math.max(0, Number(e.target.value) || 0))}
              className="w-full rounded-md border border-input bg-background px-3 py-1.5 text-sm"
            />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="gen-count">{t.howMany(Math.min(MAX_BATCH, count))}</Label>
            <Slider
              min={1}
              max={MAX_BATCH}
              step={1}
              value={Math.min(MAX_BATCH, count)}
              onValueChange={(v) => setCount(singleSliderValue(v))}
            />
          </div>
        </div>

        <div className="flex flex-wrap items-center gap-3">
          <Button disabled={!ready} onClick={generateBatch}>
            {ready ? t.generate(seedRangeLabel) : t.loading}
          </Button>
          {boards.length > 0 && (
            <>
              <Button variant="outline" onClick={copyUrls}>
                {copied === "urls" ? t.copied : t.copyUrls}
              </Button>
              <Button variant="outline" onClick={copyJsonl}>
                {copied === "jsonl" ? t.copied : t.copyJsonl}
              </Button>
            </>
          )}
        </div>

        {boards.length > 0 && (
          <>
            <p className="text-xs text-muted-foreground">{t.galleryHint}</p>
            <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4">
              {boards.map((b) => (
                <LocalizedLink
                  key={b.seed}
                  to={`/viewer?${b.params}`}
                  className="group block rounded-md border border-border p-1.5 transition-colors hover:border-primary"
                  title={t.openSeed(b.seed)}
                >
                  <LazyBoardPreview params={b.params} showConflicts={false} />
                  <div className="mt-1 text-center text-[11px] text-muted-foreground">
                    {t.seedTag(b.seed)}
                  </div>
                </LocalizedLink>
              ))}
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
}

const T = {
  en: {
    title: "Board generator",
    intro:
      "Generate solvable boards with real Eternity-II colour balance — five border colours confined to the frame, the interior balanced, all pieces distinct. Every board is deterministic from its seed, so a seed range is a reproducible batch. Click any board to open it in the viewer and score it edge by edge.",
    sizeLabel: (n: number) => `Board size: ${n}×${n}`,
    colorsLabel: (n: number) => `Interior colours: ${n}`,
    framed: "Frame-restricted colours",
    framedHint: "Confine five border colours to the frame, like the real puzzle.",
    firstSeed: "First seed",
    howMany: (n: number) => `How many boards: ${n}`,
    seedRange: (a: number, b: number) => (a === b ? `seed ${a}` : `seeds ${a}–${b}`),
    generate: (range: string) => `Generate ${range}`,
    loading: "Loading engine…",
    copyUrls: "Copy viewer URLs",
    copyJsonl: "Copy as JSONL",
    copied: "Copied ✓",
    galleryHint: "Each board is its solution — open it in the viewer to see the pieces and score.",
    openSeed: (s: number) => `Open seed ${s} in the viewer`,
    seedTag: (s: number) => `seed ${s}`,
  },
  fr: {
    title: "Générateur de plateaux",
    intro:
      "Générez des plateaux résolubles avec le vrai équilibre des couleurs d'Eternity II : cinq couleurs de bordure réservées au cadre, l'intérieur équilibré, toutes les pièces distinctes. Chaque plateau est déterministe selon sa graine, donc une plage de graines forme un lot reproductible. Cliquez sur un plateau pour l'ouvrir dans le visualiseur et le scorer arête par arête.",
    sizeLabel: (n: number) => `Taille du plateau : ${n}×${n}`,
    colorsLabel: (n: number) => `Couleurs intérieures : ${n}`,
    framed: "Couleurs réservées au cadre",
    framedHint: "Confine cinq couleurs de bordure au cadre, comme le vrai puzzle.",
    firstSeed: "Première graine",
    howMany: (n: number) => `Nombre de plateaux : ${n}`,
    seedRange: (a: number, b: number) => (a === b ? `graine ${a}` : `graines ${a}–${b}`),
    generate: (range: string) => `Générer ${range}`,
    loading: "Chargement du moteur…",
    copyUrls: "Copier les liens du visualiseur",
    copyJsonl: "Copier en JSONL",
    copied: "Copié ✓",
    galleryHint:
      "Chaque plateau est sa solution — ouvrez-le dans le visualiseur pour voir les pièces et le score.",
    openSeed: (s: number) => `Ouvrir la graine ${s} dans le visualiseur`,
    seedTag: (s: number) => `graine ${s}`,
  },
  es: {
    title: "Generador de tableros",
    intro:
      "Genera tableros resolubles con el equilibrio de colores real de Eternity II: cinco colores de borde confinados al marco, el interior equilibrado, todas las piezas distintas. Cada tablero es determinista según su semilla, así que un rango de semillas es un lote reproducible. Haz clic en cualquier tablero para abrirlo en el visor y puntuarlo arista por arista.",
    sizeLabel: (n: number) => `Tamaño del tablero: ${n}×${n}`,
    colorsLabel: (n: number) => `Colores interiores: ${n}`,
    framed: "Colores reservados al marco",
    framedHint: "Confina cinco colores de borde al marco, como el puzzle real.",
    firstSeed: "Primera semilla",
    howMany: (n: number) => `Cuántos tableros: ${n}`,
    seedRange: (a: number, b: number) => (a === b ? `semilla ${a}` : `semillas ${a}–${b}`),
    generate: (range: string) => `Generar ${range}`,
    loading: "Cargando el motor…",
    copyUrls: "Copiar URLs del visor",
    copyJsonl: "Copiar como JSONL",
    copied: "Copiado ✓",
    galleryHint:
      "Cada tablero es su solución — ábrelo en el visor para ver las piezas y la puntuación.",
    openSeed: (s: number) => `Abrir la semilla ${s} en el visor`,
    seedTag: (s: number) => `semilla ${s}`,
  },
};
