import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { MotifSwatch } from "@/components/board/MotifSwatch";
import { colorToLetter } from "@/lib/motifs";
import { PhaseTransitionLab } from "@/components/research/PhaseTransitionLab";
import data from "@/data/phase-transition.json";

const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/phase-transition";

const FRAME_COLORS = data.frameOnlyColors;
const INTERIOR_COLORS = data.interiorColors;

function Swatches({ colors }: { colors: number[] }) {
  return (
    <div className="flex flex-wrap gap-3">
      {colors.map((c) => (
        <div key={c} className="flex flex-col items-center gap-0.5">
          <MotifSwatch color={c} width={44} />
          <span className="font-mono text-xs text-muted-foreground">
            &apos;{colorToLetter(c)}&apos;
          </span>
        </div>
      ))}
    </div>
  );
}

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Tuned to the hardness peak",
    lede: "Eternity II uses 22 colors. They split 17 interior to 5 frame-only, and that 17 is exactly where this kind of puzzle is hardest to solve.",
    p1: "Hard search problems have a difficulty knob. Loosen it and there are many solutions, so a search trips over one fast. Tighten it and there are none, which is often easy to prove. In between sits a narrow band where solutions are scarce but real, and that's where search blows up. People call it a phase transition, like water freezing.",
    p2: "For edge-matching puzzles the knob is the number of colors. Too few and pieces fit together countless ways; too many and they barely fit at all. The published analysis puts the peak at around 17 interior colors, the setting where a puzzle this size has about one solution. Eternity II uses 17.",
    liveTitle: "The difficulty wall",
    splitTitle: "The split, straight from the pieces",
    splitIntro: "Sorting the official set's colors by where they appear shows the design plainly.",
    frameLabel: "5 frame-only colors",
    frameNote: "These appear only on border and corner pieces, never in the interior. They are the rare colors, kept to the edge.",
    interiorLabel: "17 interior colors",
    interiorNote: "The palette of the inside of the board, where almost all the matching happens.",
    breakdownTitle: "The set in numbers",
    rows: [
      ["Corner pieces", data.pieces.corner],
      ["Edge pieces", data.pieces.edge],
      ["Interior pieces", data.pieces.interior],
      ["Interior colors", data.interiorColorCount],
      ["Frame-only colors", data.frameOnlyColorCount],
    ] as [string, number][],
    whyTitle: "Why it matters",
    why1: "This is the clearest single sign that Eternity II was made hard on purpose. Board size, piece count and color split all aim at the same target: a puzzle with about one solution, placed at the worst possible spot for any search to find it. The difficulty was chosen, the way a good exam is neither trivial nor impossible.",
    why2: "How do we know the peak is real and not just a story? Two ways meet here. The published analysis derives it: for framed edge-matching puzzles, the color count where you'd expect about one solution falls near 17, and that is the hardest setting to search. And you can watch a piece of it yourself in the demo above: build real puzzles, count the work, and see it explode when colors are scarce. The impact is concrete. It means the gap to a solution isn't a tuning problem you can grind away with a faster machine; the puzzle was placed where search is worst on purpose, so beating it needs a genuinely better idea, not just more effort.",
    seeAlso: "See the difficulty measured live on the Algorithms page",
    reproTitle: "Reproduce it",
    reproBody: "The color split is read straight from the official set. Instant and identical every run.",
    sourceLink: "Article, source and results on GitHub",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Calé sur le pic de difficulté",
    lede: "Eternity II utilise 22 couleurs. Elles se répartissent en 17 intérieures et 5 réservées au cadre, et ce 17 est exactement là où ce type de puzzle est le plus dur à résoudre.",
    p1: "Les problèmes de recherche difficiles ont un bouton de difficulté. Desserrez-le et il y a beaucoup de solutions, qu'une recherche trouve vite. Serrez-le et il n'y en a aucune, ce qui est souvent facile à prouver. Entre les deux se trouve une bande étroite où les solutions sont rares mais réelles, et c'est là que la recherche explose. On appelle cela une transition de phase, comme l'eau qui gèle.",
    p2: "Pour les puzzles d'assemblage par les bords, le bouton est le nombre de couleurs. Trop peu et les pièces s'emboîtent d'innombrables façons ; trop et elles ne s'emboîtent presque plus. L'analyse publiée situe le pic autour de 17 couleurs intérieures, le réglage où un puzzle de cette taille a environ une solution. Eternity II en utilise 17.",
    liveTitle: "Voir le phénomène",
    splitTitle: "La répartition, directement dans les pièces",
    splitIntro: "Trier les couleurs du jeu officiel selon où elles apparaissent rend la conception évidente.",
    frameLabel: "5 couleurs réservées au cadre",
    frameNote: "Elles n'apparaissent que sur les pièces de bord et de coin, jamais à l'intérieur. Ce sont les couleurs rares, gardées en bordure.",
    interiorLabel: "17 couleurs intérieures",
    interiorNote: "La palette de l'intérieur du plateau, là où se fait presque tout l'assemblage.",
    breakdownTitle: "Le jeu en chiffres",
    rows: [
      ["Pièces de coin", data.pieces.corner],
      ["Pièces de bord", data.pieces.edge],
      ["Pièces intérieures", data.pieces.interior],
      ["Couleurs intérieures", data.interiorColorCount],
      ["Couleurs de cadre", data.frameOnlyColorCount],
    ] as [string, number][],
    whyTitle: "Pourquoi c'est important",
    why1: "C'est le signe le plus clair qu'Eternity II a été rendu difficile à dessein. Taille du plateau, nombre de pièces et répartition des couleurs visent la même cible : un puzzle à environ une solution, placé au pire endroit pour qu'une recherche la trouve. La difficulté a été choisie, comme un bon examen n'est ni trivial ni impossible.",
    why2: "Comment savoir que le pic est réel et pas seulement une jolie histoire ? Deux voies se rejoignent ici. L'analyse publiée le démontre : pour les puzzles encadrés d'assemblage par les bords, le nombre de couleurs où l'on attend environ une solution tombe près de 17, et c'est le réglage le plus dur à explorer. Et vous pouvez en observer une partie vous-même dans la démo ci-dessus : construire de vrais puzzles, compter le travail, et voir l'explosion quand les couleurs sont rares. La conséquence est concrète. L'écart vers une solution n'est pas un problème de réglage qu'on grignote avec une machine plus rapide ; le puzzle a été placé là où la recherche est la pire, à dessein, donc le battre exige une idée vraiment meilleure, pas seulement plus d'efforts.",
    seeAlso: "Voir la difficulté mesurée en direct sur la page Algorithmes",
    reproTitle: "Le reproduire",
    reproBody: "La répartition des couleurs est lue directement dans le jeu officiel. Instantané et identique à chaque exécution.",
    sourceLink: "Article, code source et résultats sur GitHub",
  },
};

