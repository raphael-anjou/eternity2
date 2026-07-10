import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";

// The five fixed clue placements on the 16x16 board, drawn to scale. Bucas
// notation: rows A-P top to bottom, columns 1-16 left to right. The mandatory
// centre clue sits near the middle; the four clue-puzzle placements are inset
// three cells from each corner, in a symmetric ring. Static: positions are
// exact, from the official clue set.

const N = 16;
const CELL = 20;

// (row, col, label, piece) — 0-indexed from the bucas squares.
const CLUES: Array<{ r: number; c: number; sq: string; piece: number; center?: boolean }> = [
  { r: 8, c: 7, sq: "I8", piece: 139, center: true },
  { r: 2, c: 2, sq: "C3", piece: 208 },
  { r: 2, c: 13, sq: "C14", piece: 255 },
  { r: 13, c: 2, sq: "N3", piece: 181 },
  { r: 13, c: 13, sq: "N14", piece: 249 },
];

const T = {
  en: {
    centre: "mandatory centre clue (I8)",
    corner: "four clue-puzzle placements",
    caption:
      "The five fixed pieces, to scale on the 16x16 grid. The mandatory centre clue is near the middle; the other four sit inset three cells from each corner, in a symmetric ring. A board that honours all five is strict-canonical; most records honour only the centre.",
    busy: "Loading…",
  },
  fr: {
    centre: "indice central obligatoire (I8)",
    corner: "quatre placements des puzzles-indices",
    caption:
      "Les cinq pièces fixes, à l'échelle sur la grille 16x16. L'indice central obligatoire est proche du milieu ; les quatre autres sont en retrait de trois cases de chaque coin, en anneau symétrique. Un plateau respectant les cinq est strict-canonique ; la plupart des records ne respectent que le centre.",
    busy: "Chargement…",
  },
};

export function CluePositionsDiagram() {
  const t = useT(T);
  const isClient = useIsClient();

  if (!isClient) {
    return (
      <div className="flex h-72 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const at = (r: number, c: number) => CLUES.find((x) => x.r === r && x.c === c);

  return (
    <div className="space-y-3">
      <div className="mx-auto max-w-sm">
        <svg
          viewBox={`0 0 ${N * CELL} ${N * CELL}`}
          className="w-full rounded-lg border bg-card"
          role="img"
          aria-label="16 by 16 board grid with the five fixed clue pieces marked: the mandatory centre clue at square I8 near the middle, and four clue-puzzle placements inset three cells from each corner at C3, C14, N3 and N14"
        >
          {Array.from({ length: N }, (_, r) =>
            Array.from({ length: N }, (_, c) => {
              const clue = at(r, c);
              return (
                <rect
                  key={`${r}-${c}`}
                  x={c * CELL + 1}
                  y={r * CELL + 1}
                  width={CELL - 2}
                  height={CELL - 2}
                  rx={2}
                  className={clue ? (clue.center ? "fill-amber-400" : "fill-sky-500") : "fill-muted"}
                />
              );
            }),
          )}
          {CLUES.map((clue) => (
            <text
              key={clue.sq}
              x={clue.c * CELL + CELL / 2}
              y={clue.r * CELL + CELL / 2 + 3}
              textAnchor="middle"
              className="fill-white font-mono"
              style={{ fontSize: 8 }}
            >
              {clue.sq}
            </text>
          ))}
        </svg>
      </div>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-amber-400" />
          {t.centre}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block h-2.5 w-2.5 rounded-sm bg-sky-500" />
          {t.corner}
        </span>
      </div>
      <p className="mx-auto max-w-md text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
