import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { PruneVsSpeedLab } from "@/components/research/PruneVsSpeedLab";

const ARTICLE_URL =
  "https://github.com/raphael-anjou/eternity2/tree/main/research/topics/prune-vs-speed";

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Why a faster computer doesn't help",
    lede: "The single most important idea in hard combinatorial search: shrinking the space you search beats searching it faster — by an exponential margin. Eternity II is engineered so you can barely shrink it at all.",
    p1: "Picture the search as a tree. From the empty board you choose a piece for the first cell; from there a piece for the second; and so on, 256 cells deep. The number of leaves at the bottom is the branching factor raised to the depth — an astronomically large number. To prove a region has no solution, a search has to walk that tree.",
    p2: "Now there are two ways to do less work. You can go faster — a better engine, more cores, hand-tuned inner loops. Or you can make the tree smaller — prune branches that can't lead to a solution, so the effective branching factor drops. These sound similar. They are not even close.",
    mathTitle: "Constant versus exponential",
    math1: "A speedup is a constant divisor. Make the machine 1000× faster and you do 1000× less waiting — the same whether the tree is ten levels deep or ten thousand. It buys you a fixed multiple, full stop.",
    math2: "A prune compounds. Shave even a few percent off the branching factor and you save that fraction at every single level. Over 256 levels the savings multiply into each other: the tree shrinks by (reduced ⁄ original) raised to the 256th power. A 5% prune, applied all the way down, beats almost any speedup a real machine can offer.",
    labTitle: "Feel the gap",
    labIntro: "Trade a raw speedup against a small per-level prune and watch the prune win by orders of magnitude.",
    e2Title: "Why this is exactly E2's curse",
    e2Intro: "If pruning is the lever that matters, the hard puzzles are the ones you can't prune. Eternity II was tuned to be precisely that. Four of its walls are, underneath, all the same statement: there is nothing local to prune on.",
    e2List: [
      ["No forced moves", "Every interior cell still has 73 to 137 legal neighbours, so propagation almost never collapses a cell to one choice. The branching factor stays stubbornly high.", "/research/why/no-forced-moves"],
      ["On the hardness peak", "The piece and colour counts sit where there is about one expected solution — no solution-dense region to aim a statistical shortcut at, the trick that cracked Eternity I.", "/research/why/phase-transition"],
      ["The area law", "The count of genuinely-distinct partial boards collapses past ~80 cells, but no local scoring signal can see that global collapse, so you can't prune toward it cheaply.", "/research/why/entropy-area-law"],
      ["Rigidity", "Even at a record board, the move to a better one is huge and indivisible — no gradient to follow, nothing nearby to prune away.", "/research/why/rigidity-wall"],
    ] as [string, string, string][],
    impactTitle: "What it means for everything else here",
    impact: "This is the lens for the whole research section. Our 232× faster engine made the same search cheaper, not smaller — and didn't move the record. Every invention that did move the needle changed the shape of the search instead: a different scan order, a learned prior over where pieces sit, a confined region for the mismatches. And every dead end is, at heart, a prune that the puzzle's global structure refuses to honour. Speed first feels productive; it is almost never where the gap to 480 is hiding.",
    note: "The tree numbers in the demo are illustrative — a branching factor and depth chosen to be E2-like and legible, not a measurement of a specific solver. The principle (constant divisor versus exponential divisor) is exact.",
    reproTitle: "Reproduce it",
    reproBody: "The hardness curve and node counts are real engine measurements on small puzzles; the illustrative tree-size estimate is labelled as such. Deterministic — reproduces identically every run.",
    sourceLink: "Article, source and results on GitHub",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Pourquoi un ordinateur plus rapide n'aide pas",
    lede: "L'idée la plus importante de la recherche combinatoire difficile : réduire l'espace à explorer bat le fait de l'explorer plus vite — d'une marge exponentielle. Eternity II est conçu pour que l'on ne puisse presque pas le réduire.",
    p1: "Imaginez la recherche comme un arbre. Depuis le plateau vide, on choisit une pièce pour la première case ; de là, une pièce pour la deuxième ; et ainsi de suite, sur 256 cases de profondeur. Le nombre de feuilles au bas de l'arbre, c'est le facteur de branchement élevé à la profondeur — un nombre astronomique. Pour prouver qu'une région n'a pas de solution, une recherche doit parcourir cet arbre.",
    p2: "Il y a alors deux façons d'en faire moins. On peut aller plus vite — un meilleur moteur, plus de cœurs, des boucles internes optimisées. Ou rendre l'arbre plus petit — élaguer les branches qui ne peuvent mener à une solution, pour faire chuter le facteur de branchement effectif. Cela semble comparable. Ça ne l'est pas du tout.",
    mathTitle: "Constant contre exponentiel",
    math1: "Une accélération est un diviseur constant. Rendez la machine 1000× plus rapide et vous attendez 1000× moins — pareil que l'arbre fasse dix niveaux ou dix mille. Elle offre un multiple fixe, point final.",
    math2: "Un élagage se compose. Retirez même quelques pour cent au facteur de branchement et vous économisez cette fraction à chaque niveau. Sur 256 niveaux, les économies se multiplient entre elles : l'arbre rétrécit de (réduit ⁄ original) à la puissance 256. Un élagage de 5 %, appliqué jusqu'en bas, bat presque n'importe quelle accélération qu'une vraie machine peut offrir.",
    labTitle: "Ressentez l'écart",
    labIntro: "Échangez une accélération brute contre un petit élagage par niveau et regardez l'élagage gagner de plusieurs ordres de grandeur.",
    e2Title: "Pourquoi c'est exactement la malédiction d'E2",
    e2Intro: "Si l'élagage est le levier qui compte, les puzzles durs sont ceux qu'on ne peut pas élaguer. Eternity II a été réglé pour être précisément cela. Quatre de ses murs sont, au fond, une même affirmation : il n'y a rien de local à élaguer.",
    e2List: [
      ["Aucun coup forcé", "Chaque case intérieure a encore 73 à 137 voisines légales, donc la propagation ne réduit presque jamais une case à un seul choix. Le facteur de branchement reste obstinément élevé.", "/research/why/no-forced-moves"],
      ["Sur le pic de difficulté", "Le nombre de pièces et de couleurs se situe là où il y a environ une solution attendue — aucune région riche en solutions où viser un raccourci statistique, l'astuce qui a cassé Eternity I.", "/research/why/phase-transition"],
      ["La loi d'aire", "Le nombre de plateaux partiels vraiment distincts s'effondre au-delà de ~80 cellules, mais aucun signal de score local ne voit cet effondrement global, donc on ne peut pas élaguer vers lui à bon compte.", "/research/why/entropy-area-law"],
      ["La rigidité", "Même à un plateau record, le mouvement vers un meilleur est énorme et indivisible — aucun gradient à suivre, rien de proche à élaguer.", "/research/why/rigidity-wall"],
    ] as [string, string, string][],
    impactTitle: "Ce que cela signifie pour tout le reste ici",
    impact: "C'est la grille de lecture de toute la section recherche. Notre moteur 232× plus rapide a rendu la même recherche moins coûteuse, pas plus petite — et n'a pas bougé le record. Chaque invention qui a fait avancer les choses a changé la forme de la recherche : un autre ordre de parcours, un prior appris sur l'endroit où les pièces se posent, une région confinée pour les défauts. Et chaque impasse est, au fond, un élagage que la structure globale du puzzle refuse d'honorer. La vitesse d'abord semble productive ; ce n'est presque jamais là que se cache l'écart vers 480.",
    note: "Les nombres de l'arbre dans la démo sont illustratifs — un facteur de branchement et une profondeur choisis pour être proches d'E2 et lisibles, pas la mesure d'un solveur précis. Le principe (diviseur constant contre diviseur exponentiel) est exact.",
    reproTitle: "Le reproduire",
    reproBody: "La courbe de difficulté et les comptages de nœuds sont de vraies mesures du moteur sur de petits puzzles ; l'estimation illustrative de la taille de l'arbre est signalée comme telle. Déterministe — se reproduit à l'identique à chaque exécution.",
    sourceLink: "Article, code source et résultats sur GitHub",
  },
};