export default function PhaseTransition() {
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
        <h2 className="text-2xl font-semibold tracking-tight">{t.liveTitle}</h2>
        <PhaseTransitionLab />
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.splitTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.splitIntro}</p>
        <div className="grid max-w-3xl gap-5 sm:grid-cols-2">
          <div className="space-y-2 rounded-lg border p-4">
            <div className="text-sm font-semibold">{t.frameLabel}</div>
            <Swatches colors={FRAME_COLORS} />
            <p className="text-xs text-muted-foreground">{t.frameNote}</p>
          </div>
          <div className="space-y-2 rounded-lg border p-4">
            <div className="text-sm font-semibold">{t.interiorLabel}</div>
            <Swatches colors={INTERIOR_COLORS} />
            <p className="text-xs text-muted-foreground">{t.interiorNote}</p>
          </div>
        </div>
      </section>

      <section className="max-w-md space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.breakdownTitle}</h2>
        <dl className="divide-y rounded-lg border">
          {t.rows.map(([label, value]) => (
            <div key={label} className="flex items-center justify-between px-4 py-2.5">
              <dt className="text-sm text-muted-foreground">{label}</dt>
              <dd className="text-sm font-semibold tabular-nums">{value}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section className="max-w-3xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.whyTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.why1}</p>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.why2}</p>
        <LocalizedLink
          to="/algorithms"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.seeAlso}
        </LocalizedLink>
      </section>

      <section className="max-w-3xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md bg-background p-3 text-xs">
          <code>{`cd research/topics/phase-transition/compute
cargo run --release > ../results/color-split.json`}</code>
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
    </div>
  );
}

export const meta = pageMeta("phase-transition");
