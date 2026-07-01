import { useT } from "@/i18n";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import refData from "@/data/reference-table.json";
import publishedData from "@/data/published-reference.json";

const DASH = "—"; // em dash, for genuinely-N/A cells (e.g. the encircle "empty" column)

// GitHub article this table is computed from — the "research/" topic that holds
// the article, the reproducible Rust generator, and the committed results JSON.
const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/subgrid-placement-counts";

type Row = {
  key: string;
  size: number;
  position: string;
  vide: number | null;
  fixe: number | null;
  fixe4: number | null;
  cN: [number, number, number, number] | null;
  note?: string;
};

const ROWS = refData.rows as Row[];

// Sylvogel's published values for the cells our exact generator cannot recompute
// (intractable distinct-counts). Shown in italics to mark them as published, not
// independently verified by this site.
const PUBLISHED = publishedData.values as Record<
  string,
  Partial<Record<"vide" | "fixe" | "fixe4", number>>
>;

// Thin-space thousands grouping, matching the source table's "1 312" style.
function fmt(n: number): string {
  return n.toLocaleString("en-US").replace(/,/g, " ");
}

// One numeric cell: prefer our computed value (plain); fall back to sylvogel's
// published value (italic, muted); otherwise a dash for a genuinely-N/A cell.
function NumCell({ computed, published }: { computed: number | null; published?: number | undefined }) {
  if (computed !== null && computed !== undefined) {
    return <TableCell className="text-right tabular-nums">{fmt(computed)}</TableCell>;
  }
  if (published !== undefined) {
    return (
      <TableCell className="text-right tabular-nums italic text-muted-foreground">
        {fmt(published)}
      </TableCell>
    );
  }
  return <TableCell className="text-right tabular-nums">{DASH}</TableCell>;
}

const ROW_LABELS: Record<string, { en: string; fr: string }> = {
  "2x2-tl": { en: "2×2 — top-left corner", fr: "2×2 — coin haut-gauche" },
  "2x2-tr": { en: "2×2 — top-right corner", fr: "2×2 — coin haut-droit" },
  "2x2-br": { en: "2×2 — bottom-right corner", fr: "2×2 — coin bas-droit" },
  "2x2-bl": { en: "2×2 — bottom-left corner", fr: "2×2 — coin bas-gauche" },
  "2x2-side": { en: "2×2 — on a side", fr: "2×2 — sur un bord" },
  "2x2-middle": { en: "2×2 — in the middle", fr: "2×2 — au milieu" },
  "3x3-tl": { en: "3×3 — top-left corner", fr: "3×3 — coin haut-gauche" },
  "3x3-tr": { en: "3×3 — top-right corner", fr: "3×3 — coin haut-droit" },
  "3x3-br": { en: "3×3 — bottom-right corner", fr: "3×3 — coin bas-droit" },
  "3x3-bl": { en: "3×3 — bottom-left corner", fr: "3×3 — coin bas-gauche" },
  "3x3-side": { en: "3×3 — on a side", fr: "3×3 — sur un bord" },
  "3x3-middle": { en: "3×3 — in the middle", fr: "3×3 — au milieu" },
  "3x3-encircle": { en: "3×3 — encircling the centre clue", fr: "3×3 — autour de l'indice central" },
  "4x4-tl": { en: "4×4 — top-left corner", fr: "4×4 — coin haut-gauche" },
  "4x4-tr": { en: "4×4 — top-right corner", fr: "4×4 — coin haut-droit" },
  "4x4-br": { en: "4×4 — bottom-right corner", fr: "4×4 — coin bas-droit" },
  "4x4-bl": { en: "4×4 — bottom-left corner", fr: "4×4 — coin bas-gauche" },
  "4x4-side": { en: "4×4 — on a side", fr: "4×4 — sur un bord" },
};

