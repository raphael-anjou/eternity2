import { pageMeta } from "@/seo";
import {
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { Math as MathInline, MathBlock } from "@/components/research/Math";
import data from "@/data/entropy-area-law.json";

const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/entropy-area-law";

const hByWidth = data.hByWidth;

// Area-law curve rho(n) = exp(-alpha n^2) for the chart.
const alpha = data.areaLawExponent;
const rhoCurve = Array.from({ length: 13 }, (_, i) => {
  const n = i + 1;
  return { cells: n * n, rho: Math.exp(-alpha * n * n) };
});

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Entropy and the area law",
    lede: "Eternity II has two rules: edges must match, and each piece is used once. The first is generous. All the hardness lives in the second.",
    p1: "Forget the use-each-piece-once rule for a moment and treat the 196 interior pieces as reusable tiles. Count the valid all-matched patches and they grow exponentially with size: there's no shortage of locally-valid ways to tile. The matching grammar is rich, not restrictive.",
    p2: "We can measure that richness exactly. For a strip of width n, the growth rate per cell is an entropy density h(n), and the sequence decreases toward the true two-dimensional value as the strip widens.",
    chartHTitle: "The grammar's entropy, width by width",
    chartHNote: "Decreasing toward the limit ≈ 0.67. The first two points are computed exactly here; the rest from an offline sweep.",
    hAxis: "entropy density h(n)",
    widthAxis: "strip width n",
    areaTitle: "Then the area law bites",
    area1: "Now put the use-each-piece-once rule back, and ask how often a random color-valid n×n patch actually uses distinct pieces. Call it ρ(n). It collapses, and crucially it collapses in the area, not the perimeter:",
    area2: "An area-law decay is brutal because area grows quadratically. The fraction of realizable patches drops below one in a thousand at around eighty cells.",
    chartRhoTitle: "Fraction of patches realizable with distinct pieces",
    rhoAxis: "ρ (distinct-realizable fraction)",
    cellsAxis: "patch size (cells)",
    impactTitle: "Why it matters",
    impact: "Eighty cells is not arbitrary. It's the size of the smallest moves that separate the best known boards. The matching grammar stays rich up to about that scale, then the distinctness rule collapses it. So the wall isn't in the part that looks hard, matching colors; it's in the quiet rule that each piece is used once, whose cost grows with area, on a board just large enough for it to bite.",
    theoremTitle: "The theorem, briefly",
    th1: "The per-width entropy has a well-defined limit. Joining an n₁-wide and an n₂-wide strip side by side only adds a seam constraint, so the eigenvalues satisfy",
    th2: "Taking logs makes log λₙ subadditive, and Fekete's lemma gives the limit as an infimum, which is exactly why the table above decreases:",
    th3: "The upper bound is the purely horizontal rate: ignoring vertical constraints only adds patches, so",
    th4: "with λ_H = 46.18 the spectral radius of the horizontal color-compatibility matrix. Positivity holds because the grammar supports exponentially many chains, so the density is strictly between zero and 1.6645, measured near 0.67.",
    reproTitle: "Reproduce it",
    reproBody: "h(1), h(2) and λ_H are recomputed exactly from the official set; h(3), h(4) and the limit come from a heavier offline sweep, labelled as such.",
    sourceLink: "Article, source and results on GitHub",
    related: "See why basin-hopping is impossible",
    busy: "Loading chart…",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Entropie et loi d'aire",
    lede: "Eternity II a deux règles : les bords doivent s'accorder, et chaque pièce sert une fois. La première est généreuse. Toute la difficulté vit dans la seconde.",
    p1: "Oubliez un instant la règle « chaque pièce une fois » et traitez les 196 pièces intérieures comme des tuiles réutilisables. Comptez les patches valides entièrement accordés : ils croissent exponentiellement avec la taille. Il n'y a pas pénurie de façons localement valides de paver. La grammaire d'accord est riche, pas restrictive.",
    p2: "On peut mesurer cette richesse exactement. Pour une bande de largeur n, le taux de croissance par cellule est une densité d'entropie h(n), et la suite décroît vers la vraie valeur bidimensionnelle quand la bande s'élargit.",
    chartHTitle: "L'entropie de la grammaire, largeur par largeur",
    chartHNote: "Décroît vers la limite ≈ 0,67. Les deux premiers points sont calculés exactement ici ; les autres viennent d'un balayage hors ligne.",
    hAxis: "densité d'entropie h(n)",
    widthAxis: "largeur de bande n",
    areaTitle: "Puis la loi d'aire frappe",
    area1: "Remettez la règle « chaque pièce une fois » et demandez à quelle fréquence un patch n×n aléatoire valide en couleurs utilise vraiment des pièces distinctes. Appelez cela ρ(n). Cela s'effondre, et surtout cela s'effondre en l'aire, pas le périmètre :",
    area2: "Une décroissance en loi d'aire est brutale car l'aire croît de façon quadratique. La fraction de patches réalisables tombe sous un pour mille vers quatre-vingts cellules.",
    chartRhoTitle: "Fraction de patches réalisables avec des pièces distinctes",
    rhoAxis: "ρ (fraction réalisable distincte)",
    cellsAxis: "taille du patch (cellules)",
    impactTitle: "Pourquoi c'est important",
    impact: "Quatre-vingts cellules n'est pas arbitraire. C'est la taille des plus petits mouvements qui séparent les meilleurs plateaux connus. La grammaire d'accord reste riche jusqu'à cette échelle environ, puis la règle de distinction l'effondre. Le mur n'est donc pas dans la partie qui semble dure, accorder les couleurs ; il est dans la règle discrète selon laquelle chaque pièce sert une fois, dont le coût croît avec l'aire, sur un plateau juste assez grand pour qu'il morde.",
    theoremTitle: "Le théorème, en bref",
    th1: "L'entropie par largeur a une limite bien définie. Joindre une bande de largeur n₁ et une de largeur n₂ côte à côte n'ajoute qu'une contrainte de couture, donc les valeurs propres vérifient",
    th2: "En passant au log, log λₙ devient sous-additive, et le lemme de Fekete donne la limite comme un infimum, ce qui explique exactement pourquoi le tableau ci-dessus décroît :",
    th3: "La borne supérieure est le taux purement horizontal : ignorer les contraintes verticales ne fait qu'ajouter des patches, donc",
    th4: "avec λ_H = 46,18 le rayon spectral de la matrice de compatibilité horizontale des couleurs. La positivité tient car la grammaire admet exponentiellement de chaînes ; la densité est donc strictement entre zéro et 1,6645, mesurée près de 0,67.",
    reproTitle: "Le reproduire",
    reproBody: "h(1), h(2) et λ_H sont recalculés exactement à partir du jeu officiel ; h(3), h(4) et la limite viennent d'un balayage hors ligne plus lourd, étiquetés comme tels.",
    sourceLink: "Article, code source et résultats sur GitHub",
    related: "Voir pourquoi sauter de bassin en bassin est impossible",
    busy: "Chargement du graphique…",
  },
};