export default function PruneVsSpeed() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div className="mx-auto max-w-3xl text-center">
        <LocalizedLink to="/research/why" className="text-sm text-muted-foreground hover:text-foreground">
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-3 text-4xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-4 text-xl text-muted-foreground">{t.lede}</p>
      </div>

      <section className="mx-auto max-w-2xl space-y-4 text-base leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
        <p className="text-foreground">{t.p2}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.mathTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.math1}</p>
        <p className="text-base leading-relaxed text-muted-foreground">{t.math2}</p>
      </section>

      <section className="space-y-4">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-2xl font-semibold tracking-tight">{t.labTitle}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.labIntro}</p>
        </div>
        <PruneVsSpeedLab />
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.e2Title}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.e2Intro}</p>
        <div className="grid gap-3 sm:grid-cols-2">
          {t.e2List.map(([title, body, to]) => (
            <LocalizedLink
              key={to}
              to={to}
              className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
            >
              <div className="text-sm font-semibold tracking-tight group-hover:underline">{title}</div>
              <p className="mt-1 text-xs leading-relaxed text-muted-foreground">{body}</p>
            </LocalizedLink>
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.impactTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.impact}</p>
      </section>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.note}</p>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <pre className="overflow-x-auto rounded-md border bg-background p-3 text-xs leading-relaxed">
          <code>{`cd research/topics/prune-vs-speed/compute
cargo run --release > ../results/prune-vs-speed.json`}</code>
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

      <RelatedRail path="/research/why/prune-vs-speed" />
    </div>
  );
}

export const meta = pageMeta("prune-vs-speed");
