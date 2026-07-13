// A before/after board pair for the ALNS page: the raw builder's board next to
// the ALNS-lifted board, each rendered with the SAME conflict-highlighting the
// board viewer and every other research board use (LazyBoardPreview →
// conflictEdges), so the red mismatch marks are consistent site-wide. The delta
// between them ("+N edges") makes what the refinement bought visible at a glance.

import { useLang } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { LazyBoardPreview } from "@/components/research/LazyBoardPreview";
import { RECORD_BOARDS } from "@/data/record-boards";

const T = {
  en: { before: "Raw builder", after: "After ALNS", edges: "edges", open: "Open", missing: "board not found:" },
  fr: { before: "Constructeur brut", after: "Après ALNS", edges: "arêtes", open: "Ouvrir", missing: "plateau introuvable :" },
};

interface Side {
  /** A RECORD_BOARDS id, OR pass `params` directly. */
  id?: string;
  /** Bucas viewer params, if not sourced from a record board. */
  params?: string;
  /** Matched-edges score shown under the board. */
  score?: number;
  /** Override the default label. */
  label?: string;
}

function resolve(side: Side): { params: string; score?: number } | null {
  if (side.params) return { params: side.params, ...(side.score !== undefined ? { score: side.score } : {}) };
  if (side.id) {
    const b = RECORD_BOARDS.find((x) => x.id === side.id);
    if (b) return { params: b.viewerParams, score: side.score ?? b.score };
  }
  return null;
}

function BoardCol({ side, fallbackLabel }: { side: Side; fallbackLabel: string }) {
  const t = T[useLang().lang];
  const r = resolve(side);
  const label = side.label ?? fallbackLabel;
  if (!r) {
    return (
      <div className="flex-1">
        <div className="mb-1 text-xs font-semibold uppercase tracking-wide text-muted-foreground">{label}</div>
        <p className="text-sm text-destructive">{t.missing} {side.id ?? "?"}</p>
      </div>
    );
  }
  return (
    <div className="flex-1">
      <div className="mb-1 flex items-baseline justify-between gap-2">
        <span className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{label}</span>
        {r.score !== undefined && (
          <span className="text-sm font-semibold tabular-nums">
            {r.score}
            <span className="text-xs font-normal text-muted-foreground">/480</span>
          </span>
        )}
      </div>
      <LocalizedLink
        to={`/viewer?${r.params}`}
        className="block rounded-lg border p-2 transition-shadow hover:shadow-md"
      >
        <LazyBoardPreview params={r.params} showConflicts />
      </LocalizedLink>
    </div>
  );
}

export function BeforeAfterBoards({ before, after }: { before: Side; after: Side }) {
  const t = T[useLang().lang];
  const b = resolve(before);
  const a = resolve(after);
  const delta = a?.score !== undefined && b?.score !== undefined ? a.score - b.score : undefined;

  return (
    <div className="not-prose my-8">
      <div className="flex flex-col items-stretch gap-4 sm:flex-row sm:items-center">
        <BoardCol side={before} fallbackLabel={t.before} />
        {delta !== undefined && (
          <div className="flex shrink-0 flex-col items-center justify-center px-1 text-center">
            <span className="text-lg font-semibold tabular-nums text-emerald-600 dark:text-emerald-400">
              {delta >= 0 ? "+" : ""}
              {delta}
            </span>
            <span className="text-[0.7rem] uppercase tracking-wide text-muted-foreground">{t.edges}</span>
          </div>
        )}
        <BoardCol side={after} fallbackLabel={t.after} />
      </div>
    </div>
  );
}