export default function EntropyAreaLaw() {
  const t = useT(T);
  const isClient = useIsClient();
  return (
    <div className="space-y-12">
      <div className="mx-auto max-w-3xl text-center">
        <LocalizedLink
          to="/research/why"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-3 text-4xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-4 text-xl text-muted-foreground">{t.lede}</p>
      </div>

      <section className="mx-auto max-w-2xl space-y-4 text-base leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
        <p>{t.p2}</p>
      </section>

      <section className="space-y-3">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.chartHTitle}</h2>
        <div className="mx-auto h-72 max-w-2xl rounded-lg border p-2">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
              <LineChart data={hByWidth} margin={{ top: 10, right: 16, bottom: 18, left: 8 }}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="width"
                  type="number"
                  domain={[1, 4]}
                  tickCount={4}
                  label={{ value: t.widthAxis, position: "insideBottom", offset: -8, fontSize: 12 }}
                  fontSize={12}
                />
                <YAxis domain={[0.5, 1.8]} width={40} fontSize={12} />
                <Tooltip
                  formatter={(v) => [Number(v).toFixed(4), t.hAxis]}
                  labelFormatter={(w) => `${t.widthAxis} = ${String(w)}`}
                />
                <Line
                  type="monotone"
                  dataKey="h"
                  stroke="#38bdf8"
                  strokeWidth={2}
                  dot={{ r: 3 }}
                  isAnimationActive={false}
                />
              </LineChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              {t.busy}
            </div>
          )}
        </div>
        <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.chartHNote}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.areaTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.area1}</p>
        <MathBlock>{String.raw`\rho(n) \;\approx\; \exp(-\alpha\, n^2), \qquad \alpha \approx 0.085.`}</MathBlock>
        <p className="text-base leading-relaxed text-muted-foreground">{t.area2}</p>
      </section>

      <section className="space-y-3">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.chartRhoTitle}</h2>
        <div className="mx-auto h-72 max-w-2xl rounded-lg border p-2">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%" minWidth={0} minHeight={0}>
              <LineChart data={rhoCurve} margin={{ top: 10, right: 16, bottom: 18, left: 8 }}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="cells"
                  type="number"
                  domain={[0, 170]}
                  tickCount={7}
                  label={{ value: t.cellsAxis, position: "insideBottom", offset: -8, fontSize: 12 }}
                  fontSize={12}
                />
                <YAxis
                  scale="log"
                  domain={[1e-7, 1]}
                  allowDataOverflow
                  tickFormatter={(n) => Number(n).toExponential(0)}
                  width={52}
                  fontSize={11}
                />
                <Tooltip
                  formatter={(v) => [Number(v).toExponential(2), "ρ"]}
                  labelFormatter={(c) => `${String(c)} ${t.cellsAxis}`}
                />
                <Line
                  type="monotone"
                  dataKey="rho"
                  stroke="#f59e0b"
                  strokeWidth={2}
                  dot={false}
                  isAnimationActive={false}
                />
              </LineChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              {t.busy}
            </div>
          )}
        </div>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.impactTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.impact}</p>
        <LocalizedLink
          to="/research/why/sigma-cycles"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.related}
        </LocalizedLink>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border p-6">
        <h2 className="text-xl font-semibold tracking-tight">{t.theoremTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.th1}</p>
        <MathBlock>{String.raw`\lambda_{n_1+n_2} \;\le\; \lambda_{n_1}\,\lambda_{n_2}.`}</MathBlock>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.th2}</p>
        <MathBlock>{String.raw`h_\infty \;=\; \lim_{n\to\infty}\frac{\log_{10}\lambda_n}{n} \;=\; \inf_n \frac{\log_{10}\lambda_n}{n}.`}</MathBlock>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.th3}</p>
        <MathBlock>{String.raw`0 \;<\; h_\infty \;\le\; \log_{10}\lambda_H \;=\; 1.6645,`}</MathBlock>
        <p className="text-sm leading-relaxed text-muted-foreground">
          {t.th4} <MathInline>{String.raw`(\,\lambda_H = 46.18\,)`}</MathInline>
        </p>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md bg-background p-3 text-xs">
          <code>{`cd research/topics/entropy-area-law/compute
cargo run --release > ../results/entropy.json`}</code>
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

      <RelatedRail path="/research/why/entropy-area-law" />
    </div>
  );
}

export const meta = pageMeta("entropy-area-law");
