import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { parseParams, decodeBucas } from "@/lib/bucas";
import { RECORD_BOARDS } from "@/data/record-boards";
import { KNOWN_BOARDS } from "@/data/known-boards";

// A gallery of notable boards as case studies (distinct from the Records
// timeline). Each tile shows the board and links into the viewer using native
// eternity2.dev params (board_w/board_h and motifs_order stripped).

type Tile = {
  key: string;
  score: number;
  origin: "project" | "community";
  viewerParams: string;
};

// Native params: keep puzzle_size + board_edges (+ board_pieces); drop bucas-only
// board_w/board_h and motifs_order.
function toNative(raw: Record<string, string>, name: string): string {
  const size = raw["puzzle_size"] ?? raw["board_w"] ?? "16";
  const parts = [`puzzle=${name}`, `puzzle_size=${size}`];
  if (raw["board_edges"]) parts.push(`board_edges=${raw["board_edges"]}`);
  if (raw["board_pieces"]) parts.push(`board_pieces=${raw["board_pieces"]}`);
  return parts.join("&");
}

// Project record boards (verified) already carry native params.
const PROJECT: Tile[] = RECORD_BOARDS.map((b) => ({
  key: b.id,
  score: b.score,
  origin: "project" as const,
  viewerParams: b.viewerParams,
}));

// Community boards from the bundled viewer data; convert to native params.
const COMMUNITY: Tile[] = KNOWN_BOARDS.filter((b) => b.score != null).map((b) => ({
  key: b.id,
  score: b.score as number,
  origin: "community" as const,
  viewerParams: toNative(parseParams(b.params), b.id),
}));

const ALL: Tile[] = [...PROJECT, ...COMMUNITY].sort((a, b) => b.score - a.score);

const T = {
  en: {
    backLabel: "← The lab notebook",
    title: "Notable boards",
    intro:
      "A gallery of the boards worth looking at: community records and the best boards this project produced. Each one is real and opens in the viewer, where you can check every edge. Sorted by score.",
    project: "this project",
    community: "community",
    matched: "matched edges",
    viewNote: "Click any board to open it in the viewer.",
  },
  fr: {
    backLabel: "← Le carnet de laboratoire",
    title: "Plateaux notables",
    intro:
      "Une galerie des plateaux qui méritent un coup d'œil : les records de la communauté et les meilleurs plateaux produits par ce projet. Chacun est réel et s'ouvre dans le visualiseur, où vous pouvez vérifier chaque bord. Triés par score.",
    project: "ce projet",
    community: "communauté",
    matched: "bords appariés",
    viewNote: "Cliquez sur un plateau pour l'ouvrir dans le visualiseur.",
  },
};

export default function BasinsHub() {
  const t = useT(T);
  return (
    <div className="space-y-8">
      <div>
        <LocalizedLink
          to="/research/lab"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
        {ALL.map((tile) => {
          let cells;
          try {
            cells = decodeBucas(tile.viewerParams).cells;
          } catch {
            return null;
          }
          return (
            <LocalizedLink
              key={tile.key}
              to={`/viewer?${tile.viewerParams}`}
              className="group block space-y-2 rounded-lg border p-3 transition-shadow hover:shadow-md"
            >
              <div className="flex items-baseline justify-between">
                <span className="text-xl font-bold tabular-nums">{tile.score}</span>
                <span
                  className={
                    "rounded-full px-2 py-0.5 text-[10px] font-medium tracking-wide uppercase " +
                    (tile.origin === "project"
                      ? "bg-sky-100 text-sky-800 dark:bg-sky-950 dark:text-sky-300"
                      : "bg-muted text-muted-foreground")
                  }
                >
                  {tile.origin === "project" ? t.project : t.community}
                </span>
              </div>
              <BoardSvg width={16} height={16} cells={cells} />
              <div className="text-xs text-muted-foreground">{t.matched}</div>
            </LocalizedLink>
          );
        })}
      </section>

      <p className="text-center text-xs text-muted-foreground">{t.viewNote}</p>
    </div>
  );
}

export const meta = pageMeta("basins");
