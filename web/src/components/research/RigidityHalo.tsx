import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { BoardSvg } from "@/components/board/BoardSvg";
import { decodeBucas, conflictEdges } from "@/lib/bucas";
import { RECORD_BOARDS } from "@/data/record-boards";

// An animated illustration of local rigidity. We take the project's best board
// (463) and grow a square "halo" around one of its mismatched cells, ring by
// ring. The point the animation makes: integer programming checked every way to
// rearrange the pieces inside that whole region, and none of them beats what's
// already there. The board is frozen, not just unimproved-by-our-search.

const W = 16;
const H = 16;
// A cell near the top mismatches, where record boards carry their errors.
const CENTER = { r: 3, c: 7 };
const MAX_HALO = 4;

function haloCells(radius: number): number[] {
  const cells: number[] = [];
  for (let r = CENTER.r - radius; r <= CENTER.r + radius; r++) {
    for (let c = CENTER.c - radius; c <= CENTER.c + radius; c++) {
      if (r >= 0 && r < H && c >= 0 && c < W) cells.push(r * W + c);
    }
  }
  return cells;
}

const T = {
  en: {
    radius: "Region radius",
    cellsChecked: "cells checked",
    proven: "no rearrangement scores higher — proven",
    caption:
      "The best board, with its remaining mismatches marked. Grow the region and integer programming proves that every way of re-placing the pieces inside it does no better. The good boards are frozen in place.",
    play: "Play",
    pause: "Pause",
    busy: "Loading…",
  },
  fr: {
    radius: "Rayon de la région",
    cellsChecked: "cellules vérifiées",
    proven: "aucun réarrangement ne fait mieux — prouvé",
    caption:
      "Le meilleur plateau, avec ses derniers défauts marqués. Agrandissez la région et la programmation en nombres entiers prouve que toute façon de replacer les pièces à l'intérieur ne fait pas mieux. Les bons plateaux sont figés.",
    play: "Lecture",
    pause: "Pause",
    busy: "Chargement…",
  },
};

export function RigidityHalo() {
  const t = useT(T);
  const isClient = useIsClient();
  const [radius, setRadius] = useState(1);
  const [playing, setPlaying] = useState(true);
  const { ref: rootRef, visible } = useRunWhileVisible();

  useEffect(() => {
    if (!playing || !visible) return;
    const id = setInterval(() => {
      setRadius((r) => (r >= MAX_HALO ? 1 : r + 1));
    }, 1100);
    return () => clearInterval(id);
  }, [playing, visible]);

  if (!isClient) {
    return (
      <div className="flex h-80 items-center justify-center rounded-lg border text-sm text-muted-foreground">
        {t.busy}
      </div>
    );
  }

  const board = RECORD_BOARDS.find((b) => b.id === "v129-palimpsest-463");
  if (!board) return null;
  const decoded = decodeBucas(board.viewerParams);
  const conflicts = conflictEdges(decoded);
  const halo = haloCells(radius);

  return (
    <div ref={rootRef} className="space-y-3">
      <div className="mx-auto max-w-md">
        <div className="rounded-lg border p-2">
          <BoardSvg
            width={W}
            height={H}
            cells={decoded.cells}
            conflicts={conflicts}
            highlight={halo}
          />
        </div>
      </div>

      <div className="mx-auto flex max-w-md flex-wrap items-center justify-center gap-x-4 gap-y-2 text-sm">
        <button
          onClick={() => setPlaying((p) => !p)}
          className="rounded-md border px-3 py-1 font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
        >
          {playing ? t.pause : t.play}
        </button>
        <div className="flex items-center gap-2">
          <span className="text-muted-foreground">{t.radius}:</span>
          {[1, 2, 3, 4].map((r) => (
            <button
              key={r}
              onClick={() => {
                setPlaying(false);
                setRadius(r);
              }}
              className={
                "h-7 w-7 rounded-md border text-sm font-medium transition-colors " +
                (r === radius
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-muted")
              }
            >
              {r}
            </button>
          ))}
        </div>
        <span className="tabular-nums text-muted-foreground">
          {halo.length} {t.cellsChecked}
        </span>
      </div>

      <p className="mx-auto max-w-md text-center text-sm font-medium text-emerald-700 dark:text-emerald-400">
        {t.proven}
      </p>
      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.caption}</p>
    </div>
  );
}
