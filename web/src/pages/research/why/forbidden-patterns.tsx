import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { BoardSvg } from "@/components/board/BoardSvg";
import type { Edges } from "@/lib/bucas";
import { ForbiddenPatchLab } from "@/components/research/ForbiddenPatchLab";
import { Math, MathBlock } from "@/components/research/Math";
import data from "@/data/forbidden-patterns.json";

// The exact, reproducible feasibility counts, computed by the research topic's
// compute crate. Mirrors research/topics/forbidden-patterns/results/feasibility.json.
const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/forbidden-patterns";

type Shape = {
  shape: string;
  cells: number;
  placements: number;
  feasible: number;
  forbidden: number;
  forbiddenPct: number;
};
const SHAPES = data.shapes as Shape[];

const fmt = (n: number) => n.toLocaleString("en-US");

// Two hand-built 2x2 illustrations using a few motif colors (URDL per cell).
// The left square matches on all four inner edges; the right one cannot be made
// to match — its inner vertical edge is a conflict. These are illustrations of
// the idea, not specific official pieces.
const FEASIBLE: (Edges | null)[] = [
  [7, 3, 12, 5], // TL: right=3, down=12
  [7, 9, 11, 3], // TR: left=3 matches TL.right
  [12, 4, 8, 5], // BL: up=12 matches TL.down
  [11, 6, 8, 4], // BR: up=11 matches TR.down(11), left=4 matches BL.right(4)
];
const FORBIDDEN: (Edges | null)[] = [
  [7, 3, 12, 5], // TL: right=3
  [7, 9, 11, 2], // TR: left=2 does NOT match TL.right=3 → conflict
  [12, 4, 8, 5],
  [11, 6, 8, 4],
];
// Mark the broken inner vertical edge on the forbidden example: TL right side.
const FORBIDDEN_CONFLICTS: [number, number][] = [
  [0, 1], // pos 0 (TL), direction 1 (right)
  [1, 3], // pos 1 (TR), direction 3 (left)
];

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Forbidden patterns",
    lede: "Almost every small patch of pieces you could build is impossible. For a 2x2 square, 99.72% of the ways to place four pieces can never be made to match.",
    p1: "Eternity II has 256 square tiles, each with a color on all four sides. 196 of them are interior pieces — the ones with no grey border edge, so the only ones that ever sit inside the board. Take a few of those, drop them into a small shape, and rotate them however you like. Most of the time the colors simply won't line up, no matter what you try.",
    p2: "The bigger the shape, the worse it gets. Two pieces side by side fail to fit about 39% of the time. Add a third in an L and you're stuck 83% of the time. Close up a 2x2 square and 99.72% of all placements are dead on arrival — only about 1 in 358 works.",
    illusTitle: "One that fits, one that can't",
    illusFeasible: "Fits: every inner edge shares a color.",
    illusForbidden: "Forbidden: the two middle pieces have no shared color, and no rotation fixes it.",
    labTitle: "Try it: draw four pieces",
    mathTitle: "Where 99.72% comes from",
    math1: "A rough estimate explains why the number is so high. Two adjacent pieces share one edge. Each interior piece carries colors from the 17-color interior palette, so a random pair of half-edges matches with probability roughly",
    math2: "A 2x2 square has four internal edges to satisfy at once. Rotations give each piece four chances, but the four edges are coupled, so a back-of-the-envelope independence estimate puts the chance all four match at very roughly",
    math3: "which is already under one percent. The exact exhaustive count lands at 0.28% feasible, that is 99.72% forbidden. The estimate is crude because the edges aren't independent and the colors aren't uniform, but it gets the order of magnitude right and shows why closing a square is so much harder than placing a single pair.",
    tableTitle: "The exact counts",
    tableIntro: "Every distinct-piece placement of each shape, checked exhaustively — no sampling.",
    colShape: "Shape",
    colPlacements: "Placements",
    colForbidden: "Forbidden",
    colPct: "Forbidden %",
    shapeNames: {
      "2-horizontal": "Two side by side",
      "2-vertical": "Two stacked",
      "L-tromino": "L of three",
      "2x2": "2x2 square",
    } as Record<string, string>,
    whyTitle: "Why it matters",
    why1: "A finished, correct board has zero forbidden patches — by definition, everything matches. So counting the forbidden patches in a board tells you roughly how far it is from a real solution, even when two boards have the same number of matched edges. Weak boards are full of forbidden squares; the best boards ever found have only a couple of dozen left.",
    why2: "It also shows, from another angle, why the puzzle shrugs off clever local fixes. When 99.72% of small squares are impossible, the pieces that do fit together are rare and specific. There's almost no room to shuffle things around without breaking something — the good arrangements are scarce and rigid.",
    reproTitle: "Reproduce it",
    reproBody: "Computed exactly from the official set; reproduces identically every run (about twenty seconds).",
    sourceLink: "Article, source and results on GitHub",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Motifs interdits",
    lede: "Presque tout petit groupe de pièces que vous pourriez assembler est impossible. Pour un carré 2x2, 99,72 % des façons de placer quatre pièces ne peuvent jamais s'accorder.",
    p1: "Eternity II compte 256 tuiles carrées, chacune colorée sur ses quatre côtés. 196 sont des pièces intérieures — celles sans bord gris, les seules à se placer à l'intérieur du plateau. Prenez-en quelques-unes, posez-les dans une petite forme et tournez-les comme bon vous semble. La plupart du temps, les couleurs ne s'alignent tout simplement pas.",
    p2: "Plus la forme est grande, pire c'est. Deux pièces côte à côte ne s'accordent pas environ 39 % du temps. Ajoutez-en une troisième en L et vous êtes bloqué 83 % du temps. Fermez un carré 2x2 et 99,72 % des placements sont perdus d'avance — environ 1 sur 358 fonctionne.",
    illusTitle: "Un qui marche, un qui ne peut pas",
    illusFeasible: "Marche : chaque bord intérieur partage une couleur.",
    illusForbidden: "Interdit : les deux pièces du milieu n'ont aucune couleur commune, et aucune rotation n'y change rien.",
    labTitle: "Essayez : tirez quatre pièces",
    mathTitle: "D'où vient le 99,72 %",
    math1: "Une estimation grossière explique pourquoi le nombre est si élevé. Deux pièces voisines partagent un bord. Chaque pièce intérieure porte des couleurs de la palette intérieure de 17 couleurs, donc une paire de demi-bords au hasard s'accorde avec une probabilité d'environ",
    math2: "Un carré 2x2 a quatre bords internes à satisfaire en même temps. Les rotations donnent quatre chances à chaque pièce, mais les quatre bords sont couplés ; une estimation d'indépendance au dos de l'enveloppe place la chance que les quatre s'accordent à très grossièrement",
    math3: "soit déjà moins d'un pour cent. Le comptage exhaustif exact tombe à 0,28 % d'accords possibles, c'est-à-dire 99,72 % d'interdits. L'estimation est grossière car les bords ne sont pas indépendants et les couleurs ne sont pas uniformes, mais elle donne le bon ordre de grandeur et montre pourquoi fermer un carré est bien plus dur que poser une seule paire.",
    tableTitle: "Les comptages exacts",
    tableIntro: "Chaque placement de pièces distinctes pour chaque forme, vérifié de façon exhaustive — sans échantillonnage.",
    colShape: "Forme",
    colPlacements: "Placements",
    colForbidden: "Interdits",
    colPct: "% interdits",
    shapeNames: {
      "2-horizontal": "Deux côte à côte",
      "2-vertical": "Deux empilées",
      "L-tromino": "L de trois",
      "2x2": "Carré 2x2",
    } as Record<string, string>,
    whyTitle: "Pourquoi c'est important",
    why1: "Un plateau terminé et correct n'a aucun motif interdit — par définition, tout s'accorde. Compter les motifs interdits d'un plateau indique donc à peu près sa distance à une vraie solution, même quand deux plateaux ont le même nombre de bords appariés. Les plateaux faibles en regorgent ; les meilleurs jamais trouvés n'en ont plus qu'une vingtaine.",
    why2: "Cela montre aussi, sous un autre angle, pourquoi le puzzle résiste aux retouches locales malignes. Quand 99,72 % des petits carrés sont impossibles, les pièces qui s'emboîtent sont rares et précises. Il n'y a presque aucune marge pour réarranger sans casser quelque chose — les bons agencements sont rares et rigides.",
    reproTitle: "Le reproduire",
    reproBody: "Calculé exactement à partir du jeu officiel ; se reproduit à l'identique à chaque exécution (une vingtaine de secondes).",
    sourceLink: "Article, code source et résultats sur GitHub",
  },
};

