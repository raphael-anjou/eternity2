// A verified record-board preview for MDX research pages: renders the board
// from its bundled edges and links into the viewer so every edge is checkable.
// Replaces the board section of the old InventionLayout.

import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { decodeBucas } from "@/lib/bucas";
import { RECORD_BOARDS } from "@/data/record-boards";

const T = {
  en: {
    cta: "Open in the viewer",
    note: "Score recomputed from the board itself. Open it to check every edge.",
    missing: "board not found:",
  },
  fr: {
    cta: "Ouvrir dans le visualiseur",
    note: "Score recalculé depuis le plateau lui-même. Ouvrez-le pour vérifier chaque arête.",
    missing: "plateau introuvable :",
  },
};

export function RecordBoard({ id }: { id: string }) {
  const t = useT(T);
  const board = RECORD_BOARDS.find((b) => b.id === id);
  if (!board) {
    return (
      <p className="text-sm text-destructive">
        {t.missing} {id}
      </p>
    );
  }
  const cells = decodeBucas(board.viewerParams).cells;
  return (
    <div className="my-6 max-w-md space-y-2">
      <LocalizedLink
        to={`/viewer?${board.viewerParams}`}
        className="block rounded-lg border p-2 transition-shadow hover:shadow-md"
      >
        <BoardSvg width={16} height={16} cells={cells} />
      </LocalizedLink>
      <div className="flex items-center justify-between gap-2">
        <p className="text-xs text-muted-foreground">{t.note}</p>
        <LocalizedLink
          to={`/viewer?${board.viewerParams}`}
          className="shrink-0 text-sm font-medium underline hover:text-foreground"
        >
          {t.cta}
        </LocalizedLink>
      </div>
    </div>
  );
}
