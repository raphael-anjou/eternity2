import type { ReactNode } from "react";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { decodeBucas } from "@/lib/bucas";
import { RECORD_BOARDS } from "@/data/record-boards";

// Shared layout for an invention write-up under /research/lab/inventions. Each
// invention page supplies its own bilingual prose; this gives them a consistent
// frame: title, score badge, the four standard sections, a verified board
// preview (when one exists) that links into the viewer, and the honesty note
// about reproducibility.

export type Reproducibility = "deterministic" | "seeded" | "stochastic";

export interface InventionCopy {
  name: string;
  tagline: string;
  idea: string;
  how: ReactNode;
  result: ReactNode;
  open: ReactNode;
}

const LABELS = {
  en: {
    back: "← Inventions",
    ideaTitle: "The idea",
    howTitle: "How it works",
    resultTitle: "The result",
    openTitle: "Open questions",
    boardTitle: "The board",
    boardCta: "Open in the viewer",
    boardNote: "Score recomputed from the board itself. Open it to check every edge.",
    reproTitle: "Reproducing it",
    repro: {
      deterministic:
        "Deterministic: the same inputs produce the same board every time.",
      seeded:
        "Seeded: fix the seed and the construction repeats; the later refinement may still vary.",
      stochastic:
        "Stochastic: the search uses randomness and won't reproduce the exact board. The board it found is bundled here and is fully checkable in the viewer.",
    } as Record<Reproducibility, string>,
    scoreSuffix: "/ 480 matched edges",
  },
  fr: {
    back: "← Inventions",
    ideaTitle: "L'idée",
    howTitle: "Comment ça marche",
    resultTitle: "Le résultat",
    openTitle: "Questions ouvertes",
    boardTitle: "Le plateau",
    boardCta: "Ouvrir dans le visualiseur",
    boardNote: "Score recalculé à partir du plateau lui-même. Ouvrez-le pour vérifier chaque bord.",
    reproTitle: "Le reproduire",
    repro: {
      deterministic:
        "Déterministe : les mêmes entrées produisent le même plateau à chaque fois.",
      seeded:
        "À graine fixée : fixez la graine et la construction se répète ; l'affinage ultérieur peut varier.",
      stochastic:
        "Stochastique : la recherche utilise du hasard et ne reproduira pas le plateau exact. Le plateau trouvé est fourni ici et entièrement vérifiable dans le visualiseur.",
    } as Record<Reproducibility, string>,
    scoreSuffix: "/ 480 bords appariés",
  },
};

export function InventionLayout({
  copy,
  score,
  boardId,
  reproducibility,
  visual,
}: {
  copy: { en: InventionCopy; fr: InventionCopy };
  score: number;
  boardId?: string;
  reproducibility: Reproducibility;
  /** Optional animation/diagram shown after "How it works". */
  visual?: ReactNode;
}) {
  const t = useT(copy);
  const l = useT(LABELS);
  const board = boardId ? RECORD_BOARDS.find((b) => b.id === boardId) : undefined;
  const cells = board ? decodeBucas(board.viewerParams).cells : null;

  return (
    <div className="space-y-10">
      <div>
        <LocalizedLink
          to="/research/lab/inventions"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {l.back}
        </LocalizedLink>
        <div className="mt-2 flex flex-wrap items-baseline gap-x-4 gap-y-1">
          <h1 className="text-3xl font-bold tracking-tight">{t.name}</h1>
          <span className="text-sm text-muted-foreground">
            <span className="text-lg font-bold text-foreground tabular-nums">{score}</span>{" "}
            {l.scoreSuffix}
          </span>
        </div>
        <p className="mt-3 max-w-3xl text-lg text-muted-foreground">{t.tagline}</p>
      </div>

      <section className="max-w-3xl space-y-2">
        <h2 className="text-xl font-semibold tracking-tight">{l.ideaTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.idea}</p>
      </section>

      <section className="max-w-3xl space-y-2">
        <h2 className="text-xl font-semibold tracking-tight">{l.howTitle}</h2>
        <div className="space-y-3 text-sm leading-relaxed text-muted-foreground">{t.how}</div>
      </section>

      {visual && <section className="space-y-3">{visual}</section>}

      {board && cells && (
        <section className="space-y-3">
          <h2 className="text-xl font-semibold tracking-tight">{l.boardTitle}</h2>
          <div className="max-w-md space-y-2">
            <LocalizedLink to={`/viewer?${board.viewerParams}`} className="block rounded-lg border p-2 transition-shadow hover:shadow-md">
              <BoardSvg width={16} height={16} cells={cells} />
            </LocalizedLink>
            <div className="flex items-center justify-between gap-2">
              <p className="text-xs text-muted-foreground">{l.boardNote}</p>
              <LocalizedLink
                to={`/viewer?${board.viewerParams}`}
                className="shrink-0 text-sm font-medium underline hover:text-foreground"
              >
                {l.boardCta}
              </LocalizedLink>
            </div>
          </div>
        </section>
      )}

      <section className="max-w-3xl space-y-2">
        <h2 className="text-xl font-semibold tracking-tight">{l.resultTitle}</h2>
        <div className="space-y-3 text-sm leading-relaxed text-muted-foreground">{t.result}</div>
      </section>

      <section className="max-w-3xl space-y-2">
        <h2 className="text-xl font-semibold tracking-tight">{l.openTitle}</h2>
        <div className="space-y-3 text-sm leading-relaxed text-muted-foreground">{t.open}</div>
      </section>

      <section className="max-w-3xl space-y-2 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{l.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{l.repro[reproducibility]}</p>
      </section>
    </div>
  );
}
