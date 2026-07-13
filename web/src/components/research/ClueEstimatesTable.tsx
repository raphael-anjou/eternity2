import { useLang } from "@/i18n";

// Brendan Owen's own complex-theory tabulation, transcribed verbatim from the
// community's "Backtracker estimates" database on groups.io (table 19128). It
// applies the estimate this page describes to the four clue puzzles and the
// real E2 board, side by side. The value here is calibration: on the small
// clue puzzles the tree was searched exhaustively, so the estimated and
// *empirical* counts sit next to each other and you can see the model land
// within about a factor of two, exactly as the page claims. It also records
// the best-known fill order per puzzle, which differs from puzzle to puzzle,
// concrete evidence that the fill order is a first-class choice.
//
// Data is verbatim from the source; only the row labels are localized. The
// long numbers are kept in the community's own notation (scientific for the
// astronomically large, grouped decimals for the exactly counted).

type Col = "clue1" | "clue2" | "clue3" | "clue4" | "e2one" | "e2five";

const COLS: { key: Col; label: string }[] = [
  { key: "clue1", label: "Clue #1" },
  { key: "clue2", label: "Clue #2" },
  { key: "clue3", label: "Clue #3" },
  { key: "clue4", label: "Clue #4" },
  { key: "e2one", label: "E2 · 1 hint" },
  { key: "e2five", label: "E2 · 5 hints" },
];

// A curated, meaningful subset of the 13 source rows (the rest are symmetry
// bookkeeping). `empirical` rows are true exhaustive counts, only known for
// the small puzzles; blanks are genuinely blank in the source.
type Row = {
  key: string;
  en: string;
  fr: string;
  vals: Partial<Record<Col, string>>;
  empirical?: boolean;
};

const ROWS: Row[] = [
  {
    key: "sol-est",
    en: "Estimated solutions",
    fr: "Solutions estimées",
    vals: {
      clue1: "2.6e11",
      clue2: "7.6e36",
      clue3: "8.2e8",
      clue4: "4.9e34",
      e2one: "14,702",
      e2five: "1",
    },
  },
  {
    key: "sol-emp",
    en: "Solutions, actually counted",
    fr: "Solutions, comptées réellement",
    empirical: true,
    vals: {
      clue1: "115,071,633,408",
      clue3: "2,195,647,488",
    },
  },
  {
    key: "nodes-est",
    en: "Estimated search-tree nodes",
    fr: "Nœuds estimés de l'arbre",
    vals: {
      clue1: "1.3e13",
      clue2: "2.0e40",
      clue3: "1.1e11",
      clue4: "8.4e37",
      e2one: "1.4e47",
      e2five: "3.1e40",
    },
  },
  {
    key: "nodes-emp",
    en: "Search-tree nodes, actually counted",
    fr: "Nœuds de l'arbre, comptés réellement",
    empirical: true,
    vals: {
      clue1: "1.0e13",
      clue3: "2.2e11",
    },
  },
  {
    key: "order",
    en: "Best known fill order",
    fr: "Meilleur ordre de remplissage connu",
    vals: {
      clue1: "spiral-in",
      clue2: "border first, then vertical scans",
      clue3: "spiral-in",
      clue4: "border first, then vertical scans",
      e2one: "horizontal scans from bottom-left",
      e2five: "horizontal scans from bottom-left",
    },
  },
];

const T = {
  en: {
    itemHdr: "quantity",
    empirical: "counted",
    caption:
      "Brendan Owen's complex-theory numbers for the four clue puzzles and the real E2 board. On the small clue puzzles the whole tree was searched, so the estimate can be checked against the true count: it lands within a factor of two. The best fill order differs per puzzle, the order is a choice, not a given.",
    footnote:
      'Transcribed verbatim from the community "Backtracker estimates" table on groups.io. Blank cells are blank in the source (the clue puzzles too large to count exhaustively, and the un-run E2 columns).',
    source: "Source: groups.io/g/eternity2 · Databases · Backtracker estimates",
  },
  fr: {
    itemHdr: "quantité",
    empirical: "compté",
    caption:
      "Les chiffres de théorie complexe de Brendan Owen pour les quatre puzzles à indices et le vrai plateau E2. Sur les petits puzzles à indices, l'arbre entier a été parcouru : l'estimation se vérifie donc contre le compte exact, et tombe à un facteur deux près. Le meilleur ordre de remplissage change d'un puzzle à l'autre, c'est un choix, pas une donnée.",
    footnote:
      "Transcrit tel quel depuis la table communautaire « Backtracker estimates » sur groups.io. Les cellules vides le sont dans la source (puzzles à indices trop grands pour un comptage exhaustif, et colonnes E2 non parcourues).",
    source: "Source : groups.io/g/eternity2 · Databases · Backtracker estimates",
  },
};

export function ClueEstimatesTable() {
  const { lang } = useLang();
  const t = T[lang];
  return (
    <figure className="not-prose my-6">
      <div className="overflow-x-auto rounded-lg border">
        <table className="w-full border-collapse text-sm">
          <thead>
            <tr className="border-b bg-muted/50 text-left">
              <th className="px-3 py-2 font-medium">{t.itemHdr}</th>
              {COLS.map((c) => (
                <th key={c.key} className="whitespace-nowrap px-3 py-2 text-right font-medium">
                  {c.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {ROWS.map((row) => (
              <tr key={row.key} className="border-b last:border-0 align-top">
                <th
                  scope="row"
                  className="px-3 py-2 text-left font-normal text-muted-foreground"
                >
                  {row[lang]}
                  {row.empirical && (
                    <span className="ml-1.5 rounded bg-emerald-500/15 px-1 py-0.5 text-[10px] font-medium uppercase tracking-wide text-emerald-700 dark:text-emerald-300">
                      {t.empirical}
                    </span>
                  )}
                </th>
                {COLS.map((c) => (
                  <td
                    key={c.key}
                    className="whitespace-nowrap px-3 py-2 text-right tabular-nums"
                  >
                    {row.vals[c.key] ?? <span className="text-muted-foreground/40">·</span>}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <figcaption className="mt-2 space-y-1 text-xs text-muted-foreground">
        <p>{t.caption}</p>
        <p>{t.footnote}</p>
        <p className="italic">{t.source}</p>
      </figcaption>
    </figure>
  );
}
