import { useMemo, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { useT } from "@/i18n";
import type { Edges } from "@/lib/bucas";
import data from "@/data/dfs-instances.json";

// A small carousel over the ten corner-pinned starting instances the DFS study
// (and the other engine studies) run on. Each is the official 256-piece puzzle
// with the same eight pinned cells (the five official clues plus three corners),
// arranged differently per variant, which is what makes the ten distinct but
// equally hard. The board is otherwise empty: this shows the STARTING position,
// not a solution, so a reader can see exactly what each engine is handed.

type HintCell = { pos: number; edges: number[] };
type Instance = { id: string; name: string; hintCount: number; hints: HintCell[] };

const D: { width: number; height: number; instances: Instance[] } = data;

const T = {
  en: {
    title: "The ten starting puzzles",
    intro:
      "Every engine runs on these ten instances: the official 256-piece puzzle with eight cells pinned (the five official clues and three corners), arranged differently each time. Only the pinned cells are shown; the rest is what the search must fill.",
    of: "of",
    hints: "pinned cells",
    prev: "Previous",
    next: "Next",
  },
  fr: {
    title: "Les dix puzzles de départ",
    intro:
      "Chaque moteur tourne sur ces dix instances : le puzzle officiel de 256 pièces avec huit cases fixées (les cinq indices officiels et trois coins), disposées différemment à chaque fois. Seules les cases fixées sont montrées ; le reste est ce que la recherche doit remplir.",
    of: "sur",
    hints: "cases fixées",
    prev: "Précédent",
    next: "Suivant",
  },
};

export function StartingPuzzleCarousel() {
  const t = useT(T);
  const [i, setI] = useState(0);
  const n = D.instances.length;
  const inst = D.instances[i];

  // Build the 256-cell board for the current instance: the pinned cells carry
  // their edges, every other cell is empty (null).
  const { cells, highlight } = useMemo(() => {
    const total = D.width * D.height;
    const cells: (Edges | null)[] = Array.from({ length: total }, () => null);
    const highlight: number[] = [];
    for (const h of inst?.hints ?? []) {
      const e = h.edges;
      if (e.length === 4) {
        cells[h.pos] = [e[0], e[1], e[2], e[3]] as Edges;
        highlight.push(h.pos);
      }
    }
    return { cells, highlight };
  }, [inst]);

  if (!inst) return null;

  return (
    <div className="not-prose my-6">
      <h3 className="text-base font-semibold tracking-tight">{t.title}</h3>
      <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.intro}</p>

      <div className="mt-4 flex flex-col items-center gap-3">
        <div className="w-full max-w-sm">
          <BoardSvg
            width={D.width}
            height={D.height}
            cells={cells}
            highlight={highlight}
            coordinates
            className="w-full"
          />
        </div>

        <div className="flex w-full max-w-sm items-center justify-between gap-3">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setI((v) => (v - 1 + n) % n)}
            aria-label={t.prev}
          >
            ‹ {t.prev}
          </Button>

          <div className="text-center">
            <div className="font-mono text-sm font-medium">{inst.id}</div>
            <div className="text-xs text-muted-foreground">
              {i + 1} {t.of} {n} · {inst.hintCount} {t.hints}
            </div>
          </div>

          <Button
            variant="outline"
            size="sm"
            onClick={() => setI((v) => (v + 1) % n)}
            aria-label={t.next}
          >
            {t.next} ›
          </Button>
        </div>

        {/* Dot indicators, also clickable, so the whole set is reachable. */}
        <div className="flex flex-wrap justify-center gap-1.5">
          {D.instances.map((it, idx) => (
            <button
              key={it.id}
              onClick={() => setI(idx)}
              aria-label={it.id}
              aria-current={idx === i}
              className={
                "h-2 w-2 rounded-full transition-colors " +
                (idx === i ? "bg-primary" : "bg-muted-foreground/30 hover:bg-muted-foreground/60")
              }
            />
          ))}
        </div>
      </div>
    </div>
  );
}