const T = {
  en: {
    computedFrom: "Computed from",
    article: "the open research note + reproducible Rust generator",
    onGithub: "on GitHub",
    colBlock: "Block",
    colVide: "Empty",
    colFixe: "Fixed",
    colFixe4: "Fixed + 4 hints",
    legendTitle: "What the columns mean",
    legend: [
      ["Empty", "Every way to fill the block with distinct official pieces of the right class, edges matching."],
      ["Fixed", "Same, but the centre clue piece is removed from the pool (it is fixed at the board centre)."],
      ["Fixed + 4 hints", "All five clue pieces removed from the pool; any clue inside the block is pinned at its official spot."],
      ["c0 – c3", "Corner blocks only: of the Fixed + 4 hints fillings, how many place each of the four corner pieces in the corner. They sum to the Fixed + 4 hints count."],
    ],
    italicNote:
      "Values in italics are sylvogel's published figures, reproduced here for completeness: their counts are too large to enumerate exactly in seconds (tens of billions to tens of trillions of fillings), so this site does not independently recompute them. Every upright value is computed here from the official piece set.",
    rules:
      "Counting rules: each cell uses a piece of the class matching its border-facing sides (corner / edge / interior); board-rim edges are grey; block-boundary edges facing the interior are non-grey; shared internal edges match; pieces are distinct within the block.",
  },
  fr: {
    computedFrom: "Calculé à partir de",
    article: "la note de recherche ouverte + le générateur Rust reproductible",
    onGithub: "sur GitHub",
    colBlock: "Bloc",
    colVide: "Vide",
    colFixe: "Fixe",
    colFixe4: "Fixe + 4 indices",
    legendTitle: "Que signifient les colonnes",
    legend: [
      ["Vide", "Toutes les façons de remplir le bloc avec des pièces officielles distinctes de la bonne classe, bords concordants."],
      ["Fixe", "Idem, mais la pièce-indice centrale est retirée du lot (elle est fixée au centre du plateau)."],
      ["Fixe + 4 indices", "Les cinq pièces-indices retirées du lot ; tout indice à l'intérieur du bloc est épinglé à sa place officielle."],
      ["c0 – c3", "Blocs de coin uniquement : parmi les remplissages Fixe + 4 indices, combien placent chacune des quatre pièces de coin dans le coin. Leur somme égale le total Fixe + 4 indices."],
    ],
    italicNote:
      "Les valeurs en italique sont les chiffres publiés par sylvogel, repris ici par souci d'exhaustivité : leurs comptages sont trop grands pour être énumérés exactement en quelques secondes (de dizaines de milliards à des dizaines de billions de remplissages), donc ce site ne les recalcule pas lui-même. Toute valeur en romain est calculée ici à partir du jeu de pièces officiel.",
    rules:
      "Règles de comptage : chaque case utilise une pièce de la classe correspondant à ses côtés bordant le cadre (coin / bord / intérieur) ; les bords du cadre sont gris ; les bords de la frontière du bloc tournés vers l'intérieur sont non gris ; les bords internes partagés concordent ; les pièces sont distinctes dans le bloc.",
  },
};

export function ReferenceTableView() {
  const t = useT(T);
  const lang = useT({ en: { l: "en" as const }, fr: { l: "fr" as const } }).l;

  return (
    <div className="space-y-10">
      <p className="text-sm text-muted-foreground">
        {t.computedFrom}{" "}
        <a
          href={ARTICLE_URL}
          target="_blank"
          rel="noreferrer"
          className="font-medium underline underline-offset-2 hover:text-foreground"
        >
          {t.article}
        </a>{" "}
        {t.onGithub}.
      </p>

      <section className="space-y-3">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t.colBlock}</TableHead>
              <TableHead className="text-right">{t.colVide}</TableHead>
              <TableHead className="text-right">{t.colFixe}</TableHead>
              <TableHead className="text-right">{t.colFixe4}</TableHead>
              <TableHead className="text-right">c0</TableHead>
              <TableHead className="text-right">c1</TableHead>
              <TableHead className="text-right">c2</TableHead>
              <TableHead className="text-right">c3</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {ROWS.map((r) => {
              const label = ROW_LABELS[r.key]?.[lang] ?? r.key;
              const pub = PUBLISHED[r.key] ?? {};
              return (
                <TableRow key={r.key}>
                  <TableCell className="font-medium whitespace-nowrap">{label}</TableCell>
                  <NumCell computed={r.vide} published={pub.vide} />
                  <NumCell computed={r.fixe} published={pub.fixe} />
                  <NumCell computed={r.fixe4} published={pub.fixe4} />
                  <NumCell computed={r.cN ? r.cN[0] : null} />
                  <NumCell computed={r.cN ? r.cN[1] : null} />
                  <NumCell computed={r.cN ? r.cN[2] : null} />
                  <NumCell computed={r.cN ? r.cN[3] : null} />
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
        <p className="text-xs text-muted-foreground italic">{t.italicNote}</p>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.legendTitle}</h2>
        <dl className="space-y-2 text-sm">
          {t.legend.map(([term, desc]) => (
            <div key={term} className="grid grid-cols-[8rem_1fr] gap-3">
              <dt className="font-medium">{term}</dt>
              <dd className="text-muted-foreground">{desc}</dd>
            </div>
          ))}
        </dl>
        <p className="pt-2 text-xs text-muted-foreground">{t.rules}</p>
      </section>
    </div>
  );
}