export default function ForbiddenPatterns() {
  const t = useT(T);
  return (
    <div className="space-y-10">
      <div>
        <LocalizedLink
          to="/research/why"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-3 max-w-3xl text-lg text-muted-foreground">{t.lede}</p>
      </div>

      <section className="max-w-3xl space-y-4 text-sm leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
        <p>{t.p2}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.illusTitle}</h2>
        <div className="grid max-w-2xl gap-6 sm:grid-cols-2">
          <figure className="space-y-2">
            <div className="rounded-lg border p-2">
              <BoardSvg width={2} height={2} cells={FEASIBLE} />
            </div>
            <figcaption className="text-xs text-muted-foreground">{t.illusFeasible}</figcaption>
          </figure>
          <figure className="space-y-2">
            <div className="rounded-lg border p-2">
              <BoardSvg
                width={2}
                height={2}
                cells={FORBIDDEN}
                conflicts={FORBIDDEN_CONFLICTS}
              />
            </div>
            <figcaption className="text-xs text-muted-foreground">{t.illusForbidden}</figcaption>
          </figure>
        </div>
      </section>

      <section className="space-y-6">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.labTitle}</h2>
        <ForbiddenPatchLab />
      </section>

      <section className="mx-auto max-w-2xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.mathTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.math1}</p>
        <MathBlock>{String.raw`\Pr[\text{one edge matches}] \;\approx\; \frac{1}{17} \;\approx\; 6\%.`}</MathBlock>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.math2}</p>
        <MathBlock>{String.raw`\Pr[\text{2}\times\text{2 feasible}] \;\sim\; 1 - \left(1 - \tfrac{1}{17}\right)^{\!c} \ \text{per rotation budget} \;\Rightarrow\; \lesssim 1\%,`}</MathBlock>
        <p className="text-sm leading-relaxed text-muted-foreground">
          {t.math3}{" "}
          <Math>{String.raw`(\,0.28\% = \tfrac{3{,}993{,}696}{1{,}431{,}033{,}240}\,)`}</Math>
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.tableTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.tableIntro}</p>
        <div className="max-w-2xl overflow-x-auto rounded-lg border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t.colShape}</TableHead>
                <TableHead className="text-right">{t.colPlacements}</TableHead>
                <TableHead className="text-right">{t.colForbidden}</TableHead>
                <TableHead className="text-right">{t.colPct}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {SHAPES.map((s) => (
                <TableRow key={s.shape}>
                  <TableCell className="font-medium">
                    {t.shapeNames[s.shape] ?? s.shape}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">{fmt(s.placements)}</TableCell>
                  <TableCell className="text-right tabular-nums">{fmt(s.forbidden)}</TableCell>
                  <TableCell className="text-right font-semibold tabular-nums">
                    {s.forbiddenPct.toFixed(2)}%
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      </section>

      <section className="max-w-3xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.whyTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.why1}</p>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.why2}</p>
      </section>

      <section className="max-w-3xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md bg-background p-3 text-xs">
          <code>{`cd research/topics/forbidden-patterns/compute
cargo run --release > ../results/feasibility.json`}</code>
        </pre>
        <a
          href={ARTICLE_URL}
          target="_blank"
          rel="noreferrer"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.sourceLink}
        </a>
      </section>

      <RelatedRail path="/research/why/forbidden-patterns" />
    </div>
  );
}

export const meta = pageMeta("forbidden-patterns");
